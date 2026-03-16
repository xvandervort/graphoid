//! Actor reference type for concurrent actor communication (Phase 19.3).
//! Extended with supervision support (Phase 19.5).
//!
//! An ActorRef is a handle to a running actor (a graph with `on_message`).
//! It holds a mailbox channel for sending messages to the actor.
//! ActorRef uses Arc-based Channel internally and is naturally thread-safe.

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;
use super::Channel;
use super::Value;
use super::channel::SendableValue;
use crate::error::{GraphoidError, Result};

static ACTOR_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Internal key for the actual message inside a request envelope.
pub const ACTOR_MSG_KEY: &str = "__actor_msg__";
/// Internal key for the reply channel inside a request envelope.
pub const REPLY_CH_KEY: &str = "__reply_ch__";
/// Variable name used for the actor's graph in the actor thread environment.
pub const ACTOR_GRAPH_VAR: &str = "__actor_graph__";

// ============================================================
// Supervision types
// ============================================================

/// Restart mode for supervised actors.
#[derive(Clone, Debug, PartialEq)]
pub enum RestartMode {
    /// Always restart on crash
    Permanent,
    /// Restart only on abnormal exit (unhandled exception)
    Transient,
    /// Never restart
    Temporary,
}

impl RestartMode {
    pub fn from_symbol(s: &str) -> Option<Self> {
        match s {
            "permanent" => Some(RestartMode::Permanent),
            "transient" => Some(RestartMode::Transient),
            "temporary" => Some(RestartMode::Temporary),
            _ => None,
        }
    }
}

/// Information about a supervised actor, stored in the global registry.
#[derive(Clone)]
pub struct SupervisionEntry {
    /// The supervisor's ActorRef (for strategy lookup)
    pub supervisor: ActorRef,
    /// How this child should be restarted
    pub restart_mode: RestartMode,
    /// Order of registration (for rest_for_one strategy)
    pub order: usize,
}

/// Spawn template stored for actor restart.
/// Contains everything needed to re-create an actor.
pub struct SpawnTemplate {
    pub graph: SendableValue,
    pub bindings: Vec<(String, SendableValue)>,
    pub globals: Vec<(String, Vec<SendableValue>)>,
}

impl Clone for SpawnTemplate {
    fn clone(&self) -> Self {
        SpawnTemplate {
            graph: self.graph.0.deep_clone_for_send(),
            bindings: self.bindings.iter()
                .map(|(n, sv)| (n.clone(), sv.0.deep_clone_for_send()))
                .collect(),
            globals: self.globals.iter()
                .map(|(n, ovs)| (n.clone(), ovs.iter().map(|sv| sv.0.deep_clone_for_send()).collect()))
                .collect(),
        }
    }
}

impl std::fmt::Debug for SpawnTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpawnTemplate").finish()
    }
}
// SAFETY: All values inside have been deep-cloned with fresh Rc handles.
unsafe impl Send for SpawnTemplate {}
unsafe impl Sync for SpawnTemplate {}

lazy_static::lazy_static! {
    /// Global registry mapping child actor ID → supervision info.
    pub static ref SUPERVISION_REGISTRY: Mutex<HashMap<u64, SupervisionEntry>> =
        Mutex::new(HashMap::new());

    /// Global registry mapping actor ID → ActorRef (for restart/lookup).
    pub static ref ACTOR_REGISTRY: Mutex<HashMap<u64, ActorRef>> =
        Mutex::new(HashMap::new());
}

// ============================================================
// ActorRef
// ============================================================

/// Reference to a running actor (thread-safe, Clone shares the reference).
#[derive(Clone, Debug)]
pub struct ActorRef {
    /// Unique actor identifier
    pub id: u64,
    /// The actor's mailbox channel (messages go in here)
    pub mailbox: Channel,
    /// Type name from the graph template (e.g., "Counter")
    pub type_name: Option<String>,
    /// Redirect target for mailbox after restart (shared across all clones)
    pub redirect: Arc<Mutex<Option<Channel>>>,
    /// Stored spawn template for re-creating actor on restart
    pub spawn_template: Arc<Mutex<Option<SpawnTemplate>>>,
    /// Supervisor config (set when this actor IS a supervisor)
    pub supervisor_config: Option<Arc<SupervisorConfig>>,
}

/// Configuration for a supervisor actor.
#[derive(Debug)]
pub struct SupervisorConfig {
    pub strategy: String,
    pub max_restarts: u32,
    /// Ordered list of supervised child actor IDs
    pub children: Mutex<Vec<u64>>,
    /// Restart tracking: (count, window_start)
    pub restart_tracking: Mutex<(u32, std::time::Instant)>,
}

impl SupervisorConfig {
    pub fn new(strategy: String, max_restarts: u32) -> Self {
        SupervisorConfig {
            strategy,
            max_restarts,
            children: Mutex::new(Vec::new()),
            restart_tracking: Mutex::new((0, std::time::Instant::now())),
        }
    }
}

impl ActorRef {
    /// Create a new actor reference with a unique ID.
    pub fn new(mailbox: Channel, type_name: Option<String>) -> Self {
        ActorRef {
            id: ACTOR_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            mailbox,
            type_name,
            redirect: Arc::new(Mutex::new(None)),
            spawn_template: Arc::new(Mutex::new(None)),
            supervisor_config: None,
        }
    }

    /// Get the effective mailbox, following redirect if actor was restarted.
    pub fn effective_mailbox(&self) -> Channel {
        if let Ok(guard) = self.redirect.lock() {
            if let Some(ref redirected) = *guard {
                return redirected.clone();
            }
        }
        self.mailbox.clone()
    }

    /// Redirect all future sends to a new mailbox (used for actor restart).
    /// Also drains any messages from the old effective mailbox into the new one,
    /// so messages sent before the redirect aren't lost.
    pub fn redirect_to(&self, new_mailbox: Channel) {
        let old_mailbox = self.effective_mailbox();
        // Set redirect first so new sends go to new mailbox
        if let Ok(mut guard) = self.redirect.lock() {
            *guard = Some(new_mailbox.clone());
        }
        // Drain old mailbox into new one (catches messages sent before redirect)
        while let Some(msg) = old_mailbox.try_receive() {
            let _ = new_mailbox.send(msg);
        }
        // Close old mailbox so old thread's receive() returns None
        old_mailbox.close();
    }

    /// Send a request to this actor and block waiting for the reply.
    /// Creates a one-shot reply channel, wraps the message as an envelope,
    /// and blocks until the actor responds.
    ///
    /// If the send fails (e.g., mailbox closed during restart), retries with
    /// the updated effective mailbox to handle transparent actor restart.
    pub fn send_request(&self, msg: &Value) -> Result<Value> {
        let reply_ch = Channel::new(Some(1));
        let mut envelope = crate::values::Hash::new();
        let _ = envelope.insert(ACTOR_MSG_KEY.to_string(), msg.deep_clone_for_send().0);
        let _ = envelope.insert(REPLY_CH_KEY.to_string(), Value::channel(reply_ch.clone()));
        let envelope_val = Value::map(envelope);

        // Retry loop: if mailbox was closed during restart, re-fetch effective mailbox
        let mut attempts = 0;
        loop {
            match self.effective_mailbox().send(envelope_val.deep_clone_for_send()) {
                Ok(()) => break,
                Err(_) if attempts < 10 => {
                    attempts += 1;
                    std::thread::sleep(std::time::Duration::from_millis(50));
                    // effective_mailbox() will return the new redirected mailbox
                }
                Err(e) => return Err(GraphoidError::runtime(e)),
            }
        }

        match reply_ch.receive() {
            Some(sendable_val) => Ok(sendable_val.0),
            None => Ok(Value::none()),
        }
    }
}

/// Extract request info from an actor message.
/// For `.request()`, messages are wrapped as `{ACTOR_MSG_KEY: msg, REPLY_CH_KEY: channel}`.
/// For `.send()`, messages are sent as-is.
pub fn extract_request_info(msg: &Value) -> (Value, Option<Channel>) {
    if let crate::values::ValueKind::Map(ref hash) = msg.kind {
        if let (Some(actual_msg), Some(reply_val)) = (
            hash.get(ACTOR_MSG_KEY),
            hash.get(REPLY_CH_KEY),
        ) {
            if let crate::values::ValueKind::Channel(ref ch) = reply_val.kind {
                return (actual_msg.clone(), Some(ch.clone()));
            }
        }
    }
    (msg.clone(), None)
}

impl PartialEq for ActorRef {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::fmt::Display for ActorRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref name) = self.type_name {
            write!(f, "<actor:{}#{}>", name, self.id)
        } else {
            write!(f, "<actor#{}>", self.id)
        }
    }
}

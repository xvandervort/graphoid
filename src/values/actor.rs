//! Actor reference type for concurrent actor communication (Phase 19.3).
//!
//! An ActorRef is a handle to a running actor (a graph with `on_message`).
//! It holds a mailbox channel for sending messages to the actor.
//! ActorRef uses Arc-based Channel internally and is naturally thread-safe.

use std::sync::atomic::{AtomicU64, Ordering};
use super::Channel;
use super::Value;
use crate::error::{GraphoidError, Result};

static ACTOR_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Internal key for the actual message inside a request envelope.
pub const ACTOR_MSG_KEY: &str = "__actor_msg__";
/// Internal key for the reply channel inside a request envelope.
pub const REPLY_CH_KEY: &str = "__reply_ch__";
/// Variable name used for the actor's graph in the actor thread environment.
pub const ACTOR_GRAPH_VAR: &str = "__actor_graph__";

/// Reference to a running actor (thread-safe, Clone shares the reference).
#[derive(Clone, Debug)]
pub struct ActorRef {
    /// Unique actor identifier
    pub id: u64,
    /// The actor's mailbox channel (messages go in here)
    pub mailbox: Channel,
    /// Type name from the graph template (e.g., "Counter")
    pub type_name: Option<String>,
}

impl ActorRef {
    /// Create a new actor reference with a unique ID.
    pub fn new(mailbox: Channel, type_name: Option<String>) -> Self {
        ActorRef {
            id: ACTOR_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            mailbox,
            type_name,
        }
    }
}

impl ActorRef {
    /// Send a request to this actor and block waiting for the reply.
    /// Creates a one-shot reply channel, wraps the message as an envelope,
    /// and blocks until the actor responds.
    pub fn send_request(&self, msg: &Value) -> Result<Value> {
        let reply_ch = Channel::new(Some(1));
        let mut envelope = crate::values::Hash::new();
        let _ = envelope.insert(ACTOR_MSG_KEY.to_string(), msg.deep_clone_for_send().0);
        let _ = envelope.insert(REPLY_CH_KEY.to_string(), Value::channel(reply_ch.clone()));
        let envelope_val = Value::map(envelope);

        self.mailbox.send(envelope_val.deep_clone_for_send())
            .map_err(|e| GraphoidError::runtime(e))?;

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

//! Channel type for concurrent task communication (Phase 19).
//!
//! Channels are thread-safe communication pipes between tasks.
//! They use Arc internally and are naturally Send + Sync.

use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::VecDeque;

/// A value wrapper that is safe to send across threads.
///
/// Created by deep-cloning a Value so all Rc handles are unique (not shared).
///
/// # Safety
/// SendableValue must only be created via `Value::deep_clone_for_send()`,
/// which guarantees all internal Rc references are freshly created and
/// not shared with any other thread.
pub struct SendableValue(pub crate::values::Value);

// SAFETY: SendableValue is only created via deep_clone_for_send(),
// which creates completely independent Rc handles. No Rc is shared
// between threads. Channel values use Arc internally and are naturally Send.
unsafe impl Send for SendableValue {}
unsafe impl Sync for SendableValue {}

/// Internal channel state, protected by Mutex + Condvar.
struct ChannelInner {
    /// Message buffer
    buffer: Mutex<VecDeque<SendableValue>>,
    /// Maximum buffer capacity. None = unbuffered (capacity 0)
    capacity: Option<usize>,
    /// Whether the channel has been closed
    closed: AtomicBool,
    /// Wakes receivers waiting for data
    not_empty: Condvar,
    /// Wakes senders waiting for space (buffered) or a receiver (unbuffered)
    not_full: Condvar,
}

/// A thread-safe communication channel between tasks.
///
/// Channels are first-class values in Graphoid, created with `channel()`.
/// Clone creates a reference to the same underlying channel (via Arc).
#[derive(Clone)]
pub struct Channel {
    inner: Arc<ChannelInner>,
}

impl std::fmt::Debug for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<channel>")
    }
}

impl Channel {
    /// Create a new channel.
    /// - `capacity: None` = unbuffered (send blocks until a receiver is ready)
    /// - `capacity: Some(n)` = buffered with n slots
    pub fn new(capacity: Option<usize>) -> Self {
        Channel {
            inner: Arc::new(ChannelInner {
                buffer: Mutex::new(VecDeque::new()),
                capacity,
                closed: AtomicBool::new(false),
                not_empty: Condvar::new(),
                not_full: Condvar::new(),
            }),
        }
    }

    /// Send a value through the channel.
    ///
    /// - Buffered: blocks if buffer is full
    /// - Unbuffered: blocks until a receiver picks up the value
    /// - Returns error if channel is closed
    pub fn send(&self, value: SendableValue) -> Result<(), String> {
        if self.inner.closed.load(Ordering::SeqCst) {
            return Err("Cannot send on closed channel".to_string());
        }

        let mut buffer = self.inner.buffer.lock().unwrap();

        match self.inner.capacity {
            Some(cap) => {
                // Buffered: wait for space
                while buffer.len() >= cap {
                    if self.inner.closed.load(Ordering::SeqCst) {
                        return Err("Cannot send on closed channel".to_string());
                    }
                    buffer = self.inner.not_full.wait(buffer).unwrap();
                }
            }
            None => {
                // Unbuffered: wait until buffer is empty (previous value consumed)
                while !buffer.is_empty() {
                    if self.inner.closed.load(Ordering::SeqCst) {
                        return Err("Cannot send on closed channel".to_string());
                    }
                    buffer = self.inner.not_full.wait(buffer).unwrap();
                }
            }
        }

        buffer.push_back(value);
        self.inner.not_empty.notify_one();
        Ok(())
    }

    /// Receive a value from the channel. Blocks until a value is available.
    ///
    /// Returns None if the channel is closed and empty.
    pub fn receive(&self) -> Option<SendableValue> {
        let mut buffer = self.inner.buffer.lock().unwrap();
        loop {
            if let Some(val) = buffer.pop_front() {
                self.inner.not_full.notify_one();
                return Some(val);
            }
            if self.inner.closed.load(Ordering::SeqCst) {
                return None; // Closed + empty
            }
            buffer = self.inner.not_empty.wait(buffer).unwrap();
        }
    }

    /// Non-blocking receive. Returns None if no value is available.
    pub fn try_receive(&self) -> Option<SendableValue> {
        let mut buffer = self.inner.buffer.lock().unwrap();
        let val = buffer.pop_front();
        if val.is_some() {
            self.inner.not_full.notify_one();
        }
        val
    }

    /// Close the channel. No more sends are allowed.
    /// Pending receives can still drain remaining messages.
    pub fn close(&self) {
        self.inner.closed.store(true, Ordering::SeqCst);
        // Wake all waiters so they can check the closed flag
        self.inner.not_empty.notify_all();
        self.inner.not_full.notify_all();
    }

    /// Check if the channel is closed.
    pub fn is_closed(&self) -> bool {
        self.inner.closed.load(Ordering::SeqCst)
    }
}

impl PartialEq for Channel {
    /// Channels are equal if they refer to the same underlying channel (Arc identity).
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

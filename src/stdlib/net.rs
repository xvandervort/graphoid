//! Network Module - Low-level network I/O primitives
//!
//! Provides minimal TCP socket operations for building HTTP and other protocols in pure Graphoid.
//! TLS is implemented in pure Graphoid (stdlib/tls.gr) using this module for TCP.
//!
//! Functions:
//! - connect(host, port) -> socket_id - Open TCP connection
//! - send(socket_id, data) -> bytes_sent - Send data to socket
//! - send_bytes(socket_id, byte_list) -> bytes_sent - Send raw bytes to socket
//! - recv(socket_id, max_bytes) -> data - Receive data from socket (as string)
//! - recv_bytes(socket_id, max_bytes) -> byte_list - Receive raw bytes from socket
//! - close(socket_id) -> bool - Close socket

use crate::error::{GraphoidError, Result};
use crate::stdlib::{NativeFunction, NativeModule};
use crate::values::{List, Value, ValueKind};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;

lazy_static::lazy_static! {
    /// Global socket handle registry
    static ref SOCKET_HANDLES: Arc<Mutex<HashMap<u64, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));
    /// Global listener handle registry (separate from sockets, shares ID counter)
    static ref LISTENER_HANDLES: Arc<Mutex<HashMap<u64, TcpListener>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref NEXT_SOCKET_ID: Arc<Mutex<u64>> = Arc::new(Mutex::new(1));
}

/// Net module for network primitives
pub struct NetModule;

impl NativeModule for NetModule {
    fn name(&self) -> &str {
        "net"
    }

    fn alias(&self) -> Option<&str> {
        None
    }

    fn functions(&self) -> HashMap<String, NativeFunction> {
        let mut functions: HashMap<String, NativeFunction> = HashMap::new();

        functions.insert("connect".to_string(), net_connect as NativeFunction);
        functions.insert("send".to_string(), net_send as NativeFunction);
        functions.insert("send_bytes".to_string(), net_send_bytes as NativeFunction);
        functions.insert("recv".to_string(), net_recv as NativeFunction);
        functions.insert("recv_bytes".to_string(), net_recv_bytes as NativeFunction);
        functions.insert("close".to_string(), net_close as NativeFunction);

        // Server primitives
        functions.insert("bind".to_string(), net_bind as NativeFunction);
        functions.insert("accept".to_string(), net_accept as NativeFunction);
        functions.insert("close_listener".to_string(), net_close_listener as NativeFunction);
        functions.insert("listener_port".to_string(), net_listener_port as NativeFunction);
        functions.insert("set_timeout".to_string(), net_set_timeout as NativeFunction);

        // Fast hex/bytes conversion utilities (used by TLS)
        functions.insert("hex_to_bytes".to_string(), hex_to_bytes as NativeFunction);
        functions.insert("bytes_to_hex".to_string(), bytes_to_hex as NativeFunction);
        functions.insert("concat_bytes".to_string(), concat_bytes as NativeFunction);
        functions.insert("bytes_to_string".to_string(), bytes_to_string as NativeFunction);

        functions
    }
}

// Helper to get string argument
fn get_string_arg(args: &[Value], index: usize, func_name: &str) -> Result<String> {
    match args.get(index) {
        Some(value) => match &value.kind {
            ValueKind::String(s) => Ok(s.clone()),
            _ => Err(GraphoidError::RuntimeError {
                message: format!("{}() argument {} must be a string", func_name, index + 1),
            }),
        },
        None => Err(GraphoidError::RuntimeError {
            message: format!("{}() missing argument at position {}", func_name, index + 1),
        }),
    }
}

// Helper to get number argument
fn get_number_arg(args: &[Value], index: usize, func_name: &str) -> Result<f64> {
    match args.get(index) {
        Some(value) => match &value.kind {
            ValueKind::Number(n) => Ok(*n),
            _ => Err(GraphoidError::RuntimeError {
                message: format!("{}() argument {} must be a number", func_name, index + 1),
            }),
        },
        None => Err(GraphoidError::RuntimeError {
            message: format!("{}() missing argument at position {}", func_name, index + 1),
        }),
    }
}

// Helper to get list argument as bytes
fn get_byte_list_arg(args: &[Value], index: usize, func_name: &str) -> Result<Vec<u8>> {
    match args.get(index) {
        Some(value) => match &value.kind {
            ValueKind::List(list) => {
                let mut bytes = Vec::new();
                for i in 0..list.len() {
                    if let Some(item) = list.get(i) {
                        match &item.kind {
                            ValueKind::Number(n) => {
                                let byte = *n as u8;
                                bytes.push(byte);
                            }
                            _ => return Err(GraphoidError::RuntimeError {
                                message: format!("{}() byte list must contain only numbers", func_name),
                            }),
                        }
                    }
                }
                Ok(bytes)
            }
            _ => Err(GraphoidError::RuntimeError {
                message: format!("{}() argument {} must be a list", func_name, index + 1),
            }),
        },
        None => Err(GraphoidError::RuntimeError {
            message: format!("{}() missing argument at position {}", func_name, index + 1),
        }),
    }
}

// Allocate the next unique handle ID (shared by sockets and listeners)
fn next_handle_id() -> u64 {
    let mut next_id = NEXT_SOCKET_ID.lock().unwrap();
    let id = *next_id;
    *next_id += 1;
    id
}

/// Connect to a TCP socket
/// net.connect(host, port) -> socket_id
fn net_connect(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::RuntimeError {
            message: "connect() requires exactly 2 arguments: host and port".to_string(),
        });
    }

    let host = get_string_arg(args, 0, "connect")?;
    let port = get_number_arg(args, 1, "connect")? as u16;

    let address = format!("{}:{}", host, port);
    let stream = TcpStream::connect(&address).map_err(|e| GraphoidError::RuntimeError {
        message: format!("Failed to connect to {}: {}", address, e),
    })?;

    // Set timeouts to prevent hanging
    stream
        .set_read_timeout(Some(Duration::from_secs(30)))
        .map_err(|e| GraphoidError::RuntimeError {
            message: format!("Failed to set read timeout: {}", e),
        })?;

    stream
        .set_write_timeout(Some(Duration::from_secs(30)))
        .map_err(|e| GraphoidError::RuntimeError {
            message: format!("Failed to set write timeout: {}", e),
        })?;

    let socket_id = next_handle_id();

    SOCKET_HANDLES.lock().unwrap().insert(socket_id, stream);

    Ok(Value::number(socket_id as f64))
}

/// Send string data to socket
/// net.send(socket_id, data) -> bytes_sent
fn net_send(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::RuntimeError {
            message: "send() requires exactly 2 arguments: socket_id and data".to_string(),
        });
    }

    let socket_id = get_number_arg(args, 0, "send")? as u64;
    let data = get_string_arg(args, 1, "send")?;

    let mut handles = SOCKET_HANDLES.lock().unwrap();
    let stream = handles.get_mut(&socket_id).ok_or_else(|| GraphoidError::RuntimeError {
        message: format!("Invalid socket handle: {}", socket_id),
    })?;

    let bytes_sent = stream.write(data.as_bytes()).map_err(|e| GraphoidError::RuntimeError {
        message: format!("Failed to send data: {}", e),
    })?;

    stream.flush().map_err(|e| GraphoidError::RuntimeError {
        message: format!("Failed to flush socket: {}", e),
    })?;

    Ok(Value::number(bytes_sent as f64))
}

/// Send raw bytes to socket
/// net.send_bytes(socket_id, byte_list) -> bytes_sent
fn net_send_bytes(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::RuntimeError {
            message: "send_bytes() requires exactly 2 arguments: socket_id and byte_list".to_string(),
        });
    }

    let socket_id = get_number_arg(args, 0, "send_bytes")? as u64;
    let bytes = get_byte_list_arg(args, 1, "send_bytes")?;

    let mut handles = SOCKET_HANDLES.lock().unwrap();
    let stream = handles.get_mut(&socket_id).ok_or_else(|| GraphoidError::RuntimeError {
        message: format!("Invalid socket handle: {}", socket_id),
    })?;

    let bytes_sent = stream.write(&bytes).map_err(|e| GraphoidError::RuntimeError {
        message: format!("Failed to send bytes: {}", e),
    })?;

    stream.flush().map_err(|e| GraphoidError::RuntimeError {
        message: format!("Failed to flush socket: {}", e),
    })?;

    Ok(Value::number(bytes_sent as f64))
}

/// Receive data from socket as string
/// net.recv(socket_id, max_bytes) -> data
fn net_recv(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::RuntimeError {
            message: "recv() requires exactly 2 arguments: socket_id and max_bytes".to_string(),
        });
    }

    let socket_id = get_number_arg(args, 0, "recv")? as u64;
    let max_bytes = get_number_arg(args, 1, "recv")? as usize;

    let mut handles = SOCKET_HANDLES.lock().unwrap();
    let stream = handles.get_mut(&socket_id).ok_or_else(|| GraphoidError::RuntimeError {
        message: format!("Invalid socket handle: {}", socket_id),
    })?;

    let mut buffer = vec![0u8; max_bytes];
    let bytes_read = stream.read(&mut buffer).map_err(|e| GraphoidError::RuntimeError {
        message: format!("Failed to receive data: {}", e),
    })?;

    buffer.truncate(bytes_read);
    let content = String::from_utf8_lossy(&buffer).to_string();

    Ok(Value::string(content))
}

/// Receive raw bytes from socket
/// net.recv_bytes(socket_id, max_bytes) -> byte_list
fn net_recv_bytes(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::RuntimeError {
            message: "recv_bytes() requires exactly 2 arguments: socket_id and max_bytes".to_string(),
        });
    }

    let socket_id = get_number_arg(args, 0, "recv_bytes")? as u64;
    let max_bytes = get_number_arg(args, 1, "recv_bytes")? as usize;

    let mut handles = SOCKET_HANDLES.lock().unwrap();
    let stream = handles.get_mut(&socket_id).ok_or_else(|| GraphoidError::RuntimeError {
        message: format!("Invalid socket handle: {}", socket_id),
    })?;

    let mut buffer = vec![0u8; max_bytes];
    let bytes_read = stream.read(&mut buffer).map_err(|e| GraphoidError::RuntimeError {
        message: format!("Failed to receive data: {}", e),
    })?;

    buffer.truncate(bytes_read);

    // Convert to list of numbers
    let byte_values: Vec<Value> = buffer.iter().map(|&b| Value::number(b as f64)).collect();
    Ok(Value::list(List::from_vec(byte_values)))
}

/// Close socket
/// net.close(socket_id) -> bool
fn net_close(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(GraphoidError::RuntimeError {
            message: "close() requires exactly 1 argument: socket_id".to_string(),
        });
    }

    let socket_id = get_number_arg(args, 0, "close")? as u64;

    let mut handles = SOCKET_HANDLES.lock().unwrap();
    let removed = handles.remove(&socket_id).is_some();

    if !removed {
        return Err(GraphoidError::RuntimeError {
            message: format!("Invalid socket handle: {}", socket_id),
        });
    }

    Ok(Value::boolean(true))
}

/// Convert hex string to byte list (fast native implementation)
/// net.hex_to_bytes(hex_str) -> byte_list
fn hex_to_bytes(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(GraphoidError::RuntimeError {
            message: "hex_to_bytes() requires exactly 1 argument: hex_string".to_string(),
        });
    }

    let hex_str = get_string_arg(args, 0, "hex_to_bytes")?;

    // Fast hex decode
    let bytes = hex::decode(&hex_str).map_err(|e| GraphoidError::RuntimeError {
        message: format!("Invalid hex string: {}", e),
    })?;

    let byte_values: Vec<Value> = bytes.iter().map(|&b| Value::number(b as f64)).collect();
    Ok(Value::list(List::from_vec(byte_values)))
}

/// Convert byte list to hex string (fast native implementation)
/// net.bytes_to_hex(byte_list) -> hex_str
fn bytes_to_hex(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(GraphoidError::RuntimeError {
            message: "bytes_to_hex() requires exactly 1 argument: byte_list".to_string(),
        });
    }

    let bytes = get_byte_list_arg(args, 0, "bytes_to_hex")?;
    Ok(Value::string(hex::encode(&bytes)))
}

/// Concatenate two byte lists (fast native implementation)
/// net.concat_bytes(list_a, list_b) -> combined_list
fn concat_bytes(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::RuntimeError {
            message: "concat_bytes() requires exactly 2 arguments: list_a and list_b".to_string(),
        });
    }

    let list_a = get_byte_list_arg(args, 0, "concat_bytes")?;
    let list_b = get_byte_list_arg(args, 1, "concat_bytes")?;

    let mut combined = list_a;
    combined.extend(list_b);

    let byte_values: Vec<Value> = combined.iter().map(|&b| Value::number(b as f64)).collect();
    Ok(Value::list(List::from_vec(byte_values)))
}

/// Convert byte list to UTF-8 string
/// net.bytes_to_string(byte_list) -> string
fn bytes_to_string(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(GraphoidError::RuntimeError {
            message: "bytes_to_string() requires exactly 1 argument: byte_list".to_string(),
        });
    }

    let bytes = get_byte_list_arg(args, 0, "bytes_to_string")?;

    // Convert to UTF-8 string, replacing invalid sequences
    let s = String::from_utf8_lossy(&bytes).into_owned();
    Ok(Value::string(s))
}

// ============================================
// Server primitives (Phase 18.6)
// ============================================

/// Bind a TCP listener on host:port
/// net.bind(host, port) -> listener_id
fn net_bind(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::RuntimeError {
            message: "bind() requires exactly 2 arguments: host and port".to_string(),
        });
    }

    let host = get_string_arg(args, 0, "bind")?;
    let port = get_number_arg(args, 1, "bind")? as u16;

    let address = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&address).map_err(|e| GraphoidError::RuntimeError {
        message: format!("Failed to bind to {}: {}", address, e),
    })?;

    let listener_id = next_handle_id();

    LISTENER_HANDLES.lock().unwrap().insert(listener_id, listener);

    Ok(Value::number(listener_id as f64))
}

/// Accept a connection on a listener (blocking)
/// net.accept(listener_id) -> socket_id
fn net_accept(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(GraphoidError::RuntimeError {
            message: "accept() requires exactly 1 argument: listener_id".to_string(),
        });
    }

    let listener_id = get_number_arg(args, 0, "accept")? as u64;

    // Clone the listener to avoid holding the mutex during blocking accept
    let listener_clone = {
        let handles = LISTENER_HANDLES.lock().unwrap();
        handles.get(&listener_id).ok_or_else(|| GraphoidError::RuntimeError {
            message: format!("Invalid listener handle: {}", listener_id),
        })?.try_clone().map_err(|e| GraphoidError::RuntimeError {
            message: format!("Failed to clone listener: {}", e),
        })?
    };

    // Accept (blocking) — outside the lock
    let (stream, _addr) = listener_clone.accept().map_err(|e| GraphoidError::RuntimeError {
        message: format!("Failed to accept connection: {}", e),
    })?;

    // Set reasonable timeouts on accepted connections
    stream.set_read_timeout(Some(Duration::from_secs(30))).ok();
    stream.set_write_timeout(Some(Duration::from_secs(30))).ok();

    let socket_id = next_handle_id();

    SOCKET_HANDLES.lock().unwrap().insert(socket_id, stream);

    Ok(Value::number(socket_id as f64))
}

/// Close a listener
/// net.close_listener(listener_id) -> bool
fn net_close_listener(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(GraphoidError::RuntimeError {
            message: "close_listener() requires exactly 1 argument: listener_id".to_string(),
        });
    }

    let listener_id = get_number_arg(args, 0, "close_listener")? as u64;

    let removed = LISTENER_HANDLES.lock().unwrap().remove(&listener_id).is_some();

    if !removed {
        return Err(GraphoidError::RuntimeError {
            message: format!("Invalid listener handle: {}", listener_id),
        });
    }

    Ok(Value::boolean(true))
}

/// Get the port a listener is bound to (useful when binding to port 0)
/// net.listener_port(listener_id) -> port
fn net_listener_port(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(GraphoidError::RuntimeError {
            message: "listener_port() requires exactly 1 argument: listener_id".to_string(),
        });
    }

    let listener_id = get_number_arg(args, 0, "listener_port")? as u64;

    let handles = LISTENER_HANDLES.lock().unwrap();
    let listener = handles.get(&listener_id).ok_or_else(|| GraphoidError::RuntimeError {
        message: format!("Invalid listener handle: {}", listener_id),
    })?;

    let port = listener.local_addr().map_err(|e| GraphoidError::RuntimeError {
        message: format!("Failed to get local address: {}", e),
    })?.port();

    Ok(Value::number(port as f64))
}

/// Set read/write timeout on a socket
/// net.set_timeout(socket_id, seconds) -> bool
fn net_set_timeout(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::RuntimeError {
            message: "set_timeout() requires exactly 2 arguments: socket_id and seconds".to_string(),
        });
    }

    let socket_id = get_number_arg(args, 0, "set_timeout")? as u64;
    let seconds = get_number_arg(args, 1, "set_timeout")?;

    let handles = SOCKET_HANDLES.lock().unwrap();
    let stream = handles.get(&socket_id).ok_or_else(|| GraphoidError::RuntimeError {
        message: format!("Invalid socket handle: {}", socket_id),
    })?;

    let duration = if seconds > 0.0 {
        Some(Duration::from_secs_f64(seconds))
    } else {
        None
    };

    stream.set_read_timeout(duration).map_err(|e| GraphoidError::RuntimeError {
        message: format!("Failed to set read timeout: {}", e),
    })?;
    stream.set_write_timeout(duration).map_err(|e| GraphoidError::RuntimeError {
        message: format!("Failed to set write timeout: {}", e),
    })?;

    Ok(Value::boolean(true))
}

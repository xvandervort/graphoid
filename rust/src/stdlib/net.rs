//! Network Module - Low-level network I/O primitives
//!
//! Provides minimal TCP socket operations for building HTTP and other protocols in pure Graphoid.
//!
//! Functions:
//! - connect(host, port) -> socket_id - Open TCP connection
//! - send(socket_id, data) -> bytes_sent - Send data to socket
//! - recv(socket_id, max_bytes) -> data - Receive data from socket
//! - close(socket_id) -> bool - Close socket

use crate::error::{GraphoidError, Result};
use crate::stdlib::{NativeFunction, NativeModule};
use crate::values::{Value, ValueKind};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;

lazy_static::lazy_static! {
    /// Global socket handle registry
    static ref SOCKET_HANDLES: Arc<Mutex<HashMap<u64, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));
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
        functions.insert("recv".to_string(), net_recv as NativeFunction);
        functions.insert("close".to_string(), net_close as NativeFunction);

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

    // Generate socket ID and store handle
    let socket_id = {
        let mut next_id = NEXT_SOCKET_ID.lock().unwrap();
        let id = *next_id;
        *next_id += 1;
        id
    };

    SOCKET_HANDLES.lock().unwrap().insert(socket_id, stream);

    Ok(Value::number(socket_id as f64))
}

/// Send data to socket
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

/// Receive data from socket
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

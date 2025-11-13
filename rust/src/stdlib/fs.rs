//! File System Module - Low-level file I/O primitives
//!
//! Provides minimal file operations for building higher-level I/O in pure Graphoid.
//!
//! Functions:
//! - open(path, mode) -> file_id - Open file and return handle
//! - read(file_id, max_bytes) -> bytes - Read bytes from file
//! - write(file_id, bytes) -> count - Write bytes to file
//! - close(file_id) -> bool - Close file handle
//!
//! Modes: "r" (read), "w" (write), "a" (append)

use crate::error::{GraphoidError, Result};
use crate::stdlib::{NativeFunction, NativeModule};
use crate::values::{Value, ValueKind};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    /// Global file handle registry
    static ref FILE_HANDLES: Arc<Mutex<HashMap<u64, File>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref NEXT_FILE_ID: Arc<Mutex<u64>> = Arc::new(Mutex::new(1));
}

/// FS module for file system primitives
pub struct FSModule;

impl NativeModule for FSModule {
    fn name(&self) -> &str {
        "fs"
    }

    fn alias(&self) -> Option<&str> {
        None
    }

    fn functions(&self) -> HashMap<String, NativeFunction> {
        let mut functions: HashMap<String, NativeFunction> = HashMap::new();

        functions.insert("open".to_string(), fs_open as NativeFunction);
        functions.insert("read".to_string(), fs_read as NativeFunction);
        functions.insert("write".to_string(), fs_write as NativeFunction);
        functions.insert("close".to_string(), fs_close as NativeFunction);

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

/// Open a file and return a file handle
/// fs.open(path, mode) -> file_id
fn fs_open(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::RuntimeError {
            message: "open() requires exactly 2 arguments: path and mode".to_string(),
        });
    }

    let path = get_string_arg(args, 0, "open")?;
    let mode = get_string_arg(args, 1, "open")?;

    // Open file based on mode
    let file = match mode.as_str() {
        "r" => File::open(&path).map_err(|e| GraphoidError::RuntimeError {
            message: format!("Failed to open file '{}' for reading: {}", path, e),
        })?,
        "w" => OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .map_err(|e| GraphoidError::RuntimeError {
                message: format!("Failed to open file '{}' for writing: {}", path, e),
            })?,
        "a" => OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| GraphoidError::RuntimeError {
                message: format!("Failed to open file '{}' for appending: {}", path, e),
            })?,
        _ => {
            return Err(GraphoidError::RuntimeError {
                message: format!("Invalid mode '{}'. Use 'r', 'w', or 'a'", mode),
            });
        }
    };

    // Generate file ID and store handle
    let file_id = {
        let mut next_id = NEXT_FILE_ID.lock().unwrap();
        let id = *next_id;
        *next_id += 1;
        id
    };

    FILE_HANDLES.lock().unwrap().insert(file_id, file);

    Ok(Value::number(file_id as f64))
}

/// Read bytes from file
/// fs.read(file_id, max_bytes) -> string
fn fs_read(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::RuntimeError {
            message: "read() requires exactly 2 arguments: file_id and max_bytes".to_string(),
        });
    }

    let file_id = get_number_arg(args, 0, "read")? as u64;
    let max_bytes = get_number_arg(args, 1, "read")? as usize;

    let mut handles = FILE_HANDLES.lock().unwrap();
    let file = handles.get_mut(&file_id).ok_or_else(|| GraphoidError::RuntimeError {
        message: format!("Invalid file handle: {}", file_id),
    })?;

    let mut buffer = vec![0u8; max_bytes];
    let bytes_read = file.read(&mut buffer).map_err(|e| GraphoidError::RuntimeError {
        message: format!("Failed to read from file: {}", e),
    })?;

    buffer.truncate(bytes_read);
    let content = String::from_utf8_lossy(&buffer).to_string();

    Ok(Value::string(content))
}

/// Write bytes to file
/// fs.write(file_id, data) -> bytes_written
fn fs_write(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::RuntimeError {
            message: "write() requires exactly 2 arguments: file_id and data".to_string(),
        });
    }

    let file_id = get_number_arg(args, 0, "write")? as u64;
    let data = get_string_arg(args, 1, "write")?;

    let mut handles = FILE_HANDLES.lock().unwrap();
    let file = handles.get_mut(&file_id).ok_or_else(|| GraphoidError::RuntimeError {
        message: format!("Invalid file handle: {}", file_id),
    })?;

    let bytes_written = file.write(data.as_bytes()).map_err(|e| GraphoidError::RuntimeError {
        message: format!("Failed to write to file: {}", e),
    })?;

    file.flush().map_err(|e| GraphoidError::RuntimeError {
        message: format!("Failed to flush file: {}", e),
    })?;

    Ok(Value::number(bytes_written as f64))
}

/// Close file handle
/// fs.close(file_id) -> bool
fn fs_close(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(GraphoidError::RuntimeError {
            message: "close() requires exactly 1 argument: file_id".to_string(),
        });
    }

    let file_id = get_number_arg(args, 0, "close")? as u64;

    let mut handles = FILE_HANDLES.lock().unwrap();
    let removed = handles.remove(&file_id).is_some();

    if !removed {
        return Err(GraphoidError::RuntimeError {
            message: format!("Invalid file handle: {}", file_id),
        });
    }

    Ok(Value::boolean(true))
}

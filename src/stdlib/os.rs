//! OS module - System primitives and platform operations
//!
//! Provides minimal system-level operations needed by higher-level modules:
//! - system_timestamp() - Current Unix timestamp (for time module)
//! - env(key) - Get environment variable
//! - env_all() - Get all environment variables
//! - getcwd() - Get current working directory
//! - platform() - Get OS platform name
//! - arch() - Get CPU architecture
//! - args() - Get command-line arguments
//! - input(prompt) - Read a line from stdin

use crate::error::{GraphoidError, Result, SourcePosition};
use crate::stdlib::{NativeFunction, NativeModule};
use crate::values::{Hash, List, Value, ValueKind};
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

/// OS module for system primitives
pub struct OSModule;

// Helper function to validate no arguments
fn require_no_args(args: &[Value], func_name: &str) -> Result<()> {
    if !args.is_empty() {
        return Err(GraphoidError::RuntimeError {
            message: format!("{}() takes no arguments", func_name),
        });
    }
    Ok(())
}

impl NativeModule for OSModule {
    fn name(&self) -> &str {
        "os"
    }

    fn alias(&self) -> Option<&str> {
        None
    }

    fn functions(&self) -> HashMap<String, NativeFunction> {
        let mut functions: HashMap<String, NativeFunction> = HashMap::new();

        functions.insert("system_timestamp".to_string(), system_timestamp as NativeFunction);
        functions.insert("env".to_string(), env_get as NativeFunction);
        functions.insert("env_all".to_string(), env_all as NativeFunction);
        functions.insert("getcwd".to_string(), getcwd as NativeFunction);
        functions.insert("platform".to_string(), platform as NativeFunction);
        functions.insert("arch".to_string(), arch as NativeFunction);
        functions.insert("args".to_string(), args as NativeFunction);
        functions.insert("input".to_string(), input as NativeFunction);

        functions
    }

    fn constants(&self) -> HashMap<String, Value> {
        HashMap::new()
    }
}

/// Get current Unix timestamp (seconds since epoch)
fn system_timestamp(args: &[Value]) -> Result<Value> {
    require_no_args(args, "system_timestamp")?;

    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| GraphoidError::RuntimeError {
            message: format!("Failed to get system time: {}", e),
        })?;

    let timestamp = duration.as_secs() as f64 + duration.subsec_nanos() as f64 / 1_000_000_000.0;
    Ok(Value::number(timestamp))
}

/// Get environment variable by key
fn env_get(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(GraphoidError::RuntimeError {
            message: "env() requires exactly one argument (variable name)".to_string(),
        });
    }

    let key = match &args[0].kind {
        ValueKind::String(s) => s.clone(),
        _ => {
            return Err(GraphoidError::TypeError {
                message: format!(
                    "env() requires string argument, got {}",
                    args[0].type_name()
                ),
                position: SourcePosition::unknown(),
            })
        }
    };

    match env::var(&key) {
        Ok(value) => Ok(Value::string(value)),
        Err(_) => Ok(Value::none()),
    }
}

/// Get all environment variables as a hash
fn env_all(args: &[Value]) -> Result<Value> {
    require_no_args(args, "env_all")?;

    let mut hash = Hash::new();
    for (key, value) in env::vars() {
        hash.insert(key, Value::string(value))
            .map_err(|e| GraphoidError::RuntimeError {
                message: format!("Failed to insert environment variable: {}", e),
            })?;
    }

    Ok(Value::map(hash))
}

/// Get current working directory
fn getcwd(args: &[Value]) -> Result<Value> {
    require_no_args(args, "getcwd")?;

    let cwd = env::current_dir().map_err(|e| GraphoidError::RuntimeError {
        message: format!("Failed to get current directory: {}", e),
    })?;

    Ok(Value::string(
        cwd.to_str()
            .ok_or_else(|| GraphoidError::RuntimeError {
                message: "Current directory path contains invalid UTF-8".to_string(),
            })?
            .to_string(),
    ))
}

/// Get platform/OS name
fn platform(args: &[Value]) -> Result<Value> {
    require_no_args(args, "platform")?;

    let platform_name = if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(unix) {
        "unix"
    } else {
        "unknown"
    };

    Ok(Value::string(platform_name.to_string()))
}

/// Get CPU architecture
fn arch(args: &[Value]) -> Result<Value> {
    require_no_args(args, "arch")?;

    let arch_name = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else if cfg!(target_arch = "arm") {
        "arm"
    } else if cfg!(target_arch = "x86") {
        "x86"
    } else {
        env::consts::ARCH
    };

    Ok(Value::string(arch_name.to_string()))
}

/// Get command-line arguments
fn args(args: &[Value]) -> Result<Value> {
    require_no_args(args, "args")?;

    let mut list = List::new();
    for arg in env::args() {
        list.append(Value::string(arg))
            .map_err(|e| GraphoidError::RuntimeError {
                message: format!("Failed to add argument to list: {}", e),
            })?;
    }

    Ok(Value::list(list))
}

/// Read a line from stdin with optional prompt
fn input(args: &[Value]) -> Result<Value> {
    // Optional prompt argument
    let prompt = if args.is_empty() {
        String::new()
    } else if args.len() == 1 {
        match &args[0].kind {
            ValueKind::String(s) => s.clone(),
            _ => {
                return Err(GraphoidError::TypeError {
                    message: format!(
                        "input() requires string argument, got {}",
                        args[0].type_name()
                    ),
                    position: SourcePosition::unknown(),
                })
            }
        }
    } else {
        return Err(GraphoidError::RuntimeError {
            message: "input() takes at most one argument (prompt)".to_string(),
        });
    };

    // Print prompt without newline
    if !prompt.is_empty() {
        print!("{}", prompt);
        io::stdout().flush().map_err(|e| GraphoidError::RuntimeError {
            message: format!("Failed to flush stdout: {}", e),
        })?;
    }

    // Read line from stdin
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .map_err(|e| GraphoidError::RuntimeError {
            message: format!("Failed to read from stdin: {}", e),
        })?;

    // Remove trailing newline
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }

    Ok(Value::string(line))
}

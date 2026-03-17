//! Library loading via libloading.

use std::path::PathBuf;
use crate::error::GraphoidError;
use crate::values::foreign::ForeignLib;

/// Resolve a library name to candidate paths, handling platform conventions.
/// Tries multiple naming patterns since libraries may be versioned (e.g., libc.so.6).
fn resolve_library_name(name: &str) -> Vec<String> {
    // Explicit path — use as-is
    if name.contains('/') || name.contains('\\') || name.starts_with('.') {
        return vec![name.to_string()];
    }

    let mut candidates = Vec::new();

    #[cfg(target_os = "linux")]
    {
        // Try versioned names first (most common on modern Linux)
        for ver in &["6", ""] {
            if ver.is_empty() {
                candidates.push(format!("lib{}.so", name));
            } else {
                candidates.push(format!("lib{}.so.{}", name, ver));
            }
        }
        candidates.push(format!("{}.so", name));
    }

    #[cfg(target_os = "macos")]
    {
        candidates.push(format!("lib{}.dylib", name));
        candidates.push(format!("{}.dylib", name));
    }

    #[cfg(target_os = "windows")]
    {
        candidates.push(format!("{}.dll", name));
    }

    candidates.push(name.to_string());
    candidates
}

/// Load a dynamic library by name or path.
pub fn load_library(name: &str) -> Result<ForeignLib, GraphoidError> {
    let candidates = resolve_library_name(name);

    let mut last_error = None;
    for candidate in &candidates {
        match unsafe { libloading::Library::new(candidate) } {
            Ok(lib) => {
                let path = PathBuf::from(candidate);
                return Ok(ForeignLib::new(lib, name.to_string(), path));
            }
            Err(e) => {
                last_error = Some(e);
            }
        }
    }

    Err(GraphoidError::runtime(format!(
        "Failed to load library '{}': {}",
        name,
        last_error.map(|e| e.to_string()).unwrap_or_else(|| "no candidates".to_string())
    )))
}

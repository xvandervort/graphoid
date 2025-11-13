//! Native standard library modules
//!
//! This module provides native (Rust) implementations of standard library modules
//! for performance-critical operations, system integration, and complex algorithms.

use crate::error::Result;
use crate::values::Value;
use std::collections::HashMap;

/// Type alias for native functions
pub type NativeFunction = fn(&[Value]) -> Result<Value>;

/// Trait for native standard library modules
pub trait NativeModule: Send + Sync {
    /// Returns the module name (used for `import "name"`)
    fn name(&self) -> &str;

    /// Returns the module alias (optional shorthand)
    fn alias(&self) -> Option<&str> {
        None
    }

    /// Returns a map of function names to native function implementations
    fn functions(&self) -> HashMap<String, NativeFunction> {
        HashMap::new()
    }

    /// Returns a map of constant names to values
    fn constants(&self) -> HashMap<String, Value> {
        HashMap::new()
    }
}

// Module implementations
pub mod constants;
pub mod random;
pub mod os;
pub mod fs;
pub mod net;

// Re-exports
pub use constants::ConstantsModule;
pub use random::RandomModule;
pub use os::OSModule;
pub use fs::FSModule;
pub use net::NetModule;

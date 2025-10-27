//! Execution engine
//!
//! This module executes AST nodes.

pub mod environment;
pub mod executor;
pub mod module_manager;

pub use environment::Environment;
pub use executor::Executor;

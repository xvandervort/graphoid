//! Execution engine
//!
//! This module executes AST nodes.

pub mod environment;
pub mod executor;

pub use environment::Environment;
pub use executor::Executor;

//! Execution engine
//!
//! This module executes AST nodes.

pub mod config;
pub mod environment;
pub mod error_collector;
pub mod executor;
pub mod module_manager;

pub use config::{Config, ConfigStack, ErrorMode, BoundsCheckingMode, TypeCoercionMode, NoneHandlingMode};
pub use environment::Environment;
pub use error_collector::{ErrorCollector, CollectedError};
pub use executor::Executor;

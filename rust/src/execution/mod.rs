//! Execution engine
//!
//! This module executes AST nodes.

pub mod config;
pub mod environment;
pub mod error_collector;
pub mod executor;
pub mod function_graph;
pub mod module_manager;

pub use config::{Config, ConfigStack, ErrorMode, BoundsCheckingMode, TypeCoercionMode, NoneHandlingMode};
pub use environment::Environment;
pub use error_collector::{ErrorCollector, CollectedError};
pub use executor::Executor;
pub use function_graph::{FunctionGraph, FunctionNode, CallEdge, FunctionEdgeType};

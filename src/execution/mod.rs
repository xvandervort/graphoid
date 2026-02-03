//! Execution engine
//!
//! This module executes AST nodes.

pub mod config;
pub mod environment;
pub mod error_collector;
pub mod function_graph;
pub mod module_manager;
pub mod pattern_matcher;

// When graph_execution is enabled, executor.rs is replaced by GraphExecutor.
// The arithmetic and methods modules use conditional Executor type.
pub mod arithmetic;
pub mod methods;

#[cfg(not(feature = "graph_execution"))]
pub mod executor;

pub use config::{Config, ConfigStack, ErrorMode, BoundsCheckingMode, TypeCoercionMode, NoneHandlingMode};

// Phase 15: Conditional Environment type based on feature flag
#[cfg(not(feature = "graph_namespace"))]
pub use environment::Environment;

#[cfg(feature = "graph_namespace")]
pub use crate::namespace::NamespaceGraph as Environment;

pub use error_collector::{ErrorCollector, CollectedError};

// Phase 16: Conditional Executor type based on feature flag
#[cfg(not(feature = "graph_execution"))]
pub use executor::Executor;

#[cfg(feature = "graph_execution")]
pub use crate::execution_graph::graph_executor::GraphExecutor as Executor;

pub use function_graph::{FunctionGraph, FunctionNode, CallEdge, FunctionEdgeType};
pub use pattern_matcher::PatternMatcher;

//! Execution engine
//!
//! This module executes AST nodes via graph traversal (GraphExecutor).

pub mod config;
pub mod error_collector;
pub mod function_graph;
pub mod module_manager;
pub mod pattern_matcher;

// The arithmetic and methods modules provide impl blocks for Executor (= GraphExecutor).
pub mod arithmetic;
pub mod methods;

pub use config::{Config, ConfigStack, ErrorMode, BoundsCheckingMode, TypeCoercionMode, NoneHandlingMode};

// Phase 15: NamespaceGraph is the environment, re-exported as Environment for API compatibility.
pub use crate::namespace::NamespaceGraph as Environment;

pub use error_collector::{ErrorCollector, CollectedError};

// Phase 16: GraphExecutor is the executor, re-exported as Executor for API compatibility.
pub use crate::execution_graph::graph_executor::GraphExecutor as Executor;

pub use function_graph::{FunctionGraph, FunctionNode, CallEdge, FunctionEdgeType};
pub use pattern_matcher::PatternMatcher;

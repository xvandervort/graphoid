//! Graphoid/Glang: A graph-theoretic programming language
//!
//! This crate implements the Graphoid language, where everything is a graph.

pub mod lexer;
pub mod parser;
pub mod ast;
pub mod execution;
pub mod values;
pub mod graph;
pub mod error;

pub use error::{GraphoidError, Result};

//! FFI subsystem (Phase 20)
//!
//! Enables Graphoid programs to load and call C libraries.

pub mod types;
pub mod library;
pub mod calling;
pub mod pointer;
pub mod cdef_parser;
pub mod struct_ops;
pub mod callback;
pub mod limits;

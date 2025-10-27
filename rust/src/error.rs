//! Error types for Graphoid

use std::fmt;

pub type Result<T> = std::result::Result<T, GraphoidError>;

#[derive(Debug, Clone, PartialEq)]
pub struct SourcePosition {
    pub line: usize,
    pub column: usize,
    pub file: Option<String>,
}

impl SourcePosition {
    /// Creates an unknown source position (for internal errors)
    pub fn unknown() -> Self {
        Self {
            line: 0,
            column: 0,
            file: None,
        }
    }
}

impl fmt::Display for SourcePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)?;
        if let Some(ref file) = self.file {
            write!(f, " in {}", file)?;
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GraphoidError {
    #[error("Syntax error: {message} at {position}")]
    SyntaxError {
        message: String,
        position: SourcePosition,
    },

    #[error("Type error: {message} at {position}")]
    TypeError {
        message: String,
        position: SourcePosition,
    },

    #[error("Runtime error: {message}")]
    RuntimeError { message: String },

    #[error("Graph rule violated: {rule} - {message}")]
    RuleViolation { rule: String, message: String },

    #[error("Module not found: '{module}' at {position}")]
    ModuleNotFound {
        module: String,
        position: SourcePosition,
    },

    #[error("I/O error: {message} at {position}")]
    IOError {
        message: String,
        position: SourcePosition,
    },

    #[error("Circular dependency detected at {position}: {}", chain.join(" → "))]
    CircularDependency {
        chain: Vec<String>,
        position: SourcePosition,
    },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl GraphoidError {
    /// Creates a runtime error for undefined variables.
    pub fn undefined_variable(name: &str) -> Self {
        GraphoidError::RuntimeError {
            message: format!("Undefined variable: {}", name),
        }
    }

    /// Creates a runtime error for type mismatches.
    pub fn type_error(expected: &str, actual: &str) -> Self {
        GraphoidError::RuntimeError {
            message: format!("Type error: expected {}, got {}", expected, actual),
        }
    }

    /// Creates a runtime error for division by zero.
    pub fn division_by_zero() -> Self {
        GraphoidError::RuntimeError {
            message: "Division by zero".to_string(),
        }
    }

    /// Creates a runtime error with a custom message.
    pub fn runtime(message: String) -> Self {
        GraphoidError::RuntimeError { message }
    }
}

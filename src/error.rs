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

    #[error("Configuration error: {message}")]
    ConfigError { message: String },

    #[error("Loop control: {control}")]
    LoopControl { control: LoopControlType },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoopControlType {
    Break,
    Continue,
}

impl std::fmt::Display for LoopControlType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoopControlType::Break => write!(f, "break"),
            LoopControlType::Continue => write!(f, "continue"),
        }
    }
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

    /// Returns the type name of the error (e.g., "SyntaxError", "RuntimeError")
    pub fn error_type(&self) -> String {
        match self {
            GraphoidError::SyntaxError { .. } => "SyntaxError".to_string(),
            GraphoidError::TypeError { .. } => "TypeError".to_string(),
            GraphoidError::RuntimeError { .. } => "RuntimeError".to_string(),
            GraphoidError::RuleViolation { .. } => "RuleViolation".to_string(),
            GraphoidError::ModuleNotFound { .. } => "ModuleNotFound".to_string(),
            GraphoidError::IOError { .. } => "IOError".to_string(),
            GraphoidError::CircularDependency { .. } => "CircularDependency".to_string(),
            GraphoidError::IoError(_) => "IOError".to_string(),
            GraphoidError::ConfigError { .. } => "ConfigError".to_string(),
            GraphoidError::LoopControl { .. } => "LoopControl".to_string(),
        }
    }

    /// Returns the source position if available, otherwise returns unknown position
    pub fn position(&self) -> SourcePosition {
        match self {
            GraphoidError::SyntaxError { position, .. } => position.clone(),
            GraphoidError::TypeError { position, .. } => position.clone(),
            GraphoidError::ModuleNotFound { position, .. } => position.clone(),
            GraphoidError::IOError { position, .. } => position.clone(),
            GraphoidError::CircularDependency { position, .. } => position.clone(),
            // Errors without position return unknown
            GraphoidError::RuntimeError { .. } => SourcePosition::unknown(),
            GraphoidError::RuleViolation { .. } => SourcePosition::unknown(),
            GraphoidError::IoError(_) => SourcePosition::unknown(),
            GraphoidError::LoopControl { .. } => SourcePosition::unknown(),
            GraphoidError::ConfigError { .. } => SourcePosition::unknown(),
        }
    }
}

impl Clone for GraphoidError {
    fn clone(&self) -> Self {
        match self {
            GraphoidError::SyntaxError { message, position } => {
                GraphoidError::SyntaxError {
                    message: message.clone(),
                    position: position.clone(),
                }
            }
            GraphoidError::TypeError { message, position } => {
                GraphoidError::TypeError {
                    message: message.clone(),
                    position: position.clone(),
                }
            }
            GraphoidError::RuntimeError { message } => {
                GraphoidError::RuntimeError {
                    message: message.clone(),
                }
            }
            GraphoidError::RuleViolation { rule, message } => {
                GraphoidError::RuleViolation {
                    rule: rule.clone(),
                    message: message.clone(),
                }
            }
            GraphoidError::ModuleNotFound { module, position } => {
                GraphoidError::ModuleNotFound {
                    module: module.clone(),
                    position: position.clone(),
                }
            }
            GraphoidError::IOError { message, position } => {
                GraphoidError::IOError {
                    message: message.clone(),
                    position: position.clone(),
                }
            }
            GraphoidError::CircularDependency { chain, position } => {
                GraphoidError::CircularDependency {
                    chain: chain.clone(),
                    position: position.clone(),
                }
            }
            // Convert IoError to RuntimeError when cloning
            GraphoidError::IoError(e) => {
                GraphoidError::RuntimeError {
                    message: format!("IO error: {}", e),
                }
            }
            GraphoidError::ConfigError { message } => {
                GraphoidError::ConfigError {
                    message: message.clone(),
                }
            }
            GraphoidError::LoopControl { control } => GraphoidError::LoopControl { control: *control },
        }
    }
}

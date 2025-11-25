// Error collector for :collect mode
//
// When error_mode is set to :collect, errors are collected instead of
// immediately halting execution

use crate::error::{GraphoidError, SourcePosition};

/// A collected error with context information
#[derive(Debug, Clone)]
pub struct CollectedError {
    pub error: GraphoidError,
    pub file: Option<String>,
    pub position: SourcePosition,
}

/// Collects errors during execution in :collect mode
pub struct ErrorCollector {
    errors: Vec<CollectedError>,
}

impl ErrorCollector {
    /// Creates a new empty error collector
    pub fn new() -> Self {
        ErrorCollector {
            errors: Vec::new(),
        }
    }

    /// Collects an error with context
    pub fn collect(&mut self, error: GraphoidError, file: Option<String>, position: SourcePosition) {
        self.errors.push(CollectedError { error, file, position });
    }

    /// Gets all collected errors
    pub fn get_errors(&self) -> &[CollectedError] {
        &self.errors
    }

    /// Clears all collected errors
    pub fn clear(&mut self) {
        self.errors.clear();
    }

    /// Checks if there are any collected errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Gets the number of collected errors
    pub fn count(&self) -> usize {
        self.errors.len()
    }
}

impl Default for ErrorCollector {
    fn default() -> Self {
        Self::new()
    }
}

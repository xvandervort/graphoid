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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::GraphoidError;

    fn test_pos() -> SourcePosition {
        SourcePosition {
            line: 1,
            column: 1,
            file: None,
        }
    }

    #[test]
    fn test_new_collector_is_empty() {
        let collector = ErrorCollector::new();
        assert!(!collector.has_errors());
        assert_eq!(collector.count(), 0);
    }

    #[test]
    fn test_collect_error() {
        let mut collector = ErrorCollector::new();
        let error = GraphoidError::runtime("Test error".to_string());

        collector.collect(error, None, test_pos());

        assert!(collector.has_errors());
        assert_eq!(collector.count(), 1);
    }

    #[test]
    fn test_collect_multiple_errors() {
        let mut collector = ErrorCollector::new();

        collector.collect(
            GraphoidError::runtime("Error 1".to_string()),
            None,
            test_pos(),
        );
        collector.collect(
            GraphoidError::runtime("Error 2".to_string()),
            None,
            test_pos(),
        );
        collector.collect(
            GraphoidError::runtime("Error 3".to_string()),
            None,
            test_pos(),
        );

        assert_eq!(collector.count(), 3);
    }

    #[test]
    fn test_clear_errors() {
        let mut collector = ErrorCollector::new();

        collector.collect(
            GraphoidError::runtime("Error 1".to_string()),
            None,
            test_pos(),
        );
        collector.collect(
            GraphoidError::runtime("Error 2".to_string()),
            None,
            test_pos(),
        );

        assert_eq!(collector.count(), 2);

        collector.clear();

        assert!(!collector.has_errors());
        assert_eq!(collector.count(), 0);
    }

    #[test]
    fn test_get_errors() {
        let mut collector = ErrorCollector::new();

        collector.collect(
            GraphoidError::runtime("Error 1".to_string()),
            Some("test.gr".to_string()),
            test_pos(),
        );

        let errors = collector.get_errors();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].file, Some("test.gr".to_string()));
    }
}

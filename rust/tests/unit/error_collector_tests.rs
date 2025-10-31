use graphoid::execution::error_collector::ErrorCollector;
use graphoid::error::{GraphoidError, SourcePosition};

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

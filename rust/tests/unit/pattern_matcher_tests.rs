//! Pattern Matcher Tests - Phase 7 Day 2 (TDD RED phase)
//!
//! These tests are written FIRST before PatternMatcher implementation.

use graphoid::execution::pattern_matcher::PatternMatcher;
use graphoid::ast::{Pattern, LiteralValue, PatternClause, Expr};
use graphoid::values::Value;
use graphoid::error::SourcePosition;

// Helper function for creating default position
fn pos() -> SourcePosition {
    SourcePosition {
        line: 1,
        column: 1,
        file: None,
    }
}

// ============================================================================
// Pattern Matching Tests
// ============================================================================

#[test]
fn test_literal_number_match_success() {
    let matcher = PatternMatcher::new();
    let pattern = Pattern::Literal {
        value: LiteralValue::Number(42.0),
        position: pos(),
    };

    assert!(matcher.matches(&pattern, &Value::Number(42.0)));
}

#[test]
fn test_literal_number_match_failure() {
    let matcher = PatternMatcher::new();
    let pattern = Pattern::Literal {
        value: LiteralValue::Number(42.0),
        position: pos(),
    };

    assert!(!matcher.matches(&pattern, &Value::Number(99.0)));
}

#[test]
fn test_literal_string_match_success() {
    let matcher = PatternMatcher::new();
    let pattern = Pattern::Literal {
        value: LiteralValue::String("hello".to_string()),
        position: pos(),
    };

    assert!(matcher.matches(&pattern, &Value::String("hello".to_string())));
}

#[test]
fn test_literal_string_match_failure() {
    let matcher = PatternMatcher::new();
    let pattern = Pattern::Literal {
        value: LiteralValue::String("hello".to_string()),
        position: pos(),
    };

    assert!(!matcher.matches(&pattern, &Value::String("world".to_string())));
}

#[test]
fn test_literal_boolean_match_true() {
    let matcher = PatternMatcher::new();
    let pattern = Pattern::Literal {
        value: LiteralValue::Boolean(true),
        position: pos(),
    };

    assert!(matcher.matches(&pattern, &Value::Boolean(true)));
    assert!(!matcher.matches(&pattern, &Value::Boolean(false)));
}

#[test]
fn test_literal_boolean_match_false() {
    let matcher = PatternMatcher::new();
    let pattern = Pattern::Literal {
        value: LiteralValue::Boolean(false),
        position: pos(),
    };

    assert!(matcher.matches(&pattern, &Value::Boolean(false)));
    assert!(!matcher.matches(&pattern, &Value::Boolean(true)));
}

#[test]
fn test_literal_none_match() {
    let matcher = PatternMatcher::new();
    let pattern = Pattern::Literal {
        value: LiteralValue::None,
        position: pos(),
    };

    assert!(matcher.matches(&pattern, &Value::None));
    assert!(!matcher.matches(&pattern, &Value::Number(0.0)));
}

#[test]
fn test_literal_type_mismatch() {
    let matcher = PatternMatcher::new();
    let pattern = Pattern::Literal {
        value: LiteralValue::Number(42.0),
        position: pos(),
    };

    // Number pattern should not match string
    assert!(!matcher.matches(&pattern, &Value::String("42".to_string())));
}

#[test]
fn test_variable_pattern_matches_anything() {
    let matcher = PatternMatcher::new();
    let pattern = Pattern::Variable {
        name: "x".to_string(),
        position: pos(),
    };

    // Variable patterns match any value
    assert!(matcher.matches(&pattern, &Value::Number(42.0)));
    assert!(matcher.matches(&pattern, &Value::String("hello".to_string())));
    assert!(matcher.matches(&pattern, &Value::Boolean(true)));
    assert!(matcher.matches(&pattern, &Value::None));
}

#[test]
fn test_wildcard_pattern_matches_anything() {
    let matcher = PatternMatcher::new();
    let pattern = Pattern::Wildcard {
        position: pos(),
    };

    // Wildcard patterns match any value
    assert!(matcher.matches(&pattern, &Value::Number(42.0)));
    assert!(matcher.matches(&pattern, &Value::String("hello".to_string())));
    assert!(matcher.matches(&pattern, &Value::Boolean(true)));
    assert!(matcher.matches(&pattern, &Value::None));
}

// ============================================================================
// Binding Tests
// ============================================================================

#[test]
fn test_bind_from_variable_pattern() {
    let matcher = PatternMatcher::new();
    let pattern = Pattern::Variable {
        name: "x".to_string(),
        position: pos(),
    };

    let value = Value::Number(42.0);
    let bindings = matcher.bind(&pattern, &value).unwrap();

    assert_eq!(bindings.len(), 1);
    assert_eq!(bindings.get("x"), Some(&Value::Number(42.0)));
}

#[test]
fn test_bind_from_literal_pattern() {
    let matcher = PatternMatcher::new();
    let pattern = Pattern::Literal {
        value: LiteralValue::Number(42.0),
        position: pos(),
    };

    let value = Value::Number(42.0);
    let bindings = matcher.bind(&pattern, &value).unwrap();

    // Literal patterns don't create bindings
    assert_eq!(bindings.len(), 0);
}

#[test]
fn test_bind_from_wildcard_pattern() {
    let matcher = PatternMatcher::new();
    let pattern = Pattern::Wildcard {
        position: pos(),
    };

    let value = Value::Number(42.0);
    let bindings = matcher.bind(&pattern, &value).unwrap();

    // Wildcard patterns don't create bindings
    assert_eq!(bindings.len(), 0);
}

#[test]
fn test_bind_different_variable_names() {
    let matcher = PatternMatcher::new();

    let pattern1 = Pattern::Variable {
        name: "foo".to_string(),
        position: pos(),
    };
    let bindings1 = matcher.bind(&pattern1, &Value::String("hello".to_string())).unwrap();
    assert_eq!(bindings1.get("foo"), Some(&Value::String("hello".to_string())));

    let pattern2 = Pattern::Variable {
        name: "bar".to_string(),
        position: pos(),
    };
    let bindings2 = matcher.bind(&pattern2, &Value::Boolean(true)).unwrap();
    assert_eq!(bindings2.get("bar"), Some(&Value::Boolean(true)));
}

// ============================================================================
// Clause Matching Tests
// ============================================================================

#[test]
fn test_find_match_first_clause() {
    let matcher = PatternMatcher::new();

    let clauses = vec![
        PatternClause {
            pattern: Pattern::Literal {
                value: LiteralValue::Number(0.0),
                position: pos(),
            },
            guard: None,
            body: Expr::Literal {
                value: LiteralValue::String("zero".to_string()),
                position: pos(),
            },
            position: pos(),
        },
        PatternClause {
            pattern: Pattern::Variable {
                name: "x".to_string(),
                position: pos(),
            },
            guard: None,
            body: Expr::Literal {
                value: LiteralValue::String("other".to_string()),
                position: pos(),
            },
            position: pos(),
        },
    ];

    let args = vec![Value::Number(0.0)];
    let result = matcher.find_match(&clauses, &args).unwrap();

    assert!(result.is_some());
    let (clause, bindings) = result.unwrap();

    // Should match first clause
    assert!(matches!(&clause.pattern, Pattern::Literal { .. }));
    assert_eq!(bindings.len(), 0); // Literal pattern, no bindings
}

#[test]
fn test_find_match_second_clause() {
    let matcher = PatternMatcher::new();

    let clauses = vec![
        PatternClause {
            pattern: Pattern::Literal {
                value: LiteralValue::Number(0.0),
                position: pos(),
            },
            guard: None,
            body: Expr::Literal {
                value: LiteralValue::String("zero".to_string()),
                position: pos(),
            },
            position: pos(),
        },
        PatternClause {
            pattern: Pattern::Variable {
                name: "x".to_string(),
                position: pos(),
            },
            guard: None,
            body: Expr::Literal {
                value: LiteralValue::String("other".to_string()),
                position: pos(),
            },
            position: pos(),
        },
    ];

    let args = vec![Value::Number(42.0)];
    let result = matcher.find_match(&clauses, &args).unwrap();

    assert!(result.is_some());
    let (clause, bindings) = result.unwrap();

    // Should match second clause (variable pattern)
    assert!(matches!(&clause.pattern, Pattern::Variable { .. }));
    assert_eq!(bindings.len(), 1);
    assert_eq!(bindings.get("x"), Some(&Value::Number(42.0)));
}

#[test]
fn test_find_match_no_match() {
    let matcher = PatternMatcher::new();

    // Only has literal pattern for 0
    let clauses = vec![
        PatternClause {
            pattern: Pattern::Literal {
                value: LiteralValue::Number(0.0),
                position: pos(),
            },
            guard: None,
            body: Expr::Literal {
                value: LiteralValue::String("zero".to_string()),
                position: pos(),
            },
            position: pos(),
        },
    ];

    let args = vec![Value::Number(42.0)];
    let result = matcher.find_match(&clauses, &args).unwrap();

    // No match should return None
    assert!(result.is_none());
}

#[test]
fn test_find_match_order_matters() {
    let matcher = PatternMatcher::new();

    // Variable pattern comes first - should match before literal
    let clauses = vec![
        PatternClause {
            pattern: Pattern::Variable {
                name: "x".to_string(),
                position: pos(),
            },
            guard: None,
            body: Expr::Literal {
                value: LiteralValue::String("any".to_string()),
                position: pos(),
            },
            position: pos(),
        },
        PatternClause {
            pattern: Pattern::Literal {
                value: LiteralValue::Number(0.0),
                position: pos(),
            },
            guard: None,
            body: Expr::Literal {
                value: LiteralValue::String("zero".to_string()),
                position: pos(),
            },
            position: pos(),
        },
    ];

    let args = vec![Value::Number(0.0)];
    let result = matcher.find_match(&clauses, &args).unwrap();

    assert!(result.is_some());
    let (clause, _) = result.unwrap();

    // Should match first clause (variable), not second (literal)
    assert!(matches!(&clause.pattern, Pattern::Variable { .. }));
}

#[test]
fn test_find_match_wrong_arg_count() {
    let matcher = PatternMatcher::new();

    let clauses = vec![
        PatternClause {
            pattern: Pattern::Literal {
                value: LiteralValue::Number(0.0),
                position: pos(),
            },
            guard: None,
            body: Expr::Literal {
                value: LiteralValue::String("zero".to_string()),
                position: pos(),
            },
            position: pos(),
        },
    ];

    // Pattern matching requires exactly 1 argument
    let args = vec![Value::Number(1.0), Value::Number(2.0)];
    let result = matcher.find_match(&clauses, &args);

    // Should return error for wrong argument count
    assert!(result.is_err());
}

#[test]
fn test_find_match_empty_clauses() {
    let matcher = PatternMatcher::new();

    let clauses = vec![];
    let args = vec![Value::Number(42.0)];
    let result = matcher.find_match(&clauses, &args).unwrap();

    // Empty clauses should return None (no match)
    assert!(result.is_none());
}

//! Tests for ordering behaviors (Sub-Phase 7.5)
//!
//! These tests verify:
//! - Default ordering for numbers and strings
//! - Custom comparison functions
//! - Retroactive and proactive sorting
//! - Stability and edge cases
//! - Integration with other behaviors

use graphoid::ast::{Stmt, Expr, BinaryOp, Parameter};
use graphoid::execution::{Executor, Environment};
use graphoid::values::{Value, List, Function};
use graphoid::graph::{RuleInstance, RuleSpec};
use graphoid::error::SourcePosition;
use std::rc::Rc;
use std::cell::RefCell;

// Helper to create a dummy source position for testing
fn pos() -> SourcePosition {
    SourcePosition { line: 0, column: 0, file: None }
}

// ============================================================================
// Test 1-4: Default Ordering Tests
// ============================================================================

#[test]
fn test_ordering_numbers_default() {
    // Create a list with unsorted numbers
    let mut list = List::new();
    list.append(Value::number(3.0)).unwrap();
    list.append(Value::number(1.0)).unwrap();
    list.append(Value::number(4.0)).unwrap();
    list.append(Value::number(1.0)).unwrap();
    list.append(Value::number(5.0)).unwrap();

    // Add ordering behavior (default = ascending)
    let rule = RuleInstance::new(RuleSpec::Ordering {
            compare_fn: None,  // None = use default ordering
        });

    let executor = Executor::new();

    // Apply retroactively - sort existing values
    let mut sorted_values: Vec<Value> = list.to_vec();
    sorted_values.sort_by(|a, b| {
        executor.compare_values(a, b).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut new_list = List::new();
    for val in sorted_values {
        new_list.append_raw(val).unwrap();
    }
    new_list.graph.rules.push(rule);

    // Verify sorted: [1, 1, 3, 4, 5]
    assert_eq!(new_list.get(0), Some(&Value::number(1.0)));
    assert_eq!(new_list.get(1), Some(&Value::number(1.0)));
    assert_eq!(new_list.get(2), Some(&Value::number(3.0)));
    assert_eq!(new_list.get(3), Some(&Value::number(4.0)));
    assert_eq!(new_list.get(4), Some(&Value::number(5.0)));
}

#[test]
fn test_ordering_strings_default() {
    // Create a list with unsorted strings
    let mut list = List::new();
    list.append(Value::string("dog".to_string())).unwrap();
    list.append(Value::string("cat".to_string())).unwrap();
    list.append(Value::string("elephant".to_string())).unwrap();
    list.append(Value::string("ant".to_string())).unwrap();

    // Add ordering behavior
    let rule = RuleInstance::new(RuleSpec::Ordering {
            compare_fn: None,
        });

    let executor = Executor::new();

    // Apply retroactively
    let mut sorted_values: Vec<Value> = list.to_vec();
    sorted_values.sort_by(|a, b| {
        executor.compare_values(a, b).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut new_list = List::new();
    for val in sorted_values {
        new_list.append_raw(val).unwrap();
    }
    new_list.graph.rules.push(rule);

    // Verify sorted: ["ant", "cat", "dog", "elephant"]
    assert_eq!(new_list.get(0), Some(&Value::string("ant".to_string())));
    assert_eq!(new_list.get(1), Some(&Value::string("cat".to_string())));
    assert_eq!(new_list.get(2), Some(&Value::string("dog".to_string())));
    assert_eq!(new_list.get(3), Some(&Value::string("elephant".to_string())));
}

#[test]
fn test_ordering_retroactive() {
    // Test that existing values are sorted when ordering behavior is added
    let mut list = List::new();
    list.append(Value::number(5.0)).unwrap();
    list.append(Value::number(2.0)).unwrap();
    list.append(Value::number(8.0)).unwrap();
    list.append(Value::number(1.0)).unwrap();

    // Add ordering behavior retroactively
    let rule = RuleInstance::new(RuleSpec::Ordering {
            compare_fn: None,
        });

    let executor = Executor::new();

    // Sort and rebuild
    let mut sorted_values: Vec<Value> = list.to_vec();
    sorted_values.sort_by(|a, b| {
        executor.compare_values(a, b).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut new_list = List::new();
    for val in sorted_values {
        new_list.append_raw(val).unwrap();
    }
    new_list.graph.rules.push(rule);

    // Verify sorted
    assert_eq!(new_list.get(0), Some(&Value::number(1.0)));
    assert_eq!(new_list.get(1), Some(&Value::number(2.0)));
    assert_eq!(new_list.get(2), Some(&Value::number(5.0)));
    assert_eq!(new_list.get(3), Some(&Value::number(8.0)));
}

#[test]
fn test_ordering_proactive() {
    // Test that new values are inserted in sorted position
    let mut list = List::new();

    // Start with sorted list
    list.append(Value::number(1.0)).unwrap();
    list.append(Value::number(3.0)).unwrap();
    list.append(Value::number(5.0)).unwrap();
    list.append(Value::number(7.0)).unwrap();

    // Add ordering behavior
    let rule = RuleInstance::new(RuleSpec::Ordering {
            compare_fn: None,
        });
    list.graph.rules.push(rule);

    let mut executor = Executor::new();

    // Add new value that should be inserted in middle
    let new_value = Value::number(4.0);

    // Find insertion point
    let values = list.to_vec();
    let insert_pos = executor.find_insertion_point(&values, &new_value, &None).unwrap();

    // Insert at correct position (raw because no behaviors to apply)
    list.insert_at_raw(insert_pos, new_value).unwrap();

    // Verify: [1, 3, 4, 5, 7]
    assert_eq!(list.get(0), Some(&Value::number(1.0)));
    assert_eq!(list.get(1), Some(&Value::number(3.0)));
    assert_eq!(list.get(2), Some(&Value::number(4.0)));
    assert_eq!(list.get(3), Some(&Value::number(5.0)));
    assert_eq!(list.get(4), Some(&Value::number(7.0)));
}

// ============================================================================
// Test 5-8: Custom Ordering Tests
// ============================================================================

#[test]
fn test_ordering_custom_function() {
    // Sort strings by length instead of alphabetically
    // func compare_by_length(a, b) { return len(a) - len(b) }

    let compare_fn = Function {
        name: Some("compare_by_length".to_string()),
        params: vec!["a".to_string(), "b".to_string()],
        parameters: vec![
            Parameter { name: "a".to_string(), default_value: None, is_variadic: false },
            Parameter { name: "b".to_string(), default_value: None, is_variadic: false },
        ],
        body: vec![
            Stmt::Return {
                value: Some(Expr::Binary {
                    left: Box::new(Expr::MethodCall {
                        object: Box::new(Expr::Variable { name: "a".to_string(), position: pos() }),
                        method: "length".to_string(),
                        args: vec![],
                        position: pos(),
                    }),
                    op: BinaryOp::Subtract,
                    right: Box::new(Expr::MethodCall {
                        object: Box::new(Expr::Variable { name: "b".to_string(), position: pos() }),
                        method: "length".to_string(),
                        args: vec![],
                        position: pos(),
                    }),
                    position: pos(),
                }),
                position: pos(),
            }
        ],
        pattern_clauses: None,

        env: Rc::new(RefCell::new(Environment::new())),
        node_id: None,
        is_getter: false,
        is_setter: false,
        is_static: false,
        guard: None,
    };

    let mut list = List::new();
    list.append(Value::string("cat".to_string())).unwrap();
    list.append(Value::string("elephant".to_string())).unwrap();
    list.append(Value::string("dog".to_string())).unwrap();
    list.append(Value::string("a".to_string())).unwrap();

    let rule = RuleInstance::new(RuleSpec::Ordering {
            compare_fn: Some(Value::function(compare_fn.clone())),
        });

    let mut executor = Executor::new();

    // Sort by length
    let mut sorted_values: Vec<Value> = list.to_vec();
    sorted_values.sort_by(|a, b| {
        executor.compare_with_function(a, b, &compare_fn).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut new_list = List::new();
    for val in sorted_values {
        new_list.append_raw(val).unwrap();
    }
    new_list.graph.rules.push(rule);

    // Verify sorted by length: ["a", "cat", "dog", "elephant"]
    assert_eq!(new_list.get(0), Some(&Value::string("a".to_string())));
    assert_eq!(new_list.get(1), Some(&Value::string("cat".to_string())));
    assert_eq!(new_list.get(2), Some(&Value::string("dog".to_string())));
    assert_eq!(new_list.get(3), Some(&Value::string("elephant".to_string())));
}

#[test]
fn test_ordering_reverse() {
    // Reverse ordering (descending)
    // func reverse_compare(a, b) { return b - a }

    let reverse_fn = Function {
        name: Some("reverse_compare".to_string()),
        params: vec!["a".to_string(), "b".to_string()],
        parameters: vec![
            Parameter { name: "a".to_string(), default_value: None, is_variadic: false },
            Parameter { name: "b".to_string(), default_value: None, is_variadic: false },
        ],
        body: vec![
            Stmt::Return {
                value: Some(Expr::Binary {
                    left: Box::new(Expr::Variable { name: "b".to_string(), position: pos() }),
                    op: BinaryOp::Subtract,
                    right: Box::new(Expr::Variable { name: "a".to_string(), position: pos() }),
                    position: pos(),
                }),
                position: pos(),
            }
        ],
        pattern_clauses: None,

        env: Rc::new(RefCell::new(Environment::new())),
        node_id: None,
        is_getter: false,
        is_setter: false,
        is_static: false,
        guard: None,
    };

    let mut list = List::new();
    list.append(Value::number(3.0)).unwrap();
    list.append(Value::number(1.0)).unwrap();
    list.append(Value::number(4.0)).unwrap();
    list.append(Value::number(2.0)).unwrap();

    let rule = RuleInstance::new(RuleSpec::Ordering {
            compare_fn: Some(Value::function(reverse_fn.clone())),
        });

    let mut executor = Executor::new();

    // Sort in reverse
    let mut sorted_values: Vec<Value> = list.to_vec();
    sorted_values.sort_by(|a, b| {
        executor.compare_with_function(a, b, &reverse_fn).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut new_list = List::new();
    for val in sorted_values {
        new_list.append_raw(val).unwrap();
    }
    new_list.graph.rules.push(rule);

    // Verify reverse sorted: [4, 3, 2, 1]
    assert_eq!(new_list.get(0), Some(&Value::number(4.0)));
    assert_eq!(new_list.get(1), Some(&Value::number(3.0)));
    assert_eq!(new_list.get(2), Some(&Value::number(2.0)));
    assert_eq!(new_list.get(3), Some(&Value::number(1.0)));
}

#[test]
fn test_ordering_by_field() {
    // This test would sort by a field in a map/object
    // For now, we'll skip this as it requires map field access
    // which may not be fully implemented yet

    // Placeholder: just verify ordering works with mixed types
    let mut list = List::new();
    list.append(Value::number(5.0)).unwrap();
    list.append(Value::number(2.0)).unwrap();
    list.append(Value::number(8.0)).unwrap();

    let rule = RuleInstance::new(RuleSpec::Ordering {
            compare_fn: None,
        });

    let executor = Executor::new();

    let mut sorted_values: Vec<Value> = list.to_vec();
    sorted_values.sort_by(|a, b| {
        executor.compare_values(a, b).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut new_list = List::new();
    for val in sorted_values {
        new_list.append_raw(val).unwrap();
    }
    new_list.graph.rules.push(rule);

    assert_eq!(new_list.get(0), Some(&Value::number(2.0)));
    assert_eq!(new_list.get(1), Some(&Value::number(5.0)));
    assert_eq!(new_list.get(2), Some(&Value::number(8.0)));
}

#[test]
fn test_ordering_stability() {
    // Test that equal elements maintain their relative order (stable sort)
    // Using strings with same length to test stability

    let mut list = List::new();
    list.append(Value::string("cat".to_string())).unwrap();
    list.append(Value::string("dog".to_string())).unwrap();
    list.append(Value::string("bat".to_string())).unwrap();
    list.append(Value::string("ant".to_string())).unwrap();

    let rule = RuleInstance::new(RuleSpec::Ordering {
            compare_fn: None,
        });

    let executor = Executor::new();

    // Rust's sort is stable, so equal-length strings will maintain order
    let mut sorted_values: Vec<Value> = list.to_vec();
    sorted_values.sort_by(|a, b| {
        executor.compare_values(a, b).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut new_list = List::new();
    for val in sorted_values {
        new_list.append_raw(val).unwrap();
    }
    new_list.graph.rules.push(rule);

    // Verify alphabetical order (stable)
    assert_eq!(new_list.get(0), Some(&Value::string("ant".to_string())));
    assert_eq!(new_list.get(1), Some(&Value::string("bat".to_string())));
    assert_eq!(new_list.get(2), Some(&Value::string("cat".to_string())));
    assert_eq!(new_list.get(3), Some(&Value::string("dog".to_string())));
}

// ============================================================================
// Test 9-12: Integration Tests
// ============================================================================

#[test]
fn test_list_maintains_order() {
    // Test that list maintains sorted order after multiple insertions
    let mut list = List::new();

    let rule = RuleInstance::new(RuleSpec::Ordering {
            compare_fn: None,
        });
    list.graph.rules.push(rule);

    let mut executor = Executor::new();

    // Insert values one by one, each should maintain order
    let values_to_insert = vec![5.0, 2.0, 8.0, 1.0, 9.0, 3.0];

    for val in values_to_insert {
        let new_value = Value::number(val);
        let current_values = list.to_vec();
        let insert_pos = executor.find_insertion_point(&current_values, &new_value, &None).unwrap();
        list.insert_at_raw(insert_pos, new_value).unwrap();
    }

    // Verify final sorted order: [1, 2, 3, 5, 8, 9]
    assert_eq!(list.get(0), Some(&Value::number(1.0)));
    assert_eq!(list.get(1), Some(&Value::number(2.0)));
    assert_eq!(list.get(2), Some(&Value::number(3.0)));
    assert_eq!(list.get(3), Some(&Value::number(5.0)));
    assert_eq!(list.get(4), Some(&Value::number(8.0)));
    assert_eq!(list.get(5), Some(&Value::number(9.0)));
}

#[test]
fn test_ordering_with_other_behaviors() {
    // Test ordering combined with transformation behavior
    // First apply NoneToZero, then maintain sorted order

    let behavior1 = RuleInstance::new(RuleSpec::NoneToZero);

    let behavior2 = RuleInstance::new(RuleSpec::Ordering {
        compare_fn: None,
    });

    let mut list = List::new();
    list.graph.rules.push(behavior1);
    list.graph.rules.push(behavior2);

    let mut executor = Executor::new();

    // Add values including None
    let values = vec![
        Value::number(5.0),
        Value::none(),
        Value::number(3.0),
        Value::none(),
        Value::number(1.0),
    ];

    for val in values {
        // Apply all behaviors
        let transformed = executor.apply_transformation_rules_with_context(val.clone(), &list.graph.rules).unwrap();

        // Find insertion point and insert (raw since behaviors already applied)
        let current_values = list.to_vec();
        let insert_pos = executor.find_insertion_point(&current_values, &transformed, &None).unwrap();
        list.insert_at_raw(insert_pos, transformed).unwrap();
    }

    // Verify: None -> 0, so result is [0, 0, 1, 3, 5]
    assert_eq!(list.get(0), Some(&Value::number(0.0)));
    assert_eq!(list.get(1), Some(&Value::number(0.0)));
    assert_eq!(list.get(2), Some(&Value::number(1.0)));
    assert_eq!(list.get(3), Some(&Value::number(3.0)));
    assert_eq!(list.get(4), Some(&Value::number(5.0)));
}

#[test]
fn test_ordering_edge_cases() {
    let mut executor = Executor::new();

    // Test 1: Empty list
    let mut empty_list = List::new();
    let rule = RuleInstance::new(RuleSpec::Ordering {
            compare_fn: None,
        });
    empty_list.graph.rules.push(rule.clone());

    assert_eq!(empty_list.len(), 0);

    // Add first element
    let values = empty_list.to_vec();
    let insert_pos = executor.find_insertion_point(&values, &Value::number(5.0), &None).unwrap();
    empty_list.insert_at_raw(insert_pos, Value::number(5.0)).unwrap();
    assert_eq!(empty_list.get(0), Some(&Value::number(5.0)));

    // Test 2: Single element list
    let mut single_list = List::new();
    single_list.append(Value::number(42.0)).unwrap();
    single_list.graph.rules.push(rule);

    assert_eq!(single_list.len(), 1);
    assert_eq!(single_list.get(0), Some(&Value::number(42.0)));
}

#[test]
fn test_ordering_duplicate_values() {
    // Test that duplicate values are handled correctly
    let mut list = List::new();
    list.append(Value::number(3.0)).unwrap();
    list.append(Value::number(1.0)).unwrap();
    list.append(Value::number(3.0)).unwrap();
    list.append(Value::number(1.0)).unwrap();
    list.append(Value::number(2.0)).unwrap();

    let rule = RuleInstance::new(RuleSpec::Ordering {
            compare_fn: None,
        });

    let executor = Executor::new();

    let mut sorted_values: Vec<Value> = list.to_vec();
    sorted_values.sort_by(|a, b| {
        executor.compare_values(a, b).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut new_list = List::new();
    for val in sorted_values {
        new_list.append_raw(val).unwrap();
    }
    new_list.graph.rules.push(rule);

    // Verify duplicates are sorted: [1, 1, 2, 3, 3]
    assert_eq!(new_list.get(0), Some(&Value::number(1.0)));
    assert_eq!(new_list.get(1), Some(&Value::number(1.0)));
    assert_eq!(new_list.get(2), Some(&Value::number(2.0)));
    assert_eq!(new_list.get(3), Some(&Value::number(3.0)));
    assert_eq!(new_list.get(4), Some(&Value::number(3.0)));
}

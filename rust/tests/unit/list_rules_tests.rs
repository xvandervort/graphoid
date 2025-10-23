//! Tests for rules on List (everything is a graph!)
//!
//! These tests verify that lists, being graphs internally, can use the full rule system.

use graphoid::values::{Value, List};
use graphoid::graph::{RuleSpec, RuleInstance};
use graphoid::error::GraphoidError;

#[test]
fn test_list_has_add_rule_method() {
    // Verify List has add_rule() method (because it's a graph)
    let mut list = List::new();
    list.add_rule(RuleInstance::new(RuleSpec::NoDuplicates)).unwrap();

    assert!(list.has_rule("no_duplicates"));
}

#[test]
fn test_list_no_duplicates_prevents_duplicate_addition() {
    // Create list with no_duplicates rule
    let mut list = List::new();
    list.add_rule(RuleInstance::new(RuleSpec::NoDuplicates)).unwrap();

    // Add first item - should succeed
    assert!(list.append(Value::Number(1.0)).is_ok());
    assert!(list.append(Value::Number(2.0)).is_ok());

    // Try to add duplicate - should fail
    let result = list.append(Value::Number(1.0));
    assert!(result.is_err());

    match result {
        Err(GraphoidError::RuleViolation { rule, message }) => {
            assert_eq!(rule, "no_duplicates");
            assert!(message.contains("already exists"));
        }
        _ => panic!("Expected RuleViolation"),
    }
}

#[test]
fn test_list_without_no_duplicates_allows_duplicates() {
    // Regular list without rule allows duplicates
    let mut list = List::new();

    assert!(list.append(Value::Number(1.0)).is_ok());
    assert!(list.append(Value::Number(1.0)).is_ok());
    assert!(list.append(Value::Number(1.0)).is_ok());

    assert_eq!(list.len(), 3);
}

#[test]
fn test_list_no_duplicates_with_strings() {
    let mut list = List::new();
    list.add_rule(RuleInstance::new(RuleSpec::NoDuplicates)).unwrap();

    assert!(list.append(Value::String("hello".to_string())).is_ok());
    assert!(list.append(Value::String("world".to_string())).is_ok());

    // Duplicate string should fail
    let result = list.append(Value::String("hello".to_string()));
    assert!(result.is_err());
}

#[test]
fn test_list_can_remove_rule() {
    let mut list = List::new();
    list.add_rule(RuleInstance::new(RuleSpec::NoDuplicates)).unwrap();

    list.append(Value::Number(1.0)).unwrap();

    // Can't add duplicate
    assert!(list.append(Value::Number(1.0)).is_err());

    // Remove the rule
    list.remove_rule(&RuleSpec::NoDuplicates);

    // Now duplicate should be allowed
    assert!(list.append(Value::Number(1.0)).is_ok());
    assert_eq!(list.len(), 2);
}

#[test]
fn test_list_add_rule_to_existing_list_with_duplicates() {
    // Create list with duplicates
    let mut list = List::from_vec(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ]);

    assert_eq!(list.len(), 4);

    // Add no_duplicates rule with retroactive cleaning
    list.add_rule(RuleInstance::new(RuleSpec::NoDuplicates)).unwrap();

    // The rule is now active
    assert!(list.has_rule("no_duplicates"));

    // Retroactive cleaning removed the duplicate "2"
    // List now has 3 items: [1, 2, 3] (second 2 was removed)
    assert_eq!(list.len(), 3);

    // Future duplicate additions should be blocked
    let result = list.append(Value::Number(2.0));
    assert!(result.is_err());
}

#[test]
fn test_list_unique_values_like_a_set() {
    // Demonstrate using a list as a set with no_duplicates
    let mut unique_items = List::new();
    unique_items.add_rule(RuleInstance::new(RuleSpec::NoDuplicates)).unwrap();

    // Try to add several items, some duplicates
    let items = vec![1.0, 2.0, 3.0, 2.0, 4.0, 1.0, 5.0];

    for val in items {
        let _ = unique_items.append(Value::Number(val)); // Ignore errors
    }

    // Only unique values were added
    assert_eq!(unique_items.len(), 5); // 1, 2, 3, 4, 5
}

#[test]
fn test_multiple_rules_on_list() {
    // Lists can have multiple rules (because they're graphs)
    let mut list = List::new();
    list.add_rule(RuleInstance::new(RuleSpec::NoDuplicates)).unwrap();
    list.add_rule(RuleInstance::new(RuleSpec::MaxDegree(1))).unwrap(); // Linear structure

    assert!(list.has_rule("no_duplicates"));
    assert!(list.has_rule("max_degree"));

    // Both rules are enforced
    list.append(Value::Number(1.0)).unwrap();

    // Duplicate should fail (no_duplicates)
    assert!(list.append(Value::Number(1.0)).is_err());

    // Adding to middle would violate max_degree (lists are linear graphs)
    // This is implicitly enforced by the list's append implementation
}

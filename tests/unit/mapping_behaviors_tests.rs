//! Tests for Sub-Phase 7.3: Mapping Behaviors
//!
//! This test file covers hash-based value mapping behaviors:
//! - Map values using a hash table
//! - Default fallback for unmapped keys
//! - Works with different value types
//! - Retroactive and proactive application
//!
//! TDD Red Phase: These tests are written FIRST and should FAIL initially.

use graphoid::graph::{RuleSpec, RuleInstance};
use graphoid::values::{Value, List};
use graphoid::error::GraphoidError;
use std::collections::HashMap;

// Helper function to apply transformation rules in sequence
fn apply_rules(value: Value, rules: &[RuleInstance]) -> Result<Value, GraphoidError> {
    let mut current = value;
    for rule_instance in rules {
        let rule = rule_instance.spec.instantiate();
        current = rule.transform(&current)?;
    }
    Ok(current)
}

// ============================================================================
// Basic Mapping Tests (4 tests)
// ============================================================================

#[test]
fn test_mapping_basic_string_to_number() {
    // Map strings to numbers: "active" → 1, "inactive" → 0
    let mut mapping = HashMap::new();
    mapping.insert("active".to_string(), Value::number(1.0));
    mapping.insert("inactive".to_string(), Value::number(0.0));

    let rule = RuleSpec::Mapping {
        mapping,
        default: Value::number(-1.0),
    };

    let rules = vec![RuleInstance::new(rule)];

    // Test mapped value
    let result = apply_rules(Value::string("active".to_string()), &rules).unwrap();
    assert_eq!(result, Value::number(1.0));

    let result = apply_rules(Value::string("inactive".to_string()), &rules).unwrap();
    assert_eq!(result, Value::number(0.0));
}

#[test]
fn test_mapping_with_default_fallback() {
    // Unmapped values use default
    let mut mapping = HashMap::new();
    mapping.insert("active".to_string(), Value::number(1.0));

    let rule = RuleSpec::Mapping {
        mapping,
        default: Value::number(-1.0),
    };

    let rules = vec![RuleInstance::new(rule)];

    // Test unmapped value (should use default)
    let result = apply_rules(Value::string("unknown".to_string()), &rules).unwrap();
    assert_eq!(result, Value::number(-1.0));
}

#[test]
fn test_mapping_all_values_mapped() {
    // All values in mapping, no defaults needed
    let mut mapping = HashMap::new();
    mapping.insert("red".to_string(), Value::number(1.0));
    mapping.insert("green".to_string(), Value::number(2.0));
    mapping.insert("blue".to_string(), Value::number(3.0));

    let rule = RuleSpec::Mapping {
        mapping,
        default: Value::number(0.0),
    };

    let rules = vec![RuleInstance::new(rule)];

    // All should map correctly
    let result = apply_rules(Value::string("red".to_string()), &rules).unwrap();
    assert_eq!(result, Value::number(1.0));

    let result = apply_rules(Value::string("green".to_string()), &rules).unwrap();
    assert_eq!(result, Value::number(2.0));

    let result = apply_rules(Value::string("blue".to_string()), &rules).unwrap();
    assert_eq!(result, Value::number(3.0));
}

#[test]
fn test_mapping_all_values_unmapped() {
    // All values use default (empty mapping)
    let mapping = HashMap::new();

    let rule = RuleSpec::Mapping {
        mapping,
        default: Value::string("default".to_string()),
    };

    let rules = vec![RuleInstance::new(rule)];

    // All should use default
    let result = apply_rules(Value::string("anything".to_string()), &rules).unwrap();
    assert_eq!(result, Value::string("default".to_string()));

    let result = apply_rules(Value::number(42.0), &rules).unwrap();
    assert_eq!(result, Value::string("default".to_string()));
}

// ============================================================================
// Edge Case Tests (3 tests)
// ============================================================================

#[test]
fn test_mapping_empty_hash() {
    // Empty mapping, all values use default
    let mapping = HashMap::new();

    let rule = RuleSpec::Mapping {
        mapping,
        default: Value::none(),
    };

    let rules = vec![RuleInstance::new(rule)];

    let result = apply_rules(Value::string("test".to_string()), &rules).unwrap();
    assert_eq!(result, Value::none());
}

#[test]
fn test_mapping_none_values() {
    // Map none to a specific value
    let mut mapping = HashMap::new();
    mapping.insert("none".to_string(), Value::number(0.0));

    let rule = RuleSpec::Mapping {
        mapping,
        default: Value::number(-1.0),
    };

    let rules = vec![RuleInstance::new(rule)];

    let result = apply_rules(Value::none(), &rules).unwrap();
    assert_eq!(result, Value::number(0.0));
}

#[test]
fn test_mapping_number_to_string() {
    // Map numbers to strings
    let mut mapping = HashMap::new();
    mapping.insert("1".to_string(), Value::string("one".to_string()));
    mapping.insert("2".to_string(), Value::string("two".to_string()));
    mapping.insert("3".to_string(), Value::string("three".to_string()));

    let rule = RuleSpec::Mapping {
        mapping,
        default: Value::string("unknown".to_string()),
    };

    let rules = vec![RuleInstance::new(rule)];

    let result = apply_rules(Value::number(1.0), &rules).unwrap();
    assert_eq!(result, Value::string("one".to_string()));

    let result = apply_rules(Value::number(2.0), &rules).unwrap();
    assert_eq!(result, Value::string("two".to_string()));

    // Unmapped number uses default
    let result = apply_rules(Value::number(99.0), &rules).unwrap();
    assert_eq!(result, Value::string("unknown".to_string()));
}

// ============================================================================
// Integration Tests (3 tests)
// ============================================================================

#[test]
fn test_list_with_mapping_rule() {
    // List transformation with mapping
    let mut list = List::new();
    list.append(Value::string("active".to_string())).unwrap();
    list.append(Value::string("unknown".to_string())).unwrap();
    list.append(Value::string("inactive".to_string())).unwrap();

    // Create mapping
    let mut mapping = HashMap::new();
    mapping.insert("active".to_string(), Value::number(1.0));
    mapping.insert("inactive".to_string(), Value::number(0.0));

    let rule = RuleSpec::Mapping {
        mapping,
        default: Value::number(-1.0),
    };

    // Add behavior - should transform retroactively
    list.add_rule(RuleInstance::new(rule)).unwrap();

    // Check transformed values
    assert_eq!(list.get(0).unwrap(), &Value::number(1.0));  // active → 1
    assert_eq!(list.get(1).unwrap(), &Value::number(-1.0)); // unknown → -1 (default)
    assert_eq!(list.get(2).unwrap(), &Value::number(0.0));  // inactive → 0

    // Add new value - should be transformed proactively
    list.append(Value::string("active".to_string())).unwrap();
    assert_eq!(list.get(3).unwrap(), &Value::number(1.0));
}

#[test]
fn test_hash_with_mapping_rule() {
    // Hash value transformation with mapping
    use graphoid::values::Hash;

    let mut hash = Hash::new();
    hash.insert("status1".to_string(), Value::string("active".to_string())).unwrap();
    hash.insert("status2".to_string(), Value::string("pending".to_string())).unwrap();

    // Create mapping
    let mut mapping = HashMap::new();
    mapping.insert("active".to_string(), Value::number(1.0));
    mapping.insert("inactive".to_string(), Value::number(0.0));
    mapping.insert("pending".to_string(), Value::number(2.0));

    let rule = RuleSpec::Mapping {
        mapping,
        default: Value::number(-1.0),
    };

    // Add behavior - should transform retroactively
    hash.add_rule(RuleInstance::new(rule)).unwrap();

    // Check transformed values
    assert_eq!(hash.get("status1").unwrap(), &Value::number(1.0));
    assert_eq!(hash.get("status2").unwrap(), &Value::number(2.0));

    // Add new value - should be transformed proactively
    hash.insert("status3".to_string(), Value::string("inactive".to_string())).unwrap();
    assert_eq!(hash.get("status3").unwrap(), &Value::number(0.0));
}

#[test]
fn test_mapping_retroactive_application() {
    // Existing values transformed with mapping
    let mut list = List::new();
    list.append(Value::string("cat".to_string())).unwrap();
    list.append(Value::string("dog".to_string())).unwrap();
    list.append(Value::string("bird".to_string())).unwrap();

    // Create mapping for animals
    let mut mapping = HashMap::new();
    mapping.insert("cat".to_string(), Value::string("meow".to_string()));
    mapping.insert("dog".to_string(), Value::string("woof".to_string()));
    mapping.insert("bird".to_string(), Value::string("tweet".to_string()));

    let rule = RuleSpec::Mapping {
        mapping,
        default: Value::string("unknown".to_string()),
    };

    // Add behavior - retroactive application
    list.add_rule(RuleInstance::new(rule)).unwrap();

    // All existing values should be transformed
    assert_eq!(list.get(0).unwrap(), &Value::string("meow".to_string()));
    assert_eq!(list.get(1).unwrap(), &Value::string("woof".to_string()));
    assert_eq!(list.get(2).unwrap(), &Value::string("tweet".to_string()));
}

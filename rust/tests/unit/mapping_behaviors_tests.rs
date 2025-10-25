//! Tests for Sub-Phase 7.3: Mapping Behaviors
//!
//! This test file covers hash-based value mapping behaviors:
//! - Map values using a hash table
//! - Default fallback for unmapped keys
//! - Works with different value types
//! - Retroactive and proactive application
//!
//! TDD Red Phase: These tests are written FIRST and should FAIL initially.

use graphoid::graph::behaviors::{BehaviorSpec, BehaviorInstance, apply_behaviors};
use graphoid::values::{Value, List};
use std::collections::HashMap;

// ============================================================================
// Basic Mapping Tests (4 tests)
// ============================================================================

#[test]
fn test_mapping_basic_string_to_number() {
    // Map strings to numbers: "active" → 1, "inactive" → 0
    let mut mapping = HashMap::new();
    mapping.insert("active".to_string(), Value::Number(1.0));
    mapping.insert("inactive".to_string(), Value::Number(0.0));

    let behavior = BehaviorSpec::Mapping {
        mapping,
        default: Value::Number(-1.0),
    };

    let behaviors = vec![BehaviorInstance::new(behavior)];

    // Test mapped value
    let result = apply_behaviors(Value::String("active".to_string()), &behaviors).unwrap();
    assert_eq!(result, Value::Number(1.0));

    let result = apply_behaviors(Value::String("inactive".to_string()), &behaviors).unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_mapping_with_default_fallback() {
    // Unmapped values use default
    let mut mapping = HashMap::new();
    mapping.insert("active".to_string(), Value::Number(1.0));

    let behavior = BehaviorSpec::Mapping {
        mapping,
        default: Value::Number(-1.0),
    };

    let behaviors = vec![BehaviorInstance::new(behavior)];

    // Test unmapped value (should use default)
    let result = apply_behaviors(Value::String("unknown".to_string()), &behaviors).unwrap();
    assert_eq!(result, Value::Number(-1.0));
}

#[test]
fn test_mapping_all_values_mapped() {
    // All values in mapping, no defaults needed
    let mut mapping = HashMap::new();
    mapping.insert("red".to_string(), Value::Number(1.0));
    mapping.insert("green".to_string(), Value::Number(2.0));
    mapping.insert("blue".to_string(), Value::Number(3.0));

    let behavior = BehaviorSpec::Mapping {
        mapping,
        default: Value::Number(0.0),
    };

    let behaviors = vec![BehaviorInstance::new(behavior)];

    // All should map correctly
    let result = apply_behaviors(Value::String("red".to_string()), &behaviors).unwrap();
    assert_eq!(result, Value::Number(1.0));

    let result = apply_behaviors(Value::String("green".to_string()), &behaviors).unwrap();
    assert_eq!(result, Value::Number(2.0));

    let result = apply_behaviors(Value::String("blue".to_string()), &behaviors).unwrap();
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn test_mapping_all_values_unmapped() {
    // All values use default (empty mapping)
    let mapping = HashMap::new();

    let behavior = BehaviorSpec::Mapping {
        mapping,
        default: Value::String("default".to_string()),
    };

    let behaviors = vec![BehaviorInstance::new(behavior)];

    // All should use default
    let result = apply_behaviors(Value::String("anything".to_string()), &behaviors).unwrap();
    assert_eq!(result, Value::String("default".to_string()));

    let result = apply_behaviors(Value::Number(42.0), &behaviors).unwrap();
    assert_eq!(result, Value::String("default".to_string()));
}

// ============================================================================
// Edge Case Tests (3 tests)
// ============================================================================

#[test]
fn test_mapping_empty_hash() {
    // Empty mapping, all values use default
    let mapping = HashMap::new();

    let behavior = BehaviorSpec::Mapping {
        mapping,
        default: Value::None,
    };

    let behaviors = vec![BehaviorInstance::new(behavior)];

    let result = apply_behaviors(Value::String("test".to_string()), &behaviors).unwrap();
    assert_eq!(result, Value::None);
}

#[test]
fn test_mapping_none_values() {
    // Map none to a specific value
    let mut mapping = HashMap::new();
    mapping.insert("none".to_string(), Value::Number(0.0));

    let behavior = BehaviorSpec::Mapping {
        mapping,
        default: Value::Number(-1.0),
    };

    let behaviors = vec![BehaviorInstance::new(behavior)];

    let result = apply_behaviors(Value::None, &behaviors).unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_mapping_number_to_string() {
    // Map numbers to strings
    let mut mapping = HashMap::new();
    mapping.insert("1".to_string(), Value::String("one".to_string()));
    mapping.insert("2".to_string(), Value::String("two".to_string()));
    mapping.insert("3".to_string(), Value::String("three".to_string()));

    let behavior = BehaviorSpec::Mapping {
        mapping,
        default: Value::String("unknown".to_string()),
    };

    let behaviors = vec![BehaviorInstance::new(behavior)];

    let result = apply_behaviors(Value::Number(1.0), &behaviors).unwrap();
    assert_eq!(result, Value::String("one".to_string()));

    let result = apply_behaviors(Value::Number(2.0), &behaviors).unwrap();
    assert_eq!(result, Value::String("two".to_string()));

    // Unmapped number uses default
    let result = apply_behaviors(Value::Number(99.0), &behaviors).unwrap();
    assert_eq!(result, Value::String("unknown".to_string()));
}

// ============================================================================
// Integration Tests (3 tests)
// ============================================================================

#[test]
fn test_list_with_mapping_rule() {
    // List transformation with mapping
    let mut list = List::new();
    list.append(Value::String("active".to_string())).unwrap();
    list.append(Value::String("unknown".to_string())).unwrap();
    list.append(Value::String("inactive".to_string())).unwrap();

    // Create mapping
    let mut mapping = HashMap::new();
    mapping.insert("active".to_string(), Value::Number(1.0));
    mapping.insert("inactive".to_string(), Value::Number(0.0));

    let behavior = BehaviorSpec::Mapping {
        mapping,
        default: Value::Number(-1.0),
    };

    // Add behavior - should transform retroactively
    list.add_behavior(BehaviorInstance::new(behavior)).unwrap();

    // Check transformed values
    assert_eq!(list.get(0).unwrap(), &Value::Number(1.0));  // active → 1
    assert_eq!(list.get(1).unwrap(), &Value::Number(-1.0)); // unknown → -1 (default)
    assert_eq!(list.get(2).unwrap(), &Value::Number(0.0));  // inactive → 0

    // Add new value - should be transformed proactively
    list.append(Value::String("active".to_string())).unwrap();
    assert_eq!(list.get(3).unwrap(), &Value::Number(1.0));
}

#[test]
fn test_hash_with_mapping_rule() {
    // Hash value transformation with mapping
    use graphoid::values::Hash;

    let mut hash = Hash::new();
    hash.insert("status1".to_string(), Value::String("active".to_string())).unwrap();
    hash.insert("status2".to_string(), Value::String("pending".to_string())).unwrap();

    // Create mapping
    let mut mapping = HashMap::new();
    mapping.insert("active".to_string(), Value::Number(1.0));
    mapping.insert("inactive".to_string(), Value::Number(0.0));
    mapping.insert("pending".to_string(), Value::Number(2.0));

    let behavior = BehaviorSpec::Mapping {
        mapping,
        default: Value::Number(-1.0),
    };

    // Add behavior - should transform retroactively
    hash.add_behavior(BehaviorInstance::new(behavior)).unwrap();

    // Check transformed values
    assert_eq!(hash.get("status1").unwrap(), &Value::Number(1.0));
    assert_eq!(hash.get("status2").unwrap(), &Value::Number(2.0));

    // Add new value - should be transformed proactively
    hash.insert("status3".to_string(), Value::String("inactive".to_string())).unwrap();
    assert_eq!(hash.get("status3").unwrap(), &Value::Number(0.0));
}

#[test]
fn test_mapping_retroactive_application() {
    // Existing values transformed with mapping
    let mut list = List::new();
    list.append(Value::String("cat".to_string())).unwrap();
    list.append(Value::String("dog".to_string())).unwrap();
    list.append(Value::String("bird".to_string())).unwrap();

    // Create mapping for animals
    let mut mapping = HashMap::new();
    mapping.insert("cat".to_string(), Value::String("meow".to_string()));
    mapping.insert("dog".to_string(), Value::String("woof".to_string()));
    mapping.insert("bird".to_string(), Value::String("tweet".to_string()));

    let behavior = BehaviorSpec::Mapping {
        mapping,
        default: Value::String("unknown".to_string()),
    };

    // Add behavior - retroactive application
    list.add_behavior(BehaviorInstance::new(behavior)).unwrap();

    // All existing values should be transformed
    assert_eq!(list.get(0).unwrap(), &Value::String("meow".to_string()));
    assert_eq!(list.get(1).unwrap(), &Value::String("woof".to_string()));
    assert_eq!(list.get(2).unwrap(), &Value::String("tweet".to_string()));
}

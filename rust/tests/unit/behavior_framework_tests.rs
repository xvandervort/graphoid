//! Tests for Sub-Phase 7.1: Behavior Framework
//!
//! This test file covers the foundational infrastructure for behaviors:
//! - Behavior trait and BehaviorSpec
//! - BehaviorInstance with RetroactivePolicy
//! - Storage in collections (List, Hash, Graph)
//! - Application logic (apply_behaviors, retroactive application)
//!
//! TDD Red Phase: These tests are written FIRST and should FAIL initially.

use graphoid::graph::behaviors::{BehaviorSpec, BehaviorInstance, apply_behaviors};
use graphoid::graph::rules::RetroactivePolicy;
use graphoid::values::{Value, List, Hash, Graph, GraphType};

// ============================================================================
// Framework Tests (10 tests)
// ============================================================================

#[test]
fn test_behavior_spec_from_symbol() {
    // Parse :none_to_zero symbol
    let spec = BehaviorSpec::from_symbol("none_to_zero");
    assert!(spec.is_some());
    assert_eq!(spec.unwrap(), BehaviorSpec::NoneToZero);

    // Parse :uppercase symbol
    let spec = BehaviorSpec::from_symbol("uppercase");
    assert!(spec.is_some());
    assert_eq!(spec.unwrap(), BehaviorSpec::Uppercase);

    // Unknown symbol returns None
    let spec = BehaviorSpec::from_symbol("unknown_behavior");
    assert!(spec.is_none());
}

#[test]
fn test_behavior_spec_name() {
    // Get name from spec
    let spec = BehaviorSpec::NoneToZero;
    assert_eq!(spec.name(), "none_to_zero");

    let spec = BehaviorSpec::Positive;
    assert_eq!(spec.name(), "positive");

    let spec = BehaviorSpec::Uppercase;
    assert_eq!(spec.name(), "uppercase");
}

#[test]
fn test_behavior_instance_creation() {
    // Create BehaviorInstance with default policy (Clean)
    let spec = BehaviorSpec::NoneToZero;
    let instance = BehaviorInstance::new(spec.clone());

    assert_eq!(instance.spec, spec);
    assert_eq!(instance.retroactive_policy, RetroactivePolicy::Clean);
}

#[test]
fn test_behavior_instance_with_policy() {
    // Create with specific RetroactivePolicy
    let spec = BehaviorSpec::Positive;
    let instance = BehaviorInstance::with_policy(spec.clone(), RetroactivePolicy::Warn);

    assert_eq!(instance.spec, spec);
    assert_eq!(instance.retroactive_policy, RetroactivePolicy::Warn);

    // Try Enforce policy
    let instance = BehaviorInstance::with_policy(
        BehaviorSpec::RoundToInt,
        RetroactivePolicy::Enforce
    );
    assert_eq!(instance.retroactive_policy, RetroactivePolicy::Enforce);
}

#[test]
fn test_apply_behaviors_empty_list() {
    // No behaviors = no change
    let value = Value::Number(42.5);
    let behaviors: Vec<BehaviorInstance> = vec![];

    let result = apply_behaviors(value.clone(), &behaviors).unwrap();
    assert_eq!(result, Value::Number(42.5));
}

#[test]
fn test_apply_behaviors_sequence() {
    // Multiple behaviors applied in order
    // Note: We'll implement actual transformations in Sub-Phase 7.2
    // For now, we're just testing the framework can apply behaviors sequentially
    let value = Value::None;
    let behaviors = vec![
        BehaviorInstance::new(BehaviorSpec::NoneToZero),
    ];

    // This will work once NoneToZero behavior is implemented in 7.2
    // For now, test should compile and framework should exist
    let result = apply_behaviors(value, &behaviors);
    assert!(result.is_ok());
}

#[test]
fn test_apply_behaviors_skip_non_applicable() {
    // Skip behaviors that don't apply to the value type
    // Uppercase only applies to strings, so it should skip numbers
    let value = Value::Number(42.0);
    let behaviors = vec![
        BehaviorInstance::new(BehaviorSpec::Uppercase),
    ];

    let result = apply_behaviors(value.clone(), &behaviors).unwrap();
    // Number should be unchanged (uppercase doesn't apply)
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_behavior_transform_returns_value() {
    // Test that transform() method exists and returns a Value
    let spec = BehaviorSpec::NoneToZero;
    let behavior = spec.instantiate();

    let value = Value::None;
    let result = behavior.transform(&value);

    // Should return a Result<Value, GraphoidError>
    assert!(result.is_ok());
}

#[test]
fn test_behavior_applies_to_filters_types() {
    // Test that applies_to() method works
    let spec = BehaviorSpec::Uppercase;
    let behavior = spec.instantiate();

    // Uppercase applies to strings
    let string_value = Value::String("hello".to_string());
    assert!(behavior.applies_to(&string_value));

    // Uppercase should NOT apply to numbers
    let number_value = Value::Number(42.0);
    assert!(!behavior.applies_to(&number_value));
}

#[test]
fn test_behavior_application_order_matters() {
    // First added = first applied
    // We'll verify this more thoroughly in Sub-Phase 7.2 with actual transformations
    let value = Value::None;

    // Create behaviors in specific order
    let behaviors = vec![
        BehaviorInstance::new(BehaviorSpec::NoneToZero),
        BehaviorInstance::new(BehaviorSpec::Positive),
    ];

    // Should apply in order: none_to_zero first, then positive
    let result = apply_behaviors(value, &behaviors);
    assert!(result.is_ok());
}

// ============================================================================
// Storage Tests (3 tests)
// ============================================================================

#[test]
fn test_list_has_behaviors_field() {
    // List should store behaviors
    let list = List::new();

    // List should have a behaviors field (we'll access it via methods later)
    // For now, just verify List can be created
    assert_eq!(list.len(), 0);

    // In Sub-Phase 7.2, we'll add get_behaviors() accessor
}

#[test]
fn test_hash_has_behaviors_field() {
    // Hash should store behaviors
    let hash = Hash::new();

    // Verify Hash can be created
    assert_eq!(hash.len(), 0);

    // In Sub-Phase 7.2, we'll add get_behaviors() accessor
}

#[test]
fn test_graph_has_behaviors_field() {
    // Graph should store behaviors
    let graph = Graph::new(GraphType::Directed);

    // Verify Graph can be created
    assert_eq!(graph.node_count(), 0);

    // In Sub-Phase 7.2, we'll add get_behaviors() accessor
}

// ============================================================================
// RetroactivePolicy Tests (4 tests)
// ============================================================================

#[test]
fn test_retroactive_policy_clean() {
    // Clean: Transform all existing values
    use graphoid::graph::behaviors::apply_retroactive_to_list;

    let mut list = List::new();
    list.append(Value::None).unwrap();
    list.append(Value::None).unwrap();
    list.append(Value::Number(42.0)).unwrap();

    // Add behavior with Clean policy (default)
    let behavior = BehaviorInstance::new(BehaviorSpec::NoneToZero);

    // Apply retroactively
    let result = apply_retroactive_to_list(&mut list, &behavior);
    assert!(result.is_ok());

    // After implementation in 7.2, the None values should be transformed to 0
    // For now, just verify the function exists and doesn't error
}

#[test]
fn test_retroactive_policy_warn() {
    // Warn: Keep existing values, print warnings
    use graphoid::graph::behaviors::apply_retroactive_to_list;

    let mut list = List::new();
    list.append(Value::None).unwrap();
    list.append(Value::Number(42.0)).unwrap();

    // Add behavior with Warn policy
    let behavior = BehaviorInstance::with_policy(
        BehaviorSpec::NoneToZero,
        RetroactivePolicy::Warn
    );

    // Apply retroactively - should warn but not transform
    let result = apply_retroactive_to_list(&mut list, &behavior);
    assert!(result.is_ok());

    // Values should be unchanged
    assert_eq!(list.get(0).unwrap(), &Value::None);
}

#[test]
fn test_retroactive_policy_enforce() {
    // Enforce: Error if any values would be transformed
    use graphoid::graph::behaviors::apply_retroactive_to_list;

    let mut list = List::new();
    list.append(Value::None).unwrap();
    list.append(Value::Number(42.0)).unwrap();

    // Add behavior with Enforce policy
    let behavior = BehaviorInstance::with_policy(
        BehaviorSpec::NoneToZero,
        RetroactivePolicy::Enforce
    );

    // Apply retroactively - should error because None would be transformed
    let result = apply_retroactive_to_list(&mut list, &behavior);
    assert!(result.is_err());

    // Error message should mention the behavior and Enforce policy
    let err = result.unwrap_err();
    let msg = format!("{}", err);
    assert!(msg.contains("none_to_zero") || msg.contains("Enforce"));
}

#[test]
fn test_retroactive_policy_ignore() {
    // Ignore: Don't check or transform existing values
    use graphoid::graph::behaviors::apply_retroactive_to_list;

    let mut list = List::new();
    list.append(Value::None).unwrap();
    list.append(Value::Number(42.0)).unwrap();

    // Add behavior with Ignore policy
    let behavior = BehaviorInstance::with_policy(
        BehaviorSpec::NoneToZero,
        RetroactivePolicy::Ignore
    );

    // Apply retroactively - should succeed without transformation
    let result = apply_retroactive_to_list(&mut list, &behavior);
    assert!(result.is_ok());

    // Values should be unchanged
    assert_eq!(list.get(0).unwrap(), &Value::None);
}

// ============================================================================
// Proactive Application Test (1 test)
// ============================================================================

#[test]
fn test_proactive_application_to_new_values() {
    // New values should be transformed when added
    // This is a placeholder test - full implementation in Sub-Phase 7.2
    // when we wire behaviors into append/insert operations

    let list = List::new();

    // For now, just verify we can create a list
    // In 7.2, we'll add:
    // list.add_behavior(BehaviorInstance::new(BehaviorSpec::NoneToZero));
    // list.append(Value::None).unwrap();
    // assert_eq!(list.get(0).unwrap(), &Value::Number(0.0));

    assert_eq!(list.len(), 0);
}

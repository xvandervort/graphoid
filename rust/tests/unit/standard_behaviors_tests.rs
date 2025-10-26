//! Tests for Sub-Phase 7.2: Standard Behaviors
//!
//! This test file covers the 7 standard built-in behaviors:
//! - NoneToZero, NoneToEmpty
//! - Positive, RoundToInt
//! - Uppercase, Lowercase
//! - ValidateRange
//!
//! Tests verify:
//! - Basic transformations work correctly
//! - Edge cases handled properly
//! - Integration with collections (List, Hash)
//! - Retroactive and proactive application
//!
//! TDD Red Phase: These tests are written FIRST and should FAIL initially.

use graphoid::graph::{RuleSpec, RuleInstance};
use graphoid::values::{Value, List, Hash};
use graphoid::error::GraphoidError;

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
// Basic Transformation Tests (7 tests)
// ============================================================================

#[test]
fn test_none_to_zero_transforms_none() {
    // none → 0
    let value = Value::None;
    let rules = vec![RuleInstance::new(RuleSpec::NoneToZero)];

    let result = apply_rules(value, &rules).unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_none_to_empty_transforms_none() {
    // none → ""
    let value = Value::None;
    let rules = vec![RuleInstance::new(RuleSpec::NoneToEmpty)];

    let result = apply_rules(value, &rules).unwrap();
    assert_eq!(result, Value::String(String::new()));
}

#[test]
fn test_positive_makes_negative_positive() {
    // -5 → 5
    let value = Value::Number(-5.0);
    let rules = vec![RuleInstance::new(RuleSpec::Positive)];

    let result = apply_rules(value, &rules).unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_round_to_int_rounds_numbers() {
    // 3.7 → 4.0, 3.2 → 3.0
    let rules = vec![RuleInstance::new(RuleSpec::RoundToInt)];

    let result = apply_rules(Value::Number(3.7), &rules).unwrap();
    assert_eq!(result, Value::Number(4.0));

    let result = apply_rules(Value::Number(3.2), &rules).unwrap();
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn test_uppercase_converts_string() {
    // "hello" → "HELLO"
    let value = Value::String("hello".to_string());
    let rules = vec![RuleInstance::new(RuleSpec::Uppercase)];

    let result = apply_rules(value, &rules).unwrap();
    assert_eq!(result, Value::String("HELLO".to_string()));
}

#[test]
fn test_lowercase_converts_string() {
    // "HELLO" → "hello"
    let value = Value::String("HELLO".to_string());
    let rules = vec![RuleInstance::new(RuleSpec::Lowercase)];

    let result = apply_rules(value, &rules).unwrap();
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_validate_range_clamps_numbers() {
    // 110 → 100 (range 0-100)
    // -10 → 0 (range 0-100)
    let rules = vec![RuleInstance::new(RuleSpec::ValidateRange {
        min: 0.0,
        max: 100.0,
    })];

    let result = apply_rules(Value::Number(110.0), &rules).unwrap();
    assert_eq!(result, Value::Number(100.0));

    let result = apply_rules(Value::Number(-10.0), &rules).unwrap();
    assert_eq!(result, Value::Number(0.0));
}

// ============================================================================
// Edge Case Tests (7 tests)
// ============================================================================

#[test]
fn test_none_to_zero_ignores_non_none() {
    // Numbers unchanged
    let value = Value::Number(42.0);
    let rules = vec![RuleInstance::new(RuleSpec::NoneToZero)];

    let result = apply_rules(value.clone(), &rules).unwrap();
    assert_eq!(result, value);

    // Strings also unchanged
    let value = Value::String("test".to_string());
    let result = apply_rules(value.clone(), &rules).unwrap();
    assert_eq!(result, value);
}

#[test]
fn test_positive_ignores_already_positive() {
    // 5 → 5 (no change)
    let value = Value::Number(5.0);
    let rules = vec![RuleInstance::new(RuleSpec::Positive)];

    let result = apply_rules(value, &rules).unwrap();
    assert_eq!(result, Value::Number(5.0));

    // Zero also unchanged
    let value = Value::Number(0.0);
    let result = apply_rules(value, &rules).unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_uppercase_ignores_non_strings() {
    // Numbers unchanged
    let value = Value::Number(42.0);
    let rules = vec![RuleInstance::new(RuleSpec::Uppercase)];

    let result = apply_rules(value.clone(), &rules).unwrap();
    assert_eq!(result, value);

    // None also unchanged
    let value = Value::None;
    let result = apply_rules(value.clone(), &rules).unwrap();
    assert_eq!(result, value);
}

#[test]
fn test_validate_range_within_range() {
    // 50 → 50 (no clamping needed)
    let rules = vec![RuleInstance::new(RuleSpec::ValidateRange {
        min: 0.0,
        max: 100.0,
    })];

    let result = apply_rules(Value::Number(50.0), &rules).unwrap();
    assert_eq!(result, Value::Number(50.0));

    // Edge values
    let result = apply_rules(Value::Number(0.0), &rules).unwrap();
    assert_eq!(result, Value::Number(0.0));

    let result = apply_rules(Value::Number(100.0), &rules).unwrap();
    assert_eq!(result, Value::Number(100.0));
}

#[test]
fn test_none_to_empty_only_affects_none() {
    // Strings unchanged
    let value = Value::String("hello".to_string());
    let rules = vec![RuleInstance::new(RuleSpec::NoneToEmpty)];

    let result = apply_rules(value.clone(), &rules).unwrap();
    assert_eq!(result, value);

    // Numbers also unchanged
    let value = Value::Number(42.0);
    let result = apply_rules(value.clone(), &rules).unwrap();
    assert_eq!(result, value);
}

#[test]
fn test_round_to_int_already_integer() {
    // 5.0 → 5.0 (no change)
    let value = Value::Number(5.0);
    let rules = vec![RuleInstance::new(RuleSpec::RoundToInt)];

    let result = apply_rules(value, &rules).unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_lowercase_empty_string() {
    // "" → ""
    let value = Value::String(String::new());
    let rules = vec![RuleInstance::new(RuleSpec::Lowercase)];

    let result = apply_rules(value, &rules).unwrap();
    assert_eq!(result, Value::String(String::new()));
}

// ============================================================================
// Integration Tests (6 tests)
// ============================================================================

#[test]
fn test_multiple_behaviors_chain() {
    // none → 0 → abs() → round()
    // Start with none, convert to 0, make positive (no change), round (no change)
    let rules = vec![
        RuleInstance::new(RuleSpec::NoneToZero),
        RuleInstance::new(RuleSpec::Positive),
        RuleInstance::new(RuleSpec::RoundToInt),
    ];

    let result = apply_rules(Value::None, &rules).unwrap();
    assert_eq!(result, Value::Number(0.0));

    // More interesting: -3.7 → positive → round
    let rules = vec![
        RuleInstance::new(RuleSpec::Positive),
        RuleInstance::new(RuleSpec::RoundToInt),
    ];

    let result = apply_rules(Value::Number(-3.7), &rules).unwrap();
    assert_eq!(result, Value::Number(4.0)); // -3.7 → 3.7 → 4.0
}

#[test]
fn test_behavior_order_matters() {
    // Verify first-added = first-applied
    // none → 0 (NoneToZero) → clamp(5, 10) → 5
    let rules = vec![
        RuleInstance::new(RuleSpec::NoneToZero),
        RuleInstance::new(RuleSpec::ValidateRange { min: 5.0, max: 10.0 }),
    ];

    let result = apply_rules(Value::None, &rules).unwrap();
    assert_eq!(result, Value::Number(5.0)); // none → 0 → clamped to 5

    // Different order: clamp first (skips none), then none_to_zero
    // Since ValidateRange only applies to numbers, it skips none
    let rules = vec![
        RuleInstance::new(RuleSpec::ValidateRange { min: 5.0, max: 10.0 }),
        RuleInstance::new(RuleSpec::NoneToZero),
    ];

    let result = apply_rules(Value::None, &rules).unwrap();
    assert_eq!(result, Value::Number(0.0)); // clamp skips → none_to_zero → 0
}

#[test]
fn test_list_with_none_to_zero() {
    // List of [none, 1, none] with add_rule should transform to [0, 1, 0]
    // This test requires add_rule() method to be implemented
    let mut list = List::new();
    list.append(Value::None).unwrap();
    list.append(Value::Number(1.0)).unwrap();
    list.append(Value::None).unwrap();

    // Add behavior - should transform retroactively
    let rule = RuleInstance::new(RuleSpec::NoneToZero);
    list.add_rule(rule).unwrap();

    // Check that existing values were transformed
    assert_eq!(list.get(0).unwrap(), &Value::Number(0.0));
    assert_eq!(list.get(1).unwrap(), &Value::Number(1.0));
    assert_eq!(list.get(2).unwrap(), &Value::Number(0.0));

    // Now append a new none - should be transformed proactively
    list.append(Value::None).unwrap();
    assert_eq!(list.get(3).unwrap(), &Value::Number(0.0));
}

#[test]
fn test_hash_with_uppercase() {
    // Hash values uppercased with add_rule
    let mut hash = Hash::new();
    hash.insert("name".to_string(), Value::String("alice".to_string())).unwrap();
    hash.insert("city".to_string(), Value::String("boston".to_string())).unwrap();

    // Add behavior - should transform retroactively
    let rule = RuleInstance::new(RuleSpec::Uppercase);
    hash.add_rule(rule).unwrap();

    // Check that existing values were transformed
    assert_eq!(hash.get("name").unwrap(), &Value::String("ALICE".to_string()));
    assert_eq!(hash.get("city").unwrap(), &Value::String("BOSTON".to_string()));

    // Now insert a new value - should be transformed proactively
    hash.insert("country".to_string(), Value::String("usa".to_string())).unwrap();
    assert_eq!(hash.get("country").unwrap(), &Value::String("USA".to_string()));
}

#[test]
fn test_retroactive_clean_transforms() {
    // Existing values transformed with RetroactivePolicy::Clean (default)
    let mut list = List::new();
    list.append(Value::String("hello".to_string())).unwrap();
    list.append(Value::String("world".to_string())).unwrap();

    // Add behavior with Clean policy (default)
    let rule = RuleInstance::new(RuleSpec::Uppercase);
    list.add_rule(rule).unwrap();

    // All existing values should be transformed
    assert_eq!(list.get(0).unwrap(), &Value::String("HELLO".to_string()));
    assert_eq!(list.get(1).unwrap(), &Value::String("WORLD".to_string()));
}

#[test]
fn test_proactive_on_append() {
    // New values transformed on append
    let mut list = List::new();

    // Add behavior first (no existing values)
    let rule = RuleInstance::new(RuleSpec::Positive);
    list.add_rule(rule).unwrap();

    // Now append negative numbers - should be transformed
    list.append(Value::Number(-5.0)).unwrap();
    list.append(Value::Number(-10.0)).unwrap();
    list.append(Value::Number(3.0)).unwrap();

    // Check that values were transformed
    assert_eq!(list.get(0).unwrap(), &Value::Number(5.0));
    assert_eq!(list.get(1).unwrap(), &Value::Number(10.0));
    assert_eq!(list.get(2).unwrap(), &Value::Number(3.0));
}

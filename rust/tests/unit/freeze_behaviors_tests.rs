//! Tests for Freeze System (Sub-Phase 8: Freeze Control)
//!
//! This test file covers the freeze/immutability system:
//! - Value::freeze(), is_frozen(), deep_copy_unfrozen()
//! - no_frozen behavior (reject frozen elements)
//! - copy_elements behavior (deep copy unfrozen)
//! - shallow_freeze_only behavior
//! - Freeze predicates (:frozen, :unfrozen)
//!
//! TDD RED Phase: These tests are written FIRST and should FAIL initially.

use graphoid::values::{Value, List, Hash};
use graphoid::graph::{RuleSpec, RuleInstance};
use graphoid::error::GraphoidError;

// ============================================================================
// Basic Freeze/Unfreeze Tests (4 tests)
// ============================================================================

#[test]
fn test_value_can_be_frozen() {
    // Value starts unfrozen, can be frozen
    let mut value = Value::number(42.0);
    assert!(!value.is_frozen());

    value.freeze();
    assert!(value.is_frozen());
}

#[test]
fn test_string_can_be_frozen() {
    let mut value = Value::string("hello".to_string());
    assert!(!value.is_frozen());

    value.freeze();
    assert!(value.is_frozen());
}

#[test]
fn test_list_can_be_frozen() {
    let mut list = List::new();
    list.append(Value::number(1.0)).unwrap();
    list.append(Value::number(2.0)).unwrap();

    let mut value = Value::list(list);
    assert!(!value.is_frozen());

    value.freeze();
    assert!(value.is_frozen());
}

#[test]
fn test_deep_copy_unfrozen_creates_mutable_copy() {
    // Frozen value can be copied as unfrozen
    let mut value = Value::number(42.0);
    value.freeze();
    assert!(value.is_frozen());

    let copy = value.deep_copy_unfrozen();
    assert!(!copy.is_frozen()); // Copy is NOT frozen
    assert_eq!(value, copy);    // But values are equal
}

// ============================================================================
// no_frozen Behavior Tests (3 tests)
// ============================================================================

#[test]
fn test_no_frozen_rejects_frozen_value() {
    // no_frozen behavior should reject frozen values
    let mut list = List::new();
    list.add_rule(RuleInstance::new(RuleSpec::NoFrozen)).unwrap();

    let mut frozen_value = Value::number(42.0);
    frozen_value.freeze();

    let result = list.append(frozen_value);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("frozen"));
}

#[test]
fn test_no_frozen_allows_unfrozen_value() {
    // no_frozen behavior should allow unfrozen values
    let mut list = List::new();
    list.add_rule(RuleInstance::new(RuleSpec::NoFrozen)).unwrap();

    let unfrozen_value = Value::number(42.0);
    assert!(!unfrozen_value.is_frozen());

    let result = list.append(unfrozen_value);
    assert!(result.is_ok());
    assert_eq!(list.len(), 1);
}

#[test]
fn test_no_frozen_retroactive_rejects_existing_frozen() {
    // Adding no_frozen retroactively should reject existing frozen elements
    let mut list = List::new();

    let mut frozen_value = Value::number(42.0);
    frozen_value.freeze();
    list.append(frozen_value.clone()).unwrap();

    // Now add no_frozen rule (retroactive)
    let result = list.add_rule(RuleInstance::new(RuleSpec::NoFrozen));
    assert!(result.is_err()); // Should fail because list contains frozen element
}

// ============================================================================
// copy_elements Behavior Tests (3 tests)
// ============================================================================

#[test]
fn test_copy_elements_creates_unfrozen_copies() {
    // copy_elements behavior should create unfrozen copies
    let mut list = List::new();

    let mut frozen_value = Value::number(42.0);
    frozen_value.freeze();
    list.append(frozen_value.clone()).unwrap();

    // Add copy_elements behavior
    list.add_rule(RuleInstance::new(RuleSpec::CopyElements)).unwrap();

    // Get first element - should be unfrozen copy
    let first = list.get(0).unwrap();
    assert!(!first.is_frozen()); // Should be unfrozen
    assert_eq!(first, &Value::number(42.0));
}

#[test]
fn test_copy_elements_on_append() {
    // copy_elements should copy elements when appending
    let mut list = List::new();
    list.add_rule(RuleInstance::new(RuleSpec::CopyElements)).unwrap();

    let mut frozen_value = Value::number(42.0);
    frozen_value.freeze();

    list.append(frozen_value.clone()).unwrap();

    // Element in list should be unfrozen copy
    let first = list.get(0).unwrap();
    assert!(!first.is_frozen());
}

#[test]
fn test_copy_elements_deep_copies_nested() {
    // copy_elements should deep copy nested structures
    let mut inner_list = List::new();
    inner_list.append(Value::number(1.0)).unwrap();

    let mut inner_value = Value::list(inner_list);
    inner_value.freeze();

    let mut outer_list = List::new();
    outer_list.add_rule(RuleInstance::new(RuleSpec::CopyElements)).unwrap();
    outer_list.append(inner_value).unwrap();

    // Nested list should be unfrozen
    let first = outer_list.get(0).unwrap();
    assert!(!first.is_frozen());
}

// ============================================================================
// shallow_freeze_only Behavior Tests (2 tests)
// ============================================================================

#[test]
fn test_shallow_freeze_only_freezes_collection_not_elements() {
    // shallow_freeze_only should freeze collection but not elements
    let mut list = List::new();
    list.append(Value::number(1.0)).unwrap();
    list.append(Value::number(2.0)).unwrap();

    // Apply shallow_freeze_only
    list.add_rule(RuleInstance::new(RuleSpec::ShallowFreezeOnly)).unwrap();

    // Collection itself should be frozen
    let list_value = Value::list(list.clone());
    assert!(list_value.is_frozen());

    // But elements should NOT be frozen
    let first = list.get(0).unwrap();
    assert!(!first.is_frozen());
}

#[test]
fn test_shallow_freeze_prevents_modification() {
    // Frozen collection should prevent modifications
    let mut list = List::new();
    list.append(Value::number(1.0)).unwrap();

    list.add_rule(RuleInstance::new(RuleSpec::ShallowFreezeOnly)).unwrap();

    // Should not be able to append to frozen collection
    let result = list.append(Value::number(2.0));
    assert!(result.is_err());
    if let Err(e) = result {
        let msg = e.to_string();
        assert!(msg.contains("frozen") || msg.contains("immutable"));
    }
}

// ============================================================================
// Freeze with Hash Tests (2 tests)
// ============================================================================

#[test]
fn test_hash_can_be_frozen() {
    let mut hash = Hash::new();
    // Note: Hash doesn't have set() method yet, skip setting value for now

    let mut value = Value::map(hash);
    assert!(!value.is_frozen());

    value.freeze();
    assert!(value.is_frozen());
}

#[test]
fn test_hash_no_frozen_behavior() {
    // no_frozen should work on hashes too
    let mut hash = Hash::new();
    hash.add_rule(RuleInstance::new(RuleSpec::NoFrozen)).unwrap();

    let mut frozen_value = Value::number(42.0);
    frozen_value.freeze();

    // Note: Hash doesn't have set() method yet, will be added later
    // For now, just test that rule was added
    assert!(hash.has_rule("no_frozen"));
}

// ============================================================================
// Integration Tests (2 tests)
// ============================================================================

#[test]
fn test_mix_frozen_and_unfrozen_without_rule() {
    // Without no_frozen rule, can mix frozen and unfrozen
    let mut list = List::new();

    let mut frozen = Value::number(1.0);
    frozen.freeze();

    let unfrozen = Value::number(2.0);

    list.append(frozen).unwrap();
    list.append(unfrozen).unwrap();

    assert_eq!(list.len(), 2);
}

#[test]
fn test_copy_elements_and_no_frozen_together() {
    // copy_elements + no_frozen should work together
    let mut list = List::new();
    list.add_rule(RuleInstance::new(RuleSpec::CopyElements)).unwrap();
    list.add_rule(RuleInstance::new(RuleSpec::NoFrozen)).unwrap();

    let mut frozen_value = Value::number(42.0);
    frozen_value.freeze();

    // copy_elements creates unfrozen copy, so no_frozen should accept it
    let result = list.append(frozen_value);
    assert!(result.is_ok());

    // Verify element is unfrozen
    let first = list.get(0).unwrap();
    assert!(!first.is_frozen());
}

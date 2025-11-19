use graphoid::execution::config::{
    Config, ConfigStack, ErrorMode, BoundsCheckingMode, TypeCoercionMode, NoneHandlingMode
};
use graphoid::values::Value;
use std::collections::HashMap;

#[test]
fn test_default_config() {
    let config = Config::default();
    assert_eq!(config.error_mode, ErrorMode::Strict);
    assert_eq!(config.bounds_checking, BoundsCheckingMode::Strict);
    assert_eq!(config.type_coercion, TypeCoercionMode::Strict);
    assert_eq!(config.none_handling, NoneHandlingMode::Propagate);
    assert_eq!(config.decimal_places, None);
    assert_eq!(config.strict_types, true);
    assert_eq!(config.edge_validation, true);
    assert_eq!(config.strict_edge_rules, true);
    assert_eq!(config.none_conversions, true);
    assert_eq!(config.skip_none, false);
}

#[test]
fn test_config_stack_new() {
    let stack = ConfigStack::new();
    assert_eq!(stack.depth(), 1);
    assert_eq!(stack.current().error_mode, ErrorMode::Strict);
}

#[test]
fn test_config_stack_push_pop() {
    let mut stack = ConfigStack::new();

    let mut new_config = Config::default();
    new_config.skip_none = true;

    stack.push(new_config);
    assert_eq!(stack.depth(), 2);
    assert_eq!(stack.current().skip_none, true);

    stack.pop();
    assert_eq!(stack.depth(), 1);
    assert_eq!(stack.current().skip_none, false);
}

#[test]
fn test_config_stack_cannot_pop_base() {
    let mut stack = ConfigStack::new();
    assert_eq!(stack.depth(), 1);

    let result = stack.pop();
    assert!(result.is_none());
    assert_eq!(stack.depth(), 1);
}

#[test]
fn test_push_with_changes_skip_none() {
    let mut stack = ConfigStack::new();

    let mut changes = HashMap::new();
    changes.insert("skip_none".to_string(), Value::boolean(true));

    stack.push_with_changes(changes).unwrap();
    assert_eq!(stack.current().skip_none, true);
    assert_eq!(stack.depth(), 2);
}

#[test]
fn test_push_with_changes_error_mode() {
    let mut stack = ConfigStack::new();

    let mut changes = HashMap::new();
    changes.insert("error_mode".to_string(), Value::symbol("lenient".to_string()));

    stack.push_with_changes(changes).unwrap();
    assert_eq!(stack.current().error_mode, ErrorMode::Lenient);
}

#[test]
fn test_push_with_changes_invalid_key() {
    let mut stack = ConfigStack::new();

    let mut changes = HashMap::new();
    changes.insert("invalid_key".to_string(), Value::boolean(true));

    let result = stack.push_with_changes(changes);
    assert!(result.is_err());
    assert_eq!(stack.depth(), 1); // Stack should remain unchanged
}

#[test]
fn test_parse_error_mode_valid() {
    // These are tested indirectly through push_with_changes
    let mut stack = ConfigStack::new();

    let mut changes = HashMap::new();
    changes.insert("error_mode".to_string(), Value::symbol("strict".to_string()));
    stack.push_with_changes(changes).unwrap();
    assert_eq!(stack.current().error_mode, ErrorMode::Strict);

    let mut stack = ConfigStack::new();
    let mut changes = HashMap::new();
    changes.insert("error_mode".to_string(), Value::symbol("lenient".to_string()));
    stack.push_with_changes(changes).unwrap();
    assert_eq!(stack.current().error_mode, ErrorMode::Lenient);

    let mut stack = ConfigStack::new();
    let mut changes = HashMap::new();
    changes.insert("error_mode".to_string(), Value::symbol("collect".to_string()));
    stack.push_with_changes(changes).unwrap();
    assert_eq!(stack.current().error_mode, ErrorMode::Collect);
}

#[test]
fn test_parse_error_mode_invalid() {
    let mut stack = ConfigStack::new();

    let mut changes = HashMap::new();
    changes.insert("error_mode".to_string(), Value::symbol("invalid".to_string()));
    assert!(stack.push_with_changes(changes).is_err());

    let mut changes = HashMap::new();
    changes.insert("error_mode".to_string(), Value::number(123.0));
    assert!(stack.push_with_changes(changes).is_err());
}

#[test]
fn test_parse_bounds_checking_mode() {
    let mut stack = ConfigStack::new();

    let mut changes = HashMap::new();
    changes.insert("bounds_checking".to_string(), Value::symbol("strict".to_string()));
    stack.push_with_changes(changes).unwrap();
    assert_eq!(stack.current().bounds_checking, BoundsCheckingMode::Strict);

    let mut stack = ConfigStack::new();
    let mut changes = HashMap::new();
    changes.insert("bounds_checking".to_string(), Value::symbol("lenient".to_string()));
    stack.push_with_changes(changes).unwrap();
    assert_eq!(stack.current().bounds_checking, BoundsCheckingMode::Lenient);
}

#[test]
fn test_parse_type_coercion_mode() {
    let mut stack = ConfigStack::new();

    let mut changes = HashMap::new();
    changes.insert("type_coercion".to_string(), Value::symbol("strict".to_string()));
    stack.push_with_changes(changes).unwrap();
    assert_eq!(stack.current().type_coercion, TypeCoercionMode::Strict);

    let mut stack = ConfigStack::new();
    let mut changes = HashMap::new();
    changes.insert("type_coercion".to_string(), Value::symbol("lenient".to_string()));
    stack.push_with_changes(changes).unwrap();
    assert_eq!(stack.current().type_coercion, TypeCoercionMode::Lenient);
}

#[test]
fn test_parse_none_handling_mode() {
    let mut stack = ConfigStack::new();

    let mut changes = HashMap::new();
    changes.insert("none_handling".to_string(), Value::symbol("propagate".to_string()));
    stack.push_with_changes(changes).unwrap();
    assert_eq!(stack.current().none_handling, NoneHandlingMode::Propagate);

    let mut stack = ConfigStack::new();
    let mut changes = HashMap::new();
    changes.insert("none_handling".to_string(), Value::symbol("skip".to_string()));
    stack.push_with_changes(changes).unwrap();
    assert_eq!(stack.current().none_handling, NoneHandlingMode::Skip);

    let mut stack = ConfigStack::new();
    let mut changes = HashMap::new();
    changes.insert("none_handling".to_string(), Value::symbol("error".to_string()));
    stack.push_with_changes(changes).unwrap();
    assert_eq!(stack.current().none_handling, NoneHandlingMode::Error);
}

#[test]
fn test_nested_config_changes() {
    let mut stack = ConfigStack::new();

    // Push first level
    let mut changes1 = HashMap::new();
    changes1.insert("skip_none".to_string(), Value::boolean(true));
    stack.push_with_changes(changes1).unwrap();
    assert_eq!(stack.depth(), 2);
    assert_eq!(stack.current().skip_none, true);

    // Push second level
    let mut changes2 = HashMap::new();
    changes2.insert("error_mode".to_string(), Value::symbol("lenient".to_string()));
    stack.push_with_changes(changes2).unwrap();
    assert_eq!(stack.depth(), 3);
    assert_eq!(stack.current().skip_none, true);  // Inherited from previous level
    assert_eq!(stack.current().error_mode, ErrorMode::Lenient);

    // Pop back to second level
    stack.pop();
    assert_eq!(stack.depth(), 2);
    assert_eq!(stack.current().skip_none, true);
    assert_eq!(stack.current().error_mode, ErrorMode::Strict);  // Back to default

    // Pop back to base
    stack.pop();
    assert_eq!(stack.depth(), 1);
    assert_eq!(stack.current().skip_none, false);
}

#[test]
fn test_config_cloning() {
    let config1 = Config::default();
    let mut config2 = config1.clone();

    config2.skip_none = true;

    assert_eq!(config1.skip_none, false);
    assert_eq!(config2.skip_none, true);
}

#[test]
fn test_multiple_config_keys() {
    let mut stack = ConfigStack::new();

    let mut changes = HashMap::new();
    changes.insert("skip_none".to_string(), Value::boolean(true));
    changes.insert("error_mode".to_string(), Value::symbol("lenient".to_string()));
    changes.insert("strict_types".to_string(), Value::boolean(false));

    stack.push_with_changes(changes).unwrap();
    assert_eq!(stack.current().skip_none, true);
    assert_eq!(stack.current().error_mode, ErrorMode::Lenient);
    assert_eq!(stack.current().strict_types, false);
}

// ===== Phase 1A: :integer Directive Tests =====

#[test]
fn test_default_config_integer_mode_false() {
    let config = Config::default();
    assert_eq!(config.integer_mode, false);
}

#[test]
fn test_integer_directive_sets_integer_mode() {
    let mut stack = ConfigStack::new();

    // Simulate precision { :integer } { ... }
    let mut changes = HashMap::new();
    changes.insert("integer".to_string(), Value::symbol("integer".to_string()));

    stack.push_with_changes(changes).unwrap();
    assert_eq!(stack.current().integer_mode, true);
    assert_eq!(stack.depth(), 2);
}

#[test]
fn test_integer_mode_scoped_correctly() {
    let mut stack = ConfigStack::new();

    // Base level - integer_mode is false
    assert_eq!(stack.current().integer_mode, false);

    // Push :integer directive
    let mut changes = HashMap::new();
    changes.insert("integer".to_string(), Value::symbol("integer".to_string()));
    stack.push_with_changes(changes).unwrap();
    assert_eq!(stack.current().integer_mode, true);
    assert_eq!(stack.depth(), 2);

    // Pop back to base level
    stack.pop();
    assert_eq!(stack.current().integer_mode, false);
    assert_eq!(stack.depth(), 1);
}

#[test]
fn test_integer_directive_combines_with_other_directives() {
    let mut stack = ConfigStack::new();

    // Combine :integer with :unsigned (both directives)
    let mut changes = HashMap::new();
    changes.insert("integer".to_string(), Value::symbol("integer".to_string()));
    changes.insert("unsigned".to_string(), Value::symbol("unsigned".to_string()));

    stack.push_with_changes(changes).unwrap();
    assert_eq!(stack.current().integer_mode, true);
    assert_eq!(stack.current().unsigned_mode, true);
}

#[test]
fn test_integer_directive_inherits_other_config() {
    let mut stack = ConfigStack::new();

    // First level: set skip_none
    let mut changes1 = HashMap::new();
    changes1.insert("skip_none".to_string(), Value::boolean(true));
    stack.push_with_changes(changes1).unwrap();
    assert_eq!(stack.current().skip_none, true);
    assert_eq!(stack.current().integer_mode, false);

    // Second level: add :integer directive
    let mut changes2 = HashMap::new();
    changes2.insert("integer".to_string(), Value::symbol("integer".to_string()));
    stack.push_with_changes(changes2).unwrap();
    assert_eq!(stack.current().skip_none, true);  // Inherited
    assert_eq!(stack.current().integer_mode, true);  // New setting

    // Pop back to first level
    stack.pop();
    assert_eq!(stack.current().skip_none, true);
    assert_eq!(stack.current().integer_mode, false);  // Back to false
}

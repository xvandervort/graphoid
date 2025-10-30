//! Tests for custom function and conditional behaviors (Sub-Phase 7.4)
//!
//! These tests verify:
//! - CustomFunction behaviors with user-defined functions
//! - Conditional behaviors with predicate-based transformations
//! - Retroactive and proactive application
//! - Integration with List and Hash collections
//! - Error handling

use graphoid::ast::{Stmt, Expr, LiteralValue, BinaryOp, UnaryOp, Parameter};
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

// Helper to create a simple doubling function: func(x) { return x * 2 }
fn create_double_fn() -> Function {
    Function {
        name: Some("double".to_string()),
        params: vec!["x".to_string()],
        parameters: vec![Parameter { name: "x".to_string(), default_value: None, is_variadic: false }],
        body: vec![
            Stmt::Return {
                value: Some(Expr::Binary {
                    left: Box::new(Expr::Variable { name: "x".to_string(), position: pos() }),
                    op: BinaryOp::Multiply,
                    right: Box::new(Expr::Literal { value: LiteralValue::Number(2.0), position: pos() }),
                    position: pos(),
                }),
                position: pos(),
            }
        ],
        env: Rc::new(RefCell::new(Environment::new())),
        node_id: None,
    }
}

// Helper to create negation function: func(x) { return -x }
fn create_negate_fn() -> Function {
    Function {
        name: Some("negate".to_string()),
        params: vec!["x".to_string()],
        parameters: vec![Parameter { name: "x".to_string(), default_value: None, is_variadic: false }],
        body: vec![
            Stmt::Return {
                value: Some(Expr::Unary {
                    op: UnaryOp::Negate,
                    operand: Box::new(Expr::Variable { name: "x".to_string(), position: pos() }),
                    position: pos(),
                }),
                position: pos(),
            }
        ],
        env: Rc::new(RefCell::new(Environment::new())),
        node_id: None,
    }
}

// Helper: func(x) { return x < 0 }
fn create_is_negative_fn() -> Function {
    Function {
        name: Some("is_negative".to_string()),
        params: vec!["x".to_string()],
        parameters: vec![Parameter { name: "x".to_string(), default_value: None, is_variadic: false }],
        body: vec![
            Stmt::Return {
                value: Some(Expr::Binary {
                    left: Box::new(Expr::Variable { name: "x".to_string(), position: pos() }),
                    op: BinaryOp::Less,
                    right: Box::new(Expr::Literal { value: LiteralValue::Number(0.0), position: pos() }),
                    position: pos(),
                }),
                position: pos(),
            }
        ],
        env: Rc::new(RefCell::new(Environment::new())),
        node_id: None,
    }
}

// ============================================================================
// Test 1-6: Custom Function Behaviors
// ============================================================================

#[test]
fn test_custom_function_basic() {
    let rule = RuleInstance::new(RuleSpec::CustomFunction {
            function: Value::Function(create_double_fn()),
        });

    let mut executor = Executor::new();

    // Test basic transformation
    let result = executor.apply_transformation_rules_with_context(Value::Number(5.0), &[rule]).unwrap();
    assert_eq!(result, Value::Number(10.0)); // 5 * 2
}

#[test]
fn test_custom_function_with_closure() {
    // Create a function that uses a captured variable
    let mut env = Environment::new();
    env.define("multiplier".to_string(), Value::Number(3.0));

    // func(x) { return x * multiplier }
    let multiply_fn = Function {
        name: Some("multiply".to_string()),
        params: vec!["x".to_string()],
        parameters: vec![Parameter { name: "x".to_string(), default_value: None, is_variadic: false }],
        body: vec![
            Stmt::Return {
                value: Some(Expr::Binary {
                    left: Box::new(Expr::Variable { name: "x".to_string(), position: pos() }),
                    op: BinaryOp::Multiply,
                    right: Box::new(Expr::Variable { name: "multiplier".to_string(), position: pos() }),
                    position: pos(),
                }),
                position: pos(),
            }
        ],
        env: Rc::new(RefCell::new(env)),
        node_id: None,
    };

    let rule = RuleInstance::new(RuleSpec::CustomFunction {
            function: Value::Function(multiply_fn),
        });

    let mut executor = Executor::new();
    let result = executor.apply_transformation_rules_with_context(Value::Number(5.0), &[rule]).unwrap();
    assert_eq!(result, Value::Number(15.0)); // 5 * 3
}

#[test]
fn test_custom_function_type_specific() {
    // func(x) { return x + 10 }
    let add_ten_fn = Function {
        name: Some("add_ten".to_string()),
        params: vec!["x".to_string()],
        parameters: vec![Parameter { name: "x".to_string(), default_value: None, is_variadic: false }],
        body: vec![
            Stmt::Return {
                value: Some(Expr::Binary {
                    left: Box::new(Expr::Variable { name: "x".to_string(), position: pos() }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Literal { value: LiteralValue::Number(10.0), position: pos() }),
                    position: pos(),
                }),
                position: pos(),
            }
        ],
        env: Rc::new(RefCell::new(Environment::new())),
        node_id: None,
    };

    let rule = RuleInstance::new(RuleSpec::CustomFunction {
            function: Value::Function(add_ten_fn),
        });

    let mut executor = Executor::new();

    // Test with number
    let result = executor.apply_transformation_rules_with_context(Value::Number(5.0), &[rule.clone()]).unwrap();
    assert_eq!(result, Value::Number(15.0));

    // Test with string - type coercion converts number to string for concatenation
    let result = executor.apply_transformation_rules_with_context(Value::String("hello".to_string()), &[rule]);
    assert_eq!(result.unwrap(), Value::String("hello10".to_string()));
}

#[test]
fn test_custom_function_error_handling() {
    // func(x) { return 10 / x }
    let divide_fn = Function {
        name: Some("divide".to_string()),
        params: vec!["x".to_string()],
        parameters: vec![Parameter { name: "x".to_string(), default_value: None, is_variadic: false }],
        body: vec![
            Stmt::Return {
                value: Some(Expr::Binary {
                    left: Box::new(Expr::Literal { value: LiteralValue::Number(10.0), position: pos() }),
                    op: BinaryOp::Divide,
                    right: Box::new(Expr::Variable { name: "x".to_string(), position: pos() }),
                    position: pos(),
                }),
                position: pos(),
            }
        ],
        env: Rc::new(RefCell::new(Environment::new())),
        node_id: None,
    };

    let rule = RuleInstance::new(RuleSpec::CustomFunction {
            function: Value::Function(divide_fn),
        });

    let mut executor = Executor::new();

    // 10 / 2 = 5
    let result = executor.apply_transformation_rules_with_context(Value::Number(2.0), &[rule.clone()]).unwrap();
    assert_eq!(result, Value::Number(5.0));

    // Division by zero should error
    let result = executor.apply_transformation_rules_with_context(Value::Number(0.0), &[rule]);
    assert!(result.is_err());
}

#[test]
fn test_custom_function_proactive() {
    // Test that new values added to a list are transformed
    let mut list = List::new();
    let rule = RuleInstance::new(RuleSpec::CustomFunction {
            function: Value::Function(create_double_fn()),
        });
    list.graph.rules.push(rule);

    let mut executor = Executor::new();

    // Add value through executor
    let val = executor.apply_transformation_rules_with_context(Value::Number(7.0), &list.graph.rules).unwrap();
    list.append_raw(val).unwrap();

    // Verify it was doubled
    assert_eq!(list.get(0), Some(&Value::Number(14.0)));
}

#[test]
fn test_custom_function_retroactive() {
    // Test retroactive application
    let mut list = List::new();
    list.append(Value::Number(5.0)).unwrap();
    list.append(Value::Number(10.0)).unwrap();

    let rule = RuleInstance::new(RuleSpec::CustomFunction {
            function: Value::Function(create_double_fn()),
        });

    let mut executor = Executor::new();

    // Apply retroactively to existing values
    let existing_values: Vec<Value> = list.to_vec();
    let mut new_list = List::new();
    for val in existing_values {
        let transformed = executor.apply_transformation_rules_with_context(val, &[rule.clone()]).unwrap();
        new_list.append_raw(transformed).unwrap();
    }
    new_list.graph.rules.push(rule);

    // Verify retroactive transformation
    assert_eq!(new_list.get(0), Some(&Value::Number(10.0))); // 5 * 2
    assert_eq!(new_list.get(1), Some(&Value::Number(20.0))); // 10 * 2
}

// ============================================================================
// Test 7-12: Conditional Behaviors
// ============================================================================

#[test]
fn test_conditional_basic() {
    let rule = RuleInstance::new(RuleSpec::Conditional {
            condition: Value::Function(create_is_negative_fn()),
            transform: Value::Function(create_negate_fn()),
            fallback: None,
        });

    let mut executor = Executor::new();

    // Negative: should be transformed to positive
    let result = executor.apply_transformation_rules_with_context(Value::Number(-5.0), &[rule.clone()]).unwrap();
    assert_eq!(result, Value::Number(5.0));

    // Positive: should stay unchanged (no fallback)
    let result = executor.apply_transformation_rules_with_context(Value::Number(3.0), &[rule]).unwrap();
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn test_conditional_with_fallback() {
    let rule = RuleInstance::new(RuleSpec::Conditional {
            condition: Value::Function(create_is_negative_fn()),
            transform: Value::Function(create_negate_fn()),
            fallback: Some(Value::Function(create_double_fn())),
        });

    let mut executor = Executor::new();

    // Negative: condition true, use transform
    let result = executor.apply_transformation_rules_with_context(Value::Number(-5.0), &[rule.clone()]).unwrap();
    assert_eq!(result, Value::Number(5.0));

    // Positive: condition false, use fallback
    let result = executor.apply_transformation_rules_with_context(Value::Number(3.0), &[rule]).unwrap();
    assert_eq!(result, Value::Number(6.0)); // 3 * 2
}

#[test]
fn test_conditional_without_fallback() {
    // func(x) { return x > 0 }
    let is_positive = Function {
        name: Some("is_positive".to_string()),
        params: vec!["x".to_string()],
        parameters: vec![Parameter { name: "x".to_string(), default_value: None, is_variadic: false }],
        body: vec![
            Stmt::Return {
                value: Some(Expr::Binary {
                    left: Box::new(Expr::Variable { name: "x".to_string(), position: pos() }),
                    op: BinaryOp::Greater,
                    right: Box::new(Expr::Literal { value: LiteralValue::Number(0.0), position: pos() }),
                    position: pos(),
                }),
                position: pos(),
            }
        ],
        env: Rc::new(RefCell::new(Environment::new())),
        node_id: None,
    };

    // func(x) { return x * x }
    let square = Function {
        name: Some("square".to_string()),
        params: vec!["x".to_string()],
        parameters: vec![Parameter { name: "x".to_string(), default_value: None, is_variadic: false }],
        body: vec![
            Stmt::Return {
                value: Some(Expr::Binary {
                    left: Box::new(Expr::Variable { name: "x".to_string(), position: pos() }),
                    op: BinaryOp::Multiply,
                    right: Box::new(Expr::Variable { name: "x".to_string(), position: pos() }),
                    position: pos(),
                }),
                position: pos(),
            }
        ],
        env: Rc::new(RefCell::new(Environment::new())),
        node_id: None,
    };

    let rule = RuleInstance::new(RuleSpec::Conditional {
            condition: Value::Function(is_positive),
            transform: Value::Function(square),
            fallback: None,
        });

    let mut executor = Executor::new();

    // Positive: square it
    let result = executor.apply_transformation_rules_with_context(Value::Number(4.0), &[rule.clone()]).unwrap();
    assert_eq!(result, Value::Number(16.0));

    // Zero or negative: keep unchanged
    let result = executor.apply_transformation_rules_with_context(Value::Number(0.0), &[rule.clone()]).unwrap();
    assert_eq!(result, Value::Number(0.0));

    let result = executor.apply_transformation_rules_with_context(Value::Number(-3.0), &[rule]).unwrap();
    assert_eq!(result, Value::Number(-3.0));
}

#[test]
fn test_conditional_proactive() {
    // Test proactive application
    let mut list = List::new();
    let rule = RuleInstance::new(RuleSpec::Conditional {
            condition: Value::Function(create_is_negative_fn()),
            transform: Value::Function(create_negate_fn()),
            fallback: None,
        });
    list.graph.rules.push(rule);

    let mut executor = Executor::new();

    // Add negative value
    let val = executor.apply_transformation_rules_with_context(Value::Number(-8.0), &list.graph.rules).unwrap();
    list.append_raw(val).unwrap();

    // Verify it was made positive
    assert_eq!(list.get(0), Some(&Value::Number(8.0)));
}

#[test]
fn test_conditional_retroactive() {
    // Test retroactive application
    let mut list = List::new();
    list.append(Value::Number(-5.0)).unwrap();
    list.append(Value::Number(3.0)).unwrap();

    let rule = RuleInstance::new(RuleSpec::Conditional {
            condition: Value::Function(create_is_negative_fn()),
            transform: Value::Function(create_negate_fn()),
            fallback: None,
        });

    let mut executor = Executor::new();

    // Apply retroactively
    let existing_values: Vec<Value> = list.to_vec();
    let mut new_list = List::new();
    for val in existing_values {
        let transformed = executor.apply_transformation_rules_with_context(val, &[rule.clone()]).unwrap();
        new_list.append_raw(transformed).unwrap();
    }
    new_list.graph.rules.push(rule);

    // Verify
    assert_eq!(new_list.get(0), Some(&Value::Number(5.0)));  // -5 -> 5
    assert_eq!(new_list.get(1), Some(&Value::Number(3.0)));  // 3 -> 3
}

#[test]
fn test_conditional_chain() {
    // func(x) { return x > 10 }
    let is_large = Function {
        name: Some("is_large".to_string()),
        params: vec!["x".to_string()],
        parameters: vec![Parameter { name: "x".to_string(), default_value: None, is_variadic: false }],
        body: vec![
            Stmt::Return {
                value: Some(Expr::Binary {
                    left: Box::new(Expr::Variable { name: "x".to_string(), position: pos() }),
                    op: BinaryOp::Greater,
                    right: Box::new(Expr::Literal { value: LiteralValue::Number(10.0), position: pos() }),
                    position: pos(),
                }),
                position: pos(),
            }
        ],
        env: Rc::new(RefCell::new(Environment::new())),
        node_id: None,
    };

    // func(x) { return 10 }
    let clamp_to_10 = Function {
        name: Some("clamp_to_10".to_string()),
        params: vec!["x".to_string()],
        parameters: vec![Parameter { name: "x".to_string(), default_value: None, is_variadic: false }],
        body: vec![
            Stmt::Return {
                value: Some(Expr::Literal { value: LiteralValue::Number(10.0), position: pos() }),
                position: pos(),
            }
        ],
        env: Rc::new(RefCell::new(Environment::new())),
        node_id: None,
    };

    let behavior1 = RuleInstance::new(RuleSpec::Conditional {
        condition: Value::Function(create_is_negative_fn()),
        transform: Value::Function(create_negate_fn()),
        fallback: None,
    });

    let behavior2 = RuleInstance::new(RuleSpec::Conditional {
        condition: Value::Function(is_large),
        transform: Value::Function(clamp_to_10),
        fallback: None,
    });

    let mut executor = Executor::new();
    let behaviors = vec![behavior1, behavior2];

    // Test: -15 -> 15 (negated) -> 10 (clamped)
    let result = executor.apply_transformation_rules_with_context(Value::Number(-15.0), &behaviors).unwrap();
    assert_eq!(result, Value::Number(10.0));

    // Test: -5 -> 5 (negated) -> 5 (not clamped)
    let result = executor.apply_transformation_rules_with_context(Value::Number(-5.0), &behaviors).unwrap();
    assert_eq!(result, Value::Number(5.0));

    // Test: 20 -> 20 (not negated) -> 10 (clamped)
    let result = executor.apply_transformation_rules_with_context(Value::Number(20.0), &behaviors).unwrap();
    assert_eq!(result, Value::Number(10.0));
}

// ============================================================================
// Test 13-15: Integration Tests
// ============================================================================

#[test]
fn test_list_with_custom_function() {
    let mut list = List::new();
    let rule = RuleInstance::new(RuleSpec::CustomFunction {
            function: Value::Function(create_double_fn()),
        });
    list.graph.rules.push(rule);

    let mut executor = Executor::new();

    // Add multiple values
    for val in vec![5.0, 10.0, 15.0] {
        let transformed = executor.apply_transformation_rules_with_context(Value::Number(val), &list.graph.rules).unwrap();
        list.append_raw(transformed).unwrap();
    }

    // Verify all were doubled
    assert_eq!(list.get(0), Some(&Value::Number(10.0)));  // 5 * 2
    assert_eq!(list.get(1), Some(&Value::Number(20.0)));  // 10 * 2
    assert_eq!(list.get(2), Some(&Value::Number(30.0)));  // 15 * 2
}

#[test]
fn test_list_with_conditional() {
    let mut list = List::new();
    let rule = RuleInstance::new(RuleSpec::Conditional {
            condition: Value::Function(create_is_negative_fn()),
            transform: Value::Function(create_negate_fn()),
            fallback: None,
        });
    list.graph.rules.push(rule);

    let mut executor = Executor::new();

    // Add mixed positive and negative values
    for val in vec![-5.0, 3.0, -2.0, 7.0] {
        let transformed = executor.apply_transformation_rules_with_context(Value::Number(val), &list.graph.rules).unwrap();
        list.append_raw(transformed).unwrap();
    }

    // Verify all are positive
    assert_eq!(list.get(0), Some(&Value::Number(5.0)));
    assert_eq!(list.get(1), Some(&Value::Number(3.0)));
    assert_eq!(list.get(2), Some(&Value::Number(2.0)));
    assert_eq!(list.get(3), Some(&Value::Number(7.0)));
}

#[test]
fn test_mixed_behaviors() {
    // Combine standard behavior (NoneToZero) with custom function
    let behavior1 = RuleInstance::new(RuleSpec::NoneToZero);

    let behavior2 = RuleInstance::new(RuleSpec::CustomFunction {
        function: Value::Function(create_double_fn()),
    });

    let behaviors = vec![behavior1, behavior2];
    let mut executor = Executor::new();

    // Test: None -> 0 (NoneToZero) -> 0 (double)
    let result = executor.apply_transformation_rules_with_context(Value::None, &behaviors).unwrap();
    assert_eq!(result, Value::Number(0.0));

    // Test: 5 -> 5 (NoneToZero doesn't apply) -> 10 (double)
    let result = executor.apply_transformation_rules_with_context(Value::Number(5.0), &behaviors).unwrap();
    assert_eq!(result, Value::Number(10.0));
}

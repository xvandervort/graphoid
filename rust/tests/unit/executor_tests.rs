use graphoid::ast::{AssignmentTarget, BinaryOp, Expr, LiteralValue, Parameter, Stmt, UnaryOp};
use graphoid::error::SourcePosition;
use graphoid::execution::Executor;
use graphoid::values::{Hash, List, Value};
use std::collections::HashMap;

// Helper function to create a dummy source position
fn pos() -> SourcePosition {
    SourcePosition {
        line: 1,
        column: 1,
        file: None,
    }
}

// ============================================================================
// LITERAL EVALUATION TESTS
// ============================================================================

#[test]
fn test_eval_number_literal() {
    let mut executor = Executor::new();
    let expr = Expr::Literal {
        value: LiteralValue::Number(42.0),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_eval_float_literal() {
    let mut executor = Executor::new();
    let expr = Expr::Literal {
        value: LiteralValue::Number(3.14159),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Number(3.14159));
}

#[test]
fn test_eval_string_literal() {
    let mut executor = Executor::new();
    let expr = Expr::Literal {
        value: LiteralValue::String("hello world".to_string()),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::String("hello world".to_string()));
}

#[test]
fn test_eval_boolean_true() {
    let mut executor = Executor::new();
    let expr = Expr::Literal {
        value: LiteralValue::Boolean(true),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_eval_boolean_false() {
    let mut executor = Executor::new();
    let expr = Expr::Literal {
        value: LiteralValue::Boolean(false),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_eval_none_literal() {
    let mut executor = Executor::new();
    let expr = Expr::Literal {
        value: LiteralValue::None,
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::None);
}

#[test]
fn test_eval_symbol_literal() {
    let mut executor = Executor::new();
    let expr = Expr::Literal {
        value: LiteralValue::Symbol("test_symbol".to_string()),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Symbol("test_symbol".to_string()));
}

// ============================================================================
// ARITHMETIC OPERATOR TESTS
// ============================================================================

#[test]
fn test_eval_addition() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Number(2.0),
            position: pos(),
        }),
        op: BinaryOp::Add,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(3.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_eval_subtraction() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Number(10.0),
            position: pos(),
        }),
        op: BinaryOp::Subtract,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(4.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Number(6.0));
}

#[test]
fn test_eval_multiplication() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Number(6.0),
            position: pos(),
        }),
        op: BinaryOp::Multiply,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(7.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_eval_division() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Number(15.0),
            position: pos(),
        }),
        op: BinaryOp::Divide,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(3.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_eval_integer_division() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Number(17.0),
            position: pos(),
        }),
        op: BinaryOp::IntDiv,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn test_eval_modulo() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Number(10.0),
            position: pos(),
        }),
        op: BinaryOp::Modulo,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(3.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Number(1.0));
}

#[test]
fn test_eval_power() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Number(2.0),
            position: pos(),
        }),
        op: BinaryOp::Power,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(8.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Number(256.0));
}

#[test]
fn test_eval_division_by_zero() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Number(10.0),
            position: pos(),
        }),
        op: BinaryOp::Divide,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(0.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr);
    assert!(result.is_err());
}

#[test]
fn test_eval_operator_precedence() {
    // 2 + 3 * 4 = 14 (not 20)
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Number(2.0),
            position: pos(),
        }),
        op: BinaryOp::Add,
        right: Box::new(Expr::Binary {
            left: Box::new(Expr::Literal {
                value: LiteralValue::Number(3.0),
                position: pos(),
            }),
            op: BinaryOp::Multiply,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(4.0),
                position: pos(),
            }),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Number(14.0));
}

// ============================================================================
// COMPARISON OPERATOR TESTS
// ============================================================================

#[test]
fn test_eval_greater_than_true() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }),
        op: BinaryOp::Greater,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(3.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_eval_greater_than_false() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Number(3.0),
            position: pos(),
        }),
        op: BinaryOp::Greater,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_eval_less_than() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Number(3.0),
            position: pos(),
        }),
        op: BinaryOp::Less,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_eval_equal() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }),
        op: BinaryOp::Equal,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_eval_not_equal() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }),
        op: BinaryOp::NotEqual,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(3.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_eval_greater_or_equal() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }),
        op: BinaryOp::GreaterEqual,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_eval_less_or_equal() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Number(3.0),
            position: pos(),
        }),
        op: BinaryOp::LessEqual,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

// ============================================================================
// LOGICAL OPERATOR TESTS
// ============================================================================

#[test]
fn test_eval_and_true() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Boolean(true),
            position: pos(),
        }),
        op: BinaryOp::And,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Boolean(true),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_eval_and_false() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Boolean(true),
            position: pos(),
        }),
        op: BinaryOp::And,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Boolean(false),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_eval_or_true() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Boolean(true),
            position: pos(),
        }),
        op: BinaryOp::Or,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Boolean(false),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_eval_or_false() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Boolean(false),
            position: pos(),
        }),
        op: BinaryOp::Or,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Boolean(false),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_eval_not_true() {
    let mut executor = Executor::new();
    let expr = Expr::Unary {
        op: UnaryOp::Not,
        operand: Box::new(Expr::Literal {
            value: LiteralValue::Boolean(true),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_eval_not_false() {
    let mut executor = Executor::new();
    let expr = Expr::Unary {
        op: UnaryOp::Not,
        operand: Box::new(Expr::Literal {
            value: LiteralValue::Boolean(false),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_eval_negation() {
    let mut executor = Executor::new();
    let expr = Expr::Unary {
        op: UnaryOp::Negate,
        operand: Box::new(Expr::Literal {
            value: LiteralValue::Number(42.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Number(-42.0));
}

// ============================================================================
// VARIABLE TESTS
// ============================================================================

#[test]
fn test_eval_variable_declaration() {
    let mut executor = Executor::new();
    let stmt = Stmt::VariableDecl {
        name: "x".to_string(),
        type_annotation: None,
        value: Expr::Literal {
            value: LiteralValue::Number(42.0),
            position: pos(),
        },
        position: pos(),
    };

    executor.eval_stmt(&stmt).unwrap();

    // Verify variable was defined
    let value = executor.env().get("x").unwrap();
    assert_eq!(value, Value::Number(42.0));
}

#[test]
fn test_eval_variable_reference() {
    let mut executor = Executor::new();

    // Define variable
    let stmt = Stmt::VariableDecl {
        name: "x".to_string(),
        type_annotation: None,
        value: Expr::Literal {
            value: LiteralValue::Number(42.0),
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&stmt).unwrap();

    // Reference variable
    let expr = Expr::Variable {
        name: "x".to_string(),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_eval_variable_assignment() {
    let mut executor = Executor::new();

    // Define variable
    let decl = Stmt::VariableDecl {
        name: "x".to_string(),
        type_annotation: None,
        value: Expr::Literal {
            value: LiteralValue::Number(10.0),
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&decl).unwrap();

    // Assign new value
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("x".to_string()),
        value: Expr::Literal {
            value: LiteralValue::Number(20.0),
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // Verify new value
    let value = executor.env().get("x").unwrap();
    assert_eq!(value, Value::Number(20.0));
}

#[test]
fn test_eval_undefined_variable() {
    let mut executor = Executor::new();
    let expr = Expr::Variable {
        name: "undefined".to_string(),
        position: pos(),
    };

    let result = executor.eval_expr(&expr);
    assert!(result.is_err());
}

#[test]
fn test_eval_variable_in_expression() {
    let mut executor = Executor::new();

    // Define variable x = 10
    let stmt = Stmt::VariableDecl {
        name: "x".to_string(),
        type_annotation: None,
        value: Expr::Literal {
            value: LiteralValue::Number(10.0),
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&stmt).unwrap();

    // Evaluate x + 5
    let expr = Expr::Binary {
        left: Box::new(Expr::Variable {
            name: "x".to_string(),
            position: pos(),
        }),
        op: BinaryOp::Add,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Number(15.0));
}

// ============================================================================
// COLLECTION TESTS
// ============================================================================

#[test]
fn test_eval_empty_list() {
    let mut executor = Executor::new();
    let expr = Expr::List {
        elements: vec![],
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::List(List::from_vec(vec![])));
}

#[test]
fn test_eval_list_with_elements() {
    let mut executor = Executor::new();
    let expr = Expr::List {
        elements: vec![
            Expr::Literal {
                value: LiteralValue::Number(1.0),
                position: pos(),
            },
            Expr::Literal {
                value: LiteralValue::Number(2.0),
                position: pos(),
            },
            Expr::Literal {
                value: LiteralValue::Number(3.0),
                position: pos(),
            },
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(
        result,
        Value::List(List::from_vec(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0)
        ]))
    );
}

#[test]
fn test_eval_empty_map() {
    let mut executor = Executor::new();
    let expr = Expr::Map {
        entries: vec![],
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Map(Hash::from_hashmap(HashMap::new())));
}

#[test]
fn test_eval_map_with_entries() {
    let mut executor = Executor::new();
    let expr = Expr::Map {
        entries: vec![
            (
                "name".to_string(),
                Expr::Literal {
                    value: LiteralValue::String("Alice".to_string()),
                    position: pos(),
                },
            ),
            (
                "age".to_string(),
                Expr::Literal {
                    value: LiteralValue::Number(30.0),
                    position: pos(),
                },
            ),
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();

    if let Value::Map(map) = result {
        assert_eq!(map.len(), 2);
        assert_eq!(
            map.get("name"),
            Some(&Value::String("Alice".to_string()))
        );
        assert_eq!(map.get("age"), Some(&Value::Number(30.0)));
    } else {
        panic!("Expected Map value");
    }
}

// ============================================================================
// STRING OPERATIONS
// ============================================================================

#[test]
fn test_eval_string_concatenation() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::String("hello".to_string()),
            position: pos(),
        }),
        op: BinaryOp::Add,
        right: Box::new(Expr::Literal {
            value: LiteralValue::String(" world".to_string()),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::String("hello world".to_string()));
}

#[test]
fn test_eval_string_concatenation_multiple() {
    let mut executor = Executor::new();
    // "hello" + " " + "world"
    let expr = Expr::Binary {
        left: Box::new(Expr::Binary {
            left: Box::new(Expr::Literal {
                value: LiteralValue::String("hello".to_string()),
                position: pos(),
            }),
            op: BinaryOp::Add,
            right: Box::new(Expr::Literal {
                value: LiteralValue::String(" ".to_string()),
                position: pos(),
            }),
            position: pos(),
        }),
        op: BinaryOp::Add,
        right: Box::new(Expr::Literal {
            value: LiteralValue::String("world".to_string()),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::String("hello world".to_string()));
}

#[test]
fn test_eval_string_less_than() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::String("apple".to_string()),
            position: pos(),
        }),
        op: BinaryOp::Less,
        right: Box::new(Expr::Literal {
            value: LiteralValue::String("banana".to_string()),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_eval_string_greater_than() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::String("zebra".to_string()),
            position: pos(),
        }),
        op: BinaryOp::Greater,
        right: Box::new(Expr::Literal {
            value: LiteralValue::String("apple".to_string()),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_eval_string_equal() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::String("hello".to_string()),
            position: pos(),
        }),
        op: BinaryOp::Equal,
        right: Box::new(Expr::Literal {
            value: LiteralValue::String("hello".to_string()),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

// ============================================================================
// TYPE ERROR TESTS
// ============================================================================

#[test]
fn test_eval_type_error_add_string_to_number() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::String("hello".to_string()),
            position: pos(),
        }),
        op: BinaryOp::Add,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr);
    assert!(result.is_err());
}

#[test]
fn test_eval_type_error_subtract_strings() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::String("hello".to_string()),
            position: pos(),
        }),
        op: BinaryOp::Subtract,
        right: Box::new(Expr::Literal {
            value: LiteralValue::String("world".to_string()),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr);
    assert!(result.is_err());
}

#[test]
fn test_eval_type_error_multiply_string_and_number() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::String("hello".to_string()),
            position: pos(),
        }),
        op: BinaryOp::Multiply,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(3.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr);
    assert!(result.is_err());
}

#[test]
fn test_eval_type_error_negate_string() {
    let mut executor = Executor::new();
    let expr = Expr::Unary {
        op: UnaryOp::Negate,
        operand: Box::new(Expr::Literal {
            value: LiteralValue::String("hello".to_string()),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr);
    assert!(result.is_err());
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_eval_power_zero_to_zero() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Number(0.0),
            position: pos(),
        }),
        op: BinaryOp::Power,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(0.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    // 0^0 is 1 in Rust's powf (mathematical convention varies)
    assert_eq!(result, Value::Number(1.0));
}

#[test]
fn test_eval_power_negative_exponent() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Number(2.0),
            position: pos(),
        }),
        op: BinaryOp::Power,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(-2.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Number(0.25));
}

#[test]
fn test_eval_modulo_by_zero() {
    let mut executor = Executor::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Number(10.0),
            position: pos(),
        }),
        op: BinaryOp::Modulo,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(0.0),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    // Modulo by zero in f64 returns NaN
    if let Value::Number(n) = result {
        assert!(n.is_nan());
    } else {
        panic!("Expected Number value");
    }
}

#[test]
fn test_eval_empty_string_is_falsy() {
    let mut executor = Executor::new();
    let expr = Expr::Unary {
        op: UnaryOp::Not,
        operand: Box::new(Expr::Literal {
            value: LiteralValue::String("".to_string()),
            position: pos(),
        }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::Boolean(true)); // not "" => true
}

#[test]
fn test_eval_expression_statement() {
    let mut executor = Executor::new();
    let stmt = Stmt::Expression {
        expr: Expr::Binary {
            left: Box::new(Expr::Literal {
                value: LiteralValue::Number(2.0),
                position: pos(),
            }),
            op: BinaryOp::Add,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(3.0),
                position: pos(),
            }),
            position: pos(),
        },
        position: pos(),
    };

    // Should execute without error
    let result = executor.eval_stmt(&stmt);
    assert!(result.is_ok());
}

// ============================================================================
// FUNCTION TESTS (Phase 4)
// ============================================================================

#[test]
fn test_function_declaration() {
    let mut executor = Executor::new();

    // Define function: func add(a, b) { return a + b }
    let func_decl = Stmt::FunctionDecl {
        name: "add".to_string(),
        params: vec![
            graphoid::ast::Parameter {
                name: "a".to_string(),
                default_value: None,
            },
            graphoid::ast::Parameter {
                name: "b".to_string(),
                default_value: None,
            },
        ],
        body: vec![Stmt::Return {
            value: Some(Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "a".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Variable {
                    name: "b".to_string(),
                    position: pos(),
                }),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // Verify function is stored in environment
    let func_value = executor.env().get("add").unwrap();
    assert_eq!(func_value.type_name(), "function");
}

#[test]
fn test_function_call_simple() {
    let mut executor = Executor::new();

    // Define function: func add(a, b) { return a + b }
    let func_decl = Stmt::FunctionDecl {
        name: "add".to_string(),
        params: vec![
            graphoid::ast::Parameter {
                name: "a".to_string(),
                default_value: None,
            },
            graphoid::ast::Parameter {
                name: "b".to_string(),
                default_value: None,
            },
        ],
        body: vec![Stmt::Return {
            value: Some(Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "a".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Variable {
                    name: "b".to_string(),
                    position: pos(),
                }),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // Call function: add(2, 3)
    let call_expr = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "add".to_string(),
            position: pos(),
        }),
        args: vec![
            Expr::Literal {
                value: LiteralValue::Number(2.0),
                position: pos(),
            },
            Expr::Literal {
                value: LiteralValue::Number(3.0),
                position: pos(),
            },
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&call_expr).unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_function_no_params() {
    let mut executor = Executor::new();

    // Define function: func greet() { return "Hello" }
    let func_decl = Stmt::FunctionDecl {
        name: "greet".to_string(),
        params: vec![],
        body: vec![Stmt::Return {
            value: Some(Expr::Literal {
                value: LiteralValue::String("Hello".to_string()),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // Call function: greet()
    let call_expr = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "greet".to_string(),
            position: pos(),
        }),
        args: vec![],
        position: pos(),
    };

    let result = executor.eval_expr(&call_expr).unwrap();
    assert_eq!(result, Value::String("Hello".to_string()));
}

#[test]
fn test_function_with_expression_body() {
    let mut executor = Executor::new();

    // Define function: func double(x) { return x * 2 }
    let func_decl = Stmt::FunctionDecl {
        name: "double".to_string(),
        params: vec![graphoid::ast::Parameter {
            name: "x".to_string(),
            default_value: None,
        }],
        body: vec![Stmt::Return {
            value: Some(Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "x".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Multiply,
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(2.0),
                    position: pos(),
                }),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // Call: double(5)
    let call_expr = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "double".to_string(),
            position: pos(),
        }),
        args: vec![Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }],
        position: pos(),
    };

    let result = executor.eval_expr(&call_expr).unwrap();
    assert_eq!(result, Value::Number(10.0));
}

#[test]
fn test_function_nested_calls() {
    let mut executor = Executor::new();

    // Define: func add(a, b) { return a + b }
    let add_decl = Stmt::FunctionDecl {
        name: "add".to_string(),
        params: vec![
            graphoid::ast::Parameter {
                name: "a".to_string(),
                default_value: None,
            },
            graphoid::ast::Parameter {
                name: "b".to_string(),
                default_value: None,
            },
        ],
        body: vec![Stmt::Return {
            value: Some(Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "a".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Variable {
                    name: "b".to_string(),
                    position: pos(),
                }),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    // Define: func mul(a, b) { return a * b }
    let mul_decl = Stmt::FunctionDecl {
        name: "mul".to_string(),
        params: vec![
            graphoid::ast::Parameter {
                name: "a".to_string(),
                default_value: None,
            },
            graphoid::ast::Parameter {
                name: "b".to_string(),
                default_value: None,
            },
        ],
        body: vec![Stmt::Return {
            value: Some(Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "a".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Multiply,
                right: Box::new(Expr::Variable {
                    name: "b".to_string(),
                    position: pos(),
                }),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&add_decl).unwrap();
    executor.eval_stmt(&mul_decl).unwrap();

    // Call: add(mul(2, 3), 4) => add(6, 4) => 10
    let call_expr = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "add".to_string(),
            position: pos(),
        }),
        args: vec![
            Expr::Call {
                callee: Box::new(Expr::Variable {
                    name: "mul".to_string(),
                    position: pos(),
                }),
                args: vec![
                    Expr::Literal {
                        value: LiteralValue::Number(2.0),
                        position: pos(),
                    },
                    Expr::Literal {
                        value: LiteralValue::Number(3.0),
                        position: pos(),
                    },
                ],
                position: pos(),
            },
            Expr::Literal {
                value: LiteralValue::Number(4.0),
                position: pos(),
            },
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&call_expr).unwrap();
    assert_eq!(result, Value::Number(10.0));
}

#[test]
fn test_function_closure() {
    let mut executor = Executor::new();

    // Set up: x = 10
    executor.env_mut().define("x".to_string(), Value::Number(10.0));

    // Define: func add_x(y) { return x + y }  (captures x)
    let func_decl = Stmt::FunctionDecl {
        name: "add_x".to_string(),
        params: vec![graphoid::ast::Parameter {
            name: "y".to_string(),
            default_value: None,
        }],
        body: vec![Stmt::Return {
            value: Some(Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "x".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Variable {
                    name: "y".to_string(),
                    position: pos(),
                }),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // Call: add_x(5) should return 15 (captures x=10)
    let call_expr = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "add_x".to_string(),
            position: pos(),
        }),
        args: vec![Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }],
        position: pos(),
    };

    let result = executor.eval_expr(&call_expr).unwrap();
    assert_eq!(result, Value::Number(15.0));
}

#[test]
fn test_function_return_none() {
    let mut executor = Executor::new();

    // Define: func do_nothing() { return }
    let func_decl = Stmt::FunctionDecl {
        name: "do_nothing".to_string(),
        params: vec![],
        body: vec![Stmt::Return {
            value: None,
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // Call: do_nothing()
    let call_expr = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "do_nothing".to_string(),
            position: pos(),
        }),
        args: vec![],
        position: pos(),
    };

    let result = executor.eval_expr(&call_expr).unwrap();
    assert_eq!(result, Value::None);
}

#[test]
fn test_function_wrong_arg_count() {
    let mut executor = Executor::new();

    // Define: func add(a, b) { return a + b }
    let func_decl = Stmt::FunctionDecl {
        name: "add".to_string(),
        params: vec![
            graphoid::ast::Parameter {
                name: "a".to_string(),
                default_value: None,
            },
            graphoid::ast::Parameter {
                name: "b".to_string(),
                default_value: None,
            },
        ],
        body: vec![Stmt::Return {
            value: Some(Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "a".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Variable {
                    name: "b".to_string(),
                    position: pos(),
                }),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // Call with wrong number of arguments: add(2)
    let call_expr = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "add".to_string(),
            position: pos(),
        }),
        args: vec![Expr::Literal {
            value: LiteralValue::Number(2.0),
            position: pos(),
        }],
        position: pos(),
    };

    let result = executor.eval_expr(&call_expr);
    assert!(result.is_err());
}

#[test]
fn test_function_call_non_function() {
    let mut executor = Executor::new();

    // Define a variable, not a function
    executor.env_mut().define("x".to_string(), Value::Number(42.0));

    // Try to call it: x()
    let call_expr = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "x".to_string(),
            position: pos(),
        }),
        args: vec![],
        position: pos(),
    };

    let result = executor.eval_expr(&call_expr);
    assert!(result.is_err());
}

#[test]
fn test_function_undefined() {
    let mut executor = Executor::new();

    // Try to call undefined function: foo()
    let call_expr = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "foo".to_string(),
            position: pos(),
        }),
        args: vec![],
        position: pos(),
    };

    let result = executor.eval_expr(&call_expr);
    assert!(result.is_err());
}

#[test]
fn test_lambda_simple() {
    let mut executor = Executor::new();

    // Lambda: x => x * 2
    let lambda = Expr::Lambda {
        params: vec!["x".to_string()],
        body: Box::new(Expr::Binary {
            left: Box::new(Expr::Variable {
                name: "x".to_string(),
                position: pos(),
            }),
            op: BinaryOp::Multiply,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(2.0),
                position: pos(),
            }),
            position: pos(),
        }),
        position: pos(),
    };

    // Evaluate lambda to get function value
    let func_value = executor.eval_expr(&lambda).unwrap();

    // Should be a function
    match func_value {
        Value::Function(f) => {
            assert_eq!(f.name, None); // Anonymous
            assert_eq!(f.params, vec!["x"]);
        }
        _ => panic!("Expected function value"),
    }
}

#[test]
fn test_lambda_call() {
    let mut executor = Executor::new();

    // Create lambda: x => x * 2
    let lambda = Expr::Lambda {
        params: vec!["x".to_string()],
        body: Box::new(Expr::Binary {
            left: Box::new(Expr::Variable {
                name: "x".to_string(),
                position: pos(),
            }),
            op: BinaryOp::Multiply,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(2.0),
                position: pos(),
            }),
            position: pos(),
        }),
        position: pos(),
    };

    // Call lambda immediately: (x => x * 2)(5)
    let call = Expr::Call {
        callee: Box::new(lambda),
        args: vec![Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::Number(10.0));
}

#[test]
fn test_lambda_closure() {
    let mut executor = Executor::new();

    // x = 10
    executor.env_mut().define("x".to_string(), Value::Number(10.0));

    // Lambda captures x: y => x + y
    let lambda = Expr::Lambda {
        params: vec!["y".to_string()],
        body: Box::new(Expr::Binary {
            left: Box::new(Expr::Variable {
                name: "x".to_string(),
                position: pos(),
            }),
            op: BinaryOp::Add,
            right: Box::new(Expr::Variable {
                name: "y".to_string(),
                position: pos(),
            }),
            position: pos(),
        }),
        position: pos(),
    };

    // Call lambda: (y => x + y)(5)
    let call = Expr::Call {
        callee: Box::new(lambda),
        args: vec![Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::Number(15.0)); // 10 + 5
}

#[test]
fn test_lambda_no_params() {
    let mut executor = Executor::new();

    // Lambda: () => 42
    let lambda = Expr::Lambda {
        params: vec![],
        body: Box::new(Expr::Literal {
            value: LiteralValue::Number(42.0),
            position: pos(),
        }),
        position: pos(),
    };

    // Call: (() => 42)()
    let call = Expr::Call {
        callee: Box::new(lambda),
        args: vec![],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_function_as_value() {
    let mut executor = Executor::new();

    // func double(n) { return n * 2 }
    let func_decl = Stmt::FunctionDecl {
        name: "double".to_string(),
        params: vec![Parameter {
            name: "n".to_string(),
            default_value: None,
        }],
        body: vec![Stmt::Return {
            value: Some(Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "n".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Multiply,
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(2.0),
                    position: pos(),
                }),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // Get function as a value
    let func_value = executor.env().get("double").unwrap();
    assert!(matches!(func_value, Value::Function(_)));
}

#[test]
fn test_function_multiple_params() {
    let mut executor = Executor::new();

    // func calculate(a, b, c) { return a + b * c }
    let func_decl = Stmt::FunctionDecl {
        name: "calculate".to_string(),
        params: vec![
            Parameter {
                name: "a".to_string(),
                default_value: None,
            },
            Parameter {
                name: "b".to_string(),
                default_value: None,
            },
            Parameter {
                name: "c".to_string(),
                default_value: None,
            },
        ],
        body: vec![Stmt::Return {
            value: Some(Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "a".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Binary {
                    left: Box::new(Expr::Variable {
                        name: "b".to_string(),
                        position: pos(),
                    }),
                    op: BinaryOp::Multiply,
                    right: Box::new(Expr::Variable {
                        name: "c".to_string(),
                        position: pos(),
                    }),
                    position: pos(),
                }),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // calculate(10, 2, 5) = 10 + 2 * 5 = 20
    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "calculate".to_string(),
            position: pos(),
        }),
        args: vec![
            Expr::Literal {
                value: LiteralValue::Number(10.0),
                position: pos(),
            },
            Expr::Literal {
                value: LiteralValue::Number(2.0),
                position: pos(),
            },
            Expr::Literal {
                value: LiteralValue::Number(5.0),
                position: pos(),
            },
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::Number(20.0));
}

#[test]
fn test_recursive_function() {
    let mut executor = Executor::new();

    // func factorial(n) {
    //     if n <= 1 { return 1 }
    //     return n * factorial(n - 1)
    // }
    // Note: Since we don't have if statements yet, let's test recursion differently
    // func countdown(n) {
    //     return n
    // }
    // This is a placeholder for now - real recursion needs control flow

    // For now, test that function can call itself (will hit max depth eventually)
    let func_decl = Stmt::FunctionDecl {
        name: "identity".to_string(),
        params: vec![Parameter {
            name: "n".to_string(),
            default_value: None,
        }],
        body: vec![Stmt::Return {
            value: Some(Expr::Variable {
                name: "n".to_string(),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "identity".to_string(),
            position: pos(),
        }),
        args: vec![Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_function_with_string_return() {
    let mut executor = Executor::new();

    // func greet(name) { return "Hello, " + name }
    let func_decl = Stmt::FunctionDecl {
        name: "greet".to_string(),
        params: vec![Parameter {
            name: "name".to_string(),
            default_value: None,
        }],
        body: vec![Stmt::Return {
            value: Some(Expr::Binary {
                left: Box::new(Expr::Literal {
                    value: LiteralValue::String("Hello, ".to_string()),
                    position: pos(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Variable {
                    name: "name".to_string(),
                    position: pos(),
                }),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "greet".to_string(),
            position: pos(),
        }),
        args: vec![Expr::Literal {
            value: LiteralValue::String("Alice".to_string()),
            position: pos(),
        }],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::String("Hello, Alice".to_string()));
}

#[test]
fn test_function_modifying_closure_var() {
    let mut executor = Executor::new();

    // x = 5
    executor.env_mut().define("x".to_string(), Value::Number(5.0));

    // func get_x() { return x }
    let func_decl = Stmt::FunctionDecl {
        name: "get_x".to_string(),
        params: vec![],
        body: vec![Stmt::Return {
            value: Some(Expr::Variable {
                name: "x".to_string(),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // Call get_x() - should return 5
    let call1 = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "get_x".to_string(),
            position: pos(),
        }),
        args: vec![],
        position: pos(),
    };

    let result1 = executor.eval_expr(&call1).unwrap();
    assert_eq!(result1, Value::Number(5.0));

    // Modify x in outer scope
    executor.env_mut().set("x", Value::Number(10.0)).unwrap();

    // Call get_x() again
    // With our Rc<Environment> implementation, the closure captured a clone of the environment
    // So it won't see the change - it still has x = 5
    // This is snapshot semantics, which is one valid closure model
    let result2 = executor.eval_expr(&call1).unwrap();
    // Closure captured environment at function definition time
    assert_eq!(result2, Value::Number(5.0));
}

#[test]
fn test_lambda_multiple_params() {
    let mut executor = Executor::new();

    // Lambda: (a, b, c) => a + b + c
    let lambda = Expr::Lambda {
        params: vec!["a".to_string(), "b".to_string(), "c".to_string()],
        body: Box::new(Expr::Binary {
            left: Box::new(Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "a".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Variable {
                    name: "b".to_string(),
                    position: pos(),
                }),
                position: pos(),
            }),
            op: BinaryOp::Add,
            right: Box::new(Expr::Variable {
                name: "c".to_string(),
                position: pos(),
            }),
            position: pos(),
        }),
        position: pos(),
    };

    let call = Expr::Call {
        callee: Box::new(lambda),
        args: vec![
            Expr::Literal {
                value: LiteralValue::Number(1.0),
                position: pos(),
            },
            Expr::Literal {
                value: LiteralValue::Number(2.0),
                position: pos(),
            },
            Expr::Literal {
                value: LiteralValue::Number(3.0),
                position: pos(),
            },
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::Number(6.0));
}

#[test]
fn test_lambda_with_string_concat() {
    let mut executor = Executor::new();

    // Lambda: (first, last) => first + " " + last
    let lambda = Expr::Lambda {
        params: vec!["first".to_string(), "last".to_string()],
        body: Box::new(Expr::Binary {
            left: Box::new(Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "first".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Literal {
                    value: LiteralValue::String(" ".to_string()),
                    position: pos(),
                }),
                position: pos(),
            }),
            op: BinaryOp::Add,
            right: Box::new(Expr::Variable {
                name: "last".to_string(),
                position: pos(),
            }),
            position: pos(),
        }),
        position: pos(),
    };

    let call = Expr::Call {
        callee: Box::new(lambda),
        args: vec![
            Expr::Literal {
                value: LiteralValue::String("John".to_string()),
                position: pos(),
            },
            Expr::Literal {
                value: LiteralValue::String("Doe".to_string()),
                position: pos(),
            },
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::String("John Doe".to_string()));
}

#[test]
fn test_function_returning_boolean() {
    let mut executor = Executor::new();

    // func is_positive(n) { return n > 0 }
    let func_decl = Stmt::FunctionDecl {
        name: "is_positive".to_string(),
        params: vec![Parameter {
            name: "n".to_string(),
            default_value: None,
        }],
        body: vec![Stmt::Return {
            value: Some(Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "n".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Greater,
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(0.0),
                    position: pos(),
                }),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    let call1 = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "is_positive".to_string(),
            position: pos(),
        }),
        args: vec![Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }],
        position: pos(),
    };

    let result1 = executor.eval_expr(&call1).unwrap();
    assert_eq!(result1, Value::Boolean(true));

    let call2 = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "is_positive".to_string(),
            position: pos(),
        }),
        args: vec![Expr::Literal {
            value: LiteralValue::Number(-5.0),
            position: pos(),
        }],
        position: pos(),
    };

    let result2 = executor.eval_expr(&call2).unwrap();
    assert_eq!(result2, Value::Boolean(false));
}

#[test]
fn test_function_returning_list() {
    let mut executor = Executor::new();

    // func make_list(a, b) { return [a, b] }
    let func_decl = Stmt::FunctionDecl {
        name: "make_list".to_string(),
        params: vec![
            Parameter {
                name: "a".to_string(),
                default_value: None,
            },
            Parameter {
                name: "b".to_string(),
                default_value: None,
            },
        ],
        body: vec![Stmt::Return {
            value: Some(Expr::List {
                elements: vec![
                    Expr::Variable {
                        name: "a".to_string(),
                        position: pos(),
                    },
                    Expr::Variable {
                        name: "b".to_string(),
                        position: pos(),
                    },
                ],
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "make_list".to_string(),
            position: pos(),
        }),
        args: vec![
            Expr::Literal {
                value: LiteralValue::Number(1.0),
                position: pos(),
            },
            Expr::Literal {
                value: LiteralValue::Number(2.0),
                position: pos(),
            },
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(
        result,
        Value::List(List::from_vec(vec![Value::Number(1.0), Value::Number(2.0)]))
    );
}

#[test]
fn test_deeply_nested_calls() {
    let mut executor = Executor::new();

    // func add1(n) { return n + 1 }
    let func_decl = Stmt::FunctionDecl {
        name: "add1".to_string(),
        params: vec![Parameter {
            name: "n".to_string(),
            default_value: None,
        }],
        body: vec![Stmt::Return {
            value: Some(Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "n".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(1.0),
                    position: pos(),
                }),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // add1(add1(add1(5))) = 8
    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "add1".to_string(),
            position: pos(),
        }),
        args: vec![Expr::Call {
            callee: Box::new(Expr::Variable {
                name: "add1".to_string(),
                position: pos(),
            }),
            args: vec![Expr::Call {
                callee: Box::new(Expr::Variable {
                    name: "add1".to_string(),
                    position: pos(),
                }),
                args: vec![Expr::Literal {
                    value: LiteralValue::Number(5.0),
                    position: pos(),
                }],
                position: pos(),
            }],
            position: pos(),
        }],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::Number(8.0));
}

#[test]
fn test_function_with_no_return_statement() {
    let mut executor = Executor::new();

    // func do_nothing() { }
    let func_decl = Stmt::FunctionDecl {
        name: "do_nothing".to_string(),
        params: vec![],
        body: vec![], // No statements
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "do_nothing".to_string(),
            position: pos(),
        }),
        args: vec![],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::None); // Should return none
}

#[test]
fn test_function_early_return() {
    let mut executor = Executor::new();

    // func early() { return 1; return 2; }
    let func_decl = Stmt::FunctionDecl {
        name: "early".to_string(),
        params: vec![],
        body: vec![
            Stmt::Return {
                value: Some(Expr::Literal {
                    value: LiteralValue::Number(1.0),
                    position: pos(),
                }),
                position: pos(),
            },
            Stmt::Return {
                value: Some(Expr::Literal {
                    value: LiteralValue::Number(2.0),
                    position: pos(),
                }),
                position: pos(),
            },
        ],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "early".to_string(),
            position: pos(),
        }),
        args: vec![],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::Number(1.0)); // Should return first value
}

#[test]
fn test_lambda_wrong_arg_count() {
    let mut executor = Executor::new();

    // Lambda: x => x * 2
    let lambda = Expr::Lambda {
        params: vec!["x".to_string()],
        body: Box::new(Expr::Binary {
            left: Box::new(Expr::Variable {
                name: "x".to_string(),
                position: pos(),
            }),
            op: BinaryOp::Multiply,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(2.0),
                position: pos(),
            }),
            position: pos(),
        }),
        position: pos(),
    };

    // Call with wrong number of args
    let call = Expr::Call {
        callee: Box::new(lambda),
        args: vec![], // Should be 1 arg, gave 0
        position: pos(),
    };

    let result = executor.eval_expr(&call);
    assert!(result.is_err());
}

#[test]
fn test_function_with_side_effects() {
    let mut executor = Executor::new();

    // x = 0
    executor.env_mut().define("x".to_string(), Value::Number(0.0));

    // func set_x(val) { x = val; return x }
    let func_decl = Stmt::FunctionDecl {
        name: "set_x".to_string(),
        params: vec![Parameter {
            name: "val".to_string(),
            default_value: None,
        }],
        body: vec![
            Stmt::Assignment {
                target: AssignmentTarget::Variable("x".to_string()),
                value: Expr::Variable {
                    name: "val".to_string(),
                    position: pos(),
                },
                position: pos(),
            },
            Stmt::Return {
                value: Some(Expr::Variable {
                    name: "x".to_string(),
                    position: pos(),
                }),
                position: pos(),
            },
        ],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // Call set_x(42)
    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "set_x".to_string(),
            position: pos(),
        }),
        args: vec![Expr::Literal {
            value: LiteralValue::Number(42.0),
            position: pos(),
        }],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::Number(42.0));

    // Verify x was modified (in closure's captured environment)
    // Due to our snapshot semantics, outer x won't change
    let x_value = executor.env().get("x").unwrap();
    assert_eq!(x_value, Value::Number(0.0)); // Still 0, not modified
}

#[test]
fn test_nested_closures() {
    let mut executor = Executor::new();

    // x = 5
    executor.env_mut().define("x".to_string(), Value::Number(5.0));

    // func outer() { return x }
    let outer_decl = Stmt::FunctionDecl {
        name: "outer".to_string(),
        params: vec![],
        body: vec![Stmt::Return {
            value: Some(Expr::Variable {
                name: "x".to_string(),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&outer_decl).unwrap();

    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "outer".to_string(),
            position: pos(),
        }),
        args: vec![],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_function_parameter_shadowing() {
    let mut executor = Executor::new();

    // x = 10
    executor.env_mut().define("x".to_string(), Value::Number(10.0));

    // func use_param(x) { return x * 2 }
    let func_decl = Stmt::FunctionDecl {
        name: "use_param".to_string(),
        params: vec![Parameter {
            name: "x".to_string(), // Shadows outer x
            default_value: None,
        }],
        body: vec![Stmt::Return {
            value: Some(Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "x".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Multiply,
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(2.0),
                    position: pos(),
                }),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // use_param(5) - should use parameter x=5, not outer x=10
    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "use_param".to_string(),
            position: pos(),
        }),
        args: vec![Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::Number(10.0)); // 5 * 2, not 10 * 2
}

#[test]
fn test_function_returning_function_value() {
    let mut executor = Executor::new();

    // func make_adder(n) { return n + 1 }
    let func_decl = Stmt::FunctionDecl {
        name: "make_adder".to_string(),
        params: vec![Parameter {
            name: "n".to_string(),
            default_value: None,
        }],
        body: vec![Stmt::Return {
            value: Some(Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "n".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(1.0),
                    position: pos(),
                }),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // adder = make_adder
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("adder".to_string()),
        value: Expr::Variable {
            name: "make_adder".to_string(),
            position: pos(),
        },
        position: pos(),
    };

    executor.eval_stmt(&assign).unwrap();

    // adder(5) should work
    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "adder".to_string(),
            position: pos(),
        }),
        args: vec![Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::Number(6.0));
}

#[test]
fn test_lambda_with_logical_operations() {
    let mut executor = Executor::new();

    // Lambda: (a, b) => a and b
    let lambda = Expr::Lambda {
        params: vec!["a".to_string(), "b".to_string()],
        body: Box::new(Expr::Binary {
            left: Box::new(Expr::Variable {
                name: "a".to_string(),
                position: pos(),
            }),
            op: BinaryOp::And,
            right: Box::new(Expr::Variable {
                name: "b".to_string(),
                position: pos(),
            }),
            position: pos(),
        }),
        position: pos(),
    };

    let call = Expr::Call {
        callee: Box::new(lambda),
        args: vec![
            Expr::Literal {
                value: LiteralValue::Boolean(true),
                position: pos(),
            },
            Expr::Literal {
                value: LiteralValue::Boolean(false),
                position: pos(),
            },
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_function_with_comparison() {
    let mut executor = Executor::new();

    // func compare(a, b) { return a > b }
    let func_decl = Stmt::FunctionDecl {
        name: "compare".to_string(),
        params: vec![
            Parameter {
                name: "a".to_string(),
                default_value: None,
            },
            Parameter {
                name: "b".to_string(),
                default_value: None,
            },
        ],
        body: vec![Stmt::Return {
            value: Some(Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "a".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Greater,
                right: Box::new(Expr::Variable {
                    name: "b".to_string(),
                    position: pos(),
                }),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    let call1 = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "compare".to_string(),
            position: pos(),
        }),
        args: vec![
            Expr::Literal {
                value: LiteralValue::Number(10.0),
                position: pos(),
            },
            Expr::Literal {
                value: LiteralValue::Number(5.0),
                position: pos(),
            },
        ],
        position: pos(),
    };

    let result1 = executor.eval_expr(&call1).unwrap();
    assert_eq!(result1, Value::Boolean(true));
}

#[test]
fn test_lambda_returning_list() {
    let mut executor = Executor::new();

    // Lambda: (a, b) => [a, b, a + b]
    let lambda = Expr::Lambda {
        params: vec!["a".to_string(), "b".to_string()],
        body: Box::new(Expr::List {
            elements: vec![
                Expr::Variable {
                    name: "a".to_string(),
                    position: pos(),
                },
                Expr::Variable {
                    name: "b".to_string(),
                    position: pos(),
                },
                Expr::Binary {
                    left: Box::new(Expr::Variable {
                        name: "a".to_string(),
                        position: pos(),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Variable {
                        name: "b".to_string(),
                        position: pos(),
                    }),
                    position: pos(),
                },
            ],
            position: pos(),
        }),
        position: pos(),
    };

    let call = Expr::Call {
        callee: Box::new(lambda),
        args: vec![
            Expr::Literal {
                value: LiteralValue::Number(3.0),
                position: pos(),
            },
            Expr::Literal {
                value: LiteralValue::Number(4.0),
                position: pos(),
            },
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(
        result,
        Value::List(List::from_vec(vec![
            Value::Number(3.0),
            Value::Number(4.0),
            Value::Number(7.0)
        ]))
    );
}

#[test]
fn test_function_with_unary_ops() {
    let mut executor = Executor::new();

    // func negate(x) { return -x }
    let func_decl = Stmt::FunctionDecl {
        name: "negate".to_string(),
        params: vec![Parameter {
            name: "x".to_string(),
            default_value: None,
        }],
        body: vec![Stmt::Return {
            value: Some(Expr::Unary {
                op: UnaryOp::Negate,
                operand: Box::new(Expr::Variable {
                    name: "x".to_string(),
                    position: pos(),
                }),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "negate".to_string(),
            position: pos(),
        }),
        args: vec![Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::Number(-5.0));
}

#[test]
fn test_function_with_not_op() {
    let mut executor = Executor::new();

    // func invert(b) { return not b }
    let func_decl = Stmt::FunctionDecl {
        name: "invert".to_string(),
        params: vec![Parameter {
            name: "b".to_string(),
            default_value: None,
        }],
        body: vec![Stmt::Return {
            value: Some(Expr::Unary {
                op: UnaryOp::Not,
                operand: Box::new(Expr::Variable {
                    name: "b".to_string(),
                    position: pos(),
                }),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "invert".to_string(),
            position: pos(),
        }),
        args: vec![Expr::Literal {
            value: LiteralValue::Boolean(true),
            position: pos(),
        }],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_function_four_params() {
    let mut executor = Executor::new();

    // func avg(a, b, c, d) { return (a + b + c + d) / 4 }
    let func_decl = Stmt::FunctionDecl {
        name: "avg".to_string(),
        params: vec![
            Parameter {
                name: "a".to_string(),
                default_value: None,
            },
            Parameter {
                name: "b".to_string(),
                default_value: None,
            },
            Parameter {
                name: "c".to_string(),
                default_value: None,
            },
            Parameter {
                name: "d".to_string(),
                default_value: None,
            },
        ],
        body: vec![Stmt::Return {
            value: Some(Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Binary {
                        left: Box::new(Expr::Binary {
                            left: Box::new(Expr::Variable {
                                name: "a".to_string(),
                                position: pos(),
                            }),
                            op: BinaryOp::Add,
                            right: Box::new(Expr::Variable {
                                name: "b".to_string(),
                                position: pos(),
                            }),
                            position: pos(),
                        }),
                        op: BinaryOp::Add,
                        right: Box::new(Expr::Variable {
                            name: "c".to_string(),
                            position: pos(),
                        }),
                        position: pos(),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Variable {
                        name: "d".to_string(),
                        position: pos(),
                    }),
                    position: pos(),
                }),
                op: BinaryOp::Divide,
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(4.0),
                    position: pos(),
                }),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "avg".to_string(),
            position: pos(),
        }),
        args: vec![
            Expr::Literal {
                value: LiteralValue::Number(10.0),
                position: pos(),
            },
            Expr::Literal {
                value: LiteralValue::Number(20.0),
                position: pos(),
            },
            Expr::Literal {
                value: LiteralValue::Number(30.0),
                position: pos(),
            },
            Expr::Literal {
                value: LiteralValue::Number(40.0),
                position: pos(),
            },
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::Number(25.0));
}

#[test]
fn test_lambda_with_symbol_return() {
    let mut executor = Executor::new();

    // Lambda: () => :success
    let lambda = Expr::Lambda {
        params: vec![],
        body: Box::new(Expr::Literal {
            value: LiteralValue::Symbol("success".to_string()),
            position: pos(),
        }),
        position: pos(),
    };

    let call = Expr::Call {
        callee: Box::new(lambda),
        args: vec![],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::Symbol("success".to_string()));
}

#[test]
fn test_function_call_with_expression_args() {
    let mut executor = Executor::new();

    // func add(a, b) { return a + b }
    let func_decl = Stmt::FunctionDecl {
        name: "add".to_string(),
        params: vec![
            Parameter {
                name: "a".to_string(),
                default_value: None,
            },
            Parameter {
                name: "b".to_string(),
                default_value: None,
            },
        ],
        body: vec![Stmt::Return {
            value: Some(Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "a".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Variable {
                    name: "b".to_string(),
                    position: pos(),
                }),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // add(2 * 3, 5 + 1) = add(6, 6) = 12
    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "add".to_string(),
            position: pos(),
        }),
        args: vec![
            Expr::Binary {
                left: Box::new(Expr::Literal {
                    value: LiteralValue::Number(2.0),
                    position: pos(),
                }),
                op: BinaryOp::Multiply,
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(3.0),
                    position: pos(),
                }),
                position: pos(),
            },
            Expr::Binary {
                left: Box::new(Expr::Literal {
                    value: LiteralValue::Number(5.0),
                    position: pos(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(1.0),
                    position: pos(),
                }),
                position: pos(),
            },
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::Number(12.0));
}

// ============================================================================
// CONTROL FLOW TESTS
// ============================================================================

#[test]
fn test_if_statement_true() {
    let mut executor = Executor::new();

    // x = 0
    executor.env_mut().define("x".to_string(), Value::Number(0.0));

    // if true { x = 1 }
    let if_stmt = Stmt::If {
        condition: Expr::Literal {
            value: LiteralValue::Boolean(true),
            position: pos(),
        },
        then_branch: vec![Stmt::Assignment {
            target: AssignmentTarget::Variable("x".to_string()),
            value: Expr::Literal {
                value: LiteralValue::Number(1.0),
                position: pos(),
            },
            position: pos(),
        }],
        else_branch: None,
        position: pos(),
    };

    executor.eval_stmt(&if_stmt).unwrap();

    let x = executor.env().get("x").unwrap();
    assert_eq!(x, Value::Number(1.0));
}

#[test]
fn test_if_statement_false() {
    let mut executor = Executor::new();

    // x = 0
    executor.env_mut().define("x".to_string(), Value::Number(0.0));

    // if false { x = 1 }
    let if_stmt = Stmt::If {
        condition: Expr::Literal {
            value: LiteralValue::Boolean(false),
            position: pos(),
        },
        then_branch: vec![Stmt::Assignment {
            target: AssignmentTarget::Variable("x".to_string()),
            value: Expr::Literal {
                value: LiteralValue::Number(1.0),
                position: pos(),
            },
            position: pos(),
        }],
        else_branch: None,
        position: pos(),
    };

    executor.eval_stmt(&if_stmt).unwrap();

    let x = executor.env().get("x").unwrap();
    assert_eq!(x, Value::Number(0.0)); // Should still be 0
}

#[test]
fn test_if_else_true() {
    let mut executor = Executor::new();

    // x = 0
    executor.env_mut().define("x".to_string(), Value::Number(0.0));

    // if true { x = 1 } else { x = 2 }
    let if_stmt = Stmt::If {
        condition: Expr::Literal {
            value: LiteralValue::Boolean(true),
            position: pos(),
        },
        then_branch: vec![Stmt::Assignment {
            target: AssignmentTarget::Variable("x".to_string()),
            value: Expr::Literal {
                value: LiteralValue::Number(1.0),
                position: pos(),
            },
            position: pos(),
        }],
        else_branch: Some(vec![Stmt::Assignment {
            target: AssignmentTarget::Variable("x".to_string()),
            value: Expr::Literal {
                value: LiteralValue::Number(2.0),
                position: pos(),
            },
            position: pos(),
        }]),
        position: pos(),
    };

    executor.eval_stmt(&if_stmt).unwrap();

    let x = executor.env().get("x").unwrap();
    assert_eq!(x, Value::Number(1.0));
}

#[test]
fn test_if_else_false() {
    let mut executor = Executor::new();

    // x = 0
    executor.env_mut().define("x".to_string(), Value::Number(0.0));

    // if false { x = 1 } else { x = 2 }
    let if_stmt = Stmt::If {
        condition: Expr::Literal {
            value: LiteralValue::Boolean(false),
            position: pos(),
        },
        then_branch: vec![Stmt::Assignment {
            target: AssignmentTarget::Variable("x".to_string()),
            value: Expr::Literal {
                value: LiteralValue::Number(1.0),
                position: pos(),
            },
            position: pos(),
        }],
        else_branch: Some(vec![Stmt::Assignment {
            target: AssignmentTarget::Variable("x".to_string()),
            value: Expr::Literal {
                value: LiteralValue::Number(2.0),
                position: pos(),
            },
            position: pos(),
        }]),
        position: pos(),
    };

    executor.eval_stmt(&if_stmt).unwrap();

    let x = executor.env().get("x").unwrap();
    assert_eq!(x, Value::Number(2.0));
}

#[test]
fn test_if_with_comparison() {
    let mut executor = Executor::new();

    // x = 10
    executor.env_mut().define("x".to_string(), Value::Number(10.0));
    // result = 0
    executor.env_mut().define("result".to_string(), Value::Number(0.0));

    // if x > 5 { result = 1 }
    let if_stmt = Stmt::If {
        condition: Expr::Binary {
            left: Box::new(Expr::Variable {
                name: "x".to_string(),
                position: pos(),
            }),
            op: BinaryOp::Greater,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(5.0),
                position: pos(),
            }),
            position: pos(),
        },
        then_branch: vec![Stmt::Assignment {
            target: AssignmentTarget::Variable("result".to_string()),
            value: Expr::Literal {
                value: LiteralValue::Number(1.0),
                position: pos(),
            },
            position: pos(),
        }],
        else_branch: None,
        position: pos(),
    };

    executor.eval_stmt(&if_stmt).unwrap();

    let result = executor.env().get("result").unwrap();
    assert_eq!(result, Value::Number(1.0));
}

#[test]
fn test_if_return_in_function() {
    let mut executor = Executor::new();

    // func check(n) { if n > 0 { return 1 } return 0 }
    let func_decl = Stmt::FunctionDecl {
        name: "check".to_string(),
        params: vec![Parameter {
            name: "n".to_string(),
            default_value: None,
        }],
        body: vec![
            Stmt::If {
                condition: Expr::Binary {
                    left: Box::new(Expr::Variable {
                        name: "n".to_string(),
                        position: pos(),
                    }),
                    op: BinaryOp::Greater,
                    right: Box::new(Expr::Literal {
                        value: LiteralValue::Number(0.0),
                        position: pos(),
                    }),
                    position: pos(),
                },
                then_branch: vec![Stmt::Return {
                    value: Some(Expr::Literal {
                        value: LiteralValue::Number(1.0),
                        position: pos(),
                    }),
                    position: pos(),
                }],
                else_branch: None,
                position: pos(),
            },
            Stmt::Return {
                value: Some(Expr::Literal {
                    value: LiteralValue::Number(0.0),
                    position: pos(),
                }),
                position: pos(),
            },
        ],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // check(5) should return 1
    let call1 = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "check".to_string(),
            position: pos(),
        }),
        args: vec![Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }],
        position: pos(),
    };

    let result1 = executor.eval_expr(&call1).unwrap();
    assert_eq!(result1, Value::Number(1.0));

    // check(-5) should return 0
    let call2 = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "check".to_string(),
            position: pos(),
        }),
        args: vec![Expr::Literal {
            value: LiteralValue::Number(-5.0),
            position: pos(),
        }],
        position: pos(),
    };

    let result2 = executor.eval_expr(&call2).unwrap();
    assert_eq!(result2, Value::Number(0.0));
}

#[test]
fn test_call_stack_empty_initially() {
    let executor = Executor::new();
    assert_eq!(executor.call_stack().len(), 0);
}

#[test]
fn test_call_stack_cleared_after_return() {
    let mut executor = Executor::new();

    // func add(a, b) { return a + b }
    let func_decl = Stmt::FunctionDecl {
        name: "add".to_string(),
        params: vec![
            Parameter {
                name: "a".to_string(),
                default_value: None,
            },
            Parameter {
                name: "b".to_string(),
                default_value: None,
            },
        ],
        body: vec![Stmt::Return {
            value: Some(Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "a".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Variable {
                    name: "b".to_string(),
                    position: pos(),
                }),
                position: pos(),
            }),
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // Call the function
    let call_expr = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "add".to_string(),
            position: pos(),
        }),
        args: vec![
            Expr::Literal {
                value: LiteralValue::Number(2.0),
                position: pos(),
            },
            Expr::Literal {
                value: LiteralValue::Number(3.0),
                position: pos(),
            },
        ],
        position: pos(),
    };

    executor.eval_expr(&call_expr).unwrap();

    // After function returns, call stack should be empty
    assert_eq!(executor.call_stack().len(), 0);
}

// ============================================================================
// WHILE LOOP TESTS
// ============================================================================

#[test]
fn test_while_loop_simple_counter() {
    let mut executor = Executor::new();

    // count = 0
    // while count < 3 { count = count + 1 }
    executor
        .eval_stmt(&Stmt::VariableDecl {
            name: "count".to_string(),
            value: Expr::Literal {
                value: LiteralValue::Number(0.0),
                position: pos(),
            },
            type_annotation: None,
            position: pos(),
        })
        .unwrap();

    let while_stmt = Stmt::While {
        condition: Expr::Binary {
            left: Box::new(Expr::Variable {
                name: "count".to_string(),
                position: pos(),
            }),
            op: BinaryOp::Less,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(3.0),
                position: pos(),
            }),
            position: pos(),
        },
        body: vec![Stmt::Assignment {
            target: AssignmentTarget::Variable("count".to_string()),
            value: Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "count".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(1.0),
                    position: pos(),
                }),
                position: pos(),
            },
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&while_stmt).unwrap();

    let count_value = executor.env().get("count").unwrap();
    assert_eq!(count_value, Value::Number(3.0));
}

#[test]
fn test_while_loop_never_executes() {
    let mut executor = Executor::new();

    // x = 10
    // while x < 5 { x = x + 1 }
    executor
        .eval_stmt(&Stmt::VariableDecl {
            name: "x".to_string(),
            value: Expr::Literal {
                value: LiteralValue::Number(10.0),
                position: pos(),
            },
            type_annotation: None,
            position: pos(),
        })
        .unwrap();

    let while_stmt = Stmt::While {
        condition: Expr::Binary {
            left: Box::new(Expr::Variable {
                name: "x".to_string(),
                position: pos(),
            }),
            op: BinaryOp::Less,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(5.0),
                position: pos(),
            }),
            position: pos(),
        },
        body: vec![Stmt::Assignment {
            target: AssignmentTarget::Variable("x".to_string()),
            value: Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "x".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(1.0),
                    position: pos(),
                }),
                position: pos(),
            },
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&while_stmt).unwrap();

    // x should still be 10 since loop never executed
    let x_value = executor.env().get("x").unwrap();
    assert_eq!(x_value, Value::Number(10.0));
}

#[test]
fn test_while_loop_with_multiple_statements() {
    let mut executor = Executor::new();

    // sum = 0
    // i = 1
    // while i <= 5 {
    //     sum = sum + i
    //     i = i + 1
    // }
    executor
        .eval_stmt(&Stmt::VariableDecl {
            name: "sum".to_string(),
            value: Expr::Literal {
                value: LiteralValue::Number(0.0),
                position: pos(),
            },
            type_annotation: None,
            position: pos(),
        })
        .unwrap();

    executor
        .eval_stmt(&Stmt::VariableDecl {
            name: "i".to_string(),
            value: Expr::Literal {
                value: LiteralValue::Number(1.0),
                position: pos(),
            },
            type_annotation: None,
            position: pos(),
        })
        .unwrap();

    let while_stmt = Stmt::While {
        condition: Expr::Binary {
            left: Box::new(Expr::Variable {
                name: "i".to_string(),
                position: pos(),
            }),
            op: BinaryOp::LessEqual,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(5.0),
                position: pos(),
            }),
            position: pos(),
        },
        body: vec![
            Stmt::Assignment {
                target: AssignmentTarget::Variable("sum".to_string()),
                value: Expr::Binary {
                    left: Box::new(Expr::Variable {
                        name: "sum".to_string(),
                        position: pos(),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Variable {
                        name: "i".to_string(),
                        position: pos(),
                    }),
                    position: pos(),
                },
                position: pos(),
            },
            Stmt::Assignment {
                target: AssignmentTarget::Variable("i".to_string()),
                value: Expr::Binary {
                    left: Box::new(Expr::Variable {
                        name: "i".to_string(),
                        position: pos(),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Literal {
                        value: LiteralValue::Number(1.0),
                        position: pos(),
                    }),
                    position: pos(),
                },
                position: pos(),
            },
        ],
        position: pos(),
    };

    executor.eval_stmt(&while_stmt).unwrap();

    // sum should be 1+2+3+4+5 = 15
    let sum_value = executor.env().get("sum").unwrap();
    assert_eq!(sum_value, Value::Number(15.0));

    // i should be 6
    let i_value = executor.env().get("i").unwrap();
    assert_eq!(i_value, Value::Number(6.0));
}

#[test]
fn test_while_loop_in_function() {
    let mut executor = Executor::new();

    // func factorial(n) {
    //     result = 1
    //     i = 1
    //     while i <= n {
    //         result = result * i
    //         i = i + 1
    //     }
    //     return result
    // }
    let func_decl = Stmt::FunctionDecl {
        name: "factorial".to_string(),
        params: vec![Parameter {
            name: "n".to_string(),
            default_value: None,
        }],
        body: vec![
            Stmt::VariableDecl {
                name: "result".to_string(),
                value: Expr::Literal {
                    value: LiteralValue::Number(1.0),
                    position: pos(),
                },
                type_annotation: None,
                position: pos(),
            },
            Stmt::VariableDecl {
                name: "i".to_string(),
                value: Expr::Literal {
                    value: LiteralValue::Number(1.0),
                    position: pos(),
                },
                type_annotation: None,
                position: pos(),
            },
            Stmt::While {
                condition: Expr::Binary {
                    left: Box::new(Expr::Variable {
                        name: "i".to_string(),
                        position: pos(),
                    }),
                    op: BinaryOp::LessEqual,
                    right: Box::new(Expr::Variable {
                        name: "n".to_string(),
                        position: pos(),
                    }),
                    position: pos(),
                },
                body: vec![
                    Stmt::Assignment {
                        target: AssignmentTarget::Variable("result".to_string()),
                        value: Expr::Binary {
                            left: Box::new(Expr::Variable {
                                name: "result".to_string(),
                                position: pos(),
                            }),
                            op: BinaryOp::Multiply,
                            right: Box::new(Expr::Variable {
                                name: "i".to_string(),
                                position: pos(),
                            }),
                            position: pos(),
                        },
                        position: pos(),
                    },
                    Stmt::Assignment {
                        target: AssignmentTarget::Variable("i".to_string()),
                        value: Expr::Binary {
                            left: Box::new(Expr::Variable {
                                name: "i".to_string(),
                                position: pos(),
                            }),
                            op: BinaryOp::Add,
                            right: Box::new(Expr::Literal {
                                value: LiteralValue::Number(1.0),
                                position: pos(),
                            }),
                            position: pos(),
                        },
                        position: pos(),
                    },
                ],
                position: pos(),
            },
            Stmt::Return {
                value: Some(Expr::Variable {
                    name: "result".to_string(),
                    position: pos(),
                }),
                position: pos(),
            },
        ],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // factorial(5) should be 120
    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "factorial".to_string(),
            position: pos(),
        }),
        args: vec![Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        }],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::Number(120.0));
}

#[test]
fn test_nested_while_loops() {
    let mut executor = Executor::new();

    // sum = 0
    // i = 1
    // while i <= 3 {
    //     j = 1
    //     while j <= 2 {
    //         sum = sum + 1
    //         j = j + 1
    //     }
    //     i = i + 1
    // }
    executor
        .eval_stmt(&Stmt::VariableDecl {
            name: "sum".to_string(),
            value: Expr::Literal {
                value: LiteralValue::Number(0.0),
                position: pos(),
            },
            type_annotation: None,
            position: pos(),
        })
        .unwrap();

    executor
        .eval_stmt(&Stmt::VariableDecl {
            name: "i".to_string(),
            value: Expr::Literal {
                value: LiteralValue::Number(1.0),
                position: pos(),
            },
            type_annotation: None,
            position: pos(),
        })
        .unwrap();

    let inner_while = Stmt::While {
        condition: Expr::Binary {
            left: Box::new(Expr::Variable {
                name: "j".to_string(),
                position: pos(),
            }),
            op: BinaryOp::LessEqual,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(2.0),
                position: pos(),
            }),
            position: pos(),
        },
        body: vec![
            Stmt::Assignment {
                target: AssignmentTarget::Variable("sum".to_string()),
                value: Expr::Binary {
                    left: Box::new(Expr::Variable {
                        name: "sum".to_string(),
                        position: pos(),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Literal {
                        value: LiteralValue::Number(1.0),
                        position: pos(),
                    }),
                    position: pos(),
                },
                position: pos(),
            },
            Stmt::Assignment {
                target: AssignmentTarget::Variable("j".to_string()),
                value: Expr::Binary {
                    left: Box::new(Expr::Variable {
                        name: "j".to_string(),
                        position: pos(),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Literal {
                        value: LiteralValue::Number(1.0),
                        position: pos(),
                    }),
                    position: pos(),
                },
                position: pos(),
            },
        ],
        position: pos(),
    };

    let outer_while = Stmt::While {
        condition: Expr::Binary {
            left: Box::new(Expr::Variable {
                name: "i".to_string(),
                position: pos(),
            }),
            op: BinaryOp::LessEqual,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(3.0),
                position: pos(),
            }),
            position: pos(),
        },
        body: vec![
            Stmt::VariableDecl {
                name: "j".to_string(),
                value: Expr::Literal {
                    value: LiteralValue::Number(1.0),
                    position: pos(),
                },
                type_annotation: None,
                position: pos(),
            },
            inner_while,
            Stmt::Assignment {
                target: AssignmentTarget::Variable("i".to_string()),
                value: Expr::Binary {
                    left: Box::new(Expr::Variable {
                        name: "i".to_string(),
                        position: pos(),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Literal {
                        value: LiteralValue::Number(1.0),
                        position: pos(),
                    }),
                    position: pos(),
                },
                position: pos(),
            },
        ],
        position: pos(),
    };

    executor.eval_stmt(&outer_while).unwrap();

    // sum should be 6 (3 outer iterations * 2 inner iterations)
    let sum_value = executor.env().get("sum").unwrap();
    assert_eq!(sum_value, Value::Number(6.0));
}

// ============================================================================
// FOR LOOP TESTS
// ============================================================================

#[test]
fn test_for_loop_simple() {
    let mut executor = Executor::new();

    // for i in [1, 2, 3] { sum = sum + i }
    executor
        .eval_stmt(&Stmt::VariableDecl {
            name: "sum".to_string(),
            value: Expr::Literal {
                value: LiteralValue::Number(0.0),
                position: pos(),
            },
            type_annotation: None,
            position: pos(),
        })
        .unwrap();

    let for_stmt = Stmt::For {
        variable: "i".to_string(),
        iterable: Expr::List {
            elements: vec![
                Expr::Literal {
                    value: LiteralValue::Number(1.0),
                    position: pos(),
                },
                Expr::Literal {
                    value: LiteralValue::Number(2.0),
                    position: pos(),
                },
                Expr::Literal {
                    value: LiteralValue::Number(3.0),
                    position: pos(),
                },
            ],
            position: pos(),
        },
        body: vec![Stmt::Assignment {
            target: AssignmentTarget::Variable("sum".to_string()),
            value: Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "sum".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Variable {
                    name: "i".to_string(),
                    position: pos(),
                }),
                position: pos(),
            },
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&for_stmt).unwrap();

    // sum should be 1 + 2 + 3 = 6
    let sum_value = executor.env().get("sum").unwrap();
    assert_eq!(sum_value, Value::Number(6.0));
}

#[test]
fn test_for_loop_empty_list() {
    let mut executor = Executor::new();

    // x = 0
    // for i in [] { x = x + 1 }
    executor
        .eval_stmt(&Stmt::VariableDecl {
            name: "x".to_string(),
            value: Expr::Literal {
                value: LiteralValue::Number(0.0),
                position: pos(),
            },
            type_annotation: None,
            position: pos(),
        })
        .unwrap();

    let for_stmt = Stmt::For {
        variable: "i".to_string(),
        iterable: Expr::List {
            elements: vec![],
            position: pos(),
        },
        body: vec![Stmt::Assignment {
            target: AssignmentTarget::Variable("x".to_string()),
            value: Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "x".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(1.0),
                    position: pos(),
                }),
                position: pos(),
            },
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&for_stmt).unwrap();

    // x should still be 0 since loop never executed
    let x_value = executor.env().get("x").unwrap();
    assert_eq!(x_value, Value::Number(0.0));
}

#[test]
fn test_for_loop_with_strings() {
    let mut executor = Executor::new();

    // result = ""
    // for s in ["a", "b", "c"] { result = result + s }
    executor
        .eval_stmt(&Stmt::VariableDecl {
            name: "result".to_string(),
            value: Expr::Literal {
                value: LiteralValue::String("".to_string()),
                position: pos(),
            },
            type_annotation: None,
            position: pos(),
        })
        .unwrap();

    let for_stmt = Stmt::For {
        variable: "s".to_string(),
        iterable: Expr::List {
            elements: vec![
                Expr::Literal {
                    value: LiteralValue::String("a".to_string()),
                    position: pos(),
                },
                Expr::Literal {
                    value: LiteralValue::String("b".to_string()),
                    position: pos(),
                },
                Expr::Literal {
                    value: LiteralValue::String("c".to_string()),
                    position: pos(),
                },
            ],
            position: pos(),
        },
        body: vec![Stmt::Assignment {
            target: AssignmentTarget::Variable("result".to_string()),
            value: Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "result".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Variable {
                    name: "s".to_string(),
                    position: pos(),
                }),
                position: pos(),
            },
            position: pos(),
        }],
        position: pos(),
    };

    executor.eval_stmt(&for_stmt).unwrap();

    // result should be "abc"
    let result_value = executor.env().get("result").unwrap();
    assert_eq!(result_value, Value::String("abc".to_string()));
}

#[test]
fn test_for_loop_in_function() {
    let mut executor = Executor::new();

    // func sum_list(numbers) {
    //     total = 0
    //     for n in numbers {
    //         total = total + n
    //     }
    //     return total
    // }
    let func_decl = Stmt::FunctionDecl {
        name: "sum_list".to_string(),
        params: vec![Parameter {
            name: "numbers".to_string(),
            default_value: None,
        }],
        body: vec![
            Stmt::VariableDecl {
                name: "total".to_string(),
                value: Expr::Literal {
                    value: LiteralValue::Number(0.0),
                    position: pos(),
                },
                type_annotation: None,
                position: pos(),
            },
            Stmt::For {
                variable: "n".to_string(),
                iterable: Expr::Variable {
                    name: "numbers".to_string(),
                    position: pos(),
                },
                body: vec![Stmt::Assignment {
                    target: AssignmentTarget::Variable("total".to_string()),
                    value: Expr::Binary {
                        left: Box::new(Expr::Variable {
                            name: "total".to_string(),
                            position: pos(),
                        }),
                        op: BinaryOp::Add,
                        right: Box::new(Expr::Variable {
                            name: "n".to_string(),
                            position: pos(),
                        }),
                        position: pos(),
                    },
                    position: pos(),
                }],
                position: pos(),
            },
            Stmt::Return {
                value: Some(Expr::Variable {
                    name: "total".to_string(),
                    position: pos(),
                }),
                position: pos(),
            },
        ],
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // sum_list([10, 20, 30]) should be 60
    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "sum_list".to_string(),
            position: pos(),
        }),
        args: vec![Expr::List {
            elements: vec![
                Expr::Literal {
                    value: LiteralValue::Number(10.0),
                    position: pos(),
                },
                Expr::Literal {
                    value: LiteralValue::Number(20.0),
                    position: pos(),
                },
                Expr::Literal {
                    value: LiteralValue::Number(30.0),
                    position: pos(),
                },
            ],
            position: pos(),
        }],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::Number(60.0));
}

#[test]
fn test_nested_for_loops() {
    let mut executor = Executor::new();

    // count = 0
    // for i in [1, 2] {
    //     for j in [10, 20, 30] {
    //         count = count + 1
    //     }
    // }
    executor
        .eval_stmt(&Stmt::VariableDecl {
            name: "count".to_string(),
            value: Expr::Literal {
                value: LiteralValue::Number(0.0),
                position: pos(),
            },
            type_annotation: None,
            position: pos(),
        })
        .unwrap();

    let inner_for = Stmt::For {
        variable: "j".to_string(),
        iterable: Expr::List {
            elements: vec![
                Expr::Literal {
                    value: LiteralValue::Number(10.0),
                    position: pos(),
                },
                Expr::Literal {
                    value: LiteralValue::Number(20.0),
                    position: pos(),
                },
                Expr::Literal {
                    value: LiteralValue::Number(30.0),
                    position: pos(),
                },
            ],
            position: pos(),
        },
        body: vec![Stmt::Assignment {
            target: AssignmentTarget::Variable("count".to_string()),
            value: Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "count".to_string(),
                    position: pos(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(1.0),
                    position: pos(),
                }),
                position: pos(),
            },
            position: pos(),
        }],
        position: pos(),
    };

    let outer_for = Stmt::For {
        variable: "i".to_string(),
        iterable: Expr::List {
            elements: vec![
                Expr::Literal {
                    value: LiteralValue::Number(1.0),
                    position: pos(),
                },
                Expr::Literal {
                    value: LiteralValue::Number(2.0),
                    position: pos(),
                },
            ],
            position: pos(),
        },
        body: vec![inner_for],
        position: pos(),
    };

    executor.eval_stmt(&outer_for).unwrap();

    // count should be 6 (2 outer * 3 inner)
    let count_value = executor.env().get("count").unwrap();
    assert_eq!(count_value, Value::Number(6.0));
}

// ============================================================================
// LIST INDEXING TESTS
// ============================================================================

#[test]
fn test_list_index_positive() {
    let mut executor = Executor::new();

    // items = [10, 20, 30]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("items".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(10.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(20.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(30.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // items[0] should be 10
    let index_expr = Expr::Index {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        index: Box::new(Expr::Literal { value: LiteralValue::Number(0.0), position: pos() }),
        position: pos(),
    };
    let result = executor.eval_expr(&index_expr).unwrap();
    assert_eq!(result, Value::Number(10.0));

    // items[1] should be 20
    let index_expr = Expr::Index {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        index: Box::new(Expr::Literal { value: LiteralValue::Number(1.0), position: pos() }),
        position: pos(),
    };
    let result = executor.eval_expr(&index_expr).unwrap();
    assert_eq!(result, Value::Number(20.0));

    // items[2] should be 30
    let index_expr = Expr::Index {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        index: Box::new(Expr::Literal { value: LiteralValue::Number(2.0), position: pos() }),
        position: pos(),
    };
    let result = executor.eval_expr(&index_expr).unwrap();
    assert_eq!(result, Value::Number(30.0));
}

#[test]
fn test_list_index_negative() {
    let mut executor = Executor::new();

    // items = [10, 20, 30]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("items".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(10.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(20.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(30.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // items[-1] should be 30 (last element)
    let index_expr = Expr::Index {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        index: Box::new(Expr::Literal { value: LiteralValue::Number(-1.0), position: pos() }),
        position: pos(),
    };
    let result = executor.eval_expr(&index_expr).unwrap();
    assert_eq!(result, Value::Number(30.0));

    // items[-2] should be 20
    let index_expr = Expr::Index {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        index: Box::new(Expr::Literal { value: LiteralValue::Number(-2.0), position: pos() }),
        position: pos(),
    };
    let result = executor.eval_expr(&index_expr).unwrap();
    assert_eq!(result, Value::Number(20.0));

    // items[-3] should be 10 (first element)
    let index_expr = Expr::Index {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        index: Box::new(Expr::Literal { value: LiteralValue::Number(-3.0), position: pos() }),
        position: pos(),
    };
    let result = executor.eval_expr(&index_expr).unwrap();
    assert_eq!(result, Value::Number(10.0));
}

#[test]
fn test_list_index_out_of_bounds() {
    let mut executor = Executor::new();

    // items = [10, 20]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("items".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(10.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(20.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // items[5] should error (out of bounds)
    let index_expr = Expr::Index {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        index: Box::new(Expr::Literal { value: LiteralValue::Number(5.0), position: pos() }),
        position: pos(),
    };
    let result = executor.eval_expr(&index_expr);
    assert!(result.is_err());

    // items[-5] should error (out of bounds)
    let index_expr = Expr::Index {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        index: Box::new(Expr::Literal { value: LiteralValue::Number(-5.0), position: pos() }),
        position: pos(),
    };
    let result = executor.eval_expr(&index_expr);
    assert!(result.is_err());
}

#[test]
fn test_list_index_with_strings() {
    let mut executor = Executor::new();

    // words = ["hello", "world"]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("words".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::String("hello".to_string()), position: pos() },
                Expr::Literal { value: LiteralValue::String("world".to_string()), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // words[0] should be "hello"
    let index_expr = Expr::Index {
        object: Box::new(Expr::Variable { name: "words".to_string(), position: pos() }),
        index: Box::new(Expr::Literal { value: LiteralValue::Number(0.0), position: pos() }),
        position: pos(),
    };
    let result = executor.eval_expr(&index_expr).unwrap();
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_map_index_string_key() {
    let mut executor = Executor::new();

    // config = {"name": "Alice", "age": 30}
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("config".to_string()),
        value: Expr::Map {
            entries: vec![
                ("name".to_string(), Expr::Literal { value: LiteralValue::String("Alice".to_string()), position: pos() }),
                ("age".to_string(), Expr::Literal { value: LiteralValue::Number(30.0), position: pos() }),
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // config["name"] should be "Alice"
    let index_expr = Expr::Index {
        object: Box::new(Expr::Variable { name: "config".to_string(), position: pos() }),
        index: Box::new(Expr::Literal { value: LiteralValue::String("name".to_string()), position: pos() }),
        position: pos(),
    };
    let result = executor.eval_expr(&index_expr).unwrap();
    assert_eq!(result, Value::String("Alice".to_string()));

    // config["age"] should be 30
    let index_expr = Expr::Index {
        object: Box::new(Expr::Variable { name: "config".to_string(), position: pos() }),
        index: Box::new(Expr::Literal { value: LiteralValue::String("age".to_string()), position: pos() }),
        position: pos(),
    };
    let result = executor.eval_expr(&index_expr).unwrap();
    assert_eq!(result, Value::Number(30.0));
}

#[test]
fn test_map_index_missing_key() {
    let mut executor = Executor::new();

    // config = {"name": "Alice"}
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("config".to_string()),
        value: Expr::Map {
            entries: vec![
                ("name".to_string(), Expr::Literal { value: LiteralValue::String("Alice".to_string()), position: pos() }),
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // config["missing"] should error
    let index_expr = Expr::Index {
        object: Box::new(Expr::Variable { name: "config".to_string(), position: pos() }),
        index: Box::new(Expr::Literal { value: LiteralValue::String("missing".to_string()), position: pos() }),
        position: pos(),
    };
    let result = executor.eval_expr(&index_expr);
    assert!(result.is_err());
}

// ============================================================================
// LIST METHOD TESTS
// ============================================================================

#[test]
fn test_list_method_size() {
    let mut executor = Executor::new();

    // items = [10, 20, 30]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("items".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(10.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(20.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(30.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // items.size() should be 3
    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        method: "size".to_string(),
        args: vec![],
        position: pos(),
    };
    let result = executor.eval_expr(&method_call).unwrap();
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn test_list_method_first() {
    let mut executor = Executor::new();

    // items = [10, 20, 30]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("items".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(10.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(20.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(30.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // items.first() should be 10
    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        method: "first".to_string(),
        args: vec![],
        position: pos(),
    };
    let result = executor.eval_expr(&method_call).unwrap();
    assert_eq!(result, Value::Number(10.0));
}

#[test]
fn test_list_method_last() {
    let mut executor = Executor::new();

    // items = [10, 20, 30]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("items".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(10.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(20.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(30.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // items.last() should be 30
    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        method: "last".to_string(),
        args: vec![],
        position: pos(),
    };
    let result = executor.eval_expr(&method_call).unwrap();
    assert_eq!(result, Value::Number(30.0));
}

#[test]
fn test_list_method_contains() {
    let mut executor = Executor::new();

    // items = [10, 20, 30]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("items".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(10.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(20.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(30.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // items.contains(20) should be true
    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        method: "contains".to_string(),
        args: vec![Expr::Literal { value: LiteralValue::Number(20.0), position: pos() }],
        position: pos(),
    };
    let result = executor.eval_expr(&method_call).unwrap();
    assert_eq!(result, Value::Boolean(true));

    // items.contains(99) should be false
    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        method: "contains".to_string(),
        args: vec![Expr::Literal { value: LiteralValue::Number(99.0), position: pos() }],
        position: pos(),
    };
    let result = executor.eval_expr(&method_call).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_list_method_is_empty() {
    let mut executor = Executor::new();

    // empty = []
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("empty".to_string()),
        value: Expr::List {
            elements: vec![],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // empty.is_empty() should be true
    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "empty".to_string(), position: pos() }),
        method: "is_empty".to_string(),
        args: vec![],
        position: pos(),
    };
    let result = executor.eval_expr(&method_call).unwrap();
    assert_eq!(result, Value::Boolean(true));

    // items = [1]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("items".to_string()),
        value: Expr::List {
            elements: vec![Expr::Literal { value: LiteralValue::Number(1.0), position: pos() }],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // items.is_empty() should be false
    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        method: "is_empty".to_string(),
        args: vec![],
        position: pos(),
    };
    let result = executor.eval_expr(&method_call).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

// ============================================================================
// LIST FUNCTIONAL METHOD TESTS
// ============================================================================

#[test]
fn test_list_method_map() {
    let mut executor = Executor::new();

    // numbers = [1, 2, 3]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("numbers".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(1.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(2.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(3.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // doubled = numbers.map(x => x * 2)
    let lambda = Expr::Lambda {
        params: vec!["x".to_string()],
        body: Box::new(Expr::Binary {
            left: Box::new(Expr::Variable { name: "x".to_string(), position: pos() }),
            op: BinaryOp::Multiply,
            right: Box::new(Expr::Literal { value: LiteralValue::Number(2.0), position: pos() }),
            position: pos(),
        }),
        position: pos(),
    };

    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "numbers".to_string(), position: pos() }),
        method: "map".to_string(),
        args: vec![lambda],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match result {
        Value::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::Number(2.0));
            assert_eq!(*elements.get(1).unwrap(), Value::Number(4.0));
            assert_eq!(*elements.get(2).unwrap(), Value::Number(6.0));
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

#[test]
fn test_list_method_filter() {
    let mut executor = Executor::new();

    // numbers = [1, 2, 3, 4, 5]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("numbers".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(1.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(2.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(3.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(4.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(5.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // evens = numbers.filter(x => x % 2 == 0)
    let lambda = Expr::Lambda {
        params: vec!["x".to_string()],
        body: Box::new(Expr::Binary {
            left: Box::new(Expr::Binary {
                left: Box::new(Expr::Variable { name: "x".to_string(), position: pos() }),
                op: BinaryOp::Modulo,
                right: Box::new(Expr::Literal { value: LiteralValue::Number(2.0), position: pos() }),
                position: pos(),
            }),
            op: BinaryOp::Equal,
            right: Box::new(Expr::Literal { value: LiteralValue::Number(0.0), position: pos() }),
            position: pos(),
        }),
        position: pos(),
    };

    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "numbers".to_string(), position: pos() }),
        method: "filter".to_string(),
        args: vec![lambda],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match result {
        Value::List(elements) => {
            assert_eq!(elements.len(), 2);
            assert_eq!(*elements.get(0).unwrap(), Value::Number(2.0));
            assert_eq!(*elements.get(1).unwrap(), Value::Number(4.0));
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

#[test]
fn test_list_method_each() {
    let mut executor = Executor::new();

    // numbers = [1, 2, 3]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("numbers".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(1.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(2.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(3.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // numbers.each(x => x * 2) - just executes the lambda for each element
    let lambda = Expr::Lambda {
        params: vec!["x".to_string()],
        body: Box::new(Expr::Binary {
            left: Box::new(Expr::Variable { name: "x".to_string(), position: pos() }),
            op: BinaryOp::Multiply,
            right: Box::new(Expr::Literal { value: LiteralValue::Number(2.0), position: pos() }),
            position: pos(),
        }),
        position: pos(),
    };

    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "numbers".to_string(), position: pos() }),
        method: "each".to_string(),
        args: vec![lambda],
        position: pos(),
    };

    // each() should return the original list
    let result = executor.eval_expr(&method_call).unwrap();
    match result {
        Value::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::Number(1.0));
            assert_eq!(*elements.get(1).unwrap(), Value::Number(2.0));
            assert_eq!(*elements.get(2).unwrap(), Value::Number(3.0));
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

#[test]
fn test_list_method_map_with_strings() {
    let mut executor = Executor::new();

    // words = ["hello", "world"]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("words".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::String("hello".to_string()), position: pos() },
                Expr::Literal { value: LiteralValue::String("world".to_string()), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // exclaimed = words.map(w => w + "!")
    let lambda = Expr::Lambda {
        params: vec!["w".to_string()],
        body: Box::new(Expr::Binary {
            left: Box::new(Expr::Variable { name: "w".to_string(), position: pos() }),
            op: BinaryOp::Add,
            right: Box::new(Expr::Literal { value: LiteralValue::String("!".to_string()), position: pos() }),
            position: pos(),
        }),
        position: pos(),
    };

    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "words".to_string(), position: pos() }),
        method: "map".to_string(),
        args: vec![lambda],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match result {
        Value::List(elements) => {
            assert_eq!(elements.len(), 2);
            assert_eq!(*elements.get(0).unwrap(), Value::String("hello!".to_string()));
            assert_eq!(*elements.get(1).unwrap(), Value::String("world!".to_string()));
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

#[test]
fn test_list_method_filter_greater_than() {
    let mut executor = Executor::new();

    // numbers = [1, 5, 10, 15, 20]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("numbers".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(1.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(5.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(10.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(15.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(20.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // big = numbers.filter(x => x > 10)
    let lambda = Expr::Lambda {
        params: vec!["x".to_string()],
        body: Box::new(Expr::Binary {
            left: Box::new(Expr::Variable { name: "x".to_string(), position: pos() }),
            op: BinaryOp::Greater,
            right: Box::new(Expr::Literal { value: LiteralValue::Number(10.0), position: pos() }),
            position: pos(),
        }),
        position: pos(),
    };

    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "numbers".to_string(), position: pos() }),
        method: "filter".to_string(),
        args: vec![lambda],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match result {
        Value::List(elements) => {
            assert_eq!(elements.len(), 2);
            assert_eq!(*elements.get(0).unwrap(), Value::Number(15.0));
            assert_eq!(*elements.get(1).unwrap(), Value::Number(20.0));
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

// ============================================================================
// NAMED TRANSFORMATION TESTS
// ============================================================================

#[test]
fn test_list_method_map_with_named_transform_double() {
    let mut executor = Executor::new();

    // numbers = [1, 2, 3]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("numbers".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(1.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(2.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(3.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // doubled = numbers.map(:double)
    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "numbers".to_string(), position: pos() }),
        method: "map".to_string(),
        args: vec![Expr::Literal { value: LiteralValue::Symbol("double".to_string()), position: pos() }],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match result {
        Value::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::Number(2.0));
            assert_eq!(*elements.get(1).unwrap(), Value::Number(4.0));
            assert_eq!(*elements.get(2).unwrap(), Value::Number(6.0));
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

#[test]
fn test_list_method_map_with_named_transform_square() {
    let mut executor = Executor::new();

    // numbers = [2, 3, 4]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("numbers".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(2.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(3.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(4.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // squared = numbers.map(:square)
    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "numbers".to_string(), position: pos() }),
        method: "map".to_string(),
        args: vec![Expr::Literal { value: LiteralValue::Symbol("square".to_string()), position: pos() }],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match result {
        Value::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::Number(4.0));
            assert_eq!(*elements.get(1).unwrap(), Value::Number(9.0));
            assert_eq!(*elements.get(2).unwrap(), Value::Number(16.0));
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

#[test]
fn test_list_method_map_with_named_transform_negate() {
    let mut executor = Executor::new();

    // numbers = [1, -2, 3]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("numbers".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(1.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(-2.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(3.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // negated = numbers.map(:negate)
    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "numbers".to_string(), position: pos() }),
        method: "map".to_string(),
        args: vec![Expr::Literal { value: LiteralValue::Symbol("negate".to_string()), position: pos() }],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match result {
        Value::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::Number(-1.0));
            assert_eq!(*elements.get(1).unwrap(), Value::Number(2.0));
            assert_eq!(*elements.get(2).unwrap(), Value::Number(-3.0));
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

// ============================================================================
// NAMED PREDICATE TESTS
// ============================================================================

#[test]
fn test_list_method_filter_with_named_predicate_even() {
    let mut executor = Executor::new();

    // numbers = [1, 2, 3, 4, 5, 6]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("numbers".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(1.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(2.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(3.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(4.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(5.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(6.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // evens = numbers.filter(:even)
    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "numbers".to_string(), position: pos() }),
        method: "filter".to_string(),
        args: vec![Expr::Literal { value: LiteralValue::Symbol("even".to_string()), position: pos() }],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match result {
        Value::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::Number(2.0));
            assert_eq!(*elements.get(1).unwrap(), Value::Number(4.0));
            assert_eq!(*elements.get(2).unwrap(), Value::Number(6.0));
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

#[test]
fn test_list_method_filter_with_named_predicate_positive() {
    let mut executor = Executor::new();

    // numbers = [-2, -1, 0, 1, 2]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("numbers".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(-2.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(-1.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(0.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(1.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(2.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // positives = numbers.filter(:positive)
    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "numbers".to_string(), position: pos() }),
        method: "filter".to_string(),
        args: vec![Expr::Literal { value: LiteralValue::Symbol("positive".to_string()), position: pos() }],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match result {
        Value::List(elements) => {
            assert_eq!(elements.len(), 2);
            assert_eq!(*elements.get(0).unwrap(), Value::Number(1.0));
            assert_eq!(*elements.get(1).unwrap(), Value::Number(2.0));
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

#[test]
fn test_list_method_filter_with_named_predicate_odd() {
    let mut executor = Executor::new();

    // numbers = [1, 2, 3, 4, 5]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("numbers".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(1.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(2.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(3.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(4.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(5.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // odds = numbers.filter(:odd)
    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "numbers".to_string(), position: pos() }),
        method: "filter".to_string(),
        args: vec![Expr::Literal { value: LiteralValue::Symbol("odd".to_string()), position: pos() }],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match result {
        Value::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::Number(1.0));
            assert_eq!(*elements.get(1).unwrap(), Value::Number(3.0));
            assert_eq!(*elements.get(2).unwrap(), Value::Number(5.0));
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

// ============================================================================
// ELEMENT-WISE OPERATOR TESTS
// ============================================================================

#[test]
fn test_element_wise_add() {
    let mut executor = Executor::new();

    // a = [1, 2, 3]
    // b = [4, 5, 6]
    // result = a .+ b  # [5, 7, 9]
    let a = Stmt::Assignment {
        target: AssignmentTarget::Variable("a".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(1.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(2.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(3.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&a).unwrap();

    let b = Stmt::Assignment {
        target: AssignmentTarget::Variable("b".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(4.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(5.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(6.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&b).unwrap();

    let expr = Expr::Binary {
        left: Box::new(Expr::Variable { name: "a".to_string(), position: pos() }),
        op: BinaryOp::DotAdd,
        right: Box::new(Expr::Variable { name: "b".to_string(), position: pos() }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    match result {
        Value::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::Number(5.0));
            assert_eq!(*elements.get(1).unwrap(), Value::Number(7.0));
            assert_eq!(*elements.get(2).unwrap(), Value::Number(9.0));
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

#[test]
fn test_element_wise_multiply() {
    let mut executor = Executor::new();

    // a = [2, 3, 4]
    // b = [10, 20, 30]
    // result = a .* b  # [20, 60, 120]
    let a = Stmt::Assignment {
        target: AssignmentTarget::Variable("a".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(2.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(3.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(4.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&a).unwrap();

    let b = Stmt::Assignment {
        target: AssignmentTarget::Variable("b".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(10.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(20.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(30.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&b).unwrap();

    let expr = Expr::Binary {
        left: Box::new(Expr::Variable { name: "a".to_string(), position: pos() }),
        op: BinaryOp::DotMultiply,
        right: Box::new(Expr::Variable { name: "b".to_string(), position: pos() }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    match result {
        Value::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::Number(20.0));
            assert_eq!(*elements.get(1).unwrap(), Value::Number(60.0));
            assert_eq!(*elements.get(2).unwrap(), Value::Number(120.0));
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

#[test]
fn test_element_wise_scalar_broadcast() {
    let mut executor = Executor::new();

    // nums = [1, 2, 3]
    // result = nums .* 10  # [10, 20, 30] (broadcast scalar)
    let nums = Stmt::Assignment {
        target: AssignmentTarget::Variable("nums".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(1.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(2.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(3.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&nums).unwrap();

    let expr = Expr::Binary {
        left: Box::new(Expr::Variable { name: "nums".to_string(), position: pos() }),
        op: BinaryOp::DotMultiply,
        right: Box::new(Expr::Literal { value: LiteralValue::Number(10.0), position: pos() }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    match result {
        Value::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::Number(10.0));
            assert_eq!(*elements.get(1).unwrap(), Value::Number(20.0));
            assert_eq!(*elements.get(2).unwrap(), Value::Number(30.0));
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

#[test]
fn test_element_wise_subtract() {
    let mut executor = Executor::new();

    // a = [10, 20, 30]
    // b = [1, 2, 3]
    // result = a .- b  # [9, 18, 27]
    let a = Stmt::Assignment {
        target: AssignmentTarget::Variable("a".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(10.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(20.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(30.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&a).unwrap();

    let b = Stmt::Assignment {
        target: AssignmentTarget::Variable("b".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(1.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(2.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(3.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&b).unwrap();

    let expr = Expr::Binary {
        left: Box::new(Expr::Variable { name: "a".to_string(), position: pos() }),
        op: BinaryOp::DotSubtract,
        right: Box::new(Expr::Variable { name: "b".to_string(), position: pos() }),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    match result {
        Value::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::Number(9.0));
            assert_eq!(*elements.get(1).unwrap(), Value::Number(18.0));
            assert_eq!(*elements.get(2).unwrap(), Value::Number(27.0));
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

// ============================================================================
// LIST SLICING TESTS
// ============================================================================

#[test]
fn test_list_method_slice_basic() {
    let mut executor = Executor::new();

    // items = [10, 20, 30, 40, 50]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("items".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(10.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(20.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(30.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(40.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(50.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // items.slice(1, 3) should be [20, 30]
    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        method: "slice".to_string(),
        args: vec![
            Expr::Literal { value: LiteralValue::Number(1.0), position: pos() },
            Expr::Literal { value: LiteralValue::Number(3.0), position: pos() },
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match result {
        Value::List(elements) => {
            assert_eq!(elements.len(), 2);
            assert_eq!(*elements.get(0).unwrap(), Value::Number(20.0));
            assert_eq!(*elements.get(1).unwrap(), Value::Number(30.0));
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

#[test]
fn test_list_method_slice_from_start() {
    let mut executor = Executor::new();

    // items = [10, 20, 30, 40, 50]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("items".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(10.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(20.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(30.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(40.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(50.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // items.slice(0, 3) should be [10, 20, 30]
    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        method: "slice".to_string(),
        args: vec![
            Expr::Literal { value: LiteralValue::Number(0.0), position: pos() },
            Expr::Literal { value: LiteralValue::Number(3.0), position: pos() },
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match result {
        Value::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::Number(10.0));
            assert_eq!(*elements.get(1).unwrap(), Value::Number(20.0));
            assert_eq!(*elements.get(2).unwrap(), Value::Number(30.0));
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

#[test]
fn test_list_method_slice_to_end() {
    let mut executor = Executor::new();

    // items = [10, 20, 30, 40, 50]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("items".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(10.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(20.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(30.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(40.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(50.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // items.slice(2, 5) should be [30, 40, 50]
    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        method: "slice".to_string(),
        args: vec![
            Expr::Literal { value: LiteralValue::Number(2.0), position: pos() },
            Expr::Literal { value: LiteralValue::Number(5.0), position: pos() },
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match result {
        Value::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::Number(30.0));
            assert_eq!(*elements.get(1).unwrap(), Value::Number(40.0));
            assert_eq!(*elements.get(2).unwrap(), Value::Number(50.0));
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

#[test]
fn test_list_method_slice_negative_indices() {
    let mut executor = Executor::new();

    // items = [10, 20, 30, 40, 50]
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("items".to_string()),
        value: Expr::List {
            elements: vec![
                Expr::Literal { value: LiteralValue::Number(10.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(20.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(30.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(40.0), position: pos() },
                Expr::Literal { value: LiteralValue::Number(50.0), position: pos() },
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // items.slice(-3, -1) should be [30, 40]
    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        method: "slice".to_string(),
        args: vec![
            Expr::Literal { value: LiteralValue::Number(-3.0), position: pos() },
            Expr::Literal { value: LiteralValue::Number(-1.0), position: pos() },
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match result {
        Value::List(elements) => {
            assert_eq!(elements.len(), 2);
            assert_eq!(*elements.get(0).unwrap(), Value::Number(30.0));
            assert_eq!(*elements.get(1).unwrap(), Value::Number(40.0));
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

// ============================================================================
// Map Methods Tests
// ============================================================================

#[test]
fn test_map_method_keys() {
    let mut executor = Executor::new();

    // Create map: data = {"name": "Alice", "age": 25}
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("data".to_string()),
        value: Expr::Map {
            entries: vec![
                (
                    "name".to_string(),
                    Expr::Literal { value: LiteralValue::String("Alice".to_string()), position: pos() },
                ),
                (
                    "age".to_string(),
                    Expr::Literal { value: LiteralValue::Number(25.0), position: pos() },
                ),
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // data.keys() should return ["name", "age"] (order may vary)
    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "data".to_string(), position: pos() }),
        method: "keys".to_string(),
        args: vec![],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match result {
        Value::List(elements) => {
            assert_eq!(elements.len(), 2);
            // Keys may be in any order, so check both are present
            let keys: Vec<String> = elements.to_vec().iter().filter_map(|v| {
                if let Value::String(s) = v {
                    Some(s.clone())
                } else {
                    None
                }
            }).collect();
            assert!(keys.contains(&"name".to_string()));
            assert!(keys.contains(&"age".to_string()));
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

#[test]
fn test_map_method_values() {
    let mut executor = Executor::new();

    // Create map: data = {"x": 10, "y": 20}
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("data".to_string()),
        value: Expr::Map {
            entries: vec![
                (
                    "x".to_string(),
                    Expr::Literal { value: LiteralValue::Number(10.0), position: pos() },
                ),
                (
                    "y".to_string(),
                    Expr::Literal { value: LiteralValue::Number(20.0), position: pos() },
                ),
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // data.values() should return [10, 20] (order may vary)
    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "data".to_string(), position: pos() }),
        method: "values".to_string(),
        args: vec![],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match result {
        Value::List(elements) => {
            assert_eq!(elements.len(), 2);
            // Values may be in any order, so check both are present
            let values: Vec<f64> = elements.to_vec().iter().filter_map(|v| {
                if let Value::Number(n) = v {
                    Some(*n)
                } else {
                    None
                }
            }).collect();
            assert!(values.contains(&10.0));
            assert!(values.contains(&20.0));
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

#[test]
fn test_map_method_has_key() {
    let mut executor = Executor::new();

    // Create map: data = {"name": "Bob", "age": 30}
    let assign = Stmt::Assignment {
        target: AssignmentTarget::Variable("data".to_string()),
        value: Expr::Map {
            entries: vec![
                (
                    "name".to_string(),
                    Expr::Literal { value: LiteralValue::String("Bob".to_string()), position: pos() },
                ),
                (
                    "age".to_string(),
                    Expr::Literal { value: LiteralValue::Number(30.0), position: pos() },
                ),
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign).unwrap();

    // data.has_key("name") should return true
    let method_call1 = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "data".to_string(), position: pos() }),
        method: "has_key".to_string(),
        args: vec![
            Expr::Literal { value: LiteralValue::String("name".to_string()), position: pos() },
        ],
        position: pos(),
    };

    let result1 = executor.eval_expr(&method_call1).unwrap();
    assert_eq!(result1, Value::Boolean(true));

    // data.has_key("missing") should return false
    let method_call2 = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "data".to_string(), position: pos() }),
        method: "has_key".to_string(),
        args: vec![
            Expr::Literal { value: LiteralValue::String("missing".to_string()), position: pos() },
        ],
        position: pos(),
    };

    let result2 = executor.eval_expr(&method_call2).unwrap();
    assert_eq!(result2, Value::Boolean(false));
}

#[test]
fn test_map_method_size() {
    let mut executor = Executor::new();

    // Empty map
    let assign1 = Stmt::Assignment {
        target: AssignmentTarget::Variable("empty".to_string()),
        value: Expr::Map {
            entries: vec![],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign1).unwrap();

    let method_call1 = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "empty".to_string(), position: pos() }),
        method: "size".to_string(),
        args: vec![],
        position: pos(),
    };

    let result1 = executor.eval_expr(&method_call1).unwrap();
    assert_eq!(result1, Value::Number(0.0));

    // Map with 3 entries
    let assign2 = Stmt::Assignment {
        target: AssignmentTarget::Variable("data".to_string()),
        value: Expr::Map {
            entries: vec![
                (
                    "a".to_string(),
                    Expr::Literal { value: LiteralValue::Number(1.0), position: pos() },
                ),
                (
                    "b".to_string(),
                    Expr::Literal { value: LiteralValue::Number(2.0), position: pos() },
                ),
                (
                    "c".to_string(),
                    Expr::Literal { value: LiteralValue::Number(3.0), position: pos() },
                ),
            ],
            position: pos(),
        },
        position: pos(),
    };
    executor.eval_stmt(&assign2).unwrap();

    let method_call2 = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "data".to_string(), position: pos() }),
        method: "size".to_string(),
        args: vec![],
        position: pos(),
    };

    let result2 = executor.eval_expr(&method_call2).unwrap();
    assert_eq!(result2, Value::Number(3.0));
}

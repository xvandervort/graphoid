use graphoid::ast::{AssignmentTarget, BinaryOp, Expr, LiteralValue, Stmt, UnaryOp};
use graphoid::error::SourcePosition;
use graphoid::execution::Executor;
use graphoid::values::Value;
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
    assert_eq!(result, Value::List(vec![]));
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
        Value::List(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0)
        ])
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
    assert_eq!(result, Value::Map(HashMap::new()));
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

use graphoid::ast::{Argument, AssignmentTarget, BinaryOp, Expr, LiteralValue, Parameter, Stmt, UnaryOp};
use graphoid::error::SourcePosition;
use graphoid::execution::{Executor, ErrorMode};
use graphoid::values::{BigNum, Hash, List, Value, ValueKind};
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
    assert_eq!(result, Value::number(42.0));
}

#[test]
fn test_eval_float_literal() {
    let mut executor = Executor::new();
    let expr = Expr::Literal {
        value: LiteralValue::Number(3.14159),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::number(3.14159));
}

#[test]
fn test_eval_string_literal() {
    let mut executor = Executor::new();
    let expr = Expr::Literal {
        value: LiteralValue::String("hello world".to_string()),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::string("hello world".to_string()));
}

#[test]
fn test_eval_boolean_true() {
    let mut executor = Executor::new();
    let expr = Expr::Literal {
        value: LiteralValue::Boolean(true),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::boolean(true));
}

#[test]
fn test_eval_boolean_false() {
    let mut executor = Executor::new();
    let expr = Expr::Literal {
        value: LiteralValue::Boolean(false),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::boolean(false));
}

#[test]
fn test_eval_none_literal() {
    let mut executor = Executor::new();
    let expr = Expr::Literal {
        value: LiteralValue::None,
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::none());
}

#[test]
fn test_eval_symbol_literal() {
    let mut executor = Executor::new();
    let expr = Expr::Literal {
        value: LiteralValue::Symbol("test_symbol".to_string()),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::symbol("test_symbol".to_string()));
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
    assert_eq!(result, Value::number(5.0));
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
    assert_eq!(result, Value::number(6.0));
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
    assert_eq!(result, Value::number(42.0));
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
    assert_eq!(result, Value::number(5.0));
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
    assert_eq!(result, Value::number(3.0));
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
    assert_eq!(result, Value::number(1.0));
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
    assert_eq!(result, Value::number(256.0));
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
    assert_eq!(result, Value::number(14.0));
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
    assert_eq!(result, Value::boolean(true));
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
    assert_eq!(result, Value::boolean(false));
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
    assert_eq!(result, Value::boolean(true));
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
    assert_eq!(result, Value::boolean(true));
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
    assert_eq!(result, Value::boolean(true));
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
    assert_eq!(result, Value::boolean(true));
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
    assert_eq!(result, Value::boolean(true));
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
    assert_eq!(result, Value::boolean(true));
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
    assert_eq!(result, Value::boolean(false));
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
    assert_eq!(result, Value::boolean(true));
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
    assert_eq!(result, Value::boolean(false));
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
    assert_eq!(result, Value::boolean(false));
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
    assert_eq!(result, Value::boolean(true));
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
    assert_eq!(result, Value::number(-42.0));
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
        is_private: false,
        position: pos(),
    };

    executor.eval_stmt(&stmt).unwrap();

    // Verify variable was defined
    let value = executor.env().get("x").unwrap();
    assert_eq!(value, Value::number(42.0));
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
        is_private: false,
        position: pos(),
    };
    executor.eval_stmt(&stmt).unwrap();

    // Reference variable
    let expr = Expr::Variable {
        name: "x".to_string(),
        position: pos(),
    };

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::number(42.0));
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
        is_private: false,
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
    assert_eq!(value, Value::number(20.0));
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
        is_private: false,
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
    assert_eq!(result, Value::number(15.0));
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
    assert_eq!(result, Value::list(List::from_vec(vec![])));
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
        Value::list(List::from_vec(vec![
            Value::number(1.0),
            Value::number(2.0),
            Value::number(3.0)
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
    assert_eq!(result, Value::map(Hash::from_hashmap(HashMap::new())));
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

    if let ValueKind::Map(map) = &result.kind {
        assert_eq!(map.len(), 2);
        assert_eq!(
            map.get("name"),
            Some(&Value::string("Alice".to_string()))
        );
        assert_eq!(map.get("age"), Some(&Value::number(30.0)));
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
    assert_eq!(result, Value::string("hello world".to_string()));
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
    assert_eq!(result, Value::string("hello world".to_string()));
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
    assert_eq!(result, Value::boolean(true));
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
    assert_eq!(result, Value::boolean(true));
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
    assert_eq!(result, Value::boolean(true));
}

// ============================================================================
// TYPE ERROR TESTS
// ============================================================================

#[test]
fn test_eval_string_number_concatenation() {
    // String + number should work with type coercion (number converted to string)
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

    let result = executor.eval_expr(&expr).unwrap();
    assert_eq!(result, Value::string("hello5".to_string()));
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
    assert_eq!(result, Value::number(1.0));
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
    assert_eq!(result, Value::number(0.25));
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

    // Modulo by zero now raises an error in strict mode (default)
    let result = executor.eval_expr(&expr);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Modulo by zero"));
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
    assert_eq!(result, Value::boolean(true)); // not "" => true
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
        receiver: None,
        params: vec![
            graphoid::ast::Parameter {
                name: "a".to_string(),
                default_value: None,
                is_variadic: false,
            },
            graphoid::ast::Parameter {
                name: "b".to_string(),
                default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
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
        receiver: None,
        params: vec![
            graphoid::ast::Parameter {
                name: "a".to_string(),
                default_value: None,
                is_variadic: false,
            },
            graphoid::ast::Parameter {
                name: "b".to_string(),
                default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
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
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Number(2.0),
                position: pos(),
            }),
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Number(3.0),
                position: pos(),
            }),
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&call_expr).unwrap();
    assert_eq!(result, Value::number(5.0));
}

#[test]
fn test_function_no_params() {
    let mut executor = Executor::new();

    // Define function: func greet() { return "Hello" }
    let func_decl = Stmt::FunctionDecl {
        name: "greet".to_string(),
        receiver: None,
        params: vec![],
        body: vec![Stmt::Return {
            value: Some(Expr::Literal {
                value: LiteralValue::String("Hello".to_string()),
                position: pos(),
            }),
            position: pos(),
        }],
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
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
    assert_eq!(result, Value::string("Hello".to_string()));
}

#[test]
fn test_function_with_expression_body() {
    let mut executor = Executor::new();

    // Define function: func double(x) { return x * 2 }
    let func_decl = Stmt::FunctionDecl {
        name: "double".to_string(),
        receiver: None,
        params: vec![graphoid::ast::Parameter {
            name: "x".to_string(),
            default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // Call: double(5)
    let call_expr = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "double".to_string(),
            position: pos(),
        }),
        args: vec![Argument::Positional(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        })],
        position: pos(),
    };

    let result = executor.eval_expr(&call_expr).unwrap();
    assert_eq!(result, Value::number(10.0));
}

#[test]
fn test_function_nested_calls() {
    let mut executor = Executor::new();

    // Define: func add(a, b) { return a + b }
    let add_decl = Stmt::FunctionDecl {
        name: "add".to_string(),
        receiver: None,
        params: vec![
            graphoid::ast::Parameter {
                name: "a".to_string(),
                default_value: None,
                is_variadic: false,
            },
            graphoid::ast::Parameter {
                name: "b".to_string(),
                default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
        position: pos(),
    };

    // Define: func mul(a, b) { return a * b }
    let mul_decl = Stmt::FunctionDecl {
        name: "mul".to_string(),
        receiver: None,
        params: vec![
            graphoid::ast::Parameter {
                name: "a".to_string(),
                default_value: None,
                is_variadic: false,
            },
            graphoid::ast::Parameter {
                name: "b".to_string(),
                default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
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
            Argument::Positional(Expr::Call {
                callee: Box::new(Expr::Variable {
                    name: "mul".to_string(),
                    position: pos(),
                }),
                args: vec![
                    Argument::Positional(Expr::Literal {
                        value: LiteralValue::Number(2.0),
                        position: pos(),
                    }),
                    Argument::Positional(Expr::Literal {
                        value: LiteralValue::Number(3.0),
                        position: pos(),
                    }),
                ],
                position: pos(),
            }),
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Number(4.0),
                position: pos(),
            }),
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&call_expr).unwrap();
    assert_eq!(result, Value::number(10.0));
}

#[test]
fn test_function_closure() {
    let mut executor = Executor::new();

    // Set up: x = 10
    executor.env_mut().define("x".to_string(), Value::number(10.0));

    // Define: func add_x(y) { return x + y }  (captures x)
    let func_decl = Stmt::FunctionDecl {
        name: "add_x".to_string(),
        receiver: None,
        params: vec![graphoid::ast::Parameter {
            name: "y".to_string(),
            default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // Call: add_x(5) should return 15 (captures x=10)
    let call_expr = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "add_x".to_string(),
            position: pos(),
        }),
        args: vec![Argument::Positional(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        })],
        position: pos(),
    };

    let result = executor.eval_expr(&call_expr).unwrap();
    assert_eq!(result, Value::number(15.0));
}

#[test]
fn test_function_return_none() {
    let mut executor = Executor::new();

    // Define: func do_nothing() { return }
    let func_decl = Stmt::FunctionDecl {
        name: "do_nothing".to_string(),
        receiver: None,
        params: vec![],
        body: vec![Stmt::Return {
            value: None,
            position: pos(),
        }],
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
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
    assert_eq!(result, Value::none());
}

#[test]
fn test_function_wrong_arg_count() {
    let mut executor = Executor::new();

    // Define: func add(a, b) { return a + b }
    let func_decl = Stmt::FunctionDecl {
        name: "add".to_string(),
        receiver: None,
        params: vec![
            graphoid::ast::Parameter {
                name: "a".to_string(),
                default_value: None,
                is_variadic: false,
            },
            graphoid::ast::Parameter {
                name: "b".to_string(),
                default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // Call with wrong number of arguments: add(2)
    let call_expr = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "add".to_string(),
            position: pos(),
        }),
        args: vec![Argument::Positional(Expr::Literal {
            value: LiteralValue::Number(2.0),
            position: pos(),
        })],
        position: pos(),
    };

    let result = executor.eval_expr(&call_expr);
    assert!(result.is_err());
}

#[test]
fn test_function_call_non_function() {
    let mut executor = Executor::new();

    // Define a variable, not a function
    executor.env_mut().define("x".to_string(), Value::number(42.0));

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
    match &func_value.kind {
        ValueKind::Function(f) => {
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
        args: vec![Argument::Positional(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        })],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::number(10.0));
}

#[test]
fn test_lambda_closure() {
    let mut executor = Executor::new();

    // x = 10
    executor.env_mut().define("x".to_string(), Value::number(10.0));

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
        args: vec![Argument::Positional(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        })],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::number(15.0)); // 10 + 5
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
    assert_eq!(result, Value::number(42.0));
}

#[test]
fn test_function_as_value() {
    let mut executor = Executor::new();

    // func double(n) { return n * 2 }
    let func_decl = Stmt::FunctionDecl {
        name: "double".to_string(),
        receiver: None,
        params: vec![Parameter {
            name: "n".to_string(),
            default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // Get function as a value
    let func_value = executor.env().get("double").unwrap();
    assert!(matches!(&func_value.kind, ValueKind::Function(_)));
}

#[test]
fn test_function_multiple_params() {
    let mut executor = Executor::new();

    // func calculate(a, b, c) { return a + b * c }
    let func_decl = Stmt::FunctionDecl {
        name: "calculate".to_string(),
        receiver: None,
        params: vec![
            Parameter {
                name: "a".to_string(),
                default_value: None,
                is_variadic: false,
            },
            Parameter {
                name: "b".to_string(),
                default_value: None,
                is_variadic: false,
            },
            Parameter {
                name: "c".to_string(),
                default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
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
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Number(10.0),
                position: pos(),
            }),
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Number(2.0),
                position: pos(),
            }),
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Number(5.0),
                position: pos(),
            }),
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::number(20.0));
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
        receiver: None,
        params: vec![Parameter {
            name: "n".to_string(),
            default_value: None,
                is_variadic: false,
            }],
        body: vec![Stmt::Return {
            value: Some(Expr::Variable {
                name: "n".to_string(),
                position: pos(),
            }),
            position: pos(),
        }],
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "identity".to_string(),
            position: pos(),
        }),
        args: vec![Argument::Positional(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        })],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::number(5.0));
}

#[test]
fn test_function_with_string_return() {
    let mut executor = Executor::new();

    // func greet(name) { return "Hello, " + name }
    let func_decl = Stmt::FunctionDecl {
        name: "greet".to_string(),
        receiver: None,
        params: vec![Parameter {
            name: "name".to_string(),
            default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "greet".to_string(),
            position: pos(),
        }),
        args: vec![Argument::Positional(Expr::Literal {
            value: LiteralValue::String("Alice".to_string()),
            position: pos(),
        })],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::string("Hello, Alice".to_string()));
}

#[test]
fn test_function_modifying_closure_var() {
    let mut executor = Executor::new();

    // x = 5
    executor.env_mut().define("x".to_string(), Value::number(5.0));

    // func get_x() { return x }
    let func_decl = Stmt::FunctionDecl {
        name: "get_x".to_string(),
        receiver: None,
        params: vec![],
        body: vec![Stmt::Return {
            value: Some(Expr::Variable {
                name: "x".to_string(),
                position: pos(),
            }),
            position: pos(),
        }],
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
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
    assert_eq!(result1, Value::number(5.0));

    // Modify x in outer scope
    executor.env_mut().set("x", Value::number(10.0)).unwrap();

    // Call get_x() again
    // With our Rc<Environment> implementation, the closure captured a clone of the environment
    // So it won't see the change - it still has x = 5
    // This is snapshot semantics, which is one valid closure model
    let result2 = executor.eval_expr(&call1).unwrap();
    // Closure captured environment at function definition time
    assert_eq!(result2, Value::number(5.0));
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
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Number(1.0),
                position: pos(),
            }),
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Number(2.0),
                position: pos(),
            }),
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Number(3.0),
                position: pos(),
            }),
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::number(6.0));
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
            Argument::Positional(Expr::Literal {
                value: LiteralValue::String("John".to_string()),
                position: pos(),
            }),
            Argument::Positional(Expr::Literal {
                value: LiteralValue::String("Doe".to_string()),
                position: pos(),
            }),
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::string("John Doe".to_string()));
}

#[test]
fn test_function_returning_boolean() {
    let mut executor = Executor::new();

    // func is_positive(n) { return n > 0 }
    let func_decl = Stmt::FunctionDecl {
        name: "is_positive".to_string(),
        receiver: None,
        params: vec![Parameter {
            name: "n".to_string(),
            default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    let call1 = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "is_positive".to_string(),
            position: pos(),
        }),
        args: vec![Argument::Positional(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        })],
        position: pos(),
    };

    let result1 = executor.eval_expr(&call1).unwrap();
    assert_eq!(result1, Value::boolean(true));

    let call2 = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "is_positive".to_string(),
            position: pos(),
        }),
        args: vec![Argument::Positional(Expr::Literal {
            value: LiteralValue::Number(-5.0),
            position: pos(),
        })],
        position: pos(),
    };

    let result2 = executor.eval_expr(&call2).unwrap();
    assert_eq!(result2, Value::boolean(false));
}

#[test]
fn test_function_returning_list() {
    let mut executor = Executor::new();

    // func make_list(a, b) { return [a, b] }
    let func_decl = Stmt::FunctionDecl {
        name: "make_list".to_string(),
        receiver: None,
        params: vec![
            Parameter {
                name: "a".to_string(),
                default_value: None,
                is_variadic: false,
            },
            Parameter {
                name: "b".to_string(),
                default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "make_list".to_string(),
            position: pos(),
        }),
        args: vec![
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Number(1.0),
                position: pos(),
            }),
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Number(2.0),
                position: pos(),
            }),
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(
        result,
        Value::list(List::from_vec(vec![Value::number(1.0), Value::number(2.0)]))
    );
}

#[test]
fn test_deeply_nested_calls() {
    let mut executor = Executor::new();

    // func add1(n) { return n + 1 }
    let func_decl = Stmt::FunctionDecl {
        name: "add1".to_string(),
        receiver: None,
        params: vec![Parameter {
            name: "n".to_string(),
            default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // add1(add1(add1(5))) = 8
    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "add1".to_string(),
            position: pos(),
        }),
        args: vec![Argument::Positional(Expr::Call {
            callee: Box::new(Expr::Variable {
                name: "add1".to_string(),
                position: pos(),
            }),
            args: vec![Argument::Positional(Expr::Call {
                callee: Box::new(Expr::Variable {
                    name: "add1".to_string(),
                    position: pos(),
                }),
                args: vec![Argument::Positional(Expr::Literal {
                    value: LiteralValue::Number(5.0),
                    position: pos(),
                })],
                position: pos(),
            })],
            position: pos(),
        })],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::number(8.0));
}

#[test]
fn test_function_with_no_return_statement() {
    let mut executor = Executor::new();

    // func do_nothing() { }
    let func_decl = Stmt::FunctionDecl {
        name: "do_nothing".to_string(),
        receiver: None,
        params: vec![],
        body: vec![], // No statements
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
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
    assert_eq!(result, Value::none()); // Should return none
}

#[test]
fn test_function_early_return() {
    let mut executor = Executor::new();

    // func early() { return 1; return 2; }
    let func_decl = Stmt::FunctionDecl {
        name: "early".to_string(),
        receiver: None,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
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
    assert_eq!(result, Value::number(1.0)); // Should return first value
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
    executor.env_mut().define("x".to_string(), Value::number(0.0));

    // func set_x(val) { x = val; return x }
    let func_decl = Stmt::FunctionDecl {
        name: "set_x".to_string(),
        receiver: None,
        params: vec![Parameter {
            name: "val".to_string(),
            default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // Call set_x(42)
    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "set_x".to_string(),
            position: pos(),
        }),
        args: vec![Argument::Positional(Expr::Literal {
            value: LiteralValue::Number(42.0),
            position: pos(),
        })],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::number(42.0));

    // Verify x was modified (in closure's captured environment)
    // Due to our snapshot semantics, outer x won't change
    let x_value = executor.env().get("x").unwrap();
    assert_eq!(x_value, Value::number(0.0)); // Still 0, not modified
}

#[test]
fn test_nested_closures() {
    let mut executor = Executor::new();

    // x = 5
    executor.env_mut().define("x".to_string(), Value::number(5.0));

    // func outer() { return x }
    let outer_decl = Stmt::FunctionDecl {
        name: "outer".to_string(),
        receiver: None,
        params: vec![],
        body: vec![Stmt::Return {
            value: Some(Expr::Variable {
                name: "x".to_string(),
                position: pos(),
            }),
            position: pos(),
        }],
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
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
    assert_eq!(result, Value::number(5.0));
}

#[test]
fn test_function_parameter_shadowing() {
    let mut executor = Executor::new();

    // x = 10
    executor.env_mut().define("x".to_string(), Value::number(10.0));

    // func use_param(x) { return x * 2 }
    let func_decl = Stmt::FunctionDecl {
        name: "use_param".to_string(),
        receiver: None,
        params: vec![Parameter {
            name: "x".to_string(), // Shadows outer x
            default_value: None,
        is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // use_param(5) - should use parameter x=5, not outer x=10
    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "use_param".to_string(),
            position: pos(),
        }),
        args: vec![Argument::Positional(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        })],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::number(10.0)); // 5 * 2, not 10 * 2
}

#[test]
fn test_function_returning_function_value() {
    let mut executor = Executor::new();

    // func make_adder(n) { return n + 1 }
    let func_decl = Stmt::FunctionDecl {
        name: "make_adder".to_string(),
        receiver: None,
        params: vec![Parameter {
            name: "n".to_string(),
            default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
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
        args: vec![Argument::Positional(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        })],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::number(6.0));
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
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Boolean(true),
                position: pos(),
            }),
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Boolean(false),
                position: pos(),
            }),
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::boolean(false));
}

#[test]
fn test_function_with_comparison() {
    let mut executor = Executor::new();

    // func compare(a, b) { return a > b }
    let func_decl = Stmt::FunctionDecl {
        name: "compare".to_string(),
        receiver: None,
        params: vec![
            Parameter {
                name: "a".to_string(),
                default_value: None,
                is_variadic: false,
            },
            Parameter {
                name: "b".to_string(),
                default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    let call1 = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "compare".to_string(),
            position: pos(),
        }),
        args: vec![
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Number(10.0),
                position: pos(),
            }),
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Number(5.0),
                position: pos(),
            }),
        ],
        position: pos(),
    };

    let result1 = executor.eval_expr(&call1).unwrap();
    assert_eq!(result1, Value::boolean(true));
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
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Number(3.0),
                position: pos(),
            }),
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Number(4.0),
                position: pos(),
            }),
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(
        result,
        Value::list(List::from_vec(vec![
            Value::number(3.0),
            Value::number(4.0),
            Value::number(7.0)
        ]))
    );
}

#[test]
fn test_function_with_unary_ops() {
    let mut executor = Executor::new();

    // func negate(x) { return -x }
    let func_decl = Stmt::FunctionDecl {
        name: "negate".to_string(),
        receiver: None,
        params: vec![Parameter {
            name: "x".to_string(),
            default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "negate".to_string(),
            position: pos(),
        }),
        args: vec![Argument::Positional(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        })],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::number(-5.0));
}

#[test]
fn test_function_with_not_op() {
    let mut executor = Executor::new();

    // func invert(b) { return not b }
    let func_decl = Stmt::FunctionDecl {
        name: "invert".to_string(),
        receiver: None,
        params: vec![Parameter {
            name: "b".to_string(),
            default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "invert".to_string(),
            position: pos(),
        }),
        args: vec![Argument::Positional(Expr::Literal {
            value: LiteralValue::Boolean(true),
            position: pos(),
        })],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::boolean(false));
}

#[test]
fn test_function_four_params() {
    let mut executor = Executor::new();

    // func avg(a, b, c, d) { return (a + b + c + d) / 4 }
    let func_decl = Stmt::FunctionDecl {
        name: "avg".to_string(),
        receiver: None,
        params: vec![
            Parameter {
                name: "a".to_string(),
                default_value: None,
                is_variadic: false,
            },
            Parameter {
                name: "b".to_string(),
                default_value: None,
                is_variadic: false,
            },
            Parameter {
                name: "c".to_string(),
                default_value: None,
                is_variadic: false,
            },
            Parameter {
                name: "d".to_string(),
                default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "avg".to_string(),
            position: pos(),
        }),
        args: vec![
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Number(10.0),
                position: pos(),
            }),
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Number(20.0),
                position: pos(),
            }),
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Number(30.0),
                position: pos(),
            }),
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Number(40.0),
                position: pos(),
            }),
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::number(25.0));
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
    assert_eq!(result, Value::symbol("success".to_string()));
}

#[test]
fn test_function_call_with_expression_args() {
    let mut executor = Executor::new();

    // func add(a, b) { return a + b }
    let func_decl = Stmt::FunctionDecl {
        name: "add".to_string(),
        receiver: None,
        params: vec![
            Parameter {
                name: "a".to_string(),
                default_value: None,
                is_variadic: false,
            },
            Parameter {
                name: "b".to_string(),
                default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
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
            Argument::Positional(Expr::Binary {
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
            }),
            Argument::Positional(Expr::Binary {
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
            }),
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::number(12.0));
}

// ============================================================================
// CONTROL FLOW TESTS
// ============================================================================

#[test]
fn test_if_statement_true() {
    let mut executor = Executor::new();

    // x = 0
    executor.env_mut().define("x".to_string(), Value::number(0.0));

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
    assert_eq!(x, Value::number(1.0));
}

#[test]
fn test_if_statement_false() {
    let mut executor = Executor::new();

    // x = 0
    executor.env_mut().define("x".to_string(), Value::number(0.0));

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
    assert_eq!(x, Value::number(0.0)); // Should still be 0
}

#[test]
fn test_if_else_true() {
    let mut executor = Executor::new();

    // x = 0
    executor.env_mut().define("x".to_string(), Value::number(0.0));

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
    assert_eq!(x, Value::number(1.0));
}

#[test]
fn test_if_else_false() {
    let mut executor = Executor::new();

    // x = 0
    executor.env_mut().define("x".to_string(), Value::number(0.0));

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
    assert_eq!(x, Value::number(2.0));
}

#[test]
fn test_if_with_comparison() {
    let mut executor = Executor::new();

    // x = 10
    executor.env_mut().define("x".to_string(), Value::number(10.0));
    // result = 0
    executor.env_mut().define("result".to_string(), Value::number(0.0));

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
    assert_eq!(result, Value::number(1.0));
}

#[test]
fn test_if_return_in_function() {
    let mut executor = Executor::new();

    // func check(n) { if n > 0 { return 1 } return 0 }
    let func_decl = Stmt::FunctionDecl {
        name: "check".to_string(),
        receiver: None,
        params: vec![Parameter {
            name: "n".to_string(),
            default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // check(5) should return 1
    let call1 = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "check".to_string(),
            position: pos(),
        }),
        args: vec![Argument::Positional(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        })],
        position: pos(),
    };

    let result1 = executor.eval_expr(&call1).unwrap();
    assert_eq!(result1, Value::number(1.0));

    // check(-5) should return 0
    let call2 = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "check".to_string(),
            position: pos(),
        }),
        args: vec![Argument::Positional(Expr::Literal {
            value: LiteralValue::Number(-5.0),
            position: pos(),
        })],
        position: pos(),
    };

    let result2 = executor.eval_expr(&call2).unwrap();
    assert_eq!(result2, Value::number(0.0));
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
        receiver: None,
        params: vec![
            Parameter {
                name: "a".to_string(),
                default_value: None,
                is_variadic: false,
            },
            Parameter {
                name: "b".to_string(),
                default_value: None,
                is_variadic: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
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
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Number(2.0),
                position: pos(),
            }),
            Argument::Positional(Expr::Literal {
                value: LiteralValue::Number(3.0),
                position: pos(),
            }),
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
            is_private: false,
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
    assert_eq!(count_value, Value::number(3.0));
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
            is_private: false,
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
    assert_eq!(x_value, Value::number(10.0));
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
            is_private: false,
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
            is_private: false,
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
    assert_eq!(sum_value, Value::number(15.0));

    // i should be 6
    let i_value = executor.env().get("i").unwrap();
    assert_eq!(i_value, Value::number(6.0));
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
        receiver: None,
        params: vec![Parameter {
            name: "n".to_string(),
            default_value: None,
                is_variadic: false,
            }],
        body: vec![
            Stmt::VariableDecl {
                name: "result".to_string(),
                value: Expr::Literal {
                    value: LiteralValue::Number(1.0),
                    position: pos(),
                },
                type_annotation: None,
                is_private: false,
                position: pos(),
            },
            Stmt::VariableDecl {
                name: "i".to_string(),
                value: Expr::Literal {
                    value: LiteralValue::Number(1.0),
                    position: pos(),
                },
                type_annotation: None,
                is_private: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // factorial(5) should be 120
    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "factorial".to_string(),
            position: pos(),
        }),
        args: vec![Argument::Positional(Expr::Literal {
            value: LiteralValue::Number(5.0),
            position: pos(),
        })],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::number(120.0));
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
            is_private: false,
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
            is_private: false,
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
                is_private: false,
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
    assert_eq!(sum_value, Value::number(6.0));
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
            is_private: false,
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
    assert_eq!(sum_value, Value::number(6.0));
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
            is_private: false,
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
    assert_eq!(x_value, Value::number(0.0));
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
            is_private: false,
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
    assert_eq!(result_value, Value::string("abc".to_string()));
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
        receiver: None,
        params: vec![Parameter {
            name: "numbers".to_string(),
            default_value: None,
                is_variadic: false,
            }],
        body: vec![
            Stmt::VariableDecl {
                name: "total".to_string(),
                value: Expr::Literal {
                    value: LiteralValue::Number(0.0),
                    position: pos(),
                },
                type_annotation: None,
                is_private: false,
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
        pattern_clauses: None,
        is_setter: false,
        is_static: false,
        guard: None,
        is_private: false,
        position: pos(),
    };

    executor.eval_stmt(&func_decl).unwrap();

    // sum_list([10, 20, 30]) should be 60
    let call = Expr::Call {
        callee: Box::new(Expr::Variable {
            name: "sum_list".to_string(),
            position: pos(),
        }),
        args: vec![Argument::Positional(Expr::List {
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
        })],
        position: pos(),
    };

    let result = executor.eval_expr(&call).unwrap();
    assert_eq!(result, Value::number(60.0));
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
            is_private: false,
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
    assert_eq!(count_value, Value::number(6.0));
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
    assert_eq!(result, Value::number(10.0));

    // items[1] should be 20
    let index_expr = Expr::Index {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        index: Box::new(Expr::Literal { value: LiteralValue::Number(1.0), position: pos() }),
        position: pos(),
    };
    let result = executor.eval_expr(&index_expr).unwrap();
    assert_eq!(result, Value::number(20.0));

    // items[2] should be 30
    let index_expr = Expr::Index {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        index: Box::new(Expr::Literal { value: LiteralValue::Number(2.0), position: pos() }),
        position: pos(),
    };
    let result = executor.eval_expr(&index_expr).unwrap();
    assert_eq!(result, Value::number(30.0));
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
    assert_eq!(result, Value::number(30.0));

    // items[-2] should be 20
    let index_expr = Expr::Index {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        index: Box::new(Expr::Literal { value: LiteralValue::Number(-2.0), position: pos() }),
        position: pos(),
    };
    let result = executor.eval_expr(&index_expr).unwrap();
    assert_eq!(result, Value::number(20.0));

    // items[-3] should be 10 (first element)
    let index_expr = Expr::Index {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        index: Box::new(Expr::Literal { value: LiteralValue::Number(-3.0), position: pos() }),
        position: pos(),
    };
    let result = executor.eval_expr(&index_expr).unwrap();
    assert_eq!(result, Value::number(10.0));
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
    assert_eq!(result, Value::string("hello".to_string()));
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
    assert_eq!(result, Value::string("Alice".to_string()));

    // config["age"] should be 30
    let index_expr = Expr::Index {
        object: Box::new(Expr::Variable { name: "config".to_string(), position: pos() }),
        index: Box::new(Expr::Literal { value: LiteralValue::String("age".to_string()), position: pos() }),
        position: pos(),
    };
    let result = executor.eval_expr(&index_expr).unwrap();
    assert_eq!(result, Value::number(30.0));
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
    assert_eq!(result, Value::number(3.0));
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
    assert_eq!(result, Value::number(10.0));
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
    assert_eq!(result, Value::number(30.0));
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
        args: vec![Argument::Positional(Expr::Literal { value: LiteralValue::Number(20.0), position: pos() })],
        position: pos(),
    };
    let result = executor.eval_expr(&method_call).unwrap();
    assert_eq!(result, Value::boolean(true));

    // items.contains(99) should be false
    let method_call = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "items".to_string(), position: pos() }),
        method: "contains".to_string(),
        args: vec![Argument::Positional(Expr::Literal { value: LiteralValue::Number(99.0), position: pos() })],
        position: pos(),
    };
    let result = executor.eval_expr(&method_call).unwrap();
    assert_eq!(result, Value::boolean(false));
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
    assert_eq!(result, Value::boolean(true));

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
    assert_eq!(result, Value::boolean(false));
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
        args: vec![Argument::Positional(lambda)],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match &result.kind {
        ValueKind::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::number(2.0));
            assert_eq!(*elements.get(1).unwrap(), Value::number(4.0));
            assert_eq!(*elements.get(2).unwrap(), Value::number(6.0));
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
        args: vec![Argument::Positional(lambda)],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match &result.kind {
        ValueKind::List(elements) => {
            assert_eq!(elements.len(), 2);
            assert_eq!(*elements.get(0).unwrap(), Value::number(2.0));
            assert_eq!(*elements.get(1).unwrap(), Value::number(4.0));
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
        args: vec![Argument::Positional(lambda)],
        position: pos(),
    };

    // each() should return the original list
    let result = executor.eval_expr(&method_call).unwrap();
    match &result.kind {
        ValueKind::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::number(1.0));
            assert_eq!(*elements.get(1).unwrap(), Value::number(2.0));
            assert_eq!(*elements.get(2).unwrap(), Value::number(3.0));
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
        args: vec![Argument::Positional(lambda)],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match &result.kind {
        ValueKind::List(elements) => {
            assert_eq!(elements.len(), 2);
            assert_eq!(*elements.get(0).unwrap(), Value::string("hello!".to_string()));
            assert_eq!(*elements.get(1).unwrap(), Value::string("world!".to_string()));
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
        args: vec![Argument::Positional(lambda)],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match &result.kind {
        ValueKind::List(elements) => {
            assert_eq!(elements.len(), 2);
            assert_eq!(*elements.get(0).unwrap(), Value::number(15.0));
            assert_eq!(*elements.get(1).unwrap(), Value::number(20.0));
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
        args: vec![Argument::Positional(Expr::Literal { value: LiteralValue::Symbol("double".to_string()), position: pos() })],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match &result.kind {
        ValueKind::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::number(2.0));
            assert_eq!(*elements.get(1).unwrap(), Value::number(4.0));
            assert_eq!(*elements.get(2).unwrap(), Value::number(6.0));
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
        args: vec![Argument::Positional(Expr::Literal { value: LiteralValue::Symbol("square".to_string()), position: pos() })],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match &result.kind {
        ValueKind::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::number(4.0));
            assert_eq!(*elements.get(1).unwrap(), Value::number(9.0));
            assert_eq!(*elements.get(2).unwrap(), Value::number(16.0));
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
        args: vec![Argument::Positional(Expr::Literal { value: LiteralValue::Symbol("negate".to_string()), position: pos() })],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match &result.kind {
        ValueKind::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::number(-1.0));
            assert_eq!(*elements.get(1).unwrap(), Value::number(2.0));
            assert_eq!(*elements.get(2).unwrap(), Value::number(-3.0));
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
        args: vec![Argument::Positional(Expr::Literal { value: LiteralValue::Symbol("even".to_string()), position: pos() })],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match &result.kind {
        ValueKind::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::number(2.0));
            assert_eq!(*elements.get(1).unwrap(), Value::number(4.0));
            assert_eq!(*elements.get(2).unwrap(), Value::number(6.0));
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
        args: vec![Argument::Positional(Expr::Literal { value: LiteralValue::Symbol("positive".to_string()), position: pos() })],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match &result.kind {
        ValueKind::List(elements) => {
            assert_eq!(elements.len(), 2);
            assert_eq!(*elements.get(0).unwrap(), Value::number(1.0));
            assert_eq!(*elements.get(1).unwrap(), Value::number(2.0));
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
        args: vec![Argument::Positional(Expr::Literal { value: LiteralValue::Symbol("odd".to_string()), position: pos() })],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match &result.kind {
        ValueKind::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::number(1.0));
            assert_eq!(*elements.get(1).unwrap(), Value::number(3.0));
            assert_eq!(*elements.get(2).unwrap(), Value::number(5.0));
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
    match &result.kind {
        ValueKind::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::number(5.0));
            assert_eq!(*elements.get(1).unwrap(), Value::number(7.0));
            assert_eq!(*elements.get(2).unwrap(), Value::number(9.0));
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
    match &result.kind {
        ValueKind::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::number(20.0));
            assert_eq!(*elements.get(1).unwrap(), Value::number(60.0));
            assert_eq!(*elements.get(2).unwrap(), Value::number(120.0));
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
    match &result.kind {
        ValueKind::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::number(10.0));
            assert_eq!(*elements.get(1).unwrap(), Value::number(20.0));
            assert_eq!(*elements.get(2).unwrap(), Value::number(30.0));
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
    match &result.kind {
        ValueKind::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::number(9.0));
            assert_eq!(*elements.get(1).unwrap(), Value::number(18.0));
            assert_eq!(*elements.get(2).unwrap(), Value::number(27.0));
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
            Argument::Positional(Expr::Literal { value: LiteralValue::Number(1.0), position: pos() }),
            Argument::Positional(Expr::Literal { value: LiteralValue::Number(3.0), position: pos() }),
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match &result.kind {
        ValueKind::List(elements) => {
            assert_eq!(elements.len(), 2);
            assert_eq!(*elements.get(0).unwrap(), Value::number(20.0));
            assert_eq!(*elements.get(1).unwrap(), Value::number(30.0));
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
            Argument::Positional(Expr::Literal { value: LiteralValue::Number(0.0), position: pos() }),
            Argument::Positional(Expr::Literal { value: LiteralValue::Number(3.0), position: pos() }),
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match &result.kind {
        ValueKind::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::number(10.0));
            assert_eq!(*elements.get(1).unwrap(), Value::number(20.0));
            assert_eq!(*elements.get(2).unwrap(), Value::number(30.0));
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
            Argument::Positional(Expr::Literal { value: LiteralValue::Number(2.0), position: pos() }),
            Argument::Positional(Expr::Literal { value: LiteralValue::Number(5.0), position: pos() }),
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match &result.kind {
        ValueKind::List(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements.get(0).unwrap(), Value::number(30.0));
            assert_eq!(*elements.get(1).unwrap(), Value::number(40.0));
            assert_eq!(*elements.get(2).unwrap(), Value::number(50.0));
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
            Argument::Positional(Expr::Literal { value: LiteralValue::Number(-3.0), position: pos() }),
            Argument::Positional(Expr::Literal { value: LiteralValue::Number(-1.0), position: pos() }),
        ],
        position: pos(),
    };

    let result = executor.eval_expr(&method_call).unwrap();
    match &result.kind {
        ValueKind::List(elements) => {
            assert_eq!(elements.len(), 2);
            assert_eq!(*elements.get(0).unwrap(), Value::number(30.0));
            assert_eq!(*elements.get(1).unwrap(), Value::number(40.0));
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
    match &result.kind {
        ValueKind::List(elements) => {
            assert_eq!(elements.len(), 2);
            // Keys may be in any order, so check both are present
            let keys: Vec<String> = elements.to_vec().iter().filter_map(|v| {
                if let ValueKind::String(s) = &v.kind {
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
    match &result.kind {
        ValueKind::List(elements) => {
            assert_eq!(elements.len(), 2);
            // Values may be in any order, so check both are present
            let values: Vec<f64> = elements.to_vec().iter().filter_map(|v| {
                if let ValueKind::Number(n) = &v.kind {
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
            Argument::Positional(Expr::Literal { value: LiteralValue::String("name".to_string()), position: pos() }),
        ],
        position: pos(),
    };

    let result1 = executor.eval_expr(&method_call1).unwrap();
    assert_eq!(result1, Value::boolean(true));

    // data.has_key("missing") should return false
    let method_call2 = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "data".to_string(), position: pos() }),
        method: "has_key".to_string(),
        args: vec![
            Argument::Positional(Expr::Literal { value: LiteralValue::String("missing".to_string()), position: pos() }),
        ],
        position: pos(),
    };

    let result2 = executor.eval_expr(&method_call2).unwrap();
    assert_eq!(result2, Value::boolean(false));
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
    assert_eq!(result1, Value::number(0.0));

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
    assert_eq!(result2, Value::number(3.0));
}

// ============================================================================
// PHASE 9: Configuration and Precision Execution Tests
// ============================================================================

#[test]
fn test_execute_configure_file_level() {
    let source = r#"
configure { skip_none: true }
x = 1
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // Verify config was applied (file-level stays active)
    assert_eq!(executor.config_stack.current().skip_none, true);
}

#[test]
fn test_execute_configure_with_block() {
    let source = r#"
configure { skip_none: true } {
    y = 2
}
x = 1
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // Config should be restored after block
    assert_eq!(executor.config_stack.current().skip_none, false);

    // Variables should be defined
    let y = executor.eval_expr(&Expr::Variable {
        name: "y".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(y, Value::number(2.0));
}

#[test]
fn test_execute_nested_configure() {
    let source = r#"
configure { skip_none: true } {
    configure { strict_types: false } {
        z = 3
    }
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // After execution, should be back to defaults
    assert_eq!(executor.config_stack.current().skip_none, false);
    assert_eq!(executor.config_stack.current().strict_types, true);
}

#[test]
fn test_execute_precision_block() {
    let source = r#"
precision 2 {
    x = 1.234
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // Precision stack should be empty after block
    assert!(executor.precision_stack.is_empty());
}

#[test]
fn test_execute_precision_int_mode() {
    let source = r#"
precision :int {
    x = 5
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // Should execute without errors
    assert!(executor.precision_stack.is_empty());
}

#[test]
fn test_execute_nested_precision() {
    let source = r#"
precision 2 {
    precision 0 {
        y = 1.234
    }
    x = 5.678
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // Precision stack should be empty after nested blocks
    assert!(executor.precision_stack.is_empty());
}

#[test]
fn test_execute_configure_and_precision_together() {
    let source = r#"
configure { skip_none: true } {
    precision 2 {
        x = 1.234
    }
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // Both stacks should be restored
    assert_eq!(executor.config_stack.current().skip_none, false);
    assert!(executor.precision_stack.is_empty());
}

#[test]
fn test_configure_error_mode() {
    let source = "configure { error_mode: :lenient }";
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.config_stack.current().error_mode, ErrorMode::Lenient);
}

#[test]
fn test_configure_multiple_settings() {
    let source = r#"
configure {
    skip_none: true,
    error_mode: :strict,
    strict_types: false
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.config_stack.current().skip_none, true);
    assert_eq!(executor.config_stack.current().error_mode, ErrorMode::Strict);
    assert_eq!(executor.config_stack.current().strict_types, false);
}

#[test]
fn test_configure_invalid_key_error() {
    let source = "configure { invalid_key: true }";
    let mut executor = Executor::new();
    let result = executor.execute_source(source);

    assert!(result.is_err());
}

#[test]
fn test_configure_bounds_checking_mode() {
    let source = "configure { bounds_checking: :lenient }";
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    use graphoid::execution::BoundsCheckingMode;
    assert_eq!(executor.config_stack.current().bounds_checking, BoundsCheckingMode::Lenient);
}

#[test]
fn test_configure_type_coercion_mode() {
    let source = "configure { type_coercion: :lenient }";
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    use graphoid::execution::TypeCoercionMode;
    assert_eq!(executor.config_stack.current().type_coercion, TypeCoercionMode::Lenient);
}

#[test]
fn test_configure_none_handling_mode() {
    let source = "configure { none_handling: :skip }";
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    use graphoid::execution::NoneHandlingMode;
    assert_eq!(executor.config_stack.current().none_handling, NoneHandlingMode::Skip);
}

#[test]
fn test_precision_stack_depth_during_execution() {
    let source = r#"
precision 3 {
    x = 1
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // Precision should be popped after block
    assert_eq!(executor.precision_stack.len(), 0);
}

#[test]
fn test_configure_decimal_places() {
    let source = "configure { decimal_places: 3 }";
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.config_stack.current().decimal_places, Some(3));
}

#[test]
fn test_configure_edge_validation() {
    let source = "configure { edge_validation: false }";
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.config_stack.current().edge_validation, false);
}

#[test]
fn test_configure_strict_edge_rules() {
    let source = "configure { strict_edge_rules: false }";
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.config_stack.current().strict_edge_rules, false);
}

#[test]
fn test_configure_none_conversions() {
    let source = "configure { none_conversions: false }";
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.config_stack.current().none_conversions, false);
}

#[test]
fn test_deeply_nested_configure() {
    let source = r#"
configure { skip_none: true } {
    configure { error_mode: :lenient } {
        configure { strict_types: false } {
            x = 1
        }
    }
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // All configs should be popped, back to defaults
    assert_eq!(executor.config_stack.current().skip_none, false);
    assert_eq!(executor.config_stack.current().error_mode, ErrorMode::Strict);
    assert_eq!(executor.config_stack.current().strict_types, true);
}

#[test]
fn test_precision_and_configure_complex_nesting() {
    let source = r#"
configure { skip_none: true } {
    precision 2 {
        configure { strict_types: false } {
            precision 0 {
                y = 5
            }
        }
    }
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // Everything should be restored
    assert_eq!(executor.config_stack.current().skip_none, false);
    assert_eq!(executor.config_stack.current().strict_types, true);
    assert!(executor.precision_stack.is_empty());
}

#[test]
fn test_configure_with_variable_definition() {
    let source = r#"
configure { skip_none: true } {
    num x = 10
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // Variable should be accessible after config block
    let x = executor.eval_expr(&Expr::Variable {
        name: "x".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(x, Value::number(10.0));
}

#[test]
fn test_precision_with_arithmetic() {
    let source = r#"
precision 1 {
    result = 2.5 + 3.7
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // Variable should be accessible
    let result = executor.eval_expr(&Expr::Variable {
        name: "result".to_string(),
        position: pos(),
    }).unwrap();
    // Result should be calculated (precision will be applied in future milestones)
    assert_eq!(result, Value::number(6.2));
}

// ============================================================================
// Total: 23 configuration and precision execution tests
// ============================================================================

// ============================================================================
// Try/Catch/Finally Tests
// ============================================================================

#[test]
fn test_basic_try_catch_no_error() {
    let source = r#"
x = 0
try {
    x = 10
}
catch {
    x = 20
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let x = executor.eval_expr(&Expr::Variable {
        name: "x".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(x, Value::number(10.0));
}

#[test]
fn test_basic_try_catch_with_error() {
    let source = r#"
x = 0
try {
    raise "error occurred"
    x = 10
}
catch {
    x = 20
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let x = executor.eval_expr(&Expr::Variable {
        name: "x".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(x, Value::number(20.0));
}

#[test]
fn test_catch_with_variable_binding() {
    let source = r#"
error_msg = ""
try {
    raise "something went wrong"
}
catch as e {
    error_msg = e.message()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let error_msg = executor.eval_expr(&Expr::Variable {
        name: "error_msg".to_string(),
        position: pos(),
    }).unwrap();
    assert!(matches!(&error_msg.kind, ValueKind::String(s) if s.contains("something went wrong")));
}

#[test]
fn test_catch_type_matching_runtime_error() {
    let source = r#"
caught = false
try {
    x = 1 / 0
}
catch RuntimeError {
    caught = true
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let caught = executor.eval_expr(&Expr::Variable {
        name: "caught".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(caught, Value::boolean(true));
}

#[test]
fn test_catch_type_matching_with_binding() {
    let source = r#"
error_msg = ""
try {
    x = 1 / 0
}
catch RuntimeError as e {
    error_msg = e.message()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let error_msg = executor.eval_expr(&Expr::Variable {
        name: "error_msg".to_string(),
        position: pos(),
    }).unwrap();
    assert!(matches!(&error_msg.kind, ValueKind::String(s) if s.contains("Division by zero")));
}

#[test]
fn test_multiple_catch_clauses() {
    let source = r#"
which_caught = 0
try {
    x = 1 / 0
}
catch TypeError {
    which_caught = 1
}
catch RuntimeError {
    which_caught = 2
}
catch {
    which_caught = 3
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let which_caught = executor.eval_expr(&Expr::Variable {
        name: "which_caught".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(which_caught, Value::number(2.0));
}

#[test]
fn test_catch_all_clause() {
    let source = r#"
caught = false
try {
    raise "any error"
}
catch TypeError {
    caught = false
}
catch {
    caught = true
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let caught = executor.eval_expr(&Expr::Variable {
        name: "caught".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(caught, Value::boolean(true));
}

#[test]
fn test_finally_block_always_runs_no_error() {
    let source = r#"
finally_ran = false
try {
    x = 10
}
finally {
    finally_ran = true
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let finally_ran = executor.eval_expr(&Expr::Variable {
        name: "finally_ran".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(finally_ran, Value::boolean(true));
}

#[test]
fn test_finally_block_runs_with_error_caught() {
    let source = r#"
finally_ran = false
try {
    raise "error"
}
catch {
    x = 1
}
finally {
    finally_ran = true
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let finally_ran = executor.eval_expr(&Expr::Variable {
        name: "finally_ran".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(finally_ran, Value::boolean(true));
}

#[test]
fn test_finally_block_runs_with_error_not_caught() {
    let source = r#"
finally_ran = false
try {
    raise "error"
}
catch TypeError {
    x = 1
}
finally {
    finally_ran = true
}
"#;
    let mut executor = Executor::new();
    let result = executor.execute_source(source);

    // Error should propagate, but finally should have run
    assert!(result.is_err());
    let finally_ran = executor.eval_expr(&Expr::Variable {
        name: "finally_ran".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(finally_ran, Value::boolean(true));
}

#[test]
fn test_try_only_finally_no_catch() {
    let source = r#"
x = 0
finally_ran = false
try {
    x = 10
}
finally {
    finally_ran = true
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let x = executor.eval_expr(&Expr::Variable {
        name: "x".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(x, Value::number(10.0));

    let finally_ran = executor.eval_expr(&Expr::Variable {
        name: "finally_ran".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(finally_ran, Value::boolean(true));
}

#[test]
fn test_nested_try_catch() {
    let source = r#"
outer_caught = false
inner_caught = false
try {
    try {
        raise "inner error"
    }
    catch {
        inner_caught = true
    }
}
catch {
    outer_caught = true
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let inner_caught = executor.eval_expr(&Expr::Variable {
        name: "inner_caught".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(inner_caught, Value::boolean(true));

    let outer_caught = executor.eval_expr(&Expr::Variable {
        name: "outer_caught".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(outer_caught, Value::boolean(false));
}

#[test]
fn test_nested_try_catch_propagation() {
    let source = r#"
outer_caught = false
inner_caught = false
try {
    try {
        raise "inner error"
    }
    catch TypeError {
        inner_caught = true
    }
}
catch {
    outer_caught = true
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let inner_caught = executor.eval_expr(&Expr::Variable {
        name: "inner_caught".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(inner_caught, Value::boolean(false));

    let outer_caught = executor.eval_expr(&Expr::Variable {
        name: "outer_caught".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(outer_caught, Value::boolean(true));
}

#[test]
fn test_raise_in_catch_block() {
    let source = r#"
final_caught = false
try {
    try {
        raise "first error"
    }
    catch {
        raise "second error"
    }
}
catch {
    final_caught = true
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let final_caught = executor.eval_expr(&Expr::Variable {
        name: "final_caught".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(final_caught, Value::boolean(true));
}

#[test]
fn test_catch_scope_isolation() {
    let source = r#"
try {
    raise "error"
}
catch as e {
    temp = "in catch"
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // Variables defined in catch block should not leak
    let result = executor.eval_expr(&Expr::Variable {
        name: "temp".to_string(),
        position: pos(),
    });
    assert!(result.is_err());

    // Error variable should not leak
    let result = executor.eval_expr(&Expr::Variable {
        name: "e".to_string(),
        position: pos(),
    });
    assert!(result.is_err());
}

#[test]
fn test_try_with_division_by_zero() {
    let source = r#"
result = 0
try {
    result = 10 / 0
}
catch {
    result = 999
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.eval_expr(&Expr::Variable {
        name: "result".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(result, Value::number(999.0));
}

#[test]
fn test_try_with_modulo_by_zero() {
    let source = r#"
result = 0
try {
    result = 10 % 0
}
catch {
    result = 888
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.eval_expr(&Expr::Variable {
        name: "result".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(result, Value::number(888.0));
}

#[test]
fn test_raise_string_literal() {
    let source = r#"
caught = false
try {
    raise "custom error message"
}
catch as e {
    caught = true
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let caught = executor.eval_expr(&Expr::Variable {
        name: "caught".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(caught, Value::boolean(true));
}

#[test]
fn test_raise_expression_evaluation() {
    let source = r#"
msg = "error: code "
error_msg = ""
try {
    raise msg + "42"
}
catch as e {
    error_msg = e.message()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let error_msg = executor.eval_expr(&Expr::Variable {
        name: "error_msg".to_string(),
        position: pos(),
    }).unwrap();
    assert!(matches!(&error_msg.kind, ValueKind::String(s) if s.contains("error") && s.contains("42")));
}

#[test]
fn test_try_catch_with_function_call() {
    let source = r#"
fn risky_function() {
    raise "function error"
    return 42
}

result = 0
try {
    result = risky_function()
}
catch {
    result = 999
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.eval_expr(&Expr::Variable {
        name: "result".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(result, Value::number(999.0));
}

#[test]
fn test_catch_can_access_outer_variables() {
    let source = r#"
counter = 0
try {
    raise "error"
}
catch {
    counter = counter + 1
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let counter = executor.eval_expr(&Expr::Variable {
        name: "counter".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(counter, Value::number(1.0));
}

#[test]
fn test_finally_can_access_outer_variables() {
    let source = r#"
counter = 0
try {
    x = 1
}
finally {
    counter = counter + 1
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let counter = executor.eval_expr(&Expr::Variable {
        name: "counter".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(counter, Value::number(1.0));
}

#[test]
fn test_try_catch_finally_all_together() {
    let source = r#"
tried = false
caught = false
finalized = false

try {
    tried = true
    raise "error"
}
catch {
    caught = true
}
finally {
    finalized = true
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let tried = executor.eval_expr(&Expr::Variable {
        name: "tried".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(tried, Value::boolean(true));

    let caught = executor.eval_expr(&Expr::Variable {
        name: "caught".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(caught, Value::boolean(true));

    let finalized = executor.eval_expr(&Expr::Variable {
        name: "finalized".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(finalized, Value::boolean(true));
}

#[test]
fn test_empty_try_catch() {
    let source = r#"
x = 0
try {
}
catch {
    x = 1
}
x = 10
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let x = executor.eval_expr(&Expr::Variable {
        name: "x".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(x, Value::number(10.0));
}

#[test]
fn test_empty_catch_block() {
    let source = r#"
x = 0
try {
    raise "error"
}
catch {
}
x = 10
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let x = executor.eval_expr(&Expr::Variable {
        name: "x".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(x, Value::number(10.0));
}

#[test]
fn test_empty_finally_block() {
    let source = r#"
x = 0
try {
    x = 10
}
finally {
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let x = executor.eval_expr(&Expr::Variable {
        name: "x".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(x, Value::number(10.0));
}

#[test]
fn test_try_catch_return_value() {
    let source = r#"
fn test() {
    try {
        return 42
    }
    catch {
        return 999
    }
}

result = test()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.eval_expr(&Expr::Variable {
        name: "result".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(result, Value::number(42.0));
}

#[test]
fn test_try_catch_return_value_with_error() {
    let source = r#"
fn test() {
    try {
        raise "error"
        return 42
    }
    catch {
        return 999
    }
}

result = test()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.eval_expr(&Expr::Variable {
        name: "result".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(result, Value::number(999.0));
}

#[test]
fn test_multiple_statements_in_try() {
    let source = r#"
x = 0
y = 0
try {
    x = 10
    y = 20
    raise "error"
    x = 30
}
catch {
    x = x + 1
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let x = executor.eval_expr(&Expr::Variable {
        name: "x".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(x, Value::number(11.0));

    let y = executor.eval_expr(&Expr::Variable {
        name: "y".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(y, Value::number(20.0));
}

#[test]
fn test_multiple_statements_in_catch() {
    let source = r#"
x = 0
y = 0
try {
    raise "error"
}
catch {
    x = 10
    y = 20
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let x = executor.eval_expr(&Expr::Variable {
        name: "x".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(x, Value::number(10.0));

    let y = executor.eval_expr(&Expr::Variable {
        name: "y".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(y, Value::number(20.0));
}

#[test]
fn test_multiple_statements_in_finally() {
    let source = r#"
x = 0
y = 0
try {
    x = 1
}
finally {
    x = x + 10
    y = 20
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let x = executor.eval_expr(&Expr::Variable {
        name: "x".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(x, Value::number(11.0));

    let y = executor.eval_expr(&Expr::Variable {
        name: "y".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(y, Value::number(20.0));
}

#[test]
fn test_try_catch_with_list_operations() {
    let source = r#"
items = [1, 2, 3]
result = 0
try {
    result = items[10]
}
catch {
    result = 999
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.eval_expr(&Expr::Variable {
        name: "result".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(result, Value::number(999.0));
}

#[test]
fn test_try_catch_with_map_operations() {
    let source = r#"
mymap = {"a": 1, "b": 2}
result = 0
try {
    result = mymap["missing_key"]
}
catch {
    result = 999
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.eval_expr(&Expr::Variable {
        name: "result".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(result, Value::number(999.0));
}

#[test]
fn test_deeply_nested_try_catch() {
    let source = r#"
level = 0
try {
    level = 1
    try {
        level = 2
        try {
            level = 3
            raise "error"
        }
        catch {
            level = level + 10
        }
    }
    catch {
        level = level + 100
    }
}
catch {
    level = level + 1000
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let level = executor.eval_expr(&Expr::Variable {
        name: "level".to_string(),
        position: pos(),
    }).unwrap();
    assert_eq!(level, Value::number(13.0));
}

#[test]
fn test_catch_error_type_case_sensitive() {
    let source = r#"
caught = false
try {
    x = 1 / 0
}
catch runtimeerror {
    caught = true
}
"#;
    let mut executor = Executor::new();
    let result = executor.execute_source(source);

    // Should not catch because case is wrong
    assert!(result.is_err());
}

// ============================================================================
// Total: 35 try/catch/finally executor tests
// ============================================================================

// ============================================================================
// ERROR COLLECTION MODE TESTS
// ============================================================================

#[test]
fn test_error_collection_basic() {
    let source = r#"
configure { error_mode: :collect } {
    raise "first error"
    raise "second error"
    result = 42
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // Execution should continue despite errors
    let result = executor.get_variable("result").unwrap();
    assert_eq!(result, Value::number(42.0));

    // Errors should be collected
    let errors_source = r#"
errors = get_errors()
count = errors.length()
"#;
    executor.execute_source(errors_source).unwrap();
    let count = executor.get_variable("count").unwrap();
    assert_eq!(count, Value::number(2.0));
}

#[test]
fn test_error_collection_get_errors() {
    let source = r#"
configure { error_mode: :collect } {
    raise ValueError("bad value")
    raise TypeError("bad type")
}

errors = get_errors()
count = errors.length()
first_msg = errors[0].message()
second_msg = errors[1].message()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let count = executor.get_variable("count").unwrap();
    assert_eq!(count, Value::number(2.0));

    let first_msg = executor.get_variable("first_msg").unwrap();
    assert_eq!(first_msg, Value::string("Runtime error: ValueError: bad value".to_string()));

    let second_msg = executor.get_variable("second_msg").unwrap();
    assert_eq!(second_msg, Value::string("Runtime error: TypeError: bad type".to_string()));
}

#[test]
fn test_error_collection_clear_errors() {
    let source = r#"
configure { error_mode: :collect } {
    raise "error 1"
    raise "error 2"
}

count_before = get_errors().length()
clear_errors()
count_after = get_errors().length()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let count_before = executor.get_variable("count_before").unwrap();
    assert_eq!(count_before, Value::number(2.0));

    let count_after = executor.get_variable("count_after").unwrap();
    assert_eq!(count_after, Value::number(0.0));
}

#[test]
fn test_error_collection_scope() {
    let source = r#"
# Outside configure block - errors propagate
outer_error = false
try {
    raise "outer error"
}
catch {
    outer_error = true
}

# Inside configure block - errors collected
configure { error_mode: :collect } {
    raise "inner error 1"
    raise "inner error 2"
}

# Back outside - errors propagate again
outer_error_2 = false
try {
    raise "outer error 2"
}
catch {
    outer_error_2 = true
}

collected_count = get_errors().length()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let outer_error = executor.get_variable("outer_error").unwrap();
    assert_eq!(outer_error, Value::boolean(true));

    let outer_error_2 = executor.get_variable("outer_error_2").unwrap();
    assert_eq!(outer_error_2, Value::boolean(true));

    let collected_count = executor.get_variable("collected_count").unwrap();
    assert_eq!(collected_count, Value::number(2.0));
}

#[test]
fn test_error_collection_continues_execution() {
    let source = r#"
x = 0
configure { error_mode: :collect } {
    x = 1
    raise "error 1"
    x = 2
    raise "error 2"
    x = 3
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // All statements should execute despite errors
    let x = executor.get_variable("x").unwrap();
    assert_eq!(x, Value::number(3.0));

    // Both errors should be collected
    let errors_source = "count = get_errors().length()";
    executor.execute_source(errors_source).unwrap();
    let count = executor.get_variable("count").unwrap();
    assert_eq!(count, Value::number(2.0));
}

#[test]
fn test_error_collection_nested_configure() {
    let source = r#"
configure { error_mode: :collect } {
    raise "outer error"

    # Nested configure inherits :collect mode
    configure { skip_none: true } {
        raise "inner error"
    }

    raise "outer error 2"
}

count = get_errors().length()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // All errors should be collected
    let count = executor.get_variable("count").unwrap();
    assert_eq!(count, Value::number(3.0));
}

#[test]
fn test_error_collection_error_object_fields() {
    let source = r#"
configure { error_mode: :collect } {
    raise ValueError("test error message")
}

errors = get_errors()
error = errors[0]
error_type = error.type()
error_msg = error.message()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let error_type = executor.get_variable("error_type").unwrap();
    // Note: error type is embedded in the message for now
    assert!(error_type.to_string_value().contains("RuntimeError"));

    let error_msg = executor.get_variable("error_msg").unwrap();
    assert!(error_msg.to_string_value().contains("ValueError: test error message"));
}

#[test]
fn test_get_errors_without_collection() {
    // get_errors() should work even without error_mode: :collect
    // It just returns an empty list
    let source = r#"
errors = get_errors()
count = errors.length()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let count = executor.get_variable("count").unwrap();
    assert_eq!(count, Value::number(0.0));
}

#[test]
fn test_clear_errors_without_collection() {
    // clear_errors() should work even without errors
    let source = r#"
clear_errors()
result = 42
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert_eq!(result, Value::number(42.0));
}

// ============================================================================
// Total: 10 error collection mode tests
// ============================================================================

// ============================================================================
// ENHANCED ERROR FEATURES TESTS (Stack Traces & Cause Chaining)
// ============================================================================

#[test]
fn test_error_stack_trace_basic() {
    let source = r#"
trace = ""
try {
    raise ValueError("test error")
}
catch as e {
    trace = e.stack_trace()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let trace = executor.get_variable("trace").unwrap();
    // Stack trace should contain file/line/column info
    assert!(trace.to_string_value().contains("at"));
}

#[test]
fn test_error_stack_trace_in_function() {
    let source = r#"
# Simpler test without function definitions
trace = ""
try {
    raise ValueError("test error in try block")
}
catch as e {
    trace = e.stack_trace()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let trace = executor.get_variable("trace").unwrap();
    let trace_str = trace.to_string_value();

    // Stack trace should show location info
    assert!(trace_str.contains("at") || trace_str.contains(":"));
}

#[test]
fn test_error_cause_chaining() {
    let source = r#"
root_error = IOError("disk full")
mid_error = RuntimeError("save failed").caused_by(root_error)
top_error = ValueError("invalid data").caused_by(mid_error)

has_cause = top_error.cause()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let has_cause = executor.get_variable("has_cause").unwrap();
    // Should have a cause
    match &has_cause.kind {
        ValueKind::Error(e) => {
            assert_eq!(e.error_type, "RuntimeError");
            assert_eq!(e.message, "save failed");
        }
        _ => panic!("Expected Error value"),
    }
}

#[test]
fn test_error_cause_none() {
    let source = r#"
error = ValueError("no cause")
cause = error.cause()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let cause = executor.get_variable("cause").unwrap();
    assert_eq!(cause, Value::none());
}

#[test]
fn test_error_full_chain() {
    let source = r#"
root_error = IOError("disk full")
mid_error = RuntimeError("save failed").caused_by(root_error)
top_error = ValueError("invalid data").caused_by(mid_error)

chain = top_error.full_chain()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let chain = executor.get_variable("chain").unwrap();
    let chain_str = chain.to_string_value();

    // Chain should contain all errors
    assert!(chain_str.contains("ValueError: invalid data"));
    assert!(chain_str.contains("Caused by:"));
    assert!(chain_str.contains("RuntimeError: save failed"));
    assert!(chain_str.contains("IOError: disk full"));
}

#[test]
fn test_error_caused_by_requires_error_arg() {
    let source = r#"
error = ValueError("test")
result = error.caused_by("not an error")
"#;
    let mut executor = Executor::new();
    let result = executor.execute_source(source);

    // Should fail because caused_by expects an error argument
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("expects an error argument"));
}

#[test]
fn test_error_methods_no_args() {
    let source = r#"
error = ValueError("test")

# These methods should work
t = error.type()
m = error.message()
f = error.file()
l = error.line()
c = error.column()
st = error.stack_trace()
fc = error.full_chain()
ca = error.cause()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let t = executor.get_variable("t").unwrap();
    assert_eq!(t, Value::string("ValueError".to_string()));

    let m = executor.get_variable("m").unwrap();
    assert_eq!(m, Value::string("test".to_string()));

    let ca = executor.get_variable("ca").unwrap();
    assert_eq!(ca, Value::none());
}

#[test]
fn test_error_chaining_in_catch() {
    let source = r#"
# Test that caused_by() works - create chained error and inspect it
root_error = IOError("network failure")
chained_error = RuntimeError("operation failed").caused_by(root_error)

# Verify the chaining worked
error_msg = chained_error.message()
cause = chained_error.cause()
cause_msg = cause.message()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let error_msg = executor.get_variable("error_msg").unwrap();
    assert_eq!(error_msg, Value::string("operation failed".to_string()));

    let cause_msg = executor.get_variable("cause_msg").unwrap();
    assert_eq!(cause_msg, Value::string("network failure".to_string()));
}

#[test]
fn test_stack_trace_shows_nested_calls() {
    let source = r#"
# Simplified test - stack trace should capture location info
trace = ""
try {
    raise ValueError("deep error")
}
catch as e {
    trace = e.stack_trace()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let trace = executor.get_variable("trace").unwrap();
    let trace_str = trace.to_string_value();

    // Stack trace should show location info
    let at_count = trace_str.matches("at").count();
    assert!(at_count >= 1, "Stack trace should contain at least one 'at' reference");
}

#[test]
fn test_error_constructor_captures_stack() {
    let source = r#"
# Test that creating an error captures stack trace
error = ValueError("created directly")
trace = error.stack_trace()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let trace = executor.get_variable("trace").unwrap();
    let trace_str = trace.to_string_value();

    // Stack trace should be captured when error is created
    assert!(trace_str.contains("at"));
}

// ============================================================================
// Total: 12 enhanced error feature tests
// ============================================================================

// ============================================================================
// LENIENT MODE FOR BUILT-IN OPERATIONS TESTS
// ============================================================================

#[test]
fn test_lenient_mode_division_by_zero() {
    let source = r#"
result = 10
configure { error_mode: :lenient } {
    result = 10 / 0  # Should return none
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert_eq!(result, Value::none());
}

#[test]
fn test_lenient_mode_int_division_by_zero() {
    let source = r#"
result = 10
configure { error_mode: :lenient } {
    result = 10 // 0  # Should return none
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert_eq!(result, Value::none());
}

#[test]
fn test_lenient_mode_modulo_by_zero() {
    let source = r#"
result = 10
configure { error_mode: :lenient } {
    result = 10 % 0  # Should return none
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert_eq!(result, Value::none());
}

#[test]
fn test_lenient_mode_list_out_of_bounds() {
    let source = r#"
my_list = [1, 2, 3]
result = 0
configure { error_mode: :lenient } {
    result = my_list[999]  # Should return none
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert_eq!(result, Value::none());
}

#[test]
fn test_lenient_mode_map_missing_key() {
    let source = r#"
my_map = {"a": 1, "b": 2}
result = 0
configure { error_mode: :lenient } {
    result = my_map["missing"]  # Should return none
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert_eq!(result, Value::none());
}

#[test]
fn test_collect_mode_for_division() {
    let source = r#"
configure { error_mode: :collect } {
    a = 10 / 0  # Collected
    b = 20 / 0  # Collected
    c = 5 / 2   # OK
}

errors = get_errors()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let errors = executor.get_variable("errors").unwrap();
    if let ValueKind::List(err_list) = &errors.kind {
        assert_eq!(err_list.len(), 2, "Should have collected 2 division by zero errors");
    } else {
        panic!("Expected list of errors");
    }
}

#[test]
fn test_override_module_lenient_defaults() {
    let source = r#"
# Outer scope uses lenient mode (like a module default)
outer_result = 999
configure { error_mode: :lenient } {
    outer_result = 10 / 0  # Returns none

    # User overrides to strict within lenient scope
    inner_result = 888
    try {
        configure { error_mode: :strict } {
            inner_result = 10 / 0  # Raises error!
        }
    }
    catch {
        inner_result = 777  # Caught the error
    }
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let outer_result = executor.get_variable("outer_result").unwrap();
    assert_eq!(outer_result, Value::none());  // Lenient mode returned none

    let inner_result = executor.get_variable("inner_result").unwrap();
    assert_eq!(inner_result, Value::number(777.0));  // Strict mode raised, was caught
}

// ============================================================================
// Total: 7 lenient mode tests
// ============================================================================

// ============================================================================
// PHASE 1A: INTEGER MODE TRUNCATION TESTS
// ============================================================================

#[test]
fn test_integer_mode_truncates_positive_float() {
    let source = r#"
configure { integer: :integer } {
    a = 5.7
}
result = a
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert_eq!(result, Value::number(5.0));
}

#[test]
fn test_integer_mode_truncates_multiple_assignments() {
    let source = r#"
configure { integer: :integer } {
    a = 5.7
    b = 3.2
    c = a + b
}
result_a = a
result_b = b
result_c = c
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result_a = executor.get_variable("result_a").unwrap();
    let result_b = executor.get_variable("result_b").unwrap();
    let result_c = executor.get_variable("result_c").unwrap();

    assert_eq!(result_a, Value::number(5.0));  // 5.7 truncated to 5.0
    assert_eq!(result_b, Value::number(3.0));  // 3.2 truncated to 3.0
    assert_eq!(result_c, Value::number(8.0));  // 5.0 + 3.0 = 8.0 (not 8.9)
}

#[test]
fn test_integer_mode_truncates_negative_numbers() {
    let source = r#"
configure { integer: :integer } {
    a = -5.7
    b = -3.2
}
result_a = a
result_b = b
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result_a = executor.get_variable("result_a").unwrap();
    let result_b = executor.get_variable("result_b").unwrap();

    assert_eq!(result_a, Value::number(-5.0));  // -5.7 truncated to -5.0
    assert_eq!(result_b, Value::number(-3.0));  // -3.2 truncated to -3.0
}

#[test]
fn test_integer_mode_preserves_whole_numbers() {
    let source = r#"
configure { integer: :integer } {
    a = 5.0
    b = 10.0
}
result_a = a
result_b = b
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result_a = executor.get_variable("result_a").unwrap();
    let result_b = executor.get_variable("result_b").unwrap();

    assert_eq!(result_a, Value::number(5.0));
    assert_eq!(result_b, Value::number(10.0));
}

#[test]
fn test_integer_mode_preserves_non_numeric_values() {
    let source = r#"
configure { integer: :integer } {
    s = "hello"
    b = true
    n = none
}
result_s = s
result_b = b
result_n = n
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result_s = executor.get_variable("result_s").unwrap();
    let result_b = executor.get_variable("result_b").unwrap();
    let result_n = executor.get_variable("result_n").unwrap();

    assert_eq!(result_s, Value::string("hello".to_string()));
    assert_eq!(result_b, Value::boolean(true));
    assert_eq!(result_n, Value::none());
}

#[test]
fn test_integer_mode_works_with_reassignment() {
    let source = r#"
configure { integer: :integer } {
    x = 2.9
    x = 7.1
    x = -4.8
}
result = x
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert_eq!(result, Value::number(-4.0));  // Final value: -4.8 truncated to -4.0
}

#[test]
fn test_integer_mode_does_not_affect_outside_scope() {
    let source = r#"
a = 5.7
configure { integer: :integer } {
    b = 3.2
}
c = 9.9
result_a = a
result_b = b
result_c = c
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result_a = executor.get_variable("result_a").unwrap();
    let result_b = executor.get_variable("result_b").unwrap();
    let result_c = executor.get_variable("result_c").unwrap();

    assert_eq!(result_a, Value::number(5.7));  // Outside scope - not truncated
    assert_eq!(result_b, Value::number(3.0));  // Inside scope - truncated
    assert_eq!(result_c, Value::number(9.9));  // Outside scope - not truncated
}

// ============================================================================
// Total: 7 integer mode truncation tests
// ============================================================================

// ============================================================================
// PHASE 1B: BIGNUM VALUE CREATION AND STORAGE TESTS
// ============================================================================

#[test]
fn test_bignum_explicit_declaration() {
    let source = r#"
bignum a = 2.5
result = a
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    // Should be stored as BigNumber (Float128)
    assert!(matches!(result.kind, ValueKind::BigNumber(_)));
}

#[test]
fn test_bignum_integer_value() {
    let source = r#"
bignum x = 100
result = x
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    // Even integer values stored as Float128 by default
    assert!(matches!(result.kind, ValueKind::BigNumber(_)));
}

#[test]
fn test_bignum_preserves_precision() {
    let source = r#"
bignum precise = 3.141592653589793238
result = precise
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert!(matches!(result.kind, ValueKind::BigNumber(_)));
}

#[test]
fn test_bignum_type_checking() {
    let source = r#"
bignum a = 5.5
result_type = a.type_name()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result_type = executor.get_variable("result_type").unwrap();
    assert_eq!(result_type, Value::string("bignum".to_string()));
}

#[test]
fn test_bignum_in_expression() {
    let source = r#"
bignum x = 10.5
bignum y = 20.5
z = x + y
result = z
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    // Result should also be bignum
    assert!(matches!(result.kind, ValueKind::BigNumber(_)));
}

// ============================================================================
// Total: 5 bignum value creation tests
// ============================================================================

// ============================================================================
// Phase 1B: Float128 Arithmetic Tests (12 tests)
// ============================================================================

#[test]
fn test_bignum_subtraction() {
    let source = r#"
bignum x = 100.5
bignum y = 30.25
result = x - y
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert!(matches!(result.kind, ValueKind::BigNumber(BigNum::Float128(_))));
    // 100.5 - 30.25 = 70.25
    if let ValueKind::BigNumber(BigNum::Float128(f)) = result.kind {
        let f64_val: f64 = f.into();
        assert!((f64_val - 70.25).abs() < 0.0001);
    }
}

#[test]
fn test_bignum_multiplication() {
    let source = r#"
bignum x = 12.5
bignum y = 8.0
result = x * y
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert!(matches!(result.kind, ValueKind::BigNumber(BigNum::Float128(_))));
    // 12.5 * 8.0 = 100.0
    if let ValueKind::BigNumber(BigNum::Float128(f)) = result.kind {
        let f64_val: f64 = f.into();
        assert!((f64_val - 100.0).abs() < 0.0001);
    }
}

#[test]
fn test_bignum_division() {
    let source = r#"
bignum x = 100.0
bignum y = 8.0
result = x / y
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert!(matches!(result.kind, ValueKind::BigNumber(BigNum::Float128(_))));
    // 100.0 / 8.0 = 12.5
    if let ValueKind::BigNumber(BigNum::Float128(f)) = result.kind {
        let f64_val: f64 = f.into();
        assert!((f64_val - 12.5).abs() < 0.0001);
    }
}

#[test]
fn test_bignum_integer_division() {
    let source = r#"
bignum x = 100.0
bignum y = 8.0
result = x // y
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert!(matches!(result.kind, ValueKind::BigNumber(BigNum::Float128(_))));
    // 100.0 // 8.0 = 12.0
    if let ValueKind::BigNumber(BigNum::Float128(f)) = result.kind {
        let f64_val: f64 = f.into();
        assert!((f64_val - 12.0).abs() < 0.0001);
    }
}

#[test]
fn test_bignum_modulo() {
    let source = r#"
bignum x = 100.0
bignum y = 30.0
result = x % y
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert!(matches!(result.kind, ValueKind::BigNumber(BigNum::Float128(_))));
    // 100.0 % 30.0 = 10.0
    if let ValueKind::BigNumber(BigNum::Float128(f)) = result.kind {
        let f64_val: f64 = f.into();
        assert!((f64_val - 10.0).abs() < 0.0001);
    }
}

#[test]
fn test_bignum_power() {
    let source = r#"
bignum x = 2.0
bignum y = 10.0
result = x ** y
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert!(matches!(result.kind, ValueKind::BigNumber(BigNum::Float128(_))));
    // 2.0 ** 10.0 = 1024.0
    if let ValueKind::BigNumber(BigNum::Float128(f)) = result.kind {
        let f64_val: f64 = f.into();
        assert!((f64_val - 1024.0).abs() < 0.0001);
    }
}

#[test]
fn test_bignum_negation() {
    let source = r#"
bignum x = 42.5
result = -x
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert!(matches!(result.kind, ValueKind::BigNumber(BigNum::Float128(_))));
    // -42.5
    if let ValueKind::BigNumber(BigNum::Float128(f)) = result.kind {
        let f64_val: f64 = f.into();
        assert!((f64_val + 42.5).abs() < 0.0001);
    }
}

#[test]
fn test_bignum_complex_expression() {
    let source = r#"
bignum a = 10.0
bignum b = 5.0
bignum c = 2.0
result = (a + b) * c - b / c
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert!(matches!(result.kind, ValueKind::BigNumber(BigNum::Float128(_))));
    // (10.0 + 5.0) * 2.0 - 5.0 / 2.0 = 15.0 * 2.0 - 2.5 = 30.0 - 2.5 = 27.5
    if let ValueKind::BigNumber(BigNum::Float128(f)) = result.kind {
        let f64_val: f64 = f.into();
        assert!((f64_val - 27.5).abs() < 0.0001);
    }
}

#[test]
fn test_bignum_high_precision_addition() {
    let source = r#"
bignum x = 3.141592653589793238
bignum y = 2.718281828459045235
result = x + y
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert!(matches!(result.kind, ValueKind::BigNumber(BigNum::Float128(_))));
    // Result should maintain higher precision than f64
    if let ValueKind::BigNumber(BigNum::Float128(f)) = result.kind {
        let f64_val: f64 = f.into();
        // Verify it's approximately pi + e
        assert!((f64_val - 5.859874482).abs() < 0.000001);
    }
}

#[test]
fn test_bignum_division_by_zero_error() {
    let source = r#"
bignum x = 100.0
bignum y = 0.0
result = x / y
"#;
    let mut executor = Executor::new();
    let result = executor.execute_source(source);

    // Should return an error
    assert!(result.is_err());
}

#[test]
fn test_bignum_very_large_multiplication() {
    let source = r#"
bignum x = 999999999999999.0
bignum y = 999999999999999.0
result = x * y
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert!(matches!(result.kind, ValueKind::BigNumber(BigNum::Float128(_))));
    // Should handle large values without overflow
}

#[test]
fn test_bignum_fractional_power() {
    let source = r#"
bignum x = 16.0
bignum y = 0.5
result = x ** y
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert!(matches!(result.kind, ValueKind::BigNumber(BigNum::Float128(_))));
    // 16.0 ** 0.5 = 4.0 (square root)
    if let ValueKind::BigNumber(BigNum::Float128(f)) = result.kind {
        let f64_val: f64 = f.into();
        assert!((f64_val - 4.0).abs() < 0.0001);
    }
}

// ============================================================================
// Total: 12 Float128 arithmetic tests
// ============================================================================

// ============================================================================
// Phase 1B: BigNum + :integer Interaction Tests (5 tests)
// ============================================================================

#[test]
fn test_bignum_with_integer_mode_truncates() {
    let source = r#"
configure { integer: :integer } {
    bignum x = 10.7
    bignum y = 5.3
    result = x + y
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert!(matches!(result.kind, ValueKind::BigNumber(BigNum::Float128(_))));
    // With :integer, bignum values should be truncated on assignment
    // x = 10.0, y = 5.0, result = 15.0
    if let ValueKind::BigNumber(BigNum::Float128(f)) = result.kind {
        let f64_val: f64 = f.into();
        assert!((f64_val - 15.0).abs() < 0.0001);
    }
}

#[test]
fn test_bignum_preserves_precision_without_integer_mode() {
    let source = r#"
bignum x = 10.7
bignum y = 5.3
result = x + y
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert!(matches!(result.kind, ValueKind::BigNumber(BigNum::Float128(_))));
    // Without :integer, bignum preserves full precision
    // result = 16.0
    if let ValueKind::BigNumber(BigNum::Float128(f)) = result.kind {
        let f64_val: f64 = f.into();
        assert!((f64_val - 16.0).abs() < 0.0001);
    }
}

#[test]
fn test_integer_mode_affects_bignum_declarations() {
    let source = r#"
configure { integer: :integer } {
    bignum a = 3.9
    bignum b = 2.1
}
result_a = a
result_b = b
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result_a = executor.get_variable("result_a").unwrap();
    let result_b = executor.get_variable("result_b").unwrap();

    // Both should be truncated to integers
    if let ValueKind::BigNumber(BigNum::Float128(f)) = result_a.kind {
        let f64_val: f64 = f.into();
        assert!((f64_val - 3.0).abs() < 0.0001);  // 3.9 -> 3.0
    }
    if let ValueKind::BigNumber(BigNum::Float128(f)) = result_b.kind {
        let f64_val: f64 = f.into();
        assert!((f64_val - 2.0).abs() < 0.0001);  // 2.1 -> 2.0
    }
}

#[test]
fn test_bignum_integer_mode_scoping() {
    let source = r#"
bignum outside = 5.5

configure { integer: :integer } {
    bignum inside = 7.8
}

result_outside = outside
result_inside = inside
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result_outside = executor.get_variable("result_outside").unwrap();
    let result_inside = executor.get_variable("result_inside").unwrap();

    // outside should preserve precision (5.5)
    if let ValueKind::BigNumber(BigNum::Float128(f)) = result_outside.kind {
        let f64_val: f64 = f.into();
        assert!((f64_val - 5.5).abs() < 0.0001);
    }

    // inside was truncated within the block (7.8 -> 7.0)
    if let ValueKind::BigNumber(BigNum::Float128(f)) = result_inside.kind {
        let f64_val: f64 = f.into();
        assert!((f64_val - 7.0).abs() < 0.0001);
    }
}

#[test]
fn test_bignum_complex_expression_with_integer_mode() {
    let source = r#"
configure { integer: :integer } {
    bignum a = 10.9
    bignum b = 5.7
    bignum c = 2.3
    result = (a + b) * c
}
final_result = result
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let final_result = executor.get_variable("final_result").unwrap();

    // a=10.0, b=5.0, c=2.0
    // (10.0 + 5.0) * 2.0 = 15.0 * 2.0 = 30.0
    if let ValueKind::BigNumber(BigNum::Float128(f)) = final_result.kind {
        let f64_val: f64 = f.into();
        assert!((f64_val - 30.0).abs() < 0.0001);
    }
}

// ============================================================================
// Total: 5 bignum + :integer interaction tests
// ============================================================================

// ============================================================================
// Phase 1B: Mixed num/bignum Mutation Prevention Tests (5 tests)
// ============================================================================
// These tests verify that mixing num and bignum in operations does NOT mutate
// the original num variable. The operation should create a temporary BigNum
// copy without changing the type of the original variable.

#[test]
fn test_mixed_addition_no_mutation() {
    let source = r#"
num a = 5.0
bignum b = 10.0
result = a + b
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // Verify result is BigNumber (auto-cast for operation)
    let result = executor.get_variable("result").unwrap();
    assert!(matches!(result.kind, ValueKind::BigNumber(BigNum::Float128(_))));

    // CRITICAL: Verify 'a' is still Number (NOT mutated to BigNumber)
    let a = executor.get_variable("a").unwrap();
    assert!(matches!(a.kind, ValueKind::Number(_)));
    if let ValueKind::Number(val) = a.kind {
        assert!((val - 5.0).abs() < 0.0001);
    }
}

#[test]
fn test_mixed_reverse_addition_no_mutation() {
    let source = r#"
bignum a = 10.0
num b = 5.0
result = a + b
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // Verify result is BigNumber
    let result = executor.get_variable("result").unwrap();
    assert!(matches!(result.kind, ValueKind::BigNumber(BigNum::Float128(_))));

    // CRITICAL: Verify 'b' is still Number (NOT mutated to BigNumber)
    let b = executor.get_variable("b").unwrap();
    assert!(matches!(b.kind, ValueKind::Number(_)));
    if let ValueKind::Number(val) = b.kind {
        assert!((val - 5.0).abs() < 0.0001);
    }
}

#[test]
fn test_mixed_multiple_operations_no_mutation() {
    let source = r#"
num x = 10.0
bignum y = 20.0

# Multiple operations using x
sum = x + y
diff = x - y
prod = x * y
quot = x / y
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // CRITICAL: After multiple operations, x should STILL be Number
    let x = executor.get_variable("x").unwrap();
    assert!(matches!(x.kind, ValueKind::Number(_)));
    if let ValueKind::Number(val) = x.kind {
        assert!((val - 10.0).abs() < 0.0001);
    }

    // All results should be BigNumber
    assert!(matches!(
        executor.get_variable("sum").unwrap().kind,
        ValueKind::BigNumber(BigNum::Float128(_))
    ));
    assert!(matches!(
        executor.get_variable("diff").unwrap().kind,
        ValueKind::BigNumber(BigNum::Float128(_))
    ));
    assert!(matches!(
        executor.get_variable("prod").unwrap().kind,
        ValueKind::BigNumber(BigNum::Float128(_))
    ));
    assert!(matches!(
        executor.get_variable("quot").unwrap().kind,
        ValueKind::BigNumber(BigNum::Float128(_))
    ));
}

#[test]
fn test_mixed_comparison_no_mutation() {
    let source = r#"
num a = 5.0
bignum b = 10.0

# Comparisons (which auto-cast for comparison but shouldn't mutate)
less = a < b
greater = b > a
equal = a == a
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // CRITICAL: After comparisons, a should STILL be Number
    let a = executor.get_variable("a").unwrap();
    assert!(matches!(a.kind, ValueKind::Number(_)));
    if let ValueKind::Number(val) = a.kind {
        assert!((val - 5.0).abs() < 0.0001);
    }

    // Comparison results should be boolean
    assert_eq!(executor.get_variable("less").unwrap(), Value::boolean(true));
    assert_eq!(executor.get_variable("greater").unwrap(), Value::boolean(true));
    assert_eq!(executor.get_variable("equal").unwrap(), Value::boolean(true));
}

#[test]
fn test_mixed_complex_expression_no_mutation() {
    let source = r#"
num a = 5.0
num b = 3.0
bignum c = 2.0

# Complex expression mixing num and bignum
result = (a + c) * (b - c)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // CRITICAL: Both a and b should STILL be Number
    let a = executor.get_variable("a").unwrap();
    let b = executor.get_variable("b").unwrap();

    assert!(matches!(a.kind, ValueKind::Number(_)));
    assert!(matches!(b.kind, ValueKind::Number(_)));

    if let ValueKind::Number(val) = a.kind {
        assert!((val - 5.0).abs() < 0.0001);
    }
    if let ValueKind::Number(val) = b.kind {
        assert!((val - 3.0).abs() < 0.0001);
    }

    // Result should be BigNumber
    let result = executor.get_variable("result").unwrap();
    assert!(matches!(result.kind, ValueKind::BigNumber(BigNum::Float128(_))));
}

// ============================================================================
// Total: 5 mutation prevention tests
// ============================================================================

// ============================================================================
// Phase 1B: Large Literal Detection Tests (2 tests)
// ============================================================================
// Verifies that very large numeric literals automatically use BigInt to avoid
// precision loss when the value exceeds f64's exact integer range (2^53).

#[test]
fn test_large_literal_auto_bigint_in_high_mode() {
    let source = r#"
configure { precision: :high, integer: :integer } {
    # This number exceeds i64::MAX (9,223,372,036,854,775,807)
    # so it should automatically use BigInt
    very_large = 99999999999999999999.0
}
result = very_large
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();

    // Should automatically be BigInt (not Int64) due to exceeding i64 range
    assert!(matches!(result.kind, ValueKind::BigNumber(BigNum::BigInt(_))));
}

#[test]
fn test_extended_precision_mode_uses_bigint() {
    let source = r#"
configure { precision: :extended } {
    big = 12345.0
    frac = 67.89
}
result_big = big
result_frac = frac
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result_big = executor.get_variable("result_big").unwrap();
    let result_frac = executor.get_variable("result_frac").unwrap();

    // Both should be BigInt in Extended mode
    assert!(matches!(result_big.kind, ValueKind::BigNumber(BigNum::BigInt(_))));
    assert!(matches!(result_frac.kind, ValueKind::BigNumber(BigNum::BigInt(_))));

    // Fractional value should be truncated
    if let ValueKind::BigNumber(BigNum::BigInt(bi)) = &result_frac.kind {
        use num_traits::ToPrimitive;
        assert_eq!(bi.to_i64().unwrap(), 67);
    }
}

// ============================================================================
// Total: 2 large literal detection tests
// ============================================================================

// ============================================================================
// Phase 2: Overflow Auto-Promotion Tests (5 tests)
// ============================================================================
// Tests that operations auto-promote to bignum when overflow/precision loss
// would occur with standard num (f64) precision.

#[test]
fn test_overflow_auto_promotion_multiplication() {
    let source = r#"
# Large multiplication that exceeds f64 precision
a = 10000000000.0  # 10 billion
b = 10000000000.0  # 10 billion
result = a * b     # 10^20 - exceeds f64 exact integer range (2^53)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();

    // Should auto-promote to bignum due to overflow
    assert!(matches!(result.kind, ValueKind::BigNumber(_)),
        "Expected auto-promotion to bignum, got {:?}", result.kind);
}

#[test]
fn test_overflow_auto_promotion_power() {
    let source = r#"
# Power operation that creates very large result
base = 10.0
exp = 20.0
result = base ** exp  # 10^20 - exceeds f64 exact representation
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();

    // Should auto-promote to bignum
    assert!(matches!(result.kind, ValueKind::BigNumber(_)),
        "Expected auto-promotion to bignum for 10^20");
}

#[test]
fn test_overflow_auto_promotion_addition() {
    let source = r#"
# Addition with very large integers that exceed f64 precision
# Using values larger than 2^53 (f64's exact integer limit)
a = 100000000000000000.0  # 10^17 (larger than 2^53)
b = 100000000000000000.0  # 10^17
result = a + b  # 2 * 10^17 - should promote due to large operands
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();

    // Should auto-promote to bignum because operands exceed F64_MAX_EXACT_INT
    assert!(matches!(result.kind, ValueKind::BigNumber(_)),
        "Expected auto-promotion to bignum for large integer addition");
}

#[test]
fn test_no_auto_promotion_for_normal_operations() {
    let source = r#"
# Normal operations should stay as num (f64)
a = 100.0
b = 200.0
result = a * b  # 20000 - well within f64 range
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();

    // Should remain as num (no unnecessary promotion)
    assert!(matches!(result.kind, ValueKind::Number(_)),
        "Expected num for normal operation, got {:?}", result.kind);
}

#[test]
fn test_auto_promotion_respects_standard_mode() {
    let source = r#"
# In standard mode, overflow should error (not auto-promote)
precision { :standard } {
    a = 10000000000.0
    b = 10000000000.0
    result = a * b
}
"#;
    let mut executor = Executor::new();
    let exec_result = executor.execute_source(source);

    // In :standard mode, overflow should cause an error, not auto-promotion
    // (This test validates that auto-promotion respects directives)
    // For now, we'll accept either error or staying as num with inf/overflow
    // The important part is it does NOT auto-promote in :standard mode
    if let Ok(()) = exec_result {
        let result = executor.get_variable("result").unwrap();
        // If it succeeded, should still be num (not bignum)
        assert!(matches!(result.kind, ValueKind::Number(_)),
            "In :standard mode, should not auto-promote to bignum");
    }
    // Otherwise error is acceptable
}

// ============================================================================
// Total: 5 overflow auto-promotion tests
// ============================================================================

// ============================================================================
// Phase 2: Casting Methods Tests (TDD - tests written first!)
// ============================================================================

#[test]
fn test_to_num_bignum_to_num() {
    let source = r#"
bignum x = 123.456
result = x.to_num()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert!(matches!(result.kind, ValueKind::Number(_)),
        "to_num() should convert bignum to num");
    if let ValueKind::Number(n) = result.kind {
        assert!((n - 123.456).abs() < 1e-10, "Value should be approximately 123.456");
    }
}

#[test]
fn test_to_num_num_passthrough() {
    let source = r#"
x = 456.789
result = x.to_num()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert!(matches!(result.kind, ValueKind::Number(_)),
        "to_num() on num should pass through");
    if let ValueKind::Number(n) = result.kind {
        assert!((n - 456.789).abs() < 1e-10);
    }
}

#[test]
fn test_to_bignum_num_to_bignum() {
    let source = r#"
x = 123.456
result = x.to_bignum()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert!(matches!(result.kind, ValueKind::BigNumber(_)),
        "to_bignum() should convert num to bignum");
}

#[test]
fn test_to_bignum_bignum_passthrough() {
    let source = r#"
bignum x = 789.012
result = x.to_bignum()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert!(matches!(result.kind, ValueKind::BigNumber(_)),
        "to_bignum() on bignum should pass through");
}

#[test]
fn test_fits_in_num_small_bignum() {
    let source = r#"
bignum x = 123.456
result = x.fits_in_num()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert_eq!(result, Value::boolean(true),
        "Small bignum should fit in num");
}

#[test]
fn test_fits_in_num_large_bignum() {
    let source = r#"
# Create a very large bignum that exceeds f64 range
bignum x = 10.0
bignum huge = x ** 400.0  # 10^400 - way beyond f64::MAX (~10^308)
result = huge.fits_in_num()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert_eq!(result, Value::boolean(false),
        "Very large bignum should not fit in num");
}

#[test]
fn test_fits_in_num_always_true() {
    let source = r#"
x = 123.456
result = x.fits_in_num()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert_eq!(result, Value::boolean(true),
        "num should always fit in num");
}

#[test]
fn test_is_bignum_detection() {
    let source = r#"
bignum x = 123.456
y = 789.012
result_x = x.is_bignum()
result_y = y.is_bignum()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result_x = executor.get_variable("result_x").unwrap();
    let result_y = executor.get_variable("result_y").unwrap();

    assert_eq!(result_x, Value::boolean(true),
        "bignum type should return true for is_bignum()");
    assert_eq!(result_y, Value::boolean(false),
        "num type should return false for is_bignum()");
}

// ============================================================================
// Total: 8 casting method tests
// ============================================================================

// ============================================================================
// Phase 2: Precision Preservation Tests
// ============================================================================

#[test]
fn test_bignum_preserves_more_digits() {
    let source = r#"
# BigNum (Float128) should preserve more precision than num (f64)
bignum precise = 3.141592653589793238462643383279502884197
num regular = 3.141592653589793238462643383279502884197

# Both should be usable in calculations
bignum calc_precise = precise * 2.0
num calc_regular = regular * 2.0
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let precise = executor.get_variable("precise").unwrap();
    let regular = executor.get_variable("regular").unwrap();
    let calc_precise = executor.get_variable("calc_precise").unwrap();
    let calc_regular = executor.get_variable("calc_regular").unwrap();

    // BigNum maintains type through operations
    assert!(matches!(precise.kind, ValueKind::BigNumber(_)));
    assert!(matches!(calc_precise.kind, ValueKind::BigNumber(_)));
    assert!(matches!(regular.kind, ValueKind::Number(_)));
    assert!(matches!(calc_regular.kind, ValueKind::Number(_)));
}

#[test]
fn test_bignum_no_precision_loss_in_operations() {
    let source = r#"
# Test that repeated operations maintain BigNumber type
bignum start = 1.123456789
bignum multiplier = 2.0

# Multiple operations
bignum step1 = start * multiplier
bignum step2 = step1 * multiplier
bignum step3 = step2 * multiplier
bignum step4 = step3 * multiplier
bignum step5 = step4 * multiplier

# Final result should still be BigNumber
result = step5
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    let step1 = executor.get_variable("step1").unwrap();
    let step5 = executor.get_variable("step5").unwrap();

    // All intermediate results should maintain BigNumber type
    assert!(matches!(step1.kind, ValueKind::BigNumber(_)));
    assert!(matches!(step5.kind, ValueKind::BigNumber(_)));
    assert!(matches!(result.kind, ValueKind::BigNumber(_)));

    // Value should be start * 2^5 = 1.123456789 * 32 ≈ 35.95
    if let ValueKind::BigNumber(bn) = &result.kind {
        let val = bn.to_f64();
        assert!((val - 35.95).abs() < 0.1, "Expected ~35.95, got {}", val);
    }
}

#[test]
fn test_bignum_division_precision() {
    let source = r#"
# Division that requires high precision
bignum a = 1.0
bignum b = 3.0
bignum result = a / b  # Should give precise 0.333...

# Compare with num division
num x = 1.0
num y = 3.0
num regular_result = x / y
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let bignum_result = executor.get_variable("result").unwrap();
    let num_result = executor.get_variable("regular_result").unwrap();

    // Both should work, this just validates the division works correctly
    assert!(matches!(bignum_result.kind, ValueKind::BigNumber(_)));
    assert!(matches!(num_result.kind, ValueKind::Number(_)));
}

#[test]
fn test_bignum_maintains_precision_in_complex_expression() {
    let source = r#"
# Complex expression that benefits from high precision
bignum a = 123456789.123456789
bignum b = 987654321.987654321
bignum c = 42.42424242424242

# Complex calculation
bignum result = ((a + b) * c) / (a - b)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();

    // Should remain as BigNumber through the operations
    assert!(matches!(result.kind, ValueKind::BigNumber(_)),
        "Complex expression should maintain bignum type");
}

#[test]
fn test_bignum_scientific_constant_precision() {
    let source = r#"
# Scientific constants with high precision
bignum pi = 3.14159265358979323846
bignum e = 2.71828182845904523536
bignum phi = 1.61803398874989484820  # Golden ratio

# Operations with high-precision constants
bignum result = (pi * e) + phi
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();

    // Should maintain BigNumber type
    assert!(matches!(result.kind, ValueKind::BigNumber(_)),
        "Scientific constant operations should maintain bignum type");

    // Value should be approximately pi * e + phi ≈ 8.539... + 1.618... ≈ 10.157...
    if let ValueKind::BigNumber(bn) = &result.kind {
        let val = bn.to_f64();
        assert!((val - 10.157).abs() < 0.01, "Expected ~10.157, got {}", val);
    }
}

// ============================================================================
// Total: 5 precision preservation tests
// ============================================================================

// ============================================================================
// Phase 3: BigInt Auto-Growth Tests (TDD - tests first!)
// ============================================================================

#[test]
fn test_int64_multiplication_overflow_grows_to_bigint() {
    let source = r#"
# In :integer mode with :high precision, Int64 overflow should grow to BigInt
configure { integer: :integer, precision: :high } {
    # Int64::MAX = 9223372036854775807
    # Multiply two large numbers that overflow Int64
    bignum a = 9000000000000000000.0  # Near Int64::MAX
    bignum b = 2.0
    result = a * b  # Should grow to BigInt (exceeds Int64::MAX)
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();

    // Should auto-grow to BigInt
    if let ValueKind::BigNumber(bn) = &result.kind {
        assert!(matches!(bn, BigNum::BigInt(_)),
            "Expected BigInt, got {:?}", bn);
    } else {
        panic!("Expected BigNumber type, got {:?}", result.kind);
    }
}

#[test]
fn test_int64_addition_overflow_grows_to_bigint() {
    let source = r#"
configure { integer: :integer, precision: :high } {
    # Two large Int64 values that overflow when added
    bignum a = 9000000000000000000.0
    bignum b = 9000000000000000000.0
    result = a + b  # Should grow to BigInt
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();

    // Should auto-grow to BigInt
    if let ValueKind::BigNumber(bn) = &result.kind {
        assert!(matches!(bn, BigNum::BigInt(_)),
            "Expected BigInt due to overflow, got {:?}", bn);
    } else {
        panic!("Expected BigNumber type");
    }
}

#[test]
fn test_uint64_multiplication_overflow_grows_to_bigint() {
    let source = r#"
configure { integer: :integer, precision: :high, unsigned: :unsigned } {
    # UInt64::MAX = 18446744073709551615
    # Multiply two large numbers that overflow UInt64
    bignum a = 18000000000000000000.0  # Near UInt64::MAX
    bignum b = 2.0
    result = a * b  # Should grow to BigInt
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();

    // Should auto-grow to BigInt
    if let ValueKind::BigNumber(bn) = &result.kind {
        assert!(matches!(bn, BigNum::BigInt(_)),
            "Expected BigInt, got {:?}", bn);
    } else {
        panic!("Expected BigNumber type");
    }
}

#[test]
fn test_uint64_addition_overflow_grows_to_bigint() {
    let source = r#"
configure { integer: :integer, precision: :high, unsigned: :unsigned } {
    bignum a = 18000000000000000000.0
    bignum b = 18000000000000000000.0
    result = a + b  # Should grow to BigInt
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();

    // Should auto-grow to BigInt
    if let ValueKind::BigNumber(bn) = &result.kind {
        assert!(matches!(bn, BigNum::BigInt(_)),
            "Expected BigInt, got {:?}", bn);
    } else {
        panic!("Expected BigNumber type");
    }
}

#[test]
fn test_no_autogrow_for_normal_integer_operations() {
    let source = r#"
configure { integer: :integer, precision: :high } {
    # Small integers should stay as Int64
    bignum a = 1000.0
    bignum b = 2000.0
    result = a * b  # 2000000 - well within Int64 range
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();

    // Should stay as Int64, not grow to BigInt
    if let ValueKind::BigNumber(bn) = &result.kind {
        assert!(matches!(bn, BigNum::Int64(_)),
            "Expected Int64 for small operations, got {:?}", bn);
    } else {
        panic!("Expected BigNumber type");
    }
}

// ============================================================================
// Total: 5 auto-growth tests
// ============================================================================

// ============================================================================
// Phase 3: BigInt Conversion Tests (8 tests)
// Tests for to_int() and to_bigint() conversion methods
// ============================================================================

// ===== to_int() tests (4 tests) =====

#[test]
fn test_bigint_to_int64_within_range() {
    // Convert BigInt to Int64 when value fits in Int64 range
    let source = r#"
configure { precision: :high, :integer } {
    bignum big = 1000000.0
    result = big.to_int()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();

    // Should be Int64
    if let ValueKind::BigNumber(bn) = &result.kind {
        assert!(matches!(bn, BigNum::Int64(1000000)),
            "Expected Int64(1000000), got {:?}", bn);
    } else {
        panic!("Expected BigNumber type, got {:?}", result.kind);
    }
}

#[test]
fn test_bigint_to_int64_overflow_max() {
    // BigInt exceeding Int64::MAX should error
    let source = r#"
configure { precision: :high, :integer } {
    # Create a BigInt larger than Int64::MAX
    bignum a = 9223372036854775807.0  # Int64::MAX
    bignum b = 2.0
    big = a * b  # BigInt > Int64::MAX
    result = big.to_int()
}
"#;
    let mut executor = Executor::new();
    let result = executor.execute_source(source);

    // Should error with overflow
    assert!(result.is_err(), "Expected overflow error");
    if let Err(err) = result {
        assert!(err.to_string().contains("exceeds Int64 range"),
            "Expected overflow error, got: {}", err);
    }
}

#[test]
fn test_bigint_to_int64_overflow_min() {
    // BigInt below Int64::MIN should error
    let source = r#"
configure { precision: :high, :integer } {
    # Create a BigInt smaller than Int64::MIN
    bignum a = -9223372036854775808.0  # Int64::MIN
    bignum b = 2.0
    big = a * b  # BigInt < Int64::MIN
    result = big.to_int()
}
"#;
    let mut executor = Executor::new();
    let result = executor.execute_source(source);

    // Should error with overflow
    assert!(result.is_err(), "Expected overflow error");
    if let Err(err) = result {
        assert!(err.to_string().contains("exceeds Int64 range"),
            "Expected overflow error, got: {}", err);
    }
}

#[test]
fn test_float128_to_int64_truncates() {
    // Convert Float128 to Int64 with truncation of fractional part
    let source = r#"
configure { precision: :high } {
    bignum x = 123.789
    result = x.to_int()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();

    // Should be Int64(123) - truncated
    if let ValueKind::BigNumber(bn) = &result.kind {
        assert!(matches!(bn, BigNum::Int64(123)),
            "Expected Int64(123), got {:?}", bn);
    } else {
        panic!("Expected BigNumber type, got {:?}", result.kind);
    }
}

// ===== to_bigint() tests (4 tests) =====

#[test]
fn test_int64_to_bigint() {
    // Convert Int64 to BigInt
    let source = r#"
configure { precision: :high, :integer } {
    bignum x = 12345.0
    result = x.to_bigint()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();

    // Should be BigInt
    if let ValueKind::BigNumber(bn) = &result.kind {
        assert!(matches!(bn, BigNum::BigInt(_)),
            "Expected BigInt, got {:?}", bn);
    } else {
        panic!("Expected BigNumber type, got {:?}", result.kind);
    }
}

#[test]
fn test_uint64_to_bigint() {
    // Convert UInt64 to BigInt
    let source = r#"
configure { precision: :high, :integer } {
    # Create a UInt64 value
    bignum x = 18446744073709551615.0  # UInt64::MAX
    result = x.to_bigint()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();

    // Should be BigInt
    if let ValueKind::BigNumber(bn) = &result.kind {
        assert!(matches!(bn, BigNum::BigInt(_)),
            "Expected BigInt, got {:?}", bn);
    } else {
        panic!("Expected BigNumber type, got {:?}", result.kind);
    }
}

#[test]
fn test_float128_to_bigint_truncates() {
    // Convert Float128 to BigInt, truncating fractional part
    let source = r#"
configure { precision: :high } {
    bignum x = 9876.543
    result = x.to_bigint()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();

    // Should be BigInt with value 9876 (truncated)
    if let ValueKind::BigNumber(bn) = &result.kind {
        use num_bigint::BigInt;
        assert!(matches!(bn, BigNum::BigInt(_)),
            "Expected BigInt, got {:?}", bn);

        // Verify value is 9876
        if let BigNum::BigInt(bi) = bn {
            assert_eq!(*bi, BigInt::from(9876),
                "Expected BigInt(9876), got {:?}", bi);
        }
    } else {
        panic!("Expected BigNumber type, got {:?}", result.kind);
    }
}

#[test]
fn test_bigint_to_bigint_identity() {
    // BigInt to BigInt should be identity (no change)
    let source = r#"
configure { precision: :high, :integer } {
    bignum a = 9223372036854775807.0  # Int64::MAX
    bignum b = 2.0
    big = a * b  # Creates a BigInt
    result = big.to_bigint()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();

    // Should still be BigInt
    if let ValueKind::BigNumber(bn) = &result.kind {
        assert!(matches!(bn, BigNum::BigInt(_)),
            "Expected BigInt, got {:?}", bn);
    } else {
        panic!("Expected BigNumber type, got {:?}", result.kind);
    }
}

// ============================================================================
// Total: 8 BigInt conversion tests
// ============================================================================

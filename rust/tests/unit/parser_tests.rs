//! Parser unit tests - Following TDD approach

use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
use graphoid::ast::*;

// ============================================================================
// PHASE 1: Literal Parsing Tests
// ============================================================================

#[test]
fn test_parse_number_literal() {
    let mut lexer = Lexer::new("42");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::Literal { value: LiteralValue::Number(n), .. } => {
                    assert_eq!(*n, 42.0);
                }
                _ => panic!("Expected number literal, got {:?}", expr),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_float_literal() {
    let mut lexer = Lexer::new("3.14");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::Literal { value: LiteralValue::Number(n), .. } => {
                    assert_eq!(*n, 3.14);
                }
                _ => panic!("Expected number literal"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_string_literal() {
    let mut lexer = Lexer::new("\"hello\"");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::Literal { value: LiteralValue::String(s), .. } => {
                    assert_eq!(s, "hello");
                }
                _ => panic!("Expected string literal"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_boolean_true() {
    let mut lexer = Lexer::new("true");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::Literal { value: LiteralValue::Boolean(b), .. } => {
                    assert_eq!(*b, true);
                }
                _ => panic!("Expected boolean literal"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_boolean_false() {
    let mut lexer = Lexer::new("false");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::Literal { value: LiteralValue::Boolean(b), .. } => {
                    assert_eq!(*b, false);
                }
                _ => panic!("Expected boolean literal"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_none_literal() {
    let mut lexer = Lexer::new("none");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::Literal { value: LiteralValue::None, .. } => {
                    // Success
                }
                _ => panic!("Expected none literal"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_symbol_literal() {
    let mut lexer = Lexer::new(":ok");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::Literal { value: LiteralValue::Symbol(s), .. } => {
                    assert_eq!(s, "ok");
                }
                _ => panic!("Expected symbol literal"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

// ============================================================================
// PHASE 2: Binary Expression Tests (Arithmetic)
// ============================================================================

#[test]
fn test_parse_addition() {
    let mut lexer = Lexer::new("2 + 3");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::Binary { left, op, right, .. } => {
                    assert_eq!(*op, BinaryOp::Add);
                    // Verify left is 2
                    match **left {
                        Expr::Literal { value: LiteralValue::Number(n), .. } => {
                            assert_eq!(n, 2.0);
                        }
                        _ => panic!("Expected number in left"),
                    }
                    // Verify right is 3
                    match **right {
                        Expr::Literal { value: LiteralValue::Number(n), .. } => {
                            assert_eq!(n, 3.0);
                        }
                        _ => panic!("Expected number in right"),
                    }
                }
                _ => panic!("Expected binary expression"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_operator_precedence() {
    // Should parse as 2 + (3 * 4) due to precedence
    let mut lexer = Lexer::new("2 + 3 * 4");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::Binary { left, op, right, .. } => {
                    assert_eq!(*op, BinaryOp::Add);
                    // Left should be 2
                    match **left {
                        Expr::Literal { value: LiteralValue::Number(n), .. } => assert_eq!(n, 2.0),
                        _ => panic!("Left should be 2"),
                    }
                    // Right should be (3 * 4)
                    match **right {
                        Expr::Binary { ref left, op: BinaryOp::Multiply, ref right, .. } => {
                            match **left {
                                Expr::Literal { value: LiteralValue::Number(n), .. } => assert_eq!(n, 3.0),
                                _ => panic!("Expected 3"),
                            }
                            match **right {
                                Expr::Literal { value: LiteralValue::Number(n), .. } => assert_eq!(n, 4.0),
                                _ => panic!("Expected 4"),
                            }
                        }
                        _ => panic!("Right should be (3 * 4)"),
                    }
                }
                _ => panic!("Expected binary expression"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_integer_division() {
    let mut lexer = Lexer::new("10 // 3");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::Binary { op: BinaryOp::IntDiv, .. } => {
                    // Success
                }
                _ => panic!("Expected integer division expression"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_element_wise_multiply() {
    let mut lexer = Lexer::new("[1,2,3] .* 2");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::Binary { op: BinaryOp::DotMultiply, .. } => {
                    // Success
                }
                _ => panic!("Expected element-wise multiply"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

// ============================================================================
// PHASE 3: Comparison and Logical Operators
// ============================================================================

#[test]
fn test_parse_comparison() {
    let mut lexer = Lexer::new("x > 5");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::Binary { op: BinaryOp::Greater, .. } => {
                    // Success
                }
                _ => panic!("Expected greater than comparison"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_logical_and() {
    let mut lexer = Lexer::new("true and false");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::Binary { op: BinaryOp::And, .. } => {
                    // Success
                }
                _ => panic!("Expected logical and"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

// ============================================================================
// PHASE 4: Variable and Identifier Tests
// ============================================================================

#[test]
fn test_parse_variable_reference() {
    let mut lexer = Lexer::new("x");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::Variable { name, .. } => {
                    assert_eq!(name, "x");
                }
                _ => panic!("Expected variable"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_assignment() {
    let mut lexer = Lexer::new("x = 42");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Assignment { target, value, .. } => {
            match target {
                AssignmentTarget::Variable(name) => {
                    assert_eq!(name, "x");
                }
                _ => panic!("Expected variable target"),
            }
            match value {
                Expr::Literal { value: LiteralValue::Number(n), .. } => {
                    assert_eq!(*n, 42.0);
                }
                _ => panic!("Expected number value"),
            }
        }
        _ => panic!("Expected assignment statement"),
    }
}

#[test]
fn test_parse_variable_declaration_with_type() {
    let mut lexer = Lexer::new("num x = 42");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::VariableDecl { name, type_annotation, .. } => {
            assert_eq!(name, "x");
            assert!(type_annotation.is_some());
            if let Some(type_ann) = type_annotation {
                assert_eq!(type_ann.base_type, "num");
            }
        }
        _ => panic!("Expected variable declaration"),
    }
}

// ============================================================================
// PHASE 5: Collection Literals
// ============================================================================

#[test]
fn test_parse_empty_list() {
    let mut lexer = Lexer::new("[]");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::List { elements, .. } => {
                    assert_eq!(elements.len(), 0);
                }
                _ => panic!("Expected list"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_list_with_elements() {
    let mut lexer = Lexer::new("[1, 2, 3]");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::List { elements, .. } => {
                    assert_eq!(elements.len(), 3);
                }
                _ => panic!("Expected list"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_map() {
    let mut lexer = Lexer::new("{\"key\": 42}");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::Map { entries, .. } => {
                    assert_eq!(entries.len(), 1);
                    assert_eq!(entries[0].0, "key");
                }
                _ => panic!("Expected map"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

// ============================================================================
// PHASE 6: Control Flow Tests
// ============================================================================

#[test]
fn test_parse_if_statement() {
    let source = r#"if x > 0 {
        y = 10
    }"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::If { condition, then_branch, else_branch, .. } => {
            // Verify condition is a comparison
            match condition {
                Expr::Binary { op: BinaryOp::Greater, .. } => {
                    // Success
                }
                _ => panic!("Expected comparison in condition"),
            }
            assert!(then_branch.len() > 0);
            assert!(else_branch.is_none());
        }
        _ => panic!("Expected if statement"),
    }
}

#[test]
fn test_parse_if_else_statement() {
    let source = r#"if x > 0 {
        y = 10
    } else {
        y = 20
    }"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::If { then_branch, else_branch, .. } => {
            assert!(then_branch.len() > 0);
            assert!(else_branch.is_some());
        }
        _ => panic!("Expected if statement"),
    }
}

#[test]
fn test_parse_while_loop() {
    let source = r#"while x > 0 {
        x = x - 1
    }"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::While { condition, body, .. } => {
            match condition {
                Expr::Binary { op: BinaryOp::Greater, .. } => {
                    // Success
                }
                _ => panic!("Expected comparison in condition"),
            }
            assert!(body.len() > 0);
        }
        _ => panic!("Expected while statement"),
    }
}

#[test]
fn test_parse_for_loop() {
    let source = r#"for i in [1, 2, 3] {
        print(i)
    }"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::For { variable, iterable, body, .. } => {
            assert_eq!(variable, "i");
            match iterable {
                Expr::List { .. } => {
                    // Success
                }
                _ => panic!("Expected list in iterable"),
            }
            assert!(body.len() > 0);
        }
        _ => panic!("Expected for statement"),
    }
}

// ============================================================================
// PHASE 7: Function Tests
// ============================================================================

#[test]
fn test_parse_function_declaration() {
    let source = r#"func add(x, y) {
        return x + y
    }"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::FunctionDecl { name, params, body, .. } => {
            assert_eq!(name, "add");
            assert_eq!(params.len(), 2);
            assert_eq!(params[0].name, "x");
            assert_eq!(params[1].name, "y");
            assert!(body.len() > 0);
        }
        _ => panic!("Expected function declaration"),
    }
}

#[test]
fn test_parse_function_call() {
    let mut lexer = Lexer::new("add(1, 2)");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::Call { args, .. } => {
                    assert_eq!(args.len(), 2);
                }
                _ => panic!("Expected function call"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_method_call() {
    let mut lexer = Lexer::new("obj.method(42)");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::MethodCall { method, args, .. } => {
                    assert_eq!(method, "method");
                    assert_eq!(args.len(), 1);
                }
                _ => panic!("Expected method call"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

// ============================================================================
// PHASE 8: Return/Break/Continue
// ============================================================================

#[test]
fn test_parse_return_statement() {
    let mut lexer = Lexer::new("return 42");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Return { value, .. } => {
            assert!(value.is_some());
        }
        _ => panic!("Expected return statement"),
    }
}

#[test]
fn test_parse_break_statement() {
    let mut lexer = Lexer::new("break");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Break { .. } => {
            // Success
        }
        _ => panic!("Expected break statement"),
    }
}

#[test]
fn test_parse_continue_statement() {
    let mut lexer = Lexer::new("continue");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Continue { .. } => {
            // Success
        }
        _ => panic!("Expected continue statement"),
    }
}

// ============================================================================
// PHASE 9: Unary Expressions
// ============================================================================

#[test]
fn test_parse_negation() {
    let mut lexer = Lexer::new("-5");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::Unary { op: UnaryOp::Negate, .. } => {
                    // Success
                }
                _ => panic!("Expected negation"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_not() {
    let mut lexer = Lexer::new("not true");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::Unary { op: UnaryOp::Not, .. } => {
                    // Success
                }
                _ => panic!("Expected not"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

// ============================================================================
// Total: 30+ comprehensive parser tests
// ============================================================================

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
    let source = r#"fn add(x, y) {
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
// LAMBDA PARSING TESTS
// ============================================================================

#[test]
fn test_parse_single_param_lambda() {
    // Parse as assignment since lambdas are typically used in context
    let mut lexer = Lexer::new("f = x => x * 2");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::Assignment { value, .. } => {
            match value {
                Expr::Lambda { params, body, .. } => {
                    assert_eq!(params.len(), 1);
                    assert_eq!(params[0], "x");
                    // Body should be: x * 2
                    match &**body {
                        Expr::Binary { op: BinaryOp::Multiply, .. } => {
                            // Success
                        }
                        _ => panic!("Expected multiplication in lambda body"),
                    }
                }
                _ => panic!("Expected lambda, got {:?}", value),
            }
        }
        _ => panic!("Expected assignment statement"),
    }
}

#[test]
fn test_parse_multi_param_lambda() {
    let mut lexer = Lexer::new("f = (a, b) => a + b");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Assignment { value, .. } => {
            match value {
                Expr::Lambda { params, body, .. } => {
                    assert_eq!(params.len(), 2);
                    assert_eq!(params[0], "a");
                    assert_eq!(params[1], "b");
                    // Body should be: a + b
                    match &**body {
                        Expr::Binary { op: BinaryOp::Add, .. } => {
                            // Success
                        }
                        _ => panic!("Expected addition in lambda body"),
                    }
                }
                _ => panic!("Expected lambda"),
            }
        }
        _ => panic!("Expected assignment statement"),
    }
}

#[test]
fn test_parse_zero_param_lambda() {
    let mut lexer = Lexer::new("f = () => 42");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Assignment { value, .. } => {
            match value {
                Expr::Lambda { params, body, .. } => {
                    assert_eq!(params.len(), 0);
                    // Body should be: 42
                    match &**body {
                        Expr::Literal { value: LiteralValue::Number(n), .. } => {
                            assert_eq!(*n, 42.0);
                        }
                        _ => panic!("Expected number literal in lambda body"),
                    }
                }
                _ => panic!("Expected lambda"),
            }
        }
        _ => panic!("Expected assignment statement"),
    }
}

#[test]
fn test_parse_lambda_in_assignment() {
    let mut lexer = Lexer::new("double = x => x * 2");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Assignment { target, value, .. } => {
            match target {
                AssignmentTarget::Variable(name) => {
                    assert_eq!(name, "double");
                }
                _ => panic!("Expected variable assignment target"),
            }
            match value {
                Expr::Lambda { params, .. } => {
                    assert_eq!(params.len(), 1);
                    assert_eq!(params[0], "x");
                }
                _ => panic!("Expected lambda in assignment value"),
            }
        }
        _ => panic!("Expected assignment statement"),
    }
}

#[test]
fn test_parse_lambda_as_argument() {
    let mut lexer = Lexer::new("numbers.map(x => x * 2)");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::MethodCall { method, args, .. } => {
                    assert_eq!(method, "map");
                    assert_eq!(args.len(), 1);
                    match &args[0] {
                        Argument::Positional(Expr::Lambda { params, .. }) => {
                            assert_eq!(params.len(), 1);
                            assert_eq!(params[0], "x");
                        }
                        _ => panic!("Expected lambda as method argument"),
                    }
                }
                _ => panic!("Expected method call"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

// ============================================================================
// PHASE 9: Configuration and Precision Tests
// ============================================================================

#[test]
fn test_parse_configure_file_level() {
    let mut lexer = Lexer::new("configure { skip_none: true }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::Configure { settings, body, .. } => {
            assert_eq!(settings.len(), 1);
            assert!(settings.contains_key("skip_none"));
            assert!(body.is_none());
        }
        _ => panic!("Expected configure statement"),
    }
}

#[test]
fn test_parse_configure_with_block() {
    let mut lexer = Lexer::new("configure { skip_none: true } { x = 1 }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Configure { settings, body, .. } => {
            assert_eq!(settings.len(), 1);
            assert!(body.is_some());
            let body_stmts = body.as_ref().unwrap();
            assert_eq!(body_stmts.len(), 1);
        }
        _ => panic!("Expected configure statement with body"),
    }
}

#[test]
fn test_parse_configure_multiple_settings() {
    let mut lexer = Lexer::new("configure { skip_none: true, error_mode: :lenient, decimal_places: 2 }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Configure { settings, .. } => {
            assert_eq!(settings.len(), 3);
            assert!(settings.contains_key("skip_none"));
            assert!(settings.contains_key("error_mode"));
            assert!(settings.contains_key("decimal_places"));
        }
        _ => panic!("Expected configure statement"),
    }
}

#[test]
fn test_parse_configure_with_symbol_value() {
    let mut lexer = Lexer::new("configure { error_mode: :strict }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Configure { settings, .. } => {
            assert!(settings.contains_key("error_mode"));
            match settings.get("error_mode").unwrap() {
                Expr::Literal { value: LiteralValue::Symbol(s), .. } => {
                    assert_eq!(s, "strict");
                }
                _ => panic!("Expected symbol value"),
            }
        }
        _ => panic!("Expected configure statement"),
    }
}

#[test]
fn test_parse_configure_with_newlines() {
    let code = r#"configure {
    skip_none: true,
    error_mode: :lenient
}"#;
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Configure { settings, .. } => {
            assert_eq!(settings.len(), 2);
        }
        _ => panic!("Expected configure statement"),
    }
}

#[test]
fn test_parse_precision_with_number() {
    let mut lexer = Lexer::new("precision 2 { x = 1.234 }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::Precision { places, body, .. } => {
            assert_eq!(*places, Some(2));
            assert_eq!(body.len(), 1);
        }
        _ => panic!("Expected precision statement"),
    }
}

#[test]
fn test_parse_precision_with_int_symbol() {
    let mut lexer = Lexer::new("precision :int { x = 1.234 }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Precision { places, .. } => {
            assert_eq!(*places, Some(0)); // :int maps to 0
        }
        _ => panic!("Expected precision statement"),
    }
}

#[test]
fn test_parse_precision_with_zero() {
    let mut lexer = Lexer::new("precision 0 { x = 1.234 }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Precision { places, .. } => {
            assert_eq!(*places, Some(0));
        }
        _ => panic!("Expected precision statement"),
    }
}

#[test]
fn test_parse_precision_with_multiple_statements() {
    let code = r#"precision 3 {
    x = 1.234
    y = 2.567
    z = x + y
}"#;
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Precision { places, body, .. } => {
            assert_eq!(*places, Some(3));
            assert_eq!(body.len(), 3);
        }
        _ => panic!("Expected precision statement"),
    }
}

#[test]
fn test_parse_nested_configure_and_precision() {
    let code = r#"configure { skip_none: true } {
    precision 2 {
        x = 1.234
    }
}"#;
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Configure { body, .. } => {
            let body_stmts = body.as_ref().unwrap();
            assert_eq!(body_stmts.len(), 1);
            match &body_stmts[0] {
                Stmt::Precision { places, .. } => {
                    assert_eq!(*places, Some(2));
                }
                _ => panic!("Expected nested precision statement"),
            }
        }
        _ => panic!("Expected configure statement"),
    }
}

#[test]
fn test_parse_precision_error_negative() {
    let mut lexer = Lexer::new("precision -1 { x = 1 }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
}

#[test]
fn test_parse_precision_error_float() {
    let mut lexer = Lexer::new("precision 2.5 { x = 1 }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
}

// ============================================================================
// PHASE 9: Try/Catch/Finally Parser Tests
// ============================================================================

#[test]
fn test_parse_basic_try_catch() {
    let mut lexer = Lexer::new("try { x = 1 } catch { y = 2 }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::Try { body, catch_clauses, finally_block, .. } => {
            assert_eq!(body.len(), 1);
            assert_eq!(catch_clauses.len(), 1);
            assert!(finally_block.is_none());
            assert_eq!(catch_clauses[0].error_type, None);
            assert_eq!(catch_clauses[0].variable, None);
        }
        _ => panic!("Expected try statement"),
    }
}

#[test]
fn test_parse_try_catch_with_error_type() {
    let mut lexer = Lexer::new("try { x = 1 } catch RuntimeError { y = 2 }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Try { catch_clauses, .. } => {
            assert_eq!(catch_clauses[0].error_type, Some("RuntimeError".to_string()));
            assert_eq!(catch_clauses[0].variable, None);
        }
        _ => panic!("Expected try statement"),
    }
}

#[test]
fn test_parse_try_catch_with_variable_binding() {
    let mut lexer = Lexer::new("try { x = 1 } catch as e { print(e) }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Try { catch_clauses, .. } => {
            assert_eq!(catch_clauses[0].error_type, None);
            assert_eq!(catch_clauses[0].variable, Some("e".to_string()));
        }
        _ => panic!("Expected try statement"),
    }
}

#[test]
fn test_parse_try_catch_with_type_and_variable() {
    let mut lexer = Lexer::new("try { x = 1 } catch TypeError as err { handle(err) }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Try { catch_clauses, .. } => {
            assert_eq!(catch_clauses[0].error_type, Some("TypeError".to_string()));
            assert_eq!(catch_clauses[0].variable, Some("err".to_string()));
        }
        _ => panic!("Expected try statement"),
    }
}

#[test]
fn test_parse_try_with_finally() {
    let mut lexer = Lexer::new("try { x = 1 } finally { cleanup() }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Try { body, catch_clauses, finally_block, .. } => {
            assert_eq!(body.len(), 1);
            assert_eq!(catch_clauses.len(), 0);
            assert!(finally_block.is_some());
            assert_eq!(finally_block.as_ref().unwrap().len(), 1);
        }
        _ => panic!("Expected try statement"),
    }
}

#[test]
fn test_parse_try_catch_finally() {
    let mut lexer = Lexer::new("try { x = 1 } catch { y = 2 } finally { z = 3 }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Try { body, catch_clauses, finally_block, .. } => {
            assert_eq!(body.len(), 1);
            assert_eq!(catch_clauses.len(), 1);
            assert!(finally_block.is_some());
        }
        _ => panic!("Expected try statement"),
    }
}

#[test]
fn test_parse_multiple_catch_clauses() {
    let source = r#"
try {
    risky_operation()
}
catch TypeError as e {
    handle_type_error(e)
}
catch RuntimeError as e {
    handle_runtime_error(e)
}
catch {
    handle_other_error()
}
"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Try { catch_clauses, .. } => {
            assert_eq!(catch_clauses.len(), 3);
            assert_eq!(catch_clauses[0].error_type, Some("TypeError".to_string()));
            assert_eq!(catch_clauses[1].error_type, Some("RuntimeError".to_string()));
            assert_eq!(catch_clauses[2].error_type, None);
        }
        _ => panic!("Expected try statement"),
    }
}

#[test]
fn test_parse_nested_try_catch() {
    let source = r#"
try {
    try {
        inner_operation()
    } catch {
        handle_inner()
    }
} catch {
    handle_outer()
}
"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Try { body, .. } => {
            assert_eq!(body.len(), 1);
            match &body[0] {
                Stmt::Try { .. } => {}, // Inner try statement
                _ => panic!("Expected nested try statement"),
            }
        }
        _ => panic!("Expected try statement"),
    }
}

#[test]
fn test_parse_try_without_catch_or_finally_error() {
    let mut lexer = Lexer::new("try { x = 1 }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
}

#[test]
fn test_parse_try_with_multiple_statements() {
    let source = r#"
try {
    x = 1
    y = 2
    z = x + y
} catch as e {
    print(e)
}
"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Try { body, .. } => {
            assert_eq!(body.len(), 3);
        }
        _ => panic!("Expected try statement"),
    }
}

#[test]
fn test_parse_raise_expression() {
    let mut lexer = Lexer::new("raise \"error message\"");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::Raise { .. } => {}, // Found raise expression
                _ => panic!("Expected raise expression"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_try_catch_with_raise() {
    let source = r#"
try {
    if error_condition {
        raise "Something went wrong"
    }
} catch as e {
    print(e)
}
"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::Try { .. } => {},
        _ => panic!("Expected try statement"),
    }
}

// ============================================================================
// Total: 60 comprehensive parser tests (48 previous + 12 try/catch)
// ============================================================================

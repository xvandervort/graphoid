//! Pattern Matching Parser Tests - Phase 7 TDD (RED phase)
//!
//! These tests are written FIRST before implementation.
//! They will fail initially, then pass as we implement pattern matching.

use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
use graphoid::ast::*;

// ============================================================================
// Pattern Parsing Tests
// ============================================================================

#[test]
fn test_parse_literal_number_pattern() {
    let code = "fn is_zero(n) { |0| => true }";
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::FunctionDecl { name, params, pattern_clauses, .. } => {
            assert_eq!(name, "is_zero");
            assert_eq!(params.len(), 1);
            assert_eq!(params[0].name, "n");

            // Check pattern_clauses exist
            assert!(pattern_clauses.is_some());
            let clauses = pattern_clauses.as_ref().unwrap();
            assert_eq!(clauses.len(), 1);

            // Check first clause
            match &clauses[0].pattern {
                Pattern::Literal { value, .. } => {
                    match value {
                        LiteralValue::Number(n) => assert_eq!(*n, 0.0),
                        _ => panic!("Expected number literal pattern"),
                    }
                }
                _ => panic!("Expected literal pattern"),
            }

            // Check body is true
            match &clauses[0].body {
                Expr::Literal { value: LiteralValue::Boolean(b), .. } => {
                    assert_eq!(*b, true);
                }
                _ => panic!("Expected boolean true in body"),
            }
        }
        _ => panic!("Expected function declaration"),
    }
}

#[test]
fn test_parse_literal_string_pattern() {
    let code = r#"fn get_sound(animal) { |"dog"| => "woof" }"#;
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::FunctionDecl { pattern_clauses, .. } => {
            let clauses = pattern_clauses.as_ref().unwrap();
            match &clauses[0].pattern {
                Pattern::Literal { value, .. } => {
                    match value {
                        LiteralValue::String(s) => assert_eq!(s, "dog"),
                        _ => panic!("Expected string literal pattern"),
                    }
                }
                _ => panic!("Expected literal pattern"),
            }
        }
        _ => panic!("Expected function declaration"),
    }
}

#[test]
fn test_parse_variable_pattern() {
    let code = "fn double(x) { |n| => n * 2 }";
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::FunctionDecl { pattern_clauses, .. } => {
            let clauses = pattern_clauses.as_ref().unwrap();
            assert_eq!(clauses.len(), 1);

            match &clauses[0].pattern {
                Pattern::Variable { name, .. } => {
                    assert_eq!(name, "n");
                }
                _ => panic!("Expected variable pattern"),
            }

            // Body should be n * 2
            match &clauses[0].body {
                Expr::Binary { left, op, right, .. } => {
                    assert_eq!(*op, BinaryOp::Multiply);
                    match &**left {
                        Expr::Variable { name, .. } => assert_eq!(name, "n"),
                        _ => panic!("Expected variable 'n'"),
                    }
                    match &**right {
                        Expr::Literal { value: LiteralValue::Number(n), .. } => {
                            assert_eq!(*n, 2.0);
                        }
                        _ => panic!("Expected number 2"),
                    }
                }
                _ => panic!("Expected binary multiplication"),
            }
        }
        _ => panic!("Expected function declaration"),
    }
}

#[test]
fn test_parse_wildcard_pattern() {
    let code = "fn always_match(x) { |_| => \"matched\" }";
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::FunctionDecl { pattern_clauses, .. } => {
            let clauses = pattern_clauses.as_ref().unwrap();
            match &clauses[0].pattern {
                Pattern::Wildcard { .. } => {
                    // Success!
                }
                _ => panic!("Expected wildcard pattern"),
            }
        }
        _ => panic!("Expected function declaration"),
    }
}

#[test]
fn test_parse_multiple_pattern_clauses() {
    let code = r#"
        fn factorial(n) {
            |0| => 1
            |1| => 1
            |x| => x * factorial(x - 1)
        }
    "#;
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::FunctionDecl { name, pattern_clauses, body, .. } => {
            assert_eq!(name, "factorial");

            // Pattern matching function should have pattern_clauses
            assert!(pattern_clauses.is_some());
            let clauses = pattern_clauses.as_ref().unwrap();
            assert_eq!(clauses.len(), 3);

            // Body should be empty for pattern matching functions
            assert!(body.is_empty());

            // Check first clause: |0| => 1
            match &clauses[0].pattern {
                Pattern::Literal { value, .. } => {
                    match value {
                        LiteralValue::Number(n) => assert_eq!(*n, 0.0),
                        _ => panic!("Expected number 0"),
                    }
                }
                _ => panic!("Expected literal pattern"),
            }

            // Check second clause: |1| => 1
            match &clauses[1].pattern {
                Pattern::Literal { value, .. } => {
                    match value {
                        LiteralValue::Number(n) => assert_eq!(*n, 1.0),
                        _ => panic!("Expected number 1"),
                    }
                }
                _ => panic!("Expected literal pattern"),
            }

            // Check third clause: |x| => x * factorial(x - 1)
            match &clauses[2].pattern {
                Pattern::Variable { name, .. } => {
                    assert_eq!(name, "x");
                }
                _ => panic!("Expected variable pattern"),
            }
        }
        _ => panic!("Expected function declaration"),
    }
}

#[test]
fn test_parse_boolean_pattern() {
    let code = r#"
        fn bool_to_string(b) {
            |true| => "yes"
            |false| => "no"
        }
    "#;
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::FunctionDecl { pattern_clauses, .. } => {
            let clauses = pattern_clauses.as_ref().unwrap();
            assert_eq!(clauses.len(), 2);

            // First: |true|
            match &clauses[0].pattern {
                Pattern::Literal { value, .. } => {
                    match value {
                        LiteralValue::Boolean(b) => assert_eq!(*b, true),
                        _ => panic!("Expected true"),
                    }
                }
                _ => panic!("Expected literal pattern"),
            }

            // Second: |false|
            match &clauses[1].pattern {
                Pattern::Literal { value, .. } => {
                    match value {
                        LiteralValue::Boolean(b) => assert_eq!(*b, false),
                        _ => panic!("Expected false"),
                    }
                }
                _ => panic!("Expected literal pattern"),
            }
        }
        _ => panic!("Expected function declaration"),
    }
}

#[test]
fn test_parse_none_pattern() {
    let code = r#"
        fn handle_optional(val) {
            |none| => "nothing"
            |x| => "something"
        }
    "#;
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::FunctionDecl { pattern_clauses, .. } => {
            let clauses = pattern_clauses.as_ref().unwrap();

            // First: |none|
            match &clauses[0].pattern {
                Pattern::Literal { value, .. } => {
                    match value {
                        LiteralValue::None => {
                            // Success!
                        }
                        _ => panic!("Expected none"),
                    }
                }
                _ => panic!("Expected literal pattern"),
            }
        }
        _ => panic!("Expected function declaration"),
    }
}

#[test]
fn test_parse_mixed_patterns() {
    let code = r#"
        fn classify(x) {
            |0| => "zero"
            |"empty"| => "string"
            |true| => "boolean"
            |none| => "none"
            |_| => "other"
        }
    "#;
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::FunctionDecl { pattern_clauses, .. } => {
            let clauses = pattern_clauses.as_ref().unwrap();
            assert_eq!(clauses.len(), 5);

            // Verify we have different pattern types
            assert!(matches!(&clauses[0].pattern, Pattern::Literal { value: LiteralValue::Number(_), .. }));
            assert!(matches!(&clauses[1].pattern, Pattern::Literal { value: LiteralValue::String(_), .. }));
            assert!(matches!(&clauses[2].pattern, Pattern::Literal { value: LiteralValue::Boolean(_), .. }));
            assert!(matches!(&clauses[3].pattern, Pattern::Literal { value: LiteralValue::None, .. }));
            assert!(matches!(&clauses[4].pattern, Pattern::Wildcard { .. }));
        }
        _ => panic!("Expected function declaration"),
    }
}

#[test]
fn test_parse_regular_function_without_patterns() {
    let code = "fn add(a, b) { return a + b }";
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::FunctionDecl { pattern_clauses, body, .. } => {
            // Regular function should have no pattern_clauses
            assert!(pattern_clauses.is_none());

            // Regular function should have a body
            assert!(!body.is_empty());
        }
        _ => panic!("Expected function declaration"),
    }
}

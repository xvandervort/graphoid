use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
use graphoid::ast::{Stmt, Expr, LiteralValue};

#[test]
fn test_parse_load_statement() {
    let source = r#"load "config.gr""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::Load { path, .. } => {
            // path is now an Expr, check it's a string literal
            match path {
                Expr::Literal { value, .. } => {
                    assert_eq!(*value, LiteralValue::String("config.gr".to_string()));
                }
                _ => panic!("Expected string literal path, got {:?}", path),
            }
        }
        _ => panic!("Expected Load statement, got {:?}", program.statements[0]),
    }
}

#[test]
fn test_parse_load_with_relative_path() {
    let source = r#"load "./utils/helpers.gr""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Load { path, .. } => {
            match path {
                Expr::Literal { value, .. } => {
                    assert_eq!(*value, LiteralValue::String("./utils/helpers.gr".to_string()));
                }
                _ => panic!("Expected string literal path, got {:?}", path),
            }
        }
        _ => panic!("Expected Load statement"),
    }
}

#[test]
fn test_load_with_variable() {
    // load now accepts expressions, including variables
    let source = "load config";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::Load { path, .. } => {
            match path {
                Expr::Variable { name, .. } => {
                    assert_eq!(name, "config");
                }
                _ => panic!("Expected variable, got {:?}", path),
            }
        }
        _ => panic!("Expected Load statement"),
    }
}

#[test]
fn test_multiple_statements_with_load() {
    let source = r#"
        load "config.gr"
        x = 10
        load "utils.gr"
    "#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 3, "Expected 3 statements, got {}", program.statements.len());
    assert!(matches!(program.statements[0], Stmt::Load { .. }), "Statement 0: {:?}", program.statements[0]);
    assert!(matches!(program.statements[1], Stmt::VariableDecl { .. }) || matches!(program.statements[1], Stmt::Assignment { .. }),
        "Statement 1: {:?}", program.statements[1]);
    assert!(matches!(program.statements[2], Stmt::Load { .. }), "Statement 2: {:?}", program.statements[2]);
}

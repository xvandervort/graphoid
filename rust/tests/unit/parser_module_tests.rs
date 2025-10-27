use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
use graphoid::ast::Stmt;

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
            assert_eq!(path, "config.gr");
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
            assert_eq!(path, "./utils/helpers.gr");
        }
        _ => panic!("Expected Load statement"),
    }
}

#[test]
fn test_load_requires_string() {
    let source = "load config";  // Missing quotes
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());  // Should fail to parse
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

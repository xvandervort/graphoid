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

// =============================================================================
// Phase 17: Selective import parser tests
// =============================================================================

fn parse(source: &str) -> graphoid::ast::Program {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    parser.parse().unwrap()
}

fn parse_err(source: &str) -> String {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    parser.parse().unwrap_err().to_string()
}

#[test]
fn test_parse_selective_import_single() {
    let program = parse(r#"import "math" { sin }"#);
    match &program.statements[0] {
        Stmt::Import { module, alias, selections, .. } => {
            assert_eq!(module, "math");
            assert!(alias.is_none());
            let items = selections.as_ref().unwrap();
            assert_eq!(items.len(), 1);
            assert_eq!(items[0].name, "sin");
            assert!(items[0].alias.is_none());
        }
        _ => panic!("Expected Import statement"),
    }
}

#[test]
fn test_parse_selective_import_multiple() {
    let program = parse(r#"import "math" { sin, cos, sqrt }"#);
    match &program.statements[0] {
        Stmt::Import { selections, .. } => {
            let items = selections.as_ref().unwrap();
            assert_eq!(items.len(), 3);
            assert_eq!(items[0].name, "sin");
            assert_eq!(items[1].name, "cos");
            assert_eq!(items[2].name, "sqrt");
        }
        _ => panic!("Expected Import statement"),
    }
}

#[test]
fn test_parse_selective_import_with_alias() {
    let program = parse(r#"import "math" { sin as sine }"#);
    match &program.statements[0] {
        Stmt::Import { selections, .. } => {
            let items = selections.as_ref().unwrap();
            assert_eq!(items.len(), 1);
            assert_eq!(items[0].name, "sin");
            assert_eq!(items[0].alias, Some("sine".to_string()));
        }
        _ => panic!("Expected Import statement"),
    }
}

#[test]
fn test_parse_selective_import_mixed_aliases() {
    let program = parse(r#"import "math" { sin as sine, cos, sqrt as square_root }"#);
    match &program.statements[0] {
        Stmt::Import { selections, .. } => {
            let items = selections.as_ref().unwrap();
            assert_eq!(items.len(), 3);
            assert_eq!(items[0].name, "sin");
            assert_eq!(items[0].alias, Some("sine".to_string()));
            assert_eq!(items[1].name, "cos");
            assert!(items[1].alias.is_none());
            assert_eq!(items[2].name, "sqrt");
            assert_eq!(items[2].alias, Some("square_root".to_string()));
        }
        _ => panic!("Expected Import statement"),
    }
}

#[test]
fn test_parse_import_unchanged_full() {
    let program = parse(r#"import "math""#);
    match &program.statements[0] {
        Stmt::Import { module, alias, selections, .. } => {
            assert_eq!(module, "math");
            assert!(alias.is_none());
            assert!(selections.is_none());
        }
        _ => panic!("Expected Import statement"),
    }
}

#[test]
fn test_parse_import_unchanged_with_alias() {
    let program = parse(r#"import "math" as m"#);
    match &program.statements[0] {
        Stmt::Import { module, alias, selections, .. } => {
            assert_eq!(module, "math");
            assert_eq!(alias.as_deref(), Some("m"));
            assert!(selections.is_none());
        }
        _ => panic!("Expected Import statement"),
    }
}

#[test]
fn test_parse_import_alias_and_selective_error() {
    let err = parse_err(r#"import "math" as m { sin }"#);
    assert!(err.contains("Cannot use both"), "Error: {}", err);
}

#[test]
fn test_parse_import_empty_selections_error() {
    let err = parse_err(r#"import "math" { }"#);
    assert!(err.contains("Empty selective import"), "Error: {}", err);
}

#[test]
fn test_parse_selective_import_trailing_comma() {
    // Trailing comma should be fine (last item parsed, comma consumed, then } breaks)
    let program = parse(r#"import "math" { sin, cos, }"#);
    match &program.statements[0] {
        Stmt::Import { selections, .. } => {
            let items = selections.as_ref().unwrap();
            assert_eq!(items.len(), 2); // sin and cos parsed, trailing comma OK
        }
        _ => panic!("Expected Import statement"),
    }
}

// =============================================================================
// Phase 17: priv { } block parser tests
// =============================================================================

#[test]
fn test_parse_priv_block_with_function() {
    let program = parse(r#"
        priv {
            fn helper() {
                return 42
            }
        }
    "#);
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::PrivBlock { body, .. } => {
            assert_eq!(body.len(), 1);
            match &body[0] {
                Stmt::FunctionDecl { name, is_private, .. } => {
                    assert_eq!(name, "helper");
                    assert!(*is_private, "Function in priv block should be private");
                }
                _ => panic!("Expected FunctionDecl, got {:?}", body[0]),
            }
        }
        _ => panic!("Expected PrivBlock, got {:?}", program.statements[0]),
    }
}

#[test]
fn test_parse_priv_block_with_variable() {
    let program = parse(r#"
        priv {
            secret = 42
        }
    "#);
    match &program.statements[0] {
        Stmt::PrivBlock { body, .. } => {
            assert_eq!(body.len(), 1);
            // Variable assignments don't have is_private — they're handled by the
            // priv_variable_declaration_without_type path. In a priv block,
            // an assignment like `secret = 42` becomes an Assignment or VariableDecl.
        }
        _ => panic!("Expected PrivBlock"),
    }
}

#[test]
fn test_parse_priv_block_multiple_items() {
    let program = parse(r#"
        priv {
            fn helper1() {
                return 1
            }
            fn helper2() {
                return 2
            }
        }
    "#);
    match &program.statements[0] {
        Stmt::PrivBlock { body, .. } => {
            assert_eq!(body.len(), 2);
            match &body[0] {
                Stmt::FunctionDecl { name, is_private, .. } => {
                    assert_eq!(name, "helper1");
                    assert!(*is_private);
                }
                _ => panic!("Expected FunctionDecl"),
            }
            match &body[1] {
                Stmt::FunctionDecl { name, is_private, .. } => {
                    assert_eq!(name, "helper2");
                    assert!(*is_private);
                }
                _ => panic!("Expected FunctionDecl"),
            }
        }
        _ => panic!("Expected PrivBlock"),
    }
}

#[test]
fn test_parse_existing_priv_fn_unchanged() {
    // Existing `priv fn` syntax should still work
    let program = parse(r#"
        priv fn helper() {
            return 42
        }
    "#);
    match &program.statements[0] {
        Stmt::FunctionDecl { name, is_private, .. } => {
            assert_eq!(name, "helper");
            assert!(*is_private);
        }
        _ => panic!("Expected FunctionDecl, got {:?}", program.statements[0]),
    }
}

#[test]
fn test_parse_existing_priv_variable_unchanged() {
    // Existing `priv x = 1` syntax should still work
    let program = parse("priv SECRET = 42");
    match &program.statements[0] {
        Stmt::VariableDecl { name, is_private, .. } => {
            assert_eq!(name, "SECRET");
            assert!(*is_private);
        }
        _ => panic!("Expected VariableDecl, got {:?}", program.statements[0]),
    }
}

//! Integration tests for end-to-end execution

use graphoid::execution::Executor;
use graphoid::lexer::Lexer;
use graphoid::parser::Parser;

/// Helper function to execute source code
fn execute(source: &str) -> Result<(), String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer
        .tokenize()
        .map_err(|e| format!("Lexer error: {}", e))?;

    let mut parser = Parser::new(tokens);
    let program = parser
        .parse()
        .map_err(|e| format!("Parser error: {}", e))?;

    let mut executor = Executor::new();
    for stmt in &program.statements {
        executor
            .eval_stmt(stmt)
            .map_err(|e| format!("Runtime error: {}", e))?;
    }

    Ok(())
}

/// Helper to execute and return variable value
fn execute_and_get(source: &str, var_name: &str) -> Result<graphoid::values::Value, String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer
        .tokenize()
        .map_err(|e| format!("Lexer error: {}", e))?;

    let mut parser = Parser::new(tokens);
    let program = parser
        .parse()
        .map_err(|e| format!("Parser error: {}", e))?;

    let mut executor = Executor::new();
    for stmt in &program.statements {
        executor
            .eval_stmt(stmt)
            .map_err(|e| format!("Runtime error: {}", e))?;
    }

    executor
        .env()
        .get(var_name)
        .map_err(|e| format!("Variable not found: {}", e))
}

#[test]
fn test_simple_arithmetic() {
    let source = r#"
        x = 2 + 3
        y = x * 2
    "#;

    execute(source).unwrap();
}

#[test]
fn test_string_operations() {
    let source = r#"
        greeting = "Hello"
        name = "World"
        message = greeting + " " + name
    "#;

    let result = execute_and_get(source, "message").unwrap();
    assert_eq!(result, graphoid::values::Value::String("Hello World".to_string()));
}

#[test]
fn test_variable_shadowing() {
    let source = r#"
        x = 10
        x = x + 5
        x = x * 2
    "#;

    let result = execute_and_get(source, "x").unwrap();
    assert_eq!(result, graphoid::values::Value::Number(30.0));
}

#[test]
fn test_collections() {
    let source = r#"
        numbers = [1, 2, 3]
        config = {"host": "localhost", "port": 8080}
    "#;

    execute(source).unwrap();
}

#[test]
fn test_boolean_logic() {
    let source = r#"
        a = true and false
        b = true or false
        c = not false
    "#;

    let a = execute_and_get(source, "a").unwrap();
    let b = execute_and_get(source, "b").unwrap();
    let c = execute_and_get(source, "c").unwrap();

    assert_eq!(a, graphoid::values::Value::Boolean(false));
    assert_eq!(b, graphoid::values::Value::Boolean(true));
    assert_eq!(c, graphoid::values::Value::Boolean(true));
}

#[test]
fn test_comparisons() {
    let source = r#"
        result1 = 5 > 3
        result2 = 10 == 10
        result3 = "apple" < "banana"
    "#;

    let r1 = execute_and_get(source, "result1").unwrap();
    let r2 = execute_and_get(source, "result2").unwrap();
    let r3 = execute_and_get(source, "result3").unwrap();

    assert_eq!(r1, graphoid::values::Value::Boolean(true));
    assert_eq!(r2, graphoid::values::Value::Boolean(true));
    assert_eq!(r3, graphoid::values::Value::Boolean(true));
}

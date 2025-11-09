//! Tests for rest patterns in match expressions

use graphoid::execution::Executor;
use graphoid::lexer::Lexer;
use graphoid::parser::Parser;

fn eval_source(source: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().map_err(|e| e.to_string())?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse().map_err(|e| e.to_string())?;
    
    let mut executor = Executor::new();
    for stmt in &program.statements {
        executor.eval_stmt(stmt).map_err(|e| e.to_string())?;
    }
    
    Ok("success".to_string())
}

fn eval_expr(source: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().map_err(|e| e.to_string())?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse().map_err(|e| e.to_string())?;
    
    let mut executor = Executor::new();
    let mut result = String::new();
    
    for stmt in &program.statements {
        if let graphoid::ast::Stmt::Expression { expr, .. } = stmt {
            let value = executor.eval_expr(expr).map_err(|e| e.to_string())?;
            result = value.to_string();
        } else {
            executor.eval_stmt(stmt).map_err(|e| e.to_string())?;
        }
    }
    
    Ok(result)
}

#[test]
fn test_rest_pattern_basic() {
    let source = r#"
        fn test(lst) {
            return match lst { [x, ...rest] => x, _ => none }
        }
        test([1, 2, 3])
    "#;
    
    let result = eval_expr(source).unwrap();
    assert_eq!(result, "1");
}

#[test]
fn test_rest_pattern_empty_list() {
    let source = r#"
        fn test(lst) {
            return match lst { [x, ...rest] => x, _ => none }
        }
        test([])
    "#;
    
    let result = eval_expr(source).unwrap();
    assert_eq!(result, "none");
}

#[test]
fn test_rest_pattern_single_element() {
    let source = r#"
        fn test(lst) {
            return match lst { [x, ...rest] => x, _ => none }
        }
        test([42])
    "#;
    
    let result = eval_expr(source).unwrap();
    assert_eq!(result, "42");
}

#[test]
fn test_rest_pattern_binding_returns_list() {
    let source = r#"
        fn get_tail(lst) {
            return match lst { [head, ...tail] => tail, _ => [] }
        }
        get_tail([1, 2, 3, 4])
    "#;
    
    let result = eval_expr(source).unwrap();
    assert_eq!(result, "[2, 3, 4]");
}

#[test]
fn test_rest_pattern_all_elements() {
    let source = r#"
        fn test(lst) {
            return match lst { [...all] => all, _ => [] }
        }
        test([1, 2, 3])
    "#;
    
    let result = eval_expr(source).unwrap();
    assert_eq!(result, "[1, 2, 3]");
}

#[test]
fn test_rest_pattern_anonymous() {
    let source = r#"
        fn test(lst) {
            return match lst { [x, ...] => x, _ => none }
        }
        test([5, 6, 7])
    "#;
    
    let result = eval_expr(source).unwrap();
    assert_eq!(result, "5");
}

#[test]
fn test_rest_pattern_multiple_fixed() {
    let source = r#"
        fn test(lst) {
            return match lst { [a, b, ...rest] => a, _ => none }
        }
        test([10, 20, 30, 40])
    "#;
    
    let result = eval_expr(source).unwrap();
    assert_eq!(result, "10");
}

#[test]
fn test_rest_pattern_empty_rest() {
    let source = r#"
        fn test(lst) {
            return match lst { [x, ...rest] => rest, _ => [] }
        }
        test([99])
    "#;
    
    let result = eval_expr(source).unwrap();
    assert_eq!(result, "[]");
}

#[test]
fn test_rest_pattern_with_nested_patterns() {
    let source = r#"
        fn test(lst) {
            return match lst { [1, ...rest] => "starts with one", [2, ...rest] => "starts with two", _ => "other" }
        }
        test([1, 5, 6])
    "#;
    
    let result = eval_expr(source).unwrap();
    assert_eq!(result, "starts with one");
}

#[test]
fn test_rest_pattern_priority() {
    let source = r#"
        fn test(lst) {
            return match lst { [] => "empty", [x] => "single", [x, y] => "pair", [x, ...rest] => "multiple" }
        }
        test([1, 2, 3])
    "#;

    let result = eval_expr(source).unwrap();
    assert_eq!(result, "multiple");
}

#[test]
fn test_rest_pattern_vs_exact_match() {
    let source = r#"
        fn test(lst) {
            return match lst { [x, y] => "exactly two", [x, ...rest] => "one or more" }
        }
        test([1, 2])
    "#;

    let result = eval_expr(source).unwrap();
    assert_eq!(result, "exactly two");
}

// ============================================================================
// Multiline Match Expression Tests
// ============================================================================

#[test]
fn test_multiline_match_basic() {
    let source = r#"
        fn test(x) {
            return match x {
                0 => "zero"
                1 => "one"
                2 => "two"
                _ => "other"
            }
        }
        test(1)
    "#;

    let result = eval_expr(source).unwrap();
    assert_eq!(result, "one");
}

#[test]
fn test_multiline_match_with_lists() {
    let source = r#"
        fn test(lst) {
            return match lst {
                [] => "empty"
                [x] => "single"
                [x, y] => "pair"
                _ => "multiple"
            }
        }
        test([1, 2])
    "#;

    let result = eval_expr(source).unwrap();
    assert_eq!(result, "pair");
}

#[test]
fn test_multiline_match_with_rest_patterns() {
    let source = r#"
        fn test(lst) {
            return match lst {
                [] => "empty"
                [x, ...rest] => "at least one"
                _ => "not a list"
            }
        }
        test([1, 2, 3])
    "#;

    let result = eval_expr(source).unwrap();
    assert_eq!(result, "at least one");
}

#[test]
fn test_multiline_match_mixed_separators() {
    let source = r#"
        fn test(x) {
            return match x {
                0 => "zero"
                1 => "one",
                2 => "two"
                _ => "other"
            }
        }
        test(2)
    "#;

    let result = eval_expr(source).unwrap();
    assert_eq!(result, "two");
}

#[test]
fn test_multiline_match_trailing_comma() {
    let source = r#"
        fn test(x) {
            return match x {
                0 => "zero",
                1 => "one",
                _ => "other",
            }
        }
        test(0)
    "#;

    let result = eval_expr(source).unwrap();
    assert_eq!(result, "zero");
}

//! Integration Tests for Missing Freeze Methods - Phase 8 (TDD RED phase)
//!
//! These tests verify the 3 missing freeze methods from the spec:
//! 1. freeze!() - In-place freeze (mutating method)
//! 2. has_frozen() - Check for frozen elements (boolean)
//! 3. has_frozen(:count) - Detailed frozen element info

use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
use graphoid::execution::Executor;
use graphoid::values::Value;
use graphoid::ast::Stmt;

/// Helper to execute code and return the value of the last expression
fn execute_and_return(code: &str) -> Result<Value, String> {
    let mut lexer = Lexer::new(code);
    let tokens = lexer
        .tokenize()
        .map_err(|e| format!("Lexer error: {}", e))?;

    let mut parser = Parser::new(tokens);
    let program = parser
        .parse()
        .map_err(|e| format!("Parser error: {}", e))?;

    let mut executor = Executor::new();

    // Execute all statements except the last
    let statements = &program.statements;
    for stmt in statements.iter().take(statements.len().saturating_sub(1)) {
        executor
            .eval_stmt(stmt)
            .map_err(|e| format!("Runtime error: {}", e))?;
    }

    // Execute the last statement and return its value
    if let Some(last_stmt) = statements.last() {
        match last_stmt {
            Stmt::Expression { expr, .. } => {
                executor
                    .eval_expr(&expr)
                    .map_err(|e| format!("Runtime error: {}", e))
            }
            _ => {
                executor
                    .eval_stmt(last_stmt)
                    .map_err(|e| format!("Runtime error: {}", e))?;
                Ok(Value::none())
            }
        }
    } else {
        Ok(Value::none())
    }
}

// ============================================================================
// freeze!() Tests - In-place freezing
// ============================================================================

#[test]
fn test_freeze_bang_freezes_list_in_place() {
    let code = r#"
        items = [1, 2, 3]
        items.freeze!()
        items.is_frozen()
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::boolean(true));
}

#[test]
fn test_freeze_bang_returns_none() {
    let code = r#"
        items = [1, 2, 3]
        items.freeze!()
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::none());
}

#[test]
fn test_freeze_bang_prevents_mutation() {
    let code = r#"
        items = [1, 2, 3]
        items.freeze!()
        items.append!(4)  # Should error - frozen collection
    "#;

    let result = execute_and_return(code);
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("frozen") || err_msg.contains("immutable"));
}

#[test]
fn test_freeze_bang_deep_freeze_by_default() {
    let code = r#"
        items = [[1, 2], [3, 4]]
        items.freeze!()
        inner = items[0]
        inner.is_frozen()
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::boolean(true));
}

#[test]
fn test_freeze_bang_on_hash() {
    let code = r#"
        config = {"host": "localhost", "port": 8080}
        config.freeze!()
        config.is_frozen()
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::boolean(true));
}

// ============================================================================
// has_frozen() Tests - Boolean query for frozen elements
// ============================================================================

#[test]
fn test_has_frozen_returns_false_for_all_unfrozen() {
    let code = r#"
        items = [1, 2, 3]
        items.has_frozen()
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::boolean(false));
}

#[test]
fn test_has_frozen_returns_true_with_frozen_element() {
    let code = r#"
        frozen = [1, 2].freeze()
        items = [frozen, [3, 4]]
        items.has_frozen()
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::boolean(true));
}

#[test]
fn test_has_frozen_returns_false_for_empty_list() {
    let code = r#"
        items = []
        items.has_frozen()
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::boolean(false));
}

#[test]
fn test_has_frozen_checks_nested_elements() {
    let code = r#"
        inner = [1, 2].freeze()
        middle = [inner, 3]
        outer = [middle, 4]
        outer.has_frozen()
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::boolean(true));
}

#[test]
fn test_has_frozen_on_hash() {
    let code = r#"
        frozen_val = [1, 2].freeze()
        items = {"key1": frozen_val, "key2": [3, 4]}
        items.has_frozen()
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::boolean(true));
}

// ============================================================================
// has_frozen(:count) Tests - Detailed frozen info
// ============================================================================

#[test]
fn test_has_frozen_verbose_returns_hash() {
    let code = r#"
        items = [1, 2, 3]
        info = items.has_frozen(:count)
        # Should return a hash
        info["has_frozen"]
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::boolean(false));
}

#[test]
fn test_has_frozen_verbose_counts_frozen_elements() {
    let code = r#"
        frozen1 = [1, 2].freeze()
        frozen2 = [3, 4].freeze()
        items = [frozen1, frozen2, [5, 6]]
        info = items.has_frozen(:count)
        info["frozen_count"]
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::number(2.0));
}

#[test]
fn test_has_frozen_verbose_counts_collections_vs_primitives() {
    let code = r#"
        frozen_list = [1, 2].freeze()
        frozen_num = 42
        # Note: Can't freeze primitives directly in current implementation
        # Just test frozen collections for now
        items = [frozen_list, [3, 4]]
        info = items.has_frozen(:count)
        info["frozen_collections"]
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::number(1.0));
}

#[test]
fn test_has_frozen_verbose_with_no_frozen() {
    let code = r#"
        items = [1, 2, 3]
        info = items.has_frozen(:count)
        info["has_frozen"]
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::boolean(false));

    // Also check frozen_count
    let code2 = r#"
        items = [1, 2, 3]
        info = items.has_frozen(:count)
        info["frozen_count"]
    "#;

    let result2 = execute_and_return(code2).unwrap();
    assert_eq!(result2, Value::number(0.0));
}

#[test]
fn test_has_frozen_count_deep_mode() {
    // Test deep recursive counting - shallow mode
    let code_shallow = r#"
        # Create nested structure with frozen elements at different depths
        inner1 = [1, 2].freeze()
        inner2 = [3, 4].freeze()
        middle = [inner1, inner2]
        middle_frozen = middle.freeze()
        outer = [middle_frozen, [5, 6]]

        # Shallow count: only counts middle_frozen (1 collection)
        shallow_info = outer.has_frozen(:count)
        shallow_info["frozen_collections"]
    "#;

    let result_shallow = execute_and_return(code_shallow).unwrap();
    assert_eq!(result_shallow, Value::number(1.0), "Shallow should count 1 collection (middle_frozen)");

    // Test deep recursive counting - deep mode
    let code_deep = r#"
        # Create same nested structure
        inner1 = [1, 2].freeze()
        inner2 = [3, 4].freeze()
        middle = [inner1, inner2]
        middle_frozen = middle.freeze()
        outer = [middle_frozen, [5, 6]]

        # Deep count: counts middle_frozen + inner1 + inner2 (3 collections total)
        deep_info = outer.has_frozen(:count, :deep)
        deep_info["frozen_collections"]
    "#;

    let result_deep = execute_and_return(code_deep).unwrap();
    assert_eq!(result_deep, Value::number(3.0), "Deep should count 3 collections (middle_frozen + inner1 + inner2)");
}

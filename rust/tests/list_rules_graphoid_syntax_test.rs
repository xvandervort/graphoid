//! Integration test for Graphoid language syntax: list.add_rule(:no_dups)
//!
//! This tests the END-TO-END use case with Graphoid code, not just Rust API.

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

#[test]
fn test_list_add_rule_no_dups_graphoid_syntax() {
    // Create a list and add no_dups rule using Graphoid syntax
    let code = r#"
        items = [1, 2, 3]
        items = items.add_rule(:no_dups)
    "#;

    execute(code).expect("Should execute successfully");

    // Verify the list was created and rule was added
    // (The fact that it doesn't error means it worked)
}

#[test]
fn test_list_no_dups_prevents_duplicate_graphoid() {
    // This should create an error when trying to append duplicate
    let code = r#"
        items = [1, 2, 3].add_rule(:no_dups)
        items.append(2)
    "#;

    let result = execute(code);

    // Should fail because 2 is already in the list
    assert!(result.is_err());

    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("no_duplicates") || err_msg.contains("already exists"));
}

#[test]
fn test_list_add_rule_with_max_degree() {
    // Test parameterized rule
    let code = r#"
        items = [].add_rule(:max_degree, 5)
    "#;

    // Should succeed - max_degree is a valid rule
    execute(code).expect("Should execute successfully");
}

#[test]
fn test_hash_add_rule_no_dups() {
    // Hashes can also have rules (they're graphs!)
    let code = r#"
        config = {"a": 1, "b": 2}.add_rule(:no_dups)
    "#;

    execute(code).expect("Should execute successfully");
}

#[test]
fn test_list_remove_rule() {
    // Add and remove rule
    let code = r#"
        items = [1, 2].add_rule(:no_dups)
        items = items.remove_rule(:no_dups)
        items.append(2)
    "#;

    // Should succeed - rule was removed, so duplicate is allowed
    execute(code).expect("Should execute successfully");
}

#[test]
fn test_unknown_rule_error() {
    // Try to add an unknown rule
    let code = r#"
        items = [].add_rule(:unknown_rule)
    "#;

    let result = execute(code);

    // Should fail with unknown rule error
    assert!(result.is_err());

    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("Unknown rule"));
}

#[test]
fn test_list_with_no_dups_alternative_syntax() {
    // Test both :no_dups and :no_duplicates
    let code1 = r#"
        items1 = [].add_rule(:no_dups)
    "#;

    let code2 = r#"
        items2 = [].add_rule(:no_duplicates)
    "#;

    execute(code1).expect("Should accept :no_dups");
    execute(code2).expect("Should accept :no_duplicates");
}

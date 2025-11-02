//! Tests for Mutation Operator Convention
//! Phase 6.5 Area 3
//!
//! Every transformative method must have two versions:
//! - Immutable (no suffix): Returns new collection, original unchanged
//! - Mutating (`!` suffix): Modifies in place, returns none

use graphoid::ast;
use graphoid::execution::Executor;
use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
use graphoid::values::{Value, List};

fn eval(code: &str) -> Value {
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut executor = Executor::new();

    for (i, stmt) in program.statements.iter().enumerate() {
        if i == program.statements.len() - 1 {
            if let ast::Stmt::Expression { expr, .. } = stmt {
                return executor.eval_expr(expr).unwrap();
            }
        }
        executor.eval_stmt(stmt).unwrap();
    }

    Value::none()
}

fn list_nums(nums: Vec<f64>) -> Value {
    Value::list(List::from_vec(nums.into_iter().map(Value::number).collect()))
}

// ============================================================================
// SORT - sort() vs sort!()
// ============================================================================

#[test]
fn test_sort_immutable() {
    let code = r#"
        original = [3, 1, 2]
        sorted = original.sort()
        sorted
    "#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0]));
}

#[test]
fn test_sort_original_unchanged() {
    let code = r#"
        original = [3, 1, 2]
        sorted = original.sort()
        original
    "#;
    assert_eq!(eval(code), list_nums(vec![3.0, 1.0, 2.0]));
}

#[test]
fn test_sort_mutating() {
    let code = r#"
        nums = [3, 1, 2]
        result = nums.sort!()
        nums
    "#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0]));
}

#[test]
fn test_sort_mutating_returns_none() {
    let code = r#"
        nums = [3, 1, 2]
        nums.sort!()
    "#;
    assert_eq!(eval(code), Value::none());
}

// ============================================================================
// REVERSE - reverse() vs reverse!()
// ============================================================================

#[test]
fn test_reverse_immutable() {
    let code = r#"
        original = [1, 2, 3]
        reversed = original.reverse()
        reversed
    "#;
    assert_eq!(eval(code), list_nums(vec![3.0, 2.0, 1.0]));
}

#[test]
fn test_reverse_original_unchanged() {
    let code = r#"
        original = [1, 2, 3]
        reversed = original.reverse()
        original
    "#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0]));
}

#[test]
fn test_reverse_mutating() {
    let code = r#"
        nums = [1, 2, 3]
        result = nums.reverse!()
        nums
    "#;
    assert_eq!(eval(code), list_nums(vec![3.0, 2.0, 1.0]));
}

#[test]
fn test_reverse_mutating_returns_none() {
    let code = r#"
        nums = [1, 2, 3]
        nums.reverse!()
    "#;
    assert_eq!(eval(code), Value::none());
}

// ============================================================================
// UNIQ - uniq() vs uniq!()
// ============================================================================

#[test]
fn test_uniq_immutable() {
    let code = r#"
        original = [1, 2, 2, 3, 1]
        unique = original.uniq()
        unique
    "#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0]));
}

#[test]
fn test_uniq_original_unchanged() {
    let code = r#"
        original = [1, 2, 2, 3, 1]
        unique = original.uniq()
        original
    "#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 2.0, 3.0, 1.0]));
}

#[test]
fn test_uniq_mutating() {
    let code = r#"
        nums = [1, 2, 2, 3, 1]
        result = nums.uniq!()
        nums
    "#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0]));
}

#[test]
fn test_uniq_mutating_returns_none() {
    let code = r#"
        nums = [1, 2, 2, 3, 1]
        nums.uniq!()
    "#;
    assert_eq!(eval(code), Value::none());
}

// ============================================================================
// MAP - map() vs map!()
// ============================================================================

#[test]
fn test_map_immutable() {
    let code = r#"
        original = [1, 2, 3]
        doubled = original.map(x => x * 2)
        doubled
    "#;
    assert_eq!(eval(code), list_nums(vec![2.0, 4.0, 6.0]));
}

#[test]
fn test_map_original_unchanged() {
    let code = r#"
        original = [1, 2, 3]
        doubled = original.map(x => x * 2)
        original
    "#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0]));
}

#[test]
fn test_map_mutating() {
    let code = r#"
        nums = [1, 2, 3]
        result = nums.map!(x => x * 2)
        nums
    "#;
    assert_eq!(eval(code), list_nums(vec![2.0, 4.0, 6.0]));
}

#[test]
fn test_map_mutating_returns_none() {
    let code = r#"
        nums = [1, 2, 3]
        nums.map!(x => x * 2)
    "#;
    assert_eq!(eval(code), Value::none());
}

// ============================================================================
// FILTER - filter() vs filter!()
// ============================================================================

#[test]
fn test_filter_immutable() {
    let code = r#"
        original = [1, 2, 3, 4, 5]
        evens = original.filter(x => x % 2 == 0)
        evens
    "#;
    assert_eq!(eval(code), list_nums(vec![2.0, 4.0]));
}

#[test]
fn test_filter_original_unchanged() {
    let code = r#"
        original = [1, 2, 3, 4, 5]
        evens = original.filter(x => x % 2 == 0)
        original
    "#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0, 4.0, 5.0]));
}

#[test]
fn test_filter_mutating() {
    let code = r#"
        nums = [1, 2, 3, 4, 5]
        result = nums.filter!(x => x % 2 == 0)
        nums
    "#;
    assert_eq!(eval(code), list_nums(vec![2.0, 4.0]));
}

#[test]
fn test_filter_mutating_returns_none() {
    let code = r#"
        nums = [1, 2, 3, 4, 5]
        nums.filter!(x => x % 2 == 0)
    "#;
    assert_eq!(eval(code), Value::none());
}

// ============================================================================
// REJECT - reject() vs reject!()
// ============================================================================

#[test]
fn test_reject_immutable() {
    let code = r#"
        original = [1, 2, 3, 4, 5]
        odds = original.reject(x => x % 2 == 0)
        odds
    "#;
    assert_eq!(eval(code), list_nums(vec![1.0, 3.0, 5.0]));
}

#[test]
fn test_reject_original_unchanged() {
    let code = r#"
        original = [1, 2, 3, 4, 5]
        odds = original.reject(x => x % 2 == 0)
        original
    "#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0, 4.0, 5.0]));
}

#[test]
fn test_reject_mutating() {
    let code = r#"
        nums = [1, 2, 3, 4, 5]
        result = nums.reject!(x => x % 2 == 0)
        nums
    "#;
    assert_eq!(eval(code), list_nums(vec![1.0, 3.0, 5.0]));
}

#[test]
fn test_reject_mutating_returns_none() {
    let code = r#"
        nums = [1, 2, 3, 4, 5]
        nums.reject!(x => x % 2 == 0)
    "#;
    assert_eq!(eval(code), Value::none());
}

// ============================================================================
// COMPACT - compact() vs compact!()
// ============================================================================

#[test]
fn test_compact_immutable() {
    let code = r#"
        original = [1, none, 2, none, 3]
        compacted = original.compact()
        compacted
    "#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0]));
}

#[test]
fn test_compact_original_unchanged() {
    let code = r#"
        original = [1, none, 2, none, 3]
        compacted = original.compact()
        original.length()
    "#;
    assert_eq!(eval(code), Value::number(5.0));
}

#[test]
fn test_compact_mutating() {
    let code = r#"
        nums = [1, none, 2, none, 3]
        result = nums.compact!()
        nums
    "#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0]));
}

#[test]
fn test_compact_mutating_returns_none() {
    let code = r#"
        nums = [1, none, 2, none, 3]
        nums.compact!()
    "#;
    assert_eq!(eval(code), Value::none());
}

// ============================================================================
// SELECT - Alias for filter
// ============================================================================

#[test]
fn test_select_is_filter_alias() {
    let code = r#"
        nums = [1, 2, 3, 4, 5]
        nums.select(x => x > 2)
    "#;
    assert_eq!(eval(code), list_nums(vec![3.0, 4.0, 5.0]));
}

#[test]
fn test_select_mutating() {
    let code = r#"
        nums = [1, 2, 3, 4, 5]
        nums.select!(x => x > 2)
        nums
    "#;
    assert_eq!(eval(code), list_nums(vec![3.0, 4.0, 5.0]));
}

//! Tests for Missing Collection Methods
//! Phase 6.5 Area 4

use graphoid::ast;
use graphoid::execution::Executor;
use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
use graphoid::values::{Value, List, ValueKind};

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
// SLICE WITH STEP PARAMETER
// ============================================================================

#[test]
fn test_slice_with_step_basic() {
    // Every other element
    let code = "[1, 2, 3, 4, 5, 6].slice(0, 6, 2)";
    assert_eq!(eval(code), list_nums(vec![1.0, 3.0, 5.0]));
}

#[test]
fn test_slice_with_step_offset() {
    // Every other element starting from index 1
    let code = "[1, 2, 3, 4, 5, 6].slice(1, 6, 2)";
    assert_eq!(eval(code), list_nums(vec![2.0, 4.0, 6.0]));
}

#[test]
fn test_slice_with_step_three() {
    // Every third element
    let code = "[1, 2, 3, 4, 5, 6, 7, 8, 9].slice(0, 9, 3)";
    assert_eq!(eval(code), list_nums(vec![1.0, 4.0, 7.0]));
}

#[test]
fn test_slice_step_one_same_as_no_step() {
    // Step of 1 should be same as normal slice
    let code = "[1, 2, 3, 4, 5].slice(1, 4, 1)";
    assert_eq!(eval(code), list_nums(vec![2.0, 3.0, 4.0]));
}

#[test]
fn test_slice_step_larger_than_range() {
    // Step larger than range returns just start element
    let code = "[1, 2, 3, 4, 5].slice(0, 5, 10)";
    assert_eq!(eval(code), list_nums(vec![1.0]));
}

// ============================================================================
// GENERATE METHOD - Range Mode
// ============================================================================

#[test]
fn test_generate_range_basic() {
    // Generate sequence with step
    let code = "list.generate(1, 10, 2)";
    assert_eq!(eval(code), list_nums(vec![1.0, 3.0, 5.0, 7.0, 9.0]));
}

#[test]
fn test_generate_range_step_one() {
    // Generate consecutive numbers
    let code = "list.generate(1, 5, 1)";
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0, 4.0, 5.0]));
}

#[test]
fn test_generate_range_negative_step() {
    // Generate descending sequence
    let code = "list.generate(10, 5, -1)";
    assert_eq!(eval(code), list_nums(vec![10.0, 9.0, 8.0, 7.0, 6.0, 5.0]));
}

// ============================================================================
// GENERATE METHOD - Function Mode
// ============================================================================

#[test]
fn test_generate_function_squares() {
    // Generate using function
    let code = "list.generate(1, 5, x => x * x)";
    assert_eq!(eval(code), list_nums(vec![1.0, 4.0, 9.0, 16.0, 25.0]));
}

#[test]
fn test_generate_function_custom() {
    // Generate using custom function
    let code = "list.generate(1, 4, x => x * 2 + 1)";
    assert_eq!(eval(code), list_nums(vec![3.0, 5.0, 7.0, 9.0]));
}

// ============================================================================
// UPTO HELPER
// ============================================================================

#[test]
fn test_upto_basic() {
    // Generate 0 to n inclusive
    let code = "list.upto(5)";
    assert_eq!(eval(code), list_nums(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0]));
}

#[test]
fn test_upto_zero() {
    // upto(0) should return [0]
    let code = "list.upto(0)";
    assert_eq!(eval(code), list_nums(vec![0.0]));
}

#[test]
fn test_upto_in_expression() {
    // Use upto in expression
    let code = "list.upto(3).map(x => x * 2)";
    assert_eq!(eval(code), list_nums(vec![0.0, 2.0, 4.0, 6.0]));
}

// ============================================================================
// ADDITIONAL PREDICATES - Verify existing ones work
// ============================================================================

#[test]
fn test_predicate_positive() {
    let code = "[-1, 0, 1, 2].filter(:positive)";
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0]));
}

#[test]
fn test_predicate_negative() {
    let code = "[-2, -1, 0, 1].filter(:negative)";
    assert_eq!(eval(code), list_nums(vec![-2.0, -1.0]));
}

#[test]
fn test_predicate_even() {
    let code = "[1, 2, 3, 4, 5].filter(:even)";
    assert_eq!(eval(code), list_nums(vec![2.0, 4.0]));
}

#[test]
fn test_predicate_odd() {
    let code = "[1, 2, 3, 4, 5].filter(:odd)";
    assert_eq!(eval(code), list_nums(vec![1.0, 3.0, 5.0]));
}

// ============================================================================
// ADDITIONAL TRANSFORMATIONS - Verify existing ones work
// ============================================================================

#[test]
fn test_transformation_double() {
    let code = "[1, 2, 3].map(:double)";
    assert_eq!(eval(code), list_nums(vec![2.0, 4.0, 6.0]));
}

#[test]
fn test_transformation_square() {
    let code = "[2, 3, 4].map(:square)";
    assert_eq!(eval(code), list_nums(vec![4.0, 9.0, 16.0]));
}

#[test]
fn test_transformation_negate() {
    let code = "[1, -2, 3].map(:negate)";
    assert_eq!(eval(code), list_nums(vec![-1.0, 2.0, -3.0]));
}

#[test]
fn test_transformation_increment() {
    let code = "[1, 2, 3].map(:increment)";
    assert_eq!(eval(code), list_nums(vec![2.0, 3.0, 4.0]));
}

#[test]
fn test_transformation_decrement() {
    let code = "[1, 2, 3].map(:decrement)";
    assert_eq!(eval(code), list_nums(vec![0.0, 1.0, 2.0]));
}

// ============================================================================
// BASIC LIST METHODS - Phase 5
// ============================================================================

#[test]
fn test_list_size() {
    let code = "[1, 2, 3, 4, 5].size()";
    assert_eq!(eval(code), Value::number(5.0));
}

#[test]
fn test_list_size_empty() {
    let code = "[].size()";
    assert_eq!(eval(code), Value::number(0.0));
}

#[test]
fn test_list_first() {
    let code = "[10, 20, 30].first()";
    assert_eq!(eval(code), Value::number(10.0));
}

#[test]
fn test_list_last() {
    let code = "[10, 20, 30].last()";
    assert_eq!(eval(code), Value::number(30.0));
}

#[test]
fn test_list_is_empty_false() {
    let code = "[1, 2, 3].is_empty()";
    assert_eq!(eval(code), Value::boolean(false));
}

#[test]
fn test_list_is_empty_true() {
    let code = "[].is_empty()";
    assert_eq!(eval(code), Value::boolean(true));
}

#[test]
fn test_list_contains_true() {
    let code = "[1, 2, 3, 4, 5].contains(3)";
    assert_eq!(eval(code), Value::boolean(true));
}

#[test]
fn test_list_contains_false() {
    let code = "[1, 2, 3, 4, 5].contains(10)";
    assert_eq!(eval(code), Value::boolean(false));
}

#[test]
fn test_list_index_of_found() {
    let code = "[10, 20, 30, 40].index_of(30)";
    assert_eq!(eval(code), Value::number(2.0));
}

#[test]
fn test_list_index_of_not_found() {
    let code = "[10, 20, 30, 40].index_of(99)";
    assert_eq!(eval(code), Value::number(-1.0));
}

// ============================================================================
// LIST FUNCTIONAL METHODS - each, reduce
// ============================================================================

#[test]
fn test_list_each_returns_original() {
    let code = r#"
sum = 0
result = [1, 2, 3].each(x => { sum = sum + x })
result
"#;
    // each should return the original list
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0]));
}

#[test]
fn test_list_reduce_sum() {
    let code = "[1, 2, 3, 4, 5].reduce(0, (acc, x) => acc + x)";
    assert_eq!(eval(code), Value::number(15.0));
}

#[test]
fn test_list_reduce_product() {
    let code = "[1, 2, 3, 4].reduce(1, (acc, x) => acc * x)";
    assert_eq!(eval(code), Value::number(24.0));
}

#[test]
fn test_list_select_alias() {
    let code = "[1, 2, 3, 4, 5].select(x => x > 3)";
    assert_eq!(eval(code), list_nums(vec![4.0, 5.0]));
}

#[test]
fn test_list_reject() {
    let code = "[1, 2, 3, 4, 5].reject(x => x > 3)";
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0]));
}

// ============================================================================
// LIST MUTATION METHODS - append!, prepend!, insert!, remove!
// ============================================================================

#[test]
fn test_list_append_immutable() {
    let code = "[1, 2, 3].append(4)";
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0, 4.0]));
}

#[test]
fn test_list_append_mutable() {
    let code = r#"
items = [1, 2, 3]
items.append!(4)
items
"#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0, 4.0]));
}

#[test]
fn test_list_prepend_immutable() {
    let code = "[2, 3, 4].prepend(1)";
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0, 4.0]));
}

#[test]
fn test_list_prepend_mutable() {
    let code = r#"
items = [2, 3, 4]
items.prepend!(1)
items
"#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0, 4.0]));
}

#[test]
fn test_list_insert_immutable() {
    let code = "[1, 2, 4].insert(2, 3)";
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0, 4.0]));
}

#[test]
fn test_list_insert_mutable() {
    let code = r#"
items = [1, 2, 4]
items.insert!(2, 3)
items
"#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0, 4.0]));
}

#[test]
fn test_list_remove_immutable() {
    let code = "[1, 2, 3, 2, 4].remove(2)";
    // Should remove first occurrence
    assert_eq!(eval(code), list_nums(vec![1.0, 3.0, 2.0, 4.0]));
}

#[test]
fn test_list_remove_mutable() {
    let code = r#"
items = [1, 2, 3, 2, 4]
items.remove!(2)
items
"#;
    // Should remove first occurrence
    assert_eq!(eval(code), list_nums(vec![1.0, 3.0, 2.0, 4.0]));
}

#[test]
fn test_list_remove_at_index_immutable() {
    let code = "[1, 2, 3, 4].remove_at_index(2)";
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 4.0]));
}

#[test]
fn test_list_remove_at_index_mutable() {
    let code = r#"
items = [1, 2, 3, 4]
items.remove_at_index!(2)
items
"#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 4.0]));
}

#[test]
fn test_list_pop_immutable() {
    let code = "[1, 2, 3].pop()";
    // pop() without ! returns the popped value
    assert_eq!(eval(code), Value::number(3.0));
}

#[test]
fn test_list_pop_mutable() {
    let code = r#"
items = [1, 2, 3]
popped = items.pop!()
popped
"#;
    // pop!() returns the popped value
    assert_eq!(eval(code), Value::number(3.0));
}

#[test]
fn test_list_clear_immutable() {
    let code = "[1, 2, 3].clear()";
    assert_eq!(eval(code), list_nums(vec![]));
}

#[test]
fn test_list_clear_mutable() {
    let code = r#"
items = [1, 2, 3]
items.clear!()
items
"#;
    assert_eq!(eval(code), list_nums(vec![]));
}

// ============================================================================
// LIST TRANSFORMATION METHODS - sort, reverse, uniq, compact
// ============================================================================

#[test]
fn test_list_sort_immutable() {
    let code = "[3, 1, 4, 1, 5].sort()";
    assert_eq!(eval(code), list_nums(vec![1.0, 1.0, 3.0, 4.0, 5.0]));
}

#[test]
fn test_list_sort_mutable() {
    let code = r#"
items = [3, 1, 4, 1, 5]
items.sort!()
items
"#;
    assert_eq!(eval(code), list_nums(vec![1.0, 1.0, 3.0, 4.0, 5.0]));
}

#[test]
fn test_list_reverse_immutable() {
    let code = "[1, 2, 3, 4].reverse()";
    assert_eq!(eval(code), list_nums(vec![4.0, 3.0, 2.0, 1.0]));
}

#[test]
fn test_list_reverse_mutable() {
    let code = r#"
items = [1, 2, 3, 4]
items.reverse!()
items
"#;
    assert_eq!(eval(code), list_nums(vec![4.0, 3.0, 2.0, 1.0]));
}

#[test]
fn test_list_uniq_immutable() {
    let code = "[1, 2, 2, 3, 3, 3, 4].uniq()";
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0, 4.0]));
}

#[test]
fn test_list_uniq_mutable() {
    let code = r#"
items = [1, 2, 2, 3, 3, 3, 4]
items.uniq!()
items
"#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0, 4.0]));
}

#[test]
fn test_list_compact_immutable() {
    let code = "[1, none, 2, none, 3].compact()";
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0]));
}

#[test]
fn test_list_compact_mutable() {
    let code = r#"
items = [1, none, 2, none, 3]
items.compact!()
items
"#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0]));
}

// ============================================================================
// STRING METHODS - CASE CONVERSION (IMMUTABLE)
// ============================================================================

#[test]
fn test_string_upper() {
    let code = r#""hello world".upper()"#;
    assert_eq!(eval(code), Value::string("HELLO WORLD".to_string()));
}

#[test]
fn test_string_upper_immutable_preserves_original() {
    let code = r#"
s = "hello"
s.upper()
s
"#;
    assert_eq!(eval(code), Value::string("hello".to_string()));
}

#[test]
fn test_string_upper_empty() {
    let code = r#""".upper()"#;
    assert_eq!(eval(code), Value::string("".to_string()));
}

#[test]
fn test_string_upper_mixed_case() {
    let code = r#""HeLLo WoRLd".upper()"#;
    assert_eq!(eval(code), Value::string("HELLO WORLD".to_string()));
}

#[test]
fn test_string_lower() {
    let code = r#""HELLO WORLD".lower()"#;
    assert_eq!(eval(code), Value::string("hello world".to_string()));
}

#[test]
fn test_string_lower_empty() {
    let code = r#""".lower()"#;
    assert_eq!(eval(code), Value::string("".to_string()));
}

#[test]
fn test_string_lower_mixed_case() {
    let code = r#""HeLLo WoRLd".lower()"#;
    assert_eq!(eval(code), Value::string("hello world".to_string()));
}

// ============================================================================
// STRING METHODS - WHITESPACE HANDLING
// ============================================================================

#[test]
fn test_string_trim() {
    let code = r#""  hello world  ".trim()"#;
    assert_eq!(eval(code), Value::string("hello world".to_string()));
}

#[test]
fn test_string_trim_leading_only() {
    let code = r#""  hello world".trim()"#;
    assert_eq!(eval(code), Value::string("hello world".to_string()));
}

#[test]
fn test_string_trim_trailing_only() {
    let code = r#""hello world  ".trim()"#;
    assert_eq!(eval(code), Value::string("hello world".to_string()));
}

#[test]
fn test_string_trim_no_whitespace() {
    let code = r#""hello".trim()"#;
    assert_eq!(eval(code), Value::string("hello".to_string()));
}

#[test]
fn test_string_trim_empty() {
    let code = r#""".trim()"#;
    assert_eq!(eval(code), Value::string("".to_string()));
}

#[test]
fn test_string_trim_all_whitespace() {
    let code = r#""   ".trim()"#;
    assert_eq!(eval(code), Value::string("".to_string()));
}

// ============================================================================
// STRING METHODS - TRANSFORMATION
// ============================================================================

#[test]
fn test_string_reverse() {
    let code = r#""hello".reverse()"#;
    assert_eq!(eval(code), Value::string("olleh".to_string()));
}

#[test]
fn test_string_reverse_empty() {
    let code = r#""".reverse()"#;
    assert_eq!(eval(code), Value::string("".to_string()));
}

#[test]
fn test_string_reverse_single_char() {
    let code = r#""a".reverse()"#;
    assert_eq!(eval(code), Value::string("a".to_string()));
}

#[test]
fn test_string_reverse_with_spaces() {
    let code = r#""hello world".reverse()"#;
    assert_eq!(eval(code), Value::string("dlrow olleh".to_string()));
}

// ============================================================================
// STRING METHODS - SUBSTRING
// ============================================================================

#[test]
fn test_string_substring_basic() {
    let code = r#""hello world".substring(0, 5)"#;
    assert_eq!(eval(code), Value::string("hello".to_string()));
}

#[test]
fn test_string_substring_middle() {
    let code = r#""hello world".substring(6, 11)"#;
    assert_eq!(eval(code), Value::string("world".to_string()));
}

#[test]
fn test_string_substring_single_char() {
    let code = r#""hello".substring(1, 2)"#;
    assert_eq!(eval(code), Value::string("e".to_string()));
}

#[test]
fn test_string_substring_full_string() {
    let code = r#""hello".substring(0, 5)"#;
    assert_eq!(eval(code), Value::string("hello".to_string()));
}

#[test]
fn test_string_substring_empty_range() {
    let code = r#""hello".substring(2, 2)"#;
    assert_eq!(eval(code), Value::string("".to_string()));
}

#[test]
fn test_string_substring_to_end() {
    let code = r#""hello world".substring(6, 20)"#;
    assert_eq!(eval(code), Value::string("world".to_string()));
}

// ============================================================================
// STRING METHODS - SPLIT
// ============================================================================

#[test]
fn test_string_split_basic() {
    let code = r#""a,b,c".split(",")"#;
    let expected = Value::list(List::from_vec(vec![
        Value::string("a".to_string()),
        Value::string("b".to_string()),
        Value::string("c".to_string()),
    ]));
    assert_eq!(eval(code), expected);
}

#[test]
fn test_string_split_spaces() {
    let code = r#""hello world test".split(" ")"#;
    let expected = Value::list(List::from_vec(vec![
        Value::string("hello".to_string()),
        Value::string("world".to_string()),
        Value::string("test".to_string()),
    ]));
    assert_eq!(eval(code), expected);
}

#[test]
fn test_string_split_no_delimiter() {
    let code = r#""hello".split(",")"#;
    let expected = Value::list(List::from_vec(vec![
        Value::string("hello".to_string()),
    ]));
    assert_eq!(eval(code), expected);
}

#[test]
fn test_string_split_empty_string() {
    let code = r#""".split(",")"#;
    let expected = Value::list(List::from_vec(vec![
        Value::string("".to_string()),
    ]));
    assert_eq!(eval(code), expected);
}

#[test]
fn test_string_split_consecutive_delimiters() {
    let code = r#""a,,b".split(",")"#;
    let expected = Value::list(List::from_vec(vec![
        Value::string("a".to_string()),
        Value::string("".to_string()),
        Value::string("b".to_string()),
    ]));
    assert_eq!(eval(code), expected);
}

// ============================================================================
// STRING METHODS - PATTERN MATCHING
// ============================================================================

#[test]
fn test_string_starts_with_true() {
    let code = r#""hello world".starts_with("hello")"#;
    assert_eq!(eval(code), Value::boolean(true));
}

#[test]
fn test_string_starts_with_false() {
    let code = r#""hello world".starts_with("world")"#;
    assert_eq!(eval(code), Value::boolean(false));
}

#[test]
fn test_string_starts_with_empty_prefix() {
    let code = r#""hello".starts_with("")"#;
    assert_eq!(eval(code), Value::boolean(true));
}

#[test]
fn test_string_starts_with_same_string() {
    let code = r#""hello".starts_with("hello")"#;
    assert_eq!(eval(code), Value::boolean(true));
}

#[test]
fn test_string_starts_with_longer_prefix() {
    let code = r#""hi".starts_with("hello")"#;
    assert_eq!(eval(code), Value::boolean(false));
}

#[test]
fn test_string_ends_with_true() {
    let code = r#""hello world".ends_with("world")"#;
    assert_eq!(eval(code), Value::boolean(true));
}

#[test]
fn test_string_ends_with_false() {
    let code = r#""hello world".ends_with("hello")"#;
    assert_eq!(eval(code), Value::boolean(false));
}

#[test]
fn test_string_ends_with_empty_suffix() {
    let code = r#""hello".ends_with("")"#;
    assert_eq!(eval(code), Value::boolean(true));
}

#[test]
fn test_string_ends_with_same_string() {
    let code = r#""hello".ends_with("hello")"#;
    assert_eq!(eval(code), Value::boolean(true));
}

#[test]
fn test_string_ends_with_longer_suffix() {
    let code = r#""hi".ends_with("hello")"#;
    assert_eq!(eval(code), Value::boolean(false));
}

#[test]
fn test_string_contains_true() {
    let code = r#""hello world".contains("lo wo")"#;
    assert_eq!(eval(code), Value::boolean(true));
}

#[test]
fn test_string_contains_false() {
    let code = r#""hello world".contains("xyz")"#;
    assert_eq!(eval(code), Value::boolean(false));
}

#[test]
fn test_string_contains_empty_substring() {
    let code = r#""hello".contains("")"#;
    assert_eq!(eval(code), Value::boolean(true));
}

#[test]
fn test_string_contains_at_start() {
    let code = r#""hello world".contains("hello")"#;
    assert_eq!(eval(code), Value::boolean(true));
}

#[test]
fn test_string_contains_at_end() {
    let code = r#""hello world".contains("world")"#;
    assert_eq!(eval(code), Value::boolean(true));
}

#[test]
fn test_string_contains_same_string() {
    let code = r#""hello".contains("hello")"#;
    assert_eq!(eval(code), Value::boolean(true));
}

// ============================================================================
// STRING METHODS - MUTABLE VERSIONS
// ============================================================================

#[test]
fn test_string_upper_mutable() {
    let code = r#"
s = "hello world"
s.upper!()
s
"#;
    assert_eq!(eval(code), Value::string("HELLO WORLD".to_string()));
}

#[test]
fn test_string_upper_mutable_returns_none() {
    let code = r#"
s = "hello"
s.upper!()
"#;
    assert_eq!(eval(code), Value::none());
}

#[test]
fn test_string_lower_mutable() {
    let code = r#"
s = "HELLO WORLD"
s.lower!()
s
"#;
    assert_eq!(eval(code), Value::string("hello world".to_string()));
}

#[test]
fn test_string_lower_mutable_returns_none() {
    let code = r#"
s = "HELLO"
s.lower!()
"#;
    assert_eq!(eval(code), Value::none());
}

#[test]
fn test_string_trim_mutable() {
    let code = r#"
s = "  hello world  "
s.trim!()
s
"#;
    assert_eq!(eval(code), Value::string("hello world".to_string()));
}

#[test]
fn test_string_trim_mutable_returns_none() {
    let code = r#"
s = "  hello  "
s.trim!()
"#;
    assert_eq!(eval(code), Value::none());
}

#[test]
fn test_string_reverse_mutable() {
    let code = r#"
s = "hello"
s.reverse!()
s
"#;
    assert_eq!(eval(code), Value::string("olleh".to_string()));
}

#[test]
fn test_string_reverse_mutable_returns_none() {
    let code = r#"
s = "abc"
s.reverse!()
"#;
    assert_eq!(eval(code), Value::none());
}

#[test]
fn test_string_mutable_chaining() {
    let code = r#"
s = "  HELLO WORLD  "
s.trim!()
s.lower!()
s
"#;
    assert_eq!(eval(code), Value::string("hello world".to_string()));
}

#[test]
fn test_string_mutable_mixed_with_immutable() {
    let code = r#"
s = "hello"
upper_s = s.upper()
s.reverse!()
s
"#;
    assert_eq!(eval(code), Value::string("olleh".to_string()));
}

#[test]
fn test_string_immutable_preserves_during_mutation() {
    let code = r#"
s = "hello"
upper_s = s.upper()
s.reverse!()
upper_s
"#;
    assert_eq!(eval(code), Value::string("HELLO".to_string()));
}

// ============================================================================
// MAP/HASH METHODS - INSPECTION
// ============================================================================

#[test]
fn test_map_keys() {
    let code = r#"
m = {"name": "Alice", "age": 30, "city": "NYC"}
m.keys()
"#;
    let result = eval(code);
    if let ValueKind::List(list) = &result.kind {
        let keys: Vec<String> = list.to_vec()
            .into_iter()
            .map(|v| match &v.kind {
                ValueKind::String(s) => s.clone(),
                _ => panic!("Expected strings"),
            })
            .collect();
        // Keys can be in any order, so check they all exist
        assert!(keys.contains(&"name".to_string()));
        assert!(keys.contains(&"age".to_string()));
        assert!(keys.contains(&"city".to_string()));
        assert_eq!(keys.len(), 3);
    } else {
        panic!("Expected list of keys");
    }
}

#[test]
fn test_map_keys_empty() {
    let code = r#"
m = {}
m.keys()
"#;
    assert_eq!(eval(code), Value::list(List::from_vec(vec![])));
}

#[test]
fn test_map_values() {
    let code = r#"
m = {"a": 1, "b": 2, "c": 3}
m.values()
"#;
    let result = eval(code);
    if let ValueKind::List(list) = &result.kind {
        let values: Vec<f64> = list.to_vec()
            .into_iter()
            .map(|v| match &v.kind {
                ValueKind::Number(n) => *n,
                _ => panic!("Expected numbers"),
            })
            .collect();
        // Values can be in any order, so check they all exist
        assert!(values.contains(&1.0));
        assert!(values.contains(&2.0));
        assert!(values.contains(&3.0));
        assert_eq!(values.len(), 3);
    } else {
        panic!("Expected list of values");
    }
}

#[test]
fn test_map_values_empty() {
    let code = r#"
m = {}
m.values()
"#;
    assert_eq!(eval(code), Value::list(List::from_vec(vec![])));
}

#[test]
fn test_map_has_key_true() {
    let code = r#"
m = {"name": "Alice", "age": 30}
m.has_key("name")
"#;
    assert_eq!(eval(code), Value::boolean(true));
}

#[test]
fn test_map_has_key_false() {
    let code = r#"
m = {"name": "Alice", "age": 30}
m.has_key("city")
"#;
    assert_eq!(eval(code), Value::boolean(false));
}

#[test]
fn test_map_has_key_empty_map() {
    let code = r#"
m = {}
m.has_key("anything")
"#;
    assert_eq!(eval(code), Value::boolean(false));
}

#[test]
fn test_map_size() {
    let code = r#"
m = {"a": 1, "b": 2, "c": 3}
m.size()
"#;
    assert_eq!(eval(code), Value::number(3.0));
}

#[test]
fn test_map_size_empty() {
    let code = r#"
m = {}
m.size()
"#;
    assert_eq!(eval(code), Value::number(0.0));
}

#[test]
fn test_map_size_after_additions() {
    let code = r#"
m = {"a": 1}
m["b"] = 2
m["c"] = 3
m.size()
"#;
    assert_eq!(eval(code), Value::number(3.0));
}

// ============================================================================
// MAP/HASH METHODS - INDEXING ACCESS
// ============================================================================

#[test]
fn test_map_index_get() {
    let code = r#"
m = {"name": "Alice", "age": 30}
m["name"]
"#;
    assert_eq!(eval(code), Value::string("Alice".to_string()));
}

#[test]
fn test_map_index_get_number() {
    let code = r#"
m = {"age": 30}
m["age"]
"#;
    assert_eq!(eval(code), Value::number(30.0));
}

#[test]
fn test_map_index_get_nonexistent_lenient() {
    let code = r#"
configure { error_mode: :lenient } {
    m = {"name": "Alice"}
    m["city"]
}
"#;
    assert_eq!(eval(code), Value::none());
}

#[test]
fn test_map_index_set_new_key() {
    let code = r#"
m = {"name": "Alice"}
m["age"] = 30
m["age"]
"#;
    assert_eq!(eval(code), Value::number(30.0));
}

#[test]
fn test_map_index_set_existing_key() {
    let code = r#"
m = {"name": "Alice", "age": 25}
m["age"] = 30
m["age"]
"#;
    assert_eq!(eval(code), Value::number(30.0));
}

#[test]
fn test_map_index_set_multiple() {
    let code = r#"
m = {}
m["a"] = 1
m["b"] = 2
m["c"] = 3
m.size()
"#;
    assert_eq!(eval(code), Value::number(3.0));
}

// ============================================================================
// MAP/HASH METHODS - MIXED VALUE TYPES
// ============================================================================

#[test]
fn test_map_mixed_value_types() {
    let code = r#"
m = {"name": "Alice", "age": 30, "active": true}
m["name"]
"#;
    assert_eq!(eval(code), Value::string("Alice".to_string()));
}

#[test]
fn test_map_nested_list_value() {
    let code = r#"
m = {"items": [1, 2, 3]}
m["items"]
"#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0]));
}

#[test]
fn test_map_none_value() {
    let code = r#"
m = {"value": none}
m["value"]
"#;
    assert_eq!(eval(code), Value::none());
}

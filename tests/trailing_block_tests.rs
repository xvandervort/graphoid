/// Trailing Block Syntax Tests
///
/// Tests for Ruby/Smalltalk-style trailing blocks: method { |params| body }

use graphoid::execution::Executor;
use graphoid::values::{Value, ValueKind};

// ============================================================================
// TRAILING BLOCKS - METHOD CALLS
// ============================================================================

#[test]
fn test_trailing_block_with_each() {
    let source = r#"
numbers = [1, 2, 3, 4, 5]

# Use each to verify it executes the block
# (each returns the original list)
result = numbers.each { |x|
    # Blocks execute for side effects
    temp = x * 2
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // Verify each returns the original list
    match &executor.get_variable("result").unwrap().kind {
        ValueKind::List(list) => {
            assert_eq!(list.len(), 5);
            assert_eq!(list.get(0).unwrap(), &Value::number(1.0));
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn test_trailing_block_with_map() {
    let source = r#"
numbers = [1, 2, 3, 4, 5]
doubled = numbers.map { |x| x * 2 }
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    match &executor.get_variable("doubled").unwrap().kind {
        ValueKind::List(list) => {
            assert_eq!(list.len(), 5);
            assert_eq!(list.get(0).unwrap(), &Value::number(2.0));
            assert_eq!(list.get(1).unwrap(), &Value::number(4.0));
            assert_eq!(list.get(2).unwrap(), &Value::number(6.0));
            assert_eq!(list.get(3).unwrap(), &Value::number(8.0));
            assert_eq!(list.get(4).unwrap(), &Value::number(10.0));
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn test_trailing_block_with_filter() {
    let source = r#"
numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
evens = numbers.filter { |x| x % 2 == 0 }
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    match &executor.get_variable("evens").unwrap().kind {
        ValueKind::List(list) => {
            assert_eq!(list.len(), 5);
            assert_eq!(list.get(0).unwrap(), &Value::number(2.0));
            assert_eq!(list.get(1).unwrap(), &Value::number(4.0));
            assert_eq!(list.get(2).unwrap(), &Value::number(6.0));
            assert_eq!(list.get(3).unwrap(), &Value::number(8.0));
            assert_eq!(list.get(4).unwrap(), &Value::number(10.0));
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn test_trailing_block_multi_param() {
    let source = r#"
fn with_coords(block) {
    result1 = block(10, 20)
    result2 = block(30, 40)
    return result1 + result2
}

result = with_coords() { |x, y|
    return x + y
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // (10+20) + (30+40) = 30 + 70 = 100
    assert_eq!(executor.get_variable("result").unwrap(), Value::number(100.0));
}

#[test]
fn test_trailing_block_no_params() {
    let source = r#"
fn run_twice(block) {
    result1 = block()
    result2 = block()
    return result1 + result2
}

count = run_twice() { ||
    return 1
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("count").unwrap(), Value::number(2.0));
}

#[test]
fn test_trailing_block_method_chaining() {
    let source = r#"
numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
result = numbers
    .filter { |x| x % 2 == 0 }
    .map { |x| x * 2 }
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    match &executor.get_variable("result").unwrap().kind {
        ValueKind::List(list) => {
            assert_eq!(list.len(), 5);
            assert_eq!(list.get(0).unwrap(), &Value::number(4.0));   // 2*2
            assert_eq!(list.get(1).unwrap(), &Value::number(8.0));   // 4*2
            assert_eq!(list.get(2).unwrap(), &Value::number(12.0));  // 6*2
            assert_eq!(list.get(3).unwrap(), &Value::number(16.0));  // 8*2
            assert_eq!(list.get(4).unwrap(), &Value::number(20.0));  // 10*2
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn test_trailing_block_with_statements() {
    let source = r#"
numbers = [1, 2, 3, 4, 5]
transformed = numbers.map { |x|
    temp = x * 2
    result = temp + 1
    return result
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    match &executor.get_variable("transformed").unwrap().kind {
        ValueKind::List(list) => {
            assert_eq!(list.len(), 5);
            assert_eq!(list.get(0).unwrap(), &Value::number(3.0));   // 1*2+1
            assert_eq!(list.get(1).unwrap(), &Value::number(5.0));   // 2*2+1
            assert_eq!(list.get(2).unwrap(), &Value::number(7.0));   // 3*2+1
            assert_eq!(list.get(3).unwrap(), &Value::number(9.0));   // 4*2+1
            assert_eq!(list.get(4).unwrap(), &Value::number(11.0));  // 5*2+1
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn test_trailing_block_with_conditionals() {
    let source = r#"
numbers = [-5, -3, 0, 3, 5]
absolutes = numbers.map { |x|
    if x < 0 {
        return -x
    }
    return x
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    match &executor.get_variable("absolutes").unwrap().kind {
        ValueKind::List(list) => {
            assert_eq!(list.len(), 5);
            assert_eq!(list.get(0).unwrap(), &Value::number(5.0));
            assert_eq!(list.get(1).unwrap(), &Value::number(3.0));
            assert_eq!(list.get(2).unwrap(), &Value::number(0.0));
            assert_eq!(list.get(3).unwrap(), &Value::number(3.0));
            assert_eq!(list.get(4).unwrap(), &Value::number(5.0));
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn test_trailing_block_closure_capture() {
    let source = r#"
multiplier = 10
numbers = [1, 2, 3]
scaled = numbers.map { |x|
    return x * multiplier
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    match &executor.get_variable("scaled").unwrap().kind {
        ValueKind::List(list) => {
            assert_eq!(list.len(), 3);
            assert_eq!(list.get(0).unwrap(), &Value::number(10.0));
            assert_eq!(list.get(1).unwrap(), &Value::number(20.0));
            assert_eq!(list.get(2).unwrap(), &Value::number(30.0));
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn test_trailing_block_with_loop() {
    let source = r#"
lists = [[1, 2], [3, 4, 5], [6]]
totals = lists.map { |lst|
    sum = 0
    for item in lst {
        sum = sum + item
    }
    return sum
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    match &executor.get_variable("totals").unwrap().kind {
        ValueKind::List(list) => {
            assert_eq!(list.len(), 3);
            assert_eq!(list.get(0).unwrap(), &Value::number(3.0));   // 1+2
            assert_eq!(list.get(1).unwrap(), &Value::number(12.0));  // 3+4+5
            assert_eq!(list.get(2).unwrap(), &Value::number(6.0));   // 6
        }
        _ => panic!("Expected list"),
    }
}

// ============================================================================
// TRAILING BLOCKS - FUNCTION CALLS
// ============================================================================

#[test]
fn test_trailing_block_function_call() {
    let source = r#"
fn apply(x, func) {
    return func(x)
}

result = apply(5) { |n|
    return n * 2
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result").unwrap(), Value::number(10.0));
}

#[test]
fn test_trailing_block_function_with_multiple_args() {
    let source = r#"
fn apply_binary(x, y, func) {
    return func(x, y)
}

result = apply_binary(10, 5) { |a, b|
    return a - b
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result").unwrap(), Value::number(5.0));
}

#[test]
fn test_trailing_block_mixed_with_regular_lambda() {
    let source = r#"
numbers = [1, 2, 3, 4, 5]

# Trailing block syntax
result1 = numbers.map { |x| x * 2 }

# Regular lambda syntax
result2 = numbers.map(x => x * 2)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result1 = executor.get_variable("result1").unwrap();
    let result2 = executor.get_variable("result2").unwrap();

    // Both should produce the same result
    match (&result1.kind, &result2.kind) {
        (ValueKind::List(list1), ValueKind::List(list2)) => {
            assert_eq!(list1.len(), list2.len());
            for i in 0..list1.len() {
                assert_eq!(list1.get(i).unwrap(), list2.get(i).unwrap());
            }
        }
        _ => panic!("Expected lists"),
    }
}

#[test]
fn test_trailing_block_string_operations() {
    let source = r#"
words = ["hello", "world"]
uppercased = words.map { |w|
    result = w  # Note: would uppercase in real impl
    return result
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    match &executor.get_variable("uppercased").unwrap().kind {
        ValueKind::List(list) => {
            assert_eq!(list.len(), 2);
            assert_eq!(list.get(0).unwrap(), &Value::string("hello".to_string()));
            assert_eq!(list.get(1).unwrap(), &Value::string("world".to_string()));
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn test_trailing_block_nested_calls() {
    let source = r#"
fn outer(block) {
    return block(10)
}

result = outer() { |x|
    inner_result = x * 2
    return inner_result
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result").unwrap(), Value::number(20.0));
}

#[test]
fn test_trailing_block_early_return() {
    let source = r#"
numbers = [1, 2, 3, 4, 5]
first_big = numbers.filter { |x|
    if x > 3 {
        return true
    }
    return false
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    match &executor.get_variable("first_big").unwrap().kind {
        ValueKind::List(list) => {
            assert_eq!(list.len(), 2);
            assert_eq!(list.get(0).unwrap(), &Value::number(4.0));
            assert_eq!(list.get(1).unwrap(), &Value::number(5.0));
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn test_trailing_block_complex_transformation() {
    let source = r#"
numbers = [1, 2, 3, 4, 5]
result = numbers.map { |x|
    if x % 2 == 0 {
        doubled = x * 2
        return doubled
    } else {
        tripled = x * 3
        return tripled
    }
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    match &executor.get_variable("result").unwrap().kind {
        ValueKind::List(list) => {
            assert_eq!(list.len(), 5);
            assert_eq!(list.get(0).unwrap(), &Value::number(3.0));   // 1*3 (odd)
            assert_eq!(list.get(1).unwrap(), &Value::number(4.0));   // 2*2 (even)
            assert_eq!(list.get(2).unwrap(), &Value::number(9.0));   // 3*3 (odd)
            assert_eq!(list.get(3).unwrap(), &Value::number(8.0));   // 4*2 (even)
            assert_eq!(list.get(4).unwrap(), &Value::number(15.0));  // 5*3 (odd)
        }
        _ => panic!("Expected list"),
    }
}

/// Lambda Block Body Tests
///
/// Tests for multi-statement lambda bodies: x => { statements }

use graphoid::execution::Executor;
use graphoid::values::Value;

// ============================================================================
// LAMBDA BLOCK BODIES
// ============================================================================

#[test]
fn test_lambda_block_single_param() {
    let source = r#"
process = x => {
    temp = x * 2
    return temp + 1
}

result = process(5)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result").unwrap(), Value::Number(11.0));
}

#[test]
fn test_lambda_block_multi_param() {
    let source = r#"
calculate = (a, b, c) => {
    sum = a + b + c
    product = a * b * c
    return sum + product
}

result = calculate(2, 3, 4)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // sum = 2 + 3 + 4 = 9
    // product = 2 * 3 * 4 = 24
    // result = 9 + 24 = 33
    assert_eq!(executor.get_variable("result").unwrap(), Value::Number(33.0));
}

#[test]
fn test_lambda_block_zero_param() {
    let source = r#"
get_value = () => {
    x = 10
    y = 20
    return x + y
}

result = get_value()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result").unwrap(), Value::Number(30.0));
}

#[test]
fn test_lambda_block_with_conditional() {
    let source = r#"
abs_value = x => {
    if x < 0 {
        return -x
    }
    return x
}

result1 = abs_value(5)
result2 = abs_value(-5)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result1").unwrap(), Value::Number(5.0));
    assert_eq!(executor.get_variable("result2").unwrap(), Value::Number(5.0));
}

#[test]
fn test_lambda_block_with_loop() {
    let source = r#"
factorial = n => {
    result = 1
    for i in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] {
        if i <= n {
            result = result * i
        }
    }
    return result
}

result1 = factorial(5)
result2 = factorial(3)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result1").unwrap(), Value::Number(120.0)); // 5! = 120
    assert_eq!(executor.get_variable("result2").unwrap(), Value::Number(6.0));   // 3! = 6
}

#[test]
fn test_lambda_block_closure_capture() {
    let source = r#"
fn make_adder(x) {
    adder = y => {
        temp = x + y
        return temp
    }
    return adder
}

add5 = make_adder(5)
result = add5(3)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result").unwrap(), Value::Number(8.0));
}

#[test]
fn test_lambda_block_with_map() {
    let source = r#"
numbers = [1, 2, 3, 4, 5]

doubled_plus_one = numbers.map(x => {
    temp = x * 2
    return temp + 1
})
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    match executor.get_variable("doubled_plus_one").unwrap() {
        Value::List(list) => {
            assert_eq!(list.len(), 5);
            assert_eq!(list.get(0).unwrap(), &Value::Number(3.0));  // 1*2+1
            assert_eq!(list.get(1).unwrap(), &Value::Number(5.0));  // 2*2+1
            assert_eq!(list.get(2).unwrap(), &Value::Number(7.0));  // 3*2+1
            assert_eq!(list.get(3).unwrap(), &Value::Number(9.0));  // 4*2+1
            assert_eq!(list.get(4).unwrap(), &Value::Number(11.0)); // 5*2+1
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn test_lambda_block_with_filter() {
    let source = r#"
numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

big_evens = numbers.filter(x => {
    is_even = x % 2 == 0
    is_big = x > 5
    return is_even and is_big
})
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    match executor.get_variable("big_evens").unwrap() {
        Value::List(list) => {
            assert_eq!(list.len(), 3);
            assert_eq!(list.get(0).unwrap(), &Value::Number(6.0));
            assert_eq!(list.get(1).unwrap(), &Value::Number(8.0));
            assert_eq!(list.get(2).unwrap(), &Value::Number(10.0));
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn test_lambda_block_early_return() {
    let source = r#"
find_first_positive = x => {
    if x > 0 {
        return x
    }
    # This shouldn't execute
    return 999
}

result1 = find_first_positive(5)
result2 = find_first_positive(-5)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result1").unwrap(), Value::Number(5.0));
    assert_eq!(executor.get_variable("result2").unwrap(), Value::Number(999.0));
}

#[test]
fn test_lambda_block_nested_blocks() {
    let source = r#"
complex = x => {
    if x > 0 {
        temp = x * 2
        return temp
    } else {
        temp = x * -1
        return temp
    }
}

result1 = complex(5)
result2 = complex(-5)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result1").unwrap(), Value::Number(10.0));
    assert_eq!(executor.get_variable("result2").unwrap(), Value::Number(5.0));
}

#[test]
fn test_lambda_block_string_operations() {
    let source = r#"
format_name = name => {
    upper = name
    result = "Hello, " + upper + "!"
    return result
}

result = format_name("Alice")
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result").unwrap(), Value::String("Hello, Alice!".to_string()));
}

#[test]
fn test_lambda_block_list_operations() {
    let source = r#"
process_list = items => {
    result = []
    for item in items {
        result = result.append(item * 2)
    }
    return result
}

result = process_list([1, 2, 3])
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    match executor.get_variable("result").unwrap() {
        Value::List(list) => {
            assert_eq!(list.len(), 3);
            assert_eq!(list.get(0).unwrap(), &Value::Number(2.0));
            assert_eq!(list.get(1).unwrap(), &Value::Number(4.0));
            assert_eq!(list.get(2).unwrap(), &Value::Number(6.0));
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn test_lambda_block_multiple_returns() {
    let source = r#"
sign = x => {
    if x > 0 {
        return 1
    }
    if x < 0 {
        return -1
    }
    return 0
}

result1 = sign(10)
result2 = sign(-10)
result3 = sign(0)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result1").unwrap(), Value::Number(1.0));
    assert_eq!(executor.get_variable("result2").unwrap(), Value::Number(-1.0));
    assert_eq!(executor.get_variable("result3").unwrap(), Value::Number(0.0));
}

#[test]
fn test_lambda_block_variable_shadowing() {
    let source = r#"
x = 100

process = y => {
    x = y * 2  # Shadows outer x
    return x
}

result = process(5)
outer_x = x  # Should still be 100
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result").unwrap(), Value::Number(10.0));
    assert_eq!(executor.get_variable("outer_x").unwrap(), Value::Number(100.0));
}

#[test]
fn test_lambda_block_mixed_with_expression_lambdas() {
    let source = r#"
# Expression lambda
simple = x => x * 2

# Block lambda
complex = x => {
    temp = x * 2
    return temp + 1
}

result1 = simple(5)
result2 = complex(5)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result1").unwrap(), Value::Number(10.0));
    assert_eq!(executor.get_variable("result2").unwrap(), Value::Number(11.0));
}

/// Advanced Function Features Tests
///
/// This file tests the advanced function capabilities:
/// 1. Closures (environment capture)
/// 2. Default parameters
/// 3. Variadic functions
/// 4. Named parameters

use graphoid::execution::Executor;
use graphoid::values::{Value, ValueKind};

// ============================================================================
// CLOSURES - Environment Capture
// ============================================================================

#[test]
fn test_closure_captures_local_variable() {
    let source = r#"
fn make_counter() {
    count = 0
    fn increment() {
        count = count + 1
        return count
    }
    return increment
}

counter = make_counter()
result1 = counter()
result2 = counter()
result3 = counter()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result1").unwrap(), Value::number(1.0));
    assert_eq!(executor.get_variable("result2").unwrap(), Value::number(2.0));
    assert_eq!(executor.get_variable("result3").unwrap(), Value::number(3.0));
}

#[test]
fn test_closure_captures_parameter() {
    let source = r#"
fn make_adder(x) {
    fn add(y) {
        return x + y
    }
    return add
}

add5 = make_adder(5)
add10 = make_adder(10)

result1 = add5(3)
result2 = add10(3)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result1").unwrap(), Value::number(8.0));
    assert_eq!(executor.get_variable("result2").unwrap(), Value::number(13.0));
}

#[test]
fn test_multiple_closures_independent() {
    let source = r#"
fn make_counter() {
    count = 0
    fn increment() {
        count = count + 1
        return count
    }
    return increment
}

counter1 = make_counter()
counter2 = make_counter()

counter1()
counter1()
result1 = counter1()  # Should be 3

counter2()
result2 = counter2()  # Should be 2
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result1").unwrap(), Value::number(3.0));
    assert_eq!(executor.get_variable("result2").unwrap(), Value::number(2.0));
}

#[test]
fn test_nested_closures() {
    let source = r#"
fn outer(x) {
    fn middle(y) {
        fn inner(z) {
            return x + y + z
        }
        return inner
    }
    return middle
}

f = outer(1)
g = f(2)
result = g(3)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result").unwrap(), Value::number(6.0));
}

#[test]
fn test_closure_with_lambda() {
    let source = r#"
fn make_multiplier(factor) {
    multiplier = x => x * factor
    return multiplier
}

times2 = make_multiplier(2)
times5 = make_multiplier(5)

result1 = times2(10)
result2 = times5(10)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result1").unwrap(), Value::number(20.0));
    assert_eq!(executor.get_variable("result2").unwrap(), Value::number(50.0));
}

// ============================================================================
// DEFAULT PARAMETERS
// ============================================================================

#[test]
fn test_default_parameter_single() {
    let source = r#"
fn greet(name = "World") {
    return "Hello " + name
}

result1 = greet()
result2 = greet("Alice")
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result1").unwrap(), Value::string("Hello World".to_string()));
    assert_eq!(executor.get_variable("result2").unwrap(), Value::string("Hello Alice".to_string()));
}

#[test]
fn test_default_parameters_multiple() {
    let source = r#"
fn create_user(name = "Anonymous", age = 0, active = true) {
    return name + ":" + age + ":" + active
}

result1 = create_user()
result2 = create_user("Bob")
result3 = create_user("Alice", 25)
result4 = create_user("Charlie", 30, false)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result1").unwrap(), Value::string("Anonymous:0:true".to_string()));
    assert_eq!(executor.get_variable("result2").unwrap(), Value::string("Bob:0:true".to_string()));
    assert_eq!(executor.get_variable("result3").unwrap(), Value::string("Alice:25:true".to_string()));
    assert_eq!(executor.get_variable("result4").unwrap(), Value::string("Charlie:30:false".to_string()));
}

#[test]
fn test_default_parameter_mixed_required_optional() {
    let source = r#"
fn power(base, exponent = 2) {
    result = 1
    for i in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] {
        if i <= exponent {
            result = result * base
        }
    }
    return result
}

result1 = power(3)     # 3^2 = 9
result2 = power(2, 3)  # 2^3 = 8
result3 = power(5, 1)  # 5^1 = 5
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result1").unwrap(), Value::number(9.0));
    assert_eq!(executor.get_variable("result2").unwrap(), Value::number(8.0));
    assert_eq!(executor.get_variable("result3").unwrap(), Value::number(5.0));
}

#[test]
fn test_default_parameter_expression() {
    let source = r#"
fn add_tax(price, tax_rate = 0.1) {
    return price * (1 + tax_rate)
}

result1 = add_tax(100)      # 100 * 1.1 = 110
result2 = add_tax(100, 0.2) # 100 * 1.2 = 120
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // Use approximate equality for floating point comparisons
    match &executor.get_variable("result1").unwrap().kind {
        ValueKind::Number(n) => assert!((n - 110.0).abs() < 1e-10, "Expected ~110.0, got {}", n),
        other => panic!("Expected number, got {:?}", other),
    }
    match &executor.get_variable("result2").unwrap().kind {
        ValueKind::Number(n) => assert!((n - 120.0).abs() < 1e-10, "Expected ~120.0, got {}", n),
        other => panic!("Expected number, got {:?}", other),
    }
}

#[test]
fn test_default_parameter_list() {
    let source = r#"
fn append_item(items = [], item = none) {
    if item != none {
        items = items.append(item)
    }
    return items
}

result1 = append_item()
result2 = append_item([1, 2, 3], 4)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // result1 should be empty list
    match &executor.get_variable("result1").unwrap().kind {
        ValueKind::List(list) => assert_eq!(list.len(), 0),
        _ => panic!("Expected list"),
    }

    // result2 should be [1, 2, 3, 4]
    match &executor.get_variable("result2").unwrap().kind {
        ValueKind::List(list) => assert_eq!(list.len(), 4),
        _ => panic!("Expected list"),
    }
}

// ============================================================================
// VARIADIC FUNCTIONS
// ============================================================================

#[test]
fn test_variadic_basic() {
    let source = r#"
fn sum(...numbers) {
    total = 0
    for n in numbers {
        total = total + n
    }
    return total
}

result1 = sum()
result2 = sum(1)
result3 = sum(1, 2, 3)
result4 = sum(1, 2, 3, 4, 5)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result1").unwrap(), Value::number(0.0));
    assert_eq!(executor.get_variable("result2").unwrap(), Value::number(1.0));
    assert_eq!(executor.get_variable("result3").unwrap(), Value::number(6.0));
    assert_eq!(executor.get_variable("result4").unwrap(), Value::number(15.0));
}

#[test]
fn test_variadic_with_required_params() {
    let source = r#"
fn format_list(prefix, ...items) {
    result = prefix
    for item in items {
        result = result + "," + item
    }
    return result
}

result1 = format_list("Values")
result2 = format_list("Numbers", "1", "2", "3")
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result1").unwrap(), Value::string("Values".to_string()));
    assert_eq!(executor.get_variable("result2").unwrap(), Value::string("Numbers,1,2,3".to_string()));
}

#[test]
fn test_variadic_with_defaults() {
    let source = r#"
fn make_list(separator = ",", ...items) {
    if items.length() == 0 {
        return ""
    }
    result = items[0]
    for i in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] {
        if i < items.length() {
            result = result + separator + items[i]
        }
    }
    return result
}

result1 = make_list()
result2 = make_list(",", "a", "b", "c")
result3 = make_list("|", "x", "y")
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result1").unwrap(), Value::string("".to_string()));
    assert_eq!(executor.get_variable("result2").unwrap(), Value::string("a,b,c".to_string()));
    assert_eq!(executor.get_variable("result3").unwrap(), Value::string("x|y".to_string()));
}

#[test]
fn test_variadic_max() {
    let source = r#"
fn max(...numbers) {
    if numbers.length() == 0 {
        return none
    }
    max_val = numbers[0]
    for n in numbers {
        if n > max_val {
            max_val = n
        }
    }
    return max_val
}

result1 = max(5, 2, 8, 1, 9, 3)
result2 = max(42)
result3 = max(-5, -2, -10)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result1").unwrap(), Value::number(9.0));
    assert_eq!(executor.get_variable("result2").unwrap(), Value::number(42.0));
    assert_eq!(executor.get_variable("result3").unwrap(), Value::number(-2.0));
}

// ============================================================================
// NAMED PARAMETERS
// ============================================================================

#[test]
fn test_named_parameters_basic() {
    let source = r#"
fn greet(name, greeting) {
    return greeting + " " + name
}

result1 = greet(name: "Alice", greeting: "Hello")
result2 = greet(greeting: "Hi", name: "Bob")
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result1").unwrap(), Value::string("Hello Alice".to_string()));
    assert_eq!(executor.get_variable("result2").unwrap(), Value::string("Hi Bob".to_string()));
}

#[test]
fn test_named_parameters_mixed_with_positional() {
    let source = r#"
fn create_user(name, age, city) {
    return name + ":" + age + ":" + city
}

result1 = create_user("Alice", age: 25, city: "NYC")
result2 = create_user("Bob", city: "LA", age: 30)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result1").unwrap(), Value::string("Alice:25:NYC".to_string()));
    assert_eq!(executor.get_variable("result2").unwrap(), Value::string("Bob:30:LA".to_string()));
}

#[test]
fn test_named_parameters_with_defaults() {
    let source = r#"
fn make_config(host = "localhost", port = 8080, debug = false) {
    return host + ":" + port + ":" + debug
}

result1 = make_config()
result2 = make_config(port: 3000)
result3 = make_config(debug: true, port: 9000)
result4 = make_config(host: "example.com", debug: true)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result1").unwrap(), Value::string("localhost:8080:false".to_string()));
    assert_eq!(executor.get_variable("result2").unwrap(), Value::string("localhost:3000:false".to_string()));
    assert_eq!(executor.get_variable("result3").unwrap(), Value::string("localhost:9000:true".to_string()));
    assert_eq!(executor.get_variable("result4").unwrap(), Value::string("example.com:8080:true".to_string()));
}

#[test]
fn test_named_parameters_all_features_combined() {
    let source = r#"
fn process(required, optional = "default", ...rest) {
    result = required + ":" + optional
    for item in rest {
        result = result + "," + item
    }
    return result
}

result1 = process("A")
result2 = process("B", optional: "custom")
result3 = process("C", "D", "E", "F")
result4 = process("G", optional: "H", "I", "J")
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("result1").unwrap(), Value::string("A:default".to_string()));
    assert_eq!(executor.get_variable("result2").unwrap(), Value::string("B:custom".to_string()));
    assert_eq!(executor.get_variable("result3").unwrap(), Value::string("C:D,E,F".to_string()));
    assert_eq!(executor.get_variable("result4").unwrap(), Value::string("G:H,I,J".to_string()));
}

// ============================================================================
// ERROR CASES
// ============================================================================

#[test]
fn test_too_many_arguments_error() {
    let source = r#"
fn add(a, b) {
    return a + b
}

result = add(1, 2, 3)
"#;
    let mut executor = Executor::new();
    let result = executor.execute_source(source);
    assert!(result.is_err());
}

#[test]
fn test_too_few_arguments_error() {
    let source = r#"
fn multiply(a, b, c) {
    return a * b * c
}

result = multiply(2, 3)
"#;
    let mut executor = Executor::new();
    let result = executor.execute_source(source);
    assert!(result.is_err());
}

#[test]
fn test_unknown_named_parameter_error() {
    let source = r#"
fn greet(name) {
    return "Hello " + name
}

result = greet(unknown: "Alice")
"#;
    let mut executor = Executor::new();
    let result = executor.execute_source(source);
    assert!(result.is_err());
}

#[test]
fn test_duplicate_named_parameter_error() {
    let source = r#"
fn add(a, b) {
    return a + b
}

result = add(a: 1, a: 2, b: 3)
"#;
    let mut executor = Executor::new();
    let result = executor.execute_source(source);
    assert!(result.is_err());
}

use graphoid::execution::Executor;
use graphoid::values::ValueKind;

// ============================================================================
// Number Methods Tests
// ============================================================================

// sqrt() tests

#[test]
fn test_sqrt_perfect_square() {
    let mut executor = Executor::new();
    let code = r#"
        result = (9.0).sqrt()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if (*n - 3.0).abs() < 0.0001));
}

#[test]
fn test_sqrt_non_perfect() {
    let mut executor = Executor::new();
    let code = r#"
        result = (2.0).sqrt()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if (*n - 1.41421356).abs() < 0.0001));
}

#[test]
fn test_sqrt_zero() {
    let mut executor = Executor::new();
    let code = r#"
        result = (0.0).sqrt()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 0.0));
}

// abs() tests

#[test]
fn test_abs_positive() {
    let mut executor = Executor::new();
    let code = r#"
        result = (5.5).abs()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 5.5));
}

#[test]
fn test_abs_negative() {
    let mut executor = Executor::new();
    let code = r#"
        result = (-5.5).abs()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 5.5));
}

#[test]
fn test_abs_zero() {
    let mut executor = Executor::new();
    let code = r#"
        result = (0.0).abs()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 0.0));
}

// up() tests

#[test]
fn test_up_no_args() {
    let mut executor = Executor::new();
    let code = r#"
        result = (3.2).up()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 4.0));
}

#[test]
fn test_up_with_decimal_places() {
    let mut executor = Executor::new();
    let code = r#"
        result = (3.14159).up(2)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if (*n - 3.15).abs() < 0.0001));
}

#[test]
fn test_up_nearest_ten() {
    let mut executor = Executor::new();
    let code = r#"
        result = (23.0).up(:nearest_ten)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 30.0));
}

#[test]
fn test_up_nearest_hundred() {
    let mut executor = Executor::new();
    let code = r#"
        result = (250.0).up(:nearest_hundred)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 300.0));
}

// down() tests

#[test]
fn test_down_no_args() {
    let mut executor = Executor::new();
    let code = r#"
        result = (3.8).down()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 3.0));
}

#[test]
fn test_down_with_decimal_places() {
    let mut executor = Executor::new();
    let code = r#"
        result = (3.14159).down(2)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if (*n - 3.14).abs() < 0.0001));
}

#[test]
fn test_down_nearest_ten() {
    let mut executor = Executor::new();
    let code = r#"
        result = (27.0).down(:nearest_ten)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 20.0));
}

// round() tests

#[test]
fn test_round_no_args() {
    let mut executor = Executor::new();
    let code = r#"
        result = (3.5).round()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 4.0));
}

#[test]
fn test_round_with_decimal_places() {
    let mut executor = Executor::new();
    let code = r#"
        result = (3.14159).round(2)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if (*n - 3.14).abs() < 0.0001));
}

#[test]
fn test_round_nearest_ten() {
    let mut executor = Executor::new();
    let code = r#"
        result = (25.0).round(:nearest_ten)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 30.0));
}

#[test]
fn test_round_nearest_ten_down() {
    let mut executor = Executor::new();
    let code = r#"
        result = (24.0).round(:nearest_ten)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 20.0));
}

// log() tests

#[test]
fn test_log_natural() {
    let mut executor = Executor::new();
    let code = r#"
        result = (2.718281828).log()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if (*n - 1.0).abs() < 0.0001));
}

#[test]
fn test_log_with_base() {
    let mut executor = Executor::new();
    let code = r#"
        result = (100.0).log(10)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if (*n - 2.0).abs() < 0.0001));
}

#[test]
fn test_log_base_2() {
    let mut executor = Executor::new();
    let code = r#"
        result = (8.0).log(2)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if (*n - 3.0).abs() < 0.0001));
}

// Negative number edge cases

#[test]
fn test_up_negative() {
    let mut executor = Executor::new();
    let code = r#"
        result = (-3.2).up()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == -3.0));
}

#[test]
fn test_down_negative() {
    let mut executor = Executor::new();
    let code = r#"
        result = (-3.2).down()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == -4.0));
}

#[test]
fn test_round_negative() {
    let mut executor = Executor::new();
    let code = r#"
        result = (-3.5).round()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == -4.0));
}

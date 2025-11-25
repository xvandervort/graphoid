// Debug test for unsigned mode

use graphoid::execution::Executor;

#[test]
fn test_simple_variable_assignment() {
    let mut exec = Executor::new();
    let source = "x = 42";
    exec.execute_source(source).unwrap();

    let x = exec.get_variable("x");
    assert!(x.is_some(), "Variable x should exist");
    assert_eq!(x.unwrap().to_number().unwrap(), 42.0);
}

#[test]
fn test_variable_in_configure_block() {
    let mut exec = Executor::new();
    let source = r#"
configure { error_mode: :lenient } {
    y = 100
}
"#;
    exec.execute_source(source).unwrap();

    let y = exec.get_variable("y");
    assert!(y.is_some(), "Variable y from configure block should exist");
    assert_eq!(y.unwrap().to_number().unwrap(), 100.0);
}

#[test]
fn test_unsigned_flag_in_config() {
    let mut exec = Executor::new();

    // Check default unsigned mode is false
    assert_eq!(exec.config_stack.current().unsigned_mode, false);

    let source = r#"
configure { :unsigned } {
    dummy = 1
}
"#;
    exec.execute_source(source).unwrap();

    // After block, should be back to false
    assert_eq!(exec.config_stack.current().unsigned_mode, false);
}

#[test]
fn test_right_shift_with_unsigned_debug() {
    let mut exec = Executor::new();

    // First test: signed mode (default)
    let source1 = "signed_val = -16 >> 2";
    exec.execute_source(source1).unwrap();
    let signed_val = exec.get_variable("signed_val").unwrap();
    println!("Signed: -16 >> 2 = {}", signed_val.to_number().unwrap());
    assert_eq!(signed_val.to_number().unwrap(), -4.0);

    // Second test: unsigned mode
    let source2 = r#"
configure { :unsigned } {
    unsigned_val = -16 >> 2
}
"#;
    exec.execute_source(source2).unwrap();

    let unsigned_val_opt = exec.get_variable("unsigned_val");
    assert!(unsigned_val_opt.is_some(), "unsigned_val should exist");

    let unsigned_val = unsigned_val_opt.unwrap();
    let num = unsigned_val.to_number().unwrap();
    println!("Unsigned: -16 >> 2 = {}", num);

    // -16 as i64 = 0xFFFFFFFFFFFFFFF0
    // as u64 = 18446744073709551600
    // >> 2 = 4611686018427387900.0
    assert!(num > 0.0, "Expected positive number, got {}", num);
}

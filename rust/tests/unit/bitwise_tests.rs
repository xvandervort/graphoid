// Phase 13: Bitwise Operators - Execution Tests
// Tests for &, |, ^, ~, <<, >>, ** operators and configure { :unsigned }

use graphoid::execution::Executor;
use graphoid::error::GraphoidError;
use graphoid::values::ValueKind;

// ============================================================================
// Basic Bitwise Operations (10 tests)
// ============================================================================

#[test]
fn test_bitwise_and_positive() {
    let mut exec = Executor::new();
    let source = "result = 12 & 10";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 8.0); // 0b1100 & 0b1010 = 0b1000
}

#[test]
fn test_bitwise_or_positive() {
    let mut exec = Executor::new();
    let source = "result = 12 | 10";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 14.0); // 0b1100 | 0b1010 = 0b1110
}

#[test]
fn test_bitwise_xor_positive() {
    let mut exec = Executor::new();
    let source = "result = 12 ^ 10";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 6.0); // 0b1100 ^ 0b1010 = 0b0110
}

#[test]
fn test_bitwise_not_positive() {
    let mut exec = Executor::new();
    let source = "result = ~5";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), -6.0); // ~0b0101 = 0b...1010 (two's complement)
}

#[test]
fn test_left_shift_basic() {
    let mut exec = Executor::new();
    let source = "result = 3 << 2";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 12.0); // 0b0011 << 2 = 0b1100
}

#[test]
fn test_right_shift_basic_signed() {
    let mut exec = Executor::new();
    let source = "result = 12 >> 2";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 3.0); // 0b1100 >> 2 = 0b0011
}

#[test]
fn test_bitwise_and_with_zero() {
    let mut exec = Executor::new();
    let source = "result = 0xFF & 0";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 0.0);
}

#[test]
fn test_bitwise_or_with_zero() {
    let mut exec = Executor::new();
    let source = "result = 42 | 0";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 42.0);
}

#[test]
fn test_bitwise_xor_with_zero() {
    let mut exec = Executor::new();
    let source = "result = 42 ^ 0";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 42.0);
}

#[test]
fn test_bitwise_not_zero() {
    let mut exec = Executor::new();
    let source = "result = ~0";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), -1.0); // ~0 = -1 (all bits set)
}

// ============================================================================
// Negative Number Handling (8 tests)
// ============================================================================

#[test]
fn test_bitwise_and_negative() {
    let mut exec = Executor::new();
    let source = "result = -1 & 0xFF";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 255.0); // -1 has all bits set
}

#[test]
fn test_bitwise_or_negative() {
    let mut exec = Executor::new();
    let source = "result = -8 | 3";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), -5.0);
}

#[test]
fn test_bitwise_xor_negative() {
    let mut exec = Executor::new();
    let source = "result = -1 ^ 0xFF";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), -256.0);
}

#[test]
fn test_bitwise_not_negative() {
    let mut exec = Executor::new();
    let source = "result = ~(-1)";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 0.0); // ~(-1) = 0
}

#[test]
fn test_left_shift_negative() {
    let mut exec = Executor::new();
    let source = "result = -4 << 2";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), -16.0);
}

#[test]
fn test_right_shift_negative_signed() {
    let mut exec = Executor::new();
    let source = "result = -16 >> 2";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), -4.0); // Arithmetic shift preserves sign
}

#[test]
fn test_right_shift_negative_unsigned() {
    let mut exec = Executor::new();
    let source = r#"
configure { :unsigned } {
    result = -16 >> 2
}
"#;
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    let num = result.to_number().unwrap();
    // -16 as u64 = 18446744073709551600, >> 2 = 4611686018427387900
    // Should be a large positive number, not -4
    assert!(num > 0.0, "Expected positive number from unsigned shift, got {}", num);
    assert!(num > 4611686018427387000.0, "Expected ~4.6e18, got {}", num);
}

#[test]
fn test_mixed_positive_negative() {
    let mut exec = Executor::new();
    let source = "result = (-8 & 15) | (12 ^ 10)";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    // -8 & 15 = 8, 12 ^ 10 = 6, 8 | 6 = 14
    assert_eq!(result.to_number().unwrap(), 14.0);
}

// ============================================================================
// Shift Edge Cases (8 tests)
// ============================================================================

#[test]
fn test_shift_by_zero() {
    let mut exec = Executor::new();
    let source = r#"
left_result = 42 << 0
right_result = 42 >> 0
"#;
    exec.execute_source(source).unwrap();

    assert_eq!(exec.get_variable("left_result").unwrap().to_number().unwrap(), 42.0);
    assert_eq!(exec.get_variable("right_result").unwrap().to_number().unwrap(), 42.0);
}

#[test]
fn test_shift_by_one() {
    let mut exec = Executor::new();
    let source = r#"
left_result = 21 << 1
right_result = 84 >> 1
"#;
    exec.execute_source(source).unwrap();

    assert_eq!(exec.get_variable("left_result").unwrap().to_number().unwrap(), 42.0);
    assert_eq!(exec.get_variable("right_result").unwrap().to_number().unwrap(), 42.0);
}

#[test]
fn test_shift_by_31() {
    let mut exec = Executor::new();
    let source = r#"
left_result = 1 << 31
right_result = 0x80000000 >> 31
"#;
    exec.execute_source(source).unwrap();

    assert_eq!(exec.get_variable("left_result").unwrap().to_number().unwrap(), 2147483648.0);
    // 0x80000000 as i64 is 2147483648, >> 31 = 1
    assert_eq!(exec.get_variable("right_result").unwrap().to_number().unwrap(), 1.0);
}

#[test]
fn test_shift_by_63_max_valid() {
    let mut exec = Executor::new();
    let source = r#"
left_result = 1 << 63
right_result = left_result >> 63
"#;
    exec.execute_source(source).unwrap();

    // 1 << 63 = -9223372036854775808 (sign bit set)
    assert_eq!(exec.get_variable("left_result").unwrap().to_number().unwrap(), -9223372036854775808.0);
    // Right shift by 63 of negative number = -1 (sign extends)
    assert_eq!(exec.get_variable("right_result").unwrap().to_number().unwrap(), -1.0);
}

#[test]
fn test_shift_by_64_error() {
    let mut exec = Executor::new();
    let source = "result = 1 << 64";
    let result = exec.execute_source(source);

    assert!(result.is_err());
    match result {
        Err(GraphoidError::RuntimeError { message, .. }) => {
            assert!(message.contains("too large"));
        }
        _ => panic!("Expected RuntimeError for shift by 64"),
    }
}

#[test]
fn test_shift_by_large_amount_error() {
    let mut exec = Executor::new();
    let source = "result = 1 >> 100";
    let result = exec.execute_source(source);

    assert!(result.is_err());
}

#[test]
fn test_left_shift_overflow() {
    let mut exec = Executor::new();
    let source = "result = 0x7FFFFFFFFFFFFFFF << 1";
    exec.execute_source(source).unwrap();

    // Overflow wraps in two's complement
    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), -2.0);
}

#[test]
fn test_right_shift_underflow() {
    let mut exec = Executor::new();
    let source = "result = 1 >> 62";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 0.0); // Shifts beyond all bits = 0
}

// ============================================================================
// Configure { :unsigned } Tests (6 tests)
// ============================================================================

#[test]
fn test_unsigned_right_shift_positive() {
    let mut exec = Executor::new();
    let source = r#"
configure { :unsigned } {
    result = 16 >> 2
}
"#;
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 4.0); // Same as signed for positive
}

#[test]
fn test_unsigned_right_shift_negative_comparison() {
    let mut exec = Executor::new();
    let source = r#"
signed_result = -8 >> 1

configure { :unsigned } {
    unsigned_result = -8 >> 1
}
"#;
    exec.execute_source(source).unwrap();

    // Signed: -8 >> 1 = -4
    assert_eq!(exec.get_variable("signed_result").unwrap().to_number().unwrap(), -4.0);

    // Unsigned: -8 as u64 >> 1 = large positive number
    let unsigned_result = exec.get_variable("unsigned_result").unwrap().to_number().unwrap();
    assert!(unsigned_result > 0.0, "Unsigned result should be positive, got {}", unsigned_result);
    assert!(unsigned_result > 9.2e18, "Expected very large positive (~9.2e18), got {}", unsigned_result);
}

#[test]
fn test_nested_configure_blocks() {
    let mut exec = Executor::new();
    let source = r#"
outer = -8 >> 1

configure { :unsigned } {
    inner = -8 >> 1
}

outer2 = -8 >> 1
"#;
    exec.execute_source(source).unwrap();

    // Before and after: signed
    assert_eq!(exec.get_variable("outer").unwrap().to_number().unwrap(), -4.0);
    assert_eq!(exec.get_variable("outer2").unwrap().to_number().unwrap(), -4.0);

    // Inside: unsigned
    let inner = exec.get_variable("inner").unwrap().to_number().unwrap();
    assert!(inner > 0.0, "Inner should be unsigned (positive), got {}", inner);
}

#[test]
fn test_configure_block_scope_exit() {
    let mut exec = Executor::new();
    let source = r#"
before_config = -16 >> 2

configure { :unsigned } {
    inside = -16 >> 2
}

after_config = -16 >> 2
"#;
    exec.execute_source(source).unwrap();

    // Verify unsigned mode only applies inside block
    assert_eq!(exec.get_variable("before_config").unwrap().to_number().unwrap(), -4.0);
    assert_eq!(exec.get_variable("after_config").unwrap().to_number().unwrap(), -4.0);

    let inside = exec.get_variable("inside").unwrap().to_number().unwrap();
    assert!(inside > 0.0, "Inside should be unsigned (positive), got {}", inside);
}

#[test]
fn test_file_level_configure_unsigned() {
    let mut exec = Executor::new();
    let source = r#"
configure { :unsigned }

result1 = -8 >> 1
result2 = -16 >> 2
"#;
    exec.execute_source(source).unwrap();

    // File-level configure stays active
    let result1 = exec.get_variable("result1").unwrap().to_number().unwrap();
    let result2 = exec.get_variable("result2").unwrap().to_number().unwrap();

    assert!(result1 > 0.0, "File-level unsigned mode should apply, got result1={}", result1);
    assert!(result2 > 0.0, "File-level unsigned mode should apply, got result2={}", result2);
}

#[test]
fn test_multiple_configure_settings() {
    let mut exec = Executor::new();
    let source = r#"
configure { :unsigned, error_mode: :lenient } {
    result = -8 >> 1
}
"#;
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap().to_number().unwrap();
    assert!(result > 0.0, "Unsigned mode with other settings should work, got {}", result);
}

// ============================================================================
// Power Operator Tests (8 tests)
// ============================================================================

#[test]
fn test_power_basic() {
    let mut exec = Executor::new();
    let source = "result = 2 ** 3";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 8.0);
}

#[test]
fn test_power_right_associativity() {
    let mut exec = Executor::new();
    let source = "result = 2 ** 3 ** 2";
    exec.execute_source(source).unwrap();

    // Should be 2 ** (3 ** 2) = 2 ** 9 = 512, not (2 ** 3) ** 2 = 64
    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 512.0);
}

#[test]
fn test_power_negative_base() {
    let mut exec = Executor::new();
    let source = "result = (-2) ** 3";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), -8.0);
}

#[test]
fn test_power_negative_exponent() {
    let mut exec = Executor::new();
    let source = "result = 2 ** (-3)";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 0.125); // 1/8
}

#[test]
fn test_power_zero_base() {
    let mut exec = Executor::new();
    let source = "result = 0 ** 5";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 0.0);
}

#[test]
fn test_power_zero_exponent() {
    let mut exec = Executor::new();
    let source = "result = 42 ** 0";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 1.0); // Any number to power 0 = 1
}

#[test]
fn test_power_fractional_exponent() {
    let mut exec = Executor::new();
    let source = "result = 9 ** 0.5";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 3.0); // Square root
}

#[test]
fn test_power_large_values() {
    let mut exec = Executor::new();
    let source = "result = 10 ** 6";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 1_000_000.0);
}

// ============================================================================
// Binary and Hex Literal Tests (6 tests)
// ============================================================================

#[test]
fn test_binary_literals() {
    let mut exec = Executor::new();
    let source = r#"
a = 0b1010
b = 0b1100
result = a & b
"#;
    exec.execute_source(source).unwrap();

    assert_eq!(exec.get_variable("a").unwrap().to_number().unwrap(), 10.0);
    assert_eq!(exec.get_variable("b").unwrap().to_number().unwrap(), 12.0);
    assert_eq!(exec.get_variable("result").unwrap().to_number().unwrap(), 8.0);
}

#[test]
fn test_hex_literals() {
    let mut exec = Executor::new();
    let source = r#"
a = 0xFF
b = 0x0F
result = a & b
"#;
    exec.execute_source(source).unwrap();

    assert_eq!(exec.get_variable("a").unwrap().to_number().unwrap(), 255.0);
    assert_eq!(exec.get_variable("b").unwrap().to_number().unwrap(), 15.0);
    assert_eq!(exec.get_variable("result").unwrap().to_number().unwrap(), 15.0);
}

#[test]
fn test_binary_with_underscores() {
    let mut exec = Executor::new();
    let source = "result = 0b1111_0000";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 240.0);
}

#[test]
fn test_hex_with_underscores() {
    let mut exec = Executor::new();
    let source = "result = 0xFF_00";
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 65280.0);
}

#[test]
fn test_mixed_literal_formats() {
    let mut exec = Executor::new();
    let source = r#"
binary = 0b1010
hex = 0xA
decimal = 10
result = (binary == hex) and (hex == decimal)
"#;
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    assert!(result.is_truthy()); // All values are equal (10)
}

#[test]
fn test_bitwise_operations_with_literals() {
    let mut exec = Executor::new();
    let source = r#"
mask = 0b11110000
value = 0xAB
result = value & mask
"#;
    exec.execute_source(source).unwrap();

    // 0xAB = 171 = 0b10101011, & 0b11110000 = 0b10100000 = 160
    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 160.0);
}

// ============================================================================
// Complex Expression Tests (4 tests)
// ============================================================================

#[test]
fn test_bitwise_precedence() {
    let mut exec = Executor::new();
    let source = "result = 8 | 4 & 2";
    exec.execute_source(source).unwrap();

    // Should be 8 | (4 & 2) = 8 | 0 = 8
    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 8.0);
}

#[test]
fn test_shift_and_arithmetic() {
    let mut exec = Executor::new();
    let source = "result = (3 << 2) + (16 >> 2)";
    exec.execute_source(source).unwrap();

    // (3 << 2) = 12, (16 >> 2) = 4, 12 + 4 = 16
    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 16.0);
}

#[test]
fn test_power_and_bitwise() {
    let mut exec = Executor::new();
    let source = "result = (2 ** 4) & 0xFF";
    exec.execute_source(source).unwrap();

    // 2 ** 4 = 16, 16 & 0xFF = 16
    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 16.0);
}

#[test]
fn test_complex_bitwise_expression() {
    let mut exec = Executor::new();
    let source = "result = (~0xFF & 0xFFFF) | (0x0F << 4)";
    exec.execute_source(source).unwrap();

    // ~0xFF = -256, & 0xFFFF = 65280, 0x0F << 4 = 240, | = 65520
    let result = exec.get_variable("result").unwrap();
    assert_eq!(result.to_number().unwrap(), 65520.0);
}

// ============================================================================
// Phase 13: :32bit Directive - Wrapping Arithmetic Tests
// ============================================================================

#[test]
fn test_32bit_addition_wrapping() {
    let mut exec = Executor::new();
    let source = r#"
        configure { :unsigned, :32bit } {
            x = 4294967295  # 0xFFFFFFFF
            y = 1
            result = x + y
        }
    "#;
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_i64().unwrap(), 0);  // Wraps to 0
        }
        _ => panic!("Expected BigNumber, got {:?}", result.kind),
    }
}

#[test]
fn test_32bit_subtraction_wrapping() {
    let mut exec = Executor::new();
    let source = r#"
        configure { :unsigned, :32bit } {
            x = 0
            y = 1
            result = x - y
        }
    "#;
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_u64().unwrap(), 4294967295);  // Wraps to 0xFFFFFFFF
        }
        _ => panic!("Expected BigNumber, got {:?}", result.kind),
    }
}

#[test]
fn test_32bit_multiplication_wrapping() {
    let mut exec = Executor::new();
    let source = r#"
        configure { :unsigned, :32bit } {
            x = 65536  # 0x10000
            y = 65536
            result = x * y
        }
    "#;
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_i64().unwrap(), 0);  // 0x100000000 wraps to 0
        }
        _ => panic!("Expected BigNumber, got {:?}", result.kind),
    }
}

#[test]
fn test_32bit_left_shift_wrapping() {
    let mut exec = Executor::new();
    let source = r#"
        configure { :unsigned, :32bit } {
            x = 305419896  # 0x12345678
            result = x << 8
        }
    "#;
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_i64().unwrap(), 0x34567800);  // High bits discarded
        }
        _ => panic!("Expected BigNumber, got {:?}", result.kind),
    }
}

#[test]
fn test_32bit_bitwise_and_wrapping() {
    let mut exec = Executor::new();
    let source = r#"
        configure { :unsigned, :32bit } {
            x = 4294967295  # 0xFFFFFFFF
            y = 252645135   # 0x0F0F0F0F
            result = x & y
        }
    "#;
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_u64().unwrap(), 252645135);  // 0x0F0F0F0F
        }
        _ => panic!("Expected BigNumber, got {:?}", result.kind),
    }
}

#[test]
fn test_32bit_bitwise_or_wrapping() {
    let mut exec = Executor::new();
    let source = r#"
        configure { :unsigned, :32bit } {
            x = 4042322160  # 0xF0F0F0F0
            y = 252645135   # 0x0F0F0F0F
            result = x | y
        }
    "#;
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_u64().unwrap(), 4294967295);  // 0xFFFFFFFF
        }
        _ => panic!("Expected BigNumber, got {:?}", result.kind),
    }
}

#[test]
fn test_32bit_bitwise_xor_wrapping() {
    let mut exec = Executor::new();
    let source = r#"
        configure { :unsigned, :32bit } {
            x = 4294967295  # 0xFFFFFFFF
            y = 4294967295  # 0xFFFFFFFF
            result = x ^ y
        }
    "#;
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_i64().unwrap(), 0);
        }
        _ => panic!("Expected BigNumber, got {:?}", result.kind),
    }
}

#[test]
fn test_64bit_no_wrapping() {
    let mut exec = Executor::new();
    let source = r#"
        configure { :unsigned } {
            x = 4294967295  # 0xFFFFFFFF
            y = 1
            result = x + y
        }
    "#;
    exec.execute_source(source).unwrap();

    // Without :high or :integer, values may be regular Numbers (f64) or BigNumbers
    let result = exec.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_u64().unwrap(), 4294967296);  // No wrapping in 64-bit mode
        }
        ValueKind::Number(n) => {
            assert_eq!(*n as u64, 4294967296);  // No wrapping in 64-bit mode
        }
        _ => panic!("Expected Number or BigNumber, got {:?}", result.kind),
    }
}

#[test]
fn test_32bit_nested_configure() {
    let mut exec = Executor::new();
    let source = r#"
        configure { :unsigned } {
            outer = 4294967295 + 1  # 64-bit: 4294967296

            configure { :32bit } {
                inner = 4294967295 + 1  # 32-bit: 0
            }

            back_to_outer = 4294967295 + 1  # 64-bit: 4294967296
        }
    "#;
    exec.execute_source(source).unwrap();

    // In 64-bit mode (:unsigned without :32bit), values are UInt64 BigNumbers
    let outer = exec.get_variable("outer").unwrap();
    match &outer.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_u64().unwrap(), 4294967296);
        }
        ValueKind::Number(n) => {
            assert_eq!(*n as u64, 4294967296);
        }
        _ => panic!("Expected Number or BigNumber for outer, got {:?}", outer.kind),
    }

    // In 32-bit mode, should be Int64/UInt64 BigNumber with value 0
    let inner = exec.get_variable("inner").unwrap();
    match &inner.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_i64().unwrap(), 0);
        }
        _ => panic!("Expected BigNumber for inner, got {:?}", inner.kind),
    }

    // Back to 64-bit mode
    let back = exec.get_variable("back_to_outer").unwrap();
    match &back.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_u64().unwrap(), 4294967296);
        }
        ValueKind::Number(n) => {
            assert_eq!(*n as u64, 4294967296);
        }
        _ => panic!("Expected Number or BigNumber for back, got {:?}", back.kind),
    }
}

#[test]
fn test_32bit_sequential_operations() {
    let mut exec = Executor::new();
    let source = r#"
        configure { :unsigned, :32bit } {
            a = 2147483648  # 0x80000000
            b = 2147483648
            result = a + b  # Should wrap to 0
        }
    "#;
    exec.execute_source(source).unwrap();

    let result = exec.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_i64().unwrap(), 0);
        }
        _ => panic!("Expected BigNumber, got {:?}", result.kind),
    }
}

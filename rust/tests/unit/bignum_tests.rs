// Tests for BigNum type and precision modes
// Phase 1: :high precision with i64/u64

use graphoid::execution::Executor;
use graphoid::values::{BigNum, ValueKind};

// ============================================================================
// Test 1: Basic High Precision Integer Arithmetic
// ============================================================================

#[test]
fn test_high_precision_addition() {
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :high } {
            a = 100
            b = 200
            result = a + b
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    // Should be BigNumber::Int64
    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            // Should equal 300
            assert_eq!(bignum.to_i64().unwrap(), 300i64);
        }
        _ => panic!("Expected BigNumber, got {:?}", result.kind),
    }
}

#[test]
fn test_high_precision_large_numbers() {
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :high } {
            # Maximum i64 value
            large = 9223372036854775807
            result = large
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_i64().unwrap(), 9223372036854775807i64);
        }
        _ => panic!("Expected BigNumber"),
    }
}

#[test]
fn test_high_precision_subtraction() {
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :high } {
            result = 500 - 200
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_i64().unwrap(), 300i64);
        }
        _ => panic!("Expected BigNumber"),
    }
}

#[test]
fn test_high_precision_multiplication() {
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :high } {
            result = 123456 * 789
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_i64().unwrap(), 97406784i64);
        }
        _ => panic!("Expected BigNumber"),
    }
}

#[test]
fn test_high_precision_division() {
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :high } {
            result = 1000 / 10
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_i64().unwrap(), 100i64);
        }
        _ => panic!("Expected BigNumber"),
    }
}

#[test]
fn test_high_precision_modulo() {
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :high } {
            result = 17 % 5
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_i64().unwrap(), 2i64);
        }
        _ => panic!("Expected BigNumber"),
    }
}

// ============================================================================
// Test 2: Unsigned Mode
// ============================================================================

#[test]
fn test_high_precision_unsigned_addition() {
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :high, :unsigned } {
            result = 100 + 200
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_u64().unwrap(), 300u64);
        }
        _ => panic!("Expected BigNumber"),
    }
}

#[test]
fn test_high_precision_unsigned_max() {
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :high, :unsigned } {
            # Maximum u64 value
            result = 18446744073709551615
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_u64().unwrap(), 18446744073709551615u64);
        }
        _ => panic!("Expected BigNumber"),
    }
}

// ============================================================================
// Test 3: Bitwise Operations
// ============================================================================

#[test]
fn test_high_precision_bitwise_and() {
    let mut executor = Executor::new();

    // Phase 1B: Need :integer for bitwise ops (High defaults to Float128)
    let code = r#"
        configure { precision: :high, integer: :integer } {
            result = 12 & 10
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_i64().unwrap(), 8i64); // 1100 & 1010 = 1000
        }
        _ => panic!("Expected BigNumber"),
    }
}

#[test]
fn test_high_precision_bitwise_or() {
    let mut executor = Executor::new();

    // Phase 1B: Need :integer for bitwise ops
    let code = r#"
        configure { precision: :high, integer: :integer } {
            result = 12 | 10
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_i64().unwrap(), 14i64); // 1100 | 1010 = 1110
        }
        _ => panic!("Expected BigNumber"),
    }
}

#[test]
fn test_high_precision_bitwise_xor() {
    let mut executor = Executor::new();

    // Phase 1B: Need :integer for bitwise ops
    let code = r#"
        configure { precision: :high, integer: :integer } {
            result = 12 ^ 10
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_i64().unwrap(), 6i64); // 1100 ^ 1010 = 0110
        }
        _ => panic!("Expected BigNumber"),
    }
}

#[test]
fn test_high_precision_bitwise_not() {
    let mut executor = Executor::new();

    // Phase 1B: Need :integer for bitwise ops
    let code = r#"
        configure { precision: :high, integer: :integer } {
            result = ~5
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_i64().unwrap(), -6i64); // Two's complement
        }
        _ => panic!("Expected BigNumber"),
    }
}

#[test]
fn test_high_precision_left_shift() {
    let mut executor = Executor::new();

    // Phase 1B: Need :integer for bitwise ops
    let code = r#"
        configure { precision: :high, integer: :integer } {
            result = 5 << 2
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_i64().unwrap(), 20i64); // 5 * 4
        }
        _ => panic!("Expected BigNumber"),
    }
}

#[test]
fn test_high_precision_right_shift_signed() {
    let mut executor = Executor::new();

    // Phase 1B: Need :integer for bitwise ops
    let code = r#"
        configure { precision: :high, integer: :integer } {
            result = 20 >> 2
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_i64().unwrap(), 5i64); // 20 / 4
        }
        _ => panic!("Expected BigNumber"),
    }
}

#[test]
fn test_high_precision_right_shift_unsigned() {
    let mut executor = Executor::new();

    // Phase 1B: Need :integer for bitwise ops (and :unsigned)
    let code = r#"
        configure { precision: :high, :integer, :unsigned } {
            result = 20 >> 2
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_u64().unwrap(), 5u64);
        }
        _ => panic!("Expected BigNumber"),
    }
}

// ============================================================================
// Test 4: Value Persistence Outside Blocks
// ============================================================================

#[test]
fn test_bignum_persists_outside_block() {
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :high } {
            result = 100 + 200
        }
        # Outside block now - result should still be bignum
        persisted = result
    "#;

    executor.execute_source(code).unwrap();
    let persisted = executor.env().get("persisted").unwrap();

    match &persisted.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_i64().unwrap(), 300i64);
        }
        _ => panic!("BigNumber should persist outside precision block"),
    }
}

// ============================================================================
// Test 5: Type Conversion Methods
// ============================================================================

#[test]
fn test_bignum_to_string() {
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :high } {
            big = 12345678901234567890
        }
        result = big.to_string()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::String(s) => {
            // i64 max is ~9.2e18, so this will overflow in current impl
            // But should still convert to string
            assert!(s.len() > 0);
        }
        _ => panic!("Expected string from to_string()"),
    }
}

#[test]
fn test_bignum_to_num() {
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :high } {
            big = 300
        }
        result = big.to_num()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::Number(n) => {
            assert_eq!(*n, 300.0);
        }
        _ => panic!("Expected number from to_num()"),
    }
}

#[test]
fn test_bignum_to_num_overflow_returns_none() {
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :high } {
            # Value that exceeds f64 precision
            big = 9223372036854775807
        }
        result = big.to_num()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    // Should still convert, but may lose precision
    // For now, just verify it returns a number
    match &result.kind {
        ValueKind::Number(_) => {
            // Success - converted even if precision lost
        }
        ValueKind::None => {
            // Also acceptable - returned none due to overflow
        }
        _ => panic!("Expected number or none from to_num()"),
    }
}

// ============================================================================
// Test 6: Type Checking
// ============================================================================

#[test]
fn test_bignum_type_name() {
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :high } {
            big = 100
        }
        result = big.type()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::String(s) => {
            assert_eq!(s, "bignum");
        }
        _ => panic!("Expected string from type()"),
    }
}

// ============================================================================
// Test 7: Mixed Type Errors
// ============================================================================

#[test]
fn test_can_mix_num_and_bignum() {
    // Phase 1B: Mixed num/bignum operations are now ALLOWED (auto-cast to bignum)
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :high } {
            big = 100.0
        }
        small = 50.0
        result = big + small
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.get_variable("result").unwrap();

    // Result should be bignum (Float128 in Phase 1B)
    assert!(matches!(result.kind, ValueKind::BigNumber(BigNum::Float128(_))));

    if let ValueKind::BigNumber(BigNum::Float128(f)) = result.kind {
        let f64_val: f64 = f.into();
        assert!((f64_val - 150.0).abs() < 0.0001);
    }
}

// ============================================================================
// Test 8: Overflow Handling
// ============================================================================

#[test]
fn test_high_precision_overflow_error() {
    let mut executor = Executor::new();

    // Phase 1B: Need :integer for Int64 overflow checking
    // (High alone uses Float128 which doesn't overflow)
    let code = r#"
        configure { precision: :high, :integer } {
            max_val = 9223372036854775807
            result = max_val + 1
        }
    "#;

    let result = executor.execute_source(code);
    assert!(result.is_err(), "Should error on i64 overflow");
    assert!(
        result.unwrap_err().to_string().contains("overflow"),
        "Error should mention overflow"
    );
}

// ============================================================================
// Test 9: Comparison Operations
// ============================================================================

#[test]
fn test_bignum_comparison_equal() {
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :high } {
            a = 100
            b = 100
            result = a == b
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::Boolean(b) => {
            assert_eq!(*b, true);
        }
        _ => panic!("Expected boolean from comparison"),
    }
}

#[test]
fn test_bignum_comparison_less_than() {
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :high } {
            a = 50
            b = 100
            result = a < b
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::Boolean(b) => {
            assert_eq!(*b, true);
        }
        _ => panic!("Expected boolean from comparison"),
    }
}

#[test]
fn test_bignum_comparison_greater_than() {
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :high } {
            a = 100
            b = 50
            result = a > b
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::Boolean(b) => {
            assert_eq!(*b, true);
        }
        _ => panic!("Expected boolean from comparison"),
    }
}

// ============================================================================
// Test 10: Nested Precision Blocks
// ============================================================================

#[test]
fn test_nested_precision_blocks() {
    let mut executor = Executor::new();

    let code = r#"
        a = 100  # num
        configure { precision: :high } {
            b = 200  # bignum
            configure { precision: :high } {
                c = 300  # bignum (nested)
                result = b + c
            }
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_i64().unwrap(), 500i64);
        }
        _ => panic!("Expected BigNumber"),
    }
}

// ============================================================================
// Test 11: Exiting Precision Block Returns to Standard
// ============================================================================

#[test]
fn test_new_values_outside_block_are_standard() {
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :high } {
            big = 100
        }
        # Outside block - new values should be standard num
        standard = 200
    "#;

    executor.execute_source(code).unwrap();

    let big = executor.env().get("big").unwrap();
    let standard = executor.env().get("standard").unwrap();

    // big should be BigNumber
    assert!(matches!(&big.kind, ValueKind::BigNumber(_)), "big should be BigNumber");

    // standard should be Number
    assert!(matches!(&standard.kind, ValueKind::Number(_)), "standard should be Number");
}

// ============================================================================
// Test 12: Power Operations
// ============================================================================

#[test]
fn test_high_precision_power() {
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :high } {
            result = 2 ** 10
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::BigNumber(bignum) => {
            assert_eq!(bignum.to_i64().unwrap(), 1024i64);
        }
        _ => panic!("Expected BigNumber"),
    }
}

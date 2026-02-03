//! Arithmetic operations for the Graphoid executor.
//!
//! This module contains all arithmetic, bitwise, and comparison operations
//! extracted from the main executor for better code organization.

use crate::ast::BinaryOp;
use crate::execution::ErrorMode;
use crate::error::{GraphoidError, Result, SourcePosition};
use crate::execution::config::PrecisionMode;
use crate::execution::Executor;
use crate::values::{BigNum, List, Value, ValueKind};

impl Executor {
    // =========================================================================
    // BigNum Helper Methods
    // =========================================================================

    pub(crate) fn num_to_bignum_f128(&self, n: f64) -> BigNum {
        use f128::f128;
        BigNum::Float128(f128::from(n))
    }

    /// Phase 2: Checks if a result should be auto-promoted to bignum due to
    /// overflow or precision loss.
    ///
    /// Auto-promotion happens ONLY in Standard mode to prevent precision loss.
    /// High/Extended modes don't need this since they already use bignum types.
    pub(crate) fn should_promote_to_bignum(&self, left: f64, right: f64, result: f64) -> bool {
        // Only auto-promote in Standard mode (High/Extended already use bignum)
        if !matches!(self.config_stack.current().precision_mode, PrecisionMode::Standard) {
            return false;
        }

        // Check for infinity (overflow)
        if result.is_infinite() {
            return true;
        }

        // Check if result exceeds f64's exact integer representation range (2^53)
        // When result is this large, we've likely lost precision
        const F64_MAX_EXACT_INT: f64 = 9_007_199_254_740_992.0; // 2^53
        if result.is_finite() && result.abs() > F64_MAX_EXACT_INT {
            // For very large results, check if inputs appear to be integer-like
            // (This distinguishes integer arithmetic from intentional float operations)
            if left.fract() == 0.0 && right.fract() == 0.0 {
                return true;
            }
        }

        // Check for significant precision loss in the operation
        // If operands are large and result is suspiciously different, precision may be lost
        let max_operand = left.abs().max(right.abs());
        if max_operand > F64_MAX_EXACT_INT {
            return true;
        }

        false
    }

    /// Phase 2: Promotes a f64 result to bignum (Float128).
    pub(crate) fn promote_to_bignum(&self, result: f64) -> Value {
        use f128::f128;
        Value::bignum(BigNum::Float128(f128::from(result)))
    }

    /// Phase 3: Checks if integer operation should grow to BigInt due to overflow.
    /// This applies to Int64 and UInt64 operations in :integer mode with :high or :extended precision.
    pub(crate) fn should_grow_to_bigint_i64(&self, _left: i64, _right: i64, would_overflow: bool) -> bool {
        // Only auto-grow in :high or :extended precision modes
        if !matches!(
            self.config_stack.current().precision_mode,
            PrecisionMode::High | PrecisionMode::Extended
        ) {
            return false;
        }

        // Check if the operation would overflow
        would_overflow
    }

    /// Phase 3: Checks if UInt64 operation should grow to BigInt.
    pub(crate) fn should_grow_to_bigint_u64(&self, _left: u64, _right: u64, would_overflow: bool) -> bool {
        // Only auto-grow in :high or :extended precision modes
        if !matches!(
            self.config_stack.current().precision_mode,
            PrecisionMode::High | PrecisionMode::Extended
        ) {
            return false;
        }

        // Check if the operation would overflow
        would_overflow
    }

    /// Phase 3: Grows an Int64 operation to BigInt.
    pub(crate) fn grow_i64_to_bigint(&self, left: i64, right: i64, op: &str) -> Result<Value> {
        use num_bigint::BigInt;
        let left_big = BigInt::from(left);
        let right_big = BigInt::from(right);

        let result = match op {
            "add" => left_big + right_big,
            "mul" => left_big * right_big,
            _ => return Err(GraphoidError::runtime(format!("Unsupported operation for BigInt growth: {}", op))),
        };

        Ok(Value::bignum(BigNum::BigInt(result)))
    }

    /// Phase 3: Grows a UInt64 operation to BigInt.
    pub(crate) fn grow_u64_to_bigint(&self, left: u64, right: u64, op: &str) -> Result<Value> {
        use num_bigint::BigInt;
        let left_big = BigInt::from(left);
        let right_big = BigInt::from(right);

        let result = match op {
            "add" => left_big + right_big,
            "mul" => left_big * right_big,
            _ => return Err(GraphoidError::runtime(format!("Unsupported operation for BigInt growth: {}", op))),
        };

        Ok(Value::bignum(BigNum::BigInt(result)))
    }

    // =========================================================================
    // Element-wise and Scalar Operations
    // =========================================================================

    pub(crate) fn eval_element_wise(&mut self, left: Value, right: Value, base_op: BinaryOp) -> Result<Value> {
        match (&left.kind, &right.kind) {
            // List-List element-wise operation (zips to shorter length)
            (ValueKind::List(left_list), ValueKind::List(right_list)) => {
                let left_elements = left_list.to_vec();
                let right_elements = right_list.to_vec();

                // Apply operation element by element (zip stops at shorter length)
                let mut results = Vec::new();
                for (left_elem, right_elem) in left_elements.iter().zip(right_elements.iter()) {
                    let result = self.apply_scalar_op(left_elem.clone(), right_elem.clone(), &base_op)?;
                    results.push(result);
                }
                Ok(Value::list(List::from_vec(results)))
            }
            // List-Scalar element-wise operation (broadcast scalar)
            (ValueKind::List(list), _scalar) => {
                let elements = list.to_vec();
                let mut results = Vec::new();
                for elem in elements.iter() {
                    let result = self.apply_scalar_op(elem.clone(), right.clone(), &base_op)?;
                    results.push(result);
                }
                Ok(Value::list(List::from_vec(results)))
            }
            // Scalar-List element-wise operation (broadcast scalar)
            (_scalar, ValueKind::List(list)) => {
                let elements = list.to_vec();
                let mut results = Vec::new();
                for elem in elements.iter() {
                    let result = self.apply_scalar_op(left.clone(), elem.clone(), &base_op)?;
                    results.push(result);
                }
                Ok(Value::list(List::from_vec(results)))
            }
            // Scalar-Scalar: not element-wise, error
            (_, _) => Err(GraphoidError::runtime(format!(
                "Element-wise operations require at least one list, got {} and {}",
                left.type_name(),
                right.type_name()
            ))),
        }
    }

    /// Applies a scalar binary operation (used by element-wise operations).
    pub(crate) fn apply_scalar_op(&mut self, left: Value, right: Value, op: &BinaryOp) -> Result<Value> {
        match op {
            // Arithmetic operators
            BinaryOp::Add => self.eval_add(left, right),
            BinaryOp::Subtract => self.eval_subtract(left, right),
            BinaryOp::Multiply => self.eval_multiply(left, right),
            BinaryOp::Divide => self.eval_divide(left, right),
            BinaryOp::IntDiv => self.eval_int_div(left, right),
            BinaryOp::Modulo => self.eval_modulo(left, right),
            BinaryOp::Power => self.eval_power(left, right),
            // Comparison operators
            BinaryOp::Equal => Ok(Value::boolean(left == right)),
            BinaryOp::NotEqual => Ok(Value::boolean(left != right)),
            BinaryOp::Less => self.eval_less(left, right),
            BinaryOp::LessEqual => self.eval_less_equal(left, right),
            BinaryOp::Greater => self.eval_greater(left, right),
            BinaryOp::GreaterEqual => self.eval_greater_equal(left, right),
            _ => Err(GraphoidError::runtime(format!(
                "Unsupported scalar operation: {:?}",
                op
            ))),
        }
    }

    // Arithmetic helpers
    pub(crate) fn eval_add(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            // BigNumber + BigNumber
            (ValueKind::BigNumber(l), ValueKind::BigNumber(r)) => {
                match (l, r) {
                    (BigNum::Int64(lv), BigNum::Int64(rv)) => {
                        // In 32-bit mode: use wrapping arithmetic
                        // In 64-bit mode: use checked arithmetic (may grow to BigInt)
                        use crate::execution::config::BitWidth;
                        match self.config_stack.current().bit_width {
                            BitWidth::Bits32 => {
                                let result = lv.wrapping_add(*rv);
                                let wrapped = self.config_stack.current().wrap_value(result);
                                Ok(Value::bignum(BigNum::Int64(wrapped)))
                            }
                            BitWidth::Bits64 => {
                                match lv.checked_add(*rv) {
                                    Some(result) => Ok(Value::bignum(BigNum::Int64(result))),
                                    None => {
                                        // Phase 3: Check if we should auto-grow to BigInt
                                        if self.should_grow_to_bigint_i64(*lv, *rv, true) {
                                            self.grow_i64_to_bigint(*lv, *rv, "add")
                                        } else {
                                            Err(GraphoidError::runtime("Integer overflow in addition".to_string()))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    (BigNum::UInt64(lv), BigNum::UInt64(rv)) => {
                        use crate::execution::config::BitWidth;
                        match self.config_stack.current().bit_width {
                            BitWidth::Bits32 => {
                                let result = lv.wrapping_add(*rv);
                                let wrapped = self.config_stack.current().wrap_value(result as i64) as u64;
                                Ok(Value::bignum(BigNum::UInt64(wrapped)))
                            }
                            BitWidth::Bits64 => {
                                match lv.checked_add(*rv) {
                                    Some(result) => Ok(Value::bignum(BigNum::UInt64(result))),
                                    None => {
                                        // Phase 3: Check if we should auto-grow to BigInt
                                        if self.should_grow_to_bigint_u64(*lv, *rv, true) {
                                            self.grow_u64_to_bigint(*lv, *rv, "add")
                                        } else {
                                            Err(GraphoidError::runtime("Integer overflow in addition".to_string()))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    (BigNum::Float128(lv), BigNum::Float128(rv)) => {
                        // Phase 1B: Float128 addition
                        Ok(Value::bignum(BigNum::Float128(*lv + *rv)))
                    }
                    (BigNum::BigInt(lv), BigNum::BigInt(rv)) => {
                        // Phase 1B: BigInt addition
                        Ok(Value::bignum(BigNum::BigInt(lv + rv)))
                    }
                    _ => Err(GraphoidError::runtime(
                        "Cannot mix different bignum types (Int64, UInt64, Float128, BigInt)".to_string()
                    ))
                }
            }

            // Number + Number - check precision mode
            (ValueKind::Number(l), ValueKind::Number(r)) => {
                match self.config_stack.current().precision_mode {
                    PrecisionMode::High => {
                        // Phase 1B: High precision defaults to Float128
                        // Only becomes Int64/UInt64 if integer_mode is also active
                        if self.config_stack.current().integer_mode {
                            // Integer mode: convert to i64/u64 based on unsigned_mode
                            if self.config_stack.current().unsigned_mode {
                                let lv = *l as u64;
                                let rv = *r as u64;
                                // Use wrapping_add for 32-bit mode, checked_add for 64-bit
                                let result = lv.wrapping_add(rv);
                                let wrapped = self.config_stack.current().wrap_value(result as i64) as u64;
                                Ok(Value::bignum(BigNum::UInt64(wrapped)))
                            } else {
                                let lv = *l as i64;
                                let rv = *r as i64;
                                // Use wrapping_add for 32-bit mode, checked_add for 64-bit
                                let result = lv.wrapping_add(rv);
                                let wrapped = self.config_stack.current().wrap_value(result);
                                Ok(Value::bignum(BigNum::Int64(wrapped)))
                            }
                        } else {
                            // Float mode (default): convert to Float128
                            use f128::f128;
                            let lv = f128::from(*l);
                            let rv = f128::from(*r);
                            Ok(Value::bignum(BigNum::Float128(lv + rv)))
                        }
                    }
                    PrecisionMode::Extended => {
                        Err(GraphoidError::runtime(
                            "Extended precision (BigInt) not yet implemented".to_string()
                        ))
                    }
                    PrecisionMode::Standard => {
                        // Standard f64 arithmetic with optional auto-promotion
                        let result = l + r;

                        // Phase 2: Check if we should auto-promote due to overflow
                        if self.should_promote_to_bignum(*l, *r, result) {
                            Ok(self.promote_to_bignum(result))
                        } else {
                            Ok(Value::number(result))
                        }
                    }
                }
            }

            // String concatenation
            (ValueKind::String(_), _) | (_, ValueKind::String(_)) => {
                // If either operand is a string, convert both to strings and concatenate
                let left_str = left.to_string_value();
                let right_str = right.to_string_value();
                Ok(Value::string(format!("{}{}", left_str, right_str)))
            }

            // Phase 1B: Mixed num/bignum operations - auto-cast num to bignum (TEMPORARY)
            // CRITICAL: This creates a TEMPORARY bignum copy for the operation.
            // The original num variable is NOT mutated!
            (ValueKind::Number(n), ValueKind::BigNumber(_bn)) => {
                let n_bignum = self.num_to_bignum_f128(*n);
                self.eval_add(Value::bignum(n_bignum), right)
            }
            (ValueKind::BigNumber(_bn), ValueKind::Number(n)) => {
                let n_bignum = self.num_to_bignum_f128(*n);
                self.eval_add(left, Value::bignum(n_bignum))
            }

            (_l, _r) => Err(GraphoidError::type_error(
                "number or string",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    pub(crate) fn eval_subtract(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            // BigNumber - BigNumber
            (ValueKind::BigNumber(l), ValueKind::BigNumber(r)) => {
                match (l, r) {
                    (BigNum::Int64(lv), BigNum::Int64(rv)) => {
                        use crate::execution::config::BitWidth;
                        match self.config_stack.current().bit_width {
                            BitWidth::Bits32 => {
                                let result = lv.wrapping_sub(*rv);
                                let wrapped = self.config_stack.current().wrap_value(result);
                                Ok(Value::bignum(BigNum::Int64(wrapped)))
                            }
                            BitWidth::Bits64 => {
                                lv.checked_sub(*rv)
                                    .map(|result| Value::bignum(BigNum::Int64(result)))
                                    .ok_or_else(|| GraphoidError::runtime("Integer overflow in subtraction".to_string()))
                            }
                        }
                    }
                    (BigNum::UInt64(lv), BigNum::UInt64(rv)) => {
                        use crate::execution::config::BitWidth;
                        match self.config_stack.current().bit_width {
                            BitWidth::Bits32 => {
                                let result = lv.wrapping_sub(*rv);
                                let wrapped = self.config_stack.current().wrap_value(result as i64) as u64;
                                Ok(Value::bignum(BigNum::UInt64(wrapped)))
                            }
                            BitWidth::Bits64 => {
                                lv.checked_sub(*rv)
                                    .map(|result| Value::bignum(BigNum::UInt64(result)))
                                    .ok_or_else(|| GraphoidError::runtime("Integer overflow in subtraction".to_string()))
                            }
                        }
                    }
                    (BigNum::Float128(lv), BigNum::Float128(rv)) => {
                        // Phase 1B: Float128 subtraction
                        Ok(Value::bignum(BigNum::Float128(*lv - *rv)))
                    }
                    (BigNum::BigInt(lv), BigNum::BigInt(rv)) => {
                        // Phase 1B: BigInt subtraction
                        Ok(Value::bignum(BigNum::BigInt(lv - rv)))
                    }
                    _ => Err(GraphoidError::runtime(
                        "Cannot mix different bignum types (Int64, UInt64, Float128, BigInt)".to_string()
                    ))
                }
            }

            // Number - Number - check precision mode
            (ValueKind::Number(l), ValueKind::Number(r)) => {
                match self.config_stack.current().precision_mode {
                    PrecisionMode::High => {
                        // Phase 1B: High precision defaults to Float128
                        if self.config_stack.current().integer_mode {
                            // Integer mode
                            if self.config_stack.current().unsigned_mode {
                                let lv = *l as u64;
                                let rv = *r as u64;
                                let result = lv.wrapping_sub(rv);
                                let wrapped = self.config_stack.current().wrap_value(result as i64) as u64;
                                Ok(Value::bignum(BigNum::UInt64(wrapped)))
                            } else {
                                let lv = *l as i64;
                                let rv = *r as i64;
                                let result = lv.wrapping_sub(rv);
                                let wrapped = self.config_stack.current().wrap_value(result);
                                Ok(Value::bignum(BigNum::Int64(wrapped)))
                            }
                        } else {
                            // Float mode (default): Float128
                            use f128::f128;
                            let lv = f128::from(*l);
                            let rv = f128::from(*r);
                            Ok(Value::bignum(BigNum::Float128(lv - rv)))
                        }
                    }
                    PrecisionMode::Extended => {
                        Err(GraphoidError::runtime(
                            "Extended precision (BigInt) not yet implemented".to_string()
                        ))
                    }
                    PrecisionMode::Standard => {
                        Ok(Value::number(l - r))
                    }
                }
            }

            // Phase 1B: Mixed num/bignum - auto-cast to Float128
            (ValueKind::Number(n), ValueKind::BigNumber(_bn)) => {
                
                let n_bignum = self.num_to_bignum_f128(*n);
                self.eval_subtract(Value::bignum(n_bignum), right)
            }
            (ValueKind::BigNumber(_bn), ValueKind::Number(n)) => {
                
                let n_bignum = self.num_to_bignum_f128(*n);
                self.eval_subtract(left, Value::bignum(n_bignum))
            }

            (_l, _r) => Err(GraphoidError::type_error(
                "number",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    pub(crate) fn eval_multiply(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            // BigNumber * BigNumber
            (ValueKind::BigNumber(l), ValueKind::BigNumber(r)) => {
                match (l, r) {
                    (BigNum::Int64(lv), BigNum::Int64(rv)) => {
                        use crate::execution::config::BitWidth;
                        match self.config_stack.current().bit_width {
                            BitWidth::Bits32 => {
                                let result = lv.wrapping_mul(*rv);
                                let wrapped = self.config_stack.current().wrap_value(result);
                                Ok(Value::bignum(BigNum::Int64(wrapped)))
                            }
                            BitWidth::Bits64 => {
                                match lv.checked_mul(*rv) {
                                    Some(result) => Ok(Value::bignum(BigNum::Int64(result))),
                                    None => {
                                        if self.should_grow_to_bigint_i64(*lv, *rv, true) {
                                            self.grow_i64_to_bigint(*lv, *rv, "mul")
                                        } else {
                                            Err(GraphoidError::runtime("Integer overflow in multiplication".to_string()))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    (BigNum::UInt64(lv), BigNum::UInt64(rv)) => {
                        use crate::execution::config::BitWidth;
                        match self.config_stack.current().bit_width {
                            BitWidth::Bits32 => {
                                let result = lv.wrapping_mul(*rv);
                                let wrapped = self.config_stack.current().wrap_value(result as i64) as u64;
                                Ok(Value::bignum(BigNum::UInt64(wrapped)))
                            }
                            BitWidth::Bits64 => {
                                match lv.checked_mul(*rv) {
                                    Some(result) => Ok(Value::bignum(BigNum::UInt64(result))),
                                    None => {
                                        if self.should_grow_to_bigint_u64(*lv, *rv, true) {
                                            self.grow_u64_to_bigint(*lv, *rv, "mul")
                                        } else {
                                            Err(GraphoidError::runtime("Integer overflow in multiplication".to_string()))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    (BigNum::Float128(lv), BigNum::Float128(rv)) => {
                        // Phase 1B: Float128 multiplication
                        Ok(Value::bignum(BigNum::Float128(*lv * *rv)))
                    }
                    (BigNum::BigInt(lv), BigNum::BigInt(rv)) => {
                        // Phase 1B: BigInt multiplication
                        Ok(Value::bignum(BigNum::BigInt(lv * rv)))
                    }
                    _ => Err(GraphoidError::runtime(
                        "Cannot mix different bignum types (Int64, UInt64, Float128, BigInt)".to_string()
                    ))
                }
            }

            // Number * Number - check precision mode
            (ValueKind::Number(l), ValueKind::Number(r)) => {
                match self.config_stack.current().precision_mode {
                    PrecisionMode::High => {
                        // Phase 1B: High precision defaults to Float128
                        if self.config_stack.current().integer_mode {
                            if self.config_stack.current().unsigned_mode {
                                let lv = *l as u64;
                                let rv = *r as u64;
                                lv.checked_mul(rv)
                                    .map(|result| Value::bignum(BigNum::UInt64(result)))
                                    .ok_or_else(|| GraphoidError::runtime("Integer overflow in multiplication".to_string()))
                            } else {
                                let lv = *l as i64;
                                let rv = *r as i64;
                                lv.checked_mul(rv)
                                    .map(|result| Value::bignum(BigNum::Int64(result)))
                                    .ok_or_else(|| GraphoidError::runtime("Integer overflow in multiplication".to_string()))
                            }
                        } else {
                            // Float mode (default): Float128
                            use f128::f128;
                            let lv = f128::from(*l);
                            let rv = f128::from(*r);
                            Ok(Value::bignum(BigNum::Float128(lv * rv)))
                        }
                    }
                    PrecisionMode::Extended => {
                        Err(GraphoidError::runtime(
                            "Extended precision (BigInt) not yet implemented".to_string()
                        ))
                    }
                    PrecisionMode::Standard => {
                        // Standard f64 arithmetic with optional auto-promotion
                        let result = l * r;

                        // Phase 2: Check if we should auto-promote due to overflow
                        if self.should_promote_to_bignum(*l, *r, result) {
                            Ok(self.promote_to_bignum(result))
                        } else {
                            Ok(Value::number(result))
                        }
                    }
                }
            }

            // Phase 1B: Mixed num/bignum - auto-cast to Float128
            (ValueKind::Number(n), ValueKind::BigNumber(_bn)) => {
                
                let n_bignum = self.num_to_bignum_f128(*n);
                self.eval_multiply(Value::bignum(n_bignum), right)
            }
            (ValueKind::BigNumber(_bn), ValueKind::Number(n)) => {
                
                let n_bignum = self.num_to_bignum_f128(*n);
                self.eval_multiply(left, Value::bignum(n_bignum))
            }

            (_l, _r) => Err(GraphoidError::type_error(
                "number",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    pub(crate) fn eval_divide(&mut self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            // BigNumber / BigNumber
            (ValueKind::BigNumber(l), ValueKind::BigNumber(r)) => {
                match (l, r) {
                    (BigNum::Int64(lv), BigNum::Int64(rv)) => {
                        if *rv == 0 {
                            return Err(GraphoidError::division_by_zero());
                        }
                        lv.checked_div(*rv)
                            .map(|result| Value::bignum(BigNum::Int64(result)))
                            .ok_or_else(|| GraphoidError::runtime("Integer overflow in division".to_string()))
                    }
                    (BigNum::UInt64(lv), BigNum::UInt64(rv)) => {
                        if *rv == 0 {
                            return Err(GraphoidError::division_by_zero());
                        }
                        lv.checked_div(*rv)
                            .map(|result| Value::bignum(BigNum::UInt64(result)))
                            .ok_or_else(|| GraphoidError::runtime("Integer overflow in division".to_string()))
                    }
                    (BigNum::Float128(lv), BigNum::Float128(rv)) => {
                        // Phase 1B: Float128 division
                        let f64_rv: f64 = (*rv).into();
                        if f64_rv == 0.0 {
                            return Err(GraphoidError::division_by_zero());
                        }
                        Ok(Value::bignum(BigNum::Float128(*lv / *rv)))
                    }
                    (BigNum::BigInt(lv), BigNum::BigInt(rv)) => {
                        // Phase 1B: BigInt division
                        use num_traits::Zero;
                        if rv.is_zero() {
                            return Err(GraphoidError::division_by_zero());
                        }
                        Ok(Value::bignum(BigNum::BigInt(lv / rv)))
                    }
                    _ => Err(GraphoidError::runtime(
                        "Cannot mix different bignum types (Int64, UInt64, Float128, BigInt)".to_string()
                    ))
                }
            }

            // Number / Number - check precision mode
            (ValueKind::Number(l), ValueKind::Number(r)) => {
                match self.config_stack.current().precision_mode {
                    PrecisionMode::High => {
                        // Phase 1B: High precision defaults to Float128
                        if self.config_stack.current().integer_mode {
                            if self.config_stack.current().unsigned_mode {
                                let lv = *l as u64;
                                let rv = *r as u64;
                                if rv == 0 {
                                    return Err(GraphoidError::division_by_zero());
                                }
                                lv.checked_div(rv)
                                    .map(|result| Value::bignum(BigNum::UInt64(result)))
                                    .ok_or_else(|| GraphoidError::runtime("Integer overflow in division".to_string()))
                            } else {
                                let lv = *l as i64;
                                let rv = *r as i64;
                                if rv == 0 {
                                    return Err(GraphoidError::division_by_zero());
                                }
                                lv.checked_div(rv)
                                    .map(|result| Value::bignum(BigNum::Int64(result)))
                                    .ok_or_else(|| GraphoidError::runtime("Integer overflow in division".to_string()))
                            }
                        } else {
                            // Float mode (default): Float128
                            use f128::f128;
                            let lv = f128::from(*l);
                            let rv = f128::from(*r);
                            if rv == f128::from(0.0) {
                                return Err(GraphoidError::division_by_zero());
                            }
                            Ok(Value::bignum(BigNum::Float128(lv / rv)))
                        }
                    }
                    PrecisionMode::Extended => {
                        Err(GraphoidError::runtime(
                            "Extended precision (BigInt) not yet implemented".to_string()
                        ))
                    }
                    PrecisionMode::Standard => {
                        if *r == 0.0 {
                            // Check error mode
                            match self.config_stack.current().error_mode {
                                ErrorMode::Lenient => {
                                    // Return none in lenient mode
                                    return Ok(Value::none());
                                }
                                ErrorMode::Collect => {
                                    // Collect error and return none
                                    let error = GraphoidError::division_by_zero();
                                    self.error_collector.collect(
                                        error,
                                        self.current_file.as_ref().map(|p| p.to_string_lossy().to_string()),
                                        SourcePosition::unknown(),
                                    );
                                    return Ok(Value::none());
                                }
                                ErrorMode::Strict => {
                                    // Default behavior - raise error
                                    return Err(GraphoidError::division_by_zero());
                                }
                            }
                        } else {
                            Ok(Value::number(l / r))
                        }
                    }
                }
            }

            // Phase 1B: Mixed num/bignum - auto-cast to Float128
            (ValueKind::Number(n), ValueKind::BigNumber(_bn)) => {
                
                let n_bignum = self.num_to_bignum_f128(*n);
                self.eval_divide(Value::bignum(n_bignum), right)
            }
            (ValueKind::BigNumber(_bn), ValueKind::Number(n)) => {
                
                let n_bignum = self.num_to_bignum_f128(*n);
                self.eval_divide(left, Value::bignum(n_bignum))
            }

            (_l, _r) => Err(GraphoidError::type_error(
                "number",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    pub(crate) fn eval_int_div(&mut self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            // BigNumber // BigNumber
            (ValueKind::BigNumber(l), ValueKind::BigNumber(r)) => {
                match (l, r) {
                    (BigNum::Float128(lv), BigNum::Float128(rv)) => {
                        // Phase 1B: Float128 integer division
                        let f64_rv: f64 = (*rv).into();
                        if f64_rv == 0.0 {
                            return Err(GraphoidError::division_by_zero());
                        }
                        // Perform division and truncate toward zero
                        let result_f64: f64 = (*lv / *rv).into();
                        use f128::f128;
                        Ok(Value::bignum(BigNum::Float128(f128::from(result_f64.trunc()))))
                    }
                    (BigNum::BigInt(lv), BigNum::BigInt(rv)) => {
                        // Phase 1B: BigInt integer division
                        use num_traits::Zero;
                        if rv.is_zero() {
                            return Err(GraphoidError::division_by_zero());
                        }
                        Ok(Value::bignum(BigNum::BigInt(lv / rv)))
                    }
                    (BigNum::Int64(lv), BigNum::Int64(rv)) => {
                        if *rv == 0 {
                            return Err(GraphoidError::division_by_zero());
                        }
                        lv.checked_div(*rv)
                            .map(|result| Value::bignum(BigNum::Int64(result)))
                            .ok_or_else(|| GraphoidError::runtime("Integer overflow in integer division".to_string()))
                    }
                    (BigNum::UInt64(lv), BigNum::UInt64(rv)) => {
                        if *rv == 0 {
                            return Err(GraphoidError::division_by_zero());
                        }
                        lv.checked_div(*rv)
                            .map(|result| Value::bignum(BigNum::UInt64(result)))
                            .ok_or_else(|| GraphoidError::runtime("Integer overflow in integer division".to_string()))
                    }
                    _ => Err(GraphoidError::runtime(
                        "Cannot mix different bignum types (Int64, UInt64, Float128, BigInt)".to_string()
                    ))
                }
            }
            (ValueKind::Number(l), ValueKind::Number(r)) => {
                match self.config_stack.current().precision_mode {
                    PrecisionMode::High => {
                        // Phase 1B: High precision defaults to Float128
                        if self.config_stack.current().integer_mode {
                            // Integer mode: Int64/UInt64 integer division
                            if self.config_stack.current().unsigned_mode {
                                let lv = *l as u64;
                                let rv = *r as u64;
                                if rv == 0 {
                                    return Err(GraphoidError::division_by_zero());
                                }
                                lv.checked_div(rv)
                                    .map(|result| Value::bignum(BigNum::UInt64(result)))
                                    .ok_or_else(|| GraphoidError::runtime("Integer overflow in integer division".to_string()))
                            } else {
                                let lv = *l as i64;
                                let rv = *r as i64;
                                if rv == 0 {
                                    return Err(GraphoidError::division_by_zero());
                                }
                                lv.checked_div(rv)
                                    .map(|result| Value::bignum(BigNum::Int64(result)))
                                    .ok_or_else(|| GraphoidError::runtime("Integer overflow in integer division".to_string()))
                            }
                        } else {
                            // Float mode (default): Float128 with truncation
                            use f128::f128;
                            let lv = f128::from(*l);
                            let rv = f128::from(*r);
                            let f64_rv: f64 = rv.into();
                            if f64_rv == 0.0 {
                                return Err(GraphoidError::division_by_zero());
                            }
                            // Perform division and truncate toward zero
                            let result_f64: f64 = (lv / rv).into();
                            Ok(Value::bignum(BigNum::Float128(f128::from(result_f64.trunc()))))
                        }
                    }
                    PrecisionMode::Standard => {
                        if *r == 0.0 {
                            // Check error mode
                            match self.config_stack.current().error_mode {
                                ErrorMode::Lenient => {
                                    return Ok(Value::none());
                                }
                                ErrorMode::Collect => {
                                    let error = GraphoidError::division_by_zero();
                                    self.error_collector.collect(
                                        error,
                                        self.current_file.as_ref().map(|p| p.to_string_lossy().to_string()),
                                        SourcePosition::unknown(),
                                    );
                                    return Ok(Value::none());
                                }
                                ErrorMode::Strict => {
                                    return Err(GraphoidError::division_by_zero());
                                }
                            }
                        } else {
                            // Truncate toward zero (not floor)
                            Ok(Value::number((l / r).trunc()))
                        }
                    }
                    PrecisionMode::Extended => {
                        // Phase 1B: Extended precision uses BigInt (future)
                        use num_bigint::BigInt;
                        let lv = BigInt::from(*l as i64);
                        let rv = BigInt::from(*r as i64);
                        use num_traits::Zero;
                        if rv.is_zero() {
                            return Err(GraphoidError::division_by_zero());
                        }
                        Ok(Value::bignum(BigNum::BigInt(&lv / &rv)))
                    }
                }
            }

            // Phase 1B: Mixed num/bignum - auto-cast num to Float128 temporarily
            (ValueKind::Number(n), ValueKind::BigNumber(_bn)) => {
                
                let n_bignum = self.num_to_bignum_f128(*n);
                self.eval_int_div(Value::bignum(n_bignum), right)
            }
            (ValueKind::BigNumber(_bn), ValueKind::Number(n)) => {
                
                let n_bignum = self.num_to_bignum_f128(*n);
                self.eval_int_div(left, Value::bignum(n_bignum))
            }

            (_l, _r) => Err(GraphoidError::type_error(
                "number or bignum",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    pub(crate) fn eval_modulo(&mut self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            // BigNumber % BigNumber
            (ValueKind::BigNumber(l), ValueKind::BigNumber(r)) => {
                match (l, r) {
                    (BigNum::Int64(lv), BigNum::Int64(rv)) => {
                        if *rv == 0 {
                            return Err(GraphoidError::runtime("Modulo by zero".to_string()));
                        }
                        lv.checked_rem(*rv)
                            .map(|result| Value::bignum(BigNum::Int64(result)))
                            .ok_or_else(|| GraphoidError::runtime("Integer overflow in modulo".to_string()))
                    }
                    (BigNum::UInt64(lv), BigNum::UInt64(rv)) => {
                        if *rv == 0 {
                            return Err(GraphoidError::runtime("Modulo by zero".to_string()));
                        }
                        lv.checked_rem(*rv)
                            .map(|result| Value::bignum(BigNum::UInt64(result)))
                            .ok_or_else(|| GraphoidError::runtime("Integer overflow in modulo".to_string()))
                    }
                    (BigNum::Float128(lv), BigNum::Float128(rv)) => {
                        // Phase 1B: Float128 modulo
                        let f64_rv: f64 = (*rv).into();
                        if f64_rv == 0.0 {
                            return Err(GraphoidError::runtime("Modulo by zero".to_string()));
                        }
                        Ok(Value::bignum(BigNum::Float128(*lv % *rv)))
                    }
                    (BigNum::BigInt(lv), BigNum::BigInt(rv)) => {
                        // Phase 1B: BigInt modulo
                        use num_traits::Zero;
                        if rv.is_zero() {
                            return Err(GraphoidError::runtime("Modulo by zero".to_string()));
                        }
                        Ok(Value::bignum(BigNum::BigInt(lv % rv)))
                    }
                    _ => Err(GraphoidError::runtime(
                        "Cannot mix different bignum types (Int64, UInt64, Float128, BigInt)".to_string()
                    ))
                }
            }

            // Number % Number - check precision mode
            (ValueKind::Number(l), ValueKind::Number(r)) => {
                match self.config_stack.current().precision_mode {
                    PrecisionMode::High => {
                        // Phase 1B: High precision defaults to Float128
                        if self.config_stack.current().integer_mode {
                            if self.config_stack.current().unsigned_mode {
                                let lv = *l as u64;
                                let rv = *r as u64;
                                if rv == 0 {
                                    return Err(GraphoidError::runtime("Modulo by zero".to_string()));
                                }
                                lv.checked_rem(rv)
                                    .map(|result| Value::bignum(BigNum::UInt64(result)))
                                    .ok_or_else(|| GraphoidError::runtime("Integer overflow in modulo".to_string()))
                            } else {
                                let lv = *l as i64;
                                let rv = *r as i64;
                                if rv == 0 {
                                    return Err(GraphoidError::runtime("Modulo by zero".to_string()));
                                }
                                lv.checked_rem(rv)
                                    .map(|result| Value::bignum(BigNum::Int64(result)))
                                    .ok_or_else(|| GraphoidError::runtime("Integer overflow in modulo".to_string()))
                            }
                        } else {
                            // Float mode (default): Float128
                            use f128::f128;
                            let lv = f128::from(*l);
                            let rv = f128::from(*r);
                            if rv == f128::from(0.0) {
                                return Err(GraphoidError::runtime("Modulo by zero".to_string()));
                            }
                            Ok(Value::bignum(BigNum::Float128(lv % rv)))
                        }
                    }
                    PrecisionMode::Extended => {
                        Err(GraphoidError::runtime(
                            "Extended precision (BigInt) not yet implemented".to_string()
                        ))
                    }
                    PrecisionMode::Standard => {
                        if *r == 0.0 {
                            // Check error mode for modulo by zero
                            match self.config_stack.current().error_mode {
                                ErrorMode::Lenient => {
                                    return Ok(Value::none());
                                }
                                ErrorMode::Collect => {
                                    let error = GraphoidError::runtime("Modulo by zero".to_string());
                                    self.error_collector.collect(
                                        error,
                                        self.current_file.as_ref().map(|p| p.to_string_lossy().to_string()),
                                        SourcePosition::unknown(),
                                    );
                                    return Ok(Value::none());
                                }
                                ErrorMode::Strict => {
                                    return Err(GraphoidError::runtime("Modulo by zero".to_string()));
                                }
                            }
                        } else {
                            Ok(Value::number(l % r))
                        }
                    }
                }
            }

            // Phase 1B: Mixed num/bignum - auto-cast to Float128
            (ValueKind::Number(n), ValueKind::BigNumber(_bn)) => {
                
                let n_bignum = self.num_to_bignum_f128(*n);
                self.eval_modulo(Value::bignum(n_bignum), right)
            }
            (ValueKind::BigNumber(_bn), ValueKind::Number(n)) => {
                
                let n_bignum = self.num_to_bignum_f128(*n);
                self.eval_modulo(left, Value::bignum(n_bignum))
            }

            (_l, _r) => Err(GraphoidError::type_error(
                "number",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    pub(crate) fn eval_power(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            // BigNumber ** BigNumber
            (ValueKind::BigNumber(l), ValueKind::BigNumber(r)) => {
                match (l, r) {
                    (BigNum::Int64(lv), BigNum::Int64(rv)) => {
                        if *rv < 0 {
                            return Err(GraphoidError::runtime("Negative exponents not supported for integer power".to_string()));
                        }
                        if *rv > u32::MAX as i64 {
                            return Err(GraphoidError::runtime("Exponent too large".to_string()));
                        }
                        lv.checked_pow(*rv as u32)
                            .map(|result| Value::bignum(BigNum::Int64(result)))
                            .ok_or_else(|| GraphoidError::runtime("Integer overflow in power operation".to_string()))
                    }
                    (BigNum::UInt64(lv), BigNum::UInt64(rv)) => {
                        if *rv > u32::MAX as u64 {
                            return Err(GraphoidError::runtime("Exponent too large".to_string()));
                        }
                        lv.checked_pow(*rv as u32)
                            .map(|result| Value::bignum(BigNum::UInt64(result)))
                            .ok_or_else(|| GraphoidError::runtime("Integer overflow in power operation".to_string()))
                    }
                    (BigNum::Float128(lv), BigNum::Float128(rv)) => {
                        // Phase 1B: Float128 power
                        // Convert to f64 for power calculation (f128 doesn't have powf)
                        let lv_f64: f64 = (*lv).into();
                        let rv_f64: f64 = (*rv).into();
                        let result = lv_f64.powf(rv_f64);
                        use f128::f128;
                        Ok(Value::bignum(BigNum::Float128(f128::from(result))))
                    }
                    (BigNum::BigInt(lv), BigNum::BigInt(rv)) => {
                        // Phase 1B: BigInt power
                        use num_traits::ToPrimitive;
                        let exp_usize = rv.to_usize().ok_or_else(||
                            GraphoidError::runtime("Exponent too large or negative for BigInt power".to_string())
                        )?;
                        use num_traits::pow;
                        Ok(Value::bignum(BigNum::BigInt(pow::pow(lv.clone(), exp_usize))))
                    }
                    _ => Err(GraphoidError::runtime(
                        "Cannot mix different bignum types (Int64, UInt64, Float128, BigInt)".to_string()
                    ))
                }
            }

            // Number ** Number - check precision mode
            (ValueKind::Number(l), ValueKind::Number(r)) => {
                match self.config_stack.current().precision_mode {
                    PrecisionMode::High => {
                        // Phase 1B: High precision defaults to Float128
                        if self.config_stack.current().integer_mode {
                            if self.config_stack.current().unsigned_mode {
                                let lv = *l as u64;
                                let rv = *r;
                                if rv < 0.0 {
                                    return Err(GraphoidError::runtime("Negative exponents not supported for integer power".to_string()));
                                }
                                if rv > u32::MAX as f64 {
                                    return Err(GraphoidError::runtime("Exponent too large".to_string()));
                                }
                                lv.checked_pow(rv as u32)
                                    .map(|result| Value::bignum(BigNum::UInt64(result)))
                                    .ok_or_else(|| GraphoidError::runtime("Integer overflow in power operation".to_string()))
                            } else {
                                let lv = *l as i64;
                                let rv = *r;
                                if rv < 0.0 {
                                    return Err(GraphoidError::runtime("Negative exponents not supported for integer power".to_string()));
                                }
                                if rv > u32::MAX as f64 {
                                    return Err(GraphoidError::runtime("Exponent too large".to_string()));
                                }
                                lv.checked_pow(rv as u32)
                                    .map(|result| Value::bignum(BigNum::Int64(result)))
                                    .ok_or_else(|| GraphoidError::runtime("Integer overflow in power operation".to_string()))
                            }
                        } else {
                            // Float mode (default): Float128
                            use f128::f128;
                            let lv = f128::from(*l);
                            let rv = f128::from(*r);
                            // Convert to f64 for powf (f128 doesn't have powf)
                            let lv_f64: f64 = lv.into();
                            let rv_f64: f64 = rv.into();
                            let result = lv_f64.powf(rv_f64);
                            Ok(Value::bignum(BigNum::Float128(f128::from(result))))
                        }
                    }
                    PrecisionMode::Extended => {
                        Err(GraphoidError::runtime(
                            "Extended precision (BigInt) not yet implemented".to_string()
                        ))
                    }
                    PrecisionMode::Standard => {
                        // Standard f64 arithmetic with optional auto-promotion
                        let result = l.powf(*r);

                        // Phase 2: Check if we should auto-promote due to overflow
                        if self.should_promote_to_bignum(*l, *r, result) {
                            Ok(self.promote_to_bignum(result))
                        } else {
                            Ok(Value::number(result))
                        }
                    }
                }
            }

            // Phase 1B: Mixed num/bignum - auto-cast to Float128
            (ValueKind::Number(n), ValueKind::BigNumber(_bn)) => {
                
                let n_bignum = self.num_to_bignum_f128(*n);
                self.eval_power(Value::bignum(n_bignum), right)
            }
            (ValueKind::BigNumber(_bn), ValueKind::Number(n)) => {
                
                let n_bignum = self.num_to_bignum_f128(*n);
                self.eval_power(left, Value::bignum(n_bignum))
            }

            (_l, _r) => Err(GraphoidError::type_error(
                "number",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    // Bitwise operation helpers (Phase 13)

    pub(crate) fn eval_bitwise_and(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            // BigNumber & BigNumber
            (ValueKind::BigNumber(l), ValueKind::BigNumber(r)) => {
                match (l, r) {
                    (BigNum::Int64(lv), BigNum::Int64(rv)) => {
                        let result = lv & rv;
                        let wrapped = self.config_stack.current().wrap_value(result);
                        Ok(Value::bignum(BigNum::Int64(wrapped)))
                    }
                    (BigNum::UInt64(lv), BigNum::UInt64(rv)) => {
                        let result = lv & rv;
                        let wrapped = self.config_stack.current().wrap_value(result as i64) as u64;
                        Ok(Value::bignum(BigNum::UInt64(wrapped)))
                    }
                    _ => Err(GraphoidError::runtime(
                        "Cannot mix signed and unsigned bignum values".to_string()
                    ))
                }
            }

            // Number & Number - check precision mode
            (ValueKind::Number(l), ValueKind::Number(r)) => {
                let l_int = l.trunc() as i64;
                let r_int = r.trunc() as i64;
                let result = l_int & r_int;
                let wrapped = self.config_stack.current().wrap_value(result);

                match self.config_stack.current().precision_mode {
                    PrecisionMode::High => {
                        if self.config_stack.current().unsigned_mode {
                            Ok(Value::bignum(BigNum::UInt64(wrapped as u64)))
                        } else {
                            Ok(Value::bignum(BigNum::Int64(wrapped)))
                        }
                    }
                    _ => Ok(Value::number(wrapped as f64))
                }
            }

            // Mixed num/bignum error
            (ValueKind::Number(_), ValueKind::BigNumber(_)) |
            (ValueKind::BigNumber(_), ValueKind::Number(_)) => {
                Err(GraphoidError::runtime(
                    "Cannot mix num and bignum types without explicit conversion".to_string()
                ))
            }

            (_l, _r) => Err(GraphoidError::type_error(
                "number or bignum",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    pub(crate) fn eval_bitwise_or(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            // BigNumber | BigNumber
            (ValueKind::BigNumber(l), ValueKind::BigNumber(r)) => {
                match (l, r) {
                    (BigNum::Int64(lv), BigNum::Int64(rv)) => {
                        let result = lv | rv;
                        let wrapped = self.config_stack.current().wrap_value(result);
                        Ok(Value::bignum(BigNum::Int64(wrapped)))
                    }
                    (BigNum::UInt64(lv), BigNum::UInt64(rv)) => {
                        let result = lv | rv;
                        let wrapped = self.config_stack.current().wrap_value(result as i64) as u64;
                        Ok(Value::bignum(BigNum::UInt64(wrapped)))
                    }
                    _ => Err(GraphoidError::runtime(
                        "Cannot mix signed and unsigned bignum values".to_string()
                    ))
                }
            }

            // Number | Number - check precision mode
            (ValueKind::Number(l), ValueKind::Number(r)) => {
                let l_int = l.trunc() as i64;
                let r_int = r.trunc() as i64;
                let result = l_int | r_int;
                let wrapped = self.config_stack.current().wrap_value(result);

                match self.config_stack.current().precision_mode {
                    PrecisionMode::High => {
                        if self.config_stack.current().unsigned_mode {
                            Ok(Value::bignum(BigNum::UInt64(wrapped as u64)))
                        } else {
                            Ok(Value::bignum(BigNum::Int64(wrapped)))
                        }
                    }
                    _ => Ok(Value::number(wrapped as f64))
                }
            }

            // Mixed num/bignum error
            (ValueKind::Number(_), ValueKind::BigNumber(_)) |
            (ValueKind::BigNumber(_), ValueKind::Number(_)) => {
                Err(GraphoidError::runtime(
                    "Cannot mix num and bignum types without explicit conversion".to_string()
                ))
            }

            (_l, _r) => Err(GraphoidError::type_error(
                "number or bignum",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    pub(crate) fn eval_bitwise_xor(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            // BigNumber ^ BigNumber
            (ValueKind::BigNumber(l), ValueKind::BigNumber(r)) => {
                match (l, r) {
                    (BigNum::Int64(lv), BigNum::Int64(rv)) => {
                        let result = lv ^ rv;
                        let wrapped = self.config_stack.current().wrap_value(result);
                        Ok(Value::bignum(BigNum::Int64(wrapped)))
                    }
                    (BigNum::UInt64(lv), BigNum::UInt64(rv)) => {
                        let result = lv ^ rv;
                        let wrapped = self.config_stack.current().wrap_value(result as i64) as u64;
                        Ok(Value::bignum(BigNum::UInt64(wrapped)))
                    }
                    _ => Err(GraphoidError::runtime(
                        "Cannot mix signed and unsigned bignum values".to_string()
                    ))
                }
            }

            // Number ^ Number - check precision mode
            (ValueKind::Number(l), ValueKind::Number(r)) => {
                let l_int = l.trunc() as i64;
                let r_int = r.trunc() as i64;
                let result = l_int ^ r_int;
                let wrapped = self.config_stack.current().wrap_value(result);

                match self.config_stack.current().precision_mode {
                    PrecisionMode::High => {
                        if self.config_stack.current().unsigned_mode {
                            Ok(Value::bignum(BigNum::UInt64(wrapped as u64)))
                        } else {
                            Ok(Value::bignum(BigNum::Int64(wrapped)))
                        }
                    }
                    _ => Ok(Value::number(wrapped as f64))
                }
            }

            // Mixed num/bignum error
            (ValueKind::Number(_), ValueKind::BigNumber(_)) |
            (ValueKind::BigNumber(_), ValueKind::Number(_)) => {
                Err(GraphoidError::runtime(
                    "Cannot mix num and bignum types without explicit conversion".to_string()
                ))
            }

            (_l, _r) => Err(GraphoidError::type_error(
                "number or bignum",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    pub(crate) fn eval_left_shift(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            // BigNumber << BigNumber/Number
            (ValueKind::BigNumber(l), ValueKind::BigNumber(_)) |
            (ValueKind::BigNumber(l), ValueKind::Number(_)) => {
                // Get shift amount
                let shift_amount = match &right.kind {
                    ValueKind::BigNumber(bn) => bn.to_i64().ok_or_else(||
                        GraphoidError::runtime("Shift amount out of range".to_string())
                    )?,
                    ValueKind::Number(n) => *n as i64,
                    _ => unreachable!(),
                };

                if shift_amount < 0 || shift_amount >= 64 {
                    return Err(GraphoidError::runtime(format!(
                        "Shift amount {} out of range (0-63)", shift_amount
                    )));
                }

                match l {
                    BigNum::Int64(lv) => {
                        let result = lv.wrapping_shl(shift_amount as u32);
                        let wrapped = self.config_stack.current().wrap_value(result);
                        Ok(Value::bignum(BigNum::Int64(wrapped)))
                    }
                    BigNum::UInt64(lv) => {
                        let result = lv.wrapping_shl(shift_amount as u32);
                        let wrapped = self.config_stack.current().wrap_value(result as i64) as u64;
                        Ok(Value::bignum(BigNum::UInt64(wrapped)))
                    }
                    BigNum::Float128(_) => {
                        Err(GraphoidError::runtime("Cannot left shift floating-point bignum".to_string()))
                    }
                    BigNum::BigInt(bi) => {
                        use num_traits::ToPrimitive;
                        let shift_u32 = (shift_amount as u32).to_usize().ok_or_else(||
                            GraphoidError::runtime("Shift amount out of range".to_string())
                        )?;
                        Ok(Value::bignum(BigNum::BigInt(bi << shift_u32)))
                    }
                }
            }

            // Number << Number - check precision mode
            (ValueKind::Number(l), ValueKind::Number(r)) => {
                let r_int = r.trunc() as u32;

                if r_int >= 64 {
                    return Err(GraphoidError::runtime(format!(
                        "Shift amount {} too large (max 63)", r_int
                    )));
                }

                let l_int = l.trunc() as i64;
                let result = l_int << r_int;
                let wrapped = self.config_stack.current().wrap_value(result);

                match self.config_stack.current().precision_mode {
                    PrecisionMode::High => {
                        if self.config_stack.current().unsigned_mode {
                            Ok(Value::bignum(BigNum::UInt64(wrapped as u64)))
                        } else {
                            Ok(Value::bignum(BigNum::Int64(wrapped)))
                        }
                    }
                    _ => Ok(Value::number(wrapped as f64))
                }
            }

            // Number << BigNumber (allow for convenience)
            (ValueKind::Number(l), ValueKind::BigNumber(r)) => {
                let shift_amount = r.to_i64().ok_or_else(||
                    GraphoidError::runtime("Shift amount out of range".to_string())
                )?;

                if shift_amount < 0 || shift_amount >= 64 {
                    return Err(GraphoidError::runtime(format!(
                        "Shift amount {} out of range (0-63)", shift_amount
                    )));
                }

                let l_int = l.trunc() as i64;
                let result = l_int << (shift_amount as u32);

                match self.config_stack.current().precision_mode {
                    PrecisionMode::High => {
                        if self.config_stack.current().unsigned_mode {
                            Ok(Value::bignum(BigNum::UInt64(result as u64)))
                        } else {
                            Ok(Value::bignum(BigNum::Int64(result)))
                        }
                    }
                    _ => Ok(Value::number(result as f64))
                }
            }

            (_l, _r) => Err(GraphoidError::type_error(
                "number or bignum",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    pub(crate) fn eval_right_shift(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            // BigNumber >> BigNumber/Number
            (ValueKind::BigNumber(l), ValueKind::BigNumber(_)) |
            (ValueKind::BigNumber(l), ValueKind::Number(_)) => {
                // Get shift amount
                let shift_amount = match &right.kind {
                    ValueKind::BigNumber(bn) => bn.to_i64().ok_or_else(||
                        GraphoidError::runtime("Shift amount out of range".to_string())
                    )?,
                    ValueKind::Number(n) => *n as i64,
                    _ => unreachable!(),
                };

                if shift_amount < 0 || shift_amount >= 64 {
                    return Err(GraphoidError::runtime(format!(
                        "Shift amount {} out of range (0-63)", shift_amount
                    )));
                }

                match l {
                    BigNum::Int64(lv) => {
                        // Arithmetic right shift for signed
                        let result = lv >> (shift_amount as u32);
                        let wrapped = self.config_stack.current().wrap_value(result);
                        Ok(Value::bignum(BigNum::Int64(wrapped)))
                    }
                    BigNum::UInt64(lv) => {
                        // Logical right shift for unsigned
                        let result = lv >> (shift_amount as u32);
                        let wrapped = self.config_stack.current().wrap_value(result as i64) as u64;
                        Ok(Value::bignum(BigNum::UInt64(wrapped)))
                    }
                    BigNum::Float128(_) => {
                        Err(GraphoidError::runtime("Cannot right shift floating-point bignum".to_string()))
                    }
                    BigNum::BigInt(bi) => {
                        use num_traits::ToPrimitive;
                        let shift_u32 = (shift_amount as u32).to_usize().ok_or_else(||
                            GraphoidError::runtime("Shift amount out of range".to_string())
                        )?;
                        Ok(Value::bignum(BigNum::BigInt(bi >> shift_u32)))
                    }
                }
            }

            // Number >> Number - check precision mode
            (ValueKind::Number(l), ValueKind::Number(r)) => {
                let r_int = r.trunc() as u32;

                if r_int >= 64 {
                    return Err(GraphoidError::runtime(format!(
                        "Shift amount {} too large (max 63)", r_int
                    )));
                }

                // Check configuration for unsigned mode and precision mode
                match self.config_stack.current().precision_mode {
                    PrecisionMode::High => {
                        if self.config_stack.current().unsigned_mode {
                            let l_uint = (*l as i64) as u64;
                            let result = l_uint >> r_int;
                            let wrapped = self.config_stack.current().wrap_value(result as i64) as u64;
                            Ok(Value::bignum(BigNum::UInt64(wrapped)))
                        } else {
                            let l_int = l.trunc() as i64;
                            let result = l_int >> r_int;
                            let wrapped = self.config_stack.current().wrap_value(result);
                            Ok(Value::bignum(BigNum::Int64(wrapped)))
                        }
                    }
                    _ => {
                        let result = if self.config_stack.current().unsigned_mode {
                            let l_int = l.trunc() as i64;
                            let l_uint = l_int as u64;
                            let shifted = l_uint >> r_int;
                            let wrapped = self.config_stack.current().wrap_value(shifted as i64) as u64;
                            wrapped as f64
                        } else {
                            let l_int = l.trunc() as i64;
                            let shifted = l_int >> r_int;
                            let wrapped = self.config_stack.current().wrap_value(shifted);
                            wrapped as f64
                        };
                        Ok(Value::number(result))
                    }
                }
            }

            // Number >> BigNumber (allow for convenience)
            (ValueKind::Number(l), ValueKind::BigNumber(r)) => {
                let shift_amount = r.to_i64().ok_or_else(||
                    GraphoidError::runtime("Shift amount out of range".to_string())
                )?;

                if shift_amount < 0 || shift_amount >= 64 {
                    return Err(GraphoidError::runtime(format!(
                        "Shift amount {} out of range (0-63)", shift_amount
                    )));
                }

                match self.config_stack.current().precision_mode {
                    PrecisionMode::High => {
                        if self.config_stack.current().unsigned_mode {
                            let l_uint = (*l as i64) as u64;
                            Ok(Value::bignum(BigNum::UInt64(l_uint >> (shift_amount as u32))))
                        } else {
                            let l_int = l.trunc() as i64;
                            Ok(Value::bignum(BigNum::Int64(l_int >> (shift_amount as u32))))
                        }
                    }
                    _ => {
                        let result = if self.config_stack.current().unsigned_mode {
                            let l_int = l.trunc() as i64;
                            let l_uint = l_int as u64;
                            (l_uint >> (shift_amount as u32)) as i64 as f64
                        } else {
                            let l_int = l.trunc() as i64;
                            (l_int >> (shift_amount as u32)) as f64
                        };
                        Ok(Value::number(result))
                    }
                }
            }

            (_l, _r) => Err(GraphoidError::type_error(
                "number or bignum",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn eval_bitwise_not(&self, val: Value) -> Result<Value> {
        match &val.kind {
            ValueKind::BigNumber(bn) => {
                match bn {
                    BigNum::Int64(v) => Ok(Value::bignum(BigNum::Int64(!v))),
                    BigNum::UInt64(v) => Ok(Value::bignum(BigNum::UInt64(!v))),
                    BigNum::Float128(_) => {
                        Err(GraphoidError::runtime("Cannot apply bitwise NOT to floating-point bignum".to_string()))
                    }
                    BigNum::BigInt(bi) => {
                        // Bitwise NOT on BigInt - flip all bits
                        Ok(Value::bignum(BigNum::BigInt(!bi)))
                    }
                }
            }
            ValueKind::Number(n) => {
                let n_int = n.trunc() as i64;
                let result = !n_int;

                match self.config_stack.current().precision_mode {
                    PrecisionMode::High => {
                        if self.config_stack.current().unsigned_mode {
                            Ok(Value::bignum(BigNum::UInt64(result as u64)))
                        } else {
                            Ok(Value::bignum(BigNum::Int64(result)))
                        }
                    }
                    _ => Ok(Value::number(result as f64))
                }
            }
            _ => Err(GraphoidError::type_error("number or bignum", val.type_name())),
        }
    }

    // Helper: Convert BigNum to f64 for comparison (may lose precision for very large numbers)
    pub(crate) fn bignum_to_f64(&self, bn: &BigNum) -> Result<f64> {
        match bn {
            BigNum::Int64(v) => Ok(*v as f64),
            BigNum::UInt64(v) => Ok(*v as f64),
            BigNum::Float128(f) => {
                let f64_val: f64 = (*f).into();
                Ok(f64_val)
            }
            BigNum::BigInt(bi) => {
                use num_traits::ToPrimitive;
                bi.to_f64().ok_or_else(||
                    GraphoidError::runtime("BigInt too large to convert to f64 for comparison".to_string())
                )
            }
        }
    }

    // Comparison helpers
    pub(crate) fn eval_less(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => Ok(Value::boolean(l < r)),
            (ValueKind::String(l), ValueKind::String(r)) => Ok(Value::boolean(l < r)),

            // BigNumber < BigNumber - compare by value
            (ValueKind::BigNumber(l), ValueKind::BigNumber(r)) => {
                // Convert both to f64 for comparison (may lose precision for very large numbers)
                let lv = self.bignum_to_f64(l)?;
                let rv = self.bignum_to_f64(r)?;
                Ok(Value::boolean(lv < rv))
            }

            // Phase 1B: Mixed num/bignum comparison - compare by value, not type
            (ValueKind::Number(n), ValueKind::BigNumber(bn)) => {
                let bn_f64 = self.bignum_to_f64(bn)?;
                Ok(Value::boolean(*n < bn_f64))
            }
            (ValueKind::BigNumber(bn), ValueKind::Number(n)) => {
                let bn_f64 = self.bignum_to_f64(bn)?;
                Ok(Value::boolean(bn_f64 < *n))
            }

            (_l, _r) => Err(GraphoidError::type_error(
                "number, string, or bignum",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    pub(crate) fn eval_less_equal(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => Ok(Value::boolean(l <= r)),
            (ValueKind::String(l), ValueKind::String(r)) => Ok(Value::boolean(l <= r)),

            // BigNumber <= BigNumber - compare by value
            (ValueKind::BigNumber(l), ValueKind::BigNumber(r)) => {
                let lv = self.bignum_to_f64(l)?;
                let rv = self.bignum_to_f64(r)?;
                Ok(Value::boolean(lv <= rv))
            }

            // Phase 1B: Mixed num/bignum comparison
            (ValueKind::Number(n), ValueKind::BigNumber(bn)) => {
                let bn_f64 = self.bignum_to_f64(bn)?;
                Ok(Value::boolean(*n <= bn_f64))
            }
            (ValueKind::BigNumber(bn), ValueKind::Number(n)) => {
                let bn_f64 = self.bignum_to_f64(bn)?;
                Ok(Value::boolean(bn_f64 <= *n))
            }

            (_l, _r) => Err(GraphoidError::type_error(
                "number, string, or bignum",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    pub(crate) fn eval_greater(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => Ok(Value::boolean(l > r)),
            (ValueKind::String(l), ValueKind::String(r)) => Ok(Value::boolean(l > r)),

            // BigNumber > BigNumber - compare by value
            (ValueKind::BigNumber(l), ValueKind::BigNumber(r)) => {
                let lv = self.bignum_to_f64(l)?;
                let rv = self.bignum_to_f64(r)?;
                Ok(Value::boolean(lv > rv))
            }

            // Phase 1B: Mixed num/bignum comparison
            (ValueKind::Number(n), ValueKind::BigNumber(bn)) => {
                let bn_f64 = self.bignum_to_f64(bn)?;
                Ok(Value::boolean(*n > bn_f64))
            }
            (ValueKind::BigNumber(bn), ValueKind::Number(n)) => {
                let bn_f64 = self.bignum_to_f64(bn)?;
                Ok(Value::boolean(bn_f64 > *n))
            }

            (_l, _r) => Err(GraphoidError::type_error(
                "number, string, or bignum",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    pub(crate) fn eval_greater_equal(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => Ok(Value::boolean(l >= r)),
            (ValueKind::String(l), ValueKind::String(r)) => Ok(Value::boolean(l >= r)),

            // BigNumber >= BigNumber - compare by value
            (ValueKind::BigNumber(l), ValueKind::BigNumber(r)) => {
                let lv = self.bignum_to_f64(l)?;
                let rv = self.bignum_to_f64(r)?;
                Ok(Value::boolean(lv >= rv))
            }

            // Phase 1B: Mixed num/bignum comparison
            (ValueKind::Number(n), ValueKind::BigNumber(bn)) => {
                let bn_f64 = self.bignum_to_f64(bn)?;
                Ok(Value::boolean(*n >= bn_f64))
            }
            (ValueKind::BigNumber(bn), ValueKind::Number(n)) => {
                let bn_f64 = self.bignum_to_f64(bn)?;
                Ok(Value::boolean(bn_f64 >= *n))
            }

            (_l, _r) => Err(GraphoidError::type_error(
                "number, string, or bignum",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

}

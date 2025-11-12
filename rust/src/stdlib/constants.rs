//! Constants Module - Mathematical and Physical Constants
//!
//! Provides access to commonly used mathematical and physical constants
//! with full precision.

use super::{NativeFunction, NativeModule};
use crate::values::Value;
use std::collections::HashMap;

/// Constants module providing mathematical and physical constants
pub struct ConstantsModule;

impl NativeModule for ConstantsModule {
    fn name(&self) -> &str {
        "constants"
    }

    fn alias(&self) -> Option<&str> {
        Some("const")
    }

    fn constants(&self) -> HashMap<String, Value> {
        let mut constants = HashMap::new();

        // Mathematical constants
        constants.insert("pi".to_string(), Value::number(std::f64::consts::PI));
        constants.insert("e".to_string(), Value::number(std::f64::consts::E));
        constants.insert("tau".to_string(), Value::number(std::f64::consts::TAU));
        constants.insert("phi".to_string(), Value::number(1.618033988749895)); // Golden ratio
        constants.insert("sqrt2".to_string(), Value::number(std::f64::consts::SQRT_2));
        constants.insert("sqrt3".to_string(), Value::number(3_f64.sqrt()));

        // Angle conversion constants
        constants.insert("deg_to_rad".to_string(), Value::number(std::f64::consts::PI / 180.0));
        constants.insert("rad_to_deg".to_string(), Value::number(180.0 / std::f64::consts::PI));

        // Physical constants
        constants.insert("c".to_string(), Value::number(299792458.0)); // Speed of light (m/s)
        constants.insert("G".to_string(), Value::number(6.67430e-11)); // Gravitational constant
        constants.insert("h".to_string(), Value::number(6.62607015e-34)); // Planck constant

        // Additional mathematical constants
        constants.insert("ln2".to_string(), Value::number(std::f64::consts::LN_2));
        constants.insert("ln10".to_string(), Value::number(std::f64::consts::LN_10));
        constants.insert("log2e".to_string(), Value::number(std::f64::consts::LOG2_E));
        constants.insert("log10e".to_string(), Value::number(std::f64::consts::LOG10_E));

        constants
    }

    fn functions(&self) -> HashMap<String, NativeFunction> {
        // Constants module has no functions, only constants
        HashMap::new()
    }
}

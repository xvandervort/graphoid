// Configuration system for Graphoid runtime
//
// Provides scoped configuration contexts that can be pushed/popped
// to control runtime behavior like error handling, type coercion, etc.

use std::collections::HashMap;
use crate::error::{GraphoidError, Result};
use crate::values::Value;

/// Configuration settings for the Graphoid runtime
#[derive(Debug, Clone)]
pub struct Config {
    // Error handling
    pub error_mode: ErrorMode,
    pub bounds_checking: BoundsCheckingMode,
    pub type_coercion: TypeCoercionMode,
    pub none_handling: NoneHandlingMode,

    // Numeric precision
    pub decimal_places: Option<usize>,  // None = no rounding

    // Type system
    pub strict_types: bool,

    // Graph validation
    pub edge_validation: bool,
    pub strict_edge_rules: bool,

    // None conversions
    pub none_conversions: bool,

    // Skip none values in operations
    pub skip_none: bool,
}

/// Error handling mode
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorMode {
    Strict,   // Errors stop execution unless caught
    Lenient,  // Use safe defaults (none, skip, etc.)
    Collect,  // Collect errors, continue execution
}

/// Bounds checking mode for array/list access
#[derive(Debug, Clone, PartialEq)]
pub enum BoundsCheckingMode {
    Strict,   // Out of bounds access raises error
    Lenient,  // Out of bounds returns none
}

/// Type coercion mode
#[derive(Debug, Clone, PartialEq)]
pub enum TypeCoercionMode {
    Strict,   // Type mismatches raise errors
    Lenient,  // Attempt conversions, return none on failure
}

/// None/null handling mode
#[derive(Debug, Clone, PartialEq)]
pub enum NoneHandlingMode {
    Propagate,  // none values pass through operations
    Skip,       // Skip none values in operations
    Error,      // Treat none as an error
}

impl Default for Config {
    fn default() -> Self {
        Config {
            error_mode: ErrorMode::Strict,
            bounds_checking: BoundsCheckingMode::Strict,
            type_coercion: TypeCoercionMode::Strict,
            none_handling: NoneHandlingMode::Propagate,
            decimal_places: None,
            strict_types: true,
            edge_validation: true,
            strict_edge_rules: true,
            none_conversions: true,
            skip_none: false,
        }
    }
}

/// Stack of configuration contexts
/// Allows pushing/popping configs for scoped behavior
pub struct ConfigStack {
    stack: Vec<Config>,
}

impl ConfigStack {
    /// Create a new config stack with default configuration
    pub fn new() -> Self {
        ConfigStack {
            stack: vec![Config::default()],
        }
    }

    /// Get the current (top) configuration
    pub fn current(&self) -> &Config {
        self.stack.last().unwrap()
    }

    /// Get mutable reference to current configuration
    pub fn current_mut(&mut self) -> &mut Config {
        self.stack.last_mut().unwrap()
    }

    /// Push a new configuration onto the stack
    pub fn push(&mut self, config: Config) {
        self.stack.push(config);
    }

    /// Pop the top configuration off the stack
    /// Returns None if trying to pop the base config (stack size 1)
    pub fn pop(&mut self) -> Option<Config> {
        if self.stack.len() > 1 {
            self.stack.pop()
        } else {
            None  // Never pop the base config
        }
    }

    /// Push a new config with specified changes
    /// Clones current config and applies changes
    pub fn push_with_changes(&mut self, changes: HashMap<String, Value>) -> Result<()> {
        let mut new_config = self.current().clone();

        // Apply changes to new_config
        for (key, value) in changes {
            match key.as_str() {
                "error_mode" => {
                    new_config.error_mode = parse_error_mode(&value)?;
                }
                "bounds_checking" => {
                    new_config.bounds_checking = parse_bounds_checking_mode(&value)?;
                }
                "type_coercion" => {
                    new_config.type_coercion = parse_type_coercion_mode(&value)?;
                }
                "none_handling" => {
                    new_config.none_handling = parse_none_handling_mode(&value)?;
                }
                "decimal_places" => {
                    let num = value.to_number().ok_or_else(|| GraphoidError::ConfigError {
                        message: format!("decimal_places must be a number, got {}", value.type_name()),
                    })?;
                    new_config.decimal_places = Some(num as usize);
                }
                "skip_none" => {
                    new_config.skip_none = value.is_truthy();
                }
                "strict_types" => {
                    new_config.strict_types = value.is_truthy();
                }
                "edge_validation" => {
                    new_config.edge_validation = value.is_truthy();
                }
                "strict_edge_rules" => {
                    new_config.strict_edge_rules = value.is_truthy();
                }
                "none_conversions" => {
                    new_config.none_conversions = value.is_truthy();
                }
                _ => {
                    return Err(GraphoidError::ConfigError {
                        message: format!("Unknown configuration key: {}", key),
                    });
                }
            }
        }

        self.push(new_config);
        Ok(())
    }

    /// Get the stack depth
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
}

impl Default for ConfigStack {
    fn default() -> Self {
        Self::new()
    }
}

// Helper functions to parse mode values from Value types

fn parse_error_mode(value: &Value) -> Result<ErrorMode> {
    match value {
        Value::Symbol(s) => match s.as_str() {
            "strict" => Ok(ErrorMode::Strict),
            "lenient" => Ok(ErrorMode::Lenient),
            "collect" => Ok(ErrorMode::Collect),
            _ => Err(GraphoidError::ConfigError {
                message: format!("Invalid error_mode: :{}, expected :strict, :lenient, or :collect", s),
            }),
        },
        _ => Err(GraphoidError::ConfigError {
            message: format!("error_mode must be a symbol, got {}", value.type_name()),
        }),
    }
}

fn parse_bounds_checking_mode(value: &Value) -> Result<BoundsCheckingMode> {
    match value {
        Value::Symbol(s) => match s.as_str() {
            "strict" => Ok(BoundsCheckingMode::Strict),
            "lenient" => Ok(BoundsCheckingMode::Lenient),
            _ => Err(GraphoidError::ConfigError {
                message: format!("Invalid bounds_checking: :{}, expected :strict or :lenient", s),
            }),
        },
        _ => Err(GraphoidError::ConfigError {
            message: format!("bounds_checking must be a symbol, got {}", value.type_name()),
        }),
    }
}

fn parse_type_coercion_mode(value: &Value) -> Result<TypeCoercionMode> {
    match value {
        Value::Symbol(s) => match s.as_str() {
            "strict" => Ok(TypeCoercionMode::Strict),
            "lenient" => Ok(TypeCoercionMode::Lenient),
            _ => Err(GraphoidError::ConfigError {
                message: format!("Invalid type_coercion: :{}, expected :strict or :lenient", s),
            }),
        },
        _ => Err(GraphoidError::ConfigError {
            message: format!("type_coercion must be a symbol, got {}", value.type_name()),
        }),
    }
}

fn parse_none_handling_mode(value: &Value) -> Result<NoneHandlingMode> {
    match value {
        Value::Symbol(s) => match s.as_str() {
            "propagate" => Ok(NoneHandlingMode::Propagate),
            "skip" => Ok(NoneHandlingMode::Skip),
            "error" => Ok(NoneHandlingMode::Error),
            _ => Err(GraphoidError::ConfigError {
                message: format!("Invalid none_handling: :{}, expected :propagate, :skip, or :error", s),
            }),
        },
        _ => Err(GraphoidError::ConfigError {
            message: format!("none_handling must be a symbol, got {}", value.type_name()),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.error_mode, ErrorMode::Strict);
        assert_eq!(config.bounds_checking, BoundsCheckingMode::Strict);
        assert_eq!(config.type_coercion, TypeCoercionMode::Strict);
        assert_eq!(config.none_handling, NoneHandlingMode::Propagate);
        assert_eq!(config.decimal_places, None);
        assert_eq!(config.strict_types, true);
        assert_eq!(config.edge_validation, true);
        assert_eq!(config.strict_edge_rules, true);
        assert_eq!(config.none_conversions, true);
        assert_eq!(config.skip_none, false);
    }

    #[test]
    fn test_config_stack_new() {
        let stack = ConfigStack::new();
        assert_eq!(stack.depth(), 1);
        assert_eq!(stack.current().error_mode, ErrorMode::Strict);
    }

    #[test]
    fn test_config_stack_push_pop() {
        let mut stack = ConfigStack::new();

        let mut new_config = Config::default();
        new_config.skip_none = true;

        stack.push(new_config);
        assert_eq!(stack.depth(), 2);
        assert_eq!(stack.current().skip_none, true);

        stack.pop();
        assert_eq!(stack.depth(), 1);
        assert_eq!(stack.current().skip_none, false);
    }

    #[test]
    fn test_config_stack_cannot_pop_base() {
        let mut stack = ConfigStack::new();
        assert_eq!(stack.depth(), 1);

        let result = stack.pop();
        assert!(result.is_none());
        assert_eq!(stack.depth(), 1);
    }

    #[test]
    fn test_push_with_changes_skip_none() {
        let mut stack = ConfigStack::new();

        let mut changes = HashMap::new();
        changes.insert("skip_none".to_string(), Value::Boolean(true));

        stack.push_with_changes(changes).unwrap();
        assert_eq!(stack.current().skip_none, true);
        assert_eq!(stack.depth(), 2);
    }

    #[test]
    fn test_push_with_changes_error_mode() {
        let mut stack = ConfigStack::new();

        let mut changes = HashMap::new();
        changes.insert("error_mode".to_string(), Value::Symbol("lenient".to_string()));

        stack.push_with_changes(changes).unwrap();
        assert_eq!(stack.current().error_mode, ErrorMode::Lenient);
    }

    #[test]
    fn test_push_with_changes_invalid_key() {
        let mut stack = ConfigStack::new();

        let mut changes = HashMap::new();
        changes.insert("invalid_key".to_string(), Value::Boolean(true));

        let result = stack.push_with_changes(changes);
        assert!(result.is_err());
        assert_eq!(stack.depth(), 1); // Stack should remain unchanged
    }

    #[test]
    fn test_parse_error_mode_valid() {
        let strict = Value::Symbol("strict".to_string());
        let lenient = Value::Symbol("lenient".to_string());
        let collect = Value::Symbol("collect".to_string());

        assert_eq!(parse_error_mode(&strict).unwrap(), ErrorMode::Strict);
        assert_eq!(parse_error_mode(&lenient).unwrap(), ErrorMode::Lenient);
        assert_eq!(parse_error_mode(&collect).unwrap(), ErrorMode::Collect);
    }

    #[test]
    fn test_parse_error_mode_invalid() {
        let invalid = Value::Symbol("invalid".to_string());
        assert!(parse_error_mode(&invalid).is_err());

        let not_symbol = Value::Number(123.0);
        assert!(parse_error_mode(&not_symbol).is_err());
    }

    #[test]
    fn test_parse_bounds_checking_mode() {
        let strict = Value::Symbol("strict".to_string());
        let lenient = Value::Symbol("lenient".to_string());

        assert_eq!(parse_bounds_checking_mode(&strict).unwrap(), BoundsCheckingMode::Strict);
        assert_eq!(parse_bounds_checking_mode(&lenient).unwrap(), BoundsCheckingMode::Lenient);
    }

    #[test]
    fn test_parse_type_coercion_mode() {
        let strict = Value::Symbol("strict".to_string());
        let lenient = Value::Symbol("lenient".to_string());

        assert_eq!(parse_type_coercion_mode(&strict).unwrap(), TypeCoercionMode::Strict);
        assert_eq!(parse_type_coercion_mode(&lenient).unwrap(), TypeCoercionMode::Lenient);
    }

    #[test]
    fn test_parse_none_handling_mode() {
        let propagate = Value::Symbol("propagate".to_string());
        let skip = Value::Symbol("skip".to_string());
        let error = Value::Symbol("error".to_string());

        assert_eq!(parse_none_handling_mode(&propagate).unwrap(), NoneHandlingMode::Propagate);
        assert_eq!(parse_none_handling_mode(&skip).unwrap(), NoneHandlingMode::Skip);
        assert_eq!(parse_none_handling_mode(&error).unwrap(), NoneHandlingMode::Error);
    }

    #[test]
    fn test_nested_config_changes() {
        let mut stack = ConfigStack::new();

        // Push first level
        let mut changes1 = HashMap::new();
        changes1.insert("skip_none".to_string(), Value::Boolean(true));
        stack.push_with_changes(changes1).unwrap();
        assert_eq!(stack.depth(), 2);
        assert_eq!(stack.current().skip_none, true);

        // Push second level
        let mut changes2 = HashMap::new();
        changes2.insert("error_mode".to_string(), Value::Symbol("lenient".to_string()));
        stack.push_with_changes(changes2).unwrap();
        assert_eq!(stack.depth(), 3);
        assert_eq!(stack.current().skip_none, true);  // Inherited from previous level
        assert_eq!(stack.current().error_mode, ErrorMode::Lenient);

        // Pop back to second level
        stack.pop();
        assert_eq!(stack.depth(), 2);
        assert_eq!(stack.current().skip_none, true);
        assert_eq!(stack.current().error_mode, ErrorMode::Strict);  // Back to default

        // Pop back to base
        stack.pop();
        assert_eq!(stack.depth(), 1);
        assert_eq!(stack.current().skip_none, false);
    }

    #[test]
    fn test_config_cloning() {
        let config1 = Config::default();
        let mut config2 = config1.clone();

        config2.skip_none = true;

        assert_eq!(config1.skip_none, false);
        assert_eq!(config2.skip_none, true);
    }

    #[test]
    fn test_multiple_config_keys() {
        let mut stack = ConfigStack::new();

        let mut changes = HashMap::new();
        changes.insert("skip_none".to_string(), Value::Boolean(true));
        changes.insert("error_mode".to_string(), Value::Symbol("lenient".to_string()));
        changes.insert("strict_types".to_string(), Value::Boolean(false));

        stack.push_with_changes(changes).unwrap();
        assert_eq!(stack.current().skip_none, true);
        assert_eq!(stack.current().error_mode, ErrorMode::Lenient);
        assert_eq!(stack.current().strict_types, false);
    }
}

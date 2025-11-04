// Configuration system for Graphoid runtime
//
// Provides scoped configuration contexts that can be pushed/popped
// to control runtime behavior like error handling, type coercion, etc.

use std::collections::HashMap;
use crate::error::{GraphoidError, Result};
use crate::values::{Value, ValueKind};
use crate::values::graph::{OrphanPolicy, ReconnectStrategy};

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

    // Graph orphan management
    pub orphan_policy: Option<OrphanPolicy>,
    pub reconnect_strategy: Option<ReconnectStrategy>,
    pub allow_overrides: Option<bool>,
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
            orphan_policy: None,
            reconnect_strategy: None,
            allow_overrides: None,
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
                "orphan_policy" => {
                    new_config.orphan_policy = Some(parse_orphan_policy(&value)?);
                }
                "reconnect_strategy" => {
                    new_config.reconnect_strategy = Some(parse_reconnect_strategy(&value)?);
                }
                "allow_overrides" => {
                    new_config.allow_overrides = Some(value.is_truthy());
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
    match &value.kind {
        ValueKind::Symbol(s) => match s.as_str() {
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
    match &value.kind {
        ValueKind::Symbol(s) => match s.as_str() {
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
    match &value.kind {
        ValueKind::Symbol(s) => match s.as_str() {
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
    match &value.kind {
        ValueKind::Symbol(s) => match s.as_str() {
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

fn parse_orphan_policy(value: &Value) -> Result<OrphanPolicy> {
    match &value.kind {
        ValueKind::Symbol(s) => match s.as_str() {
            "allow" => Ok(OrphanPolicy::Allow),
            "reject" => Ok(OrphanPolicy::Reject),
            "delete" => Ok(OrphanPolicy::Delete),
            "reconnect" => Ok(OrphanPolicy::Reconnect),
            _ => Err(GraphoidError::ConfigError {
                message: format!("Invalid orphan_policy: :{}, expected :allow, :reject, :delete, or :reconnect", s),
            }),
        },
        _ => Err(GraphoidError::ConfigError {
            message: format!("orphan_policy must be a symbol, got {}", value.type_name()),
        }),
    }
}

fn parse_reconnect_strategy(value: &Value) -> Result<ReconnectStrategy> {
    match &value.kind {
        ValueKind::Symbol(s) => match s.as_str() {
            "to_root" => Ok(ReconnectStrategy::ToRoot),
            "to_parent_siblings" => Ok(ReconnectStrategy::ToParentSiblings),
            _ => Err(GraphoidError::ConfigError {
                message: format!("Invalid reconnect_strategy: :{}, expected :to_root or :to_parent_siblings", s),
            }),
        },
        _ => Err(GraphoidError::ConfigError {
            message: format!("reconnect_strategy must be a symbol, got {}", value.type_name()),
        }),
    }
}

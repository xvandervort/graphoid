//! Map/Hash method implementations for the Graphoid executor.
//!
//! This module contains map-specific method handling,
//! extracted from the main executor for better code organization.

use crate::error::{GraphoidError, Result};
use crate::execution::Executor;
use crate::graph::{RuleSpec, RuleInstance};
use crate::values::{Value, ValueKind, List, Hash};

impl Executor {
    // =========================================================================
    // Map Instance Methods
    // =========================================================================

    pub(crate) fn eval_map_method(&mut self, hash: &Hash, method: &str, args: &[Value]) -> Result<Value> {
        match method {
            "keys" => {
                // Return list of all keys
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(
                        "keys() takes no arguments".to_string()
                    ));
                }
                let keys: Vec<Value> = hash.keys()
                    .iter()
                    .map(|k| Value::string(k.clone()))
                    .collect();
                Ok(Value::list(List::from_vec(keys)))
            }
            "values" => {
                // Return list of all values
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(
                        "values() takes no arguments".to_string()
                    ));
                }
                let values: Vec<Value> = hash.values();
                Ok(Value::list(List::from_vec(values)))
            }
            "has_key" => {
                // Check if key exists
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(
                        "has_key() requires exactly one argument".to_string()
                    ));
                }
                let key = match &args[0].kind {
                    ValueKind::String(s) => s,
                    _ => return Err(GraphoidError::runtime(
                        "has_key() requires a string argument".to_string()
                    )),
                };
                Ok(Value::boolean(hash.contains_key(key)))
            }
            "size" | "len" | "length" => {
                // Return number of entries
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(
                        "size()/len()/length() takes no arguments".to_string()
                    ));
                }
                Ok(Value::number(hash.len() as f64))
            }
            "is_empty" => {
                // Check if map is empty
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(
                        "is_empty() takes no arguments".to_string()
                    ));
                }
                Ok(Value::boolean(hash.is_empty()))
            }
            "add_rule" => {
                // add_rule(rule_symbol) or add_rule(rule_symbol, param) or add_rule(rule_symbol, param1, param2)
                // Handles BOTH validation rules AND transformation rules (behaviors)
                if args.is_empty() || args.len() > 3 {
                    return Err(GraphoidError::runtime(format!(
                        "add_rule() expects 1-3 arguments, but got {}",
                        args.len()
                    )));
                }

                // Get rule symbol
                let rule_symbol = match &args[0].kind {
                    ValueKind::Symbol(name) => name.as_str(),
                    _other => {
                        return Err(GraphoidError::runtime(format!(
                            "add_rule() expects a symbol, got {}",
                            args[0].type_name()
                        )));
                    }
                };

                // Clone hash
                let mut new_hash = hash.clone();

                // Handle validate_range specially (needs 2 params: min, max)
                let rule_spec = if rule_symbol == "validate_range" {
                    if args.len() != 3 {
                        return Err(GraphoidError::runtime(format!(
                            "validate_range requires 2 arguments (min, max), but got {}",
                            args.len() - 1
                        )));
                    }
                    let min = match &args[1].kind {
                        ValueKind::Number(n) => *n,
                        _other => {
                            return Err(GraphoidError::runtime(format!(
                                "validate_range min must be a number, got {}",
                                args[1].type_name()
                            )));
                        }
                    };
                    let max = match &args[2].kind {
                        ValueKind::Number(n) => *n,
                        _other => {
                            return Err(GraphoidError::runtime(format!(
                                "validate_range max must be a number, got {}",
                                args[2].type_name()
                            )));
                        }
                    };
                    RuleSpec::ValidateRange { min, max }
                } else {
                    // For all other rules, get optional single parameter
                    let param = if args.len() >= 2 {
                        match &args[1].kind {
                            ValueKind::Number(n) => Some(*n),
                            _other => {
                                return Err(GraphoidError::runtime(format!(
                                    "add_rule() parameter must be a number, got {}",
                                    args[1].type_name()
                                )));
                            }
                        }
                    } else {
                        None
                    };

                    // Convert symbol to RuleSpec
                    Self::symbol_to_rule_spec(rule_symbol, param)?
                };

                // Add rule and return new hash
                new_hash.add_rule(RuleInstance::new(rule_spec))?;
                Ok(Value::map(new_hash))
            }
            "remove_rule" => {
                // remove_rule(rule_symbol) or remove_rule(rule_symbol, param)
                if args.is_empty() || args.len() > 2 {
                    return Err(GraphoidError::runtime(format!(
                        "remove_rule() expects 1 or 2 arguments, but got {}",
                        args.len()
                    )));
                }

                // Get rule symbol
                let rule_symbol = match &args[0].kind {
                    ValueKind::Symbol(name) => name.as_str(),
                    _other => {
                        return Err(GraphoidError::runtime(format!(
                            "remove_rule() expects a symbol, got {}",
                            args[0].type_name()
                        )));
                    }
                };

                // Get optional parameter
                let param = if args.len() == 2 {
                    match &args[1].kind {
                        ValueKind::Number(n) => Some(*n),
                        _other => {
                            return Err(GraphoidError::runtime(format!(
                                "remove_rule() parameter must be a number, got {}",
                                args[1].type_name()
                            )));
                        }
                    }
                } else {
                    None
                };

                // Convert to RuleSpec
                let rule_spec = Self::symbol_to_rule_spec(rule_symbol, param)?;

                // Clone hash, remove rule, return
                let mut new_hash = hash.clone();
                new_hash.remove_rule(&rule_spec);
                Ok(Value::map(new_hash))
            }
            "remove" => {
                // remove(key) - returns new hash without the specified key
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "remove() expects 1 argument (key), but got {}",
                        args.len()
                    )));
                }
                let key = match &args[0].kind {
                    ValueKind::String(s) => s.clone(),
                    _other => {
                        return Err(GraphoidError::runtime(format!(
                            "remove() requires a string key, got {}",
                            args[0].type_name()
                        )));
                    }
                };

                // Clone hash and remove key
                let mut new_hash = hash.clone();
                let _ = new_hash.remove(&key);  // Ignore result - Ok if key doesn't exist
                Ok(Value::map(new_hash))
            }
            _ => {
                // Check if this is property-style access (no arguments, method name matches a key)
                if args.is_empty() {
                    if let Some(value) = hash.get(method) {
                        return Ok(value.clone());
                    }
                }

                Err(GraphoidError::runtime(format!(
                    "Map does not have method '{}'",
                    method
                )))
            }
        }
    }
}

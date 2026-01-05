//! List method implementations for the Graphoid executor.
//!
//! This module contains list-specific method handling,
//! extracted from the main executor for better code organization.


use crate::graph::{RuleSpec, RuleInstance};
use crate::error::{GraphoidError, Result};
use crate::execution::executor::Executor;
use crate::values::{Value, ValueKind, List};

impl Executor {
    // =========================================================================
    // List Static Methods
    // =========================================================================

    pub(crate) fn eval_list_static_method(&mut self, method: &str, args: &[Value]) -> Result<Value> {
        match method {
            "generate" => {
                if args.len() != 3 {
                    return Err(GraphoidError::runtime(format!(
                        "list.generate() expects 3 arguments, but got {}",
                        args.len()
                    )));
                }

                let start = match &args[0].kind {
                    ValueKind::Number(n) => *n,
                    _other => {
                        return Err(GraphoidError::type_error("number", args[0].type_name()));
                    }
                };

                let end = match &args[1].kind {
                    ValueKind::Number(n) => *n,
                    _other => {
                        return Err(GraphoidError::type_error("number", args[1].type_name()));
                    }
                };

                // Check if third argument is a function or a number (step)
                match &args[2].kind {
                    ValueKind::Number(step) => {
                        // Range mode with step
                        let mut result = Vec::new();
                        if *step > 0.0 {
                            let mut current = start;
                            while current <= end {
                                result.push(Value::number(current));
                                current += step;
                            }
                        } else if *step < 0.0 {
                            let mut current = start;
                            while current >= end {
                                result.push(Value::number(current));
                                current += step;
                            }
                        } else {
                            return Err(GraphoidError::runtime("generate step cannot be zero".to_string()));
                        }
                        Ok(Value::list(List::from_vec(result)))
                    }
                    ValueKind::Function(func) => {
                        // Function mode
                        let mut result = Vec::new();
                        let start_i = start as i64;
                        let end_i = end as i64;
                        for i in start_i..=end_i {
                            let arg = Value::number(i as f64);
                            let value = self.call_function(func, &[arg])?;
                            result.push(value);
                        }
                        Ok(Value::list(List::from_vec(result)))
                    }
                    _other => {
                        return Err(GraphoidError::runtime(format!(
                            "list.generate() expects third argument to be number or function, got {}",
                            args[2].type_name()
                        )));
                    }
                }
            }
            "upto" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "list.upto() expects 1 argument, but got {}",
                        args.len()
                    )));
                }

                let n = match &args[0].kind {
                    ValueKind::Number(num) => *num as i64,
                    _other => {
                        return Err(GraphoidError::type_error("number", args[0].type_name()));
                    }
                };

                let mut result = Vec::new();
                for i in 0..=n {
                    result.push(Value::number(i as f64));
                }
                Ok(Value::list(List::from_vec(result)))
            }
            _ => Err(GraphoidError::runtime(format!(
                "list does not have static method '{}'",
                method
            ))),
        }
    }

    /// Evaluates static methods on the string type (e.g., string.generate).

    // =========================================================================
    // List Instance Methods
    // =========================================================================

    pub(crate) fn eval_list_method(&mut self, list: &List, method: &str, args: &[Value]) -> Result<Value> {
        let elements = list.to_vec();
        match method {
            "size" | "length" | "len" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'size'/'length'/'len' expects 0 arguments, but got {}",
                        args.len()
                    )));
                }
                Ok(Value::number(list.len() as f64))
            }
            "first" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'first' expects 0 arguments, but got {}",
                        args.len()
                    )));
                }
                elements.first()
                    .cloned()
                    .ok_or_else(|| GraphoidError::runtime("Cannot get first element of empty list".to_string()))
            }
            "last" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'last' expects 0 arguments, but got {}",
                        args.len()
                    )));
                }
                elements.last()
                    .cloned()
                    .ok_or_else(|| GraphoidError::runtime("Cannot get last element of empty list".to_string()))
            }
            "contains" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'contains' expects 1 argument, but got {}",
                        args.len()
                    )));
                }
                let search_value = &args[0];
                for element in &elements {
                    if element == search_value {
                        return Ok(Value::boolean(true));
                    }
                }
                Ok(Value::boolean(false))
            }
            "is_empty" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'is_empty' expects 0 arguments, but got {}",
                        args.len()
                    )));
                }
                Ok(Value::boolean(list.is_empty()))
            }
            "map" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'map' expects 1 argument, but got {}",
                        args.len()
                    )));
                }

                // Check if argument is a symbol (named transformation) or function
                match &args[0].kind {
                    ValueKind::Symbol(transform_name) => {
                        // Apply named transformation
                        let mut results = Vec::new();
                        for element in &elements {
                            let result = self.apply_named_transformation(element, transform_name)?;
                            results.push(result);
                        }
                        Ok(Value::list(List::from_vec(results)))
                    }
                    ValueKind::Function(func) => {
                        // Apply the function to each element
                        let mut results = Vec::new();
                        for element in &elements {
                            // Call the function with this element
                            let result = self.call_function(func, &[element.clone()])?;
                            results.push(result);
                        }
                        Ok(Value::list(List::from_vec(results)))
                    }
                    _other => {
                        return Err(GraphoidError::runtime(format!(
                            "Method 'map' expects function or symbol, got {}",
                            args[0].type_name()
                        )));
                    }
                }
            }
            "filter" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'filter' expects 1 argument, but got {}",
                        args.len()
                    )));
                }

                // Check if argument is a symbol (named predicate) or function
                match &args[0].kind {
                    ValueKind::Symbol(predicate_name) => {
                        // Apply named predicate
                        let mut results = Vec::new();
                        for element in &elements {
                            if self.apply_named_predicate(element, predicate_name)? {
                                results.push(element.clone());
                            }
                        }
                        Ok(Value::list(List::from_vec(results)))
                    }
                    ValueKind::Function(func) => {
                        // Filter elements based on predicate function
                        let mut results = Vec::new();
                        for element in &elements {
                            // Call the function with this element
                            let result = self.call_function(func, &[element.clone()])?;

                            // Check if result is truthy
                            if result.is_truthy() {
                                results.push(element.clone());
                            }
                        }
                        Ok(Value::list(List::from_vec(results)))
                    }
                    _other => {
                        return Err(GraphoidError::runtime(format!(
                            "Method 'filter' expects function or symbol, got {}",
                            args[0].type_name()
                        )));
                    }
                }
            }
            "each" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'each' expects 1 argument, but got {}",
                        args.len()
                    )));
                }

                // Get the function argument
                let func = match &args[0].kind {
                    ValueKind::Function(f) => f,
                    _other => {
                        return Err(GraphoidError::type_error(
                            "function",
                            args[0].type_name(),
                        ));
                    }
                };

                // Execute the function for each element (for side effects)
                for element in &elements {
                    // Call the function with this element, ignore result
                    let _ = self.call_function(func, &[element.clone()])?;
                }

                // Return the original list
                Ok(Value::list(list.clone()))
            }
            "slice" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'slice' expects 2 or 3 arguments, but got {}",
                        args.len()
                    )));
                }

                // Get start and end indices
                let start_idx = match &args[0].kind {
                    ValueKind::Number(n) => *n as i64,
                    _other => {
                        return Err(GraphoidError::type_error(
                            "number",
                            args[0].type_name(),
                        ));
                    }
                };

                let end_idx = match &args[1].kind {
                    ValueKind::Number(n) => *n as i64,
                    _other => {
                        return Err(GraphoidError::type_error(
                            "number",
                            args[1].type_name(),
                        ));
                    }
                };

                // Get optional step parameter (default 1)
                let step = if args.len() == 3 {
                    match &args[2].kind {
                        ValueKind::Number(n) => *n as i64,
                        _other => {
                            return Err(GraphoidError::type_error(
                                "number",
                                args[2].type_name(),
                            ));
                        }
                    }
                } else {
                    1
                };

                if step == 0 {
                    return Err(GraphoidError::runtime("slice step cannot be zero".to_string()));
                }

                let len = elements.len() as i64;

                // Normalize negative indices
                let actual_start = if start_idx < 0 {
                    (len + start_idx).max(0)
                } else {
                    start_idx.min(len)
                };

                let actual_end = if end_idx < 0 {
                    (len + end_idx).max(0)
                } else {
                    end_idx.min(len)
                };

                // Ensure start <= end
                if actual_start > actual_end {
                    return Ok(Value::list(List::new()));
                }

                // Extract slice with step
                let mut slice = Vec::new();
                let mut i = actual_start;
                while i < actual_end {
                    slice.push(elements[i as usize].clone());
                    i += step;
                }
                Ok(Value::list(List::from_vec(slice)))
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

                // Clone list
                let mut new_list = list.clone();

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

                // Add rule and return new list
                new_list.add_rule(RuleInstance::new(rule_spec))?;
                Ok(Value::list(new_list))
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

                // Clone list, remove rule, return
                let mut new_list = list.clone();
                new_list.remove_rule(&rule_spec);
                Ok(Value::list(new_list))
            }
            "sort" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'sort' expects 0 arguments, but got {}",
                        args.len()
                    )));
                }

                // Sort numeric lists
                let mut sorted = elements.clone();
                sorted.sort_by(|a, b| {
                    match (&a.kind, &b.kind) {
                        (ValueKind::Number(n1), ValueKind::Number(n2)) => {
                            n1.partial_cmp(n2).unwrap_or(std::cmp::Ordering::Equal)
                        }
                        _ => std::cmp::Ordering::Equal,
                    }
                });
                Ok(Value::list(List::from_vec(sorted)))
            }
            "reverse" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'reverse' expects 0 arguments, but got {}",
                        args.len()
                    )));
                }

                let mut reversed = elements.clone();
                reversed.reverse();
                Ok(Value::list(List::from_vec(reversed)))
            }
            "join" => {
                // join(separator) - join list elements into a string
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'join' expects 1 argument (separator), but got {}",
                        args.len()
                    )));
                }

                let separator = match &args[0].kind {
                    ValueKind::String(s) => s.clone(),
                    _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
                };

                let string_elements: Vec<String> = elements
                    .iter()
                    .map(|e| e.to_string_value())
                    .collect();

                Ok(Value::string(string_elements.join(&separator)))
            }
            "uniq" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'uniq' expects 0 arguments, but got {}",
                        args.len()
                    )));
                }

                // Remove duplicates (keep first occurrence)
                let mut seen = std::collections::HashSet::new();
                let mut unique = Vec::new();
                for elem in &elements {
                    // Create a simple hash key from the value
                    let key = format!("{:?}", elem);
                    if seen.insert(key) {
                        unique.push(elem.clone());
                    }
                }
                Ok(Value::list(List::from_vec(unique)))
            }
            "reject" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'reject' expects 1 argument, but got {}",
                        args.len()
                    )));
                }

                // Reject is opposite of filter
                match &args[0].kind {
                    ValueKind::Symbol(predicate_name) => {
                        let mut results = Vec::new();
                        for element in &elements {
                            if !self.apply_named_predicate(element, predicate_name)? {
                                results.push(element.clone());
                            }
                        }
                        Ok(Value::list(List::from_vec(results)))
                    }
                    ValueKind::Function(func) => {
                        let mut results = Vec::new();
                        for element in &elements {
                            let result = self.call_function(func, &[element.clone()])?;
                            if !result.is_truthy() {
                                results.push(element.clone());
                            }
                        }
                        Ok(Value::list(List::from_vec(results)))
                    }
                    _other => {
                        return Err(GraphoidError::runtime(format!(
                            "Method 'reject' expects function or symbol, got {}",
                            args[0].type_name()
                        )));
                    }
                }
            }
            "compact" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'compact' expects 0 arguments, but got {}",
                        args.len()
                    )));
                }

                // Remove all none values
                let compacted: Vec<Value> = elements
                    .iter()
                    .filter(|v| !matches!(&v.kind, ValueKind::None))
                    .cloned()
                    .collect();
                Ok(Value::list(List::from_vec(compacted)))
            }
            "select" => {
                // select is an alias for filter
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'select' expects 1 argument, but got {}",
                        args.len()
                    )));
                }

                match &args[0].kind {
                    ValueKind::Symbol(predicate_name) => {
                        let mut results = Vec::new();
                        for element in &elements {
                            if self.apply_named_predicate(element, predicate_name)? {
                                results.push(element.clone());
                            }
                        }
                        Ok(Value::list(List::from_vec(results)))
                    }
                    ValueKind::Function(func) => {
                        let mut results = Vec::new();
                        for element in &elements {
                            let result = self.call_function(func, &[element.clone()])?;
                            if result.is_truthy() {
                                results.push(element.clone());
                            }
                        }
                        Ok(Value::list(List::from_vec(results)))
                    }
                    _other => {
                        return Err(GraphoidError::runtime(format!(
                            "Method 'select' expects function or symbol, got {}",
                            args[0].type_name()
                        )));
                    }
                }
            }
            "append" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "append() expects 1 argument, but got {}",
                        args.len()
                    )));
                }

                // Clone list
                let mut new_list = list.clone();

                // Apply transformation rules with executor context (handles both standard and function-based)
                let transformed_value = self.apply_transformation_rules_with_context(args[0].clone(), &new_list.graph.rules)?;

                // Append without re-applying behaviors (already done above)
                new_list.append_raw(transformed_value)?;
                Ok(Value::list(new_list))
            }
            "index_of" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'index_of' expects 1 argument, but got {}",
                        args.len()
                    )));
                }
                let search_value = &args[0];
                for (idx, element) in elements.iter().enumerate() {
                    if element == search_value {
                        return Ok(Value::number(idx as f64));
                    }
                }
                // Not found, return -1
                Ok(Value::number(-1.0))
            }
            "prepend" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'prepend' expects 1 argument, but got {}",
                        args.len()
                    )));
                }
                let mut new_list = list.clone();
                let transformed_value = self.apply_transformation_rules_with_context(args[0].clone(), &new_list.graph.rules)?;
                new_list.prepend_raw(transformed_value)?;
                Ok(Value::list(new_list))
            }
            "insert" => {
                if args.len() != 2 {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'insert' expects 2 arguments (index, value), but got {}",
                        args.len()
                    )));
                }
                let index = match &args[0].kind {
                    ValueKind::Number(n) => *n as usize,
                    _other => {
                        return Err(GraphoidError::type_error("number", args[0].type_name()));
                    }
                };
                let mut new_list = list.clone();
                let transformed_value = self.apply_transformation_rules_with_context(args[1].clone(), &new_list.graph.rules)?;
                new_list.insert_at_raw(index, transformed_value)?;
                Ok(Value::list(new_list))
            }
            "remove" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'remove' expects 1 argument, but got {}",
                        args.len()
                    )));
                }
                let mut new_list = list.clone();
                new_list.remove_value(&args[0])?;
                Ok(Value::list(new_list))
            }
            "remove_at_index" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'remove_at_index' expects 1 argument, but got {}",
                        args.len()
                    )));
                }
                let index = match &args[0].kind {
                    ValueKind::Number(n) => *n as usize,
                    _other => {
                        return Err(GraphoidError::type_error("number", args[0].type_name()));
                    }
                };
                let mut new_list = list.clone();
                new_list.remove_at_index(index)?;
                Ok(Value::list(new_list))
            }
            "pop" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'pop' expects 0 arguments, but got {}",
                        args.len()
                    )));
                }
                // pop() returns the last element (like last()) but is typically used with !
                // for mutation. Without !, it just returns the value.
                let elements = list.to_vec();
                elements.last()
                    .cloned()
                    .ok_or_else(|| GraphoidError::runtime("Cannot pop from empty list".to_string()))
            }
            "clear" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'clear' expects 0 arguments, but got {}",
                        args.len()
                    )));
                }
                let mut new_list = list.clone();
                new_list.clear();
                Ok(Value::list(new_list))
            }
            "reduce" => {
                if args.len() != 2 {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'reduce' expects 2 arguments (initial, function), but got {}",
                        args.len()
                    )));
                }
                let mut accumulator = args[0].clone();
                let func = match &args[1].kind {
                    ValueKind::Function(f) => f,
                    _other => {
                        return Err(GraphoidError::type_error("function", args[1].type_name()));
                    }
                };

                for element in &elements {
                    accumulator = self.call_function(func, &[accumulator, element.clone()])?;
                }

                Ok(accumulator)
            }
            _ => Err(GraphoidError::runtime(format!(
                "List does not have method '{}'",
                method
            ))),
        }
    }

}

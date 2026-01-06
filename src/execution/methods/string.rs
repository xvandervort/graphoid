//! String method implementations for the Graphoid executor.
//!
//! This module contains string-specific method handling,
//! extracted from the main executor for better code organization.

use crate::ast::Expr;
use crate::error::{GraphoidError, Result};
use crate::execution::executor::Executor;
use crate::values::{Value, ValueKind, List};

impl Executor {
    // =========================================================================
    // String Static Methods
    // =========================================================================

    pub(crate) fn eval_string_static_method(&self, method: &str, args: &[Value]) -> Result<Value> {
        match method {
            "generate" => {
                if args.len() != 2 {
                    return Err(GraphoidError::runtime(format!(
                        "string.generate() expects 2 arguments, but got {}",
                        args.len()
                    )));
                }

                // Mode detection: Check second argument type
                match &args[1].kind {
                    ValueKind::Number(count) => {
                        // Repetition mode: string.generate(str, count)
                        let str_to_repeat = match &args[0].kind {
                            ValueKind::String(s) => s,
                            _other => {
                                return Err(GraphoidError::type_error("string", args[0].type_name()));
                            }
                        };

                        if *count < 0.0 {
                            return Err(GraphoidError::runtime(
                                "string.generate() count cannot be negative".to_string()
                            ));
                        }

                        let count_usize = *count as usize;
                        Ok(Value::string(str_to_repeat.repeat(count_usize)))
                    }
                    ValueKind::String(to_char) => {
                        // Sequence mode: string.generate(from_char, to_char)
                        let from_char = match &args[0].kind {
                            ValueKind::String(s) => s,
                            _other => {
                                return Err(GraphoidError::type_error("string", args[0].type_name()));
                            }
                        };

                        // Validate single characters
                        if from_char.chars().count() != 1 {
                            return Err(GraphoidError::runtime(format!(
                                "string.generate() sequence mode requires single character, got '{}' ({} chars)",
                                from_char,
                                from_char.chars().count()
                            )));
                        }
                        if to_char.chars().count() != 1 {
                            return Err(GraphoidError::runtime(format!(
                                "string.generate() sequence mode requires single character, got '{}' ({} chars)",
                                to_char,
                                to_char.chars().count()
                            )));
                        }

                        let from = from_char.chars().next().unwrap() as u32;
                        let to = to_char.chars().next().unwrap() as u32;

                        // Generate character sequence
                        let mut result = String::new();
                        if from <= to {
                            // Forward sequence
                            for code in from..=to {
                                if let Some(ch) = char::from_u32(code) {
                                    result.push(ch);
                                }
                            }
                        } else {
                            // Reverse sequence
                            for code in (to..=from).rev() {
                                if let Some(ch) = char::from_u32(code) {
                                    result.push(ch);
                                }
                            }
                        }

                        Ok(Value::string(result))
                    }
                    _other => {
                        return Err(GraphoidError::runtime(format!(
                            "string.generate() expects second argument to be number (repetition mode) or string (sequence mode), got {}",
                            args[1].type_name()
                        )));
                    }
                }
            }
            _ => Err(GraphoidError::runtime(format!(
                "string does not have static method '{}'",
                method
            ))),
        }
    }

    /// Evaluates static methods on the time type (e.g., time.now, time.today).

    // =========================================================================
    // String Helper Functions
    // =========================================================================

    pub(crate) fn extract_pattern_symbol(arg: &Value) -> Result<&str> {
        match &arg.kind {
            ValueKind::Symbol(p) => Ok(p.as_str()),
            _ => Err(GraphoidError::runtime(
                "Expected a pattern symbol (e.g., :digits, :letters)".to_string()
            ))
        }
    }

    /// Helper function to check if a character matches a pattern symbol.
    /// Used for string pattern matching methods like contains(), extract(), etc.
    pub(crate) fn matches_pattern(ch: char, pattern: &str) -> bool {
        match pattern {
            "digits" | "numbers" => ch.is_numeric(),
            "letters" => ch.is_alphabetic(),
            "uppercase" => ch.is_uppercase(),
            "lowercase" => ch.is_lowercase(),
            "spaces" | "whitespace" => ch.is_whitespace(),
            "punctuation" => ch.is_ascii_punctuation(),
            "alphanumeric" => ch.is_alphanumeric(),
            "symbols" => !ch.is_alphanumeric() && !ch.is_whitespace(),
            _ => false,
        }
    }

    /// Helper function to extract sequences of characters matching a pattern.
    /// Used by extract() method for patterns like :words, :numbers, etc.
    pub(crate) fn extract_sequences<F>(s: &str, matcher: F) -> Vec<String>
    where
        F: Fn(char) -> bool,
    {
        let mut sequences = Vec::new();
        let mut current = String::new();

        for ch in s.chars() {
            if matcher(ch) {
                current.push(ch);
            } else if !current.is_empty() {
                sequences.push(current.clone());
                current.clear();
            }
        }

        if !current.is_empty() {
            sequences.push(current);
        }

        sequences
    }

    /// Helper function to check if a string looks like an email.
    /// Simple heuristic: word@word.word
    pub(crate) fn is_email_like(s: &str) -> bool {
        let parts: Vec<&str> = s.split('@').collect();
        if parts.len() != 2 {
            return false;
        }

        let domain_parts: Vec<&str> = parts[1].split('.').collect();
        domain_parts.len() >= 2
            && !parts[0].is_empty()
            && domain_parts.iter().all(|p| !p.is_empty())
    }

    /// Helper function to extract email addresses from a string.
    pub(crate) fn extract_emails(s: &str) -> Vec<String> {
        let mut emails = Vec::new();
        let words: Vec<&str> = s.split_whitespace().collect();

        for word in words {
            // Remove trailing punctuation
            let cleaned = word.trim_end_matches(|c: char| c.is_ascii_punctuation());
            if Self::is_email_like(cleaned) {
                emails.push(cleaned.to_string());
            }
        }

        emails
    }

    /// Evaluates a method call on a string.
    pub(crate) fn eval_string_method(&mut self, s: &str, method: &str, args: &[Value], object_expr: &Expr) -> Result<Value> {
        match method {
            "length" | "size" | "len" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "String method '{}' takes no arguments, but got {}",
                        method,
                        args.len()
                    )));
                }
                Ok(Value::number(s.chars().count() as f64))
            }
            // List-like methods: strings behave as graphs of characters
            "first" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'first' takes no arguments, but got {}",
                        args.len()
                    )));
                }
                match s.chars().next() {
                    Some(c) => Ok(Value::string(c.to_string())),
                    None => Ok(Value::none()),
                }
            }
            "last" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'last' takes no arguments, but got {}",
                        args.len()
                    )));
                }
                match s.chars().last() {
                    Some(c) => Ok(Value::string(c.to_string())),
                    None => Ok(Value::none()),
                }
            }
            "is_empty" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'is_empty' takes no arguments, but got {}",
                        args.len()
                    )));
                }
                Ok(Value::boolean(s.is_empty()))
            }
            "slice" => {
                // Alias for substring - strings behave like lists
                if args.len() != 2 {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'slice' expects 2 arguments (start, end), but got {}",
                        args.len()
                    )));
                }
                let start = match &args[0].kind {
                    ValueKind::Number(n) => *n as usize,
                    _other => {
                        return Err(GraphoidError::type_error("number", args[0].type_name()));
                    }
                };
                let end = match &args[1].kind {
                    ValueKind::Number(n) => *n as usize,
                    _other => {
                        return Err(GraphoidError::type_error("number", args[1].type_name()));
                    }
                };

                let chars: Vec<char> = s.chars().collect();
                let start = start.min(chars.len());
                let end = end.min(chars.len());

                if start > end {
                    return Ok(Value::string(String::new()));
                }

                Ok(Value::string(chars[start..end].iter().collect()))
            }
            // Functional methods: strings behave as graphs of characters
            "map" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'map' expects 1 argument, but got {}",
                        args.len()
                    )));
                }

                match &args[0].kind {
                    ValueKind::Function(func) => {
                        // Apply the function to each character
                        let mut results = Vec::new();
                        for c in s.chars() {
                            let char_value = Value::string(c.to_string());
                            let result = self.call_function(func, &[char_value])?;
                            results.push(result);
                        }
                        Ok(Value::list(List::from_vec(results)))
                    }
                    _other => {
                        return Err(GraphoidError::runtime(format!(
                            "String method 'map' expects function, got {}",
                            args[0].type_name()
                        )));
                    }
                }
            }
            "filter" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'filter' expects 1 argument, but got {}",
                        args.len()
                    )));
                }

                match &args[0].kind {
                    ValueKind::Function(func) => {
                        // Filter characters where predicate returns true
                        let mut result = String::new();
                        for c in s.chars() {
                            let char_value = Value::string(c.to_string());
                            let keep = self.call_function(func, &[char_value])?;
                            if keep.is_truthy() {
                                result.push(c);
                            }
                        }
                        Ok(Value::string(result))
                    }
                    _other => {
                        return Err(GraphoidError::runtime(format!(
                            "String method 'filter' expects function, got {}",
                            args[0].type_name()
                        )));
                    }
                }
            }
            "reject" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'reject' expects 1 argument, but got {}",
                        args.len()
                    )));
                }

                match &args[0].kind {
                    ValueKind::Function(func) => {
                        // Keep characters where predicate returns false
                        let mut result = String::new();
                        for c in s.chars() {
                            let char_value = Value::string(c.to_string());
                            let reject = self.call_function(func, &[char_value])?;
                            if !reject.is_truthy() {
                                result.push(c);
                            }
                        }
                        Ok(Value::string(result))
                    }
                    _other => {
                        return Err(GraphoidError::runtime(format!(
                            "String method 'reject' expects function, got {}",
                            args[0].type_name()
                        )));
                    }
                }
            }
            "each" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'each' expects 1 argument, but got {}",
                        args.len()
                    )));
                }

                match &args[0].kind {
                    ValueKind::Function(func) => {
                        // Call function for each character (for side effects)
                        for c in s.chars() {
                            let char_value = Value::string(c.to_string());
                            self.call_function(func, &[char_value])?;
                        }
                        Ok(Value::none())
                    }
                    _other => {
                        return Err(GraphoidError::runtime(format!(
                            "String method 'each' expects function, got {}",
                            args[0].type_name()
                        )));
                    }
                }
            }
            "upper" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'upper' takes no arguments, but got {}",
                        args.len()
                    )));
                }
                Ok(Value::string(s.to_uppercase()))
            }
            "lower" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'lower' takes no arguments, but got {}",
                        args.len()
                    )));
                }
                Ok(Value::string(s.to_lowercase()))
            }
            "trim" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'trim' takes no arguments, but got {}",
                        args.len()
                    )));
                }
                Ok(Value::string(s.trim().to_string()))
            }
            "reverse" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'reverse' takes no arguments, but got {}",
                        args.len()
                    )));
                }
                Ok(Value::string(s.chars().rev().collect()))
            }
            "substring" => {
                if args.len() != 2 {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'substring' expects 2 arguments (start, end), but got {}",
                        args.len()
                    )));
                }
                let start = match &args[0].kind {
                    ValueKind::Number(n) => *n as usize,
                    _other => {
                        return Err(GraphoidError::type_error("number", args[0].type_name()));
                    }
                };
                let end = match &args[1].kind {
                    ValueKind::Number(n) => *n as usize,
                    _other => {
                        return Err(GraphoidError::type_error("number", args[1].type_name()));
                    }
                };

                let chars: Vec<char> = s.chars().collect();
                let start = start.min(chars.len());
                let end = end.min(chars.len());

                if start > end {
                    return Ok(Value::string(String::new()));
                }

                Ok(Value::string(chars[start..end].iter().collect()))
            }
            "split" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'split' expects 1 argument (delimiter), but got {}",
                        args.len()
                    )));
                }
                let delimiter = match &args[0].kind {
                    ValueKind::String(d) => d,
                    _other => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                let parts: Vec<Value> = s.split(delimiter.as_str())
                    .map(|part| Value::string(part.to_string()))
                    .collect();

                Ok(Value::list(crate::values::List::from_vec(parts)))
            }
            "starts_with" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'starts_with' expects 1 argument (prefix), but got {}",
                        args.len()
                    )));
                }
                let prefix = match &args[0].kind {
                    ValueKind::String(p) => p,
                    _other => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                Ok(Value::boolean(s.starts_with(prefix)))
            }
            "ends_with" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'ends_with' expects 1 argument (suffix), but got {}",
                        args.len()
                    )));
                }
                let suffix = match &args[0].kind {
                    ValueKind::String(suf) => suf,
                    _other => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                Ok(Value::boolean(s.ends_with(suffix)))
            }
            "contains" => {
                if args.is_empty() {
                    return Err(GraphoidError::runtime(
                        "String method 'contains' expects at least 1 argument".to_string()
                    ));
                }

                // Check first argument to determine mode
                match &args[0].kind {
                    // Pattern matching mode: contains(mode, patterns...)
                    ValueKind::Symbol(mode) => {
                        if args.len() < 2 {
                            return Err(GraphoidError::runtime(
                                "Pattern matching contains() requires mode and at least one pattern".to_string()
                            ));
                        }

                        // Extract pattern symbols from remaining args
                        let mut patterns = Vec::new();
                        for arg in &args[1..] {
                            match &arg.kind {
                                ValueKind::Symbol(pattern) => patterns.push(pattern.as_str()),
                                _ => {
                                    return Err(GraphoidError::runtime(
                                        "Pattern matching contains() expects symbol patterns".to_string()
                                    ));
                                }
                            }
                        }

                        // Apply the appropriate mode
                        let result = match mode.as_str() {
                            "any" => {
                                // Check if string contains at least one match of any pattern
                                s.chars().any(|ch| {
                                    patterns.iter().any(|&pattern| Self::matches_pattern(ch, pattern))
                                })
                            }
                            "all" => {
                                // Check if string contains at least one match of ALL patterns
                                patterns.iter().all(|&pattern| {
                                    s.chars().any(|ch| Self::matches_pattern(ch, pattern))
                                })
                            }
                            "only" => {
                                // Check if string contains ONLY characters matching patterns
                                if s.is_empty() {
                                    true  // Empty string is "only" anything
                                } else {
                                    s.chars().all(|ch| {
                                        patterns.iter().any(|&pattern| Self::matches_pattern(ch, pattern))
                                    })
                                }
                            }
                            _ => {
                                return Err(GraphoidError::runtime(format!(
                                    "Invalid contains() mode '{}'. Expected :any, :all, or :only",
                                    mode
                                )));
                            }
                        };

                        Ok(Value::boolean(result))
                    }
                    // Substring search mode (original behavior): contains(substring)
                    ValueKind::String(substring) => {
                        if args.len() != 1 {
                            return Err(GraphoidError::runtime(format!(
                                "Substring contains() expects exactly 1 argument, but got {}",
                                args.len()
                            )));
                        }
                        Ok(Value::boolean(s.contains(substring.as_str())))
                    }
                    _ => {
                        Err(GraphoidError::runtime(
                            "contains() first argument must be a mode symbol (:any, :all, :only) or a substring".to_string()
                        ))
                    }
                }
            }
            "extract" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'extract' expects 1 argument (pattern symbol), but got {}",
                        args.len()
                    )));
                }

                // Extract pattern symbol
                let pattern = Self::extract_pattern_symbol(&args[0])?;

                // Extract sequences based on pattern
                let sequences = match pattern {
                    "words" => {
                        // Extract word sequences (letters only)
                        Self::extract_sequences(s, |ch| ch.is_alphabetic())
                    }
                    "numbers" | "digits" => {
                        // Extract number sequences
                        Self::extract_sequences(s, |ch| ch.is_numeric())
                    }
                    "emails" => {
                        // Extract email addresses
                        Self::extract_emails(s)
                    }
                    // For other patterns, extract character sequences
                    _ => {
                        Self::extract_sequences(s, |ch| Self::matches_pattern(ch, pattern))
                    }
                };

                // Convert to list of strings
                let values: Vec<Value> = sequences
                    .into_iter()
                    .map(Value::string)
                    .collect();

                Ok(Value::list(List::from_vec(values)))
            }
            "count" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'count' expects 1 argument (pattern symbol), but got {}",
                        args.len()
                    )));
                }

                // Extract pattern symbol
                let pattern = Self::extract_pattern_symbol(&args[0])?;

                // Count based on pattern type
                let count = match pattern {
                    "words" => {
                        // Count word sequences
                        Self::extract_sequences(s, |ch| ch.is_alphabetic()).len()
                    }
                    "numbers" | "digits" => {
                        // For character-level patterns, count individual characters
                        s.chars().filter(|ch| ch.is_numeric()).count()
                    }
                    "emails" => {
                        // Count email addresses
                        Self::extract_emails(s).len()
                    }
                    // For other patterns, count individual matching characters
                    _ => {
                        s.chars().filter(|ch| Self::matches_pattern(*ch, pattern)).count()
                    }
                };

                Ok(Value::number(count as f64))
            }
            "find" => {
                if args.is_empty() || args.len() > 2 {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'find' expects 1-2 arguments, but got {}",
                        args.len()
                    )));
                }

                // Extract pattern symbol
                let pattern = Self::extract_pattern_symbol(&args[0])?;

                // Find all positions matching the pattern
                let positions: Vec<usize> = s
                    .chars()
                    .enumerate()
                    .filter(|(_i, ch)| Self::matches_pattern(*ch, pattern))
                    .map(|(i, _ch)| i)
                    .collect();

                // Handle second argument (limit or mode)
                if args.len() == 2 {
                    match &args[1].kind {
                        ValueKind::Symbol(mode) if mode == "first" => {
                            // Return first position or -1
                            if let Some(&pos) = positions.first() {
                                Ok(Value::number(pos as f64))
                            } else {
                                Ok(Value::number(-1.0))
                            }
                        }
                        ValueKind::Number(limit) => {
                            // Return first N positions
                            let limit = *limit as usize;
                            let limited: Vec<Value> = positions
                                .into_iter()
                                .take(limit)
                                .map(|pos| Value::number(pos as f64))
                                .collect();
                            Ok(Value::list(List::from_vec(limited)))
                        }
                        _ => {
                            Err(GraphoidError::runtime(
                                "find() second argument must be :first or a number limit".to_string()
                            ))
                        }
                    }
                } else {
                    // No second argument - return all positions
                    let values: Vec<Value> = positions
                        .into_iter()
                        .map(|pos| Value::number(pos as f64))
                        .collect();
                    Ok(Value::list(List::from_vec(values)))
                }
            }
            "replace" => {
                if args.len() != 2 {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'replace' expects 2 arguments (old, new), but got {}",
                        args.len()
                    )));
                }
                let old = match &args[0].kind {
                    ValueKind::String(o) => o,
                    _other => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };
                let new = match &args[1].kind {
                    ValueKind::String(n) => n,
                    _other => {
                        return Err(GraphoidError::type_error("string", args[1].type_name()));
                    }
                };

                Ok(Value::string(s.replace(old.as_str(), new.as_str())))
            }
            "index_of" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'index_of' expects 1 argument (substring), but got {}",
                        args.len()
                    )));
                }
                let substring = match &args[0].kind {
                    ValueKind::String(sub) => sub,
                    _other => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                match s.find(substring.as_str()) {
                    Some(index) => Ok(Value::number(index as f64)),
                    None => Ok(Value::number(-1.0)),
                }
            }
            // Mutating methods
            "upper!" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'upper!' takes no arguments, but got {}",
                        args.len()
                    )));
                }

                // Must be called on a variable, not a literal
                if let Expr::Variable { name, .. } = object_expr {
                    let new_string = s.to_uppercase();
                    self.env.set(name, Value::string(new_string))?;
                    return Ok(Value::none());
                }

                Err(GraphoidError::runtime("upper!() can only be called on variables, not literals".to_string()))
            }
            "lower!" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'lower!' takes no arguments, but got {}",
                        args.len()
                    )));
                }

                // Must be called on a variable, not a literal
                if let Expr::Variable { name, .. } = object_expr {
                    let new_string = s.to_lowercase();
                    self.env.set(name, Value::string(new_string))?;
                    return Ok(Value::none());
                }

                Err(GraphoidError::runtime("lower!() can only be called on variables, not literals".to_string()))
            }
            "trim!" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'trim!' takes no arguments, but got {}",
                        args.len()
                    )));
                }

                // Must be called on a variable, not a literal
                if let Expr::Variable { name, .. } = object_expr {
                    let new_string = s.trim().to_string();
                    self.env.set(name, Value::string(new_string))?;
                    return Ok(Value::none());
                }

                Err(GraphoidError::runtime("trim!() can only be called on variables, not literals".to_string()))
            }
            "reverse!" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'reverse!' takes no arguments, but got {}",
                        args.len()
                    )));
                }

                // Must be called on a variable, not a literal
                if let Expr::Variable { name, .. } = object_expr {
                    let new_string: String = s.chars().rev().collect();
                    self.env.set(name, Value::string(new_string))?;
                    return Ok(Value::none());
                }

                Err(GraphoidError::runtime("reverse!() can only be called on variables, not literals".to_string()))
            }
            // Character code methods (Phase 12 - stdlib robustness)
            "char_code" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'char_code' expects 1 argument (index), but got {}",
                        args.len()
                    )));
                }
                let index = match &args[0].kind {
                    ValueKind::Number(n) => *n as usize,
                    _other => {
                        return Err(GraphoidError::type_error("number", args[0].type_name()));
                    }
                };

                let bytes = s.as_bytes();
                if index >= bytes.len() {
                    return Err(GraphoidError::runtime(format!(
                        "String index {} out of bounds for string of length {}",
                        index,
                        bytes.len()
                    )));
                }

                Ok(Value::number(bytes[index] as f64))
            }
            "to_bytes" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'to_bytes' takes no arguments, but got {}",
                        args.len()
                    )));
                }

                let bytes: Vec<Value> = s.bytes()
                    .map(|b| Value::number(b as f64))
                    .collect();

                Ok(Value::list(List::from_vec(bytes)))
            }
            _ => Err(GraphoidError::runtime(format!(
                "String does not have method '{}'",
                method
            ))),
        }
    }

}

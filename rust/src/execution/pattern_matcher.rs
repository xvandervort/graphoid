//! Pattern Matching Engine - Phase 7
//!
//! Implements the pipe syntax pattern matching for function definitions.
//!
//! # Pattern Types
//!
//! - **Literal**: Matches exact values (`|0|`, `|"hello"|`, `|true|`, `|none|`)
//! - **Variable**: Binds to any value (`|x|`)
//! - **Wildcard**: Matches anything without binding (`|_|`)
//!
//! # Matching Semantics
//!
//! - Patterns are tried in order (first match wins)
//! - Variable patterns bind the matched value to the variable name
//! - If no pattern matches, the function returns `none`
//!
//! # Example
//!
//! ```graphoid
//! fn factorial(n) {
//!     |0| => 1
//!     |1| => 1
//!     |x| => x * factorial(x - 1)
//! }
//! ```

use crate::ast::{Pattern, LiteralValue, PatternClause};
use crate::values::Value;
use crate::error::{GraphoidError, Result};
use std::collections::HashMap;

/// Pattern matching engine for Graphoid functions
pub struct PatternMatcher;

impl PatternMatcher {
    /// Create a new pattern matcher
    pub fn new() -> Self {
        PatternMatcher
    }

    /// Check if a value matches a pattern
    ///
    /// # Examples
    ///
    /// ```
    /// use graphoid::execution::pattern_matcher::PatternMatcher;
    /// use graphoid::ast::{Pattern, LiteralValue};
    /// use graphoid::values::Value;
    /// use graphoid::error::SourcePosition;
    ///
    /// let matcher = PatternMatcher::new();
    /// let pattern = Pattern::Literal {
    ///     value: LiteralValue::Number(0.0),
    ///     position: SourcePosition { line: 1, column: 1, file: None },
    /// };
    /// assert!(matcher.matches(&pattern, &Value::Number(0.0)));
    /// assert!(!matcher.matches(&pattern, &Value::Number(42.0)));
    /// ```
    pub fn matches(&self, pattern: &Pattern, value: &Value) -> bool {
        match pattern {
            Pattern::Wildcard { .. } => true,  // _ matches anything

            Pattern::Variable { .. } => true,  // Variables match anything

            Pattern::Literal { value: lit, .. } => {
                self.literal_matches(lit, value)
            }
        }
    }

    /// Check if a literal pattern matches a value
    fn literal_matches(&self, literal: &LiteralValue, value: &Value) -> bool {
        match (literal, value) {
            (LiteralValue::Number(n1), Value::Number(n2)) => {
                // Use epsilon comparison for floating point
                (n1 - n2).abs() < f64::EPSILON
            }
            (LiteralValue::String(s1), Value::String(s2)) => s1 == s2,
            (LiteralValue::Boolean(b1), Value::Boolean(b2)) => b1 == b2,
            (LiteralValue::None, Value::None) => true,
            _ => false, // Type mismatch
        }
    }

    /// Bind a pattern to a value, returning variable bindings
    ///
    /// Returns a HashMap of variable names to their bound values.
    /// Literal and wildcard patterns produce no bindings.
    ///
    /// # Examples
    ///
    /// ```
    /// use graphoid::execution::pattern_matcher::PatternMatcher;
    /// use graphoid::ast::Pattern;
    /// use graphoid::values::Value;
    /// use graphoid::error::SourcePosition;
    ///
    /// let matcher = PatternMatcher::new();
    /// let pattern = Pattern::Variable {
    ///     name: "x".to_string(),
    ///     position: SourcePosition { line: 1, column: 1, file: None },
    /// };
    /// let bindings = matcher.bind(&pattern, &Value::Number(42.0)).unwrap();
    /// assert_eq!(bindings.get("x"), Some(&Value::Number(42.0)));
    /// ```
    pub fn bind(&self, pattern: &Pattern, value: &Value)
        -> Result<HashMap<String, Value>>
    {
        let mut bindings = HashMap::new();

        match pattern {
            Pattern::Variable { name, .. } => {
                bindings.insert(name.clone(), value.clone());
            }
            Pattern::Wildcard { .. } | Pattern::Literal { .. } => {
                // No bindings for wildcards or literals
            }
        }

        Ok(bindings)
    }

    /// Find the first matching pattern clause for the given arguments
    ///
    /// Returns the matched clause and its bindings, or None if no match found.
    ///
    /// # Arguments
    ///
    /// * `clauses` - The pattern clauses to match against
    /// * `args` - The function arguments (currently must be exactly 1)
    ///
    /// # Returns
    ///
    /// * `Ok(Some((clause, bindings)))` - Found a matching clause
    /// * `Ok(None)` - No match found (function will return none)
    /// * `Err(_)` - Wrong number of arguments
    ///
    /// # Examples
    ///
    /// ```
    /// use graphoid::execution::pattern_matcher::PatternMatcher;
    /// use graphoid::ast::{Pattern, PatternClause, LiteralValue, Expr};
    /// use graphoid::values::Value;
    /// use graphoid::error::SourcePosition;
    ///
    /// let matcher = PatternMatcher::new();
    /// let pos = SourcePosition { line: 1, column: 1, file: None };
    ///
    /// // Create pattern clauses: |0| => "zero", |x| => "other"
    /// let clauses = vec![
    ///     PatternClause {
    ///         pattern: Pattern::Literal {
    ///             value: LiteralValue::Number(0.0),
    ///             position: pos.clone(),
    ///         },
    ///         guard: None,
    ///         body: Expr::Literal {
    ///             value: LiteralValue::String("zero".to_string()),
    ///             position: pos.clone(),
    ///         },
    ///         position: pos.clone(),
    ///     },
    /// ];
    ///
    /// let args = vec![Value::Number(0.0)];
    /// let result = matcher.find_match(&clauses, &args).unwrap();
    /// assert!(result.is_some());
    /// ```
    pub fn find_match<'a>(
        &self,
        clauses: &'a [PatternClause],
        args: &[Value]
    ) -> Result<Option<(&'a PatternClause, HashMap<String, Value>)>> {
        // For now, pattern matching requires exactly 1 argument
        // Future: support multiple-parameter patterns
        if args.len() != 1 {
            return Err(GraphoidError::runtime(
                format!("Pattern matching requires exactly 1 argument, got {}", args.len())
            ));
        }

        let arg = &args[0];

        // Try each clause in order (first match wins)
        for clause in clauses {
            if self.matches(&clause.pattern, arg) {
                let bindings = self.bind(&clause.pattern, arg)?;
                return Ok(Some((clause, bindings)));
            }
        }

        // No match found - return None
        Ok(None)
    }
}

impl Default for PatternMatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::SourcePosition;

    fn pos() -> SourcePosition {
        SourcePosition {
            line: 1,
            column: 1,
            file: None,
        }
    }

    #[test]
    fn test_basic_number_match() {
        let matcher = PatternMatcher::new();
        let pattern = Pattern::Literal {
            value: LiteralValue::Number(42.0),
            position: pos(),
        };

        assert!(matcher.matches(&pattern, &Value::Number(42.0)));
        assert!(!matcher.matches(&pattern, &Value::Number(0.0)));
    }

    #[test]
    fn test_variable_binding() {
        let matcher = PatternMatcher::new();
        let pattern = Pattern::Variable {
            name: "x".to_string(),
            position: pos(),
        };

        let bindings = matcher.bind(&pattern, &Value::Number(99.0)).unwrap();
        assert_eq!(bindings.get("x"), Some(&Value::Number(99.0)));
    }
}

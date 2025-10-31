# Phase 7: Function Pattern Matching - Detailed Implementation Plan

**Duration**: 5-7 days
**Status**: Not started - NEW phase
**Goal**: Implement pipe syntax pattern matching for elegant function definitions

---

## Overview

Pattern matching with pipe syntax allows functions to handle different cases elegantly without verbose if/else chains. This is a foundational feature that:
- Provides elegant syntax for recursive functions
- Enables clean case handling
- Serves as the foundation for graph pattern matching (Phase 9)
- Has **zero dependencies** - can start immediately

**Key Syntax**: `|pattern| => result`

**From Language Specification §2365**:
- Pipe syntax clearly distinguishes pattern matching from lambdas
- Automatic fallthrough to `none` if no pattern matches
- Perfect for recursive functions
- Functional elegance with imperative practicality

---

## Architecture

### AST Nodes Needed

**File**: `src/ast/mod.rs`

```rust
/// Pattern matching clause
#[derive(Debug, Clone, PartialEq)]
pub struct PatternClause {
    pub pattern: Pattern,
    pub guard: Option<Expr>,  // Future: if conditions
    pub body: Expr,
    pub position: SourcePosition,
}

/// Pattern types
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Literal {
        value: LiteralValue,
        position: SourcePosition,
    },
    Variable {
        name: String,
        position: SourcePosition,
    },
    Wildcard {
        position: SourcePosition,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    None,
}

/// Update FunctionDecl to include pattern clauses
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<Parameter>,
    pub body: Vec<Stmt>,
    pub pattern_clauses: Option<Vec<PatternClause>>,  // NEW
    pub position: SourcePosition,
}
```

### Execution Engine

**File**: `src/execution/pattern_matcher.rs` (NEW)

```rust
pub struct PatternMatcher {
    // Pattern matching logic
}

impl PatternMatcher {
    pub fn new() -> Self { ... }

    /// Match a value against a pattern
    pub fn matches(&self, pattern: &Pattern, value: &Value) -> bool { ... }

    /// Extract bindings from a pattern match
    pub fn extract_bindings(&self, pattern: &Pattern, value: &Value)
        -> Result<HashMap<String, Value>, GraphoidError> { ... }

    /// Match value against all clauses, return first match
    pub fn match_clauses(&self, clauses: &[PatternClause], args: &[Value])
        -> Result<Option<&PatternClause>, GraphoidError> { ... }
}
```

---

## Implementation Plan

### Day 1: Parser Extension for Pipe Syntax

**Goal**: Parse `|pattern| => result` syntax

#### 1.1 Lexer Updates

**File**: `src/lexer/mod.rs`

Ensure these tokens exist (already added in Phase 1):
- `Token::Pipe` (`|`)
- `Token::Arrow` (`=>`)

#### 1.2 Parser Updates

**File**: `src/parser/mod.rs`

**Add pattern parsing**:
```rust
impl Parser {
    /// Parse a pattern: |pattern|
    fn parse_pattern(&mut self) -> Result<Pattern, GraphoidError> {
        self.expect(Token::Pipe)?;

        let pattern = match self.current_token() {
            Token::Number(n) => Pattern::Literal {
                value: LiteralValue::Number(*n),
                position: self.position(),
            },
            Token::String(s) => Pattern::Literal {
                value: LiteralValue::String(s.clone()),
                position: self.position(),
            },
            Token::Symbol(sym) if sym == "none" => Pattern::Literal {
                value: LiteralValue::None,
                position: self.position(),
            },
            Token::Symbol(sym) if sym == "true" || sym == "false" => Pattern::Literal {
                value: LiteralValue::Boolean(sym == "true"),
                position: self.position(),
            },
            Token::Symbol(sym) if sym == "_" => Pattern::Wildcard {
                position: self.position(),
            },
            Token::Symbol(name) => Pattern::Variable {
                name: name.clone(),
                position: self.position(),
            },
            _ => return Err(GraphoidError::parse_error(
                format!("Expected pattern, got {:?}", self.current_token()),
                self.position()
            )),
        };

        self.advance();
        self.expect(Token::Pipe)?;

        Ok(pattern)
    }

    /// Parse pattern clause: |pattern| => result
    fn parse_pattern_clause(&mut self) -> Result<PatternClause, GraphoidError> {
        let pattern = self.parse_pattern()?;
        self.expect(Token::Arrow)?;
        let body = self.parse_expression()?;

        Ok(PatternClause {
            pattern,
            guard: None,  // Future: guards
            body,
            position: self.position(),
        })
    }

    /// Update parse_function_decl to handle pattern clauses
    fn parse_function_decl(&mut self) -> Result<Stmt, GraphoidError> {
        // ... existing parsing ...

        // Check if function body starts with pattern clauses
        let pattern_clauses = if self.check(Token::Pipe) {
            let mut clauses = vec![];
            while self.check(Token::Pipe) {
                clauses.push(self.parse_pattern_clause()?);
            }
            Some(clauses)
        } else {
            None
        };

        Ok(Stmt::FunctionDecl {
            name,
            params,
            body: if pattern_clauses.is_some() { vec![] } else { body },
            pattern_clauses,
            position,
        })
    }
}
```

**Tests**: `tests/unit/parser_tests.rs`
```rust
#[test]
fn test_parse_literal_pattern() {
    let code = "fn factorial(n) { |0| => 1 }";
    // Assert pattern clause parsed correctly
}

#[test]
fn test_parse_variable_pattern() {
    let code = "fn double(x) { |n| => n * 2 }";
    // Assert variable binding
}

#[test]
fn test_parse_multiple_patterns() {
    let code = r#"
        fn factorial(n) {
            |0| => 1
            |1| => 1
            |x| => x * factorial(x - 1)
        }
    "#;
    // Assert multiple clauses
}

#[test]
fn test_parse_string_patterns() {
    let code = r#"
        fn get_sound(animal) {
            |"dog"| => "woof"
            |"cat"| => "meow"
        }
    "#;
    // Assert string literal patterns
}
```

---

### Day 2: Pattern Matching Engine

**Goal**: Core pattern matching logic

#### 2.1 Create Pattern Matcher Module

**File**: `src/execution/pattern_matcher.rs` (NEW)

```rust
use crate::ast::{Pattern, LiteralValue, PatternClause};
use crate::values::Value;
use crate::error::{GraphoidError, Result};
use std::collections::HashMap;

pub struct PatternMatcher;

impl PatternMatcher {
    pub fn new() -> Self {
        PatternMatcher
    }

    /// Check if a value matches a pattern
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
            (LiteralValue::Number(n1), Value::Number(n2)) => (n1 - n2).abs() < f64::EPSILON,
            (LiteralValue::String(s1), Value::String(s2)) => s1 == s2,
            (LiteralValue::Boolean(b1), Value::Boolean(b2)) => b1 == b2,
            (LiteralValue::None, Value::None) => true,
            _ => false,
        }
    }

    /// Extract variable bindings from a successful match
    pub fn extract_bindings(&self, pattern: &Pattern, value: &Value)
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

    /// Find the first matching pattern clause
    pub fn find_match<'a>(
        &self,
        clauses: &'a [PatternClause],
        args: &[Value]
    ) -> Result<Option<(&'a PatternClause, HashMap<String, Value>)>> {
        // For now, assume single-parameter functions
        if args.len() != 1 {
            return Err(GraphoidError::runtime_error(
                format!("Pattern matching requires exactly 1 argument, got {}", args.len()),
                None
            ));
        }

        let arg = &args[0];

        for clause in clauses {
            if self.matches(&clause.pattern, arg) {
                let bindings = self.extract_bindings(&clause.pattern, arg)?;
                return Ok(Some((clause, bindings)));
            }
        }

        // No match found - return none
        Ok(None)
    }
}
```

**Tests**: `tests/unit/pattern_matcher_tests.rs` (NEW)
```rust
use graphoid::execution::pattern_matcher::PatternMatcher;
use graphoid::ast::{Pattern, LiteralValue};
use graphoid::values::Value;

#[test]
fn test_literal_number_match() {
    let matcher = PatternMatcher::new();
    let pattern = Pattern::Literal {
        value: LiteralValue::Number(0.0),
        position: Default::default(),
    };
    assert!(matcher.matches(&pattern, &Value::Number(0.0)));
    assert!(!matcher.matches(&pattern, &Value::Number(1.0)));
}

#[test]
fn test_variable_match() {
    let matcher = PatternMatcher::new();
    let pattern = Pattern::Variable {
        name: "x".to_string(),
        position: Default::default(),
    };
    assert!(matcher.matches(&pattern, &Value::Number(42.0)));
}

#[test]
fn test_extract_variable_binding() {
    let matcher = PatternMatcher::new();
    let pattern = Pattern::Variable {
        name: "x".to_string(),
        position: Default::default(),
    };
    let bindings = matcher.extract_bindings(&pattern, &Value::Number(42.0)).unwrap();
    assert_eq!(bindings.get("x"), Some(&Value::Number(42.0)));
}
```

---

### Day 3: Executor Integration

**Goal**: Execute functions with pattern matching

#### 3.1 Update Executor

**File**: `src/execution/executor.rs`

```rust
use crate::execution::pattern_matcher::PatternMatcher;

impl Executor {
    pub fn eval_stmt(&mut self, stmt: &Stmt) -> Result<Option<Value>> {
        match stmt {
            Stmt::FunctionDecl { name, params, body, pattern_clauses, .. } => {
                let func = if let Some(clauses) = pattern_clauses {
                    // Pattern matching function
                    Function::PatternMatching {
                        name: name.clone(),
                        params: params.clone(),
                        clauses: clauses.clone(),
                    }
                } else {
                    // Regular function
                    Function::UserDefined {
                        name: name.clone(),
                        params: params.clone(),
                        body: body.clone(),
                    }
                };

                self.env.define(name.clone(), Value::Function(func));
                Ok(None)
            }
            // ... other statements ...
        }
    }

    fn call_function(&mut self, func: &Function, args: Vec<Value>)
        -> Result<Value>
    {
        match func {
            Function::PatternMatching { name, params, clauses } => {
                let matcher = PatternMatcher::new();

                match matcher.find_match(clauses, &args)? {
                    Some((clause, bindings)) => {
                        // Create new scope with bindings
                        self.env.push_scope();
                        for (name, value) in bindings {
                            self.env.define(name, value);
                        }

                        // Evaluate the clause body
                        let result = self.eval_expr(&clause.body)?;

                        self.env.pop_scope();
                        Ok(result)
                    }
                    None => {
                        // No match - return none
                        Ok(Value::None)
                    }
                }
            }
            Function::UserDefined { .. } => {
                // ... existing logic ...
            }
            // ... other function types ...
        }
    }
}
```

#### 3.2 Update Function Value Type

**File**: `src/values/mod.rs`

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    UserDefined {
        name: String,
        params: Vec<Parameter>,
        body: Vec<Stmt>,
    },
    PatternMatching {
        name: String,
        params: Vec<Parameter>,
        clauses: Vec<PatternClause>,
    },
    // ... other variants ...
}
```

**Tests**: `tests/unit/executor_tests.rs`
```rust
#[test]
fn test_pattern_matching_literal() {
    let code = r#"
        fn is_zero(n) {
            |0| => true
            |x| => false
        }
        is_zero(0)
    "#;
    assert_eq!(eval(code), Value::Boolean(true));
}

#[test]
fn test_pattern_matching_factorial() {
    let code = r#"
        fn factorial(n) {
            |0| => 1
            |1| => 1
            |x| => x * factorial(x - 1)
        }
        factorial(5)
    "#;
    assert_eq!(eval(code), Value::Number(120.0));
}

#[test]
fn test_pattern_matching_no_match() {
    let code = r#"
        fn get_sound(animal) {
            |"dog"| => "woof"
            |"cat"| => "meow"
        }
        get_sound("bird")
    "#;
    assert_eq!(eval(code), Value::None);
}
```

---

### Day 4: Comprehensive Testing

**Goal**: Test all pattern matching scenarios

**File**: `tests/pattern_matching_tests.rs` (NEW)

```rust
use graphoid::values::Value;
// ... test helpers ...

#[test]
fn test_fibonacci_pattern_matching() {
    let code = r#"
        fn fib(n) {
            |0| => 0
            |1| => 1
            |x| => fib(x - 1) + fib(x - 2)
        }
        fib(10)
    "#;
    assert_eq!(eval(code), Value::Number(55.0));
}

#[test]
fn test_string_pattern_matching() {
    let code = r#"
        fn greet(lang) {
            |"english"| => "Hello"
            |"spanish"| => "Hola"
            |"french"| => "Bonjour"
            |x| => "Unknown language"
        }
        greet("spanish")
    "#;
    assert_eq!(eval(code), Value::String("Hola".to_string()));
}

#[test]
fn test_boolean_patterns() {
    let code = r#"
        fn describe(b) {
            |true| => "yes"
            |false| => "no"
        }
        describe(true)
    "#;
    assert_eq!(eval(code), Value::String("yes".to_string()));
}

#[test]
fn test_none_pattern() {
    let code = r#"
        fn handle_optional(val) {
            |none| => "nothing"
            |x| => "something"
        }
        handle_optional(none)
    "#;
    assert_eq!(eval(code), Value::String("nothing".to_string()));
}

#[test]
fn test_wildcard_pattern() {
    let code = r#"
        fn always_match(x) {
            |_| => "matched"
        }
        always_match(42)
    "#;
    assert_eq!(eval(code), Value::String("matched".to_string()));
}

#[test]
fn test_variable_shadowing_in_patterns() {
    let code = r#"
        x = 100
        fn use_x(n) {
            |0| => x
            |x| => x * 2
        }
        [use_x(0), use_x(5)]
    "#;
    // use_x(0) returns outer x=100, use_x(5) returns 5*2=10
    let expected = Value::List(List::from_vec(vec![
        Value::Number(100.0),
        Value::Number(10.0),
    ]));
    assert_eq!(eval(code), expected);
}

#[test]
fn test_pattern_order_matters() {
    let code = r#"
        fn classify(n) {
            |x| => "any"
            |0| => "zero"
        }
        classify(0)
    "#;
    // First pattern matches, so returns "any"
    assert_eq!(eval(code), Value::String("any".to_string()));
}

#[test]
fn test_nested_function_calls_with_patterns() {
    let code = r#"
        fn double(x) { |n| => n * 2 }
        fn triple(x) { |n| => n * 3 }
        fn apply(f, x) { f(x) }

        apply(double, 5)
    "#;
    assert_eq!(eval(code), Value::Number(10.0));
}
```

---

### Day 5: Error Handling & Edge Cases

**Goal**: Robust error handling and edge cases

#### 5.1 Error Cases

**Tests**: `tests/pattern_matching_tests.rs`

```rust
#[test]
fn test_pattern_matching_type_error() {
    let code = r#"
        fn process(x, y) {  # Two parameters
            |5| => "five"
        }
        process(5, 10)
    "#;
    // Should error: pattern matching requires exactly 1 argument
    let result = try_eval(code);
    assert!(result.is_err());
    assert!(result.unwrap_err().message().contains("exactly 1 argument"));
}

#[test]
fn test_empty_pattern_clauses() {
    let code = r#"
        fn bad_func(x) {
            # No pattern clauses
        }
        bad_func(5)
    "#;
    // Should error during parsing or execution
    let result = try_eval(code);
    assert!(result.is_err());
}

#[test]
fn test_pattern_with_complex_expression() {
    let code = r#"
        fn calc(x) {
            |5| => x * 2 + 10  # x should be 5 from binding
        }
        calc(5)
    "#;
    assert_eq!(eval(code), Value::Number(20.0));
}
```

#### 5.2 Performance Considerations

Add documentation about pattern matching performance:
- Linear search through patterns (first match wins)
- Recommend putting common cases first
- Future optimization: pattern compilation

---

### Day 6-7: Documentation & Integration

**Goal**: Complete documentation and integration

#### 6.1 API Documentation

**File**: `src/execution/pattern_matcher.rs`

Add comprehensive rustdoc comments:
```rust
/// Pattern matching engine for Graphoid functions
///
/// Implements the pipe syntax pattern matching described in the language
/// specification §2365.
///
/// # Pattern Types
///
/// - **Literal**: Matches exact values (`|0|`, `|"hello"|`)
/// - **Variable**: Binds to any value (`|x|`)
/// - **Wildcard**: Matches anything without binding (`|_|`)
///
/// # Matching Semantics
///
/// - Patterns are tried in order (first match wins)
/// - Variable patterns bind the matched value to the variable name
/// - If no pattern matches, the function returns `none`
///
/// # Examples
///
/// ```graphoid
/// fn factorial(n) {
///     |0| => 1
///     |1| => 1
///     |x| => x * factorial(x - 1)
/// }
/// ```
```

#### 6.2 Integration Testing

**Tests**: `tests/integration/pattern_matching_integration.rs` (NEW)

```rust
// Test pattern matching with other language features

#[test]
fn test_pattern_matching_with_lists() {
    let code = r#"
        fn process(x) {
            |0| => []
            |n| => [n, n * 2, n * 3]
        }

        result = process(5)
        result.size()
    "#;
    assert_eq!(eval(code), Value::Number(3.0));
}

#[test]
fn test_pattern_matching_with_configure_blocks() {
    let code = r#"
        fn safe_divide(x) {
            |0| => none
            |n| => 100 / n
        }

        configure { error_mode: :lenient } {
            safe_divide(0)
        }
    "#;
    assert_eq!(eval(code), Value::None);
}

#[test]
fn test_higher_order_functions_with_patterns() {
    let code = r#"
        fn apply_twice(f, x) {
            f(f(x))
        }

        fn inc(n) {
            |x| => x + 1
        }

        apply_twice(inc, 5)
    "#;
    assert_eq!(eval(code), Value::Number(7.0));
}
```

#### 6.3 REPL Support

**File**: `src/main.rs`

Ensure pattern matching works in REPL:
```rust
// Test in REPL:
// > fn is_even(n) { |x| => x % 2 == 0 }
// > is_even(4)
// true
```

---

## Success Criteria

- [ ] ✅ Parser handles `|pattern| => result` syntax
- [ ] ✅ Literal patterns work (numbers, strings, booleans, none)
- [ ] ✅ Variable patterns bind correctly
- [ ] ✅ Wildcard patterns (`|_|`) match everything
- [ ] ✅ First-match semantics enforced (order matters)
- [ ] ✅ Fallthrough to `none` when no pattern matches
- [ ] ✅ Recursive functions work (factorial, fibonacci)
- [ ] ✅ String pattern matching works
- [ ] ✅ Variable shadowing handled correctly
- [ ] ✅ Error messages helpful and clear
- [ ] ✅ 40+ tests passing
- [ ] ✅ Zero compiler warnings
- [ ] ✅ REPL support working
- [ ] ✅ Documentation complete

---

## Future Enhancements (Not in This Phase)

### Guards (Conditional Patterns)

From spec §2385 - marked as "future":
```graphoid
fn classify(n) {
    |x| if x < 0 => "negative"
    |0| => "zero"
    |x| if x > 0 => "positive"
}
```

**Deferred to**: Post-Phase 10 (after core language complete)

**Why deferred**: Guards add complexity, and simple patterns cover 90% of use cases

---

## Dependencies

**None** - This phase can start immediately after Phase 6.

---

## Notes for Implementation

1. **Disambiguation from Lambdas**: The `|pattern| =>` syntax is unambiguous because:
   - Lambdas use `x => body` (no pipes)
   - Patterns use `|x| => body` (with pipes)
   - Parser can distinguish at parse time

2. **Single Parameter Only**: Initial implementation supports only single-parameter pattern matching. Multi-parameter patterns are future work.

3. **Pattern Compilation**: Current implementation uses linear search. Future optimization could compile patterns into decision trees.

4. **Guards Future-Proofing**: AST includes `guard: Option<Expr>` field for future guard support, but parser/executor ignore it for now.

---

## References

- **Language Specification**: §2365 "Pattern Matching"
- **Related**: Phase 9 will use this foundation for graph pattern matching
- **Disambiguation**: §2399 explains syntax disambiguation from lambdas

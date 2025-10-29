use std::fmt;
use std::rc::Rc;

use crate::ast::Stmt;
use crate::execution::Environment;
use crate::execution::module_manager::Module;

pub mod graph;
pub mod list;
pub mod hash;
// pub mod tree; // DELETED in Step 5 - trees are now graphs with rules

pub use graph::{Graph, GraphType, ExecutionPlan};
pub use list::List;
pub use hash::Hash;
// Tree type removed - use graph{}.with_ruleset(:tree) instead

/// An error object with type, message, source location, stack trace, and optional cause.
#[derive(Debug, Clone)]
pub struct ErrorObject {
    /// Error type name (e.g., "RuntimeError", "ValueError")
    pub error_type: String,
    /// Error message
    pub message: String,
    /// Source file where error occurred
    pub file: Option<String>,
    /// Line number where error occurred
    pub line: usize,
    /// Column number where error occurred
    pub column: usize,
    /// Call stack at the time of error (function names)
    pub stack_trace: Vec<String>,
    /// Optional underlying cause of this error (for error chaining)
    pub cause: Option<Box<ErrorObject>>,
}

// Custom PartialEq that excludes cause to avoid infinite recursion
impl PartialEq for ErrorObject {
    fn eq(&self, other: &Self) -> bool {
        self.error_type == other.error_type
            && self.message == other.message
            && self.file == other.file
            && self.line == other.line
            && self.column == other.column
            && self.stack_trace == other.stack_trace
        // Intentionally exclude cause from equality
    }
}

impl Eq for ErrorObject {}

// Custom Hash that excludes cause to avoid infinite recursion
impl std::hash::Hash for ErrorObject {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.error_type.hash(state);
        self.message.hash(state);
        self.file.hash(state);
        self.line.hash(state);
        self.column.hash(state);
        self.stack_trace.hash(state);
        // Intentionally exclude cause from hash
    }
}

impl ErrorObject {
    /// Create a new error object
    pub fn new(
        error_type: String,
        message: String,
        file: Option<String>,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            error_type,
            message,
            file,
            line,
            column,
            stack_trace: Vec::new(),
            cause: None,
        }
    }

    /// Create a new error object with stack trace
    pub fn with_stack_trace(
        error_type: String,
        message: String,
        file: Option<String>,
        line: usize,
        column: usize,
        stack_trace: Vec<String>,
    ) -> Self {
        Self {
            error_type,
            message,
            file,
            line,
            column,
            stack_trace,
            cause: None,
        }
    }

    /// Create a RuntimeError
    pub fn runtime(message: String) -> Self {
        Self::new("RuntimeError".to_string(), message, None, 0, 0)
    }

    /// Create a TypeError
    pub fn type_error(message: String) -> Self {
        Self::new("TypeError".to_string(), message, None, 0, 0)
    }

    /// Create a ValueError
    pub fn value_error(message: String) -> Self {
        Self::new("ValueError".to_string(), message, None, 0, 0)
    }

    /// Create an IOError
    pub fn io_error(message: String) -> Self {
        Self::new("IOError".to_string(), message, None, 0, 0)
    }

    /// Set the cause of this error (for error chaining)
    pub fn with_cause(mut self, cause: ErrorObject) -> Self {
        self.cause = Some(Box::new(cause));
        self
    }

    /// Get the full error message including type
    pub fn full_message(&self) -> String {
        format!("{}: {}", self.error_type, self.message)
    }

    /// Get a formatted stack trace string
    pub fn formatted_stack_trace(&self) -> String {
        if self.stack_trace.is_empty() {
            format!(
                "  at {}:{}:{}",
                self.file.as_ref().map(|f| f.as_str()).unwrap_or("<unknown>"),
                self.line,
                self.column
            )
        } else {
            let mut trace = String::new();
            // Add error location first
            trace.push_str(&format!(
                "  at {}:{}:{}\n",
                self.file.as_ref().map(|f| f.as_str()).unwrap_or("<unknown>"),
                self.line,
                self.column
            ));
            // Add call stack
            for func in self.stack_trace.iter().rev() {
                trace.push_str(&format!("  at {}\n", func));
            }
            trace.trim_end().to_string()
        }
    }

    /// Get the full error chain including causes
    pub fn full_chain(&self) -> String {
        let mut chain = self.full_message();
        chain.push('\n');
        chain.push_str(&self.formatted_stack_trace());

        if let Some(ref cause) = self.cause {
            chain.push_str("\nCaused by: ");
            chain.push_str(&cause.full_chain());
        }

        chain
    }
}

/// A function value with its captured environment (closure).
#[derive(Debug, Clone)]
pub struct Function {
    /// Function name (None for anonymous lambdas)
    pub name: Option<String>,
    /// Parameter names
    pub params: Vec<String>,
    /// Function body statements
    pub body: Vec<Stmt>,
    /// Captured environment (for closures)
    pub env: Rc<Environment>,
    /// Node ID in the function graph (set when registered, prevents duplicate registration)
    pub node_id: Option<String>,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        // Functions are equal if they have the same name and parameters
        // (We don't compare body or environment for equality)
        self.name == other.name && self.params == other.params
    }
}

impl Eq for Function {}

impl std::hash::Hash for Function {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash based on name and parameters only
        self.name.hash(state);
        self.params.hash(state);
    }
}

/// Runtime value types in Graphoid.
///
/// IMPORTANT: List and Map are graphs internally.
/// This means ALL collections have the full rule system available.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Numeric value (64-bit floating point)
    Number(f64),
    /// String value
    String(String),
    /// Boolean value
    Boolean(bool),
    /// None/null value
    None,
    /// Symbol literal (e.g., :symbol_name)
    Symbol(String),
    /// List/array of values (backed by linear graph)
    List(List),
    /// Map/dictionary with string keys (backed by key-value graph)
    Map(Hash),
    /// Function value (Phase 4)
    Function(Function),
    /// Graph value (Phase 6)
    Graph(Graph),
    /// Module value (Phase 8) - imported module namespace
    Module(Module),
    /// Error object (Phase 9) - raised errors with type and location info
    Error(ErrorObject),
}

impl Value {
    /// Returns true if the value is "truthy" in Graphoid.
    /// Falsy values: `false`, `none`, `0`, empty strings, and empty collections.
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::None => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Map(h) => !h.is_empty(),
            Value::Symbol(_) => true,
            Value::Function(_) => true, // Functions are always truthy
            Value::Graph(g) => g.node_count() > 0,
            Value::Module(_) => true, // Modules are always truthy
            Value::Error(_) => true, // Errors are always truthy
        }
    }

    /// Converts value to a number if possible.
    /// Returns None if conversion is not possible.
    pub fn to_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            Value::Boolean(true) => Some(1.0),
            Value::Boolean(false) => Some(0.0),
            Value::String(s) => s.parse::<f64>().ok(),
            _ => None,
        }
    }

    /// Converts value to a string.
    pub fn to_string_value(&self) -> String {
        match self {
            Value::Number(n) => {
                // Format numbers nicely (no .0 for integers)
                if n.fract() == 0.0 {
                    format!("{:.0}", n)
                } else {
                    n.to_string()
                }
            }
            Value::String(s) => s.clone(),
            Value::Boolean(b) => b.to_string(),
            Value::None => "none".to_string(),
            Value::Symbol(s) => format!(":{}", s),
            Value::List(list) => {
                let strs: Vec<String> = list.to_vec().iter().map(|v| v.to_string_value()).collect();
                format!("[{}]", strs.join(", "))
            }
            Value::Map(hash) => {
                let pairs: Vec<String> = hash.to_hashmap()
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, v.to_string_value()))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            }
            Value::Function(func) => {
                if let Some(name) = &func.name {
                    format!("<function {}>", name)
                } else {
                    format!("<lambda({})>", func.params.join(", "))
                }
            }
            Value::Graph(g) => {
                format!("<graph: {} nodes, {} edges>", g.node_count(), g.edge_count())
            }
            Value::Module(m) => {
                format!("<module {}>", m.name)
            }
            Value::Error(e) => e.full_message(),
        }
    }

    /// Returns the type name of the value as a string.
    pub fn type_name(&self) -> &str {
        match self {
            Value::Number(_) => "num",
            Value::String(_) => "string",
            Value::Boolean(_) => "bool",
            Value::None => "none",
            Value::Symbol(_) => "symbol",
            Value::List(_) => "list",
            Value::Map(_) => "map",
            Value::Function(_) => "function",
            Value::Graph(_) => "graph",
            Value::Module(_) => "module",
            Value::Error(_) => "error",
        }
    }
}

/// Display implementation for user-friendly output.
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_value_creation() {
        let num = Value::Number(42.0);
        let str_val = Value::String("hello".to_string());
        let bool_val = Value::Boolean(true);
        let none_val = Value::None;
        let sym = Value::Symbol("test".to_string());

        assert_eq!(num, Value::Number(42.0));
        assert_eq!(str_val, Value::String("hello".to_string()));
        assert_eq!(bool_val, Value::Boolean(true));
        assert_eq!(none_val, Value::None);
        assert_eq!(sym, Value::Symbol("test".to_string()));
    }

    #[test]
    fn test_is_truthy() {
        assert!(Value::Boolean(true).is_truthy());
        assert!(!Value::Boolean(false).is_truthy());
        assert!(!Value::None.is_truthy());
        assert!(!Value::Number(0.0).is_truthy());
        assert!(Value::Number(1.0).is_truthy());
        assert!(Value::Number(-5.0).is_truthy());
        assert!(Value::String("hello".to_string()).is_truthy());
        assert!(!Value::String("".to_string()).is_truthy());
        assert!(Value::Symbol("test".to_string()).is_truthy());
        assert!(Value::List(List::from_vec(vec![Value::Number(1.0)])).is_truthy());
        assert!(!Value::List(List::new()).is_truthy());
    }

    #[test]
    fn test_to_number() {
        assert_eq!(Value::Number(42.5).to_number(), Some(42.5));
        assert_eq!(Value::Boolean(true).to_number(), Some(1.0));
        assert_eq!(Value::Boolean(false).to_number(), Some(0.0));
        assert_eq!(Value::String("123.45".to_string()).to_number(), Some(123.45));
        assert_eq!(Value::String("not a number".to_string()).to_number(), None);
        assert_eq!(Value::None.to_number(), None);
    }

    #[test]
    fn test_to_string_value() {
        assert_eq!(Value::Number(42.0).to_string_value(), "42");
        assert_eq!(Value::Number(42.5).to_string_value(), "42.5");
        assert_eq!(Value::String("hello".to_string()).to_string_value(), "hello");
        assert_eq!(Value::Boolean(true).to_string_value(), "true");
        assert_eq!(Value::Boolean(false).to_string_value(), "false");
        assert_eq!(Value::None.to_string_value(), "none");
        assert_eq!(Value::Symbol("test".to_string()).to_string_value(), ":test");
    }

    #[test]
    fn test_type_name() {
        assert_eq!(Value::Number(42.0).type_name(), "num");
        assert_eq!(Value::String("hello".to_string()).type_name(), "string");
        assert_eq!(Value::Boolean(true).type_name(), "bool");
        assert_eq!(Value::None.type_name(), "none");
        assert_eq!(Value::Symbol("test".to_string()).type_name(), "symbol");
        assert_eq!(Value::List(List::new()).type_name(), "list");
        assert_eq!(Value::Map(Hash::new()).type_name(), "map");
    }

    #[test]
    fn test_list_creation() {
        let list = Value::List(List::from_vec(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]));

        if let Value::List(l) = list {
            assert_eq!(l.len(), 3);
            assert_eq!(l.get(0), Some(&Value::Number(1.0)));
        } else {
            panic!("Expected List variant");
        }
    }

    #[test]
    fn test_map_creation() {
        let mut map = HashMap::new();
        map.insert("name".to_string(), Value::String("Alice".to_string()));
        map.insert("age".to_string(), Value::Number(30.0));

        let map_val = Value::Map(Hash::from_hashmap(map));

        if let Value::Map(h) = map_val {
            assert_eq!(h.len(), 2);
            assert_eq!(h.get("name"), Some(&Value::String("Alice".to_string())));
            assert_eq!(h.get("age"), Some(&Value::Number(30.0)));
        } else {
            panic!("Expected Map variant");
        }
    }
}

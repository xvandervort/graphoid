use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use crate::ast::{Stmt, Parameter, PatternClause};
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

// Pattern matching types are defined below in this module and automatically exported

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
    /// Parameter names (for backward compatibility)
    pub params: Vec<String>,
    /// Full parameter information including default values
    pub parameters: Vec<Parameter>,
    /// Function body statements
    pub body: Vec<Stmt>,
    /// Pattern matching clauses (Phase 7) - for pipe syntax functions
    pub pattern_clauses: Option<Vec<PatternClause>>,
    /// Captured environment (for closures) - shared mutable for closure state
    pub env: Rc<RefCell<Environment>>,
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

/// Pattern node object for graph pattern matching
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PatternNode {
    /// Variable name for this node in the pattern
    pub variable: Option<String>,
    /// Node type constraint (e.g., "User"), or None for any type
    pub node_type: Option<String>,
}

/// Pattern edge object for graph pattern matching
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PatternEdge {
    /// Edge type constraint (e.g., "FRIEND"), or None for any type
    pub edge_type: Option<String>,
    /// Edge direction: :outgoing, :incoming, or :both
    pub direction: String, // Symbol as string
}

/// Pattern path object for variable-length graph pattern matching
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PatternPath {
    /// Edge type for the path
    pub edge_type: String,
    /// Minimum path length
    pub min: usize,
    /// Maximum path length
    pub max: usize,
    /// Path direction: :outgoing, :incoming, or :both
    pub direction: String, // Symbol as string
}

/// Pattern match results wrapper that provides filtering and projection methods
#[derive(Debug, Clone)]
pub struct PatternMatchResults {
    /// List of variable bindings (variable_name -> node_id)
    bindings: Vec<std::collections::HashMap<String, String>>,
    /// Reference to the source graph (needed for filtering)
    graph: Graph,
}

impl PatternMatchResults {
    /// Create new pattern match results
    pub fn new(bindings: Vec<std::collections::HashMap<String, String>>, graph: Graph) -> Self {
        PatternMatchResults { bindings, graph }
    }

    /// Get the number of matches
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Check if results are empty
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    /// Get a specific match by index
    pub fn get(&self, index: usize) -> Option<&std::collections::HashMap<String, String>> {
        self.bindings.get(index)
    }

    /// Iterate over the bindings
    pub fn iter(&self) -> std::slice::Iter<'_, std::collections::HashMap<String, String>> {
        self.bindings.iter()
    }

    /// Filter results based on a node's value
    pub fn where_node_value<F>(&self, variable: &str, predicate: F) -> Result<Self, crate::error::GraphoidError>
    where
        F: Fn(&Value) -> bool,
    {
        let filtered: Vec<std::collections::HashMap<String, String>> = self.bindings
            .iter()
            .filter(|binding| {
                if let Some(node_id) = binding.get(variable) {
                    if let Some(node) = self.graph.nodes.get(node_id) {
                        return predicate(&node.value);
                    }
                }
                false
            })
            .cloned()
            .collect();

        Ok(PatternMatchResults::new(filtered, self.graph.clone()))
    }

    /// Filter results based on two nodes' values
    pub fn where_both_nodes<F>(&self, var1: &str, var2: &str, predicate: F) -> Result<Self, crate::error::GraphoidError>
    where
        F: Fn(&Value, &Value) -> bool,
    {
        let filtered: Vec<std::collections::HashMap<String, String>> = self.bindings
            .iter()
            .filter(|binding| {
                if let (Some(node_id1), Some(node_id2)) = (binding.get(var1), binding.get(var2)) {
                    if let (Some(node1), Some(node2)) = (self.graph.nodes.get(node_id1), self.graph.nodes.get(node_id2)) {
                        return predicate(&node1.value, &node2.value);
                    }
                }
                false
            })
            .cloned()
            .collect();

        Ok(PatternMatchResults::new(filtered, self.graph.clone()))
    }

    /// Filter results based on a node's property value
    pub fn where_node_property<F>(&self, variable: &str, property: &str, predicate: F) -> Result<Self, crate::error::GraphoidError>
    where
        F: Fn(Option<&Value>) -> bool,
    {
        let filtered: Vec<std::collections::HashMap<String, String>> = self.bindings
            .iter()
            .filter(|binding| {
                if let Some(node_id) = binding.get(variable) {
                    if let Some(node) = self.graph.nodes.get(node_id) {
                        let prop_value = node.properties.get(property);
                        return predicate(prop_value);
                    }
                }
                false
            })
            .cloned()
            .collect();

        Ok(PatternMatchResults::new(filtered, self.graph.clone()))
    }

    /// Project only specific variables from the match results (similar to Cypher's RETURN clause)
    ///
    /// # Arguments
    /// * `vars` - List of variable names to include in the projection
    ///
    /// # Returns
    /// New PatternMatchResults with only the specified variables in each binding
    ///
    /// # Example
    /// ```ignore
    /// // Match returns: {person: "Alice", friend: "Bob"}
    /// let projected = results.return_vars(vec!["person"]);
    /// // Returns: {person: "Alice"}
    /// ```
    pub fn return_vars(&self, vars: Vec<&str>) -> Self {
        let projected: Vec<std::collections::HashMap<String, String>> = self.bindings
            .iter()
            .map(|binding| {
                let mut new_binding = std::collections::HashMap::new();
                for var in &vars {
                    if let Some(node_id) = binding.get(*var) {
                        new_binding.insert(var.to_string(), node_id.clone());
                    }
                }
                new_binding
            })
            .collect();

        PatternMatchResults::new(projected, self.graph.clone())
    }

    /// Project specific properties from bound nodes (similar to Cypher's RETURN with property access)
    ///
    /// # Arguments
    /// * `specs` - List of property specifications in "variable.property" format
    ///
    /// # Returns
    /// Vector of HashMaps where keys are "variable.property" and values are the property values
    ///
    /// # Example
    /// ```ignore
    /// // Match returns: {person: "Alice", friend: "Bob"}
    /// // Where Alice has age=30, Bob has age=25
    /// let projected = results.return_properties(vec!["person.age", "friend.age"])?;
    /// // Returns: [{"person.age": 30.0, "friend.age": 25.0}]
    /// ```
    pub fn return_properties(&self, specs: Vec<&str>) -> Result<Vec<std::collections::HashMap<String, Value>>, crate::error::GraphoidError> {
        let mut result = Vec::new();

        for binding in &self.bindings {
            let mut props_map = std::collections::HashMap::new();

            for spec in &specs {
                // Parse "variable.property" format
                let parts: Vec<&str> = spec.split('.').collect();
                if parts.len() != 2 {
                    return Err(crate::error::GraphoidError::runtime(
                        format!("Invalid property specification: '{}'. Expected format: 'variable.property'", spec)
                    ));
                }

                let variable = parts[0];
                let property = parts[1];

                // Get the node for this variable
                if let Some(node_id) = binding.get(variable) {
                    if let Some(node) = self.graph.nodes.get(node_id) {
                        // Get the property value (or None if it doesn't exist)
                        let prop_value = node.properties.get(property)
                            .cloned()
                            .unwrap_or_else(|| Value::none());
                        props_map.insert(spec.to_string(), prop_value);
                    } else {
                        // Node doesn't exist - shouldn't happen but handle gracefully
                        props_map.insert(spec.to_string(), Value::none());
                    }
                } else {
                    // Variable not in binding - return None
                    props_map.insert(spec.to_string(), Value::none());
                }
            }

            result.push(props_map);
        }

        Ok(result)
    }
}

// Implement Index trait for array-like access
impl std::ops::Index<usize> for PatternMatchResults {
    type Output = std::collections::HashMap<String, String>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bindings[index]
    }
}

// Implement IntoIterator for &PatternMatchResults to support for loops
impl<'a> IntoIterator for &'a PatternMatchResults {
    type Item = &'a std::collections::HashMap<String, String>;
    type IntoIter = std::slice::Iter<'a, std::collections::HashMap<String, String>>;

    fn into_iter(self) -> Self::IntoIter {
        self.bindings.iter()
    }
}

/// The actual data/kind of a value
#[derive(Debug, Clone, PartialEq)]
pub enum ValueKind {
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
    /// Pattern node object (Phase 9) - for graph pattern matching
    PatternNode(PatternNode),
    /// Pattern edge object (Phase 9) - for graph pattern matching
    PatternEdge(PatternEdge),
    /// Pattern path object (Phase 9) - for variable-length pattern matching
    PatternPath(PatternPath),
}

/// A value with freeze tracking
///
/// All values (including primitives) can be frozen to prevent modification.
#[derive(Debug, Clone)]
pub struct Value {
    /// The actual data/kind of this value
    pub kind: ValueKind,
    /// Whether this value is frozen (immutable)
    pub frozen: bool,
}

// Custom PartialEq that only compares the kind, not the frozen status
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
        // Intentionally exclude frozen from equality comparison
        // Two values with the same data but different frozen status are considered equal
    }
}

impl Eq for Value {}

impl Value {
    // Constructors
    pub fn number(n: f64) -> Self {
        Value { kind: ValueKind::Number(n), frozen: false }
    }

    pub fn string(s: String) -> Self {
        Value { kind: ValueKind::String(s), frozen: false }
    }

    pub fn boolean(b: bool) -> Self {
        Value { kind: ValueKind::Boolean(b), frozen: false }
    }

    pub fn none() -> Self {
        Value { kind: ValueKind::None, frozen: false }
    }

    pub fn symbol(s: String) -> Self {
        Value { kind: ValueKind::Symbol(s), frozen: false }
    }

    pub fn list(l: List) -> Self {
        let frozen = l.graph.is_frozen();
        Value { kind: ValueKind::List(l), frozen }
    }

    pub fn map(h: Hash) -> Self {
        let frozen = h.graph.is_frozen();
        Value { kind: ValueKind::Map(h), frozen }
    }

    pub fn function(f: Function) -> Self {
        Value { kind: ValueKind::Function(f), frozen: false }
    }

    pub fn graph(g: Graph) -> Self {
        let frozen = g.is_frozen();
        Value { kind: ValueKind::Graph(g), frozen }
    }

    pub fn module(m: Module) -> Self {
        Value { kind: ValueKind::Module(m), frozen: false }
    }

    pub fn error(e: ErrorObject) -> Self {
        Value { kind: ValueKind::Error(e), frozen: false }
    }

    pub fn pattern_node(variable: Option<String>, node_type: Option<String>) -> Self {
        Value {
            kind: ValueKind::PatternNode(PatternNode { variable, node_type }),
            frozen: false
        }
    }

    pub fn pattern_edge(edge_type: Option<String>, direction: String) -> Self {
        Value {
            kind: ValueKind::PatternEdge(PatternEdge { edge_type, direction }),
            frozen: false
        }
    }

    pub fn pattern_path(edge_type: String, min: usize, max: usize, direction: String) -> Self {
        Value {
            kind: ValueKind::PatternPath(PatternPath { edge_type, min, max, direction }),
            frozen: false
        }
    }

    /// Returns true if the value is "truthy" in Graphoid.
    /// Falsy values: `false`, `none`, `0`, empty strings, and empty collections.
    pub fn is_truthy(&self) -> bool {
        match &self.kind {
            ValueKind::Boolean(b) => *b,
            ValueKind::None => false,
            ValueKind::Number(n) => *n != 0.0,
            ValueKind::String(s) => !s.is_empty(),
            ValueKind::List(l) => !l.is_empty(),
            ValueKind::Map(h) => !h.is_empty(),
            ValueKind::Symbol(_) => true,
            ValueKind::Function(_) => true, // Functions are always truthy
            ValueKind::Graph(g) => g.node_count() > 0,
            ValueKind::Module(_) => true, // Modules are always truthy
            ValueKind::Error(_) => true, // Errors are always truthy
            ValueKind::PatternNode(_) => true, // Pattern objects are always truthy
            ValueKind::PatternEdge(_) => true,
            ValueKind::PatternPath(_) => true,
        }
    }

    /// Converts value to a number if possible.
    /// Returns None if conversion is not possible.
    pub fn to_number(&self) -> Option<f64> {
        match &self.kind {
            ValueKind::Number(n) => Some(*n),
            ValueKind::Boolean(true) => Some(1.0),
            ValueKind::Boolean(false) => Some(0.0),
            ValueKind::String(s) => s.parse::<f64>().ok(),
            _ => None,
        }
    }

    /// Converts value to a string.
    pub fn to_string_value(&self) -> String {
        match &self.kind {
            ValueKind::Number(n) => {
                // Format numbers nicely (no .0 for integers)
                if n.fract() == 0.0 {
                    format!("{:.0}", n)
                } else {
                    n.to_string()
                }
            }
            ValueKind::String(s) => s.clone(),
            ValueKind::Boolean(b) => b.to_string(),
            ValueKind::None => "none".to_string(),
            ValueKind::Symbol(s) => format!(":{}", s),
            ValueKind::List(list) => {
                let strs: Vec<String> = list.to_vec().iter().map(|v| v.to_string_value()).collect();
                format!("[{}]", strs.join(", "))
            }
            ValueKind::Map(hash) => {
                let pairs: Vec<String> = hash.to_hashmap()
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, v.to_string_value()))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            }
            ValueKind::Function(func) => {
                if let Some(name) = &func.name {
                    format!("<function {}>", name)
                } else {
                    format!("<lambda({})>", func.params.join(", "))
                }
            }
            ValueKind::Graph(g) => {
                format!("<graph: {} nodes, {} edges>", g.node_count(), g.edge_count())
            }
            ValueKind::Module(m) => {
                format!("<module {}>", m.name)
            }
            ValueKind::Error(e) => e.full_message(),
            ValueKind::PatternNode(pn) => {
                match (&pn.variable, &pn.node_type) {
                    (Some(var), Some(typ)) => format!("<pattern node({}, type: \"{}\")>", var, typ),
                    (Some(var), None) => format!("<pattern node(\"{}\")>", var),
                    (None, Some(typ)) => format!("<pattern node(type: \"{}\")>", typ),
                    (None, None) => "<pattern node()>".to_string(),
                }
            }
            ValueKind::PatternEdge(pe) => {
                match &pe.edge_type {
                    Some(et) => format!("<pattern edge(type: \"{}\", direction: :{})>", et, pe.direction),
                    None => format!("<pattern edge(direction: :{})>", pe.direction),
                }
            }
            ValueKind::PatternPath(pp) => {
                format!("<pattern path(edge_type: \"{}\", min: {}, max: {}, direction: :{})>",
                    pp.edge_type, pp.min, pp.max, pp.direction)
            }
        }
    }

    /// Returns the type name of the value as a string.
    pub fn type_name(&self) -> &str {
        match &self.kind {
            ValueKind::Number(_) => "num",
            ValueKind::String(_) => "string",
            ValueKind::Boolean(_) => "bool",
            ValueKind::None => "none",
            ValueKind::Symbol(_) => "symbol",
            ValueKind::List(_) => "list",
            ValueKind::Map(_) => "map",
            ValueKind::Function(_) => "function",
            ValueKind::Graph(_) => "graph",
            ValueKind::Module(_) => "module",
            ValueKind::Error(_) => "error",
            ValueKind::PatternNode(_) => "pattern_node",
            ValueKind::PatternEdge(_) => "pattern_edge",
            ValueKind::PatternPath(_) => "pattern_path",
        }
    }

    // =========================================================================
    // Freeze Control (Phase 8)
    // =========================================================================

    /// Mark this value as frozen (immutable)
    ///
    /// All values can be frozen, including primitives.
    pub fn freeze(&mut self) {
        self.frozen = true;
        // Deep freeze: also freeze nested elements in collections
        match &mut self.kind {
            ValueKind::List(list) => {
                // Freeze each element BEFORE freezing the backing graph
                let len = list.len();
                for i in 0..len {
                    if let Some(node_id) = list.graph.nodes.keys().nth(i) {
                        let node_id = node_id.clone();
                        if let Some(node) = list.graph.nodes.get_mut(&node_id) {
                            node.value.freeze(); // Recursive freeze
                        }
                    }
                }
                list.graph.freeze();
            }
            ValueKind::Map(map) => {
                // Freeze each value BEFORE freezing the backing graph
                let keys: Vec<_> = map.keys();
                for key in keys {
                    let node_id = format!("key_{}", key);
                    if let Some(node) = map.graph.nodes.get_mut(&node_id) {
                        node.value.freeze(); // Recursive freeze
                    }
                }
                map.graph.freeze();
            }
            ValueKind::Graph(graph) => {
                // Freeze all node values in the graph
                for node_id in graph.nodes.keys().cloned().collect::<Vec<_>>() {
                    if let Some(node) = graph.nodes.get_mut(&node_id) {
                        node.value.freeze();
                    }
                }
                graph.freeze();
            }
            _ => {},
        }
    }

    /// Check if this value is frozen
    pub fn is_frozen(&self) -> bool {
        self.frozen
    }

    /// Create an unfrozen deep copy of this value
    ///
    /// The copy will have the same data, but frozen=false
    pub fn deep_copy_unfrozen(&self) -> Self {
        let new_kind = match &self.kind {
            ValueKind::List(list) => {
                let mut new_list = list.clone();
                new_list.graph = list.graph.deep_copy_unfrozen();
                ValueKind::List(new_list)
            }
            ValueKind::Map(map) => {
                let mut new_map = map.clone();
                new_map.graph = map.graph.deep_copy_unfrozen();
                ValueKind::Map(new_map)
            }
            ValueKind::Graph(graph) => ValueKind::Graph(graph.deep_copy_unfrozen()),
            // Primitive types just clone
            other => other.clone(),
        };
        Value { kind: new_kind, frozen: false }
    }
}

/// Display implementation for user-friendly output.
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_value())
    }
}

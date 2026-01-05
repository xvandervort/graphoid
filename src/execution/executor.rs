use crate::ast::{AssignmentTarget, BinaryOp, Expr, GraphMethod, GraphProperty, GraphRule, LiteralValue, Parameter, Stmt, UnaryOp, extract_property_references};
use std::collections::HashMap;
use crate::error::{GraphoidError, Result, SourcePosition};
use crate::execution::Environment;
use crate::execution::config::{ConfigStack, ErrorMode, PrecisionMode};
use crate::execution::error_collector::ErrorCollector;
use crate::execution::function_graph::FunctionGraph;
use crate::execution::module_manager::{ModuleManager, Module, ConfigScope, ErrorMode as ModuleErrorMode, BoundsMode};
use crate::values::{Function, Value, ValueKind, List, Hash, ErrorObject, BigNum, Graph};
use crate::graph::RuleSpec;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;

/// The executor evaluates AST nodes and produces values.
/// Info for writing back mutable argument values after a function call
#[derive(Debug, Clone)]
pub(crate) struct WritebackInfo {
    /// Name of the parameter in the called function
    param_name: String,
    /// Name of the variable in the caller's scope to write back to
    source_var_name: String,
}

pub struct Executor {
    pub(crate) env: Environment,
    pub(crate) call_stack: Vec<String>,
    pub(crate) module_manager: ModuleManager,
    pub(crate) current_file: Option<PathBuf>,
    pub config_stack: ConfigStack,
    pub precision_stack: Vec<Option<usize>>,
    pub error_collector: ErrorCollector,
    /// Global function graph tracking all function definitions and calls
    pub function_graph: Rc<RefCell<FunctionGraph>>,
    /// Global function table (for recursion support and function overloading)
    /// Maps function name to list of overloaded functions with different arities
    pub(crate) global_functions: HashMap<String, Vec<Function>>,
    /// Private symbols (Phase 10: priv keyword support)
    pub(crate) private_symbols: std::collections::HashSet<String>,
    /// Output capture support for exec()
    pub(crate) output_capture_enabled: bool,
    pub(crate) output_buffer: String,
    /// Stack of graph variable names we're currently inside a method of (Phase 15: private methods)
    /// When we're executing a method on a graph stored in variable "MyGraph",
    /// this stack contains "MyGraph" so we can allow private method calls on self.
    pub(crate) method_context_stack: Vec<String>,
    /// Stack of graphs for super call resolution (Phase 16: super calls)
    /// When executing a method, this tracks which graph's method we're in,
    /// so super.method() can find the correct parent.
    pub(crate) super_context_stack: Vec<crate::values::Graph>,
    /// Pending write-backs for mutable arguments (arg! syntax)
    /// Stack of vectors - one per active function call
    pub(crate) writeback_stack: Vec<Vec<WritebackInfo>>,
    /// Stack of `self` values for block context propagation
    /// When a method is called, push its receiver. Blocks called from within
    /// that method will have access to this `self` for implicit method resolution.
    pub(crate) block_self_stack: Vec<BlockSelfEntry>,
}

/// Entry in the block_self_stack tracking the `self` value for block context
#[derive(Clone)]
pub(crate) struct BlockSelfEntry {
    /// The graph value that should be used as `self` in blocks
    value: Value,
}

impl Executor {
    /// Creates a new executor with a fresh environment.
    pub fn new() -> Self {
        Executor {
            env: Environment::new(),
            call_stack: Vec::new(),
            module_manager: ModuleManager::new(),
            current_file: None,
            config_stack: ConfigStack::new(),
            precision_stack: Vec::new(),
            error_collector: ErrorCollector::new(),
            function_graph: Rc::new(RefCell::new(FunctionGraph::new())),
            global_functions: HashMap::new(),
            private_symbols: std::collections::HashSet::new(),
            output_capture_enabled: false,
            output_buffer: String::new(),
            method_context_stack: Vec::new(),
            super_context_stack: Vec::new(),
            writeback_stack: Vec::new(),
            block_self_stack: Vec::new(),
        }
    }

    /// Creates a new executor with a given environment.
    pub fn with_env(env: Environment) -> Self {
        Executor {
            env,
            call_stack: Vec::new(),
            module_manager: ModuleManager::new(),
            current_file: None,
            config_stack: ConfigStack::new(),
            precision_stack: Vec::new(),
            error_collector: ErrorCollector::new(),
            function_graph: Rc::new(RefCell::new(FunctionGraph::new())),
            global_functions: HashMap::new(),
            private_symbols: std::collections::HashSet::new(),
            output_capture_enabled: false,
            output_buffer: String::new(),
            method_context_stack: Vec::new(),
            super_context_stack: Vec::new(),
            writeback_stack: Vec::new(),
            block_self_stack: Vec::new(),
        }
    }

    /// Sets the current file path (for module resolution).
    pub fn set_current_file(&mut self, path: Option<PathBuf>) {
        self.current_file = path;
    }

    /// Executes Graphoid source code and returns the result.
    /// This parses and executes the source in the current environment.
    pub fn execute_source(&mut self, source: &str) -> Result<()> {
        // Tokenize
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;

        // Parse
        let mut parser = Parser::new(tokens);
        let program = parser.parse()?;

        // Create a synthetic __toplevel__ function to represent the main program scope
        // This ensures all top-level function calls have a caller and create edges in the graph
        let toplevel_func = Function {
            name: Some("__toplevel__".to_string()),
            params: Vec::new(),
            parameters: Vec::new(),
            body: Vec::new(),
            pattern_clauses: None,
            env: Rc::new(RefCell::new(self.env.clone())),
            node_id: None,
            is_setter: false,
            is_static: false,
            guard: None,
        };

        let toplevel_id = self.function_graph.borrow_mut().register_function(toplevel_func);
        self.function_graph.borrow_mut().push_call(toplevel_id.clone(), Vec::new());

        // Execute all statements
        for stmt in &program.statements {
            self.eval_stmt(stmt)?;
        }

        // Pop the toplevel function from the call stack
        self.function_graph.borrow_mut().pop_call(Value::none());

        Ok(())
    }

    /// Gets a variable from the environment (for testing).
    pub fn get_variable(&self, name: &str) -> Option<Value> {
        self.env.get(name).ok()
    }

    /// Convert a symbol name to a RuleSpec
    pub(crate) fn symbol_to_rule_spec(symbol: &str, param: Option<f64>) -> Result<RuleSpec> {
        match (symbol, param) {
            // ================================================================
            // Validation Rules (Structural constraints)
            // ================================================================
            ("no_cycles", None) => Ok(RuleSpec::NoCycles),
            ("single_root", None) => Ok(RuleSpec::SingleRoot),
            ("connected", None) => Ok(RuleSpec::Connected),
            ("binary_tree", None) => Ok(RuleSpec::BinaryTree),
            ("no_dups" | "no_duplicates", None) => Ok(RuleSpec::NoDuplicates),
            ("max_degree", Some(n)) => Ok(RuleSpec::MaxDegree(n as usize)),
            ("weighted_edges", None) => Ok(RuleSpec::WeightedEdges),
            ("unweighted_edges", None) => Ok(RuleSpec::UnweightedEdges),

            // ================================================================
            // Transformation Rules (Value transformations)
            // ================================================================
            ("none_to_zero", None) => Ok(RuleSpec::NoneToZero),
            ("none_to_empty", None) => Ok(RuleSpec::NoneToEmpty),
            ("positive", None) => Ok(RuleSpec::Positive),
            ("round_to_int", None) => Ok(RuleSpec::RoundToInt),
            ("uppercase", None) => Ok(RuleSpec::Uppercase),
            ("lowercase", None) => Ok(RuleSpec::Lowercase),

            // ================================================================
            // Freeze Control Rules
            // ================================================================
            ("no_frozen", None) => Ok(RuleSpec::NoFrozen),
            ("copy_elements", None) => Ok(RuleSpec::CopyElements),
            ("shallow_freeze_only", None) => Ok(RuleSpec::ShallowFreezeOnly),

            // ================================================================
            // Method Constraint Rules (Phase 11)
            // ================================================================
            ("no_node_removals", None) => Ok(RuleSpec::NoNodeRemovals),
            ("no_edge_removals", None) => Ok(RuleSpec::NoEdgeRemovals),
            ("read_only", None) => Ok(RuleSpec::ReadOnly),

            // ================================================================
            // Error handling
            // ================================================================
            (name, None) => Err(GraphoidError::runtime(format!(
                "Unknown rule: :{}",
                name
            ))),
            (name, Some(_)) => Err(GraphoidError::runtime(format!(
                "Rule :{} does not accept parameters",
                name
            ))),
        }
    }

    /// Evaluates an expression and returns its value.
    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Literal { value, .. } => self.eval_literal(value),
            Expr::Variable { name, .. } => {
                // Try environment first
                match self.env.get(name) {
                    Ok(value) => Ok(value),
                    Err(_) => {
                        // Phase 2 Implicit Self: Check if `self` is a graph with this property
                        if let Ok(self_value) = self.env.get("self") {
                            if let ValueKind::Graph(ref graph) = self_value.kind {
                                // Check for property in __properties__/ branch
                                let property_node_id = Graph::property_node_id(name);
                                if graph.has_node(&property_node_id) {
                                    if let Some(prop_value) = graph.get_node(&property_node_id) {
                                        return Ok(prop_value.clone());
                                    }
                                }
                                // Note: Implicit method calls are handled in eval_call
                            }
                        }

                        // If not in environment or implicit self, check global functions table
                        // For variable lookup (no arity info), return last defined overload
                        if let Some(funcs) = self.global_functions.get(name) {
                            if let Some(func) = funcs.last() {
                                Ok(Value::function(func.clone()))
                            } else {
                                Err(GraphoidError::undefined_variable(name))
                            }
                        } else {
                            Err(GraphoidError::undefined_variable(name))
                        }
                    }
                }
            },
            Expr::Binary {
                left,
                op,
                right,
                ..
            } => self.eval_binary(left, op, right),
            Expr::Unary { op, operand, .. } => self.eval_unary(op, operand),
            Expr::Call { callee, args, .. } => self.eval_call(callee, args),
            Expr::Lambda { params, body, .. } => self.eval_lambda(params, body),
            Expr::Block { statements, .. } => self.eval_block(statements),
            Expr::List { elements, .. } => self.eval_list(elements),
            Expr::Map { entries, .. } => self.eval_map(entries),
            Expr::Index { object, index, .. } => self.eval_index(object, index),
            Expr::MethodCall { object, method, args, .. } => self.eval_method_call(object, method, args),
            Expr::PropertyAccess { object, property, .. } => self.eval_property_access(object, property),
            Expr::SuperMethodCall { method, args, position } => self.eval_super_method_call(method, args, position),
            Expr::Graph { config, parent, .. } => self.eval_graph(config, parent),
            // Expr::Tree removed in Step 7 - tree{} now desugars to graph{}.with_ruleset(:tree) in parser
            Expr::Conditional {
                condition,
                then_expr,
                else_expr,
                is_unless,
                ..
            } => self.eval_conditional(condition, then_expr, else_expr, *is_unless),
            Expr::Raise { error, position } => {
                // Evaluate the error expression and raise it
                let error_value = self.eval_expr(error)?;

                // Convert to GraphoidError based on value type
                let graphoid_error = match &error_value.kind {
                    // If it's already an Error value, convert to GraphoidError
                    ValueKind::Error(err_obj) => {
                        // All user-raised errors become RuntimeError in GraphoidError
                        // The error type is preserved in the Error object itself
                        GraphoidError::runtime(err_obj.full_message())
                    },
                    // If it's a string, create a RuntimeError
                    ValueKind::String(s) => GraphoidError::runtime(s.clone()),
                    // Any other value, convert to string and create RuntimeError
                    other => GraphoidError::runtime(format!("{:?}", other)),
                };

                // Check if we're in error collection mode
                if self.config_stack.current().error_mode == ErrorMode::Collect {
                    // Collect the error instead of propagating it
                    self.error_collector.collect(
                        graphoid_error,
                        self.current_file.as_ref().map(|p| p.to_string_lossy().to_string()),
                        position.clone(),
                    );
                    // Return None to continue execution
                    Ok(Value::none())
                } else {
                    // Propagate the error (default behavior)
                    Err(graphoid_error)
                }
            }
            Expr::Match { value, arms, position } => self.eval_match(value, arms, position),
            Expr::Instantiate { class_name, overrides, .. } => {
                // CLG instantiation: ClassName { prop: value, ... }
                // 1. Evaluate the class expression (usually a Variable)
                let class_value = self.eval_expr(class_name)?;

                // 2. The class must be a graph (CLG)
                let graph = match &class_value.kind {
                    ValueKind::Graph(g) => g.clone(),
                    _ => {
                        return Err(GraphoidError::type_error(
                            "graph (CLG)",
                            class_value.type_name(),
                        ));
                    }
                };

                // 3. If no overrides, just return the clone (accessing a graph already clones it)
                if overrides.is_empty() {
                    return Ok(class_value);
                }

                // 4. Apply property overrides
                let mut instance = graph;
                for (prop_name, prop_expr) in overrides {
                    let prop_value = self.eval_expr(prop_expr)?;
                    let property_node_id = Graph::property_node_id(&prop_name);

                    // Check if property exists
                    if !instance.has_node(&property_node_id) {
                        return Err(GraphoidError::runtime(format!(
                            "Property '{}' does not exist on this graph. Available properties: {:?}",
                            prop_name,
                            instance.property_node_ids()
                        )));
                    }

                    // Update the property
                    instance.add_node(property_node_id, prop_value)?;
                }

                Ok(Value::graph(instance))
            }
        }
    }

    /// Executes a statement.
    /// Returns Ok(None) for normal statement execution.
    /// Returns Ok(Some(value)) when a return statement is executed.
    pub fn eval_stmt(&mut self, stmt: &Stmt) -> Result<Option<Value>> {
        match stmt {
            Stmt::VariableDecl {
                name,
                type_annotation,
                value,
                is_private,
                ..
            } => {
                let mut val = self.eval_expr(value)?;

                // Phase 1B: Convert to BigNum if type annotation is bignum
                if let Some(type_ann) = type_annotation {
                    if type_ann.base_type == "bignum" {
                        val = self.convert_to_bignum(val)?;
                    }
                }

                // Phase 1A: Truncate if integer mode is active
                let val = self.truncate_if_integer_mode(val);

                // Phase 18: Set type_name on graphs when assigned to a variable
                let val = if let ValueKind::Graph(mut graph) = val.kind {
                    // Only set type_name if it's not already set (e.g., for clones that keep their type)
                    if graph.type_name.is_none() {
                        graph.type_name = Some(name.clone());
                    }
                    Value::graph(graph)
                } else {
                    val
                };

                self.env.define(name.clone(), val);

                // Phase 10: Track private symbols
                if *is_private {
                    self.private_symbols.insert(name.clone());
                }

                Ok(None)
            }
            Stmt::Assignment { target, value, .. } => {
                let val = self.eval_expr(value)?;
                // Phase 1A: Truncate if integer mode is active
                let val = self.truncate_if_integer_mode(val);
                match target {
                    AssignmentTarget::Variable(name) => {
                        // Phase 18: Set type_name on graphs when assigned to a variable
                        let val = if let ValueKind::Graph(mut graph) = val.kind {
                            // Only set type_name if it's not already set (e.g., for clones that keep their type)
                            if graph.type_name.is_none() {
                                graph.type_name = Some(name.clone());
                            }
                            Value::graph(graph)
                        } else {
                            val
                        };

                        // Try to update existing variable, or create new one if it doesn't exist
                        if self.env.exists(name) {
                            self.env.set(name, val)?;
                        } else {
                            // Phase 2 Implicit Self: Check if `self` is a graph with this property
                            // If so, assign to self.property instead of creating a local variable
                            // Properties are stored in __properties__/ branch
                            let property_node_id = Graph::property_node_id(&name);
                            let assigned_to_self = if let Ok(self_value) = self.env.get("self") {
                                if let ValueKind::Graph(mut graph) = self_value.kind {
                                    if graph.has_node(&property_node_id) {
                                        // Update the graph property in __properties__/ branch
                                        graph.add_node(property_node_id, val.clone())?;
                                        // Write the modified graph back to self
                                        self.env.set("self", Value::graph(graph))?;
                                        true
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            } else {
                                false
                            };

                            // If not assigned to self, create new local variable
                            if !assigned_to_self {
                                self.env.define(name.clone(), val);
                            }
                        }
                        Ok(None)
                    }
                    AssignmentTarget::Index { object, index } => {
                        // Evaluate object and index
                        let obj = self.eval_expr(object)?;
                        let idx = self.eval_expr(index)?;

                        // Handle different collection types
                        match obj.kind {
                            ValueKind::Graph(mut graph) => {
                                // For graphs, index must be a string (node ID)
                                let node_id = match &idx.kind {
                                    ValueKind::String(s) => s.clone(),
                                    _ => return Err(GraphoidError::type_error("string", idx.type_name())),
                                };

                                // Add or update the node
                                graph.add_node(node_id, val)?;

                                // Update the graph in the environment
                                // We need to get the variable name from the object expression
                                if let Expr::Variable { name, .. } = object.as_ref() {
                                    self.env.set(name, Value::graph(graph))?;
                                }
                                Ok(None)
                            }
                            ValueKind::Map(mut hash) => {
                                // For maps, index must be a string (key)
                                let key = match &idx.kind {
                                    ValueKind::String(s) => s.clone(),
                                    _ => return Err(GraphoidError::type_error("string", idx.type_name())),
                                };

                                // Apply transformation rules with executor context if hash has them
                                let transformed_val = self.apply_transformation_rules_with_context(val, &hash.graph.rules)?;

                                // Insert key-value pair (using raw to avoid double-applying behaviors)
                                hash.insert_raw(key, transformed_val)?;

                                // Update the map in the environment
                                if let Expr::Variable { name, .. } = object.as_ref() {
                                    self.env.set(name, Value::map(hash))?;
                                }
                                Ok(None)
                            }
                            ValueKind::List(mut list) => {
                                // For lists, index must be a number
                                let index_num = match &idx.kind {
                                    ValueKind::Number(n) => *n as usize,
                                    _ => return Err(GraphoidError::type_error("number", idx.type_name())),
                                };

                                // Apply transformation rules with executor context if list has them
                                let transformed_val = self.apply_transformation_rules_with_context(val, &list.graph.rules)?;

                                // Update element at index (using raw to avoid double-applying behaviors)
                                list.set_raw(index_num, transformed_val)?;

                                // Update the list in the environment
                                if let Expr::Variable { name, .. } = object.as_ref() {
                                    self.env.set(name, Value::list(list))?;
                                }
                                Ok(None)
                            }
                            _ => Err(GraphoidError::runtime(format!(
                                "Cannot use index assignment on type {}",
                                obj.type_name()
                            ))),
                        }
                    }
                    AssignmentTarget::Property { object, property } => {
                        // Property assignment: object.property = value
                        // Don't allow assignment to internal properties
                        if property.starts_with("__") {
                            return Err(GraphoidError::runtime(format!(
                                "Cannot assign to internal property '{}'",
                                property
                            )));
                        }

                        let obj = self.eval_expr(object)?;

                        match obj.kind {
                            ValueKind::Graph(mut graph) => {
                                // Phase 19: Check if there's a setter for this property
                                if let Some(setter_func) = graph.get_setter(property).cloned() {
                                    // Call the setter with the value being assigned
                                    // The setter receives `self` (the graph) and the value as an argument
                                    self.call_graph_method(&graph, &setter_func, &[val], object)?;

                                    // The graph is already updated in the environment by call_graph_method
                                    Ok(None)
                                } else {
                                    // No setter - update property or data node directly
                                    // First, check if this is a CLG property in __properties__/ branch
                                    let property_node_id = Graph::property_node_id(&property);
                                    if graph.has_node(&property_node_id) {
                                        // Update CLG property
                                        graph.add_node(property_node_id, val)?;
                                    } else {
                                        // Add/update as regular user data node
                                        graph.add_node(property.clone(), val)?;
                                    }

                                    // Update the graph in the environment
                                    if let Expr::Variable { name, .. } = object.as_ref() {
                                        self.env.set(name, Value::graph(graph))?;
                                    }
                                    Ok(None)
                                }
                            }
                            ValueKind::Map(mut hash) => {
                                // Apply transformation rules if hash has them
                                let transformed_val = self.apply_transformation_rules_with_context(val, &hash.graph.rules)?;

                                // Insert key-value pair
                                hash.insert_raw(property.clone(), transformed_val)?;

                                // Update the map in the environment
                                if let Expr::Variable { name, .. } = object.as_ref() {
                                    self.env.set(name, Value::map(hash))?;
                                }
                                Ok(None)
                            }
                            _ => Err(GraphoidError::runtime(format!(
                                "Cannot use property assignment on type {}",
                                obj.type_name()
                            ))),
                        }
                    }
                }
            }
            Stmt::FunctionDecl {
                name,
                receiver,
                params,
                body,
                pattern_clauses,
                is_private,
                is_setter,
                is_static,
                guard,
                ..
            } => {
                // Extract parameter names
                let param_names: Vec<String> = params.iter().map(|p| p.name.clone()).collect();

                // Create function value with PLACEHOLDER environment
                // We'll update it after adding the function to the environment to support recursion
                let placeholder_env = Rc::new(RefCell::new(self.env.clone()));
                let mut func = Function {
                    name: Some(name.clone()),
                    params: param_names,
                    parameters: params.clone(),
                    body: body.clone(),
                    pattern_clauses: pattern_clauses.clone(),
                    env: placeholder_env.clone(),
                    node_id: None,
                    is_setter: *is_setter,  // Phase 19: computed property assignment (setters)
                    is_static: *is_static,  // Phase 20: class methods
                    guard: guard.clone(),   // Phase 21: structure-based dispatch
                };

                // Register function in the function graph and store its node_id
                let node_id = self.function_graph.borrow_mut().register_function(func.clone());
                func.node_id = Some(node_id);

                // Check if this is a method (has receiver) or regular function
                if let Some(receiver_name) = receiver {
                    // Method syntax: fn Receiver.method_name()
                    // Look up the receiver graph in the environment and attach the method
                    let receiver_value = self.env.get(receiver_name)?;

                    // Receiver must be a graph
                    match &receiver_value.kind {
                        ValueKind::Graph(graph) => {
                            // Clone the graph, attach the method/setter/static, and update the binding
                            let mut graph_clone = graph.clone();
                            if *is_static {
                                // Phase 20: Attach as a static method
                                graph_clone.attach_static_method(name.clone(), func.clone());
                            } else if *is_setter {
                                // Phase 19: Attach as a setter
                                graph_clone.attach_setter(name.clone(), func.clone());
                            } else {
                                // Regular method
                                graph_clone.attach_method(name.clone(), func.clone());
                            }
                            self.env.define(receiver_name.clone(), Value::graph(graph_clone));
                        }
                        _ => {
                            return Err(GraphoidError::runtime(format!(
                                "Cannot attach method '{}' to '{}': expected graph, got {}",
                                name, receiver_name, receiver_value.type_name()
                            )));
                        }
                    }
                } else {
                    // Regular function - store in environment and global functions table

                    // Store in global functions table (for recursion support and overloading)
                    // Multiple functions with same name but different arities are supported
                    self.global_functions
                        .entry(name.clone())
                        .or_insert_with(Vec::new)
                        .push(func.clone());

                    // Store function in environment (last definition wins for direct calls)
                    self.env.define(name.clone(), Value::function(func.clone()));

                    // NOW update the function's captured environment to include itself for recursion
                    // Since we use Rc<RefCell>, this updates all copies of the function
                    *placeholder_env.borrow_mut() = self.env.clone();

                    // Phase 10: Track private symbols
                    if *is_private {
                        self.private_symbols.insert(name.clone());
                    }
                }

                Ok(None)
            }
            Stmt::Return { value, .. } => {
                let return_value = if let Some(expr) = value {
                    self.eval_expr(expr)?
                } else {
                    Value::none()
                };
                Ok(Some(return_value))
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                let cond_value = self.eval_expr(condition)?;
                if cond_value.is_truthy() {
                    // Execute then branch
                    for stmt in then_branch {
                        if let Some(val) = self.eval_stmt(stmt)? {
                            // Return statement in then branch
                            return Ok(Some(val));
                        }
                    }
                } else if let Some(else_stmts) = else_branch {
                    // Execute else branch
                    for stmt in else_stmts {
                        if let Some(val) = self.eval_stmt(stmt)? {
                            // Return statement in else branch
                            return Ok(Some(val));
                        }
                    }
                }
                Ok(None)
            }
            Stmt::Expression { expr, .. } => {
                // Evaluate expression and discard result
                // In REPL mode, caller may want to print the result
                self.eval_expr(expr)?;
                Ok(None)
            }
            Stmt::While {
                condition,
                body,
                ..
            } => {
                // While loop: evaluate condition, execute body, repeat
                loop {
                    let cond_value = self.eval_expr(condition)?;
                    if !cond_value.is_truthy() {
                        // Condition is false, exit loop
                        break;
                    }

                    // Execute loop body
                    let mut should_break = false;
                    for stmt in body {
                        match self.eval_stmt(stmt) {
                            Ok(Some(val)) => {
                                // Return statement in loop body
                                return Ok(Some(val));
                            }
                            Err(GraphoidError::LoopControl { control }) => {
                                match control {
                                    crate::error::LoopControlType::Break => {
                                        should_break = true;
                                        break;
                                    }
                                    crate::error::LoopControlType::Continue => {
                                        break; // Break inner loop, continue outer loop
                                    }
                                }
                            }
                            Err(e) => return Err(e),
                            Ok(None) => {}
                        }
                    }

                    if should_break {
                        break;
                    }
                }
                Ok(None)
            }
            Stmt::For {
                variable,
                iterable,
                body,
                ..
            } => {
                // For loop: evaluate iterable, iterate over elements
                let iterable_value = self.eval_expr(iterable)?;

                // Get the list of values to iterate over
                let values = match &iterable_value.kind {
                    ValueKind::List(ref items) => items.to_vec(),
                    _other => {
                        return Err(GraphoidError::type_error(
                            "list",
                            iterable_value.type_name(),
                        ));
                    }
                };

                // Iterate over each value
                for value in values {
                    // Bind loop variable to current value
                    if self.env.exists(variable) {
                        self.env.set(variable, value)?;
                    } else {
                        self.env.define(variable.clone(), value);
                    }

                    // Execute loop body
                    for stmt in body {
                        if let Some(val) = self.eval_stmt(stmt)? {
                            // Return statement in loop body
                            return Ok(Some(val));
                        }
                    }
                }
                Ok(None)
            }
            Stmt::Import { module, alias, .. } => {
                // Import a module and create a namespace in the current environment
                let module_value = self.load_module(module, alias.as_ref())?;

                // Determine the binding name:
                // 1. Use explicit import alias if provided (import "foo" as bar)
                // 2. Use module's declared alias if available (module foo alias bar)
                // 3. Use module's declared name (module foo)
                // 4. Fall back to filename stem (already handled in load_module)
                let binding_name = if let Some(alias_name) = alias {
                    alias_name.clone()
                } else if let ValueKind::Module(ref m) = module_value.kind {
                    // Prefer module's declared alias, fall back to module name
                    m.alias.clone().unwrap_or_else(|| m.name.clone())
                } else {
                    module.clone()
                };

                // Bind the module to the environment
                self.env.define(binding_name, module_value);
                Ok(None)
            }
            Stmt::ModuleDecl { name, alias, .. } => {
                // Module declaration - store metadata for later use
                // For now, we just note that this file declares itself as a module
                // The actual module name/alias are used when importing this file
                self.env.define("__module_name__".to_string(), Value::string(name.clone()));
                if let Some(alias_name) = alias {
                    self.env.define("__module_alias__".to_string(), Value::string(alias_name.clone()));
                }
                Ok(None)
            }
            Stmt::Load { path, .. } => {
                // Load statement - inline file contents into current scope
                // Unlike import, this merges variables into the current namespace
                self.execute_load(path)?;
                Ok(None)
            }
            Stmt::Configure { settings, body, .. } => {
                // Evaluate settings and push new config
                let mut config_changes = HashMap::new();
                for (key, value_expr) in settings {
                    let value = self.eval_expr(value_expr)?;
                    config_changes.insert(key.clone(), value);
                }

                // Push new config with changes
                self.config_stack.push_with_changes(config_changes)?;

                // If there's a body, execute it and pop config after (scoped)
                // If no body (file-level), keep config active
                if let Some(body_stmts) = body {
                    let mut result = None;
                    for stmt in body_stmts {
                        if let Some(val) = self.eval_stmt(stmt)? {
                            result = Some(val);
                            break;
                        }
                    }

                    // Pop config after block (restore previous)
                    self.config_stack.pop();

                    Ok(result)
                } else {
                    // File-level configure: keep config active, don't pop
                    Ok(None)
                }
            }
            Stmt::Precision { places, body, .. } => {
                // Push precision onto stack
                self.precision_stack.push(*places);

                // Execute body
                let mut result = None;
                for stmt in body {
                    if let Some(val) = self.eval_stmt(stmt)? {
                        result = Some(val);
                        break;
                    }
                }

                // Pop precision (restore previous)
                self.precision_stack.pop();

                Ok(result)
            }
            Stmt::Try { body, catch_clauses, finally_block, .. } => {
                self.execute_try(body, catch_clauses, finally_block)
            }
            Stmt::Break { .. } => {
                // Break statement - signal loop termination
                Err(GraphoidError::LoopControl {
                    control: crate::error::LoopControlType::Break,
                })
            }
            Stmt::Continue { .. } => {
                // Continue statement - signal loop continuation
                Err(GraphoidError::LoopControl {
                    control: crate::error::LoopControlType::Continue,
                })
            }
            Stmt::GraphDecl {
                name,
                graph_type,
                parent,
                properties,
                methods,
                rules,
                config,
                position,
            } => {
                self.eval_graph_decl(name, graph_type, parent, properties, methods, rules, config, position)
            }
        }
    }

    /// Phase 1A: Truncates numeric values when integer_mode is active
    /// This implements truncation-on-assignment for the :integer directive
    fn truncate_if_integer_mode(&self, value: Value) -> Value {
        // Check if integer mode is active
        if !self.config_stack.current().integer_mode {
            return value;
        }

        // Truncate numeric values (both Number and BigNumber)
        match &value.kind {
            ValueKind::Number(n) => Value::number(n.trunc()),
            ValueKind::BigNumber(bn) => {
                // Phase 1B: Truncate BigNumber values in integer mode
                match bn {
                    BigNum::Float128(f) => {
                        // Convert to f64, truncate, convert back to f128
                        let f64_val: f64 = (*f).into();
                        use f128::f128;
                        Value::bignum(BigNum::Float128(f128::from(f64_val.trunc())))
                    }
                    // Int64 and UInt64 are already integers, pass through
                    BigNum::Int64(_) | BigNum::UInt64(_) => value,
                    // BigInt is already integer, pass through
                    BigNum::BigInt(_) => value,
                }
            }
            // Non-numeric values pass through unchanged
            _ => value,
        }
    }

    /// Phase 1B: Converts a value to BigNum (Float128).
    /// Converts Number to BigNum, passes through existing BigNum, errors on other types.
    fn convert_to_bignum(&self, value: Value) -> Result<Value> {
        match &value.kind {
            ValueKind::Number(n) => {
                // Convert f64 to f128
                use f128::f128;
                let f128_val = f128::from(*n);
                Ok(Value::bignum(BigNum::Float128(f128_val)))
            }
            ValueKind::BigNumber(_) => {
                // Already a bignum, pass through
                Ok(value)
            }
            _ => {
                Err(GraphoidError::type_error(
                    "number or bignum",
                    value.type_name()
                ))
            }
        }
    }


    /// Evaluates a literal value.
    fn eval_literal(&self, lit: &LiteralValue) -> Result<Value> {
        match lit {
            LiteralValue::Number(n) => {
                // Phase 1B: Large literal detection
                // Only promote to BigInt if the value exceeds what Int64/UInt64 can represent
                const I64_MAX: f64 = 9_223_372_036_854_775_807.0; // 2^63 - 1
                const U64_MAX: f64 = 18_446_744_073_709_551_615.0; // 2^64 - 1

                // Phase 1B: Check if we're in high precision mode
                match self.config_stack.current().precision_mode {
                    PrecisionMode::High => {
                        // Phase 1B: High precision defaults to Float128
                        // Only becomes Int64/UInt64 if integer_mode is also active
                        if self.config_stack.current().integer_mode {
                            // Check if literal exceeds Int64/UInt64 range
                            let exceeds_int_range = if self.config_stack.current().unsigned_mode {
                                n.abs() > U64_MAX
                            } else {
                                n.abs() > I64_MAX
                            };

                            // If too large for Int64/UInt64, use BigInt
                            if exceeds_int_range && n.fract() == 0.0 {
                                use num_bigint::BigInt;
                                let big_int = BigInt::from(*n as i64);
                                return Ok(Value::bignum(BigNum::BigInt(big_int)));
                            }

                            // Integer mode: convert to i64/u64 based on unsigned mode
                            if self.config_stack.current().unsigned_mode {
                                Ok(Value::bignum(BigNum::UInt64(*n as u64)))
                            } else {
                                Ok(Value::bignum(BigNum::Int64(*n as i64)))
                            }
                        } else {
                            // Float mode (default): convert to Float128
                            use f128::f128;
                            Ok(Value::bignum(BigNum::Float128(f128::from(*n))))
                        }
                    }
                    PrecisionMode::Extended => {
                        // Phase 1B: Extended precision uses BigInt
                        use num_bigint::BigInt;
                        let big_int = if n.fract() == 0.0 {
                            // Integer value - convert directly
                            BigInt::from(*n as i64)
                        } else {
                            // Fractional value - truncate to integer for BigInt
                            BigInt::from(n.trunc() as i64)
                        };
                        Ok(Value::bignum(BigNum::BigInt(big_int)))
                    }
                    PrecisionMode::Standard => {
                        // Standard mode: use f64
                        // Note: Large values may lose precision, but that's expected in Standard mode
                        Ok(Value::number(*n))
                    }
                }
            }
            LiteralValue::String(s) => Ok(Value::string(s.clone())),
            LiteralValue::Boolean(b) => Ok(Value::boolean(*b)),
            LiteralValue::None => Ok(Value::none()),
            LiteralValue::Symbol(s) => Ok(Value::symbol(s.clone())),
        }
    }

    /// Evaluates a binary expression.
    fn eval_binary(&mut self, left: &Expr, op: &BinaryOp, right: &Expr) -> Result<Value> {
        let left_val = self.eval_expr(left)?;
        let right_val = self.eval_expr(right)?;

        match op {
            // Arithmetic operators
            BinaryOp::Add => self.eval_add(left_val, right_val),
            BinaryOp::Subtract => self.eval_subtract(left_val, right_val),
            BinaryOp::Multiply => self.eval_multiply(left_val, right_val),
            BinaryOp::Divide => self.eval_divide(left_val, right_val),
            BinaryOp::IntDiv => self.eval_int_div(left_val, right_val),
            BinaryOp::Modulo => self.eval_modulo(left_val, right_val),
            BinaryOp::Power => self.eval_power(left_val, right_val),

            // Bitwise operators (Phase 13)
            BinaryOp::BitwiseAnd => self.eval_bitwise_and(left_val, right_val),
            BinaryOp::BitwiseOr => self.eval_bitwise_or(left_val, right_val),
            BinaryOp::BitwiseXor => self.eval_bitwise_xor(left_val, right_val),
            BinaryOp::LeftShift => self.eval_left_shift(left_val, right_val),
            BinaryOp::RightShift => self.eval_right_shift(left_val, right_val),

            // Comparison operators
            BinaryOp::Equal => Ok(Value::boolean(left_val == right_val)),
            BinaryOp::NotEqual => Ok(Value::boolean(left_val != right_val)),
            BinaryOp::Less => self.eval_less(left_val, right_val),
            BinaryOp::LessEqual => self.eval_less_equal(left_val, right_val),
            BinaryOp::Greater => self.eval_greater(left_val, right_val),
            BinaryOp::GreaterEqual => self.eval_greater_equal(left_val, right_val),

            // Logical operators
            BinaryOp::And => Ok(Value::boolean(left_val.is_truthy() && right_val.is_truthy())),
            BinaryOp::Or => Ok(Value::boolean(left_val.is_truthy() || right_val.is_truthy())),

            // Element-wise operators
            BinaryOp::DotAdd => self.eval_element_wise(left_val, right_val, BinaryOp::Add),
            BinaryOp::DotSubtract => self.eval_element_wise(left_val, right_val, BinaryOp::Subtract),
            BinaryOp::DotMultiply => self.eval_element_wise(left_val, right_val, BinaryOp::Multiply),
            BinaryOp::DotDivide => self.eval_element_wise(left_val, right_val, BinaryOp::Divide),
            BinaryOp::DotIntDiv => self.eval_element_wise(left_val, right_val, BinaryOp::IntDiv),
            BinaryOp::DotModulo => self.eval_element_wise(left_val, right_val, BinaryOp::Modulo),
            BinaryOp::DotPower => self.eval_element_wise(left_val, right_val, BinaryOp::Power),
            BinaryOp::DotXor => self.eval_element_wise(left_val, right_val, BinaryOp::BitwiseXor),
            BinaryOp::DotEqual => self.eval_element_wise(left_val, right_val, BinaryOp::Equal),
            BinaryOp::DotNotEqual => self.eval_element_wise(left_val, right_val, BinaryOp::NotEqual),
            BinaryOp::DotLess => self.eval_element_wise(left_val, right_val, BinaryOp::Less),
            BinaryOp::DotLessEqual => self.eval_element_wise(left_val, right_val, BinaryOp::LessEqual),
            BinaryOp::DotGreater => self.eval_element_wise(left_val, right_val, BinaryOp::Greater),
            BinaryOp::DotGreaterEqual => self.eval_element_wise(left_val, right_val, BinaryOp::GreaterEqual),

            _ => Err(GraphoidError::runtime(format!(
                "Unsupported binary operator: {:?}",
                op
            ))),
        }
    }

    /// Evaluates a unary expression.
    fn eval_unary(&mut self, op: &UnaryOp, operand: &Expr) -> Result<Value> {
        let val = self.eval_expr(operand)?;

        match op {
            UnaryOp::Negate => match &val.kind {
                ValueKind::Number(n) => Ok(Value::number(-n)),
                ValueKind::BigNumber(bn) => match bn {
                    BigNum::Int64(v) => {
                        v.checked_neg()
                            .map(|result| Value::bignum(BigNum::Int64(result)))
                            .ok_or_else(|| GraphoidError::runtime("Integer overflow in negation".to_string()))
                    }
                    BigNum::UInt64(_) => {
                        Err(GraphoidError::runtime("Cannot negate unsigned bignum value".to_string()))
                    }
                    BigNum::Float128(f) => {
                        Ok(Value::bignum(BigNum::Float128(-*f)))
                    }
                    BigNum::BigInt(bi) => {
                        Ok(Value::bignum(BigNum::BigInt(-bi)))
                    }
                },
                _ => Err(GraphoidError::type_error("number or bignum", val.type_name())),
            },
            UnaryOp::Not => Ok(Value::boolean(!val.is_truthy())),
            UnaryOp::BitwiseNot => self.eval_bitwise_not(val),
        }
    }

    /// Evaluates a list expression.
    fn eval_list(&mut self, elements: &[Expr]) -> Result<Value> {
        let mut values = Vec::new();
        for elem in elements {
            values.push(self.eval_expr(elem)?);
        }
        Ok(Value::list(List::from_vec(values)))
    }

    /// Evaluates a map expression.
    fn eval_map(&mut self, entries: &[(String, Expr)]) -> Result<Value> {
        let mut map = HashMap::new();
        for (key, value_expr) in entries {
            let value = self.eval_expr(value_expr)?;
            map.insert(key.clone(), value);
        }
        Ok(Value::map(Hash::from_hashmap(map)))
    }

    /// Evaluates a graph expression.
    fn eval_graph(&mut self, config: &[(String, Expr)], parent_expr: &Option<Box<Expr>>) -> Result<Value> {
        use crate::values::{Graph, GraphType};

        // If there's a parent, evaluate it and create child graph via inheritance
        if let Some(parent_box) = parent_expr {
            let parent_value = self.eval_expr(parent_box)?;
            if let ValueKind::Graph(parent_graph) = parent_value.kind {
                // Create child that inherits from parent
                let child = Graph::from_parent(parent_graph);
                return Ok(Value::graph(child));
            } else {
                return Err(GraphoidError::runtime(format!(
                    "Cannot inherit from non-graph type '{}'. Expected graph.",
                    parent_value.type_name()
                )));
            }
        }

        // No parent - create a new empty graph
        // Parse configuration to determine graph type
        let mut graph_type = GraphType::Directed; // Default

        for (key, value_expr) in config {
            if key == "type" {
                let value = self.eval_expr(value_expr)?;
                if let ValueKind::Symbol(s) = &value.kind {
                    match s.as_str() {
                        "directed" => graph_type = GraphType::Directed,
                        "undirected" => graph_type = GraphType::Undirected,
                        _ => return Err(GraphoidError::runtime(format!(
                            "Invalid graph type: :{}. Expected :directed or :undirected",
                            s
                        ))),
                    }
                } else {
                    return Err(GraphoidError::type_error("symbol", value.type_name()));
                }
            }
        }

        Ok(Value::graph(Graph::new(graph_type)))
    }

    /// Evaluates a named graph declaration: graph Name { configure {...}, properties, methods }
    /// This creates a graph with intrinsic type_name and binds it to the identifier.
    fn eval_graph_decl(
        &mut self,
        name: &str,
        graph_type: &Option<String>,
        parent_expr: &Option<Box<Expr>>,
        properties: &[GraphProperty],
        methods: &[GraphMethod],
        rules: &[GraphRule],
        config: &HashMap<String, Vec<String>>,
        _position: &SourcePosition,
    ) -> Result<Option<Value>> {
        use crate::values::{Graph, GraphType};
        use crate::graph::RuleInstance;

        // Determine base graph type
        let base_graph_type = match graph_type.as_deref() {
            Some("dag") | Some("tree") => GraphType::Directed,
            Some("undirected") => GraphType::Undirected,
            _ => GraphType::Directed, // Default
        };

        // Create the graph, possibly inheriting from parent
        let mut graph = if let Some(parent_box) = parent_expr {
            let parent_value = self.eval_expr(parent_box)?;
            if let ValueKind::Graph(parent_graph) = parent_value.kind {
                Graph::from_parent(parent_graph)
            } else {
                return Err(GraphoidError::runtime(format!(
                    "Cannot inherit from non-graph type '{}'. Expected graph.",
                    parent_value.type_name()
                )));
            }
        } else {
            Graph::new(base_graph_type.clone())
        };

        // Set the intrinsic type name
        graph.type_name = Some(name.to_string());

        // Phase 3: Update __self__ node to use actual type name for inheritance edges
        graph.finalize_inheritance_node(name);

        // Apply graph type ruleset if specified
        if let Some(gtype) = graph_type {
            match gtype.as_str() {
                "dag" => {
                    graph.rulesets.push("dag".to_string());
                }
                "tree" => {
                    graph.rulesets.push("tree".to_string());
                }
                _ => {} // Unknown types are ignored for now
            }
        }

        // Add properties under __properties__/ branch
        // This follows the same pattern as methods stored under __methods__/ branch
        // Properties are stored at __properties__/name, keeping them separate from user data
        for prop in properties {
            let value = self.eval_expr(&prop.value)?;
            let property_node_id = Graph::property_node_id(&prop.name);
            graph.add_node(property_node_id, value)?;
        }

        // Process rule declarations (rule :name or rule :name, param)
        for rule in rules {
            // Get optional parameter value
            let param = if let Some(param_expr) = &rule.param {
                let param_value = self.eval_expr(param_expr)?;
                match &param_value.kind {
                    ValueKind::Number(n) => Some(*n),
                    _ => {
                        return Err(GraphoidError::runtime(format!(
                            "Rule parameter must be a number, got {}",
                            param_value.type_name()
                        )));
                    }
                }
            } else {
                None
            };

            // Convert rule name to RuleSpec and add to graph
            let rule_spec = Self::symbol_to_rule_spec(&rule.name, param)?;
            graph.add_rule(RuleInstance::new(rule_spec))?;
        }

        // Process configure block options
        // readable: [:x, :y] - generates getter methods
        // writable: [:x] - generates setter methods (set_x)
        // accessible: [:x] - generates both getter and setter
        let mut readable_props: Vec<String> = Vec::new();
        let mut writable_props: Vec<String> = Vec::new();

        if let Some(symbols) = config.get("readable") {
            readable_props.extend(symbols.clone());
        }
        if let Some(symbols) = config.get("writable") {
            writable_props.extend(symbols.clone());
        }
        if let Some(symbols) = config.get("accessible") {
            // Accessible = both readable and writable
            readable_props.extend(symbols.clone());
            writable_props.extend(symbols.clone());
        }

        // Generate getter methods for readable properties
        for prop_name in &readable_props {
            // Don't generate if a method with this name already exists
            if !graph.has_method(prop_name) {
                let getter = self.generate_getter_method(prop_name);
                graph.attach_method(prop_name.clone(), getter);
            }
        }

        // Generate setter methods for writable properties
        for prop_name in &writable_props {
            let setter_name = format!("set_{}", prop_name);
            // Don't generate if a setter already exists
            if !graph.has_method(&setter_name) {
                let setter = self.generate_setter_method(prop_name);
                graph.attach_method(setter_name, setter);
            }
        }

        // Get property names for semantic edge analysis
        let property_names: Vec<String> = properties.iter().map(|p| p.name.clone()).collect();

        // IMPORTANT: Temporarily bind the graph to its name BEFORE creating methods.
        // This allows methods to capture an environment that includes the class name,
        // enabling patterns like `instance = ClassName {}` inside instance methods.
        // The binding will be updated with the final graph (including methods) at the end.
        self.env.define(name.to_string(), Value::graph(graph.clone()));

        // Add explicitly defined methods
        for method in methods {
            // For private methods declared with 'priv' keyword,
            // prefix the name with underscore (Graphoid convention)
            let method_name = if method.is_private && !method.name.starts_with('_') {
                format!("_{}", method.name)
            } else {
                method.name.clone()
            };

            let func = Function {
                name: Some(method_name.clone()),
                params: method.params.iter().map(|p| p.name.clone()).collect(),
                parameters: method.params.clone(),
                body: method.body.clone(),
                pattern_clauses: None,
                env: Rc::new(RefCell::new(self.env.clone())),
                node_id: None,
                is_setter: method.is_setter,
                is_static: method.is_static,
                guard: method.guard.clone(),
            };

            // Analyze method body for property references (semantic edges)
            let prop_refs = extract_property_references(&method.body, &property_names);

            // Store method in graph using the existing method storage mechanism
            graph.attach_method(method_name.clone(), func.clone());

            // Add semantic edges from method to properties it reads/writes
            graph.add_method_property_edges(&method_name, &prop_refs.reads, &prop_refs.writes);
        }

        // Create the value and bind it
        let value = Value::graph(graph);
        self.env.define(name.to_string(), value);

        Ok(None)
    }

    /// Generate a getter method for a property: returns the property value
    fn generate_getter_method(&self, prop_name: &str) -> Function {
        // Create a function that returns self.prop_name
        // Body: return self.prop_name
        let body = vec![
            Stmt::Return {
                value: Some(Expr::PropertyAccess {
                    object: Box::new(Expr::Variable {
                        name: "self".to_string(),
                        position: SourcePosition::unknown(),
                    }),
                    property: prop_name.to_string(),
                    position: SourcePosition::unknown(),
                }),
                position: SourcePosition::unknown(),
            }
        ];

        Function {
            name: Some(prop_name.to_string()),
            params: vec![],
            parameters: vec![],
            body,
            pattern_clauses: None,
            env: Rc::new(RefCell::new(self.env.clone())),
            node_id: None,
            is_setter: false,
            is_static: false,
            guard: None,
        }
    }

    /// Generate a setter method for a property: sets self.prop_name = value
    fn generate_setter_method(&self, prop_name: &str) -> Function {
        // Create a function that sets self.prop_name = value
        // Body: self.prop_name = value
        let body = vec![
            Stmt::Assignment {
                target: AssignmentTarget::Property {
                    object: Box::new(Expr::Variable {
                        name: "self".to_string(),
                        position: SourcePosition::unknown(),
                    }),
                    property: prop_name.to_string(),
                },
                value: Expr::Variable {
                    name: "value".to_string(),
                    position: SourcePosition::unknown(),
                },
                position: SourcePosition::unknown(),
            }
        ];

        Function {
            name: Some(format!("set_{}", prop_name)),
            params: vec!["value".to_string()],
            parameters: vec![Parameter {
                name: "value".to_string(),
                default_value: None,
                is_variadic: false,
            }],
            body,
            pattern_clauses: None,
            env: Rc::new(RefCell::new(self.env.clone())),
            node_id: None,
            is_setter: true,
            is_static: false,
            guard: None,
        }
    }

    /// Evaluates a conditional expression (inline if-then-else or suffix if/unless).
    fn eval_conditional(
        &mut self,
        condition: &Expr,
        then_expr: &Expr,
        else_expr: &Option<Box<Expr>>,
        is_unless: bool,
    ) -> Result<Value> {
        // Evaluate the condition
        let condition_value = self.eval_expr(condition)?;

        // Check if condition is truthy
        let is_truthy = match &condition_value.kind {
            ValueKind::Boolean(b) => *b,
            ValueKind::None => false,
            ValueKind::Number(n) => *n != 0.0,
            ValueKind::String(ref s) => !s.is_empty(),
            ValueKind::List(ref l) => l.len() > 0,
            ValueKind::Map(ref h) => h.len() > 0,
            ValueKind::Graph(ref g) => g.node_count() > 0,
            _ => true, // Everything else is truthy
        };

        // For unless, invert the condition
        let should_execute = if is_unless { !is_truthy } else { is_truthy };

        if should_execute {
            // Execute then branch
            self.eval_expr(then_expr)
        } else {
            // Execute else branch (or return none if suffix form)
            match else_expr {
                Some(else_e) => self.eval_expr(else_e),
                None => Ok(Value::none()),
            }
        }
    }

    /// Evaluates a lambda expression.
    /// Creates an anonymous function that captures the current environment.
    fn eval_lambda(&self, params: &[String], body: &Expr) -> Result<Value> {
        // Convert body to function body statements
        let body_stmts = match body {
            // Block body: use statements directly
            Expr::Block { statements, .. } => statements.clone(),
            // Expression body: wrap in return statement
            _ => vec![Stmt::Return {
                value: Some((*body).clone()),
                position: body.position().clone(),
            }],
        };

        // Create anonymous function with captured environment
        // Convert param names to Parameter objects (lambdas don't have defaults or variadic)
        let parameters: Vec<Parameter> = params.iter().map(|name| Parameter {
            name: name.clone(),
            default_value: None,
            is_variadic: false,  // Lambdas don't support variadic parameters
        }).collect();

        let mut func = Function {
            name: None, // Anonymous
            params: params.to_vec(),
            parameters,
            body: body_stmts,
            pattern_clauses: None,
            env: Rc::new(RefCell::new(self.env.clone())),
            node_id: None,
            is_setter: false,  // Lambdas are never setters
            is_static: false,  // Lambdas are never static
            guard: None,       // Lambdas don't have guards
        };

        // Register lambda in the function graph and store its node_id
        let node_id = self.function_graph.borrow_mut().register_function(func.clone());
        func.node_id = Some(node_id);

        Ok(Value::function(func))
    }

    /// Evaluates a block expression (used in lambda bodies).
    /// Returns the value of the last expression, or none if the block is empty or only has statements.
    fn eval_block(&mut self, statements: &[Stmt]) -> Result<Value> {
        if statements.is_empty() {
            return Ok(Value::none());
        }

        // Execute all statements except the last
        for stmt in &statements[..statements.len() - 1] {
            // Execute the statement and check for returns
            if let Some(return_value) = self.eval_stmt(stmt)? {
                return Ok(return_value);
            }
        }

        // Handle the last statement specially - it might be an implicit return
        let last_stmt = &statements[statements.len() - 1];

        // If it's an expression statement, return its value
        if let Stmt::Expression { expr, .. } = last_stmt {
            return self.eval_expr(expr);
        }

        // Otherwise, execute it normally and check for explicit return
        if let Some(return_value) = self.eval_stmt(last_stmt)? {
            return Ok(return_value);
        }

        // No explicit return and last statement is not an expression, return none
        Ok(Value::none())
    }

    /// Evaluates an index expression (list[i] or map[key]).
    fn eval_index(&mut self, object: &Expr, index: &Expr) -> Result<Value> {
        // Evaluate the object being indexed
        let object_value = self.eval_expr(object)?;

        // Evaluate the index expression
        let index_value = self.eval_expr(index)?;

        match &object_value.kind {
            ValueKind::List(ref list) => {
                // Index must be a number for lists
                let idx = match &index_value.kind {
                    ValueKind::Number(n) => n,
                    _other => {
                        return Err(GraphoidError::type_error(
                            "number",
                            index_value.type_name(),
                        ));
                    }
                };

                // Handle fractional indices by truncating to integer
                let idx_int = *idx as i64;

                // Calculate actual index (handle negative indices)
                let actual_index = if idx_int < 0 {
                    // Negative index: count from end
                    let len = list.len() as i64;
                    len + idx_int
                } else {
                    idx_int
                };

                // Check bounds
                if actual_index < 0 || actual_index >= list.len() as i64 {
                    // Check error mode
                    match self.config_stack.current().error_mode {
                        ErrorMode::Lenient => {
                            return Ok(Value::none());
                        }
                        ErrorMode::Collect => {
                            let error = GraphoidError::runtime(format!(
                                "List index out of bounds: index {} for list of length {}",
                                idx_int,
                                list.len()
                            ));
                            self.error_collector.collect(
                                error,
                                self.current_file.as_ref().map(|p| p.to_string_lossy().to_string()),
                                SourcePosition::unknown(),
                            );
                            return Ok(Value::none());
                        }
                        ErrorMode::Strict => {
                            return Err(GraphoidError::runtime(format!(
                                "List index out of bounds: index {} for list of length {}",
                                idx_int,
                                list.len()
                            )));
                        }
                    }
                }

                Ok(list.get(actual_index as usize).unwrap().clone())
            }
            ValueKind::Map(ref hash) => {
                // Index must be a string for maps
                let key = match &index_value.kind {
                    ValueKind::String(s) => s,
                    _other => {
                        return Err(GraphoidError::type_error(
                            "string",
                            index_value.type_name(),
                        ));
                    }
                };

                // Look up the key
                match hash.get(&key) {
                    Some(value) => Ok(value.clone()),
                    None => {
                        // Check error mode
                        match self.config_stack.current().error_mode {
                            ErrorMode::Lenient => {
                                return Ok(Value::none());
                            }
                            ErrorMode::Collect => {
                                let error = GraphoidError::runtime(format!(
                                    "Map key not found: '{}'",
                                    key
                                ));
                                self.error_collector.collect(
                                    error,
                                    self.current_file.as_ref().map(|p| p.to_string_lossy().to_string()),
                                    SourcePosition::unknown(),
                                );
                                return Ok(Value::none());
                            }
                            ErrorMode::Strict => {
                                return Err(GraphoidError::runtime(format!(
                                    "Map key not found: '{}'",
                                    key
                                )));
                            }
                        }
                    }
                }
            }
            ValueKind::String(ref s) => {
                // Index must be a number for strings
                let idx = match &index_value.kind {
                    ValueKind::Number(n) => n,
                    _other => {
                        return Err(GraphoidError::type_error(
                            "number",
                            index_value.type_name(),
                        ));
                    }
                };

                // Handle fractional indices by truncating to integer
                let idx_int = *idx as i64;

                // Get string as chars
                let chars: Vec<char> = s.chars().collect();

                // Calculate actual index (handle negative indices)
                let actual_index = if idx_int < 0 {
                    // Negative index: count from end
                    let len = chars.len() as i64;
                    len + idx_int
                } else {
                    idx_int
                };

                // Check bounds
                if actual_index < 0 || actual_index >= chars.len() as i64 {
                    return Err(GraphoidError::runtime(format!(
                        "String index out of bounds: index {} for string of length {}",
                        idx_int,
                        chars.len()
                    )));
                }

                // Return character as a string
                Ok(Value::string(chars[actual_index as usize].to_string()))
            }
            ValueKind::Graph(ref graph) => {
                // Index must be a string for graphs (node ID)
                let node_id = match &index_value.kind {
                    ValueKind::String(s) => s,
                    _other => {
                        return Err(GraphoidError::type_error(
                            "string",
                            index_value.type_name(),
                        ));
                    }
                };

                // Don't allow access to internal nodes via index syntax
                if node_id.starts_with("__") {
                    return Ok(Value::none());
                }

                // Look up the node
                match graph.get_node(&node_id) {
                    Some(value) => Ok(value.clone()),
                    None => Err(GraphoidError::runtime(format!(
                        "Graph node not found: '{}'",
                        node_id
                    ))),
                }
            }
            _other => {
                Err(GraphoidError::runtime(format!(
                    "Cannot index value of type '{}'",
                    object_value.type_name()
                )))
            }
        }
    }

    /// Evaluates a property access expression (object.property without parentheses).
    /// For graphs: Returns data node value.
    /// For hashes: Returns the value for the key.
    /// Returns none if property doesn't exist.
    fn eval_property_access(&mut self, object: &Expr, property: &str) -> Result<Value> {
        let object_value = self.eval_expr(object)?;

        match &object_value.kind {
            ValueKind::Graph(ref graph) => {
                // Don't allow access to internal nodes via property syntax
                if property.starts_with("__") {
                    return Ok(Value::none());
                }

                // First, check for CLG property in __properties__/ branch
                let property_node_id = Graph::property_node_id(&property);
                if let Some(value) = graph.get_node(&property_node_id) {
                    return Ok(value.clone());
                }

                // Then, try to get a user data node with this name
                if let Some(value) = graph.get_node(property) {
                    return Ok(value.clone());
                }

                // Nothing found, return none
                Ok(Value::none())
            }
            ValueKind::Map(ref hash) => {
                // For hashes, property access is equivalent to index access
                match hash.get(&property.to_string()) {
                    Some(value) => Ok(value.clone()),
                    None => Ok(Value::none()),
                }
            }
            _other => {
                // For other types, try calling a method with this name (no args)
                // This maintains backward compatibility with code like `list.length`
                self.eval_method_call(object, property, &[])
            }
        }
    }

    /// Evaluates a super method call (super.method(args)).
    /// This calls the parent graph's implementation of the method.
    fn eval_super_method_call(
        &mut self,
        method: &str,
        args: &[crate::ast::Argument],
        _position: &crate::error::SourcePosition,
    ) -> Result<Value> {
        // Get `self` from the current environment
        let self_value = self.env.get("self").map_err(|_| {
            GraphoidError::runtime("'super' can only be used within a method".to_string())
        })?;

        // Extract the graph from self
        let self_graph = match &self_value.kind {
            ValueKind::Graph(g) => g.clone(),
            _ => {
                return Err(GraphoidError::runtime(
                    "'super' can only be used within a graph method".to_string(),
                ));
            }
        };

        // Get the context graph from super_context_stack (which graph's parent we should use)
        // This enables multi-level super calls: if we're in a super method, use that method's
        // defining graph's parent, not self's parent.
        let context_graph = self.super_context_stack.last().cloned().ok_or_else(|| {
            GraphoidError::runtime("'super' can only be used within a method".to_string())
        })?;

        // Get the parent graph from the context graph
        let parent_graph = match &context_graph.parent {
            Some(parent) => (**parent).clone(),
            None => {
                return Err(GraphoidError::runtime(format!(
                    "Cannot call super.{}(): graph has no parent",
                    method
                )));
            }
        };

        // Find the method on the parent
        let parent_method = parent_graph.get_method(method).ok_or_else(|| {
            GraphoidError::runtime(format!(
                "Method '{}' not found on parent graph",
                method
            ))
        })?;

        // Evaluate argument expressions
        let arg_values = self.eval_arguments(args)?;

        // Push the parent graph onto super_context_stack before calling
        // This ensures that if the parent method also calls super, it will
        // use the parent's parent, not loop back to itself
        self.super_context_stack.push(parent_graph.clone());

        // Call the parent's method implementation with `self` as the receiver
        // This is important: we use `self_graph` (the child), not `parent_graph`
        // The method runs on the child but uses the parent's implementation
        // We need a dummy expression for the object_expr parameter
        let dummy_expr = Expr::Variable {
            name: "self".to_string(),
            position: crate::error::SourcePosition { line: 0, column: 0, file: None },
        };
        // Use call_graph_method_impl with manage_super_context=false since we manage the stack ourselves
        let result = self.call_graph_method_impl(&self_graph, &parent_method, &arg_values, &dummy_expr, false);

        // Pop the parent from super_context_stack
        self.super_context_stack.pop();

        result
    }

    /// Helper to evaluate arguments (positional only for now).
    /// Named arguments in method calls are not yet supported.
    pub(crate) fn eval_arguments(&mut self, args: &[crate::ast::Argument]) -> Result<Vec<Value>> {
        use crate::ast::Argument;
        let mut arg_values = Vec::new();
        for arg in args {
            match arg {
                Argument::Positional { expr, .. } => {
                    arg_values.push(self.eval_expr(expr)?);
                }
                Argument::Named { name, .. } => {
                    return Err(GraphoidError::runtime(format!(
                        "Named arguments are not supported in method calls (parameter '{}')",
                        name
                    )));
                }
            }
        }
        Ok(arg_values)
    }

    /// Evaluates a method call expression (object.method(args)).
    fn eval_method_call(&mut self, object: &Expr, method: &str, args: &[crate::ast::Argument]) -> Result<Value> {
        // Check for static method calls on type identifiers (e.g., list.generate, time.now)
        // BUT: Check if the name is defined as a variable/module first!
        if let Expr::Variable { name, .. } = object {
            if name == "list" {
                // Evaluate argument expressions
                let arg_values = self.eval_arguments(args)?;
                return self.eval_list_static_method(method, &arg_values);
            }
            if name == "string" {
                // Evaluate argument expressions
                let arg_values = self.eval_arguments(args)?;
                return self.eval_string_static_method(method, &arg_values);
            }
            if name == "time" {
                // Check if 'time' is defined as a variable (e.g., imported module)
                if !self.env.exists("time") {
                    // Not defined - use built-in static methods
                    let arg_values = self.eval_arguments(args)?;
                    return self.eval_time_static_method(method, &arg_values);
                }
                // Otherwise fall through to normal module handling
            }
        }

        // Evaluate the object once
        let object_value = self.eval_expr(object)?;

        // Check for module member access (e.g., module.function(args) or module.variable)
        if let ValueKind::Module(ref module) = &object_value.kind {
            // Phase 10: Check if member is private
            if module.private_symbols.contains(method) {
                return Err(GraphoidError::runtime(format!(
                    "Cannot access private symbol '{}' from module '{}'",
                    method, module.name
                )));
            }

            // Look up the member in the module's namespace FIRST (module-qualified calls
            // should always resolve to the module's definition, not global_functions)
            let member = module.namespace.get(method)?;

            // If it's a function, call it with args
            if let ValueKind::Function(func) = &member.kind {
                // Evaluate argument expressions if not already done
                let arg_values = if args.is_empty() {
                    Vec::new()
                } else {
                    self.eval_arguments(args)?
                };
                return self.call_function(&func, &arg_values);
            } else if let ValueKind::NativeFunction(native_func) = &member.kind {
                // Native function - call it with evaluated args
                let arg_values = self.eval_arguments(args)?;
                return native_func(&arg_values);
            } else {
                // If it's a variable and no args, return it directly
                if args.is_empty() {
                    return Ok(member);
                } else {
                    // Can't call non-functions with arguments
                    return Err(GraphoidError::runtime(format!(
                        "Module member '{}' is not a function, cannot be called with arguments",
                        method
                    )));
                }
            }
        }

        // Check if this is a mutating method (ends with !)
        let is_mutating = method.ends_with('!');
        let base_method = if is_mutating {
            &method[..method.len() - 1]
        } else {
            method
        };

        // Special handling for 'where' method on lists - needs unevaluated expression
        // PatternMatchResults.where() uses evaluated arguments, so skip this for that type
        if base_method == "where" && !args.is_empty() && matches!(object_value.kind, ValueKind::List(_)) {
            return self.eval_where_method(object_value, args);
        }

        // Special handling for 'return' method on lists - needs unevaluated expressions
        // PatternMatchResults.return_vars() / return_properties() use evaluated arguments
        if base_method == "return" && !args.is_empty() && matches!(object_value.kind, ValueKind::List(_)) {
            return self.eval_return_method(object_value, args);
        }

        // Evaluate all argument expressions
        let arg_values = self.eval_arguments(args)?;

        if is_mutating {
            // Extract variable name from object expression
            let var_name = match object {
                Expr::Variable { name, .. } => name.clone(),
                _ => {
                    return Err(GraphoidError::runtime(format!(
                        "Mutating method '{}' requires a variable, not an expression",
                        method
                    )))
                }
            };

            // Special case for pop: it returns the popped value, not the mutated list
            if base_method == "pop" {
                // Clone the list for mutation
                if let ValueKind::List(list) = &object_value.kind {
                    let mut list_to_mutate = list.clone();
                    let popped_value = list_to_mutate.pop()?; // Get popped value and mutate
                    self.env.set(&var_name, Value::list(list_to_mutate))?;
                    return Ok(popped_value); // Return the popped value
                }
            }

            // For other mutating methods, apply method and update variable
            let result = self.apply_method_to_value(object_value, base_method, &arg_values, object)?;
            self.env.set(&var_name, result)?;

            // Mutating methods return none
            Ok(Value::none())
        } else {
            // Immutable method - use the already-evaluated value
            self.apply_method_to_value(object_value, base_method, &arg_values, object)
        }
    }

    /// Evaluates the 'where' method for filtering lists with pattern variable bindings.
    /// The where method receives unevaluated expressions so they can be evaluated with
    /// temporary variable bindings from each list element (typically pattern match results).
    fn eval_where_method(&mut self, list_value: Value, args: &[crate::ast::Argument]) -> Result<Value> {
        // where() must be called on a list
        let list = match &list_value.kind {
            ValueKind::List(l) => l,
            _ => {
                return Err(GraphoidError::runtime(format!(
                    "where() can only be called on lists, got {}",
                    list_value.type_name()
                )));
            }
        };

        // where() takes exactly one argument (the predicate expression)
        if args.len() != 1 {
            return Err(GraphoidError::runtime(format!(
                "where() expects 1 argument, but got {}",
                args.len()
            )));
        }

        // Get the predicate expression (NOT evaluated yet)
        // Extract expression from Argument enum
        let predicate_expr = match &args[0] {
            crate::ast::Argument::Positional { expr, .. } => expr,
            crate::ast::Argument::Named { value, .. } => value,
        };

        // Filter the list
        let mut filtered = Vec::new();
        let elements = list.to_vec();

        for element in elements {
            // If element is a map (hash), bind its keys as temporary variables
            if let ValueKind::Map(hash) = &element.kind {
                // Save current environment state
                let keys = hash.keys();
                let saved_vars: Vec<(String, Option<Value>)> = keys
                    .iter()
                    .map(|key| {
                        let saved = self.env.get(key).ok();
                        (key.clone(), saved)
                    })
                    .collect();

                // Bind hash keys as temporary variables
                for key in &keys {
                    if let Some(value) = hash.get(key) {
                        self.env.define(key.clone(), value.clone());
                    }
                }

                // Evaluate the predicate with these bindings
                let result = self.eval_expr(predicate_expr)?;

                // Restore previous environment
                for (key, saved_value) in saved_vars {
                    if let Some(val) = saved_value {
                        self.env.define(key, val);
                    } else {
                        // Variable didn't exist before, remove it
                        self.env.remove_variable(&key);
                    }
                }

                // Keep element if predicate is truthy
                if result.is_truthy() {
                    filtered.push(element);
                }
            } else {
                // For non-hash elements, evaluate predicate as-is
                let result = self.eval_expr(predicate_expr)?;
                if result.is_truthy() {
                    filtered.push(element);
                }
            }
        }

        Ok(Value::list(crate::values::List::from_vec(filtered)))
    }

    /// Evaluates the 'return' method for projecting specific fields from pattern matches.
    /// The return method receives unevaluated expressions so they can be evaluated with
    /// temporary variable bindings from each list element (typically pattern match results).
    fn eval_return_method(&mut self, list_value: Value, args: &[crate::ast::Argument]) -> Result<Value> {
        // return() must be called on a list
        let list = match &list_value.kind {
            ValueKind::List(l) => l,
            _ => {
                return Err(GraphoidError::runtime(format!(
                    "return() can only be called on lists, got {}",
                    list_value.type_name()
                )));
            }
        };

        // return() takes at least one argument (the fields to project)
        if args.is_empty() {
            return Err(GraphoidError::runtime(
                "return() expects at least 1 argument".to_string()
            ));
        }

        // Extract return expressions (NOT evaluated yet)
        let return_exprs: Vec<&Expr> = args
            .iter()
            .map(|arg| match arg {
                crate::ast::Argument::Positional { expr, .. } => expr,
                crate::ast::Argument::Named { value, .. } => value,
            })
            .collect();

        // Project the list
        let mut projected = Vec::new();
        let elements = list.to_vec();

        for element in elements {
            // If element is a map (hash), bind its keys as temporary variables
            if let ValueKind::Map(hash) = &element.kind {
                // Save current environment state
                let keys = hash.keys();
                let saved_vars: Vec<(String, Option<Value>)> = keys
                    .iter()
                    .map(|key| {
                        let saved = self.env.get(key).ok();
                        (key.clone(), saved)
                    })
                    .collect();

                // Bind hash keys as temporary variables
                for key in &keys {
                    if let Some(value) = hash.get(key) {
                        self.env.define(key.clone(), value.clone());
                    }
                }

                // Build new hash with only selected fields
                let mut result_hash = crate::values::Hash::new();

                for return_expr in &return_exprs {
                    // Evaluate the expression with bindings
                    let field_value = self.eval_expr(return_expr)?;

                    // Generate intelligent key name from the expression
                    // For property access like "person.name", use "person.name" as key
                    let key = self.expr_to_field_name(return_expr);

                    let _ = result_hash.insert(key, field_value);
                }

                // Restore previous environment
                for (key, saved_value) in saved_vars {
                    if let Some(val) = saved_value {
                        self.env.define(key, val);
                    } else {
                        // Variable didn't exist before, remove it
                        self.env.remove_variable(&key);
                    }
                }

                projected.push(Value::map(result_hash));
            } else {
                // For non-hash elements, can't do field projection
                return Err(GraphoidError::runtime(format!(
                    "return() requires elements to be maps for field projection, got {}",
                    element.type_name()
                )));
            }
        }

        Ok(Value::list(crate::values::List::from_vec(projected)))
    }

    /// Converts an expression to a field name for return clauses.
    /// Examples: person.name -> "person.name", friend.age -> "friend.age"
    fn expr_to_field_name(&self, expr: &Expr) -> String {
        match expr {
            // Property access: object.property
            Expr::MethodCall { object, method, args, .. } if args.is_empty() => {
                // Recursively build the name
                let object_name = self.expr_to_field_name(object);
                format!("{}.{}", object_name, method)
            }
            // Variable reference
            Expr::Variable { name, .. } => name.clone(),
            // For other expressions, generate a generic name
            _ => "field".to_string(),
        }
    }

    /// Applies a method to a value (helper to avoid duplication).
    fn apply_method_to_value(&mut self, value: Value, method: &str, args: &[Value], object_expr: &Expr) -> Result<Value> {
        // Handle generic freeze-related methods that work on all value types
        match method {
            "freeze" => {
                // Returns frozen copy
                let mut frozen_copy = value.clone();
                frozen_copy.freeze();
                return Ok(frozen_copy);
            }
            "is_frozen" => {
                // Returns boolean indicating if value is frozen
                return Ok(Value::boolean(value.is_frozen()));
            }
            "has_frozen" => {
                // Check for :count symbol argument (for detailed stats)
                let wants_count = args.get(0).map_or(false, |arg| {
                    matches!(&arg.kind, ValueKind::Symbol(s) if s == "count")
                });

                // Check for :deep symbol argument (for recursive counting)
                let deep = args.get(1).map_or(false, |arg| {
                    matches!(&arg.kind, ValueKind::Symbol(s) if s == "deep")
                });

                if wants_count {
                    // Return detailed hash with counts
                    return self.eval_has_frozen_count(&value, deep);
                } else {
                    // Return boolean - check if any elements are frozen (always recursive)
                    return Ok(Value::boolean(self.check_has_frozen(&value)));
                }
            }
            // Universal casting methods (spec line 3185) - work on all types
            "to_num" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'to_num' takes no arguments, but got {}",
                        args.len()
                    )));
                }
                return self.value_to_num(&value);
            }
            // Phase 2: BigNum casting methods
            "to_bignum" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'to_bignum' takes no arguments, but got {}",
                        args.len()
                    )));
                }
                return self.value_to_bignum(&value);
            }
            "fits_in_num" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'fits_in_num' takes no arguments, but got {}",
                        args.len()
                    )));
                }
                return self.value_fits_in_num(&value);
            }
            "is_bignum" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'is_bignum' takes no arguments, but got {}",
                        args.len()
                    )));
                }
                return Ok(Value::boolean(matches!(&value.kind, ValueKind::BigNumber(_))));
            }
            "to_string" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'to_string' takes no arguments, but got {}",
                        args.len()
                    )));
                }
                return self.value_to_string(&value);
            }
            "to_bool" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'to_bool' takes no arguments, but got {}",
                        args.len()
                    )));
                }
                return self.value_to_bool(&value);
            }
            "type" => {
                // Note: Error and PatternNode have their own type() methods that return
                // different information, so we let them handle it in their specific dispatchers
                if !matches!(&value.kind, ValueKind::Error(_) | ValueKind::PatternNode(_)) {
                    if !args.is_empty() {
                        return Err(GraphoidError::runtime(format!(
                            "Method 'type' takes no arguments, but got {}",
                            args.len()
                        )));
                    }
                    return Ok(Value::string(value.type_name().to_string()));
                }
                // Let Error and PatternNode handle their own type() method
            }
            "type_name" => {
                // Alias for type() method - mirrors Rust API naming
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'type_name' takes no arguments, but got {}",
                        args.len()
                    )));
                }
                return Ok(Value::string(value.type_name().to_string()));
            }
            _ => {}
        }

        // Dispatch to type-specific methods
        match &value.kind {
            ValueKind::Number(n) => self.eval_number_method(*n, method, args),
            ValueKind::BigNumber(bn) => self.eval_bignum_method(bn, method, args),
            ValueKind::Time(timestamp) => self.eval_time_method(*timestamp, method, args),
            ValueKind::List(list) => self.eval_list_method(&list, method, args),
            ValueKind::Map(hash) => self.eval_map_method(&hash, method, args),
            ValueKind::Graph(graph) => self.eval_graph_method(graph.clone(), method, args, object_expr),
            ValueKind::String(ref s) => self.eval_string_method(s, method, args, object_expr),
            ValueKind::Error(ref err) => self.eval_error_method(err, method, args),
            ValueKind::PatternNode(pn) => self.eval_pattern_node_method(pn, method, args),
            ValueKind::PatternEdge(pe) => self.eval_pattern_edge_method(pe, method, args),
            ValueKind::PatternPath(pp) => self.eval_pattern_path_method(pp, method, args),
            ValueKind::PatternMatchResults(results) => self.eval_pattern_match_results_method(results, method, args),
            _other => Err(GraphoidError::runtime(format!(
                "Type '{}' does not have method '{}'",
                value.type_name(),
                method
            ))),
        }
    }

    /// Evaluates static methods on the list type (e.g., list.generate, list.upto).
    fn eval_time_static_method(&self, method: &str, args: &[Value]) -> Result<Value> {
        use chrono::{Utc, TimeZone, Datelike};

        match method {
            "now" => {
                // time.now() - current UTC timestamp
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "time.now() expects 0 arguments, but got {}",
                        args.len()
                    )));
                }
                let now = Utc::now();
                let timestamp = now.timestamp() as f64 + (now.timestamp_subsec_nanos() as f64 / 1_000_000_000.0);
                Ok(Value::time(timestamp))
            }
            "today" => {
                // time.today() - midnight UTC today
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "time.today() expects 0 arguments, but got {}",
                        args.len()
                    )));
                }
                let now = Utc::now();
                let today = Utc.with_ymd_and_hms(now.year(), now.month(), now.day(), 0, 0, 0)
                    .single()
                    .ok_or_else(|| GraphoidError::runtime("Failed to create today's date".to_string()))?;
                let timestamp = today.timestamp() as f64;
                Ok(Value::time(timestamp))
            }
            "from_numbers" => {
                // time.from_numbers(year, month, day, hour, min, sec)
                if args.len() != 6 {
                    return Err(GraphoidError::runtime(format!(
                        "time.from_numbers() expects 6 arguments (year, month, day, hour, min, sec), but got {}",
                        args.len()
                    )));
                }

                let year = match &args[0].kind {
                    ValueKind::Number(n) => *n as i32,
                    _ => return Err(GraphoidError::type_error("number", args[0].type_name()))
                };
                let month = match &args[1].kind {
                    ValueKind::Number(n) => *n as u32,
                    _ => return Err(GraphoidError::type_error("number", args[1].type_name()))
                };
                let day = match &args[2].kind {
                    ValueKind::Number(n) => *n as u32,
                    _ => return Err(GraphoidError::type_error("number", args[2].type_name()))
                };
                let hour = match &args[3].kind {
                    ValueKind::Number(n) => *n as u32,
                    _ => return Err(GraphoidError::type_error("number", args[3].type_name()))
                };
                let min = match &args[4].kind {
                    ValueKind::Number(n) => *n as u32,
                    _ => return Err(GraphoidError::type_error("number", args[4].type_name()))
                };
                let sec = match &args[5].kind {
                    ValueKind::Number(n) => *n as u32,
                    _ => return Err(GraphoidError::type_error("number", args[5].type_name()))
                };

                let dt = Utc.with_ymd_and_hms(year, month, day, hour, min, sec)
                    .single()
                    .ok_or_else(|| GraphoidError::runtime(format!(
                        "Invalid date/time: {}-{:02}-{:02} {:02}:{:02}:{:02}",
                        year, month, day, hour, min, sec
                    )))?;

                let timestamp = dt.timestamp() as f64;
                Ok(Value::time(timestamp))
            }
            "from_string" => {
                // time.from_string(iso_string)
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "time.from_string() expects 1 argument (ISO 8601 string), but got {}",
                        args.len()
                    )));
                }

                let iso_string = match &args[0].kind {
                    ValueKind::String(s) => s,
                    _ => return Err(GraphoidError::type_error("string", args[0].type_name()))
                };

                // Try to parse as RFC3339/ISO 8601
                let dt = chrono::DateTime::parse_from_rfc3339(iso_string)
                    .map_err(|e| GraphoidError::runtime(format!(
                        "Failed to parse time string '{}': {}",
                        iso_string, e
                    )))?;

                let timestamp = dt.timestamp() as f64 + (dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0);
                Ok(Value::time(timestamp))
            }
            "from_timestamp" => {
                // time.from_timestamp(number)
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "time.from_timestamp() expects 1 argument (Unix timestamp), but got {}",
                        args.len()
                    )));
                }

                let timestamp = match &args[0].kind {
                    ValueKind::Number(n) => *n,
                    _ => return Err(GraphoidError::type_error("number", args[0].type_name()))
                };

                Ok(Value::time(timestamp))
            }
            _ => Err(GraphoidError::runtime(format!(
                "time does not have static method '{}'",
                method
            ))),
        }
    }

    /// Evaluates a method call on a list.
    fn eval_number_method(&self, n: f64, method: &str, args: &[Value]) -> Result<Value> {
        match method {
            "sqrt" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Number method 'sqrt' takes no arguments, but got {}",
                        args.len()
                    )));
                }
                Ok(Value::number(n.sqrt()))
            }
            "abs" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Number method 'abs' takes no arguments, but got {}",
                        args.len()
                    )));
                }
                Ok(Value::number(n.abs()))
            }
            "up" => {
                // up() - ceil to integer
                // up(n) - ceil to n decimal places
                // up(:nearest_ten) - ceil to nearest 10
                // up(:nearest_hundred) - ceil to nearest 100
                if args.is_empty() {
                    return Ok(Value::number(n.ceil()));
                }

                match &args[0].kind {
                    ValueKind::Number(decimal_places) => {
                        if args.len() > 1 {
                            return Err(GraphoidError::runtime(format!(
                                "Number method 'up' expects 0 or 1 arguments, but got {}",
                                args.len()
                            )));
                        }
                        let places = *decimal_places as i32;
                        let multiplier = 10_f64.powi(places);
                        Ok(Value::number((n * multiplier).ceil() / multiplier))
                    }
                    ValueKind::Symbol(mode) => {
                        if args.len() > 1 {
                            return Err(GraphoidError::runtime(format!(
                                "Number method 'up' expects 0 or 1 arguments, but got {}",
                                args.len()
                            )));
                        }
                        match mode.as_str() {
                            "nearest_ten" => Ok(Value::number((n / 10.0).ceil() * 10.0)),
                            "nearest_hundred" => Ok(Value::number((n / 100.0).ceil() * 100.0)),
                            _ => Err(GraphoidError::runtime(format!(
                                "Unknown up() mode: :{}. Valid modes: :nearest_ten, :nearest_hundred",
                                mode
                            )))
                        }
                    }
                    _ => Err(GraphoidError::runtime(format!(
                        "up() argument must be a number (decimal places) or symbol (mode), got {}",
                        args[0].type_name()
                    )))
                }
            }
            "down" => {
                // down() - floor to integer
                // down(n) - floor to n decimal places
                // down(:nearest_ten) - floor to nearest 10
                // down(:nearest_hundred) - floor to nearest 100
                if args.is_empty() {
                    return Ok(Value::number(n.floor()));
                }

                match &args[0].kind {
                    ValueKind::Number(decimal_places) => {
                        if args.len() > 1 {
                            return Err(GraphoidError::runtime(format!(
                                "Number method 'down' expects 0 or 1 arguments, but got {}",
                                args.len()
                            )));
                        }
                        let places = *decimal_places as i32;
                        let multiplier = 10_f64.powi(places);
                        Ok(Value::number((n * multiplier).floor() / multiplier))
                    }
                    ValueKind::Symbol(mode) => {
                        if args.len() > 1 {
                            return Err(GraphoidError::runtime(format!(
                                "Number method 'down' expects 0 or 1 arguments, but got {}",
                                args.len()
                            )));
                        }
                        match mode.as_str() {
                            "nearest_ten" => Ok(Value::number((n / 10.0).floor() * 10.0)),
                            "nearest_hundred" => Ok(Value::number((n / 100.0).floor() * 100.0)),
                            _ => Err(GraphoidError::runtime(format!(
                                "Unknown down() mode: :{}. Valid modes: :nearest_ten, :nearest_hundred",
                                mode
                            )))
                        }
                    }
                    _ => Err(GraphoidError::runtime(format!(
                        "down() argument must be a number (decimal places) or symbol (mode), got {}",
                        args[0].type_name()
                    )))
                }
            }
            "round" => {
                // round() - round to integer
                // round(n) - round to n decimal places
                // round(:nearest_ten) - round to nearest 10
                // round(:nearest_hundred) - round to nearest 100
                if args.is_empty() {
                    return Ok(Value::number(n.round()));
                }

                match &args[0].kind {
                    ValueKind::Number(decimal_places) => {
                        if args.len() > 1 {
                            return Err(GraphoidError::runtime(format!(
                                "Number method 'round' expects 0 or 1 arguments, but got {}",
                                args.len()
                            )));
                        }
                        let places = *decimal_places as i32;
                        let multiplier = 10_f64.powi(places);
                        Ok(Value::number((n * multiplier).round() / multiplier))
                    }
                    ValueKind::Symbol(mode) => {
                        if args.len() > 1 {
                            return Err(GraphoidError::runtime(format!(
                                "Number method 'round' expects 0 or 1 arguments, but got {}",
                                args.len()
                            )));
                        }
                        match mode.as_str() {
                            "nearest_ten" => Ok(Value::number((n / 10.0).round() * 10.0)),
                            "nearest_hundred" => Ok(Value::number((n / 100.0).round() * 100.0)),
                            _ => Err(GraphoidError::runtime(format!(
                                "Unknown round() mode: :{}. Valid modes: :nearest_ten, :nearest_hundred",
                                mode
                            )))
                        }
                    }
                    _ => Err(GraphoidError::runtime(format!(
                        "round() argument must be a number (decimal places) or symbol (mode), got {}",
                        args[0].type_name()
                    )))
                }
            }
            "log" => {
                // log() - natural logarithm
                // log(base) - logarithm with specified base
                if args.is_empty() {
                    return Ok(Value::number(n.ln()));
                }

                if args.len() > 1 {
                    return Err(GraphoidError::runtime(format!(
                        "Number method 'log' expects 0 or 1 arguments, but got {}",
                        args.len()
                    )));
                }

                match &args[0].kind {
                    ValueKind::Number(base) => {
                        Ok(Value::number(n.log(*base)))
                    }
                    _ => Err(GraphoidError::runtime(format!(
                        "log() base must be a number, got {}",
                        args[0].type_name()
                    )))
                }
            }
            // Character code conversion (Phase 12 - stdlib robustness)
            "to_char" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Number method 'to_char' takes no arguments, but got {}",
                        args.len()
                    )));
                }

                let code = n.trunc() as i64;
                if code < 0 || code > 127 {
                    return Err(GraphoidError::runtime(format!(
                        "Character code {} out of ASCII range (0-127)",
                        code
                    )));
                }

                let ch = char::from_u32(code as u32)
                    .ok_or_else(|| GraphoidError::runtime(format!(
                        "Invalid character code: {}",
                        code
                    )))?;

                Ok(Value::string(ch.to_string()))
            }
            _ => Err(GraphoidError::runtime(format!(
                "Number does not have method '{}'",
                method
            ))),
        }
    }

    /// Phase 3: Evaluates a method call on a bignum value.
    fn eval_bignum_method(&self, bn: &BigNum, method: &str, args: &[Value]) -> Result<Value> {
        use num_bigint::BigInt;

        match method {
            "to_int" => {
                // Convert BigNum to Int64, checking for overflow
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'to_int' takes no arguments, but got {}",
                        args.len()
                    )));
                }

                match bn {
                    BigNum::Int64(i) => Ok(Value::bignum(BigNum::Int64(*i))),
                    BigNum::UInt64(u) => {
                        // Try to convert UInt64 to Int64
                        if *u > i64::MAX as u64 {
                            return Err(GraphoidError::runtime(format!(
                                "UInt64 value {} exceeds Int64::MAX, cannot convert to int",
                                u
                            )));
                        }
                        Ok(Value::bignum(BigNum::Int64(*u as i64)))
                    }
                    BigNum::Float128(f) => {
                        // Convert to f64, truncate, then check range
                        let f64_val: f64 = (*f).into();
                        let truncated = f64_val.trunc();

                        // Check if it fits in Int64 range
                        if truncated > i64::MAX as f64 || truncated < i64::MIN as f64 {
                            return Err(GraphoidError::runtime(format!(
                                "Float128 value exceeds Int64 range, cannot convert to int"
                            )));
                        }

                        // Convert to i64
                        Ok(Value::bignum(BigNum::Int64(truncated as i64)))
                    }
                    BigNum::BigInt(bi) => {
                        // Try to convert BigInt to Int64
                        use num_traits::ToPrimitive;

                        match bi.to_i64() {
                            Some(i) => Ok(Value::bignum(BigNum::Int64(i))),
                            None => Err(GraphoidError::runtime(format!(
                                "BigInt value exceeds Int64 range, cannot convert to int"
                            ))),
                        }
                    }
                }
            }
            "to_bigint" => {
                // Convert BigNum to BigInt
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'to_bigint' takes no arguments, but got {}",
                        args.len()
                    )));
                }

                match bn {
                    BigNum::Int64(i) => Ok(Value::bignum(BigNum::BigInt(BigInt::from(*i)))),
                    BigNum::UInt64(u) => Ok(Value::bignum(BigNum::BigInt(BigInt::from(*u)))),
                    BigNum::Float128(f) => {
                        // Convert to f64, truncate
                        let f64_val: f64 = (*f).into();
                        let truncated = f64_val.trunc();

                        // Convert to BigInt via i64 (safer than i128 from f64)
                        let as_i64 = truncated as i64;
                        Ok(Value::bignum(BigNum::BigInt(BigInt::from(as_i64))))
                    }
                    BigNum::BigInt(bi) => Ok(Value::bignum(BigNum::BigInt(bi.clone()))),
                }
            }
            _ => Err(GraphoidError::runtime(format!(
                "BigNumber does not have method '{}'",
                method
            ))),
        }
    }

    /// Evaluates a method call on a time value.
    fn eval_time_method(&self, timestamp: f64, method: &str, args: &[Value]) -> Result<Value> {
        use chrono::{DateTime, Datelike, Timelike};

        match method {
            "time_numbers" => {
                // time_numbers() - Extract year, month, day, hour, minute, second, weekday, day_of_year
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Time method 'time_numbers' takes no arguments, but got {}",
                        args.len()
                    )));
                }

                let seconds = timestamp.trunc() as i64;
                let nanos = ((timestamp - timestamp.trunc()) * 1_000_000_000.0) as u32;

                let dt = DateTime::from_timestamp(seconds, nanos)
                    .ok_or_else(|| GraphoidError::runtime("Invalid timestamp".to_string()))?;

                // Create hash with time components
                let mut hash = crate::values::Hash::new();
                hash.insert("year".to_string(), Value::number(dt.year() as f64)).ok();
                hash.insert("month".to_string(), Value::number(dt.month() as f64)).ok();
                hash.insert("day".to_string(), Value::number(dt.day() as f64)).ok();
                hash.insert("hour".to_string(), Value::number(dt.hour() as f64)).ok();
                hash.insert("minute".to_string(), Value::number(dt.minute() as f64)).ok();
                hash.insert("second".to_string(), Value::number(dt.second() as f64)).ok();
                hash.insert("weekday".to_string(), Value::number(dt.weekday().num_days_from_sunday() as f64)).ok();
                hash.insert("day_of_year".to_string(), Value::number(dt.ordinal() as f64)).ok();

                Ok(Value::map(hash))
            }
            _ => Err(GraphoidError::runtime(format!(
                "Time does not have method '{}'",
                method
            ))),
        }
    }

    /// Universal type casting: to_num() (spec line 3185)
    fn value_to_num(&self, value: &Value) -> Result<Value> {
        match &value.kind {
            ValueKind::Number(n) => Ok(Value::number(*n)),
            ValueKind::BigNumber(bn) => {
                // Convert BigNumber to f64 (may lose precision)
                Ok(Value::number(bn.to_f64()))
            }
            ValueKind::Boolean(b) => Ok(Value::number(if *b { 1.0 } else { 0.0 })),
            ValueKind::String(s) => {
                // Try to parse string to number, return none if invalid (spec line 3296)
                match s.parse::<f64>() {
                    Ok(n) => Ok(Value::number(n)),
                    Err(_) => Ok(Value::none()), // Spec: Invalid strings return none
                }
            }
            ValueKind::None => Ok(Value::number(0.0)), // Spec line 206: none.to_num() => 0
            ValueKind::Time(timestamp) => Ok(Value::number(*timestamp)), // Spec line 221: time.to_num() => Unix timestamp
            ValueKind::List(list) => {
                // List to_num() returns its size
                Ok(Value::number(list.len() as f64))
            }
            ValueKind::Map(hash) => {
                // Hash to_num() returns its size (number of keys)
                Ok(Value::number(hash.len() as f64))
            }
            _ => {
                // For other types, return 0 as a safe default
                Ok(Value::number(0.0))
            }
        }
    }

    /// Universal type casting: to_string() (spec line 3185)
    fn value_to_string(&self, value: &Value) -> Result<Value> {
        match &value.kind {
            ValueKind::String(s) => Ok(Value::string(s.clone())),
            ValueKind::Number(n) => Ok(Value::string(n.to_string())),
            ValueKind::Boolean(b) => Ok(Value::string(if *b { "true".to_string() } else { "false".to_string() })),
            ValueKind::None => Ok(Value::string(String::new())), // Spec line 207: none.to_string() => ""
            ValueKind::Time(timestamp) => {
                // Spec line 219: time.to_string() => ISO 8601 format
                use chrono::DateTime;
                let seconds = timestamp.trunc() as i64;
                let nanos = ((timestamp - timestamp.trunc()) * 1_000_000_000.0) as u32;
                if let Some(dt) = DateTime::from_timestamp(seconds, nanos) {
                    Ok(Value::string(dt.to_rfc3339()))
                } else {
                    // Invalid timestamp
                    Ok(Value::string("Invalid Time".to_string()))
                }
            }
            ValueKind::List(list) => {
                let items = list.to_vec();

                // Standard list stringification (removed byte array auto-conversion)
                // Note: Byte array conversion was too aggressive - it converted any numeric
                // list with values 0-255 to a string, causing [1,2,3] to display as empty
                // control characters. If byte array conversion is needed, it should be
                // explicit via a method like .to_bytes() or .to_utf8()
                let elements: Vec<String> = items
                    .iter()
                    .map(|v| match &v.kind {
                        ValueKind::String(s) => format!("\"{}\"", s),
                        ValueKind::Number(n) => {
                            // Format numbers nicely (no .0 for integers)
                            if n.fract() == 0.0 {
                                format!("{:.0}", n)
                            } else {
                                n.to_string()
                            }
                        }
                        ValueKind::Boolean(b) => b.to_string(),
                        ValueKind::None => "none".to_string(),
                        _ => v.type_name().to_string(),
                    })
                    .collect();
                Ok(Value::string(format!("[{}]", elements.join(", "))))
            }
            ValueKind::Map(hash) => {
                // Convert hash to string representation
                let keys = hash.keys();
                let mut pairs: Vec<String> = keys
                    .iter()
                    .map(|k| {
                        let v = hash.get(k).unwrap(); // Key came from keys(), must exist
                        let val_str = match &v.kind {
                            ValueKind::String(s) => format!("\"{}\"", s),
                            ValueKind::Number(n) => n.to_string(),
                            ValueKind::Boolean(b) => b.to_string(),
                            ValueKind::None => "none".to_string(),
                            _ => v.type_name().to_string(),
                        };
                        format!("\"{}\": {}", k, val_str)
                    })
                    .collect();
                pairs.sort(); // Deterministic order
                Ok(Value::string(format!("{{{}}}", pairs.join(", "))))
            }
            _ => {
                // For other types, use their type name
                Ok(Value::string(value.type_name().to_string()))
            }
        }
    }

    /// Phase 2: BigNum casting - to_bignum()
    /// Converts a value to BigNum (Float128)
    fn value_to_bignum(&self, value: &Value) -> Result<Value> {
        match &value.kind {
            ValueKind::Number(n) => {
                // Convert f64 to Float128
                use f128::f128;
                Ok(Value::bignum(BigNum::Float128(f128::from(*n))))
            }
            ValueKind::BigNumber(_) => {
                // Already a bignum, pass through
                Ok(value.clone())
            }
            ValueKind::String(s) => {
                // Try to parse string to bignum
                match s.parse::<f64>() {
                    Ok(n) => {
                        use f128::f128;
                        Ok(Value::bignum(BigNum::Float128(f128::from(n))))
                    }
                    Err(_) => Ok(Value::none()), // Invalid strings return none
                }
            }
            _ => {
                Err(GraphoidError::type_error(
                    "number, bignum, or string",
                    value.type_name()
                ))
            }
        }
    }

    /// Phase 2: BigNum casting - fits_in_num()
    /// Returns true if value can fit in f64 without overflow/underflow
    fn value_fits_in_num(&self, value: &Value) -> Result<Value> {
        match &value.kind {
            ValueKind::Number(_) => {
                // Numbers always fit in num
                Ok(Value::boolean(true))
            }
            ValueKind::BigNumber(bn) => {
                // Check if BigNumber can be represented in f64 without overflow
                let f64_val = bn.to_f64();

                // Check for infinity (overflow)
                if f64_val.is_infinite() {
                    return Ok(Value::boolean(false));
                }

                // Value fits in f64
                Ok(Value::boolean(true))
            }
            _ => {
                // Other types can be converted to num, so they "fit"
                Ok(Value::boolean(true))
            }
        }
    }

    /// Universal type casting: to_bool() (spec line 3185)
    fn value_to_bool(&self, value: &Value) -> Result<Value> {
        match &value.kind {
            ValueKind::Boolean(b) => Ok(Value::boolean(*b)),
            ValueKind::Number(n) => Ok(Value::boolean(*n != 0.0 && !n.is_nan())),
            ValueKind::String(s) => Ok(Value::boolean(!s.is_empty())),
            ValueKind::None => Ok(Value::boolean(false)),
            ValueKind::Time(_) => Ok(Value::boolean(true)), // Time values are always truthy
            ValueKind::List(list) => Ok(Value::boolean(!list.is_empty())),
            ValueKind::Map(hash) => Ok(Value::boolean(!hash.is_empty())),
            _ => {
                // For other types, default to true (non-empty values are truthy)
                Ok(Value::boolean(true))
            }
        }
    }

    /// Evaluates a method call on an error object.
    fn eval_error_method(&self, err: &ErrorObject, method: &str, args: &[Value]) -> Result<Value> {
        match method {
            "type" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Error.type() takes no arguments, but got {}", args.len()
                    )));
                }
                Ok(Value::string(err.error_type.clone()))
            }
            "message" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Error.message() takes no arguments, but got {}", args.len()
                    )));
                }
                Ok(Value::string(err.message.clone()))
            }
            "file" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Error.file() takes no arguments, but got {}", args.len()
                    )));
                }
                Ok(err.file.as_ref().map(|f| Value::string(f.clone())).unwrap_or(Value::none()))
            }
            "line" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Error.line() takes no arguments, but got {}", args.len()
                    )));
                }
                Ok(Value::number(err.line as f64))
            }
            "column" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Error.column() takes no arguments, but got {}", args.len()
                    )));
                }
                Ok(Value::number(err.column as f64))
            }
            "stack_trace" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Error.stack_trace() takes no arguments, but got {}", args.len()
                    )));
                }
                Ok(Value::string(err.formatted_stack_trace()))
            }
            "full_chain" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Error.full_chain() takes no arguments, but got {}", args.len()
                    )));
                }
                Ok(Value::string(err.full_chain()))
            }
            "cause" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Error.cause() takes no arguments, but got {}", args.len()
                    )));
                }
                Ok(err.cause.as_ref().map(|c| Value::error((**c).clone())).unwrap_or(Value::none()))
            }
            "caused_by" => {
                // caused_by(other_error) - chain errors
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "Error.caused_by() expects 1 argument (error), got {}", args.len()
                    )));
                }
                match &args[0].kind {
                    ValueKind::Error(cause) => {
                        let mut new_err = err.clone();
                        new_err.cause = Some(Box::new(cause.clone()));
                        Ok(Value::error(new_err))
                    }
                    _other => Err(GraphoidError::runtime(format!(
                        "Error.caused_by() expects an error argument, got {}", args[0].type_name()
                    )))
                }
            }
            _ => Err(GraphoidError::runtime(format!(
                "Error does not have method '{}'",
                method
            ))),
        }
    }

    /// Evaluates a method call on a pattern node object.
    fn eval_pattern_node_method(&self, pn: &crate::values::PatternNode, method: &str, args: &[Value]) -> Result<Value> {
        match method {
            "bind" => {
                // node_obj.bind("new_var") - returns new PatternNode with updated variable
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "PatternNode.bind() expects 1 argument (variable name), got {}",
                        args.len()
                    )));
                }
                let new_variable = args[0].to_string_value();
                Ok(Value::pattern_node(Some(new_variable), pn.node_type.clone()))
            }
            "variable" => {
                // Property access - returns the variable name
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "PatternNode.variable is a property, not a method (got {} arguments)",
                        args.len()
                    )));
                }
                Ok(pn.variable.as_ref().map(|v| Value::string(v.clone())).unwrap_or(Value::none()))
            }
            "type" => {
                // Property access - returns the node type
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "PatternNode.type is a property, not a method (got {} arguments)",
                        args.len()
                    )));
                }
                Ok(pn.node_type.as_ref().map(|t| Value::string(t.clone())).unwrap_or(Value::none()))
            }
            "pattern_type" => {
                // Returns symbol :node
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "PatternNode.pattern_type is a property, not a method (got {} arguments)",
                        args.len()
                    )));
                }
                Ok(Value::symbol("node".to_string()))
            }
            _ => Err(GraphoidError::runtime(format!(
                "PatternNode does not have method '{}'",
                method
            ))),
        }
    }

    /// Evaluates a method call on a pattern edge object.
    fn eval_pattern_edge_method(&self, pe: &crate::values::PatternEdge, method: &str, args: &[Value]) -> Result<Value> {
        match method {
            "edge_type" => {
                // Property access - returns the edge type
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "PatternEdge.edge_type is a property, not a method (got {} arguments)",
                        args.len()
                    )));
                }
                Ok(pe.edge_type.as_ref().map(|t| Value::string(t.clone())).unwrap_or(Value::none()))
            }
            "direction" => {
                // Property access - returns the direction symbol
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "PatternEdge.direction is a property, not a method (got {} arguments)",
                        args.len()
                    )));
                }
                Ok(Value::symbol(pe.direction.clone()))
            }
            "pattern_type" => {
                // Returns symbol :edge
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "PatternEdge.pattern_type is a property, not a method (got {} arguments)",
                        args.len()
                    )));
                }
                Ok(Value::symbol("edge".to_string()))
            }
            _ => Err(GraphoidError::runtime(format!(
                "PatternEdge does not have method '{}'",
                method
            ))),
        }
    }

    /// Evaluates a method call on a pattern path object.
    fn eval_pattern_path_method(&self, pp: &crate::values::PatternPath, method: &str, args: &[Value]) -> Result<Value> {
        match method {
            "edge_type" => {
                // Property access - returns the edge type
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "PatternPath.edge_type is a property, not a method (got {} arguments)",
                        args.len()
                    )));
                }
                Ok(Value::string(pp.edge_type.clone()))
            }
            "min" => {
                // Property access - returns the minimum path length
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "PatternPath.min is a property, not a method (got {} arguments)",
                        args.len()
                    )));
                }
                Ok(Value::number(pp.min as f64))
            }
            "max" => {
                // Property access - returns the maximum path length
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "PatternPath.max is a property, not a method (got {} arguments)",
                        args.len()
                    )));
                }
                Ok(Value::number(pp.max as f64))
            }
            "direction" => {
                // Property access - returns the direction symbol
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "PatternPath.direction is a property, not a method (got {} arguments)",
                        args.len()
                    )));
                }
                Ok(Value::symbol(pp.direction.clone()))
            }
            "pattern_type" => {
                // Returns symbol :path
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "PatternPath.pattern_type is a property, not a method (got {} arguments)",
                        args.len()
                    )));
                }
                Ok(Value::symbol("path".to_string()))
            }
            _ => Err(GraphoidError::runtime(format!(
                "PatternPath does not have method '{}'",
                method
            ))),
        }
    }

    /// Evaluates a method call on pattern match results.
    fn eval_pattern_match_results_method(&mut self, results: &crate::values::PatternMatchResults, method: &str, args: &[Value]) -> Result<Value> {
        match method {
            "len" | "count" | "size" => {
                // Get the number of matches
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "PatternMatchResults.{}() expects no arguments, got {}",
                        method,
                        args.len()
                    )));
                }
                Ok(Value::number(results.len() as f64))
            }
            "where" => {
                // Filter results with a lambda
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "PatternMatchResults.where() expects 1 argument (predicate function), got {}",
                        args.len()
                    )));
                }

                // Clone results for mutation
                let filtered = results.clone();

                // Apply filter - extract function from predicate value
                let func = match &args[0].kind {
                    ValueKind::Function(f) => f,
                    _ => {
                        return Err(GraphoidError::runtime(format!(
                            "PatternMatchResults.where() expects a function, got {}",
                            args[0].type_name()
                        )));
                    }
                };

                // Filter using the provided predicate
                // Each match is a HashMap<String, String> (variable -> node_id)
                // Convert to a map and pass to the predicate
                let original_bindings = filtered.iter().cloned().collect::<Vec<_>>();
                let mut kept_bindings = Vec::new();

                for binding in original_bindings {
                    // Convert binding to a Map value
                    let mut map = crate::values::Hash::new();
                    for (var, node_id) in &binding {
                        // Get node value from graph
                        if let Some(node) = filtered.graph().nodes.get(node_id) {
                            let _ = map.insert(var.clone(), node.value.clone());
                        }
                    }
                    let map_value = Value::map(map);

                    // Call predicate
                    let result = self.call_function(func, &[map_value])?;

                    // Keep if truthy
                    if result.is_truthy() {
                        kept_bindings.push(binding);
                    }
                }

                // Create new PatternMatchResults with filtered bindings
                let new_results = crate::values::PatternMatchResults::new(kept_bindings, filtered.graph().clone());
                Ok(Value::pattern_match_results(new_results))
            }
            "select" | "return" => {
                // Unified select/return method - projects variables and/or properties
                // Both names supported: select() and return()
                // Accepts list of specifiers:
                //   "person" -> returns full variable binding
                //   "person.name" -> returns property value
                //   Mixed: ["person", "friend.age"] -> returns both

                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "PatternMatchResults.return() expects 1 argument (list of specifiers), got {}",
                        args.len()
                    )));
                }

                // Get specifiers from list
                let specifiers = match &args[0].kind {
                    ValueKind::List(list) => {
                        list.to_vec()
                            .iter()
                            .map(|v| v.to_string_value())
                            .collect::<Vec<String>>()
                    }
                    _ => {
                        return Err(GraphoidError::type_error("list", args[0].type_name()));
                    }
                };

                // Process each match and build projection
                let mut projected_list = Vec::new();

                for binding in results.iter() {
                    let mut result_map = crate::values::Hash::new();

                    for spec in &specifiers {
                        if spec.contains('.') {
                            // Property access: "variable.property"
                            let parts: Vec<&str> = spec.splitn(2, '.').collect();
                            if parts.len() == 2 {
                                let var_name = parts[0];
                                let prop_name = parts[1];

                                // Get node from binding
                                if let Some(node_id) = binding.get(var_name) {
                                    if let Some(node) = results.graph().nodes.get(node_id) {
                                        // Try to get property from node's value
                                        let prop_value = match &node.value.kind {
                                            ValueKind::Map(map) => {
                                                map.get(prop_name).cloned().unwrap_or(Value::none())
                                            }
                                            _ => {
                                                // If node value isn't a map, check properties directly
                                                node.properties.get(prop_name)
                                                    .cloned()
                                                    .unwrap_or(Value::none())
                                            }
                                        };

                                        let _ = result_map.insert(spec.clone(), prop_value);
                                    }
                                }
                            }
                        } else {
                            // Variable access: return full node value
                            if let Some(node_id) = binding.get(spec) {
                                if let Some(node) = results.graph().nodes.get(node_id) {
                                    let _ = result_map.insert(spec.clone(), node.value.clone());
                                }
                            }
                        }
                    }

                    projected_list.push(Value::map(result_map));
                }

                Ok(Value::list(crate::values::List::from_vec(projected_list)))
            }
            _ => Err(GraphoidError::runtime(format!(
                "PatternMatchResults does not have method '{}'",
                method
            ))),
        }
    }

    /// Helper: Apply node filter to graph, returns set of matching node IDs.
    /// If invert=true, returns nodes that DON'T match the filter.
    fn eval_call(&mut self, callee: &Expr, args: &[crate::ast::Argument]) -> Result<Value> {
        use crate::ast::Argument;

        // Check if this is a builtin function call (special handling)
        if let Expr::Variable { name, .. } = callee {
            match name.as_str() {
                "print" => {
                    // print(...) - outputs values to stdout with space separation
                    // Accepts variable number of arguments
                    let mut output = String::new();

                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            output.push(' ');
                        }

                        let value = match arg {
                            Argument::Positional { expr, .. } => self.eval_expr(expr)?,
                            Argument::Named { .. } => {
                                return Err(GraphoidError::runtime(
                                    "print() does not accept named arguments".to_string()
                                ));
                            }
                        };

                        // Convert value to string representation
                        let str_repr = match &value.kind {
                            ValueKind::String(s) => s.clone(),
                            ValueKind::Number(n) => {
                                // Format numbers nicely - remove .0 for integers
                                if n.fract() == 0.0 && n.is_finite() {
                                    format!("{:.0}", n)
                                } else {
                                    n.to_string()
                                }
                            }
                            ValueKind::Boolean(b) => b.to_string(),
                            ValueKind::Symbol(s) => format!(":{}", s),
                            ValueKind::None => "none".to_string(),
                            _ => value.to_string_value(),
                        };

                        output.push_str(&str_repr);
                    }

                    // Output to console or buffer depending on capture mode
                    if self.output_capture_enabled {
                        self.capture_print(&output);
                    } else {
                        println!("{}", output);
                    }
                    return Ok(Value::none());
                }
                "RuntimeError" | "ValueError" | "TypeError" | "IOError" | "NetworkError" | "ParseError" => {
                    // Evaluate the message argument
                    if args.len() != 1 {
                        return Err(GraphoidError::runtime(format!(
                            "{} constructor expects 1 argument (message), got {}",
                            name, args.len()
                        )));
                    }
                    let message_value = match &args[0] {
                        Argument::Positional { expr, .. } => self.eval_expr(expr)?,
                        Argument::Named { .. } => {
                            return Err(GraphoidError::runtime(format!(
                                "{} constructor does not support named arguments",
                                name
                            )));
                        }
                    };
                    let message = message_value.to_string_value();

                    // Create the error object with current call stack
                    let error_obj = ErrorObject::with_stack_trace(
                        name.clone(),
                        message,
                        self.current_file.as_ref().map(|p| p.to_string_lossy().to_string()),
                        0,    // line (not available at call site)
                        0,    // column (not available at call site)
                        self.call_stack.clone(),
                    );
                    return Ok(Value::error(error_obj));
                }
                "get_errors" => {
                    // get_errors() - returns list of collected errors
                    if !args.is_empty() {
                        return Err(GraphoidError::runtime(format!(
                            "get_errors() takes no arguments, got {}",
                            args.len()
                        )));
                    }

                    // Get collected errors from error_collector
                    let errors = self.error_collector.get_errors();

                    // Convert to list of error objects
                    let error_values: Vec<Value> = errors
                        .iter()
                        .map(|collected_err| {
                            Value::error(ErrorObject::new(
                                collected_err.error.error_type(),
                                collected_err.error.to_string(),
                                collected_err.file.clone(),
                                collected_err.position.line,
                                collected_err.position.column,
                            ))
                        })
                        .collect();

                    return Ok(Value::list(List::from_vec(error_values)));
                }
                "clear_errors" => {
                    // clear_errors() - clears the error collection
                    if !args.is_empty() {
                        return Err(GraphoidError::runtime(format!(
                            "clear_errors() takes no arguments, got {}",
                            args.len()
                        )));
                    }

                    self.error_collector.clear();
                    return Ok(Value::none());
                }
                "exec" => {
                    // exec(path) - execute a .gr file and return its stdout output as string
                    if args.len() != 1 {
                        return Err(GraphoidError::runtime(format!(
                            "exec() expects 1 argument (file path), got {}",
                            args.len()
                        )));
                    }

                    let path_value = match &args[0] {
                        Argument::Positional { expr, .. } => self.eval_expr(expr)?,
                        Argument::Named { .. } => {
                            return Err(GraphoidError::runtime(
                                "exec() does not accept named arguments".to_string()
                            ));
                        }
                    };

                    let path = match &path_value.kind {
                        ValueKind::String(s) => s.clone(),
                        _ => {
                            return Err(GraphoidError::runtime(format!(
                                "exec() path must be a string, got {}",
                                path_value.type_name()
                            )));
                        }
                    };

                    // Read the file
                    let source = std::fs::read_to_string(&path).map_err(|e| {
                        GraphoidError::runtime(format!("exec(): failed to read file '{}': {}", path, e))
                    })?;

                    // Create a new executor with output capture enabled
                    let mut file_executor = Executor::new();
                    file_executor.enable_output_capture();
                    file_executor.set_current_file(Some(std::path::PathBuf::from(&path)));

                    // Execute the file
                    file_executor.execute_source(&source)?;

                    // Get captured output
                    let output = file_executor.get_captured_output();

                    return Ok(Value::string(output));
                }
                "node" => {
                    // node(variable, type: optional) - creates a pattern node object
                    // First positional arg is variable (optional)
                    // Named arg "type" is node type (optional)

                    let mut variable: Option<String> = None;
                    let mut node_type: Option<String> = None;

                    for arg in args {
                        match arg {
                            Argument::Positional { expr, .. } => {
                                if variable.is_some() {
                                    return Err(GraphoidError::runtime(
                                        "node() accepts at most one positional argument (variable)".to_string()
                                    ));
                                }
                                let val = self.eval_expr(expr)?;
                                variable = Some(val.to_string_value());
                            }
                            Argument::Named { name: param_name, value, .. } => {
                                if param_name == "type" {
                                    let val = self.eval_expr(value)?;
                                    node_type = Some(val.to_string_value());
                                } else {
                                    return Err(GraphoidError::runtime(format!(
                                        "node() does not accept parameter '{}'", param_name
                                    )));
                                }
                            }
                        }
                    }

                    return Ok(Value::pattern_node(variable, node_type));
                }
                "edge" => {
                    // edge(type: optional, direction: optional) - creates a pattern edge object
                    // Both args are named parameters

                    let mut edge_type: Option<String> = None;
                    let mut direction: String = "outgoing".to_string(); // default

                    for arg in args {
                        match arg {
                            Argument::Positional { .. } => {
                                return Err(GraphoidError::runtime(
                                    "edge() does not accept positional arguments, use named parameters: type, direction".to_string()
                                ));
                            }
                            Argument::Named { name: param_name, value, .. } => {
                                let val = self.eval_expr(value)?;
                                match param_name.as_str() {
                                    "type" => {
                                        edge_type = Some(val.to_string_value());
                                    }
                                    "direction" => {
                                        // Should be a symbol like :outgoing
                                        if let ValueKind::Symbol(s) = &val.kind {
                                            direction = s.clone();
                                        } else {
                                            return Err(GraphoidError::runtime(format!(
                                                "edge() direction must be a symbol (:outgoing, :incoming, or :both), got {}",
                                                val.type_name()
                                            )));
                                        }
                                    }
                                    _ => {
                                        return Err(GraphoidError::runtime(format!(
                                            "edge() does not accept parameter '{}'", param_name
                                        )));
                                    }
                                }
                            }
                        }
                    }

                    return Ok(Value::pattern_edge(edge_type, direction));
                }
                "path" => {
                    // path(edge_type: string, min: num, max: num, direction: optional)
                    // All named parameters

                    let mut edge_type: Option<String> = None;
                    let mut min: Option<usize> = None;
                    let mut max: Option<usize> = None;
                    let mut direction: String = "outgoing".to_string(); // default

                    for arg in args {
                        match arg {
                            Argument::Positional { .. } => {
                                return Err(GraphoidError::runtime(
                                    "path() does not accept positional arguments, use named parameters: edge_type, min, max, direction".to_string()
                                ));
                            }
                            Argument::Named { name: param_name, value, .. } => {
                                let val = self.eval_expr(value)?;
                                match param_name.as_str() {
                                    "edge_type" | "type" => {
                                        edge_type = Some(val.to_string_value());
                                    }
                                    "min" => {
                                        if let ValueKind::Number(n) = val.kind {
                                            min = Some(n as usize);
                                        } else {
                                            return Err(GraphoidError::runtime(format!(
                                                "path() min must be a number, got {}",
                                                val.type_name()
                                            )));
                                        }
                                    }
                                    "max" => {
                                        if let ValueKind::Number(n) = val.kind {
                                            max = Some(n as usize);
                                        } else {
                                            return Err(GraphoidError::runtime(format!(
                                                "path() max must be a number, got {}",
                                                val.type_name()
                                            )));
                                        }
                                    }
                                    "direction" => {
                                        if let ValueKind::Symbol(s) = &val.kind {
                                            direction = s.clone();
                                        } else {
                                            return Err(GraphoidError::runtime(format!(
                                                "path() direction must be a symbol (:outgoing, :incoming, or :both), got {}",
                                                val.type_name()
                                            )));
                                        }
                                    }
                                    _ => {
                                        return Err(GraphoidError::runtime(format!(
                                            "path() does not accept parameter '{}'", param_name
                                        )));
                                    }
                                }
                            }
                        }
                    }

                    // Validate required parameters
                    // edge_type is optional - if not provided, match any edge type
                    let edge_type = edge_type.unwrap_or_else(|| "".to_string()); // Empty string means any type
                    let min = min.ok_or_else(|| {
                        GraphoidError::runtime("path() requires 'min' parameter".to_string())
                    })?;
                    let max = max.ok_or_else(|| {
                        GraphoidError::runtime("path() requires 'max' parameter".to_string())
                    })?;

                    // Validate min <= max
                    if min > max {
                        return Err(GraphoidError::runtime(format!(
                            "path() min ({}) must be <= max ({})",
                            min, max
                        )));
                    }

                    return Ok(Value::pattern_path(edge_type, min, max, direction));
                }
                _ => {}
            }
        }

        // Special handling for function calls by name (to support overloading)
        // If callee is a variable, try to find overloaded function with matching arity
        if let Expr::Variable { name, .. } = callee {
            // Count arguments to determine arity
            let arity = args.len();

            // Check if there are overloaded functions with this name
            if let Some(overloads) = self.global_functions.get(name) {
                // Find function with matching arity
                if let Some(func) = overloads.iter().find(|f| f.parameters.len() == arity).cloned() {
                    // Process arguments and call the matched overload
                    let arg_values = self.process_arguments(&func, args)?;

                    // Collect writeback info for mutable arguments
                    let writebacks = self.collect_writebacks(&func, args);
                    self.writeback_stack.push(writebacks);

                    return self.call_function(&func, &arg_values);
                }
            }

            // Phase 2 Implicit Self: Check if `self` is a graph with this method
            // This allows calling methods without explicit `self.` prefix
            if let Ok(self_value) = self.env.get("self") {
                if let ValueKind::Graph(ref graph) = self_value.kind {
                    if graph.has_method(name) {
                        // Clone the graph for method call
                        let graph_clone = graph.clone();
                        // Create a dummy Variable expression for self to use in call_graph_method
                        let self_expr = Expr::Variable {
                            name: "self".to_string(),
                            position: crate::error::SourcePosition::unknown(),
                        };
                        // Get the method
                        if let Some(func) = graph_clone.get_method(name).cloned() {
                            // Evaluate arguments
                            let arg_values = self.eval_arguments(args)?;
                            // Call the method with self binding
                            return self.call_graph_method(&graph_clone, &func, &arg_values, &self_expr);
                        }
                    }
                }
            }
        }

        // Fallback: Evaluate the callee to get the function (for non-overloaded calls)
        let callee_value = self.eval_expr(callee)?;

        // Check if it's a function
        let func = match &callee_value.kind {
            ValueKind::Function(f) => f,
            _other => {
                return Err(GraphoidError::type_error(
                    "function",
                    callee_value.type_name(),
                ));
            }
        };

        // Process arguments (positional and named) and match to parameters
        let arg_values = self.process_arguments(&func, args)?;

        // Collect writeback info for mutable arguments
        let writebacks = self.collect_writebacks(&func, args);
        self.writeback_stack.push(writebacks);

        // Delegate to call_function (which has graph tracking)
        let result = self.call_function(&func, &arg_values);

        // Process writebacks (pop happens inside call_function before env restore)
        // The actual writeback is done in call_function

        result
    }

    /// Collect writeback info for mutable arguments (arg! syntax).
    /// Returns a list of WritebackInfo for parameters that need to be written back.
    fn collect_writebacks(&self, func: &Function, args: &[crate::ast::Argument]) -> Vec<WritebackInfo> {
        use crate::ast::Argument;

        let mut writebacks = Vec::new();
        let mut positional_idx = 0;

        for arg in args {
            match arg {
                Argument::Positional { expr, mutable } => {
                    if *mutable {
                        // Get the source variable name (only simple variables can be written back)
                        if let Expr::Variable { name, .. } = expr {
                            // Get the parameter name at this position
                            if positional_idx < func.parameters.len() {
                                writebacks.push(WritebackInfo {
                                    param_name: func.parameters[positional_idx].name.clone(),
                                    source_var_name: name.clone(),
                                });
                            }
                        }
                        // For non-variable expressions with !, we silently ignore
                        // (could add a warning later)
                    }
                    positional_idx += 1;
                }
                Argument::Named { name, value, mutable } => {
                    if *mutable {
                        // Get the source variable name
                        if let Expr::Variable { name: var_name, .. } = value {
                            writebacks.push(WritebackInfo {
                                param_name: name.clone(),
                                source_var_name: var_name.clone(),
                            });
                        }
                    }
                }
            }
        }

        writebacks
    }

    /// Process function arguments (positional and named) and match them to parameters.
    /// Returns a Vec<Value> with values in parameter order.
    /// Handles variadic parameters by collecting remaining args into a list.
    fn process_arguments(&mut self, func: &Function, args: &[crate::ast::Argument]) -> Result<Vec<Value>> {
        use crate::ast::Argument;
        use std::collections::{HashMap, HashSet};

        let param_count = func.parameters.len();

        // Find variadic parameter index if any
        let variadic_idx = func.parameters.iter().position(|p| p.is_variadic);

        // Track which parameters have been assigned
        let mut assigned: Vec<Option<Value>> = vec![None; param_count];
        let mut assigned_names: HashSet<String> = HashSet::new();
        let mut variadic_values: Vec<Value> = Vec::new();

        // Build parameter name -> index mapping
        let mut param_index: HashMap<String, usize> = HashMap::new();
        for (i, param) in func.parameters.iter().enumerate() {
            param_index.insert(param.name.clone(), i);
        }

        // Track the next positional parameter index
        let mut next_positional_idx = 0;

        // Process each argument
        for arg in args {
            match arg {
                Argument::Named { name, value, .. } => {
                    // Find parameter by name
                    let idx = param_index.get(name).ok_or_else(|| {
                        GraphoidError::runtime(format!(
                            "Unknown parameter '{}' in function '{}'",
                            name,
                            func.name.as_ref().unwrap_or(&"<anonymous>".to_string())
                        ))
                    })?;

                    // Check if already assigned
                    if assigned_names.contains(name) {
                        return Err(GraphoidError::runtime(format!(
                            "Parameter '{}' specified multiple times",
                            name
                        )));
                    }

                    // Evaluate and assign
                    let val = self.eval_expr(value)?;
                    assigned[*idx] = Some(val);
                    assigned_names.insert(name.clone());
                }
                Argument::Positional { expr, .. } => {
                    // Find next unassigned positional parameter
                    while next_positional_idx < param_count && assigned[next_positional_idx].is_some() {
                        next_positional_idx += 1;
                    }

                    // If we've reached a variadic parameter, collect remaining args
                    if let Some(var_idx) = variadic_idx {
                        if next_positional_idx == var_idx {
                            // Collect this and all remaining positional args for variadic
                            let val = self.eval_expr(expr)?;
                            variadic_values.push(val);
                            continue;
                        }
                    }

                    if next_positional_idx >= param_count {
                        return Err(GraphoidError::runtime(format!(
                            "Too many arguments for function '{}'",
                            func.name.as_ref().unwrap_or(&"<anonymous>".to_string())
                        )));
                    }

                    // Evaluate and assign
                    let val = self.eval_expr(expr)?;
                    assigned[next_positional_idx] = Some(val);
                    assigned_names.insert(func.parameters[next_positional_idx].name.clone());
                    next_positional_idx += 1;
                }
            }
        }

        // Fill in defaults and check for missing required parameters
        let mut result: Vec<Value> = Vec::new();
        for (i, param) in func.parameters.iter().enumerate() {
            if param.is_variadic {
                // Assign collected variadic values as a list
                result.push(Value::list(List::from_vec(variadic_values.clone())));
            } else if let Some(val) = assigned[i].take() {
                result.push(val);
            } else if let Some(default_expr) = &param.default_value {
                // Evaluate default value
                let default_val = self.eval_expr(default_expr)?;
                result.push(default_val);
            } else {
                // Required parameter not provided
                return Err(GraphoidError::runtime(format!(
                    "Missing required parameter '{}' in function '{}'",
                    param.name,
                    func.name.as_ref().unwrap_or(&"<anonymous>".to_string())
                )));
            }
        }

        Ok(result)
    }

    /// Helper method to call a function with given argument values.
    /// Used by map, filter, each, and other functional methods.
    pub(crate) fn call_function(&mut self, func: &Function, arg_values: &[Value]) -> Result<Value> {
        // Note: Argument validation is done by process_arguments() in eval_call()
        // For direct calls (e.g., from map/filter), we trust arg_values are correct

        // Quick sanity check: arg_values should match parameter count
        if arg_values.len() != func.parameters.len() {
            return Err(GraphoidError::runtime(format!(
                "Internal error: arg_values length ({}) doesn't match parameter count ({})",
                arg_values.len(),
                func.parameters.len()
            )));
        }

        // Find or register function in the function graph
        let func_id = if let Some(node_id) = &func.node_id {
            // Function already has a node_id (was registered at definition time)
            node_id.clone()
        } else if let Some(fname) = &func.name {
            // Named function without node_id: look up existing node
            let graph = self.function_graph.borrow();
            if let Some(node) = graph.get_function_by_name(fname) {
                node.node_id.clone()
            } else {
                // Not found, register it
                drop(graph);
                self.function_graph.borrow_mut().register_function(func.clone())
            }
        } else {
            // Lambda without node_id: register it now
            self.function_graph.borrow_mut().register_function(func.clone())
        };

        // Push function onto call stack (traditional - for backward compatibility)
        let func_name = func.name.as_ref().unwrap_or(&"<anonymous>".to_string()).clone();
        self.call_stack.push(func_name.clone());

        // Push function call onto the graph (this is the graph path!)
        self.function_graph.borrow_mut().push_call(func_id.clone(), arg_values.to_vec());

        // Save current environment FIRST (before creating call_env)
        // For named functions, we need to use the environment where functions are defined,
        // not the current call environment (which might be inside another function)

        // Use the captured environment (shared mutable for closures)
        // This enables closures to maintain state across calls
        let parent_env = func.env.borrow().clone();

        let call_env = Environment::with_parent(parent_env);

        // Save current environment and switch to call environment
        let mut saved_env = std::mem::replace(&mut self.env, call_env);

        // Implicit self in blocks: If this is an anonymous closure (block) and we're
        // inside a method context, inject the method's `self` into the block's environment.
        // This enables DSL-style syntax like: runner.describe("x") { it("y") { ... } }
        //
        // Priority: Use the parent environment's `self` if it exists (in case the method
        // modified self before calling the block), otherwise fall back to block_self_stack.
        if func.name.is_none() {
            let block_self = saved_env.get("self").ok()
                .or_else(|| self.block_self_stack.last().map(|e| e.value.clone()));
            if let Some(self_value) = block_self {
                self.env.define("self".to_string(), self_value);
            }
        }

        // Execute function body - either pattern matching or traditional
        let mut return_value = Value::none();
        let execution_result: Result<()> = (|| {
            // Check if this is a pattern matching function
            if let Some(ref pattern_clauses) = func.pattern_clauses {
                // Pattern matching function - use PatternMatcher to find matching clause
                use crate::execution::PatternMatcher;

                let matcher = PatternMatcher::new();

                // Create a closure to evaluate guard expressions with temporary bindings
                let eval_guard = |guard_expr: &crate::ast::Expr, bindings: &std::collections::HashMap<String, Value>| -> Result<bool> {
                    // Create temporary scope with current environment as parent
                    let temp_env = Environment::with_parent(self.env.clone());
                    let saved_env = std::mem::replace(&mut self.env, temp_env);

                    // Bind pattern variables in temporary scope
                    for (var_name, value) in bindings {
                        self.env.define(var_name.clone(), value.clone());
                    }

                    // Evaluate guard expression
                    let guard_value = self.eval_expr(guard_expr);

                    // Restore original environment
                    self.env = saved_env;

                    // Return whether guard is truthy
                    guard_value.map(|v| v.is_truthy())
                };

                let match_result = matcher.find_match(pattern_clauses, arg_values, eval_guard)?;

                if let Some((matched_clause, bindings)) = match_result {
                    // Bind pattern variables to environment
                    for (var_name, value) in bindings {
                        self.env.define(var_name, value);
                    }

                    // Execute the clause body expression
                    return_value = self.eval_expr(&matched_clause.body)?;
                } else {
                    // No pattern matched - return none
                    return_value = Value::none();
                }
            } else {
                // Traditional function with parameter binding and statement body

                // Bind parameters to argument values
                // Note: arg_values already has variadic parameters properly bundled as lists
                // thanks to process_arguments(), so we just bind them directly
                for (i, param) in func.parameters.iter().enumerate() {
                    if i < arg_values.len() {
                        self.env.define(param.name.clone(), arg_values[i].clone());
                    } else {
                        // This should not happen since process_arguments validates everything
                        return Err(GraphoidError::runtime(format!(
                            "Internal error: missing value for parameter '{}'",
                            param.name
                        )));
                    }
                }

                // Execute function body statements
                for stmt in &func.body {
                    match self.eval_stmt(stmt)? {
                        Some(val) => {
                            // Return statement executed
                            return_value = val;
                            break;
                        }
                        None => {
                            // Normal statement, continue
                        }
                    }
                }
            }
            Ok(())
        })();

        // Process writebacks for mutable arguments (arg! syntax)
        // Capture parameter values before restoring the environment
        let writeback_values: Vec<(String, Value)> = if let Some(writebacks) = self.writeback_stack.pop() {
            writebacks
                .iter()
                .filter_map(|wb| {
                    self.env.get(&wb.param_name).ok().map(|val| {
                        (wb.source_var_name.clone(), val)
                    })
                })
                .collect()
        } else {
            Vec::new()
        };

        // Save modifications back to the captured environment (for closure state)
        // Extract the parent environment from call_env (which may have been modified)
        if let Some(modified_parent) = self.env.take_parent() {
            // Update the captured environment with modifications
            *func.env.borrow_mut() = *modified_parent;
        }

        // Implicit self writeback: If this was an anonymous block with injected `self`,
        // propagate any modifications to `self` back to the block_self_stack AND
        // to the parent environment (so the method's `self` stays in sync).
        // This enables DSL patterns where blocks modify state through method calls.
        if func.name.is_none() {
            if let Some(modified_self) = self.env.get("self").ok() {
                // Update block_self_stack so outer methods see the change
                if let Some(entry) = self.block_self_stack.last_mut() {
                    entry.value = modified_self.clone();
                }
                // Also update `self` in the saved (parent) environment
                // This ensures the enclosing method's `self` is updated
                let _ = saved_env.set("self", modified_self);
            }
        }

        // Restore original environment
        self.env = saved_env;

        // Write back mutable argument values to the caller's scope
        for (var_name, value) in writeback_values {
            // Try to set in the restored environment
            let _ = self.env.set(&var_name, value);
        }

        // Pop function from call stack (traditional)
        self.call_stack.pop();

        // Pop function from graph with return value
        self.function_graph.borrow_mut().pop_call(return_value.clone());

        // Propagate errors
        execution_result?;

        Ok(return_value)
    }

    /// Helper method to call a user-defined method on a graph with `self` binding.
    /// Used for class-like graph methods defined with `fn Graph.method() { }` syntax.
    ///
    /// The `object_expr` parameter is used to persist mutations to `self` back to the
    /// original graph variable after method execution.
    ///
    /// This method enforces method constraint rules (Phase 11):
    /// - `:no_node_additions` - Methods cannot add nodes
    /// - `:no_node_removals` - Methods cannot remove nodes
    /// - `:no_edge_additions` - Methods cannot add edges
    /// - `:no_edge_removals` - Methods cannot remove edges
    /// - `:read_only` - Methods cannot modify the graph at all
    pub(crate) fn call_graph_method(&mut self, graph: &crate::values::Graph, func: &Function, arg_values: &[Value], object_expr: &Expr) -> Result<Value> {
        self.call_graph_method_impl(graph, func, arg_values, object_expr, true)
    }

    /// Phase 20: Call a static method (class method) without binding `self`.
    /// Static methods are called on the class (graph) itself, not on instances.
    pub(crate) fn call_static_method(&mut self, func: &Function, arg_values: &[Value]) -> Result<Value> {
        // Validate argument count
        if arg_values.len() != func.parameters.len() {
            return Err(GraphoidError::runtime(format!(
                "Static method '{}' expects {} arguments, but got {}",
                func.name.as_ref().unwrap_or(&"<anonymous>".to_string()),
                func.parameters.len(),
                arg_values.len()
            )));
        }

        // Push method onto call stack
        let method_name = func.name.as_ref().unwrap_or(&"<anonymous>".to_string()).clone();
        self.call_stack.push(method_name.clone());

        // Push function call onto the graph
        if let Some(ref func_id) = func.node_id {
            self.function_graph.borrow_mut().push_call(func_id.clone(), arg_values.to_vec());
        }

        // Static methods execute in the current environment directly.
        // This allows them to modify class-level state (e.g., Counter._count).
        // We only need to temporarily bind the parameters, then clean them up.

        // Save any existing bindings for parameter names (to restore after)
        let mut saved_params: Vec<(String, Option<Value>)> = Vec::new();
        for param in &func.parameters {
            let existing = self.env.get(&param.name).ok();
            saved_params.push((param.name.clone(), existing));
        }

        // Bind arguments - NO `self` binding for static methods
        for (param, value) in func.parameters.iter().zip(arg_values.iter()) {
            self.env.define(param.name.clone(), value.clone());
        }

        // Execute method body
        let mut return_value = Value::none();
        for stmt in &func.body {
            if let Some(ret_val) = self.eval_stmt(stmt)? {
                return_value = ret_val;
                break;
            }
        }

        // Restore original parameter bindings (or remove if they didn't exist)
        for (name, original) in saved_params {
            if let Some(val) = original {
                self.env.define(name, val);
            } else {
                self.env.remove_variable(&name);
            }
        }

        // Pop from call stack
        self.call_stack.pop();

        // Pop function call from graph
        self.function_graph.borrow_mut().pop_call(return_value.clone());

        Ok(return_value)
    }

    /// Internal implementation of call_graph_method with control over super context management.
    /// When `manage_super_context` is false, the caller is responsible for managing super_context_stack.
    pub(crate) fn call_graph_method_impl(&mut self, graph: &crate::values::Graph, func: &Function, arg_values: &[Value], object_expr: &Expr, manage_super_context: bool) -> Result<Value> {
        // Validate argument count
        if arg_values.len() != func.parameters.len() {
            return Err(GraphoidError::runtime(format!(
                "Method '{}' expects {} arguments, but got {}",
                func.name.as_ref().unwrap_or(&"<anonymous>".to_string()),
                func.parameters.len(),
                arg_values.len()
            )));
        }

        // Capture graph state before method execution for constraint checking
        // Use constrainable_node_ids() which includes CLG properties (for accurate constraint checking)
        // but excludes internal nodes like __methods__, __parent__, __self__
        let before_node_ids: std::collections::HashSet<String> = graph.constrainable_node_ids().into_iter().collect();
        let before_edge_count = graph.data_edge_list().len();

        // Push method onto call stack
        let method_name = func.name.as_ref().unwrap_or(&"<anonymous>".to_string()).clone();
        self.call_stack.push(method_name.clone());

        // Phase 15: Push graph variable name to method context stack for private method access
        let graph_var_name = if let Expr::Variable { name, .. } = object_expr {
            Some(name.clone())
        } else {
            None
        };
        if let Some(ref var_name) = graph_var_name {
            self.method_context_stack.push(var_name.clone());
        }

        // Phase 16: Push the graph onto super_context_stack for super call resolution
        // This tells super.method() which graph's parent to look at
        // Only do this if manage_super_context is true; super calls manage their own stack
        if manage_super_context {
            self.super_context_stack.push(graph.clone());
        }

        // Push self onto block_self_stack for implicit self in blocks
        // This allows closures called from within this method to access `self`
        self.block_self_stack.push(BlockSelfEntry {
            value: Value::graph(graph.clone()),
        });

        // Push function call onto the graph
        if let Some(ref func_id) = func.node_id {
            self.function_graph.borrow_mut().push_call(func_id.clone(), arg_values.to_vec());
        }

        // Save current environment first
        let saved_env_clone = self.env.clone();

        // For static methods, use CURRENT env as parent (late binding)
        // This allows static methods to reference the graph type by name
        // For instance methods, use the captured env (closures work as expected)
        let call_env = if func.is_static {
            Environment::with_parent(saved_env_clone)
        } else {
            let parent_env = func.env.borrow().clone();
            Environment::with_parent(parent_env)
        };

        // Save current environment and switch to call environment
        let saved_env = std::mem::replace(&mut self.env, call_env);

        // Bind `self` to the graph
        self.env.define("self".to_string(), Value::graph(graph.clone()));

        // IMPORTANT: For instance methods, bind the class name from the CURRENT environment.
        // This enables patterns like `instance = ClassName {}` inside methods, where methods
        // need access to the final class definition (with all methods attached).
        // Without this, methods would only see the class as it was when the method was defined,
        // which doesn't include the methods themselves.
        if !func.is_static {
            if let Some(type_name) = &graph.type_name {
                if let Ok(class_value) = saved_env.get(type_name) {
                    self.env.define(type_name.clone(), class_value);
                }
            }
        }

        // Bind parameters to argument values
        for (param, arg) in func.parameters.iter().zip(arg_values.iter()) {
            self.env.define(param.name.clone(), arg.clone());
        }

        // Execute function body
        let mut return_value = Value::none();
        let execution_result: Result<()> = (|| {
            for stmt in &func.body {
                if let Some(ret_val) = self.eval_stmt(stmt)? {
                    return_value = ret_val;
                    return Ok(());
                }
            }
            Ok(())
        })();

        // Enhance undefined variable errors with property suggestions
        let execution_result = execution_result.map_err(|err| {
            if let Some(var_name) = err.get_undefined_variable_name() {
                let suggestions = graph.suggest_similar_properties(&var_name);
                if !suggestions.is_empty() {
                    return GraphoidError::undefined_variable_with_suggestions(&var_name, &suggestions);
                }
            }
            err
        });

        // Get the (possibly modified) `self` before restoring environment
        let modified_self = self.env.get("self").ok();

        // Restore original environment
        self.env = saved_env;

        // Check method constraints before persisting changes
        if let Some(Value { kind: ValueKind::Graph(ref modified_graph), .. }) = modified_self {
            // Get the after state (use constrainable_node_ids for accurate constraint checking)
            let after_node_ids: std::collections::HashSet<String> = modified_graph.constrainable_node_ids().into_iter().collect();
            let after_edge_count = modified_graph.data_edge_list().len();

            // Check for constraint violations
            for rule_instance in &graph.rules {
                if rule_instance.spec.is_method_constraint() {
                    match &rule_instance.spec {
                        RuleSpec::ReadOnly => {
                            // Any change violates read_only
                            if before_node_ids != after_node_ids || before_edge_count != after_edge_count {
                                // Pop call stack before returning error
                                self.call_stack.pop();
                                if func.node_id.is_some() {
                                    self.function_graph.borrow_mut().pop_call(Value::none());
                                }
                                return Err(GraphoidError::runtime(format!(
                                    "Method '{}' violates :read_only constraint: graph was modified",
                                    method_name
                                )));
                            }
                        }
                        RuleSpec::NoNodeRemovals => {
                            // Check if any nodes were removed
                            let removed: Vec<_> = before_node_ids.difference(&after_node_ids).collect();
                            if !removed.is_empty() {
                                self.call_stack.pop();
                                if func.node_id.is_some() {
                                    self.function_graph.borrow_mut().pop_call(Value::none());
                                }
                                return Err(GraphoidError::runtime(format!(
                                    "Method '{}' violates :no_node_removals constraint: removed node(s) {:?}",
                                    method_name, removed
                                )));
                            }
                        }
                        RuleSpec::NoEdgeRemovals => {
                            // Check if edges were removed
                            if after_edge_count < before_edge_count {
                                self.call_stack.pop();
                                if func.node_id.is_some() {
                                    self.function_graph.borrow_mut().pop_call(Value::none());
                                }
                                return Err(GraphoidError::runtime(format!(
                                    "Method '{}' violates :no_edge_removals constraint: edges removed",
                                    method_name
                                )));
                            }
                        }
                        RuleSpec::CustomMethodConstraint { function, name } => {
                            // Call user-defined constraint function with (before_graph, after_graph)
                            // The function returns true if the operation is allowed
                            let before_graph_value = Value::graph(graph.clone());
                            let after_graph_value = Value::graph(modified_graph.clone());

                            let result = match &function.kind {
                                ValueKind::Function(constraint_func) => {
                                    self.call_function(constraint_func, &[before_graph_value, after_graph_value])
                                }
                                _ => {
                                    Err(GraphoidError::runtime(
                                        "Custom method constraint must be a function".to_string()
                                    ))
                                }
                            };

                            match result {
                                Ok(val) => {
                                    // Check if result is truthy (constraint passed)
                                    let is_allowed = match &val.kind {
                                        ValueKind::Boolean(b) => *b,
                                        ValueKind::None => false,
                                        ValueKind::Number(n) => *n != 0.0,
                                        _ => true,
                                    };

                                    if !is_allowed {
                                        self.call_stack.pop();
                                        if func.node_id.is_some() {
                                            self.function_graph.borrow_mut().pop_call(Value::none());
                                        }
                                        return Err(GraphoidError::runtime(format!(
                                            "Method '{}' violates custom constraint '{}': constraint returned false",
                                            method_name, name
                                        )));
                                    }
                                }
                                Err(e) => {
                                    self.call_stack.pop();
                                    if func.node_id.is_some() {
                                        self.function_graph.borrow_mut().pop_call(Value::none());
                                    }
                                    return Err(GraphoidError::runtime(format!(
                                        "Method '{}': custom constraint '{}' failed: {}",
                                        method_name, name, e
                                    )));
                                }
                            }
                        }
                        _ => {} // Other rules are not method constraints
                    }
                }
            }
        }

        // Persist mutations to `self` back to the original graph variable
        if let Some(modified_graph) = modified_self {
            // Only update if object_expr is a simple variable reference
            if let Expr::Variable { name, .. } = object_expr {
                // Try to set in environment first
                if self.env.set(name, modified_graph.clone()).is_err() {
                    // Variable not in environment - check if it's accessed via implicit self
                    // If so, update the property on the outer `self` graph
                    if let Ok(outer_self) = self.env.get("self") {
                        if let ValueKind::Graph(mut outer_graph) = outer_self.kind.clone() {
                            if outer_graph.has_node(name) {
                                // Update the property on the outer self (add_node preserves edges)
                                outer_graph.add_node(name.to_string(), modified_graph)?;
                                self.env.set("self", Value::graph(outer_graph))?;
                            }
                            // If it's not a property on outer self either, ignore
                            // (could be a read-only method call on a computed expression)
                        }
                    }
                }
            }
        }

        // Pop method from call stack
        self.call_stack.pop();

        // Phase 15: Pop method context stack
        if graph_var_name.is_some() {
            self.method_context_stack.pop();
        }

        // Phase 16: Pop super context stack (only if we pushed)
        if manage_super_context {
            self.super_context_stack.pop();
        }

        // Pop block_self_stack (we always push, so always pop)
        self.block_self_stack.pop();

        // Pop function from graph with return value
        if func.node_id.is_some() {
            self.function_graph.borrow_mut().pop_call(return_value.clone());
        }

        // Propagate errors
        execution_result?;

        Ok(return_value)
    }

    /// Apply behaviors to a value with executor context for function-based behaviors
    ///
    /// This method handles both standard behaviors (via Behavior trait) and
    /// function-based behaviors (CustomFunction, Conditional) that require
    /// executor context to call user functions.
    ///
    /// # Arguments
    /// * `value` - The value to transform
    /// * `rules` - The rules to apply (only transformation rules will be applied), in order
    ///
    /// # Returns
    /// The transformed value, or an error if any transformation fails
    pub fn apply_transformation_rules_with_context(
        &mut self,
        value: Value,
        rules: &[crate::graph::RuleInstance],
    ) -> Result<Value> {
        use crate::graph::RuleSpec;

        let mut current = value;

        for rule_instance in rules {
            // Skip non-transformation rules
            if !rule_instance.spec.is_transformation_rule() {
                continue;
            }

            match &rule_instance.spec {
                RuleSpec::CustomFunction { function } => {
                    // Extract function from Value
                    match &function.kind {
                        ValueKind::Function(func) => {
                            // Call function with executor context
                            current = self.call_function(func, &[current])?;
                        }
                        _ => {
                            return Err(GraphoidError::runtime(
                                "CustomFunction behavior requires a function value".to_string()
                            ));
                        }
                    }
                }
                RuleSpec::Conditional { condition, transform, fallback } => {
                    // Call condition predicate
                    let condition_func = match &condition.kind {
                        ValueKind::Function(f) => f,
                        _ => {
                            return Err(GraphoidError::runtime(
                                "Conditional behavior condition must be a function".to_string()
                            ));
                        }
                    };

                    let condition_result = self.call_function(condition_func, &[current.clone()])?;

                    // Check if condition is truthy
                    let is_truthy = match &condition_result.kind {
                        ValueKind::Boolean(b) => *b,
                        ValueKind::None => false,
                        ValueKind::Number(n) => *n != 0.0,
                        _ => true, // Non-false, non-none values are truthy
                    };

                    if is_truthy {
                        // Apply transform function
                        let transform_func = match &transform.kind {
                            ValueKind::Function(f) => f,
                            _ => {
                                return Err(GraphoidError::runtime(
                                    "Conditional behavior transform must be a function".to_string()
                                ));
                            }
                        };
                        current = self.call_function(transform_func, &[current])?;
                    } else if let Some(fallback_val) = fallback {
                        // Apply fallback function
                        let fallback_func = match &fallback_val.kind {
                            ValueKind::Function(f) => f,
                            _ => {
                                return Err(GraphoidError::runtime(
                                    "Conditional behavior fallback must be a function".to_string()
                                ));
                            }
                        };
                        current = self.call_function(fallback_func, &[current])?;
                    }
                    // else: no fallback, keep current value unchanged
                }
                _ => {
                    // Standard transformation rules use Rule trait's transform method
                    let rule = rule_instance.spec.instantiate();
                    current = rule.transform(&current)?;
                }
            }
        }

        Ok(current)
    }

    /// Compare two values using default ordering
    ///
    /// Returns std::cmp::Ordering for use with sort functions.
    ///
    /// Default ordering:
    /// - None < Boolean < Number < String < Symbol < List < Map < Graph < Function
    /// - Within same type: natural ordering (numeric, lexicographic, etc.)
    pub fn compare_values(&self, a: &Value, b: &Value) -> Result<std::cmp::Ordering> {
        use std::cmp::Ordering;

        match (&a.kind, &b.kind) {
            // Same types - compare naturally
            (ValueKind::None, ValueKind::None) => Ok(Ordering::Equal),
            (ValueKind::Boolean(a), ValueKind::Boolean(b)) => Ok(a.cmp(b)),
            (ValueKind::Number(a), ValueKind::Number(b)) => {
                // Handle NaN and infinities
                if a.is_nan() && b.is_nan() {
                    Ok(Ordering::Equal)
                } else if a.is_nan() {
                    Ok(Ordering::Greater)  // NaN sorts last
                } else if b.is_nan() {
                    Ok(Ordering::Less)
                } else {
                    Ok(a.partial_cmp(b).unwrap_or(Ordering::Equal))
                }
            }
            (ValueKind::String(a), ValueKind::String(b)) => Ok(a.cmp(b)),
            (ValueKind::Symbol(a), ValueKind::Symbol(b)) => Ok(a.cmp(b)),

            // Different types - use type ordering
            (ValueKind::None, _) => Ok(Ordering::Less),
            (_, ValueKind::None) => Ok(Ordering::Greater),
            (ValueKind::Boolean(_), ValueKind::Number(_)) => Ok(Ordering::Less),
            (ValueKind::Number(_), ValueKind::Boolean(_)) => Ok(Ordering::Greater),
            (ValueKind::Boolean(_), ValueKind::String(_)) => Ok(Ordering::Less),
            (ValueKind::String(_), ValueKind::Boolean(_)) => Ok(Ordering::Greater),
            (ValueKind::Number(_), ValueKind::String(_)) => Ok(Ordering::Less),
            (ValueKind::String(_), ValueKind::Number(_)) => Ok(Ordering::Greater),
            (ValueKind::Number(_), ValueKind::Symbol(_)) => Ok(Ordering::Less),
            (ValueKind::Symbol(_), ValueKind::Number(_)) => Ok(Ordering::Greater),
            (ValueKind::String(_), ValueKind::Symbol(_)) => Ok(Ordering::Less),
            (ValueKind::Symbol(_), ValueKind::String(_)) => Ok(Ordering::Greater),

            // Collections and complex types
            _ => Ok(Ordering::Equal),  // For now, complex types are equal
        }
    }

    /// Compare two values using a custom comparison function
    ///
    /// The function should return a number: < 0 (a < b), 0 (a == b), > 0 (a > b)
    pub fn compare_with_function(
        &mut self,
        a: &Value,
        b: &Value,
        func: &Function,
    ) -> Result<std::cmp::Ordering> {
        use std::cmp::Ordering;

        // Call comparison function with both values
        let result = self.call_function(func, &[a.clone(), b.clone()])?;

        // Convert result to Ordering
        match &result.kind {
            ValueKind::Number(n) => {
                if *n < 0.0 {
                    Ok(Ordering::Less)
                } else if *n > 0.0 {
                    Ok(Ordering::Greater)
                } else {
                    Ok(Ordering::Equal)
                }
            }
            _ => Err(GraphoidError::runtime(
                "Comparison function must return a number".to_string()
            )),
        }
    }

    /// Find the correct insertion point for a value in a sorted list
    ///
    /// Uses binary search to find where to insert a value to maintain sorted order.
    ///
    /// # Arguments
    /// * `values` - The sorted list of values
    /// * `new_value` - The value to insert
    /// * `compare_fn` - Optional custom comparison function
    ///
    /// # Returns
    /// The index where the value should be inserted
    pub fn find_insertion_point(
        &mut self,
        values: &[Value],
        new_value: &Value,
        compare_fn: &Option<Value>,
    ) -> Result<usize> {
        if values.is_empty() {
            return Ok(0);
        }

        // Binary search for insertion point
        let mut left = 0;
        let mut right = values.len();

        while left < right {
            let mid = left + (right - left) / 2;

            let ordering = match compare_fn.as_ref().map(|v| &v.kind) {
                Some(ValueKind::Function(func)) => {
                    self.compare_with_function(&values[mid], new_value, func)?
                }
                _ => self.compare_values(&values[mid], new_value)?,
            };

            match ordering {
                std::cmp::Ordering::Less => left = mid + 1,
                std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => right = mid,
            }
        }

        Ok(left)
    }

    /// Applies a named transformation to a value.
    /// Named transformations: double, square, negate, increment, decrement, etc.
    pub(crate) fn apply_named_transformation(&self, value: &Value, transform_name: &str) -> Result<Value> {
        match transform_name {
            "double" => {
                match &value.kind {
                    ValueKind::Number(n) => Ok(Value::number(n * 2.0)),
                    _ => Err(GraphoidError::runtime(format!(
                        "Transformation 'double' requires a number, got {}",
                        value.type_name()
                    ))),
                }
            }
            "square" => {
                match &value.kind {
                    ValueKind::Number(n) => Ok(Value::number(n * n)),
                    _ => Err(GraphoidError::runtime(format!(
                        "Transformation 'square' requires a number, got {}",
                        value.type_name()
                    ))),
                }
            }
            "negate" => {
                match &value.kind {
                    ValueKind::Number(n) => Ok(Value::number(-n)),
                    _ => Err(GraphoidError::runtime(format!(
                        "Transformation 'negate' requires a number, got {}",
                        value.type_name()
                    ))),
                }
            }
            "increment" | "inc" => {
                match &value.kind {
                    ValueKind::Number(n) => Ok(Value::number(n + 1.0)),
                    _ => Err(GraphoidError::runtime(format!(
                        "Transformation 'increment' requires a number, got {}",
                        value.type_name()
                    ))),
                }
            }
            "decrement" | "dec" => {
                match &value.kind {
                    ValueKind::Number(n) => Ok(Value::number(n - 1.0)),
                    _ => Err(GraphoidError::runtime(format!(
                        "Transformation 'decrement' requires a number, got {}",
                        value.type_name()
                    ))),
                }
            }
            _ => Err(GraphoidError::runtime(format!(
                "Unknown named transformation: '{}'",
                transform_name
            ))),
        }
    }

    /// Applies a named predicate to a value.
    /// Named predicates: even, odd, positive, negative, zero, etc.
    pub(crate) fn apply_named_predicate(&self, value: &Value, predicate_name: &str) -> Result<bool> {
        match predicate_name {
            "even" => {
                match &value.kind {
                    ValueKind::Number(n) => Ok((n % 2.0).abs() < 0.0001), // Handle floating point comparison
                    _ => Err(GraphoidError::runtime(format!(
                        "Predicate 'even' requires a number, got {}",
                        value.type_name()
                    ))),
                }
            }
            "odd" => {
                match &value.kind {
                    ValueKind::Number(n) => Ok((n % 2.0).abs() > 0.0001), // Handle floating point comparison
                    _ => Err(GraphoidError::runtime(format!(
                        "Predicate 'odd' requires a number, got {}",
                        value.type_name()
                    ))),
                }
            }
            "positive" | "pos" => {
                match &value.kind {
                    ValueKind::Number(n) => Ok(*n > 0.0),
                    _ => Err(GraphoidError::runtime(format!(
                        "Predicate 'positive' requires a number, got {}",
                        value.type_name()
                    ))),
                }
            }
            "negative" | "neg" => {
                match &value.kind {
                    ValueKind::Number(n) => Ok(*n < 0.0),
                    _ => Err(GraphoidError::runtime(format!(
                        "Predicate 'negative' requires a number, got {}",
                        value.type_name()
                    ))),
                }
            }
            "zero" => {
                match &value.kind {
                    ValueKind::Number(n) => Ok(n.abs() < 0.0001), // Handle floating point comparison
                    _ => Err(GraphoidError::runtime(format!(
                        "Predicate 'zero' requires a number, got {}",
                        value.type_name()
                    ))),
                }
            }
            _ => Err(GraphoidError::runtime(format!(
                "Unknown named predicate: '{}'",
                predicate_name
            ))),
        }
    }


    /// Gets a reference to the environment (for testing).
    pub fn env(&self) -> &Environment {
        &self.env
    }

    /// Gets a mutable reference to the environment (for testing).
    pub fn env_mut(&mut self) -> &mut Environment {
        &mut self.env
    }

    /// Gets the current call stack (for debugging and error reporting).
    pub fn call_stack(&self) -> &[String] {
        &self.call_stack
    }

    /// Loads a module from a file path or module name.
    /// Creates an isolated environment, executes the module, and returns a Module value.
    fn load_module(&mut self, module_path: &str, _alias: Option<&String>) -> Result<Value> {
        use std::fs;

        // Check for native modules first (before file resolution)
        if self.module_manager.is_native_module(module_path) {
            // Check if already cached
            let cache_key = format!("native:{}", module_path);
            if let Some(module) = self.module_manager.get_module(&cache_key) {
                return Ok(Value::module(module.clone()));
            }

            // Load native module
            if let Some((native_env, native_alias)) = self.module_manager.get_native_module_env(module_path) {
                let module = Module {
                    name: module_path.to_string(),
                    alias: native_alias,
                    namespace: native_env,
                    file_path: PathBuf::from(format!("<native:{}>", module_path)),
                    config: None,
                    private_symbols: std::collections::HashSet::new(),
                };

                // Cache native module
                self.module_manager.register_module(cache_key, module.clone());

                return Ok(Value::module(module));
            }
        }

        // Resolve the module path for file-based modules
        let resolved_path = if let Some(ref current) = self.current_file {
            self.module_manager.resolve_module_path(module_path, Some(current))?
        } else {
            self.module_manager.resolve_module_path(module_path, None)?
        };

        // Check if already loaded (cached)
        if let Some(module) = self.module_manager.get_module(&resolved_path.to_string_lossy().to_string()) {
            return Ok(Value::module(module.clone()));
        }

        // Check for circular dependency
        self.module_manager.check_circular(&resolved_path)?;

        // Begin loading
        self.module_manager.begin_loading(resolved_path.clone())?;

        // Read module source
        let source = fs::read_to_string(&resolved_path)?;

        // Create isolated environment for module
        let module_env = Environment::new();
        let mut module_executor = Executor::with_env(module_env);
        module_executor.set_current_file(Some(resolved_path.clone()));

        // Execute module source
        module_executor.execute_source(&source)?;

        // Extract module name and alias (from module declarations or filename)
        let module_name = if let Some(v) = module_executor.get_variable("__module_name__") {
            if let ValueKind::String(name) = &v.kind {
                name.clone()
            } else {
                resolved_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unnamed")
                    .to_string()
            }
        } else {
            // Use filename without extension as module name
            resolved_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unnamed")
                .to_string()
        };

        let module_alias = if let Some(v) = module_executor.get_variable("__module_alias__") {
            if let ValueKind::String(alias) = &v.kind {
                Some(alias.clone())
            } else {
                None
            }
        } else {
            None
        };

        // Extract module config from executor's config stack
        let module_config = Self::extract_module_config(&module_executor);

        // Create Module value
        let module = Module {
            name: module_name,
            alias: module_alias,
            namespace: module_executor.env.clone(),
            file_path: resolved_path.clone(),
            config: module_config,
            private_symbols: module_executor.private_symbols.clone(),  // Phase 10: Track private symbols
        };

        // Copy module's global functions to main executor (enables forward references and overloading)
        for (func_name, funcs) in module_executor.global_functions {
            self.global_functions
                .entry(func_name)
                .or_insert_with(Vec::new)
                .extend(funcs);
        }

        // Register module in manager
        self.module_manager.register_module(resolved_path.to_string_lossy().to_string(), module.clone());

        // End loading
        self.module_manager.end_loading(&resolved_path);

        Ok(Value::module(module))
    }

    /// Extract module configuration from executor's config stack
    fn extract_module_config(executor: &Executor) -> Option<ConfigScope> {
        let config = executor.config_stack.current();

        // Only create ConfigScope if there are non-default settings
        let has_custom_config = config.decimal_places.is_some()
            || config.error_mode != crate::execution::config::ErrorMode::Strict
            || config.bounds_checking != crate::execution::config::BoundsCheckingMode::Strict;

        if !has_custom_config {
            return None;
        }

        Some(ConfigScope {
            decimal_places: config.decimal_places.map(|p| p as u8),
            error_mode: Some(match config.error_mode {
                crate::execution::config::ErrorMode::Strict => ModuleErrorMode::Strict,
                crate::execution::config::ErrorMode::Lenient => ModuleErrorMode::Lenient,
                crate::execution::config::ErrorMode::Collect => ModuleErrorMode::Collect,
            }),
            bounds_checking: Some(match config.bounds_checking {
                crate::execution::config::BoundsCheckingMode::Strict => BoundsMode::Strict,
                crate::execution::config::BoundsCheckingMode::Lenient => BoundsMode::Lenient,
            }),
        })
    }

    /// Executes a load statement - merges file contents into current namespace
    fn execute_load(&mut self, file_path: &str) -> Result<()> {
        use std::fs;

        // Resolve the file path
        let resolved_path = if let Some(ref current) = self.current_file {
            self.module_manager.resolve_module_path(file_path, Some(current))?
        } else {
            self.module_manager.resolve_module_path(file_path, None)?
        };

        // Read file source
        let source = fs::read_to_string(&resolved_path)?;

        // Create temporary executor with fresh environment to execute the file
        let temp_env = Environment::new();
        let mut temp_executor = Executor::with_env(temp_env);
        temp_executor.set_current_file(Some(resolved_path.clone()));

        // Execute file source in temporary environment
        temp_executor.execute_source(&source)?;

        // Merge all variables from temporary environment into current environment
        let all_vars = temp_executor.env.get_all_bindings();
        for (name, value) in all_vars {
            // Skip internal module metadata variables
            if !name.starts_with("__") {
                self.env.define(name, value);
            }
        }

        Ok(())
    }

    /// Executes a try/catch/finally statement
    fn execute_try(
        &mut self,
        body: &[Stmt],
        catch_clauses: &[crate::ast::CatchClause],
        finally_block: &Option<Vec<Stmt>>,
    ) -> Result<Option<Value>> {
        // Try to execute the try body
        let try_result = self.execute_try_body(body);

        // Determine if we need to execute a catch clause
        // Don't use ? here - we need to run finally block regardless
        let catch_result = if let Err(ref error) = try_result {
            // Try to find a matching catch clause
            self.find_and_execute_catch(error, catch_clauses)
        } else {
            // No error, use try result
            try_result
        };

        // Always execute finally block if present
        if let Some(finally_stmts) = finally_block {
            for stmt in finally_stmts {
                self.eval_stmt(stmt)?;
            }
        }

        // Return the catch result (which may be an error)
        catch_result
    }

    /// Executes the try body and returns the result
    fn execute_try_body(&mut self, body: &[Stmt]) -> Result<Option<Value>> {
        let mut result = None;
        for stmt in body {
            match self.eval_stmt(stmt) {
                Ok(Some(val)) => {
                    result = Some(val);
                    break;
                }
                Ok(None) => {
                    // Statement executed successfully with no return value
                    continue;
                }
                Err(e) => {
                    // Error occurred - propagate it so execute_try can catch it
                    return Err(e);
                }
            }
        }
        Ok(result)
    }

    /// Finds a matching catch clause and executes it
    fn find_and_execute_catch(
        &mut self,
        error: &GraphoidError,
        catch_clauses: &[crate::ast::CatchClause],
    ) -> Result<Option<Value>> {
        // Extract error type from GraphoidError
        // First check if the error message contains a user-raised error type (e.g., "ValueError: message")
        let error_message = error.to_string();
        let error_type_name: String;
        let actual_message: String;

        if let Some(colon_pos) = error_message.find(':') {
            let potential_type = &error_message[..colon_pos];
            // Check if it's a known error type
            if matches!(potential_type, "ValueError" | "TypeError" | "IOError" | "NetworkError" | "ParseError" | "RuntimeError") {
                error_type_name = potential_type.to_string();
                actual_message = error_message[(colon_pos + 1)..].trim().to_string();
            } else {
                // Not a recognized error type prefix, use GraphoidError type
                error_type_name = match error {
                    GraphoidError::SyntaxError { .. } => "SyntaxError".to_string(),
                    GraphoidError::TypeError { .. } => "TypeError".to_string(),
                    GraphoidError::RuntimeError { .. } => "RuntimeError".to_string(),
                    GraphoidError::RuleViolation { .. } => "RuleViolation".to_string(),
                    GraphoidError::ModuleNotFound { .. } => "ModuleNotFound".to_string(),
                    GraphoidError::IOError { .. } => "IOError".to_string(),
                    GraphoidError::CircularDependency { .. } => "CircularDependency".to_string(),
                    GraphoidError::IoError(_) => "IoError".to_string(),
                    GraphoidError::ConfigError { .. } => "ConfigError".to_string(),
                    GraphoidError::LoopControl { .. } => "LoopControl".to_string(),
                };
                actual_message = error_message.clone();
            }
        } else {
            // No colon, use GraphoidError type
            error_type_name = match error {
                GraphoidError::SyntaxError { .. } => "SyntaxError".to_string(),
                GraphoidError::TypeError { .. } => "TypeError".to_string(),
                GraphoidError::RuntimeError { .. } => "RuntimeError".to_string(),
                GraphoidError::RuleViolation { .. } => "RuleViolation".to_string(),
                GraphoidError::ModuleNotFound { .. } => "ModuleNotFound".to_string(),
                GraphoidError::IOError { .. } => "IOError".to_string(),
                GraphoidError::CircularDependency { .. } => "CircularDependency".to_string(),
                GraphoidError::IoError(_) => "IoError".to_string(),
                GraphoidError::ConfigError { .. } => "ConfigError".to_string(),
                    GraphoidError::LoopControl { .. } => "LoopControl".to_string(),
            };
            actual_message = error_message.clone();
        }

        // Search for a matching catch clause
        for catch_clause in catch_clauses {
            // Check if this catch clause matches the error type
            let matches = if let Some(ref expected_type) = catch_clause.error_type {
                expected_type == &error_type_name
            } else {
                // Catch-all clause (no type specified)
                true
            };

            if matches {
                // Create a child scope for the catch block
                // This ensures that variables defined in catch don't leak to outer scope,
                // but modifications to existing variables (via set()) persist to parent scope
                let parent_env_clone = self.env.clone();
                self.env = Environment::with_parent(self.env.clone());

                // Bind error to variable if specified (in the catch scope)
                if let Some(ref var_name) = catch_clause.variable {
                    // Extract position from error
                    let error_position = error.position();

                    // Create an Error object from the GraphoidError with call stack
                    let error_obj = ErrorObject::with_stack_trace(
                        error_type_name.clone(),
                        actual_message.clone(),
                        self.current_file.as_ref().map(|p| p.to_string_lossy().to_string()),
                        error_position.line,
                        error_position.column,
                        self.call_stack.clone(),
                    );
                    self.env.define(var_name.clone(), Value::error(error_obj));
                }

                // Execute catch body in the child scope
                let mut result = None;
                for stmt in &catch_clause.body {
                    if let Some(val) = self.eval_stmt(stmt)? {
                        result = Some(val);
                        break;
                    }
                }

                // Extract the modified parent environment from the child
                // The parent field contains any modifications made via set()
                if let Some(boxed_parent) = self.env.take_parent() {
                    self.env = *boxed_parent;
                } else {
                    // This shouldn't happen since we just created a child with a parent
                    self.env = parent_env_clone;
                }

                return Ok(result);
            }
        }

        // No matching catch clause found - re-throw the error
        Err(error.clone())
    }

    /// Check if a value or any of its nested elements are frozen
    fn check_has_frozen(&self, value: &Value) -> bool {
        // If the value itself is frozen, return true
        if value.is_frozen() {
            return true;
        }

        // Check nested elements
        match &value.kind {
            ValueKind::List(list) => {
                // Check if any list element is frozen
                for i in 0..list.len() {
                    if let Some(elem) = list.get(i) {
                        if self.check_has_frozen(elem) {
                            return true;
                        }
                    }
                }
                false
            }
            ValueKind::Map(hash) => {
                // Check if any map value is frozen
                for key in hash.keys() {
                    if let Some(val) = hash.get(&key) {
                        if self.check_has_frozen(&val) {
                            return true;
                        }
                    }
                }
                false
            }
            _ => false, // Primitives don't have nested elements
        }
    }

    /// Generate detailed freeze count information for a value
    ///
    /// # Arguments
    /// * `value` - The value to analyze
    /// * `deep` - If true, recursively counts through entire tree; if false, counts immediate children only
    ///
    /// # Returns
    /// Hash with keys:
    /// - "has_frozen": boolean indicating if any frozen elements exist
    /// - "frozen_count": total number of frozen elements
    /// - "frozen_collections": number of frozen collections (lists, maps, graphs)
    /// - "frozen_primitives": number of frozen primitives (numbers, strings, etc.)
    fn eval_has_frozen_count(&self, value: &Value, deep: bool) -> Result<Value> {
        let mut frozen_count = 0;
        let mut frozen_collections = 0;
        let mut frozen_primitives = 0;

        // Count with specified mode (shallow by default, deep if requested)
        self.count_frozen(value, &mut frozen_count, &mut frozen_collections, &mut frozen_primitives, deep);

        // Create result hash
        let mut result = Hash::new();
        result.insert("has_frozen".to_string(), Value::boolean(frozen_count > 0)).unwrap();
        result.insert("frozen_count".to_string(), Value::number(frozen_count as f64)).unwrap();
        result.insert("frozen_collections".to_string(), Value::number(frozen_collections as f64)).unwrap();
        result.insert("frozen_primitives".to_string(), Value::number(frozen_primitives as f64)).unwrap();

        Ok(Value::map(result))
    }

    /// Count frozen elements with optional recursive mode
    ///
    /// By default, counts immediate children only (shallow mode).
    /// This is usually what you want: "how many of my direct children are frozen?"
    ///
    /// With recursive=true, counts all descendants at any depth.
    /// Useful when you need total count across entire tree.
    fn count_frozen(&self, value: &Value, total: &mut usize, collections: &mut usize, primitives: &mut usize, recursive: bool) {
        match &value.kind {
            ValueKind::List(list) => {
                for i in 0..list.len() {
                    if let Some(elem) = list.get(i) {
                        if elem.is_frozen() {
                            *total += 1;
                            match &elem.kind {
                                ValueKind::List(_) | ValueKind::Map(_) | ValueKind::Graph(_) => {
                                    *collections += 1;
                                }
                                _ => {
                                    *primitives += 1;
                                }
                            }
                        }
                        // Recursively count in child elements if requested
                        if recursive {
                            self.count_frozen(elem, total, collections, primitives, recursive);
                        }
                    }
                }
            }
            ValueKind::Map(hash) => {
                for key in hash.keys() {
                    if let Some(val) = hash.get(&key) {
                        if val.is_frozen() {
                            *total += 1;
                            match &val.kind {
                                ValueKind::List(_) | ValueKind::Map(_) | ValueKind::Graph(_) => {
                                    *collections += 1;
                                }
                                _ => {
                                    *primitives += 1;
                                }
                            }
                        }
                        // Recursively count in child values if requested
                        if recursive {
                            self.count_frozen(&val, total, collections, primitives, recursive);
                        }
                    }
                }
            }
            _ => {} // Primitives don't have children
        }
    }

    /// Match a graph pattern against a graph and return all matches.
    /// Returns a vector of variable bindings where each binding maps variable names to node IDs.
    pub(crate) fn match_pattern(
        &self,
        graph: &crate::values::Graph,
        pattern: &crate::ast::GraphPattern,
    ) -> Result<Vec<std::collections::HashMap<String, String>>> {
        use std::collections::HashMap;

        let mut all_matches = Vec::new();

        // Pattern must have at least one node
        if pattern.nodes.is_empty() {
            return Ok(all_matches);
        }

        // Start matching from the first node in the pattern
        let first_pattern_node = &pattern.nodes[0];

        // Try each node in the graph as a potential match for the first pattern node
        for node_id in graph.nodes.keys() {
            // Check if this node matches the pattern's type constraint
            if !self.node_matches_type(graph, node_id, first_pattern_node)? {
                continue;
            }

            let mut bindings = HashMap::new();
            bindings.insert(first_pattern_node.variable.clone(), node_id.clone());

            // Collect ALL possible extensions from this starting node
            self.extend_pattern_match_all(graph, pattern, bindings, 0, &mut all_matches)?;
        }

        Ok(all_matches)
    }

    /// Find all paths from a node within a given hop range (for variable-length paths).
    /// Returns a vector of (destination_node_id, path_length) tuples.
    fn find_variable_length_paths(
        &self,
        graph: &crate::values::Graph,
        from_id: &str,
        edge_type: Option<&str>,
        min_hops: usize,
        max_hops: usize,
    ) -> Vec<(String, usize)> {
        use std::collections::{VecDeque, HashSet};

        let mut results = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        // BFS: queue contains (current_node_id, current_depth)
        queue.push_back((from_id.to_string(), 0));

        while let Some((current_id, depth)) = queue.pop_front() {
            // If we've exceeded max hops, stop exploring this path
            if depth >= max_hops {
                // Check if we're at exactly max_hops and should record this node
                if depth == max_hops && depth >= min_hops {
                    results.push((current_id.clone(), depth));
                }
                continue;
            }

            // Get the current node
            let current_node = match graph.nodes.get(&current_id) {
                Some(node) => node,
                None => continue,
            };

            // Explore neighbors
            for (neighbor_id, edge_info) in &current_node.neighbors {
                // Check edge type if specified
                if let Some(required_type) = edge_type {
                    if edge_info.edge_type != required_type {
                        continue;
                    }
                }

                let new_depth = depth + 1;

                // Record this node if it's within the valid hop range
                if new_depth >= min_hops && new_depth <= max_hops {
                    results.push((neighbor_id.clone(), new_depth));
                }

                // Continue exploring if we haven't exceeded max_hops
                // Use a state key that includes path to allow multiple visits at different depths
                let state_key = (neighbor_id.clone(), new_depth);
                if !visited.contains(&state_key) && new_depth < max_hops {
                    visited.insert(state_key);
                    queue.push_back((neighbor_id.clone(), new_depth));
                }
            }
        }

        results
    }

    /// Process edges from a given edge collection, checking type and binding constraints.
    /// This helper method eliminates duplication between forward and backward edge traversal.
    fn process_edges_for_pattern(
        &self,
        graph: &crate::values::Graph,
        pattern: &crate::ast::GraphPattern,
        edges: &std::collections::HashMap<String, crate::values::graph::EdgeInfo>,
        edge_pattern: &crate::ast::PatternEdge,
        next_node_pattern: &crate::ast::PatternNode,
        bindings: &std::collections::HashMap<String, String>,
        edge_index: usize,
        all_matches: &mut Vec<std::collections::HashMap<String, String>>,
    ) -> Result<()> {
        for (to_id, edge_info) in edges {
            // Check if edge type matches (if specified in pattern)
            if let Some(ref pattern_edge_type) = edge_pattern.edge_type {
                if edge_info.edge_type != *pattern_edge_type {
                    continue;
                }
            }

            // Check if target node matches the pattern's type constraint
            if !self.node_matches_type(graph, to_id, next_node_pattern)? {
                continue;
            }

            // Check if we've already bound this variable
            if let Some(existing_binding) = bindings.get(&next_node_pattern.variable) {
                // Variable already bound - check if it matches
                if existing_binding != to_id {
                    continue;  // Doesn't match, try next edge
                }
                // Matches existing binding - continue with same bindings
                self.extend_pattern_match_all(graph, pattern, bindings.clone(), edge_index + 1, all_matches)?;
            } else {
                // Bind this variable and continue
                let mut new_bindings = bindings.clone();
                new_bindings.insert(next_node_pattern.variable.clone(), to_id.to_string());
                self.extend_pattern_match_all(graph, pattern, new_bindings, edge_index + 1, all_matches)?;
            }
        }

        Ok(())
    }

    /// Recursively extend a partial pattern match, collecting ALL possible matches.
    /// This version collects all matches instead of returning after finding the first one.
    fn extend_pattern_match_all(
        &self,
        graph: &crate::values::Graph,
        pattern: &crate::ast::GraphPattern,
        bindings: std::collections::HashMap<String, String>,
        edge_index: usize,
        all_matches: &mut Vec<std::collections::HashMap<String, String>>,
    ) -> Result<()> {
        // If we've matched all edges, we have a complete match
        if edge_index >= pattern.edges.len() {
            all_matches.push(bindings);
            return Ok(());
        }

        let edge_pattern = &pattern.edges[edge_index];
        let next_node_pattern = &pattern.nodes[edge_index + 1];

        // Get the source node from bindings
        let from_id = bindings.get(&pattern.nodes[edge_index].variable)
            .ok_or_else(|| GraphoidError::runtime(
                "Internal error: missing node binding in pattern match".to_string()
            ))?;

        // Check if this is a variable-length path
        use crate::ast::EdgeLength;
        match &edge_pattern.length {
            EdgeLength::Variable { min, max } => {
                // Find all nodes reachable within min..max hops
                let reachable = self.find_variable_length_paths(
                    graph,
                    from_id,
                    edge_pattern.edge_type.as_deref(),
                    *min,
                    *max,
                );

                // For each reachable node, try to continue the pattern match
                for (to_id, _path_length) in reachable {
                    // Check if target node matches the pattern's type constraint
                    if !self.node_matches_type(graph, &to_id, next_node_pattern)? {
                        continue;
                    }

                    // Check if we've already bound this variable
                    if let Some(existing_binding) = bindings.get(&next_node_pattern.variable) {
                        // Variable already bound - check if it matches
                        if existing_binding != &to_id {
                            continue;  // Doesn't match, try next path
                        }
                        // Matches existing binding - continue with same bindings
                        self.extend_pattern_match_all(graph, pattern, bindings.clone(), edge_index + 1, all_matches)?;
                    } else {
                        // Bind this variable and continue
                        let mut new_bindings = bindings.clone();
                        new_bindings.insert(next_node_pattern.variable.clone(), to_id.clone());
                        self.extend_pattern_match_all(graph, pattern, new_bindings, edge_index + 1, all_matches)?;
                    }
                }

                return Ok(());
            }
            EdgeLength::Fixed => {
                // Original fixed-length edge logic
            }
        }

        // Get the source node
        let from_node = graph.nodes.get(from_id)
            .ok_or_else(|| GraphoidError::runtime(
                "Internal error: node not found in graph".to_string()
            ))?;

        // Check direction to determine which edges to explore
        use crate::ast::EdgeDirection;
        let check_forward = match &edge_pattern.direction {
            EdgeDirection::Directed => true,
            EdgeDirection::Bidirectional => true,
        };
        let check_backward = match &edge_pattern.direction {
            EdgeDirection::Directed => false,
            EdgeDirection::Bidirectional => true,
        };

        // Process edges in forward direction
        if check_forward {
            self.process_edges_for_pattern(
                graph,
                pattern,
                &from_node.neighbors,
                edge_pattern,
                next_node_pattern,
                &bindings,
                edge_index,
                all_matches,
            )?;
        }

        // Process edges in backward direction
        if check_backward {
            self.process_edges_for_pattern(
                graph,
                pattern,
                &from_node.predecessors,
                edge_pattern,
                next_node_pattern,
                &bindings,
                edge_index,
                all_matches,
            )?;
        }

        Ok(())
    }

    /// Recursively extend a partial pattern match.
    /// edge_index indicates which edge we're trying to match next.

    /// Check if a node matches a pattern node's type constraint
    fn node_matches_type(
        &self,
        graph: &crate::values::Graph,
        node_id: &str,
        pattern_node: &crate::ast::PatternNode,
    ) -> Result<bool> {
        // If pattern has a type constraint, check it
        if let Some(ref required_type) = pattern_node.node_type {
            let node = graph.nodes.get(node_id)
                .ok_or_else(|| GraphoidError::runtime(
                    "Internal error: node not found in graph".to_string()
                ))?;

            // Check if node's type matches the required type
            match &node.node_type {
                Some(actual_type) => Ok(actual_type == required_type),
                None => Ok(false),  // Node has no type, doesn't match
            }
        } else {
            // No type constraint, any node matches
            Ok(true)
        }
    }

    /// Evaluate a match expression
    fn eval_match(
        &mut self,
        value_expr: &Expr,
        arms: &[crate::ast::MatchArm],
        _position: &SourcePosition,
    ) -> Result<Value> {
        // Evaluate the value to match against
        let value = self.eval_expr(value_expr)?;

        // Try each arm in order
        for arm in arms {
            // Try to match the pattern
            if let Some(bindings) = self.match_expr_pattern(&arm.pattern, &value)? {
                // Pattern matched! Create a new child environment with bindings
                let mut child_env = Environment::with_parent(self.env.clone());
                for (var_name, var_value) in bindings {
                    child_env.define(var_name, var_value);
                }

                // Swap environments temporarily
                let old_env = std::mem::replace(&mut self.env, child_env);

                // Evaluate the arm body
                let result = self.eval_expr(&arm.body);

                // Restore the previous environment
                self.env = old_env;

                return result;
            }
        }

        // No pattern matched - this is an error
        Err(GraphoidError::runtime(format!(
            "No match arm matched value: {:?}",
            value
        )))
    }

    /// Try to match a pattern against a value (for match expressions)
    /// Returns Some(bindings) if match succeeds, None if it fails
    fn match_expr_pattern(
        &self,
        pattern: &crate::ast::MatchPattern,
        value: &Value,
    ) -> Result<Option<Vec<(String, Value)>>> {
        use crate::ast::{MatchPattern, LiteralValue};
        use crate::values::ValueKind;

        match pattern {
            // Wildcard matches everything
            MatchPattern::Wildcard => Ok(Some(Vec::new())),

            // Variable binding matches everything and binds the value
            MatchPattern::Variable(name) => {
                // Special case: "_" is wildcard, not a binding
                if name == "_" {
                    Ok(Some(Vec::new()))
                } else {
                    Ok(Some(vec![(name.clone(), value.clone())]))
                }
            }

            // Literal pattern: must match exactly
            MatchPattern::Literal(lit) => {
                let matches = match (lit, &value.kind) {
                    (LiteralValue::Number(n1), ValueKind::Number(n2)) => (n1 - n2).abs() < f64::EPSILON,
                    (LiteralValue::String(s1), ValueKind::String(s2)) => s1 == s2,
                    (LiteralValue::Boolean(b1), ValueKind::Boolean(b2)) => b1 == b2,
                    (LiteralValue::None, ValueKind::None) => true,
                    _ => false,
                };
                Ok(if matches { Some(Vec::new()) } else { None })
            }

            // List pattern
            MatchPattern::List { elements, rest_name } => {
                match &value.kind {
                    ValueKind::List(list_val) => {
                        let list_elements = list_val.to_vec();

                        // Check length compatibility
                        if rest_name.is_some() {
                            // With rest pattern, we need at least as many elements as non-rest patterns
                            if list_elements.len() < elements.len() {
                                return Ok(None);
                            }
                        } else {
                            // Without rest, must match exactly
                            if list_elements.len() != elements.len() {
                                return Ok(None);
                            }
                        }

                        // Match each fixed element
                        let mut all_bindings = Vec::new();
                        for (i, pat) in elements.iter().enumerate() {
                            if let Some(bindings) = self.match_expr_pattern(pat, &list_elements[i])? {
                                all_bindings.extend(bindings);
                            } else {
                                return Ok(None);
                            }
                        }

                        // Handle rest pattern if present
                        if let Some(rest_var) = rest_name {
                            if rest_var != "_" {
                                // Bind the remaining elements to the rest variable
                                let rest_elements: Vec<Value> = list_elements[elements.len()..].to_vec();
                                let rest_list = List::from_vec(rest_elements);
                                all_bindings.push((rest_var.clone(), Value {
                                    kind: ValueKind::List(rest_list),
                                    frozen: false,
                                }));
                            }
                            // If rest_var is "_", don't bind (anonymous rest)
                        }

                        Ok(Some(all_bindings))
                    }
                    _ => Ok(None),  // Not a list
                }
            }
        }
    }

    /// Enable output capture for exec() - print statements go to buffer instead of stdout
    pub fn enable_output_capture(&mut self) {
        self.output_capture_enabled = true;
        self.output_buffer.clear();
    }

    /// Get captured output and clear the buffer
    pub fn get_captured_output(&mut self) -> String {
        let output = self.output_buffer.clone();
        self.output_buffer.clear();
        output
    }

    /// Capture print output (called by print builtin when capture is enabled)
    fn capture_print(&mut self, text: &str) {
        if !self.output_buffer.is_empty() {
            self.output_buffer.push('\n');
        }
        self.output_buffer.push_str(text);
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

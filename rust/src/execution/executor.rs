use crate::ast::{AssignmentTarget, BinaryOp, Expr, LiteralValue, Parameter, Stmt, UnaryOp};
use crate::error::{GraphoidError, Result, SourcePosition};
use crate::execution::Environment;
use crate::execution::config::{ConfigStack, ErrorMode};
use crate::execution::error_collector::ErrorCollector;
use crate::execution::function_graph::FunctionGraph;
use crate::execution::module_manager::{ModuleManager, Module};
use crate::values::{Function, Value, ValueKind, List, Hash, ErrorObject};
use crate::graph::{RuleSpec, RuleInstance};
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;

/// The executor evaluates AST nodes and produces values.
pub struct Executor {
    env: Environment,
    call_stack: Vec<String>,
    module_manager: ModuleManager,
    current_file: Option<PathBuf>,
    pub config_stack: ConfigStack,
    pub precision_stack: Vec<Option<usize>>,
    pub error_collector: ErrorCollector,
    /// Global function graph tracking all function definitions and calls
    pub function_graph: Rc<RefCell<FunctionGraph>>,
    /// Global function table (for recursion support)
    global_functions: HashMap<String, Function>,
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
    fn symbol_to_rule_spec(symbol: &str, param: Option<f64>) -> Result<RuleSpec> {
        match (symbol, param) {
            ("no_cycles", None) => Ok(RuleSpec::NoCycles),
            ("single_root", None) => Ok(RuleSpec::SingleRoot),
            ("connected", None) => Ok(RuleSpec::Connected),
            ("binary_tree", None) => Ok(RuleSpec::BinaryTree),
            ("no_dups" | "no_duplicates", None) => Ok(RuleSpec::NoDuplicates),
            ("max_degree", Some(n)) => Ok(RuleSpec::MaxDegree(n as usize)),
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
                        // If not in environment, check global functions table
                        if let Some(func) = self.global_functions.get(name) {
                            Ok(Value::function(func.clone()))
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
            Expr::Graph { config, .. } => self.eval_graph(config),
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
        }
    }

    /// Executes a statement.
    /// Returns Ok(None) for normal statement execution.
    /// Returns Ok(Some(value)) when a return statement is executed.
    pub fn eval_stmt(&mut self, stmt: &Stmt) -> Result<Option<Value>> {
        match stmt {
            Stmt::VariableDecl {
                name,
                value,
                ..
            } => {
                let val = self.eval_expr(value)?;
                self.env.define(name.clone(), val);
                Ok(None)
            }
            Stmt::Assignment { target, value, .. } => {
                let val = self.eval_expr(value)?;
                match target {
                    AssignmentTarget::Variable(name) => {
                        // Try to update existing variable, or create new one if it doesn't exist
                        if self.env.exists(name) {
                            self.env.set(name, val)?;
                        } else {
                            self.env.define(name.clone(), val);
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
                }
            }
            Stmt::FunctionDecl {
                name,
                params,
                body,
                pattern_clauses,
                ..
            } => {
                // Extract parameter names
                let param_names: Vec<String> = params.iter().map(|p| p.name.clone()).collect();

                // Create function value
                let mut func = Function {
                    name: Some(name.clone()),
                    params: param_names,
                    parameters: params.clone(),
                    body: body.clone(),
                    pattern_clauses: pattern_clauses.clone(),
                    env: Rc::new(RefCell::new(self.env.clone())),
                    node_id: None,
                };

                // Register function in the function graph and store its node_id
                let node_id = self.function_graph.borrow_mut().register_function(func.clone());
                func.node_id = Some(node_id);

                // Store in global functions table (for recursion support)
                self.global_functions.insert(name.clone(), func.clone());

                // Store function in environment
                self.env.define(name.clone(), Value::function(func));
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
                    for stmt in body {
                        if let Some(val) = self.eval_stmt(stmt)? {
                            // Return statement in loop body
                            return Ok(Some(val));
                        }
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
                    other => {
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

                // Determine the binding name (alias or module name)
                let binding_name = alias.as_ref().unwrap_or(module);

                // Bind the module to the environment
                self.env.define(binding_name.clone(), module_value);
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
            Stmt::Load { .. } => {
                // Load statement - inline file contents into current scope
                // TODO: Implement in Day 5
                Err(GraphoidError::runtime("Load statement not yet implemented".to_string()))
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
            _ => Err(GraphoidError::runtime(format!(
                "Unsupported statement type: {:?}",
                stmt
            ))),
        }
    }

    /// Evaluates a literal value.
    fn eval_literal(&self, lit: &LiteralValue) -> Result<Value> {
        match lit {
            LiteralValue::Number(n) => Ok(Value::number(*n)),
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
                _ => Err(GraphoidError::type_error("number", val.type_name())),
            },
            UnaryOp::Not => Ok(Value::boolean(!val.is_truthy())),
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
    fn eval_graph(&mut self, config: &[(String, Expr)]) -> Result<Value> {
        use crate::values::{Graph, GraphType};

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
        };

        // Register lambda in the function graph and store its node_id
        let node_id = self.function_graph.borrow_mut().register_function(func.clone());
        func.node_id = Some(node_id);

        Ok(Value::function(func))
    }

    /// Evaluates a block expression (used in lambda bodies).
    /// Returns the value of the last expression, or none if the block is empty or only has statements.
    fn eval_block(&mut self, statements: &[Stmt]) -> Result<Value> {
        // Execute all statements in the block
        for stmt in statements {
            // Execute the statement and check for returns
            if let Some(return_value) = self.eval_stmt(stmt)? {
                return Ok(return_value);
            }
        }

        // No explicit return, return none
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
                    other => {
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
                    other => {
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
            ValueKind::Graph(ref graph) => {
                // Index must be a string for graphs (node ID)
                let node_id = match &index_value.kind {
                    ValueKind::String(s) => s,
                    other => {
                        return Err(GraphoidError::type_error(
                            "string",
                            index_value.type_name(),
                        ));
                    }
                };

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

    /// Helper to evaluate arguments (positional only for now).
    /// Named arguments in method calls are not yet supported.
    fn eval_arguments(&mut self, args: &[crate::ast::Argument]) -> Result<Vec<Value>> {
        use crate::ast::Argument;
        let mut arg_values = Vec::new();
        for arg in args {
            match arg {
                Argument::Positional(expr) => {
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
        // Check for static method calls on type identifiers (e.g., list.generate)
        if let Expr::Variable { name, .. } = object {
            if name == "list" {
                // Evaluate argument expressions
                let arg_values = self.eval_arguments(args)?;
                return self.eval_list_static_method(method, &arg_values);
            }
        }

        // Evaluate the object once
        let object_value = self.eval_expr(object)?;

        // Check for module member access (e.g., module.function(args) or module.variable)
        if let ValueKind::Module(ref module) = &object_value.kind {
            // Look up the member in the module's namespace
            let member = module.namespace.get(method)?;

            // If it's a function, call it with args
            if let ValueKind::Function(func) = &member.kind {
                // Evaluate argument expressions
                let arg_values = self.eval_arguments(args)?;
                return self.call_function(&func, &arg_values);
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

    /// Applies a method to a value (helper to avoid duplication).
    fn apply_method_to_value(&mut self, value: Value, method: &str, args: &[Value], object_expr: &Expr) -> Result<Value> {
        match &value.kind {
            ValueKind::List(list) => self.eval_list_method(&list, method, args),
            ValueKind::Map(hash) => self.eval_map_method(&hash, method, args),
            ValueKind::Graph(graph) => self.eval_graph_method(graph.clone(), method, args, object_expr),
            ValueKind::String(ref s) => self.eval_string_method(s, method, args),
            ValueKind::Error(ref err) => self.eval_error_method(err, method, args),
            other => Err(GraphoidError::runtime(format!(
                "Type '{}' does not have method '{}'",
                value.type_name(),
                method
            ))),
        }
    }

    /// Evaluates static methods on the list type (e.g., list.generate, list.upto).
    fn eval_list_static_method(&mut self, method: &str, args: &[Value]) -> Result<Value> {
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
                    other => {
                        return Err(GraphoidError::type_error("number", args[0].type_name()));
                    }
                };

                let end = match &args[1].kind {
                    ValueKind::Number(n) => *n,
                    other => {
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
                    other => {
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
                    other => {
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

    /// Evaluates a method call on a list.
    fn eval_list_method(&mut self, list: &List, method: &str, args: &[Value]) -> Result<Value> {
        let elements = list.to_vec();
        match method {
            "size" | "length" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'size'/'length' expects 0 arguments, but got {}",
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
                    other => {
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
                    other => {
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
                    other => {
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
                    other => {
                        return Err(GraphoidError::type_error(
                            "number",
                            args[0].type_name(),
                        ));
                    }
                };

                let end_idx = match &args[1].kind {
                    ValueKind::Number(n) => *n as i64,
                    other => {
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
                        other => {
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
                // add_rule(rule_symbol) or add_rule(rule_symbol, param)
                // Handles BOTH validation rules AND transformation rules (behaviors)
                if args.is_empty() || args.len() > 2 {
                    return Err(GraphoidError::runtime(format!(
                        "add_rule() expects 1 or 2 arguments, but got {}",
                        args.len()
                    )));
                }

                // Get rule symbol
                let rule_symbol = match &args[0].kind {
                    ValueKind::Symbol(name) => name.as_str(),
                    other => {
                        return Err(GraphoidError::runtime(format!(
                            "add_rule() expects a symbol, got {}",
                            args[0].type_name()
                        )));
                    }
                };

                // Clone list
                let mut new_list = list.clone();

                // Try to parse rule from symbol (handles both validation and transformation rules)
                // Get optional parameter
                let param = if args.len() == 2 {
                    match &args[1].kind {
                        ValueKind::Number(n) => Some(*n),
                        other => {
                            return Err(GraphoidError::runtime(format!(
                                "add_rule() parameter must be a number, got {}",
                                args[1].type_name()
                            )));
                        }
                    }
                } else {
                    None
                };

                // Convert to RuleSpec
                let rule_spec = Self::symbol_to_rule_spec(rule_symbol, param)?;

                // Add validation rule, return
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
                    other => {
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
                        other => {
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
                    other => {
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
                    other => {
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
                    other => {
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
                    other => {
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
                    other => {
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

    /// Evaluates a method call on a map.
    fn eval_map_method(&mut self, hash: &Hash, method: &str, args: &[Value]) -> Result<Value> {
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
            "size" => {
                // Return number of entries
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(
                        "size() takes no arguments".to_string()
                    ));
                }
                Ok(Value::number(hash.len() as f64))
            }
            "add_rule" => {
                // add_rule(rule_symbol) or add_rule(rule_symbol, param)
                // Handles BOTH validation rules AND transformation rules (behaviors)
                if args.is_empty() || args.len() > 2 {
                    return Err(GraphoidError::runtime(format!(
                        "add_rule() expects 1 or 2 arguments, but got {}",
                        args.len()
                    )));
                }

                // Get rule symbol
                let rule_symbol = match &args[0].kind {
                    ValueKind::Symbol(name) => name.as_str(),
                    other => {
                        return Err(GraphoidError::runtime(format!(
                            "add_rule() expects a symbol, got {}",
                            args[0].type_name()
                        )));
                    }
                };

                // Clone hash
                let mut new_hash = hash.clone();

                // Try to parse rule from symbol (handles both validation and transformation rules)
                // Get optional parameter
                let param = if args.len() == 2 {
                    match &args[1].kind {
                        ValueKind::Number(n) => Some(*n),
                        other => {
                            return Err(GraphoidError::runtime(format!(
                                "add_rule() parameter must be a number, got {}",
                                args[1].type_name()
                            )));
                        }
                    }
                } else {
                    None
                };

                // Convert to RuleSpec
                let rule_spec = Self::symbol_to_rule_spec(rule_symbol, param)?;

                // Add validation rule, return
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
                    other => {
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
                        other => {
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
            _ => Err(GraphoidError::runtime(format!(
                "Map does not have method '{}'",
                method
            ))),
        }
    }

    /// Evaluates a method call on a string.
    fn eval_string_method(&self, s: &str, method: &str, args: &[Value]) -> Result<Value> {
        match method {
            "length" | "size" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "String method '{}' takes no arguments, but got {}",
                        method,
                        args.len()
                    )));
                }
                Ok(Value::number(s.len() as f64))
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
                    other => {
                        return Err(GraphoidError::type_error("number", args[0].type_name()));
                    }
                };
                let end = match &args[1].kind {
                    ValueKind::Number(n) => *n as usize,
                    other => {
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
                    other => {
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
                    other => {
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
                    other => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                Ok(Value::boolean(s.ends_with(suffix)))
            }
            "contains" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "String method 'contains' expects 1 argument (substring), but got {}",
                        args.len()
                    )));
                }
                let substring = match &args[0].kind {
                    ValueKind::String(sub) => sub,
                    other => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                Ok(Value::boolean(s.contains(substring.as_str())))
            }
            _ => Err(GraphoidError::runtime(format!(
                "String does not have method '{}'",
                method
            ))),
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
                    other => Err(GraphoidError::runtime(format!(
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

    /// Evaluates a method call on a graph.
    fn eval_graph_method(&mut self, mut graph: crate::values::Graph, method: &str, args: &[Value], object_expr: &Expr) -> Result<Value> {
        match method {
            "add_node" => {
                // Add a node to the graph
                if args.len() != 2 {
                    return Err(GraphoidError::runtime(format!(
                        "add_node() expects 2 arguments (node_id, value), but got {}",
                        args.len()
                    )));
                }

                // Get node ID (must be string)
                let node_id = match &args[0].kind {
                    ValueKind::String(s) => s.clone(),
                    other => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                // Get node value
                let node_value = args[1].clone();

                // Add the node
                graph.add_node(node_id, node_value)?;

                // Update graph in environment
                if let Expr::Variable { name, .. } = object_expr {
                    self.env.set(name, Value::graph(graph))?;
                }

                Ok(Value::none())
            }
            "add_edge" => {
                // Add an edge between two nodes
                if args.len() < 2 || args.len() > 3 {
                    return Err(GraphoidError::runtime(format!(
                        "add_edge() expects 2-3 arguments (from, to, [edge_type]), but got {}",
                        args.len()
                    )));
                }

                // Get from node ID
                let from = match &args[0].kind {
                    ValueKind::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                // Get to node ID
                let to = match &args[1].kind {
                    ValueKind::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", args[1].type_name()));
                    }
                };

                // Get optional edge type (default to "edge")
                let edge_type = if args.len() == 3 {
                    match &args[2].kind {
                        ValueKind::String(s) => s.clone(),
                        other => {
                            return Err(GraphoidError::type_error("string", args[2].type_name()));
                        }
                    }
                } else {
                    "edge".to_string()
                };

                // Add the edge with empty properties and no weight
                use std::collections::HashMap;
                let properties = HashMap::new();
                graph.add_edge(from, to, edge_type, None, properties)?;

                // Update graph in environment
                if let Expr::Variable { name, .. } = object_expr {
                    self.env.set(name, Value::graph(graph))?;
                }

                Ok(Value::none())
            }
            "remove_node" => {
                // Remove a node from the graph
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "remove_node() expects 1 argument (node_id), but got {}",
                        args.len()
                    )));
                }

                // Get node ID
                let node_id = match &args[0].kind {
                    ValueKind::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                // Remove the node
                graph.remove_node(node_id)?;

                // Update graph in environment
                if let Expr::Variable { name, .. } = object_expr {
                    self.env.set(name, Value::graph(graph))?;
                }

                Ok(Value::none())
            }
            "remove_edge" => {
                // Remove an edge from the graph
                if args.len() != 2 {
                    return Err(GraphoidError::runtime(format!(
                        "remove_edge() expects 2 arguments (from, to), but got {}",
                        args.len()
                    )));
                }

                // Get from node ID
                let from = match &args[0].kind {
                    ValueKind::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                // Get to node ID
                let to = match &args[1].kind {
                    ValueKind::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", args[1].type_name()));
                    }
                };

                // Remove the edge
                graph.remove_edge(from, to)?;

                // Update graph in environment
                if let Expr::Variable { name, .. } = object_expr {
                    self.env.set(name, Value::graph(graph))?;
                }

                Ok(Value::none())
            }
            "with_ruleset" => {
                // Apply a ruleset to the graph
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "with_ruleset() expects 1 argument, but got {}",
                        args.len()
                    )));
                }

                // Get the ruleset name from symbol argument
                let ruleset_name = match &args[0].kind {
                    ValueKind::Symbol(name) => name.clone(),
                    other => {
                        return Err(GraphoidError::runtime(format!(
                            "with_ruleset() expects a symbol argument, got {}",
                            args[0].type_name()
                        )));
                    }
                };

                // Apply the ruleset (currently just stores the name)
                graph = graph.with_ruleset(ruleset_name);
                Ok(Value::graph(graph))
            }
            "has_ruleset" => {
                // Check if graph has a specific ruleset
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "has_ruleset() expects 1 argument, but got {}",
                        args.len()
                    )));
                }

                let ruleset_name = match &args[0].kind {
                    ValueKind::Symbol(name) => name.as_str(),
                    other => {
                        return Err(GraphoidError::runtime(format!(
                            "has_ruleset() expects a symbol argument, got {}",
                            args[0].type_name()
                        )));
                    }
                };

                Ok(Value::boolean(graph.has_ruleset(ruleset_name)))
            }
            "has_path" => {
                // Check if a path exists between two nodes
                if args.len() != 2 {
                    return Err(GraphoidError::runtime(format!(
                        "has_path() expects 2 arguments (from, to), but got {}",
                        args.len()
                    )));
                }

                // Get from node ID
                let from = match &args[0].kind {
                    ValueKind::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                // Get to node ID
                let to = match &args[1].kind {
                    ValueKind::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", args[1].type_name()));
                    }
                };

                // Check if path exists
                let has_path = graph.has_path(from, to);
                Ok(Value::boolean(has_path))
            }
            "distance" => {
                // Get shortest path distance between two nodes
                if args.len() != 2 {
                    return Err(GraphoidError::runtime(format!(
                        "distance() expects 2 arguments (from, to), but got {}",
                        args.len()
                    )));
                }

                // Get from node ID
                let from = match &args[0].kind {
                    ValueKind::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                // Get to node ID
                let to = match &args[1].kind {
                    ValueKind::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", args[1].type_name()));
                    }
                };

                // Get distance
                let dist = graph.distance(from, to);
                Ok(Value::number(dist as f64))
            }
            "all_paths" => {
                // Find all paths between two nodes up to max length
                if args.len() != 3 {
                    return Err(GraphoidError::runtime(format!(
                        "all_paths() expects 3 arguments (from, to, max_length), but got {}",
                        args.len()
                    )));
                }

                // Get from node ID
                let from = match &args[0].kind {
                    ValueKind::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                // Get to node ID
                let to = match &args[1].kind {
                    ValueKind::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", args[1].type_name()));
                    }
                };

                // Get max length
                let max_len = match &args[2].kind {
                    ValueKind::Number(n) => *n as usize,
                    other => {
                        return Err(GraphoidError::type_error("number", args[2].type_name()));
                    }
                };

                // Find all paths
                let paths = graph.all_paths(from, to, max_len);

                // Convert Vec<Vec<String>> to ValueKind::List(List of Lists)
                use crate::values::List;
                let path_values: Vec<Value> = paths
                    .into_iter()
                    .map(|path| {
                        let string_values: Vec<Value> = path
                            .into_iter()
                            .map(|s| Value::string(s))
                            .collect();
                        Value::list(List::from_vec(string_values))
                    })
                    .collect();

                Ok(Value::list(List::from_vec(path_values)))
            }
            _ => Err(GraphoidError::runtime(format!(
                "Graph does not have method '{}'",
                method
            ))),
        }
    }

    /// Evaluates a function call expression.
    fn eval_call(&mut self, callee: &Expr, args: &[crate::ast::Argument]) -> Result<Value> {
        use crate::ast::Argument;

        // Check if this is a builtin function call (special handling)
        if let Expr::Variable { name, .. } = callee {
            match name.as_str() {
                "RuntimeError" | "ValueError" | "TypeError" | "IOError" | "NetworkError" | "ParseError" => {
                    // Evaluate the message argument
                    if args.len() != 1 {
                        return Err(GraphoidError::runtime(format!(
                            "{} constructor expects 1 argument (message), got {}",
                            name, args.len()
                        )));
                    }
                    let message_value = match &args[0] {
                        Argument::Positional(expr) => self.eval_expr(expr)?,
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
                                "RuntimeError".to_string(), // TODO: preserve actual error type
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
                _ => {}
            }
        }

        // Evaluate the callee to get the function
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

        // Delegate to call_function (which has graph tracking)
        self.call_function(&func, &arg_values)
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
                Argument::Named { name, value } => {
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
                Argument::Positional(expr) => {
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
    fn call_function(&mut self, func: &Function, arg_values: &[Value]) -> Result<Value> {
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
        let saved_env = std::mem::replace(&mut self.env, call_env);

        // Execute function body - either pattern matching or traditional
        let mut return_value = Value::none();
        let execution_result: Result<()> = (|| {
            // Check if this is a pattern matching function
            if let Some(ref pattern_clauses) = func.pattern_clauses {
                // Pattern matching function - use PatternMatcher to find matching clause
                use crate::execution::PatternMatcher;

                let matcher = PatternMatcher::new();
                let match_result = matcher.find_match(pattern_clauses, arg_values)?;

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

        // Save modifications back to the captured environment (for closure state)
        // Extract the parent environment from call_env (which may have been modified)
        if let Some(modified_parent) = self.env.take_parent() {
            // Update the captured environment with modifications
            *func.env.borrow_mut() = *modified_parent;
        }

        // Restore original environment
        self.env = saved_env;

        // Pop function from call stack (traditional)
        self.call_stack.pop();

        // Pop function from graph with return value
        self.function_graph.borrow_mut().pop_call(return_value.clone());

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
    fn apply_named_transformation(&self, value: &Value, transform_name: &str) -> Result<Value> {
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
    fn apply_named_predicate(&self, value: &Value, predicate_name: &str) -> Result<bool> {
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

    /// Applies an element-wise operation to lists or scalars.
    /// Supports list-list and list-scalar operations (broadcasting).
    fn eval_element_wise(&mut self, left: Value, right: Value, base_op: BinaryOp) -> Result<Value> {
        match (&left.kind, &right.kind) {
            // List-List element-wise operation (zips to shorter length)
            (ValueKind::List(left_list), ValueKind::List(right_list)) => {
                let left_elements = left_list.to_vec();
                let right_elements = right_list.to_vec();

                // Apply operation element by element (zip stops at shorter length)
                let mut results = Vec::new();
                for (left_elem, right_elem) in left_elements.iter().zip(right_elements.iter()) {
                    let result = self.apply_scalar_op(left_elem.clone(), right_elem.clone(), &base_op)?;
                    results.push(result);
                }
                Ok(Value::list(List::from_vec(results)))
            }
            // List-Scalar element-wise operation (broadcast scalar)
            (ValueKind::List(list), _scalar) => {
                let elements = list.to_vec();
                let mut results = Vec::new();
                for elem in elements.iter() {
                    let result = self.apply_scalar_op(elem.clone(), right.clone(), &base_op)?;
                    results.push(result);
                }
                Ok(Value::list(List::from_vec(results)))
            }
            // Scalar-List element-wise operation (broadcast scalar)
            (_scalar, ValueKind::List(list)) => {
                let elements = list.to_vec();
                let mut results = Vec::new();
                for elem in elements.iter() {
                    let result = self.apply_scalar_op(left.clone(), elem.clone(), &base_op)?;
                    results.push(result);
                }
                Ok(Value::list(List::from_vec(results)))
            }
            // Scalar-Scalar: not element-wise, error
            (_, _) => Err(GraphoidError::runtime(format!(
                "Element-wise operations require at least one list, got {} and {}",
                left.type_name(),
                right.type_name()
            ))),
        }
    }

    /// Applies a scalar binary operation (used by element-wise operations).
    fn apply_scalar_op(&mut self, left: Value, right: Value, op: &BinaryOp) -> Result<Value> {
        match op {
            // Arithmetic operators
            BinaryOp::Add => self.eval_add(left, right),
            BinaryOp::Subtract => self.eval_subtract(left, right),
            BinaryOp::Multiply => self.eval_multiply(left, right),
            BinaryOp::Divide => self.eval_divide(left, right),
            BinaryOp::IntDiv => self.eval_int_div(left, right),
            BinaryOp::Modulo => self.eval_modulo(left, right),
            BinaryOp::Power => self.eval_power(left, right),
            // Comparison operators
            BinaryOp::Equal => Ok(Value::boolean(left == right)),
            BinaryOp::NotEqual => Ok(Value::boolean(left != right)),
            BinaryOp::Less => self.eval_less(left, right),
            BinaryOp::LessEqual => self.eval_less_equal(left, right),
            BinaryOp::Greater => self.eval_greater(left, right),
            BinaryOp::GreaterEqual => self.eval_greater_equal(left, right),
            _ => Err(GraphoidError::runtime(format!(
                "Unsupported scalar operation: {:?}",
                op
            ))),
        }
    }

    // Arithmetic helpers
    fn eval_add(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => Ok(Value::number(l + r)),
            (ValueKind::String(_), _) | (_, ValueKind::String(_)) => {
                // If either operand is a string, convert both to strings and concatenate
                let left_str = left.to_string_value();
                let right_str = right.to_string_value();
                Ok(Value::string(format!("{}{}", left_str, right_str)))
            }
            (l, r) => Err(GraphoidError::type_error(
                "number or string",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    fn eval_subtract(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => Ok(Value::number(l - r)),
            (l, r) => Err(GraphoidError::type_error(
                "number",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    fn eval_multiply(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => Ok(Value::number(l * r)),
            (l, r) => Err(GraphoidError::type_error(
                "number",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    fn eval_divide(&mut self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => {
                if *r == 0.0 {
                    // Check error mode
                    match self.config_stack.current().error_mode {
                        ErrorMode::Lenient => {
                            // Return none in lenient mode
                            return Ok(Value::none());
                        }
                        ErrorMode::Collect => {
                            // Collect error and return none
                            let error = GraphoidError::division_by_zero();
                            self.error_collector.collect(
                                error,
                                self.current_file.as_ref().map(|p| p.to_string_lossy().to_string()),
                                SourcePosition::unknown(),
                            );
                            return Ok(Value::none());
                        }
                        ErrorMode::Strict => {
                            // Default behavior - raise error
                            return Err(GraphoidError::division_by_zero());
                        }
                    }
                } else {
                    Ok(Value::number(l / r))
                }
            }
            (l, r) => Err(GraphoidError::type_error(
                "number",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    fn eval_int_div(&mut self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => {
                if *r == 0.0 {
                    // Check error mode
                    match self.config_stack.current().error_mode {
                        ErrorMode::Lenient => {
                            return Ok(Value::none());
                        }
                        ErrorMode::Collect => {
                            let error = GraphoidError::division_by_zero();
                            self.error_collector.collect(
                                error,
                                self.current_file.as_ref().map(|p| p.to_string_lossy().to_string()),
                                SourcePosition::unknown(),
                            );
                            return Ok(Value::none());
                        }
                        ErrorMode::Strict => {
                            return Err(GraphoidError::division_by_zero());
                        }
                    }
                } else {
                    // Truncate toward zero (not floor)
                    Ok(Value::number((l / r).trunc()))
                }
            }
            (l, r) => Err(GraphoidError::type_error(
                "number",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    fn eval_modulo(&mut self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => {
                if *r == 0.0 {
                    // Check error mode for modulo by zero
                    match self.config_stack.current().error_mode {
                        ErrorMode::Lenient => {
                            return Ok(Value::none());
                        }
                        ErrorMode::Collect => {
                            let error = GraphoidError::runtime("Modulo by zero".to_string());
                            self.error_collector.collect(
                                error,
                                self.current_file.as_ref().map(|p| p.to_string_lossy().to_string()),
                                SourcePosition::unknown(),
                            );
                            return Ok(Value::none());
                        }
                        ErrorMode::Strict => {
                            return Err(GraphoidError::runtime("Modulo by zero".to_string()));
                        }
                    }
                } else {
                    Ok(Value::number(l % r))
                }
            }
            (l, r) => Err(GraphoidError::type_error(
                "number",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    fn eval_power(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => Ok(Value::number(l.powf(*r))),
            (l, r) => Err(GraphoidError::type_error(
                "number",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    // Comparison helpers
    fn eval_less(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => Ok(Value::boolean(l < r)),
            (ValueKind::String(l), ValueKind::String(r)) => Ok(Value::boolean(l < r)),
            (l, r) => Err(GraphoidError::type_error(
                "number or string",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    fn eval_less_equal(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => Ok(Value::boolean(l <= r)),
            (ValueKind::String(l), ValueKind::String(r)) => Ok(Value::boolean(l <= r)),
            (l, r) => Err(GraphoidError::type_error(
                "number or string",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    fn eval_greater(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => Ok(Value::boolean(l > r)),
            (ValueKind::String(l), ValueKind::String(r)) => Ok(Value::boolean(l > r)),
            (l, r) => Err(GraphoidError::type_error(
                "number or string",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
        }
    }

    fn eval_greater_equal(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => Ok(Value::boolean(l >= r)),
            (ValueKind::String(l), ValueKind::String(r)) => Ok(Value::boolean(l >= r)),
            (l, r) => Err(GraphoidError::type_error(
                "number or string",
                &format!("{} and {}", left.type_name(), right.type_name()),
            )),
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

        // Resolve the module path
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

        // Create Module value
        let module = Module {
            name: module_name,
            alias: module_alias,
            namespace: module_executor.env.clone(),
            file_path: resolved_path.clone(),
            config: None, // TODO: Extract config from module
        };

        // Register module in manager
        self.module_manager.register_module(resolved_path.to_string_lossy().to_string(), module.clone());

        // End loading
        self.module_manager.end_loading(&resolved_path);

        Ok(Value::module(module))
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
                    // Create an Error object from the GraphoidError with call stack
                    let error_obj = ErrorObject::with_stack_trace(
                        error_type_name.clone(),
                        actual_message.clone(),
                        self.current_file.as_ref().map(|p| p.to_string_lossy().to_string()),
                        0,    // TODO: Extract from GraphoidError position
                        0,    // TODO: Extract from GraphoidError position
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
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

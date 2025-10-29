use crate::ast::{AssignmentTarget, BinaryOp, Expr, LiteralValue, Stmt, UnaryOp};
use crate::error::{GraphoidError, Result};
use crate::execution::Environment;
use crate::execution::config::ConfigStack;
use crate::execution::error_collector::ErrorCollector;
use crate::execution::module_manager::{ModuleManager, Module};
use crate::values::{Function, Value, List, Hash};
use crate::graph::{RuleSpec, RuleInstance};
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::collections::HashMap;
use std::rc::Rc;
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

        // Execute all statements
        for stmt in &program.statements {
            self.eval_stmt(stmt)?;
        }

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
            Expr::Variable { name, .. } => self.env.get(name),
            Expr::Binary {
                left,
                op,
                right,
                ..
            } => self.eval_binary(left, op, right),
            Expr::Unary { op, operand, .. } => self.eval_unary(op, operand),
            Expr::Call { callee, args, .. } => self.eval_call(callee, args),
            Expr::Lambda { params, body, .. } => self.eval_lambda(params, body),
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
            Expr::Raise { error, .. } => {
                // Evaluate the error expression and raise it
                let error_value = self.eval_expr(error)?;
                let message = match error_value {
                    Value::String(s) => s,
                    other => format!("{:?}", other),
                };
                Err(GraphoidError::runtime(message))
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
                        match obj {
                            Value::Graph(mut graph) => {
                                // For graphs, index must be a string (node ID)
                                let node_id = match idx {
                                    Value::String(s) => s,
                                    _ => return Err(GraphoidError::type_error("string", idx.type_name())),
                                };

                                // Add or update the node
                                graph.add_node(node_id, val)?;

                                // Update the graph in the environment
                                // We need to get the variable name from the object expression
                                if let Expr::Variable { name, .. } = object.as_ref() {
                                    self.env.set(name, Value::Graph(graph))?;
                                }
                                Ok(None)
                            }
                            Value::Map(mut hash) => {
                                // For maps, index must be a string (key)
                                let key = match idx {
                                    Value::String(s) => s,
                                    _ => return Err(GraphoidError::type_error("string", idx.type_name())),
                                };

                                // Apply transformation rules with executor context if hash has them
                                let transformed_val = self.apply_transformation_rules_with_context(val, &hash.graph.rules)?;

                                // Insert key-value pair (using raw to avoid double-applying behaviors)
                                hash.insert_raw(key, transformed_val)?;

                                // Update the map in the environment
                                if let Expr::Variable { name, .. } = object.as_ref() {
                                    self.env.set(name, Value::Map(hash))?;
                                }
                                Ok(None)
                            }
                            Value::List(mut list) => {
                                // For lists, index must be a number
                                let index_num = match idx {
                                    Value::Number(n) => n as usize,
                                    _ => return Err(GraphoidError::type_error("number", idx.type_name())),
                                };

                                // Apply transformation rules with executor context if list has them
                                let transformed_val = self.apply_transformation_rules_with_context(val, &list.graph.rules)?;

                                // Update element at index (using raw to avoid double-applying behaviors)
                                list.set_raw(index_num, transformed_val)?;

                                // Update the list in the environment
                                if let Expr::Variable { name, .. } = object.as_ref() {
                                    self.env.set(name, Value::List(list))?;
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
                ..
            } => {
                // Extract parameter names (ignore default values for now)
                let param_names: Vec<String> = params.iter().map(|p| p.name.clone()).collect();

                // Create function value with captured environment
                let func = Function {
                    name: Some(name.clone()),
                    params: param_names,
                    body: body.clone(),
                    env: Rc::new(self.env.clone()),
                };

                // Store function in environment
                self.env.define(name.clone(), Value::Function(func));
                Ok(None)
            }
            Stmt::Return { value, .. } => {
                let return_value = if let Some(expr) = value {
                    self.eval_expr(expr)?
                } else {
                    Value::None
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
                let values = match iterable_value {
                    Value::List(ref items) => items.to_vec(),
                    other => {
                        return Err(GraphoidError::type_error(
                            "list",
                            other.type_name(),
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
                self.env.define("__module_name__".to_string(), Value::String(name.clone()));
                if let Some(alias_name) = alias {
                    self.env.define("__module_alias__".to_string(), Value::String(alias_name.clone()));
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
            LiteralValue::Number(n) => Ok(Value::Number(*n)),
            LiteralValue::String(s) => Ok(Value::String(s.clone())),
            LiteralValue::Boolean(b) => Ok(Value::Boolean(*b)),
            LiteralValue::None => Ok(Value::None),
            LiteralValue::Symbol(s) => Ok(Value::Symbol(s.clone())),
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
            BinaryOp::Equal => Ok(Value::Boolean(left_val == right_val)),
            BinaryOp::NotEqual => Ok(Value::Boolean(left_val != right_val)),
            BinaryOp::Less => self.eval_less(left_val, right_val),
            BinaryOp::LessEqual => self.eval_less_equal(left_val, right_val),
            BinaryOp::Greater => self.eval_greater(left_val, right_val),
            BinaryOp::GreaterEqual => self.eval_greater_equal(left_val, right_val),

            // Logical operators
            BinaryOp::And => Ok(Value::Boolean(left_val.is_truthy() && right_val.is_truthy())),
            BinaryOp::Or => Ok(Value::Boolean(left_val.is_truthy() || right_val.is_truthy())),

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
            UnaryOp::Negate => match val {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(GraphoidError::type_error("number", val.type_name())),
            },
            UnaryOp::Not => Ok(Value::Boolean(!val.is_truthy())),
        }
    }

    /// Evaluates a list expression.
    fn eval_list(&mut self, elements: &[Expr]) -> Result<Value> {
        let mut values = Vec::new();
        for elem in elements {
            values.push(self.eval_expr(elem)?);
        }
        Ok(Value::List(List::from_vec(values)))
    }

    /// Evaluates a map expression.
    fn eval_map(&mut self, entries: &[(String, Expr)]) -> Result<Value> {
        let mut map = HashMap::new();
        for (key, value_expr) in entries {
            let value = self.eval_expr(value_expr)?;
            map.insert(key.clone(), value);
        }
        Ok(Value::Map(Hash::from_hashmap(map)))
    }

    /// Evaluates a graph expression.
    fn eval_graph(&mut self, config: &[(String, Expr)]) -> Result<Value> {
        use crate::values::{Graph, GraphType};

        // Parse configuration to determine graph type
        let mut graph_type = GraphType::Directed; // Default

        for (key, value_expr) in config {
            if key == "type" {
                let value = self.eval_expr(value_expr)?;
                if let Value::Symbol(s) = value {
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

        Ok(Value::Graph(Graph::new(graph_type)))
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
        let is_truthy = match condition_value {
            Value::Boolean(b) => b,
            Value::None => false,
            Value::Number(n) => n != 0.0,
            Value::String(ref s) => !s.is_empty(),
            Value::List(ref l) => l.len() > 0,
            Value::Map(ref h) => h.len() > 0,
            Value::Graph(ref g) => g.node_count() > 0,
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
                None => Ok(Value::None),
            }
        }
    }

    /// Evaluates a lambda expression.
    /// Creates an anonymous function that captures the current environment.
    fn eval_lambda(&self, params: &[String], body: &Expr) -> Result<Value> {
        // Convert expression body to a return statement
        let return_stmt = Stmt::Return {
            value: Some((*body).clone()),
            position: body.position().clone(),
        };

        // Create anonymous function with captured environment
        let func = Function {
            name: None, // Anonymous
            params: params.to_vec(),
            body: vec![return_stmt],
            env: Rc::new(self.env.clone()),
        };

        Ok(Value::Function(func))
    }

    /// Evaluates an index expression (list[i] or map[key]).
    fn eval_index(&mut self, object: &Expr, index: &Expr) -> Result<Value> {
        // Evaluate the object being indexed
        let object_value = self.eval_expr(object)?;

        // Evaluate the index expression
        let index_value = self.eval_expr(index)?;

        match object_value {
            Value::List(ref list) => {
                // Index must be a number for lists
                let idx = match index_value {
                    Value::Number(n) => n,
                    other => {
                        return Err(GraphoidError::type_error(
                            "number",
                            other.type_name(),
                        ));
                    }
                };

                // Handle fractional indices by truncating to integer
                let idx_int = idx as i64;

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
                    return Err(GraphoidError::runtime(format!(
                        "List index out of bounds: index {} for list of length {}",
                        idx_int,
                        list.len()
                    )));
                }

                Ok(list.get(actual_index as usize).unwrap().clone())
            }
            Value::Map(ref hash) => {
                // Index must be a string for maps
                let key = match index_value {
                    Value::String(s) => s,
                    other => {
                        return Err(GraphoidError::type_error(
                            "string",
                            other.type_name(),
                        ));
                    }
                };

                // Look up the key
                match hash.get(&key) {
                    Some(value) => Ok(value.clone()),
                    None => Err(GraphoidError::runtime(format!(
                        "Map key not found: '{}'",
                        key
                    ))),
                }
            }
            Value::Graph(ref graph) => {
                // Index must be a string for graphs (node ID)
                let node_id = match index_value {
                    Value::String(s) => s,
                    other => {
                        return Err(GraphoidError::type_error(
                            "string",
                            other.type_name(),
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
            other => {
                Err(GraphoidError::runtime(format!(
                    "Cannot index value of type '{}'",
                    other.type_name()
                )))
            }
        }
    }

    /// Evaluates a method call expression (object.method(args)).
    fn eval_method_call(&mut self, object: &Expr, method: &str, args: &[Expr]) -> Result<Value> {
        // Check for static method calls on type identifiers (e.g., list.generate)
        if let Expr::Variable { name, .. } = object {
            if name == "list" {
                // Evaluate argument expressions
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.eval_expr(arg)?);
                }
                return self.eval_list_static_method(method, &arg_values);
            }
        }

        // Evaluate the object once
        let object_value = self.eval_expr(object)?;

        // Check for module member access (e.g., module.function(args) or module.variable)
        if let Value::Module(ref module) = object_value {
            // Look up the member in the module's namespace
            let member = module.namespace.get(method)?;

            // If it's a function, call it with args
            if let Value::Function(func) = member {
                // Evaluate argument expressions
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.eval_expr(arg)?);
                }
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
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.eval_expr(arg)?);
        }

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

            // Apply method to the already-evaluated value, update variable
            let new_value = self.apply_method_to_value(object_value, base_method, &arg_values, object)?;
            self.env.set(&var_name, new_value)?;

            // Mutating methods return none
            Ok(Value::None)
        } else {
            // Immutable method - use the already-evaluated value
            self.apply_method_to_value(object_value, base_method, &arg_values, object)
        }
    }

    /// Applies a method to a value (helper to avoid duplication).
    fn apply_method_to_value(&mut self, value: Value, method: &str, args: &[Value], object_expr: &Expr) -> Result<Value> {
        match value {
            Value::List(list) => self.eval_list_method(&list, method, args),
            Value::Map(hash) => self.eval_map_method(&hash, method, args),
            Value::Graph(graph) => self.eval_graph_method(graph, method, args, object_expr),
            Value::String(ref s) => self.eval_string_method(s, method, args),
            other => Err(GraphoidError::runtime(format!(
                "Type '{}' does not have method '{}'",
                other.type_name(),
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

                let start = match &args[0] {
                    Value::Number(n) => *n,
                    other => {
                        return Err(GraphoidError::type_error("number", other.type_name()));
                    }
                };

                let end = match &args[1] {
                    Value::Number(n) => *n,
                    other => {
                        return Err(GraphoidError::type_error("number", other.type_name()));
                    }
                };

                // Check if third argument is a function or a number (step)
                match &args[2] {
                    Value::Number(step) => {
                        // Range mode with step
                        let mut result = Vec::new();
                        if *step > 0.0 {
                            let mut current = start;
                            while current <= end {
                                result.push(Value::Number(current));
                                current += step;
                            }
                        } else if *step < 0.0 {
                            let mut current = start;
                            while current >= end {
                                result.push(Value::Number(current));
                                current += step;
                            }
                        } else {
                            return Err(GraphoidError::runtime("generate step cannot be zero".to_string()));
                        }
                        Ok(Value::List(List::from_vec(result)))
                    }
                    Value::Function(func) => {
                        // Function mode
                        let mut result = Vec::new();
                        let start_i = start as i64;
                        let end_i = end as i64;
                        for i in start_i..=end_i {
                            let arg = Value::Number(i as f64);
                            let value = self.call_function(func, &[arg])?;
                            result.push(value);
                        }
                        Ok(Value::List(List::from_vec(result)))
                    }
                    other => {
                        return Err(GraphoidError::runtime(format!(
                            "list.generate() expects third argument to be number or function, got {}",
                            other.type_name()
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

                let n = match &args[0] {
                    Value::Number(num) => *num as i64,
                    other => {
                        return Err(GraphoidError::type_error("number", other.type_name()));
                    }
                };

                let mut result = Vec::new();
                for i in 0..=n {
                    result.push(Value::Number(i as f64));
                }
                Ok(Value::List(List::from_vec(result)))
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
                Ok(Value::Number(list.len() as f64))
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
                        return Ok(Value::Boolean(true));
                    }
                }
                Ok(Value::Boolean(false))
            }
            "is_empty" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'is_empty' expects 0 arguments, but got {}",
                        args.len()
                    )));
                }
                Ok(Value::Boolean(list.is_empty()))
            }
            "map" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'map' expects 1 argument, but got {}",
                        args.len()
                    )));
                }

                // Check if argument is a symbol (named transformation) or function
                match &args[0] {
                    Value::Symbol(transform_name) => {
                        // Apply named transformation
                        let mut results = Vec::new();
                        for element in &elements {
                            let result = self.apply_named_transformation(element, transform_name)?;
                            results.push(result);
                        }
                        Ok(Value::List(List::from_vec(results)))
                    }
                    Value::Function(func) => {
                        // Apply the function to each element
                        let mut results = Vec::new();
                        for element in &elements {
                            // Call the function with this element
                            let result = self.call_function(func, &[element.clone()])?;
                            results.push(result);
                        }
                        Ok(Value::List(List::from_vec(results)))
                    }
                    other => {
                        return Err(GraphoidError::runtime(format!(
                            "Method 'map' expects function or symbol, got {}",
                            other.type_name()
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
                match &args[0] {
                    Value::Symbol(predicate_name) => {
                        // Apply named predicate
                        let mut results = Vec::new();
                        for element in &elements {
                            if self.apply_named_predicate(element, predicate_name)? {
                                results.push(element.clone());
                            }
                        }
                        Ok(Value::List(List::from_vec(results)))
                    }
                    Value::Function(func) => {
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
                        Ok(Value::List(List::from_vec(results)))
                    }
                    other => {
                        return Err(GraphoidError::runtime(format!(
                            "Method 'filter' expects function or symbol, got {}",
                            other.type_name()
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
                let func = match &args[0] {
                    Value::Function(f) => f,
                    other => {
                        return Err(GraphoidError::type_error(
                            "function",
                            other.type_name(),
                        ));
                    }
                };

                // Execute the function for each element (for side effects)
                for element in &elements {
                    // Call the function with this element, ignore result
                    let _ = self.call_function(func, &[element.clone()])?;
                }

                // Return the original list
                Ok(Value::List(list.clone()))
            }
            "slice" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'slice' expects 2 or 3 arguments, but got {}",
                        args.len()
                    )));
                }

                // Get start and end indices
                let start_idx = match &args[0] {
                    Value::Number(n) => *n as i64,
                    other => {
                        return Err(GraphoidError::type_error(
                            "number",
                            other.type_name(),
                        ));
                    }
                };

                let end_idx = match &args[1] {
                    Value::Number(n) => *n as i64,
                    other => {
                        return Err(GraphoidError::type_error(
                            "number",
                            other.type_name(),
                        ));
                    }
                };

                // Get optional step parameter (default 1)
                let step = if args.len() == 3 {
                    match &args[2] {
                        Value::Number(n) => *n as i64,
                        other => {
                            return Err(GraphoidError::type_error(
                                "number",
                                other.type_name(),
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
                    return Ok(Value::List(List::new()));
                }

                // Extract slice with step
                let mut slice = Vec::new();
                let mut i = actual_start;
                while i < actual_end {
                    slice.push(elements[i as usize].clone());
                    i += step;
                }
                Ok(Value::List(List::from_vec(slice)))
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
                let rule_symbol = match &args[0] {
                    Value::Symbol(name) => name.as_str(),
                    other => {
                        return Err(GraphoidError::runtime(format!(
                            "add_rule() expects a symbol, got {}",
                            other.type_name()
                        )));
                    }
                };

                // Clone list
                let mut new_list = list.clone();

                // Try to parse rule from symbol (handles both validation and transformation rules)
                // Get optional parameter
                let param = if args.len() == 2 {
                    match &args[1] {
                        Value::Number(n) => Some(*n),
                        other => {
                            return Err(GraphoidError::runtime(format!(
                                "add_rule() parameter must be a number, got {}",
                                other.type_name()
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
                Ok(Value::List(new_list))
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
                let rule_symbol = match &args[0] {
                    Value::Symbol(name) => name.as_str(),
                    other => {
                        return Err(GraphoidError::runtime(format!(
                            "remove_rule() expects a symbol, got {}",
                            other.type_name()
                        )));
                    }
                };

                // Get optional parameter
                let param = if args.len() == 2 {
                    match &args[1] {
                        Value::Number(n) => Some(*n),
                        other => {
                            return Err(GraphoidError::runtime(format!(
                                "remove_rule() parameter must be a number, got {}",
                                other.type_name()
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
                Ok(Value::List(new_list))
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
                    match (a, b) {
                        (Value::Number(n1), Value::Number(n2)) => {
                            n1.partial_cmp(n2).unwrap_or(std::cmp::Ordering::Equal)
                        }
                        _ => std::cmp::Ordering::Equal,
                    }
                });
                Ok(Value::List(List::from_vec(sorted)))
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
                Ok(Value::List(List::from_vec(reversed)))
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
                Ok(Value::List(List::from_vec(unique)))
            }
            "reject" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'reject' expects 1 argument, but got {}",
                        args.len()
                    )));
                }

                // Reject is opposite of filter
                match &args[0] {
                    Value::Symbol(predicate_name) => {
                        let mut results = Vec::new();
                        for element in &elements {
                            if !self.apply_named_predicate(element, predicate_name)? {
                                results.push(element.clone());
                            }
                        }
                        Ok(Value::List(List::from_vec(results)))
                    }
                    Value::Function(func) => {
                        let mut results = Vec::new();
                        for element in &elements {
                            let result = self.call_function(func, &[element.clone()])?;
                            if !result.is_truthy() {
                                results.push(element.clone());
                            }
                        }
                        Ok(Value::List(List::from_vec(results)))
                    }
                    other => {
                        return Err(GraphoidError::runtime(format!(
                            "Method 'reject' expects function or symbol, got {}",
                            other.type_name()
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
                    .filter(|v| !matches!(v, Value::None))
                    .cloned()
                    .collect();
                Ok(Value::List(List::from_vec(compacted)))
            }
            "select" => {
                // select is an alias for filter
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'select' expects 1 argument, but got {}",
                        args.len()
                    )));
                }

                match &args[0] {
                    Value::Symbol(predicate_name) => {
                        let mut results = Vec::new();
                        for element in &elements {
                            if self.apply_named_predicate(element, predicate_name)? {
                                results.push(element.clone());
                            }
                        }
                        Ok(Value::List(List::from_vec(results)))
                    }
                    Value::Function(func) => {
                        let mut results = Vec::new();
                        for element in &elements {
                            let result = self.call_function(func, &[element.clone()])?;
                            if result.is_truthy() {
                                results.push(element.clone());
                            }
                        }
                        Ok(Value::List(List::from_vec(results)))
                    }
                    other => {
                        return Err(GraphoidError::runtime(format!(
                            "Method 'select' expects function or symbol, got {}",
                            other.type_name()
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
                Ok(Value::List(new_list))
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
                    .map(|k| Value::String(k.clone()))
                    .collect();
                Ok(Value::List(List::from_vec(keys)))
            }
            "values" => {
                // Return list of all values
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(
                        "values() takes no arguments".to_string()
                    ));
                }
                let values: Vec<Value> = hash.values();
                Ok(Value::List(List::from_vec(values)))
            }
            "has_key" => {
                // Check if key exists
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(
                        "has_key() requires exactly one argument".to_string()
                    ));
                }
                let key = match &args[0] {
                    Value::String(s) => s,
                    _ => return Err(GraphoidError::runtime(
                        "has_key() requires a string argument".to_string()
                    )),
                };
                Ok(Value::Boolean(hash.contains_key(key)))
            }
            "size" => {
                // Return number of entries
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(
                        "size() takes no arguments".to_string()
                    ));
                }
                Ok(Value::Number(hash.len() as f64))
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
                let rule_symbol = match &args[0] {
                    Value::Symbol(name) => name.as_str(),
                    other => {
                        return Err(GraphoidError::runtime(format!(
                            "add_rule() expects a symbol, got {}",
                            other.type_name()
                        )));
                    }
                };

                // Clone hash
                let mut new_hash = hash.clone();

                // Try to parse rule from symbol (handles both validation and transformation rules)
                // Get optional parameter
                let param = if args.len() == 2 {
                    match &args[1] {
                        Value::Number(n) => Some(*n),
                        other => {
                            return Err(GraphoidError::runtime(format!(
                                "add_rule() parameter must be a number, got {}",
                                other.type_name()
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
                Ok(Value::Map(new_hash))
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
                let rule_symbol = match &args[0] {
                    Value::Symbol(name) => name.as_str(),
                    other => {
                        return Err(GraphoidError::runtime(format!(
                            "remove_rule() expects a symbol, got {}",
                            other.type_name()
                        )));
                    }
                };

                // Get optional parameter
                let param = if args.len() == 2 {
                    match &args[1] {
                        Value::Number(n) => Some(*n),
                        other => {
                            return Err(GraphoidError::runtime(format!(
                                "remove_rule() parameter must be a number, got {}",
                                other.type_name()
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
                Ok(Value::Map(new_hash))
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
                Ok(Value::Number(s.len() as f64))
            }
            _ => Err(GraphoidError::runtime(format!(
                "String does not have method '{}'",
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
                let node_id = match &args[0] {
                    Value::String(s) => s.clone(),
                    other => {
                        return Err(GraphoidError::type_error("string", other.type_name()));
                    }
                };

                // Get node value
                let node_value = args[1].clone();

                // Add the node
                graph.add_node(node_id, node_value)?;

                // Update graph in environment
                if let Expr::Variable { name, .. } = object_expr {
                    self.env.set(name, Value::Graph(graph))?;
                }

                Ok(Value::None)
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
                let from = match &args[0] {
                    Value::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", other.type_name()));
                    }
                };

                // Get to node ID
                let to = match &args[1] {
                    Value::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", other.type_name()));
                    }
                };

                // Get optional edge type (default to "edge")
                let edge_type = if args.len() == 3 {
                    match &args[2] {
                        Value::String(s) => s.clone(),
                        other => {
                            return Err(GraphoidError::type_error("string", other.type_name()));
                        }
                    }
                } else {
                    "edge".to_string()
                };

                // Add the edge with empty properties
                use std::collections::HashMap;
                let properties = HashMap::new();
                graph.add_edge(from, to, edge_type, properties)?;

                // Update graph in environment
                if let Expr::Variable { name, .. } = object_expr {
                    self.env.set(name, Value::Graph(graph))?;
                }

                Ok(Value::None)
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
                let node_id = match &args[0] {
                    Value::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", other.type_name()));
                    }
                };

                // Remove the node
                graph.remove_node(node_id)?;

                // Update graph in environment
                if let Expr::Variable { name, .. } = object_expr {
                    self.env.set(name, Value::Graph(graph))?;
                }

                Ok(Value::None)
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
                let from = match &args[0] {
                    Value::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", other.type_name()));
                    }
                };

                // Get to node ID
                let to = match &args[1] {
                    Value::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", other.type_name()));
                    }
                };

                // Remove the edge
                graph.remove_edge(from, to)?;

                // Update graph in environment
                if let Expr::Variable { name, .. } = object_expr {
                    self.env.set(name, Value::Graph(graph))?;
                }

                Ok(Value::None)
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
                let ruleset_name = match &args[0] {
                    Value::Symbol(name) => name.clone(),
                    other => {
                        return Err(GraphoidError::runtime(format!(
                            "with_ruleset() expects a symbol argument, got {}",
                            other.type_name()
                        )));
                    }
                };

                // Apply the ruleset (currently just stores the name)
                graph = graph.with_ruleset(ruleset_name);
                Ok(Value::Graph(graph))
            }
            "has_ruleset" => {
                // Check if graph has a specific ruleset
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "has_ruleset() expects 1 argument, but got {}",
                        args.len()
                    )));
                }

                let ruleset_name = match &args[0] {
                    Value::Symbol(name) => name.as_str(),
                    other => {
                        return Err(GraphoidError::runtime(format!(
                            "has_ruleset() expects a symbol argument, got {}",
                            other.type_name()
                        )));
                    }
                };

                Ok(Value::Boolean(graph.has_ruleset(ruleset_name)))
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
                let from = match &args[0] {
                    Value::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", other.type_name()));
                    }
                };

                // Get to node ID
                let to = match &args[1] {
                    Value::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", other.type_name()));
                    }
                };

                // Check if path exists
                let has_path = graph.has_path(from, to);
                Ok(Value::Boolean(has_path))
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
                let from = match &args[0] {
                    Value::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", other.type_name()));
                    }
                };

                // Get to node ID
                let to = match &args[1] {
                    Value::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", other.type_name()));
                    }
                };

                // Get distance
                let dist = graph.distance(from, to);
                Ok(Value::Number(dist as f64))
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
                let from = match &args[0] {
                    Value::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", other.type_name()));
                    }
                };

                // Get to node ID
                let to = match &args[1] {
                    Value::String(s) => s.as_str(),
                    other => {
                        return Err(GraphoidError::type_error("string", other.type_name()));
                    }
                };

                // Get max length
                let max_len = match &args[2] {
                    Value::Number(n) => *n as usize,
                    other => {
                        return Err(GraphoidError::type_error("number", other.type_name()));
                    }
                };

                // Find all paths
                let paths = graph.all_paths(from, to, max_len);

                // Convert Vec<Vec<String>> to Value::List(List of Lists)
                use crate::values::List;
                let path_values: Vec<Value> = paths
                    .into_iter()
                    .map(|path| {
                        let string_values: Vec<Value> = path
                            .into_iter()
                            .map(|s| Value::String(s))
                            .collect();
                        Value::List(List::from_vec(string_values))
                    })
                    .collect();

                Ok(Value::List(List::from_vec(path_values)))
            }
            _ => Err(GraphoidError::runtime(format!(
                "Graph does not have method '{}'",
                method
            ))),
        }
    }

    /// Evaluates a function call expression.
    fn eval_call(&mut self, callee: &Expr, args: &[Expr]) -> Result<Value> {
        // Evaluate the callee to get the function
        let callee_value = self.eval_expr(callee)?;

        // Check if it's a function
        let func = match callee_value {
            Value::Function(f) => f,
            other => {
                return Err(GraphoidError::type_error(
                    "function",
                    other.type_name(),
                ));
            }
        };

        // Evaluate all argument expressions
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.eval_expr(arg)?);
        }

        // Check argument count
        if arg_values.len() != func.params.len() {
            return Err(GraphoidError::runtime(format!(
                "Function '{}' expects {} arguments, but got {}",
                func.name.as_ref().unwrap_or(&"<anonymous>".to_string()),
                func.params.len(),
                arg_values.len()
            )));
        }

        // Push function name onto call stack
        let func_name = func.name.as_ref().unwrap_or(&"<anonymous>".to_string()).clone();
        self.call_stack.push(func_name.clone());

        // Create new environment as child of function's closure environment
        let mut call_env = Environment::with_parent((*func.env).clone());

        // Bind parameters to argument values
        for (param_name, arg_value) in func.params.iter().zip(arg_values.iter()) {
            call_env.define(param_name.clone(), arg_value.clone());
        }

        // Save current environment and switch to call environment
        let saved_env = std::mem::replace(&mut self.env, call_env);

        // Execute function body
        let mut return_value = Value::None;
        let execution_result: Result<()> = (|| {
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
            Ok(())
        })();

        // Restore original environment
        self.env = saved_env;

        // Pop function from call stack
        self.call_stack.pop();

        // Propagate errors
        execution_result?;

        Ok(return_value)
    }

    /// Helper method to call a function with given argument values.
    /// Used by map, filter, each, and other functional methods.
    fn call_function(&mut self, func: &Function, arg_values: &[Value]) -> Result<Value> {
        // Check argument count
        if arg_values.len() != func.params.len() {
            return Err(GraphoidError::runtime(format!(
                "Function '{}' expects {} arguments, but got {}",
                func.name.as_ref().unwrap_or(&"<anonymous>".to_string()),
                func.params.len(),
                arg_values.len()
            )));
        }

        // Push function name onto call stack
        let func_name = func.name.as_ref().unwrap_or(&"<anonymous>".to_string()).clone();
        self.call_stack.push(func_name.clone());

        // Create new environment as child of function's closure environment
        let mut call_env = Environment::with_parent((*func.env).clone());

        // Bind parameters to argument values
        for (param_name, arg_value) in func.params.iter().zip(arg_values.iter()) {
            call_env.define(param_name.clone(), arg_value.clone());
        }

        // Save current environment and switch to call environment
        let saved_env = std::mem::replace(&mut self.env, call_env);

        // Execute function body
        let mut return_value = Value::None;
        let execution_result: Result<()> = (|| {
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
            Ok(())
        })();

        // Restore original environment
        self.env = saved_env;

        // Pop function from call stack
        self.call_stack.pop();

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
                    match function {
                        Value::Function(func) => {
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
                    let condition_func = match condition {
                        Value::Function(f) => f,
                        _ => {
                            return Err(GraphoidError::runtime(
                                "Conditional behavior condition must be a function".to_string()
                            ));
                        }
                    };

                    let condition_result = self.call_function(condition_func, &[current.clone()])?;

                    // Check if condition is truthy
                    let is_truthy = match condition_result {
                        Value::Boolean(b) => b,
                        Value::None => false,
                        Value::Number(n) => n != 0.0,
                        _ => true, // Non-false, non-none values are truthy
                    };

                    if is_truthy {
                        // Apply transform function
                        let transform_func = match transform {
                            Value::Function(f) => f,
                            _ => {
                                return Err(GraphoidError::runtime(
                                    "Conditional behavior transform must be a function".to_string()
                                ));
                            }
                        };
                        current = self.call_function(transform_func, &[current])?;
                    } else if let Some(fallback_val) = fallback {
                        // Apply fallback function
                        let fallback_func = match fallback_val {
                            Value::Function(f) => f,
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

        match (a, b) {
            // Same types - compare naturally
            (Value::None, Value::None) => Ok(Ordering::Equal),
            (Value::Boolean(a), Value::Boolean(b)) => Ok(a.cmp(b)),
            (Value::Number(a), Value::Number(b)) => {
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
            (Value::String(a), Value::String(b)) => Ok(a.cmp(b)),
            (Value::Symbol(a), Value::Symbol(b)) => Ok(a.cmp(b)),

            // Different types - use type ordering
            (Value::None, _) => Ok(Ordering::Less),
            (_, Value::None) => Ok(Ordering::Greater),
            (Value::Boolean(_), Value::Number(_)) => Ok(Ordering::Less),
            (Value::Number(_), Value::Boolean(_)) => Ok(Ordering::Greater),
            (Value::Boolean(_), Value::String(_)) => Ok(Ordering::Less),
            (Value::String(_), Value::Boolean(_)) => Ok(Ordering::Greater),
            (Value::Number(_), Value::String(_)) => Ok(Ordering::Less),
            (Value::String(_), Value::Number(_)) => Ok(Ordering::Greater),
            (Value::Number(_), Value::Symbol(_)) => Ok(Ordering::Less),
            (Value::Symbol(_), Value::Number(_)) => Ok(Ordering::Greater),
            (Value::String(_), Value::Symbol(_)) => Ok(Ordering::Less),
            (Value::Symbol(_), Value::String(_)) => Ok(Ordering::Greater),

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
        match result {
            Value::Number(n) => {
                if n < 0.0 {
                    Ok(Ordering::Less)
                } else if n > 0.0 {
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

            let ordering = match compare_fn {
                Some(Value::Function(func)) => {
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
                match value {
                    Value::Number(n) => Ok(Value::Number(n * 2.0)),
                    _ => Err(GraphoidError::runtime(format!(
                        "Transformation 'double' requires a number, got {}",
                        value.type_name()
                    ))),
                }
            }
            "square" => {
                match value {
                    Value::Number(n) => Ok(Value::Number(n * n)),
                    _ => Err(GraphoidError::runtime(format!(
                        "Transformation 'square' requires a number, got {}",
                        value.type_name()
                    ))),
                }
            }
            "negate" => {
                match value {
                    Value::Number(n) => Ok(Value::Number(-n)),
                    _ => Err(GraphoidError::runtime(format!(
                        "Transformation 'negate' requires a number, got {}",
                        value.type_name()
                    ))),
                }
            }
            "increment" | "inc" => {
                match value {
                    Value::Number(n) => Ok(Value::Number(n + 1.0)),
                    _ => Err(GraphoidError::runtime(format!(
                        "Transformation 'increment' requires a number, got {}",
                        value.type_name()
                    ))),
                }
            }
            "decrement" | "dec" => {
                match value {
                    Value::Number(n) => Ok(Value::Number(n - 1.0)),
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
                match value {
                    Value::Number(n) => Ok((n % 2.0).abs() < 0.0001), // Handle floating point comparison
                    _ => Err(GraphoidError::runtime(format!(
                        "Predicate 'even' requires a number, got {}",
                        value.type_name()
                    ))),
                }
            }
            "odd" => {
                match value {
                    Value::Number(n) => Ok((n % 2.0).abs() > 0.0001), // Handle floating point comparison
                    _ => Err(GraphoidError::runtime(format!(
                        "Predicate 'odd' requires a number, got {}",
                        value.type_name()
                    ))),
                }
            }
            "positive" | "pos" => {
                match value {
                    Value::Number(n) => Ok(*n > 0.0),
                    _ => Err(GraphoidError::runtime(format!(
                        "Predicate 'positive' requires a number, got {}",
                        value.type_name()
                    ))),
                }
            }
            "negative" | "neg" => {
                match value {
                    Value::Number(n) => Ok(*n < 0.0),
                    _ => Err(GraphoidError::runtime(format!(
                        "Predicate 'negative' requires a number, got {}",
                        value.type_name()
                    ))),
                }
            }
            "zero" => {
                match value {
                    Value::Number(n) => Ok(n.abs() < 0.0001), // Handle floating point comparison
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
        match (left, right) {
            // List-List element-wise operation (zips to shorter length)
            (Value::List(left_list), Value::List(right_list)) => {
                let left_elements = left_list.to_vec();
                let right_elements = right_list.to_vec();

                // Apply operation element by element (zip stops at shorter length)
                let mut results = Vec::new();
                for (left_elem, right_elem) in left_elements.iter().zip(right_elements.iter()) {
                    let result = self.apply_scalar_op(left_elem.clone(), right_elem.clone(), &base_op)?;
                    results.push(result);
                }
                Ok(Value::List(List::from_vec(results)))
            }
            // List-Scalar element-wise operation (broadcast scalar)
            (Value::List(list), scalar) => {
                let elements = list.to_vec();
                let mut results = Vec::new();
                for elem in elements.iter() {
                    let result = self.apply_scalar_op(elem.clone(), scalar.clone(), &base_op)?;
                    results.push(result);
                }
                Ok(Value::List(List::from_vec(results)))
            }
            // Scalar-List element-wise operation (broadcast scalar)
            (scalar, Value::List(list)) => {
                let elements = list.to_vec();
                let mut results = Vec::new();
                for elem in elements.iter() {
                    let result = self.apply_scalar_op(scalar.clone(), elem.clone(), &base_op)?;
                    results.push(result);
                }
                Ok(Value::List(List::from_vec(results)))
            }
            // Scalar-Scalar: not element-wise, error
            (left, right) => Err(GraphoidError::runtime(format!(
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
            BinaryOp::Equal => Ok(Value::Boolean(left == right)),
            BinaryOp::NotEqual => Ok(Value::Boolean(left != right)),
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
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
            (Value::String(l), Value::String(r)) => {
                let mut result = l.clone();
                result.push_str(&r);
                Ok(Value::String(result))
            }
            (l, r) => Err(GraphoidError::type_error(
                "number or string",
                &format!("{} and {}", l.type_name(), r.type_name()),
            )),
        }
    }

    fn eval_subtract(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
            (l, r) => Err(GraphoidError::type_error(
                "number",
                &format!("{} and {}", l.type_name(), r.type_name()),
            )),
        }
    }

    fn eval_multiply(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
            (l, r) => Err(GraphoidError::type_error(
                "number",
                &format!("{} and {}", l.type_name(), r.type_name()),
            )),
        }
    }

    fn eval_divide(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => {
                if r == 0.0 {
                    Err(GraphoidError::division_by_zero())
                } else {
                    Ok(Value::Number(l / r))
                }
            }
            (l, r) => Err(GraphoidError::type_error(
                "number",
                &format!("{} and {}", l.type_name(), r.type_name()),
            )),
        }
    }

    fn eval_int_div(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => {
                if r == 0.0 {
                    Err(GraphoidError::division_by_zero())
                } else {
                    // Truncate toward zero (not floor)
                    Ok(Value::Number((l / r).trunc()))
                }
            }
            (l, r) => Err(GraphoidError::type_error(
                "number",
                &format!("{} and {}", l.type_name(), r.type_name()),
            )),
        }
    }

    fn eval_modulo(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l % r)),
            (l, r) => Err(GraphoidError::type_error(
                "number",
                &format!("{} and {}", l.type_name(), r.type_name()),
            )),
        }
    }

    fn eval_power(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l.powf(r))),
            (l, r) => Err(GraphoidError::type_error(
                "number",
                &format!("{} and {}", l.type_name(), r.type_name()),
            )),
        }
    }

    // Comparison helpers
    fn eval_less(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l < r)),
            (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l < r)),
            (l, r) => Err(GraphoidError::type_error(
                "number or string",
                &format!("{} and {}", l.type_name(), r.type_name()),
            )),
        }
    }

    fn eval_less_equal(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l <= r)),
            (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l <= r)),
            (l, r) => Err(GraphoidError::type_error(
                "number or string",
                &format!("{} and {}", l.type_name(), r.type_name()),
            )),
        }
    }

    fn eval_greater(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l > r)),
            (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l > r)),
            (l, r) => Err(GraphoidError::type_error(
                "number or string",
                &format!("{} and {}", l.type_name(), r.type_name()),
            )),
        }
    }

    fn eval_greater_equal(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l >= r)),
            (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l >= r)),
            (l, r) => Err(GraphoidError::type_error(
                "number or string",
                &format!("{} and {}", l.type_name(), r.type_name()),
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
            return Ok(Value::Module(module.clone()));
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
        let module_name = if let Some(Value::String(name)) = module_executor.get_variable("__module_name__") {
            name
        } else {
            // Use filename without extension as module name
            resolved_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unnamed")
                .to_string()
        };

        let module_alias = if let Some(Value::String(alias)) = module_executor.get_variable("__module_alias__") {
            Some(alias)
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

        Ok(Value::Module(module))
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
        let error_type_name = match error {
            GraphoidError::SyntaxError { .. } => "SyntaxError",
            GraphoidError::TypeError { .. } => "TypeError",
            GraphoidError::RuntimeError { .. } => "RuntimeError",
            GraphoidError::RuleViolation { .. } => "RuleViolation",
            GraphoidError::ModuleNotFound { .. } => "ModuleNotFound",
            GraphoidError::IOError { .. } => "IOError",
            GraphoidError::CircularDependency { .. } => "CircularDependency",
            GraphoidError::IoError(_) => "IoError",
            GraphoidError::ConfigError { .. } => "ConfigError",
        };

        // Search for a matching catch clause
        for catch_clause in catch_clauses {
            // Check if this catch clause matches the error type
            let matches = if let Some(ref expected_type) = catch_clause.error_type {
                expected_type == error_type_name
            } else {
                // Catch-all clause (no type specified)
                true
            };

            if matches {
                // Bind error to variable if specified (in current scope)
                if let Some(ref var_name) = catch_clause.variable {
                    // Convert error to a string value for binding
                    let error_message = error.to_string();
                    self.env.define(var_name.clone(), Value::String(error_message));
                }

                // Execute catch body
                let mut result = None;
                for stmt in &catch_clause.body {
                    if let Some(val) = self.eval_stmt(stmt)? {
                        result = Some(val);
                        break;
                    }
                }

                // Note: We're not removing the error variable to keep this simple
                // In a real implementation, we'd need a proper scoping mechanism

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

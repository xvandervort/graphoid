use crate::ast::{AssignmentTarget, BinaryOp, Expr, LiteralValue, Stmt, UnaryOp};
use crate::error::{GraphoidError, Result};
use crate::execution::Environment;
use crate::values::{Function, Value};
use std::collections::HashMap;
use std::rc::Rc;

/// The executor evaluates AST nodes and produces values.
pub struct Executor {
    env: Environment,
    call_stack: Vec<String>,
}

impl Executor {
    /// Creates a new executor with a fresh environment.
    pub fn new() -> Self {
        Executor {
            env: Environment::new(),
            call_stack: Vec::new(),
        }
    }

    /// Creates a new executor with a given environment.
    pub fn with_env(env: Environment) -> Self {
        Executor {
            env,
            call_stack: Vec::new(),
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
                    _ => Err(GraphoidError::runtime(
                        "Index assignment not yet supported".to_string(),
                    )),
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
                    Value::List(ref items) => items.clone(),
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
        Ok(Value::List(values))
    }

    /// Evaluates a map expression.
    fn eval_map(&mut self, entries: &[(String, Expr)]) -> Result<Value> {
        let mut map = HashMap::new();
        for (key, value_expr) in entries {
            let value = self.eval_expr(value_expr)?;
            map.insert(key.clone(), value);
        }
        Ok(Value::Map(map))
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
            Value::List(ref elements) => {
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
                    let len = elements.len() as i64;
                    len + idx_int
                } else {
                    idx_int
                };

                // Check bounds
                if actual_index < 0 || actual_index >= elements.len() as i64 {
                    return Err(GraphoidError::runtime(format!(
                        "List index out of bounds: index {} for list of length {}",
                        idx_int,
                        elements.len()
                    )));
                }

                Ok(elements[actual_index as usize].clone())
            }
            Value::Map(ref map) => {
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
                match map.get(&key) {
                    Some(value) => Ok(value.clone()),
                    None => Err(GraphoidError::runtime(format!(
                        "Map key not found: '{}'",
                        key
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
        // Evaluate the object
        let object_value = self.eval_expr(object)?;

        // Evaluate all argument expressions
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.eval_expr(arg)?);
        }

        // Dispatch based on object type and method name
        match &object_value {
            Value::List(elements) => self.eval_list_method(elements, method, &arg_values),
            Value::Map(map) => self.eval_map_method(map, method, &arg_values),
            other => Err(GraphoidError::runtime(format!(
                "Type '{}' does not have method '{}'",
                other.type_name(),
                method
            ))),
        }
    }

    /// Evaluates a method call on a list.
    fn eval_list_method(&mut self, elements: &[Value], method: &str, args: &[Value]) -> Result<Value> {
        match method {
            "size" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'size' expects 0 arguments, but got {}",
                        args.len()
                    )));
                }
                Ok(Value::Number(elements.len() as f64))
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
                for element in elements {
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
                Ok(Value::Boolean(elements.is_empty()))
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
                        for element in elements {
                            let result = self.apply_named_transformation(element, transform_name)?;
                            results.push(result);
                        }
                        Ok(Value::List(results))
                    }
                    Value::Function(func) => {
                        // Apply the function to each element
                        let mut results = Vec::new();
                        for element in elements {
                            // Call the function with this element
                            let result = self.call_function(func, &[element.clone()])?;
                            results.push(result);
                        }
                        Ok(Value::List(results))
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
                        for element in elements {
                            if self.apply_named_predicate(element, predicate_name)? {
                                results.push(element.clone());
                            }
                        }
                        Ok(Value::List(results))
                    }
                    Value::Function(func) => {
                        // Filter elements based on predicate function
                        let mut results = Vec::new();
                        for element in elements {
                            // Call the function with this element
                            let result = self.call_function(func, &[element.clone()])?;

                            // Check if result is truthy
                            if result.is_truthy() {
                                results.push(element.clone());
                            }
                        }
                        Ok(Value::List(results))
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
                for element in elements {
                    // Call the function with this element, ignore result
                    let _ = self.call_function(func, &[element.clone()])?;
                }

                // Return the original list
                Ok(Value::List(elements.to_vec()))
            }
            _ => Err(GraphoidError::runtime(format!(
                "List does not have method '{}'",
                method
            ))),
        }
    }

    /// Evaluates a method call on a map.
    fn eval_map_method(&mut self, _map: &HashMap<String, Value>, method: &str, _args: &[Value]) -> Result<Value> {
        // Placeholder for map methods - will implement later
        Err(GraphoidError::runtime(format!(
            "Map does not have method '{}' (not yet implemented)",
            method
        )))
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
                    Ok(Value::Number((l / r).floor()))
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
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

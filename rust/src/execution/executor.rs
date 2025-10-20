use crate::ast::{AssignmentTarget, BinaryOp, Expr, LiteralValue, Stmt, UnaryOp};
use crate::error::{GraphoidError, Result};
use crate::execution::Environment;
use crate::values::{Function, Value};
use std::collections::HashMap;
use std::rc::Rc;

/// The executor evaluates AST nodes and produces values.
pub struct Executor {
    env: Environment,
}

impl Executor {
    /// Creates a new executor with a fresh environment.
    pub fn new() -> Self {
        Executor {
            env: Environment::new(),
        }
    }

    /// Creates a new executor with a given environment.
    pub fn with_env(env: Environment) -> Self {
        Executor { env }
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
            Expr::List { elements, .. } => self.eval_list(elements),
            Expr::Map { entries, .. } => self.eval_map(entries),
            _ => Err(GraphoidError::runtime(format!(
                "Unsupported expression type: {:?}",
                expr
            ))),
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
            Stmt::Expression { expr, .. } => {
                // Evaluate expression and discard result
                // In REPL mode, caller may want to print the result
                self.eval_expr(expr)?;
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

        // Restore original environment
        self.env = saved_env;

        Ok(return_value)
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
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

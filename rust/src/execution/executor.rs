use crate::ast::{AssignmentTarget, BinaryOp, Expr, LiteralValue, Stmt, UnaryOp};
use crate::error::{GraphoidError, Result};
use crate::execution::Environment;
use crate::values::Value;
use std::collections::HashMap;

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
            Expr::List { elements, .. } => self.eval_list(elements),
            Expr::Map { entries, .. } => self.eval_map(entries),
            _ => Err(GraphoidError::runtime(format!(
                "Unsupported expression type: {:?}",
                expr
            ))),
        }
    }

    /// Executes a statement.
    pub fn eval_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::VariableDecl {
                name,
                value,
                ..
            } => {
                let val = self.eval_expr(value)?;
                self.env.define(name.clone(), val);
                Ok(())
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
                        Ok(())
                    }
                    _ => Err(GraphoidError::runtime(
                        "Index assignment not yet supported".to_string(),
                    )),
                }
            }
            Stmt::Expression { expr, .. } => {
                // Evaluate expression and discard result
                // In REPL mode, caller may want to print the result
                self.eval_expr(expr)?;
                Ok(())
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

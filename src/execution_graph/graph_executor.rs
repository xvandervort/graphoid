//! Graph-based executor: traverses an ExecutionGraph to produce Values.
//!
//! Phase 16: Replaces the tree-walking interpreter with graph traversal.

use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;

use crate::ast::{BinaryOp, Parameter};
use crate::error::{GraphoidError, Result};
use crate::execution::{Environment, ConfigStack, ErrorCollector};
use crate::execution::function_graph::FunctionGraph;
use crate::execution::module_manager::ModuleManager;
use crate::values::{Value, ValueKind, Function};
use crate::lexer::Lexer;
use crate::parser::Parser;

use super::ExecutionGraph;
use super::arena::NodeRef;
use super::converter::AstToGraphConverter;
use super::node::*;

/// The graph-based executor. Traverses an ExecutionGraph to produce values.
pub struct GraphExecutor {
    pub(crate) env: Environment,
    pub(crate) call_stack: Vec<String>,
    pub(crate) module_manager: ModuleManager,
    pub(crate) current_file: Option<PathBuf>,
    pub config_stack: ConfigStack,
    pub precision_stack: Vec<Option<usize>>,
    pub error_collector: ErrorCollector,
    pub function_graph: Rc<RefCell<FunctionGraph>>,
    pub(crate) global_functions: HashMap<String, Vec<Function>>,
    pub(crate) private_symbols: std::collections::HashSet<String>,
    pub(crate) output_capture_enabled: bool,
    pub(crate) output_buffer: String,
    pub(crate) method_context_stack: Vec<String>,
    pub(crate) super_context_stack: Vec<crate::values::Graph>,
    pub(crate) writeback_stack: Vec<Vec<()>>, // Simplified for now
    pub(crate) block_self_stack: Vec<Value>,
    pub(crate) graph_method_value_stack: Vec<Value>,
    pub(crate) suppress_self_property_assignment: usize,
    pub(crate) function_call_depth: usize,
    /// The execution graph being traversed (set during execution)
    graph: Option<ExecutionGraph>,
    /// Maps function IDs to their body NodeRef (for graph-based function execution)
    graph_function_bodies: HashMap<String, NodeRef>,
    /// Counter for generating unique function IDs
    next_func_id: usize,
}

impl GraphExecutor {
    pub fn new() -> Self {
        GraphExecutor {
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
            graph_method_value_stack: Vec::new(),
            suppress_self_property_assignment: 0,
            function_call_depth: 0,
            graph: None,
            graph_function_bodies: HashMap::new(),
            next_func_id: 0,
        }
    }

    /// Execute source code: lex → parse → convert → execute.
    pub fn execute_source(&mut self, source: &str) -> Result<Value> {
        let tokens = Lexer::new(source).tokenize()
            .map_err(|e| GraphoidError::runtime(format!("Lexer error: {}", e)))?;
        let program = Parser::new(tokens).parse()
            .map_err(|e| GraphoidError::runtime(format!("Parser error: {}", e)))?;

        let mut converter = AstToGraphConverter::new();
        let root = converter.convert_program(&program);
        let exec_graph = converter.into_graph();

        self.execute(exec_graph, root)
    }

    /// Execute an execution graph from the given root node.
    pub fn execute(&mut self, graph: ExecutionGraph, root: NodeRef) -> Result<Value> {
        self.graph = Some(graph);
        let result = self.execute_node(root);
        result
    }

    /// Get a variable value from the environment.
    pub fn get_variable(&self, name: &str) -> Option<Value> {
        self.env.get(name).ok()
    }

    /// Execute a single node by dispatching on its type.
    fn execute_node(&mut self, node_ref: NodeRef) -> Result<Value> {
        let node = self.get_node(node_ref)?;
        let node_type = node.node_type.clone();

        match node_type {
            // Literals
            AstNodeType::NumberLit => self.exec_number_lit(node_ref),
            AstNodeType::StringLit => self.exec_string_lit(node_ref),
            AstNodeType::BoolLit => self.exec_bool_lit(node_ref),
            AstNodeType::NoneLit => Ok(Value::none()),
            AstNodeType::SymbolLit => self.exec_symbol_lit(node_ref),

            // Identifier
            AstNodeType::Identifier => self.exec_identifier(node_ref),

            // Expressions
            AstNodeType::BinaryExpr => self.exec_binary(node_ref),
            AstNodeType::UnaryExpr => self.exec_unary(node_ref),
            AstNodeType::ListExpr => self.exec_list(node_ref),
            AstNodeType::MapExpr => self.exec_map(node_ref),

            // Statements
            AstNodeType::Program => self.exec_program(node_ref),
            AstNodeType::ExpressionStmt => self.exec_expression_stmt(node_ref),
            AstNodeType::VarDeclStmt => self.exec_var_decl(node_ref),
            AstNodeType::AssignStmt => self.exec_assign(node_ref),
            AstNodeType::BlockExpr => self.exec_block(node_ref),

            // Functions
            AstNodeType::FuncDeclStmt => self.exec_func_decl(node_ref),
            AstNodeType::CallExpr => self.exec_call(node_ref),
            AstNodeType::LambdaExpr => self.exec_lambda(node_ref),

            // Methods and property access
            AstNodeType::MethodCallExpr => self.exec_method_call(node_ref),
            AstNodeType::IndexExpr => self.exec_index(node_ref),
            AstNodeType::PropertyAccessExpr => self.exec_property_access(node_ref),

            // Exceptions
            AstNodeType::TryStmt => self.exec_try(node_ref),
            AstNodeType::RaiseExpr => self.exec_raise(node_ref),

            // Control flow
            AstNodeType::IfStmt => self.exec_if(node_ref),
            AstNodeType::WhileStmt => self.exec_while(node_ref),
            AstNodeType::ForStmt => self.exec_for(node_ref),
            AstNodeType::ReturnStmt => self.exec_return(node_ref),
            AstNodeType::BreakStmt => Err(GraphoidError::LoopControl {
                control: crate::error::LoopControlType::Break,
            }),
            AstNodeType::ContinueStmt => Err(GraphoidError::LoopControl {
                control: crate::error::LoopControlType::Continue,
            }),
            AstNodeType::ConditionalExpr => self.exec_conditional(node_ref),

            _ => Err(GraphoidError::runtime(format!(
                "Unimplemented node type: {:?}", node_type
            ))),
        }
    }

    // --- Helper to borrow graph ---

    fn get_node(&self, node_ref: NodeRef) -> Result<&AstGraphNode> {
        self.graph.as_ref()
            .and_then(|g| g.get_node(node_ref))
            .ok_or_else(|| GraphoidError::runtime("Node not found in execution graph".to_string()))
    }

    fn get_edge_target(&self, from: NodeRef, edge_type: &ExecEdgeType) -> Option<NodeRef> {
        self.graph.as_ref().and_then(|g| g.get_edge_target(from, edge_type))
    }

    fn get_ordered_edges(&self, from: NodeRef, prefix: &str) -> Vec<NodeRef> {
        self.graph.as_ref().map(|g| g.get_ordered_edges(from, prefix)).unwrap_or_default()
    }

    fn get_property(&self, node_ref: NodeRef, key: &str) -> Option<AstProperty> {
        self.get_node(node_ref).ok()
            .and_then(|n| n.properties.get(key).cloned())
    }

    fn get_str_property(&self, node_ref: NodeRef, key: &str) -> Option<String> {
        match self.get_property(node_ref, key)? {
            AstProperty::Str(s) => Some(s),
            _ => None,
        }
    }

    // --- Literal execution ---

    fn exec_number_lit(&self, node_ref: NodeRef) -> Result<Value> {
        match self.get_property(node_ref, "value") {
            Some(AstProperty::Num(n)) => Ok(Value::number(n)),
            _ => Err(GraphoidError::runtime("Missing number value".to_string())),
        }
    }

    fn exec_string_lit(&self, node_ref: NodeRef) -> Result<Value> {
        match self.get_property(node_ref, "value") {
            Some(AstProperty::Str(s)) => Ok(Value::string(s)),
            _ => Err(GraphoidError::runtime("Missing string value".to_string())),
        }
    }

    fn exec_bool_lit(&self, node_ref: NodeRef) -> Result<Value> {
        match self.get_property(node_ref, "value") {
            Some(AstProperty::Bool(b)) => Ok(Value::boolean(b)),
            _ => Err(GraphoidError::runtime("Missing boolean value".to_string())),
        }
    }

    fn exec_symbol_lit(&self, node_ref: NodeRef) -> Result<Value> {
        match self.get_property(node_ref, "value") {
            Some(AstProperty::Str(s)) => Ok(Value::symbol(s)),
            _ => Err(GraphoidError::runtime("Missing symbol value".to_string())),
        }
    }

    // --- Identifier ---

    fn exec_identifier(&self, node_ref: NodeRef) -> Result<Value> {
        let name = self.get_str_property(node_ref, "name")
            .ok_or_else(|| GraphoidError::runtime("Missing identifier name".to_string()))?;

        // Check environment
        if self.env.exists(&name) {
            return self.env.get(&name);
        }

        // Check global functions
        if let Some(funcs) = self.global_functions.get(&name) {
            if let Some(f) = funcs.first() {
                return Ok(Value::function(f.clone()));
            }
        }

        // Special constants
        match name.as_str() {
            "true" => return Ok(Value::boolean(true)),
            "false" => return Ok(Value::boolean(false)),
            "none" => return Ok(Value::none()),
            _ => {}
        }

        Err(GraphoidError::undefined_variable(&name))
    }

    // --- Binary expression ---

    fn exec_binary(&mut self, node_ref: NodeRef) -> Result<Value> {
        let op = match self.get_property(node_ref, "operator") {
            Some(AstProperty::BinaryOp(op)) => op,
            _ => return Err(GraphoidError::runtime("Missing binary operator".to_string())),
        };

        // Short-circuit for logical operators
        match op {
            BinaryOp::And => {
                let left_ref = self.get_edge_target(node_ref, &ExecEdgeType::Left)
                    .ok_or_else(|| GraphoidError::runtime("Missing left operand".to_string()))?;
                let left = self.execute_node(left_ref)?;
                if !left.is_truthy() {
                    return Ok(Value::boolean(false));
                }
                let right_ref = self.get_edge_target(node_ref, &ExecEdgeType::Right)
                    .ok_or_else(|| GraphoidError::runtime("Missing right operand".to_string()))?;
                let right = self.execute_node(right_ref)?;
                return Ok(Value::boolean(right.is_truthy()));
            }
            BinaryOp::Or => {
                let left_ref = self.get_edge_target(node_ref, &ExecEdgeType::Left)
                    .ok_or_else(|| GraphoidError::runtime("Missing left operand".to_string()))?;
                let left = self.execute_node(left_ref)?;
                if left.is_truthy() {
                    return Ok(Value::boolean(true));
                }
                let right_ref = self.get_edge_target(node_ref, &ExecEdgeType::Right)
                    .ok_or_else(|| GraphoidError::runtime("Missing right operand".to_string()))?;
                let right = self.execute_node(right_ref)?;
                return Ok(Value::boolean(right.is_truthy()));
            }
            _ => {}
        }

        let left_ref = self.get_edge_target(node_ref, &ExecEdgeType::Left)
            .ok_or_else(|| GraphoidError::runtime("Missing left operand".to_string()))?;
        let right_ref = self.get_edge_target(node_ref, &ExecEdgeType::Right)
            .ok_or_else(|| GraphoidError::runtime("Missing right operand".to_string()))?;

        let left = self.execute_node(left_ref)?;
        let right = self.execute_node(right_ref)?;

        self.eval_binary_op(&op, left, right)
    }

    fn eval_binary_op(&self, op: &BinaryOp, left: Value, right: Value) -> Result<Value> {
        match op {
            BinaryOp::Add => self.eval_add(left, right),
            BinaryOp::Subtract => self.eval_subtract(left, right),
            BinaryOp::Multiply => self.eval_multiply(left, right),
            BinaryOp::Divide => self.eval_divide(left, right),
            BinaryOp::IntDiv => self.eval_int_div(left, right),
            BinaryOp::Modulo => self.eval_modulo(left, right),
            BinaryOp::Power => self.eval_power(left, right),
            BinaryOp::Equal => Ok(Value::boolean(left == right)),
            BinaryOp::NotEqual => Ok(Value::boolean(left != right)),
            BinaryOp::Less => match (&left.kind, &right.kind) {
                (ValueKind::String(_), ValueKind::String(_)) => self.eval_string_compare(left, right, |a, b| a < b),
                _ => self.eval_compare(left, right, |a, b| a < b),
            },
            BinaryOp::LessEqual => match (&left.kind, &right.kind) {
                (ValueKind::String(_), ValueKind::String(_)) => self.eval_string_compare(left, right, |a, b| a <= b),
                _ => self.eval_compare(left, right, |a, b| a <= b),
            },
            BinaryOp::Greater => match (&left.kind, &right.kind) {
                (ValueKind::String(_), ValueKind::String(_)) => self.eval_string_compare(left, right, |a, b| a > b),
                _ => self.eval_compare(left, right, |a, b| a > b),
            },
            BinaryOp::GreaterEqual => match (&left.kind, &right.kind) {
                (ValueKind::String(_), ValueKind::String(_)) => self.eval_string_compare(left, right, |a, b| a >= b),
                _ => self.eval_compare(left, right, |a, b| a >= b),
            },
            _ => Err(GraphoidError::runtime(format!("Unimplemented operator: {:?}", op))),
        }
    }

    fn eval_add(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => Ok(Value::number(l + r)),
            (ValueKind::String(l), ValueKind::String(r)) => Ok(Value::string(format!("{}{}", l, r))),
            (ValueKind::String(l), _) => Ok(Value::string(format!("{}{}", l, right.to_string()))),
            (_, ValueKind::String(r)) => Ok(Value::string(format!("{}{}", left.to_string(), r))),
            _ => Err(GraphoidError::type_error("number or string", &format!("{} and {}", left.type_name(), right.type_name()))),
        }
    }

    fn eval_subtract(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => Ok(Value::number(l - r)),
            _ => Err(GraphoidError::type_error("number", &format!("{} and {}", left.type_name(), right.type_name()))),
        }
    }

    fn eval_multiply(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => Ok(Value::number(l * r)),
            (ValueKind::String(s), ValueKind::Number(n)) | (ValueKind::Number(n), ValueKind::String(s)) => {
                Ok(Value::string(s.repeat(*n as usize)))
            }
            _ => Err(GraphoidError::type_error("number", &format!("{} and {}", left.type_name(), right.type_name()))),
        }
    }

    fn eval_divide(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => {
                if *r == 0.0 {
                    return Err(GraphoidError::division_by_zero());
                }
                Ok(Value::number(l / r))
            }
            _ => Err(GraphoidError::type_error("number", &format!("{} and {}", left.type_name(), right.type_name()))),
        }
    }

    fn eval_int_div(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => {
                if *r == 0.0 {
                    return Err(GraphoidError::division_by_zero());
                }
                Ok(Value::number((*l as i64 / *r as i64) as f64))
            }
            _ => Err(GraphoidError::type_error("number", &format!("{} and {}", left.type_name(), right.type_name()))),
        }
    }

    fn eval_modulo(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => {
                if *r == 0.0 {
                    return Err(GraphoidError::division_by_zero());
                }
                Ok(Value::number(l % r))
            }
            _ => Err(GraphoidError::type_error("number", &format!("{} and {}", left.type_name(), right.type_name()))),
        }
    }

    fn eval_power(&self, left: Value, right: Value) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => Ok(Value::number(l.powf(*r))),
            _ => Err(GraphoidError::type_error("number", &format!("{} and {}", left.type_name(), right.type_name()))),
        }
    }

    fn eval_compare(&self, left: Value, right: Value, cmp: fn(f64, f64) -> bool) -> Result<Value> {
        match (&left.kind, &right.kind) {
            (ValueKind::Number(l), ValueKind::Number(r)) => Ok(Value::boolean(cmp(*l, *r))),
            _ => Err(GraphoidError::type_error("comparable types", &format!("{} and {}", left.type_name(), right.type_name()))),
        }
    }

    fn eval_string_compare<F>(&self, left: Value, right: Value, cmp: F) -> Result<Value>
    where
        F: FnOnce(&str, &str) -> bool,
    {
        match (&left.kind, &right.kind) {
            (ValueKind::String(l), ValueKind::String(r)) => Ok(Value::boolean(cmp(l, r))),
            _ => Err(GraphoidError::type_error("string", &format!("{} and {}", left.type_name(), right.type_name()))),
        }
    }

    // --- Unary expression ---

    fn exec_unary(&mut self, node_ref: NodeRef) -> Result<Value> {
        let op = match self.get_property(node_ref, "operator") {
            Some(AstProperty::UnaryOp(op)) => op,
            _ => return Err(GraphoidError::runtime("Missing unary operator".to_string())),
        };

        let operand_ref = self.get_edge_target(node_ref, &ExecEdgeType::Operand)
            .ok_or_else(|| GraphoidError::runtime("Missing operand".to_string()))?;
        let operand = self.execute_node(operand_ref)?;

        match op {
            crate::ast::UnaryOp::Negate => {
                match &operand.kind {
                    ValueKind::Number(n) => Ok(Value::number(-n)),
                    _ => Err(GraphoidError::type_error("number", &operand.type_name())),
                }
            }
            crate::ast::UnaryOp::Not => {
                Ok(Value::boolean(!operand.is_truthy()))
            }
            crate::ast::UnaryOp::BitwiseNot => {
                match &operand.kind {
                    ValueKind::Number(n) => Ok(Value::number(!(*n as i64) as f64)),
                    _ => Err(GraphoidError::type_error("number", &operand.type_name())),
                }
            }
        }
    }

    // --- List literal ---

    fn exec_list(&mut self, node_ref: NodeRef) -> Result<Value> {
        let element_refs = self.get_ordered_edges(node_ref, "Element");
        let mut elements = Vec::new();
        for elem_ref in element_refs {
            let val = self.execute_node(elem_ref)?;
            elements.push(val);
        }
        Ok(Value::list(crate::values::List::from_vec(elements)))
    }

    // --- Map literal ---

    fn exec_map(&mut self, node_ref: NodeRef) -> Result<Value> {
        let entry_refs = self.get_ordered_edges(node_ref, "Element");
        let mut map = crate::values::Hash::new();
        for entry_ref in entry_refs {
            let key = self.get_str_property(entry_ref, "key")
                .ok_or_else(|| GraphoidError::runtime("Missing map key".to_string()))?;
            let val_ref = self.get_edge_target(entry_ref, &ExecEdgeType::ValueEdge)
                .ok_or_else(|| GraphoidError::runtime("Missing map value".to_string()))?;
            let val = self.execute_node(val_ref)?;
            map.insert(key, val).map_err(|e| GraphoidError::runtime(format!("{}", e)))?;
        }
        Ok(Value::map(map))
    }

    // --- Program ---

    fn exec_program(&mut self, node_ref: NodeRef) -> Result<Value> {
        let stmt_refs = self.get_ordered_edges(node_ref, "Element");
        let mut last_value = Value::none();
        for stmt_ref in stmt_refs {
            match self.execute_node(stmt_ref) {
                Ok(val) => last_value = val,
                Err(GraphoidError::ReturnControl { value }) => return Ok(value),
                Err(e) => return Err(e),
            }
        }
        Ok(last_value)
    }

    // --- Expression statement ---

    fn exec_expression_stmt(&mut self, node_ref: NodeRef) -> Result<Value> {
        let expr_ref = self.get_edge_target(node_ref, &ExecEdgeType::ValueEdge)
            .ok_or_else(|| GraphoidError::runtime("Missing expression in statement".to_string()))?;
        self.execute_node(expr_ref)
    }

    // --- Variable declaration ---

    fn exec_var_decl(&mut self, node_ref: NodeRef) -> Result<Value> {
        let name = self.get_str_property(node_ref, "name")
            .ok_or_else(|| GraphoidError::runtime("Missing variable name".to_string()))?;
        let val_ref = self.get_edge_target(node_ref, &ExecEdgeType::ValueEdge)
            .ok_or_else(|| GraphoidError::runtime("Missing variable value".to_string()))?;
        let value = self.execute_node(val_ref)?;
        self.env.define(name, value.clone());
        Ok(value)
    }

    // --- Assignment ---

    fn exec_assign(&mut self, node_ref: NodeRef) -> Result<Value> {
        let target_type = self.get_str_property(node_ref, "target_type")
            .ok_or_else(|| GraphoidError::runtime("Missing assignment target type".to_string()))?;

        let val_ref = self.get_edge_target(node_ref, &ExecEdgeType::ValueEdge)
            .ok_or_else(|| GraphoidError::runtime("Missing assignment value".to_string()))?;
        let value = self.execute_node(val_ref)?;

        match target_type.as_str() {
            "variable" => {
                let name = self.get_str_property(node_ref, "target_name")
                    .ok_or_else(|| GraphoidError::runtime("Missing target name".to_string()))?;
                if self.env.exists(&name) {
                    self.env.set(&name, value.clone())?;
                } else {
                    self.env.define(name, value.clone());
                }
            }
            "index" => {
                let obj_ref = self.get_edge_target(node_ref, &ExecEdgeType::Object)
                    .ok_or_else(|| GraphoidError::runtime("Missing index assignment object".to_string()))?;
                let idx_ref = self.get_edge_target(node_ref, &ExecEdgeType::Target)
                    .ok_or_else(|| GraphoidError::runtime("Missing index assignment index".to_string()))?;

                // We need the variable name to reassign
                let obj_node = self.get_node(obj_ref)?;
                let obj_name = if obj_node.node_type == AstNodeType::Identifier {
                    obj_node.properties.get("name").and_then(|p| match p {
                        AstProperty::Str(s) => Some(s.clone()),
                        _ => None,
                    })
                } else {
                    None
                };

                let mut object = self.execute_node(obj_ref)?;
                let index = self.execute_node(idx_ref)?;

                match (&mut object.kind, &index.kind) {
                    (ValueKind::List(ref mut items), ValueKind::Number(n)) => {
                        let idx = *n as usize;
                        items.set(idx, value.clone())?;
                    }
                    (ValueKind::Map(ref mut m), ValueKind::String(key)) => {
                        m.insert(key.clone(), value.clone())?;
                    }
                    _ => {
                        return Err(GraphoidError::runtime("Invalid index assignment target".to_string()));
                    }
                }

                // Write back the modified object
                if let Some(name) = obj_name {
                    self.env.set(&name, object)?;
                }
            }
            "property" => {
                let obj_ref = self.get_edge_target(node_ref, &ExecEdgeType::Object)
                    .ok_or_else(|| GraphoidError::runtime("Missing property assignment object".to_string()))?;
                let prop_name = self.get_str_property(node_ref, "target_name")
                    .ok_or_else(|| GraphoidError::runtime("Missing property name".to_string()))?;

                let obj_node = self.get_node(obj_ref)?;
                let obj_name = if obj_node.node_type == AstNodeType::Identifier {
                    obj_node.properties.get("name").and_then(|p| match p {
                        AstProperty::Str(s) => Some(s.clone()),
                        _ => None,
                    })
                } else {
                    None
                };

                let mut object = self.execute_node(obj_ref)?;

                match &mut object.kind {
                    ValueKind::Map(ref mut m) => {
                        m.insert(prop_name, value.clone())?;
                    }
                    _ => {
                        return Err(GraphoidError::runtime(format!(
                            "Cannot set property on type '{}'", object.type_name()
                        )));
                    }
                }

                // Write back the modified object
                if let Some(name) = obj_name {
                    self.env.set(&name, object)?;
                }
            }
            _ => {
                return Err(GraphoidError::runtime(format!("Unimplemented assignment target: {}", target_type)));
            }
        }

        Ok(value)
    }

    // --- Block ---

    fn exec_block(&mut self, node_ref: NodeRef) -> Result<Value> {
        let stmt_refs = self.get_ordered_edges(node_ref, "Element");
        let mut last_value = Value::none();
        for stmt_ref in stmt_refs {
            last_value = self.execute_node(stmt_ref)?;
        }
        Ok(last_value)
    }

    // --- If statement ---

    fn exec_if(&mut self, node_ref: NodeRef) -> Result<Value> {
        let cond_ref = self.get_edge_target(node_ref, &ExecEdgeType::Condition)
            .ok_or_else(|| GraphoidError::runtime("Missing if condition".to_string()))?;
        let cond_val = self.execute_node(cond_ref)?;

        if cond_val.is_truthy() {
            let then_ref = self.get_edge_target(node_ref, &ExecEdgeType::ThenBranch)
                .ok_or_else(|| GraphoidError::runtime("Missing then branch".to_string()))?;
            self.execute_node(then_ref)
        } else if let Some(else_ref) = self.get_edge_target(node_ref, &ExecEdgeType::ElseBranch) {
            self.execute_node(else_ref)
        } else {
            Ok(Value::none())
        }
    }

    // --- While statement ---

    fn exec_while(&mut self, node_ref: NodeRef) -> Result<Value> {
        let cond_ref = self.get_edge_target(node_ref, &ExecEdgeType::Condition)
            .ok_or_else(|| GraphoidError::runtime("Missing while condition".to_string()))?;
        let body_ref = self.get_edge_target(node_ref, &ExecEdgeType::Body)
            .ok_or_else(|| GraphoidError::runtime("Missing while body".to_string()))?;

        loop {
            let cond_val = self.execute_node(cond_ref)?;
            if !cond_val.is_truthy() {
                break;
            }

            match self.execute_node(body_ref) {
                Ok(_) => {}
                Err(GraphoidError::LoopControl { control }) => {
                    match control {
                        crate::error::LoopControlType::Break => break,
                        crate::error::LoopControlType::Continue => continue,
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Ok(Value::none())
    }

    // --- For statement ---

    fn exec_for(&mut self, node_ref: NodeRef) -> Result<Value> {
        let var_name = self.get_str_property(node_ref, "variable")
            .ok_or_else(|| GraphoidError::runtime("Missing for loop variable".to_string()))?;
        let iter_ref = self.get_edge_target(node_ref, &ExecEdgeType::Iterable)
            .ok_or_else(|| GraphoidError::runtime("Missing for loop iterable".to_string()))?;
        let body_ref = self.get_edge_target(node_ref, &ExecEdgeType::Body)
            .ok_or_else(|| GraphoidError::runtime("Missing for loop body".to_string()))?;

        let iterable_value = self.execute_node(iter_ref)?;

        // Get values to iterate over
        let values: Vec<Value> = match &iterable_value.kind {
            ValueKind::List(items) => items.to_vec(),
            ValueKind::String(s) => {
                s.chars().map(|c| Value::string(c.to_string())).collect()
            }
            _ => {
                return Err(GraphoidError::type_error(
                    "list or string",
                    iterable_value.type_name(),
                ));
            }
        };

        for value in values {
            // Bind loop variable
            if self.env.exists(&var_name) {
                self.env.set(&var_name, value)?;
            } else {
                self.env.define(var_name.clone(), value);
            }

            // Execute body
            match self.execute_node(body_ref) {
                Ok(_) => {}
                Err(GraphoidError::LoopControl { control }) => {
                    match control {
                        crate::error::LoopControlType::Break => break,
                        crate::error::LoopControlType::Continue => continue,
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Ok(Value::none())
    }

    // --- Return statement ---

    fn exec_return(&mut self, node_ref: NodeRef) -> Result<Value> {
        let value = if let Some(val_ref) = self.get_edge_target(node_ref, &ExecEdgeType::ValueEdge) {
            self.execute_node(val_ref)?
        } else {
            Value::none()
        };
        Err(GraphoidError::ReturnControl { value })
    }

    // --- Function declaration ---

    fn exec_func_decl(&mut self, node_ref: NodeRef) -> Result<Value> {
        let name = self.get_str_property(node_ref, "name")
            .ok_or_else(|| GraphoidError::runtime("Missing function name".to_string()))?;

        let body_ref = self.get_edge_target(node_ref, &ExecEdgeType::Body)
            .ok_or_else(|| GraphoidError::runtime("Missing function body".to_string()))?;

        // Read parameters from graph
        let param_refs = self.get_ordered_edges(node_ref, "Parameter");
        let mut param_names = Vec::new();
        let mut parameters = Vec::new();

        for param_ref in &param_refs {
            let param_name = self.get_str_property(*param_ref, "name")
                .ok_or_else(|| GraphoidError::runtime("Missing parameter name".to_string()))?;
            let is_variadic = match self.get_property(*param_ref, "is_variadic") {
                Some(AstProperty::Bool(b)) => b,
                _ => false,
            };
            let has_default = self.get_edge_target(*param_ref, &ExecEdgeType::DefaultValue).is_some();

            param_names.push(param_name.clone());
            parameters.push(Parameter {
                name: param_name,
                default_value: if has_default {
                    // Placeholder — we'll evaluate default at call time from the graph
                    Some(crate::ast::Expr::Literal {
                        value: crate::ast::LiteralValue::None,
                        position: crate::error::SourcePosition::unknown(),
                    })
                } else {
                    None
                },
                is_variadic,
            });
        }

        // Generate unique function ID and store body ref
        let func_id = format!("__graph_func_{}", self.next_func_id);
        self.next_func_id += 1;
        self.graph_function_bodies.insert(func_id.clone(), body_ref);

        // Also store parameter default NodeRefs
        for (i, param_ref) in param_refs.iter().enumerate() {
            if let Some(default_ref) = self.get_edge_target(*param_ref, &ExecEdgeType::DefaultValue) {
                let key = format!("{}__default_{}", func_id, i);
                self.graph_function_bodies.insert(key, default_ref);
            }
        }

        // Create the Function value with a placeholder env
        let env = Rc::new(RefCell::new(self.env.clone()));
        let func = Function {
            name: Some(name.clone()),
            params: param_names,
            parameters,
            body: Vec::new(), // Empty — we use graph_function_bodies instead
            pattern_clauses: None,
            env: env.clone(),
            node_id: Some(func_id),
            is_setter: false,
            is_static: false,
            guard: None,
        };

        // Store in global functions (overloading by arity)
        self.global_functions
            .entry(name.clone())
            .or_default()
            .push(func.clone());

        // Also store in environment so it can be passed as a value
        self.env.define(name, Value::function(func));

        Ok(Value::none())
    }

    // --- Call expression ---

    fn exec_call(&mut self, node_ref: NodeRef) -> Result<Value> {
        let callee_ref = self.get_edge_target(node_ref, &ExecEdgeType::Callee)
            .ok_or_else(|| GraphoidError::runtime("Missing callee".to_string()))?;

        // Evaluate arguments
        let arg_refs = self.get_ordered_edges(node_ref, "Argument");
        let mut arg_values = Vec::new();
        for arg_ref in &arg_refs {
            let val = self.execute_node(*arg_ref)?;
            arg_values.push(val);
        }

        // Check if callee is a builtin name
        let callee_node = self.get_node(callee_ref)?;
        if callee_node.node_type == AstNodeType::Identifier {
            if let Some(name) = callee_node.properties.get("name") {
                if let AstProperty::Str(func_name) = name {
                    let func_name = func_name.clone();

                    // Try builtins first
                    if let Some(result) = self.try_builtin(&func_name, &arg_values)? {
                        return Ok(result);
                    }
                }
            }
        }

        // Evaluate callee as expression (could be a function value)
        let callee_val = self.execute_node(callee_ref)?;

        match &callee_val.kind {
            ValueKind::Function(func) => {
                self.call_graph_function(func.clone(), arg_values)
            }
            _ => Err(GraphoidError::type_error("function", callee_val.type_name())),
        }
    }

    /// Try to call a builtin function. Returns Some(value) if it's a builtin, None if not.
    fn try_builtin(&mut self, name: &str, args: &[Value]) -> Result<Option<Value>> {
        match name {
            "print" => {
                let output: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                println!("{}", output.join(" "));
                Ok(Some(Value::none()))
            }
            "typeof" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime("typeof() requires exactly 1 argument".to_string()));
                }
                Ok(Some(Value::string(args[0].type_name().to_string())))
            }
            "length" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime("length() requires exactly 1 argument".to_string()));
                }
                let len = match &args[0].kind {
                    ValueKind::String(s) => s.len() as f64,
                    ValueKind::List(l) => l.len() as f64,
                    ValueKind::Map(m) => m.len() as f64,
                    _ => return Err(GraphoidError::type_error("string, list, or map", args[0].type_name())),
                };
                Ok(Some(Value::number(len)))
            }
            "string" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime("string() requires exactly 1 argument".to_string()));
                }
                Ok(Some(Value::string(args[0].to_string())))
            }
            "num" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime("num() requires exactly 1 argument".to_string()));
                }
                match &args[0].kind {
                    ValueKind::Number(n) => Ok(Some(Value::number(*n))),
                    ValueKind::String(s) => {
                        let n: f64 = s.parse().map_err(|_| {
                            GraphoidError::runtime(format!("Cannot convert '{}' to number", s))
                        })?;
                        Ok(Some(Value::number(n)))
                    }
                    ValueKind::Boolean(b) => Ok(Some(Value::number(if *b { 1.0 } else { 0.0 }))),
                    _ => Err(GraphoidError::type_error("number, string, or boolean", args[0].type_name())),
                }
            }
            "int" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime("int() requires exactly 1 argument".to_string()));
                }
                match &args[0].kind {
                    ValueKind::Number(n) => Ok(Some(Value::number((*n as i64) as f64))),
                    ValueKind::String(s) => {
                        let n: f64 = s.parse().map_err(|_| {
                            GraphoidError::runtime(format!("Cannot convert '{}' to int", s))
                        })?;
                        Ok(Some(Value::number((n as i64) as f64)))
                    }
                    _ => Err(GraphoidError::type_error("number or string", args[0].type_name())),
                }
            }
            _ => Ok(None), // Not a builtin
        }
    }

    /// Call a graph-based function (one whose body is stored as a NodeRef).
    fn call_graph_function(&mut self, func: Function, args: Vec<Value>) -> Result<Value> {
        // Look up the body NodeRef
        let func_id = func.node_id.as_ref()
            .ok_or_else(|| GraphoidError::runtime("Function has no graph ID".to_string()))?;
        let body_ref = *self.graph_function_bodies.get(func_id)
            .ok_or_else(|| GraphoidError::runtime(format!("Function body not found for {}", func_id)))?;

        // Build call environment from captured env
        let mut call_env = (*func.env.borrow()).clone();

        // Bind parameters
        let _param_count = func.params.len();
        for (i, param_name) in func.params.iter().enumerate() {
            if i < args.len() {
                call_env.define(param_name.clone(), args[i].clone());
            } else {
                // Try default value
                let default_key = format!("{}__default_{}", func_id, i);
                if let Some(default_ref) = self.graph_function_bodies.get(&default_key).copied() {
                    let default_val = self.execute_node(default_ref)?;
                    call_env.define(param_name.clone(), default_val);
                } else {
                    call_env.define(param_name.clone(), Value::none());
                }
            }
        }

        // Swap environments
        let saved_env = std::mem::replace(&mut self.env, call_env);
        self.function_call_depth += 1;

        // Execute body
        // Named functions: only explicit `return` produces a value
        // Lambdas: body expression IS the return value
        let is_lambda = func.name.is_none();
        let result = match self.execute_node(body_ref) {
            Ok(val) => {
                if is_lambda {
                    Ok(val) // Lambda body expression is the return value
                } else {
                    Ok(Value::none()) // Named function without return returns none
                }
            }
            Err(GraphoidError::ReturnControl { value }) => Ok(value),
            Err(e) => Err(e),
        };

        // Restore environment
        self.function_call_depth -= 1;
        let call_env_after = std::mem::replace(&mut self.env, saved_env);

        // Update closure state (for closures that modify captured variables)
        *func.env.borrow_mut() = call_env_after;

        result
    }

    // --- Lambda expression ---

    fn exec_lambda(&mut self, node_ref: NodeRef) -> Result<Value> {
        let param_count = match self.get_property(node_ref, "param_count") {
            Some(AstProperty::Int(n)) => n as usize,
            _ => 0,
        };

        let body_ref = self.get_edge_target(node_ref, &ExecEdgeType::Body)
            .ok_or_else(|| GraphoidError::runtime("Missing lambda body".to_string()))?;

        // Read parameter names
        let mut param_names = Vec::new();
        let mut parameters = Vec::new();
        for i in 0..param_count {
            let key = format!("param_{}", i);
            let name = match self.get_property(node_ref, &key) {
                Some(AstProperty::Str(s)) => s,
                _ => format!("arg{}", i),
            };
            param_names.push(name.clone());
            parameters.push(Parameter {
                name,
                default_value: None,
                is_variadic: false,
            });
        }

        // Generate unique function ID and store body ref
        let func_id = format!("__graph_lambda_{}", self.next_func_id);
        self.next_func_id += 1;
        self.graph_function_bodies.insert(func_id.clone(), body_ref);

        // Capture current environment
        let env = Rc::new(RefCell::new(self.env.clone()));

        let func = Function {
            name: None,
            params: param_names,
            parameters,
            body: Vec::new(),
            pattern_clauses: None,
            env,
            node_id: Some(func_id),
            is_setter: false,
            is_static: false,
            guard: None,
        };

        Ok(Value::function(func))
    }

    // --- Method call ---

    fn exec_method_call(&mut self, node_ref: NodeRef) -> Result<Value> {
        let method_name = self.get_str_property(node_ref, "method")
            .ok_or_else(|| GraphoidError::runtime("Missing method name".to_string()))?;

        let obj_ref = self.get_edge_target(node_ref, &ExecEdgeType::Object)
            .ok_or_else(|| GraphoidError::runtime("Missing method call object".to_string()))?;
        let object = self.execute_node(obj_ref)?;

        // Evaluate arguments
        let arg_refs = self.get_ordered_edges(node_ref, "Argument");
        let mut args = Vec::new();
        for arg_ref in &arg_refs {
            let val = self.execute_node(*arg_ref)?;
            args.push(val);
        }

        self.dispatch_method(object, &method_name, args)
    }

    /// Dispatch a method call on a value.
    fn dispatch_method(&mut self, object: Value, method: &str, args: Vec<Value>) -> Result<Value> {
        match &object.kind {
            ValueKind::String(s) => self.dispatch_string_method(s.clone(), method, &args),
            ValueKind::List(_) => self.dispatch_list_method(object.clone(), method, args),
            ValueKind::Map(_) => self.dispatch_map_method(object.clone(), method, &args),
            _ => Err(GraphoidError::runtime(format!(
                "Cannot call method '{}' on type '{}'", method, object.type_name()
            ))),
        }
    }

    fn dispatch_string_method(&self, s: String, method: &str, args: &[Value]) -> Result<Value> {
        match method {
            "length" => Ok(Value::number(s.len() as f64)),
            "upper" => Ok(Value::string(s.to_uppercase())),
            "lower" => Ok(Value::string(s.to_lowercase())),
            "trim" => Ok(Value::string(s.trim().to_string())),
            "contains" => {
                let substr = match args.first() {
                    Some(v) => v.to_string(),
                    None => return Err(GraphoidError::runtime("contains() requires 1 argument".to_string())),
                };
                Ok(Value::boolean(s.contains(&substr)))
            }
            "starts_with" => {
                let prefix = match args.first() {
                    Some(v) => v.to_string(),
                    None => return Err(GraphoidError::runtime("starts_with() requires 1 argument".to_string())),
                };
                Ok(Value::boolean(s.starts_with(&prefix)))
            }
            "ends_with" => {
                let suffix = match args.first() {
                    Some(v) => v.to_string(),
                    None => return Err(GraphoidError::runtime("ends_with() requires 1 argument".to_string())),
                };
                Ok(Value::boolean(s.ends_with(&suffix)))
            }
            "split" => {
                let delimiter = match args.first() {
                    Some(v) => v.to_string(),
                    None => return Err(GraphoidError::runtime("split() requires 1 argument".to_string())),
                };
                let parts: Vec<Value> = s.split(&delimiter).map(|p| Value::string(p.to_string())).collect();
                Ok(Value::list(crate::values::List::from_vec(parts)))
            }
            "replace" => {
                if args.len() < 2 {
                    return Err(GraphoidError::runtime("replace() requires 2 arguments".to_string()));
                }
                let from = args[0].to_string();
                let to = args[1].to_string();
                Ok(Value::string(s.replace(&from, &to)))
            }
            _ => Err(GraphoidError::runtime(format!("Unknown string method: {}", method))),
        }
    }

    fn dispatch_list_method(&mut self, object: Value, method: &str, args: Vec<Value>) -> Result<Value> {
        match method {
            "length" => {
                if let ValueKind::List(ref l) = object.kind {
                    Ok(Value::number(l.len() as f64))
                } else {
                    unreachable!()
                }
            }
            "append" => {
                // Clone the list, append, return new list (Graphoid is immutable by default)
                if let ValueKind::List(ref l) = object.kind {
                    let mut new_list = l.clone();
                    if let Some(val) = args.first() {
                        new_list.append_raw(val.clone())?;
                    }
                    Ok(Value::list(new_list))
                } else {
                    unreachable!()
                }
            }
            "map" => {
                if let ValueKind::List(ref l) = object.kind {
                    let items = l.to_vec();
                    let mut result = Vec::new();
                    if let Some(func_val) = args.first() {
                        if let ValueKind::Function(ref func) = func_val.kind {
                            for item in items {
                                let mapped = self.call_graph_function(func.clone(), vec![item])?;
                                result.push(mapped);
                            }
                        }
                    }
                    Ok(Value::list(crate::values::List::from_vec(result)))
                } else {
                    unreachable!()
                }
            }
            "filter" => {
                if let ValueKind::List(ref l) = object.kind {
                    let items = l.to_vec();
                    let mut result = Vec::new();
                    if let Some(func_val) = args.first() {
                        if let ValueKind::Function(ref func) = func_val.kind {
                            for item in items {
                                let keep = self.call_graph_function(func.clone(), vec![item.clone()])?;
                                if keep.is_truthy() {
                                    result.push(item);
                                }
                            }
                        }
                    }
                    Ok(Value::list(crate::values::List::from_vec(result)))
                } else {
                    unreachable!()
                }
            }
            _ => Err(GraphoidError::runtime(format!("Unknown list method: {}", method))),
        }
    }

    fn dispatch_map_method(&self, object: Value, method: &str, _args: &[Value]) -> Result<Value> {
        match method {
            "keys" => {
                if let ValueKind::Map(ref m) = object.kind {
                    let keys: Vec<Value> = m.keys().into_iter().map(|k| Value::string(k)).collect();
                    Ok(Value::list(crate::values::List::from_vec(keys)))
                } else {
                    unreachable!()
                }
            }
            "values" => {
                if let ValueKind::Map(ref m) = object.kind {
                    let vals: Vec<Value> = m.values().into_iter().collect();
                    Ok(Value::list(crate::values::List::from_vec(vals)))
                } else {
                    unreachable!()
                }
            }
            "length" => {
                if let ValueKind::Map(ref m) = object.kind {
                    Ok(Value::number(m.len() as f64))
                } else {
                    unreachable!()
                }
            }
            _ => Err(GraphoidError::runtime(format!("Unknown map method: {}", method))),
        }
    }

    // --- Index expression ---

    fn exec_index(&mut self, node_ref: NodeRef) -> Result<Value> {
        let obj_ref = self.get_edge_target(node_ref, &ExecEdgeType::Object)
            .ok_or_else(|| GraphoidError::runtime("Missing index object".to_string()))?;
        let idx_ref = self.get_edge_target(node_ref, &ExecEdgeType::ValueEdge)
            .ok_or_else(|| GraphoidError::runtime("Missing index value".to_string()))?;

        let object = self.execute_node(obj_ref)?;
        let index = self.execute_node(idx_ref)?;

        match (&object.kind, &index.kind) {
            (ValueKind::List(items), ValueKind::Number(n)) => {
                let idx = *n as i64;
                let len = items.len() as i64;
                let actual_idx = if idx < 0 { len + idx } else { idx } as usize;
                items.get(actual_idx).cloned()
                    .ok_or_else(|| GraphoidError::runtime(format!("List index {} out of bounds", n)))
            }
            (ValueKind::Map(m), ValueKind::String(key)) => {
                m.get(key).cloned()
                    .ok_or_else(|| GraphoidError::runtime(format!("Key '{}' not found", key)))
            }
            (ValueKind::String(s), ValueKind::Number(n)) => {
                let idx = *n as i64;
                let len = s.len() as i64;
                let actual_idx = if idx < 0 { len + idx } else { idx } as usize;
                s.chars().nth(actual_idx)
                    .map(|c| Value::string(c.to_string()))
                    .ok_or_else(|| GraphoidError::runtime(format!("String index {} out of bounds", n)))
            }
            _ => Err(GraphoidError::type_error(
                "indexable type (list, map, or string)",
                object.type_name(),
            )),
        }
    }

    // --- Property access ---

    fn exec_property_access(&mut self, node_ref: NodeRef) -> Result<Value> {
        let property = self.get_str_property(node_ref, "property")
            .ok_or_else(|| GraphoidError::runtime("Missing property name".to_string()))?;
        let obj_ref = self.get_edge_target(node_ref, &ExecEdgeType::Object)
            .ok_or_else(|| GraphoidError::runtime("Missing property access object".to_string()))?;
        let object = self.execute_node(obj_ref)?;

        match &object.kind {
            ValueKind::Map(m) => {
                m.get(&property).cloned()
                    .ok_or_else(|| GraphoidError::runtime(format!("Property '{}' not found", property)))
            }
            _ => Err(GraphoidError::runtime(format!(
                "Cannot access property '{}' on type '{}'", property, object.type_name()
            ))),
        }
    }

    // --- Try/catch/finally ---

    fn exec_try(&mut self, node_ref: NodeRef) -> Result<Value> {
        let body_ref = self.get_edge_target(node_ref, &ExecEdgeType::Body)
            .ok_or_else(|| GraphoidError::runtime("Missing try body".to_string()))?;
        let finally_ref = self.get_edge_target(node_ref, &ExecEdgeType::FinallyBlock);
        let catch_refs = self.get_ordered_edges(node_ref, "CatchHandler");

        // Execute try body
        let result = match self.execute_node(body_ref) {
            Ok(val) => Ok(val),
            Err(e) => {
                // Check for control flow errors that should not be caught
                match &e {
                    GraphoidError::LoopControl { .. } | GraphoidError::ReturnControl { .. } => {
                        // Run finally if present, then re-propagate
                        if let Some(fin_ref) = finally_ref {
                            let _ = self.execute_node(fin_ref);
                        }
                        return Err(e);
                    }
                    _ => {}
                }

                // Try each catch handler
                let mut caught = false;
                let mut catch_result = Ok(Value::none());
                for catch_ref in &catch_refs {
                    let error_type = self.get_str_property(*catch_ref, "error_type");
                    let variable = self.get_str_property(*catch_ref, "variable");
                    let catch_body_ref = self.get_edge_target(*catch_ref, &ExecEdgeType::Body);

                    // Check if this catch matches the error type
                    let matches = match &error_type {
                        Some(et) => e.error_type() == *et || et == "RuntimeError",
                        None => true, // Bare catch catches everything
                    };

                    if matches {
                        caught = true;
                        // Bind error to variable if specified
                        if let Some(var_name) = variable {
                            let error_val = Value::string(format!("{}", e));
                            self.env.define(var_name, error_val);
                        }
                        if let Some(body_ref) = catch_body_ref {
                            catch_result = self.execute_node(body_ref);
                        }
                        break;
                    }
                }

                if caught {
                    catch_result
                } else {
                    Err(e)
                }
            }
        };

        // Always run finally block
        if let Some(fin_ref) = finally_ref {
            let _ = self.execute_node(fin_ref);
        }

        result
    }

    // --- Raise ---

    fn exec_raise(&mut self, node_ref: NodeRef) -> Result<Value> {
        let val_ref = self.get_edge_target(node_ref, &ExecEdgeType::ValueEdge)
            .ok_or_else(|| GraphoidError::runtime("Missing raise value".to_string()))?;
        let value = self.execute_node(val_ref)?;

        let message = match &value.kind {
            ValueKind::String(s) => s.clone(),
            _ => value.to_string(),
        };

        Err(GraphoidError::runtime(message))
    }

    // --- Conditional expression (ternary: value if condition else other) ---

    fn exec_conditional(&mut self, node_ref: NodeRef) -> Result<Value> {
        let is_unless = match self.get_property(node_ref, "is_unless") {
            Some(AstProperty::Bool(b)) => b,
            _ => false,
        };

        let cond_ref = self.get_edge_target(node_ref, &ExecEdgeType::Condition)
            .ok_or_else(|| GraphoidError::runtime("Missing conditional condition".to_string()))?;
        let then_ref = self.get_edge_target(node_ref, &ExecEdgeType::ThenBranch)
            .ok_or_else(|| GraphoidError::runtime("Missing conditional then branch".to_string()))?;

        let cond_val = self.execute_node(cond_ref)?;
        let condition_met = if is_unless { !cond_val.is_truthy() } else { cond_val.is_truthy() };

        if condition_met {
            self.execute_node(then_ref)
        } else if let Some(else_ref) = self.get_edge_target(node_ref, &ExecEdgeType::ElseBranch) {
            self.execute_node(else_ref)
        } else {
            Ok(Value::none())
        }
    }
}

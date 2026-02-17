//! Graph-based executor: traverses an ExecutionGraph to produce Values.
//!
//! Phase 16: Replaces the tree-walking interpreter with graph traversal.

use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;

use crate::ast::{BinaryOp, Expr, Parameter, Program, Stmt};
use crate::error::{GraphoidError, SourcePosition, Result};
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
    #[allow(dead_code)]
    pub(crate) writeback_stack: Vec<Vec<()>>, // Required by method files; mutation writeback not yet needed
    pub(crate) block_self_stack: Vec<Value>,
    pub(crate) graph_method_value_stack: Vec<Value>,
    pub(crate) suppress_self_property_assignment: usize,
    pub(crate) function_call_depth: usize,
    /// The execution graph being traversed (set during execution)
    graph: Option<ExecutionGraph>,
    /// Maps function IDs to their body NodeRef (for graph-based function execution)
    graph_function_bodies: HashMap<String, NodeRef>,
    /// Maps function IDs to their pattern clauses (for pattern-matching functions)
    graph_pattern_clauses: HashMap<String, Vec<GraphPatternClause>>,
    /// Maps function IDs to their guard NodeRef (for structure-based dispatch)
    pub(crate) graph_method_guards: HashMap<String, NodeRef>,
    /// Counter for generating unique function IDs
    next_func_id: usize,
    /// Maps Function.node_id -> FunctionGraph node_id for identity-based lambda tracking
    func_to_fg_id: HashMap<String, String>,
    /// Phase 17: when true, variable definitions/assignments are tracked as private
    in_priv_block: bool,
    /// Phase 18: persistent universe graph (type hierarchy + modules + import edges)
    universe_graph: Rc<RefCell<crate::values::graph::Graph>>,
}

/// A pattern clause stored as graph references (for pattern-matching functions).
#[derive(Debug, Clone)]
struct GraphPatternClause {
    pattern_type: String,       // "literal", "variable", "wildcard"
    pattern_value: Option<AstProperty>,  // For literal patterns
    pattern_name: Option<String>,        // For variable patterns
    guard_ref: Option<NodeRef>,
    body_ref: NodeRef,
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
            graph_pattern_clauses: HashMap::new(),
            graph_method_guards: HashMap::new(),
            next_func_id: 0,
            func_to_fg_id: HashMap::new(),
            in_priv_block: false,
            universe_graph: Rc::new(RefCell::new(Self::build_initial_universe_graph())),
        }
    }

    /// Build the initial universe graph with the built-in type hierarchy.
    fn build_initial_universe_graph() -> crate::values::graph::Graph {
        use crate::values::graph::{Graph, GraphType};
        let mut g = Graph::new(GraphType::Directed);

        let type_nodes = [
            "type:any", "type:num", "type:bignum",
            "type:string", "type:bool", "type:none", "type:symbol",
            "type:collection", "type:list", "type:map", "type:graph",
            "type:function", "type:module", "type:error", "type:time",
        ];
        for tn in &type_nodes {
            let _ = g.add_node(tn.to_string(), Value::string(tn.to_string()));
        }

        let subtypes: &[(&str, &str)] = &[
            ("type:num", "type:any"),
            ("type:bignum", "type:num"),
            ("type:string", "type:any"),
            ("type:bool", "type:any"),
            ("type:none", "type:any"),
            ("type:symbol", "type:any"),
            ("type:collection", "type:any"),
            ("type:list", "type:collection"),
            ("type:map", "type:collection"),
            ("type:graph", "type:collection"),
            ("type:function", "type:any"),
            ("type:module", "type:any"),
            ("type:error", "type:any"),
            ("type:time", "type:any"),
        ];
        for (child, parent) in subtypes {
            let _ = g.add_edge(child, parent, "subtype_of".to_string(), None, HashMap::new());
        }

        // Add scope:main node (source of import edges)
        let _ = g.add_node("scope:main".to_string(), Value::string("main".to_string()));

        // Error type hierarchy (subtypes of type:error)
        let error_types = [
            "RuntimeError", "ValueError", "TypeError",
            "IOError", "NetworkError", "ParseError",
        ];
        for et in &error_types {
            let node_id = format!("error:{}", et);
            let _ = g.add_node(node_id.clone(), Value::string(et.to_string()));
            let _ = g.add_edge(&node_id, "type:error", "subtype_of".to_string(), None, HashMap::new());
        }
        // IOError subtypes
        for sub in &["FileError", "NetError"] {
            let node_id = format!("error:{}", sub);
            let _ = g.add_node(node_id.clone(), Value::string(sub.to_string()));
            let _ = g.add_edge(&node_id, "error:IOError", "subtype_of".to_string(), None, HashMap::new());
        }

        g
    }

    /// Register a module node in the persistent universe graph (idempotent).
    fn register_module_in_universe(&self, module: &crate::execution::module_manager::Module) {
        let display_name = module.alias.clone().unwrap_or_else(|| module.name.clone());
        let node_id = format!("module:{}", display_name);
        let mut ug = self.universe_graph.borrow_mut();
        if !ug.has_node(&node_id) {
            let _ = ug.add_node(node_id, Value::string(display_name));
        }
    }

    /// Convert a rule symbol name to a RuleSpec.
    pub(crate) fn symbol_to_rule_spec(symbol: &str, param: Option<f64>) -> Result<crate::graph::RuleSpec> {
        use crate::graph::RuleSpec;
        match (symbol, param) {
            ("no_cycles", None) => Ok(RuleSpec::NoCycles),
            ("single_root", None) => Ok(RuleSpec::SingleRoot),
            ("connected", None) => Ok(RuleSpec::Connected),
            ("binary_tree", None) => Ok(RuleSpec::BinaryTree),
            ("no_dups" | "no_duplicates", None) => Ok(RuleSpec::NoDuplicates),
            ("max_degree", Some(n)) => Ok(RuleSpec::MaxDegree(n as usize)),
            ("weighted_edges", None) => Ok(RuleSpec::WeightedEdges),
            ("unweighted_edges", None) => Ok(RuleSpec::UnweightedEdges),
            ("none_to_zero", None) => Ok(RuleSpec::NoneToZero),
            ("none_to_empty", None) => Ok(RuleSpec::NoneToEmpty),
            ("positive", None) => Ok(RuleSpec::Positive),
            ("round_to_int", None) => Ok(RuleSpec::RoundToInt),
            ("uppercase", None) => Ok(RuleSpec::Uppercase),
            ("lowercase", None) => Ok(RuleSpec::Lowercase),
            ("no_frozen", None) => Ok(RuleSpec::NoFrozen),
            ("copy_elements", None) => Ok(RuleSpec::CopyElements),
            ("shallow_freeze_only", None) => Ok(RuleSpec::ShallowFreezeOnly),
            ("no_node_removals", None) => Ok(RuleSpec::NoNodeRemovals),
            ("no_edge_removals", None) => Ok(RuleSpec::NoEdgeRemovals),
            ("read_only", None) => Ok(RuleSpec::ReadOnly),
            (name, None) => Err(GraphoidError::runtime(format!("Unknown rule: :{}", name))),
            (name, Some(_)) => Err(GraphoidError::runtime(format!("Rule :{} does not accept parameters", name))),
        }
    }

    /// Execute source code: lex → parse → convert → execute. Returns the final value.
    pub fn execute_source_value(&mut self, source: &str) -> Result<Value> {
        let tokens = Lexer::new(source).tokenize()
            .map_err(|e| GraphoidError::runtime(format!("Lexer error: {}", e)))?;
        let program = Parser::new(tokens).parse()
            .map_err(|e| GraphoidError::runtime(format!("Parser error: {}", e)))?;

        // Register and push a toplevel function for function graph tracking
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
        self.function_graph.borrow_mut().push_call(toplevel_id, Vec::new());

        let mut converter = AstToGraphConverter::new();
        let root = converter.convert_program(&program);
        let exec_graph = converter.into_graph();

        let result = self.execute(exec_graph, root);

        self.function_graph.borrow_mut().pop_call(Value::none());

        result
    }

    /// Execute source code (API-compatible with Executor). Returns Result<()>.
    pub fn execute_source(&mut self, source: &str) -> Result<()> {
        self.execute_source_value(source)?;
        Ok(())
    }

    /// Execute an execution graph from the given root node.
    /// If a graph already exists, merge the new one in (preserving function bodies etc.).
    pub fn execute(&mut self, graph: ExecutionGraph, root: NodeRef) -> Result<Value> {
        if let Some(ref mut existing) = self.graph {
            // Merge new graph into existing, get remapped root
            let remapped_root = existing.merge(graph)
                .ok_or_else(|| GraphoidError::runtime("Merged graph has no root".to_string()))?;
            self.execute_node(remapped_root)
        } else {
            self.graph = Some(graph);
            self.execute_node(root)
        }
    }

    /// Get a variable value from the environment.
    pub fn get_variable(&self, name: &str) -> Option<Value> {
        self.env.get(name).ok()
    }

    /// Execute a single node by dispatching on its type.
    pub(crate) fn execute_node(&mut self, node_ref: NodeRef) -> Result<Value> {
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

            // Module system
            AstNodeType::ImportStmt => self.exec_import(node_ref),
            AstNodeType::ModuleDeclStmt => self.exec_module_decl(node_ref),
            AstNodeType::LoadStmt => self.exec_load(node_ref),

            // Configuration
            AstNodeType::ConfigureStmt => self.exec_configure(node_ref),
            AstNodeType::PrecisionStmt => self.exec_precision(node_ref),

            // Graph declarations and expressions
            AstNodeType::GraphDeclStmt => self.exec_graph_decl(node_ref),
            AstNodeType::GraphExpr => self.exec_graph_expr(node_ref),
            AstNodeType::InstantiateExpr => self.exec_instantiate(node_ref),

            // Pattern matching
            AstNodeType::MatchExpr => self.exec_match(node_ref),

            // Super method calls
            AstNodeType::SuperMethodCallExpr => self.exec_super_method_call(node_ref),

            // Phase 17: Privacy block
            AstNodeType::PrivBlockStmt => self.exec_priv_block(node_ref),

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

    /// Write back a value to either an environment variable or an implicit self property.
    /// Used by mutating methods (bang methods) to handle both regular variables and
    /// graph properties accessed via implicit self.
    fn set_variable_or_self_property(&mut self, name: &str, value: Value) -> Result<()> {
        if self.env.exists(name) {
            self.env.set(name, value)?;
        } else if self.suppress_self_property_assignment == 0 {
            if let Ok(self_value) = self.env.get("self") {
                if let ValueKind::Graph(ref graph_rc) = self_value.kind {
                    let property_node_id = crate::values::Graph::property_node_id(name);
                    if graph_rc.borrow().has_node(&property_node_id) {
                        graph_rc.borrow_mut().add_node(property_node_id, value).ok();
                        return Ok(());
                    }
                }
            }
            // Variable doesn't exist anywhere — define it
            self.env.define(name.to_string(), value);
        } else {
            self.env.define(name.to_string(), value);
        }
        Ok(())
    }

    fn get_str_property(&self, node_ref: NodeRef, key: &str) -> Option<String> {
        match self.get_property(node_ref, key)? {
            AstProperty::Str(s) => Some(s),
            _ => None,
        }
    }

    fn get_bool_property(&self, node_ref: NodeRef, key: &str) -> Option<bool> {
        match self.get_property(node_ref, key)? {
            AstProperty::Bool(b) => Some(b),
            _ => None,
        }
    }

    // --- Literal execution ---

    fn exec_number_lit(&self, node_ref: NodeRef) -> Result<Value> {
        match self.get_property(node_ref, "value") {
            Some(AstProperty::Num(n)) => {
                use crate::execution::config::PrecisionMode;
                use crate::values::BigNum;
                match self.config_stack.current().precision_mode {
                    PrecisionMode::High => {
                        if self.config_stack.current().integer_mode {
                            const I64_MAX: f64 = 9_223_372_036_854_775_807.0;
                            const U64_MAX: f64 = 18_446_744_073_709_551_615.0;
                            let exceeds = if self.config_stack.current().unsigned_mode {
                                n.abs() > U64_MAX
                            } else {
                                n.abs() > I64_MAX
                            };
                            if exceeds && n.fract() == 0.0 {
                                use num_bigint::BigInt;
                                let big_int = BigInt::from(n as i64);
                                Ok(Value::bignum(BigNum::BigInt(big_int)))
                            } else if self.config_stack.current().unsigned_mode {
                                Ok(Value::bignum(BigNum::UInt64(n as u64)))
                            } else {
                                Ok(Value::bignum(BigNum::Int64(n as i64)))
                            }
                        } else {
                            use f128::f128;
                            Ok(Value::bignum(BigNum::Float128(f128::from(n))))
                        }
                    }
                    PrecisionMode::Extended => {
                        use num_bigint::BigInt;
                        let big_int = if n.fract() == 0.0 {
                            BigInt::from(n as i64)
                        } else {
                            // Fractional value - truncate to integer for BigInt
                            BigInt::from(n.trunc() as i64)
                        };
                        Ok(Value::bignum(BigNum::BigInt(big_int)))
                    }
                    PrecisionMode::Standard => Ok(Value::number(n)),
                }
            }
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

        // Implicit self: check if `self` is a graph with this property
        if let Ok(self_value) = self.env.get("self") {
            if let ValueKind::Graph(ref graph_rc) = self_value.kind {
                let graph = graph_rc.borrow();
                let property_node_id = crate::values::Graph::property_node_id(&name);
                if let Some(val) = graph.get_node(&property_node_id) {
                    return Ok(val.clone());
                }
                // Also check for methods
                let method_node_id = format!("__methods__/{}", name);
                if let Some(val) = graph.get_node(&method_node_id) {
                    return Ok(val.clone());
                }
            }
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

    /// Delegates to arithmetic.rs for full implementations.
    fn eval_binary_op(&mut self, op: &BinaryOp, left: Value, right: Value) -> Result<Value> {
        match op {
            BinaryOp::BitwiseAnd => self.eval_bitwise_and(left, right),
            BinaryOp::BitwiseOr => self.eval_bitwise_or(left, right),
            BinaryOp::BitwiseXor => self.eval_bitwise_xor(left, right),
            BinaryOp::LeftShift => self.eval_left_shift(left, right),
            BinaryOp::RightShift => self.eval_right_shift(left, right),
            BinaryOp::DotAdd => self.eval_element_wise(left, right, BinaryOp::Add),
            BinaryOp::DotSubtract => self.eval_element_wise(left, right, BinaryOp::Subtract),
            BinaryOp::DotMultiply => self.eval_element_wise(left, right, BinaryOp::Multiply),
            BinaryOp::DotDivide => self.eval_element_wise(left, right, BinaryOp::Divide),
            BinaryOp::DotIntDiv => self.eval_element_wise(left, right, BinaryOp::IntDiv),
            BinaryOp::DotPower => self.eval_element_wise(left, right, BinaryOp::Power),
            BinaryOp::DotEqual => self.eval_element_wise(left, right, BinaryOp::Equal),
            BinaryOp::DotNotEqual => self.eval_element_wise(left, right, BinaryOp::NotEqual),
            BinaryOp::DotLess => self.eval_element_wise(left, right, BinaryOp::Less),
            BinaryOp::DotLessEqual => self.eval_element_wise(left, right, BinaryOp::LessEqual),
            BinaryOp::DotGreater => self.eval_element_wise(left, right, BinaryOp::Greater),
            BinaryOp::DotGreaterEqual => self.eval_element_wise(left, right, BinaryOp::GreaterEqual),
            _ => self.apply_scalar_op(left, right, op),
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
                    ValueKind::BigNumber(bn) => {
                        use crate::values::BigNum;
                        match bn {
                            BigNum::Int64(i) => Ok(Value::bignum(BigNum::Int64(-i))),
                            BigNum::UInt64(u) => Ok(Value::bignum(BigNum::Int64(-(*u as i64)))),
                            BigNum::Float128(f) => Ok(Value::bignum(BigNum::Float128(-*f))),
                            BigNum::BigInt(bi) => Ok(Value::bignum(BigNum::BigInt(-bi.clone()))),
                        }
                    }
                    _ => Err(GraphoidError::type_error("number", &operand.type_name())),
                }
            }
            crate::ast::UnaryOp::Not => {
                Ok(Value::boolean(!operand.is_truthy()))
            }
            crate::ast::UnaryOp::BitwiseNot => {
                match &operand.kind {
                    ValueKind::Number(n) => Ok(Value::number(!(*n as i64) as f64)),
                    ValueKind::BigNumber(bn) => {
                        use crate::values::BigNum;
                        match bn {
                            BigNum::Int64(i) => Ok(Value::bignum(BigNum::Int64(!i))),
                            BigNum::UInt64(u) => Ok(Value::bignum(BigNum::UInt64(!u))),
                            _ => Err(GraphoidError::runtime("Bitwise NOT only supported on integer bignums".to_string())),
                        }
                    }
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
        let is_private = self.get_bool_property(node_ref, "is_private").unwrap_or(false);
        let type_base = self.get_str_property(node_ref, "type_base");
        let val_ref = self.get_edge_target(node_ref, &ExecEdgeType::ValueEdge)
            .ok_or_else(|| GraphoidError::runtime("Missing variable value".to_string()))?;
        let mut value = self.execute_node(val_ref)?;

        // Handle type annotations that require value conversion
        if let Some(ref tb) = type_base {
            match tb.as_str() {
                "bignum" => {
                    value = self.convert_to_bignum(value)?;
                }
                _ => {} // Other type annotations are just for documentation/checking
            }
        }

        // Truncate if integer mode is active
        value = self.truncate_if_integer_mode(value);

        // Track private symbols for module exports
        if is_private || self.in_priv_block {
            self.private_symbols.insert(name.clone());
        }

        self.env.define(name, value.clone());
        Ok(value)
    }

    /// Truncates numeric values when integer_mode is active
    fn truncate_if_integer_mode(&self, value: Value) -> Value {
        if !self.config_stack.current().integer_mode {
            return value;
        }
        match &value.kind {
            ValueKind::Number(n) => Value::number(n.trunc()),
            ValueKind::BigNumber(bn) => {
                use crate::values::BigNum;
                match bn {
                    BigNum::Float128(f) => {
                        let f64_val: f64 = (*f).into();
                        use f128::f128;
                        Value::bignum(BigNum::Float128(f128::from(f64_val.trunc())))
                    }
                    BigNum::Int64(_) | BigNum::UInt64(_) | BigNum::BigInt(_) => value,
                }
            }
            _ => value,
        }
    }

    fn convert_to_bignum(&self, value: Value) -> Result<Value> {
        use crate::values::BigNum;
        match &value.kind {
            ValueKind::Number(n) => {
                use f128::f128;
                let f128_val = f128::from(*n);
                Ok(Value::bignum(BigNum::Float128(f128_val)))
            }
            ValueKind::BigNumber(_) => Ok(value),
            _ => Err(GraphoidError::type_error("number or bignum", value.type_name())),
        }
    }

    fn check_has_frozen(&self, value: &Value) -> bool {
        if value.is_frozen() {
            return true;
        }
        match &value.kind {
            ValueKind::List(list) => {
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
                for key in hash.keys() {
                    if let Some(val) = hash.get(&key) {
                        if self.check_has_frozen(&val) {
                            return true;
                        }
                    }
                }
                false
            }
            _ => false,
        }
    }

    fn eval_has_frozen_count(&self, value: &Value, deep: bool) -> Result<Value> {
        let mut frozen_count = 0;
        let mut frozen_collections = 0;
        let mut frozen_primitives = 0;
        self.count_frozen(value, &mut frozen_count, &mut frozen_collections, &mut frozen_primitives, deep);
        let mut result = crate::values::Hash::new();
        result.insert("has_frozen".to_string(), Value::boolean(frozen_count > 0)).unwrap();
        result.insert("frozen_count".to_string(), Value::number(frozen_count as f64)).unwrap();
        result.insert("frozen_collections".to_string(), Value::number(frozen_collections as f64)).unwrap();
        result.insert("frozen_primitives".to_string(), Value::number(frozen_primitives as f64)).unwrap();
        Ok(Value::map(result))
    }

    fn count_frozen(&self, value: &Value, total: &mut usize, collections: &mut usize, primitives: &mut usize, recursive: bool) {
        match &value.kind {
            ValueKind::List(list) => {
                for i in 0..list.len() {
                    if let Some(elem) = list.get(i) {
                        if elem.is_frozen() {
                            *total += 1;
                            match &elem.kind {
                                ValueKind::List(_) | ValueKind::Map(_) | ValueKind::Graph(_) => *collections += 1,
                                _ => *primitives += 1,
                            }
                        }
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
                                ValueKind::List(_) | ValueKind::Map(_) | ValueKind::Graph(_) => *collections += 1,
                                _ => *primitives += 1,
                            }
                        }
                        if recursive {
                            self.count_frozen(&val, total, collections, primitives, recursive);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // --- Assignment ---

    fn exec_assign(&mut self, node_ref: NodeRef) -> Result<Value> {
        let target_type = self.get_str_property(node_ref, "target_type")
            .ok_or_else(|| GraphoidError::runtime("Missing assignment target type".to_string()))?;

        let val_ref = self.get_edge_target(node_ref, &ExecEdgeType::ValueEdge)
            .ok_or_else(|| GraphoidError::runtime("Missing assignment value".to_string()))?;
        let value = self.execute_node(val_ref)?;
        let value = self.truncate_if_integer_mode(value);

        match target_type.as_str() {
            "variable" => {
                let name = self.get_str_property(node_ref, "target_name")
                    .ok_or_else(|| GraphoidError::runtime("Missing target name".to_string()))?;
                // Track private symbols inside priv { } blocks
                if self.in_priv_block {
                    self.private_symbols.insert(name.clone());
                }
                if self.env.exists(&name) {
                    self.env.set(&name, value.clone())?;
                } else {
                    // Implicit self: check if `self` is a graph with this property
                    let assigned_to_self = if self.suppress_self_property_assignment == 0 {
                        if let Ok(self_value) = self.env.get("self") {
                            if let ValueKind::Graph(ref graph_rc) = self_value.kind {
                                let property_node_id = crate::values::Graph::property_node_id(&name);
                                let has_property = graph_rc.borrow().has_node(&property_node_id);
                                if has_property {
                                    graph_rc.borrow_mut().add_node(property_node_id, value.clone()).ok();
                                    true
                                } else {
                                    false
                                }
                            } else { false }
                        } else { false }
                    } else { false };
                    if !assigned_to_self {
                        self.env.define(name, value.clone());
                    }
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
                    (ValueKind::Graph(ref g), ValueKind::String(key)) => {
                        g.borrow_mut().add_node(key.clone(), value.clone())?;
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
                    ValueKind::Graph(ref g) => {
                        // Set property in __properties__/ branch
                        let prop_node_id = crate::values::Graph::property_node_id(&prop_name);
                        g.borrow_mut().add_node(prop_node_id, value.clone()).ok();
                        // Graph is Rc<RefCell> so no writeback needed
                        return Ok(value);
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
        let is_private = self.get_bool_property(node_ref, "is_private").unwrap_or(false);
        let receiver = self.get_str_property(node_ref, "receiver");
        let is_static = self.get_bool_property(node_ref, "is_static").unwrap_or(false);

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

        // Read pattern clauses from graph
        let clause_refs = self.get_ordered_edges_cloned(node_ref, "PatternClause");
        let has_pattern_clauses = !clause_refs.is_empty();
        if has_pattern_clauses {
            let mut clauses = Vec::new();
            for clause_ref in clause_refs {
                let pattern_ref = self.get_edge_target(clause_ref, &ExecEdgeType::ArmPattern);
                let body_ref = self.get_edge_target(clause_ref, &ExecEdgeType::ArmBody);
                let guard_ref = self.get_edge_target(clause_ref, &ExecEdgeType::Guard);

                if let (Some(pat_ref), Some(bod_ref)) = (pattern_ref, body_ref) {
                    let pattern_type = self.get_str_property(pat_ref, "pattern_type")
                        .unwrap_or_default();
                    let pattern_value = self.get_property(pat_ref, "value");
                    let pattern_name = self.get_str_property(pat_ref, "name");

                    clauses.push(GraphPatternClause {
                        pattern_type,
                        pattern_value,
                        pattern_name,
                        guard_ref,
                        body_ref: bod_ref,
                    });
                }
            }
            if !clauses.is_empty() {
                self.graph_pattern_clauses.insert(func_id.clone(), clauses);
            }
        }

        // Create the Function value with the current captured env
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
            is_static,
            guard: None,
        };

        // Register in function graph for tracking
        self.function_graph.borrow_mut().register_function(func.clone());

        // If receiver is specified (fn graph.method() syntax), attach to that graph
        if let Some(recv_name) = receiver {
            let recv_val = self.env.get(&recv_name).map_err(|_| {
                GraphoidError::runtime(format!(
                    "Cannot attach method '{}' to '{}': variable not found",
                    name, recv_name
                ))
            })?;
            match &recv_val.kind {
                ValueKind::Graph(g) => {
                    if is_static {
                        g.borrow_mut().attach_static_method(name, func);
                    } else {
                        g.borrow_mut().attach_method(name, func);
                    }
                    return Ok(Value::none());
                }
                _ => {
                    return Err(GraphoidError::runtime(format!(
                        "Cannot attach method to '{}': not a graph",
                        recv_name
                    )));
                }
            }
        }

        // Store in global functions (overloading by arity)
        self.global_functions
            .entry(name.clone())
            .or_default()
            .push(func.clone());

        // Track private symbols for module exports
        if is_private || self.in_priv_block {
            self.private_symbols.insert(name.clone());
        }

        // Store in environment with overloading support
        // Only create overloads for functions IN THE SAME SCOPE with different arities
        // Using get_in_current_scope to avoid accidentally overloading with parent scope functions
        let new_arity = func.params.len();
        if let Some(existing) = self.env.get_in_current_scope(&name) {
            match &existing.kind {
                ValueKind::Function(existing_func) => {
                    let existing_arity = existing_func.params.len();
                    if existing_arity != new_arity {
                        // Different arity in same scope - create overload list
                        let list = crate::values::List::from_vec(vec![existing, Value::function(func)]);
                        self.env.define(name, Value::list(list));
                    } else {
                        // Same arity in same scope - replace
                        self.env.define(name, Value::function(func));
                    }
                }
                ValueKind::List(list) => {
                    // Check if this arity already exists in the list
                    let mut has_same_arity = false;
                    for item in list.to_vec() {
                        if let ValueKind::Function(f) = &item.kind {
                            if f.params.len() == new_arity {
                                has_same_arity = true;
                                break;
                            }
                        }
                    }
                    if has_same_arity {
                        // Same arity exists - replace that overload
                        let mut new_items: Vec<Value> = list.to_vec().into_iter()
                            .filter(|item| {
                                if let ValueKind::Function(f) = &item.kind {
                                    f.params.len() != new_arity
                                } else {
                                    true
                                }
                            })
                            .collect();
                        new_items.push(Value::function(func));
                        let new_list = crate::values::List::from_vec(new_items);
                        self.env.define(name, Value::list(new_list));
                    } else {
                        // Different arity - append new function
                        let mut new_list = list.clone();
                        let _ = new_list.append_raw(Value::function(func));
                        self.env.define(name, Value::list(new_list));
                    }
                }
                _ => {
                    // Replace non-function with new function
                    self.env.define(name, Value::function(func));
                }
            }
        } else {
            // No function with this name in current scope - define new
            self.env.define(name, Value::function(func));
        }

        Ok(Value::none())
    }

    // --- Call expression ---

    fn exec_call(&mut self, node_ref: NodeRef) -> Result<Value> {
        let callee_ref = self.get_edge_target(node_ref, &ExecEdgeType::Callee)
            .ok_or_else(|| GraphoidError::runtime("Missing callee".to_string()))?;

        // Evaluate arguments, tracking named arg info
        let arg_refs = self.get_ordered_edges(node_ref, "Argument");
        let mut arg_values = Vec::new();
        let mut arg_names: Vec<Option<String>> = Vec::new();
        for arg_ref in &arg_refs {
            // Check if this argument node has an arg_name property (named argument)
            let arg_name = if let Ok(node) = self.get_node(*arg_ref) {
                if let Some(AstProperty::Str(name)) = node.properties.get("arg_name") {
                    Some(name.clone())
                } else {
                    None
                }
            } else {
                None
            };
            let val = self.execute_node(*arg_ref)?;
            arg_values.push(val);
            arg_names.push(arg_name);
        }

        let has_named = arg_names.iter().any(|n| n.is_some());

        // Check if callee is a builtin name
        let callee_node = self.get_node(callee_ref)?;
        if callee_node.node_type == AstNodeType::Identifier {
            if let Some(name) = callee_node.properties.get("name") {
                if let AstProperty::Str(func_name) = name {
                    let func_name = func_name.clone();

                    // Try pattern object builtins with named args first
                    if let Some(result) = self.try_pattern_builtin(&func_name, &arg_values, &arg_names)? {
                        return Ok(result);
                    }

                    // Try builtins first
                    if let Some(result) = self.try_builtin(&func_name, &arg_values)? {
                        return Ok(result);
                    }

                    // Implicit self method call: if `self` is a graph with this method,
                    // call it as a method call (with proper self binding)
                    // Uses eval_graph_method for proper guard evaluation (Phase 21)
                    // Also checks block_self_stack for trailing block contexts
                    // (e.g., `obj.method() { || bare_call() }` where bare_call is a method on obj)
                    let implicit_self = self.env.get("self").ok()
                        .or_else(|| self.block_self_stack.last().cloned());
                    if let Some(self_value) = implicit_self {
                        if let ValueKind::Graph(ref graph_rc) = self_value.kind {
                            let graph = graph_rc.borrow();
                            if graph.has_method(&func_name) {
                                let graph_clone = graph.clone();
                                drop(graph);
                                let self_expr = Expr::Variable {
                                    name: "self".to_string(),
                                    position: SourcePosition::unknown(),
                                };
                                // Use eval_graph_method which handles guard dispatch
                                self.graph_method_value_stack.push(self_value.clone());
                                let result = self.eval_graph_method(graph_clone, &func_name, &arg_values, &self_expr);
                                self.graph_method_value_stack.pop();
                                return result;
                            }
                        }
                    }

                    // Check environment for function (respects scoping - shadows global_functions)
                    if let Ok(env_val) = self.env.get(&func_name) {
                        match &env_val.kind {
                            ValueKind::Function(func) => {
                                if has_named {
                                    return self.call_graph_function_named(func.clone(), arg_values, arg_names);
                                } else {
                                    return self.call_graph_function(func.clone(), arg_values);
                                }
                            }
                            ValueKind::List(list) => {
                                // List of overloads - find matching arity
                                let arity = arg_values.len();
                                for item in list.to_vec() {
                                    if let ValueKind::Function(func) = &item.kind {
                                        let param_count = func.params.len();
                                        let has_variadic = func.parameters.iter().any(|p| p.is_variadic);
                                        if param_count == arity || (has_variadic && arity >= param_count - 1) {
                                            if has_named {
                                                return self.call_graph_function_named(func.clone(), arg_values, arg_names);
                                            } else {
                                                return self.call_graph_function(func.clone(), arg_values);
                                            }
                                        }
                                    }
                                }
                                // Fall through if no matching arity
                            }
                            _ => {} // Not a function, fall through
                        }
                    }

                    // Fall back to global_functions for overloaded functions (by arity)
                    if let Some(overloads) = self.global_functions.get(&func_name) {
                        let arity = arg_values.len();
                        for func in overloads {
                            let param_count = func.params.len();
                            let has_variadic = func.parameters.iter().any(|p| p.is_variadic);
                            if param_count == arity || (has_variadic && arity >= param_count - 1) {
                                if has_named {
                                    return self.call_graph_function_named(func.clone(), arg_values, arg_names);
                                } else {
                                    return self.call_graph_function(func.clone(), arg_values);
                                }
                            }
                        }
                        // No matching arity found - fall through to error or other handling
                    }
                }
            }
        }

        // Evaluate callee as expression (could be a function value)
        let callee_val = self.execute_node(callee_ref)?;

        match &callee_val.kind {
            ValueKind::Function(func) => {
                if has_named {
                    self.call_graph_function_named(func.clone(), arg_values, arg_names)
                } else {
                    self.call_graph_function(func.clone(), arg_values)
                }
            }
            ValueKind::NativeFunction(native_func) => {
                native_func(&arg_values)
            }
            ValueKind::List(list) => {
                // List of function overloads - find the right one by arity
                let arity = arg_values.len();
                for item in list.to_vec() {
                    if let ValueKind::Function(func) = &item.kind {
                        let param_count = func.params.len();
                        let has_variadic = func.parameters.iter().any(|p| p.is_variadic);
                        if param_count == arity || (has_variadic && arity >= param_count - 1) {
                            if has_named {
                                return self.call_graph_function_named(func.clone(), arg_values, arg_names);
                            } else {
                                return self.call_graph_function(func.clone(), arg_values);
                            }
                        }
                    }
                }
                // No matching overload - fall back to first function (for better error message)
                if let Some(first) = list.get(0) {
                    if let ValueKind::Function(func) = &first.kind {
                        return self.call_graph_function(func.clone(), arg_values);
                    }
                }
                Err(GraphoidError::runtime(format!(
                    "No matching function overload for {} arguments", arity
                )))
            }
            _ => Err(GraphoidError::type_error("function", callee_val.type_name())),
        }
    }

    /// Try to call pattern object builtins (node, edge, path) that need named arg info.
    fn try_pattern_builtin(&self, name: &str, args: &[Value], arg_names: &[Option<String>]) -> Result<Option<Value>> {
        match name {
            "node" => {
                let mut variable: Option<String> = None;
                let mut node_type: Option<String> = None;
                for (i, val) in args.iter().enumerate() {
                    let arg_name = arg_names.get(i).and_then(|n| n.as_ref());
                    if let Some(name) = arg_name {
                        if name == "type" {
                            node_type = Some(val.to_string_value());
                        } else {
                            return Err(GraphoidError::runtime(format!(
                                "node() does not accept parameter '{}'", name
                            )));
                        }
                    } else {
                        if variable.is_some() {
                            return Err(GraphoidError::runtime(
                                "node() accepts at most one positional argument (variable)".to_string()
                            ));
                        }
                        variable = Some(val.to_string_value());
                    }
                }
                Ok(Some(Value::pattern_node(variable, node_type)))
            }
            "edge" => {
                let mut edge_type: Option<String> = None;
                let mut direction = "outgoing".to_string();
                for (i, val) in args.iter().enumerate() {
                    let arg_name = arg_names.get(i).and_then(|n| n.as_ref());
                    if let Some(name) = arg_name {
                        match name.as_str() {
                            "type" => { edge_type = Some(val.to_string_value()); }
                            "direction" => {
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
                                    "edge() does not accept parameter '{}'", name
                                )));
                            }
                        }
                    } else {
                        return Err(GraphoidError::runtime(
                            "edge() does not accept positional arguments, use named parameters: type, direction".to_string()
                        ));
                    }
                }
                Ok(Some(Value::pattern_edge(edge_type, direction)))
            }
            "path" => {
                let mut edge_type: Option<String> = None;
                let mut min: Option<usize> = None;
                let mut max: Option<usize> = None;
                let mut direction = "outgoing".to_string();
                for (i, val) in args.iter().enumerate() {
                    let arg_name = arg_names.get(i).and_then(|n| n.as_ref());
                    if let Some(name) = arg_name {
                        match name.as_str() {
                            "edge_type" | "type" => { edge_type = Some(val.to_string_value()); }
                            "min" => {
                                if let ValueKind::Number(n) = val.kind {
                                    min = Some(n as usize);
                                } else {
                                    return Err(GraphoidError::runtime(format!(
                                        "path() min must be a number, got {}", val.type_name()
                                    )));
                                }
                            }
                            "max" => {
                                if let ValueKind::Number(n) = val.kind {
                                    max = Some(n as usize);
                                } else {
                                    return Err(GraphoidError::runtime(format!(
                                        "path() max must be a number, got {}", val.type_name()
                                    )));
                                }
                            }
                            "direction" => {
                                if let ValueKind::Symbol(s) = &val.kind {
                                    direction = s.clone();
                                } else {
                                    return Err(GraphoidError::runtime(format!(
                                        "path() direction must be a symbol, got {}", val.type_name()
                                    )));
                                }
                            }
                            _ => {
                                return Err(GraphoidError::runtime(format!(
                                    "path() does not accept parameter '{}'", name
                                )));
                            }
                        }
                    } else {
                        return Err(GraphoidError::runtime(
                            "path() does not accept positional arguments, use named parameters: edge_type, min, max, direction".to_string()
                        ));
                    }
                }
                // All parameters are optional with sensible defaults
                // edge_type: None means any edge type (represented as empty string)
                // min: defaults to 1
                // max: defaults to min (or 1 if min not specified)
                let min_val = min.unwrap_or(1);
                let max_val = max.unwrap_or(min_val);
                if min_val > max_val {
                    return Err(GraphoidError::runtime(
                        "path() min cannot be greater than max".to_string()
                    ));
                }
                // Empty string means match any edge type
                let edge_type_str = edge_type.unwrap_or_default();
                Ok(Some(Value::pattern_path(edge_type_str, min_val, max_val, direction)))
            }
            _ => Ok(None),
        }
    }

    /// Try to call a builtin function. Returns Some(value) if it's a builtin, None if not.
    fn try_builtin(&mut self, name: &str, args: &[Value]) -> Result<Option<Value>> {
        match name {
            "print" => {
                let output: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                let text = output.join(" ");
                if self.output_capture_enabled {
                    self.output_buffer.push_str(&text);
                    self.output_buffer.push('\n');
                } else {
                    println!("{}", text);
                }
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
            // Error type constructors
            "RuntimeError" | "ValueError" | "TypeError" | "IOError" | "NetworkError" | "ParseError" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "{} constructor expects 1 argument (message), got {}",
                        name, args.len()
                    )));
                }
                let message = args[0].to_string_value();
                let error_obj = crate::values::ErrorObject::with_stack_trace(
                    name.to_string(),
                    message,
                    self.current_file.as_ref().map(|p| p.to_string_lossy().to_string()),
                    0, 0, self.call_stack.clone(),
                );
                Ok(Some(Value::error(error_obj)))
            }
            "get_errors" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "get_errors() takes no arguments, but got {}", args.len()
                    )));
                }
                let errors = self.error_collector.get_errors();
                let error_values: Vec<Value> = errors.iter().map(|collected_err| {
                    Value::error(crate::values::ErrorObject::new(
                        collected_err.error.error_type(),
                        collected_err.error.to_string(),
                        collected_err.file.clone(),
                        collected_err.position.line,
                        collected_err.position.column,
                    ))
                }).collect();
                Ok(Some(Value::list(crate::values::List::from_vec(error_values))))
            }
            "clear_errors" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "clear_errors() takes no arguments, but got {}", args.len()
                    )));
                }
                self.error_collector.clear();
                Ok(Some(Value::none()))
            }
            "exec" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "exec() expects 1 argument (file path), got {}", args.len()
                    )));
                }
                let path = match &args[0].kind {
                    ValueKind::String(s) => s.clone(),
                    _ => return Err(GraphoidError::runtime(format!(
                        "exec() path must be a string, got {}", args[0].type_name()
                    ))),
                };
                let source = std::fs::read_to_string(&path).map_err(|e| {
                    GraphoidError::runtime(format!("exec(): failed to read file '{}': {}", path, e))
                })?;
                let mut file_executor = GraphExecutor::new();
                file_executor.enable_output_capture();
                file_executor.set_current_file(Some(std::path::PathBuf::from(&path)));
                file_executor.execute_source(&source)?;
                let output = file_executor.get_captured_output();
                Ok(Some(Value::string(output)))
            }
            _ => Ok(None), // Not a builtin
        }
    }

    /// Call a graph-based function (one whose body is stored as a NodeRef).
    fn call_graph_function(&mut self, func: Function, args: Vec<Value>) -> Result<Value> {
        // If function has no graph ID or no graph body, fall back to AST-based execution
        let func_id = match func.node_id.as_ref() {
            Some(id) => id.clone(),
            None => {
                if !func.body.is_empty() {
                    return self.call_ast_function(&func, args);
                }
                return Err(GraphoidError::runtime("Function has no graph ID and no AST body".to_string()));
            }
        };
        let body_ref_opt = self.graph_function_bodies.get(&func_id).copied();

        // If no graph body found, fall back to AST-based execution (for stdlib .gr functions)
        if body_ref_opt.is_none() && !func.body.is_empty() {
            return self.call_ast_function(&func, args);
        }

        let body_ref = body_ref_opt
            .ok_or_else(|| GraphoidError::runtime(format!("Function body not found for {}", func_id)))?;

        // Check if this is a pattern-matching function
        if let Some(clauses) = self.graph_pattern_clauses.get(&func_id).cloned() {
            return self.call_pattern_matching_function(&func, &args, &clauses);
        }

        // Build call environment from captured env
        let mut call_env = (*func.env.borrow()).clone();

        // Check for too many arguments (only if no variadic param)
        let has_variadic = func.parameters.iter().any(|p| p.is_variadic);
        if !has_variadic && args.len() > func.parameters.len() {
            return Err(GraphoidError::runtime(format!(
                "Function '{}' expects {} argument(s), got {}",
                func.name.as_deref().unwrap_or("<anonymous>"),
                func.parameters.len(),
                args.len()
            )));
        }

        // Bind parameters (with variadic support)
        for (i, param) in func.parameters.iter().enumerate() {
            if param.is_variadic {
                // Collect all remaining arguments into a list
                let rest: Vec<Value> = if i < args.len() {
                    args[i..].to_vec()
                } else {
                    Vec::new()
                };
                call_env.define(param.name.clone(), Value::list(crate::values::List::from_vec(rest)));
            } else if i < args.len() {
                call_env.define(param.name.clone(), args[i].clone());
            } else {
                // Try default value
                let default_key = format!("{}__default_{}", func_id, i);
                if let Some(default_ref) = self.graph_function_bodies.get(&default_key).copied() {
                    let default_val = self.execute_node(default_ref)?;
                    call_env.define(param.name.clone(), default_val);
                } else {
                    call_env.define(param.name.clone(), Value::none());
                }
            }
        }

        // Track function call in function graph
        let func_name = func.name.as_deref().unwrap_or("<anonymous>").to_string();
        self.call_stack.push(func_name.clone());

        // Get or register in function graph and push call
        // Named functions: use name-based dedup. Lambdas: use node_id-based identity.
        let fg_node_id = if func.name.is_some() {
            // Named function — deduplicate by name
            let mut fg = self.function_graph.borrow_mut();
            if let Some(node) = fg.get_function_by_name(&func_name) {
                node.node_id.clone()
            } else {
                fg.register_function(func.clone())
            }
        } else if let Some(ref fid) = func.node_id {
            // Lambda with node_id — deduplicate by identity
            if let Some(existing_fg_id) = self.func_to_fg_id.get(fid) {
                existing_fg_id.clone()
            } else {
                let fg_id = self.function_graph.borrow_mut().register_function(func.clone());
                self.func_to_fg_id.insert(fid.clone(), fg_id.clone());
                fg_id
            }
        } else {
            // Anonymous function with no node_id — register each time
            self.function_graph.borrow_mut().register_function(func.clone())
        };
        self.function_graph.borrow_mut().push_call(fg_node_id, args.clone());

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
        self.call_stack.pop();
        // Record exception propagation edge if function exited with error
        if result.is_err() {
            let error_type = result.as_ref().unwrap_err().error_type();
            self.function_graph.borrow_mut().pop_call_exception(error_type);
        } else {
            self.function_graph.borrow_mut().pop_call(result.as_ref().ok().cloned().unwrap_or(Value::none()));
        }
        let call_env_after = std::mem::replace(&mut self.env, saved_env);

        // Update closure state (for closures that modify captured variables)
        *func.env.borrow_mut() = call_env_after;

        result
    }

    /// Fallback: execute an AST-bodied function (for stdlib .gr modules loaded at runtime).
    fn call_ast_function(&mut self, func: &Function, args: Vec<Value>) -> Result<Value> {
        // Build call environment
        let mut call_env = (*func.env.borrow()).clone();

        // Bind parameters
        let has_variadic = func.parameters.iter().any(|p| p.is_variadic);
        if !has_variadic && args.len() > func.parameters.len() {
            return Err(GraphoidError::runtime(format!(
                "Function '{}' expects {} argument(s), got {}",
                func.name.as_deref().unwrap_or("<anonymous>"),
                func.parameters.len(),
                args.len()
            )));
        }

        for (i, param) in func.parameters.iter().enumerate() {
            if param.is_variadic {
                let rest: Vec<Value> = if i < args.len() { args[i..].to_vec() } else { Vec::new() };
                call_env.define(param.name.clone(), Value::list(crate::values::List::from_vec(rest)));
            } else if i < args.len() {
                call_env.define(param.name.clone(), args[i].clone());
            } else {
                call_env.define(param.name.clone(), Value::none());
            }
        }

        // Track call
        let func_name = func.name.as_deref().unwrap_or("<anonymous>").to_string();
        self.call_stack.push(func_name.clone());
        self.function_call_depth += 1;

        // Swap environments
        let saved_env = std::mem::replace(&mut self.env, call_env);

        // Execute AST body statements
        let mut return_value = Value::none();
        let result: Result<()> = (|| {
            for stmt in &func.body {
                match self.eval_stmt(stmt)? {
                    Some(val) => {
                        return_value = val;
                        break;
                    }
                    None => {}
                }
            }
            Ok(())
        })();

        // Restore environment
        self.function_call_depth -= 1;
        self.call_stack.pop();
        let call_env_after = std::mem::replace(&mut self.env, saved_env);
        *func.env.borrow_mut() = call_env_after;

        match result {
            Ok(()) => Ok(return_value),
            Err(GraphoidError::ReturnControl { value }) => Ok(value),
            Err(e) => Err(e),
        }
    }

    /// Execute a pattern-matching function by matching args against clauses.
    fn call_pattern_matching_function(
        &mut self,
        func: &Function,
        args: &[Value],
        clauses: &[GraphPatternClause],
    ) -> Result<Value> {
        // Pattern matching requires exactly 1 argument
        if args.len() != 1 {
            return Err(GraphoidError::runtime(
                format!("Pattern matching requires exactly 1 argument, got {}", args.len())
            ));
        }
        let arg = &args[0];

        // Set up call environment with parameter bound
        let mut call_env = (*func.env.borrow()).clone();
        if let Some(param) = func.parameters.first() {
            call_env.define(param.name.clone(), arg.clone());
        }

        // Try each clause in order
        for clause in clauses {
            let matched = match clause.pattern_type.as_str() {
                "wildcard" => true,
                "variable" => true,
                "literal" => {
                    match &clause.pattern_value {
                        Some(AstProperty::Num(n)) => {
                            if let ValueKind::Number(v) = &arg.kind {
                                (n - v).abs() < f64::EPSILON
                            } else {
                                false
                            }
                        }
                        Some(AstProperty::Str(s)) => {
                            if let ValueKind::String(v) = &arg.kind {
                                s == v
                            } else {
                                false
                            }
                        }
                        Some(AstProperty::Bool(b)) => {
                            if let ValueKind::Boolean(v) = &arg.kind {
                                b == v
                            } else {
                                false
                            }
                        }
                        Some(AstProperty::None) => {
                            matches!(&arg.kind, ValueKind::None)
                        }
                        _ => false,
                    }
                }
                _ => false,
            };

            if !matched {
                continue;
            }

            // Bind pattern variable if present
            let mut bindings = std::collections::HashMap::new();
            if clause.pattern_type == "variable" {
                if let Some(ref name) = clause.pattern_name {
                    bindings.insert(name.clone(), arg.clone());
                }
            }

            // Check guard if present
            if let Some(guard_ref) = clause.guard_ref {
                // Evaluate guard in a temp env with bindings
                let temp_env = Environment::with_parent(call_env.clone());
                let saved_env = std::mem::replace(&mut self.env, temp_env);
                for (name, value) in &bindings {
                    self.env.define(name.clone(), value.clone());
                }
                // Also bind the function parameter
                if let Some(param) = func.parameters.first() {
                    self.env.define(param.name.clone(), arg.clone());
                }
                let guard_result = self.execute_node(guard_ref);
                self.env = saved_env;

                match guard_result {
                    Ok(v) if v.is_truthy() => {} // Guard passed
                    _ => continue, // Guard failed, try next clause
                }
            }

            // Match found - execute the body in proper env
            let mut body_env = call_env.clone();
            for (name, value) in bindings {
                body_env.define(name, value);
            }
            // Also bind function parameter
            if let Some(param) = func.parameters.first() {
                body_env.define(param.name.clone(), arg.clone());
            }

            let saved_env = std::mem::replace(&mut self.env, body_env);
            self.function_call_depth += 1;

            let result = match self.execute_node(clause.body_ref) {
                Ok(val) => Ok(val),
                Err(GraphoidError::ReturnControl { value }) => Ok(value),
                Err(e) => Err(e),
            };

            self.function_call_depth -= 1;
            let body_env_after = std::mem::replace(&mut self.env, saved_env);
            *func.env.borrow_mut() = body_env_after;

            return result;
        }

        // No pattern matched
        Ok(Value::none())
    }

    /// Call a graph-based function with named arguments.
    /// Named args are reordered to match parameter positions; supports variadic and defaults.
    fn call_graph_function_named(&mut self, func: Function, args: Vec<Value>, arg_names: Vec<Option<String>>) -> Result<Value> {
        let func_id = func.node_id.as_ref()
            .ok_or_else(|| GraphoidError::runtime("Function has no graph ID".to_string()))?;
        let body_ref = *self.graph_function_bodies.get(func_id)
            .ok_or_else(|| GraphoidError::runtime(format!("Function body not found for {}", func_id)))?;

        // Separate non-variadic params from the variadic one
        let has_variadic = func.parameters.iter().any(|p| p.is_variadic);
        let non_variadic_count = func.parameters.iter().filter(|p| !p.is_variadic).count();

        // Reorder: named args go to matching positions, positional fill remaining slots
        let mut reordered: Vec<Option<Value>> = vec![None; non_variadic_count];
        let mut variadic_values: Vec<Value> = Vec::new();
        let mut positional_idx = 0;

        for (i, arg_name) in arg_names.iter().enumerate() {
            if let Some(name) = arg_name {
                // Named argument — find matching non-variadic parameter
                let pos = func.parameters.iter()
                    .enumerate()
                    .filter(|(_, p)| !p.is_variadic)
                    .position(|(_, p)| p.name == *name);
                if let Some(pos) = pos {
                    if reordered[pos].is_some() {
                        return Err(GraphoidError::runtime(format!(
                            "Duplicate named parameter: '{}'", name
                        )));
                    }
                    reordered[pos] = Some(args[i].clone());
                } else {
                    return Err(GraphoidError::runtime(format!(
                        "Unknown named parameter: '{}' for function '{}'",
                        name,
                        func.name.as_deref().unwrap_or("<anonymous>")
                    )));
                }
            } else {
                // Positional argument — fill next available non-variadic slot, overflow to variadic
                while positional_idx < non_variadic_count && reordered[positional_idx].is_some() {
                    positional_idx += 1;
                }
                if positional_idx < non_variadic_count {
                    reordered[positional_idx] = Some(args[i].clone());
                    positional_idx += 1;
                } else if has_variadic {
                    variadic_values.push(args[i].clone());
                } else {
                    return Err(GraphoidError::runtime(format!(
                        "Too many arguments for function '{}': expected {}, got {}",
                        func.name.as_deref().unwrap_or("<anonymous>"),
                        non_variadic_count,
                        args.len()
                    )));
                }
            }
        }

        // Build call environment
        let mut call_env = (*func.env.borrow()).clone();

        // Bind non-variadic params (with defaults for unfilled slots)
        let mut nv_idx = 0;
        for (i, param) in func.parameters.iter().enumerate() {
            if param.is_variadic {
                call_env.define(param.name.clone(), Value::list(crate::values::List::from_vec(variadic_values.clone())));
            } else {
                if let Some(val) = reordered[nv_idx].take() {
                    call_env.define(param.name.clone(), val);
                } else {
                    // Try default value
                    let default_key = format!("{}__default_{}", func_id, i);
                    if let Some(default_ref) = self.graph_function_bodies.get(&default_key).copied() {
                        let default_val = self.execute_node(default_ref)?;
                        call_env.define(param.name.clone(), default_val);
                    } else {
                        call_env.define(param.name.clone(), Value::none());
                    }
                }
                nv_idx += 1;
            }
        }

        // Swap environments and execute (same as call_graph_function)
        let saved_env = std::mem::replace(&mut self.env, call_env);
        self.function_call_depth += 1;

        let is_lambda = func.name.is_none();
        let result = match self.execute_node(body_ref) {
            Ok(val) => {
                if is_lambda {
                    Ok(val)
                } else {
                    Ok(Value::none())
                }
            }
            Err(GraphoidError::ReturnControl { value }) => Ok(value),
            Err(e) => Err(e),
        };

        self.function_call_depth -= 1;
        let call_env_after = std::mem::replace(&mut self.env, saved_env);
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

        // Build a synthetic object_expr for mutation-back-to-variable
        let obj_node = self.get_node(obj_ref)?;
        let obj_var_name = if obj_node.node_type == AstNodeType::Identifier {
            obj_node.get_str("name")
        } else {
            None
        };
        let object_expr = if let Some(ref name) = obj_var_name {
            Expr::Variable { name: name.clone(), position: obj_node.position.clone() }
        } else {
            Expr::Literal { value: crate::ast::LiteralValue::None, position: SourcePosition::unknown() }
        };

        // Check for static method calls on built-in type identifiers (time, list, string)
        if let Some(ref name) = obj_var_name {
            let static_dispatch = match name.as_str() {
                "time" if !self.env.exists("time") => Some("time"),
                "list" => Some("list"),
                "string" => Some("string"),
                "reflect" if !self.env.exists("reflect") => Some("reflect"),
                _ => None,
            };
            if let Some(type_name) = static_dispatch {
                let arg_refs = self.get_ordered_edges(node_ref, "Argument");

                // Special form: reflect.pattern() inspects unevaluated AST structure
                if type_name == "reflect" && method_name == "pattern" {
                    return self.eval_reflect_pattern(&arg_refs);
                }

                let mut args = Vec::new();
                for arg_ref in &arg_refs {
                    let val = self.execute_node(*arg_ref)?;
                    args.push(val);
                }
                return match type_name {
                    "time" => self.eval_time_static_method(&method_name, &args),
                    "list" => self.eval_list_static_method(&method_name, &args),
                    "string" => self.eval_string_static_method(&method_name, &args),
                    "reflect" => self.eval_reflect_static_method(&method_name, &args),
                    _ => unreachable!(),
                };
            }
        }

        let object = self.execute_node(obj_ref)?;

        // Evaluate arguments
        let arg_refs = self.get_ordered_edges(node_ref, "Argument");
        let mut args = Vec::new();
        for arg_ref in &arg_refs {
            let val = self.execute_node(*arg_ref)?;
            args.push(val);
        }

        self.dispatch_method(object, &method_name, args, &object_expr)
    }

    /// Full method dispatch using implementations from src/execution/methods/*.rs.
    fn dispatch_method(&mut self, object: Value, method: &str, args: Vec<Value>, object_expr: &Expr) -> Result<Value> {
        // Check if this is a mutating method (ends with !)
        let is_mutating = method.ends_with('!');
        let base_method = if is_mutating {
            &method[..method.len() - 1]
        } else {
            method
        };

        if is_mutating {
            let var_name = match object_expr {
                Expr::Variable { name, .. } => name.clone(),
                _ => return Err(GraphoidError::runtime(format!(
                    "Mutating method '{}' requires a variable, not an expression", method
                ))),
            };

            // Special case for pop: returns the popped value
            if base_method == "pop" {
                if let ValueKind::List(list) = &object.kind {
                    let mut list_to_mutate = list.clone();
                    let popped_value = list_to_mutate.pop()?;
                    self.set_variable_or_self_property(&var_name, Value::list(list_to_mutate))?;
                    return Ok(popped_value);
                }
            }

            let result = self.dispatch_method_inner(object, base_method, args, object_expr)?;
            self.set_variable_or_self_property(&var_name, result)?;
            return Ok(Value::none());
        }

        self.dispatch_method_inner(object, base_method, args, object_expr)
    }

    fn dispatch_method_inner(&mut self, object: Value, method: &str, args: Vec<Value>, object_expr: &Expr) -> Result<Value> {
        // Universal methods that work on all types (skip for modules — module members take priority)
        if !matches!(&object.kind, ValueKind::Module(_)) {
            if let Some(result) = self.try_universal_method(&object, method, &args)? {
                return Ok(result);
            }
        }

        match &object.kind {
            ValueKind::String(s) => self.eval_string_method(s, method, &args, object_expr),
            ValueKind::List(l) => self.eval_list_method(l, method, &args),
            ValueKind::Map(h) => self.eval_map_method(h, method, &args),
            ValueKind::Graph(g) => {
                let graph = g.borrow().clone();
                self.graph_method_value_stack.push(object.clone());
                let result = self.eval_graph_method(graph, method, &args, object_expr);
                self.graph_method_value_stack.pop();
                result
            }
            ValueKind::Number(_) => self.dispatch_number_method(&object, method, &args),
            ValueKind::Error(ref err) => self.eval_error_method(err, method, &args),
            ValueKind::Time(timestamp) => self.eval_time_method(*timestamp, method, &args),
            ValueKind::BigNumber(ref bn) => self.eval_bignum_method(bn, method, &args),
            ValueKind::PatternNode(ref pn) => self.eval_pattern_node_method(pn, method, &args),
            ValueKind::PatternEdge(ref pe) => self.eval_pattern_edge_method(pe, method, &args),
            ValueKind::PatternPath(ref pp) => self.eval_pattern_path_method(pp, method, &args),
            ValueKind::PatternMatchResults(ref results) => {
                let results_clone = results.clone();
                self.eval_pattern_match_results_method(&results_clone, method, &args)
            }
            ValueKind::Module(ref module) => {
                // Phase 17: Module introspection methods
                match method {
                    "exports" => {
                        let items: Vec<Value> = module.exports.iter()
                            .map(|s| Value::string(s.clone()))
                            .collect();
                        return Ok(Value::list(crate::values::List::from_vec(items)));
                    }
                    "name" => {
                        return Ok(Value::string(module.name.clone()));
                    }
                    "path" => {
                        return Ok(Value::string(module.file_path.to_string_lossy().to_string()));
                    }
                    "imports" => {
                        let key = format!("file:{}", module.file_path.to_string_lossy());
                        let deps = self.module_manager.get_dependencies(&key);
                        let items: Vec<Value> = deps.into_iter()
                            .map(Value::string)
                            .collect();
                        return Ok(Value::list(crate::values::List::from_vec(items)));
                    }
                    _ => {}
                }

                // Module method dispatch: look up method in module namespace
                if module.private_symbols.contains(method) {
                    return Err(GraphoidError::runtime(format!(
                        "Cannot access private symbol '{}' from module '{}'", method, module.name
                    )));
                }
                let member = module.namespace.get(method)?;
                match &member.kind {
                    ValueKind::Function(func) => {
                        self.call_graph_function(func.clone(), args)
                    }
                    ValueKind::NativeFunction(native_func) => {
                        native_func(&args)
                    }
                    ValueKind::List(list) => {
                        // List of function overloads - find correct arity
                        let arity = args.len();
                        for item in list.to_vec() {
                            if let ValueKind::Function(func) = &item.kind {
                                let param_count = func.params.len();
                                let has_variadic = func.parameters.iter().any(|p| p.is_variadic);
                                if param_count == arity || has_variadic {
                                    return self.call_graph_function(func.clone(), args);
                                }
                            }
                        }
                        // Fall back to first function
                        if let Some(first) = list.get(0) {
                            if let ValueKind::Function(func) = &first.kind {
                                return self.call_graph_function(func.clone(), args);
                            }
                        }
                        Err(GraphoidError::runtime(format!(
                            "No matching overload for '{}' with {} args", method, arity
                        )))
                    }
                    _ => {
                        if args.is_empty() {
                            Ok(member)
                        } else {
                            Err(GraphoidError::runtime(format!(
                                "Module member '{}' is not a function, cannot be called with arguments", method
                            )))
                        }
                    }
                }
            }
            _ => Err(GraphoidError::runtime(format!(
                "Cannot call method '{}' on type '{}'", method, object.type_name()
            ))),
        }
    }

    /// Error object methods: type(), message(), stack_trace(), etc.
    fn eval_error_method(&self, err: &crate::values::ErrorObject, method: &str, args: &[Value]) -> Result<Value> {
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
                Ok(err.file.as_ref().map(|f| Value::string(f.clone())).unwrap_or(Value::none()))
            }
            "line" => Ok(Value::number(err.line as f64)),
            "column" => Ok(Value::number(err.column as f64)),
            "stack_trace" => {
                Ok(Value::string(err.formatted_stack_trace()))
            }
            "full_chain" => {
                Ok(Value::string(err.full_chain()))
            }
            "cause" => {
                Ok(err.cause.as_ref().map(|c| Value::error((**c).clone())).unwrap_or(Value::none()))
            }
            "caused_by" => {
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
                    _ => Err(GraphoidError::runtime(format!(
                        "Error.caused_by() expects an error argument, got {}", args[0].type_name()
                    ))),
                }
            }
            _ => Err(GraphoidError::runtime(format!(
                "Error does not have method '{}'", method
            ))),
        }
    }

    /// Time instance methods
    fn eval_time_method(&self, timestamp: f64, method: &str, args: &[Value]) -> Result<Value> {
        use chrono::{Utc, TimeZone, Datelike, Timelike};
        match method {
            "to_string" | "to_str" => {
                let secs = timestamp.trunc() as i64;
                let nsecs = ((timestamp.fract()) * 1_000_000_000.0) as u32;
                if let Some(dt) = Utc.timestamp_opt(secs, nsecs).single() {
                    Ok(Value::string(dt.to_rfc3339()))
                } else {
                    Ok(Value::string(format!("{}", timestamp)))
                }
            }
            "to_num" => Ok(Value::number(timestamp)),
            "to_bool" => Ok(Value::boolean(true)),
            "year" | "month" | "day" | "hour" | "minute" | "second" => {
                let secs = timestamp.trunc() as i64;
                if let Some(dt) = Utc.timestamp_opt(secs, 0).single() {
                    let val = match method {
                        "year" => dt.year() as f64,
                        "month" => dt.month() as f64,
                        "day" => dt.day() as f64,
                        "hour" => dt.hour() as f64,
                        "minute" => dt.minute() as f64,
                        "second" => dt.second() as f64,
                        _ => unreachable!(),
                    };
                    Ok(Value::number(val))
                } else {
                    Err(GraphoidError::runtime("Invalid timestamp".to_string()))
                }
            }
            "time_numbers" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Time method 'time_numbers' takes no arguments, but got {}", args.len()
                    )));
                }
                let seconds = timestamp.trunc() as i64;
                let nanos = ((timestamp - timestamp.trunc()) * 1_000_000_000.0) as u32;
                use chrono::DateTime;
                let dt = DateTime::from_timestamp(seconds, nanos)
                    .ok_or_else(|| GraphoidError::runtime("Invalid timestamp".to_string()))?;
                let mut hash = crate::values::Hash::new();
                let _ = hash.insert("year".to_string(), Value::number(dt.year() as f64));
                let _ = hash.insert("month".to_string(), Value::number(dt.month() as f64));
                let _ = hash.insert("day".to_string(), Value::number(dt.day() as f64));
                let _ = hash.insert("hour".to_string(), Value::number(dt.hour() as f64));
                let _ = hash.insert("minute".to_string(), Value::number(dt.minute() as f64));
                let _ = hash.insert("second".to_string(), Value::number(dt.second() as f64));
                let _ = hash.insert("weekday".to_string(), Value::number(dt.weekday().num_days_from_sunday() as f64));
                let _ = hash.insert("day_of_year".to_string(), Value::number(dt.ordinal() as f64));
                Ok(Value::map(hash))
            }
            _ => Err(GraphoidError::runtime(format!(
                "Time does not have method '{}'", method
            ))),
        }
    }

    /// Time static methods: time.now(), time.today(), etc.
    fn eval_time_static_method(&self, method: &str, args: &[Value]) -> Result<Value> {
        use chrono::{Utc, TimeZone, Datelike};
        match method {
            "now" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "time.now() takes no arguments, but got {}", args.len()
                    )));
                }
                let now = Utc::now();
                let timestamp = now.timestamp() as f64 + (now.timestamp_subsec_nanos() as f64 / 1_000_000_000.0);
                Ok(Value::time(timestamp))
            }
            "today" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "time.today() takes no arguments, but got {}", args.len()
                    )));
                }
                let now = Utc::now();
                let today = Utc.with_ymd_and_hms(now.year(), now.month(), now.day(), 0, 0, 0)
                    .single()
                    .ok_or_else(|| GraphoidError::runtime("Failed to create today's date".to_string()))?;
                Ok(Value::time(today.timestamp() as f64))
            }
            "from_numbers" => {
                if args.len() != 6 {
                    return Err(GraphoidError::runtime(format!(
                        "time.from_numbers() expects 6 arguments (year, month, day, hour, min, sec), but got {}", args.len()
                    )));
                }
                let year = match &args[0].kind { ValueKind::Number(n) => *n as i32, _ => return Err(GraphoidError::type_error("number", args[0].type_name())) };
                let month = match &args[1].kind { ValueKind::Number(n) => *n as u32, _ => return Err(GraphoidError::type_error("number", args[1].type_name())) };
                let day = match &args[2].kind { ValueKind::Number(n) => *n as u32, _ => return Err(GraphoidError::type_error("number", args[2].type_name())) };
                let hour = match &args[3].kind { ValueKind::Number(n) => *n as u32, _ => return Err(GraphoidError::type_error("number", args[3].type_name())) };
                let min = match &args[4].kind { ValueKind::Number(n) => *n as u32, _ => return Err(GraphoidError::type_error("number", args[4].type_name())) };
                let sec = match &args[5].kind { ValueKind::Number(n) => *n as u32, _ => return Err(GraphoidError::type_error("number", args[5].type_name())) };
                let dt = Utc.with_ymd_and_hms(year, month, day, hour, min, sec)
                    .single()
                    .ok_or_else(|| GraphoidError::runtime(format!(
                        "Invalid date/time: {}-{:02}-{:02} {:02}:{:02}:{:02}", year, month, day, hour, min, sec
                    )))?;
                Ok(Value::time(dt.timestamp() as f64))
            }
            "from_string" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "time.from_string() expects 1 argument (ISO 8601 string), but got {}", args.len()
                    )));
                }
                let iso_string = match &args[0].kind {
                    ValueKind::String(s) => s,
                    _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
                };
                let dt = chrono::DateTime::parse_from_rfc3339(iso_string)
                    .map_err(|e| GraphoidError::runtime(format!(
                        "Failed to parse time string '{}': {}", iso_string, e
                    )))?;
                let timestamp = dt.timestamp() as f64 + (dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0);
                Ok(Value::time(timestamp))
            }
            "from_timestamp" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "time.from_timestamp() expects 1 argument (Unix timestamp), but got {}", args.len()
                    )));
                }
                let timestamp = match &args[0].kind {
                    ValueKind::Number(n) => *n,
                    _ => return Err(GraphoidError::type_error("number", args[0].type_name())),
                };
                Ok(Value::time(timestamp))
            }
            _ => Err(GraphoidError::runtime(format!(
                "Time does not have method '{}'", method
            ))),
        }
    }

    /// Phase 17: reflect.* static methods for module introspection
    fn eval_reflect_static_method(&self, method: &str, args: &[Value]) -> Result<Value> {
        match method {
            "loaded_modules" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "reflect.loaded_modules() takes no arguments, but got {}", args.len()
                    )));
                }
                let modules = self.module_manager.get_all_modules();
                let mut names: Vec<String> = modules.iter()
                    .map(|m| m.alias.clone().unwrap_or_else(|| m.name.clone()))
                    .collect();
                names.sort();
                names.dedup();
                let items: Vec<Value> = names.into_iter()
                    .map(Value::string)
                    .collect();
                Ok(Value::list(crate::values::List::from_vec(items)))
            }
            "module" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "reflect.module() expects 1 argument (module name), but got {}", args.len()
                    )));
                }
                let name = match &args[0].kind {
                    ValueKind::String(s) => s.clone(),
                    _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
                };
                match self.module_manager.find_module_by_name(&name) {
                    Some(m) => Ok(Value::module(m.clone())),
                    None => Ok(Value::none()),
                }
            }
            "universe" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "reflect.universe() takes no arguments, but got {}", args.len()
                    )));
                }
                // Phase 18: Return clone of persistent universe graph
                let graph_clone = self.universe_graph.borrow().clone();
                Ok(Value::graph(graph_clone))
            }
            "type_hierarchy" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "reflect.type_hierarchy() takes no arguments, but got {}", args.len()
                    )));
                }
                // Phase 18: Extract type subgraph from universe graph
                use crate::values::graph::{Graph, GraphType};
                let ug = self.universe_graph.borrow();
                let mut subgraph = Graph::new(GraphType::Directed);
                // Copy only type:* nodes
                for (node_id, graph_node) in &ug.nodes {
                    if node_id.starts_with("type:") {
                        let _ = subgraph.add_node(node_id.clone(), graph_node.value.clone());
                    }
                }
                // Copy only subtype_of edges between type nodes
                for (node_id, graph_node) in &ug.nodes {
                    if !node_id.starts_with("type:") { continue; }
                    for (neighbor_id, edge_info) in &graph_node.neighbors {
                        if edge_info.edge_type == "subtype_of" && neighbor_id.starts_with("type:") {
                            let _ = subgraph.add_edge(
                                node_id, neighbor_id,
                                "subtype_of".to_string(), None, HashMap::new(),
                            );
                        }
                    }
                }
                Ok(Value::graph(subgraph))
            }
            "current_scope" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "reflect.current_scope() takes no arguments, but got {}", args.len()
                    )));
                }
                use crate::namespace::ScopeType;
                let mut hash = crate::values::Hash::new();
                let scope_type_str = match self.env.current_scope_type() {
                    ScopeType::Global => "global",
                    ScopeType::Function(_) => "function",
                    ScopeType::Block => "block",
                    ScopeType::Module(_) => "module",
                    ScopeType::Class(_) => "class",
                };
                let _ = hash.insert("type".to_string(), Value::string(scope_type_str.to_string()));
                let var_names: Vec<Value> = self.env.get_variable_names().into_iter()
                    .map(Value::string)
                    .collect();
                let _ = hash.insert("variables".to_string(), Value::list(crate::values::List::from_vec(var_names)));
                let _ = hash.insert("depth".to_string(), Value::number(self.env.scope_depth() as f64));
                Ok(Value::map(hash))
            }
            "call_graph" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "reflect.call_graph() takes no arguments, but got {}", args.len()
                    )));
                }
                // Phase 18 Section 4: Return function graph as queryable Graphoid graph
                // with function nodes, call edges, and exception propagation edges.
                use crate::values::graph::{Graph, GraphType};
                use crate::execution::function_graph::FunctionEdgeType;
                let fg = self.function_graph.borrow();
                let mut g = Graph::new(GraphType::Directed);

                // Add function nodes with properties
                for (node_id, fn_node) in fg.get_all_function_nodes() {
                    let mut props = crate::values::Hash::new();
                    let name = fn_node.function.name.clone().unwrap_or_else(|| "anonymous".to_string());
                    let _ = props.insert("name".to_string(), Value::string(name.clone()));
                    let _ = props.insert("call_count".to_string(), Value::number(fn_node.call_count as f64));
                    // Use "fn:{name}" as the graph node ID for user-friendliness
                    let graph_node_id = format!("fn:{}", name);
                    // Avoid duplicate node IDs (multiple overloads get same name)
                    if g.nodes.contains_key(&graph_node_id) {
                        // Use full internal ID for disambiguation
                        let _ = g.add_node(format!("fn:{}", node_id), Value::map(props));
                    } else {
                        let _ = g.add_node(graph_node_id, Value::map(props));
                    }
                }

                // Add edges (call and exception)
                for edge in fg.get_all_edges() {
                    let edge_label = match &edge.edge_type {
                        FunctionEdgeType::Call => "calls",
                        FunctionEdgeType::ExceptionPropagation => "exception",
                        FunctionEdgeType::Captures => "captures",
                        FunctionEdgeType::PassedTo => "passed_to",
                        FunctionEdgeType::Imports => "imports",
                    };
                    // Map internal IDs to fn:{name} node IDs
                    let from_name = fg.get_function(&edge.from)
                        .and_then(|n| n.function.name.clone())
                        .unwrap_or_else(|| edge.from.clone());
                    let to_name = fg.get_function(&edge.to)
                        .and_then(|n| n.function.name.clone())
                        .unwrap_or_else(|| edge.to.clone());
                    let from_id = format!("fn:{}", from_name);
                    let to_id = format!("fn:{}", to_name);
                    if g.nodes.contains_key(&from_id) && g.nodes.contains_key(&to_id) {
                        let _ = g.add_edge(&from_id, &to_id, edge_label.to_string(), None, HashMap::new());
                    }
                }

                Ok(Value::graph(g))
            }
            _ => Err(GraphoidError::runtime(format!(
                "reflect does not have method '{}'", method
            ))),
        }
    }

    /// Phase 18 Section 3: reflect.pattern() — inspect unevaluated expression as pattern graph.
    ///
    /// This is a special form: the first argument is NOT evaluated. Instead, its
    /// execution graph structure is walked to build a pattern graph with binding,
    /// literal, and wildcard nodes. An optional second argument (a lambda) is
    /// evaluated and becomes a guard node.
    fn eval_reflect_pattern(&mut self, arg_refs: &[NodeRef]) -> Result<Value> {
        if arg_refs.is_empty() || arg_refs.len() > 2 {
            return Err(GraphoidError::runtime(format!(
                "reflect.pattern() expects 1-2 arguments, but got {}", arg_refs.len()
            )));
        }

        use crate::values::graph::{Graph, GraphType};
        use crate::values::Hash;

        let mut pattern_graph = Graph::new(GraphType::Directed);
        let expr_ref = arg_refs[0];
        let node = self.get_node(expr_ref)?;

        match &node.node_type {
            AstNodeType::MapExpr => {
                // Map destructuring pattern: { key: binding, key2: literal, ... }
                let entry_refs = self.get_ordered_edges(expr_ref, "Element");
                let mut root_props = Hash::new();
                let _ = root_props.insert("pattern_type".to_string(), Value::string("map".to_string()));
                let _ = root_props.insert("field_count".to_string(), Value::number(entry_refs.len() as f64));
                let _ = pattern_graph.add_node("pattern:root".to_string(), Value::map(root_props));

                for entry_ref in &entry_refs {
                    let entry_node = self.get_node(*entry_ref)?;
                    let key = entry_node.get_str("key").unwrap_or_default();
                    let field_id = format!("field:{}", key);

                    // Get the value expression node
                    if let Some(val_ref) = self.get_edge_target(*entry_ref, &ExecEdgeType::ValueEdge) {
                        let val_node = self.get_node(val_ref)?;
                        let field_value = self.pattern_node_to_value(&key, None, val_node)?;
                        let _ = pattern_graph.add_node(field_id.clone(), Value::map(field_value));
                    } else {
                        let mut props = Hash::new();
                        let _ = props.insert("type".to_string(), Value::string("unknown".to_string()));
                        let _ = props.insert("key".to_string(), Value::string(key.clone()));
                        let _ = pattern_graph.add_node(field_id.clone(), Value::map(props));
                    }
                    let _ = pattern_graph.add_edge("pattern:root", &field_id, "has_field".to_string(), None, HashMap::new());
                }
            }
            AstNodeType::ListExpr => {
                // List destructuring pattern: [binding, literal, wildcard, ...]
                let elem_refs = self.get_ordered_edges(expr_ref, "Element");
                let mut root_props = Hash::new();
                let _ = root_props.insert("pattern_type".to_string(), Value::string("list".to_string()));
                let _ = root_props.insert("element_count".to_string(), Value::number(elem_refs.len() as f64));
                let _ = pattern_graph.add_node("pattern:root".to_string(), Value::map(root_props));

                for (i, elem_ref) in elem_refs.iter().enumerate() {
                    let elem_node = self.get_node(*elem_ref)?;
                    let elem_id = format!("element:{}", i);
                    let elem_value = self.pattern_node_to_value("", Some(i), elem_node)?;
                    let _ = pattern_graph.add_node(elem_id.clone(), Value::map(elem_value));
                    let _ = pattern_graph.add_edge("pattern:root", &elem_id, "has_element".to_string(), None, HashMap::new());
                }
            }
            // Single-node patterns: identifier (binding/wildcard) or any literal type.
            // Reuse pattern_node_to_value then swap "type" → "pattern_type" for root node.
            AstNodeType::Identifier | AstNodeType::NumberLit | AstNodeType::StringLit |
            AstNodeType::BoolLit | AstNodeType::NoneLit | AstNodeType::SymbolLit => {
                let mut root_props = self.pattern_node_to_value("", None, node)?;
                if let Some(type_val) = root_props.get("type").cloned() {
                    let _ = root_props.insert("pattern_type".to_string(), type_val);
                    let _ = root_props.remove("type");
                }
                let _ = pattern_graph.add_node("pattern:root".to_string(), Value::map(root_props));
            }
            other => {
                return Err(GraphoidError::runtime(format!(
                    "reflect.pattern() cannot interpret {:?} as a pattern", other
                )));
            }
        }

        // Optional guard (second argument — evaluated as a lambda)
        if arg_refs.len() == 2 {
            let _guard_val = self.execute_node(arg_refs[1])?;
            let mut guard_props = Hash::new();
            let _ = guard_props.insert("type".to_string(), Value::string("guard".to_string()));
            let _ = pattern_graph.add_node("guard:0".to_string(), Value::map(guard_props));
            let _ = pattern_graph.add_edge("pattern:root", "guard:0", "has_guard".to_string(), None, HashMap::new());
        }

        Ok(Value::graph(pattern_graph))
    }

    /// Helper: convert an execution graph node to a pattern node value (Hash/map).
    fn pattern_node_to_value(&self, key: &str, index: Option<usize>, node: &AstGraphNode) -> Result<crate::values::Hash> {
        use crate::values::Hash;
        let mut props = Hash::new();

        // Add key or index if present
        if !key.is_empty() {
            let _ = props.insert("key".to_string(), Value::string(key.to_string()));
        }
        if let Some(i) = index {
            let _ = props.insert("index".to_string(), Value::number(i as f64));
        }

        match &node.node_type {
            AstNodeType::Identifier => {
                let name = node.get_str("name").unwrap_or_default();
                if name == "_" {
                    let _ = props.insert("type".to_string(), Value::string("wildcard".to_string()));
                } else {
                    let _ = props.insert("type".to_string(), Value::string("binding".to_string()));
                    let _ = props.insert("name".to_string(), Value::string(name));
                }
            }
            AstNodeType::NumberLit => {
                let _ = props.insert("type".to_string(), Value::string("literal".to_string()));
                if let Some(n) = node.get_num("value") {
                    let _ = props.insert("value".to_string(), Value::number(n));
                }
            }
            AstNodeType::StringLit => {
                let _ = props.insert("type".to_string(), Value::string("literal".to_string()));
                if let Some(s) = node.get_str("value") {
                    let _ = props.insert("value".to_string(), Value::string(s));
                }
            }
            AstNodeType::BoolLit => {
                let _ = props.insert("type".to_string(), Value::string("literal".to_string()));
                if let Some(b) = node.get_bool("value") {
                    let _ = props.insert("value".to_string(), Value::boolean(b));
                }
            }
            AstNodeType::NoneLit => {
                let _ = props.insert("type".to_string(), Value::string("literal".to_string()));
                let _ = props.insert("value".to_string(), Value::none());
            }
            AstNodeType::SymbolLit => {
                let _ = props.insert("type".to_string(), Value::string("literal".to_string()));
                if let Some(s) = node.get_str("value") {
                    let _ = props.insert("value".to_string(), Value::symbol(s));
                }
            }
            _ => {
                let _ = props.insert("type".to_string(), Value::string("expression".to_string()));
            }
        }

        Ok(props)
    }

    /// BigNum methods: to_int, to_bigint
    fn eval_bignum_method(&self, bn: &crate::values::BigNum, method: &str, args: &[Value]) -> Result<Value> {
        use crate::values::BigNum;
        use num_bigint::BigInt;
        match method {
            "to_int" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'to_int' takes no arguments, but got {}", args.len()
                    )));
                }
                match bn {
                    BigNum::Int64(i) => Ok(Value::bignum(BigNum::Int64(*i))),
                    BigNum::UInt64(u) => {
                        if *u > i64::MAX as u64 {
                            return Err(GraphoidError::runtime(format!(
                                "UInt64 value {} exceeds Int64::MAX, cannot convert to int", u
                            )));
                        }
                        Ok(Value::bignum(BigNum::Int64(*u as i64)))
                    }
                    BigNum::Float128(f) => {
                        let f64_val: f64 = (*f).into();
                        let truncated = f64_val.trunc();
                        if truncated > i64::MAX as f64 || truncated < i64::MIN as f64 {
                            return Err(GraphoidError::runtime(
                                "Float128 value exceeds Int64 range, cannot convert to int".to_string()
                            ));
                        }
                        Ok(Value::bignum(BigNum::Int64(truncated as i64)))
                    }
                    BigNum::BigInt(bi) => {
                        use num_traits::ToPrimitive;
                        match bi.to_i64() {
                            Some(i) => Ok(Value::bignum(BigNum::Int64(i))),
                            None => Err(GraphoidError::runtime(
                                "BigInt value exceeds Int64 range, cannot convert to int".to_string()
                            )),
                        }
                    }
                }
            }
            "to_bigint" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'to_bigint' takes no arguments, but got {}", args.len()
                    )));
                }
                match bn {
                    BigNum::Int64(i) => Ok(Value::bignum(BigNum::BigInt(BigInt::from(*i)))),
                    BigNum::UInt64(u) => Ok(Value::bignum(BigNum::BigInt(BigInt::from(*u)))),
                    BigNum::Float128(f) => {
                        let f64_val: f64 = (*f).into();
                        let truncated = f64_val.trunc() as i64;
                        Ok(Value::bignum(BigNum::BigInt(BigInt::from(truncated))))
                    }
                    BigNum::BigInt(bi) => Ok(Value::bignum(BigNum::BigInt(bi.clone()))),
                }
            }
            "to_string" | "to_str" => {
                let s = match bn {
                    BigNum::Int64(n) => n.to_string(),
                    BigNum::UInt64(n) => n.to_string(),
                    BigNum::Float128(f) => { let v: f64 = (*f).into(); v.to_string() }
                    BigNum::BigInt(bi) => bi.to_string(),
                };
                Ok(Value::string(s))
            }
            "to_num" => {
                match bn {
                    BigNum::Int64(i) => Ok(Value::number(*i as f64)),
                    BigNum::UInt64(u) => Ok(Value::number(*u as f64)),
                    BigNum::Float128(f) => Ok(Value::number((*f).into())),
                    BigNum::BigInt(bi) => {
                        use num_traits::ToPrimitive;
                        Ok(Value::number(bi.to_f64().unwrap_or(f64::NAN)))
                    }
                }
            }
            _ => Err(GraphoidError::runtime(format!(
                "BigNum does not have method '{}'", method
            ))),
        }
    }

    fn eval_pattern_node_method(&self, pn: &crate::values::PatternNode, method: &str, args: &[Value]) -> Result<Value> {
        match method {
            "bind" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "PatternNode.bind() expects 1 argument (variable name), got {}", args.len()
                    )));
                }
                let new_variable = args[0].to_string_value();
                Ok(Value::pattern_node(Some(new_variable), pn.node_type.clone()))
            }
            "variable" => {
                Ok(pn.variable.as_ref().map(|v| Value::string(v.clone())).unwrap_or(Value::none()))
            }
            "type" => {
                Ok(pn.node_type.as_ref().map(|t| Value::string(t.clone())).unwrap_or(Value::none()))
            }
            "pattern_type" => Ok(Value::symbol("node".to_string())),
            _ => Err(GraphoidError::runtime(format!("PatternNode does not have method '{}'", method))),
        }
    }

    fn eval_pattern_edge_method(&self, pe: &crate::values::PatternEdge, method: &str, _args: &[Value]) -> Result<Value> {
        match method {
            "edge_type" => Ok(pe.edge_type.as_ref().map(|t| Value::string(t.clone())).unwrap_or(Value::none())),
            "direction" => Ok(Value::symbol(pe.direction.clone())),
            "pattern_type" => Ok(Value::symbol("edge".to_string())),
            _ => Err(GraphoidError::runtime(format!("PatternEdge does not have method '{}'", method))),
        }
    }

    fn eval_pattern_path_method(&self, pp: &crate::values::PatternPath, method: &str, _args: &[Value]) -> Result<Value> {
        match method {
            "edge_type" => Ok(Value::string(pp.edge_type.clone())),
            "min" => Ok(Value::number(pp.min as f64)),
            "max" => Ok(Value::number(pp.max as f64)),
            "direction" => Ok(Value::symbol(pp.direction.clone())),
            "pattern_type" => Ok(Value::symbol("path".to_string())),
            _ => Err(GraphoidError::runtime(format!("PatternPath does not have method '{}'", method))),
        }
    }

    fn eval_pattern_match_results_method(&mut self, results: &crate::values::PatternMatchResults, method: &str, args: &[Value]) -> Result<Value> {
        match method {
            "len" | "count" | "size" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "PatternMatchResults.{}() takes no arguments, but got {}", method, args.len()
                    )));
                }
                Ok(Value::number(results.len() as f64))
            }
            "where" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "PatternMatchResults.where() expects 1 argument (predicate function), got {}", args.len()
                    )));
                }
                let func = match &args[0].kind {
                    ValueKind::Function(f) => f.clone(),
                    _ => return Err(GraphoidError::runtime(format!(
                        "PatternMatchResults.where() expects a function, got {}", args[0].type_name()
                    ))),
                };
                let filtered = results.clone();
                let original_bindings = filtered.iter().cloned().collect::<Vec<_>>();
                let mut kept_bindings = Vec::new();
                for binding in original_bindings {
                    let mut map = crate::values::Hash::new();
                    for (var, node_id) in &binding {
                        if let Some(node) = filtered.graph().nodes.get(node_id) {
                            let _ = map.insert(var.clone(), node.value.clone());
                        }
                    }
                    let map_value = Value::map(map);
                    let result = self.call_graph_function(func.clone(), vec![map_value])?;
                    if result.is_truthy() {
                        kept_bindings.push(binding);
                    }
                }
                let new_results = crate::values::PatternMatchResults::new(kept_bindings, filtered.graph().clone());
                Ok(Value::pattern_match_results(new_results))
            }
            "select" | "return" => {
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "PatternMatchResults.return() expects 1 argument (list of specifiers), got {}", args.len()
                    )));
                }
                let specifiers = match &args[0].kind {
                    ValueKind::List(list) => {
                        list.to_vec().iter().map(|v| v.to_string_value()).collect::<Vec<String>>()
                    }
                    _ => return Err(GraphoidError::type_error("list", args[0].type_name())),
                };
                let mut projected_list = Vec::new();
                for binding in results.iter() {
                    let mut result_map = crate::values::Hash::new();
                    for spec in &specifiers {
                        if spec.contains('.') {
                            let parts: Vec<&str> = spec.splitn(2, '.').collect();
                            if parts.len() == 2 {
                                let var_name = parts[0];
                                let prop_name = parts[1];
                                if let Some(node_id) = binding.get(var_name) {
                                    if let Some(node) = results.graph().nodes.get(node_id) {
                                        let prop_value = match &node.value.kind {
                                            ValueKind::Map(map) => {
                                                map.get(prop_name).cloned().unwrap_or(Value::none())
                                            }
                                            _ => {
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
            _ => Err(GraphoidError::runtime(format!("PatternMatchResults does not have method '{}'", method))),
        }
    }

    /// Universal methods that work on any type (to_string, to_num, to_bool, etc.)
    fn try_universal_method(&self, value: &Value, method: &str, args: &[Value]) -> Result<Option<Value>> {
        match method {
            "to_string" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'to_string' takes no arguments, but got {}", args.len()
                    )));
                }
                Ok(Some(self.value_to_string_impl(value)))
            }
            "to_num" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'to_num' takes no arguments, but got {}", args.len()
                    )));
                }
                match &value.kind {
                    ValueKind::Number(_) => Ok(Some(value.clone())),
                    ValueKind::BigNumber(bn) => Ok(Some(Value::number(bn.to_f64()))),
                    ValueKind::String(s) => {
                        // Invalid strings return none (spec line 3296)
                        match s.parse::<f64>() {
                            Ok(n) => Ok(Some(Value::number(n))),
                            Err(_) => Ok(Some(Value::none())),
                        }
                    }
                    ValueKind::Boolean(b) => Ok(Some(Value::number(if *b { 1.0 } else { 0.0 }))),
                    ValueKind::None => Ok(Some(Value::number(0.0))),
                    ValueKind::Time(timestamp) => Ok(Some(Value::number(*timestamp))),
                    ValueKind::List(list) => Ok(Some(Value::number(list.len() as f64))),
                    ValueKind::Map(hash) => Ok(Some(Value::number(hash.len() as f64))),
                    _ => Ok(Some(Value::number(0.0))),
                }
            }
            "to_bool" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'to_bool' takes no arguments, but got {}", args.len()
                    )));
                }
                Ok(Some(Value::boolean(value.is_truthy())))
            }
            "freeze" => {
                let mut frozen_copy = value.clone();
                frozen_copy.freeze();
                Ok(Some(frozen_copy))
            }
            "is_frozen" => {
                Ok(Some(Value::boolean(value.is_frozen())))
            }
            "has_frozen" => {
                let wants_count = args.first().map_or(false, |arg| {
                    matches!(&arg.kind, ValueKind::Symbol(s) if s == "count")
                });
                let deep = args.get(1).map_or(false, |arg| {
                    matches!(&arg.kind, ValueKind::Symbol(s) if s == "deep")
                });
                if wants_count {
                    Ok(Some(self.eval_has_frozen_count(value, deep)?))
                } else {
                    Ok(Some(Value::boolean(self.check_has_frozen(value))))
                }
            }
            "to_bignum" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'to_bignum' takes no arguments, but got {}", args.len()
                    )));
                }
                Ok(Some(self.convert_to_bignum(value.clone())?))
            }
            "is_bignum" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'is_bignum' takes no arguments, but got {}", args.len()
                    )));
                }
                Ok(Some(Value::boolean(matches!(&value.kind, ValueKind::BigNumber(_)))))
            }
            "fits_in_num" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'fits_in_num' takes no arguments, but got {}", args.len()
                    )));
                }
                // Numbers always fit, BigNumbers may not
                let fits = match &value.kind {
                    ValueKind::Number(_) => true,
                    ValueKind::BigNumber(bn) => {
                        use crate::values::BigNum;
                        match bn {
                            BigNum::Int64(_) | BigNum::UInt64(_) => true,
                            BigNum::Float128(f) => {
                                let v: f64 = (*f).into();
                                v.is_finite()
                            }
                            BigNum::BigInt(bi) => {
                                use num_traits::ToPrimitive;
                                bi.to_f64().map(|f| f.is_finite()).unwrap_or(false)
                            }
                        }
                    }
                    _ => false,
                };
                Ok(Some(Value::boolean(fits)))
            }
            "type" => {
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'type' takes no arguments, but got {}", args.len()
                    )));
                }
                // Error and PatternNode have their own type() methods
                match &value.kind {
                    ValueKind::Error(ref err) => return Ok(Some(Value::string(err.error_type.clone()))),
                    ValueKind::PatternNode(_) => return Ok(None), // Handled by type-specific dispatch
                    _ => {}
                }
                Ok(Some(Value::string(value.type_name().to_string())))
            }
            "type_name" => {
                // Alias for type() method - mirrors Rust API naming
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "Method 'type_name' takes no arguments, but got {}", args.len()
                    )));
                }
                Ok(Some(Value::string(value.type_name().to_string())))
            }
            _ => Ok(None), // Not a universal method
        }
    }

    /// Convert a value to its string representation.
    fn value_to_string_impl(&self, value: &Value) -> Value {
        match &value.kind {
            ValueKind::String(s) => Value::string(s.clone()),
            ValueKind::Number(n) => Value::string(n.to_string()),
            ValueKind::Boolean(b) => Value::string(if *b { "true".to_string() } else { "false".to_string() }),
            ValueKind::None => Value::string(String::new()),
            ValueKind::List(list) => {
                let items = list.to_vec();
                let elements: Vec<String> = items.iter().map(|v| match &v.kind {
                    ValueKind::String(s) => format!("\"{}\"", s),
                    ValueKind::Number(n) => {
                        if n.fract() == 0.0 { format!("{:.0}", n) } else { n.to_string() }
                    }
                    ValueKind::Boolean(b) => b.to_string(),
                    ValueKind::None => "none".to_string(),
                    _ => v.type_name().to_string(),
                }).collect();
                Value::string(format!("[{}]", elements.join(", ")))
            }
            ValueKind::Map(m) => {
                let entries: Vec<String> = m.keys().iter().map(|k| {
                    let v = m.get(k).map(|v| v.to_string()).unwrap_or_default();
                    format!("\"{}\": {}", k, v)
                }).collect();
                Value::string(format!("{{{}}}", entries.join(", ")))
            }
            _ => Value::string(value.to_string()),
        }
    }

    fn dispatch_number_method(&self, num: &Value, method: &str, args: &[Value]) -> Result<Value> {
        match &num.kind {
            ValueKind::Number(n) => match method {
                "abs" => Ok(Value::number(n.abs())),
                "floor" => Ok(Value::number(n.floor())),
                "ceil" => Ok(Value::number(n.ceil())),
                "sqrt" => {
                    if !args.is_empty() {
                        return Err(GraphoidError::runtime(format!(
                            "Number method 'sqrt' takes no arguments, but got {}", args.len()
                        )));
                    }
                    Ok(Value::number(n.sqrt()))
                }
                "round" => {
                    if args.is_empty() {
                        return Ok(Value::number(n.round()));
                    }
                    match &args[0].kind {
                        ValueKind::Number(decimal_places) => {
                            let places = *decimal_places as i32;
                            let multiplier = 10_f64.powi(places);
                            Ok(Value::number((n * multiplier).round() / multiplier))
                        }
                        ValueKind::Symbol(mode) => match mode.as_str() {
                            "nearest_ten" => Ok(Value::number((n / 10.0).round() * 10.0)),
                            "nearest_hundred" => Ok(Value::number((n / 100.0).round() * 100.0)),
                            _ => Err(GraphoidError::runtime(format!("Unknown rounding mode: {}", mode))),
                        },
                        _ => Err(GraphoidError::type_error("number or symbol", args[0].type_name())),
                    }
                }
                "up" => {
                    if args.is_empty() {
                        return Ok(Value::number(n.ceil()));
                    }
                    match &args[0].kind {
                        ValueKind::Number(decimal_places) => {
                            let places = *decimal_places as i32;
                            let multiplier = 10_f64.powi(places);
                            Ok(Value::number((n * multiplier).ceil() / multiplier))
                        }
                        ValueKind::Symbol(mode) => match mode.as_str() {
                            "nearest_ten" => Ok(Value::number((n / 10.0).ceil() * 10.0)),
                            "nearest_hundred" => Ok(Value::number((n / 100.0).ceil() * 100.0)),
                            _ => Err(GraphoidError::runtime(format!("Unknown rounding mode: {}", mode))),
                        },
                        _ => Err(GraphoidError::type_error("number or symbol", args[0].type_name())),
                    }
                }
                "down" => {
                    if args.is_empty() {
                        return Ok(Value::number(n.floor()));
                    }
                    match &args[0].kind {
                        ValueKind::Number(decimal_places) => {
                            let places = *decimal_places as i32;
                            let multiplier = 10_f64.powi(places);
                            Ok(Value::number((n * multiplier).floor() / multiplier))
                        }
                        ValueKind::Symbol(mode) => match mode.as_str() {
                            "nearest_ten" => Ok(Value::number((n / 10.0).floor() * 10.0)),
                            "nearest_hundred" => Ok(Value::number((n / 100.0).floor() * 100.0)),
                            _ => Err(GraphoidError::runtime(format!("Unknown rounding mode: {}", mode))),
                        },
                        _ => Err(GraphoidError::type_error("number or symbol", args[0].type_name())),
                    }
                }
                "log" => {
                    if args.is_empty() {
                        return Ok(Value::number(n.ln()));
                    }
                    if args.len() > 1 {
                        return Err(GraphoidError::runtime(format!(
                            "Number method 'log' takes 0 or 1 arguments, but got {}", args.len()
                        )));
                    }
                    match &args[0].kind {
                        ValueKind::Number(base) => Ok(Value::number(n.log(*base))),
                        _ => Err(GraphoidError::type_error("number", args[0].type_name())),
                    }
                }
                "to_char" => {
                    let code = n.trunc() as i64;
                    if code < 0 || code > 127 {
                        return Err(GraphoidError::runtime(format!(
                            "to_char(): value {} is outside ASCII range (0-127)", code
                        )));
                    }
                    let ch = char::from_u32(code as u32)
                        .ok_or_else(|| GraphoidError::runtime(format!("Invalid character code: {}", code)))?;
                    Ok(Value::string(ch.to_string()))
                }
                "to_string" | "to_str" => Ok(Value::string(format!("{}", n))),
                "to_num" => Ok(Value::number(*n)),
                "to_bool" => Ok(Value::boolean(*n != 0.0)),
                _ => Err(GraphoidError::runtime(format!("Unknown number method: {}", method))),
            },
            _ => Err(GraphoidError::runtime(format!("Cannot call method '{}' on type '{}'", method, num.type_name()))),
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
                match items.get(actual_idx).cloned() {
                    Some(v) => Ok(v),
                    None => {
                        use crate::execution::ErrorMode;
                        match self.config_stack.current().error_mode {
                            ErrorMode::Lenient => Ok(Value::none()),
                            ErrorMode::Collect => {
                                let error = GraphoidError::runtime(format!(
                                    "List index out of bounds: index {} for list of length {}", idx, len
                                ));
                                self.error_collector.collect(error,
                                    self.current_file.as_ref().map(|p| p.to_string_lossy().to_string()),
                                    SourcePosition::unknown());
                                Ok(Value::none())
                            }
                            ErrorMode::Strict => Err(GraphoidError::runtime(format!("List index {} out of bounds", n))),
                        }
                    }
                }
            }
            (ValueKind::Map(m), ValueKind::String(key)) => {
                match m.get(key).cloned() {
                    Some(v) => Ok(v),
                    None => {
                        use crate::execution::ErrorMode;
                        match self.config_stack.current().error_mode {
                            ErrorMode::Lenient => Ok(Value::none()),
                            ErrorMode::Collect => {
                                let error = GraphoidError::runtime(format!("Map key not found: '{}'", key));
                                self.error_collector.collect(error,
                                    self.current_file.as_ref().map(|p| p.to_string_lossy().to_string()),
                                    SourcePosition::unknown());
                                Ok(Value::none())
                            }
                            ErrorMode::Strict => Err(GraphoidError::runtime(format!("Key '{}' not found", key))),
                        }
                    }
                }
            }
            (ValueKind::String(s), ValueKind::Number(n)) => {
                let idx = *n as i64;
                let len = s.len() as i64;
                let actual_idx = if idx < 0 { len + idx } else { idx } as usize;
                s.chars().nth(actual_idx)
                    .map(|c| Value::string(c.to_string()))
                    .ok_or_else(|| GraphoidError::runtime(format!("String index {} out of bounds", n)))
            }
            (ValueKind::Graph(g), ValueKind::String(key)) => {
                let graph = g.borrow();
                graph.get_node(key).cloned()
                    .ok_or_else(|| GraphoidError::runtime(format!("Node '{}' not found in graph", key)))
            }
            _ => Err(GraphoidError::type_error(
                "indexable type (list, map, string, or graph)",
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
            ValueKind::Graph(g) => {
                let graph = g.borrow();
                // Check __properties__/ branch first (CLG property access)
                let prop_node_id = crate::values::Graph::property_node_id(&property);
                if let Some(val) = graph.get_node(&prop_node_id) {
                    return Ok(val.clone());
                }
                // Then check regular nodes
                if let Some(val) = graph.get_node(&property) {
                    Ok(val.clone())
                } else {
                    // Return none for missing properties (consistent with other languages)
                    Ok(Value::none())
                }
            }
            ValueKind::Module(ref module) => {
                if module.private_symbols.contains(&property) {
                    return Err(GraphoidError::runtime(format!(
                        "Cannot access private symbol '{}' from module '{}'", property, module.name
                    )));
                }
                module.namespace.get(&property)
            }
            ValueKind::PatternNode(ref pn) => {
                self.eval_pattern_node_method(pn, &property, &[])
            }
            ValueKind::PatternEdge(ref pe) => {
                self.eval_pattern_edge_method(pe, &property, &[])
            }
            ValueKind::PatternPath(ref pp) => {
                self.eval_pattern_path_method(pp, &property, &[])
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

                // Extract the actual error type from the error message
                // User-raised errors are wrapped like: "Runtime error: ValueError: message"
                let error_message = e.to_string();
                let prefixes = [
                    "Runtime error: ", "Type error: ", "Syntax error: ",
                    "IO error: ", "Rule violation: ", "Module not found: ",
                    "Circular dependency: ", "Configuration error: ",
                ];
                let mut inner_message = error_message.as_str();
                for prefix in &prefixes {
                    if let Some(stripped) = inner_message.strip_prefix(prefix) {
                        inner_message = stripped;
                        break;
                    }
                }

                let (error_type_name, actual_message) = if let Some(colon_pos) = inner_message.find(':') {
                    let potential_type = &inner_message[..colon_pos];
                    if matches!(potential_type, "ValueError" | "TypeError" | "IOError" | "NetworkError" | "ParseError" | "RuntimeError" | "FileError" | "NetError") {
                        (potential_type.to_string(), inner_message[(colon_pos + 1)..].trim().to_string())
                    } else {
                        (e.error_type(), inner_message.to_string())
                    }
                } else {
                    (e.error_type(), inner_message.to_string())
                };

                // Try each catch handler
                let mut caught = false;
                let mut catch_result = Ok(Value::none());
                for catch_ref in &catch_refs {
                    let catch_error_type = self.get_str_property(*catch_ref, "error_type");
                    let variable = self.get_str_property(*catch_ref, "variable");
                    let catch_body_ref = self.get_edge_target(*catch_ref, &ExecEdgeType::Body);

                    // Check if this catch matches the error type (with hierarchy)
                    let matches = match &catch_error_type {
                        Some(et) => {
                            *et == error_type_name || {
                                let catch_id = format!("error:{}", et);
                                let actual_id = format!("error:{}", error_type_name);
                                self.universe_graph.borrow().has_path(&actual_id, &catch_id)
                            }
                        }
                        None => true, // Bare catch catches everything
                    };

                    if matches {
                        caught = true;
                        // Create a child scope for catch block
                        let parent_env_clone = self.env.clone();
                        self.env = Environment::with_parent(self.env.clone());

                        // Bind error to variable if specified
                        if let Some(var_name) = variable {
                            let error_obj = crate::values::ErrorObject::with_stack_trace(
                                error_type_name.clone(),
                                actual_message.clone(),
                                self.current_file.as_ref().map(|p| p.to_string_lossy().to_string()),
                                0, 0, self.call_stack.clone(),
                            );
                            self.env.define(var_name, Value::error(error_obj));
                        }
                        if let Some(body_ref) = catch_body_ref {
                            catch_result = self.execute_node(body_ref);
                        }

                        // Extract the modified parent environment from the child
                        if let Some(boxed_parent) = self.env.take_parent() {
                            self.env = *boxed_parent;
                        } else {
                            self.env = parent_env_clone;
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
        use crate::execution::ErrorMode;

        let val_ref = self.get_edge_target(node_ref, &ExecEdgeType::ValueEdge)
            .ok_or_else(|| GraphoidError::runtime("Missing raise value".to_string()))?;

        let node = self.get_node(node_ref)?;
        let position = node.position.clone();

        let value = self.execute_node(val_ref)?;

        let message = match &value.kind {
            ValueKind::Error(err_obj) => err_obj.full_message(),
            ValueKind::String(s) => s.clone(),
            _ => value.to_string(),
        };

        let graphoid_error = GraphoidError::runtime(message);

        // Check if we're in error collection mode
        if self.config_stack.current().error_mode == ErrorMode::Collect {
            self.error_collector.collect(
                graphoid_error,
                self.current_file.as_ref().map(|p| p.to_string_lossy().to_string()),
                position,
            );
            Ok(Value::none())
        } else {
            Err(graphoid_error)
        }
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

    // =========================================================================
    // Function call compatibility
    // =========================================================================

    /// Call a user-defined function (API-compatible with Executor::call_function).
    pub(crate) fn call_function(&mut self, func: &Function, arg_values: &[Value]) -> Result<Value> {
        self.call_graph_function(func.clone(), arg_values.to_vec())
    }

    // =========================================================================
    // Module system
    // =========================================================================

    fn exec_import(&mut self, node_ref: NodeRef) -> Result<Value> {
        let node = self.get_node(node_ref)?;
        let module_name = node.get_str("module").unwrap_or_default();
        let alias = node.get_str("alias");
        let selection_count = node.get_int("selection_count");

        // Use the module loading infrastructure
        let module_value = self.load_module(&module_name, alias.as_ref())?;

        // Phase 17: Selective import — bind selected symbols directly
        if let Some(count) = selection_count {
            let module = match &module_value.kind {
                ValueKind::Module(m) => m,
                _ => return Err(GraphoidError::runtime(
                    format!("'{}' is not a module", module_name)
                )),
            };

            let mut imported_items = Vec::new();
            for i in 0..count as u32 {
                let item_ref = self.get_edge_target(node_ref, &ExecEdgeType::Element(i))
                    .ok_or_else(|| GraphoidError::runtime(
                        format!("Missing import item {} in selective import", i)
                    ))?;
                let item_node = self.get_node(item_ref)?;
                let item_name = item_node.get_str("name").unwrap_or_default();
                let item_alias = item_node.get_str("alias");

                // Check privacy
                if module.private_symbols.contains(&item_name) {
                    return Err(GraphoidError::runtime(
                        format!("Cannot import private symbol '{}' from module '{}'", item_name, module_name)
                    ));
                }

                // Look up the value in the module's namespace
                let value = module.namespace.get(&item_name).map_err(|_| {
                    GraphoidError::runtime(
                        format!("Symbol '{}' not found in module '{}'", item_name, module_name)
                    )
                })?;

                // Bind using alias if provided, otherwise original name
                let bind_name = item_alias.unwrap_or(item_name.clone());
                self.env.define(bind_name, value);
                imported_items.push(item_name);
            }

            // Phase 18: Add selective import edge to universe graph
            {
                let mod_display = module.alias.clone().unwrap_or_else(|| module.name.clone());
                let mod_node_id = format!("module:{}", mod_display);
                let mut ug = self.universe_graph.borrow_mut();
                let mut props = HashMap::new();
                props.insert("items".to_string(), Value::string(imported_items.join(",")));
                let _ = ug.add_edge("scope:main", &mod_node_id, "imports".to_string(), None, props);
            }

            return Ok(Value::none());
        }

        // Full import: existing behavior — bind module value to name
        let binding_name = if let Some(alias_name) = alias {
            alias_name
        } else if let ValueKind::Module(ref m) = module_value.kind {
            m.alias.clone().unwrap_or_else(|| m.name.clone())
        } else {
            module_name
        };

        // Phase 18: Add full import edge to universe graph
        {
            let mod_node_id = format!("module:{}", binding_name);
            let mut ug = self.universe_graph.borrow_mut();
            let _ = ug.add_edge("scope:main", &mod_node_id, "imports".to_string(), None, HashMap::new());
        }

        self.env.define(binding_name, module_value);
        Ok(Value::none())
    }

    fn exec_module_decl(&mut self, node_ref: NodeRef) -> Result<Value> {
        let node = self.get_node(node_ref)?;
        let name = node.get_str("name").unwrap_or_default();
        let alias = node.get_str("alias");

        self.env.define("__module_name__".to_string(), Value::string(name));
        if let Some(alias_name) = alias {
            self.env.define("__module_alias__".to_string(), Value::string(alias_name));
        }
        Ok(Value::none())
    }

    fn exec_load(&mut self, node_ref: NodeRef) -> Result<Value> {
        let val_ref = self.get_edge_target(node_ref, &ExecEdgeType::ValueEdge)
            .ok_or_else(|| GraphoidError::runtime("Missing load path".to_string()))?;
        let path_val = self.execute_node(val_ref)?;
        let path_str = match &path_val.kind {
            ValueKind::String(s) => s.clone(),
            _ => return Err(GraphoidError::runtime("load path must be a string".to_string())),
        };
        self.execute_load(&path_str)?;
        Ok(Value::none())
    }

    /// Phase 17: Execute a priv { } block — run all child statements
    fn exec_priv_block(&mut self, node_ref: NodeRef) -> Result<Value> {
        let prev = self.in_priv_block;
        self.in_priv_block = true;
        let mut i = 0u32;
        let result = loop {
            match self.get_edge_target(node_ref, &ExecEdgeType::Element(i)) {
                Some(child_ref) => {
                    self.execute_node(child_ref)?;
                }
                None => break Ok(Value::none()),
            }
            i += 1;
        };
        self.in_priv_block = prev;
        result
    }

    fn load_module(&mut self, module_path: &str, _alias: Option<&String>) -> Result<Value> {
        use std::fs;
        use crate::execution::module_manager::Module;

        // Check for native modules first
        if self.module_manager.is_native_module(module_path) {
            let cache_key = format!("native:{}", module_path);
            if let Some(module) = self.module_manager.get_module(&cache_key) {
                return Ok(Value::module(module.clone()));
            }
            if let Some((native_env, native_alias)) = self.module_manager.get_native_module_env(module_path) {
                let native_exports: Vec<String> = native_env.get_all_bindings()
                    .into_iter().map(|(name, _)| name).collect();
                let module = Module {
                    name: module_path.to_string(),
                    alias: native_alias,
                    namespace: native_env,
                    file_path: PathBuf::from(format!("<native:{}>", module_path)),
                    config: None,
                    private_symbols: std::collections::HashSet::new(),
                    exports: native_exports,
                };
                self.module_manager.register_module(cache_key.clone(), module.clone());
                // Phase 17: Record dependency edge for native modules
                if let Some(ref current) = self.current_file {
                    self.module_manager.record_dependency(
                        &current.to_string_lossy(),
                        &cache_key,
                    );
                }
                self.register_module_in_universe(&module);
                return Ok(Value::module(module));
            }
        }

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
        self.module_manager.begin_loading(resolved_path.clone())?;

        // Read and execute module source in isolated environment
        let source = fs::read_to_string(&resolved_path)?;
        let module_env = Environment::new();
        let mut module_executor = GraphExecutor::with_env(module_env);
        module_executor.set_current_file(Some(resolved_path.clone()));

        // Pass magic variables
        for (name, value) in self.env.get_all_bindings() {
            if name.starts_with("__") {
                module_executor.env.define(name, value);
            }
        }

        // Set module executor's func ID counter to avoid collisions with parent
        module_executor.next_func_id = self.next_func_id;

        module_executor.execute_source(&source)?;

        // Update parent's func ID counter to account for IDs used by module
        self.next_func_id = module_executor.next_func_id;

        // Propagate magic variables back
        for (name, value) in module_executor.env.get_all_bindings() {
            if name.starts_with("__") && name != "__module_name__" && name != "__module_alias__" {
                self.env.define(name, value);
            }
        }

        let module_name = if let Some(v) = module_executor.get_variable("__module_name__") {
            if let ValueKind::String(name) = &v.kind { name.clone() }
            else { resolved_path.file_stem().and_then(|s| s.to_str()).unwrap_or("unnamed").to_string() }
        } else {
            resolved_path.file_stem().and_then(|s| s.to_str()).unwrap_or("unnamed").to_string()
        };

        let module_alias = if let Some(v) = module_executor.get_variable("__module_alias__") {
            if let ValueKind::String(alias) = &v.kind { Some(alias.clone()) } else { None }
        } else { None };

        // Phase 17: Compute exports (all bindings minus private symbols)
        let all_bindings: Vec<String> = module_executor.env.get_all_bindings()
            .into_iter().map(|(name, _)| name).collect();
        let module_exports: Vec<String> = all_bindings.into_iter()
            .filter(|name| !module_executor.private_symbols.contains(name))
            .filter(|name| !name.starts_with("__"))  // exclude magic variables
            .collect();

        let module = Module {
            name: module_name,
            alias: module_alias,
            namespace: module_executor.env.clone(),
            file_path: resolved_path.clone(),
            config: None,
            private_symbols: module_executor.private_symbols.clone(),
            exports: module_exports,
        };

        // Merge the module's execution graph into the parent's graph
        // Must happen BEFORE transferring body refs so we can remap NodeRefs
        let remap_offset = if let Some(module_graph) = module_executor.graph {
            if let Some(ref mut existing) = self.graph {
                let offset = existing.nodes.next_arena_id();
                existing.merge(module_graph);
                Some(offset)
            } else {
                self.graph = Some(module_graph);
                None // No remapping needed — graph is used directly
            }
        } else {
            None
        };

        let remap_ref = |nr: NodeRef, offset: Option<u32>| -> NodeRef {
            if let Some(off) = offset {
                NodeRef::new(crate::execution_graph::arena::ArenaId(nr.arena_id.0 + off), nr.index)
            } else {
                nr
            }
        };

        // Transfer graph function bodies with remapped NodeRefs
        for (func_id, body_ref) in &module_executor.graph_function_bodies {
            self.graph_function_bodies.insert(func_id.clone(), remap_ref(*body_ref, remap_offset));
        }

        // Transfer pattern clauses with remapped NodeRefs
        for (func_id, clauses) in &module_executor.graph_pattern_clauses {
            let remapped: Vec<GraphPatternClause> = clauses.iter().map(|c| {
                GraphPatternClause {
                    pattern_type: c.pattern_type.clone(),
                    pattern_value: c.pattern_value.clone(),
                    pattern_name: c.pattern_name.clone(),
                    guard_ref: c.guard_ref.map(|r| remap_ref(r, remap_offset)),
                    body_ref: remap_ref(c.body_ref, remap_offset),
                }
            }).collect();
            self.graph_pattern_clauses.insert(func_id.clone(), remapped);
        }

        // Also register functions from module into global_functions for resolution
        for (func_name, func_list) in &module_executor.global_functions {
            self.global_functions.entry(func_name.clone())
                .or_insert_with(Vec::new)
                .extend(func_list.iter().cloned());
        }

        let module_key = resolved_path.to_string_lossy().to_string();
        self.module_manager.register_module(module_key.clone(), module.clone());
        self.module_manager.end_loading(&resolved_path);

        self.register_module_in_universe(&module);

        // Phase 17: Record dependency edge (current file imports this module)
        if let Some(ref current) = self.current_file {
            self.module_manager.record_dependency(
                &current.to_string_lossy(),
                &module_key,
            );
        }

        Ok(Value::module(module))
    }

    fn execute_load(&mut self, path_str: &str) -> Result<()> {
        use std::fs;

        // For load(), first try the path relative to current working directory.
        // This is important for spec_runner which loads files like "tests/gspec/foo_spec.gr"
        // from the project root, even though spec_runner.gr itself is in stdlib.
        let cwd_path = PathBuf::from(path_str);
        let resolved_path = if cwd_path.exists() && cwd_path.is_file() {
            cwd_path.canonicalize().map_err(|e| GraphoidError::IOError {
                message: format!("Failed to canonicalize path: {}", e),
                position: crate::error::SourcePosition::unknown(),
            })?
        } else if let Some(ref current) = self.current_file {
            self.module_manager.resolve_module_path(path_str, Some(current))?
        } else {
            self.module_manager.resolve_module_path(path_str, None)?
        };

        let source = fs::read_to_string(&resolved_path)?;
        self.execute_source(&source)?;
        Ok(())
    }

    // =========================================================================
    // Configuration
    // =========================================================================

    fn exec_configure(&mut self, node_ref: NodeRef) -> Result<Value> {
        // Evaluate settings
        let edges = self.get_edges_cloned(node_ref);
        let mut config_changes = HashMap::new();

        for (edge_type, target) in &edges {
            if let ExecEdgeType::Setting(key) = edge_type {
                let val = self.execute_node(*target)?;
                config_changes.insert(key.clone(), val);
            }
        }

        // Push new config
        self.config_stack.push_with_changes(config_changes)?;

        // If there's a body, execute scoped then pop; otherwise keep active
        if let Some(body_ref) = self.get_edge_target(node_ref, &ExecEdgeType::Body) {
            let result = self.execute_node(body_ref);
            self.config_stack.pop();
            result
        } else {
            Ok(Value::none())
        }
    }

    fn exec_precision(&mut self, node_ref: NodeRef) -> Result<Value> {
        let node = self.get_node(node_ref)?;
        let places = node.get_int("places").map(|v| Some(v as usize)).unwrap_or(None);

        self.precision_stack.push(places);

        let body_ref = self.get_edge_target(node_ref, &ExecEdgeType::Body)
            .ok_or_else(|| GraphoidError::runtime("Missing precision body".to_string()))?;
        let result = self.execute_node(body_ref);

        self.precision_stack.pop();
        result
    }

    // =========================================================================
    // Graph declarations and expressions
    // =========================================================================

    fn exec_graph_decl(&mut self, node_ref: NodeRef) -> Result<Value> {
        use crate::values::Graph;
        use crate::values::graph::GraphType;

        let node = self.get_node(node_ref)?;
        let name = node.get_str("name").unwrap_or_default();
        let graph_type_str = node.get_str("graph_type");

        let graph_type = match graph_type_str.as_deref() {
            Some("directed") | Some("dag") => GraphType::Directed,
            Some("undirected") => GraphType::Undirected,
            Some("tree") => GraphType::Directed, // Trees are directed graphs with constraints
            _ => GraphType::Directed,
        };

        let mut graph = Graph::new(graph_type);
        graph.type_name = Some(name.clone());

        // Add rulesets for graph types
        match graph_type_str.as_deref() {
            Some("dag") => { graph.rulesets.push("dag".to_string()); }
            Some("tree") => { graph.rulesets.push("tree".to_string()); }
            _ => {}
        }

        // Process parent (CLG inheritance)
        if let Some(parent_ref) = self.get_edge_target(node_ref, &ExecEdgeType::Parent) {
            let parent_val = self.execute_node(parent_ref)?;
            if let ValueKind::Graph(parent_graph) = &parent_val.kind {
                graph = Graph::from_parent(parent_graph.borrow().clone());
                graph.type_name = Some(name.clone());
                graph.finalize_inheritance_node(&name);
            }
        }

        // Process properties
        let prop_refs = self.get_ordered_edges_cloned(node_ref, "Property");
        for prop_ref in prop_refs {
            let prop_node = self.get_node(prop_ref)?;
            let prop_name = prop_node.get_str("name").unwrap_or_default();

            if let Some(val_ref) = self.get_edge_target(prop_ref, &ExecEdgeType::ValueEdge) {
                let val = self.execute_node(val_ref)?;
                let node_id = Graph::property_node_id(&prop_name);
                graph.add_node(node_id.clone(), val).ok();
            } else {
                let node_id = Graph::property_node_id(&prop_name);
                graph.add_node(node_id, Value::none()).ok();
            }
        }

        // Process methods — collect method info first to avoid borrow conflicts
        let method_refs = self.get_ordered_edges_cloned(node_ref, "GraphMethod");
        struct MethodInfo {
            method_name: String,
            is_static: bool,
            is_private: bool,
            body_ref: NodeRef,
            param_names: Vec<String>,
            parameters: Vec<Parameter>,
            guard_ref: Option<NodeRef>,
        }
        let mut method_infos = Vec::new();
        for method_ref in &method_refs {
            let method_node = self.get_node(*method_ref)?;
            let method_name = method_node.get_str("name").unwrap_or_default();
            let is_static = method_node.get_bool("is_static").unwrap_or(false);
            let is_private = method_node.get_bool("is_private").unwrap_or(false);

            // Check for guard expression (stored as NodeRef, evaluated at dispatch time)
            let guard_ref = self.get_edge_target(*method_ref, &ExecEdgeType::Guard);

            if let Some(body_ref) = self.get_edge_target(*method_ref, &ExecEdgeType::Body) {
                let mut param_names = Vec::new();
                let mut parameters = Vec::new();
                let param_refs = self.get_ordered_edges_cloned(*method_ref, "Parameter");
                for param_ref in &param_refs {
                    let pn = self.get_node(*param_ref)?;
                    let pname = pn.get_str("name").unwrap_or_default();
                    let is_variadic = pn.get_bool("is_variadic").unwrap_or(false);
                    let has_default = self.get_edge_target(*param_ref, &ExecEdgeType::DefaultValue).is_some();
                    param_names.push(pname.clone());
                    parameters.push(Parameter {
                        name: pname,
                        default_value: if has_default {
                            Some(crate::ast::Expr::Literal {
                                value: crate::ast::LiteralValue::None,
                                position: SourcePosition::unknown(),
                            })
                        } else { None },
                        is_variadic,
                    });
                }
                method_infos.push(MethodInfo { method_name, is_static, is_private, body_ref, param_names, parameters, guard_ref });
            }
        }
        // Now register methods (mutable borrow is safe here)
        // Use variant counters to give each method overload a unique func_id
        let mut method_variant_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for mi in method_infos {
            {
                // Private methods are renamed with underscore prefix
                let registered_name = if mi.is_private {
                    format!("_{}", mi.method_name)
                } else {
                    mi.method_name.clone()
                };

                // Get variant index for this method name to ensure unique func_id
                let variant_idx = *method_variant_counts.get(&registered_name).unwrap_or(&0);
                method_variant_counts.insert(registered_name.clone(), variant_idx + 1);

                let func_id = format!("__graph_method_{}_{}_{}", name, registered_name, variant_idx);
                self.store_function_body(func_id.clone(), mi.body_ref);

                // Store guard NodeRef for dispatch-time evaluation
                if let Some(guard_ref) = mi.guard_ref {
                    self.graph_method_guards.insert(func_id.clone(), guard_ref);
                }

                let env = Rc::new(RefCell::new(self.env.clone()));
                let func = Function {
                    name: Some(registered_name.clone()),
                    params: mi.param_names,
                    parameters: mi.parameters,
                    body: Vec::new(),
                    pattern_clauses: None,
                    env,
                    node_id: Some(func_id),
                    is_setter: false,
                    is_static: mi.is_static,
                    guard: None,  // Guard evaluated at dispatch time via graph_method_guards
                };

                graph.attach_method(registered_name, func);
            }
        }

        // Process rules — apply via add_rule()
        let rule_refs = self.get_ordered_edges_cloned(node_ref, "Rule");
        for rule_ref in rule_refs {
            let rule_node = self.get_node(rule_ref)?;
            let rule_name = rule_node.get_str("name").unwrap_or_default();
            // Check for parameter value
            let param_val = if let Some(param_ref) = self.get_edge_target(rule_ref, &ExecEdgeType::ValueEdge) {
                let v = self.execute_node(param_ref)?;
                match &v.kind {
                    ValueKind::Number(n) => Some(*n),
                    _ => None,
                }
            } else {
                None
            };
            if let Ok(rule_spec) = Self::symbol_to_rule_spec(&rule_name, param_val) {
                use crate::graph::RuleInstance;
                let _ = graph.add_rule(RuleInstance::new(rule_spec));
            }
        }

        // Process config settings (readable, writable, frozen, etc.)
        let mut readable_props: Vec<String> = Vec::new();
        let mut writable_props: Vec<String> = Vec::new();
        let edges = self.get_edges_cloned(node_ref);
        for (edge_type, target) in &edges {
            if let ExecEdgeType::Setting(key) = edge_type {
                let cfg_node = self.get_node(*target)?;
                let count = cfg_node.get_int("count").unwrap_or(0) as usize;
                let mut values = Vec::new();
                for j in 0..count {
                    if let Some(v) = cfg_node.get_str(&format!("value_{}", j)) {
                        values.push(v);
                    }
                }
                match key.as_str() {
                    "frozen" => {
                        if values.iter().any(|v| v == "true") {
                            graph.freeze();
                        }
                    }
                    "readable" => { readable_props.extend(values); }
                    "writable" => { writable_props.extend(values); }
                    "accessible" => {
                        readable_props.extend(values.clone());
                        writable_props.extend(values);
                    }
                    "behaviors" => {
                        for rule_name in &values {
                            let spec = Self::symbol_to_rule_spec(rule_name, None)?;
                            graph.add_rule(crate::graph::RuleInstance::new(spec))?;
                        }
                    }
                    _ => {
                        let config_id = format!("__config__/{}", key);
                        let config_list: Vec<Value> = values.iter().map(|v| Value::string(v.clone())).collect();
                        graph.add_node(config_id, Value::list(crate::values::List::from_vec(config_list))).ok();
                    }
                }
            }
        }

        // Generate getter methods for readable properties
        for prop_name in &readable_props {
            if !graph.has_method(prop_name) {
                let getter = self.generate_getter_method(prop_name);
                graph.attach_method(prop_name.clone(), getter);
            }
        }

        // Generate setter methods for writable properties
        for prop_name in &writable_props {
            let setter_name = format!("set_{}", prop_name);
            if !graph.has_method(&setter_name) {
                let setter = self.generate_setter_method(prop_name);
                graph.attach_method(setter_name, setter);
            }
        }

        // Phase 18: Register graph template in universe graph
        {
            let node_id = format!("graph:{}", name);
            let mut ug = self.universe_graph.borrow_mut();
            if !ug.has_node(&node_id) {
                let _ = ug.add_node(node_id.clone(), Value::string(name.clone()));
                let _ = ug.add_edge(&node_id, "type:graph", "subtype_of".to_string(), None, HashMap::new());
            }
            // If graph has a parent, add subtype_of edge to parent graph node
            if let Some(ref parent) = graph.parent {
                if let Some(ref parent_name) = parent.type_name {
                    let parent_node_id = format!("graph:{}", parent_name);
                    if ug.has_node(&parent_node_id) {
                        let _ = ug.add_edge(&node_id, &parent_node_id, "subtype_of".to_string(), None, HashMap::new());
                    }
                }
            }
        }

        let graph_val = Value::graph(graph);
        self.env.define(name, graph_val.clone());
        Ok(graph_val)
    }

    fn exec_graph_expr(&mut self, node_ref: NodeRef) -> Result<Value> {
        use crate::values::Graph;
        use crate::values::graph::GraphType;

        // Get graph type from Setting("type") edge if present
        let mut graph_type_str: Option<String> = None;
        let mut ruleset_str: Option<String> = None;

        let edges = self.get_edges_cloned(node_ref);
        for (edge_type, target) in &edges {
            if let ExecEdgeType::Setting(key) = edge_type {
                let val = self.execute_node(*target)?;
                match key.as_str() {
                    "type" => {
                        graph_type_str = Some(val.to_string_value());
                    }
                    "ruleset" => {
                        ruleset_str = Some(val.to_string_value());
                    }
                    _ => {}
                }
            }
        }

        // Strip leading colon from symbol values (":directed" -> "directed")
        let type_str_normalized = graph_type_str.as_ref().map(|s| {
            if s.starts_with(':') { s[1..].to_string() } else { s.clone() }
        });
        let graph_type = match type_str_normalized.as_deref() {
            Some("directed") | Some("dag") => GraphType::Directed,
            Some("undirected") => GraphType::Undirected,
            Some("tree") => GraphType::Directed, // Trees are directed graphs with constraints
            _ => GraphType::Directed,
        };

        let mut graph = Graph::new(graph_type);

        // Apply ruleset if specified
        // Also normalize ruleset string (strip leading colon)
        let ruleset_normalized = ruleset_str.as_ref().map(|s| {
            if s.starts_with(':') { s[1..].to_string() } else { s.clone() }
        });
        if let Some(ruleset) = ruleset_normalized {
            graph.rulesets.push(ruleset);
        } else if let Some(ref type_str) = type_str_normalized {
            // Auto-apply ruleset for certain types
            match type_str.as_str() {
                "dag" => { graph.rulesets.push("dag".to_string()); }
                "tree" => { graph.rulesets.push("tree".to_string()); }
                _ => {}
            }
        }

        Ok(Value::graph(graph))
    }

    fn exec_instantiate(&mut self, node_ref: NodeRef) -> Result<Value> {
        let obj_ref = self.get_edge_target(node_ref, &ExecEdgeType::Object)
            .ok_or_else(|| GraphoidError::runtime("Missing instantiation target".to_string()))?;
        let base_val = self.execute_node(obj_ref)?;

        match &base_val.kind {
            ValueKind::Graph(base_graph) => {
                let mut new_graph = base_graph.borrow().clone();

                // Phase 18: Set template reference for edge-based method lookup
                // Methods are looked up via template traversal, not cloned onto instances
                new_graph.template = Some(std::rc::Rc::clone(base_graph));

                // Strip cloned methods from instance — they live on the template
                new_graph.remove_all_methods();

                // Apply overrides
                let override_refs = self.get_ordered_edges_cloned(node_ref, "Override");
                for ovr_ref in override_refs {
                    let ovr_node = self.get_node(ovr_ref)?;
                    let prop_name = ovr_node.get_str("key").unwrap_or_default();
                    if let Some(val_ref) = self.get_edge_target(ovr_ref, &ExecEdgeType::ValueEdge) {
                        let val = self.execute_node(val_ref)?;
                        let node_id = crate::values::Graph::property_node_id(&prop_name);
                        new_graph.add_node(node_id, val).ok();
                    }
                }

                Ok(Value::graph(new_graph))
            }
            _ => Err(GraphoidError::runtime("Can only instantiate graph types".to_string())),
        }
    }

    // =========================================================================
    // Pattern matching
    // =========================================================================

    fn exec_match(&mut self, node_ref: NodeRef) -> Result<Value> {
        let match_val_ref = self.get_edge_target(node_ref, &ExecEdgeType::MatchValue)
            .ok_or_else(|| GraphoidError::runtime("Missing match value".to_string()))?;
        let match_val = self.execute_node(match_val_ref)?;

        let arm_refs = self.get_ordered_edges_cloned(node_ref, "MatchArm");
        for arm_ref in arm_refs {
            let pattern_ref = self.get_edge_target(arm_ref, &ExecEdgeType::ArmPattern)
                .ok_or_else(|| GraphoidError::runtime("Missing match arm pattern".to_string()))?;
            let body_ref = self.get_edge_target(arm_ref, &ExecEdgeType::ArmBody)
                .ok_or_else(|| GraphoidError::runtime("Missing match arm body".to_string()))?;

            let pattern_node = self.get_node(pattern_ref)?;
            let matched = match &pattern_node.node_type {
                AstNodeType::MatchPatternNode => {
                    // Proper pattern node from converter
                    let pattern_type = pattern_node.get_str("pattern_type").unwrap_or_default();
                    match pattern_type.as_str() {
                        "wildcard" => true,
                        "variable" => {
                            let var_name = pattern_node.get_str("name").unwrap_or_default();
                            self.env.define(var_name, match_val.clone());
                            true
                        }
                        "literal" => {
                            self.match_literal_pattern(pattern_ref, &match_val)?
                        }
                        "list" => {
                            self.match_list_pattern(pattern_ref, &match_val)?
                        }
                        _ => false,
                    }
                }
                AstNodeType::Identifier => {
                    let var_name = pattern_node.get_str("name").unwrap_or_default();
                    if var_name == "_" {
                        true
                    } else {
                        self.env.define(var_name, match_val.clone());
                        true
                    }
                }
                AstNodeType::NumberLit | AstNodeType::StringLit |
                AstNodeType::BoolLit | AstNodeType::NoneLit |
                AstNodeType::SymbolLit => {
                    let pattern_val = self.execute_node(pattern_ref)?;
                    pattern_val == match_val
                }
                _ => {
                    let pattern_val = self.execute_node(pattern_ref)?;
                    pattern_val == match_val
                }
            };

            if matched {
                return self.execute_node(body_ref);
            }
        }

        Ok(Value::none()) // No arm matched
    }

    /// Match a literal pattern node against a value.
    fn match_literal_pattern(&self, pattern_ref: NodeRef, value: &Value) -> Result<bool> {
        let node = self.get_node(pattern_ref)?;
        match node.properties.get("value") {
            Some(AstProperty::Num(n)) => {
                if let ValueKind::Number(v) = &value.kind {
                    Ok((n - v).abs() < f64::EPSILON)
                } else {
                    Ok(false)
                }
            }
            Some(AstProperty::Str(s)) => {
                if let ValueKind::String(v) = &value.kind {
                    Ok(s == v)
                } else {
                    Ok(false)
                }
            }
            Some(AstProperty::Bool(b)) => {
                if let ValueKind::Boolean(v) = &value.kind {
                    Ok(b == v)
                } else {
                    Ok(false)
                }
            }
            Some(AstProperty::None) => Ok(matches!(&value.kind, ValueKind::None)),
            _ => Ok(false),
        }
    }

    /// Match a list pattern node against a value (for rest patterns in match expressions).
    fn match_list_pattern(&mut self, pattern_ref: NodeRef, value: &Value) -> Result<bool> {
        let list = match &value.kind {
            ValueKind::List(l) => l.clone(),
            _ => return Ok(false),
        };
        let items = list.to_vec();

        let node = self.get_node(pattern_ref)?;
        let element_count = match node.properties.get("element_count") {
            Some(AstProperty::Int(n)) => *n as usize,
            _ => 0,
        };
        let rest_name = node.get_str("rest_name");

        if let Some(rest) = rest_name {
            // List pattern with rest: [a, b, ...rest]
            if items.len() < element_count {
                return Ok(false);
            }
            // Bind fixed element sub-patterns
            let elem_refs = self.get_ordered_edges_cloned(pattern_ref, "Element");
            for (i, elem_ref) in elem_refs.iter().enumerate() {
                if i < items.len() {
                    let sub_node = self.get_node(*elem_ref)?;
                    let sub_type = sub_node.get_str("pattern_type").unwrap_or_default();
                    match sub_type.as_str() {
                        "variable" => {
                            let var_name = sub_node.get_str("name").unwrap_or_default();
                            self.env.define(var_name, items[i].clone());
                        }
                        "literal" => {
                            if !self.match_literal_pattern(*elem_ref, &items[i])? {
                                return Ok(false);
                            }
                        }
                        "wildcard" => {} // matches anything, no binding
                        _ => {}
                    }
                }
            }
            // Bind rest
            let rest_items: Vec<Value> = items[element_count..].to_vec();
            self.env.define(rest, Value::list(crate::values::List::from_vec(rest_items)));
            Ok(true)
        } else {
            // Exact list match: [a, b, c]
            if items.len() != element_count {
                return Ok(false);
            }
            // Bind fixed element sub-patterns
            let elem_refs = self.get_ordered_edges_cloned(pattern_ref, "Element");
            for (i, elem_ref) in elem_refs.iter().enumerate() {
                if i < items.len() {
                    let sub_node = self.get_node(*elem_ref)?;
                    let sub_type = sub_node.get_str("pattern_type").unwrap_or_default();
                    match sub_type.as_str() {
                        "variable" => {
                            let var_name = sub_node.get_str("name").unwrap_or_default();
                            self.env.define(var_name, items[i].clone());
                        }
                        "literal" => {
                            if !self.match_literal_pattern(*elem_ref, &items[i])? {
                                return Ok(false);
                            }
                        }
                        "wildcard" => {}
                        _ => {}
                    }
                }
            }
            Ok(true)
        }
    }

    fn exec_super_method_call(&mut self, node_ref: NodeRef) -> Result<Value> {
        let node = self.get_node(node_ref)?;
        let method_name = node.get_str("method").unwrap_or_default();

        let arg_refs = self.get_ordered_edges_cloned(node_ref, "Argument");
        let mut args = Vec::new();
        for arg_ref in arg_refs {
            args.push(self.execute_node(arg_ref)?);
        }

        // Get the current self graph and look up the method on its parent
        // super.method() calls the parent's method but with `self` bound to the child
        let self_value = self.env.get("self")?;
        if let ValueKind::Graph(ref graph_rc) = self_value.kind {
            // Clone what we need before releasing borrows
            let child_graph = graph_rc.borrow().clone();
            let func_to_call: Option<Function> = {
                if let Some(ref parent_box) = child_graph.parent {
                    let parent = parent_box.as_ref();
                    let method_node_id = format!("__methods__/{}", method_name);
                    if let Some(method_val) = parent.get_node(&method_node_id) {
                        if let ValueKind::Function(func) = &method_val.kind {
                            Some(func.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    return Err(GraphoidError::runtime("No parent graph available for super call".to_string()));
                }
            };

            if let Some(func) = func_to_call {
                // Call parent's method with `self` bound to child graph
                // This ensures properties like `speed` access the child's values
                let self_expr = Expr::Variable {
                    name: "self".to_string(),
                    position: SourcePosition::unknown(),
                };
                return self.call_graph_method_impl(&child_graph, &func, &args, &self_expr, false);
            }
            return Err(GraphoidError::runtime(format!("No method '{}' on parent graph", method_name)));
        }
        Err(GraphoidError::runtime("super can only be used within a graph method".to_string()))
    }

    // =========================================================================
    // Helper: clone edges to avoid borrow conflicts
    // =========================================================================

    fn get_edges_cloned(&self, node_ref: NodeRef) -> Vec<(ExecEdgeType, NodeRef)> {
        self.graph.as_ref()
            .map(|g| g.get_edges(node_ref).to_vec())
            .unwrap_or_default()
    }

    fn get_ordered_edges_cloned(&self, node_ref: NodeRef, prefix: &str) -> Vec<NodeRef> {
        self.graph.as_ref()
            .map(|g| g.get_ordered_edges(node_ref, prefix))
            .unwrap_or_default()
    }

    fn store_function_body(&mut self, func_id: String, body_ref: NodeRef) {
        self.graph_function_bodies.insert(func_id, body_ref);
    }

    /// Generate a getter method for a configure { readable: } property.
    fn generate_getter_method(&self, prop_name: &str) -> Function {
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

    /// Generate a setter method for a configure { writable: } property.
    fn generate_setter_method(&self, prop_name: &str) -> Function {
        use crate::ast::AssignmentTarget;
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

    // =========================================================================
    // API-compatible methods (matching Executor's public interface)
    // =========================================================================

    /// Evaluate a single AST statement (API-compatible with Executor).
    pub fn eval_stmt(&mut self, stmt: &Stmt) -> Result<Option<Value>> {
        let program = Program { statements: vec![stmt.clone()] };
        let mut converter = AstToGraphConverter::new();
        let root = converter.convert_program(&program);
        let exec_graph = converter.into_graph();
        let val = self.execute(exec_graph, root)?;
        Ok(Some(val))
    }

    /// Evaluate a single AST expression (API-compatible with Executor).
    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value> {
        let stmt = Stmt::Expression {
            expr: expr.clone(),
            position: SourcePosition::unknown(),
        };
        let program = Program { statements: vec![stmt] };
        let mut converter = AstToGraphConverter::new();
        let root = converter.convert_program(&program);
        let exec_graph = converter.into_graph();
        self.execute(exec_graph, root)
    }

    /// Set the current file for module resolution.
    pub fn set_current_file(&mut self, path: Option<PathBuf>) {
        self.current_file = path;
    }

    /// Enable output capture (for testing).
    pub fn enable_output_capture(&mut self) {
        self.output_capture_enabled = true;
    }

    /// Get captured output and reset the buffer.
    pub fn get_captured_output(&mut self) -> String {
        std::mem::take(&mut self.output_buffer)
    }

    /// Get a reference to the environment.
    pub fn env(&self) -> &Environment {
        &self.env
    }

    /// Get a mutable reference to the environment.
    pub fn env_mut(&mut self) -> &mut Environment {
        &mut self.env
    }

    /// Get the call stack.
    pub fn call_stack(&self) -> &[String] {
        &self.call_stack
    }

    /// Create an executor with an existing environment.
    pub fn with_env(env: Environment) -> Self {
        let mut executor = Self::new();
        executor.env = env;
        executor
    }

    /// Apply transformation rules with executor context (API-compatible with Executor).
    pub fn apply_transformation_rules_with_context(
        &mut self,
        value: Value,
        rules: &[crate::graph::RuleInstance],
    ) -> Result<Value> {
        use crate::graph::RuleSpec;

        let mut current = value;

        for rule_instance in rules {
            if !rule_instance.spec.is_transformation_rule() {
                continue;
            }

            match &rule_instance.spec {
                RuleSpec::CustomFunction { function } => {
                    match &function.kind {
                        ValueKind::Function(func) => {
                            current = self.call_graph_function(func.clone(), vec![current])?;
                        }
                        _ => {
                            return Err(GraphoidError::runtime(
                                "CustomFunction behavior requires a function value".to_string()
                            ));
                        }
                    }
                }
                RuleSpec::Conditional { condition, transform, fallback } => {
                    let condition_func = match &condition.kind {
                        ValueKind::Function(f) => f,
                        _ => {
                            return Err(GraphoidError::runtime(
                                "Conditional behavior condition must be a function".to_string()
                            ));
                        }
                    };

                    let condition_result = self.call_graph_function(condition_func.clone(), vec![current.clone()])?;
                    let is_truthy = condition_result.is_truthy();

                    if is_truthy {
                        let transform_func = match &transform.kind {
                            ValueKind::Function(f) => f,
                            _ => {
                                return Err(GraphoidError::runtime(
                                    "Conditional behavior transform must be a function".to_string()
                                ));
                            }
                        };
                        current = self.call_graph_function(transform_func.clone(), vec![current])?;
                    } else if let Some(fallback_val) = fallback {
                        let fallback_func = match &fallback_val.kind {
                            ValueKind::Function(f) => f,
                            _ => {
                                return Err(GraphoidError::runtime(
                                    "Conditional behavior fallback must be a function".to_string()
                                ));
                            }
                        };
                        current = self.call_graph_function(fallback_func.clone(), vec![current])?;
                    }
                }
                _ => {
                    let rule = rule_instance.spec.instantiate();
                    current = rule.transform(&current)?;
                }
            }
        }

        Ok(current)
    }

    /// Compare two values using a custom comparison function (API-compatible with Executor).
    pub fn compare_with_function(
        &mut self,
        a: &Value,
        b: &Value,
        func: &Function,
    ) -> Result<std::cmp::Ordering> {
        use std::cmp::Ordering;

        let result = self.call_graph_function(func.clone(), vec![a.clone(), b.clone()])?;

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

    /// Find insertion point in sorted list (API-compatible with Executor).
    pub fn find_insertion_point(
        &mut self,
        values: &[Value],
        new_value: &Value,
        compare_fn: &Option<Value>,
    ) -> Result<usize> {
        if values.is_empty() {
            return Ok(0);
        }
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

    /// Compare two values for ordering (API-compatible with Executor).
    pub fn compare_values(&self, a: &Value, b: &Value) -> Result<std::cmp::Ordering> {
        use std::cmp::Ordering;
        match (&a.kind, &b.kind) {
            (ValueKind::None, ValueKind::None) => Ok(Ordering::Equal),
            (ValueKind::Boolean(a), ValueKind::Boolean(b)) => Ok(a.cmp(b)),
            (ValueKind::Number(a), ValueKind::Number(b)) => {
                if a.is_nan() && b.is_nan() { Ok(Ordering::Equal) }
                else if a.is_nan() { Ok(Ordering::Greater) }
                else if b.is_nan() { Ok(Ordering::Less) }
                else { Ok(a.partial_cmp(b).unwrap_or(Ordering::Equal)) }
            }
            (ValueKind::String(a), ValueKind::String(b)) => Ok(a.cmp(b)),
            (ValueKind::Symbol(a), ValueKind::Symbol(b)) => Ok(a.cmp(b)),
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
            _ => Ok(Ordering::Equal),
        }
    }

    // =========================================================================
    // Methods required by src/execution/methods/ files
    // =========================================================================

    /// Apply a named transformation to a value (double, square, negate, etc.)
    pub(crate) fn apply_named_transformation(&self, value: &Value, transform_name: &str) -> Result<Value> {
        match transform_name {
            "double" => match &value.kind {
                ValueKind::Number(n) => Ok(Value::number(n * 2.0)),
                _ => Err(GraphoidError::runtime(format!("Transformation 'double' requires a number, got {}", value.type_name()))),
            },
            "square" => match &value.kind {
                ValueKind::Number(n) => Ok(Value::number(n * n)),
                _ => Err(GraphoidError::runtime(format!("Transformation 'square' requires a number, got {}", value.type_name()))),
            },
            "negate" => match &value.kind {
                ValueKind::Number(n) => Ok(Value::number(-n)),
                _ => Err(GraphoidError::runtime(format!("Transformation 'negate' requires a number, got {}", value.type_name()))),
            },
            "increment" | "inc" => match &value.kind {
                ValueKind::Number(n) => Ok(Value::number(n + 1.0)),
                _ => Err(GraphoidError::runtime(format!("Transformation 'increment' requires a number, got {}", value.type_name()))),
            },
            "decrement" | "dec" => match &value.kind {
                ValueKind::Number(n) => Ok(Value::number(n - 1.0)),
                _ => Err(GraphoidError::runtime(format!("Transformation 'decrement' requires a number, got {}", value.type_name()))),
            },
            _ => Err(GraphoidError::runtime(format!("Unknown named transformation: '{}'", transform_name))),
        }
    }

    /// Apply a named predicate to a value (even, odd, positive, negative, zero)
    pub(crate) fn apply_named_predicate(&self, value: &Value, predicate_name: &str) -> Result<bool> {
        match predicate_name {
            "even" => match &value.kind {
                ValueKind::Number(n) => Ok((n % 2.0).abs() < 0.0001),
                _ => Err(GraphoidError::runtime(format!("Predicate 'even' requires a number, got {}", value.type_name()))),
            },
            "odd" => match &value.kind {
                ValueKind::Number(n) => Ok((n % 2.0).abs() > 0.0001),
                _ => Err(GraphoidError::runtime(format!("Predicate 'odd' requires a number, got {}", value.type_name()))),
            },
            "positive" | "pos" => match &value.kind {
                ValueKind::Number(n) => Ok(*n > 0.0),
                _ => Err(GraphoidError::runtime(format!("Predicate 'positive' requires a number, got {}", value.type_name()))),
            },
            "negative" | "neg" => match &value.kind {
                ValueKind::Number(n) => Ok(*n < 0.0),
                _ => Err(GraphoidError::runtime(format!("Predicate 'negative' requires a number, got {}", value.type_name()))),
            },
            "zero" => match &value.kind {
                ValueKind::Number(n) => Ok(n.abs() < 0.0001),
                _ => Err(GraphoidError::runtime(format!("Predicate 'zero' requires a number, got {}", value.type_name()))),
            },
            _ => Err(GraphoidError::runtime(format!("Unknown named predicate: '{}'", predicate_name))),
        }
    }

    /// Call a static method (class method) without binding `self`.
    pub(crate) fn call_static_method(&mut self, func: &Function, arg_values: &[Value]) -> Result<Value> {
        if arg_values.len() != func.parameters.len() {
            return Err(GraphoidError::runtime(format!(
                "Static method '{}' expects {} arguments, but got {}",
                func.name.as_ref().unwrap_or(&"<anonymous>".to_string()),
                func.parameters.len(),
                arg_values.len()
            )));
        }

        let method_name = func.name.as_ref().unwrap_or(&"<anonymous>".to_string()).clone();
        self.call_stack.push(method_name);

        if let Some(ref func_id) = func.node_id {
            self.function_graph.borrow_mut().push_call(func_id.clone(), arg_values.to_vec());
        }

        // Save existing bindings for parameter names
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

        // Restore original parameter bindings
        for (name, original) in saved_params {
            if let Some(val) = original {
                self.env.define(name, val);
            } else {
                self.env.remove_variable(&name);
            }
        }

        self.call_stack.pop();
        self.function_graph.borrow_mut().pop_call(return_value.clone());

        Ok(return_value)
    }

    /// Call a graph method with `self` binding.
    pub(crate) fn call_graph_method(&mut self, graph: &crate::values::Graph, func: &Function, arg_values: &[Value], object_expr: &Expr) -> Result<Value> {
        self.call_graph_method_impl(graph, func, arg_values, object_expr, true)
    }

    /// Internal implementation of call_graph_method.
    pub(crate) fn call_graph_method_impl(
        &mut self,
        graph: &crate::values::Graph,
        func: &Function,
        arg_values: &[Value],
        object_expr: &Expr,
        manage_super_context: bool,
    ) -> Result<Value> {
        use crate::graph::RuleSpec;

        if arg_values.len() != func.parameters.len() {
            return Err(GraphoidError::runtime(format!(
                "Method '{}' expects {} arguments, but got {}",
                func.name.as_ref().unwrap_or(&"<anonymous>".to_string()),
                func.parameters.len(),
                arg_values.len()
            )));
        }

        // Capture graph state before method execution for constraint checking
        let before_node_ids: std::collections::HashSet<String> = graph.constrainable_node_ids().into_iter().collect();
        let before_edge_count = graph.data_edge_list().len();

        let method_name = func.name.as_ref().unwrap_or(&"<anonymous>".to_string()).clone();
        self.call_stack.push(method_name.clone());

        // Push graph variable name to method context stack
        let graph_var_name = if let Expr::Variable { name, .. } = object_expr {
            Some(name.clone())
        } else {
            None
        };
        if let Some(ref var_name) = graph_var_name {
            self.method_context_stack.push(var_name.clone());
        }

        if manage_super_context {
            self.super_context_stack.push(graph.clone());
        }

        // Push self onto block_self_stack
        let block_self_value = if let Some(original_value) = self.graph_method_value_stack.last() {
            original_value.clone()
        } else {
            Value::graph(graph.clone())
        };
        self.block_self_stack.push(block_self_value);

        if let Some(ref func_id) = func.node_id {
            self.function_graph.borrow_mut().push_call(func_id.clone(), arg_values.to_vec());
        }

        let saved_suppress = self.suppress_self_property_assignment;
        self.suppress_self_property_assignment = 0;

        // Save current environment
        let saved_env_clone = self.env.clone();

        let call_env = if func.is_static {
            Environment::with_parent(saved_env_clone)
        } else {
            let parent_env = func.env.borrow().clone();
            Environment::with_parent(parent_env)
        };

        let saved_env = std::mem::replace(&mut self.env, call_env);

        // Bind `self` to the graph
        let self_value = if let Some(original_value) = self.graph_method_value_stack.last() {
            original_value.clone()
        } else {
            Value::graph(graph.clone())
        };
        self.env.define("self".to_string(), self_value);

        // Bind class name from saved env for instance methods
        if !func.is_static {
            if let Some(type_name) = &graph.type_name {
                if let Ok(class_value) = saved_env.get(type_name) {
                    self.env.define(type_name.clone(), class_value);
                }
            }
        }

        // Bind parameters
        for (param, arg) in func.parameters.iter().zip(arg_values.iter()) {
            self.env.define(param.name.clone(), arg.clone());
        }

        // Execute function body
        let mut return_value = Value::none();
        let execution_result: Result<()> = (|| {
            if func.body.is_empty() {
                // Graph-based function body: use stored node_id to find body
                if let Some(ref func_id) = func.node_id {
                    if let Some(&body_ref) = self.graph_function_bodies.get(func_id) {
                        match self.execute_node(body_ref) {
                            Ok(val) => { return_value = val; }
                            Err(GraphoidError::ReturnControl { value }) => {
                                return_value = value;
                            }
                            Err(e) => return Err(e),
                        }
                    }
                }
            } else {
                for stmt in &func.body {
                    if let Some(ret_val) = self.eval_stmt(stmt)? {
                        return_value = ret_val;
                        return Ok(());
                    }
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

        // Get modified self before restoring environment
        let modified_self = self.env.get("self").ok();

        // Restore original environment
        self.env = saved_env;

        // Check method constraints before persisting changes
        if let Some(Value { kind: ValueKind::Graph(ref modified_graph_rc), .. }) = modified_self {
            let modified_graph = modified_graph_rc.borrow();
            let after_node_ids: std::collections::HashSet<String> = modified_graph.constrainable_node_ids().into_iter().collect();
            let after_edge_count = modified_graph.data_edge_list().len();

            for rule_instance in &graph.rules {
                if rule_instance.spec.is_method_constraint() {
                    match &rule_instance.spec {
                        RuleSpec::ReadOnly => {
                            if before_node_ids != after_node_ids || before_edge_count != after_edge_count {
                                self.suppress_self_property_assignment = saved_suppress;
                                self.call_stack.pop();
                                if func.node_id.is_some() {
                                    self.function_graph.borrow_mut().pop_call(Value::none());
                                }
                                return Err(GraphoidError::runtime(format!(
                                    "Method '{}' violates :read_only constraint: graph was modified", method_name
                                )));
                            }
                        }
                        RuleSpec::NoNodeRemovals => {
                            let removed: Vec<_> = before_node_ids.difference(&after_node_ids).collect();
                            if !removed.is_empty() {
                                self.suppress_self_property_assignment = saved_suppress;
                                self.call_stack.pop();
                                if func.node_id.is_some() {
                                    self.function_graph.borrow_mut().pop_call(Value::none());
                                }
                                return Err(GraphoidError::runtime(format!(
                                    "Method '{}' violates :no_node_removals constraint: removed node(s) {:?}", method_name, removed
                                )));
                            }
                        }
                        RuleSpec::NoEdgeRemovals => {
                            if after_edge_count < before_edge_count {
                                self.suppress_self_property_assignment = saved_suppress;
                                self.call_stack.pop();
                                if func.node_id.is_some() {
                                    self.function_graph.borrow_mut().pop_call(Value::none());
                                }
                                return Err(GraphoidError::runtime(format!(
                                    "Method '{}' violates :no_edge_removals constraint: edges removed", method_name
                                )));
                            }
                        }
                        RuleSpec::CustomMethodConstraint { function, name } => {
                            let before_graph_value = Value::graph(graph.clone());
                            let after_graph_value = Value::graph(modified_graph.clone());
                            let result = match &function.kind {
                                ValueKind::Function(constraint_func) => {
                                    self.call_function(constraint_func, &[before_graph_value, after_graph_value])
                                }
                                _ => Err(GraphoidError::runtime("Custom method constraint must be a function".to_string())),
                            };
                            match result {
                                Ok(val) => {
                                    let is_allowed = match &val.kind {
                                        ValueKind::Boolean(b) => *b,
                                        ValueKind::None => false,
                                        ValueKind::Number(n) => *n != 0.0,
                                        _ => true,
                                    };
                                    if !is_allowed {
                                        self.suppress_self_property_assignment = saved_suppress;
                                        self.call_stack.pop();
                                        if func.node_id.is_some() {
                                            self.function_graph.borrow_mut().pop_call(Value::none());
                                        }
                                        return Err(GraphoidError::runtime(format!(
                                            "Method '{}' violates custom constraint '{}': constraint returned false", method_name, name
                                        )));
                                    }
                                }
                                Err(e) => {
                                    self.suppress_self_property_assignment = saved_suppress;
                                    self.call_stack.pop();
                                    if func.node_id.is_some() {
                                        self.function_graph.borrow_mut().pop_call(Value::none());
                                    }
                                    return Err(GraphoidError::runtime(format!(
                                        "Method '{}': custom constraint '{}' failed: {}", method_name, name, e
                                    )));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // Persist mutations to `self` back to the original graph variable
        if let Some(modified_graph_value) = modified_self {
            if let Expr::Variable { name, .. } = object_expr {
                if self.env.set(name, modified_graph_value.clone()).is_err() {
                    if let Ok(outer_self) = self.env.get("self") {
                        if let ValueKind::Graph(ref outer_graph_rc) = outer_self.kind {
                            let mut outer_graph = outer_graph_rc.borrow_mut();
                            if outer_graph.has_node(name) {
                                outer_graph.add_node(name.to_string(), modified_graph_value)?;
                            }
                        }
                    }
                }
            }
        }

        self.suppress_self_property_assignment = saved_suppress;
        self.call_stack.pop();

        if graph_var_name.is_some() {
            self.method_context_stack.pop();
        }
        if manage_super_context {
            self.super_context_stack.pop();
        }

        // Pop block_self_stack
        self.block_self_stack.pop();

        // Pop function call from graph
        self.function_graph.borrow_mut().pop_call(return_value.clone());

        // Now check execution result
        execution_result?;

        Ok(return_value)
    }

    /// Match a graph pattern against a graph, returning all variable bindings.
    pub(crate) fn match_pattern(
        &self,
        graph: &crate::values::Graph,
        pattern: &crate::ast::GraphPattern,
    ) -> Result<Vec<std::collections::HashMap<String, String>>> {
        use std::collections::HashMap;

        let mut all_matches = Vec::new();

        if pattern.nodes.is_empty() {
            return Ok(all_matches);
        }

        let first_pattern_node = &pattern.nodes[0];

        for node_id in graph.nodes.keys() {
            if !self.node_matches_type(graph, node_id, first_pattern_node)? {
                continue;
            }

            let mut bindings = HashMap::new();
            bindings.insert(first_pattern_node.variable.clone(), node_id.clone());

            self.extend_pattern_match_all(graph, pattern, bindings, 0, &mut all_matches)?;
        }

        Ok(all_matches)
    }

    fn node_matches_type(
        &self,
        graph: &crate::values::Graph,
        node_id: &str,
        pattern_node: &crate::ast::PatternNode,
    ) -> Result<bool> {
        if let Some(ref required_type) = pattern_node.node_type {
            let node = graph.nodes.get(node_id)
                .ok_or_else(|| GraphoidError::runtime("Internal error: node not found in graph".to_string()))?;
            match &node.node_type {
                Some(actual_type) => Ok(actual_type == required_type),
                None => Ok(false),
            }
        } else {
            Ok(true)
        }
    }

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

        queue.push_back((from_id.to_string(), 0));

        while let Some((current_id, depth)) = queue.pop_front() {
            if depth >= max_hops {
                if depth == max_hops && depth >= min_hops {
                    results.push((current_id.clone(), depth));
                }
                continue;
            }

            let current_node = match graph.nodes.get(&current_id) {
                Some(node) => node,
                None => continue,
            };

            for (neighbor_id, edge_info) in &current_node.neighbors {
                if let Some(required_type) = edge_type {
                    if edge_info.edge_type != required_type {
                        continue;
                    }
                }

                let new_depth = depth + 1;

                if new_depth >= min_hops && new_depth <= max_hops {
                    results.push((neighbor_id.clone(), new_depth));
                }

                let state_key = (neighbor_id.clone(), new_depth);
                if !visited.contains(&state_key) && new_depth < max_hops {
                    visited.insert(state_key);
                    queue.push_back((neighbor_id.clone(), new_depth));
                }
            }
        }

        results
    }

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
            if let Some(ref pattern_edge_type) = edge_pattern.edge_type {
                if edge_info.edge_type != *pattern_edge_type {
                    continue;
                }
            }
            if !self.node_matches_type(graph, to_id, next_node_pattern)? {
                continue;
            }
            if let Some(existing_binding) = bindings.get(&next_node_pattern.variable) {
                if existing_binding != to_id {
                    continue;
                }
                self.extend_pattern_match_all(graph, pattern, bindings.clone(), edge_index + 1, all_matches)?;
            } else {
                let mut new_bindings = bindings.clone();
                new_bindings.insert(next_node_pattern.variable.clone(), to_id.to_string());
                self.extend_pattern_match_all(graph, pattern, new_bindings, edge_index + 1, all_matches)?;
            }
        }
        Ok(())
    }

    fn extend_pattern_match_all(
        &self,
        graph: &crate::values::Graph,
        pattern: &crate::ast::GraphPattern,
        bindings: std::collections::HashMap<String, String>,
        edge_index: usize,
        all_matches: &mut Vec<std::collections::HashMap<String, String>>,
    ) -> Result<()> {
        if edge_index >= pattern.edges.len() {
            all_matches.push(bindings);
            return Ok(());
        }

        let edge_pattern = &pattern.edges[edge_index];
        let next_node_pattern = &pattern.nodes[edge_index + 1];

        let from_id = bindings.get(&pattern.nodes[edge_index].variable)
            .ok_or_else(|| GraphoidError::runtime("Internal error: missing node binding in pattern match".to_string()))?;

        use crate::ast::EdgeLength;
        match &edge_pattern.length {
            EdgeLength::Variable { min, max } => {
                let reachable = self.find_variable_length_paths(
                    graph, from_id, edge_pattern.edge_type.as_deref(), *min, *max,
                );
                for (to_id, _path_length) in reachable {
                    if !self.node_matches_type(graph, &to_id, next_node_pattern)? {
                        continue;
                    }
                    if let Some(existing_binding) = bindings.get(&next_node_pattern.variable) {
                        if existing_binding != &to_id {
                            continue;
                        }
                        self.extend_pattern_match_all(graph, pattern, bindings.clone(), edge_index + 1, all_matches)?;
                    } else {
                        let mut new_bindings = bindings.clone();
                        new_bindings.insert(next_node_pattern.variable.clone(), to_id.clone());
                        self.extend_pattern_match_all(graph, pattern, new_bindings, edge_index + 1, all_matches)?;
                    }
                }
                return Ok(());
            }
            _ => {}
        }

        // Fixed-length (1 hop) path
        let from_node = graph.nodes.get(from_id.as_str())
            .ok_or_else(|| GraphoidError::runtime("Internal error: source node not found".to_string()))?;

        // Check direction
        use crate::ast::EdgeDirection;
        let check_forward = match &edge_pattern.direction {
            EdgeDirection::Directed => true,
            EdgeDirection::Bidirectional => true,
        };
        let check_backward = match &edge_pattern.direction {
            EdgeDirection::Directed => false,
            EdgeDirection::Bidirectional => true,
        };

        if check_forward {
            self.process_edges_for_pattern(graph, pattern, &from_node.neighbors, edge_pattern, next_node_pattern, &bindings, edge_index, all_matches)?;
        }

        if check_backward {
            self.process_edges_for_pattern(graph, pattern, &from_node.predecessors, edge_pattern, next_node_pattern, &bindings, edge_index, all_matches)?;
        }

        Ok(())
    }
}

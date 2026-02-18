//! AST to ExecutionGraph converter.
//!
//! Walks the existing AST (Expr/Stmt/Program) and builds an ExecutionGraph
//! with arena-allocated nodes and typed edges.

use std::collections::HashMap;
use crate::ast::*;
use crate::execution_graph::arena::{ArenaId, NodeRef};
use crate::execution_graph::node::*;
use crate::execution_graph::ExecutionGraph;

/// Converts AST structures into an ExecutionGraph.
pub struct AstToGraphConverter {
    graph: ExecutionGraph,
    /// The default arena for top-level code.
    default_arena: ArenaId,
}

impl AstToGraphConverter {
    pub fn new() -> Self {
        let mut graph = ExecutionGraph::new();
        let default_arena = graph.new_arena();
        AstToGraphConverter {
            graph,
            default_arena,
        }
    }

    /// Convert a Program into the execution graph, returning the root NodeRef.
    pub fn convert_program(&mut self, program: &Program) -> NodeRef {
        let root = self.add_node(self.default_arena, AstNodeType::Program, HashMap::new(), default_pos());
        for (i, stmt) in program.statements.iter().enumerate() {
            let stmt_ref = self.convert_stmt(stmt, self.default_arena);
            self.graph.add_edge(root, ExecEdgeType::Element(i as u32), stmt_ref);
        }
        self.graph.set_root(root);
        root
    }

    /// Convert an expression, using the default arena.
    pub fn convert_expr(&mut self, expr: &Expr) -> NodeRef {
        self.convert_expr_in(expr, self.default_arena)
    }

    /// Convert an expression into the specified arena.
    pub fn convert_expr_in(&mut self, expr: &Expr, arena: ArenaId) -> NodeRef {
        match expr {
            Expr::Literal { value, position } => self.convert_literal(value, position, arena),
            Expr::Variable { name, position } => {
                let mut props = HashMap::new();
                props.insert("name".to_string(), AstProperty::Str(name.clone()));
                self.add_node(arena, AstNodeType::Identifier, props, position.clone())
            }
            Expr::Binary { left, op, right, position } => {
                let mut props = HashMap::new();
                props.insert("operator".to_string(), AstProperty::BinaryOp(op.clone()));
                let node = self.add_node(arena, AstNodeType::BinaryExpr, props, position.clone());
                let left_ref = self.convert_expr_in(left, arena);
                let right_ref = self.convert_expr_in(right, arena);
                self.graph.add_edge(node, ExecEdgeType::Left, left_ref);
                self.graph.add_edge(node, ExecEdgeType::Right, right_ref);
                node
            }
            Expr::Unary { op, operand, position } => {
                let mut props = HashMap::new();
                props.insert("operator".to_string(), AstProperty::UnaryOp(op.clone()));
                let node = self.add_node(arena, AstNodeType::UnaryExpr, props, position.clone());
                let operand_ref = self.convert_expr_in(operand, arena);
                self.graph.add_edge(node, ExecEdgeType::Operand, operand_ref);
                node
            }
            Expr::Call { callee, args, position } => {
                let node = self.add_node(arena, AstNodeType::CallExpr, HashMap::new(), position.clone());
                let callee_ref = self.convert_expr_in(callee, arena);
                self.graph.add_edge(node, ExecEdgeType::Callee, callee_ref);
                for (i, arg) in args.iter().enumerate() {
                    let arg_ref = self.convert_argument(arg, arena, i as u32);
                    self.graph.add_edge(node, ExecEdgeType::Argument(i as u32), arg_ref);
                }
                node
            }
            Expr::MethodCall { object, method, args, position } => {
                let mut props = HashMap::new();
                props.insert("method".to_string(), AstProperty::Str(method.clone()));
                let node = self.add_node(arena, AstNodeType::MethodCallExpr, props, position.clone());
                let obj_ref = self.convert_expr_in(object, arena);
                self.graph.add_edge(node, ExecEdgeType::Object, obj_ref);
                for (i, arg) in args.iter().enumerate() {
                    let arg_ref = self.convert_argument(arg, arena, i as u32);
                    self.graph.add_edge(node, ExecEdgeType::Argument(i as u32), arg_ref);
                }
                node
            }
            Expr::SuperMethodCall { method, args, position } => {
                let mut props = HashMap::new();
                props.insert("method".to_string(), AstProperty::Str(method.clone()));
                let node = self.add_node(arena, AstNodeType::SuperMethodCallExpr, props, position.clone());
                for (i, arg) in args.iter().enumerate() {
                    let arg_ref = self.convert_argument(arg, arena, i as u32);
                    self.graph.add_edge(node, ExecEdgeType::Argument(i as u32), arg_ref);
                }
                node
            }
            Expr::PropertyAccess { object, property, position } => {
                let mut props = HashMap::new();
                props.insert("property".to_string(), AstProperty::Str(property.clone()));
                let node = self.add_node(arena, AstNodeType::PropertyAccessExpr, props, position.clone());
                let obj_ref = self.convert_expr_in(object, arena);
                self.graph.add_edge(node, ExecEdgeType::Object, obj_ref);
                node
            }
            Expr::Index { object, index, position } => {
                let node = self.add_node(arena, AstNodeType::IndexExpr, HashMap::new(), position.clone());
                let obj_ref = self.convert_expr_in(object, arena);
                let idx_ref = self.convert_expr_in(index, arena);
                self.graph.add_edge(node, ExecEdgeType::Object, obj_ref);
                self.graph.add_edge(node, ExecEdgeType::ValueEdge, idx_ref);
                node
            }
            Expr::Lambda { params, body, position } => {
                let mut props = HashMap::new();
                props.insert("param_count".to_string(), AstProperty::Int(params.len() as i64));
                for (i, p) in params.iter().enumerate() {
                    props.insert(format!("param_{}", i), AstProperty::Str(p.clone()));
                }
                let node = self.add_node(arena, AstNodeType::LambdaExpr, props, position.clone());
                // Lambda body gets its own arena for incremental re-parsing
                let body_arena = self.graph.new_arena();
                let body_ref = self.convert_expr_in(body, body_arena);
                self.graph.add_edge(node, ExecEdgeType::Body, body_ref);
                node
            }
            Expr::Block { statements, position } => {
                let node = self.add_node(arena, AstNodeType::BlockExpr, HashMap::new(), position.clone());
                for (i, stmt) in statements.iter().enumerate() {
                    let stmt_ref = self.convert_stmt(stmt, arena);
                    self.graph.add_edge(node, ExecEdgeType::Element(i as u32), stmt_ref);
                }
                node
            }
            Expr::List { elements, position } => {
                let node = self.add_node(arena, AstNodeType::ListExpr, HashMap::new(), position.clone());
                for (i, elem) in elements.iter().enumerate() {
                    let elem_ref = self.convert_expr_in(elem, arena);
                    self.graph.add_edge(node, ExecEdgeType::Element(i as u32), elem_ref);
                }
                node
            }
            Expr::Map { entries, position } => {
                let node = self.add_node(arena, AstNodeType::MapExpr, HashMap::new(), position.clone());
                for (i, (key, value)) in entries.iter().enumerate() {
                    // Each entry is a MapEntryNode with key property and ValueEdge
                    let mut entry_props = HashMap::new();
                    entry_props.insert("key".to_string(), AstProperty::Str(key.clone()));
                    let entry_node = self.add_node(arena, AstNodeType::MapEntryNode, entry_props, position.clone());
                    let val_ref = self.convert_expr_in(value, arena);
                    self.graph.add_edge(entry_node, ExecEdgeType::ValueEdge, val_ref);
                    self.graph.add_edge(node, ExecEdgeType::Element(i as u32), entry_node);
                }
                node
            }
            Expr::Graph { config, parent, position } => {
                let node = self.add_node(arena, AstNodeType::GraphExpr, HashMap::new(), position.clone());
                for (key, val_expr) in config {
                    let val_ref = self.convert_expr_in(val_expr, arena);
                    self.graph.add_edge(node, ExecEdgeType::Setting(key.clone()), val_ref);
                }
                if let Some(parent_expr) = parent {
                    let parent_ref = self.convert_expr_in(parent_expr, arena);
                    self.graph.add_edge(node, ExecEdgeType::Parent, parent_ref);
                }
                node
            }
            Expr::Conditional { condition, then_expr, else_expr, is_unless, position } => {
                let mut props = HashMap::new();
                props.insert("is_unless".to_string(), AstProperty::Bool(*is_unless));
                let node = self.add_node(arena, AstNodeType::ConditionalExpr, props, position.clone());
                let cond_ref = self.convert_expr_in(condition, arena);
                let then_ref = self.convert_expr_in(then_expr, arena);
                self.graph.add_edge(node, ExecEdgeType::Condition, cond_ref);
                self.graph.add_edge(node, ExecEdgeType::ThenBranch, then_ref);
                if let Some(else_e) = else_expr {
                    let else_ref = self.convert_expr_in(else_e, arena);
                    self.graph.add_edge(node, ExecEdgeType::ElseBranch, else_ref);
                }
                node
            }
            Expr::Raise { error, position } => {
                let node = self.add_node(arena, AstNodeType::RaiseExpr, HashMap::new(), position.clone());
                let err_ref = self.convert_expr_in(error, arena);
                self.graph.add_edge(node, ExecEdgeType::ValueEdge, err_ref);
                node
            }
            Expr::Match { value, arms, position } => {
                let node = self.add_node(arena, AstNodeType::MatchExpr, HashMap::new(), position.clone());
                let val_ref = self.convert_expr_in(value, arena);
                self.graph.add_edge(node, ExecEdgeType::MatchValue, val_ref);
                for (i, arm) in arms.iter().enumerate() {
                    let arm_ref = self.convert_match_arm(arm, arena);
                    self.graph.add_edge(node, ExecEdgeType::MatchArm(i as u32), arm_ref);
                }
                node
            }
            Expr::Instantiate { class_name, overrides, position } => {
                let node = self.add_node(arena, AstNodeType::InstantiateExpr, HashMap::new(), position.clone());
                let class_ref = self.convert_expr_in(class_name, arena);
                self.graph.add_edge(node, ExecEdgeType::Object, class_ref);
                for (i, (key, val_expr)) in overrides.iter().enumerate() {
                    let mut ov_props = HashMap::new();
                    ov_props.insert("key".to_string(), AstProperty::Str(key.clone()));
                    let ov_node = self.add_node(arena, AstNodeType::MapEntryNode, ov_props, position.clone());
                    let val_ref = self.convert_expr_in(val_expr, arena);
                    self.graph.add_edge(ov_node, ExecEdgeType::ValueEdge, val_ref);
                    self.graph.add_edge(node, ExecEdgeType::Override(i as u32), ov_node);
                }
                node
            }
        }
    }

    /// Convert a statement into the specified arena.
    pub fn convert_stmt(&mut self, stmt: &Stmt, arena: ArenaId) -> NodeRef {
        match stmt {
            Stmt::Expression { expr, position } => {
                let node = self.add_node(arena, AstNodeType::ExpressionStmt, HashMap::new(), position.clone());
                let expr_ref = self.convert_expr_in(expr, arena);
                self.graph.add_edge(node, ExecEdgeType::ValueEdge, expr_ref);
                node
            }
            Stmt::VariableDecl { name, type_annotation, value, is_private, position } => {
                let mut props = HashMap::new();
                props.insert("name".to_string(), AstProperty::Str(name.clone()));
                props.insert("is_private".to_string(), AstProperty::Bool(*is_private));
                if let Some(ta) = type_annotation {
                    props.insert("type_base".to_string(), AstProperty::Str(ta.base_type.clone()));
                    if let Some(c) = &ta.constraint {
                        props.insert("type_constraint".to_string(), AstProperty::Str(c.clone()));
                    }
                }
                let node = self.add_node(arena, AstNodeType::VarDeclStmt, props, position.clone());
                let val_ref = self.convert_expr_in(value, arena);
                self.graph.add_edge(node, ExecEdgeType::ValueEdge, val_ref);
                node
            }
            Stmt::Assignment { target, value, position } => {
                let mut props = HashMap::new();
                match target {
                    AssignmentTarget::Variable(name) => {
                        props.insert("target_type".to_string(), AstProperty::Str("variable".to_string()));
                        props.insert("target_name".to_string(), AstProperty::Str(name.clone()));
                    }
                    AssignmentTarget::Index { .. } => {
                        props.insert("target_type".to_string(), AstProperty::Str("index".to_string()));
                    }
                    AssignmentTarget::Property { property, .. } => {
                        props.insert("target_type".to_string(), AstProperty::Str("property".to_string()));
                        props.insert("target_name".to_string(), AstProperty::Str(property.clone()));
                    }
                }
                let node = self.add_node(arena, AstNodeType::AssignStmt, props, position.clone());
                let val_ref = self.convert_expr_in(value, arena);
                self.graph.add_edge(node, ExecEdgeType::ValueEdge, val_ref);
                // For index/property targets, add Object and Target edges
                match target {
                    AssignmentTarget::Index { object, index } => {
                        let obj_ref = self.convert_expr_in(object, arena);
                        let idx_ref = self.convert_expr_in(index, arena);
                        self.graph.add_edge(node, ExecEdgeType::Object, obj_ref);
                        self.graph.add_edge(node, ExecEdgeType::Target, idx_ref);
                    }
                    AssignmentTarget::Property { object, .. } => {
                        let obj_ref = self.convert_expr_in(object, arena);
                        self.graph.add_edge(node, ExecEdgeType::Object, obj_ref);
                    }
                    _ => {}
                }
                node
            }
            Stmt::FunctionDecl { name, receiver, params, body, pattern_clauses, is_private, is_setter, is_static, guard, position } => {
                let mut props = HashMap::new();
                props.insert("name".to_string(), AstProperty::Str(name.clone()));
                props.insert("param_count".to_string(), AstProperty::Int(params.len() as i64));
                props.insert("is_private".to_string(), AstProperty::Bool(*is_private));
                props.insert("is_setter".to_string(), AstProperty::Bool(*is_setter));
                props.insert("is_static".to_string(), AstProperty::Bool(*is_static));
                if let Some(recv) = receiver {
                    props.insert("receiver".to_string(), AstProperty::Str(recv.clone()));
                }
                let node = self.add_node(arena, AstNodeType::FuncDeclStmt, props, position.clone());

                // Parameters as child nodes
                for (i, param) in params.iter().enumerate() {
                    let param_ref = self.convert_parameter(param, arena);
                    self.graph.add_edge(node, ExecEdgeType::Parameter(i as u32), param_ref);
                }

                // Body in its own arena
                let body_arena = self.graph.new_arena();
                let body_node = self.add_node(body_arena, AstNodeType::BlockExpr, HashMap::new(), position.clone());
                for (i, s) in body.iter().enumerate() {
                    let s_ref = self.convert_stmt(s, body_arena);
                    self.graph.add_edge(body_node, ExecEdgeType::Element(i as u32), s_ref);
                }
                self.graph.add_edge(node, ExecEdgeType::Body, body_node);

                // Pattern clauses
                if let Some(clauses) = pattern_clauses {
                    for (i, clause) in clauses.iter().enumerate() {
                        let clause_ref = self.convert_pattern_clause(clause, arena);
                        self.graph.add_edge(node, ExecEdgeType::PatternClause(i as u32), clause_ref);
                    }
                }

                // Guard
                if let Some(guard_expr) = guard {
                    let guard_ref = self.convert_expr_in(guard_expr, arena);
                    self.graph.add_edge(node, ExecEdgeType::Guard, guard_ref);
                }

                node
            }
            Stmt::If { condition, then_branch, else_branch, position } => {
                let node = self.add_node(arena, AstNodeType::IfStmt, HashMap::new(), position.clone());
                let cond_ref = self.convert_expr_in(condition, arena);
                self.graph.add_edge(node, ExecEdgeType::Condition, cond_ref);

                // Then branch as block
                let then_node = self.convert_stmt_block(then_branch, arena, position);
                self.graph.add_edge(node, ExecEdgeType::ThenBranch, then_node);

                // Else branch
                if let Some(else_stmts) = else_branch {
                    let else_node = self.convert_stmt_block(else_stmts, arena, position);
                    self.graph.add_edge(node, ExecEdgeType::ElseBranch, else_node);
                }
                node
            }
            Stmt::While { condition, body, position } => {
                let node = self.add_node(arena, AstNodeType::WhileStmt, HashMap::new(), position.clone());
                let cond_ref = self.convert_expr_in(condition, arena);
                self.graph.add_edge(node, ExecEdgeType::Condition, cond_ref);
                let body_node = self.convert_stmt_block(body, arena, position);
                self.graph.add_edge(node, ExecEdgeType::Body, body_node);
                node
            }
            Stmt::For { variable, iterable, body, position } => {
                let mut props = HashMap::new();
                props.insert("variable".to_string(), AstProperty::Str(variable.clone()));
                let node = self.add_node(arena, AstNodeType::ForStmt, props, position.clone());
                let iter_ref = self.convert_expr_in(iterable, arena);
                self.graph.add_edge(node, ExecEdgeType::Iterable, iter_ref);
                let body_node = self.convert_stmt_block(body, arena, position);
                self.graph.add_edge(node, ExecEdgeType::Body, body_node);
                node
            }
            Stmt::Return { value, position } => {
                let node = self.add_node(arena, AstNodeType::ReturnStmt, HashMap::new(), position.clone());
                if let Some(val) = value {
                    let val_ref = self.convert_expr_in(val, arena);
                    self.graph.add_edge(node, ExecEdgeType::ValueEdge, val_ref);
                }
                node
            }
            Stmt::Break { position } => {
                self.add_node(arena, AstNodeType::BreakStmt, HashMap::new(), position.clone())
            }
            Stmt::Continue { position } => {
                self.add_node(arena, AstNodeType::ContinueStmt, HashMap::new(), position.clone())
            }
            Stmt::Import { module, alias, selections, position } => {
                let mut props = HashMap::new();
                props.insert("module".to_string(), AstProperty::Str(module.clone()));
                if let Some(a) = alias {
                    props.insert("alias".to_string(), AstProperty::Str(a.clone()));
                }
                if let Some(ref items) = selections {
                    props.insert("selection_count".to_string(), AstProperty::Int(items.len() as i64));
                }
                let node = self.add_node(arena, AstNodeType::ImportStmt, props, position.clone());
                // Phase 17: Store each selective import item as a child node
                if let Some(items) = selections {
                    for (i, item) in items.iter().enumerate() {
                        let mut item_props = HashMap::new();
                        item_props.insert("name".to_string(), AstProperty::Str(item.name.clone()));
                        if let Some(ref a) = item.alias {
                            item_props.insert("alias".to_string(), AstProperty::Str(a.clone()));
                        }
                        let item_node = self.add_node(arena, AstNodeType::ImportItemNode, item_props, position.clone());
                        self.graph.add_edge(node, ExecEdgeType::Element(i as u32), item_node);
                    }
                }
                node
            }
            Stmt::ModuleDecl { name, alias, position } => {
                let mut props = HashMap::new();
                props.insert("name".to_string(), AstProperty::Str(name.clone()));
                if let Some(a) = alias {
                    props.insert("alias".to_string(), AstProperty::Str(a.clone()));
                }
                self.add_node(arena, AstNodeType::ModuleDeclStmt, props, position.clone())
            }
            Stmt::Load { path, position } => {
                let node = self.add_node(arena, AstNodeType::LoadStmt, HashMap::new(), position.clone());
                let path_ref = self.convert_expr_in(path, arena);
                self.graph.add_edge(node, ExecEdgeType::ValueEdge, path_ref);
                node
            }
            Stmt::Configure { settings, body, position } => {
                let node = self.add_node(arena, AstNodeType::ConfigureStmt, HashMap::new(), position.clone());
                for (key, val_expr) in settings {
                    let val_ref = self.convert_expr_in(val_expr, arena);
                    self.graph.add_edge(node, ExecEdgeType::Setting(key.clone()), val_ref);
                }
                if let Some(body_stmts) = body {
                    let body_node = self.convert_stmt_block(body_stmts, arena, position);
                    self.graph.add_edge(node, ExecEdgeType::Body, body_node);
                }
                node
            }
            Stmt::Precision { places, body, position } => {
                let mut props = HashMap::new();
                if let Some(p) = places {
                    props.insert("places".to_string(), AstProperty::Int(*p as i64));
                }
                let node = self.add_node(arena, AstNodeType::PrecisionStmt, props, position.clone());
                let body_node = self.convert_stmt_block(body, arena, position);
                self.graph.add_edge(node, ExecEdgeType::Body, body_node);
                node
            }
            Stmt::Try { body, catch_clauses, finally_block, position } => {
                let node = self.add_node(arena, AstNodeType::TryStmt, HashMap::new(), position.clone());
                let body_node = self.convert_stmt_block(body, arena, position);
                self.graph.add_edge(node, ExecEdgeType::Body, body_node);

                for (i, clause) in catch_clauses.iter().enumerate() {
                    let catch_ref = self.convert_catch_clause(clause, arena);
                    self.graph.add_edge(node, ExecEdgeType::CatchHandler(i as u32), catch_ref);
                }

                if let Some(finally) = finally_block {
                    let finally_node = self.convert_stmt_block(finally, arena, position);
                    self.graph.add_edge(node, ExecEdgeType::FinallyBlock, finally_node);
                }
                node
            }
            Stmt::GraphDecl { name, graph_type, parent, properties, methods, rules, config, position } => {
                let mut props = HashMap::new();
                props.insert("name".to_string(), AstProperty::Str(name.clone()));
                if let Some(gt) = graph_type {
                    props.insert("graph_type".to_string(), AstProperty::Str(gt.clone()));
                }
                let node = self.add_node(arena, AstNodeType::GraphDeclStmt, props, position.clone());

                // Parent
                if let Some(parent_expr) = parent {
                    let parent_ref = self.convert_expr_in(parent_expr, arena);
                    self.graph.add_edge(node, ExecEdgeType::Parent, parent_ref);
                }

                // Properties
                for (i, prop) in properties.iter().enumerate() {
                    let prop_ref = self.convert_graph_property(prop, arena);
                    self.graph.add_edge(node, ExecEdgeType::Property(i as u32), prop_ref);
                }

                // Methods
                for (i, method) in methods.iter().enumerate() {
                    let method_ref = self.convert_graph_method(method, arena);
                    self.graph.add_edge(node, ExecEdgeType::GraphMethod(i as u32), method_ref);
                }

                // Rules
                for (i, rule) in rules.iter().enumerate() {
                    let rule_ref = self.convert_graph_rule(rule, arena);
                    self.graph.add_edge(node, ExecEdgeType::Rule(i as u32), rule_ref);
                }

                // Config settings (readable, writable, etc.)
                for (key, values) in config {
                    let mut cfg_props = HashMap::new();
                    cfg_props.insert("key".to_string(), AstProperty::Str(key.clone()));
                    for (j, v) in values.iter().enumerate() {
                        cfg_props.insert(format!("value_{}", j), AstProperty::Str(v.clone()));
                    }
                    cfg_props.insert("count".to_string(), AstProperty::Int(values.len() as i64));
                    let cfg_node = self.add_node(arena, AstNodeType::MapEntryNode, cfg_props, position.clone());
                    self.graph.add_edge(node, ExecEdgeType::Setting(key.clone()), cfg_node);
                }

                node
            }
            Stmt::PrivBlock { body, position } => {
                let node = self.add_node(arena, AstNodeType::PrivBlockStmt, HashMap::new(), position.clone());
                for (i, s) in body.iter().enumerate() {
                    let s_ref = self.convert_stmt(s, arena);
                    self.graph.add_edge(node, ExecEdgeType::Element(i as u32), s_ref);
                }
                node
            }
        }
    }

    /// Convert a list of statements into a BlockExpr node.
    fn convert_stmt_block(&mut self, stmts: &[Stmt], arena: ArenaId, position: &crate::error::SourcePosition) -> NodeRef {
        let block = self.add_node(arena, AstNodeType::BlockExpr, HashMap::new(), position.clone());
        for (i, s) in stmts.iter().enumerate() {
            let s_ref = self.convert_stmt(s, arena);
            self.graph.add_edge(block, ExecEdgeType::Element(i as u32), s_ref);
        }
        block
    }

    /// Convert a catch clause.
    fn convert_catch_clause(&mut self, clause: &CatchClause, arena: ArenaId) -> NodeRef {
        let mut props = HashMap::new();
        if let Some(et) = &clause.error_type {
            props.insert("error_type".to_string(), AstProperty::Str(et.clone()));
        }
        if let Some(var) = &clause.variable {
            props.insert("variable".to_string(), AstProperty::Str(var.clone()));
        }
        let node = self.add_node(arena, AstNodeType::CatchClauseNode, props, clause.position.clone());
        let body = self.convert_stmt_block(&clause.body, arena, &clause.position);
        self.graph.add_edge(node, ExecEdgeType::Body, body);
        node
    }

    /// Convert a function parameter.
    fn convert_parameter(&mut self, param: &Parameter, arena: ArenaId) -> NodeRef {
        let mut props = HashMap::new();
        props.insert("name".to_string(), AstProperty::Str(param.name.clone()));
        props.insert("is_variadic".to_string(), AstProperty::Bool(param.is_variadic));
        let node = self.add_node(arena, AstNodeType::Identifier, props, default_pos());
        if let Some(default) = &param.default_value {
            let default_ref = self.convert_expr_in(default, arena);
            self.graph.add_edge(node, ExecEdgeType::DefaultValue, default_ref);
        }
        node
    }

    /// Convert a pattern clause (for pattern-matching functions).
    fn convert_pattern_clause(&mut self, clause: &PatternClause, arena: ArenaId) -> NodeRef {
        let node = self.add_node(arena, AstNodeType::PatternClauseNode, HashMap::new(), clause.position.clone());
        // Pattern
        let pattern_ref = self.convert_function_pattern(&clause.pattern, arena);
        self.graph.add_edge(node, ExecEdgeType::ArmPattern, pattern_ref);
        // Body
        let body_ref = self.convert_expr_in(&clause.body, arena);
        self.graph.add_edge(node, ExecEdgeType::ArmBody, body_ref);
        // Guard
        if let Some(guard) = &clause.guard {
            let guard_ref = self.convert_expr_in(guard, arena);
            self.graph.add_edge(node, ExecEdgeType::Guard, guard_ref);
        }
        node
    }

    /// Convert a function pattern.
    fn convert_function_pattern(&mut self, pattern: &Pattern, arena: ArenaId) -> NodeRef {
        let mut props = HashMap::new();
        match pattern {
            Pattern::Literal { value, .. } => {
                props.insert("pattern_type".to_string(), AstProperty::Str("literal".to_string()));
                match value {
                    LiteralValue::Number(n) => { props.insert("value".to_string(), AstProperty::Num(*n)); }
                    LiteralValue::String(s) => { props.insert("value".to_string(), AstProperty::Str(s.clone())); }
                    LiteralValue::Boolean(b) => { props.insert("value".to_string(), AstProperty::Bool(*b)); }
                    LiteralValue::None => { props.insert("value".to_string(), AstProperty::None); }
                    LiteralValue::Symbol(s) => { props.insert("value".to_string(), AstProperty::Str(s.clone())); }
                }
            }
            Pattern::Variable { name, .. } => {
                props.insert("pattern_type".to_string(), AstProperty::Str("variable".to_string()));
                props.insert("name".to_string(), AstProperty::Str(name.clone()));
            }
            Pattern::Wildcard { .. } => {
                props.insert("pattern_type".to_string(), AstProperty::Str("wildcard".to_string()));
            }
        }
        let pos = match pattern {
            Pattern::Literal { position, .. } => position.clone(),
            Pattern::Variable { position, .. } => position.clone(),
            Pattern::Wildcard { position } => position.clone(),
        };
        self.add_node(arena, AstNodeType::MatchPatternNode, props, pos)
    }

    /// Convert a graph property declaration.
    fn convert_graph_property(&mut self, prop: &GraphProperty, arena: ArenaId) -> NodeRef {
        let mut props = HashMap::new();
        props.insert("name".to_string(), AstProperty::Str(prop.name.clone()));
        let node = self.add_node(arena, AstNodeType::GraphPropertyNode, props, prop.position.clone());
        let val_ref = self.convert_expr_in(&prop.value, arena);
        self.graph.add_edge(node, ExecEdgeType::ValueEdge, val_ref);
        node
    }

    /// Convert a graph method declaration.
    fn convert_graph_method(&mut self, method: &GraphMethod, arena: ArenaId) -> NodeRef {
        let mut props = HashMap::new();
        props.insert("name".to_string(), AstProperty::Str(method.name.clone()));
        props.insert("is_static".to_string(), AstProperty::Bool(method.is_static));
        props.insert("is_setter".to_string(), AstProperty::Bool(method.is_setter));
        props.insert("is_private".to_string(), AstProperty::Bool(method.is_private));
        props.insert("param_count".to_string(), AstProperty::Int(method.params.len() as i64));
        let node = self.add_node(arena, AstNodeType::GraphMethodNode, props, method.position.clone());

        // Parameters
        for (i, param) in method.params.iter().enumerate() {
            let param_ref = self.convert_parameter(param, arena);
            self.graph.add_edge(node, ExecEdgeType::Parameter(i as u32), param_ref);
        }

        // Body in its own arena
        let body_arena = self.graph.new_arena();
        let body_node = self.add_node(body_arena, AstNodeType::BlockExpr, HashMap::new(), method.position.clone());
        for (i, s) in method.body.iter().enumerate() {
            let s_ref = self.convert_stmt(s, body_arena);
            self.graph.add_edge(body_node, ExecEdgeType::Element(i as u32), s_ref);
        }
        self.graph.add_edge(node, ExecEdgeType::Body, body_node);

        // Guard
        if let Some(guard_expr) = &method.guard {
            let guard_ref = self.convert_expr_in(guard_expr, arena);
            self.graph.add_edge(node, ExecEdgeType::Guard, guard_ref);
        }

        node
    }

    /// Convert a graph rule declaration.
    fn convert_graph_rule(&mut self, rule: &GraphRule, arena: ArenaId) -> NodeRef {
        let mut props = HashMap::new();
        props.insert("name".to_string(), AstProperty::Str(rule.name.clone()));
        let node = self.add_node(arena, AstNodeType::GraphRuleNode, props, rule.position.clone());
        if let Some(param) = &rule.param {
            let param_ref = self.convert_expr_in(param, arena);
            self.graph.add_edge(node, ExecEdgeType::ValueEdge, param_ref);
        }
        node
    }

    /// Convert a function call argument into a graph node.
    fn convert_argument(&mut self, arg: &Argument, arena: ArenaId, _index: u32) -> NodeRef {
        match arg {
            Argument::Positional { expr, mutable } => {
                let expr_ref = self.convert_expr_in(expr, arena);
                if *mutable {
                    // Wrap in an argument node with mutable flag
                    let mut props = HashMap::new();
                    props.insert("mutable".to_string(), AstProperty::Bool(true));
                    let wrapper = self.add_node(arena, AstNodeType::ExpressionStmt, props, expr.position().clone());
                    self.graph.add_edge(wrapper, ExecEdgeType::ValueEdge, expr_ref);
                    wrapper
                } else {
                    expr_ref
                }
            }
            Argument::Named { name, value, mutable } => {
                let mut props = HashMap::new();
                props.insert("arg_name".to_string(), AstProperty::Str(name.clone()));
                if *mutable {
                    props.insert("mutable".to_string(), AstProperty::Bool(true));
                }
                let wrapper = self.add_node(arena, AstNodeType::ExpressionStmt, props, value.position().clone());
                let val_ref = self.convert_expr_in(value, arena);
                self.graph.add_edge(wrapper, ExecEdgeType::ValueEdge, val_ref);
                wrapper
            }
        }
    }

    /// Convert a match arm.
    fn convert_match_arm(&mut self, arm: &MatchArm, arena: ArenaId) -> NodeRef {
        let node = self.add_node(arena, AstNodeType::MatchArmNode, HashMap::new(), arm.position.clone());

        // Pattern
        let pattern_ref = self.convert_match_pattern(&arm.pattern, arena);
        self.graph.add_edge(node, ExecEdgeType::ArmPattern, pattern_ref);

        // Body
        let body_ref = self.convert_expr_in(&arm.body, arena);
        self.graph.add_edge(node, ExecEdgeType::ArmBody, body_ref);

        node
    }

    /// Convert a match pattern.
    fn convert_match_pattern(&mut self, pattern: &MatchPattern, arena: ArenaId) -> NodeRef {
        let mut props = HashMap::new();
        match pattern {
            MatchPattern::Literal(lit) => {
                props.insert("pattern_type".to_string(), AstProperty::Str("literal".to_string()));
                match lit {
                    LiteralValue::Number(n) => { props.insert("value".to_string(), AstProperty::Num(*n)); }
                    LiteralValue::String(s) => { props.insert("value".to_string(), AstProperty::Str(s.clone())); }
                    LiteralValue::Boolean(b) => { props.insert("value".to_string(), AstProperty::Bool(*b)); }
                    LiteralValue::None => { props.insert("value".to_string(), AstProperty::None); }
                    LiteralValue::Symbol(s) => { props.insert("value".to_string(), AstProperty::Str(s.clone())); }
                }
            }
            MatchPattern::Variable(name) => {
                props.insert("pattern_type".to_string(), AstProperty::Str("variable".to_string()));
                props.insert("name".to_string(), AstProperty::Str(name.clone()));
            }
            MatchPattern::Wildcard => {
                props.insert("pattern_type".to_string(), AstProperty::Str("wildcard".to_string()));
            }
            MatchPattern::List { elements, rest_name } => {
                props.insert("pattern_type".to_string(), AstProperty::Str("list".to_string()));
                props.insert("element_count".to_string(), AstProperty::Int(elements.len() as i64));
                if let Some(rest) = rest_name {
                    props.insert("rest_name".to_string(), AstProperty::Str(rest.clone()));
                }
                let node = self.add_node(arena, AstNodeType::MatchPatternNode, props, default_pos());
                // Add Element edges for sub-patterns
                for (i, elem) in elements.iter().enumerate() {
                    let elem_ref = self.convert_match_pattern(elem, arena);
                    self.graph.add_edge(node, ExecEdgeType::Element(i as u32), elem_ref);
                }
                return node;
            }
        }
        self.add_node(arena, AstNodeType::MatchPatternNode, props, default_pos())
    }

    /// Convert a literal value to the appropriate node type.
    fn convert_literal(&mut self, value: &LiteralValue, position: &crate::error::SourcePosition, arena: ArenaId) -> NodeRef {
        match value {
            LiteralValue::Number(n) => {
                let mut props = HashMap::new();
                props.insert("value".to_string(), AstProperty::Num(*n));
                self.add_node(arena, AstNodeType::NumberLit, props, position.clone())
            }
            LiteralValue::String(s) => {
                let mut props = HashMap::new();
                props.insert("value".to_string(), AstProperty::Str(s.clone()));
                self.add_node(arena, AstNodeType::StringLit, props, position.clone())
            }
            LiteralValue::Boolean(b) => {
                let mut props = HashMap::new();
                props.insert("value".to_string(), AstProperty::Bool(*b));
                self.add_node(arena, AstNodeType::BoolLit, props, position.clone())
            }
            LiteralValue::None => {
                self.add_node(arena, AstNodeType::NoneLit, HashMap::new(), position.clone())
            }
            LiteralValue::Symbol(s) => {
                let mut props = HashMap::new();
                props.insert("value".to_string(), AstProperty::Str(s.clone()));
                self.add_node(arena, AstNodeType::SymbolLit, props, position.clone())
            }
        }
    }

    /// Helper to add a node to the graph.
    fn add_node(&mut self, arena: ArenaId, node_type: AstNodeType, properties: HashMap<String, AstProperty>, position: crate::error::SourcePosition) -> NodeRef {
        self.graph.add_node(arena, AstGraphNode {
            node_type,
            properties,
            position,
        })
    }

    /// Consume the converter and return the built graph.
    pub fn into_graph(self) -> ExecutionGraph {
        self.graph
    }
}

fn default_pos() -> crate::error::SourcePosition {
    crate::error::SourcePosition { line: 0, column: 0, file: None }
}

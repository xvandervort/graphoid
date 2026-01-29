//! AST graph node types, edge types, and properties for the execution graph.

use std::collections::HashMap;
use crate::ast::{BinaryOp, UnaryOp};
use crate::error::SourcePosition;

/// A node in the execution graph representing an AST construct.
#[derive(Debug, Clone)]
pub struct AstGraphNode {
    pub node_type: AstNodeType,
    pub properties: HashMap<String, AstProperty>,
    pub position: SourcePosition,
}

/// The type of an AST node in the execution graph.
/// One variant per AST construct (expressions + statements + auxiliary).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstNodeType {
    // Literal expressions
    NumberLit,
    StringLit,
    BoolLit,
    NoneLit,
    SymbolLit,

    // Identifier
    Identifier,

    // Expressions
    BinaryExpr,
    UnaryExpr,
    CallExpr,
    MethodCallExpr,
    SuperMethodCallExpr,
    PropertyAccessExpr,
    IndexExpr,
    LambdaExpr,
    BlockExpr,
    ListExpr,
    MapExpr,
    GraphExpr,
    ConditionalExpr,
    RaiseExpr,
    MatchExpr,
    InstantiateExpr,

    // Statements
    VarDeclStmt,
    AssignStmt,
    FuncDeclStmt,
    IfStmt,
    WhileStmt,
    ForStmt,
    ReturnStmt,
    BreakStmt,
    ContinueStmt,
    ImportStmt,
    ModuleDeclStmt,
    LoadStmt,
    ConfigureStmt,
    PrecisionStmt,
    TryStmt,
    GraphDeclStmt,
    ExpressionStmt,

    // Structural
    Program,

    // Auxiliary nodes (children of compound constructs)
    MatchArmNode,
    MatchPatternNode,
    PatternClauseNode,
    CatchClauseNode,
    MapEntryNode,
    GraphPropertyNode,
    GraphMethodNode,
    GraphRuleNode,
}

/// Edge types connecting nodes in the execution graph.
/// These represent structural relationships from the AST.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExecEdgeType {
    // Binary expression operands
    Left,
    Right,

    // Unary expression operand
    Operand,

    // Call expression
    Callee,
    Argument(u32),

    // Control flow
    Condition,
    ThenBranch,
    ElseBranch,

    // Function/loop body
    Body,

    // Assignment
    Target,
    ValueEdge,

    // Ordered children (blocks, lists, programs)
    Element(u32),

    // Method call / property access
    Object,

    // Function parameters
    Parameter(u32),

    // Try/catch
    CatchHandler(u32),
    FinallyBlock,

    // Inheritance
    Parent,

    // Match expression
    MatchValue,
    MatchArm(u32),
    ArmBody,
    ArmPattern,

    // For loop
    Iterable,

    // GraphDecl children
    Property(u32),
    Rule(u32),
    GraphMethod(u32),

    // Function guard
    Guard,

    // Instantiation overrides
    Override(u32),

    // Configure settings
    Setting(String),

    // Default value (for parameters, etc.)
    DefaultValue,

    // Pattern clause
    PatternClause(u32),
}

impl ExecEdgeType {
    /// Returns the ordering key for indexed edge types (Argument, Element, etc.).
    /// Returns None for non-indexed types.
    pub fn index(&self) -> Option<u32> {
        match self {
            ExecEdgeType::Argument(i) => Some(*i),
            ExecEdgeType::Element(i) => Some(*i),
            ExecEdgeType::Parameter(i) => Some(*i),
            ExecEdgeType::CatchHandler(i) => Some(*i),
            ExecEdgeType::MatchArm(i) => Some(*i),
            ExecEdgeType::Property(i) => Some(*i),
            ExecEdgeType::Rule(i) => Some(*i),
            ExecEdgeType::GraphMethod(i) => Some(*i),
            ExecEdgeType::Override(i) => Some(*i),
            ExecEdgeType::PatternClause(i) => Some(*i),
            _ => None,
        }
    }

    /// Returns the prefix name for grouping (e.g., "Argument" for Argument(0)).
    pub fn prefix(&self) -> &str {
        match self {
            ExecEdgeType::Left => "Left",
            ExecEdgeType::Right => "Right",
            ExecEdgeType::Operand => "Operand",
            ExecEdgeType::Callee => "Callee",
            ExecEdgeType::Argument(_) => "Argument",
            ExecEdgeType::Condition => "Condition",
            ExecEdgeType::ThenBranch => "ThenBranch",
            ExecEdgeType::ElseBranch => "ElseBranch",
            ExecEdgeType::Body => "Body",
            ExecEdgeType::Target => "Target",
            ExecEdgeType::ValueEdge => "ValueEdge",
            ExecEdgeType::Element(_) => "Element",
            ExecEdgeType::Object => "Object",
            ExecEdgeType::Parameter(_) => "Parameter",
            ExecEdgeType::CatchHandler(_) => "CatchHandler",
            ExecEdgeType::FinallyBlock => "FinallyBlock",
            ExecEdgeType::Parent => "Parent",
            ExecEdgeType::MatchValue => "MatchValue",
            ExecEdgeType::MatchArm(_) => "MatchArm",
            ExecEdgeType::ArmBody => "ArmBody",
            ExecEdgeType::ArmPattern => "ArmPattern",
            ExecEdgeType::Iterable => "Iterable",
            ExecEdgeType::Property(_) => "Property",
            ExecEdgeType::Rule(_) => "Rule",
            ExecEdgeType::GraphMethod(_) => "GraphMethod",
            ExecEdgeType::Guard => "Guard",
            ExecEdgeType::Override(_) => "Override",
            ExecEdgeType::Setting(_) => "Setting",
            ExecEdgeType::DefaultValue => "DefaultValue",
            ExecEdgeType::PatternClause(_) => "PatternClause",
        }
    }
}

/// Property values stored on AST graph nodes.
/// A small enum for AST-level metadata, avoiding the full Value type.
#[derive(Debug, Clone, PartialEq)]
pub enum AstProperty {
    Str(String),
    Num(f64),
    Bool(bool),
    Int(i64),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    None,
}

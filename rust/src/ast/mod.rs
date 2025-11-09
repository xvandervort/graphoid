//! Abstract Syntax Tree node types

use crate::error::SourcePosition;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    VariableDecl {
        name: String,
        type_annotation: Option<TypeAnnotation>,
        value: Expr,
        is_private: bool,  // Phase 10: priv keyword support
        position: SourcePosition,
    },
    Assignment {
        target: AssignmentTarget,
        value: Expr,
        position: SourcePosition,
    },
    FunctionDecl {
        name: String,
        params: Vec<Parameter>,
        body: Vec<Stmt>,
        pattern_clauses: Option<Vec<PatternClause>>,  // Phase 7: Pattern matching
        is_private: bool,  // Phase 10: priv keyword support
        position: SourcePosition,
    },
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
        position: SourcePosition,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
        position: SourcePosition,
    },
    For {
        variable: String,
        iterable: Expr,
        body: Vec<Stmt>,
        position: SourcePosition,
    },
    Return {
        value: Option<Expr>,
        position: SourcePosition,
    },
    Break {
        position: SourcePosition,
    },
    Continue {
        position: SourcePosition,
    },
    Import {
        module: String,
        alias: Option<String>,
        position: SourcePosition,
    },
    ModuleDecl {
        name: String,
        alias: Option<String>,
        position: SourcePosition,
    },
    Load {
        path: String,
        position: SourcePosition,
    },
    Configure {
        settings: std::collections::HashMap<String, Expr>,
        body: Option<Vec<Stmt>>,
        position: SourcePosition,
    },
    Precision {
        places: Option<usize>,  // None for :int mode, Some(n) for n decimal places
        body: Vec<Stmt>,
        position: SourcePosition,
    },
    Try {
        body: Vec<Stmt>,
        catch_clauses: Vec<CatchClause>,
        finally_block: Option<Vec<Stmt>>,
        position: SourcePosition,
    },
    Expression {
        expr: Expr,
        position: SourcePosition,
    },
}

/// A catch clause in a try/catch statement
#[derive(Debug, Clone, PartialEq)]
pub struct CatchClause {
    pub error_type: Option<String>,  // None = catch all errors
    pub variable: Option<String>,    // None = no binding
    pub body: Vec<Stmt>,
    pub position: SourcePosition,
}

/// Function call argument - can be positional or named
#[derive(Debug, Clone, PartialEq)]
pub enum Argument {
    /// Positional argument: just the expression
    Positional(Expr),
    /// Named argument: name and expression (e.g., name: "Alice")
    Named {
        name: String,
        value: Expr,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal {
        value: LiteralValue,
        position: SourcePosition,
    },
    Variable {
        name: String,
        position: SourcePosition,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        position: SourcePosition,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
        position: SourcePosition,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Argument>,
        position: SourcePosition,
    },
    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Argument>,
        position: SourcePosition,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
        position: SourcePosition,
    },
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
        position: SourcePosition,
    },
    Block {
        statements: Vec<Stmt>,
        position: SourcePosition,
    },
    List {
        elements: Vec<Expr>,
        position: SourcePosition,
    },
    Map {
        entries: Vec<(String, Expr)>,
        position: SourcePosition,
    },
    Graph {
        config: Vec<(String, Expr)>,
        position: SourcePosition,
    },
    // Tree variant removed in Step 7 - tree{} now desugars to graph{}.with_ruleset(:tree)
    Conditional {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Option<Box<Expr>>, // None for suffix if/unless
        is_unless: bool,               // true for unless, false for if
        position: SourcePosition,
    },
    Raise {
        error: Box<Expr>,  // Error value/message to raise
        position: SourcePosition,
    },
    Match {
        value: Box<Expr>,
        arms: Vec<MatchArm>,
        position: SourcePosition,
    },
}

/// A single arm in a match expression
#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: MatchPattern,
    pub body: Expr,
    pub position: SourcePosition,
}

/// Pattern for match expressions
#[derive(Debug, Clone, PartialEq)]
pub enum MatchPattern {
    /// Literal value pattern (42, "hello", true)
    Literal(LiteralValue),
    /// Variable binding pattern (x, name)
    Variable(String),
    /// Wildcard pattern (_)
    Wildcard,
    /// List pattern ([x, y, z] or [x, ...rest])
    List {
        elements: Vec<MatchPattern>,
        rest_name: Option<String>,  // Some("rest") for [x, ...rest], None for fixed-length
    },
}

impl Expr {
    pub fn position(&self) -> &SourcePosition {
        match self {
            Expr::Literal { position, .. } => position,
            Expr::Variable { position, .. } => position,
            Expr::Binary { position, .. } => position,
            Expr::Unary { position, .. } => position,
            Expr::Call { position, .. } => position,
            Expr::MethodCall { position, .. } => position,
            Expr::Index { position, .. } => position,
            Expr::Lambda { position, .. } => position,
            Expr::Block { position, .. } => position,
            Expr::List { position, .. } => position,
            Expr::Map { position, .. } => position,
            Expr::Graph { position, .. } => position,
            Expr::Conditional { position, .. } => position,
            Expr::Raise { position, .. } => position,
            Expr::Match { position, .. } => position,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    None,
    Symbol(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    IntDiv,      // //
    Modulo,
    Power,

    // Comparison
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Logical
    And,
    Or,

    // Regex
    RegexMatch,
    RegexNoMatch,

    // Element-wise
    DotAdd,
    DotSubtract,
    DotMultiply,
    DotDivide,
    DotIntDiv,
    DotModulo,
    DotPower,
    DotEqual,
    DotNotEqual,
    DotLess,
    DotLessEqual,
    DotGreater,
    DotGreaterEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub default_value: Option<Expr>,
    pub is_variadic: bool,  // true if parameter is ...name
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAnnotation {
    pub base_type: String,
    pub constraint: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentTarget {
    Variable(String),
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
}

/// Pattern matching clause (Phase 7)
/// Represents a pattern clause in a function: |pattern| => result
#[derive(Debug, Clone, PartialEq)]
pub struct PatternClause {
    pub pattern: Pattern,
    pub guard: Option<Expr>,  // Future: if conditions
    pub body: Expr,
    pub position: SourcePosition,
}

/// Pattern types for pattern matching (Phase 7)
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Literal {
        value: LiteralValue,
        position: SourcePosition,
    },
    Variable {
        name: String,
        position: SourcePosition,
    },
    Wildcard {
        position: SourcePosition,
    },
}

// ============================================================================
// Phase 9: Graph Pattern Matching AST Nodes
// ============================================================================

/// Graph pattern match expression (Phase 9)
/// Represents Cypher-style graph patterns: (node:Type) -[:EDGE]-> (other:Type)
#[derive(Debug, Clone, PartialEq)]
pub struct GraphPattern {
    pub nodes: Vec<PatternNode>,
    pub edges: Vec<PatternEdge>,
    pub where_clause: Option<Vec<Expr>>,
    pub return_clause: Option<Vec<Expr>>,
    pub position: SourcePosition,
}

/// Node in a graph pattern: (variable:Type)
#[derive(Debug, Clone, PartialEq)]
pub struct PatternNode {
    pub variable: String,           // Variable name (e.g., "person")
    pub node_type: Option<String>,  // Optional type (e.g., "User")
    pub position: SourcePosition,
}

/// Edge in a graph pattern: -[:TYPE]-> or -[:TYPE*min..max]->
#[derive(Debug, Clone, PartialEq)]
pub struct PatternEdge {
    pub from: String,                      // Source node variable
    pub to: String,                        // Target node variable
    pub edge_type: Option<String>,         // Optional edge type
    pub direction: EdgeDirection,          // Directed or bidirectional
    pub length: EdgeLength,                // Fixed or variable-length
    pub position: SourcePosition,
}

/// Edge direction in graph patterns
#[derive(Debug, Clone, PartialEq)]
pub enum EdgeDirection {
    Directed,       // -> (one direction)
    Bidirectional,  // - (both directions)
}

/// Edge length specification for graph patterns
#[derive(Debug, Clone, PartialEq)]
pub enum EdgeLength {
    Fixed,                      // Single edge
    Variable { min: usize, max: usize },  // Variable-length path
}

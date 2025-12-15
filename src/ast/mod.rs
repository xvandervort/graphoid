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
        receiver: Option<String>,  // For method syntax: fn Graph.method() - receiver is "Graph"
        params: Vec<Parameter>,
        body: Vec<Stmt>,
        pattern_clauses: Option<Vec<PatternClause>>,  // Phase 7: Pattern matching
        is_private: bool,  // Phase 10: priv keyword support
        is_getter: bool,  // Phase 17: True if defined with `get` keyword (computed property)
        is_setter: bool,  // Phase 19: True if defined with `set` keyword (computed property assignment)
        is_static: bool,  // Phase 20: True if defined with `static` keyword (class method)
        guard: Option<Box<Expr>>,  // Phase 21: Guard clause for structure-based dispatch (`when` clause)
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
    /// Named graph declaration: graph Name { configure {...}, properties, methods, rules }
    /// This creates a graph type with intrinsic identity
    GraphDecl {
        name: String,
        graph_type: Option<String>,  // Optional: :dag, :tree, etc.
        parent: Option<Box<Expr>>,   // Optional: graph Name from Parent { }
        properties: Vec<GraphProperty>,
        methods: Vec<GraphMethod>,
        rules: Vec<GraphRule>,       // rule :no_cycles, rule :max_degree, 3
        config: std::collections::HashMap<String, Vec<String>>,  // configure { readable: [:x], writable: :y }
        position: SourcePosition,
    },
    Expression {
        expr: Expr,
        position: SourcePosition,
    },
}

/// A property declaration inside a graph body: name: value
#[derive(Debug, Clone, PartialEq)]
pub struct GraphProperty {
    pub name: String,
    pub value: Expr,
    pub position: SourcePosition,
}

/// A method declaration inside a graph body: fn name(params) { body }
#[derive(Debug, Clone, PartialEq)]
pub struct GraphMethod {
    pub name: String,
    pub params: Vec<Parameter>,
    pub body: Vec<Stmt>,
    pub is_static: bool,
    pub is_getter: bool,
    pub is_setter: bool,
    pub is_private: bool,
    pub guard: Option<Box<Expr>>,
    pub position: SourcePosition,
}

/// A rule declaration inside a graph body: rule :no_cycles or rule :max_degree, 3
#[derive(Debug, Clone, PartialEq)]
pub struct GraphRule {
    pub name: String,                // The rule name (e.g., "no_cycles", "max_degree")
    pub param: Option<Expr>,         // Optional parameter (e.g., 3 for :max_degree, 3)
    pub position: SourcePosition,
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
    /// Super method call: super.method(args)
    /// Calls parent graph's implementation of a method
    SuperMethodCall {
        method: String,
        args: Vec<Argument>,
        position: SourcePosition,
    },
    /// Property access: object.property (no parentheses)
    /// Used for data node access on graphs and key access on hashes
    PropertyAccess {
        object: Box<Expr>,
        property: String,
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
        parent: Option<Box<Expr>>,  // For inheritance: graph from Parent {}
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
            Expr::PropertyAccess { position, .. } => position,
            Expr::Index { position, .. } => position,
            Expr::Lambda { position, .. } => position,
            Expr::Block { position, .. } => position,
            Expr::List { position, .. } => position,
            Expr::Map { position, .. } => position,
            Expr::Graph { position, .. } => position,
            Expr::Conditional { position, .. } => position,
            Expr::Raise { position, .. } => position,
            Expr::Match { position, .. } => position,
            Expr::SuperMethodCall { position, .. } => position,
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
    Power,       // ** (Phase 13+)

    // Bitwise (Phase 13)
    BitwiseAnd,      // &
    BitwiseOr,       // |
    BitwiseXor,      // ^
    LeftShift,       // <<
    RightShift,      // >>

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
    DotXor,      // .^ (element-wise XOR, Phase 13+)
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
    BitwiseNot,  // ~ (Phase 13)
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
    /// Property access assignment: object.property = value
    Property {
        object: Box<Expr>,
        property: String,
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

// ============================================================================
// Semantic Edges: AST Analysis Helpers
// ============================================================================

/// Result of analyzing a method body for property references
#[derive(Debug, Clone, Default)]
pub struct PropertyReferences {
    /// Properties that are read (variable access)
    pub reads: Vec<String>,
    /// Properties that are written (assignment targets)
    pub writes: Vec<String>,
}

impl PropertyReferences {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Extract property references from a method body
/// Given a list of valid property names, find reads and writes
pub fn extract_property_references(body: &[Stmt], properties: &[String]) -> PropertyReferences {
    let mut refs = PropertyReferences::new();
    let prop_set: std::collections::HashSet<&String> = properties.iter().collect();

    for stmt in body {
        collect_from_stmt(stmt, &prop_set, &mut refs);
    }

    // Remove duplicates
    refs.reads.sort();
    refs.reads.dedup();
    refs.writes.sort();
    refs.writes.dedup();

    refs
}

fn collect_from_stmt(stmt: &Stmt, properties: &std::collections::HashSet<&String>, refs: &mut PropertyReferences) {
    match stmt {
        Stmt::Assignment { target, value, .. } => {
            // Check if assignment target is a property
            if let AssignmentTarget::Variable(name) = target {
                if properties.contains(name) {
                    refs.writes.push(name.clone());
                }
            }
            // Also collect reads from the value expression
            collect_from_expr(value, properties, refs);
        }
        Stmt::VariableDecl { value, .. } => {
            collect_from_expr(value, properties, refs);
        }
        Stmt::Return { value, .. } => {
            if let Some(expr) = value {
                collect_from_expr(expr, properties, refs);
            }
        }
        Stmt::Expression { expr, .. } => {
            collect_from_expr(expr, properties, refs);
        }
        Stmt::If { condition, then_branch, else_branch, .. } => {
            collect_from_expr(condition, properties, refs);
            for s in then_branch {
                collect_from_stmt(s, properties, refs);
            }
            if let Some(else_stmts) = else_branch {
                for s in else_stmts {
                    collect_from_stmt(s, properties, refs);
                }
            }
        }
        Stmt::While { condition, body, .. } => {
            collect_from_expr(condition, properties, refs);
            for s in body {
                collect_from_stmt(s, properties, refs);
            }
        }
        Stmt::For { iterable, body, .. } => {
            collect_from_expr(iterable, properties, refs);
            for s in body {
                collect_from_stmt(s, properties, refs);
            }
        }
        _ => {}
    }
}

fn collect_from_expr(expr: &Expr, properties: &std::collections::HashSet<&String>, refs: &mut PropertyReferences) {
    match expr {
        Expr::Variable { name, .. } => {
            if properties.contains(name) {
                refs.reads.push(name.clone());
            }
        }
        Expr::Binary { left, right, .. } => {
            collect_from_expr(left, properties, refs);
            collect_from_expr(right, properties, refs);
        }
        Expr::Unary { operand, .. } => {
            collect_from_expr(operand, properties, refs);
        }
        Expr::Call { callee, args, .. } => {
            collect_from_expr(callee, properties, refs);
            for arg in args {
                match arg {
                    Argument::Positional(expr) => collect_from_expr(expr, properties, refs),
                    Argument::Named { value, .. } => collect_from_expr(value, properties, refs),
                }
            }
        }
        Expr::MethodCall { object, args, .. } => {
            collect_from_expr(object, properties, refs);
            for arg in args {
                match arg {
                    Argument::Positional(expr) => collect_from_expr(expr, properties, refs),
                    Argument::Named { value, .. } => collect_from_expr(value, properties, refs),
                }
            }
        }
        Expr::Index { object, index, .. } => {
            collect_from_expr(object, properties, refs);
            collect_from_expr(index, properties, refs);
        }
        Expr::PropertyAccess { object, .. } => {
            collect_from_expr(object, properties, refs);
        }
        Expr::List { elements, .. } => {
            for elem in elements {
                collect_from_expr(elem, properties, refs);
            }
        }
        Expr::Map { entries, .. } => {
            for (_, v) in entries {
                collect_from_expr(v, properties, refs);
            }
        }
        Expr::Lambda { body, .. } => {
            collect_from_expr(body, properties, refs);
        }
        Expr::Conditional { condition, then_expr, else_expr, .. } => {
            collect_from_expr(condition, properties, refs);
            collect_from_expr(then_expr, properties, refs);
            if let Some(else_e) = else_expr {
                collect_from_expr(else_e, properties, refs);
            }
        }
        _ => {}
    }
}

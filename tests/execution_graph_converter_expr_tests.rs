use graphoid::execution_graph::node::{AstNodeType, ExecEdgeType, AstProperty};
use graphoid::execution_graph::converter::AstToGraphConverter;
use graphoid::ast::*;
use graphoid::error::SourcePosition;

fn dummy_pos() -> SourcePosition {
    SourcePosition { line: 1, column: 1, file: None }
}

fn convert_expr(expr: &Expr) -> (graphoid::execution_graph::ExecutionGraph, graphoid::execution_graph::arena::NodeRef) {
    let mut converter = AstToGraphConverter::new();
    let node_ref = converter.convert_expr(expr);
    (converter.into_graph(), node_ref)
}

// --- Literal tests ---

#[test]
fn test_convert_number_literal() {
    let expr = Expr::Literal {
        value: LiteralValue::Number(42.0),
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::NumberLit);
    assert_eq!(node.properties.get("value"), Some(&AstProperty::Num(42.0)));
}

#[test]
fn test_convert_string_literal() {
    let expr = Expr::Literal {
        value: LiteralValue::String("hello".to_string()),
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::StringLit);
    assert_eq!(node.properties.get("value"), Some(&AstProperty::Str("hello".to_string())));
}

#[test]
fn test_convert_bool_literal() {
    let expr = Expr::Literal {
        value: LiteralValue::Boolean(true),
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::BoolLit);
    assert_eq!(node.properties.get("value"), Some(&AstProperty::Bool(true)));
}

#[test]
fn test_convert_none_literal() {
    let expr = Expr::Literal {
        value: LiteralValue::None,
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::NoneLit);
}

#[test]
fn test_convert_symbol_literal() {
    let expr = Expr::Literal {
        value: LiteralValue::Symbol("my_symbol".to_string()),
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::SymbolLit);
    assert_eq!(node.properties.get("value"), Some(&AstProperty::Str("my_symbol".to_string())));
}

// --- Variable ---

#[test]
fn test_convert_variable() {
    let expr = Expr::Variable {
        name: "x".to_string(),
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::Identifier);
    assert_eq!(node.properties.get("name"), Some(&AstProperty::Str("x".to_string())));
}

// --- Binary ---

#[test]
fn test_convert_binary_expr() {
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal { value: LiteralValue::Number(3.0), position: dummy_pos() }),
        op: BinaryOp::Add,
        right: Box::new(Expr::Literal { value: LiteralValue::Number(4.0), position: dummy_pos() }),
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::BinaryExpr);
    assert_eq!(node.properties.get("operator"), Some(&AstProperty::BinaryOp(BinaryOp::Add)));

    // Check left and right edges
    let left = graph.get_edge_target(root, &ExecEdgeType::Left).unwrap();
    let right = graph.get_edge_target(root, &ExecEdgeType::Right).unwrap();
    assert_eq!(graph.get_node(left).unwrap().node_type, AstNodeType::NumberLit);
    assert_eq!(graph.get_node(right).unwrap().node_type, AstNodeType::NumberLit);
}

// --- Unary ---

#[test]
fn test_convert_unary_expr() {
    let expr = Expr::Unary {
        op: UnaryOp::Negate,
        operand: Box::new(Expr::Literal { value: LiteralValue::Number(5.0), position: dummy_pos() }),
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::UnaryExpr);
    assert_eq!(node.properties.get("operator"), Some(&AstProperty::UnaryOp(UnaryOp::Negate)));

    let operand = graph.get_edge_target(root, &ExecEdgeType::Operand).unwrap();
    assert_eq!(graph.get_node(operand).unwrap().node_type, AstNodeType::NumberLit);
}

// --- Call ---

#[test]
fn test_convert_call_expr() {
    let expr = Expr::Call {
        callee: Box::new(Expr::Variable { name: "add".to_string(), position: dummy_pos() }),
        args: vec![
            Argument::Positional { expr: Expr::Literal { value: LiteralValue::Number(1.0), position: dummy_pos() }, mutable: false },
            Argument::Positional { expr: Expr::Literal { value: LiteralValue::Number(2.0), position: dummy_pos() }, mutable: false },
        ],
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::CallExpr);

    // Callee edge
    let callee = graph.get_edge_target(root, &ExecEdgeType::Callee).unwrap();
    assert_eq!(graph.get_node(callee).unwrap().node_type, AstNodeType::Identifier);

    // Arguments in order
    let args = graph.get_ordered_edges(root, "Argument");
    assert_eq!(args.len(), 2);
    assert_eq!(graph.get_node(args[0]).unwrap().node_type, AstNodeType::NumberLit);
    assert_eq!(graph.get_node(args[1]).unwrap().node_type, AstNodeType::NumberLit);
}

#[test]
fn test_convert_call_with_named_arg() {
    let expr = Expr::Call {
        callee: Box::new(Expr::Variable { name: "f".to_string(), position: dummy_pos() }),
        args: vec![
            Argument::Named { name: "key".to_string(), value: Expr::Literal { value: LiteralValue::Number(1.0), position: dummy_pos() }, mutable: false },
        ],
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let args = graph.get_ordered_edges(root, "Argument");
    assert_eq!(args.len(), 1);
    // Named arg stores the name as a property
    let arg_node = graph.get_node(args[0]).unwrap();
    assert_eq!(arg_node.properties.get("arg_name"), Some(&AstProperty::Str("key".to_string())));
}

#[test]
fn test_convert_call_with_mutable_arg() {
    let expr = Expr::Call {
        callee: Box::new(Expr::Variable { name: "f".to_string(), position: dummy_pos() }),
        args: vec![
            Argument::Positional {
                expr: Expr::Variable { name: "x".to_string(), position: dummy_pos() },
                mutable: true,
            },
        ],
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let args = graph.get_ordered_edges(root, "Argument");
    assert_eq!(args.len(), 1);
    let arg_node = graph.get_node(args[0]).unwrap();
    assert_eq!(arg_node.properties.get("mutable"), Some(&AstProperty::Bool(true)));
}

// --- MethodCall ---

#[test]
fn test_convert_method_call() {
    let expr = Expr::MethodCall {
        object: Box::new(Expr::Variable { name: "list".to_string(), position: dummy_pos() }),
        method: "length".to_string(),
        args: vec![],
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::MethodCallExpr);
    assert_eq!(node.properties.get("method"), Some(&AstProperty::Str("length".to_string())));

    let object = graph.get_edge_target(root, &ExecEdgeType::Object).unwrap();
    assert_eq!(graph.get_node(object).unwrap().node_type, AstNodeType::Identifier);
}

// --- SuperMethodCall ---

#[test]
fn test_convert_super_method_call() {
    let expr = Expr::SuperMethodCall {
        method: "init".to_string(),
        args: vec![
            Argument::Positional { expr: Expr::Literal { value: LiteralValue::Number(1.0), position: dummy_pos() }, mutable: false },
        ],
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::SuperMethodCallExpr);
    assert_eq!(node.properties.get("method"), Some(&AstProperty::Str("init".to_string())));
    let args = graph.get_ordered_edges(root, "Argument");
    assert_eq!(args.len(), 1);
}

// --- PropertyAccess ---

#[test]
fn test_convert_property_access() {
    let expr = Expr::PropertyAccess {
        object: Box::new(Expr::Variable { name: "obj".to_string(), position: dummy_pos() }),
        property: "name".to_string(),
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::PropertyAccessExpr);
    assert_eq!(node.properties.get("property"), Some(&AstProperty::Str("name".to_string())));

    let object = graph.get_edge_target(root, &ExecEdgeType::Object).unwrap();
    assert_eq!(graph.get_node(object).unwrap().node_type, AstNodeType::Identifier);
}

// --- Index ---

#[test]
fn test_convert_index_expr() {
    let expr = Expr::Index {
        object: Box::new(Expr::Variable { name: "arr".to_string(), position: dummy_pos() }),
        index: Box::new(Expr::Literal { value: LiteralValue::Number(0.0), position: dummy_pos() }),
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::IndexExpr);

    let object = graph.get_edge_target(root, &ExecEdgeType::Object).unwrap();
    let index = graph.get_edge_target(root, &ExecEdgeType::ValueEdge).unwrap();
    assert_eq!(graph.get_node(object).unwrap().node_type, AstNodeType::Identifier);
    assert_eq!(graph.get_node(index).unwrap().node_type, AstNodeType::NumberLit);
}

// --- Lambda ---

#[test]
fn test_convert_lambda() {
    let expr = Expr::Lambda {
        params: vec!["x".to_string(), "y".to_string()],
        body: Box::new(Expr::Binary {
            left: Box::new(Expr::Variable { name: "x".to_string(), position: dummy_pos() }),
            op: BinaryOp::Add,
            right: Box::new(Expr::Variable { name: "y".to_string(), position: dummy_pos() }),
            position: dummy_pos(),
        }),
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::LambdaExpr);
    // Params stored as comma-separated string or individual properties
    assert_eq!(node.properties.get("param_count"), Some(&AstProperty::Int(2)));

    let body = graph.get_edge_target(root, &ExecEdgeType::Body).unwrap();
    assert_eq!(graph.get_node(body).unwrap().node_type, AstNodeType::BinaryExpr);
}

// --- Block ---

#[test]
fn test_convert_block() {
    let expr = Expr::Block {
        statements: vec![
            Stmt::Expression {
                expr: Expr::Literal { value: LiteralValue::Number(1.0), position: dummy_pos() },
                position: dummy_pos(),
            },
            Stmt::Expression {
                expr: Expr::Literal { value: LiteralValue::Number(2.0), position: dummy_pos() },
                position: dummy_pos(),
            },
        ],
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::BlockExpr);

    let elements = graph.get_ordered_edges(root, "Element");
    assert_eq!(elements.len(), 2);
}

// --- List ---

#[test]
fn test_convert_list() {
    let expr = Expr::List {
        elements: vec![
            Expr::Literal { value: LiteralValue::Number(1.0), position: dummy_pos() },
            Expr::Literal { value: LiteralValue::Number(2.0), position: dummy_pos() },
            Expr::Literal { value: LiteralValue::Number(3.0), position: dummy_pos() },
        ],
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::ListExpr);

    let elements = graph.get_ordered_edges(root, "Element");
    assert_eq!(elements.len(), 3);
}

// --- Map ---

#[test]
fn test_convert_map() {
    let expr = Expr::Map {
        entries: vec![
            ("a".to_string(), Expr::Literal { value: LiteralValue::Number(1.0), position: dummy_pos() }),
            ("b".to_string(), Expr::Literal { value: LiteralValue::Number(2.0), position: dummy_pos() }),
        ],
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::MapExpr);

    let entries = graph.get_ordered_edges(root, "Element");
    assert_eq!(entries.len(), 2);
    // Each entry node has a key property
    let entry0 = graph.get_node(entries[0]).unwrap();
    assert_eq!(entry0.node_type, AstNodeType::MapEntryNode);
    assert_eq!(entry0.properties.get("key"), Some(&AstProperty::Str("a".to_string())));
    // Each entry has a ValueEdge to the value expression
    let val = graph.get_edge_target(entries[0], &ExecEdgeType::ValueEdge).unwrap();
    assert_eq!(graph.get_node(val).unwrap().node_type, AstNodeType::NumberLit);
}

// --- Graph ---

#[test]
fn test_convert_graph_expr() {
    let expr = Expr::Graph {
        config: vec![
            ("type".to_string(), Expr::Literal { value: LiteralValue::Symbol("dag".to_string()), position: dummy_pos() }),
        ],
        parent: None,
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::GraphExpr);

    // Config as Setting edges
    let setting = graph.get_edge_target(root, &ExecEdgeType::Setting("type".to_string()));
    assert!(setting.is_some());
}

#[test]
fn test_convert_graph_expr_with_parent() {
    let expr = Expr::Graph {
        config: vec![],
        parent: Some(Box::new(Expr::Variable { name: "BaseGraph".to_string(), position: dummy_pos() })),
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let parent = graph.get_edge_target(root, &ExecEdgeType::Parent);
    assert!(parent.is_some());
    assert_eq!(graph.get_node(parent.unwrap()).unwrap().node_type, AstNodeType::Identifier);
}

// --- Conditional ---

#[test]
fn test_convert_conditional() {
    let expr = Expr::Conditional {
        condition: Box::new(Expr::Literal { value: LiteralValue::Boolean(true), position: dummy_pos() }),
        then_expr: Box::new(Expr::Literal { value: LiteralValue::Number(1.0), position: dummy_pos() }),
        else_expr: Some(Box::new(Expr::Literal { value: LiteralValue::Number(2.0), position: dummy_pos() })),
        is_unless: false,
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::ConditionalExpr);
    assert_eq!(node.properties.get("is_unless"), Some(&AstProperty::Bool(false)));

    assert!(graph.get_edge_target(root, &ExecEdgeType::Condition).is_some());
    assert!(graph.get_edge_target(root, &ExecEdgeType::ThenBranch).is_some());
    assert!(graph.get_edge_target(root, &ExecEdgeType::ElseBranch).is_some());
}

// --- Raise ---

#[test]
fn test_convert_raise() {
    let expr = Expr::Raise {
        error: Box::new(Expr::Literal { value: LiteralValue::String("oops".to_string()), position: dummy_pos() }),
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::RaiseExpr);
    assert!(graph.get_edge_target(root, &ExecEdgeType::ValueEdge).is_some());
}

// --- Match ---

#[test]
fn test_convert_match() {
    let expr = Expr::Match {
        value: Box::new(Expr::Variable { name: "x".to_string(), position: dummy_pos() }),
        arms: vec![
            MatchArm {
                pattern: MatchPattern::Literal(LiteralValue::Number(1.0)),
                body: Expr::Literal { value: LiteralValue::String("one".to_string()), position: dummy_pos() },
                position: dummy_pos(),
            },
            MatchArm {
                pattern: MatchPattern::Wildcard,
                body: Expr::Literal { value: LiteralValue::String("other".to_string()), position: dummy_pos() },
                position: dummy_pos(),
            },
        ],
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::MatchExpr);

    assert!(graph.get_edge_target(root, &ExecEdgeType::MatchValue).is_some());
    let arms = graph.get_ordered_edges(root, "MatchArm");
    assert_eq!(arms.len(), 2);

    // Each arm has ArmPattern and ArmBody edges
    let arm0 = arms[0];
    assert_eq!(graph.get_node(arm0).unwrap().node_type, AstNodeType::MatchArmNode);
    assert!(graph.get_edge_target(arm0, &ExecEdgeType::ArmPattern).is_some());
    assert!(graph.get_edge_target(arm0, &ExecEdgeType::ArmBody).is_some());
}

// --- Instantiate ---

#[test]
fn test_convert_instantiate() {
    let expr = Expr::Instantiate {
        class_name: Box::new(Expr::Variable { name: "Point".to_string(), position: dummy_pos() }),
        overrides: vec![
            ("x".to_string(), Expr::Literal { value: LiteralValue::Number(3.0), position: dummy_pos() }),
            ("y".to_string(), Expr::Literal { value: LiteralValue::Number(4.0), position: dummy_pos() }),
        ],
        position: dummy_pos(),
    };
    let (graph, root) = convert_expr(&expr);
    let node = graph.get_node(root).unwrap();
    assert_eq!(node.node_type, AstNodeType::InstantiateExpr);

    let class = graph.get_edge_target(root, &ExecEdgeType::Object).unwrap();
    assert_eq!(graph.get_node(class).unwrap().node_type, AstNodeType::Identifier);

    let overrides = graph.get_ordered_edges(root, "Override");
    assert_eq!(overrides.len(), 2);
    // Override nodes have key property
    let ov0 = graph.get_node(overrides[0]).unwrap();
    assert_eq!(ov0.properties.get("key"), Some(&AstProperty::Str("x".to_string())));
}

// --- Round-trip from source ---

#[test]
fn test_round_trip_from_source() {
    use graphoid::lexer::Lexer;
    use graphoid::parser::Parser;

    let source = "3 + 4 * 2";
    let tokens = Lexer::new(source).tokenize().unwrap();
    let program = Parser::new(tokens).parse().unwrap();
    // Program has one expression statement
    assert_eq!(program.statements.len(), 1);

    // Convert the full program
    let mut converter = AstToGraphConverter::new();
    let root = converter.convert_program(&program);
    let graph = converter.into_graph();

    // Root is a Program node
    let root_node = graph.get_node(root).unwrap();
    assert_eq!(root_node.node_type, AstNodeType::Program);

    // Has 1 element (the expression statement)
    let elements = graph.get_ordered_edges(root, "Element");
    assert_eq!(elements.len(), 1);

    // The element is an ExpressionStmt
    let stmt = graph.get_node(elements[0]).unwrap();
    assert_eq!(stmt.node_type, AstNodeType::ExpressionStmt);
}

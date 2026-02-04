use graphoid::execution_graph::node::{AstGraphNode, AstNodeType, ExecEdgeType, AstProperty};
use graphoid::execution_graph::ExecutionGraph;
use graphoid::error::SourcePosition;
use graphoid::ast::BinaryOp;

fn dummy_pos() -> SourcePosition {
    SourcePosition { line: 1, column: 1, file: None }
}

#[test]
fn test_execution_graph_add_node_and_get() {
    let mut graph = ExecutionGraph::new();
    let arena = graph.new_arena();
    let node = AstGraphNode {
        node_type: AstNodeType::NumberLit,
        properties: {
            let mut m = std::collections::HashMap::new();
            m.insert("value".to_string(), AstProperty::Num(42.0));
            m
        },
        position: dummy_pos(),
    };
    let node_ref = graph.add_node(arena, node);
    let retrieved = graph.get_node(node_ref).unwrap();
    assert_eq!(retrieved.node_type, AstNodeType::NumberLit);
    assert_eq!(retrieved.properties.get("value"), Some(&AstProperty::Num(42.0)));
}

#[test]
fn test_execution_graph_add_edge_and_query() {
    let mut graph = ExecutionGraph::new();
    let arena = graph.new_arena();

    // Build: 3 + 4
    let left = graph.add_node(arena, AstGraphNode {
        node_type: AstNodeType::NumberLit,
        properties: {
            let mut m = std::collections::HashMap::new();
            m.insert("value".to_string(), AstProperty::Num(3.0));
            m
        },
        position: dummy_pos(),
    });
    let right = graph.add_node(arena, AstGraphNode {
        node_type: AstNodeType::NumberLit,
        properties: {
            let mut m = std::collections::HashMap::new();
            m.insert("value".to_string(), AstProperty::Num(4.0));
            m
        },
        position: dummy_pos(),
    });
    let binary = graph.add_node(arena, AstGraphNode {
        node_type: AstNodeType::BinaryExpr,
        properties: {
            let mut m = std::collections::HashMap::new();
            m.insert("operator".to_string(), AstProperty::BinaryOp(BinaryOp::Add));
            m
        },
        position: dummy_pos(),
    });

    graph.add_edge(binary, ExecEdgeType::Left, left);
    graph.add_edge(binary, ExecEdgeType::Right, right);

    // Query edges
    let left_target = graph.get_edge_target(binary, &ExecEdgeType::Left);
    assert_eq!(left_target, Some(left));

    let right_target = graph.get_edge_target(binary, &ExecEdgeType::Right);
    assert_eq!(right_target, Some(right));
}

#[test]
fn test_get_edge_target_returns_none_for_missing() {
    let mut graph = ExecutionGraph::new();
    let arena = graph.new_arena();
    let node = graph.add_node(arena, AstGraphNode {
        node_type: AstNodeType::NumberLit,
        properties: std::collections::HashMap::new(),
        position: dummy_pos(),
    });

    assert_eq!(graph.get_edge_target(node, &ExecEdgeType::Left), None);
}

#[test]
fn test_get_edges_returns_all() {
    let mut graph = ExecutionGraph::new();
    let arena = graph.new_arena();

    let call = graph.add_node(arena, AstGraphNode {
        node_type: AstNodeType::CallExpr,
        properties: std::collections::HashMap::new(),
        position: dummy_pos(),
    });
    let callee = graph.add_node(arena, AstGraphNode {
        node_type: AstNodeType::Identifier,
        properties: std::collections::HashMap::new(),
        position: dummy_pos(),
    });
    let arg0 = graph.add_node(arena, AstGraphNode {
        node_type: AstNodeType::NumberLit,
        properties: std::collections::HashMap::new(),
        position: dummy_pos(),
    });
    let arg1 = graph.add_node(arena, AstGraphNode {
        node_type: AstNodeType::NumberLit,
        properties: std::collections::HashMap::new(),
        position: dummy_pos(),
    });

    graph.add_edge(call, ExecEdgeType::Callee, callee);
    graph.add_edge(call, ExecEdgeType::Argument(0), arg0);
    graph.add_edge(call, ExecEdgeType::Argument(1), arg1);

    let edges = graph.get_edges(call);
    assert_eq!(edges.len(), 3);
}

#[test]
fn test_get_ordered_argument_edges() {
    let mut graph = ExecutionGraph::new();
    let arena = graph.new_arena();

    let call = graph.add_node(arena, AstGraphNode {
        node_type: AstNodeType::CallExpr,
        properties: std::collections::HashMap::new(),
        position: dummy_pos(),
    });
    let arg0 = graph.add_node(arena, AstGraphNode {
        node_type: AstNodeType::NumberLit,
        properties: std::collections::HashMap::new(),
        position: dummy_pos(),
    });
    let arg1 = graph.add_node(arena, AstGraphNode {
        node_type: AstNodeType::StringLit,
        properties: std::collections::HashMap::new(),
        position: dummy_pos(),
    });
    let arg2 = graph.add_node(arena, AstGraphNode {
        node_type: AstNodeType::BoolLit,
        properties: std::collections::HashMap::new(),
        position: dummy_pos(),
    });

    // Add in reverse order to test sorting
    graph.add_edge(call, ExecEdgeType::Argument(2), arg2);
    graph.add_edge(call, ExecEdgeType::Argument(0), arg0);
    graph.add_edge(call, ExecEdgeType::Argument(1), arg1);

    let ordered = graph.get_ordered_edges(call, "Argument");
    assert_eq!(ordered.len(), 3);
    assert_eq!(ordered[0], arg0);
    assert_eq!(ordered[1], arg1);
    assert_eq!(ordered[2], arg2);
}

#[test]
fn test_get_ordered_element_edges() {
    let mut graph = ExecutionGraph::new();
    let arena = graph.new_arena();

    let list = graph.add_node(arena, AstGraphNode {
        node_type: AstNodeType::ListExpr,
        properties: std::collections::HashMap::new(),
        position: dummy_pos(),
    });
    let e0 = graph.add_node(arena, AstGraphNode {
        node_type: AstNodeType::NumberLit,
        properties: std::collections::HashMap::new(),
        position: dummy_pos(),
    });
    let e1 = graph.add_node(arena, AstGraphNode {
        node_type: AstNodeType::NumberLit,
        properties: std::collections::HashMap::new(),
        position: dummy_pos(),
    });

    graph.add_edge(list, ExecEdgeType::Element(0), e0);
    graph.add_edge(list, ExecEdgeType::Element(1), e1);

    let ordered = graph.get_ordered_edges(list, "Element");
    assert_eq!(ordered.len(), 2);
    assert_eq!(ordered[0], e0);
    assert_eq!(ordered[1], e1);
}

#[test]
fn test_set_root() {
    let mut graph = ExecutionGraph::new();
    let arena = graph.new_arena();
    let node = graph.add_node(arena, AstGraphNode {
        node_type: AstNodeType::Program,
        properties: std::collections::HashMap::new(),
        position: dummy_pos(),
    });
    graph.set_root(node);
    assert_eq!(graph.root(), Some(node));
}

#[test]
fn test_root_default_none() {
    let graph = ExecutionGraph::new();
    assert_eq!(graph.root(), None);
}

#[test]
fn test_node_type_variants_exist() {
    // Verify key node types can be constructed
    let types = vec![
        AstNodeType::NumberLit,
        AstNodeType::StringLit,
        AstNodeType::BoolLit,
        AstNodeType::NoneLit,
        AstNodeType::SymbolLit,
        AstNodeType::Identifier,
        AstNodeType::BinaryExpr,
        AstNodeType::UnaryExpr,
        AstNodeType::CallExpr,
        AstNodeType::MethodCallExpr,
        AstNodeType::SuperMethodCallExpr,
        AstNodeType::PropertyAccessExpr,
        AstNodeType::IndexExpr,
        AstNodeType::LambdaExpr,
        AstNodeType::BlockExpr,
        AstNodeType::ListExpr,
        AstNodeType::MapExpr,
        AstNodeType::GraphExpr,
        AstNodeType::ConditionalExpr,
        AstNodeType::RaiseExpr,
        AstNodeType::MatchExpr,
        AstNodeType::InstantiateExpr,
        AstNodeType::VarDeclStmt,
        AstNodeType::AssignStmt,
        AstNodeType::FuncDeclStmt,
        AstNodeType::IfStmt,
        AstNodeType::WhileStmt,
        AstNodeType::ForStmt,
        AstNodeType::ReturnStmt,
        AstNodeType::BreakStmt,
        AstNodeType::ContinueStmt,
        AstNodeType::ImportStmt,
        AstNodeType::ModuleDeclStmt,
        AstNodeType::LoadStmt,
        AstNodeType::ConfigureStmt,
        AstNodeType::PrecisionStmt,
        AstNodeType::TryStmt,
        AstNodeType::GraphDeclStmt,
        AstNodeType::ExpressionStmt,
        AstNodeType::Program,
        AstNodeType::MatchArmNode,
        AstNodeType::MatchPatternNode,
        AstNodeType::PatternClauseNode,
        AstNodeType::CatchClauseNode,
        AstNodeType::MapEntryNode,
        AstNodeType::GraphPropertyNode,
        AstNodeType::GraphMethodNode,
        AstNodeType::GraphRuleNode,
    ];
    // Just verify they all exist and are different from at least some others
    assert!(types.len() > 40);
}

#[test]
fn test_edge_type_variants_exist() {
    // Verify key edge types can be constructed
    let edges = vec![
        ExecEdgeType::Left,
        ExecEdgeType::Right,
        ExecEdgeType::Operand,
        ExecEdgeType::Callee,
        ExecEdgeType::Argument(0),
        ExecEdgeType::Condition,
        ExecEdgeType::ThenBranch,
        ExecEdgeType::ElseBranch,
        ExecEdgeType::Body,
        ExecEdgeType::Target,
        ExecEdgeType::ValueEdge,
        ExecEdgeType::Element(0),
        ExecEdgeType::Object,
        ExecEdgeType::Parameter(0),
        ExecEdgeType::CatchHandler(0),
        ExecEdgeType::FinallyBlock,
        ExecEdgeType::Parent,
        ExecEdgeType::MatchValue,
        ExecEdgeType::MatchArm(0),
        ExecEdgeType::ArmBody,
        ExecEdgeType::ArmPattern,
        ExecEdgeType::Iterable,
        ExecEdgeType::Property(0),
        ExecEdgeType::Rule(0),
        ExecEdgeType::GraphMethod(0),
        ExecEdgeType::Guard,
        ExecEdgeType::Override(0),
        ExecEdgeType::Setting(String::new()),
        ExecEdgeType::DefaultValue,
    ];
    assert!(edges.len() > 25);
}

#[test]
fn test_ast_property_variants() {
    let props = vec![
        AstProperty::Str("hello".to_string()),
        AstProperty::Num(3.14),
        AstProperty::Bool(true),
        AstProperty::Int(42),
        AstProperty::BinaryOp(BinaryOp::Add),
        AstProperty::UnaryOp(graphoid::ast::UnaryOp::Negate),
        AstProperty::None,
    ];
    assert_eq!(props.len(), 7);

    // Test equality
    assert_eq!(AstProperty::Num(1.0), AstProperty::Num(1.0));
    assert_ne!(AstProperty::Num(1.0), AstProperty::Num(2.0));
    assert_eq!(AstProperty::Str("a".to_string()), AstProperty::Str("a".to_string()));
}

#[test]
fn test_execution_graph_node_count() {
    let mut graph = ExecutionGraph::new();
    assert_eq!(graph.node_count(), 0);

    let arena = graph.new_arena();
    graph.add_node(arena, AstGraphNode {
        node_type: AstNodeType::NumberLit,
        properties: std::collections::HashMap::new(),
        position: dummy_pos(),
    });
    assert_eq!(graph.node_count(), 1);

    graph.add_node(arena, AstGraphNode {
        node_type: AstNodeType::NumberLit,
        properties: std::collections::HashMap::new(),
        position: dummy_pos(),
    });
    assert_eq!(graph.node_count(), 2);
}

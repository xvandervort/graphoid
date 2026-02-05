use graphoid::execution_graph::node::{AstNodeType, ExecEdgeType, AstProperty};
use graphoid::execution_graph::converter::AstToGraphConverter;
use graphoid::ast::*;
use graphoid::error::SourcePosition;

fn dummy_pos() -> SourcePosition {
    SourcePosition { line: 1, column: 1, file: None }
}

fn num_expr(n: f64) -> Expr {
    Expr::Literal { value: LiteralValue::Number(n), position: dummy_pos() }
}

fn var_expr(name: &str) -> Expr {
    Expr::Variable { name: name.to_string(), position: dummy_pos() }
}

fn convert_program(stmts: Vec<Stmt>) -> (graphoid::execution_graph::ExecutionGraph, graphoid::execution_graph::arena::NodeRef) {
    let program = Program { statements: stmts };
    let mut converter = AstToGraphConverter::new();
    let root = converter.convert_program(&program);
    (converter.into_graph(), root)
}

// --- Variable Declaration ---

#[test]
fn test_convert_var_decl() {
    let stmt = Stmt::VariableDecl {
        name: "x".to_string(),
        type_annotation: None,
        value: num_expr(42.0),
        is_private: false,
        position: dummy_pos(),
    };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    assert_eq!(elements.len(), 1);

    let decl = graph.get_node(elements[0]).unwrap();
    assert_eq!(decl.node_type, AstNodeType::VarDeclStmt);
    assert_eq!(decl.properties.get("name"), Some(&AstProperty::Str("x".to_string())));
    assert_eq!(decl.properties.get("is_private"), Some(&AstProperty::Bool(false)));

    // Value edge
    let val = graph.get_edge_target(elements[0], &ExecEdgeType::ValueEdge).unwrap();
    assert_eq!(graph.get_node(val).unwrap().node_type, AstNodeType::NumberLit);
}

#[test]
fn test_convert_var_decl_with_type_annotation() {
    let stmt = Stmt::VariableDecl {
        name: "x".to_string(),
        type_annotation: Some(TypeAnnotation { base_type: "num".to_string(), constraint: None }),
        value: num_expr(42.0),
        is_private: false,
        position: dummy_pos(),
    };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    let decl = graph.get_node(elements[0]).unwrap();
    assert_eq!(decl.properties.get("type_base"), Some(&AstProperty::Str("num".to_string())));
}

// --- Assignment ---

#[test]
fn test_convert_assignment_variable() {
    let stmt = Stmt::Assignment {
        target: AssignmentTarget::Variable("x".to_string()),
        value: num_expr(10.0),
        position: dummy_pos(),
    };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    let assign = graph.get_node(elements[0]).unwrap();
    assert_eq!(assign.node_type, AstNodeType::AssignStmt);
    assert_eq!(assign.properties.get("target_type"), Some(&AstProperty::Str("variable".to_string())));
    assert_eq!(assign.properties.get("target_name"), Some(&AstProperty::Str("x".to_string())));

    let val = graph.get_edge_target(elements[0], &ExecEdgeType::ValueEdge).unwrap();
    assert_eq!(graph.get_node(val).unwrap().node_type, AstNodeType::NumberLit);
}

#[test]
fn test_convert_assignment_index() {
    let stmt = Stmt::Assignment {
        target: AssignmentTarget::Index {
            object: Box::new(var_expr("arr")),
            index: Box::new(num_expr(0.0)),
        },
        value: num_expr(99.0),
        position: dummy_pos(),
    };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    let assign = graph.get_node(elements[0]).unwrap();
    assert_eq!(assign.properties.get("target_type"), Some(&AstProperty::Str("index".to_string())));

    // Object and index edges on the target
    assert!(graph.get_edge_target(elements[0], &ExecEdgeType::Object).is_some());
    assert!(graph.get_edge_target(elements[0], &ExecEdgeType::Target).is_some());
}

#[test]
fn test_convert_assignment_property() {
    let stmt = Stmt::Assignment {
        target: AssignmentTarget::Property {
            object: Box::new(var_expr("obj")),
            property: "name".to_string(),
        },
        value: Expr::Literal { value: LiteralValue::String("Alice".to_string()), position: dummy_pos() },
        position: dummy_pos(),
    };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    let assign = graph.get_node(elements[0]).unwrap();
    assert_eq!(assign.properties.get("target_type"), Some(&AstProperty::Str("property".to_string())));
    assert_eq!(assign.properties.get("target_name"), Some(&AstProperty::Str("name".to_string())));
    assert!(graph.get_edge_target(elements[0], &ExecEdgeType::Object).is_some());
}

// --- If Statement ---

#[test]
fn test_convert_if_stmt() {
    let stmt = Stmt::If {
        condition: Expr::Literal { value: LiteralValue::Boolean(true), position: dummy_pos() },
        then_branch: vec![Stmt::Expression { expr: num_expr(1.0), position: dummy_pos() }],
        else_branch: Some(vec![Stmt::Expression { expr: num_expr(2.0), position: dummy_pos() }]),
        position: dummy_pos(),
    };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    let if_node = elements[0];
    assert_eq!(graph.get_node(if_node).unwrap().node_type, AstNodeType::IfStmt);

    assert!(graph.get_edge_target(if_node, &ExecEdgeType::Condition).is_some());
    assert!(graph.get_edge_target(if_node, &ExecEdgeType::ThenBranch).is_some());
    assert!(graph.get_edge_target(if_node, &ExecEdgeType::ElseBranch).is_some());

    // Then branch is a block with 1 element
    let then_ref = graph.get_edge_target(if_node, &ExecEdgeType::ThenBranch).unwrap();
    let then_elements = graph.get_ordered_edges(then_ref, "Element");
    assert_eq!(then_elements.len(), 1);
}

#[test]
fn test_convert_if_stmt_no_else() {
    let stmt = Stmt::If {
        condition: Expr::Literal { value: LiteralValue::Boolean(true), position: dummy_pos() },
        then_branch: vec![Stmt::Expression { expr: num_expr(1.0), position: dummy_pos() }],
        else_branch: None,
        position: dummy_pos(),
    };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    let if_node = elements[0];
    assert!(graph.get_edge_target(if_node, &ExecEdgeType::ElseBranch).is_none());
}

// --- While ---

#[test]
fn test_convert_while_stmt() {
    let stmt = Stmt::While {
        condition: Expr::Literal { value: LiteralValue::Boolean(true), position: dummy_pos() },
        body: vec![Stmt::Break { position: dummy_pos() }],
        position: dummy_pos(),
    };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    let while_node = elements[0];
    assert_eq!(graph.get_node(while_node).unwrap().node_type, AstNodeType::WhileStmt);
    assert!(graph.get_edge_target(while_node, &ExecEdgeType::Condition).is_some());
    assert!(graph.get_edge_target(while_node, &ExecEdgeType::Body).is_some());
}

// --- For ---

#[test]
fn test_convert_for_stmt() {
    let stmt = Stmt::For {
        variable: "i".to_string(),
        iterable: Expr::List { elements: vec![num_expr(1.0), num_expr(2.0)], position: dummy_pos() },
        body: vec![Stmt::Expression { expr: var_expr("i"), position: dummy_pos() }],
        position: dummy_pos(),
    };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    let for_node = elements[0];
    assert_eq!(graph.get_node(for_node).unwrap().node_type, AstNodeType::ForStmt);
    assert_eq!(graph.get_node(for_node).unwrap().properties.get("variable"), Some(&AstProperty::Str("i".to_string())));
    assert!(graph.get_edge_target(for_node, &ExecEdgeType::Iterable).is_some());
    assert!(graph.get_edge_target(for_node, &ExecEdgeType::Body).is_some());
}

// --- Return ---

#[test]
fn test_convert_return_with_value() {
    let stmt = Stmt::Return { value: Some(num_expr(42.0)), position: dummy_pos() };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    let ret = elements[0];
    assert_eq!(graph.get_node(ret).unwrap().node_type, AstNodeType::ReturnStmt);
    assert!(graph.get_edge_target(ret, &ExecEdgeType::ValueEdge).is_some());
}

#[test]
fn test_convert_return_no_value() {
    let stmt = Stmt::Return { value: None, position: dummy_pos() };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    let ret = elements[0];
    assert_eq!(graph.get_node(ret).unwrap().node_type, AstNodeType::ReturnStmt);
    assert!(graph.get_edge_target(ret, &ExecEdgeType::ValueEdge).is_none());
}

// --- Break/Continue ---

#[test]
fn test_convert_break_continue() {
    let stmts = vec![
        Stmt::Break { position: dummy_pos() },
        Stmt::Continue { position: dummy_pos() },
    ];
    let (graph, root) = convert_program(stmts);
    let elements = graph.get_ordered_edges(root, "Element");
    assert_eq!(elements.len(), 2);
    assert_eq!(graph.get_node(elements[0]).unwrap().node_type, AstNodeType::BreakStmt);
    assert_eq!(graph.get_node(elements[1]).unwrap().node_type, AstNodeType::ContinueStmt);
}

// --- Import ---

#[test]
fn test_convert_import() {
    let stmt = Stmt::Import { module: "math".to_string(), alias: None, selections: None, position: dummy_pos() };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    let import = graph.get_node(elements[0]).unwrap();
    assert_eq!(import.node_type, AstNodeType::ImportStmt);
    assert_eq!(import.properties.get("module"), Some(&AstProperty::Str("math".to_string())));
}

#[test]
fn test_convert_import_with_alias() {
    let stmt = Stmt::Import { module: "math".to_string(), alias: Some("m".to_string()), selections: None, position: dummy_pos() };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    let import = graph.get_node(elements[0]).unwrap();
    assert_eq!(import.properties.get("alias"), Some(&AstProperty::Str("m".to_string())));
}

// --- FunctionDecl ---

#[test]
fn test_convert_function_decl() {
    let stmt = Stmt::FunctionDecl {
        name: "add".to_string(),
        receiver: None,
        params: vec![
            Parameter { name: "a".to_string(), default_value: None, is_variadic: false },
            Parameter { name: "b".to_string(), default_value: None, is_variadic: false },
        ],
        body: vec![
            Stmt::Return { value: Some(Expr::Binary {
                left: Box::new(var_expr("a")),
                op: BinaryOp::Add,
                right: Box::new(var_expr("b")),
                position: dummy_pos(),
            }), position: dummy_pos() },
        ],
        pattern_clauses: None,
        is_private: false,
        is_setter: false,
        is_static: false,
        guard: None,
        position: dummy_pos(),
    };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    let func = graph.get_node(elements[0]).unwrap();
    assert_eq!(func.node_type, AstNodeType::FuncDeclStmt);
    assert_eq!(func.properties.get("name"), Some(&AstProperty::Str("add".to_string())));
    assert_eq!(func.properties.get("param_count"), Some(&AstProperty::Int(2)));

    // Parameters
    let params = graph.get_ordered_edges(elements[0], "Parameter");
    assert_eq!(params.len(), 2);
    assert_eq!(graph.get_node(params[0]).unwrap().properties.get("name"), Some(&AstProperty::Str("a".to_string())));
    assert_eq!(graph.get_node(params[1]).unwrap().properties.get("name"), Some(&AstProperty::Str("b".to_string())));

    // Body edge
    let body = graph.get_edge_target(elements[0], &ExecEdgeType::Body);
    assert!(body.is_some());
}

#[test]
fn test_convert_function_with_default_param() {
    let stmt = Stmt::FunctionDecl {
        name: "greet".to_string(),
        receiver: None,
        params: vec![
            Parameter { name: "name".to_string(), default_value: Some(Expr::Literal { value: LiteralValue::String("World".to_string()), position: dummy_pos() }), is_variadic: false },
        ],
        body: vec![],
        pattern_clauses: None,
        is_private: false,
        is_setter: false,
        is_static: false,
        guard: None,
        position: dummy_pos(),
    };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    let params = graph.get_ordered_edges(elements[0], "Parameter");
    // Default value edge on param node
    let default_val = graph.get_edge_target(params[0], &ExecEdgeType::DefaultValue);
    assert!(default_val.is_some());
}

// --- Try/Catch ---

#[test]
fn test_convert_try_stmt() {
    let stmt = Stmt::Try {
        body: vec![Stmt::Expression { expr: num_expr(1.0), position: dummy_pos() }],
        catch_clauses: vec![
            CatchClause {
                error_type: None,
                variable: Some("e".to_string()),
                body: vec![Stmt::Expression { expr: var_expr("e"), position: dummy_pos() }],
                position: dummy_pos(),
            },
        ],
        finally_block: Some(vec![Stmt::Expression { expr: num_expr(99.0), position: dummy_pos() }]),
        position: dummy_pos(),
    };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    let try_node = elements[0];
    assert_eq!(graph.get_node(try_node).unwrap().node_type, AstNodeType::TryStmt);

    // Body
    assert!(graph.get_edge_target(try_node, &ExecEdgeType::Body).is_some());

    // Catch handlers
    let catches = graph.get_ordered_edges(try_node, "CatchHandler");
    assert_eq!(catches.len(), 1);
    let catch = graph.get_node(catches[0]).unwrap();
    assert_eq!(catch.node_type, AstNodeType::CatchClauseNode);
    assert_eq!(catch.properties.get("variable"), Some(&AstProperty::Str("e".to_string())));

    // Finally
    assert!(graph.get_edge_target(try_node, &ExecEdgeType::FinallyBlock).is_some());
}

// --- Configure ---

#[test]
fn test_convert_configure() {
    let mut settings = std::collections::HashMap::new();
    settings.insert("error_mode".to_string(), Expr::Literal { value: LiteralValue::Symbol("collect".to_string()), position: dummy_pos() });
    let stmt = Stmt::Configure {
        settings,
        body: Some(vec![Stmt::Expression { expr: num_expr(1.0), position: dummy_pos() }]),
        position: dummy_pos(),
    };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    let config = elements[0];
    assert_eq!(graph.get_node(config).unwrap().node_type, AstNodeType::ConfigureStmt);

    // Setting edge
    let setting = graph.get_edge_target(config, &ExecEdgeType::Setting("error_mode".to_string()));
    assert!(setting.is_some());

    // Body
    assert!(graph.get_edge_target(config, &ExecEdgeType::Body).is_some());
}

// --- Precision ---

#[test]
fn test_convert_precision() {
    let stmt = Stmt::Precision {
        places: Some(2),
        body: vec![Stmt::Expression { expr: num_expr(3.14), position: dummy_pos() }],
        position: dummy_pos(),
    };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    let prec = graph.get_node(elements[0]).unwrap();
    assert_eq!(prec.node_type, AstNodeType::PrecisionStmt);
    assert_eq!(prec.properties.get("places"), Some(&AstProperty::Int(2)));
}

// --- GraphDecl ---

#[test]
fn test_convert_graph_decl() {
    let stmt = Stmt::GraphDecl {
        name: "Dog".to_string(),
        graph_type: None,
        parent: None,
        properties: vec![
            GraphProperty { name: "name".to_string(), value: Expr::Literal { value: LiteralValue::String("".to_string()), position: dummy_pos() }, position: dummy_pos() },
        ],
        methods: vec![
            GraphMethod {
                name: "bark".to_string(),
                params: vec![],
                body: vec![],
                is_static: false,
                is_setter: false,
                is_private: false,
                guard: None,
                position: dummy_pos(),
            },
        ],
        rules: vec![
            GraphRule { name: "no_cycles".to_string(), param: None, position: dummy_pos() },
        ],
        config: std::collections::HashMap::new(),
        position: dummy_pos(),
    };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    let gd = elements[0];
    assert_eq!(graph.get_node(gd).unwrap().node_type, AstNodeType::GraphDeclStmt);
    assert_eq!(graph.get_node(gd).unwrap().properties.get("name"), Some(&AstProperty::Str("Dog".to_string())));

    // Properties
    let props = graph.get_ordered_edges(gd, "Property");
    assert_eq!(props.len(), 1);
    assert_eq!(graph.get_node(props[0]).unwrap().node_type, AstNodeType::GraphPropertyNode);
    assert_eq!(graph.get_node(props[0]).unwrap().properties.get("name"), Some(&AstProperty::Str("name".to_string())));

    // Methods
    let methods = graph.get_ordered_edges(gd, "GraphMethod");
    assert_eq!(methods.len(), 1);
    assert_eq!(graph.get_node(methods[0]).unwrap().node_type, AstNodeType::GraphMethodNode);

    // Rules
    let rules = graph.get_ordered_edges(gd, "Rule");
    assert_eq!(rules.len(), 1);
    assert_eq!(graph.get_node(rules[0]).unwrap().node_type, AstNodeType::GraphRuleNode);
}

// --- Load ---

#[test]
fn test_convert_load() {
    let stmt = Stmt::Load {
        path: Expr::Literal { value: LiteralValue::String("utils.gr".to_string()), position: dummy_pos() },
        position: dummy_pos(),
    };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    assert_eq!(graph.get_node(elements[0]).unwrap().node_type, AstNodeType::LoadStmt);
    assert!(graph.get_edge_target(elements[0], &ExecEdgeType::ValueEdge).is_some());
}

// --- ModuleDecl ---

#[test]
fn test_convert_module_decl() {
    let stmt = Stmt::ModuleDecl {
        name: "mymod".to_string(),
        alias: Some("m".to_string()),
        position: dummy_pos(),
    };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    let md = graph.get_node(elements[0]).unwrap();
    assert_eq!(md.node_type, AstNodeType::ModuleDeclStmt);
    assert_eq!(md.properties.get("name"), Some(&AstProperty::Str("mymod".to_string())));
    assert_eq!(md.properties.get("alias"), Some(&AstProperty::Str("m".to_string())));
}

// --- Full program round-trip ---

#[test]
fn test_full_program_round_trip() {
    use graphoid::lexer::Lexer;
    use graphoid::parser::Parser;

    let source = r#"
x = 5
y = x + 3
if y > 7 {
    z = "big"
} else {
    z = "small"
}
"#;
    let tokens = Lexer::new(source).tokenize().unwrap();
    let program = Parser::new(tokens).parse().unwrap();

    let mut converter = AstToGraphConverter::new();
    let root = converter.convert_program(&program);
    let graph = converter.into_graph();

    let root_node = graph.get_node(root).unwrap();
    assert_eq!(root_node.node_type, AstNodeType::Program);

    let elements = graph.get_ordered_edges(root, "Element");
    assert_eq!(elements.len(), 3); // x=5, y=x+3, if/else

    // Parser may produce VarDeclStmt or AssignStmt depending on context
    let n0 = graph.get_node(elements[0]).unwrap().node_type.clone();
    let n1 = graph.get_node(elements[1]).unwrap().node_type.clone();
    let n2 = graph.get_node(elements[2]).unwrap().node_type.clone();
    assert!(n0 == AstNodeType::VarDeclStmt || n0 == AstNodeType::AssignStmt);
    assert!(n1 == AstNodeType::VarDeclStmt || n1 == AstNodeType::AssignStmt);
    assert_eq!(n2, AstNodeType::IfStmt);
}

// --- Verify function gets own arena ---

#[test]
fn test_function_gets_own_arena() {
    let stmt = Stmt::FunctionDecl {
        name: "f".to_string(),
        receiver: None,
        params: vec![],
        body: vec![Stmt::Return { value: Some(num_expr(1.0)), position: dummy_pos() }],
        pattern_clauses: None,
        is_private: false,
        is_setter: false,
        is_static: false,
        guard: None,
        position: dummy_pos(),
    };
    let (graph, root) = convert_program(vec![stmt]);
    let elements = graph.get_ordered_edges(root, "Element");
    let func_node = elements[0];

    // The body should be in a different arena than the function node
    let body_ref = graph.get_edge_target(func_node, &ExecEdgeType::Body).unwrap();
    assert_ne!(func_node.arena_id, body_ref.arena_id);
}

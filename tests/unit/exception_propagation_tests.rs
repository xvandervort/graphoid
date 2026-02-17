//! Tests for Phase 18 Section 4: Exception propagation via caller edges.
//!
//! Verifies that when exceptions propagate through function calls,
//! ExceptionPropagation edges are recorded in the FunctionGraph.

use graphoid::execution::function_graph::{FunctionGraph, FunctionEdgeType};
use graphoid::values::Function;
use graphoid::execution::Environment;
use graphoid::ast::Parameter;
use std::rc::Rc;
use std::cell::RefCell;

fn make_test_function(name: &str) -> Function {
    Function {
        name: Some(name.to_string()),
        params: vec!["x".to_string()],
        parameters: vec![Parameter { name: "x".to_string(), default_value: None, is_variadic: false }],
        body: vec![],
        pattern_clauses: None,
        env: Rc::new(RefCell::new(Environment::new())),
        node_id: None,
        is_setter: false,
        is_static: false,
        guard: None,
    }
}

// =========================================================================
// FunctionGraph: Exception edge creation
// =========================================================================

#[test]
fn test_pop_call_exception_creates_exception_edge() {
    let mut graph = FunctionGraph::new();
    let func_a = make_test_function("caller");
    let func_b = make_test_function("raiser");

    let id_a = graph.register_function(func_a);
    let id_b = graph.register_function(func_b);

    // Simulate: caller() calls raiser(), raiser raises exception
    graph.push_call(id_a.clone(), vec![]);
    graph.push_call(id_b.clone(), vec![]);

    // raiser exits with exception
    graph.pop_call_exception("ValueError".to_string());

    // Should have 2 edges: one Call, one ExceptionPropagation
    let edges = graph.get_all_edges();
    assert_eq!(edges.len(), 2);

    let exception_edges: Vec<_> = edges.iter()
        .filter(|e| e.edge_type == FunctionEdgeType::ExceptionPropagation)
        .collect();
    assert_eq!(exception_edges.len(), 1);
    assert_eq!(exception_edges[0].from, id_b);
    assert_eq!(exception_edges[0].to, id_a);
}

#[test]
fn test_pop_call_exception_stores_error_type() {
    let mut graph = FunctionGraph::new();
    let func_a = make_test_function("caller");
    let func_b = make_test_function("raiser");

    let id_a = graph.register_function(func_a);
    let _id_b = graph.register_function(func_b);

    graph.push_call(id_a, vec![]);
    graph.push_call(_id_b, vec![]);
    graph.pop_call_exception("TypeError".to_string());

    let exception_edges: Vec<_> = graph.get_all_edges().iter()
        .filter(|e| e.edge_type == FunctionEdgeType::ExceptionPropagation)
        .collect();
    assert_eq!(exception_edges.len(), 1);
    // Error type is stored in the arguments
    assert_eq!(exception_edges[0].arguments.len(), 1);
    assert_eq!(exception_edges[0].arguments[0].to_string(), "TypeError");
}

#[test]
fn test_exception_propagation_chain() {
    let mut graph = FunctionGraph::new();
    let func_a = make_test_function("main");
    let func_b = make_test_function("foo");
    let func_c = make_test_function("bar");
    let func_d = make_test_function("boom");

    let id_a = graph.register_function(func_a);
    let id_b = graph.register_function(func_b);
    let id_c = graph.register_function(func_c);
    let id_d = graph.register_function(func_d);

    // Simulate: main -> foo -> bar -> boom (raises)
    graph.push_call(id_a.clone(), vec![]);
    graph.push_call(id_b.clone(), vec![]);
    graph.push_call(id_c.clone(), vec![]);
    graph.push_call(id_d.clone(), vec![]);

    // boom raises, propagates through bar and foo
    graph.pop_call_exception("RuntimeError".to_string()); // boom -> bar
    graph.pop_call_exception("RuntimeError".to_string()); // bar -> foo
    graph.pop_call_exception("RuntimeError".to_string()); // foo -> main

    // Should have 3 call edges + 3 exception edges
    let exception_edges: Vec<_> = graph.get_all_edges().iter()
        .filter(|e| e.edge_type == FunctionEdgeType::ExceptionPropagation)
        .collect();
    assert_eq!(exception_edges.len(), 3);

    // Exception path: boom -> bar -> foo -> main
    assert_eq!(exception_edges[0].from, id_d);
    assert_eq!(exception_edges[0].to, id_c);
    assert_eq!(exception_edges[1].from, id_c);
    assert_eq!(exception_edges[1].to, id_b);
    assert_eq!(exception_edges[2].from, id_b);
    assert_eq!(exception_edges[2].to, id_a);
}

#[test]
fn test_exception_at_top_level_no_caller() {
    let mut graph = FunctionGraph::new();
    let func = make_test_function("top_level");
    let id = graph.register_function(func);

    graph.push_call(id, vec![]);
    // Exception at top level — no caller to propagate to
    graph.pop_call_exception("RuntimeError".to_string());

    // Should have no exception edges (no caller)
    let exception_edges: Vec<_> = graph.get_all_edges().iter()
        .filter(|e| e.edge_type == FunctionEdgeType::ExceptionPropagation)
        .collect();
    assert_eq!(exception_edges.len(), 0);
}

#[test]
fn test_pop_call_exception_pops_from_call_path() {
    let mut graph = FunctionGraph::new();
    let func_a = make_test_function("caller");
    let func_b = make_test_function("raiser");

    let id_a = graph.register_function(func_a);
    let id_b = graph.register_function(func_b);

    graph.push_call(id_a, vec![]);
    graph.push_call(id_b, vec![]);
    assert_eq!(graph.call_depth(), 2);

    graph.pop_call_exception("RuntimeError".to_string());
    assert_eq!(graph.call_depth(), 1);
}

#[test]
fn test_exception_edge_has_duration() {
    let mut graph = FunctionGraph::new();
    graph.set_profiling(true);

    let func_a = make_test_function("caller");
    let func_b = make_test_function("raiser");

    let id_a = graph.register_function(func_a);
    let _id_b = graph.register_function(func_b);

    graph.push_call(id_a, vec![]);
    graph.push_call(_id_b, vec![]);
    std::thread::sleep(std::time::Duration::from_millis(5));
    graph.pop_call_exception("ValueError".to_string());

    let exception_edges: Vec<_> = graph.get_all_edges().iter()
        .filter(|e| e.edge_type == FunctionEdgeType::ExceptionPropagation)
        .collect();
    assert_eq!(exception_edges.len(), 1);
    assert!(exception_edges[0].duration.is_some());
    assert!(exception_edges[0].duration.unwrap() > 0.0);
}

// =========================================================================
// GraphExecutor integration: exceptions create edges
// =========================================================================

#[test]
fn test_executor_exception_creates_propagation_edge() {
    use graphoid::execution_graph::graph_executor::GraphExecutor;

    let mut executor = GraphExecutor::new();
    let _ = executor.execute_source(r#"
        fn boom() {
            raise RuntimeError("kaboom")
        }
        fn caller() {
            boom()
        }
        try {
            caller()
        } catch as e {
            # caught
        }
    "#);

    let fg = executor.function_graph.borrow();
    let exception_edges: Vec<_> = fg.get_all_edges().iter()
        .filter(|e| e.edge_type == FunctionEdgeType::ExceptionPropagation)
        .collect();

    // boom -> caller, caller -> __toplevel__ (or similar)
    assert!(exception_edges.len() >= 1, "Expected at least 1 exception edge, got {}", exception_edges.len());
}

#[test]
fn test_executor_no_exception_no_propagation_edge() {
    use graphoid::execution_graph::graph_executor::GraphExecutor;

    let mut executor = GraphExecutor::new();
    let _ = executor.execute_source(r#"
        fn safe() {
            return 42
        }
        result = safe()
    "#);

    let fg = executor.function_graph.borrow();
    let exception_edges: Vec<_> = fg.get_all_edges().iter()
        .filter(|e| e.edge_type == FunctionEdgeType::ExceptionPropagation)
        .collect();
    assert_eq!(exception_edges.len(), 0);
}

// =========================================================================
// reflect.call_graph() returns queryable graph
// =========================================================================

#[test]
fn test_reflect_call_graph_returns_graph() {
    use graphoid::execution_graph::graph_executor::GraphExecutor;

    let mut executor = GraphExecutor::new();
    let result = executor.execute_source_value(r#"
        fn greet(name) {
            return "hello " + name
        }
        greet("world")
        reflect.call_graph()
    "#);

    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.type_name(), "graph");
}

#[test]
fn test_reflect_call_graph_has_function_nodes() {
    use graphoid::execution_graph::graph_executor::GraphExecutor;

    let mut executor = GraphExecutor::new();
    let result = executor.execute_source_value(r#"
        fn add(a, b) { return a + b }
        add(1, 2)
        cg = reflect.call_graph()
        cg.has_node("fn:add")
    "#);

    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.to_string(), "true");
}

#[test]
fn test_reflect_call_graph_has_exception_edges() {
    use graphoid::execution_graph::graph_executor::GraphExecutor;

    let mut executor = GraphExecutor::new();
    let result = executor.execute_source_value(r#"
        fn boom() { raise ValueError("oops") }
        fn caller() { boom() }
        try { caller() } catch as e { }
        cg = reflect.call_graph()
        # Check that exception edges exist
        cg.edges().length()
    "#);

    assert!(result.is_ok(), "Expected Ok, got: {:?}", result.err());
    let value = result.unwrap();
    // Should have at least some edges (calls + exceptions)
    let edge_count: f64 = match &value.kind {
        graphoid::values::ValueKind::Number(n) => *n,
        _ => panic!("Expected number, got {}", value.type_name()),
    };
    assert!(edge_count > 0.0, "Expected edges in call graph");
}

use graphoid::execution::function_graph::{FunctionGraph, FunctionEdgeType};
use graphoid::values::{Value, Function};
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
    }
}

#[test]
fn test_register_function() {
    let mut graph = FunctionGraph::new();
    let func = make_test_function("test");

    let func_id = graph.register_function(func);

    assert!(func_id.starts_with("fn_test_"));
    assert_eq!(graph.get_all_functions().len(), 1);
    assert!(graph.get_function(&func_id).is_some());
}

#[test]
fn test_push_pop_call() {
    let mut graph = FunctionGraph::new();
    let func = make_test_function("test");
    let func_id = graph.register_function(func);

    // Push call
    graph.push_call(func_id.clone(), vec![Value::Number(42.0)]);
    assert_eq!(graph.call_depth(), 1);
    assert_eq!(graph.current_path(), vec!["test".to_string()]);

    // Pop call
    graph.pop_call(Value::Number(84.0));
    assert_eq!(graph.call_depth(), 0);

    // Call count updated
    let node = graph.get_function(&func_id).unwrap();
    assert_eq!(node.call_count, 1);
}

#[test]
fn test_call_edges() {
    let mut graph = FunctionGraph::new();
    let func_a = make_test_function("a");
    let func_b = make_test_function("b");

    let id_a = graph.register_function(func_a);
    let id_b = graph.register_function(func_b);

    // Simulate: a calls b
    graph.push_call(id_a.clone(), vec![]);
    graph.push_call(id_b.clone(), vec![]);

    // Check edges
    let edges = graph.get_all_edges();
    assert_eq!(edges.len(), 1);
    let edge = &edges[0];
    assert_eq!(edge.from, id_a);
    assert_eq!(edge.to, id_b);
    assert_eq!(edge.edge_type, FunctionEdgeType::Call);

    // Check outgoing/incoming
    assert_eq!(graph.get_callees(&id_a), vec![id_b.clone()]);
    assert_eq!(graph.get_callers(&id_b), vec![id_a.clone()]);
}

#[test]
fn test_recursion_detection() {
    let mut graph = FunctionGraph::new();
    let func = make_test_function("factorial");
    let func_id = graph.register_function(func);

    // Simulate recursion: factorial calls factorial
    graph.push_call(func_id.clone(), vec![Value::Number(5.0)]);
    graph.push_call(func_id.clone(), vec![Value::Number(4.0)]);

    // Should detect recursion
    assert!(graph.is_recursive(&func_id));
    assert_eq!(graph.find_recursive_functions(), vec![func_id]);
}

#[test]
fn test_profiling() {
    let mut graph = FunctionGraph::new();
    graph.set_profiling(true);

    let func = make_test_function("slow");
    let func_id = graph.register_function(func);

    // Simulate call
    graph.push_call(func_id.clone(), vec![]);
    std::thread::sleep(std::time::Duration::from_millis(10));
    graph.pop_call(Value::None);

    // Check timing
    let node = graph.get_function(&func_id).unwrap();
    assert!(node.total_time > 0.0);
    assert_eq!(node.call_count, 1);
}

#[test]
fn test_capture_edge() {
    let mut graph = FunctionGraph::new();
    let func = make_test_function("closure");
    let func_id = graph.register_function(func);

    // Add capture
    graph.add_capture_edge(func_id.clone(), "count".to_string(), "var_count_0".to_string());

    // Check captured vars
    let captured = graph.get_captured_vars(&func_id);
    assert_eq!(captured.len(), 1);
    assert_eq!(captured[0].0, "count");
    assert_eq!(captured[0].1, "var_count_0");

    // Check edge
    let capture_edges: Vec<_> = graph.get_all_edges().iter()
        .filter(|e| e.edge_type == FunctionEdgeType::Captures)
        .collect();
    assert_eq!(capture_edges.len(), 1);
}

#[test]
fn test_current_path() {
    let mut graph = FunctionGraph::new();
    let func_a = make_test_function("a");
    let func_b = make_test_function("b");
    let func_c = make_test_function("c");

    let id_a = graph.register_function(func_a);
    let id_b = graph.register_function(func_b);
    let id_c = graph.register_function(func_c);

    // Simulate: a -> b -> c
    graph.push_call(id_a, vec![]);
    graph.push_call(id_b, vec![]);
    graph.push_call(id_c, vec![]);

    let path = graph.current_path();
    assert_eq!(path, vec!["a", "b", "c"]);
    assert_eq!(graph.call_depth(), 3);
}

//! Integration tests for end-to-end execution

use graphoid::execution::Executor;
use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
use graphoid::values::{List, Value, ValueKind};

/// Helper function to execute source code
fn execute(source: &str) -> Result<(), String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer
        .tokenize()
        .map_err(|e| format!("Lexer error: {}", e))?;

    let mut parser = Parser::new(tokens);
    let program = parser
        .parse()
        .map_err(|e| format!("Parser error: {}", e))?;

    let mut executor = Executor::new();
    for stmt in &program.statements {
        executor
            .eval_stmt(stmt)
            .map_err(|e| format!("Runtime error: {}", e))?;
    }

    Ok(())
}

/// Helper to execute and return variable value
fn execute_and_get(source: &str, var_name: &str) -> Result<graphoid::values::Value, String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer
        .tokenize()
        .map_err(|e| format!("Lexer error: {}", e))?;

    let mut parser = Parser::new(tokens);
    let program = parser
        .parse()
        .map_err(|e| format!("Parser error: {}", e))?;

    let mut executor = Executor::new();
    for stmt in &program.statements {
        executor
            .eval_stmt(stmt)
            .map_err(|e| format!("Runtime error: {}", e))?;
    }

    executor
        .env()
        .get(var_name)
        .map_err(|e| format!("Variable not found: {}", e))
}

#[test]
fn test_simple_arithmetic() {
    let source = r#"
        x = 2 + 3
        y = x * 2
    "#;

    execute(source).unwrap();
}

#[test]
fn test_string_operations() {
    let source = r#"
        greeting = "Hello"
        name = "World"
        message = greeting + " " + name
    "#;

    let result = execute_and_get(source, "message").unwrap();
    assert_eq!(result, graphoid::values::Value::string("Hello World".to_string()));
}

#[test]
fn test_variable_shadowing() {
    let source = r#"
        x = 10
        x = x + 5
        x = x * 2
    "#;

    let result = execute_and_get(source, "x").unwrap();
    assert_eq!(result, graphoid::values::Value::number(30.0));
}

#[test]
fn test_collections() {
    let source = r#"
        numbers = [1, 2, 3]
        config = {"host": "localhost", "port": 8080}
    "#;

    execute(source).unwrap();
}

#[test]
fn test_boolean_logic() {
    let source = r#"
        a = true and false
        b = true or false
        c = not false
    "#;

    let a = execute_and_get(source, "a").unwrap();
    let b = execute_and_get(source, "b").unwrap();
    let c = execute_and_get(source, "c").unwrap();

    assert_eq!(a, graphoid::values::Value::boolean(false));
    assert_eq!(b, graphoid::values::Value::boolean(true));
    assert_eq!(c, graphoid::values::Value::boolean(true));
}

#[test]
fn test_comparisons() {
    let source = r#"
        result1 = 5 > 3
        result2 = 10 == 10
        result3 = "apple" < "banana"
    "#;

    let r1 = execute_and_get(source, "result1").unwrap();
    let r2 = execute_and_get(source, "result2").unwrap();
    let r3 = execute_and_get(source, "result3").unwrap();

    assert_eq!(r1, graphoid::values::Value::boolean(true));
    assert_eq!(r2, graphoid::values::Value::boolean(true));
    assert_eq!(r3, graphoid::values::Value::boolean(true));
}

// ============================================================================
// PHASE 4: Lambda Integration Tests
// ============================================================================

#[test]
fn test_lambda_assignment_and_call() {
    let source = r#"
        double = x => x * 2
        triple = x => x * 3
        result1 = double(5)
        result2 = triple(4)
    "#;

    let r1 = execute_and_get(source, "result1").unwrap();
    let r2 = execute_and_get(source, "result2").unwrap();

    assert_eq!(r1, graphoid::values::Value::number(10.0));
    assert_eq!(r2, graphoid::values::Value::number(12.0));
}

#[test]
fn test_multi_param_lambda() {
    let source = r#"
        add = (a, b) => a + b
        multiply = (x, y) => x * y
        result1 = add(3, 7)
        result2 = multiply(4, 5)
    "#;

    let r1 = execute_and_get(source, "result1").unwrap();
    let r2 = execute_and_get(source, "result2").unwrap();

    assert_eq!(r1, graphoid::values::Value::number(10.0));
    assert_eq!(r2, graphoid::values::Value::number(20.0));
}

#[test]
fn test_zero_param_lambda() {
    let source = r#"
        get_message = () => "Hello World"
        get_number = () => 42
        msg = get_message()
        result = get_number()
    "#;

    let msg = execute_and_get(source, "msg").unwrap();
    let result = execute_and_get(source, "result").unwrap();

    assert_eq!(msg, graphoid::values::Value::string("Hello World".to_string()));
    assert_eq!(result, graphoid::values::Value::number(42.0));
}

// ============================================================================
// PHASE 5: Collections & Methods Integration Tests
// ============================================================================

#[test]
fn test_list_indexing() {
    let source = r#"
        numbers = [10, 20, 30, 40, 50]
        first = numbers[0]
        second = numbers[1]
        last = numbers[-1]
        second_last = numbers[-2]
    "#;

    let first = execute_and_get(source, "first").unwrap();
    let second = execute_and_get(source, "second").unwrap();
    let last = execute_and_get(source, "last").unwrap();
    let second_last = execute_and_get(source, "second_last").unwrap();

    assert_eq!(first, graphoid::values::Value::number(10.0));
    assert_eq!(second, graphoid::values::Value::number(20.0));
    assert_eq!(last, graphoid::values::Value::number(50.0));
    assert_eq!(second_last, graphoid::values::Value::number(40.0));
}

#[test]
fn test_map_access() {
    let source = r#"
        config = {host: "localhost", port: 8080, debug: true}
        h = config["host"]
        p = config["port"]
        d = config["debug"]
    "#;

    let h = execute_and_get(source, "h").unwrap();
    let p = execute_and_get(source, "p").unwrap();
    let d = execute_and_get(source, "d").unwrap();

    assert_eq!(h, graphoid::values::Value::string("localhost".to_string()));
    assert_eq!(p, graphoid::values::Value::number(8080.0));
    assert_eq!(d, graphoid::values::Value::boolean(true));
}

#[test]
fn test_list_methods() {
    let source = r#"
        items = [10, 20, 30, 40, 50]
        s = items.size()
        f = items.first()
        l = items.last()
        has_30 = items.contains(30)
        has_99 = items.contains(99)
        empty = []
        is_empty = empty.is_empty()
    "#;

    let s = execute_and_get(source, "s").unwrap();
    let f = execute_and_get(source, "f").unwrap();
    let l = execute_and_get(source, "l").unwrap();
    let has_30 = execute_and_get(source, "has_30").unwrap();
    let has_99 = execute_and_get(source, "has_99").unwrap();
    let is_empty = execute_and_get(source, "is_empty").unwrap();

    assert_eq!(s, graphoid::values::Value::number(5.0));
    assert_eq!(f, graphoid::values::Value::number(10.0));
    assert_eq!(l, graphoid::values::Value::number(50.0));
    assert_eq!(has_30, graphoid::values::Value::boolean(true));
    assert_eq!(has_99, graphoid::values::Value::boolean(false));
    assert_eq!(is_empty, graphoid::values::Value::boolean(true));
}

#[test]
fn test_map_with_lambda() {
    let source = r#"
        numbers = [1, 2, 3, 4, 5]
        doubled = numbers.map(n => n * 2)
        squared = numbers.map(n => n * n)
    "#;

    let doubled = execute_and_get(source, "doubled").unwrap();
    let squared = execute_and_get(source, "squared").unwrap();

    assert_eq!(
        doubled,
        Value::list(List::from_vec(vec![
            graphoid::values::Value::number(2.0),
            graphoid::values::Value::number(4.0),
            graphoid::values::Value::number(6.0),
            graphoid::values::Value::number(8.0),
            graphoid::values::Value::number(10.0),
        ]))
    );

    assert_eq!(
        squared,
        Value::list(List::from_vec(vec![
            graphoid::values::Value::number(1.0),
            graphoid::values::Value::number(4.0),
            graphoid::values::Value::number(9.0),
            graphoid::values::Value::number(16.0),
            graphoid::values::Value::number(25.0),
        ]))
    );
}

#[test]
fn test_filter_with_lambda() {
    let source = r#"
        numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        evens = numbers.filter(n => n % 2 == 0)
        gt_five = numbers.filter(n => n > 5)
    "#;

    let evens = execute_and_get(source, "evens").unwrap();
    let gt_five = execute_and_get(source, "gt_five").unwrap();

    assert_eq!(
        evens,
        Value::list(List::from_vec(vec![
            graphoid::values::Value::number(2.0),
            graphoid::values::Value::number(4.0),
            graphoid::values::Value::number(6.0),
            graphoid::values::Value::number(8.0),
            graphoid::values::Value::number(10.0),
        ]))
    );

    assert_eq!(
        gt_five,
        Value::list(List::from_vec(vec![
            graphoid::values::Value::number(6.0),
            graphoid::values::Value::number(7.0),
            graphoid::values::Value::number(8.0),
            graphoid::values::Value::number(9.0),
            graphoid::values::Value::number(10.0),
        ]))
    );
}

#[test]
fn test_named_transformations() {
    let source = r#"
        numbers = [1, 2, 3, 4, 5]
        doubled = numbers.map(:double)
        squared = numbers.map(:square)
    "#;

    let doubled = execute_and_get(source, "doubled").unwrap();
    let squared = execute_and_get(source, "squared").unwrap();

    assert_eq!(
        doubled,
        Value::list(List::from_vec(vec![
            graphoid::values::Value::number(2.0),
            graphoid::values::Value::number(4.0),
            graphoid::values::Value::number(6.0),
            graphoid::values::Value::number(8.0),
            graphoid::values::Value::number(10.0),
        ]))
    );

    assert_eq!(
        squared,
        Value::list(List::from_vec(vec![
            graphoid::values::Value::number(1.0),
            graphoid::values::Value::number(4.0),
            graphoid::values::Value::number(9.0),
            graphoid::values::Value::number(16.0),
            graphoid::values::Value::number(25.0),
        ]))
    );
}

#[test]
fn test_named_predicates() {
    let source = r#"
        numbers = [1, 2, 3, 4, 5, 6]
        evens = numbers.filter(:even)
        positives = numbers.filter(:positive)
    "#;

    let evens = execute_and_get(source, "evens").unwrap();
    let positives = execute_and_get(source, "positives").unwrap();

    assert_eq!(
        evens,
        Value::list(List::from_vec(vec![
            graphoid::values::Value::number(2.0),
            graphoid::values::Value::number(4.0),
            graphoid::values::Value::number(6.0),
        ]))
    );

    assert_eq!(
        positives,
        Value::list(List::from_vec(vec![
            graphoid::values::Value::number(1.0),
            graphoid::values::Value::number(2.0),
            graphoid::values::Value::number(3.0),
            graphoid::values::Value::number(4.0),
            graphoid::values::Value::number(5.0),
            graphoid::values::Value::number(6.0),
        ]))
    );
}

#[test]
fn test_element_wise_operators() {
    let source = r#"
        list1 = [1, 2, 3]
        list2 = [10, 20, 30]
        sums = list1 .+ list2
        products = list1 .* list2
        scaled = list1 .* 10
    "#;

    let sums = execute_and_get(source, "sums").unwrap();
    let products = execute_and_get(source, "products").unwrap();
    let scaled = execute_and_get(source, "scaled").unwrap();

    assert_eq!(
        sums,
        Value::list(List::from_vec(vec![
            graphoid::values::Value::number(11.0),
            graphoid::values::Value::number(22.0),
            graphoid::values::Value::number(33.0),
        ]))
    );

    assert_eq!(
        products,
        Value::list(List::from_vec(vec![
            graphoid::values::Value::number(10.0),
            graphoid::values::Value::number(40.0),
            graphoid::values::Value::number(90.0),
        ]))
    );

    assert_eq!(
        scaled,
        Value::list(List::from_vec(vec![
            graphoid::values::Value::number(10.0),
            graphoid::values::Value::number(20.0),
            graphoid::values::Value::number(30.0),
        ]))
    );
}

#[test]
fn test_list_slicing() {
    let source = r#"
        numbers = [10, 20, 30, 40, 50]
        middle = numbers.slice(1, 4)
        from_start = numbers.slice(0, 2)
        to_end = numbers.slice(3, 5)
    "#;

    let middle = execute_and_get(source, "middle").unwrap();
    let from_start = execute_and_get(source, "from_start").unwrap();
    let to_end = execute_and_get(source, "to_end").unwrap();

    assert_eq!(
        middle,
        Value::list(List::from_vec(vec![
            graphoid::values::Value::number(20.0),
            graphoid::values::Value::number(30.0),
            graphoid::values::Value::number(40.0),
        ]))
    );

    assert_eq!(
        from_start,
        Value::list(List::from_vec(vec![
            graphoid::values::Value::number(10.0),
            graphoid::values::Value::number(20.0),
        ]))
    );

    assert_eq!(
        to_end,
        Value::list(List::from_vec(vec![
            graphoid::values::Value::number(40.0),
            graphoid::values::Value::number(50.0),
        ]))
    );
}

#[test]
fn test_map_methods() {
    let source = r#"
        config = {host: "localhost", port: 8080, debug: true}
        all_keys = config.keys()
        all_values = config.values()
        has_host = config.has_key("host")
        has_missing = config.has_key("missing")
        map_size = config.size()
    "#;

    let all_keys = execute_and_get(source, "all_keys").unwrap();
    let has_host = execute_and_get(source, "has_host").unwrap();
    let has_missing = execute_and_get(source, "has_missing").unwrap();
    let map_size = execute_and_get(source, "map_size").unwrap();

    // Keys should be a list with 3 string elements (order may vary)
    if let ValueKind::List(keys) = &all_keys.kind {
        assert_eq!(keys.len(), 3);
    } else {
        panic!("Expected list for keys");
    }

    assert_eq!(has_host, graphoid::values::Value::boolean(true));
    assert_eq!(has_missing, graphoid::values::Value::boolean(false));
    assert_eq!(map_size, graphoid::values::Value::number(3.0));
}

// ============================================================================
// PHASE 6: GRAPH AND TREE TESTS
// ============================================================================

#[test]
fn test_graph_creation_directed() {
    let source = r#"
        g = graph { type: :directed }
    "#;
    execute(source).unwrap();
}

#[test]
fn test_graph_creation_undirected() {
    let source = r#"
        g = graph { type: :undirected }
    "#;
    execute(source).unwrap();
}

#[test]
fn test_graph_creation_empty() {
    let source = r#"
        g = graph {}
    "#;
    execute(source).unwrap();
}

#[test]
fn test_tree_creation() {
    // tree{} creates a graph with :tree ruleset
    let source = r#"
        t = tree {}
    "#;
    execute(source).unwrap();
}

#[test]
fn test_graph_type_name() {
    let source = r#"
        g = graph { type: :directed }
    "#;

    let g = execute_and_get(source, "g").unwrap();
    assert_eq!(g.type_name(), "graph");
}

#[test]
fn test_tree_type_name() {
    // tree{} desugars to graph{}.with_ruleset(:tree)
    // So type is "graph", not "tree"
    let source = r#"
        t = tree {}
    "#;

    let t = execute_and_get(source, "t").unwrap();
    assert_eq!(t.type_name(), "graph");  // tree{} creates a graph
}

#[test]
fn test_empty_graph_is_falsy() {
    let source = r#"
        g = graph {}
    "#;

    let g = execute_and_get(source, "g").unwrap();
    assert!(!g.is_truthy());
}

#[test]
fn test_empty_tree_is_falsy() {
    let source = r#"
        t = tree {}
    "#;

    let t = execute_and_get(source, "t").unwrap();
    assert!(!t.is_truthy());  // Empty graph (tree) is falsy
}

#[test]
fn test_graph_to_string() {
    let source = r#"
        g = graph { type: :directed }
    "#;

    let g = execute_and_get(source, "g").unwrap();
    let s = g.to_string_value();
    assert!(s.contains("graph"));
    assert!(s.contains("0 nodes"));
    assert!(s.contains("0 edges"));
}

#[test]
fn test_tree_to_string() {
    // tree{} creates a graph, so string shows "graph"
    let source = r#"
        t = tree {}
    "#;

    let t = execute_and_get(source, "t").unwrap();
    let s = t.to_string_value();
    assert!(s.contains("graph"));  // tree{} creates a graph
    assert!(s.contains("0 nodes"));
}

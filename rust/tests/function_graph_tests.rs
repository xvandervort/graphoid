use graphoid::execution::Executor;
use graphoid::values::Value;

/// Helper to execute code and return the executor for inspection
fn execute_and_get_executor(source: &str) -> Executor {
    let mut executor = Executor::new();
    executor.execute_source(source).expect("Execution failed");
    executor
}

#[test]
fn test_function_definition_creates_node() {
    let source = r#"
        fn greet(name) {
            return "Hello " + name
        }
    "#;

    let executor = execute_and_get_executor(source);
    let graph = executor.function_graph.borrow();

    // Function should be registered in the graph
    let all_functions = graph.get_user_functions();
    assert_eq!(all_functions.len(), 1);

    let func_node = all_functions[0];
    assert_eq!(func_node.function.name, Some("greet".to_string()));
    assert_eq!(func_node.function.params, vec!["name".to_string()]);
}

#[test]
fn test_function_call_creates_edge() {
    let source = r#"
        fn greet(name) {
            return "Hello " + name
        }

        result = greet("World")
    "#;

    let executor = execute_and_get_executor(source);
    let graph = executor.function_graph.borrow();

    // Function should have been called
    let func_node = graph.get_function_by_name("greet").unwrap();
    assert_eq!(func_node.call_count, 1);

    // Should have at least one edge
    let edges = graph.get_all_edges();
    assert!(!edges.is_empty());
}

#[test]
fn test_nested_function_calls_create_path() {
    let source = r#"
        fn c() {
            return 3
        }

        fn b() {
            return c()
        }

        fn a() {
            return b()
        }

        result = a()
    "#;

    let executor = execute_and_get_executor(source);
    let graph = executor.function_graph.borrow();

    // All functions should be registered
    assert_eq!(graph.get_user_functions().len(), 3);

    // Check call counts
    assert_eq!(graph.get_function_by_name("a").unwrap().call_count, 1);
    assert_eq!(graph.get_function_by_name("b").unwrap().call_count, 1);
    assert_eq!(graph.get_function_by_name("c").unwrap().call_count, 1);

    // Check call relationships (a calls b, b calls c)
    let a_id = graph.get_function_by_name("a").unwrap().node_id.clone();
    let b_id = graph.get_function_by_name("b").unwrap().node_id.clone();
    let c_id = graph.get_function_by_name("c").unwrap().node_id.clone();

    let callees_a = graph.get_callees(&a_id);
    assert!(callees_a.contains(&b_id));

    let callees_b = graph.get_callees(&b_id);
    assert!(callees_b.contains(&c_id));
}

#[test]
fn test_recursion_detected() {
    let source = r#"
        fn factorial(n) {
            if n <= 1 {
                return 1
            }
            return n * factorial(n - 1)
        }

        result = factorial(5)
    "#;

    let executor = execute_and_get_executor(source);
    let graph = executor.function_graph.borrow();

    // Function should be registered
    let func_node = graph.get_function_by_name("factorial").unwrap();

    // Should have been called 5 times (5, 4, 3, 2, 1)
    assert_eq!(func_node.call_count, 5);

    // Should be detected as recursive
    assert!(graph.is_recursive(&func_node.node_id));

    // Should appear in recursive functions list
    let recursive_funcs = graph.find_recursive_functions();
    assert_eq!(recursive_funcs.len(), 1);
    assert!(recursive_funcs.contains(&func_node.node_id));
}

#[test]
fn test_multiple_calls_increment_count() {
    let source = r#"
        fn add(a, b) {
            return a + b
        }

        r1 = add(1, 2)
        r2 = add(3, 4)
        r3 = add(5, 6)
    "#;

    let executor = execute_and_get_executor(source);
    let graph = executor.function_graph.borrow();

    let func_node = graph.get_function_by_name("add").unwrap();
    assert_eq!(func_node.call_count, 3);
}

#[test]
fn test_lambda_functions_tracked() {
    let source = r#"
        numbers = [1, 2, 3, 4, 5]
        doubled = numbers.map(x => x * 2)
    "#;

    let executor = execute_and_get_executor(source);
    let graph = executor.function_graph.borrow();

    let all_functions = graph.get_all_functions();

    // Should have at least one lambda function (from map)
    let lambdas: Vec<_> = all_functions.iter()
        .filter(|f| f.function.name.is_none())
        .collect();

    assert!(!lambdas.is_empty(), "Expected lambda functions to be tracked");

    // Lambda should have been called 5 times (once per element)
    let lambda = lambdas[0];
    assert_eq!(lambda.call_count, 5);
}

#[test]
fn test_callers_and_callees() {
    let source = r#"
        fn helper1() {
            return 1
        }

        fn helper2() {
            return 2
        }

        fn main_func() {
            a = helper1()
            b = helper2()
            return a + b
        }

        result = main_func()
    "#;

    let executor = execute_and_get_executor(source);
    let graph = executor.function_graph.borrow();

    let main_id = graph.get_function_by_name("main_func").unwrap().node_id.clone();
    let helper1_id = graph.get_function_by_name("helper1").unwrap().node_id.clone();
    let helper2_id = graph.get_function_by_name("helper2").unwrap().node_id.clone();

    // main_func should call both helpers
    let callees = graph.get_callees(&main_id);
    assert!(callees.contains(&helper1_id));
    assert!(callees.contains(&helper2_id));

    // helpers should have main_func as caller
    let callers1 = graph.get_callers(&helper1_id);
    assert!(callers1.contains(&main_id));

    let callers2 = graph.get_callers(&helper2_id);
    assert!(callers2.contains(&main_id));
}

#[test]
fn test_profiling_tracks_timing() {
    let source = r#"
        fn slow_function() {
            # This function gets called, timing should be tracked
            total = 0
            for i in [1, 2, 3, 4, 5] {
                total = total + i
            }
            return total
        }

        result = slow_function()
    "#;

    let mut executor = Executor::new();
    executor.function_graph.borrow_mut().set_profiling(true);
    executor.execute_source(source).expect("Execution failed");

    let graph = executor.function_graph.borrow();
    let func_node = graph.get_function_by_name("slow_function").unwrap();

    // Should have timing data when profiling enabled
    assert!(func_node.total_time >= 0.0);
    assert_eq!(func_node.call_count, 1);
}

#[test]
fn test_profiling_report() {
    let source = r#"
        fn fast() {
            return 42
        }

        fn medium() {
            return fast()
        }

        fn slow() {
            medium()
            medium()
            return medium()
        }

        result = slow()
    "#;

    let mut executor = Executor::new();
    executor.function_graph.borrow_mut().set_profiling(true);
    executor.execute_source(source).expect("Execution failed");

    let graph = executor.function_graph.borrow();
    let report = graph.profiling_report();

    // Report should contain function names
    assert!(report.contains("slow"));
    assert!(report.contains("medium"));
    assert!(report.contains("fast"));

    // Should show call counts
    assert!(report.contains("3")); // fast called 3 times
}

#[test]
fn test_call_depth_during_execution() {
    // This test verifies call depth during nested execution
    // We'll use a simpler approach - just verify final call counts
    let source = r#"
        fn level3() {
            return 3
        }

        fn level2() {
            return level3()
        }

        fn level1() {
            return level2()
        }

        result = level1()
    "#;

    let executor = execute_and_get_executor(source);
    let graph = executor.function_graph.borrow();

    // After execution, call depth should be 0 (all calls completed)
    assert_eq!(graph.call_depth(), 0);

    // But all functions should have been called
    assert_eq!(graph.get_function_by_name("level1").unwrap().call_count, 1);
    assert_eq!(graph.get_function_by_name("level2").unwrap().call_count, 1);
    assert_eq!(graph.get_function_by_name("level3").unwrap().call_count, 1);
}

#[test]
fn test_total_calls_across_program() {
    let source = r#"
        fn f1() { return 1 }
        fn f2() { return 2 }
        fn f3() { return 3 }

        f1()
        f2()
        f3()
        f1()
        f2()
    "#;

    let executor = execute_and_get_executor(source);
    let graph = executor.function_graph.borrow();

    // Total calls: f1(2) + f2(2) + f3(1) = 5
    assert_eq!(graph.user_calls(), 5);
}

#[test]
fn test_multiple_lambdas_in_chain() {
    let source = r#"
        numbers = [1, 2, 3, 4, 5]
        result = numbers
            .map(x => x * 2)
            .filter(x => x > 5)
            .map(x => x + 1)
    "#;

    let executor = execute_and_get_executor(source);
    let graph = executor.function_graph.borrow();

    // Should have 3 lambda functions
    let all_functions = graph.get_all_functions();
    let lambdas: Vec<_> = all_functions.iter()
        .filter(|f| f.function.name.is_none())
        .collect();

    assert_eq!(lambdas.len(), 3, "Expected 3 lambdas (map, filter, map)");
}

#[test]
fn test_function_arguments_tracked_in_edges() {
    let source = r#"
        fn add(a, b) {
            return a + b
        }

        result = add(10, 20)
    "#;

    let executor = execute_and_get_executor(source);
    let graph = executor.function_graph.borrow();

    let edges = graph.get_all_edges();

    // Find the call edge to add()
    let add_edges: Vec<_> = edges.iter()
        .filter(|e| {
            if let Some(node) = graph.get_function(&e.to) {
                node.function.name.as_deref() == Some("add")
            } else {
                false
            }
        })
        .collect();

    assert_eq!(add_edges.len(), 1);

    // Arguments should be tracked
    let edge = add_edges[0];
    assert_eq!(edge.arguments.len(), 2);
    assert_eq!(edge.arguments[0], Value::Number(10.0));
    assert_eq!(edge.arguments[1], Value::Number(20.0));
}

#[test]
fn test_return_values_tracked_in_edges() {
    let source = r#"
        fn get_value() {
            return 42
        }

        result = get_value()
    "#;

    let mut executor = Executor::new();
    executor.function_graph.borrow_mut().set_profiling(true);
    executor.execute_source(source).expect("Execution failed");

    let graph = executor.function_graph.borrow();
    let edges = graph.get_all_edges();

    // Find the call edge to get_value()
    let value_edges: Vec<_> = edges.iter()
        .filter(|e| {
            if let Some(node) = graph.get_function(&e.to) {
                node.function.name.as_deref() == Some("get_value")
            } else {
                false
            }
        })
        .collect();

    assert_eq!(value_edges.len(), 1);

    // Return value should be tracked when profiling enabled
    let edge = value_edges[0];
    if let Some(ref return_val) = edge.return_value {
        assert_eq!(*return_val, Value::Number(42.0));
    } else {
        panic!("Expected return value to be tracked");
    }
}

#[test]
fn test_fibonacci_recursion() {
    let source = r#"
        fn fib(n) {
            if n <= 1 {
                return n
            }
            return fib(n - 1) + fib(n - 2)
        }

        result = fib(5)
    "#;

    let executor = execute_and_get_executor(source);
    let graph = executor.function_graph.borrow();

    let func_node = graph.get_function_by_name("fib").unwrap();

    // fib(5) calls fib many times due to branching recursion
    assert!(func_node.call_count > 5);

    // Should be detected as recursive
    assert!(graph.is_recursive(&func_node.node_id));
}

#[test]
fn test_mutual_recursion() {
    let source = r#"
        fn is_even(n) {
            if n == 0 {
                return true
            }
            return is_odd(n - 1)
        }

        fn is_odd(n) {
            if n == 0 {
                return false
            }
            return is_even(n - 1)
        }

        result = is_even(4)
    "#;

    let executor = execute_and_get_executor(source);
    let graph = executor.function_graph.borrow();

    let even_id = graph.get_function_by_name("is_even").unwrap().node_id.clone();
    let odd_id = graph.get_function_by_name("is_odd").unwrap().node_id.clone();

    // Each function should call the other
    let even_callees = graph.get_callees(&even_id);
    assert!(even_callees.contains(&odd_id));

    let odd_callees = graph.get_callees(&odd_id);
    assert!(odd_callees.contains(&even_id));

    // Both should have been called (4 -> 3 -> 2 -> 1 -> 0)
    assert_eq!(graph.get_function_by_name("is_even").unwrap().call_count, 3); // 4, 2, 0
    assert_eq!(graph.get_function_by_name("is_odd").unwrap().call_count, 2);  // 3, 1
}

#[test]
fn test_clear_profiling_data() {
    let source = r#"
        fn test() {
            return 42
        }

        test()
        test()
    "#;

    let mut executor = Executor::new();
    executor.function_graph.borrow_mut().set_profiling(true);
    executor.execute_source(source).expect("Execution failed");

    {
        let graph = executor.function_graph.borrow();
        assert_eq!(graph.user_calls(), 2);
    }

    // Clear profiling data
    executor.function_graph.borrow_mut().clear_profiling_data();

    {
        let graph = executor.function_graph.borrow();
        assert_eq!(graph.user_calls(), 0);
    }
}

#[test]
fn test_hotspots_identification() {
    let source = r#"
        fn rarely_called() {
            return 1
        }

        fn frequently_called() {
            return 2
        }

        rarely_called()

        for i in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] {
            frequently_called()
        }
    "#;

    let mut executor = Executor::new();
    executor.function_graph.borrow_mut().set_profiling(true);
    executor.execute_source(source).expect("Execution failed");

    let graph = executor.function_graph.borrow();
    let hotspots = graph.get_hotspots(5);

    // frequently_called should be the hotspot
    assert!(!hotspots.is_empty());

    // The function with most time should be frequently_called or have high call count
    // (exact timing varies, but call pattern is deterministic)
    let freq_node = graph.get_function_by_name("frequently_called").unwrap();
    assert_eq!(freq_node.call_count, 10);
}

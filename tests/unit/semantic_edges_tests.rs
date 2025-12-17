//! TDD Tests for Semantic Edges in Graph-Theoretic Class-Like Graphs
//!
//! Phase 1: Method-Data Accessor Edges
//! - Methods should have edges to properties they read/write
//!
//! Phase 2: Property Dependency Edges
//! - Computed properties (getters) should have edges to their dependencies
//!
//! Phase 3: Inheritance as Graph Structure
//! - Parent graphs should be nodes with inherits_from edges

use graphoid::execution::Executor;
use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
use graphoid::values::{Value, ValueKind};

/// Helper to extract string from Value
fn get_string(v: &Value) -> Option<String> {
    if let ValueKind::String(s) = &v.kind {
        Some(s.clone())
    } else {
        None
    }
}

/// Helper function to execute source code and return the executor
fn execute_with_result(source: &str) -> Result<Executor, String> {
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

    Ok(executor)
}

// =============================================================================
// Phase 1: Method-Data Accessor Edges
// =============================================================================

#[test]
fn test_method_creates_reads_edge() {
    // A method that reads a property should create a "reads" edge
    let code = r#"
        graph Counter {
            count: 0

            fn get_count() {
                return count
            }
        }

        # Check that the method has a reads edge to count
        edges = Counter.edges(:all)
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let edges = exec.env().get("edges").expect("Should have edges");
    if let ValueKind::List(edge_list) = &edges.kind {
        let edges_vec = edge_list.to_vec();
        // Should find an edge from __methods__/get_count to count with type "reads"
        let has_reads_edge = edges_vec.iter().any(|edge| {
            if let ValueKind::List(edge_parts) = &edge.kind {
                let parts = edge_parts.to_vec();
                if parts.len() >= 3 {
                    let from = get_string(&parts[0]).unwrap_or_default();
                    let to = get_string(&parts[1]).unwrap_or_default();
                    let edge_type = get_string(&parts[2]).unwrap_or_default();
                    return from.contains("get_count") && to == "count" && edge_type == "reads";
                }
            }
            false
        });
        assert!(has_reads_edge, "Should have reads edge from method to property. Edges: {:?}", edges_vec);
    } else {
        panic!("edges should be a list");
    }
}

#[test]
fn test_method_creates_writes_edge() {
    // A method that writes to a property should create a "writes" edge
    let code = r#"
        graph Counter {
            count: 0

            fn increment() {
                count = count + 1
            }
        }

        edges = Counter.edges(:all)
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let edges = exec.env().get("edges").expect("Should have edges");
    if let ValueKind::List(edge_list) = &edges.kind {
        let edges_vec = edge_list.to_vec();
        // Should find a "writes" edge from increment to count
        let has_writes_edge = edges_vec.iter().any(|edge| {
            if let ValueKind::List(edge_parts) = &edge.kind {
                let parts = edge_parts.to_vec();
                if parts.len() >= 3 {
                    let from = get_string(&parts[0]).unwrap_or_default();
                    let to = get_string(&parts[1]).unwrap_or_default();
                    let edge_type = get_string(&parts[2]).unwrap_or_default();
                    return from.contains("increment") && to == "count" && edge_type == "writes";
                }
            }
            false
        });
        assert!(has_writes_edge, "Should have writes edge from method to property. Edges: {:?}", edges_vec);
    } else {
        panic!("edges should be a list");
    }
}

#[test]
fn test_method_reads_returns_properties() {
    // method_reads("name") should return list of properties the method reads
    let code = r#"
        graph Rectangle {
            width: 10
            height: 20

            fn area() {
                return width * height
            }
        }

        reads = Rectangle.method_reads("area")
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let reads = exec.env().get("reads").expect("Should have reads");
    if let ValueKind::List(read_list) = &reads.kind {
        let props: Vec<String> = read_list.to_vec().iter()
            .filter_map(|v| {
                if let ValueKind::String(s) = &v.kind {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .collect();
        assert!(props.contains(&"width".to_string()), "Should read width");
        assert!(props.contains(&"height".to_string()), "Should read height");
    } else {
        panic!("method_reads should return a list");
    }
}

#[test]
fn test_method_writes_returns_properties() {
    // method_writes("name") should return list of properties the method writes
    let code = r#"
        graph Counter {
            count: 0
            total: 0

            fn add(n) {
                count = count + n
                total = total + n
            }
        }

        writes = Counter.method_writes("add")
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let writes = exec.env().get("writes").expect("Should have writes");
    if let ValueKind::List(write_list) = &writes.kind {
        let props: Vec<String> = write_list.to_vec().iter()
            .filter_map(|v| {
                if let ValueKind::String(s) = &v.kind {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .collect();
        assert!(props.contains(&"count".to_string()), "Should write count");
        assert!(props.contains(&"total".to_string()), "Should write total");
    } else {
        panic!("method_writes should return a list");
    }
}

#[test]
fn test_property_readers_returns_methods() {
    // property_readers("name") returns methods that read this property
    let code = r#"
        graph Counter {
            count: 0

            fn get_count() {
                return count
            }

            fn double_count() {
                return count * 2
            }
        }

        readers = Counter.property_readers("count")
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let readers = exec.env().get("readers").expect("Should have readers");
    if let ValueKind::List(reader_list) = &readers.kind {
        let methods: Vec<String> = reader_list.to_vec().iter()
            .filter_map(|v| {
                if let ValueKind::String(s) = &v.kind {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .collect();
        assert!(methods.contains(&"get_count".to_string()), "get_count should read count");
        assert!(methods.contains(&"double_count".to_string()), "double_count should read count");
    } else {
        panic!("property_readers should return a list");
    }
}

#[test]
fn test_property_writers_returns_methods() {
    // property_writers("name") returns methods that write this property
    let code = r#"
        graph Counter {
            count: 0

            fn increment() {
                count = count + 1
            }

            fn reset() {
                count = 0
            }
        }

        writers = Counter.property_writers("count")
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let writers = exec.env().get("writers").expect("Should have writers");
    if let ValueKind::List(writer_list) = &writers.kind {
        let methods: Vec<String> = writer_list.to_vec().iter()
            .filter_map(|v| {
                if let ValueKind::String(s) = &v.kind {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .collect();
        assert!(methods.contains(&"increment".to_string()), "increment should write count");
        assert!(methods.contains(&"reset".to_string()), "reset should write count");
    } else {
        panic!("property_writers should return a list");
    }
}

#[test]
fn test_has_path_from_method_to_property() {
    // Graph traversal should work from methods to properties
    let code = r#"
        graph Counter {
            count: 0

            fn get_count() {
                return count
            }
        }

        has_path = Counter.has_path("__methods__/get_count", "count")
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let has_path = exec.env().get("has_path").expect("Should have has_path");
    assert!(matches!(has_path.kind, ValueKind::Boolean(true)),
            "Should have path from method to property it reads");
}

// =============================================================================
// Phase 3: Inheritance as Graph Structure
// =============================================================================

#[test]
fn test_inheritance_creates_parent_node() {
    // Inheriting from a parent should create a __parent__ node
    let code = r#"
        graph Animal {
            sound: "..."
        }

        graph Dog from Animal {
            sound: "woof"
        }

        has_parent = Dog.get_node("__parent__") != none
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let has_parent = exec.env().get("has_parent").expect("Should have has_parent");
    assert!(matches!(has_parent.kind, ValueKind::Boolean(true)),
            "Dog should have __parent__ node");
}

#[test]
fn test_inheritance_creates_inherits_from_edge() {
    // The graph should have an inherits_from edge
    let code = r#"
        graph Animal {
            sound: "..."
        }

        graph Dog from Animal {
            sound: "woof"
        }

        edges = Dog.edges(:all)
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let edges = exec.env().get("edges").expect("Should have edges");
    if let ValueKind::List(edge_list) = &edges.kind {
        let edges_vec = edge_list.to_vec();
        let has_inherit_edge = edges_vec.iter().any(|edge| {
            if let ValueKind::List(edge_parts) = &edge.kind {
                let parts = edge_parts.to_vec();
                if parts.len() >= 3 {
                    let edge_type = get_string(&parts[2]).unwrap_or_default();
                    return edge_type == "inherits_from";
                }
            }
            false
        });
        assert!(has_inherit_edge, "Should have inherits_from edge. Edges: {:?}", edges_vec);
    } else {
        panic!("edges should be a list");
    }
}

#[test]
fn test_ancestors_follows_inheritance_chain() {
    // ancestors() should follow the inheritance chain
    let code = r#"
        graph Animal {
            sound: "..."
        }

        graph Dog from Animal {
            sound: "woof"
        }

        graph Poodle from Dog {
            sound: "yip"
        }

        ancestors = Poodle.ancestors()
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let ancestors = exec.env().get("ancestors").expect("Should have ancestors");
    if let ValueKind::List(ancestor_list) = &ancestors.kind {
        let names: Vec<String> = ancestor_list.to_vec().iter()
            .filter_map(|v| {
                if let ValueKind::String(s) = &v.kind {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .collect();
        assert!(names.contains(&"Dog".to_string()), "Poodle should have Dog as ancestor");
        assert!(names.contains(&"Animal".to_string()), "Poodle should have Animal as ancestor");
    } else {
        panic!("ancestors should return a list");
    }
}

#[test]
fn test_has_path_works_with_inheritance() {
    // Graph traversal should work through inheritance edges
    let code = r#"
        graph Animal {
            sound: "..."
        }

        graph Dog from Animal {
            sound: "woof"
        }

        # Should be able to traverse from Dog to its parent
        has_path = Dog.has_path("Dog", "__parent__")
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let has_path = exec.env().get("has_path").expect("Should have has_path");
    assert!(matches!(has_path.kind, ValueKind::Boolean(true)),
            "Should have path from graph to parent through inheritance edge");
}

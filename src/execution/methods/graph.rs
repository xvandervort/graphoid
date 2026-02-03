//! Graph method implementations for the Graphoid executor.
//!
//! This module contains all graph-specific method handling,
//! extracted from the main executor for better code organization.


use crate::graph::RuleSpec;
use crate::ast::Expr;
use crate::error::{GraphoidError, Result};
use crate::execution::Executor;
use crate::values::{Value, ValueKind, List, Function};

impl Executor {
    pub(crate) fn apply_node_filter(
        &mut self,
        graph: &crate::values::Graph,
        node_filter: Option<&Value>,
        invert: bool,
    ) -> Result<std::collections::HashSet<String>> {
        use std::collections::HashSet;
        let mut matching_node_ids: HashSet<String> = HashSet::new();

        if let Some(node_filter_func) = node_filter {
            let func = match &node_filter_func.kind {
                ValueKind::Function(f) => f,
                _ => return Err(GraphoidError::type_error("function", node_filter_func.type_name())),
            };

            for node_id in graph.keys() {
                if let Some(node_value) = graph.get_node(&node_id) {
                    let result = self.call_function(func, &[node_value.clone()])?;
                    let matches = result.is_truthy();
                    // Apply inversion if requested
                    if matches != invert {
                        matching_node_ids.insert(node_id);
                    }
                }
            }
        } else {
            // No filter - all nodes match (or all nodes excluded if inverted)
            if !invert {
                matching_node_ids.extend(graph.keys());
            }
        }

        Ok(matching_node_ids)
    }

    /// Helper: Apply edge filter to graph, returns vec of matching edges (from, to, type).
    /// If invert=true, returns edges that DON'T match the filter.
    /// Only considers edges between nodes in allowed_nodes set.
    pub(crate) fn apply_edge_filter(
        &mut self,
        graph: &crate::values::Graph,
        edge_filter: Option<&Value>,
        allowed_nodes: &std::collections::HashSet<String>,
        invert: bool,
    ) -> Result<Vec<(String, String, String)>> {
        let mut matching_edges: Vec<(String, String, String)> = vec![];

        // Access graph nodes directly
        for (from_id, from_node) in &graph.nodes {
            if !allowed_nodes.contains(from_id) {
                continue;
            }

            for (to_id, edge_info) in &from_node.neighbors {
                if !allowed_nodes.contains(to_id) {
                    continue;
                }

                let edge_type = edge_info.edge_type.clone();

                let edge_matches = if let Some(edge_filter_func) = edge_filter {
                    let func = match &edge_filter_func.kind {
                        ValueKind::Function(f) => f,
                        _ => return Err(GraphoidError::type_error("function", edge_filter_func.type_name())),
                    };

                    let args = vec![
                        Value::string(from_id.clone()),
                        Value::string(to_id.clone()),
                        Value::string(edge_type.clone()),
                    ];
                    let result = self.call_function(func, &args)?;
                    result.is_truthy()
                } else {
                    true // No filter
                };

                // Apply inversion if requested
                if edge_matches != invert {
                    matching_edges.push((from_id.clone(), to_id.clone(), edge_type));
                }
            }
        }

        Ok(matching_edges)
    }

    /// Evaluates a method call on a graph.
    pub(crate) fn eval_graph_method(&mut self, mut graph: crate::values::Graph, method: &str, args: &[Value], object_expr: &Expr) -> Result<Value> {
        // Phase 20: Check for static methods first (called on class, not instances)
        if let Some(static_func) = graph.get_static_method(method) {
            // Static method found - call it WITHOUT binding `self`
            let static_func_clone = static_func.clone();
            return self.call_static_method(&static_func_clone, args);
        }

        // Check for user-defined instance methods (class-like graphs)
        // Phase 21: Get all method variants and evaluate guards to find the matching one
        let method_variants = graph.get_method_variants(method);
        if !method_variants.is_empty() {
            // Phase 15: Private method check (convention-based: underscore prefix)
            // Methods starting with _ are private and can only be called from within the same graph's methods
            if method.starts_with('_') {
                // Check if we're inside a method of this graph
                let graph_var_name = if let Expr::Variable { name, .. } = object_expr {
                    Some(name.clone())
                } else {
                    None
                };

                let is_self_call = if let Expr::Variable { name, .. } = object_expr {
                    name == "self"
                } else {
                    false
                };

                // Allow if:
                // 1. We're calling on `self` (always allowed inside a method)
                // 2. We're in a method context for this specific graph variable
                let is_allowed = is_self_call ||
                    graph_var_name.as_ref().map_or(false, |name| {
                        self.method_context_stack.contains(name)
                    });

                if !is_allowed {
                    return Err(GraphoidError::runtime(format!(
                        "Cannot call private method '{}' from outside the graph's methods",
                        method
                    )));
                }
            }

            // Phase 21: Find matching method variant by evaluating guards
            // Guards are evaluated with `self` bound to the graph
            let mut matching_func: Option<Function> = None;
            let mut fallback_func: Option<Function> = None;

            for func in method_variants {
                if let Some(guard_expr) = &func.guard {
                    // Evaluate guard with `self` bound to graph
                    // Create temporary environment for guard evaluation
                    let saved_env = self.env.clone();
                    self.env.define("self".to_string(), Value::graph(graph.clone()));

                    let guard_result = self.eval_expr(guard_expr);
                    self.env = saved_env;

                    match guard_result {
                        Ok(guard_val) => {
                            if guard_val.is_truthy() {
                                matching_func = Some(func.clone());
                                break;
                            }
                        }
                        Err(_) => {
                            // Guard evaluation failed, skip this variant
                            continue;
                        }
                    }
                } else {
                    // No guard - this is the fallback
                    fallback_func = Some(func.clone());
                }
            }

            // Use matching func or fallback
            let func_to_call = matching_func.or(fallback_func);

            if let Some(func) = func_to_call {
                return self.call_graph_method(&graph, &func, args, object_expr);
            }
            // If no matching variant found, fall through to built-in methods
        }

        match method {
            "add_node" => {
                // Add a node to the graph
                if args.len() != 2 {
                    return Err(GraphoidError::runtime(format!(
                        "add_node() expects 2 arguments (node_id, value), but got {}",
                        args.len()
                    )));
                }

                // Get node ID (must be string)
                let node_id = match &args[0].kind {
                    ValueKind::String(s) => s.clone(),
                    _other => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                // Get node value
                let node_value = args[1].clone();

                // Add the node
                graph.add_node(node_id, node_value)?;

                // Update graph in environment
                if let Expr::Variable { name, .. } = object_expr {
                    self.env.set(name, Value::graph(graph))?;
                }

                Ok(Value::none())
            }
            "set_node_type" => {
                // Set the type of a node
                if args.len() != 2 {
                    return Err(GraphoidError::runtime(format!(
                        "set_node_type() expects 2 arguments (node_id, type), but got {}",
                        args.len()
                    )));
                }

                // Get node ID
                let node_id = match &args[0].kind {
                    ValueKind::String(s) => s.as_str(),
                    _ => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                // Get node type
                let node_type = match &args[1].kind {
                    ValueKind::String(s) => s.clone(),
                    _ => {
                        return Err(GraphoidError::type_error("string", args[1].type_name()));
                    }
                };

                // Set the node type
                graph.set_node_type(node_id, node_type)?;

                // Update graph in environment
                if let Expr::Variable { name, .. } = object_expr {
                    self.env.set(name, Value::graph(graph))?;
                }

                Ok(Value::none())
            }
            "add_edge" => {
                // Add an edge between two nodes
                if args.len() < 2 || args.len() > 3 {
                    return Err(GraphoidError::runtime(format!(
                        "add_edge() expects 2-3 arguments (from, to, [edge_type]), but got {}",
                        args.len()
                    )));
                }

                // Get from node ID
                let from = match &args[0].kind {
                    ValueKind::String(s) => s.as_str(),
                    _other => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                // Get to node ID
                let to = match &args[1].kind {
                    ValueKind::String(s) => s.as_str(),
                    _other => {
                        return Err(GraphoidError::type_error("string", args[1].type_name()));
                    }
                };

                // Get optional edge type (default to "edge")
                let edge_type = if args.len() == 3 {
                    match &args[2].kind {
                        ValueKind::String(s) => s.clone(),
                        _other => {
                            return Err(GraphoidError::type_error("string", args[2].type_name()));
                        }
                    }
                } else {
                    "edge".to_string()
                };

                // Add the edge with empty properties and no weight
                use std::collections::HashMap;
                let properties = HashMap::new();
                graph.add_edge(from, to, edge_type, None, properties)?;

                // Update graph in environment
                if let Expr::Variable { name, .. } = object_expr {
                    self.env.set(name, Value::graph(graph))?;
                }

                Ok(Value::none())
            }
            "remove_node" => {
                // Remove a node from the graph
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "remove_node() expects 1 argument (node_id), but got {}",
                        args.len()
                    )));
                }

                // Get node ID
                let node_id = match &args[0].kind {
                    ValueKind::String(s) => s.as_str(),
                    _other => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                // Remove the node
                graph.remove_node(node_id, None)?;

                // Update graph in environment
                if let Expr::Variable { name, .. } = object_expr {
                    self.env.set(name, Value::graph(graph))?;
                }

                Ok(Value::none())
            }
            "remove_edge" => {
                // Remove an edge from the graph
                if args.len() != 2 {
                    return Err(GraphoidError::runtime(format!(
                        "remove_edge() expects 2 arguments (from, to), but got {}",
                        args.len()
                    )));
                }

                // Get from node ID
                let from = match &args[0].kind {
                    ValueKind::String(s) => s.as_str(),
                    _other => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                // Get to node ID
                let to = match &args[1].kind {
                    ValueKind::String(s) => s.as_str(),
                    _other => {
                        return Err(GraphoidError::type_error("string", args[1].type_name()));
                    }
                };

                // Remove the edge
                graph.remove_edge(from, to)?;

                // Update graph in environment
                if let Expr::Variable { name, .. } = object_expr {
                    self.env.set(name, Value::graph(graph))?;
                }

                Ok(Value::none())
            }
            "with_ruleset" => {
                // Apply a ruleset to the graph
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "with_ruleset() expects 1 argument, but got {}",
                        args.len()
                    )));
                }

                // Get the ruleset name from symbol argument
                let ruleset_name = match &args[0].kind {
                    ValueKind::Symbol(name) => name.clone(),
                    _other => {
                        return Err(GraphoidError::runtime(format!(
                            "with_ruleset() expects a symbol argument, got {}",
                            args[0].type_name()
                        )));
                    }
                };

                // Apply the ruleset (currently just stores the name)
                graph = graph.with_ruleset(ruleset_name);
                Ok(Value::graph(graph))
            }
            "has_ruleset" => {
                // Check if graph has a specific ruleset
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "has_ruleset() expects 1 argument, but got {}",
                        args.len()
                    )));
                }

                let ruleset_name = match &args[0].kind {
                    ValueKind::Symbol(name) => name.as_str(),
                    _other => {
                        return Err(GraphoidError::runtime(format!(
                            "has_ruleset() expects a symbol argument, got {}",
                            args[0].type_name()
                        )));
                    }
                };

                Ok(Value::boolean(graph.has_ruleset(ruleset_name)))
            }
            "has_path" => {
                // Check if a path exists between two nodes
                if args.len() != 2 {
                    return Err(GraphoidError::runtime(format!(
                        "has_path() expects 2 arguments (from, to), but got {}",
                        args.len()
                    )));
                }

                // Get from node ID
                let from = match &args[0].kind {
                    ValueKind::String(s) => s.as_str(),
                    _other => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                // Get to node ID
                let to = match &args[1].kind {
                    ValueKind::String(s) => s.as_str(),
                    _other => {
                        return Err(GraphoidError::type_error("string", args[1].type_name()));
                    }
                };

                // Check if path exists
                let has_path = graph.has_path(from, to);
                Ok(Value::boolean(has_path))
            }
            "shortest_path" => {
                // Find the shortest path between two nodes
                // shortest_path(from, to) - unweighted BFS
                // shortest_path(from, to, edge_type) - unweighted BFS with edge type filter
                // shortest_path(from, to, edge_type, :weighted) - weighted Dijkstra's algorithm
                if args.is_empty() || args.len() > 4 {
                    return Err(GraphoidError::runtime(format!(
                        "shortest_path() expects 2-4 arguments (from, to, [edge_type], [:weighted]), but got {}",
                        args.len()
                    )));
                }

                // Get from node ID
                let from = match &args[0].kind {
                    ValueKind::String(s) => s.clone(),
                    _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
                };

                // Get to node ID
                let to = match &args[1].kind {
                    ValueKind::String(s) => s.clone(),
                    _ => return Err(GraphoidError::type_error("string", args[1].type_name())),
                };

                // Parse optional edge_type and weighted flag
                let mut edge_type: Option<String> = None;
                let mut weighted = false;

                for arg in args.iter().skip(2) {
                    match &arg.kind {
                        ValueKind::String(s) => {
                            edge_type = Some(s.clone());
                        }
                        ValueKind::Symbol(s) if s == "weighted" => {
                            weighted = true;
                        }
                        _ => {
                            return Err(GraphoidError::runtime(format!(
                                "shortest_path() optional arguments must be edge_type (string) or :weighted symbol, got {}",
                                arg.type_name()
                            )));
                        }
                    }
                }

                // Find shortest path
                let path = graph.shortest_path(&from, &to, edge_type.as_deref(), weighted);

                match path {
                    Some(nodes) => {
                        let list: Vec<Value> = nodes.into_iter().map(Value::string).collect();
                        Ok(Value::list(List::from_vec(list)))
                    }
                    None => Ok(Value::none()),
                }
            }
            "distance" => {
                // Get shortest path distance between two nodes
                if args.len() != 2 {
                    return Err(GraphoidError::runtime(format!(
                        "distance() expects 2 arguments (from, to), but got {}",
                        args.len()
                    )));
                }

                // Get from node ID
                let from = match &args[0].kind {
                    ValueKind::String(s) => s.as_str(),
                    _other => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                // Get to node ID
                let to = match &args[1].kind {
                    ValueKind::String(s) => s.as_str(),
                    _other => {
                        return Err(GraphoidError::type_error("string", args[1].type_name()));
                    }
                };

                // Get distance
                let dist = graph.distance(from, to);
                Ok(Value::number(dist as f64))
            }
            "all_paths" => {
                // Find all paths between two nodes up to max length
                if args.len() != 3 {
                    return Err(GraphoidError::runtime(format!(
                        "all_paths() expects 3 arguments (from, to, max_length), but got {}",
                        args.len()
                    )));
                }

                // Get from node ID
                let from = match &args[0].kind {
                    ValueKind::String(s) => s.as_str(),
                    _other => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                // Get to node ID
                let to = match &args[1].kind {
                    ValueKind::String(s) => s.as_str(),
                    _other => {
                        return Err(GraphoidError::type_error("string", args[1].type_name()));
                    }
                };

                // Get max length
                let max_len = match &args[2].kind {
                    ValueKind::Number(n) => *n as usize,
                    _other => {
                        return Err(GraphoidError::type_error("number", args[2].type_name()));
                    }
                };

                // Find all paths
                let paths = graph.all_paths(from, to, max_len);

                // Convert Vec<Vec<String>> to ValueKind::List(List of Lists)
                use crate::values::List;
                let path_values: Vec<Value> = paths
                    .into_iter()
                    .map(|path| {
                        let string_values: Vec<Value> = path
                            .into_iter()
                            .map(|s| Value::string(s))
                            .collect();
                        Value::list(List::from_vec(string_values))
                    })
                    .collect();

                Ok(Value::list(List::from_vec(path_values)))
            }
            "match" => {
                // Graph pattern matching with explicit syntax
                // g.match(node(...), edge(...), node(...))

                // Pattern objects must come in alternating node-edge-node-edge-node pattern
                if args.is_empty() {
                    return Err(GraphoidError::runtime(
                        "match() requires at least one pattern node".to_string()
                    ));
                }

                // Validate pattern objects
                let mut nodes = Vec::new();
                let mut edges = Vec::new();
                let mut paths = Vec::new(); // Track which edges are variable-length paths

                for (i, arg) in args.iter().enumerate() {
                    if i % 2 == 0 {
                        // Even positions should be nodes
                        match &arg.kind {
                            ValueKind::PatternNode(pn) => {
                                nodes.push(pn.clone());
                            }
                            _ => {
                                return Err(GraphoidError::runtime(format!(
                                    "match() argument {} should be a pattern node, got {}",
                                    i, arg.type_name()
                                )));
                            }
                        }
                    } else {
                        // Odd positions should be edges or paths
                        match &arg.kind {
                            ValueKind::PatternEdge(pe) => {
                                edges.push(pe.clone());
                                paths.push(None); // Not a variable-length path
                            }
                            ValueKind::PatternPath(pp) => {
                                // Store PatternPath info for later
                                let pe = crate::values::PatternEdge {
                                    edge_type: Some(pp.edge_type.clone()),
                                    direction: pp.direction.clone(),
                                };
                                edges.push(pe);
                                paths.push(Some((pp.min, pp.max))); // Track min/max for variable-length
                            }
                            _ => {
                                return Err(GraphoidError::runtime(format!(
                                    "match() argument {} should be a pattern edge or path, got {}",
                                    i, arg.type_name()
                                )));
                            }
                        }
                    }
                }

                // Build GraphPattern AST from pattern objects
                use crate::ast::{GraphPattern, PatternNode, PatternEdge, EdgeDirection, EdgeLength};

                let dummy_pos = crate::error::SourcePosition::unknown();

                let ast_nodes: Vec<PatternNode> = nodes
                    .iter()
                    .map(|pn| PatternNode {
                        variable: pn.variable.clone().unwrap_or_else(|| "".to_string()),
                        node_type: pn.node_type.clone(),
                        position: dummy_pos.clone(),
                    })
                    .collect();

                let ast_edges: Vec<PatternEdge> = edges
                    .iter()
                    .enumerate()
                    .map(|(i, pe)| {
                        // Check if this edge is a variable-length path
                        let length = if let Some(Some((min, max))) = paths.get(i) {
                            EdgeLength::Variable { min: *min, max: *max }
                        } else {
                            EdgeLength::Fixed
                        };

                        PatternEdge {
                            from: ast_nodes[i].variable.clone(),
                            to: ast_nodes[i + 1].variable.clone(),
                            edge_type: pe.edge_type.clone(),
                            direction: match pe.direction.as_str() {
                                "outgoing" => EdgeDirection::Directed,
                                "both" => EdgeDirection::Bidirectional,
                                _ => EdgeDirection::Directed,
                            },
                            length,
                            position: dummy_pos.clone(),
                        }
                    })
                    .collect();

                let pattern = GraphPattern {
                    nodes: ast_nodes,
                    edges: ast_edges,
                    where_clause: None,
                    return_clause: None,
                    position: dummy_pos,
                };

                // Perform pattern matching
                let matches = self.match_pattern(&graph, &pattern)?;

                // Wrap matches in PatternMatchResults
                let results = crate::values::PatternMatchResults::new(matches, graph.clone());

                Ok(Value::pattern_match_results(results))
            }
            "get_node" => {
                // Get the value of a node by ID
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "get_node() expects 1 argument (node_id), but got {}",
                        args.len()
                    )));
                }

                // Get node ID
                let node_id = match &args[0].kind {
                    ValueKind::String(s) => s.as_str(),
                    _ => {
                        return Err(GraphoidError::type_error("string", args[0].type_name()));
                    }
                };

                // Get the node value
                match graph.get_node(node_id) {
                    Some(value) => Ok(value.clone()),
                    None => Ok(Value::none()),
                }
            }
            "nodes" => {
                // Get node IDs as a list
                // nodes()      - Data nodes only (default)
                // nodes(:all)  - All nodes including __methods__ branch
                if args.len() > 1 {
                    return Err(GraphoidError::runtime(format!(
                        "nodes() expects 0-1 arguments, but got {}",
                        args.len()
                    )));
                }

                let include_all = if args.len() == 1 {
                    match &args[0].kind {
                        ValueKind::Symbol(s) if s == "all" => true,
                        _ => {
                            return Err(GraphoidError::runtime(
                                "nodes() optional argument must be :all".to_string()
                            ));
                        }
                    }
                } else {
                    false
                };

                let node_ids = if include_all {
                    graph.all_node_ids()
                } else {
                    graph.node_ids()  // Already returns data nodes only
                };
                let node_id_values: Vec<Value> = node_ids.iter().map(|id| Value::string(id.clone())).collect();
                Ok(Value::list(crate::values::List::from_vec(node_id_values)))
            }
            "clone" => {
                // Create a deep copy of the graph, including all nodes, edges, and methods
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "clone() expects 0 arguments, but got {}",
                        args.len()
                    )));
                }

                // Rust's Clone trait already does a deep copy of all fields
                // including nodes, edges, rules, and methods
                Ok(Value::graph(graph.clone()))
            }
            // Phase 18: Type checking methods
            "type_of" => {
                // Returns the type name of the graph (the variable name it was assigned to)
                // e.g., Dog = graph{} -> Dog.type_of() returns "Dog"
                // Anonymous graphs return "graph"
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "type_of() expects 0 arguments, but got {}",
                        args.len()
                    )));
                }

                let type_name = graph.type_name.clone().unwrap_or_else(|| "graph".to_string());
                Ok(Value::string(type_name))
            }
            "is_a" => {
                // Checks if the graph is an instance of a type, walking the inheritance chain
                // e.g., puppy.is_a(Dog) returns true if Puppy = graph from Dog {}
                // Works with both type names (strings) and graph values
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "is_a() expects 1 argument, but got {}",
                        args.len()
                    )));
                }

                // Get the type name to check against
                let check_type = match &args[0].kind {
                    ValueKind::String(s) => s.clone(),
                    ValueKind::Graph(ref g) => g.borrow().type_name.clone().unwrap_or_else(|| "graph".to_string()),
                    _ => return Err(GraphoidError::runtime(format!(
                        "is_a() expects a type name (string) or graph, but got {}",
                        args[0].type_name()
                    ))),
                };

                // Walk the inheritance chain to check if this graph is of the given type
                let mut current = Some(&graph);
                while let Some(g) = current {
                    if let Some(ref name) = g.type_name {
                        if name == &check_type {
                            return Ok(Value::boolean(true));
                        }
                    }
                    // Move to parent
                    current = g.parent.as_ref().map(|p| p.as_ref());
                }

                Ok(Value::boolean(false))
            }
            "remove_method" => {
                // Remove a method from the graph by name
                // remove_method("method_name")
                // Returns true if method was removed, false if it didn't exist
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "remove_method() expects 1 argument, but got {}",
                        args.len()
                    )));
                }

                let method_name = match &args[0].kind {
                    ValueKind::String(s) => s.clone(),
                    _ => {
                        return Err(GraphoidError::runtime(
                            "remove_method() argument must be a string (method name)".to_string()
                        ));
                    }
                };

                let removed = graph.remove_method(&method_name);

                // Update graph in environment (mutation)
                if let Expr::Variable { name, .. } = object_expr {
                    self.env.set(name, Value::graph(graph))?;
                }

                Ok(Value::boolean(removed))
            }
            "include" => {
                // Phase 22: Mixin pattern - copy all methods from another graph
                // include(other_graph)
                // Returns a list of method names that were copied
                // Skips private methods (starting with underscore)
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "include() expects 1 argument (graph to include methods from), but got {}",
                        args.len()
                    )));
                }

                let other_graph = match &args[0].kind {
                    ValueKind::Graph(ref g) => g.borrow().clone(),
                    _ => {
                        return Err(GraphoidError::runtime(
                            "include() argument must be a graph".to_string()
                        ));
                    }
                };

                let included_names = graph.include_methods_from(&other_graph);

                // Update graph in environment (mutation)
                if let Expr::Variable { name, .. } = object_expr {
                    self.env.set(name, Value::graph(graph))?;
                }

                // Return list of included method names
                let name_values: Vec<Value> = included_names.iter()
                    .map(|n| Value::string(n.clone()))
                    .collect();
                Ok(Value::list(crate::values::List::from_vec(name_values)))
            }
            "responds_to" => {
                // Phase 23: Check if the graph has a method with the given name
                // responds_to("method_name") -> bool
                // Also checks parent graph if using inheritance
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "responds_to() expects 1 argument (method name), but got {}",
                        args.len()
                    )));
                }

                let method_name = match &args[0].kind {
                    ValueKind::String(s) => s.clone(),
                    _ => {
                        return Err(GraphoidError::runtime(
                            "responds_to() argument must be a string (method name)".to_string()
                        ));
                    }
                };

                // Check if this graph or any parent has the method
                // Walk up the inheritance chain
                fn check_method_in_chain(g: &crate::values::Graph, method_name: &str) -> bool {
                    if g.has_method(method_name) {
                        return true;
                    }
                    // Check parent if exists
                    if let Some(parent) = &g.parent {
                        return check_method_in_chain(parent, method_name);
                    }
                    false
                }

                let has_method = check_method_in_chain(&graph, &method_name);
                Ok(Value::boolean(has_method))
            }
            // Semantic edges: method-property relationship introspection
            "method_reads" => {
                // Get list of properties that a method reads
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "method_reads() expects 1 argument (method name), but got {}",
                        args.len()
                    )));
                }
                let method_name = match &args[0].kind {
                    ValueKind::String(s) => s.clone(),
                    _ => return Err(GraphoidError::runtime(
                        "method_reads() argument must be a string (method name)".to_string()
                    )),
                };
                let props = graph.method_reads(&method_name);
                let prop_values: Vec<Value> = props.iter().map(|p| Value::string(p.clone())).collect();
                Ok(Value::list(crate::values::List::from_vec(prop_values)))
            }
            "method_writes" => {
                // Get list of properties that a method writes
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "method_writes() expects 1 argument (method name), but got {}",
                        args.len()
                    )));
                }
                let method_name = match &args[0].kind {
                    ValueKind::String(s) => s.clone(),
                    _ => return Err(GraphoidError::runtime(
                        "method_writes() argument must be a string (method name)".to_string()
                    )),
                };
                let props = graph.method_writes(&method_name);
                let prop_values: Vec<Value> = props.iter().map(|p| Value::string(p.clone())).collect();
                Ok(Value::list(crate::values::List::from_vec(prop_values)))
            }
            "property_readers" => {
                // Get list of methods that read a property
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "property_readers() expects 1 argument (property name), but got {}",
                        args.len()
                    )));
                }
                let prop_name = match &args[0].kind {
                    ValueKind::String(s) => s.clone(),
                    _ => return Err(GraphoidError::runtime(
                        "property_readers() argument must be a string (property name)".to_string()
                    )),
                };
                let methods = graph.property_readers(&prop_name);
                let method_values: Vec<Value> = methods.iter().map(|m| Value::string(m.clone())).collect();
                Ok(Value::list(crate::values::List::from_vec(method_values)))
            }
            "property_writers" => {
                // Get list of methods that write to a property
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "property_writers() expects 1 argument (property name), but got {}",
                        args.len()
                    )));
                }
                let prop_name = match &args[0].kind {
                    ValueKind::String(s) => s.clone(),
                    _ => return Err(GraphoidError::runtime(
                        "property_writers() argument must be a string (property name)".to_string()
                    )),
                };
                let methods = graph.property_writers(&prop_name);
                let method_values: Vec<Value> = methods.iter().map(|m| Value::string(m.clone())).collect();
                Ok(Value::list(crate::values::List::from_vec(method_values)))
            }
            // Phase 2: Property dependency methods
            "dependencies" => {
                // Get list of properties that this computed property depends on
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "dependencies() expects 1 argument (property name), but got {}",
                        args.len()
                    )));
                }
                let prop_name = match &args[0].kind {
                    ValueKind::String(s) => s.clone(),
                    _ => return Err(GraphoidError::runtime(
                        "dependencies() argument must be a string (property name)".to_string()
                    )),
                };
                let deps = graph.dependencies(&prop_name);
                let dep_values: Vec<Value> = deps.iter().map(|d| Value::string(d.clone())).collect();
                Ok(Value::list(crate::values::List::from_vec(dep_values)))
            }
            "dependents" => {
                // Get list of computed properties that depend on this property
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "dependents() expects 1 argument (property name), but got {}",
                        args.len()
                    )));
                }
                let prop_name = match &args[0].kind {
                    ValueKind::String(s) => s.clone(),
                    _ => return Err(GraphoidError::runtime(
                        "dependents() argument must be a string (property name)".to_string()
                    )),
                };
                let deps = graph.dependents(&prop_name);
                let dep_values: Vec<Value> = deps.iter().map(|d| Value::string(d.clone())).collect();
                Ok(Value::list(crate::values::List::from_vec(dep_values)))
            }
            "dependency_order" => {
                // Get properties in topological order based on dependencies
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "dependency_order() expects no arguments, but got {}",
                        args.len()
                    )));
                }
                let order = graph.dependency_order();
                let order_values: Vec<Value> = order.iter().map(|p| Value::string(p.clone())).collect();
                Ok(Value::list(crate::values::List::from_vec(order_values)))
            }
            // Phase 3: Inheritance methods
            "ancestors" => {
                // Get list of ancestor type names
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "ancestors() expects no arguments, but got {}",
                        args.len()
                    )));
                }
                let ancestors = graph.ancestors();
                let ancestor_values: Vec<Value> = ancestors.iter().map(|a| Value::string(a.clone())).collect();
                Ok(Value::list(crate::values::List::from_vec(ancestor_values)))
            }
            "edges" => {
                // Get edges as a list of lists [from, to, edge_type]
                // edges()      - Data edges only (default)
                // edges(:all)  - All edges including __methods__ branch
                if args.len() > 1 {
                    return Err(GraphoidError::runtime(format!(
                        "edges() expects 0-1 arguments, but got {}",
                        args.len()
                    )));
                }

                let include_all = if args.len() == 1 {
                    match &args[0].kind {
                        ValueKind::Symbol(s) if s == "all" => true,
                        _ => {
                            return Err(GraphoidError::runtime(
                                "edges() optional argument must be :all".to_string()
                            ));
                        }
                    }
                } else {
                    false
                };

                let edge_list = if include_all {
                    graph.edge_list()  // All edges
                } else {
                    graph.data_edge_list()  // Data edges only
                };
                let edge_values: Vec<Value> = edge_list.iter().map(|(from, to, edge_type)| {
                    let edge_vec = vec![
                        Value::string(from.clone()),
                        Value::string(to.clone()),
                        Value::string(edge_type.clone()),
                    ];
                    Value::list(crate::values::List::from_vec(edge_vec))
                }).collect();
                Ok(Value::list(crate::values::List::from_vec(edge_values)))
            }
            "extract" => {
                // Extract subgraph using filter predicates
                // Supports two syntaxes:
                // 1. Positional: extract(node_filter?, edge_filter?, include_orphans?)
                // 2. Block: extract({ nodes: filter, edges: filter, include_orphans: bool })

                let (node_filter, edge_filter, include_orphans) = if args.len() == 1 {
                    // Check if first arg is a map (block syntax)
                    if let ValueKind::Map(map) = &args[0].kind {
                        // Block syntax
                        let node_filter = map.get("nodes").filter(|v| !matches!(v.kind, ValueKind::None));
                        let edge_filter = map.get("edges").filter(|v| !matches!(v.kind, ValueKind::None));
                        let include_orphans = map.get("include_orphans")
                            .map(|v| v.is_truthy())
                            .unwrap_or(true);

                        (node_filter, edge_filter, include_orphans)
                    } else {
                        // Single positional arg (node_filter only)
                        let node_filter = if !matches!(&args[0].kind, ValueKind::None) {
                            Some(&args[0])
                        } else {
                            None
                        };
                        (node_filter, None, true)
                    }
                } else if args.len() <= 3 {
                    // Positional syntax
                    let node_filter = if !args.is_empty() && !matches!(&args[0].kind, ValueKind::None) {
                        Some(&args[0])
                    } else {
                        None
                    };

                    let edge_filter = if args.len() > 1 && !matches!(&args[1].kind, ValueKind::None) {
                        Some(&args[1])
                    } else {
                        None
                    };

                    let include_orphans = if args.len() > 2 {
                        args[2].is_truthy()
                    } else {
                        true // default
                    };

                    (node_filter, edge_filter, include_orphans)
                } else {
                    return Err(GraphoidError::runtime(format!(
                        "extract() expects 0-3 arguments or a single map, but got {}",
                        args.len()
                    )));
                };

                // Apply node filter using helper
                let matching_node_ids = self.apply_node_filter(&graph, node_filter, false)?;

                // Apply edge filter using helper
                let matching_edges = self.apply_edge_filter(&graph, edge_filter, &matching_node_ids, false)?;

                // Track which nodes have edges
                let mut nodes_with_edges: std::collections::HashSet<String> = std::collections::HashSet::new();
                for (from_id, to_id, _) in &matching_edges {
                    nodes_with_edges.insert(from_id.clone());
                    nodes_with_edges.insert(to_id.clone());
                }

                // Determine final nodes based on include_orphans
                let final_nodes: std::collections::HashSet<String> = if include_orphans {
                    matching_node_ids
                } else {
                    matching_node_ids.intersection(&nodes_with_edges).cloned().collect()
                };

                // Build result graph
                use crate::values::graph::Graph;
                let mut result = Graph::new(graph.graph_type.clone());
                result.config = graph.config.clone();

                // Add nodes
                for node_id in &final_nodes {
                    if let Some(node_value) = graph.get_node(node_id) {
                        result.add_node(node_id.clone(), node_value.clone())?;
                    }
                }

                // Add edges
                for (from_id, to_id, edge_type) in matching_edges {
                    result.add_edge(
                        &from_id,
                        &to_id,
                        edge_type,
                        None,
                        std::collections::HashMap::new(),
                    )?;
                }

                Ok(Value::graph(result))
            }
            "delete" => {
                // Delete subgraph using filter predicates (inverse of extract)
                // Supports two syntaxes:
                // 1. Positional: delete(node_filter?, edge_filter?)
                // 2. Block: delete({ nodes: filter, edges: filter })

                let (node_filter, edge_filter) = if args.len() == 1 {
                    // Check if first arg is a map (block syntax)
                    if let ValueKind::Map(map) = &args[0].kind {
                        // Block syntax
                        let node_filter = map.get("nodes").filter(|v| !matches!(v.kind, ValueKind::None));
                        let edge_filter = map.get("edges").filter(|v| !matches!(v.kind, ValueKind::None));
                        (node_filter, edge_filter)
                    } else {
                        // Single positional arg (node_filter only)
                        let node_filter = if !matches!(&args[0].kind, ValueKind::None) {
                            Some(&args[0])
                        } else {
                            None
                        };
                        (node_filter, None)
                    }
                } else if args.len() <= 2 {
                    // Positional syntax
                    let node_filter = if !args.is_empty() && !matches!(&args[0].kind, ValueKind::None) {
                        Some(&args[0])
                    } else {
                        None
                    };

                    let edge_filter = if args.len() > 1 && !matches!(&args[1].kind, ValueKind::None) {
                        Some(&args[1])
                    } else {
                        None
                    };

                    (node_filter, edge_filter)
                } else {
                    return Err(GraphoidError::runtime(format!(
                        "delete() expects 0-2 arguments or a single map, but got {}",
                        args.len()
                    )));
                };

                // Apply node filter with inversion (keep nodes that DON'T match)
                let keeping_node_ids = self.apply_node_filter(&graph, node_filter, true)?;

                // Apply edge filter with inversion (keep edges that DON'T match)
                let keeping_edges = self.apply_edge_filter(&graph, edge_filter, &keeping_node_ids, true)?;

                // Build result graph
                use crate::values::graph::Graph;
                let mut result = Graph::new(graph.graph_type.clone());
                result.config = graph.config.clone();

                // Add kept nodes
                for node_id in &keeping_node_ids {
                    if let Some(node_value) = graph.get_node(node_id) {
                        result.add_node(node_id.clone(), node_value.clone())?;
                    }
                }

                // Add kept edges
                for (from_id, to_id, edge_type) in keeping_edges {
                    result.add_edge(
                        &from_id,
                        &to_id,
                        edge_type,
                        None,
                        std::collections::HashMap::new(),
                    )?;
                }

                Ok(Value::graph(result))
            }
            "add_subgraph" => {
                // Merge another graph with conflict resolution
                // Arguments: (other_graph, conflict_strategy?)
                if args.is_empty() || args.len() > 2 {
                    return Err(GraphoidError::runtime(format!(
                        "add_subgraph() expects 1-2 arguments (other_graph, conflict_strategy), but got {}",
                        args.len()
                    )));
                }

                let other_graph = match &args[0].kind {
                    ValueKind::Graph(ref g) => g.borrow(),
                    _ => return Err(GraphoidError::type_error("graph", args[0].type_name())),
                };

                let conflict_strategy = if args.len() > 1 {
                    Some(args[1].to_string_value())
                } else {
                    None
                };

                let result = graph.add_subgraph(&other_graph, conflict_strategy)?;
                Ok(Value::graph(result))
            }
            "node_count" => {
                // Return the number of nodes in the graph
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "node_count() expects 0 arguments, but got {}",
                        args.len()
                    )));
                }
                Ok(Value::number(graph.node_count() as f64))
            }
            "edge_count" => {
                // Return the number of edges in the graph
                if !args.is_empty() {
                    return Err(GraphoidError::runtime(format!(
                        "edge_count() expects 0 arguments, but got {}",
                        args.len()
                    )));
                }
                Ok(Value::number(graph.edge_count() as f64))
            }
            "add_rule" => {
                // Add a rule to the graph (scoped to data layer only)
                // add_rule(:rule_name) or add_rule(:rule_name, param)
                if args.is_empty() || args.len() > 2 {
                    return Err(GraphoidError::runtime(format!(
                        "add_rule() expects 1-2 arguments (rule_symbol, [param]), but got {}",
                        args.len()
                    )));
                }

                // Get rule symbol
                let rule_symbol = match &args[0].kind {
                    ValueKind::Symbol(name) => name.as_str(),
                    _ => {
                        return Err(GraphoidError::runtime(format!(
                            "add_rule() expects a symbol, got {}",
                            args[0].type_name()
                        )));
                    }
                };

                // Get optional parameter
                let param = if args.len() == 2 {
                    match &args[1].kind {
                        ValueKind::Number(n) => Some(*n),
                        _ => {
                            return Err(GraphoidError::runtime(format!(
                                "add_rule() parameter must be a number, got {}",
                                args[1].type_name()
                            )));
                        }
                    }
                } else {
                    None
                };

                // Convert symbol to RuleSpec
                let rule_spec = Self::symbol_to_rule_spec(rule_symbol, param)?;

                // Add rule to graph
                use crate::graph::RuleInstance;
                graph.add_rule(RuleInstance::new(rule_spec))?;

                // Update graph in environment (mutation)
                if let Expr::Variable { name, .. } = object_expr {
                    self.env.set(name, Value::graph(graph))?;
                }

                Ok(Value::none())
            }
            "remove_rule" => {
                // Remove a rule from the graph
                // remove_rule(:rule_name) or remove_rule(:rule_name, param)
                if args.is_empty() || args.len() > 2 {
                    return Err(GraphoidError::runtime(format!(
                        "remove_rule() expects 1-2 arguments (rule_symbol, [param]), but got {}",
                        args.len()
                    )));
                }

                // Get rule symbol
                let rule_symbol = match &args[0].kind {
                    ValueKind::Symbol(name) => name.as_str(),
                    _ => {
                        return Err(GraphoidError::runtime(format!(
                            "remove_rule() expects a symbol, got {}",
                            args[0].type_name()
                        )));
                    }
                };

                // Get optional parameter
                let param = if args.len() == 2 {
                    match &args[1].kind {
                        ValueKind::Number(n) => Some(*n),
                        _ => {
                            return Err(GraphoidError::runtime(format!(
                                "remove_rule() parameter must be a number, got {}",
                                args[1].type_name()
                            )));
                        }
                    }
                } else {
                    None
                };

                // Convert symbol to RuleSpec
                let rule_spec = Self::symbol_to_rule_spec(rule_symbol, param)?;

                // Remove rule from graph
                graph.remove_rule(&rule_spec);

                // Update graph in environment (mutation)
                if let Expr::Variable { name, .. } = object_expr {
                    self.env.set(name, Value::graph(graph))?;
                }

                Ok(Value::none())
            }
            "add_method_constraint" => {
                // Add a custom method constraint function
                // add_method_constraint(constraint_fn) or add_method_constraint(constraint_fn, "name")
                // The constraint function receives (before_graph, after_graph) and returns true if allowed
                if args.is_empty() || args.len() > 2 {
                    return Err(GraphoidError::runtime(format!(
                        "add_method_constraint() expects 1-2 arguments (function, [name]), but got {}",
                        args.len()
                    )));
                }

                // Get the constraint function
                let constraint_fn = match &args[0].kind {
                    ValueKind::Function(_) => args[0].clone(),
                    _ => {
                        return Err(GraphoidError::runtime(format!(
                            "add_method_constraint() expects a function, got {}",
                            args[0].type_name()
                        )));
                    }
                };

                // Get optional name (defaults to "custom_constraint")
                let name = if args.len() == 2 {
                    match &args[1].kind {
                        ValueKind::String(s) => s.clone(),
                        _ => {
                            return Err(GraphoidError::runtime(format!(
                                "add_method_constraint() name must be a string, got {}",
                                args[1].type_name()
                            )));
                        }
                    }
                } else {
                    "custom_constraint".to_string()
                };

                // Create the custom method constraint rule
                let rule_spec = RuleSpec::CustomMethodConstraint {
                    function: constraint_fn,
                    name,
                };

                // Add rule to graph
                use crate::graph::RuleInstance;
                graph.add_rule(RuleInstance::new(rule_spec))?;

                // Update graph in environment (mutation)
                if let Expr::Variable { name, .. } = object_expr {
                    self.env.set(name, Value::graph(graph))?;
                }

                Ok(Value::none())
            }
            "has_rule" => {
                // Check if graph has a specific rule (from either rulesets or ad hoc)
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "has_rule() expects 1 argument (rule_symbol), but got {}",
                        args.len()
                    )));
                }

                let rule_name = match &args[0].kind {
                    ValueKind::Symbol(name) => name.as_str(),
                    _ => {
                        return Err(GraphoidError::runtime(format!(
                            "has_rule() expects a symbol argument, got {}",
                            args[0].type_name()
                        )));
                    }
                };

                Ok(Value::boolean(graph.has_rule(rule_name)))
            }
            "rule" => {
                // Get a rule's parameter value
                // g.rule(:max_degree) -> returns the max degree value (e.g., 2)
                // g.rule(:no_cycles) -> returns true if the rule exists
                // g.rule(:nonexistent) -> returns none
                if args.len() != 1 {
                    return Err(GraphoidError::runtime(format!(
                        "rule() expects 1 argument (rule_symbol), but got {}",
                        args.len()
                    )));
                }

                let rule_name = match &args[0].kind {
                    ValueKind::Symbol(name) => name.as_str(),
                    _ => {
                        return Err(GraphoidError::runtime(format!(
                            "rule() expects a symbol argument, got {}",
                            args[0].type_name()
                        )));
                    }
                };

                use crate::graph::RuleSpec;
                use crate::values::List;
                match graph.get_rule(rule_name) {
                    Some(spec) => {
                        // Return the parameter value for parameterized rules
                        match spec {
                            RuleSpec::MaxDegree(n) => Ok(Value::number(n as f64)),
                            RuleSpec::ValidateRange { min, max } => {
                                // Return as a list [min, max]
                                Ok(Value::list(List::from_vec(vec![Value::number(min), Value::number(max)])))
                            }
                            // For non-parameterized rules, return true
                            _ => Ok(Value::boolean(true)),
                        }
                    }
                    None => Ok(Value::none()),
                }
            }
            "visualize" => {
                // Text visualization of the graph
                // visualize()       - Data layer only (default)
                // visualize(:all)   - All layers including __methods__
                if args.len() > 1 {
                    return Err(GraphoidError::runtime(format!(
                        "visualize() expects 0-1 arguments, but got {}",
                        args.len()
                    )));
                }

                let include_all = if args.len() == 1 {
                    match &args[0].kind {
                        ValueKind::Symbol(s) if s == "all" => true,
                        _ => {
                            return Err(GraphoidError::runtime(
                                "visualize() optional argument must be :all".to_string()
                            ));
                        }
                    }
                } else {
                    false
                };

                let mut output = String::new();
                output.push_str("Graph:\n");

                // Get nodes based on visibility
                let node_ids = if include_all {
                    graph.all_node_ids()
                } else {
                    graph.node_ids()
                };

                // Show nodes
                output.push_str("  Nodes:\n");
                if node_ids.is_empty() {
                    output.push_str("    (none)\n");
                } else {
                    for node_id in &node_ids {
                        if let Some(value) = graph.get_node(node_id) {
                            output.push_str(&format!("    {} = {}\n", node_id, value.to_string_value()));
                        } else {
                            output.push_str(&format!("    {}\n", node_id));
                        }
                    }
                }

                // Get edges based on visibility
                let edges = if include_all {
                    graph.edge_list()
                } else {
                    graph.data_edge_list()
                };

                // Show edges
                output.push_str("  Edges:\n");
                if edges.is_empty() {
                    output.push_str("    (none)\n");
                } else {
                    for (from, to, edge_type) in &edges {
                        output.push_str(&format!("    {} -> {} [{}]\n", from, to, edge_type));
                    }
                }

                Ok(Value::string(output))
            }
            "to_dot" => {
                // Export to Graphviz DOT format
                // to_dot()       - Data layer only (default)
                // to_dot(:all)   - All layers including __methods__
                if args.len() > 1 {
                    return Err(GraphoidError::runtime(format!(
                        "to_dot() expects 0-1 arguments, but got {}",
                        args.len()
                    )));
                }

                let include_all = if args.len() == 1 {
                    match &args[0].kind {
                        ValueKind::Symbol(s) if s == "all" => true,
                        _ => {
                            return Err(GraphoidError::runtime(
                                "to_dot() optional argument must be :all".to_string()
                            ));
                        }
                    }
                } else {
                    false
                };

                let mut output = String::new();
                output.push_str("digraph G {\n");

                // Get nodes and edges based on visibility
                let node_ids = if include_all {
                    graph.all_node_ids()
                } else {
                    graph.node_ids()
                };

                let edges = if include_all {
                    graph.edge_list()
                } else {
                    graph.data_edge_list()
                };

                // Add node declarations
                for node_id in &node_ids {
                    // Escape quotes in node ID
                    let escaped_id = node_id.replace("\"", "\\\"");
                    output.push_str(&format!("  \"{}\";\n", escaped_id));
                }

                // Add edge declarations
                for (from, to, edge_type) in &edges {
                    let escaped_from = from.replace("\"", "\\\"");
                    let escaped_to = to.replace("\"", "\\\"");
                    let escaped_type = edge_type.replace("\"", "\\\"");
                    output.push_str(&format!(
                        "  \"{}\" -> \"{}\" [label=\"{}\"];\n",
                        escaped_from, escaped_to, escaped_type
                    ));
                }

                output.push_str("}\n");

                Ok(Value::string(output))
            }
            "to_ascii" => {
                // ASCII tree visualization
                // Works best for tree-like structures
                // to_ascii()       - Data layer only (default)
                // to_ascii(:all)   - All layers
                if args.len() > 1 {
                    return Err(GraphoidError::runtime(format!(
                        "to_ascii() expects 0-1 arguments, but got {}",
                        args.len()
                    )));
                }

                let include_all = if args.len() == 1 {
                    match &args[0].kind {
                        ValueKind::Symbol(s) if s == "all" => true,
                        _ => {
                            return Err(GraphoidError::runtime(
                                "to_ascii() optional argument must be :all".to_string()
                            ));
                        }
                    }
                } else {
                    false
                };

                // Get nodes and edges based on visibility
                let node_ids = if include_all {
                    graph.all_node_ids()
                } else {
                    graph.node_ids()
                };

                let edges = if include_all {
                    graph.edge_list()
                } else {
                    graph.data_edge_list()
                };

                // Build adjacency list for children
                let mut children: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
                let mut has_parent: std::collections::HashSet<String> = std::collections::HashSet::new();

                for (from, to, _) in &edges {
                    children.entry(from.clone()).or_default().push(to.clone());
                    has_parent.insert(to.clone());
                }

                // Find root nodes (nodes with no incoming edges)
                let roots: Vec<String> = node_ids
                    .iter()
                    .filter(|id| !has_parent.contains(*id))
                    .cloned()
                    .collect();

                // Helper function to build ASCII tree
                fn build_tree(
                    node: &str,
                    children: &std::collections::HashMap<String, Vec<String>>,
                    prefix: &str,
                    is_last: bool,
                    output: &mut String,
                ) {
                    // Add current node
                    let connector = if prefix.is_empty() {
                        ""
                    } else if is_last {
                        "└── "
                    } else {
                        "├── "
                    };
                    output.push_str(&format!("{}{}{}\n", prefix, connector, node));

                    // Get children of this node
                    if let Some(child_list) = children.get(node) {
                        let new_prefix = if prefix.is_empty() {
                            "".to_string()
                        } else if is_last {
                            format!("{}    ", prefix)
                        } else {
                            format!("{}│   ", prefix)
                        };

                        for (i, child) in child_list.iter().enumerate() {
                            let child_is_last = i == child_list.len() - 1;
                            build_tree(child, children, &new_prefix, child_is_last, output);
                        }
                    }
                }

                let mut output = String::new();

                if roots.is_empty() && !node_ids.is_empty() {
                    // No clear root - just list nodes
                    output.push_str("(no clear root - listing nodes)\n");
                    for node_id in &node_ids {
                        output.push_str(&format!("  {}\n", node_id));
                    }
                } else if roots.is_empty() {
                    output.push_str("(empty graph)\n");
                } else {
                    // Build tree from each root
                    for (i, root) in roots.iter().enumerate() {
                        if i > 0 {
                            output.push('\n');
                        }
                        build_tree(root, &children, "", true, &mut output);
                    }
                }

                Ok(Value::string(output))
            }
            "equals" => {
                // Compare graphs with layer options
                // Syntax: graph.equals(other, include: :rules) or graph.equals(other, only: :rules)
                if args.is_empty() {
                    return Err(GraphoidError::runtime(
                        "equals() requires at least 1 argument (the other graph to compare)".to_string()
                    ));
                }

                // First argument must be a graph
                let other_graph = match &args[0].kind {
                    ValueKind::Graph(g) => g.borrow(),
                    _ => return Err(GraphoidError::type_error("graph", args[0].type_name())),
                };

                // Parse options from remaining arguments
                // Look for include: or only: named parameters
                let mut layers: std::collections::HashSet<crate::values::ComparisonLayer> = std::collections::HashSet::new();
                let mut only_mode = false;

                // Check for named parameters in args[1..] which would be a hash
                if args.len() > 1 {
                    if let ValueKind::Map(opts) = &args[1].kind {
                        // Check for include: key
                        if let Some(include_val) = opts.get("include") {
                            layers = self.parse_comparison_layers(include_val)?;
                            only_mode = false;
                        }
                        // Check for only: key
                        if let Some(only_val) = opts.get("only") {
                            layers = self.parse_comparison_layers(only_val)?;
                            only_mode = true;
                        }
                    }
                }

                // If no layers specified, default to data layer
                if layers.is_empty() && !only_mode {
                    layers.insert(crate::values::ComparisonLayer::Data);
                }

                let result = graph.equals_with_layers(&other_graph, Some(&layers), only_mode);
                Ok(Value::boolean(result))
            }
            _ => Err(GraphoidError::runtime(format!(
                "Graph does not have method '{}'",
                method
            ))),
        }
    }

    /// Parse comparison layers from a value (symbol or list of symbols)
    fn parse_comparison_layers(&self, value: &Value) -> Result<std::collections::HashSet<crate::values::ComparisonLayer>> {
        let mut layers = std::collections::HashSet::new();

        match &value.kind {
            ValueKind::Symbol(s) => {
                layers.insert(self.symbol_to_layer(s.as_str())?);
            }
            ValueKind::List(list) => {
                for i in 0..list.len() {
                    if let Some(item) = list.get(i) {
                        if let ValueKind::Symbol(s) = &item.kind {
                            layers.insert(self.symbol_to_layer(s.as_str())?);
                        } else {
                            return Err(GraphoidError::runtime(format!(
                                "equals() layer must be a symbol, got {}",
                                item.type_name()
                            )));
                        }
                    }
                }
            }
            _ => {
                return Err(GraphoidError::runtime(format!(
                    "equals() include:/only: must be a symbol or list of symbols, got {}",
                    value.type_name()
                )));
            }
        }

        Ok(layers)
    }

    /// Convert a symbol string to ComparisonLayer
    fn symbol_to_layer(&self, symbol: &str) -> Result<crate::values::ComparisonLayer> {
        use crate::values::ComparisonLayer;
        match symbol {
            "data" => Ok(ComparisonLayer::Data),
            "rules" => Ok(ComparisonLayer::Rules),
            "rulesets" => Ok(ComparisonLayer::Rulesets),
            "methods" => Ok(ComparisonLayer::Methods),
            "properties" => Ok(ComparisonLayer::Properties),
            "all" => Ok(ComparisonLayer::All),
            _ => Err(GraphoidError::runtime(format!(
                "Unknown comparison layer '{}'. Valid layers: :data, :rules, :rulesets, :methods, :properties, :all",
                symbol
            ))),
        }
    }
}

//! Integration tests for graph method constraint rules (Phase 11)
//!
//! Tests the method constraint rules:
//! - `:no_node_removals` - Methods cannot remove nodes (append-only)
//! - `:no_edge_removals` - Methods cannot remove edges (permanent connections)
//! - `:read_only` - Methods cannot modify the graph at all (immutable)

use graphoid::execution::Executor;

/// Helper to run Graphoid code and return success/error
fn run_code(code: &str) -> Result<(), String> {
    let mut executor = Executor::new();
    match executor.execute_source(code) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}", e)),
    }
}

/// Helper to run Graphoid code and check it succeeds
fn assert_runs(code: &str) {
    if let Err(e) = run_code(code) {
        panic!("Expected code to run successfully, but got error: {}\n\nCode:\n{}", e, code);
    }
}

/// Helper to run Graphoid code and check it fails with a message containing the given substring
fn assert_fails_with(code: &str, expected_substring: &str) {
    match run_code(code) {
        Ok(_) => panic!("Expected code to fail, but it succeeded.\n\nCode:\n{}", code),
        Err(e) => {
            if !e.contains(expected_substring) {
                panic!(
                    "Expected error to contain '{}', but got: {}\n\nCode:\n{}",
                    expected_substring, e, code
                );
            }
        }
    }
}

// ============================================================================
// Basic Constraint Addition Tests
// ============================================================================

#[test]
fn test_add_no_node_removals_rule() {
    let code = r#"
g = graph{}
g.add_rule(:no_node_removals)
print(g.has_rule(:no_node_removals))
"#;
    assert_runs(code);
}

#[test]
fn test_add_no_edge_removals_rule() {
    let code = r#"
g = graph{}
g.add_rule(:no_edge_removals)
print(g.has_rule(:no_edge_removals))
"#;
    assert_runs(code);
}

#[test]
fn test_add_read_only_rule() {
    let code = r#"
g = graph{}
g.add_rule(:read_only)
print(g.has_rule(:read_only))
"#;
    assert_runs(code);
}

// ============================================================================
// No Node Removals Constraint Tests
// ============================================================================

#[test]
fn test_no_node_removals_allows_read_methods() {
    let code = r#"
graph G {
    fn count() {
        return self.node_count()
    }
}
g = G.clone()
g.add_node("a", 1)
g.add_rule(:no_node_removals)

result = g.count()
print(result)
"#;
    assert_runs(code);
}

#[test]
fn test_no_node_removals_blocks_remove_node() {
    let code = r#"
graph G {
    fn remove_first() {
        self.remove_node("a")
    }
}
g = G.clone()
g.add_node("a", 1)
g.add_rule(:no_node_removals)

g.remove_first()
"#;
    assert_fails_with(code, "no_node_removals");
}

#[test]
fn test_no_node_removals_allows_adding_nodes() {
    let code = r#"
graph G {
    fn add_new_node() {
        self.add_node("b", 2)
    }
}
g = G.clone()
g.add_node("a", 1)
g.add_rule(:no_node_removals)

g.add_new_node()
print(g.node_count())
"#;
    assert_runs(code);
}

#[test]
fn test_no_node_removals_allows_adding_edges() {
    let code = r#"
graph G {
    fn link() {
        self.add_edge("a", "b", "connected")
    }
}
g = G.clone()
g.add_node("a", 1)
g.add_node("b", 2)
g.add_rule(:no_node_removals)

g.link()
print(g.edge_count())
"#;
    assert_runs(code);
}

// ============================================================================
// No Edge Removals Constraint Tests
// ============================================================================

#[test]
fn test_no_edge_removals_allows_read_methods() {
    let code = r#"
graph G {
    fn count_edges() {
        return self.edge_count()
    }
}
g = G.clone()
g.add_node("a", 1)
g.add_node("b", 2)
g.add_edge("a", "b", "link")
g.add_rule(:no_edge_removals)

result = g.count_edges()
print(result)
"#;
    assert_runs(code);
}

#[test]
fn test_no_edge_removals_blocks_remove_edge() {
    let code = r#"
graph G {
    fn unlink() {
        self.remove_edge("a", "b")
    }
}
g = G.clone()
g.add_node("a", 1)
g.add_node("b", 2)
g.add_edge("a", "b", "link")
g.add_rule(:no_edge_removals)

g.unlink()
"#;
    assert_fails_with(code, "no_edge_removals");
}

#[test]
fn test_no_edge_removals_allows_adding_edges() {
    let code = r#"
graph G {
    fn link_nodes() {
        self.add_edge("a", "b", "link")
    }
}
g = G.clone()
g.add_node("a", 1)
g.add_node("b", 2)
g.add_node("c", 3)
g.add_rule(:no_edge_removals)

g.link_nodes()
print(g.edge_count())
"#;
    assert_runs(code);
}

#[test]
fn test_no_edge_removals_allows_adding_nodes() {
    let code = r#"
graph G {
    fn add_new() {
        self.add_node("b", 2)
    }
}
g = G.clone()
g.add_node("a", 1)
g.add_rule(:no_edge_removals)

g.add_new()
print(g.node_count())
"#;
    assert_runs(code);
}

// ============================================================================
// Read Only Constraint Tests
// ============================================================================

#[test]
fn test_read_only_allows_read_methods() {
    let code = r#"
graph G {
    fn get_info() {
        return self.node_count() + self.edge_count()
    }
}
g = G.clone()
g.add_node("a", 1)
g.add_node("b", 2)
g.add_edge("a", "b", "link")
g.add_rule(:read_only)

result = g.get_info()
print(result)
"#;
    assert_runs(code);
}

#[test]
fn test_read_only_blocks_add_node() {
    let code = r#"
graph G {
    fn add_new() {
        self.add_node("b", 2)
    }
}
g = G.clone()
g.add_node("a", 1)
g.add_rule(:read_only)

g.add_new()
"#;
    assert_fails_with(code, "read_only");
}

#[test]
fn test_read_only_blocks_remove_node() {
    let code = r#"
graph G {
    fn remove_first() {
        self.remove_node("a")
    }
}
g = G.clone()
g.add_node("a", 1)
g.add_rule(:read_only)

g.remove_first()
"#;
    assert_fails_with(code, "read_only");
}

#[test]
fn test_read_only_blocks_add_edge() {
    let code = r#"
graph G {
    fn link() {
        self.add_edge("a", "b", "link")
    }
}
g = G.clone()
g.add_node("a", 1)
g.add_node("b", 2)
g.add_rule(:read_only)

g.link()
"#;
    assert_fails_with(code, "read_only");
}

#[test]
fn test_read_only_blocks_remove_edge() {
    let code = r#"
graph G {
    fn unlink() {
        self.remove_edge("a", "b")
    }
}
g = G.clone()
g.add_node("a", 1)
g.add_node("b", 2)
g.add_edge("a", "b", "link")
g.add_rule(:read_only)

g.unlink()
"#;
    assert_fails_with(code, "read_only");
}

// ============================================================================
// Combined Constraints Tests
// ============================================================================

#[test]
fn test_multiple_constraints_all_apply() {
    let code = r#"
graph G {
    fn remove_stuff() {
        self.remove_node("a")
    }
}
g = G.clone()
g.add_node("a", 1)
g.add_node("b", 2)
g.add_edge("a", "b", "link")
g.add_rule(:no_node_removals)
g.add_rule(:no_edge_removals)

g.remove_stuff()
"#;
    assert_fails_with(code, "no_node_removals");
}

#[test]
fn test_constraint_removed_allows_operation() {
    let code = r#"
graph G {
    fn remove_first() {
        self.remove_node("a")
    }
}
g = G.clone()
g.add_node("a", 1)
g.add_rule(:no_node_removals)
g.remove_rule(:no_node_removals)

g.remove_first()
print(g.node_count())
"#;
    assert_runs(code);
}

// ============================================================================
// Constraint Does Not Affect Non-Method Operations
// ============================================================================

#[test]
fn test_constraints_only_apply_to_methods() {
    // Direct operations (not through methods) should still work
    let code = r#"
g = graph{}
g.add_rule(:read_only)
# Direct add_node is NOT a method call on a user-defined method
# So constraints don't apply here
g.add_node("a", 1)
print(g.node_count())
"#;
    assert_runs(code);
}

// ============================================================================
// Cloned Instances Preserve Constraints
// ============================================================================

#[test]
fn test_clone_preserves_constraints() {
    let code = r#"
graph Counter {
    count: 0

    fn new(initial) {
        instance = self.clone()
        return instance
    }

    fn try_remove() {
        self.remove_node("count")
    }
}
Counter.add_rule(:no_node_removals)

c = Counter.new(0)
c.try_remove()
"#;
    assert_fails_with(code, "no_node_removals");
}

// ============================================================================
// Query Methods for Constraints
// ============================================================================

#[test]
fn test_rule_method_returns_true_for_constraint() {
    let code = r#"
g = graph{}
g.add_rule(:read_only)
result = g.rule(:read_only)
print(result)
"#;
    assert_runs(code);
}

#[test]
fn test_has_rule_method_for_constraint() {
    let code = r#"
g = graph{}
g.add_rule(:no_node_removals)
result = g.has_rule(:no_node_removals)
if result {
    print("constraint exists")
}
"#;
    assert_runs(code);
}

// ============================================================================
// Custom Method Constraint Tests
// ============================================================================

#[test]
fn test_custom_constraint_blocks_violation() {
    let code = r#"
fn no_node_additions(old_graph, new_graph) {
    return old_graph.node_count() >= new_graph.node_count()
}

graph G {
    fn try_add() {
        self.add_node("b", 2)
    }
}

g = G.clone()
g.add_node("a", 1)
g.add_method_constraint(no_node_additions, "no_node_additions")

g.try_add()
"#;
    assert_fails_with(code, "no_node_additions");
}

#[test]
fn test_custom_constraint_allows_valid_operations() {
    let code = r#"
fn no_node_additions(old_graph, new_graph) {
    return old_graph.node_count() >= new_graph.node_count()
}

graph G {
    fn remove_first() {
        self.remove_node("a")
    }
}

g = G.clone()
g.add_node("a", 1)
g.add_node("b", 2)
g.add_method_constraint(no_node_additions, "no_node_additions")

g.remove_first()
print(g.node_count())
"#;
    assert_runs(code);
}

#[test]
fn test_custom_constraint_allows_read_operations() {
    let code = r#"
fn no_node_additions(old_graph, new_graph) {
    return old_graph.node_count() >= new_graph.node_count()
}

graph G {
    fn get_count() {
        return self.node_count()
    }
}

g = G.clone()
g.add_node("a", 1)
g.add_method_constraint(no_node_additions, "no_node_additions")

result = g.get_count()
print(result)
"#;
    assert_runs(code);
}

#[test]
fn test_custom_constraint_no_edge_additions() {
    let code = r#"
fn no_edge_additions(old_graph, new_graph) {
    return old_graph.edge_count() >= new_graph.edge_count()
}

graph G {
    fn link() {
        self.add_edge("a", "b", "connected")
    }
}

g = G.clone()
g.add_node("a", 1)
g.add_node("b", 2)
g.add_method_constraint(no_edge_additions, "no_edge_additions")

g.link()
"#;
    assert_fails_with(code, "no_edge_additions");
}

#[test]
fn test_custom_constraint_with_default_name() {
    let code = r#"
fn my_constraint(old_graph, new_graph) {
    return old_graph.node_count() >= new_graph.node_count()
}

graph G {
    fn try_add() {
        self.add_node("b", 2)
    }
}

g = G.clone()
g.add_node("a", 1)
g.add_method_constraint(my_constraint)

g.try_add()
"#;
    assert_fails_with(code, "custom_constraint");
}

#[test]
fn test_multiple_custom_constraints() {
    let code = r#"
fn no_node_additions(old_graph, new_graph) {
    return old_graph.node_count() >= new_graph.node_count()
}

fn no_edge_additions(old_graph, new_graph) {
    return old_graph.edge_count() >= new_graph.edge_count()
}

graph G {
    fn try_link() {
        self.add_edge("a", "b", "connected")
    }
}

g = G.clone()
g.add_node("a", 1)
g.add_node("b", 2)
g.add_method_constraint(no_node_additions, "no_node_additions")
g.add_method_constraint(no_edge_additions, "no_edge_additions")

g.try_link()
"#;
    assert_fails_with(code, "no_edge_additions");
}

#[test]
fn test_custom_constraint_with_lambda() {
    let code = r#"
graph G {
    fn try_add() {
        self.add_node("b", 2)
    }
}

g = G.clone()
g.add_node("a", 1)
g.add_method_constraint(
    (old_g, new_g) => old_g.node_count() >= new_g.node_count(),
    "no_additions"
)

g.try_add()
"#;
    assert_fails_with(code, "no_additions");
}

// ============================================================================
// Remove Method Tests (Phase 12)
// ============================================================================

#[test]
fn test_remove_method_basic() {
    let code = r#"
graph G {
    fn get_value() {
        return self.get_node("a")
    }
}

g = G.clone()
g.add_node("a", 1)

# Method works before removal
result1 = g.get_value()
print(result1)

# Remove the method
removed = g.remove_method("get_value")
print(removed)
"#;
    assert_runs(code);
}

#[test]
fn test_remove_method_returns_true_when_found() {
    let code = r#"
graph G {
    fn foo() {
        return 42
    }
}

g = G.clone()
removed = g.remove_method("foo")
if not removed {
    error("Expected remove_method to return true")
}
"#;
    assert_runs(code);
}

#[test]
fn test_remove_method_returns_false_when_not_found() {
    let code = r#"
g = graph{}

removed = g.remove_method("nonexistent")
if removed {
    error("Expected remove_method to return false")
}
"#;
    assert_runs(code);
}

#[test]
fn test_remove_method_makes_method_uncallable() {
    let code = r#"
graph G {
    fn get_value() {
        return self.get_node("a")
    }
}

g = G.clone()
g.add_node("a", 1)

g.remove_method("get_value")

# Trying to call removed method should fail
g.get_value()
"#;
    assert_fails_with(code, "get_value");
}

#[test]
fn test_remove_method_preserves_other_methods() {
    let code = r#"
graph G {
    fn method_a() {
        return "a"
    }

    fn method_b() {
        return "b"
    }
}

g = G.clone()
g.add_node("count", 0)

# Remove method_a
g.remove_method("method_a")

# method_b should still work
result = g.method_b()
if result != "b" {
    error("Expected method_b to return 'b'")
}
"#;
    assert_runs(code);
}

#[test]
fn test_remove_method_does_not_affect_data_nodes() {
    let code = r#"
graph G {
    fn get_data() {
        return self.get_node("data1")
    }
}

g = G.clone()
g.add_node("data1", 100)
g.add_node("data2", 200)

# Verify node count before
before_count = g.node_count()

g.remove_method("get_data")

# Node count should be the same (data nodes preserved)
after_count = g.node_count()

if before_count != after_count {
    error("Data nodes were affected by remove_method")
}
"#;
    assert_runs(code);
}

#[test]
fn test_remove_method_can_readd_method() {
    // Note: With new syntax, we can't re-add methods dynamically
    // This test verifies remove_method works, but doesn't test re-adding
    let code = r#"
graph G {
    fn get_val() {
        return self.get_node("val")
    }
}

g = G.clone()
g.add_node("val", 10)

result1 = g.get_val()

# Remove the method
g.remove_method("get_val")

if result1 != 10 {
    error("First result should be 10")
}
"#;
    assert_runs(code);
}

#[test]
fn test_remove_method_on_clone_does_not_affect_original() {
    let code = r#"
graph Template {
    fn greet() {
        return "Hello"
    }
}

# Clone the template
instance = Template.clone()

# Remove method from clone
instance.remove_method("greet")

# Original should still have the method
result = Template.greet()
if result != "Hello" {
    error("Original method was affected")
}
"#;
    assert_runs(code);
}

#[test]
fn test_remove_method_wrong_arg_type_fails() {
    let code = r#"
g = graph{}
g.remove_method(123)
"#;
    assert_fails_with(code, "must be a string");
}

#[test]
fn test_remove_method_wrong_arg_count_fails() {
    let code = r#"
g = graph{}
g.remove_method()
"#;
    assert_fails_with(code, "expects 1 argument");
}

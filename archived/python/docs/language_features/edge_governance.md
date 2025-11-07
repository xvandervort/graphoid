# Edge Governance in Glang

Glang provides a sophisticated **edge governance system** that ensures safe graph operations while allowing controlled experimentation. This system prevents dangerous operations like circular references and cross-structure contamination by default, while providing tools to override restrictions when needed.

## Core Concepts

### What is Edge Governance?

Edge governance is Glang's **Control Layer** (Layer 3) that validates graph operations before they occur. When you try to create edges between nodes in lists or hashes, the governance system checks active rules and either allows or blocks the operation.

```glang
numbers = [1, 2, 3, 4]

# This works - forward edge is safe
numbers.add_edge(0, 1, "next")

# This is blocked - would create a cycle
numbers.add_edge(3, 0, "cycle")  # Error: Rule 'no_list_cycles' violated
```

### Default Safety Rules

Glang ships with two fundamental safety rules **enabled by default**:

1. **`no_list_cycles`** - Prevents circular references in sequential structures (lists)
2. **`same_structure_only`** - Prevents edges between different graph structures

These rules prevent common data structure corruption scenarios while preserving the flexibility to create complex graph relationships within safe boundaries.

## Configuration Modes

The governance system provides pre-configured modes for different use cases:

### Safe Mode (Default)
Maximum safety with all rules enabled:
```glang
my_list.configure_for_safe_mode()
```

### Experimental Mode
No restrictions - all rules disabled for research:
```glang
my_list.configure_for_experimental_mode()

# Now dangerous operations are allowed
my_list.add_edge(2, 0, "cycle")  # Creates circular reference
```

### List Processing Mode
Optimized for list operations - cycles prevented, cross-structure allowed:
```glang
my_list.configure_for_list_processing()
```

### Tree Structures Mode
Strict hierarchy enforcement:
```glang
my_tree.configure_for_tree_structures()
```

## Edge Inspection Methods

### Check Available Edges
```glang
numbers = [10, 20, 30]
numbers.add_edge(0, 1, "connects")
numbers.add_edge(1, 2, "flows")

# Get all edges as [from_index, to_index, relationship] lists
edges = numbers.get_edges()
print(edges)  # [[0, 1, "connects"], [1, 2, "flows"]]

# Count total edges
count = numbers.get_edge_count()
print(count)  # 2
```

### Validate Edge Operations
```glang
numbers = [1, 2, 3]

# Check if edge can be added (returns true/false)
can_add = numbers.can_add_edge(0, 1, "safe")
print(can_add)  # true

# Try invalid edge (would create cycle)
can_add = numbers.can_add_edge(2, 0, "cycle")
print(can_add)  # false
```

### Hash Edges
```glang
config = { "host": "localhost", "port": 8080 }
config.add_value_edge("host", "port", "connects")

# Get edges between hash values
edges = config.get_edges()
print(edges)  # [["host", "port", "connects"]]

# Check edge validity
can_add = config.can_add_edge("port", "host", "reverse")
print(can_add)  # Depends on current rules
```

## Rule Management

### View Active Rules
```glang
# Get list of currently active rule names
active = my_list.get_active_rules()
print(active)  # ["no_list_cycles", "same_structure_only"]

# Check specific rule status
status = my_list.get_rule_status("no_list_cycles")
print(status)  # "active", "disabled", or "unknown"
```

### Disable/Enable Rules
```glang
# Temporarily disable cycle prevention
my_list.disable_rule("no_list_cycles")

# Now cycles are allowed
my_list.add_edge(2, 0, "cycle")  # Works!

# Re-enable the rule
my_list.enable_rule("no_list_cycles")
```

## Graph Visualization

### Get Graph Summary
```glang
numbers = [1, 2, 3]
numbers.add_edge(0, 1, "next")

summary = numbers.get_graph_summary()
print(summary["type"])        # "list"
print(summary["node_count"])  # 3
print(summary["edge_count"])  # 1
print(summary["active_rules"]) # ["no_list_cycles", "same_structure_only"]
```

### Visualize Structure
```glang
# Text visualization (human readable)
text_viz = numbers.visualize_structure("text")
print(text_viz)
# Graph Structure:
# ========================================
# Type: list
# Nodes: 3
# Edges: 1
# Active Rules: no_list_cycles, same_structure_only
#
# Node Connections:
#   node_0:
#     → node_1 (next)
#   node_1: (no outgoing edges)
#   node_2: (no outgoing edges)

# DOT format (for Graphviz)
dot_viz = numbers.visualize_structure("dot")
print(dot_viz)
# digraph GraphStructure {
#   rankdir=LR;
#   node [shape=box];
#   "node_0";
#   "node_1";
#   "node_2";
#   "node_0" -> "node_1" [label="next"];
# }

# Summary format (compact)
summary_viz = numbers.visualize_structure("summary")
print(summary_viz)
# [LIST] 3 nodes, 1 edges
# Rules: no_list_cycles, same_structure_only
```

## Configuration Analysis

### Get Current Configuration Status
```glang
status = my_list.get_configuration_status()
print(status["mode"])         # "safe", "experimental", "list_processing", etc.
print(status["description"]) # "Maximum safety - all rules enabled"
print(status["recommendations"]) # Suggestions for optimization
```

The system intelligently detects your current configuration and provides recommendations for different use cases.

## Safety Patterns

### Research Mode Pattern
```glang
# Save current configuration
original_rules = data.get_active_rules()

# Switch to experimental mode for research
data.configure_for_experimental_mode()

# Perform experimental operations
data.add_edge(2, 0, "experimental_cycle")

# Restore safe configuration
data.configure_for_safe_mode()
```

### Conditional Edge Creation
```glang
func safe_add_edge(structure, from_idx, to_idx, relationship) {
    can_add = structure.can_add_edge(from_idx, to_idx, relationship)
    if can_add {
        structure.add_edge(from_idx, to_idx, relationship)
        return true
    } else {
        print("Edge blocked by governance rules")
        return false
    }
}
```

## Best Practices

1. **Start Safe**: Use default safe mode for production code
2. **Experiment Safely**: Use experimental mode only for research, restore to safe mode afterward
3. **Check Before Adding**: Use `can_add_edge()` to validate operations before attempting them
4. **Visualize Complex Structures**: Use visualization methods to understand graph topology
5. **Document Rule Changes**: When disabling rules, document why and ensure proper restoration

## Error Messages

When edge operations are blocked, you'll see descriptive error messages:

```glang
# Attempting to create a cycle
numbers.add_edge(2, 0, "cycle")
# Error: Rule 'no_list_cycles' violated: Edge from index 2 to 0 would create a cycle in sequential structure

# Attempting cross-structure contamination
list1.add_edge_to_external(list2_node, "cross")
# Error: Rule 'same_structure_only' violated: Cannot create edges between nodes from different graph structures
```

The edge governance system makes Glang's graph operations both powerful and safe, giving you the tools to build complex data structures with confidence.
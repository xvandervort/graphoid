# Chapter 6: Graph Operations

Graphs are the heart of Graphoid. In this chapter, we'll explore advanced graph operations, algorithms, rules, and behaviors that make Graphoid unique.

## Graph Basics Review

```graphoid
# Create a directed graph
g = graph { type: :directed }

# Add nodes
g.add_node("A", 100)
g.add_node("B", 200)
g.add_node("C", 300)

# Add edges
g.add_edge("A", "B", "depends_on")
g.add_edge("B", "C", "depends_on")
```

## Graph Algorithms

### Traversal Algorithms

#### Depth-First Search (DFS)

```graphoid
g = graph{}
g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
g.add_node("D", 4)
g.add_edge("A", "B")
g.add_edge("A", "C")
g.add_edge("B", "D")

# DFS from node A
result = g.dfs("A")
print(result)  # ["A", "B", "D", "C"] (or another valid DFS order)
```

#### Breadth-First Search (BFS)

```graphoid
# BFS from node A
result = g.bfs("A")
print(result)  # ["A", "B", "C", "D"]
```

### Shortest Path Algorithms

#### Dijkstra's Algorithm (Weighted Graphs)

```graphoid
g = graph { type: :directed }

# Build a weighted graph (distances between cities)
g.add_node("Boston", 0)
g.add_node("NYC", 0)
g.add_node("Philly", 0)
g.add_node("DC", 0)

g.add_edge("Boston", "NYC", "road", 215)
g.add_edge("NYC", "Philly", "road", 95)
g.add_edge("Philly", "DC", "road", 140)
g.add_edge("Boston", "DC", "road", 440)

# Find shortest path from Boston to DC
path = g.shortest_path("Boston", "DC")
print(path)  # ["Boston", "NYC", "Philly", "DC"]

# Get path distance
distance = g.path_distance(path)
print(distance)  # 450 (215 + 95 + 140)
```

#### Unweighted Shortest Path (BFS)

```graphoid
g = graph{}
g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
g.add_node("D", 4)
g.add_edge("A", "B")
g.add_edge("B", "C")
g.add_edge("A", "D")
g.add_edge("D", "C")

# Find shortest path (fewest edges)
path = g.shortest_path("A", "C")
print(path)  # ["A", "D", "C"] or ["A", "B", "C"]
```

### Cycle Detection

```graphoid
g = graph { type: :directed }
g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
g.add_edge("A", "B")
g.add_edge("B", "C")
g.add_edge("C", "A")  # Creates a cycle!

has_cycle = g.has_cycle()
print(has_cycle)  # true

cycle = g.find_cycle()
print(cycle)  # ["A", "B", "C", "A"]
```

### Topological Sort (DAGs)

For Directed Acyclic Graphs, find a linear ordering:

```graphoid
dag = graph { type: :dag }

# Build task dependency graph
dag.add_node("wake_up", 1)
dag.add_node("shower", 2)
dag.add_node("breakfast", 3)
dag.add_node("work", 4)

dag.add_edge("wake_up", "shower")
dag.add_edge("wake_up", "breakfast")
dag.add_edge("shower", "work")
dag.add_edge("breakfast", "work")

# Get valid execution order
order = dag.topological_sort()
print(order)
# ["wake_up", "shower", "breakfast", "work"]
# (or another valid ordering)
```

### Connected Components

```graphoid
g = graph { type: :undirected }

# Add disconnected components
g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
g.add_edge("A", "B")

g.add_node("D", 4)
g.add_node("E", 5)
g.add_edge("D", "E")

# Find connected components
components = g.connected_components()
print(components)
# [["A", "B", "C"], ["D", "E"]]
```

### Minimum Spanning Tree

```graphoid
g = graph { type: :undirected }

g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
g.add_node("D", 4)

g.add_edge("A", "B", "road", 4)
g.add_edge("A", "C", "road", 2)
g.add_edge("B", "C", "road", 1)
g.add_edge("B", "D", "road", 5)
g.add_edge("C", "D", "road", 8)

# Kruskal's or Prim's algorithm
mst = g.minimum_spanning_tree()
print(mst.edges())
# Edges with total weight = 12
```

## Graph Rules

Rules enforce constraints on graph structure. Graphoid can automatically validate these rules.

### Built-in Rules

#### No Cycles

```graphoid
dag = graph { type: :dag }
dag.add_rule("no_cycles")

dag.add_node("A", 1)
dag.add_node("B", 2)
dag.add_edge("A", "B")
# dag.add_edge("B", "A")  # Error: Would create a cycle
```

#### Single Root

```graphoid
tree = tree{}
tree.add_rule("single_root")

# Tree operations automatically enforce single root
```

#### Max Children

```graphoid
binary_tree = tree{}
binary_tree.add_rule("max_children", 2)

# Each node can have at most 2 children
```

#### Max Degree

```graphoid
g = graph{}
g.add_rule("max_degree", 3)

# Each node can have at most 3 edges
```

### Custom Rules

Define your own validation rules:

```graphoid
fn validate_positive_values(graph) {
    for node in graph.nodes() {
        value = graph.get_value(node)
        if value < 0 {
            return false
        }
    }
    return true
}

g = graph{}
g.add_rule(validate_positive_values)

g.add_node("A", 10)   # OK
g.add_node("B", 20)   # OK
# g.add_node("C", -5)  # Error: Rule violation
```

### Rule Policies

Control how rules are enforced:

```graphoid
g = graph{}

# Strict: Reject violations immediately (default)
g.add_rule("no_cycles", :strict)

# Warn: Log warning but allow
g.add_rule("no_cycles", :warn)

# Ignore existing: Only enforce for new changes
g.add_rule("no_cycles", :ignore_existing)
```

## Graph Behaviors

Behaviors automatically transform node or edge values when they're accessed or modified.

### None Handling Behaviors

```graphoid
temperatures = [98.6, none, 102.5, none, 99.1]

# Convert none to zero
temperatures.add_rule("none_to_zero")
print(temperatures)  # [98.6, 0, 102.5, 0, 99.1]
```

### Range Validation

```graphoid
temperatures = [98.6, 110, 102.5]

# Clamp values to valid range (95-105)
temperatures.add_rule("validate_range", 95, 105)
print(temperatures)  # [98.6, 105, 102.5]
```

### Mapping Rules

```graphoid
color_map = {
    "red": 1,
    "green": 2,
    "blue": 3
}

colors = ["red", "blue", "purple", "green"]

# Map colors to numbers, use 0 for unknown
colors.add_mapping_rule(color_map, 0)
print(colors)  # [1, 3, 0, 2]
```

### Custom Behaviors

```graphoid
fn auto_uppercase(value) {
    if value.is_string() {
        return value.to_upper()
    }
    return value
}

names = ["alice", "bob", "charlie"]
names.add_behavior(auto_uppercase)

print(names)  # ["ALICE", "BOB", "CHARLIE"]
```

### Behavior Chaining

```graphoid
data = [1, none, 3, none, 5]

# Apply multiple behaviors
data.add_rule("none_to_zero")
data.add_rule("validate_range", 0, 10)
data.add_behavior(x => x * 2)

print(data)  # [2, 0, 6, 0, 10]
```

## Graph Queries

### Finding Nodes

```graphoid
g = graph{}
g.add_node("Alice", {"age": 30, "city": "Boston"})
g.add_node("Bob", {"age": 25, "city": "NYC"})
g.add_node("Charlie", {"age": 35, "city": "Boston"})

# Find nodes by predicate
boston_people = g.find_nodes(node => {
    value = g.get_value(node)
    return value["city"] == "Boston"
})

print(boston_people)  # ["Alice", "Charlie"]
```

### Finding Paths

```graphoid
# Find all paths between two nodes
paths = g.find_all_paths("A", "D")

# Find paths with maximum length
short_paths = g.find_paths("A", "D", max_length: 3)
```

### Subgraphs

```graphoid
# Extract subgraph containing specific nodes
nodes = ["A", "B", "C"]
subgraph = g.subgraph(nodes)

# Extract subgraph matching predicate
subgraph = g.subgraph_where(node => {
    value = g.get_value(node)
    return value > 100
})
```

## Graph Pattern Matching

Match graph structures:

```graphoid
# Match pattern: A -> B -> C
pattern = graph_pattern {
    node "A"
    node "B"
    node "C"
    edge "A" -> "B"
    edge "B" -> "C"
}

matches = g.match_pattern(pattern)
```

### Advanced Patterns

```graphoid
# Match triangle (3 nodes all connected)
pattern = graph_pattern {
    node "X"
    node "Y"
    node "Z"
    edge "X" -> "Y"
    edge "Y" -> "Z"
    edge "Z" -> "X"
}

# Find all triangles in graph
triangles = g.match_pattern(pattern)
```

## Graph Metrics

### Basic Metrics

```graphoid
g = graph{}
# ... build graph ...

# Number of nodes
node_count = g.node_count()

# Number of edges
edge_count = g.edge_count()

# Density (edges / possible edges)
density = g.density()

# Degree of a node
degree = g.degree("A")
in_degree = g.in_degree("A")
out_degree = g.out_degree("A")
```

### Centrality Measures

```graphoid
# How "central" is each node?

# Degree centrality
centrality = g.degree_centrality()

# Betweenness centrality
betweenness = g.betweenness_centrality()

# Closeness centrality
closeness = g.closeness_centrality()
```

## Practical Examples

### Dependency Resolution

```graphoid
deps = graph { type: :dag }

# Define package dependencies
deps.add_node("app", "1.0")
deps.add_node("web_framework", "2.0")
deps.add_node("database", "3.0")
deps.add_node("logger", "1.5")

deps.add_edge("app", "web_framework")
deps.add_edge("app", "database")
deps.add_edge("web_framework", "logger")
deps.add_edge("database", "logger")

# Get installation order
install_order = deps.topological_sort()
print(install_order)
# ["logger", "web_framework", "database", "app"]
```

### Social Network Analysis

```graphoid
social = graph { type: :undirected }

# Add people
social.add_node("Alice", {"interests": ["coding", "music"]})
social.add_node("Bob", {"interests": ["sports", "music"]})
social.add_node("Charlie", {"interests": ["coding", "gaming"]})

# Add friendships
social.add_edge("Alice", "Bob", "friend")
social.add_edge("Alice", "Charlie", "friend")

# Find mutual friends
alice_friends = social.neighbors("Alice")
bob_friends = social.neighbors("Bob")
mutual = alice_friends.filter(f => bob_friends.contains(f))

# Find people with common interests
fn common_interests(person1, person2) {
    p1 = social.get_value(person1)
    p2 = social.get_value(person2)
    common = []
    for interest in p1["interests"] {
        if p2["interests"].contains(interest) {
            common.append(interest)
        }
    }
    return common
}

common = common_interests("Alice", "Bob")
print(common)  # ["music"]
```

### State Machine

```graphoid
fsm = graph { type: :directed }

# Define states
fsm.add_node("idle", {})
fsm.add_node("running", {})
fsm.add_node("paused", {})
fsm.add_node("stopped", {})

# Define transitions
fsm.add_edge("idle", "running", "start")
fsm.add_edge("running", "paused", "pause")
fsm.add_edge("paused", "running", "resume")
fsm.add_edge("running", "stopped", "stop")
fsm.add_edge("paused", "stopped", "stop")

# Check valid transitions
current = "running"
valid_transitions = fsm.neighbors(current)
print(valid_transitions)  # ["paused", "stopped"]
```

## Best Practices

### Choose the Right Graph Type

```graphoid
# Directed: One-way relationships
deps = graph { type: :directed }

# Undirected: Two-way relationships
social = graph { type: :undirected }

# DAG: No cycles allowed
tasks = graph { type: :dag }
```

### Use Rules for Validation

```graphoid
# Enforce constraints automatically
g.add_rule("no_cycles")
g.add_rule("max_degree", 5)
```

### Use Behaviors for Transformations

```graphoid
# Transform data automatically
data.add_rule("none_to_zero")
data.add_behavior(x => x.to_string())
```

### Cache Expensive Computations

```graphoid
# For graphs that don't change often
shortest_paths = {}

fn get_shortest_path(from, to) {
    key = from + "->" + to
    if not shortest_paths.has_key(key) {
        shortest_paths[key] = g.shortest_path(from, to)
    }
    return shortest_paths[key]
}
```

## Summary

In this chapter, you learned:

- ✅ **Graph Algorithms**: DFS, BFS, shortest path, cycle detection, topological sort
- ✅ **Graph Rules**: Built-in and custom validation rules
- ✅ **Graph Behaviors**: Automatic value transformations
- ✅ **Graph Queries**: Finding nodes, paths, subgraphs
- ✅ **Pattern Matching**: Matching graph structures
- ✅ **Graph Metrics**: Centrality, density, degree
- ✅ **Practical Examples**: Dependencies, social networks, state machines

---

## Quick Reference

```graphoid
# Algorithms
g.dfs("start")
g.bfs("start")
g.shortest_path("from", "to")
g.has_cycle()
g.topological_sort()

# Rules
g.add_rule("no_cycles")
g.add_rule(custom_validator)

# Behaviors
data.add_rule("none_to_zero")
data.add_behavior(transformer)

# Queries
g.find_nodes(predicate)
g.subgraph(nodes)
g.match_pattern(pattern)
```

---

## Exercises

1. **Graph Builder**: Create a function that builds a graph from an adjacency list

2. **Cycle Finder**: Write a function that finds all cycles in a directed graph

3. **Six Degrees**: Implement "six degrees of separation" - find if two nodes are connected within N hops

4. **Graph Isomorphism**: Write a function to check if two graphs are isomorphic

5. **Strongly Connected Components**: Find all strongly connected components in a directed graph

6. **Critical Path**: Given a project task graph with durations, find the critical path

**Solutions** are available in `examples/06-graph-operations/exercises.gr`

---

[← Previous: Collections](05-collections.md) | [Next: Modules →](07-modules.md)

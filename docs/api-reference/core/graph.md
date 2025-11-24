# graph - Graph Data Structure

Graphs are the heart of Graphoid. Everything in Graphoid is ultimately a graph - lists are linked graphs, hashes are graphs with key-value edges, and even variables form a meta-graph. This reference covers explicit graph operations.

## Graph Literals

### Basic Graphs

Graphs are created using the `graph{}` literal:

```graphoid
# Empty graph
g = graph{}

# Directed graph (default)
directed = graph { type: :directed }

# Undirected graph
undirected = graph { type: :undirected }

# DAG (Directed Acyclic Graph)
dag = graph { type: :dag }

# Weighted graph
weighted = graph { type: :weighted }
```

### Graph Types

| Type | Description | Use Case |
|------|-------------|----------|
| `:directed` | Edges have direction | Dependencies, hierarchies, workflows |
| `:undirected` | Edges are bidirectional | Social networks, maps, connections |
| `:dag` | No cycles allowed | Task scheduling, inheritance, compilation |
| `:weighted` | Edges have weights | Distances, costs, priorities |

---

## Node Operations

### add_node(id, value)

Adds a node to the graph.

**Syntax**: `graph.add_node(id, value, metadata)`

**Parameters**:
- `id` (string): Unique node identifier
- `value`: Node value (any type)
- `metadata` (hash, optional): Additional node properties

**Returns**: none (modifies graph in place)

**Examples**:
```graphoid
g = graph{}

# Add nodes
g.add_node("A", 100)
g.add_node("B", 200)
g.add_node("C", 300)

# Add node with metadata
g.add_node("D", 400, {"color": "red", "size": "large"})

# Nodes with complex values
users = graph{}
users.add_node("alice", {"name": "Alice", "age": 30})
users.add_node("bob", {"name": "Bob", "age": 25})
```

**Errors**: Adding duplicate node ID raises error

**See also**: `remove_node()`, `has_node()`

---

### remove_node(id)

Removes a node and all its connected edges.

**Syntax**: `graph.remove_node(id)`

**Parameters**:
- `id` (string): Node identifier

**Returns**: (bool) `true` if removed, `false` if not found

**Examples**:
```graphoid
g = graph{}
g.add_node("A", 100)
g.add_node("B", 200)
g.add_edge("A", "B")

removed = g.remove_node("A")
print(removed)  # true

# Edge A->B is also removed
print(g.edges())  # []

removed = g.remove_node("Z")
print(removed)  # false (doesn't exist)
```

**See also**: `add_node()`, `clear()`

---

### has_node(id)

Tests if a node exists.

**Syntax**: `graph.has_node(id)`

**Parameters**:
- `id` (string): Node identifier

**Returns**: (bool) `true` if exists, `false` otherwise

**Examples**:
```graphoid
g = graph{}
g.add_node("A", 100)

result = g.has_node("A")  # true
result = g.has_node("Z")  # false

# Conditional operations
if not g.has_node("A") {
    g.add_node("A", 100)
}
```

**See also**: `add_node()`, `nodes()`

---

### get_value(id)

Gets a node's value.

**Syntax**: `graph.get_value(id)`

**Parameters**:
- `id` (string): Node identifier

**Returns**: Node value

**Examples**:
```graphoid
g = graph{}
g.add_node("A", 100)
g.add_node("B", 200)

value = g.get_value("A")
print(value)  # 100

# With complex values
users = graph{}
users.add_node("alice", {"name": "Alice", "age": 30})
user_data = users.get_value("alice")
print(user_data["name"])  # "Alice"
```

**Errors**: Non-existent node raises error

**See also**: `set_value()`, `add_node()`

---

### set_value(id, value)

Sets a node's value.

**Syntax**: `graph.set_value(id, value)`

**Parameters**:
- `id` (string): Node identifier
- `value`: New value

**Returns**: none (modifies graph in place)

**Examples**:
```graphoid
g = graph{}
g.add_node("A", 100)
g.set_value("A", 200)
print(g.get_value("A"))  # 200

# Update complex values
users.set_value("alice", {"name": "Alice", "age": 31})
```

**Errors**: Non-existent node raises error

**See also**: `get_value()`, `update_node()`

---

### nodes()

Returns a list of all node IDs.

**Syntax**: `graph.nodes()`

**Returns**: (list) List of node IDs

**Examples**:
```graphoid
g = graph{}
g.add_node("A", 100)
g.add_node("B", 200)
g.add_node("C", 300)

node_list = g.nodes()
print(node_list)  # ["A", "B", "C"]

# Iterate over nodes
for node in g.nodes() {
    value = g.get_value(node)
    print("${node}: ${value}")
}
```

**See also**: `edges()`, `node_count()`

---

### node_count()

Returns the number of nodes.

**Syntax**: `graph.node_count()`

**Returns**: (num) Number of nodes

**Examples**:
```graphoid
g = graph{}
g.add_node("A", 100)
g.add_node("B", 200)

count = g.node_count()
print(count)  # 2
```

**See also**: `nodes()`, `edge_count()`

---

## Edge Operations

### add_edge(from, to, label, weight, metadata)

Adds an edge between two nodes.

**Syntax**: `graph.add_edge(from, to, label, weight, metadata)`

**Parameters**:
- `from` (string): Source node ID
- `to` (string): Target node ID
- `label` (string, optional): Edge label/type
- `weight` (num, optional): Edge weight (for weighted graphs)
- `metadata` (hash, optional): Additional edge properties

**Returns**: none (modifies graph in place)

**Examples**:
```graphoid
g = graph { type: :directed }
g.add_node("A", 1)
g.add_node("B", 2)

# Basic edge
g.add_edge("A", "B")

# Labeled edge
g.add_edge("A", "C", "depends_on")

# Weighted edge
cities = graph { type: :weighted }
cities.add_node("Boston", {})
cities.add_node("NYC", {})
cities.add_edge("Boston", "NYC", "road", 215)

# Edge with metadata
g.add_edge("A", "D", "link", 1.0, {"color": "blue"})
```

**Errors**:
- Non-existent nodes raise error
- DAGs reject edges that create cycles

**See also**: `remove_edge()`, `has_edge()`

---

### remove_edge(from, to, label)

Removes an edge.

**Syntax**: `graph.remove_edge(from, to, label)`

**Parameters**:
- `from` (string): Source node ID
- `to` (string): Target node ID
- `label` (string, optional): Edge label (if multiple edges exist)

**Returns**: (bool) `true` if removed, `false` if not found

**Examples**:
```graphoid
g = graph{}
g.add_node("A", 1)
g.add_node("B", 2)
g.add_edge("A", "B", "link")

removed = g.remove_edge("A", "B")
print(removed)  # true

removed = g.remove_edge("X", "Y")
print(removed)  # false (doesn't exist)

# Remove specific labeled edge
g.add_edge("A", "B", "type1")
g.add_edge("A", "B", "type2")
g.remove_edge("A", "B", "type1")  # Only removes type1 edge
```

**See also**: `add_edge()`, `edges()`

---

### has_edge(from, to, label)

Tests if an edge exists.

**Syntax**: `graph.has_edge(from, to, label)`

**Parameters**:
- `from` (string): Source node ID
- `to` (string): Target node ID
- `label` (string, optional): Edge label

**Returns**: (bool) `true` if exists, `false` otherwise

**Examples**:
```graphoid
g = graph{}
g.add_node("A", 1)
g.add_node("B", 2)
g.add_edge("A", "B")

result = g.has_edge("A", "B")  # true
result = g.has_edge("B", "A")  # false (directed)
result = g.has_edge("A", "C")  # false

# Check before adding
if not g.has_edge("A", "B") {
    g.add_edge("A", "B")
}
```

**See also**: `add_edge()`, `edges()`

---

### edges()

Returns a list of all edges.

**Syntax**: `graph.edges()`

**Returns**: (list) List of edge hashes `{"from": "A", "to": "B", "label": "...", "weight": ...}`

**Examples**:
```graphoid
g = graph{}
g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
g.add_edge("A", "B", "link")
g.add_edge("B", "C", "link")

edge_list = g.edges()
print(edge_list)
# [
#   {"from": "A", "to": "B", "label": "link"},
#   {"from": "B", "to": "C", "label": "link"}
# ]

# Filter edges
links = g.edges().filter(e => e["label"] == "link")
```

**See also**: `nodes()`, `edge_count()`

---

### edge_count()

Returns the number of edges.

**Syntax**: `graph.edge_count()`

**Returns**: (num) Number of edges

**Examples**:
```graphoid
g = graph{}
g.add_node("A", 1)
g.add_node("B", 2)
g.add_edge("A", "B")

count = g.edge_count()
print(count)  # 1
```

**See also**: `edges()`, `node_count()`

---

### neighbors(id)

Returns IDs of nodes connected to a given node.

**Syntax**: `graph.neighbors(id, direction)`

**Parameters**:
- `id` (string): Node identifier
- `direction` (symbol, optional): `:out` (default), `:in`, or `:both`

**Returns**: (list) List of neighbor node IDs

**Examples**:
```graphoid
g = graph { type: :directed }
g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
g.add_edge("A", "B")
g.add_edge("A", "C")
g.add_edge("C", "A")

# Outgoing neighbors
out = g.neighbors("A")        # ["B", "C"]
out = g.neighbors("A", :out)  # ["B", "C"]

# Incoming neighbors
in = g.neighbors("A", :in)    # ["C"]

# Both directions
both = g.neighbors("A", :both) # ["B", "C"]

# Undirected graph
ug = graph { type: :undirected }
ug.add_node("A", 1)
ug.add_node("B", 2)
ug.add_edge("A", "B")
neighbors = ug.neighbors("A")  # ["B"]
```

**See also**: `degree()`, `edges()`

---

### degree(id)

Returns the degree (number of connected edges) of a node.

**Syntax**: `graph.degree(id, direction)`

**Parameters**:
- `id` (string): Node identifier
- `direction` (symbol, optional): `:out`, `:in`, or `:both` (default)

**Returns**: (num) Degree count

**Examples**:
```graphoid
g = graph { type: :directed }
g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
g.add_edge("A", "B")
g.add_edge("A", "C")
g.add_edge("C", "A")

# Total degree
deg = g.degree("A")      # 3 (2 out + 1 in)

# Out-degree
out = g.degree("A", :out)  # 2
out = g.out_degree("A")    # 2 (alias)

# In-degree
in = g.degree("A", :in)    # 1
in = g.in_degree("A")      # 1 (alias)
```

**See also**: `in_degree()`, `out_degree()`, `neighbors()`

---

### in_degree(id)

Returns the in-degree (incoming edges).

**Syntax**: `graph.in_degree(id)`

**Parameters**:
- `id` (string): Node identifier

**Returns**: (num) In-degree count

**Examples**:
```graphoid
g = graph { type: :directed }
g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
g.add_edge("B", "A")
g.add_edge("C", "A")

in_deg = g.in_degree("A")
print(in_deg)  # 2
```

**See also**: `out_degree()`, `degree()`

---

### out_degree(id)

Returns the out-degree (outgoing edges).

**Syntax**: `graph.out_degree(id)`

**Parameters**:
- `id` (string): Node identifier

**Returns**: (num) Out-degree count

**Examples**:
```graphoid
g = graph { type: :directed }
g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
g.add_edge("A", "B")
g.add_edge("A", "C")

out_deg = g.out_degree("A")
print(out_deg)  # 2
```

**See also**: `in_degree()`, `degree()`

---

## Graph Algorithms

### dfs(start)

Depth-First Search traversal.

**Syntax**: `graph.dfs(start, visitor)`

**Parameters**:
- `start` (string): Starting node ID
- `visitor` (function, optional): Callback function for each node

**Returns**: (list) List of node IDs in DFS order

**Examples**:
```graphoid
g = graph{}
g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
g.add_node("D", 4)
g.add_edge("A", "B")
g.add_edge("A", "C")
g.add_edge("B", "D")

result = g.dfs("A")
print(result)  # ["A", "B", "D", "C"] or similar

# With visitor
g.dfs("A", node => {
    print("Visiting: " + node)
})
```

**See also**: `bfs()`, `topological_sort()`

---

### bfs(start)

Breadth-First Search traversal.

**Syntax**: `graph.bfs(start, visitor)`

**Parameters**:
- `start` (string): Starting node ID
- `visitor` (function, optional): Callback function for each node

**Returns**: (list) List of node IDs in BFS order

**Examples**:
```graphoid
g = graph{}
g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
g.add_node("D", 4)
g.add_edge("A", "B")
g.add_edge("A", "C")
g.add_edge("B", "D")

result = g.bfs("A")
print(result)  # ["A", "B", "C", "D"]

# Level-order traversal
g.bfs("A", node => {
    print("Visiting: " + node)
})
```

**See also**: `dfs()`, `shortest_path()`

---

### shortest_path(from, to)

Finds shortest path between two nodes.

**Syntax**: `graph.shortest_path(from, to, edge_type, weighted)`

**Parameters**:
- `from` (string): Source node ID
- `to` (string): Target node ID
- `edge_type` (string, optional): Consider only edges of this type
- `weighted` (bool, optional): Use edge weights (default: false)

**Returns**: (list) List of node IDs representing the path, or `none` if no path exists

**Examples**:
```graphoid
# Unweighted shortest path (BFS)
g = graph{}
g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
g.add_node("D", 4)
g.add_edge("A", "B")
g.add_edge("B", "C")
g.add_edge("A", "D")
g.add_edge("D", "C")

path = g.shortest_path("A", "C")
print(path)  # ["A", "D", "C"] or ["A", "B", "C"]

# Weighted shortest path (Dijkstra's algorithm)
cities = graph { type: :weighted }
cities.add_node("Boston", {})
cities.add_node("NYC", {})
cities.add_node("Philly", {})
cities.add_node("DC", {})
cities.add_edge("Boston", "NYC", "road", 215)
cities.add_edge("NYC", "Philly", "road", 95)
cities.add_edge("Philly", "DC", "road", 140)
cities.add_edge("Boston", "DC", "road", 440)

path = cities.shortest_path("Boston", "DC", none, true)
print(path)  # ["Boston", "NYC", "Philly", "DC"]
```

**See also**: `bfs()`, `path_distance()`

---

### path_distance(path)

Calculates total distance/weight of a path.

**Syntax**: `graph.path_distance(path)`

**Parameters**:
- `path` (list): List of node IDs representing a path

**Returns**: (num) Total weight

**Examples**:
```graphoid
cities = graph { type: :weighted }
cities.add_node("Boston", {})
cities.add_node("NYC", {})
cities.add_node("DC", {})
cities.add_edge("Boston", "NYC", "road", 215)
cities.add_edge("NYC", "DC", "road", 225)

path = ["Boston", "NYC", "DC"]
distance = cities.path_distance(path)
print(distance)  # 440 (215 + 225)
```

**See also**: `shortest_path()`

---

### has_cycle()

Tests if the graph contains a cycle.

**Syntax**: `graph.has_cycle()`

**Returns**: (bool) `true` if cycle exists, `false` otherwise

**Examples**:
```graphoid
g = graph { type: :directed }
g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
g.add_edge("A", "B")
g.add_edge("B", "C")

print(g.has_cycle())  # false

g.add_edge("C", "A")  # Creates cycle
print(g.has_cycle())  # true
```

**See also**: `find_cycle()`, `topological_sort()`

---

### find_cycle()

Finds a cycle in the graph.

**Syntax**: `graph.find_cycle()`

**Returns**: (list) List of node IDs forming a cycle, or `none` if no cycle exists

**Examples**:
```graphoid
g = graph { type: :directed }
g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
g.add_edge("A", "B")
g.add_edge("B", "C")
g.add_edge("C", "A")

cycle = g.find_cycle()
print(cycle)  # ["A", "B", "C", "A"]
```

**See also**: `has_cycle()`

---

### topological_sort()

Returns a topological ordering of nodes (DAGs only).

**Syntax**: `graph.topological_sort()`

**Returns**: (list) List of node IDs in topological order

**Examples**:
```graphoid
dag = graph { type: :dag }
dag.add_node("wake_up", 1)
dag.add_node("shower", 2)
dag.add_node("breakfast", 3)
dag.add_node("work", 4)
dag.add_edge("wake_up", "shower")
dag.add_edge("wake_up", "breakfast")
dag.add_edge("shower", "work")
dag.add_edge("breakfast", "work")

order = dag.topological_sort()
print(order)
# ["wake_up", "shower", "breakfast", "work"]
# (or another valid topological ordering)
```

**Errors**: Graphs with cycles cannot be topologically sorted

**See also**: `has_cycle()`, `dfs()`

---

### connected_components()

Finds connected components (undirected graphs).

**Syntax**: `graph.connected_components()`

**Returns**: (list) List of components, where each component is a list of node IDs

**Examples**:
```graphoid
g = graph { type: :undirected }
g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
g.add_edge("A", "B")

g.add_node("D", 4)
g.add_node("E", 5)
g.add_edge("D", "E")

components = g.connected_components()
print(components)
# [["A", "B", "C"], ["D", "E"]]
```

**See also**: `strongly_connected_components()`

---

### strongly_connected_components()

Finds strongly connected components (directed graphs).

**Syntax**: `graph.strongly_connected_components()`

**Returns**: (list) List of components, where each component is a list of node IDs

**Examples**:
```graphoid
g = graph { type: :directed }
# Add nodes and edges forming SCCs
g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
g.add_edge("A", "B")
g.add_edge("B", "C")
g.add_edge("C", "A")  # SCC: A-B-C

sccs = g.strongly_connected_components()
print(sccs)
# [["A", "B", "C"]]
```

**See also**: `connected_components()`

---

### minimum_spanning_tree()

Finds minimum spanning tree (undirected weighted graphs).

**Syntax**: `graph.minimum_spanning_tree(algorithm)`

**Parameters**:
- `algorithm` (symbol, optional): `:kruskal` (default) or `:prim`

**Returns**: (graph) New graph representing the MST

**Examples**:
```graphoid
g = graph { type: :undirected, weighted: true }
g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
g.add_node("D", 4)
g.add_edge("A", "B", "edge", 4)
g.add_edge("A", "C", "edge", 2)
g.add_edge("B", "C", "edge", 1)
g.add_edge("B", "D", "edge", 5)
g.add_edge("C", "D", "edge", 8)

mst = g.minimum_spanning_tree()
print(mst.edges())
# Edges with minimum total weight
```

**See also**: `shortest_path()`

---

## Graph Metrics

### density()

Calculates graph density (ratio of actual edges to possible edges).

**Syntax**: `graph.density()`

**Returns**: (num) Density value (0.0 to 1.0)

**Examples**:
```graphoid
g = graph{}
g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
g.add_edge("A", "B")
g.add_edge("B", "C")
g.add_edge("C", "A")

# 3 edges out of possible 6 (for 3 nodes)
density = g.density()
print(density)  # 0.5
```

**See also**: `node_count()`, `edge_count()`

---

### degree_centrality()

Calculates degree centrality for all nodes.

**Syntax**: `graph.degree_centrality()`

**Returns**: (hash) Hash of node IDs to centrality scores

**Examples**:
```graphoid
g = graph { type: :undirected }
g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
g.add_node("D", 4)
g.add_edge("A", "B")
g.add_edge("A", "C")
g.add_edge("A", "D")

centrality = g.degree_centrality()
print(centrality)
# {"A": 1.0, "B": 0.33, "C": 0.33, "D": 0.33}
# A is most central (connected to all)
```

**See also**: `betweenness_centrality()`, `closeness_centrality()`

---

### betweenness_centrality()

Calculates betweenness centrality (how often node appears on shortest paths).

**Syntax**: `graph.betweenness_centrality()`

**Returns**: (hash) Hash of node IDs to centrality scores

**Examples**:
```graphoid
g = graph{}
# Build graph...
centrality = g.betweenness_centrality()
# Nodes with high scores are "bridges"
```

**See also**: `degree_centrality()`, `closeness_centrality()`

---

### closeness_centrality()

Calculates closeness centrality (average distance to all other nodes).

**Syntax**: `graph.closeness_centrality()`

**Returns**: (hash) Hash of node IDs to centrality scores

**Examples**:
```graphoid
g = graph{}
# Build graph...
centrality = g.closeness_centrality()
# Nodes with high scores are "closer" to everything
```

**See also**: `degree_centrality()`, `betweenness_centrality()`

---

## Graph Rules

### add_rule(rule)

Adds a validation rule to the graph.

**Syntax**: `graph.add_rule(rule, ...args, policy)`

**Parameters**:
- `rule` (string or function): Rule name or validation function
- `args`: Additional arguments for the rule
- `policy` (symbol, optional): `:strict` (default), `:warn`, or `:ignore_existing`

**Returns**: none

**Examples**:
```graphoid
# No cycles
dag = graph { type: :dag }
dag.add_rule("no_cycles")

# Maximum degree
g = graph{}
g.add_rule("max_degree", 3)

# Custom validation
fn validate_positive_values(graph) {
    for node in graph.nodes() {
        if graph.get_value(node) < 0 {
            return false
        }
    }
    return true
}

numbers = graph{}
numbers.add_rule(validate_positive_values)
```

**Built-in Rules**:
- `"no_cycles"` - Prevent cycles
- `"max_degree"` - Limit edges per node
- `"connected"` - Ensure graph is connected
- `"acyclic"` - Same as no_cycles
- `"simple"` - No self-loops or parallel edges

**See also**: [User Guide: Graph Rules](../../user-guide/06-graph-operations.md#graph-rules)

---

## Graph Pattern Matching

### match_pattern(pattern)

Finds subgraphs matching a pattern.

**Syntax**: `graph.match_pattern(pattern)`

**Parameters**:
- `pattern`: Graph pattern to match

**Returns**: (list) List of matches

**Examples**:
```graphoid
# Match triangle pattern
pattern = graph_pattern {
    node "X"
    node "Y"
    node "Z"
    edge "X" -> "Y"
    edge "Y" -> "Z"
    edge "Z" -> "X"
}

matches = g.match_pattern(pattern)
# Returns all triangles in graph
```

**See also**: [User Guide: Graph Pattern Matching](../../user-guide/06-graph-operations.md#graph-pattern-matching)

---

## Utility Methods

### clear()

Removes all nodes and edges.

**Syntax**: `graph.clear()`

**Returns**: none (modifies graph in place)

**Examples**:
```graphoid
g = graph{}
g.add_node("A", 1)
g.add_node("B", 2)
g.add_edge("A", "B")

g.clear()
print(g.node_count())  # 0
print(g.edge_count())  # 0
```

**See also**: `remove_node()`, `remove_edge()`

---

### copy()

Creates a copy of the graph.

**Syntax**: `graph.copy()`

**Returns**: (graph) New graph with same structure

**Examples**:
```graphoid
g = graph{}
g.add_node("A", 1)
g.add_node("B", 2)
g.add_edge("A", "B")

copy = g.copy()
copy.add_node("C", 3)

print(g.node_count())     # 2 (original unchanged)
print(copy.node_count())  # 3
```

**See also**: `subgraph()`

---

### subgraph(nodes)

Extracts a subgraph containing specified nodes.

**Syntax**: `graph.subgraph(nodes)`

**Parameters**:
- `nodes` (list): List of node IDs to include

**Returns**: (graph) New graph with only specified nodes and edges between them

**Examples**:
```graphoid
g = graph{}
g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
g.add_node("D", 4)
g.add_edge("A", "B")
g.add_edge("B", "C")
g.add_edge("C", "D")

sub = g.subgraph(["A", "B", "C"])
print(sub.nodes())  # ["A", "B", "C"]
print(sub.edges())  # Edges between A, B, C
```

**See also**: `copy()`, `subgraph_where()`

---

### subgraph_where(predicate)

Extracts a subgraph with nodes matching a condition.

**Syntax**: `graph.subgraph_where(predicate)`

**Parameters**:
- `predicate`: Test function `(node_id, value) => bool`

**Returns**: (graph) New graph with matching nodes

**Examples**:
```graphoid
g = graph{}
g.add_node("A", 100)
g.add_node("B", 50)
g.add_node("C", 150)
g.add_edge("A", "B")
g.add_edge("B", "C")

sub = g.subgraph_where((id, value) => value > 75)
print(sub.nodes())  # ["A", "C"]
```

**See also**: `subgraph()`, `find_nodes()`

---

### find_nodes(predicate)

Finds nodes matching a condition.

**Syntax**: `graph.find_nodes(predicate)`

**Parameters**:
- `predicate`: Test function `(node_id, value) => bool`

**Returns**: (list) List of matching node IDs

**Examples**:
```graphoid
users = graph{}
users.add_node("alice", {"age": 30, "city": "Boston"})
users.add_node("bob", {"age": 25, "city": "NYC"})
users.add_node("charlie", {"age": 35, "city": "Boston"})

boston = users.find_nodes((id, value) => value["city"] == "Boston")
print(boston)  # ["alice", "charlie"]
```

**See also**: `subgraph_where()`, `filter()`

---

## Type Checking

### is_graph()

Tests if a value is a graph.

**Syntax**: `value.is_graph()`

**Returns**: (bool) `true` if graph, `false` otherwise

**Examples**:
```graphoid
result = graph{}.is_graph()     # true
result = [1, 2, 3].is_graph()   # false

# Type validation
if not value.is_graph() {
    print("Expected graph")
}
```

**See also**: `is_list()`, `is_hash()`, `is_tree()`

---

## See Also

- [list](list.md) - Lists are internally graphs
- [hash](hash.md) - Hashes are internally graphs
- [tree](tree.md) - Trees are constrained graphs
- [User Guide: Graph Operations](../../user-guide/06-graph-operations.md) - Graph algorithms and patterns
- [User Guide: Collections](../../user-guide/05-collections.md) - Graph-backed collections

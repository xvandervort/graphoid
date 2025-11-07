# List to Graph Conversion Plan

## Overview

This document outlines a rough plan for converting linear lists into general graphs where nodes can have multiple successors (like trees) and potentially multiple predecessors. This is a fundamental shift from linear structures to true graph structures.

**Status**: DRAFT - Needs refinement before implementation

## Core Concepts

### Current State
```bash
# Linear list - each node has at most one successor
list fruits = [apple, banana, cherry]
# Internal: apple → banana → cherry
```

### Proposed Graph Structure
```bash
# Graph where nodes can have multiple connections
graph network = from_list([alice, bob, charlie, david])
network.connect(alice, [bob, charlie])  # alice connects to both bob and charlie
network.connect(bob, david)
network.connect(charlie, david)         # david has two predecessors

# Visual representation:
#     alice
#    /      \
#   bob    charlie
#    \      /
#     david
```

## Notation Design

### Option 1: Method-Based Building
```bash
# Start with a list, convert to graph, then add edges
list people = [alice, bob, charlie, david]
graph network = people.to_graph()
network.add_edge(alice, bob)
network.add_edge(alice, charlie)
network.add_edge(bob, david)
network.add_edge(charlie, david)
```

### Option 2: Declaration Syntax
```bash
# Declare connections inline using arrow notation
graph network = {
    alice -> [bob, charlie],
    bob -> david,
    charlie -> david
}
```

### Option 3: Builder Pattern
```bash
# Build incrementally
graph network = new_graph()
network.add(alice)
network.add(bob) 
network.add(charlie)
network.add(david)
network.alice -> [bob, charlie]  # Property-style edge creation
network.bob -> david
network.charlie -> david
```

### Option 4: Edge List Syntax
```bash
# Define nodes and edges separately
graph network = {
    nodes: [alice, bob, charlie, david],
    edges: [
        (alice, bob),
        (alice, charlie),
        (bob, david),
        (charlie, david)
    ]
}
```

## Display Strategies

### 1. Adjacency List Display (Default)
```bash
glang> network
Graph 'network' (4 nodes, 4 edges):
  alice -> [bob, charlie]
  bob -> [david]
  charlie -> [david]
  david -> []
```

### 2. ASCII Art Tree Display
```bash
glang> network --tree
    alice
    ├── bob
    │   └── david
    └── charlie
        └── david*  (* = already shown)
```

### 3. Matrix Display
```bash
glang> network --matrix
         alice  bob  charlie  david
alice      0     1      1       0
bob        0     0      0       1
charlie    0     0      0       1
david      0     0      0       0
```

### 4. GraphViz DOT Format
```bash
glang> network --dot
digraph network {
    alice -> bob;
    alice -> charlie;
    bob -> david;
    charlie -> david;
}
```

### 5. Path-Based Display
```bash
glang> network --paths
Paths from roots:
  alice -> bob -> david
  alice -> charlie -> david
```

## Directionality Options

### Phase 1: Unidirectional (Directed Graphs)
```bash
# All edges have direction
graph web = directed()
web.add_edge(page1, page2)  # page1 links to page2, not vice versa
```

### Phase 2: Bidirectional Support
```bash
# Undirected graphs - edges work both ways
graph friends = undirected()
friends.add_edge(alice, bob)  # alice knows bob AND bob knows alice

# Mixed graphs (future)
graph mixed = mixed_graph()
mixed.add_directed(alice, bob)    # one-way
mixed.add_undirected(bob, charlie) # two-way
```

## Conversion Methods

### From List to Graph
```python
class Graph:
    @classmethod
    def from_list(cls, lst: List, connection_pattern: str = None):
        """Convert list to graph with optional connection pattern."""
        if connection_pattern == "chain":
            # Default linear chain: a -> b -> c
        elif connection_pattern == "star":
            # First node connects to all others
        elif connection_pattern == "complete":
            # Every node connects to every other
        elif connection_pattern == "tree":
            # Binary tree structure
        else:
            # Just create nodes, no edges
```

### Conversion Examples
```bash
# Linear chain (default)
list items = [a, b, c, d]
graph chain = items.to_graph()  # a -> b -> c -> d

# Star pattern
graph star = items.to_graph("star")  # a -> b, a -> c, a -> d

# Complete graph
graph complete = items.to_graph("complete")  # all interconnected

# Custom connections after conversion
graph custom = items.to_graph()
custom.connect(a, [c, d])  # skip b
```

## Operations on General Graphs

### Navigation
```bash
# Get successors/predecessors
network.successors(alice)    # [bob, charlie]
network.predecessors(david)  # [bob, charlie]

# Check connectivity
network.connected(alice, david)  # true (path exists)
network.distance(alice, david)   # 2 (shortest path length)
```

### Traversal
```bash
# Different traversal strategies
network.bfs()  # Breadth-first search
network.dfs()  # Depth-first search
network.topological_sort()  # If DAG
```

### Modification
```bash
# Add/remove edges
network.add_edge(alice, david)  # Direct connection
network.remove_edge(bob, david)

# Add/remove nodes
network.add_node(eve)
network.remove_node(charlie)  # Also removes connected edges
```

## Implementation Challenges

### 1. Cycle Detection
- Need algorithms to detect cycles for certain operations
- Decide how to handle/display cycles

### 2. Multiple Edges
- Should we allow multiple edges between same nodes?
- How to represent weighted edges?

### 3. Node Identity
- How to handle duplicate values in different nodes?
- Use node IDs vs node values for connections?

### 4. Performance
- Adjacency list vs adjacency matrix internal representation
- Trade-offs for different operation types

### 5. Backwards Compatibility
- Ensure linear lists still work as expected
- Gradual migration path from lists to graphs

## Rough Implementation Timeline

### Phase 1: Basic Directed Graphs (1-2 weeks)
- [ ] Extend Graph class for multiple successors
- [ ] Basic `to_graph()` conversion
- [ ] Simple adjacency list display
- [ ] Add/remove edge operations

### Phase 2: Display & Navigation (1 week)
- [ ] Multiple display formats
- [ ] Traversal algorithms
- [ ] Path finding methods

### Phase 3: Advanced Features (2 weeks)
- [ ] Undirected graph support
- [ ] Weighted edges
- [ ] Cycle detection
- [ ] Graph algorithms (shortest path, etc.)

### Phase 4: Integration (1 week)
- [ ] Update parser for graph syntax
- [ ] Tab completion for graph methods
- [ ] Comprehensive testing
- [ ] Documentation

## Open Questions

1. **Syntax preference**: Which notation style feels most natural?
2. **Default behavior**: What should `list.to_graph()` do with no parameters?
3. **Node references**: Use values or introduce node IDs?
4. **Edge properties**: How to add weights, labels, or other metadata?
5. **Visualization**: What should be the default display format?
6. **Type safety**: How to ensure operations make sense for graph type?
7. **Memory model**: How to efficiently store large graphs?

## Example Use Cases

### Social Network
```bash
graph social = new_graph()
social.add_nodes([alice, bob, charlie, david, eve])
social.add_edges([
    (alice, bob, "friend"),
    (alice, charlie, "colleague"),
    (bob, david, "friend"),
    (charlie, david, "colleague"),
    (david, eve, "friend")
])

# Find connections
social.shortest_path(alice, eve)  # alice -> bob -> david -> eve
social.common_neighbors(bob, charlie)  # [david]
```

### Dependency Graph
```bash
graph deps = {
    nodes: [moduleA, moduleB, moduleC, moduleD],
    edges: [
        moduleA -> [moduleB, moduleC],
        moduleB -> moduleD,
        moduleC -> moduleD
    ]
}

deps.topological_sort()  # [moduleA, moduleB, moduleC, moduleD]
deps.find_cycles()  # None (it's a DAG)
```

### State Machine
```bash
graph states = directed()
states.add_nodes([idle, running, paused, stopped])
states.add_edges([
    (idle, running, "start"),
    (running, paused, "pause"),
    (paused, running, "resume"),
    (running, stopped, "stop"),
    (paused, stopped, "stop")
])

states.reachable_from(idle)  # All states
states.transitions(running)  # [paused, stopped]
```

## Notes for Refinement

**This is a rough draft that needs significant refinement before implementation:**

1. **Syntax decisions**: Need user feedback on preferred notation
2. **Performance analysis**: Benchmark different internal representations
3. **API design**: Finalize method names and signatures
4. **Error handling**: Define behavior for invalid operations
5. **Compatibility matrix**: Ensure all existing features work with graphs
6. **Visual design**: Create better ASCII art algorithms for graph display
7. **Algorithm selection**: Choose optimal algorithms for common operations
8. **Memory optimization**: Design efficient storage for large graphs
9. **Serialization format**: How to save/load graphs
10. **Interop considerations**: How graphs interact with other features

## Related Documents

- [SYNTAX_IMPROVEMENTS_PLAN.md](./SYNTAX_IMPROVEMENTS_PLAN.md) - Overall syntax improvements
- [CLAUDE.md](../CLAUDE.md) - Project overview and philosophy

---

*Last Updated: [Draft Version]*
*Status: Needs refinement before implementation*
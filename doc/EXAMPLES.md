# Glang Examples: Seeing Graphs Everywhere

This document demonstrates glang's "everything is a graph" philosophy through interactive examples.

## Basic Graph Creation and Manipulation

```bash
$ glang
Glang 0.1.0
A prototype programming language that uses graphs as first-class objects
✨ Try: create fruits [apple, banana] then 'namespace' to see the magic! ✨

glang> create fruits [apple, banana, cherry]
Created linear graph 'fruits' with 3 elements

glang> show fruits
Graph 'fruits' (linear):
[apple] -> [banana] -> [cherry]

glang> append orange
Appended orange to 'fruits'

glang> show fruits  
Graph 'fruits' (linear):
[apple] -> [banana] -> [cherry] -> [orange]
```

So far, this looks like a normal list. But here's where it gets interesting...

## The Meta-Graph Revelation

```bash
glang> namespace
Variable Namespace Graph:
  Variables: 1
  Total nodes: 2
  Assignment edges: 1

  📛 fruits -> 📊 linear graph (4 nodes)
```

**🤯 What just happened?**

The `namespace` command revealed that our variable storage itself is a graph:
- `📛 fruits` is a **name node** storing the variable name
- `📊 linear graph` is a **value node** storing the actual graph
- There's an **assignment edge** connecting the name to the value
- The variable system has 2 nodes total (name + value) and 1 edge (assignment)

## Seeing Multiple Variables in the Meta-Graph

```bash
glang> create numbers [1, 2, 3, 4, 5]
Created linear graph 'numbers' with 5 elements

glang> create empty directed
Created directed graph 'empty'

glang> graphs
Available graphs:
   fruits: linear (4 nodes)
   numbers: linear (5 nodes)  
 * empty: undefined

glang> namespace
Variable Namespace Graph:
  Variables: 3
  Total nodes: 6
  Assignment edges: 3

  📛 fruits -> 📊 linear graph (4 nodes)
  📛 numbers -> 📊 linear graph (5 nodes)
  📛 empty -> 📊 directed graph (0 nodes)
```

Now we can see the **meta-graph structure**:
- 3 variables = 3 name nodes + 3 value nodes = 6 total nodes
- 3 assignment edges connecting names to values
- Each variable's data graph is separate from the namespace graph

## Statistics: Data vs. Meta-Data

```bash
glang> stats
Variable namespace statistics:
  Variables: 3
  Total data nodes: 9        # Nodes in the actual data graphs
  Total data edges: 7        # Edges in the actual data graphs  
  Namespace nodes: 6         # Nodes in the meta-graph (3 names + 3 values)
  Assignment edges: 3        # Edges in the meta-graph
  Graph types:
    linear: 2
```

This shows **two levels of graphs**:
1. **Data level**: 9 nodes + 7 edges across all the data graphs
2. **Meta level**: 6 nodes + 3 edges in the variable namespace graph

## Detailed Variable Information

```bash
glang> info fruits
Variable: fruits
Type: linear
Size: 4 nodes
Edges: 3
Data: ['apple', 'banana', 'cherry', 'orange']

glang> info numbers
Variable: numbers  
Type: linear
Size: 5 nodes
Edges: 4
Data: ['1', '2', '3', '4', '5']
```

## Graph Operations Change Both Levels

```bash
glang> show numbers
Graph 'numbers' (linear):
[1] -> [2] -> [3] -> [4] -> [5]

glang> reverse
Reversed graph 'numbers'

glang> show numbers
Graph 'numbers' (linear):
[5] -> [4] -> [3] -> [2] -> [1]

glang> namespace
Variable Namespace Graph:
  Variables: 3
  Total nodes: 6
  Assignment edges: 3

  📛 fruits -> 📊 linear graph (4 nodes)
  📛 numbers -> 📊 linear graph (5 nodes)  # Still 5 nodes, just reversed!
  📛 empty -> 📊 directed graph (0 nodes)
```

Notice how:
- The **data graph** changed (reversed order)
- The **meta-graph** stayed the same (still the same variable with the same graph type and size)
- Both levels are tracked and can be inspected

## Variable Deletion Affects the Meta-Graph

```bash
glang> delete empty
Deleted graph 'empty'

glang> namespace
Variable Namespace Graph:
  Variables: 2
  Total nodes: 4             # Now 2 names + 2 values = 4 nodes
  Assignment edges: 2        # Now 2 assignment edges

  📛 fruits -> 📊 linear graph (4 nodes)
  📛 numbers -> 📊 linear graph (5 nodes)

glang> stats  
Variable namespace statistics:
  Variables: 2
  Total data nodes: 9
  Total data edges: 7
  Namespace nodes: 4         # Reduced from 6 to 4
  Assignment edges: 2        # Reduced from 3 to 2
  Graph types:
    linear: 2
```

Deleting a variable removes nodes and edges from the **meta-graph**!

## The Philosophy in Practice

What makes this revolutionary is that **there's no artificial boundary** between:

- **Your data** (the lists of fruits and numbers)
- **How your data is stored** (the variable namespace graph)  
- **Introspection capabilities** (examining both levels)

In traditional languages, you can't ask "how are my variables stored?" because it's hidden. In glang, you can literally visualize and analyze the storage mechanism because it's **just another graph**.

## Future Possibilities

This foundation enables powerful features:

```bash
# Hypothetical future commands:
glang> import math_utils     # Creates import edges in namespace graph
glang> namespace            # Shows module connections
glang> dependencies fruits  # Shows what 'fruits' depends on
glang> copy numbers numbers2 # Graph cloning operations
glang> merge fruits numbers  # Graph composition
```

Since everything is a graph, all operations work consistently across all levels of the system.

---

**The takeaway**: In glang, you're not just working with graphs as data structures - you're working within a completely graph-based computational environment where you can inspect and manipulate the very fabric of how the language operates.

That's the power of "everything is a graph"! 🚀
# File Handles and Graph Boundaries in Glang

*Date: January 2025*  
*Context: Discussion during implementation of file handle support*

## The Conceptual Challenge

When implementing file handles for Glang, we encountered a fundamental question: How do file handles fit into Glang's graph-theoretic paradigm? Are they nodes? Edges? Something else?

## Initial Considerations

### The Graph-Theoretic View
Initially considered treating file handles (or streams) as **edges** in the graph, since they represent connections and data flow:
- Streams have direction (read/write)
- They're stateful (position, open/closed)
- They connect sources to sinks
- They enable data flow

### The Problem with This View
An edge requires two nodes. But what are the nodes?
- One end: The opened file? But files aren't really nodes *in* our graph
- Other end: The program? The data structure receiving the data?
- This doesn't quite work conceptually

## The Key Insight: Boundary Crossings

File handles don't fit the node/edge paradigm because they represent something different: **boundaries** between the program's internal graph and the external world.

### Multiple Graph Layers

We need to distinguish between different levels of graphs:

1. **System Graph** (Outside Program)
   - Filesystem, OS, Program Process
   - File handles connect to this layer

2. **Execution Graph** (Runtime Flow)
   - Operations and data flow during execution
   - File handles are operations here

3. **Program Data Graph** (Internal Structure)
   - The actual data structures and transformations in the program
   - File handles are NOT part of this graph

## File Handles as Boundary Capabilities

File handles are best understood as **portals** or **capabilities** - special values that provide controlled access to the external world:

```glang
# File handle as a portal/capability
portal = io.open("data.txt", "r")  # Creates a portal to external world

# The portal can inject data into the graph
data = portal.read()  # Data crosses boundary here
my_list.append(data)  # Now data is IN the graph

# Or extract data from the graph
portal.write(my_data)  # Data crosses boundary outward
```

### Key Characteristics

1. **Immutable Identity**: You can't turn a file handle into a different file handle
2. **Unidirectional**: A handle is either for reading OR writing, never both
3. **Boundary Operations**: Can only pull data in or push data out
4. **Not Graph Participants**: Don't participate in graph transformations
5. **Explicitly Stateful**: Maintain external state (file position, open/closed)

## Impedance Mismatch

This reveals a fundamental impedance mismatch that most languages hide:
- **Graph operations**: Pure transformations within the program
- **I/O operations**: Boundary crossings to/from external world

Glang, being graph-aware, makes this distinction visible rather than hiding it.

## Design Implications

### Current Implementation (Correct)
Treating file handles as special "boundary values" that:
- Can be stored in data nodes (as values in key-value pairs)
- Have methods for boundary operations (read, write, flush, close)
- Are NOT participants in graph transformations
- Are explicitly stateful and external

```glang
# File handles as boundary references in data nodes
config = {
    "source": io.open("input.txt", "r"),  # Boundary reference
    "data": []  # Internal graph data
}

# Explicit boundary crossing
config["data"] = config["source"].read()  # Cross boundary
config["source"].close()  # Release boundary
```

### Future Considerations

Should Glang explicitly distinguish between:
- **Internal operations** (pure graph transformations)
- **Boundary operations** (I/O, system calls)
- **Hybrid operations** (async generators that bridge both)?

## Philosophical Significance

This distinction between "data in the graph" and "portals to external data" is real and meaningful. Most languages sweep this under the rug, but Glang's graph-theoretic foundation forces us to confront it directly.

By treating file handles as boundary capabilities rather than trying to force them into the node/edge paradigm, we:
1. Maintain conceptual clarity
2. Make the boundary between program and world explicit
3. Enable future innovations in how programs interact with their environment

## Summary

File handles in Glang are **boundary capabilities** - immutable portals that allow controlled, unidirectional data flow between the program's internal graph and the external world. They are not nodes or edges in the graph, but rather access points where data can enter or leave the graph.

This conceptual clarity is important for Glang's future development, especially as we move toward:
- Distributed graph systems
- Self-aware data structures
- Safe mutation boundaries
- Explicit resource management

## Related Concepts to Explore

- Network sockets as boundary capabilities
- Database connections as persistent portals
- Inter-process communication as graph bridges
- Async generators as temporal boundaries
- Hardware interfaces as capability objects
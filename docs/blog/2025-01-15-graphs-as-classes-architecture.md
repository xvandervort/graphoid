# Graphs as Classes: Designing Glang's Path to Self-Hosting

*January 15, 2025*

## Introduction

Glang has reached a fascinating architectural crossroads. With the successful implementation of binary trees as true graph structures with edge governance, we've proven that Glang can support sophisticated data structures built on genuine graph foundations. But this success has revealed deeper questions about the language's future: How do we move from container-based thinking to graph-native programming? What does object-oriented programming look like in a graph-theoretic language? And crucially, how do we chart a path to full self-hosting where Glang can implement its own interpreter and standard library?

This post explores our recent architectural discussions and the design decisions that will shape Glang's evolution over the next 3-5 years.

## The Graph-as-Classes Realization

### Where We Are Today

Glang's binary tree implementation demonstrated something remarkable: when you combine a graph structure (nodes and edges) with attached behaviors, you get something that feels very much like a class in object-oriented programming. A binary tree has:

- **Data layer**: The values stored in nodes
- **Structure layer**: The graph topology (left/right relationships)
- **Behavior layer**: Methods like `insert()`, `search()`, `in_order()`
- **Governance layer**: Rules that enforce tree constraints

This multi-layered approach gives us class-like encapsulation and behavior without traditional OOP inheritance hierarchies.

### The Missing Piece: Custom Behaviors

But there's a critical gap: users cannot yet attach their own custom functions to the behavior plane of a graph. You can use the built-in behaviors like `nil_to_zero` or `positive`, but you can't write:

```glang
# This doesn't work yet, but it should:
my_tree.add_behavior("balance", func(self) {
    # Custom tree balancing logic
    return self.rebalance_nodes()
})

# Or define it declaratively:
MyBalancedTree from BinaryTree {
    behavior {
        balance: func(self) { ... }
        pretty_print: func(self) { ... }
    }
}
```

This capability would transform Glang from a language with graph data structures into a true graph-oriented programming language where graphs serve the role that classes serve in traditional OOP.

## Design Decisions: Learning from the Pitfalls

### The Inheritance Problem

Traditional object-oriented inheritance doesn't map cleanly to graph structures. Consider what multiple inheritance would mean in a graph context:

```glang
# What would this even mean?
class TreeList inherits BinaryTree, List {
    # How do you merge a tree graph with a list graph?
    # Tree has left/right edges, List has sequential edges
    # Conflicting governance rules
    # Incompatible traversal patterns
}
```

Multiple inheritance would require **merging graphs**, which introduces complex questions:
- How do you resolve conflicting edge semantics?
- What happens when governance rules conflict?
- How do you maintain the integrity of each graph's invariants?

Our solution: **composition over merging**. Instead of inheriting from multiple graph types, you compose behaviors and extend from a single base type.

### Terminology That Matters

We've chosen `from` as our extension keyword, deliberately avoiding the overloaded `extends`. The syntax feels natural:

```glang
MyCustomTree from BinaryTree {
    behavior {
        custom_method: func(self, x) { ... }
    }

    governance {
        custom_rule: "no_duplicate_values"
    }
}
```

This clearly indicates that `MyCustomTree` is **based on** `BinaryTree` but adds its own behaviors and rules.

### No Free-Form Graphs (Yet)

In the early phases, we're deliberately constraining users to start from built-in graph types (BinaryTree, List, Hash). This prevents users from creating invalid graph structures while they're learning the paradigm. Free-form graph creation is a nice-to-have feature for advanced users, but solid foundations come first.

## The Multi-File Challenge

### Beyond Single Modules

As soon as users can create custom graph types, they're going to want to organize them across multiple files. Our current `import "module"` system works for single files, but real applications need more:

```glang
# Current: Single file imports
import "utils" as Utils

# Needed: Application and library loading
load "my_app"           # Entire directory structure
load "graphics_lib"     # Complex library with dependencies
load "data_science"     # Package with internal organization
```

We need a flexible system inspired by Ruby's approach: a single file can load individual files, lists of files, or entire directory hierarchies. This will test our internal call graph system, but we don't want to make the user experience harder by thinking small.

### Directory-First Design

The loading system needs to support complex application structures from day one:

```
my_app/
├── main.gr
├── types/
│   ├── user.gr
│   ├── data_structures.gr
│   └── algorithms/
│       ├── sorting.gr
│       └── searching.gr
├── lib/
│   └── utils.gr
└── config/
    └── settings.gr
```

A call to `load "my_app"` should intelligently traverse this structure, respecting dependencies and loading order.

## The Self-Hosting Vision

### Timeline: 3-5 Years

Self-hosting - where Glang can compile and run its own interpreter - is our long-term goal. This isn't just about language maturity; it's about proving that Glang's graph-theoretic approach can handle the complexity of real-world systems programming.

### The Bootstrap Problem

Self-hosting presents a classic chicken-and-egg problem: you need a Glang interpreter to run Glang code that implements a Glang interpreter. Our planned approach:

**Phase 1: Foundation (Current - 18 months)**
- Custom behavior API (immediate priority)
- Multi-file loading system
- Graph type extension with `from` syntax

**Phase 2: Standard Library Migration (18-36 months)**
- Core algorithms implemented in Glang
- String processing, data structures, I/O patterns
- Performance optimization for self-hosting workloads

**Phase 3: Interpreter Components (36-48 months)**
- Lexer and parser implemented in Glang
- Semantic analyzer and type checker in Glang
- AST evaluation engine in Glang

**Phase 4: Full Bootstrap (48-60 months)**
- Complete Glang interpreter written in Glang
- Self-compilation: Glang compiling Glang
- Standard library entirely self-hosted

### Architecture Implications

Self-hosting will require some features we haven't fully designed yet:

- **Meta-graph operations**: The interpreter will need to manipulate its own graph structures
- **Performance optimization**: Self-hosted code needs to be fast enough for practical use
- **Bootstrap verification**: How do we ensure the self-hosted interpreter produces identical results?

But it will also validate our core thesis: that graph-theoretic programming can handle arbitrary computational complexity.

## Implementation Strategy

### Phase 1: Custom Behaviors (Immediate)

The highest priority is enabling custom behaviors. This unlocks graph-as-classes functionality and provides the foundation for everything else. Key components:

1. **Behavior attachment API**: Runtime addition of custom functions to graph behavior planes
2. **Declarative syntax**: Define custom behaviors in graph type definitions
3. **Behavior inheritance**: How custom behaviors compose with base type behaviors
4. **Method resolution**: How the system finds and calls the right behavior function

### Phase 2: Multi-File Loading (6-12 months)

Once custom behaviors work, developers will immediately need better code organization:

1. **Flexible loading semantics**: Single files, file lists, or directory trees
2. **Dependency resolution**: Automatic loading of required dependencies
3. **Namespace management**: Preventing naming conflicts across loaded components
4. **Performance optimization**: Lazy loading and caching for large applications

### Phase 3: Advanced Features (12+ months)

With the foundations solid, we can tackle more sophisticated features:

1. **Governance extension**: Custom rules for custom graph types
2. **Template inheritance**: More sophisticated composition patterns
3. **Graph merging**: Advanced feature for power users (experimental mode)
4. **Self-hosting preparation**: Performance and meta-programming features

## Avoiding the Pitfalls

### The Complexity Trap

It would be easy to over-engineer this system. We're deliberately starting simple:
- One base type per custom graph type (no multiple inheritance)
- Built-in base types only (no free-form graphs initially)
- Composition over complex inheritance hierarchies
- Clear error messages when users hit the boundaries

### The Performance Trap

Graph operations can be expensive. We need to ensure that common operations remain fast:
- Method dispatch should be O(1) for behavior lookup
- Graph traversal should be optimized for common patterns
- Memory usage should be reasonable for typical applications

### The Usability Trap

The system needs to feel natural to developers coming from other languages:
- Familiar syntax where possible (`from` instead of mathematical symbols)
- Clear error messages that guide users toward correct usage
- Good defaults that work for most cases
- Documentation with practical examples

## Looking Forward

### What This Enables

When complete, this architecture will enable entirely new programming patterns:

**Graph-Native Libraries**: Libraries designed around graph traversal and manipulation, not just collections and algorithms.

**Self-Describing Systems**: Applications that can introspect and modify their own structure at runtime.

**Evolutionary Programs**: Code that can adapt its own behavior based on runtime conditions.

**Distributed Graph Computing**: Eventually, graph structures that span multiple machines transparently.

### The Broader Vision

Glang's graph-as-classes approach isn't just about syntax or performance - it's about fundamentally changing how we think about computation. Instead of objects that send messages, we have graph nodes that establish relationships. Instead of inheritance hierarchies, we have composition planes. Instead of method dispatch, we have graph traversal.

This shift enables programming patterns that are difficult or impossible in traditional languages, while still feeling familiar to developers who understand object-oriented concepts.

## Conclusion

The path from our current binary tree implementation to full self-hosting is ambitious but achievable. By focusing on custom behaviors first, then multi-file organization, then advanced features, we can build the foundations methodically while keeping the system usable at each step.

The 3-5 year timeline for self-hosting isn't just about language completeness - it's about proving that graph-theoretic programming can handle the complexity of real systems. When a Glang interpreter written in Glang can successfully compile and run itself, we'll have demonstrated that this approach scales to arbitrary computational challenges.

Every design decision we make now - from the `from` keyword to the composition-over-inheritance principle - shapes whether that vision becomes reality. By keeping the foundations solid and the user experience clean, we're building not just a programming language, but a new computational paradigm.

The future of programming might well be graph-shaped. Glang is our attempt to get there first, and to get there right.

---

*This post represents the current state of our architectural thinking. As Glang evolves, some details may change, but the core vision of graph-native programming with a path to self-hosting remains constant.*

*For technical implementation details, see the [Primary Roadmap](../dev_docs/PRIMARY_ROADMAP.md) and [Edge Governance Design](../dev_docs/EDGE_GOVERNANCE_DESIGN.md).*
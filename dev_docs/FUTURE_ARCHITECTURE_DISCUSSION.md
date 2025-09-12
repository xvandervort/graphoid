# Future Architecture Discussion

*Date: 2025-01-08*
*Context: Post-Phase 3, planning advanced language features*

## Priority Order for Future Implementation

1. **Continue Current Plan** - Complete Phases 4-6 as outlined
2. **Node Metadata System** - High priority, very interesting for graph philosophy
3. **Scopes & Methods** - Core programming constructs
4. **First-Class Functions & Modules** - Advanced functional features

---

## Node Metadata System

### Vision: Graph Nodes with Rich Metadata
Every value in Glang should be a graph node capable of carrying metadata tags.

### Key Design Decisions

**Metadata as Tagging System:**
- NOT for adding class-like methods to nodes ("That's crazy!")
- FOR tracking provenance, progression, trust levels, relationships
- FOR filtering and querying nodes based on metadata

### Use Cases Identified

**Provenance Tracking:**
```glang
# Example: Track data sources
string user_input = get_web_form_data()  # Tagged: source="web_form"
string clean_data = scrub(user_input)    # Tagged: source="web_form", processed="scrubbed"
```

**Trust Levels:**
```glang
# Filter nodes by trust level based on provenance
trusted_nodes = find_nodes(metadata="source:database")
untrusted_nodes = find_nodes(metadata="source:user_input")
```

**Progression Tracking:**
```glang
# Track how data flows through system
node.add_metadata("connections", count_connections(node))
node.add_metadata("processing_stage", "validated")
```

**Graph Relationship Metadata:**
```glang
# Semantic relationships beyond structural
a.add_relationship_metadata(b, "follows")
a.add_relationship_metadata(c, "depends_on")
```

### Implementation Approach
- Start with transparent debugging info (automatic provenance)
- Evolve to full user API for metadata manipulation
- Focus on tagging/filtering rather than complex metadata objects

---

## Scopes & Methods

### Key Design Decisions

**Scopes as Graph Boundaries:**
- **Question:** "Should scopes create graph boundaries?"  
- **Answer:** "Yes, I believe so. I envision a scope as a subgraph."
- Scopes should be represented as subgraphs in the meta-graph
- Variables within a scope are nodes within that subgraph
- Scope resolution becomes graph traversal

**Method Syntax:**
```glang
def name(string one, list two) { 
    // method body 
}
```
- No output type declaration (reduces verbosity)
- Curly braces for scope boundaries
- Clear parameter typing

---

## Function Calls & Optional Parentheses

### Flexible Call Syntax
**Question:** How do optional parentheses work with method calls?  
**Answer:** "Both should work unless there's something else going on."

**Examples:**
```glang
func arg1, arg2        # Works for simple calls
func(arg1, arg2)       # Works always

# Method chaining requires parentheses for clarity
obj.method1().method2()   # Clear chaining
obj.method1.method2       # Ambiguous - avoid
```

**Design Principle:** Maintain Ruby-style flexibility while ensuring method chaining clarity.

---

## First-Class Functions & Modules

### Functions as Values
- Functions should be assignable to variables
- Modules should be passable as values
- Enables functional programming patterns

### Implementation Challenges to Solve Later
1. Function type system
2. Closure capture semantics  
3. Call syntax disambiguation
4. Module-as-value semantics

---

## Garbage Collection Strategy

### Current Approach
- Leverage Python's GC initially
- Plan for graph-aware cycle detection later

### Future Graph-Aware GC
- Use graph algorithms (strongly connected components) for cycle detection
- Handle cycles naturally occurring in graph structures
- Consider reference counting + cycle detection hybrid

---

## Classes vs Modules Decision

**Agreed:** Avoid classes initially, use modules for organization.

**Module-based "Classes":**
- Data organization through exported variables
- Method organization through exported functions
- Multiple "instances" through module factories
- Fits better with graph philosophy than traditional OOP

---

## Next Steps

1. **Complete Current Roadmap** (Phases 4-6)
2. **Node Metadata Proposal** - Claude to propose implementation approach
3. **Discussion & Implementation** - Collaborative design and coding
4. **Scopes & Methods** - After metadata system is working

---

*This document captures the architectural vision for Glang's evolution beyond basic data types, ensuring advanced features align with the core graph-theoretic philosophy.*
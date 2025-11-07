# Hash Access Design Problem

*Analysis Date: January 2025*

## The Problem

Current hash access behavior is philosophically consistent but practically painful:

```glang
config = { "host": "localhost", "port": 8080 }
host = config["host"]          # Returns: { "host": "localhost" } (data node)
host_value = config["host"].value()  # Returns: "localhost" (actual value)
```

This creates significant friction for developers who expect:
```glang
host = config["host"]          # Should return: "localhost"
```

## Why This Happens

### Philosophical Consistency
- In Glang's graph-theoretic model, hashes are collections of data nodes
- Each data node is a key-value pair that maintains its identity
- Accessing a hash element returns the full node, preserving its graph nature

### Current Architecture
```
Hash (container)
  ├── DataNode { "host": "localhost" }
  ├── DataNode { "port": 8080 }
  └── DataNode { "debug": true }
```

When you access `hash["key"]`, you're getting the DataNode, not its value.

## Proposed Solutions

### Solution 1: Smart Context-Aware Access (Recommended)

Make hash access **context-aware** - return the value in most contexts, but the node when needed:

```glang
# Value contexts (99% of usage) - return the value directly
host = config["host"]              # "localhost" (value)
if config["debug"] == true { }     # true (value)
port_str = config["port"].to_string()  # "8080" (value)

# Node contexts - return the full node
node = config.get_node("host")     # { "host": "localhost" } (node)
key = config["host"].key()         # "host" (accessing node method)
config["host"].behaviors.add(...)  # Working with node itself
```

**Implementation Strategy:**
- Default `[]` operator returns values for practical use
- Add explicit `.get_node()` method for graph operations
- Maintain `.get()` as current behavior for backward compatibility

### Solution 2: Dual Access Operators

Introduce a new operator for value access while keeping `[]` for nodes:

```glang
# Current behavior - returns node
node = config["host"]              # { "host": "localhost" }

# New operator - returns value
host = config."host"               # "localhost"
# OR
host = config@"host"               # "localhost"
# OR
host = config->"host"              # "localhost"
```

**Pros:** Clear distinction between node and value access
**Cons:** New syntax to learn, potential confusion

### Solution 3: Property-Style Access

Allow property-style access for values, bracket access for nodes:

```glang
# Property access - returns value (new)
host = config.host                 # "localhost"
port = config.port                 # 8080

# Bracket access - returns node (current)
node = config["host"]              # { "host": "localhost" }

# Dynamic access still works
key = "host"
value = config.get_value(key)      # "localhost"
node = config[key]                  # { "host": "localhost" }
```

**Pros:** Very intuitive, follows JavaScript/Python conventions
**Cons:** Only works for valid identifier keys

### Solution 4: Automatic Value Unwrapping (Magic)

Keep current structure but automatically unwrap values in common operations:

```glang
config = { "host": "localhost", "port": 8080 }

# Auto-unwrap in assignments
host = config["host"]              # "localhost" (auto-unwrapped)

# Auto-unwrap in comparisons
if config["debug"] == true { }     # Works (auto-unwrapped)

# Explicit node access when needed
node = config.node("host")         # { "host": "localhost" }

# Method calls on the result know what to do
result = config["host"]
result.to_string()                 # "localhost" (auto-detects and unwraps)
result.value()                     # Still works for compatibility
```

**Implementation:** The interpreter detects context and unwraps accordingly

### Solution 5: Configuration Option

Let users choose their preferred behavior:

```glang
# At file/module level
pragma hash_access: value  # or "node"

# Or globally in REPL/config
set_hash_behavior("value")

# Then all hash access returns values
host = config["host"]              # "localhost"
```

## Recommended Approach

**Implement Solution 1 (Smart Context-Aware Access) with aspects of Solution 4:**

1. **Default Behavior Change:** Make `hash["key"]` return the value (breaking change, but worth it)

2. **Explicit Node Access:** Add `.get_node(key)` for when you need the full node

3. **Backward Compatibility:** Keep `.get(key)` with current behavior, deprecate over time

4. **Auto-unwrapping:** In contexts where values are clearly expected, auto-unwrap

5. **Clear Documentation:** Explain the philosophy while prioritizing usability

## Implementation Plan

### Phase 1: Add New Methods (Non-breaking)
```glang
config.get_value("host")           # "localhost" (new)
config.get_node("host")            # { "host": "localhost" } (new)
config["host"]                     # { "host": "localhost" } (current)
```

### Phase 2: Deprecation Notices
- Warn when using `["key"].value()` pattern
- Suggest using new methods

### Phase 3: Behavior Switch (Breaking)
```glang
config["host"]                     # "localhost" (NEW - returns value)
config.get_node("host")            # { "host": "localhost" } (for node access)
```

## Alternative: Type-Based Solution

Another approach using Glang's type system:

```glang
# Declare intent with types
hash<string> config = { "host": "localhost" }  # Values are strings
data<string> nodes = { "host": "localhost" }   # Values are nodes

host = config["host"]              # "localhost" (value)
node = nodes["host"]               # { "host": "localhost" } (node)
```

## Philosophical Considerations

### Why This Change Makes Sense

1. **Practical Over Pure:** While philosophically pure, the current behavior creates friction

2. **Progressive Disclosure:** Simple things (get value) should be simple; complex things (get node) can require explicit methods

3. **User Expectations:** 99% of hash access is for values, not nodes

4. **Language Evolution:** Even philosophically-driven languages must prioritize usability

### Maintaining Graph Philosophy

The graph-theoretic nature remains intact:
- Nodes still exist and are accessible
- Graph operations still work on the full structure
- Advanced users can access full node capabilities
- We're just optimizing the common case

## Decision Framework

Choose based on priorities:

1. **Usability First:** Solution 1 (Smart Context-Aware)
2. **Philosophical Purity:** Keep current behavior
3. **Flexibility:** Solution 5 (Configuration Option)
4. **Gradual Migration:** Phased implementation plan

## Recommendation

**Implement Solution 1** with a phased rollout:

1. **Immediately:** Add `get_value()` and `get_node()` methods
2. **Next Release:** Add deprecation warnings for `.value()` pattern
3. **Future Release:** Switch `[]` to return values directly
4. **Long Term:** Consider property-style access for common cases

This balances philosophical consistency with practical usability, making Glang more intuitive while preserving its graph-theoretic foundations.

## Impact Analysis

### Code That Would Improve
```glang
# Before (current)
name = config["name"].value()
port = config["port"].value()
if config["debug"].value() == true { }

# After (proposed)
name = config["name"]
port = config["port"]
if config["debug"] == true { }
```

### Code That Would Need Updates
```glang
# Getting node properties (rare)
# Before
node = config["host"]
key = node.key()

# After
node = config.get_node("host")
key = node.key()
```

The improvement in common cases far outweighs the verbosity in rare cases.
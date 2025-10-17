# Edge Governance System Design

## Problem Statement

Glang's current edge system is **fully functional but dangerously permissive**. It allows operations that can break data structure integrity:

1. **Circular Lists**: Creating cycles in sequential structures
2. **Cross-Structure Edges**: Connecting nodes from different data structures
3. **Structural Invariant Violations**: Breaking assumptions about list ordering, hash key access, etc.

These operations can cause:
- Infinite loops in traversal algorithms
- Memory leaks and reference cycles
- Type system confusion
- Serialization failures
- Unpredictable behavior in graph operations

## Current State Analysis

### ✅ What Works (High-Level APIs)
- `ListValue.add_edge(from_index, to_index, relationship)` - restricted to same list
- `HashValue.add_value_edge(from_key, to_key, relationship)` - restricted to same hash

### ⚠️ What's Dangerous (Foundation Level)
- `GraphNode.add_edge_to(target, metadata)` - **NO RESTRICTIONS**
- Direct node access allows bypassing all safety checks
- Foundation level can create hybrid structures that break type system

### 🚨 Confirmed Dangerous Operations
Testing reveals these operations are currently possible:
```python
# Create circular list (breaks traversal algorithms)
numbers.add_edge(2, 0, "circular")  # ✓ Works, creates cycle

# Cross-structure contamination (breaks type system)
list_node.add_edge_to(hash_node, metadata)  # ✓ Works, creates hybrid monster

# Sequential order violations (breaks list assumptions)
sequence.add_edge(4, 1, "jump_back")  # ✓ Works, violates ordering
```

## Design Approach: Control Plane Governance

### Why Control Plane vs Hard-Coded?

**Control Plane Advantages (Glangish Way):**
- ✅ **Configurable**: Rules can be adjusted per use case
- ✅ **Extensible**: New constraint types without code changes
- ✅ **Transparent**: Users can see and understand the rules
- ✅ **Overridable**: Advanced users can relax restrictions when needed
- ✅ **Consistent**: Uses same architecture as behavior system and configuration context

**Hard-Coded Disadvantages:**
- ❌ **Inflexible**: Fixed rules that can't adapt to new use cases
- ❌ **Hidden**: Users don't understand why operations fail
- ❌ **Brittle**: Changes require code modifications and releases

### Proposed Architecture: Five-Layer Integration

Building on Glang's **five-layer graph architecture**:

1. **Data Layer** - Pure information storage and relationships
2. **Behavior Layer** - Computational capabilities (already implemented)
3. **Control Layer** - Rule enforcement (**EDGE GOVERNANCE GOES HERE**)
4. **Metadata Layer** - Graph history & context (partially implemented)
5. **System Boundary Layer** - External interfaces (future)

**Edge governance belongs in the Control Layer** - this is exactly what control planes are for!

```python
class ControlLayer:
    """Layer 3: Rule enforcement and governance for graph operations."""

    def __init__(self, parent_graph: 'GraphStructure'):
        self.parent_graph = parent_graph

        # Default rule set (shared, not copied)
        self.rule_defaults = EdgeRuleDefaults.get_standard_rules()

        # Graph-specific rule customizations (only created when needed)
        self.custom_rules: Optional[Dict[str, EdgeRule]] = None
        self.disabled_rules: Set[str] = set()
        self.rule_overrides: Dict[str, Dict] = {}

    def validate_edge_operation(self, from_node: GraphNode, to_node: GraphNode,
                               metadata: EdgeMetadata) -> Tuple[bool, str]:
        """Validate edge operation using five-layer context."""

        # Build context from all layers
        context = {
            'data_layer': self.parent_graph.nodes,           # Layer 1
            'behavior_layer': getattr(self.parent_graph, 'behaviors', {}),  # Layer 2
            'metadata_layer': self.parent_graph.metadata,    # Layer 4
            'graph_type': self._infer_graph_type(),
            'structure_info': self._analyze_structure()
        }

        # Check all active rules
        active_rules = self._get_active_rules()
        for rule_name, rule in active_rules.items():
            if rule_name in self.disabled_rules:
                continue

            is_valid, reason = rule.validate(from_node, to_node, metadata, context)
            if not is_valid:
                return False, f"Control layer rule '{rule_name}' violated: {reason}"

        return True, ""

    def _get_active_rules(self) -> Dict[str, EdgeRule]:
        """Get active rules: defaults + custom rules."""
        if self.custom_rules is None:
            # Pure default mode - no copying overhead
            return self.rule_defaults
        else:
            # Custom mode - merge defaults with customizations
            combined = self.rule_defaults.copy()
            combined.update(self.custom_rules)
            return combined

class EdgeRuleDefaults:
    """Shared default rule set - no per-graph copying overhead."""
    _standard_rules: Optional[Dict[str, EdgeRule]] = None

    @classmethod
    def get_standard_rules(cls) -> Dict[str, EdgeRule]:
        """Get shared standard rules (computed once, shared by all graphs)."""
        if cls._standard_rules is None:
            cls._standard_rules = cls._create_standard_rules()
        return cls._standard_rules

    @classmethod
    def _create_standard_rules(cls) -> Dict[str, EdgeRule]:
        """Create the standard rule set (computed once)."""
        return {
            'no_list_cycles': EdgeRule(...),
            'same_structure_only': EdgeRule(...),
            'preserve_hash_keys': EdgeRule(...),
            'compatible_types': EdgeRule(...),
        }
```

### Standard Edge Rules

#### 1. Structural Integrity Rules
```python
# Prevent cycles in sequential structures
no_cycles_rule = EdgeRule(
    name="no_list_cycles",
    applies_to="list",
    validator=lambda from_node, to_node, metadata, context:
        not would_create_cycle(from_node, to_node, context["structure"]),
    description="Prevent circular references in sequential lists"
)

# Maintain sequential ordering
sequential_order_rule = EdgeRule(
    name="maintain_list_order",
    applies_to="list",
    validator=lambda from_node, to_node, metadata, context:
        is_forward_reference(from_node, to_node, context["sequence"]),
    description="Only allow edges that maintain sequential ordering"
)
```

#### 2. Cross-Structure Protection Rules
```python
# Prevent cross-contamination
same_structure_rule = EdgeRule(
    name="same_structure_only",
    applies_to="*",
    validator=lambda from_node, to_node, metadata, context:
        from_node._graph is to_node._graph,
    description="Edges can only connect nodes within the same data structure"
)

# Type compatibility
type_compatibility_rule = EdgeRule(
    name="compatible_types",
    applies_to="*",
    validator=lambda from_node, to_node, metadata, context:
        are_types_compatible(from_node.value, to_node.value),
    description="Edge endpoints must have compatible types"
)
```

#### 3. Hash-Specific Rules
```python
# Preserve key-based access
hash_key_access_rule = EdgeRule(
    name="preserve_hash_keys",
    applies_to="hash",
    validator=lambda from_node, to_node, metadata, context:
        not would_shadow_key_access(from_node, to_node, context["keys"]),
    description="Custom edges cannot interfere with key-based access"
)
```

### Integration with Existing Systems

#### Configuration Context Integration
```python
# Add edge governance settings to configuration system
EDGE_GOVERNANCE_SETTINGS = {
    'edge_validation': True,           # Enable edge rule checking
    'strict_edge_rules': False,        # Strict vs permissive mode
    'allow_advanced_edges': False,     # Allow power-user overrides
    'edge_rule_logging': False,        # Log rule violations
}
```

#### Rule Default System (Efficiency Focus)

**Key Insight**: All graphs start with shared defaults - only copy when customized.

```python
class GraphStructure:
    def __init__(self, root_node: Optional[GraphNode] = None):
        self.nodes: Dict[str, GraphNode] = {}           # Layer 1: Data
        # Layer 2: Behaviors (implemented via GraphContainer mixin)
        self.control_layer = ControlLayer(self)         # Layer 3: Control (NEW)
        self.metadata = MetadataLayer()                 # Layer 4: Metadata
        # Layer 5: System Boundary (future)

    def validate_edge(self, from_node: GraphNode, to_node: GraphNode,
                     metadata: EdgeMetadata) -> Tuple[bool, str]:
        """Validate edge through control layer."""
        return self.control_layer.validate_edge_operation(from_node, to_node, metadata)

class ControlLayer:
    def customize_rule(self, rule_name: str, new_rule: EdgeRule) -> None:
        """Copy-on-write: create custom rules only when needed."""
        if self.custom_rules is None:
            # First customization - NOW we copy
            self.custom_rules = {}  # Start empty, will merge with defaults

        self.custom_rules[rule_name] = new_rule

    def disable_rule(self, rule_name: str) -> None:
        """Disable a rule without copying entire rule set."""
        self.disabled_rules.add(rule_name)
```

### Language-Level Syntax (Strict by Default)

```glang
# DEFAULT BEHAVIOR: Strict rules active, safe by default
people = ["Alice", "Bob", "Charlie"]
people.add_edge(0, 1, "friend")    # ✓ Allowed (same structure)
people.add_edge(2, 0, "circular")  # ❌ BLOCKED (creates cycle) - STRICT BY DEFAULT

# Power users must explicitly opt out of safety
configure { strict_edge_rules: false } {
    # Dangerous operations now allowed with explicit intent
    circular_buffer.add_edge(last_index, 0, "wrap_around")  # ✓ Explicitly allowed
}

# Or disable specific rules with clear reasoning
configure { disabled_edge_rules: ["no_list_cycles"] } {
    # Specific danger allowed for ring buffer implementation
    ring_buffer.add_edge(tail_index, head_index, "circular")  # ✓ Dangerous but intentional
}

# Graph-level customization (triggers rule copying)
my_special_list.control_layer.disable_rule("no_list_cycles")
my_special_list.add_edge(3, 0, "wrap")  # ✓ Now allowed for this specific graph
```

#### Method Integration
```glang
# Check edge rules before operation
my_list.can_add_edge(2, 0, "circular")  # Returns false + reason

# Get current edge rules
current_rules = my_list.get_edge_rules()  # ["no_list_cycles", "same_structure_only"]

# Temporarily disable rules (power user feature)
my_list.with_edge_rules([]) {
    # Dangerous operations allowed in this scope
    my_list.add_edge(2, 0, "circular")
}
```

## Implementation Plan

### Phase 1: Foundation
1. **Create EdgeRule and EdgeGovernor classes**
2. **Implement standard rules** (no_cycles, same_structure, etc.)
3. **Integrate with GraphNode.add_edge_to()** - add validation layer

### Phase 2: Configuration Integration
1. **Add edge governance settings** to ConfigurationContext
2. **Create configure block support** for edge rules
3. **Add method integration** (can_add_edge, get_edge_rules, etc.)

### Phase 3: Language Syntax
1. **Add edge rule syntax** to parser and AST
2. **Implement with_edge_rules blocks**
3. **Add edge introspection methods** to language

### Phase 4: Advanced Features
1. **Custom rule definition** in Glang code
2. **Rule composition and inheritance**
3. **Performance optimization** for rule checking

## Benefits of This Approach

### ✅ Safety
- **Prevents dangerous operations** that could break data structures
- **Maintains type system integrity** across graph operations
- **Catches errors early** with clear explanations

### ✅ Flexibility
- **Configurable rules** adapt to different use cases
- **Power user overrides** for advanced scenarios
- **Extensible architecture** supports new constraint types

### ✅ Glang Philosophy Alignment
- **Self-governing data structures** - containers enforce their own rules
- **Transparent control plane** - users understand and control the governance
- **Gradual sophistication** - simple by default, powerful when needed

### ✅ Developer Experience
- **Clear error messages** explain why operations are blocked
- **Discoverable constraints** through introspection methods
- **Consistent with existing systems** (behaviors, configuration)

### Five-Layer Graph Architecture Integration

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        COMPLETE GRAPH STRUCTURE                              │
├─────────────────┬─────────────────┬─────────────────┬─────────────────────────┤
│  DATA LAYER     │  BEHAVIOR LAYER │  CONTROL LAYER  │    METADATA LAYER       │
│  (Layer 1)      │  (Layer 2)      │  (Layer 3)      │    (Layer 4)            │
│                 │                 │                 │                         │
│ • Values        │ • Transformers  │ • Edge Rules    │ • Element Names         │
│ • Connections   │ • Validators    │ • Constraints   │ • Provenance            │
│ • Structure     │ • Processors    │ • Governance    │ • History               │
│ • Relationships │ • Behaviors     │ • Safety        │ • Audit Trail           │
└─────────────────┴─────────────────┴─────────────────┴─────────────────────────┘
│                           SYSTEM BOUNDARY LAYER (Layer 5)                    │
│                        • File Handles  • Network  • External APIs            │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Edge governance in Layer 3 (Control) validates operations across ALL layers:**
- **Layer 1 validation**: Check structural integrity (cycles, cross-contamination)
- **Layer 2 integration**: Respect behavior constraints and transformations
- **Layer 4 access**: Use metadata for rule context and decision making
- **Layer 5 boundaries**: Prevent edge operations across system boundaries

## Migration Strategy

### Backward Compatibility
1. **Default strict mode** - safe by default for experimental language
2. **Explicit opt-out** - power users can disable specific rules
3. **Clear migration path** - existing dangerous patterns get clear error messages

### User Education
1. **Documentation** explaining edge governance
2. **Examples** showing safe vs dangerous patterns
3. **Migration guides** for codebases using advanced edges

## Conclusion

The edge governance system transforms Glang's graph capabilities from "dangerously powerful" to "safely powerful" while properly integrating with the **five-layer architecture**:

### ✅ **Architectural Alignment**
- **Layer 3 (Control)** is the natural home for edge governance
- **Cross-layer validation** ensures operations respect all five layers
- **Shared defaults with copy-on-write** eliminates overhead for standard use cases

### ✅ **Safety First Approach**
- **Strict by default** - experimental language can afford to be cautious
- **Explicit opt-out** - dangerous operations require clear intent
- **Educational value** - users learn graph safety through error messages

### ✅ **Efficiency Design**
- **Shared rule defaults** - no per-graph copying until customized
- **Copy-on-write customization** - only pay overhead when needed
- **Five-layer context** - validation uses all available information

### ✅ **Glang Philosophy**
- **Self-governing data structures** - graphs enforce their own rules
- **Constitutional computing** - Layer 3 governance is intrinsic to the graph
- **Transparent control** - users can inspect and modify governance

This positions Glang as the first language with **true multi-layer graph governance** - making graph programming both accessible and reliable through intelligent, intrinsic control systems rather than external restrictions.
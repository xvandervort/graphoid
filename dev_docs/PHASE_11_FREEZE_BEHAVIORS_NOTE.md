# Phase 11: Add Freeze Behaviors

**Note for Phase 11 Planning**: The following behaviors were deferred from Phase 7 and should be added to Phase 11 scope:

## Freeze Control Behaviors

These behaviors require `.freeze()` to be implemented first.

### 1. `:no_frozen` Behavior

**Purpose**: Reject frozen elements (throws error on attempt to add)

**Spec Reference**: Line 738

**Example**:
```graphoid
container = []
container.add_rule(:no_frozen)

frozen_item = [1, 2, 3].freeze()
container.append(frozen_item)  # ❌ ERROR: Cannot add frozen element
```

**Implementation**: Check if value is frozen before adding, raise error if true.

**Estimated Time**: 0.5 day (3-4 tests)

---

### 2. `:copy_elements` Behavior

**Purpose**: Copy all elements on insertion (copies are unfrozen)

**Spec Reference**: Line 739

**Example**:
```graphoid
container = []
container.add_rule(:copy_elements)

original = [1, 2, 3]
container.append(original)
container[0][0] = 99  # Modifies copy, not original
print(original)  # [1, 2, 3] - unchanged
```

**Implementation**: Clone values on insertion. Clones are unfrozen by default.

**Estimated Time**: 0.5 day (3-4 tests)

---

### 3. `:shallow_freeze_only` Behavior

**Purpose**: Freeze operations only freeze the collection, not nested elements

**Spec Reference**: Line 740

**Example**:
```graphoid
container = [[1, 2], [3, 4]]
container.add_rule(:shallow_freeze_only)
container.freeze()

container.append([5, 6])  # ❌ ERROR: container is frozen
container[0].append(99)   # ✅ OK: nested lists are mutable
print(container)  # [[1, 2, 99], [3, 4]]
```

**Implementation**: When freezing, only mark the collection itself, not recursive contents.

**Estimated Time**: 0.5 day (3-4 tests)

---

### 4. Freezing Rulesets and Rule Configurations

**Purpose**: Freeze the set of rules/behaviors attached to a data structure, preventing addition or removal of rules.

**Spec Status**: New feature to be added in Phase 11

**Example**:
```graphoid
# Freeze the rules on a collection
temperatures = [98.6, 100.2, 101.5]
temperatures.add_rule(:none_to_zero)
temperatures.add_rule(:positive)
temperatures.freeze_rules()  # Lock the ruleset

temperatures.add_rule(:round_to_int)  # ❌ ERROR: Ruleset is frozen
temperatures.remove_rule(:positive)    # ❌ ERROR: Ruleset is frozen
temperatures.append(none)              # ✅ OK: Data operations still work, rules apply

# Unfreeze to modify rules again
temperatures.unfreeze_rules()
temperatures.add_rule(:round_to_int)  # ✅ OK: Now allowed
```

**Implementation**:
- Add `rules_frozen` boolean field to List, Hash, Graph
- Check before rule addition/removal operations
- Separate from data freezing (`.freeze()` vs `.freeze_rules()`)

**Estimated Time**: 1 day (8-10 tests)

---

### 5. Type-Level vs Instance-Level Rule Freezing

**Design Consideration**: When graph types are implemented (future phase), freezing rules at the type level has interesting semantics.

**Scenario**: Graph Type Definition with Frozen Rules

```graphoid
# Define a graph type with specific rules
type ValidatedTree = graph {
    type: :tree,
    rules: [:no_cycles, :single_root, :max_children_2]
}

# Freeze the rules at the TYPE level
ValidatedTree.freeze_rules()

# Create instances - what happens?
tree1 = ValidatedTree.new()  # Instance created with type's rules

# Configuration Policy Question:
# Are the INSTANCE's rules also frozen?
# Option A: Instances inherit frozen state (rules locked on tree1)
# Option B: Instances start unfrozen (tree1 can add/remove rules)
# Option C: Configurable via policy
```

**Design Space to Explore**:

1. **Inheritance Semantics**:
   - Do instances inherit the frozen state from their type?
   - Or does type-level freezing only prevent type modification?

2. **Configuration Policies**:
   ```graphoid
   # Policy 1: Strict Inheritance
   ValidatedTree.set_policy(:rules_frozen_inheritance, :strict)
   tree1 = ValidatedTree.new()  # tree1.rules are frozen

   # Policy 2: Independent Instances
   ValidatedTree.set_policy(:rules_frozen_inheritance, :independent)
   tree2 = ValidatedTree.new()  # tree2.rules start unfrozen
   tree2.add_rule(:max_depth_10)  # ✅ OK

   # Policy 3: Copy-on-Write
   ValidatedTree.set_policy(:rules_frozen_inheritance, :copy_on_write)
   tree3 = ValidatedTree.new()  # Shares type's rules
   tree3.add_rule(:max_depth_10)  # Creates instance-specific copy
   ```

3. **Use Cases**:
   - **Strict**: Enforce architectural constraints (e.g., all binary trees MUST have max_children_2)
   - **Independent**: Type provides default rules, instances can customize
   - **Copy-on-Write**: Performance optimization + flexibility

**Implementation Complexity**: High - requires careful architectural design

**When to Implement**:
- **NOT in Phase 11** - too complex, depends on type system
- **Possibly Phase 15+** - After type system is mature
- **Requires**: Graph types, type definitions, inheritance semantics

**Documentation Needed**:
- Specification of inheritance semantics
- Policy configuration API
- Clear mental model for users
- Migration guide for existing code

**Estimated Time** (when implemented): 3-5 days
- Design specification (1 day)
- Implementation (1-2 days)
- Testing (1-2 days)
- Documentation (0.5 day)

---

## Phase 11 Additions

**Add to Phase 11 scope**:
1. Implement `.freeze()` method for all collections (1-2 days)
2. Implement `.is_frozen()` introspection (0.5 day)
3. Implement freeze behaviors (1.5 days total)
   - `:no_frozen` (0.5 day)
   - `:copy_elements` (0.5 day)
   - `:shallow_freeze_only` (0.5 day)
4. Implement ruleset freezing (1 day)
   - `.freeze_rules()` method
   - `.unfreeze_rules()` method
   - `.rules_frozen()` introspection

**Total Addition**: ~4 days

**Updated Phase 11 Duration**: 18-25 days (was 14-21)

**Deferred to Phase 15+**:
- Type-level rule freezing with inheritance policies (Section 5 above)
- Requires mature type system and graph type definitions

---

**Dependencies**:
- Freeze behaviors depend on `.freeze()` implementation
- Should be implemented after basic freeze/unfreeze works
- Can be tested incrementally

**Integration**:
- These behaviors fit naturally into existing behavior system
- Use same BehaviorSpec/BehaviorInstance infrastructure
- RetroactivePolicy applies (Clean = retroactively apply freeze rules)

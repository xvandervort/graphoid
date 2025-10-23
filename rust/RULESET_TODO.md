# Ruleset Implementation TODO

**Status**: PARTIAL IMPLEMENTATION (January 2025)
**Scheduled Completion**: Phase 6 Week 2

---

## ✅ What's Implemented (Refactor - January 2025)

### Minimal Working Implementation
The following was implemented during the Option A refactor to avoid half-baked features:

1. **Storage** (src/values/graph.rs)
   - `Graph.rulesets: Vec<String>` field
   - Stores list of active ruleset names (e.g., "tree", "dag", "bst")

2. **Methods** (src/values/graph.rs)
   - `with_ruleset(ruleset: String) -> Self` - adds ruleset and returns self
   - `has_ruleset(ruleset: &str) -> bool` - checks if ruleset is active
   - `get_rulesets() -> &[String]` - returns all active rulesets

3. **Executor Support** (src/execution/executor.rs)
   - `eval_graph_method()` dispatches Graph method calls
   - Handles `.with_ruleset(:symbol)` method calls
   - Handles `.has_ruleset(:symbol)` method calls

4. **Parser Desugaring** (src/parser/mod.rs)
   - `tree{}` → `graph{}.with_ruleset(:tree)`
   - Fully functional end-to-end

### What This Enables

```graphoid
# Works perfectly RIGHT NOW:
t = tree {}                      # Desugars to graph{}.with_ruleset(:tree)
has_tree = t.has_ruleset(:tree)  # Returns true

# Multiple rulesets:
g = graph{}.with_ruleset(:dag).with_ruleset(:acyclic)
```

---

## ❌ What's NOT Implemented (Phase 6 Week 2)

### Rule Enforcement - THE BIG MISSING PIECE

**None of these work yet:**

1. **Rule Validation on Mutations**
   ```graphoid
   t = tree {}
   t.add_node("orphan", 10)  # Should fail: violates single_root
   t.add_edge("a", "b")
   t.add_edge("b", "a")      # Should fail: creates cycle
   ```
   **Current behavior**: No validation, mutations always succeed

2. **Predefined Rulesets**
   - `:tree` ruleset definition (no_cycles, single_root, connected)
   - `:dag` ruleset definition (no_cycles)
   - `:binary_tree` ruleset definition (max_children_2)
   - `:bst` ruleset definition (bst_ordering)

   **Current behavior**: Rulesets are just stored names, no rules enforced

3. **Rule System Architecture**
   - `Rule` trait for validation logic
   - `RuleViolation` error type
   - Pre-mutation validation hooks
   - Ruleset composition (e.g., :bst includes :binary_tree includes :tree)

4. **Custom User Rules**
   ```graphoid
   func validate_positive_values(graph) { ... }
   my_graph.add_rule(validate_positive_values)
   ```
   **Current behavior**: Not supported at all

---

## 📅 Implementation Schedule

### Phase 6 Week 2 (7-10 days)

**File**: `src/graph/rules.rs` (new file)

1. **Day 1-2**: Rule trait and basic system
   ```rust
   pub trait Rule {
       fn validate(&self, graph: &Graph) -> Result<(), RuleViolation>;
   }
   ```

2. **Day 3-4**: Built-in rules
   - `NoCyclesRule`
   - `SingleRootRule`
   - `ConnectedRule`
   - `MaxChildrenRule`
   - `BSTOrderingRule`

3. **Day 5-6**: Ruleset definitions
   - Predefined rulesets (:tree, :dag, :binary_tree, :bst)
   - Ruleset composition/inheritance

4. **Day 7**: Integration
   - Hook validation into Graph mutations (add_node, add_edge, etc.)
   - Update executor to handle rule violations
   - Error messages

5. **Day 8-10**: Testing
   - Ruleset validation tests
   - Rule violation tests
   - Edge case handling

---

## 🎯 Acceptance Criteria (Phase 6 Week 2)

### Must Have

- ✅ `:tree` ruleset enforces no_cycles, single_root, connected
- ✅ `:binary_tree` ruleset enforces max 2 children
- ✅ `:bst` ruleset enforces BST ordering
- ✅ Rule violations throw clear error messages
- ✅ 30+ ruleset tests passing

### Nice to Have

- ✅ Custom user-defined rules
- ✅ Ruleset inheritance/composition
- ✅ Helpful validation error messages with suggestions

---

## 🔗 Related Documents

- **Design**: `TREE_RULESET_DESIGN.md` - Tree type hierarchy and philosophy
- **Roadmap**: `/home/irv/work/grang/dev_docs/RUST_IMPLEMENTATION_ROADMAP.md` - Phase 6 details
- **Code**:
  - Current implementation: `src/values/graph.rs` (lines 43-387)
  - Executor support: `src/execution/executor.rs` (lines 734-787)
  - Parser desugaring: `src/parser/mod.rs` (lines 1095-1131)

---

## ⚠️ IMPORTANT NOTES

1. **Current implementation is INTENTIONAL**
   - We implemented minimal ruleset support during refactor
   - This avoids half-baked features and failing tests
   - Everything works, just without enforcement

2. **Do NOT skip Phase 6 Week 2**
   - Rule enforcement is critical for the Graphoid philosophy
   - Trees ARE graphs with rules - we need to enforce those rules
   - This is what makes the philosophy concrete and real

3. **Testing strategy**
   - Current tests work because no enforcement yet
   - Phase 6 Week 2 will add enforcement tests
   - Some current tests may need updates when enforcement is added

---

**Last Updated**: January 2025 (during Option A refactor)
**Next Action**: Complete refactor (Steps 5-7), then Phase 6 Week 2

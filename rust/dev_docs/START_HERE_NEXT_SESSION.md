# START HERE - CRITICAL REFACTORING IN PROGRESS ⚠️

**Last Updated**: October 25, 2025
**Current Status**: 🚨 ARCHITECTURE REFACTORING REQUIRED
**Tests Passing**: 398/398 (but architecture is wrong)
**What's Next**: 🎯 COMPLETE RULE SYSTEM UNIFICATION

---

## 🚨 CRITICAL ISSUE: Architecture Does Not Match Specification

### The Problem

The current implementation violates the language specification by having a split between "rules" and "behaviors" internally:

**CURRENT (WRONG)**:
- Validation rules → stored in `graph.rules`
- Transformation rules → stored in `list.behaviors` and `hash.behaviors`
- Separate `RuleSpec` and `BehaviorSpec` enums
- Separate concepts even internally

**SPECIFICATION SAYS**:
> "The spec uses `add_rule()` for both validation rules and behavior rules"
> "Rules and behaviors share the `add_rule()` namespace"

**Everything should be a RULE**. Some rules validate (reject invalid), some rules transform (accept after transformation). There should be NO separate "behaviors" concept anywhere, even internally.

---

## 📋 Refactoring Plan

**See**: `UNIFICATION_PLAN.md` for complete step-by-step instructions

### Summary of Required Changes

1. ✅ **DONE**: Merged BehaviorSpec variants into RuleSpec enum
2. **TODO**: Add `from_symbol()` and `name()` methods to RuleSpec
3. **TODO**: Extend Rule trait to handle transformation
4. **TODO**: Update RuleSpec::instantiate() for transformation rules
5. **TODO**: Remove `behaviors` field from List and Hash
6. **TODO**: Store ALL rules in `graph.rules`
7. **TODO**: Update List/Hash methods to apply transformation rules
8. **TODO**: Update executor to use unified RuleSpec
9. **TODO**: Fix/update behaviors.rs (rename to transformation_rules.rs?)
10. **TODO**: Update all test imports
11. **TODO**: Verify all 398 tests still pass

### Current Progress

**Step 1 Complete**: RuleSpec now includes all transformation rule variants:
- `NoneToZero`, `NoneToEmpty`, `Positive`, `RoundToInt`
- `Uppercase`, `Lowercase`
- `ValidateRange`, `Mapping`, `CustomFunction`, `Conditional`, `Ordering`

**Remaining Work**: Steps 2-11 (see UNIFICATION_PLAN.md)

---

## 🎯 Immediate Next Steps

1. **Read**: `UNIFICATION_PLAN.md` - Complete implementation plan
2. **Execute**: Steps 2-11 systematically
3. **Test**: After each step, run `cargo build` and `cargo test`
4. **Verify**: All 398 tests passing with zero warnings
5. **Confirm**: No "behaviors" references in core data structures

### Recommended Approach

Work through the plan step-by-step:
- Don't skip steps
- Test after each change
- Fix issues immediately before proceeding
- Keep the architecture clean and unified

---

## 📁 Key Files

### Files Modified So Far

1. **`src/graph/rules.rs`** - Added transformation rule variants to RuleSpec enum

### Files That Need Modification

1. **`src/graph/rules.rs`** - Add from_symbol(), name(), extend trait
2. **`src/graph/behaviors.rs`** - Adapt to implement Rule trait or merge
3. **`src/values/list.rs`** - Remove behaviors field, update methods
4. **`src/values/hash.rs`** - Remove behaviors field, update methods
5. **`src/execution/executor.rs`** - Use RuleSpec only, no BehaviorSpec
6. **All test files** - Update imports from BehaviorSpec to RuleSpec

---

## 💡 Key Architectural Principles

### What "Rules All The Way Down" Means

**User-Facing**:
```graphoid
# Everything uses .add_rule()
my_tree.add_rule(:no_cycles)        # Validation rule
temperatures.add_rule(:none_to_zero)  # Transformation rule
```

**Internal Storage**:
```rust
pub struct List {
    pub graph: Graph,  // ALL rules stored in graph.rules
    length: usize,
    // NO behaviors field!
}
```

**Rule Types**:
```rust
pub enum RuleSpec {
    // Validation rules (structural constraints)
    NoCycles, SingleRoot, MaxDegree(usize), ...

    // Transformation rules (value transformations)
    NoneToZero, Uppercase, ValidateRange { min, max }, ...
}
```

### Why This Matters

1. **Specification Compliance**: The language spec says "rules", so everything should be rules
2. **Conceptual Clarity**: Users see one unified system, not two separate systems
3. **Code Simplicity**: One storage location (graph.rules), one API, one mental model
4. **Consistency**: If it's a "rule" in the API, it should be a "rule" internally

---

## 🔍 Verification Commands

```bash
# Check compilation
~/.cargo/bin/cargo build

# Run all tests
~/.cargo/bin/cargo test

# Check for "behavior" references (should only be in comments/history)
grep -r "behavior" src/ --include="*.rs" | grep -v "// "

# Verify rule count in core structures
grep -n "pub behaviors:" src/values/*.rs  # Should find NOTHING
```

---

## 📚 Background Context

### Phase 7 History

Phase 7 implemented the "Intrinsic Behavior System" following the spec's terminology:
- 75 transformation rule tests (all passing)
- Standard rules: none_to_zero, uppercase, positive, etc.
- Mapping, custom function, conditional, ordering rules
- Retroactive + proactive application

**The Problem**: Implementation used separate "behaviors" field and BehaviorSpec enum, violating the "rules all the way down" principle.

### The Fix

Unify everything under the Rule system:
- Single RuleSpec enum
- Single storage location (graph.rules)
- Single API (.add_rule(), .remove_rule(), etc.)
- Clean, spec-compliant architecture

---

## ⚠️ Important Notes

- **DO NOT** start Phase 8 until this refactoring is complete
- **DO NOT** rush the refactoring - follow the plan systematically
- **TEST** after each step to catch issues early
- **VERIFY** the final result matches the specification exactly

---

## 🎉 When Complete

After completing the unification:
- All 398 tests passing ✅
- Zero warnings ✅
- No "behaviors" in core data structures ✅
- Architecture matches specification ✅
- Ready to proceed to Phase 8 (Module System) ✅

---

## 🚀 Ready to Start

Read `UNIFICATION_PLAN.md` and execute Steps 2-11 systematically.

**This is critical infrastructure work. Take the time to do it right.**

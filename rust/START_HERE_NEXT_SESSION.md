# START HERE - Phase 7.2 Implementation Ready! 🚀

**Last Updated**: October 25, 2025
**Current Status**: ✅ PHASE 7.1 COMPLETE! Phase 7.2 ready to start!
**Tests Passing**: 341/341 (100%)
**What's Next**: 🎯 BEGIN PHASE 7.2 - STANDARD BEHAVIORS

---

## 🎉 Major Milestone: Sub-Phase 7.1 Complete!

### What Was Accomplished (Sub-Phase 7.1)

**Behavior Framework - Foundation Built**:
- ✅ Core Behavior trait defined
- ✅ BehaviorSpec enum with all 5 behavior types
- ✅ BehaviorInstance wrapper with RetroactivePolicy support
- ✅ apply_behaviors() helper function
- ✅ apply_retroactive_to_list() function with 4 policy modes
- ✅ All collections (List, Hash, Graph) have behaviors field
- ✅ 18 framework tests passing
- ✅ Zero compiler warnings
- ✅ Comprehensive rustdoc documentation

**Deliverables**:
1. `src/graph/behaviors.rs` (630 lines) - Core behavior system
2. `tests/unit/behavior_framework_tests.rs` (18 tests)
3. Updated `List`, `Hash`, `Graph` with `behaviors: Vec<BehaviorInstance>`

**Test Breakdown** (18 tests):
- Framework tests: 10
- Storage tests: 3
- RetroactivePolicy tests: 4
- Proactive test: 1 (placeholder)

**Key Features**:
- **Retroactive Policies**:
  - Clean (default): Transform all existing values
  - Warn: Keep existing, print warnings
  - Enforce: Error if transformation needed
  - Ignore: Skip existing values
- **Sequential Application**: First added = first applied
- **Type Filtering**: `applies_to()` skips non-applicable values

---

## 🚀 Starting Phase 7.2: Standard Behaviors

### Goal
Implement the 7 built-in standard behaviors with full transformation logic, replacing the stub implementations from 7.1.

### Duration
2-3 days

### What You're Building
Complete implementations of standard value transformations:

1. **`:none_to_zero`** - Transform `none` to `0`
2. **`:none_to_empty`** - Transform `none` to `""`
3. **`:positive`** - Make numbers positive (absolute value)
4. **`:round_to_int`** - Round numbers to nearest integer
5. **`:uppercase`** - Convert strings to uppercase
6. **`:lowercase`** - Convert strings to lowercase
7. **`:validate_range`** - Clamp numbers to [min, max] range

### TDD Workflow (Red → Green → Refactor)

#### Step 1: RED Phase - Write Tests FIRST (20 tests)

Create `tests/unit/standard_behaviors_tests.rs` with 20 tests:

**Basic Transformation Tests** (7 tests):
1. `test_none_to_zero_transforms_none()` - none → 0
2. `test_none_to_empty_transforms_none()` - none → ""
3. `test_positive_makes_negative_positive()` - -5 → 5
4. `test_round_to_int_rounds_numbers()` - 3.7 → 4.0
5. `test_uppercase_converts_string()` - "hello" → "HELLO"
6. `test_lowercase_converts_string()` - "HELLO" → "hello"
7. `test_validate_range_clamps_numbers()` - 110 clamped to 100

**Edge Cases** (7 tests):
8. `test_none_to_zero_ignores_non_none()` - Numbers unchanged
9. `test_positive_ignores_already_positive()` - 5 → 5
10. `test_uppercase_ignores_non_strings()` - Numbers unchanged
11. `test_validate_range_within_range()` - No clamping needed
12. `test_none_to_empty_only_affects_none()` - Strings unchanged
13. `test_round_to_int_already_integer()` - 5.0 → 5.0
14. `test_lowercase_empty_string()` - "" → ""

**Integration Tests** (6 tests):
15. `test_multiple_behaviors_chain()` - none → 0 → abs() → round()
16. `test_behavior_order_matters()` - Verify first-added-first-applied
17. `test_list_with_none_to_zero()` - List of [none, 1, none] → [0, 1, 0]
18. `test_hash_with_uppercase()` - Hash values uppercased
19. `test_retroactive_clean_transforms()` - Existing values transformed
20. `test_proactive_on_append()` - New values transformed on append

**Run tests**: `~/.cargo/bin/cargo test --test standard_behaviors_tests`
**Expected**: All 20 tests FAIL (no implementation yet)

#### Step 2: GREEN Phase - Make Tests Pass

**Implementation Order**:

1. **Update stub implementations in `src/graph/behaviors.rs`**:
   - Already done for NoneToZero, NoneToEmpty, Positive, RoundToInt, Uppercase, Lowercase
   - Just need to verify ValidateRange implementation
   - Remove stub comments

2. **Wire behaviors into List operations** (Sub-phase 7.2.5):
   - Modify `List::append()` to apply behaviors
   - Modify `List::set()` to apply behaviors
   - Modify `List::insert()` to apply behaviors (if exists)

3. **Wire behaviors into Hash operations**:
   - Modify `Hash::insert()` to apply behaviors
   - Modify `Hash::set()` to apply behaviors (if exists)

4. **Add behavior management methods**:
   - `add_behavior(behavior_instance)` - Add and apply retroactively
   - `get_behaviors() -> &[BehaviorInstance]` - Introspection

**Run tests**: `~/.cargo/bin/cargo test --test standard_behaviors_tests`
**Expected**: All 20 tests PASS

#### Step 3: REFACTOR Phase - Polish

- Clean up any duplicated code
- Ensure zero compiler warnings
- Add comprehensive rustdoc comments
- Verify test names are clear
- Consider extracting common behavior logic

### Acceptance Criteria

- ✅ 20 tests passing
- ✅ All 7 standard behaviors fully implemented
- ✅ Behaviors wired into List::append(), List::set()
- ✅ Behaviors wired into Hash::insert()
- ✅ add_behavior() method on collections
- ✅ Retroactive application works correctly
- ✅ Proactive application works correctly
- ✅ Zero compiler warnings
- ✅ All code documented with rustdoc comments

---

## 📋 Phase 7 Overview

### Seven Sub-Phases

| Sub-Phase | Duration | Focus | Tests | Status |
|-----------|----------|-------|-------|--------|
| **7.1** | 1-2 days | Behavior Framework | 18 | ✅ COMPLETE |
| **7.2** | 2-3 days | Standard Behaviors | 20 | 🔜 NEXT |
| **7.3** | 1-2 days | Mapping Behaviors | 10 | Pending |
| **7.4** | 2-3 days | Custom/Conditional | 15 | Pending |
| **7.5** | 1-2 days | Ordering Behaviors | 12 | Pending |
| **7.6** | 1 day | Behavior Management | 8 | Pending |
| **7.7** | 0.5-1 day | Quality Gate | 12 | Pending |

**Total**: 8-11 days, 95+ tests
**Progress**: 18/95 tests (19% complete)

---

## 📁 Key Files Reference

### Phase 7 Planning Documents (dev_docs/)

1. **PHASE_7_BEHAVIOR_SYSTEM_PLAN.md** (1680 lines)
   - Complete implementation plan
   - Read lines 392-600 for Sub-Phase 7.2 details
   - All 7 sub-phases detailed

2. **PHASE_7_5_ORDERING_BEHAVIORS.md**
   - Ordering behavior specification
   - Needed for Sub-Phase 7.5

### Files You'll Create Today (Sub-Phase 7.2)

1. **`tests/unit/standard_behaviors_tests.rs`** (~500 lines)
   - 20 standard behavior tests

### Files You'll Modify Today

1. **`src/graph/behaviors.rs`** - Verify/complete standard behavior implementations
2. **`src/values/list.rs`** - Wire behaviors into append(), set()
3. **`src/values/hash.rs`** - Wire behaviors into insert()
4. **`tests/unit/mod.rs`** - Add standard_behaviors_tests module

---

## 💡 Implementation Tips

### Behavior Application Pattern

When modifying collection operations (append, insert, set):

```rust
pub fn append(&mut self, value: Value) -> Result<(), GraphoidError> {
    // Apply behaviors to incoming value
    let transformed = apply_behaviors(value, &self.behaviors)?;

    // Proceed with normal append logic
    let new_id = format!("node_{}", self.length);
    self.graph.add_node(new_id.clone(), transformed)?;

    // ... rest of append logic
    Ok(())
}
```

### add_behavior() Method Pattern

```rust
pub fn add_behavior(&mut self, behavior: BehaviorInstance) -> Result<(), GraphoidError> {
    // Apply retroactively based on policy
    apply_retroactive_to_list(self, &behavior)?;

    // Add to behaviors list for future proactive application
    self.behaviors.push(behavior);

    Ok(())
}
```

---

## 🔍 Verification Commands

```bash
# Run all tests
~/.cargo/bin/cargo test

# Run just standard behavior tests
~/.cargo/bin/cargo test --test standard_behaviors_tests

# Check for warnings
~/.cargo/bin/cargo build --quiet 2>&1 | grep -i warning || echo "Zero warnings"

# Count tests
~/.cargo/bin/cargo test 2>&1 | grep "test result:"
```

**Current Baseline**: 341 tests passing from Phase 7.1
**Target After 7.2**: 361 tests passing (+20)

---

## 📚 Required Reading

Before starting implementation:

1. **`dev_docs/PHASE_7_BEHAVIOR_SYSTEM_PLAN.md`**
   - Read lines 392-600 (Sub-Phase 7.2 section)
   - Review the 20 test cases
   - Understand proactive vs retroactive application

2. **`src/graph/behaviors.rs`**
   - Review existing stub implementations
   - Understand Behavior trait
   - See how BehaviorSpec::instantiate() works

3. **`src/values/list.rs`**
   - Review append(), set() methods
   - Understand how to modify them

---

## 🎯 Success Criteria for Sub-Phase 7.2

### Code Quality
- ✅ Zero compiler warnings
- ✅ Zero clippy warnings
- ✅ All public APIs documented
- ✅ Clean, idiomatic Rust

### Tests
- ✅ 20/20 tests passing
- ✅ Tests written FIRST (TDD)
- ✅ Clear test names
- ✅ Comprehensive coverage

### Functionality
- ✅ All 7 standard behaviors work correctly
- ✅ Retroactive application works
- ✅ Proactive application works
- ✅ Behaviors wire into List and Hash operations
- ✅ add_behavior() method available

### Ready for Sub-Phase 7.3
- ✅ Standard behaviors solid and tested
- ✅ Collection operations properly wire behaviors
- ✅ Pattern established for adding new behavior types

---

## 🎉 Ready for Sub-Phase 7.2!

**First Step**: Read `dev_docs/PHASE_7_BEHAVIOR_SYSTEM_PLAN.md` lines 392-600 (Sub-Phase 7.2)

**Then**: Write the 20 standard behavior tests FIRST, watch them fail

**Finally**: Implement behavior wiring to make them pass

**Remember**: Red → Green → Refactor

---

**TDD is working great! Foundation is solid. Let's implement the standard behaviors! 🚀**

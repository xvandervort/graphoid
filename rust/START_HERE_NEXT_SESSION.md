# START HERE - Phase 7.5 Ready! 🚀

**Last Updated**: October 25, 2025
**Current Status**: ✅ PHASE 7.4 COMPLETE! Phase 7.5 ready to start!
**Tests Passing**: 386/386 (100%)
**What's Next**: 🎯 BEGIN PHASE 7.5 - ORDERING BEHAVIORS

---

## 🎉 Major Milestone: Sub-Phase 7.4 Complete!

### What Was Accomplished (Sub-Phase 7.4)

**Custom Function & Conditional Behaviors - Fully Implemented**:
- ✅ CustomFunction behavior with user-defined functions
- ✅ Conditional behavior with predicate-based transformations
- ✅ Executor-level behavior application with function execution context
- ✅ `apply_behaviors_with_context()` method in executor
- ✅ Support for closures and captured variables
- ✅ Retroactive and proactive application
- ✅ 15 custom/conditional behavior tests passing
- ✅ Zero compiler warnings

**Deliverables**:
1. `tests/unit/custom_conditional_behaviors_tests.rs` (15 tests)
2. `Executor::apply_behaviors_with_context()` method for function-based behaviors
3. `List::append_raw()`, `List::set_raw()`, `Hash::insert_raw()` internal methods
4. Behavior application wired into executor's index assignment operations

**Key Architecture**:

Two-level behavior application system:
1. **Standard Behaviors**: Use Behavior trait's `transform()` method (no executor context needed)
2. **Function-based Behaviors**: Handled specially in `apply_behaviors_with_context()` with full executor context

```rust
// In executor
pub fn apply_behaviors_with_context(
    &mut self,
    value: Value,
    behaviors: &[BehaviorInstance],
) -> Result<Value> {
    for behavior in behaviors {
        match &behavior.spec {
            BehaviorSpec::CustomFunction { function } => {
                // Call user function with executor context
                current = self.call_function(function, &[current])?;
            }
            BehaviorSpec::Conditional { condition, transform, fallback } => {
                // Evaluate predicate, apply transform or fallback
            }
            _ => {
                // Use standard Behavior trait
                current = behavior.spec.instantiate().transform(&current)?;
            }
        }
    }
    Ok(current)
}
```

**Test Coverage**:
- Custom function basic transformation ✅
- Custom function with closures ✅
- Type-specific behavior errors ✅
- Error handling (division by zero) ✅
- Retroactive application ✅
- Proactive application ✅
- Conditional basic (if-then) ✅
- Conditional with fallback (if-then-else) ✅
- Conditional without fallback ✅
- Conditional chains (multiple predicates) ✅
- List integration ✅
- Mixed standard + custom behaviors ✅

---

## 🚀 Starting Phase 7.5: Ordering Behaviors

### Goal
Implement ordering/sorting behaviors that automatically maintain sorted order in collections.

### Duration
1-2 days

### What You're Building

Ordering behaviors that keep lists sorted:

**Example Default Ordering**:
```graphoid
numbers = [3, 1, 4, 1, 5, 9]
numbers.add_ordering_rule()  # Default numeric ordering
# Result: [1, 1, 3, 4, 5, 9]

numbers.append(2)
# Result: [1, 1, 2, 3, 4, 5, 9]  # Automatically inserted in correct position
```

**Example Custom Ordering**:
```graphoid
words = ["cat", "elephant", "dog"]

# Sort by length
func by_length(a, b) {
    return len(a) - len(b)
}

words.add_ordering_rule(by_length)
# Result: ["cat", "dog", "elephant"]
```

### TDD Workflow (Red → Green → Refactor)

#### Step 1: RED Phase - Write Tests FIRST (12 tests)

Create `tests/unit/ordering_behaviors_tests.rs` with 12 tests:

**Default Ordering Tests** (4 tests):
1. `test_ordering_numbers_default()` - Default numeric sort
2. `test_ordering_strings_default()` - Default string sort (lexicographic)
3. `test_ordering_retroactive()` - Existing values sorted when rule added
4. `test_ordering_proactive()` - New values inserted in sorted position

**Custom Ordering Tests** (4 tests):
5. `test_ordering_custom_function()` - User-defined comparison function
6. `test_ordering_reverse()` - Reverse ordering
7. `test_ordering_by_field()` - Sort by specific field/property
8. `test_ordering_stability()` - Stable sort (equal elements maintain order)

**Integration Tests** (4 tests):
9. `test_list_maintains_order()` - List stays sorted after multiple operations
10. `test_ordering_with_other_behaviors()` - Ordering + transformation behaviors
11. `test_ordering_edge_cases()` - Empty list, single element
12. `test_ordering_duplicate_values()` - Multiple equal values

**Run tests**: `~/.cargo/bin/cargo test --test ordering_behaviors_tests`
**Expected**: All 12 tests FAIL (no implementation yet)

#### Step 2: GREEN Phase - Make Tests Pass

**Implementation Order**:

1. **Complete OrderingBehavior in `src/graph/behaviors.rs`**:
   ```rust
   impl Behavior for OrderingBehavior {
       fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
           // Ordering doesn't transform individual values
           // It's applied at collection level when values are added
           Ok(value.clone())
       }
   }
   ```

2. **Add ordering logic to List** in executor:
   - When appending with ordering behavior, find correct insertion point
   - Use binary search for efficiency
   - Support custom comparison functions via executor context

3. **Implement default ordering** for different value types:
   - Numbers: numeric comparison
   - Strings: lexicographic comparison
   - Booleans: false < true
   - None: always sorts first

**Run tests**: `~/.cargo/bin/cargo test --test ordering_behaviors_tests`
**Expected**: All 12 tests PASS

#### Step 3: REFACTOR Phase - Polish

- Add comprehensive rustdoc comments
- Optimize binary search insertion
- Test edge cases
- Zero warnings

### Acceptance Criteria

- ✅ 12 tests passing
- ✅ OrderingBehavior implemented
- ✅ Default ordering for all value types
- ✅ Custom comparison function support
- ✅ Efficient binary search insertion
- ✅ Retroactive and proactive sorting work
- ✅ Zero compiler warnings
- ✅ All code documented with rustdoc comments

---

## 📋 Phase 7 Overview

### Seven Sub-Phases

| Sub-Phase | Duration | Focus | Tests | Status |
|-----------|----------|-------|-------|--------|
| **7.1** | 1-2 days | Behavior Framework | 18 | ✅ COMPLETE |
| **7.2** | 2-3 days | Standard Behaviors | 20 | ✅ COMPLETE |
| **7.3** | 1-2 days | Mapping Behaviors | 10 | ✅ COMPLETE |
| **7.4** | 2-3 days | Custom/Conditional | 15 | ✅ COMPLETE |
| **7.5** | 1-2 days | Ordering Behaviors | 12 | 🔜 NEXT |
| **7.6** | 1 day | Behavior Management | 8 | Pending |
| **7.7** | 0.5-1 day | Quality Gate | 12 | Pending |

**Total**: 8-11 days, 95+ tests
**Progress**: 63/95 tests (66% complete)

---

## 📁 Key Files Reference

### Phase 7 Planning Documents (dev_docs/)

1. **PHASE_7_BEHAVIOR_SYSTEM_PLAN.md** (1680 lines)
   - Complete implementation plan
   - Read lines 1050-1200 for Sub-Phase 7.5 details

2. **PHASE_7_5_ORDERING_BEHAVIORS.md** (if exists)
   - Detailed ordering behavior specification

### Files You'll Create (Sub-Phase 7.5)

1. **`tests/unit/ordering_behaviors_tests.rs`** (~400 lines)

### Files You'll Modify

1. **`src/graph/behaviors.rs`** - Complete OrderingBehavior implementation
2. **`src/execution/executor.rs`** - Add sorted insertion logic
3. **`tests/unit_tests.rs`** - Add test module

---

## 💡 Implementation Notes

### Ordering Strategy

**Proactive Application** (when adding new values):
- Find correct position using binary search
- Insert value at that position
- O(log n) search + O(n) insertion

**Retroactive Application** (when adding ordering rule):
- Sort existing list using comparison function
- Replace list contents with sorted version
- O(n log n) sort

**Comparison Functions**:
- Default: Compare values directly (numbers, strings, etc.)
- Custom: User-provided function that returns -1, 0, or 1

**Binary Search in Executor**:
```rust
// When appending to list with ordering behavior
if list has ordering behavior {
    let insert_pos = binary_search(&list, &value, &compare_fn)?;
    list.insert_at(insert_pos, value)?;  // New method needed
} else {
    list.append(value)?;
}
```

---

## 🔍 Verification Commands

```bash
# Run all tests
~/.cargo/bin/cargo test

# Run specific ordering tests
~/.cargo/bin/cargo test --test ordering_behaviors_tests

# Check for warnings
~/.cargo/bin/cargo build --quiet 2>&1 | grep -i warning || echo "Zero warnings"

# Count tests
~/.cargo/bin/cargo test 2>&1 | grep "test result:"
```

**Current Baseline**: 386 tests passing from Sub-Phase 7.4
**Target After 7.5**: 398 tests passing (+12)

---

## 📚 Required Reading

Before starting implementation:

1. **`dev_docs/PHASE_7_BEHAVIOR_SYSTEM_PLAN.md`**
   - Lines 1050-1200 for Sub-Phase 7.5 details
2. **`dev_docs/PHASE_7_5_ORDERING_BEHAVIORS.md`** (if exists)

---

## 🎯 Recommended Next Step

1. **Create test file** `tests/unit/ordering_behaviors_tests.rs` with 12 failing tests (RED phase)
2. **Implement OrderingBehavior** logic in executor (GREEN phase)
3. **Add List::insert_at()** method for inserting at specific position
4. **Test and refine** until all 12 tests pass

---

## 🎉 Sub-Phase 7.4 Complete!

**Custom and conditional behaviors fully working! Function-based behaviors integrate seamlessly with executor context. Phase 7 is 66% done! 🚀**

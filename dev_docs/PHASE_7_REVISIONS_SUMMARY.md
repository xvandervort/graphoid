# Phase 7 Plan Revisions Summary

**Date**: October 24, 2025
**Status**: Revisions in progress based on user feedback

---

## User Feedback Incorporated

### 1. ✅ API Naming: Use `add_rule()` for Both Rules and Behaviors

**Decision**: Behaviors share the `add_rule()` namespace with validation rules.

**Rationale**: Matches language spec, provides consistent API.

**Changes Made**:
- Updated examples to use `temperatures.add_rule(:none_to_zero)` instead of `add_behavior()`
- Added note explaining that rules and behaviors share the same user-facing API
- Internally, they remain separate (rules in Phase 6, behaviors in Phase 7)

**Status**: ✅ Examples updated in executive summary. Need to update all code examples throughout plan.

---

### 2. ✅ Retroactive Behaviors MUST Be Supported

**Decision**: Full RetroactivePolicy support required (Clean, Warn, Enforce, Ignore).

**Changes Made**:
- Added `retroactive_policy: RetroactivePolicy` field to `BehaviorInstance`
- Implemented `BehaviorInstance::with_policy()` constructor
- Updated `apply_retroactive_to_list()` to handle all 4 policies:
  - **Clean** (default): Transform all existing values
  - **Warn**: Keep existing, warn about potential transforms
  - **Enforce**: Error if any would be transformed
  - **Ignore**: Skip existing values entirely
- Added 4 new tests in Sub-Phase 7.1 for each policy

**Status**: ✅ Framework complete (Sub-Phase 7.1 updated, test count: 15 → 18)

---

### 3. ✅ Defer Freeze Behaviors to Phase 11

**Decision**: Freeze behaviors (`:no_frozen`, `:copy_elements`, `:shallow_freeze_only`) deferred until `.freeze()` is implemented.

**Changes Made**:
- Removed freeze behaviors from BehaviorSpec enum in Sub-Phase 7.1
- Added note: "Freeze behaviors deferred to Phase 11"
- Updated Sub-Phase 7.2 test count: 20 → 18 (removed 2 freeze behavior tests)

**Status**: ✅ Phase 7 plan updated. ⏳ Need to update Phase 11 plan.

---

### 4. ✅ Ordering is a Behavior for ALL Graphs (Not Just BSTs)

**Decision**: Implement ordering behavior that works for any ordered collection, not BST-specific.

**Design**:
```graphoid
# Default ordering (uses < comparison)
numbers = [3, 1, 4, 1, 5, 9]
numbers.add_rule(:maintain_order)
# Existing values sorted: [1, 1, 3, 4, 5, 9]
numbers.append(2)  # Inserted in sorted position: [1, 1, 2, 3, 4, 5, 9]

# Custom comparison function
strings = ["apple", "Banana", "cherry"]
strings.add_rule(:ordering, compare_fn: x, y => x.lower() < y.lower())
# Case-insensitive sorting
```

**Changes Made**:
- Added `Ordering { compare_fn: Option<Value> }` to BehaviorSpec enum
- Created new Sub-Phase 7.5: Ordering Behaviors (1-2 days, 12 tests)
- Renumbered:
  - Old 7.5 (Management) → New 7.6
  - Old 7.6 (Quality Gate) → New 7.7
- Updated phase overview table
- Updated total test count: 68 → 93 tests
- Updated duration: 7-10 days → 8-11 days

**Status**: ✅ Overview updated. ⏳ Need to write detailed Sub-Phase 7.5 section.

---

## Updated Phase Structure

| Sub-Phase | Duration | Focus | Tests | Status |
|-----------|----------|-------|-------|--------|
| **7.1** | 1-2 days | Behavior Framework + RetroactivePolicy | 18 | ✅ Updated |
| **7.2** | 2-3 days | Standard Behaviors (6, no freeze) | 18 | ⏳ Need to remove freeze examples |
| **7.3** | 1-2 days | Mapping Behaviors | 10 | ⏳ Need `add_rule()` updates |
| **7.4** | 2-3 days | Custom/Conditional Behaviors | 15 | ⏳ Need `add_rule()` updates |
| **7.5** | 1-2 days | **Ordering Behaviors** (NEW) | 12 | ⏳ Need to write section |
| **7.6** | 1 day | Behavior Management | 8 | ⏳ Need `add_rule()` updates |
| **7.7** | 0.5-1 day | Quality Gate | 12 | ⏳ Need ordering integration tests |

**Total**: 8-11 days, 93 tests

---

## Remaining Work

### High Priority

1. **Sub-Phase 7.5: Write Ordering Behaviors Section** ⏳
   - Design ordering behavior implementation
   - Define default comparison (uses < operator)
   - Define custom comparison function API
   - Retroactive: sorts existing values
   - Proactive: inserts new values in sorted position
   - Write 12 tests

2. **Update All Code Examples** ⏳
   - Change `add_behavior()` → `add_rule()` throughout
   - Change `add_mapping_behavior()` → `add_mapping_rule()`
   - Change `add_custom_behavior()` → `add_custom_rule()`
   - Change `add_conditional_behavior()` → `add_conditional_rule()`
   - Update in Sub-Phases 7.2, 7.3, 7.4, 7.6

3. **Sub-Phase 7.2: Remove Freeze Behaviors** ⏳
   - Remove `:no_frozen` and `:copy_elements` test examples
   - Update test count: 20 → 18
   - Update acceptance criteria

### Medium Priority

4. **Update Phase 11 Plan** ⏳
   - Add freeze behaviors to Phase 11 scope
   - Note dependency on `.freeze()` implementation
   - Estimate 2-3 days for freeze behaviors

5. **Update Questions Section** ⏳
   - Remove questions 1, 3, 4 (answered)
   - Keep question 2 about RetroactivePolicy (now confirmed YES)

### Low Priority

6. **Update Success Metrics** ⏳
   - Change "68+ tests" → "93+ tests"
   - Change "~397 total" → "~422 total" (329 current + 93 new)

---

## Implementation Notes

### Ordering Behavior Design Considerations

**Key Questions to Address in 7.5**:

1. **How does default ordering work?**
   - Use Rust's PartialOrd trait?
   - Fall back to string comparison for mixed types?
   - Error on incomparable types?

2. **How does insertion maintain order?**
   - Binary search for insertion point?
   - Linear scan? (simpler, works for small lists)
   - Performance trade-offs?

3. **What about graphs with multiple edges?**
   - Ordering only makes sense for linear structures (lists)
   - For trees: ordering defines left/right child placement
   - For DAGs: topological ordering already exists

4. **Retroactive sorting**:
   - Use standard sort algorithm
   - In-place vs new sorted list?
   - Stable sort?

**Proposed API**:
```graphoid
# Simple ordering (default comparison)
list.add_rule(:maintain_order)

# Custom ordering
list.add_rule(:ordering, fn(a, b) { a.property < b.property })
# Or with named parameter:
list.add_rule(:ordering, compare: sort_function)
```

---

## Next Steps

1. Write detailed Sub-Phase 7.5 section (Ordering Behaviors)
2. Search and replace all `add_behavior()` → `add_rule()` in Sub-Phases 7.2-7.6
3. Remove freeze behavior examples from Sub-Phase 7.2
4. Update Phase 11 plan
5. Final review and consistency check

---

## Summary

**Completed**:
- ✅ API naming decision (add_rule for both)
- ✅ RetroactivePolicy fully integrated into Sub-Phase 7.1
- ✅ Freeze behaviors deferred (removed from 7.1, 7.2)
- ✅ Ordering behavior added to plan structure

**In Progress**:
- ⏳ Writing Sub-Phase 7.5 (Ordering Behaviors)
- ⏳ Updating code examples throughout

**Pending**:
- ⏳ Phase 11 plan update
- ⏳ Final consistency review

**Estimated Completion**: 1-2 hours of additional editing

---

**Ready to proceed with implementation once revisions complete!**

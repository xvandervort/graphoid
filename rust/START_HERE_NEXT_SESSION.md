# START HERE - Phase 7.4 Implementation Ready! 🚀

**Last Updated**: October 25, 2025
**Current Status**: ✅ PHASE 7.3 COMPLETE! Phase 7.4 ready to start!
**Tests Passing**: 371/371 (100%)
**What's Next**: 🎯 BEGIN PHASE 7.4 - CUSTOM/CONDITIONAL BEHAVIORS

---

## 🎉 Major Milestone: Sub-Phase 7.3 Complete!

### What Was Accomplished (Sub-Phase 7.3)

**Mapping Behaviors - Fully Implemented**:
- ✅ MappingBehavior fully functional
- ✅ value_to_key() helper for Value → String conversion
- ✅ Default fallback for unmapped keys
- ✅ Works with all Value types
- ✅ Retroactive and proactive application
- ✅ 10 mapping behavior tests passing
- ✅ Zero compiler warnings

**Deliverables**:
1. `tests/unit/mapping_behaviors_tests.rs` (10 tests)
2. `value_to_key()` helper function
3. Complete `MappingBehavior::transform()` implementation

**Key Implementation**:
```rust
fn value_to_key(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => {
            if n.fract() == 0.0 { format!("{}", *n as i64) }
            else { n.to_string() }
        }
        Value::Symbol(s) => s.clone(),
        Value::Boolean(b) => b.to_string(),
        Value::None => "none".to_string(),
        _ => format!("{:?}", value),
    }
}
```

**Mapping Features Verified**:
- Map any value type to any value type ✅
- Default fallback for unmapped keys ✅
- Retroactive transformation ✅
- Proactive transformation ✅
- Integration with List and Hash ✅

---

## 🚀 Starting Phase 7.4: Custom/Conditional Behaviors

### Goal
Implement user-defined custom functions and conditional behaviors with predicate-based transformations.

### Duration
2-3 days

### What You're Building

Two advanced behavior types:

1. **Custom Function Behaviors** - User-defined transformation functions
2. **Conditional Behaviors** - Transform based on predicates

**Example Custom Function**:
```graphoid
# Define custom transformation
func double(x) { return x * 2 }

numbers = [1, 2, 3]
numbers.add_custom_rule(double)
# Result: [2, 4, 6]
```

**Example Conditional Behavior**:
```graphoid
# Transform if condition met
numbers = [-5, 3, -2, 7]

# If negative, make positive; otherwise keep unchanged
numbers.add_conditional_rule(
    func(x) { return x < 0 },      # condition
    func(x) { return -x },          # transform
    func(x) { return x }            # fallback (optional)
)
# Result: [5, 3, 2, 7]
```

### TDD Workflow (Red → Green → Refactor)

#### Step 1: RED Phase - Write Tests FIRST (15 tests)

Create `tests/unit/custom_conditional_behaviors_tests.rs` with 15 tests:

**Custom Function Tests** (6 tests):
1. `test_custom_function_basic()` - Simple transformation function
2. `test_custom_function_with_closure()` - Closure as behavior
3. `test_custom_function_type_specific()` - Only applies to numbers
4. `test_custom_function_retroactive()` - Existing values transformed
5. `test_custom_function_proactive()` - New values transformed
6. `test_custom_function_error_handling()` - Function errors handled

**Conditional Tests** (6 tests):
7. `test_conditional_basic()` - Simple predicate-based transform
8. `test_conditional_with_fallback()` - Fallback for false condition
9. `test_conditional_without_fallback()` - No fallback (keep original)
10. `test_conditional_retroactive()` - Existing values transformed
11. `test_conditional_proactive()` - New values transformed
12. `test_conditional_chain()` - Multiple conditionals

**Integration Tests** (3 tests):
13. `test_list_with_custom_function()` - List transformation
14. `test_list_with_conditional()` - Conditional on list
15. `test_mixed_behaviors()` - Standard + custom + conditional

**Run tests**: `~/.cargo/bin/cargo test --test custom_conditional_behaviors_tests`
**Expected**: All 15 tests FAIL (no implementation yet)

#### Step 2: GREEN Phase - Make Tests Pass

**Implementation Order**:

1. **Complete CustomFunctionBehavior in `src/graph/behaviors.rs`**:
   ```rust
   impl Behavior for CustomFunctionBehavior {
       fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
           // Execute function with value as argument
           match &self.function {
               Value::Function(func) => {
                   // Call function with value
                   func.call(vec![value.clone()])
               }
               _ => Err(GraphoidError::runtime("Custom behavior requires function"))
           }
       }
   }
   ```

2. **Complete ConditionalBehavior in `src/graph/behaviors.rs`**:
   ```rust
   impl Behavior for ConditionalBehavior {
       fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
           // Evaluate condition
           let condition_result = evaluate_predicate(&self.condition, value)?;

           if condition_result {
               // Apply transform
               apply_function(&self.transform, value)
           } else if let Some(ref fallback) = self.fallback {
               // Apply fallback
               apply_function(fallback, value)
           } else {
               // No fallback, keep original
               Ok(value.clone())
           }
       }
   }
   ```

3. **Note**: Function execution depends on Phase 4 Function implementation
   - For Sub-Phase 7.4, we may need to stub or skip function-based tests
   - Or implement minimal function support for testing

**Run tests**: `~/.cargo/bin/cargo test --test custom_conditional_behaviors_tests`
**Expected**: All 15 tests PASS

#### Step 3: REFACTOR Phase - Polish

- Add comprehensive rustdoc comments
- Verify error handling
- Test edge cases
- Zero warnings

### Acceptance Criteria

- ✅ 15 tests passing (or documented as pending if Functions not ready)
- ✅ CustomFunctionBehavior implemented
- ✅ ConditionalBehavior implemented
- ✅ Retroactive and proactive application work
- ✅ Error handling for invalid functions
- ✅ Zero compiler warnings
- ✅ All code documented with rustdoc comments

### **IMPORTANT NOTE**: Function Implementation Dependency

Sub-Phase 7.4 depends on Function values from Phase 4, which may not be fully implemented yet. Options:

1. **Option A**: Skip 7.4 and move to 7.5 (Ordering Behaviors)
2. **Option B**: Implement minimal function support for testing
3. **Option C**: Write tests but mark as `#[ignore]` until Phase 4

**Recommendation**: Check if Value::Function is usable. If not, **skip to 7.5**.

---

## 📋 Phase 7 Overview

### Seven Sub-Phases

| Sub-Phase | Duration | Focus | Tests | Status |
|-----------|----------|-------|-------|--------|
| **7.1** | 1-2 days | Behavior Framework | 18 | ✅ COMPLETE |
| **7.2** | 2-3 days | Standard Behaviors | 20 | ✅ COMPLETE |
| **7.3** | 1-2 days | Mapping Behaviors | 10 | ✅ COMPLETE |
| **7.4** | 2-3 days | Custom/Conditional | 15 | 🔜 NEXT (or SKIP) |
| **7.5** | 1-2 days | Ordering Behaviors | 12 | Alternative Next |
| **7.6** | 1 day | Behavior Management | 8 | Pending |
| **7.7** | 0.5-1 day | Quality Gate | 12 | Pending |

**Total**: 8-11 days, 95+ tests
**Progress**: 48/95 tests (51% complete)

---

## 📁 Key Files Reference

### Phase 7 Planning Documents (dev_docs/)

1. **PHASE_7_BEHAVIOR_SYSTEM_PLAN.md** (1680 lines)
   - Complete implementation plan
   - Read lines 850-1050 for Sub-Phase 7.4 details
   - Read lines 1050-1200 for Sub-Phase 7.5 (Ordering) if skipping 7.4

### Files You'll Create (Sub-Phase 7.4 or 7.5)

If doing **7.4** (Custom/Conditional):
1. **`tests/unit/custom_conditional_behaviors_tests.rs`** (~500 lines)

If doing **7.5** (Ordering):
1. **`tests/unit/ordering_behaviors_tests.rs`** (~400 lines)

### Files You'll Modify

1. **`src/graph/behaviors.rs`** - Complete CustomFunctionBehavior and ConditionalBehavior
2. **`tests/unit/mod.rs`** - Add test module

---

## 💡 Decision Point: 7.4 or 7.5?

### Check Function Implementation Status

```bash
# Check if Value::Function is implemented
grep -A 10 "pub enum Value" src/values/mod.rs | grep Function

# Check if Function struct exists
grep -n "pub struct Function" src/values/*.rs
```

**If Functions are NOT ready**:
- Skip to Sub-Phase 7.5 (Ordering Behaviors)
- Ordering doesn't depend on Functions
- Come back to 7.4 after Phase 4

**If Functions ARE ready**:
- Continue with Sub-Phase 7.4
- Implement custom function behaviors

---

## 🔍 Verification Commands

```bash
# Run all tests
~/.cargo/bin/cargo test

# Run specific behavior tests
~/.cargo/bin/cargo test --test custom_conditional_behaviors_tests
# OR
~/.cargo/bin/cargo test --test ordering_behaviors_tests

# Check for warnings
~/.cargo/bin/cargo build --quiet 2>&1 | grep -i warning || echo "Zero warnings"

# Count tests
~/.cargo/bin/cargo test 2>&1 | grep "test result:"
```

**Current Baseline**: 371 tests passing from Phase 7.3
**Target After 7.4**: 386 tests passing (+15)
**Target After 7.5**: 383 tests passing (+12)

---

## 📚 Required Reading

Before starting implementation:

1. **Check Function status first**
2. **`dev_docs/PHASE_7_BEHAVIOR_SYSTEM_PLAN.md`**
   - Lines 850-1050 for Sub-Phase 7.4
   - Lines 1050-1200 for Sub-Phase 7.5
3. **`dev_docs/PHASE_7_5_ORDERING_BEHAVIORS.md`** if doing 7.5

---

## 🎯 Recommended Next Step

1. **Check if Functions are implemented** (see Decision Point above)
2. **If NO**: Skip to Sub-Phase 7.5 (Ordering Behaviors)
3. **If YES**: Continue with Sub-Phase 7.4 (Custom/Conditional)

---

## 🎉 Ready for Next Sub-Phase!

**Mapping behaviors complete! Foundation is solid. Phase 7 is 51% done! 🚀**

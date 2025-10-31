# Phase 7: Behavior System - Detailed Implementation Plan

**Duration**: 5-7 days
**Status**: Partially implemented (75 tests passing)
**Goal**: Complete the intrinsic behavior system for automatic value transformation

---

## Overview

The Behavior System allows data structures (lists, hashes, graphs) to automatically transform values during operations like `append`, `insert`, and `set`. This is a core feature that makes Graphoid's collections "self-aware" and intelligent.

**Current Status**:
- ✅ Behavior framework exists (`src/graph/behaviors.rs`, 1005 lines)
- ✅ 75 behavior tests passing
- ⏳ Missing: Complete integration with all collection types
- ⏳ Missing: Full executor support for all behavior types
- ⏳ Missing: Freeze control behaviors

---

## Architecture Summary

**Files Involved**:
- `src/graph/behaviors.rs` - Behavior definitions and implementations
- `src/graph/rules.rs` - Rule system (behaviors use rules)
- `src/values/list.rs` - List behavior integration
- `src/values/hash.rs` - Hash behavior integration
- `src/values/graph.rs` - Graph behavior integration
- `src/execution/executor.rs` - Behavior application in execution

**Behavior Types** (from spec):
1. **Standard Transformations** - Built-in named transformations
2. **Mapping Behaviors** - Hash-based value mappings
3. **Custom Function Behaviors** - User-defined transformation functions
4. **Conditional Behaviors** - Context-aware transformations
5. **Rulesets** - Bundled behavior collections
6. **Freeze Control** - Immutability behaviors

---

## Day 1-2: Standard Transformation Behaviors

### Goal
Complete all standard built-in behaviors from the spec.

### Tasks

#### 1.1 Value Transformation Behaviors
**File**: `src/graph/behaviors.rs`

Ensure these are implemented and tested:
- ✅ `none_to_zero` - Convert none to 0 (verify exists)
- ✅ `none_to_empty` - Convert none to "" (verify exists)
- ✅ `positive` - Absolute value (verify exists)
- ✅ `round_to_int` - Round decimals (verify exists)

**Tests**: 4 tests (one per behavior)
```graphoid
# Example test
temps = [98.6, none, 102.5]
temps.add_rule(:none_to_zero)
# Expected: [98.6, 0, 102.5]
```

#### 1.2 String Transformation Behaviors
**File**: `src/graph/behaviors.rs`

- ✅ `uppercase` - Convert to uppercase (verify exists)
- ✅ `lowercase` - Convert to lowercase (verify exists)

**Tests**: 2 tests

#### 1.3 Validation Behaviors
**File**: `src/graph/behaviors.rs`

- ⏳ `validate_range(min, max)` - Clamp numbers to range

**Implementation**:
```rust
pub struct ValidateRange {
    min: f64,
    max: f64,
}

impl TransformationRule for ValidateRange {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        match value {
            Value::Number(n) => {
                let clamped = n.max(self.min).min(self.max);
                Ok(Value::Number(clamped))
            }
            other => Ok(other.clone()),
        }
    }
}
```

**Tests**: 3 tests (min clamp, max clamp, in-range)

#### 1.4 Executor Integration
**File**: `src/execution/executor.rs`

Ensure `add_rule()` method calls work:
```graphoid
list.add_rule(:none_to_zero)
list.add_rule(:validate_range, 0, 100)
```

**Implementation**: Check `eval_method_call()` handles behavior symbols

**Tests**: 5 integration tests

**Acceptance Criteria**:
- ✅ All 9 standard behaviors implemented
- ✅ Behaviors work retroactively (transform existing values)
- ✅ Behaviors work proactively (transform new values)
- ✅ 14+ tests passing
- ✅ Executor correctly routes `add_rule()` calls

---

## Day 3: Mapping Behaviors

### Goal
Complete hash-based value mapping behaviors.

### Tasks

#### 3.1 Mapping Rule Implementation
**File**: `src/graph/behaviors.rs`

```rust
pub struct MappingBehavior {
    mapping: HashMap<String, Value>,
    default_value: Option<Value>,
}

impl TransformationRule for MappingBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        let key = value.to_string();
        if let Some(mapped) = self.mapping.get(&key) {
            Ok(mapped.clone())
        } else if let Some(default) = &self.default_value {
            Ok(default.clone())
        } else {
            Ok(value.clone())
        }
    }
}
```

#### 3.2 Syntax Support
**File**: `src/execution/executor.rs`

Support this syntax:
```graphoid
status_map = {"active": 1, "inactive": 0}
statuses.add_mapping_rule(status_map, -1)
```

**Implementation**: New method `add_mapping_rule()` in executor

#### 3.3 Chained Mappings
Support multiple mapping stages:
```graphoid
codes.add_mapping_rule(first_map)
codes.add_mapping_rule(second_map)
```

**Tests**: 8 tests
- Basic mapping
- Mapping with default
- Unmapped values
- Chained mappings
- Type conversions
- Empty mapping
- Mapping to none
- Complex values

**Acceptance Criteria**:
- ✅ Mapping behaviors work on lists, hashes
- ✅ Default values supported
- ✅ Chain mappings work correctly
- ✅ 8+ tests passing

---

## Day 4: Custom Function Behaviors

### Goal
Support user-defined transformation functions as behaviors.

### Tasks

#### 4.1 Function-Based Behaviors
**File**: `src/graph/behaviors.rs`

```rust
pub struct CustomFunctionBehavior {
    function: Rc<Function>,
}

impl TransformationRule for CustomFunctionBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        // Call function with value, return result
        // Requires executor context for function calls!
    }
}
```

**Challenge**: Behaviors need executor context to call functions.

**Solution**: Behaviors store function reference, executor applies them with context.

#### 4.2 Syntax Support
```graphoid
fn normalize_temp(value) {
    if value < 95 { return 95 }
    if value > 105 { return 105 }
    return value
}

temperatures.add_custom_rule(normalize_temp)
```

**Implementation**:
- Parse `add_custom_rule(function_name)`
- Store function in behavior
- Apply during append/insert with executor context

#### 4.3 Executor Changes
**File**: `src/execution/executor.rs`

- Modify behavior application to pass executor context
- Handle function calls within behaviors
- Ensure proper scoping and closure support

**Tests**: 10 tests
- Basic custom function
- Function with conditionals
- Function with multiple params (closure)
- Function returning different types
- Function that errors
- Multiple custom functions
- Custom + standard mix
- Recursive function (edge case)
- Function with side effects
- Function accessing closure variables

**Acceptance Criteria**:
- ✅ Functions can be used as behaviors
- ✅ Functions have access to executor context
- ✅ Error handling works correctly
- ✅ 10+ tests passing

---

## Day 5: Conditional Behaviors

### Goal
Context-aware behaviors that only apply when conditions are met.

### Tasks

#### 5.1 Conditional Behavior Implementation
**File**: `src/graph/behaviors.rs`

```rust
pub struct ConditionalBehavior {
    condition_fn: Rc<Function>,
    transform_fn: Rc<Function>,
    fallback_fn: Option<Rc<Function>>,
}

impl TransformationRule for ConditionalBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        // 1. Call condition function
        // 2. If true, apply transform
        // 3. If false and fallback exists, apply fallback
        // 4. Otherwise return original
    }
}
```

#### 5.2 Syntax Support
```graphoid
# With functions
mixed_data.add_rule(is_string, to_upper)

# With fallback
numbers.add_rule(:is_negative, :make_positive, :leave_unchanged)

# Symbol predicates
data.add_rule(:is_string, :uppercase)
```

#### 5.3 Symbol Support
Support built-in symbol predicates and transforms:
- Predicates: `:is_string`, `:is_number`, `:is_negative`, `:is_positive`
- Transforms: `:uppercase`, `:lowercase`, `:double`, `:negate`

**Tests**: 12 tests
- Basic conditional
- Conditional with fallback
- Symbol-based conditional
- Multiple conditions
- Chained conditionals
- Condition returns none
- Transform returns none
- Type mismatches
- Error in condition
- Error in transform
- Mixed types in collection
- Conditional on graph

**Acceptance Criteria**:
- ✅ Conditional behaviors work
- ✅ Fallback functions supported
- ✅ Symbol predicates work
- ✅ 12+ tests passing

---

## Day 6: Rulesets

### Goal
Bundled behavior collections for reusability.

### Tasks

#### 6.1 Ruleset Definition
**File**: `src/graph/behaviors.rs` or new file `src/graph/behavior_rulesets.rs`

```rust
pub struct BehaviorRuleset {
    name: String,
    rules: Vec<RuleInstance>,
}

// Predefined rulesets
pub fn get_behavior_ruleset(name: &str) -> Option<BehaviorRuleset> {
    match name {
        "data_cleaning" => Some(data_cleaning_ruleset()),
        "string_normalization" => Some(string_normalization_ruleset()),
        _ => None,
    }
}

fn data_cleaning_ruleset() -> BehaviorRuleset {
    BehaviorRuleset {
        name: "data_cleaning".to_string(),
        rules: vec![
            RuleInstance::new(RuleSpec::NoneToZero),
            RuleInstance::new(RuleSpec::Positive),
            RuleInstance::new(RuleSpec::RoundToInt),
        ],
    }
}
```

#### 6.2 Syntax Support
```graphoid
# Define ruleset (or use predefined)
data_cleaning = [:none_to_zero, :positive, :round_to_int]

# Apply to collections
temperatures.add_rules(data_cleaning)
blood_pressure.add_rules(data_cleaning)
```

#### 6.3 Predefined Rulesets
Create these standard rulesets:
- `data_cleaning` - none_to_zero, positive, round_to_int
- `string_normalization` - lowercase, trim
- `strict_validation` - validate types, reject none

**Tests**: 8 tests
- Apply ruleset to list
- Apply ruleset to hash
- Custom ruleset definition
- Multiple rulesets on same collection
- Ruleset order matters
- Empty ruleset
- Ruleset with invalid rule
- Predefined rulesets work

**Acceptance Criteria**:
- ✅ Rulesets can be defined
- ✅ `add_rules()` method works
- ✅ Predefined rulesets available
- ✅ 8+ tests passing

---

## Day 7: Freeze Control Behaviors

### Goal
Implement immutability behaviors (freeze system).

### Tasks

#### 7.1 Freeze Behavior Implementation
**File**: `src/graph/behaviors.rs`

```rust
pub struct NoFrozenBehavior;

impl TransformationRule for NoFrozenBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        if value.is_frozen() {
            Err(GraphoidError::runtime("Cannot add frozen elements"))
        } else {
            Ok(value.clone())
        }
    }
}

pub struct CopyElementsBehavior;

impl TransformationRule for CopyElementsBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        // Deep copy value (copies are unfrozen)
        Ok(value.deep_copy_unfrozen())
    }
}
```

#### 7.2 Freeze System Support
**File**: `src/values/mod.rs`

Add freeze support to `Value` enum:
```rust
impl Value {
    pub fn freeze(&mut self) { /* mark as frozen */ }
    pub fn is_frozen(&self) -> bool { /* check frozen state */ }
    pub fn deep_copy_unfrozen(&self) -> Value { /* copy without freeze */ }
}
```

#### 7.3 Behaviors
Implement:
- `no_frozen` - Reject frozen elements
- `copy_elements` - Copy all elements (unfrozen)
- `shallow_freeze_only` - Only freeze collection, not contents

**Tests**: 10 tests
- no_frozen rejects frozen values
- copy_elements creates unfrozen copies
- shallow_freeze_only behavior
- Freeze list doesn't freeze elements
- Freeze nested structures
- Mix frozen and unfrozen
- Error messages clear
- Freeze predicates (:frozen, :unfrozen)
- Freeze on graphs
- Freeze on hashes

**Acceptance Criteria**:
- ✅ Freeze system implemented
- ✅ Freeze behaviors work
- ✅ Predicates support freeze
- ✅ 10+ tests passing

---

## Behavior Management Methods

### Required Methods (all collections)

```graphoid
# Check if behavior exists
has_rule = list.has_rule(:positive)

# Get all active behaviors
behaviors = list.rules()

# Remove specific behavior
list.remove_rule(:positive)

# Clear all behaviors
list.clear_rules()
```

**Implementation**:
- `has_rule(symbol)` -> bool
- `rules()` -> list of symbols
- `remove_rule(symbol)` -> none
- `clear_rules()` -> none

**Tests**: 8 tests (2 per method × 4 methods)

---

## Integration Tests

**File**: `tests/behavior_integration_tests.rs`

Create comprehensive integration tests:
1. Behaviors + type constraints
2. Behaviors + graph rules
3. Behaviors on nested structures
4. Behaviors + method chaining
5. Behaviors + freezing
6. Retroactive + proactive application
7. Order dependence
8. Error propagation
9. Performance with many behaviors
10. Behaviors across module boundaries

**Tests**: 15+ integration tests

---

## Documentation Updates

### Files to Update

1. **Language Specification** (`dev_docs/LANGUAGE_SPECIFICATION.md`)
   - Verify all examples work
   - Add any missing behaviors
   - Document behavior order
   - Document freeze system

2. **Architecture** (`dev_docs/ARCHITECTURE_DESIGN.md`)
   - Explain behavior application
   - Document executor integration
   - Performance considerations

3. **README** (`rust/README.md`)
   - Add behavior system examples
   - Link to spec

---

## Complete Phase 7 Acceptance Criteria

**Standard Behaviors**:
- ✅ All 9 standard transformations implemented
- ✅ Retroactive + proactive application works
- ✅ 14+ tests passing

**Mapping Behaviors**:
- ✅ Hash-based mappings work
- ✅ Chained mappings supported
- ✅ 8+ tests passing

**Custom Functions**:
- ✅ Functions as behaviors work
- ✅ Executor context available
- ✅ 10+ tests passing

**Conditional Behaviors**:
- ✅ Condition + transform + fallback works
- ✅ Symbol predicates supported
- ✅ 12+ tests passing

**Rulesets**:
- ✅ Ruleset definition and application works
- ✅ Predefined rulesets available
- ✅ 8+ tests passing

**Freeze Control**:
- ✅ Freeze system implemented
- ✅ Freeze behaviors work
- ✅ 10+ tests passing

**Management**:
- ✅ has_rule, rules, remove_rule, clear_rules all work
- ✅ 8+ tests passing

**Integration**:
- ✅ 15+ integration tests passing
- ✅ All collection types support behaviors
- ✅ Documentation complete

**Totals**:
- ✅ **85+ new tests** (current: 75, target: 160+)
- ✅ Zero compiler warnings
- ✅ All spec examples work

---

## Risk Assessment

**Low Risk**:
- Standard behaviors (already mostly implemented)
- Mapping behaviors (straightforward)

**Medium Risk**:
- Custom functions (need executor context)
- Conditional behaviors (complex logic)

**High Risk**:
- Freeze system (new feature, affects all types)
- Integration with existing code (may need refactoring)

**Mitigation**:
- TDD approach (write tests first)
- Incremental integration
- Extensive testing at each stage

---

## Success Metrics

1. **Test Coverage**: 160+ behavior tests passing
2. **All Spec Examples**: Every code example in spec works
3. **Performance**: Behaviors add < 10% overhead
4. **Documentation**: Complete and accurate
5. **Zero Warnings**: Clean compilation

---

## Next Phase Preview

**Phase 8** will build on the behavior system by:
- Using behaviors in stdlib modules
- Module-level behavior definitions
- Cross-module behavior sharing

The behavior system is foundational for making Graphoid collections truly "smart" and self-managing!

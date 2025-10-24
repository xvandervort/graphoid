# Phase 7: Behavior System - Detailed Implementation Plan

**Created**: October 24, 2025
**Revised**: October 24, 2025
**Estimated Duration**: 8-11 days
**Prerequisites**: Phase 6.5 complete (329 tests passing)
**Success Criteria**: All standard behaviors working, custom/conditional/ordering behaviors supported, 75+ new tests

---

## Executive Summary

Phase 7 implements the **Intrinsic Behavior System** - a powerful feature that allows data structures to automatically transform values during operations (append, insert, set). Unlike **Rules** (which validate and reject), **Behaviors** transform and accept.

### Key Concepts

**Rules vs Behaviors**:
- **Rules** (Phase 6) - Validate structure, **REJECT** invalid operations
- **Behaviors** (Phase 7) - Transform values, **ACCEPT** after transformation

**API Design** - Rules and behaviors share the `add_rule()` namespace:

**Example**:
```graphoid
# Structural Rule: Prevents cycles (validates, rejects)
my_tree.add_rule(:no_cycles)
my_tree.add_edge("A", "A")  # ❌ REJECTED - would create cycle

# Behavior Rule: Transforms values (transforms, accepts)
temperatures.add_rule(:none_to_zero)
temperatures.append(none)  # ✅ ACCEPTED - becomes 0
```

**Note**: The spec uses `add_rule()` for both validation rules and behavior rules. Internally, they are implemented separately (rules in Phase 6, behaviors in Phase 7), but they share the same user-facing API for consistency.

### Architecture Integration

Behaviors fit into the **Five-Layer Architecture** (spec lines 1426-1458):
- **Layer 2 (Behavior Layer)**: Transformations and validations
- **Layer 3 (Control Layer)**: Rule enforcement (already implemented)

### Five Types of Behaviors

1. **Standard Behaviors** - Built-in transformations (`:none_to_zero`, `:uppercase`, `:positive`)
2. **Mapping Behaviors** - Hash-based value mappings with defaults
3. **Custom Function Behaviors** - User-defined transformation functions
4. **Conditional Behaviors** - Condition + transform + optional fallback
5. **Ordering Behaviors** - Maintain sorted order using comparison functions (works for all ordered collections, not just BSTs)

---

## Phase 7 Sub-Phases Overview

| Sub-Phase | Duration | Focus | Tests | Deliverables |
|-----------|----------|-------|-------|--------------|
| **7.1** | 1-2 days | Behavior Framework | 18 | Trait, BehaviorSpec, RetroactivePolicy |
| **7.2** | 2-3 days | Standard Behaviors | 20 | 7 built-in behaviors (no freeze) |
| **7.3** | 1-2 days | Mapping Behaviors | 10 | add_mapping_rule() with hashes |
| **7.4** | 2-3 days | Custom/Conditional | 15 | Function-based behaviors |
| **7.5** | 1-2 days | Ordering Behaviors | 12 | Maintain sorted order |
| **7.6** | 1 day | Behavior Management | 8 | Inspection, removal APIs |
| **7.7** | 0.5-1 day | Quality Gate | 12 | Final verification |

**Total**: 8-11 days, 95+ tests

**Note**: Freeze behaviors (`:no_frozen`, `:copy_elements`, `:shallow_freeze_only`) deferred to Phase 11 when `.freeze()` is implemented.

---

## Sub-Phase 7.1: Behavior Framework (Days 1-2)

### Goal

Create the foundational infrastructure for behaviors, mirroring the Rule system architecture but with transformation semantics instead of validation.

### Architecture Design

**Key Design Decisions**:

1. **Parallel to Rules**: Behaviors use similar architecture to rules (Behavior trait, BehaviorSpec enum, BehaviorInstance)
2. **Separate from Rules**: Stored separately in collections, different lifecycle
3. **Transform, Don't Validate**: Behaviors return transformed values, rules return validation results
4. **Retroactive + Proactive**: Applied to existing values when added, applied to new values during operations

### Deliverables

#### 1. Create `src/graph/behaviors.rs` (Core Trait System)

**File Structure**:
```rust
//! Behavior system for automatic value transformation
//!
//! Behaviors transform values during operations (append, insert, set).
//! Unlike rules (which validate), behaviors accept values after transformation.

use crate::error::GraphoidError;
use crate::values::Value;

/// Core behavior trait - transforms a value
pub trait Behavior: std::fmt::Debug {
    /// Transform a value according to this behavior
    /// Returns the transformed value
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError>;

    /// Get the name of this behavior
    fn name(&self) -> &str;

    /// Check if this behavior applies to a given value type
    /// Returns true if the behavior should run on this value
    fn applies_to(&self, value: &Value) -> bool {
        // Default: applies to all values
        true
    }
}

/// Specification for a behavior that can be stored and cloned
#[derive(Debug, Clone, PartialEq)]
pub enum BehaviorSpec {
    // Standard value transformations
    NoneToZero,
    NoneToEmpty,
    Positive,
    RoundToInt,

    // String transformations
    Uppercase,
    Lowercase,

    // Validation/clamping
    ValidateRange { min: f64, max: f64 },

    // Mapping behaviors (Sub-phase 7.3)
    Mapping {
        mapping: HashMap<String, Value>,
        default: Value,
    },

    // Custom function behavior (Sub-phase 7.4)
    CustomFunction {
        function: Value,  // Must be Value::Function
    },

    // Conditional behavior (Sub-phase 7.4)
    Conditional {
        condition: Value,   // Predicate function
        transform: Value,   // Transform function
        fallback: Option<Value>,  // Optional fallback function
    },

    // Ordering behavior (Sub-phase 7.5)
    // Maintains sorted order using comparison function
    Ordering {
        compare_fn: Option<Value>,  // Optional comparison function
                                     // None = use default ordering
    },
}

// NOTE: Freeze behaviors (:no_frozen, :copy_elements, :shallow_freeze_only)
// are DEFERRED to Phase 11 when .freeze() is implemented

impl BehaviorSpec {
    /// Convert specification to actual Behavior instance
    pub fn instantiate(&self) -> Box<dyn Behavior> {
        // Implementation in Sub-phase 7.2
    }

    /// Get the name of this behavior
    pub fn name(&self) -> &str {
        // Returns behavior name for introspection
    }

    /// Create from symbol (for Graphoid syntax)
    pub fn from_symbol(sym: &str) -> Option<BehaviorSpec> {
        match sym {
            "none_to_zero" => Some(BehaviorSpec::NoneToZero),
            "none_to_empty" => Some(BehaviorSpec::NoneToEmpty),
            "positive" => Some(BehaviorSpec::Positive),
            "round_to_int" => Some(BehaviorSpec::RoundToInt),
            "uppercase" => Some(BehaviorSpec::Uppercase),
            "lowercase" => Some(BehaviorSpec::Lowercase),
            _ => None,
        }
    }
}

/// Instance of a behavior with retroactive policy
/// Similar to RuleInstance but for transformations
#[derive(Debug, Clone, PartialEq)]
pub struct BehaviorInstance {
    pub spec: BehaviorSpec,
    /// How to handle existing values when behavior is added
    pub retroactive_policy: RetroactivePolicy,
}

impl BehaviorInstance {
    /// Create new behavior instance with default retroactive policy (Clean)
    pub fn new(spec: BehaviorSpec) -> Self {
        BehaviorInstance {
            spec,
            retroactive_policy: RetroactivePolicy::Clean,
        }
    }

    /// Create behavior instance with specific retroactive policy
    pub fn with_policy(spec: BehaviorSpec, policy: RetroactivePolicy) -> Self {
        BehaviorInstance {
            spec,
            retroactive_policy: policy,
        }
    }
}

// Re-use RetroactivePolicy from rules module
use crate::graph::rules::RetroactivePolicy;
```

#### 2. Add Behavior Storage to Collections

**Modify `src/values/list.rs`**:
```rust
use crate::graph::behaviors::BehaviorInstance;

pub struct List {
    pub graph: Graph,
    // Existing fields...

    /// Behaviors attached to this list
    /// Applied in order: first added = first applied
    behaviors: Vec<BehaviorInstance>,
}

impl List {
    pub fn new() -> Self {
        List {
            graph: Graph::new(GraphType::Directed),
            behaviors: Vec::new(),
        }
    }
}
```

**Modify `src/values/hash.rs`** (similar structure):
```rust
pub struct Hash {
    pub graph: Graph,
    behaviors: Vec<BehaviorInstance>,
}
```

**Modify `src/values/graph.rs`** (similar structure):
```rust
pub struct Graph {
    // Existing fields...

    /// Behaviors attached to this graph
    behaviors: Vec<BehaviorInstance>,
}
```

#### 3. Behavior Application Logic

**Add to `src/graph/behaviors.rs`**:
```rust
/// Apply a sequence of behaviors to a value
pub fn apply_behaviors(
    value: Value,
    behaviors: &[BehaviorInstance]
) -> Result<Value, GraphoidError> {
    let mut current = value;

    for behavior_instance in behaviors {
        let behavior = behavior_instance.spec.instantiate();

        // Only apply if behavior applies to this value type
        if behavior.applies_to(&current) {
            current = behavior.transform(&current)?;
        }
    }

    Ok(current)
}

/// Apply behaviors retroactively to all existing values in a collection
/// Used when a new behavior is added
/// Respects the RetroactivePolicy setting
pub fn apply_retroactive_to_list(
    list: &mut List,
    new_behavior: &BehaviorInstance
) -> Result<(), GraphoidError> {
    use crate::graph::rules::RetroactivePolicy;

    let behavior = new_behavior.spec.instantiate();
    let elements = list.elements();

    match new_behavior.retroactive_policy {
        RetroactivePolicy::Clean => {
            // Transform all existing values that apply
            for (index, element) in elements.iter().enumerate() {
                if behavior.applies_to(element) {
                    let transformed = behavior.transform(element)?;
                    let node_id = format!("node_{}", index);
                    if let Some(node) = list.graph.nodes.get_mut(&node_id) {
                        node.value = transformed;
                    }
                }
            }
        }
        RetroactivePolicy::Warn => {
            // Keep existing data, warn about values that would be transformed
            let mut warned = false;
            for (index, element) in elements.iter().enumerate() {
                if behavior.applies_to(element) {
                    eprintln!("WARNING: Behavior '{}' would transform element at index {} from {:?}",
                              behavior.name(), index, element);
                    warned = true;
                }
            }
            if warned {
                eprintln!("WARNING: Existing values NOT transformed. Use RetroactivePolicy::Clean to transform.");
            }
        }
        RetroactivePolicy::Enforce => {
            // Error if any values would be transformed
            for (index, element) in elements.iter().enumerate() {
                if behavior.applies_to(element) {
                    return Err(GraphoidError::runtime(format!(
                        "Behavior '{}' would transform existing element at index {} from {:?}. \
                         Cannot add behavior with RetroactivePolicy::Enforce.",
                        behavior.name(), index, element
                    )));
                }
            }
        }
        RetroactivePolicy::Ignore => {
            // Don't check or transform existing values
            // Only new values will be transformed
        }
    }

    Ok(())
}
```

### Test Strategy (TDD)

#### Test File: `tests/unit/behavior_framework_tests.rs`

**Tests to Write FIRST** (Red Phase):

1. `test_behavior_spec_from_symbol()` - Parse :none_to_zero symbol
2. `test_behavior_spec_name()` - Get name from spec
3. `test_behavior_instance_creation()` - Create BehaviorInstance with default policy
4. `test_behavior_instance_with_policy()` - Create with specific RetroactivePolicy
5. `test_apply_behaviors_empty_list()` - No behaviors = no change
6. `test_apply_behaviors_sequence()` - Multiple behaviors applied in order
7. `test_apply_behaviors_skip_non_applicable()` - Skip if doesn't apply
8. `test_list_has_behaviors_field()` - List stores behaviors
9. `test_hash_has_behaviors_field()` - Hash stores behaviors
10. `test_graph_has_behaviors_field()` - Graph stores behaviors
11. `test_behavior_application_order_matters()` - First added = first applied
12. `test_retroactive_policy_clean()` - Clean transforms existing values
13. `test_retroactive_policy_warn()` - Warn keeps existing values
14. `test_retroactive_policy_enforce()` - Enforce errors on conflicts
15. `test_retroactive_policy_ignore()` - Ignore skips existing values
16. `test_proactive_application_to_new_values()` - New values transformed (placeholder)
17. `test_behavior_transform_returns_value()` - Transform returns new value
18. `test_behavior_applies_to_filters_types()` - applies_to() works

**Expected**: All 18 tests fail (no implementation yet)

### Implementation Steps (Green Phase)

1. Create `src/graph/behaviors.rs` with trait and spec definitions
2. Add `behaviors` field to List, Hash, Graph
3. Implement `apply_behaviors()` helper
4. Implement `apply_retroactive_to_list()` helper
5. Add `pub mod behaviors;` to `src/graph/mod.rs`
6. Run tests - should pass
7. Zero warnings

### Acceptance Criteria

- ✅ 18 tests passing
- ✅ Behavior trait defined and documented
- ✅ BehaviorSpec enum with all behavior types listed (6 standard + mapping + custom + conditional + ordering)
- ✅ BehaviorInstance wrapper with RetroactivePolicy support
- ✅ All collections (List, Hash, Graph) have `behaviors: Vec<BehaviorInstance>` field
- ✅ apply_behaviors() helper works correctly
- ✅ Retroactive application with all 4 policies (Clean, Warn, Enforce, Ignore) working
- ✅ Zero compiler warnings
- ✅ All code documented with rustdoc comments

---

## Sub-Phase 7.2: Standard Behaviors (Days 3-5)

### Goal

Implement all 7 standard behaviors as concrete Behavior trait implementations.

**Note**: Freeze behaviors (`:no_frozen`, `:copy_elements`, `:shallow_freeze_only`) are deferred to Phase 11.

### Standard Behaviors to Implement

**From spec lines 722-740**:

1. **NoneToZero** - Convert none → 0
2. **NoneToEmpty** - Convert none → ""
3. **Positive** - Make negative numbers positive (abs)
4. **RoundToInt** - Round decimals to integers
5. **Uppercase** - Convert strings to UPPERCASE
6. **Lowercase** - Convert strings to lowercase
7. **ValidateRange(min, max)** - Clamp numbers to range

### Deliverables

#### 1. Implement Concrete Behavior Structs

**Add to `src/graph/behaviors.rs`**:

```rust
// ============================================================================
// Standard Behavior Implementations
// ============================================================================

/// Convert none values to 0
#[derive(Debug)]
struct NoneToZeroBehavior;

impl Behavior for NoneToZeroBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        match value {
            Value::None => Ok(Value::Number(0.0)),
            other => Ok(other.clone()),
        }
    }

    fn name(&self) -> &str {
        "none_to_zero"
    }

    fn applies_to(&self, value: &Value) -> bool {
        matches!(value, Value::None)
    }
}

/// Convert none values to empty string
#[derive(Debug)]
struct NoneToEmptyBehavior;

impl Behavior for NoneToEmptyBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        match value {
            Value::None => Ok(Value::String("".to_string())),
            other => Ok(other.clone()),
        }
    }

    fn name(&self) -> &str {
        "none_to_empty"
    }

    fn applies_to(&self, value: &Value) -> bool {
        matches!(value, Value::None)
    }
}

/// Make negative numbers positive (absolute value)
#[derive(Debug)]
struct PositiveBehavior;

impl Behavior for PositiveBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        match value {
            Value::Number(n) if *n < 0.0 => Ok(Value::Number(n.abs())),
            other => Ok(other.clone()),
        }
    }

    fn name(&self) -> &str {
        "positive"
    }

    fn applies_to(&self, value: &Value) -> bool {
        if let Value::Number(n) = value {
            *n < 0.0
        } else {
            false
        }
    }
}

/// Round decimal numbers to integers
#[derive(Debug)]
struct RoundToIntBehavior;

impl Behavior for RoundToIntBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        match value {
            Value::Number(n) => Ok(Value::Number(n.round())),
            other => Ok(other.clone()),
        }
    }

    fn name(&self) -> &str {
        "round_to_int"
    }

    fn applies_to(&self, value: &Value) -> bool {
        matches!(value, Value::Number(_))
    }
}

/// Convert strings to UPPERCASE
#[derive(Debug)]
struct UppercaseBehavior;

impl Behavior for UppercaseBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        match value {
            Value::String(s) => Ok(Value::String(s.to_uppercase())),
            other => Ok(other.clone()),
        }
    }

    fn name(&self) -> &str {
        "uppercase"
    }

    fn applies_to(&self, value: &Value) -> bool {
        matches!(value, Value::String(_))
    }
}

/// Convert strings to lowercase
#[derive(Debug)]
struct LowercaseBehavior;

impl Behavior for LowercaseBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        match value {
            Value::String(s) => Ok(Value::String(s.to_lowercase())),
            other => Ok(other.clone()),
        }
    }

    fn name(&self) -> &str {
        "lowercase"
    }

    fn applies_to(&self, value: &Value) -> bool {
        matches!(value, Value::String(_))
    }
}

/// Clamp numbers to specified range
#[derive(Debug)]
struct ValidateRangeBehavior {
    min: f64,
    max: f64,
}

impl Behavior for ValidateRangeBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        match value {
            Value::Number(n) => {
                let clamped = n.max(self.min).min(self.max);
                Ok(Value::Number(clamped))
            }
            other => Ok(other.clone()),
        }
    }

    fn name(&self) -> &str {
        "validate_range"
    }

    fn applies_to(&self, value: &Value) -> bool {
        matches!(value, Value::Number(_))
    }
}
```

#### 2. Implement BehaviorSpec::instantiate()

```rust
impl BehaviorSpec {
    pub fn instantiate(&self) -> Box<dyn Behavior> {
        match self {
            BehaviorSpec::NoneToZero => Box::new(NoneToZeroBehavior),
            BehaviorSpec::NoneToEmpty => Box::new(NoneToEmptyBehavior),
            BehaviorSpec::Positive => Box::new(PositiveBehavior),
            BehaviorSpec::RoundToInt => Box::new(RoundToIntBehavior),
            BehaviorSpec::Uppercase => Box::new(UppercaseBehavior),
            BehaviorSpec::Lowercase => Box::new(LowercaseBehavior),
            BehaviorSpec::ValidateRange { min, max } => {
                Box::new(ValidateRangeBehavior { min: *min, max: *max })
            }
            _ => unimplemented!("Behavior not yet implemented"),
        }
    }
}
```

#### 3. Expose Behaviors to Executor via add_rule()

**Note**: Behaviors and validation rules share the `add_rule()` API. The executor distinguishes them by checking if the symbol is a known behavior.

**Add to `src/execution/executor.rs`** in `eval_list_method()`:

```rust
"add_rule" => {
    // Check if this is a behavior or validation rule
    if args.len() < 1 {
        return Err(GraphoidError::runtime("add_rule() expects at least 1 argument"));
    }

    // Try to parse as behavior first
    if let Value::Symbol(sym) = &args[0] {
        if let Some(behavior_spec) = BehaviorSpec::from_symbol(sym) {
            // It's a behavior - apply it
            let behavior_instance = BehaviorInstance::new(behavior_spec);

            // Apply retroactively to existing values
            apply_retroactive_to_list(&mut list, &behavior_instance)?;

            // Add to list's behaviors
            list.behaviors.push(behavior_instance);

            // Update in environment
            if let Expr::Variable { name, .. } = object_expr {
                self.env.set(name, Value::List(list))?;
            }

            return Ok(Value::None);
        }
    }

    // Not a behavior - delegate to structural rule handling (from Phase 6)
    // This would be list.graph.add_rule() for structural rules
    return Err(GraphoidError::runtime(format!(
        "Unknown rule or behavior: {:?}",
        args[0]
    )));
}
```

**Similar implementation** for Hash and Graph.

### Test Strategy (TDD)

#### Test File: `tests/behavior_standard_tests.rs`

**Integration Tests** (write FIRST):

**NoneToZero** (3 tests):
1. `test_none_to_zero_converts_none()` - none → 0
2. `test_none_to_zero_leaves_numbers()` - 5 → 5
3. `test_none_to_zero_retroactive()` - Existing nones converted

**NoneToEmpty** (3 tests):
4. `test_none_to_empty_converts_none()` - none → ""
5. `test_none_to_empty_leaves_strings()` - "hello" → "hello"
6. `test_none_to_empty_retroactive()` - Existing nones converted

**Positive** (3 tests):
7. `test_positive_converts_negative()` - -5 → 5
8. `test_positive_leaves_positive()` - 5 → 5
9. `test_positive_retroactive()` - Existing negatives converted

**RoundToInt** (3 tests):
10. `test_round_to_int_rounds()` - 3.7 → 4, 3.2 → 3
11. `test_round_to_int_leaves_integers()` - 5 → 5
12. `test_round_to_int_retroactive()` - Existing decimals rounded

**Uppercase** (2 tests):
13. `test_uppercase_converts()` - "hello" → "HELLO"
14. `test_uppercase_retroactive()` - Existing strings converted

**Lowercase** (2 tests):
15. `test_lowercase_converts()` - "HELLO" → "hello"
16. `test_lowercase_retroactive()` - Existing strings converted

**ValidateRange** (4 tests):
17. `test_validate_range_clamps_high()` - 110 → 100 (range 0-100)
18. `test_validate_range_clamps_low()` - -10 → 0 (range 0-100)
19. `test_validate_range_leaves_in_range()` - 50 → 50
20. `test_validate_range_retroactive()` - Existing values clamped

**Total**: 20 integration tests

### Implementation Steps

1. Write all 20 tests FIRST (red phase)
2. Implement all 7 behavior structs
3. Implement BehaviorSpec::instantiate()
4. Expose via add_rule() in executor for List
5. Expose via add_rule() in executor for Hash
6. Expose via add_rule() in executor for Graph
7. Run tests - should pass (green phase)
8. Refactor for clarity
9. Zero warnings

### Acceptance Criteria

- ✅ 20 tests passing
- ✅ All 7 standard behaviors implemented
- ✅ add_rule(:symbol) works for lists, hashes, graphs
- ✅ Retroactive application works (existing values transformed)
- ✅ Proactive application works (new values transformed)
- ✅ Behaviors apply in correct order
- ✅ Zero compiler warnings

---

## Sub-Phase 7.3: Mapping Behaviors (Days 6-7)

### Goal

Implement hash-based value mapping behaviors with default fallback for unmapped keys.

**Spec Reference**: Lines 742-762

### Concept

```graphoid
# Define custom mapping
status_map = { "active": 1, "inactive": 0, "pending": 2 }
user_statuses = ["active", "unknown", "inactive"]

# Apply with default for unmapped keys
user_statuses.add_mapping_rule(status_map, -1)  # -1 for unmapped
# Result: [1, -1, 0]
```

### Deliverables

#### 1. Mapping Behavior Implementation

**Add to `src/graph/behaviors.rs`**:

```rust
/// Map values using a hash table with default fallback
#[derive(Debug)]
struct MappingBehavior {
    /// Map from string representation to target value
    mapping: HashMap<String, Value>,
    /// Default value for unmapped keys
    default: Value,
}

impl Behavior for MappingBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        // Convert value to string key
        let key = value.to_string();

        // Look up in mapping
        if let Some(mapped) = self.mapping.get(&key) {
            Ok(mapped.clone())
        } else {
            Ok(self.default.clone())
        }
    }

    fn name(&self) -> &str {
        "mapping"
    }

    fn applies_to(&self, _value: &Value) -> bool {
        // Applies to all values
        true
    }
}
```

**Update BehaviorSpec**:
```rust
BehaviorSpec::Mapping {
    mapping: HashMap<String, Value>,
    default: Value,
}
```

#### 2. Expose to Executor

**Add to `src/execution/executor.rs`**:

```rust
"add_mapping_rule" => {
    // Expects 2 arguments: mapping hash, default value
    if args.len() != 2 {
        return Err(GraphoidError::runtime(format!(
            "add_mapping_rule() expects 2 arguments, got {}",
            args.len()
        )));
    }

    // Extract mapping hash
    let mapping_hash = match &args[0] {
        Value::Map(h) => h.clone(),
        other => {
            return Err(GraphoidError::type_error("hash", other.type_name()));
        }
    };

    // Convert Hash to HashMap<String, Value>
    let mut mapping = HashMap::new();
    for (key, value) in mapping_hash.entries() {
        mapping.insert(key.clone(), value.clone());
    }

    let default_value = args[1].clone();

    let behavior_spec = BehaviorSpec::Mapping {
        mapping,
        default: default_value,
    };

    let behavior_instance = BehaviorInstance::new(behavior_spec);

    // Apply retroactively
    apply_retroactive_to_list(&mut list, &behavior_instance)?;

    // Add to behaviors
    list.behaviors.push(behavior_instance);

    // Update environment
    if let Expr::Variable { name, .. } = object_expr {
        self.env.set(name, Value::List(list))?;
    }

    Ok(Value::None)
}
```

### Test Strategy (TDD)

#### Test File: `tests/behavior_mapping_tests.rs`

**Tests to Write FIRST**:

1. `test_mapping_basic()` - Simple string → number mapping
2. `test_mapping_with_default()` - Unmapped key uses default
3. `test_mapping_all_mapped()` - All values in mapping
4. `test_mapping_all_unmapped()` - All values use default
5. `test_mapping_mixed()` - Some mapped, some default
6. `test_mapping_retroactive()` - Existing values transformed
7. `test_mapping_chained()` - Chain two mappings (spec line 756)
8. `test_mapping_number_to_string()` - Map numbers to strings
9. `test_mapping_boolean_values()` - Map to boolean values
10. `test_mapping_empty_hash()` - Empty mapping uses all defaults

**Total**: 10 tests

### Implementation Steps

1. Write all 10 tests FIRST
2. Implement MappingBehavior struct
3. Add BehaviorSpec::Mapping variant
4. Update BehaviorSpec::instantiate()
5. Add add_mapping_rule() to executor (within add_rule() method)
6. Run tests - should pass
7. Zero warnings

### Acceptance Criteria

- ✅ 10 tests passing
- ✅ Mapping behavior works with hash tables
- ✅ Default fallback works for unmapped keys
- ✅ Chaining multiple mappings works
- ✅ Retroactive application works
- ✅ Zero compiler warnings

---

## Sub-Phase 7.4: Custom & Conditional Behaviors (Days 7-9)

### Goal

Implement user-defined function behaviors and conditional behaviors with predicates.

**Spec Reference**: Lines 764-816

### Custom Function Behaviors

**Concept** (spec lines 766-780):
```graphoid
fn normalize_temp(value) {
    if value < 95 { return 95 }
    if value > 105 { return 105 }
    return value
}

temperatures.add_custom_rule(normalize_temp)
temperatures.append(110)  # Becomes 105
```

### Conditional Behaviors

**Concept** (spec lines 782-816):
```graphoid
fn is_string(value) {
    return value.type() == "string"
}

fn to_upper(value) {
    return value.upper()
}

mixed_data.add_conditional_rule(is_string, to_upper)
# [42, "hello", -10, "world"] → [42, "HELLO", -10, "WORLD"]
```

### Deliverables

#### 1. Custom Function Behavior

**Add to `src/graph/behaviors.rs`**:

```rust
/// Custom user-defined transformation function
#[derive(Debug, Clone)]
struct CustomFunctionBehavior {
    function: Function,  // Stored function
}

impl Behavior for CustomFunctionBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        // Call the function with value as argument
        // This requires access to Executor - we'll store a closure instead
        // OR: We store the function and executor calls it
        // Design decision: Pass executor context through transform()

        // For now, store Function and rely on executor integration
        unimplemented!("Requires executor integration")
    }

    fn name(&self) -> &str {
        "custom_function"
    }
}
```

**Design Challenge**: Behaviors need to call user functions, which requires executor context.

**Solution**: Transform behaviors at the executor level, not in the Behavior trait.

**Revised Architecture**:

```rust
// In executor.rs
fn apply_behaviors_with_context(
    &mut self,
    value: Value,
    behaviors: &[BehaviorInstance]
) -> Result<Value, GraphoidError> {
    let mut current = value;

    for behavior_instance in behaviors {
        match &behavior_instance.spec {
            BehaviorSpec::CustomFunction { function } => {
                // Call function with executor context
                current = self.call_function(function, &[current])?;
            }
            BehaviorSpec::Conditional { condition, transform, fallback } => {
                // Call condition predicate
                let matches = self.call_function(condition, &[current.clone()])?;

                if matches.is_truthy() {
                    // Apply transform
                    current = self.call_function(transform, &[current])?;
                } else if let Some(fb) = fallback {
                    // Apply fallback
                    current = self.call_function(fb, &[current])?;
                }
                // else: leave unchanged
            }
            _ => {
                // Standard behaviors use trait
                let behavior = behavior_instance.spec.instantiate();
                if behavior.applies_to(&current) {
                    current = behavior.transform(&current)?;
                }
            }
        }
    }

    Ok(current)
}
```

#### 2. Expose to Executor

**Add to executor.rs**:

```rust
"add_custom_rule" => {
    if args.len() != 1 {
        return Err(GraphoidError::runtime(
            "add_custom_rule() expects 1 function argument"
        ));
    }

    let function = match &args[0] {
        Value::Function(f) => f.clone(),
        other => {
            return Err(GraphoidError::type_error("function", other.type_name()));
        }
    };

    let behavior_spec = BehaviorSpec::CustomFunction {
        function: Value::Function(function),
    };

    let behavior_instance = BehaviorInstance::new(behavior_spec);

    // Apply retroactively (requires executor context)
    self.apply_behaviors_retroactively_to_list(&mut list, &behavior_instance)?;

    list.behaviors.push(behavior_instance);

    // Update environment
    if let Expr::Variable { name, .. } = object_expr {
        self.env.set(name, Value::List(list))?;
    }

    Ok(Value::None)
}

"add_conditional_rule" => {
    // Expects 2-3 arguments: condition, transform, [fallback]
    if args.len() < 2 || args.len() > 3 {
        return Err(GraphoidError::runtime(
            "add_conditional_rule() expects 2-3 arguments"
        ));
    }

    let condition = match &args[0] {
        Value::Function(f) => Value::Function(f.clone()),
        other => {
            return Err(GraphoidError::type_error("function", other.type_name()));
        }
    };

    let transform = match &args[1] {
        Value::Function(f) => Value::Function(f.clone()),
        other => {
            return Err(GraphoidError::type_error("function", other.type_name()));
        }
    };

    let fallback = if args.len() == 3 {
        match &args[2] {
            Value::Function(f) => Some(Value::Function(f.clone())),
            other => {
                return Err(GraphoidError::type_error("function", other.type_name()));
            }
        }
    } else {
        None
    };

    let behavior_spec = BehaviorSpec::Conditional {
        condition,
        transform,
        fallback,
    };

    let behavior_instance = BehaviorInstance::new(behavior_spec);

    // Apply retroactively
    self.apply_behaviors_retroactively_to_list(&mut list, &behavior_instance)?;

    list.behaviors.push(behavior_instance);

    // Update environment
    if let Expr::Variable { name, .. } = object_expr {
        self.env.set(name, Value::List(list))?;
    }

    Ok(Value::None)
}
```

### Test Strategy (TDD)

#### Test File: `tests/behavior_custom_tests.rs`

**Custom Function Behaviors** (8 tests):
1. `test_custom_function_basic()` - Simple transformation
2. `test_custom_function_clamp()` - Clamp values (spec example)
3. `test_custom_function_retroactive()` - Existing values transformed
4. `test_custom_function_multiple_behaviors()` - Chain custom + standard
5. `test_custom_function_with_closure()` - Function captures environment
6. `test_custom_function_error_handling()` - Handle function errors
7. `test_custom_function_type_specific()` - Only transform certain types
8. `test_custom_function_identity()` - No-op function

**Conditional Behaviors** (7 tests):
9. `test_conditional_basic()` - is_string → uppercase (spec example)
10. `test_conditional_with_fallback()` - Fallback for non-matching
11. `test_conditional_all_match()` - All values match condition
12. `test_conditional_none_match()` - No values match
13. `test_conditional_mixed()` - Some match, some don't
14. `test_conditional_retroactive()` - Existing values transformed
15. `test_conditional_chained()` - Multiple conditional behaviors

**Total**: 15 tests

### Implementation Steps

1. Write all 15 tests FIRST
2. Update BehaviorSpec with CustomFunction and Conditional variants
3. Implement apply_behaviors_with_context() in executor
4. Implement apply_behaviors_retroactively_to_list() in executor
5. Add add_custom_rule() handling in add_rule() method
6. Add add_conditional_rule() handling in add_rule() method
7. Modify List::append(), Hash::set(), Graph::add_node() to apply behaviors
8. Run tests - should pass
9. Zero warnings

### Acceptance Criteria

- ✅ 15 tests passing
- ✅ Custom function behaviors work
- ✅ Conditional behaviors work with predicates
- ✅ Fallback functions work
- ✅ Retroactive application works
- ✅ Behaviors integrate with executor properly
- ✅ Zero compiler warnings

---

## Sub-Phase 7.5: Ordering Behaviors (Days 8-9)

**See separate document**: `/home/irv/work/grang/dev_docs/PHASE_7_5_ORDERING_BEHAVIORS.md`

This sub-phase implements ordering behaviors that maintain sorted order for collections (lists, trees, graphs). Ordering is a collection-level behavior (not value-level) that works with default or custom comparison functions.

**Deliverables**: 12 tests, binary search insertion, stable sort for retroactive application

---

## Sub-Phase 7.6: Behavior Management (Day 10)

### Goal

Implement introspection and management APIs for behaviors.

**Spec Reference**: Lines 836-858

### APIs to Implement

**Note**: Rules and behaviors share the same management API. The implementation checks both structural rules and behaviors.

```graphoid
# Check if rule/behavior exists
has_positive = list.has_rule(:positive)  # Works for both behaviors and structural rules

# Get all active rules/behaviors (sorted alphabetically)
rules = list.rules()  # Returns list of all rule names (structural + behavior)

# Remove specific rule/behavior
list.remove_rule(:positive)  # Removes from behaviors or structural rules

# Clear all rules/behaviors
list.clear_rules()  # Clears both behaviors and structural rules

# Add multiple rules/behaviors (rulesets)
data_cleaning = [:none_to_zero, :positive, :round_to_int]
temperatures.add_rules(data_cleaning)  # Note: add_rules (plural)
```

### Deliverables

#### 1. Unified Rule/Behavior Management Methods

**Add to `src/values/list.rs`**:

```rust
impl List {
    /// Check if a specific rule or behavior is active
    pub fn has_rule(&self, rule_name: &str) -> bool {
        // Check behaviors first
        if self.behaviors.iter().any(|b| b.spec.name() == rule_name) {
            return true;
        }
        // Then check structural rules in the underlying graph
        self.graph.has_rule(rule_name)
    }

    /// Get list of all active rule names (behaviors + structural rules, sorted)
    pub fn rule_names(&self) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();

        // Add behavior names
        names.extend(self.behaviors.iter().map(|b| b.spec.name().to_string()));

        // Add structural rule names from graph
        names.extend(self.graph.get_active_rule_specs().iter().map(|spec| spec.name().to_string()));

        names.sort();
        names.dedup();  // Remove duplicates if any
        names
    }

    /// Remove a specific rule or behavior by name
    pub fn remove_rule(&mut self, rule_name: &str) -> bool {
        // Try to remove from behaviors first
        let orig_behavior_len = self.behaviors.len();
        self.behaviors.retain(|b| b.spec.name() != rule_name);

        if self.behaviors.len() < orig_behavior_len {
            return true;  // Removed from behaviors
        }

        // Otherwise try to remove from structural rules
        self.graph.remove_rule(rule_name)
    }

    /// Clear all rules and behaviors
    pub fn clear_rules(&mut self) {
        self.behaviors.clear();
        self.graph.clear_rules();
    }
}
```

**Similar implementations** for Hash and Graph.

#### 2. Expose to Executor

**Add to executor.rs**:

```rust
"has_rule" => {
    // Works for both structural rules and behaviors
    if args.len() != 1 {
        return Err(GraphoidError::runtime(
            "has_rule() expects 1 symbol argument"
        ));
    }

    let rule_name = match &args[0] {
        Value::Symbol(s) => s.as_str(),
        other => {
            return Err(GraphoidError::type_error("symbol", other.type_name()));
        }
    };

    Ok(Value::Boolean(list.has_rule(rule_name)))
}

"rules" => {
    // Returns all rules (structural + behaviors)
    if !args.is_empty() {
        return Err(GraphoidError::runtime(
            "rules() expects no arguments"
        ));
    }

    let names = list.rule_names();
    let name_values: Vec<Value> = names.into_iter()
        .map(Value::String)
        .collect();

    Ok(Value::List(List::from_vec(name_values)))
}

"remove_rule" => {
    // Removes from either behaviors or structural rules
    if args.len() != 1 {
        return Err(GraphoidError::runtime(
            "remove_rule() expects 1 symbol argument"
        ));
    }

    let rule_name = match &args[0] {
        Value::Symbol(s) => s.as_str(),
        other => {
            return Err(GraphoidError::type_error("symbol", other.type_name()));
        }
    };

    let removed = list.remove_rule(rule_name);

    // Update environment
    if let Expr::Variable { name, .. } = object_expr {
        self.env.set(name, Value::List(list))?;
    }

    Ok(Value::Boolean(removed))
}

"clear_rules" => {
    // Clears both behaviors and structural rules
    if !args.is_empty() {
        return Err(GraphoidError::runtime(
            "clear_rules() expects no arguments"
        ));
    }

    list.clear_rules();

    // Update environment
    if let Expr::Variable { name, .. } = object_expr {
        self.env.set(name, Value::List(list))?;
    }

    Ok(Value::None)
}

"add_rules" => {
    // Expects a list of symbols (can be behaviors or structural rules)
    if args.len() != 1 {
        return Err(GraphoidError::runtime(
            "add_rules() expects 1 list argument"
        ));
    }

    let rule_list = match &args[0] {
        Value::List(l) => l,
        other => {
            return Err(GraphoidError::type_error("list", other.type_name()));
        }
    };

    // Parse each symbol - try as behavior first, then structural rule
    let elements = rule_list.elements();
    for elem in elements {
        let symbol = match elem {
            Value::Symbol(s) => s,
            other => {
                return Err(GraphoidError::type_error("symbol", other.type_name()));
            }
        };

        // Try to parse as behavior first
        if let Some(behavior_spec) = BehaviorSpec::from_symbol(&symbol) {
            let behavior_instance = BehaviorInstance::new(behavior_spec);

            // Apply retroactively
            self.apply_behaviors_retroactively_to_list(&mut list, &behavior_instance)?;

            // Add to list's behaviors
            list.behaviors.push(behavior_instance);
        } else {
            // Try as structural rule
            // Delegate to graph's add_rule if it's not a behavior
            return Err(GraphoidError::runtime(format!(
                "Unknown rule or behavior: :{}",
                symbol
            )));
        }
    }

    // Update environment
    if let Expr::Variable { name, .. } = object_expr {
        self.env.set(name, Value::List(list))?;
    }

    Ok(Value::None)
}
```

### Test Strategy (TDD)

#### Test File: `tests/behavior_management_tests.rs`

**Tests to Write FIRST**:

1. `test_has_rule_for_behavior_true()` - Behavior exists
2. `test_has_rule_for_behavior_false()` - Behavior doesn't exist
3. `test_rules_returns_sorted_list()` - Get all rule/behavior names
4. `test_rules_empty()` - No rules/behaviors returns empty list
5. `test_remove_rule_for_behavior_success()` - Remove existing behavior
6. `test_remove_rule_for_behavior_not_found()` - Remove non-existent behavior
7. `test_clear_rules_clears_behaviors()` - Clear all rules/behaviors
8. `test_add_rules_ruleset()` - Add multiple rules/behaviors at once

**Total**: 8 tests

### Implementation Steps

1. Write all 8 tests FIRST
2. Implement unified management methods on List, Hash, Graph (has_rule, rules, remove_rule, clear_rules)
3. Add executor methods (has_rule, rules, remove_rule, clear_rules, add_rules)
4. Run tests - should pass
5. Zero warnings

### Acceptance Criteria

- ✅ 8 tests passing
- ✅ has_rule() works for behaviors and structural rules
- ✅ rules() returns sorted list of all rule/behavior names
- ✅ remove_rule() works for both behaviors and structural rules
- ✅ clear_rules() clears both behaviors and structural rules
- ✅ add_rules() works for rulesets (behaviors + structural rules)
- ✅ Zero compiler warnings

---

## Sub-Phase 7.7: Integration & Quality Gate (Day 11)

### Goal

Ensure behaviors integrate properly with all operations and pass comprehensive quality checks.

### Integration Tasks

#### 1. Behavior Application in Operations

**Modify List::append()**:
```rust
impl List {
    pub fn append(&mut self, value: Value) -> Result<(), GraphoidError> {
        // Apply behaviors to incoming value
        let transformed = if !self.behaviors.is_empty() {
            // Need executor context for custom/conditional behaviors
            // For now, apply only standard behaviors
            apply_behaviors(value, &self.behaviors)?
        } else {
            value
        };

        // Existing append logic with transformed value
        // ...
    }
}
```

**Note**: This reveals a design challenge - we need executor context for custom behaviors.

**Solution**: Behaviors applied at executor level in `eval_method_call()`:

```rust
// In executor.rs eval_method_call()
match method {
    "append" => {
        // Apply behaviors BEFORE calling append
        let transformed_args: Vec<Value> = args.iter()
            .map(|arg| self.apply_behaviors_with_context(arg.clone(), &list.behaviors))
            .collect::<Result<Vec<_>, _>>()?;

        // Call append with transformed value
        list.append(transformed_args[0].clone())?;

        // Update environment
        // ...
    }
    // Similar for insert, prepend, etc.
}
```

#### 2. Comprehensive Integration Tests

**Test File**: `tests/behavior_integration_tests.rs`

1. `test_behaviors_with_append()` - Behavior applied on append
2. `test_behaviors_with_insert()` - Behavior applied on insert
3. `test_behaviors_with_prepend()` - Behavior applied on prepend
4. `test_behaviors_with_hash_set()` - Behavior applied on hash["key"] = value
5. `test_behaviors_with_graph_add_node()` - Behavior applied on add_node
6. `test_multiple_behaviors_order()` - Order matters test
7. `test_behaviors_with_type_constraints()` - Behaviors + list<num>
8. `test_behaviors_persist_across_operations()` - Behaviors survive modifications
9. `test_remove_rule_stops_transformation()` - Removed behavior doesn't apply (via remove_rule)
10. `test_standard_and_custom_mixed()` - Mix standard + custom behaviors
11. `test_ordering_behavior_integration()` - Ordering behavior maintains sorted order
12. `test_behaviors_and_structural_rules_coexist()` - Both types of rules work together

### Quality Gate Checklist

#### Code Quality
- ✅ Zero compiler warnings
- ✅ Zero clippy warnings
- ✅ All code documented with rustdoc
- ✅ No unwrap() or expect() in production code
- ✅ Proper error handling throughout

#### Test Coverage
- ✅ All 95+ tests passing
- ✅ Framework tests (18)
- ✅ Standard behavior tests (20)
- ✅ Mapping behavior tests (10)
- ✅ Custom/conditional behavior tests (15)
- ✅ Ordering behavior tests (12)
- ✅ Management tests (8)
- ✅ Integration tests (12)

#### Spec Conformance
- ✅ All standard behaviors implemented
- ✅ Mapping behaviors work
- ✅ Custom function behaviors work
- ✅ Conditional behaviors work
- ✅ Behavior management APIs work
- ✅ Retroactive application works
- ✅ Proactive application works
- ✅ Order matters (first added = first applied)

#### Architecture
- ✅ Behaviors separate from rules
- ✅ Proper integration with executor
- ✅ Collections store behaviors
- ✅ No rule/behavior confusion

#### Documentation
- ✅ All public APIs documented
- ✅ Usage examples in doc comments
- ✅ Behavior system overview documented
- ✅ Difference from rules explained

### Final Verification

```bash
# All tests pass
~/.cargo/bin/cargo test

# No warnings
~/.cargo/bin/cargo build 2>&1 | grep -i warning

# Count tests
~/.cargo/bin/cargo test 2>&1 | grep "test result:"

# Clippy check
~/.cargo/bin/cargo clippy -- -D warnings

# Documentation check
~/.cargo/bin/cargo doc --no-deps
```

### Acceptance Criteria

- ✅ 95+ tests passing (total ~424 with previous phases: 329 from Phase 6.5 + 95 new)
- ✅ Zero compiler warnings
- ✅ Zero clippy warnings
- ✅ All behaviors work correctly
- ✅ Integration with operations complete
- ✅ Full spec conformance for Phase 7 scope
- ✅ Documentation complete
- ✅ Ready for Phase 8

---

## TDD Methodology Summary

### Red-Green-Refactor Cycle

**Every sub-phase follows strict TDD**:

1. **RED Phase**: Write tests FIRST
   - Tests define the API
   - Tests fail (no implementation yet)
   - Tests document expected behavior

2. **GREEN Phase**: Make tests pass
   - Write minimal code to pass tests
   - Don't worry about perfection
   - Focus on correctness

3. **REFACTOR Phase**: Improve code
   - Extract duplicated code
   - Improve names and structure
   - Maintain test passing throughout

### Testing Principles

1. **Test First, Always** - No code without tests
2. **One Test, One Concern** - Each test verifies one thing
3. **Clear Test Names** - `test_behavior_action_expected()`
4. **Arrange-Act-Assert** - Setup, execute, verify
5. **No Mocking Collections** - Use real List, Hash, Graph instances
6. **Integration Over Unit** - Test through Graphoid code when possible

### Test Organization

```
tests/
├── unit/
│   └── behavior_framework_tests.rs      # Sub-phase 7.1 (18 tests)
├── behavior_standard_tests.rs           # Sub-phase 7.2 (20 tests)
├── behavior_mapping_tests.rs            # Sub-phase 7.3 (10 tests)
├── behavior_custom_tests.rs             # Sub-phase 7.4 (15 tests)
├── behavior_ordering_tests.rs           # Sub-phase 7.5 (12 tests)
├── behavior_management_tests.rs         # Sub-phase 7.6 (8 tests)
└── behavior_integration_tests.rs        # Sub-phase 7.7 (12 tests)
```

---

## Risk Assessment & Mitigation

### Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Executor context for custom behaviors | High | High | Design upfront, use apply_behaviors_with_context() |
| Retroactive application performance | Medium | Medium | Benchmark, optimize if needed |
| Behavior order complexity | Low | High | Clear documentation, comprehensive tests |
| Rule/behavior confusion | Medium | Medium | Clear naming, separate storage |

### Schedule Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Custom behaviors take longer | Medium | Medium | Time-box, defer if needed |
| Integration issues | Low | High | Test early, test often |
| Spec ambiguity | Low | Medium | Ask user for clarification |

---

## Success Metrics

### Quantitative

- ✅ 95+ new tests passing
- ✅ Total test count: ~424 (329 current + 95 new)
- ✅ Zero compiler warnings
- ✅ Zero clippy warnings
- ✅ 100% test pass rate maintained

### Qualitative

- ✅ All standard behaviors work as specified
- ✅ Custom function behaviors integrate properly
- ✅ Conditional behaviors work correctly
- ✅ Code is clean, readable, well-documented
- ✅ Architecture is sound and extensible
- ✅ Ready for Phase 8 (Module System)

---

## Deliverables Summary

### Code Files

**New Files**:
1. `src/graph/behaviors.rs` - Behavior trait, specs, implementations (~800 lines)
2. `tests/unit/behavior_framework_tests.rs` - Framework tests (15 tests)
3. `tests/behavior_standard_tests.rs` - Standard behavior tests (20 tests)
4. `tests/behavior_mapping_tests.rs` - Mapping tests (10 tests)
5. `tests/behavior_custom_tests.rs` - Custom/conditional tests (15 tests)
6. `tests/behavior_management_tests.rs` - Management tests (8 tests)
7. `tests/behavior_integration_tests.rs` - Integration tests (10 tests)

**Modified Files**:
1. `src/graph/mod.rs` - Export behaviors module
2. `src/values/list.rs` - Add behaviors field, management methods
3. `src/values/hash.rs` - Add behaviors field, management methods
4. `src/values/graph.rs` - Add behaviors field, management methods
5. `src/execution/executor.rs` - Add behavior application logic, expose APIs

### Documentation

1. **Rustdoc**: All public APIs documented
2. **Module docs**: behaviors.rs has comprehensive overview
3. **Examples**: Doc comments include usage examples
4. **Architecture**: Clear distinction between rules and behaviors

---

## Next Steps After Phase 7

Once Phase 7 is complete:

1. **Phase 8**: Module System (import/export, multi-file projects)
2. **Phase 9**: Native Stdlib Modules (math, time, random, io, regex)
3. **Phase 10**: Pure Graphoid Stdlib (statistics, csv, http)
4. **Phase 11**: Advanced Features (precision contexts, configuration blocks)

---

## User Decisions (Resolved)

All planning questions have been answered. See `/home/irv/work/grang/dev_docs/PHASE_7_REVISIONS_SUMMARY.md` for details.

**Decisions**:
1. ✅ **Unified API**: Use `add_rule()` for both structural rules and behaviors
2. ✅ **Retroactive Policy**: Full support for all 4 policies (Clean, Warn, Enforce, Ignore)
3. ✅ **Freeze Behaviors**: Deferred to Phase 11 (see `PHASE_11_FREEZE_BEHAVIORS_NOTE.md`)
4. ✅ **Ordering Behaviors**: Implemented for ALL collections in Sub-Phase 7.5 (see `PHASE_7_5_ORDERING_BEHAVIORS.md`)

---

**End of Phase 7 Plan** - Ready to begin implementation! 🚀

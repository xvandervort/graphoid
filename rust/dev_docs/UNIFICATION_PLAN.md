# Rule System Unification Plan

**Status**: IN PROGRESS
**Priority**: CRITICAL - Architecture must match specification
**Created**: October 25, 2025

---

## Problem

The current implementation has a split between "rules" (validation) and "behaviors" (transformation), even internally. The language specification is clear: **everything is a rule**. Some rules validate (and reject), some rules transform (and accept).

## Current State (WRONG)

- Validation rules stored in `graph.rules`
- Transformation rules stored separately in `list.behaviors` and `hash.behaviors`
- Separate `RuleSpec` and `BehaviorSpec` enums
- Separate `Rule` and `Behavior` traits
- behaviors.rs file (764 lines)

## Target State (CORRECT)

- ALL rules stored in `graph.rules`
- Single `RuleSpec` enum with both validation and transformation variants
- Single approach to rule application
- No separate "behaviors" concept anywhere

---

## Implementation Steps

### Step 1: Merge RuleSpec and BehaviorSpec ✅ DONE

**File**: `src/graph/rules.rs`

Added all transformation rule variants to RuleSpec:
- `NoneToZero`
- `NoneToEmpty`
- `Positive`
- `RoundToInt`
- `Uppercase`
- `Lowercase`
- `ValidateRange { min, max }`
- `Mapping { mapping, default }`
- `CustomFunction { function }`
- `Conditional { condition, transform, fallback }`
- `Ordering { compare_fn }`

### Step 2: Add from_symbol() to RuleSpec

**File**: `src/graph/rules.rs`

Add method to RuleSpec impl:
```rust
pub fn from_symbol(sym: &str) -> Option<RuleSpec> {
    match sym {
        // Validation rules
        "no_cycles" => Some(RuleSpec::NoCycles),
        "single_root" => Some(RuleSpec::SingleRoot),
        // ... etc

        // Transformation rules
        "none_to_zero" => Some(RuleSpec::NoneToZero),
        "none_to_empty" => Some(RuleSpec::NoneToEmpty),
        "positive" => Some(RuleSpec::Positive),
        "round_to_int" => Some(RuleSpec::RoundToInt),
        "uppercase" => Some(RuleSpec::Uppercase),
        "lowercase" => Some(RuleSpec::Lowercase),
        _ => None,
    }
}
```

### Step 3: Add name() method to RuleSpec

**File**: `src/graph/rules.rs`

```rust
pub fn name(&self) -> &str {
    match self {
        RuleSpec::NoCycles => "no_cycles",
        RuleSpec::SingleRoot => "single_root",
        // ...
        RuleSpec::NoneToZero => "none_to_zero",
        RuleSpec::Uppercase => "uppercase",
        // ... etc
    }
}
```

### Step 4: Extend Rule trait or create unified approach

**Options**:

A) Extend Rule trait with transform capability:
```rust
pub trait Rule {
    fn validate(&self, graph: &Graph) -> Result<(), GraphoidError>;
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        Ok(value.clone())  // Default: no transformation
    }
    fn is_transformation_rule(&self) -> bool { false }
}
```

B) Keep transformation logic in behaviors.rs but have them implement Rule trait

C) Create a RuleApplication enum:
```rust
pub enum RuleApplication {
    Validation(Box<dyn Fn(&Graph) -> Result<(), GraphoidError>>),
    Transformation(Box<dyn Fn(&Value) -> Result<Value, GraphoidError>>),
}
```

**RECOMMENDED**: Option A - extend Rule trait

### Step 5: Update RuleSpec::instantiate()

Add cases for all transformation rules:
```rust
pub fn instantiate(&self) -> Box<dyn Rule> {
    match self {
        // Existing validation rules
        RuleSpec::NoCycles => Box::new(NoCyclesRule::new()),
        // ...

        // New transformation rules
        RuleSpec::NoneToZero => Box::new(NoneToZeroRule::new()),
        RuleSpec::Uppercase => Box::new(UppercaseRule::new()),
        // ... etc
    }
}
```

### Step 6: Move/adapt transformation rule implementations

**Option A**: Keep in behaviors.rs but rename to transformation_rules.rs
**Option B**: Move into rules.rs
**Option C**: Create rules/validation.rs and rules/transformation.rs

**RECOMMENDED**: Keep in behaviors.rs but have all structs implement Rule trait

### Step 7: Remove behaviors field from List

**File**: `src/values/list.rs`

```rust
pub struct List {
    pub graph: Graph,
    length: usize,
    // REMOVE: pub behaviors: Vec<BehaviorInstance>,
}
```

All rules (validation + transformation) go in `graph.rules`.

### Step 8: Remove behaviors field from Hash

**File**: `src/values/hash.rs`

```rust
pub struct Hash {
    graph: Graph,
    // REMOVE: pub behaviors: Vec<BehaviorInstance>,
}
```

### Step 9: Update List methods

**File**: `src/values/list.rs`

Remove:
- `add_behavior()`
- `get_behaviors()`

Everything now uses:
- `add_rule()` - adds to graph.rules
- `get_rules()` - returns graph.rules
- `has_rule()` - checks graph.rules
- `remove_rule()` - removes from graph.rules

### Step 10: Update Hash methods

**File**: `src/values/hash.rs`

Same as List - remove behavior-specific methods.

### Step 11: Update rule application in List/Hash

When appending/inserting values:
1. Get all transformation rules from `graph.rules`
2. Apply them in order to the incoming value
3. Insert the transformed value

Example in List::append():
```rust
pub fn append(&mut self, value: Value) -> Result<(), GraphoidError> {
    // Get transformation rules from graph
    let transformation_rules: Vec<_> = self.graph.get_rules()
        .iter()
        .filter(|r| r.spec.is_transformation_rule())
        .collect();

    // Apply transformations
    let mut transformed = value;
    for rule in transformation_rules {
        let rule_impl = rule.spec.instantiate();
        transformed = rule_impl.transform(&transformed)?;
    }

    // Add to graph
    let new_id = format!("node_{}", self.length);
    self.graph.add_node(new_id.clone(), transformed)?;
    // ... rest of append logic
}
```

### Step 12: Update executor

**File**: `src/execution/executor.rs`

Remove references to `BehaviorSpec::from_symbol()`.

The `.add_rule()` method now just uses `RuleSpec::from_symbol()` for everything:

```rust
"add_rule" => {
    let rule_symbol = match &args[0] {
        Value::Symbol(name) => name.as_str(),
        _ => return Err(...),
    };

    // Try to parse as RuleSpec (handles both validation and transformation)
    let rule_spec = RuleSpec::from_symbol(rule_symbol)
        .or_else(|| Self::symbol_to_rule_spec_with_params(...))
        .ok_or_else(|| GraphoidError::runtime(format!("Unknown rule: {}", rule_symbol)))?;

    let mut new_list = list.clone();
    new_list.add_rule(RuleInstance::new(rule_spec))?;
    Ok(Value::List(new_list))
}
```

### Step 13: Update or remove behaviors.rs

**Options**:
A) Rename to `transformation_rules.rs` and update to implement Rule trait
B) Merge into `rules.rs`
C) Delete and reimplement in `rules.rs`

**RECOMMENDED**: Option A

### Step 14: Update all test imports

Change:
```rust
use graphoid::graph::behaviors::{BehaviorSpec, BehaviorInstance};
```

To:
```rust
use graphoid::graph::rules::{RuleSpec, RuleInstance};
```

### Step 15: Run all tests

```bash
~/.cargo/bin/cargo test
```

Fix any failures.

---

## Files to Modify

1. ✅ `src/graph/rules.rs` - Add transformation rule variants
2. `src/graph/rules.rs` - Add from_symbol(), name(), extend Rule trait
3. `src/graph/behaviors.rs` - Rename or merge, update to implement Rule
4. `src/values/list.rs` - Remove behaviors field, update methods
5. `src/values/hash.rs` - Remove behaviors field, update methods
6. `src/execution/executor.rs` - Update to use unified RuleSpec
7. All test files - Update imports and expectations

---

## Testing Strategy

After each step:
1. Run `cargo build` to check compilation
2. Run `cargo test` to verify existing tests
3. Fix any breakage before proceeding

Final verification:
- All 398 tests passing
- Zero warnings
- No references to "behavior" except in comments explaining history

---

## Completion Criteria

- ✅ Single RuleSpec enum with all rule types
- ✅ No separate BehaviorSpec enum
- ✅ No behaviors field in List or Hash
- ✅ All rules stored in graph.rules
- ✅ Single .add_rule() API
- ✅ All 398 tests passing
- ✅ Zero warnings
- ✅ Clean, unified architecture

---

## Notes

- The LANGUAGE SPECIFICATION uses the term "rules" for everything
- Internally we can organize code (separate files for transformation vs validation)
- But the user-facing API and core data structures must use "rules" terminology only
- No "behaviors" anywhere in the public API or core data structures

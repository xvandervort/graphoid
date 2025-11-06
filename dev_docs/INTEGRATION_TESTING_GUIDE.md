# Graphoid Integration Testing Guide

**Purpose**: Prevent the Rust API vs .gr Language Gap from recurring
**Audience**: All contributors to Graphoid
**Status**: MANDATORY for all feature development

---

## The Problem We're Solving

**What Happened**: Phase 9 was marked "complete" with 186 passing Rust tests, but pattern matching was completely unusable from `.gr` files because executor integration was never implemented.

**Root Cause**: Rust unit tests only verify the internal API. They don't test that features are actually accessible through the executor from user-facing `.gr` files.

**Impact**: False sense of completion, unusable features, wasted effort.

**Solution**: Require integration tests for ALL features. No exceptions.

---

## The Two-Level Testing Requirement

Every feature MUST be tested at BOTH levels:

### Level 1: Rust API Testing (Unit Tests)
**Purpose**: Verify internal implementation correctness
**Location**: `tests/unit/`
**Tests**: Data structures, algorithms, edge cases
**Tool**: `cargo test --lib`

### Level 2: .gr Integration Testing (Executor Tests)
**Purpose**: Verify feature is accessible from user-facing language
**Location**: `tests/integration/`
**Tests**: End-to-end workflows, user scenarios
**Tool**: `cargo run --quiet test.gr`

**CRITICAL**: A feature is NOT complete until BOTH levels pass!

---

## Integration Test Structure

### Directory Layout
```
tests/integration/
├── README.md                    # How to run tests, expected behavior
├── 01_builtins.gr              # print, len, type, etc.
├── 02_variables.gr             # Variable assignment, scoping
├── 03_functions.gr             # Function definitions and calls
├── 04_control_flow.gr          # if, while, for, break, continue
├── 05_lists.gr                 # List creation and methods
├── 06_maps.gr                  # Map/hash operations
├── 07_graphs_basic.gr          # Graph creation, add_node, add_edge
├── 08_graphs_query.gr          # nodes(), edges(), neighbors()
├── 09_graphs_rules.gr          # Rule system, validation
├── 10_behaviors.gr             # Behavior system (Phase 7)
├── 11_modules.gr               # Module system (Phase 8)
├── 12_pattern_matching.gr      # Pattern matching (Phase 9)
└── ... (more as features added)
```

### Integration Test Format

Each `.gr` file should:
1. **Be self-documenting** with comments
2. **Include expected output** in comments
3. **Use print statements** to show results
4. **Test multiple related features** in one file
5. **Be runnable** with `cargo run --quiet path/to/test.gr`

**Template**:
```graphoid
# tests/integration/NN_feature_name.gr
#
# Purpose: Test feature_name functionality
# Phase: N
# Features tested:
#   - feature_1
#   - feature_2
#   - feature_3
#
# Expected output:
#   Testing feature_name...
#   Test 1: PASS
#   Test 2: PASS
#   ✅ All feature_name tests passed!

print("Testing feature_name...")

# Test 1: Basic functionality
x = feature_1(args)
if x == expected {
    print("Test 1: PASS")
} else {
    print("Test 1: FAIL - Expected", expected, "got", x)
}

# Test 2: Edge case
y = feature_2(edge_case)
if y == expected {
    print("Test 2: PASS")
} else {
    print("Test 2: FAIL - Expected", expected, "got", y)
}

print("✅ All feature_name tests passed!")
```

---

## Feature Development Workflow

### OLD Workflow (Broken)
1. Implement Rust API in `src/values/*.rs`
2. Write Rust unit tests
3. Tests pass → **Mark as complete** ❌ WRONG!

### NEW Workflow (Required)

#### Step 1: Implement Rust API
```rust
// src/values/graph.rs
impl Graph {
    pub fn new_method(&self, args: Args) -> Result<Value> {
        // Implementation
    }
}
```

#### Step 2: Write Rust Unit Tests (TDD)
```rust
// tests/unit/graph_tests.rs
#[test]
fn test_new_method() {
    let g = Graph::new(...);
    let result = g.new_method(args).unwrap();
    assert_eq!(result, expected);
}
```

#### Step 3: Register in Executor ⚠️ CRITICAL STEP
```rust
// src/execution/executor.rs

// For methods:
ValueKind::Graph => {
    match method_name.as_str() {
        // ... existing methods ...
        "new_method" => {
            // Handle arguments
            let result = graph.new_method(processed_args)?;
            Ok(result)
        }
        // ...
    }
}

// For built-in functions:
fn execute_call(&mut self, name: &str, args: Vec<Expr>) -> Result<Value> {
    match name {
        // ... existing functions ...
        "new_function" => {
            let arg1 = self.evaluate_expr(&args[0])?;
            let result = new_function_impl(arg1)?;
            Ok(result)
        }
        // ...
    }
}
```

#### Step 4: Write .gr Integration Test
```graphoid
# tests/integration/NN_new_method.gr
print("Testing new_method...")

g = graph {type: :directed}
result = g.new_method(args)

if result == expected {
    print("✅ new_method works!")
} else {
    print("❌ FAIL: Expected", expected, "got", result)
}
```

#### Step 5: Run Integration Test
```bash
cargo run --quiet tests/integration/NN_new_method.gr
```

**Expected**: Should print success message, no errors

#### Step 6: Add to Example Files
```graphoid
# examples/new_method_demo.gr
# Demonstrate new_method with realistic use case
...
```

#### Step 7: Update Documentation
- Add to `dev_docs/LANGUAGE_SPECIFICATION.md`
- Update relevant phase status docs
- Add to `CLAUDE.md` if significant

#### Step 8: Commit
```bash
git add .
git commit -m "feat: implement new_method with integration tests"
```

**NOW** the feature is complete! ✅

---

## Definition of Done Checklist

Before marking ANY feature as "complete", verify ALL of these:

```markdown
## Feature: [name]

### Implementation
- [ ] Rust API implemented in `src/values/*.rs`
- [ ] Code follows Rust best practices
- [ ] No compiler warnings

### Unit Testing (Level 1)
- [ ] Rust unit tests written in `tests/unit/`
- [ ] All unit tests pass (`cargo test --lib`)
- [ ] Edge cases covered
- [ ] Test coverage > 80%

### Executor Integration ⚠️ CRITICAL
- [ ] Method/function registered in `src/execution/executor.rs`
- [ ] Argument handling implemented
- [ ] Return value handling implemented
- [ ] Error cases handled gracefully

### Integration Testing (Level 2)
- [ ] .gr integration test written in `tests/integration/`
- [ ] Integration test passes (`cargo run --quiet test.gr`)
- [ ] Realistic use case tested
- [ ] Output verified manually

### Documentation
- [ ] Language spec updated (if user-facing)
- [ ] Example file added to `examples/` (if significant feature)
- [ ] Phase status docs updated

### Verification
- [ ] Feature works from .gr file (tested manually)
- [ ] No "Undefined variable" errors
- [ ] No "does not have method" errors
- [ ] Example file can be demonstrated

**IMPORTANT**: If ANY checkbox is unchecked, feature is NOT complete!
```

---

## Running Integration Tests

### Individual Test
```bash
# Run specific test
cargo run --quiet tests/integration/01_builtins.gr

# With output comparison
cargo run --quiet tests/integration/01_builtins.gr > /tmp/actual.txt
diff /tmp/actual.txt tests/integration/01_builtins.expected.txt
```

### All Tests (Manual)
```bash
# Run all integration tests
for file in tests/integration/*.gr; do
    echo "Testing $file..."
    cargo run --quiet "$file"
    if [ $? -ne 0 ]; then
        echo "❌ FAILED: $file"
    else
        echo "✅ PASSED: $file"
    fi
done
```

### All Tests (Automated Script)
```bash
# Use provided script
bash scripts/test_integration.sh

# Output:
# Testing tests/integration/01_builtins.gr...
# ✅ PASSED: tests/integration/01_builtins.gr
# Testing tests/integration/02_variables.gr...
# ✅ PASSED: tests/integration/02_variables.gr
# ...
# ✅ All integration tests passed
```

### CI Integration
```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run unit tests
        run: cargo test --lib

      - name: Run integration tests
        run: bash scripts/test_integration.sh
```

---

## Common Patterns and Examples

### Pattern 1: Registering a Method

**Location**: `src/execution/executor.rs`, in `execute_method_call()`

```rust
// Find the appropriate ValueKind branch
ValueKind::Graph => {
    match method_name.as_str() {
        "existing_method" => { /* ... */ }

        // Add new method here
        "new_method" => {
            // 1. Validate argument count
            if args.len() != expected_count {
                return Err(GraphoidError::runtime(
                    format!("new_method() expects {} arguments, got {}",
                            expected_count, args.len())
                ));
            }

            // 2. Evaluate and extract arguments
            let arg1 = self.evaluate_expr(&args[0])?;
            let arg1_value = match &arg1.kind {
                ValueKind::String(s) => s.clone(),
                _ => return Err(GraphoidError::runtime(
                    "new_method() expects string argument".to_string()
                )),
            };

            // 3. Call the Rust API method
            let result = graph.new_method(arg1_value)?;

            // 4. Return result
            Ok(result)
        }

        // ...
    }
}
```

### Pattern 2: Registering a Built-in Function

**Location**: `src/execution/executor.rs`, in `execute_call()`

```rust
fn execute_call(&mut self, name: &str, args: Vec<Expr>) -> Result<Value> {
    match name {
        "existing_function" => { /* ... */ }

        // Add new built-in here
        "new_function" => {
            // Validate
            if args.is_empty() {
                return Err(GraphoidError::runtime(
                    "new_function() requires at least 1 argument".to_string()
                ));
            }

            // Evaluate
            let values: Result<Vec<Value>> = args.iter()
                .map(|arg| self.evaluate_expr(arg))
                .collect();
            let values = values?;

            // Call implementation
            let result = self.new_function_impl(values)?;
            Ok(result)
        }

        // ...
    }
}
```

### Pattern 3: Testing a Method

```graphoid
# tests/integration/test_new_method.gr
print("Testing Graph.new_method()...")

g = graph {type: :directed}

# Test 1: Basic usage
result1 = g.new_method("arg1")
print("Test 1:", result1)

# Test 2: Edge case
result2 = g.new_method("edge_case")
print("Test 2:", result2)

# Test 3: Error handling
# (This should fail gracefully if implemented)
# result3 = g.new_method()  # Wrong arg count

print("✅ new_method tests passed!")
```

---

## FAQ

### Q: Do I need integration tests for internal helper methods?
**A**: No. Only for user-facing features accessible from .gr files.

### Q: Can I write integration tests after implementation?
**A**: You CAN, but you SHOULD write them as part of implementation. Integration tests ARE part of TDD.

### Q: What if my feature is just refactoring internals?
**A**: Then you don't need new integration tests. But run existing integration tests to ensure no regression.

### Q: How do I test error cases in .gr files?
**A**: Currently, we don't have a good way to assert errors. Comment out error test cases or verify manually.

### Q: Can integration tests replace unit tests?
**A**: NO! You need BOTH. Unit tests verify internal correctness, integration tests verify user accessibility.

### Q: What if integration test fails but unit tests pass?
**A**: This means executor registration is missing or broken. Fix executor registration.

### Q: What if unit test fails but integration test passes?
**A**: This shouldn't happen. If it does, your unit test is wrong or testing the wrong thing.

### Q: How often should I run integration tests?
**A**:
- **During development**: After each executor registration
- **Before commit**: Always
- **In CI**: On every push

### Q: What makes a good integration test?
**A**:
- Tests realistic use cases
- Self-documenting
- Prints clear success/failure messages
- Can be run independently
- Completes in < 1 second

---

## Integration Test Examples

### Example 1: Testing Built-in Function
```graphoid
# tests/integration/test_print.gr
# Purpose: Verify print() function works
# Expected output:
#   Hello World
#   Multiple arguments: 1 2 3
#   With variables: x = 10

print("Hello World")
print("Multiple arguments:", 1, 2, 3)

x = 10
print("With variables: x =", x)
```

### Example 2: Testing Graph Methods
```graphoid
# tests/integration/test_graph_basic.gr
# Purpose: Verify basic graph operations
# Expected output:
#   Created graph
#   Nodes: 3
#   Edges: 2
#   ✅ Basic graph operations work

g = graph {type: :directed}
print("Created graph")

g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)
print("Nodes:", g.node_count())

g.add_edge("A", "B", "LINK")
g.add_edge("B", "C", "LINK")
print("Edges:", g.edge_count())

print("✅ Basic graph operations work")
```

### Example 3: Testing Pattern Matching
```graphoid
# tests/integration/test_pattern_match.gr
# Purpose: Verify pattern matching workflow (Phase 9)
# Expected output:
#   Created social graph
#   Found 2 friend connections
#   Alice -> Bob
#   Bob -> Charlie
#   ✅ Pattern matching works

g = graph {type: :directed}
g.add_node("alice", {name: "Alice", age: 30})
g.add_node("bob", {name: "Bob", age: 25})
g.add_node("charlie", {name: "Charlie", age: 28})

g.add_edge("alice", "bob", "FRIEND")
g.add_edge("bob", "charlie", "FRIEND")

print("Created social graph")

# Match pattern: person -> FRIEND -> friend
pattern = [
    node("person"),
    edge(type: "FRIEND"),
    node("friend")
]

results = g.match_pattern(pattern)
print("Found", results.len(), "friend connections")

# Print matches
for match in results {
    person_name = match["person"].get("name")
    friend_name = match["friend"].get("name")
    print(person_name, "->", friend_name)
}

print("✅ Pattern matching works")
```

---

## Troubleshooting

### Problem: "Undefined variable: function_name"
**Cause**: Function not registered in executor
**Fix**: Add to `execute_call()` in `src/execution/executor.rs`

### Problem: "Type does not have method 'method_name'"
**Cause**: Method not registered for that type
**Fix**: Add to appropriate ValueKind branch in `execute_method_call()`

### Problem: Integration test works but example file doesn't
**Cause**: Different file paths or environment
**Fix**: Verify import paths, check for relative path issues

### Problem: Integration test fails with type error
**Cause**: Argument types not handled in executor
**Fix**: Add type conversion in executor registration code

### Problem: Integration test output is empty
**Cause**: Print not working or no output generated
**Fix**: First verify print() itself works with simple test

---

## Migration Guide for Existing Features

If you're retrofitting integration tests to existing "complete" features:

### Step 1: Audit
```bash
# List all methods in implementation
grep "pub fn " src/values/graph.rs

# Check if registered
for method in $(grep "pub fn " src/values/graph.rs | awk '{print $3}' | cut -d'(' -f1); do
    if grep -q "\"$method\"" src/execution/executor.rs; then
        echo "✅ $method"
    else
        echo "❌ $method - NOT REGISTERED"
    fi
done
```

### Step 2: Register Missing Methods
Follow Pattern 1 or Pattern 2 above for each missing method.

### Step 3: Write Integration Tests
Create .gr file testing the feature end-to-end.

### Step 4: Update Phase Status
Only after all three steps, update the phase as "complete".

---

## Summary: The Three Laws of Integration Testing

1. **You MUST write integration tests for all user-facing features**
   - No exceptions, no "I'll do it later"

2. **You MUST register methods/functions in the executor**
   - Implementation without registration is incomplete

3. **You MUST verify the .gr file works before marking complete**
   - Passing Rust tests alone is NOT sufficient

Follow these laws, and the Rust API vs .gr language gap will never happen again.

---

## Quick Reference Card

```
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃  FEATURE DEVELOPMENT CHECKLIST                           ┃
┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
┃  [ ] 1. Implement Rust API                               ┃
┃  [ ] 2. Write Rust unit tests                            ┃
┃  [ ] 3. Register in executor ⚠️ CRITICAL                 ┃
┃  [ ] 4. Write .gr integration test                       ┃
┃  [ ] 5. Verify: cargo run --quiet test.gr               ┃
┃  [ ] 6. Add example file                                 ┃
┃  [ ] 7. Update docs                                      ┃
┃  [ ] 8. Commit                                           ┃
┃                                                           ┃
┃  NOT COMPLETE UNTIL ALL 8 STEPS DONE!                    ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

WHERE TO ADD CODE:
  • Rust API:        src/values/*.rs
  • Unit tests:      tests/unit/*_tests.rs
  • Registration:    src/execution/executor.rs
  • Integration:     tests/integration/*.gr
  • Examples:        examples/*.gr

COMMANDS:
  cargo test --lib                    # Run unit tests
  cargo run --quiet path/to/test.gr   # Run integration test
  bash scripts/test_integration.sh    # Run all integration tests
```

---

**Remember**: A feature that works in Rust tests but not in .gr files is NOT a feature - it's a bug waiting to be discovered!

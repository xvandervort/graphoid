# Testing Guidelines

**Graphoid Rust Implementation**

This document establishes the testing conventions and best practices for the Graphoid Rust implementation.

---

## ⚠️ MANDATORY PRACTICES

### 1. Test-Driven Development (TDD) is REQUIRED

**All development follows strict TDD**:

1. 🔴 **RED**: Write failing tests FIRST (before any implementation)
2. 🟢 **GREEN**: Write minimal code to make tests pass
3. 🔵 **REFACTOR**: Clean up code while keeping tests passing

**This is non-negotiable**. Do not write implementation code before writing tests.

### 2. Separate Tests from Implementation

**Rule**: Tests must NEVER be placed inline with implementation code using `#[cfg(test)]` modules.

### ❌ WRONG - Inline Tests (DO NOT DO THIS)

```rust
// src/graph/rules.rs

pub struct NoCyclesRule { /* ... */ }

impl NoCyclesRule {
    pub fn new() -> Self { /* ... */ }
    pub fn validate(&self, graph: &Graph) -> Result<(), Error> { /* ... */ }
}

// ❌ WRONG: Tests in the same file as implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_cycles_rule() {
        // Test code...
    }
}
```

### ✅ CORRECT - Separate Test Files

```rust
// src/graph/rules.rs
// (Implementation only - NO tests)

pub struct NoCyclesRule { /* ... */ }

impl NoCyclesRule {
    pub fn new() -> Self { /* ... */ }
    pub fn validate(&self, graph: &Graph) -> Result<(), Error> { /* ... */ }
}
```

```rust
// tests/unit/graph_rules_tests.rs
// (Tests in separate file)

use graphoid::graph::rules::NoCyclesRule;
use graphoid::values::{Graph, GraphType, Value};

#[test]
fn test_no_cycles_rule_allows_acyclic_edge() {
    let mut graph = Graph::new(GraphType::Directed);
    // Test code...
}

#[test]
fn test_no_cycles_rule_detects_cycle() {
    let mut graph = Graph::new(GraphType::Directed);
    // Test code...
}
```

---

## Test Organization Structure

```
rust/
├── src/                          # Implementation code (NO tests!)
│   ├── lib.rs
│   ├── main.rs
│   ├── graph/
│   │   ├── rules.rs             # ✅ No #[cfg(test)]
│   │   └── rulesets.rs          # ✅ No #[cfg(test)]
│   ├── values/
│   │   ├── mod.rs               # ✅ No #[cfg(test)]
│   │   └── graph.rs             # ✅ No #[cfg(test)]
│   └── execution/
│       ├── executor.rs          # ✅ No #[cfg(test)]
│       └── environment.rs       # ✅ No #[cfg(test)]
│
└── tests/                        # All tests here!
    ├── unit/                     # Unit tests
    │   ├── graph_rules_tests.rs
    │   ├── rulesets_tests.rs
    │   ├── values_tests.rs
    │   ├── graph_tests.rs
    │   └── executor_tests.rs
    │
    ├── integration/              # Integration tests
    │   └── end_to_end_tests.rs
    │
    └── unit_tests.rs             # Register unit test modules here
```

---

## Naming Conventions

### Test File Names

- **Pattern**: `{module_name}_tests.rs`
- **Examples**:
  - `src/graph/rules.rs` → `tests/unit/graph_rules_tests.rs`
  - `src/values/graph.rs` → `tests/unit/graph_tests.rs`
  - `src/execution/executor.rs` → `tests/unit/executor_tests.rs`

### Test Function Names

- **Pattern**: `test_{what_is_being_tested}_{scenario}`
- **Examples**:
  - `test_no_cycles_rule_allows_acyclic_edge()`
  - `test_dijkstra_chooses_lighter_path()`
  - `test_nodes_within_zero_hops()`

### Test Module Organization

Group related tests with comments:

```rust
// ============================================================================
// EdgeInfo Weight Methods Tests (10 tests)
// ============================================================================

#[test]
fn test_edgeinfo_new_creates_unweighted_edge() { /* ... */ }

#[test]
fn test_edgeinfo_new_weighted_creates_weighted_edge() { /* ... */ }

// ============================================================================
// Graph Weight Mutation Tests (15 tests)
// ============================================================================

#[test]
fn test_graph_get_edge_weight_unweighted() { /* ... */ }
```

---

## Test File Template

```rust
//! Tests for [module description]
//!
//! This file tests [what functionality]

use graphoid::module::path::{Type1, Type2};
use graphoid::values::{Value, Graph};
use std::collections::HashMap;

// ============================================================================
// [Test Category Name] ([N] tests)
// ============================================================================

#[test]
fn test_feature_basic_case() {
    // Arrange
    let mut graph = Graph::new(GraphType::Directed);

    // Act
    let result = graph.do_something();

    // Assert
    assert_eq!(result, expected_value);
}

#[test]
fn test_feature_edge_case() {
    // Test implementation...
}
```

---

## Registering New Test Files

When creating a new test file in `tests/unit/`, register it in `tests/unit_tests.rs`:

```rust
//! Unit tests for Graphoid

mod unit {
    pub mod lexer_tests;
    pub mod parser_tests;
    pub mod executor_tests;
    pub mod graph_tests;
    // ... existing modules ...
    pub mod your_new_test_file;  // ← Add here
}
```

---

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Specific Test File
```bash
cargo test graph_rules
```

### Run Specific Test
```bash
cargo test test_no_cycles_rule_detects_cycle
```

### Run Unit Tests Only
```bash
cargo test --test unit_tests
```

### Run Integration Tests Only
```bash
cargo test --test integration_tests
```

### Run with Output
```bash
cargo test -- --nocapture
```

---

## Test-Driven Development (TDD)

**MANDATORY**: Graphoid follows strict TDD practices for all development.

### Why TDD is Required

1. **Complete Test Coverage**: Writing tests first ensures every feature is tested
2. **Better API Design**: Tests reveal API usability issues before implementation
3. **Prevents Regressions**: Comprehensive test suite catches breaking changes
4. **Living Documentation**: Tests serve as executable examples
5. **Confidence**: Change code fearlessly knowing tests will catch issues

### The TDD Cycle

**ALWAYS follow this cycle** - never skip directly to implementation:

### 🔴 RED Phase
Write failing tests FIRST

```rust
#[test]
fn test_dijkstra_simple_weighted_path() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
    graph.add_edge("A", "B", "road".to_string(), Some(5.0), HashMap::new()).unwrap();

    // This will fail initially - method doesn't exist yet
    let path = graph.shortest_path("A", "B", None, true).unwrap();
    assert_eq!(path, vec!["A".to_string(), "B".to_string()]);
}
```

### 🟢 GREEN Phase
Implement the minimum code to make tests pass

```rust
// src/values/graph.rs
impl Graph {
    pub fn shortest_path(&self, from: &str, to: &str, edge_type: Option<&str>, weighted: bool) -> Option<Vec<String>> {
        // Implementation here...
    }
}
```

### 🔵 REFACTOR Phase
Clean up code while keeping tests passing

### Real Example from Phase 6.6

**Task**: Implement `nodes_within()` method for hop-limited graph traversal

**Step 1 (RED)**: Write 10 failing tests first
```rust
// tests/unit/weighted_graph_tests.rs
#[test]
fn test_nodes_within_zero_hops() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
    graph.add_edge("A", "B", "road".to_string(), None, HashMap::new()).unwrap();

    let nodes = graph.nodes_within("A", 0, None);  // Method doesn't exist yet!
    assert_eq!(nodes, vec!["A".to_string()]);
}

// ... 9 more tests ...
```

**Step 2 (GREEN)**: Implement to make tests pass
```rust
// src/values/graph.rs
pub fn nodes_within(&self, start: &str, hops: usize, edge_type: Option<&str>) -> Vec<String> {
    // BFS implementation with hop tracking...
}
```

**Step 3 (REFACTOR)**: Verify all 10 tests pass, clean up code
```bash
cargo test nodes_within
# Result: test result: ok. 10 passed; 0 failed
```

**Benefit**: We had 10 comprehensive tests before writing any implementation code, ensuring complete coverage and validating the API design.

---

## Verification

### Ensure No Inline Tests

Run this command regularly to verify compliance:

```bash
find src -name "*.rs" -type f -exec grep -l "#\[cfg(test)\]" {} \;
```

**Expected output**: (empty)

If any files are listed, tests need to be moved to `tests/unit/`.

### Check Test Count

```bash
cargo test 2>&1 | grep "test result"
```

Expected output format:
```
test result: ok. 625 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Verify TDD Compliance

**Code Review Checklist**:

When reviewing PRs, verify TDD was followed:

- [ ] Are there new tests for new features?
- [ ] Were tests written before implementation?
- [ ] Do tests cover edge cases and error conditions?
- [ ] Do all tests pass?
- [ ] Is test coverage maintained or improved?

**Red Flags** (indicates TDD was NOT followed):
- ❌ Large implementation PR with no new tests
- ❌ Tests added as an afterthought (in separate commit after implementation)
- ❌ Tests only cover happy path, not edge cases
- ❌ Tests have same commit timestamp as implementation (should be earlier)

**Good Signs** (indicates TDD was followed):
- ✅ Test commit precedes implementation commit
- ✅ Comprehensive test coverage including edge cases
- ✅ Tests written in RED-GREEN-REFACTOR cycle
- ✅ Commit messages mention "TDD" or "write tests first"

---

## Why This Convention?

### Benefits

1. **Cleaner Source Files**
   - Implementation code is not cluttered with test code
   - Easier to navigate and understand production code
   - Smaller file sizes for implementation

2. **Faster Compilation**
   - Tests only compile when running test suite
   - Production builds skip test code entirely
   - Faster development iterations

3. **Better Organization**
   - All tests in one place (`tests/`)
   - Easy to find and maintain tests
   - Clear separation of concerns

4. **Scalability**
   - Works well as codebase grows
   - Standard practice for larger Rust projects
   - Follows Cargo conventions

5. **Test Discoverability**
   - New contributors can easily find tests
   - Clear mapping: `src/module.rs` → `tests/unit/module_tests.rs`
   - Test coverage is visible

---

## Common Patterns

### Testing Public APIs

```rust
// tests/unit/graph_tests.rs
use graphoid::values::{Graph, GraphType, Value};

#[test]
fn test_graph_add_node() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    assert!(graph.has_node("A"));
}
```

### Testing Internal Types

If you need to test internal (non-pub) types, make them `pub(crate)`:

```rust
// src/graph/rules.rs
pub(crate) struct RuleContext { /* ... */ }  // Visible in tests
```

Then test from `tests/unit/`:

```rust
// tests/unit/graph_rules_tests.rs
use graphoid::graph::rules::RuleContext;  // Works because pub(crate)
```

### Testing Error Cases

```rust
#[test]
fn test_graph_add_edge_nonexistent_node_fails() {
    let mut graph = Graph::new(GraphType::Directed);

    let result = graph.add_edge("A", "B", "edge".to_string(), None, HashMap::new());

    assert!(result.is_err());
    if let Err(GraphoidError::Runtime { message, .. }) = result {
        assert!(message.contains("Node 'A' not found"));
    }
}
```

### Testing with Setup/Teardown

```rust
// Helper function for common setup
fn create_test_graph() -> Graph {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
    graph
}

#[test]
fn test_with_common_setup() {
    let graph = create_test_graph();
    // Test using pre-configured graph...
}
```

---

## Migration Guide

If you find inline tests (violating the convention), migrate them:

### Step 1: Create Test File

```bash
touch tests/unit/module_name_tests.rs
```

### Step 2: Copy Test Module Content

Copy the contents of the `#[cfg(test)] mod tests { ... }` block

### Step 3: Update Imports

Change from:
```rust
use super::*;
```

To:
```rust
use graphoid::module::path::{Type1, Type2};
```

### Step 4: Remove Inline Tests

Delete the entire `#[cfg(test)] mod tests { ... }` block from source file

### Step 5: Register Test Module

Add to `tests/unit_tests.rs`:
```rust
pub mod module_name_tests;
```

### Step 6: Verify

```bash
cargo test module_name
```

---

## Questions?

- Check existing test files in `tests/unit/` for examples
- Consult this guide
- Ask in project discussions
- Reference: [Rust Testing Best Practices](https://doc.rust-lang.org/book/ch11-00-testing.html)

---

**Last Updated**: October 31, 2025
**Status**: 625 tests passing, 0 inline test modules in src/

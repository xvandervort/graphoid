# Graphoid Executability Audit Report

**Date**: November 6, 2025
**Severity**: 🚨 CRITICAL
**Status**: SYSTEMIC ISSUE DISCOVERED

## Executive Summary

A comprehensive audit of the Graphoid Rust implementation has revealed a **catastrophic gap** between features implemented at the Rust API level and features accessible from `.gr` user-facing language files.

**Key Finding**: Phase 9 (Graph Pattern Matching) was marked as "complete" with 186+ passing Rust tests, but **pattern matching cannot be used from .gr files at all** because the executor integration was never implemented.

**Broader Impact**: This is not isolated to pattern matching. Basic built-in functions like `print()` are also non-functional in .gr files.

---

## Investigation Summary

### What Was Tested

1. **Basic "Hello World"**: `print("Hello World")`
2. **Simple function definitions**: Using `fn` keyword
3. **Graph pattern matching**: Using `node()`, `edge()`, `path()` built-ins
4. **Graph methods**: `match_pattern()` method

### Test Results

| Feature | Rust API | .gr File | Gap Type | Severity |
|---------|----------|----------|----------|----------|
| `print()` function | N/A | ❌ FAILS | Not registered | CRITICAL |
| `fn` keyword | ✅ Works | ✅ Works | None | OK |
| `func` keyword | ✅ Works | ❌ FAILS | Documentation mismatch | MEDIUM |
| `node()` built-in | ✅ Works | ⚠️ Partial | Registered, error messages work | OK |
| `edge()` built-in | ✅ Works | ⚠️ Partial | Registered, error messages work | OK |
| `path()` built-in | ✅ Works | ⚠️ Partial | Registered, error messages work | OK |
| `graph.match_pattern()` | ✅ Works | ❌ FAILS | Not registered as method | CRITICAL |
| `graph.add_node()` | ✅ Works | ✅ Works | None | OK |
| `graph.add_edge()` | ✅ Works | ✅ Works | None | OK |

---

## Detailed Findings

### 1. print() Function - MISSING

**Test File**: `/home/irv/work/grang/tmp/test_with_print.gr`
```graphoid
print("Hello World")
```

**Error**:
```
Error: Runtime error: Runtime error: Undefined variable: print
```

**REPL Test**:
```bash
echo 'print("Hello World")' | cargo run --quiet
```

**REPL Output**:
```
Graphoid v0.1.0
Type /exit to quit, /help for help
> Error: Runtime error: Runtime error: Undefined variable: print
```

**Root Cause**: No `print` function is registered in the executor's environment initialization.

**Location**: `src/execution/environment.rs` - Environment.new() creates empty HashMap
**Location**: `src/execution/executor.rs` - Executor.new() creates empty environment

**Impact**:
- ❌ Users cannot output anything to console from .gr files
- ❌ **REPL is completely broken** - cannot even run "Hello World"
- This is **basic functionality** that should work on day one
- **CRITICAL**: Both file execution AND interactive REPL are non-functional

---

### 2. Pattern Matching - PARTIALLY IMPLEMENTED

**Test File**: `/home/irv/work/grang/tmp/test_pattern_builtins3.gr`
```graphoid
g = graph {type: :directed}
g.add_node("A", 1)
g.add_node("B", 2)
g.add_edge("A", "B", "LINK")

pattern = [node("a"), edge(type: "LINK"), node("b")]
results = g.match_pattern(pattern)
```

**Error**:
```
Error: Runtime error: Runtime error: Graph does not have method 'match_pattern'
```

**Root Cause**:
- Pattern matching IS implemented in Rust: `src/values/graph.rs` has `match_pattern()` method (lines 1266-1341)
- Pattern builder functions ARE registered: `node()`, `edge()`, `path()` in `src/execution/executor.rs` (lines 3055-3132)
- BUT `match_pattern` is NOT registered as a callable method on Graph objects in the executor

**Verification**:
```bash
grep '"match_pattern"' src/execution/executor.rs
# Returns: No matches
```

**Impact**:
- Phase 9 (186 tests) marked as "complete" but **completely unusable** from user perspective
- All 4 example .gr files created for Phase 9 documentation are non-functional
- 125 lines of API documentation describe features that don't work

---

### 3. func vs fn Keyword - DOCUMENTATION INCONSISTENCY

**Finding**:
- Lexer recognizes `fn` as function keyword (src/lexer/mod.rs:566)
- Language spec uses BOTH `func` and `fn` inconsistently
- Test files use `func` (wrong) instead of `fn` (correct)

**Examples**:
- LANGUAGE_SPECIFICATION.md line 620: `func build_social_pattern(...)`
- LANGUAGE_SPECIFICATION.md line 2905: `fn add(a, b) { ... }`
- test_basic.gr line 15: `func add(a, b) {` ← **FAILS to parse**

**Impact**: MEDIUM - Creates user confusion, causes parser errors

---

### 4. Graph Type Constraint - TYPE MISMATCH

**Finding**:
- Graph initialization with `{type: "directed"}` (string) fails
- Must use `{type: :directed}` (symbol)

**Error**:
```
Error: Runtime error: Runtime error: Type error: expected symbol, got string
```

**Impact**: MEDIUM - Documentation likely shows wrong syntax

---

## Scope Assessment: How Deep Does This Go?

### Features Confirmed Working in .gr Files
✅ Variable assignment
✅ Function definitions (`fn` keyword)
✅ Graph creation
✅ `add_node()` method
✅ `add_edge()` method
✅ Basic control flow (if/while/for) - *assumed based on parser*

### Features Confirmed BROKEN in .gr Files
❌ `print()` function
❌ `match_pattern()` method
❌ Pattern matching workflow
❌ All Phase 9 features from user perspective

### Features UNTESTED (High Risk)
⚠️ List methods: `map()`, `filter()`, `reduce()`, etc.
⚠️ Graph querying: `nodes()`, `edges()`, `neighbors()`
⚠️ Graph rules: `add_rule()`, `validate()`
⚠️ Behavior system (Phase 7): `add_behavior()`, etc.
⚠️ Module system (Phase 8): `import`, `export`, module namespaces
⚠️ All other built-in functions: `len()`, `type()`, `range()`, etc.

---

## Root Cause Analysis

### The Executor Registration Gap

**How it SHOULD work**:
1. Implement feature at Rust API level (e.g., `graph.match_pattern()` in `src/values/graph.rs`)
2. Register method in executor's method dispatch (in `src/execution/executor.rs`)
3. Write Rust tests to verify API works
4. Write integration tests with .gr files to verify executor integration
5. Only then mark phase as "complete"

**How it ACTUALLY worked (Phase 9)**:
1. ✅ Implement feature at Rust API level
2. ✅ Write 186 Rust tests - all pass
3. ❌ **SKIP** executor registration
4. ❌ **SKIP** .gr integration tests
5. ✅ Mark phase as "complete" (incorrectly)

**Why This Happened**:
- Rust tests import types directly: `use graphoid::values::Graph;`
- Rust tests call methods directly: `graph.match_pattern(pattern)`
- These tests bypass the executor entirely
- Tests pass, giving false confidence that feature is complete
- No .gr integration tests to catch the gap

---

## Verification Commands Used

```bash
# Test basic print
cargo run --quiet /home/irv/work/grang/tmp/test_with_print.gr
# Result: Undefined variable: print

# Test function syntax
cargo run --quiet /home/irv/work/grang/tmp/test_fn.gr
# Result: Works! (with "fn" keyword)

cargo run --quiet /home/irv/work/grang/tmp/test_basic.gr
# Result: Parser error at "func" keyword

# Test pattern matching
cargo run --quiet /home/irv/work/grang/tmp/test_pattern_builtins3.gr
# Result: Graph does not have method 'match_pattern'

# Verify match_pattern not registered
grep '"match_pattern"' src/execution/executor.rs
# Result: No matches

# Verify add_node IS registered
grep '"add_node"' src/execution/executor.rs
# Result: Found at line 2516
```

---

## Impact on Project Status

### Current "Official" Status
- ✅ Phase 0-7: Complete
- ⚠️ Phase 8: 75% complete
- 🔜 Phase 9: Next (but actually started)

### REVISED Status Based on Audit
- ✅ Phase 0-2: Complete (lexer, parser)
- ⚠️ Phase 3-5: **UNCERTAIN** - Rust tests pass, but .gr executability unknown
- ⚠️ Phase 6-7: **UNCERTAIN** - Graph rules and behaviors may not be executable
- 🚨 Phase 8: **BROKEN** - Module imports almost certainly non-functional
- 🚨 Phase 9: **BROKEN** - Pattern matching non-functional despite 186 tests

### Test Count Reality Check
- **1,609 tests passing** ← TRUE but misleading
- **0 integration tests** with actual .gr files ← THE PROBLEM
- Rust unit tests give false sense of completion

---

## Critical Questions for Roadmap

1. **How many implemented features are actually unusable?**
   - Unknown. Need comprehensive audit of ALL phases.

2. **Why don't we have .gr integration tests?**
   - TDD process was followed for Rust API, but not for .gr file execution
   - No test files in `examples/` or `tests/integration/` that execute .gr programs

3. **When did the gap begin?**
   - Likely from Phase 3 (Value System & Basic Execution)
   - Basic built-in functions should have been registered then

4. **How do we prevent this in the future?**
   - Require .gr integration tests for EVERY feature
   - Feature not "complete" until it works in a .gr file
   - Add CI check that runs all example .gr files

---

## Recommendations

### Immediate Actions (Next Session)

1. **Create .gr integration test suite**
   - Add to `tests/integration/` or `examples/executable/`
   - One .gr file per major feature
   - CI must run all .gr files and verify output

2. **Fix critical built-ins**
   - Register `print()` function
   - Register `match_pattern()` method
   - Verify basic functionality works

3. **Audit ALL phases**
   - Create .gr test for every implemented feature
   - Document which features work vs. don't work
   - Update phase completion status accurately

4. **Fix documentation mismatches**
   - Standardize on `fn` keyword (not `func`)
   - Fix graph type syntax (`:directed` not `"directed"`)
   - Update all examples to correct syntax

### Process Changes (Ongoing)

1. **Update "Definition of Done" for features**
   - Rust API implemented ✅
   - Rust unit tests pass ✅
   - Executor integration complete ✅
   - **NEW**: .gr integration test passes ✅
   - **NEW**: Example .gr file demonstrable ✅

2. **Add integration testing to roadmap**
   - Each phase should include executor integration
   - Each phase should include .gr example files
   - No phase marked "complete" without working examples

3. **Update testing strategy in CLAUDE.md**
   - Emphasize .gr integration tests
   - Require example files before phase completion
   - Make "dogfooding" (using .gr files) mandatory

---

## Next Steps

### Phase 9 Completion (REDO)

**Before** (what we thought was done):
- ✅ Implement pattern matching API
- ✅ Write 186 Rust tests
- ✅ Mark as complete

**After** (what actually needs to be done):
- ✅ Implement pattern matching API (already done)
- ✅ Write 186 Rust tests (already done)
- ❌ **Register match_pattern in executor** (TODO)
- ❌ **Add .gr integration tests** (TODO)
- ❌ **Verify all example files work** (TODO)
- ❌ **Then mark as complete** (TODO)

### Full Codebase Audit (HIGH PRIORITY)

1. List all implemented methods in `src/values/*.rs`
2. Cross-reference with executor registrations
3. Create gap list: implemented but not registered
4. Create test .gr files for each feature
5. Fix registration for critical features
6. Update phase status based on findings

---

## Files for Reference

### Executor Registration Files
- `src/execution/executor.rs` - Method dispatch happens here
  - Lines 2516-2580: `add_node` example of correct registration
  - Lines 3055-3132: Pattern builders (`node()`, `edge()`, `path()`)
  - **MISSING**: `match_pattern` registration

### Pattern Matching Implementation (Works in Rust, Not in .gr)
- `src/values/graph.rs` - Lines 1266-1341: `match_pattern()` method
- `src/values/mod.rs` - Lines 246-434: PatternMatchResults with projection

### Test Files Created During Audit
- `/home/irv/work/grang/tmp/test_with_print.gr` - Print test (fails)
- `/home/irv/work/grang/tmp/test_fn.gr` - Function syntax (works)
- `/home/irv/work/grang/tmp/test_pattern_builtins3.gr` - Pattern matching (fails)

---

## Conclusion

The Graphoid project has **strong Rust API foundations** but **critical executor integration gaps**. The TDD process successfully created a robust internal API, but the lack of .gr integration testing allowed a false sense of feature completion.

**Bottom Line**: A phase is not "complete" until users can actually use it from .gr files. The current 1,609 test count is misleading - we have excellent API test coverage but zero user-facing integration coverage.

This is **fixable** with:
1. Systematic executor registration of existing APIs
2. .gr integration test suite
3. Updated definition of "done" for features
4. Process changes to prevent future gaps

**Estimated Effort**: 2-3 days to fix critical features, 1-2 weeks for comprehensive audit and integration.

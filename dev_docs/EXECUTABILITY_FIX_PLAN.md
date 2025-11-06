# Graphoid Executability Fix Plan

**Date**: November 6, 2025
**Related**: See `EXECUTABILITY_AUDIT.md` for full findings
**Priority**: CRITICAL
**Timeline**: 2-3 days critical fixes, 1-2 weeks comprehensive

---

## Plan Overview

This document outlines the step-by-step plan to fix the executor integration gaps identified in the executability audit.

**Goal**: Make ALL implemented Rust API features accessible from .gr user-facing language files.

**Approach**: Three-phase strategy
1. **Phase 1**: Fix critical built-ins (print, pattern matching) - 1 day
2. **Phase 2**: Comprehensive feature audit and registration - 3-5 days
3. **Phase 3**: Integration test suite and process improvements - 3-5 days

---

## Phase 1: Critical Fixes (1 Day)

### Fix 1.1: Register print() Function

**Priority**: CRITICAL - Users cannot output anything without this

**Files to Modify**:
- `src/execution/executor.rs` - Add print handling to function call dispatch

**Implementation**:
```rust
// In executor.rs, in the execute_call method
// Around line 2979 in the function dispatch section

"print" => {
    // Handle variable arguments
    let mut output = String::new();
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            output.push(' ');
        }
        let value = self.evaluate_expr(arg)?;
        output.push_str(&self.value_to_string(&value));
    }
    println!("{}", output);
    Ok(Value::none())
}
```

**Test**:
```graphoid
# test_print.gr
print("Hello World")
print("x =", 10)
print(1, 2, 3)
```

**Expected Output**:
```
Hello World
x = 10
1 2 3
```

---

### Fix 1.2: Register match_pattern() Method

**Priority**: CRITICAL - Phase 9 completely non-functional without this

**Files to Modify**:
- `src/execution/executor.rs` - Add to Graph method dispatch

**Location**: In `execute_method_call()`, in the Graph method dispatch section (around line 2516 where `add_node` is)

**Implementation**:
```rust
// In the ValueKind::Graph branch, add after existing methods:

"match_pattern" => {
    // match_pattern expects a list of pattern elements
    if args.len() != 1 {
        return Err(GraphoidError::runtime(
            format!("match_pattern() expects 1 argument (pattern list), got {}", args.len())
        ));
    }

    let pattern_arg = self.evaluate_expr(&args[0])?;

    // Convert Value list to pattern elements
    let pattern_list = match &pattern_arg.kind {
        ValueKind::List(items) => items,
        _ => return Err(GraphoidError::runtime(
            "match_pattern() expects a list of pattern elements".to_string()
        )),
    };

    let pattern: Vec<PatternElement> = pattern_list.iter()
        .map(|item| {
            // Extract PatternElement from Value
            // Assuming PatternElement is wrapped in Value somehow
            // This may need adjustment based on how node()/edge() work
            match &item.kind {
                ValueKind::PatternElement(elem) => Ok(elem.clone()),
                _ => Err(GraphoidError::runtime(
                    "Pattern list must contain pattern elements (from node/edge/path)".to_string()
                ))
            }
        })
        .collect::<Result<Vec<_>>>()?;

    let results = graph.match_pattern(pattern)?;
    Ok(Value::pattern_results(results))
}
```

**Note**: This implementation assumes PatternElement can be stored in Value. May need to add a ValueKind::PatternElement variant.

**Test**:
```graphoid
# test_match_pattern.gr
g = graph {type: :directed}
g.add_node("A", 1)
g.add_node("B", 2)
g.add_edge("A", "B", "LINK")

pattern = [node("a"), edge(type: "LINK"), node("b")]
results = g.match_pattern(pattern)
print("Found", results.len(), "matches")
```

**Expected Output**:
```
Found 1 matches
```

---

### Fix 1.3: Documentation Sync - func vs fn

**Priority**: MEDIUM - Causes parser errors

**Files to Modify**:
- `dev_docs/LANGUAGE_SPECIFICATION.md` - Change all `func` to `fn`
- `/home/irv/work/grang/tmp/test_basic.gr` - Fix example
- Any other documentation files

**Strategy**:
```bash
# Find all occurrences
grep -r "^func " dev_docs/
grep -r "^\tfunc " dev_docs/

# Replace with fn
# Do manually to avoid breaking "function" in prose
```

**Verification**:
```bash
# Should return nothing:
grep "^func " dev_docs/LANGUAGE_SPECIFICATION.md
```

---

## Phase 2: Comprehensive Audit (3-5 Days)

### Step 2.1: Inventory ALL Rust API Methods

**Goal**: Create complete list of implemented methods

**Files to Audit**:
- `src/values/graph.rs` - All Graph methods
- `src/values/list.rs` - All List methods (if exists)
- `src/values/mod.rs` - All Value methods
- Other value type files

**Method**:
```bash
# Extract all pub fn methods
grep "pub fn " src/values/*.rs > /tmp/api_methods.txt

# Manual review to categorize:
# - Built-in functions (print, len, type, etc.)
# - Graph methods (add_node, match_pattern, etc.)
# - List methods (map, filter, reduce, etc.)
# - Other collection methods
```

**Deliverable**: `API_INVENTORY.md` with complete method list

---

### Step 2.2: Check Executor Registration for Each Method

**Goal**: Identify which methods are NOT registered

**Method**:
For each method in inventory:
1. Search in `src/execution/executor.rs` for method name
2. Mark as REGISTERED or MISSING
3. Prioritize by usage frequency/importance

**Script Template**:
```bash
# For each method
method="method_name"
if grep -q "\"$method\"" src/execution/executor.rs; then
    echo "✅ $method - REGISTERED"
else
    echo "❌ $method - MISSING"
fi
```

**Deliverable**: `REGISTRATION_GAPS.md` with prioritized fix list

---

### Step 2.3: Register Missing Methods by Priority

**Priority Tiers**:
1. **CRITICAL**: Basic functionality (print, len, type, etc.)
2. **HIGH**: Phase 3-7 core features (collections, graphs, behaviors)
3. **MEDIUM**: Phase 8-9 features (modules, pattern matching)
4. **LOW**: Advanced/optional features

**Daily Goals**:
- Day 1: Register CRITICAL built-ins
- Day 2: Register HIGH priority methods
- Day 3-4: Register MEDIUM priority methods
- Day 5: Register LOW priority methods, cleanup

**Implementation Pattern**:
For each method:
1. Find implementation in `src/values/*.rs`
2. Add case to appropriate match statement in `src/execution/executor.rs`
3. Handle arguments and return values
4. Write .gr test file
5. Verify test passes
6. Commit

---

### Step 2.4: Test Each Feature with .gr Files

**Goal**: Verify every method works from .gr files

**Structure**:
```
tests/integration/
├── 01_builtins.gr          # print, len, type, etc.
├── 02_variables.gr         # assignment, scoping
├── 03_functions.gr         # fn definitions, calls
├── 04_lists.gr             # list methods: map, filter, etc.
├── 05_graphs_basic.gr      # add_node, add_edge
├── 06_graphs_query.gr      # nodes, edges, neighbors
├── 07_graphs_rules.gr      # add_rule, validate
├── 08_behaviors.gr         # behavior system
├── 09_modules.gr           # import, export
├── 10_pattern_matching.gr  # Phase 9 features
└── README.md               # How to run, expected outputs
```

**Each .gr file**:
- Tests multiple related features
- Includes expected output in comments
- Self-documenting with print statements
- Can be run with `cargo run --quiet tests/integration/XX_name.gr`

**Example**:
```graphoid
# tests/integration/01_builtins.gr
# Expected output:
#   Hello World
#   5
#   num

print("Hello World")
x = [1, 2, 3, 4, 5]
print(x.len())
print(x.type())
```

---

## Phase 3: Process Improvements (3-5 Days)

### Step 3.1: Create CI Integration Test Runner

**Goal**: Automatically verify all .gr files execute correctly

**Implementation**:
```bash
# scripts/test_integration.sh
#!/bin/bash

FAILED=0
for file in tests/integration/*.gr; do
    echo "Testing $file..."
    cargo run --quiet "$file" > /tmp/output.txt 2>&1

    if [ $? -ne 0 ]; then
        echo "❌ FAILED: $file"
        cat /tmp/output.txt
        FAILED=$((FAILED + 1))
    else
        echo "✅ PASSED: $file"
    fi
done

if [ $FAILED -gt 0 ]; then
    echo ""
    echo "❌ $FAILED integration tests failed"
    exit 1
else
    echo ""
    echo "✅ All integration tests passed"
    exit 0
fi
```

**Add to CI**:
```yaml
# .github/workflows/test.yml
- name: Run integration tests
  run: bash scripts/test_integration.sh
```

---

### Step 3.2: Update "Definition of Done"

**Files to Modify**:
- `CLAUDE.md` - Development Guidelines section
- `dev_docs/RUST_IMPLEMENTATION_ROADMAP.md` - Each phase's success criteria

**New "Feature Complete" Checklist**:
```markdown
## Feature Completion Checklist

A feature is considered "complete" when ALL of the following are true:

- [ ] Rust API implemented in `src/values/*.rs`
- [ ] Rust unit tests written and passing
- [ ] **NEW**: Method registered in `src/execution/executor.rs`
- [ ] **NEW**: Integration test (.gr file) written
- [ ] **NEW**: Integration test passes
- [ ] **NEW**: Example .gr file added to `examples/`
- [ ] Documentation updated
- [ ] No compiler warnings

**IMPORTANT**: Features are NOT complete until they work from .gr files!
```

---

### Step 3.3: Update CLAUDE.md with New Process

**Section to Add**: Integration Testing Requirements

```markdown
### Integration Testing Strategy

**CRITICAL REQUIREMENT**: All features must be tested at both the Rust API level
AND the .gr user-facing level.

**Why**: Rust unit tests only verify the internal API. They don't test that
features are actually accessible from .gr files through the executor.

**Process for Each Feature**:
1. Implement Rust API in `src/values/*.rs`
2. Write Rust unit tests (RED-GREEN-REFACTOR)
3. **Register in executor** (`src/execution/executor.rs`)
4. **Write .gr integration test** (in `tests/integration/`)
5. **Verify .gr test passes** (`cargo run --quiet test.gr`)
6. Add example to `examples/`
7. Only then mark feature as "complete"

**Example .gr Integration Test**:
```graphoid
# tests/integration/test_feature_name.gr
# Tests: feature_name functionality
# Expected output: (describe what should print)

print("Testing feature_name...")
# ... test code ...
print("✅ feature_name works!")
```

**Running Integration Tests**:
```bash
# Run all integration tests
bash scripts/test_integration.sh

# Run specific test
cargo run --quiet tests/integration/01_builtins.gr
```
```

---

### Step 3.4: Audit Existing Phase Completion Status

**Goal**: Update phase status based on actual .gr executability

**Method**:
For each completed phase (0-7):
1. List all features claimed as "complete"
2. Test each feature from .gr file
3. Mark phase as:
   - ✅ COMPLETE: All features work in .gr
   - ⚠️ PARTIAL: Some features work
   - ❌ INCOMPLETE: Major features don't work

**Update Files**:
- `dev_docs/PHASE_9_ACTUAL_STATUS.md`
- `dev_docs/RUST_IMPLEMENTATION_ROADMAP.md`
- `CLAUDE.md` - Current status section

**Expected Outcome**: More honest assessment of project status

---

## Timeline Summary

### Week 1: Critical Fixes and Audit
- **Day 1**: Implement Phase 1 critical fixes (print, match_pattern)
- **Day 2**: Complete API inventory and registration audit
- **Day 3**: Register high-priority missing methods
- **Day 4**: Continue registration, start .gr test suite
- **Day 5**: Complete .gr test suite for existing features

### Week 2: Integration and Process
- **Day 6**: Create CI integration test runner
- **Day 7**: Update all documentation (CLAUDE.md, roadmap)
- **Day 8**: Audit existing phase completion status
- **Day 9**: Fix any remaining gaps found in audit
- **Day 10**: Final verification, update project status

---

## Success Criteria

**Phase 1 Success**:
- [ ] print() works from .gr files
- [ ] match_pattern() works from .gr files
- [ ] func/fn documentation consistent
- [ ] Test files execute successfully

**Phase 2 Success**:
- [ ] Complete API inventory exists
- [ ] Registration gap list documented
- [ ] All high-priority methods registered
- [ ] .gr test file for each feature

**Phase 3 Success**:
- [ ] CI runs integration tests automatically
- [ ] Definition of Done updated everywhere
- [ ] Phase status accurately reflects .gr executability
- [ ] Process prevents future gaps

**Overall Success**:
- [ ] ALL implemented features work from .gr files
- [ ] Zero "Undefined variable" errors for implemented features
- [ ] Zero "does not have method" errors for implemented methods
- [ ] Example .gr files are demonstrable
- [ ] Future features require .gr tests before completion

---

## Risk Assessment

### High Risks
1. **Scope Creep**: Audit reveals even more gaps than expected
   - **Mitigation**: Strict prioritization, focus on critical features first

2. **Type System Complexity**: Some methods may need Value enum extensions
   - **Mitigation**: Document pattern, create helper functions

3. **Breaking Changes**: Executor changes might break existing functionality
   - **Mitigation**: Run all 1,609 Rust tests after each change

### Medium Risks
1. **Timeline Slippage**: 2-3 days could become 1-2 weeks
   - **Mitigation**: Daily progress checks, adjust scope if needed

2. **Documentation Decay**: Docs updated but code changes again
   - **Mitigation**: Update docs in same commit as code changes

### Low Risks
1. **Performance**: Executor dispatch overhead
   - **Mitigation**: Profile after implementation, optimize if needed

---

## Rollback Plan

If critical issues arise during implementation:

**Phase 1 Rollback**: Revert executor changes, return to audit
**Phase 2 Rollback**: Stop registration, focus on fixing what's already registered
**Phase 3 Rollback**: Process changes are documentation-only, safe to skip if needed

**Commit Strategy**: Commit after each working feature to enable easy rollback

---

## Next Immediate Action

**START HERE**: Implement Fix 1.1 (Register print function)

1. Open `src/execution/executor.rs`
2. Find the `execute_call` method (around line 2979)
3. Add print handling in function dispatch
4. Create test file `tests/integration/01_print.gr`
5. Test: `cargo run --quiet tests/integration/01_print.gr`
6. Commit: "feat: register print() built-in function"

**Command to run**:
```bash
# After implementing print
cargo run --quiet tests/integration/01_print.gr
# Should output: Hello World
```

---

## Questions to Consider

1. **Should we pause new features until gaps are fixed?**
   - Recommendation: YES - Don't dig the hole deeper

2. **Should we downgrade phase completion status?**
   - Recommendation: YES - Be honest about current state

3. **Should we add .gr integration tests to all existing tests?**
   - Recommendation: YES for critical features, EVENTUALLY for all

4. **Who defines "critical features"?**
   - Recommendation: User decides, but suggest: print, basic collections, graphs

---

## Files to Create/Modify

### New Files
- [ ] `dev_docs/API_INVENTORY.md`
- [ ] `dev_docs/REGISTRATION_GAPS.md`
- [ ] `tests/integration/01_builtins.gr`
- [ ] `tests/integration/02_variables.gr`
- [ ] `tests/integration/03_functions.gr`
- [ ] `tests/integration/04_lists.gr`
- [ ] `tests/integration/05_graphs_basic.gr`
- [ ] `tests/integration/06_graphs_query.gr`
- [ ] `tests/integration/07_graphs_rules.gr`
- [ ] `tests/integration/08_behaviors.gr`
- [ ] `tests/integration/09_modules.gr`
- [ ] `tests/integration/10_pattern_matching.gr`
- [ ] `tests/integration/README.md`
- [ ] `scripts/test_integration.sh`

### Modified Files
- [ ] `src/execution/executor.rs` - Register print() and match_pattern()
- [ ] `dev_docs/LANGUAGE_SPECIFICATION.md` - func → fn
- [ ] `CLAUDE.md` - Add integration testing section
- [ ] `dev_docs/RUST_IMPLEMENTATION_ROADMAP.md` - Update success criteria
- [ ] `dev_docs/PHASE_9_ACTUAL_STATUS.md` - Revise completion status
- [ ] `.github/workflows/test.yml` - Add integration test step (if CI exists)

---

## Conclusion

This is a **fixable problem** with a clear path forward. The Rust API foundation is solid, we just need to connect it to the user-facing language through proper executor registration and integration testing.

**Key Insight**: TDD worked for internal API, but we need to extend TDD to include user-facing integration tests.

**Estimated Total Effort**: 2-3 days for critical features, 1-2 weeks for comprehensive fix.

**Blocker Status**: This should be considered a BLOCKER for marking any future phases as "complete" until the integration gap is resolved.

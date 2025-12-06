# Testing Framework Implementation Plan

**Created**: December 5, 2025
**Status**: Planning
**Priority**: High (enables better development workflow)

---

## Motivation

Currently, testing Graphoid code requires creating ad-hoc files in `/tmp` and manually running them. This is:
- Messy (files scattered everywhere)
- Not reproducible (tests not committed)
- No reporting (just "it worked" or stack traces)
- No organization (flat file execution)

The full spec in `PRODUCTION_TOOLING_SPECIFICATION.md` is comprehensive but large. This plan prioritizes a **minimal viable testing framework** that can grow incrementally.

---

## Implementation Phases

### Phase 1: Core Testing Primitives (MVP)
**Estimated effort**: 2-3 days
**Goal**: Basic `describe`/`it`/`expect` working

#### 1.1 Create `stdlib/spec.gr` module

Implement in pure Graphoid:

```graphoid
# Global test state
_spec_results = []
_current_describe = []

# describe block - just sets context name
fn describe(name, block) {
    _current_describe = _current_describe.append(name)
    block()
    _current_describe = _current_describe.slice(0, _current_describe.size() - 1)
}

# it block - runs a test
fn it(description, block) {
    full_name = _current_describe.join(" > ") + " > " + description
    try {
        block()
        _spec_results = _spec_results.append({
            "name": full_name,
            "status": "pass"
        })
        print("  ✓ " + description)
    } catch(e) {
        _spec_results = _spec_results.append({
            "name": full_name,
            "status": "fail",
            "error": e
        })
        print("  ✗ " + description)
        print("    " + e.to_string())
    }
}

# Expectation object
fn expect(actual) {
    return {
        "actual": actual,
        "to_equal": fn(expected) {
            if actual != expected {
                raise("Expected " + expected.to_string() + " but got " + actual.to_string())
            }
        },
        "to_be_true": fn() {
            if actual != true {
                raise("Expected true but got " + actual.to_string())
            }
        },
        # ... more matchers
    }
}
```

#### 1.2 Required Language Features

Check/implement if missing:
- [ ] `try`/`catch` blocks (for catching test failures)
- [ ] Returning hashes with function values (for `expect()` API)
- [ ] Anonymous functions as hash values

#### 1.3 `let` for Shared Setup

RSpec's `let` is essential for readable, DRY tests:

```graphoid
describe "Calculator" {
    let("calc", fn() { Calculator.new() })
    let("result", fn() { calc.add(2, 3) })

    it "adds numbers" {
        expect(result).to_equal(5)
    }

    describe "with negative numbers" {
        # Override in nested block
        let("result", fn() { calc.add(-2, -3) })

        it "handles negatives" {
            expect(result).to_equal(-5)
        }
    }
}
```

Key properties:
- **Lazy evaluation** - only computed when first accessed in a test
- **Memoization** - computed once per test, cached for that test
- **Scoped** - available in defining block and all nested blocks
- **Overridable** - nested contexts can redefine

Implementation approach:
- Store `let` definitions in a scoped registry
- On first access within an `it` block, evaluate and cache
- Clear cache between tests
- Look up parent scopes if not found in current

Also include `subject` (special case of `let` for the thing under test):

```graphoid
describe "List" {
    subject(fn() { [1, 2, 3] })

    it "has size" {
        expect(subject.size()).to_equal(3)
    }
}
```

#### 1.4 Basic Matchers

Start with essentials:
- `to_equal(expected)` - equality check
- `to_be_true()` / `to_be_false()` - boolean checks
- `to_be_none()` - none check
- `not_to_equal(expected)` - negation

---

### Phase 2: Test Runner
**Estimated effort**: 1-2 days
**Goal**: `graphoid spec` command works

#### 2.1 Add `spec` subcommand to CLI

In `src/main.rs`:
```rust
// graphoid spec [path]
// - If path is directory, find all *.spec.gr files
// - If path is file, run that file
// - If no path, search ./specs/ and ./tests/
```

#### 2.2 Test Discovery

- Find `*.spec.gr` files in specified paths
- Run each file
- Collect results
- Print summary

#### 2.3 Exit Codes

- Exit 0 if all tests pass
- Exit 1 if any test fails
- Enables CI integration

---

### Phase 3: Better Reporting
**Estimated effort**: 1 day
**Goal**: Clear, informative output

#### 3.1 Documentation Format

```
Calculator
  add
    ✓ adds positive numbers (0.2ms)
    ✓ adds negative numbers (0.1ms)
  divide
    ✗ handles division by zero
      Expected error to be raised
      at calculator.spec.gr:45

Specs: 2 passed, 1 failed, 3 total
Time: 0.5s
```

#### 3.2 Progress Format (for many tests)

```
...F..
1 failure
```

---

### Phase 4: Hooks
**Estimated effort**: 1 day
**Goal**: Setup/teardown support

- `before_each { }` - run before each `it`
- `after_each { }` - run after each `it`
- `before_all { }` - run once before describe block
- `after_all { }` - run once after describe block

---

### Phase 5: Additional Matchers
**Estimated effort**: 1-2 days
**Goal**: Rich expectation vocabulary

- `to_be_greater_than(n)`
- `to_be_less_than(n)`
- `to_contain(element)` - for collections
- `to_be_empty()`
- `to_have_size(n)`
- `to_match(pattern)` - regex
- `to_raise(error_type)` - exception testing
- `to_be_close_to(n, delta)` - float comparison

---

### Phase 6: Advanced Features (Future)
**Estimated effort**: Variable
**Goal**: Full-featured testing

- `shared_examples` / `it_behaves_like`
- Table-driven tests with `where` blocks
- Mocking/stubbing
- Property-based testing
- Coverage reporting
- Parallel execution
- Watch mode

---

## File Structure

```
project/
├── src/
│   └── mycode.gr
├── specs/                    # or tests/
│   ├── mycode.spec.gr
│   └── integration/
│       └── api.spec.gr
└── graphoid.toml             # Optional config
```

---

## Implementation Notes

### Option A: Pure Graphoid Implementation

Implement `spec.gr` entirely in Graphoid.

**Pros**:
- Dogfooding the language
- No Rust changes needed
- Self-hosting aligned

**Cons**:
- Need try/catch in Graphoid
- Need method-returning-function pattern
- Performance for large test suites

### Option B: Hybrid Implementation

Spec module in Graphoid, runner in Rust.

**Pros**:
- Better performance
- Easier file discovery
- Better error formatting with source locations

**Cons**:
- More Rust code to maintain
- Less dogfooding

### Recommendation

**Start with Option A** (pure Graphoid) for Phase 1. If performance or capability issues arise, add Rust support incrementally.

---

## Immediate Next Steps

1. **Check prerequisites**: Does Graphoid have try/catch? Can functions be hash values?
2. **Create `stdlib/spec.gr`** with minimal implementation
3. **Create a test spec file** to validate the framework
4. **Add `spec` subcommand** to CLI
5. **Test the testing framework** (meta!)

---

## Implementation Status

**Phase 1: COMPLETE** (December 5, 2025)

- [x] `describe`, `it`, `expect` working in `stdlib/spec.gr`
- [x] 10 matchers implemented (to_equal, to_be_true, to_be_false, to_be_none, to_be_greater_than, to_be_less_than, to_contain, to_be_empty, to_have_size, to_be_close_to)
- [x] `let` function defined (but see limitations)
- [ ] `let` memoization needs testing
- [ ] `graphoid spec` CLI command (Phase 2)

**Known Limitations**:

1. **Matcher syntax**: Must use `expect(x)["to_equal"](y)` instead of `expect(x).to_equal(y)` because Graphoid's executor doesn't support calling functions stored as hash values via method syntax.

2. **Summary counter broken**: The `_passed`/`_failed` counters don't update because module-level variables can't be modified from functions (creates local variable instead). Individual test results display correctly.

3. **Reserved keywords released**: Had to un-reserve `describe`, `it`, `expect`, etc. from the lexer to allow them as function names.

**Files created**:
- `stdlib/spec.gr` - The testing framework module
- `specs/spec_framework.spec.gr` - Tests for the framework itself
- `specs/simple_test.spec.gr` - Simple example test

---

## Success Criteria for MVP

- [x] Can write `.spec.gr` files with `describe`/`it`/`expect`
- [ ] `let` works with lazy evaluation and memoization
- [ ] `let` definitions are properly scoped and overridable
- [ ] Running `graphoid spec` finds and runs tests
- [x] Pass/fail clearly reported (individual tests)
- [ ] Exit code reflects test results
- [x] At least 5 matchers working (10 implemented!)

---

## References

- Full spec: `dev_docs/PRODUCTION_TOOLING_SPECIFICATION.md`
- RSpec comparison: `dev_docs/archive/sessions/2025-01-spec-review/TESTING_FRAMEWORK_COMPARISON.md`

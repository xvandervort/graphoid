# Implicit Self in Blocks - TDD Implementation Plan

**Goal**: When a block is called from within a method, the block has access to the method's `self`, enabling DSL-style syntax.

**Target Syntax**:
```graphoid
runner.describe("Calculator") {
    it("adds") {           # Resolves to self.it() where self=runner
        expect(2+2).to_equal(4)
    }
}
```

---

## Design Decision: Automatic vs Opt-In

### Option 1: Automatic (Recommended)
When any closure is called from within a method, `self` is automatically available in the closure.

**Pros**: Clean syntax, no boilerplate
**Cons**: Could be surprising if unexpected

### Option 2: Opt-In via `eval_with`
Methods explicitly pass context: `block.eval_with(self)`

**Pros**: Explicit, predictable
**Cons**: Requires method authors to remember

**Decision**: **Option 1 (Automatic)** - aligns with Ruby's behavior and enables cleanest DSL syntax.

---

## Detailed Semantics

### Rule 1: Block Inherits Caller's `self`

When a **closure** (anonymous function) is called from within a **method context**, the method's `self` is bound in the closure's execution environment.

```graphoid
graph Runner {
    fn run(block) {
        # self = the Runner instance
        block()  # Inside block, self = this Runner instance
    }
}

r = Runner {}
r.run() {
    print(self)  # Prints the Runner instance
}
```

### Rule 2: Implicit Method Resolution

When a name is not found in local scope, check if `self` has a method with that name.

```graphoid
graph Runner {
    fn greet() {
        print("Hello!")
    }

    fn run(block) {
        block()
    }
}

r = Runner {}
r.run() {
    greet()  # Resolves to self.greet() -> prints "Hello!"
}
```

### Rule 3: Nested Blocks Propagate `self`

When a method calls a block, and that block calls another block from a method, `self` propagates through the chain.

```graphoid
graph TestRunner {
    fn describe(name, block) {
        print("describe: " + name)
        block()
    }

    fn it(name, block) {
        print("  it: " + name)
        result = block()
        # ... record result
    }
}

runner = TestRunner {}
runner.describe("Math") {
    it("adds") {        # self.it() where self=runner
        return 2+2 == 4
    }
}
```

### Rule 4: Local Variables Shadow `self` Properties

If a local variable is defined with the same name as a `self` method/property, the local takes precedence.

```graphoid
graph Runner {
    fn foo() { return "from self" }

    fn run(block) {
        block()
    }
}

r = Runner {}
r.run() {
    foo = "local"
    print(foo)      # Prints "local" (local variable)
    print(self.foo())  # Prints "from self" (explicit self)
}
```

### Rule 5: Named Functions Do NOT Inherit `self`

Only anonymous closures (blocks) inherit `self`. Named functions maintain their own context.

```graphoid
fn helper() {
    print(self)  # ERROR: self not defined (unless called as method)
}

graph Runner {
    fn run(block) {
        block()   # Block gets self
        helper()  # helper does NOT get self (it's a named function)
    }
}
```

---

## Test-Driven Development Plan

### Phase 1: Basic Self Propagation (RED → GREEN)

#### Test 1.1: Block Receives Caller's Self
```graphoid
# test_implicit_self_basic.gr
graph Greeter {
    name: "World"

    fn run(block) {
        block()
    }

    fn get_name() {
        return name
    }
}

g = Greeter { name: "Test" }
result = none

g.run() {
    result = self.get_name()
}

assert(result == "Test", "Block should have access to caller's self")
print("✓ Test 1.1 passed")
```

#### Test 1.2: Self Available in Nested Calls
```graphoid
# test_implicit_self_nested.gr
graph Outer {
    value: 42

    fn outer_run(block) {
        block()
    }

    fn inner_run(block) {
        block()
    }

    fn get_value() {
        return value
    }
}

o = Outer {}
result = none

o.outer_run() {
    inner_run() {    # self.inner_run()
        result = get_value()  # self.get_value()
    }
}

assert(result == 42, "Nested blocks should propagate self")
print("✓ Test 1.2 passed")
```

### Phase 2: Implicit Method Resolution (RED → GREEN)

#### Test 2.1: Method Name Resolves to Self
```graphoid
# test_implicit_method_resolution.gr
graph Calculator {
    fn run(block) {
        block()
    }

    fn add(a, b) {
        return a + b
    }
}

calc = Calculator {}
result = none

calc.run() {
    result = add(2, 3)  # Should resolve to self.add(2, 3)
}

assert(result == 5, "add() should resolve to self.add()")
print("✓ Test 2.1 passed")
```

#### Test 2.2: Property Access Resolves to Self
```graphoid
# test_implicit_property_resolution.gr
graph Config {
    debug: true

    fn run(block) {
        block()
    }

    fn is_debug() {
        return debug
    }
}

c = Config {}
result = none

c.run() {
    result = is_debug()  # Should resolve to self.is_debug()
}

assert(result == true, "is_debug() should resolve to self.is_debug()")
print("✓ Test 2.2 passed")
```

### Phase 3: gspec Pattern (RED → GREEN)

#### Test 3.1: Full describe/it Pattern
```graphoid
# test_gspec_pattern.gr
graph TestRunner {
    passed: 0
    failed: 0

    fn describe(name, block) {
        print("describe " + name)
        block()
    }

    fn context(name, block) {
        print("  context " + name)
        block()
    }

    fn it(name, block) {
        result = block()
        if result == true {
            passed = passed + 1
            print("    ✓ " + name)
        } else {
            failed = failed + 1
            print("    ✗ " + name)
        }
    }

    fn get_passed() { return passed }
    fn get_failed() { return failed }
}

runner = TestRunner {}

runner.describe("Calculator") {
    context("addition") {
        it("adds positive numbers") {
            return 2 + 3 == 5
        }

        it("adds negative numbers") {
            return -2 + -3 == -5
        }
    }

    context("comparison") {
        it("compares correctly") {
            return 10 > 5
        }

        it("fails intentionally") {
            return 1 == 2
        }
    }
}

assert(runner.get_passed() == 3, "Should have 3 passed tests")
assert(runner.get_failed() == 1, "Should have 1 failed test")
print("✓ Test 3.1 passed: gspec pattern works!")
```

### Phase 4: Edge Cases (RED → GREEN)

#### Test 4.1: Local Variable Shadows Self Method
```graphoid
# test_local_shadows_self.gr
graph Runner {
    fn run(block) {
        block()
    }

    fn foo() {
        return "from self"
    }
}

r = Runner {}
result = none

r.run() {
    foo = "local variable"
    result = foo  # Should be "local variable", not self.foo()
}

assert(result == "local variable", "Local should shadow self method")
print("✓ Test 4.1 passed")
```

#### Test 4.2: Explicit Self Still Works
```graphoid
# test_explicit_self.gr
graph Runner {
    value: 100

    fn run(block) {
        block()
    }

    fn get_value() {
        return value
    }
}

r = Runner {}
result = none

r.run() {
    result = self.get_value()  # Explicit self should work
}

assert(result == 100, "Explicit self.method() should work")
print("✓ Test 4.2 passed")
```

#### Test 4.3: Named Function Does Not Inherit Self
```graphoid
# test_named_function_no_self.gr
graph Runner {
    fn run(block) {
        block()
    }
}

fn standalone() {
    # self should not be defined here when called normally
    return "standalone"
}

r = Runner {}
result = none

r.run() {
    result = standalone()  # Call named function, it should NOT have self
}

assert(result == "standalone", "Named function should not inherit self")
print("✓ Test 4.3 passed")
```

#### Test 4.4: Self Not Available Outside Method Context
```graphoid
# test_no_self_outside_method.gr
# At module level, self should not be defined

has_self = false
try {
    x = self  # Should error
    has_self = true
} catch e {
    has_self = false
}

assert(has_self == false, "self should not exist at module level")
print("✓ Test 4.4 passed")
```

### Phase 5: Interaction with Existing Features (RED → GREEN)

#### Test 5.1: Works with CLG Properties
```graphoid
# test_with_clg_properties.gr
graph Counter {
    configure { readable: :count }
    count: 0

    fn run(block) {
        block()
    }

    fn increment() {
        count = count + 1
    }
}

c = Counter {}

c.run() {
    increment()
    increment()
    increment()
}

assert(c.count() == 3, "CLG properties should work with implicit self")
print("✓ Test 5.1 passed")
```

#### Test 5.2: Works with Return Self Pattern
```graphoid
# test_return_self_chaining.gr
graph Builder {
    parts: []

    fn run(block) {
        block()
        return self
    }

    fn add(part) {
        parts.append!(part)
        return self
    }

    fn get_parts() {
        return parts
    }
}

b = Builder {}

b.run() {
    add("header")
    add("body")
    add("footer")
}

assert(b.get_parts().length() == 3, "Builder pattern should work")
print("✓ Test 5.2 passed")
```

---

## Implementation Plan

### Step 1: Track Method Context in Executor

**File**: `src/execution/executor.rs`

Add tracking for the current method's `self`:

```rust
pub struct Executor {
    // ... existing fields ...

    /// Stack of `self` values for method contexts
    /// When a method is called, push its receiver
    /// When a method returns, pop
    method_self_stack: Vec<Value>,
}
```

### Step 2: Push/Pop Self on Method Calls

When calling a method on a graph:

```rust
fn call_method_on_graph(&mut self, graph: &Graph, method_name: &str, args: &[Value]) -> Result<Value> {
    // Push self onto stack
    self.method_self_stack.push(Value::graph(graph.clone()));

    // ... existing method call logic ...

    // Pop self from stack
    self.method_self_stack.pop();

    result
}
```

### Step 3: Inject Self When Calling Closures

When calling a Function that is anonymous (block):

```rust
fn call_function(&mut self, func: &Function, args: &[Value]) -> Result<Value> {
    // ... existing setup ...

    // If this is an anonymous closure AND we're in a method context,
    // inject the current self into the closure's environment
    if func.name.is_none() && !self.method_self_stack.is_empty() {
        let current_self = self.method_self_stack.last().unwrap().clone();
        self.env.define("self".to_string(), current_self);
    }

    // ... rest of call_function ...
}
```

### Step 4: Implicit Method Resolution

When resolving a variable that's not found locally, check if `self` has a method with that name:

```rust
fn resolve_variable(&self, name: &str) -> Result<Value> {
    // 1. Check local scope
    if let Ok(value) = self.env.get(name) {
        return Ok(value);
    }

    // 2. Check if self has a method with this name (NEW)
    if let Ok(self_value) = self.env.get("self") {
        if let ValueKind::Graph(ref graph) = self_value.kind {
            if graph.has_method(name) {
                // Return a bound method or marker for implicit call
                return Ok(/* bound method value */);
            }
        }
    }

    // 3. Not found
    Err(GraphoidError::undefined_variable(name))
}
```

### Step 5: Handle Implicit Method Calls

When evaluating a function call where the function name resolves to a method on `self`:

```rust
// In eval_call or similar:
if resolved_to_self_method {
    let self_value = self.env.get("self")?;
    return self.call_method_on_value(&self_value, method_name, args);
}
```

---

## Implementation Order (TDD)

1. **Write Test 1.1** → Run (FAIL) → Implement Step 1-3 → Run (PASS)
2. **Write Test 1.2** → Run (FAIL) → Verify propagation → Run (PASS)
3. **Write Test 2.1** → Run (FAIL) → Implement Step 4-5 → Run (PASS)
4. **Write Test 2.2** → Run (PASS, should work with 2.1)
5. **Write Test 3.1** → Run (should PASS if all above works)
6. **Write Tests 4.1-4.4** → Run → Fix edge cases
7. **Write Tests 5.1-5.2** → Run → Verify integration

---

## Files to Modify

| File | Changes |
|------|---------|
| `src/execution/executor.rs` | Add `method_self_stack`, modify `call_function`, modify variable resolution |
| `src/values/mod.rs` | Possibly add bound method type (or use existing Function) |
| `tests/integration/` | Add test files |

---

## Risks and Mitigations

### Risk 1: Breaking Existing CLG Code

**Concern**: Existing methods already have implicit `self` for property access.

**Mitigation**: This change ADDS `self` to blocks, doesn't change how methods work. Existing code should be unaffected.

### Risk 2: Performance

**Concern**: Extra stack operations on every method call.

**Mitigation**: Stack push/pop is O(1). Negligible overhead.

### Risk 3: Confusing Semantics

**Concern**: Users might not expect blocks to have access to caller's `self`.

**Mitigation**: Document clearly. This matches Ruby behavior which is well-understood.

### Risk 4: Name Conflicts

**Concern**: A local variable might accidentally shadow a self method.

**Mitigation**: This is the expected behavior (Rule 4). Explicit `self.method()` always works.

---

## Success Criteria

- [ ] All 10+ test cases pass
- [ ] gspec pattern works without `|r|` parameter passing
- [ ] Existing samples still work (run full test suite)
- [ ] No performance regression
- [ ] Backwards compatibility verified
- [ ] Sample file created and working
- [ ] User documentation written

---

## Phase 6: Backwards Compatibility Verification (CRITICAL)

Before merging, run comprehensive compatibility checks:

### Step 6.1: Run All Existing Tests

```bash
~/.cargo/bin/cargo test --lib
```

All 1,175+ tests must pass.

### Step 6.2: Run All Sample Files

```bash
for f in samples/**/*.gr; do
    echo "Testing: $f"
    ~/.cargo/bin/cargo run --quiet "$f" || echo "FAILED: $f"
done
```

All 30 sample files must produce expected output.

### Step 6.3: Specific Risk Areas to Verify

| Risk Area | Test | Expected Behavior |
|-----------|------|-------------------|
| CLG methods | `samples/03-advanced/class_like_graphs.gr` | Implicit `self` in methods unchanged |
| Configure blocks | `samples/02-intermediate/integer_mode.gr` | Configure semantics unchanged |
| Lambda transforms | `samples/01-basics/collections.gr` | `.map()`, `.filter()` work as before |
| Graph methods | `samples/06-projects/social/social_network.gr` | All methods work correctly |
| Elevator simulation | `samples/06-projects/elevator/elevator.gr` | Full simulation runs |

### Step 6.4: Edge Case Verification

Create specific tests to verify NO unintended behavior:

```graphoid
# test_backwards_compat.gr

# 1. Regular lambdas should NOT get self injected
numbers = [1, 2, 3]
doubled = numbers.map(x => x * 2)
assert(doubled == [2, 4, 6], "Lambda transforms must work")

# 2. Named functions should NOT get self
fn standalone_func() {
    # self should not be defined here
    return "ok"
}
result = standalone_func()
assert(result == "ok", "Standalone functions must work")

# 3. CLG methods should work as before
graph Counter {
    count: 0
    fn increment() {
        count = count + 1  # Implicit self.count
    }
    fn get() { return count }
}
c = Counter {}
c.increment()
c.increment()
assert(c.get() == 2, "CLG implicit self must work")

# 4. For loops should NOT be affected
total = 0
for i in [1, 2, 3] {
    total = total + i
}
assert(total == 6, "For loops must work")

print("✓ All backwards compatibility tests passed")
```

---

## Phase 7: Sample File (REQUIRED)

Create `samples/03-advanced/implicit_self_blocks.gr`:

```graphoid
# implicit_self_blocks.gr
# Demonstrates implicit self in blocks - enabling DSL-style syntax
#
# When a block is passed to a method and called, the block automatically
# has access to the method's `self`. This enables clean, Ruby-style DSLs.

print("=== Implicit Self in Blocks ===\n")

# -----------------------------------------------------------------------------
# Example 1: Simple DSL Pattern
# -----------------------------------------------------------------------------

graph HtmlBuilder {
    _output: ""

    fn build(block) {
        _output = ""
        block()
        return _output
    }

    fn div(content) {
        _output = _output + "<div>" + content + "</div>\n"
    }

    fn p(content) {
        _output = _output + "<p>" + content + "</p>\n"
    }

    fn span(content) {
        _output = _output + "<span>" + content + "</span>\n"
    }
}

html = HtmlBuilder {}

result = html.build() {
    div("Hello World")      # Calls self.div() - no explicit self needed!
    p("This is a paragraph")
    span("Inline text")
}

print("HTML Output:")
print(result)

# -----------------------------------------------------------------------------
# Example 2: Configuration DSL
# -----------------------------------------------------------------------------

graph ServerConfig {
    _host: "localhost"
    _port: 8080
    _debug: false

    fn configure(block) {
        block()
        return self
    }

    fn host(value) {
        _host = value
    }

    fn port(value) {
        _port = value
    }

    fn debug(value) {
        _debug = value
    }

    fn summary() {
        return "Server at " + _host + ":" + _port.to_string() + " (debug=" + _debug.to_string() + ")"
    }
}

server = ServerConfig {}

server.configure() {
    host("api.example.com")  # self.host()
    port(443)                # self.port()
    debug(true)              # self.debug()
}

print("\nServer Config:")
print(server.summary())

# -----------------------------------------------------------------------------
# Example 3: Test Framework Pattern (gspec-style)
# -----------------------------------------------------------------------------

graph TestRunner {
    _passed: 0
    _failed: 0

    fn describe(name, block) {
        print("\ndescribe " + name)
        block()
    }

    fn context(name, block) {
        print("  context " + name)
        block()
    }

    fn it(name, block) {
        result = block()
        if result == true {
            _passed = _passed + 1
            print("    ✓ " + name)
        } else {
            _failed = _failed + 1
            print("    ✗ " + name)
        }
    }

    fn summary() {
        print("\n" + _passed.to_string() + " passed, " + _failed.to_string() + " failed")
    }
}

runner = TestRunner {}

runner.describe("Calculator") {
    context("addition") {
        it("adds positive numbers") {
            return 2 + 3 == 5
        }

        it("adds negative numbers") {
            return -2 + -3 == -5
        }
    }

    context("multiplication") {
        it("multiplies by zero") {
            return 5 * 0 == 0
        }

        it("multiplies negatives") {
            return -3 * -4 == 12
        }
    }
}

runner.summary()

# -----------------------------------------------------------------------------
# Example 4: Nested Blocks
# -----------------------------------------------------------------------------

graph Menu {
    _items: []

    fn menu(name, block) {
        print("\n=== " + name + " ===")
        block()
    }

    fn section(name, block) {
        print("  [" + name + "]")
        block()
    }

    fn item(name, price) {
        print("    " + name + " - $" + price.to_string())
        _items.append!({ name: name, price: price })
    }

    fn total() {
        sum = 0
        for i in _items {
            sum = sum + i["price"]
        }
        return sum
    }
}

cafe = Menu {}

cafe.menu("Coffee Shop") {
    section("Hot Drinks") {
        item("Espresso", 3.50)
        item("Latte", 4.50)
        item("Cappuccino", 4.00)
    }

    section("Cold Drinks") {
        item("Iced Coffee", 3.75)
        item("Cold Brew", 4.25)
    }
}

print("\nTotal menu value: $" + cafe.total().to_string())

print("\n=== All Examples Complete ===")
```

---

## Phase 8: User Documentation (REQUIRED)

Create `docs/user-guide/implicit-self-blocks.md`:

```markdown
# Implicit Self in Blocks

Graphoid supports **implicit self** in blocks, enabling clean DSL-style syntax
similar to Ruby. When you pass a block to a method, the block automatically
has access to the method's `self`, allowing you to call methods without
explicit qualification.

## Basic Example

```graphoid
graph Greeter {
    fn run(block) {
        block()
    }

    fn hello() {
        print("Hello!")
    }

    fn goodbye() {
        print("Goodbye!")
    }
}

g = Greeter {}

g.run() {
    hello()    # Calls self.hello() - no explicit self needed!
    goodbye()  # Calls self.goodbye()
}
```

Output:
```
Hello!
Goodbye!
```

## How It Works

When a **block** (anonymous closure using `{ }` syntax) is called from within
a **method**, Graphoid automatically makes the method's `self` available inside
the block. This means:

1. You can call methods on `self` without writing `self.method()`
2. The behavior propagates through nested blocks
3. Local variables still take precedence over `self` methods

## Use Cases

### DSL-Style Configuration

```graphoid
graph Config {
    fn configure(block) {
        block()
        return self
    }

    fn host(value) { _host = value }
    fn port(value) { _port = value }
}

server = Config {}
server.configure() {
    host("localhost")  # Clean syntax!
    port(8080)
}
```

### Builder Pattern

```graphoid
graph Html {
    fn build(block) {
        block()
    }

    fn div(content) { ... }
    fn p(content) { ... }
}

html = Html {}
html.build() {
    div("Header")
    p("Content")
}
```

### Testing Frameworks

```graphoid
runner.describe("Feature") {
    it("works") {
        return expect(result).to_equal(expected)
    }
}
```

## Rules

### 1. Only Blocks, Not Named Functions

Implicit self only applies to **anonymous blocks** passed to methods, not to
named functions:

```graphoid
fn standalone() {
    hello()  # ERROR: hello not defined (no implicit self)
}

g.run() {
    hello()  # OK: implicit self.hello()
}
```

### 2. Local Variables Take Precedence

If you define a local variable with the same name as a method, the local wins:

```graphoid
g.run() {
    hello = "local"
    print(hello)        # Prints "local" (the variable)
    print(self.hello()) # Calls the method explicitly
}
```

### 3. Explicit Self Always Works

You can always use `self.method()` for clarity:

```graphoid
g.run() {
    self.hello()  # Explicit - always works
    hello()       # Implicit - also works
}
```

### 4. Nested Blocks Propagate Self

When blocks call other blocks via methods, `self` propagates:

```graphoid
runner.describe("Outer") {
    context("Inner") {       # self.context()
        it("test") {         # self.it()
            return true
        }
    }
}
```

## When NOT to Use

- **Pure transformations**: Use lambdas for `.map()`, `.filter()`, etc.
- **Standalone utilities**: Named functions don't get implicit self
- **When clarity matters**: Use explicit `self.method()` if the code is confusing

## See Also

- [Class-Like Graphs](class-like-graphs.md) - How `self` works in CLG methods
- [Testing with gspec](gspec.md) - Full testing framework documentation
```

---

## Timeline Estimate

| Phase | Time |
|-------|------|
| Test writing (Phases 1-5) | 1 hour |
| Implementation (Steps 1-5) | 2-3 hours |
| Edge case fixes | 1-2 hours |
| Backwards compatibility (Phase 6) | 1 hour |
| Sample file (Phase 7) | 30 min |
| User documentation (Phase 8) | 1 hour |

**Total**: ~1.5 days

---

## Approval Checklist

- [ ] Design reviewed
- [ ] TDD test cases defined
- [ ] Implementation steps clear
- [ ] Risks understood
- [ ] Ready for implementation

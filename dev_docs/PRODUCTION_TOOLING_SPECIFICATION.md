# Production Tooling Specification

**Version**: 1.0
**Created**: January 2025
**Status**: Specification for production-ready Graphoid/Glang tooling

This document specifies the essential production tooling required for Graphoid to be a mature, professional programming language.

---

## 1. Testing Framework

### Philosophy

- **Built into the language** - Not a separate library
- **Behavior-oriented** - Describe what code should do, not implementation
- **Self-documenting** - Tests read like specifications
- **Fast feedback** - Tests run in milliseconds
- **Expressive expectations** - Natural language assertions

### Test File Convention

Test files use `.spec.gr` extension (spec = specification):

```
myproject/
├── src/
│   ├── calculator.gr
│   └── utils.gr
└── specs/
    ├── calculator.spec.gr
    └── utils.spec.gr
```

### Basic Test Syntax (RSpec-Style)

```glang
# In calculator.spec.gr
import "spec"
import "../src/calculator"

describe "Calculator" {

    describe "add" {
        it "adds two positive numbers" {
            result = calculator.add(2, 3)
            expect(result).to_equal(5)
        }

        it "adds negative numbers" {
            result = calculator.add(-2, -3)
            expect(result).to_equal(-5)
        }
    }

    describe "divide" {
        context "when dividing by zero" {
            it "raises an error" {
                expect(func() {
                    calculator.divide(10, 0)
                }).to_raise("RuntimeError")
            }
        }

        context "when dividing valid numbers" {
            it "returns the quotient" {
                result = calculator.divide(10, 2)
                expect(result).to_equal(5)
            }
        }
    }
}
```

### Expectation API (RSpec-Style)

**Built-in `spec` module** provides behavior-driven testing:

```glang
import "spec"

# Basic structure
describe "Feature" {
    it "does something" {
        expect(actual).to_equal(expected)
    }
}

# Equality matchers
expect(value).to_equal(expected)
expect(value).not_to_equal(expected)
expect(value).to_be(expected)        # Identity check
expect(value).not_to_be(expected)

# Truthiness matchers
expect(value).to_be_true()
expect(value).to_be_false()
expect(value).to_be_truthy()
expect(value).to_be_falsy()
expect(value).to_be_none()

# Comparison matchers
expect(value).to_be_greater_than(threshold)
expect(value).to_be_less_than(threshold)
expect(value).to_be_at_least(threshold)    # >=
expect(value).to_be_at_most(threshold)     # <=
expect(value).to_be_between(min, max)

# Type matchers
expect(value).to_be_a("num")        # Type check
expect(value).to_be_a_num()
expect(value).to_be_a_string()
expect(value).to_be_a_list()
expect(value).to_be_a_map()

# Collection matchers
expect(collection).to_contain(element)
expect(collection).not_to_contain(element)
expect(collection).to_be_empty()
expect(collection).not_to_be_empty()
expect(collection).to_have_size(expected_size)
expect(list).to_include(item1, item2, item3)  # Multiple items

# Approximate equality (for floats)
expect(3.14159).to_be_close_to(3.14, 0.01)
expect(value).to_be_within(0.001).of(expected)

# Exception matchers
expect(func() { risky_operation() }).to_raise("RuntimeError")
expect(func() { risky_operation() }).to_raise_error()  # Any error
expect(func() { safe_operation() }).not_to_raise()

# Deep equality (nested structures)
expect(nested_structure).to_deeply_equal(expected)

# Regex matchers
expect(text).to_match(/pattern/)
expect(text).not_to_match(/pattern/)

# Custom matchers
expect(value).to_satisfy(func(v) { return v > 0 and v < 100 })

# Negation (works with any matcher)
expect(value).not_to_equal(unwanted)
```

### Test Organization

#### Setup and Teardown (Hooks)

```glang
describe "Calculator" {

    # Hooks - run before/after tests
    before_all {
        # Run once before all tests in this describe block
        print("Setting up calculator tests")
    }

    after_all {
        # Run once after all tests in this describe block
        print("Tearing down calculator tests")
    }

    before_each {
        # Run before EACH test in this block
        calculator = Calculator.new()
    }

    after_each {
        # Run after EACH test in this block
        calculator.cleanup()
    }

    it "performs basic arithmetic" {
        expect(calculator.add(2, 3)).to_equal(5)
    }
}
```

#### Nested Describe Blocks

```glang
describe "Calculator" {

    describe "arithmetic operations" {

        describe "add" {
            it "adds positive numbers" {
                expect(calc.add(2, 3)).to_equal(5)
            }

            it "adds negative numbers" {
                expect(calc.add(-2, -3)).to_equal(-5)
            }
        }

        describe "subtract" {
            it "subtracts positive numbers" {
                expect(calc.subtract(5, 3)).to_equal(2)
            }
        }
    }

    describe "edge cases" {
        context "when dividing by zero" {
            it "raises an error" {
                expect(func() {
                    calc.divide(10, 0)
                }).to_raise("RuntimeError")
            }
        }

        context "when handling large numbers" {
            it "detects overflow" {
                huge = 10.pow(308)
                result = calc.add(huge, huge)
                expect(result.is_infinite()).to_be_true()
            }
        }
    }
}
```

#### Context Blocks for Scenarios

```glang
describe "User authentication" {

    context "when user exists" {
        before_each {
            user = create_user("alice", "password123")
        }

        it "authenticates with correct password" {
            result = auth.login("alice", "password123")
            expect(result).to_be_truthy()
        }

        it "rejects incorrect password" {
            result = auth.login("alice", "wrongpassword")
            expect(result).to_be_falsy()
        }
    }

    context "when user does not exist" {
        it "returns error" {
            expect(func() {
                auth.login("nonexistent", "password")
            }).to_raise("UserNotFoundError")
        }
    }
}
```

### Running Tests

#### Command Line

```bash
# Run all specs
graphoid spec

# Run specific file
graphoid spec specs/calculator.spec.gr

# Run specific describe block
graphoid spec specs/calculator.spec.gr --filter "Calculator::add"

# Run with verbose output
graphoid spec --verbose

# Run with coverage report
graphoid spec --coverage

# Run specs matching pattern
graphoid spec --pattern "*authentication*"

# Run and watch for changes
graphoid spec --watch

# Run specs in parallel
graphoid spec --parallel

# Format options
graphoid spec --format documentation  # Detailed, hierarchical
graphoid spec --format progress       # Dots (default)
graphoid spec --format json           # JSON output
```

#### Output Format (Documentation Style)

```
Calculator
  arithmetic operations
    add
      ✓ adds positive numbers (0.2ms)
      ✓ adds negative numbers (0.1ms)
    subtract
      ✓ subtracts positive numbers (0.1ms)
  edge cases
    when dividing by zero
      ✗ raises an error
        Expected RuntimeError to be raised
        But got no error
        at line 45 in specs/calculator.spec.gr
    when handling large numbers
      ✓ detects overflow (0.3ms)

User authentication
  when user exists
    ✓ authenticates with correct password (1.2ms)
    ✓ rejects incorrect password (1.1ms)
  when user does not exist
    ✓ returns error (0.5ms)

Specs: 6 passed, 1 failed, 7 total
Time: 3.5s
Coverage: 87%
```

#### Output Format (Progress Style)

```
.......F...

Failures:

1) Calculator edge cases when dividing by zero raises an error
   Expected RuntimeError to be raised
   But got no error
   at line 45 in specs/calculator.spec.gr

Specs: 6 passed, 1 failed, 7 total
Time: 3.5s
```

### Test Configuration

**In `graphoid.toml` (project file)**:

```toml
[spec]
# Spec file patterns
patterns = ["**/*.spec.gr", "**/*_spec.gr"]

# Directories to search
paths = ["specs/", "src/"]

# Parallel execution
parallel = true
max_workers = 4

# Coverage settings
coverage = true
coverage_threshold = 80  # Fail if below 80%
coverage_exclude = ["samples/", "examples/"]

# Timeout per spec (milliseconds)
timeout = 5000

# Output format
format = "documentation"  # "documentation", "progress", "json", "junit"

# Color output
color = true

# Fail fast (stop on first failure)
fail_fast = false

# Random order (find order dependencies)
random_order = false
```

### Advanced Features

#### Shared Examples (Reusable Behavior)

```glang
# Define shared behavior
shared_examples "a collection" {
    it "has a size method" {
        expect(collection.size).to_be_a("num")
    }

    it "can be empty" {
        collection.clear()
        expect(collection).to_be_empty()
    }
}

# Use shared examples
describe "List" {
    before_each {
        collection = [1, 2, 3]
    }

    it_behaves_like "a collection"

    it "supports indexing" {
        expect(collection[0]).to_equal(1)
    }
}

describe "Map" {
    before_each {
        collection = {"a": 1, "b": 2}
    }

    it_behaves_like "a collection"

    it "supports key access" {
        expect(collection["a"]).to_equal(1)
    }
}
```

#### Parameterized Specs (Table-Driven)

```glang
describe "square function" {
    test_cases = [
        { input: 2, expected: 4 },
        { input: 3, expected: 9 },
        { input: 4, expected: 16 },
        { input: 5, expected: 25 },
    ]

    for case in test_cases {
        it "squares " + case.input.to_string() {
            result = square(case.input)
            expect(result).to_equal(case.expected)
        }
    }
}

# Or use where block (cleaner syntax)
describe "division" {
    where {
        dividend | divisor | expected
        10       | 2       | 5
        20       | 4       | 5
        100      | 10      | 10
        7        | 2       | 3.5
    }

    it "divides {dividend} by {divisor}" {
        result = divide(dividend, divisor)
        expect(result).to_equal(expected)
    }
}
```

#### Mocking and Stubbing

```glang
describe "API client" {

    it "fetches user data" {
        # Stub HTTP response
        http_mock = stub("http.get")
        http_mock.returns({
            "status": 200,
            "body": '{"name": "Alice", "age": 30}'
        })

        # Test code
        user = api.fetch_user(123)

        # Expectations
        expect(user.name).to_equal("Alice")
        expect(user.age).to_equal(30)

        # Verify stub was called correctly
        expect(http_mock).to_have_been_called()
        expect(http_mock).to_have_been_called_with("https://api.example.com/users/123")
        expect(http_mock).to_have_been_called_once()
    }

    it "handles errors" {
        # Stub to raise error
        http_mock = stub("http.get")
        http_mock.raises("NetworkError")

        # Test error handling
        expect(func() {
            api.fetch_user(123)
        }).to_raise("NetworkError")
    }
}

# Spy (track calls but keep original behavior)
describe "Logger" {
    it "logs messages" {
        log_spy = spy("logger.info")

        my_function()

        expect(log_spy).to_have_been_called()
        expect(log_spy).to_have_been_called_with("Operation completed")
    }
}

# Partial stub (some methods stubbed, others real)
describe "Database" {
    it "queries with cache" {
        db_double = double("database")
        allow(db_double).to_receive("query").and_return([1, 2, 3])
        allow(db_double).to_receive("close").and_call_through()

        results = cache.get_with_db(db_double)

        expect(results).to_equal([1, 2, 3])
        expect(db_double).to_have_received("query")
    }
}
```

#### Property-Based Testing

```glang
import "property"

# Test properties that should ALWAYS be true

property.test("addition is commutative", func() {
    a = property.random_num()
    b = property.random_num()
    assert.equal(a + b, b + a)
})

property.test("reversing twice gives original", func() {
    list = property.random_list(property.random_string, 10)
    assert.deep_equal(list.reverse().reverse(), list)
})
```

---

## 2. Debugger

### Philosophy

- **Interactive debugging** - Pause and inspect
- **REPL integration** - Debug from REPL
- **Graph visualization** - See graph structures
- **Time travel** - Step backwards through execution

### Basic Debugging

#### Breakpoints

```glang
# Set breakpoint with debug.break()

func fibonacci(n) {
    if n <= 1 {
        return n
    }

    debug.break()  # Execution pauses here

    return fibonacci(n - 1) + fibonacci(n - 2)
}
```

#### Conditional Breakpoints

```glang
func process_items(items) {
    for item in items {
        # Only break when condition is true
        debug.break_if(item.value < 0, "Negative value found")

        process(item)
    }
}
```

### Debug Commands

When paused at a breakpoint:

```
> continue (c)     - Continue execution
> step (s)         - Step to next line
> step_into (si)   - Step into function
> step_out (so)    - Step out of function
> next (n)         - Skip to next statement
>
> print <expr>     - Evaluate and print expression
> locals           - Show local variables
> globals          - Show global variables
> stack            - Show call stack
>
> watch <expr>     - Watch expression value
> unwatch <expr>   - Stop watching
> watches          - Show all watches
>
> breakpoints      - List all breakpoints
> break <line>     - Set breakpoint at line
> delete <id>      - Delete breakpoint
>
> graph <var>      - Visualize graph structure
> quit (q)         - Stop debugging
```

### Debug Module API

```glang
import "debug"

# Execution control
debug.break()                    # Pause execution
debug.break_if(condition, msg?)  # Conditional break
debug.trace()                    # Print stack trace

# Variable inspection
vars = debug.locals()            # Get local variables
globs = debug.globals()          # Get global variables
stack = debug.stack_trace()      # Get call stack

# Performance profiling
debug.start_profile()
expensive_operation()
profile = debug.stop_profile()
print(profile.report())

# Memory inspection
mem = debug.memory_usage()
print("Memory: " + mem.to_string() + " MB")

# Graph introspection
nodes = debug.graph_nodes(my_graph)
edges = debug.graph_edges(my_graph)
viz = debug.visualize_graph(my_graph, format: "dot")
```

### IDE Integration

#### VSCode Extension Features

1. **Visual breakpoints** - Click in gutter to set
2. **Variable inspection** - Hover to see values
3. **Watch panel** - Monitor expressions
4. **Call stack panel** - Navigate stack frames
5. **Debug console** - Evaluate expressions
6. **Graph visualizer** - Interactive graph view

#### Debug Adapter Protocol (DAP)

Graphoid implements DAP for editor integration:

```bash
# Start debug server
graphoid debug-server --port 5678

# VSCode connects to server
# Set breakpoints, inspect variables, etc.
```

### Advanced Debugging

#### Time-Travel Debugging

```glang
# Record execution for replay

debug.start_recording()

# Run code
for i in [1, 2, 3, 4, 5] {
    process(i)
}

recording = debug.stop_recording()

# Replay execution
debug.replay(recording)

# Step backwards
debug.step_back()  # Undo last step

# Jump to specific point
debug.jump_to(line: 42)
```

#### Post-Mortem Debugging

```glang
# Automatic dump on crash

configure { debug: true, crash_dump: "crash.dump" } {
    # If crash occurs, state is saved
    risky_operation()
}

# Later, load crash dump
graphoid debug crash.dump

# Inspect state at crash point
> locals
> stack
> print problem_variable
```

### Performance Profiling

```glang
import "profile"

# Profile function execution
prof = profile.run(func() {
    expensive_computation()
})

print("Total time: " + prof.total_time.to_string() + "ms")
print("Function calls:")
for call in prof.calls {
    print("  " + call.name + ": " + call.time.to_string() + "ms")
}

# Memory profiling
mem_prof = profile.memory(func() {
    create_large_data_structure()
})

print("Peak memory: " + mem_prof.peak.to_string() + " MB")
print("Allocations: " + mem_prof.alloc_count.to_string())
```

---

## 3. Package Manager

### Philosophy

- **Simple and fast** - Like npm, cargo, pip
- **Semantic versioning** - Clear version constraints
- **Graph-based dependencies** - Use graphs for dep resolution!
- **Reproducible builds** - Lock files ensure consistency
- **Decentralized** - GitHub, GitLab, custom registries

### Package Structure

```
mypackage/
├── graphoid.toml         # Package manifest
├── graphoid.lock         # Dependency lock file
├── README.md             # Documentation
├── LICENSE               # License file
├── src/
│   ├── main.gr           # Entry point
│   ├── utils.gr
│   └── helpers.gr
├── tests/
│   └── main.test.gr
└── examples/
    └── demo.gr
```

### Package Manifest (`graphoid.toml`)

```toml
[package]
name = "my-awesome-lib"
version = "1.2.3"
description = "An awesome library for Graphoid"
authors = ["Alice <alice@example.com>", "Bob <bob@example.com>"]
license = "MIT"
repository = "https://github.com/alice/my-awesome-lib"
documentation = "https://docs.example.com/my-awesome-lib"
keywords = ["graph", "algorithms", "data-structures"]
categories = ["algorithms", "data-structures"]

# Entry points
main = "src/main.gr"      # Main module
lib = "src/lib.gr"        # Library entry

# Minimum Graphoid version required
graphoid_version = ">=0.5.0"

[dependencies]
# From registry
graph-utils = "^2.0.0"           # Caret: 2.x.x (SemVer compatible)
json-parser = "~1.4.0"           # Tilde: 1.4.x (patch updates only)
http-client = ">=1.0.0, <2.0.0"  # Range

# From git repository
my-internal-lib = { git = "https://github.com/myorg/internal", tag = "v1.0.0" }
dev-tools = { git = "https://github.com/devtools/tools", branch = "main" }
experimental = { git = "https://github.com/exp/lib", rev = "abc123" }

# From local path (development)
local-module = { path = "../local-module" }

[dev-dependencies]
# Only installed for development/testing
test-helpers = "^1.0.0"
mock = "^0.5.0"

[build-dependencies]
# Only for build scripts
code-gen = "^2.1.0"

[scripts]
# Custom commands
test = "graphoid test"
bench = "graphoid benchmark"
docs = "graphoid docs generate"
publish = "graphoid publish"

[features]
# Optional features
default = ["logging"]
logging = []
advanced = ["graph-viz"]
all = ["logging", "advanced"]
```

### Dependency Lock File (`graphoid.lock`)

```toml
# Auto-generated - do not edit manually
# Ensures reproducible builds

[[package]]
name = "graph-utils"
version = "2.1.5"
source = "registry+https://packages.graphoid.org"
checksum = "sha256:abc123..."
dependencies = [
    "data-structures 1.0.0"
]

[[package]]
name = "data-structures"
version = "1.0.0"
source = "registry+https://packages.graphoid.org"
checksum = "sha256:def456..."
dependencies = []

[[package]]
name = "my-internal-lib"
version = "1.0.0"
source = "git+https://github.com/myorg/internal#v1.0.0"
checksum = "git:abc123def456"
dependencies = []
```

### Package Manager Commands

#### Installation

```bash
# Install all dependencies
graphoid install

# Install specific package
graphoid install graph-utils

# Install specific version
graphoid install graph-utils@2.1.5

# Install from git
graphoid install https://github.com/user/repo

# Install globally
graphoid install -g cli-tool

# Install as dev dependency
graphoid install --dev test-helpers

# Update dependencies
graphoid update

# Update specific package
graphoid update graph-utils

# Remove package
graphoid uninstall graph-utils
```

#### Project Management

```bash
# Create new project
graphoid new myproject
# Creates directory with graphoid.toml, src/, tests/, etc.

# Create new library
graphoid new --lib mylib

# Initialize in existing directory
graphoid init

# Check dependencies
graphoid check

# List dependencies
graphoid list
graphoid list --tree  # Show dependency tree
```

#### Publishing

```bash
# Build package
graphoid build

# Run tests before publishing
graphoid test

# Publish to registry
graphoid publish

# Publish specific version
graphoid publish --version 1.2.3

# Dry run (don't actually publish)
graphoid publish --dry-run

# Yank version (mark as broken)
graphoid yank 1.2.2

# Un-yank version
graphoid unyank 1.2.2
```

#### Registry Management

```bash
# Login to registry
graphoid login

# Logout
graphoid logout

# Search packages
graphoid search "graph algorithms"

# Show package info
graphoid info graph-utils

# Show package versions
graphoid versions graph-utils

# Configure registry
graphoid config set registry https://my-registry.com
```

### Dependency Resolution

Uses **graph algorithms** for dependency resolution!

```glang
# Pseudo-code of how graphoid resolves dependencies

# 1. Build dependency graph
dep_graph = graph { type: :dag }

for package in dependencies {
    dep_graph.add_node(package.name, package)
    for dep in package.dependencies {
        dep_graph.add_edge(package.name, dep.name, "depends_on")
    }
}

# 2. Check for cycles (should be DAG)
if dep_graph.has_cycle() {
    error("Circular dependency detected!")
}

# 3. Topological sort for install order
install_order = dep_graph.topological_sort()

# 4. Resolve version constraints
resolved = resolve_versions(install_order, constraints)

# 5. Check for conflicts
conflicts = check_version_conflicts(resolved)
if conflicts.size() > 0 {
    error("Version conflicts: " + conflicts.to_string())
}

# 6. Install in order
for package in resolved {
    install_package(package)
}
```

### Version Constraints (SemVer)

```toml
[dependencies]
# Exact version
exact = "=1.2.3"          # Only 1.2.3

# Caret (compatible)
caret = "^1.2.3"          # >=1.2.3, <2.0.0
caret_minor = "^0.2.3"    # >=0.2.3, <0.3.0
caret_patch = "^0.0.3"    # >=0.0.3, <0.0.4

# Tilde (patch updates)
tilde = "~1.2.3"          # >=1.2.3, <1.3.0
tilde_minor = "~1.2"      # >=1.2.0, <1.3.0

# Wildcards
wildcard = "1.*"          # >=1.0.0, <2.0.0
wildcard_minor = "1.2.*"  # >=1.2.0, <1.3.0

# Comparison
greater = ">1.2.3"
greater_equal = ">=1.2.3"
less = "<2.0.0"
less_equal = "<=2.0.0"

# Ranges
range = ">=1.2.3, <2.0.0"
```

### Package Registry

#### Default Registry

**packages.graphoid.org** - Official package registry

```bash
# Configure default registry
graphoid config set registry https://packages.graphoid.org
```

#### Private Registry

```bash
# Use private registry for organization
graphoid config set registry https://packages.mycompany.com

# Or per-package
graphoid install graph-utils --registry https://packages.mycompany.com
```

#### Alternative Sources

```toml
[package]
name = "mylib"
version = "1.0.0"

[dependencies]
# From specific registry
pkg1 = { version = "1.0.0", registry = "https://alt-registry.com" }

# From git
pkg2 = { git = "https://github.com/user/repo" }

# From local path
pkg3 = { path = "../sibling-package" }

# From tarball URL
pkg4 = { url = "https://example.com/package.tar.gz" }
```

### Build Scripts

**`build.gr`** - Custom build logic

```glang
# build.gr - runs before installation

import "io"
import "json"

# Generate code
func generate_bindings() {
    config = json.parse(io.read_file("config.json"))

    code = "// Auto-generated bindings\n"
    for item in config.items {
        code = code + "func " + item.name + "() { ... }\n"
    }

    io.write_file("src/generated.gr", code)
}

# Run build
generate_bindings()
print("Build complete!")
```

### Package Distribution

#### Building for Distribution

```bash
# Build optimized package
graphoid build --release

# Creates:
# dist/
#   ├── mypackage-1.2.3.tar.gz     # Source distribution
#   └── mypackage-1.2.3.gro        # Compiled/optimized (future)
```

#### Installation from Local Package

```bash
# Install from .tar.gz
graphoid install ./mypackage-1.2.3.tar.gz

# Install from directory
graphoid install ./mypackage/
```

---

## 4. Integration with Language

### Testing in Code

```glang
# Tests can be written inline for quick validation

configure { enable_tests: true } {
    func add(a, b) {
        return a + b
    }

    # Inline test - only runs in test mode
    test func test_add() {
        assert.equal(add(2, 3), 5)
    }
}
```

### Debug Annotations

```glang
# Mark functions for debugging

@debug  # Auto-break on entry
func problematic_function(data) {
    # ...
}

@trace  # Log all calls
func important_function(x) {
    # ...
}

@profile  # Auto-profile
func slow_function() {
    # ...
}
```

### Package Metadata in Code

```glang
# Access package metadata at runtime

import "package"

version = package.version()       # "1.2.3"
name = package.name()             # "my-awesome-lib"
deps = package.dependencies()     # ["graph-utils", "json-parser"]

# Check version requirements
if package.requires("graph-utils", ">=2.0.0") {
    print("Using new API")
} else {
    print("Using legacy API")
}
```

---

## 5. Implementation Roadmap

### Phase 1: Testing Framework (Week 1-2)

1. Implement `assert` module
2. Add test discovery and runner
3. Create test reporter
4. Add setup/teardown support
5. Implement test configuration

**Deliverable**: `graphoid test` command works

### Phase 2: Basic Debugging (Week 3-4)

1. Implement `debug` module
2. Add breakpoint support
3. Create debug REPL
4. Implement stack trace
5. Add variable inspection

**Deliverable**: `debug.break()` and REPL work

### Phase 3: Package Manager (Week 5-8)

1. Define package manifest format
2. Implement dependency resolution
3. Create package registry client
4. Build install/uninstall commands
5. Implement lock files
6. Create `graphoid new` scaffolding

**Deliverable**: Can install packages from registry

### Phase 4: Advanced Features (Week 9-12)

1. Property-based testing
2. Mocking/stubbing system
3. Time-travel debugging
4. Performance profiling
5. DAP integration
6. Private registry support

**Deliverable**: Production-ready tooling

---

## 6. Success Criteria

### Testing Framework

- ✅ Can write and run tests in `.test.gr` files
- ✅ Rich assertions with clear messages
- ✅ Setup/teardown hooks work
- ✅ Coverage reporting functional
- ✅ 95%+ of standard library has tests

### Debugger

- ✅ Breakpoints work in REPL and IDE
- ✅ Can inspect variables and stack
- ✅ Step through code execution
- ✅ Graph visualization works
- ✅ VSCode extension functional

### Package Manager

- ✅ Can create, publish, install packages
- ✅ Dependency resolution works correctly
- ✅ Lock files ensure reproducibility
- ✅ Registry hosting available
- ✅ 100+ packages published to registry

---

## 7. Comparison with Other Languages

### Testing

| Language | Framework | Built-in? | Notes |
|----------|-----------|-----------|-------|
| Python | pytest, unittest | unittest only | pytest is standard but external |
| Rust | cargo test | ✅ Yes | Excellent integration |
| Go | go test | ✅ Yes | Simple, effective |
| **Graphoid** | graphoid test | ✅ Yes | Aims for Rust-level integration |

### Debugging

| Language | Debugger | IDE Support | Notes |
|----------|----------|-------------|-------|
| Python | pdb | Excellent | VSCode, PyCharm |
| Rust | lldb/gdb | Good | Via DAP |
| Go | delve | Excellent | Great DAP support |
| **Graphoid** | graphoid debug | Planned | DAP for all IDEs |

### Package Management

| Language | Tool | Registry | Notes |
|----------|------|----------|-------|
| Python | pip | PyPI | 400k+ packages |
| Rust | cargo | crates.io | 150k+ packages |
| Go | go mod | pkg.go.dev | Decentralized |
| Node.js | npm | npmjs.com | 2M+ packages |
| **Graphoid** | gpm | packages.graphoid.org | Targeting Rust model |

---

## 8. Future Enhancements

### Testing

- Snapshot testing (like Jest)
- Visual regression testing
- Mutation testing
- Fuzz testing integration
- Continuous testing (watch mode++)

### Debugging

- Remote debugging (attach to running process)
- Record/replay debugging
- Distributed debugging (across network)
- Live editing (hot reload during debug)
- AI-assisted debugging (suggest fixes)

### Package Management

- Workspace support (monorepos)
- Binary caching
- Offline mode
- Mirror/proxy support
- Security audit (like npm audit)
- Automatic dependency updates
- Compatibility checking

---

**This specification ensures Graphoid has production-grade tooling from day one, not as an afterthought.**

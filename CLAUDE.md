# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

---

## Project Overview

**Graphoid** is a revolutionary graph-theoretic programming language where **everything is a graph**. Unlike traditional languages that bolt graphs onto the side, Graphoid makes graphs the fundamental abstraction at every level: data structures, variable storage, and even the runtime environment itself.

### Current Status (December 2025)

**Major Milestone: Pure Graphoid HTTPS Working!**

TLS 1.3 has been implemented in pure Graphoid. HTTPS requests now work:
```graphoid
import "http"
response = http.get("https://example.com/")
print(response["body"])
```

**Implementation Status:**

- ✅ **Phases 0-11 Complete** - Core language fully functional
- ✅ **TLS 1.3** - X25519 key exchange, AES-GCM encryption, HKDF key derivation
- ✅ **HTTPS** - Working via `http.get()` in pure Graphoid
- ✅ **Module System** - Stdlib auto-discovery (no environment variables needed)
- ✅ **Project Structure** - Flattened (no more `rust/` subdirectory)
- 📊 **2,228+ Rust Tests Passing**
- 📊 **30/30 Sample Files Working**
- 🔄 **Phase 12** - Native stdlib modules (~15% complete)

**Recent Fixes (December 2025):**
- Module function resolution (stack overflow bug fixed)
- Stdlib path auto-discovery
- List `sublist()` method for performance
- HMAC hex input handling
- TLS multi-record response handling

**Python Implementation**: The Python implementation in `python/` serves as a **reference prototype** demonstrating language concepts. The Rust implementation is the **current bootstrap target**, with the ultimate goal being a self-hosted Graphoid implementation.

---

## Core Philosophy: Everything is a Graph

Graphoid is built on three levels of graph abstraction:

### Level 1: Data Structures as Graphs
- A list `[1, 2, 3]` is internally `Node(1) → Node(2) → Node(3)`
- Trees are graphs with hierarchical constraints
- Maps are graphs with key→value edges
- No artificial boundary between "graph" and "non-graph" data

### Level 2: Variable Storage as Graphs (Meta-Graph)
- Variables are nodes in a meta-graph
- Variable assignment creates edges between name nodes and value nodes
- The namespace itself IS a graph that can be inspected and manipulated

### Level 3: Runtime Environment as Graphs
- Function calls use graph traversal
- Functions are nodes in a call graph
- Modules are subgraphs with import/export edges

---

## Design Principles

- **KISS Principle** - Keep It Simple, Stupid! Graphoid despises unnecessary verbiage
- **Practical** - Must be useful for real-world applications (allows experimental features)
- **No Hidden Side Effects** - Operations never mutate their operands unless explicitly requested with `!` suffix (Principle of Least Surprise)
- **Graph-Theoretic Foundation** - Graphs are fundamental, not bolted-on
- **Self-Aware Data Structures** - Collections understand their own structure
- **Developer Experience** - Excellent error messages, rich tooling, great docs
- **Behavior-Driven Testing** - RSpec-style testing built into the language
- **No Semantic Markers** - All code fully implemented with real enforcement
- **No Method Proliferation** - One method with parameters beats many similar methods
- **Dogfooding** - Use Graphoid extensively to validate its expressiveness
- **🚫 NO GENERICS - EVER** - See `dev_docs/NO_GENERICS_POLICY.md` - Non-negotiable!

---

## Ultimate Goal: Self-Hosting & Zero External Dependencies

**The Rust implementation is a bootstrap, not a foundation.**

The end goal for Graphoid is complete self-sufficiency:

1. **Self-Hosting** - Graphoid compiler/interpreter written entirely in Graphoid
2. **Zero Rust Dependencies** - No Rust code in the final product
3. **Direct System Interface** - Graphoid runtime makes syscalls directly, not via Rust
4. **Pure Graphoid Standard Library** - All stdlib modules (crypto, json, http, etc.) in pure Graphoid

### The Bootstrap Path

The current Rust implementation exists only to birth Graphoid to the point where it can implement itself:

| Phase | Description |
|-------|-------------|
| **Current** | Rust bootstrap - getting Graphoid functional |
| **Phase 13** | Bitwise operators - enables pure Graphoid crypto |
| **Future** | Rewrite lexer, parser, executor in Graphoid |
| **Future** | Graphoid runtime with direct syscalls |
| **End State** | Delete Rust, Graphoid runs Graphoid |

### What This Means for Development

- **Rust code is temporary scaffolding** - Write it knowing it will be replaced
- **Native modules are stopgaps** - Every Rust stdlib module should have a pure Graphoid replacement path
- **No permanent Rust dependencies** - Even syscall wrappers will eventually be pure Graphoid
- **Performance via native acceleration, not native requirement** - Rust crypto may be faster, but pure Graphoid crypto must be *possible*

This is not aspirational - it is the architectural mandate. Every design decision must keep self-hosting in view.

---

## Repository Structure

```
/home/irv/work/grang/
├── CLAUDE.md                    # This file - guidance for Claude Code
├── README.md                    # Project readme
├── Cargo.toml                   # Rust project configuration
├── src/                         # 🎯 Rust implementation source
│   ├── lib.rs                   # Library root
│   ├── main.rs                  # CLI & REPL
│   ├── error.rs                 # Error types
│   ├── lexer/                   # Tokenization
│   ├── parser/                  # AST parsing
│   ├── ast/                     # Syntax tree nodes
│   ├── execution/               # Execution engine
│   ├── values/                  # Value system
│   ├── graph/                   # Graph types & rules
│   └── stdlib/                  # Native stdlib modules (Rust)
├── stdlib/                      # 📦 Standard library (.gr files)
│   ├── tls.gr                   # TLS 1.3 implementation (pure Graphoid!)
│   ├── http.gr                  # HTTP client using TLS
│   ├── math.gr                  # Math functions
│   ├── json.gr                  # JSON parsing
│   ├── time.gr                  # Time/date functions
│   └── ... (15+ modules)
├── samples/                     # Example programs (30 files)
│   ├── 01-basics/               # Hello world, functions, collections
│   ├── 02-intermediate/         # Behaviors, patterns, bitwise
│   ├── 03-advanced/             # Graph pattern matching
│   ├── 04-modules/              # Module system examples
│   └── 05-stdlib/               # Standard library usage
├── tests/                       # Rust tests
│   ├── unit/                    # Unit tests
│   └── integration/             # Integration tests
├── docs/                        # 📖 User documentation
│   ├── WHY_GRAPHOID.md          # Why use Graphoid
│   ├── DESIGN_PHILOSOPHY.md     # Theoretical foundations
│   └── user-guide/              # Tutorial chapters
├── dev_docs/                    # 📋 Development documentation
│   ├── LANGUAGE_SPECIFICATION.md           # Canonical language spec
│   ├── RUST_IMPLEMENTATION_ROADMAP.md      # 14-phase implementation plan
│   ├── ARCHITECTURE_DESIGN.md              # Internal architecture decisions
│   ├── PRODUCTION_TOOLING_SPECIFICATION.md # Testing, debugging, packaging
│   ├── PRODUCTION_TOOLING_SUMMARY.md       # Tooling executive summary
│   └── TESTING_FRAMEWORK_COMPARISON.md     # RSpec-style vs traditional
└── docs/                        # 📖 USER DOCUMENTATION (future)
```

### Documentation Organization Rules

**CRITICAL**: NEVER place documentation files (.md) in the root directory!

- **Root directory**: Only README.md and CLAUDE.md belong here
- **User documentation**: Always goes in `docs/` (user-facing guides, tutorials, API references)
- **Development docs**: Always goes in `dev_docs/` (architecture, design decisions, roadmaps)

### Understanding the dev_docs/ Structure

The `dev_docs/` directory contains **comprehensive development documentation** in the **project root** (`/home/irv/work/grang/dev_docs/`), NOT in the rust directory. These documents guide the entire project implementation:

**Core Specifications:**
1. **`LANGUAGE_SPECIFICATION.md`** - The canonical reference
   - Complete syntax and semantics
   - All language features documented
   - RSpec-style testing framework specification
   - Standard library API
   - **Use this** to understand what Graphoid should do

2. **`NO_GENERICS_POLICY.md`** - 🚫 Non-negotiable design principle
   - Why Graphoid NEVER has user-space generics
   - What's allowed: `list<num>` (single param, runtime-checked, built-in only)
   - What's forbidden: Multiple params, user-defined generics, nested constraints
   - Parser and semantic analyzer enforcement rules
   - Alternative patterns (duck typing, graph rules)
   - **Read this FIRST** before implementing any type-related features

3. **`RUST_IMPLEMENTATION_ROADMAP.md`** - The implementation plan
   - 14 phases from setup to production tooling
   - Copy-paste-ready code for each phase
   - Success criteria and timelines
   - **Use this** to know what to implement next

4. **`ARCHITECTURE_DESIGN.md`** - Design decisions
   - Two-tier value system
   - Five-layer graph architecture
   - Critical implementation choices
   - **Use this** to understand WHY things are designed this way

**Production Tooling:**
5. **`PRODUCTION_TOOLING_SPECIFICATION.md`** - Full tooling specs
   - Testing framework (RSpec-style)
   - Debugger specification
   - Package manager design
   - **Use this** for Phases 12-14

6. **`PRODUCTION_TOOLING_SUMMARY.md`** - Executive summary
7. **`TESTING_FRAMEWORK_COMPARISON.md`** - Why RSpec-style testing

**Session Documentation (in project root):**
- `SESSION_SUMMARY.md` - What was accomplished this session
- `START_HERE_NEXT_SESSION.md` - Quick start guide for next session

**DO NOT create new documentation files** in `dev_docs/` without explicit user request. The existing documents are comprehensive and authoritative.

---

## Current Development: Rust Implementation

### Where We Are Now (December 2025)

**Phases 0-11: ✅ COMPLETE**
- Lexer, Parser, AST
- Value System & Basic Execution
- Functions & Lambdas
- Collections & Methods
- Graph Types & Rules
- Module System
- Pattern Matching
- Pure Graphoid Stdlib (11 modules)

**Phase 12: 🔄 IN PROGRESS** (~15% complete)
- Native stdlib modules (Rust acceleration)
- Constants, Random modules complete
- TLS 1.3, HTTP working in pure Graphoid

**Key Capabilities:**
- ✅ **HTTPS Working** - `http.get("https://...")` in pure Graphoid
- ✅ **TLS 1.3** - X25519, AES-GCM, HKDF implemented
- ✅ **Module System** - Auto-discovery, no env vars needed
- ✅ **30 Sample Programs** - All working, well-organized

**Current Test Status**: ✅ 2,228+ tests passing

### Development Commands

```bash
# Rust implementation (ACTIVE)
# From project root

# Build
~/.cargo/bin/cargo build

# Run tests
~/.cargo/bin/cargo test --lib

# Run REPL
~/.cargo/bin/cargo run

# Execute a .gr file
~/.cargo/bin/cargo run -- path/to/file.gr

# Run with release optimizations
~/.cargo/bin/cargo build --release
```

### Python Reference Implementation

```bash
# Python prototype (reference only)
cd python

# Activate virtual environment
source ../.venv/bin/activate

# Run Python REPL
python -m glang.repl

# Execute a .gr file
python -m glang.cli samples/demo.gr

# Run Python tests
pytest test/
```

---

## Key Language Features

### RSpec-Style Testing (Built-In!)

Graphoid includes a **behavior-driven testing framework** inspired by RSpec:

```graphoid
# In calculator.spec.gr
import "spec"

describe "Calculator" {
    describe "add" {
        it "adds two positive numbers" {
            result = calculator.add(2, 3)
            expect(result).to_equal(5)
        }

        it "handles negative numbers" {
            expect(calculator.add(-2, -3)).to_equal(-5)
        }
    }

    context "when dividing by zero" {
        it "raises an error" {
            expect(fn() {
                calculator.divide(10, 0)
            }).to_raise("RuntimeError")
        }
    }
}
```

**Command**: `graphoid spec` (once implemented)

**Features**:
- Natural language expectations: `expect().to_equal()`, `expect().to_be_truthy()`
- Hierarchical organization: `describe`, `context`, `it`
- Hooks: `before_all`, `after_all`, `before_each`, `after_each`
- Shared examples: `it_behaves_like "a collection"`
- Table-driven tests with `where` blocks
- Mocking and stubbing

See `dev_docs/PRODUCTION_TOOLING_SPECIFICATION.md` for complete details.

### Graph-Based Collections

```graphoid
# Lists are linked graphs
items = [1, 2, 3]
items.append(4)

# Maps are hash graphs
config = {"host": "localhost", "port": 8080}
config["debug"] = true

# Trees are constrained graphs
numbers = tree{}
numbers.insert(5)
numbers.insert(3)
numbers.insert(7)

# Graphs are... graphs!
my_graph = graph { type: :dag }
my_graph.add_node("A", 100)
my_graph.add_edge("A", "B", "depends_on")
```

### Intrinsic Behavior System

Data structures can have behaviors that automatically transform values:

```graphoid
# Automatic nil handling
temperatures = [98.6, none, 102.5]
temperatures.add_rule("none_to_zero")
print(temperatures)  # [98.6, 0, 102.5]

# Range validation
temperatures.add_rule("validate_range", 95, 105)
temperatures.append(110)  # Automatically clamped to 105

# Custom mappings
color_map = {"red": 1, "green": 2, "blue": 3}
colors = ["red", "blue", "purple"]
colors.add_mapping_rule(color_map, 0)  # Default 0 for unmapped
print(colors)  # [1, 3, 0]
```

### Functional Programming

```graphoid
numbers = [1, 2, 3, 4, 5]

# Named transformations
numbers.map("double")          # [2, 4, 6, 8, 10]
numbers.map("square")          # [1, 4, 9, 16, 25]

# Named predicates
numbers.filter("even")         # [2, 4]
numbers.filter("positive")     # [1, 2, 3, 4, 5]
numbers.reject("even")         # [1, 3, 5]

# Chaining
numbers.filter("positive").map("double").reject("even")

# Lambdas
numbers.map(x => x * 3)
numbers.filter(x => x > 10)
```

### String Generators

Graphoid provides string generation static methods mirroring list generators:

```graphoid
# Repetition mode: string.generate(str, count)
padding = string.generate(" ", 10)        # "          " (10 spaces)
separator = string.generate("-", 20)      # "--------------------"
bar = string.generate("#", count)         # Dynamic repetition

# Sequence mode: string.generate(from_char, to_char)
lowercase = string.generate("a", "z")     # "abcdefghijklmnopqrstuvwxyz"
uppercase = string.generate("A", "Z")     # "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
digits = string.generate("0", "9")        # "0123456789"

# Use in formatting
fn center(text, width) {
    padding = (width - text.length()) / 2
    return string.generate(" ", padding) + text + string.generate(" ", padding)
}
```

### Optional Type System

**IMPORTANT**: Graphoid does NOT have generics. See `dev_docs/NO_GENERICS_POLICY.md`

```graphoid
# Type inference (recommended)
name = "Alice"              # Infers string
age = 25                    # Infers num
items = [1, 2, 3]           # Infers list

# Explicit types when needed
string username = "Bob"
num max_age = 100

# ✅ ALLOWED: Single-parameter runtime type constraints on built-in collections
list<num> scores = [95, 87, 92]       # Runtime-checked, single param
tree<string> words = tree{}           # Built-in collection only
hash<num> config = {}                 # Values only (keys always string)

# ❌ FORBIDDEN: See NO_GENERICS_POLICY.md
# hash<string, num> data = {}         # Multiple params - NEVER
# fn process<T>(x: T) { ... }         # Generic functions - NEVER
# class Container<T> { ... }          # Generic classes - NEVER
# list<list<num>> matrix = []         # Nested constraints - NEVER
```

### Graph Rules

```graphoid
# Built-in rules
my_tree.add_rule("no_cycles")
my_tree.add_rule("single_root")
my_tree.add_rule("max_children_2")  # Binary tree

# User-defined rules
fn validate_positive_values(graph) {
    for node in graph.nodes() {
        if node.value() < 0 {
            return false
        }
    }
    return true
}

my_graph.add_rule(validate_positive_values)
```

---

## Production Tooling

Graphoid includes **professional-grade tooling** from day one:

### 1. Testing Framework (Phase 13)
- **RSpec-style** behavior-driven testing
- Command: `graphoid spec`
- File extension: `.spec.gr`
- Natural language expectations
- Hierarchical organization
- Mocking, stubbing, property-based testing

### 2. Debugger (Phase 14)
- Breakpoints: `debug.break()`, `debug.break_if()`
- Interactive debug REPL
- Variable and stack inspection
- Step-through execution
- Performance profiling
- Graph visualization
- VSCode integration via DAP

### 3. Package Manager (Phase 15)
- Package manifest: `graphoid.toml`
- Lock files: `graphoid.lock`
- SemVer version constraints
- **Graph-based dependency resolution** (dogfooding!)
- Commands: `graphoid new`, `graphoid install`, `graphoid publish`
- Registry: packages.graphoid.org

---

## Implementation Roadmap

See `dev_docs/RUST_IMPLEMENTATION_ROADMAP.md` for the complete 18-phase plan:

| Phase | Focus | Duration | Status |
|-------|-------|----------|--------|
| 0 | Project Setup | 1-2 days | ✅ COMPLETE |
| 1 | Lexer | 3-5 days | ✅ COMPLETE (54 tests) |
| 2 | Parser & AST | 5-7 days | ✅ COMPLETE (31 tests) |
| 3 | Value System & Basic Execution | 5-7 days | ✅ COMPLETE |
| 4 | Functions & Lambdas | 4-6 days | ✅ COMPLETE |
| 5 | Collections & Methods | 7-10 days | ✅ COMPLETE |
| 6 | Graph Types & Rules | 10-14 days | ✅ COMPLETE |
| 6.5 | Foundational Gaps & Verification | 5-7 days | ✅ COMPLETE (132+ tests) |
| 7 | Function Pattern Matching & Behaviors | 5-7 days | ✅ COMPLETE (186+ tests) |
| 8 | Module System (Basic) | 4-6 days | ✅ COMPLETE (31 tests) |
| 9 | Graph Pattern Matching | 7-10 days | ✅ COMPLETE (69 tests) |
| 10 | Advanced Module Features | 3-5 days | ✅ COMPLETE (40 tests) |
| 11 | Pure Graphoid Stdlib | 10-14 days | ✅ COMPLETE (11 modules, 24 tests) |
| 12 | Native Stdlib Modules | 14-21 days | 🔄 ~15% COMPLETE (Constants, Random) |
| 13 | Bitwise Operators & Integer Types | 5-7 days | 🔲 Pending |
| 14 | Stdlib Translation to Pure Graphoid | 7-10 days | 🔲 Pending |
| 15 | Testing Framework | 7-10 days | 🔲 Pending |
| 16 | Debugger | 10-14 days | 🔲 Pending |
| 17 | Package Manager | 14-21 days | 🔲 Pending |

**Milestones**:
- **MVP** (Phases 0-5): 6-8 weeks - Basic language works ✅
- **Feature Complete** (Phases 0-14): 20-25 weeks - All language features + 90% self-hosting
- **Production Tools** (Phases 0-17): 24-32 weeks - Testing, debugging, packaging
- **Production Ready**: 32-40 weeks - Optimized, documented, polished

---

## Key Documents

### Essential Reading

1. **`dev_docs/LANGUAGE_SPECIFICATION.md`** (1780 lines)
   - Canonical language specification
   - Complete syntax and semantics
   - Type system, collections, graph rules
   - Built-in testing framework (RSpec-style)
   - Standard library reference

2. **`dev_docs/NO_GENERICS_POLICY.md`** 🚫 (CRITICAL - Non-negotiable)
   - Why Graphoid NEVER has user-space generics
   - Allowed: Single-parameter runtime type constraints on built-in collections
   - Forbidden: Multiple params, user-defined generics, generic functions, nested constraints
   - Parser/analyzer enforcement rules with error message templates
   - Alternative patterns: duck typing, graph rules, runtime checks
   - **READ THIS BEFORE implementing type-related features**

3. **`dev_docs/RUST_IMPLEMENTATION_ROADMAP.md`** (1840 lines)
   - 14-phase implementation plan
   - Copy-paste-ready code for each phase
   - Success criteria and timelines
   - Testing strategy

4. **`dev_docs/ARCHITECTURE_DESIGN.md`** (detailed architecture)
   - Two-tier value system (Simple vs Graph-backed)
   - Five-layer graph architecture
   - Rule validation context
   - Node identity strategy
   - Critical implementation decisions

5. **`dev_docs/PRODUCTION_TOOLING_SPECIFICATION.md`** (60+ pages)
   - RSpec-style testing framework
   - Interactive debugger
   - Package manager (Cargo-inspired)
   - Complete API references

6. **`dev_docs/PRODUCTION_TOOLING_SUMMARY.md`**
   - Executive summary of tooling
   - Comparison with other languages
   - FAQ and migration path

7. **`dev_docs/TESTING_FRAMEWORK_COMPARISON.md`**
   - Before/after comparison of testing styles
   - Why RSpec-style wins
   - Migration examples

### Quick Reference

- Phases 0-11: ✅ Complete (core language functional)
- Phase 12: 🔄 Native stdlib modules (~15%)
- **TLS 1.3: ✅ Working** (pure Graphoid!)
- **HTTPS: ✅ Working** via `http.get()`
- Total tests: **2,228+ passing**
- Samples: **30/30 working**
- Command: `~/.cargo/bin/cargo test --lib`
- Build: `~/.cargo/bin/cargo build`

---

## Development Guidelines

### Code Quality Standards

- **Test-Driven Development (TDD)** - Write tests FIRST, then implement (RED-GREEN-REFACTOR) - MANDATORY
- **Example-Driven Development** - Create `.gr` example files for EVERY significant new feature - MANDATORY
- **Idiomatic Rust** - Follow Rust best practices
- **Zero warnings** - `cargo build` must be clean
- **Test coverage** - 80%+ for core features (achieved through TDD)
- **Documentation** - All public APIs documented
- **Error messages** - Rich, helpful, with source positions

### Example-Driven Development Rule

**CRITICAL**: After implementing ANY significant new feature, you MUST create one or more `.gr` example files demonstrating it.

**Why This Rule Exists:**
- Ensures features are actually accessible from .gr programs (not just in Rust tests)
- Validates end-to-end functionality
- Provides immediate documentation through working code
- Prevents implementation gaps (like Phase 7 behaviors being inaccessible)

**What Counts as "Significant":**
- New language features (pattern matching, modules, etc.)
- New collection methods or transformations
- New behavior/validation rules
- New graph operations
- New built-in functions

**Where to Put Examples:**
- `samples/*.gr` - Standalone example files
- Each example should have clear comments explaining what it demonstrates
- Update `samples/README.md` with descriptions

**Example Checklist for New Features:**
1. ✅ Implement feature in Rust
2. ✅ Write Rust unit tests (TDD)
3. ✅ **Create `.gr` example file(s)** demonstrating the feature
4. ✅ Run the example to verify it works: `cargo run --quiet samples/your_example.gr`
5. ✅ Update `samples/README.md` with description
6. ✅ Consider updating `docs/QUICKSTART.md` if it's a major feature

**Real Example:**
When Phase 7 behavior system was implemented:
- ❌ BAD: 91 Rust tests passed, but behaviors weren't accessible from .gr programs
- ✅ GOOD: After fixing executor registration, created `samples/behaviors.gr` showing all transformation rules

**If you can't create a working `.gr` example, the feature isn't done!**

### Testing Strategy

**🚨 CRITICAL: Features MUST Be Usable by Programmers to Be "Complete"**

A feature is NOT complete until it works from `.gr` files. Passing Rust tests alone is insufficient.

**The Three-Level Validation Requirement**:
1. **Level 1: Rust API Testing** - Unit tests verify internal implementation
2. **Level 2: Executor Integration** - Feature must be registered in executor and accessible from .gr programs
3. **Level 3: Example Documentation** - One or more `.gr` example files demonstrating the feature

**All three levels are MANDATORY**. A feature that passes Rust tests but can't be used from .gr programs is NOT done.

---

**CRITICAL: Test-Driven Development (TDD) is MANDATORY**

All development follows strict TDD methodology:

1. 🔴 **RED**: Write failing tests FIRST (before any implementation)
2. 🟢 **GREEN**: Write minimal code to make tests pass
3. 🔵 **REFACTOR**: Clean up code while keeping tests passing

**Example TDD Workflow**:
```rust
// Step 1 (RED): Write test first in tests/unit/weighted_graph_tests.rs
#[test]
fn test_dijkstra_simple_weighted_path() {
    let mut g = Graph::new(GraphType::Directed);
    g.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    g.add_edge("A", "B", "road".to_string(), Some(5.0), HashMap::new()).unwrap();

    // This will fail - method doesn't exist yet
    let path = g.shortest_path("A", "B", None, true).unwrap();
    assert_eq!(path, vec!["A", "B"]);
}

// Step 2 (GREEN): Implement in src/values/graph.rs to make test pass
pub fn shortest_path(&self, from: &str, to: &str, edge_type: Option<&str>, weighted: bool) -> Option<Vec<String>> {
    // Implementation...
}

// Step 3 (REFACTOR): Clean up while keeping tests passing
```

**Test Organization**:
- **Unit tests (Rust)** - In `tests/unit/` for internal API verification
- **Integration tests (.gr files)** - In `tests/integration/` for user-facing executability
- **Example files (.gr)** - In `samples/` for documentation and demonstration
- **Property-based tests** - For algorithmic correctness
- **Regression tests** - For every bug fix

**IMPORTANT: Test File Organization**

- ❌ **NEVER** use `#[cfg(test)]` modules in `src/` files
- ✅ **ALWAYS** place Rust tests in `tests/unit/` or `tests/integration/`
- ✅ **ALWAYS** create `.gr` integration tests in `tests/integration/`
- ✅ **ALWAYS** create `.gr` example files in `samples/` for new features
- ✅ **ALWAYS** write tests BEFORE implementation (TDD)
- ✅ **ALWAYS** register methods/functions in executor after implementing
- ✅ **ALWAYS** verify examples run: `cargo run --quiet samples/your_example.gr`
- Tests in `tests/unit/` should import from the crate: `use graphoid::module::Type;`
- Each source module can have a corresponding test file (e.g., `src/graph/rules.rs` → `tests/unit/graph_rules_tests.rs`)
- Register new test files in `tests/unit_tests.rs`

**Verification**:
- Rust: Run `find src -name "*.rs" -exec grep -l "#\[cfg(test)\]" {} \;` - should return no results
- Integration: Run `bash scripts/test_integration.sh` - all .gr files should execute
- Examples: Run `for f in examples/*.gr; do cargo run --quiet "$f" || exit 1; done` - all examples should run successfully

**Why TDD**: Writing tests first ensures complete test coverage, better API design, prevents regressions, and validates requirements before implementation.

**Why Separate Files**: Keeps source files clean, reduces compilation time for non-test builds, and follows Rust best practices for larger projects.

**Why .gr Integration Tests**: Rust unit tests only verify internal API. They don't test executor integration or user-facing accessibility. A feature that passes Rust tests but fails from .gr files is NOT complete.

### Git Workflow

- **Frequent commits** - After each passing test suite
- **Clear messages** - Descriptive commit messages
- **Feature branches** - For major work
- **No semantic markers** - All code fully implemented

### Implementation Priorities

1. **Correctness** - Get it right first
2. **Clarity** - Write readable, maintainable code
3. **Performance** - Profile before optimizing
4. **Polish** - Excellent error messages and UX

---

## Common Patterns

### Type Inference First

```graphoid
# ✅ GOOD - Let type inference work
name = "Alice"
items = [1, 2, 3]

# ❌ BAD - Unnecessary type annotations
string name = "Alice"
list items = [1, 2, 3]

# ✅ GOOD - Use explicit types for constraints
list<num> scores = [95, 87, 92]
```

### No Import Aliases for Stdlib

```graphoid
# ✅ GOOD - Built-in aliases work automatically
import "random"     # Both random.choice() and rand.choice() work
import "statistics" # Both statistics.mean() and stats.mean() work

# ❌ BAD - Redundant aliases
import "random" as rand        # rand is already available!
import "statistics" as stats   # stats is already available!
```

### One Method with Parameters

```graphoid
# ✅ GOOD - One method, parameter controls behavior
io.print("message", false)     # No newline
list.remove(element, :all)     # Remove all occurrences

# ❌ BAD - Method proliferation
io.print_nonewline("message")
list.remove_all(element)
```

---

## FAQ

### Why Rust instead of staying with Python?

**Performance, safety, and production-readiness**. Python was great for prototyping, but Rust gives us:
- 100x+ performance
- Memory safety without GC
- Fearless concurrency
- Better error messages
- Zero-cost abstractions

### What happens to the Python implementation?

It becomes a **reference implementation** demonstrating language concepts. The Rust implementation is the production target, but Python helps us understand what works.

### How long until Graphoid is usable?

Graphoid is usable **now** for many tasks:
- ✅ **HTTPS/TLS working** - Can fetch web APIs
- ✅ **JSON parsing** - Handle API responses
- ✅ **File I/O** - Read/write files
- ✅ **Math/Statistics** - Data analysis

Still in progress: Testing framework, debugger, package manager.

### Can I use Graphoid today?

Yes! The Rust implementation is functional. Try:
```bash
~/.cargo/bin/cargo run --quiet samples/01-basics/hello_world.gr
```

### What makes Graphoid different from other languages?

1. **Everything is a graph** - Not bolted on, fundamental
2. **Intrinsic behaviors** - Data structures transform values automatically
3. **RSpec-style testing** - Built into the language
4. **Graph-based package manager** - Dependency resolution uses graph algorithms
5. **Self-aware collections** - Data understands its own structure

---

## Next Steps

### For Contributors

1. Read `dev_docs/LANGUAGE_SPECIFICATION.md` to understand the language
2. Read `dev_docs/ARCHITECTURE_DESIGN.md` to understand the design
3. Start with Phase 1 (Lexer) in `dev_docs/RUST_IMPLEMENTATION_ROADMAP.md`
4. Write tests first (TDD)
5. Keep `cargo build` warning-free

### For Next Session

**START HERE**: Read `START_HERE_NEXT_SESSION.md` for detailed guide

**Major Milestone Achieved: Pure Graphoid HTTPS Working! (December 2025)**

**Last Session**: December 1-2, 2025

**Recent Accomplishments**:
- ✅ **TLS 1.3 Complete** - X25519, AES-GCM, HKDF working in pure Graphoid
- ✅ **HTTPS Working** - `http.get("https://...")` fetches real websites
- ✅ **6 Major Bugs Fixed** - Module resolution, HMAC, stdlib path, etc.
- ✅ **Project Restructured** - No more `rust/` subdirectory

**Next Goal**: Build Bitcoin Price Tracker application
- Fetch BTC prices via HTTPS
- Store in JSON
- Perform statistical analysis
- All in 100% Graphoid

**🚀 Recommended Next Steps:**

1. **⭐ RECOMMENDED: Bitcoin Price Tracker** (The original goal!)
   - Fetch BTC prices from a public API via HTTPS
   - Store historical data in JSON files
   - Calculate statistics (mean, std dev, trends)
   - Ask: "Let's build the Bitcoin price tracker"

2. **Add HTTP POST Support** (~1 day)
   - Extend http.gr with `http.post(url, body, headers)`
   - Needed for APIs that require POST

3. **Complete API Reference Documentation** (~2-3 days)
   - Finish remaining stdlib module documentation
   - Document tls.gr and http.gr

4. **Improve TLS Error Handling** (~1 day)
   - Better error messages for connection failures
   - Timeout handling

---

**Questions?** Check the specs first, then ask!

**Remember**: Graphoid is about making graphs **fundamental**, not optional. Every design decision should reinforce this vision.

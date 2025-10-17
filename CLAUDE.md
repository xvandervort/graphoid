# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

---

## Project Overview

**Graphoid** is a revolutionary graph-theoretic programming language where **everything is a graph**. Unlike traditional languages that bolt graphs onto the side, Graphoid makes graphs the fundamental abstraction at every level: data structures, variable storage, and even the runtime environment itself.

### Current Status (January 2025)

**FRESH START - RUST IMPLEMENTATION UNDERWAY**

The project is undergoing a **clean-slate Rust implementation** to build a production-ready, high-performance language from the ground up.

- ✅ **Phase 0 Complete** - Project structure, dependencies, error types, CLI/REPL skeleton
- 🔜 **Phase 1 Starting** - Lexer (tokenization) implementation
- 📋 **14-Phase Roadmap** - Complete path to production-ready language with professional tooling
- 📚 **Comprehensive Specifications** - Language spec, architecture design, production tooling all documented

**Python Implementation**: The Python implementation in `python/` serves as a **reference prototype** demonstrating language concepts. The Rust implementation in `rust/` is the **production target**.

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
- **Practical First** - Must be useful for real-world applications
- **Graph-Theoretic Foundation** - Graphs are fundamental, not bolted-on
- **Self-Aware Data Structures** - Collections understand their own structure
- **Developer Experience** - Excellent error messages, rich tooling, great docs
- **Behavior-Driven Testing** - RSpec-style testing built into the language
- **No Semantic Markers** - All code fully implemented with real enforcement
- **No Method Proliferation** - One method with parameters beats many similar methods
- **Dogfooding** - Use Graphoid extensively to validate its expressiveness

---

## Repository Structure

```
/home/irv/work/grang/
├── CLAUDE.md                    # This file - guidance for Claude Code
├── README.md                    # Project readme
├── rust/                        # 🎯 ACTIVE DEVELOPMENT - Rust implementation
│   ├── src/                     # Source code
│   │   ├── lib.rs               # Library root
│   │   ├── main.rs              # CLI & REPL
│   │   ├── error.rs             # Error types (complete)
│   │   ├── lexer/               # Tokenization (Phase 1 - next)
│   │   ├── parser/              # AST parsing (Phase 2)
│   │   ├── ast/                 # Syntax tree nodes (Phase 2)
│   │   ├── execution/           # Execution engine (Phase 3)
│   │   ├── values/              # Value system (Phase 3)
│   │   └── graph/               # Graph types & rules (Phase 6)
│   ├── tests/                   # Rust tests
│   │   ├── unit/                # Unit tests
│   │   └── integration/         # Integration tests
│   ├── benches/                 # Performance benchmarks
│   ├── examples/                # Example programs
│   ├── Cargo.toml               # Rust dependencies
│   └── README.md                # Rust-specific readme
├── python/                      # Python prototype (reference only)
│   ├── src/glang/               # Python implementation
│   ├── test/                    # Python tests
│   ├── stdlib/                  # Standard library in .gr files
│   └── samples/                 # Example .gr programs
├── dev_docs/                    # 📋 DEVELOPMENT DOCUMENTATION
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

---

## Current Development: Rust Implementation

### Where We Are Now

**Phase 0: ✅ COMPLETE** (January 2025)
- Rust project structure created
- All dependencies configured (thiserror, regex, chrono, serde, crypto, etc.)
- Error types with source position tracking
- CLI and REPL skeleton functional
- `cargo build` and `cargo test` working

**Phase 1: 🔜 STARTING NEXT** - Lexer (Tokenization)
- Define all token types
- Implement tokenizer with position tracking
- Handle comments, strings, numbers, symbols, regex literals
- Write 20+ tests

### Development Commands

```bash
# Rust implementation (ACTIVE)
cd rust

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
            expect(func() {
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

### Optional Type System

```graphoid
# Type inference (recommended)
name = "Alice"              # Infers string
age = 25                    # Infers num
items = [1, 2, 3]           # Infers list

# Explicit types when needed
string username = "Bob"
num max_age = 100
list<num> scores = [95, 87, 92]
tree<string> words = tree{}
```

### Graph Rules

```graphoid
# Built-in rules
my_tree.add_rule("no_cycles")
my_tree.add_rule("single_root")
my_tree.add_rule("max_children_2")  # Binary tree

# User-defined rules
func validate_positive_values(graph) {
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

### 1. Testing Framework (Phase 12)
- **RSpec-style** behavior-driven testing
- Command: `graphoid spec`
- File extension: `.spec.gr`
- Natural language expectations
- Hierarchical organization
- Mocking, stubbing, property-based testing

### 2. Debugger (Phase 13)
- Breakpoints: `debug.break()`, `debug.break_if()`
- Interactive debug REPL
- Variable and stack inspection
- Step-through execution
- Performance profiling
- Graph visualization
- VSCode integration via DAP

### 3. Package Manager (Phase 14)
- Package manifest: `graphoid.toml`
- Lock files: `graphoid.lock`
- SemVer version constraints
- **Graph-based dependency resolution** (dogfooding!)
- Commands: `graphoid new`, `graphoid install`, `graphoid publish`
- Registry: packages.graphoid.org

---

## Implementation Roadmap

See `dev_docs/RUST_IMPLEMENTATION_ROADMAP.md` for the complete 14-phase plan:

| Phase | Focus | Duration | Status |
|-------|-------|----------|--------|
| 0 | Project Setup | 1-2 days | ✅ COMPLETE |
| 1 | Lexer | 3-5 days | 🔜 NEXT |
| 2 | Parser & AST | 5-7 days | 🔲 Pending |
| 3 | Value System & Basic Execution | 5-7 days | 🔲 Pending |
| 4 | Functions & Lambdas | 4-6 days | 🔲 Pending |
| 5 | Collections & Methods | 7-10 days | 🔲 Pending |
| 6 | Graph Types & Rules | 10-14 days | 🔲 Pending |
| 7 | Behavior System | 5-7 days | 🔲 Pending |
| 8 | Module System | 4-6 days | 🔲 Pending |
| 9 | Native Stdlib Modules | 14-21 days | 🔲 Pending |
| 10 | Pure Graphoid Stdlib | 10-14 days | 🔲 Pending |
| 11 | Advanced Features | 14-21 days | 🔲 Pending |
| 12 | Testing Framework | 7-10 days | 🔲 Pending |
| 13 | Debugger | 10-14 days | 🔲 Pending |
| 14 | Package Manager | 14-21 days | 🔲 Pending |

**Milestones**:
- **MVP** (Phases 0-5): 6-8 weeks - Basic language works
- **Feature Complete** (Phases 0-11): 12-16 weeks - All language features
- **Production Tools** (Phases 0-14): 16-22 weeks - Testing, debugging, packaging
- **Production Ready**: 24-28 weeks - Optimized, documented, polished

---

## Key Documents

### Essential Reading

1. **`dev_docs/LANGUAGE_SPECIFICATION.md`** (1780 lines)
   - Canonical language specification
   - Complete syntax and semantics
   - Type system, collections, graph rules
   - Built-in testing framework (RSpec-style)
   - Standard library reference

2. **`dev_docs/RUST_IMPLEMENTATION_ROADMAP.md`** (1840 lines)
   - 14-phase implementation plan
   - Copy-paste-ready code for each phase
   - Success criteria and timelines
   - Testing strategy

3. **`dev_docs/ARCHITECTURE_DESIGN.md`** (detailed architecture)
   - Two-tier value system (Simple vs Graph-backed)
   - Five-layer graph architecture
   - Rule validation context
   - Node identity strategy
   - Critical implementation decisions

4. **`dev_docs/PRODUCTION_TOOLING_SPECIFICATION.md`** (60+ pages)
   - RSpec-style testing framework
   - Interactive debugger
   - Package manager (Cargo-inspired)
   - Complete API references

5. **`dev_docs/PRODUCTION_TOOLING_SUMMARY.md`**
   - Executive summary of tooling
   - Comparison with other languages
   - FAQ and migration path

6. **`dev_docs/TESTING_FRAMEWORK_COMPARISON.md`**
   - Before/after comparison of testing styles
   - Why RSpec-style wins
   - Migration examples

### Quick Reference

- Phase 0 complete: ✅ Project setup done
- Phase 1 next: 🔜 Implement lexer/tokenizer
- Command: `cd rust && cargo build`
- Tests: `cargo test --lib`
- REPL: `cargo run`

---

## Development Guidelines

### Code Quality Standards

- **Idiomatic Rust** - Follow Rust best practices
- **Zero warnings** - `cargo build` must be clean
- **Test coverage** - 80%+ for core features
- **Documentation** - All public APIs documented
- **Error messages** - Rich, helpful, with source positions

### Testing Strategy

- **Unit tests** - In `tests/unit/` for individual components
- **Integration tests** - In `tests/integration/` for workflows
- **Property-based tests** - For algorithmic correctness
- **Regression tests** - For every bug fix

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

- **MVP (basic language)**: 6-8 weeks from now
- **Feature complete**: 12-16 weeks
- **Production ready** (with tooling): 24-28 weeks

### Can I use Graphoid today?

The Python prototype works for experimentation. The Rust implementation will be production-ready in ~6 months.

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

### For This Session

**START HERE**: Begin Phase 1 - Lexer implementation

1. Read Phase 1 section in roadmap
2. Define token types in `rust/src/lexer/token.rs`
3. Implement lexer in `rust/src/lexer/mod.rs`
4. Write 20+ tests in `rust/tests/unit/lexer_tests.rs`
5. Ensure all tests pass and build is clean

---

**Questions?** Check the specs first, then ask!

**Remember**: Graphoid is about making graphs **fundamental**, not optional. Every design decision should reinforce this vision.

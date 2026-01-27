# Why Graphoid?

Graphoid is an **experimental programming language** where graphs are fundamental, not an afterthought. More than that, it's an experiment in **AI-assisted language development** - exploring what's possible when human creativity and AI capabilities work together from day one.

**Current Status: Alpha** - Graphoid is early-stage and evolving. It's perfect for experimenters, learners, and anyone who wants to influence the direction of an important new project. It's **not yet suitable for production use**, but it's a fascinating space to explore.

---

## An Experiment in AI-Assisted Language Development

Graphoid is being developed **with extensive AI assistance**, and we're discovering new possibilities in the process:

- **Rapid iteration**: From concept to working implementation in months, not years
- **Comprehensive from day one**: Complete documentation, testing framework, and tooling designed upfront
- **Living documentation**: The language specification, implementation roadmap, and code evolve together
- **Quality at speed**: Test-driven development with AI assistance maintains high quality while moving fast
- **Transparent process**: The entire development process is documented and visible

This isn't just about using AI as a coding assistant. It's about **rethinking the entire language development process** when AI is a full collaborator. You're invited to watch, participate, and help shape what emerges.

---

## What Makes Graphoid Different?

### Everything is a Graph (Really!)

Most languages add graphs as a library. Graphoid makes graphs **fundamental** at three levels:

**Level 1: Data Structures Are Graphs**
```graphoid
# A list is internally a linked graph
items = [1, 2, 3]  # Node(1) → Node(2) → Node(3)

# Trees are constrained graphs
tree = tree{}
tree.insert(5).insert(3).insert(7)

# Graphs are first-class citizens
network = graph { type: :dag }
network.add_node("server1", {"cpu": 2.5, "mem": 16})
network.add_edge("server1", "server2", "connects_to")
```

**Level 2: Variable Storage is a Graph**
```graphoid
# Variables themselves form a meta-graph
x = 10
y = x  # Creates an edge in the variable graph
# You can query and traverse the namespace itself
```

**Level 3: The Runtime is a Graph**
```graphoid
# Function calls traverse a call graph
# Modules are subgraphs with import/export edges
# The entire execution environment is graph-based
```

### Configuration Directives: Per-File Control

Want different behavior in different parts of your codebase? Use **configuration directives**:

```graphoid
# Use 32-bit integers for memory-constrained systems
:32bit

# Strict type checking for critical code
:strict

# Relaxed mode for quick scripts
:relaxed

# Debug mode with extra validation
:debug

# Combine directives
:32bit :strict :debug
```

Each file can have its own configuration. Mix strict types in your core library with relaxed scripting for your build tools.

### Rules: Self-Enforcing Invariants

Instead of scattering validation code everywhere, **declare rules once** and let the data structure enforce them:

```graphoid
# Traditional approach - validation everywhere
fn add_user(graph, user_id, age) {
    if age < 0 or age > 150 {
        raise "Invalid age"
    }
    if graph.has_node(user_id) {
        raise "User already exists"
    }
    graph.add_node(user_id, age)
}

# Graphoid approach - rules handle it automatically
users = graph { type: :dag }
users.add_rule("validate_range", :value, 0, 150)
users.add_rule("no_cycles")
users.add_node("alice", 32)  # Rules enforced automatically
```

**Built-in rules** include:
- `no_cycles`, `no_self_loops`, `connected`
- `single_root`, `max_children_N`, `min_children_N`
- `validate_range`, `enforce_type`, `none_to_zero`

**Custom rules** let you define your own invariants:
```graphoid
fn validate_email(graph) {
    for node in graph.nodes() {
        if not node.value().contains("@") {
            return false
        }
    }
    return true
}

contacts = graph{}
contacts.add_rule(validate_email)
# Now every modification is validated automatically
```

Your invariants become **self-enforcing**. The data structure does the work.

### Behaviors: Automatic Data Transformation

Data structures can **transform values automatically** as they're added:

```graphoid
# Handle missing data transparently
temperatures = [98.6, none, 102.5]
temperatures.add_rule("none_to_zero")
print(temperatures)  # [98.6, 0, 102.5]

# Automatic range clamping
scores = []
scores.add_rule("validate_range", 0, 100)
scores.append(150)  # Automatically clamped to 100

# Value mapping with defaults
color_map = {"red": 1, "green": 2, "blue": 3}
colors = ["red", "purple", "blue"]
colors.add_mapping_rule(color_map, 0)
print(colors)  # [1, 0, 3] - "purple" maps to default 0
```

No more scattered `if none then 0` checks throughout your code.

### Named Transformations: Readable Functional Code

Instead of cryptic lambdas, use **named transformations** that read like English:

```graphoid
numbers = [1, 2, 3, 4, 5]

# Clear and self-documenting
numbers.map("double")          # [2, 4, 6, 8, 10]
numbers.map("square")          # [1, 4, 9, 16, 25]
numbers.filter("even")         # [2, 4]
numbers.reject("negative")     # [1, 2, 3, 4, 5]

# Chain operations naturally
numbers
    .filter("positive")
    .map("double")
    .reject("even")
```

Built-in transformations include: `double`, `triple`, `square`, `negate`, `abs`, `inc`, `dec`, `halve`, `reciprocal`, `sqrt`, and more.

Lambdas still work when needed (`numbers.map(x => x * 3)`), but named transformations improve readability.

### String Generators: Consistency Everywhere

Just as lists have generators, so do strings:

```graphoid
# Repetition mode
padding = string.generate(" ", 10)        # "          "
separator = string.generate("-", 40)      # "----------------------------------------"

# Sequence mode (like list.generate)
lowercase = string.generate("a", "z")     # Full alphabet
digits = string.generate("0", "9")        # "0123456789"

# Use in formatting
fn box(text) {
    border = string.generate("-", text.length() + 4)
    return border + "\n| " + text + " |\n" + border
}
```

The API mirrors list generators: `list.generate(1, 100)` and `string.generate("a", "z")` work the same way.

---

## Professional Features (Built-In!)

### RSpec-Style Testing Framework

Testing is **built into the language**, not bolted on:

```graphoid
import "spec"

describe "Calculator" {
    describe "add" {
        it "adds two positive numbers" {
            result = calculator.add(2, 3)
            expect(result).to_equal(5)
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

Natural language expectations: `to_equal`, `to_be_truthy`, `to_contain`, `to_raise`, `to_match`.

### Powerful Graph Algorithms

Graph operations are first-class:

```graphoid
# Shortest path
path = network.shortest_path("A", "B")

# Connected components
components = graph.connected_components()

# Topological sort (for DAGs)
order = dag.topological_sort()

# Cycle detection
has_cycle = graph.has_cycle()

# Community detection
communities = social_network.detect_communities()
```

### Simple, Clean Syntax

Graphoid follows the **KISS principle** - no unnecessary verbosity:

```graphoid
# Print something
print("Hello")

# No mutations unless you ask for them
a = [1, 2, 3]
b = a.append(4)   # Returns new list, 'a' unchanged
c = a.append!(4)  # Mutates 'a' (note the '!' suffix)

# Type inference by default
name = "Alice"              # Infers string
items = [1, 2, 3]           # Infers list

# Explicit types when you want them
list<num> scores = [95, 87, 92]
string username = input()
```

**Principle of Least Surprise**: Operations are immutable by default. If you don't see `!`, nothing changes.

---

## Who Should Explore Graphoid?

Graphoid is ideal for:

- **Experimenters**: Try new ideas in a fresh environment
- **Learners**: Study language design and graph algorithms
- **Contributors**: Help shape an emerging language
- **Graph enthusiasts**: Work with graphs as a core abstraction
- **AI-interested developers**: See AI-assisted development in action

Graphoid is **not yet ready** for:

- Production systems (it's in alpha!)
- Mission-critical applications
- Projects requiring a mature ecosystem
- Teams needing stability guarantees

---

## Current Status (November 2025)

**Phase 13: Documentation & Publishing** (~35% complete)

**What Works Today**:
- ✅ Core language (variables, functions, control flow)
- ✅ Collections (lists, hashes, trees)
- ✅ Graph types and algorithms
- ✅ Module system
- ✅ Pattern matching
- ✅ Configuration directives
- ✅ Rules and behaviors
- ✅ 11 standard library modules

**In Active Development**:
- 🔄 Documentation (user guide + API reference)
- 🔄 Native stdlib modules (crypto, time, system)
- 🔄 Example programs collection

**Coming Next**:
- 🔲 Package manager
- 🔲 Interactive debugger with graph visualization
- 🔲 Testing framework implementation
- 🔲 Performance optimizations
- 🔲 VSCode extension

**Test Coverage**: 1,600+ passing tests

---

## Try Graphoid

```bash
# Clone the repository
git clone https://github.com/your-org/graphoid.git
cd graphoid/rust

# Build the compiler
cargo build --release

# Install
make install

# Run the REPL
gr

# Execute a program
gr samples/01-basics/hello_world.gr
```

**Learn More**:
- [Quick Start Guide](user-guide/01-getting-started.md)
- [Language Specification](../dev_docs/LANGUAGE_SPECIFICATION.md)
- [API Reference](api-reference/README.md)
- [Example Programs](../rust/samples/)

---

## Get Involved

Graphoid is **open for participation**:

- **Try it**: Download and experiment
- **Report issues**: Found a bug? Let us know
- **Suggest features**: Have ideas? Share them
- **Contribute code**: PRs welcome
- **Write examples**: Show what's possible
- **Improve docs**: Help others learn

This is your chance to influence a programming language from its early stages. Your feedback, ideas, and contributions shape what Graphoid becomes.

---

## The Vision

Graphoid explores what's possible when:

1. **Graphs are fundamental** - Not a library, but the foundation
2. **Abstraction is powerful** - Rules and behaviors reduce boilerplate
3. **Readability matters** - Named transformations, clear syntax
4. **AI assists from day one** - Rapid, comprehensive development
5. **Configuration is flexible** - Per-file directives give fine control
6. **Data is self-aware** - Structures understand and enforce their own rules

It's early days, but the potential is exciting. Come explore with us.

---

**Questions?** Open an issue on GitHub or check the documentation.

*Last updated: November 2025*

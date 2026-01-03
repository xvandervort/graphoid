# Graphoid

**A graph-theoretic programming language where everything is a graph.**

> **Note:** Graphoid is an experimental language and a work in progress. It's also an experiment in AI-assisted development—the vast majority of this codebase was written through collaboration between a human designer and Claude (Anthropic's AI assistant). We're exploring what's possible when AI handles implementation while humans focus on vision and architecture.

Graphoid is a modern programming language built on a radical premise: graphs shouldn't be bolted onto a language—they should *be* the language. Lists, maps, trees, and even the runtime itself are all graphs under the hood.

```graphoid
# Create a social network
network = graph { type: :directed }

network.add_node("alice", {name: "Alice", age: 30})
network.add_node("bob", {name: "Bob", age: 25})
network.add_edge("alice", "bob", "FRIEND")

# Find all friendships using pattern matching
friends = network.match(
    node("person"),
    edge(type: "FRIEND"),
    node("friend")
)
```

## Features

### Graphs as First-Class Citizens

Every data structure is a graph. A list `[1, 2, 3]` is internally `Node(1) → Node(2) → Node(3)`. This isn't just theory—it's how the language actually works.

```graphoid
# These are all graphs
items = [1, 2, 3]              # Linked graph
config = {"host": "localhost"} # Hash graph
tree = tree{}                  # Tree graph
network = graph { type: :dag } # Explicit graph
```

### Pattern Matching

Powerful pattern matching on values, lists, and graphs:

```graphoid
fn describe(value) {
    return match value {
        [] => "empty",
        [x] => "single item",
        [head, ...tail] => "list with head and tail",
        _ => "something else"
    }
}
```

### Graph Pattern Matching

Query your graphs with explicit, readable pattern syntax:

```graphoid
# Find friends of friends
fof = network.match(
    node("user"),
    edge(type: "FRIEND"),
    node("intermediate"),
    edge(type: "FRIEND"),
    node("friend_of_friend")
)
```

### Class-Like Graphs

Graphs can have attached methods, enabling object-oriented patterns:

```graphoid
graph Counter {
    count: 0

    fn new(initial) {
        instance = self.clone()
        instance.count = initial
        return instance
    }

    fn increment() {
        count = count + 1
        return self
    }

    fn value() {
        return count
    }
}

c = Counter.new(0)
c.increment().increment()
print(c.value())  # 2
```

### Functional Programming

Transform data with expressive functional operations:

```graphoid
numbers = [1, 2, 3, 4, 5]

numbers.map(x => x * 2)           # [2, 4, 6, 8, 10]
numbers.filter(x => x > 2)        # [3, 4, 5]
numbers.reduce(0, (acc, x) => acc + x)  # 15

# Chaining
numbers.filter(x => x % 2 == 0).map(x => x * x)  # [4, 16]
```

### HTTPS & TLS Support

Full TLS 1.3 implementation in pure Graphoid:

```graphoid
import "http"

response = http.get("https://api.example.com/data")
print(response["body"])
```

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/graphoid.git
cd graphoid

# Build with Cargo
cargo build --release

# Run the REPL
cargo run

# Or run a file
cargo run -- examples/hello.gr
```

### Hello World

```graphoid
print("Hello, World!")

name = "Alice"
age = 25
print(name, "is", age, "years old")
```

### Your First Graph

```graphoid
# Create a directed graph
g = graph { type: :directed }

# Add nodes with values
g.add_node("A", 100)
g.add_node("B", 200)
g.add_node("C", 150)

# Connect them
g.add_edge("A", "B", "connects")
g.add_edge("B", "C", "connects")

print("Nodes:", g.nodes())
print("Edges:", g.edges())
```

## Standard Library

Graphoid includes a good and growing standard library:

| Module | Description |
|--------|-------------|
| `math` | Mathematical functions |
| `statistics` | Statistical analysis |
| `json` | JSON parsing and generation |
| `http` | HTTP/HTTPS client |
| `tls` | TLS 1.3 implementation |
| `io` | File I/O operations |
| `csv` | CSV parsing |
| `time` | Date and time handling |
| `regex` | Regular expressions |
| `collections` | Advanced collection utilities |

```graphoid
import "json"
import "statistics"

data = json.parse('{"values": [1, 2, 3, 4, 5]}')
print(statistics.mean(data["values"]))  # 3.0
```

## Current Status

Graphoid is in active development with core features complete:

- Lexer, Parser, AST
- Value system and execution engine
- Functions, lambdas, closures
- Collections (lists, maps, trees, graphs)
- Graph pattern matching
- Module system with imports
- Pattern matching on values and structures
- Standard library (19 modules)
- TLS 1.3 and HTTPS support
- **2,200+ tests passing**

### Roadmap

- [ ] RSpec-style testing framework
- [ ] Interactive debugger
- [ ] Package manager
- [ ] Self-hosting compiler
- [ ] Other stuff at Dave's discretion.

## Examples

The `samples/` directory contains many examples:

```
samples/
├── 01-basics/          # Hello world, functions, collections
├── 02-intermediate/    # Pattern matching, behaviors, bitwise ops
├── 03-advanced/        # Graph patterns, class-like graphs
├── 04-modules/         # Module system examples
├── 05-stdlib/          # Standard library usage
└── 06-projects/        # Complete applications
```

Run any example:

```bash
cargo run -- samples/01-basics/hello_world.gr
```

## Why Graphoid?

Traditional languages treat graphs as an afterthought—a library you import when needed. Graphoid inverts this: **graphs are the primitive**, and lists, maps, and trees are just graphs with constraints.

This design enables:

- **Unified data model**: Everything responds to graph operations
- **Natural pattern matching**: Query any data structure the same way
- **Self-aware collections**: Data understands its own structure
- **Intrinsic behaviors**: Collections can automatically transform values

```graphoid
# Add a rule to validate all values
prices = [99.99, 149.99, 199.99]
prices.add_rule("validate_range", 0, 1000)
prices.append(1500)  # Automatically clamped to 1000
```

## Documentation

- [Language Specification](dev_docs/LANGUAGE_SPECIFICATION.md)
- [Design Philosophy](docs/DESIGN_PHILOSOPHY.md)
- [Architecture](dev_docs/ARCHITECTURE_DESIGN.md)

## Contributing

Contributions are welcome! Please read the contributing guidelines and ensure:

- Tests pass: `cargo test --lib`
- No warnings: `cargo build`
- Code follows existing patterns

## License

Graphoid is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

**Graphoid**: Where graphs aren't a feature—they're the foundation.

# Glang

> **"Everything is a graph"** - A prototype programming language where graphs are the fundamental abstraction at every level.

## Overview

Glang takes the radical approach that **everything should be a graph** - not just data structures, but the entire runtime environment:

- 📊 **Data structures** are graphs (lists are linear graphs: `A -> B -> C`)
- 📛 **Variables** are stored in a graph (namespace meta-graph with name->value edges)  
- 🔗 **Relationships** between program elements use graph connections
- 🔍 **Introspection** allows examining graph structures at runtime

### Design Principles
- **Intuitive structure** - Graph concepts feel natural
- **Stability** - Consistent graph-based behavior at all levels
- **Flexibility** - Easy transformation between graph types
- **Self-consistency** - The same graph principles apply everywhere

This prototype is implemented in Python for rapid development and testing.

## Installation

### Prerequisites
- Python 3.8 or higher

### Setup

```bash
# Clone the repository
git clone <repository-url> (There isn't one yet)
cd glang

# Create and activate virtual environment
python -m venv .venv
source .venv/bin/activate  # On Linux/Mac
# or
.venv\Scripts\activate     # On Windows

# Install in development mode with all dependencies
pip install -e ".[dev]"
```

### Activating the Environment Later

```bash
# Navigate to project directory
cd grang

# Activate virtual environment
source .venv/bin/activate  # On Linux/Mac
# or  
.venv\Scripts\activate     # On Windows
```

## Usage

### Running the REPL

```bash
# Using the installed command
glang

# Or run as a Python module
python -m glang.repl
```

### Available REPL Commands

#### Graph Management
```bash
# Create graphs from data
create fruits [apple, banana, cherry]
create numbers [1, 2, 3, 4, 5] 
create empty directed

# View and manipulate
graphs           # List all variables
show fruits      # Visualize: [apple] -> [banana] -> [cherry]
append orange    # Add to current graph
traverse         # Show: [apple, banana, cherry, orange]
```

#### Meta-Graph Introspection
```bash
namespace        # 🤯 Show the variable storage graph itself!
stats            # Statistics about the namespace meta-graph
info fruits      # Detailed information about a variable
```

#### Basic Commands
- `ver`/`version` - Show version information
- `h`/`help` - Show all available commands
- `x`/`exit` - Exit the REPL

## Development

**Note:** Make sure your virtual environment is activated before running development commands.

### Running Tests

```bash
pytest test/
```

### Code Formatting

```bash
black src/ test/
```

### Type Checking

```bash
mypy src/
```

## The Graph Philosophy in Action

### Example Session
```bash
$ glang
glang> create fruits [apple, banana, cherry]
Created linear graph 'fruits' with 3 elements

glang> namespace
Variable Namespace Graph:
  Variables: 1
  Total nodes: 2
  Assignment edges: 1
  📛 fruits -> 📊 linear graph (3 nodes)

glang> append orange
Appended orange to 'fruits'

glang> show
Graph 'fruits' (linear):
[apple] -> [banana] -> [cherry] -> [orange]

glang> stats
Variable namespace statistics:
  Variables: 1
  Total data nodes: 4
  Total data edges: 3
  Namespace nodes: 2
  Assignment edges: 1
  Graph types:
    linear: 1
```

### What Makes This Special?

Notice how `namespace` shows the **variable storage itself as a graph**:
- `📛 fruits` is a name node
- `📊 linear graph` is a value node  
- There's an assignment edge connecting them
- The `stats` command reveals both the data graphs AND the meta-graph structure

This isn't just storing graphs in variables - **the variable system itself is a graph**!

## Project Status

### ✅ Completed (Phase 1)
- [x] Core graph infrastructure (Node, Edge, Graph classes)
- [x] Linear graph operations (append, prepend, insert, delete, traverse)
- [x] **Variable namespace as meta-graph** (VariableGraph system)
- [x] REPL with graph commands and visualization
- [x] Comprehensive test suite (60+ tests, 67% coverage)
- [x] Graph introspection capabilities

### 🚧 Next Phase
- [ ] Tree structures and hierarchical graphs
- [ ] Weighted graphs and pathfinding algorithms  
- [ ] Cyclic graphs and loop detection
- [ ] Graph query language
- [ ] Cross-graph references and dependencies
- [ ] Function definitions as graph nodes
- [ ] Module system using graph connections

## Contributing

When contributing to glang, remember the core philosophy: **"Everything is a graph"**

- New features should be graph-native, not traditional structures with graphs bolted on
- Consider how your feature fits into the meta-graph architecture
- Provide introspection capabilities for any new graph structures
- Test both functional behavior and graph structure integrity

See `CLAUDE.md` for detailed development guidelines.

## License

MIT

# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Glang** is a prototype programming language where **everything is a graph**. This fundamental philosophy permeates every aspect of the language:

### Core Philosophy: "Graphs All The Way Down"
- **Data structures** are graphs (lists, trees, databases)
- **Variable storage** is a graph (namespace meta-graph)
- **Runtime environment** uses graph-based relationships
- **Future features** will extend this to modules, functions, and program structure

### Design Principles:
- **Intuitive structure and syntax** - Graph concepts should feel natural
- **Stability** - Consistent graph-based behavior at all levels
- **Flexibility** - Easy transformation between graph types
- **Introspection** - Ability to examine graph structures at runtime

This prototype is implemented in Python for rapid development and testing.

## Repository Structure

- `src/glang/` - Main language package
  - `core/` - Core graph data structures
    - `node.py` - Graph nodes with unique IDs and data
    - `edge.py` - Directed edges with metadata
    - `graph.py` - Graph container with linear list operations
    - `variable_graph.py` - Meta-graph for variable namespace
    - `graph_types.py` - Extensible graph type system
  - `repl/` - REPL implementation
    - `repl.py` - Main REPL with graph commands
    - `graph_manager.py` - Variable management using VariableGraph
  - `visualization/` - Graph rendering and display
  - `cli.py` - Command-line interface
- `test/` - Comprehensive test suite (60+ tests, 67% coverage)
- `doc/` - Documentation files

## Development Setup

This project uses a Python virtual environment. Make sure it's activated before running commands:

```bash
# Activate virtual environment (if not already activated)
source .venv/bin/activate  # On Linux/Mac
# or
.venv\Scripts\activate     # On Windows
```

## Development Commands

```bash
# Run the REPL (with full navigation: ↑/↓ history, tab completion)
python -m glang.repl
# or after installation:
glang

# Quick demo (scripted)
echo -e "create fruits [apple, banana]\\nnamespace\\nstats\\nexit" | glang

# Run tests
pytest test/

# Install in development mode with all dependencies
pip install -e ".[dev]"

# Code formatting
black src/ test/

# Type checking
mypy src/

# Demonstrate the philosophy
echo -e "create fruits [apple, banana]\nnamespace\nstats\nexit" | glang
```

## REPL Commands

### Basic Commands
- `ver`/`version` - Show version information
- `h`/`help` - Show help information  
- `x`/`exit` - Exit the REPL

### Graph Management
- `create <name> [1,2,3]` - Create graph from list
- `graphs` - List all graphs
- `show [name]` - Show graph structure
- `traverse [name]` - Show graph traversal
- `delete <name>` - Delete a graph
- `info [name]` - Show detailed variable info

### Variable Namespace (Meta-Graph)
- `namespace` - **Show the variable graph itself**
- `stats` - Show namespace statistics

### Modern Syntax (Phase 4A & 4B)

#### Variable Declarations with Type Constraints
```bash
# Basic declarations
list fruits = [apple, banana, cherry]
list numbers = [1, 2, 3, 4, 5]

# Type-constrained declarations  
list<num> scores = [95, 87, 92]
list<string> names = [alice, bob, charlie]
list<bool> flags = [true, false, true]
list<list> matrix = [[1, 2], [3, 4]]
```

#### Advanced Indexing & Assignment
```bash
# Index access
fruits[0]          # Get first element
numbers[-1]        # Get last element
matrix[1][0]       # Multi-dimensional access

# Index assignment
fruits[0] = mango
scores[1] = 99
matrix[0][1] = 42

# Slice access
numbers[1:4]       # Elements 1 through 3
data[::2]          # Every 2nd element
items[2:]          # From index 2 to end
text[:-1]          # All but last element

# Slice assignment
fruits[1:3] = [kiwi, orange]
numbers[::2] = [10, 30, 50]
```

#### Method Calls with Type Safety
```bash
# Basic methods (with constraint enforcement)
scores.append 88      # ✓ Valid (num)
scores.append hello   # ✗ Error (string != num constraint)

# Type introspection methods
scores.constraint()              # Show type constraint
scores.validate_constraint()     # Validate all elements
scores.type_summary()           # Count elements by type
scores.types()                  # List all element types
mixed.coerce_to_constraint()    # Attempt type coercion
```

### Legacy Commands (Still Supported)
- `create <name> [1,2,3]` - Create graph from list
- `graphs` - List all graphs
- `show [name]` - Show graph structure
- `traverse [name]` - Show graph traversal
- `delete <name>` - Delete a graph
- `info [name]` - Show detailed variable info

### Graph Operations
- `append <value>` - Add to end
- `prepend <value>` - Add to beginning
- `insert <index> <val>` - Insert at position
- `reverse` - Reverse the graph

## Architecture Notes

### Graph-Based Development Philosophy

When developing glang features, **think in graphs**:

1. **Variable Storage**: Variables aren't stored in a hash table - they're nodes in a VariableGraph with assignment edges connecting names to values.

2. **Data Structures**: A list `[1,2,3]` is internally `Node(1) -> Node(2) -> Node(3)` - a simple directed graph.

3. **Extensibility**: New features should leverage the graph infrastructure. Examples:
   - Functions could be graph nodes with parameter/return edges
   - Modules could be subgraphs with import/export edges
   - Error handling could use exception graphs

4. **Introspection**: Users can examine the meta-structure:
   ```bash
   glang> create nums [1,2,3]
   glang> namespace  # Shows variable graph structure
   glang> stats      # Shows meta-graph statistics
   ```

5. **Testing**: Test both data-level graphs AND meta-level graph structures.

### Current Implementation Status
- ✅ Core graph infrastructure (Node, Edge, Graph classes)
- ✅ Linear graphs (list operations)
- ✅ Variable namespace as VariableGraph  
- ✅ REPL with graph visualization
- ✅ Comprehensive test suite
- 🚧 Tree structures, weighted graphs, cyclic graphs
- 🚧 Graph query language
- 🚧 Cross-graph references and dependencies

### Development Guidelines
- **New features should be graph-native** - don't bolt graphs onto traditional structures
- **Maintain the "everything is a graph" philosophy** at all levels
- **Provide introspection capabilities** for any new graph structures
- **Test both functional behavior and graph structure integrity**
# Call Graph Debugging and Introspection

Glang provides powerful call graph introspection capabilities that let you visualize and analyze your program's function structure. This is especially useful for debugging complex function relationships and understanding program flow.

## Quick Start

Import the call graph module and start exploring:

```glang
import "call_graph" as cg

# See what functions exist
funcs = cg.get_reachable_functions()
print("Available functions: " + funcs.to_string())

# Visualize the call graph
viz = cg.visualize()
print(viz)
```

## Call Graph Module Functions

### Basic Information

#### `current_scope()` → string
Get the current scope name:
```glang
scope = cg.current_scope()
print("Currently in: " + scope)  # "Currently in: global"
```

#### `count_functions([scope])` → number
Count functions in a scope (or all scopes if no scope given):
```glang
total = cg.count_functions()           # All functions
global_count = cg.count_functions("global")  # Just global functions
```

#### `list_scopes()` → list<string>
Get all available scopes:
```glang
scopes = cg.list_scopes()
for scope in scopes {
    count = cg.count_functions(scope)
    print(scope + ": " + count.to_string() + " functions")
}
```

### Function Discovery

#### `get_reachable_functions([scope])` → list<string>
Get all functions you can call from current or specified scope:
```glang
# From current scope
funcs = cg.get_reachable_functions()

# From specific scope
module_funcs = cg.get_reachable_functions("MyModule")
```

#### `get_function_info(name, [scope])` → hash
Get detailed information about a function:
```glang
info = cg.get_function_info("calculate")
if info != none {
    print("Function: " + info["name"])
    print("Scope: " + info["scope"])
    print("Parameters: " + info["parameters"].to_string())
    print("Connected to: " + info["connected_functions"].to_string())
}
```

#### `find_path(from_func, to_func, [scope])` → list<string> | none
Find a path between two functions:
```glang
path = cg.find_path("main", "helper")
if path != none {
    print("Path: " + path.to_string())  # "Path: [main, helper]"
}
```

### Visualization

#### `visualize([format])` → string
Generate call graph visualization in different formats:

- `"text"` (default) - Human-readable text
- `"dot"` - Graphviz DOT format
- `"mermaid"` - Mermaid diagram syntax

```glang
# Text format (default)
text_viz = cg.visualize()
print(text_viz)

# DOT format for Graphviz
dot_viz = cg.visualize("dot")
print(dot_viz)  # Save this to a .dot file

# Mermaid format for documentation
mermaid_viz = cg.visualize("mermaid")
print(mermaid_viz)  # Paste into Mermaid editor
```

#### `visualize_scope([scope])` → string
Focus on a specific scope:
```glang
# Current scope
current_viz = cg.visualize_scope()

# Specific module
module_viz = cg.visualize_scope("DataProcessor")
print(module_viz)
```

## REPL Debugging Examples

### Exploring Function Relationships

```glang
# Start REPL and define some functions
glang> func main() { process_data() }
glang> func process_data() { validate() }
glang> func validate() { check_format() }
glang> func check_format() { return true }

# Import call graph
glang> import "call_graph" as cg

# See what we have
glang> cg.count_functions()
4

glang> cg.get_reachable_functions()
[check_format, main, process_data, validate]

# Visualize the structure
glang> cg.visualize()
==================================================
COMPLETE CALL GRAPH
==================================================

[global]
  check_format
    → main
    → process_data
    → validate
  main
    → check_format
    → process_data
    → validate
  process_data
    → check_format
    → main
    → validate
  validate
    → check_format
    → main
    → process_data
```

### Module Debugging

```glang
# Load a module
glang> load "my_calculator.gr"

# See what scopes we have now
glang> cg.list_scopes()
[Calculator, global]

# Focus on the module
glang> cg.visualize_scope("Calculator")
Call Graph - Scope: Calculator
========================================
Functions in Calculator: 3
  - add
    Connected to: multiply, subtract
  - multiply
    Connected to: add, subtract
  - subtract
    Connected to: add, multiply

# Get detailed info about a function
glang> info = cg.get_function_info("add", "Calculator")
glang> info["parameters"]
[x, y]
glang> info["connected_functions"]
[multiply, subtract]
```

### Path Finding for Complex Flows

```glang
# Define a complex flow
glang> func start() { preprocess() }
glang> func preprocess() { validate_input() }
glang> func validate_input() { parse_data() }
glang> func parse_data() { finish() }
glang> func finish() { return "done" }

# Find the path from start to finish
glang> path = cg.find_path("start", "finish")
glang> path.to_string()
[start, finish]  # Direct connection since they're in same scope

# Check if a function is reachable
glang> path = cg.find_path("finish", "start")
glang> path != none
true  # Functions are bidirectionally connected in same scope
```

### Debugging Module Interactions

```glang
# When functions can't find each other, debug with call graph
glang> load "module_a.gr"  # defines process()
glang> load "module_b.gr"  # defines helper()

# Check if they can see each other
glang> cg.find_path("process", "helper")
none  # They're in different scopes!

# See what each module contains
glang> cg.visualize_scope("ModuleA")
glang> cg.visualize_scope("ModuleB")

# Check what's reachable from each scope
glang> cg.get_reachable_functions("ModuleA")
[process, other_func]  # Only ModuleA functions

glang> cg.get_reachable_functions("ModuleB")
[helper, utility]      # Only ModuleB functions
```

## Export Formats

### Graphviz DOT
Save the DOT output to visualize with Graphviz tools:

```glang
dot_output = cg.visualize("dot")
# Save dot_output to file.dot, then:
# dot -Tpng file.dot -o graph.png
```

### Mermaid Diagrams
Use Mermaid output in documentation:

```glang
mermaid_output = cg.visualize("mermaid")
# Paste into https://mermaid.live/ or GitHub markdown
```

## Common Debugging Patterns

### "Function Not Found" Errors
```glang
# When you get "Function 'foo' not found"
glang> cg.get_reachable_functions()  # See what's available
glang> cg.list_scopes()              # Check if it's in another scope
glang> cg.find_path("current_func", "foo")  # See if there's a path
```

### Understanding Module Structure
```glang
# Explore a loaded module
glang> scopes = cg.list_scopes()
glang> for scope in scopes { print(scope + ": " + cg.count_functions(scope).to_string()) }
glang> cg.visualize_scope("ModuleName")  # Deep dive into specific module
```

### Checking Function Connectivity
```glang
# See what a function can reach
glang> info = cg.get_function_info("my_function")
glang> info["connected_functions"]
glang> info["reachable"]
```

The call graph system gives you unprecedented visibility into your program's structure, making debugging and understanding complex function relationships much easier!
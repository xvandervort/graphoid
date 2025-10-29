# Graph-Based Functions in Graphoid

**Version**: 1.0
**Last Updated**: January 2025
**Status**: Design specification for graph-aware function implementation

This document explains how Graphoid implements functions as first-class graph citizens, making "everything is a graph" apply to the runtime environment itself.

---

## Table of Contents

1. [Philosophy: Why Functions as Graphs?](#philosophy-why-functions-as-graphs)
2. [The Three Levels Applied to Functions](#the-three-levels-applied-to-functions)
3. [Architecture Overview](#architecture-overview)
4. [Function Nodes](#function-nodes)
5. [Call Edges](#call-edges)
6. [Closures as Subgraph Connections](#closures-as-subgraph-connections)
7. [The Call Stack as a Path](#the-call-stack-as-a-path)
8. [Parameter Passing as Labeled Edges](#parameter-passing-as-labeled-edges)
9. [Implementation Details](#implementation-details)
10. [Integration with Five-Layer Architecture](#integration-with-five-layer-architecture)
11. [Unique Features Enabled](#unique-features-enabled)
12. [Examples](#examples)

---

## Philosophy: Why Functions as Graphs?

### The Problem with Traditional Function Implementation

Most languages treat functions as isolated units:
- Function calls are stack frames (linear, opaque)
- Closures are "magic" captured environments
- Call relationships are implicit
- Debugging requires special tools to reconstruct call graphs
- Profiling requires instrumentation

### The Graph-Native Solution

Graphoid makes the call graph **explicit and first-class**:
- Functions are **nodes** in a runtime graph
- Function calls create **edges** between caller and callee
- The call stack **is** a path through this graph
- Closures are **subgraph connections** to captured variables
- Recursion is a **cycle** in the graph
- The entire runtime is **introspectable** as a graph

**Core Insight**: If everything is a graph, then the runtime environment (functions, calls, scope) must also be a graph. Not simulated, not metaphorical - **actually implemented** as a graph.

---

## The Three Levels Applied to Functions

Recall Graphoid's three levels of graph abstraction (from LANGUAGE_SPECIFICATION.md):

### Level 1: Data Structures as Graphs
Already implemented:
- Lists are linear graphs (`[1,2,3]` → Node₁→Node₂→Node₃)
- Maps are key-value graphs
- Trees and DAGs are explicit graph types

### Level 2: Variable Storage as Graphs (Meta-Graph)
Already implemented:
- Variables are nodes in a namespace graph
- Assignment creates edges: `x = 5` creates edge `📛"x" → 📊Value(5)`
- The namespace IS a graph that can be inspected

### Level 3: Runtime Environment as Graphs ⭐ **THIS DOCUMENT**
Functions and execution as graphs:
- **Functions are nodes** with identity in a global function graph
- **Function calls create edges** (caller → callee)
- **Call stack is a path** through the function graph
- **Recursion is a cycle** in the call graph
- **Closures are subgraph connections** to captured variables
- **Modules are subgraphs** with import/export edges

---

## Architecture Overview

### The Global Function Graph

Every Graphoid program has a global **FunctionGraph** that tracks:

```
┌─────────────────────────────────────┐
│       Global Function Graph         │
├─────────────────────────────────────┤
│ Nodes: All defined functions        │
│ Edges: Call relationships           │
│ Path:  Current call stack            │
│ Metadata: Profiling, timing, counts │
└─────────────────────────────────────┘
```

**Key Properties**:
1. **Persistent**: The function graph exists for the lifetime of the program
2. **Introspectable**: User code can query it via `debug.*` functions
3. **Self-updating**: Automatically tracks calls, closures, recursion
4. **Integrated**: Uses the same graph infrastructure as data structures

### Relationship to Other Graphs

```
Program Runtime (Everything is a Graph)
├── Data Layer: Collection graphs (lists, maps, trees)
├── Meta Layer: Variable/namespace graphs
└── Runtime Layer: Function call graph ⭐
    ├── Function nodes (definitions)
    ├── Call edges (invocations)
    └── Closure edges (variable capture)
```

All three layers use the **same underlying graph system** (5-layer architecture from ARCHITECTURE_DESIGN.md).

---

## Function Nodes

### Function as a Graph Node

Every function definition creates a node in the function graph:

```graphoid
fn factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}
```

**Becomes**:
```
┌─────────────────────────────┐
│ Node: "factorial"           │
├─────────────────────────────┤
│ Type: Function              │
│ Parameters: ["n"]           │
│ Body: [Stmt...]             │
│ Captured: {}                │
│ CallCount: 0                │
│ TotalTime: 0.0              │
└─────────────────────────────┘
```

### Node Metadata

Each function node tracks:

| Field | Type | Purpose |
|-------|------|---------|
| `node_id` | String | Unique identifier (e.g., "fn_factorial_0") |
| `name` | Option\<String\> | Function name (None for lambdas) |
| `params` | Vec\<String\> | Parameter names |
| `body` | Vec\<Stmt\> | AST of function body |
| `env` | Rc\<Environment\> | Captured environment (closure) |
| `outgoing_calls` | Vec\<String\> | Functions this calls |
| `incoming_calls` | Vec\<String\> | Functions that call this |
| `call_count` | usize | Total invocations |
| `total_time` | f64 | Cumulative execution time |
| `captured_vars` | Vec\<(String, NodeId)\> | Closure captures |

### Anonymous Functions (Lambdas)

Lambdas get auto-generated IDs:

```graphoid
numbers.map(x => x * 2)
```

Creates node:
```
┌──────────────────────────┐
│ Node: "lambda_0"         │
├──────────────────────────┤
│ Type: Function           │
│ Parameters: ["x"]        │
│ Body: [Return(x * 2)]    │
│ Captured: {}             │
└──────────────────────────┘
```

---

## Call Edges

### Function Call as Edge Creation

Every function call creates an edge in the function graph:

```graphoid
fn main() {
    result = factorial(5)
}

fn factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}
```

**Creates edges**:
```
main ──calls──> factorial
factorial ──calls──> factorial  (recursion!)
```

### Edge Attributes

Call edges carry metadata:

| Attribute | Type | Purpose |
|-----------|------|---------|
| `from` | NodeId | Caller function |
| `to` | NodeId | Callee function |
| `edge_type` | EdgeType::Call | Distinguishes from other edge types |
| `timestamp` | f64 | When call occurred |
| `arguments` | Vec\<Value\> | Arguments passed (for debugging) |
| `return_value` | Option\<Value\> | Return value (after completion) |
| `duration` | Option\<f64\> | Execution time |

### Edge Types

```rust
pub enum FunctionEdgeType {
    /// Direct function call (A calls B)
    Call,

    /// Closure capture (lambda captures variable)
    Captures,

    /// Higher-order function relationship (A passes B as argument)
    PassedTo,

    /// Module import relationship
    Imports,
}
```

---

## Closures as Subgraph Connections

### The Closure Problem

Traditional closures are opaque:
```javascript
// JavaScript
function makeCounter() {
    let count = 0;  // Captured, but how?
    return function() {
        count++;
        return count;
    }
}
```

**Questions**:
- What variables are captured?
- Where is `count` stored?
- What if I capture a large data structure?
- Can I visualize the closure?

### Graph-Based Closure Solution

In Graphoid, closures are **explicit graph connections**:

```graphoid
fn make_counter() {
    count = 0
    return fn() {
        count = count + 1
        return count
    }
}

counter = make_counter()
```

**Creates**:
```
┌─────────────────────────┐      ┌──────────────────────┐
│ Node: make_counter      │      │ Variable: count      │
│ Type: Function          │      │ Value: 0             │
└─────────────────────────┘      └──────────────────────┘
            │                              ▲
            │                              │
            │         ┌────────────────────┘
            │         │ Captures edge
            ▼         │
    ┌─────────────────────────┐
    │ Node: lambda_0          │
    │ Type: Function          │
    │ Captured: [("count", *)]│
    └─────────────────────────┘
```

**Key Insights**:
1. The lambda has an **edge** to the `count` variable
2. The edge type is `FunctionEdgeType::Captures`
3. This is **visible** and **introspectable**
4. Memory leaks (circular captures) are **detectable** by finding cycles

### Closure Introspection

```graphoid
counter = make_counter()

# Query what variables are captured
captured = debug.captured_vars(counter)
# Returns: [("count", <node_id>)]

# Visualize the closure
debug.visualize_closure(counter)
# Shows: lambda_0 ──captures──> count
```

### Multiple Closures Sharing Variables

```graphoid
fn make_counter_pair() {
    count = 0

    inc = fn() { count = count + 1 }
    dec = fn() { count = count - 1 }

    return [inc, dec]
}

pair = make_counter_pair()
```

**Creates**:
```
┌─────────────┐
│ count: 0    │
└──────┬──────┘
       │
   ┌───┴────┐
   │        │
   ▼        ▼
┌─────┐  ┌─────┐
│ inc │  │ dec │
└─────┘  └─────┘
```

Both closures have edges to the same `count` node. This is **visible** in the graph!

---

## The Call Stack as a Path

### Traditional Call Stack (Hidden)

Most languages hide the call stack:
```
factorial(5)
  factorial(4)
    factorial(3)
      factorial(2)
        factorial(1)
```

It's **implicit**, only visible via debugger.

### Graph-Based Call Stack (Explicit)

In Graphoid, the call stack **IS** a path through the function graph:

```graphoid
fn factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}

result = factorial(5)
```

**Call stack as path**:
```
Path in Function Graph:
<root> → factorial → factorial → factorial → factorial → factorial
         (n=5)       (n=4)       (n=3)       (n=2)       (n=1)
```

### Implementation

```rust
pub struct FunctionGraph {
    // ... other fields ...

    /// Current call stack as a path through the graph
    /// Each element is (node_id, arguments)
    call_path: Vec<(String, Vec<Value>)>,
}

impl FunctionGraph {
    /// Push a function call onto the stack (add to path)
    pub fn push_call(&mut self, func_id: String, args: Vec<Value>) {
        self.call_path.push((func_id, args));

        // Create call edge if previous function exists
        if self.call_path.len() > 1 {
            let caller = &self.call_path[self.call_path.len() - 2].0;
            self.add_call_edge(caller.clone(), func_id.clone());
        }
    }

    /// Pop a function return from the stack (remove from path)
    pub fn pop_call(&mut self) -> Option<(String, Vec<Value>)> {
        self.call_path.pop()
    }

    /// Get current call depth
    pub fn call_depth(&self) -> usize {
        self.call_path.len()
    }

    /// Get the current path as a list of function names
    pub fn current_path(&self) -> Vec<String> {
        self.call_path.iter()
            .map(|(id, _)| self.get_function_name(id).unwrap_or(id.clone()))
            .collect()
    }
}
```

### Stack Traces are Graph Paths

When an error occurs, the stack trace is literally the current path:

```graphoid
fn a() { b() }
fn b() { c() }
fn c() { d() }
fn d() {
    x = 1 / 0  # Error!
}

a()
```

**Error output**:
```
RuntimeError: division by zero
  at d() [line 5]
  at c() [line 3]
  at b() [line 2]
  at a() [line 1]

Call path: <root> → a → b → c → d
```

The stack trace **IS** the graph path, traversed in reverse!

---

## Parameter Passing as Labeled Edges

### Basic Parameters

```graphoid
fn greet(name, age) {
    print("Hello " + name + ", age " + age)
}

greet("Alice", 25)
```

**Call edge**:
```
┌──────────────────────────┐
│ Edge: <root> → greet     │
├──────────────────────────┤
│ Type: Call               │
│ Arguments: {             │
│   0: "Alice"             │
│   1: 25                  │
│ }                        │
└──────────────────────────┘
```

### Default Parameters

```graphoid
fn greet(name, greeting = "Hello") {
    print(greeting + " " + name)
}

greet("Bob")
greet("Charlie", "Hi")
```

**Function node metadata**:
```
┌─────────────────────────────────┐
│ Node: greet                     │
├─────────────────────────────────┤
│ Parameters: [                   │
│   {name: "name", default: none} │
│   {name: "greeting",            │
│    default: "Hello"}            │
│ ]                               │
└─────────────────────────────────┘
```

### Named Parameters

```graphoid
fn create_user(name, email, age = 18, active = true) {
    # ...
}

# Positional
create_user("Alice", "alice@example.com")

# Named (future feature)
create_user(name: "Bob", age: 25, email: "bob@example.com")
```

**Call edge with named args**:
```
┌──────────────────────────────────┐
│ Edge: <root> → create_user       │
├──────────────────────────────────┤
│ Type: Call                       │
│ Arguments: {                     │
│   "name": "Bob"                  │
│   "age": 25                      │
│   "email": "bob@example.com"     │
│   "active": true (default)       │
│ }                                │
└──────────────────────────────────┘
```

### Variadic Parameters

```graphoid
fn sum(numbers...) {
    total = 0
    for n in numbers {
        total = total + n
    }
    return total
}

result = sum(1, 2, 3, 4, 5)
```

**Call edge**:
```
┌──────────────────────────────┐
│ Edge: <root> → sum           │
├──────────────────────────────┤
│ Type: Call                   │
│ Arguments: {                 │
│   "numbers": [1, 2, 3, 4, 5] │
│ }                            │
└──────────────────────────────┘
```

---

## Implementation Details

### Core Data Structures

```rust
// In src/execution/function_graph.rs

use std::collections::HashMap;
use std::time::Instant;
use crate::values::{Value, Function};

/// A node in the function graph representing a function definition
#[derive(Debug, Clone)]
pub struct FunctionNode {
    /// Unique identifier for this function node
    pub node_id: String,

    /// The actual function data
    pub function: Function,

    /// Functions this function calls (outgoing edges)
    pub outgoing_calls: Vec<String>,

    /// Functions that call this function (incoming edges)
    pub incoming_calls: Vec<String>,

    /// Number of times this function has been called
    pub call_count: usize,

    /// Total execution time across all calls
    pub total_time: f64,

    /// Variables captured by this function (for closures)
    /// Format: (variable_name, variable_node_id_in_namespace)
    pub captured_vars: Vec<(String, String)>,

    /// Timestamp when function was defined
    pub defined_at: f64,
}

/// An edge representing a function call
#[derive(Debug, Clone)]
pub struct CallEdge {
    /// Caller function node ID
    pub from: String,

    /// Callee function node ID
    pub to: String,

    /// Edge type (Call, Captures, PassedTo, Imports)
    pub edge_type: FunctionEdgeType,

    /// Arguments passed (for debugging/profiling)
    pub arguments: Vec<Value>,

    /// Return value (Some after call completes, None during call)
    pub return_value: Option<Value>,

    /// When the call started
    pub start_time: f64,

    /// How long the call took (Some after completion, None during call)
    pub duration: Option<f64>,
}

/// Type of relationship between function nodes
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionEdgeType {
    /// Direct function call (A calls B)
    Call,

    /// Closure capture (lambda captures variable)
    Captures,

    /// Higher-order function (A passes B as argument to C)
    PassedTo,

    /// Module import
    Imports,
}

/// The global function graph tracking all functions and calls
pub struct FunctionGraph {
    /// All function nodes indexed by node_id
    nodes: HashMap<String, FunctionNode>,

    /// All call edges
    edges: Vec<CallEdge>,

    /// Current call stack as a path through the graph
    /// Format: (function_node_id, arguments, start_time)
    call_path: Vec<(String, Vec<Value>, Instant)>,

    /// Counter for generating unique node IDs
    next_node_id: usize,

    /// Whether to track profiling data (performance overhead)
    profiling_enabled: bool,
}

impl FunctionGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            call_path: Vec::new(),
            next_node_id: 0,
            profiling_enabled: false,
        }
    }

    /// Register a new function definition
    pub fn register_function(&mut self, function: Function) -> String {
        let node_id = if let Some(name) = &function.name {
            format!("fn_{}_{}", name, self.next_node_id)
        } else {
            format!("lambda_{}", self.next_node_id)
        };
        self.next_node_id += 1;

        let node = FunctionNode {
            node_id: node_id.clone(),
            function,
            outgoing_calls: Vec::new(),
            incoming_calls: Vec::new(),
            call_count: 0,
            total_time: 0.0,
            captured_vars: Vec::new(),
            defined_at: current_time(),
        };

        self.nodes.insert(node_id.clone(), node);
        node_id
    }

    /// Push a function call onto the stack
    pub fn push_call(&mut self, func_id: String, args: Vec<Value>) {
        let start_time = Instant::now();

        // Update call count
        if let Some(node) = self.nodes.get_mut(&func_id) {
            node.call_count += 1;
        }

        // Add to call path
        self.call_path.push((func_id.clone(), args.clone(), start_time));

        // Create call edge if there's a caller
        if self.call_path.len() > 1 {
            let caller_id = self.call_path[self.call_path.len() - 2].0.clone();
            self.add_call_edge(caller_id, func_id, args);
        }
    }

    /// Pop a function return from the stack
    pub fn pop_call(&mut self, return_value: Value) {
        if let Some((func_id, _, start_time)) = self.call_path.pop() {
            let duration = start_time.elapsed().as_secs_f64();

            // Update total time
            if let Some(node) = self.nodes.get_mut(&func_id) {
                node.total_time += duration;
            }

            // Update most recent call edge with return value and duration
            if let Some(edge) = self.edges.iter_mut().rev().find(|e| e.to == func_id) {
                edge.return_value = Some(return_value);
                edge.duration = Some(duration);
            }
        }
    }

    /// Add a call edge between two functions
    fn add_call_edge(&mut self, from: String, to: String, args: Vec<Value>) {
        // Update outgoing/incoming call lists
        if let Some(caller) = self.nodes.get_mut(&from) {
            if !caller.outgoing_calls.contains(&to) {
                caller.outgoing_calls.push(to.clone());
            }
        }

        if let Some(callee) = self.nodes.get_mut(&to) {
            if !callee.incoming_calls.contains(&from) {
                callee.incoming_calls.push(from.clone());
            }
        }

        // Create edge
        let edge = CallEdge {
            from,
            to,
            edge_type: FunctionEdgeType::Call,
            arguments: args,
            return_value: None,
            start_time: current_time(),
            duration: None,
        };

        self.edges.push(edge);
    }

    /// Get current call depth
    pub fn call_depth(&self) -> usize {
        self.call_path.len()
    }

    /// Get current call path as function names
    pub fn current_path(&self) -> Vec<String> {
        self.call_path.iter()
            .filter_map(|(id, _, _)| {
                self.nodes.get(id).and_then(|n| {
                    n.function.name.clone().or(Some(id.clone()))
                })
            })
            .collect()
    }

    /// Detect recursion (function has edge to itself)
    pub fn is_recursive(&self, func_id: &str) -> bool {
        if let Some(node) = self.nodes.get(func_id) {
            node.outgoing_calls.contains(&func_id.to_string())
        } else {
            false
        }
    }

    /// Find all recursive functions
    pub fn find_recursive_functions(&self) -> Vec<String> {
        self.nodes.keys()
            .filter(|id| self.is_recursive(id))
            .cloned()
            .collect()
    }

    /// Get all functions called by a given function
    pub fn get_callees(&self, func_id: &str) -> Vec<String> {
        self.nodes.get(func_id)
            .map(|n| n.outgoing_calls.clone())
            .unwrap_or_default()
    }

    /// Get all functions that call a given function
    pub fn get_callers(&self, func_id: &str) -> Vec<String> {
        self.nodes.get(func_id)
            .map(|n| n.incoming_calls.clone())
            .unwrap_or_default()
    }
}

// Helper function to get current time as f64 (seconds since epoch)
fn current_time() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}
```

### Integration with Executor

The executor must maintain a reference to the global function graph:

```rust
// In src/execution/executor.rs

pub struct Executor {
    // ... existing fields ...

    /// Global function graph tracking all function definitions and calls
    pub function_graph: Rc<RefCell<FunctionGraph>>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            // ... existing initialization ...
            function_graph: Rc::new(RefCell::new(FunctionGraph::new())),
        }
    }

    /// Execute a function call (existing method, enhanced)
    fn call_function(
        &mut self,
        func: &Function,
        args: Vec<Value>,
    ) -> Result<Value> {
        // Register function if not already registered
        let func_id = self.function_graph.borrow_mut().register_function(func.clone());

        // Push call onto graph
        self.function_graph.borrow_mut().push_call(func_id.clone(), args.clone());

        // Execute function body
        // ... existing execution logic ...

        // Pop call from graph with return value
        self.function_graph.borrow_mut().pop_call(return_value.clone());

        Ok(return_value)
    }
}
```

---

## Integration with Five-Layer Architecture

The function graph integrates with Graphoid's five-layer architecture:

### Layer 1: Data Layer
- **Function nodes** store the actual function data (name, params, body)
- **Call edges** connect caller to callee
- **Closure edges** connect lambdas to captured variables

### Layer 2: Behavior Layer
Functions can have behaviors applied to them:
```graphoid
# Add memoization behavior to expensive function
fibonacci.add_behavior("memoize")

# Add logging behavior
process_data.add_behavior("log_calls")
```

### Layer 3: Control Layer
Rules govern function behavior:
```graphoid
# Limit recursion depth
factorial.add_rule("max_recursion_depth", 1000)

# Detect infinite loops
process.add_rule("timeout", 5.0)  # 5 seconds max
```

### Layer 4: Metadata Layer
Automatically tracks:
- Call count per function
- Total execution time
- Call graph structure (who calls whom)
- Closure captures
- Timestamps

### Layer 5: System Boundary Layer (Future)
- File I/O tracking
- Network calls
- External API usage
- FFI boundaries

---

## Unique Features Enabled

### 1. Call Graph Introspection

```graphoid
# Find all functions that call validate()
callers = debug.callers("validate")
# Returns: ["process_user", "process_order", "process_payment"]

# Find all functions called by process()
callees = debug.callees("process")
# Returns: ["validate", "transform", "save"]

# Find recursive functions
recursive = debug.find_recursive()
# Returns: ["factorial", "fibonacci", "traverse_tree"]
```

### 2. Visual Debugging

```graphoid
# Visualize the entire call graph
debug.visualize_calls()

# Visualize call path for specific execution
debug.trace_execution(fn() {
    result = process_data(data)
})
# Shows: <root> → process_data → validate → transform → save
```

### 3. Performance Profiling

```graphoid
# Enable profiling
configure { profiling: true } {
    result = expensive_computation()
}

# Get profiling report
profile.report()
# Shows:
# Function          Calls    Total Time    Avg Time
# ------------------------------------------------
# expensive_comp    1        2.543s        2.543s
# inner_loop        1000     2.100s        0.002s
# validate          1000     0.300s        0.0003s

# Find bottlenecks
hotspots = profile.hotspots()
# Returns: [("inner_loop", 2.1), ("expensive_comp", 2.543)]

# Visualize time spent as graph node sizes
profile.visualize()
```

### 4. Closure Inspection

```graphoid
counter = make_counter()

# What variables does this closure capture?
captured = debug.captured_vars(counter)
# Returns: [("count", 0)]

# Visualize closure relationships
debug.visualize_closure(counter)
# Shows: lambda_0 ──captures──> count (value: 0)

# Detect closure memory leaks
leaks = debug.find_circular_captures()
# Returns closures with circular variable references
```

### 5. Dependency Analysis

```graphoid
# What functions depend on validate()?
deps = debug.function_dependencies("validate")
# Returns all functions that call validate, directly or indirectly

# Build module dependency graph
module_deps = debug.module_dependencies()
# Shows which modules depend on which

# Find dead code (never called functions)
dead = debug.find_dead_functions()
```

### 6. Stack Trace Enhancement

When errors occur, you get **graph-aware stack traces**:

```graphoid
fn a() { b() }
fn b() { c() }
fn c() { d() }
fn d() {
    x = 1 / 0
}

try {
    a()
}
catch as e {
    # Standard stack trace
    print(e.stack_trace())
    # Output:
    #   at d() [line 5]
    #   at c() [line 3]
    #   at b() [line 2]
    #   at a() [line 1]

    # Graph-based enhancements
    print(e.call_path())
    # Output: <root> → a → b → c → d

    print(e.call_graph_distance("a", "d"))
    # Output: 3 (three hops from a to d)
}
```

### 7. Recursive Function Analysis

```graphoid
fn factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}

# Check if recursive
is_rec = debug.is_recursive("factorial")
# Returns: true

# Get maximum recursion depth for an execution
debug.trace_execution(fn() {
    result = factorial(10)
})
max_depth = debug.max_recursion_depth("factorial")
# Returns: 10

# Detect infinite recursion (graph cycle analysis)
configure { max_recursion_depth: 100 } {
    factorial(1000)  # Raises: RecursionError with call graph visualization
}
```

---

## Examples

### Example 1: Simple Function Call Graph

```graphoid
fn main() {
    result = process(42)
}

fn process(x) {
    validated = validate(x)
    return transform(validated)
}

fn validate(x) {
    if x > 0 { return x }
    return 0
}

fn transform(x) {
    return x * 2
}

main()
```

**Resulting call graph**:
```
main
 └─> process
      ├─> validate
      └─> transform
```

**Accessible via**:
```graphoid
debug.visualize_calls()
# Shows the above tree

debug.current_path()
# During execution of transform: ["main", "process", "transform"]
```

### Example 2: Recursive Function

```graphoid
fn factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}

result = factorial(5)
```

**Call graph**:
```
<root>
 └─> factorial(5)
      └─> factorial(4)
           └─> factorial(3)
                └─> factorial(2)
                     └─> factorial(1)
```

**Graph shows recursion**:
```
factorial ──calls──> factorial (cycle!)
```

### Example 3: Higher-Order Functions

```graphoid
fn map(list, func) {
    result = []
    for item in list {
        result.append(func(item))
    }
    return result
}

fn double(x) {
    return x * 2
}

numbers = [1, 2, 3]
doubled = map(numbers, double)
```

**Call graph**:
```
<root>
 └─> map
      └─> double (called 3 times)
```

**Edge types**:
```
<root> ──calls──> map
map ──calls──> double
<root> ──passes──> double (to map as argument)
```

### Example 4: Closures

```graphoid
fn make_adder(x) {
    return fn(y) {
        return x + y
    }
}

add5 = make_adder(5)
result = add5(10)  # Returns 15
```

**Function graph**:
```
┌──────────────┐
│ make_adder   │
└──────┬───────┘
       │ creates
       ▼
┌──────────────┐      ┌───────────┐
│ lambda_0     │──────│ x: 5      │
│ params: [y]  │      │ (captured)│
└──────────────┘      └───────────┘
     captures
```

**Call graph**:
```
<root>
 ├─> make_adder(5)
 └─> lambda_0(10)  [captures x=5]
```

### Example 5: Profiling

```graphoid
configure { profiling: true } {
    fn slow_function() {
        total = 0
        for i in range(1000000) {
            total = total + i
        }
        return total
    }

    fn fast_function() {
        return 42
    }

    fn main() {
        fast_function()
        slow_function()
        fast_function()
    }

    main()
}

profile.report()
```

**Output**:
```
Function Profile Report
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Function         Calls    Total Time    Avg Time
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
slow_function    1        1.234s        1.234s
main             1        1.235s        1.235s
fast_function    2        0.001s        0.0005s
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Call Graph:
main
 ├─> fast_function
 ├─> slow_function
 └─> fast_function

Hotspots (>1s): slow_function (1.234s)
```

---

## Implementation Roadmap

### Phase 1: Core Function Graph (Week 1-2)
- [ ] Create `src/execution/function_graph.rs`
- [ ] Implement `FunctionNode`, `CallEdge`, `FunctionGraph`
- [ ] Integrate with `Executor`
- [ ] Register functions on definition
- [ ] Track calls (push/pop)
- [ ] Basic tests

### Phase 2: Call Stack as Path (Week 2-3)
- [ ] Implement `call_path` tracking
- [ ] Enhance error stack traces with graph path
- [ ] Add `current_path()` introspection
- [ ] Test recursion detection

### Phase 3: Closures as Graph Edges (Week 3-4)
- [ ] Implement closure capture tracking
- [ ] Add `Captures` edge type
- [ ] Track captured variables as edges
- [ ] Test closure introspection

### Phase 4: Advanced Parameters (Week 4-5)
- [ ] Default parameters
- [ ] Variadic parameters (`args...`)
- [ ] Named parameters (future)
- [ ] Parameter metadata in function nodes

### Phase 5: Profiling & Introspection (Week 5-6)
- [ ] Add timing to call edges
- [ ] Implement `debug.callers()`, `debug.callees()`
- [ ] Implement `profile.report()`, `profile.hotspots()`
- [ ] Implement `debug.is_recursive()`, `debug.find_recursive()`

### Phase 6: Visualization (Week 6-7)
- [ ] Implement `debug.visualize_calls()`
- [ ] Implement `debug.visualize_closure()`
- [ ] Implement `profile.visualize()`
- [ ] Export to DOT format for GraphViz

---

## Conclusion

By implementing functions as first-class graph citizens, Graphoid achieves:

1. ✅ **Philosophical Consistency**: Everything truly is a graph
2. ✅ **Unique Features**: Call graph introspection, visual debugging, graph-based profiling
3. ✅ **Better DX**: Clearer error messages, easier debugging, performance insights
4. ✅ **Dogfooding**: The language uses its own graph system for runtime management
5. ✅ **Competitive Advantage**: No other language exposes the call graph this way

**This is what makes Graphoid revolutionary** - not just graph data structures, but a fully graph-based runtime environment.

---

**Next Steps**: Begin implementation in `src/execution/function_graph.rs`

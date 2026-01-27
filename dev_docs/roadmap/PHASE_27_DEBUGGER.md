# Phase 27: Debugger

**Duration**: 10-14 days
**Priority**: Low
**Dependencies**: Phase 16 (Execution Graph), Phase 18 (Complete Graph Model)
**Status**: Ready (can be done after Phase 18)

---

## Goal

Interactive debugging that exposes Graphoid's graph-based execution model. Debugging IS graph inspection - stepping through code is traversing the execution graph, inspecting variables is querying the namespace graph.

**Key principle**: The debugger doesn't just help you debug programs - it reveals how Graphoid actually works. Every debug command maps to a graph operation.

---

## Core Concept: Debugging as Graph Traversal

```
┌─────────────────────────────────────────────────────────────────┐
│  Traditional Debugging          Graphoid Debugging              │
│                                                                 │
│  step                           Advance to next execution node  │
│  locals                         Query namespace subgraph        │
│  stack                          Show call graph path            │
│  breakpoint                     Mark execution graph node       │
│  watch                          Subscribe to namespace edge     │
│                                                                 │
│  Debugging IS graph inspection                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Execution Graph Model

When you debug a program, you're traversing the execution graph (Phase 16):

```
┌─────────────────────────────────────────────────────────────────┐
│  Execution Graph (simplified)                                   │
│                                                                 │
│  ┌────────┐     ┌────────┐     ┌────────┐     ┌────────┐       │
│  │ stmt:1 │────►│ stmt:2 │────►│ call:  │────►│ stmt:4 │       │
│  │ x = 5  │     │ y = 10 │     │ foo()  │     │ z = x+y│       │
│  └────────┘     └────────┘     └───┬────┘     └────────┘       │
│                                    │                            │
│                          call      │                            │
│                                    ▼                            │
│                              ┌────────────┐                     │
│                              │ fn:foo     │                     │
│                              │ body graph │                     │
│                              └────────────┘                     │
│                                                                 │
│  "step" = advance along execution edges                         │
│  "step_into" = follow call edge into function body              │
│  "step_out" = return to caller node                             │
└─────────────────────────────────────────────────────────────────┘
```

### Namespace Graph Model

Variable inspection queries the namespace graph (Phase 15):

```
┌─────────────────────────────────────────────────────────────────┐
│  Namespace Graph at Breakpoint                                  │
│                                                                 │
│  universe:namespaces                                            │
│      │                                                          │
│      └── scope:main                                             │
│              ├── x ──► 5                                        │
│              ├── y ──► 10                                       │
│              │                                                  │
│              └── scope:foo (current)                            │
│                      ├── a ──► 3                                │
│                      └── b ──► 7                                │
│                                                                 │
│  "locals" = query current scope subgraph                        │
│  "globals" = query parent scope chain                           │
│  "watch x" = subscribe to edge universe:namespaces/.../x        │
└─────────────────────────────────────────────────────────────────┘
```

---

## Core Features

### 1. Breakpoints (Execution Graph Markers)

Breakpoints are markers on execution graph nodes:

```graphoid
import "debug"

fn complex_algorithm(data) {
    # Programmatic breakpoint - marks current execution node
    debug.break()

    # Conditional breakpoint - marks node with guard
    debug.break_if(data.length() > 1000, "Large dataset detected")

    for item in data {
        # Break when condition becomes true
        debug.break_if(item.value < 0, "Negative value found")
        process(item)
    }
}
```

Internally, breakpoints are nodes in a debug subgraph:

```
universe:debug
    └── breakpoints/
            ├── bp:1 ──► execution_node:stmt_15
            ├── bp:2 ──► execution_node:stmt_23
            │       └── condition ──► (guard subgraph)
            └── bp:3 ──► execution_node:loop_body
```

### 2. Debug REPL (Graph Query Interface)

When execution pauses, you're querying the runtime graphs:

```
Breakpoint hit at algorithm.gr:15
Execution node: stmt:15 (assignment)
Namespace: scope:complex_algorithm

> help

Graph Navigation:
  step (s)         - Advance to next execution node
  step_into (si)   - Follow call edge into function
  step_out (so)    - Return to caller node
  next (n)         - Advance, skipping call subgraphs
  continue (c)     - Run until next breakpoint

Graph Inspection:
  locals           - Query current namespace subgraph
  globals          - Query ancestor namespace chain
  stack            - Show call graph path to current node
  execution        - Show execution graph around current node

  print <expr>     - Evaluate expression in current namespace
  graph <var>      - Visualize data graph structure
  universe         - Show universe graph overview

Breakpoint Management:
  break <line>     - Mark execution node at line
  breakpoints      - List all breakpoint nodes
  delete <id>      - Remove breakpoint node

Watch (Namespace Subscriptions):
  watch <expr>     - Subscribe to namespace changes
  unwatch <id>     - Unsubscribe
  watches          - List active subscriptions

> locals
Current namespace: scope:complex_algorithm
  data ──► [1, 2, 3, 4, 5]  (list, 5 nodes)
  item ──► 3                 (num)
  index ──► 2                (num)

> stack
Call graph path:
  scope:main
    └── call ──► scope:complex_algorithm  ◄── (current)

> execution
Execution graph (current ± 2 nodes):
  [stmt:13] ──► [stmt:14] ──► [stmt:15] ──► [stmt:16] ──► [stmt:17]
                               ▲ current
```

### 3. Call Stack as Graph Path

The call stack is a path through the call graph:

```graphoid
fn level3() {
    debug.trace()  # Print call graph path
}

fn level2() {
    level3()
}

fn level1() {
    level2()
}

level1()

# Output shows graph path:
# Call graph path to current node:
#   scope:main
#     └── call ──► scope:level1
#           └── call ──► scope:level2
#                 └── call ──► scope:level3  ◄── (current)
#
# As edges:
#   main ──call──► level1 ──call──► level2 ──call──► level3
```

### 4. Execution Graph Visualization

See where you are in the execution graph:

```
> execution full

Execution graph for complex_algorithm:
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│  [entry] ──► [param:data] ──► [stmt:assign i=0]            │
│                                      │                      │
│                                      ▼                      │
│                              ┌──► [loop:for]                │
│                              │       │                      │
│                              │       ▼ (body)               │
│                              │   [stmt:15] ◄── CURRENT      │
│                              │       │                      │
│                              │       ▼                      │
│                              │   [call:process]             │
│                              │       │                      │
│                              └───────┘ (back edge)          │
│                                      │                      │
│                                      ▼ (exit)               │
│                                  [return]                   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 5. Namespace Graph Inspection

Inspect the namespace graph structure:

```
> namespace

Namespace graph (current scope chain):
┌─────────────────────────────────────────────────────────────┐
│  universe:namespaces                                        │
│      │                                                      │
│      ├── scope:global                                       │
│      │       ├── print ──► (builtin)                        │
│      │       ├── complex_algorithm ──► (function)           │
│      │       └── ...                                        │
│      │                                                      │
│      └── scope:main                                         │
│              ├── data ──► [1,2,3,4,5]                       │
│              │                                              │
│              └── scope:complex_algorithm  ◄── CURRENT       │
│                      ├── data ──► [1,2,3,4,5] (param)       │
│                      ├── item ──► 3                         │
│                      └── index ──► 2                        │
└─────────────────────────────────────────────────────────────┘
```

### 6. Watch as Namespace Subscription

Watches subscribe to namespace graph edges:

```
> watch data.length()
Watch 1: Subscribed to data.length()
Current value: 5

> watch item
Watch 2: Subscribed to edge scope:complex_algorithm/item
Current value: 3

> continue
...
Watch 2 triggered: item changed 3 → 4
  Edge scope:complex_algorithm/item updated

Breakpoint 1 hit at algorithm.gr:15

> watches
Active subscriptions:
  Watch 1: data.length() = 5 (unchanged)
  Watch 2: item = 4 (changed from 3)
```

### 7. Data Graph Visualization

Visualize user data graphs:

```graphoid
# ASCII visualization of data graph
debug.visualize(my_graph)

# Output:
#     ┌───┐
#     │ A │
#     └─┬─┘
#       │ friend
#   ┌───┴───┐
#   ▼       ▼
# ┌───┐   ┌───┐
# │ B │   │ C │
# └─┬─┘   └───┘
#   │ colleague
#   ▼
# ┌───┐
# │ D │
# └───┘

# Graph statistics via graph inspection
stats = debug.graph_stats(my_graph)
print(stats)
# { nodes: 4, edges: 3, density: 0.25, components: 1,
#   layers: { data: 4, behaviors: 0, control: 0, metadata: 1 } }
```

### 8. Performance Profiling (Execution Graph Timing)

Profiling annotates execution graph nodes with timing:

```graphoid
import "debug"

debug.start_profile()

expensive_operation()
another_operation()

report = debug.stop_profile()
report.print()

# Output:
# === Profile Report (Execution Graph Analysis) ===
# Total execution nodes visited: 15,234
# Total time: 1523ms
#
# Hot paths (most time spent):
#   main ──► expensive_operation: 1200ms (78.8%)
#     └── inner_loop (5000 iterations): 1150ms
#   main ──► another_operation: 300ms (19.7%)
#
# Function timing:
#   expensive_operation   1200ms   78.8%   1 call
#   another_operation      300ms   19.7%   1 call
#   helper_fn               20ms    1.3%   150 calls
```

### 9. Universe Graph Overview

See the entire runtime state:

```
> universe

Universe graph overview:
┌─────────────────────────────────────────────────────────────┐
│  universe                                                   │
│      │                                                      │
│      ├── types/ (23 nodes)                                  │
│      │       └── any ──► num, string, bool, collection...  │
│      │                                                      │
│      ├── modules/ (5 nodes)                                 │
│      │       └── math, json, debug, my_app, algorithm       │
│      │                                                      │
│      ├── namespaces/ (3 scopes)                             │
│      │       └── global ──► main ──► complex_algorithm      │
│      │                                                      │
│      ├── execution/ (current)                               │
│      │       └── stmt:15 in complex_algorithm               │
│      │                                                      │
│      └── debug/                                             │
│              ├── breakpoints/ (2 nodes)                     │
│              └── watches/ (2 subscriptions)                 │
└─────────────────────────────────────────────────────────────┘
```

---

## Implementation Plan

### Day 1-2: Debug State as Graph

```rust
// Debug state lives in universe:debug subgraph
impl UniverseGraph {
    fn debug_subgraph(&self) -> &Graph {
        self.subgraph("debug")
    }

    fn add_breakpoint(&mut self, location: SourceLocation, condition: Option<Expr>) {
        let bp_id = self.next_breakpoint_id();
        let exec_node = self.execution_node_at(location);

        // Add breakpoint node to universe:debug/breakpoints
        self.debug_subgraph().add_node(format!("bp:{}", bp_id), {
            "target": exec_node,
            "condition": condition,
            "hit_count": 0,
        });
    }
}

// Stepping mode as edge in execution graph
enum SteppingMode {
    Continue,              // Run until breakpoint node
    Step,                  // Advance one execution edge
    StepInto,              // Follow call edges
    StepOut,               // Return to caller node
    StepOver,              // Skip call subgraphs
}
```

### Day 3-4: Debug REPL with Graph Commands

```rust
impl Executor {
    fn enter_debug_repl(&mut self) -> Result<DebugAction> {
        loop {
            print!("> ");
            let input = read_line();

            match self.parse_debug_command(&input)? {
                // Graph navigation
                DebugCommand::Step => return Ok(DebugAction::AdvanceExecutionEdge),
                DebugCommand::StepInto => return Ok(DebugAction::FollowCallEdge),
                DebugCommand::StepOut => return Ok(DebugAction::ReturnToCallerNode),

                // Graph inspection
                DebugCommand::Locals => {
                    let scope = self.current_namespace_subgraph();
                    self.print_graph_nodes(scope);
                }
                DebugCommand::Stack => {
                    let path = self.call_graph_path_to_current();
                    self.print_graph_path(path);
                }
                DebugCommand::Execution => {
                    let subgraph = self.execution_graph_around_current(2);
                    self.visualize_execution_graph(subgraph);
                }
                DebugCommand::Universe => {
                    self.print_universe_overview();
                }

                // Expression evaluation
                DebugCommand::Print(expr) => {
                    let value = self.evaluate_in_current_namespace(&expr)?;
                    println!("{}", value.display());
                }
                DebugCommand::Graph(name) => {
                    let graph = self.resolve_in_current_namespace(&name)?;
                    self.visualize_data_graph(&graph)?;
                }

                DebugCommand::Quit => return Err(DebugError::Quit),
            }
        }
    }
}
```

### Day 5-6: Execution Graph Navigation

```rust
impl Executor {
    fn should_break(&self) -> bool {
        let current_node = self.current_execution_node();

        // Check if any breakpoint node targets current execution node
        for bp in self.universe.debug_subgraph().nodes_with_label("breakpoint") {
            if bp.get("target") == current_node {
                if let Some(condition) = bp.get("condition") {
                    if !self.evaluate_condition(condition) {
                        continue;
                    }
                }
                bp.increment("hit_count");
                return true;
            }
        }

        // Check stepping mode
        match self.stepping_mode {
            SteppingMode::Step => true,
            SteppingMode::StepOver => self.at_same_call_depth(),
            SteppingMode::StepOut => self.returned_from_call(),
            _ => false,
        }
    }

    fn current_execution_node(&self) -> NodeId {
        self.universe.execution_subgraph().get("current")
    }

    fn advance_execution_edge(&mut self) {
        let current = self.current_execution_node();
        let next = self.universe.execution_subgraph()
            .traverse_edge(current, "next");
        self.universe.execution_subgraph().set("current", next);
    }
}
```

### Day 7-8: Namespace Graph Inspection

```rust
impl Executor {
    fn current_namespace_subgraph(&self) -> Graph {
        let scope_path = self.current_scope_path();
        self.universe.namespaces_subgraph().subgraph_at_path(scope_path)
    }

    fn call_graph_path_to_current(&self) -> Vec<NodeId> {
        // Traverse parent edges from current scope to root
        let mut path = vec![];
        let mut current = self.current_scope_node();

        while let Some(parent) = current.traverse_edge("parent") {
            path.push(current.id.clone());
            current = parent;
        }
        path.push(current.id); // root

        path.reverse();
        path
    }

    fn add_watch(&mut self, expr: &str) -> WatchId {
        // Subscribe to namespace edge changes
        let watch_id = self.next_watch_id();

        self.universe.debug_subgraph().add_node(format!("watch:{}", watch_id), {
            "expression": expr,
            "last_value": self.evaluate(expr).ok(),
        });

        watch_id
    }

    fn check_watches(&mut self) -> Vec<WatchTriggered> {
        let mut triggered = vec![];

        for watch in self.universe.debug_subgraph().nodes_with_label("watch") {
            let expr = watch.get("expression");
            let new_value = self.evaluate(expr).ok();
            let old_value = watch.get("last_value");

            if new_value != old_value {
                triggered.push(WatchTriggered {
                    id: watch.id,
                    expr: expr.clone(),
                    old: old_value,
                    new: new_value.clone(),
                });
                watch.set("last_value", new_value);
            }
        }

        triggered
    }
}
```

### Day 9-10: Graph Visualization

```rust
impl Graph {
    fn visualize_ascii(&self) -> String {
        // ASCII visualization of any graph
        let mut output = String::new();

        // Use topological sort for DAGs, BFS for general
        let layers = self.compute_layers();

        for (depth, nodes) in layers.iter().enumerate() {
            let indent = "  ".repeat(depth);
            for node in nodes {
                output.push_str(&format!("{}[{}]\n", indent, node.id));
                for edge in self.out_edges(node) {
                    output.push_str(&format!("{}  └── {} ──► {}\n",
                        indent, edge.label, edge.to));
                }
            }
        }

        output
    }

    fn visualize_execution_graph(&self, current: NodeId, radius: usize) -> String {
        // Show execution nodes around current position
        let neighborhood = self.neighborhood(current, radius);

        let mut output = String::new();
        for node in neighborhood.topo_sort() {
            let marker = if node.id == current { " ◄── CURRENT" } else { "" };
            output.push_str(&format!("[{}]{}\n", node.id, marker));

            if let Some(next) = self.traverse_edge(&node.id, "next") {
                output.push_str(&format!("  │\n  ▼\n"));
            }
        }

        output
    }
}
```

### Day 11-12: Profiling as Graph Annotation

```rust
struct Profiler {
    // Timing stored as metadata on execution graph nodes
    execution_graph: Graph,
}

impl Profiler {
    fn start(&mut self) {
        // Mark start node in execution graph
        let current = self.executor.current_execution_node();
        current.set_metadata("profile_start", Instant::now());
    }

    fn record_node_timing(&mut self, node: NodeId, duration: Duration) {
        // Add timing to execution graph node metadata
        self.execution_graph.get_node(&node)
            .update_metadata("total_time", |t| t + duration);
        self.execution_graph.get_node(&node)
            .increment_metadata("visit_count");
    }

    fn report(&self) -> ProfileReport {
        // Analyze execution graph metadata
        let hot_nodes = self.execution_graph.nodes()
            .filter(|n| n.get_metadata("total_time").is_some())
            .sorted_by(|a, b| b.get_metadata("total_time").cmp(&a.get_metadata("total_time")))
            .take(10)
            .collect();

        ProfileReport { hot_nodes, /* ... */ }
    }
}
```

### Day 13-14: DAP Integration (Optional) & Testing

```rust
// Debug Adapter Protocol maps to graph operations
impl DapServer {
    fn handle_request(&mut self, request: Request) -> Response {
        match request.command.as_str() {
            "stackTrace" => {
                // Return call graph path
                let path = self.executor.call_graph_path_to_current();
                Response::stack_frames(path)
            }
            "scopes" => {
                // Return namespace subgraphs
                let scopes = self.executor.namespace_scope_chain();
                Response::scopes(scopes)
            }
            "variables" => {
                // Query namespace subgraph
                let scope = self.executor.namespace_subgraph(request.scope_id);
                let vars = scope.edges().map(|e| Variable::from_edge(e));
                Response::variables(vars)
            }
            _ => { /* ... */ }
        }
    }
}
```

---

## Success Criteria

- [ ] `debug.break()` marks execution graph node
- [ ] `debug.break_if()` adds conditional marker
- [ ] Debug REPL with graph navigation commands
- [ ] `locals` queries current namespace subgraph
- [ ] `stack` shows call graph path
- [ ] `execution` visualizes execution graph around current node
- [ ] `namespace` shows namespace graph structure
- [ ] `universe` shows universe graph overview
- [ ] Watch expressions subscribe to namespace edges
- [ ] Profiling annotates execution graph with timing
- [ ] ASCII visualization for all graph types
- [ ] At least 35 debugger tests
- [ ] Example: Debugging session showing graph inspection
- [ ] Documentation explaining graph model
- [ ] (Stretch) DAP integration for VSCode

---

## Example Debugging Session

```
$ gr debug examples/algorithm.gr

Graphoid Debugger v0.1
Debugging IS graph inspection. Type 'help' for commands.

Loading examples/algorithm.gr...
Building execution graph... done (23 nodes)

> break 15
Breakpoint 1: Marked execution node stmt:15

> run
Traversing execution graph...
Breakpoint 1 hit at stmt:15 (algorithm.gr:15)

> execution
Execution graph (current ± 2):
  [stmt:13 assign] ──► [stmt:14 assign] ──► [stmt:15 call] ──► [stmt:16 if]
                                              ▲ CURRENT

> locals
Namespace subgraph: scope:complex_algorithm
  data ──► [1, 2, 3, 4, 5]  (list)
  item ──► 3                 (num)
  index ──► 2                (num)

> stack
Call graph path:
  scope:main
    └── call ──► scope:complex_algorithm  ◄── CURRENT

> watch item
Watch 1: Subscribed to edge scope:complex_algorithm/item
Current value: 3

> step
Advanced to stmt:16

> step
Advanced to stmt:17

Watch 1 triggered: item = 4 (was 3)
  Edge scope:complex_algorithm/item updated

> graph data
Data graph visualization:
  [node:0] ──next──► [node:1] ──next──► [node:2] ──next──► [node:3] ──next──► [node:4]
      │                  │                  │                  │                  │
      ▼                  ▼                  ▼                  ▼                  ▼
      1                  2                  3                  4                  5

> universe
Universe graph:
  types/      (23 nodes)
  modules/    (5 nodes)
  namespaces/ (3 scopes: global → main → complex_algorithm)
  execution/  (current: stmt:17)
  debug/      (1 breakpoint, 1 watch)

> continue
...
Program finished. Exit code: 0
```

---

## Related Documents

- [PHASE_15_NAMESPACE_GRAPH.md](PHASE_15_NAMESPACE_GRAPH.md) - Namespace graph structure
- [PHASE_16_EXECUTION_GRAPH.md](PHASE_16_EXECUTION_GRAPH.md) - Execution graph structure
- [PHASE_18_COMPLETE_GRAPH_MODEL.md](PHASE_18_COMPLETE_GRAPH_MODEL.md) - Universe graph structure

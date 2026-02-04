# Phase 16: Execution as Graph Traversal

**Duration**: 14-21 days
**Priority**: Critical (Blocker for compilation)
**Dependencies**: Phase 15 (Namespace Graph)
**Status**: Planning

---

## Goal

Replace the match-based tree-walking interpreter with graph-based execution where:
- AST is represented as a graph (not Rust enums)
- Execution is graph traversal
- Results are attached to nodes
- Optimization is graph rewriting

---

## Current Implementation (Problem)

```rust
// src/execution/executor.rs - CURRENT
impl Executor {
    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Binary { left, op, right } => {
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                // ... 50+ lines of operator handling
            }
            Expr::Call { callee, args } => {
                // ... function call handling
            }
            // ... 20+ more match arms
        }
    }
}
```

**Problems**:
1. AST is Rust enum, not graph
2. Execution is recursive function calls, not traversal
3. No graph operations possible on code
4. Cannot do graph-based optimization
5. Pattern matching overhead

---

## Target Architecture

### Functions as Subgraphs

A function is not a special construct—it's a subgraph:

```
┌─────────────────────────────────────────────────────────────────────┐
│  Function: add(a, b) { return a + b }                               │
│                                                                     │
│  ┌─────────────┐                                                    │
│  │ fn:add      │ ◄── Function node                                  │
│  └──────┬──────┘                                                    │
│         │                                                           │
│    ┌────┴────┬────────────┐                                         │
│    │ params  │    body    │                                         │
│    ▼         ▼            ▼                                         │
│ ┌──────┐ ┌──────┐    ┌───────────────────────┐                      │
│ │ "a"  │ │ "b"  │    │ Body Subgraph         │                      │
│ └──────┘ └──────┘    │  Return ─► Binary(+)  │                      │
│                      │            ┌──┴──┐    │                      │
│                      │           "a"   "b"   │                      │
│                      └───────────────────────┘                      │
└─────────────────────────────────────────────────────────────────────┘
```

**Key insights**:
- Function body IS a graph (the body subgraph)
- Parameters are nodes with edges from the function node
- Function call = create call frame + traverse body subgraph
- Closures = subgraph with edges to outer scope variables
- Composition = connecting subgraphs
- Inlining = graph substitution
- Partial application = subgraph with pre-bound edges

### AST as Graph

```
Source: result = add(x, 3)

AST Graph:
==========
┌─────────────────────────────────────────────────────────────────────┐
│                                                                     │
│              ┌──────────────────┐                                   │
│              │ Node: Assign     │                                   │
│              │ type: :assign    │                                   │
│              └────────┬─────────┘                                   │
│                       │                                             │
│         ┌─────────────┼─────────────┐                               │
│         │ (target)    │ (value)     │                               │
│         ▼             │             ▼                               │
│  ┌──────────────┐     │      ┌──────────────┐                       │
│  │ Node: Ident  │     │      │ Node: Call   │                       │
│  │ name: "result"│    │      │ type: :call  │                       │
│  └──────────────┘     │      └──────┬───────┘                       │
│                       │             │                               │
│                       │    ┌────────┼────────┐                      │
│                       │    │ (fn)   │ (args) │                      │
│                       │    ▼        │        ▼                      │
│                       │ ┌────────┐  │  ┌──────────┐                 │
│                       │ │ Ident  │  │  │ ArgList  │                 │
│                       │ │ "add"  │  │  └────┬─────┘                 │
│                       │ └────────┘  │       │                       │
│                       │             │  ┌────┴────┐                  │
│                       │             │  ▼         ▼                  │
│                       │             │ ┌────┐  ┌────┐                │
│                       │             │ │"x" │  │ 3  │                │
│                       │             │ └────┘  └────┘                │
│                                                                     │
│  Each node has: type, properties, edges to children                 │
│  Execution: Depth-first traversal                                   │
│  Results: Attached as "result" property on nodes                    │
└─────────────────────────────────────────────────────────────────────┘
```

### Core Data Structures

```rust
// src/execution_graph/mod.rs

use crate::graph::Graph;
use crate::values::Value;

/// An AST node in the execution graph
#[derive(Clone, Debug)]
pub struct AstNode {
    pub id: NodeId,
    pub node_type: AstNodeType,
    pub properties: HashMap<String, Value>,
    pub source_location: SourceLocation,
}

#[derive(Clone, Debug)]
pub enum AstNodeType {
    // Literals
    Number,
    String,
    Boolean,
    None,
    Symbol,

    // Identifiers
    Identifier,

    // Expressions
    Binary,
    Unary,
    Call,
    Index,
    FieldAccess,
    Lambda,

    // Statements
    VariableDecl,
    Assignment,
    If,
    While,
    For,
    Return,
    Block,
    FunctionDef,
    ClassDef,

    // Collections
    List,
    Map,
    Graph,

    // Pattern Matching
    Match,
    MatchArm,
    Pattern,
}

/// The execution graph - AST represented as graph
pub struct ExecutionGraph {
    graph: Graph,
    root: NodeId,
}

/// Edge types in execution graph
pub enum EdgeType {
    // Structural edges (AST structure)
    Left,           // Binary expression left operand
    Right,          // Binary expression right operand
    Operand,        // Unary expression operand
    Callee,         // Call expression function
    Argument(u32),  // Call expression argument (indexed)
    Condition,      // If/While condition
    ThenBranch,     // If then branch
    ElseBranch,     // If else branch
    Body,           // Function/loop body
    Target,         // Assignment target
    Value,          // Assignment value
    Element(u32),   // Collection element (indexed)

    // Semantic edges (resolved during analysis)
    ResolvesTo,     // Identifier resolves to definition
    InstanceOf,     // Value is instance of class

    // Execution edges (added during execution)
    Result,         // Node's computed result
    NextExec,       // Next node to execute (for control flow)
}
```

### Graph-Based Executor

```rust
// src/execution_graph/executor.rs

pub struct GraphExecutor {
    graph: ExecutionGraph,
    namespace: NamespaceGraph,  // From Phase 15
    call_stack: Vec<CallFrame>,
}

impl GraphExecutor {
    /// Execute from root node
    pub fn execute(&mut self) -> Result<Value> {
        self.execute_node(self.graph.root)
    }

    /// Execute a single node
    fn execute_node(&mut self, node_id: NodeId) -> Result<Value> {
        let node = self.graph.get_node(node_id)?;

        let result = match node.node_type {
            AstNodeType::Number => {
                self.execute_number(node)
            }
            AstNodeType::String => {
                self.execute_string(node)
            }
            AstNodeType::Binary => {
                self.execute_binary(node_id, node)
            }
            AstNodeType::Call => {
                self.execute_call(node_id, node)
            }
            // ... other node types
        };

        // Attach result to node
        if let Ok(ref value) = result {
            self.graph.set_node_property(node_id, "result", value.clone());
        }

        result
    }

    /// Execute binary expression
    fn execute_binary(&mut self, node_id: NodeId, node: &AstNode) -> Result<Value> {
        // Get operator from node properties
        let op = node.properties.get("operator")?;

        // Traverse to child nodes
        let left_id = self.graph.get_edge_target(node_id, EdgeType::Left)?;
        let right_id = self.graph.get_edge_target(node_id, EdgeType::Right)?;

        // Execute children (graph traversal!)
        let left_val = self.execute_node(left_id)?;
        let right_val = self.execute_node(right_id)?;

        // Perform operation
        self.apply_binary_op(op, left_val, right_val)
    }

    /// Execute function call
    fn execute_call(&mut self, node_id: NodeId, node: &AstNode) -> Result<Value> {
        // Get callee
        let callee_id = self.graph.get_edge_target(node_id, EdgeType::Callee)?;
        let callee = self.execute_node(callee_id)?;

        // Get arguments (traverse argument edges)
        let arg_ids = self.graph.get_edges_by_type(node_id, EdgeType::Argument);
        let mut args = Vec::new();
        for (_, arg_id) in arg_ids {
            args.push(self.execute_node(arg_id)?);
        }

        // Call function
        self.call_function(callee, args)
    }

    /// Execute block (sequence of statements)
    fn execute_block(&mut self, node_id: NodeId) -> Result<Value> {
        let statement_ids = self.graph.get_edges_by_type(node_id, EdgeType::Element);

        let mut result = Value::None;
        for (_, stmt_id) in statement_ids.into_iter().sorted_by_key(|(i, _)| *i) {
            result = self.execute_node(stmt_id)?;

            // Check for return/break/continue
            if self.control_flow_pending() {
                break;
            }
        }

        Ok(result)
    }
}
```

---

## Parser Modifications

The parser must produce an `ExecutionGraph` instead of AST enums:

```rust
// src/parser/graph_parser.rs

pub struct GraphParser {
    tokens: Vec<Token>,
    pos: usize,
    graph: ExecutionGraph,
}

impl GraphParser {
    pub fn parse(&mut self) -> Result<ExecutionGraph> {
        let root = self.parse_program()?;
        self.graph.root = root;
        Ok(std::mem::take(&mut self.graph))
    }

    fn parse_binary_expr(&mut self) -> Result<NodeId> {
        let left = self.parse_unary()?;

        if let Some(op) = self.match_binary_operator() {
            let right = self.parse_binary_expr()?;

            // Create binary node
            let node_id = self.graph.add_node(AstNode {
                node_type: AstNodeType::Binary,
                properties: hashmap!{ "operator" => op },
                source_location: self.current_location(),
            });

            // Add edges to children
            self.graph.add_edge(node_id, left, EdgeType::Left);
            self.graph.add_edge(node_id, right, EdgeType::Right);

            Ok(node_id)
        } else {
            Ok(left)
        }
    }

    fn parse_call_expr(&mut self, callee: NodeId) -> Result<NodeId> {
        self.expect(Token::LeftParen)?;

        // Create call node
        let node_id = self.graph.add_node(AstNode {
            node_type: AstNodeType::Call,
            properties: HashMap::new(),
            source_location: self.current_location(),
        });

        // Add callee edge
        self.graph.add_edge(node_id, callee, EdgeType::Callee);

        // Parse arguments
        let mut arg_index = 0;
        while !self.check(Token::RightParen) {
            let arg = self.parse_expression()?;
            self.graph.add_edge(node_id, arg, EdgeType::Argument(arg_index));
            arg_index += 1;

            if !self.match_token(Token::Comma) {
                break;
            }
        }

        self.expect(Token::RightParen)?;
        Ok(node_id)
    }
}
```

---

## Graph-Based Optimization

One major benefit: code optimization via graph rewriting.

### Constant Folding

```
Before:
┌─────────────┐
│ Binary(+)   │
└──────┬──────┘
  ┌────┴────┐
  ▼         ▼
┌───┐     ┌───┐
│ 3 │     │ 4 │
└───┘     └───┘

After (rewrite):
┌───┐
│ 7 │
└───┘
```

```rust
impl ExecutionGraph {
    pub fn optimize(&mut self) {
        self.fold_constants();
        self.eliminate_dead_code();
        self.inline_small_functions();
    }

    fn fold_constants(&mut self) {
        // Find all Binary nodes with constant children
        let candidates = self.graph.find_nodes(|n| {
            n.node_type == AstNodeType::Binary &&
            self.all_children_constant(n.id)
        });

        for node_id in candidates {
            let value = self.evaluate_constant(node_id);
            self.replace_with_constant(node_id, value);
        }
    }

    fn eliminate_dead_code(&mut self) {
        // Remove unreachable nodes
        let reachable = self.graph.reachable_from(self.root);
        self.graph.retain_nodes(|id| reachable.contains(&id));
    }
}
```

### Common Subexpression Elimination

```
Before:                        After:
  ┌─────────┐                   ┌─────────┐
  │ a + b   │                   │  temp   │
  └─────────┘                   └─────────┘
                                     │
  ┌─────────┐                   ┌────┴────┐
  │ a + b   │     ───────►      │(shared) │
  └─────────┘                   └─────────┘
```

---

## Parallel Execution

Graph structure enables parallel execution of independent subgraphs:

```rust
impl GraphExecutor {
    fn execute_parallel(&mut self, node_id: NodeId) -> Result<Value> {
        let node = self.graph.get_node(node_id)?;

        match node.node_type {
            AstNodeType::Binary => {
                // Left and right are independent - can execute in parallel!
                let left_id = self.graph.get_edge_target(node_id, EdgeType::Left)?;
                let right_id = self.graph.get_edge_target(node_id, EdgeType::Right)?;

                // Check for data dependencies
                if !self.has_dependency(left_id, right_id) {
                    // Execute in parallel
                    let (left_val, right_val) = rayon::join(
                        || self.execute_node(left_id),
                        || self.execute_node(right_id)
                    );
                    return self.apply_binary_op(op, left_val?, right_val?);
                }
            }
            // ...
        }

        // Fall back to sequential
        self.execute_node(node_id)
    }
}
```

---

## Migration Strategy

### Step 1: Create Parallel Infrastructure

Build new execution graph alongside existing:
- `src/execution_graph/mod.rs` - New module
- Keep existing `src/execution/` working
- Feature flag to switch

### Step 2: Implement Graph Parser

Create parser that outputs `ExecutionGraph`:
- `src/parser/graph_parser.rs`
- Use alongside existing parser initially
- Verify both produce equivalent results

### Step 3: Implement Graph Executor

Create executor that traverses graph:
- `src/execution_graph/executor.rs`
- Start with simple expressions
- Gradually add statement types

### Step 4: Integration Testing

Run all tests with both implementations:
- Feature flag: `--features graph_execution`
- Verify identical results
- Benchmark performance

### Step 5: Optimization Passes

Add optimization capabilities:
- Constant folding
- Dead code elimination
- Common subexpression elimination

### Step 6: Remove Old Code

Once validated:
- Remove `src/execution/executor.rs`
- Remove AST enums (or keep for debugging)
- Remove feature flags

---

## Implementation Plan

### Week 1: Core Structures

| Day | Task |
|-----|------|
| 1-2 | Create ExecutionGraph, AstNode, EdgeType |
| 3-4 | Implement basic graph operations |
| 5 | Unit tests for graph structure |

### Week 2: Graph Parser

| Day | Task |
|-----|------|
| 6-7 | Parse expressions to graph |
| 8-9 | Parse statements to graph |
| 10 | Parse functions, classes |

### Week 3: Graph Executor

| Day | Task |
|-----|------|
| 11-12 | Execute expressions via traversal |
| 13-14 | Execute statements, control flow |
| 15 | Execute functions, closures |

### Week 4: Advanced Features

| Day | Task |
|-----|------|
| 16-17 | Collections (list, map, graph) |
| 18-19 | Pattern matching |
| 20 | Exception handling |

### Week 5: Integration

| Day | Task |
|-----|------|
| 21-22 | Feature flag integration |
| 23-24 | Run all tests with graph execution |
| 25 | Optimization passes (basic) |

### Week 6: Polish

| Day | Task |
|-----|------|
| 26-27 | Performance benchmarking |
| 28-29 | Documentation |
| 30 | Remove old code (if validated) |

---

## Success Criteria

### Functional

- [ ] All expressions execute via graph traversal
- [ ] All statements execute via graph traversal
- [ ] All existing tests pass with graph execution
- [ ] Closures work correctly
- [ ] Pattern matching works correctly

### Performance

- [ ] Execution ≤1.5x tree-walking overhead
- [ ] Optimization passes improve performance
- [ ] Memory overhead ≤40% increase

### Architecture

- [ ] AST is a graph, not enums
- [ ] Execution is graph traversal
- [ ] Results attached to nodes
- [ ] At least 2 optimization passes working

---

## Debugging Support

### Visualize Execution

```graphoid
debug.execution_graph()
# Returns graph visualization of current program

debug.trace()
# Shows step-by-step graph traversal

debug.result(node_id)
# Shows computed result for a node
```

### Execution Trace

```
Executing: result = add(x, 3)

[1] Node: Assign → evaluate
    [2] Node: Call → evaluate
        [3] Node: Ident "add" → resolves to Fn{add}
        [4] Node: Ident "x" → resolves to 5
        [5] Node: Number 3 → 3
        [2] Call result: 8
    [1] Assign: result = 8

Result: 8
```

---

## Design Decisions (Resolved)

1. **No hybrid approach** — The AST is fully graph-based. No fallback to Rust enums for any operations. This is non-negotiable given the "everything is a graph" mandate.

2. **No lazy evaluation** — The full graph is built upfront before execution begins. Lazy construction would block graph-wide optimization passes (constant folding, dead code elimination, CSE), complicate parallel execution analysis, and add per-node "is this built?" branching overhead.

3. **Yes: Incremental parsing** — Subgraphs can be rebuilt without reconstructing the entire execution graph. When source changes (REPL input, module reload, IDE edit), only the affected subgraph is re-parsed and swapped in. This is a natural graph operation (subgraph replacement) and provides the foundation for:
   - Fast REPL re-evaluation
   - Hot module reload (Phase 18.5)
   - Future IDE/LSP support

4. **Yes: Arena allocation with per-scope arenas** — Nodes are arena-allocated for cache-friendly layout, fast bump-pointer allocation, and simplified Rust ownership (indices instead of `Rc`/`RefCell`). Arenas are scoped per-module or per-function so that incremental re-parsing can drop and rebuild individual subgraphs without leaking the entire arena. The top-level execution graph references nodes via `(ArenaId, NodeIndex)` pairs rather than raw indices.

   ```
   ┌─────────────────────────────────────────────┐
   │  ExecutionGraph                              │
   │                                              │
   │  ┌──────────────┐  ┌──────────────┐          │
   │  │ Arena: main   │  │ Arena: mod_a │          │
   │  │ [node][node]  │  │ [node][node] │          │
   │  │ [node][node]  │  │ [node]       │          │
   │  └──────────────┘  └──────────────┘          │
   │                                              │
   │  ┌──────────────┐  ┌──────────────┐          │
   │  │ Arena: fn_foo │  │ Arena: fn_bar│          │
   │  │ [node][node]  │  │ [node][node] │          │
   │  └──────────────┘  └──────────────┘          │
   │                                              │
   │  Edges reference: (ArenaId, NodeIndex)       │
   │  Re-parse fn_foo → drop Arena:fn_foo,        │
   │    allocate new arena, rebuild subgraph,     │
   │    repoint edges from parent                 │
   └─────────────────────────────────────────────┘
   ```

---

## Related Documents

- [GRAPH_CENTRIC_ARCHITECTURE_RATIONALE.md](GRAPH_CENTRIC_ARCHITECTURE_RATIONALE.md) - Why this matters
- [PHASE_15_NAMESPACE_GRAPH.md](PHASE_15_NAMESPACE_GRAPH.md) - Prerequisite
- [PHASE_29_COMPILATION_STRATEGY.md](PHASE_29_COMPILATION_STRATEGY.md) - Compilation options (if pursued)
- [PHASE_19_CONCURRENCY.md](PHASE_19_CONCURRENCY.md) - Parallel execution benefits

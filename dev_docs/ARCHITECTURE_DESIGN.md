# Graphoid/Glang: Core Architecture Design

**Version**: 1.0
**Last Updated**: January 2025
**Status**: Foundational design for Rust implementation

This document addresses the critical architectural decisions that must be made before implementation begins, particularly around graph layer construction, rule access patterns, and internal representation.

---

## Critical Questions Addressed

1. **How are graph layers constructed internally?**
2. **How do rules access the data layer and structure?**
3. **How do user-defined functions safely inspect graphs?**
4. **Where is the boundary between Values and Graphs?**
5. **How do we handle node identity and references in Rust?**
6. **When and how are behaviors applied?**
7. **How does the executor interact with different value types?**

---

## Part 1: Value System vs Graph System

### The Fundamental Split

**Problem**: The specification says lists and maps should support behaviors, but behaviors imply graph structure. How do we reconcile simple collections with graph-backed collections?

**Solution**: Two-tier system with automatic promotion.

### Simple Values (Fast Path)

```rust
/// Simple values - no graph structure, no behaviors
pub enum SimpleValue {
    Number(NumberData),
    String(String),
    Boolean(bool),
    None,
    Symbol(String),
    Time(f64),

    // Simple collections - just containers
    List(Vec<Value>),
    Map(HashMap<String, Value>),
}
```

**Characteristics**:
- Fast operations (direct Vec/HashMap operations)
- No behavior support
- No rule validation
- Used for simple data that doesn't need graph features

### Graph-Backed Values (Full Power)

```rust
/// Graph-backed collections - full five-layer architecture
pub struct GraphValue {
    // The five layers (see Part 2)
    data: DataLayer,
    behaviors: BehaviorLayer,
    control: ControlLayer,
    metadata: MetadataLayer,
    boundary: BoundaryLayer,  // Future

    // Type information
    graph_type: GraphType,
}

pub enum GraphType {
    Tree,
    List,  // List as a graph (linked nodes)
    Map,   // Map as a graph (hash structure)
    DAG,
    General,
}
```

**Characteristics**:
- Full five-layer architecture
- Behavior support
- Rule validation
- Used when behaviors/rules are needed

### Automatic Promotion

```rust
impl Value {
    /// Add a behavior to a collection
    /// Promotes simple collection to graph if needed
    pub fn add_behavior(&mut self, behavior: Behavior) -> Result<()> {
        match self {
            Value::Simple(SimpleValue::List(items)) => {
                // Promote to graph-backed list
                let graph = GraphValue::from_list(items.clone())?;
                *self = Value::Graph(Box::new(graph));
                self.add_behavior(behavior)?;  // Recursively add to promoted graph
            }

            Value::Graph(ref mut graph) => {
                graph.behaviors.add(behavior);
            }

            _ => {
                return Err(GraphoidError::TypeError {
                    message: format!("Cannot add behavior to {}", self.get_type()),
                    position: SourcePosition::unknown(),
                });
            }
        }
        Ok(())
    }
}
```

**Key Insight**: Start simple (fast), promote when needed (powerful). Most lists/maps never need behaviors and stay fast.

---

## Part 2: Five-Layer Architecture

### Layer Structure

```rust
pub struct GraphValue {
    data: DataLayer,
    behaviors: BehaviorLayer,
    control: ControlLayer,
    metadata: MetadataLayer,
    graph_type: GraphType,
}

/// Layer 1: Pure data and structure
pub struct DataLayer {
    nodes: HashMap<NodeId, Node>,
    edges: Vec<Edge>,
    root: Option<NodeId>,
    next_node_id: u64,
}

/// Layer 2: Computational transformations
pub struct BehaviorLayer {
    transformations: Vec<Transformation>,
    validators: Vec<Validator>,
    mappings: Vec<MappingRule>,
    conditionals: Vec<ConditionalRule>,
}

/// Layer 3: Rule enforcement and governance
pub struct ControlLayer {
    builtin_rules: Vec<BuiltinRule>,
    user_rules: Vec<UserRule>,
    disabled_rules: HashSet<String>,
}

/// Layer 4: History and provenance
pub struct MetadataLayer {
    element_names: HashMap<NodeId, String>,
    creation_times: HashMap<NodeId, f64>,
    operation_log: Vec<Operation>,
    custom_metadata: HashMap<String, String>,
}

/// Node representation
pub struct Node {
    id: NodeId,
    value: Value,  // The actual data stored in this node
    local_metadata: HashMap<String, String>,
}

/// Edge representation
pub struct Edge {
    from: NodeId,
    to: NodeId,
    edge_type: EdgeType,
    weight: Option<f64>,
    metadata: HashMap<String, String>,
}

pub type NodeId = String;  // "node_0", "node_1", etc.

pub enum EdgeType {
    Child,    // Parent-child (trees)
    Next,     // Sequential (lists)
    Depends,  // Dependencies (DAGs)
    Custom(String),
}
```

### Why This Structure?

1. **Clear Separation**: Each layer has distinct responsibilities
2. **No Circular Dependencies**: Layers reference each other in one direction only
3. **Rust-Friendly**: No Rc/RefCell needed with ID-based node references
4. **Extensible**: Easy to add new layer functionality

---

## Part 3: Rule Validation Context

### The Problem

Rules need to:
- **Read** data layer structure (node count, edges, paths)
- **Read** behavior layer (what transforms are active)
- **Read** metadata layer (history, names)
- **NOT mutate** anything (rules are validators, not mutators)

### The Solution: Immutable Context

```rust
/// Read-only view of graph for validation
pub struct ValidationContext<'a> {
    data: &'a DataLayer,
    behaviors: &'a BehaviorLayer,
    metadata: &'a MetadataLayer,
    graph_type: &'a GraphType,
}

impl<'a> ValidationContext<'a> {
    /// Query methods - no mutation allowed
    pub fn node_count(&self) -> usize {
        self.data.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.data.edges.len()
    }

    pub fn has_path(&self, from: &NodeId, to: &NodeId) -> bool {
        // BFS/DFS to check path existence
        self.data.has_path(from, to)
    }

    pub fn get_children(&self, node: &NodeId) -> Vec<NodeId> {
        self.data.edges.iter()
            .filter(|e| e.from == *node && matches!(e.edge_type, EdgeType::Child))
            .map(|e| e.to.clone())
            .collect()
    }

    pub fn count_roots(&self) -> usize {
        self.data.count_roots()
    }

    pub fn would_create_cycle(&self, from: &NodeId, to: &NodeId) -> bool {
        // Check if adding edge from->to would create a cycle
        self.has_path(to, from)
    }

    pub fn get_node_value(&self, id: &NodeId) -> Option<&Value> {
        self.data.nodes.get(id).map(|n| &n.value)
    }

    pub fn active_behaviors(&self) -> &[Transformation] {
        &self.behaviors.transformations
    }
}
```

### Built-in Rule Validators

```rust
pub trait RuleValidator {
    fn validate(&self, ctx: &ValidationContext, op: &GraphOperation) -> Result<(), String>;

    /// Optional: suggest parent for insert operations
    fn suggest_parent(&self, ctx: &ValidationContext, value: &Value) -> Option<NodeId> {
        None
    }
}

/// Example: SingleRoot rule
pub struct SingleRootValidator;

impl RuleValidator for SingleRootValidator {
    fn validate(&self, ctx: &ValidationContext, op: &GraphOperation) -> Result<(), String> {
        match op {
            GraphOperation::AddNode { .. } => {
                // After this operation, how many roots would there be?
                let current_roots = ctx.count_roots();

                // If adding a disconnected node, we'd have current_roots + 1
                // This is only OK if current_roots == 0
                if current_roots > 0 {
                    return Err(format!(
                        "Operation would create {} roots (single_root requires exactly 1)",
                        current_roots + 1
                    ));
                }
                Ok(())
            }

            GraphOperation::AddEdge { from, to, .. } => {
                // Adding an edge might change root count
                // Validate final state would have exactly 1 root
                // (Implementation details...)
                Ok(())
            }

            _ => Ok(())
        }
    }
}

/// Graph operation types
pub enum GraphOperation {
    AddNode { id: NodeId, value: Value },
    AddEdge { from: NodeId, to: NodeId, weight: Option<f64> },
    RemoveNode { id: NodeId },
    RemoveEdge { from: NodeId, to: NodeId },
}
```

### Control Layer Validation Flow

```rust
impl ControlLayer {
    pub fn validate(&self, ctx: &ValidationContext, op: &GraphOperation) -> Result<()> {
        // Check all active rules
        for rule in &self.builtin_rules {
            if !self.disabled_rules.contains(rule.name()) {
                let validator = rule.get_validator();
                validator.validate(ctx, op)
                    .map_err(|msg| GraphoidError::RuleViolation {
                        rule: rule.name().to_string(),
                        message: msg,
                    })?;
            }
        }

        // Check user-defined rules
        for user_rule in &self.user_rules {
            self.validate_user_rule(ctx, op, user_rule)?;
        }

        Ok(())
    }
}
```

---

## Part 4: User-Defined Rule Access

### The Challenge

User-defined rules are functions written in Graphoid:

```glang
func validate_employee_graph(graph) {
    if graph.nodes.size() > 100 {
        return false
    }
    return true
}
```

How does the user function safely access `graph`?

### The Solution: Graph Introspection Object

```rust
/// Safe wrapper for user function access to graph structure
pub struct GraphIntrospection {
    node_count: usize,
    edge_count: usize,
    root_count: usize,
    // Cached data that's safe to expose
}

impl GraphIntrospection {
    pub fn from_context(ctx: &ValidationContext) -> Self {
        Self {
            node_count: ctx.node_count(),
            edge_count: ctx.edge_count(),
            root_count: ctx.count_roots(),
        }
    }

    /// Convert to a Value that can be passed to user functions
    pub fn to_value(&self) -> Value {
        let mut map = HashMap::new();
        map.insert("node_count".to_string(), Value::number(self.node_count as f64));
        map.insert("edge_count".to_string(), Value::number(self.edge_count as f64));
        map.insert("root_count".to_string(), Value::number(self.root_count as f64));
        Value::Map(map)
    }
}
```

### Calling User-Defined Rules

```rust
impl ControlLayer {
    fn validate_user_rule(
        &self,
        ctx: &ValidationContext,
        op: &GraphOperation,
        user_rule: &UserRule,
    ) -> Result<()> {
        // Create safe introspection object
        let introspection = GraphIntrospection::from_context(ctx);
        let graph_value = introspection.to_value();

        // Call user function with introspection value
        // (Executor passed in from outside, or stored in ControlLayer)
        let result = self.executor.call_function(
            &user_rule.function,
            vec![graph_value],
        )?;

        // User function must return boolean
        match result {
            Value::Boolean(true) => Ok(()),
            Value::Boolean(false) => Err(GraphoidError::RuleViolation {
                rule: user_rule.name.clone(),
                message: "User-defined rule returned false".to_string(),
            }),
            _ => Err(GraphoidError::TypeError {
                message: format!(
                    "Rule function must return bool, got {}",
                    result.get_type()
                ),
                position: SourcePosition::unknown(),
            }),
        }
    }
}

pub struct UserRule {
    name: String,
    function: FunctionValue,  // Reference to user-defined function
}
```

### Safe Method Exposure

Instead of exposing the entire graph, we expose specific safe methods:

```glang
# User code can call these methods on the graph introspection object
graph.node_count()   # Returns number
graph.edge_count()   # Returns number
graph.root_count()   # Returns number

# Future: More sophisticated queries
graph.has_path(from, to)
graph.get_depth()
graph.is_balanced()
```

**Implementation**:

```rust
impl Executor {
    fn execute_method_call(&mut self, object: Value, method: &str, args: Vec<Value>) -> Result<Value> {
        match object {
            Value::Map(ref map) if map.contains_key("__introspection__") => {
                // This is a graph introspection object
                match method {
                    "node_count" => Ok(map.get("node_count").unwrap().clone()),
                    "edge_count" => Ok(map.get("edge_count").unwrap().clone()),
                    "root_count" => Ok(map.get("root_count").unwrap().clone()),
                    _ => Err(GraphoidError::RuntimeError {
                        message: format!("Unknown introspection method: {}", method),
                    }),
                }
            }
            // ... other cases
        }
    }
}
```

---

## Part 5: Node Identity and References

### The Options

**Option A: String IDs** (RECOMMENDED)
```rust
pub type NodeId = String;  // "node_0", "node_1", etc.

pub struct DataLayer {
    nodes: HashMap<NodeId, Node>,  // Fast lookup by ID
    edges: Vec<Edge>,              // Edges store IDs
}
```

**Pros**:
- No circular references
- Rust-friendly (no Rc/RefCell)
- Easy to serialize/debug
- Can generate human-readable IDs

**Cons**:
- Hash lookup for every node access
- Slightly more memory (though interned strings help)

**Option B: Rc<RefCell<Node>>**
```rust
pub type NodeRef = Rc<RefCell<Node>>;
```

**Pros**:
- Direct references
- No lookup needed

**Cons**:
- Runtime borrow checking (can panic!)
- Circular references require Weak<>
- Complex lifetime management
- Not thread-safe (need Arc<Mutex<>> for threading)

**Option C: Arena/Index-based**
```rust
pub type NodeId = usize;  // Index into Vec<Node>

pub struct DataLayer {
    nodes: Vec<Option<Node>>,  // Option for deletion/tombstones
    edges: Vec<Edge>,
}
```

**Pros**:
- Fast lookup (array indexing)
- Cache-friendly
- No hash computation

**Cons**:
- Deletion is complex (tombstones or compaction)
- Indices can become invalid
- Not as debuggable

### Recommendation: Start with Option A

Use **String IDs** initially:
1. Simplest to implement correctly
2. No Rust borrow-checker battles
3. Easy to debug and serialize
4. Can optimize to Option C later if profiling shows need

```rust
impl DataLayer {
    fn generate_node_id(&mut self) -> NodeId {
        let id = format!("node_{}", self.next_node_id);
        self.next_node_id += 1;
        id
    }

    pub fn add_node(&mut self, value: Value) -> NodeId {
        let id = self.generate_node_id();
        let node = Node {
            id: id.clone(),
            value,
            local_metadata: HashMap::new(),
        };
        self.nodes.insert(id.clone(), node);
        id
    }

    pub fn get_node(&self, id: &NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn add_edge(&mut self, from: NodeId, to: NodeId, edge_type: EdgeType, weight: Option<f64>) {
        self.edges.push(Edge {
            from,
            to,
            edge_type,
            weight,
            metadata: HashMap::new(),
        });
    }
}
```

---

## Part 6: Behavior Application

### When Are Behaviors Applied?

**Answer**: Eagerly, at the moment values enter the graph.

```rust
impl GraphValue {
    pub fn insert_node(&mut self, value: Value, parent: Option<NodeId>) -> Result<NodeId> {
        // STEP 1: Apply behaviors to transform the value
        let transformed_value = self.behaviors.apply(value)?;

        // STEP 2: Create operation for validation
        let node_id = self.data.generate_node_id();
        let op = GraphOperation::AddNode {
            id: node_id.clone(),
            value: transformed_value.clone(),
        };

        // STEP 3: Validate with control layer
        let ctx = self.make_validation_context();
        self.control.validate(&ctx, &op)?;

        // STEP 4: Add to data layer
        let actual_id = self.data.add_node(transformed_value);

        // STEP 5: If parent specified, add edge
        if let Some(parent_id) = parent {
            let edge_op = GraphOperation::AddEdge {
                from: parent_id.clone(),
                to: actual_id.clone(),
                weight: None,
            };
            self.control.validate(&ctx, &edge_op)?;
            self.data.add_edge(parent_id, actual_id.clone(), EdgeType::Child, None);
        }

        // STEP 6: Update metadata
        self.metadata.record_operation(&op);

        Ok(actual_id)
    }

    fn make_validation_context(&self) -> ValidationContext {
        ValidationContext {
            data: &self.data,
            behaviors: &self.behaviors,
            metadata: &self.metadata,
            graph_type: &self.graph_type,
        }
    }
}
```

### Behavior Layer Implementation

```rust
impl BehaviorLayer {
    /// Apply all behaviors to a value
    pub fn apply(&self, mut value: Value) -> Result<Value> {
        // Apply transformations in order
        for transformation in &self.transformations {
            value = transformation.apply(value)?;
        }

        // Apply conditional rules
        for conditional in &self.conditionals {
            if conditional.condition.evaluate(&value)? {
                value = conditional.then_transform.apply(value)?;
            } else if let Some(ref else_transform) = conditional.else_transform {
                value = else_transform.apply(value)?;
            }
        }

        // Apply mappings
        for mapping in &self.mappings {
            value = mapping.apply(value)?;
        }

        // Validate with validators
        for validator in &self.validators {
            validator.validate(&value)?;
        }

        Ok(value)
    }
}

pub enum Transformation {
    NoneToZero,
    Positive,
    RoundToInt,
    Custom(Box<dyn Fn(Value) -> Result<Value>>),
}

impl Transformation {
    pub fn apply(&self, value: Value) -> Result<Value> {
        match self {
            Transformation::NoneToZero => {
                match value {
                    Value::None => Ok(Value::number(0.0)),
                    v => Ok(v),
                }
            }
            Transformation::Positive => {
                match value {
                    Value::Number(n) if n.value < 0.0 => {
                        Ok(Value::Number(NumberData {
                            value: -n.value,
                            display_precision: n.display_precision,
                        }))
                    }
                    v => Ok(v),
                }
            }
            Transformation::Custom(f) => f(value),
            // ... other transformations
        }
    }
}
```

---

## Part 7: Executor Interaction

### Unified Value Handling

The executor must handle both simple values and graph values transparently:

```rust
impl Executor {
    pub fn execute_method_call(
        &mut self,
        object: &Expr,
        method: &str,
        args: &[Expr],
    ) -> Result<Value> {
        let obj = self.evaluate_expression(object)?;
        let arg_values: Vec<Value> = args.iter()
            .map(|e| self.evaluate_expression(e))
            .collect::<Result<_>>()?;

        match obj {
            // Simple list - direct operations
            Value::Simple(SimpleValue::List(ref items)) => {
                self.execute_list_method(items, method, arg_values)
            }

            // Simple map - direct operations
            Value::Simple(SimpleValue::Map(ref map)) => {
                self.execute_map_method(map, method, arg_values)
            }

            // Graph-backed collection - full five-layer system
            Value::Graph(ref graph) => {
                self.execute_graph_method(graph, method, arg_values)
            }

            // Other simple types
            Value::Simple(SimpleValue::String(ref s)) => {
                self.execute_string_method(s, method, arg_values)
            }

            Value::Simple(SimpleValue::Number(ref n)) => {
                self.execute_number_method(n, method, arg_values)
            }

            _ => Err(GraphoidError::RuntimeError {
                message: format!("Type {} has no method '{}'", obj.get_type(), method),
            }),
        }
    }

    fn execute_graph_method(
        &mut self,
        graph: &GraphValue,
        method: &str,
        args: Vec<Value>,
    ) -> Result<Value> {
        match method {
            // Behavior layer methods
            "add_rule" => {
                let rule_name = match &args[0] {
                    Value::Simple(SimpleValue::String(s)) => s.clone(),
                    Value::Simple(SimpleValue::Symbol(s)) => s.clone(),
                    _ => return Err(GraphoidError::TypeError {
                        message: "add_rule expects string or symbol".to_string(),
                        position: SourcePosition::unknown(),
                    }),
                };

                // Clone graph, add rule, return modified graph
                let mut new_graph = graph.clone();
                new_graph.behaviors.add_transformation(rule_name)?;
                Ok(Value::Graph(Box::new(new_graph)))
            }

            "has_rule" => {
                let rule_name = self.extract_string(&args[0])?;
                Ok(Value::Boolean(graph.behaviors.has_transformation(&rule_name)))
            }

            // Data layer methods
            "insert" => {
                let value = args[0].clone();
                let parent = if args.len() > 1 {
                    Some(self.extract_node_id(&args[1])?)
                } else {
                    None
                };

                let mut new_graph = graph.clone();
                let node_id = new_graph.insert_node(value, parent)?;
                Ok(Value::Graph(Box::new(new_graph)))
            }

            "size" => {
                Ok(Value::number(graph.data.nodes.len() as f64))
            }

            // Delegate to graph type
            _ => graph.execute_type_specific_method(method, args),
        }
    }
}
```

### Value Wrapper Enum

To unify simple and complex values:

```rust
pub enum Value {
    Simple(SimpleValue),
    Graph(Box<GraphValue>),
    Function(FunctionValue),
    Lambda(LambdaValue),
}

impl Value {
    pub fn get_type(&self) -> &str {
        match self {
            Value::Simple(sv) => sv.get_type(),
            Value::Graph(g) => g.get_type(),
            Value::Function(_) => "function",
            Value::Lambda(_) => "lambda",
        }
    }

    pub fn to_display_string(&self) -> String {
        match self {
            Value::Simple(sv) => sv.to_display_string(),
            Value::Graph(g) => g.to_display_string(),
            Value::Function(f) => format!("function {}(...)", f.name),
            Value::Lambda(_) => "lambda(...)".to_string(),
        }
    }
}
```

---

## Part 8: Critical Implementation Order

### Phase 1: Simple Values Only
1. Implement `SimpleValue` enum
2. Implement basic executor with simple values
3. Get arithmetic, variables, functions working
4. **Milestone**: Can run simple programs without graphs

### Phase 2: Add Graph Infrastructure
1. Implement `DataLayer` with nodes and edges
2. Implement basic `GraphValue` (no behaviors/rules yet)
3. Implement tree as a graph with basic rules
4. **Milestone**: Can create and use trees

### Phase 3: Add Behavior Layer
1. Implement `BehaviorLayer` with transformations
2. Implement list/map promotion to graphs
3. Test behavior application
4. **Milestone**: Behaviors work on collections

### Phase 4: Add Control Layer
1. Implement `ValidationContext`
2. Implement built-in rule validators
3. Implement user-defined rule execution
4. **Milestone**: Full rule system working

### Phase 5: Complete the System
1. Implement `MetadataLayer`
2. Implement introspection APIs
3. Add edge governance
4. **Milestone**: Full five-layer architecture

---

## Part 9: Key Design Decisions Summary

| Decision | Choice | Rationale |
|----------|--------|-----------|
| **Simple vs Graph** | Two-tier with promotion | Performance + power when needed |
| **Node IDs** | String-based | Rust-friendly, debuggable, simple |
| **Layer Access** | Immutable context | Rules can't mutate, only validate |
| **User Rule Access** | Introspection wrapper | Safe, limited API exposure |
| **Behavior Timing** | Eager (on insert) | Predictable, no hidden transforms |
| **Value Enum** | Simple/Graph/Function | Clear categorization |

---

## Part 10: Open Questions

### 1. Executor Ownership in ControlLayer

**Question**: User-defined rules need to call user functions. How does ControlLayer get access to Executor?

**Options**:
- A) Pass executor reference to validate()
- B) Store Executor reference in ControlLayer
- C) Have a separate "rule executor" that's simpler

**Recommendation**: Option A - pass executor reference when validating user rules.

### 2. Graph Cloning for Immutability

**Question**: Do we clone graphs for every modification or use interior mutability?

**Options**:
- A) Clone on write (functional style) - more memory, simpler
- B) Interior mutability with RefCell - less memory, more complex
- C) Copy-on-write with Rc - best of both?

**Recommendation**: Start with A (clone), optimize to C if needed.

### 3. Thread Safety

**Question**: Do we need thread-safe graphs from the start?

**Answer**: No. Start with single-threaded. Use Arc/Mutex later if needed for parallel execution.

---

## Conclusion

This architecture provides:

✅ **Clear separation** between simple values and graph values
✅ **Safe rule access** through immutable validation contexts
✅ **User function isolation** via introspection wrappers
✅ **Rust-friendly design** with no Rc/RefCell nightmares
✅ **Incremental implementation** path with clear milestones
✅ **Performance** through two-tier system
✅ **Power** through full five-layer architecture when needed

The key insight: **Start simple, promote when needed**. Most data never needs graph features. When it does, we have the full power available.

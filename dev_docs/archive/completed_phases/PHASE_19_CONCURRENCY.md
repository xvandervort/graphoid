# Phase 19: Concurrency & Parallelism (Graph-Native)

> **SUPERSEDED**: This monolithic spec was replaced in February 2026 with a sub-phase breakdown (19.1-19.6). The concurrency syntax is now defined in `dev_docs/LANGUAGE_SPECIFICATION.md` § Concurrency. See `ROADMAP_INDEX.md` for the current sub-phase structure.

**Duration**: 14-18 days
**Priority**: Critical
**Dependencies**: Phase 15 (Namespace Graph), Phase 16 (Execution Graph)
**Status**: Ready to Begin

---

## Goal

Implement concurrency where **actors ARE nodes** and **channels ARE edges**. The graph IS the concurrent system—not a separate runtime with graph-like API.

**Key Insight**: Since Phases 15-18 make namespace and execution graph-native, concurrency should extend this model rather than create parallel infrastructure.

---

## Core Principle: Actors ARE Nodes

```
┌─────────────────────────────────────────────────────────────────────┐
│                   TRADITIONAL ACTOR SYSTEM                           │
│                                                                     │
│  ActorRegistry (HashMap<ID, Actor>)                                 │
│       │                                                             │
│       ├── Actor("alice") { state, mailbox }                         │
│       ├── Actor("bob") { state, mailbox }                           │
│       └── Actor("charlie") { state, mailbox }                       │
│                                                                     │
│  MessageRouter (separate from graph)                                │
│  SupervisorTree (separate from graph)                               │
│                                                                     │
│  ❌ Actors are NOT in the universe graph                            │
│  ❌ Graph operations don't work on actors                           │
│  ❌ Two parallel systems to maintain                                 │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│                   GRAPH-NATIVE ACTOR SYSTEM                          │
│                                                                     │
│  Universe Graph                                                     │
│       │                                                             │
│       ├── scope:global                                              │
│       │       ├── var:alice ──binds_to──► node:actor_alice          │
│       │       ├── var:bob ──binds_to──► node:actor_bob              │
│       │       └── var:charlie ──binds_to──► node:actor_charlie      │
│       │                                                             │
│       ├── node:actor_alice ──mailbox──► queue:alice_mail            │
│       │       │                                                     │
│       │       └── supervises ──► node:actor_bob                     │
│       │                                                             │
│       └── channel:alice_bob ──from──► alice ──to──► bob             │
│                                                                     │
│  ✅ Actors ARE nodes in universe graph                              │
│  ✅ Supervision IS edges (supervises)                                │
│  ✅ Channels ARE edges with queue property                           │
│  ✅ Graph operations work on everything                              │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Architecture: Extending the Graph Model

### Actor = Node with Behavior

An actor is just a node with:
- A `mailbox` edge pointing to a queue node
- A `behavior` edge pointing to a handler function
- A `state` property storing current state

```rust
// NOT a separate struct - actors live in the namespace graph
impl NamespaceGraph {
    /// Spawn an actor - adds node to graph, returns reference
    pub fn spawn_actor(&mut self, name: &str, behavior: Value) -> NodeId {
        // Create actor node
        let actor_id = self.graph.add_node(
            format!("actor:{}", name),
            Value::Actor(ActorState::new())
        );

        // Create mailbox queue node
        let mailbox_id = self.graph.add_node(
            format!("mailbox:{}", name),
            Value::Queue(VecDeque::new())
        );

        // Actor -> mailbox edge
        self.graph.add_edge(
            &actor_id.to_string(),
            &mailbox_id.to_string(),
            "mailbox".to_string(),
            None,
            HashMap::new()
        );

        // Actor -> behavior edge
        let behavior_id = self.store_value(behavior);
        self.graph.add_edge(
            &actor_id.to_string(),
            &behavior_id.to_string(),
            "behavior".to_string(),
            None,
            HashMap::new()
        );

        // Bind to variable in current scope
        self.define(name, Value::ActorRef(actor_id));

        actor_id
    }
}
```

### Channel = Edge with Queue

A channel is an edge between nodes with queue semantics:

```rust
// Channels are edges, not separate structs
impl NamespaceGraph {
    pub fn create_channel(&mut self, from: &str, to: &str) -> EdgeId {
        // Channel is just an edge with a queue property
        let edge_id = self.graph.add_edge(
            from,
            to,
            "channel".to_string(),
            None,
            hashmap!{
                "queue" => Value::Queue(VecDeque::new()),
                "capacity" => Value::Number(100.0),  // bounded
            }
        );

        edge_id
    }

    pub fn send_via_channel(&mut self, edge_id: EdgeId, msg: Value) -> Result<()> {
        let edge = self.graph.get_edge_mut(edge_id)?;
        let queue = edge.properties.get_mut("queue")?;
        queue.push_back(msg);
        Ok(())
    }

    pub fn receive_from_channel(&mut self, edge_id: EdgeId) -> Option<Value> {
        let edge = self.graph.get_edge_mut(edge_id)?;
        let queue = edge.properties.get_mut("queue")?;
        queue.pop_front()
    }
}
```

### Supervision = Edge Relationship

Supervision is just an edge type:

```rust
impl NamespaceGraph {
    pub fn supervise(&mut self, supervisor: NodeId, child: NodeId, strategy: RestartStrategy) {
        self.graph.add_edge(
            &supervisor.to_string(),
            &child.to_string(),
            "supervises".to_string(),
            None,
            hashmap!{
                "strategy" => Value::from(strategy),
            }
        );
    }

    pub fn get_supervisor(&self, actor: NodeId) -> Option<NodeId> {
        // Find incoming "supervises" edge
        self.graph.edges_to(&actor.to_string())
            .iter()
            .find(|e| e.label == "supervises")
            .map(|e| NodeId::from_string(&e.from))
    }

    pub fn get_children(&self, supervisor: NodeId) -> Vec<NodeId> {
        self.graph.edges_from(&supervisor.to_string())
            .iter()
            .filter(|e| e.label == "supervises")
            .map(|e| NodeId::from_string(&e.to))
            .collect()
    }
}
```

---

## Graphoid Syntax

### Spawning Actors

```graphoid
# Actor with inline behavior
counter = spawn {
    count = 0

    fn on_message(msg) {
        match msg {
            :increment => { count = count + 1 }
            :get => { return count }
        }
    }
}

# Actor from class
actor CounterActor {
    count = 0

    fn on_message(msg) {
        match msg {
            [:add, n] => { count = count + n }
            :reset => { count = 0 }
            :get => { return count }
        }
    }
}

counter = spawn CounterActor { count: 100 }
```

### Sending Messages (Graph Traversal)

```graphoid
# Send to specific actor (by reference)
counter.send(:increment)

# Send to actor by name (graph lookup)
send(:increment, to: "counter")

# Send along edge (channel)
alice.send_via("output", [:data, payload])

# Broadcast to all actors matching predicate
broadcast(:ping, where: a => a.type == :worker)

# Send to neighbors in graph
node.send_to_neighbors("CONNECTED", :heartbeat)
```

### Request-Response

```graphoid
# Async request
result = await counter.request(:get)

# Request with timeout
result = await counter.request(:get, timeout: 1000)

# Request to multiple
results = await request(:status, where: a => a.type == :server)
```

### Channels

```graphoid
# Create channel (edge between actors)
ch = channel(from: producer, to: consumer)

# Or create standalone channel nodes
input = channel()
output = channel()

# Send/receive
ch.send("hello")
msg = ch.receive()        # blocking
msg = ch.try_receive()    # non-blocking
msg = await ch.async_receive()  # async

# Buffered channel
ch = channel(capacity: 100)
```

### Select

```graphoid
select {
    ch1.receive() as msg => handle_ch1(msg)
    ch2.receive() as msg => handle_ch2(msg)
    actor.message() as msg => handle_actor(msg)
    timeout(5000) => handle_timeout()
    default => nothing_ready()
}
```

### Supervision

```graphoid
# Supervision via graph edges
supervisor = spawn Supervisor {
    strategy: :one_for_one,
    max_restarts: 3
}

worker1 = spawn Worker{}
worker2 = spawn Worker{}

# These create "supervises" edges
supervisor.supervise(worker1)
supervisor.supervise(worker2)

# Or declarative
supervisor = spawn_supervisor({
    strategy: :one_for_all,
    children: [
        { actor: DatabaseWorker, restart: :permanent },
        { actor: CacheWorker, restart: :transient }
    ]
})
```

### Async/Await

```graphoid
async fn fetch_data(url) {
    response = await http.get(url)
    return response["body"]
}

async fn fetch_all(urls) {
    futures = urls.map(url => fetch_data(url))
    return await all(futures)
}
```

---

## Graph-Native Message Routing

Since actors ARE nodes, message routing IS graph traversal:

```graphoid
# These are all graph operations!

# By ID - direct node lookup
graph.send(:msg, to: "alice")

# By predicate - filter nodes
graph.send(:msg, where: n => n.get("status") == "pending")

# By subgraph - named subgraph
graph.define_subgraph("admins", where: n => n.get("role") == "admin")
graph.send(:alert, to: "admins")

# Along edges - graph traversal
graph.send(:hello, from: "alice", via: "FRIEND")
graph.send(:invite, from: "alice", via: "FRIEND", depth: 2)

# Broadcast - all nodes
graph.broadcast(:heartbeat)
```

### Implementation

```rust
impl NamespaceGraph {
    /// Send message - this IS graph traversal
    pub fn send_message(&mut self, msg: Value, target: MessageTarget) -> Result<()> {
        let target_nodes = match target {
            MessageTarget::ById(id) => {
                vec![self.graph.get_node(&id)?]
            }
            MessageTarget::ByPredicate(pred) => {
                self.graph.nodes()
                    .filter(|n| n.is_actor() && pred(n))
                    .collect()
            }
            MessageTarget::BySubgraph(name) => {
                self.get_subgraph(&name)?.nodes().collect()
            }
            MessageTarget::FromVia { from, edge_type, depth } => {
                self.graph.traverse_bfs(&from, &edge_type, depth)
                    .filter(|n| n.is_actor())
                    .collect()
            }
            MessageTarget::Broadcast => {
                self.graph.nodes()
                    .filter(|n| n.is_actor())
                    .collect()
            }
        };

        for node in target_nodes {
            self.enqueue_message(node.id, msg.clone())?;
        }

        Ok(())
    }

    fn enqueue_message(&mut self, actor_id: NodeId, msg: Value) -> Result<()> {
        // Find mailbox via edge traversal
        let mailbox_edge = self.graph.edges_from(&actor_id.to_string())
            .iter()
            .find(|e| e.label == "mailbox")
            .ok_or("Actor has no mailbox")?;

        let mailbox = self.graph.get_node_mut(&mailbox_edge.to)?;
        mailbox.as_queue_mut()?.push_back(msg);

        // Wake up actor if sleeping
        self.scheduler.wake(actor_id);

        Ok(())
    }
}
```

---

## Runtime: M:N Scheduler

The scheduler runs actors, but actors ARE graph nodes:

```rust
pub struct Scheduler {
    /// Reference to universe graph
    universe: Arc<RwLock<NamespaceGraph>>,

    /// Active tasks (running actor message handlers)
    tasks: Vec<Task>,

    /// Worker threads
    workers: Vec<JoinHandle<()>>,

    /// Ready queue (actors with pending messages)
    ready: crossbeam::deque::Injector<NodeId>,
}

impl Scheduler {
    /// Main loop - process actor messages
    pub fn run(&mut self) {
        loop {
            // Steal work from ready queue
            if let Some(actor_id) = self.steal_work() {
                // Get actor from graph
                let mut universe = self.universe.write();
                let actor = universe.graph.get_node(&actor_id.to_string())?;

                if let Some(msg) = self.dequeue_message(&mut universe, actor_id) {
                    // Get behavior via edge traversal
                    let behavior = self.get_behavior(&universe, actor_id)?;

                    // Execute message handler
                    drop(universe);  // Release lock during execution
                    let result = self.execute_handler(actor_id, behavior, msg);

                    // Handle result (may spawn, send, etc.)
                    self.handle_result(actor_id, result);
                }
            }
        }
    }

    fn get_behavior(&self, universe: &NamespaceGraph, actor_id: NodeId) -> Result<Value> {
        // Behavior is found via graph edge traversal!
        let behavior_edge = universe.graph.edges_from(&actor_id.to_string())
            .iter()
            .find(|e| e.label == "behavior")
            .ok_or("Actor has no behavior")?;

        universe.graph.get_node(&behavior_edge.to)?.value().clone()
    }
}
```

---

## Virtual Actors (Activation/Passivation)

Virtual actors are nodes that can be passivated to disk and rehydrated:

```rust
impl NamespaceGraph {
    /// Passivate idle actor - serialize to storage
    pub fn passivate(&mut self, actor_id: NodeId) -> Result<()> {
        let actor = self.graph.get_node(&actor_id.to_string())?;

        // Serialize actor state
        let serialized = serialize_actor(actor)?;

        // Store to persistence layer
        self.storage.write(&actor_id.to_string(), serialized)?;

        // Replace with placeholder (node stays in graph!)
        self.graph.set_node_property(
            &actor_id.to_string(),
            "state",
            Value::Passivated
        );

        Ok(())
    }

    /// Rehydrate actor - deserialize from storage
    pub fn hydrate(&mut self, actor_id: NodeId) -> Result<()> {
        let actor = self.graph.get_node(&actor_id.to_string())?;

        if !actor.is_passivated() {
            return Ok(());  // Already active
        }

        // Load from storage
        let serialized = self.storage.read(&actor_id.to_string())?;
        let state = deserialize_actor(serialized)?;

        // Restore state
        self.graph.set_node_property(
            &actor_id.to_string(),
            "state",
            state
        );

        Ok(())
    }

    /// Ensure actor is hydrated before sending
    pub fn send_with_hydration(&mut self, actor_id: NodeId, msg: Value) -> Result<()> {
        self.hydrate(actor_id)?;
        self.enqueue_message(actor_id, msg)
    }
}
```

---

## Data Parallelism

Parallel collection operations via Rayon, but on graph-backed collections:

```graphoid
# Parallel operations on lists
results = list.parallel_map(item => expensive_compute(item))
filtered = list.parallel_filter(item => item.is_valid())
total = list.parallel_reduce(0, (acc, item) => acc + item.value)

# Parallel graph traversal
results = graph.parallel_traverse(start, fn(node) {
    return expensive_analysis(node)
})
```

```rust
impl ListValue {
    pub fn parallel_map(&self, func: &Function) -> Result<ListValue> {
        use rayon::prelude::*;

        let results: Vec<Value> = self.elements
            .par_iter()
            .map(|elem| execute_function(func, vec![elem.clone()]))
            .collect::<Result<Vec<_>>>()?;

        Ok(ListValue::from(results))
    }
}
```

---

## Example: PageRank (Graph-Native)

```graphoid
# PageRank using actors-as-nodes

# Each node in the graph becomes an actor
fn make_pagerank_graph(edges) {
    g = graph{}

    for [from, to] in edges {
        # Add actor nodes
        if not g.has_node(from) {
            g.add_node(from, spawn PageRankNode{})
        }
        if not g.has_node(to) {
            g.add_node(to, spawn PageRankNode{})
        }
        g.add_edge(from, to, "LINK")
    }

    return g
}

actor PageRankNode {
    rank = 1.0
    incoming = 0.0
    damping = 0.85
    n = 1

    fn on_message(msg) {
        match msg {
            [:init, params] => {
                n = params["n"]
                rank = 1.0 / n
            }
            :send_contributions => {
                contribution = rank / self.out_degree()
                self.send_to_neighbors("LINK", [:contribution, contribution])
            }
            [:contribution, amount] => {
                incoming = incoming + amount
            }
            :update_rank => {
                rank = (1 - damping) / n + damping * incoming
                incoming = 0.0
            }
            :get_rank => {
                return rank
            }
        }
    }
}

# Run PageRank
fn pagerank(g, iterations) {
    n = g.node_count()

    # Initialize all
    g.broadcast([:init, { n: n }])

    # Iterate
    for i in range(iterations) {
        g.broadcast(:send_contributions)
        await g.barrier()  # Wait for all contributions
        g.broadcast(:update_rank)
    }

    # Collect results
    ranks = await g.request(:get_rank, to_all: true)
    return ranks
}
```

---

## Implementation Plan

### Week 1: Actor Nodes

| Day | Task |
|-----|------|
| 1-2 | Extend NamespaceGraph with spawn_actor, actor state |
| 3-4 | Mailbox as edge, message enqueueing |
| 5 | Actor behavior lookup via edge traversal |

### Week 2: Channels and Routing

| Day | Task |
|-----|------|
| 6-7 | Channels as edges with queue property |
| 8-9 | Message routing via graph traversal |
| 10 | Select statement (multiplex channels) |

### Week 3: Scheduler and Supervision

| Day | Task |
|-----|------|
| 11-12 | M:N scheduler, work stealing |
| 13-14 | Supervision as edges, restart strategies |
| 15 | Virtual actors (passivation/hydration) |

### Week 4: Async and Parallelism

| Day | Task |
|-----|------|
| 16-17 | Async/await, futures |
| 18-19 | Data parallelism (parallel_map, etc.) |
| 20-21 | Integration testing, benchmarks |

---

## Success Criteria

### Architectural

- [ ] Actors ARE nodes in namespace graph
- [ ] Channels ARE edges with queue property
- [ ] Supervision IS edge relationships
- [ ] Message routing IS graph traversal
- [ ] No separate ActorRegistry or MessageRouter structs

### Functional

- [ ] spawn creates actor node
- [ ] send routes via graph
- [ ] Channels work (send/receive/select)
- [ ] Supervision restarts children
- [ ] Async/await works
- [ ] Parallel collections work

### Performance

- [ ] 1M+ messages/second
- [ ] ≤1.5x overhead vs non-graph actors
- [ ] Efficient work stealing

---

## Related Documents

- [PHASE_15_NAMESPACE_GRAPH.md](PHASE_15_NAMESPACE_GRAPH.md) - Actors live here
- [PHASE_16_EXECUTION_GRAPH.md](PHASE_16_EXECUTION_GRAPH.md) - Message handlers execute here
- [PHASE_23_DISTRIBUTED_PRIMITIVES.md](PHASE_23_DISTRIBUTED_PRIMITIVES.md) - Remote actors
- [PHASE_24_DISTRIBUTED_EXECUTION.md](PHASE_24_DISTRIBUTED_EXECUTION.md) - Distributed algorithms
- [CONCURRENCY_MODEL_RATIONALE.md](CONCURRENCY_MODEL_RATIONALE.md) - Design rationale

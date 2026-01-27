# Graphoid Concurrency Model: Design Rationale

**Document Status**: Architectural Decision Record
**Date**: January 2026
**Decision**: M:N Green Threads with Virtual Actors and Graph-Native Messaging

---

## Executive Summary

Graphoid adopts an **M:N green thread model with virtual actors and graph-native messaging**:

1. **Virtual Actors** - Nodes are always addressable; runtime handles hydration/passivation
2. **Graph-Native Messaging** - Specify WHAT to reach, not HOW (declarative, not imperative)
3. **Query Routing** - Runtime routes based on partition strategy, not omniscient metadata

**Core principle**: "Send messages to graph specifications, not individual nodes."

---

## The Evolution of the Design

### Stage 1: Explicit Activation (Rejected)

```graphoid
# BAD - explicit lifecycle management
node.activate()
node.send(:msg)
node.deactivate()
```

**Problem**: Doesn't scale. Can't activate a billion nodes.

### Stage 2: Implicit Activation / Virtual Actors (Better)

```graphoid
# BETTER - just use nodes
node.send(:msg)  # Runtime hydrates automatically
```

**Problem**: Still requires knowing specific node IDs. Imperative.

### Stage 3: Graph-Native Messaging (Final Design)

```graphoid
# BEST - declarative, graph-native
graph.send(:msg, to: "alice")           # By ID
graph.send(:msg, where: predicate)      # By query
graph.send(:msg, to: subgraph)          # By subgraph
graph.send(:msg, matching: pattern)     # By pattern
graph.send(:msg, from: "x", via: "EDGE") # Along edges
graph.broadcast(:msg)                    # To all
```

**Why this is right**: You specify WHAT you want to reach. The runtime figures out HOW.

---

## Graph-Native Messaging

### The Key Insight

In a graph-centric language, you shouldn't think about individual nodes. You should think about:
- Subgraphs
- Patterns
- Predicates
- Paths and neighborhoods

The messaging API reflects this.

### Messaging Patterns

| Pattern | Syntax | Use Case |
|---------|--------|----------|
| **By ID** | `graph.send(:msg, to: "alice")` | Known specific node |
| **By Query** | `graph.send(:msg, where: predicate)` | Nodes matching condition |
| **By Subgraph** | `graph.send(:msg, to: subgraph)` | Pre-defined node set |
| **By Pattern** | `graph.send(:msg, matching: "(a)-[:X]->(b)")` | Structural pattern |
| **By Traversal** | `graph.send(:msg, from: "x", via: "EDGE")` | Along edges |
| **Broadcast** | `graph.broadcast(:msg)` | All nodes |

### Request-Response

```graphoid
# Get responses from matching nodes
results = await graph.request(:get_status, where: n => n.type == "Server")
# Returns list of responses from all matching nodes
```

### Named Subgraphs

```graphoid
# Define once
graph.define_subgraph("admins", where: n => n.get("role") == "admin")
graph.define_subgraph("vips", matching: "(n:User)-[:HAS]->(:Premium)")

# Use many times
graph.send(:alert, to: "admins")
graph.send(:offer, to: "vips")
```

---

## Virtual Actors

### How They Work

```
┌─────────────────────────────────────────────────────────────┐
│                   Node Lifecycle (Automatic)                 │
│                                                             │
│   VIRTUAL ───> HYDRATING ───> ACTIVE ───> PASSIVATING ───> │
│      ▲                           │                    │     │
│      └───────────────────────────┴────────────────────┘     │
│                                                             │
│   Message arrives → route → hydrate if needed → deliver    │
│   Idle timeout → passivate → back to virtual               │
│                                                             │
│   User never manages this. Just send messages.             │
└─────────────────────────────────────────────────────────────┘
```

### Memory Efficiency

With a billion-node graph:

```
┌─────────────────────────────────────────────────────────────┐
│              Graph with 1 Billion Nodes                      │
│                                                             │
│   [virtual] [virtual] [HYDRATED] [virtual] [virtual] ...   │
│                                                             │
│   • Only ~1000 nodes in memory (configurable)              │
│   • 999,999,000 nodes are virtual                          │
│   • Hydrate on demand, passivate when idle                 │
│   • LRU or configurable eviction                           │
└─────────────────────────────────────────────────────────────┘
```

---

## Query Routing (Not Omniscient Planning)

### The Problem

With a billion distributed nodes, no "query planner" can have all metadata.

### What the Runtime Knows

| Runtime KNOWS | Runtime DOESN'T KNOW |
|---------------|---------------------|
| Partition topology | Individual node data |
| Partitioning strategy (hash/range) | Which specific nodes exist |
| Partition locations (host:port) | Actual paths between nodes |
| Aggregate statistics | Node attribute values |

### How Routing Works

```graphoid
# graph.send(:msg, to: "alice")
# Runtime: hash("alice") → partition 7 → route → hydrate → deliver

# graph.send(:msg, where: predicate)
# Runtime: scatter to all partitions → each filters locally → deliver

# graph.send(:msg, from: "x", via: "FRIEND")
# Runtime: find x's FRIEND edges → route to each target → deliver
```

---

## Algorithm Expression

### PageRank with Graph-Native Messaging

```graphoid
fn pagerank(graph, iterations, damping) {
    n = graph.node_count()

    # Initialize all nodes
    graph.broadcast(:init, { rank: 1.0 / n, damping: damping, n: n })

    for i in range(iterations) {
        # Each node sends contributions to neighbors
        graph.broadcast(:send_contributions)

        # Each node updates its rank
        graph.broadcast(:update_rank)
    }
}

# Node behavior
graph PageRankNode {
    rank = 0.0
    incoming = 0.0

    fn on_message(msg) {
        match msg {
            [:init, params] => {
                rank = params["rank"]
            }
            :send_contributions => {
                contribution = rank / self.out_degree()
                self.send_to_neighbors("*", [:contribution, contribution])
            }
            [:contribution, amount] => {
                incoming = incoming + amount
            }
            :update_rank => {
                rank = (1 - damping) / n + damping * incoming
                incoming = 0.0
            }
        }
    }
}
```

The algorithm is expressed as:
1. Broadcasts to all nodes
2. Local node behavior
3. Edge-based communication

No explicit loops over nodes. No activation management. Just graph operations.

---

## Comparison with Other Systems

| Feature | Erlang/OTP | Orleans | Akka | Graphoid |
|---------|-----------|---------|------|----------|
| Actor model | Yes | Yes (Virtual) | Yes | Yes (Virtual) |
| Explicit addressing | Yes (PID) | Yes (Grain ID) | Yes (ActorRef) | **Optional** |
| Query-based addressing | No | No | No | **Yes** |
| Pattern-based addressing | No | No | No | **Yes** |
| Subgraph operations | No | No | No | **Yes** |
| Graph-native | No | No | No | **Yes** |

Graphoid is unique in supporting declarative, graph-native addressing.

---

## Edge Duality: Structure vs Channel

Edges serve two purposes:

| Purpose | When | Description |
|---------|------|-------------|
| **Structural** | Always | Graph topology for queries |
| **Communication** | When used | Message channel between nodes |

```graphoid
# Structural - query the graph
path = graph.shortest_path("alice", "david")
neighbors = graph.neighbors("alice")

# Communication - send along edges
graph.send(:hello, from: "alice", via: "FRIEND")
```

You don't "activate" edges. You just use them.

---

## Implementation Strategy

### Message Router

```rust
enum MessageTarget {
    ById(String),
    ByPredicate(Function),
    BySubgraph(SubgraphRef),
    ByPattern(Pattern),
    FromVia { from: String, edge_type: String, depth: usize },
    Broadcast,
}

impl MessageRouter {
    async fn send(&self, msg: Message, target: MessageTarget) -> Result<()> {
        match target {
            MessageTarget::ById(id) => self.send_to_node(&id, msg).await,
            MessageTarget::ByPredicate(pred) => self.scatter_with_predicate(msg, pred).await,
            MessageTarget::FromVia { from, edge_type, depth } => {
                let neighbors = self.get_neighbors(&from, &edge_type, depth).await?;
                for neighbor in neighbors {
                    self.send_to_node(&neighbor, msg.clone()).await?;
                }
                Ok(())
            }
            MessageTarget::Broadcast => self.broadcast(msg).await,
            // ...
        }
    }
}
```

### Virtual Actor Registry

```rust
struct VirtualActorRegistry {
    actors: DashMap<String, VirtualActor>,
    max_hydrated: usize,
    idle_timeout: Duration,
}

impl VirtualActorRegistry {
    async fn send(&self, id: &str, msg: Message) -> Result<()> {
        self.ensure_hydrated(id).await?;
        self.actors.get(id).unwrap().mailbox.send(msg).await
    }
}
```

---

## Success Criteria

The concurrency model is successful if:

1. **Declarative** - Users specify WHAT, not HOW
2. **Scalable** - Works for 100 nodes or 100 billion
3. **Graph-Native** - Messaging uses graph concepts (subgraphs, patterns, edges)
4. **Transparent** - Same code works local or distributed
5. **Simple** - No lifecycle management, no explicit routing

---

## Conclusion

Graph-native messaging with virtual actors is the right choice for Graphoid because:

1. **Declarative** - Specify what to reach, not how
2. **Graph-Native** - Uses subgraphs, patterns, predicates, not just IDs
3. **Scalable** - Virtual actors handle billions of nodes
4. **Simple** - No activation, no lifecycle management
5. **Aligned** - Reflects "everything is a graph" philosophy

This model makes distributed graph programming feel like local graph programming.

---

## References

- Bykov et al. "Orleans: Distributed Virtual Actors" (Microsoft Research)
- Armstrong, Joe. "Making reliable distributed systems" (Erlang thesis)
- Hewitt, Carl. "A Universal Modular ACTOR Formalism"

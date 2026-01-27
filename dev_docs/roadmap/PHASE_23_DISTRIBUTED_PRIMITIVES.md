# Phase 23: Distribution Primitives

**Duration**: 12-16 days
**Priority**: High
**Dependencies**: Phase 15 (Concurrency), Phase 20 (FFI)
**Status**: Planning

---

## Goal

Provide **language-level primitives** that enable building multiple distribution models on top of Graphoid. This phase does NOT implement a specific distributed computing model - instead, it provides the building blocks that a higher-level platform (see Phase 30: Graphoid Platform) can use to implement Pregel, MapReduce, Actors, CRDTs, and other paradigms.

**Key principle**: The language provides primitives. The platform provides patterns.

---

## Why This Approach

Rather than baking one distribution model into the language (like Erlang's actors), we provide flexible primitives because:

1. **Different problems need different models** - ML training wants parameter servers, graph algorithms want Pregel, IoT wants actors
2. **Models evolve** - New paradigms emerge; primitives are stable
3. **Community-driven** - Platform patterns can be developed independently
4. **Graphoid's core insight is "everything is a graph"** - Distribution is orthogonal

```
┌─────────────────────────────────────────────────────────────────┐
│  Graphoid Platform (separate project, Phase 30)                  │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌────────────┐ │
│  │   Pregel    │ │   Actors    │ │  MapReduce  │ │   CRDTs    │ │
│  │   Model     │ │   Model     │ │   Model     │ │   Model    │ │
│  └─────────────┘ └─────────────┘ └─────────────┘ └────────────┘ │
├─────────────────────────────────────────────────────────────────┤
│  Graphoid Language - Phase 23 Primitives                         │
│  • Serialization       • Remote References                      │
│  • Network I/O         • Partitioning API                       │
│  • Message Routing     • Distribution Hooks                     │
└─────────────────────────────────────────────────────────────────┘
```

---

## Core Primitives

### 1. Serialization

Convert graphs and values to bytes for network transmission.

```graphoid
import "serialize"

# Serialize a graph
bytes = serialize.to_bytes(my_graph)
restored = serialize.from_bytes(bytes)

# Format options
json_str = serialize.to_json(my_graph)
msgpack = serialize.to_msgpack(my_graph, { compact: true })

# Streaming for large graphs
stream = serialize.stream(large_graph)
for chunk in stream {
    network.send(chunk)
}

# Custom serialization for specific types
serialize.register(MyType, {
    encode: fn(value) { ... },
    decode: fn(bytes) { ... }
})
```

### 2. Remote References

Abstract handle to "something that lives elsewhere". The language doesn't know or care what distribution model is used - it just provides the abstraction.

```graphoid
import "remote"

# Create a remote reference (abstract - platform defines semantics)
ref = remote.ref({
    id: "node_123",
    location: "partition_2",   # Platform-defined
    metadata: { type: "user" } # Optional metadata
})

# Check if reference is local or remote
ref.is_local()    # false
ref.is_remote()   # true

# Get location info (platform-defined structure)
ref.location()    # "partition_2"

# Resolve to actual value (may involve network)
value = await ref.resolve()

# Send message to remote ref (platform handles routing)
await ref.send(:message, payload)

# Request-response pattern
result = await ref.request(:get_data, { key: "foo" })
```

### 3. Network Abstractions

Low-level network I/O that platforms can build on.

```graphoid
import "net"

# TCP connections
conn = await net.connect("10.0.1.100", 9000)
await conn.send(bytes)
received = await conn.receive()
conn.close()

# Listen for connections
server = net.listen("0.0.0.0", 9000)
server.on_connection(fn(conn) {
    data = await conn.receive()
    await conn.send(response)
})

# UDP for low-latency messaging
udp = net.udp_socket(9001)
await udp.send_to(bytes, "10.0.1.100", 9001)
[data, addr] = await udp.receive_from()

# Connection pooling
pool = net.pool({
    host: "10.0.1.100",
    port: 9000,
    min_connections: 5,
    max_connections: 20
})
conn = await pool.acquire()
# ... use connection ...
pool.release(conn)
```

### 4. Message Routing Hooks

Extensible routing system that platforms can customize.

```graphoid
import "routing"

# Register a router (platform provides implementation)
routing.register("my_platform", {
    # Called when sending to a remote ref
    route: fn(ref, message) {
        partition = hash(ref.id) % partition_count
        connection = get_connection(partition)
        await connection.send(serialize.to_bytes({
            target: ref.id,
            message: message
        }))
    },

    # Called when receiving a message
    receive: fn(bytes) {
        msg = serialize.from_bytes(bytes)
        local_handle = lookup(msg.target)
        local_handle.deliver(msg.message)
    }
})

# Activate a router
routing.use("my_platform")
```

### 5. Partitioning API

Primitives for dividing graphs across locations.

```graphoid
import "partition"

# Partitioning strategies (platform can add more)
partition.register_strategy("hash", fn(node_id, partition_count) {
    return hash(node_id) % partition_count
})

partition.register_strategy("range", fn(node_id, ranges) {
    for range in ranges {
        if node_id >= range.min and node_id < range.max {
            return range.partition
        }
    }
    return "default"
})

# Use a strategy
strategy = partition.strategy("hash", { partitions: 8 })
partition_id = strategy.partition_for("alice")  # e.g., 3

# Partition a graph
partitions = partition.split(my_graph, strategy)
# Returns: { 0: subgraph, 1: subgraph, ... }

# Identify cross-partition edges
cross_edges = partition.cross_edges(my_graph, strategy)
# Returns edges where from and to are in different partitions
```

### 6. Distribution Hooks

Extension points for platforms to intercept and customize behavior.

```graphoid
import "hooks"

# Hook into graph operations for distribution awareness
hooks.on_add_node(fn(graph, node_id, value) {
    if graph.is_distributed() {
        partition = graph.partition_strategy.partition_for(node_id)
        if partition != local_partition {
            return remote.create_on(partition, node_id, value)
        }
    }
    # Default local behavior
    return :continue
})

hooks.on_add_edge(fn(graph, from, to, label) {
    # Platform can handle cross-partition edges
    if different_partitions(from, to) {
        register_cross_partition_edge(from, to, label)
    }
    return :continue
})

hooks.on_query(fn(graph, query) {
    # Platform can intercept queries for distributed execution
    if graph.is_distributed() {
        return distributed_query_plan(query)
    }
    return :continue
})
```

### 7. Distributed Context

Scoped context for distributed operations (tracing, timeouts, etc.).

```graphoid
import "context"

# Create a context for a distributed operation
ctx = context.create({
    timeout: 5000ms,
    trace: true,
    metadata: { request_id: "abc123" }
})

# Pass context through operations
result = await some_remote_call(ctx, args)

# Context propagates automatically to child operations
ctx.with_timeout(1000ms) {
    # Operations here have 1s timeout
    await fast_operation()
}

# Access trace information
ctx.trace.spans()  # List of traced operations
```

---

## Universe Graph Integration

Distribution primitives integrate with the universe graph (Phase 18):

```
universe
├── modules/
├── packages/
├── connections/
│
└── distribution/           # New namespace for distribution state
    ├── local_identity      # This node's identity
    ├── peers/              # Known remote peers (Bridge Nodes)
    │   ├── peer_1 ──────► foreign: 10.0.1.100:9000
    │   └── peer_2 ──────► foreign: 10.0.1.101:9000
    ├── routers/            # Active routing configurations
    └── partitions/         # Partition metadata (if applicable)
```

```graphoid
# Query distribution state
peers = reflect.universe().distribution().peers()
for peer in peers {
    print(peer.id + ": " + peer.state)  # "peer_1: connected"
}
```

---

## What This Phase Does NOT Include

The following are **platform concerns**, not language primitives:

- Specific distributed graph implementation
- Pregel-style supersteps
- Actor mailboxes and supervision
- MapReduce job scheduling
- CRDT merge semantics
- Partition registry (etcd, consul, etc.)
- Distributed query planning
- Fault tolerance policies
- Circuit breakers
- Consensus protocols

These belong in Phase 30: Graphoid Platform.

---

## Implementation Plan

### Day 1-3: Serialization

```rust
trait Serializable {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Result<Self>;
}

// Implement for all Value types
impl Serializable for Value { ... }
impl Serializable for Graph { ... }

// Streaming serialization for large graphs
struct GraphStreamSerializer {
    graph: Graph,
    chunk_size: usize,
}
impl Iterator for GraphStreamSerializer { ... }
```

### Day 4-6: Remote References

```rust
struct RemoteRef {
    id: String,
    location: Value,  // Platform-defined structure
    metadata: HashMap<String, Value>,
}

impl RemoteRef {
    fn is_local(&self) -> bool;
    fn is_remote(&self) -> bool;
    async fn resolve(&self) -> Result<Value>;
    async fn send(&self, message: Value) -> Result<()>;
    async fn request(&self, message: Value) -> Result<Value>;
}
```

### Day 7-9: Network Abstractions

```rust
// TCP
struct TcpConnection { ... }
impl TcpConnection {
    async fn connect(host: &str, port: u16) -> Result<Self>;
    async fn send(&self, data: &[u8]) -> Result<()>;
    async fn receive(&self) -> Result<Vec<u8>>;
}

// Connection pool
struct ConnectionPool { ... }
impl ConnectionPool {
    async fn acquire(&self) -> Result<PooledConnection>;
    fn release(&self, conn: PooledConnection);
}

// Server
struct TcpServer { ... }
impl TcpServer {
    fn listen(addr: &str, port: u16) -> Result<Self>;
    fn on_connection(&self, handler: impl Fn(TcpConnection));
}
```

### Day 10-12: Routing & Partitioning

```rust
// Routing
trait Router {
    async fn route(&self, ref: &RemoteRef, message: Value) -> Result<()>;
    async fn receive(&self, bytes: &[u8]) -> Result<()>;
}

struct RouterRegistry {
    routers: HashMap<String, Box<dyn Router>>,
    active: Option<String>,
}

// Partitioning
trait PartitionStrategy {
    fn partition_for(&self, node_id: &str) -> PartitionId;
}

struct PartitionRegistry {
    strategies: HashMap<String, Box<dyn PartitionStrategy>>,
}
```

### Day 13-14: Distribution Hooks

```rust
struct DistributionHooks {
    on_add_node: Vec<Box<dyn Fn(&Graph, &str, &Value) -> HookResult>>,
    on_add_edge: Vec<Box<dyn Fn(&Graph, &str, &str, &str) -> HookResult>>,
    on_query: Vec<Box<dyn Fn(&Graph, &Query) -> HookResult>>,
}

enum HookResult {
    Continue,           // Proceed with default behavior
    Handled(Value),     // Hook handled it, use this result
    Redirect(RemoteRef), // Redirect to remote
}
```

### Day 15-16: Integration & Testing

- Integrate with universe graph
- Unit tests for each primitive
- Integration tests for primitive combinations
- Documentation
- Example: Simple custom router

---

## Success Criteria

### Core Primitives
- [ ] Serialization: to_bytes, from_bytes, streaming
- [ ] Serialization: JSON, MessagePack formats
- [ ] Remote references: create, resolve, send, request
- [ ] Network: TCP connect, send, receive, close
- [ ] Network: TCP server, connection handling
- [ ] Network: Connection pooling
- [ ] Routing: Register custom routers
- [ ] Routing: Route messages through active router
- [ ] Partitioning: Register strategies
- [ ] Partitioning: Split graph by strategy

### Extensibility
- [ ] Hooks: on_add_node, on_add_edge, on_query
- [ ] Custom serializers can be registered
- [ ] Custom partition strategies can be registered
- [ ] Custom routers can be registered

### Integration
- [ ] Distribution namespace in universe graph
- [ ] Peer tracking as Bridge Nodes
- [ ] Context propagation works

### Testing & Documentation
- [ ] At least 80 unit tests
- [ ] Example: Custom echo router
- [ ] Example: Hash partitioning
- [ ] Documentation complete

---

## Example: Building a Simple Router

Shows how a platform might use these primitives:

```graphoid
import "routing"
import "net"
import "serialize"

# Simple hash-based router
routing.register("simple_hash", {
    partitions: {},  # partition_id -> connection

    init: fn(config) {
        for p in config.partitions {
            partitions[p.id] = await net.connect(p.host, p.port)
        }
    },

    route: fn(ref, message) {
        partition_id = hash(ref.id) % partitions.length()
        conn = partitions[partition_id]
        bytes = serialize.to_bytes({
            target: ref.id,
            payload: message
        })
        await conn.send(bytes)
    },

    receive: fn(bytes) {
        msg = serialize.from_bytes(bytes)
        # Deliver to local handler
        local_deliver(msg.target, msg.payload)
    }
})

# Use the router
routing.use("simple_hash")
routing.init({ partitions: [
    { id: 0, host: "10.0.1.100", port: 9000 },
    { id: 1, host: "10.0.1.101", port: 9000 }
]})

# Now remote refs route through this
ref = remote.ref({ id: "user_123" })
await ref.send(:hello)  # Routes via simple_hash
```

---

## Resolved Questions

| Question | Resolution |
|----------|------------|
| Remote nodes in universe graph? | No - only peers (as Bridge Nodes) and local partition metadata |
| One distribution model or many? | Many - language provides primitives, platform provides models |
| Where does routing logic live? | Platform registers routers, language provides hooks |

---

## Open Questions

1. **Error propagation** - How do remote errors surface locally?
2. **Backpressure** - How to handle slow consumers?
3. **Security** - Authentication/encryption at primitive level or platform level?

---

## Related Documents

- [PHASE_15_CONCURRENCY.md](PHASE_15_CONCURRENCY.md) - Async foundation
- [PHASE_18_COMPLETE_GRAPH_MODEL.md](PHASE_18_COMPLETE_GRAPH_MODEL.md) - Universe graph structure
- [PHASE_20_FFI.md](PHASE_20_FFI.md) - Bridge Node pattern
- [GRAPHOID_PLATFORM.md](platform/GRAPHOID_PLATFORM.md) - Platform built on these primitives (separate project)

# Phase 24: Distributed Execution

**Duration**: 24-30 days
**Priority**: High
**Dependencies**: Phase 23 (Distributed Primitives)
**Status**: Blocked on Phase 23

---

## Goal

Enable graphs to execute as distributed programs: computation happens where data lives, with coordination via consensus and fault tolerance via replication.

**Key principle**: A distributed graph IS a distributed program. Nodes are computational units, edges are communication channels. Just write the algorithm - the runtime handles distribution.

---

## Core Concept: Self-Executing Distributed Graphs

```
Traditional:  Code runs → accesses data (pull model)
Graphoid:     Graph IS both code and data → executes itself (push model)
```

---

## Five-Layer Architecture Integration

Distributed execution operates on Graphoid's standard five-layer architecture (see `ARCHITECTURE_DESIGN.md`). This ensures operations target user data, not internal structure.

### Layer Separation for Distributed Graphs

```
┌─────────────────────────────────────────────────────────────────┐
│  Distributed Graph (GraphValue)                                 │
│                                                                 │
│  DataLayer:                                                     │
│    • User data nodes (what broadcast/map/reduce operate on)     │
│    • User-defined edges (relationships between data)            │
│                                                                 │
│  MetadataLayer:                                                 │
│    • partition_assignment: {node_1: "partition_A", ...}         │
│    • replication_info: {node_1: ["replica_1", "replica_2"]}     │
│    • distribution_config: {replication_factor: 3, ...}          │
│                                                                 │
│  BehaviorLayer:                                                 │
│    • Node message handlers (on_message functions)               │
│    • Convergence behaviors                                      │
│                                                                 │
│  ControlLayer:                                                  │
│    • Consistency constraints                                    │
│    • Partition rules                                            │
└─────────────────────────────────────────────────────────────────┘
```

### What This Means for Distributed Operations

| Operation | Operates On | Description |
|-----------|-------------|-------------|
| `graph.broadcast(msg)` | DataLayer nodes | Sends message to all user data nodes |
| `graph.send(msg, from:, via:)` | DataLayer edges | Routes along user-defined edges |
| `distributed.map(graph, fn)` | DataLayer nodes | Maps function over user data nodes |
| `distributed.reduce(graph, init, fn)` | DataLayer nodes | Reduces over user data nodes |
| `graph.nodes()` | DataLayer | Returns user data nodes only |

**Key principle**: `broadcast()`, `map()`, `reduce()`, and `nodes()` operate on the DataLayer only. Partition metadata, replication state, and distribution configuration live in MetadataLayer and don't pollute user operations.

---

### Two Complementary Patterns

| Pattern | Use Case | API |
|---------|----------|-----|
| **Graph-Native Messaging** | Iterative algorithms, actor-style | `graph.broadcast()`, `graph.send()` |
| **Map-Reduce** | Aggregation, transformation | `distributed.map()`, `distributed.reduce()` |

Both patterns are first-class. Graph-native messaging excels at iterative, neighbor-communicating algorithms (PageRank, label propagation). Map-reduce excels at embarrassingly parallel aggregations (word count, sum).

```
┌─────────────────────────────────────────────────────────────┐
│                    Distributed Graph Execution               │
│                                                             │
│   ┌──────────────┐         ┌──────────────┐                │
│   │  Partition 1 │         │  Partition 2 │                │
│   │              │         │              │                │
│   │  [compute]───┼────────▶│───[compute]  │                │
│   │      │       │         │       │      │                │
│   │      ▼       │         │       ▼      │                │
│   │  [compute]   │◀────────┼──[compute]   │                │
│   │              │         │              │                │
│   └──────────────┘         └──────────────┘                │
│                                                             │
│   Nodes execute where they live                            │
│   Messages flow along edges (local or remote)              │
│   Runtime coordinates, handles failures                    │
└─────────────────────────────────────────────────────────────┘
```

---

## Core Features

### 1. Distributed Graph Algorithms

```graphoid
# PageRank - just write the algorithm
# Works identically on local or distributed graphs
fn pagerank(graph, iterations, damping) {
    n = graph.node_count()

    # Initialize all nodes via broadcast
    graph.broadcast(:init, { rank: 1.0 / n, damping: damping, n: n })

    for i in range(iterations) {
        # Each node sends contributions to neighbors
        graph.broadcast(:send_contributions)

        # Each node updates its rank
        graph.broadcast(:update_rank)
    }
}

# Node behavior (defined once)
graph PageRankNode {
    rank = 0.0
    incoming = 0.0
    damping = 0.85
    n = 1

    fn on_message(msg) {
        match msg {
            [:init, params] => {
                rank = params["rank"]
                damping = params["damping"]
                n = params["n"]
            }
            :send_contributions => {
                contribution = rank / self.out_degree()
                # Send along all outgoing edges
                self.graph.send([:contribution, contribution], from: self.id, via: "*")
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

# Just call it - runtime handles distribution
pagerank(billion_node_graph, 10, 0.85)
```

### 2. Map-Reduce on Graphs

```graphoid
# Distributed map over all nodes
results = distributed.map(graph, fn(node) {
    return expensive_computation(node)
})
# Runtime: scatter to partitions, compute locally, gather results

# Distributed reduce
total = distributed.reduce(graph, 0, fn(acc, node) {
    return acc + node.get("value")
})
# Runtime: reduce locally on each partition, then reduce across partitions

# Map-reduce pipeline
word_counts = distributed.map_reduce(
    documents,
    map: fn(doc) {
        words = doc.get("content").split(" ")
        return words.map(w => [w, 1])
    },
    reduce: fn(word, counts) {
        return [word, counts.sum()]
    }
)
```

### 3. Distributed Queries

```graphoid
# Query executes across all partitions
high_degree = distributed.query(graph, fn(partition) {
    return partition.nodes().filter(n => n.degree() > 100)
})
# Runtime: scatter query, execute locally, gather results

# Pattern matching across partitions
triangles = distributed.find_pattern(graph, pattern {
    a -> b [FRIEND]
    b -> c [FRIEND]
    c -> a [FRIEND]
})
# Runtime: coordinate pattern matching across partition boundaries
```

### 4. Consensus for Coordination

```graphoid
# Distributed lock
lock = distributed.lock("resource-name")
await lock.acquire()
# ... critical section ...
lock.release()

# Leader election
leader = distributed.elect_leader("cluster-name")
if leader.is_me() {
    # I'm the leader, coordinate work
    distribute_tasks()
}

# Distributed counter (CRDT)
counter = distributed.counter("visitor-count")
counter.increment()
total = await counter.value()  # Eventually consistent
```

### 5. Fault Tolerance

```graphoid
# Configure replication
graph.configure({
    replication_factor: 3,
    consistency: :quorum  # Read/write from majority
})

# Automatic failover handling
graph.on_partition_failure(fn(partition) {
    print("Partition " + partition + " failed, redirecting to replica")
})

# Manual recovery
failed_partition = graph.get_partition("user-data-east")
await failed_partition.recover_from_replica()
```

### 6. Consistency Models

| Model | Description | Use Case |
|-------|-------------|----------|
| **Strong** | All replicas agree before returning | Financial transactions |
| **Quorum** | Majority agrees | Most applications |
| **Eventual** | Updates propagate over time | High availability |
| **Causal** | Respects causal ordering | Collaborative editing |

```graphoid
# Configure per-operation consistency
result = graph.query("...", consistency: :strong)
graph.execute("...", consistency: :eventual)
```

### 7. Streaming Responses

```graphoid
# For autoregressive generation or large result sets
stream = graph.stream_request(:generate, to: "model", {
    prompt: "Write a story about graphs...",
    max_tokens: 1000
})

# Process tokens as they arrive
for token in stream {
    print(token)  # User sees output incrementally

    if user_cancelled() {
        stream.cancel()
        break
    }
}

# Streaming with backpressure
stream = graph.stream_request(:large_query, to: "data_source", {
    buffer_size: 100,      # Max items to buffer
    backpressure: :pause   # Pause producer when buffer full
})

# Collect streamed results with transformation
results = stream
    .filter(item => item.score > 0.5)
    .take(50)
    .collect()
```

### 8. Speculative Execution

```graphoid
# Start operations speculatively before dependencies resolve
# Useful for reducing latency in pipelines

# Launch speculative work
spec_layer2 = graph.send_speculative(:prepare, to: "layer_2")
spec_layer3 = graph.send_speculative(:prepare, to: "layer_3")

# Do the actual work
result1 = await graph.request(:compute, to: "layer_1")

# Resolve speculation with actual input
if result1.success {
    result2 = await spec_layer2.resolve({ input: result1.output })
    result3 = await spec_layer3.resolve({ input: result2.output })
} else {
    # Cancel speculative work if not needed
    spec_layer2.cancel()
    spec_layer3.cancel()
}

# Automatic speculation for known pipelines
graph.configure_pipeline("inference", {
    stages: ["embed", "retrieve", "attend", "generate"],
    speculative: true,     # Auto-speculate next stages
    speculation_depth: 2   # How far ahead to speculate
})

result = await graph.execute_pipeline("inference", { input: prompt })
```

### 9. Backpressure and Load Shedding

```graphoid
# Configure backpressure handling
graph.configure({
    backpressure: {
        max_pending_requests: 10000,
        overflow_strategy: :drop_oldest,  # :drop_oldest, :reject_new, :queue
        priority_queues: [:critical, :high, :normal, :low]
    },
    load_shedding: {
        enabled: true,
        threshold: 0.9,        # Shed load when 90% capacity
        shed_priority: :low    # Which priority to shed first
    }
})

# Monitor backpressure
graph.on_backpressure(fn(stats) {
    log("Backpressure detected: " + stats.pending_requests + " pending")
})

# Adaptive rate limiting
graph.configure({
    rate_limit: {
        strategy: :adaptive,   # Adjusts based on latency
        target_latency: 100ms,
        min_rate: 100,         # Requests per second
        max_rate: 10000
    }
})
```

---

## Execution Model

### Work Distribution

```
┌─────────────────────────────────────────────────────────────┐
│                    Query Coordinator                         │
│                                                             │
│   1. Receive query/algorithm                                │
│   2. Determine affected partitions (via routing strategy)   │
│   3. Dispatch to each partition                             │
│   4. Coordinate execution                                   │
│   5. Collect and merge results                              │
└─────────────────────────────────────────────────────────────┘
         │              │              │
         ▼              ▼              ▼
┌─────────────┐  ┌─────────────┐  ┌─────────────┐
│ Partition A │  │ Partition B │  │ Partition C │
│             │  │             │  │             │
│ Execute     │  │ Execute     │  │ Execute     │
│ locally     │  │ locally     │  │ locally     │
│             │  │             │  │             │
│ Return      │  │ Return      │  │ Return      │
│ results     │  │ results     │  │ results     │
└─────────────┘  └─────────────┘  └─────────────┘
```

### Algorithm Classes

| Class | Pattern | Example |
|-------|---------|---------|
| **Embarrassingly parallel** | No coordination | Map, filter |
| **Iterative convergence** | Iterate until stable | PageRank, label propagation |
| **Aggregation** | Reduce across partitions | Sum, count, average |
| **Graph traversal** | Coordinate at boundaries | BFS, shortest path |

---

## Implementation Plan

### Day 1-3: Distributed Behavior Framework

```rust
struct DistributedBehavior {
    name: String,
    local_fn: Function,      // Runs on each partition
    aggregate_fn: Function,  // Merges results
    config: BehaviorConfig,
}

impl DistributedBehavior {
    async fn execute(&self, graph: &DistributedGraph) -> Value {
        // 1. Get all partitions
        let partitions = graph.partitions();

        // 2. Execute local_fn on each partition in parallel
        let local_results = futures::join_all(
            partitions.iter().map(|p| self.execute_on_partition(p))
        ).await;

        // 3. Aggregate results
        self.aggregate(local_results)
    }
}
```

### Day 4-6: Map-Reduce Engine

```rust
struct MapReduceJob<M, R> {
    map_fn: M,
    reduce_fn: R,
    partitioner: Box<dyn Partitioner>,
}

impl<M, R> MapReduceJob<M, R> {
    async fn execute(&self, graph: &DistributedGraph) -> HashMap<Value, Value> {
        // Map phase: run map_fn on all nodes
        let mapped = self.map_phase(graph).await;

        // Shuffle phase: group by key
        let shuffled = self.shuffle(mapped);

        // Reduce phase: apply reduce_fn to each group
        self.reduce_phase(shuffled)
    }
}
```

### Day 7-9: Consensus Implementation

```rust
// CRDT implementations
enum CRDT {
    GCounter(HashMap<String, u64>),
    PNCounter { positive: HashMap<String, u64>, negative: HashMap<String, u64> },
    LWWRegister { value: Value, timestamp: u64 },
    ORSet { elements: HashMap<Value, HashSet<String>> },
}

impl CRDT {
    fn merge(&mut self, other: &CRDT);
    fn value(&self) -> Value;
}

// Distributed lock (via etcd or similar)
struct DistributedLock {
    name: String,
    registry: PartitionRegistry,
}

impl DistributedLock {
    async fn acquire(&self) -> Result<LockGuard>;
    async fn try_acquire(&self) -> Option<LockGuard>;
}
```

### Day 10-12: Fault Tolerance

```rust
struct ReplicationManager {
    replication_factor: u32,
    consistency_level: ConsistencyLevel,
}

impl ReplicationManager {
    async fn write(&self, key: &str, value: Value) -> Result<()> {
        match self.consistency_level {
            ConsistencyLevel::Strong => self.write_all(key, value).await,
            ConsistencyLevel::Quorum => self.write_quorum(key, value).await,
            ConsistencyLevel::Eventual => self.write_one(key, value).await,
        }
    }

    async fn handle_failure(&self, partition: &str) -> Result<()> {
        // 1. Detect failure (heartbeat timeout)
        // 2. Promote replica to primary
        // 3. Notify registry
        // 4. Start recovery process
    }
}
```

### Day 13-15: Distributed Query Optimizer

```rust
struct DistributedQueryPlanner {
    graph: DistributedGraph,
    statistics: GraphStatistics,  // Aggregate stats, not per-node
}

impl DistributedQueryPlanner {
    fn plan(&self, query: &Query) -> DistributedQueryPlan {
        // 1. Determine which partitions are affected (via routing)
        // 2. Push predicates to partitions
        // 3. Minimize cross-partition communication
        // 4. Generate execution plan
    }
}

struct DistributedQueryPlan {
    stages: Vec<QueryStage>,
    communication_pattern: CommunicationPattern,
}
```

### Day 16-18: Iterative Algorithm Support

```rust
struct IterativeAlgorithm {
    init_fn: Function,
    step_fn: Function,
    converge_fn: Function,
    max_iterations: usize,
}

impl IterativeAlgorithm {
    async fn execute(&self, graph: &DistributedGraph) -> Result<()> {
        // Initialize
        self.scatter_execute(graph, &self.init_fn).await?;

        for i in 0..self.max_iterations {
            // Execute step
            self.scatter_execute(graph, &self.step_fn).await?;

            // Check convergence
            if self.check_convergence(graph).await? {
                break;
            }
        }

        Ok(())
    }
}
```

### Day 19-21: Streaming Responses

```rust
struct StreamingResponse {
    receiver: mpsc::Receiver<StreamItem>,
    cancel_token: CancellationToken,
}

enum StreamItem {
    Data(Value),
    Error(Error),
    Complete,
}

impl StreamingResponse {
    fn cancel(&self) {
        self.cancel_token.cancel();
    }
}

struct StreamingRequestHandler {
    buffer_size: usize,
    backpressure_strategy: BackpressureStrategy,
}

enum BackpressureStrategy {
    Pause,       // Pause producer when buffer full
    DropOldest,  // Drop oldest items
    DropNewest,  // Drop newest items
}

impl StreamingRequestHandler {
    async fn stream_request(
        &self,
        target: MessageTarget,
        msg: Message,
    ) -> StreamingResponse {
        let (tx, rx) = mpsc::channel(self.buffer_size);
        let cancel_token = CancellationToken::new();

        // Spawn task to handle streaming
        tokio::spawn(self.handle_stream(target, msg, tx, cancel_token.clone()));

        StreamingResponse { receiver: rx, cancel_token }
    }

    async fn handle_stream(
        &self,
        target: MessageTarget,
        msg: Message,
        tx: mpsc::Sender<StreamItem>,
        cancel: CancellationToken,
    ) {
        // Connect to target and stream results
        let mut stream = self.open_stream(target, msg).await;

        loop {
            tokio::select! {
                _ = cancel.cancelled() => {
                    // Client cancelled, cleanup
                    stream.close().await;
                    break;
                }
                item = stream.next() => {
                    match item {
                        Some(Ok(value)) => {
                            if tx.send(StreamItem::Data(value)).await.is_err() {
                                // Receiver dropped, stop streaming
                                break;
                            }
                        }
                        Some(Err(e)) => {
                            let _ = tx.send(StreamItem::Error(e)).await;
                            break;
                        }
                        None => {
                            let _ = tx.send(StreamItem::Complete).await;
                            break;
                        }
                    }
                }
            }
        }
    }
}
```

### Day 22-24: Speculative Execution

```rust
struct SpeculativeExecution {
    id: Uuid,
    target: MessageTarget,
    result: Arc<OnceCell<Result<Value>>>,
    cancel_token: CancellationToken,
}

impl SpeculativeExecution {
    // Resolve speculation with actual input
    async fn resolve(&self, actual_input: Value) -> Result<Value> {
        // If speculation completed, validate against actual input
        if let Some(result) = self.result.get() {
            if self.is_compatible(result, &actual_input) {
                return result.clone();
            }
        }

        // Re-execute with actual input
        self.cancel_token.cancel();
        self.execute_with_input(actual_input).await
    }

    fn cancel(&self) {
        self.cancel_token.cancel();
    }
}

struct SpeculativeExecutor {
    max_speculation_depth: usize,
    speculation_budget: AtomicUsize,
}

impl SpeculativeExecutor {
    fn send_speculative(
        &self,
        target: MessageTarget,
        msg: Message,
    ) -> SpeculativeExecution {
        let spec = SpeculativeExecution {
            id: Uuid::new_v4(),
            target: target.clone(),
            result: Arc::new(OnceCell::new()),
            cancel_token: CancellationToken::new(),
        };

        // Launch speculative execution if within budget
        if self.speculation_budget.fetch_sub(1) > 0 {
            let result = spec.result.clone();
            let cancel = spec.cancel_token.clone();

            tokio::spawn(async move {
                tokio::select! {
                    _ = cancel.cancelled() => {}
                    res = execute(target, msg) => {
                        let _ = result.set(res);
                    }
                }
            });
        }

        spec
    }
}

// Pipeline with automatic speculation
struct SpeculativePipeline {
    stages: Vec<PipelineStage>,
    speculation_depth: usize,
}

impl SpeculativePipeline {
    async fn execute(&self, input: Value) -> Result<Value> {
        let mut current = input;
        let mut speculations = VecDeque::new();

        for (i, stage) in self.stages.iter().enumerate() {
            // Launch speculative execution for future stages
            for j in 1..=self.speculation_depth {
                if i + j < self.stages.len() {
                    let spec = self.speculate_stage(i + j, &current);
                    speculations.push_back(spec);
                }
            }

            // Execute current stage
            current = stage.execute(current).await?;

            // Resolve any speculation for next stage
            if let Some(spec) = speculations.pop_front() {
                current = spec.resolve(current).await?;
            }
        }

        Ok(current)
    }
}
```

### Day 25-27: Integration & Testing

- Multi-node cluster tests
- Failure injection tests
- Streaming and backpressure tests
- Speculative execution validation
- Performance benchmarks
- Documentation

---

## Success Criteria

- [ ] Distributed map/reduce on graphs
- [ ] Distributed queries with predicate pushdown
- [ ] Iterative algorithms (PageRank, label propagation)
- [ ] Consensus primitives (locks, leader election)
- [ ] CRDT support (counters, sets)
- [ ] Fault tolerance (replication, failover)
- [ ] Configurable consistency levels
- [ ] Streaming responses with backpressure
- [ ] Speculative execution for pipelines
- [ ] Load shedding and adaptive rate limiting
- [ ] At least 80 distributed execution tests
- [ ] Streaming and cancellation tests
- [ ] Speculation correctness tests
- [ ] Example: Distributed PageRank
- [ ] Example: Distributed word count
- [ ] Example: Streaming inference pipeline
- [ ] Documentation complete

---

## Example: Distributed PageRank

```graphoid
import "distributed"

fn distributed_pagerank(graph, iterations, damping) {
    n = graph.node_count()
    initial_rank = 1.0 / n

    # Initialize all nodes via broadcast
    graph.broadcast(:init, { rank: initial_rank, damping: damping, n: n })

    # Iterate using graph-native messaging
    for i in range(0, iterations) {
        # Each node sends contribution to neighbors
        graph.broadcast(:send_contributions)

        # Each node updates its rank
        graph.broadcast(:update_rank)
    }
}

# Node behavior handles messages
graph PageRankNode {
    rank = 0.0
    incoming = 0.0
    damping = 0.85
    n = 1

    fn on_message(msg) {
        match msg {
            [:init, params] => {
                rank = params["rank"]
                damping = params["damping"]
                n = params["n"]
            }
            :send_contributions => {
                out_degree = self.out_degree()
                if out_degree > 0 {
                    contribution = rank / out_degree
                    # Send along ALL outgoing edges (may cross partitions - transparent!)
                    self.graph.send([:contribution, contribution], from: self.id, via: "*")
                }
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

# Run on distributed graph - same code!
social = distributed.load("social-network")
distributed_pagerank(social, 10, 0.85)

# Get top ranked nodes using request-response
top = await social.request(:get_rank, where: n => true)
    .sort_by(r => r["rank"], :desc)
    .take(10)

print(top)
```

---

## Example: Distributed Word Count

```graphoid
import "distributed"

# Documents stored as graph nodes
documents = distributed.load("document-graph")

# Method 1: Map-reduce (gather-scatter pattern)
word_counts = distributed.map_reduce(
    documents,
    map: fn(doc_node) {
        content = doc_node.get("content")
        words = content.lower().split(/\s+/)
        return words.map(word => [word, 1])
    },
    reduce: fn(word, counts) {
        return [word, counts.sum()]
    }
)

# Method 2: Graph-native messaging (actor pattern)
# Each document node processes itself and reports
documents.broadcast(:count_words)
word_results = await documents.request(:get_word_counts, where: n => true)
word_counts = merge_counts(word_results)

# Find most common words
top_words = word_counts
    .to_list()
    .sort_by(pair => pair[1], :desc)
    .take(100)

print(top_words)
```

---

## Open Questions

1. **Scheduling** - How to balance load across partitions?
2. **Stragglers** - How to handle slow partitions?
3. **Transactions** - Distributed transactions vs eventual consistency?
4. **Debugging** - How to debug distributed executions?

---

## Related Documents

- [PHASE_23_DISTRIBUTED_PRIMITIVES.md](PHASE_23_DISTRIBUTED_PRIMITIVES.md) - Required primitives
- [PHASE_19_CONCURRENCY.md](PHASE_19_CONCURRENCY.md) - Virtual actor foundation
- [CONCURRENCY_MODEL_RATIONALE.md](CONCURRENCY_MODEL_RATIONALE.md) - Design rationale

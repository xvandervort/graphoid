# Phase 25: Vector Search & Similarity Graphs

**Duration**: 14-18 days
**Priority**: High
**Dependencies**: Phase 19 (Concurrency), Phase 23 (Distributed Primitives)
**Status**: Planned

---

## Goal

Implement HNSW (Hierarchical Navigable Small World) as a **first-class graph type** in Graphoid, enabling efficient approximate nearest neighbor search for embeddings, recommendations, and similarity-based retrieval.

**Key principle**: HNSW is not a module or library - it IS a graph. The hierarchical layers are subgraphs, the connections are edges, and vector nodes are addressable actors.

---

## Core Concept: HNSW as a Graph Type

```
┌─────────────────────────────────────────────────────────────┐
│                    HNSW Graph Structure                      │
│                                                             │
│   "HNSW is a graph. Layers are subgraphs. Search is        │
│    traversal. This is native to Graphoid."                  │
│                                                             │
│   Layer 2 (sparse):    [A] ─────────────────── [D]          │
│                         │                       │           │
│   Layer 1 (medium):    [A] ── [B] ── [C] ───── [D] ── [E]   │
│                         │     │      │         │      │     │
│   Layer 0 (dense):     [A]─[B]─[C]─[D]─[E]─[F]─[G]─[H]─[I]  │
│                                                             │
│   Each [node] is a virtual actor with an embedding          │
│   Each ─ is an edge (HNSW connection)                       │
│   Search = greedy graph traversal from top to bottom        │
└─────────────────────────────────────────────────────────────┘
```

---

## Five-Layer Architecture Integration

HNSW graphs use Graphoid's standard five-layer architecture (see `ARCHITECTURE_DESIGN.md`). This ensures clean separation between user data and internal HNSW structure.

### Layer Separation

```
┌─────────────────────────────────────────────────────────────────┐
│  HNSW Graph (GraphValue)                                        │
│                                                                 │
│  DataLayer:                                                     │
│    • Vector nodes: doc_1, doc_2, doc_3, ...                     │
│    • HNSW connection edges between nodes                        │
│    • Node values include embedding + user metadata              │
│                                                                 │
│  MetadataLayer:                                                 │
│    • layer_membership: {doc_1: [0,1,2], doc_2: [0], ...}        │
│    • entry_point: "doc_1"                                       │
│    • hnsw_config: {m: 16, ef_construction: 200, ...}            │
│    • layer_stats: {0: {node_count: 1000}, 1: {node_count: 50}}  │
│                                                                 │
│  BehaviorLayer:                                                 │
│    • Embedding validation (dimension check)                     │
│    • Distance metric function                                   │
│                                                                 │
│  ControlLayer:                                                  │
│    • Max connections per layer (M parameter)                    │
│    • Dimension constraints                                      │
└─────────────────────────────────────────────────────────────────┘
```

### What This Means for API

| Method | Operates On | Returns |
|--------|-------------|---------|
| `embeddings.nodes()` | DataLayer | Vector nodes only (user data) |
| `embeddings.edges()` | DataLayer | HNSW connection edges only |
| `embeddings.node_count()` | DataLayer | Count of vector nodes |
| `embeddings.layer(n)` | MetadataLayer query | Filtered view of DataLayer nodes in layer n |
| `embeddings.search()` | Both layers | Traverses using MetadataLayer structure |

**Key principle**: `.nodes()` and `.edges()` always return user data from the DataLayer. Internal HNSW structure (layer membership, entry points) lives in MetadataLayer and is accessed via specialized methods.

---

## Core Features

### 1. HNSW Graph Type

```graphoid
# Create an HNSW graph - it's a graph!
embeddings = hnsw {
    dimensions: 768,          # Vector dimensions
    metric: :cosine,          # :cosine, :euclidean, :dot_product
    m: 16,                    # Max connections per node per layer
    ef_construction: 200,     # Search width during construction
    ef_search: 50             # Search width during queries
}

# It's a graph, so standard operations work
embeddings.node_count()
embeddings.edge_count()
embeddings.nodes()

# Add nodes with embeddings
embeddings.add_node("doc_1", {
    embedding: [0.1, 0.2, ...],  # 768-dimensional vector
    title: "Introduction to Graphs",
    category: "tutorial"
})

# Edges are created automatically by HNSW algorithm
# But you can inspect them
embeddings.edges()  # Returns HNSW connections
embeddings.neighbors("doc_1")  # HNSW neighbors at layer 0
```

### 2. Similarity Search (Graph Traversal)

```graphoid
# Search is specialized graph traversal
results = embeddings.search(query_vector, k: 10)
# Returns: [
#   { id: "doc_42", distance: 0.05, node: <node_ref> },
#   { id: "doc_17", distance: 0.08, node: <node_ref> },
#   ...
# ]

# Search with filters (predicate pushed into traversal)
results = embeddings.search(query_vector, k: 10,
    where: n => n.get("category") == "tutorial"
)

# Search with metadata retrieval
results = embeddings.search(query_vector, k: 10,
    include: ["title", "category"]
)

# Batch search (multiple queries)
results = embeddings.search_batch([query1, query2, query3], k: 10)
```

### 3. Layer Access (Subgraphs)

```graphoid
# Each HNSW layer is a subgraph
layer_0 = embeddings.layer(0)  # Dense bottom layer
layer_1 = embeddings.layer(1)  # Sparser
layer_2 = embeddings.layer(2)  # Sparsest (entry points)

# Layer statistics
layer_0.node_count()  # All nodes are in layer 0
layer_2.node_count()  # Only entry points

# Layer is a real graph - can traverse it
for node in layer_2.nodes() {
    print("Entry point: " + node.id)
    print("Connections: " + node.neighbors().count())
}

# Get node's layer membership
node = embeddings.get_node("doc_1")
node.get("_layers")  # [0, 1] - which layers this node appears in
```

### 4. Graph-Native Messaging on HNSW

```graphoid
# HNSW nodes are virtual actors - messaging works!

# Send to specific node
embeddings.send(:update_embedding, to: "doc_1", {
    new_embedding: updated_vector
})

# Broadcast to all nodes in a layer
embeddings.send(:recompute_connections,
    where: n => n.in_layer(0))

# Send along HNSW edges (neighbors)
embeddings.send(:propagate_update, from: "doc_1", via: "HNSW_EDGE")

# Request-response for similarity
similar_docs = await embeddings.request(:get_similar,
    to: "doc_1",
    { k: 5 }
)
```

### 5. Distance Metrics

```graphoid
# Cosine similarity (default for text embeddings)
text_index = hnsw {
    dimensions: 768,
    metric: :cosine
}

# Euclidean distance (for spatial data)
spatial_index = hnsw {
    dimensions: 3,
    metric: :euclidean
}

# Dot product (for maximum inner product search)
mips_index = hnsw {
    dimensions: 768,
    metric: :dot_product
}

# Custom distance function
custom_index = hnsw {
    dimensions: 768,
    metric: fn(a, b) {
        # Weighted cosine
        weighted_a = a.zip(weights).map((v, w) => v * w)
        weighted_b = b.zip(weights).map((v, w) => v * w)
        return cosine_distance(weighted_a, weighted_b)
    }
}
```

### 6. Incremental Updates

```graphoid
# Insert new vector (builds connections automatically)
embeddings.add_node("new_doc", { embedding: new_vector })

# Update existing vector (rebuilds connections)
embeddings.update_node("doc_1", { embedding: updated_vector })

# Remove node (repairs connections)
embeddings.remove_node("old_doc")

# Batch operations
embeddings.add_nodes([
    { id: "doc_100", embedding: vec100 },
    { id: "doc_101", embedding: vec101 },
    ...
])
```

---

## Distributed HNSW

### Partitioning Strategies

```graphoid
import "distributed"

# Strategy 1: Layer-based partitioning
# Top layers on coordinator, bottom layer distributed
distributed_embeddings = distributed.hnsw({
    name: "billion-embeddings",
    dimensions: 768,
    partitioning: {
        strategy: :layer_based,
        coordinator_layers: [1, 2, 3],  # Top layers centralized
        distributed_layer: 0,            # Bottom layer distributed
        partitions: 100
    },
    replication_factor: 3
})

# Strategy 2: Region-based partitioning
# Cluster vectors, each partition owns a region
distributed_embeddings = distributed.hnsw({
    name: "billion-embeddings",
    dimensions: 768,
    partitioning: {
        strategy: :region_based,
        regions: 100,                    # k-means clusters
        overlap: 0.1                     # 10% overlap for boundary queries
    },
    replication_factor: 3
})
```

### Distributed Search

```graphoid
# Search across distributed HNSW
results = await distributed_embeddings.search(query_vector, k: 10, {
    timeout: 100ms,
    quorum: 0.8,           # Accept if 80% of relevant partitions respond
    fallback: :partial_results
})

# Layer-based search flow:
# 1. Query coordinator for top layers → find entry regions
# 2. Fan out to relevant partitions for layer 0 search
# 3. Merge results, return top k

# Region-based search flow:
# 1. Identify candidate regions (may use separate routing index)
# 2. Query candidate partitions in parallel
# 3. Merge results, return top k
```

### Building Distributed Index

```graphoid
# Build from existing graph
source_graph = distributed.load("document-embeddings")

# Create HNSW index from source
index = distributed.build_hnsw(source_graph, {
    embedding_field: "embedding",
    dimensions: 768,
    metric: :cosine,
    partitioning: { strategy: :region_based, regions: 100 }
})

# Or stream build
index = distributed.hnsw({ ... })
for doc in document_stream {
    index.add_node(doc.id, { embedding: doc.embedding })
}
index.finalize()  # Optimize structure after bulk load
```

---

## Integration with Embedding Graphs

### Dual Structure: Data Graph + Index Graph

```graphoid
# Main data graph with embeddings as node properties
documents = distributed.graph({
    name: "documents",
    partitioning: { strategy: :hash, partitions: 100 }
})

documents.add_node("doc_1", {
    title: "Introduction to Graphs",
    content: "...",
    embedding: compute_embedding(content)
})

# HNSW index references the data graph
doc_index = hnsw {
    dimensions: 768,
    source: documents,           # Link to data graph
    embedding_field: "embedding"
}

# Search returns references to data graph nodes
results = doc_index.search(query_vector, k: 10)
for result in results {
    doc = result.node  # Reference to documents graph node
    print(doc.get("title"))
}
```

### Synchronized Updates

```graphoid
# When data graph updates, index updates automatically
documents.add_node("new_doc", {
    title: "New Document",
    embedding: new_embedding
})
# doc_index automatically adds corresponding node

# Or manual sync
doc_index.sync()

# Configure sync behavior
doc_index = hnsw {
    source: documents,
    sync: :automatic,     # :automatic, :manual, :batch
    sync_interval: 1000ms # For batch mode
}
```

---

## Node Behavior for HNSW

```graphoid
# Define custom behavior for HNSW nodes
graph HnswVectorNode {
    embedding = []
    layers = [0]
    connections = {}  # layer -> neighbor_ids

    fn on_message(msg) {
        match msg {
            [:search_step, query_vec, state] => {
                # Compute distance to query
                dist = distance(embedding, query_vec)

                # Find closer neighbors
                closer = connections[state.layer]
                    .filter(n => n.distance(query_vec) < dist)

                if closer.empty() {
                    # Local minimum - descend or return
                    if state.layer == 0 {
                        self.graph.send([:search_result, self.id, dist],
                            to: state.coordinator)
                    } else {
                        # Descend to next layer
                        self.graph.send([:search_step, query_vec,
                            state.with({ layer: state.layer - 1 })],
                            to: self.id)
                    }
                } else {
                    # Continue to closer neighbor
                    self.graph.send([:search_step, query_vec, state],
                        to: closer.first())
                }
            }

            [:update_embedding, new_vec] => {
                embedding = new_vec
                # Trigger reconnection
                self.rebuild_connections()
            }
        }
    }

    fn rebuild_connections() {
        for layer in layers {
            neighbors = self.find_neighbors(layer)
            connections[layer] = neighbors
        }
    }
}
```

---

## Implementation Plan

### Day 1-3: Core HNSW Graph Type

```rust
// HNSW as a graph type
struct HnswGraph {
    config: HnswConfig,
    layers: Vec<LayerGraph>,
    entry_point: Option<NodeId>,
    rng: StdRng,  // For layer selection
}

struct HnswConfig {
    dimensions: usize,
    m: usize,              // Max connections per layer
    m_max_0: usize,        // Max connections at layer 0
    ef_construction: usize,
    ef_search: usize,
    distance_metric: DistanceMetric,
    ml: f64,               // Level multiplier (1/ln(M))
}

struct LayerGraph {
    level: usize,
    nodes: HashSet<NodeId>,
    edges: HashMap<NodeId, Vec<(NodeId, f32)>>,  // neighbor, distance
}

impl HnswGraph {
    fn new(config: HnswConfig) -> Self {
        Self {
            config,
            layers: vec![LayerGraph::new(0)],
            entry_point: None,
            rng: StdRng::from_entropy(),
        }
    }

    fn select_layer(&mut self) -> usize {
        // Exponential decay: -ln(uniform) * ml
        let r: f64 = self.rng.gen();
        (-r.ln() * self.config.ml).floor() as usize
    }
}
```

### Day 4-6: Insert Algorithm

```rust
impl HnswGraph {
    fn insert(&mut self, id: NodeId, vector: Vec<f32>) -> Result<()> {
        let target_layer = self.select_layer();

        // Ensure enough layers exist
        while self.layers.len() <= target_layer {
            self.layers.push(LayerGraph::new(self.layers.len()));
        }

        // Find entry point for insertion
        let mut current = self.entry_point.clone();
        let mut current_layer = self.layers.len() - 1;

        // Traverse from top to target_layer + 1
        while current_layer > target_layer {
            current = self.search_layer_greedy(
                current.as_ref(),
                &vector,
                current_layer,
                1
            ).first().cloned();
            current_layer -= 1;
        }

        // Insert at each layer from target_layer down to 0
        for layer in (0..=target_layer).rev() {
            let neighbors = self.search_layer(
                current.as_ref(),
                &vector,
                layer,
                self.config.ef_construction
            );

            // Select M best neighbors
            let selected = self.select_neighbors(&vector, neighbors, layer);

            // Add bidirectional edges
            self.add_connections(&id, &selected, layer);

            current = Some(id.clone());
        }

        // Update entry point if this node is in highest layer
        if target_layer >= self.layers.len() - 1 {
            self.entry_point = Some(id);
        }

        Ok(())
    }
}
```

### Day 7-9: Search Algorithm

```rust
impl HnswGraph {
    fn search(&self, query: &[f32], k: usize, ef: usize) -> Vec<SearchResult> {
        let mut current = match &self.entry_point {
            Some(ep) => ep.clone(),
            None => return vec![],
        };

        // Traverse from top layer to layer 1
        for layer in (1..self.layers.len()).rev() {
            current = self.search_layer_greedy(
                Some(&current),
                query,
                layer,
                1
            ).into_iter().next().unwrap_or(current);
        }

        // Search layer 0 with ef candidates
        let candidates = self.search_layer(
            Some(&current),
            query,
            0,
            ef.max(k)
        );

        // Return top k
        candidates.into_iter()
            .take(k)
            .map(|(id, dist)| SearchResult { id, distance: dist })
            .collect()
    }

    fn search_layer(
        &self,
        entry: Option<&NodeId>,
        query: &[f32],
        layer: usize,
        ef: usize,
    ) -> Vec<(NodeId, f32)> {
        let mut visited = HashSet::new();
        let mut candidates = BinaryHeap::new();  // Min-heap by distance
        let mut results = BinaryHeap::new();     // Max-heap for top-k

        if let Some(ep) = entry {
            let dist = self.distance(query, ep);
            candidates.push(Reverse((OrderedFloat(dist), ep.clone())));
            results.push((OrderedFloat(dist), ep.clone()));
            visited.insert(ep.clone());
        }

        while let Some(Reverse((dist, node))) = candidates.pop() {
            let worst_result = results.peek().map(|(d, _)| d.0).unwrap_or(f32::MAX);

            if dist.0 > worst_result {
                break;  // All remaining candidates are farther than worst result
            }

            // Explore neighbors
            for (neighbor, _) in self.get_neighbors(&node, layer) {
                if visited.insert(neighbor.clone()) {
                    let neighbor_dist = self.distance(query, &neighbor);

                    if neighbor_dist < worst_result || results.len() < ef {
                        candidates.push(Reverse((OrderedFloat(neighbor_dist), neighbor.clone())));
                        results.push((OrderedFloat(neighbor_dist), neighbor.clone()));

                        if results.len() > ef {
                            results.pop();
                        }
                    }
                }
            }
        }

        results.into_sorted_vec()
            .into_iter()
            .map(|(d, id)| (id, d.0))
            .collect()
    }
}
```

### Day 10-12: Distance Metrics & SIMD

```rust
enum DistanceMetric {
    Cosine,
    Euclidean,
    DotProduct,
    Custom(Arc<dyn Fn(&[f32], &[f32]) -> f32 + Send + Sync>),
}

impl DistanceMetric {
    fn compute(&self, a: &[f32], b: &[f32]) -> f32 {
        match self {
            Self::Cosine => cosine_distance_simd(a, b),
            Self::Euclidean => euclidean_distance_simd(a, b),
            Self::DotProduct => -dot_product_simd(a, b),  // Negative for min-heap
            Self::Custom(f) => f(a, b),
        }
    }
}

// SIMD-optimized cosine distance
#[cfg(target_arch = "x86_64")]
fn cosine_distance_simd(a: &[f32], b: &[f32]) -> f32 {
    use std::arch::x86_64::*;

    unsafe {
        let mut dot = _mm256_setzero_ps();
        let mut norm_a = _mm256_setzero_ps();
        let mut norm_b = _mm256_setzero_ps();

        for i in (0..a.len()).step_by(8) {
            let va = _mm256_loadu_ps(a.as_ptr().add(i));
            let vb = _mm256_loadu_ps(b.as_ptr().add(i));

            dot = _mm256_fmadd_ps(va, vb, dot);
            norm_a = _mm256_fmadd_ps(va, va, norm_a);
            norm_b = _mm256_fmadd_ps(vb, vb, norm_b);
        }

        let dot_sum = hsum_avx(dot);
        let norm_a_sum = hsum_avx(norm_a).sqrt();
        let norm_b_sum = hsum_avx(norm_b).sqrt();

        1.0 - (dot_sum / (norm_a_sum * norm_b_sum))
    }
}
```

### Day 13-14: Executor Integration

```rust
// Register hnsw as a graph type in executor
impl Executor {
    fn create_hnsw(&mut self, config: Value) -> Result<Value> {
        let dimensions = config.get("dimensions")?.as_int()?;
        let metric = match config.get("metric")?.as_symbol()? {
            "cosine" => DistanceMetric::Cosine,
            "euclidean" => DistanceMetric::Euclidean,
            "dot_product" => DistanceMetric::DotProduct,
            _ => return Err("Unknown metric"),
        };

        let hnsw = HnswGraph::new(HnswConfig {
            dimensions,
            metric,
            m: config.get("m").unwrap_or(16),
            ef_construction: config.get("ef_construction").unwrap_or(200),
            ef_search: config.get("ef_search").unwrap_or(50),
            ..Default::default()
        });

        Ok(Value::Graph(Arc::new(RwLock::new(hnsw))))
    }
}

// Methods on HNSW graphs
impl HnswGraph {
    fn call_method(&self, name: &str, args: &[Value]) -> Result<Value> {
        match name {
            "search" => {
                let query = args[0].as_float_vec()?;
                let k = args[1].as_int()?;
                let results = self.search(&query, k, self.config.ef_search);
                Ok(Value::List(results.into_iter().map(|r| r.to_value()).collect()))
            }
            "add_node" => {
                let id = args[0].as_string()?;
                let props = args[1].as_map()?;
                let embedding = props.get("embedding")?.as_float_vec()?;
                self.insert(id, embedding)?;
                Ok(Value::None)
            }
            "layer" => {
                let level = args[0].as_int()?;
                Ok(Value::Graph(self.get_layer_subgraph(level)))
            }
            // Standard graph methods also work
            "nodes" => self.nodes(),
            "edges" => self.edges(),
            "node_count" => Ok(Value::Number(self.node_count() as f64)),
            _ => Err(format!("Unknown method: {}", name)),
        }
    }
}
```

### Day 15-18: Distributed HNSW

```rust
struct DistributedHnsw {
    config: DistributedHnswConfig,
    coordinator: Option<HnswCoordinator>,
    partitions: Vec<PartitionId>,
    router: MessageRouter,
}

struct HnswCoordinator {
    top_layers: HnswGraph,  // Layers 1+ centralized
    region_map: HashMap<RegionId, PartitionId>,
}

impl DistributedHnsw {
    async fn search(
        &self,
        query: &[f32],
        k: usize,
        options: RequestOptions,
    ) -> Result<Vec<SearchResult>> {
        // 1. Search top layers on coordinator
        let entry_regions = self.coordinator
            .as_ref()
            .map(|c| c.find_entry_regions(query, options.ef))
            .unwrap_or_else(|| self.all_partitions());

        // 2. Fan out to partitions
        let partition_futures = entry_regions.iter().map(|region| {
            let partition = self.region_to_partition(region);
            self.router.request_with_options(
                partition,
                Message::HnswSearch { query: query.to_vec(), k, ef: options.ef },
                options.clone(),
            )
        });

        // 3. Collect results with quorum
        let results = collect_with_quorum(
            partition_futures,
            options.quorum,
            options.timeout,
        ).await?;

        // 4. Merge and return top k
        Ok(merge_search_results(results, k))
    }
}
```

---

## Success Criteria

- [ ] HNSW as first-class graph type (`hnsw { ... }`)
- [ ] Standard graph operations work (nodes, edges, node_count)
- [ ] Search with configurable k and ef
- [ ] Distance metrics (cosine, euclidean, dot product)
- [ ] Layer access as subgraphs
- [ ] Graph-native messaging on HNSW nodes
- [ ] Incremental insert/update/delete
- [ ] SIMD-optimized distance calculations
- [ ] Distributed HNSW (layer-based partitioning)
- [ ] Distributed HNSW (region-based partitioning)
- [ ] Integration with data graphs (source linking)
- [ ] At least 50 vector search tests
- [ ] Benchmark: 1M vectors, <10ms search latency
- [ ] Example: Document similarity search
- [ ] Example: Recommendation system
- [ ] Documentation complete

---

## Example: Document Similarity Search

```graphoid
# Create HNSW index for documents
doc_index = hnsw {
    dimensions: 768,
    metric: :cosine,
    m: 16,
    ef_construction: 200
}

# Add documents with embeddings
fn index_document(doc) {
    embedding = compute_embedding(doc.content)
    doc_index.add_node(doc.id, {
        embedding: embedding,
        title: doc.title,
        category: doc.category
    })
}

# Index corpus
for doc in corpus {
    index_document(doc)
}

# Search for similar documents
fn find_similar(query_text, k) {
    query_embedding = compute_embedding(query_text)
    results = doc_index.search(query_embedding, k: k)

    return results.map(r => {
        node = r.node
        {
            id: r.id,
            title: node.get("title"),
            similarity: 1 - r.distance,
            category: node.get("category")
        }
    })
}

# Usage
similar = find_similar("How do graphs work?", 10)
for doc in similar {
    print(doc.title + " (" + doc.similarity + ")")
}
```

---

## Example: Distributed RAG Pipeline

```graphoid
import "distributed"

# Distributed embedding index
embeddings = distributed.hnsw({
    name: "knowledge-base",
    dimensions: 768,
    metric: :cosine,
    partitioning: {
        strategy: :region_based,
        regions: 100
    },
    replication_factor: 3
})

# RAG retrieval with fault tolerance
async fn retrieve_context(query, k) {
    query_embedding = compute_embedding(query)

    results = await embeddings.search(query_embedding, k: k, {
        timeout: 100ms,
        quorum: 0.8,
        fallback: :partial_results
    })

    return results.map(r => r.node.get("content"))
}

# Use in generation pipeline
async fn generate_answer(question) {
    # Retrieve relevant context
    context = await retrieve_context(question, k: 10)

    # Generate answer using context
    prompt = build_prompt(question, context)
    answer = await model.generate(prompt)

    return answer
}
```

---

## Open Questions

1. **Persistence** - How to persist HNSW graphs to disk efficiently?
2. **Quantization** - Support for product quantization to reduce memory?
3. **Hybrid Search** - Combine vector similarity with keyword/attribute filters?
4. **Dynamic Updates** - Optimize for high write throughput scenarios?

---

## Future Optimizations (Post-Phase 25)

These optimizations emerged from stress-testing the architecture against a distributed LLM inference scenario:

| Optimization | Description | Benefit |
|--------------|-------------|---------|
| **Prefix Caching** | Cache common prompt prefixes (e.g., system prompts) across users | Reduce redundant embedding lookups |
| **KV-Cache as Graph** | Store transformer attention KV cache as graph nodes for reuse across similar queries | Massive speedup for follow-up queries |
| **Adaptive Speculation** | Dynamically adjust speculation depth based on observed latency and failure rates | Balance latency vs resource usage |
| **Hot Path Replication** | Auto-detect and replicate frequently accessed embeddings closer to query entry points | Reduce cross-partition hops for popular content |

These are not required for Phase 25 but represent natural evolution once the core infrastructure is proven.

---

## Related Documents

- [PHASE_19_CONCURRENCY.md](PHASE_19_CONCURRENCY.md) - Virtual actor foundation
- [PHASE_23_DISTRIBUTED_PRIMITIVES.md](PHASE_23_DISTRIBUTED_PRIMITIVES.md) - Distributed infrastructure
- [PHASE_24_DISTRIBUTED_EXECUTION.md](PHASE_24_DISTRIBUTED_EXECUTION.md) - Streaming and fault tolerance
- [CONCURRENCY_MODEL_RATIONALE.md](CONCURRENCY_MODEL_RATIONALE.md) - Design philosophy

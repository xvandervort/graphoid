# Associative Memory Graph Simulation (Design Draft)

Status: Draft
Owner: sims

## Purpose
Design an associative memory simulation using Graphoid’s future language features (graphs as first-class, behaviors/rules, RSpec-style testing). The goal is to model memory retrieval and creativity-like recombination by propagating activation over a graph of concepts with adaptive, learned edge weights.

## Core Ideas
- Nodes represent concepts/objects; each has structured data (attributes, embeddings, tags) and behaviors.
- Edges represent associative strength (bidirectional by default; potentially asymmetric later). Edge weights adapt via local plasticity rules.
- Retrieval is activation-based: cues inject activation to one or more nodes; activation diffuses over edges; nodes compete/cooperate; a stable pattern emerges (the “recalled” memory).
- Learning updates edge weights from co-activation and similarity/difference signals.
- Creativity emerges from controlled diffusion, noise, and rule-based bridging across distant clusters.

## Assemblies (Engrams), not global symbols

While we sometimes speak in “concept” terms for brevity, the memory substrate should be modeled as distributed cell assemblies (engrams), not single global symbol nodes.

- **Assembly as subgraph**: A concept like `wing` or `lift` is realized as a recurrently co-activating subgraph of many microfeature nodes, not a single node. An optional “assembly handle” node can exist as an index for tooling, but content lives in the distributed pattern.
- **Overlap and reuse**: Different experiences reuse partially overlapping microfeatures; assemblies overlap rather than reference a single canonical symbol. Pattern separation vs. completion are emergent from overlap, inhibition, and plasticity.
- **Schemas**: Frequently co-activated motifs form higher-order assemblies (“schemas”) that can gate or coordinate lower-level assemblies, again as distributed patterns rather than unique symbols.
- **Provenance and context**: Context nodes modulate which portions of an assembly participate during recall (e.g., episodic vs. semantic retrieval modes).

## Graph Model
- Node fields:
  - `id: string`
  - `features: map<string, any>` (symbolic + numeric)
  - `embedding: list<num>` (optional)
  - `tags: list<string>`
  - `activation: num` (transient, 0..1)
  - `fatigue: num` (transient; inhibits repeated firing)
  - `threshold: num` (e.g., 0.5)
- Edge fields:
  - `weight: num` (−1..1; negative edges model dissimilarity/inhibition)
  - `age: num` (for decay)
  - `type: string` ("associative", "causal", "semantic", etc.)

## Key Processes
1. Initialization
   - Build graph from a corpus (symbols + vectors). Initialize weights using similarity metrics (cosine on embeddings, Jaccard on tags).
2. Cueing
   - Given one or more cue nodes or a query vector, set initial `activation` and optionally inject input currents for T steps.
3. Propagation (per tick)
   - For each node, compute input: `sum_over_neighbors(activation_neighbor * effective_weight)`.
   - Update node activation with nonlinearity and fatigue: `a_t+1 = clip( (1−λ)a_t + σ(input − threshold) − fatigue, 0, 1 )`.
   - Compete: soft WTA via lateral inhibition or k-sparse top-k masking.
4. Learning (Hebbian + anti-Hebbian)
   - `Δw_ij = η * (a_i * a_j − β * dissimilarity(i,j)) − μ * |w_ij|`.
   - Cap to [−wmax, wmax]; optionally asymmetric updates.
5. Consolidation/Decay
   - Weight decay and edge pruning below magnitude ε; strengthen frequently used paths.
6. Readout
   - The recalled set is nodes with activation ≥ τ; ranked by activation and recency; optionally paths that carried most current (explainability).

## Similarity and Difference Signals
- Similarity(i,j):
  - Cosine(embedding_i, embedding_j) if embeddings exist
  - Jaccard(tags_i, tags_j)
  - Weighted combination via behavior rule
- Difference(i,j): 1 − Similarity(i,j) or domain-specific predicate mismatches (e.g., color vs shape conflicts)

## Behaviors and Rules (Graphoid flavor)
- Node behaviors:
  - `compute_similarity(neighbor)` returns num
  - `update_activation(inputs)` handles nonlinearity and fatigue
  - `consolidate()` updates thresholds/fatigue
- Graph rules:
  - `no_cycles` optional (we usually allow cycles)
  - `max_degree(n)` soft limit via pruning
  - `weight_bounds` enforce [−wmax, wmax]
  - `sparsify(k)` keep top-k edges per node periodically

## Control Parameters
- `λ` leak rate; `η` learning rate; `β` dissimilarity penalty; `μ` L1 decay; `wmax` max |weight|; `τ` readout threshold; tick count; noise amplitude; top-k.

## Algorithms
### Build
1. Add nodes with features and embeddings.
2. For each node, connect to top-k most similar nodes with initial weight = similarity.
3. Optionally add negative edges to most dissimilar nodes (inhibition).

### Recall
1. Reset transient states (`activation`, `fatigue`).
2. Inject cue(s) for `T_inject` ticks.
3. For t in 1..T:
   - For each node: input = Σ a_j * w_ij
   - Update activations with leak, nonlinearity (e.g., `σ(x)=tanh(x)` mapped to [0,1])
   - Apply lateral inhibition or k-sparsity
   - Apply noise ε
4. Readout nodes with a≥τ; capture top paths via edge currents a_j*w_ij.

### Learn
- After each tick or episode: update weights with Hebbian rule; prune edges below ε; renormalize degree-wise if needed.

## Creativity Mode
- Increase diffusion radius (more ticks, less sparsity, more noise).
- Allow controlled bridge formation: when two distant clusters co-activate, create low-weight exploratory edges; keep if used again.

## API Sketch (Future Graphoid)
```graphoid
import "graphs"
import "statistics" # for cosine

my_graph = graph { type: :associative }

my_graph.add_rule("weight_bounds", -1.0, 1.0)
my_graph.add_rule("sparsify", 8)

func build(nodes) {
  for n in nodes {
    my_graph.add_node(n.id, { features: n.features, embedding: n.embedding, tags: n.tags, activation: 0.0, fatigue: 0.0, threshold: 0.5 })
  }
  for n in my_graph.nodes() {
    sims = []
    for m in my_graph.nodes() {
      if n == m { continue }
      s = similarity(n, m)
      sims.append({ node: m, score: s })
    }
    sims.sort_by("score", :desc)
    for entry in sims.take(8) {
      my_graph.add_edge(n, entry.node, { weight: entry.score, type: "associative" })
    }
  }
}

func cue(ids, strength, ticks) {
  for id in ids { my_graph.node(id).activation = strength }
  for t in range(0, ticks) { tick() }
}

func tick() {
  inputs = {}
  for n in my_graph.nodes() {
    total = 0.0
    for e in n.edges() {
      total += e.other(n).activation * e.weight
    }
    inputs[n.id] = total
  }
  # update
  for n in my_graph.nodes() {
    a = n.activation
    x = inputs[n.id] - n.threshold - n.fatigue
    a_next = clip((1 - lambda) * a + sigmoid(x), 0.0, 1.0)
    n.activation = a_next
  }
  lateral_inhibit_top_k(20)
  learn()
}

func learn() {
  for e in my_graph.edges() {
    i = e.a; j = e.b
    dw = eta * (i.activation * j.activation - beta * difference(i, j)) - mu * abs(e.weight)
    e.weight = clamp(e.weight + dw, -wmax, wmax)
  }
}
```

## Data Ingestion
- Start with small hand-authored concept sets (e.g., animals, tools, colors).
- Optionally embed text descriptions via external model; store vectors in `embedding`.

## Evaluation Scenarios
- Pattern completion: cue subset of attributes; expect full concept activation.
- Disambiguation: cue ambiguous features; expect context to resolve.
- Creative association: cue two distant concepts; expect bridging concepts.
- Robustness: noisy cues still recall correct items.

## Instrumentation & Explainability
- Track per-tick activation snapshots.
- Edge current heatmaps; path extraction via maximum flow per episode.
- Stability metrics: entropy of activation distribution over ticks.

## Testing Plan (RSpec-style)
- Unit: similarity functions, activation update, learning rule bounds.
- Integration: build→cue→recall pipeline; pruning and sparsification.
- Property: symmetry of cosine similarity; bounded activations and weights.
- Regression: scenarios above with fixed seeds.

## Risks & Mitigations
- Runaway activation → leaks, inhibition, caps.
- Graph bloat → sparsify, degree caps, pruning.
- Confirmation bias (only strengthen) → anti-Hebbian term and negative edges.

## Extensions
- Asymmetric edges; typed relations; temporal sequence memory; context nodes gating plasticity; hierarchical graphs; episodic buffers.

## Overlapping Snapshots and Episodes

Real memories overlap: textbook knowledge about flight, field observations of birds, and a personal flight in a twin‑engine Cessna share distributed assemblies and relations but differ in context, time, and specificity. Model overlap with assemblies/engrams rather than global symbols:

- Assemblies vs. episode-bound reactivations
  - Maintain distributed assemblies for recurring motifs (e.g., `wing_profile_microfeatures`, `lift_mechanics`, `crosswind_cue`). These are subgraphs with dense recurrent connectivity.
  - An episode/snapshot reactivates subsets of these assemblies and also recruits episode-specific microfeatures (e.g., smell, emotion, location) forming an episodic assembly tied to that time/place.
  - Optional handle nodes (assemblies) serve as indices to member microfeatures via `member_of` edges for manageability, but the memory content remains distributed.

- Scoped connectivity (assembly/local/bridge)
  - Assembly-internal edges: recurrent excitatory/inhibitory micro-connections that stabilize the pattern (semantic-like regularities).
  - Episode-local bindings: bindings between co-present assemblies and episode-specific features (time, place, affect).
  - Bridge links: learned connections where partial overlaps and repeated co-activation create routes between assemblies across episodes (analogy/transfer).
  - Represent scope with an edge field `scope: :assembly | :snapshot(id) | :bridge` and gate their gain during propagation/learning.

- Activation and gating across overlap
  - Cue-driven activation recruits assemblies via partial microfeature matches; episodic context boosts episode-local bindings.
  - Overlap is naturally integrated as shared microfeatures contribute current to multiple assemblies; lateral inhibition prevents runaway fusion.
  - Bridge diffusion is attenuated unless both sides show partial activation, encouraging meaningful analogies over noise.

- Learning policy
  - Intra-assembly: strengthen stabilizing recurrent paths and feature bindings when recall succeeds; induce sparsification to maintain capacity.
  - Consolidation: repeated cross-episode co-activation forms higher-order schema assemblies that coordinate lower-level assemblies.
  - Bridges: keep low-weight exploratory links formed during co-activation; retain only if reused.

- Conflict, provenance, and uncertainty
  - Store provenance on bindings and relation patterns; weight currents by confidence and source type (textbook, observation, episodic).
  - Represent contradictions between relation assemblies; retrieval arbitrates via task/goal and confidence.

### Worked example: textbook, birds, and Uncle Joe's Cessna

- Assembly A (Textbook Flight Ch.1): distributed pattern for `lift_mechanics` including microfeatures for airfoil curvature, airflow vectors, angle-of-attack regimes; strong recurrent stabilization.
- Assembly B (Bird Observation 2024‑05‑12): distributed pattern for `flapping_lift` with overlaps on wing surface features and airflow perturbations; bridges to A emerge via overlapping microfeatures and repeated co-activation.
- Assembly C (Personal Flight with Uncle Joe, twin‑engine Cessna): episodic assembly binding `lift_mechanics` subset with episode-specific microfeatures (runway texture, crosswind cue at 270°, engine vibration, Uncle Joe, cockpit visuals).
  - Episode-local bindings couple sensory microfeatures with control actions (pilot, takeoff roll) and context (airport, emotion).
  - During recall with cue “uncle joe cessna takeoff”, Assembly C activates and recruits shared microfeatures that also light up Assembly A; optional creative mode may recruit Assembly B if bridges exist.

### API sketch (future Graphoid)

```graphoid
schema_flight = schema.from_frame("FLIGHT")

# Assemblies as handles (content is distributed across microfeatures)
assembly_lift = assembly.new({ id: "lift_mechanics" })
assembly_flap = assembly.new({ id: "flapping_lift" })

# Microfeatures (illustrative)
mf_wing_curvature = feature.ensure({ id: "wing_curvature" })
mf_aoa = feature.ensure({ id: "angle_of_attack_region" })
mf_crosswind270 = feature.ensure({ id: "crosswind_270" })

# Membership (indexing) — actual recall uses the distributed microfeatures
assembly_lift.member(mf_wing_curvature)
assembly_lift.member(mf_aoa)
assembly_flap.member(mf_wing_curvature)

# Episode assembly for Uncle Joe's flight (binds distributed microfeatures + episodic cues)
snap_c = snapshot.new({ id: "Episode_UncleJoe_Cessna_Twin", provenance: ["episodic:memory"], time_range: [t_start, t_end] })
assembly_episode_c = assembly.new({ id: "episode_uncle_joe_cessna" })
assembly_episode_c.member(mf_crosswind270)
assembly_episode_c.member(mf_aoa)

# Bridge links emerge via co-activation
graph.add_edge(assembly_episode_c, assembly_lift, { type: "bridge", scope: :bridge, weight: 0.2 })

# Scope gains for propagation
graph.set_scope_gain(:snapshot("Episode_UncleJoe_Cessna_Twin"), 1.4)
graph.set_scope_gain(:bridge, 0.6)
```

### Retrieval over overlap

- Stage 1: score and activate snapshots by cue; adjust by provenance preference (episodic vs. semantic).
- Stage 2: recruit assemblies via partial microfeature matches; apply k‑sparsity/inhibition to stabilize.
- Stage 3: optional bridge expansion into aligned assemblies; read out active assemblies, salient microfeatures, and high‑current paths with provenance.

## Neuroscience inspiration (sidebar)

- Assemblies/engrams: Hebbian cell assemblies and engram work suggest memories are distributed across co-activating neurons; our assemblies emulate this with recurrent microfeature subgraphs and plasticity.
- Pattern separation/completion: DG/CA3-like roles can be mimicked via stronger inhibition and higher leak for separation, and stronger recurrence for completion; expose these as per-scope gains.
- Inhibition and sparsity: Interneuron-driven lateral inhibition maps to our k-sparsity and inhibitory edges, stabilizing attractors and preventing runaway fusion of overlaps.
- Neuromodulation: Context nodes can act as dopamine/norepinephrine proxies that gate plasticity (η) and exploration (noise), e.g., high novelty increases bridge formation.
- Consolidation and replay: Off-line ticks can replay recent episodes with higher η on assembly-internal edges to consolidate, while decay prunes unused connections.
- Timing vs rate: Spike timing (STDP) can be approximated with tick-local co-activation; if needed, add short eligibility traces to approximate temporal credit assignment.

## Toy dataset plan (for early experiments)

- Microfeatures (10–30): airflow_vector, aoa_low/med/high, wing_curvature, flapping_pattern, crosswind_270, runway_texture, engine_vibration, cockpit_visuals, bird_feather_surface, etc.
- Assemblies:
  - lift_mechanics: {airflow_vector, wing_curvature, aoa_med}
  - flapping_lift: {flapping_pattern, airflow_vector, wing_curvature}
  - episode_uncle_joe_cessna: {crosswind_270, runway_texture, engine_vibration, cockpit_visuals, aoa_med}
- Episodes/snapshots:
  - Textbook_Flight_Ch1 reactivates lift_mechanics strongly.
  - Birds_Observation reactivates flapping_lift.
  - UncleJoe_Cessna reactivates episode_uncle_joe_cessna and partially lift_mechanics.
- Overlap knobs: ensure airflow_vector and wing_curvature are shared; aoa_med shared between lift and episode; no direct overlap for flapping_pattern.
- Tasks:
  - Pattern completion: cue {runway_texture, engine_vibration} → retrieve episode_uncle_joe_cessna and partially lift_mechanics.
  - Analogy: cue {wing_curvature, airflow_vector} → retrieve lift_mechanics; with relaxed sparsity/noise, recruit flapping_lift.
  - Disambiguation: cue {airflow_vector} with textbook context → prefer lift_mechanics over flapping_lift.
- Metrics: top‑k accuracy of target assembly, activation entropy over ticks, bridge usage counts, stability (fixpoint distance), and consolidation gain (pre/post replay).
- Reproducibility: fixed RNG seed; deterministic update order; config snapshots for parameters (λ, η, β, μ, k, noise).


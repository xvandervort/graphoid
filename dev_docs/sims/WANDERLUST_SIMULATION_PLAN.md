# Wanderlust as an Evolutionary Force — Simulation Plan (Glang/Graphoid)

Updated: 2025-10-05

## Executive summary
- Model wanderlust as a heritable trait that increases exploration of novel environments; adaptability modulates survival after dispersal; creativity/intelligence modulate problem-solving and niche creation.
- Use a graph-native world: regions are nodes, traversable connections are edges; populations occupy nodes and flow along edges.
- Simulate agents (or cohorts) moving, surviving, reproducing, mutating, and occasionally speciating when gene flow is interrupted or trait divergence grows.
- Start with a tractable, deterministic-by-seed agent-based model; progress to cohort and fully graph-aggregate modes for performance.
- Call out missing primitives (graph ops, distributions, dataframes, visualization) and propose concrete language and stdlib extensions aligned with Graphoid roadmap.

---

## Conceptual model

- **World as graph**: Nodes = regions/biomes with environmental vectors (climate, resources, ruggedness, disease, predators). Edges = traversable connections with movement costs and permeability.
- **Populations/agents as graph substructures**: Individuals or cohorts reside on nodes; edges carry dispersal flow. Gene flow = co-location events on nodes.
- **Traits**:
  - wanderlust: baseline exploration drive, sensitivity to novelty.
  - adaptability: robustness to environmental mismatch; faster acclimatization.
  - creativity: increases ability to extract resources from challenging environments; creates micro-niches that raise effective carrying capacity.
  - intelligence: improves long-horizon navigation and social transmission; boosts survival in sparse/novel contexts.
- **Dynamics per tick (generation)**:
  1. Perceived novelty and expected utility computed for neighboring nodes
  2. Move decision influenced by wanderlust, novelty, and edge cost
  3. Survival in destination depends on adaptability vs environment mismatch (modulated by creativity/intelligence)
  4. Reproduction and mutation (small trait drifts; rare leaps)
  5. Gene flow update (which subpopulations co-located this tick)
  6. Speciation check: long isolation or trait distance triggers split

---

## Graph-centric data model

- `WorldGraph`
  - `nodes`: map<string, Region>
  - `edges`: map<string, list<string>> (adjacency)
- `Region`
  - `id`: string
  - `env`: map<string, num> (e.g., temp, aridity, seasonality, pathogens, resources, ruggedness)
  - `novelty_decay`: num (speed at which novelty fades with occupancy)
- `Species`
  - `name`: string
  - `traits`: map<string, num> baseline means
  - `tolerance`: map<string, num> acceptable environment deltas
- `Individual` (agent mode) or `Cohort` (aggregate mode)
  - `id`: string
  - `species`: string
  - `traits`: map<string, num> (wanderlust, adaptability, creativity, intelligence)
  - `loc`: string (region id)
  - `energy`: num
  - `age`: num
  - `parents`: list<string> (optional)

- Global run-time state
  - `occupancy[region][species] -> count`
  - `novelty[region] -> num` (decays with occupancy; increases after local extirpation)
  - `last_contact[speciesA][speciesB] -> tick` (for speciation clock)
  - `seed`: num for deterministic RNG

---

## Core equations (suggested)

- Novelty signal at neighbor `r`: \( N_r = novelty[r] \)
- Movement propensity for individual/cohort `i` from `cur` to `r`:
  \[ P_\text{move}(i, cur \to r) = \sigma( \alpha_w \cdot wanderlust_i \cdot N_r - \alpha_c \cdot cost(cur,r) + \alpha_u \cdot U(i,r) ) \]
  where \( \sigma(x) = 1 / (1 + e^{-x}) \)

- Expected utility at `r` given environment mismatch `\Delta` and creativity/intelligence:
  \[ U(i,r) = base\_resources(r) - \beta_m \cdot \lVert \Delta(i,r) \rVert + \gamma_c \cdot creativity_i + \gamma_g \cdot intelligence_i \]

- Survival probability after move:
  \[ P_\text{survive}(i,r) = \sigma( \eta_a \cdot adaptability_i - \beta_m \cdot \lVert \Delta(i,r) \rVert ) \]

- Speciation triggers (either condition is sufficient):
  - Isolation: time since last co-location between subpopulations > `ISOLATION_TICKS`
  - Trait divergence: \( \lVert mean\_traits_{sub1} - mean\_traits_{sub2} \rVert_2 > D_{spec}\)

---

## Simulation loop (agent-mode first)

1. Initialize world graph, species, seed populations.
2. For `tick` in `0..T`:
   - For each individual/cohort:
     - Evaluate neighbors; compute `P_move` and select destination
     - Attempt move; if moved, evaluate survival `P_survive`
     - Reproduce if energy/age thresholds met; mutate traits
   - Update novelty, occupancy, last_contact
   - Run speciation checks
   - Log metrics (range size, dispersal distances, survival, diversity indices)

Cohort mode mirrors agent mode but updates counts and trait distributions per node instead of iterating each individual.

---

## Sample Glang code (agent mode, minimal viable)

```glang
import "random"   # rand.* alias also available
import "time"     # optional, for timestamps
# import "io"     # for CSV output later

# --- Parameters ---
MAX_TICKS = 200
ISOLATION_TICKS = 40
D_SPEC = 1.2  # trait-distance threshold for speciation
ALPHA_W = 1.0
ALPHA_C = 0.6
ALPHA_U = 0.4
BETA_M = 0.8
ETA_A = 0.9
MUTATION_SD = 0.05
CARRYING_CAP = 200  # per region baseline

# --- Utility functions ---
func logistic(x) { return 1.0 / (1.0 + (0.0 - x).exp()) }

# NOTE: If rand.normal/gauss is not available yet, replace with small uniform noise
func mutate(value, sd) {
    # Prefer Gaussian: value + rand.normal(0, sd)
    noise = (rand.random() - 0.5) * 2.0 * sd  # fallback uniform in [-sd, sd]
    return value + noise
}

func env_mismatch(ind, region) {
    # Sum absolute differences across tracked environmental dimensions
    total = 0.0
    for key in region["env"].keys() {
        if ind["tolerance"].has_key(key) {
            diff = (region["env"][key] - ind["env_pref"][key]).abs()
            total = total + (diff - ind["tolerance"][key])
        } else {
            # If no tolerance defined, treat as fully mismatched
            total = total + (region["env"][key]).abs()
        }
    }
    return total.max(0.0)
}

func expected_utility(ind, region) {
    base = region["env"]["resources"] - (BETA_M * env_mismatch(ind, region))
    return base + (0.3 * ind["traits"]["creativity"]) + (0.2 * ind["traits"]["intelligence"])
}

func move_probability(ind, cur_region, next_region, novelty, edge_cost) {
    drive = ALPHA_W * ind["traits"]["wanderlust"] * novelty
    util = ALPHA_U * expected_utility(ind, next_region)
    cost = ALPHA_C * edge_cost
    return logistic(drive + util - cost)
}

# Pick neighbor by softmax over move probabilities (or epsilon-greedy)
func pick_destination(ind, cur_id, world, novelty_map) {
    neighbors = world["edges"][cur_id]
    if neighbors.size() == 0 { return cur_id }

    best = cur_id
    best_p = 0.0
    for nid in neighbors {
        nregion = world["nodes"][nid]
        edge_cost = nregion["env"]["ruggedness"]
        p = move_probability(ind, world["nodes"][cur_id], nregion, novelty_map[nid], edge_cost)
        if p > best_p { best_p = p; best = nid }
    }

    # Stochastic realization
    if rand.random() < best_p { return best } else { return cur_id }
}

func survive_probability(ind, region) {
    return logistic(ETA_A * ind["traits"]["adaptability"] - (BETA_M * env_mismatch(ind, region)))
}

func attempt_reproduction(ind) {
    # Simple energy/age gates; replace with richer life history later
    return (ind["energy"] > 1.2) and (ind["age"] > 3)
}

func reproduce(ind, next_id) {
    child = {
        "id": "i_" + next_id.to_string(),
        "species": ind["species"],
        "traits": {
            "wanderlust": mutate(ind["traits"]["wanderlust"], MUTATION_SD),
            "adaptability": mutate(ind["traits"]["adaptability"], MUTATION_SD),
            "creativity": mutate(ind["traits"]["creativity"], MUTATION_SD),
            "intelligence": mutate(ind["traits"]["intelligence"], MUTATION_SD)
        },
        "env_pref": ind["env_pref"],
        "tolerance": ind["tolerance"],
        "loc": ind["loc"],
        "energy": 1.0,
        "age": 0,
        "parents": [ind["id"]]
    }
    return child
}

# --- Initialization ---
func make_world() {
    nodes = {}
    # Example: three-region chain with differing resources/climate
    nodes["A"] = { "id": "A", "env": { "resources": 1.0, "temp": 0.2, "aridity": 0.3, "ruggedness": 0.2 }, "novelty_decay": 0.02 }
    nodes["B"] = { "id": "B", "env": { "resources": 1.2, "temp": 0.5, "aridity": 0.4, "ruggedness": 0.3 }, "novelty_decay": 0.03 }
    nodes["C"] = { "id": "C", "env": { "resources": 0.9, "temp": 0.7, "aridity": 0.6, "ruggedness": 0.5 }, "novelty_decay": 0.05 }

    edges = { "A": ["B"], "B": ["A", "C"], "C": ["B"] }
    return { "nodes": nodes, "edges": edges }
}

func seed_population(species_name) {
    pop = []
    for k in [0,1,2,3,4] {
        pop.append({
            "id": "i_" + k.to_string(),
            "species": species_name,
            "traits": { "wanderlust": 0.6, "adaptability": 0.6, "creativity": 0.4, "intelligence": 0.5 },
            "env_pref": { "temp": 0.4, "aridity": 0.4 },
            "tolerance": { "temp": 0.25, "aridity": 0.25 },
            "loc": "A",
            "energy": 1.0,
            "age": 0,
            "parents": []
        })
    }
    return pop
}

# --- Main simulation ---
func run() {
    world = make_world()
    novelty = { "A": 1.0, "B": 1.0, "C": 1.0 }

    pop = seed_population("Homo_like")
    next_id = 5

    tick = 0
    while tick < MAX_TICKS {
        # Movement + survival
        survivors = []
        for ind in pop {
            dest = pick_destination(ind, ind["loc"], world, novelty)
            ind["loc"] = dest

            sprob = survive_probability(ind, world["nodes"][dest])
            if rand.random() < sprob {
                ind["energy"] = (ind["energy"] + expected_utility(ind, world["nodes"][dest]))
                ind["age"] = ind["age"] + 1
                survivors.append(ind)
            }
        }

        # Density dependence and novelty decay
        occupancy = { "A": 0, "B": 0, "C": 0 }
        for s in survivors { occupancy[s["loc"]] = occupancy[s["loc"]] + 1 }
        for rid in occupancy.keys() {
            load = occupancy[rid] / CARRYING_CAP
            novelty[rid] = (novelty[rid] - world["nodes"][rid]["novelty_decay"]) .max(0.0)
            novelty[rid] = novelty[rid] - (0.1 * load)
            if novelty[rid] < 0.0 { novelty[rid] = 0.0 }
            if occupancy[rid] == 0 { novelty[rid] = (novelty[rid] + 0.02).min(1.0) }
        }

        # Reproduction
        next_gen = []
        for s in survivors {
            next_gen.append(s)
            if attempt_reproduction(s) {
                child = reproduce(s, next_id)
                next_id = next_id + 1
                next_gen.append(child)
            }
        }

        pop = next_gen
        tick = tick + 1
    }

    # TODO: logging, speciation check, metrics export
}

run()
```

Notes:
- The example uses a uniform-noise fallback for mutation; replace with `rand.normal()` once available.
- Replace density/novelty rules with graph-behavior rules once graph behaviors are first-class.

---

## Experiments to run
- Vary wanderlust while holding adaptability fixed; measure range size, dispersal distance distribution, extinction risk.
- Vary adaptability while holding wanderlust fixed; measure survival upon dispersal and speciation rate.
- Orthogonal grid: sweep wanderlust × adaptability; quantify Pareto frontier between range expansion and speciation.
- Add creativity/intelligence; test if they reduce extinction in marginal/novel environments and increase realized carrying capacity.

---

## Metrics
- Range size over time (unique nodes occupied per species)
- Mean dispersal distance per generation; tail heaviness (rare long jumps)
- Survival rates post-dispersal; local extinction rates
- Gene flow continuity (time since last contact per subpopulation pair)
- Speciation events over time; trait distance at speciation
- Diversity indices (Shannon/Simpson) per region and globally

---

## Language and stdlib gaps (with proposals)

1. Graph operations (Priority High)
   - Needed: BFS/DFS, shortest paths, connected components, betweenness/degree, per-edge weights, per-node attributes.
   - Proposal: `graph` module with:
     - `graph.from_adjacency(map)`; `graph.bfs(start)`; `graph.shortest_path(a,b, weight="cost")`
     - Node/edge property getters/setters; graph behaviors (e.g., automatic novelty decay per tick)

2. Probabilistic distributions
   - Needed: Normal/Gaussian, lognormal, beta/binomial, categorical with weights.
   - Proposal: `random.normal(mu, sigma)`, `random.choice_weighted(values, weights)`.

3. Aggregates/DataFrames
   - Needed: grouping by region/species, rolling means, pivots for metrics.
   - Proposal: `dataframe`-lite or `stats` module with `group_by`, `agg`, `mean`, `variance`.

4. Visualization/Export
   - Needed: DOT/Mermaid export of occupancy graph; CSV/JSON logs.
   - Proposal: `graph.export("dot")`, `io.csv.write(rows, path)`, `json.encode(map)` (Rust stdlib JSON module migration).

5. Time and scheduling
   - Needed: tick scheduler, event queue for agent-based simulations.
   - Proposal: `sim` module: `sim.tick(n)`, `sim.schedule(time, func)`.

6. Performance
   - Needed: cohort mode helpers; vectorized operations; sampling utilities.
   - Proposal: `cohort` helpers: update counts and trait means per node; `sample_without_replacement`.

7. Reproducibility
   - Needed: explicit RNG seeding and streams.
   - Proposal: `random.seed(n)`, `random.stream(id)` to create independent deterministic streams.

8. Math helpers
   - Needed: `exp`, `abs`, `max/min` already present via number methods; ensure completeness and consistency.

9. Logging and tracing
   - Needed: lightweight, structured logging with levels; per-tick tracing toggle.
   - Proposal: `log.info(map)`, `log.debug(...)`, `configure { log_level: "info" } { ... }`.

---

## Roadmap to implementation
1. Start with agent mode on a small world graph; confirm qualitative behaviors (higher wanderlust → broader range; low adaptability → higher mortality and speciation via isolation).
2. Add cohort mode for performance (counts + trait means/variances per node); validate parity with agent mode on small cases.
3. Implement speciation tracking: connected components of co-breeding graph over sliding windows; trait distance thresholding.
4. Add CSV/JSON logging; produce plots externally while native visualization matures.
5. Introduce graph ops and weighted-choice in stdlib; refactor movement and novelty to graph behaviors.
6. Scale to larger worlds; run parameter sweeps.

---

## Open questions
- How should graph behaviors compose with container behaviors (e.g., novelty decay and density dependence)? Ordering guarantees needed.
- Best interface for co-breeding graph extraction from agent/cohort records?
- Should creativity/intelligence affect group-level cultural transmission explicitly (e.g., shared buffers on nodes)?

---

## Appendix: Cohort-mode sketch

Cohort mode replaces individual iteration with per-node state updates:
- State: `pop[region][species] -> { count, traits_mean_vec, traits_var_vec }`
- Movement: fraction flows along edges using softmax of `P_move` expectations
- Survival: apply multiplicative survival to counts using mean mismatch and adaptability
- Reproduction/Mutation: update means/vars via standard moment updates

This reduces complexity from O(N_agents) to O(N_nodes × N_species × degree).


# Training Data Generation Plan

## Context

Graphoid exists in zero pre-training corpora. To fine-tune a local LLM to write idiomatic Graphoid, we need instruction pairs (SFT), preference pairs (DPO), Rosetta Stone cross-language pairs, and annotated examples — all organized by **language feature**, not implementation phase.

Current state: 5 instruction pairs, 2 DPO pairs, 1 Rosetta pair. Targets: 5,000 SFT, 500 DPO, 100+ Rosetta, 200 eval problems.

## Approach

Each session tackles one or more **feature groups**. For each group we produce:
- **Instruction pairs** (JSONL) — "How do I X?" / "Write code that Y" → working Graphoid code
- **DPO pairs** (JSONL) — idiomatic vs anti-pattern for the same prompt
- **Rosetta pairs** — Python + Graphoid doing the same thing
- **Annotated examples** — richly commented .gr files

Source of truth: `dev_docs/LANGUAGE_SPECIFICATION.md` and existing `samples/`.

## Feature Groups & Session Plan

### Session 1: Graphs & Collections (highest priority — core differentiator)
**Instruction pairs** (~40): `training/instruct/graphs.jsonl`
- Graph creation (directed, dag, tree, undirected)
- Node/edge CRUD (add_node, add_edge, remove)
- Querying (nodes, edges, neighbors, has_node, has_edge)
- Path finding (has_path, shortest_path, all_paths)
- Subgraph operations (subgraph, merge)
- Graph rules (no_cycles, single_root, custom rules)
- Lists, maps, trees — creation, methods, iteration
- Element-wise operations (.+, .-, .*)

**DPO pairs** (~8): `training/dpo/graphs.jsonl`
- Graph rules vs manual validation
- for..in vs index-based iteration
- Functional (.map/.filter) vs imperative loops

**Rosetta pairs** (3): `training/rosetta/python_graphoid/`
- Graph traversal (networkx vs Graphoid)
- Collection operations (list comprehension vs .map/.filter)
- Tree operations

### Session 2: Concurrency & Actors
**Instruction pairs** (~30): `training/instruct/concurrency.jsonl`
- spawn syntax and task isolation
- Channel creation, send, receive, close
- for..in channel iteration
- select() multiplexing with match
- Actor spawning (spawn Graph{})
- Actor messaging (.send, .request)
- Supervision (.supervise, restart strategies)
- Timers (timer.after, timer.every)
- Signals (signal handling patterns)

**DPO pairs** (~6): `training/dpo/concurrency.jsonl`
- Channel-based communication vs shared state
- Proper actor message patterns vs raw channel use
- Supervision vs manual error handling

**Rosetta pairs** (3): `training/rosetta/python_graphoid/`
- Threading/asyncio vs spawn+channels
- Actor pattern (manual vs built-in)

### Session 3: Functions, Pattern Matching & Error Handling
**Instruction pairs** (~35): `training/instruct/functions.jsonl`
- Function definitions (fn, lambdas, closures)
- Pattern matching (match, guards, destructuring)
- Function overloading (multiple clauses)
- Named arguments, default values
- Receiver functions (fn graph.method)
- Try/catch/raise
- Custom error types
- Error propagation patterns

**DPO pairs** (~6): `training/dpo/functions.jsonl`
- Pattern match vs if/else chains
- try/catch vs return-value error handling
- Lambda vs named function (when each is appropriate)

**Rosetta pairs** (3): `training/rosetta/python_graphoid/`
- Pattern matching (Python match vs Graphoid match)
- Error handling (try/except vs try/catch)

### Session 4: Behaviors, Rules & Class-Like Graphs
**Instruction pairs** (~30): `training/instruct/behaviors.jsonl`
- Intrinsic behaviors (add_rule for transforms)
- Standard behaviors (none_to_zero, validate_range)
- Custom function behaviors
- Conditional behaviors
- Rulesets (declarative bundles)
- CLG patterns (Graph as class: properties, methods, inheritance)
- configure directive (readable, writable)
- Privacy (priv blocks)
- Graph method constraints, when dispatch

**DPO pairs** (~6): `training/dpo/behaviors.jsonl`
- configure { readable: :x } vs manual getters
- ClassName { prop: val } vs new() boilerplate
- Behaviors vs manual validation code

**Rosetta pairs** (2): `training/rosetta/python_graphoid/`
- Python class vs Graphoid CLG
- Python decorator/validator vs behaviors

### Session 5: Modules, Stdlib & Strings
**Instruction pairs** (~30): `training/instruct/modules.jsonl`
- import syntax, selective imports
- Module creation, aliases
- priv keyword and blocks
- Reflect API (reflect.universe, modules.list, runtime.*)
- String methods (split, join, replace, slice, upper, lower)
- String generators (string.generate)
- Math, statistics, time, json, random stdlib usage
- fs/net/http stdlib usage

**DPO pairs** (~5): `training/dpo/modules.jsonl`
- import "random" (alias auto-available) vs import "random" as rand
- One method with params vs method proliferation
- Stdlib usage vs reimplementation

**Rosetta pairs** (3): `training/rosetta/python_graphoid/`
- Python import vs Graphoid import
- String operations comparison
- JSON/HTTP comparison

### Session 6: FFI, Testing & Eval Benchmark
**Instruction pairs** (~25): `training/instruct/ffi_extended.jsonl` + `testing.jsonl`
- FFI: lib.cdef, structs, callbacks, taint, limits (extending existing 5)
- gspec: describe/context/it blocks
- Expectations (to_equal, to_be_truthy, to_raise, etc.)
- before_each/after_each hooks
- Running specs (gr spec)

**DPO pairs** (~5): `training/dpo/testing.jsonl`
- gspec style vs ad-hoc print-based testing
- Proper expectation matchers vs manual assert

**Rosetta pairs** (2): `training/rosetta/python_graphoid/`
- pytest vs gspec
- ctypes vs Graphoid FFI

**Eval benchmark** (200 problems): `training/eval/graphoid_eval.jsonl`
- 25 per category: basics, collections, graphs, functions, concurrency, modules, testing, real-world

## File Structure

```
training/
  instruct/
    ffi_safety.jsonl        # existing (5 pairs)
    graphs.jsonl             # Session 1
    concurrency.jsonl        # Session 2
    functions.jsonl          # Session 3
    behaviors.jsonl          # Session 4
    modules.jsonl            # Session 5
    ffi_extended.jsonl       # Session 6
    testing.jsonl            # Session 6
  dpo/
    ffi_safety.jsonl         # existing (2 pairs)
    graphs.jsonl             # Session 1
    concurrency.jsonl        # Session 2
    functions.jsonl          # Session 3
    behaviors.jsonl          # Session 4
    modules.jsonl            # Session 5
    testing.jsonl            # Session 6
  rosetta/python_graphoid/
    ffi_safety.{py,gr}       # existing
    graph_traversal.{py,gr}  # Session 1
    collections.{py,gr}      # Session 1
    concurrency.{py,gr}      # Session 2
    pattern_matching.{py,gr} # Session 3
    error_handling.{py,gr}   # Session 3
    classes_vs_clg.{py,gr}   # Session 4
    imports.{py,gr}          # Session 5
    strings.{py,gr}          # Session 5
    testing.{py,gr}          # Session 6
  eval/
    graphoid_eval.jsonl      # Session 6
```

## Expected Totals After All Sessions

| Type | Per Session | Sessions | Total | Target | Coverage |
|------|-----------|----------|-------|--------|----------|
| Instruction | ~30-40 | 6 | ~190 | 5,000 | 4% |
| DPO | ~5-8 | 6 | ~36 | 500 | 7% |
| Rosetta | ~2-3 | 6 | ~16 | 100 | 16% |
| Eval | 200 | 1 | 200 | 200 | 100% |

**Honest assessment**: 6 sessions of hand-crafted pairs gets us ~190 SFT and ~36 DPO — well short of 5K/500 targets. To reach those targets we'll need:
- A generation script that creates variations from templates (10x multiplier)
- Mining existing samples/tests for implicit instruction pairs
- Bulk Rosetta generation from the 89 sample files

The 6 sessions establish **high-quality seed data** covering every feature. Scaling to target volumes is a separate automation step.

## Progress Tracking

After each session, update this table:

| Session | Feature Group | SFT | DPO | Rosetta | Status |
|---------|--------------|-----|-----|---------|--------|
| — | FFI Safety (Phase 20c) | 5 | 2 | 1 | Done |
| 1 | Graphs & Collections | 50 | 8 | 3 | Done |
| 2 | Concurrency & Actors | 29 | 6 | 2 | Done |
| 3 | Functions & Error Handling | 32 | 6 | 2 | Done |
| 4 | Behaviors & CLG | 29 | 6 | 2 | Done |
| 5 | Modules & Stdlib | | | | |
| 6 | FFI, Testing & Eval | | | | |

## Per-Session Workflow

1. Read relevant spec sections for the feature group
2. Read existing samples for that feature
3. Write instruction pairs (JSONL)
4. Write DPO pairs (JSONL)
5. Write Rosetta pairs (.py + .gr)
6. Run `python3 training/scripts/stats.py` to verify
7. Run `python3 training/scripts/collect_corpus.py` to rebuild corpus
8. Update progress table above

## Verification

After each session: `python3 training/scripts/stats.py` shows updated counts.
After all sessions: validate JSONL format, run eval benchmark scoring.

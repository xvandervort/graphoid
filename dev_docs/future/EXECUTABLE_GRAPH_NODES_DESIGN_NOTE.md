## Graphoid Executable Graph Nodes Design Note

### Purpose
- Explore how graphs can host executable nodes that behave like class methods while preserving Graphoid's graph-first philosophy.
- Outline mechanisms for invoking code from within the graph, including self-modification capabilities and external execution targets.
- Provide implementation direction for integrating executable nodes without compromising current roadmap milestones.

### Concept Overview
- Treat executable nodes as first-class citizens that encapsulate code, execution context, and linkage to data nodes.
- Support dual execution pathways: **embedded** (interpreted/compiled within Graphoid runtime) and **delegated** (invoked via external executors or services).
- Enable graph-driven metaprogramming where code nodes can traverse and mutate graph structure, enabling object-like behaviors.

### Executable Node Anatomy
- **Code Payload**: source in Graphoid, WASM bytecode, or external function references.
- **Signature Edge Set**: incoming edges describing parameters, required capabilities, input graph scopes.
- **Effect Contracts**: metadata edges specifying permissible graph mutations, resource usage, and side effects.
- **Execution Bindings**: edges linking to environment nodes (credentials, runtime settings, scheduler hints).

### Invocation Models
- **In-graph invocation**: other nodes issue `call` edges, scheduling the executable node through the runtime's behavior system.
- **Event-triggered invocation**: behaviors attach to graph mutations or stream events, enqueueing code nodes automatically.
- **External orchestration**: module system exports executable node descriptors that external orchestrators can call, treating Graphoid as a programmable knowledge base.

### Safety & Determinism
- Require explicit capability grants for structural mutation (add/remove nodes, rewire edges) to prevent runaway self-modification.
- Provide sandboxed execution contexts with resource quotas (time, memory) and audit logs mapping mutations to execution IDs.
- Introduce behavior-level checkpoints that capture before/after graph snapshots for rollback or replay.

### Integration with Existing Architecture
- **Behavior System**: treat executable nodes as specialized behaviors, allowing reuse of scheduling, ordering, and freeze mechanics.
- **Module System (Phase 8)**: expose executable nodes via module exports, enabling encapsulated graph-object definitions.
- **Execution Engine (Phase 3+)**: extend evaluator to recognize `ExecNode` values, with adapters for internal vs external execution.
- **Graph Rules**: enforce invariants that limit where executable nodes can connect, avoiding cyclic self-invocation explosions.

### Object-Like Patterns
- Define graph archetypes resembling classes: a root node representing the object, with executable nodes for `init`, `update`, `query`, etc.
- Support instance creation by cloning archetype subgraphs and binding them to runtime environments.
- Allow executable nodes to mutate their local instance subgraph, enabling encapsulated state transitions reminiscent of methods.

### External Execution Bridge
- Specify a standard descriptor (JSON/YAML/WASM manifest) exported by executable nodes for remote execution engines.
- Include cryptographic signatures and hash edges for integrity, enabling trusted execution by external services.
- Provide callback edges for responses so external engines can re-enter the graph with results or mutations.

### Tooling Considerations
- Extend debugger plans (Phase 13) with step-through capabilities for executable nodes, including graph diff visualizations.
- Enhance testing framework (Phase 12) to mock executable node invocations and assert graph mutation expectations.
- Prepare package manager (Phase 14) to distribute executable node archetypes and shared manifests.

### Roadmap Alignment
- **Phase 3/4**: design value representations for executable nodes and extend environment storage for code payloads.
- **Phase 6/7**: apply graph rules and behavior ordering to manage execution dependencies safely.
- **Phase 8+**: publish module patterns for object-like graph assemblies.
- **Phase 10-11**: integrate native modules for external execution connectors (e.g., WASM runtime, RPC hooks).

### Open Questions
- What execution formats should be supported first (pure Graphoid, WASM, native Rust callbacks)?
- How to guarantee idempotent mutations when executable nodes modify graph structure?
- Should the runtime permit self-modifying code that rewrites its own payload, and how do we audit it?
- What policies govern recursive or concurrent invocations to prevent deadlocks or runaway growth?

### Immediate Next Steps
- Prototype executable node representation in the Python reference to clarify data model and mutation semantics.
- Draft capability schemas describing allowable graph mutations for executable nodes.
- Review module system plans to ensure export/import of executable nodes is feasible.
- Collect use cases from teams seeking graph-backed objects to prioritize features (e.g., workflow engines, knowledge graph automation).

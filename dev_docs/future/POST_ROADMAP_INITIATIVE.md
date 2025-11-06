## Graphoid Post-Roadmap Initiative

### Purpose
- Outline foundational tasks that extend the current 14-phase roadmap once Phase 14 completes.
- Capture early planning for capability annotations, deterministic scheduling hooks, and fixed-width numeric types needed for advanced targets (embedded, robotics, self-hosting).
- Provide scheduling anchors for future prioritization alongside other forthcoming initiatives.

### Initiative Themes

#### 1. Capability Annotation System
- **Goal**: Introduce graph-native capability metadata controlling access to privileged operations (hardware IO, memory regions, external services).
- **Deliverables**:
  - Language syntax for annotating nodes/edges with capability requirements.
  - Runtime enforcement engine mapping capabilities to execution contexts.
  - Module metadata schema so packaged components declare required/provided capabilities.
- **Dependencies**: Completion of module system (Phase 8) and graph rule infrastructure (Phase 6).
- **Open Questions**: how granular should capabilities be; how to integrate with testing to simulate restricted environments.

#### 2. Deterministic Scheduler Hooks
- **Goal**: Extend execution engine with explicit hooks for real-time and deterministic scheduling policies.
- **Deliverables**:
  - Scheduling API allowing behaviors to declare priorities, periods, and deadlines.
  - Runtime hooks for plugging in scheduling algorithms (rate-monotonic, EDF) and tracing execution order.
  - Testing harness to validate timing contracts (e.g., `expect_schedule_within`).
- **Dependencies**: Execution engine maturity (Phase 3-5) and behavior ordering work (Phase 7).
- **Open Questions**: default scheduling model vs optional RT extensions; interaction with distributed execution queues.

#### 3. Fixed-Width Numeric Types & Bit Operations
- **Goal**: Provide deterministic, hardware-friendly numeric primitives (`u8`, `i16`, `f32`, etc.) within Graphoid's type system while respecting the no-generics policy.
- **Deliverables**:
  - Type definitions and parser support for fixed-width scalars.
  - Arithmetic, bitwise, and conversion operations with overflow policies (configurable wrap, saturate, trap).
  - Interoperability layer ensuring collections and graph values can store fixed-width types alongside existing numeric abstractions.
- **Dependencies**: Value system extensibility (Phase 3) and type checking rules in parser.
- **Open Questions**: default overflow behavior; how to expose SIMD or hardware acceleration.

### Supporting Infrastructure
- **Testing Enhancements**: Extend RSpec-style framework with numeric precision assertions and deterministic schedule simulations.
- **Documentation**: Update language specification to cover capability annotations, scheduler semantics, and numeric types; produce migration guides for existing code.
- **Tooling**: Enhance debugger to visualize capability violations, scheduler timelines, and binary-level data inspection.
- **Packaging**: Plan for capability-aware manifests in the package manager, enabling deployment tools to validate hardware requirements.

### Suggested Timeline (Sequenced Post-Phase 14)
- **Milestone A**: Capability annotation prototype (2-3 weeks) → integrate with module metadata (1 week) → add enforcement to runtime (2 weeks).
- **Milestone B**: Deterministic scheduling hooks design review (1 week) → runtime implementation (3-4 weeks) → testing harness & debugger updates (2 weeks).
- **Milestone C**: Fixed-width type parser + value support (2 weeks) → operations and conversions (2 weeks) → documentation and tooling updates (1 week).
- **Integration Review**: Cross-cutting audit ensuring annotations, scheduler hooks, and numeric types interoperate without regressing existing behavior (1 week).

### Next Steps
- Socialize this initiative document with roadmap owners to slot into long-range planning.
- Prioritize capability annotation design to unlock safe experimentation with privileged modules.
- Begin exploratory spikes in Python prototype to validate scheduler APIs and fixed-width type ergonomics before committing to Rust implementation.

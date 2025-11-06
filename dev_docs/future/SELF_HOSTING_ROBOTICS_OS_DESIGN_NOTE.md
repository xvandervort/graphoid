## Graphoid Self-Hosting Robotics OS Design Note

### Purpose
- Explore requirements for using Graphoid to build low-level operating systems for drones/robots.
- Identify language/runtime changes needed for self-hosting in resource-constrained, real-time environments.
- Map long-term roadmap extensions to bridge the gap between current high-level focus and bare-metal control.

### Target Scenario
- Graphoid program orchestrates sensor fusion, control loops, navigation, and mission planning on embedded hardware.
- Graph nodes represent hardware interfaces, drivers, tasks, and behaviors; executable nodes coordinate control logic.
- System must guarantee deterministic timing, bounded resource usage, and safe self-modification.

### Required Capabilities

#### 1. Low-Level Hardware Access
- Memory-mapped I/O and register-level operations exposed through graph-native abstractions (e.g., `hardware::register` nodes).
- Interrupt handling modeled as event streams feeding into behaviors; ability to prioritize latency-critical handlers.
- Direct integration with DMA, timers, and sensor buses (I2C, SPI, CAN) via dedicated module bindings.

#### 2. Deterministic Scheduling & Real-Time Guarantees
- Hard real-time scheduler with priority queues for behaviors and executable nodes; support rate-monotonic and EDF policies.
- Static analysis tools to compute worst-case execution time (WCET) for graph segments.
- Graph-level preemption metadata to indicate which behaviors are interruptible.

#### 3. Memory Management & Footprint Control
- Region-based or arena allocators represented as graph resources, enabling compile-time reasoning about lifetime.
- Ability to pin subgraphs into static memory to avoid allocator jitter; optional garbage collector disabled by default in safety-critical builds.
- Support for persistent storage via flash/EEPROM nodes with journaling behaviors.

#### 4. Self-Hosting Compilation Toolchain
- Graphoid compiler written in Graphoid, capable of emitting low-level code (LLVM IR, WASM, or custom ISA) suitable for embedded targets.
- Bootstrapping strategy: use current Rust implementation as stage-0, then gradually replace components with Graphoid equivalents.
- Cross-compilation pipeline hooking into package manager for firmware images.

#### 5. Safety & Verification
- Formal specification of critical behaviors, enabling model checking or theorem proving on control graphs.
- Contracts on executable nodes describing invariants (e.g., motor command bounds) enforced at compile time or runtime.
- Fault containment regions: subgraphs that can be reset or rolled back without rebooting entire system.

#### 6. Offline & Autonomous Operation
- Deterministic deployment bundles containing pruned subgraphs, configuration manifests, and embedded assets.
- Replay logs for post-flight analysis encoded as graph deltas.
- Support for over-the-air updates via graph diff application with safety staging.

### Language Changes
- Introduce `capability` annotations on nodes/edges to control access to privileged operations.
- Extend type system with fixed-size numeric types (`u8`, `i16`, `f32`) and bit-level operations while respecting no-generics policy (single-parameter constraints still allowed).
- Provide syntactic sugar for periodic tasks, interrupts, and state machines (e.g., `behavior tick(period=5ms)` blocks).
- Add compile-time resource declarations (`memory region`, `io port`) integrated into module system.

### Runtime & Tooling Extensions
- Real-time kernel subset implemented in Graphoid: scheduler graph, driver graph, device trees expressed as subgraphs.
- Deterministic simulation environment to test graph-based firmware on host machines with virtual sensors/actuators.
- Static analyzer generating safety/latency reports; integrate into testing DSL with real-time assertions (`expect_latency_below`).
- Debugger extensions for timeline tracing, interrupt visualization, and memory inspection at node granularity.

### Roadmap Alignment
- **Phase 3-5**: ensure value system and execution engine can operate deterministically with bounded allocations.
- **Phase 6**: enhance graph rules to express safety invariants and capability constraints.
- **Phase 8**: module system must let hardware vendors ship driver graphs and capability manifests.
- **Beyond Phase 14**: introduce self-hosting compiler milestones, real-time runtime modules, and embedded deployment tooling.

### Advantages
- Unified abstraction from high-level mission planning to low-level control, enabling introspection and live adaptation.
- Graph-based representation simplifies reasoning about dependencies, safety zones, and fault recovery.
- Self-hosting fosters ecosystem autonomy: Graphoid can evolve without relying on external toolchains.

### Challenges & Risks
- Significant effort to achieve hard real-time guarantees and certification (DO-178C, ISO 26262) on graph-driven runtimes.
- Language complexity increases; must preserve simplicity for non-embedded users via modular opt-in.
- Bootstrapping self-hosting requires multi-stage compiler strategy and rigorous verification to avoid regressions.

### Open Questions
- Which embedded targets to prioritize (ARM Cortex-M, RISC-V)?
- Should Graphoid runtime include a microkernel or cooperate with existing RTOS (Zephyr, FreeRTOS)?
- How to provide formal verification tooling accessible to graph-first developers?
- What minimum subset of Graphoid is necessary for trusted firmware (e.g., no dynamic behaviors, restricted mutability)?

### Immediate Next Steps
- Draft a capability model aligning hardware access with graph permissions.
- Prototype graph-based driver definitions in the Python reference to validate abstractions.
- Evaluate existing real-time schedulers for integration or reimplementation in Graphoid.
- Develop a staged self-hosting plan mapping compiler components to Graphoid equivalents.

# Platform Process

**Status**: Design
**Priority**: 7 (Advanced)
**Dependencies**: Phase 19 Concurrency (19.1-19.5), Platform Runtime, Platform Monitor

---

## Overview

The Platform Process component provides process isolation and supervision where **supervision trees ARE graphs**. Processes are nodes; supervision relationships are edges.

**Key Principles**:
- Processes are nodes in a supervision graph
- Supervisor relationships are edges
- Fault propagation follows edges
- Recovery strategies are graph operations
- Inspired by Erlang/OTP, native to Graphoid's graph model

---

## When You Need This

Most applications don't need explicit process management. Use this when:

- Running multiple independent tasks concurrently
- Isolating untrusted or risky code
- Building fault-tolerant systems
- Implementing worker pools
- Creating long-running background tasks

---

## Basic Process Creation

```graphoid
import "platform/process"

# Spawn a process
pid = process.spawn(fn() {
    log("Hello from process")
    # Do work...
})

# Wait for completion
result = process.await(pid)
```

### Process with Arguments

```graphoid
fn worker(name, count) {
    for i in range(count) {
        log(name + ": " + i)
    }
    return count
}

pid = process.spawn(worker, ["Worker-1", 10])
result = process.await(pid)  # → 10
```

---

## Process Isolation

Each process has:

| Isolation | Description |
|-----------|-------------|
| **Namespace** | Own variables, can't see parent's |
| **State** | Own module-level state |
| **Errors** | Crashes don't affect other processes |
| **Resources** | Own memory/CPU allocation |

```graphoid
counter = 0

pid = process.spawn(fn() {
    counter = 100  # This is a NEW variable, not parent's
    return counter
})

result = process.await(pid)  # → 100
print(counter)                # → 0 (unchanged)
```

---

## Process Communication

### Channels

Processes communicate via channels:

```graphoid
import "platform/process"
import "platform/channel"

# Create a channel
ch = channel.create()

# Producer process
producer = process.spawn(fn() {
    for i in range(10) {
        ch.send(i)
    }
    ch.close()
})

# Consumer process
consumer = process.spawn(fn() {
    total = 0
    for value in ch {
        total = total + value
    }
    return total
})

result = process.await(consumer)  # → 45
```

### Channel Operations

```graphoid
ch = channel.create()
ch = channel.create({ buffer: 10 })  # Buffered channel

ch.send(value)        # Send (blocks if unbuffered and no receiver)
value = ch.receive()  # Receive (blocks until value available)
ch.close()            # Close channel

# Non-blocking
ch.try_send(value)    # → true/false
value = ch.try_receive()  # → value or none

# Select from multiple channels
result = channel.select([ch1, ch2, ch3])
# → { channel: ch1, value: ... }
```

---

## Supervision

Supervisors manage process lifecycles:

```graphoid
import "platform/supervisor"

# Define a supervisor
sup = supervisor.create("game_supervisor", {
    strategy: :one_for_one  # Restart failed child only
})

# Add children
sup.add("game_loop", fn() {
    game.run()
}, { restart: :permanent })

sup.add("stats_collector", fn() {
    stats.collect_loop()
}, { restart: :transient })

# Start supervisor (starts all children)
sup.start()
```

### Restart Strategies

| Strategy | Behavior |
|----------|----------|
| `:one_for_one` | Restart only the failed child |
| `:one_for_all` | Restart all children if one fails |
| `:rest_for_one` | Restart failed child and all started after it |

### Child Restart Types

| Type | When to Restart |
|------|-----------------|
| `:permanent` | Always restart |
| `:transient` | Restart only if crashed (not normal exit) |
| `:temporary` | Never restart |

### Restart Limits

```graphoid
sup = supervisor.create("my_sup", {
    strategy: :one_for_one,
    max_restarts: 3,      # Max restarts...
    max_seconds: 5        # ...within this window
})
```

If limit exceeded, supervisor itself crashes (escalates to parent).

---

## Supervisor Trees ARE Graphs

The supervision hierarchy IS a graph — nodes are processes, edges are supervision relationships:

```graphoid
# The supervision tree emerges from the structure you define
root = supervisor.create("root")

# Game subsystem
game_sup = supervisor.create("game")
game_sup.add("game_loop", game.run)    # Edge: game → game_loop
game_sup.add("ai_engine", ai.run)       # Edge: game → ai_engine

# Network subsystem
net_sup = supervisor.create("network")
net_sup.add("listener", net.listen)     # Edge: network → listener
net_sup.add("broadcaster", net.broadcast)

# Build tree — creates edges
root.add_supervisor(game_sup)           # Edge: root → game
root.add_supervisor(net_sup)            # Edge: root → network

root.start()
```

The result is a graph you can traverse:

```graphoid
# Query the supervision graph
root.children                    # → [game, network]
game.children                    # → [game_loop, ai_engine]
ai_engine.supervisor             # → game

# Fault propagation follows edges
ai_engine.crash()                # Follows edge to notify 'game'
                                 # 'game' decides: restart 'ai_engine'
```

If `ai` crashes, only `ai` restarts. If `game` supervisor crashes, fault propagates to `root`, and the entire `game` subgraph restarts.

---

## Process Monitoring

### Watch a Process

```graphoid
pid = process.spawn(risky_work)

process.monitor(pid, fn(reason) {
    if reason == :normal {
        log("Process completed normally")
    } else {
        log.error("Process crashed", reason)
    }
})
```

### Link Processes

Linked processes crash together:

```graphoid
pid1 = process.spawn(fn() {
    pid2 = process.spawn(dangerous_work)
    process.link(pid2)  # If pid2 crashes, so do I

    # ... continue work ...
})
```

### Process Info

```graphoid
info = process.info(pid)
# → {
#     status: :running,
#     memory: 1024000,
#     started_at: timestamp,
#     current_function: "worker"
#   }
```

---

## Resource Limits

Constrain process resources:

```graphoid
pid = process.spawn(untrusted_code, [], {
    memory_limit: 10_000_000,  # 10MB
    timeout: 30_000,           # 30 seconds
    cpu_percent: 25            # 25% CPU
})
```

### What Happens at Limit

| Limit | Behavior |
|-------|----------|
| Memory | Process killed with `:out_of_memory` |
| Timeout | Process killed with `:timeout` |
| CPU | Process throttled (not killed) |

---

## Process Patterns

### Worker Pool

```graphoid
import "platform/process"
import "platform/pool"

# Create pool of 4 workers
pool = pool.create("workers", 4, fn(task) {
    process_task(task)
})

# Submit work
pool.submit(task1)
pool.submit(task2)
pool.submit(task3)

# Or with result
result = pool.submit_await(task4)
```

### Task Queue

```graphoid
import "platform/process"
import "platform/queue"

# Background job processor
queue = queue.create("jobs")

# Producer adds jobs
queue.push({ type: "email", to: "user@example.com" })
queue.push({ type: "report", id: 123 })

# Worker processes jobs
process.spawn(fn() {
    for job in queue {
        process_job(job)
    }
})
```

### Periodic Tasks

```graphoid
import "platform/process"

# Run every 60 seconds
process.periodic(60, fn() {
    cleanup_old_sessions()
})

# Run with initial delay
process.periodic(60, fn() {
    sync_stats()
}, { initial_delay: 10 })
```

---

## Error Handling

### Crash Isolation

```graphoid
pid = process.spawn(fn() {
    raise "Something went wrong"
})

# Main process continues
log("Main process still running")

# Can check result
result = process.await(pid)  # → raises the error
# Or
result = process.try_await(pid)  # → { status: :error, reason: "Something went wrong" }
```

### Graceful Shutdown

```graphoid
pid = process.spawn(fn() {
    process.on_shutdown(fn() {
        cleanup_resources()
        save_state()
    })

    # ... do work ...
})

# Later
process.shutdown(pid)  # Triggers on_shutdown callback
```

---

## Platform Integration

### With Monitor

```graphoid
import "platform/monitor"
import "platform/process"

# Processes report to monitor
pid = process.spawn(worker)
monitor.track_process(pid, "background_worker")

# View in status
status = platform.status()
# → { processes: [{ name: "background_worker", status: :running }] }
```

### With Reload

```graphoid
# Processes can be notified of reload
process.on(:reload, fn(module_name) {
    if module_name == "lib/worker" {
        # Restart worker with new code
        restart_workers()
    }
})
```

---

## Language Requirements

| Capability | Status | Notes |
|------------|--------|-------|
| Concurrency primitives | Phase 19 | Spawn, channels |
| Isolated namespaces | **Needed** | Process isolation |
| Memory limits | **Needed** | Resource control |
| Signal handling | **Needed** | Graceful shutdown |

### Feedback to Graphoid Roadmap

1. **Isolated execution context**: Create namespace that's separate from parent
2. **Memory limits**: Set memory ceiling for execution context
3. **Process registry**: Map names to process identifiers
4. **Crash propagation**: Link processes so crashes propagate

---

## Implementation Phases

### Phase 1: Basic Spawn

- `process.spawn(fn)` creates lightweight process
- `process.await(pid)` waits for completion
- Basic isolation (separate namespace)

**Milestone**: Can run code in separate process

### Phase 2: Channels

- `channel.create()` for communication
- `send()` and `receive()` operations
- Buffered channels

**Milestone**: Processes can communicate

### Phase 3: Monitoring

- `process.monitor(pid, callback)`
- `process.link(pid)`
- Process info inspection

**Milestone**: Know when processes fail

### Phase 4: Supervision

- Basic supervisor
- Restart strategies
- Restart limits

**Milestone**: Automatic restart on failure

### Phase 5: Supervisor Trees

- Nested supervisors
- `add_supervisor()` function
- Hierarchical restart

**Milestone**: Complex supervision hierarchies

### Phase 6: Resource Limits

- Memory limits
- Timeouts
- CPU throttling

**Milestone**: Constrained execution

### Phase 7: Patterns

- Worker pool
- Task queue
- Periodic tasks

**Milestone**: Common patterns available

---

## Open Questions

1. **Scheduling**: Cooperative or preemptive multitasking?

2. **Serialization**: Can closures be sent across processes?

3. **Distribution**: How do processes span multiple machines? (See Phase 24)

4. **Debugging**: How to debug multi-process applications?

---

## Related Documents

- [PHASE_19_CONCURRENCY.md](../PHASE_19_CONCURRENCY.md) — Underlying concurrency primitives
- [PLATFORM_RUNTIME.md](PLATFORM_RUNTIME.md) — Platform lifecycle
- [PLATFORM_MONITOR.md](PLATFORM_MONITOR.md) — Process monitoring integration
- [PHASE_24_DISTRIBUTED_EXECUTION.md](../PHASE_24_DISTRIBUTED_EXECUTION.md) — Distributed processes

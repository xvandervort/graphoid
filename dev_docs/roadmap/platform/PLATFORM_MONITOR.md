# Platform Monitor

**Status**: Design
**Priority**: 5 (Observability)
**Dependencies**: Platform Runtime, Platform Logger

---

## Overview

The Platform Monitor provides observability through **graph introspection**. The platform IS a graph — monitoring is just querying that graph for health, metrics, and relationships.

**Key Principles**:
- Monitoring is graph introspection
- Health propagates through edges
- Metrics are node properties
- Query the graph to understand the system

---

## Basic Usage

### Status Check

```graphoid
import "platform"

status = platform.status()
# → {
#     running: true,
#     uptime: 3600,
#     modules: 12,
#     memory: { used: 45000000, limit: 100000000 },
#     errors: 0
#   }
```

### From Command Line

```bash
graphoid status           # Quick status
graphoid status --full    # Detailed status
graphoid status --watch   # Continuous monitoring
```

---

## Health Checks

### Built-in Health

Platform automatically tracks:

| Metric | Description |
|--------|-------------|
| `running` | Is the application running? |
| `uptime` | Seconds since start |
| `memory.used` | Current memory usage |
| `memory.limit` | Configured memory limit |
| `modules.loaded` | Number of loaded modules |
| `errors.count` | Errors since start |
| `errors.recent` | Errors in last 5 minutes |

### Custom Health Checks

Register application-specific health checks:

```graphoid
import "platform/monitor"

monitor.health("database", fn() {
    if database.connected?() {
        return { status: :healthy }
    } else {
        return { status: :unhealthy, reason: "Connection lost" }
    }
})

monitor.health("game_state", fn() {
    if game.players.length() > 0 {
        return { status: :healthy, players: game.players.length() }
    } else {
        return { status: :degraded, reason: "No active players" }
    }
})
```

### Health Status Values

| Status | Meaning |
|--------|---------|
| `:healthy` | Everything working |
| `:degraded` | Working but not optimal |
| `:unhealthy` | Not working properly |

### Health Through Graph Traversal

Health status is a property on each node. Query the graph to understand system health:

```graphoid
# Each module is a node with health
platform.models.player.health       # → :healthy
platform.lib.database.health        # → :unhealthy

# Find all unhealthy modules
platform.unhealthy                  # → [lib/database]

# Health propagates through edges — if database is unhealthy,
# what else might be affected?
platform.lib.database.dependents    # → [lib/game, models/player]

# Aggregate health is graph-wide query
platform.health                     # → :degraded (worst of all nodes)
```

Overall status is the worst across all nodes — no separate aggregation needed.

---

## Metrics

### Built-in Metrics

Platform collects automatically:

```graphoid
metrics = monitor.metrics()
# → {
#     memory: { used: 45000000, peak: 52000000 },
#     cpu: { percent: 12.5 },
#     modules: { loaded: 12, reloaded: 3 },
#     requests: { total: 1500, errors: 2 },
#     uptime: 3600
#   }
```

### Custom Metrics

Track application-specific metrics:

```graphoid
import "platform/monitor"

# Counter — tracks occurrences
monitor.counter("games_played")
monitor.counter("games_won")

# In game code
fn end_game(result) {
    monitor.increment("games_played")
    if result == :won {
        monitor.increment("games_won")
    }
}

# Gauge — tracks current value
monitor.gauge("active_players", fn() {
    return players.length()
})

# Histogram — tracks distribution
monitor.histogram("game_duration")

fn end_game(result, duration) {
    monitor.record("game_duration", duration)
}
```

### Reading Metrics

```graphoid
games = monitor.get("games_played")     # → 150
win_rate = monitor.get("games_won") / monitor.get("games_played")

# Histogram stats
duration_stats = monitor.stats("game_duration")
# → { min: 30, max: 3600, avg: 450, p50: 400, p95: 1200, p99: 2500 }
```

---

## Alerting

### Define Alerts

```graphoid
import "platform/monitor"

monitor.alert("high_memory", {
    condition: fn() { monitor.get("memory.used") > 80000000 },
    message: "Memory usage above 80MB",
    severity: :warning
})

monitor.alert("no_players", {
    condition: fn() { monitor.get("active_players") == 0 },
    after: 300,  # Only alert if true for 5 minutes
    message: "No active players for 5 minutes",
    severity: :info
})

monitor.alert("database_down", {
    condition: fn() { monitor.health("database").status == :unhealthy },
    message: "Database connection lost",
    severity: :critical
})
```

### Alert Severity

| Severity | Meaning |
|----------|---------|
| `:info` | Informational, no action needed |
| `:warning` | Should investigate soon |
| `:error` | Problem affecting functionality |
| `:critical` | Immediate attention required |

### Alert Handlers

```graphoid
monitor.on_alert(fn(alert) {
    log.warn("Alert: " + alert.message)

    if alert.severity == :critical {
        # Could send notification, page on-call, etc.
        notify_admin(alert)
    }
})
```

### Alert Status

```graphoid
alerts = monitor.active_alerts()
# → [
#     { name: "high_memory", severity: :warning, since: timestamp }
#   ]
```

---

## Dashboard

### Console Dashboard

```bash
graphoid status --watch
```

```
┌─────────────────────────────────────────────────────────┐
│ Graphoid Platform - my_game                             │
├─────────────────────────────────────────────────────────┤
│ Status: RUNNING                    Uptime: 1h 23m 45s   │
├─────────────────────────────────────────────────────────┤
│ Memory: ████████░░░░░░░░ 45MB / 100MB                   │
│ CPU:    ██░░░░░░░░░░░░░░ 12%                            │
├─────────────────────────────────────────────────────────┤
│ Health Checks:                                          │
│   ✓ platform     healthy                                │
│   ✓ database     healthy                                │
│   ◐ game_state   degraded (no active players)          │
├─────────────────────────────────────────────────────────┤
│ Metrics:                                                │
│   games_played:   150                                   │
│   games_won:      87 (58%)                              │
│   active_players: 0                                     │
├─────────────────────────────────────────────────────────┤
│ Alerts: 1 active                                        │
│   ⚠ no_players - No active players for 5 minutes       │
└─────────────────────────────────────────────────────────┘
```

### Programmatic Dashboard

```graphoid
import "platform/monitor"

fn display_status() {
    status = monitor.full_status()
    print("=== Application Status ===")
    print("Uptime: " + format_duration(status.uptime))
    print("Memory: " + format_bytes(status.memory.used))
    print("Games: " + status.metrics.games_played)
    # ...
}
```

---

## Inspection (Graph Introspection)

### Module Inspection

The platform IS a graph. Inspection is just graph traversal:

```graphoid
# What modules are loaded? (nodes in the graph)
platform.nodes                          # → [models/player, models/room, lib/game, ...]

# Details about a module (node properties)
platform.models.player.path             # → "app/models/player.gr"
platform.models.player.loaded_at        # → timestamp
platform.models.player.reload_count     # → 2
platform.models.player.functions        # → [create, take_damage, is_alive]

# Relationships through edges
platform.models.player.depends_on       # → [models/room]
platform.models.player.dependents       # → [lib/game, lib/battle]

# Query patterns
for module in platform where module.reload_count > 0 {
    log(module.name + " has been reloaded " + module.reload_count + " times")
}
```

### State Inspection

```graphoid
# Inspect module-level state (for debugging)
state = monitor.inspect("models/player", "players")
# → { "Alice": <player graph>, "Bob": <player graph> }
```

### Call Tracing (Debug Mode)

```graphoid
monitor.trace("lib/game")  # Start tracing

# ... run some code ...

calls = monitor.trace_log("lib/game")
# → [
#     { fn: "start_game", args: ["Alice"], time: 0.5 },
#     { fn: "move_player", args: [:north], time: 0.1 },
#     ...
#   ]

monitor.untrace("lib/game")  # Stop tracing
```

---

## Configuration

```graphoid
# config/settings.gr

monitor = {
    enabled: true,

    # Collection intervals
    collect_interval: 10,       # Seconds between metric collection
    health_interval: 30,        # Seconds between health checks

    # Retention
    metrics_retention: 3600,    # Keep 1 hour of metrics

    # Overhead control
    tracing: false,             # Disable tracing in production
    detailed_memory: false      # Skip detailed memory breakdown
}
```

---

## Performance

### Overhead Budget

| Feature | Target Overhead |
|---------|-----------------|
| Basic metrics | < 1% CPU |
| Health checks | < 0.5% CPU |
| Full monitoring | < 3% CPU |
| Tracing (debug) | < 10% CPU |

### Lazy Collection

Metrics are collected on-demand or at intervals, not continuously:

```graphoid
# This doesn't run expensive collection
monitor.enabled?()  # → true

# This triggers collection
monitor.metrics()  # Collects current metrics
```

---

## Language Requirements

| Capability | Status | Notes |
|------------|--------|-------|
| Memory introspection | **Needed** | Current memory usage |
| Timer callbacks | **Needed** | Periodic collection |
| Module introspection | **Needed** | List functions, state |

### Feedback to Graphoid Roadmap

1. **Memory stats**: `runtime.memory()` → `{ used, limit, peak }`
2. **Module introspection**: `module.functions()`, `module.state()`
3. **Timers**: For periodic health checks and collection

---

## Implementation Phases

### Phase 1: Basic Status

- `platform.status()` with uptime, module count
- Memory usage (if available)
- Error count from logger

**Milestone**: Can see basic application status

### Phase 2: Health Checks

- Built-in platform health
- Custom health check registration
- Aggregate health status

**Milestone**: Health endpoint works

### Phase 3: Metrics

- Counter, gauge, histogram types
- Built-in platform metrics
- Custom metric registration

**Milestone**: Can track custom metrics

### Phase 4: Alerting

- Alert definition
- Condition evaluation
- Alert handlers

**Milestone**: Alerts fire on conditions

### Phase 5: Dashboard

- Console dashboard (`graphoid status --watch`)
- Continuous updates
- Visual formatting

**Milestone**: Visual monitoring

### Phase 6: Inspection

- Module inspection
- State inspection
- Call tracing (debug)

**Milestone**: Deep debugging support

---

## Open Questions

1. **External monitoring**: Export metrics to Prometheus/Grafana?

2. **Distributed monitoring**: How to monitor multi-process apps?

3. **Historical data**: Persist metrics across restarts?

4. **Sampling**: For high-frequency events, sample instead of count all?

---

## Related Documents

- [PLATFORM_RUNTIME.md](PLATFORM_RUNTIME.md) — Runtime status
- [PLATFORM_LOGGER.md](PLATFORM_LOGGER.md) — Error tracking
- [PLATFORM_LOADER.md](PLATFORM_LOADER.md) — Module information

# Platform Logger

**Status**: Design
**Priority**: 3 (Core Infrastructure)
**Dependencies**: Platform Runtime, File I/O

---

## Overview

The Platform Logger provides always-on, zero-configuration logging for Graphoid applications. It's available from the moment the platform starts, with no imports or setup required.

**Key Principles**:
- Always on — no setup required
- Convention over configuration — sensible defaults
- Available everywhere — no imports needed
- Configurable when needed — but not before

---

## Basic Usage

Available in any Graphoid code without imports:

```graphoid
log("Player entered room 5")
log.debug("Position: [3, 4]")
log.info("Game started")
log.warn("Health below 20%")
log.error("Failed to save", error)
```

That's it. No imports. No configuration. It just works.

---

## Log Levels

| Level | Method | Use For |
|-------|--------|---------|
| `debug` | `log.debug()` | Detailed diagnostic info |
| `info` | `log.info()` or `log()` | Normal operations |
| `warn` | `log.warn()` | Unexpected but handled situations |
| `error` | `log.error()` | Failures and exceptions |
| `fatal` | `log.fatal()` | Unrecoverable errors |

### Default Level

- Development: `debug` (show everything)
- Production: `info` (skip debug)

---

## Log Output

### Default Location

```
project/
└── log/
    ├── app.log           # Application logs
    └── platform.log      # Platform internal logs
```

Created automatically on first log write.

### Console Output

In development, logs also print to console:

```
[2024-01-15 10:23:45] INFO  Player entered room 5
[2024-01-15 10:23:46] DEBUG Position: [3, 4]
[2024-01-15 10:23:47] WARN  Health below 20%
```

---

## Log Format

### Default (Text)

```
[2024-01-15 10:23:45] INFO  models/player Player created: Alice
[2024-01-15 10:23:46] ERROR lib/game Failed to save game
  Error: Connection refused
  Stack:
    at save() in lib/game.gr:45
    at main() in app.gr:12
```

### JSON (For Production/Aggregation)

```json
{
  "timestamp": "2024-01-15T10:23:45Z",
  "level": "INFO",
  "module": "models/player",
  "message": "Player created: Alice",
  "data": { "player_name": "Alice" }
}
```

---

## Structured Logging

Pass additional data with log messages:

```graphoid
log.info("Player created", { name: player.name, health: player.health })
# → [10:23:45] INFO Player created name=Alice health=100

log.error("Save failed", error, { retry_count: 3 })
# → [10:23:45] ERROR Save failed retry_count=3
#     Error: Connection refused
```

---

## Configuration

### Via config/settings.gr

```graphoid
# config/settings.gr

log = {
    level: :debug,              # Minimum level to log
    format: :text,              # :text or :json
    destination: "log/app.log", # File path, :stdout, or :stderr
    console: true,              # Also print to console?
    rotate: :daily,             # :daily, :size, or :never
    max_size: "10mb",           # For :size rotation
    max_files: 5                # Keep N rotated files
}
```

### Environment Override

```bash
GRAPHOID_LOG_LEVEL=debug graphoid run
```

### Runtime Configuration

```graphoid
log.set_level(:debug)
log.set_destination("log/custom.log")
```

---

## Module Context (Graph Position)

Logs automatically include the source module — the logger knows where you are in the platform graph:

```graphoid
# In app/models/player.gr (a node in the platform graph)
log.info("Created player")
# → [10:23:45] INFO models/player Created player

# In app/lib/game.gr (another node in the graph)
log.info("Game started")
# → [10:23:45] INFO lib/game Game started
```

The module name comes from the node's position in the platform graph — no configuration needed.

---

## Error Logging

When logging errors, stack traces are included:

```graphoid
try {
    save_game(state)
} catch error {
    log.error("Failed to save game", error)
}
```

Output:
```
[10:23:45] ERROR lib/game Failed to save game
  Error: Connection refused
  Stack:
    at connect() in lib/database.gr:23
    at save_game() in lib/game.gr:45
    at main() in app.gr:12
```

---

## Platform vs Application Logs

| Log File | Contains |
|----------|----------|
| `log/app.log` | User application logs |
| `log/platform.log` | Platform internal logs (reload, lifecycle, etc.) |

Platform logs are separate so they don't clutter application logs.

### Viewing Platform Logs

```graphoid
# Platform logs its own operations
# [10:23:40] INFO platform Starting platform
# [10:23:41] INFO loader Discovered 5 modules
# [10:23:41] INFO loader Loaded models/player
# [10:23:45] INFO reload Reloading lib/game
```

---

## Log Rotation

### Daily Rotation (Default)

```
log/
├── app.log               # Current
├── app.2024-01-14.log    # Yesterday
├── app.2024-01-13.log    # Day before
└── app.2024-01-12.log    # Older
```

### Size-Based Rotation

```graphoid
# config/settings.gr
log = {
    rotate: :size,
    max_size: "10mb",
    max_files: 5
}
```

```
log/
├── app.log      # Current
├── app.1.log    # Previous
├── app.2.log    # Older
└── app.3.log    # Oldest
```

---

## Performance

### Lazy Evaluation

For expensive debug logs, use a function:

```graphoid
log.debug(fn() { "State: " + json.encode(large_state) })
# Function only called if debug level is enabled
```

### Buffering

Logs are buffered and flushed periodically:
- Every 100 messages, or
- Every 1 second, or
- On `log.flush()`, or
- On shutdown

```graphoid
log.flush()  # Force immediate write
```

---

## Testing

### Capturing Logs in Tests

```graphoid
# In tests
import "platform/logger"

describe "Game" {
    it "logs when player dies" {
        logger.capture()  # Start capturing

        player.die()

        logs = logger.captured()
        expect(logs).to_include("Player died")

        logger.release()  # Stop capturing
    }
}
```

### Disabling Logs in Tests

```graphoid
# In test setup
log.set_level(:fatal)  # Only show fatal errors
```

---

## Implementation

### Built-in Global

The `log` function is injected by the platform runtime before user code runs:

```graphoid
# Conceptually (done by runtime)
log = platform.create_logger()

# Then user code runs with log available
```

### Logger Implementation

```graphoid
# platform/logger.gr (conceptual)

fn create(config) {
    logger = graph { type: :logger }
    logger.level = config.level or :info
    logger.destination = config.destination or "log/app.log"
    logger.format = config.format or :text

    return logger
}

fn logger.info(msg, data) {
    if level_enabled?(:info) {
        write_log(:info, msg, data)
    }
}

# ... etc
```

---

## Language Requirements

| Capability | Status | Notes |
|------------|--------|-------|
| File I/O | Exists | Writing logs |
| Timestamps | Exists | Via time module |
| String formatting | Exists | Log message formatting |
| Stack traces | **Needed** | For error logging |
| Module context | **Needed** | Know which module is logging |

### Feedback to Graphoid Roadmap

1. **Stack trace access**: `error.stack()` or similar
2. **Current module**: `__MODULE__` or `module.current()`

---

## Implementation Phases

### Phase 1: Basic Logging

- `log()` function available globally
- Write to `log/app.log`
- Console output
- Levels: info, warn, error

**Milestone**: Can see what's happening

### Phase 2: Configuration

- Load settings from `config/settings.gr`
- Level filtering
- Format selection (text/json)

**Milestone**: Configurable logging

### Phase 3: Structured Logging

- Additional data parameters
- Error with stack trace
- JSON format support

**Milestone**: Production-ready logs

### Phase 4: Rotation

- Daily rotation
- Size-based rotation
- Max file limits

**Milestone**: Long-running applications

### Phase 5: Testing Support

- Log capturing
- Level override
- Test isolation

**Milestone**: Testable logging

---

## Open Questions

1. **Async logging?** Should log writes be async to avoid blocking?

2. **Remote logging?** Send logs to external service (future)?

3. **Correlation IDs?** For tracing requests across components?

4. **Log context?** `log.with({ user: "alice" }).info("Action")` pattern?

---

## Related Documents

- [PLATFORM_RUNTIME.md](PLATFORM_RUNTIME.md) — Provides the logging global
- [PLATFORM_LOADER.md](PLATFORM_LOADER.md) — Module context for logs

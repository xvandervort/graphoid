# Platform Component Structure

**Status**: Design
**Purpose**: Define how platform components are organized and structured

---

## Overview

This document defines the standard structure for platform components. Following this structure ensures:

- Consistency across components
- Compatibility with the package manager (Phase 21)
- Clear interfaces between components
- Predictable behavior for platform users

---

## Component Definition

A **platform component** is a self-contained module that:

1. Provides a specific capability (logging, monitoring, etc.)
2. Has a defined public API
3. May depend on other components
4. Integrates with the platform lifecycle

---

## Directory Structure

Each component follows this structure:

```
platform/
└── <component>/
    ├── mod.gr              # Main entry point, exports public API
    ├── internal/           # Private implementation (optional)
    │   ├── helpers.gr
    │   └── ...
    ├── tests/              # Component tests
    │   └── spec/
    │       └── <component>.spec.gr
    └── README.md           # Component documentation (optional)
```

### Example: Logger Component

```
platform/
└── logger/
    ├── mod.gr              # Exports: log, log.debug, log.info, etc.
    ├── internal/
    │   ├── formatter.gr    # Text/JSON formatting
    │   ├── writer.gr       # File/console output
    │   └── rotation.gr     # Log rotation logic
    └── tests/
        └── spec/
            └── logger.spec.gr
```

---

## Module Entry Point (mod.gr)

The `mod.gr` file is the public interface:

```graphoid
# platform/logger/mod.gr

# Dependencies on other components
import "platform/runtime" as runtime

# Internal implementation
import "./internal/formatter"
import "./internal/writer"

# Configuration (from platform config)
config = runtime.config("logger", {
    level: :info,
    format: :text,
    destination: "log/app.log"
})

# Public API
fn log(message) {
    log_at_level(:info, message)
}

fn debug(message) { log_at_level(:debug, message) }
fn info(message) { log_at_level(:info, message) }
fn warn(message) { log_at_level(:warn, message) }
fn error(message, err) { log_at_level(:error, message, err) }

# Lifecycle hooks (called by platform)
fn on_platform_start() {
    writer.open(config.destination)
}

fn on_platform_stop() {
    writer.flush()
    writer.close()
}

# Export public interface
export { log, debug, info, warn, error }
```

---

## Component Manifest

Each component declares its metadata:

```graphoid
# platform/logger/mod.gr (at top)

manifest = {
    name: "logger",
    version: "0.1.0",
    description: "Always-on logging for platform applications",

    # Dependencies on other platform components
    dependencies: ["runtime"],

    # Optional dependencies (used if available)
    optional_dependencies: ["monitor"],

    # Language features required
    requires: {
        graphoid: ">=0.1.0",
        features: []  # No special features needed
    },

    # Lifecycle participation
    lifecycle: {
        on_start: true,
        on_stop: true,
        on_reload: false
    }
}
```

---

## Public API Convention

### Naming

- Functions use `snake_case`
- Constants use `UPPER_CASE`
- Internal functions prefixed with `_` (not exported)

### Documentation

```graphoid
# Public functions should have doc comments

## Logs a message at the specified level.
##
## @param level - The log level (:debug, :info, :warn, :error)
## @param message - The message to log
## @param data - Optional structured data
##
## @example
##   log.info("User logged in", { user_id: 123 })
##
fn log_at_level(level, message, data) {
    # ...
}
```

---

## Lifecycle Integration

Components can participate in platform lifecycle:

```graphoid
# Called when platform starts (after all components loaded)
fn on_platform_start() {
    # Initialize resources
}

# Called when platform stops
fn on_platform_stop() {
    # Cleanup resources
}

# Called when this component is reloaded
fn on_before_reload() {
    # Return state to preserve
    return { ... }
}

fn on_after_reload(preserved) {
    # Restore state
}

# Called when any module reloads (optional)
fn on_module_reload(module_name) {
    # React to other module changes
}
```

---

## Configuration

Components read configuration from the platform:

```graphoid
import "platform/runtime" as runtime

# Get component config with defaults
config = runtime.config("logger", {
    level: :info,       # Default if not specified
    format: :text,
    destination: "log/app.log"
})

# User overrides in config/settings.gr:
# logger = {
#     level: :debug,
#     format: :json
# }
```

---

## Inter-Component Dependencies

### Required Dependencies

```graphoid
import "platform/runtime" as runtime  # Must exist
```

If required dependency is missing, platform fails to start with clear error.

### Optional Dependencies

```graphoid
# Check if available
if platform.has_component?("monitor") {
    import "platform/monitor" as monitor
    monitor.register_health_check("logger", health_check_fn)
}
```

### Circular Dependency Prevention

Components must not have circular dependencies. The platform validates this at startup.

---

## Testing

### Test Structure

```graphoid
# platform/logger/tests/spec/logger.spec.gr

import "platform/logger"
import "spec"

describe "Logger" {
    describe "log levels" {
        it "logs info messages by default" {
            # ...
        }

        it "filters debug messages when level is info" {
            # ...
        }
    }

    describe "formatting" {
        it "formats text output correctly" {
            # ...
        }
    }
}
```

### Running Tests

```bash
graphoid test platform/logger
```

---

## Versioning

Components follow semantic versioning:

- **MAJOR**: Breaking API changes
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes

```graphoid
manifest = {
    version: "1.2.3"
}
```

---

## Compatibility with Package Manager

Platform components are structured to work with the package manager (Phase 21):

| Platform Structure | Package Equivalent |
|-------------------|-------------------|
| `platform/logger/mod.gr` | `src/mod.gr` |
| `platform/logger/internal/` | `src/internal/` |
| `platform/logger/tests/` | `tests/` |
| Component manifest | `package.gr` manifest |

When the package manager exists, platform components can be:
- Versioned independently
- Published to package registry
- Updated without full platform update

---

## Creating a New Component

### Step 1: Create Structure

```bash
mkdir -p platform/mycomponent/internal
mkdir -p platform/mycomponent/tests/spec
touch platform/mycomponent/mod.gr
```

### Step 2: Define Manifest

```graphoid
# platform/mycomponent/mod.gr

manifest = {
    name: "mycomponent",
    version: "0.1.0",
    description: "What this component does",
    dependencies: ["runtime"],
    requires: { graphoid: ">=0.1.0" }
}
```

### Step 3: Implement Public API

```graphoid
# Public functions
fn do_something() {
    # ...
}

export { do_something }
```

### Step 4: Add Tests

```graphoid
# platform/mycomponent/tests/spec/mycomponent.spec.gr

describe "MyComponent" {
    it "does something" {
        # ...
    }
}
```

### Step 5: Register with Platform

Add to platform initialization (in `platform/core.gr`):

```graphoid
components = [
    "runtime",
    "loader",
    "logger",
    "mycomponent"  # New component
]
```

---

## Open Questions

1. **Export syntax**: Does Graphoid have `export`? If not, how do we define public API?

2. **Import aliases**: Does `import "x" as y` work?

3. **Relative imports**: Does `import "./internal/foo"` work?

4. **Manifest format**: Inline in mod.gr or separate file?

---

## Related Documents

- [GRAPHOID_PLATFORM.md](GRAPHOID_PLATFORM.md) — Platform overview
- [PHASE_21_PACKAGE_MANAGER.md](../PHASE_21_PACKAGE_MANAGER.md) — Package structure alignment

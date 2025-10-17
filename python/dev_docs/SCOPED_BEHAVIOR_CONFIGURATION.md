# Scoped Behavior Configuration System

## Overview
A system to allow configuration of behaviors at different scope levels (file, function, block) to control how Glang handles various operations, particularly around none values, precision, and type strictness.

## Motivation
- Users need control over default behaviors (e.g., how `none` is handled in calculations)
- Different problem domains require different defaults (data science vs finance vs systems)
- Configuration should be explicit and visible, not hidden magic
- Scopes should be composable and override hierarchically

## Proposed Design

### 1. File-Level Configuration
```glang
# Option A: Decorator-style at top of file
@configure skip_none: false
@configure precision: 2

# Option B: Configuration block at top of file
configure {
    skip_none: false      # Don't skip none in calculations
    precision: 2          # Default decimal precision
    strict_types: true    # Enforce strict type checking
    none_to_zero: false   # Don't auto-convert none to 0
}

# All code in file uses these configurations
data = [1, 2, none, 4]
data.mean()  # Would error on none instead of skipping
```

### 2. Function-Level Configuration
```glang
# Option A: Decorator on function
@configure skip_none: false
func calculate_average(values) {
    return values.sum() / values.size()  # Errors if none present
}

# Option B: Inline with function declaration
func calculate_average(values) with [strict_none, precision: 4] {
    return values.sum() / values.size()
}

# Option C: Configuration block in function
func process_data(values) {
    configure {
        skip_none: false
        precision: 4
    }
    # Function body uses these settings
}
```

### 3. Block-Level Configuration
```glang
# Similar to existing precision blocks
behavior skip_none: false {
    # Everything in this block treats none strictly
    result = data.mean()  # Errors if data contains none
}

# Multiple behaviors in one block
with behaviors [strict_none, round_to_int, precision: 2] {
    # All operations here use these behaviors
    processed = data.map("normalize").filter("valid")
}

# Temporary override
data = [1, 2, none, 4, none, 6]
result1 = data.mean()  # Uses file/default config

behavior skip_none: true {
    result2 = data.mean()  # Temporarily skips none
}

result3 = data.mean()  # Back to file/default config
```

### 4. Configuration Inheritance

#### Scope Resolution Order
1. Block-level configuration (most specific)
2. Function-level configuration
3. File-level configuration
4. System defaults (least specific)

#### Example
```glang
# File level
configure { skip_none: false }  # Strict by default

func analyze_data(values) with [skip_none: true] {
    # Function overrides file-level
    mean1 = values.mean()  # Skips none

    behavior skip_none: false {
        # Block overrides function-level
        mean2 = values.mean()  # Errors on none
    }

    mean3 = values.mean()  # Back to function-level (skips none)
}

# Outside function - uses file-level
global_mean = data.mean()  # Errors on none
```

## Standard Behavior Categories

### 1. None Handling
- `skip_none`: Skip none values in calculations (default: true)
- `none_to_zero`: Auto-convert none to 0
- `none_to_empty`: Auto-convert none to empty string
- `strict_none`: Error on any none operation

### 2. Numeric Precision
- `precision`: Decimal places (default: system)
- `round_mode`: Rounding mode (up, down, nearest)
- `integer_only`: Force integer arithmetic

### 3. Type Strictness
- `strict_types`: No implicit conversions
- `allow_coercion`: Allow type coercion
- `warn_on_conversion`: Warn but allow conversions

### 4. Collection Behaviors
- `immutable_default`: Collections are immutable by default
- `strict_bounds`: Error on out-of-bounds access
- `auto_grow`: Allow collections to auto-expand

## Implementation Approach

### Phase 1: Parser Support
- Add configuration syntax to parser
- Create configuration AST nodes
- Validate configuration keys

### Phase 2: Configuration Context
- Create configuration stack for scope management
- Implement inheritance/override logic
- Add configuration to execution context

### Phase 3: Behavior Integration
- Modify operations to check configuration
- Update collection methods to respect configuration
- Add configuration query methods

### Phase 4: REPL and Tooling
- `/config` command to show current configuration
- Syntax highlighting for configuration blocks
- Configuration validation and warnings

## Benefits

1. **Explicit Control**: Users can see and control all behaviors
2. **Domain Flexibility**: Different defaults for different problem domains
3. **Gradual Adoption**: Start with defaults, configure as needed
4. **Testing**: Can test same code with different configurations
5. **Documentation**: Configuration serves as documentation of assumptions

## Example Use Cases

### Data Science Configuration
```glang
configure {
    skip_none: true        # Handle missing data gracefully
    precision: 6           # Higher precision for calculations
    auto_impute: mean      # Auto-impute missing values
}
```

### Financial Configuration
```glang
configure {
    skip_none: false       # Never ignore missing data
    precision: 2           # Exactly 2 decimal places
    rounding: banker       # Banker's rounding
    strict_types: true     # No implicit conversions
}
```

### Systems Programming Configuration
```glang
configure {
    strict_types: true     # No implicit conversions
    strict_bounds: true    # Array bounds checking
    integer_only: true     # No floating point
    immutable_default: true # Safer defaults
}
```

## Open Questions

1. Should configuration be compile-time or runtime?
2. How to handle configuration conflicts?
3. Should we allow custom user-defined behaviors?
4. How to make configuration visible in error messages?
5. Should configuration affect parsing or just execution?

## Timeline

- **Phase 1** (1 month): Design finalization and parser support
- **Phase 2** (2 months): Core implementation and context management
- **Phase 3** (1 month): Integration with existing behaviors
- **Phase 4** (2 weeks): REPL and tooling support

## Related Work
- Python's `__future__` imports for behavior changes
- Perl's `use strict` and `use warnings` pragmas
- Ruby's `$SAFE` levels
- TypeScript's `tsconfig.json` compiler options
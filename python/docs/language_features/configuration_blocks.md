# Configuration Blocks

**Status**: ✅ Basic syntax implemented (Phase 1)
**Full Behavior Integration**: 🔄 Coming in Phase 2

Configuration blocks provide language-level control over default behaviors and settings at different scope levels. This revolutionary feature allows you to explicitly configure how Glang handles various operations like `none` values, precision, and type strictness.

## Overview

Configuration blocks solve the critical problem of **explicit vs implicit behavior**. Instead of hidden magic, all behavior configuration is visible and scoped:

```glang
# File-level configuration - applies to entire file
configure { skip_none: false }

# Block-level configuration - applies only within the block
configure { skip_none: true } {
    data = [1, 2, none, 4]
    data.mean()  # Would skip none due to explicit configuration
}
```

## Syntax

### File-Level Configuration
```glang
configure { key: value, key2: value2 }

# Continue with normal code
string message = "Configuration applied to entire file"
```

### Block-Level Configuration
```glang
configure { key: value } {
    # Code here uses the specified configuration
    # All operations respect these settings
}
```

### Multiple Settings
```glang
configure {
    skip_none: false,
    decimal_places: 2,
    strict_types: true
} {
    # All three settings apply within this block
}
```

## Standard Configuration Categories

### 1. None Handling
Controls how `none` values are processed in operations:

```glang
configure { skip_none: false } {
    data = [1, 2, none, 4]
    result = data.mean()  # Error: cannot process none
}

configure { skip_none: true } {
    data = [1, 2, none, 4]
    result = data.mean()  # Result: 2.33 (none skipped)
}
```

**Available Settings:**
- `skip_none: true/false` - Skip none values in calculations (default: true)
- `none_to_zero: true/false` - Auto-convert none to 0
- `none_to_empty: true/false` - Auto-convert none to empty string
- `strict_none: true/false` - Error on any none operation

### 2. Numeric Precision
Controls decimal precision for calculations:

```glang
configure { decimal_places: 2 } {
    price = 19.99
    tax = price * 0.085     # Result: 1.70 (exactly 2 decimal places)
    total = price + tax     # Result: 21.69 (exactly 2 decimal places)
}
```

**Available Settings:**
- `decimal_places: N` - Number of decimal places
- `round_mode: "up"/"down"/"nearest"` - Rounding behavior
- `integer_only: true/false` - Force integer arithmetic

### 3. Type Strictness
Controls type conversion and validation:

```glang
configure { strict_types: true } {
    # No implicit conversions allowed
    result = "5" + 3  # Error: cannot add string and number
}

configure { allow_coercion: true } {
    # Automatic type conversion permitted
    result = "5" + 3  # Result: "53" or 8 (depending on implementation)
}
```

**Available Settings:**
- `strict_types: true/false` - No implicit conversions
- `allow_coercion: true/false` - Allow type coercion
- `warn_on_conversion: true/false` - Warn but allow conversions

### 4. Collection Behaviors
Controls how collections behave:

```glang
configure { strict_bounds: true } {
    items = [1, 2, 3]
    value = items[5]  # Error: index out of bounds
}

configure { auto_grow: true } {
    items = [1, 2, 3]
    items[5] = 42     # Auto-expands list with none values
}
```

**Available Settings:**
- `immutable_default: true/false` - Collections immutable by default
- `strict_bounds: true/false` - Error on out-of-bounds access
- `auto_grow: true/false` - Allow collections to auto-expand

## Configuration Inheritance

Configuration follows a clear hierarchy with inner scopes overriding outer scopes:

1. **Block-level configuration** (most specific)
2. **Function-level configuration** *(coming in Phase 2)*
3. **File-level configuration**
4. **System defaults** (least specific)

### Example: Nested Configuration
```glang
# File-level default
configure { skip_none: false }

func analyze_data(values) {
    # Function inherits file-level: skip_none: false
    mean1 = values.mean()  # Errors on none

    configure { skip_none: true } {
        # Block overrides function-level
        mean2 = values.mean()  # Skips none
    }

    # Back to function-level (file-level): skip_none: false
    mean3 = values.mean()  # Errors on none again
}

# File-level still applies
global_mean = data.mean()  # Errors on none
```

## Domain-Specific Examples

### Data Science Configuration
```glang
configure {
    skip_none: true,        # Handle missing data gracefully
    decimal_places: 6,      # Higher precision for calculations
    auto_impute: "mean"     # Auto-impute missing values
}

# All data operations use these settings
dataset = load_csv("data.csv")
cleaned = dataset.filter("valid").map("normalize")
statistics = cleaned.describe()
```

### Financial Configuration
```glang
configure {
    skip_none: false,       # Never ignore missing data
    decimal_places: 2,      # Exactly 2 decimal places
    rounding: "banker",     # Banker's rounding
    strict_types: true      # No implicit conversions
}

# All financial calculations use exact precision
price = 19.99
tax_rate = 0.085
tax = price * tax_rate      # Exactly 1.70
total = price + tax         # Exactly 21.69
```

### Systems Programming Configuration
```glang
configure {
    strict_types: true,     # No implicit conversions
    strict_bounds: true,    # Array bounds checking
    integer_only: true,     # No floating point
    immutable_default: true # Safer defaults
}

# All operations use strict checking
buffer = create_buffer(1024)
process_data(buffer, strict_validation=true)
```

## Implementation Status

### ✅ Phase 1: Parser Support (COMPLETED)
- ✅ Configuration syntax parsing
- ✅ AST node creation (`ConfigurationBlock`)
- ✅ Keyword registration (`configure`)
- ✅ Basic execution infrastructure

### 🔄 Phase 2: Configuration Context (PLANNED)
- Configuration stack for scope management
- Inheritance/override logic implementation
- Integration with execution context
- Actual behavior enforcement

### 🔄 Phase 3: Behavior Integration (PLANNED)
- Modify operations to respect configuration
- Update collection methods with configuration awareness
- Add configuration query methods
- Performance optimization

### 🔄 Phase 4: REPL and Tooling (PLANNED)
- `/config` command to show current configuration
- Syntax highlighting for configuration blocks
- Configuration validation and warnings

## Benefits

1. **Explicit Control**: All behaviors are visible and configurable
2. **Domain Flexibility**: Different defaults for different problem domains
3. **Gradual Adoption**: Start with defaults, configure as needed
4. **Testing**: Test same code with different configurations
5. **Documentation**: Configuration serves as executable documentation

## Error Handling

Configuration blocks provide clear error messages for invalid settings:

```glang
configure { invalid_setting: true } {
    # Error: Unknown configuration key 'invalid_setting'
    # Available keys: skip_none, decimal_places, strict_types, ...
}

configure { decimal_places: "high" } {
    # Error: Configuration 'decimal_places' must be a number, got string
}
```

## Future Enhancements

- **Custom Behaviors**: User-defined configuration functions
- **Configuration Profiles**: Predefined setting collections
- **Runtime Configuration**: Dynamic configuration changes
- **Configuration Inheritance**: Cross-file configuration sharing

---

*Configuration blocks represent Glang's commitment to explicit, visible behavior control instead of hidden magic.*
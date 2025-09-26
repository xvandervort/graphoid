# Enhanced Behavior System Specification

*Comprehensive specification for Enhanced Behavior System enhancements - January 2025*

## Overview

Building on Glang's existing intrinsic behavior system, these enhancements provide more powerful, flexible, and user-friendly behavior management capabilities. All enhancements maintain backward compatibility with existing behavior functionality.

## Current System (Baseline)

```glang
# Existing functionality (works today)
temperatures = [98.6, none, 102.5]
temperatures.add_rule("none_to_zero")
temperatures.add_rule("validate_range", 95, 105)
temperatures.add_rule("positive")

# Management methods
temperatures.has_rule("none_to_zero")    # true
temperatures.get_rules()                 # ["none_to_zero", "validate_range", "positive"]
temperatures.remove_rule("positive")
temperatures.clear_rules()
```

## Enhancement Priorities

### 1. **✅ CRITICAL BUG FIX: nil → none References** (COMPLETED)

**Problem**: Throughout the behavior system, references used `nil` instead of Glang's actual `none` keyword.

**Solution**: Global find/replace completed across entire codebase - all behavior references now correctly use `none_to_zero`, `none_to_empty`, etc.

### 2. **Configurable None Conversion Behaviors**

**Problem**: `none.to_string()`, `none.to_number()`, etc. currently throw disruptive errors instead of gracefully handling conversions.

**Solution**: User-configurable none conversion policies that make none a "cooperative" value rather than a "dangerous" one.

```glang
# Default configuration (graceful defaults)
configure {
    none_conversions: {
        to_string: "empty_string",    # none.to_string() → ""
        to_number: "zero",            # none.to_number() → 0
        to_bool: "false"              # none.to_bool() → false
    }
}

# Domain-specific configurations
# Financial context (explicit handling)
configure { none_conversions: { to_number: "error" } }

# Text processing context
configure { none_conversions: { to_string: "none_literal" } }  # → "none"

# Medical context (safe defaults)
configure { none_conversions: { to_number: "zero", to_string: "empty_string" } }
```

**Benefits**:
- **Graceful Degradation**: No disruptive exceptions for reasonable operations
- **Context Awareness**: Different domains can configure appropriate none handling
- **User Control**: Explicit choice between error-throwing and graceful conversion
- **Backward Compatible**: Can still configure error-throwing behavior when needed

### 3. **ORIGINAL BUG FIX: nil → none References** (MOVED TO COMPLETED)

**Problem**: Throughout the behavior system, references use `nil` instead of Glang's actual `none` keyword.

**Current (incorrect):**
```glang
temperatures.add_rule("nil_to_zero")     # ❌ Wrong - Glang uses 'none'
```

**Fixed (correct):**
```glang
temperatures.add_rule("none_to_zero")    # ✅ Proper Glang syntax
```

**Affected Files**:
- `/src/glang/behaviors.py` - Core behavior definitions
- `/samples/behaviors_demo.gr` - Example code
- `/docs/GLANG_CHEAT_SHEET.md` - Documentation
- All behavior-related test files

**Action Required**: Global find/replace of "nil_to_zero" → "none_to_zero" and "nil_to_empty" → "none_to_empty"

### 3. **Generic Mapping Behavior System**

**Problem**: Current `map_colors` behavior is too specific and inflexible.

**Current (limited):**
```glang
colors.add_rule("map_colors")  # Only works for predefined color names
```

**Enhanced (generic):**
```glang
# Generic value mapping with explicit mappings
colors.add_mapping_rule({
    "red": 1,
    "green": 2,
    "blue": 3,
    "yellow": 4,
    default: 0  # Fallback for unmapped values
})

# Employee to department mapping
employees.add_mapping_rule({
    "walter": "HR",
    "james": "IT",
    "emily": "Admin",
    default: "Unknown"
})

# ASCII to UTF-8 character mapping
ascii_chars.add_mapping_rule({
    "a": "α",
    "b": "β",
    "g": "γ"
})
```

**API Design:**
- `container.add_mapping_rule(mapping_dict)` - Apply generic mapping
- Support for `default` key as fallback value
- Works with any data types (string→number, string→string, etc.)

### 4. **Custom Function Behaviors**

**Capability**: Attach user-defined functions as behaviors.

```glang
# User-defined transformation function
func normalize_temperature(value) {
    if value < 95 { return 95 }
    if value > 105 { return 105 }
    return value
}

# Attach custom function as behavior
temperatures.add_custom_rule(normalize_temperature)

# Lambda functions as behaviors
scores.add_custom_rule(x => x > 100 ? 100 : x)  # Cap at 100

# Email validation behavior
func validate_email(value) {
    if value.contains("@") and value.contains(".") {
        return value
    }
    return "invalid@example.com"
}

emails.add_custom_rule(validate_email)
```

**API Design:**
- `container.add_custom_rule(function)` - Attach function as behavior
- Functions must take single parameter and return transformed value
- Support both named functions and lambda expressions

### 5. **Conditional Behavior System**

**Capability**: Apply behaviors only when specific conditions are met.

```glang
# Context-aware string processing
user_data.add_conditional_rule(
    condition: x => x.type() == "string",
    transform: x => x.trim().lowercase()
)

# Conditional numeric clamping
scores.add_conditional_rule(
    condition: x => x > 100,
    transform: x => 100
)

# Multiple condition types
financial_data.add_conditional_rule(
    condition: x => x.type() == "number" and x > 0,
    transform: x => x.round_to_int(),
    on_fail: x => 0  # What to do if condition fails
)
```

**API Design:**
- `container.add_conditional_rule(condition: lambda, transform: lambda)`
- Optional `on_fail` parameter for handling condition failures
- Conditions return boolean, transforms return new value

### 6. **Ruleset System (Declarative Bundle Application)**

**Capability**: Create reusable behavior bundles with clean declarative syntax.

```glang
# Declarative ruleset creation (uses existing multiline list support)
data_cleaning = Rules[
    :none_to_zero,
    :validate_range[min: 60, max: 200],
    :positive_only,
    :round_to_integers,
    custom_sanitizer()
]

# Reusable across multiple datasets
temperatures.add_rules(data_cleaning)
blood_pressure.add_rules(data_cleaning)
heart_rate.add_rules(data_cleaning)

# Medical validation ruleset
medical_rules = Rules[
    :none_to_zero,
    :validate_range[min: 60, max: 200],  # Blood pressure range
    custom_medical_validator()
]

# Financial data ruleset
financial_rules = Rules[
    :none_to_zero,
    :positive_only,
    :round_to_cents,
    :validate_range[min: 0, max: 1000000]
]
```

**Syntax Specifications:**
- **Keyword Parameters**: `:rule_name[key: value, key2: value2]`
- **Symbol Syntax**: `:rule_name` for built-in rules
- **Function Calls**: `custom_function()` for user-defined behaviors
- **Multiline Support**: Leverages existing list syntax
- **Square Brackets**: `Rules[...]` for collection-like syntax

**API Design:**
- `Rules[rule_list]` - Create ruleset from declarative list
- `container.add_rules(ruleset)` - Apply entire ruleset at once
- `ruleset.add_rule(rule)` - Add rule to existing ruleset
- `ruleset.get_rules()` - List all rules in ruleset

**Implementation Notes:**
- Rules are graph nodes within a ruleset graph container
- More efficient than individual rule application
- Enables rule composition and reuse
- Foundation for future behavior inheritance system

## Behavior Inheritance (DEFERRED)

**Status**: Deferred until graph-based inheritance is implemented.

**Rationale**: Behavior inheritance requires the planned graph inheritance infrastructure (`CustomTree from BinaryTree` syntax). Without graph composition mechanisms, behavior propagation cannot be implemented cleanly.

**Future Design** (when graph inheritance exists):
```glang
# Future: behavior inheritance through graph relationships
parent_dataset.add_rules(data_cleaning)
# All child graphs automatically inherit behaviors
```

## Implementation Order

1. ✅ **Fix nil→none references** (critical bug - affects all behavior code) - **COMPLETED**
2. ✅ **Configurable none conversion behaviors** (graceful none handling) - **COMPLETED**
3. **Generic mapping system** (replaces overly specific map_colors)
4. **Custom function behaviors** (user-defined transformations)
5. **Conditional behaviors** (context-aware rule application)
6. **Ruleset system** (declarative bundle interface)

## Backward Compatibility

All enhancements maintain full backward compatibility:
- Existing `add_rule()`, `remove_rule()`, `has_rule()` methods unchanged
- Current behavior names continue to work (after nil→none fix)
- New functionality additive, not replacing

## Benefits

- **Cleaner Syntax**: Declarative rulesets vs procedural rule addition
- **Reusability**: Rulesets can be applied across multiple datasets
- **Flexibility**: Custom functions and conditional logic
- **Efficiency**: Bundle application more performant than individual rules
- **Foundation**: Prepares infrastructure for future graph inheritance

---

**Status**: Specification complete, ready for implementation
**Dependencies**: None (uses existing Glang capabilities)
**Target**: Enhanced Behavior System milestone in Primary Roadmap
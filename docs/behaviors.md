# Glang Behavior System

The behavior system provides a powerful way to transform and validate data in Glang containers (lists, hashes, and future graphs) without adding language complexity.

## Overview

Behaviors are composable transformations that can:
- Transform values (e.g., nil → 0, "green" → 2)
- Validate and constrain data (e.g., clamp to range)
- Handle missing or invalid data gracefully
- Create domain-specific logic for your application

## Quick Start

```python
from glang.behaviors import BehaviorPipeline
from glang.execution.values import NumberValue, NoneValue, ListValue

# Create a pipeline for sensor data
sensor_pipeline = BehaviorPipeline()
sensor_pipeline.add("nil_to_zero")           # Handle missing readings
sensor_pipeline.add("validate_range", -50, 150)  # Temperature range
sensor_pipeline.add("round_to_int")          # Round for display

# Apply to a single value
result = sensor_pipeline.apply(NoneValue())  # Returns NumberValue(0)

# Apply to a list
readings = ListValue([NumberValue(23.7), NoneValue(), NumberValue(200)])
validated = sensor_pipeline.apply_to_list(readings)
# Result: [24, 0, 150]
```

## Standard Behaviors

### nil_to_zero
Converts nil/none values to 0. Useful for missing numeric data.
```python
pipeline.add("nil_to_zero")
```

### nil_to_empty
Converts nil/none values to empty string. Useful for missing text data.
```python
pipeline.add("nil_to_empty")
```

### validate_range(min, max)
Clamps numeric values to a specified range.
```python
pipeline.add("validate_range", 0, 100)  # Clamp to 0-100
```

### map_colors
Maps color name strings to numeric values.
- "red" → 1, "green" → 2, "blue" → 3, "yellow" → 4
- "black" → 0, "white" → 5
```python
pipeline.add("map_colors")
```

### uppercase / lowercase
Converts string case.
```python
pipeline.add("uppercase")  # "hello" → "HELLO"
pipeline.add("lowercase")  # "WORLD" → "world"
```

### round_to_int
Rounds numeric values to nearest integer.
```python
pipeline.add("round_to_int")  # 3.7 → 4
```

### positive
Ensures numeric values are positive (converts negative to absolute value).
```python
pipeline.add("positive")  # -5 → 5
```

## Creating Custom Behaviors

You can create domain-specific behaviors for your application:

```python
from glang.behaviors import create_behavior, BehaviorRegistry

# Simple transformation
double_behavior = create_behavior(
    "double",
    transform=lambda value: NumberValue(value.value * 2) 
                            if isinstance(value, NumberValue) 
                            else value
)

# Validation with fallback
def validate_even(value):
    if isinstance(value, NumberValue):
        return value.value % 2 == 0
    return True

def make_even(value):
    if isinstance(value, NumberValue) and value.value % 2 != 0:
        return NumberValue(value.value + 1)
    return value

even_only = create_behavior(
    "even_only",
    validate=validate_even,
    on_invalid=make_even
)

# Register for use
registry = BehaviorRegistry()
registry.register("double", double_behavior)
registry.register("even_only", even_only)

# Use in pipeline
pipeline = BehaviorPipeline(registry)
pipeline.add("double")
pipeline.add("even_only")
```

## Practical Examples

### Medical Data Validation
```python
# Blood pressure readings with safety constraints
bp_pipeline = BehaviorPipeline()
bp_pipeline.add("nil_to_zero")
bp_pipeline.add("validate_range", 60, 200)  # Reasonable BP range

readings = ListValue([
    NumberValue(120),    # Normal
    NoneValue(),         # Missing → 60
    NumberValue(250),    # Too high → 200
])
validated = bp_pipeline.apply_to_list(readings)
```

### Configuration Normalization
```python
# Server port configuration
config_pipeline = BehaviorPipeline()
config_pipeline.add("nil_to_zero")
config_pipeline.add("validate_range", 1024, 65535)  # Valid port range

config = HashValue([
    ("port", NumberValue(8080)),        # Valid → 8080
    ("admin_port", NumberValue(80)),    # Too low → 1024
    ("debug_port", NoneValue()),        # Missing → 1024
])

# Apply to specific hash keys
config_pipeline.apply_to_hash_value(config, "admin_port")
config_pipeline.apply_to_hash_value(config, "debug_port")
```

### Financial Data Processing
```python
# Price validation with rounding
price_pipeline = BehaviorPipeline()
price_pipeline.add("nil_to_zero")
price_pipeline.add("positive")           # No negative prices
price_pipeline.add("validate_range", 0.01, 999999)
price_pipeline.add("round_to_int")       # Round to cents

prices = ListValue([
    NumberValue(19.99),     # Valid
    NumberValue(-5),        # Negative → 5
    NoneValue(),            # Missing → 0.01
    NumberValue(1000000),   # Too high → 999999
])
```

## Behavior Composition

Behaviors apply in the order they're added to the pipeline:

```python
pipeline = BehaviorPipeline()
pipeline.add("nil_to_zero")       # 1st: Handle missing data
pipeline.add("positive")          # 2nd: Make positive
pipeline.add("validate_range", 0, 100)  # 3rd: Clamp to range
pipeline.add("round_to_int")      # 4th: Round result

# For input: NoneValue()
# Step 1: nil → 0
# Step 2: 0 → 0 (already positive)
# Step 3: 0 → 0 (within range)
# Step 4: 0 → 0 (already integer)
# Result: 0
```

## Future: Native Glang Syntax

Currently, behaviors are accessed through the Python API. Future versions will support native Glang syntax:

```glang
# Future syntax (not yet implemented)
temperature: num with [NilToZero, ValidateRange(95, 105)]
priority: var with [MapColors, NilToZero]

# Applied automatically on assignment
temperature = nil        # Becomes 95 (nil → 0 → clamped to 95)
priority = "green"       # Becomes 2
```

## Use Cases

The behavior system is ideal for:

- **Medical/Healthcare**: Validate vital signs, handle missing readings
- **Financial**: Normalize prices, validate amounts, handle currencies
- **Configuration**: Validate settings, provide defaults
- **IoT/Sensors**: Handle missing data, validate ranges, filter noise
- **Gaming**: Clamp stats, validate inputs, normalize values
- **Data Import**: Clean messy data, handle nulls, standardize formats

## Performance Considerations

- Behaviors are applied eagerly (not lazy)
- Pipeline order matters for both correctness and performance
- Custom behaviors should be stateless for thread safety
- Behaviors can be reused across multiple pipelines

## API Reference

### BehaviorPipeline
```python
pipeline = BehaviorPipeline(registry=None)  # Use custom or global registry
pipeline.add(behavior_name, *args)          # Add behavior to pipeline
pipeline.apply(value)                       # Apply to single value
pipeline.apply_to_list(list_value)          # Apply to all list elements
pipeline.apply_to_hash_value(hash, key)     # Apply to specific hash key
```

### create_behavior
```python
behavior = create_behavior(
    name,                    # Behavior name
    transform=None,          # Transform function (value → value)
    validate=None,           # Validation function (value → bool)
    on_invalid=None          # Fallback for invalid values
)
```

### BehaviorRegistry
```python
registry = BehaviorRegistry()        # Create custom registry
registry.register(name, behavior)    # Register custom behavior
behavior = registry.get(name)        # Retrieve behavior
```

## Best Practices

1. **Order matters**: Place nil handlers before validators
2. **Be explicit**: Name behaviors clearly (e.g., "celsius_to_fahrenheit" not "convert")
3. **Compose small behaviors**: Better to chain simple behaviors than create complex ones
4. **Document domain logic**: Explain why certain validations/transformations are needed
5. **Test edge cases**: Especially nil, negative, and out-of-range values
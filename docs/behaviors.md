# Glang Intrinsic Behavior System

## Overview

Glang's intrinsic behavior system allows you to attach transformation and validation rules directly to data structures (lists and hashes). Once attached, these behaviors automatically apply to all current and future values in the container.

**Related**: See [Configuration Blocks](language_features/configuration_blocks.md) for language-level behavior control that applies to entire scopes rather than individual containers.

## Key Concepts

### Intrinsic vs External
Unlike traditional approaches where you apply transformations externally, Glang behaviors are **intrinsic** - they become part of the data structure itself:

```glang
# Traditional (external) approach - NOT how Glang works
processed = transform(data, rule)  # Must remember to transform each time

# Glang (intrinsic) approach - behaviors are part of the data
data.add_rule("nil_to_zero")       # One-time setup
data.append(nil)                   # Automatically becomes 0
```

### Automatic Application
Behaviors apply automatically in two scenarios:
1. **Retroactively** to all existing values when a rule is added
2. **Proactively** to all new values during operations (append, set, etc.)

## Basic Usage

### Adding Behaviors to Lists

```glang
# Create a list with some problematic values
readings = [98.6, nil, 102.5, nil, -5]

# Add behavior to handle nil values
readings.add_rule("nil_to_zero")
print(readings)  # [98.6, 0, 102.5, 0, -5]

# Add behavior to ensure positive values
readings.add_rule("positive")
print(readings)  # [98.6, 0, 102.5, 0, 5]

# New values are automatically processed
readings.append(nil)      # Becomes 0
readings.append(-10)      # Becomes 10
print(readings)           # [98.6, 0, 102.5, 0, 5, 0, 10]
```

### Adding Behaviors to Hashes

```glang
# Create configuration with questionable values
config = {
    "timeout": nil,
    "retries": -5,
    "port": 99999,
    "debug": true
}

# Add behaviors to sanitize configuration
config.add_rule("nil_to_zero")              # Handle missing values
config.add_rule("positive")                 # No negative numbers
config.add_rule("validate_range", 1, 65535) # Valid port range

print(config["timeout"])  # 0 (was nil)
print(config["retries"])  # 5 (was -5)
print(config["port"])     # 65535 (was 99999, clamped)

# New entries are automatically processed
config["max_connections"] = nil   # Becomes 0
config["min_threads"] = -10       # Becomes 10
```

## Behaviors with Parameters

Some behaviors accept parameters to customize their operation:

```glang
# Temperature readings that should be in Fahrenheit body temp range
temps = [32, 98.6, 212, nil]

# Add range validation with parameters
temps.add_rule("validate_range", 95, 105)  # Normal body temp range
temps.add_rule("nil_to_zero")

print(temps)  # [95, 98.6, 105, 0]
```

## Managing Behaviors

### Query Behaviors

```glang
list = [1, 2, 3]
list.add_rule("positive")
list.add_rule("round_to_int")

# Check if a specific rule exists
has_positive = list.has_rule("positive")    # true
has_negative = list.has_rule("negative")    # false

# Get all active rules (sorted alphabetically)
rules = list.get_rules()                    # ["positive", "round_to_int"]
```

### Remove Behaviors

```glang
# Remove a specific behavior
removed = list.remove_rule("positive")      # Returns true if removed

# Clear all behaviors
list.clear_rules()                          # Remove all behaviors
```

## Standard Behaviors

### Value Transformation
- `nil_to_zero` - Convert nil/null values to 0
- `nil_to_empty` - Convert nil/null values to empty string ""
- `positive` - Make negative numbers positive (absolute value)
- `round_to_int` - Round decimal numbers to integers

### String Transformation
- `uppercase` - Convert strings to UPPERCASE
- `lowercase` - Convert strings to lowercase
- `map_colors` - Map color names to numbers (red→1, green→2, blue→3, etc.)

### Validation
- `validate_range(min, max)` - Clamp numbers to specified range

## Multiple Behaviors

Behaviors are applied in the order they were added:

```glang
data = [nil, -50.7]

# Order matters!
data.add_rule("nil_to_zero")    # First: nil → 0
data.add_rule("positive")       # Second: negatives → positive
data.add_rule("round_to_int")   # Third: decimals → integers

print(data)  # [0, 51]
# nil → 0 → 0 → 0
# -50.7 → -50.7 → 50.7 → 51
```

## Type Constraints

Behaviors work seamlessly with type constraints:

```glang
# List constrained to numbers
list<num> scores = [95, 87, nil]

# This behavior maintains the type constraint
scores.add_rule("nil_to_zero")  # nil → 0 (still a number)

scores.append(nil)               # Becomes 0, satisfies constraint
scores.append("text")            # ERROR: Cannot append string to list<num>
```

## Symbol Syntax (Future)

Once the parser supports symbols, you'll be able to use cleaner syntax:

```glang
# Future syntax with symbols (not yet implemented)
list.add_rule(:nil_to_zero)              # Cleaner than "nil_to_zero"
list.add_rule(:validate_range, 0, 100)   # With parameters
list.has_rule(:positive)                 # Query with symbol
list.remove_rule(:round_to_int)          # Remove with symbol
```

## Practical Examples

### Configuration Validation

```glang
# Database configuration with sensible defaults
db_config = {
    "host": "localhost",
    "port": nil,
    "timeout": -1,
    "max_connections": 1000000
}

# Apply behaviors for production safety
db_config.add_rule("nil_to_zero")
db_config.add_rule("positive")
db_config.add_rule("validate_range", 1, 10000)

# Results in safe configuration:
# port: 0 → 3306 (would need custom behavior for default)
# timeout: -1 → 1
# max_connections: 1000000 → 10000 (clamped)
```

### Sensor Data Cleaning

```glang
# Temperature sensors sometimes report invalid data
sensor_readings = []
sensor_readings.add_rule("nil_to_zero")          # Handle missing readings
sensor_readings.add_rule("validate_range", -50, 150)  # Reasonable Earth temps

# Now just append data without worrying about validation
sensor_readings.append(72.5)    # Valid: 72.5
sensor_readings.append(nil)     # Becomes: 0
sensor_readings.append(9999)    # Clamped: 150
sensor_readings.append(-100)    # Clamped: -50
```

### Financial Calculations

```glang
# Prices should never be negative and always rounded to cents
prices = []
prices.add_rule("positive")
prices.add_rule("round_to", 2)  # Round to 2 decimal places

prices.append(19.999)   # Becomes: 20.00
prices.append(-5.50)    # Becomes: 5.50
prices.append(nil)      # Would need nil_to_zero first
```

## Best Practices

1. **Add behaviors early** - Set up behaviors when creating the container
2. **Order matters** - Add behaviors in logical transformation order
3. **Document behaviors** - Comment which behaviors are active and why
4. **Test edge cases** - Verify behaviors handle nil, negative, and extreme values
5. **Use type constraints** - Combine behaviors with type constraints for safety

## Performance Considerations

- Behaviors are applied during operations (append, set), not on access
- Adding a behavior to existing data applies it once to all elements
- Removing behaviors doesn't undo previous transformations
- Behaviors are lightweight - just function calls during mutations

## Future Enhancements

The behavior system will be extended with:
- Custom behaviors defined in Glang functions
- Behavior inheritance in graph hierarchies
- Conditional behaviors based on context
- Behavior composition and pipelines
- Graph-specific behaviors for nodes and edges
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
- `map_colors` - Map color names to numbers (deprecated - use generic mapping instead)

### Validation
- `validate_range(min, max)` - Clamp numbers to specified range

## Generic Mapping Behaviors

Create custom value mappings using hash graphs. This powerful feature replaces hardcoded behaviors like `map_colors` with user-defined mappings.

### Basic Mapping

```glang
# Define a status mapping
status_map = { "active": 1, "inactive": 0, "pending": 2 }
user_statuses = ["active", "unknown", "inactive"]

# Apply the mapping to the list
user_statuses.add_mapping_rule(status_map)
print(user_statuses)  # [1, "unknown", 0]
```

### Mapping with Default Values

```glang
# Provide a default value for unmapped keys
user_statuses.add_mapping_rule(status_map, -1)  # Default -1 for unmapped
print(user_statuses)  # [1, -1, 0]
```

### Multi-Stage Transformations

```glang
# Chain mappings for complex transformations
codes = ["a", "b", "c"]

# First mapping: letters to Greek names
first_map = { "a": "alpha", "b": "beta", "c": "gamma" }
codes.add_mapping_rule(first_map)
print(codes)  # ["alpha", "beta", "gamma"]

# Second mapping: Greek names to numbers
second_map = { "alpha": 1, "beta": 2, "gamma": 3 }
codes.add_mapping_rule(second_map)
print(codes)  # [1, 2, 3]
```

### Hash Mapping

```glang
# Apply mappings to hash values
config = { "env": "dev", "region": "us-west", "unknown": "weird" }

env_mapping = { "dev": "development", "prod": "production", "staging": "staging" }
config.add_mapping_rule(env_mapping, "unknown_environment")

print(config["env"])     # "development" (mapped)
print(config["region"])  # "us-west" (unchanged)
print(config["unknown"]) # "unknown_environment" (default used)
```

### Practical Examples

#### Color Code Mapping
Replace the deprecated `map_colors` behavior:

```glang
# Old way (deprecated)
colors.add_rule("map_colors")  # Limited to predefined colors

# New way (flexible)
color_map = { "red": 1, "green": 2, "blue": 3, "yellow": 4, "purple": 5 }
colors.add_mapping_rule(color_map, 0)  # 0 for unknown colors
```

#### Grade Letter to Number
```glang
grades = ["A", "B", "C", "F", "A+"]
grade_map = { "A+": 97, "A": 95, "B": 85, "C": 75, "D": 65, "F": 0 }
grades.add_mapping_rule(grade_map, 50)  # 50 for unrecognized grades
print(grades)  # [95, 85, 75, 0, 97]
```

#### Department Code Translation
```glang
employees = ["walter", "james", "unknown", "emily"]
dept_map = { "walter": "HR", "james": "IT", "emily": "Admin" }
employees.add_mapping_rule(dept_map, "Unassigned")
print(employees)  # ["HR", "IT", "Unassigned", "Admin"]
```

### Key Benefits

- **User-Defined**: Create any mapping you need using hash graphs
- **Graph-Based**: Uses Glang's native HashValue graph structures
- **Default Values**: Specify fallback values for unmapped keys
- **Composable**: Chain multiple mappings for complex transformations
- **Type-Safe**: Works with type constraints and other behaviors
- **Replaces Hardcoded**: Eliminates need for predefined behaviors like `map_colors`

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

## Custom Function Behaviors

Create custom behaviors using your own Glang functions. This powerful feature allows you to define any transformation logic you need.

### Basic Custom Functions

```glang
# Define a temperature normalization function
func normalize_temp(value) {
    if value < 95 { return 95 }
    if value > 105 { return 105 }
    return value
}

# Attach the function as a behavior
temperatures = [85, 98.6, 110, 102]
temperatures.add_custom_rule(normalize_temp)
print(temperatures)  # [95, 98.6, 105, 102]
```

### Lambda Functions (Future)

```glang
# Future: Lambda expressions as behaviors
scores.add_custom_rule(x => x > 100 ? 100 : x)  # Cap scores at 100
```

### Practical Examples

#### Data Validation
```glang
# Email validation function
func validate_email(value) {
    if value.contains("@") and value.contains(".") {
        return value
    }
    return "invalid@example.com"
}

emails = ["user@example.com", "invalid-email", "admin@company.org"]
emails.add_custom_rule(validate_email)
print(emails)  # ["user@example.com", "invalid@example.com", "admin@company.org"]
```

#### Mathematical Transformations
```glang
# Custom mathematical function
func apply_discount(price) {
    discount_rate = 0.15  # 15% discount
    discounted = price * (1 - discount_rate)
    return discounted
}

prices = [100, 250, 50]
prices.add_custom_rule(apply_discount)
print(prices)  # [85, 212.5, 42.5]
```

#### Complex Business Logic
```glang
# Grade calculation with custom logic
func calculate_final_grade(raw_score) {
    # Apply curve and grade boundaries
    curved = raw_score + 5  # 5 point curve

    if curved >= 90 { return "A" }
    if curved >= 80 { return "B" }
    if curved >= 70 { return "C" }
    if curved >= 60 { return "D" }
    return "F"
}

scores = [85, 92, 67, 78]
scores.add_custom_rule(calculate_final_grade)
print(scores)  # ["A", "A", "C", "B"]
```

### Combining Custom Functions with Standard Behaviors

```glang
# Use custom functions with standard behaviors
sensor_data = [none, -5.2, 150, 98.6]

# First handle missing data, then apply custom logic
sensor_data.add_rule("none_to_zero")     # none -> 0
sensor_data.add_custom_rule(normalize_temp)  # Apply custom function
print(sensor_data)  # [95, 95, 105, 98.6]
```

### Error Handling

Custom functions that fail gracefully - if your function throws an error, the original value is preserved:

```glang
# Function that might fail
func risky_transform(value) {
    return value / 0  # Division by zero
}

numbers = [10, 20, 30]
numbers.add_custom_rule(risky_transform)
print(numbers)  # [10, 20, 30] - original values preserved
```

### Key Benefits

- **Unlimited Flexibility**: Define any transformation logic you need
- **Type-Safe**: Works with Glang's type system and constraints
- **Composable**: Chain with standard behaviors and other custom functions
- **Error-Safe**: Failed functions don't break your data pipeline
- **Graph-Native**: Uses Glang's function system directly

## Conditional Behavior System

Create context-aware behaviors that only apply when specific conditions are met. This powerful feature allows you to define behaviors that process different types of data differently.

### Basic Conditional Behaviors

```glang
# Apply different transformations based on value type
mixed_data = [42, "hello", -10, "WORLD", nil]

# Define condition and transform functions
func is_string(value) {
    return value.get_type() == "string"
}

func to_upper(value) {
    return value.upper()
}

# Apply conditional behavior - only strings get uppercased
mixed_data.add_conditional_rule(is_string, to_upper)
print(mixed_data)  # [42, "HELLO", -10, "WORLD", nil]
```

### Conditional Behaviors with Fallback

```glang
# Process numbers differently based on their sign
numbers = [5, -3, 0, 15, -8]

# Define condition and transform functions
func is_negative(value) {
    return value.get_type() == "number" and value < 0
}

func make_positive(value) {
    return -value  # Convert to positive
}

func mark_zero(value) {
    return 999  # Special marker for non-negative numbers
}

# Apply conditional with fallback
numbers.add_conditional_rule(is_negative, make_positive, mark_zero)
print(numbers)  # [999, 3, 999, 999, 8]
```

### Type-Specific Processing

```glang
# Clean and normalize mixed user input
user_input = ["  Alice  ", 42, "BOB", nil, "charlie", 0]

# String normalization
func is_string(value) {
    return value.get_type() == "string"
}

func normalize_string(value) {
    return value.trim().title()  # Trim and capitalize
}

func default_string(value) {
    return "Unknown"  # Default for non-strings
}

user_input.add_conditional_rule(is_string, normalize_string, default_string)
print(user_input)  # ["Alice", "Unknown", "Bob", "Unknown", "Charlie", "Unknown"]
```

### Data Validation with Conditions

```glang
# Validate and clean sensor readings
sensor_data = [98.6, -999, 102.1, nil, 150.5, 72.0]

# Valid temperature range condition
func is_valid_temp(value) {
    if value.get_type() != "number" { return false }
    return value >= 95 and value <= 105
}

func keep_temp(value) {
    return value  # Keep valid temperatures
}

func replace_invalid(value) {
    return 98.6  # Default normal temperature
}

sensor_data.add_conditional_rule(is_valid_temp, keep_temp, replace_invalid)
print(sensor_data)  # [98.6, 98.6, 102.1, 98.6, 98.6, 98.6]
```

### Complex Business Logic

```glang
# Grade assignment based on score ranges
scores = [95, 87, 76, 65, 45, 92]

func is_excellent(value) {
    return value >= 90
}

func assign_a_plus(value) {
    return "A+"
}

func assign_default_grade(value) {
    # Further processing for other ranges
    if value >= 80 { return "A" }
    if value >= 70 { return "B" }
    if value >= 60 { return "C" }
    return "F"
}

scores.add_conditional_rule(is_excellent, assign_a_plus, assign_default_grade)
print(scores)  # ["A+", "A", "B", "C", "F", "A+"]
```

### Multiple Conditional Rules

```glang
# Chain multiple conditional behaviors for complex processing
data = ["email@test.com", "not-email", "user@domain.org", "invalid"]

# First rule: validate emails
func is_email(value) {
    return value.contains("@") and value.contains(".")
}

func keep_email(value) {
    return value
}

func mark_invalid_email(value) {
    return "INVALID_EMAIL"
}

data.add_conditional_rule(is_email, keep_email, mark_invalid_email)

# Second rule: normalize valid emails
func is_valid_email(value) {
    return value != "INVALID_EMAIL"
}

func normalize_email(value) {
    return value.lower()
}

data.add_conditional_rule(is_valid_email, normalize_email)
print(data)  # ["email@test.com", "INVALID_EMAIL", "user@domain.org", "INVALID_EMAIL"]
```

### Key Benefits

- **Context-Aware Processing**: Apply different logic based on value characteristics
- **Type-Safe Operations**: Only process values that meet specific criteria
- **Fallback Handling**: Define what happens when conditions aren't met
- **Composable Logic**: Chain multiple conditional behaviors for complex workflows
- **Error-Safe**: Failed conditions or transforms don't break your data pipeline
- **Graph-Native**: Uses Glang's function system directly for maximum flexibility

### API Reference

```glang
# Basic conditional rule
container.add_conditional_rule(condition_func, transform_func)

# Conditional rule with fallback
container.add_conditional_rule(condition_func, transform_func, on_fail_func)
```

**Parameters:**
- `condition_func`: Function that takes a value and returns boolean
- `transform_func`: Function to apply when condition is true
- `on_fail_func`: Optional function to apply when condition is false

**Returns:** `nil` (behaviors modify the container in place)

### Error Handling

Conditional behaviors are designed to be robust:

```glang
# If condition function fails, value is left unchanged
func risky_condition(value) {
    return value.some_method_that_might_not_exist()
}

func safe_transform(value) {
    return value.upper()
}

data.add_conditional_rule(risky_condition, safe_transform)
# Values that cause condition to fail are left as-is
```

## Ruleset System (Declarative Bundle Application)

Create reusable behavior bundles with clean declarative syntax. The ruleset system allows you to define collections of behaviors once and apply them to multiple containers efficiently.

### Basic Ruleset Creation

```glang
# Create a data cleaning ruleset directly
data_cleaning_rules = [
    "none_to_zero",
    "positive",
    "round_to_int"
]

data_cleaning = Rules[data_cleaning_rules]

# Apply to multiple datasets
temperatures = [98.6, none, -102.5, 72.1]
blood_pressure = [120, none, -80, 140]
heart_rate = [75, none, -60, 85]

# Apply the same ruleset to all containers
temperatures.add_rules(data_cleaning)
blood_pressure.add_rules(data_cleaning)
heart_rate.add_rules(data_cleaning)

print(temperatures)    # [98.6, 0, 102.5, 72] - cleaned data
print(blood_pressure)  # [120, 0, 80, 140] - cleaned data
print(heart_rate)      # [75, 0, 60, 85] - cleaned data
```

### Medical Validation Ruleset

```glang
# Create specialized rulesets for different domains
medical_validation = Rules[
    "none_to_zero",
    "positive",
    "validate_range", 60, 200  # Valid vital sign range
]

# Apply to all medical datasets
systolic_pressure.add_rules(medical_validation)
diastolic_pressure.add_rules(medical_validation)
pulse_rate.add_rules(medical_validation)
```

### Financial Data Processing

```glang
# Financial data requires specific validation
financial_rules = Rules[
    "none_to_zero",
    "positive",
    "round_to_int",  # Round to cents
    "validate_range", 0, 1000000  # Reasonable financial range
]

# Apply to financial datasets
prices = [19.99, none, -5.50, 1500000]
revenues = [100000, none, -500, 250000]

prices.add_rules(financial_rules)
revenues.add_rules(financial_rules)

print(prices)    # [20, 0, 6, 1000000] - cleaned financial data
print(revenues)  # [100000, 0, 500, 250000] - cleaned financial data
```

### Combining Different Rule Types

```glang
# Rulesets can contain different types of behaviors
complex_cleaning = Rules[
    "none_to_zero",           # Standard behavior
    "positive"                # Another standard behavior
]

# Add custom function to existing ruleset
func normalize_temperature(value) {
    if value < 95 { return 95 }
    if value > 105 { return 105 }
    return value
}

# You can add more rules to existing rulesets
complex_cleaning.add_rule(normalize_temperature)

# Apply the enhanced ruleset
sensor_data.add_rules(complex_cleaning)
```

### Ruleset Management

```glang
# Query ruleset information
ruleset = Rules["none_to_zero", "positive"]

print("Ruleset size: " + ruleset.size().to_string())     # "Ruleset size: 2"
print("Rules: " + ruleset.get_rules().to_string())       # List of rule names

# Add more rules dynamically
ruleset.add_rule("round_to_int")
print("New size: " + ruleset.size().to_string())         # "New size: 3"
```

### Domain-Specific Rulesets

```glang
# Environmental monitoring ruleset
environmental_monitoring = Rules[
    "none_to_zero",
    "validate_range", -50, 150,  # Temperature range in Celsius
    "positive"                   # For humidity, pressure readings
]

# Web scraping data cleanup
web_scraping_cleanup = Rules[
    "none_to_empty",            # Convert none to empty string
    "uppercase",                # Normalize text case
    "validate_range", 0, 1000   # Reasonable numeric ranges
]

# Apply to appropriate datasets
temperature_sensors.add_rules(environmental_monitoring)
humidity_sensors.add_rules(environmental_monitoring)

scraped_titles.add_rules(web_scraping_cleanup)
scraped_descriptions.add_rules(web_scraping_cleanup)
```

### Efficiency Benefits

```glang
# Instead of applying rules individually:
data.add_rule("none_to_zero")
data.add_rule("positive")
data.add_rule("round_to_int")
data.add_rule("validate_range", 0, 100)

# Use rulesets for better performance and reusability:
data_rules = Rules[
    "none_to_zero",
    "positive",
    "round_to_int",
    "validate_range", 0, 100
]

data.add_rules(data_rules)  # Single call applies all rules efficiently
```

### Key Benefits

- **Declarative Syntax**: Define behavior collections cleanly and clearly
- **Reusability**: Create once, apply to multiple containers
- **Efficiency**: Bundle application is more performant than individual rule addition
- **Organization**: Group related behaviors logically by domain or purpose
- **Maintainability**: Update ruleset definitions in one place
- **Composability**: Combine with existing behavior system seamlessly

### API Reference

```glang
# Create ruleset from rule list
ruleset = Rules[rule_list]

# Apply ruleset to container
container.add_rules(ruleset)

# Manage ruleset contents
ruleset.add_rule(rule_spec)
ruleset.get_rules()
ruleset.size()
```

**Parameters:**
- `rule_list`: Array of rule specifications (strings, symbols, functions)
- `rule_spec`: Individual rule specification to add to ruleset

**Returns:** Ruleset operations return appropriate values (NoneValue for mutations, data for queries)

### Integration with Other Behavior Types

Rulesets work seamlessly with all behavior types:

```glang
# Mix standard behaviors, custom functions, mappings, and conditionals
comprehensive_ruleset = Rules[
    "none_to_zero",           # Standard behavior
    "positive"                # Standard behavior
]

# Add custom function
comprehensive_ruleset.add_rule(custom_validator)

# Add mapping behavior (when implemented)
comprehensive_ruleset.add_rule(color_mapping)

# Apply everything at once
mixed_data.add_rules(comprehensive_ruleset)
```

## Future Enhancements

The behavior system will be extended with:
- Behavior composition and pipelines
- Graph-specific behaviors for nodes and edges
- Behavior inheritance in graph hierarchies

**Completed**:
- Generic mapping behaviors (available now!)
- Custom function behaviors (available now!)
- Conditional behavior system (available now!)
- Ruleset system (available now!)
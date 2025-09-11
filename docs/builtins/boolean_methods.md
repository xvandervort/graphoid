# Boolean Methods

Boolean (bool) is a built-in type in Glang representing true/false values. Booleans support logical operations and various utility methods.

## Type Information

### type()
Returns the type of the value.
```glang
flag = true
flag.type()  # Returns "bool"
```

### methods()
Returns a list of all available methods for booleans.
```glang
flag = false
flag.methods()  # Returns ["type", "methods", "can", "inspect", "flip", "toggle", "numify", ...]
```

### can(method_name)
Checks if a method is available on the boolean.
```glang
flag = true
flag.can("flip")  # Returns true
flag.can("invalid")  # Returns false
```

### inspect()
Returns detailed information about the boolean.
```glang
flag = true
flag.inspect()  # Returns detailed boolean information
```

### size()
For booleans, returns 1 (atomic value).
```glang
flag = true
flag.size()  # Returns 1
```

## Boolean Operations

### flip()
Toggles the boolean value (true becomes false, false becomes true).
```glang
flag = true
flag.flip()  # Returns false

flag = false
flag.flip()  # Returns true
```

### toggle()
Alias for flip(). Toggles the boolean value.
```glang
is_active = true
is_active.toggle()  # Returns false
```

## Type Conversion

### to_string()
Converts the boolean to a string.
```glang
true.to_string()  # Returns "true"
false.to_string()  # Returns "false"
```

### to_num()
Converts the boolean to a number (true = 1, false = 0).
```glang
true.to_num()  # Returns 1
false.to_num()  # Returns 0
```

### numify()
Alias for to_num(). Converts to number.
```glang
flag = true
flag.numify()  # Returns 1
```

### toNum()
Another alias for to_num().
```glang
is_valid = false
is_valid.toNum()  # Returns 0
```

### to_bool()
Returns the boolean itself (identity operation for booleans).
```glang
flag = true
flag.to_bool()  # Returns true
```

## Immutability Methods

### freeze()
Makes the boolean immutable. Returns self for chaining.
```glang
flag = true
flag.freeze()
flag.is_frozen()  # Returns true
```

### is_frozen()
Checks if the boolean is frozen (immutable).
```glang
flag = false
flag.freeze()
flag.is_frozen()  # Returns true
```

### contains_frozen()
For booleans, returns the same as is_frozen() since booleans are atomic values.
```glang
flag = true
flag.freeze()
flag.contains_frozen()  # Returns true
```

## Logical Operations

Booleans support standard logical operations:

### Logical AND
```glang
a = true
b = false

a and b  # Returns false
true and true  # Returns true
false and true  # Returns false
```

### Logical OR
```glang
a = true
b = false

a or b  # Returns true
false or false  # Returns false
true or false  # Returns true
```

### Logical NOT
```glang
flag = true
not flag  # Returns false

not false  # Returns true
not true  # Returns false
```

### Comparison
```glang
a = true
b = false

a == b  # Returns false
a != b  # Returns true
true == true  # Returns true
false != false  # Returns false
```

## Examples

### Flag Management
```glang
# Toggle feature flags
debug_mode = false
verbose = true

# Toggle debug mode
debug_mode = debug_mode.flip()
print("Debug mode: " + debug_mode.to_string())

# Convert to number for calculations
flags_enabled = debug_mode.to_num() + verbose.to_num()
print("Active flags: " + flags_enabled.to_string())
```

### Conditional Logic
```glang
# User permissions
is_admin = true
is_active = true
has_permission = false

# Check access
can_access = is_admin and is_active
print("Can access: " + can_access.to_string())

# Complex conditions
should_notify = (is_admin or has_permission) and is_active
print("Should notify: " + should_notify.to_string())
```

### Boolean Arithmetic
```glang
# Count true values
flags = [true, false, true, true, false]
true_count = 0

for flag in flags {
    true_count = true_count + flag.to_num()
}

print("True values: " + true_count.to_string())
print("False values: " + (flags.size() - true_count).to_string())
```

### State Toggling
```glang
# Light switch simulation
light_on = false

# Toggle light multiple times
light_on = light_on.toggle()
print("Light: " + light_on.to_string())  # "true"

light_on = light_on.toggle()
print("Light: " + light_on.to_string())  # "false"

light_on = light_on.toggle()
print("Light: " + light_on.to_string())  # "true"
```

### Boolean to String Formatting
```glang
# Format boolean values for display
is_valid = true
is_complete = false

status = "Valid: " + is_valid.to_string() + ", Complete: " + is_complete.to_string()
print(status)

# Custom formatting
if is_valid {
    validity = "✓"
} else {
    validity = "✗"
}
print("Status: " + validity)
```

### Boolean Collections
```glang
# Process boolean results
test_results = [true, true, false, true, false]

# Count passes and fails
passes = 0
for result in test_results {
    passes = passes + result.to_num()
}
fails = test_results.size() - passes

print("Tests passed: " + passes.to_string())
print("Tests failed: " + fails.to_string())

# Calculate pass rate
pass_rate = passes * 100 / test_results.size()
print("Pass rate: " + pass_rate.to_string() + "%")
```

### Truthiness Evaluation
```glang
# Different values convert to boolean differently
values = [0, 1, "", "hello", [], [1, 2, 3]]

for value in values {
    bool_value = value.to_bool()
    print(value.to_string() + " -> " + bool_value.to_string())
}
# Output:
# 0 -> false
# 1 -> true
# "" -> false
# "hello" -> true
# [] -> false
# [1, 2, 3] -> true
```

### Feature Flags Pattern
```glang
# Application feature flags
features = {
    "dark_mode": true,
    "notifications": false,
    "auto_save": true,
    "beta_features": false
}

# Count enabled features
enabled_count = 0
for key in features.keys() {
    if features[key].value() {
        enabled_count = enabled_count + 1
        print("Enabled: " + key)
    }
}

print("Total enabled features: " + enabled_count.to_string())
```
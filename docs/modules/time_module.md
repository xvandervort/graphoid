# Time Module Documentation

The Time module provides a single `Time` type for working with dates and times in Glang. All times are stored internally as UTC timestamps and can be converted to different representations as needed.

## Design Philosophy

- **Single Type**: One `Time` type instead of separate date, time, and datetime types
- **UTC Internal Storage**: All times stored as UTC timestamps for consistency
- **Natural Methods**: Intuitive method names like `as_date()` instead of format strings
- **Full Type Casting**: Seamless conversion between time, number, and string types
- **Calendar Awareness**: Proper handling of date arithmetic and edge cases

## Importing the Module

```glang
import "time" as Time
```

## Creating Time Values

### Current Time Functions

```glang
# Get current time
current = Time.now()
print(current.to_string())  # "2025-01-15T14:30:00Z"

# Get start of today (00:00:00 UTC)
today = Time.today()
print(today.to_string())    # "2025-01-15T00:00:00Z"
```

### From Components

```glang
# Date only (defaults to 00:00:00 UTC)
birthday = Time.from_components(1990, 12, 25)
print(birthday.to_string())  # "1990-12-25T00:00:00Z"

# Full date and time
meeting = Time.from_components(2025, 1, 15, 14, 30, 0)
print(meeting.to_string())   # "2025-01-15T14:30:00Z"

# Partial time specification
lunch = Time.from_components(2025, 6, 15, 12, 30)  # seconds default to 0
print(lunch.to_string())     # "2025-06-15T12:30:00Z"
```

### From String

```glang
# Parse ISO format strings
parsed = Time.from_string("2025-01-15T14:30:00")
print(parsed.to_string())    # "2025-01-15T14:30:00Z"

# With explicit Z suffix
utc_time = Time.from_string("2025-01-15T14:30:00Z")
print(utc_time.to_string())  # "2025-01-15T14:30:00Z"
```

## Working with Time Values

### Basic Methods

```glang
import "time" as Time

t = Time.from_components(2025, 6, 15, 10, 30, 45)

# Get type information (both syntaxes work)
print(t.get_type())      # "time" - with parentheses
print(t.get_type)        # "time" - without parentheses (more elegant)

# String representation (ISO format)
print(t.to_string())     # "2025-06-15T10:30:45Z" - with parentheses
print(t.to_string)       # "2025-06-15T10:30:45Z" - without parentheses
```

**Glang Feature**: Zero-argument methods can be called with or without parentheses, making property-like access feel natural.

### Type Casting

The Time module provides comprehensive type casting between time values, numbers (timestamps), and strings.

#### Time to Number (Timestamp)

```glang
# Convert time to Unix timestamp
time_value = Time.from_components(2025, 1, 1, 0, 0, 0)
timestamp = time_value.to_num()
print(timestamp.to_string())  # "1735689600"
```

#### Number to Time

```glang
# Convert Unix timestamp to time
timestamp = 1735689600
time_value = timestamp.to_time()
print(time_value.to_string())  # "2025-01-01T00:00:00Z"
```

#### String to Time

```glang
# Parse ISO format string to time
time_str = "2025-01-15T14:30:00"
time_value = time_str.to_time()
print(time_value.to_string())  # "2025-01-15T14:30:00Z"
```

#### Round-Trip Consistency

All casting operations maintain perfect consistency:

```glang
import "time" as Time

# Original time
original = Time.from_components(2025, 6, 15, 10, 30, 45)

# Round-trip through number
number_trip = original.to_num().to_time()
print(original.to_string() == number_trip.to_string())  # true

# Round-trip through string
string_trip = original.to_string().to_time()
print(original.to_string() == string_trip.to_string())  # true
```

## Function Reference

### Module Functions

| Function | Description | Example |
|----------|-------------|---------|
| `Time.now()` | Get current time | `Time.now()` |
| `Time.today()` | Get start of today (00:00:00 UTC) | `Time.today()` |
| `Time.from_components(year, month, day, [hour], [minute], [second])` | Create time from date/time components | `Time.from_components(2025, 1, 15, 14, 30)` |
| `Time.from_string(iso_string)` | Parse ISO format string | `Time.from_string("2025-01-15T14:30:00")` |

### Time Value Methods

| Method | Return Type | Description | Example |
|--------|-------------|-------------|---------|
| `get_type()` | string | Get type name | `time_val.get_type()` → `"time"` |
| `to_string()` | string | Convert to ISO format string | `time_val.to_string()` → `"2025-01-15T14:30:00Z"` |
| `to_num()` | num | Convert to Unix timestamp | `time_val.to_num()` → `1736951400` |

### Type Casting Methods (Other Types)

| Method | On Type | Return Type | Description |
|--------|---------|-------------|-------------|
| `to_time()` | num | time | Convert Unix timestamp to time |
| `to_time()` | string | time | Parse ISO format string to time |

## Error Handling

### Invalid Inputs

```glang
# Invalid date components
result = Time.from_components(2025, 13, 35)  # Error: Invalid date/time

# Invalid string format
result = "not-a-date".to_time()  # Error: Invalid time format

# Wrong argument types
result = Time.from_components("2025", 1, 1)  # Error: Year, month, and day must be numbers
```

### Method Argument Validation

```glang
import "time" as Time
t = Time.now()

# All time methods take no arguments
result = t.to_string("extra")  # Error: to_string() takes no arguments
result = t.to_num("extra")     # Error: to_num() takes no arguments
```

## Examples

### Working with Timestamps

```glang
import "time" as Time

# Create a specific time
event = Time.from_components(2025, 12, 31, 23, 59, 59)
print("Event: " + event.to_string())

# Convert to timestamp for storage/comparison
timestamp = event.to_num()
print("Timestamp: " + timestamp.to_string())

# Convert back when needed
retrieved = timestamp.to_time()
print("Retrieved: " + retrieved.to_string())
```

### Parsing User Input

```glang
import "time" as Time

# Parse different time formats (all converted to UTC)
user_inputs = [
    "2025-01-15T14:30:00",
    "2025-06-21T09:00:00Z",
    "2025-12-31T23:59:59"
]

for input_str in user_inputs {
    parsed = input_str.to_time()
    print("Parsed: " + parsed.to_string())
}
```

### Time Calculations

```glang
import "time" as Time

# Get current time and convert to timestamp for arithmetic
now = Time.now()
now_timestamp = now.to_num()

# Add one hour (3600 seconds)
later_timestamp = now_timestamp + 3600
later = later_timestamp.to_time()

print("Now: " + now.to_string())
print("Later: " + later.to_string())
```

## Implementation Notes

- All times are stored internally as UTC timestamps (Unix time)
- String representation always uses ISO 8601 format with 'Z' suffix
- Type casting maintains precision and consistency
- Calendar arithmetic respects leap years and month boundaries
- Error messages provide clear guidance for invalid inputs

## Future Enhancements

The Time module is designed to be extensible. Future versions may include:

- Natural date arithmetic methods (`add_days`, `add_months`, etc.)
- Timezone conversion methods (`as_timezone`, `to_local`)
- Date formatting options (`as_date`, `as_time`, `fmt`)
- Comparison operations (`before`, `after`, `between`)
- Duration calculations (`diff`, `elapsed`)

For now, the focus is on providing solid, consistent basic functionality that integrates well with Glang's type system and method-based design.
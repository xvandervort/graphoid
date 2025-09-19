# Universal Metadata Layer

The Universal Metadata Layer is a foundational feature of Glang's graph architecture that provides extensible metadata storage for all graph structures. Every graph in Glang has an always-present metadata layer that enables R vector-style element naming, data provenance tracking, and arbitrary metadata storage.

## Overview

### Key Principles

1. **Always Present**: Every graph structure has a metadata layer - no "optional" metadata
2. **Graceful Degradation**: Operations work whether metadata exists or not
3. **Extensible**: Supports any key-value metadata, not just element names
4. **Graph-Native**: Built into the core graph foundation, not layered on top
5. **Non-Intrusive**: Existing functionality works unchanged

### Architecture

The metadata layer consists of:

- **MetadataLayer Class**: Universal key-value storage for any metadata
- **Graph Integration**: All `GraphStructure` instances include `metadata` property
- **List Enhancement**: SequentialGraph supports R vector-style element naming
- **API Exposure**: ListValue exposes metadata functionality to Glang code

## Basic Usage

### Accessing Metadata

Every list has a `metadata` property for general metadata storage:

```glang
heights = [165, 180, 175]

# Set custom metadata
heights.metadata.set("units", "centimeters")
heights.metadata.set("source", "health_survey_2025")
heights.metadata.set("accuracy", 95.5)

# Retrieve metadata
units = heights.metadata.get("units")        # "centimeters"
source = heights.metadata.get("source")      # "health_survey_2025"
missing = heights.metadata.get("missing", "default")  # "default"

# Check existence
has_units = heights.metadata.has("units")    # true
```

## Element Naming (R Vector Style)

### Basic Naming

```glang
# Create a list
heights = [165, 180, 175]

# Initially no names
names = heights.get_names()       # [nil, nil, nil]
has_names = heights.has_names()   # false

# Set names for all elements
heights.set_names(["alice", "bob", "charlie"])

# Now has names
names = heights.get_names()       # ["alice", "bob", "charlie"]
has_names = heights.has_names()   # true
```

### Method Chaining for Inline Initialization

Since `set_names` returns the list itself, you can chain it during initialization:

```glang
# Inline initialization with names
heights = [165, 180, 175].set_names(["alice", "bob", "charlie"])

# This is equivalent to:
# heights = [165, 180, 175]
# heights.set_names(["alice", "bob", "charlie"])

# The list is immediately ready with both values and names
alice_height = heights["alice"]   # 165
bob_height = heights[1]           # 180 (same element as heights["bob"])
```

**Note**: Future versions may support R-style inline syntax like `[alice = 165, bob = 180, charlie = 175]` for even cleaner initialization.

### Dual Access Pattern

Once names are set, elements can be accessed both by index and by name:

```glang
heights = [165, 180, 175]
heights.set_names(["alice", "bob", "charlie"])

# Numeric access (unchanged)
alice_height = heights[0]         # 165
bob_height = heights[1]           # 180

# Name-based access (new)
alice_height = heights["alice"]   # 165
bob_height = heights["bob"]       # 180

# Both refer to the same element
heights[0] = 170
print(heights["alice"])           # 170 (updated)
```

### Sparse Naming

Not all elements need names - you can name some and leave others unnamed:

```glang
data = [10, 20, 30, 40]
data.set_names(["first", nil, "third", nil])

# Named elements accessible by name
first = data["first"]             # 10
third = data["third"]             # 30

# Unnamed elements only accessible by index
second = data[1]                  # 20 (no name)
fourth = data[3]                  # 40 (no name)

# Check names
names = data.get_names()          # ["first", nil, "third", nil]
```

### Individual Name Operations

```glang
scores = [85, 92, 78]

# Set individual names
scores.set_name(0, "math")
scores.set_name(2, "english")
# Leave index 1 unnamed

# Result: ["math", nil, "english"]
names = scores.get_names()

# Get individual name
math_name = scores.get_name(0)    # "math"
unnamed = scores.get_name(1)      # nil
```

### Graceful Degradation

Name-based access gracefully handles missing names:

```glang
heights = [165, 180, 175]
# No names set

# Accessing by non-existent name returns nil (no error)
alice = heights["alice"]          # nil
bob = heights["bob"]              # nil

# Setting value for non-existent name is ignored (no error)
heights["alice"] = 170            # Does nothing, no crash
```

## Advanced Usage

### Combining Names and General Metadata

```glang
measurements = [165, 180, 175]

# Set element names
measurements.set_names(["height", "weight", "age"])

# Set general metadata
measurements.metadata.set("units", "mixed")
measurements.metadata.set("subject", "patient_001")
measurements.metadata.set("timestamp", Time.now())

# Both work together
height = measurements["height"]                    # 165
units = measurements.metadata.get("units")        # "mixed"
subject = measurements.metadata.get("subject")    # "patient_001"
```

### Data Provenance and Versioning

```glang
experiment_data = [95.2, 87.1, 92.8]

# Track data provenance
experiment_data.metadata.set("experiment_id", "EXP-2025-001")
experiment_data.metadata.set("researcher", "Dr. Smith")
experiment_data.metadata.set("collection_date", "2025-01-15")
experiment_data.metadata.set("instrument", "Spectrometer-X200")
experiment_data.metadata.set("calibration_version", "v2.1.3")

# Set meaningful names
experiment_data.set_names(["sample_a", "sample_b", "sample_c"])

# Full context preserved
sample_a_result = experiment_data["sample_a"]     # 95.2
instrument = experiment_data.metadata.get("instrument")  # "Spectrometer-X200"
```

### Scientific Data Management

```glang
# Temperature readings with full metadata
temperatures = [20.5, 22.1, 21.8, 23.2]

# Element names (measurement points)
temperatures.set_names(["north_sensor", "south_sensor", "east_sensor", "west_sensor"])

# Measurement metadata
temperatures.metadata.set("units", "celsius")
temperatures.metadata.set("precision", 0.1)
temperatures.metadata.set("measurement_date", "2025-01-15")
temperatures.metadata.set("weather_conditions", "partly_cloudy")
temperatures.metadata.set("calibration_drift", -0.05)

# Access specific measurements
north_temp = temperatures["north_sensor"]         # 20.5
precision = temperatures.metadata.get("precision") # 0.1

# Quality control
drift = temperatures.metadata.get("calibration_drift")
if drift != 0 {
    print("Warning: sensor drift detected: " + drift.to_string())
}
```

## Implementation Details

### Architecture Components

1. **MetadataLayer**: Core key-value storage class
2. **GraphStructure**: Enhanced with `metadata` property
3. **SequentialGraph**: Element naming methods using metadata layer
4. **ListValue**: Exposes metadata API to Glang code

### Storage Format

Element names are stored as a special metadata property:
- Key: `"element_names"`
- Value: List of strings/nil for each element position

General metadata uses arbitrary keys:
```python
metadata.properties = {
    "element_names": ["alice", None, "charlie"],
    "units": "centimeters",
    "source": "survey_2025",
    "accuracy": 95.5
}
```

### Performance Characteristics

- **Memory Efficient**: Empty metadata has minimal overhead
- **Lazy Creation**: Metadata only allocated when needed
- **Fast Access**: Direct dictionary lookup for metadata
- **Sparse Naming**: Only stores names that exist

## Future Extensions

The metadata layer foundation enables future enhancements:

### Control Layer Integration
```glang
# Future: User-defined methods in metadata
data.metadata.set_method("validate", func() {
    # Custom validation logic
})
```

### Schema Validation
```glang
# Future: Schema enforcement
data.metadata.set("schema", {
    "type": "numeric_series",
    "range": [0, 100],
    "required_names": ["min", "max", "mean"]
})
```

### Serialization Support
```glang
# Future: Layer-aware serialization
data.save("experiment.glr", layers: ["data", "metadata"])
data.save("public.glr", layers: ["data"], exclude_metadata: ["internal_notes"])
```

## Best Practices

### Naming Conventions

1. **Use descriptive names**: `["baseline", "treatment", "control"]` not `["a", "b", "c"]`
2. **Be consistent**: Choose a naming pattern and stick to it
3. **Consider domain conventions**: Use field-standard names when possible

### Metadata Organization

1. **Namespace related metadata**: Use prefixes like `"measurement_"`, `"analysis_"`
2. **Include provenance**: Always track data source and collection method
3. **Version metadata schemas**: Include version info for compatibility

### Performance Tips

1. **Lazy naming**: Only add names when you need name-based access
2. **Minimal metadata**: Don't store derived values that can be computed
3. **Batch operations**: Set all names at once with `set_names()` when possible

## Examples

See the test suite in `test/test_metadata_layer.py` for comprehensive usage examples covering all functionality.
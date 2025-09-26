# CSV Module Documentation

The CSV module provides basic functionality for parsing and generating CSV (Comma-Separated Values) data in Glang programs.

## Overview

This is a simplified CSV implementation written in pure Glang, demonstrating the language's ability to implement real-world functionality without relying on Python modules. The current implementation provides basic CSV parsing and generation capabilities.

## Importing the Module

```glang
import "csv" as csv
```

## Core Functions

### `csv.parse(csv_text, has_headers, delimiter)`

Parses CSV text into a list of lists.

**Parameters:**
- `csv_text` (string): The CSV content to parse
- `has_headers` (bool): Whether the first row contains headers (currently ignored)
- `delimiter` (string): Field delimiter. Use `none` for default comma delimiter

**Returns:**
- List of lists, where each inner list contains the row values as strings
- Currently only returns the first line of multiline CSV (limitation of current implementation)

**Examples:**

```glang
# Parse single line CSV
csv_data = "Alice,25,Engineer"
result = csv.parse(csv_data, false, ",")
# Returns: [["Alice", "25", "Engineer"]]

first_row = result[0]
name = first_row[0]  # "Alice"
age = first_row[1]   # "25"

# Parse with custom delimiter
tab_data = "Alice\t25\tEngineer"
result = csv.parse(tab_data, false, "\t")
# Returns: [["Alice", "25", "Engineer"]]
```

### `csv.generate(items, headers, delimiter)`

Generates CSV text from a list of lists.

**Parameters:**
- `items` (list): List of lists where each inner list represents a row
- `headers` (list): Optional header row. Use `none` to skip headers
- `delimiter` (string): Field delimiter. Use `none` for default comma delimiter

**Returns:**
- String containing the generated CSV content

**Examples:**

```glang
# Generate CSV from data
data = [
    ["Alice", "25", "Engineer"],
    ["Bob", "30", "Designer"]
]
headers = ["name", "age", "role"]
csv_output = csv.generate(data, headers, ",")
# Returns: "name,age,role\nAlice,25,Engineer\nBob,30,Designer"

# Generate without headers
csv_output = csv.generate(data, none, ",")
# Returns: "Alice,25,Engineer\nBob,30,Designer"

# Use custom delimiter
csv_output = csv.generate(data, headers, ";")
# Returns: "name;age;role\nAlice;25;Engineer\nBob;30;Designer"
```

### `csv.validate(csv_text, delimiter)`

Provides basic validation information about CSV structure.

**Parameters:**
- `csv_text` (string): CSV content to validate
- `delimiter` (string): Expected delimiter (currently not used)

**Returns:**
- Hash containing basic validation results:
  - `valid` (bool): Always returns `true` in current implementation
  - `rows` (num): Number of rows (simplified calculation)
  - `columns` (num): Number of columns (fixed at 2 in current implementation)

**Example:**

```glang
csv_data = "name,age\nAlice,25"
validation = csv.validate(csv_data, ",")
print("Valid: " + validation["valid"].to_string())     # "true"
print("Rows: " + validation["rows"].to_string())       # "1"
print("Columns: " + validation["columns"].to_string()) # "2"
```

## Current Limitations

This is a simplified implementation with the following limitations:

1. **Multiline Parsing**: Only processes the first line of multiline CSV data
2. **No Type Conversion**: All values are returned as strings
3. **Basic Validation**: The validate function provides minimal information
4. **No Auto-Detection**: Delimiters and headers must be specified explicitly
5. **No Quote Handling**: Does not handle quoted fields or escaped characters

## Practical Examples

### Basic Data Processing

```glang
import "csv" as csv

# Generate sample data
data = [
    ["Product A", "100", "Electronics"],
    ["Product B", "200", "Books"],
    ["Product C", "150", "Electronics"]
]

# Create CSV with headers
headers = ["name", "price", "category"]
csv_content = csv.generate(data, headers, ",")
print("Generated CSV:")
print(csv_content)

# Parse it back
parsed = csv.parse(csv_content, true, ",")
print("Parsed " + parsed.size().to_string() + " rows")
```

### File Integration

```glang
import "csv" as csv
import "io"

# Read CSV from file (assuming it exists)
file_handle = io.open("data.csv", "r")
csv_content = file_handle.read()

# Parse the content
rows = csv.parse(csv_content, false, ",")
print("Loaded " + rows.size().to_string() + " rows")

# Process and generate new CSV
processed_data = []
for row in rows {
    # Simple processing example
    processed_data.append(row)
}

# Save processed data
output_csv = csv.generate(processed_data, none, ",")
output_handle = io.open("output.csv", "w")
output_handle.write(output_csv)
output_handle.close()
```

## Development Notes

This CSV module demonstrates:

1. **Pure Glang Implementation**: Written entirely in Glang without Python dependencies
2. **Self-Hosting Philosophy**: Validates Glang's expressiveness for real-world tasks
3. **Modular Design**: Clean separation of parsing, generation, and helper functions
4. **Type Safety**: Works within Glang's type system constraints

## Future Enhancements

Planned improvements for future versions:

- Full multiline CSV parsing support
- Automatic type conversion for numbers and booleans
- Proper quote handling and field escaping
- Header detection and hash-based parsing results
- Enhanced validation with detailed error reporting
- Pretty formatting for human-readable output

## Integration with Other Modules

```glang
import "csv" as csv
import "json"

# Convert CSV to JSON
csv_data = "name,age\nAlice,25\nBob,30"
rows = csv.parse(csv_data, true, ",")

# Simple JSON conversion (manual for now)
json_data = json.encode(rows)
print("JSON output: " + json_data)
```
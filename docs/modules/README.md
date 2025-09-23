# Glang Standard Library Modules

This directory contains comprehensive documentation for all Glang standard library modules. These modules provide essential functionality for real-world application development.

## Available Modules

### Core Modules

#### [Time Module](time.md)
Date and time operations with UTC timestamps and full type casting support.
```glang
import "time" as Time
current = Time.now()
birthday = Time.from_components(1990, 12, 25)
```

#### [JSON Module](json.md) 
Data serialization, deserialization, and validation with pretty printing.
```glang
import "json"
data = {"name": "Alice", "age": 30}
json_text = json.encode(data)
parsed = json.decode(json_text)
```

#### [I/O Module](io.md)
File operations, console I/O, network requests, and file system operations.
```glang
import "io"
content = io.read_file("data.txt")
user_name = io.input("Enter name: ")
```

### Data Processing Modules

#### [CSV Module](csv.md) ⭐ NEW
Basic CSV parsing and generation with pure Glang implementation.
```glang
import "csv" as csv
data = [["Alice", "25"], ["Bob", "30"]]
csv_output = csv.generate(data, ["name", "age"], ",")
parsed = csv.parse(csv_output, true, ",")
```

### Text Processing Modules

#### [Regex Module](regex.md) ⭐ NEW
Pattern matching, text extraction, and validation with comprehensive regular expression support.
```glang
import "regex"
phone_pattern = "\\d{3}-\\d{3}-\\d{4}"
is_valid = regex.validate(phone_pattern, "555-123-4567")
emails = regex.find_all("\\b\\w+@\\w+\\.\\w+\\b", text)
```

### Utility Modules

#### [Benchmark Module](../stdlib/benchmark.md) ⭐ NEW
Performance measurement and timing analysis with function parameter support.
```glang
load "stdlib/benchmark.gr"
timing = time_operation(my_function, 10)
print("Ops/sec: " + timing["operations_per_second"].to_string())
```

#### [Random Module](random.md) ⭐ NEW
Random number generation, statistical distributions, sampling, and cryptographic randomness.
```glang
import "random" as rand
dice_roll = rand.randint(1, 6)
sample_data = rand.sample(population, 100)
secure_token = rand.secure_token(32)
```

#### [Crypto Module](crypto.md)
Cryptographic operations including hashing, encryption, and secure data handling.
```glang
import "crypto"
password_hash = crypto.sha256("password123")
secure_data = crypto.encrypt(plaintext, key)
```

## Module Usage Patterns

### Import Styles
```glang
# Standard import (use module name as prefix)
import "json"
result = json.encode(data)

# Import with alias (custom prefix)
import "random" as rand
number = rand.randint(1, 100)

# Import time with conventional alias
import "time" as Time
now = Time.now()
```

### Error Handling
```glang
import "json"
import "io"

try {
    content = io.read_file("config.json")
    config = json.decode(content)
} catch error {
    io.print("Failed to load config: " + error.message())
}
```

### Module Combinations
```glang
import "json"
import "io" 
import "time" as Time

# Save timestamped data
data = {
    "timestamp": Time.now().to_string(),
    "events": ["login", "purchase", "logout"]
}
json_data = json.encode(data)
io.write_file("events.json", json_data)
```

## Development Status

| Module | Status | Features | Documentation |
|--------|--------|----------|---------------|
| **time** | ✅ Complete | UTC timestamps, type casting, component parsing | [Full docs](time.md) |
| **json** | ✅ Complete | Encode/decode, validation, pretty printing | [Full docs](json.md) |
| **io** | ✅ Complete | File I/O, console, network, file system | [Full docs](io.md) |
| **csv** | 🚧 Basic | Parse/generate CSV, pure Glang implementation | [Full docs](csv.md) |
| **regex** | ✅ Complete | Pattern matching, extraction, validation | [Full docs](regex.md) |
| **random** | ✅ Complete | RNG, distributions, sampling, security | [Full docs](random.md) |
| **crypto** | ✅ Complete | Hashing, encryption, secure operations | [Full docs](crypto.md) |
| **benchmark** | ✅ Complete | Function timing, performance analysis, comparisons | [Full docs](../stdlib/benchmark.md) |

## Design Philosophy

Glang modules are designed with these principles:

1. **Practical First** - Solve real-world problems with clean APIs
2. **Type Safe** - Full integration with Glang's type system
3. **Error Transparent** - Clear error messages with helpful context
4. **Consistent Interface** - Similar patterns across all modules
5. **Performance Focused** - Efficient implementations with caching where appropriate

## Getting Help

- Check individual module documentation for detailed API references
- See [Language Cheat Sheet](../GLANG_CHEAT_SHEET.md) for quick syntax reference
- Use REPL commands like `/methods module_name` to explore module functionality
- Consult [behaviors documentation](../behaviors.md) for data validation and transformation

## Contributing

When adding new modules:
1. Follow the established documentation format (see existing modules)
2. Provide comprehensive examples and use cases
3. Include error handling patterns
4. Add integration examples with other modules
5. Ensure full test coverage
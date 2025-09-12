# Glang Language Cheat Sheet 🚀

*A quick reference for the Glang programming language*

## 📝 Variable Declarations

```glang
# Type inference (recommended)
name = "Alice"              # string
age = 25                    # num  
active = true               # bool
items = [1, 2, 3]          # list

# Explicit types (when needed)
string username = "Bob"
num score = 95.5
bool is_valid = false
list<num> grades = [90, 85, 92]
```

## 📊 Data Structures

### Lists
```glang
# Creation
numbers = [1, 2, 3, 4, 5]
names = ["alice", "bob", "charlie"]
mixed = [1, "hello", true]

# Type-constrained lists
list<string> cities = ["NYC", "LA", "Chicago"]
list<num> temperatures = [72.5, 68.2, 75.1]

# Access & modification
numbers[0]                 # Get: 1
numbers[0] = 99           # Set
numbers.append(6)         # Add to end
numbers.size()            # Get length: 6
```

### Data Nodes (Key-Value Pairs)
```glang
# Creation
user = { "name": "Alice" }
config = { "port": 8080 }
data<num> score = { "final": 95 }

# Access
user.key()                # Get key: "name"  
user.value()              # Get value: "Alice"
```

### Hashes (Collections of Data Nodes)
```glang
# Creation  
settings = { "theme": "dark", "lang": "en", "debug": true }
hash<string> prefs = { "color": "blue", "font": "arial" }

# Access & modification
settings["theme"]         # Get: { "theme": "dark" }
settings["theme"] = "light"  # Set
settings.get("theme")     # Get data node
settings.has_key("debug") # Check existence: true
settings.keys()           # Get all keys as list
settings.values()         # Get all values as list
```

## 🔄 Control Flow

### If/Else Statements
```glang
if condition {
    # execute when true
}

if score > 90 {
    grade = "A"
} else {
    grade = "B"
}

# With method chaining
if numbers.filter("even").size() > 0 {
    print("Found even numbers!")
}
```

### Precision Context Blocks
```glang
# Control numeric precision for calculations (decimal places)
precision 5 {
    # All arithmetic uses 5 decimal places precision  
    pi = 3.14159265358979323846  # Result: 3.14159 (5 decimal places)
    area = pi * radius * radius  # All calculations use 5 decimal places
}

# Integer arithmetic with precision 0
precision 0 {
    pi = 3.14159265358979323846  # Result: 3 (integer, no decimal point)
    area = pi * 10 * 10          # Result: 300 (integer arithmetic)
}

# Financial calculations with 2 decimal places
precision 2 {
    price = 19.99
    tax = price * 0.085          # Result: 1.70 (exactly 2 decimal places)
    total = price + tax          # Result: 21.69 (exactly 2 decimal places)
}

# Nested precision contexts
precision 3 {
    outer = 22.0 / 7.0           # Result: 3.143 (3 decimal places)
    
    precision 1 {
        inner = 22.0 / 7.0       # Result: 3.1 (1 decimal place)
    }
    
    back = 22.0 / 7.0            # Result: 3.143 (3 decimal places restored)
}
```

### Loops
```glang
# While loops
counter = 0
while counter < 5 {
    print(counter)
    counter = counter + 1
}

# For-in loops
for item in items {
    print(item)
}

for name in ["alice", "bob", "charlie"] {
    print(name.up())
}

# Loop control
for item in items {
    if item == 5 {
        break      # Exit loop
    }
    if item % 2 == 0 {
        continue   # Skip to next iteration
    }
    print(item)
}
```

## ⚡ Functions & Lambda Expressions

### Function Declarations
```glang
# Basic function
func greet(name) {
    return "Hello, " + name + "!"
}

# Function with multiple parameters
func add(x, y) {
    return x + y
}

# Function without return (returns none)
func say_hello() {
    print("Hello from function!")
}

# Function with early returns
func find_max(a, b) {
    if a >= b {
        return a
    }
    return b
}
```

### Function Calls
```glang
# Calling functions
message = greet("World")      # "Hello, World!"
result = add(15, 27)          # 42
say_hello()                   # Prints and returns none

# Recursive functions
func fibonacci(n) {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

fib_result = fibonacci(6)     # 8
```

### Lambda Expressions
```glang
# Single parameter lambda
double = x => x * 2
result = double(5)            # 10

# Multiple parameter lambda
multiply = (x, y) => x * y
product = multiply(7, 8)      # 56

# Lambda with complex expressions
square_and_add = x => x * x + 1
value = square_and_add(4)     # 17

# Using lambdas directly
numbers = [1, 2, 3, 4, 5]
squared = numbers.map(x => x * x)  # [1, 4, 9, 16, 25]
```

### Function Integration
```glang
# Functions work with all language features
func process_list(items, threshold) {
    result = []
    for item in items {
        if item > threshold {
            result.append(item * 2)
        }
    }
    return result
}

data = [1, 2, 3, 4, 5]
processed = process_list(data, 2)  # [6, 8, 10]

# Functions can be assigned to variables
operation = add
sum_result = operation(10, 20)     # 30
```

## 🎯 Functional Programming

### Map (Transform Elements)
```glang
numbers = [1, 2, 3, 4]
numbers.map("double")     # [2, 4, 6, 8]
names.map("upper")        # ["ALICE", "BOB"]
scores.map("to_string")   # Convert to strings

# Available transformations:
# Numeric: double, square, negate, increment, decrement
# String: upper, lower, trim, reverse  
# Conversion: to_string, to_num, to_bool
```

### Filter (Select Elements)
```glang
numbers = [1, 2, 3, 4, 5, 6]
numbers.filter("even")    # [2, 4, 6]
numbers.filter("positive") # [1, 2, 3, 4, 5, 6]
names.filter("non_empty") # Remove empty strings

# Available predicates:
# Numeric: positive, negative, zero, even, odd
# String/Collection: empty, non_empty, uppercase, lowercase
# Type: is_string, is_number, is_bool, is_list
# General: truthy, falsy
```

### Other Functional Methods
```glang
numbers.each("print")     # Execute action on each element
numbers.select("even")    # Alias for filter
numbers.reject("odd")     # Opposite of filter

# Method chaining
numbers.filter("positive").map("double").each("print")
```

## 🧮 Mathematical Methods

### Number Methods
```glang
x = 16
x.abs()                   # Absolute value: 16
x.sqrt()                  # Square root: 4.0
x.log()                   # Natural logarithm: 2.77...
x.log(10)                # Log base 10: 1.20...
x.pow(2)                  # Power: 256
x.to(2)                   # Truncate to 2 decimals

# Rounding methods
y = 3.14159
y.rnd()                   # Round to nearest: 3
y.rnd(2)                  # Round to 2 places: 3.14
y.rnd_up()               # Round up (ceiling): 4
y.rnd_up(2)              # Round up to 2 places: 3.15
y.rnd_dwn()              # Round down (floor): 3
y.rnd_dwn(2)             # Round down to 2 places: 3.14
```

### Boolean Methods
```glang
flag = true
flag.flip()               # Toggle: false
flag.toggle()             # Alias for flip: false
flag.numify()             # Convert to number: 1
flag.toNum()              # Alias for numify: 1
```

### Mathematical Constants
```glang
load "stdlib/math.gr"     # Load mathematical constants
print(pi)                 # 3.141592653589793
print(e)                  # 2.718281828459045

# Example: Calculate circle area
radius = 5
area = pi * radius.pow(2)
print("Area: " + area.to_string())
```

## 🔄 Type Casting

### Convert Any Type to String
```glang
(42).to_string()          # "42"
true.to_string()          # "true"
[1, 2, 3].to_string()     # "[1, 2, 3]"
```

### Convert to Numbers
```glang
"123".to_num()            # 123
"3.14".to_num()           # 3.14
true.to_num()             # 1
false.to_num()            # 0
```

### Convert to Booleans
```glang
(42).to_bool()            # true (non-zero)
(0).to_bool()             # false (zero)
"hello".to_bool()         # true (non-empty)
"".to_bool()              # false (empty)
[1, 2].to_bool()          # true (non-empty)
[].to_bool()              # false (empty)
```

### Convert to Time (and back)
```glang
# Number (Unix timestamp) to Time
timestamp = 1735689600
time_value = timestamp.to_time()  # 2025-01-01T00:00:00Z

# String (ISO format) to Time  
iso_string = "2025-01-15T14:30:00"
parsed_time = iso_string.to_time() # 2025-01-15T14:30:00Z

# Time to Number (timestamp)
import "time" as Time
current = Time.now()
timestamp = current.to_num()      # Unix timestamp

# Time to String (ISO format) 
iso_format = current.to_string()  # "2025-01-15T14:30:00Z"
```

### Chained Conversions
```glang
x = 42
result = x.to_string().to_bool()  # true
b = true.to_num().to_string()     # "1"

# Time casting chains
import "time" as Time
now = Time.now()
round_trip = now.to_num().to_time().to_string()  # Perfect consistency
```

## 🔤 String Operations

### Text Manipulation
```glang
text = "Hello, World!"

# Case conversion
text.up()                  # "HELLO, WORLD!"
text.down()                # "hello, world!"

# Whitespace handling
spaced = "  hello  "
spaced.trim()              # "hello"

# String analysis
text.chars()               # ["H", "e", "l", "l", "o", ",", " ", "W", "o", "r", "l", "d", "!"]
text.length()              # 13
text.contains("World")     # true
```

### String Splitting
```glang
# Split by spaces (default)
sentence = "hello world test"
words = sentence.split()   # ["hello", "world", "test"]

# Split by custom delimiter
csv = "apple,banana,cherry"
fruits = csv.split(",")    # ["apple", "banana", "cherry"]

# Split by newlines (great for file processing!)
content = "line1\nline2\nline3"
lines = content.split("\n")  # ["line1", "line2", "line3"]

# Split by any character
path = "folder/subfolder/file.txt"
parts = path.split("/")    # ["folder", "subfolder", "file.txt"]
```

### Practical String Processing
```glang
# Process file content line by line
content = io.read_file("data.txt")
lines = content.split("\n")
for line in lines {
    if line.trim() != "" {
        processed = line.trim().up()
        io.print("Processed: " + processed)
    }
}

# Parse CSV-like data
data = "John,25,Engineer\nJane,30,Designer"
lines = data.split("\n")
for line in lines {
    fields = line.split(",")
    name = fields[0]
    age = fields[1].to_num()
    job = fields[2]
    io.print(name + " is " + age.to_string() + " years old")
}
```

## 🔒 Data Immutability System

### Freezing Data
```glang
# Make any value immutable
name = "Alice"
name.freeze()              # Returns self for chaining
print(name.is_frozen())    # true

# Chaining example
config = { "host": "localhost", "port": 8080 }
config.freeze().inspect()  # Freeze and inspect in one line
```

### Deep Freezing Collections
```glang
# Freezing collections also freezes their contents
items = [1, 2, 3]
items.freeze()             # All elements become frozen too
print(items.contains_frozen())  # true

# Same for hashes and data nodes
user = { "name": "Alice", "age": 25 }
user.freeze()              # All values become frozen
```

### Contamination Rules (Strict Separation)
```glang
# Frozen and unfrozen data cannot mix
list1 = [1, 2, 3]         # Unfrozen list
item = "hello"
item.freeze()              # Frozen string

# This will throw an error:
# list1.append(item)       # Cannot mix frozen/unfrozen data

# Check compatibility before operations
if list1.can_accept(item) {
    list1.append(item)
} else {
    print("Cannot add frozen item to unfrozen list")
}
```

### Mutation Prevention
```glang
config = { "debug": true }
config.freeze()

# These will throw runtime errors:
# config["debug"] = false  # Cannot modify frozen hash
# config.set("port", 8080) # Cannot add to frozen hash

# Use is_frozen() to check before mutations
if !config.is_frozen() {
    config["new_key"] = "value"
}
```

### Immutability Methods (Available on All Types)
```glang
# Universal methods
value.freeze()             # Make immutable (returns self)
value.is_frozen()          # Check if frozen: true/false
value.contains_frozen()    # Check if contains frozen data

# Collection-specific methods  
list.can_accept(item)      # Check if item can be added
hash.can_accept(value)     # Check if value can be stored
data.can_accept(value)     # Check if value can be set
```

### Practical Use Cases
```glang
# Configuration that shouldn't change
config = { "api_key": "secret", "endpoint": "api.com" }
config.freeze()            # Prevent accidental modifications

# Immutable data structures
numbers = [1, 2, 3, 4, 5]
frozen_numbers = numbers.freeze()  # Prevent mutations
# Can still read, but not modify

# Safe data sharing
def process_data(data) {
    if data.is_frozen() {
        # Safe to use without worrying about modifications
        return data.map("double")
    } else {
        # Make a frozen copy to prevent side effects  
        return data.freeze().map("double")
    }
}
```

## 🔢 Operators & Comparisons

### Arithmetic
```glang
a + b     # Addition
a - b     # Subtraction  
a * b     # Multiplication
a / b     # Division
a % b     # Modulo
```

### Comparisons
```glang
a == b    # Equal
a != b    # Not equal
a > b     # Greater than
a < b     # Less than
a >= b    # Greater or equal
a <= b    # Less or equal
```

### List Arithmetic
```glang
[1, 2] + [3, 4]          # [1, 2, 3, 4] (concatenation)
[1, 2, 3] * 2            # [2, 4, 6] (scalar multiplication)
[1, 2, 3] + 5            # [6, 7, 8] (scalar addition)
```

## 📁 File Operations

### Loading Glang Files
```glang
# Load another .gr file into current scope
load "config.gr"         # Variables become available directly

# REPL file commands  
/load config.gr          # Load file in REPL
/save myprogram.gr       # Save current session
/run example.gr          # Run file in fresh session
```

## 📦 Module System

### Importing Modules
```glang
# Import built-in modules
import "io"               # Import I/O module as 'io'
import "math" as calc     # Import with custom alias

# Import .gr modules (with module declaration)
import "my_library.gr"    # Uses alias from module file
import "utils.gr" as tools # Override with custom alias
```

### Creating Modules
```glang
# In a .gr file (e.g., math_utils.gr):
module mathematical_utilities  # Full module name
alias math                    # Default import alias

# Module variables and functions
pi = 3.14159
e = 2.71828

func circle_area(radius) {
    return pi * radius * radius
}

# Usage after import:
import "math_utils.gr"
area = math.circle_area(5.0)
```

## 📤 Input/Output Operations

### Console Output
```glang
import "io"

# Print with newline (default)
io.print("Hello, World!")       # Prints with newline
io.print(42)                    # Prints with newline
io.print([1, 2, 3])             # Prints with newline

# Print without newline (pass false)
io.print("Enter name: ", false) # No newline
io.print("Value: " + value.to_string(), false)

# Explicit newline control
io.print("Name: " + name + ", Age: " + age.to_string(), true)  # With newline
io.print("Processing...", false)  # Continue on same line
io.print(" Done!")               # Finish the line
```

### File Operations
```glang
import "io"

# Read entire file
content = io.read_file("data.txt")
print("File content: " + content)

# Write to file (overwrites existing)
data = "Hello, World!\nLine 2\nLine 3"
io.write_file("output.txt", data)

# Append to file
io.append_file("log.txt", "New log entry\n")
```

### User Input
```glang
import "io"

# Get user input
name = io.input("Enter your name: ")
io.print("Hello, " + name + "!")

# Input with type conversion
age_str = io.input("Enter your age: ")
age = age_str.to_num()
io.print("You are " + age.to_string() + " years old")
```

### File System Operations
```glang
import "io"

# Check if file exists
if io.exists("config.txt") {
    config = io.read_file("config.txt")
} else {
    io.print("Config file not found!")
}

# Get file size
size = io.file_size("data.csv")
io.print("File size: " + size.to_string() + " bytes")

# List directory contents
files = io.list_dir(".")
for file in files {
    io.print("Found: " + file)
}
```

### Network Operations (v0.5+)
```glang
import "io"

# HTTP GET request
response = io.http_get("https://api.example.com/data")

# HTTP POST request
result = io.http_post("https://api.service.com/submit", "key=value")

# Download files
success = io.download_file("https://example.com/file.txt", "local_file.txt")

# Web API integration
api_response = io.http_get("https://jsonplaceholder.typicode.com/users/1") 
user_data = json.decode(api_response)
name = user_data.get("name").value()
```

## 🔗 JSON Operations

### Importing JSON Module
```glang
import "json"               # Import JSON module
import "io"                 # Often needed for file operations
```

### Quick JSON Reference
```glang
# Essential JSON operations:
json.encode(data)           # Convert to JSON string
json.encode_pretty(data)    # Convert to formatted JSON  
json.decode(json_string)    # Parse JSON to Glang values
json.is_valid(json_string)  # Check if valid JSON

# Complete file workflow:
# 1. Create data → 2. Encode to JSON → 3. Write to file
```

## ⏰ Time Operations

### Importing Time Module
```glang
import "time" as Time       # Import Time module
```

### Working with Time Values
```glang
# Creating times
current = Time.now()                              # Current time
today = Time.today()                             # Start of today (UTC)
birthday = Time.from_components(1990, 12, 25)   # Date only
meeting = Time.from_components(2025, 1, 15, 14, 30, 0) # Full datetime
parsed = Time.from_string("2025-01-15T14:30:00") # From ISO string

# Using time values (parentheses optional for zero-arg methods)
print(current.to_string())   # "2025-01-15T14:30:00Z" (ISO format)
print(current.to_string)     # Same result, more elegant syntax
print(current.get_type())    # "time"  
print(current.get_type)      # "time" - property-like access

# Type casting (bidirectional)
timestamp = current.to_num()               # Time → Number (Unix timestamp)
time_from_num = timestamp.to_time()        # Number → Time
time_from_str = "2025-01-15T14:30:00".to_time() # String → Time

# Round-trip consistency guaranteed
original = Time.now()
round_trip = original.to_num().to_time()
print(original.to_string() == round_trip.to_string()) # true
# 4. Read file → 5. Validate → 6. Decode JSON → 7. Use data
```

### Encoding to JSON
```glang
# Encode Glang values to JSON strings
data = { "name": "Alice", "age": 25, "active": true }
json_string = json.encode(data)
io.print(json_string)      # {"name":"Alice","age":25,"active":true}

# Pretty formatting with indentation
pretty_json = json.encode_pretty(data)
io.print(pretty_json)
# {
#   "name": "Alice", 
#   "age": 25,
#   "active": true
# }

# Encode different data types
numbers = [1, 2, 3, 4, 5]
json.encode(numbers)       # [1,2,3,4,5]

name = "Hello World"
json.encode(name)          # "Hello World"

flag = true
json.encode(flag)          # true
```

### Decoding from JSON
```glang
# Decode JSON strings to Glang values
json_data = '{"name": "Bob", "score": 95}'
parsed = json.decode(json_data)

# Access decoded data
name_data = parsed["name"]
io.print(name_data.value())  # "Bob"

score_data = parsed["score"] 
io.print(score_data.value()) # 95

# Decode arrays
json_array = "[1, 2, 3, 4]"
numbers = json.decode(json_array)
io.print(numbers[0])       # 1

# Decode simple values
json.decode("42")          # 42
json.decode("true")        # true
json.decode('"hello"')     # "hello"
```

### JSON Validation
```glang
# Check if string is valid JSON
valid_json = '{"key": "value"}'
is_valid = json.is_valid(valid_json)
io.print(is_valid)         # true

invalid_json = "not json"
is_valid = json.is_valid(invalid_json)
io.print(is_valid)         # false

# Use before parsing
json_text = io.input("Enter JSON: ")
if json.is_valid(json_text) {
    data = json.decode(json_text)
    io.print("Parsed successfully!")
} else {
    io.print("Invalid JSON format")
}
```

### Creating and Writing JSON Files
```glang
import "json"
import "io"

# Create data structure (build nested objects first)
preferences = { "theme": "dark", "notifications": true }
scores = [95, 87, 92]

user_data = {
    "name": "Alice Johnson",
    "age": 28,
    "email": "alice@example.com",
    "preferences": preferences,
    "scores": scores
}

# Convert to JSON and write to file
json_string = json.encode_pretty(user_data)
io.write_file("user.json", json_string)
io.print("User data saved to user.json")

# For compact JSON (no formatting):
compact_json = json.encode(user_data)
io.write_file("user_compact.json", compact_json)
```

### Reading and Parsing JSON Files
```glang
import "json"
import "io"

# Read JSON file and parse it
if io.exists("user.json") {
    file_content = io.read_file("user.json")
    
    # Validate before parsing
    if json.is_valid(file_content) {
        user = json.decode(file_content)
        
        # Access data (use explicit types for JSON decoded values)
        string name = user["name"].value()
        num age = user["age"].value()
        string theme = user["preferences"]["theme"].value()
        
        io.print("User: " + name + ", Age: " + age.to_string())
        io.print("Theme: " + theme)
        
        # Process array data
        scores = user["scores"].value()
        io.print("Scores: " + scores.to_string())
    } else {
        io.print("Invalid JSON in file")
    }
} else {
    io.print("File not found")
}
```

### Complete JSON File Workflow
```glang
import "json"
import "io"

# Step 1: Create configuration data
database_config = { "host": "localhost", "port": 5432, "name": "myapp" }
features = ["auth", "logging", "cache"]

config = {
    "database": database_config,
    "features": features,
    "debug": true,
    "version": "1.0.0"
}

# Step 2: Save to JSON file
io.print("Saving configuration...")
config_json = json.encode_pretty(config)
io.write_file("config.json", config_json)
io.print("Configuration saved to config.json")

# Step 3: Later, load and use the configuration
io.print("Loading configuration...")
if io.exists("config.json") {
    content = io.read_file("config.json")
    if json.is_valid(content) {
        loaded_config = json.decode(content)
        
        # Extract configuration values (explicit types recommended)
        string db_host = loaded_config["database"]["host"].value()
        num db_port = loaded_config["database"]["port"].value()
        bool debug_mode = loaded_config["debug"].value()
        
        io.print("Database: " + db_host + ":" + db_port.to_string())
        io.print("Debug mode: " + debug_mode.to_string())
        
        # Process features array
        features = loaded_config["features"].value()
        io.print("Enabled features:")
        for feature in features.elements {
            io.print("  - " + feature)
        }
    }
}

```

### API Data Processing with JSON
```glang
import "json"
import "io"

# Simulate API response
api_response = '{"users": [{"name": "Alice", "id": 1}, {"name": "Bob", "id": 2}]}'
data = json.decode(api_response)
users = data["users"].value()  # Get the array

for user in users.elements {
    string user_name = user["name"].value()
    num user_id = user["id"].value()
    io.print("User: " + user_name + " (ID: " + user_id.to_string() + ")")
}

# Save processed data to file
processed_users = []
for user in users.elements {
    string name = user["name"].value()
    num id = user["id"].value()
    user_info = {
        "display_name": name.up(),
        "user_id": id
    }
    processed_users.append(user_info)
}

output_data = { "processed_users": processed_users }
output_json = json.encode_pretty(output_data)
io.write_file("processed_users.json", output_json)
```

### JSON Type Conversions
```glang
# Round-trip conversion (Glang -> JSON -> Glang)
original = { "items": [1, 2, 3], "text": "hello", "flag": true }

# Convert to JSON and back
json_str = json.encode(original)
decoded = json.decode(json_str)

# Data preserves structure and types (use explicit types for clarity)
list items = decoded["items"].value()   # List with [1, 2, 3]
string text = decoded["text"].value()   # String "hello"  
bool flag = decoded["flag"].value()     # Boolean true
```

## 🛠️ REPL Commands

### Essential Commands
```
/help or /h              # Show help
/exit or /x              # Exit REPL
/version or /ver         # Show version
```

### Variable Inspection
```
/namespace or /ns        # Show all variables
/type varname            # Show type info
/methods varname         # Show available methods
```

### Session Management
```
/stats                   # Show session statistics
/clear                   # Clear all variables
```

## 💡 Quick Examples

### Data Processing
```glang
# Process a list of numbers
data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
evens = data.filter("even")
doubled = evens.map("double")
print(doubled)  # [4, 8, 12, 16, 20]
```

### Configuration Management
```glang
config = { "host": "localhost", "port": 8080, "debug": true }

if config.has_key("debug") {
    if config["debug"].value() {
        print("Debug mode enabled")
    }
}
```

### Complex Control Flow
```glang
matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
result = []

for row in matrix {
    for item in row {
        if item % 2 == 1 {  # Odd numbers
            result.append(item)
        }
    }
}
print(result)  # [1, 3, 5, 7, 9]
```

### Mathematical Calculations
```glang
# Load mathematical constants
load "stdlib/math.gr"

# Calculate various formulas
radius = 7.5
area = pi * radius.pow(2)
circumference = 2 * pi * radius

print("Circle with radius " + radius.to_string())
print("Area: " + area.rnd(2).to_string())
print("Circumference: " + circumference.rnd(2).to_string())

# Temperature conversion
celsius = 25
fahrenheit = celsius * 9 / 5 + 32
print(celsius.to_string() + "°C = " + fahrenheit.to_string() + "°F")
```

### Type Conversion Pipeline with Lambdas
```glang
# Data processing with type conversion and custom functions
scores = ["95", "87", "92", "76", "88"]

# Convert to numbers, filter, and format using lambdas
high_scores = scores.map(s => s.to_num())
                   .filter(s => s > 90)
                   .map(s => s.to_string())

for score in high_scores {
    print("High score: " + score)
}

# Custom function for grading
func calculate_grade(score) {
    if score >= 90 {
        return "A"
    } else if score >= 80 {
        return "B" 
    } else {
        return "C"
    }
}

grades = scores.map(s => calculate_grade(s.to_num()))
print("Grades: " + grades.to_string())
```

### Nested Functional Operations
```glang
numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

# Complex pipeline with mathematical operations
processed = numbers.filter("even")
                  .map(x => x.pow(2))
                  .filter(x => x > 10)
                  .map(x => x.sqrt().rnd(2))

print("Processed: " + processed.to_string())
```

### File I/O with Error Handling
```glang
import "io"

# Read and process a data file
filename = "scores.txt"
if io.exists(filename) {
    content = io.read_file(filename)
    lines = content.split("\n")
    
    total = 0
    count = 0
    
    for line in lines {
        if line != "" {
            score = line.to_num()
            total = total + score
            count = count + 1
        }
    }
    
    if count > 0 {
        average = total / count
        io.print("Average score: " + average.rnd(2).to_string())
        
        # Write result to new file
        result = "Total scores: " + count.to_string() + "\n"
        result = result + "Average: " + average.rnd(2).to_string()
        io.write_file("result.txt", result)
    }
} else {
    io.print("File not found: " + filename)
}
```

### Module-Based Calculator
```glang
# calculator.gr module file:
module advanced_calculator
alias calc

func add(a, b) {
    return a + b
}

func multiply(a, b) {
    return a * b
}

func power(base, exponent) {
    return base.pow(exponent)
}

# Main program:
import "calculator.gr"
import "io"

a = io.input("Enter first number: ").to_num()
b = io.input("Enter second number: ").to_num()

sum = calc.add(a, b)
product = calc.multiply(a, b)
power_result = calc.power(a, 2)

io.print("Sum: " + sum.to_string())
io.print("Product: " + product.to_string())
io.print(a.to_string() + " squared: " + power_result.to_string())
```

---

## 🎓 Pro Tips

1. **Type Inference**: Let Glang infer types when obvious: `name = "Alice"` instead of `string name = "Alice"`

2. **Method Chaining**: Chain operations for concise code: `numbers.filter("even").map("double")`

3. **Multiline in REPL**: The REPL supports multiline statements - just keep typing and it will show `...>` prompts

4. **Property vs Method**: Both `list.size` and `list.size()` work - use what feels natural

5. **Functional Style**: Use `filter()` and `map()` for data transformations instead of explicit loops when possible

6. **Immutability**: Use `freeze()` to prevent accidental data modifications, especially for configuration and shared data structures

7. **Contamination Checking**: Use `can_accept()` to check if frozen and unfrozen data can be safely mixed before operations

8. **Module Organization**: Use modules to organize related functions - create focused modules like `math_utils.gr` or `string_helpers.gr`

9. **Built-in Modules**: Always import `"io"` for file operations and user interaction - it's essential for most programs

10. **Module vs Load**: Use `import` for reusable code with namespacing, `load` for configuration and simple scripts

11. **String Processing**: Use `string.split()` for parsing - `split()` for spaces, `split(",")` for CSV, `split("\n")` for lines

## Crypto Module

The crypto module provides secure cryptographic operations for data protection and integrity verification.

### Import and Basic Usage
```glang
import "crypto"

# Convert string to bytes for crypto operations
message = "Hello World"
# Manual conversion (until we have string.to_bytes())
data = [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]
```

### Hashing Operations
```glang
# Hash data with different algorithms
md5_hash = crypto.hash_md5(data)        # 16 bytes
sha1_hash = crypto.hash_sha1(data)      # 20 bytes  
sha256_hash = crypto.hash_sha256(data)  # 32 bytes
sha512_hash = crypto.hash_sha512(data)  # 64 bytes

# Convert hash to readable hex string
hex_hash = crypto.to_hex(sha256_hash)
print(hex_hash)  # "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e"
```

### Symmetric Encryption (AES-256)
```glang
# Generate cryptographic key
key = crypto.random_bytes(32)  # 32 bytes for AES-256

# Encrypt data
encrypted = crypto.aes_encrypt(data, key)  # Returns IV + ciphertext

# Decrypt data  
decrypted = crypto.aes_decrypt(encrypted, key)  # Returns original data

# Verify roundtrip
if decrypted == data {
    print("Encryption successful!")
}
```

### Random Number Generation
```glang
# Generate cryptographically secure random bytes
salt = crypto.random_bytes(16)      # 16 random bytes
nonce = crypto.random_bytes(12)     # 12 bytes for GCM mode
session_key = crypto.random_bytes(32)  # 32 bytes for keys
```

### Format Conversion (Solves Hex Literal Gap)
```glang
# Convert bytes to/from hexadecimal
packet_data = [18, 52, 86, 120]        # Instead of [0x12, 0x34, 0x56, 0x78]
hex_string = crypto.to_hex(packet_data) # "12345678"
restored = crypto.from_hex("FFABCD")    # [255, 171, 205]

# Convert bytes to/from base64
encoded = crypto.to_base64(data)        # "SGVsbG8gV29ybGQ="
decoded = crypto.from_base64(encoded)   # Original data back
```

### Complete Example: File Encryption
```glang
import "io"
import "crypto"

# Read file as binary data
data = io.read_binary("secret.txt")

# Generate encryption key (save this securely!)
key = crypto.random_bytes(32)
key_hex = crypto.to_hex(key)
print("Save this key: " + key_hex)

# Encrypt the file
encrypted = crypto.aes_encrypt(data, key)

# Save encrypted file
io.write_binary("secret.txt.encrypted", encrypted)

# Later: decrypt the file
saved_key = crypto.from_hex(key_hex)
encrypted_data = io.read_binary("secret.txt.encrypted")
decrypted = crypto.aes_decrypt(encrypted_data, saved_key)

# Save decrypted file
io.write_binary("secret_restored.txt", decrypted)
```

### Security Best Practices
```glang
# Always use proper key sizes
aes_key = crypto.random_bytes(32)      # ✅ 256-bit AES key
weak_key = crypto.random_bytes(8)      # ❌ Too short

# Use SHA-256 or SHA-512 for new applications  
secure_hash = crypto.hash_sha256(data) # ✅ Cryptographically secure
legacy_hash = crypto.hash_md5(data)    # ⚠️ Use only for compatibility

# Store keys securely (never in source code)
config_key = crypto.from_hex(io.read_file("key.txt"))  # ✅ Read from secure file
hardcoded = [1, 2, 3, 4]              # ❌ Never hardcode keys
```

---

*Happy coding with Glang! 🚀*
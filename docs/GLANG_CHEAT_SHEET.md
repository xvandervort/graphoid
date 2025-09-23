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

# Generators (programmatic creation)
nums = []
sequence = nums.generate(1, 10, 2)     # [1, 3, 5, 7, 9]
count = nums.upto(5)                   # [0, 1, 2, 3, 4, 5]
squares = nums.from_function(4, x => x * x)  # [0, 1, 4, 9]

# Access & modification
numbers[0]                 # Get: 1
numbers[0] = 99           # Set
numbers.append(6)         # Add to end
numbers.size()            # Get length: 6

# Element naming (R vector style)
heights = [165, 180, 175].set_names(["alice", "bob", "charlie"])
heights["alice"]           # Get by name: 165 (same as heights[0])
heights[0]                 # Get by index: 165 (same as heights["alice"])
heights.get_names()        # Get all names: ["alice", "bob", "charlie"]
heights.has_names()        # Check if named: true
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

### Maps (Collections of Data Nodes)
```glang
# Creation
settings = { "theme": "dark", "lang": "en", "debug": true }
map<string> prefs = { "color": "blue", "font": "arial" }

# Access & modification
settings["theme"]         # Get: { "theme": "dark" }
settings["theme"] = "light"  # Set
settings.get("theme")     # Get data node
settings.has_key("debug") # Check existence: true
settings.keys()           # Get all keys as list
settings.values()         # Get all values as list

# Element naming (for additional metadata)
config = { "host": "localhost", "port": 8080 }.set_names(["server", "connection"])
config["host"]            # Access by key: { "host": "localhost" }
config["server"]          # Access by name: { "host": "localhost" } (same element)
config.get_names()        # Get all names: ["server", "connection"]
config.has_names()        # Check if named: true
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

## 🛡️ Intrinsic Behaviors (NEW!)

### Adding Behaviors to Lists
```glang
# Create list with behaviors that auto-apply to all values
temps = [98.6, nil, 105.2, -10]
temps.add_rule("nil_to_zero")         # nil becomes 0
temps.add_rule("validate_range", 95, 105)  # Clamp to range

print(temps)                           # [98.6, 0, 105, 95]

# New values are automatically processed
temps.append(nil)                      # Becomes 0
temps.append(110)                      # Clamped to 105
```

### Adding Behaviors to Maps
```glang
# Behaviors work on map values too
config = { "timeout": nil, "retries": -5 }
config.add_rule("nil_to_zero")        # nil → 0
config.add_rule("positive")           # negative → positive

print(config["timeout"])              # 0 (was nil)
print(config["retries"])              # 5 (was -5)
```

### Managing Behaviors
```glang
# Query and manage behaviors
list.has_rule("nil_to_zero")          # Check if rule exists
list.get_rules()                       # Get all active rules
list.remove_rule("positive")          # Remove specific rule
list.clear_rules()                     # Remove all rules
```

### Standard Behaviors
- `nil_to_zero` - Convert nil to 0
- `nil_to_empty` - Convert nil to ""
- `positive` - Make negatives positive
- `round_to_int` - Round to integer
- `uppercase`/`lowercase` - String case
- `validate_range(min, max)` - Clamp to range

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
text.starts_with("Hello")  # true
text.ends_with("!")        # true
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

### String Prefix and Suffix Checking
```glang
filename = "document.pdf"
url = "https://example.com"
email = "user@domain.com"

# Check file extensions
if filename.ends_with(".pdf") {
    print("This is a PDF file")
}

# Check URL protocols
if url.starts_with("https://") {
    print("Secure connection")
} else if url.starts_with("http://") {
    print("Insecure connection")
}

# Validate email domains
if email.ends_with("@domain.com") {
    print("Internal email")
}

# Case-sensitive checks
name = "Hello World"
name.starts_with("hello")  # false (case-sensitive)
name.starts_with("Hello")  # true
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

# Same for maps and data nodes
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
# config["debug"] = false  # Cannot modify frozen map
# config.set("port", 8080) # Cannot add to frozen map

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
map.can_accept(value)      # Check if value can be stored
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

## 🔍 Regex Module

The regex module provides powerful pattern matching and text processing capabilities for complex string operations.

### Import and Basic Usage
```glang
import "regex"

# Basic pattern matching
phone_pattern = "\\d{3}-\\d{3}-\\d{4}"
text = "Call 555-123-4567 for help"

# Check if pattern exists anywhere
found = regex.search(phone_pattern, text)       # true

# Validate entire text matches pattern
phone = "555-123-4567"
is_valid = regex.validate(phone_pattern, phone) # true

# Check if pattern matches at start
starts_with = regex.match(phone_pattern, text)  # false
```

### Text Extraction
```glang
import "regex"

# Extract all numbers
text = "Order 123 contains 45 items costing $67.89"
numbers = regex.find_all("\\d+", text)
# Returns: ["123", "45", "67", "89"]

# Extract with capture groups
data_pattern = "(\\w+):\\s*([^,]+)"
text = "name: Alice, age: 30, city: New York"
groups = regex.find_groups(data_pattern, text)
# Returns: [["name", "Alice"], ["age", "30"], ["city", "New York"]]
```

### Text Transformation
```glang
import "regex"

# Simple replacement
text = "I have 42 apples and 17 oranges"
result = regex.replace("\\d+", "X", text)
# Result: "I have X apples and X oranges"

# Replacement with capture groups
email_pattern = "(\\w+)@(\\w+\\.\\w+)"
text = "Contact alice@example.com for help"
result = regex.replace(email_pattern, "$1 at $2", text)
# Result: "Contact alice at example.com for help"

# Split on pattern
text = "apple,banana;orange:grape|kiwi"
parts = regex.split("[,;:|]", text)
# Returns: ["apple", "banana", "orange", "grape", "kiwi"]
```

### Common Patterns
```glang
import "regex"

# Email validation
email_pattern = "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
is_email = regex.validate(email_pattern, "user@example.com")  # true

# Phone number extraction
phone_pattern = "\\b\\d{3}-\\d{3}-\\d{4}\\b"
text = "Call 555-123-4567 or 555-987-6543 for help"
phones = regex.find_all(phone_pattern, text)
# Returns: ["555-123-4567", "555-987-6543"]

# HTML tag removal
html_text = "<p>Hello <b>World</b>! Visit <a href='link'>here</a>.</p>"
clean = regex.replace("<[^>]+>", "", html_text)
# Result: "Hello World! Visit here."
```

### Regex Flags
```glang
import "regex"

# Case insensitive search
found = regex.search("hello", "HELLO WORLD", "i")        # true

# Multiline mode
text = "Hello\nWorld"
found = regex.search("^World", text, "m")                # true

# Combine flags
pattern = "^hello.*world$"
text = "HELLO\nWORLD"
found = regex.search(pattern, text, "ims")               # true (case-insensitive + multiline + dotall)
```

## 🎲 Random Module

The random module provides comprehensive random number generation, statistical distributions, and secure randomness.

### Import and Basic Usage
```glang
import "random" as rand

# Basic random numbers
dice_roll = rand.randint(1, 6)                    # 1-6 inclusive
probability = rand.random()                       # 0.0 to 1.0 (exclusive)
price = rand.uniform(10.0, 50.0)                 # 10.0 to 50.0 (exclusive)

# Random choice from list
colors = ["red", "green", "blue", "yellow"]
chosen = rand.choice(colors)                      # Pick one random color
```

### Statistical Distributions
```glang
import "random" as rand

# Normal distribution (bell curve)
height = rand.normal(170.0, 10.0)                # Mean=170cm, std=10cm
iq_score = rand.normal(100.0, 15.0)              # Mean=100, std=15

# Exponential distribution (time intervals)
wait_time = rand.exponential(0.5)                # Average wait = 1/0.5 = 2 units
service_time = rand.exponential(1.0)             # Average service = 1 unit

# Gamma distribution (positive skewed data)
task_duration = rand.gamma(2.0, 3.0)             # Shape=2, Scale=3
```

### Random Sampling
```glang
import "random" as rand

# Sample without replacement
population = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
sample_data = rand.sample(population, 3)         # Pick 3 unique elements

# Shuffle list (returns new shuffled copy)
original = [1, 2, 3, 4, 5]
shuffled = rand.shuffle(original)                # New shuffled list
# original remains unchanged

# Random choice (with replacement)
responses = ["Yes", "No", "Maybe", "Ask again"]
answer = rand.choice(responses)                  # Magic 8-ball
```

### Seeding for Reproducibility
```glang
import "random" as rand

# Seed for reproducible results
rand.seed(12345)
sequence1 = [rand.randint(1, 10), rand.randint(1, 10), rand.randint(1, 10)]

rand.seed(12345)  # Same seed
sequence2 = [rand.randint(1, 10), rand.randint(1, 10), rand.randint(1, 10)]
# sequence1 == sequence2 (true)

# Reset to secure random mode
rand.reset()
secure_value = rand.random()                     # Cryptographically secure
```

### Secure Random Generation
```glang
import "random" as rand

# Always cryptographically secure (ignores seeding)
secure_number = rand.secure_random()             # Secure float
secure_int = rand.secure_randint(1000, 9999)    # Secure 4-digit PIN

# Secure tokens for authentication
session_token = rand.secure_token(32)           # 64-character hex string
api_key = rand.secure_token(16)                 # 32-character hex string

# UUID generation
user_id = rand.uuid4()                          # Random UUID
record_id = rand.uuid1()                        # Time-based UUID
```

### Practical Examples
```glang
import "random" as rand

# Game mechanics
func roll_dice(sides) {
    return rand.randint(1, sides)
}
d6 = roll_dice(6)                               # Standard die
d20 = roll_dice(20)                             # RPG die

# Password generation
func generate_password(length) {
    chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    password = ""
    for i = 0; i < length; i = i + 1 {
        idx = rand.secure_randint(0, chars.length() - 1)
        password = password + chars[idx]
    }
    return password
}

secure_password = generate_password(12)         # 12-character password

# A/B testing with consistent assignment
func assign_test_group(user_id) {
    rand.seed(user_id)                          # Consistent assignment
    group = if rand.random() < 0.5 { "A" } else { "B" }
    rand.reset()                                # Back to secure mode
    return group
}
```

## 🐛 Enhanced Error Handling & Debugging

### Stack Traces with Full Context
```glang
# When errors occur, Glang provides detailed stack traces:

func outer_function(z) {
    return middle_function(z + 1)
}

func middle_function(y) {
    return inner_function(y * 2)
}

func inner_function(x) {
    return missing_variable + x    # Error: undefined variable
}

result = outer_function(5)

# Output:
# Traceback (most recent call last):
#   in inner_function() at line 7, column 20
#     return inner_function(y * 2)
#     ~~~~~~~~~~~~~~~~~~~^
#     Local variables: {'x': '12', 'y': '6'}
#   in middle_function() at line 11, column 20
#     return middle_function(z + 1)
#     ~~~~~~~~~~~~~~~~~~~^
#     Local variables: {'z': '5'}
#   in outer_function() at line 14, column 18
#     result = outer_function(5)
#     ~~~~~~~~~~~~~~~~~^
# VariableNotFoundError: Variable 'missing_variable' not found
```

### Error-as-Data Pattern
```glang
# Use result tuples for clean error handling
func safe_divide(a, b) {
    if b == 0 {
        return [:error, "Division by zero"]
    }
    return [:ok, a / b]
}

# Pattern match on results
result = safe_divide(10, 2)
message = match result {
    [:ok, value] => "Success: " + value.to_string(),
    [:error, msg] => "Error: " + msg,
    _ => "Unknown result"
}

# Handle multiple operations
results = [
    safe_divide(10, 2),    # [:ok, 5]
    safe_divide(10, 0),    # [:error, "Division by zero"]
    safe_divide(20, 4)     # [:ok, 5]
]

for result in results {
    status = match result {
        [:ok, val] => "✓ " + val.to_string(),
        [:error, err] => "✗ " + err,
        _ => "? Unknown"
    }
    print(status)
}
```

### Error Information Features
- **Full call chain** - See exactly where errors originated
- **Source context** - View the failing line with visual pointer
- **Local variables** - Inspect variable values at each stack level
- **Pattern matching** - Use `:ok`/`:error` symbols for structured error handling
- **Stack traces** - Professional debugging information like Python/Java

---

*Happy coding with Glang! 🚀*
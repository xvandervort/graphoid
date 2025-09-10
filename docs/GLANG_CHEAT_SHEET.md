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

### Chained Conversions
```glang
x = 42
result = x.to_string().to_bool()  # true
b = true.to_num().to_string()     # "1"
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

```glang
# Load another .gr file
load "config.gr"         # Variables become available

# REPL file commands  
/load config.gr          # Load file in REPL
/save myprogram.gr       # Save current session
/run example.gr          # Run file in fresh session
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

---

## 🎓 Pro Tips

1. **Type Inference**: Let Glang infer types when obvious: `name = "Alice"` instead of `string name = "Alice"`

2. **Method Chaining**: Chain operations for concise code: `numbers.filter("even").map("double")`

3. **Multiline in REPL**: The REPL supports multiline statements - just keep typing and it will show `...>` prompts

4. **Property vs Method**: Both `list.size` and `list.size()` work - use what feels natural

5. **Functional Style**: Use `filter()` and `map()` for data transformations instead of explicit loops when possible

6. **Immutability**: Use `freeze()` to prevent accidental data modifications, especially for configuration and shared data structures

7. **Contamination Checking**: Use `can_accept()` to check if frozen and unfrozen data can be safely mixed before operations

---

*Happy coding with Glang! 🚀*
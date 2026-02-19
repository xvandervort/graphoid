# Chapter 9: Standard Library

Graphoid includes a comprehensive standard library covering mathematics, I/O, data formats, networking, and more. This chapter provides an overview of available modules.

## Core Modules

### math - Mathematical Functions

```graphoid
import "math"

# Constants
pi = math.pi      # 3.14159...
e = math.e        # 2.71828...

# Basic functions
sqrt = math.sqrt(16)         # 4
pow = math.pow(2, 8)         # 256
abs = math.abs(-5)           # 5

# Trigonometry
sin = math.sin(math.pi / 2)  # 1.0
cos = math.cos(0)            # 1.0
tan = math.tan(math.pi / 4)  # 1.0

# Rounding
ceil = math.ceil(3.2)        # 4
floor = math.floor(3.8)      # 3
round = math.round(3.5)      # 4

# Logarithms
log = math.log(10)           # Natural log
log10 = math.log10(100)      # 2
log2 = math.log2(8)          # 3

# Min/Max
min = math.min(5, 3, 8)      # 3
max = math.max(5, 3, 8)      # 8
```

### string - String Operations

```graphoid
import "string"

# Case conversion
upper = string.to_upper("hello")     # "HELLO"
lower = string.to_lower("HELLO")     # "hello"

# Trimming
trimmed = string.trim("  hello  ")   # "hello"

# Splitting/Joining
parts = string.split("a,b,c", ",")   # ["a", "b", "c"]
joined = string.join(["a", "b"], "-") # "a-b"

# Searching
contains = string.contains("hello", "ll")  # true
index = string.index_of("hello", "ll")     # 2

# Generators
padding = string.generate(" ", 10)         # 10 spaces
alphabet = string.generate("a", "z")       # "abc...xyz"
```

### random - Random Numbers

Built-in alias: `rand`

```graphoid
import "random"

# Random integer
num = random.random_int(1, 10)    # 1-10 inclusive
# or: num = rand.random_int(1, 10)

# Random float
flt = random.random_float()       # 0.0-1.0

# Random choice
choice = random.choice([1, 2, 3, 4, 5])

# Shuffle
shuffled = random.shuffle([1, 2, 3, 4, 5])

# Random seed (for reproducibility)
random.seed(42)
```

### time - Time and Date

```graphoid
import "time"

# Current time
now = time.now()                 # Unix timestamp

# Format time
formatted = time.format(now, "YYYY-MM-DD HH:mm:ss")

# Parse time
parsed = time.parse("2024-01-15", "YYYY-MM-DD")

# Sleep
time.sleep(1.5)  # Sleep 1.5 seconds

# Measure execution time
start = time.now()
# ... do work ...
elapsed = time.now() - start
```

## Data Format Modules

### json - JSON Parsing

```graphoid
import "json"

# Parse JSON string
data = json.parse('{"name": "Alice", "age": 30}')
print(data["name"])  # "Alice"

# Convert to JSON
obj = {"name": "Bob", "scores": [95, 87, 92]}
json_str = json.to_string(obj)
# '{"name":"Bob","scores":[95,87,92]}'
```

### csv - CSV Handling

```graphoid
import "csv"

# Parse CSV
data = csv.parse("name,age\nAlice,30\nBob,25")
# [{"name": "Alice", "age": "30"}, {"name": "Bob", "age": "25"}]

# Write CSV
rows = [
    {"name": "Alice", "age": 30},
    {"name": "Bob", "age": 25}
]
csv_str = csv.stringify(rows)
# "name,age\nAlice,30\nBob,25"
```

### regex - Regular Expressions

```graphoid
import "regex"

# Match
matches = regex.match("hello 123", "[0-9]+")
print(matches)  # ["123"]

# Test
is_email = regex.test("user@example.com", ".+@.+\\..+")
# true

# Replace
result = regex.replace("hello world", "world", "Graphoid")
# "hello Graphoid"

# Split
parts = regex.split("a1b2c3", "[0-9]+")
# ["a", "b", "c"]
```

## I/O Modules

### io - Input/Output

```graphoid
import "io"

# Print to console
io.print("Hello")
io.print("No newline", false)

# Read line
line = io.read_line()

# Read file
content = io.read_file("data.txt")

# Write file
io.write_file("output.txt", "Hello, World!")

# Append to file
io.append_file("log.txt", "New entry\n")
```

### fs - File System

```graphoid
import "fs"

# Check if file exists
exists = fs.exists("file.txt")

# List directory
files = fs.list_dir("./")

# Create directory
fs.make_dir("new_folder")

# Delete file
fs.remove("file.txt")

# Copy file
fs.copy("source.txt", "dest.txt")

# Move file
fs.move("old.txt", "new.txt")

# Get file info
info = fs.stat("file.txt")
print(info.size)
print(info.modified)
```

## Network Modules

### net - Networking (TCP)

```graphoid
import "net"

# TCP client
socket = net.connect("example.com", 80)
net.send(socket, "GET / HTTP/1.1\r\nHost: example.com\r\n\r\n")
response = net.recv(socket, 4096)
net.close(socket)

# TCP server
listener = net.bind("127.0.0.1", 8080)
socket = net.accept(listener)       # Blocks until connection
data = net.recv(socket, 4096)
net.send(socket, "Hello!\n")
net.close(socket)
net.close_listener(listener)
```

### http - HTTP Client and Server

```graphoid
import "http"

# --- Client ---
response = http.get("https://api.example.com/data")
print(response["status"])
print(response["body"])

# --- Server ---
fn handle_home(req) {
    return http.response(200, "text/html", "<h1>Hello!</h1>")
}

fn handle_api(req) {
    return http.json_response({"status": "ok"})
}

server = http.create_server("127.0.0.1", 8080)
server = http.add_route(server, "GET", "/", handle_home)
server = http.add_route(server, "GET", "/api", handle_api)
http.serve(server)
```

See `docs/api-reference/stdlib/http.md` for complete API reference.

## System Modules

### os - Operating System

```graphoid
import "os"

# Environment variables
home = os.get_env("HOME")
os.set_env("MY_VAR", "value")

# Current directory
cwd = os.getcwd()

# Change directory
os.chdir("/path/to/dir")

# Execute command
result = os.execute("ls -la")
print(result.stdout)
print(result.exit_code)

# Platform info
platform = os.platform()  # "linux", "macos", "windows"
```

## Advanced Modules

### statistics - Statistical Functions

Built-in alias: `stats`

```graphoid
import "statistics"

data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

# Central tendency
mean = statistics.mean(data)       # 5.5
median = statistics.median(data)   # 5.5
mode = statistics.mode([1,1,2,3])  # 1

# Spread
stdev = statistics.stdev(data)     # Standard deviation
variance = statistics.variance(data)

# Quantiles
q1 = statistics.quantile(data, 0.25)
q3 = statistics.quantile(data, 0.75)
```

### collections - Advanced Collections

```graphoid
import "collections"

# Deque (double-ended queue)
dq = collections.deque()
dq.push_front(1)
dq.push_back(2)
front = dq.pop_front()

# Counter
counter = collections.counter(["a", "b", "a", "c", "a"])
# {"a": 3, "b": 1, "c": 1}

# Default dict
dd = collections.default_dict(0)
dd["key"]  # Returns 0 if key doesn't exist
```

### crypto - Cryptography

```graphoid
import "crypto"

# Hash functions
sha256 = crypto.sha256("hello")
md5 = crypto.md5("hello")

# HMAC
key = "secret_key"
hmac = crypto.hmac_sha256(key, "message")

# Base64
encoded = crypto.base64_encode("hello")
decoded = crypto.base64_decode(encoded)

# Random bytes
random_bytes = crypto.random_bytes(16)
```

### constants - Mathematical Constants

```graphoid
import "constants"

# Mathematical
pi = constants.pi
e = constants.e
tau = constants.tau
phi = constants.phi  # Golden ratio

# Physical
c = constants.speed_of_light
g = constants.gravitational_constant
```

## Utility Modules

### pp - Pretty Print

```graphoid
import "pp"

# Pretty print data structures
data = {
    "users": [
        {"name": "Alice", "age": 30},
        {"name": "Bob", "age": 25}
    ]
}

pp.print(data)
# Formatted, colored output
```

### optparse - Command-Line Parsing

```graphoid
import "optparse"

# Define options
parser = optparse.create()
parser.add_option("--name", "Name of user", required: true)
parser.add_option("--age", "Age of user", type: "int")
parser.add_option("--verbose", "Verbose mode", flag: true)

# Parse arguments
args = parser.parse()
print(args.name)
print(args.age)
print(args.verbose)
```

## Module Usage Patterns

### Error Handling with I/O

```graphoid
import "io"
import "fs"

if fs.exists("config.json") {
    content = io.read_file("config.json")
    config = json.parse(content)
} else {
    print("Config file not found!")
}
```

### HTTP API Call

```graphoid
import "http"
import "json"

response = http.get("https://api.github.com/users/octocat")

if response["status"] == 200 {
    user = json.parse(response["body"])
    print("Name: " + user["name"])
    print("Repos: " + user["public_repos"].to_string())
}
```

### Web Server

```graphoid
import "http"

fn handle_home(req) {
    return http.response(200, "text/html", "<h1>Hello!</h1>")
}

fn handle_api(req) {
    return http.json_response({"status": "ok"})
}

server = http.create_server("127.0.0.1", 8080)
server = http.add_route(server, "GET", "/", handle_home)
server = http.add_route(server, "GET", "/api", handle_api)
http.serve(server)
```

### File Processing

```graphoid
import "io"
import "csv"

# Read CSV file
content = io.read_file("data.csv")
rows = csv.parse(content)

# Process data
for row in rows {
    print(row["name"] + ": " + row["score"])
}
```

## Finding Documentation

For detailed API documentation, see:
- **API Reference**: `docs/api-reference/stdlib/`
- **Examples**: `examples/stdlib/`
- **Module Source**: `stdlib/*.gr`

## Summary

In this chapter, you learned about:

- ✅ **Core modules**: math, string, random, time
- ✅ **Data formats**: json, csv, regex
- ✅ **I/O**: io, fs
- ✅ **Networking**: net, http
- ✅ **System**: os
- ✅ **Advanced**: statistics, collections, crypto, constants
- ✅ **Utilities**: pp, optparse
- ✅ **Common patterns**: Error handling, API calls, file processing

---

## Quick Reference

```graphoid
# Core
import "math"
import "string"
import "random"
import "time"

# Data
import "json"
import "csv"
import "regex"

# I/O
import "io"
import "fs"

# Network
import "net"
import "http"

# System
import "os"

# Advanced
import "statistics"
import "collections"
import "crypto"
import "constants"

# Utilities
import "pp"
import "optparse"
```

---

## Exercises

1. **File Statistics**: Read a text file and compute word frequency using `io`, `string`, and a hash

2. **API Client**: Create a simple REST API client using `http` and `json`

3. **CSV Processor**: Read a CSV file, compute statistics on numeric columns using `statistics`

4. **Directory Walker**: Recursively list all files in a directory tree using `fs`

5. **Log Analyzer**: Parse log files using `regex` and compute statistics

6. **Config Manager**: Create a config system that reads JSON/CSV with defaults

7. **Web Server**: Build a web server with HTML and JSON endpoints using `http`

**Solutions** are available in `examples/09-standard-library/exercises.gr`

---

[← Previous: Directives](08-directives.md) | [Next: Best Practices →](10-best-practices.md)

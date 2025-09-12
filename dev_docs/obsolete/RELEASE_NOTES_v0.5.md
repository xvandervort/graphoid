# Glang v0.5 Release Notes
*Released: January 2025*

## 🎯 Solid Foundation: Core Language Features Complete

Glang v0.5 establishes a solid foundation with essential language features, comprehensive I/O operations, advanced string manipulation, and network capabilities. While still early in development, v0.5 demonstrates Glang's potential and provides a stable base for future enhancements.

## 🆕 New Features

### 🌐 Complete Network I/O Operations
**NEW**: Full HTTP client capabilities for web integration:
- `io.http_get(url)` - Make HTTP GET requests
- `io.http_post(url, data)` - Make HTTP POST requests with data
- `io.download_file(url, filepath)` - Download files from URLs
- `io.send_email()` - Email notifications (placeholder for future SMTP)

```glang
import "io"

# Fetch data from APIs
response = io.http_get("https://api.example.com/data")

# Download files 
io.download_file("https://example.com/file.txt", "local_file.txt")

# Post data to services
result = io.http_post("https://api.example.com/submit", "key=value")
```

### 📁 Enhanced File Operations
**IMPROVED**: Comprehensive file system operations:
- **Text Files**: `read_file()`, `write_file()`, `append_file()`
- **Binary Files**: `read_binary()`, `write_binary()` for images, executables, etc.
- **Line Operations**: `read_lines()`, `write_lines()` for structured text
- **Directory Management**: `make_dir()`, `remove_dir()`, `list_dir()`
- **Path Utilities**: `join_path()`, `split_path()`, `get_basename()`, `get_dirname()`, `get_extension()`, `resolve_path()`

### 🔤 Advanced String Manipulation
**COMPLETE**: Production-ready string processing:
- **Case Transformation**: `up()`, `down()`, `toUpper()`, `toLower()`
- **Text Processing**: `trim()`, `split()`, `join()`
- **Regular Expressions**: `matches()`, `replace()`, `findAll()`
- **Character Operations**: `chars()`, `reverse()`, `unique()`
- **Type Conversion**: `to_string()`, `to_num()`, `to_bool()`

```glang
text = "  Hello, World! 123  "
clean = text.trim().up()           # "HELLO, WORLD! 123"
words = text.split(" ")            # ["Hello,", "World!", "123"]
no_nums = text.replace("[0-9]+", "X")  # "Hello, World! X"
```

## 🏗️ Technical Improvements

### ✅ Enhanced Testing Coverage
- **608+ tests** with **29% code coverage** (newly added network tests)
- **15 original I/O tests** plus **11 new network tests** 
- All I/O operations thoroughly tested including error cases
- String manipulation edge cases covered
- Network operations validated with real HTTP requests and error conditions

### 🎯 Module System Maturity
**COMPLETE**: Built-in module ecosystem:
- ✅ **Math Module**: Constants and mathematical functions
- ✅ **JSON Module**: Encoding, decoding, validation, pretty printing
- ✅ **I/O Module**: Complete file, console, and network operations
- ✅ **Crypto Module**: Hashing and cryptographic operations

### 🔧 Developer Experience
- **Interactive REPL** with full command history and tab completion
- **Method Discovery**: `/methods <var>` shows available operations
- **Type Inspection**: `/type <var>` displays detailed type information
- **Variable Introspection**: `/inspect <var>` for deep analysis

## 📊 Current Capabilities

### ✅ Core Language Features (v0.5)
- **Complete Function System**: Functions, lambdas, closures, recursion
- **Strong Type System**: Optional type inference and constraints
- **Modern Collections**: Lists, hashes, data nodes with method operations
- **File Loading System**: Modular programming with `.gr` files
- **Standard Library**: Math, JSON, I/O, Crypto modules
- **Network Integration**: HTTP client for web services and APIs

### 🎮 Example: Real-World Web Service
```glang
import "io"
import "json"

# Fetch user data from API
string response = io.http_get("https://jsonplaceholder.typicode.com/users/1")
hash user_data = json.decode(response)

# Process and save locally
string name = user_data.get("name").value()
string email = user_data.get("email").value()

list<string> summary = [
    "User: " + name,
    "Email: " + email,
    "Fetched at: " + io.get_cwd()
]

io.write_lines("user_summary.txt", summary)
print("✅ User data processed and saved!")
```

## 🗺️ Roadmap Progress

### Phase 1: Production Readiness (Q1-Q2 2025) - **~50% COMPLETE**
- ✅ **Math module** with constants and functions
- ✅ **JSON module** for data exchange
- ✅ **Complete I/O operations** (file, network, console)
- ✅ **String manipulation utilities** with regex support
- ⏳ **Date/time handling** (next priority)
- ⏳ **Regular expressions** (advanced features)
- ⏳ **Random number generation**

### What's Next for v0.6-v1.0
- **Date/time handling** module
- **Regular expressions** (advanced features) 
- **Random number generation**
- **Performance benchmarking** suite
- **Enhanced error messages** with stack traces
- **VS Code extension** prototype

*For the complete development roadmap through 2026, see [PRIMARY_ROADMAP.md](./PRIMARY_ROADMAP.md)*

## 🎯 Breaking Changes
**None** - This release is fully backward compatible with v0.8.

## 🐛 Bug Fixes & Testing
- **Added comprehensive network I/O tests**: 11 new tests covering HTTP requests, file downloads, and error conditions
- Enhanced type inference for module function returns
- Improved error handling in network operations with proper exception handling
- Better memory management for large file operations
- Fixed edge cases in string regex operations
- Robust error handling for temporary service failures in network tests

## 📝 Documentation Updates
- **Consolidated roadmap** with [PRIMARY_ROADMAP.md](./PRIMARY_ROADMAP.md)
- **Updated README** with current feature set and examples
- **Comprehensive examples** in `/samples` directory
- **Network and I/O demos** showing real-world usage

## 🔮 Looking Ahead: The Graph Revolution

While v0.5 establishes the foundational language features, Glang's unique vision remains: **revolutionary graph computing**. Future releases will transform current containers into true graph structures with:

- **Phase 2 (Q3-Q4 2025)**: Graph Foundation with nodes, edges, and traversal
- **Phase 3 (Q4 2025)**: Self-Aware Data Structures with reflection
- **Phase 4 (2026)**: Distributed Graph Systems across multiple machines

## 🙏 Acknowledgments

Glang v0.5 represents important progress in establishing a solid foundation for graph computing. While still early in development, the core language features and comprehensive I/O capabilities demonstrate Glang's potential and provide a stable base for the revolutionary features ahead.

---

**Download**: Available through the repository  
**Documentation**: See [README.md](./README.md) and [CLAUDE.md](./CLAUDE.md)  
**Roadmap**: [PRIMARY_ROADMAP.md](./PRIMARY_ROADMAP.md)
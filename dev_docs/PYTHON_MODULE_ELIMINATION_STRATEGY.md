# Python Module Elimination Strategy

**Status**: Analysis Complete
**Goal**: Transition Glang standard library from Python dependencies to pure Glang implementations
**Timeline**: Phased approach over 3-6 months

## Current Architecture Problem

Glang currently has a **dual-layer module system** that creates unnecessary Python dependencies:

### Layer 1: Python Modules (Core Implementation)
Located in `src/glang/modules/`:
```
http_module.py      -> Registered as 'http' built-in
json_module.py      -> Registered as 'json' built-in
time_module.py      -> Registered as 'time' built-in
html_module.py      -> Registered as 'html_parser' built-in
io_module.py        -> Registered as 'io' built-in
...
```

### Layer 2: Glang Standard Library (User-Facing API)
Located in `stdlib/`:
```
network.gr          -> import "http" as http
html.gr             -> import "html_parser" as html_core
benchmark.gr        -> import "time" as Time
```

**Problem**: Users get Glang modules that depend on Python implementations, reducing the "pure Glang" experience and creating deployment complexity.

## Current Dependency Analysis

### ✅ Already Pure Glang
- `stdlib/math.gr` - Mathematical constants and utilities
- `stdlib/csv.gr` - CSV parsing and generation
- `stdlib/behaviors.gr` - Behavior system utilities
- `stdlib/conversions.gr` - Type conversion utilities
- `stdlib/random.gr` - Random number utilities (likely pure)
- `stdlib/regex.gr` - Pattern matching utilities (likely pure)

### 🔄 Currently Depends on Python
- `stdlib/network.gr` → `"http"` → `http_module.py`
- `stdlib/html.gr` → `"html_parser"` → `html_module.py`
- `stdlib/benchmark.gr` → `"time"` → `time_module_simple.py`

### 🔍 Python Modules Not Exposed to Users
Many Python modules exist but aren't used by stdlib:
- `crypto_module.py`, `call_graph_module.py`, `regex_module.py`, etc.
- These are available as built-ins but may not need user-facing stdlib wrappers

## Elimination Strategy - 3 Phases

### Phase 1: Easy Wins (1-2 weeks) 🟢
**Target**: Replace simple logic/parsing modules

#### 1.1 JSON Module
- **Current**: `json_module.py` (Python json library)
- **Replace with**: Pure Glang JSON parser/encoder
- **Complexity**: Medium - recursive parsing, string escaping
- **Impact**: High - JSON is widely used

**Implementation**:
```glang
# stdlib/json_pure.gr
module json

func encode(value) {
    if value.type() == "string" {
        return "\"" + escape_json_string(value) + "\""
    } else if value.type() == "num" {
        return value.to_string()
    } else if value.type() == "bool" {
        return value.to_string()
    } else if value.type() == "list" {
        return encode_array(value)
    } else if value.type() == "map" {
        return encode_object(value)
    }
    return "null"
}

func decode(json_text) {
    parser = create_parser(json_text)
    return parse_value(parser)
}
```

#### 1.2 CSV Enhancement
- **Current**: `stdlib/csv.gr` (already pure!)
- **Action**: Enhance existing pure implementation
- **Add**: Proper quote handling, escape sequences, multi-line support

### Phase 2: Moderate Complexity (2-4 weeks) 🟡
**Target**: Replace modules with moderate system integration

#### 2.1 Time Module
- **Current**: `time_module_simple.py` (Python datetime)
- **Replace with**: Minimal system integration + pure Glang
- **Strategy**: Only get UTC timestamp from system, do all formatting/parsing in Glang

```glang
# stdlib/time_pure.gr
module time

# System call for current UTC timestamp (minimal Python bridge)
func now_utc_timestamp() {
    # Single system call - get seconds since epoch
    return system_utc_timestamp()
}

func now() {
    timestamp = now_utc_timestamp()
    return create_time_from_timestamp(timestamp)
}

func from_components(year, month, day, hour, minute, second) {
    # Pure Glang date/time arithmetic
    return calculate_timestamp(year, month, day, hour, minute, second)
}
```

#### 2.2 HTML Module
- **Current**: `html_module.py` (Python html.parser)
- **Replace with**: Pure Glang HTML tokenizer/parser
- **Complexity**: Medium-High - HTML is complex but manageable

```glang
# stdlib/html_pure.gr
module html

func parse(html_content) {
    tokenizer = create_html_tokenizer(html_content)
    tokens = tokenize_all(tokenizer)
    return build_tree(tokens)
}

func find_by_tag(elements, tag_name) {
    # Pure Glang tree traversal
}
```

#### 2.3 Call Graph Module
- **Current**: `call_graph_module.py` (Python introspection)
- **Replace with**: Pure Glang introspection of Glang structures
- **Strategy**: Since this inspects Glang's own data, it should be implementable in Glang

### Phase 3: Strategic Core (Future) 🔴
**Target**: Identify permanent system boundary

#### 3.1 HTTP Module - Keep Python (For Now)
- **Rationale**: Network sockets, TLS, HTTP protocol implementation is extremely complex
- **Strategy**: Keep `http_module.py` as the core system boundary
- **Future**: Could be implemented in Glang as advanced project

#### 3.2 I/O Module - Keep Python
- **Rationale**: File system operations require deep OS integration
- **Strategy**: Keep `io_module.py` as necessary system boundary
- **Note**: File I/O is fundamental and Python integration is appropriate here

#### 3.3 Crypto Module - Keep Python
- **Rationale**: Cryptographic algorithms require careful, audited implementation
- **Strategy**: Keep `crypto_module.py` for security and correctness

## Implementation Process

### For Each Module Elimination:

1. **Create Pure Glang Version**
   ```bash
   # Create alongside existing Python version
   stdlib/json_pure.gr     # New pure implementation
   # Keep json_module.py    # Old Python version
   ```

2. **Comprehensive Testing**
   ```glang
   # Test compatibility with existing code
   import "json" as json          # Should work exactly the same
   data = { "name": "Alice", "age": 25 }
   encoded = json.encode(data)
   decoded = json.decode(encoded)
   ```

3. **Update Built-in Registration**
   ```python
   # src/glang/modules/builtin_modules.py
   # Comment out Python version:
   # from .json_module import create_json_module_namespace
   # cls._builtin_modules['json'] = create_json_module_namespace()

   # Add pure Glang version:
   from .module_manager import load_stdlib_module
   cls._builtin_modules['json'] = load_stdlib_module('json_pure.gr')
   ```

4. **Remove Python Module**
   ```bash
   rm src/glang/modules/json_module.py  # After thorough testing
   mv stdlib/json_pure.gr stdlib/json.gr
   ```

5. **Update Documentation**
   - Update module docs to reflect pure Glang implementation
   - Add migration notes for any breaking changes

## Benefits of Elimination

### For Users
- **Simpler Deployment**: Fewer Python dependencies
- **Better Performance**: No Python/Glang boundary crossing
- **Pure Glang Experience**: Everything feels native to the language
- **Self-Hosting Progress**: Glang implementing more of itself

### For Development
- **Cleaner Architecture**: Single-layer module system
- **Better Testing**: All code in the same language
- **Easier Debugging**: No cross-language debugging needed
- **Language Validation**: Proves Glang can handle complex tasks

## Migration Compatibility

### Backward Compatibility
- All existing user code continues to work
- Same import statements: `import "json" as json`
- Same API: `json.encode()`, `json.decode()`, etc.
- Same behavior: Identical input/output

### Performance Expectations
- **Pure Glang modules may be slower** than optimized Python libraries
- **But will be "fast enough"** for most use cases
- **Trade performance for simplicity and purity**

## Success Metrics

### Phase 1 Complete When:
- ✅ `import "json"` uses pure Glang implementation
- ✅ `import "time"` uses minimal Python bridge
- ✅ All existing demos/tests pass unchanged
- ✅ No user-visible behavior changes

### Phase 2 Complete When:
- ✅ `import "html"` uses pure Glang parser
- ✅ Call graph functionality is pure Glang
- ✅ Performance is acceptable (within 2x of Python)

### Final Success:
- ✅ Only HTTP, I/O, and Crypto modules depend on Python
- ✅ 80%+ of stdlib functionality is pure Glang
- ✅ Clear separation between "pure Glang" and "system integration" modules

## Next Steps

1. **Start with JSON Module** - highest impact, moderate complexity
2. **Create development branch** for pure implementations
3. **Build comprehensive test suite** comparing Python vs Glang versions
4. **Implement one module at a time** with thorough validation
5. **Document the process** to refine strategy for remaining modules

This strategy provides a clear path to significantly reduce Python dependencies while maintaining compatibility and identifying the appropriate system boundary for a pure Glang standard library.
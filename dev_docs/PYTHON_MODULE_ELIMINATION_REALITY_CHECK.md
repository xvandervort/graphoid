# Python Module Elimination - Reality Check

**Status**: Deep Analysis Complete
**Original Assessment**: Too optimistic - major revision required
**Key Finding**: Core language infrastructure missing for complex text/data processing

## Original Plan vs Reality

### Original Assessment (❌ Overly Optimistic)
- **Phase 1 "Easy Wins"**: JSON and Time modules in 1-2 weeks
- **Assumption**: Basic string processing exists in Glang
- **Timeline**: 3-6 months to eliminate most Python dependencies

### Reality Check (✅ Accurate Assessment)
- **Phase 1 Reality**: 6-8 weeks of **core language development** required FIRST
- **Finding**: Glang lacks fundamental string/parsing capabilities
- **Timeline**: 12-18 months with significant core language work

## Critical Infrastructure Gaps

### 🔴 Missing Core String Methods
**Current StringValue class lacks essential methods needed for ANY parsing:**

```python
# MISSING from src/glang/execution/values.py StringValue class:
def contains(self, substring) -> BooleanValue     # ❌ Not implemented
def index_of(self, substring) -> NumberValue      # ❌ Not implemented
def substring(self, start, end) -> StringValue    # ❌ Not implemented
def replace(self, old, new) -> StringValue        # ❌ Not implemented
```

**Available methods are limited:**
```python
# What StringValue CAN do:
split()        # Basic splitting
join()         # Join with delimiter
starts_with()  # Prefix check
ends_with()    # Suffix check
to_upper()     # Case conversion
to_lower()     # Case conversion
```

**Impact**: **Cannot implement ANY complex parsing** (JSON, HTML, CSV enhancements) without these basic operations.

### 🔴 No Type Construction from Glang
**Python modules create Glang types like this:**
```python
# This works in Python modules:
return StringValue("result", position)
return NumberValue(42, position)
return ListValue(elements, "string", position)
```

**But Glang code cannot create proper typed values:**
```glang
# This is IMPOSSIBLE in pure Glang:
func make_string_result() {
    return StringValue("hello")  # ❌ StringValue constructor not available
}

# Glang can only return untyped literals:
func make_result() {
    return "hello"  # Returns string literal, not StringValue object
}
```

**Impact**: Pure Glang modules **cannot return properly typed values** that integrate with the rest of the system.

### 🔴 No Error Handling Mechanism
**Python modules use exceptions:**
```python
try:
    result = json.loads(json_str.value)
    return JSONModule._python_to_glang(result, position)
except json.JSONDecodeError as e:
    raise RuntimeError(f"JSON parsing failed: {str(e)}", position)
```

**Glang has no equivalent error handling:**
```glang
# This is impossible - no try/catch, no error types:
func parse_json(text) {
    # How do we handle parse errors?
    # How do we report what went wrong?
    # How do we propagate errors up the call stack?
}
```

**Impact**: Cannot build robust parsing modules without error handling infrastructure.

## Detailed Module Analysis

### JSON Module - Originally Rated "Easy" ❌
**Reality**: **EXTREMELY DIFFICULT**

**Prerequisites needed:**
1. **String processing methods**: `contains()`, `index_of()`, `substring()`, `replace()`
2. **Character-level operations**: Unicode handling, escape sequences
3. **Type construction**: Create `StringValue`, `NumberValue`, `ListValue`, `HashValue`
4. **Error handling**: Report parsing errors with context
5. **Recursive parsing logic**: Handle nested structures
6. **Number parsing**: Convert strings to numbers with validation

**Example - Simple JSON string parsing:**
```glang
# This should be simple, but is IMPOSSIBLE in current Glang:
func parse_json_string(text) {
    # Find opening quote
    if text.contains("\"") == false {     # ❌ contains() doesn't exist
        return error("Not a string")      # ❌ No error system
    }

    start = text.index_of("\"")           # ❌ index_of() doesn't exist
    if start == -1 {
        return error("No opening quote")
    }

    # Find closing quote (with escape handling)
    end = find_closing_quote(text, start + 1)  # ❌ Complex logic needed
    if end == -1 {
        return error("No closing quote")
    }

    # Extract content
    content = text.substring(start + 1, end)   # ❌ substring() doesn't exist

    # Handle escape sequences
    unescaped = unescape_json(content)         # ❌ No escape handling

    return StringValue(unescaped)              # ❌ Can't create StringValue
}
```

**Realistic Timeline**: 8-12 weeks after core language development

### Time Module - Originally Rated "Minimal Integration" ❌
**Reality**: **COMPLEX SYSTEM INTEGRATION**

**Dependencies discovered:**
```python
# Current Python implementation uses:
import datetime as python_datetime
import time
import calendar

# Complex operations like:
dt = python_datetime.datetime(year, month, day, hour, minute, second,
                            tzinfo=python_datetime.timezone.utc)
timestamp = dt.timestamp()
```

**What pure Glang would need:**
1. **System timestamp access**: Requires OS integration
2. **Calendar arithmetic**: Leap years, days per month, etc.
3. **Timezone handling**: Complex timezone database and rules
4. **Date/time parsing**: Parse ISO formats like "2025-01-15T14:30:00Z"
5. **Date validation**: Ensure February 30 is invalid, etc.

**Realistic approach**: Keep minimal Python bridge for system timestamp, implement formatting/parsing in Glang.

**Timeline**: 4-6 weeks with hybrid approach

### HTML Module - Originally Rated "Medium-High" ❌
**Reality**: **MOST COMPLEX PARSING PROJECT**

**What HTML parsing actually requires:**
1. **All JSON prerequisites** (string methods, type construction, etc.)
2. **State machine implementation**: Track parser state across tokens
3. **Tokenizer**: Break HTML into tags, attributes, text, comments
4. **Tree construction**: Build nested element hierarchy
5. **Attribute parsing**: Handle quoted/unquoted attributes
6. **HTML entity decoding**: `&amp;` → `&`, `&lt;` → `<`, etc.
7. **Self-closing tag detection**: `<img/>`, `<br>`, etc.
8. **Error recovery**: Handle malformed HTML gracefully
9. **CSS selector logic**: For `find_by_class`, `find_by_tag`, etc.

**Example complexity - just parsing a simple tag:**
```glang
# Parsing <div class="container" id="main"> is surprisingly complex:
func parse_opening_tag(text) {
    # Find tag name
    if text.starts_with("<") == false {        # ✅ This exists
        return error("Not a tag")              # ❌ No error system
    }

    # Extract tag name (until space or >)
    name_end = find_tag_name_end(text)         # ❌ Complex parsing needed
    tag_name = text.substring(1, name_end)     # ❌ substring() doesn't exist

    # Parse attributes (quoted strings, unquoted values, etc.)
    attrs = parse_attributes(text, name_end)   # ❌ Extremely complex

    # Build element structure
    return HTMLElement(tag_name, attrs)        # ❌ No HTMLElement type
}
```

**Realistic Timeline**: 12-16 weeks of intensive development after core language work

## Core Language Development Required

### Stage 0: Essential Infrastructure (6-8 weeks)
**Must be implemented before ANY module conversion:**

#### String Processing Methods
```python
# Add to src/glang/execution/values.py StringValue class:

def contains(self, substring: 'StringValue') -> 'BooleanValue':
    """Check if string contains substring."""
    if not isinstance(substring, StringValue):
        raise ValueError(f"Contains argument must be string, got {substring.get_type()}")
    return BooleanValue(substring.value in self.value, self.position)

def index_of(self, substring: 'StringValue', start: int = 0) -> 'NumberValue':
    """Find index of substring, return -1 if not found."""
    if not isinstance(substring, StringValue):
        raise ValueError(f"Index_of argument must be string, got {substring.get_type()}")
    try:
        index = self.value.index(substring.value, start)
        return NumberValue(index, self.position)
    except ValueError:
        return NumberValue(-1, self.position)

def substring(self, start: 'NumberValue', end: 'NumberValue' = None) -> 'StringValue':
    """Extract substring from start to end."""
    start_idx = int(start.to_python())
    end_idx = int(end.to_python()) if end else len(self.value)
    result = self.value[start_idx:end_idx]
    return StringValue(result, self.position)

def replace(self, old: 'StringValue', new: 'StringValue') -> 'StringValue':
    """Replace all occurrences of old with new."""
    result = self.value.replace(old.value, new.value)
    return StringValue(result, self.position)
```

#### Type Construction System
```python
# Add to builtin_modules.py or similar:
def create_glang_string(value: str, position: Optional[SourcePosition] = None) -> StringValue:
    return StringValue(value, position)

def create_glang_number(value: float, position: Optional[SourcePosition] = None) -> NumberValue:
    return NumberValue(value, position)

def create_glang_list(elements: List[GlangValue], position: Optional[SourcePosition] = None) -> ListValue:
    return ListValue(elements, 'any', position)

# Register as built-in functions:
'make_string': create_glang_string,
'make_number': create_glang_number,
'make_list': create_glang_list,
```

#### Error Handling Framework
```glang
# Need language-level error handling:
func try_parse_json(text) {
    result = attempt_parse(text)
    if result.has_error() {
        return error("Parse failed: " + result.error_message())
    }
    return result.value()
}
```

### Stage 1: Proof of Concept (2-4 weeks)
**Simple modules to validate infrastructure:**

#### Enhanced CSV Module
```glang
# Build upon existing stdlib/csv.gr:
func parse_with_quotes(csv_text) {
    # Now possible with string methods:
    if csv_text.contains("\"") {
        return parse_quoted_csv(csv_text)
    } else {
        return parse_simple_csv(csv_text)
    }
}

func parse_quoted_csv(text) {
    result = []
    current_field = ""
    in_quotes = false

    i = 0
    while i < text.length() {
        char = text.substring(i, i + 1)  # Now possible!

        if char == "\"" {
            in_quotes = not in_quotes
        } else if char == "," and not in_quotes {
            result.append(current_field)
            current_field = ""
        } else {
            current_field = current_field + char
        }
        i = i + 1
    }

    return result
}
```

#### Simple JSON Subset
```glang
# Basic JSON parser (strings and numbers only):
func parse_simple_json(text) {
    text = text.trim()

    if text.starts_with("\"") and text.ends_with("\"") {
        # String value
        content = text.substring(1, text.length() - 1)  # Now possible!
        return make_string(content)  # Now possible!
    } else if is_numeric(text) {
        # Number value
        return make_number(text.to_num())  # Now possible!
    } else {
        return error("Unsupported JSON type")
    }
}
```

## Realistic Implementation Timeline

### Phase 0: Core Language (8-12 weeks)
- ✅ Add essential string methods (`contains`, `index_of`, `substring`, `replace`)
- ✅ Add type construction functions (`make_string`, `make_number`, etc.)
- ✅ Add basic error handling framework
- ✅ Add character/Unicode operations
- ✅ Test with simple parsing examples

### Phase 1: Simple Modules (4-6 weeks)
- ✅ Enhanced CSV module (quoted fields, escaping)
- ✅ Math library expansion (pure computation)
- ✅ Simple JSON subset (proof of concept)
- ✅ Basic pattern matching utilities

### Phase 2: Complex Text Processing (8-12 weeks)
- ⏳ Full JSON implementation (recursive parsing)
- ⏳ Time module (hybrid Python/Glang approach)
- ⏳ Advanced string processing utilities

### Phase 3: Advanced Parsing (12-16 weeks)
- ⏳ HTML parser implementation
- ⏳ Full regex/pattern matching
- ⏳ Other complex parsing modules

## Revised Strategy Recommendations

### Immediate Actions (Next 3 months)
1. **Focus on core language development** - string methods are essential
2. **Keep existing Python modules** - they work and are needed now
3. **Start with CSV enhancement** - build upon existing pure Glang foundation
4. **Proof of concept with simple JSON** - validate the new infrastructure

### Medium Term (6-12 months)
1. **Implement full string processing capabilities**
2. **Add proper error handling to the language**
3. **Convert 2-3 simple modules** to validate the approach
4. **Build parsing/processing libraries** in pure Glang

### Long Term (12+ months)
1. **Consider complex modules** like JSON, HTML after infrastructure is solid
2. **Keep system integration modules** in Python (HTTP, I/O, crypto)
3. **Focus on language expressiveness** rather than Python elimination

## Key Insight

**The core finding**: Eliminating Python dependencies requires **fundamental language development first**. The current Glang lacks basic text processing capabilities that any complex module needs.

**Strategic recommendation**:
- Accept that some Python integration is appropriate and necessary
- Focus language development on areas that provide the most value
- Build the string/parsing infrastructure that enables more complex pure Glang development
- Don't attempt to eliminate dependencies that require massive infrastructure development

The goal should be **strategic independence** where Glang can handle what it's designed for, while relying on Python for appropriate system integration points.
# Bug Report - January 2025

## Bugs Discovered During JSON Module Implementation

### Bug #1: Array Element Method Calls Fail with Type 'any'

**Status**: ACTIVE
**Severity**: HIGH
**Discovered**: During JSON validation implementation

**Description**:
When accessing an array element and calling a method on it, if the array doesn't have a specific type constraint, the element is treated as type 'any' and method calls fail.

**Example**:
```glang
valid_tests = ["hello", "123", "true"]
test1 = valid_tests[0]  # Returns type 'any'
if test1.starts_with("h") {  # ERROR: Invalid method call 'starts_with' on any
    # ...
}
```

**Error Message**:
```
Invalid method call 'starts_with' on any: Unknown type 'any' at line 1, column 10
```

**Expected Behavior**:
The array should infer that all elements are strings (since all literals are strings), or at minimum, the runtime should check the actual type of the element and allow appropriate method calls.

**Workaround**:
Must explicitly type the array:
```glang
list<string> valid_tests = ["hello", "123", "true"]
```

**Impact**:
- Prevents generic array processing
- Forces explicit typing even when type is obvious
- Breaks natural coding patterns

---

### Bug #2: Variable Keys in Hash Access Not Supported

**Status**: ACTIVE
**Severity**: MEDIUM
**Discovered**: During JSON parser state management implementation

**Description**:
Hash access using variable keys fails with a parser error. Only string literals are accepted as hash keys.

**Example**:
```glang
state = {"position": 0, "text": "example"}
key = "position"
value = state[key]  # ERROR: Key must be a string literal
```

**Error Message**:
```
Parse error: Key must be a string literal at line X, column Y
```

**Expected Behavior**:
Should support variable keys for dynamic hash access, essential for many programming patterns.

**Workaround**:
Must use string literals directly:
```glang
value = state["position"]  # Works, but not dynamic
```

**Impact**:
- Prevents dynamic hash access patterns
- Makes configuration and state management difficult
- Forces code duplication instead of parameterized access

---

### Bug #3: Parser Errors with Valid Function Definitions

**Status**: ACTIVE
**Severity**: HIGH
**Discovered**: During JSON parsing function implementation

**Description**:
Certain valid function definitions cause parser errors, specifically reporting "Expected '{'" when the brace is present.

**Example**:
```glang
func is_digit_char(char) {
    return char == "0" or char == "1" or char == "2"
}
```

**Error Message**:
```
Parse error: Expected '{'. Expected '{' but got 'if' at line 6, column 12
```

**Observations**:
- Error message is misleading - it says "got 'if'" when there's no 'if' at that location
- The same function pattern works in some files but not others
- May be related to file encoding, line endings, or hidden characters
- Could be context-dependent based on preceding code

**Workaround**:
Various attempts made:
- Renaming parameters (char -> c)
- Simplifying expressions
- Rewriting from scratch
- None consistently resolved the issue

**Impact**:
- Prevents implementation of utility functions
- Makes file creation unpredictable
- Severely impacts development productivity

---

### Bug #4: Method Calls on Hash Values

**Status**: NEEDS INVESTIGATION
**Severity**: MEDIUM
**Discovered**: Implied during JSON implementation

**Description**:
When retrieving values from hashes, the type information may be lost, preventing method calls.

**Example**:
```glang
config = {"timeout": 30, "host": "localhost"}
timeout_value = config["timeout"]
timeout_str = timeout_value.to_string()  # May fail if type is 'any'
```

---

## Summary

These bugs significantly impact Glang's usability and self-hosting goals:

1. **Type System Issues**: The 'any' type is too restrictive, preventing common operations
2. **Parser Limitations**: Variable hash keys and mysterious function definition errors
3. **Development Impact**: These bugs forced workarounds and prevented full JSON implementation

## Recommendations

1. **Priority Fix**: The array element type inference issue (#1) - this breaks intuitive code patterns
2. **Parser Investigation**: The function definition bug (#3) needs deep investigation - it's blocking development
3. **Hash Access Enhancement**: Variable key support (#2) is essential for real-world programming
4. **Type System Review**: Consider allowing method calls on 'any' with runtime type checking

## Files Affected

- `/home/irv/work/grang/samples/json_module_glang_demo.gr` - Hit bugs #2 and #3
- `/home/irv/work/grang/samples/json_minimal_demo.gr` - Hit bug #3
- `/home/irv/work/grang/samples/json_concept_demo.gr` - Hit bug #1
- Multiple other JSON implementation attempts affected

## Next Steps

These bugs should be addressed before continuing with self-hosting efforts, as they prevent implementation of core functionality in Glang itself.
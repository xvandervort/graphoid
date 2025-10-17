# CRITICAL BUG: Module Function Scoping Issue

**Priority**: IMMEDIATE - Blocks basic programming functionality
**Severity**: Critical - Breaks fundamental language features
**Status**: Identified, needs immediate fix

## Problem Statement

Functions defined within a Glang module **cannot call other functions** from the same module. This is a fundamental programming language limitation that prevents basic code organization and modularity.

## Reproduction Case

```glang
# File: test_module.gr
module test

func helper() {
    return 42
}

func main() {
    return helper()  # ERROR: Function 'helper' not found
}
```

When importing and calling:
```glang
import "test_module" as test
result = test.main()  # Fails with "Function 'helper' not found"
```

## Root Cause Analysis

**Location**: `src/glang/execution/executor.py:3520`
```python
func_value = self.context.get_variable(node.name)
```

**Issue**: When a function executes, it runs in the calling context, not the module's context. The function has no access to other functions defined in its own module.

**Architecture Problem**:
1. Module loads and functions are defined in main execution context
2. Functions are moved to module namespace after loading (`pipeline.py:257-259`)
3. When function executes later, it runs in caller's context
4. Function cannot find other functions from its original module

## Impact

This bug prevents:
- ✗ Basic function composition within modules
- ✗ Helper function patterns
- ✗ Code organization and reusability
- ✗ Modular programming principles
- ✗ Clean library design

**Workaround Required**: Every function must be completely self-contained, leading to:
- Code duplication
- Maintenance nightmares
- Poor code organization
- Violation of DRY principles

## Proposed Solution

### Option 1: Function Context Binding (Recommended)

Modify function execution to maintain reference to original module context:

```python
# In FunctionValue class
class FunctionValue:
    def __init__(self, name, parameters, body, position, module_context=None):
        self.module_context = module_context  # NEW: Store module context

# In executor.py call_function()
def call_function(self, func_value, arguments, position=None):
    if isinstance(func_value, FunctionValue) and func_value.module_context:
        # Create hybrid context: module + current scope
        with self.context.push_scope(func_value.module_context):
            # Function can now access both module functions and current variables
            return self.execute_function_body(func_value, arguments)
```

### Option 2: Module-Aware Variable Lookup

Modify `get_variable()` to check module context when in module function:

```python
def get_variable(self, name: str) -> Optional[GlangValue]:
    # Check current scope first
    if name in self.variables:
        return self.variables[name]

    # If in module function, check module namespace
    if self.current_module_context:
        module_var = self.current_module_context.get(name)
        if module_var:
            return module_var

    # Check module-qualified names
    if '.' in name:
        # existing logic...
```

## Test Cases Needed

```glang
# Basic function calling
func helper() { return 42 }
func main() { return helper() }

# Recursive calls
func factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}

# Complex interdependence
func a() { return b() + 1 }
func b() { return c() * 2 }
func c() { return 10 }
```

## Priority Justification

This is a **language-breaking bug** that prevents basic programming patterns. It should be fixed before any other features are added because:

1. **Fundamental Language Feature**: Function calls are core functionality
2. **Blocks Development**: Cannot write clean, modular code
3. **User Experience**: Extremely confusing for developers
4. **Architecture Debt**: Gets harder to fix as more code is built around workarounds

## Immediate Action Required

1. ✅ **Document the bug** (this file)
2. 🔄 **Create minimal reproduction test**
3. 🔄 **Implement fix in executor.py**
4. 🔄 **Add comprehensive test coverage**
5. 🔄 **Validate fix doesn't break existing functionality**
6. 🔄 **Update conversions.gr to use proper function composition**

## Files to Modify

- `src/glang/execution/executor.py` - Function execution context
- `src/glang/execution/values.py` - FunctionValue class
- `src/glang/execution/pipeline.py` - Module loading logic
- `test/test_module_scoping.py` - New comprehensive tests

---

**This bug must be fixed immediately as it blocks fundamental programming functionality.**
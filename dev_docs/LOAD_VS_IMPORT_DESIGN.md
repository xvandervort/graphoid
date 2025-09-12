# Load vs Import Design Document
*Created: 2025-01-05*

## Overview
This document clarifies the semantic distinction between `load` and `import` operations in glang, ensuring consistent design for file inclusion and module systems.

## Key Distinction

### `load` - Script Execution
**Purpose**: Execute a script file in the current namespace
**Behavior**: All statements executed as if typed directly in current context
**Namespace**: Variables added directly to current scope
**Use Cases**: Configuration files, initialization scripts, shared utility code

### `import` - Module System  
**Purpose**: Import a module with isolated namespace
**Behavior**: Creates separate namespace, requires qualified access
**Namespace**: Qualified access (`module.function`) or aliasing
**Use Cases**: Libraries, reusable modules, avoiding name conflicts

## Implementation Plan

### Phase A: Language-Level `load` Statement (Current)
Add `load` as a language statement that can be used in .gr files:

```glang
# config.gr
debug_mode = true
max_items = 100
default_greeting = "Hello"

# main.gr  
load "config.gr"     # Variables now directly available
if debug_mode {      # Can use without qualification
    print("Debug enabled")
}
```

**Implementation Steps**:
1. Add `LoadStatement` AST node
2. Update AST parser to recognize `load "filename.gr"`
3. Add `visit_load_statement` to executor
4. Keep existing `/load` REPL command for development workflow

### Phase B: Module System with `import` (Future - Phase 6)
Full module system with namespaces:

```glang
# math.gr (module file)
export func add(a, b) = a + b
export func multiply(a, b) = a * b
export pi = 3.14159

# private function (not exported)
func internal_helper() = "helper"

# main.gr
import "math.gr"          # Creates math namespace
result = math.add(5, 3)   # Qualified access required
pi_val = math.pi

# Alternative syntax with alias
import "math.gr" as m
result = m.add(5, 3)

# Selective import (future enhancement)
import { add, pi } from "math.gr"
result = add(5, 3)        # Direct access to imported symbols
```

**Implementation Requirements**:
- Module namespace isolation
- Export/import declarations
- Qualified name resolution
- Import aliasing
- Circular dependency detection

## Current State

### Existing `/load` REPL Command
- **Status**: Working, keep for development
- **Behavior**: Executes file in current ExecutionSession
- **Usage**: `/load "filename.gr"` (REPL only)

### Planned `load` Language Statement
- **Status**: To be implemented
- **Behavior**: Same as REPL command but usable in .gr files
- **Usage**: `load "filename.gr"` (in any .gr file)

### Future `import` Statement
- **Status**: Design phase
- **Behavior**: Module system with namespaces
- **Usage**: `import "module.gr"` with qualified access

## Usage Examples

### Script Execution with `load`
```glang
# shared_utils.gr
func log(message) = print("[LOG] " + message)
debug = true

# app.gr
load "shared_utils.gr"    # Direct inclusion
log("Application started")  # Function available directly
if debug {                   # Variable available directly
    log("Debug mode enabled")
}
```

### Module System with `import` (Future)
```glang
# logger.gr (module)
export func info(msg) = print("[INFO] " + msg)
export func error(msg) = print("[ERROR] " + msg)
export level = "INFO"

# app.gr
import "logger.gr"
logger.info("Application started")    # Qualified access required
if logger.level == "DEBUG" {          # Namespace prevents conflicts
    logger.info("Debug logging")
}
```

## Design Benefits

### Clear Semantic Distinction
- **`load`**: "Include this code as if I wrote it here"
- **`import`**: "Make this module available with its own namespace"

### Use Case Alignment
- **`load`**: Configuration, utilities, shared code snippets
- **`import`**: Libraries, complex modules, namespace isolation

### Migration Path
- Phase A provides immediate file inclusion capability
- Phase B adds full module system without breaking existing code
- REPL `/load` command continues to work for development

## Technical Implementation Notes

### LoadStatement AST Node
```python
@dataclass
class LoadStatement(Statement):
    """Load statement: load "filename.gr" """
    filename: str
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_load_statement(self)
```

### Parser Recognition
```python
# In parse_statement()
if self.check(TokenType.IDENTIFIER) and self.peek().value == "load":
    return self.parse_load_statement()
```

### Executor Implementation
```python
def visit_load_statement(self, node: LoadStatement) -> None:
    """Execute load statement - include file in current namespace."""
    result = self.file_manager.load_file(node.filename, self.execution_session)
    if not result.success:
        raise RuntimeError(f"Failed to load {node.filename}: {result.error}")
    self.result = result.value
```

## Error Handling

### Load Statement Errors
- File not found: Clear error with file path
- Syntax errors in loaded file: Show file and line information
- Circular loads: Detection and prevention
- Permission errors: Clear explanation

### Import Statement Errors (Future)
- Module not found: Search path information
- Export not found: Available exports list
- Circular imports: Dependency chain display
- Namespace conflicts: Collision details

## Testing Strategy

### Load Statement Tests
- Basic file loading functionality
- Error scenarios (missing file, syntax error)
- Nested loads (file A loads file B)
- Variable availability after load
- Integration with existing type inference

### Import Statement Tests (Future)
- Module namespace isolation
- Qualified access resolution
- Import aliases
- Export visibility
- Circular dependency detection

## Conclusion

This design provides:
1. **Immediate value**: `load` statement for script inclusion
2. **Clear semantics**: Different purposes for `load` vs `import`
3. **Future extensibility**: Path to full module system
4. **Development workflow**: REPL `/load` command preserved

The separation ensures developers understand when to use each mechanism and provides a solid foundation for glang's file organization capabilities.
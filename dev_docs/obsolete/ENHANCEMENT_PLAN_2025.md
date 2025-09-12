# Glang Enhancement Plan - 2025 [OBSOLETE]

**STATUS: SUPERSEDED BY DATA TYPE OPERATIONS PLAN**
**DATE OBSOLETED: 2025-01-06**

*This document has been superseded by the new data type operations plan. The module system and keyword architecture have been completed.*

## Overview
This document outlines a comprehensive plan to address fundamental architectural issues and enhance the Glang programming language based on user feedback.

## Key Issues Identified

1. **Scalar/Atomic Values Architecture**: Scalars (strings, numbers, booleans) are currently stored as single-node graphs but behave like lists
2. **Assignment and Reference Issues**: Cannot properly extract values from lists or assign them to variables
3. **Command Namespace Collision**: Variable names conflict with REPL commands
4. **Type Declaration Verbosity**: Explicit type declarations should be optional when type is obvious
5. **File Import System**: Need ability to load and execute .gr files

## Enhancement Phases

### Phase 1: Node vs Graph Architecture Refactoring ✅ COMPLETED
**Priority: CRITICAL**
**Estimated Effort: High**
**Status: COMPLETED 2025-09-03**

#### Problem
- Scalars are stored as single-node graphs but expose graph methods (append, prepend, etc.)
- This violates the conceptual model where atomic values should be immutable nodes
- Example: `string a = "hello"` allows `a.append("world")` which shouldn't be valid

#### Solution Design
1. **Separate Node and Graph concepts**:
   ```python
   # Current (problematic):
   string a = "hello"  # Creates Graph with single Node
   a.append("!")       # Works but shouldn't
   
   # Proposed:
   string a = "hello"  # Creates atomic Node value
   a.append("!")       # Error: Cannot append to atomic value
   ```

2. **Implementation Steps**: ✅ COMPLETED
   - ✅ Create `AtomicValue` class separate from `Graph`
   - ✅ Store scalars as `AtomicValue` instances  
   - ✅ Lists remain as `Graph` instances
   - ✅ Update variable storage to handle both types
   - ✅ Restrict methods based on type (no list methods on atomics)
   - ✅ Fix all legacy commands to handle AtomicValue without crashing
   - ✅ Add comprehensive test coverage (34 new tests)

3. **Type Hierarchy**:
   ```
   Value (abstract)
   ├── AtomicValue (scalars: string, num, bool)
   │   └── Methods: toString(), toNum(), toBool()
   └── Graph (collections: list, tree, graph)
       └── Methods: append(), prepend(), insert(), etc.
   ```

### Phase 2: Proper Variable Reference and Assignment
**Priority: HIGH**
**Estimated Effort: Medium**

#### Problem
- `string b = a[0]` assigns the literal string "a[0]" instead of the value
- Cannot extract values from lists properly
- No way to pass values between variables correctly

#### Solution Design
1. **Fix indexing to return actual values**:
   ```glang
   list a = ["one", "two"]
   string b = a[0]  # Should assign "one" to b
   b                # Should display: "one"
   ```

2. **Enable nested access**:
   ```glang
   list matrix = [["a", "b"], ["c", "d"]]
   string val = matrix[0][1]  # Should assign "b"
   ```

3. **Implementation Steps**:
   - Update `_handle_scalar_declaration` to evaluate index expressions
   - Create expression evaluator for complex assignments
   - Support chained indexing in assignments
   - Add proper value extraction from graphs

### Phase 3: Command Prefix System
**Priority: MEDIUM**
**Estimated Effort: Low**

#### Problem
- Variable named 'h' conflicts with help command
- No way to disambiguate between variables and commands

#### Solution Design
1. **Add slash prefix for commands**:
   ```glang
   h = "hello"     # Creates variable
   h               # Shows variable value
   /h or /help     # Shows help
   /exit or /x     # Exits REPL
   ```

2. **Implementation Steps**:
   - Update command parser to recognize `/` prefix
   - ~~Maintain backward compatibility (commands work without `/` if no conflict)~~ [CHANGED: Commands now require `/` prefix]
   - Variable names take precedence without prefix
   - Update help text to show new syntax

### Phase 4: Type Inference for Declarations
**Priority: MEDIUM**
**Estimated Effort: Medium**

#### Problem
- Verbose to always specify type: `string a = "hello"`
- Type is often obvious from the initializer

#### Solution Design
1. **Allow implicit type declarations**:
   ```glang
   # Explicit (still supported):
   string name = "Alice"
   num age = 25
   list items = ["a", "b"]
   
   # Implicit (new):
   name = "Alice"        # Inferred as string
   age = 25              # Inferred as num
   items = ["a", "b"]    # Inferred as list
   mixed = [1, "two"]    # Inferred as list (no constraint)
   ```

2. **Type Inference Rules**:
   - String literal → string type
   - Number literal → num type
   - Boolean literal → bool type
   - List literal → list type
   - Empty list → list with no constraint

3. **Implementation Steps**:
   - Update parser to handle assignment without type keyword
   - Implement type inference from initializer
   - Maintain explicit type syntax for constraints
   - Add validation for type consistency

### Phase 5: File Import System
**Priority: HIGH**
**Estimated Effort: Medium**

#### Problem
- No way to save and load programs
- Cannot build larger programs from modules
- Need persistence and code organization

#### Solution Design
1. **Basic file operations**:
   ```glang
   /load "program.gr"     # Execute file in current namespace
   /save "program.gr"     # Save current namespace to file
   /run "program.gr"      # Execute file in fresh namespace
   ```

2. **File format (.gr files)**:
   ```glang
   # program.gr
   # Glang program file
   
   list fruits = ["apple", "banana"]
   num count = 2
   
   fruits.append("cherry")
   count = 3
   ```

3. **Import system (future)**:
   ```glang
   /import "library.gr" as lib
   lib.function_name()  # Namespaced access
   ```

4. **Implementation Steps**:
   - Create file reader/writer for .gr format
   - Implement `/load` command for file execution
   - Add `/save` to export current namespace
   - Handle file errors gracefully
   - Future: module system with namespacing

## Implementation Priority Order

1. **Week 1**: Phase 3 (Command Prefix) - Quick win, improves usability
2. **Week 2-3**: Phase 1 (Node vs Graph) - Critical architecture fix
3. **Week 4**: Phase 2 (Variable References) - Depends on Phase 1
4. **Week 5**: Phase 4 (Type Inference) - Quality of life improvement
5. **Week 6**: Phase 5 (File Import) - Essential for real programs

## Testing Strategy

### Phase 1 Tests
- Verify atomic values don't have list methods
- Ensure proper type checking
- Test value immutability

### Phase 2 Tests
- Test index assignment: `b = a[0]`
- Test nested indexing: `c = matrix[0][1]`
- Test value extraction and copying

### Phase 3 Tests
- Test command prefix parsing
- Verify variable precedence
- Test backward compatibility

### Phase 4 Tests
- Test type inference for all literal types
- Verify constraint handling
- Test mixed-type lists

### Phase 5 Tests
- Test file loading and execution
- Verify namespace preservation
- Test error handling for missing files

## Success Criteria

1. **Scalars behave as atomic values**, not single-element lists
2. **Variable assignment and referencing works intuitively**
3. **No namespace collisions** between variables and commands
4. **Type declarations are optional** when type is obvious
5. **Programs can be saved and loaded** from .gr files

## Risks and Mitigations

### Risk 1: Breaking Changes
- **Risk**: Phase 1 changes fundamental architecture
- **Mitigation**: Comprehensive test suite, gradual migration

### Risk 2: Performance Impact
- **Risk**: Separating nodes and graphs may impact performance
- **Mitigation**: Profile and optimize critical paths

### Risk 3: User Confusion
- **Risk**: New command prefix system may confuse users
- **Mitigation**: Clear documentation, helpful error messages

## Future Enhancements (Post-Plan)

1. **Module System**: Import with namespacing
2. **Function Definitions**: Functions as graph structures
3. **Pattern Matching**: Match on graph structures
4. **Graph Transformations**: map, filter, reduce on graphs
5. **Persistence**: Save/load binary graph format
6. **Graph Visualization**: Built-in graph rendering
7. **Type System**: User-defined types and constraints

## Conclusion

This plan addresses the immediate pain points while laying groundwork for Glang to evolve into a full-featured graph-based programming language. The phased approach allows for incremental improvements while maintaining system stability.
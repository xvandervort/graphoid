# Glang Design Principles

*Last Updated: 2025-01-07*

This document captures the core design principles that guide Glang's development. These principles have been established through practical implementation and refinement.

## 1. Graph-Theoretic Foundation

**Principle**: All data structures are graphs at their core.

- Strings are linear graphs of character nodes
- Lists are ordered graph collections  
- Numbers and booleans are atomic graph nodes
- The namespace itself is a meta-graph

**Implication**: Operations should respect and leverage the graph nature of data.

## 2. Practical Functional Programming

**Principle**: We adopt functional programming concepts pragmatically, not dogmatically.

### Hybrid Immutability Model (Established 2025-01-07)

Not all operations are purely functional. We follow the **principle of least surprise**:

- **Transformation methods** return new values (immutable)
  - `sort()` → returns new sorted list
  - `reverse()` → returns new reversed list
  - String methods like `up()`, `down()` → return new strings
  
- **Modification methods** change the original (mutable)
  - `append()`, `prepend()`, `insert()` → modify the list in place
  - These are practical for incremental list building
  
- **Analysis methods** are non-mutating
  - `indexOf()`, `count()`, `min()`, `max()`, `sum()` → return information

**Rationale**: Pure immutability is elegant but impractical for many real-world tasks. Our hybrid approach provides clarity through naming conventions while maintaining practical usability.

## 3. Consistent Node Inheritance

**Principle**: All values inherit universal behaviors from a base node class.

### Universal Reflection Methods (Established 2025-01-07)

Every value in Glang, regardless of type, inherits these reflection capabilities:

- `type()` - Returns the type name as a string
- `methods()` - Lists all available methods
- `can(method_name)` - Checks if a method exists
- `inspect()` - Provides detailed value information
- `size()` - Returns the graph node count

**Implementation**: These are inherited from the base `GlangValue` class, with type-specific overrides where needed.

**Rationale**: This provides consistent discoverability and introspection across all data types, aligning with the graph philosophy where all values are nodes.

## 4. Method Naming Clarity

**Principle**: Method names should clearly indicate their behavior.

- Methods that **sound transformative** (`sort`, `reverse`) should return new values
- Methods that **sound mutative** (`append`, `insert`, `remove`) should modify in place
- Methods that **sound analytical** (`count`, `contains`, `indexOf`) should be read-only

**Future Consideration**: If we need both behaviors, use clear naming:
- `sort()` vs `sorted()` (mutate vs return new)
- `reverse()` vs `reversed()` (mutate vs return new)

## 5. Type Safety Through Methods

**Principle**: Type-specific operations belong to their types, not as global functions.

- Numbers have `num.to(digits)` for precision control
- Strings have `string.up()` for case conversion
- Lists have `list.sort()` for ordering

**Benefit**: Eliminates type errors like `round("string", 10)` - syntactically impossible!

## 6. Discoverability First

**Principle**: Users should be able to explore the language interactively.

- Universal reflection methods on all values
- Comprehensive `/namespace` command showing the variable graph
- Methods return meaningful values, not just status messages
- Error messages are clear and actionable

## 7. Graph Operations Preserve Structure

**Principle**: Operations on graph-based data should respect the underlying structure.

- List operations work with ordered node collections
- String operations can work at the character node level
- Transformations create new graph structures rather than corrupting existing ones

## 8. Practical Over Pure

**Principle**: We prioritize practical usability over theoretical purity.

Examples:
- Hybrid immutability instead of pure functional programming
- Allowing both `/` and non-slash REPL commands for convenience  
- Supporting multiple aliases for common operations (`up()`/`toUpper()`, `flip()`/`toggle()`)

## 9. Consistency Across Types

**Principle**: Similar operations should behave similarly across different types.

- All transformation methods return new values (strings and lists)
- All types support universal reflection methods
- Binary operators work consistently (`+` for addition/concatenation/union)

## 10. Progressive Disclosure

**Principle**: Simple things should be simple; complex things should be possible.

- Basic operations are straightforward: `list.append(5)`
- Advanced operations are available: `list1 & list2` (intersection)
- Reflection provides depth when needed: `value.inspect()`

## 11. Flexible Syntax Over Dogma

**Principle**: Simplicity and flexibility, not dogmatic adherence to unnecessary standards.

**Ruby-Inspired Approach**: Parentheses should be optional for method calls and function calls, required only when needed for disambiguation or clarity.

Examples:
- `print "Hello World"` vs `print("Hello World")` - both work
- `list.append 5` vs `list.append(5)` - both valid
- `numbers.count 1` vs `numbers.count(1)` - developer's choice
- Use parentheses when clarity demands: `calculate(a + b, c * d)` vs `calculate a + b, c * d`

**Rationale**: Language syntax should serve the developer, not force arbitrary constraints. Like Ruby, we trust developers to use parentheses when they add clarity and omit them when they don't. This reduces visual noise while preserving expressiveness.

---

## Design Decision Log

### 2025-01-07: Hybrid Immutability Model

**Issue**: List methods were mutating while string methods were immutable, creating inconsistency.

**Decision**: Adopt a hybrid model where:
- Transformations are immutable (return new values)
- Modifications are mutable (change in place)
- Analysis operations are non-mutating

**Rationale**: Pure immutability is impractical for incremental list building. This approach follows the principle of least surprise based on method naming.

### 2025-01-07: Inheritance-Based Universal Methods

**Issue**: Universal reflection methods were duplicated across type dispatchers.

**Decision**: Move universal methods to base `GlangValue` class with inheritance.

**Rationale**: Follows object-oriented principles and the graph philosophy where all values are nodes with shared behaviors.

### 2025-01-08: Ruby-Style Print Implementation

**Issue**: Initial print implementation required parentheses like Python, creating unnecessary syntactic rigidity.

**Decision**: Reimplemented print as a special function call with optional parentheses, supporting both `print "Hello"` and `print("Hello")` syntax.

**Implementation**: Changed from statement keyword to special expression parser, added `PrintExpression` AST node.

**Rationale**: Embraces the principle of "Flexible Syntax Over Dogma" - parentheses should be optional unless needed for clarity. This provides Ruby-like elegance while maintaining full backward compatibility.

---

*This document should be updated as new principles are established or existing ones are refined.*
# Graphoid: The "No Generics" Policy

**Version**: 1.0  
**Last Updated**: January 2025  
**Status**: Non-negotiable design principle

---

## Core Principle

**Graphoid will NEVER have user-space generics.** Period.

This is not negotiable. Generics are complexity cancer that metastasizes into:
- Generic classes/structs (`class Foo<T>`)
- Multiple type parameters (`HashMap<K, V>`)
- Generic constraints (`where T: Trait`)
- Higher-kinded types
- Variance annotations
- Type parameter inference hell

**Graphoid rejects ALL of this.**

---

## What We Have (Allowed)

### 1. Simple Runtime Type Assertions (Optional)

```graphoid
list<num> scores = [95, 87, 92]    # Runtime check: all elements must be numbers
hash<string> config = {...}         # Runtime check: all values must be strings
```

**These are NOT generics. They are:**
- **Single-parameter ONLY** (no `HashMap<K, V>`)
- **OPTIONAL** (you can just use `list` or `hash`)
- **RUNTIME checks**, not compile-time
- **Built-in collections ONLY**
- **NO user-defined generic types**

### 2. The Rule: One Type Parameter, Runtime-Checked, Built-In Only

| Syntax | Allowed? | Reason |
|--------|----------|--------|
| `list<num>` | ✅ Yes | Single param, built-in, runtime |
| `hash<string>` | ✅ Yes | Values only, keys always string |
| `tree<num>` | ✅ Yes | Single param, built-in, runtime |
| `graph<num>` | ✅ Yes | Single param, built-in, runtime |
| `HashMap<K, V>` | ❌ NEVER | Multiple params |
| `Result<T, E>` | ❌ NEVER | Multiple params |
| `struct Foo<T>` | ❌ NEVER | User-defined generics |
| `fn process<T>(x: T)` | ❌ NEVER | Generic functions |
| `list<list<num>>` | ❌ NEVER | Nested constraints |

---

## What We'll NEVER Have (Forbidden Forever)

### ❌ User-Defined Generic Types

```graphoid
# THIS WILL NEVER EXIST IN GRAPHOID
class Container<T> {    # FORBIDDEN
    value: T
}

struct Pair<A, B> {     # FORBIDDEN
    first: A
    second: B
}
```

**Why forbidden**: Users don't need this. If you need polymorphism, use:
- **Duck typing** (dynamic dispatch)
- **Graph rules** (structural constraints)
- **Protocols/interfaces** (future consideration)

### ❌ Multiple Type Parameters

```graphoid
# THIS WILL NEVER EXIST
hash<string, num>       # FORBIDDEN - hash only constrains values
map<K, V>              # FORBIDDEN - no multiple params
Result<T, E>           # FORBIDDEN - no error type generics
```

**Why forbidden**: One parameter is enough. Hashes have string keys - that's it. Lists hold one element type. **Simple.**

### ❌ Generic Functions

```graphoid
# THIS WILL NEVER EXIST
fn identity<T>(x: T) -> T {    # FORBIDDEN
    return x
}

fn map<T, U>(list: list<T>, fn: T -> U) -> list<U> {  # FORBIDDEN
    ...
}
```

**Why forbidden**: Functions in Graphoid work on **VALUES, not types**. Use duck typing.

### ❌ Type Constraints/Bounds

```graphoid
# THIS WILL NEVER EXIST
fn print_all<T: Printable>(items: list<T>) {  # FORBIDDEN
    ...
}

class Wrapper<T> where T: Comparable {        # FORBIDDEN
    ...
}
```

**Why forbidden**: This is generic complexity hell. Use duck typing or graph rules instead.

### ❌ Variance Annotations

```graphoid
# THIS WILL NEVER EXIST
class Producer<+T>     # Covariance - FORBIDDEN
class Consumer<-T>     # Contravariance - FORBIDDEN
class Processor<T>     # Invariance - FORBIDDEN
```

**Why forbidden**: Variance is a PhD-level concept that makes languages incomprehensible. **Graphoid is for humans.**

### ❌ Nested Type Constraints

```graphoid
# THIS WILL NEVER EXIST
list<list<num>>        # FORBIDDEN - no nesting
hash<list<string>>     # FORBIDDEN - no nesting
graph<tree<num>>       # FORBIDDEN - no nesting
```

**Why forbidden**: Nesting encourages generic thinking. Keep it flat.

---

## Implementation Boundaries

### Parser Rules (MUST ENFORCE)

The parser **MUST** enforce these rules at parse time:

1. **Type annotations only allow**:
   - Base types: `num`, `string`, `bool`, `none`, `symbol`, `time`
   - Collections with optional **single** constraint: `list`, `list<TYPE>`, `hash`, `hash<TYPE>`, `tree`, `tree<TYPE>`, `graph`, `graph<TYPE>`
   - The `<TYPE>` must be a base type only
   
2. **NEVER parse** (syntax errors):
   - Multiple angle brackets: `<T, U>` → **SYNTAX ERROR**
   - Type parameters in user definitions: `class Foo<T>` → **SYNTAX ERROR**
   - Generic function syntax: `fn foo<T>()` → **SYNTAX ERROR**
   - Nested constraints: `list<list<num>>` → **SYNTAX ERROR**
   - Type variables: `<T>` where T is not a concrete type → **SYNTAX ERROR**

### Semantic Analyzer Rules (MUST ENFORCE)

The analyzer **MUST** enforce at semantic analysis time:

1. Type constraints **only** on built-in collections (`list`, `hash`, `tree`, `graph`)
2. Only **primitive types** allowed as constraints (`num`, `string`, `bool`, `symbol`, `time`, `none`)
3. No user-defined types in constraints
4. Runtime checks insert validation code

### Runtime Behavior

When a type constraint is present:

```graphoid
list<num> scores = [95, 87, 92]
scores.append("hello")  # Runtime error: "Cannot append string to list<num>"
```

The runtime:
1. Checks type on every mutation operation (`append`, `insert`, etc.)
2. Throws descriptive error if type doesn't match
3. Does **NOT** check on read operations (performance)

### Error Messages (MUST BE CLEAR)

When users try to use generics:

```
Error: Generic types are not supported in Graphoid
  --> example.gr:5:8
   |
 5 | class Container<T> {
   |                ^^^ multiple type parameters or generic declarations forbidden
   |
Graphoid uses duck typing and graph rules instead of generics.
See: https://graphoid.dev/docs/no-generics

Hint: Remove the <T> and use duck typing, or use graph rules for constraints.
```

When users try multiple parameters:

```
Error: Multiple type parameters are not supported
  --> example.gr:3:12
   |
 3 | hash<string, num> data = {}
   |            ^^^^^^ only single type parameter allowed
   |
Graphoid collections support at most one type constraint.
Use: hash<num> (constrains values only, keys are always strings)

See: https://graphoid.dev/docs/no-generics
```

When users try nesting:

```
Error: Nested type constraints are not supported
  --> example.gr:2:5
   |
 2 | list<list<num>> matrix = [[1, 2], [3, 4]]
   |     ^^^^^^^^^^ nested type constraints forbidden
   |
Keep type constraints flat. Use `list` without constraints for nested collections.

See: https://graphoid.dev/docs/no-generics
```

---

## Alternative Patterns

Instead of generics, Graphoid provides superior alternatives:

### 1. Duck Typing (Primary Pattern)

```graphoid
fn process(item) {
    # Works on anything with a .process() method
    return item.process()
}

# Works on lists, hashes, graphs - anything iterable
fn count_elements(collection) {
    return collection.size()
}

# Works on anything with .x and .y
fn calculate_distance(point1, point2) {
    dx = point2.x - point1.x
    dy = point2.y - point1.y
    return (dx^2 + dy^2).sqrt()
}
```

**Benefits:**
- No type declarations needed
- Works with any compatible type
- Runtime dispatch
- Simple and flexible

### 2. Graph Rules (Structural Constraints)

```graphoid
# Instead of generic constraints, use graph rules
my_data = graph{}
my_data.add_rule("all_values_positive")
my_data.add_rule("max_depth", 5)
my_data.add_rule("connected")

# Rules enforce structural invariants dynamically
my_data.add_node("A", -5)  # Error: violates "all_values_positive" rule
```

**Benefits:**
- More powerful than generics
- Can express complex invariants
- Runtime enforced
- Graph-theoretic foundation

### 3. Runtime Type Checks (When Needed)

```graphoid
fn process_numbers(items) {
    # Explicit runtime check if needed
    if not items.all?(:is_number) {
        throw Error("Expected list of numbers")
    }
    return items.sum()
}

fn validate_config(config) {
    # Check structure at runtime
    required_keys = ["host", "port", "debug"]
    for key in required_keys {
        if not config.has_key(key) {
            throw Error("Missing required key: " + key)
        }
    }
    return config
}
```

**Benefits:**
- Explicit and clear
- Fail fast
- Custom error messages
- Developer controls when to check

### 4. Optional Type Hints (Documentation Only)

```graphoid
# Type annotations are HINTS for humans and tools, not enforced contracts
fn calculate_total(prices: list<num>) -> num {
    return prices.sum()
}

# This still compiles and runs:
calculate_total([1, 2, "three"])  # Runtime error when sum() encounters string
```

**Benefits:**
- Self-documenting code
- IDE autocompletion hints
- Optional static analysis
- No runtime overhead
- Not enforced rigidly

---

## The Philosophy

### Graphoid Trusts Programmers

- You don't need the type system to babysit you
- If you want type safety, add runtime checks
- If you want constraints, use graph rules
- If you want documentation, use type hints
- But the language **won't force complexity** on you

### Complexity is the Enemy. Simplicity is the Goal.

**Generic type systems add:**
- Cognitive overhead
- Verbose syntax
- Compiler complexity
- Error message confusion
- Learning curve steepness

**Graphoid prioritizes:**
- Readable code
- Fast iteration
- Human understanding
- Minimal syntax
- Get things done

### When You Think You Need Generics, You Don't

**"I need a generic container!"**
→ Use duck typing. Any collection works.

**"I need type safety!"**
→ Use runtime checks or graph rules.

**"I need reusable algorithms!"**
→ Functions work on values. Duck typing handles polymorphism.

**"I need to constrain what types are allowed!"**
→ Use built-in type constraints (`list<num>`) or graph rules.

**"But language X has generics and it's popular!"**
→ Graphoid isn't trying to be language X. Graphoid is **deliberately simple**.

---

## Enforcement Checklist

Before **ANY** feature is added to Graphoid, ask:

- [ ] Does this add generic type parameters? → **REJECT**
- [ ] Does this allow multiple type arguments? → **REJECT**
- [ ] Does this enable user-defined generic types? → **REJECT**
- [ ] Does this require type parameter inference? → **REJECT**
- [ ] Does this add variance/covariance? → **REJECT**
- [ ] Does this add higher-kinded types? → **REJECT**
- [ ] Does this make the type system more complex? → **REJECT**

If **ANY** answer is "yes", the feature is **REJECTED** immediately, no exceptions.

---

## For Contributors

### If Someone Proposes Generics

**Response template:**

> Graphoid has a strict "No Generics" policy. This is a fundamental design decision, not up for debate.
> 
> Please read: `dev_docs/NO_GENERICS_POLICY.md`
> 
> Alternative solutions:
> 1. Use duck typing
> 2. Use graph rules
> 3. Use runtime type checks
> 4. Use optional type hints for documentation
> 
> If you need generics, Graphoid is not the right language for your use case.

### Parser Implementation Must Reject

If you're implementing the parser, you **MUST**:
- Reject `<T, U>` syntax
- Reject `<T>` on user-defined types
- Reject `fn foo<T>()`
- Reject `class Foo<T>`
- Reject `list<list<num>>`

**Add specific error messages** (see above) that explain **why** and suggest alternatives.

### Code Review Red Flags

❌ Any PR that adds:
- Type parameter syntax
- Multiple type arguments
- Generic inference
- Variance annotations
- Template-like features

**Immediate rejection.** No discussion needed.

---

## Conclusion

### Graphoid Will Have

✅ Optional runtime type assertions on built-in collections (`list<num>`)  
✅ Duck typing  
✅ Graph rules for structural constraints  
✅ Type hints for documentation  
✅ Runtime type checking when needed  

### Graphoid Will NEVER Have

❌ User-defined generic types  
❌ Multiple type parameters  
❌ Generic functions  
❌ Type constraints/bounds  
❌ Variance annotations  
❌ Higher-kinded types  
❌ Nested type constraints  
❌ Any of that complexity garbage  

---

**If you want generics, use Rust, TypeScript, Haskell, Swift, or Java.**

**Graphoid is deliberately simple. This is a feature, not a bug.**

---

**End of Policy Document**

This policy is non-negotiable and will be enforced in:
- Parser (syntax errors)
- Semantic analyzer (type errors)
- Code review (immediate rejection)
- Documentation (clear warnings)
- Community standards (firm boundaries)

If you disagree with this policy, Graphoid is not the right language for you.

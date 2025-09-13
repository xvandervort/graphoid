# Regular Expression Implementation Analysis for Glang

*Created: January 2025*

## The Challenge

Regular expressions are a complex feature that requires careful consideration. Simply wrapping Python's `re` module would be the easiest path but contradicts Glang's philosophy of being a self-contained language with its own semantics.

## Implementation Options

### Option 1: Wrap Python's `re` Module (Not Recommended)
**Pros:**
- Immediate availability
- Battle-tested implementation
- Full PCRE-like features
- Minimal development effort

**Cons:**
- **Philosophical violation**: Glang should define its own semantics
- **Dependency on Python**: Ties us to Python's regex behavior forever
- **No learning opportunity**: We don't understand our own language deeply
- **Limited customization**: Can't add Glang-specific pattern features
- **Graph pattern mismatch**: Python regex doesn't understand graphs

### Option 2: Build a Simple Regex Engine (Recommended for v1.0)
**Pros:**
- **Philosophical alignment**: We control our language's behavior
- **Educational value**: Deep understanding of pattern matching
- **Customizable**: Can add Glang-specific features later
- **Graph-ready**: Can evolve to match graph patterns, not just strings

**Cons:**
- Significant development effort (2-4 weeks for basic engine)
- Initial version will be limited compared to PCRE
- Performance may be slower initially
- More testing required

### Option 3: Defer Regex to Post-1.0 (Alternative Recommendation)
**Pros:**
- **Focus on core**: Complete other critical features first
- **Time to design properly**: Can plan graph-aware pattern matching
- **User feedback**: Learn what patterns users actually need
- **No rushed decisions**: Avoid locking in a bad design

**Cons:**
- Missing expected feature for 1.0
- Some string operations harder without regex
- Users may need workarounds

### Option 4: Hybrid Approach (Pragmatic Middle Ground)
**Pros:**
- Start with Python wrapper marked as "provisional"
- Plan to replace with native implementation
- Users get functionality now
- Time to build proper solution

**Cons:**
- Breaking changes when we switch implementations
- Sets expectation we might not meet
- Technical debt

## What Would a Simple Glang Regex Engine Include?

### Phase 1: Basic Patterns (2 weeks)
```glang
pattern p = /[a-z]+@[a-z]+\.[a-z]+/  # Simple email
matches = text.match(p)
```
- Character classes: `[a-z]`, `[0-9]`, `.`
- Quantifiers: `*`, `+`, `?`, `{n,m}`
- Anchors: `^`, `$`
- Groups: `(...)` 
- Alternation: `|`

### Phase 2: Glang-Specific Features (2 weeks)
```glang
# Method-based pattern building (no regex syntax)
pattern email = Pattern.new()
    .chars("a-z0-9._")
    .one_or_more()
    .literal("@")
    .chars("a-z0-9")
    .one_or_more()
    .literal(".")
    .chars("a-z")
    .at_least(2)
```

### Phase 3: Graph Patterns (Future)
```glang
# Match patterns in graph structures
pattern triangle = GraphPattern.new()
    .node("a")
    .edge_to("b")
    .edge_to("c")
    .edge_from("c", to: "a")

matches = my_graph.find_pattern(triangle)
```

## The Deeper Question: What Pattern Matching Means in Glang

Since Glang is fundamentally about graphs, should we even have traditional regex? Or should we focus on:

1. **Graph patterns**: Finding subgraph structures
2. **Path patterns**: Matching traversal paths
3. **Node patterns**: Matching node properties
4. **Edge patterns**: Matching relationships

Traditional regex is just pattern matching on linear sequences (strings). In a graph-centric language, pattern matching should be richer.

## Recommendation

### For Immediate Roadmap (v0.9):
**Skip regex entirely**. Focus on:
1. Random number generation (simpler, well-defined)
2. Enhanced string methods that reduce regex need:
   - `string.split_on_any(chars)`
   - `string.extract_numbers()`
   - `string.extract_words()`
   - `string.is_email()` (built-in validation)
   - `string.is_url()`
   - `string.is_phone()`

### For v1.0:
**Implement basic pattern matching** as a Glang-native feature:
- Start with string patterns using method-based API
- No regex syntax initially (avoid parsing complexity)
- Build foundation for graph patterns

### For v2.0:
**Full graph-aware pattern system**:
- Unify string, list, and graph pattern matching
- Single pattern language for all data types
- True innovation in pattern matching

## Alternative: What Users Actually Need

Most regex use cases are:
1. **Validation**: Email, phone, URL, etc.
   - Solution: Built-in validators
2. **Extraction**: Find numbers, words, dates in text
   - Solution: Specialized extraction methods
3. **Splitting**: Complex string splitting
   - Solution: Enhanced split methods
4. **Replacement**: Complex find-and-replace
   - Solution: Template/format methods

We could provide these without regex and satisfy 90% of use cases.

## Decision Points

1. **Is regex critical for v1.0?** Probably not if we provide alternatives
2. **Should we wrap Python temporarily?** No, sets bad precedent
3. **Can we innovate in pattern matching?** Yes, graph patterns are unique
4. **What do users expect?** Basic string manipulation, not necessarily regex

## Proposed Action

1. **Remove regex from v1.0 roadmap**
2. **Add enhanced string methods** (1 week of work)
3. **Design graph pattern matching** for v2.0
4. **Document this decision** in roadmap

This aligns with Glang's philosophy: do things right or don't do them yet.
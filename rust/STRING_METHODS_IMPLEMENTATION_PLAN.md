# String Methods Implementation Plan

**Created**: 2025-11-12
**Status**: Ready for Implementation
**Estimated Time**: 7-11 hours total

---

## 📊 Overview

Implement missing string methods that are specified in the language spec but not yet implemented. These are **built-in methods** on the string type, NOT separate module functions.

### Current Gap

**Implemented (12 methods):**
- `length`, `upper`, `lower`, `trim`, `reverse`, `substring`, `split`
- `starts_with`, `ends_with`, `contains`, `replace`, `index_of`

**Missing (9 methods):**
- **Mutating**: `upper!()`, `lower!()`, `trim!()`, `reverse!()`
- **Advanced Pattern**: `contains(mode, patterns...)`, `extract(pattern)`, `count(pattern)`, `find(pattern, options...)`

---

## 🎯 Implementation Phases

### Phase 1: Mutating Methods (1-2 hours)

Implement the `!` suffix versions that mutate strings in place.

#### Methods to Implement

1. **`upper!()`** - Convert to uppercase in place, return `none`
2. **`lower!()`** - Convert to lowercase in place, return `none`
3. **`trim!()`** - Remove whitespace in place, return `none`
4. **`reverse!()`** - Reverse character order in place, return `none`

#### Implementation Details

- All mutating methods return `none` (not the modified string)
- Must check if string is frozen before mutation
- Update string through environment variable (strings aren't `Copy` in Rust)
- Location: `src/execution/executor.rs` in `eval_string_method()`

#### Example Implementation

```rust
"upper!" => {
    if !args.is_empty() {
        return Err(GraphoidError::runtime("upper!() takes no arguments"));
    }

    // Get the variable name from object expression
    if let Expr::Variable { name, .. } = object_expr {
        let new_string = s.to_uppercase();
        self.env.set(name, Value::string(new_string))?;
        return Ok(Value::none());  // Mutating methods return none
    }

    Err(GraphoidError::runtime("upper!() can only be called on variables"))
}
```

#### Test Requirements

- Test each method returns `none`
- Test mutation actually happens
- Test frozen string rejection
- Test calling on non-variable expressions fails appropriately
- **Total: ~20 tests**

#### Example Usage

```graphoid
text = "hello world"
text.upper!()
print(text)  # "HELLO WORLD"

result = text.lower!()
print(result)  # none (mutating methods return none)
```

---

### Phase 2: Pattern Symbol Support (2-3 hours)

Implement pattern matching infrastructure for semantic string operations.

#### Pattern Symbols to Support

| Symbol | Alias | Description |
|--------|-------|-------------|
| `:digits` | `:numbers` | Numeric characters (0-9) |
| `:letters` | - | Alphabetic characters (a-z, A-Z) |
| `:uppercase` | - | Uppercase letters (A-Z) |
| `:lowercase` | - | Lowercase letters (a-z) |
| `:spaces` | `:whitespace` | Whitespace characters |
| `:punctuation` | - | Punctuation marks |
| `:symbols` | - | Symbols (non-alphanumeric, non-whitespace) |
| `:alphanumeric` | - | Letters and numbers |
| `:words` | - | Word boundaries (letter sequences) |
| `:emails` | - | Email-like patterns |

#### Implementation Approach

Create helper function in `src/execution/executor.rs`:

```rust
fn matches_pattern(ch: char, pattern: &str) -> bool {
    match pattern {
        "digits" | "numbers" => ch.is_numeric(),
        "letters" => ch.is_alphabetic(),
        "uppercase" => ch.is_uppercase(),
        "lowercase" => ch.is_lowercase(),
        "spaces" | "whitespace" => ch.is_whitespace(),
        "punctuation" => ch.is_ascii_punctuation(),
        "alphanumeric" => ch.is_alphanumeric(),
        "symbols" => !ch.is_alphanumeric() && !ch.is_whitespace(),
        _ => false,
    }
}
```

For `:words` and `:emails`, need sequence-level matching, not char-level.

#### Test Requirements

- Test each pattern type
- Test both primary and alias forms (`:digits` and `:numbers`)
- Test edge cases (empty strings, special characters)
- **Total: ~15 tests**

---

### Phase 3: Advanced Pattern Methods (3-4 hours)

Implement the four advanced pattern methods.

---

#### Method 1: `contains(mode, patterns...)`

**Signature:** `string.contains(mode, pattern1, pattern2, ...)`

**Modes:**
- `:any` - String contains at least one match of any pattern
- `:all` - String contains at least one match of ALL patterns
- `:only` - String contains ONLY characters matching patterns (nothing else)

**Examples:**

```graphoid
"Hello123".contains(:any, :digits)           # true (has digits)
"Hello123".contains(:all, :letters, :digits) # true (has both)
"Hello World".contains(:only, :letters, :spaces) # true (only letters and spaces)
"Hello123!".contains(:only, :letters, :spaces) # false (has digits and punctuation)
```

**Implementation Notes:**
- First arg must be a symbol (`:any`, `:all`, `:only`)
- Remaining args are pattern symbols
- For `:only`, check that ALL characters match at least one pattern

**Tests:** ~12 tests

---

#### Method 2: `extract(pattern)`

**Signature:** `string.extract(pattern) -> list<string>`

**Behavior:** Extract all substrings matching the pattern

**Examples:**

```graphoid
"Hello World 123".extract(:numbers)  # ["123"]
"Call 555-1234 or email test@example.com".extract(:emails)
  # ["test@example.com"]
"Hello World Test".extract(:words)   # ["Hello", "World", "Test"]
"one, two, three".extract(:letters)  # ["one", "two", "three"]
```

**Implementation:**
- For char-level patterns (`:digits`, `:letters`, etc.): Extract continuous sequences
- For `:words`: Extract sequences of letters (word boundaries)
- For `:numbers`: Extract sequences of digits
- For `:emails`: Extract email-like patterns (simple heuristic: `word@word.word`)
- Return list of matching substrings

**Email Pattern Heuristic:**
```
<alphanumeric+>@<alphanumeric+>.<alphanumeric+>
```

**Tests:** ~10 tests

---

#### Method 3: `count(pattern)`

**Signature:** `string.count(pattern) -> num`

**Behavior:** Count occurrences of pattern

**Examples:**

```graphoid
"Hello World 123".count(:digits)     # 3 (count individual digits)
"Hello World".count(:words)          # 2 (count word sequences)
"test@example.com and hello".count(:emails) # 1 (count email patterns)
"a b c".count(:letters)              # 3 (count individual letters)
```

**Implementation:**
- For char-level patterns: Count individual characters
- For sequence patterns (`:words`, `:numbers`, `:emails`): Count sequences/matches

**Tests:** ~8 tests

---

#### Method 4: `find(pattern, limit_or_mode)`

**Signature:**
- `string.find(pattern)` -> `list<num>` (all positions)
- `string.find(pattern, limit)` -> `list<num>` (first N positions)
- `string.find(pattern, :first)` -> `num` (first position only, -1 if not found)

**IMPORTANT:** Uses `:first` symbol, NOT a separate `find_first()` method.
**Why:** Avoids method proliferation (KISS principle).

**Examples:**

```graphoid
"Hello World 123".find(:digits)        # [12, 13, 14] (positions of all digits)
"Hello World 123".find(:digits, 2)     # [12, 13] (first 2 positions)
"Hello World 123".find(:digits, :first) # 12 (first position only)
"No digits here".find(:digits, :first)  # -1 (not found)
```

**Implementation:**
- Default (1 arg): Return list of all positions
- Numeric second arg: Limit number of results
- `:first` symbol: Return single number (first position or -1)

**Tests:** ~12 tests

---

### Phase 4: Integration and Examples (1-2 hours)

Create comprehensive example file and verify end-to-end functionality.

#### Create `samples/string_methods.gr`

```graphoid
# Demonstrate all string methods

print("=== Mutating Methods ===")
text = "hello world"
text.upper!()
print(text)  # "HELLO WORLD"

text.lower!()
print(text)  # "hello world"

text = "  spaces  "
text.trim!()
print(text)  # "spaces"

text = "hello"
text.reverse!()
print(text)  # "olleh"

print("\n=== Advanced Pattern Methods ===")
mixed = "Hello World 123! Email: test@example.com"

# contains with modes
print(mixed.contains(:any, :digits))        # true
print(mixed.contains(:all, :letters, :digits)) # true
print("abc".contains(:only, :letters))      # true
print("abc123".contains(:only, :letters))   # false

# extract patterns
numbers = mixed.extract(:numbers)
print(numbers)  # ["123"]

words = mixed.extract(:words)
print(words)    # ["Hello", "World", "Email"]

emails = mixed.extract(:emails)
print(emails)   # ["test@example.com"]

# count patterns
digit_count = mixed.count(:digits)
print(digit_count)  # 3

word_count = mixed.count(:words)
print(word_count)   # 3

# find patterns
positions = mixed.find(:digits)
print(positions)    # [12, 13, 14]

first_digit = mixed.find(:digits, :first)
print(first_digit)  # 12

first_two = mixed.find(:digits, 2)
print(first_two)    # [12, 13]

not_found = "no digits".find(:digits, :first)
print(not_found)    # -1
```

#### Verification

```bash
cd /home/irv/work/grang/rust
cargo run --quiet samples/string_methods.gr
```

---

## 🧪 Test-Driven Development (TDD)

**For EACH method, follow this cycle:**

1. 🔴 **RED**: Write failing tests first
2. 🟢 **GREEN**: Implement minimal code to pass tests
3. 🔵 **REFACTOR**: Clean up while keeping tests passing

### Test File: `tests/unit/string_methods_tests.rs`

```rust
use graphoid::execution::Executor;
use graphoid::values::Value;

#[test]
fn test_upper_mutating() {
    let mut executor = Executor::new();
    let code = r#"
        text = "hello"
        result = text.upper!()
        text
    "#;

    let result = executor.execute(code).unwrap();

    // Verify mutation happened
    assert_eq!(executor.env.get("text").unwrap().as_string(), "HELLO");

    // Verify upper!() returned none
    let result_val = executor.env.get("result").unwrap();
    assert!(result_val.is_none());
}

#[test]
fn test_contains_any_mode() {
    let mut executor = Executor::new();
    let code = r#"
        result = "Hello123".contains(:any, :digits)
    "#;

    executor.execute(code).unwrap();
    let result = executor.env.get("result").unwrap();
    assert_eq!(result.as_bool(), true);
}

#[test]
fn test_extract_numbers() {
    let mut executor = Executor::new();
    let code = r#"
        result = "Hello 123 World 456".extract(:numbers)
    "#;

    executor.execute(code).unwrap();
    let result = executor.env.get("result").unwrap();
    let list = result.as_list().unwrap();
    assert_eq!(list.len(), 2);
    assert_eq!(list[0].as_string(), "123");
    assert_eq!(list[1].as_string(), "456");
}

// ... more tests
```

### Register Tests in `tests/unit_tests.rs`

```rust
mod string_methods_tests;
```

---

## 📊 Success Criteria

When implementation is complete:

- ✅ All 4 mutating methods implemented and tested
- ✅ All 11 pattern symbols supported (including aliases)
- ✅ All 4 advanced pattern methods implemented and tested
- ✅ ~67 new tests passing:
  - 20 tests (mutating methods)
  - 15 tests (pattern symbols)
  - 12 tests (contains)
  - 10 tests (extract)
  - 8 tests (count)
  - 12 tests (find)
- ✅ Total tests: 844+ (777 current + 67 new)
- ✅ Working example file: `samples/string_methods.gr`
- ✅ Zero compiler warnings
- ✅ All methods accessible from .gr programs
- ✅ Documentation updated

---

## 📁 File Locations

**Implementation:**
- `src/execution/executor.rs` - Add to `eval_string_method()` (line ~2358)

**Tests:**
- `tests/unit/string_methods_tests.rs` - New file
- `tests/unit_tests.rs` - Register new test module

**Examples:**
- `samples/string_methods.gr` - New file

**Documentation:**
- Update `rust/SESSION_SUMMARY.md` when complete
- Update `rust/START_HERE_NEXT_SESSION.md` after completion

---

## 🎯 Implementation Checklist

### Before Starting
- [ ] Review language spec sections on strings
- [ ] Understand current string method implementation
- [ ] Review pattern symbols list
- [ ] Understand mutation convention

### Phase 1: Mutating Methods
- [ ] Create `tests/unit/string_methods_tests.rs`
- [ ] Write tests for `upper!()` (TDD - RED)
- [ ] Implement `upper!()` (GREEN)
- [ ] Write tests for `lower!()` (RED)
- [ ] Implement `lower!()` (GREEN)
- [ ] Write tests for `trim!()` (RED)
- [ ] Implement `trim!()` (GREEN)
- [ ] Write tests for `reverse!()` (RED)
- [ ] Implement `reverse!()` (GREEN)
- [ ] Refactor if needed (REFACTOR)
- [ ] All Phase 1 tests passing (~20 tests)

### Phase 2: Pattern Symbols
- [ ] Create `matches_pattern()` helper function
- [ ] Write tests for each pattern type (RED)
- [ ] Implement pattern matching logic (GREEN)
- [ ] Test primary and alias forms
- [ ] All Phase 2 tests passing (~15 tests)

### Phase 3: Advanced Methods
- [ ] Write tests for `contains(mode, patterns...)` (RED)
- [ ] Implement `contains()` (GREEN)
- [ ] Write tests for `extract(pattern)` (RED)
- [ ] Implement `extract()` (GREEN)
- [ ] Write tests for `count(pattern)` (RED)
- [ ] Implement `count()` (GREEN)
- [ ] Write tests for `find(pattern, options...)` (RED)
- [ ] Implement `find()` (GREEN)
- [ ] All Phase 3 tests passing (~42 tests)

### Phase 4: Integration
- [ ] Create `samples/string_methods.gr`
- [ ] Test all mutating methods from .gr
- [ ] Test all pattern methods from .gr
- [ ] Verify example runs successfully
- [ ] Update documentation

### Final Verification
- [ ] All 844+ tests passing
- [ ] Zero compiler warnings
- [ ] Example file runs successfully
- [ ] Code is clean and well-documented
- [ ] Update `SESSION_SUMMARY.md`
- [ ] Update `START_HERE_NEXT_SESSION.md`

---

## 💡 Implementation Notes

### String Mutation Challenge

Rust strings aren't `Copy`, so mutating methods need special handling:

**Pattern to Follow:**
```rust
"upper!" => {
    if !args.is_empty() {
        return Err(GraphoidError::runtime("upper!() takes no arguments"));
    }

    // Must get variable name from object expression
    if let Expr::Variable { name, .. } = object_expr {
        let new_string = s.to_uppercase();
        self.env.set(name, Value::string(new_string))?;
        return Ok(Value::none());
    }

    Err(GraphoidError::runtime("upper!() can only be called on variables"))
}
```

**Key Points:**
- Extract variable name from `object_expr`
- Create new string with modification
- Update environment variable
- Return `Value::none()`
- Error if called on non-variable (e.g., `"literal".upper!()`)

### Pattern Matching Helper

```rust
fn matches_pattern(ch: char, pattern: &str) -> bool {
    match pattern {
        "digits" | "numbers" => ch.is_numeric(),
        "letters" => ch.is_alphabetic(),
        "uppercase" => ch.is_uppercase(),
        "lowercase" => ch.is_lowercase(),
        "spaces" | "whitespace" => ch.is_whitespace(),
        "punctuation" => ch.is_ascii_punctuation(),
        "alphanumeric" => ch.is_alphanumeric(),
        "symbols" => !ch.is_alphanumeric() && !ch.is_whitespace(),
        _ => false,
    }
}
```

### Email Pattern Heuristic

Simple email detection (no full RFC 5322 validation needed):

```rust
fn is_email_like(s: &str) -> bool {
    // Simple heuristic: word@word.word
    let parts: Vec<&str> = s.split('@').collect();
    if parts.len() != 2 {
        return false;
    }

    let domain_parts: Vec<&str> = parts[1].split('.').collect();
    domain_parts.len() >= 2 &&
        !parts[0].is_empty() &&
        domain_parts.iter().all(|p| !p.is_empty())
}
```

### Extracting Sequences

For `extract()` and similar methods:

```rust
fn extract_sequences<F>(s: &str, matcher: F) -> Vec<String>
where
    F: Fn(char) -> bool,
{
    let mut sequences = Vec::new();
    let mut current = String::new();

    for ch in s.chars() {
        if matcher(ch) {
            current.push(ch);
        } else if !current.is_empty() {
            sequences.push(current.clone());
            current.clear();
        }
    }

    if !current.is_empty() {
        sequences.push(current);
    }

    sequences
}
```

---

## 🔗 Key Design Principles

### 1. KISS - No Method Proliferation

✅ **Good:** `find(pattern, :first)` - One method, parameter controls behavior
❌ **Bad:** `find_first(pattern)` - Separate method for similar operation

This is a core Graphoid principle: **one method with parameters beats many similar methods**.

### 2. Mutation Convention

- Methods without `!`: Return new value, leave original unchanged (immutable)
- Methods with `!`: Modify in place, return `none` (mutating)
- This makes mutation **explicit and visible** at call sites

### 3. Pattern Symbols

- Pre-defined by the language/stdlib
- Users cannot create custom symbols
- Symbols are interned for efficiency
- Support both primary names and aliases (`:digits` / `:numbers`)

### 4. Immutable First

- Default methods are immutable
- Mutating methods require explicit `!` suffix
- Encourages functional programming style
- Makes data flow easier to reason about

---

## 📚 Reference Materials

### Language Specification
- **Strings**: `dev_docs/LANGUAGE_SPECIFICATION.md` lines 145-175
- **String Semantic Methods**: Lines 2085-2108
- **Mutation Convention**: Lines 2321-2376
- **Pattern Symbols**: Line 176

### Current Implementation
- **String methods**: `src/execution/executor.rs` lines 2357-2550
- **String indexing**: Lines 953-991
- **Method call handling**: Lines 1344-1396
- **List mutating methods**: Reference for mutation pattern

### Test Examples
- **List method tests**: `tests/unit/list_methods_tests.rs`
- **Map method tests**: `tests/unit/map_methods_tests.rs`

---

## ⏱️ Time Estimates

**Phase 1 (Mutating):** 1-2 hours
- 4 methods × 15-20 min each
- Testing and debugging

**Phase 2 (Patterns):** 2-3 hours
- Helper function implementation
- 11 pattern types
- Testing each type + aliases

**Phase 3 (Advanced):** 3-4 hours
- `contains()`: 45-60 min
- `extract()`: 45-60 min
- `count()`: 30-45 min
- `find()`: 45-60 min

**Phase 4 (Integration):** 1-2 hours
- Example file creation
- End-to-end testing
- Documentation updates

**Total:** 7-11 hours

**Recommended:** Split across 2-3 sessions if needed.

---

## 🎉 After Completion

Once string methods are complete, recommended next steps:

1. **Continue Phase 12**: Implement remaining stdlib modules
   - Regex module (pattern matching)
   - I/O module (file operations)
   - JSON module (parsing/serialization)
   - YAML module (parsing/serialization)
   - Crypto module (hashing/encryption)

2. **Phase 11**: Implement pure Graphoid stdlib
   - Can now use complete string methods
   - Statistics module
   - More high-level utilities

3. **Phase 13**: RSpec-style testing framework
   - Built-in testing DSL
   - Professional test runner

---

## ✅ Quick Start

```bash
cd /home/irv/work/grang/rust

# Verify current state
cargo test   # Should show 777 tests passing
cargo build  # Should compile with 0 warnings

# Start implementation
# Just ask Claude: "Implement missing string methods with TDD"
# or: "Start Phase 1 of string methods plan"
```

---

**Remember:**
1. **TDD always** - RED → GREEN → REFACTOR
2. **Three-level validation** - Rust tests + executor registration + .gr examples
3. **KISS principle** - Avoid method proliferation
4. **Mutating methods return `none`** - Not the modified value
5. **A feature isn't done if .gr files can't use it**

**Let's implement complete string support! 🚀**

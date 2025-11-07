# Enhanced String Methods Design

*Created: January 2025*

## Current Issues

1. **Inconsistent naming**: `findAll` should be `find_all` for consistency
2. **Regex dependency**: Methods like `findAll`, `matches`, `replace` require regex patterns
3. **Missing semantic shortcuts**: `contains()` could have semantic variants like `contains("numbers")`
4. **Limited extraction methods**: No easy way to extract common patterns

## Proposed String Method Enhancements

### 1. Semantic Contains Methods

Instead of just `contains(substring)`, add semantic pattern checking:

```glang
text = "Hello World 123"

# Current (only substring matching)
text.contains("World")     # true
text.contains("123")        # true

# Proposed semantic shortcuts
text.contains_any("digits")     # true (has "123")
text.contains_any("letters")    # true (has "Hello World")
text.contains_any("uppercase")  # true (has "H" and "W")
text.contains_any("lowercase")  # true (has "ello orld")
text.contains_any("spaces")     # true (has spaces)
text.contains_any("punctuation") # false
text.contains_any("symbols")    # false

# Multiple checks
text.contains_all("letters", "digits")  # true (has both)
text.contains_only("letters", "spaces") # false (also has digits)
```

### 2. Enhanced Extraction Methods

Replace regex-based extraction with semantic methods:

```glang
text = "Contact John at john@example.com or call 555-1234"

# Extract common patterns
text.extract_emails()      # ["john@example.com"]
text.extract_numbers()     # ["555", "1234"] or ["555-1234"] with option
text.extract_words()       # ["Contact", "John", "at", "john", "example", "com", "or", "call"]
text.extract_urls()        # [] (none in this example)
text.extract_phone_numbers() # ["555-1234"]

# Extract by character type
text.count("digits")       # Count digits in text
text.count("letters")      # Count letters in text  
text.count("uppercase")    # Count uppercase letters
text.count("words")        # Count word sequences
```

### 3. Count Methods

Count specific patterns with a unified interface:

```glang
text = "Hello World 123!"
password = "MyP@ssw0rd"

# Count by pattern type
text.count("digits")       # 3 - count all digits
text.count("letters")      # 10 - count all letters  
text.count("uppercase")    # 2 - count uppercase letters
text.count("words")        # 2 - count word sequences
text.count("spaces")       # 1 - count spaces
password.count("symbols")  # 1 - count symbols (@)

# Count specific characters/substrings
email = "user@example.com"
email.count("@")           # 1 - single character
email.count_chars("com")   # 1 - multi-character substring
```

### 4. Find First Methods

Find position of first occurrence:

```glang
text = "Hello World 123!"

# Find by pattern type
text.find_first("digits")     # 12 - position of first digit
text.find_first("uppercase")  # 0 - position of first uppercase  
text.find_first("space")      # 5 - position of first space

# Find specific characters/substrings
text.find_first_char(" ")     # 5 - single character
text.find_first_char("World") # 6 - multi-character substring
text.find_first_char("xyz")   # -1 - not found
```

### 5. Validation Methods

Common validation without regex:

```glang
email = "user@example.com"
phone = "555-123-4567"
url = "https://example.com"

# Validation methods
email.is_email()           # true
phone.is_phone_number()    # true (recognizes common formats)
url.is_url()              # true
"123.45".is_number()      # true
"abc123".is_alphanumeric() # true
"hello".is_alphabetic()   # true
"123".is_numeric()        # true
```

### 4. Enhanced Split Methods

More flexible splitting without regex:

```glang
text = "apple,banana;orange|grape"

# Current
text.split(",")            # ["apple", "banana;orange|grape"]

# Proposed
text.split_on_any(",;|")   # ["apple", "banana", "orange", "grape"]
text.split_on_spaces()     # Split on any whitespace
text.split_on_lines()      # Split on newlines
text.split_into_words()    # Smart word extraction
text.split_into_sentences() # Smart sentence extraction
```

### 5. Pattern Finding Without Regex

Replace `findAll` with semantic methods:

```glang
text = "The price is $19.99 and the tax is $1.50"

# Instead of regex-based findAll
text.find_all(r"\$\d+\.\d+")  # Current: requires regex

# Proposed semantic approach
text.find_numbers()        # ["19.99", "1.50"]
text.find_prices()         # ["$19.99", "$1.50"]
text.find_words()          # ["The", "price", "is", "and", "the", "tax", "is"]
text.find_capitalized()    # ["The"]
```

### 6. Fix Naming Inconsistencies

Rename methods to follow snake_case convention:
- `findAll` → `find_all`
- `toUpper` → `to_upper` (keep `up` as shorthand)
- `toLower` → `to_lower` (keep `down` as shorthand)

### 7. Character Class Predicates

For use with filter operations:

```glang
chars = "Hello123!".chars()  # ['H', 'e', 'l', 'l', 'o', '1', '2', '3', '!']

# Filter using predicates
chars.filter("letter")     # ['H', 'e', 'l', 'l', 'o']
chars.filter("digit")      # ['1', '2', '3']
chars.filter("uppercase")  # ['H']
chars.filter("lowercase")  # ['e', 'l', 'l', 'o']
chars.filter("punctuation") # ['!']
```

## Implementation Strategy

### Phase 1: Core Enhancements (1-2 days)
1. Fix naming inconsistencies (`findAll` → `find_all`)
2. Add `contains_any()`, `contains_all()`, `contains_only()`
3. Add basic extraction: `extract_numbers()`, `extract_words()`
4. Add basic validation: `is_email()`, `is_number()`, `is_url()`

### Phase 2: Advanced Features (2-3 days)
1. Add `split_on_any()`, `split_into_words()`
2. Add more extraction methods
3. Add phone number recognition
4. Character class support for contains/extract

### Phase 3: Remove Regex Dependencies (Optional)
1. Reimplement `matches()` with pattern objects
2. Reimplement `replace()` with semantic patterns
3. Deprecate regex-based methods

## Benefits

1. **No regex knowledge required**: Users can do common operations without learning regex
2. **More readable code**: `text.is_email()` vs `text.matches(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")`
3. **Consistent with Glang philosophy**: Semantic, intuitive methods
4. **Graph-ready**: These patterns can later extend to graph structures
5. **Eliminates regex dependency**: Most use cases covered without regex engine

## Examples in Practice

```glang
# Email validation and extraction
input = "Contact us at support@example.com or sales@example.com"
if input.contains_any("email") {
    emails = input.extract_emails()
    for email in emails {
        if email.is_email() {
            print("Valid email: " + email)
        }
    }
}

# Data cleaning
dirty_phone = "(555) 123-4567 ext. 890"
clean_phone = dirty_phone.extract_digits()  # "5551234567890"
main_number = clean_phone.substring(0, 10)  # "5551234567"

# Smart parsing
csv_line = "John,Doe,30,john@example.com"
fields = csv_line.split(",")
if fields[3].is_email() {
    user_email = fields[3]
}

# Pattern checking without regex
password = "MyP@ssw0rd"
has_upper = password.contains_any("uppercase")
has_lower = password.contains_any("lowercase")
has_digit = password.contains_any("digits")
has_symbol = password.contains_any("symbols")
is_strong = has_upper && has_lower && has_digit && has_symbol
```

## The Philosophy: Why We Don't Need Regex Yet

### Glang's Approach to Pattern Matching
Glang deliberately **does not include a regular expression engine** in v1.0. This isn't an oversight - it's a philosophical choice:

1. **Regex is a Power Tool**: Regex is incredibly powerful but has a steep learning curve
2. **90% Rule**: 90% of "regex use cases" are actually simple pattern operations
3. **Semantic > Syntax**: `text.is_email()` is more readable than `text.matches(r"^[a-zA-Z0-9._%+-]+@...")`
4. **Future Innovation**: We're planning graph-aware pattern matching for v2.0

### What We Provide Instead
Rather than implementing regex, we provide semantic methods that cover the most common use cases:

**Instead of this regex:**
```python
import re
# Email validation
if re.match(r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$', email):
    # Extract all numbers
    numbers = re.findall(r'-?\d+\.?\d*', text)
    # Count uppercase letters
    upper_count = len(re.findall(r'[A-Z]', password))
    # Split on multiple delimiters
    parts = re.split(r'[,;|]+', data)
```

**Use this Glang:**
```glang
# Email validation
if email.is_email() {
    # Extract all numbers
    numbers = text.extract_numbers()
    # Count uppercase letters
    upper_count = password.count("uppercase")
    # Split on multiple delimiters
    parts = data.split_on_any(",;|")
}
```

### When You Actually Need Regex
If you have a use case that our semantic methods don't cover, you can:
1. Use the existing `find_all()` and `matches()` methods (they accept regex patterns)
2. Request new semantic methods for common patterns
3. Wait for v2.0's graph pattern matching system

### The Future: Graph Pattern Matching
Glang's long-term vision goes beyond traditional regex:
```glang
# Future v2.0 - Pattern matching on any graph structure
pattern email_pattern = Pattern.new()
    .chars("a-z0-9._")
    .one_or_more()
    .literal("@")
    .chars("a-z0-9.-")
    .one_or_more()
    .literal(".")
    .chars("a-z")
    .at_least(2)

# Or even graph structure patterns
pattern triangle = GraphPattern.new()
    .node("A")
    .edge_to("B") 
    .edge_to("C")
    .edge_from("C", to: "A")
```

## Implementation Status: Complete ✅ - Unified Interface

**Enhanced with unified interface** - We built comprehensive semantic string methods with a clean, unified interface that eliminates both regex needs AND method proliferation while maintaining Glang's philosophy of intuitive, readable code.

### Final Implementation: Unified Methods

Instead of proliferating dozens of similar methods, we implemented a unified interface:

#### Unified `contains()` Method
```glang
# Single method with mode parameter instead of 3 separate methods
text.contains("any", "digits")           # Instead of text.contains_any("digits")
text.contains("all", "letters", "digits") # Instead of text.contains_all("letters", "digits")  
text.contains("only", "letters", "spaces") # Instead of text.contains_only("letters", "spaces")

# Backward compatibility - old substring search still works
text.contains("World")                   # Still works: substring search
```

#### Unified `extract()` Method
```glang  
# Single method with pattern parameter instead of multiple methods
text.extract("numbers")                  # Instead of text.extract_numbers()
text.extract("words")                    # Instead of text.extract_words()
text.extract("emails")                   # Instead of text.extract_emails()
```

#### Unified `count()` Method
```glang
# Already implemented with unified interface
text.count("digits")                     # Count digits
text.count("words")                      # Count words
text.count_chars("@")                    # Count specific characters (kept separate - different purpose)
```

#### Simple Validation Methods (Kept Simple)
```glang
# These stay simple since they're just predicates
email.is_email()                         # Simple boolean check
number.is_number()                       # Simple boolean check  
url.is_url()                            # Simple boolean check
```

### Key Benefits of Unified Interface

1. **Eliminates Method Proliferation**: Instead of `contains_any`, `contains_all`, `contains_only`, we have one `contains()` method
2. **Consistent Learning Pattern**: Users learn `method(mode, pattern...)` instead of dozens of method names
3. **Full Backward Compatibility**: Old `contains("substring")` still works alongside new `contains("mode", "pattern")`
4. **Extensible Design**: Adding new patterns only requires updating the pattern list, not creating new methods
5. **Follows Glang Philosophy**: Semantic, readable, intuitive without cognitive overhead

### Pattern Types Supported

**Character Types**: `digits`/`numbers`, `letters`, `uppercase`, `lowercase`, `spaces`/`whitespace`, `punctuation`, `symbols`, `alphanumeric`

**Content Types**: `words`, `emails` (extraction only)

### Complete Example
```glang
text = "Hello World 123! Contact us at support@example.com"

# Unified contains interface
if text.contains("all", "letters", "digits", "punctuation") {
    print("Text has letters, digits, and punctuation")
}

# Unified extraction interface  
emails = text.extract("emails")          # ["support@example.com"]
numbers = text.extract("numbers")        # ["123"]

# Unified counting interface
digit_count = text.count("digits")       # 3
word_count = text.count("words")         # 7

# Simple validation
for email in emails {
    if email.is_email() {
        print("Valid: " + email)
    }
}
```

This implementation successfully demonstrates how to create powerful, flexible APIs without method explosion while maintaining full backward compatibility and Glang's core design principles.
# Regex Module

The **regex** module provides comprehensive regular expression pattern matching and text processing capabilities. While Glang's built-in string methods handle 90% of common text processing needs, regex is essential for complex pattern matching, validation, and text transformation tasks.

## Overview

Regular expressions in Glang are designed to complement the existing unified string interface, providing powerful pattern matching for cases where simple string operations aren't sufficient.

### Key Features

- **Pattern Matching**: `match()`, `search()`, `validate()` for different matching scenarios
- **Text Extraction**: `find_all()`, `find_groups()` for extracting data from text
- **Text Transformation**: `replace()` with capture group support
- **Advanced Features**: Regex flags, pattern escaping, comprehensive error handling
- **Performance**: Automatic pattern caching for repeated operations

## Basic Usage

```glang
import "regex" as regex

# Basic pattern matching
pattern = "\\d{3}-\\d{3}-\\d{4}"
text = "Call 555-123-4567 for help"

# Check if pattern exists anywhere in text
found = regex.search(pattern, text)  # true

# Validate entire text matches pattern
phone = "555-123-4567"
is_valid = regex.validate(pattern, phone)  # true

# Check if pattern matches at start of text
starts_with_phone = regex.match(pattern, text)  # false
```

## Pattern Matching Functions

### `match(pattern, text, flags?)`

Tests if pattern matches at the **beginning** of text.

```glang
# Match URL at start of string
url_pattern = "https?://"
text1 = "https://example.com"
text2 = "Visit https://example.com"

regex.match(url_pattern, text1)  # true
regex.match(url_pattern, text2)  # false (doesn't start with pattern)
```

### `search(pattern, text, flags?)`

Tests if pattern is found **anywhere** in text.

```glang
# Find email addresses anywhere in text
email_pattern = "\\b[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}\\b"
text = "Contact alice@example.com for support"

regex.search(email_pattern, text)  # true
```

### `validate(pattern, text, flags?)`

Tests if **entire text** matches pattern (full match).

```glang
# Validate complete input
date_pattern = "\\d{4}-\\d{2}-\\d{2}"

regex.validate(date_pattern, "2025-01-15")        # true
regex.validate(date_pattern, "2025-01-15 10:30")  # false (extra text)
```

## Text Extraction Functions

### `find_all(pattern, text, flags?)`

Finds all non-overlapping matches of pattern in text.

```glang
# Extract all numbers from text
number_pattern = "\\d+"
text = "Order 123 contains 45 items costing $67.89"

numbers = regex.find_all(number_pattern, text)
# Returns: ["123", "45", "67", "89"]
```

### `find_groups(pattern, text, flags?)`

Finds all matches with capture groups.

```glang
# Parse structured data
data_pattern = "(\\w+):\\s*([^,]+)"
text = "name: Alice, age: 30, city: New York"

groups = regex.find_groups(data_pattern, text)
# Returns: [["name", "Alice"], ["age", "30"], ["city", "New York"]]

# Access individual matches
for match in groups.elements {
    key = match.elements[0].value    # "name", "age", "city"
    value = match.elements[1].value  # "Alice", "30", "New York"
}
```

## Text Transformation Functions

### `replace(pattern, replacement, text, flags?)`

Replaces all occurrences of pattern with replacement text.

```glang
# Simple replacement
number_pattern = "\\d+"
text = "I have 42 apples and 17 oranges"

result = regex.replace(number_pattern, "X", text)
# Result: "I have X apples and X oranges"

# Replacement with capture groups
email_pattern = "(\\w+)@(\\w+\\.\\w+)"
text = "Contact alice@example.com for help"

result = regex.replace(email_pattern, "$1 at $2", text)
# Result: "Contact alice at example.com for help"
```

### `split(pattern, text, flags?)`

Splits text using regex pattern as delimiter.

```glang
# Split on multiple delimiters
delimiter_pattern = "[,;:|]"
text = "apple,banana;orange:grape|kiwi"

parts = regex.split(delimiter_pattern, text)
# Returns: ["apple", "banana", "orange", "grape", "kiwi"]
```

## Utility Functions

### `escape(text)`

Escapes special regex characters in text for literal matching.

```glang
# Make special characters literal
special_text = "Price: $19.99 (includes tax)"
escaped = regex.escape(special_text)
# Result: "Price:\\ \\$19\\.99\\ \\(includes\\ tax\\)"

# Use escaped text as literal pattern
search_text = "The price is: Price: $19.99 (includes tax) total"
found = regex.search(escaped, search_text)  # true
```

## Regex Flags

Modify regex behavior with flag strings:

| Flag | Description | Example |
|------|-------------|---------|
| `i` | Case insensitive | `regex.search("HELLO", "hello world", "i")` |
| `m` | Multiline mode | `regex.search("^World", "Hello\\nWorld", "m")` |
| `s` | Dotall (. matches newlines) | `regex.search("Hello.*World", "Hello\\nWorld", "s")` |
| `x` | Verbose mode | Ignore whitespace in patterns |
| `a` | ASCII matching only | ASCII-only `\\w`, `\\d`, etc. |

```glang
# Combine multiple flags
pattern = "^hello.*world$"
text = "HELLO\\nWORLD"
flags = "ims"  # case-insensitive + multiline + dotall

result = regex.search(pattern, text, flags)  # true
```

## Common Patterns

### Email Validation

```glang
email_pattern = "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"

regex.validate(email_pattern, "user@example.com")     # true
regex.validate(email_pattern, "invalid.email")       # false
```

### Phone Number Extraction

```glang
phone_pattern = "\\b\\d{3}-\\d{3}-\\d{4}\\b"
text = "Call 555-123-4567 or 555-987-6543 for help"

phones = regex.find_all(phone_pattern, text)
# Returns: ["555-123-4567", "555-987-6543"]
```

### URL Parsing

```glang
url_pattern = "https?://([^/]+)(/[^?\\s]*)?(?:\\?([^&\\s]*))?"
text = "Visit https://example.com/docs/guide?version=2.0 for help"

parts = regex.find_groups(url_pattern, text)
# Returns: [["example.com", "/docs/guide", "version=2.0"]]
```

### HTML Tag Removal

```glang
html_pattern = "<[^>]+>"
html_text = "<p>Hello <b>World</b>! Visit <a href='link'>here</a>.</p>"

clean_text = regex.replace(html_pattern, "", html_text)
# Result: "Hello World! Visit here."
```

### Password Strength Validation

```glang
# At least 8 chars, with uppercase, lowercase, digit, and special char
strong_password_pattern = "^(?=.*[a-z])(?=.*[A-Z])(?=.*\\d)(?=.*[@$!%*?&])[A-Za-z\\d@$!%*?&]{8,}$"

regex.validate(strong_password_pattern, "MyPass123!")  # true
regex.validate(strong_password_pattern, "weakpass")   # false
```

## Best Practices

### 1. Use Raw Strings for Patterns
Always use raw strings (with `\\`) to avoid double-escaping:

```glang
# Good: Raw string with proper escaping
pattern = "\\d{3}-\\d{3}-\\d{4}"

# Avoid: Double escaping
pattern = "\\\\d{3}-\\\\d{3}-\\\\d{4}"  # Harder to read
```

### 2. Validate Input Early
Use `validate()` for strict input validation:

```glang
# Validate complete input format
if regex.validate("^\\d{4}-\\d{2}-\\d{2}$", user_date) {
    # Process valid date
} else {
    # Handle invalid format
}
```

### 3. Escape User Input
Always escape user-provided text when using it in patterns:

```glang
user_text = "Price: $19.99"
escaped_pattern = regex.escape(user_text)
# Now safe to use as literal pattern
```

### 4. Use Appropriate Function
Choose the right function for your use case:

- `search()` - Find pattern anywhere
- `match()` - Pattern must be at start  
- `validate()` - Pattern must match entire text
- `find_all()` - Extract all matches
- `find_groups()` - Extract with capture groups

## Performance Tips

### Pattern Caching
The regex module automatically caches compiled patterns for better performance:

```glang
# This pattern is compiled once and reused
email_pattern = "\\b\\w+@\\w+\\.\\w+\\b"

for text in email_list.elements {
    # Pattern is reused from cache
    if regex.search(email_pattern, text) {
        # Process email
    }
}
```

### Optimize Patterns
- Use specific quantifiers instead of `.*` when possible
- Anchor patterns with `^` and `$` for validation
- Use non-capturing groups `(?:...)` when you don't need the groups

## Error Handling

The regex module provides clear error messages for common issues:

```glang
# Invalid pattern
try {
    regex.match("[invalid", "test")  # Missing closing bracket
} catch error {
    # Error: "Invalid regex pattern '[invalid': unterminated character set"
}

# Invalid flags
try {
    regex.match("test", "test", "z")  # Invalid flag
} catch error {
    # Error: "Unknown regex flag: 'z'"
}
```

## Integration with Glang Strings

Regex complements Glang's built-in string methods:

```glang
text = "Hello World 123! Contact support@example.com"

# Use built-in methods for simple cases
if text.contains("World") {
    # Simple substring search
}

# Use regex for complex patterns
if regex.search("\\b\\w+@\\w+\\.\\w+\\b", text) {
    # Extract email with regex
    emails = regex.find_all("\\b\\w+@\\w+\\.\\w+\\b", text)
}
```

## See Also

- [String Methods](../builtins/string_methods.md) - Built-in string processing
- [IO Module](io.md) - File operations and text processing
- [Time Module](time.md) - Date/time parsing and formatting
# string - String Type

Strings in Graphoid represent text values. They are immutable sequences of Unicode characters, similar to strings in Python and JavaScript.

## String Literals

### Basic Strings

Strings are created using double quotes:

```graphoid
name = "Alice"
message = "Hello, world!"
empty = ""
```

### Escape Sequences

Strings support standard escape sequences:

| Escape | Meaning |
|--------|---------|
| `\n` | Newline |
| `\t` | Tab |
| `\r` | Carriage return |
| `\\` | Backslash |
| `\"` | Double quote |
| `\uXXXX` | Unicode character (4 hex digits) |

**Examples**:
```graphoid
multiline = "Line 1\nLine 2\nLine 3"
path = "C:\\Users\\Alice\\Documents"
quote = "She said, \"Hello!\""
emoji = "\u263A"  # ☺
```

---

## String Operators

### Concatenation (`+`)

Joins two strings together.

**Syntax**: `string1 + string2`

**Parameters**:
- `string1` (string): First string
- `string2` (string): Second string

**Returns**: (string) Concatenated string

**Examples**:
```graphoid
greeting = "Hello, " + "world!"
print(greeting)  # "Hello, world!"

# With variables
name = "Alice"
message = "Hello, " + name + "!"
print(message)  # "Hello, Alice!"

# Build strings
result = "a" + "b" + "c"  # "abc"
```

**See also**: `join()`

---

### Repetition (`*`)

Repeats a string multiple times.

**Syntax**: `string * count`

**Parameters**:
- `string` (string): String to repeat
- `count` (num): Number of repetitions

**Returns**: (string) Repeated string

**Examples**:
```graphoid
separator = "-" * 20
print(separator)  # "--------------------"

padding = " " * 10   # 10 spaces

# Create patterns
pattern = "abc" * 3  # "abcabcabc"
```

**See also**: `string.generate()`

---

### Indexing (`[]`)

Access individual characters by position (zero-indexed).

**Syntax**: `string[index]`

**Parameters**:
- `index` (num): Character position (0-based)

**Returns**: (string) Single character at position

**Examples**:
```graphoid
word = "hello"
first = word[0]    # "h"
last = word[4]     # "o"

# Negative indices count from end
last = word[-1]    # "o"
second_last = word[-2]  # "l"
```

**Errors**: Index out of bounds raises error

**See also**: `substring()`, `length()`

---

### Slicing (`[start:end]`)

Extract a substring.

**Syntax**: `string[start:end]`

**Parameters**:
- `start` (num): Starting index (inclusive)
- `end` (num): Ending index (exclusive)

**Returns**: (string) Substring from `start` to `end`

**Examples**:
```graphoid
text = "hello world"

# Basic slicing
substr = text[0:5]   # "hello"
substr = text[6:11]  # "world"

# Omit start or end
substr = text[:5]    # "hello" (from beginning)
substr = text[6:]    # "world" (to end)

# Negative indices
substr = text[-5:]   # "world" (last 5 chars)
```

**See also**: `substring()`, `[]`

---

### Comparison

Strings can be compared lexicographically using standard comparison operators.

**Operators**: `==`, `!=`, `<`, `<=`, `>`, `>=`

**Examples**:
```graphoid
# Equality
"hello" == "hello"  # true
"hello" == "world"  # false

# Ordering
"apple" < "banana"  # true
"zoo" > "aardvark"  # true

# Case-sensitive
"Hello" == "hello"  # false
"Apple" < "apple"   # true (uppercase comes first)
```

**See also**: `to_lower()`, `to_upper()`

---

## String Methods

### length()

Returns the number of characters in the string.

**Syntax**: `string.length()`

**Returns**: (num) Number of characters

**Examples**:
```graphoid
len = "hello".length()
print(len)  # 5

len = "".length()
print(len)  # 0

# Validation
if password.length() < 8 {
    print("Password too short")
}
```

**See also**: `is_empty()`

---

### substring(start, end)

Extracts a substring (alternative to slicing).

**Syntax**: `string.substring(start, end)`

**Parameters**:
- `start` (num): Starting index (inclusive)
- `end` (num, optional): Ending index (exclusive). If omitted, goes to end.

**Returns**: (string) Substring

**Examples**:
```graphoid
text = "hello world"
result = text.substring(0, 5)   # "hello"
result = text.substring(6)      # "world"
result = text.substring(6, 11)  # "world"
```

**See also**: `[:]` (slice notation)

---

### to_upper()

Converts string to uppercase.

**Syntax**: `string.to_upper()`

**Returns**: (string) Uppercase version

**Examples**:
```graphoid
result = "hello".to_upper()
print(result)  # "HELLO"

result = "Hello World".to_upper()
print(result)  # "HELLO WORLD"

# Case-insensitive comparison
if name.to_upper() == "ALICE" {
    print("Found Alice!")
}
```

**See also**: `to_lower()`

---

### to_lower()

Converts string to lowercase.

**Syntax**: `string.to_lower()`

**Returns**: (string) Lowercase version

**Examples**:
```graphoid
result = "HELLO".to_lower()
print(result)  # "hello"

result = "Hello World".to_lower()
print(result)  # "hello world"

# Case-insensitive matching
if email.to_lower().ends_with("@example.com") {
    print("Example domain")
}
```

**See also**: `to_upper()`

---

### trim()

Removes whitespace from both ends of the string.

**Syntax**: `string.trim()`

**Returns**: (string) Trimmed string

**Examples**:
```graphoid
result = "  hello  ".trim()
print(result)  # "hello"

result = "\t\nhello\n\t".trim()
print(result)  # "hello"

# Clean user input
username = input.trim()
```

**See also**: `trim_start()`, `trim_end()`

---

### trim_start()

Removes whitespace from the beginning of the string.

**Syntax**: `string.trim_start()`

**Returns**: (string) Trimmed string

**Examples**:
```graphoid
result = "  hello".trim_start()
print(result)  # "hello"

result = "  hello  ".trim_start()
print(result)  # "hello  "
```

**See also**: `trim()`, `trim_end()`

---

### trim_end()

Removes whitespace from the end of the string.

**Syntax**: `string.trim_end()`

**Returns**: (string) Trimmed string

**Examples**:
```graphoid
result = "hello  ".trim_end()
print(result)  # "hello"

result = "  hello  ".trim_end()
print(result)  # "  hello"
```

**See also**: `trim()`, `trim_start()`

---

### split(delimiter)

Splits string into a list of substrings.

**Syntax**: `string.split(delimiter)`

**Parameters**:
- `delimiter` (string): Separator string

**Returns**: (list) List of substrings

**Examples**:
```graphoid
# Split on comma
result = "a,b,c".split(",")
print(result)  # ["a", "b", "c"]

# Split on spaces
words = "hello world test".split(" ")
print(words)  # ["hello", "world", "test"]

# Parse CSV
line = "Alice,30,Boston"
fields = line.split(",")
name = fields[0]   # "Alice"
age = fields[1]    # "30"
city = fields[2]   # "Boston"
```

**See also**: `join()`

---

### join(list)

Joins list elements into a string (static method).

**Syntax**: `string.join(list, separator)`

**Parameters**:
- `list` (list): List of strings to join
- `separator` (string): Separator between elements

**Returns**: (string) Joined string

**Examples**:
```graphoid
words = ["hello", "world"]
result = string.join(words, " ")
print(result)  # "hello world"

items = ["a", "b", "c"]
result = string.join(items, ", ")
print(result)  # "a, b, c"

# Build CSV
fields = ["Alice", "30", "Boston"]
line = string.join(fields, ",")
# "Alice,30,Boston"
```

**See also**: `split()`

---

### contains(substring)

Tests if string contains a substring.

**Syntax**: `string.contains(substring)`

**Parameters**:
- `substring` (string): String to search for

**Returns**: (bool) `true` if found, `false` otherwise

**Examples**:
```graphoid
result = "hello world".contains("world")
print(result)  # true

result = "hello world".contains("foo")
print(result)  # false

# Validation
if email.contains("@") {
    print("Valid email format")
}
```

**See also**: `starts_with()`, `ends_with()`, `index_of()`

---

### starts_with(prefix)

Tests if string starts with a prefix.

**Syntax**: `string.starts_with(prefix)`

**Parameters**:
- `prefix` (string): Prefix to check

**Returns**: (bool) `true` if starts with prefix, `false` otherwise

**Examples**:
```graphoid
result = "hello world".starts_with("hello")
print(result)  # true

result = "hello world".starts_with("world")
print(result)  # false

# File extension check
if filename.starts_with("test_") {
    print("Test file")
}
```

**See also**: `ends_with()`, `contains()`

---

### ends_with(suffix)

Tests if string ends with a suffix.

**Syntax**: `string.ends_with(suffix)`

**Parameters**:
- `suffix` (string): Suffix to check

**Returns**: (bool) `true` if ends with suffix, `false` otherwise

**Examples**:
```graphoid
result = "hello world".ends_with("world")
print(result)  # true

result = "hello world".ends_with("hello")
print(result)  # false

# File extension
if filename.ends_with(".gr") {
    print("Graphoid file")
}
```

**See also**: `starts_with()`, `contains()`

---

### index_of(substring)

Finds the position of a substring.

**Syntax**: `string.index_of(substring, start)`

**Parameters**:
- `substring` (string): String to find
- `start` (num, optional): Starting position (default: 0)

**Returns**: (num) Index of first occurrence, or -1 if not found

**Examples**:
```graphoid
index = "hello world".index_of("world")
print(index)  # 6

index = "hello world".index_of("foo")
print(index)  # -1

# Find multiple occurrences
text = "the quick brown fox"
first = text.index_of("o")        # 12 (in "brown")
second = text.index_of("o", first + 1)  # 17 (in "fox")
```

**See also**: `contains()`, `last_index_of()`

---

### last_index_of(substring)

Finds the last occurrence of a substring.

**Syntax**: `string.last_index_of(substring)`

**Parameters**:
- `substring` (string): String to find

**Returns**: (num) Index of last occurrence, or -1 if not found

**Examples**:
```graphoid
index = "hello hello".last_index_of("hello")
print(index)  # 6

# Find file extension
filename = "document.backup.txt"
dot = filename.last_index_of(".")
extension = filename.substring(dot + 1)  # "txt"
```

**See also**: `index_of()`

---

### replace(old, new)

Replaces all occurrences of a substring.

**Syntax**: `string.replace(old, new, count)`

**Parameters**:
- `old` (string): Substring to replace
- `new` (string): Replacement string
- `count` (num, optional): Maximum replacements (default: all)

**Returns**: (string) String with replacements

**Examples**:
```graphoid
result = "hello world".replace("world", "Graphoid")
print(result)  # "hello Graphoid"

# Replace all
result = "foo bar foo".replace("foo", "baz")
print(result)  # "baz bar baz"

# Replace first only
result = "foo bar foo".replace("foo", "baz", 1)
print(result)  # "baz bar foo"

# Remove substring
result = "hello world".replace("world", "")
print(result)  # "hello "
```

**See also**: `split()`, `join()`

---

### reverse()

Reverses the string.

**Syntax**: `string.reverse()`

**Returns**: (string) Reversed string

**Examples**:
```graphoid
result = "hello".reverse()
print(result)  # "olleh"

# Palindrome check
fn is_palindrome(s) {
    return s == s.reverse()
}

print(is_palindrome("racecar"))  # true
```

**See also**: `list.reverse()`

---

### repeat(count)

Repeats the string (alternative to `*` operator).

**Syntax**: `string.repeat(count)`

**Parameters**:
- `count` (num): Number of repetitions

**Returns**: (string) Repeated string

**Examples**:
```graphoid
result = "abc".repeat(3)
print(result)  # "abcabcabc"

# Same as
result = "abc" * 3
```

**See also**: `*` operator, `string.generate()`

---

### to_num()

Converts string to number.

**Syntax**: `string.to_num()`

**Returns**: (num) Numeric value, or `none` if invalid

**Examples**:
```graphoid
result = "42".to_num()
print(result)  # 42

result = "3.14".to_num()
print(result)  # 3.14

result = "not a number".to_num()
print(result)  # none

# Parse user input
input = "123"
value = input.to_num()
if value != none {
    print("Valid number: " + value.to_string())
}
```

**See also**: `num.to_string()`, `is_numeric()`

---

### is_empty()

Tests if string is empty (length 0).

**Syntax**: `string.is_empty()`

**Returns**: (bool) `true` if empty, `false` otherwise

**Examples**:
```graphoid
result = "".is_empty()
print(result)  # true

result = "hello".is_empty()
print(result)  # false

# Validation
if username.trim().is_empty() {
    print("Username required")
}
```

**See also**: `length()`

---

### is_numeric()

Tests if string represents a valid number.

**Syntax**: `string.is_numeric()`

**Returns**: (bool) `true` if numeric, `false` otherwise

**Examples**:
```graphoid
"42".is_numeric()      # true
"3.14".is_numeric()    # true
"-5".is_numeric()      # true
"hello".is_numeric()   # false

# Validation
if input.is_numeric() {
    value = input.to_num()
}
```

**See also**: `to_num()`

---

### is_alpha()

Tests if string contains only alphabetic characters.

**Syntax**: `string.is_alpha()`

**Returns**: (bool) `true` if alphabetic, `false` otherwise

**Examples**:
```graphoid
"hello".is_alpha()     # true
"Hello".is_alpha()     # true
"hello123".is_alpha()  # false
"hello world".is_alpha()  # false (space)
```

**See also**: `is_alphanumeric()`, `is_numeric()`

---

### is_alphanumeric()

Tests if string contains only letters and digits.

**Syntax**: `string.is_alphanumeric()`

**Returns**: (bool) `true` if alphanumeric, `false` otherwise

**Examples**:
```graphoid
"hello123".is_alphanumeric()  # true
"hello".is_alphanumeric()     # true
"123".is_alphanumeric()       # true
"hello 123".is_alphanumeric() # false (space)
"hello!".is_alphanumeric()    # false (!)
```

**See also**: `is_alpha()`, `is_numeric()`

---

## Static Methods

Static methods are called on the `string` type itself, not on string instances.

### string.generate(str, count) - Repetition Mode

Generates a string by repeating a string multiple times.

**Syntax**: `string.generate(str, count)`

**Parameters**:
- `str` (string): String to repeat
- `count` (num): Number of repetitions

**Returns**: (string) Repeated string

**Examples**:
```graphoid
# Create padding
padding = string.generate(" ", 10)   # "          " (10 spaces)

# Create separator
line = string.generate("-", 40)      # 40 dashes

# Create pattern
pattern = string.generate("*", 5)    # "*****"

# Dynamic repetition
bar = string.generate("#", count)
```

**See also**: `*` operator, `repeat()`

---

### string.generate(from_char, to_char) - Sequence Mode

Generates a sequence of characters from one character to another.

**Syntax**: `string.generate(from_char, to_char)`

**Parameters**:
- `from_char` (string): Starting character (single char)
- `to_char` (string): Ending character (single char)

**Returns**: (string) Character sequence

**Examples**:
```graphoid
# Alphabet
lowercase = string.generate("a", "z")
# "abcdefghijklmnopqrstuvwxyz"

uppercase = string.generate("A", "Z")
# "ABCDEFGHIJKLMNOPQRSTUVWXYZ"

# Digits
digits = string.generate("0", "9")
# "0123456789"

# Subset
vowels_range = string.generate("a", "e")
# "abcde"

# Use in validation
fn is_lowercase_letter(ch) {
    alphabet = string.generate("a", "z")
    return alphabet.contains(ch)
}
```

**See also**: `list.generate()`

---

## Type Checking

### is_string()

Tests if a value is a string.

**Syntax**: `value.is_string()`

**Returns**: (bool) `true` if string, `false` otherwise

**Examples**:
```graphoid
result = "hello".is_string()  # true
result = 42.is_string()       # false

# Type validation
if not value.is_string() {
    print("Expected string")
}
```

**See also**: `is_number()`, `is_list()`

---

## String Formatting

### Template Interpolation

Graphoid supports string template interpolation using `${}`:

```graphoid
name = "Alice"
age = 30

# Template interpolation
message = "Hello, ${name}! You are ${age} years old."
print(message)  # "Hello, Alice! You are 30 years old."

# Expressions in templates
result = "Sum: ${5 + 3}"
print(result)  # "Sum: 8"

# Method calls
text = "value"
result = "Uppercase: ${text.to_upper()}"
print(result)  # "Uppercase: VALUE"
```

---

## Unicode Support

Graphoid strings are UTF-8 encoded and support Unicode:

```graphoid
# Unicode characters
emoji = "Hello 👋 World 🌍"
chinese = "你好世界"
arabic = "مرحبا بالعالم"

# Unicode escape sequences
heart = "\u2665"    # ♥
smiley = "\u263A"   # ☺

# Length counts Unicode characters (not bytes)
emoji_str = "👋🌍"
print(emoji_str.length())  # 2 (not 8 bytes)
```

---

## Common Patterns

### Case-Insensitive Comparison

```graphoid
fn equals_ignore_case(s1, s2) {
    return s1.to_lower() == s2.to_lower()
}

result = equals_ignore_case("Hello", "HELLO")  # true
```

### Centering Text

```graphoid
fn center(text, width) {
    padding = (width - text.length()) / 2
    left_pad = string.generate(" ", padding)
    right_pad = string.generate(" ", width - text.length() - padding)
    return left_pad + text + right_pad
}

result = center("Title", 20)
```

### Word Count

```graphoid
fn word_count(text) {
    words = text.trim().split(" ")
    return words.length()
}

count = word_count("hello world test")  # 3
```

### Title Case

```graphoid
fn to_title_case(text) {
    words = text.split(" ")
    capitalized = words.map(w => {
        if w.length() == 0 { return w }
        return w[0].to_upper() + w.substring(1).to_lower()
    })
    return string.join(capitalized, " ")
}

result = to_title_case("hello world")  # "Hello World"
```

### Sanitize Input

```graphoid
fn sanitize(input) {
    return input
        .trim()
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("&", "&amp;")
}

safe = sanitize("  <script>alert('xss')</script>  ")
```

---

## See Also

- [regex](../stdlib/regex.md) - Pattern matching with regular expressions
- [string (stdlib)](../stdlib/string.md) - Additional string utilities
- [list](list.md) - List type (strings are similar)
- [operators](../operators.md) - Complete operator reference

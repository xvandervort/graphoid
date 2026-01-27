# string - String Type

Strings in Graphoid represent text values. They are immutable sequences of Unicode characters, similar to strings in Python and JavaScript.

---

## Strings as Graphs of Characters

In Graphoid, **everything is a graph**. Strings are no exception - they behave as graphs of characters, responding to many of the same methods as lists. This unified design means you can:

- **Iterate** over characters just like iterating over list elements
- **Use functional methods** like `map()`, `filter()`, and `reject()` on strings
- **Access elements** with `first()`, `last()`, and `slice()`

This approach follows Graphoid's core philosophy: data structures should be consistent and composable. If you know how to work with lists, you already know how to work with strings at the character level.

### Iteration

Strings can be iterated directly in `for` loops, yielding each character as a single-character string:

```graphoid
for c in "hello" {
    print(c)  # Prints: h, e, l, l, o (one per line)
}

# Collect characters into a list
chars = []
for c in "abc" {
    chars = chars.insert(chars.length(), c)
}
print(chars)  # ["a", "b", "c"]

# Works with Unicode
for c in "café" {
    print(c)  # Prints: c, a, f, é
}

# Empty strings iterate zero times (no error)
for c in "" {
    print("never printed")
}
```

Each character yielded during iteration is a **single-character string**, not a special character type. This means you can call any string method on individual characters:

```graphoid
for c in "hello" {
    print(c.upper())  # Prints: H, E, L, L, O
}
```

---

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

**See also**: `is_empty()`, `first()`, `last()`

---

### first()

Returns the first character of the string.

**Syntax**: `string.first()`

**Returns**: (string) First character, or `none` if empty

**Examples**:
```graphoid
result = "hello".first()
print(result)  # "h"

result = "".first()
print(result)  # none

# Works with Unicode
result = "émoji".first()
print(result)  # "é"

# Safe access pattern
ch = text.first()
if ch != none {
    print("First character: " + ch)
}
```

**See also**: `last()`, `[]` (indexing)

---

### last()

Returns the last character of the string.

**Syntax**: `string.last()`

**Returns**: (string) Last character, or `none` if empty

**Examples**:
```graphoid
result = "hello".last()
print(result)  # "o"

result = "".last()
print(result)  # none

# Works with Unicode
result = "café".last()
print(result)  # "é"

# Check file extension
if filename.last() == "/" {
    print("Directory path")
}
```

**See also**: `first()`, `[]` (indexing)

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

**See also**: `[:]` (slice notation), `slice()`

---

### slice(start, end)

Extracts a substring by character indices. This method mirrors the `slice()` method available on lists, providing consistent behavior across collection types.

**Syntax**: `string.slice(start, end)`

**Parameters**:
- `start` (num): Starting index (inclusive, 0-based)
- `end` (num): Ending index (exclusive)

**Returns**: (string) Substring from `start` to `end`

**Examples**:
```graphoid
result = "hello".slice(1, 4)
print(result)  # "ell"

# Start at beginning
result = "hello".slice(0, 2)
print(result)  # "he"

# Go to end
result = "hello".slice(3, 5)
print(result)  # "lo"

# Equal indices = empty string
result = "hello".slice(2, 2)
print(result)  # ""

# Reversed indices = empty string (safe)
result = "hello".slice(4, 2)
print(result)  # ""

# Out-of-bounds indices are clamped
result = "hello".slice(0, 100)
print(result)  # "hello"

# Works with Unicode
result = "café".slice(1, 3)
print(result)  # "af"
```

**Consistency with lists**: This method behaves identically to `list.slice()`, allowing the same code patterns to work on both strings and lists.

**See also**: `substring()`, `[:]` (slice notation), `list.slice()`

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

## Functional Methods

Strings support functional programming methods that mirror those available on lists. This is part of Graphoid's "strings as graphs of characters" philosophy - strings respond to the same operations as other collections.

### map(function)

Transforms each character using a function, returning a **list** of results.

**Syntax**: `string.map(function)`

**Parameters**:
- `function` (lambda): Function taking a character and returning a value

**Returns**: (list) List of transformed values

**Examples**:
```graphoid
# Transform to uppercase
result = "abc".map(c => c.upper())
print(result)  # ["A", "B", "C"]

# Convert to character codes (via to_num on digits)
result = "123".map(c => c.to_num())
print(result)  # [1, 2, 3]

# Each character's length (always 1)
result = "hello".map(c => c.length())
print(result)  # [1, 1, 1, 1, 1]

# Empty string returns empty list
result = "".map(c => c.upper())
print(result)  # []

# Join results back to string if needed
chars = "hello".map(c => c.upper())
result = string.join(chars, "")  # "HELLO"
```

**Note**: Unlike `filter()` and `reject()`, `map()` returns a **list** because transformations may produce non-string values.

**See also**: `filter()`, `reject()`, `list.map()`

---

### filter(predicate)

Keeps characters matching a predicate, returning a **string**.

**Syntax**: `string.filter(predicate)`

**Parameters**:
- `predicate` (lambda): Function taking a character and returning bool

**Returns**: (string) String containing only matching characters

**Examples**:
```graphoid
# Keep only vowels
vowels = "hello".filter(c => c == "e" or c == "o")
print(vowels)  # "eo"

# Keep only "l" characters
result = "hello".filter(c => c == "l")
print(result)  # "ll"

# Result is a string, not a list
print(result.type())  # "string"

# Returns empty string when nothing matches
result = "hello".filter(c => c == "z")
print(result)  # ""

# Returns same string when all match
result = "aaa".filter(c => c == "a")
print(result)  # "aaa"

# Remove spaces
result = "a B c".filter(c => c != " ")
print(result)  # "aBc"

# Empty string returns empty string
result = "".filter(c => true)
print(result)  # ""
```

**See also**: `reject()`, `map()`, `list.filter()`

---

### reject(predicate)

Removes characters matching a predicate, returning a **string**. This is the inverse of `filter()`.

**Syntax**: `string.reject(predicate)`

**Parameters**:
- `predicate` (lambda): Function taking a character and returning bool

**Returns**: (string) String with matching characters removed

**Examples**:
```graphoid
# Remove vowels (keep consonants)
consonants = "hello".reject(c => c == "e" or c == "o")
print(consonants)  # "hll"

# Remove "l" characters
result = "hello".reject(c => c == "l")
print(result)  # "heo"

# Result is a string, not a list
print(result.type())  # "string"

# Returns same string when nothing matches
result = "hello".reject(c => c == "z")
print(result)  # "hello"

# Returns empty string when all match
result = "aaa".reject(c => c == "a")
print(result)  # ""

# filter and reject are inverses
s = "hello"
filtered = s.filter(c => c == "l")
rejected = s.reject(c => c == "l")
print(filtered.length() + rejected.length())  # 5 (equals s.length())
```

**See also**: `filter()`, `map()`, `list.reject()`

---

### each(function)

Calls a function for each character. Returns `none`.

**Syntax**: `string.each(function)`

**Parameters**:
- `function` (lambda): Function taking a character

**Returns**: (none) Always returns `none`

**Examples**:
```graphoid
# Process each character
"hello".each(c => print(c))
# Prints: h, e, l, l, o (one per line)

# each() returns none
result = "abc".each(c => c)
print(result)  # none

# Empty string calls function zero times
"".each(c => print("never called"))

# Use map() instead if you need results
result = "abc".map(c => c.upper())  # ["A", "B", "C"]
```

**Note**: Use `each()` for side effects. If you need to collect results, use `map()` instead.

**See also**: `map()`, `list.each()`

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

### String/List Consistency

Because strings behave as graphs of characters, you can use the same patterns for both:

```graphoid
# Both strings and lists support iteration
for c in "hello" { print(c) }
for item in ["h", "e", "l", "l", "o"] { print(item) }

# Both support first()/last()
"hello".first()                    # "h"
["h", "e", "l", "l", "o"].first()  # "h"

# Both support map() (returns list)
"abc".map(c => c.upper())              # ["A", "B", "C"]
["a", "b", "c"].map(c => c.upper())    # ["A", "B", "C"]

# filter() returns same type
"hello".filter(c => c == "l")          # "ll" (string)
["h", "e", "l", "l", "o"].filter(c => c == "l")  # ["l", "l"] (list)

# Write generic functions that work on both
fn count_matches(collection, pred) {
    matches = 0
    for item in collection {
        if pred(item) { matches = matches + 1 }
    }
    return matches
}

count_matches("hello", c => c == "l")           # 2
count_matches(["h", "e", "l", "l", "o"], c => c == "l")  # 2
```

---

## See Also

- [list](list.md) - List type (strings share many methods with lists as both are graphs)
- [regex](../stdlib/regex.md) - Pattern matching with regular expressions
- [string (stdlib)](../stdlib/string.md) - Additional string utilities
- [operators](../operators.md) - Complete operator reference

### Related Concepts

- **Everything is a Graph**: Strings are graphs of characters, just as lists are graphs of elements. This is Graphoid's foundational principle.
- **Functional Methods**: `map()`, `filter()`, `reject()`, `each()` work consistently across all collection types.

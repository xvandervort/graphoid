# String Methods

String is a built-in type in Glang representing text data. Strings support various methods for manipulation, analysis, and transformation.

## Type Information

### type()
Returns the type of the value.
```glang
text = "hello"
text.type()  # Returns "string"
```

### methods()
Returns a list of all available methods for strings.
```glang
text = "hello"
text.methods()  # Returns ["type", "methods", "can", "inspect", "size", "upper", "lower", ...]
```

### can(method_name)
Checks if a method is available on the string.
```glang
text = "hello"
text.can("upper")  # Returns true
text.can("invalid")  # Returns false
```

### inspect()
Returns detailed information about the string.
```glang
text = "hello"
text.inspect()  # Returns detailed string information
```

## Size and Length

### size()
Returns the number of characters in the string.
```glang
text = "hello"
text.size()  # Returns 5
```

### length()
Alias for size(). Returns the number of characters.
```glang
text = "hello world"
text.length()  # Returns 11
```

### empty()
Checks if the string is empty.
```glang
"".empty()  # Returns true
"hello".empty()  # Returns false
```

## Case Conversion

### upper()
Converts the string to uppercase.
```glang
text = "hello"
text.upper()  # Returns "HELLO"
```

### up()
Alias for upper(). Converts to uppercase.
```glang
text = "hello"
text.up()  # Returns "HELLO"
```

### toUpper()
Another alias for upper().
```glang
text = "hello"
text.toUpper()  # Returns "HELLO"
```

### lower()
Converts the string to lowercase.
```glang
text = "HELLO"
text.lower()  # Returns "hello"
```

### down()
Alias for lower(). Converts to lowercase.
```glang
text = "HELLO"
text.down()  # Returns "hello"
```

### toLower()
Another alias for lower().
```glang
text = "HELLO"
text.toLower()  # Returns "hello"
```

## String Manipulation

### trim()
Removes leading and trailing whitespace.
```glang
text = "  hello  "
text.trim()  # Returns "hello"
```

### reverse()
Reverses the string.
```glang
text = "hello"
text.reverse()  # Returns "olleh"
```

### replace(old, new)
Replaces all occurrences of a substring with another.
```glang
text = "hello world"
text.replace("world", "glang")  # Returns "hello glang"
```

## String Analysis

### contains(substring)
Checks if the string contains a substring.
```glang
text = "hello world"
text.contains("world")  # Returns true
text.contains("xyz")  # Returns false
```

### matches(pattern)
Checks if the string matches a regular expression pattern.
```glang
email = "user@example.com"
email.matches(".*@.*\\..*")  # Returns true
```

### findAll(pattern)
Finds all matches of a regular expression pattern.
```glang
text = "cat and dog and cat"
text.findAll("cat")  # Returns ["cat", "cat"]
```

## String Splitting and Joining

### split(delimiter)
Splits the string by a delimiter. If no delimiter is provided, splits by whitespace.
```glang
text = "hello world test"
text.split()  # Returns ["hello", "world", "test"]

csv = "apple,banana,cherry"
csv.split(",")  # Returns ["apple", "banana", "cherry"]

lines = "line1\nline2\nline3"
lines.split("\n")  # Returns ["line1", "line2", "line3"]
```

### chars()
Splits the string into individual characters.
```glang
text = "hello"
text.chars()  # Returns ["h", "e", "l", "l", "o"]
```

### join(list)
Joins a list of strings using the current string as a separator.
```glang
separator = ", "
separator.join(["apple", "banana", "cherry"])  # Returns "apple, banana, cherry"
```

### unique()
Returns unique characters in the string.
```glang
text = "hello"
text.unique()  # Returns "helo" (unique characters)
```

## Type Conversion

### to_string()
Returns the string itself (identity operation for strings).
```glang
text = "hello"
text.to_string()  # Returns "hello"
```

### to_num()
Converts the string to a number if possible.
```glang
"42".to_num()  # Returns 42
"3.14".to_num()  # Returns 3.14
"invalid".to_num()  # Throws error
```

### to_bool()
Converts the string to a boolean. Empty strings are false, non-empty are true.
```glang
"hello".to_bool()  # Returns true
"".to_bool()  # Returns false
```

## Immutability Methods

### freeze()
Makes the string immutable. Returns self for chaining.
```glang
text = "hello"
text.freeze()
text.is_frozen()  # Returns true
```

### is_frozen()
Checks if the string is frozen (immutable).
```glang
text = "hello"
text.freeze()
text.is_frozen()  # Returns true
```

### contains_frozen()
For strings, returns the same as is_frozen() since strings are atomic values.
```glang
text = "hello"
text.freeze()
text.contains_frozen()  # Returns true
```

## String Indexing and Slicing

Strings support indexing and slicing operations:

### Index Access
```glang
text = "hello"
text[0]  # Returns "h"
text[1]  # Returns "e"
text[-1]  # Returns "o" (last character)
```

### Slice Access
```glang
text = "hello world"
text[0:5]  # Returns "hello"
text[6:]  # Returns "world"
text[::2]  # Returns "hlowr" (every 2nd character)
```

## Examples

### Email Validation
```glang
email = "user@example.com"
if email.contains("@") and email.split("@").size() == 2 {
    print("Valid email format")
}
```

### Text Processing
```glang
# Process user input
input = "  Hello, WORLD!  "
cleaned = input.trim().lower()
print(cleaned)  # "hello, world!"

# Extract words
words = cleaned.split()
for word in words {
    print(word.upper())
}
```

### CSV Parsing
```glang
csv_line = "John,25,Engineer"
fields = csv_line.split(",")
name = fields[0]
age = fields[1].to_num()
job = fields[2]
print(name + " is " + age.to_string() + " years old")
```

### String Building
```glang
parts = ["apple", "banana", "cherry"]
result = ", ".join(parts)
print("Fruits: " + result)  # "Fruits: apple, banana, cherry"
```
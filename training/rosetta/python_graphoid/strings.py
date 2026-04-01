# Rosetta Stone: String Operations — Python
# Demonstrates: case, whitespace, splitting, patterns, formatting, slicing

# --- Case conversion ---
greeting = "Hello, World!"
print(greeting.upper())        # "HELLO, WORLD!"
print(greeting.lower())        # "hello, world!"

# --- Whitespace handling ---
padded = "   extra spaces   "
print(repr(padded.strip()))    # "extra spaces"
print(repr(padded.lstrip()))   # "extra spaces   "
print(repr(padded.rstrip()))   # "   extra spaces"

# --- Length ---
msg = "Graphoid"
print(f"Length: {len(msg)}")   # 8

# --- Splitting and joining ---
csv_line = "apple,banana,cherry,date"
fruits = csv_line.split(",")
print(fruits)                  # ['apple', 'banana', 'cherry', 'date']

rejoined = " | ".join(fruits)
print(rejoined)                # "apple | banana | cherry | date"

# --- Starts with / ends with ---
filename = "report_2026.pdf"
print(filename.startswith("report"))  # True
print(filename.endswith(".pdf"))      # True

# --- Search and contains ---
sentence = "The quick brown fox jumps over the lazy dog"
print("fox" in sentence)              # True
print(sentence.find("fox"))           # 16
print(sentence.count("the"))          # 1 (case-sensitive)

# --- Replace ---
original = "I like cats and cats like me"
replaced = original.replace("cats", "dogs")
print(replaced)  # "I like dogs and dogs like me"

# --- Slicing / substring ---
text = "Hello, Graphoid!"
print(text[0:5])     # "Hello"
print(text[7:15])    # "Graphoid"

# --- f-string formatting ---
name = "Alice"
age = 30
print(f"Name: {name}, Age: {age}")

# --- Repetition ---
separator = "-" * 40
print(separator)  # "----------------------------------------"
padding = " " * 10
print(f"|{padding}centered{padding}|")

# --- String conversion ---
num = 42
as_string = str(num)
print("The answer is " + as_string)

# --- Checking string content ---
print("12345".isdigit())      # True
print("hello".isalpha())      # True

# --- Summary ---
# Python string ops:
#   .upper(), .lower()
#   .strip(), .lstrip(), .rstrip()
#   .split(sep), sep.join(list)
#   .startswith(), .endswith()
#   .find(), .count(), "in" operator
#   .replace(old, new)
#   slicing: s[start:end]
#   f"formatted {expr}"
#   "char" * n for repetition
#   str(value) for conversion

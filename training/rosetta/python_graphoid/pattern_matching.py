"""Pattern Matching — Python equivalent of pattern_matching.gr

Python 3.10+ has structural pattern matching (match/case).
Earlier versions use if/elif chains.
"""

# --- Basic match on values ---
def classify(x):
    match x:
        case 0:
            return "zero"
        case 1:
            return "one"
        case 2:
            return "two"
        case _:
            return "other"

print(classify(0))   # "zero"
print(classify(99))  # "other"

# --- Match on strings ---
def respond(cmd):
    match cmd:
        case "start":
            return "Starting..."
        case "stop":
            return "Stopping..."
        case _:
            return f"Unknown: {cmd}"

print(respond("start"))  # "Starting..."
print(respond("quit"))   # "Unknown: quit"

# --- List destructuring ---
def describe_list(lst):
    match lst:
        case []:
            return "empty"
        case [x]:
            return f"single: {x}"
        case [x, y]:
            return f"pair: {x}, {y}"
        case [first, *rest]:
            return f"first={first} rest={rest}"

print(describe_list([]))         # "empty"
print(describe_list([42]))       # "single: 42"
print(describe_list([1, 2]))     # "pair: 1, 2"
print(describe_list([1, 2, 3]))  # "first=1 rest=[2, 3]"

# --- Guards (Python uses if after pattern) ---
def classify_number(n):
    match n:
        case 0:
            return "zero"
        case n if n > 0:
            return "positive"
        case n if n < 0:
            return "negative"

print(classify_number(5))   # "positive"
print(classify_number(-3))  # "negative"
print(classify_number(0))   # "zero"

# --- Recursive with pattern matching ---
def sum_list(lst):
    match lst:
        case []:
            return 0
        case [first, *rest]:
            return first + sum_list(rest)

print(sum_list([1, 2, 3, 4, 5]))  # 15

# --- Factorial with pattern matching ---
def factorial(n):
    match n:
        case 0:
            return 1
        case n if n > 0:
            return n * factorial(n - 1)

print(factorial(5))  # 120

# --- Pattern matching on tagged tuples ---
def process(data):
    match data:
        case ["error", msg]:
            return f"Error: {msg}"
        case ["ok", value]:
            return f"Got: {value}"
        case _:
            return "Unknown format"

print(process(["ok", 42]))         # "Got: 42"
print(process(["error", "timeout"]))  # "Error: timeout"

print("Pattern matching demo complete!")

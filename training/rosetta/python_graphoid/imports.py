# Rosetta Stone: Module Imports — Python
# Demonstrates: importing, selective imports, aliases, privacy conventions

# --- Full module import ---
import math
import json
import os

# Using fully qualified names
radius = 5.0
area = math.pi * radius ** 2
print(f"Circle area: {area:.2f}")

circumference = 2 * math.pi * radius
print(f"Circumference: {circumference:.2f}")

# --- Selective imports ---
from math import sqrt, ceil, floor

diagonal = sqrt(3**2 + 4**2)
print(f"Diagonal: {diagonal}")
print(f"Ceil of 2.3: {ceil(2.3)}")
print(f"Floor of 2.7: {floor(2.7)}")

# --- Module alias ---
import statistics as stats

scores = [88, 92, 75, 95, 80]
print(f"Mean score: {stats.mean(scores)}")
print(f"Median score: {stats.median(scores)}")

# --- JSON usage ---
data = {"name": "Alice", "age": 30, "languages": ["Python", "Graphoid"]}
json_str = json.dumps(data, indent=2)
print(f"JSON:\n{json_str}")

parsed = json.loads(json_str)
print(f"Name from JSON: {parsed['name']}")

# --- Privacy convention (underscore prefix) ---
def _internal_helper(x):
    """By convention, single underscore means 'private'."""
    return x * 2

def public_api(value):
    """Public function that uses the private helper."""
    result = _internal_helper(value)
    return result + 1

print(f"Public API result: {public_api(10)}")

# --- Conditional import / import error handling ---
try:
    import nonexistent_module
except ImportError:
    print("Module not found — using fallback")

# --- Using os module ---
cwd = os.getcwd()
print(f"Current directory: {cwd}")

# --- Summary ---
# Python imports:
#   import module              — full module
#   from module import name    — selective
#   import module as alias     — alias
#   _prefix convention         — privacy (not enforced)
#   try/except ImportError     — conditional import

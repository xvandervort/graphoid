"""Collection Operations — Python equivalent of collections.gr

Compares Python list comprehensions and built-in functions
with Graphoid's functional methods and element-wise operators.
"""

# --- List creation and basic ops ---
numbers = [1, 2, 3, 4, 5]
print(f"First: {numbers[0]}")
print(f"Last: {numbers[-1]}")
print(f"Length: {len(numbers)}")
print(f"Contains 3? {3 in numbers}")

# --- Functional operations ---
# Python uses list comprehensions or map/filter
doubled = [x * 2 for x in numbers]
print(f"Doubled: {doubled}")

evens = [x for x in numbers if x % 2 == 0]
print(f"Evens: {evens}")

odds = [x for x in numbers if x % 2 != 0]
print(f"Odds: {odds}")

# Reduce requires functools
from functools import reduce
total = reduce(lambda acc, x: acc + x, numbers, 0)
print(f"Sum: {total}")

# Chaining requires intermediate variables or nested comprehensions
result = [x * 10 for x in numbers if x > 2]
print(f"Filter>2 then *10: {result}")

# --- Immutability ---
# Python lists are mutable by default
items = [3, 1, 4]
sorted_items = sorted(items)          # returns new list
items.sort()                          # mutates in place (no ! needed)
print(f"Sorted: {sorted_items}")

reversed_items = list(reversed(items))
print(f"Reversed: {reversed_items}")

unique = list(dict.fromkeys([3, 1, 4, 1, 5]))  # preserve order
print(f"Unique: {unique}")

# --- Element-wise operations ---
# Python: requires numpy or manual comprehension
a = [1, 2, 3]
# No built-in element-wise ops — need comprehension
scaled = [x * 3 for x in a]
print(f"a * 3: {scaled}")

added = [x + 10 for x in a]
print(f"a + 10: {added}")

b = [4, 5, 6]
zipped = [x + y for x, y in zip(a, b)]
print(f"a + b: {zipped}")

# Celsius to Fahrenheit
temps_c = [0, 10, 20, 30]
temps_f = [c * 1.8 + 32 for c in temps_c]
print(f"Fahrenheit: {temps_f}")

# --- Maps/Dicts ---
person = {"name": "Alice", "age": 30, "city": "Portland"}
print(f"Name: {person['name']}")
print(f"Has age? {'age' in person}")
print(f"Keys: {list(person.keys())}")

# Remove key (returns new dict in Python 3.9+ with | operator)
without_city = {k: v for k, v in person.items() if k != "city"}
print(f"Without city: {without_city}")

# --- Sorting, dedup, compact ---
data = [3, 1, None, 4, 1, None, 5]
cleaned = [x for x in data if x is not None]
print(f"Compact: {cleaned}")

deduped = list(dict.fromkeys(cleaned))
print(f"Unique: {deduped}")

final = sorted(deduped)
print(f"Sorted unique: {final}")

print("Collections demo complete!")

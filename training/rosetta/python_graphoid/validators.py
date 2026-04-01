"""Data Validation — Python equivalent of validators.gr

Python has no built-in behavior rules for collections.
Validation requires manual code or external libraries like pydantic.
"""

# --- Clamping values to a range ---
def clamp_list(values, lo, hi):
    return [max(lo, min(hi, v)) for v in values]

scores = [50, 110, 95, 130, 85, 70]
result = clamp_list(scores, 60, 100)
print(f"Clamped: {result}")  # [60, 100, 95, 100, 85, 70]

# Must call clamp_list after every modification!
scores.append(200)
scores = clamp_list(scores, 60, 100)  # easy to forget

# --- Replacing None values ---
def replace_none(values, default):
    return [default if v is None else v for v in values]

data = [1, None, 3, None, 5]
clean = replace_none(data, 0)
print(f"Cleaned: {clean}")  # [1, 0, 3, 0, 5]

# --- Preventing duplicates ---
def unique_list(values):
    seen = set()
    result = []
    for v in values:
        if v not in seen:
            seen.add(v)
            result.append(v)
    return result

items = [1, 2, 3, 2, 4, 3, 5]
unique = unique_list(items)
print(f"Unique: {unique}")  # [1, 2, 3, 4, 5]

# --- Force uppercase ---
names = ["alice", "bob", "carol"]
upper_names = [n.upper() for n in names]
print(f"Upper: {upper_names}")  # ["ALICE", "BOB", "CAROL"]

# --- Chaining operations ---
data = [1, None, -3, None, 5.7]
# Python: multiple manual passes
step1 = replace_none(data, 0)
step2 = [abs(v) if isinstance(v, (int, float)) else v for v in step1]
step3 = [round(v) if isinstance(v, (int, float)) else v for v in step2]
print(f"Chained: {step3}")  # [1, 0, 3, 0, 6]

print("Validation demo complete!")

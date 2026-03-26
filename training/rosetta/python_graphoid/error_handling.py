"""Error Handling — Python equivalent of error_handling.gr

Python uses try/except with exception classes.
"""

# --- Basic try/except ---
try:
    result = 10 / 0
except ZeroDivisionError as e:
    print(f"Error: {e}")

# --- Multiple except clauses ---
def risky(input_val):
    if input_val == "type":
        raise TypeError("wrong type")
    if input_val == "value":
        raise ValueError("bad value")
    if input_val == "io":
        raise IOError("disk full")
    return "ok"

try:
    risky("value")
except TypeError as e:
    print(f"Type: {e}")
except ValueError as e:
    print(f"Value: {e}")
except IOError as e:
    print(f"IO: {e}")
except Exception as e:
    print(f"Other: {e}")

# --- try/finally ---
def process_file(path):
    handle = None
    try:
        handle = open(path, "r")
        return handle.read()
    except Exception as e:
        print(f"Error: {e}")
        return None
    finally:
        if handle is not None:
            handle.close()

# --- Custom exceptions ---
class ValidationError(Exception):
    pass

def validate_age(age):
    if age < 0:
        raise ValidationError("Age cannot be negative")
    if age > 150:
        raise ValidationError("Age unrealistically high")
    return True

try:
    validate_age(-5)
except ValidationError as e:
    print(f"Validation failed: {e}")

# --- Re-raising ---
try:
    raise ValueError("original")
except ValueError as e:
    print(f"Caught: {e}")
    raise  # re-raise

# Won't reach here

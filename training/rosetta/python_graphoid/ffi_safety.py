"""FFI Safety — Python equivalent of ffi_safety.gr

Python's ctypes doesn't have built-in taint tracking,
so we simulate the concept with wrapper classes.
"""

import ctypes
import math

# --- Load library and call function ---
libm = ctypes.CDLL("libm.so.6")
libm.sqrt.restype = ctypes.c_double
libm.sqrt.argtypes = [ctypes.c_double]

result = libm.sqrt(16.0)
print(f"sqrt(16) = {result}")

# Python has no taint tracking — you'd need a wrapper
print(f"Is tainted? N/A (Python has no taint tracking)")

# --- Taint propagation ---
derived = result + 10
print(f"result + 10 = {derived}")
print(f"Derived is tainted? N/A")

# --- No equivalent to ffi.trust() in Python ---
# Python trusts all foreign data implicitly

# --- No bridge nodes in Python ---
# ctypes doesn't track loaded libraries in a queryable graph

# --- No resource limits in Python ctypes ---
# You'd need OS-level limits (ulimit) or custom wrappers

print("FFI safety demo complete!")

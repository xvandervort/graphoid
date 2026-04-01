# Rosetta Stone: FFI / C Interop — Python (ctypes)

import ctypes
import sys

# =============================================================
# 1. Loading a shared library
# =============================================================

libm = ctypes.CDLL("libm.so.6")

# =============================================================
# 2. Declaring and calling functions
# =============================================================

libm.sqrt.argtypes = [ctypes.c_double]
libm.sqrt.restype = ctypes.c_double

result = libm.sqrt(16.0)
print(f"sqrt(16.0) = {result}")  # 4.0

libm.pow.argtypes = [ctypes.c_double, ctypes.c_double]
libm.pow.restype = ctypes.c_double

result = libm.pow(2.0, 10.0)
print(f"pow(2.0, 10.0) = {result}")  # 1024.0

libm.ceil.argtypes = [ctypes.c_double]
libm.ceil.restype = ctypes.c_double
print(f"ceil(3.2) = {libm.ceil(3.2)}")  # 4.0

# =============================================================
# 3. Struct definition and usage
# =============================================================

class Point(ctypes.Structure):
    _fields_ = [
        ("x", ctypes.c_double),
        ("y", ctypes.c_double),
    ]

p = Point(3.0, 4.0)
print(f"Point: ({p.x}, {p.y})")

dist = libm.sqrt(p.x * p.x + p.y * p.y)
print(f"Distance: {dist}")  # 5.0

p.x = 10.0
print(f"Modified x: {p.x}")

# =============================================================
# 4. Memory allocation
# =============================================================

buf = ctypes.create_string_buffer(256)
print(f"Buffer size: {ctypes.sizeof(buf)}")

buf.value = b"Hello from Python"
print(f"Buffer: {buf.value.decode()}")

# =============================================================
# 5. Callbacks
# =============================================================

CMPFUNC = ctypes.CFUNCTYPE(ctypes.c_int, ctypes.POINTER(ctypes.c_int),
                           ctypes.POINTER(ctypes.c_int))

def py_compare(a, b):
    return a[0] - b[0]

c_compare = CMPFUNC(py_compare)

# =============================================================
# 6. Platform info
# =============================================================

print(f"Platform: {sys.platform}")
print(f"Pointer size: {ctypes.sizeof(ctypes.c_void_p)} bytes")

# =============================================================
# 7. No safety features
# =============================================================

# Python ctypes has NO taint tracking or safety checks.
# All validation is the programmer's responsibility.
print("Note: ctypes has no taint tracking or safety features.")

# Summary:
# ctypes: CDLL(), .argtypes/.restype, Structure, create_string_buffer
# CFUNCTYPE for callbacks, sizeof for sizes
# No safety: no taint, no use-after-free detection

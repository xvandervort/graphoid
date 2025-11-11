# Phase 11: Native Standard Library Modules - Detailed Implementation Plan

**Duration**: 16-23 days
**Status**: Not started
**Goal**: Implement performance-critical standard library modules in Rust

---

## Overview

Phase 11 implements standard library modules in native Rust for performance, system integration, and access to complex algorithms. These modules require system calls, cryptographic primitives, or performance-critical implementations.

**Why Native Rust**:
- Performance (parsing, hashing, crypto)
- System calls (file I/O, process management)
- External library integration (regex engines, crypto)
- Safety guarantees (memory safety, thread safety)

**All Prerequisites Complete**:
- ✅ Module system (Phase 10) - can load native modules
- ✅ FFI/native module binding mechanism
- ✅ Error handling (try/catch)

---

## Module List

**Total Modules**: 10 (9 originally planned + 1 moved from Phase 10)

### 1. Constants Module

**File**: `src/stdlib/constants.rs`
**Duration**: 1 day
**From Language Spec**: §1560-1579

**API**:
```graphoid
import "constants"

# Mathematical constants
pi = constants.pi              # 3.141592653589793
e = constants.e                # 2.718281828459045
tau = constants.tau            # 6.283185307179586 (2π)
phi = constants.phi            # 1.618033988749895 (golden ratio)
sqrt2 = constants.sqrt2        # √2
sqrt3 = constants.sqrt3        # √3

# Angle conversions
deg_to_rad = constants.deg_to_rad  # π/180
rad_to_deg = constants.rad_to_deg  # 180/π

# Physical constants
c = constants.c                # 299792458 m/s (speed of light)
G = constants.G                # 6.67430e-11 (gravitational constant)
h = constants.h                # 6.62607015e-34 (Planck constant)
```

**Implementation**:
```rust
pub struct ConstantsModule;

impl NativeModule for ConstantsModule {
    fn name(&self) -> &str { "constants" }

    fn get_constant(&self, name: &str) -> Option<Value> {
        match name {
            "pi" => Some(Value::Number(std::f64::consts::PI)),
            "e" => Some(Value::Number(std::f64::consts::E)),
            "tau" => Some(Value::Number(std::f64::consts::TAU)),
            "phi" => Some(Value::Number(1.618033988749895)),
            // ... other constants
            _ => None,
        }
    }
}
```

**Tests**: 15+ tests verifying all constants

---

### 2. Random Module (rand)

**File**: `src/stdlib/random.rs`
**Duration**: 2 days
**From Language Spec**: §1613-1641

**API**:
```graphoid
import "random"  # alias: rand

# Cryptographically secure
rand.random()                    # Float [0.0, 1.0)
rand.randint(min, max)          # Integer [min, max] inclusive
rand.uniform(min, max)          # Float [min, max)
rand.normal(mean, std_dev)      # Normal distribution
rand.exponential(lambda)        # Exponential distribution

# Deterministic (seeded)
rand.seed(42)                   # Seed RNG
rand.det_random()               # Deterministic random
rand.det_randint(min, max)      # Deterministic integer

# UUID generation
rand.uuid4()                    # Random UUID v4
rand.uuid1()                    # Time-based UUID v1

# Secure tokens
rand.token(length)              # Hex token (length bytes)
rand.token_urlsafe(length)      # URL-safe token
```

**Dependencies**:
- `rand` crate for RNG
- `uuid` crate for UUIDs
- `ring` or `rust-crypto` for cryptographic RNG

**Implementation Details**:
- Use `ThreadRng` for secure random
- Separate seeded RNG for deterministic operations
- UUID v4 uses cryptographic RNG
- Token generation uses secure bytes

**Tests**: 30+ tests covering all distributions and UUID generation

---

### 3. Time Module

**File**: `src/stdlib/time.rs`
**Duration**: 3 days
**From Language Spec**: §1643-1655

**API**:
```graphoid
import "time"

# Create time values
current = time.now()                          # Current time
today = time.today()                         # Start of today
birthday = time.from_numbers(1990, 12, 25)
meeting = time.from_numbers(2025, 1, 15, 14, 30, 0)
parsed = time.from_string("2025-01-15T14:30:00Z")
from_ts = time.from_timestamp(1704067200)

# Format time
formatted = current.format("%Y-%m-%d %H:%M:%S")
iso = current.iso()                          # ISO 8601 format

# Extract components
year = current.year()
month = current.month()                      # 1-12
day = current.day()                          # 1-31
hour = current.hour()                        # 0-23
minute = current.minute()                    # 0-59
second = current.second()                    # 0-59
weekday = current.weekday()                  # 1=Mon, 7=Sun

# Arithmetic
tomorrow = today.add_days(1)
next_week = today.add_weeks(1)
in_2_hours = current.add_hours(2)
diff = time2.diff(time1)                     # Duration in seconds

# Comparisons
is_after = time1.after(time2)
is_before = time1.before(time2)
is_same = time1.same_as(time2)

# Timezone support
utc = time.now_utc()
local = utc.to_local()
ny_time = utc.to_timezone("America/New_York")
```

**Dependencies**:
- `chrono` crate for date/time handling
- `chrono-tz` for timezone support

**Time Value Type**:
```rust
pub struct TimeValue {
    datetime: chrono::DateTime<chrono::Utc>,
}

impl TimeValue {
    pub fn now() -> Self { ... }
    pub fn from_timestamp(ts: i64) -> Self { ... }
    pub fn year(&self) -> i32 { ... }
    // ... other methods
}
```

**Tests**: 40+ tests covering creation, formatting, arithmetic, timezones

---

### 4. Regex Module (re)

**File**: `src/stdlib/regex.rs`
**Duration**: 2 days
**From Language Spec**: Not fully specified (basic regex support mentioned)

**API**:
```graphoid
import "regex"  # alias: re

# Create pattern
pattern = regex.compile(r"\d{3}-\d{4}")

# Match
match = pattern.match("Phone: 555-1234")
if match {
    full_match = match.group(0)
    print("Found: " + full_match)
}

# Find all matches
matches = pattern.find_all(text)
for m in matches {
    print(m.group(0))
}

# Replace
new_text = pattern.replace(text, "XXX-XXXX")

# Split
parts = regex.split(r"\s+", text)

# Test
is_match = regex.test(r"^\d+$", "12345")  # Quick test
```

**Functions**:
- `compile(pattern)` - Compile regex pattern
- `test(pattern, text)` - Quick boolean test
- `split(pattern, text)` - Split by pattern
- Pattern methods:
  - `.match(text)` - Find first match
  - `.find_all(text)` - Find all matches
  - `.replace(text, replacement)` - Replace matches
  - `.groups()` - Capture groups

**Dependencies**:
- `regex` crate

**Implementation**:
```rust
pub struct RegexPattern {
    regex: regex::Regex,
}

impl RegexPattern {
    pub fn match_text(&self, text: &str) -> Option<RegexMatch> {
        self.regex.find(text).map(|m| RegexMatch {
            text: m.as_str().to_string(),
            start: m.start(),
            end: m.end(),
        })
    }
}
```

**Tests**: 35+ tests covering patterns, groups, replacements

---

### 5. I/O Module (io)

**File**: `src/stdlib/io.rs`
**Duration**: 2 days
**From Language Spec**: §1638 (file I/O mentioned)

**API**:
```graphoid
import "io"

# Read file
content = io.read("data.txt")
lines = io.read_lines("data.txt")
bytes = io.read_bytes("image.png")

# Write file
io.write("output.txt", content)
io.write_lines("output.txt", lines)
io.write_bytes("output.png", bytes)

# Append
io.append("log.txt", "Log entry\n")

# File operations
exists = io.exists("file.txt")
size = io.size("file.txt")
io.delete("temp.txt")
io.copy("src.txt", "dest.txt")
io.move("old.txt", "new.txt")

# Directory operations
io.mkdir("new_dir")
io.rmdir("old_dir")
files = io.list_dir(".")
is_dir = io.is_dir("path")
is_file = io.is_file("path")

# Path operations
abs_path = io.absolute_path("relative/path")
joined = io.join_path("dir", "file.txt")
basename = io.basename("/path/to/file.txt")  # "file.txt"
dirname = io.dirname("/path/to/file.txt")    # "/path/to"
```

**Dependencies**:
- Standard library `std::fs`
- `std::path::Path`

**Error Handling**:
```graphoid
try {
    content = io.read("file.txt")
} catch IOError as e {
    print("Error reading file: " + e.message())
}
```

**Tests**: 40+ tests covering all file operations and error cases

---

### 6. JSON Module

**File**: `src/stdlib/json.rs`
**Duration**: 2 days
**From Language Spec**: §1773-1781

**API**:
```graphoid
import "json"

# Parse JSON
data = json.parse('{"name": "Alice", "age": 30}')
name = data["name"]

# Stringify to JSON
json_str = json.stringify(data)

# Pretty-print
pretty = json.stringify(data, indent: 2)

# Parse with error handling
try {
    data = json.parse(user_input)
} catch JSONParseError as e {
    print("Invalid JSON: " + e.message())
}
```

**Functions**:
- `parse(text)` - Parse JSON string to Graphoid value
- `stringify(value)` - Convert value to JSON string
- `stringify(value, options)` - With formatting options

**Options**:
- `indent` - Indentation spaces (default: 0 for compact)
- `sort_keys` - Sort object keys (default: false)

**Dependencies**:
- `serde_json` crate

**Implementation**:
```rust
pub fn parse_json(text: &str) -> Result<Value, GraphoidError> {
    let json_value: serde_json::Value = serde_json::from_str(text)?;
    convert_json_to_value(json_value)
}

fn convert_json_to_value(json: serde_json::Value) -> Value {
    match json {
        serde_json::Value::Null => Value::None,
        serde_json::Value::Bool(b) => Value::Boolean(b),
        serde_json::Value::Number(n) => Value::Number(n.as_f64().unwrap()),
        serde_json::Value::String(s) => Value::String(s),
        serde_json::Value::Array(arr) => {
            let values: Vec<Value> = arr.into_iter()
                .map(convert_json_to_value)
                .collect();
            Value::List(List::from_vec(values))
        }
        serde_json::Value::Object(obj) => {
            let mut hash = HashMap::new();
            for (k, v) in obj {
                hash.insert(k, convert_json_to_value(v));
            }
            Value::Hash(hash)
        }
    }
}
```

**Tests**: 30+ tests covering parsing, generation, error cases

---

### 7. YAML Module

**File**: `src/stdlib/yaml.rs`
**Duration**: 2 days
**New Module** (not in original spec)

**API**:
```graphoid
import "yaml"

# Parse YAML
data = yaml.parse(yaml_text)

# Generate YAML
yaml_str = yaml.generate(data)

# Parse from file
data = yaml.parse_file("config.yaml")

# Multi-document YAML
docs = yaml.parse_all(multi_doc_yaml)
```

**Functions**:
- `parse(text)` - Parse YAML string
- `parse_file(path)` - Parse YAML file
- `parse_all(text)` - Parse multi-document YAML
- `generate(value)` - Convert value to YAML
- `generate(value, options)` - With formatting options

**Dependencies**:
- `serde_yaml` crate

**Implementation**: Similar to JSON module but using `serde_yaml`

**Tests**: 25+ tests covering YAML features (anchors, multi-docs, etc.)

---

### 8. Crypto Module

**File**: `src/stdlib/crypto.rs`
**Duration**: 3 days
**From Language Spec**: §1644 (mentioned as crypto module)

**API**:
```graphoid
import "crypto"

# Hashing
md5 = crypto.md5("message")
sha1 = crypto.sha1("message")
sha256 = crypto.sha256("message")
sha512 = crypto.sha512("message")

# HMAC
hmac = crypto.hmac_sha256("message", "secret_key")

# Encryption (AES)
encrypted = crypto.aes_encrypt(plaintext, key, iv)
decrypted = crypto.aes_decrypt(encrypted, key, iv)

# Base64 encoding
encoded = crypto.base64_encode(data)
decoded = crypto.base64_decode(encoded)

# Hex encoding
hex_str = crypto.hex_encode(bytes)
bytes = crypto.hex_decode(hex_str)

# Password hashing (bcrypt)
hashed = crypto.bcrypt("password", cost: 12)
is_valid = crypto.bcrypt_verify("password", hashed)
```

**Dependencies**:
- `sha2`, `md-5` crates for hashing
- `aes` crate for encryption
- `base64` crate for encoding
- `bcrypt` crate for password hashing

**Security Considerations**:
- Use constant-time comparisons
- Zeroize sensitive data
- Warn about MD5/SHA1 deprecation

**Tests**: 35+ tests covering all crypto operations

---

### 9. Math Module

**File**: `src/stdlib/math.rs`
**Duration**: 2 days
**Moved from Phase 10** - Mathematical functions

**API**:
```graphoid
import "math"

# Basic operations
result = math.sqrt(16)                    # 4.0
power = math.pow(2, 8)                    # 256.0
absolute = math.abs(-42)                  # 42

# Trigonometric functions
sine = math.sin(math.PI / 2)              # 1.0
cosine = math.cos(0)                      # 1.0
tangent = math.tan(math.PI / 4)           # 1.0
arcsine = math.asin(1)                    # π/2
arccosine = math.acos(1)                  # 0
arctangent = math.atan(1)                 # π/4
atan2 = math.atan2(1, 1)                  # π/4

# Hyperbolic functions
sinh = math.sinh(1)
cosh = math.cosh(1)
tanh = math.tanh(1)

# Rounding
floored = math.floor(3.7)                 # 3.0
ceiled = math.ceil(3.2)                   # 4.0
rounded = math.round(3.5)                 # 4.0
rounded_prec = math.round(3.14159, 2)     # 3.14
truncated = math.trunc(3.9)               # 3.0

# Logarithms and exponentials
natural_log = math.log(math.E)            # 1.0
log10 = math.log10(100)                   # 2.0
log2 = math.log2(8)                       # 3.0
exp = math.exp(1)                         # e (2.718...)
pow_e = math.exp(2)                       # e^2

# Advanced functions
hypotenuse = math.hypot(3, 4)             # 5.0 (sqrt(3^2 + 4^2))
factorial = math.factorial(5)             # 120
gcd = math.gcd(48, 18)                    # 6
lcm = math.lcm(12, 18)                    # 36

# Angle conversion
radians = math.radians(180)               # π
degrees = math.degrees(math.PI)           # 180.0

# Special values
inf = math.inf                            # Infinity
neg_inf = math.neg_inf                    # -Infinity
nan = math.nan                            # Not a Number

# Checks
is_nan = math.is_nan(value)
is_inf = math.is_inf(value)
is_finite = math.is_finite(value)

# Min/Max (variadic)
minimum = math.min(5, 2, 8, 1)            # 1
maximum = math.max(5, 2, 8, 1)            # 8

# Clamp
clamped = math.clamp(15, 0, 10)           # 10 (clamps to [0, 10])

# Sign
sign_pos = math.sign(42)                  # 1
sign_neg = math.sign(-42)                 # -1
sign_zero = math.sign(0)                  # 0
```

**Mathematical Constants** (also available via constants module):
```graphoid
PI = math.PI                              # 3.141592653589793
E = math.E                                # 2.718281828459045
TAU = math.TAU                            # 6.283185307179586 (2π)
PHI = math.PHI                            # 1.618033988749895 (golden ratio)
SQRT2 = math.SQRT2                        # √2
SQRT3 = math.SQRT3                        # √3
```

**Functions**:
- `sqrt(x)` - Square root
- `pow(base, exp)` - Power (base^exp)
- `abs(x)` - Absolute value
- `sin(x)`, `cos(x)`, `tan(x)` - Trigonometric functions (radians)
- `asin(x)`, `acos(x)`, `atan(x)` - Inverse trigonometric
- `atan2(y, x)` - Two-argument arctangent
- `sinh(x)`, `cosh(x)`, `tanh(x)` - Hyperbolic functions
- `floor(x)` - Round down
- `ceil(x)` - Round up
- `round(x)` - Round to nearest
- `round(x, precision)` - Round to n decimal places
- `trunc(x)` - Truncate to integer
- `log(x)` - Natural logarithm (base e)
- `log10(x)` - Logarithm base 10
- `log2(x)` - Logarithm base 2
- `exp(x)` - e^x
- `hypot(x, y)` - Euclidean distance sqrt(x^2 + y^2)
- `factorial(n)` - n! (positive integers only)
- `gcd(a, b)` - Greatest common divisor
- `lcm(a, b)` - Least common multiple
- `radians(degrees)` - Convert degrees to radians
- `degrees(radians)` - Convert radians to degrees
- `is_nan(x)` - Check if NaN
- `is_inf(x)` - Check if infinite
- `is_finite(x)` - Check if finite
- `min(...)` - Minimum of arguments (variadic)
- `max(...)` - Maximum of arguments (variadic)
- `clamp(value, min, max)` - Clamp value to range
- `sign(x)` - Sign of number (-1, 0, or 1)

**Implementation**:
```rust
pub struct MathModule;

impl NativeModule for MathModule {
    fn name(&self) -> &str { "math" }

    fn functions(&self) -> HashMap<String, NativeFunction> {
        let mut funcs = HashMap::new();
        funcs.insert("sqrt".to_string(), math_sqrt as NativeFunction);
        funcs.insert("pow".to_string(), math_pow as NativeFunction);
        funcs.insert("abs".to_string(), math_abs as NativeFunction);
        funcs.insert("sin".to_string(), math_sin as NativeFunction);
        // ... etc
        funcs
    }

    fn constants(&self) -> HashMap<String, Value> {
        let mut consts = HashMap::new();
        consts.insert("PI".to_string(), Value::Number(std::f64::consts::PI));
        consts.insert("E".to_string(), Value::Number(std::f64::consts::E));
        consts.insert("TAU".to_string(), Value::Number(std::f64::consts::TAU));
        consts.insert("inf".to_string(), Value::Number(f64::INFINITY));
        consts.insert("nan".to_string(), Value::Number(f64::NAN));
        // ... etc
        consts
    }
}

fn math_sqrt(args: &[Value]) -> Result<Value, GraphoidError> {
    if args.len() != 1 {
        return Err(GraphoidError::runtime("sqrt requires 1 argument"));
    }
    let num = args[0].as_number()?;
    if num < 0.0 {
        return Err(GraphoidError::runtime("sqrt of negative number"));
    }
    Ok(Value::Number(num.sqrt()))
}

fn math_pow(args: &[Value]) -> Result<Value, GraphoidError> {
    if args.len() != 2 {
        return Err(GraphoidError::runtime("pow requires 2 arguments"));
    }
    let base = args[0].as_number()?;
    let exp = args[1].as_number()?;
    Ok(Value::Number(base.powf(exp)))
}

// ... other functions
```

**Dependencies**:
- Standard library `std::f64` for math functions

**Tests**: 40+ tests covering all mathematical operations and edge cases

---

### 10. OS Module

**File**: `src/stdlib/os.rs`
**Duration**: 3 days
**New Module** (inspired by Python's os module)

**API**:
```graphoid
import "os"

# Environment variables
home = os.getenv("HOME")
os.setenv("MY_VAR", "value")
os.unsetenv("MY_VAR")
all_env = os.environ()  # Hash of all env vars

# Current directory
cwd = os.getcwd()
os.chdir("/path/to/dir")

# Process info
pid = os.getpid()
ppid = os.getppid()
uid = os.getuid()       # Unix only
gid = os.getgid()       # Unix only

# Execute commands
exit_code = os.system("ls -la")
output = os.exec("echo 'hello'")  # Returns stdout

# Process management
child_pid = os.spawn("command", ["arg1", "arg2"])
exit_status = os.wait(child_pid)
os.kill(pid, signal)

# Platform info
platform = os.platform()  # "linux", "macos", "windows"
arch = os.arch()          # "x86_64", "aarch64", etc.
hostname = os.hostname()

# User info
username = os.username()
homedir = os.homedir()

# Path separators
sep = os.path_sep()       # "/" or "\"
pathsep = os.pathlist_sep()  # ":" or ";"

# Temp directory
tmpdir = os.tmpdir()
tmpfile = os.tmpfile()    # Create temp file

# File permissions (Unix)
os.chmod("file.txt", 0o755)
perms = os.stat("file.txt")  # File metadata
```

**Functions**:

**Environment**:
- `getenv(name)` - Get environment variable
- `setenv(name, value)` - Set environment variable
- `unsetenv(name)` - Remove environment variable
- `environ()` - All environment variables as hash

**Directory**:
- `getcwd()` - Current working directory
- `chdir(path)` - Change directory
- `listdir(path)` - List directory contents (delegated to io module)

**Process**:
- `getpid()` - Current process ID
- `getppid()` - Parent process ID
- `system(command)` - Execute shell command
- `exec(command)` - Execute and capture output
- `spawn(command, args)` - Spawn child process
- `wait(pid)` - Wait for process
- `kill(pid, signal)` - Send signal to process

**Platform**:
- `platform()` - OS name
- `arch()` - CPU architecture
- `hostname()` - Machine hostname

**User**:
- `username()` - Current username
- `homedir()` - Home directory
- `getuid()` / `getgid()` - Unix user/group ID

**Dependencies**:
- Standard library `std::env`, `std::process`
- `users` crate for user information
- `hostname` crate for hostname

**Platform-Specific**:
- Unix-only functions return error on Windows
- Windows-only functions return error on Unix
- Document platform compatibility

**Tests**: 40+ tests (platform-specific test suites)

---

## Implementation Strategy

### Week 1 (Days 1-5):
- **Day 1**: Constants module
- **Day 2-3**: Random module (distributions, UUIDs)
- **Day 4-5**: I/O module (file operations)

### Week 2 (Days 6-10):
- **Day 6-8**: Time module (chrono integration, timezones)
- **Day 9-10**: JSON module

### Week 3 (Days 11-14):
- **Day 11-12**: Regex module
- **Day 13-14**: YAML module

### Week 4 (Days 15-19):
- **Day 15-17**: Crypto module (hashing, encryption)
- **Day 18-19**: Math module (mathematical functions)

### Week 5 (Days 20-22):
- **Day 20-22**: OS module (environment, process)

### Week 6 (Days 23):
- Integration testing across all modules
- Performance profiling
- Documentation

---

## Native Module Interface

**Rust Trait for Native Modules**:
```rust
pub trait NativeModule {
    fn name(&self) -> &str;
    fn functions(&self) -> HashMap<String, NativeFunction>;
    fn constants(&self) -> HashMap<String, Value>;
}

pub type NativeFunction = fn(&[Value]) -> Result<Value, GraphoidError>;
```

**Registration**:
```rust
pub fn register_stdlib_modules(executor: &mut Executor) {
    executor.register_native_module(Box::new(ConstantsModule));
    executor.register_native_module(Box::new(RandomModule));
    executor.register_native_module(Box::new(TimeModule));
    // ... etc
}
```

---

## Testing Strategy

**Each module must have**:
- 25-40 unit tests
- Integration tests
- Error case coverage
- Performance benchmarks (for critical paths)

**Total Tests**: 290-340 tests for Phase 12

**Platform Testing**:
- Run on Linux, macOS, Windows
- Platform-specific tests marked appropriately

---

## Performance Benchmarks

**Benchmark critical operations**:
- JSON parsing (large files)
- Regex matching (complex patterns)
- Crypto hashing (various algorithms)
- File I/O (large files)

**File**: `benches/stdlib_benchmarks.rs`

---

## Success Criteria

- [ ] ✅ All 10 modules implemented in Rust
- [ ] ✅ 290+ tests passing
- [ ] ✅ Cross-platform compatibility (Linux, macOS, Windows)
- [ ] ✅ Performance benchmarks meet targets
- [ ] ✅ Memory safety verified (no unsafe without justification)
- [ ] ✅ Error handling robust
- [ ] ✅ Documentation complete with examples
- [ ] ✅ REPL works with all native modules
- [ ] ✅ Zero compiler warnings

---

## Dependencies

**Rust Crates**:
- `rand`, `uuid` - Random module
- `chrono`, `chrono-tz` - Time module
- `regex` - Regex module
- `serde_json` - JSON module
- `serde_yaml` - YAML module
- `sha2`, `md-5`, `aes`, `base64`, `bcrypt` - Crypto module
- `users`, `hostname` - OS module

**Add to `Cargo.toml`**:
```toml
[dependencies]
rand = "0.8"
uuid = { version = "1.0", features = ["v4", "v1"] }
chrono = { version = "0.4", features = ["serde"] }
chrono-tz = "0.8"
regex = "1.10"
serde_json = "1.0"
serde_yaml = "0.9"
sha2 = "0.10"
md-5 = "0.10"
aes = "0.8"
base64 = "0.21"
bcrypt = "0.15"
users = "0.11"
hostname = "0.3"
```

---

## References

- **Language Specification**: §1556-1852 "Standard Library"
- **Rust Book**: For Rust best practices
- **Cargo Book**: For dependency management
- **Phase 10**: Module system provides loading mechanism

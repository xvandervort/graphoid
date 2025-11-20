# Phase 11: Pure Graphoid Stdlib - Completion Checklist

## Overview

Phase 11 implements the standard library in **pure Graphoid** (.gr files) to dogfood the language and achieve 90%+ self-hosting. All modules build on the native primitives from Phase 12 (os, fs, net, random, constants).

**Current Status**: ~60% Complete (7/11 planned modules exist, need verification + 4 missing modules)

---

## Completion Checklist

### ✅ Completed Modules (VERIFIED - November 20, 2025)

All modules verified and tested! See `dev_docs/PHASE_11_ISSUES.md` for detailed findings.

- [x] **statistics.gr** (491 lines) - Descriptive statistics
  - [x] Test: Can import and use from .gr file ✅
  - [x] Test: All functions work (mean, median, stdev, variance, min, max, sum, count, quantile, range, mode) ✅
  - [x] Verify: Default value handling for empty lists ✅
  - **Test file:** `tests/stdlib/test_statistics.gr`

- [x] **csv.gr** (166 lines) - CSV parsing and generation
  - [x] Test: Can parse CSV lines ✅
  - [x] Test: Can format values to CSV ✅
  - [x] Verify: Quote escaping works correctly ✅
  - **Test file:** `tests/stdlib/test_csv.gr`

- [x] **json.gr** (351 lines) - JSON parsing (bonus module)
  - [x] Test: Can parse JSON (primitives, arrays, objects) ✅
  - [x] Test: Can stringify to JSON ✅
  - [x] Verify: Nested objects and arrays work ✅
  - **Test file:** `tests/stdlib/test_json.gr`

- [x] **regex.gr** (559 lines) - Regular expressions (bonus module)
  - [x] Test: Pattern matching works (`matches()`) ✅
  - [x] Test: Find patterns (`find()`) ✅
  - [x] Test: Replace patterns (`replace()`) ✅
  - **Test file:** `tests/stdlib/test_regex.gr`

- [x] **time.gr** (310 lines) - Date/time handling (bonus module)
  - [x] Test: Current timestamp (`now()`) ✅
  - [x] Test: Date creation (`from_date()`) ✅
  - [x] Test: Formatting (ISO format) ✅
  - [x] Test: Leap year detection ✅
  - **Test file:** `tests/stdlib/test_time.gr`

- [x] **collections.gr** (583 lines) - Collection utilities (bonus module)
  - [x] Test: zip, flatten, unique functions work ✅
  - [x] Verify: Complements built-in methods ✅
  - Note: Minor display issues with `.to_string()`, but functionality correct
  - **Test file:** `tests/stdlib/test_collections.gr`

- [x] **http.gr** (278 lines) - HTTP client
  - [x] Test: Module imports successfully ✅
  - [x] Test: URL parsing works (`parse_url()`) ✅
  - [x] Verify: GET/POST request structure exists ✅
  - **Test file:** `tests/stdlib/test_http.gr`

### 🔄 Modules to Relocate

- [ ] **crypto_pure.gr** (295 lines) - SHA-256 implementation
  - [ ] Move to `stdlib/experimental/crypto_experimental.gr`
  - [ ] Add warning header about experimental status
  - [ ] Update with `:32bit` directive (after Phase 13)
  - [ ] Keep as demonstration of language capability

### ❌ Missing Modules (Need Implementation)

#### 1. **sql.gr** - SQL Query Builder
**Priority**: MEDIUM
**Estimated**: 300-400 lines, 2-3 days

**Purpose**: Fluent interface for building SQL queries safely

**Planned API**:
```graphoid
import "sql"

query = sql.select("users")
  .where("age", ">", 18)
  .order_by("name")
  .limit(10)

print(query.to_sql())
# => "SELECT * FROM users WHERE age > 18 ORDER BY name LIMIT 10"

# With parameter binding
query = sql.insert("users")
  .values({"name": "Alice", "age": 25})

sql_str = query.to_sql()
params = query.params()  # Returns [name: "Alice", age: 25]
```

**Features**:
- SELECT, INSERT, UPDATE, DELETE builders
- WHERE conditions with operators
- JOINs
- ORDER BY, GROUP BY, HAVING
- LIMIT, OFFSET
- Parameter binding (SQL injection prevention)

**Why Pure Graphoid**: String manipulation, method chaining

#### 2. **html.gr** - HTML Parsing and Manipulation
**Priority**: MEDIUM
**Estimated**: 400-500 lines, 3-4 days

**Purpose**: Parse HTML, query DOM, manipulate structure

**Planned API**:
```graphoid
import "html"

doc = html.parse("<html><body><h1>Title</h1></body></html>")

# Query DOM
title = doc.select("h1").first().text()  # "Title"

# Manipulation
doc.select("h1").set_text("New Title")
doc.select("body").append("<p>New paragraph</p>")

print(doc.to_html())
```

**Features**:
- HTML parsing to DOM tree
- CSS selector queries (like querySelector)
- Element manipulation (add, remove, modify)
- Attribute handling
- Text extraction

**Why Pure Graphoid**: Tree manipulation (uses graph/tree features)

#### 3. **pp.gr** (Pretty-Print) - Formatted Output
**Priority**: HIGH
**Estimated**: 200-300 lines, 1-2 days

**Purpose**: Format data structures for debugging and display

**Planned API**:
```graphoid
import "pp"

data = {
  "name": "Alice",
  "scores": [95, 87, 92],
  "metadata": {"course": "CS101"}
}

pp.print(data)
# Output:
# {
#   "name": "Alice",
#   "scores": [
#     95,
#     87,
#     92
#   ],
#   "metadata": {
#     "course": "CS101"
#   }
# }

# Customization
pp.print(data, indent: 2, colors: true, max_depth: 3)
```

**Features**:
- Indented formatting for hashes, lists, trees
- Configurable indentation
- Color output (ANSI codes)
- Max depth limiting
- Custom formatters for types

**Why Pure Graphoid**: String formatting, pattern matching for types

#### 4. **optparse.gr** (Option Parser) - CLI Arguments
**Priority**: HIGH
**Estimated**: 250-350 lines, 2-3 days

**Purpose**: Parse command-line arguments with flags, options, validation

**Planned API**:
```graphoid
import "optparse"

parser = optparse.create("My Program")
  .flag("--verbose", "-v", "Enable verbose output")
  .option("--output", "-o", "Output file", required: true)
  .option("--count", "-n", "Number of items", type: :integer, default: 10)
  .argument("input_file", "Input file to process")

opts = parser.parse(os.args())

if opts.verbose {
  print("Verbose mode enabled")
}

print("Output: " + opts.output)
print("Count: " + opts.count.to_string())
print("Input: " + opts.input_file)
```

**Features**:
- Long and short options (--verbose, -v)
- Required vs optional
- Type validation (integer, string, boolean)
- Default values
- Help text generation
- Error messages for invalid input

**Why Pure Graphoid**: String parsing, pattern matching, validation

---

## Implementation Priority

### Phase 11.1: Verification (1-2 days) - ✅ COMPLETE (November 20, 2025)
1. ✅ Test all existing modules from .gr files
2. ✅ Document any issues with module system integration
3. ✅ Create test suite for each module (7 test files + runner script)
4. ✅ All modules functional - no fixes needed!

**Key Finding:** Must set `GRAPHOID_STDLIB_PATH` environment variable for imports to work.

**Files Created:**
- `tests/stdlib/test_*.gr` (7 test files)
- `tests/stdlib/run_all_tests.sh` (automated test runner)
- `dev_docs/PHASE_11_ISSUES.md` (detailed findings)

### Phase 11.2: High Priority Missing Modules (3-5 days)
1. **pp.gr** - Pretty-Print (most useful for debugging)
2. **optparse.gr** - Option Parser (needed for CLI tools)

### Phase 11.3: Medium Priority Missing Modules (5-7 days)
3. **sql.gr** - SQL Query Builder
4. **html.gr** - HTML Parsing

### Phase 11.4: Cleanup (1 day)
5. Move crypto_pure.gr to experimental/
6. Update documentation
7. Create examples for each module

**Total Estimated Time**: 10-15 days

---

## Testing Strategy

For each module, create test file in `tests/stdlib/`:

```
tests/
  stdlib/
    test_statistics.gr     - Test statistics module
    test_csv.gr           - Test CSV parsing
    test_http.gr          - Test HTTP client
    test_json.gr          - Test JSON parser
    test_regex.gr         - Test regex matching
    test_time.gr          - Test date/time
    test_collections.gr   - Test collection utils
    test_sql.gr           - Test SQL builder (once implemented)
    test_html.gr          - Test HTML parser (once implemented)
    test_pp.gr            - Test pretty printer (once implemented)
    test_optparse.gr      - Test option parser (once implemented)
```

**Test Requirements**:
- Import module successfully
- Call all major functions
- Verify correct output
- Test error cases

---

## Module Integration Checklist

For each module to be "complete":

- [ ] Module file exists in `stdlib/`
- [ ] Module can be imported: `import "module_name"`
- [ ] All documented functions work
- [ ] Test file exists in `tests/stdlib/`
- [ ] Tests pass
- [ ] API documentation exists
- [ ] Example program exists
- [ ] No Rust warnings when running
- [ ] Performance is acceptable

---

## Current Status Summary (Updated November 20, 2025)

| Category | Count | Status |
|----------|-------|--------|
| **Planned modules** | 7 | - |
| ✅ Verified & Working | 7 | statistics, csv, json, regex, time, collections, http |
| ❌ Missing | 4 | pp, optparse, sql, html |
| **Total modules** | 11 | 7 verified, 4 to implement |
| **Completion** | ~64% | Phase 11.1 COMPLETE ✅ |

**Phase 11.1 Status:** ✅ **COMPLETE**
- All 7 existing modules verified functional
- Test suite created (`tests/stdlib/*.gr`)
- Test runner script created (`tests/stdlib/run_all_tests.sh`)
- Issues documented (`dev_docs/PHASE_11_ISSUES.md`)

**Next:** Phase 11.2 - Implement high-priority missing modules (pp.gr, optparse.gr)

---

## Next Steps

1. **Verify existing modules** (Phase 11.1)
   - Create test files for statistics, csv, http, json, regex, time, collections
   - Run tests, document any issues
   - Fix module system integration if needed

2. **Implement high-priority missing modules** (Phase 11.2)
   - pp.gr - Pretty-Print formatter
   - optparse.gr - CLI argument parser

3. **Implement remaining modules** (Phase 11.3)
   - sql.gr - SQL query builder
   - html.gr - HTML parser

4. **Cleanup and document** (Phase 11.4)
   - Move crypto to experimental/
   - Update all documentation
   - Create comprehensive examples

5. **Proceed to Phase 13**
   - :32bit directive
   - Documentation & publishing prep

---

## Success Criteria

Phase 11 is complete when:
- ✅ All 11 modules implemented and tested
- ✅ All modules importable and functional
- ✅ Test suite for each module
- ✅ Documentation for each module
- ✅ Examples for major use cases
- ✅ Zero warnings when running any module
- ✅ Performance is acceptable for all modules

**Then**: Ready for Phase 13 (`:32bit` directive + publishing preparation)

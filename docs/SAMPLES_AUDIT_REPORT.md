# Sample Files Audit Report
**Date:** November 24, 2025
**Total Files Audited:** 44
**Files Passing:** 36 (82%)
**Files Failing:** 7 (16%)
**Empty/No Output:** 1 (2%)

---

## Executive Summary

The `rust/samples/` directory contains 44 `.gr` files that were developed incrementally during feature implementation. Many were created as ad-hoc feature tests rather than educational examples. This audit evaluates each file for:

1. **Correctness** - Does it run without errors?
2. **Educational Value** - Does it teach users something useful?
3. **Completeness** - Is it well-commented and self-contained?
4. **Appropriateness** - Is it suitable for user-facing samples?

### Key Findings

**Problems Identified:**
- 7 files fail to execute (syntax errors, missing dependencies, incomplete features)
- ~15 files are feature tests, not educational examples
- Significant duplication (e.g., 3 similar string pattern examples)
- Inconsistent organization (no clear categorization)
- Mix of beginner and advanced content without clear labeling

**Recommendations:**
1. **Remove** feature test files (11 files)
2. **Fix or remove** broken files (7 files)
3. **Reorganize** into clear categories (beginner/intermediate/advanced)
4. **Consolidate** duplicates (3 files can become 1)
5. **Create** a README with descriptions and progression path

---

## Detailed Analysis

### ❌ FAILED FILES (7 files - REMOVE OR FIX)

#### 1. `collections_demo.gr` ❌ REMOVE
- **Error:** Runtime error: Undefined variable: `chunk`
- **Issue:** Tests functions from `collections` stdlib module that don't exist yet
- **Verdict:** Pure feature test, not educational. REMOVE.

#### 2. `crypto_examples.gr` ❌ REMOVE OR FIX LATER
- **Error:** Exit code 101 (runtime error)
- **Issue:** Uses crypto functions that aren't fully implemented
- **Content:** Comprehensive examples of crypto module (PBKDF2, AES, HMAC, BLAKE3)
- **Verdict:** Good educational content BUT depends on incomplete features. REMOVE for now, restore when crypto module is complete.

#### 3. `database_module.gr` ❌ FIX OR REMOVE
- **Error:** Parser error: Expected ':' after map key at line 13, column 10
- **Issue:** Has `func` instead of `fn` (typo)
- **Content:** Demonstrates `priv` keyword for module encapsulation
- **Verdict:** Good concept but has syntax error. Either FIX or REMOVE (covered by `priv_keyword.gr`).

#### 4. `load_vs_import.gr` ❌ FIX
- **Error:** Module not found: 'math'
- **Issue:** Tries to import "math" but stdlib path not set properly in example
- **Content:** EXCELLENT educational comparison of `load` vs `import`
- **Verdict:** KEEP and FIX. Very valuable for users. Make it work without requiring stdlib.

#### 5. `multi_file_project.gr` ❌ REMOVE
- **Error:** Exit code 101
- **Issue:** Not a runnable program, just documentation/explanatory text
- **Content:** Best practices guide for project organization
- **Verdict:** Good content but wrong format. Move to `docs/user-guide/` as markdown. REMOVE from samples.

#### 6. `statistics_demo.gr` ❌ REMOVE OR FIX LATER
- **Error:** Exit code 101
- **Issue:** Depends on incomplete statistics stdlib module
- **Content:** Comprehensive statistics module demonstration
- **Verdict:** REMOVE for now, restore when statistics module is complete.

#### 7. `collections.gr` ⚠️ KEEP (actually works!)
- **Error:** False alarm - this one PASSES
- **Content:** Good intro to lists, maps, and transformations
- **Verdict:** KEEP - it's educational and works

---

### 🧪 FEATURE TEST FILES (11 files - REMOVE)

These were clearly created to test specific features during development, not to educate users:

1. **`bignum_basics.gr`** ❌ REMOVE
   - Empty output, just tests bignum arithmetic
   - Not educational

2. **`high_precision.gr`** ❌ REMOVE
   - Empty output, bignum test
   - Not educational

3. **`large_integer_arithmetic.gr`** ❌ REMOVE
   - Empty output, bignum test
   - Not educational

4. **`mixed_operations.gr`** ❌ REMOVE
   - Empty output, arithmetic test
   - Not educational

5. **`modules_basic.gr`** ❌ REMOVE
   - Just prints instructional text, not a real example
   - Says "See rust/tests/fixtures/modules/ for working examples"

6. **`modules_math.gr`** ❌ REMOVE
   - Empty output, module definition without usage
   - Not standalone educational

7. **`utils_module.gr`** ❌ REMOVE
   - Empty output, just module definition
   - Not educational

8. **`service_module.gr`** ⚠️ KEEP (part of multi-module example)
   - Used by `app_main.gr` - keep as supporting file
   - Demonstrates 3-level module imports

9. **`sha256_demo.gr`** ⚠️ DEBATABLE
   - Mostly just explanatory text about SHA-256
   - Says to run a different file for actual demo
   - Consider removing or merging with actual SHA-256 example

10. **`bitwise_unsigned.gr`** ⚠️ CONSOLIDATE
    - Very specialized, focuses only on `:unsigned` directive
    - Could merge into `bitwise_operations.gr`

11. **`string_pattern_methods_simple.gr`** ❌ REMOVE
    - Duplicate of `string_pattern_methods.gr` but simplified
    - Keep only one version

---

### ✅ GOOD EDUCATIONAL FILES (16 files - KEEP)

These are high-quality, educational examples that work correctly:

#### Beginner Level

1. **`hello_world.gr`** ⭐⭐⭐
   - Perfect first program
   - Covers: print, variables, math, strings
   - EXCELLENT starter example

2. **`functions.gr`** ⭐⭐⭐
   - Functions, lambdas, conditionals, loops
   - Well-commented, clear progression
   - KEEP

3. **`collections.gr`** ⭐⭐⭐
   - Lists, maps, transformations
   - Good breadth of collection operations
   - KEEP

4. **`graphs.gr`** ⭐⭐
   - Basic graph creation, nodes, edges
   - Could use more detail
   - KEEP but consider expanding

#### Intermediate Level

5. **`behaviors.gr`** ⭐⭐⭐
   - EXCELLENT demonstration of behavior system
   - Clear examples with output
   - Shows chaining
   - KEEP

6. **`pattern_matching.gr`** ⭐⭐⭐
   - Clear examples of all pattern types
   - Well-organized
   - KEEP

7. **`string_generators.gr`** ⭐⭐⭐
   - Comprehensive string.generate() demo
   - Two modes clearly explained
   - Practical examples (bar charts, tables)
   - KEEP

8. **`string_mutating_methods.gr`** ⭐⭐
   - Shows mutating vs non-mutating methods
   - Clear demonstration of `!` suffix
   - KEEP

9. **`string_pattern_methods.gr`** ⭐⭐⭐
   - Comprehensive pattern matching demo
   - covers contains(), extract(), count(), find()
   - KEEP (remove the "_simple" version)

10. **`number_methods.gr`** ⭐⭐
    - sqrt(), abs(), up(), down(), round(), log()
    - Practical examples
    - KEEP

11. **`universal_casting.gr`** ⭐⭐⭐
    - Type casting across all types
    - Practical examples of truthiness
    - KEEP

12. **`integer_mode.gr`** ⭐⭐
    - Demonstrates `:integer` directive
    - Clear before/after examples
    - KEEP

13. **`function_overloading.gr`** ⭐⭐
    - Shows function overloading by arity
    - Clear examples
    - KEEP

#### Advanced Level

14. **`bitwise_operations.gr`** ⭐⭐⭐
    - Comprehensive bitwise operators
    - AND, OR, XOR, NOT, shifts, power
    - Excellent comments
    - KEEP

15. **`property_projection.gr`** ⭐⭐
    - Advanced graph pattern matching
    - Property-based queries
    - KEEP but label as advanced

16. **`subgraph_operations.gr`** ⭐⭐
    - Subgraph extraction, deletion, merging
    - Good for graph operations
    - KEEP but label as advanced

---

### 📚 MODULE SYSTEM EXAMPLES (6 files - REORGANIZE)

Module-related files should be grouped together:

1. **`app_main.gr`** ⭐⭐⭐
   - EXCELLENT 3-level module import demo
   - Shows: app → service → utils
   - Well-commented, clear output
   - KEEP

2. **`service_module.gr`** ⭐
   - Supporting file for app_main.gr
   - KEEP (required by app_main)

3. **`utils_module.gr`** ⭐
   - Supporting file for app_main.gr
   - KEEP (required by app_main)

4. **`modules_main.gr`** ⭐⭐
   - Uses math module
   - Shows practical module usage
   - KEEP

5. **`priv_keyword.gr`** ⭐⭐⭐
   - EXCELLENT demo of private symbols
   - Clear examples of what's accessible
   - KEEP

6. **`load_vs_import.gr`** ⭐⭐⭐
   - MUST FIX (currently fails)
   - EXCELLENT educational content
   - Clear comparison of two approaches
   - FIX and KEEP

---

### 🔬 STDLIB MODULE DEMONSTRATIONS (11 files - ORGANIZE BY MODULE)

These demonstrate standard library modules:

#### Math & Numbers
1. **`approx_demo.gr`** ⭐⭐⭐
   - Approximate equality comparisons
   - Absolute and relative tolerance
   - Time comparisons
   - KEEP

2. **`constants.gr`** ⭐⭐⭐
   - Math constants (π, e, τ, φ, √2, etc.)
   - Physical constants (c, G, h)
   - Example calculations
   - KEEP

#### Random
3. **`random.gr`** ⭐⭐⭐
   - Comprehensive random module demo
   - All functions covered
   - Seeding, distributions, UUID
   - KEEP

#### Time
4. **`time_type.gr`** ⭐⭐⭐
   - Time creation, components, conversions
   - Practical examples
   - KEEP

#### Crypto (currently broken)
5. **`crypto_examples.gr`** ❌ BROKEN
   - REMOVE for now, restore when crypto complete

6. **`crypto_sha256.gr`** ⚠️ DEBATABLE
   - Just explanatory text
   - Consider removing

7. **`sha256_demo.gr`** ⚠️ DEBATABLE
   - Mostly text, refers to other file
   - Consider removing

#### Collections (broken)
8. **`collections_demo.gr`** ❌ BROKEN
   - REMOVE

#### Statistics (broken)
9. **`statistics_demo.gr`** ❌ BROKEN
   - REMOVE for now, restore when complete

#### Graph Pattern Matching
10. **`recommendation_system.gr`** ⭐⭐
    - Friend recommendation algorithm
    - Shows graph pattern matching
    - KEEP but label as advanced

11. **`social_network_patterns.gr`** ⭐⭐
    - Social network queries
    - Pattern matching examples
    - KEEP but label as advanced

12. **`variable_length_paths.gr`** ⭐⭐
    - Variable-length path matching
    - Advanced graph queries
    - KEEP but label as advanced

---

## Recommendations

### Phase 1: Immediate Cleanup (Remove 18 files)

**Remove these broken/incomplete files (7):**
```bash
rm rust/samples/collections_demo.gr
rm rust/samples/crypto_examples.gr
rm rust/samples/database_module.gr
rm rust/samples/multi_file_project.gr
rm rust/samples/statistics_demo.gr
rm rust/samples/load_vs_import.gr  # FIX FIRST, then restore
```

**Remove these feature test files (11):**
```bash
rm rust/samples/bignum_basics.gr
rm rust/samples/high_precision.gr
rm rust/samples/large_integer_arithmetic.gr
rm rust/samples/mixed_operations.gr
rm rust/samples/modules_basic.gr
rm rust/samples/modules_math.gr
rm rust/samples/utils_module.gr  # Used by app_main - DON'T remove yet
rm rust/samples/service_module.gr  # Used by app_main - DON'T remove yet
rm rust/samples/sha256_demo.gr
rm rust/samples/string_pattern_methods_simple.gr  # Keep the full version
```

**Actually, only remove these 9 standalone test files:**
```bash
rm rust/samples/bignum_basics.gr
rm rust/samples/high_precision.gr
rm rust/samples/large_integer_arithmetic.gr
rm rust/samples/mixed_operations.gr
rm rust/samples/modules_basic.gr
rm rust/samples/modules_math.gr
rm rust/samples/sha256_demo.gr
rm rust/samples/crypto_sha256.gr
rm rust/samples/string_pattern_methods_simple.gr
```

### Phase 2: Fix Recoverable Files (2 files)

**Fix `load_vs_import.gr`:**
- Remove dependency on "math" module
- Make example self-contained
- Restore to samples

**Fix `database_module.gr` (optional):**
- Change `func` to `fn` on line 12
- OR just remove (redundant with `priv_keyword.gr`)

### Phase 3: Reorganize (Create Directory Structure)

Create organized structure:

```
rust/samples/
├── README.md                          # NEW: Index with descriptions
├── 01-basics/
│   ├── hello_world.gr
│   ├── functions.gr
│   ├── collections.gr
│   └── graphs.gr
├── 02-intermediate/
│   ├── behaviors.gr
│   ├── pattern_matching.gr
│   ├── string_generators.gr
│   ├── string_mutating_methods.gr
│   ├── string_pattern_methods.gr
│   ├── number_methods.gr
│   ├── universal_casting.gr
│   ├── integer_mode.gr
│   ├── function_overloading.gr
│   └── bitwise_operations.gr
├── 03-advanced/
│   ├── property_projection.gr
│   ├── subgraph_operations.gr
│   ├── recommendation_system.gr
│   ├── social_network_patterns.gr
│   └── variable_length_paths.gr
├── 04-modules/
│   ├── app_main.gr
│   ├── service_module.gr
│   ├── utils_module.gr
│   ├── modules_main.gr
│   ├── priv_keyword.gr
│   └── load_vs_import.gr  # After fixing
└── 05-stdlib/
    ├── approx_demo.gr
    ├── constants.gr
    ├── random.gr
    └── time_type.gr
```

### Phase 4: Create README

Create `rust/samples/README.md`:

```markdown
# Graphoid Sample Programs

This directory contains educational examples demonstrating Graphoid features.

## Getting Started

Start with the basics:
1. `01-basics/hello_world.gr` - Your first Graphoid program
2. `01-basics/functions.gr` - Functions, lambdas, control flow
3. `01-basics/collections.gr` - Lists, maps, and transformations
4. `01-basics/graphs.gr` - Basic graph operations

## Learning Path

**Beginner** → **Intermediate** → **Advanced** → **Modules** → **Stdlib**

- **01-basics/** - Core language features (start here!)
- **02-intermediate/** - Behaviors, patterns, string operations
- **03-advanced/** - Graph pattern matching, subgraph operations
- **04-modules/** - Module system and code organization
- **05-stdlib/** - Standard library demonstrations

## Running Examples

```bash
# From rust/ directory
cargo run --release -- samples/01-basics/hello_world.gr

# With stdlib path
GRAPHOID_STDLIB_PATH=../stdlib cargo run -- samples/02-intermediate/behaviors.gr
```

## File Descriptions

### 01-basics/
- `hello_world.gr` - Print, variables, math, strings
- `functions.gr` - Function definitions, lambdas, conditionals, loops
- `collections.gr` - Lists, maps, transformations (filter, map, reject)
- `graphs.gr` - Creating graphs, adding nodes/edges, queries

### 02-intermediate/
- `behaviors.gr` - Automatic value transformations (none_to_zero, validate_range)
- `pattern_matching.gr` - Match expressions with numbers, strings, lists
- `string_generators.gr` - string.generate() for repetition and sequences
- `string_mutating_methods.gr` - Mutating vs non-mutating methods (! suffix)
- `string_pattern_methods.gr` - Pattern matching (contains, extract, count, find)
- `number_methods.gr` - Math methods (sqrt, abs, round, log)
- `universal_casting.gr` - Type casting and truthiness
- `integer_mode.gr` - :integer directive for truncating floats
- `function_overloading.gr` - Overloading by arity
- `bitwise_operations.gr` - Bitwise AND, OR, XOR, shifts, power operator

### 03-advanced/
- `property_projection.gr` - Graph pattern matching with property filters
- `subgraph_operations.gr` - Extract, delete, merge subgraphs
- `recommendation_system.gr` - Friend recommendation algorithm
- `social_network_patterns.gr` - Social network queries
- `variable_length_paths.gr` - Variable-length path matching (1-N hops)

### 04-modules/
- `app_main.gr` - Three-level module import demo (app → service → utils)
- `priv_keyword.gr` - Private symbols in modules
- `load_vs_import.gr` - Comparison of load (merge) vs import (namespace)
- Supporting files: `service_module.gr`, `utils_module.gr`, `modules_main.gr`

### 05-stdlib/
- `approx_demo.gr` - Approximate equality (approx module)
- `constants.gr` - Math and physical constants
- `random.gr` - Random numbers, distributions, UUID
- `time_type.gr` - Time creation, parsing, components

## Contributing Examples

Good examples should:
- ✓ Be self-contained and runnable
- ✓ Have clear comments explaining concepts
- ✓ Show expected output
- ✓ Demonstrate one main concept
- ✓ Use realistic variable names
- ✗ Don't be feature tests
- ✗ Don't require deep Graphoid knowledge
```

---

## Summary Statistics

### Current State (44 files)
- ✅ Passing: 36 files (82%)
- ❌ Failing: 7 files (16%)
- 🧪 Feature tests: ~11 files (25%)
- 📚 Educational: ~25 files (57%)

### After Cleanup (26 files recommended)
- **Basics:** 4 files
- **Intermediate:** 10 files
- **Advanced:** 5 files
- **Modules:** 5 files
- **Stdlib:** 4 files
- **To Fix:** 2 files (load_vs_import.gr, database_module.gr)

### Quality Distribution
- ⭐⭐⭐ Excellent (13 files): 50%
- ⭐⭐ Good (10 files): 38%
- ⭐ Adequate (3 files): 12%

---

## Conclusion

The samples directory needs significant cleanup:
1. **Remove 9-11 files** that are feature tests
2. **Fix 2 files** that have good content but don't run
3. **Remove 5 files** that depend on incomplete features
4. **Reorganize remaining ~26 files** into clear categories
5. **Create README** with learning path

This will result in a clean, organized set of educational examples that guide users from beginner to advanced topics.

**Recommended Immediate Action:**
Delete the 9 standalone feature test files and 5 broken/incomplete files (14 total), leaving 30 files. Then organize those 30 into the suggested directory structure.

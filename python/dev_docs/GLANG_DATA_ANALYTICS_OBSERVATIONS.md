# Glang Data Analytics Experiment - Observations and Findings

## Experiment Overview

Successfully created and analyzed a realistic cryptocurrency dataset containing 900 records across 3 cryptocurrencies (Bitcoin, Litecoin, Ethereum) over 300 days. Performed comprehensive analytics including price ranges, returns calculation, volatility analysis, and performance comparisons.

## ✅ What Works Well

### 1. Basic Data Processing
- **CSV File I/O**: Excellent - `io.read_file()` and string processing work flawlessly
- **String Manipulation**: Very good - `split()`, `trim()`, `length()` operations are reliable
- **List Operations**: Good - `append()`, `size()`, index access work well
- **Type Conversion**: Solid - `to_num()`, `to_string()` conversions are accurate
- **Numeric Calculations**: Excellent - arithmetic operations, comparisons work perfectly

### 2. Control Flow and Logic
- **While Loops**: Work perfectly for data iteration
- **Conditional Logic**: Basic `if`/`else` statements work reliably
- **Variable Management**: Basic variable declarations and assignments work fine

### 3. Performance
- **Speed**: Processed 900 records quickly with no performance issues
- **Memory**: No memory issues with moderately sized datasets
- **Accuracy**: All calculations (min, max, averages, percentages) were mathematically correct

## ⚠️ Significant Limitations Discovered

### 1. Hash/Map Data Structure Issues
**Critical Problem**: Hash key access with variables is severely limited
```glang
# THIS DOESN'T WORK:
string key = "symbol"
value = record[key]  # Parser error: "Key must be a string literal"

# WORKAROUND REQUIRED:
symbol_node = record.node("symbol")
string symbol = symbol_node.value().to_string()
```

**Impact**: Makes data structure manipulation very cumbersome. Had to abandon hash-based approach entirely and use separate lists.

### 2. Logical Operator Precedence Issues
**Critical Problem**: Complex logical expressions parse incorrectly
```glang
# THIS BREAKS:
if a > 0 && b > 0 && c > 0 { ... }

# WORKAROUND REQUIRED:
if a > 0 {
    if b > 0 {
        if c > 0 { ... }
    }
}
```

**Impact**: Forces verbose, deeply nested code instead of natural logical expressions.

### 3. Variable Scoping Problems
**Problem**: Variables declared in different scopes conflict globally
```glang
# THIS FAILS:
for record in records {
    num price = record.price()  # Works first time
}
for record in other_records {
    num price = record.price()  # ERROR: 'price' already declared
}
```

**Impact**: Forces unique variable names throughout entire program, making code repetitive and less readable.

### 4. Limited Data Structure Support
**Missing Features**:
- No built-in data frames or structured data types
- Hash access limitations make record-like structures difficult
- No native support for CSV parsing (had to implement manually)
- No grouping or aggregation functions

## 🎯 Successful Analytics Achieved

Despite limitations, successfully implemented:

### Statistical Analysis
- **Min/Max/Average calculations** across 300 data points per currency
- **Percentage change calculations** over time periods
- **Performance comparisons** between multiple assets
- **Volatility analysis** (positive vs negative days)

### Data Transformation
- **String to numeric conversion** for calculations
- **Data filtering** by cryptocurrency type
- **Time series analysis** (first vs last values)
- **Return calculations** with proper percentage formatting

### Results Summary
- **Bitcoin**: -29.68% return (bear market simulation)
- **Litecoin**: +6.59% return (modest growth)
- **Ethereum**: +406.24% return (strong growth)
- **Best Performer**: Ethereum identified correctly

## 📊 Code Complexity Assessment

### Simple Analytics: ⭐⭐⭐⭐⭐ (Excellent)
Basic calculations, single-pass data processing work beautifully.

### Intermediate Analytics: ⭐⭐⭐ (Good with workarounds)
Multi-variable analysis possible but requires careful variable naming and nested logic.

### Advanced Analytics: ⭐⭐ (Limited)
Complex data structures, grouping operations, and advanced statistical functions would be very difficult.

## 🔧 Recommended Improvements for Glang

### Priority 1: Parser Fixes
1. **Fix logical operator precedence** - Enable `a && b && c` syntax
2. **Allow variable hash keys** - Enable `hash[variable_key]` access
3. **Improve variable scoping** - Allow variable reuse in different scopes

### Priority 2: Data Structure Enhancements
1. **Native CSV parsing** - Built-in CSV reader/writer
2. **Data frame type** - Structured data with column access
3. **Grouping operations** - `group_by()`, `aggregate()` functions
4. **Statistical functions** - `sum()`, `mean()`, `std_dev()`, etc.

### Priority 3: Library Additions
1. **Statistics module** - Advanced statistical functions
2. **Data analysis module** - Filtering, sorting, grouping operations
3. **Export functions** - JSON, CSV output formatting

## 📈 Overall Assessment

**Current State**: Glang handles basic data analytics well but struggles with complex data manipulation.

**Strengths**:
- Solid foundation for numeric computation
- Reliable I/O and string processing
- Good performance for moderate datasets

**Weaknesses**:
- Hash/map limitations severely restrict data structure flexibility
- Parser bugs make complex expressions difficult
- Missing higher-level data analysis abstractions

**Recommendation**: Fix the parser issues and hash limitations as immediate priorities. These are blocking issues that make real-world data analysis unnecessarily difficult. The underlying computational engine is solid - it's the language syntax that needs work.

**Verdict**: With the parser fixes we've planned for next session, Glang would become significantly more powerful for data analytics. The foundation is strong, but the syntax limitations are holding it back.

---

*Experiment Date: September 2025*
*Dataset: 900 cryptocurrency records, 3 assets, 300 days*
*Analysis Types: Statistical, time series, performance comparison*
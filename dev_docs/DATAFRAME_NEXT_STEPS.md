# DataFrame Development - Next Steps

**Status**: January 2025 - Forward/backward fill behaviors completed, documentation updated

## Recently Completed ✅

### Missing Data Handling Implementation
- **✅ Forward Fill Behavior**: `forward_fill` behavior implemented with marker-based two-pass processing
- **✅ Backward Fill Behavior**: `backward_fill` behavior implemented with contextual processing
- **✅ Behavior Integration**: DataFrame columns inherit from `GraphContainer`, full behavior support available
- **✅ Comprehensive Testing**: 14 test cases covering all edge cases (leading/trailing nones, empty data, mixed types)
- **✅ Documentation**: Added comprehensive "Missing Data Handling" section to `docs/dataframe_reference.md`

**Architecture Achievement**: Glang DataFrames now have **superior missing data handling** compared to pandas:
- Behaviors are intrinsic to the data structure
- Automatically apply to new values
- Composable (multiple behaviors on same column)
- Type-safe and memory efficient

## Remaining Priorities 🔄

### 1. CSV Type Auto-Detection
**Goal**: Improve CSV import to automatically detect numeric vs string data types

**Current Issue**:
```glang
# Current behavior - everything imported as strings
csv_data = "name,age,salary\nAlice,30,75000\nBob,25,65000"
df = df.from_csv(csv_data, true)
# age column: ["30", "25"] (strings)
# salary column: ["75000", "65000"] (strings)
```

**Target Behavior**:
```glang
# Desired behavior - automatic type detection
df = df.from_csv(csv_data, true)
# age column: [30, 25] (numbers)
# salary column: [75000, 65000] (numbers)
```

**Implementation Approach**:
1. Extend `from_csv()` function in `stdlib/dataframe.gr`
2. Add type detection logic for each parsed value:
   - Try parsing as number first
   - Fall back to string if parsing fails
   - Handle special cases: "true"/"false" → boolean
3. Maintain backward compatibility with `has_headers` parameter

**Files to Modify**:
- `stdlib/dataframe.gr` - Update `from_csv()` function
- `test/test_dataframe_csv.py` - Add type detection tests

### 2. DataFrame Join Operations
**Goal**: Implement left/outer joins for merging DataFrames

**Current Gap**:
```glang
# No join operations currently available
employees = df.from_column_data({...})
departments = df.from_column_data({...})
# No way to: merged = df.merge(employees, departments, "dept_id", "left")
```

**Target API**:
```glang
# Left join
left_joined = df.merge(employees, departments, "dept_id", "left")

# Outer join
full_joined = df.merge(employees, departments, "dept_id", "outer")

# Inner join (may already work)
inner_joined = df.merge(employees, departments, "dept_id", "inner")
```

**Implementation Approach**:
1. Add `merge()` function to `stdlib/dataframe.gr`
2. Support join types: "inner", "left", "right", "outer"
3. Handle missing keys appropriately for each join type
4. Preserve DataFrame structure and metadata

**Files to Modify**:
- `stdlib/dataframe.gr` - Add `merge()` function
- `docs/dataframe_reference.md` - Document join operations
- `test/test_dataframe_joins.py` - Comprehensive join tests

### 3. Row-Level DataFrame Operations
**Goal**: Enable row-based operations and iteration

**Current Limitation**:
```glang
# Can access columns, but no row operations
df["salary"]  # ✅ Works - gets column
# df.row[0]   # ❌ Doesn't exist - get first row
# df.rows     # ❌ Doesn't exist - row iterator
```

**Target API**:
```glang
# Row access
first_row = df.row[0]               # Get first row as map-like object
alice_data = first_row["name"]      # "Alice"
alice_salary = first_row["salary"]  # 75000

# Row operations
row_sum = df.row[0].sum()           # Sum all numeric values in row
row_total = df.row[1].sum("salary", "bonus")  # Sum specific columns

# Row iteration
for row in df.rows {
    print("Employee: " + row["name"] + ", Salary: $" + row["salary"].to_string())
}

# Row modification
df.row[0]["salary"] = 80000         # Update specific cell
df.row[0].apply("salary", x => x * 1.1)  # Transform specific cell
```

**Implementation Approach**:
1. Add `row` property to DataFrame objects that returns a RowAccessor
2. Add `rows` property that returns a RowIterator
3. RowAccessor supports indexing: `row[index]` returns RowObject
4. RowObject supports map-like access and operations
5. RowIterator supports `for row in df.rows` syntax

**Files to Modify**:
- `stdlib/dataframe.gr` - Add row access functionality
- `src/glang/execution/graph_values.py` - May need row access support
- `docs/dataframe_reference.md` - Document row operations
- `test/test_dataframe_rows.py` - Row operation tests

## Implementation Priority

1. **CSV Type Auto-Detection** - Most straightforward, high user value
2. **Row-Level Operations** - Core functionality, enables many use cases
3. **DataFrame Joins** - More complex, but essential for data analysis

## Technical Notes

### Behavior System Integration
All new DataFrame features should leverage the behavior system:
- Row operations should respect column behaviors
- Join operations should preserve behaviors from both DataFrames
- CSV import should allow behavior attachment during import

### Testing Strategy
Each feature needs comprehensive tests:
- Basic functionality tests
- Edge case tests (empty DataFrames, single rows, etc.)
- Integration tests with existing features
- Performance tests for large DataFrames

### Documentation Updates
Update both:
- `docs/dataframe_reference.md` - User-facing API documentation
- `docs/dataframe_limitations.md` - Move completed items from ❌ to ✅

## Files That Will Need Updates

### Core Implementation
- `stdlib/dataframe.gr` - Main DataFrame module
- `src/glang/execution/graph_values.py` - May need row access support

### Documentation
- `docs/dataframe_reference.md` - API documentation
- `docs/dataframe_limitations.md` - Status tracking

### Tests
- `test/test_dataframe_csv.py` - CSV import tests
- `test/test_dataframe_joins.py` - Join operation tests (new file)
- `test/test_dataframe_rows.py` - Row operation tests (new file)

This completes the foundation for Glang DataFrames becoming a full-featured data analysis tool comparable to pandas, while leveraging Glang's unique graph architecture and behavior system.
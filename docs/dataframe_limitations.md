# DataFrame Capabilities and Comparison vs Pandas

## Current Capabilities ✅

Our pure Glang DataFrame implementation (`stdlib/dataframe.gr`) now supports:

### 1. **DataFrame Creation**
   - `create(columns)` - Create empty DataFrame with specified columns
   - `from_column_data(column_data)` - Create from map of column arrays
   - `from_records(records, columns)` - Create from list of row maps
   - `from_csv(csv_text, has_headers)` - Import from CSV text

### 2. **Basic Operations**
   - `add_row(df, row_data)` - Add row with map-like syntax
   - `info(df)` - Display DataFrame structure and row count
   - `head(df, n)` - Get first n rows
   - `to_csv(df)` - Export to CSV format

### 3. **Column Operations**
   - `select(df, column_names)` - Select specific columns
   - **🆕 `transform_column(df, column, lambda_func)`** - Transform column with custom lambda
   - **🆕 `normalize_column(df, column, lambda_func)`** - Normalize column values with lambda

### 4. **Filtering & Selection**
   - `filter(df, column, predicate)` - Built-in predicates: "positive", "negative", "non_empty", "truthy"
   - **🆕 `filter_by(df, column, lambda_func)`** - Filter with custom lambda functions

### 5. **Statistics & Aggregation**
   - `aggregate(df, column, operation)` - Built-in operations: "sum", "mean", "min", "max", "count"
   - **🆕 `compute_basic_stats(df, column)`** - Complete statistical summary (count, mean, min, max, range)

### 6. **Advanced Group Operations**
   - **✅ `group_by(df, group_col, agg_col, operation)`** - Group and aggregate (uses `map.keys()`)
   - **✅ `group_by_dataframes(df, group_column)`** - Return sub-DataFrames for each group
   - **✅ `group_by_agg(df, group_col, agg_operations)`** - Multiple aggregations per group

### 7. **Lambda-Powered Analytics** 🚀
   **NEW**: Full lambda support enables pandas-like custom transformations:
   ```glang
   # Custom column transformations
   df.transform_column(employees, "salary", x => x * 1.10)  # 10% raise

   # Complex filtering with lambdas
   filtered = df.filter_by(employees, "salary", x => x > 75000)

   # Statistical analysis
   stats = df.compute_basic_stats(employees, "salary")
   # Returns: { "count": 5, "mean": 78000, "min": 65000, "max": 95000, "range": 30000 }

   # Custom normalization
   df.normalize_column(employees, "salary", x => x / 1000)  # Convert to thousands
   ```

## Major Missing Features ❌

### 1. **Advanced Group By**
```python
# Pandas
df.groupby(['category', 'region']).agg({
    'sales': ['sum', 'mean', 'std'],
    'quantity': 'sum'
})
```
**Blocker**: Needs `map.keys()` method to iterate over groups

### 2. **Multi-Index Support**
```python
# Pandas
df.set_index(['date', 'product'])
df.loc[('2024-01-01', 'Widget')]
```
**Blocker**: Requires composite key support in maps

### 3. **Advanced Slicing**
```python
# Pandas
df.iloc[5:15, 2:5]  # Row and column ranges
df.loc[df['price'] > 100, ['name', 'quantity']]
```
**Blocker**: Needs better indexing syntax in parser

### 4. **Pivot/Reshape**
```python
# Pandas
df.pivot_table(values='sales', index='date', columns='product')
df.melt(id_vars=['date'], value_vars=['A', 'B'])
```
**Complexity**: Requires sophisticated data restructuring

### 5. **Statistical Operations**
```python
# Pandas
df.describe()  # Summary statistics
df.std()       # Standard deviation
df.corr()      # Correlation matrix
df.quantile([0.25, 0.5, 0.75])
```
**Effort**: Needs math functions (sqrt for std dev)

### 6. **Time Series**
```python
# Pandas
df.resample('D').mean()  # Daily aggregation
df.rolling(7).mean()      # 7-day moving average
df.shift(1)               # Lag values
```
**Blocker**: Requires date/time type understanding

### 7. **Missing Data Handling**
```python
# Pandas
df.fillna(method='forward')
df.interpolate()
df.dropna(subset=['col1', 'col2'])
```
**Partially Available**: We handle `none` but no sophisticated imputation

### 8. **Type System**
```python
# Pandas
df.dtypes
df.astype({'price': float, 'quantity': int})
df.select_dtypes(include=['number'])
```
**Blocker**: Glang's dynamic typing makes this complex

### 9. **I/O Formats**
```python
# Pandas
df.to_parquet(), df.to_excel(), df.to_json()
pd.read_sql(), pd.read_html()
```
**Effort**: Each format needs a parser/writer

### 10. **Outer/Left/Right Joins**
```python
# Pandas
df1.merge(df2, how='left', on='id')
df1.merge(df2, how='outer', on=['id', 'date'])
```
**Complexity**: More sophisticated join logic needed

## Language-Level Issues - RESOLVED! ✅

Recent enhancements have fixed most language-level blockers:

1. ~~**No `map.keys()` method**~~ **✅ FIXED**
   - ✅ `map.keys()` is available and working
   - ✅ Powers proper `group_by()` implementation
   - ✅ Enables advanced group operations

2. ~~**Limited map literal syntax**~~ **✅ FIXED**
   - ✅ Variables and expressions can be used as keys: `{ variable: value }`
   - ✅ Dynamic map construction works: `{ prefix + "_id": 123 }`

3. ~~**No lambda parameters in map/filter**~~ **✅ FIXED**
   - ✅ Full lambda support: `df.filter_by(df, "salary", x => x > 80000)`
   - ✅ Custom transformations: `df.transform_column(df, "price", x => x * 1.1)`
   - ✅ Complex analytics: `salaries.map(x => x > threshold && x < limit)`

4. ~~**Multi-dimensional indexing**~~ **✅ NOT A BUG**
   - ✅ Glang uses column-first access by design: `df["column"][index]`
   - ✅ Functional operations replace matrix syntax: `df.select()`, `df.filter()`
   - ✅ Intentional design choice, not a limitation

5. **Operator overloading** - Can't use `df1 + df2`
   - Blocks: Intuitive mathematical operations
   - Workaround: Explicit method calls (`df.merge()`, etc.)

## What's Most Important Now?

With the major blockers resolved, priorities for enhanced DataFrame capabilities:

1. **✅ Statistical analysis** - `compute_basic_stats()` provides comprehensive metrics
2. **Add standard deviation** - Need `sqrt()` function for full statistical suite
3. **Improve CSV type parsing** - Auto-detect numbers vs strings
4. **Implement left/outer joins** - More flexible data combining
5. **Add pivot operations** - Reshape data for different analyses

## Philosophical Note

The limitations highlight an important principle: **DataFrames in Glang are governed graphs, not specialized data structures**. This means:

- Operations must respect graph governance rules
- Performance may differ from specialized implementations
- Flexibility comes from rules, not hard-coded behavior
- Future enhancements will add rules, not special cases

The beauty is that as Glang's graph capabilities grow, DataFrames automatically benefit without special implementation.
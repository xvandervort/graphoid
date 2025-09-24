# DataFrame Limitations vs Pandas

## Current Capabilities ✅

Our pure Glang DataFrame implementation (`stdlib/df.gr`) currently supports:

1. **Basic Operations**
   - Create DataFrames with named columns
   - Add rows with map-like syntax
   - Row count and column access
   - Head operation (first n rows)

2. **Filtering**
   - `filter_positive()` - values > 0
   - `filter_by_value()` - exact match
   - `filter_greater_than()` - threshold comparison

3. **Aggregation**
   - `sum_column()` - column sum
   - `avg_column()` - column average
   - `count_by()` - frequency counts
   - `get_unique()` - distinct values

4. **Transformations**
   - `select()` - column subset
   - `sort_by()` - sort by column (ascending/descending)
   - `inner_join()` - join two DataFrames on common column

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

## Language-Level Blockers

Several missing features are blocked by Glang language limitations:

1. **No `map.keys()` method** - Can't iterate over map keys
   - Blocks: Proper group_by implementation
   - Workaround: Track keys separately

2. **Limited map literal syntax** - Can't use variables as keys in literals
   - Blocks: Dynamic map construction
   - Workaround: Build maps incrementally

3. **No lambda parameters in map/filter** - Can't pass custom functions easily
   - Blocks: Flexible transformations
   - Workaround: Named predicates only

4. **No multi-dimensional indexing** - `df[1:5, 2:4]` not possible
   - Blocks: Matrix-like operations
   - Workaround: Sequential operations

5. **No operator overloading** - Can't use `df1 + df2`
   - Blocks: Intuitive mathematical operations
   - Workaround: Explicit method calls

## What's Most Important?

For practical data analysis in Glang, the priorities should be:

1. **Fix `map.keys()`** - Unblocks group_by and many operations
2. **Add standard deviation** - Essential for statistics
3. **Improve CSV handling** - Parse types, not just strings
4. **Add `describe()` method** - Quick statistical summary
5. **Implement left join** - Common data operation

## Philosophical Note

The limitations highlight an important principle: **DataFrames in Glang are governed graphs, not specialized data structures**. This means:

- Operations must respect graph governance rules
- Performance may differ from specialized implementations
- Flexibility comes from rules, not hard-coded behavior
- Future enhancements will add rules, not special cases

The beauty is that as Glang's graph capabilities grow, DataFrames automatically benefit without special implementation.
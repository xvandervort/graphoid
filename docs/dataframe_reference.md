# DataFrame Reference Guide

Complete API reference for Glang DataFrames with lambda-powered analytics capabilities.

## Quick Start

```glang
import "dataframe" as df

# Create DataFrame from column data
employees = df.from_column_data({
    "name": ["Alice", "Bob", "Charlie", "Diana"],
    "salary": [75000, 65000, 95000, 85000],
    "department": ["Engineering", "Engineering", "Management", "Marketing"]
})

# Lambda-powered transformations
df.transform_column(employees, "salary", x => x * 1.10)  # 10% raise
stats = df.compute_basic_stats(employees, "salary")      # Full statistics
```

## DataFrame Creation

### `create(columns)`
Create empty DataFrame with specified columns.
```glang
empty_df = df.create(["name", "age", "score"])
```

### `from_column_data(column_data)`
Create DataFrame from map of column arrays.
```glang
data = {
    "product": ["Widget", "Gadget", "Tool"],
    "price": [19.99, 29.99, 15.50],
    "quantity": [100, 75, 200]
}
products = df.from_column_data(data)
```

### `from_records(records, columns)`
Create DataFrame from list of row maps.
```glang
records = [
    { "name": "Alice", "age": 30 },
    { "name": "Bob", "age": 25 }
]
people = df.from_records(records, ["name", "age"])
```

### `from_csv(csv_text, has_headers)`
Import DataFrame from CSV text.
```glang
csv_data = "name,age,city\nAlice,30,NYC\nBob,25,LA"
df_from_csv = df.from_csv(csv_data, true)
```

## Basic Operations

### `info(df)`
Display DataFrame structure and row count.
```glang
df.info(employees)
# Output: DataFrame with 4 rows
#         Columns: ["name", "salary", "department"]
```

### `head(df, n)`
Get first n rows as new DataFrame.
```glang
first_two = df.head(employees, 2)
```

### `add_row(df, row_data)`
Add new row to DataFrame.
```glang
new_employee = { "name": "Eve", "salary": 90000, "department": "Engineering" }
df.add_row(employees, new_employee)
```

### `to_csv(df)`
Export DataFrame to CSV format.
```glang
csv_output = df.to_csv(employees)
```

## Column Operations

### `select(df, column_names)`
Select specific columns, returns new DataFrame.
```glang
names_and_salaries = df.select(employees, ["name", "salary"])
```

### `transform_column(df, column, lambda_func)` 🆕
Transform column values using lambda function (modifies original).
```glang
# Apply 15% raise to all salaries
df.transform_column(employees, "salary", x => x * 1.15)

# Convert names to uppercase
df.transform_column(employees, "name", x => x.upper())

# Apply complex business logic
df.transform_column(products, "price", x => x > 20 ? x * 0.9 : x)  # Discount expensive items
```

### `normalize_column(df, column, lambda_func)` 🆕
Normalize column values using lambda function (modifies original).
```glang
# Convert salaries to thousands
df.normalize_column(employees, "salary", x => x / 1000)

# Z-score normalization (if you have mean/std)
mean_salary = 78000
df.normalize_column(employees, "salary", x => (x - mean_salary) / 10000)
```

## Filtering & Selection

### `filter(df, column, predicate)`
Filter rows using built-in predicates.
```glang
# Built-in predicates: "positive", "negative", "non_empty", "truthy"
positive_salaries = df.filter(employees, "salary", "positive")
non_empty_names = df.filter(employees, "name", "non_empty")
```

### `filter_by(df, column, lambda_func)` 🆕
Filter rows using custom lambda functions.
```glang
# High earners
high_earners = df.filter_by(employees, "salary", x => x > 80000)

# Complex conditions
senior_engineers = df.filter_by(employees, "salary", x => x > 70000 && x < 100000)

# String filtering
a_names = df.filter_by(employees, "name", x => x.starts_with("A"))
```

## Statistics & Aggregation

### `aggregate(df, column, operation)`
Apply built-in aggregation operations.
```glang
# Available operations: "sum", "mean", "min", "max", "count"
total_salary = df.aggregate(employees, "salary", "sum")
avg_salary = df.aggregate(employees, "salary", "mean")
max_salary = df.aggregate(employees, "salary", "max")
employee_count = df.aggregate(employees, "name", "count")
```

### `compute_basic_stats(df, column)` 🆕
Get comprehensive statistical summary.
```glang
stats = df.compute_basic_stats(employees, "salary")
# Returns: {
#   "count": 4,
#   "mean": 78750,
#   "min": 65000,
#   "max": 95000,
#   "range": 30000
# }

print("Average salary: $" + stats["mean"].to_string())
print("Salary range: $" + stats["range"].to_string())
```

## Group Operations

### `group_by(df, group_column, agg_column, operation)`
Group by column and apply aggregation.
```glang
# Get total salary by department
dept_totals = df.group_by(employees, "department", "salary", "sum")

# Access results
total_eng = dept_totals["Engineering"]  # Total engineering salaries
total_mgmt = dept_totals["Management"]  # Total management salaries
```

### `group_by_dataframes(df, group_column)`
Group by column, return sub-DataFrames.
```glang
dept_groups = df.group_by_dataframes(employees, "department")

# Access individual department DataFrames
engineering_team = dept_groups["Engineering"]
df.info(engineering_team)  # Show engineering employees only
```

### `group_by_agg(df, group_column, agg_operations)`
Apply multiple aggregations per group.
```glang
# Multiple operations per group
agg_ops = {
    "salary": "mean",    # Average salary per department
    "name": "count"      # Number of employees per department
}
dept_summary = df.group_by_agg(employees, "department", agg_ops)

# Access results
eng_avg_salary = dept_summary["Engineering"]["salary"]
eng_count = dept_summary["Engineering"]["name"]
```

## Advanced Analytics Examples

### Business Intelligence Workflows
```glang
# Sales analysis pipeline
sales_df = df.from_csv(sales_data, true)

# 1. Clean and transform data
df.transform_column(sales_df, "amount", x => x > 0 ? x : 0)  # Remove negative amounts
df.normalize_column(sales_df, "date", x => x.to_date())       # Parse dates

# 2. Calculate derived metrics
df.transform_column(sales_df, "profit", x => x * 0.3)         # 30% profit margin

# 3. Filter for analysis period
q4_sales = df.filter_by(sales_df, "date", x => x.quarter() == 4)

# 4. Statistical analysis
quarterly_stats = df.compute_basic_stats(q4_sales, "amount")
print("Q4 Average Sale: $" + quarterly_stats["mean"].to_string())

# 5. Group analysis
region_performance = df.group_by(q4_sales, "region", "amount", "sum")
```

### Data Quality Workflows
```glang
# Data validation and cleaning
raw_data = df.from_csv(uploaded_file, true)

# 1. Remove invalid entries
clean_data = df.filter_by(raw_data, "email", x => x.contains("@"))
clean_data = df.filter_by(clean_data, "age", x => x > 0 && x < 120)

# 2. Standardize formats
df.transform_column(clean_data, "phone", x => x.replace("-", "").replace(" ", ""))
df.transform_column(clean_data, "name", x => x.trim().title())

# 3. Calculate quality metrics
total_records = df.aggregate(raw_data, "id", "count")
clean_records = df.aggregate(clean_data, "id", "count")
quality_rate = clean_records / total_records

print("Data quality rate: " + (quality_rate * 100).to_string() + "%")
```

### Financial Analysis
```glang
# Portfolio analysis
portfolio = df.from_column_data({
    "symbol": ["AAPL", "GOOGL", "MSFT", "AMZN"],
    "shares": [100, 50, 75, 25],
    "price": [150.00, 2800.00, 300.00, 3200.00]
})

# Calculate position values
df.transform_column(portfolio, "value", i => {
    return portfolio["shares"][i] * portfolio["price"][i]
})

# Calculate portfolio statistics
total_value = df.aggregate(portfolio, "value", "sum")
portfolio_stats = df.compute_basic_stats(portfolio, "value")

print("Portfolio value: $" + total_value.to_string())
print("Largest position: $" + portfolio_stats["max"].to_string())
print("Position range: $" + portfolio_stats["range"].to_string())
```

## Performance Tips

1. **Use lambda functions for complex logic**: More flexible than built-in predicates
2. **Chain operations efficiently**: `filter_by()` before expensive `transform_column()`
3. **Leverage statistical summaries**: `compute_basic_stats()` is faster than individual aggregations
4. **Group operations**: Use `group_by_agg()` for multiple metrics per group

## Migration from pandas

| pandas | Glang DataFrame | Notes |
|--------|----------------|--------|
| `df['column']` | `df["column"]` | Same syntax |
| `df.loc[df['col'] > 5]` | `df.filter_by(df, "col", x => x > 5)` | Lambda-powered filtering |
| `df['col'].apply(func)` | `df.transform_column(df, "col", func)` | Direct lambda application |
| `df.describe()` | `df.compute_basic_stats(df, "col")` | Statistical summary |
| `df.groupby('col').sum()` | `df.group_by(df, "col", "target", "sum")` | Group and aggregate |
| `df.head(n)` | `df.head(df, n)` | First n rows |

## Graph Architecture Benefits

Glang DataFrames leverage graph architecture for:
- **Memory efficiency**: Shared column data across operations
- **Lazy evaluation**: Operations can be optimized before execution
- **Rule-based validation**: Governance rules ensure data integrity
- **Future extensibility**: Graph traversal enables advanced analytics
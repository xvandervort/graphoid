# SQL Query Builder Module

The SQL module provides a pure Glang implementation for building SQL queries using a graph-based, lazy evaluation approach. Queries are constructed as a graph of transformations that generate SQL only when needed.

## Installation

The SQL module is part of the Glang standard library. Import it into your program:

```glang
import "sql" as q
```

## Basic Usage

### Creating Tables

Start by creating a table reference:

```glang
users = q.Table("users")
```

This doesn't execute any SQL - it just creates a query node representing the table.

### Generating SQL

Convert any query to SQL using `to_sql()`:

```glang
sql_text = q.to_sql(users)
# Returns: "SELECT * FROM users"
```

## Query Building

### Filtering (WHERE clauses)

Add conditions to filter results:

```glang
# Simple condition
active_users = q.filter(users, "active = true")
# SELECT * FROM users WHERE active = true

# Multiple conditions (AND)
admins = q.filter(active_users, "role = 'admin'")
# SELECT * FROM users WHERE active = true AND role = 'admin'

# Using the where() alias
recent = q.where(users, "created_at > '2024-01-01'")
```

### Sorting (ORDER BY)

Sort results in ascending or descending order:

```glang
# Ascending sort
sorted = q.sort(users, "name")
# SELECT * FROM users ORDER BY name ASC

# Descending sort
top_scores = q.sort_desc(users, "score")
# SELECT * FROM users ORDER BY score DESC

# Multiple sorts
by_date = q.sort(users, "created_at")
by_date_and_name = q.sort(by_date, "name")
# SELECT * FROM users ORDER BY created_at ASC, name ASC
```

### Limiting Results

Restrict the number of results:

```glang
# Using limit()
top_10 = q.limit(users, 10)
# SELECT * FROM users LIMIT 10

# Using take() alias
first_5 = q.take(users, 5)
# SELECT * FROM users LIMIT 5
```

## Condition Helpers

The SQL module provides helper functions for building type-safe conditions:

### Comparison Functions

```glang
# Equality - automatically quotes strings
q.eq("name", "Alice")        # name = 'Alice'
q.eq("age", 25)              # age = 25

# Greater than
q.gt("price", 100)           # price > 100
q.gt("date", "2024-01-01")   # date > '2024-01-01'

# Less than
q.lt("stock", 50)            # stock < 50

# Greater than or equal
q.gte("score", 90)           # score >= 90

# Less than or equal
q.lte("priority", 3)         # priority <= 3
```

### Pattern Matching

```glang
# LIKE patterns for text search
q.like("name", "%Smith%")    # name LIKE '%Smith%'
q.like("email", "%@gmail.com") # email LIKE '%@gmail.com'
```

### IN Lists

```glang
# Automatically handles type detection and quoting
q.in_list("status", ["pending", "active", "completed"])
# status IN ('pending', 'active', 'completed')

q.in_list("id", [1, 2, 3])
# id IN (1, 2, 3)

# Mixed types are handled correctly
q.in_list("value", [1, "two", 3])
# value IN (1, 'two', 3)
```

## Complex Queries

Build complex queries by chaining operations:

```glang
# Step-by-step construction
products = q.Table("products")
in_stock = q.filter(products, q.gt("quantity", 0))
electronics = q.filter(in_stock, q.eq("category", "Electronics"))
sorted = q.sort_desc(electronics, "price")
top_items = q.limit(sorted, 20)

sql = q.to_sql(top_items)
# SELECT * FROM products
# WHERE quantity > 0 AND category = 'Electronics'
# ORDER BY price DESC
# LIMIT 20
```

## Lazy Evaluation

Queries are lazy - they build a graph structure but don't generate SQL until explicitly requested:

```glang
# Create a base query
base = q.filter(q.Table("posts"), "published = true")

# Branch into different queries from the same base
recent = q.limit(q.sort_desc(base, "created_at"), 10)
popular = q.limit(q.sort_desc(base, "views"), 10)

# SQL is only generated when needed
print(q.to_sql(recent))  # Generates SQL for recent posts
print(q.to_sql(popular)) # Generates SQL for popular posts
```

## Complete Example

```glang
import "sql" as q

# Build a complex query for an e-commerce system
orders = q.Table("orders")

# Filter for recent high-value orders
recent_orders = q.filter(orders, q.gte("order_date", "2024-01-01"))
high_value = q.filter(recent_orders, q.gt("total_amount", 1000))

# Add status filter
pending = q.filter(high_value, q.in_list("status", ["pending", "processing"]))

# Sort by urgency (date) and limit
urgent = q.sort(pending, "order_date")
top_urgent = q.limit(urgent, 50)

# Generate the SQL
sql = q.to_sql(top_urgent)
print(sql)

# Output:
# SELECT * FROM orders
# WHERE order_date >= '2024-01-01'
#   AND total_amount > 1000
#   AND status IN ('pending', 'processing')
# ORDER BY order_date ASC
# LIMIT 50
```

## API Reference

### Table Creation
- `Table(name)` - Create a query node for a table

### Query Operations
- `filter(query, condition)` - Add a WHERE condition
- `where(query, condition)` - Alias for filter()
- `sort(query, field)` - Sort ascending by field
- `sort_desc(query, field)` - Sort descending by field
- `limit(query, count)` - Limit result count
- `take(query, count)` - Alias for limit()

### SQL Generation
- `to_sql(query)` - Generate SQL string from query graph

### Condition Helpers
- `eq(field, value)` - Equality comparison
- `gt(field, value)` - Greater than
- `lt(field, value)` - Less than
- `gte(field, value)` - Greater than or equal
- `lte(field, value)` - Less than or equal
- `like(field, pattern)` - LIKE pattern matching
- `in_list(field, values)` - IN list comparison

## Design Philosophy

The SQL module embraces Glang's graph-theoretic nature:

1. **Queries are Graphs** - Each operation creates a new node in a query graph
2. **Lazy Evaluation** - SQL is only generated when explicitly requested
3. **Immutable** - Operations return new queries, never modify existing ones
4. **Type-Safe** - Automatic type detection ensures proper SQL formatting
5. **Pure Glang** - No external dependencies, will survive the Rust port

## Limitations

- Currently generates SELECT statements only
- No JOIN support yet (future enhancement)
- No aggregate functions yet (COUNT, SUM, etc.)
- Read-only - no INSERT/UPDATE/DELETE generation

These limitations will be addressed as Glang's capabilities expand.

## Future Enhancements

When Glang gains additional features, the SQL module will support:

- JOIN operations as graph edges
- Aggregate functions with GROUP BY
- Subqueries as nested graphs
- INSERT/UPDATE/DELETE operations
- Direct database execution (once connectivity is available)
# DataFrames as Governed Graphs in Glang

## Core Philosophy

In Glang, **DataFrames are not special code** - they are simply graphs with specific governance rules that enforce tabular structure. This design follows Glang's fundamental principle: data structures are distinguished by their **rules**, not by their implementation.

## What Makes a DataFrame?

A DataFrame is a graph structure with five key governance rules:

### 1. Tabular Structure Rule
- Edges maintain rectangular table shape
- Cells connect only within rows and columns
- No diagonal or arbitrary connections

### 2. Column Type Consistency Rule
- Values within a column must have consistent types
- Prevents mixing strings and numbers in same column
- Enforced when adding new values

### 3. Row Integrity Rule
- All rows must have the same columns
- Missing values filled with `none`
- Ensures rectangular shape is maintained

### 4. No External Edges Rule
- DataFrame cells cannot link to nodes outside the DataFrame
- Prevents cross-contamination between structures
- Maintains data isolation

### 5. Ordered Access Pattern Rule
- Rows accessed sequentially by index (0, 1, 2...)
- Columns accessed by name
- Consistent with tabular data expectations

## Pure Glang Implementation

The DataFrame module (`stdlib/df.gr`) is implemented entirely in Glang, demonstrating that DataFrames don't require special runtime support:

```glang
import "df" as dataframes

# Create a DataFrame - just a map with governance rules
frame = dataframes.create(["name", "age", "score"])

# Add rows - governance ensures all columns present
row = {}
row["name"] = "Alice"
row["age"] = 30
row["score"] = 95
dataframes.add_row(frame, row)

# Operations preserve governance
subset = dataframes.select(frame, ["name", "score"])  # Still tabular
high_scores = dataframes.filter_positive(frame, "score")  # Still rectangular
```

## Graph Structure View

Internally, a DataFrame is a graph where:
- **Nodes** represent cells in the table
- **Edges** connect cells to their columns and rows
- **Metadata** stores column names and row indices
- **Governance** ensures tabular structure is maintained

```
DataFrame Graph Structure:
    Column Headers (nodes)
         |
    [name] [age] [score]
      |     |      |
    Alice   30     95    <- Row 0 (connected cells)
      |     |      |
     Bob    25     87    <- Row 1 (connected cells)
```

## Operations as Graph Transformations

DataFrame operations are graph transformations that preserve governance:

- **Select**: Creates subgraph with specific column edges
- **Filter**: Creates subgraph with specific row edges
- **Aggregate**: Traverses column edges to compute values
- **Join**: Merges graphs while maintaining tabular structure

## Benefits of This Approach

1. **Simplicity**: DataFrames are just graphs with rules, not special code
2. **Consistency**: All Glang structures follow the same graph model
3. **Flexibility**: Rules can be relaxed for specialized use cases
4. **Self-Hosting**: Implementation in pure Glang reduces Python dependency
5. **Future-Proof**: Rules translate directly to Rust implementation

## Governance in Action

The control layer actively prevents operations that would violate DataFrame semantics:

```glang
# These would be BLOCKED by governance:

# ❌ Adding row with wrong columns - blocked by row_integrity
# ❌ Mixing types in column - blocked by column_consistency
# ❌ Creating edge to external node - blocked by no_external_edges
# ❌ Non-rectangular structure - blocked by tabular_structure
```

## Usage Example

See `samples/dataframe_demo.gr` for a complete example:

```bash
glang samples/dataframe_demo.gr
```

## Future Enhancements

As Glang evolves, DataFrames will gain additional capabilities:
- Native graph syntax for DataFrame operations
- Automatic type inference for columns
- Parallel processing through graph traversal
- Distributed DataFrames across multiple nodes

## Key Takeaway

DataFrames in Glang demonstrate the power of the **"graphs with governance"** paradigm. By defining data structures through rules rather than implementation, Glang achieves both simplicity and power, allowing the same underlying graph engine to support lists, maps, trees, and now DataFrames - all distinguished only by their governance rules.
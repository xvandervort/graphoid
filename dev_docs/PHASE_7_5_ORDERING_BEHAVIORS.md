# Sub-Phase 7.5: Ordering Behaviors (Days 8-9)

## Goal

Implement ordering behaviors that maintain sorted order for any ordered collection (lists, and potentially trees/graphs with ordering semantics). This is NOT BST-specific - it's a general-purpose mechanism for maintaining order.

## Concept

```graphoid
# Default ordering (uses natural comparison)
numbers = [3, 1, 4, 1, 5, 9]
numbers.add_rule(:maintain_order)
# Retroactive: existing values sorted → [1, 1, 3, 4, 5, 9]
numbers.append(2)  # Proactive: inserted in position → [1, 1, 2, 3, 4, 5, 9]

# Custom comparison function
people = [
    {"name": "Bob", "age": 30},
    {"name": "Alice", "age": 25},
    {"name": "Charlie", "age": 35}
]
people.add_rule(:ordering, fn(a, b) { a["age"] < b["age"] })
# Sorted by age: Alice (25), Bob (30), Charlie (35)

# Case-insensitive string sorting
words = ["apple", "Banana", "cherry", "Date"]
words.add_rule(:ordering, fn(a, b) { a.lower() < b.lower() })
# ["apple", "Banana", "cherry", "Date"]
```

## Design Decisions

### 1. Default Comparison

**For numbers**: Use numeric comparison (`<`)
**For strings**: Use lexicographic comparison
**For booleans**: false < true
**For mixed types**: Error (incomparable)
**For none**: Treat as smallest value (sorts first)

### 2. Insertion Strategy

**Retroactive** (when behavior is added):
- Sort all existing elements using stable sort
- Preserve relative order of equal elements

**Proactive** (when new element is added):
- Use binary search to find insertion point (O(log n))
- Insert at correct position to maintain order
- Shift subsequent elements

### 3. Scope

**Applies to**:
- Lists (primary use case)
- Trees (determines left/right child placement - future enhancement)
- Graphs with "next" edges (maintains topological order - future enhancement)

**Phase 7 Scope**: Lists only. Tree/graph ordering deferred.

## Deliverables

### 1. Ordering Behavior Implementation

**Add to `src/graph/behaviors.rs`**:

```rust
/// Maintains sorted order using comparison function
/// Retroactive: sorts existing elements
/// Proactive: inserts new elements in sorted position
#[derive(Debug, Clone)]
struct OrderingBehavior {
    compare_fn: Option<Function>,  // None = default comparison
}

impl Behavior for OrderingBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        // Ordering doesn't transform individual values
        // It transforms the collection structure
        // This should not be called for ordering behavior
        Ok(value.clone())
    }

    fn name(&self) -> &str {
        "ordering"
    }

    fn applies_to(&self, _value: &Value) -> bool {
        // Applies to all values (ordering is collection-level)
        true
    }
}

// Special handling: ordering is a COLLECTION behavior, not a VALUE behavior
// It needs special treatment in the executor
```

### 2. Special Executor Handling

**Ordering is different**: It doesn't transform individual values - it transforms collection structure.

**Add to `src/execution/executor.rs`**:

```rust
"add_rule" => {
    // Check if this is an ordering behavior
    if let Value::Symbol(sym) = &args[0] {
        if sym == "ordering" || sym == "maintain_order" {
            // Special handling for ordering behavior
            return self.handle_ordering_rule(&mut list, args, object_expr);
        }
    }

    // Normal behavior handling...
}

fn handle_ordering_rule(
    &mut self,
    list: &mut List,
    args: &[Value],
    object_expr: &Expr
) -> Result<Value, GraphoidError> {
    // Parse comparison function (if provided)
    let compare_fn = if args.len() == 2 {
        match &args[1] {
            Value::Function(f) => Some(f.clone()),
            other => {
                return Err(GraphoidError::type_error("function", other.type_name()));
            }
        }
    } else if args.len() == 1 {
        None  // Use default comparison
    } else {
        return Err(GraphoidError::runtime(
            "ordering rule expects 0 or 1 argument (optional comparison function)"
        ));
    };

    // Retroactive: sort existing elements
    self.sort_list_with_comparator(list, compare_fn.as_ref())?;

    // Store ordering behavior for proactive application
    let behavior_spec = BehaviorSpec::Ordering {
        compare_fn: compare_fn.map(Value::Function),
    };
    let behavior_instance = BehaviorInstance::new(behavior_spec);
    list.behaviors.push(behavior_instance);

    // Update environment
    if let Expr::Variable { name, .. } = object_expr {
        self.env.set(name, Value::List(list.clone()))?;
    }

    Ok(Value::None)
}

fn sort_list_with_comparator(
    &mut self,
    list: &mut List,
    compare_fn: Option<&Function>
) -> Result<(), GraphoidError> {
    let elements = list.elements();

    if elements.is_empty() {
        return Ok(());
    }

    // Sort elements
    let mut sorted = elements.clone();

    if let Some(cmp_fn) = compare_fn {
        // Custom comparison
        sorted.sort_by(|a, b| {
            // Call comparison function: fn(a, b) -> bool
            // Returns true if a < b
            match self.call_function(cmp_fn, &[a.clone(), b.clone()]) {
                Ok(Value::Boolean(true)) => std::cmp::Ordering::Less,
                Ok(Value::Boolean(false)) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            }
        });
    } else {
        // Default comparison
        sorted.sort_by(|a, b| self.default_compare(a, b));
    }

    // Rebuild list with sorted elements
    // Clear current list
    list.graph.nodes.clear();
    list.graph.edges.clear();

    // Add sorted elements back
    for (i, element) in sorted.iter().enumerate() {
        let node_id = format!("node_{}", i);
        list.graph.add_node(node_id.clone(), element.clone())?;

        // Add "next" edge to next element
        if i > 0 {
            let prev_id = format!("node_{}", i - 1);
            list.graph.add_edge(&prev_id, &node_id, "next".to_string(), HashMap::new())?;
        }
    }

    Ok(())
}

fn default_compare(&self, a: &Value, b: &Value) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    match (a, b) {
        // None sorts first
        (Value::None, Value::None) => Ordering::Equal,
        (Value::None, _) => Ordering::Less,
        (_, Value::None) => Ordering::Greater,

        // Numbers
        (Value::Number(x), Value::Number(y)) => {
            if x < y {
                Ordering::Less
            } else if x > y {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }

        // Strings
        (Value::String(x), Value::String(y)) => x.cmp(y),

        // Booleans (false < true)
        (Value::Boolean(x), Value::Boolean(y)) => x.cmp(y),

        // Mixed types: error in practice, but for sorting, use type names
        _ => {
            let type_a = a.type_name();
            let type_b = b.type_name();
            type_a.cmp(type_b)
        }
    }
}
```

### 3. Proactive Insertion (Maintain Order on Append)

**Modify `List::append()` handling in executor**:

```rust
"append" => {
    // Check if list has ordering behavior
    let has_ordering = list.behaviors.iter().any(|b| matches!(b.spec, BehaviorSpec::Ordering { .. }));

    if has_ordering {
        // Don't just append - insert in sorted position
        let new_value = args[0].clone();

        // Find insertion point using binary search
        let elements = list.elements();
        let insert_pos = self.find_insertion_point(&elements, &new_value, &list.behaviors)?;

        // Insert at position
        list.insert(insert_pos, new_value)?;
    } else {
        // Normal append
        list.append(args[0].clone())?;
    }

    // Update environment...
}

fn find_insertion_point(
    &mut self,
    elements: &[Value],
    new_value: &Value,
    behaviors: &[BehaviorInstance]
) -> Result<usize, GraphoidError> {
    // Find the ordering behavior
    let ordering_behavior = behaviors.iter()
        .find(|b| matches!(b.spec, BehaviorSpec::Ordering { .. }))
        .unwrap();

    let compare_fn = if let BehaviorSpec::Ordering { compare_fn } = &ordering_behavior.spec {
        compare_fn.as_ref()
    } else {
        unreachable!()
    };

    // Binary search for insertion point
    let mut left = 0;
    let mut right = elements.len();

    while left < right {
        let mid = (left + right) / 2;

        let is_less = if let Some(Value::Function(cmp_fn)) = compare_fn {
            // Custom comparison: cmp_fn(new_value, elements[mid])
            match self.call_function(cmp_fn, &[new_value.clone(), elements[mid].clone()])? {
                Value::Boolean(b) => b,
                _ => false,
            }
        } else {
            // Default comparison
            matches!(self.default_compare(new_value, &elements[mid]), std::cmp::Ordering::Less)
        };

        if is_less {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    Ok(left)
}
```

## Test Strategy (TDD)

### Test File: `tests/behavior_ordering_tests.rs`

**Tests to Write FIRST**:

**Default Ordering** (5 tests):
1. `test_ordering_numbers_default()` - Sort numbers ascending
2. `test_ordering_strings_default()` - Sort strings lexicographically
3. `test_ordering_mixed_with_none()` - None sorts first
4. `test_ordering_retroactive()` - Existing values sorted
5. `test_ordering_proactive_insertion()` - New values inserted in position

**Custom Ordering** (4 tests):
6. `test_ordering_custom_comparator()` - Custom comparison function
7. `test_ordering_descending()` - Reverse order (fn(a, b) { a > b })
8. `test_ordering_by_property()` - Sort objects by property
9. `test_ordering_case_insensitive()` - Case-insensitive string sort

**Edge Cases** (3 tests):
10. `test_ordering_empty_list()` - No effect on empty list
11. `test_ordering_single_element()` - Single element unchanged
12. `test_ordering_stable_sort()` - Equal elements preserve relative order

**Total**: 12 tests

## Implementation Steps

1. Write all 12 tests FIRST (red phase)
2. Add BehaviorSpec::Ordering variant (already done in 7.1)
3. Implement handle_ordering_rule() in executor
4. Implement sort_list_with_comparator() in executor
5. Implement default_compare() in executor
6. Implement find_insertion_point() for proactive insertion
7. Modify append/insert handling to respect ordering
8. Run tests - should pass (green phase)
9. Refactor for performance and clarity
10. Zero warnings

## Acceptance Criteria

- ✅ 12 tests passing
- ✅ Default ordering works for numbers, strings, booleans, none
- ✅ Custom comparison functions work
- ✅ Retroactive sorting works (existing values sorted)
- ✅ Proactive insertion works (new values inserted in sorted position)
- ✅ Binary search for insertion point (O(log n) lookup)
- ✅ Stable sort (equal elements preserve order)
- ✅ Works with other behaviors (e.g., ordering + none_to_zero)
- ✅ Zero compiler warnings

## Performance Notes

**Retroactive sorting**: O(n log n) using Rust's stable sort
**Proactive insertion**: O(log n) binary search + O(n) shift = O(n) total
**Trade-off**: Maintaining order has overhead. Users can choose:
- Bulk load, then add ordering rule (one sort)
- Add ordering rule early (pay per-insert cost)

## Future Enhancements (Post-Phase 7)

**Tree Ordering** (Phase 11+):
- For BSTs: ordering determines left/right child placement
- compare_fn(new_value, parent_value) determines which child

**Graph Ordering** (Phase 11+):
- Topological ordering for DAGs
- Custom edge-based ordering

**List Performance** (Phase 11+):
- Skip list data structure for O(log n) insertion
- Lazy sorting (sort on read, not on write)

---

**This completes Sub-Phase 7.5 specification!**

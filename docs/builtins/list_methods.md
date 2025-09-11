# List Methods

List is a built-in collection type in Glang representing ordered sequences of values. Lists can be homogeneous (type-constrained) or heterogeneous (mixed types).

## Type Information

### type()
Returns the type of the value.
```glang
items = [1, 2, 3]
items.type()  # Returns "list"
```

### methods()
Returns a list of all available methods for lists.
```glang
items = [1, 2, 3]
items.methods()  # Returns ["type", "methods", "can", "inspect", "size", "append", "prepend", ...]
```

### can(method_name)
Checks if a method is available on the list.
```glang
items = [1, 2, 3]
items.can("append")  # Returns true
items.can("invalid")  # Returns false
```

### inspect()
Returns detailed information about the list.
```glang
items = [1, 2, 3]
items.inspect()  # Returns detailed list information
```

## Size and State

### size()
Returns the number of elements in the list.
```glang
items = [1, 2, 3, 4, 5]
items.size()  # Returns 5
```

### empty()
Checks if the list is empty.
```glang
[].empty()  # Returns true
[1, 2, 3].empty()  # Returns false
```

## Type Constraints

### constraint()
Returns the type constraint of the list, if any.
```glang
list<num> scores = [95, 87, 92]
scores.constraint()  # Returns "num"

mixed = [1, "hello", true]
mixed.constraint()  # Returns none
```

### validate_constraint()
Validates that all elements match the constraint.
```glang
list<num> scores = [95, 87, 92]
scores.validate_constraint()  # Returns true
```

### type_summary()
Returns a summary of types in the list.
```glang
mixed = [1, "hello", true, 2, "world"]
mixed.type_summary()  # Returns {"num": 2, "string": 2, "bool": 1}
```

### types()
Returns unique types present in the list.
```glang
mixed = [1, "hello", true, 2]
mixed.types()  # Returns ["num", "string", "bool"]
```

### coerce_to_constraint(type)
Attempts to convert all elements to the specified type.
```glang
strings = ["1", "2", "3"]
strings.coerce_to_constraint("num")  # Returns [1, 2, 3]
```

## Adding Elements

### append(element)
Adds an element to the end of the list.
```glang
items = [1, 2, 3]
items.append(4)  # items becomes [1, 2, 3, 4]
```

### prepend(element)
Adds an element to the beginning of the list.
```glang
items = [2, 3, 4]
items.prepend(1)  # items becomes [1, 2, 3, 4]
```

### insert(index, element)
Inserts an element at the specified index.
```glang
items = [1, 2, 4, 5]
items.insert(2, 3)  # items becomes [1, 2, 3, 4, 5]
```

## Removing Elements

### remove(element)
Removes the first occurrence of an element.
```glang
items = [1, 2, 3, 2, 4]
items.remove(2)  # items becomes [1, 3, 2, 4]
```

### pop()
Removes and returns the last element.
```glang
items = [1, 2, 3, 4]
last = items.pop()  # last is 4, items becomes [1, 2, 3]
```

### pop(index)
Removes and returns the element at the specified index.
```glang
items = [1, 2, 3, 4]
element = items.pop(1)  # element is 2, items becomes [1, 3, 4]
```

### clear()
Removes all elements from the list.
```glang
items = [1, 2, 3, 4]
items.clear()  # items becomes []
```

## Searching and Counting

### indexOf(element)
Returns the index of the first occurrence of an element.
```glang
items = [10, 20, 30, 20, 40]
items.indexOf(20)  # Returns 1
items.indexOf(50)  # Returns -1 (not found)
```

### count(element)
Counts occurrences of an element.
```glang
items = [1, 2, 3, 2, 2, 4]
items.count(2)  # Returns 3
```

### count_values(value)
Counts occurrences of a value (alias for count).
```glang
items = ["a", "b", "a", "c", "a"]
items.count_values("a")  # Returns 3
```

## Ordering and Sorting

### reverse()
Reverses the list in place.
```glang
items = [1, 2, 3, 4]
items.reverse()  # items becomes [4, 3, 2, 1]
```

### sort()
Sorts the list in ascending order.
```glang
items = [3, 1, 4, 1, 5, 9]
items.sort()  # items becomes [1, 1, 3, 4, 5, 9]
```

## Mathematical Operations (for numeric lists)

### min()
Returns the minimum value in the list.
```glang
numbers = [3, 1, 4, 1, 5, 9]
numbers.min()  # Returns 1
```

### max()
Returns the maximum value in the list.
```glang
numbers = [3, 1, 4, 1, 5, 9]
numbers.max()  # Returns 9
```

### sum()
Returns the sum of all elements.
```glang
numbers = [1, 2, 3, 4, 5]
numbers.sum()  # Returns 15
```

## Functional Programming

### map(transformation)
Applies a transformation to each element.
```glang
numbers = [1, 2, 3, 4]
numbers.map("double")  # Returns [2, 4, 6, 8]
numbers.map("square")  # Returns [1, 4, 9, 16]

names = ["alice", "bob"]
names.map("upper")  # Returns ["ALICE", "BOB"]
```

Available transformations:
- Numeric: `double`, `square`, `negate`, `increment`, `decrement`
- String: `upper`, `lower`, `trim`, `reverse`
- Type conversion: `to_string`, `to_num`, `to_bool`

### filter(predicate)
Filters elements based on a predicate.
```glang
numbers = [1, 2, 3, 4, 5, 6]
numbers.filter("even")  # Returns [2, 4, 6]
numbers.filter("positive")  # Returns [1, 2, 3, 4, 5, 6]
```

Available predicates:
- Numeric: `positive`, `negative`, `zero`, `even`, `odd`
- String/Collection: `empty`, `non_empty`, `uppercase`, `lowercase`
- Type checks: `is_string`, `is_number`, `is_bool`, `is_list`
- General: `truthy`, `falsy`

### select(predicate)
Alias for filter. Selects elements matching the predicate.
```glang
numbers = [1, 2, 3, 4, 5]
numbers.select("odd")  # Returns [1, 3, 5]
```

### reject(predicate)
Returns elements that don't match the predicate.
```glang
numbers = [1, 2, 3, 4, 5, 6]
numbers.reject("even")  # Returns [1, 3, 5]
```

### each(action)
Applies an action to each element (for side effects).
```glang
names = ["alice", "bob", "charlie"]
names.each("print")  # Prints each name
```

## Type Conversion

### to_string()
Converts the list to a string representation.
```glang
items = [1, 2, 3]
items.to_string()  # Returns "[1, 2, 3]"
```

### to_bool()
Converts the list to a boolean. Empty lists are false, non-empty are true.
```glang
[1, 2, 3].to_bool()  # Returns true
[].to_bool()  # Returns false
```

## Immutability Methods

### freeze()
Makes the list and all its elements immutable. Returns self for chaining.
```glang
items = [1, 2, 3]
items.freeze()
items.is_frozen()  # Returns true
# items.append(4)  # Would throw error
```

### is_frozen()
Checks if the list is frozen (immutable).
```glang
items = [1, 2, 3]
items.freeze()
items.is_frozen()  # Returns true
```

### contains_frozen()
Checks if the list contains any frozen elements.
```glang
items = [1, 2, 3]
frozen_item = "hello"
frozen_item.freeze()
items.append(frozen_item)  # Error: can't mix frozen/unfrozen
```

### can_accept(value)
Checks if a value can be added to the list (respects frozen state).
```glang
items = [1, 2, 3]
frozen_value = 4
frozen_value.freeze()
items.can_accept(frozen_value)  # Returns false (can't mix)
```

## List Arithmetic

Lists support arithmetic operations with scalars and other lists:

### Scalar Addition
```glang
numbers = [1, 2, 3]
result = numbers + 5  # Returns [6, 7, 8]
```

### Scalar Multiplication
```glang
numbers = [1, 2, 3]
result = numbers * 2  # Returns [2, 4, 6]
```

### List Concatenation
```glang
list1 = [1, 2, 3]
list2 = [4, 5, 6]
result = list1 + list2  # Returns [1, 2, 3, 4, 5, 6]
```

### Element-wise Operations (dot operators)
```glang
list1 = [1, 2, 3]
list2 = [4, 5, 6]
result = list1 +. list2  # Returns [5, 7, 9] (element-wise addition)
result = list1 *. list2  # Returns [4, 10, 18] (element-wise multiplication)
```

## Indexing and Slicing

### Index Access
```glang
items = [10, 20, 30, 40, 50]
items[0]  # Returns 10
items[2]  # Returns 30
items[-1]  # Returns 50 (last element)
```

### Index Assignment
```glang
items = [10, 20, 30]
items[1] = 25  # items becomes [10, 25, 30]
```

### Slice Access
```glang
items = [1, 2, 3, 4, 5]
items[1:3]  # Returns [2, 3]
items[2:]  # Returns [3, 4, 5]
items[:3]  # Returns [1, 2, 3]
items[::2]  # Returns [1, 3, 5] (every 2nd element)
```

## Examples

### Processing Numeric Data
```glang
scores = [95, 87, 92, 76, 88, 91]

# Find statistics
average = scores.sum() / scores.size()
highest = scores.max()
lowest = scores.min()

# Filter and transform
high_scores = scores.filter(s => s > 90)
curved_scores = scores.map(s => s + 5)
```

### Type-Constrained Lists
```glang
# Enforce numeric elements only
list<num> temperatures = [72.5, 68.2, 75.1]
temperatures.append(70.0)  # OK
# temperatures.append("hot")  # Would throw error

# Validate constraint
if temperatures.validate_constraint() {
    print("All temperatures are numbers")
}
```

### Functional Pipeline
```glang
# Complex data processing
data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
result = data.filter("even")
             .map("square")
             .filter(x => x > 10)
             .sort()
print(result)  # [16, 36, 64, 100]
```

### List Building
```glang
# Build a list dynamically
results = []
for i in [1, 2, 3, 4, 5] {
    if i % 2 == 0 {
        results.append(i * i)
    }
}
print(results)  # [4, 16]
```

### Immutable Data Structures
```glang
# Create immutable configuration
config_values = [8080, "localhost", true]
config_values.freeze()

# Safe to share - cannot be modified
process_config(config_values)
```
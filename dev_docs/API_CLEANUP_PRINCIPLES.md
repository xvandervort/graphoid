# API Cleanup Principles and Implementation Guide

## Session Summary: List Visualization & Names API Cleanup

### Core Problems Identified
1. **Visualization method confusion** - `inspect` was useless, `visualize_structure` was doing too much
2. **UUID pollution** - Incomprehensible UUIDs in visualization output instead of meaningful values
3. **Missing name display** - Names weren't shown in visualization methods
4. **Setter/getter semantic garbage** - Verbose `set_names()` instead of clean assignment-style syntax
5. **Method list inconsistencies** - Listed methods that didn't work, inconsistent naming patterns
6. **API inconsistency** - Mix of `count_X` and `X_count` patterns

### Fixes Applied

#### 1. Visualization Method Hierarchy Redesign
- **`inspect`**: Deep technical details (graph structure, rules, connections with names)
- **`visualize`**: Quick shape overview (`[red → orange → yellow]` or abbreviated for large structures)
- **`view`**: Clean semantic display (`["red": 1, "orange": 2, "yellow": 3]`)

#### 2. UUID Elimination
- **Before**: `3584cf35-a077... → a10df7ff-984e... (0)`
- **After**: `red (1) → orange (2) (0)` or just `red → orange` in shape view

#### 3. Name Integration
- All visualization methods now show names when available
- Fallback to values when names not set
- Consistent name-first, value-in-parentheses format for detailed views

#### 4. Setter/Getter Elimination
- **Before**: `a.set_names(['red', 'orange', 'yellow'])` (verbose OOP pollution)
- **After**: `a.names(['red', 'orange', 'yellow'])` (clean overloaded method)
- **Get**: `a.names` or `a.names()`
- **Set**: `a.names(list)`

#### 5. Method Consistency
- Standardized on `count_X` pattern (action_object: `count_edges`, `count_nodes`)
- Removed `X_count` variants (`edge_count`, `node_count`)
- Eliminated redundant methods from all registries

#### 6. Method List Integrity
- Removed all non-working methods from lists
- Added all new methods to both executor and semantic analyzer
- Verified every listed method actually works

## Core Principles for Future Cleanup

### Principle 1: Eliminate Setter/Getter Pollution
**Problem**: Traditional OOP setters/getters are verbose semantic garbage
```glang
// BAD - Verbose OOP pollution
obj.set_property(value)
obj.get_property()

// GOOD - Clean overloaded methods
obj.property()        // get
obj.property(value)   // set
```

**Implementation**: Modify method handlers to accept 0 or 1 arguments, branch on `len(args)`

### Principle 2: Consistent Naming Patterns
**Problem**: Mix of `action_object` and `object_action` patterns causes confusion
```glang
// BAD - Inconsistent patterns
count_edges()  vs  edge_count()
get_names()    vs  names()

// GOOD - Consistent action_object pattern
count_edges()
count_nodes()
```

**Implementation**: Pick one pattern and eliminate all variants from method registries

### Principle 3: Method List Integrity
**Problem**: Listed methods that don't work destroy user trust
```glang
a.methods  // Shows methods that throw "not found" errors
```

**Implementation**:
1. Update both `executor.py` method registry AND `semantic/analyzer.py` valid methods
2. Test every method after changes
3. Remove methods from lists BEFORE removing implementations

### Principle 4: Meaningful Visualization Hierarchy
**Problem**: Single visualization method trying to do everything
```glang
// BAD - One method for everything
visualize_structure()  // Too detailed for quick overview
```

**Implementation**:
- **`inspect`**: Maximum detail for debugging (structure, rules, connections, metadata)
- **`visualize`**: Quick shape/flow overview (arrows, names if available, abbreviated for large data)
- **`view`**: Clean semantic content display (names with values, readable format)

### Principle 5: Names Integration
**Problem**: Names exist but aren't shown in outputs
```glang
// BAD - Names set but not displayed
a.names(['red', 'orange', 'yellow'])
a.visualize  // Shows: [1 → 2 → 3] (ignores names!)
```

**Implementation**:
1. Check for names availability with `hasattr(target, 'has_names') and target.has_names()`
2. Create index mapping: `node_id_to_index` for efficient lookup
3. Prefer names over values: `get_display_name(index, elem)` helper function
4. Use `name (value)` format for detailed views, `name` only for shape views

### Principle 6: UUID Elimination
**Problem**: Internal UUIDs leak into user-facing output
```glang
// BAD - Incomprehensible technical garbage
3584cf35-a077-4594-a954-44937ddda58f

// GOOD - Meaningful user values
red (1)  or  red
```

**Implementation**: Always use `node.value.to_display_string()` or names, never raw `node_id`

## Implementation Checklist for Hash/Binary Tree Cleanup

### Phase 1: Analysis
- [ ] Check current visualization methods and their output quality
- [ ] Identify setter/getter patterns (`set_X`, `get_X`)
- [ ] Find naming inconsistencies (`X_count` vs `count_X`)
- [ ] Test method lists for broken methods
- [ ] Document current UUID pollution locations

### Phase 2: Method Hierarchy
- [ ] Fix `inspect` to show meaningful structural details
- [ ] Implement `visualize` for shape overview (tree structure, hash key flow)
- [ ] Implement `view` for clean names+values display
- [ ] Ensure names integration in all visualization methods

### Phase 3: API Cleanup
- [ ] Convert setter/getters to overloaded methods
- [ ] Standardize naming patterns (choose `count_X` consistently)
- [ ] Remove redundant methods from both executor and semantic analyzer
- [ ] Update method registries

### Phase 4: Testing & Verification
- [ ] Test every method in `.methods` list works
- [ ] Verify removed methods actually fail
- [ ] Test name display in all visualization methods
- [ ] Confirm no UUID pollution in user-facing output

## Files Modified (Reference for Future Sessions)

### Core Implementation Files
- `src/glang/execution/executor.py` - Method handlers and registries
- `src/glang/semantic/analyzer.py` - Valid method lists
- `src/glang/execution/control_layer.py` - Visualization logic

### Key Sections Modified
- Method registry in `_get_available_methods()`
- Individual method handlers (search for `elif method_name ==`)
- Visualization logic in `_visualize_text()` method
- Name mapping logic for node index resolution

### Testing Approach
```glang
// Test pattern for verification
a = [1, 2, 3]
a.names(['alpha', 'beta', 'gamma'])
a.inspect    // Detailed structure
a.visualize  // Shape: [alpha → beta → gamma]
a.view       // Semantic: ["alpha": 1, "beta": 2, "gamma": 3]
a.methods    // Verify all listed methods work
```

## Success Metrics
1. **Method List Integrity**: Every listed method works, no false promises
2. **No UUID Pollution**: All user-facing output shows meaningful values/names
3. **Clean API**: No verbose setters/getters, consistent naming patterns
4. **Visualization Clarity**: Clear hierarchy of detail levels
5. **Name Integration**: Names displayed consistently across all methods

## Next Session Target Structures
- **Hash structures**: Key-value visualization, hash-specific naming patterns
- **Binary tree structures**: Tree visualization, node hierarchy display

These principles ensure consistent, clean APIs that respect user intent and eliminate semantic garbage from traditional OOP patterns.
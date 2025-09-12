# Glang REPL Syntax Improvements Plan

## Overview

Transform glang from command-based REPL to language-like syntax for better usability and language feel.

## Current State Analysis

### Current Syntax Issues
```bash
# Current (command-based)
glang> create fruits [apple, banana, cherry]
glang> show fruits              # Explicit command needed
glang> append orange            # Works on "current" graph (confusing)
glang> traverse                 # Implicit target
```

**Problems:**
1. **Implicit targeting** - `append` works on "current" graph, not explicit
2. **Command-based** - Feels like CLI tools, not programming language  
3. **Verbose** - Need `show` command to see variable contents
4. **Inconsistent** - Mix of explicit (`show fruits`) and implicit (`append`) targeting

### Desired Language-Like Syntax
```bash
# Proposed (language-like)
glang> list fruits = [guava, pineapple, 'golden delicious']
glang> fruits                   # Just typing name shows contents
[guava, pineapple, golden delicious]
glang> fruits.append 'wheat toast'
glang> fruits
[guava, pineapple, golden delicious, wheat toast]
```

## Proposed Changes

### 1. Variable Declaration Syntax

#### Current vs Proposed
```bash
# Current
create fruits [apple, banana, cherry]
create numbers [1, 2, 3, 4, 5]
create empty directed

# Proposed
list fruits = [apple, banana, cherry]
list numbers = [1, 2, 3, 4, 5] 
graph empty = directed()        # or: directed empty = ()
```

#### Syntax Rules
- **Format**: `<type> <name> = <initializer>`
- **Types**: `list`, `tree`, `graph`, `directed`, `weighted`, etc.
- **Name**: Valid identifier (letters, numbers, underscore)
- **Initializer**: 
  - List syntax: `[value1, value2, ...]`
  - Empty constructor: `()` or `directed()`, `tree()`, etc.

### 2. Method Call Syntax

#### Proposed Method Syntax
```bash
# Object.method pattern
fruits.append 'new_item'
fruits.prepend 'first_item'  
fruits.insert 2 'middle_item'
fruits.reverse()
fruits.delete 1
```

#### Method Categories
- **Mutating methods**: `append`, `prepend`, `insert`, `reverse`, `delete`
- **Query methods**: `size()`, `empty()`, `find 'item'`
- **Conversion methods**: `to_tree()`, `to_directed()`

### 3. Variable Display

#### Automatic Display
```bash
glang> fruits                   # Just typing name shows contents
[apple, banana, cherry]

glang> fruits --show-nodes      # Flag for detailed view
Graph 'fruits' (linear):
Node(a1b2c3..., data=apple) -> Node(d4e5f6..., data=banana) -> Node(g7h8i9..., data=cherry)
```

#### Display Modes
- **Default**: Simple list-like display `[item1, item2, item3]`
- **Detailed**: Node-level graph display (with `--show-nodes` or similar)
- **Meta**: Variable info with `fruits --info`

### 4. Backward Compatibility

#### Dual Syntax Support
```bash
# New syntax (preferred)
list fruits = [apple, banana]
fruits.append cherry

# Legacy syntax (still works)
create fruits [apple, banana]  
append cherry
show fruits
```

## Implementation Strategy

### Phase 1: Parser Enhancement

#### 1.1 Input Classification
```python
class InputType(Enum):
    VARIABLE_DECLARATION = auto()  # list fruits = [...]
    METHOD_CALL = auto()           # fruits.append value
    VARIABLE_ACCESS = auto()       # fruits
    LEGACY_COMMAND = auto()        # create fruits [...]
```

#### 1.2 Syntax Parser
```python
class SyntaxParser:
    def parse_input(self, input_str: str) -> ParsedCommand:
        if self._is_variable_declaration(input_str):
            return self._parse_declaration(input_str)
        elif self._is_method_call(input_str):
            return self._parse_method_call(input_str)
        elif self._is_variable_access(input_str):
            return self._parse_variable_access(input_str)
        else:
            return self._parse_legacy_command(input_str)
    
    def _parse_declaration(self, input_str: str) -> VariableDeclaration:
        # Parse: list fruits = [apple, banana, cherry]
        # Extract: type='list', name='fruits', values=['apple', 'banana', 'cherry']
        
    def _parse_method_call(self, input_str: str) -> MethodCall:
        # Parse: fruits.append 'new_item'
        # Extract: variable='fruits', method='append', args=['new_item']
```

### Phase 2: Display System

#### 2.1 Display Modes
```python
class DisplayMode(Enum):
    SIMPLE = auto()     # [item1, item2, item3]
    DETAILED = auto()   # Node -> Node -> Node  
    META = auto()       # Variable info

class GraphRenderer:
    def render(self, graph: Graph, mode: DisplayMode = DisplayMode.SIMPLE) -> str:
        if mode == DisplayMode.SIMPLE:
            return self._render_simple(graph)
        elif mode == DisplayMode.DETAILED:  
            return self._render_detailed(graph)
        elif mode == DisplayMode.META:
            return self._render_meta(graph)
```

#### 2.2 Variable Access Handler
```python
class VariableAccessHandler:
    def handle_variable_access(self, name: str, flags: List[str] = None) -> str:
        graph = self.graph_manager.get_variable(name)
        if not graph:
            return f"Variable '{name}' not found"
            
        mode = DisplayMode.SIMPLE
        if '--show-nodes' in flags:
            mode = DisplayMode.DETAILED
        elif '--info' in flags:
            mode = DisplayMode.META
            
        return self.renderer.render(graph, mode)
```

### Phase 3: Method System

#### 3.1 Method Dispatch
```python
class MethodDispatcher:
    def dispatch_method(self, variable_name: str, method_name: str, args: List[str]) -> str:
        graph = self.graph_manager.get_variable(variable_name)
        if not graph:
            return f"Variable '{variable_name}' not found"
            
        if not hasattr(self, f'_method_{method_name}'):
            return f"Method '{method_name}' not supported"
            
        method = getattr(self, f'_method_{method_name}')
        return method(graph, variable_name, args)
    
    def _method_append(self, graph: Graph, var_name: str, args: List[str]) -> str:
        if not graph.graph_type.is_linear():
            return f"append() only works on linear graphs"
        if not args:
            return f"append requires a value"
        graph.append(args[0])
        return f"Appended {args[0]} to {var_name}"
```

### Phase 4: Enhanced Tab Completion

#### 4.1 Context-Aware Completion
```python
class EnhancedCompletion:
    def get_completions(self, text: str, line: str) -> List[str]:
        if self._is_declaring_type(line):
            return ['list', 'tree', 'graph', 'directed', 'weighted']
        elif self._is_method_context(line):
            return self._get_method_completions(line, text)
        elif self._is_variable_context(line):
            return self._get_variable_completions(text)
        else:
            return self._get_legacy_completions(text, line)
    
    def _get_method_completions(self, line: str, text: str) -> List[str]:
        # Complete: fruits.app<Tab> -> fruits.append
        var_name = self._extract_variable_name(line)
        graph = self.graph_manager.get_variable(var_name)
        if graph and graph.graph_type.is_linear():
            methods = ['append', 'prepend', 'insert', 'reverse', 'delete', 'size', 'empty']
            return [m for m in methods if m.startswith(text)]
        return []
```

## Implementation Files

### New Files to Create
```
src/glang/parser/
├── __init__.py
├── syntax_parser.py      # Main parsing logic
├── ast_nodes.py         # AST node definitions  
└── tokenizer.py         # Basic tokenization

src/glang/display/
├── __init__.py
├── renderer.py          # Display mode handling
└── formatters.py        # Format-specific rendering

src/glang/methods/
├── __init__.py
├── dispatcher.py        # Method dispatch system
├── linear_methods.py    # Methods for linear graphs
└── graph_methods.py     # Methods for general graphs
```

### Files to Modify
```
src/glang/repl/repl.py           # Main input processing
src/glang/repl/graph_manager.py  # Add method dispatch support
test/                            # Comprehensive syntax tests
```

## Testing Strategy

### Test Categories
1. **Syntax Parsing Tests**
   - Variable declarations: `list fruits = [apple, banana]`
   - Method calls: `fruits.append cherry`  
   - Variable access: `fruits`
   - Edge cases and error handling

2. **Display System Tests**
   - Simple mode rendering
   - Detailed mode rendering  
   - Flag parsing and handling

3. **Method Dispatch Tests**
   - All supported methods
   - Error handling for unknown methods
   - Type checking (linear vs general graphs)

4. **Integration Tests**  
   - Full workflow: declare → modify → display
   - Mixed syntax usage (new + legacy)
   - Tab completion with new syntax

5. **Backward Compatibility Tests**
   - Ensure all current functionality still works
   - Mixed usage of old and new syntax

## Migration Strategy

### Phased Rollout
1. **Phase 1**: Add new syntax alongside existing (dual support)
2. **Phase 2**: Update documentation to prefer new syntax
3. **Phase 3**: Add deprecation warnings for old syntax (future)
4. **Phase 4**: Eventually remove old syntax (far future)

### User Experience
```bash
# Day 1: Both syntaxes work
glang> create fruits [apple, banana]     # Old way (still works)
glang> list numbers = [1, 2, 3]         # New way (now available)

# Day N: New syntax feels natural
glang> list groceries = [bread, milk, eggs]
glang> groceries.append butter
glang> groceries
[bread, milk, eggs, butter]
```

## Expected Benefits

### Developer Experience
- **More intuitive** - Feels like programming language, not CLI tool
- **Less verbose** - No explicit `show` commands needed
- **More explicit** - Clear targeting with `fruits.append` vs implicit `append`
- **Better discovery** - Method completion reveals available operations

### Language Evolution
- **Foundation for functions** - `def process_list(items) = { ... }`
- **Object-oriented feel** - Methods naturally extend to user-defined types
- **Scalable syntax** - Easy to add new graph types and methods

### Graph Philosophy Preserved
- **Still graph-based** - All operations work on graph structures
- **Meta-graph intact** - Variable storage still uses VariableGraph
- **Introspection enhanced** - Better display modes show graph structure

## Timeline Estimate

### Day 1: Core Parsing
- Implement `SyntaxParser` with basic declaration and method parsing
- Add `MethodDispatcher` with essential linear graph methods
- Update REPL input processing

### Day 2: Display System  
- Implement `GraphRenderer` with simple and detailed modes
- Add variable access handling (just typing variable name)
- Integrate flag parsing for display options

### Day 3: Integration & Testing
- Enhanced tab completion for new syntax
- Comprehensive test suite
- Documentation updates
- Backward compatibility verification

### Day 4: Polish & Edge Cases
- Error handling improvements
- Edge case handling
- User experience refinements
- Performance optimization

This plan transforms glang from a graph manipulation tool into a true graph-based programming language while maintaining all existing functionality and the core "everything is a graph" philosophy!

## Phase 4: Advanced Data Access & Types

### 4.1 Array/List Indexing Syntax

#### Current vs Proposed
```bash
# Current (method-based)
glang> fruits.get 0
apple
glang> fruits.get 2  
cherry

# Proposed (direct indexing)
glang> fruits[0]
apple
glang> fruits[2]
cherry
glang> fruits[-1]    # Negative indexing
cherry
```

#### Implementation Requirements
- **Parser enhancement**: Recognize `variable[index]` pattern
- **Index validation**: Bounds checking with proper error messages
- **Negative indexing**: Support Python-style negative indices
- **Assignment support** (future): `fruits[0] = 'mango'`

### 4.2 Type System for Elements

#### Data Type Support
```bash
# Mixed types in lists
glang> list mixed = [42, 'hello', 3.14, true]
glang> mixed[0]
42 (num)
glang> mixed[1]
'hello' (string)

# Type introspection
glang> mixed.types()
[num, string, num, bool]
glang> mixed[0].type()
num
```

#### Core Types to Implement
1. **Numbers** (`num`): Integers and floats
   - Auto-detection: `42` → int, `3.14` → float
   - Mathematical operations support
   
2. **Strings** (`string`): Text values
   - Quote handling: `'text'` or `"text"`
   - Multi-word without quotes for backward compatibility
   
3. **Booleans** (`bool`): `true`/`false`
   
4. **Lists** (`list`): Nested graph structures (see 4.3)

#### Type Inference Rules
```bash
# Automatic type detection
glang> list data = [42, hello, 'multi word', 3.14]
# Interpreted as:
#   42 → num (integer)
#   hello → string (unquoted single word)
#   'multi word' → string (quoted)
#   3.14 → num (float)
```

### 4.3 Nested Lists/Graphs

#### Nested List Support
```bash
# Lists containing lists
glang> list matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
glang> matrix[0]
[1, 2, 3]
glang> matrix[0][1]
2

# Mixed nesting
glang> list data = [1, [a, b, c], 'text', [x, [y, z]]]
glang> data[1]
[a, b, c]
glang> data[3][1]
[y, z]
```

#### Graph Representation
```
# Internal structure for nested list
matrix (LINEAR graph)
  ├─ Node(data=[1,2,3], type=list) → points to another LINEAR graph
  ├─ Node(data=[4,5,6], type=list) → points to another LINEAR graph
  └─ Node(data=[7,8,9], type=list) → points to another LINEAR graph
```

#### Implementation Considerations
- **Recursive parsing**: Handle arbitrary nesting depth
- **Memory management**: Subgraphs as node values
- **Display formatting**: Pretty-print nested structures
- **Type checking**: `is_nested()`, `depth()` methods

### 4.4 Implementation Plan

#### Parser Updates
```python
class EnhancedParser:
    def parse_index_access(self, input_str: str):
        # Parse: fruits[0], matrix[1][2]
        pattern = r'(\w+)(\[[-?\d+\])+'
        # Extract variable and indices
        
    def parse_nested_list(self, list_str: str):
        # Parse: [[1,2], [3,4]]
        # Recursive parsing for nested structures
        
    def infer_type(self, value: str):
        # Type detection logic
        if value.isdigit():
            return ('num', int(value))
        elif '.' in value and is_float(value):
            return ('num', float(value))
        elif value in ['true', 'false']:
            return ('bool', value == 'true')
        elif value.startswith('['):
            return ('list', self.parse_nested_list(value))
        else:
            return ('string', value)
```

#### Graph Extensions
```python
class TypedNode(Node):
    """Node that tracks data type."""
    def __init__(self, data, data_type=None):
        super().__init__(data)
        self.data_type = data_type or self.infer_type(data)
    
    def is_list(self):
        return self.data_type == 'list'
    
    def is_num(self):
        return self.data_type == 'num'

class Graph:
    def __getitem__(self, index):
        """Support indexing: graph[0]"""
        if not self.graph_type.is_linear():
            raise TypeError("Indexing only supported for linear graphs")
        return self.get_at_index(index)
    
    def get_element_types(self):
        """Return list of element types."""
        return [node.data_type for node in self.nodes]
```

### 4.5 Future Enhancements (Phase 5+)

After implementing the above, consider:

1. **Slice notation**: `fruits[1:3]`, `fruits[::2]`
2. **Multi-dimensional indexing**: `matrix[0, 1]` as shorthand for `matrix[0][1]`
3. **Type constraints**: `list<num> scores = [95, 87, 92]`
4. **Graph references**: Nodes can reference other graphs
5. **Lazy evaluation**: Large nested structures evaluated on-demand
6. **Pattern matching**: `case fruits[0] of 'apple' -> ...`
7. **Comprehensions**: `[x*2 for x in numbers if x > 5]`

### 4.6 Testing Requirements

#### New Test Categories
1. **Indexing tests**
   - Valid indices (positive, negative)
   - Out of bounds handling
   - Chained indexing for nested structures

2. **Type system tests**
   - Type inference accuracy
   - Mixed type lists
   - Type introspection methods

3. **Nested structure tests**
   - Parse nested lists correctly
   - Access nested elements
   - Modify nested structures
   - Display nested structures

4. **Performance tests**
   - Large nested structures
   - Deep nesting levels
   - Memory usage with subgraphs

### 4.7 User Experience Examples

```bash
# Natural indexing
glang> list fruits = [apple, banana, cherry, date]
glang> fruits[0]
apple
glang> fruits[-1]
date

# Type awareness
glang> list mixed = [42, 'hello', [1, 2, 3]]
glang> mixed[0] * 2
84
glang> mixed[1].upper()
HELLO
glang> mixed[2][1]
2

# Nested structures
glang> list table = [['Name', 'Age'], ['Alice', 30], ['Bob', 25]]
glang> table[0]
['Name', 'Age']
glang> table[1][0]
Alice

# Future: assignment
glang> fruits[0] = 'mango'
glang> fruits
[mango, banana, cherry, date]
```

This enhancement phase will make glang feel more like a complete programming language with natural data access patterns while maintaining the graph-based architecture underneath!
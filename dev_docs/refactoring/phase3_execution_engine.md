# Phase 3: Execution Engine - AST-Based Interpretation

**Status: ✅ COMPLETE**  
**Prerequisites: ✅ Phases 1 & 2 Complete**  
**Completion Date: 2025-01-04**

## Overview
Replace the current string-based execution system with proper AST interpretation. The execution engine will take semantically analyzed AST nodes and execute them using visitor pattern, connecting to the existing graph infrastructure.

## Goals
1. **AST-Based Execution**: Interpret AST nodes instead of parsing strings at runtime
2. **Clean Value System**: Proper representation of glang values (strings, numbers, lists, booleans)
3. **Method Dispatch**: AST-based method calls with proper argument evaluation
4. **Graph Integration**: Connect AST execution with existing graph operations
5. **Error Handling**: Runtime errors with position information from AST nodes

---

## Architecture Design

### Current State (Phase 2 Complete):
```
Input → Tokenize → Parse → Semantic Analysis → AnalysisResult
                                                      ↓
                                               [AST + Symbol Table]
```

### Target State (Phase 3):
```
Input → Tokenize → Parse → Semantic Analysis → AST Execution → Result
                                                      ↓
                                               [Value + Graph Updates]
```

---

## Step 3.1: Value System

### Target: `/src/glang/execution/values.py`

**Status: PENDING**

Core value representation for glang runtime:

```python
from abc import ABC, abstractmethod
from typing import Any, List, Optional, Union
from ..ast.nodes import SourcePosition

class GlangValue(ABC):
    """Base class for all glang runtime values."""
    
    def __init__(self, position: Optional[SourcePosition] = None):
        self.position = position
    
    @abstractmethod
    def to_python(self) -> Any:
        """Convert to Python equivalent."""
        pass
    
    @abstractmethod  
    def get_type(self) -> str:
        """Get glang type name."""
        pass
    
    @abstractmethod
    def to_display_string(self) -> str:
        """String for display to user."""
        pass

class StringValue(GlangValue):
    def __init__(self, value: str, position: Optional[SourcePosition] = None):
        super().__init__(position)
        self.value = value
    
    def to_python(self) -> str:
        return self.value
    
    def get_type(self) -> str:
        return "string"

class NumberValue(GlangValue):
    def __init__(self, value: Union[int, float], position: Optional[SourcePosition] = None):
        super().__init__(position)
        self.value = value
    
    def to_python(self) -> Union[int, float]:
        return self.value
    
    def get_type(self) -> str:
        return "num"

class BooleanValue(GlangValue):
    def __init__(self, value: bool, position: Optional[SourcePosition] = None):
        super().__init__(position)
        self.value = value
        
    def to_python(self) -> bool:
        return self.value
    
    def get_type(self) -> str:
        return "bool"

class ListValue(GlangValue):
    def __init__(self, elements: List[GlangValue], constraint: Optional[str] = None, 
                 position: Optional[SourcePosition] = None):
        super().__init__(position)
        self.elements = elements
        self.constraint = constraint
    
    def to_python(self) -> List[Any]:
        return [elem.to_python() for elem in self.elements]
    
    def get_type(self) -> str:
        return "list"
    
    def validate_constraint(self, value: GlangValue) -> bool:
        """Check if value matches list constraint."""
        if not self.constraint:
            return True
        return value.get_type() == self.constraint
```

**Key Features:**
- Proper type system with constraint validation
- Position information for runtime errors
- Clean conversion between glang and Python values
- Foundation for method dispatch

---

## Step 3.2: AST Executor

### Target: `/src/glang/execution/executor.py`

**Status: PENDING**

Main execution engine using visitor pattern:

```python
from typing import Dict, Any, Optional
from ..ast.nodes import *
from ..semantic.symbol_table import SymbolTable
from .values import *
from .errors import RuntimeError

class ExecutionContext:
    """Context for AST execution."""
    
    def __init__(self, symbol_table: SymbolTable):
        self.symbol_table = symbol_table
        self.variables: Dict[str, GlangValue] = {}
    
    def get_variable(self, name: str) -> Optional[GlangValue]:
        return self.variables.get(name)
    
    def set_variable(self, name: str, value: GlangValue) -> None:
        self.variables[name] = value

class ASTExecutor(ASTVisitor):
    """Executes semantically analyzed AST nodes."""
    
    def __init__(self, context: ExecutionContext):
        self.context = context
        self.result = None
    
    def execute(self, node: ASTNode) -> Any:
        """Execute an AST node and return the result."""
        self.result = None
        node.accept(self)
        return self.result
    
    # Statement execution
    def visit_variable_declaration(self, node: VariableDeclaration) -> None:
        # Execute initializer
        initializer_value = self.execute(node.initializer)
        
        # Store in context
        self.context.set_variable(node.name, initializer_value)
        
        self.result = f"Declared {node.var_type} variable '{node.name}'"
    
    def visit_method_call(self, node: MethodCall) -> None:
        # Get target value
        target_value = self.execute(node.target)
        
        # Execute arguments
        arg_values = [self.execute(arg) for arg in node.arguments]
        
        # Dispatch method call
        result = self._dispatch_method(target_value, node.method_name, arg_values, node.position)
        self.result = result
    
    # Expression evaluation
    def visit_variable_ref(self, node: VariableRef) -> None:
        value = self.context.get_variable(node.name)
        if value is None:
            raise RuntimeError(f"Variable '{node.name}' not found", node.position)
        self.result = value
    
    def visit_string_literal(self, node: StringLiteral) -> None:
        self.result = StringValue(node.value, node.position)
    
    def visit_number_literal(self, node: NumberLiteral) -> None:
        self.result = NumberValue(node.value, node.position)
    
    def visit_boolean_literal(self, node: BooleanLiteral) -> None:
        self.result = BooleanValue(node.value, node.position)
    
    def visit_list_literal(self, node: ListLiteral) -> None:
        elements = [self.execute(elem) for elem in node.elements]
        self.result = ListValue(elements, None, node.position)
    
    # Helper methods
    def _dispatch_method(self, target: GlangValue, method_name: str, 
                        args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Dispatch method call to appropriate handler."""
        # This will integrate with existing method system
        pass
```

**Key Features:**
- Visitor pattern for clean AST interpretation
- Execution context for variable storage
- Integration points with existing method system
- Proper error handling with source positions

---

## Step 3.3: Method Integration

### Target: `/src/glang/execution/method_dispatcher.py`

**Status: PENDING**

Bridge between AST execution and existing method system:

```python
from typing import List, Any, Optional
from ..core.graph import Graph
from ..methods.linear_methods import LinearGraphMethods
from .values import *
from .errors import RuntimeError

class ASTMethodDispatcher:
    """Dispatches method calls from AST execution to graph methods."""
    
    def __init__(self):
        self.linear_methods = LinearGraphMethods()
    
    def dispatch_method(self, target: GlangValue, method_name: str, 
                       args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Dispatch method call based on target type."""
        
        target_type = target.get_type()
        
        if target_type == "list":
            return self._dispatch_list_method(target, method_name, args, position)
        elif target_type == "string":
            return self._dispatch_string_method(target, method_name, args, position)
        else:
            raise RuntimeError(f"Type '{target_type}' has no methods", position)
    
    def _dispatch_list_method(self, target: ListValue, method_name: str, 
                             args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Handle list method calls."""
        
        # Convert to graph for existing method system
        graph = self._list_value_to_graph(target)
        
        if method_name == "append":
            if len(args) != 1:
                raise RuntimeError(f"append() takes 1 argument, got {len(args)}", position)
            
            # Validate constraint
            if target.constraint and not args[0].get_type() == target.constraint:
                raise RuntimeError(
                    f"Cannot append {args[0].get_type()} to list<{target.constraint}>", 
                    position
                )
            
            # Call existing method
            python_value = args[0].to_python()
            result = self.linear_methods.append(graph, "temp", [str(python_value)])
            
            # Update list value
            target.elements.append(args[0])
            return result
        
        # Add more method implementations...
    
    def _list_value_to_graph(self, list_value: ListValue) -> Graph:
        """Convert ListValue to Graph for method calls."""
        # Integration with existing graph system
        pass
```

**Key Features:**
- Bridge between new AST system and existing method implementations
- Type constraint validation at runtime
- Clean error reporting with positions
- Gradual migration path for existing methods

---

## Step 3.4: Runtime Error System

### Target: `/src/glang/execution/errors.py`

**Status: PENDING**

Runtime error handling with AST position information:

```python
from typing import Optional
from ..ast.nodes import SourcePosition

class RuntimeError(Exception):
    """Base runtime error with position information."""
    
    def __init__(self, message: str, position: Optional[SourcePosition] = None):
        self.message = message
        self.position = position
        super().__init__(self._format_error())
    
    def _format_error(self) -> str:
        if self.position:
            return f"Runtime error: {self.message} at line {self.position.line}, column {self.position.column}"
        return f"Runtime error: {self.message}"

class VariableNotFoundError(RuntimeError):
    """Variable not found in execution context."""
    pass

class TypeConstraintError(RuntimeError):
    """Type constraint violation at runtime."""
    pass

class MethodNotFoundError(RuntimeError):
    """Method not found on target type."""
    pass

class ArgumentError(RuntimeError):
    """Invalid method arguments."""
    pass
```

---

## Step 3.5: Execution Pipeline

### Target: `/src/glang/execution/pipeline.py`

**Status: PENDING**

Complete pipeline from source code to execution:

```python
from ..semantic.pipeline import SemanticPipeline, AnalysisResult
from .executor import ASTExecutor, ExecutionContext
from .values import GlangValue

class ExecutionPipeline:
    """Complete pipeline: source → tokens → AST → semantic analysis → execution"""
    
    def __init__(self):
        self.semantic_pipeline = SemanticPipeline()
    
    def execute_code(self, input_str: str) -> Any:
        """Execute source code and return result."""
        
        # Phase 1 & 2: Parse and analyze
        analysis_result = self.semantic_pipeline.analyze_code(input_str)
        
        if not analysis_result.success:
            # Return semantic errors
            return analysis_result
        
        # Phase 3: Execute
        context = ExecutionContext(analysis_result.symbol_table)
        executor = ASTExecutor(context)
        
        try:
            result = executor.execute(analysis_result.ast)
            return ExecutionResult(result, context, success=True)
        except RuntimeError as e:
            return ExecutionResult(None, context, success=False, error=e)

@dataclass
class ExecutionResult:
    """Result of code execution."""
    value: Any
    context: ExecutionContext
    success: bool
    error: Optional[RuntimeError] = None

class ExecutionSession:
    """Session with persistent execution context."""
    
    def __init__(self):
        self.pipeline = ExecutionPipeline()
        self.context = ExecutionContext(SymbolTable())
    
    def execute_statement(self, input_str: str) -> ExecutionResult:
        """Execute statement in persistent context."""
        # Similar to SemanticSession but for execution
        pass
```

---

## Integration Strategy

### With Existing System:
1. **Graph Operations**: Connect through existing `LinearGraphMethods`, `GraphMethods`
2. **REPL Integration**: Replace string parsing in REPL with `ExecutionSession`
3. **Value Storage**: Bridge between `GlangValue` and existing graph storage
4. **Backward Compatibility**: Maintain existing CLI behavior

### Migration Path:
1. **Phase 3.1**: Implement core execution engine
2. **Phase 3.2**: Connect to existing method system  
3. **Phase 3.3**: Create execution pipeline
4. **Phase 3.4**: Integration testing with current system

---

## Testing Strategy

### Unit Tests:
- `test/test_execution_values.py` - Value system functionality
- `test/test_ast_executor.py` - AST execution engine
- `test/test_execution_pipeline.py` - End-to-end execution
- `test/test_method_dispatch.py` - Method dispatch integration

### Integration Tests:
- Execute same commands through old and new systems
- Compare results for consistency
- Performance benchmarking
- Error message quality comparison

### Test Cases:
```python
def test_variable_declaration_execution():
    pipeline = ExecutionPipeline()
    result = pipeline.execute_code('string greeting = "hello"')
    assert result.success
    assert "greeting" in result.context.variables

def test_method_call_execution():
    session = ExecutionSession()
    session.execute_statement('list items = [1, 2, 3]')
    result = session.execute_statement('items.append(4)')
    assert result.success
    # Verify list was modified

def test_constraint_validation():
    session = ExecutionSession()
    session.execute_statement('list<num> numbers = [1, 2, 3]')
    result = session.execute_statement('numbers.append("string")')
    assert not result.success
    assert "type constraint" in str(result.error).lower()
```

---

## Success Criteria

### Phase 3 Complete When:
1. ✅ AST nodes can be executed to produce values
2. ✅ Variable declarations create proper value storage  
3. ✅ Method calls work through AST dispatch system
4. ✅ Type constraints enforced at runtime
5. ✅ Runtime errors include source position information
6. ✅ Integration with existing graph operations
7. ✅ Full test coverage for execution scenarios

### Key Benefits:
- **No More String Parsing**: All execution through proper AST interpretation
- **Better Error Messages**: Runtime errors with exact source locations
- **Type Safety**: Constraint validation throughout execution
- **Clean Architecture**: Clear separation of parsing, analysis, and execution

---

## Implementation Checklist

### Week 1: Core Implementation
- [ ] Create execution package structure
- [ ] Implement value system (`GlangValue` hierarchy)
- [ ] Build AST executor with visitor pattern
- [ ] Create execution context and variable storage

### Week 2: Integration & Testing
- [ ] Connect method dispatch to existing system
- [ ] Build execution pipeline
- [ ] Create comprehensive test suite
- [ ] Integration testing with current functionality

### Completion Criteria:
- [ ] All execution tests passing
- [ ] Integration with existing graph system working
- [ ] Runtime errors properly formatted with positions
- [ ] Ready for Phase 4 REPL integration

---

**Dependencies**: Phases 1 & 2 must be complete (✅)  
**Next Phase**: Phase 4 - Integration with REPL/CLI systems  
**Estimated Completion**: 1-2 days for experienced developer
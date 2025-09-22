"""
Glang AST Executor

Executes semantically analyzed AST nodes using the visitor pattern.
This replaces the previous string-based execution system with proper
AST interpretation.
"""

from typing import Dict, Any, Optional, List
import sys
import os

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../../..'))

from glang.ast.nodes import *
from glang.semantic.symbol_table import SymbolTable
from .values import *
from .errors import RuntimeError, VariableNotFoundError, TypeConstraintError
from .stack_trace import get_stack_collector, create_enhanced_error_trace, push_execution_frame, pop_execution_frame, update_frame_variables
# Graph-based ListValue and HashValue are now the primary implementations
from .graph_values import ListValue, HashValue
from .configuration_context import ConfigurationContext
from .call_graph import CallGraph


class BreakException(Exception):
    """Exception raised by break statement to exit loops."""
    pass


class ContinueException(Exception):
    """Exception raised by continue statement to continue loops."""
    pass


class ReturnException(Exception):
    """Exception raised by return statement to return from functions."""
    def __init__(self, value: Optional['GlangValue'] = None):
        self.value = value
        super().__init__()


class ExecutionContext:
    """Context for AST execution with variable storage."""
    
    def __init__(self, symbol_table: SymbolTable, module_manager=None):
        self.symbol_table = symbol_table
        self.variables: Dict[str, GlangValue] = {}
        self.module_manager = module_manager  # Will be set by execution pipeline
        self.config = ConfigurationContext()  # Configuration stack
        self.loading_module = False  # Track if we're currently loading a module
        self.module_functions = {}   # Functions created during module loading
        self.call_graph = CallGraph()  # True graph-based function discovery
        self.current_module = None   # Track current module for scoping
    
    def get_variable(self, name: str) -> Optional[GlangValue]:
        """Get variable value by name.
        
        Supports module-qualified names like 'math.pi'.
        """
        # Check for module-qualified name
        if '.' in name:
            parts = name.split('.', 1)
            module_name = parts[0]
            symbol_name = parts[1]
            
            if self.module_manager:
                module = self.module_manager.get_module(module_name)
                if module:
                    return module.namespace.get_symbol(symbol_name)
            return None
        
        # Regular variable lookup
        return self.variables.get(name)
    
    def set_variable(self, name: str, value: GlangValue) -> None:
        """Set variable value."""
        # Module-qualified names cannot be set directly
        if '.' in name:
            raise RuntimeError(f"Cannot assign to module-qualified name: {name}")
        self.variables[name] = value
    
    def has_variable(self, name: str) -> bool:
        """Check if variable exists in context."""
        # Check for module-qualified name
        if '.' in name:
            parts = name.split('.', 1)
            module_name = parts[0]
            symbol_name = parts[1]
            
            if self.module_manager:
                module = self.module_manager.get_module(module_name)
                if module:
                    return module.namespace.get_symbol(symbol_name) is not None
            return False
        
        return name in self.variables
    
    def list_variables(self) -> List[str]:
        """Get list of all variable names."""
        return list(self.variables.keys())


class ASTExecutor(BaseASTVisitor):
    """Executes semantically analyzed AST nodes."""
    
    def __init__(self, context: ExecutionContext, file_manager=None):
        self.context = context
        self.file_manager = file_manager
        self.result = None

        # Setup call graph module with executor reference
        from glang.modules.call_graph_module import setup_call_graph_module
        setup_call_graph_module(self)

        # Set default precision if not already set
        from decimal import getcontext
        if getcontext().prec == 0 or getcontext().prec == 28:
            # Use Glang's default precision
            from ..execution.glang_number import PrecisionGlangNumber
            getcontext().prec = PrecisionGlangNumber.DEFAULT_PRECISION
    
    def execute(self, node: ASTNode) -> Any:
        """Execute an AST node and return the result."""
        self.result = None
        node.accept(self)
        return self.result
    
    # Statement execution
    def visit_variable_declaration(self, node: VariableDeclaration) -> None:
        """Execute variable declaration."""
        # Execute initializer to get value
        initializer_value = self.execute(node.initializer)
        
        # Ensure we have a GlangValue
        if not isinstance(initializer_value, GlangValue):
            initializer_value = python_to_glang_value(initializer_value, node.position)
        
        # For list declarations, set constraint if specified
        if node.var_type == "list" and initializer_value.get_type() == "list":
            if node.type_constraint:
                initializer_value.constraint = node.type_constraint
                # Validate existing elements against constraint
                for elem in initializer_value.elements:
                    if not initializer_value.validate_constraint(elem):
                        raise TypeConstraintError(
                            f"Element {elem.to_display_string()} violates list<{node.type_constraint}> constraint",
                            elem.position or node.position
                        )

        # For data declarations, set constraint if specified
        elif node.var_type == "data" and initializer_value.get_type() == "data":
            if node.type_constraint:
                initializer_value.constraint = node.type_constraint
                # Validate existing value against constraint
                if not initializer_value.validate_constraint(initializer_value.value):
                    raise TypeConstraintError(
                        f"Value {initializer_value.value.to_display_string()} violates data<{node.type_constraint}> constraint",
                        initializer_value.value.position or node.position
                    )
        
        # For hash declarations, handle both HashValue and DataValue (single pair) initializers
        elif node.var_type == "hash":
            init_type = initializer_value.get_type()
            if init_type == "data":
                # Convert single DataValue to HashValue for hash declarations
                pairs = [(initializer_value.key, initializer_value.value)]
                initializer_value = HashValue(pairs, node.type_constraint, initializer_value.position)
            elif init_type == "hash":
                if node.type_constraint:
                    initializer_value.constraint = node.type_constraint

            # Validate constraint if specified
            if node.type_constraint and initializer_value.get_type() == "hash":
                for key, value in initializer_value.pairs.items():
                    if not initializer_value.validate_constraint(value):
                        raise TypeConstraintError(
                            f"Value {value.to_display_string()} for key '{key}' violates hash<{node.type_constraint}> constraint",
                            value.position or node.position
                        )
        
        # Apply behaviors if present
        if node.behaviors:
            behavior_pipeline = self._build_behavior_pipeline(node.behaviors)
            value_type = initializer_value.get_type()
            if value_type == "list":
                initializer_value = behavior_pipeline.apply_to_list(initializer_value)
            elif value_type == "hash":
                # For hashes, we'd need to apply to specific keys
                # For now, just apply to the hash as a whole (limited functionality)
                pass
            else:
                initializer_value = behavior_pipeline.apply(initializer_value)
        
        # Store in context
        self.context.set_variable(node.name, initializer_value)
        
        # Return description of what was declared
        constraint_str = f"<{node.type_constraint}>" if node.type_constraint else ""
        behavior_str = f" with {len(node.behaviors.behaviors)} behaviors" if node.behaviors else ""
        self.result = f"Declared {node.var_type}{constraint_str} variable '{node.name}'{behavior_str}"
    
    def visit_assignment(self, node: Assignment) -> None:
        """Execute assignment statement."""
        # Get the value to assign
        value = self.execute(node.value)
        
        if not isinstance(value, GlangValue):
            value = python_to_glang_value(value, node.position)
        
        if isinstance(node.target, VariableRef):
            # Simple variable assignment
            var_name = node.target.name
            
            # Check if variable exists
            if not self.context.has_variable(var_name):
                # NEW: Type inference - create variable with inferred type
                self.context.set_variable(var_name, value)
                inferred_type = value.get_type()
                self.result = f"Declared {inferred_type} variable '{var_name}' (inferred)"
            else:
                # Variable exists - check constraints and assign
                existing_var = self.context.get_variable(var_name)
                if existing_var.get_type() == "list" and hasattr(existing_var, 'constraint') and existing_var.constraint:
                    if not existing_var.validate_constraint(value):
                        raise TypeConstraintError(
                            f"Cannot assign {value.get_type()} to {existing_var.get_type()}<{existing_var.constraint}>",
                            node.position
                        )
                
                self.context.set_variable(var_name, value)
                self.result = f"Assigned {value.to_display_string()} to {var_name}"
        
        elif isinstance(node.target, IndexAccess):
            # Index assignment like arr[0] = value
            target_value = self.execute(node.target.target)
            
            if len(node.target.indices) != 1:
                raise RuntimeError(
                    f"Multi-dimensional indexing not yet supported", 
                    node.target.position
                )
            
            index_value = self.execute(node.target.indices[0])
            
            if not isinstance(target_value, ListValue):
                raise RuntimeError(
                    f"Cannot index {target_value.get_type()}", 
                    node.target.position
                )
            
            # Support both integer and string indices for enhanced indexing
            if isinstance(index_value, NumberValue) and isinstance(index_value.value, int):
                # Integer indexing
                index_key = index_value.value
            elif isinstance(index_value, StringValue):
                # String-based name indexing
                index_key = index_value.value
            else:
                raise RuntimeError(
                    f"List index must be integer or string, got {index_value.get_type()}",
                    node.target.indices[0].position
                )

            target_value[index_key] = value
            # Get target name safely - could be VariableRef or nested IndexAccess
            target_name = getattr(node.target.target, 'name', 'target')
            self.result = f"Set {target_name}[{index_key}] = {value.to_display_string()}"
        
        elif isinstance(node.target, (MethodCall, MethodCallExpression)):
            # Method call assignment - provide specific error messages
            method_name = node.target.method_name
            
            if hasattr(node.target.target, 'name'):
                var_name = node.target.target.name
                if method_name == 'key':
                    # Keys are immutable
                    raise RuntimeError(
                        f"Assignment to data node key is not allowed. "
                        f"Data node keys are immutable.",
                        node.position
                    )
                elif method_name == 'value':
                    # Handle data node value assignment
                    target_var = self.context.variables.get(var_name)
                    if target_var and hasattr(target_var, 'value') and hasattr(target_var, 'set_value'):
                        # This is a data node - update its value
                        new_value = self.execute(node.value)
                        target_var.set_value(new_value)
                        self.result = f"Updated data node '{var_name}' value"
                    else:
                        raise RuntimeError(
                            f"Variable '{var_name}' is not a data node or does not support value assignment.",
                            node.position
                        )
                else:
                    raise RuntimeError(
                        f"Property assignment '{var_name}.{method_name} = ...' is not supported. "
                        f"Use method calls like '{var_name}.{method_name}()' to access values.",
                        node.position
                    )
            else:
                raise RuntimeError(
                    f"Property assignment is not supported in this language. Use method calls instead.",
                    node.position
                )
        
        else:
            raise RuntimeError(f"Invalid assignment target", node.position)
    
    def visit_method_call(self, node: MethodCall) -> None:
        """Execute method call."""
        # Check if this might be a module-qualified access (e.g., math.pi or io.read_file)
        # Check if target is a VariableRef (not a method call chain)
        # Check if target is a VariableRef by class name to avoid import path issues
        is_variable_ref = node.target.__class__.__name__ == 'VariableRef'
        if is_variable_ref:
            # Check if the target is a module value
            from .module_value import ModuleValue
            target_value = self.context.get_variable(node.target.name)
            if isinstance(target_value, ModuleValue):
                # Access symbol from the module
                module = target_value.module
                symbol_name = node.method_name
                symbol_value = module.namespace.get_symbol(symbol_name)
                
                if symbol_value is not None:
                    # Check if it's a function that needs to be called
                    from ..execution.function_value import BuiltinFunctionValue
                    from .values import FunctionValue
                    if isinstance(symbol_value, BuiltinFunctionValue):
                        # Execute arguments for built-in function
                        arg_values = []
                        for arg in node.arguments:
                            arg_value = self.execute(arg)
                            if not isinstance(arg_value, GlangValue):
                                arg_value = python_to_glang_value(arg_value, arg.position if hasattr(arg, 'position') else node.position)
                            arg_values.append(arg_value)
                        
                        # Call the built-in function
                        self.result = symbol_value.call(arg_values, node.position)
                        return
                    elif isinstance(symbol_value, FunctionValue):
                        # Execute arguments for user-defined function
                        arg_values = []
                        for arg in node.arguments:
                            arg_value = self.execute(arg)
                            if not isinstance(arg_value, GlangValue):
                                arg_value = python_to_glang_value(arg_value, arg.position if hasattr(arg, 'position') else node.position)
                            arg_values.append(arg_value)
                        
                        # Call the user-defined function using call_function
                        self.result = self.call_function(symbol_value, arg_values, node.position)
                        return
                    elif len(node.arguments) == 0:
                        # It's a property/constant access (like math.pi)
                        self.result = symbol_value
                        return
                    else:
                        # It's a symbol but not a callable and has arguments
                        # This shouldn't happen with our current module system
                        from ..modules.errors import ModuleSymbolError
                        raise ModuleSymbolError(node.target.name, symbol_name, node.position)
                else:
                    # Symbol not found in module
                    from ..modules.errors import ModuleSymbolError
                    raise ModuleSymbolError(node.target.name, symbol_name, node.position)
                # Return here to prevent falling through to regular method dispatch
                return
            
            # Fall back to old module manager approach for compatibility
            elif self.context.module_manager:
                # Try to resolve as module.symbol first
                module_name = node.target.name
                symbol_name = node.method_name
                module = self.context.module_manager.get_module(module_name)
                
                if module:
                    symbol_value = module.namespace.get_symbol(symbol_name)
                    if symbol_value is not None:
                        # Check if it's a built-in function that needs to be called
                        from ..execution.function_value import BuiltinFunctionValue
                        if isinstance(symbol_value, BuiltinFunctionValue):
                            # Execute arguments
                            arg_values = []
                            for arg in node.arguments:
                                arg_value = self.execute(arg)
                                if not isinstance(arg_value, GlangValue):
                                    arg_value = python_to_glang_value(arg_value, arg.position if hasattr(arg, 'position') else node.position)
                                arg_values.append(arg_value)
                            
                            # Call the built-in function
                            self.result = symbol_value.call(arg_values, node.position)
                            return
                        elif len(node.arguments) == 0:
                            # It's a property/constant access (like math.pi)
                            self.result = symbol_value
                            return
        
        # Fall back to regular method call execution
        try:
            # Get target value
            target_value = self.execute(node.target)
            
            if not isinstance(target_value, GlangValue):
                target_value = python_to_glang_value(target_value, node.position)
            
            # Execute arguments
            arg_values = []
            for arg in node.arguments:
                arg_value = self.execute(arg)
                if not isinstance(arg_value, GlangValue):
                    arg_value = python_to_glang_value(arg_value, arg.position)
                arg_values.append(arg_value)
            
            # Dispatch method call
            result = self._dispatch_method(target_value, node.method_name, arg_values, node.position)
            self.result = result
        except VariableNotFoundError:
            # If the target variable doesn't exist, give a more helpful error message
            if isinstance(node.target, VariableRef) and self.context.module_manager:
                module_name = node.target.name
                symbol_name = node.method_name
                if self.context.module_manager.get_module(module_name):
                    # Module exists but symbol doesn't
                    from ..modules.errors import ModuleSymbolError
                    raise ModuleSymbolError(module_name, symbol_name, node.position)
            raise
    
    # Expression evaluation
    def visit_variable_ref(self, node: VariableRef) -> None:
        """Evaluate variable reference."""
        value = self.context.get_variable(node.name)
        if value is None:
            stack_trace = create_enhanced_error_trace(f"Variable '{node.name}' not found", "VariableNotFoundError")
            raise VariableNotFoundError(node.name, node.position, stack_trace)
        self.result = value
    
    def visit_string_literal(self, node: StringLiteral) -> None:
        """Evaluate string literal."""
        # Remove quotes from the literal value
        cleaned_value = node.value
        if (cleaned_value.startswith('"') and cleaned_value.endswith('"')) or \
           (cleaned_value.startswith("'") and cleaned_value.endswith("'")):
            cleaned_value = cleaned_value[1:-1]

        # Process escape sequences
        cleaned_value = self._process_escape_sequences(cleaned_value)

        self.result = StringValue(cleaned_value, node.position)
    
    def visit_number_literal(self, node: NumberLiteral) -> None:
        """Evaluate number literal."""
        self.result = NumberValue(node.value, node.position)
    
    def visit_boolean_literal(self, node: BooleanLiteral) -> None:
        """Evaluate boolean literal."""
        self.result = BooleanValue(node.value, node.position)

    def visit_none_literal(self, node: NoneLiteral) -> None:
        """Evaluate none literal."""
        self.result = NoneValue(node.position)

    def visit_symbol_literal(self, node: SymbolLiteral) -> None:
        """Evaluate symbol literal."""
        self.result = SymbolValue(node.name, node.position)

    def visit_list_literal(self, node: ListLiteral) -> None:
        """Evaluate list literal."""
        elements = []
        for elem in node.elements:
            elem_value = self.execute(elem)
            if not isinstance(elem_value, GlangValue):
                elem_value = python_to_glang_value(elem_value, elem.position)
            elements.append(elem_value)
        
        self.result = ListValue(elements, None, node.position)
    
    def visit_data_node_literal(self, node: DataNodeLiteral) -> None:
        """Evaluate data node literal."""
        # Evaluate the value expression
        value = self.execute(node.value)
        if not isinstance(value, GlangValue):
            value = python_to_glang_value(value, node.value.position)
        
        # Create DataValue with the key and evaluated value
        self.result = DataValue(node.key, value, None, node.position)
    
    def visit_map_literal(self, node: MapLiteral) -> None:
        """Evaluate map literal."""
        # Evaluate all value expressions
        evaluated_pairs = []
        for key, value_expr in node.pairs:
            value = self.execute(value_expr)
            if not isinstance(value, GlangValue):
                value = python_to_glang_value(value, value_expr.position)
            evaluated_pairs.append((key, value))
        
        # Create HashValue with the evaluated pairs
        self.result = HashValue(evaluated_pairs, None, node.position)
    
    def visit_index_access(self, node: IndexAccess) -> None:
        """Evaluate index access."""
        target_value = self.execute(node.target)
        
        # For now, only handle single-dimensional indexing
        if len(node.indices) != 1:
            raise RuntimeError(
                f"Multi-dimensional indexing not yet supported", 
                node.position
            )
        
        index_value = self.execute(node.indices[0])
        
        # Handle list indexing (both old and new implementations)
        # Using already imported ListValue, HashValue
        if isinstance(target_value, (ListValue, ListValue)):
            # Support both integer and string indices for enhanced indexing
            if isinstance(index_value, NumberValue) and isinstance(index_value.value, int):
                # Integer indexing
                index_key = index_value.value
            elif isinstance(index_value, StringValue):
                # String-based name indexing
                index_key = index_value.value
            else:
                raise RuntimeError(
                    f"List index must be integer or string, got {index_value.get_type()}",
                    node.indices[0].position
                )

            # Use the appropriate method based on the implementation
            if hasattr(target_value, 'get_element'):
                self.result = target_value.get_element(index_key)
            else:
                # Use graph-based ListValue implementation with enhanced indexing
                self.result = target_value[index_key]
        
        # Handle string indexing
        elif isinstance(target_value, StringValue):
            if not isinstance(index_value, NumberValue) or not isinstance(index_value.value, int):
                raise RuntimeError(
                    f"String index must be integer, got {index_value.get_type()}",
                    node.indices[0].position
                )
            
            idx = index_value.value
            # Use character node-based indexing with automatic negative index handling
            try:
                self.result = target_value.get_char_at(idx)
            except IndexError as e:
                raise RuntimeError(str(e), node.indices[0].position)
        
        # Handle hash indexing - returns data node, not raw value (both old and new implementations)
        elif isinstance(target_value, (HashValue, HashValue)):
            if not isinstance(index_value, StringValue):
                raise RuntimeError(
                    f"Hash index must be string, got {index_value.get_type()}",
                    node.indices[0].position
                )

            key = index_value.value

            # Use HashValue's enhanced __getitem__ which handles both keys and names
            try:
                self.result = target_value[key]
            except KeyError:
                raise RuntimeError(
                    f"Key '{key}' not found in map",
                    node.indices[0].position
                )
        
        else:
            raise RuntimeError(
                f"Cannot index {target_value.get_type()}", 
                node.position
            )
    
    # Helper methods
    def _dispatch_method(self, target: GlangValue, method_name: str, 
                        args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Dispatch method call to appropriate handler.
        
        Resolution order:
        1. Universal methods (inherited from GlangValue)  
        2. Type-specific methods (in dispatchers)
        """
        # First check for universal methods
        if method_name in ['type', 'size', 'inspect', 'freeze', 'is_frozen', 'contains_frozen', 'node']:
            return self._dispatch_universal_method(target, method_name, args, position)
        elif method_name in ['methods', 'can']:
            # These need access to the method registry, so handled specially
            return self._dispatch_registry_method(target, method_name, args, position)
        
        # Then check type-specific methods
        target_type = target.get_type()
        
        if target_type == "list":
            return self._dispatch_list_method(target, method_name, args, position)
        elif target_type == "string":
            return self._dispatch_string_method(target, method_name, args, position)
        elif target_type == "num":
            return self._dispatch_num_method(target, method_name, args, position)
        elif target_type == "bool":
            return self._dispatch_bool_method(target, method_name, args, position)
        elif target_type == "data":
            return self._dispatch_data_method(target, method_name, args, position)
        elif target_type == "hash":
            return self._dispatch_hash_method(target, method_name, args, position)
        elif target_type == "tree":
            return self._dispatch_tree_method(target, method_name, args, position)
        elif target_type == "time":
            return self._dispatch_time_method(target, method_name, args, position)
        elif target_type == "file":
            return self._dispatch_file_method(target, method_name, args, position)
        elif target_type == "node":
            return self._dispatch_node_method(target, method_name, args, position)
        else:
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, target_type, position)
    
    def _process_escape_sequences(self, text: str) -> str:
        """Process escape sequences in string literals."""
        if not text:
            return text

        result = []
        i = 0
        while i < len(text):
            if text[i] == '\\' and i + 1 < len(text):
                # Found escape sequence
                next_char = text[i + 1]
                if next_char == 'n':
                    result.append('\n')
                elif next_char == 't':
                    result.append('\t')
                elif next_char == 'r':
                    result.append('\r')
                elif next_char == '\\':
                    result.append('\\')
                elif next_char == '"':
                    result.append('"')
                elif next_char == "'":
                    result.append("'")
                else:
                    # Unknown escape sequence - keep as is
                    result.append('\\')
                    result.append(next_char)
                i += 2  # Skip both characters
            else:
                result.append(text[i])
                i += 1

        return ''.join(result)

    def _dispatch_node_method(self, target, method_name: str,
                            args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Handle node method calls."""
        from .node_value import NodeValue

        if not isinstance(target, NodeValue):
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, "node", position)

        if method_name == "neighbors":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"neighbors() takes no arguments, got {len(args)}", position)
            return target.neighbors

        elif method_name == "value":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"value() takes no arguments, got {len(args)}", position)
            return target.value

        elif method_name == "container":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"container() takes no arguments, got {len(args)}", position)
            return target.container

        elif method_name == "id":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"id() takes no arguments, got {len(args)}", position)
            return target.id

        elif method_name == "has_neighbor":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"has_neighbor() takes 1 argument, got {len(args)}", position)
            return target.has_neighbor(args[0])

        elif method_name == "path_to":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"path_to() takes 1 argument, got {len(args)}", position)
            return target.path_to(args[0])

        elif method_name == "distance_to":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"distance_to() takes 1 argument, got {len(args)}", position)
            return target.distance_to(args[0])

        elif method_name == "edges":
            if len(args) > 1:
                from .errors import ArgumentError
                raise ArgumentError(f"edges() takes 0 or 1 arguments, got {len(args)}", position)

            if len(args) == 0:
                return target.edges()  # Default to out
            else:
                # Get direction argument
                if not isinstance(args[0], StringValue):
                    from .errors import ArgumentError
                    raise ArgumentError(f"edges() argument must be string, got {args[0].get_type()}", position)

                direction = args[0].value
                if direction not in ("out", "in", "all"):
                    from .errors import ArgumentError
                    raise ArgumentError(f"edges() direction must be 'out', 'in', or 'all', got '{direction}'", position)

                return target.edges(direction)

        else:
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, "node", position)

    def _dispatch_universal_method(self, target: GlangValue, method_name: str,
                                  args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Handle universal methods that all nodes inherit."""
        
        if method_name == "type":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"type() takes no arguments, got {len(args)}", position)
            return target.universal_type()
        
        elif method_name == "size":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"size() takes no arguments, got {len(args)}", position)
            return target.universal_size()
        
        elif method_name == "inspect":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"inspect() takes no arguments, got {len(args)}", position)
            # Show detailed structural information (what visualize_structure used to do)
            if hasattr(target, 'visualize_structure'):
                structure_info = target.visualize_structure()
                return StringValue(structure_info, position)
            else:
                # Fallback for non-graph types
                return StringValue(f"{target.get_type()} with value: {target.to_display_string()}", position)
        
        elif method_name == "freeze":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"freeze() takes no arguments, got {len(args)}", position)
            return target.freeze()
        
        elif method_name == "is_frozen":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"is_frozen() takes no arguments, got {len(args)}", position)
            return BooleanValue(target.is_frozen_value(), position)
        
        elif method_name == "contains_frozen":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"contains_frozen() takes no arguments, got {len(args)}", position)
            return BooleanValue(target.contains_frozen_data(), position)

        elif method_name == "node":
            # Access the graph node wrapper for this value
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"node() takes no arguments, got {len(args)}", position)
            # Wrap the GraphNode in a NodeValue for method dispatch
            from .node_value import NodeValue
            return NodeValue(target.node, position)

        else:
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, target.get_type(), position)
    
    def _dispatch_registry_method(self, target: GlangValue, method_name: str,
                                 args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Handle methods that need access to the method registry."""
        
        if method_name == "methods":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"methods() takes no arguments, got {len(args)}", position)
            
            # Provide method registry to the target for universal_methods() call
            target_type = target.get_type()
            available_methods = self._get_available_methods(target_type)
            method_strings = [StringValue(method, position) for method in available_methods]
            return ListValue(method_strings, "string", position)
        
        elif method_name == "can":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"can() takes 1 argument, got {len(args)}", position)
            
            if not isinstance(args[0], StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"can() argument must be string, got {args[0].get_type()}", position)
            
            # Provide method registry to the target for universal_can() call
            method_to_check = args[0].value
            target_type = target.get_type()
            available_methods = self._get_available_methods(target_type)
            return BooleanValue(method_to_check in available_methods, position)
        
        else:
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, target.get_type(), position)
    
    def _get_available_methods(self, target_type: str) -> List[str]:
        """Get list of available methods for a given type."""
        # Universal methods available on all types
        universal_methods = ['type', 'methods', 'can', 'inspect', 'size', 'freeze', 'is_frozen', 'contains_frozen', 'node']
        
        # Behavior management methods (available on list and hash types)
        behavior_methods = ['add_rule', 'remove_rule', 'has_rule', 'rules', 'clear_rules']

        # Type-specific methods
        type_methods = {
            'list': ['append', 'prepend', 'insert', 'reverse', 'index_of', 'count', 'min', 'max', 'sum', 'sort', 'map', 'filter', 'select', 'reject', 'each', 'clear', 'empty', 'pop', 'remove', 'constraint', 'types', 'type_summary', 'validate_constraint', 'coerce_to_constraint', 'to_string', 'to_bool', 'can_accept', 'add_edge', 'connected_to', 'to_graph', 'edges', 'can_add_edge', 'count_edges', 'count_nodes', 'graph_summary', 'visualize_structure', 'visualize', 'view', 'names', 'has_names', 'name', 'set_name'] + behavior_methods,
            'string': ['length', 'contains', 'extract', 'count', 'count_chars', 'find_first', 'find_first_char', 'up', 'toUpper', 'down', 'toLower', 'split', 'split_on_any', 'trim', 'join', 'matches', 'replace', 'find_all', 'findAll', 'is_email', 'is_number', 'is_url', 'reverse', 'unique', 'chars', 'starts_with', 'ends_with', 'to_string', 'to_num', 'to_bool', 'to_time'],
            'num': ['to', 'abs', 'sqrt', 'log', 'pow', 'rnd', 'rnd_up', 'rnd_dwn', 'to_string', 'to_num', 'to_bool', 'to_time'],
            'bool': ['flip', 'toggle', 'numify', 'toNum', 'to_string', 'to_num', 'to_bool'],
            'data': ['key', 'value', 'can_accept'],
            'hash': ['set', 'has_key', 'count_values', 'keys', 'values', 'remove', 'empty', 'merge', 'push', 'pop', 'can_accept', 'to_string', 'to_bool', 'add_value_edge', 'get_connected_keys', 'names', 'has_names', 'can_add_edge', 'count_edges', 'count_nodes', 'get_edges', 'get_graph_summary', 'get_active_rules', 'get_rule_status', 'disable_rule', 'enable_rule', 'visualize', 'view'] + behavior_methods,
            'node': ['neighbors', 'value', 'container', 'id', 'has_neighbor', 'path_to', 'distance_to', 'edges'],
            'time': ['get_type', 'to_string', 'to_num']
        }
        
        specific_methods = type_methods.get(target_type, [])
        return universal_methods + specific_methods
    
    def _dispatch_list_method(self, target: ListValue, method_name: str, 
                             args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Handle list method calls."""
        
        # Type conversion methods
        if method_name == "to_string":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"to_string() takes no arguments, got {len(args)}", position)
            # Convert list to string representation
            element_strs = [elem.to_display_string() for elem in target.elements]
            return StringValue(f"[{', '.join(element_strs)}]", position)
        
        elif method_name == "to_bool":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"to_bool() takes no arguments, got {len(args)}", position)
            # Non-empty list is true, empty is false
            return BooleanValue(len(target.elements) > 0, position)
        
        elif method_name == "append":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"append() takes 1 argument, got {len(args)}", position)
            
            # Use ListValue's append method (includes constraint validation)
            target.append(args[0])
            return target  # Return the list for chaining
        
        elif method_name == "prepend":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"prepend() takes 1 argument, got {len(args)}", position)
            
            # Validate constraint
            if not target.validate_constraint(args[0]):
                raise TypeConstraintError(
                    f"Cannot prepend {args[0].get_type()} to list<{target.constraint}>",
                    position
                )
            
            target.elements.insert(0, args[0])
            return target  # Return the list for chaining
        
        elif method_name == "insert":
            if len(args) != 2:
                from .errors import ArgumentError
                raise ArgumentError(f"insert() takes 2 arguments, got {len(args)}", position)
            
            index_arg, value_arg = args
            if not isinstance(index_arg, NumberValue) or not isinstance(index_arg.value, int):
                from .errors import ArgumentError
                raise ArgumentError("insert() first argument must be integer", position)
            
            # Validate constraint
            if not target.validate_constraint(value_arg):
                raise TypeConstraintError(
                    f"Cannot insert {value_arg.get_type()} into list<{target.constraint}>",
                    position
                )
            
            index = index_arg.value
            if not 0 <= index <= len(target.elements):
                from .errors import IndexError
                raise IndexError(f"Insert index {index} out of range", position)
            
            target.elements.insert(index, value_arg)
            return target  # Return the list for chaining
        
        elif method_name == "reverse":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"reverse() takes no arguments, got {len(args)}", position)
            
            # Create a copy and reverse it (immutable operation)
            reversed_elements = target.elements.copy()
            reversed_elements.reverse()
            return ListValue(reversed_elements, target.constraint, position)
        
        # List analysis methods
        
        elif method_name == "count":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"count() takes 1 argument, got {len(args)}", position)
            
            search_value = args[0]
            count = sum(1 for element in target.elements if element == search_value)
            return NumberValue(count, position)
        
        elif method_name == "min":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"min() takes no arguments, got {len(args)}", position)

            if len(target.elements) == 0:
                raise RuntimeError("Cannot find minimum of empty list", position)

            # Check configuration for skip_none behavior
            skip_none = self.context.config.should_skip_none()

            # Filter elements based on configuration
            valid_elements = []
            for element in target.elements:
                if isinstance(element, NoneValue):
                    if not skip_none:
                        from .errors import ArgumentError
                        raise ArgumentError(f"min() requires all elements to be numbers, found none", position)
                    # Skip none values when skip_none is True
                    continue
                elif not isinstance(element, NumberValue):
                    from .errors import ArgumentError
                    raise ArgumentError(f"min() requires all elements to be numbers, found {element.get_type()}", position)
                else:
                    valid_elements.append(element)

            if not valid_elements:
                raise RuntimeError("Cannot find minimum of empty list after filtering", position)

            min_element = min(valid_elements, key=lambda x: x.value)
            return NumberValue(min_element.value, position)
        
        elif method_name == "max":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"max() takes no arguments, got {len(args)}", position)

            if len(target.elements) == 0:
                raise RuntimeError("Cannot find maximum of empty list", position)

            # Check configuration for skip_none behavior
            skip_none = self.context.config.should_skip_none()

            # Filter elements based on configuration
            valid_elements = []
            for element in target.elements:
                if isinstance(element, NoneValue):
                    if not skip_none:
                        from .errors import ArgumentError
                        raise ArgumentError(f"max() requires all elements to be numbers, found none", position)
                    # Skip none values when skip_none is True
                    continue
                elif not isinstance(element, NumberValue):
                    from .errors import ArgumentError
                    raise ArgumentError(f"max() requires all elements to be numbers, found {element.get_type()}", position)
                else:
                    valid_elements.append(element)

            if not valid_elements:
                raise RuntimeError("Cannot find maximum of empty list after filtering", position)

            max_element = max(valid_elements, key=lambda x: x.value)
            return NumberValue(max_element.value, position)
        
        elif method_name == "sum":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"sum() takes no arguments, got {len(args)}", position)

            if len(target.elements) == 0:
                return NumberValue(0, position)  # Sum of empty list is 0

            # Check configuration for skip_none behavior
            skip_none = self.context.config.should_skip_none()

            # Filter elements based on configuration
            elements_to_sum = []
            for element in target.elements:
                if isinstance(element, NoneValue):
                    if not skip_none:
                        from .errors import ArgumentError
                        raise ArgumentError(f"sum() requires all elements to be numbers, found none", position)
                    # Skip none values when skip_none is True
                    continue
                elif not isinstance(element, NumberValue):
                    from .errors import ArgumentError
                    raise ArgumentError(f"sum() requires all elements to be numbers, found {element.get_type()}", position)
                else:
                    elements_to_sum.append(element)

            # Calculate sum
            if not elements_to_sum:
                return NumberValue(0, position)  # Sum of empty list after filtering is 0

            total = sum(element.value for element in elements_to_sum)
            return NumberValue(total, position)
        
        # List transformation methods
        elif method_name == "sort":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"sort() takes no arguments, got {len(args)}", position)
            
            if len(target.elements) == 0:
                # Return new empty list with same constraint
                return ListValue([], target.constraint, position)
            
            # Check that all elements are the same type and comparable
            first_element = target.elements[0]
            element_type = type(first_element)
            
            for element in target.elements:
                if type(element) != element_type:
                    from .errors import ArgumentError
                    raise ArgumentError(f"sort() requires all elements to be the same type", position)
            
            # Create a copy of elements for sorting (immutable operation)
            sorted_elements = target.elements.copy()
            
            # Sort using Glang comparison semantics
            try:
                import functools
                sorted_elements.sort(key=functools.cmp_to_key(
                    lambda x, y: ListValue._glang_compare(x, y)
                ))
            except ValueError as e:
                from .errors import ArgumentError
                raise ArgumentError(f"sort() failed: {str(e)}", position)
            
            # Return new sorted list
            return ListValue(sorted_elements, target.constraint, position)
        
        # Functional programming methods using built-in transformations
        elif method_name == "map":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"map() takes 1 argument, got {len(args)}", position)
            
            if not isinstance(args[0], StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"map() argument must be a string naming a transformation", position)
            
            transform_name = args[0].value
            
            # Import transformation registry
            from .transformations import transformation_registry
            transform_func = transformation_registry.get_transformation(transform_name)
            
            if not transform_func:
                from .errors import ArgumentError
                available = ", ".join(sorted(transformation_registry.transformations.keys()))
                raise ArgumentError(
                    f"Unknown transformation '{transform_name}'. Available: {available}", 
                    position
                )
            
            # Apply transformation to each element
            result_elements = []
            for element in target.elements:
                try:
                    transformed = transform_func(element)
                    result_elements.append(transformed)
                except ValueError as e:
                    raise RuntimeError(
                        f"Transformation '{transform_name}' failed: {e}",
                        position
                    )
            
            # Infer constraint from first result element if any
            new_constraint = None
            if result_elements:
                first_type = result_elements[0].get_type()
                # Check all elements are same type
                if all(elem.get_type() == first_type for elem in result_elements):
                    new_constraint = first_type
            
            return ListValue(result_elements, new_constraint, position)
        
        elif method_name == "filter":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"filter() takes 1 argument, got {len(args)}", position)
            
            if not isinstance(args[0], StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"filter() argument must be a string naming a predicate", position)
            
            predicate_name = args[0].value
            
            # Import transformation registry
            from .transformations import transformation_registry
            predicate_func = transformation_registry.get_predicate(predicate_name)
            
            if not predicate_func:
                from .errors import ArgumentError
                available = ", ".join(sorted(transformation_registry.predicates.keys()))
                raise ArgumentError(
                    f"Unknown predicate '{predicate_name}'. Available: {available}", 
                    position
                )
            
            # Filter elements using predicate
            result_elements = []
            for element in target.elements:
                try:
                    if predicate_func(element):
                        result_elements.append(element)
                except Exception as e:
                    raise RuntimeError(
                        f"Predicate '{predicate_name}' failed: {e}",
                        position
                    )
            
            # Maintain original constraint
            return ListValue(result_elements, target.constraint, position)
        
        elif method_name == "each":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"each() takes 1 argument, got {len(args)}", position)
            
            if not isinstance(args[0], StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"each() argument must be a string naming an action", position)
            
            action_name = args[0].value
            
            # Special built-in actions
            if action_name == "print":
                # Print each element
                for element in target.elements:
                    print(element.to_display_string())
            else:
                from .errors import ArgumentError
                raise ArgumentError(
                    f"Unknown action '{action_name}'. Available: print", 
                    position
                )
            
            # Return original list for chaining
            return target
        
        # Aliases for functional methods
        elif method_name == "select":
            # Alias for filter (Ruby-style)
            return self._dispatch_list_method(target, "filter", args, position)
        
        elif method_name == "reject":
            # Opposite of filter - keep elements that fail predicate
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"reject() takes 1 argument, got {len(args)}", position)
            
            if not isinstance(args[0], StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"reject() argument must be a string naming a predicate", position)
            
            predicate_name = args[0].value
            
            # Import transformation registry
            from .transformations import transformation_registry
            predicate_func = transformation_registry.get_predicate(predicate_name)
            
            if not predicate_func:
                from .errors import ArgumentError
                available = ", ".join(sorted(transformation_registry.predicates.keys()))
                raise ArgumentError(
                    f"Unknown predicate '{predicate_name}'. Available: {available}", 
                    position
                )
            
            # Filter elements using inverted predicate
            result_elements = []
            for element in target.elements:
                try:
                    if not predicate_func(element):  # Note the NOT
                        result_elements.append(element)
                except Exception as e:
                    raise RuntimeError(
                        f"Predicate '{predicate_name}' failed: {e}",
                        position
                    )
            
            # Maintain original constraint
            return ListValue(result_elements, target.constraint, position)
        
        elif method_name == "can_accept":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"can_accept() takes exactly one argument, got {len(args)}", position)
            
            # Check if the list can accept the given element
            can_accept, message = target.can_accept_element(args[0])
            if can_accept:
                return BooleanValue(True, position)
            else:
                return StringValue(message, position)

        # Graph-specific methods (for ListValue)
        elif method_name == "add_edge":
            if len(args) < 2:
                from .errors import ArgumentError
                raise ArgumentError(f"add_edge() takes at least 2 arguments (from_index, to_index), got {len(args)}", position)

            from_index = args[0]
            to_index = args[1]
            relationship = args[2] if len(args) > 2 else StringValue("related")

            # Validate arguments
            if not isinstance(from_index, NumberValue) or not isinstance(to_index, NumberValue):
                raise RuntimeError("add_edge() requires numeric indices", position)
            if not isinstance(relationship, StringValue):
                raise RuntimeError("add_edge() relationship must be a string", position)

            # Call the graph method
            success = target.add_edge(int(from_index.value), int(to_index.value), relationship.value)
            return BooleanValue(success, position)

        elif method_name == "get_connected_to":
            if len(args) < 1:
                from .errors import ArgumentError
                raise ArgumentError(f"get_connected_to() takes at least 1 argument (index), got {len(args)}", position)

            index = args[0]
            relationship = args[1] if len(args) > 1 else StringValue("related")

            # Validate arguments
            if not isinstance(index, NumberValue):
                raise RuntimeError("get_connected_to() requires a numeric index", position)
            if not isinstance(relationship, StringValue):
                raise RuntimeError("get_connected_to() relationship must be a string", position)

            # Call the graph method and return indices as a list
            connected_indices = target.get_connected_to(int(index.value), relationship.value)
            index_values = [NumberValue(idx) for idx in connected_indices]
            return ListValue(index_values, "num", position)

        elif method_name == "to_graph":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"to_graph() takes exactly 1 argument (pattern), got {len(args)}", position)

            pattern = args[0]
            if not isinstance(pattern, StringValue):
                raise RuntimeError("to_graph() pattern must be a string", position)

            # Call the graph method
            result_graph = target.to_graph(pattern.value)
            return result_graph

        # Edge inspection methods
        elif method_name == "get_edges":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"get_edges() takes no arguments, got {len(args)}", position)
            # Get all custom edges as (from_index, to_index, relationship) tuples
            edges = target.get_edges()
            edge_lists = []
            for from_idx, to_idx, relationship in edges:
                edge_data = [NumberValue(from_idx), NumberValue(to_idx), StringValue(relationship)]
                edge_lists.append(ListValue(edge_data, None, position))
            return ListValue(edge_lists, None, position)


        elif method_name == "can_add_edge":
            if len(args) < 2:
                from .errors import ArgumentError
                raise ArgumentError(f"can_add_edge() takes at least 2 arguments (from_index, to_index), got {len(args)}", position)
            from_index = args[0]
            to_index = args[1]
            relationship = args[2] if len(args) > 2 else StringValue("related")
            # Validate arguments
            if not isinstance(from_index, NumberValue) or not isinstance(to_index, NumberValue):
                raise RuntimeError("can_add_edge() requires numeric indices", position)
            if not isinstance(relationship, StringValue):
                raise RuntimeError("can_add_edge() relationship must be a string", position)
            # Check if edge can be added
            can_add, reason = target.can_add_edge(int(from_index.value), int(to_index.value), relationship.value)
            return BooleanValue(can_add, position)

        # Better named edge/node counting methods
        elif method_name == "count_edges":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"count_edges() takes no arguments, got {len(args)}", position)
            count = target.count_edges()
            return NumberValue(count, position)

        elif method_name == "count_nodes":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"count_nodes() takes no arguments, got {len(args)}", position)
            count = target.count_nodes()
            return NumberValue(count, position)

        elif method_name == "graph_summary":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"graph_summary() takes no arguments, got {len(args)}", position)
            summary = target.graph_summary()
            # Convert to a hash structure that Glang can use
            hash_pairs = []
            for key, value in summary.items():
                if isinstance(value, list):
                    # Convert lists to ListValue
                    list_elements = [StringValue(str(v)) for v in value]
                    hash_pairs.append((key, ListValue(list_elements)))
                else:
                    # Convert other values to strings
                    hash_pairs.append((key, StringValue(str(value))))
            return HashValue(hash_pairs, position)

        # Control layer access methods (Layer 3)
        elif method_name == "get_active_rules":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"get_active_rules() takes no arguments, got {len(args)}", position)
            active_rules = target.get_active_rules()
            rule_strings = [StringValue(rule) for rule in active_rules]
            return ListValue(rule_strings, "string", position)

        elif method_name == "get_rule_status":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"get_rule_status() takes exactly 1 argument (rule_name), got {len(args)}", position)
            rule_name = args[0]
            if not isinstance(rule_name, StringValue):
                raise RuntimeError("get_rule_status() rule_name must be a string", position)
            status = target.get_rule_status(rule_name.value)
            return StringValue(status, position)

        elif method_name == "disable_rule":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"disable_rule() takes exactly 1 argument (rule_name), got {len(args)}", position)
            rule_name = args[0]
            if not isinstance(rule_name, StringValue):
                raise RuntimeError("disable_rule() rule_name must be a string", position)
            return target.disable_rule(rule_name.value)

        elif method_name == "enable_rule":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"enable_rule() takes exactly 1 argument (rule_name), got {len(args)}", position)
            rule_name = args[0]
            if not isinstance(rule_name, StringValue):
                raise RuntimeError("enable_rule() rule_name must be a string", position)
            return target.enable_rule(rule_name.value)

        # Behavior management methods (from GraphContainer mixin)
        elif method_name == "add_rule":
            if len(args) < 1:
                from .errors import ArgumentError
                raise ArgumentError(f"add_rule() takes at least 1 argument (behavior name), got {len(args)}", position)
            return target.add_rule(*args)

        elif method_name == "remove_rule":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"remove_rule() takes 1 argument, got {len(args)}", position)
            return target.remove_rule(args[0])

        elif method_name == "has_rule":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"has_rule() takes 1 argument, got {len(args)}", position)
            return target.has_rule(args[0])

        elif method_name == "get_rules":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"get_rules() takes no arguments, got {len(args)}", position)
            return target.get_rules()

        elif method_name == "clear_rules":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"clear_rules() takes no arguments, got {len(args)}", position)
            return target.clear_rules()

        # Metadata layer methods
        elif method_name == "set_names":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"set_names() takes 1 argument, got {len(args)}", position)

            # Convert Glang list to Python list of optional strings
            names_list = args[0]
            if not isinstance(names_list, ListValue):
                from .errors import ArgumentError
                raise ArgumentError(f"set_names() expects a list, got {names_list.get_type()}", position)

            python_names = []
            for elem in names_list.elements:
                if isinstance(elem, StringValue):
                    python_names.append(elem.value)
                elif isinstance(elem, NoneValue):
                    python_names.append(None)
                else:
                    from .errors import ArgumentError
                    raise ArgumentError(f"Names must be strings or nil, got {elem.get_type()}", position)

            return target.set_names(python_names)

        elif method_name == "get_names":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"get_names() takes no arguments, got {len(args)}", position)

            # Convert Python list back to Glang list
            python_names = target.get_names()
            glang_names = []
            for name in python_names:
                if name is None:
                    glang_names.append(NoneValue(position))
                else:
                    glang_names.append(StringValue(name, position))

            return ListValue(glang_names, None, position)

        elif method_name == "has_names":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"has_names() takes no arguments, got {len(args)}", position)

            return BooleanValue(target.has_names(), position)

        elif method_name == "get_name":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"get_name() takes 1 argument, got {len(args)}", position)

            index_arg = args[0]
            if not isinstance(index_arg, NumberValue):
                from .errors import ArgumentError
                raise ArgumentError(f"get_name() expects integer index, got {index_arg.get_type()}", position)

            name = target.get_name(int(index_arg.value))
            if name is None:
                return NoneValue(position)
            else:
                    return StringValue(name, position)

        elif method_name == "set_name":
            if len(args) != 2:
                from .errors import ArgumentError
                raise ArgumentError(f"set_name() takes 2 arguments, got {len(args)}", position)

            index_arg, name_arg = args
            if not isinstance(index_arg, NumberValue):
                from .errors import ArgumentError
                raise ArgumentError(f"set_name() expects integer index, got {index_arg.get_type()}", position)

            if isinstance(name_arg, StringValue):
                name = name_arg.value
            elif isinstance(name_arg, NoneValue):
                name = None
            else:
                from .errors import ArgumentError
                raise ArgumentError(f"set_name() expects string or nil, got {name_arg.get_type()}", position)

            return target.set_name(int(index_arg.value), name)

        elif method_name == "metadata":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"metadata property takes no arguments, got {len(args)}", position)

            # Return the metadata layer as a special metadata value that can be used for get/set operations
            # For now, return a placeholder since we need to implement a MetadataValue type
            return StringValue("metadata_access_placeholder", position)

        # Check if it's a universal method
        elif method_name in ['freeze', 'is_frozen', 'contains_frozen']:
            return self._dispatch_universal_method(target, method_name, args, position)

        # Missing methods that are listed but not implemented
        elif method_name == "visualize_structure":
            if len(args) > 1:
                from .errors import ArgumentError
                raise ArgumentError(f"visualize_structure() takes 0-1 arguments, got {len(args)}", position)
            format_arg = args[0] if len(args) == 1 else StringValue("text")
            if not isinstance(format_arg, StringValue):
                raise RuntimeError("visualize_structure() format must be a string", position)
            result = target.visualize_structure(format_arg.value)
            return StringValue(result, position)

        elif method_name == "visualize":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"visualize() takes no arguments, got {len(args)}", position)
            # Show shape/structure representation with names if available
            size = len(target.elements)

            # Get display names (prefer names over values)
            def get_display_name(index, elem):
                if hasattr(target, 'has_names') and target.has_names():
                    names = target.get_names()
                    if index < len(names) and names[index] is not None:
                        return names[index]
                return elem.to_display_string()

            if size <= 10:
                # Small list: show actual shape
                shape = " → ".join([get_display_name(i, elem) for i, elem in enumerate(target.elements)])
                result = f"[{shape}]"
            else:
                # Large list: show abbreviated shape
                first_three = " → ".join([get_display_name(i, elem) for i, elem in enumerate(target.elements[:3])])
                last_three = " → ".join([get_display_name(size-3+i, elem) for i, elem in enumerate(target.elements[-3:])])
                result = f"[{first_three} → ... → {last_three}] ({size} elements)"
            return StringValue(result, position)

        elif method_name == "view":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"view() takes no arguments, got {len(args)}", position)
            # Show names and values together
            if hasattr(target, 'has_names') and target.has_names():
                names = target.get_names()
                elements = []
                for i, elem in enumerate(target.elements):
                    if i < len(names) and names[i] is not None:
                        elements.append(f'"{names[i]}": {elem.to_display_string()}')
                    else:
                        elements.append(elem.to_display_string())
                result = "[" + ", ".join(elements) + "]"
            else:
                # No names, just show values
                result = "[" + ", ".join([elem.to_display_string() for elem in target.elements]) + "]"
            return StringValue(result, position)

        # Snake_case alternatives for verbose methods
        # Proper snake_case method
        elif method_name == "index_of":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"index_of() takes 1 argument, got {len(args)}", position)

            search_value = args[0]
            for i, element in enumerate(target.elements):
                if element == search_value:
                    return NumberValue(i, position)

            # Return -1 if not found (following common convention)
            return NumberValue(-1, position)

        # Clean snake_case methods (no more "get_" delegation)
        elif method_name == "edges":
            # Get custom edges directly
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"edges() takes no arguments, got {len(args)}", position)
            edges = target.get_edges()  # Call the underlying method directly
            edge_lists = []
            for from_idx, to_idx, relationship in edges:
                edge_data = [NumberValue(from_idx), NumberValue(to_idx), StringValue(relationship)]
                edge_lists.append(ListValue(edge_data, None, position))
            return ListValue(edge_lists, None, position)

        elif method_name == "connected_to":
            # Get connected elements directly
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"connected_to() takes 1 argument, got {len(args)}", position)
            index_arg = args[0]
            if not isinstance(index_arg, NumberValue):
                from .errors import ArgumentError
                raise ArgumentError(f"connected_to() expects integer index, got {index_arg.get_type()}", position)
            connected = target.get_connected_to(int(index_arg.value))
            connected_values = [NumberValue(idx) for idx in connected]
            return ListValue(connected_values, "num", position)

        elif method_name == "names":
            # Overloaded method: get names (no args) or set names (with args)
            if len(args) == 0:
                # Get node names directly
                names = target.get_names()
                if names is None:
                    return ListValue([], "string", position)
                name_values = [StringValue(name) if name is not None else NoneValue() for name in names]
                return ListValue(name_values, None, position)
            elif len(args) == 1:
                # Set names (replaces set_names)
                names_list = args[0]
                if not isinstance(names_list, ListValue):
                    from .errors import ArgumentError
                    raise ArgumentError(f"names() expects a list, got {names_list.get_type()}", position)

                # Convert Glang list to Python list of optional strings
                python_names = []
                for elem in names_list.elements:
                    if isinstance(elem, StringValue):
                        python_names.append(elem.value)
                    elif isinstance(elem, NoneValue):
                        python_names.append(None)
                    else:
                        from .errors import ArgumentError
                        raise ArgumentError(f"Names must be strings or nil, got {elem.get_type()}", position)

                target.set_names(python_names)
                return names_list  # Return the list that was set
            else:
                from .errors import ArgumentError
                raise ArgumentError(f"names() takes 0 or 1 arguments, got {len(args)}", position)

        elif method_name == "rules":
            # Get active rules directly
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"rules() takes no arguments, got {len(args)}", position)
            rules = target.get_rules()
            rule_values = [StringValue(rule) for rule in rules]
            return ListValue(rule_values, "string", position)

        else:
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, "list", position)
    
    def _dispatch_string_method(self, target: StringValue, method_name: str, 
                               args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Handle string method calls."""
        
        # Type conversion methods
        if method_name == "to_string":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"to_string() takes no arguments, got {len(args)}", position)
            return target  # Already a string
        
        elif method_name == "to_num":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"to_num() takes no arguments, got {len(args)}", position)
            try:
                # Try int first, then float
                if '.' in target.value:
                    return NumberValue(float(target.value), position)
                else:
                    return NumberValue(int(target.value), position)
            except ValueError:
                raise RuntimeError(f"Cannot convert '{target.value}' to number", position)
        
        elif method_name == "to_bool":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"to_bool() takes no arguments, got {len(args)}", position)
            # Empty string is false, non-empty is true
            return BooleanValue(len(target.value) > 0, position)
        
        # Length method
        elif method_name == "length":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"length() takes no arguments, got {len(args)}", position)
            
            return NumberValue(len(target.value), position)
        
        # Contains method - unified interface with backward compatibility
        elif method_name == "contains":
            if len(args) == 1:
                # Backward compatibility: substring search
                if not isinstance(args[0], StringValue):
                    from .errors import ArgumentError
                    raise ArgumentError(f"contains() argument must be string, got {args[0].get_type()}", position)
                
                return BooleanValue(args[0].value in target.value, position)
            
            elif len(args) >= 2:
                # New unified interface: contains(mode, pattern, ...)
                if not isinstance(args[0], StringValue):
                    from .errors import ArgumentError
                    raise ArgumentError(f"contains() first argument (mode) must be string, got {args[0].get_type()}", position)
                
                mode = args[0].value.lower()
                text = target.value
                
                def check_pattern(pattern_type):
                    """Helper function to check if text matches a pattern type"""
                    pattern_type = pattern_type.lower()
                    if pattern_type == "digits" or pattern_type == "numbers":
                        return any(c.isdigit() for c in text)
                    elif pattern_type == "letters":
                        return any(c.isalpha() for c in text)
                    elif pattern_type == "uppercase":
                        return any(c.isupper() for c in text)
                    elif pattern_type == "lowercase":
                        return any(c.islower() for c in text)
                    elif pattern_type == "spaces" or pattern_type == "whitespace":
                        return any(c.isspace() for c in text)
                    elif pattern_type == "punctuation":
                        import string
                        return any(c in string.punctuation for c in text)
                    elif pattern_type == "symbols":
                        import string
                        return any(c in string.punctuation + "~`!@#$%^&*()_+-=[]{}|;':\",./<>?" for c in text)
                    elif pattern_type == "alphanumeric":
                        return any(c.isalnum() for c in text)
                    else:
                        from .errors import ArgumentError
                        raise ArgumentError(f"Unknown pattern type '{pattern_type}'. Available: digits, letters, uppercase, lowercase, spaces, punctuation, symbols, alphanumeric", position)
                
                def char_matches_pattern(char, pattern_type):
                    """Helper function to check if a single character matches a pattern type"""
                    pattern_type = pattern_type.lower()
                    if pattern_type == "digits" or pattern_type == "numbers":
                        return char.isdigit()
                    elif pattern_type == "letters":
                        return char.isalpha()
                    elif pattern_type == "uppercase":
                        return char.isupper()
                    elif pattern_type == "lowercase":
                        return char.islower()
                    elif pattern_type == "spaces" or pattern_type == "whitespace":
                        return char.isspace()
                    elif pattern_type == "punctuation":
                        import string
                        return char in string.punctuation
                    elif pattern_type == "symbols":
                        import string
                        return char in string.punctuation + "~`!@#$%^&*()_+-=[]{}|;':\",./<>?"
                    elif pattern_type == "alphanumeric":
                        return char.isalnum()
                    return False
                
                if mode == "any":
                    if len(args) != 2:
                        from .errors import ArgumentError
                        raise ArgumentError(f"contains('any', pattern) takes exactly 2 arguments, got {len(args)}", position)
                    
                    if not isinstance(args[1], StringValue):
                        from .errors import ArgumentError
                        raise ArgumentError(f"contains() pattern must be string, got {args[1].get_type()}", position)
                    
                    result = check_pattern(args[1].value)
                    return BooleanValue(result, position)
                
                elif mode == "all":
                    for i in range(1, len(args)):
                        if not isinstance(args[i], StringValue):
                            from .errors import ArgumentError
                            raise ArgumentError(f"contains() pattern must be string, got {args[i].get_type()}", position)
                        
                        if not check_pattern(args[i].value):
                            return BooleanValue(False, position)
                    
                    return BooleanValue(True, position)
                
                elif mode == "only":
                    pattern_types = [args[i].value for i in range(1, len(args))]
                    
                    for char in text:
                        char_matches_any = False
                        for pattern_type in pattern_types:
                            if char_matches_pattern(char, pattern_type):
                                char_matches_any = True
                                break
                        
                        if not char_matches_any:
                            return BooleanValue(False, position)
                    
                    return BooleanValue(True, position)
                
                else:
                    from .errors import ArgumentError
                    raise ArgumentError(f"contains() mode must be 'any', 'all', or 'only', got '{mode}'", position)
            
            else:
                from .errors import ArgumentError
                raise ArgumentError(f"contains() takes at least 1 argument (substring) or 2+ arguments (mode, pattern, ...), got {len(args)}", position)
        
        # Upper case methods (up and toUpper as alias)
        elif method_name in ["up", "toUpper"]:
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"{method_name}() takes no arguments, got {len(args)}", position)
            
            return target.to_upper()
        
        # Lower case methods (down and toLower as alias)
        elif method_name in ["down", "toLower"]:
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"{method_name}() takes no arguments, got {len(args)}", position)
            
            return target.to_lower()
        
        # Split method
        elif method_name == "split":
            if len(args) > 1:
                from .errors import ArgumentError
                raise ArgumentError(f"split() takes 0 or 1 argument, got {len(args)}", position)
            
            # Default delimiter is space
            delimiter = StringValue(" ")
            if len(args) == 1:
                if not isinstance(args[0], StringValue):
                    from .errors import ArgumentError
                    raise ArgumentError(f"split() argument must be string, got {args[0].get_type()}", position)
                delimiter = args[0]
            
            # Use character node-based split
            return target.split(delimiter)
        
        # Trim method (remove leading/trailing whitespace)
        elif method_name == "trim":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"trim() takes no arguments, got {len(args)}", position)
            
            return target.trim()
        
        # Join method (join elements of a list with this string as separator)
        elif method_name == "join":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"join() takes 1 argument, got {len(args)}", position)
            
            # Using already imported ListValue
            if not isinstance(args[0], (ListValue, ListValue)):
                from .errors import ArgumentError
                raise ArgumentError(f"join() argument must be list, got {args[0].get_type()}", position)
            
            # Convert all list elements to StringValues for character node-based joining
            list_arg = args[0]
            string_values = []
            for element in list_arg.elements:
                # Convert element to string if it's not already
                if isinstance(element, StringValue):
                    string_values.append(element)
                else:
                    string_values.append(StringValue(element.to_display_string(), element.position))
            
            # Create temporary ListValue and use character node-based join
            string_list = ListValue(string_values, "string", position)
            return target.join(string_list)
        
        # Pattern matching methods
        elif method_name == "matches":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"matches() takes 1 argument, got {len(args)}", position)
            
            if not isinstance(args[0], StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"matches() argument must be string, got {args[0].get_type()}", position)
            
            import re
            pattern = args[0].value
            try:
                result = bool(re.search(pattern, target.value))
                return BooleanValue(result, position)
            except re.error as e:
                from .errors import ArgumentError
                raise ArgumentError(f"Invalid regex pattern: {e}", position)
        
        elif method_name == "replace":
            if len(args) != 2:
                from .errors import ArgumentError
                raise ArgumentError(f"replace() takes 2 arguments, got {len(args)}", position)
            
            if not isinstance(args[0], StringValue) or not isinstance(args[1], StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"replace() arguments must be strings", position)
            
            import re
            pattern = args[0].value
            replacement = args[1].value
            try:
                result = re.sub(pattern, replacement, target.value)
                return StringValue(result, position)
            except re.error as e:
                from .errors import ArgumentError
                raise ArgumentError(f"Invalid regex pattern: {e}", position)
        
        elif method_name == "find_all" or method_name == "findAll":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"find_all() takes 1 argument, got {len(args)}", position)
            
            if not isinstance(args[0], StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"find_all() argument must be string, got {args[0].get_type()}", position)
            
            import re
            pattern = args[0].value
            try:
                matches = re.findall(pattern, target.value)
                string_matches = [StringValue(match, position) for match in matches]
                return ListValue(string_matches, "string", position)
            except re.error as e:
                from .errors import ArgumentError
                raise ArgumentError(f"Invalid regex pattern: {e}", position)
        
        # Unified extraction method
        elif method_name == "extract":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"extract() takes 1 argument (pattern), got {len(args)}", position)
            
            if not isinstance(args[0], StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"extract() pattern must be string, got {args[0].get_type()}", position)
            
            pattern_type = args[0].value.lower()
            text = target.value
            
            if pattern_type == "numbers":
                import re
                # Find numbers including integers and floats
                number_pattern = r'-?\d+\.?\d*'
                matches = re.findall(number_pattern, text)
                # Filter out empty strings and lone dots
                valid_matches = [match for match in matches if match and match != '.' and not match.endswith('.')]
                result_elements = [StringValue(match, position) for match in valid_matches]
                return ListValue(result_elements, "string", position)
            
            elif pattern_type == "words":
                import re
                # Extract sequences of letters (word characters)
                word_pattern = r'[a-zA-Z]+'
                matches = re.findall(word_pattern, text)
                result_elements = [StringValue(match, position) for match in matches]
                return ListValue(result_elements, "string", position)
            
            elif pattern_type == "emails":
                import re
                # Basic email pattern - not perfect but covers most common cases
                email_pattern = r'[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}'
                matches = re.findall(email_pattern, text)
                result_elements = [StringValue(match, position) for match in matches]
                return ListValue(result_elements, "string", position)
            
            else:
                from .errors import ArgumentError
                raise ArgumentError(f"Unknown extraction pattern '{pattern_type}'. Available: numbers, words, emails", position)
        
        # Validation methods
        elif method_name == "is_email":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"is_email() takes no arguments, got {len(args)}", position)
            
            import re
            # Basic email validation - covers most common cases
            email_pattern = r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'
            result = bool(re.match(email_pattern, target.value))
            return BooleanValue(result, position)
        
        elif method_name == "is_number":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"is_number() takes no arguments, got {len(args)}", position)
            
            try:
                float(target.value)
                return BooleanValue(True, position)
            except ValueError:
                return BooleanValue(False, position)
        
        elif method_name == "is_url":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"is_url() takes no arguments, got {len(args)}", position)
            
            import re
            # Basic URL pattern - covers http, https, ftp
            url_pattern = r'^https?://[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}(/.*)?$|^ftp://[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}(/.*)?$'
            result = bool(re.match(url_pattern, target.value))
            return BooleanValue(result, position)
        
        # Enhanced split methods
        elif method_name == "split_on_any":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"split_on_any() takes 1 argument, got {len(args)}", position)
            
            if not isinstance(args[0], StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"split_on_any() argument must be string, got {args[0].get_type()}", position)
            
            import re
            # Escape special regex characters and create character class
            delimiter_chars = args[0].value
            escaped_chars = re.escape(delimiter_chars)
            pattern = f'[{escaped_chars}]+'
            
            parts = re.split(pattern, target.value)
            # Filter out empty strings
            filtered_parts = [part for part in parts if part]
            result_elements = [StringValue(part, position) for part in filtered_parts]
            return ListValue(result_elements, "string", position)
        
        # Count methods
        elif method_name == "count":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"count() takes 1 argument, got {len(args)}", position)
            
            if not isinstance(args[0], StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"count() argument must be string, got {args[0].get_type()}", position)
            
            pattern_type = args[0].value.lower()
            text = target.value
            count = 0
            
            if pattern_type == "digits" or pattern_type == "numbers":
                count = sum(1 for c in text if c.isdigit())
            elif pattern_type == "letters":
                count = sum(1 for c in text if c.isalpha())
            elif pattern_type == "uppercase":
                count = sum(1 for c in text if c.isupper())
            elif pattern_type == "lowercase":
                count = sum(1 for c in text if c.islower())
            elif pattern_type == "spaces" or pattern_type == "whitespace":
                count = sum(1 for c in text if c.isspace())
            elif pattern_type == "punctuation":
                count = sum(1 for c in text if c in __import__('string').punctuation)
            elif pattern_type == "symbols":
                import string
                count = sum(1 for c in text if c in string.punctuation + "~`!@#$%^&*()_+-=[]{}|;':\",./<>?")
            elif pattern_type == "alphanumeric":
                count = sum(1 for c in text if c.isalnum())
            elif pattern_type == "words":
                import re
                # Count sequences of letters (word characters)
                words = re.findall(r'[a-zA-Z]+', text)
                count = len(words)
            elif len(pattern_type) == 1:
                # Single character count
                count = text.count(pattern_type)
            else:
                from .errors import ArgumentError
                raise ArgumentError(f"Unknown pattern type or multi-character string '{pattern_type}'. Use count_chars() for multi-character strings or use pattern types: digits, letters, uppercase, lowercase, spaces, punctuation, symbols, alphanumeric, words", position)
            
            return NumberValue(count, position)
        
        elif method_name == "count_chars":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"count_chars() takes 1 argument, got {len(args)}", position)
            
            if not isinstance(args[0], StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"count_chars() argument must be string, got {args[0].get_type()}", position)
            
            substring = args[0].value
            count = target.value.count(substring)
            return NumberValue(count, position)
        
        
        # Find first methods
        elif method_name == "find_first":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"find_first() takes 1 argument, got {len(args)}", position)
            
            if not isinstance(args[0], StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"find_first() argument must be string, got {args[0].get_type()}", position)
            
            pattern_type = args[0].value.lower()
            text = target.value
            
            for i, char in enumerate(text):
                if pattern_type == "digits" or pattern_type == "numbers":
                    if char.isdigit():
                        return NumberValue(i, position)
                elif pattern_type == "letters":
                    if char.isalpha():
                        return NumberValue(i, position)
                elif pattern_type == "uppercase":
                    if char.isupper():
                        return NumberValue(i, position)
                elif pattern_type == "lowercase":
                    if char.islower():
                        return NumberValue(i, position)
                elif pattern_type == "spaces" or pattern_type == "whitespace":
                    if char.isspace():
                        return NumberValue(i, position)
                elif pattern_type == "punctuation":
                    if char in __import__('string').punctuation:
                        return NumberValue(i, position)
                elif pattern_type == "symbols":
                    import string
                    if char in string.punctuation + "~`!@#$%^&*()_+-=[]{}|;':\",./<>?":
                        return NumberValue(i, position)
                elif pattern_type == "alphanumeric":
                    if char.isalnum():
                        return NumberValue(i, position)
                elif len(pattern_type) == 1:
                    if char == pattern_type:
                        return NumberValue(i, position)
                else:
                    from .errors import ArgumentError
                    raise ArgumentError(f"Unknown pattern type or multi-character string '{pattern_type}'. Use find_first_char() for multi-character strings or use pattern types: digits, letters, uppercase, lowercase, spaces, punctuation, symbols, alphanumeric", position)
            
            # Return -1 if not found (consistent with indexOf)
            return NumberValue(-1, position)
        
        elif method_name == "find_first_char":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"find_first_char() takes 1 argument, got {len(args)}", position)
            
            if not isinstance(args[0], StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"find_first_char() argument must be string, got {args[0].get_type()}", position)
            
            substring = args[0].value
            index = target.value.find(substring)
            return NumberValue(index, position)  # Returns -1 if not found
        
        
        # Graph operations that work on character level
        elif method_name == "reverse":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"reverse() takes no arguments, got {len(args)}", position)
            
            # Use char nodes for graph-like operation
            char_nodes = target.to_char_nodes()
            reversed_nodes = list(reversed(char_nodes))
            return target.from_char_nodes(reversed_nodes)
        
        elif method_name == "unique":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"unique() takes no arguments, got {len(args)}", position)
            
            # Use char nodes to remove duplicate characters while preserving order
            char_nodes = target.to_char_nodes()
            seen = set()
            unique_nodes = []
            for node in char_nodes:
                if node.value not in seen:
                    seen.add(node.value)
                    unique_nodes.append(node)
            return target.from_char_nodes(unique_nodes)
        
        elif method_name == "chars":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"chars() takes no arguments, got {len(args)}", position)
            
            # Return list of individual characters as strings
            char_nodes = target.to_char_nodes()
            char_strings = [StringValue(node.value, position) for node in char_nodes]
            return ListValue(char_strings, "string", position)
        
        elif method_name == "to_time":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"to_time() takes no arguments, got {len(args)}", position)
            # Parse time string to time value - similar to Time.from_string
            try:
                import datetime as python_datetime
                time_str_val = target.value
                
                # Support basic ISO format: "2025-01-15T14:30:00"
                dt = python_datetime.datetime.fromisoformat(time_str_val.replace('Z', '+00:00'))
                if dt.tzinfo is None:
                    dt = dt.replace(tzinfo=python_datetime.timezone.utc)
                timestamp = dt.timestamp()
                from .values import TimeValue
                return TimeValue(timestamp, position)
            except ValueError:
                raise RuntimeError(f"Invalid time format: {time_str_val}. Expected ISO format like '2025-01-15T14:30:00'", position)
            except Exception as e:
                raise RuntimeError(f"Failed to parse time string: {str(e)}", position)
        
        # Starts with and ends with methods
        elif method_name == "starts_with":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"starts_with() takes 1 argument (prefix), got {len(args)}", position)
            
            if not isinstance(args[0], StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"starts_with() argument must be string, got {args[0].get_type()}", position)
            
            return target.starts_with(args[0])
        
        elif method_name == "ends_with":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"ends_with() takes 1 argument (suffix), got {len(args)}", position)
            
            if not isinstance(args[0], StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"ends_with() argument must be string, got {args[0].get_type()}", position)
            
            return target.ends_with(args[0])
        
        # Check if it's a universal method
        elif method_name in ['freeze', 'is_frozen', 'contains_frozen']:
            return self._dispatch_universal_method(target, method_name, args, position)
        
        else:
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, "string", position)
    
    def _dispatch_num_method(self, target: NumberValue, method_name: str,
                            args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Handle number method calls."""
        import math
        
        # Type conversion methods
        if method_name == "to_string":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"to_string() takes no arguments, got {len(args)}", position)
            return StringValue(str(target.value), position)
        
        elif method_name == "to_bool":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"to_bool() takes no arguments, got {len(args)}", position)
            return BooleanValue(target.value != 0, position)
        
        elif method_name == "to_num":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"to_num() takes no arguments, got {len(args)}", position)
            return target  # Already a number
        
        # to() method for precision truncation
        elif method_name == "to":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"to() takes 1 argument, got {len(args)}", position)
            
            if not isinstance(args[0], NumberValue) or not isinstance(args[0].value, int):
                from .errors import ArgumentError
                raise ArgumentError(f"to() argument must be integer, got {args[0].get_type()}", position)
            
            digits = args[0].value
            if digits < 0:
                from .errors import ArgumentError
                raise ArgumentError(f"to() argument must be non-negative, got {digits}", position)
            
            # Truncate to specified decimal places (not rounding)
            value = target.value
            if digits == 0:
                # Truncate to integer
                truncated = int(value)
            else:
                # Truncate to specified decimal places
                multiplier = 10 ** digits
                truncated = int(value * multiplier) / multiplier
            
            return NumberValue(truncated, position)
        
        # Mathematical methods - basic functions
        elif method_name == "abs":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"abs() takes no arguments, got {len(args)}", position)
            return NumberValue(abs(target.value), position)
        
        elif method_name == "sqrt":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"sqrt() takes no arguments, got {len(args)}", position)
            try:
                return target.sqrt()
            except ValueError as e:
                raise RuntimeError(str(e), position)
        
        elif method_name == "log":
            if len(args) > 1:
                from .errors import ArgumentError
                raise ArgumentError(f"log() takes 0 or 1 arguments, got {len(args)}", position)
            if target.value <= 0:
                raise RuntimeError("Cannot take logarithm of non-positive number", position)
            
            if len(args) == 0:
                # Natural log (base e)
                return NumberValue(math.log(target.value), position)
            else:
                # Log with specified base
                if not isinstance(args[0], NumberValue):
                    from .errors import ArgumentError
                    raise ArgumentError(f"log() base must be number, got {args[0].get_type()}", position)
                base = args[0].value
                if base <= 0 or base == 1:
                    raise RuntimeError("Logarithm base must be positive and not equal to 1", position)
                return NumberValue(math.log(target.value, base), position)
        
        elif method_name == "pow":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"pow() takes 1 argument, got {len(args)}", position)
            if not isinstance(args[0], NumberValue):
                from .errors import ArgumentError
                raise ArgumentError(f"pow() exponent must be number, got {args[0].get_type()}", position)
            
            exponent = args[0].value
            try:
                result = pow(target.value, exponent)
                return NumberValue(result, position)
            except (ValueError, ZeroDivisionError) as e:
                raise RuntimeError(f"Power operation failed: {e}", position)
        
        # Rounding methods
        elif method_name == "rnd":
            # Round to nearest integer or specified decimal places
            if len(args) > 1:
                from .errors import ArgumentError
                raise ArgumentError(f"rnd() takes 0 or 1 arguments, got {len(args)}", position)
            
            if len(args) == 0:
                # Round to nearest integer
                return NumberValue(round(target.value), position)
            else:
                # Round to specified decimal places
                if not isinstance(args[0], NumberValue) or not isinstance(args[0].value, int):
                    from .errors import ArgumentError
                    raise ArgumentError(f"rnd() places must be integer, got {args[0].get_type()}", position)
                places = args[0].value
                if places < 0:
                    from .errors import ArgumentError
                    raise ArgumentError(f"rnd() places must be non-negative, got {places}", position)
                return NumberValue(round(target.value, places), position)
        
        elif method_name == "rnd_up":
            # Always round up (ceiling)
            if len(args) > 1:
                from .errors import ArgumentError
                raise ArgumentError(f"rnd_up() takes 0 or 1 arguments, got {len(args)}", position)
            
            if len(args) == 0:
                # Ceiling to integer
                return target.ceil()
            else:
                # Ceiling to specified decimal places
                if not isinstance(args[0], NumberValue) or not isinstance(args[0].value, int):
                    from .errors import ArgumentError
                    raise ArgumentError(f"rnd_up() places must be integer, got {args[0].get_type()}", position)
                places = args[0].value
                if places < 0:
                    from .errors import ArgumentError
                    raise ArgumentError(f"rnd_up() places must be non-negative, got {places}", position)
                multiplier = 10 ** places
                return NumberValue(math.ceil(target.value * multiplier) / multiplier, position)
        
        elif method_name == "rnd_dwn":
            # Always round down (floor)
            if len(args) > 1:
                from .errors import ArgumentError
                raise ArgumentError(f"rnd_dwn() takes 0 or 1 arguments, got {len(args)}", position)
            
            if len(args) == 0:
                # Floor to integer
                return NumberValue(math.floor(target.value), position)
            else:
                # Floor to specified decimal places
                if not isinstance(args[0], NumberValue) or not isinstance(args[0].value, int):
                    from .errors import ArgumentError
                    raise ArgumentError(f"rnd_dwn() places must be integer, got {args[0].get_type()}", position)
                places = args[0].value
                if places < 0:
                    from .errors import ArgumentError
                    raise ArgumentError(f"rnd_dwn() places must be non-negative, got {places}", position)
                multiplier = 10 ** places
                return NumberValue(math.floor(target.value * multiplier) / multiplier, position)
        
        elif method_name == "to_time":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"to_time() takes no arguments, got {len(args)}", position)
            # Convert timestamp to time value
            from .values import TimeValue
            return TimeValue(target.value, position)

        # Check if it's a universal method
        elif method_name in ['freeze', 'is_frozen', 'contains_frozen', 'node']:
            return self._dispatch_universal_method(target, method_name, args, position)
        
        else:
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, "num", position)
    
    def _dispatch_bool_method(self, target: BooleanValue, method_name: str,
                             args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Handle boolean method calls."""
        
        # Type conversion methods
        if method_name == "to_string":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"to_string() takes no arguments, got {len(args)}", position)
            return StringValue(str(target.value).lower(), position)  # "true" or "false"
        
        elif method_name == "to_num":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"to_num() takes no arguments, got {len(args)}", position)
            return NumberValue(1 if target.value else 0, position)
        
        elif method_name == "to_bool":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"to_bool() takes no arguments, got {len(args)}", position)
            return target  # Already a boolean
        
        # flip() and toggle() methods (aliases)
        elif method_name in ["flip", "toggle"]:
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"{method_name}() takes no arguments, got {len(args)}", position)
            
            return BooleanValue(not target.value, position)
        
        # numify() and toNum() methods (aliases)
        elif method_name in ["numify", "toNum"]:
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"{method_name}() takes no arguments, got {len(args)}", position)
            
            return NumberValue(1 if target.value else 0, position)
        
        # Check if it's a universal method
        elif method_name in ['freeze', 'is_frozen', 'contains_frozen']:
            return self._dispatch_universal_method(target, method_name, args, position)
        
        else:
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, "bool", position)
    
    def _dispatch_data_method(self, target: DataValue, method_name: str,
                             args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Handle data node method calls."""
        
        if method_name == "key":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"key() takes no arguments, got {len(args)}", position)
            
            # Return the key as a StringValue
            return target.get_key()
        
        elif method_name == "value":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"value() takes no arguments, got {len(args)}", position)
            
            # Return the value
            return target.get_value()
        
        elif method_name == "can_accept":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"can_accept() takes exactly one argument, got {len(args)}", position)
            
            # Check if the data node can accept the given value
            can_accept, message = target.can_accept_value(args[0])
            if can_accept:
                return BooleanValue(True, position)
            else:
                return StringValue(message, position)
        
        else:
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, "data", position)
    
    def _dispatch_hash_method(self, target: HashValue, method_name: str,
                            args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Handle hash method calls."""
        
        if method_name == "node":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"node() takes 1 argument, got {len(args)}", position)

            key_arg = args[0]
            if not isinstance(key_arg, StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"Hash key must be string, got {key_arg.get_type()}", position)

            value = target.get(key_arg.value)
            if value is None:
                raise RuntimeError(f"Key '{key_arg.value}' not found in hash", position)

            # Return the data node explicitly
            from .values import DataValue
            return DataValue(key_arg.value, value, target.constraint, position)
        
        elif method_name == "set":
            if len(args) != 2:
                from .errors import ArgumentError
                raise ArgumentError(f"set() takes 2 arguments, got {len(args)}", position)
            
            key_arg = args[0]
            value_arg = args[1]
            if not isinstance(key_arg, StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"Hash key must be string, got {key_arg.get_type()}", position)
            
            target.set(key_arg.value, value_arg)
            return target  # Return the map for chaining
        
        elif method_name == "has_key":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"has_key() takes 1 argument, got {len(args)}", position)
            
            key_arg = args[0]
            if not isinstance(key_arg, StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"Hash key must be string, got {key_arg.get_type()}", position)
            
            return BooleanValue(target.has_key(key_arg.value), position)
        
        elif method_name == "count_values":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"count_values() takes 1 argument, got {len(args)}", position)
            
            search_value = args[0]
            count = 0
            for value in target.values():
                # Compare the actual values (using Python equality)
                if value.to_python() == search_value.to_python():
                    count += 1
            
            return NumberValue(count, position)
        
        elif method_name == "keys":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"keys() takes no arguments, got {len(args)}", position)
            
            key_strings = [StringValue(key, position) for key in target.keys()]
            return ListValue(key_strings, None, position)
        
        elif method_name == "values":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"values() takes no arguments, got {len(args)}", position)
            
            return ListValue(target.values(), None, position)
        
        elif method_name == "remove":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"remove() takes 1 argument, got {len(args)}", position)
            
            key_arg = args[0]
            if not isinstance(key_arg, StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"Hash key must be string, got {key_arg.get_type()}", position)
            
            target.remove(key_arg.value)
            return target  # Return the map for chaining
        
        elif method_name == "empty":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"empty() takes no arguments, got {len(args)}", position)
            
            return BooleanValue(len(target.pairs) == 0, position)
        
        elif method_name == "merge":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"merge() takes 1 argument, got {len(args)}", position)
            
            other_map = args[0]
            # Using already imported HashValue
            if not isinstance(other_map, (HashValue, HashValue)):
                from .errors import ArgumentError
                raise ArgumentError(f"merge() requires a hash argument, got {other_map.get_type()}", position)
            
            # Check constraint compatibility
            if target.constraint and other_map.constraint and target.constraint != other_map.constraint:
                from .errors import TypeConstraintError
                raise TypeConstraintError(
                    f"Cannot merge hash<{other_map.constraint}> into hash<{target.constraint}>",
                    position
                )
            
            # Merge the other map into this map
            for key, value in other_map.pairs.items():
                # Validate constraint if target has one
                if not target.validate_constraint(value):
                    from .errors import TypeConstraintError
                    raise TypeConstraintError(
                        f"Value {value.to_display_string()} for key '{key}' violates hash<{target.constraint}> constraint",
                        value.position or position
                    )
                target.set(key, value)
            
            return StringValue(f"Merged {len(other_map.pairs)} pairs", position)
        
        elif method_name == "push":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"push() takes 1 argument, got {len(args)}", position)
            
            data_node = args[0]
            from .values import DataValue
            if not isinstance(data_node, DataValue):
                from .errors import ArgumentError
                raise ArgumentError(f"push() requires a data node argument, got {data_node.get_type()}", position)
            
            # Extract the key-value pair from the data node
            key = data_node.key
            value = data_node.value
            
            # Validate constraint against the data node's value (not the data node itself)
            if not target.validate_constraint(value):
                from .errors import TypeConstraintError
                raise TypeConstraintError(
                    f"Value {value.to_display_string()} violates hash<{target.constraint}> constraint",
                    value.position or position
                )
            
            # Set the unwrapped key-value pair in the map
            target.set(key, value)
            return StringValue(f"Pushed data node with key '{key}' and value {value.to_display_string()}", position)
        
        elif method_name == "pop":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"pop() takes 1 argument, got {len(args)}", position)
            
            key_arg = args[0]
            if not isinstance(key_arg, StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"Hash key must be string, got {key_arg.get_type()}", position)
            
            # Get the value before removing it
            value = target.get(key_arg.value)
            if value is None:
                # Return empty string for missing keys (consistent with get())
                return StringValue("", position)
            
            # Remove the key-value pair
            existed = target.remove(key_arg.value)
            
            # Return the value that was removed
            return value
        
        elif method_name == "can_accept":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"can_accept() takes exactly one argument, got {len(args)}", position)
            
            # Check if the map can accept the given value
            can_accept, message = target.can_accept_value(args[0])
            if can_accept:
                return BooleanValue(True, position)
            else:
                return StringValue(message, position)

        # Type conversion methods
        elif method_name == "to_string":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"to_string() takes no arguments, got {len(args)}", position)
            # Convert hash to string representation
            if len(target.pairs) == 0:
                return StringValue("{}", position)
            else:
                items = []
                for key in target.keys():
                    value = target.pairs.get(key)
                    items.append(f'"{key}": {value.to_display_string()}')
                return StringValue(f"{{{', '.join(items)}}}", position)

        elif method_name == "to_bool":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"to_bool() takes no arguments, got {len(args)}", position)
            # Non-empty hash is true, empty is false
            return BooleanValue(len(target.pairs) > 0, position)

        # Graph-specific methods (for HashValue)
        elif method_name == "add_value_edge":
            if len(args) < 2:
                from .errors import ArgumentError
                raise ArgumentError(f"add_value_edge() takes at least 2 arguments (from_key, to_key), got {len(args)}", position)

            from_key = args[0]
            to_key = args[1]
            relationship = args[2] if len(args) > 2 else StringValue("related")

            # Validate arguments
            if not isinstance(from_key, StringValue) or not isinstance(to_key, StringValue):
                raise RuntimeError("add_value_edge() requires string keys", position)
            if not isinstance(relationship, StringValue):
                raise RuntimeError("add_value_edge() relationship must be a string", position)

            # Call the graph method
            success = target.add_value_edge(from_key.value, to_key.value, relationship.value)
            return BooleanValue(success, position)

        elif method_name == "get_connected_keys":
            if len(args) < 1:
                from .errors import ArgumentError
                raise ArgumentError(f"get_connected_keys() takes at least 1 argument (key), got {len(args)}", position)

            key = args[0]
            relationship = args[1] if len(args) > 1 else StringValue("related")

            # Validate arguments
            if not isinstance(key, StringValue):
                raise RuntimeError("get_connected_keys() requires a string key", position)
            if not isinstance(relationship, StringValue):
                raise RuntimeError("get_connected_keys() relationship must be a string", position)

            # Call the graph method and return keys as a list
            connected_keys = target.get_connected_keys(key.value, relationship.value)
            key_values = [StringValue(k) for k in connected_keys]
            return ListValue(key_values, "string", position)

        # Edge inspection methods (for consistency with ListValue)
        elif method_name == "get_edges":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"get_edges() takes no arguments, got {len(args)}", position)
            # Get all custom edges as (from_key, to_key, relationship) tuples
            edges = target.get_edges()
            edge_lists = []
            for from_key, to_key, relationship in edges:
                edge_data = [StringValue(from_key), StringValue(to_key), StringValue(relationship)]
                edge_lists.append(ListValue(edge_data, None, position))
            return ListValue(edge_lists, None, position)

        elif method_name == "get_edge_count":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"get_edge_count() takes no arguments, got {len(args)}", position)
            return NumberValue(target.get_edge_count(), position)

        elif method_name == "can_add_edge":
            if len(args) < 2 or len(args) > 3:
                from .errors import ArgumentError
                raise ArgumentError(f"can_add_edge() takes 2-3 arguments (from_key, to_key, [relationship]), got {len(args)}", position)

            from_key = args[0]
            to_key = args[1]
            relationship = args[2] if len(args) == 3 else StringValue("related")

            if not isinstance(from_key, StringValue):
                raise RuntimeError("can_add_edge() from_key must be a string", position)
            if not isinstance(to_key, StringValue):
                raise RuntimeError("can_add_edge() to_key must be a string", position)
            if not isinstance(relationship, StringValue):
                raise RuntimeError("can_add_edge() relationship must be a string", position)

            # Check if edge can be added
            can_add, reason = target.can_add_edge(from_key.value, to_key.value, relationship.value)
            return BooleanValue(can_add, position)

        # Control layer access methods (Layer 3)
        elif method_name == "get_active_rules":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"get_active_rules() takes no arguments, got {len(args)}", position)
            active_rules = target.get_active_rules()
            rule_strings = [StringValue(rule) for rule in active_rules]
            return ListValue(rule_strings, "string", position)

        elif method_name == "get_rule_status":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"get_rule_status() takes exactly 1 argument (rule_name), got {len(args)}", position)
            rule_name = args[0]
            if not isinstance(rule_name, StringValue):
                raise RuntimeError("get_rule_status() rule_name must be a string", position)
            status = target.get_rule_status(rule_name.value)
            return StringValue(status, position)

        elif method_name == "disable_rule":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"disable_rule() takes exactly 1 argument (rule_name), got {len(args)}", position)
            rule_name = args[0]
            if not isinstance(rule_name, StringValue):
                raise RuntimeError("disable_rule() rule_name must be a string", position)
            return target.disable_rule(rule_name.value)

        elif method_name == "enable_rule":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"enable_rule() takes exactly 1 argument (rule_name), got {len(args)}", position)
            rule_name = args[0]
            if not isinstance(rule_name, StringValue):
                raise RuntimeError("enable_rule() rule_name must be a string", position)
            return target.enable_rule(rule_name.value)

        # Behavior management methods (from GraphContainer mixin)
        elif method_name == "add_rule":
            if len(args) < 1:
                from .errors import ArgumentError
                raise ArgumentError(f"add_rule() takes at least 1 argument (behavior name), got {len(args)}", position)
            return target.add_rule(*args)

        elif method_name == "remove_rule":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"remove_rule() takes 1 argument, got {len(args)}", position)
            return target.remove_rule(args[0])

        elif method_name == "has_rule":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"has_rule() takes 1 argument, got {len(args)}", position)
            return target.has_rule(args[0])

        elif method_name == "get_rules":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"get_rules() takes no arguments, got {len(args)}", position)
            return target.get_rules()

        elif method_name == "clear_rules":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"clear_rules() takes no arguments, got {len(args)}", position)
            return target.clear_rules()

        # Metadata layer methods
        elif method_name == "set_names":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"set_names() takes 1 argument, got {len(args)}", position)

            # Convert Glang list to Python list of optional strings
            names_list = args[0]
            if not isinstance(names_list, ListValue):
                from .errors import ArgumentError
                raise ArgumentError(f"set_names() expects a list, got {names_list.get_type()}", position)

            python_names = []
            for elem in names_list.elements:
                if isinstance(elem, StringValue):
                    python_names.append(elem.value)
                elif isinstance(elem, NoneValue):
                    python_names.append(None)
                else:
                    from .errors import ArgumentError
                    raise ArgumentError(f"Names must be strings or nil, got {elem.get_type()}", position)

            return target.set_names(python_names)

        elif method_name == "get_names":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"get_names() takes no arguments, got {len(args)}", position)

            # Convert Python list back to Glang list
            python_names = target.get_names()
            glang_names = []
            for name in python_names:
                if name is None:
                    glang_names.append(NoneValue(position))
                else:
                    glang_names.append(StringValue(name, position))

            return ListValue(glang_names, None, position)

        elif method_name == "has_names":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"has_names() takes no arguments, got {len(args)}", position)

            return BooleanValue(target.has_names(), position)

        elif method_name == "get_name":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"get_name() takes 1 argument, got {len(args)}", position)

            index_arg = args[0]
            if not isinstance(index_arg, NumberValue):
                from .errors import ArgumentError
                raise ArgumentError(f"get_name() expects integer index, got {index_arg.get_type()}", position)

            name = target.get_name(int(index_arg.value))
            if name is None:
                return NoneValue(position)
            else:
                    return StringValue(name, position)

        elif method_name == "set_name":
            if len(args) != 2:
                from .errors import ArgumentError
                raise ArgumentError(f"set_name() takes 2 arguments, got {len(args)}", position)

            index_arg = args[0]
            name_arg = args[1]

            if not isinstance(index_arg, NumberValue):
                from .errors import ArgumentError
                raise ArgumentError(f"set_name() expects integer index, got {index_arg.get_type()}", position)

            if isinstance(name_arg, StringValue):
                name = name_arg.value
            elif isinstance(name_arg, NoneValue):
                name = None
            else:
                from .errors import ArgumentError
                raise ArgumentError(f"set_name() expects string or nil, got {name_arg.get_type()}", position)

            return target.set_name(int(index_arg.value), name)

        elif method_name == "metadata":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"metadata property takes no arguments, got {len(args)}", position)

            # Return the metadata layer as a special metadata value that can be used for get/set operations
            # For now, return a placeholder since we need to implement a MetadataValue type
            return StringValue("metadata_access_placeholder", position)

        # NEW: Overloaded names() method for hash
        elif method_name == "names":
            # Overloaded method: 0 args = get, 1 arg = set
            if len(args) == 0:
                # Get names
                python_names = target.get_names()
                glang_names = []
                for name in python_names:
                    if name is None:
                        glang_names.append(NoneValue(position))
                    else:
                        glang_names.append(StringValue(name, position))
                return ListValue(glang_names, None, position)
            elif len(args) == 1:
                # Set names
                names_list = args[0]
                if not isinstance(names_list, ListValue):
                    from .errors import ArgumentError
                    raise ArgumentError(f"names() setter expects a list, got {names_list.get_type()}", position)

                python_names = []
                for elem in names_list.elements:
                    if isinstance(elem, StringValue):
                        python_names.append(elem.value)
                    elif isinstance(elem, NoneValue):
                        python_names.append(None)
                    else:
                        from .errors import ArgumentError
                        raise ArgumentError(f"Names must be strings or nil, got {elem.get_type()}", position)

                return target.set_names(python_names)
            else:
                from .errors import ArgumentError
                raise ArgumentError(f"names() takes 0 or 1 arguments, got {len(args)}", position)

        # NEW: Visualization methods for hashes
        elif method_name == "visualize":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"visualize() takes no arguments, got {len(args)}", position)
            # Quick shape overview of the hash
            if len(target.pairs) == 0:
                return StringValue("{}", position)
            elif len(target.pairs) <= 10:
                # Show all keys for small hashes
                keys = sorted(target.keys())
                return StringValue(f"{{ {', '.join(keys)} }}", position)
            else:
                # Abbreviated view for large hashes
                keys = sorted(target.keys())
                shown_keys = keys[:3] + ['...'] + keys[-2:]
                return StringValue(f"{{ {', '.join(shown_keys)} }} ({len(target.pairs)} entries)", position)

        elif method_name == "view":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"view() takes no arguments, got {len(args)}", position)
            # Clean semantic display of keys and values
            if len(target.pairs) == 0:
                return StringValue("{}", position)
            else:
                items = []
                for key in sorted(target.keys()):
                    value = target.get(key)
                    value_str = value.to_display_string()
                    items.append(f'"{key}": {value_str}')
                # Limit items shown for very large hashes
                if len(items) > 20:
                    shown_items = items[:10] + [f'... {len(items) - 15} more entries ...'] + items[-5:]
                    return StringValue(f"{{ {', '.join(shown_items)} }}", position)
                else:
                    return StringValue(f"{{ {', '.join(items)} }}", position)

        # NEW: Consistent naming for edge/node counting
        elif method_name == "count_edges":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"count_edges() takes no arguments, got {len(args)}", position)
            # Use the hash's structural edge counting method
            return NumberValue(target.count_edges(), position)

        elif method_name == "count_nodes":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"count_nodes() takes no arguments, got {len(args)}", position)
            # Use the hash's node counting method
            return NumberValue(target.count_nodes(), position)

        else:
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, "hash", position)

    def _dispatch_tree_method(self, target, method_name: str,
                             args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Handle binary tree method calls."""

        # Import BinaryTreeValue here to avoid circular import issues
        from .tree_structures import BinaryTreeValue

        if not isinstance(target, BinaryTreeValue):
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, target.get_type(), position)

        # Tree operations
        if method_name == "insert":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"insert() takes exactly 1 argument (value), got {len(args)}", position)
            return target.insert(args[0])

        elif method_name == "search":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"search() takes exactly 1 argument (value), got {len(args)}", position)
            return target.search(args[0])

        elif method_name == "size":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"size() takes no arguments, got {len(args)}", position)
            return target.size()

        elif method_name == "empty":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"empty() takes no arguments, got {len(args)}", position)
            return target.empty()

        elif method_name == "height":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"height() takes no arguments, got {len(args)}", position)
            return target.height()

        # Traversal methods
        elif method_name == "in_order":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"in_order() takes no arguments, got {len(args)}", position)
            return target.in_order()

        elif method_name == "pre_order":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"pre_order() takes no arguments, got {len(args)}", position)
            return target.pre_order()

        elif method_name == "post_order":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"post_order() takes no arguments, got {len(args)}", position)
            return target.post_order()

        # Edge governance methods
        elif method_name == "get_active_rules":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"get_active_rules() takes no arguments, got {len(args)}", position)
            active_rules = target.get_active_rules()
            rule_strings = [StringValue(rule) for rule in active_rules]
            return ListValue(rule_strings, "string", position)

        elif method_name == "get_rule_status":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"get_rule_status() takes exactly 1 argument (rule_name), got {len(args)}", position)
            rule_name = args[0]
            if not isinstance(rule_name, StringValue):
                raise RuntimeError("get_rule_status() rule_name must be a string", position)
            status = target.get_rule_status(rule_name.value)
            return StringValue(status, position)

        elif method_name == "disable_rule":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"disable_rule() takes exactly 1 argument (rule_name), got {len(args)}", position)
            rule_name = args[0]
            if not isinstance(rule_name, StringValue):
                raise RuntimeError("disable_rule() rule_name must be a string", position)
            return target.disable_rule(rule_name.value)

        elif method_name == "enable_rule":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"enable_rule() takes exactly 1 argument (rule_name), got {len(args)}", position)
            rule_name = args[0]
            if not isinstance(rule_name, StringValue):
                raise RuntimeError("enable_rule() rule_name must be a string", position)
            return target.enable_rule(rule_name.value)

        # Visualization methods
        elif method_name == "get_graph_summary":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"get_graph_summary() takes no arguments, got {len(args)}", position)

            summary = target.get_graph_summary()
            # Convert to a hash structure that Glang can use
            hash_pairs = []
            for key, value in summary.items():
                if isinstance(value, list):
                    # Convert lists to ListValue
                    list_items = [StringValue(item) for item in value]
                    hash_pairs.append((key, ListValue(list_items, "string", position)))
                elif isinstance(value, (int, float)):
                    hash_pairs.append((key, NumberValue(value, position)))
                elif isinstance(value, str):
                    hash_pairs.append((key, StringValue(value, position)))
                else:
                    hash_pairs.append((key, StringValue(str(value), position)))

            return HashValue(hash_pairs, None, position)

        elif method_name == "visualize_structure":
            if len(args) > 1:
                from .errors import ArgumentError
                raise ArgumentError(f"visualize_structure() takes 0-1 arguments, got {len(args)}", position)

            format_arg = args[0] if len(args) == 1 else StringValue("text")
            if not isinstance(format_arg, StringValue):
                raise RuntimeError("visualize_structure() format must be a string", position)

            result = target.visualize_structure(format_arg.value)
            return StringValue(result, position)

        # Universal methods
        elif method_name == "to_string":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"to_string() takes no arguments, got {len(args)}", position)
            return StringValue(target.to_display_string(), position)

        elif method_name == "to_bool":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"to_bool() takes no arguments, got {len(args)}", position)
            # Tree is truthy if it has nodes
            return BooleanValue(not target.empty().value, position)

        else:
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, "tree", position)

    def _dispatch_time_method(self, target, method_name: str,
                             args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Handle time method calls."""
        
        # Import TimeValue here to avoid circular import issues
        from .values import TimeValue
        
        if method_name == "get_type":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"get_type() takes no arguments, got {len(args)}", position)
            return StringValue(target.get_type(), position)
        
        elif method_name == "to_string":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"to_string() takes no arguments, got {len(args)}", position)
            return StringValue(target.to_string(), position)
        
        elif method_name == "to_num":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"to_num() takes no arguments, got {len(args)}", position)
            # Return the timestamp as a number
            from .values import NumberValue
            return NumberValue(target.to_python(), position)
        
        # Check if it's a universal method
        elif method_name in ['freeze', 'is_frozen', 'contains_frozen']:
            return self._dispatch_universal_method(target, method_name, args, position)
        
        else:
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, "time", position)
    
    def _dispatch_file_method(self, target, method_name: str,
                             args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Handle boundary operations on file capabilities.
        
        File handles are boundary capabilities that provide controlled, unidirectional
        access to external resources. Operations are strictly constrained by capability type.
        """
        
        # Import FileHandleValue here to avoid circular import issues
        from .values import FileHandleValue
        
        if not isinstance(target, FileHandleValue):
            raise RuntimeError(f"Expected file capability, got {target.get_type()}", position)
        
        # Boundary operation: write (only for write/append capabilities)
        if method_name == "write":
            if not target.is_write_capability():
                raise RuntimeError(
                    f"Cannot write to {target.get_capability_type()} capability. "
                    f"Writing requires write or append capability.", 
                    position
                )
            
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"write() takes 1 argument, got {len(args)}", position)
            
            # Convert argument to string (using Glang's string representation)
            content = args[0]
            if isinstance(content, StringValue):
                content_str = content.value
            else:
                content_str = content.to_display_string()
            
            try:
                target._ensure_active()
                target._python_handle.write(content_str)
                # Update logical position
                target._position += len(content_str)
                return BooleanValue(True, position)
            except Exception as e:
                raise RuntimeError(f"Error in write boundary operation: {str(e)}", position)
        
        # Boundary operation: read (only for read capabilities)
        elif method_name == "read":
            if not target.is_read_capability():
                raise RuntimeError(
                    f"Cannot read from {target.get_capability_type()} capability. "
                    f"Reading requires read capability.", 
                    position
                )
            
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"read() takes no arguments, got {len(args)}", position)
            
            try:
                target._ensure_active()
                content = target._python_handle.read()
                # Update logical position
                target._position += len(content)
                
                # Auto-close on EOF: read() always reads to end of file
                target._ensure_inactive()
                
                return StringValue(content, position)
            except Exception as e:
                raise RuntimeError(f"Error in read boundary operation: {str(e)}", position)
        
        # Boundary operation: read_line (only for read capabilities)
        elif method_name == "read_line":
            if not target.is_read_capability():
                raise RuntimeError(
                    f"Cannot read from {target.get_capability_type()} capability. "
                    f"Line reading requires read capability.", 
                    position
                )
            
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"read_line() takes no arguments, got {len(args)}", position)
            
            try:
                target._ensure_active()
                line = target._python_handle.readline()
                
                # Check if we hit EOF (empty string means EOF)
                if line == "":
                    # Auto-close on EOF
                    target._ensure_inactive()
                    return StringValue("", position)
                
                # Remove trailing newline if present
                if line.endswith('\n'):
                    line = line[:-1]
                    target._position += len(line) + 1  # Include newline in position
                else:
                    target._position += len(line)
                    # If no newline, we're at EOF, so auto-close
                    target._ensure_inactive()
                
                return StringValue(line, position)
            except Exception as e:
                raise RuntimeError(f"Error in read_line boundary operation: {str(e)}", position)
        
        # Boundary operation: flush (only for write capabilities)
        elif method_name == "flush":
            if not target.is_write_capability():
                raise RuntimeError(
                    f"Cannot flush {target.get_capability_type()} capability. "
                    f"Flushing requires write or append capability.", 
                    position
                )
            
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"flush() takes no arguments, got {len(args)}", position)
            
            try:
                target._ensure_active()
                target._python_handle.flush()
                return BooleanValue(True, position)
            except Exception as e:
                raise RuntimeError(f"Error in flush boundary operation: {str(e)}", position)
        
        # Boundary operation: close (for all capabilities)
        elif method_name == "close":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"close() takes no arguments, got {len(args)}", position)
            
            try:
                target._ensure_inactive()
                return BooleanValue(True, position)
            except Exception as e:
                raise RuntimeError(f"Error in close boundary operation: {str(e)}", position)
        
        # Capability lifecycle: kill (permanent destruction)
        elif method_name == "kill":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"kill() takes no arguments, got {len(args)}", position)
            
            try:
                target._kill_capability()
                return BooleanValue(True, position)
            except Exception as e:
                raise RuntimeError(f"Error in kill capability operation: {str(e)}", position)
        
        # Capability introspection methods
        elif method_name == "capability_type":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"capability_type() takes no arguments, got {len(args)}", position)
            return StringValue(target.get_capability_type(), position)
        
        # Check if it's a universal method
        elif method_name in ['type', 'size', 'inspect']:
            return self._dispatch_universal_method(target, method_name, args, position)
        
        else:
            from .errors import MethodNotFoundError
            available_methods = ['write', 'flush', 'close', 'kill'] if target.is_write_capability() else ['read', 'read_line', 'close', 'kill']
            available_methods.extend(['capability_type', 'type', 'size', 'inspect'])
            raise MethodNotFoundError(
                method_name, 
                f"{target.get_capability_type()}-capability", 
                position,
                f"Available methods for {target.get_capability_type()} capability: {', '.join(available_methods)}"
            )
    
    # Additional visitor methods that need to be implemented
    def visit_expression_statement(self, node) -> None:
        """Visit an expression statement."""
        self.result = self.execute(node.expression)
    
    def visit_index_assignment(self, node) -> None:
        """Visit an index assignment - delegate to assignment logic."""
        # Import graph values for isinstance checks
        # Using already imported ListValue, HashValue

        # Use the same logic as assignment but for index targets
        value = self.execute(node.value)

        if not isinstance(value, GlangValue):
            value = python_to_glang_value(value, node.position)

        target_value = self.execute(node.target.target)

        if len(node.target.indices) != 1:
            raise RuntimeError(
                f"Multi-dimensional indexing not yet supported",
                node.target.position
            )

        index_value = self.execute(node.target.indices[0])

        # Handle list index assignment (both old and new implementations)
        if isinstance(target_value, (ListValue, ListValue)):
            # Support both integer and string indices for enhanced indexing
            if isinstance(index_value, NumberValue) and isinstance(index_value.value, int):
                # Integer indexing
                index_key = index_value.value
            elif isinstance(index_value, StringValue):
                # String-based name indexing
                index_key = index_value.value
            else:
                raise RuntimeError(
                    f"List index must be integer or string, got {index_value.get_type()}",
                    node.target.indices[0].position
                )

            # Use graph-based ListValue implementation with enhanced indexing
            try:
                target_value[index_key] = value
            except (IndexError, TypeError) as e:
                raise RuntimeError(f"Index {index_key} out of range", node.target.indices[0].position)
            # Get target name safely - could be VariableRef or nested IndexAccess
            target_name = getattr(node.target.target, 'name', 'target')
            self.result = f"Set {target_name}[{index_key}] = {value.to_display_string()}"

        # Handle hash index assignment - creates/updates data node (both old and new implementations)
        elif isinstance(target_value, (HashValue, HashValue)):
            if not isinstance(index_value, StringValue):
                raise RuntimeError(
                    f"Hash index must be string, got {index_value.get_type()}",
                    node.target.indices[0].position
                )

            key = index_value.value
            # Use HashValue's enhanced __setitem__ which handles both keys and names
            target_value[key] = value
            # Get target name safely - could be VariableRef or nested IndexAccess
            target_name = getattr(node.target.target, 'name', 'target')
            self.result = f"Set {target_name}['{key}'] = {value.to_display_string()}"
        
        else:
            raise RuntimeError(
                f"Cannot index {target_value.get_type()}", 
                node.target.position
            )
    
    def visit_slice_assignment(self, node) -> None:
        """Visit a slice assignment (not yet fully implemented)."""
        raise RuntimeError("Slice assignment not yet implemented", node.position)
    
    def visit_slice_access(self, node) -> None:
        """Visit slice access for strings and lists."""
        target_value = self.execute(node.target)

        # Evaluate slice parameters
        start = None if node.start is None else self.execute(node.start)
        stop = None if node.stop is None else self.execute(node.stop)
        step = None if node.step is None else self.execute(node.step)

        # Helper function to convert name to index for lists with names
        def name_to_index(param, param_name, target_value):
            """Convert a string name to an index for slicing."""
            if isinstance(param, StringValue) and isinstance(target_value, ListValue):
                # Try to find the index of this name
                names = target_value.get_names()
                name_str = param.value
                try:
                    index = names.index(name_str)
                    # Keep the same exclusive stop behavior as numeric slicing
                    return NumberValue(index, param.position)
                except (ValueError, AttributeError):
                    raise RuntimeError(
                        f"Name '{name_str}' not found in list names",
                        node.position
                    )
            return param

        # Convert string names to indices for lists
        if isinstance(target_value, ListValue):
            start = name_to_index(start, "start", target_value)
            stop = name_to_index(stop, "stop", target_value)
            # Note: step should always be numeric, not a name

        # Validate slice parameters are integers or None
        for param, name in [(start, "start"), (stop, "stop"), (step, "step")]:
            if param is not None:
                if not isinstance(param, NumberValue) or not isinstance(param.value, int):
                    raise RuntimeError(
                        f"Slice {name} must be integer, got {param.get_type()}",
                        node.position
                    )

        # Extract integer values or None
        start_val = None if start is None else start.value
        stop_val = None if stop is None else stop.value
        step_val = None if step is None else step.value
        
        # Handle string slicing
        if isinstance(target_value, StringValue):
            string_val = target_value.value
            sliced = string_val[start_val:stop_val:step_val]
            self.result = StringValue(sliced, node.position)
        
        # Handle list slicing (both old and new implementations)
        elif isinstance(target_value, (ListValue, ListValue)):
            elements = target_value.elements
            sliced_elements = elements[start_val:stop_val:step_val]
            self.result = ListValue(sliced_elements, target_value.constraint, node.position)
        
        else:
            raise RuntimeError(
                f"Cannot slice {target_value.get_type()}", 
                node.position
            )
    
    def visit_method_call_expression(self, node) -> None:
        """Visit method call in expression context."""
        # Same as method call statement, but in expression context
        return self.visit_method_call(node)
    
    def visit_binary_operation(self, node: BinaryOperation) -> None:
        """Execute binary operation (arithmetic, comparison)."""
        # Special handling for logical operators to enable short-circuit evaluation
        if node.operator in ("and", "or"):
            left_value = self.execute(node.left)
            if not isinstance(left_value, GlangValue):
                left_value = python_to_glang_value(left_value)

            # Short-circuit evaluation for logical operators
            if node.operator == "and":
                # If left is false, return false without evaluating right
                if not self._to_boolean(left_value):
                    self.result = BooleanValue(False)
                    return
                # Otherwise evaluate right and return its truthiness
                right_value = self.execute(node.right)
                if not isinstance(right_value, GlangValue):
                    right_value = python_to_glang_value(right_value)
                self.result = BooleanValue(self._to_boolean(right_value))
                return
            elif node.operator == "or":
                # If left is true, return true without evaluating right
                if self._to_boolean(left_value):
                    self.result = BooleanValue(True)
                    return
                # Otherwise evaluate right and return its truthiness
                right_value = self.execute(node.right)
                if not isinstance(right_value, GlangValue):
                    right_value = python_to_glang_value(right_value)
                self.result = BooleanValue(self._to_boolean(right_value))
                return

        # For all other operators, evaluate both operands
        left_value = self.execute(node.left)
        right_value = self.execute(node.right)

        # Convert to GlangValues if needed
        if not isinstance(left_value, GlangValue):
            left_value = python_to_glang_value(left_value)
        if not isinstance(right_value, GlangValue):
            right_value = python_to_glang_value(right_value)

        # Perform the operation based on operator
        if node.operator == "+":
            self.result = self.perform_addition(left_value, right_value)
        elif node.operator == "-":
            self.result = self.perform_subtraction(left_value, right_value)
        elif node.operator == "*":
            self.result = self.perform_multiplication(left_value, right_value)
        elif node.operator == "/":
            self.result = self.perform_division(left_value, right_value)
        elif node.operator == "%":
            self.result = self.perform_modulo(left_value, right_value)
        elif node.operator == ">":
            self.result = self.perform_comparison(left_value, right_value, "greater")
        elif node.operator == "<":
            self.result = self.perform_comparison(left_value, right_value, "less")
        elif node.operator == ">=":
            self.result = self.perform_comparison(left_value, right_value, "greater_equal")
        elif node.operator == "<=":
            self.result = self.perform_comparison(left_value, right_value, "less_equal")
        elif node.operator == "==":
            self.result = self.perform_comparison(left_value, right_value, "equal")
        elif node.operator == "!=":
            self.result = self.perform_comparison(left_value, right_value, "not_equal")
        elif node.operator == "!>":  # Intuitive "not greater than" = less than or equal
            self.result = self.perform_comparison(left_value, right_value, "less_equal")
        elif node.operator == "!<":  # Intuitive "not less than" = greater than or equal
            self.result = self.perform_comparison(left_value, right_value, "greater_equal")
        elif node.operator == "&":
            self.result = self.perform_intersection(left_value, right_value)
        elif node.operator == "+.":
            self.result = self.perform_elementwise_addition(left_value, right_value)
        elif node.operator == "-.":
            self.result = self.perform_elementwise_subtraction(left_value, right_value)
        elif node.operator == "*.":
            self.result = self.perform_elementwise_multiplication(left_value, right_value)
        elif node.operator == "/.":
            self.result = self.perform_elementwise_division(left_value, right_value)
        elif node.operator == "%.":
            self.result = self.perform_elementwise_modulo(left_value, right_value)
        else:
            raise RuntimeError(f"Unknown binary operator: {node.operator}", node.position)
    
    def visit_unary_operation(self, node: UnaryOperation) -> None:
        """Execute unary operation."""
        operand_value = self.execute(node.operand)
        
        if not isinstance(operand_value, GlangValue):
            operand_value = python_to_glang_value(operand_value)
        
        if node.operator == "-":
            if isinstance(operand_value, NumberValue):
                self.result = NumberValue(-operand_value.value)
            else:
                raise RuntimeError(f"Cannot negate non-numeric value: {operand_value.get_type()}", node.position)
        elif node.operator == "!" or node.operator == "not":
            # Support both ! and not for logical negation
            if isinstance(operand_value, BooleanValue):
                self.result = BooleanValue(not operand_value.value)
            else:
                # Try to convert to boolean using truthiness rules
                truthiness = self._to_boolean(operand_value)
                self.result = BooleanValue(not truthiness)
        else:
            raise RuntimeError(f"Unknown unary operator: {node.operator}", node.position)
    
    def visit_load_statement(self, node: LoadStatement) -> None:
        """Visit load statement - include file in current namespace."""
        if not self.file_manager:
            raise RuntimeError("File manager not available for load operation", node.position)
        
        # For load statements within the AST executor, we need to delegate back to the execution session
        # This is handled by raising a special exception that the execution session can catch
        from .errors import LoadRequest
        raise LoadRequest(node.filename, node.position)
    
    def visit_print_statement(self, node: 'PrintStatement') -> None:
        """Visit print statement - print values to output."""
        from ..ast.nodes import PrintStatement
        
        # Evaluate all arguments and print them
        output_parts = []
        for arg in node.arguments:
            # Execute the argument expression to get its value
            value = self.execute(arg)
            
            # Convert to display string
            if value is not None:
                output_parts.append(value.to_display_string())
            else:
                output_parts.append("None")
        
        # Print all parts separated by spaces, or just empty line if no arguments
        if output_parts:
            print(" ".join(output_parts))
        else:
            print()
        
        # Print statements don't return values
        self.result = None
    
    def visit_print_expression(self, node: 'PrintExpression') -> None:
        """Visit print expression - print values and return None."""
        from ..ast.nodes import PrintExpression
        
        # Evaluate all arguments and print them
        output_parts = []
        for arg in node.arguments:
            # Execute the argument expression to get its value
            value = self.execute(arg)
            
            # Convert to display string
            if value is not None:
                output_parts.append(value.to_display_string())
            else:
                output_parts.append("None")
        
        # Print all parts separated by spaces, or just empty line if no arguments
        if output_parts:
            print(" ".join(output_parts))
        else:
            print()
        
        # Print expressions return None 
        self.result = None
    
    def visit_import_statement(self, node: ImportStatement) -> None:
        """Visit import statement - load module into namespace."""
        if not self.context.module_manager:
            raise RuntimeError("Module manager not available for import operation", node.position)
        
        # Similar to load statements, we need to delegate back to the execution session
        # This is handled by raising a special exception
        from .errors import ImportRequest
        raise ImportRequest(node.filename, node.alias, node.position)
    
    def visit_module_declaration(self, node: ModuleDeclaration) -> None:
        """Visit module declaration - store module name for current file."""
        # Store the declared module name
        if not hasattr(self.context, '_module_name'):
            self.context._module_name = node.name

        # Set current module in call graph for proper scoping
        self.context.current_module = node.name
        self.context.call_graph.enter_scope(node.name)

        return None
    
    def visit_alias_declaration(self, node: AliasDeclaration) -> None:
        """Visit alias declaration - store alias for current module."""
        # Store the declared alias
        if not hasattr(self.context, '_module_alias'):
            self.context._module_alias = node.name
        return None
    
    def visit_noop(self, node) -> None:
        """Visit no-op statement - do nothing."""
        return None
    
    # =============================================================================
    # Arithmetic Operations
    # =============================================================================
    
    def _is_numeric_list(self, list_value: ListValue) -> bool:
        """Check if a list contains all numeric elements."""
        if list_value.constraint == "num":
            return True
        if list_value.constraint is not None and list_value.constraint != "num":
            return False
        # For unconstrained lists, check if all elements are numeric
        return all(isinstance(elem, NumberValue) for elem in list_value.elements)
    
    def perform_addition(self, left: GlangValue, right: GlangValue) -> GlangValue:
        """Perform addition operation."""
        # Import graph values for isinstance checks
        from .graph_values import ListValue

        if isinstance(left, NumberValue) and isinstance(right, NumberValue):
            result = left.add(right)
            # Apply decimal_places configuration if set
            decimal_places = self.context.config.get_decimal_places()
            if decimal_places is not None and isinstance(result, NumberValue):
                rounded_value = round(result.value, decimal_places)
                return NumberValue(rounded_value, result.position)
            return result
        elif isinstance(left, StringValue) and isinstance(right, StringValue):
            return left.concatenate(right)
        elif isinstance(left, (ListValue, ListValue)) and isinstance(right, (ListValue, ListValue)):
            # List concatenation/union - combine all elements
            combined_elements = left.elements + right.elements
            # Use the constraint from the left list (or None if both have different constraints)
            constraint = left.constraint if left.constraint == right.constraint else None
            return ListValue(combined_elements, constraint, left.position)
        # Note: List-scalar arithmetic moved to +. operator
        else:
            raise RuntimeError(f"Cannot add {left.get_type()} and {right.get_type()}")
    
    def perform_subtraction(self, left: GlangValue, right: GlangValue) -> GlangValue:
        """Perform subtraction operation."""
        # Import graph values for isinstance checks
        from .graph_values import ListValue

        if isinstance(left, NumberValue) and isinstance(right, NumberValue):
            result = left.subtract(right)
            # Apply decimal_places configuration if set
            decimal_places = self.context.config.get_decimal_places()
            if decimal_places is not None and isinstance(result, NumberValue):
                rounded_value = round(result.value, decimal_places)
                return NumberValue(rounded_value, result.position)
            return result
        elif isinstance(left, (ListValue, ListValue)) and isinstance(right, (ListValue, ListValue)):
            # List set difference - remove elements from left that are in right
            result_elements = []
            for element in left.elements:
                if not right.contains(element):
                    result_elements.append(element)
            return ListValue(result_elements, left.constraint, left.position)
        # Note: List-scalar arithmetic moved to -. operator
        else:
            raise RuntimeError(f"Cannot subtract {right.get_type()} from {left.get_type()}")
    
    def perform_multiplication(self, left: GlangValue, right: GlangValue) -> GlangValue:
        """Perform multiplication operation (numbers only - use *. for element-wise)."""
        if isinstance(left, NumberValue) and isinstance(right, NumberValue):
            result = left.multiply(right)
            # Apply decimal_places configuration if set
            decimal_places = self.context.config.get_decimal_places()
            if decimal_places is not None and isinstance(result, NumberValue):
                rounded_value = round(result.value, decimal_places)
                return NumberValue(rounded_value, result.position)
            return result
        else:
            raise RuntimeError(f"Cannot multiply {left.get_type()} and {right.get_type()} - use *. for element-wise operations")
    
    def perform_division(self, left: GlangValue, right: GlangValue) -> GlangValue:
        """Perform division operation (numbers only - use /. for element-wise)."""
        if isinstance(left, NumberValue) and isinstance(right, NumberValue):
            try:
                result = left.divide(right)
                # Apply decimal_places configuration if set
                decimal_places = self.context.config.get_decimal_places()
                if decimal_places is not None and isinstance(result, NumberValue):
                    rounded_value = round(result.value, decimal_places)
                    return NumberValue(rounded_value, result.position)
                return result
            except ValueError as e:
                raise RuntimeError(str(e))
        else:
            raise RuntimeError(f"Cannot divide {left.get_type()} by {right.get_type()} - use /. for element-wise operations")
    
    def perform_modulo(self, left: GlangValue, right: GlangValue) -> GlangValue:
        """Perform modulo operation (numbers only - use %. for element-wise)."""
        if isinstance(left, NumberValue) and isinstance(right, NumberValue):
            try:
                return left.modulo(right)
            except ValueError as e:
                raise RuntimeError(str(e))
        else:
            raise RuntimeError(f"Cannot perform modulo on {left.get_type()} and {right.get_type()} - use %. for element-wise operations")
    
    def perform_intersection(self, left: GlangValue, right: GlangValue) -> GlangValue:
        """Perform intersection operation."""
        from .graph_values import ListValue
        if isinstance(left, (ListValue, ListValue)) and isinstance(right, (ListValue, ListValue)):
            # List intersection - keep elements that appear in both lists
            result_elements = []
            # Create a temporary list to check for duplicates in result
            result_list = ListValue([], left.constraint, left.position)
            for element in left.elements:
                if right.contains(element) and not result_list.contains(element):
                    result_elements.append(element)
                    result_list.append(element)  # Use append method for both types
            return ListValue(result_elements, left.constraint, left.position)
        else:
            raise RuntimeError(f"Cannot perform intersection on {left.get_type()} and {right.get_type()}")

    def perform_logical_and(self, left: GlangValue, right: GlangValue) -> GlangValue:
        """Perform logical AND operation."""
        # Convert values to boolean using truthiness
        left_bool = self._to_boolean(left)

        # Short-circuit: if left is false, return false without evaluating right
        if not left_bool:
            return BooleanValue(False, left.position)

        # Left is true, so result depends on right
        right_bool = self._to_boolean(right)
        return BooleanValue(right_bool, right.position)

    def perform_logical_or(self, left: GlangValue, right: GlangValue) -> GlangValue:
        """Perform logical OR operation."""
        # Convert values to boolean using truthiness
        left_bool = self._to_boolean(left)

        # Short-circuit: if left is true, return true without evaluating right
        if left_bool:
            return BooleanValue(True, left.position)

        # Left is false, so result depends on right
        right_bool = self._to_boolean(right)
        return BooleanValue(right_bool, right.position)

    def _to_boolean(self, value: GlangValue) -> bool:
        """Convert a GlangValue to boolean using truthiness rules."""
        if isinstance(value, BooleanValue):
            return value.value
        elif isinstance(value, NumberValue):
            return value.value != 0
        elif isinstance(value, StringValue):
            return len(value.value) > 0
        elif isinstance(value, ListValue):
            return len(value.elements) > 0
        elif isinstance(value, HashValue):
            return len(value.pairs) > 0
        elif isinstance(value, DataNodeValue):
            return value.value is not None and self._to_boolean(value.value)
        else:
            # Default: non-null values are truthy
            return True

    # =============================================================================
    # Element-wise Arithmetic Operations (Dot Operators)
    # =============================================================================
    
    def perform_elementwise_addition(self, left: GlangValue, right: GlangValue) -> GlangValue:
        """Perform element-wise addition using +. operator."""
        # Import graph values for isinstance checks
        from .graph_values import ListValue

        if isinstance(left, (ListValue, ListValue)) and isinstance(right, (ListValue, ListValue)):
            if not self._is_numeric_list(left) or not self._is_numeric_list(right):
                raise RuntimeError("Element-wise addition requires numeric lists")
            if len(left.elements) != len(right.elements):
                raise RuntimeError(f"Element-wise addition requires lists of same length ({len(left.elements)} != {len(right.elements)})")
            
            result_elements = []
            for i in range(len(left.elements)):
                left_elem = left.elements[i]
                right_elem = right.elements[i]
                if isinstance(left_elem, NumberValue) and isinstance(right_elem, NumberValue):
                    result_elements.append(left_elem.add(right_elem))
                else:
                    raise RuntimeError("Element-wise addition requires all elements to be numbers")
            return ListValue(result_elements, "num", left.position)
        elif isinstance(left, (ListValue, ListValue)) and isinstance(right, NumberValue):
            # List-scalar element-wise addition
            if not self._is_numeric_list(left):
                raise RuntimeError(f"Cannot perform element-wise addition on list of {left.constraint or 'mixed types'}")
            
            result_elements = []
            for element in left.elements:
                if isinstance(element, NumberValue):
                    result_elements.append(element.add(right))
                else:
                    raise RuntimeError("Cannot add number to non-numeric list element")
            return ListValue(result_elements, "num", left.position)
        elif isinstance(left, NumberValue) and isinstance(right, (ListValue, ListValue)):
            # Scalar-list element-wise addition
            if not self._is_numeric_list(right):
                raise RuntimeError(f"Cannot perform element-wise addition on list of {right.constraint or 'mixed types'}")
            
            result_elements = []
            for element in right.elements:
                if isinstance(element, NumberValue):
                    result_elements.append(left.add(element))
                else:
                    raise RuntimeError("Cannot add number to non-numeric list element")
            return ListValue(result_elements, "num", right.position)
        else:
            raise RuntimeError(f"Element-wise addition not supported for {left.get_type()} and {right.get_type()}")
    
    def perform_elementwise_subtraction(self, left: GlangValue, right: GlangValue) -> GlangValue:
        """Perform element-wise subtraction using -. operator."""
        # Import graph values for isinstance checks
        from .graph_values import ListValue

        if isinstance(left, (ListValue, ListValue)) and isinstance(right, (ListValue, ListValue)):
            if not self._is_numeric_list(left) or not self._is_numeric_list(right):
                raise RuntimeError("Element-wise subtraction requires numeric lists")
            if len(left.elements) != len(right.elements):
                raise RuntimeError(f"Element-wise subtraction requires lists of same length ({len(left.elements)} != {len(right.elements)})")
            
            result_elements = []
            for i in range(len(left.elements)):
                left_elem = left.elements[i]
                right_elem = right.elements[i]
                if isinstance(left_elem, NumberValue) and isinstance(right_elem, NumberValue):
                    result_elements.append(left_elem.subtract(right_elem))
                else:
                    raise RuntimeError("Element-wise subtraction requires all elements to be numbers")
            return ListValue(result_elements, "num", left.position)
        elif isinstance(left, (ListValue, ListValue)) and isinstance(right, NumberValue):
            # List-scalar element-wise subtraction
            if not self._is_numeric_list(left):
                raise RuntimeError(f"Cannot perform element-wise subtraction on list of {left.constraint or 'mixed types'}")
            
            result_elements = []
            for element in left.elements:
                if isinstance(element, NumberValue):
                    result_elements.append(element.subtract(right))
                else:
                    raise RuntimeError("Cannot subtract number from non-numeric list element")
            return ListValue(result_elements, "num", left.position)
        elif isinstance(left, NumberValue) and isinstance(right, (ListValue, ListValue)):
            # Scalar-list element-wise subtraction
            if not self._is_numeric_list(right):
                raise RuntimeError(f"Cannot perform element-wise subtraction on list of {right.constraint or 'mixed types'}")
            
            result_elements = []
            for element in right.elements:
                if isinstance(element, NumberValue):
                    result_elements.append(left.subtract(element))
                else:
                    raise RuntimeError("Cannot subtract non-numeric list element from number")
            return ListValue(result_elements, "num", right.position)
        else:
            raise RuntimeError(f"Element-wise subtraction not supported for {left.get_type()} and {right.get_type()}")
    
    def perform_elementwise_multiplication(self, left: GlangValue, right: GlangValue) -> GlangValue:
        """Perform element-wise multiplication using *. operator."""
        # Import graph values for isinstance checks
        from .graph_values import ListValue

        if isinstance(left, (ListValue, ListValue)) and isinstance(right, (ListValue, ListValue)):
            if not self._is_numeric_list(left) or not self._is_numeric_list(right):
                raise RuntimeError("Element-wise multiplication requires numeric lists")
            if len(left.elements) != len(right.elements):
                raise RuntimeError(f"Element-wise multiplication requires lists of same length ({len(left.elements)} != {len(right.elements)})")
            
            result_elements = []
            for i in range(len(left.elements)):
                left_elem = left.elements[i]
                right_elem = right.elements[i]
                if isinstance(left_elem, NumberValue) and isinstance(right_elem, NumberValue):
                    result_elements.append(left_elem.multiply(right_elem))
                else:
                    raise RuntimeError("Element-wise multiplication requires all elements to be numbers")
            return ListValue(result_elements, "num", left.position)
        elif isinstance(left, (ListValue, ListValue)) and isinstance(right, NumberValue):
            # List-scalar element-wise multiplication
            if not self._is_numeric_list(left):
                raise RuntimeError(f"Cannot perform element-wise multiplication on list of {left.constraint or 'mixed types'}")
            
            result_elements = []
            for element in left.elements:
                if isinstance(element, NumberValue):
                    result_elements.append(element.multiply(right))
                else:
                    raise RuntimeError("Cannot multiply non-numeric list element with number")
            return ListValue(result_elements, "num", left.position)
        elif isinstance(left, NumberValue) and isinstance(right, (ListValue, ListValue)):
            # Scalar-list element-wise multiplication
            if not self._is_numeric_list(right):
                raise RuntimeError(f"Cannot perform element-wise multiplication on list of {right.constraint or 'mixed types'}")
            
            result_elements = []
            for element in right.elements:
                if isinstance(element, NumberValue):
                    result_elements.append(left.multiply(element))
                else:
                    raise RuntimeError("Cannot multiply number with non-numeric list element")
            return ListValue(result_elements, "num", right.position)
        else:
            raise RuntimeError(f"Element-wise multiplication not supported for {left.get_type()} and {right.get_type()}")
    
    def perform_elementwise_division(self, left: GlangValue, right: GlangValue) -> GlangValue:
        """Perform element-wise division using /. operator."""
        # Import graph values for isinstance checks
        from .graph_values import ListValue

        if isinstance(left, (ListValue, ListValue)) and isinstance(right, (ListValue, ListValue)):
            if not self._is_numeric_list(left) or not self._is_numeric_list(right):
                raise RuntimeError("Element-wise division requires numeric lists")
            if len(left.elements) != len(right.elements):
                raise RuntimeError(f"Element-wise division requires lists of same length ({len(left.elements)} != {len(right.elements)})")
            
            result_elements = []
            for i in range(len(left.elements)):
                left_elem = left.elements[i]
                right_elem = right.elements[i]
                if isinstance(left_elem, NumberValue) and isinstance(right_elem, NumberValue):
                    if right_elem.value == 0:
                        raise RuntimeError("Division by zero in element-wise division")
                    result_elements.append(left_elem.divide(right_elem))
                else:
                    raise RuntimeError("Element-wise division requires all elements to be numbers")
            return ListValue(result_elements, "num", left.position)
        elif isinstance(left, (ListValue, ListValue)) and isinstance(right, NumberValue):
            # List-scalar element-wise division
            if not self._is_numeric_list(left):
                raise RuntimeError(f"Cannot perform element-wise division on list of {left.constraint or 'mixed types'}")
            if right.value == 0:
                raise RuntimeError("Division by zero")
            
            result_elements = []
            for element in left.elements:
                if isinstance(element, NumberValue):
                    result_elements.append(element.divide(right))
                else:
                    raise RuntimeError("Cannot divide non-numeric list element by number")
            return ListValue(result_elements, "num", left.position)
        elif isinstance(left, NumberValue) and isinstance(right, (ListValue, ListValue)):
            # Scalar-list element-wise division
            if not self._is_numeric_list(right):
                raise RuntimeError(f"Cannot perform element-wise division on list of {right.constraint or 'mixed types'}")
            
            result_elements = []
            for element in right.elements:
                if isinstance(element, NumberValue):
                    if element.value == 0:
                        raise RuntimeError("Division by zero in element-wise division")
                    result_elements.append(left.divide(element))
                else:
                    raise RuntimeError("Cannot divide number by non-numeric list element")
            return ListValue(result_elements, "num", right.position)
        else:
            raise RuntimeError(f"Element-wise division not supported for {left.get_type()} and {right.get_type()}")
    
    def perform_elementwise_modulo(self, left: GlangValue, right: GlangValue) -> GlangValue:
        """Perform element-wise modulo using %. operator."""
        # Import graph values for isinstance checks
        from .graph_values import ListValue

        if isinstance(left, (ListValue, ListValue)) and isinstance(right, (ListValue, ListValue)):
            if not self._is_numeric_list(left) or not self._is_numeric_list(right):
                raise RuntimeError("Element-wise modulo requires numeric lists")
            if len(left.elements) != len(right.elements):
                raise RuntimeError(f"Element-wise modulo requires lists of same length ({len(left.elements)} != {len(right.elements)})")
            
            result_elements = []
            for i in range(len(left.elements)):
                left_elem = left.elements[i]
                right_elem = right.elements[i]
                if isinstance(left_elem, NumberValue) and isinstance(right_elem, NumberValue):
                    if right_elem.value == 0:
                        raise RuntimeError("Modulo by zero in element-wise modulo")
                    result_elements.append(left_elem.modulo(right_elem))
                else:
                    raise RuntimeError("Element-wise modulo requires all elements to be numbers")
            return ListValue(result_elements, "num", left.position)
        elif isinstance(left, (ListValue, ListValue)) and isinstance(right, NumberValue):
            # List-scalar element-wise modulo
            if not self._is_numeric_list(left):
                raise RuntimeError(f"Cannot perform element-wise modulo on list of {left.constraint or 'mixed types'}")
            if right.value == 0:
                raise RuntimeError("Modulo by zero")
            
            result_elements = []
            for element in left.elements:
                if isinstance(element, NumberValue):
                    result_elements.append(element.modulo(right))
                else:
                    raise RuntimeError("Cannot perform modulo on non-numeric list element with number")
            return ListValue(result_elements, "num", left.position)
        elif isinstance(left, NumberValue) and isinstance(right, (ListValue, ListValue)):
            # Scalar-list element-wise modulo
            if not self._is_numeric_list(right):
                raise RuntimeError(f"Cannot perform element-wise modulo on list of {right.constraint or 'mixed types'}")
            
            result_elements = []
            for element in right.elements:
                if isinstance(element, NumberValue):
                    if element.value == 0:
                        raise RuntimeError("Modulo by zero in element-wise modulo")
                    result_elements.append(left.modulo(element))
                else:
                    raise RuntimeError("Cannot perform modulo on number with non-numeric list element")
            return ListValue(result_elements, "num", right.position)
        else:
            raise RuntimeError(f"Element-wise modulo not supported for {left.get_type()} and {right.get_type()}")
    
    def perform_comparison(self, left: GlangValue, right: GlangValue, operation: str) -> BooleanValue:
        """Perform comparison operations."""
        # For equality operations, use Glang's equality semantics
        if operation == "equal":
            from .graph_values import ListValue
            return BooleanValue(ListValue._glang_equals(left, right))
        elif operation == "not_equal":
            from .graph_values import ListValue
            return BooleanValue(not ListValue._glang_equals(left, right))
        
        # For ordering operations, only support compatible types
        if isinstance(left, NumberValue) and isinstance(right, NumberValue):
            if operation == "greater":
                return left.greater_than(right)
            elif operation == "less":
                return left.less_than(right)
            elif operation == "greater_equal":
                return left.greater_equal(right)
            elif operation == "less_equal":
                return left.less_equal(right)
        elif isinstance(left, StringValue) and isinstance(right, StringValue):
            # String comparisons (lexicographic)
            if operation == "greater":
                return left.greater_than(right)
            elif operation == "less":
                return left.less_than(right)
            elif operation == "greater_equal":
                return left.greater_equal(right)
            elif operation == "less_equal":
                return left.less_equal(right)
        
        raise RuntimeError(f"Cannot compare {left.get_type()} and {right.get_type()} with {operation}")
    
    # Control flow visit methods
    
    def visit_if_statement(self, node: IfStatement) -> None:
        """Execute if statement."""
        # Evaluate condition
        condition_value = self.execute(node.condition)
        
        # Convert to boolean
        if isinstance(condition_value, BooleanValue):
            condition_bool = condition_value.value
        elif isinstance(condition_value, NumberValue):
            condition_bool = condition_value.value != 0
        elif isinstance(condition_value, StringValue):
            condition_bool = len(condition_value.value) > 0
        elif isinstance(condition_value, ListValue):
            condition_bool = len(condition_value.elements) > 0
        else:
            condition_bool = True  # Default truthy
        
        # Debug output
        #print(f"DEBUG: Condition value: {condition_value}, bool: {condition_bool}")
        
        # Execute appropriate block
        if condition_bool:
            self.execute(node.then_block)
        elif node.else_block:
            self.execute(node.else_block)
    
    def visit_precision_block(self, node) -> None:
        """Execute precision context block."""
        from decimal import getcontext
        
        # Evaluate precision value
        precision_val = self.execute(node.precision_value)
        
        # Convert to integer
        if isinstance(precision_val, NumberValue):
            precision = int(precision_val.value)
        else:
            raise RuntimeError(
                f"Precision value must be a number, got {precision_val.get_type()}",
                node.position
            )
        
        # Validate precision
        if precision < 0 or precision > 1000:
            raise RuntimeError(
                f"Precision must be between 0 and 1000, got {precision}",
                node.position
            )
        
        # Save current precision
        old_precision = getcontext().prec
        
        # Import here to avoid circular imports
        from .glang_number import PrecisionGlangNumber
        
        # Save current Glang precision
        old_glang_precision = PrecisionGlangNumber._glang_decimal_places
        
        try:
            # Set Glang decimal places precision (this is the new correct behavior)
            PrecisionGlangNumber.set_glang_precision(precision)
            
            # Execute block with new precision
            self.execute(node.body)
        finally:
            # Restore previous Glang precision
            PrecisionGlangNumber.set_glang_precision(old_glang_precision)
            # Also restore Python's precision context
            getcontext().prec = old_precision
        
        self.result = None  # Precision blocks don't return values

    def visit_configuration_block(self, node) -> None:
        """Execute configuration block with scoped behavior settings."""
        # Parse configuration values from AST
        config_dict = {}
        for key, value_expr in node.configurations:
            # Execute the value expression to get its runtime value
            value = self.execute(value_expr)

            # Convert GlangValue to Python value for configuration
            if isinstance(value, BooleanValue):
                config_dict[key] = value.value
            elif isinstance(value, NumberValue):
                config_dict[key] = int(value.value)  # Configuration numbers should be integers
            elif isinstance(value, StringValue):
                config_dict[key] = value.value
            elif isinstance(value, NoneValue):
                config_dict[key] = None
            else:
                raise RuntimeError(
                    f"Configuration value for '{key}' must be a boolean, number, string, or none",
                    node.position
                )

        # Push configuration onto stack
        self.context.config.push_configuration(config_dict, scope_name="block")

        try:
            # Execute the body with the new configuration
            if node.body:
                self.execute(node.body)
        finally:
            # Always restore previous configuration
            self.context.config.pop_configuration()

        self.result = None  # Configuration blocks don't return values

    def visit_while_statement(self, node: WhileStatement) -> None:
        """Execute while loop."""
        while True:
            # Evaluate condition
            condition_value = self.execute(node.condition)
            
            # Convert to boolean
            if isinstance(condition_value, BooleanValue):
                condition_bool = condition_value.value
            elif isinstance(condition_value, NumberValue):
                condition_bool = condition_value.value != 0
            elif isinstance(condition_value, StringValue):
                condition_bool = len(condition_value.value) > 0
            elif isinstance(condition_value, ListValue):
                condition_bool = len(condition_value.elements) > 0
            else:
                condition_bool = True  # Default truthy
            
            # Exit if condition is false
            if not condition_bool:
                break
            
            # Execute body, handling break/continue
            try:
                self.execute(node.body)
            except BreakException:
                break
            except ContinueException:
                continue
    
    def visit_for_in_statement(self, node: ForInStatement) -> None:
        """Execute for-in loop."""
        # Import graph values for isinstance checks
        # Using already imported ListValue, HashValue

        # Evaluate iterable
        iterable_value = self.execute(node.iterable)

        # Ensure it's iterable (both old and new implementations)
        if isinstance(iterable_value, (ListValue, ListValue)):
            elements = iterable_value.elements
        elif isinstance(iterable_value, StringValue):
            # String is iterable by character
            elements = [StringValue(char, iterable_value.position) for char in iterable_value.value]
        elif isinstance(iterable_value, (HashValue, HashValue)):
            # Hash is iterable by keys (as data nodes)
            if hasattr(iterable_value, 'pairs'):
                # Old HashValue
                elements = [DataNodeValue(key, value, iterable_value.position)
                           for key, value in iterable_value.pairs.items()]
            else:
                # New HashValue
                elements = [DataNodeValue(key, value, iterable_value.position)
                           for key, value in iterable_value.items()]
        else:
            raise RuntimeError(f"Cannot iterate over {iterable_value.get_type()}")
        
        # Execute body for each element
        for element in elements:
            # Set loop variable
            self.context.set_variable(node.variable, element)
            
            # Execute body, handling break/continue
            try:
                self.execute(node.body)
            except BreakException:
                break
            except ContinueException:
                continue
    
    def visit_break_statement(self, node: BreakStatement) -> None:
        """Execute break statement."""
        raise BreakException()
    
    def visit_continue_statement(self, node: ContinueStatement) -> None:
        """Execute continue statement."""
        raise ContinueException()
    
    def visit_function_declaration(self, node: FunctionDeclaration) -> None:
        """Execute function declaration - creates and stores function value."""
        from .values import FunctionValue
        
        # Create function value
        func_value = FunctionValue(
            name=node.name,
            parameters=node.parameters,
            body=node.body,
            position=node.position
        )
        
        # Phase 3: Functions are now handled via AST subgraph merging
        # No variable storage - pure graph-based function discovery only
        # Functions were already added to call graph during subgraph merge phase

        # Note: Individual function declarations during REPL will still need direct addition
        # Check if this function was already added via subgraph (from file loading)
        current_scope = self.context.current_module or "global"
        existing_func = self.context.call_graph.find_function(node.name, current_scope)

        if existing_func is None:
            # REPL or individual declaration - add directly to call graph
            self.context.call_graph.add_function(node.name, func_value, current_scope)

        # Phase 3: Also add to variables if in module context for module namespace transfer
        if self.context.current_module is not None:
            # In module context - add to variables so module loading can transfer to namespace
            self.context.set_variable(node.name, func_value)

        # Store result
        self.result = func_value
    
    def visit_return_statement(self, node: ReturnStatement) -> None:
        """Execute return statement."""
        if node.value is not None:
            value = self.execute(node.value)
            raise ReturnException(value)
        else:
            raise ReturnException()
    
    def visit_function_call(self, node: FunctionCall) -> Any:
        """Execute function call."""
        from .values import FunctionValue, LambdaValue
        from .function_value import BuiltinFunctionValue
        
        # Phase 3: PURE GRAPH TRAVERSAL for function discovery
        func_value = self.context.call_graph.find_function(node.name, self.context.current_module)

        # Limited fallback ONLY for lambdas and builtin functions (not regular functions)
        if func_value is None:
            candidate = self.context.get_variable(node.name)
            # Only allow LambdaValue and BuiltinFunctionValue (not regular FunctionValue)
            if isinstance(candidate, (LambdaValue, BuiltinFunctionValue)):
                func_value = candidate
            # Regular FunctionValue should NEVER be found via variable lookup in Phase 3

        if func_value is None:
            from .errors import VariableNotFoundError
            stack_trace = create_enhanced_error_trace(f"Function '{node.name}' not found", "VariableNotFoundError")
            raise VariableNotFoundError(f"Function '{node.name}' not found", node.position, stack_trace)
        
        if not isinstance(func_value, (FunctionValue, LambdaValue, BuiltinFunctionValue)):
            raise RuntimeError(f"'{node.name}' is not a function", node.position)

        # Check arity
        if len(node.arguments) != func_value.arity():
            raise RuntimeError(
                f"Function '{node.name}' expects {func_value.arity()} arguments but got {len(node.arguments)}",
                node.position
            )
        
        # Evaluate arguments
        arg_values = [self.execute(arg) for arg in node.arguments]
        
        # Call the function
        result = self.call_function(func_value, arg_values, node.position)
        
        self.result = result
        return result
    
    def visit_lambda_expression(self, node: LambdaExpression) -> Any:
        """Execute lambda expression - creates lambda value."""
        from .values import LambdaValue
        
        # Create lambda value
        lambda_value = LambdaValue(
            parameters=node.parameters,
            body=node.body,
            position=node.position
        )
        
        self.result = lambda_value
        return lambda_value
    
    def call_function(self, func_value: 'GlangValue', arguments: List['GlangValue'], position: Optional[SourcePosition] = None) -> Any:
        """Call a function or lambda with given arguments."""
        from .values import FunctionValue, LambdaValue

        if isinstance(func_value, FunctionValue):
            # Push stack frame for function call
            func_args = dict(zip(func_value.parameters, [str(arg) for arg in arguments]))
            push_execution_frame(func_value.name, position, func_args)

            # Create new execution context for function scope
            # Save current variable state
            old_vars = self.context.variables.copy()

            try:
                # If this function has module context, add module functions to scope
                if func_value.module_context:
                    # Add all module functions and variables to current scope
                    for name, value in func_value.module_context.items():
                        self.context.set_variable(name, value)

                # Bind parameters to arguments
                for param_name, arg_value in zip(func_value.parameters, arguments):
                    self.context.set_variable(param_name, arg_value)

                # Update stack frame with current variables
                current_vars = {name: str(value) for name, value in self.context.variables.items()}
                update_frame_variables(current_vars)

                # Execute function body
                try:
                    self.execute(func_value.body)
                    # If no return statement, return None equivalent
                    from .values import NoneValue
                    return NoneValue()
                except ReturnException as ret:
                    return ret.value if ret.value is not None else NoneValue()

            finally:
                # Pop stack frame
                pop_execution_frame()
                # Restore variable state
                self.context.variables = old_vars
        
        elif isinstance(func_value, LambdaValue):
            # Push stack frame for lambda call
            lambda_args = dict(zip(func_value.parameters, [str(arg) for arg in arguments]))
            push_execution_frame("<lambda>", position, lambda_args)

            # Similar to function but execute expression instead of block
            old_vars = self.context.variables.copy()

            try:
                # Bind parameters to arguments
                for param_name, arg_value in zip(func_value.parameters, arguments):
                    self.context.set_variable(param_name, arg_value)

                # Update stack frame with current variables
                current_vars = {name: str(value) for name, value in self.context.variables.items()}
                update_frame_variables(current_vars)

                # Execute lambda body (expression)
                result = self.execute(func_value.body)
                return result

            finally:
                # Pop stack frame
                pop_execution_frame()
                # Restore variable state
                self.context.variables = old_vars
        
        elif hasattr(func_value, 'call'):  # BuiltinFunctionValue
            # Call the builtin function directly
            return func_value.call(arguments, position)
    
    def visit_block(self, node: Block) -> None:
        """Execute block of statements."""
        for statement in node.statements:
            self.execute(statement)
    
    def visit_behavior_call(self, node) -> None:
        """Execute behavior call - shouldn't be called directly."""
        # This is handled by _build_behavior_pipeline
        raise RuntimeError("BehaviorCall nodes should not be executed directly")
    
    def visit_behavior_list(self, node) -> None:
        """Execute behavior list - shouldn't be called directly."""
        # This is handled by _build_behavior_pipeline
        raise RuntimeError("BehaviorList nodes should not be executed directly")
    
    def _build_behavior_pipeline(self, behavior_list_node):
        """Build a BehaviorPipeline from a BehaviorList AST node."""
        from glang.behaviors import BehaviorPipeline
        
        pipeline = BehaviorPipeline()
        
        for behavior in behavior_list_node.behaviors:
            if isinstance(behavior, str):
                # Simple behavior name
                pipeline.add(behavior)
            else:
                # BehaviorCall with arguments
                args = []
                for arg_node in behavior.arguments:
                    arg_value = self.execute(arg_node)
                    # Convert to Python values for behavior arguments
                    if hasattr(arg_value, 'to_python'):
                        args.append(arg_value.to_python())
                    else:
                        args.append(arg_value)
                
                pipeline.add(behavior.name, *args)
        
        return pipeline

    def visit_match_expression(self, node) -> Any:
        """Execute match expression."""
        from .errors import MatchError

        # Evaluate the expression to match against
        target_value = self.execute(node.expr)

        # Try each arm in order
        for arm in node.arms:
            # Try to match pattern
            bindings = self.match_pattern(arm.pattern, target_value)
            if bindings is not None:
                # Pattern matched! Save current variables and add bindings
                saved_vars = {}
                for var_name in bindings.keys():
                    if self.context.has_variable(var_name):
                        saved_vars[var_name] = self.context.get_variable(var_name)

                try:
                    # Add pattern variable bindings
                    for var_name, var_value in bindings.items():
                        self.context.set_variable(var_name, var_value)

                    # Execute result expression with bindings
                    result = self.execute(arm.result)
                    return result
                finally:
                    # Restore original variables and remove pattern bindings
                    for var_name in bindings.keys():
                        if var_name in saved_vars:
                            self.context.set_variable(var_name, saved_vars[var_name])
                        elif var_name in self.context.variables:
                            del self.context.variables[var_name]

        # No patterns matched
        raise MatchError(f"No pattern matched value {target_value.to_display_string()}", node.position)

    def match_pattern(self, pattern, value):
        """
        Try to match a pattern against a value.
        Returns dict of variable bindings if match succeeds, None if it fails.
        """
        from .values import SymbolValue
        from .graph_values import ListValue

        if type(pattern).__name__ == 'WildcardPattern':
            # Wildcard matches anything, no bindings
            return {}

        elif type(pattern).__name__ == 'VariablePattern':
            # Variable matches anything and binds the value
            return {pattern.name: value}

        elif type(pattern).__name__ == 'LiteralPattern':
            # Literal pattern must match exactly
            if isinstance(pattern.value, SymbolValue):
                # Symbol pattern - check if value is SymbolValue with same name
                if isinstance(value, SymbolValue) and value.name == pattern.value.name:
                    return {}
                return None
            else:
                # Other literal patterns (numbers, strings, booleans)
                if hasattr(value, 'value'):
                    if value.value == pattern.value:
                        return {}
                elif value == pattern.value:
                    return {}
                return None

        elif type(pattern).__name__ == 'ListPattern':
            # List pattern must match ListValue or ListValue
            # Using already imported ListValue
            if not isinstance(value, (ListValue, ListValue)):
                return None

            # Handle rest variable (...rest syntax)
            if pattern.rest_variable:
                # Pattern like [first, ...rest]
                required_elements = len(pattern.elements)
                if len(value.elements) < required_elements:
                    return None

                bindings = {}

                # Match fixed elements
                for i, elem_pattern in enumerate(pattern.elements):
                    elem_bindings = self.match_pattern(elem_pattern, value.elements[i])
                    if elem_bindings is None:
                        return None
                    bindings.update(elem_bindings)

                # Bind rest elements
                rest_elements = value.elements[required_elements:]
                rest_list = ListValue(rest_elements, getattr(value, 'constraint', None), value.position)
                bindings[pattern.rest_variable] = rest_list

                return bindings
            else:
                # Pattern like [a, b, c] - must match exact length
                if len(value.elements) != len(pattern.elements):
                    return None

                bindings = {}
                for elem_pattern, elem_value in zip(pattern.elements, value.elements):
                    elem_bindings = self.match_pattern(elem_pattern, elem_value)
                    if elem_bindings is None:
                        return None
                    bindings.update(elem_bindings)

                return bindings

        else:
            # Unknown pattern type
            return None

    def visit_literal_pattern(self, node) -> Any:
        """Patterns are not executed directly."""
        raise RuntimeError("Patterns should not be executed directly")

    def visit_variable_pattern(self, node) -> Any:
        """Patterns are not executed directly."""
        raise RuntimeError("Patterns should not be executed directly")

    def visit_wildcard_pattern(self, node) -> Any:
        """Patterns are not executed directly."""
        raise RuntimeError("Patterns should not be executed directly")

    def visit_list_pattern(self, node) -> Any:
        """Patterns are not executed directly."""
        raise RuntimeError("Patterns should not be executed directly")
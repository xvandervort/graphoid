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


class ExecutionContext:
    """Context for AST execution with variable storage."""
    
    def __init__(self, symbol_table: SymbolTable, module_manager=None):
        self.symbol_table = symbol_table
        self.variables: Dict[str, GlangValue] = {}
        self.module_manager = module_manager  # Will be set by execution pipeline
    
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
        if node.var_type == "list" and isinstance(initializer_value, ListValue):
            if node.type_constraint:
                initializer_value.constraint = node.type_constraint
                # Validate existing elements against constraint
                for elem in initializer_value.elements:
                    if not initializer_value.validate_constraint(elem):
                        raise TypeConstraintError(
                            f"Element {elem.to_display_string()} violates list<{node.type_constraint}> constraint",
                            elem.position or node.position
                        )
        
        # Store in context
        self.context.set_variable(node.name, initializer_value)
        
        # Return description of what was declared
        constraint_str = f"<{node.type_constraint}>" if node.type_constraint else ""
        self.result = f"Declared {node.var_type}{constraint_str} variable '{node.name}'"
    
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
                if isinstance(existing_var, ListValue) and existing_var.constraint:
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
            
            if not isinstance(index_value, NumberValue) or not isinstance(index_value.value, int):
                raise RuntimeError(
                    f"List index must be integer, got {index_value.get_type()}",
                    node.target.indices[0].position
                )
            
            target_value.set_element(index_value.value, value)
            self.result = f"Set {node.target.target.name}[{index_value.value}] = {value.to_display_string()}"
        
        else:
            raise RuntimeError(f"Invalid assignment target", node.position)
    
    def visit_method_call(self, node: MethodCall) -> None:
        """Execute method call."""
        # Check if this might be a module-qualified variable access (e.g., math.pi)
        # Use string comparison instead of isinstance to avoid import issues
        is_variable_ref = type(node.target).__name__ == 'VariableRef'
        if (is_variable_ref and 
            len(node.arguments) == 0 and  # No arguments means it's likely a property access
            self.context.module_manager):
            
            # Try to resolve as module.symbol first
            module_name = node.target.name
            symbol_name = node.method_name
            module = self.context.module_manager.get_module(module_name)
            
            if module:
                symbol_value = module.namespace.get_symbol(symbol_name)
                if symbol_value is not None:
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
            raise VariableNotFoundError(node.name, node.position)
        self.result = value
    
    def visit_string_literal(self, node: StringLiteral) -> None:
        """Evaluate string literal."""
        # Remove quotes from the literal value
        cleaned_value = node.value
        if (cleaned_value.startswith('"') and cleaned_value.endswith('"')) or \
           (cleaned_value.startswith("'") and cleaned_value.endswith("'")):
            cleaned_value = cleaned_value[1:-1]
        
        self.result = StringValue(cleaned_value, node.position)
    
    def visit_number_literal(self, node: NumberLiteral) -> None:
        """Evaluate number literal."""
        self.result = NumberValue(node.value, node.position)
    
    def visit_boolean_literal(self, node: BooleanLiteral) -> None:
        """Evaluate boolean literal."""
        self.result = BooleanValue(node.value, node.position)
    
    def visit_list_literal(self, node: ListLiteral) -> None:
        """Evaluate list literal."""
        elements = []
        for elem in node.elements:
            elem_value = self.execute(elem)
            if not isinstance(elem_value, GlangValue):
                elem_value = python_to_glang_value(elem_value, elem.position)
            elements.append(elem_value)
        
        self.result = ListValue(elements, None, node.position)
    
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
        
        # Handle list indexing
        if isinstance(target_value, ListValue):
            if not isinstance(index_value, NumberValue) or not isinstance(index_value.value, int):
                raise RuntimeError(
                    f"List index must be integer, got {index_value.get_type()}",
                    node.indices[0].position
                )
            self.result = target_value.get_element(index_value.value)
        
        # Handle string indexing
        elif isinstance(target_value, StringValue):
            if not isinstance(index_value, NumberValue) or not isinstance(index_value.value, int):
                raise RuntimeError(
                    f"String index must be integer, got {index_value.get_type()}",
                    node.indices[0].position
                )
            
            idx = index_value.value
            string_val = target_value.value
            
            # Handle negative indices
            if idx < 0:
                idx = len(string_val) + idx
            
            # Check bounds
            if idx < 0 or idx >= len(string_val):
                raise RuntimeError(
                    f"String index {index_value.value} out of range for string of length {len(string_val)}",
                    node.indices[0].position
                )
            
            # Return character as a string
            self.result = StringValue(string_val[idx], node.position)
        
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
        if method_name in ['type', 'size', 'inspect']:
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
        else:
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, target_type, position)
    
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
            return target.universal_inspect()
        
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
        universal_methods = ['type', 'methods', 'can', 'inspect', 'size']
        
        # Type-specific methods
        type_methods = {
            'list': ['append', 'prepend', 'insert', 'reverse', 'indexOf', 'count', 'min', 'max', 'sum', 'sort'],
            'string': ['length', 'contains', 'up', 'toUpper', 'down', 'toLower', 'split', 'reverse', 'unique', 'chars'],
            'num': ['to'],
            'bool': ['flip', 'toggle', 'numify', 'toNum']
        }
        
        specific_methods = type_methods.get(target_type, [])
        return universal_methods + specific_methods
    
    def _dispatch_list_method(self, target: ListValue, method_name: str, 
                             args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Handle list method calls."""
        
        if method_name == "append":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"append() takes 1 argument, got {len(args)}", position)
            
            # Use ListValue's append method (includes constraint validation)
            target.append(args[0])
            return f"Appended {args[0].to_display_string()} to list"
        
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
            return f"Prepended {args[0].to_display_string()} to list"
        
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
            return f"Inserted {value_arg.to_display_string()} at index {index}"
        
        elif method_name == "reverse":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"reverse() takes no arguments, got {len(args)}", position)
            
            # Create a copy and reverse it (immutable operation)
            reversed_elements = target.elements.copy()
            reversed_elements.reverse()
            return ListValue(reversed_elements, target.constraint, position)
        
        # List analysis methods
        elif method_name == "indexOf":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"indexOf() takes 1 argument, got {len(args)}", position)
            
            search_value = args[0]
            for i, element in enumerate(target.elements):
                if element == search_value:
                    return NumberValue(i, position)
            
            # Return -1 if not found (following common convention)
            return NumberValue(-1, position)
        
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
                from .errors import RuntimeError
                raise RuntimeError("Cannot find minimum of empty list", position)
            
            # Check that all elements are numbers
            for element in target.elements:
                if not isinstance(element, NumberValue):
                    from .errors import ArgumentError
                    raise ArgumentError(f"min() requires all elements to be numbers, found {element.get_type()}", position)
            
            min_element = min(target.elements, key=lambda x: x.value)
            return NumberValue(min_element.value, position)
        
        elif method_name == "max":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"max() takes no arguments, got {len(args)}", position)
            
            if len(target.elements) == 0:
                from .errors import RuntimeError
                raise RuntimeError("Cannot find maximum of empty list", position)
            
            # Check that all elements are numbers
            for element in target.elements:
                if not isinstance(element, NumberValue):
                    from .errors import ArgumentError
                    raise ArgumentError(f"max() requires all elements to be numbers, found {element.get_type()}", position)
            
            max_element = max(target.elements, key=lambda x: x.value)
            return NumberValue(max_element.value, position)
        
        elif method_name == "sum":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"sum() takes no arguments, got {len(args)}", position)
            
            if len(target.elements) == 0:
                return NumberValue(0, position)  # Sum of empty list is 0
            
            # Check that all elements are numbers
            for element in target.elements:
                if not isinstance(element, NumberValue):
                    from .errors import ArgumentError
                    raise ArgumentError(f"sum() requires all elements to be numbers, found {element.get_type()}", position)
            
            total = sum(element.value for element in target.elements)
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
            
            # Sort based on element type
            if isinstance(first_element, NumberValue):
                sorted_elements.sort(key=lambda x: x.value)
            elif isinstance(first_element, StringValue):
                sorted_elements.sort(key=lambda x: x.value)
            elif isinstance(first_element, BooleanValue):
                sorted_elements.sort(key=lambda x: x.value)  # False < True
            else:
                from .errors import ArgumentError
                raise ArgumentError(f"sort() does not support {first_element.get_type()} elements", position)
            
            # Return new sorted list
            return ListValue(sorted_elements, target.constraint, position)
        
        # Note: map, filter, reduce will be implemented when lambda functions are available
        # For now, we skip these advanced functional programming methods
        
        else:
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, "list", position)
    
    def _dispatch_string_method(self, target: StringValue, method_name: str, 
                               args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Handle string method calls."""
        
        # Length method
        if method_name == "length":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"length() takes no arguments, got {len(args)}", position)
            
            return NumberValue(len(target.value), position)
        
        # Contains method
        elif method_name == "contains":
            if len(args) != 1:
                from .errors import ArgumentError
                raise ArgumentError(f"contains() takes 1 argument, got {len(args)}", position)
            
            if not isinstance(args[0], StringValue):
                from .errors import ArgumentError
                raise ArgumentError(f"contains() argument must be string, got {args[0].get_type()}", position)
            
            return BooleanValue(args[0].value in target.value, position)
        
        # Upper case methods (up and toUpper as alias)
        elif method_name in ["up", "toUpper"]:
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"{method_name}() takes no arguments, got {len(args)}", position)
            
            return StringValue(target.value.upper(), position)
        
        # Lower case methods (down and toLower as alias)
        elif method_name in ["down", "toLower"]:
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"{method_name}() takes no arguments, got {len(args)}", position)
            
            return StringValue(target.value.lower(), position)
        
        # Split method
        elif method_name == "split":
            if len(args) > 1:
                from .errors import ArgumentError
                raise ArgumentError(f"split() takes 0 or 1 argument, got {len(args)}", position)
            
            # Default delimiter is space
            delimiter = " "
            if len(args) == 1:
                if not isinstance(args[0], StringValue):
                    from .errors import ArgumentError
                    raise ArgumentError(f"split() argument must be string, got {args[0].get_type()}", position)
                delimiter = args[0].value
            
            # Split the string and convert to list of StringValues
            parts = target.value.split(delimiter)
            string_values = [StringValue(part, position) for part in parts]
            return ListValue(string_values, "string", position)
        
        
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
        
        else:
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, "string", position)
    
    def _dispatch_num_method(self, target: NumberValue, method_name: str,
                            args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Handle number method calls."""
        
        # to() method for precision truncation
        if method_name == "to":
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
        
        
        else:
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, "num", position)
    
    def _dispatch_bool_method(self, target: BooleanValue, method_name: str,
                             args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Handle boolean method calls."""
        
        # flip() and toggle() methods (aliases)
        if method_name in ["flip", "toggle"]:
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
        
        
        else:
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, "bool", position)
    
    # Additional visitor methods that need to be implemented
    def visit_expression_statement(self, node) -> None:
        """Visit an expression statement."""
        self.result = self.execute(node.expression)
    
    def visit_index_assignment(self, node) -> None:
        """Visit an index assignment - delegate to assignment logic."""
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
        
        if not isinstance(target_value, ListValue):
            raise RuntimeError(
                f"Cannot index {target_value.get_type()}", 
                node.target.position
            )
        
        if not isinstance(index_value, NumberValue) or not isinstance(index_value.value, int):
            raise RuntimeError(
                f"List index must be integer, got {index_value.get_type()}",
                node.target.indices[0].position
            )
        
        target_value.set_element(index_value.value, value)
        self.result = f"Set {node.target.target.name}[{index_value.value}] = {value.to_display_string()}"
    
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
        
        # Handle list slicing
        elif isinstance(target_value, ListValue):
            elements = target_value.elements
            sliced_elements = elements[start_val:stop_val:step_val]
            self.result = ListValue(sliced_elements, target_value.element_type, node.position)
        
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
        elif node.operator == "!":
            if isinstance(operand_value, BooleanValue):
                self.result = BooleanValue(not operand_value.value)
            else:
                raise RuntimeError(f"Cannot negate non-boolean value: {operand_value.get_type()}", node.position)
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
    
    def perform_addition(self, left: GlangValue, right: GlangValue) -> GlangValue:
        """Perform addition operation."""
        if isinstance(left, NumberValue) and isinstance(right, NumberValue):
            return NumberValue(left.value + right.value)
        elif isinstance(left, StringValue) and isinstance(right, StringValue):
            return StringValue(left.value + right.value)
        elif isinstance(left, ListValue) and isinstance(right, ListValue):
            # List union (concatenation) - combine all elements
            combined_elements = left.elements + right.elements
            # Use the constraint from the left list (or None if both have different constraints)
            constraint = left.constraint if left.constraint == right.constraint else None
            return ListValue(combined_elements, constraint, left.position)
        else:
            raise RuntimeError(f"Cannot add {left.get_type()} and {right.get_type()}")
    
    def perform_subtraction(self, left: GlangValue, right: GlangValue) -> GlangValue:
        """Perform subtraction operation."""
        if isinstance(left, NumberValue) and isinstance(right, NumberValue):
            return NumberValue(left.value - right.value)
        elif isinstance(left, ListValue) and isinstance(right, ListValue):
            # List difference - remove elements from left that are in right
            result_elements = []
            for element in left.elements:
                if element not in right.elements:
                    result_elements.append(element)
            return ListValue(result_elements, left.constraint, left.position)
        else:
            raise RuntimeError(f"Cannot subtract {right.get_type()} from {left.get_type()}")
    
    def perform_multiplication(self, left: GlangValue, right: GlangValue) -> GlangValue:
        """Perform multiplication operation."""
        if isinstance(left, NumberValue) and isinstance(right, NumberValue):
            return NumberValue(left.value * right.value)
        else:
            raise RuntimeError(f"Cannot multiply {left.get_type()} and {right.get_type()}")
    
    def perform_division(self, left: GlangValue, right: GlangValue) -> GlangValue:
        """Perform division operation."""
        if isinstance(left, NumberValue) and isinstance(right, NumberValue):
            if right.value == 0:
                raise RuntimeError("Division by zero")
            return NumberValue(left.value / right.value)
        else:
            raise RuntimeError(f"Cannot divide {left.get_type()} by {right.get_type()}")
    
    def perform_modulo(self, left: GlangValue, right: GlangValue) -> GlangValue:
        """Perform modulo operation."""
        if isinstance(left, NumberValue) and isinstance(right, NumberValue):
            if right.value == 0:
                raise RuntimeError("Modulo by zero")
            return NumberValue(left.value % right.value)
        else:
            raise RuntimeError(f"Cannot perform modulo on {left.get_type()} and {right.get_type()}")
    
    def perform_intersection(self, left: GlangValue, right: GlangValue) -> GlangValue:
        """Perform intersection operation."""
        if isinstance(left, ListValue) and isinstance(right, ListValue):
            # List intersection - keep elements that appear in both lists
            result_elements = []
            for element in left.elements:
                if element in right.elements and element not in result_elements:
                    result_elements.append(element)
            return ListValue(result_elements, left.constraint, left.position)
        else:
            raise RuntimeError(f"Cannot perform intersection on {left.get_type()} and {right.get_type()}")
    
    def perform_comparison(self, left: GlangValue, right: GlangValue, operation: str) -> BooleanValue:
        """Perform comparison operations."""
        if isinstance(left, NumberValue) and isinstance(right, NumberValue):
            if operation == "greater":
                return BooleanValue(left.value > right.value)
            elif operation == "less":
                return BooleanValue(left.value < right.value)
            elif operation == "greater_equal":
                return BooleanValue(left.value >= right.value)
            elif operation == "less_equal":
                return BooleanValue(left.value <= right.value)
            elif operation == "equal":
                return BooleanValue(left.value == right.value)
            elif operation == "not_equal":
                return BooleanValue(left.value != right.value)
        elif isinstance(left, StringValue) and isinstance(right, StringValue):
            if operation == "equal":
                return BooleanValue(left.value == right.value)
            elif operation == "not_equal":
                return BooleanValue(left.value != right.value)
            # String comparisons (lexicographic)
            elif operation == "greater":
                return BooleanValue(left.value > right.value)
            elif operation == "less":
                return BooleanValue(left.value < right.value)
            elif operation == "greater_equal":
                return BooleanValue(left.value >= right.value)
            elif operation == "less_equal":
                return BooleanValue(left.value <= right.value)
        elif isinstance(left, BooleanValue) and isinstance(right, BooleanValue):
            if operation == "equal":
                return BooleanValue(left.value == right.value)
            elif operation == "not_equal":
                return BooleanValue(left.value != right.value)
        
        raise RuntimeError(f"Cannot compare {left.get_type()} and {right.get_type()} with {operation}")
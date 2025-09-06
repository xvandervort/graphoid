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
    
    def __init__(self, symbol_table: SymbolTable):
        self.symbol_table = symbol_table
        self.variables: Dict[str, GlangValue] = {}
    
    def get_variable(self, name: str) -> Optional[GlangValue]:
        """Get variable value by name."""
        return self.variables.get(name)
    
    def set_variable(self, name: str, value: GlangValue) -> None:
        """Set variable value."""
        self.variables[name] = value
    
    def has_variable(self, name: str) -> bool:
        """Check if variable exists in context."""
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
        
        if not isinstance(target_value, ListValue):
            raise RuntimeError(
                f"Cannot index {target_value.get_type()}", 
                node.position
            )
        
        if not isinstance(index_value, NumberValue) or not isinstance(index_value.value, int):
            raise RuntimeError(
                f"List index must be integer, got {index_value.get_type()}",
                node.indices[0].position
            )
        
        self.result = target_value.get_element(index_value.value)
    
    # Helper methods
    def _dispatch_method(self, target: GlangValue, method_name: str, 
                        args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Dispatch method call to appropriate handler."""
        target_type = target.get_type()
        
        if target_type == "list":
            return self._dispatch_list_method(target, method_name, args, position)
        elif target_type == "string":
            return self._dispatch_string_method(target, method_name, args, position)
        else:
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, target_type, position)
    
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
            
            target.elements.reverse()
            return "Reversed list"
        
        else:
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, "list", position)
    
    def _dispatch_string_method(self, target: StringValue, method_name: str, 
                               args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Handle string method calls."""
        
        # For now, just provide basic string methods
        if method_name == "length":
            if len(args) != 0:
                from .errors import ArgumentError
                raise ArgumentError(f"length() takes no arguments, got {len(args)}", position)
            
            return NumberValue(len(target.value), position)
        
        else:
            from .errors import MethodNotFoundError
            raise MethodNotFoundError(method_name, "string", position)
    
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
        """Visit slice access (not yet fully implemented)."""
        raise RuntimeError("Slice access not yet implemented", node.position)
    
    def visit_method_call_expression(self, node) -> None:
        """Visit method call in expression context."""
        # Same as method call statement, but in expression context
        self.visit_method_call(node)
    
    def visit_load_statement(self, node: LoadStatement) -> None:
        """Visit load statement - include file in current namespace."""
        if not self.file_manager:
            raise RuntimeError("File manager not available for load operation", node.position)
        
        # For load statements within the AST executor, we need to delegate back to the execution session
        # This is handled by raising a special exception that the execution session can catch
        from .errors import LoadRequest
        raise LoadRequest(node.filename, node.position)
    
    def visit_import_statement(self, node: ImportStatement) -> None:
        """Visit import statement - load module into namespace."""
        # For now, raise an error as module system is not yet implemented
        raise NotImplementedError(
            f"Module imports not yet implemented. Use 'load \"{node.filename}\"' instead",
            node.position
        )
    
    def visit_noop(self, node) -> None:
        """Visit no-op statement - do nothing."""
        return None
"""Semantic error classes for glang analysis."""

from typing import Optional
from ..ast.nodes import SourcePosition


class SemanticError(Exception):
    """Base class for semantic analysis errors."""
    
    def __init__(self, message: str, position: Optional[SourcePosition] = None):
        self.message = message
        self.position = position
        super().__init__(self._format_error())
    
    def _format_error(self) -> str:
        """Format the error message with position information."""
        if self.position:
            return f"{self.message} at line {self.position.line}, column {self.position.column}"
        return self.message
    
    def __str__(self) -> str:
        return self._format_error()


class UndefinedVariableError(SemanticError):
    """Error for references to undefined variables."""
    
    def __init__(self, variable_name: str, position: Optional[SourcePosition] = None):
        self.variable_name = variable_name
        message = f"Undefined variable '{variable_name}'"
        super().__init__(message, position)


class TypeMismatchError(SemanticError):
    """Error for type mismatches in assignments or operations."""
    
    def __init__(self, expected: str, actual: str, context: str = "", 
                 position: Optional[SourcePosition] = None):
        self.expected = expected
        self.actual = actual  
        self.context = context
        
        if context:
            message = f"Type mismatch in {context}: expected {expected}, got {actual}"
        else:
            message = f"Type mismatch: expected {expected}, got {actual}"
        
        super().__init__(message, position)


class ConstraintViolationError(SemanticError):
    """Error for violations of type constraints."""
    
    def __init__(self, variable_name: str, constraint: str, actual_type: str,
                 position: Optional[SourcePosition] = None):
        self.variable_name = variable_name
        self.constraint = constraint
        self.actual_type = actual_type
        
        message = f"Constraint violation for '{variable_name}': expected {constraint}, got {actual_type}"
        super().__init__(message, position)


class InvalidMethodCallError(SemanticError):
    """Error for invalid method calls."""
    
    def __init__(self, method_name: str, target_type: str, reason: str = "",
                 position: Optional[SourcePosition] = None):
        self.method_name = method_name
        self.target_type = target_type
        self.reason = reason
        
        if reason:
            message = f"Invalid method call '{method_name}' on {target_type}: {reason}"
        else:
            message = f"Method '{method_name}' not available on {target_type}"
        
        super().__init__(message, position)


class RedeclarationError(SemanticError):
    """Error for redeclaring an existing variable."""
    
    def __init__(self, variable_name: str, original_position: Optional[SourcePosition] = None,
                 new_position: Optional[SourcePosition] = None):
        self.variable_name = variable_name
        self.original_position = original_position
        self.new_position = new_position
        
        message = f"Variable '{variable_name}' already declared"
        if original_position:
            message += f" (originally at line {original_position.line})"
        
        super().__init__(message, new_position)


class InvalidTypeError(SemanticError):
    """Error for invalid type names in declarations."""
    
    def __init__(self, type_name: str, position: Optional[SourcePosition] = None):
        self.type_name = type_name
        message = f"Invalid type '{type_name}'"
        super().__init__(message, position)


class InvalidConstraintError(SemanticError):
    """Error for invalid type constraints."""
    
    def __init__(self, constraint: str, base_type: str, 
                 position: Optional[SourcePosition] = None):
        self.constraint = constraint
        self.base_type = base_type
        message = f"Invalid constraint '{constraint}' for type '{base_type}'"
        super().__init__(message, position)
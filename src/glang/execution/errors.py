"""
Glang Runtime Error System

Provides runtime error handling with source position information
for better error reporting during AST execution.
"""

from typing import Optional
import sys
import os

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../../..'))

from glang.ast.nodes import SourcePosition


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
    
    def __init__(self, variable_name: str, position: Optional[SourcePosition] = None):
        message = f"Variable '{variable_name}' not found"
        super().__init__(message, position)
        self.variable_name = variable_name


class TypeConstraintError(RuntimeError):
    """Type constraint violation at runtime."""
    
    def __init__(self, message: str, position: Optional[SourcePosition] = None):
        super().__init__(message, position)


class MethodNotFoundError(RuntimeError):
    """Method not found on target type."""
    
    def __init__(self, method_name: str, target_type: str, position: Optional[SourcePosition] = None):
        message = f"Method '{method_name}' not found on type '{target_type}'"
        super().__init__(message, position)
        self.method_name = method_name
        self.target_type = target_type


class ArgumentError(RuntimeError):
    """Invalid method arguments."""
    
    def __init__(self, message: str, position: Optional[SourcePosition] = None):
        super().__init__(message, position)


class IndexError(RuntimeError):
    """Index out of bounds or invalid."""
    
    def __init__(self, message: str, position: Optional[SourcePosition] = None):
        super().__init__(message, position)
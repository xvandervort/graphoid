"""
Glang Runtime Error System

Provides runtime error handling with source position information
and enhanced stack traces for better error reporting during AST execution.
"""

from typing import Optional, TYPE_CHECKING
import sys
import os

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../../..'))

from glang.ast.nodes import SourcePosition

if TYPE_CHECKING:
    from .stack_trace import EnhancedStackTrace


class RuntimeError(Exception):
    """Base runtime error with position information and optional stack trace."""

    def __init__(self, message: str, position: Optional[SourcePosition] = None, stack_trace: Optional['EnhancedStackTrace'] = None):
        self.message = message
        self.position = position
        self.stack_trace = stack_trace
        super().__init__(self._format_error())

    def _format_error(self) -> str:
        if self.position:
            return f"Runtime error: {self.message} at line {self.position.line}, column {self.position.column}"
        return f"Runtime error: {self.message}"

    def get_enhanced_message(self) -> str:
        """Get enhanced error message with stack trace if available."""
        if self.stack_trace:
            return self.stack_trace.format_full_trace()
        return self._format_error()

    def get_compact_message(self) -> str:
        """Get compact error message with call chain if available."""
        if self.stack_trace:
            return self.stack_trace.format_compact_trace()
        return self._format_error()


class VariableNotFoundError(RuntimeError):
    """Variable not found in execution context."""

    def __init__(self, variable_name: str, position: Optional[SourcePosition] = None, stack_trace: Optional['EnhancedStackTrace'] = None):
        message = f"Variable '{variable_name}' not found"
        super().__init__(message, position, stack_trace)
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


class LoadRequest(Exception):
    """Special exception to request file loading from execution session."""
    
    def __init__(self, filename: str, position: Optional[SourcePosition] = None):
        self.filename = filename
        self.position = position
        super().__init__(f"Load request: {filename}")

class ImportRequest(Exception):
    """Special exception to request module import from execution session."""

    def __init__(self, filename: str, alias: Optional[str] = None, position: Optional[SourcePosition] = None):
        self.filename = filename
        self.alias = alias
        self.position = position
        super().__init__(f"Import request: {filename} as {alias}")


class MatchError(RuntimeError):
    """Error when no pattern matches in a match expression."""

    def __init__(self, message: str, position: Optional[SourcePosition] = None):
        super().__init__(message, position)
"""
Function value types for Glang

Provides value wrappers for functions and built-in operations.
"""

from typing import Callable, List, Optional, Any
from .values import GlangValue, StringValue
from ..ast.nodes import SourcePosition


class BuiltinFunctionValue(GlangValue):
    """Represents a built-in function that can be called from Glang."""
    
    def __init__(self, name: str, func: Callable, position: Optional[SourcePosition] = None):
        """Initialize a built-in function value.
        
        Args:
            name: The function name
            func: The Python callable implementing the function
            position: Optional source position
        """
        super().__init__(position)
        self.name = name
        self.func = func
    
    def to_python(self) -> Callable:
        """Return the underlying Python function."""
        return self.func
    
    def get_type(self) -> str:
        """Return the type name."""
        return "builtin_function"
    
    def to_display_string(self) -> str:
        """Return display representation."""
        return f"<builtin function {self.name}>"
    
    def call(self, args: List[GlangValue], position: Optional[SourcePosition] = None) -> GlangValue:
        """Call the built-in function with the given arguments.
        
        Args:
            args: List of GlangValue arguments
            position: Source position for error reporting
            
        Returns:
            The result as a GlangValue
        """
        try:
            # Call the function with unpacked arguments and position
            if len(args) == 0:
                # No arguments case
                return self.func(position=position)
            elif len(args) == 1:
                # Single argument case
                return self.func(args[0], position=position)
            elif len(args) == 2:
                # Two argument case
                return self.func(args[0], args[1], position=position)
            elif len(args) == 3:
                # Three argument case
                return self.func(args[0], args[1], args[2], position=position)
            else:
                # For more arguments, pass as a list
                return self.func(*args, position=position)
        except TypeError as e:
            # Handle argument count mismatch
            from ..execution.errors import RuntimeError
            raise RuntimeError(
                f"Error calling {self.name}: {str(e)}", 
                position
            )
    
    def universal_size(self) -> 'GlangValue':
        """Built-in functions have size 1."""
        from .values import NumberValue
        return NumberValue(1, self.position)
    
    def universal_inspect(self) -> 'StringValue':
        """Return inspection info for built-in function."""
        info = f"<builtin function {self.name}>"
        return StringValue(info, self.position)
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
            # Determine if function accepts position parameter
            import inspect
            sig = inspect.signature(self.func)
            accepts_position = 'position' in sig.parameters
            
            # Call the function with appropriate arguments
            if accepts_position:
                return self.func(*args, position=position)
            else:
                return self.func(*args)
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
    
    def arity(self) -> int:
        """Return the number of parameters this function expects.
        
        For builtin functions, we'll use introspection of the Python function.
        """
        import inspect
        try:
            sig = inspect.signature(self.func)
            # Count parameters, excluding 'position' parameter if present
            params = [p for name, p in sig.parameters.items() if name != 'position']
            return len(params)
        except (ValueError, TypeError):
            # If we can't inspect the signature, assume variable args
            return 0  # Accept any number of arguments
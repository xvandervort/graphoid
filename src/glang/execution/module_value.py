"""
Module value type for Glang

Provides value wrapper for modules to make them accessible as variables.
"""

from typing import Optional
from .values import GlangValue, StringValue
from ..ast.nodes import SourcePosition
from ..modules.module_manager import Module


class ModuleValue(GlangValue):
    """Represents a module that can be accessed from Glang."""
    
    def __init__(self, module: Module, position: Optional[SourcePosition] = None):
        """Initialize a module value.
        
        Args:
            module: The Module instance
            position: Optional source position
        """
        super().__init__(position)
        self.module = module
        self.name = module.name
    
    def to_python(self) -> Module:
        """Return the underlying Module."""
        return self.module
    
    def get_type(self) -> str:
        """Return the type name."""
        return "module"
    
    def to_display_string(self) -> str:
        """Return display representation."""
        return f"<module '{self.name}'>"
    
    def universal_size(self) -> 'GlangValue':
        """Modules have size 1."""
        from .values import NumberValue
        return NumberValue(1, self.position)
    
    def universal_inspect(self) -> 'StringValue':
        """Return inspection info for module."""
        # Count symbols in the module
        symbol_count = len(self.module.namespace.symbols)
        info = f"<module '{self.name}' with {symbol_count} symbols>"
        return StringValue(info, self.position)
"""Error classes for the module system."""

from typing import Optional
from ..ast.nodes import SourcePosition

class ModuleError(Exception):
    """Base class for module-related errors."""
    
    def __init__(self, message: str, position: Optional[SourcePosition] = None):
        self.message = message
        self.position = position
        super().__init__(message)
    
    def __str__(self):
        if self.position:
            return f"{self.message} at {self.position}"
        return self.message

class ModuleNotFoundError(ModuleError):
    """Raised when a module file cannot be found."""
    
    def __init__(self, filename: str, position: Optional[SourcePosition] = None):
        super().__init__(f"Module not found: {filename}", position)
        self.filename = filename

class CircularImportError(ModuleError):
    """Raised when a circular import is detected."""
    
    def __init__(self, filename: str, import_chain: list, position: Optional[SourcePosition] = None):
        chain_str = " -> ".join(import_chain + [filename])
        super().__init__(f"Circular import detected: {chain_str}", position)
        self.filename = filename
        self.import_chain = import_chain

class ModuleSymbolError(ModuleError):
    """Raised when accessing an undefined symbol in a module."""
    
    def __init__(self, module_name: str, symbol_name: str, position: Optional[SourcePosition] = None):
        super().__init__(f"Symbol '{symbol_name}' not found in module '{module_name}'", position)
        self.module_name = module_name
        self.symbol_name = symbol_name
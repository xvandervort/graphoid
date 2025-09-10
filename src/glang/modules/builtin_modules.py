"""
Built-in module registry for Glang

Manages built-in modules that are always available without import.
"""

from typing import Dict, Optional
from .module_manager import Module, ModuleNamespace


class BuiltinModuleRegistry:
    """Registry for built-in modules."""
    
    _builtin_modules: Dict[str, ModuleNamespace] = {}
    _initialized = False
    
    @classmethod
    def initialize(cls):
        """Initialize all built-in modules."""
        if cls._initialized:
            return
        
        # Register I/O module
        from .io_module import create_io_module_namespace
        cls._builtin_modules['io'] = create_io_module_namespace()
        
        # Register JSON module
        from .json_module import create_json_module_namespace
        cls._builtin_modules['json'] = create_json_module_namespace()
        
        cls._initialized = True
    
    @classmethod
    def get_builtin_module(cls, name: str) -> Optional[ModuleNamespace]:
        """Get a built-in module namespace by name.
        
        Args:
            name: The module name (e.g., 'io')
            
        Returns:
            ModuleNamespace if found, None otherwise
        """
        cls.initialize()
        return cls._builtin_modules.get(name)
    
    @classmethod
    def is_builtin_module(cls, name: str) -> bool:
        """Check if a name refers to a built-in module.
        
        Args:
            name: The module name to check
            
        Returns:
            True if it's a built-in module, False otherwise
        """
        cls.initialize()
        return name in cls._builtin_modules
    
    @classmethod
    def list_builtin_modules(cls) -> list[str]:
        """Get list of all built-in module names.
        
        Returns:
            List of module names
        """
        cls.initialize()
        return list(cls._builtin_modules.keys())
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
        
        # Register Crypto module
        from .crypto_module import create_crypto_module_namespace
        cls._builtin_modules['crypto'] = create_crypto_module_namespace()
        
        # Register Time module
        from .time_module_simple import create_time_module_namespace
        cls._builtin_modules['time'] = create_time_module_namespace()

        # Register Network module (low-level HTTP operations)
        from .network_module import create_network_module
        cls._builtin_modules['http'] = create_network_module()

        # Register HTML module (low-level HTML parsing)
        from .html_module import create_html_module
        cls._builtin_modules['html_parser'] = create_html_module()

        # Register Call Graph module (graph introspection)
        from .call_graph_module import create_call_graph_module_namespace
        cls._builtin_modules['call_graph'] = create_call_graph_module_namespace()

        # NOTE: CSV module is now implemented as a Glang file (stdlib/csv_simple.gr)
        # It will be loaded as a regular module, not a built-in Python wrapper

        # NOTE: Regex module is now implemented as a Glang file (stdlib/regex.gr)
        # It will be loaded as a regular module, not a built-in Python wrapper
        
        # NOTE: Random module is now implemented as a Glang file (stdlib/random.gr)  
        # It will be loaded as a regular module, not a built-in Python wrapper
        
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
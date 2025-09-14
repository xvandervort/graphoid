"""Module manager for handling imports and module namespaces."""

import os
from typing import Dict, Optional, Set, Any
from dataclasses import dataclass, field
from pathlib import Path

from ..execution.values import GlangValue
from ..files.file_manager import FileManager
from ..ast.nodes import SourcePosition
from .errors import ModuleNotFoundError, CircularImportError, ModuleSymbolError


@dataclass
class ModuleNamespace:
    """Represents a module's namespace containing its exported symbols."""
    
    filename: str
    symbols: Dict[str, GlangValue] = field(default_factory=dict)
    exported_symbols: Set[str] = field(default_factory=set)
    
    def get_symbol(self, name: str) -> Optional[GlangValue]:
        """Get a symbol from the module namespace."""
        # For now, all symbols are public (until we implement export control)
        return self.symbols.get(name)
    
    def set_symbol(self, name: str, value: GlangValue, export: bool = True):
        """Set a symbol in the module namespace."""
        self.symbols[name] = value
        if export:
            self.exported_symbols.add(name)
    
    def is_exported(self, name: str) -> bool:
        """Check if a symbol is exported."""
        # For now, all symbols are exported (until we implement export control)
        return name in self.symbols


@dataclass
class Module:
    """Represents a loaded module."""
    
    filename: str
    import_alias: Optional[str]  # Import-site alias override
    namespace: ModuleNamespace
    declared_name: Optional[str] = None  # Module-declared name from 'module' statement
    declared_alias: Optional[str] = None  # Module-declared alias from 'alias' statement
    
    @property
    def name(self) -> str:
        """Get the module's effective name.
        
        Priority order:
        1. Import-site alias (highest priority)
        2. Module-declared alias (from 'alias' statement)
        3. Module-declared name (from 'module' statement) 
        4. Filename without extension (fallback)
        """
        if self.import_alias:
            return self.import_alias
        if self.declared_alias:
            return self.declared_alias
        if self.declared_name:
            return self.declared_name
        # Extract module name from filename (e.g., "math.gr" -> "math")
        return Path(self.filename).stem


class ModuleManager:
    """Manages module loading, caching, and namespace resolution."""
    
    def __init__(self, file_manager: Optional[FileManager] = None):
        """Initialize the module manager.
        
        Args:
            file_manager: FileManager instance for loading module files
        """
        self.file_manager = file_manager or FileManager()
        self.loaded_modules: Dict[str, Module] = {}  # filename -> Module
        self.module_aliases: Dict[str, Module] = {}  # alias -> Module
        self.import_stack: list = []  # For circular dependency detection
        self.current_file_context: Optional[str] = None  # Current file being processed
    
    def import_module(self, filename: str, alias: Optional[str] = None, 
                     position: Optional[SourcePosition] = None) -> Module:
        """Import a module and return its namespace.
        
        Args:
            filename: Path to the module file or built-in module name
            alias: Optional alias for the module
            position: Source position for error reporting
            
        Returns:
            Module instance with loaded namespace
            
        Raises:
            ModuleNotFoundError: If the module file doesn't exist
            CircularImportError: If circular import detected
        """
        # Check if this is a built-in module first
        from .builtin_modules import BuiltinModuleRegistry
        
        # Try without .gr extension first for built-in modules
        module_name = filename.replace('.gr', '') if filename.endswith('.gr') else filename
        
        if BuiltinModuleRegistry.is_builtin_module(module_name):
            # Check if already loaded with this name
            if module_name in self.loaded_modules:
                module = self.loaded_modules[module_name]
                # If importing with a different alias, update the alias mapping
                if alias and alias != module.import_alias:
                    self.module_aliases[alias] = module
                return module
            
            # Create a new Module instance for the built-in
            namespace = BuiltinModuleRegistry.get_builtin_module(module_name)
            module = Module(
                filename=module_name,  # Use module name as filename for built-ins
                import_alias=alias,
                namespace=namespace,
                declared_name=module_name  # Built-in modules have their name declared
            )
            
            # Cache the module
            self.loaded_modules[module_name] = module
            
            # Add to aliases
            effective_name = module.name
            if effective_name:
                self.module_aliases[effective_name] = module
            
            return module
        
        # Not a built-in, proceed with file-based module loading
        # Normalize the filename
        filename = self._normalize_path(filename)
        
        # Check for circular imports
        if filename in self.import_stack:
            raise CircularImportError(filename, self.import_stack.copy(), position)
        
        # Check if module is already loaded
        if filename in self.loaded_modules:
            module = self.loaded_modules[filename]
            # If importing with a different alias, update the alias mapping
            if alias and alias != module.import_alias:
                self.module_aliases[alias] = module
            return module
        
        # Check if file exists
        if not self._file_exists(filename):
            raise ModuleNotFoundError(filename, position)
        
        # Add to import stack for circular dependency detection
        self.import_stack.append(filename)
        
        try:
            # Extract declared module name and alias from the file
            declared_name, declared_alias = self._extract_module_declarations(filename)
            
            # Create module namespace
            namespace = ModuleNamespace(filename)
            
            # Create module instance with declared name and alias
            module = Module(filename, alias, namespace, declared_name, declared_alias)
            
            # Cache the module
            self.loaded_modules[filename] = module
            
            # Add to aliases - prefer import-site alias, then declared name
            effective_name = module.name
            if effective_name:
                self.module_aliases[effective_name] = module
            
            # Load and execute the module file
            # This will be done by the execution pipeline
            # For now, we just return the empty module
            
            return module
            
        finally:
            # Remove from import stack
            self.import_stack.pop()
    
    def get_module(self, name: str) -> Optional[Module]:
        """Get a module by name or alias.
        
        Args:
            name: Module name or alias
            
        Returns:
            Module instance or None if not found
        """
        # Check aliases first
        if name in self.module_aliases:
            return self.module_aliases[name]
        
        # Check by filename
        if name in self.loaded_modules:
            return self.loaded_modules[name]
        
        # Try adding .gr extension
        filename = f"{name}.gr"
        if filename in self.loaded_modules:
            return self.loaded_modules[filename]
        
        return None
    
    def get_module_symbol(self, module_name: str, symbol_name: str, 
                         position: Optional[SourcePosition] = None) -> GlangValue:
        """Get a symbol from a module's namespace.
        
        Args:
            module_name: Name or alias of the module
            symbol_name: Name of the symbol to retrieve
            position: Source position for error reporting
            
        Returns:
            The symbol's value
            
        Raises:
            ModuleSymbolError: If module or symbol not found
        """
        module = self.get_module(module_name)
        if not module:
            raise ModuleSymbolError(module_name, symbol_name, position)
        
        value = module.namespace.get_symbol(symbol_name)
        if value is None:
            raise ModuleSymbolError(module_name, symbol_name, position)
        
        return value
    
    def clear_modules(self):
        """Clear all loaded modules."""
        self.loaded_modules.clear()
        self.module_aliases.clear()
        self.import_stack.clear()
    
    def set_current_file_context(self, filepath: str):
        """Set the current file being processed for relative path resolution."""
        self.current_file_context = os.path.abspath(filepath)
    
    def clear_current_file_context(self):
        """Clear the current file context."""
        self.current_file_context = None
    
    def _normalize_path(self, filename: str) -> str:
        """Normalize a file path for consistent module identification."""
        # Ensure .gr extension
        if not filename.endswith('.gr'):
            filename = f"{filename}.gr"
        
        # If it's already absolute, return as is
        if os.path.isabs(filename):
            return filename
        
        # Try to resolve relative to current file context
        if self.current_file_context:
            context_dir = os.path.dirname(self.current_file_context)
            candidate = os.path.join(context_dir, filename)
            if os.path.exists(candidate):
                return os.path.abspath(candidate)
        
        # If we're currently processing a file (in import stack),
        # resolve relative to that file's directory
        if self.import_stack:
            for import_path in reversed(self.import_stack):
                if os.path.isabs(import_path):
                    import_dir = os.path.dirname(import_path)
                    candidate = os.path.join(import_dir, filename)
                    if os.path.exists(candidate):
                        return os.path.abspath(candidate)
        
        # Try stdlib directory
        stdlib_dir = os.path.join(os.path.dirname(__file__), '..', '..', '..', 'stdlib')
        stdlib_candidate = os.path.join(stdlib_dir, filename)
        if os.path.exists(stdlib_candidate):
            return os.path.abspath(stdlib_candidate)
        
        # Try current directory
        if os.path.exists(filename):
            return os.path.abspath(filename)
        
        # Return as is if we can't resolve it (let the error handling catch it)
        return filename
    
    def _file_exists(self, filename: str) -> bool:
        """Check if a module file exists."""
        # If it's already an absolute path, just check if it exists
        if os.path.isabs(filename):
            return os.path.exists(filename)
        
        # Try relative to current file context first
        if self.current_file_context:
            context_dir = os.path.dirname(self.current_file_context)
            candidate = os.path.join(context_dir, filename)
            if os.path.exists(candidate):
                return True
        
        # Try stdlib directory
        stdlib_dir = os.path.join(os.path.dirname(__file__), '..', '..', '..', 'stdlib')
        stdlib_candidate = os.path.join(stdlib_dir, filename)
        if os.path.exists(stdlib_candidate):
            return True
        
        # Try current directory
        if os.path.exists(filename):
            return True
        
        # If we're currently processing a file (in import stack),
        # try relative to that file's directory
        if self.import_stack:
            for import_path in reversed(self.import_stack):
                if os.path.isabs(import_path):
                    import_dir = os.path.dirname(import_path)
                    candidate = os.path.join(import_dir, filename)
                    if os.path.exists(candidate):
                        return True
        
        # Could add more search paths here (e.g., lib/, modules/, etc.)
        return False
    
    def _extract_module_declarations(self, filename: str) -> tuple[Optional[str], Optional[str]]:
        """Extract the declared module name and alias from a file.
        
        Returns:
            Tuple of (declared_name, declared_alias). Both can be None.
        """
        try:
            # Read and parse the file to look for module/alias declarations
            with open(filename, 'r') as f:
                content = f.read()
            
            # Parse the content to find module and alias declarations
            from ..parser import ASTParser
            from ..ast.nodes import ModuleDeclaration, AliasDeclaration
            
            parser = ASTParser()
            declared_name = None
            declared_alias = None
            
            # Parse each line separately to find declarations
            lines = content.strip().split('\n')
            for line in lines:
                line = line.strip()
                if line.startswith('module ') and not declared_name:
                    try:
                        ast = parser.parse(line)
                        if isinstance(ast, ModuleDeclaration):
                            declared_name = ast.name
                    except:
                        # If parsing fails, continue looking
                        continue
                elif line.startswith('alias ') and not declared_alias:
                    try:
                        ast = parser.parse(line)
                        if isinstance(ast, AliasDeclaration):
                            declared_alias = ast.name
                    except:
                        # If parsing fails, continue looking
                        continue
                
                # If we found both, we can stop looking
                if declared_name and declared_alias:
                    break
            
            return declared_name, declared_alias
            
        except Exception:
            # If we can't read or parse the file, just return None for both
            return None, None
"""Tests for module namespace management."""

import pytest
import os
import tempfile
from src.glang.modules.module_manager import ModuleManager, ModuleNamespace
from src.glang.modules.errors import ModuleNotFoundError, CircularImportError
from src.glang.execution.values import StringValue, NumberValue
from src.glang.files.file_manager import FileManager

class TestModuleNamespace:
    """Test ModuleNamespace functionality."""
    
    def test_create_namespace(self):
        """Test creating a module namespace."""
        ns = ModuleNamespace("test.gr")
        assert ns.filename == "test.gr"
        assert len(ns.symbols) == 0
        assert len(ns.exported_symbols) == 0
    
    def test_set_and_get_symbol(self):
        """Test setting and getting symbols."""
        ns = ModuleNamespace("test.gr")
        value = StringValue("hello")
        
        ns.set_symbol("greeting", value)
        assert ns.get_symbol("greeting") == value
        assert ns.is_exported("greeting")
    
    def test_get_nonexistent_symbol(self):
        """Test getting non-existent symbol returns None."""
        ns = ModuleNamespace("test.gr")
        assert ns.get_symbol("missing") is None


class TestModuleManager:
    """Test ModuleManager functionality."""
    
    def setup_method(self):
        """Setup for each test."""
        self.file_manager = FileManager()
        self.module_manager = ModuleManager(self.file_manager)
    
    def test_module_manager_creation(self):
        """Test creating module manager."""
        assert self.module_manager.file_manager is not None
        assert len(self.module_manager.loaded_modules) == 0
        assert len(self.module_manager.module_aliases) == 0
    
    def test_import_nonexistent_module_fails(self):
        """Test importing non-existent module fails."""
        with pytest.raises(ModuleNotFoundError) as exc:
            self.module_manager.import_module("nonexistent.gr")
        assert "nonexistent.gr" in str(exc.value)
    
    def test_import_module_with_alias(self):
        """Test importing module with alias."""
        # Create a temporary test file
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('string greeting = "Hello from module"')
            temp_file = f.name
        
        try:
            module = self.module_manager.import_module(temp_file, "test_mod")
            
            assert module.filename == temp_file
            assert module.import_alias == "test_mod"
            assert module.name == "test_mod"
            assert temp_file in self.module_manager.loaded_modules
            assert "test_mod" in self.module_manager.module_aliases
        finally:
            os.unlink(temp_file)
    
    def test_import_module_without_alias(self):
        """Test importing module without alias uses filename."""
        # Create a temporary test file
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('string greeting = "Hello"')
            temp_file = f.name
        
        try:
            module = self.module_manager.import_module(temp_file)
            
            # Name should be derived from filename without extension
            expected_name = os.path.splitext(os.path.basename(temp_file))[0]
            assert module.name == expected_name
            assert module.import_alias is None
        finally:
            os.unlink(temp_file)
    
    def test_get_module_by_name(self):
        """Test getting module by name."""
        # Create a temporary test file
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('num value = 42')
            temp_file = f.name
        
        try:
            # Import with alias
            module = self.module_manager.import_module(temp_file, "test_alias")
            
            # Should be able to get by alias
            found = self.module_manager.get_module("test_alias")
            assert found is module
            
            # Should also work by filename
            found = self.module_manager.get_module(temp_file)
            assert found is module
        finally:
            os.unlink(temp_file)
    
    def test_get_nonexistent_module(self):
        """Test getting non-existent module returns None."""
        assert self.module_manager.get_module("nonexistent") is None
    
    def test_circular_import_detection(self):
        """Test circular import detection."""
        # This is a simplified test - real circular imports would require
        # actual files that import each other
        manager = self.module_manager
        
        # Simulate being in an import chain
        manager.import_stack.append("file1.gr")
        manager.import_stack.append("file2.gr")
        
        # Now trying to import file1.gr should detect the circular dependency
        with pytest.raises(CircularImportError) as exc:
            # This will fail at file existence check, but we're testing the
            # circular dependency detection which happens first
            try:
                manager.import_module("file1.gr")
            except ModuleNotFoundError:
                # Expected because file doesn't exist, but check the stack was used
                pass
        
        # The error should contain the import chain
        assert "file1.gr" in manager.import_stack
    
    def test_clear_modules(self):
        """Test clearing all modules."""
        # Create a temporary test file
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('bool flag = true')
            temp_file = f.name
        
        try:
            # Import a module
            self.module_manager.import_module(temp_file, "temp_mod")
            
            # Verify it's loaded
            assert len(self.module_manager.loaded_modules) > 0
            assert len(self.module_manager.module_aliases) > 0
            
            # Clear all modules
            self.module_manager.clear_modules()
            
            # Verify everything is cleared
            assert len(self.module_manager.loaded_modules) == 0
            assert len(self.module_manager.module_aliases) == 0
            assert len(self.module_manager.import_stack) == 0
        finally:
            os.unlink(temp_file)
    
    def test_module_declared_name_preferred(self):
        """Test that module-declared names are preferred over filename."""
        # Create a file with module declaration
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('module math\nnum pi = 3.14159')
            temp_file = f.name
        
        try:
            # Import without alias - should use declared name
            module = self.module_manager.import_module(temp_file)
            
            assert module.declared_name == "math"
            assert module.name == "math"  # Should prefer declared name
            
            # Should be accessible by declared name
            found = self.module_manager.get_module("math")
            assert found is module
        finally:
            os.unlink(temp_file)
    
    def test_import_alias_overrides_declared_name(self):
        """Test that import-site alias overrides declared name."""
        # Create a file with module declaration
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('module math\nnum pi = 3.14159')
            temp_file = f.name
        
        try:
            # Import with explicit alias - should use alias over declared name
            module = self.module_manager.import_module(temp_file, "custom_math")
            
            assert module.declared_name == "math"
            assert module.import_alias == "custom_math"
            assert module.name == "custom_math"  # Should prefer alias
            
            # Should be accessible by alias
            found = self.module_manager.get_module("custom_math")
            assert found is module
            
            # Should NOT be accessible by declared name when alias is specified
            found_by_declared = self.module_manager.get_module("math")
            assert found_by_declared is None
        finally:
            os.unlink(temp_file)
    
    def test_module_without_declaration_uses_filename(self):
        """Test that modules without declaration use filename."""
        # Create a file without module declaration
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('num value = 42')
            temp_file = f.name
        
        try:
            module = self.module_manager.import_module(temp_file)
            
            assert module.declared_name is None
            # Should use filename stem
            expected_name = os.path.splitext(os.path.basename(temp_file))[0]
            assert module.name == expected_name
        finally:
            os.unlink(temp_file)
    
    def test_dual_module_alias_declarations(self):
        """Test module with both module and alias declarations."""
        # Create a file with both module and alias declarations
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('module advanced_scientific_mathematics\nalias math\nnum pi = 3.14159')
            temp_file = f.name
        
        try:
            # Import without alias - should use declared alias
            module = self.module_manager.import_module(temp_file)
            
            assert module.declared_name == "advanced_scientific_mathematics"
            assert module.declared_alias == "math"
            assert module.name == "math"  # Should prefer declared alias
            
            # Should be accessible by declared alias
            found = self.module_manager.get_module("math")
            assert found is module
        finally:
            os.unlink(temp_file)
    
    def test_import_alias_overrides_all_declarations(self):
        """Test that import alias overrides both module and alias declarations."""
        # Create a file with both module and alias declarations
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('module advanced_scientific_mathematics\nalias math\nnum pi = 3.14159')
            temp_file = f.name
        
        try:
            # Import with explicit alias - should override everything
            module = self.module_manager.import_module(temp_file, "custom")
            
            assert module.declared_name == "advanced_scientific_mathematics"
            assert module.declared_alias == "math"
            assert module.import_alias == "custom"
            assert module.name == "custom"  # Import alias wins
            
            # Should be accessible by import alias only
            found = self.module_manager.get_module("custom")
            assert found is module
            
            # Should NOT be accessible by declared alias when import alias specified
            found_by_declared = self.module_manager.get_module("math")
            assert found_by_declared is None
        finally:
            os.unlink(temp_file)
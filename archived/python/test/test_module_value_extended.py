"""Additional tests for module value type."""

import pytest
from unittest.mock import Mock, MagicMock
from src.glang.execution.module_value import ModuleValue
from src.glang.execution.values import NumberValue, StringValue
from src.glang.ast.nodes import SourcePosition


class TestModuleValue:
    """Test ModuleValue class."""
    
    def setup_method(self):
        """Set up test fixtures."""
        # Create a mock module
        self.mock_module = Mock()
        self.mock_module.name = "test_module"
        
        # Create a mock namespace with symbols
        self.mock_namespace = Mock()
        self.mock_namespace.symbols = {"symbol1": "value1", "symbol2": "value2"}
        self.mock_module.namespace = self.mock_namespace
    
    def test_basic_construction(self):
        """Test basic construction of module value."""
        module_value = ModuleValue(self.mock_module)
        
        assert module_value.module is self.mock_module
        assert module_value.name == "test_module"
        assert module_value.position is None
    
    def test_construction_with_position(self):
        """Test construction with position information."""
        position = SourcePosition(5, 10)
        module_value = ModuleValue(self.mock_module, position)
        
        assert module_value.module is self.mock_module
        assert module_value.name == "test_module"
        assert module_value.position == position
    
    def test_to_python(self):
        """Test to_python method returns underlying module."""
        module_value = ModuleValue(self.mock_module)
        result = module_value.to_python()
        
        assert result is self.mock_module
    
    def test_get_type(self):
        """Test get_type returns correct type string."""
        module_value = ModuleValue(self.mock_module)
        
        assert module_value.get_type() == "module"
    
    def test_to_display_string(self):
        """Test display string representation."""
        module_value = ModuleValue(self.mock_module)
        display = module_value.to_display_string()
        
        assert display == "<module 'test_module'>"
    
    def test_to_display_string_different_names(self):
        """Test display string with different module names."""
        # Test with different module name
        different_module = Mock()
        different_module.name = "math_module"
        different_module.namespace = Mock()
        different_module.namespace.symbols = {}
        
        module_value = ModuleValue(different_module)
        display = module_value.to_display_string()
        
        assert display == "<module 'math_module'>"
    
    def test_universal_size(self):
        """Test universal_size returns 1."""
        module_value = ModuleValue(self.mock_module)
        size = module_value.universal_size()
        
        assert isinstance(size, NumberValue)
        assert size.glang_number.to_python_int() == 1
    
    def test_universal_size_with_position(self):
        """Test universal_size preserves position."""
        position = SourcePosition(3, 7)
        module_value = ModuleValue(self.mock_module, position)
        size = module_value.universal_size()
        
        assert size.position == position
    
    def test_universal_inspect(self):
        """Test universal_inspect returns module info."""
        module_value = ModuleValue(self.mock_module)
        inspect_result = module_value.universal_inspect()
        
        assert isinstance(inspect_result, StringValue)
        expected = "<module 'test_module' with 2 symbols>"
        assert inspect_result.value == expected
    
    def test_universal_inspect_with_position(self):
        """Test universal_inspect preserves position."""
        position = SourcePosition(8, 15)
        module_value = ModuleValue(self.mock_module, position)
        inspect_result = module_value.universal_inspect()
        
        assert inspect_result.position == position
    
    def test_universal_inspect_empty_module(self):
        """Test universal_inspect with empty module."""
        empty_module = Mock()
        empty_module.name = "empty_module"
        empty_module.namespace = Mock()
        empty_module.namespace.symbols = {}
        
        module_value = ModuleValue(empty_module)
        inspect_result = module_value.universal_inspect()
        
        assert isinstance(inspect_result, StringValue)
        expected = "<module 'empty_module' with 0 symbols>"
        assert inspect_result.value == expected
    
    def test_universal_inspect_many_symbols(self):
        """Test universal_inspect with many symbols."""
        many_symbols_module = Mock()
        many_symbols_module.name = "large_module"
        many_symbols_module.namespace = Mock()
        many_symbols_module.namespace.symbols = {f"symbol{i}": f"value{i}" for i in range(10)}
        
        module_value = ModuleValue(many_symbols_module)
        inspect_result = module_value.universal_inspect()
        
        assert isinstance(inspect_result, StringValue)
        expected = "<module 'large_module' with 10 symbols>"
        assert inspect_result.value == expected


class TestModuleValueEdgeCases:
    """Test edge cases for ModuleValue."""
    
    def setup_method(self):
        """Set up test fixtures."""
        # Create a mock module
        self.mock_module = Mock()
        self.mock_module.name = "test_module"
        
        # Create a mock namespace with symbols
        self.mock_namespace = Mock()
        self.mock_namespace.symbols = {"symbol1": "value1", "symbol2": "value2"}
        self.mock_module.namespace = self.mock_namespace
    
    def test_module_with_special_characters_in_name(self):
        """Test module with special characters in name."""
        special_module = Mock()
        special_module.name = "special-module_123"
        special_module.namespace = Mock()
        special_module.namespace.symbols = {}
        
        module_value = ModuleValue(special_module)
        
        assert module_value.name == "special-module_123"
        assert module_value.to_display_string() == "<module 'special-module_123'>"
    
    def test_module_with_empty_name(self):
        """Test module with empty name."""
        empty_name_module = Mock()
        empty_name_module.name = ""
        empty_name_module.namespace = Mock()
        empty_name_module.namespace.symbols = {}
        
        module_value = ModuleValue(empty_name_module)
        
        assert module_value.name == ""
        assert module_value.to_display_string() == "<module ''>"
    
    def test_module_with_very_long_name(self):
        """Test module with very long name."""
        long_name = "a" * 1000  # 1000 character name
        long_name_module = Mock()
        long_name_module.name = long_name
        long_name_module.namespace = Mock()
        long_name_module.namespace.symbols = {}
        
        module_value = ModuleValue(long_name_module)
        
        assert module_value.name == long_name
        assert long_name in module_value.to_display_string()
    
    def test_module_namespace_access(self):
        """Test that module namespace is accessible."""
        module_value = ModuleValue(self.mock_module)
        
        # Should be able to access namespace through module
        namespace = module_value.module.namespace
        assert namespace is self.mock_namespace
        assert len(namespace.symbols) == 2
    
    def test_position_propagation_consistency(self):
        """Test that position is consistently propagated."""
        position = SourcePosition(12, 25)
        module_value = ModuleValue(self.mock_module, position)
        
        # All methods that return values should preserve position
        size_result = module_value.universal_size()
        inspect_result = module_value.universal_inspect()
        
        assert module_value.position == position
        assert size_result.position == position
        assert inspect_result.position == position


class TestModuleValueWithRealModule:
    """Test ModuleValue with more realistic module objects."""
    
    def test_with_module_like_object(self):
        """Test with object that behaves like a real module."""
        
        class MockNamespace:
            def __init__(self, symbols_dict):
                self.symbols = symbols_dict
        
        class MockModule:
            def __init__(self, name, symbols):
                self.name = name
                self.namespace = MockNamespace(symbols)
        
        # Create a more realistic module
        symbols = {
            "PI": 3.14159,
            "E": 2.71828,
            "sqrt": "sqrt_function",
            "sin": "sin_function",
            "cos": "cos_function"
        }
        
        math_module = MockModule("math", symbols)
        module_value = ModuleValue(math_module)
        
        assert module_value.name == "math"
        assert module_value.get_type() == "module"
        
        # Check inspect output
        inspect_result = module_value.universal_inspect()
        assert "math" in inspect_result.value
        assert "5 symbols" in inspect_result.value
    
    def test_module_comparison_behavior(self):
        """Test behavior when comparing modules."""
        module1 = Mock()
        module1.name = "module1"
        module1.namespace = Mock()
        module1.namespace.symbols = {"a": 1}
        
        module2 = Mock()
        module2.name = "module2"  
        module2.namespace = Mock()
        module2.namespace.symbols = {"b": 2}
        
        value1 = ModuleValue(module1)
        value2 = ModuleValue(module2)
        
        # They should be different objects
        assert value1.module is not value2.module
        assert value1.name != value2.name
        
        # But same module should produce same wrapper behavior
        value1_duplicate = ModuleValue(module1)
        assert value1_duplicate.module is value1.module
        assert value1_duplicate.name == value1.name


class TestModuleValueNameAccess:
    """Test name access and consistency."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.mock_module = Mock()
        self.mock_module.name = "consistent_module"
        self.mock_module.namespace = Mock()
        self.mock_module.namespace.symbols = {"test": "value"}
    
    def test_name_consistency(self):
        """Test that name is consistent between module and value."""
        module_value = ModuleValue(self.mock_module)
        
        # Name should match module name
        assert module_value.name == self.mock_module.name
        assert module_value.name == "consistent_module"
    
    def test_name_updates_with_module(self):
        """Test behavior when module name changes."""
        module_value = ModuleValue(self.mock_module)
        
        # Initial name
        assert module_value.name == "consistent_module"
        
        # Change module name
        self.mock_module.name = "changed_name"
        
        # ModuleValue name should still be the original (it's set at construction)
        assert module_value.name == "consistent_module"
        
        # But accessing through module should show new name
        assert module_value.module.name == "changed_name"
    
    def test_module_value_independent_of_module_changes(self):
        """Test that ModuleValue is independent of later module changes."""
        module_value = ModuleValue(self.mock_module)
        
        original_display = module_value.to_display_string()
        
        # Change module name
        self.mock_module.name = "different_name"
        
        # Display should still use original name
        assert module_value.to_display_string() == original_display
        assert "consistent_module" in module_value.to_display_string()
        
        # But inspect might reflect current state (it accesses module.namespace)
        inspect_result = module_value.universal_inspect()
        # The inspect method uses self.name, not self.module.name
        assert "consistent_module" in inspect_result.value
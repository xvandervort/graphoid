"""
Tests for AtomicValue functionality (Phase 1 enhancement).

This tests the separation of atomic/scalar values from graph collections.
"""

import pytest
from glang.core.atomic_value import AtomicValue
from glang.repl.repl import REPL
from glang.repl.graph_manager import GraphManager


class TestAtomicValue:
    """Test the AtomicValue class directly."""
    
    def test_string_atomic_value(self):
        """Test string atomic values."""
        atomic = AtomicValue("hello", "string")
        assert atomic.value == "hello"
        assert atomic.atomic_type == "string"
        assert str(atomic) == "'hello'"
        assert atomic.to_string() == "hello"
    
    def test_num_atomic_value(self):
        """Test numeric atomic values."""
        atomic = AtomicValue(42, "num")
        assert atomic.value == 42
        assert atomic.atomic_type == "num"
        assert str(atomic) == "42"
        assert atomic.to_num() == 42
    
    def test_bool_atomic_value(self):
        """Test boolean atomic values."""
        atomic_true = AtomicValue(True, "bool")
        atomic_false = AtomicValue(False, "bool")
        
        assert atomic_true.value is True
        assert str(atomic_true) == "true"
        assert atomic_true.to_bool() is True
        
        assert atomic_false.value is False
        assert str(atomic_false) == "false"
        assert atomic_false.to_bool() is False
    
    def test_type_validation(self):
        """Test that type validation works."""
        # Valid combinations
        AtomicValue("test", "string")  # Should not raise
        AtomicValue(123, "num")       # Should not raise
        AtomicValue(True, "bool")     # Should not raise
        
        # Invalid combinations should raise ValueError
        with pytest.raises(ValueError):
            AtomicValue(123, "string")  # Number with string type
        
        with pytest.raises(ValueError):
            AtomicValue("hello", "num")  # String with num type
        
        with pytest.raises(ValueError):
            AtomicValue("hello", "invalid_type")  # Invalid type
    
    def test_conversion_methods(self):
        """Test conversion between types."""
        # String conversions
        string_val = AtomicValue("123", "string")
        assert string_val.to_num() == 123
        assert string_val.to_bool() is True
        
        # Numeric conversions
        num_val = AtomicValue(0, "num")
        assert num_val.to_string() == "0"
        assert num_val.to_bool() is False
        
        # Boolean conversions
        bool_val = AtomicValue(True, "bool")
        assert bool_val.to_string() == "true"
        assert bool_val.to_num() == 1
    
    def test_equality(self):
        """Test equality comparisons."""
        atomic1 = AtomicValue("hello", "string")
        atomic2 = AtomicValue("hello", "string")
        atomic3 = AtomicValue("world", "string")
        
        assert atomic1 == atomic2
        assert atomic1 != atomic3
        assert atomic1 == "hello"  # Compare with raw value
        assert atomic1 != "world"


class TestAtomicValueREPLIntegration:
    """Test AtomicValue integration with REPL."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.repl = REPL()
        self.graph_manager = GraphManager()
    
    def test_scalar_variable_creation(self):
        """Test creating scalar variables."""
        # Create atomic values using graph manager
        result = self.graph_manager.create_atomic_value("name", "Alice", "string")
        assert "Created string variable 'name'" in result
        
        result = self.graph_manager.create_atomic_value("age", 25, "num")
        assert "Created num variable 'age'" in result
        
        result = self.graph_manager.create_atomic_value("active", True, "bool")
        assert "Created bool variable 'active'" in result
    
    def test_scalar_variable_retrieval(self):
        """Test retrieving atomic values."""
        # Create and retrieve
        self.graph_manager.create_atomic_value("test", "value", "string")
        variable = self.graph_manager.get_variable("test")
        
        assert isinstance(variable, AtomicValue)
        assert variable.value == "value"
        assert variable.atomic_type == "string"
    
    def test_scalar_declaration_via_repl(self):
        """Test scalar declarations through REPL processing."""
        # Test string declaration
        self.repl._process_input('string greeting = "hello"')
        variable = self.repl.graph_manager.get_variable("greeting")
        assert isinstance(variable, AtomicValue)
        assert variable.value == "hello"
        
        # Test num declaration  
        self.repl._process_input("num count = 42")
        variable = self.repl.graph_manager.get_variable("count")
        assert isinstance(variable, AtomicValue)
        assert variable.value == 42
        
        # Test bool declaration
        self.repl._process_input("bool flag = true")
        variable = self.repl.graph_manager.get_variable("flag")
        assert isinstance(variable, AtomicValue)
        assert variable.value is True


class TestAtomicValueMethodRestrictions:
    """Test that atomic values properly restrict graph methods."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.repl = REPL()
        # Create a string atomic value
        self.repl._process_input('string text = "hello"')
    
    def test_append_method_restricted(self):
        """Test that append() is not allowed on atomic values."""
        self.repl._process_input('text.append("world")')
        # Should get an error message about atomic values being immutable
        
    def test_prepend_method_restricted(self):
        """Test that prepend() is not allowed on atomic values."""
        self.repl._process_input('text.prepend("hi")')
        # Should get an error message
    
    def test_reverse_method_restricted(self):
        """Test that reverse() is not allowed on atomic values."""
        self.repl._process_input('text.reverse()')
        # Should get an error message
    
    def test_conversion_methods_allowed(self):
        """Test that conversion methods work on atomic values."""
        # These should work without error
        self.repl._process_input('text.to_string()')
        self.repl._process_input('text.to_bool()')
    
    def test_helpful_error_messages(self):
        """Test that error messages are helpful."""
        # Create a mock to capture output and verify error messages
        import io
        from unittest.mock import patch
        
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('text.append test')
            output = mock_stdout.getvalue()
            assert "Cannot call append() on atomic value" in output
            assert "immutable scalars" in output


class TestAtomicValueDisplay:
    """Test display of atomic values."""
    
    def setup_method(self):
        """Set up test fixtures.""" 
        self.repl = REPL()
    
    def test_string_display(self):
        """Test display of string atomic values."""
        import io
        from unittest.mock import patch
        
        self.repl._process_input('string name = "Alice"')
        
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('name')
            output = mock_stdout.getvalue().strip()
            assert "'Alice'" in output
    
    def test_num_display(self):
        """Test display of numeric atomic values."""
        import io
        from unittest.mock import patch
        
        self.repl._process_input('num age = 25')
        
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('age')
            output = mock_stdout.getvalue().strip()
            assert "25" in output
    
    def test_bool_display(self):
        """Test display of boolean atomic values."""
        import io
        from unittest.mock import patch
        
        self.repl._process_input('bool active = true')
        
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('active')
            output = mock_stdout.getvalue().strip()
            assert "true" in output
    
    def test_info_flag_display(self):
        """Test --info flag display for atomic values."""
        import io
        from unittest.mock import patch
        
        self.repl._process_input('string test = "value"')
        
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('test --info')
            output = mock_stdout.getvalue()
            assert "AtomicValue" in output
            assert "string" in output


class TestGraphVsAtomicSeparation:
    """Test that graphs and atomic values are properly separated."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.repl = REPL()
    
    def test_list_has_graph_methods(self):
        """Test that lists still have graph methods."""
        import io
        from unittest.mock import patch
        
        self.repl._process_input('list items = [1, 2, 3]')
        
        # This should work - lists are graphs
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('items.append(4)')
            output = mock_stdout.getvalue()
            assert "Error" not in output  # Should succeed
    
    def test_string_lacks_graph_methods(self):
        """Test that strings don't have graph methods."""
        import io
        from unittest.mock import patch
        
        self.repl._process_input('string text = "hello"')
        
        # This should fail - strings are atomic
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('text.append world')
            output = mock_stdout.getvalue()
            assert "Cannot call append() on atomic value" in output
    
    def test_different_variable_types_coexist(self):
        """Test that both types can exist in the same namespace."""
        self.repl._process_input('string name = "Alice"')
        self.repl._process_input('list hobbies = ["reading", "coding"]')
        
        # Both should exist with correct types
        name_var = self.repl.graph_manager.get_variable("name")
        hobbies_var = self.repl.graph_manager.get_variable("hobbies")
        
        assert isinstance(name_var, AtomicValue)
        assert name_var.atomic_type == "string"
        
        from glang.core import Graph
        assert isinstance(hobbies_var, Graph)
        assert hobbies_var.graph_type.is_linear()
"""Test starts_with and ends_with string methods."""

import pytest
from glang.execution.values import StringValue, BooleanValue
from glang.execution.executor import ASTExecutor, ExecutionContext
from glang.semantic.symbol_table import SymbolTable

class TestStringStartsEndsWith:
    """Test suite for starts_with and ends_with string methods."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.symbol_table = SymbolTable()
        self.context = ExecutionContext(self.symbol_table)
        self.executor = ASTExecutor(self.context)
    
    def execute_string_method(self, string_value, method_name, *args):
        """Helper to execute string methods directly."""
        string_val = StringValue(string_value)
        arg_values = [StringValue(str(arg)) for arg in args]
        return self.executor._dispatch_string_method(string_val, method_name, arg_values, None)
    
    def test_starts_with_basic(self):
        """Test basic starts_with functionality."""
        result = self.execute_string_method("Hello World", "starts_with", "Hello")
        assert isinstance(result, BooleanValue)
        assert result.value is True
        
        result = self.execute_string_method("Hello World", "starts_with", "World")
        assert result.value is False
        
        result = self.execute_string_method("Hello World", "starts_with", "Hi")
        assert result.value is False
    
    def test_ends_with_basic(self):
        """Test basic ends_with functionality."""
        result = self.execute_string_method("Hello World", "ends_with", "World")
        assert isinstance(result, BooleanValue)
        assert result.value is True
        
        result = self.execute_string_method("Hello World", "ends_with", "Hello")
        assert result.value is False
        
        result = self.execute_string_method("Hello World", "ends_with", "ld")
        assert result.value is True
    
    def test_starts_with_empty_string(self):
        """Test starts_with with empty string."""
        result = self.execute_string_method("Hello", "starts_with", "")
        assert result.value is True  # Everything starts with empty string
        
        result = self.execute_string_method("", "starts_with", "Hello")
        assert result.value is False
        
        result = self.execute_string_method("", "starts_with", "")
        assert result.value is True
    
    def test_ends_with_empty_string(self):
        """Test ends_with with empty string."""
        result = self.execute_string_method("Hello", "ends_with", "")
        assert result.value is True  # Everything ends with empty string
        
        result = self.execute_string_method("", "ends_with", "Hello")
        assert result.value is False
        
        result = self.execute_string_method("", "ends_with", "")
        assert result.value is True
    
    def test_starts_with_same_string(self):
        """Test starts_with when string equals prefix."""
        result = self.execute_string_method("Hello", "starts_with", "Hello")
        assert result.value is True
    
    def test_ends_with_same_string(self):
        """Test ends_with when string equals suffix."""
        result = self.execute_string_method("Hello", "ends_with", "Hello")
        assert result.value is True
    
    def test_starts_with_longer_prefix(self):
        """Test starts_with when prefix is longer than string."""
        result = self.execute_string_method("Hi", "starts_with", "Hello")
        assert result.value is False
    
    def test_ends_with_longer_suffix(self):
        """Test ends_with when suffix is longer than string."""
        result = self.execute_string_method("Hi", "ends_with", "Hello")
        assert result.value is False
    
    def test_starts_with_case_sensitive(self):
        """Test that starts_with is case sensitive."""
        result = self.execute_string_method("Hello World", "starts_with", "hello")
        assert result.value is False
        
        result = self.execute_string_method("Hello World", "starts_with", "HELLO")
        assert result.value is False
    
    def test_ends_with_case_sensitive(self):
        """Test that ends_with is case sensitive."""
        result = self.execute_string_method("Hello World", "ends_with", "world")
        assert result.value is False
        
        result = self.execute_string_method("Hello World", "ends_with", "WORLD")
        assert result.value is False
    
    def test_starts_with_special_characters(self):
        """Test starts_with with special characters."""
        result = self.execute_string_method("@#$Hello", "starts_with", "@#$")
        assert result.value is True
        
        result = self.execute_string_method("@#$Hello", "starts_with", "@#")
        assert result.value is True
    
    def test_ends_with_special_characters(self):
        """Test ends_with with special characters."""
        result = self.execute_string_method("Hello!?.", "ends_with", "!?.")
        assert result.value is True
        
        result = self.execute_string_method("Hello!?.", "ends_with", "?.")
        assert result.value is True
    
    def test_starts_with_with_spaces(self):
        """Test starts_with with whitespace."""
        result = self.execute_string_method("  Hello World", "starts_with", "  ")
        assert result.value is True
        
        result = self.execute_string_method("  Hello World", "starts_with", "Hello")
        assert result.value is False
    
    def test_ends_with_with_spaces(self):
        """Test ends_with with whitespace."""
        result = self.execute_string_method("Hello World  ", "ends_with", "  ")
        assert result.value is True
        
        result = self.execute_string_method("Hello World  ", "ends_with", "World")
        assert result.value is False
    
    def test_starts_with_wrong_type_error(self):
        """Test starts_with with non-string argument raises error."""
        # This test needs to pass a non-string value, which our helper doesn't support
        # So we'll test it manually
        string_val = StringValue("Hello")
        with pytest.raises(Exception) as exc_info:
            from glang.execution.values import NumberValue
            self.executor._dispatch_string_method(string_val, "starts_with", [NumberValue(123)], None)
        assert "must be string" in str(exc_info.value)
    
    def test_ends_with_wrong_type_error(self):
        """Test ends_with with non-string argument raises error."""
        string_val = StringValue("Hello")
        with pytest.raises(Exception) as exc_info:
            from glang.execution.values import NumberValue
            self.executor._dispatch_string_method(string_val, "ends_with", [NumberValue(123)], None)
        assert "must be string" in str(exc_info.value)
    
    def test_starts_with_no_args_error(self):
        """Test starts_with with no arguments raises error."""
        string_val = StringValue("Hello")
        with pytest.raises(Exception) as exc_info:
            self.executor._dispatch_string_method(string_val, "starts_with", [], None)
        assert "takes 1 argument" in str(exc_info.value)
    
    def test_ends_with_no_args_error(self):
        """Test ends_with with no arguments raises error."""
        string_val = StringValue("Hello")
        with pytest.raises(Exception) as exc_info:
            self.executor._dispatch_string_method(string_val, "ends_with", [], None)
        assert "takes 1 argument" in str(exc_info.value)
    
    def test_starts_with_too_many_args_error(self):
        """Test starts_with with too many arguments raises error."""
        string_val = StringValue("Hello")
        with pytest.raises(Exception) as exc_info:
            self.executor._dispatch_string_method(string_val, "starts_with", 
                                                 [StringValue("He"), StringValue("llo")], None)
        assert "takes 1 argument" in str(exc_info.value)
    
    def test_ends_with_too_many_args_error(self):
        """Test ends_with with too many arguments raises error."""
        string_val = StringValue("Hello")
        with pytest.raises(Exception) as exc_info:
            self.executor._dispatch_string_method(string_val, "ends_with", 
                                                 [StringValue("lo"), StringValue("llo")], None)
        assert "takes 1 argument" in str(exc_info.value)
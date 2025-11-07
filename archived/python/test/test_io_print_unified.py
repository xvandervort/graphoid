"""Tests for the unified I/O print function with optional newline flag."""

import pytest
from glang.execution.pipeline import ExecutionSession


class TestIOPrintUnified:
    """Test the unified I/O print function."""
    
    def setup_method(self):
        """Setup for each test."""
        self.session = ExecutionSession()
        # Import I/O module
        result = self.session.execute_statement('import "io"')
        assert result.success, f"Failed to import I/O module: {result}"
    
    def test_print_default_newline(self):
        """Test print with default newline (true)."""
        result = self.session.execute_statement('io.print("Hello")')
        assert result.success
        # Should return void/none
        assert result.value is None or result.value.get_type() == 'none'
    
    def test_print_without_newline(self):
        """Test print without newline (false)."""
        result = self.session.execute_statement('io.print("Test", false)')
        assert result.success
        # Should return void/none
        assert result.value is None or result.value.get_type() == 'none'
    
    def test_print_with_explicit_newline(self):
        """Test print with explicit newline (true)."""
        result = self.session.execute_statement('io.print("Test", true)')
        assert result.success
        # Should return void/none
        assert result.value is None or result.value.get_type() == 'none'
    
    def test_print_with_numbers(self):
        """Test print with non-string values."""
        result = self.session.execute_statement('io.print(42)')
        assert result.success
        
        result = self.session.execute_statement('io.print(3.14)')
        assert result.success
        
        result = self.session.execute_statement('io.print(true)')
        assert result.success
    
    def test_print_with_complex_types(self):
        """Test print with lists and data structures."""
        self.session.execute_statement('numbers = [1, 2, 3]')
        result = self.session.execute_statement('io.print(numbers)')
        assert result.success
        
        self.session.execute_statement('config = {"host": "localhost"}')
        result = self.session.execute_statement('io.print(config)')
        assert result.success
    
    def test_print_type_inference(self):
        """Test that print calls are properly type-inferred as void."""
        # This should be valid - print returns void, can't be assigned
        result = self.session.execute_statement('io.print("test")')
        assert result.success
        
        # The result should be void/none type
        assert result.value is None or result.value.get_type() == 'none'
    
    def test_print_error_handling(self):
        """Test error handling for invalid newline parameter."""
        # Invalid newline parameter type
        result = self.session.execute_statement('io.print("test", "invalid")')
        assert not result.success
        assert "expects bool" in str(result.error)
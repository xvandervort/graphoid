"""
Tests for print statement functionality.

This module tests the print statement implementation including parsing, semantic
analysis, and execution.
"""

import pytest
import sys
import os
from io import StringIO
from contextlib import redirect_stdout

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from glang.execution.pipeline import ExecutionSession


class TestPrintStatement:
    """Test print statement functionality."""
    
    def test_print_single_string(self):
        """Test printing a single string variable."""
        session = ExecutionSession()
        
        # Declare a string variable
        result = session.execute_statement('string message = "Hello World"')
        assert result.success
        
        # Capture print output with parentheses
        output = StringIO()
        with redirect_stdout(output):
            result = session.execute_statement('print(message)')
        
        assert result.success
        assert output.getvalue().strip() == "Hello World"
        
        # Test without parentheses (Ruby-style)
        output = StringIO()
        with redirect_stdout(output):
            result = session.execute_statement('print message')
        
        assert result.success
        assert output.getvalue().strip() == "Hello World"
    
    def test_print_multiple_values(self):
        """Test printing multiple values separated by spaces."""
        session = ExecutionSession()
        
        # Declare variables
        result = session.execute_statement('string name = "Alice"')
        assert result.success
        result = session.execute_statement('num age = 25')
        assert result.success
        result = session.execute_statement('bool active = true')
        assert result.success
        
        # Capture print output
        output = StringIO()
        with redirect_stdout(output):
            result = session.execute_statement('print(name, age, active)')
        
        assert result.success
        assert output.getvalue().strip() == "Alice 25 true"
    
    def test_print_literal_values(self):
        """Test printing literal values directly."""
        session = ExecutionSession()
        
        # Capture print output
        output = StringIO()
        with redirect_stdout(output):
            result = session.execute_statement('print("Direct string", 42, false)')
        
        assert result.success
        assert output.getvalue().strip() == "Direct string 42 false"
    
    def test_print_list_values(self):
        """Test printing list values."""
        session = ExecutionSession()
        
        # Declare a list
        result = session.execute_statement('list<num> numbers = [1, 2, 3]')
        assert result.success
        
        # Capture print output
        output = StringIO()
        with redirect_stdout(output):
            result = session.execute_statement('print(numbers)')
        
        assert result.success
        assert output.getvalue().strip() == "[1, 2, 3]"
    
    def test_print_empty_parentheses(self):
        """Test printing with no arguments (empty line)."""
        session = ExecutionSession()
        
        # Capture print output with parentheses
        output = StringIO()
        with redirect_stdout(output):
            result = session.execute_statement('print()')
        
        assert result.success
        assert output.getvalue() == "\n"
        
        # Test without parentheses 
        output = StringIO()
        with redirect_stdout(output):
            result = session.execute_statement('print')
        
        assert result.success
        assert output.getvalue() == "\n"
    
    def test_print_with_method_call_results(self):
        """Test printing results of method calls."""
        session = ExecutionSession()
        
        # Declare a list and perform operations
        result = session.execute_statement('list<num> numbers = [5, 1, 9, 3, 7]')
        assert result.success
        
        # Capture print output for method results
        output = StringIO()
        with redirect_stdout(output):
            result = session.execute_statement('print("Min:", numbers.min(), "Max:", numbers.max())')
        
        assert result.success
        assert output.getvalue().strip() == "Min: 1 Max: 9"
    
    def test_print_flexible_syntax(self):
        """Test print with flexible parentheses (Ruby-style)."""
        session = ExecutionSession()
        
        # Declare some variables
        result = session.execute_statement('string name = "Alice"')
        assert result.success
        result = session.execute_statement('num age = 25')
        assert result.success
        
        # Test various flexible syntaxes
        output = StringIO()
        with redirect_stdout(output):
            result = session.execute_statement('print name, "is", age, "years old"')
        assert result.success
        assert output.getvalue().strip() == "Alice is 25 years old"
        
        # Mix of parentheses and no parentheses should work
        output = StringIO()
        with redirect_stdout(output):
            result = session.execute_statement('print "Age:", age')
        assert result.success
        assert output.getvalue().strip() == "Age: 25"
    
    def test_print_semantic_analysis(self):
        """Test that print statements pass semantic analysis."""
        session = ExecutionSession()
        
        # This should pass semantic analysis without errors (with parens)
        result = session.execute_statement('print("Hello", 42)')
        assert result.success
        
        # This should pass semantic analysis without errors (without parens)
        result = session.execute_statement('print "Hello", 42')
        assert result.success
        
        # Test with undefined variable should fail
        result = session.execute_statement('print(undefined_variable)')
        assert not result.success
        assert "undefined" in str(result.error).lower()
        
        # Test with undefined variable should fail (no parens)
        result = session.execute_statement('print undefined_variable')
        assert not result.success
        assert "undefined" in str(result.error).lower()


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
#!/usr/bin/env python3

"""
Test suite for lambda functions in map() and filter() operations
"""

import pytest
import sys
import os

# Add the src directory to the path so we can import glang modules
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from glang.parser import ASTParser
from glang.semantic import SemanticAnalyzer
from glang.execution import ASTExecutor, ExecutionContext
from glang.semantic.symbol_table import SymbolTable

class TestLambdaOperations:
    """Test lambda functions can be passed to map() and filter() operations"""

    def setup_method(self):
        """Set up test environment"""
        self.parser = ASTParser()
        self.analyzer = SemanticAnalyzer()
        self.symbol_table = self.analyzer.symbol_table
        self.context = ExecutionContext(self.symbol_table)
        self.executor = ASTExecutor(self.context)

    def execute_code(self, code: str):
        """Parse, analyze and execute code."""
        ast = self.parser.parse(code)
        self.analyzer.analyze(ast)
        return self.executor.execute(ast)

    def test_map_with_lambda_function(self):
        """Test map() accepts custom lambda functions"""
        code = '''
        numbers = [1, 2, 3, 4, 5]
        triple = x => x * 3
        result = numbers.map(triple)
        '''

        self.execute_code(code)

        result = self.context.get_variable('result')
        assert result is not None

        # Should be [3, 6, 9, 12, 15]
        elements = [elem.value for elem in result.elements]
        assert elements == [3, 6, 9, 12, 15]

    def test_map_with_string_still_works(self):
        """Test that built-in string transformations still work"""
        code = '''
        numbers = [1, 2, 3, 4, 5]
        result = numbers.map("double")
        '''

        self.execute_code(code)

        result = self.context.get_variable('result')
        assert result is not None

        # Should be [2, 4, 6, 8, 10]
        elements = [elem.value for elem in result.elements]
        assert elements == [2, 4, 6, 8, 10]

    def test_filter_with_lambda_function(self):
        """Test filter() accepts custom lambda functions"""
        code = '''
        numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        is_even = x => (x % 2) == 0
        result = numbers.filter(is_even)
        '''

        self.execute_code(code)

        result = self.context.get_variable('result')
        assert result is not None

        # Should be [2, 4, 6, 8, 10]
        elements = [elem.value for elem in result.elements]
        assert elements == [2, 4, 6, 8, 10]

    def test_filter_with_string_still_works(self):
        """Test that built-in string predicates still work"""
        code = '''
        numbers = [1, 2, 3, 4, 5, 6]
        result = numbers.filter("even")
        '''

        self.execute_code(code)

        result = self.context.get_variable('result')
        assert result is not None

        # Should be [2, 4, 6]
        elements = [elem.value for elem in result.elements]
        assert elements == [2, 4, 6]

    def test_complex_lambda_operations(self):
        """Test complex lambda expressions"""
        code = '''
        numbers = [1, 2, 3, 4, 5]

        # Complex transformation
        square_plus_one = x => (x * x) + 1
        transformed = numbers.map(square_plus_one)

        # Complex filtering
        greater_than_ten = x => x > 10
        filtered = transformed.filter(greater_than_ten)
        '''

        self.execute_code(code)

        transformed = self.context.get_variable('transformed')
        filtered = self.context.get_variable('filtered')

        # transformed should be [2, 5, 10, 17, 26] (x^2 + 1)
        transformed_elements = [elem.value for elem in transformed.elements]
        assert transformed_elements == [2, 5, 10, 17, 26]

        # filtered should be [17, 26] (values > 10)
        filtered_elements = [elem.value for elem in filtered.elements]
        assert filtered_elements == [17, 26]

    def test_chained_lambda_operations(self):
        """Test chaining map and filter with lambdas"""
        code = '''
        numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

        # Chain operations: filter evens, then double them
        is_even = x => (x % 2) == 0
        double = x => x * 2

        result = numbers.filter(is_even).map(double)
        '''

        self.execute_code(code)

        result = self.context.get_variable('result')
        assert result is not None

        # Should filter [2,4,6,8,10] then double to [4,8,12,16,20]
        elements = [elem.value for elem in result.elements]
        assert elements == [4, 8, 12, 16, 20]

    def test_lambda_with_string_operations(self):
        """Test lambdas that work with string operations"""
        code = '''
        words = ["hello", "world", "glang", "lambda"]

        # Filter words longer than 4 characters
        long_words = word => word.size() > 4
        result = words.filter(long_words)
        '''

        self.execute_code(code)

        result = self.context.get_variable('result')
        assert result is not None

        # Should be ["hello", "world", "glang", "lambda"] - all > 4 chars
        elements = [elem.value for elem in result.elements]
        assert elements == ["hello", "world", "glang", "lambda"]

    def test_lambda_error_handling(self):
        """Test error cases for lambda operations"""
        code = '''
        numbers = [1, 2, 3]

        # Lambda with wrong number of parameters
        try_invalid = (x, y) => x + y
        '''

        self.execute_code(code)

        # Test that map with wrong parameter count fails
        with pytest.raises(Exception) as exc_info:
            error_code = '''
            numbers = [1, 2, 3]
            invalid_lambda = (x, y) => x + y
            result = numbers.map(invalid_lambda)
            '''
            self.execute_code(error_code)

        assert "must have exactly 1 parameter" in str(exc_info.value)

if __name__ == '__main__':
    pytest.main([__file__, '-v'])
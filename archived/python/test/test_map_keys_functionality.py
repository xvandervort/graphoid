#!/usr/bin/env python3

"""
Core test for map.keys() functionality that enables DataFrame group_by
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

class TestMapKeysForGroupBy:
    """Test that map.keys() method works - the foundation for DataFrame group_by"""

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

    def test_map_has_keys_method(self):
        """Test that maps have the keys() method - core requirement for group_by"""
        code = '''
        data = { "name": "Alice", "age": 30, "dept": "Engineering" }
        keys = data.keys()
        keys_string = keys.to_string()
        key_count = keys.size()
        '''

        self.execute_code(code)

        keys_var = self.context.get_variable('keys')
        keys_string = self.context.get_variable('keys_string')
        key_count = self.context.get_variable('key_count')

        assert keys_var is not None
        assert key_count.value == 3

        # Keys should contain the expected keys
        keys_str_value = keys_string.value
        assert 'name' in keys_str_value
        assert 'age' in keys_str_value
        assert 'dept' in keys_str_value

    def test_keys_can_be_indexed(self):
        """Test that keys can be accessed by index - needed for iteration"""
        code = '''
        groups = { "Engineering": 82500, "Sales": 62500 }
        dept_keys = groups.keys()
        first_dept = dept_keys[0]
        second_dept = dept_keys[1]
        total_depts = dept_keys.size()
        '''

        self.execute_code(code)

        first_dept = self.context.get_variable('first_dept')
        second_dept = self.context.get_variable('second_dept')
        total_depts = self.context.get_variable('total_depts')

        assert first_dept is not None
        assert second_dept is not None
        assert total_depts.value == 2

        # Keys should be the department names
        first_value = first_dept.value
        second_value = second_dept.value

        departments = {first_value, second_value}
        assert departments == {'Engineering', 'Sales'}

    def test_keys_enable_dynamic_access(self):
        """Test using keys to dynamically access map values - core group_by pattern"""
        code = '''
        results = { "Engineering": 82500, "Sales": 62500, "Marketing": 70000 }
        dept_keys = results.keys()

        # Access first department dynamically
        first_key = dept_keys[0]
        first_value = results[first_key]

        # Count total departments
        dept_count = dept_keys.size()
        '''

        self.execute_code(code)

        first_key = self.context.get_variable('first_key')
        first_value = self.context.get_variable('first_value')
        dept_count = self.context.get_variable('dept_count')

        assert first_key is not None
        assert first_value is not None
        assert dept_count.value == 3

        # The value should correspond to the key
        key_name = first_key.value
        expected_values = {"Engineering": 82500, "Sales": 62500, "Marketing": 70000}
        assert first_value.value == expected_values[key_name]

if __name__ == '__main__':
    pytest.main([__file__])
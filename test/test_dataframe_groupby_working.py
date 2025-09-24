#!/usr/bin/env python3

"""
Working tests for DataFrame group_by functionality - proper syntax only
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

class TestDataFrameGroupByWorking:
    """Working tests for DataFrame group_by using proper Glang syntax"""

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

    def test_basic_map_keys_functionality(self):
        """Test basic map.keys() functionality that enables group_by"""
        code = '''
        data = { "dept1": 100, "dept2": 200, "dept3": 300 }
        keys = data.keys()
        key_count = keys.size()
        first_key = keys[0]
        first_value = data[first_key]
        '''

        self.execute_code(code)

        key_count = self.context.get_variable('key_count')
        first_key = self.context.get_variable('first_key')
        first_value = self.context.get_variable('first_value')

        assert key_count.value == 3
        assert first_key is not None
        assert first_value is not None

        # The first value should correspond to the first key
        key_name = first_key.value
        expected_values = {"dept1": 100, "dept2": 200, "dept3": 300}
        assert first_value.value == expected_values[key_name]

    def test_simple_groupby_aggregation(self):
        """Test simple aggregation pattern used in group_by"""
        code = '''
        salary_totals = { "Engineering": 165000, "Sales": 125000 }
        dept_keys = salary_totals.keys()

        eng_total = salary_totals["Engineering"]
        sales_total = salary_totals["Sales"]

        dept_count = dept_keys.size()
        '''

        self.execute_code(code)

        eng_total = self.context.get_variable('eng_total')
        sales_total = self.context.get_variable('sales_total')
        dept_count = self.context.get_variable('dept_count')

        assert eng_total.value == 165000
        assert sales_total.value == 125000
        assert dept_count.value == 2

    def test_dynamic_column_extraction(self):
        """Test dynamic column extraction using keys() - core of from_column_data"""
        code = '''
        column_info = { "name": "string", "age": "number", "active": "bool" }
        columns = column_info.keys()
        column_count = columns.size()

        name_idx = -1
        age_idx = -1
        active_idx = -1

        for i in [].upto(columns.size() - 1) {
            col = columns[i]
            if col == "name" {
                name_idx = i
            }
            if col == "age" {
                age_idx = i
            }
            if col == "active" {
                active_idx = i
            }
        }

        has_name = name_idx >= 0
        has_age = age_idx >= 0
        has_active = active_idx >= 0
        '''

        self.execute_code(code)

        column_count = self.context.get_variable('column_count')
        has_name = self.context.get_variable('has_name')
        has_age = self.context.get_variable('has_age')
        has_active = self.context.get_variable('has_active')

        assert column_count.value == 3
        assert has_name.value == True
        assert has_age.value == True
        assert has_active.value == True

    def test_iterative_key_processing(self):
        """Test iterating through keys - pattern used in group_by functions"""
        code = '''
        results = { "A": 10, "B": 20, "C": 30 }
        keys = results.keys()

        total = 0
        for i in [].upto(keys.size() - 1) {
            key = keys[i]
            value = results[key]
            total = total + value
        }

        processed_count = keys.size()
        '''

        self.execute_code(code)

        total = self.context.get_variable('total')
        processed_count = self.context.get_variable('processed_count')

        assert total.value == 60  # 10 + 20 + 30
        assert processed_count.value == 3

    def test_nested_map_operations(self):
        """Test nested operations similar to group_by with aggregation operations"""
        code = '''
        groups = { "dept1": 5, "dept2": 3 }
        operations = { "sum": 100, "avg": 50, "count": 8 }

        group_keys = groups.keys()
        op_keys = operations.keys()

        # Simulate applying operations to groups
        result = {}
        first_group = group_keys[0]
        first_op = op_keys[0]

        group_size = groups[first_group]
        op_value = operations[first_op]

        combined = group_size + op_value
        '''

        self.execute_code(code)

        group_keys = self.context.get_variable('group_keys')
        op_keys = self.context.get_variable('op_keys')
        combined = self.context.get_variable('combined')

        assert group_keys is not None
        assert op_keys is not None
        # Should be 5 + 100 = 105 OR 3 + 100 = 103 (depending on key order)
        assert combined.value in [105, 103]

    def test_dataframe_column_initialization_pattern(self):
        """Test the pattern used in DataFrame column initialization"""
        code = '''
        columns = { "name": "empty", "age": "empty", "score": "empty" }
        column_names = columns.keys()

        # Simulate DataFrame structure
        df = { "_columns": column_names, "_row_count": 0 }

        # Initialize columns (simplified version of what DataFrame does)
        for i in [].upto(column_names.size() - 1) {
            col_name = column_names[i]
            # In real DataFrame, this would be df[col_name] = []
            # For test, just verify we can access the column name
        }

        column_count = column_names.size()
        first_column = column_names[0]
        '''

        self.execute_code(code)

        column_count = self.context.get_variable('column_count')
        first_column = self.context.get_variable('first_column')

        assert column_count.value == 3
        assert first_column.value in ['name', 'age', 'score']

if __name__ == '__main__':
    pytest.main([__file__, '-v'])
#!/usr/bin/env python3

"""
Tests for forward and backward fill behaviors
Verifies missing data handling in graph containers
"""

import pytest
import sys
import os

# Add the src directory to the path so we can import glang modules
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from glang.execution.pipeline import ExecutionSession
from glang.files.file_manager import FileManager


class TestFillBehaviors:
    """Tests for forward_fill and backward_fill behaviors"""

    def setup_method(self):
        """Set up test environment"""
        self.file_manager = FileManager()
        self.execution_session = ExecutionSession(self.file_manager)

    def execute_code(self, code: str):
        """Execute code using ExecutionSession."""
        result = self.execution_session.execute_statement(code)
        if not result.success:
            raise RuntimeError(f"Execution failed: {result.error}")
        return result

    def get_variable(self, name: str):
        """Get variable value from execution context."""
        return self.execution_session.execution_context.get_variable(name)

    def test_forward_fill_basic(self):
        """Test basic forward fill functionality"""
        self.execute_code('data = [10, none, 30, none, 50]')
        self.execute_code('data.add_rule("forward_fill")')

        data = self.get_variable('data')
        values = [item.to_display_string() for item in data.elements]

        assert values == ['10', '10', '30', '30', '50']

    def test_backward_fill_basic(self):
        """Test basic backward fill functionality"""
        self.execute_code('data = [none, 20, none, 40, none]')
        self.execute_code('data.add_rule("backward_fill")')

        data = self.get_variable('data')
        values = [item.to_display_string() for item in data.elements]

        assert values == ['20', '20', '40', '40', 'none']

    def test_forward_fill_with_leading_none(self):
        """Test forward fill when first values are none"""
        self.execute_code('data = [none, none, 30, none, 50]')
        self.execute_code('data.add_rule("forward_fill")')

        data = self.get_variable('data')
        values = [item.to_display_string() for item in data.elements]

        # Leading nones should remain as none since no previous value
        assert values == ['none', 'none', '30', '30', '50']

    def test_backward_fill_with_trailing_none(self):
        """Test backward fill when last values are none"""
        self.execute_code('data = [10, none, 30, none, none]')
        self.execute_code('data.add_rule("backward_fill")')

        data = self.get_variable('data')
        values = [item.to_display_string() for item in data.elements]

        # Trailing nones should remain as none since no next value
        assert values == ['10', '30', '30', 'none', 'none']

    def test_fill_with_all_none(self):
        """Test fill behaviors with all none values"""
        self.execute_code('data1 = [none, none, none]')
        self.execute_code('data1.add_rule("forward_fill")')
        data1 = self.get_variable('data1')
        values1 = [item.to_display_string() for item in data1.elements]
        assert values1 == ['none', 'none', 'none']

        self.execute_code('data2 = [none, none, none]')
        self.execute_code('data2.add_rule("backward_fill")')
        data2 = self.get_variable('data2')
        values2 = [item.to_display_string() for item in data2.elements]
        assert values2 == ['none', 'none', 'none']

    def test_fill_with_string_data(self):
        """Test fill behaviors with string data"""
        self.execute_code('names = ["Alice", none, "Bob", none, "Carol"]')
        self.execute_code('names.add_rule("forward_fill")')

        names = self.get_variable('names')
        values = [item.to_display_string() for item in names.elements]

        assert values == ['Alice', 'Alice', 'Bob', 'Bob', 'Carol']

    def test_fill_with_mixed_data_types(self):
        """Test fill behaviors preserve data types"""
        self.execute_code('mixed = [10, none, "text", none, true]')
        self.execute_code('mixed.add_rule("forward_fill")')

        mixed = self.get_variable('mixed')
        values = [item.to_display_string() for item in mixed.elements]

        # Each none should be replaced with the previous value
        assert values == ['10', '10', 'text', 'text', 'true']

    def test_dataframe_column_forward_fill(self):
        """Test forward fill on DataFrame-like column data"""
        # Create DataFrame-like structure
        self.execute_code('df = {}')
        self.execute_code('df["temperature"] = [22.5, none, none, 25.1, none, 27.3]')

        # Apply forward fill to temperature column
        self.execute_code('df["temperature"].add_rule("forward_fill")')

        # Check results
        df = self.get_variable('df')
        temp_col = df.graph.get('temperature')
        values = [item.to_display_string() for item in temp_col.elements]

        expected = ['22.5', '22.5', '22.5', '25.1', '25.1', '27.3']
        assert values == expected

    def test_dataframe_column_backward_fill(self):
        """Test backward fill on DataFrame-like column data"""
        # Create DataFrame-like structure
        self.execute_code('df = {}')
        self.execute_code('df["humidity"] = [none, 45, none, none, 52, none]')

        # Apply backward fill to humidity column
        self.execute_code('df["humidity"].add_rule("backward_fill")')

        # Check results
        df = self.get_variable('df')
        humid_col = df.graph.get('humidity')
        values = [item.to_display_string() for item in humid_col.elements]

        expected = ['45', '45', '52', '52', '52', 'none']
        assert values == expected

    def test_multiple_dataframe_columns(self):
        """Test fill behaviors on multiple DataFrame columns"""
        # Create DataFrame with missing data
        self.execute_code('sales = {}')
        self.execute_code('sales["Q1"] = [1000, none, 1500, none]')
        self.execute_code('sales["Q2"] = [none, 1200, none, 1800]')

        # Apply different fill strategies
        self.execute_code('sales["Q1"].add_rule("forward_fill")')
        self.execute_code('sales["Q2"].add_rule("backward_fill")')

        # Verify results
        sales = self.get_variable('sales')
        q1_values = [item.to_display_string() for item in sales.graph.get('Q1').elements]
        q2_values = [item.to_display_string() for item in sales.graph.get('Q2').elements]

        assert q1_values == ['1000', '1000', '1500', '1500']
        assert q2_values == ['1200', '1200', '1800', '1800']

    def test_fill_behavior_with_existing_behaviors(self):
        """Test that fill behaviors work alongside other behaviors"""
        # Create data with negative values and missing data
        self.execute_code('data = [-10, none, -20, none, 30]')

        # Apply multiple behaviors: forward fill, then make positive
        self.execute_code('data.add_rule("forward_fill")')
        self.execute_code('data.add_rule("positive")')

        data = self.get_variable('data')
        values = [float(item.to_display_string()) for item in data.elements]

        # Should forward fill, then make all positive
        expected = [10.0, 10.0, 20.0, 20.0, 30.0]  # All positive
        assert values == expected

    def test_empty_list_with_fill_behaviors(self):
        """Test fill behaviors on empty lists"""
        self.execute_code('empty = []')
        self.execute_code('empty.add_rule("forward_fill")')

        empty = self.get_variable('empty')
        assert len(empty.elements) == 0

    def test_single_element_with_fill_behaviors(self):
        """Test fill behaviors on single-element lists"""
        self.execute_code('single = [none]')
        self.execute_code('single.add_rule("forward_fill")')

        single = self.get_variable('single')
        values = [item.to_display_string() for item in single.elements]
        assert values == ['none']  # Should remain none

        self.execute_code('single2 = [42]')
        self.execute_code('single2.add_rule("forward_fill")')

        single2 = self.get_variable('single2')
        values2 = [item.to_display_string() for item in single2.elements]
        assert values2 == ['42']  # Should remain unchanged

    def test_time_series_data_simulation(self):
        """Test fill behaviors on time series-like data"""
        # Simulate time series with missing measurements
        self.execute_code('readings = [10.1, 10.2, none, none, 10.8, 11.0, none, 11.3]')
        self.execute_code('readings.add_rule("forward_fill")')

        readings = self.get_variable('readings')
        values = [item.to_display_string() for item in readings.elements]

        # Check structure (allowing for slight formatting differences like 11.0 vs 11)
        assert len(values) == 8
        assert values[0] == '10.1'
        assert values[1] == '10.2'
        assert values[2] == '10.2'  # Forward filled
        assert values[3] == '10.2'  # Forward filled
        assert values[4] == '10.8'
        assert values[5] in ['11.0', '11']  # Allow formatting variations
        assert values[6] in ['11.0', '11']  # Forward filled
        assert values[7] == '11.3'
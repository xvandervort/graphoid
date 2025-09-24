#!/usr/bin/env python3

"""
Tests for the statistics module (stdlib/statistics.gr)
Comprehensive testing of all statistical functions
"""

import pytest
import sys
import os

# Add the src directory to the path so we can import glang modules
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from glang.execution.pipeline import ExecutionSession
from glang.files.file_manager import FileManager


class TestStatisticsModule:
    """Tests for statistics module functions"""

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

    def test_load_statistics_module(self):
        """Test that statistics module loads without errors"""
        self.execute_code('load "stdlib/statistics.gr"')
        # If we get here without exception, the module loaded successfully
        assert True

    def test_mean_function_basic(self):
        """Test mean() function with basic data"""
        self.execute_code('load "stdlib/statistics.gr"')
        self.execute_code('test_data = [1, 2, 3, 4, 5]')
        self.execute_code('result = mean(test_data)')

        result = self.get_variable('result')
        assert result.value == 3.0  # (1+2+3+4+5)/5 = 3

    def test_mean_function_empty_list(self):
        """Test mean() function with empty list"""
        self.execute_code('load "stdlib/statistics.gr"')
        self.execute_code('empty_data = []')
        self.execute_code('result = mean(empty_data)')

        result = self.get_variable('result')
        assert result.is_none()

    def test_mean_function_single_value(self):
        """Test mean() function with single value"""
        self.execute_code('load "stdlib/statistics.gr"')
        self.execute_code('single_data = [42]')
        self.execute_code('result = mean(single_data)')

        result = self.get_variable('result')
        assert result.value == 42

    def test_variance_function_basic(self):
        """Test variance() function with known values"""
        self.execute_code('load "stdlib/statistics.gr"')
        self.execute_code('test_data = [2, 4, 4, 4, 5, 5, 7, 9]')  # Known sample variance = 4.57
        self.execute_code('result = variance(test_data)')

        result = self.get_variable('result')
        # Sample variance calculation: sum((x - mean)²) / (n-1)
        assert abs(result.value - 4.571428571428571) < 0.1  # Allow small floating point errors

    def test_variance_function_edge_cases(self):
        """Test variance() function with edge cases"""
        self.execute_code('load "stdlib/statistics.gr"')

        # Single value should return 0
        self.execute_code('single_data = [5]')
        self.execute_code('result_single = variance(single_data)')
        result_single = self.get_variable('result_single')
        assert result_single.value == 0

        # Empty list should return 0
        self.execute_code('empty_data = []')
        self.execute_code('result_empty = variance(empty_data)')
        result_empty = self.get_variable('result_empty')
        assert result_empty.value == 0

    def test_std_function_basic(self):
        """Test std() function (standard deviation)"""
        self.execute_code('load "stdlib/statistics.gr"')
        self.execute_code('test_data = [1, 2, 3, 4, 5]')
        self.execute_code('result = std(test_data)')

        result = self.get_variable('result')
        # Standard deviation of [1,2,3,4,5] ≈ 1.58
        assert abs(result.value - 1.58) < 0.1

    def test_std_function_zero_variance(self):
        """Test std() function when all values are the same"""
        self.execute_code('load "stdlib/statistics.gr"')
        self.execute_code('same_data = [5, 5, 5, 5]')
        self.execute_code('result = std(same_data)')

        result = self.get_variable('result')
        assert result.value == 0  # No variation = std = 0

    def test_population_variance_vs_sample_variance(self):
        """Test difference between population and sample variance"""
        self.execute_code('load "stdlib/statistics.gr"')
        self.execute_code('test_data = [1, 2, 3, 4, 5]')
        self.execute_code('sample_var = variance(test_data)')
        self.execute_code('pop_var = population_variance(test_data)')

        sample_var = self.get_variable('sample_var')
        pop_var = self.get_variable('pop_var')

        # Population variance should be smaller (divides by n, not n-1)
        assert pop_var.value < sample_var.value

    def test_population_std_vs_sample_std(self):
        """Test difference between population and sample standard deviation"""
        self.execute_code('load "stdlib/statistics.gr"')
        self.execute_code('test_data = [1, 2, 3, 4, 5]')
        self.execute_code('sample_std = std(test_data)')
        self.execute_code('pop_std = population_std(test_data)')

        sample_std = self.get_variable('sample_std')
        pop_std = self.get_variable('pop_std')

        # Population std should be smaller
        assert pop_std.value < sample_std.value

    def test_min_max_functions(self):
        """Test min_value() and max_value() functions"""
        self.execute_code('load "stdlib/statistics.gr"')
        self.execute_code('test_data = [3, 1, 4, 1, 5, 9, 2, 6]')
        self.execute_code('min_val = min_value(test_data)')
        self.execute_code('max_val = max_value(test_data)')

        min_val = self.get_variable('min_val')
        max_val = self.get_variable('max_val')

        assert min_val.value == 1
        assert max_val.value == 9

    def test_range_function(self):
        """Test range_value() function"""
        self.execute_code('load "stdlib/statistics.gr"')
        self.execute_code('test_data = [10, 20, 5, 25, 15]')
        self.execute_code('result = range_value(test_data)')

        result = self.get_variable('result')
        assert result.value == 20  # max(25) - min(5) = 20

    def test_covariance_function(self):
        """Test covariance() function"""
        self.execute_code('load "stdlib/statistics.gr"')

        # Perfectly correlated data
        self.execute_code('data1 = [1, 2, 3, 4, 5]')
        self.execute_code('data2 = [2, 4, 6, 8, 10]')  # data2 = 2 * data1
        self.execute_code('cov_result = covariance(data1, data2)')

        cov_result = self.get_variable('cov_result')
        assert cov_result.value > 0  # Positive covariance for positive correlation

    def test_covariance_edge_cases(self):
        """Test covariance() function edge cases"""
        self.execute_code('load "stdlib/statistics.gr"')

        # Different sized arrays should return none
        self.execute_code('data1 = [1, 2, 3]')
        self.execute_code('data2 = [1, 2]')
        self.execute_code('cov_diff_size = covariance(data1, data2)')

        cov_diff_size = self.get_variable('cov_diff_size')
        assert cov_diff_size.is_none()

        # Single value arrays should return none
        self.execute_code('single1 = [5]')
        self.execute_code('single2 = [10]')
        self.execute_code('cov_single = covariance(single1, single2)')

        cov_single = self.get_variable('cov_single')
        assert cov_single.is_none()

    def test_correlation_function(self):
        """Test correlation() function"""
        self.execute_code('load "stdlib/statistics.gr"')

        # Perfect positive correlation
        self.execute_code('data1 = [1, 2, 3, 4, 5]')
        self.execute_code('data2 = [2, 4, 6, 8, 10]')
        self.execute_code('corr_perfect = correlation(data1, data2)')

        corr_perfect = self.get_variable('corr_perfect')
        assert abs(corr_perfect.value - 1.0) < 0.01  # Should be very close to 1

        # Weak correlation
        self.execute_code('data3 = [1, 2, 3, 4, 5]')
        self.execute_code('data4 = [5, 3, 1, 4, 2]')  # Random order
        self.execute_code('corr_weak = correlation(data3, data4)')

        corr_weak = self.get_variable('corr_weak')
        # Should be moderate correlation (not perfect)
        assert abs(corr_weak.value) < 1.0  # Not perfect correlation

    def test_correlation_edge_cases(self):
        """Test correlation() function edge cases"""
        self.execute_code('load "stdlib/statistics.gr"')

        # Zero variance should return none
        self.execute_code('constant = [5, 5, 5, 5]')
        self.execute_code('variable = [1, 2, 3, 4]')
        self.execute_code('corr_constant = correlation(constant, variable)')

        corr_constant = self.get_variable('corr_constant')
        assert corr_constant.is_none()

    def test_describe_function_comprehensive(self):
        """Test describe() function returns complete statistics"""
        self.execute_code('load "stdlib/statistics.gr"')
        self.execute_code('test_data = [10, 20, 30, 40, 50]')
        self.execute_code('stats = describe(test_data)')

        # Check all expected fields are present
        self.execute_code('count = stats["count"]')
        self.execute_code('mean_val = stats["mean"]')
        self.execute_code('std_val = stats["std"]')
        self.execute_code('min_val = stats["min"]')
        self.execute_code('max_val = stats["max"]')
        self.execute_code('range_val = stats["range"]')

        count = self.get_variable('count')
        mean_val = self.get_variable('mean_val')
        min_val = self.get_variable('min_val')
        max_val = self.get_variable('max_val')
        range_val = self.get_variable('range_val')

        assert count.value == 5
        assert mean_val.value == 30  # (10+20+30+40+50)/5
        assert min_val.value == 10
        assert max_val.value == 50
        assert range_val.value == 40  # 50 - 10

    def test_describe_function_empty_data(self):
        """Test describe() function with empty data"""
        self.execute_code('load "stdlib/statistics.gr"')
        self.execute_code('empty_data = []')
        self.execute_code('stats = describe(empty_data)')

        self.execute_code('count = stats["count"]')
        self.execute_code('mean_val = stats["mean"]')

        count = self.get_variable('count')
        mean_val = self.get_variable('mean_val')

        assert count.value == 0
        assert mean_val.is_none()

    def test_std_dev_alias(self):
        """Test that std_dev() is an alias for std()"""
        self.execute_code('load "stdlib/statistics.gr"')
        self.execute_code('test_data = [1, 2, 3, 4, 5]')
        self.execute_code('std_result = std(test_data)')
        self.execute_code('std_dev_result = std_dev(test_data)')

        std_result = self.get_variable('std_result')
        std_dev_result = self.get_variable('std_dev_result')

        assert std_result.value == std_dev_result.value

    def test_mathematical_properties(self):
        """Test mathematical properties of statistical functions"""
        self.execute_code('load "stdlib/statistics.gr"')

        # Test that variance of mean-centered data is correct
        self.execute_code('data = [1, 3, 5, 7, 9]')
        self.execute_code('data_mean = mean(data)')
        self.execute_code('data_var = variance(data)')
        self.execute_code('data_std = std(data)')

        # Verify std = sqrt(variance)
        self.execute_code('sqrt_var = data_var.sqrt()')

        data_std = self.get_variable('data_std')
        sqrt_var = self.get_variable('sqrt_var')

        # std should equal sqrt(variance)
        assert abs(data_std.value - sqrt_var.value) < 0.001


class TestStatisticsModuleIntegration:
    """Integration tests for statistics module with other systems"""

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

    def test_statistics_with_list_operations(self):
        """Test statistics functions work with list manipulation"""
        self.execute_code('load "stdlib/statistics.gr"')

        # Create and manipulate data
        self.execute_code('data = [1, 2, 3]')
        self.execute_code('data.append(4)')
        self.execute_code('data.append(5)')

        # Use statistics on manipulated data
        self.execute_code('final_mean = mean(data)')
        self.execute_code('final_std = std(data)')

        final_mean = self.get_variable('final_mean')
        assert final_mean.value == 3.0  # (1+2+3+4+5)/5

    def test_statistics_with_mathematical_operations(self):
        """Test statistics functions work with mathematical transformations"""
        self.execute_code('load "stdlib/statistics.gr"')

        # Transform data and compute statistics
        self.execute_code('base_data = [1, 2, 3, 4, 5]')
        self.execute_code('transformed = []')
        self.execute_code('for value in base_data { transformed.append(value * 2) }')

        self.execute_code('base_mean = mean(base_data)')
        self.execute_code('trans_mean = mean(transformed)')

        base_mean = self.get_variable('base_mean')
        trans_mean = self.get_variable('trans_mean')

        # Transformed mean should be double the base mean
        assert abs(trans_mean.value - (base_mean.value * 2)) < 0.001

    def test_statistics_precision_consistency(self):
        """Test that statistics functions work consistently with precision contexts"""
        self.execute_code('load "stdlib/statistics.gr"')

        self.execute_code('data = [1, 2, 3, 4, 5]')
        self.execute_code('normal_mean = mean(data)')

        # Test with precision context
        self.execute_code('''
        precision 2 {
            precision_mean = mean(data)
        }
        ''')

        normal_mean = self.get_variable('normal_mean')
        precision_mean = self.get_variable('precision_mean')

        # Should be functionally equivalent (precision affects display, not calculation)
        assert abs(normal_mean.value - precision_mean.value) < 0.01
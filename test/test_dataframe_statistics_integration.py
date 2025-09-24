#!/usr/bin/env python3

"""
Tests for DataFrame + Statistics module integration
Verifies that DataFrames can leverage the comprehensive statistics module
"""

import pytest
import sys
import os

# Add the src directory to the path so we can import glang modules
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from glang.execution.pipeline import ExecutionSession
from glang.files.file_manager import FileManager


class TestDataFrameStatisticsIntegration:
    """Tests for DataFrame + Statistics integration"""

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

    def test_dataframe_column_statistics(self):
        """Test that statistics module can analyze tabular data (DataFrame-like)"""
        # Load statistics module
        self.execute_code('load "stdlib/statistics.gr"')

        # Create DataFrame-like data structure manually
        self.execute_code('employees = {}')
        self.execute_code('employees["salary"] = [65000, 75000, 85000, 95000, 105000]')
        self.execute_code('employees["age"] = [25, 30, 35, 40, 45]')
        self.execute_code('employees["_columns"] = ["salary", "age"]')
        self.execute_code('employees["_row_count"] = 5')

        # Use statistics module on tabular data columns
        self.execute_code('salary_mean = mean(employees["salary"])')
        self.execute_code('salary_std = std(employees["salary"])')
        self.execute_code('age_range = range_value(employees["age"])')

        # Verify results
        salary_mean = self.get_variable('salary_mean')
        salary_std = self.get_variable('salary_std')
        age_range = self.get_variable('age_range')

        assert salary_mean.value == 85000  # Mean of salaries
        assert salary_std.value > 0  # Should have standard deviation
        assert age_range.value == 20  # 45 - 25 = 20

    def test_dataframe_comprehensive_analysis(self):
        """Test comprehensive statistical analysis of tabular data"""
        # Load statistics module
        self.execute_code('load "stdlib/statistics.gr"')

        # Create tabular data structure manually
        self.execute_code('students = {}')
        self.execute_code('students["test_scores"] = [88, 92, 78, 96, 84, 90]')
        self.execute_code('students["hours_studied"] = [5, 8, 3, 10, 4, 7]')

        # Comprehensive statistical analysis
        self.execute_code('scores_stats = describe(students["test_scores"])')
        self.execute_code('hours_stats = describe(students["hours_studied"])')

        # Verify comprehensive stats were generated
        self.execute_code('scores_count = scores_stats["count"]')
        self.execute_code('scores_mean = scores_stats["mean"]')
        self.execute_code('hours_count = hours_stats["count"]')

        scores_count = self.get_variable('scores_count')
        scores_mean = self.get_variable('scores_mean')
        hours_count = self.get_variable('hours_count')

        assert scores_count.value == 6
        assert abs(scores_mean.value - 88.0) < 1  # Mean should be around 88
        assert hours_count.value == 6

    def test_dataframe_correlation_analysis(self):
        """Test correlation analysis between tabular data columns"""
        # Load statistics module
        self.execute_code('load "stdlib/statistics.gr"')

        # Create tabular data with correlated columns
        self.execute_code('people = {}')
        self.execute_code('people["height"] = [170, 175, 180, 185, 190]')  # Heights in cm
        self.execute_code('people["weight"] = [65, 70, 75, 80, 85]')       # Weights in kg (correlated)

        # Analyze correlation
        self.execute_code('height_weight_corr = correlation(people["height"], people["weight"])')

        height_weight_corr = self.get_variable('height_weight_corr')

        # Height and weight should be positively correlated
        assert height_weight_corr.value > 0.9  # Very strong positive correlation

    def test_dataframe_multiple_column_analysis(self):
        """Test analyzing multiple tabular data columns simultaneously"""
        # Load statistics module
        self.execute_code('load "stdlib/statistics.gr"')

        # Create tabular data with multiple numeric columns
        self.execute_code('quarterly_sales = {}')
        self.execute_code('quarterly_sales["Q1"] = [1000, 1500, 2000, 1800, 1200]')
        self.execute_code('quarterly_sales["Q2"] = [1100, 1600, 2100, 1900, 1300]')
        self.execute_code('quarterly_sales["Q3"] = [1200, 1700, 2200, 2000, 1400]')

        # Analyze each quarter
        self.execute_code('q1_mean = mean(quarterly_sales["Q1"])')
        self.execute_code('q2_mean = mean(quarterly_sales["Q2"])')
        self.execute_code('q3_mean = mean(quarterly_sales["Q3"])')

        # Compare quarters
        self.execute_code('q1_std = std(quarterly_sales["Q1"])')
        self.execute_code('q3_std = std(quarterly_sales["Q3"])')

        q1_mean = self.get_variable('q1_mean')
        q2_mean = self.get_variable('q2_mean')
        q3_mean = self.get_variable('q3_mean')
        q1_std = self.get_variable('q1_std')
        q3_std = self.get_variable('q3_std')

        # Q3 should have higher mean than Q1 (growth)
        assert q3_mean.value > q2_mean.value > q1_mean.value
        # Standard deviations should be similar (consistent variation)
        assert abs(q1_std.value - q3_std.value) < 100

    def test_dataframe_with_existing_functions(self):
        """Test that statistics work with tabular data operations"""
        # Load statistics module
        self.execute_code('load "stdlib/statistics.gr"')

        # Create tabular data
        self.execute_code('products = {}')
        self.execute_code('products["price"] = [10, 20, 30, 40, 50]')
        self.execute_code('products["quantity"] = [100, 80, 60, 40, 20]')

        # Use statistics on data columns
        self.execute_code('price_mean = mean(products["price"])')
        self.execute_code('price_variance = variance(products["price"])')

        price_mean = self.get_variable('price_mean')
        price_variance = self.get_variable('price_variance')

        assert price_mean.value == 30  # (10+20+30+40+50)/5
        assert price_variance.value > 0  # Should have variance

    def test_dataframe_statistical_filtering(self):
        """Test statistical analysis for filtering decisions"""
        # Load statistics module
        self.execute_code('load "stdlib/statistics.gr"')

        # Create tabular data with varied scores
        self.execute_code('students = {}')
        self.execute_code('students["score"] = [95, 85, 75, 65, 55, 92, 88, 72]')
        self.execute_code('students["name"] = ["Alice", "Bob", "Carol", "Dave", "Eve", "Frank", "Grace", "Henry"]')

        # Calculate statistical thresholds
        self.execute_code('score_mean = mean(students["score"])')
        self.execute_code('score_std = std(students["score"])')

        # Test that statistical calculations work for filtering decisions
        score_mean = self.get_variable('score_mean')
        score_std = self.get_variable('score_std')

        assert score_mean.value == 78.375  # Mean of the scores
        assert score_std.value > 0  # Should have standard deviation

    def test_dataframe_grouped_statistics(self):
        """Test statistics on grouped tabular data"""
        # Load statistics module
        self.execute_code('load "stdlib/statistics.gr"')

        # Simulate grouped analysis by extracting department data manually
        self.execute_code('sales_salaries = [50000, 55000]')
        self.execute_code('eng_salaries = [80000, 85000, 90000]')

        # Analyze groups separately
        self.execute_code('sales_mean = mean(sales_salaries)')
        self.execute_code('eng_mean = mean(eng_salaries)')
        self.execute_code('sales_std = std(sales_salaries)')
        self.execute_code('eng_std = std(eng_salaries)')

        sales_mean = self.get_variable('sales_mean')
        eng_mean = self.get_variable('eng_mean')
        sales_std = self.get_variable('sales_std')
        eng_std = self.get_variable('eng_std')

        # Engineering should have higher average salary
        assert eng_mean.value > sales_mean.value
        # Both should have some standard deviation
        assert sales_std.value > 0
        assert eng_std.value > 0

    def test_dataframe_time_series_statistics(self):
        """Test statistics on time series tabular data"""
        # Load statistics module
        self.execute_code('load "stdlib/statistics.gr"')

        # Calculate rolling statistics (manual implementation)
        self.execute_code('first_quarter = [1000, 1100, 1050]')
        self.execute_code('second_quarter = [1200, 1300, 1250]')

        self.execute_code('q1_mean = mean(first_quarter)')
        self.execute_code('q2_mean = mean(second_quarter)')
        self.execute_code('q1_std = std(first_quarter)')
        self.execute_code('q2_std = std(second_quarter)')

        q1_mean = self.get_variable('q1_mean')
        q2_mean = self.get_variable('q2_mean')
        q1_std = self.get_variable('q1_std')
        q2_std = self.get_variable('q2_std')

        # Second quarter should have higher average sales
        assert q2_mean.value > q1_mean.value
        # Both quarters should have variability
        assert q1_std.value > 0
        assert q2_std.value > 0

    def test_dataframe_advanced_statistical_operations(self):
        """Test advanced statistical operations on tabular data"""
        # Load statistics module
        self.execute_code('load "stdlib/statistics.gr"')

        # Create financial data
        self.execute_code('stock_prices = [100, 105, 98, 110, 108, 115, 120, 118]')
        self.execute_code('volumes = [1000, 1200, 800, 1500, 1300, 1600, 1800, 1400]')

        # Advanced statistical analysis
        self.execute_code('price_stats = describe(stock_prices)')
        self.execute_code('volume_stats = describe(volumes)')
        self.execute_code('price_volume_corr = correlation(stock_prices, volumes)')

        # Population vs sample statistics
        self.execute_code('price_sample_std = std(stock_prices)')
        self.execute_code('price_pop_std = population_std(stock_prices)')

        price_stats = self.get_variable('price_stats')
        price_volume_corr = self.get_variable('price_volume_corr')
        price_sample_std = self.get_variable('price_sample_std')
        price_pop_std = self.get_variable('price_pop_std')

        # Verify comprehensive analysis
        self.execute_code('price_count = price_stats["count"]')
        price_count = self.get_variable('price_count')
        assert price_count.value == 8

        # Sample std should be larger than population std
        assert price_sample_std.value > price_pop_std.value

        # Correlation should exist (may be positive or negative)
        assert price_volume_corr.value is not None


class TestDataFrameStatisticsPerformance:
    """Performance and edge case tests"""

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

    def test_large_dataframe_statistics(self):
        """Test statistics on larger datasets"""
        # Load statistics module
        self.execute_code('load "stdlib/statistics.gr"')

        # Create larger dataset
        self.execute_code('large_data = []')
        self.execute_code('for i in [].upto(99) { large_data.append(i + 1) }')  # 1 to 100

        # Calculate statistics on large dataset
        self.execute_code('large_mean = mean(large_data)')
        self.execute_code('large_std = std(large_data)')
        self.execute_code('large_range = range_value(large_data)')

        large_mean = self.get_variable('large_mean')
        large_std = self.get_variable('large_std')
        large_range = self.get_variable('large_range')

        assert abs(large_mean.value - 50.5) < 0.1  # Mean of 1-100 is 50.5
        assert large_std.value > 0
        assert large_range.value == 99  # 100 - 1 = 99

    def test_dataframe_statistics_edge_cases(self):
        """Test statistics on edge cases"""
        # Load statistics module
        self.execute_code('load "stdlib/statistics.gr"')

        # Test with single value
        self.execute_code('single_value = [42]')
        self.execute_code('single_mean = mean(single_value)')
        self.execute_code('single_std = std(single_value)')

        single_mean = self.get_variable('single_mean')
        single_std = self.get_variable('single_std')

        assert single_mean.value == 42
        assert single_std.value == 0  # No variation in single value

        # Test with identical values
        self.execute_code('identical_values = [10, 10, 10, 10]')
        self.execute_code('identical_mean = mean(identical_values)')
        self.execute_code('identical_std = std(identical_values)')

        identical_mean = self.get_variable('identical_mean')
        identical_std = self.get_variable('identical_std')

        assert identical_mean.value == 10
        assert identical_std.value == 0  # No variation
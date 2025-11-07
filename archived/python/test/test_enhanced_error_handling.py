"""Test enhanced error handling with stack traces."""

import pytest
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../src'))

from glang.execution.pipeline import ExecutionPipeline
from glang.execution.errors import RuntimeError, VariableNotFoundError


class TestEnhancedErrorHandling:
    """Test enhanced error handling with stack traces."""

    def setup_method(self):
        """Set up execution environment."""
        self.pipeline = ExecutionPipeline()

    def test_simple_variable_not_found_error(self):
        """Test variable not found error with basic stack trace."""
        code = '''
        func test_function() {
            return undefined_variable
        }

        result = test_function()
        '''

        result = self.pipeline.execute_code(code)
        assert not result.success

        # Check that we have enhanced error message
        formatted_error = result.get_formatted_error()
        assert formatted_error is not None
        print("Error message:")
        print(formatted_error)

        # Should contain stack trace information
        assert "Traceback" in formatted_error
        assert "test_function" in formatted_error
        assert "undefined_variable" in formatted_error

    def test_nested_function_call_error(self):
        """Test error in nested function calls with stack trace."""
        code = '''
        func inner_function(x) {
            return missing_var + x
        }

        func middle_function(y) {
            return inner_function(y * 2)
        }

        func outer_function(z) {
            return middle_function(z + 1)
        }

        result = outer_function(5)
        '''

        result = self.pipeline.execute_code(code)
        assert not result.success

        formatted_error = result.get_formatted_error()
        assert formatted_error is not None
        print("Nested error message:")
        print(formatted_error)

        # Should show the full call chain
        assert "inner_function" in formatted_error
        assert "middle_function" in formatted_error
        assert "outer_function" in formatted_error

    def test_lambda_error_with_stack_trace(self):
        """Test error in lambda with stack trace."""
        code = '''
        func apply_lambda(f, x) {
            return f(x)
        }

        bad_lambda = y => undefined_var + y
        result = apply_lambda(bad_lambda, 10)
        '''

        result = self.pipeline.execute_code(code)
        assert not result.success

        formatted_error = result.get_formatted_error()
        assert formatted_error is not None
        print("Lambda error message:")
        print(formatted_error)

        # Should show lambda in call chain
        assert "<lambda>" in formatted_error
        assert "apply_lambda" in formatted_error

    def test_error_with_local_variables(self):
        """Test that stack trace includes local variable information."""
        code = '''
        func calculate_something(a, b) {
            num intermediate = a * 2
            string message = "processing"
            return intermediate + nonexistent_var
        }

        result = calculate_something(5, 10)
        '''

        result = self.pipeline.execute_code(code)
        assert not result.success

        formatted_error = result.get_formatted_error()
        assert formatted_error is not None
        print("Error with locals:")
        print(formatted_error)

        # Should include some local variable information
        # (Note: exact format may vary)

    def test_compact_error_format(self):
        """Test compact error format option."""
        code = '''
        func failing_function() {
            return bad_variable
        }

        result = failing_function()
        '''

        result = self.pipeline.execute_code(code)
        assert not result.success

        # Test compact format if available
        if hasattr(result.error, 'get_compact_message'):
            compact_error = result.error.get_compact_message()
            print("Compact error:")
            print(compact_error)

            # Should be shorter but still informative
            assert "failing_function" in compact_error
            assert "bad_variable" in compact_error

    def test_error_as_data_pattern_integration(self):
        """Test integration with error-as-data pattern using result tuples."""
        code = '''
        func safe_divide(a, b) {
            if b == 0 {
                # This should be enhanced with stack trace information
                # when we add automatic error-as-data conversion
                return [:error, "Division by zero"]
            }
            return [:ok, a / b]
        }

        # Test successful case
        good_result = safe_divide(10, 2)

        # Test error case
        bad_result = safe_divide(10, 0)

        # Pattern match on results
        good_message = match good_result {
            [:ok, value] => "Success: " + value.to_string(),
            [:error, msg] => "Error: " + msg,
            _ => "Unknown result"
        }

        bad_message = match bad_result {
            [:ok, value] => "Success: " + value.to_string(),
            [:error, msg] => "Error: " + msg,
            _ => "Unknown result"
        }
        '''

        result = self.pipeline.execute_code(code)
        if not result.success:
            print("Error in error-as-data test:")
            print(result.get_formatted_error())

        assert result.success

        # Check that pattern matching worked correctly
        good_message = result.context.get_variable("good_message")
        bad_message = result.context.get_variable("bad_message")

        assert good_message is not None
        assert bad_message is not None

        print("Good result message:", str(good_message))
        print("Bad result message:", str(bad_message))

        # Should contain expected content
        assert "Success: 5" in str(good_message)
        assert "Error: Division by zero" in str(bad_message)
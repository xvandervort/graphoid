"""
Test the new dot operators for element-wise arithmetic.

This replaces the old mixed behavior with clean separation:
- +, -, *, /, % work only on numbers (or + for list concatenation, - for set difference)
- +., -., *., /., %. work for element-wise operations on lists and scalars
"""

import pytest
from src.glang.execution.pipeline import ExecutionSession


class TestDotOperators:
    """Test element-wise arithmetic operators."""
    
    def setup_method(self):
        """Set up test session for each test method."""
        self.session = ExecutionSession()
    
    def test_elementwise_list_list_addition(self):
        """Test element-wise addition between two lists."""
        self.session.execute_statement('list<num> a = [1, 2, 3]')
        self.session.execute_statement('list<num> b = [10, 20, 30]')
        result = self.session.execute_statement('result = a +. b')
        assert result.success
        
        result_value = self.session.execution_context.get_variable('result')
        assert [elem.value for elem in result_value.elements] == [11, 22, 33]
        assert result_value.constraint == "num"
    
    def test_elementwise_list_scalar_addition(self):
        """Test element-wise addition between list and scalar."""
        self.session.execute_statement('list<num> numbers = [1, 2, 3, 4]')
        result = self.session.execute_statement('result = numbers +. 10')
        assert result.success
        
        result_value = self.session.execution_context.get_variable('result')
        assert [elem.value for elem in result_value.elements] == [11, 12, 13, 14]
        assert result_value.constraint == "num"
    
    def test_elementwise_scalar_list_addition(self):
        """Test element-wise addition between scalar and list."""
        self.session.execute_statement('list<num> numbers = [5, 10, 15]')
        result = self.session.execute_statement('result = 100 +. numbers')
        assert result.success
        
        result_value = self.session.execution_context.get_variable('result')
        assert [elem.value for elem in result_value.elements] == [105, 110, 115]
        assert result_value.constraint == "num"
    
    def test_elementwise_subtraction(self):
        """Test element-wise subtraction."""
        self.session.execute_statement('a = [10, 20, 30]')
        self.session.execute_statement('b = [1, 3, 5]')
        result = self.session.execute_statement('result = a -. b')
        assert result.success
        
        result_value = self.session.execution_context.get_variable('result')
        assert [elem.value for elem in result_value.elements] == [9, 17, 25]
    
    def test_elementwise_multiplication(self):
        """Test element-wise multiplication."""
        self.session.execute_statement('a = [2, 3, 4]')
        result = self.session.execute_statement('result = a *. 3')
        assert result.success
        
        result_value = self.session.execution_context.get_variable('result')
        assert [elem.value for elem in result_value.elements] == [6, 9, 12]
    
    def test_elementwise_division(self):
        """Test element-wise division."""
        self.session.execute_statement('a = [12, 18, 24]')
        result = self.session.execute_statement('result = a /. 3')
        assert result.success
        
        result_value = self.session.execution_context.get_variable('result')
        assert [elem.value for elem in result_value.elements] == [4.0, 6.0, 8.0]
    
    def test_elementwise_modulo(self):
        """Test element-wise modulo."""
        self.session.execute_statement('a = [7, 8, 9, 10]')
        result = self.session.execute_statement('result = a %. 3')
        assert result.success
        
        result_value = self.session.execution_context.get_variable('result')
        assert [elem.value for elem in result_value.elements] == [1, 2, 0, 1]
    
    def test_type_inference_works_with_dot_operators(self):
        """Test that type inference works with dot operators."""
        self.session.execute_statement('a = [1, 2, 3]')  # Type inference
        result = self.session.execute_statement('result = a +. 5')
        assert result.success
        
        result_value = self.session.execution_context.get_variable('result')
        assert [elem.value for elem in result_value.elements] == [6, 7, 8]
        assert result_value.constraint == "num"
    
    def test_different_length_lists_fail(self):
        """Test that element-wise operations require same-length lists."""
        self.session.execute_statement('a = [1, 2, 3]')
        self.session.execute_statement('b = [10, 20]')  # Different length
        result = self.session.execute_statement('result = a +. b')
        assert not result.success
        assert "same length" in str(result.error).lower()
    
    def test_string_lists_fail_with_dot_operators(self):
        """Test that string lists fail with dot operators."""
        self.session.execute_statement('words = ["hello", "world"]')
        result = self.session.execute_statement('result = words +. 5')
        assert not result.success
        assert "numeric" in str(result.error).lower() or "mixed types" in str(result.error).lower()
    
    def test_division_by_zero(self):
        """Test division by zero with dot operators."""
        self.session.execute_statement('numbers = [1, 2, 3]')
        result = self.session.execute_statement('result = numbers /. 0')
        assert not result.success
        assert "division by zero" in str(result.error).lower()


class TestListOperationsStillWork:
    """Test that list operations (concatenation, set difference) still work correctly."""
    
    def setup_method(self):
        """Set up test session for each test method."""
        self.session = ExecutionSession()
    
    def test_list_concatenation_always_works(self):
        """Test that + always concatenates lists regardless of length or content."""
        # Same length numeric lists - should concatenate, not element-wise
        self.session.execute_statement('a = [1, 2, 3]')
        self.session.execute_statement('b = [10, 20, 30]')
        result = self.session.execute_statement('result = a + b')
        assert result.success
        
        result_value = self.session.execution_context.get_variable('result')
        assert [elem.value for elem in result_value.elements] == [1, 2, 3, 10, 20, 30]
        
        # Different length lists - should also concatenate
        self.session.execute_statement('c = [1, 2]')
        self.session.execute_statement('d = [10, 20, 30]')
        result = self.session.execute_statement('result2 = c + d')
        assert result.success
        
        result_value = self.session.execution_context.get_variable('result2')
        assert [elem.value for elem in result_value.elements] == [1, 2, 10, 20, 30]
        
        # String lists - should concatenate
        self.session.execute_statement('words1 = ["hello", "world"]')
        self.session.execute_statement('words2 = ["foo", "bar"]')
        result = self.session.execute_statement('result3 = words1 + words2')
        assert result.success
        
        result_value = self.session.execution_context.get_variable('result3')
        assert [elem.value for elem in result_value.elements] == ["hello", "world", "foo", "bar"]
    
    def test_list_set_difference_works(self):
        """Test that - does set difference on lists."""
        self.session.execute_statement('a = [1, 2, 3, 4, 5]')
        self.session.execute_statement('b = [2, 4]')
        result = self.session.execute_statement('result = a - b')
        assert result.success
        
        result_value = self.session.execution_context.get_variable('result')
        assert [elem.value for elem in result_value.elements] == [1, 3, 5]
    
    def test_old_arithmetic_operators_fail_on_lists(self):
        """Test that old arithmetic operators now give clear error messages."""
        self.session.execute_statement('a = [1, 2, 3]')
        self.session.execute_statement('b = [10, 20, 30]')
        
        # These should all fail with helpful messages
        result = self.session.execute_statement('result = a * b')
        assert not result.success
        assert "use *. for element-wise" in str(result.error)
        
        result = self.session.execute_statement('result = a / b')
        assert not result.success
        assert "use /. for element-wise" in str(result.error)
        
        result = self.session.execute_statement('result = a % b')
        assert not result.success
        assert "use %. for element-wise" in str(result.error)
        
        # List-scalar should also fail with helpful messages
        result = self.session.execute_statement('result = a * 5')
        assert not result.success
        assert "use *. for element-wise" in str(result.error)


class TestBackwardCompatibilityBreaking:
    """Test that confirms we've intentionally broken backward compatibility for clarity."""
    
    def setup_method(self):
        """Set up test session for each test method."""
        self.session = ExecutionSession()
    
    def test_old_confusing_behavior_is_gone(self):
        """Test that the old confusing length-dependent behavior is completely gone."""
        self.session.execute_statement('same_length_a = [1, 2, 3]')
        self.session.execute_statement('same_length_b = [10, 20, 30]')
        
        # This used to do element-wise for same-length numeric lists
        # Now it ALWAYS does concatenation
        result = self.session.execute_statement('result = same_length_a + same_length_b')
        assert result.success
        
        result_value = self.session.execution_context.get_variable('result')
        # Should be concatenation [1,2,3,10,20,30], NOT element-wise [11,22,33]
        assert [elem.value for elem in result_value.elements] == [1, 2, 3, 10, 20, 30]
        assert [elem.value for elem in result_value.elements] != [11, 22, 33]
        
    def test_element_wise_requires_explicit_dot_operators(self):
        """Test that element-wise operations now require explicit dot operators."""
        self.session.execute_statement('a = [1, 2, 3]')
        self.session.execute_statement('b = [10, 20, 30]')
        
        # Element-wise now requires explicit dot operator
        result = self.session.execute_statement('result = a +. b')
        assert result.success
        
        result_value = self.session.execution_context.get_variable('result')
        assert [elem.value for elem in result_value.elements] == [11, 22, 33]
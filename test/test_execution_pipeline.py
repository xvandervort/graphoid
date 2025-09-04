"""Tests for the execution pipeline."""

import pytest
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../src'))

from glang.execution.pipeline import ExecutionPipeline, ExecutionSession, ExecutionResult
from glang.execution.executor import ExecutionContext
from glang.execution.values import *
from glang.semantic.symbol_table import SymbolTable


class TestExecutionPipeline:
    """Test the complete execution pipeline."""
    
    def setup_method(self):
        self.pipeline = ExecutionPipeline()
    
    def test_simple_variable_declaration(self):
        """Test executing a simple variable declaration."""
        code = 'string greeting = "hello"'
        result = self.pipeline.execute_code(code)
        
        assert result.success is True
        assert "Declared string variable 'greeting'" in str(result.value)
        assert "greeting" in result.context.variables
        
        # Check the actual stored value
        stored_value = result.context.get_variable("greeting")
        assert isinstance(stored_value, StringValue)
        assert stored_value.value == "hello"
    
    def test_number_declaration(self):
        """Test declaring a number variable."""
        code = 'num score = 95'
        result = self.pipeline.execute_code(code)
        
        assert result.success is True
        stored_value = result.context.get_variable("score")
        assert isinstance(stored_value, NumberValue)
        assert stored_value.value == 95
    
    def test_boolean_declaration(self):
        """Test declaring a boolean variable."""
        code = 'bool active = true'
        result = self.pipeline.execute_code(code)
        
        assert result.success is True
        stored_value = result.context.get_variable("active")
        assert isinstance(stored_value, BooleanValue)
        assert stored_value.value is True
    
    def test_list_declaration(self):
        """Test declaring a list variable."""
        code = 'list numbers = [1, 2, 3]'
        result = self.pipeline.execute_code(code)
        
        assert result.success is True
        stored_value = result.context.get_variable("numbers")
        assert isinstance(stored_value, ListValue)
        assert len(stored_value) == 3
        assert stored_value.elements[0].value == 1
        assert stored_value.elements[1].value == 2
        assert stored_value.elements[2].value == 3
    
    def test_constrained_list_declaration(self):
        """Test declaring a type-constrained list."""
        code = 'list<string> names = ["alice", "bob"]'
        result = self.pipeline.execute_code(code)
        
        assert result.success is True
        stored_value = result.context.get_variable("names")
        assert isinstance(stored_value, ListValue)
        assert stored_value.constraint == "string"
        assert len(stored_value) == 2
    
    def test_syntax_error_handling(self):
        """Test handling of syntax errors."""
        code = 'invalid syntax here!'
        result = self.pipeline.execute_code(code)
        
        assert result.success is False
        # Could be tokenizer error or parse error
        assert ("Unexpected character" in str(result.error) or 
                "Parse error" in str(result.error) or 
                "Semantic analysis failed" in str(result.error))
    
    def test_semantic_error_handling(self):
        """Test handling of semantic errors."""
        # Try to declare variable with invalid type constraint
        code = 'list<invalid_type> data = [1, 2, 3]'
        result = self.pipeline.execute_code(code)
        
        assert result.success is False
        assert "Semantic analysis failed" in str(result.error)


class TestExecutionSession:
    """Test the execution session for persistent context."""
    
    def setup_method(self):
        self.session = ExecutionSession()
    
    def test_persistent_variables(self):
        """Test that variables persist across statements."""
        # Declare a variable
        result1 = self.session.execute_statement('string name = "alice"')
        assert result1.success is True
        
        # Use the variable in another statement  
        result2 = self.session.execute_statement('string greeting = name')
        assert result2.success is True
        
        # Check both variables exist
        variables = self.session.list_variables()
        assert "name" in variables
        assert "greeting" in variables
        
        # Check values
        name_value = self.session.get_variable_value("name")
        greeting_value = self.session.get_variable_value("greeting")
        assert name_value.value == "alice"
        assert greeting_value.value == "alice"
    
    def test_method_calls_in_session(self):
        """Test method calls in persistent session."""
        # Create a list
        result1 = self.session.execute_statement('list<num> scores = [85, 92]')
        assert result1.success is True
        
        # Append to the list
        result2 = self.session.execute_statement('scores.append(88)')
        assert result2.success is True
        
        # Check the list was modified
        scores_value = self.session.get_variable_value("scores")
        assert len(scores_value) == 3
        assert scores_value.elements[2].value == 88
    
    def test_index_access_in_session(self):
        """Test index access in session."""
        # Create a list
        self.session.execute_statement('list fruits = ["apple", "banana", "cherry"]')
        
        # Access first element
        result = self.session.execute_statement('fruits[0]')
        assert result.success is True
        assert isinstance(result.value, StringValue)
        assert result.value.value == "apple"
        
        # Access last element with negative index
        result = self.session.execute_statement('fruits[-1]')
        assert result.success is True
        assert result.value.value == "cherry"
    
    def test_index_assignment_in_session(self):
        """Test index assignment in session."""
        # Create a list
        self.session.execute_statement('list colors = ["red", "green", "blue"]')
        
        # Modify an element
        result = self.session.execute_statement('colors[1] = "yellow"')
        assert result.success is True
        
        # Check the modification
        colors_value = self.session.get_variable_value("colors")
        assert colors_value.elements[1].value == "yellow"
    
    def test_type_constraint_enforcement(self):
        """Test that type constraints are enforced at runtime."""
        # Create constrained list
        self.session.execute_statement('list<num> numbers = [1, 2, 3]')
        
        # Try to append wrong type
        result = self.session.execute_statement('numbers.append("string")')
        assert result.success is False
        assert "Cannot append string to list<num>" in str(result.error)
        
        # Verify list wasn't modified
        numbers_value = self.session.get_variable_value("numbers")
        assert len(numbers_value) == 3
    
    def test_variable_reassignment(self):
        """Test variable reassignment in session."""
        # Declare variable
        self.session.execute_statement('string message = "hello"')
        
        # Reassign it
        result = self.session.execute_statement('message = "goodbye"')
        assert result.success is True
        
        # Check new value
        message_value = self.session.get_variable_value("message")
        assert message_value.value == "goodbye"
    
    def test_session_info(self):
        """Test session information retrieval."""
        # Initially empty
        info = self.session.get_session_info()
        assert info['variable_count'] == 0
        assert info['variables'] == []
        
        # Add some variables
        self.session.execute_statement('string name = "test"')
        self.session.execute_statement('num count = 5')
        
        # Check updated info
        info = self.session.get_session_info()
        assert info['variable_count'] == 2
        assert "name" in info['variables']
        assert "count" in info['variables']
    
    def test_list_variables_details(self):
        """Test detailed variable listing."""
        # Create different types of variables
        self.session.execute_statement('string text = "hello"')
        self.session.execute_statement('num value = 42')
        self.session.execute_statement('bool flag = true')
        self.session.execute_statement('list<string> items = ["a", "b"]')
        
        variables = self.session.list_variables()
        
        # Check text variable
        assert variables['text']['type'] == 'string'
        assert variables['text']['display'] == 'hello'
        
        # Check num variable
        assert variables['value']['type'] == 'num'
        assert variables['value']['display'] == '42'
        
        # Check bool variable
        assert variables['flag']['type'] == 'bool'
        assert variables['flag']['display'] == 'true'
        
        # Check list variable
        assert variables['items']['type'] == 'list'
        assert 'a' in variables['items']['display']
        assert 'b' in variables['items']['display']
    
    def test_clear_variables(self):
        """Test clearing all variables from session."""
        # Add variables
        self.session.execute_statement('string test = "value"')
        self.session.execute_statement('num count = 10')
        
        assert self.session.get_session_info()['variable_count'] == 2
        
        # Clear variables
        self.session.clear_variables()
        
        # Verify cleared
        assert self.session.get_session_info()['variable_count'] == 0
        assert self.session.get_variable_value("test") is None
        assert self.session.get_variable_value("count") is None


class TestExecutionResult:
    """Test the ExecutionResult class."""
    
    def test_successful_result_string(self):
        """Test string representation of successful result."""
        context = ExecutionContext(SymbolTable())
        result = ExecutionResult("test value", context, True)
        
        assert str(result) == "test value"
    
    def test_failed_result_string(self):
        """Test string representation of failed result."""
        context = ExecutionContext(SymbolTable())
        error = Exception("Test error")
        result = ExecutionResult(None, context, False, error)
        
        assert "Execution failed: Test error" in str(result)
    
    def test_no_result_string(self):
        """Test string representation when no result value."""
        context = ExecutionContext(SymbolTable())
        result = ExecutionResult(None, context, True)
        
        assert str(result) == "No result"


class TestComplexExecutionScenarios:
    """Test complex execution scenarios."""
    
    def setup_method(self):
        self.session = ExecutionSession()
    
    def test_mixed_operations(self):
        """Test mixing different operations in one session."""
        # Declare multiple variables
        self.session.execute_statement('list<num> scores = [85, 92, 78]')
        self.session.execute_statement('string subject = "math"')
        
        # Perform operations
        self.session.execute_statement('scores.append(95)')
        self.session.execute_statement('scores[0] = 90')  # Modify first score
        
        # Access values
        result = self.session.execute_statement('scores[1]')
        assert result.value.value == 92
        
        # Verify final state
        scores = self.session.get_variable_value("scores")
        assert len(scores) == 4
        assert scores.elements[0].value == 90  # Modified
        assert scores.elements[3].value == 95  # Appended
    
    def test_original_bug_scenario(self):
        """Test the original bug scenario that motivated this refactoring."""
        # This was the original problem:
        # 'list a = [1, 2]' then 'd.append a' resulted in ['a'] instead of [1, 2]
        
        # Create first list
        result1 = self.session.execute_statement('list a = [1, 2]')
        assert result1.success is True
        
        # Create second list
        result2 = self.session.execute_statement('list d = []')
        assert result2.success is True
        
        # Append variable contents (not literal 'a')
        result3 = self.session.execute_statement('d.append(a)')
        
        # This should fail because we're trying to append a list to a list
        # but if we had list<list> d, it should work
        
        # Let's test the corrected scenario
        self.session.clear_variables()
        
        self.session.execute_statement('list a = [1, 2]')
        self.session.execute_statement('list<list> d = []')
        result = self.session.execute_statement('d.append(a)')
        
        assert result.success is True
        
        # Verify d contains the list [1, 2], not the string 'a'
        d_value = self.session.get_variable_value("d")
        assert len(d_value) == 1
        appended_list = d_value.elements[0]
        assert isinstance(appended_list, ListValue)
        assert len(appended_list) == 2
        assert appended_list.elements[0].value == 1
        assert appended_list.elements[1].value == 2


if __name__ == '__main__':
    pytest.main([__file__])
"""Tests for type inference functionality (Phase 4)."""

import pytest
from glang.execution import ExecutionSession
from glang.execution.values import StringValue, NumberValue, BooleanValue
from glang.execution.graph_values import ListValue, HashValue


class TestTypeInference:
    """Test type inference for implicit variable declarations."""
    
    def setup_method(self):
        """Set up test session."""
        self.session = ExecutionSession()
    
    def test_string_type_inference(self):
        """Test that string literals are inferred as string type."""
        result = self.session.execute_statement('name = "Alice"')
        
        assert result.success
        assert result.value == "Declared string variable 'name' (inferred)"
        
        # Verify variable exists and has correct type
        name_var = result.context.get_variable('name')
        assert isinstance(name_var, StringValue)
        assert name_var.value == "Alice"
        assert name_var.get_type() == "string"
    
    def test_number_type_inference(self):
        """Test that number literals are inferred as num type."""
        # Integer
        result = self.session.execute_statement('age = 25')
        
        assert result.success
        assert result.value == "Declared num variable 'age' (inferred)"
        
        age_var = result.context.get_variable('age')
        assert isinstance(age_var, NumberValue)
        assert age_var.value == 25
        assert age_var.get_type() == "num"
        
        # Float
        result = self.session.execute_statement('pi = 3.14')
        
        assert result.success
        pi_var = result.context.get_variable('pi')
        assert isinstance(pi_var, NumberValue)
        assert pi_var.value == 3.14
        assert pi_var.get_type() == "num"
    
    def test_boolean_type_inference(self):
        """Test that boolean literals are inferred as bool type."""
        # True
        result = self.session.execute_statement('flag = true')
        
        assert result.success
        assert result.value == "Declared bool variable 'flag' (inferred)"
        
        flag_var = result.context.get_variable('flag')
        assert isinstance(flag_var, BooleanValue)
        assert flag_var.value == True
        assert flag_var.get_type() == "bool"
        
        # False
        result = self.session.execute_statement('disabled = false')
        
        assert result.success
        disabled_var = result.context.get_variable('disabled')
        assert isinstance(disabled_var, BooleanValue)
        assert disabled_var.value == False
        assert disabled_var.get_type() == "bool"
    
    def test_list_type_inference(self):
        """Test that list literals are inferred as list type."""
        result = self.session.execute_statement('items = ["apple", "banana"]')
        
        assert result.success
        assert result.value == "Declared list variable 'items' (inferred)"
        
        items_var = result.context.get_variable('items')
        assert isinstance(items_var, ListValue)
        assert len(items_var.elements) == 2
        assert items_var.get_type() == "list"
        assert items_var.constraint is None  # No constraint inferred
    
    def test_empty_list_inference(self):
        """Test that empty lists are inferred as unconstrained lists."""
        result = self.session.execute_statement('empty = []')
        
        assert result.success
        
        empty_var = result.context.get_variable('empty')
        assert isinstance(empty_var, ListValue)
        assert len(empty_var.elements) == 0
        assert empty_var.get_type() == "list"
        assert empty_var.constraint is None
    
    def test_reassignment_after_inference(self):
        """Test that variables can be reassigned after type inference."""
        # Create with inference
        result = self.session.execute_statement('value = "hello"')
        assert result.success
        
        value_var = result.context.get_variable('value')
        assert isinstance(value_var, StringValue)
        assert value_var.value == "hello"
        
        # Reassign with new string
        result = self.session.execute_statement('value = "world"')
        assert result.success
        assert result.value == 'Assigned world to value'
        
        value_var = result.context.get_variable('value')
        assert isinstance(value_var, StringValue)
        assert value_var.value == "world"
    
    def test_mixed_inference_and_explicit(self):
        """Test that inferred and explicit declarations work together."""
        # Explicit declaration
        result = self.session.execute_statement('string explicit = "Bob"')
        assert result.success
        assert result.value == "Declared string variable 'explicit'"
        
        # Inferred declaration
        result = self.session.execute_statement('inferred = "Alice"')
        assert result.success
        assert result.value == "Declared string variable 'inferred' (inferred)"
        
        # Both should work
        explicit_var = result.context.get_variable('explicit')
        inferred_var = result.context.get_variable('inferred')
        
        assert isinstance(explicit_var, StringValue)
        assert isinstance(inferred_var, StringValue)
        assert explicit_var.value == "Bob"
        assert inferred_var.value == "Alice"
    
    def test_explicit_constraints_still_enforced(self):
        """Test that explicit type constraints are still enforced."""
        # Create constrained list
        result = self.session.execute_statement('list<string> names = ["Alice"]')
        assert result.success
        
        # Should enforce constraint
        result = self.session.execute_statement('names.append("Bob")')
        assert result.success
        
        # Should reject wrong type
        result = self.session.execute_statement('names.append(123)')
        assert not result.success
        assert "Cannot append num to list<string>" in str(result.error)


class TestTypeInferenceEdgeCases:
    """Test edge cases for type inference."""
    
    def setup_method(self):
        """Set up test session."""
        self.session = ExecutionSession()
    
    def test_variable_access_still_works(self):
        """Test that accessing existing variables still works correctly."""
        # Create variable with inference
        result = self.session.execute_statement('greeting = "Hello"')
        assert result.success
        
        # Access the variable
        result = self.session.execute_statement('greeting')
        assert result.success
        assert isinstance(result.value, StringValue)
        assert result.value.value == "Hello"
    
    def test_undefined_variable_error(self):
        """Test that accessing truly undefined variables still raises error."""
        result = self.session.execute_statement('undefined_var')
        assert not result.success
        assert "undefined_var" in str(result.error) and ("not found" in str(result.error) or "Undefined variable" in str(result.error))
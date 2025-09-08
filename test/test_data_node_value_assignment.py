"""Test data node value assignment functionality."""

import pytest
from glang.execution.pipeline import ExecutionSession


class TestDataNodeValueAssignment:
    """Test that data node values can be assigned while keys remain immutable."""
    
    def test_basic_value_assignment(self):
        """Test basic data node value assignment."""
        session = ExecutionSession()
        
        # Create data node
        result = session.execute_statement('data d = { "name": "Alice" }')
        assert result.success
        
        # Verify initial value
        result = session.execute_statement('d.value')
        assert result.success
        assert str(result.value) == "Alice"
        
        # Assign new value
        result = session.execute_statement('d.value = "Bob"')
        assert result.success
        assert "Updated data node" in str(result.value)
        
        # Verify value was updated
        result = session.execute_statement('d.value')
        assert result.success
        assert str(result.value) == "Bob"
        
        # Verify key remains unchanged
        result = session.execute_statement('d.key')
        assert result.success
        assert str(result.value) == "name"
    
    def test_value_assignment_different_types(self):
        """Test data node value assignment with different types."""
        session = ExecutionSession()
        
        # Create data node with string value
        result = session.execute_statement('data d = { "data": "initial" }')
        assert result.success
        
        # Assign number value
        result = session.execute_statement('d.value = 42')
        assert result.success
        result = session.execute_statement('d.value')
        assert result.success
        assert str(result.value) == "42"
        
        # Assign boolean value
        result = session.execute_statement('d.value = true')
        assert result.success
        result = session.execute_statement('d.value')
        assert result.success
        assert str(result.value) == "true"
        
        # Assign list value
        result = session.execute_statement('d.value = [1, 2, 3]')
        assert result.success
        result = session.execute_statement('d.value')
        assert result.success
        assert str(result.value) == "[1, 2, 3]"
    
    def test_key_assignment_blocked(self):
        """Test that data node key assignment is properly blocked."""
        session = ExecutionSession()
        
        # Create data node
        result = session.execute_statement('data d = { "original_key": "value" }')
        assert result.success
        
        # Try to assign new key (should fail)
        result = session.execute_statement('d.key = "new_key"')
        assert not result.success
        assert "Assignment to data node key is not allowed" in str(result.error)
        assert "immutable" in str(result.error)
        
        # Verify key remains unchanged
        result = session.execute_statement('d.key')
        assert result.success
        assert str(result.value) == "original_key"
    
    def test_constrained_data_node_value_assignment(self):
        """Test value assignment with type constraints."""
        session = ExecutionSession()
        
        # Create constrained data node
        result = session.execute_statement('data<string> d = { "name": "Alice" }')
        assert result.success
        
        # Assign valid string value (should work)
        result = session.execute_statement('d.value = "Bob"')
        assert result.success
        result = session.execute_statement('d.value')
        assert result.success
        assert str(result.value) == "Bob"
        
        # Try to assign invalid type (should fail)
        result = session.execute_statement('d.value = 42')
        assert not result.success
        # Should fail due to type constraint
    
    def test_multiple_data_nodes(self):
        """Test value assignment with multiple data nodes."""
        session = ExecutionSession()
        
        # Create multiple data nodes
        result = session.execute_statement('data person = { "name": "Alice" }')
        assert result.success
        result = session.execute_statement('data config = { "debug": true }')
        assert result.success
        
        # Update first data node
        result = session.execute_statement('person.value = "Bob"')
        assert result.success
        
        # Update second data node  
        result = session.execute_statement('config.value = false')
        assert result.success
        
        # Verify both are updated correctly
        result = session.execute_statement('person.value')
        assert result.success
        assert str(result.value) == "Bob"
        
        result = session.execute_statement('config.value')
        assert result.success
        assert str(result.value) == "false"
        
        # Verify keys remain unchanged
        result = session.execute_statement('person.key')
        assert result.success
        assert str(result.value) == "name"
        
        result = session.execute_statement('config.key')
        assert result.success
        assert str(result.value) == "debug"
    
    def test_value_assignment_expression_values(self):
        """Test data node value assignment with expression values."""
        session = ExecutionSession()
        
        # Create data nodes and variables
        result = session.execute_statement('data d = { "result": 0 }')
        assert result.success
        result = session.execute_statement('num x = 10')
        assert result.success
        result = session.execute_statement('num y = 5')
        assert result.success
        
        # Assign expression result
        result = session.execute_statement('d.value = x + y')
        assert result.success
        
        # Verify expression was evaluated
        result = session.execute_statement('d.value')
        assert result.success
        assert str(result.value) == "15"
    
    def test_data_node_display_after_value_assignment(self):
        """Test that data node displays correctly after value assignment."""
        session = ExecutionSession()
        
        # Create and modify data node
        result = session.execute_statement('data d = { "status": "initial" }')
        assert result.success
        
        # Check initial display
        result = session.execute_statement('d')
        assert result.success
        assert str(result.value) == '{ "status": initial }'
        
        # Update value
        result = session.execute_statement('d.value = "updated"')
        assert result.success
        
        # Check updated display
        result = session.execute_statement('d')
        assert result.success
        assert str(result.value) == '{ "status": updated }'
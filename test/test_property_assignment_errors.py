"""Test proper error handling for property assignment attempts."""

import pytest
from glang.parser.ast_parser import ASTParser
from glang.execution.pipeline import ExecutionSession
from glang.ast.nodes import Assignment, MethodCallExpression, VariableRef, DataNodeLiteral


class TestPropertyAssignmentErrors:
    """Test that property assignment attempts give proper error messages."""
    
    def test_data_property_assignment_parsing(self):
        """Test that property assignment attempts parse as Assignment nodes."""
        parser = ASTParser()
        
        # Test d.key = "value" parses as Assignment
        ast = parser.parse('d.key = "newkey"')
        assert isinstance(ast, Assignment)
        assert isinstance(ast.target, MethodCallExpression)
        assert ast.target.method_name == "key"
        assert isinstance(ast.target.target, VariableRef)
        assert ast.target.target.name == "d"
        assert ast.value.value == "newkey"
        
        # Test d.value = "newvalue" parses as Assignment
        ast = parser.parse('d.value = "newvalue"')
        assert isinstance(ast, Assignment)
        assert isinstance(ast.target, MethodCallExpression)
        assert ast.target.method_name == "value"
        assert ast.value.value == "newvalue"
    
    def test_data_property_assignment_execution_errors(self):
        """Test that property assignments fail at execution with proper error messages."""
        session = ExecutionSession()

        # Create a map and extract a data node
        result = session.execute_statement('m = { "mykey": "myvalue" }')
        assert result.success
        result = session.execute_statement('d = m.node("mykey")')
        assert result.success

        # Test key assignment fails with proper error (keys are immutable)
        result = session.execute_statement('d.key = "newkey"')
        assert not result.success
        assert "Assignment to data node key is not allowed" in str(result.error)
        assert "immutable" in str(result.error)

        # Test value assignment succeeds (values are mutable)
        result = session.execute_statement('d.value = "newvalue"')
        assert result.success
        assert "Updated data node" in str(result.value)
        
        # Verify the value was actually updated
        result = session.execute_statement('d.value')
        assert result.success
        assert str(result.value) == "newvalue"
    
    def test_list_property_assignment_parsing(self):
        """Test that list property assignment attempts parse correctly."""
        parser = ASTParser()
        
        # Test list.size = 5 parses as Assignment
        ast = parser.parse('mylist.size = 5')
        assert isinstance(ast, Assignment)
        assert isinstance(ast.target, MethodCallExpression)
        assert ast.target.method_name == "size"
        assert ast.value.value == 5
    
    def test_list_property_assignment_execution_errors(self):
        """Test that list property assignments fail with proper errors."""
        session = ExecutionSession()
        
        # Create a list
        result = session.execute_statement('list mylist = [1, 2, 3]')
        assert result.success
        
        # Test size assignment fails
        result = session.execute_statement('mylist.size = 5')
        assert not result.success
        assert "Property assignment" in str(result.error)
        assert "not supported" in str(result.error)
    
    def test_string_property_assignment_errors(self):
        """Test that string property assignments fail properly."""
        session = ExecutionSession()
        
        # Create a string
        result = session.execute_statement('string mystr = "hello"')
        assert result.success
        
        # Test length/size assignment fails
        result = session.execute_statement('mystr.size = 10')
        assert not result.success
        assert "Property assignment" in str(result.error)
        assert "not supported" in str(result.error)
    
    def test_method_calls_still_work(self):
        """Test that regular method calls (not assignments) still work correctly."""
        session = ExecutionSession()

        # Create a map and extract a data node
        result = session.execute_statement('m = { "testkey": "testvalue" }')
        assert result.success
        result = session.execute_statement('d = m.node("testkey")')
        assert result.success

        # Test method calls without parentheses work
        result = session.execute_statement('d.key')
        assert result.success
        assert str(result.value) == "testkey"

        # Test method calls with parentheses work
        result = session.execute_statement('d.key()')
        assert result.success
        assert str(result.value) == "testkey"

        # Test value method calls
        result = session.execute_statement('d.value')
        assert result.success
        assert str(result.value) == "testvalue"
    
    def test_assignment_vs_method_call_distinction(self):
        """Test that we can distinguish between assignments and method calls."""
        parser = ASTParser()
        
        # Method call should parse as MethodCall/ExpressionStatement
        method_ast = parser.parse('d.key')
        # This should be a statement containing a method call
        
        # Assignment should parse as Assignment
        assign_ast = parser.parse('d.key = "newvalue"')
        assert isinstance(assign_ast, Assignment)
        
        # They should be different types
        assert type(method_ast) != type(assign_ast)
    
    def test_complex_property_assignment_scenarios(self):
        """Test various complex scenarios for property assignments."""
        session = ExecutionSession()

        # Set up test data
        result = session.execute_statement('m = { "key": [1, 2, 3] }')
        assert result.success
        result = session.execute_statement('d = m.node("key")')
        assert result.success

        # Test nested property access assignment (should fail)
        result = session.execute_statement('d.value.size = 10')
        assert not result.success
        # This should fail because d.value returns a list, and list.size assignment is invalid
        
    def test_before_vs_after_error_messages(self):
        """Demonstrate the improvement in error messaging."""
        # This test documents the behavior change:
        # BEFORE: "Unexpected token: =" (confusing parser error)
        # AFTER: Meaningful execution-time errors with proper distinction between keys and values

        session = ExecutionSession()
        result = session.execute_statement('m = { "key": "value" }')
        assert result.success
        result = session.execute_statement('d = m.node("key")')
        assert result.success

        # Test key assignment - should fail with clear message
        result = session.execute_statement('d.key = "shizzle"')
        assert not result.success
        error_msg = str(result.error)
        assert "Assignment to data node key is not allowed" in error_msg
        assert "immutable" in error_msg
        assert "Unexpected token" not in error_msg  # No more parser errors

        # Test value assignment - should succeed (values are mutable!)
        result = session.execute_statement('d.value = "shizzle"')
        assert result.success
        assert "Updated data node" in str(result.value)
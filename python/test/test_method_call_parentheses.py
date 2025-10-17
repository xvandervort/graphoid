"""Test that both parentheses and no-parentheses method call styles work."""

import pytest
from glang.parser.ast_parser import ASTParser
from glang.execution.pipeline import ExecutionSession
from glang.ast.nodes import MethodCall, VariableRef, StringLiteral, NumberLiteral


class TestMethodCallParentheses:
    """Test that method calls work with and without parentheses."""
    
    def test_parsing_both_styles(self):
        """Test that both syntax styles parse correctly."""
        parser = ASTParser()
        
        # With parentheses
        ast1 = parser.parse('items.append("test")')
        assert isinstance(ast1, MethodCall)
        assert ast1.method_name == "append"
        assert len(ast1.arguments) == 1
        assert ast1.arguments[0].value == "test"
        
        # Without parentheses  
        ast2 = parser.parse('items.append "test"')
        assert isinstance(ast2, MethodCall)
        assert ast2.method_name == "append"
        assert len(ast2.arguments) == 1
        assert ast2.arguments[0].value == "test"
        
        # With parentheses and spaces
        ast3 = parser.parse('items.append( "test" )')
        assert isinstance(ast3, MethodCall)
        assert ast3.method_name == "append"
        assert len(ast3.arguments) == 1
        assert ast3.arguments[0].value == "test"
    
    def test_execution_both_styles(self):
        """Test that both syntax styles execute correctly."""
        session = ExecutionSession()
        
        # Set up list
        result = session.execute_statement('list items = ["a", "b"]')
        assert result.success
        
        # Test without parentheses
        result = session.execute_statement('items.append "c"')
        assert result.success
        
        # Test with parentheses
        result = session.execute_statement('items.append("d")')
        assert result.success
        
        # Test with spaces in parentheses
        result = session.execute_statement('items.append( "e" )')
        assert result.success
        
        # Verify final result
        result = session.execute_statement('items')
        assert result.success
        assert str(result.value) == "[a, b, c, d, e]"
    
    def test_multiple_arguments_with_parentheses(self):
        """Test multiple arguments only work with parentheses (for comma separation)."""
        parser = ASTParser()
        
        # Multiple args with parentheses should parse
        ast = parser.parse('obj.method("arg1", 42, true)')
        assert isinstance(ast, MethodCall)
        assert len(ast.arguments) == 3
        
        # Single arg without parentheses should parse
        ast = parser.parse('obj.method "single_arg"')
        assert isinstance(ast, MethodCall)
        assert len(ast.arguments) == 1
    
    def test_empty_parentheses(self):
        """Test empty parentheses for zero-argument methods."""
        parser = ASTParser()
        
        ast = parser.parse('obj.method()')
        assert isinstance(ast, MethodCall)
        assert len(ast.arguments) == 0
    
    def test_nested_method_calls_with_parentheses(self):
        """Test nested method calls with parentheses."""
        parser = ASTParser()
        
        # Test chained calls
        ast = parser.parse('items[0].toString()')
        assert isinstance(ast, MethodCall)
        
        # Test method call as argument
        ast = parser.parse('obj.method(other.getValue())')
        assert isinstance(ast, MethodCall)
        assert len(ast.arguments) == 1
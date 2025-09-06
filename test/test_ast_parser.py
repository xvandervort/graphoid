"""Tests for the AST parser."""

import pytest
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../src'))

from glang.parser.ast_parser import ASTParser, ParseError
from glang.ast.nodes import *

class TestASTParser:
    """Test AST parser functionality."""
    
    def setup_method(self):
        self.parser = ASTParser()
    
    def test_parse_variable_declaration(self):
        """Test parsing variable declarations."""
        # Basic string declaration
        ast = self.parser.parse('string name = "hello"')
        
        assert isinstance(ast, VariableDeclaration)
        assert ast.var_type == "string"
        assert ast.name == "name"
        assert isinstance(ast.initializer, StringLiteral)
        assert ast.initializer.value == "hello"
        assert ast.type_constraint is None
        
        # Number declaration
        ast = self.parser.parse('num count = 42')
        assert isinstance(ast, VariableDeclaration)
        assert ast.var_type == "num"
        assert ast.name == "count"
        assert isinstance(ast.initializer, NumberLiteral)
        assert ast.initializer.value == 42
        
        # Boolean declaration
        ast = self.parser.parse('bool flag = true')
        assert isinstance(ast, VariableDeclaration)
        assert ast.var_type == "bool"
        assert isinstance(ast.initializer, BooleanLiteral)
        assert ast.initializer.value == True
    
    def test_parse_list_declaration(self):
        """Test parsing list declarations."""
        # Simple list
        ast = self.parser.parse('list items = ["apple", "banana"]')
        
        assert isinstance(ast, VariableDeclaration)
        assert ast.var_type == "list"
        assert ast.name == "items"
        assert isinstance(ast.initializer, ListLiteral)
        assert len(ast.initializer.elements) == 2
        assert ast.initializer.elements[0].value == "apple"
        assert ast.initializer.elements[1].value == "banana"
        
        # Empty list
        ast = self.parser.parse('list empty = []')
        assert isinstance(ast.initializer, ListLiteral)
        assert len(ast.initializer.elements) == 0
        
        # Mixed types
        ast = self.parser.parse('list mixed = [1, "hello", true]')
        elements = ast.initializer.elements
        assert isinstance(elements[0], NumberLiteral)
        assert isinstance(elements[1], StringLiteral)
        assert isinstance(elements[2], BooleanLiteral)
    
    def test_parse_constrained_declaration(self):
        """Test parsing type-constrained declarations."""
        ast = self.parser.parse('list<num> numbers = [1, 2, 3]')
        
        assert isinstance(ast, VariableDeclaration)
        assert ast.var_type == "list"
        assert ast.type_constraint == "num"
        assert isinstance(ast.initializer, ListLiteral)
        
        # Other constraint types
        ast = self.parser.parse('list<string> names = ["alice", "bob"]')
        assert ast.type_constraint == "string"
        
        ast = self.parser.parse('list<bool> flags = [true, false]')
        assert ast.type_constraint == "bool"
    
    def test_parse_variable_access(self):
        """Test parsing variable access expressions."""
        ast = self.parser.parse('myvar')
        
        assert isinstance(ast, ExpressionStatement)
        assert isinstance(ast.expression, VariableRef)
        assert ast.expression.name == "myvar"
    
    def test_parse_method_call(self):
        """Test parsing method calls."""
        # Method call with string argument
        ast = self.parser.parse('obj.append("value")')
        
        assert isinstance(ast, MethodCall)
        assert isinstance(ast.target, VariableRef)
        assert ast.target.name == "obj"
        assert ast.method_name == "append"
        assert len(ast.arguments) == 1
        assert isinstance(ast.arguments[0], StringLiteral)
        assert ast.arguments[0].value == "value"
        
        # Method call with multiple arguments
        ast = self.parser.parse('obj.method("str", 42, true)')
        assert len(ast.arguments) == 3
        assert isinstance(ast.arguments[0], StringLiteral)
        assert isinstance(ast.arguments[1], NumberLiteral) 
        assert isinstance(ast.arguments[2], BooleanLiteral)
        
        # Method call without parentheses
        ast = self.parser.parse('list.append value')
        assert isinstance(ast, MethodCall)
        assert ast.method_name == "append"
        assert len(ast.arguments) == 1
        assert isinstance(ast.arguments[0], VariableRef)
        assert ast.arguments[0].name == "value"
    
    def test_parse_index_access(self):
        """Test parsing index access expressions."""
        # Simple index
        ast = self.parser.parse('arr[0]')
        
        assert isinstance(ast, ExpressionStatement)
        assert isinstance(ast.expression, IndexAccess)
        assert isinstance(ast.expression.target, VariableRef)
        assert ast.expression.target.name == "arr"
        assert len(ast.expression.indices) == 1
        assert ast.expression.indices[0].value == 0
        
        # Negative index
        ast = self.parser.parse('arr[-1]')
        expr = ast.expression
        assert expr.indices[0].value == -1
        
        # Chained indexing
        ast = self.parser.parse('matrix[1][2]')
        expr = ast.expression
        
        # Should be IndexAccess(IndexAccess(matrix, [1]), [2])
        assert isinstance(expr, IndexAccess)
        assert isinstance(expr.target, IndexAccess)
        assert expr.target.target.name == "matrix"
        assert expr.target.indices[0].value == 1
        assert expr.indices[0].value == 2
    
    def test_parse_slice_access(self):
        """Test parsing slice access expressions."""
        # Full slice
        ast = self.parser.parse('arr[1:3:2]')
        
        assert isinstance(ast, ExpressionStatement)
        assert isinstance(ast.expression, SliceAccess)
        expr = ast.expression
        assert isinstance(expr.target, VariableRef)
        assert expr.start.value == 1
        assert expr.stop.value == 3
        assert expr.step.value == 2
        
        # Partial slices
        ast = self.parser.parse('arr[1:]')
        expr = ast.expression
        assert expr.start.value == 1
        assert expr.stop is None
        assert expr.step is None
        
        ast = self.parser.parse('arr[:3]')
        expr = ast.expression
        assert expr.start is None
        assert expr.stop.value == 3
        
        ast = self.parser.parse('arr[::2]')
        expr = ast.expression
        assert expr.start is None
        assert expr.stop is None
        assert expr.step.value == 2
    
    def test_parse_index_assignment(self):
        """Test parsing index assignments."""
        ast = self.parser.parse('arr[0] = "new_value"')
        
        assert isinstance(ast, IndexAssignment)
        assert isinstance(ast.target, IndexAccess)
        assert ast.target.target.name == "arr"
        assert ast.target.indices[0].value == 0
        assert isinstance(ast.value, StringLiteral)
        assert ast.value.value == "new_value"
        
        # Chained index assignment
        ast = self.parser.parse('matrix[1][2] = 42')
        assert isinstance(ast, IndexAssignment)
        assert isinstance(ast.target, IndexAccess)
        # Target should be matrix[1][2] structure
        assert isinstance(ast.target.target, IndexAccess)
    
    def test_parse_slice_assignment(self):
        """Test parsing slice assignments."""
        ast = self.parser.parse('arr[1:3] = ["a", "b"]')
        
        assert isinstance(ast, SliceAssignment)
        assert isinstance(ast.target, SliceAccess)
        assert ast.target.start.value == 1
        assert ast.target.stop.value == 3
        assert isinstance(ast.value, ListLiteral)
        assert len(ast.value.elements) == 2
    
    def test_complex_expressions(self):
        """Test parsing complex nested expressions."""
        # Method call with index access argument
        ast = self.parser.parse('dest.append(source[0])')
        
        assert isinstance(ast, MethodCall)
        assert ast.method_name == "append"
        assert len(ast.arguments) == 1
        assert isinstance(ast.arguments[0], IndexAccess)
        assert ast.arguments[0].target.name == "source"
        
        # Chained method calls
        ast = self.parser.parse('obj.method1().method2("arg")')
        # This would require return value handling - for now just test single method
        
        # Variable declaration with complex initializer
        ast = self.parser.parse('list matrix = [[1, 2], [3, 4]]')
        assert isinstance(ast, VariableDeclaration)
        list_init = ast.initializer
        assert len(list_init.elements) == 2
        assert isinstance(list_init.elements[0], ListLiteral)
        assert len(list_init.elements[0].elements) == 2
    
    def test_error_handling(self):
        """Test parser error handling."""
        # Invalid syntax - these should still fail
        with pytest.raises(ParseError):
            self.parser.parse('string name')  # Missing = and initializer
        
        with pytest.raises(ParseError):
            self.parser.parse('invalid_type name = "value"')  # Invalid type
        
        # Unmatched brackets
        with pytest.raises(ParseError):
            self.parser.parse('arr[0')  # Missing ]
        
        with pytest.raises(ParseError):
            self.parser.parse('list items = [1, 2')  # Unmatched [
    
    def test_position_information(self):
        """Test that AST nodes have position information."""
        ast = self.parser.parse('string name = "hello"')
        
        assert ast.position is not None
        assert ast.position.line == 1
        assert ast.position.column == 1
        
        # Check that expressions also have position info
        assert ast.initializer.position is not None
    
    def test_whitespace_and_formatting(self):
        """Test that parser handles various whitespace and formatting."""
        # Extra whitespace
        ast = self.parser.parse('  string   name  =  "hello"  ')
        assert isinstance(ast, VariableDeclaration)
        assert ast.name == "name"
        
        # No spaces around operators
        ast = self.parser.parse('string name="hello"')
        assert isinstance(ast, VariableDeclaration)
        assert ast.name == "name"
        
        # Method calls with various spacing
        ast = self.parser.parse('obj . method ( "arg" )')
        assert isinstance(ast, MethodCall)
        assert ast.method_name == "method"
    
    def test_expression_vs_statement_distinction(self):
        """Test proper distinction between expressions and statements."""
        # Expression statement (variable access)
        ast = self.parser.parse('myvar')
        assert isinstance(ast, ExpressionStatement)
        assert isinstance(ast.expression, VariableRef)
        
        # Variable declaration statement  
        ast = self.parser.parse('string x = "value"')
        assert isinstance(ast, VariableDeclaration)
        
        # Method call statement
        ast = self.parser.parse('obj.method("arg")')
        assert isinstance(ast, MethodCall)
        
        # Assignment statement
        ast = self.parser.parse('arr[0] = "value"')
        assert isinstance(ast, IndexAssignment)

class TestParserIntegration:
    """Integration tests for parser with complex scenarios."""
    
    def setup_method(self):
        self.parser = ASTParser()
    
    def test_realistic_variable_declarations(self):
        """Test realistic variable declaration scenarios."""
        # Complex list with mixed types
        code = 'list<string> names = ["Alice", "Bob", "Charlie"]'
        ast = self.parser.parse(code)
        
        assert isinstance(ast, VariableDeclaration)
        assert ast.var_type == "list"
        assert ast.type_constraint == "string"
        assert len(ast.initializer.elements) == 3
        
        # Nested list
        code = 'list matrix = [[1, 2], [3, 4], [5, 6]]'
        ast = self.parser.parse(code)
        
        matrix_list = ast.initializer
        assert len(matrix_list.elements) == 3
        for row in matrix_list.elements:
            assert isinstance(row, ListLiteral)
            assert len(row.elements) == 2
    
    def test_realistic_method_calls(self):
        """Test realistic method call scenarios."""
        # Method call with variable argument
        code = 'list.append item'
        ast = self.parser.parse(code)
        
        assert isinstance(ast, MethodCall)
        assert ast.method_name == "append"
        assert isinstance(ast.arguments[0], VariableRef)
        assert ast.arguments[0].name == "item"
        
        # Method call with index access argument
        code = 'dest.append(source[0])'
        ast = self.parser.parse(code)
        
        assert len(ast.arguments) == 1
        assert isinstance(ast.arguments[0], IndexAccess)
    
    def test_complex_indexing_scenarios(self):
        """Test complex indexing scenarios."""
        # Multi-dimensional with assignment
        code = 'matrix[0][1] = 99'
        ast = self.parser.parse(code)
        
        assert isinstance(ast, IndexAssignment)
        target = ast.target
        assert isinstance(target, IndexAccess)
        assert isinstance(target.target, IndexAccess)
        
        # Slice with assignment
        code = 'arr[1:3] = [10, 20]'
        ast = self.parser.parse(code)
        
        assert isinstance(ast, SliceAssignment)
        assert isinstance(ast.target, SliceAccess)
        assert isinstance(ast.value, ListLiteral)


if __name__ == '__main__':
    pytest.main([__file__])
"""Test assignment detection with various expression types."""

import pytest
from glang.parser.ast_parser import ASTParser
from glang.ast.nodes import (
    Assignment, MethodCallExpression, MethodCall, IndexAccess, 
    VariableRef, StringLiteral, NumberLiteral
)


class TestAssignmentDetection:
    """Test that assignment detection works uniformly across expression types."""
    
    def test_variable_assignment_detection(self):
        """Test basic variable assignment detection."""
        parser = ASTParser()
        
        ast = parser.parse('x = 5')
        assert isinstance(ast, Assignment)
        assert isinstance(ast.target, VariableRef)
        assert ast.target.name == "x"
        assert ast.value.value == 5
    
    def test_index_assignment_detection(self):
        """Test index assignment detection."""
        parser = ASTParser()
        
        ast = parser.parse('arr[0] = "value"')
        # Index assignments are a special case with their own AST node type
        from glang.ast.nodes import IndexAssignment
        assert isinstance(ast, IndexAssignment)
        assert isinstance(ast.target, IndexAccess)
        assert isinstance(ast.target.target, VariableRef)
        assert ast.target.target.name == "arr"
        assert ast.value.value == "value"
    
    def test_method_call_assignment_detection(self):
        """Test method call assignment detection (the new functionality)."""
        parser = ASTParser()
        
        # Method call without parentheses
        ast = parser.parse('obj.prop = "value"')
        assert isinstance(ast, Assignment)
        assert isinstance(ast.target, MethodCallExpression)
        assert ast.target.method_name == "prop"
        assert isinstance(ast.target.target, VariableRef)
        assert ast.target.target.name == "obj"
        assert ast.value.value == "value"
        
        # Method call with parentheses (though this would be unusual)
        ast = parser.parse('obj.prop() = "value"')
        assert isinstance(ast, Assignment)
        assert isinstance(ast.target, MethodCallExpression)
        assert ast.target.method_name == "prop"
        assert len(ast.target.arguments) == 0
    
    def test_complex_assignment_targets(self):
        """Test assignment detection with complex targets.""" 
        parser = ASTParser()
        
        # Method call on indexed element assignment
        ast = parser.parse('arr[0].prop = "value"')
        assert isinstance(ast, Assignment)
        assert isinstance(ast.target, MethodCallExpression)
        assert ast.target.method_name == "prop"
        # The target should be an index access
        assert isinstance(ast.target.target, IndexAccess)
    
    def test_assignment_vs_expression_distinction(self):
        """Test that we can distinguish assignments from expressions."""
        parser = ASTParser()
        
        # Expression (no assignment)
        expr_ast = parser.parse('obj.method')
        assert isinstance(expr_ast, MethodCall)
        assert not isinstance(expr_ast, Assignment)
        
        # Assignment
        assign_ast = parser.parse('obj.method = "value"')
        assert isinstance(assign_ast, Assignment)
        assert isinstance(assign_ast.target, MethodCallExpression)
    
    def test_assignment_with_complex_values(self):
        """Test assignment detection with complex right-hand side values."""
        parser = ASTParser()
        
        # Assignment with method call value
        ast = parser.parse('obj.prop = other.getValue()')
        assert isinstance(ast, Assignment)
        assert isinstance(ast.target, MethodCallExpression)
        assert ast.target.method_name == "prop"
        assert isinstance(ast.value, MethodCallExpression)
        assert ast.value.method_name == "getValue"
        
        # Assignment with list literal
        ast = parser.parse('obj.prop = [1, 2, 3]')
        assert isinstance(ast, Assignment)
        assert isinstance(ast.target, MethodCallExpression)
        # Value should be a list literal
        assert hasattr(ast.value, 'elements')
        assert len(ast.value.elements) == 3
    
    def test_no_false_positive_assignments(self):
        """Test that non-assignment expressions aren't detected as assignments."""
        parser = ASTParser()
        
        # Method call with arguments should not be assignment
        ast = parser.parse('obj.method("arg")')
        assert not isinstance(ast, Assignment)
        
        # Comparison should not be assignment
        ast = parser.parse('x == 5')
        assert not isinstance(ast, Assignment)
        
        # Binary operation should not be assignment
        ast = parser.parse('a + b')
        assert not isinstance(ast, Assignment)
    
    def test_assignment_operator_precedence(self):
        """Test that assignment has the correct precedence."""
        parser = ASTParser()
        
        # Assignment should bind less tightly than arithmetic
        ast = parser.parse('x = a + b')
        assert isinstance(ast, Assignment)
        assert isinstance(ast.target, VariableRef)
        assert ast.target.name == "x"
        # The value should be the binary operation
        assert hasattr(ast.value, 'left')  # Binary operation has left/right
        assert hasattr(ast.value, 'right')
    
    def test_multiple_assignment_styles_consistency(self):
        """Test that all assignment styles are handled consistently."""
        parser = ASTParser()
        from glang.ast.nodes import IndexAssignment
        
        # Variable assignment
        ast = parser.parse('variable = 5')
        assert isinstance(ast, Assignment)
        assert isinstance(ast.target, VariableRef)
        assert ast.value.value == 5
        
        # Index assignment (special case)
        ast = parser.parse('arr[0] = 5')
        assert isinstance(ast, IndexAssignment)
        assert isinstance(ast.target, IndexAccess)
        assert ast.value.value == 5
        
        # Method call assignment (new functionality)
        ast = parser.parse('obj.prop = 5')
        assert isinstance(ast, Assignment)
        assert isinstance(ast.target, MethodCallExpression)
        assert ast.value.value == 5
    
    def test_left_to_right_parsing_order(self):
        """Test that parsing follows left-to-right evaluation order."""
        parser = ASTParser()
        
        # This should parse the left side first (as a valid expression)
        # Then detect the assignment, then parse the right side
        ast = parser.parse('obj.property = other.getValue()')
        
        assert isinstance(ast, Assignment)
        
        # Left side should be parsed as method call expression
        assert isinstance(ast.target, MethodCallExpression)
        assert ast.target.method_name == "property"
        assert isinstance(ast.target.target, VariableRef)
        assert ast.target.target.name == "obj"
        
        # Right side should be parsed as method call expression
        assert isinstance(ast.value, MethodCallExpression)
        assert ast.value.method_name == "getValue"
        assert isinstance(ast.value.target, VariableRef)
        assert ast.value.target.name == "other"
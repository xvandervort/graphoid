"""Tests for control flow structures: if/else, while, for-in, break/continue."""

import pytest
from glang.execution.pipeline import ExecutionSession
from glang.parser.ast_parser import ASTParser
from glang.ast.nodes import *


class TestControlFlowParsing:
    """Test that control flow structures parse correctly."""
    
    def test_parse_if_statement(self):
        """Test parsing of if statements."""
        parser = ASTParser()
        
        # Simple if statement
        ast = parser.parse('if true { a = 1 }')
        assert isinstance(ast, IfStatement)
        assert hasattr(ast, 'condition')
        assert hasattr(ast, 'then_block')
        assert ast.else_block is None
    
    def test_parse_if_else_statement(self):
        """Test parsing of if-else statements."""
        parser = ASTParser()
        
        # If-else statement
        ast = parser.parse('if false { a = 1 } else { a = 2 }')
        assert isinstance(ast, IfStatement)
        assert hasattr(ast, 'condition')
        assert hasattr(ast, 'then_block')
        assert ast.else_block is not None
    
    def test_parse_while_statement(self):
        """Test parsing of while statements."""
        parser = ASTParser()
        
        # While loop
        ast = parser.parse('while true { a = 1 }')
        assert isinstance(ast, WhileStatement)
        assert hasattr(ast, 'condition')
        assert hasattr(ast, 'body')
    
    def test_parse_for_in_statement(self):
        """Test parsing of for-in statements."""
        parser = ASTParser()
        
        # For-in loop
        ast = parser.parse('for item in items { print(item) }')
        assert isinstance(ast, ForInStatement)
        assert hasattr(ast, 'variable')
        assert hasattr(ast, 'iterable')
        assert hasattr(ast, 'body')
        assert ast.variable == "item"
    
    def test_parse_break_statement(self):
        """Test parsing of break statements."""
        parser = ASTParser()
        
        ast = parser.parse('break')
        assert isinstance(ast, BreakStatement)
    
    def test_parse_continue_statement(self):
        """Test parsing of continue statements."""
        parser = ASTParser()
        
        ast = parser.parse('continue')
        assert isinstance(ast, ContinueStatement)
    
    def test_parse_block_with_multiple_statements(self):
        """Test parsing blocks with multiple statements."""
        parser = ASTParser()
        
        ast = parser.parse('if true { a = 1; b = 2; c = 3 }')
        assert isinstance(ast, IfStatement)
        assert len(ast.then_block.statements) == 3


class TestControlFlowBasicExecution:
    """Test basic execution of control flow structures."""
    
    def test_if_statement_true_condition(self):
        """Test if statement with true condition."""
        session = ExecutionSession()
        
        result = session.execute_statement('if true { x = 42 }')
        assert result.success
        
        # Check that x was set
        check_result = session.execute_statement('x')
        assert check_result.success
        assert check_result.value.value == 42
    
    def test_if_statement_false_condition(self):
        """Test if statement with false condition."""
        session = ExecutionSession()
        session.execute_statement('x = 0')
        
        result = session.execute_statement('if false { x = 42 }')
        assert result.success
        
        # Check that x was not changed
        check_result = session.execute_statement('x')
        assert check_result.success
        assert check_result.value.value == 0
    
    def test_if_else_statement(self):
        """Test if-else statement execution."""
        session = ExecutionSession()
        
        # True condition - should execute then block
        result = session.execute_statement('if true { x = 1 } else { x = 2 }')
        assert result.success
        
        check_result = session.execute_statement('x')
        assert check_result.success
        assert check_result.value.value == 1
        
        # False condition - should execute else block
        result = session.execute_statement('if false { y = 1 } else { y = 2 }')
        assert result.success
        
        check_result = session.execute_statement('y')
        assert check_result.success
        assert check_result.value.value == 2
    
    def test_while_loop_basic(self):
        """Test basic while loop execution."""
        session = ExecutionSession()
        session.execute_statement('counter = 0')
        session.execute_statement('max = 3')
        
        result = session.execute_statement('while counter < max { counter = counter + 1 }')
        assert result.success
        
        # Check final counter value
        check_result = session.execute_statement('counter')
        assert check_result.success
        assert check_result.value.value == 3
    
    def test_for_in_loop_with_list(self):
        """Test for-in loop with list."""
        session = ExecutionSession()
        session.execute_statement('items = [1, 2, 3]')
        session.execute_statement('sum = 0')
        
        result = session.execute_statement('for item in items { sum = sum + item }')
        assert result.success
        
        # Check final sum
        check_result = session.execute_statement('sum')
        assert check_result.success
        assert check_result.value.value == 6


class TestControlFlowNesting:
    """Test nested control flow structures."""
    
    def test_nested_if_statements(self):
        """Test nested if statements."""
        session = ExecutionSession()
        
        result = session.execute_statement('''
        if true {
            if true {
                x = 42
            }
        }
        ''')
        assert result.success
        
        check_result = session.execute_statement('x')
        assert check_result.success
        assert check_result.value.value == 42
    
    def test_if_inside_while(self):
        """Test if statement inside while loop."""
        session = ExecutionSession()
        session.execute_statement('i = 0')
        session.execute_statement('even_sum = 0')
        
        result = session.execute_statement('''
        while i < 5 {
            if i % 2 == 0 {
                even_sum = even_sum + i
            }
            i = i + 1
        }
        ''')
        assert result.success
        
        check_result = session.execute_statement('even_sum')
        assert check_result.success
        assert check_result.value.value == 6  # 0 + 2 + 4
"""Test comment parsing functionality."""

import pytest
from glang.parser.ast_parser import ASTParser
from glang.ast.nodes import NoOp
from glang.semantic.pipeline import SemanticSession
from glang.execution.pipeline import ExecutionSession


class TestCommentParsing:
    """Test that comments are handled gracefully."""
    
    def test_comment_only_line(self):
        """Test that comment-only lines return NoOp."""
        parser = ASTParser()
        
        # Single comment line
        ast = parser.parse("# This is a comment")
        assert isinstance(ast, NoOp)
        
        # Comment with spaces
        ast = parser.parse("  # This is a comment with spaces")
        assert isinstance(ast, NoOp)
        
        # Empty line (just newline)
        ast = parser.parse("")
        assert isinstance(ast, NoOp)
    
    def test_semantic_analysis_of_comments(self):
        """Test that NoOp statements pass semantic analysis."""
        session = SemanticSession()
        
        # Comment-only line should analyze successfully
        result = session.analyze_statement("# This is a comment")
        assert result.success
        assert isinstance(result.ast, NoOp)
        
        # Empty line should also work
        result = session.analyze_statement("")
        assert result.success
        assert isinstance(result.ast, NoOp)
    
    def test_execution_of_comments(self):
        """Test that NoOp statements execute without error."""
        session = ExecutionSession()
        
        # Comment-only line should execute successfully
        result = session.execute_statement("# This is a comment")
        assert result.success
        assert result.value is None  # NoOp returns None
        
        # Empty line should also work
        result = session.execute_statement("")
        assert result.success
        assert result.value is None
    
    def test_mixed_content_with_comments(self):
        """Test that we can mix comments with real statements."""
        session = ExecutionSession()
        
        # Execute a comment
        result = session.execute_statement("# This is a comment")
        assert result.success
        
        # Execute a real statement
        result = session.execute_statement('string name = "Alice"')
        assert result.success
        
        # Execute another comment
        result = session.execute_statement("# Another comment")
        assert result.success
        
        # Access the variable
        result = session.execute_statement("name")
        assert result.success
        assert str(result.value) == "Alice"
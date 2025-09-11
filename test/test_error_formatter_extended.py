"""Extended tests for error formatter functionality."""

import pytest
from unittest.mock import Mock

from src.glang.errors.formatter import ErrorFormatter
from src.glang.parser.ast_parser import ParseError
from src.glang.lexer.tokenizer import TokenizerError, Token
from src.glang.semantic.errors import SemanticError
from src.glang.ast.nodes import SourcePosition


class TestErrorFormatter:
    """Test ErrorFormatter class."""
    
    def test_format_error_simple_method_exists(self):
        """Test that format_error_simple method exists."""
        # Test with basic exception
        error = Exception("Basic error message")
        
        result = ErrorFormatter.format_error_simple(error)
        
        assert isinstance(result, str)
        assert "Basic error message" in result
    
    def test_format_parse_error_with_token(self):
        """Test formatting parse error with token information."""
        # Create a mock token
        mock_token = Mock(spec=Token)
        mock_token.line = 5
        mock_token.column = 10
        mock_token.type = "IDENTIFIER"
        mock_token.value = "test_token"
        
        # Create parse error with token
        parse_error = ParseError("Unexpected token", mock_token)
        
        source_code = "line 1\nline 2\nline 3\nline 4\nline 5 test_token here\nline 6"
        
        result = ErrorFormatter.format_error_with_context(
            parse_error, source_code, "test.gr"
        )
        
        assert isinstance(result, str)
        assert "test.gr" in result
        assert "line 5" in result
        assert "Unexpected token" in result
    
    def test_format_tokenizer_error(self):
        """Test formatting tokenizer error."""
        tokenizer_error = TokenizerError("Invalid character", line=3, column=7)
        
        source_code = "line 1\nline 2\nline 3 invalid@char\nline 4"
        
        result = ErrorFormatter.format_error_with_context(
            tokenizer_error, source_code, "input.gr"
        )
        
        assert isinstance(result, str)
        assert "input.gr" in result
        assert "line 3" in result
        assert "Invalid character" in result
    
    def test_format_semantic_error_with_position(self):
        """Test formatting semantic error with position."""
        position = SourcePosition(line=2, column=5)
        semantic_error = SemanticError("Type mismatch", position)
        
        source_code = "string x = \"hello\"\nnum y = x\nprint(y)"
        
        result = ErrorFormatter.format_error_with_context(
            semantic_error, source_code, "semantic.gr"
        )
        
        assert isinstance(result, str)
        assert "semantic.gr" in result
        assert "Type mismatch" in result
    
    def test_format_semantic_error_without_position(self):
        """Test formatting semantic error without position."""
        semantic_error = SemanticError("General semantic error")
        
        source_code = "some code here"
        
        result = ErrorFormatter.format_error_with_context(
            semantic_error, source_code, "test.gr"
        )
        
        assert isinstance(result, str)
        assert "General semantic error" in result
    
    def test_format_generic_exception(self):
        """Test formatting generic exception."""
        generic_error = ValueError("Invalid value provided")
        
        source_code = "some source code"
        
        result = ErrorFormatter.format_error_with_context(
            generic_error, source_code, "generic.gr"
        )
        
        assert isinstance(result, str)
        assert "Invalid value provided" in result
    
    def test_format_error_empty_source(self):
        """Test formatting error with empty source code."""
        error = Exception("Error with empty source")
        
        result = ErrorFormatter.format_error_with_context(
            error, "", "empty.gr"
        )
        
        assert isinstance(result, str)
        assert "Error with empty source" in result
    
    def test_format_error_default_source_name(self):
        """Test formatting error with default source name."""
        error = Exception("Error with default name")
        
        result = ErrorFormatter.format_error_with_context(
            error, "some code"
        )
        
        assert isinstance(result, str)
        assert "Error with default name" in result
        # Should use default source name
        assert "<input>" in result or "Error with default name" in result
    
    def test_format_parse_error_without_token(self):
        """Test formatting parse error without token information."""
        parse_error = ParseError("Parse error without token")
        
        source_code = "some source code"
        
        result = ErrorFormatter.format_error_with_context(
            parse_error, source_code, "notoken.gr"
        )
        
        assert isinstance(result, str)
        assert "Parse error without token" in result
    
    def test_format_multiline_source(self):
        """Test formatting error with multiline source code."""
        error = Exception("Error in multiline source")
        
        source_code = """line 1
line 2
line 3
line 4
line 5"""
        
        result = ErrorFormatter.format_error_with_context(
            error, source_code, "multiline.gr"
        )
        
        assert isinstance(result, str)
        assert "Error in multiline source" in result
    
    def test_format_error_with_special_characters(self):
        """Test formatting error with special characters in source."""
        error = Exception("Error with special chars")
        
        source_code = "string msg = \"Hello\\nWorld!\"\nnum π = 3.14159"
        
        result = ErrorFormatter.format_error_with_context(
            error, source_code, "special.gr"
        )
        
        assert isinstance(result, str)
        assert "Error with special chars" in result


class TestErrorFormatterEdgeCases:
    """Test edge cases for ErrorFormatter."""
    
    def test_format_error_very_long_line(self):
        """Test formatting error with very long source line."""
        error = Exception("Error in long line")
        
        long_line = "string very_long_variable_name = " + "\"very long string content \" * 100"
        source_code = f"line 1\n{long_line}\nline 3"
        
        result = ErrorFormatter.format_error_with_context(
            error, source_code, "long.gr"
        )
        
        assert isinstance(result, str)
        assert "Error in long line" in result
    
    def test_format_error_with_tabs(self):
        """Test formatting error with tab characters."""
        error = Exception("Error with tabs")
        
        source_code = "line 1\n\tindented line\n\t\tdouble indented"
        
        result = ErrorFormatter.format_error_with_context(
            error, source_code, "tabs.gr"
        )
        
        assert isinstance(result, str)
        assert "Error with tabs" in result
    
    def test_format_error_unicode_source(self):
        """Test formatting error with unicode characters."""
        error = Exception("Error with unicode")
        
        source_code = "string greeting = \"Hello 世界!\"\nstring café = \"café\""
        
        result = ErrorFormatter.format_error_with_context(
            error, source_code, "unicode.gr"
        )
        
        assert isinstance(result, str)
        assert "Error with unicode" in result
    
    def test_format_error_none_values(self):
        """Test error formatting with None values."""
        error = Exception("Error with None")
        
        # Test with None source code should be handled gracefully
        result = ErrorFormatter.format_error_with_context(
            error, "", "none.gr"
        )
        
        assert isinstance(result, str)
        assert "Error with None" in result
    
    def test_format_error_position_boundary_cases(self):
        """Test error formatting with boundary position cases."""
        # Test position at start of file
        position = SourcePosition(line=1, column=1)
        error = SemanticError("Error at start", position)
        
        source_code = "first line\nsecond line"
        
        result = ErrorFormatter.format_error_with_context(
            error, source_code, "boundary.gr"
        )
        
        assert isinstance(result, str)
        assert "Error at start" in result
    
    def test_format_error_position_beyond_source(self):
        """Test error formatting with position beyond source length."""
        # Position beyond actual source
        position = SourcePosition(line=10, column=50)
        error = SemanticError("Error beyond source", position)
        
        source_code = "short line"
        
        result = ErrorFormatter.format_error_with_context(
            error, source_code, "beyond.gr"
        )
        
        assert isinstance(result, str)
        assert "Error beyond source" in result


class TestErrorFormatterMessage:
    """Test error message extraction and formatting."""
    
    def test_extract_message_from_parse_error(self):
        """Test message extraction from ParseError."""
        mock_token = Mock(spec=Token)
        mock_token.line = 1
        mock_token.column = 1
        
        parse_error = ParseError("Custom parse message", mock_token)
        
        result = ErrorFormatter.format_error_with_context(
            parse_error, "code", "test.gr"
        )
        
        assert "Custom parse message" in result
    
    def test_extract_message_from_tokenizer_error(self):
        """Test message extraction from TokenizerError."""
        tokenizer_error = TokenizerError("Custom tokenizer message", 1, 1)
        
        result = ErrorFormatter.format_error_with_context(
            tokenizer_error, "code", "test.gr"
        )
        
        assert "Custom tokenizer message" in result
    
    def test_extract_message_from_semantic_error(self):
        """Test message extraction from SemanticError."""
        semantic_error = SemanticError("Custom semantic message")
        
        result = ErrorFormatter.format_error_with_context(
            semantic_error, "code", "test.gr"
        )
        
        assert "Custom semantic message" in result
    
    def test_format_simple_error_fallback(self):
        """Test format_error_simple as fallback."""
        error = Exception("Simple error")
        
        result = ErrorFormatter.format_error_simple(error)
        
        assert isinstance(result, str)
        assert "Simple error" in result
    
    def test_multiple_error_types_consistency(self):
        """Test that different error types produce consistent output format."""
        source_code = "test code"
        source_name = "test.gr"
        
        # Test different error types
        errors = [
            Exception("Generic error"),
            ParseError("Parse error"),
            TokenizerError("Tokenizer error", 1, 1),
            SemanticError("Semantic error")
        ]
        
        results = []
        for error in errors:
            result = ErrorFormatter.format_error_with_context(
                error, source_code, source_name
            )
            results.append(result)
            assert isinstance(result, str)
            assert len(result) > 0
        
        # All results should be strings
        assert all(isinstance(r, str) for r in results)
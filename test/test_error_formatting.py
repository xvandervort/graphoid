"""Test enhanced error message formatting."""

import pytest
from glang.errors import ErrorFormatter
from glang.parser.ast_parser import ParseError
from glang.lexer.tokenizer import TokenizerError, Token, TokenType
from glang.semantic.errors import SemanticError, UndefinedVariableError
from glang.ast.nodes import SourcePosition


class TestErrorFormatter:
    """Test enhanced error message formatting."""
    
    def test_parse_error_formatting(self):
        """Test formatting of parse errors with context."""
        source = "string test = \"hello\"\ninvalid syntax here!\nnum x = 42"
        token = Token(TokenType.IDENTIFIER, "syntax", 2, 8)
        error = ParseError("Unexpected token", token)
        
        result = ErrorFormatter.format_error_with_context(error, source, "test.gr")
        
        expected_lines = [
            "Error in test.gr at line 2, column 8:",
            "  invalid syntax here!",
            "  ~~~~~~~^",
            "Unexpected token"
        ]
        
        assert result == "\n".join(expected_lines)
    
    def test_tokenizer_error_formatting(self):
        """Test formatting of tokenizer errors."""
        source = "string name = \"unclosed string\nnum x = 42"
        error = TokenizerError("Unterminated string literal", 1, 15)
        
        result = ErrorFormatter.format_error_with_context(error, source, "<input>")
        
        expected_lines = [
            "Error in <input> at line 1, column 15:",
            "  string name = \"unclosed string",
            "  ~~~~~~~~~~~~~~^",
            "Unterminated string literal"
        ]
        
        assert result == "\n".join(expected_lines)
    
    def test_semantic_error_formatting(self):
        """Test formatting of semantic errors."""
        source = "undefined_variable"
        position = SourcePosition(1, 1)
        error = UndefinedVariableError("undefined_variable", position)
        
        result = ErrorFormatter.format_error_with_context(error, source, "<input>")
        
        expected_lines = [
            "Error in <input> at line 1, column 1:",
            "  undefined_variable",
            "  ^",
            f"Undefined variable 'undefined_variable' at line 1, column 1"
        ]
        
        assert result == "\n".join(expected_lines)
    
    def test_error_without_position(self):
        """Test formatting errors without position information."""
        error = Exception("Generic error message")
        source = "some code here"
        
        result = ErrorFormatter.format_error_with_context(error, source, "test.gr")
        
        assert result == "Error: Generic error message"
    
    def test_invalid_line_number(self):
        """Test handling of invalid line numbers."""
        source = "line 1\nline 2"
        token = Token(TokenType.IDENTIFIER, "test", 10, 5)  # Invalid line number
        error = ParseError("Test error", token)
        
        result = ErrorFormatter.format_error_with_context(error, source, "test.gr")
        
        assert result == "Error at line 10, column 5: Test error"
    
    def test_multiple_errors_formatting(self):
        """Test formatting multiple errors."""
        source = "string x = unknown\nnum y = another_unknown"
        
        errors = [
            UndefinedVariableError("unknown", SourcePosition(1, 11)),
            UndefinedVariableError("another_unknown", SourcePosition(2, 8))
        ]
        
        result = ErrorFormatter.format_multiple_errors(errors, source, "test.gr")
        
        assert "Multiple errors found in test.gr:" in result
        assert "Error 1:" in result
        assert "Error 2:" in result
        assert "unknown" in result
        assert "another_unknown" in result
    
    def test_single_error_in_multiple_errors(self):
        """Test that single error in list is formatted normally."""
        source = "undefined_var"
        errors = [UndefinedVariableError("undefined_var", SourcePosition(1, 1))]
        
        result = ErrorFormatter.format_multiple_errors(errors, source, "test.gr")
        
        # Should format as single error, not as multiple
        assert "Multiple errors found" not in result
        assert "Error in test.gr at line 1, column 1:" in result
    
    def test_edge_cases(self):
        """Test edge cases in error formatting."""
        # Test with empty source
        result = ErrorFormatter.format_error_with_context(
            Exception("Test"), "", "empty.gr"
        )
        assert result == "Error: Test"
        
        # Test with column 0 (shouldn't create pointer)
        source = "test line"
        token = Token(TokenType.IDENTIFIER, "test", 1, 0)
        error = ParseError("Test error", token)
        
        result = ErrorFormatter.format_error_with_context(error, source, "test.gr")
        lines = result.split('\n')
        # Should not have a pointer line since column is 0
        assert len(lines) == 3  # Error line, source line, message
    
    def test_column_boundary(self):
        """Test pointer positioning at different column positions."""
        source = "short"
        
        # Test column 1 (first character)
        token = Token(TokenType.IDENTIFIER, "s", 1, 1)
        error = ParseError("First char error", token)
        result = ErrorFormatter.format_error_with_context(error, source, "test.gr")
        lines = result.split('\n')
        assert "  ^" in result
        
        # Test column at end of line
        token = Token(TokenType.IDENTIFIER, "t", 1, 5)
        error = ParseError("Last char error", token)
        result = ErrorFormatter.format_error_with_context(error, source, "test.gr")
        lines = result.split('\n')
        assert "  ~~~~^" in result
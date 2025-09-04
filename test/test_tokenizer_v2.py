"""Tests for the enhanced tokenizer."""

import pytest
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../src'))

from glang.lexer.tokenizer import Tokenizer, Token, TokenType, TokenizerError

class TestTokenizer:
    """Test the enhanced tokenizer functionality."""
    
    def setup_method(self):
        self.tokenizer = Tokenizer()
    
    def test_basic_tokenization(self):
        """Test basic token recognition."""
        tokens = self.tokenizer.tokenize("hello world")
        
        assert len(tokens) == 4  # hello, world, newline, EOF
        assert tokens[0].type == TokenType.IDENTIFIER
        assert tokens[0].value == "hello"
        assert tokens[1].type == TokenType.IDENTIFIER
        assert tokens[1].value == "world"
        assert tokens[2].type == TokenType.NEWLINE
        assert tokens[3].type == TokenType.EOF
    
    def test_string_literals(self):
        """Test string literal tokenization."""
        # Double quotes
        tokens = self.tokenizer.tokenize('"hello world"')
        assert tokens[0].type == TokenType.STRING_LITERAL
        assert tokens[0].value == '"hello world"'
        
        # Single quotes
        tokens = self.tokenizer.tokenize("'test string'")
        assert tokens[0].type == TokenType.STRING_LITERAL
        assert tokens[0].value == "'test string'"
        
        # Escaped quotes
        tokens = self.tokenizer.tokenize('"he said \\"hello\\""')
        assert tokens[0].type == TokenType.STRING_LITERAL
        assert tokens[0].value == '"he said \\"hello\\""'
    
    def test_number_literals(self):
        """Test number literal tokenization."""
        # Integer
        tokens = self.tokenizer.tokenize("42")
        assert tokens[0].type == TokenType.NUMBER_LITERAL
        assert tokens[0].value == "42"
        
        # Float
        tokens = self.tokenizer.tokenize("3.14")
        assert tokens[0].type == TokenType.NUMBER_LITERAL
        assert tokens[0].value == "3.14"
        
        # Multiple numbers
        tokens = self.tokenizer.tokenize("1 2.5 999")
        numbers = [t for t in tokens if t.type == TokenType.NUMBER_LITERAL]
        assert len(numbers) == 3
        assert numbers[0].value == "1"
        assert numbers[1].value == "2.5"
        assert numbers[2].value == "999"
    
    def test_boolean_literals(self):
        """Test boolean literal tokenization."""
        tokens = self.tokenizer.tokenize("true false")
        
        assert tokens[0].type == TokenType.TRUE
        assert tokens[0].value == "true"
        assert tokens[1].type == TokenType.FALSE
        assert tokens[1].value == "false"
    
    def test_keywords(self):
        """Test keyword recognition."""
        tokens = self.tokenizer.tokenize("list string num bool")
        
        assert tokens[0].type == TokenType.LIST
        assert tokens[1].type == TokenType.STRING
        assert tokens[2].type == TokenType.NUM
        assert tokens[3].type == TokenType.BOOL
    
    def test_keywords_vs_identifiers(self):
        """Test that keywords are properly distinguished from identifiers."""
        # Keywords should be recognized as keywords
        tokens = self.tokenizer.tokenize("list")
        assert tokens[0].type == TokenType.LIST
        
        # Identifiers that contain keywords should be identifiers
        tokens = self.tokenizer.tokenize("listvar")
        assert tokens[0].type == TokenType.IDENTIFIER
        assert tokens[0].value == "listvar"
        
        tokens = self.tokenizer.tokenize("mylist")
        assert tokens[0].type == TokenType.IDENTIFIER
        assert tokens[0].value == "mylist"
    
    def test_operators_and_punctuation(self):
        """Test operator and punctuation tokenization."""
        tokens = self.tokenizer.tokenize("= . , [ ] ( ) < > : /")
        
        expected_types = [
            TokenType.EQUALS, TokenType.DOT, TokenType.COMMA,
            TokenType.LBRACKET, TokenType.RBRACKET,
            TokenType.LPAREN, TokenType.RPAREN,
            TokenType.LANGLE, TokenType.RANGLE,
            TokenType.COLON, TokenType.SLASH,
            TokenType.NEWLINE, TokenType.EOF
        ]
        
        for i, expected_type in enumerate(expected_types):
            assert tokens[i].type == expected_type
    
    def test_position_tracking(self):
        """Test line and column position tracking."""
        code = """line1 token
line2 another
line3"""
        tokens = self.tokenizer.tokenize(code)
        
        # Filter out newlines and EOF for easier testing
        content_tokens = [t for t in tokens if t.type not in (TokenType.NEWLINE, TokenType.EOF)]
        
        assert content_tokens[0].line == 1  # line1
        assert content_tokens[0].column == 1
        assert content_tokens[1].line == 1  # token  
        assert content_tokens[1].column == 7
        
        assert content_tokens[2].line == 2  # line2
        assert content_tokens[2].column == 1
        assert content_tokens[3].line == 2  # another
        assert content_tokens[3].column == 7
        
        assert content_tokens[4].line == 3  # line3
        assert content_tokens[4].column == 1
    
    def test_comments(self):
        """Test comment handling (should be skipped)."""
        tokens = self.tokenizer.tokenize("hello # this is a comment\nworld")
        
        # Comments should be completely skipped
        content_tokens = [t for t in tokens if t.type not in (TokenType.NEWLINE, TokenType.EOF)]
        assert len(content_tokens) == 2
        assert content_tokens[0].value == "hello"
        assert content_tokens[1].value == "world"
    
    def test_whitespace_handling(self):
        """Test whitespace handling."""
        tokens = self.tokenizer.tokenize("  hello   world  ")
        
        # Whitespace should be skipped, but structure preserved
        content_tokens = [t for t in tokens if t.type not in (TokenType.NEWLINE, TokenType.EOF)]
        assert len(content_tokens) == 2
        assert content_tokens[0].value == "hello"
        assert content_tokens[1].value == "world"
    
    def test_complex_expression(self):
        """Test tokenizing a complex expression."""
        code = 'list<num> numbers = [1, 2, 3]'
        tokens = self.tokenizer.tokenize(code)
        
        expected_sequence = [
            (TokenType.LIST, "list"),
            (TokenType.LANGLE, "<"),
            (TokenType.NUM, "num"), 
            (TokenType.RANGLE, ">"),
            (TokenType.IDENTIFIER, "numbers"),
            (TokenType.EQUALS, "="),
            (TokenType.LBRACKET, "["),
            (TokenType.NUMBER_LITERAL, "1"),
            (TokenType.COMMA, ","),
            (TokenType.NUMBER_LITERAL, "2"),
            (TokenType.COMMA, ","),
            (TokenType.NUMBER_LITERAL, "3"),
            (TokenType.RBRACKET, "]"),
            (TokenType.NEWLINE, "\\n"),
            (TokenType.EOF, "")
        ]
        
        assert len(tokens) == len(expected_sequence)
        for i, (expected_type, expected_value) in enumerate(expected_sequence):
            assert tokens[i].type == expected_type
            if expected_value:  # Skip checking empty values
                assert tokens[i].value == expected_value
    
    def test_method_call_tokenization(self):
        """Test tokenizing method calls."""
        code = 'obj.method("arg", 42)'
        tokens = self.tokenizer.tokenize(code)
        
        expected_types = [
            TokenType.IDENTIFIER,     # obj
            TokenType.DOT,           # .
            TokenType.IDENTIFIER,     # method
            TokenType.LPAREN,        # (
            TokenType.STRING_LITERAL, # "arg"
            TokenType.COMMA,         # ,
            TokenType.NUMBER_LITERAL, # 42
            TokenType.RPAREN,        # )
            TokenType.NEWLINE,
            TokenType.EOF
        ]
        
        for i, expected_type in enumerate(expected_types):
            assert tokens[i].type == expected_type
    
    def test_index_access_tokenization(self):
        """Test tokenizing index access expressions."""
        code = 'arr[0][1:3]'
        tokens = self.tokenizer.tokenize(code)
        
        expected_types = [
            TokenType.IDENTIFIER,     # arr
            TokenType.LBRACKET,      # [
            TokenType.NUMBER_LITERAL, # 0
            TokenType.RBRACKET,      # ]
            TokenType.LBRACKET,      # [
            TokenType.NUMBER_LITERAL, # 1
            TokenType.COLON,         # :
            TokenType.NUMBER_LITERAL, # 3
            TokenType.RBRACKET,      # ]
            TokenType.NEWLINE,
            TokenType.EOF
        ]
        
        for i, expected_type in enumerate(expected_types):
            assert tokens[i].type == expected_type
    
    def test_error_handling(self):
        """Test error handling for invalid characters."""
        with pytest.raises(TokenizerError) as exc_info:
            self.tokenizer.tokenize("hello @ world")
        
        assert "Unexpected character '@'" in str(exc_info.value)
        assert "line 1, column 7" in str(exc_info.value)
    
    def test_tokenize_expression(self):
        """Test the tokenize_expression convenience method."""
        tokens = self.tokenizer.tokenize_expression("var.method(42)")
        
        # Should not include newline or EOF
        assert len(tokens) == 6  # var . method ( 42 )
        assert tokens[0].type == TokenType.IDENTIFIER
        assert tokens[1].type == TokenType.DOT
        assert tokens[2].type == TokenType.IDENTIFIER
        assert tokens[3].type == TokenType.LPAREN
        assert tokens[4].type == TokenType.NUMBER_LITERAL
        assert tokens[5].type == TokenType.RPAREN
    
    def test_utility_methods(self):
        """Test utility methods."""
        # is_keyword
        assert self.tokenizer.is_keyword("list") == True
        assert self.tokenizer.is_keyword("string") == True
        assert self.tokenizer.is_keyword("myvar") == False
        
        # is_literal_token
        assert self.tokenizer.is_literal_token(TokenType.STRING_LITERAL) == True
        assert self.tokenizer.is_literal_token(TokenType.NUMBER_LITERAL) == True
        assert self.tokenizer.is_literal_token(TokenType.TRUE) == True
        assert self.tokenizer.is_literal_token(TokenType.IDENTIFIER) == False
        
        # is_type_keyword  
        assert self.tokenizer.is_type_keyword(TokenType.LIST) == True
        assert self.tokenizer.is_type_keyword(TokenType.STRING) == True
        assert self.tokenizer.is_type_keyword(TokenType.IDENTIFIER) == False

class TestTokenUtilities:
    """Test token utility functions."""
    
    def test_format_tokens(self):
        """Test token formatting for debugging."""
        from glang.lexer.tokenizer import format_tokens
        
        tokens = [
            Token(TokenType.IDENTIFIER, "hello", 1, 1),
            Token(TokenType.NUMBER_LITERAL, "42", 1, 7)
        ]
        
        formatted = format_tokens(tokens)
        assert "IDENTIFIER('hello') at 1:1" in formatted
        assert "NUMBER_LITERAL('42') at 1:7" in formatted
    
    def test_filter_tokens(self):
        """Test token filtering."""
        from glang.lexer.tokenizer import filter_tokens
        
        tokens = [
            Token(TokenType.IDENTIFIER, "hello", 1, 1),
            Token(TokenType.NEWLINE, "\\n", 1, 6),
            Token(TokenType.NUMBER_LITERAL, "42", 2, 1),
            Token(TokenType.EOF, "", 2, 3)
        ]
        
        # Filter to include only content tokens
        content_tokens = filter_tokens(tokens, exclude=[TokenType.NEWLINE, TokenType.EOF])
        assert len(content_tokens) == 2
        assert content_tokens[0].type == TokenType.IDENTIFIER
        assert content_tokens[1].type == TokenType.NUMBER_LITERAL
        
        # Filter to include only literals
        literals = filter_tokens(tokens, include=[TokenType.NUMBER_LITERAL, TokenType.STRING_LITERAL])
        assert len(literals) == 1
        assert literals[0].type == TokenType.NUMBER_LITERAL


if __name__ == '__main__':
    pytest.main([__file__])
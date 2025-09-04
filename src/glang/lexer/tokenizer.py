"""Enhanced tokenizer for glang with proper token types and position tracking."""

from enum import Enum
from dataclasses import dataclass
from typing import List, Optional, Tuple
import re

class TokenType(Enum):
    """Token types for glang language."""
    
    # Literals
    IDENTIFIER = "IDENTIFIER"
    STRING_LITERAL = "STRING_LITERAL"
    NUMBER_LITERAL = "NUMBER_LITERAL" 
    BOOLEAN_LITERAL = "BOOLEAN_LITERAL"
    
    # Operators & Punctuation
    DOT = "DOT"                    # .
    COMMA = "COMMA"                # ,
    EQUALS = "EQUALS"              # =
    LBRACKET = "LBRACKET"          # [
    RBRACKET = "RBRACKET"          # ]
    LPAREN = "LPAREN"              # (
    RPAREN = "RPAREN"              # )
    LANGLE = "LANGLE"              # <
    RANGLE = "RANGLE"              # >
    COLON = "COLON"                # :
    SLASH = "SLASH"                # /
    
    # Keywords (type keywords)
    LIST = "LIST"                  # list
    STRING = "STRING"              # string  
    NUM = "NUM"                    # num
    BOOL = "BOOL"                  # bool
    
    # Boolean literals (handled as keywords initially)
    TRUE = "TRUE"                  # true
    FALSE = "FALSE"                # false
    
    # Special
    NEWLINE = "NEWLINE"            # \n
    EOF = "EOF"                    # End of input

@dataclass
class Token:
    """A single token with type, value, and position information."""
    type: TokenType
    value: str
    line: int
    column: int
    
    def __str__(self):
        return f"{self.type.name}('{self.value}') at {self.line}:{self.column}"

class TokenizerError(Exception):
    """Error during tokenization."""
    def __init__(self, message: str, line: int, column: int):
        self.message = message
        self.line = line
        self.column = column
        super().__init__(f"{message} at line {line}, column {column}")

class Tokenizer:
    """Enhanced tokenizer with proper token classification."""
    
    def __init__(self):
        # Token patterns (order matters - more specific patterns first!)
        self.patterns: List[Tuple[str, Optional[TokenType]]] = [
            # Comments (skip entirely)
            (r'#.*', None),
            
            # String literals (quoted strings)
            (r'"([^"\\]|\\.)*"', TokenType.STRING_LITERAL),
            (r"'([^'\\]|\\.)*'", TokenType.STRING_LITERAL),
            
            # Number literals (floats before integers to catch decimals)
            # Include negative numbers
            (r'-?\d+\.\d+', TokenType.NUMBER_LITERAL),
            (r'-?\d+', TokenType.NUMBER_LITERAL),
            
            # Keywords (must come before identifiers!)
            # Boolean literals
            (r'\btrue\b', TokenType.TRUE),
            (r'\bfalse\b', TokenType.FALSE),
            
            # Type keywords
            (r'\blist\b', TokenType.LIST),
            (r'\bstring\b', TokenType.STRING),
            (r'\bnum\b', TokenType.NUM),
            (r'\bbool\b', TokenType.BOOL),
            
            # Identifiers (variable names, method names, etc.)
            (r'[a-zA-Z_][a-zA-Z0-9_]*', TokenType.IDENTIFIER),
            
            # Multi-character operators (before single character ones)
            (r'==', TokenType.EQUALS),  # Future: comparison operator
            
            # Single-character operators & punctuation
            (r'=', TokenType.EQUALS),
            (r'\.', TokenType.DOT),
            (r',', TokenType.COMMA),
            (r'\[', TokenType.LBRACKET),
            (r'\]', TokenType.RBRACKET),
            (r'\(', TokenType.LPAREN),
            (r'\)', TokenType.RPAREN),
            (r'<', TokenType.LANGLE),
            (r'>', TokenType.RANGLE),
            (r':', TokenType.COLON),
            (r'/', TokenType.SLASH),
            
            # Whitespace (skip)
            (r'[ \t]+', None),
            
            # Newlines (keep for statement separation)
            (r'\n', TokenType.NEWLINE),
        ]
        
        # Compile patterns for performance
        self.compiled_patterns = [
            (re.compile(pattern), token_type) 
            for pattern, token_type in self.patterns
        ]
    
    def tokenize(self, text: str) -> List[Token]:
        """
        Tokenize input text into a list of tokens.
        
        Args:
            text: The input text to tokenize
            
        Returns:
            List of Token objects
            
        Raises:
            TokenizerError: If invalid characters are encountered
        """
        tokens = []
        lines = text.split('\n')
        
        for line_num, line in enumerate(lines, 1):
            column = 1
            pos = 0
            
            while pos < len(line):
                matched = False
                
                # Try each pattern
                for pattern, token_type in self.compiled_patterns:
                    match = pattern.match(line, pos)
                    if match:
                        value = match.group(0)
                        
                        # Skip None token types (comments, whitespace)
                        if token_type is not None:
                            token = Token(token_type, value, line_num, column)
                            tokens.append(token)
                        
                        # Advance position
                        pos = match.end()
                        column += len(value)
                        matched = True
                        break
                
                if not matched:
                    raise TokenizerError(
                        f"Unexpected character '{line[pos]}'", 
                        line_num, 
                        column
                    )
            
            # Add newline token at end of each line (except empty lines at EOF)
            if line_num < len(lines) or line.strip():
                tokens.append(Token(TokenType.NEWLINE, '\\n', line_num, len(line) + 1))
        
        # Always end with EOF token
        final_line = len(lines) if lines else 1
        tokens.append(Token(TokenType.EOF, '', final_line, 1))
        return tokens
    
    def tokenize_expression(self, expr: str) -> List[Token]:
        """
        Tokenize a single expression (no newlines expected).
        
        Convenience method for parsing single expressions.
        """
        tokens = self.tokenize(expr)
        # Remove newline and EOF for expression parsing
        return [t for t in tokens if t.type not in (TokenType.NEWLINE, TokenType.EOF)]
    
    def is_keyword(self, value: str) -> bool:
        """Check if a string value is a keyword."""
        keyword_values = {
            'list', 'string', 'num', 'bool', 'true', 'false'
        }
        return value.lower() in keyword_values
    
    def is_literal_token(self, token_type: TokenType) -> bool:
        """Check if a token type represents a literal value."""
        return token_type in {
            TokenType.STRING_LITERAL,
            TokenType.NUMBER_LITERAL, 
            TokenType.TRUE,
            TokenType.FALSE
        }
    
    def is_type_keyword(self, token_type: TokenType) -> bool:
        """Check if a token type is a type keyword."""
        return token_type in {
            TokenType.LIST,
            TokenType.STRING,
            TokenType.NUM,
            TokenType.BOOL
        }

# Utility functions for working with tokens
def format_tokens(tokens: List[Token]) -> str:
    """Format tokens for debugging/display."""
    return '\n'.join(str(token) for token in tokens)

def filter_tokens(tokens: List[Token], 
                 include: Optional[List[TokenType]] = None,
                 exclude: Optional[List[TokenType]] = None) -> List[Token]:
    """
    Filter tokens by type.
    
    Args:
        tokens: List of tokens to filter
        include: If provided, only include these token types
        exclude: If provided, exclude these token types
    """
    result = tokens
    
    if include:
        result = [t for t in result if t.type in include]
    
    if exclude:
        result = [t for t in result if t.type not in exclude]
    
    return result
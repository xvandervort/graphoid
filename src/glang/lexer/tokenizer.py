"""Enhanced tokenizer for glang with proper token types and position tracking."""

from enum import Enum
from dataclasses import dataclass
from typing import List, Optional, Tuple, Dict
import re
import sys
import os

# Add src to path for imports 
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../../..'))

from glang.language import KEYWORD_REGISTRY, get_token_type_name

def _create_dynamic_token_type():
    """Create TokenType enum with both fixed and dynamic keyword tokens."""
    
    # Base token types that are always present
    base_tokens = {
        # Literals
        "IDENTIFIER": "IDENTIFIER",
        "STRING_LITERAL": "STRING_LITERAL", 
        "NUMBER_LITERAL": "NUMBER_LITERAL",
        "BOOLEAN_LITERAL": "BOOLEAN_LITERAL",
        
        # Operators & Punctuation  
        "DOT": "DOT",                    # .
        "COMMA": "COMMA",                # ,
        "EQUALS": "EQUALS",              # =
        "LBRACKET": "LBRACKET",          # [
        "RBRACKET": "RBRACKET",          # ]
        "LPAREN": "LPAREN",              # (
        "RPAREN": "RPAREN",              # )
        "LANGLE": "LANGLE",              # <
        "RANGLE": "RANGLE",              # >
        "COLON": "COLON",                # :
        "SLASH": "SLASH",                # /
        
        # Special
        "NEWLINE": "NEWLINE",            # \n
        "EOF": "EOF",                    # End of input
    }
    
    # Add dynamic keyword tokens from registry
    keyword_tokens = {}
    for keyword_text, definition in KEYWORD_REGISTRY.get_all_keywords().items():
        token_name = definition.get_token_type_name()
        keyword_tokens[token_name] = token_name
    
    # Combine base and keyword tokens
    all_tokens = {**base_tokens, **keyword_tokens}
    
    # Create the enum class dynamically
    return Enum('TokenType', all_tokens)

# Create the TokenType enum with dynamic keyword support
TokenType = _create_dynamic_token_type()

def _refresh_token_type():
    """Refresh TokenType enum when new keywords are added."""
    global TokenType
    TokenType = _create_dynamic_token_type()

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
            
            # Identifiers (variable names, method names, and keywords - post-processed)
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
                            # Post-process identifiers to convert keywords to proper token types
                            if token_type == TokenType.IDENTIFIER:
                                keyword_type = self.get_keyword_token_type(value)
                                if keyword_type:
                                    token_type = keyword_type
                            
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
        return KEYWORD_REGISTRY.is_keyword(value)
    
    def get_keyword_token_type(self, value: str) -> Optional[TokenType]:
        """Get the token type for a keyword, or None if not a keyword."""
        token_type_name = KEYWORD_REGISTRY.get_token_type_name(value)
        if token_type_name:
            # Get the token type by name from our dynamic enum
            return getattr(TokenType, token_type_name, None)
        return None
    
    def is_literal_token(self, token_type: TokenType) -> bool:
        """Check if a token type represents a literal value."""
        # Base literal types
        base_literals = {TokenType.STRING_LITERAL, TokenType.NUMBER_LITERAL}
        
        # Add boolean literal keywords from registry
        from glang.language import KeywordCategory
        boolean_keywords = KEYWORD_REGISTRY.get_keywords_by_category(KeywordCategory.LITERAL)
        for keyword_name in boolean_keywords:
            token_type_name = KEYWORD_REGISTRY.get_token_type_name(keyword_name)
            if hasattr(TokenType, token_type_name):
                base_literals.add(getattr(TokenType, token_type_name))
        
        return token_type in base_literals
    
    def is_type_keyword(self, token_type: TokenType) -> bool:
        """Check if a token type is a type keyword."""
        from glang.language import KeywordCategory
        type_keywords = KEYWORD_REGISTRY.get_keywords_by_category(KeywordCategory.TYPE)
        
        type_token_types = set()
        for keyword_name in type_keywords:
            token_type_name = KEYWORD_REGISTRY.get_token_type_name(keyword_name)
            if hasattr(TokenType, token_type_name):
                type_token_types.add(getattr(TokenType, token_type_name))
        
        return token_type in type_token_types

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
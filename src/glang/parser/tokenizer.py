"""Basic tokenizer for glang syntax."""

import re
from typing import List, Tuple, Optional


class Token:
    """Represents a single token."""
    def __init__(self, type_: str, value: str, position: int = 0):
        self.type = type_
        self.value = value
        self.position = position
    
    def __repr__(self):
        return f"Token({self.type}, {self.value!r})"


class Tokenizer:
    """Tokenizes glang input strings."""
    
    # Token patterns
    PATTERNS = [
        ('GRAPH_TYPE', r'\b(list|graph|tree|directed|weighted)\b'),
        ('IDENTIFIER', r'[a-zA-Z_][a-zA-Z0-9_]*'),
        ('NUMBER', r'-?\d+(\.\d+)?'),
        ('STRING_SINGLE', r"'([^']*)'"),
        ('STRING_DOUBLE', r'"([^"]*)"'),
        ('EQUALS', r'='),
        ('DOT', r'\.'),
        ('LBRACKET', r'\['),
        ('RBRACKET', r'\]'),
        ('LPAREN', r'\('),
        ('RPAREN', r'\)'),
        ('COMMA', r','),
        ('FLAG', r'--[a-zA-Z-]+'),
        ('WHITESPACE', r'\s+'),
    ]
    
    def __init__(self):
        self.pattern = self._compile_patterns()
    
    def _compile_patterns(self) -> re.Pattern:
        """Compile all patterns into a single regex."""
        pattern_parts = []
        for name, pattern in self.PATTERNS:
            pattern_parts.append(f'(?P<{name}>{pattern})')
        return re.compile('|'.join(pattern_parts))
    
    def tokenize(self, text: str) -> List[Token]:
        """Tokenize input text into a list of tokens."""
        tokens = []
        position = 0
        
        for match in self.pattern.finditer(text):
            token_type = match.lastgroup
            token_value = match.group()
            
            # Skip whitespace tokens
            if token_type == 'WHITESPACE':
                continue
            
            # Extract string content without quotes
            if token_type in ('STRING_SINGLE', 'STRING_DOUBLE'):
                token_value = token_value[1:-1]  # Remove quotes
                token_type = 'STRING'
            
            tokens.append(Token(token_type, token_value, match.start()))
        
        return tokens
    
    def peek_first_tokens(self, text: str, count: int = 3) -> List[Token]:
        """Get first N tokens without full tokenization."""
        tokens = []
        for match in self.pattern.finditer(text):
            if match.lastgroup != 'WHITESPACE':
                token_value = match.group()
                if match.lastgroup in ('STRING_SINGLE', 'STRING_DOUBLE'):
                    token_value = token_value[1:-1]
                    token_type = 'STRING'
                else:
                    token_type = match.lastgroup
                tokens.append(Token(token_type, token_value, match.start()))
                if len(tokens) >= count:
                    break
        return tokens
    
    def parse_list_literal(self, text: str) -> Optional[List[str]]:
        """Parse a list literal like [apple, banana, cherry]."""
        text = text.strip()
        if not text.startswith('[') or not text.endswith(']'):
            return None
        
        # Extract content between brackets
        content = text[1:-1].strip()
        if not content:
            return []
        
        # Parse comma-separated values
        items = []
        current = []
        in_string = False
        string_char = None
        
        for char in content:
            if char in ('"', "'") and not in_string:
                in_string = True
                string_char = char
            elif char == string_char and in_string:
                in_string = False
                string_char = None
            elif char == ',' and not in_string:
                item = ''.join(current).strip()
                if item:
                    # Remove quotes if present
                    if (item.startswith('"') and item.endswith('"')) or \
                       (item.startswith("'") and item.endswith("'")):
                        item = item[1:-1]
                    items.append(item)
                current = []
            else:
                current.append(char)
        
        # Add last item
        if current:
            item = ''.join(current).strip()
            if item:
                if (item.startswith('"') and item.endswith('"')) or \
                   (item.startswith("'") and item.endswith("'")):
                    item = item[1:-1]
                items.append(item)
        
        return items
    
    def infer_value_type(self, value: str):
        """Infer the type and convert a string value to the appropriate type."""
        value = value.strip()
        
        # Boolean values
        if value.lower() in ('true', 'false'):
            return bool(value.lower() == 'true')
        
        # Numbers - check for integer first
        try:
            # Try integer first
            if '.' not in value and 'e' not in value.lower():
                return int(value)
            else:
                # Float
                return float(value)
        except ValueError:
            pass
        
        # Everything else is a string
        return value
    
    def parse_list_literal_with_types(self, text: str) -> Optional[List]:
        """Parse a list literal and infer types for each element, including nested lists."""
        text = text.strip()
        if not text.startswith('[') or not text.endswith(']'):
            return None
        
        # Extract content between brackets
        content = text[1:-1].strip()
        if not content:
            return []
        
        # Parse comma-separated values with proper bracket nesting
        items = self._parse_nested_items(content)
        
        # Process each item - quoted items stay as strings, nested lists are parsed recursively
        typed_items = []
        for item in items:
            if (item.startswith('"') and item.endswith('"')) or \
               (item.startswith("'") and item.endswith("'")):
                # Quoted string - remove quotes and keep as string
                typed_items.append(item[1:-1])
            elif item.startswith('[') and item.endswith(']'):
                # Nested list - parse recursively
                nested_list = self.parse_list_literal_with_types(item)
                typed_items.append(nested_list)
            else:
                # Unquoted - apply type inference
                typed_items.append(self.infer_value_type(item))
        
        return typed_items
    
    def _parse_nested_items(self, content: str) -> List[str]:
        """Parse items from list content while respecting nested brackets and quotes."""
        items = []
        current = []
        bracket_depth = 0
        in_string = False
        string_char = None
        
        for char in content:
            if char in ('"', "'") and not in_string:
                in_string = True
                string_char = char
                current.append(char)
            elif char == string_char and in_string:
                in_string = False
                string_char = None
                current.append(char)
            elif char == '[' and not in_string:
                bracket_depth += 1
                current.append(char)
            elif char == ']' and not in_string:
                bracket_depth -= 1
                current.append(char)
            elif char == ',' and not in_string and bracket_depth == 0:
                # Only split on comma if we're not inside nested brackets
                item = ''.join(current).strip()
                if item:
                    items.append(item)
                current = []
            else:
                current.append(char)
        
        # Add last item
        if current:
            item = ''.join(current).strip()
            if item:
                items.append(item)
        
        return items
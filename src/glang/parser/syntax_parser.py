"""Main syntax parser for glang."""

import re
from typing import Optional, List, Any

from .ast_nodes import (
    ParsedCommand,
    VariableDeclaration,
    MethodCall,
    VariableAccess,
    LegacyCommand
)
from .tokenizer import Tokenizer


class SyntaxParser:
    """Parses glang input into AST nodes."""
    
    def __init__(self):
        self.tokenizer = Tokenizer()
        
        # Graph type keywords
        self.graph_types = {'list', 'graph', 'tree', 'directed', 'weighted'}
        
        # Legacy command keywords
        self.legacy_commands = {
            'create', 'show', 'delete', 'append', 'prepend', 'insert',
            'reverse', 'traverse', 'graphs', 'namespace', 'stats', 'info',
            'help', 'h', 'exit', 'x', 'version', 'ver', 'clear'
        }
    
    def parse_input(self, input_str: str) -> ParsedCommand:
        """Parse input string into appropriate command type."""
        input_str = input_str.strip()
        if not input_str:
            return LegacyCommand(raw_input=input_str, command='', arguments=[])
        
        # Quick peek at first tokens to determine input type
        tokens = self.tokenizer.peek_first_tokens(input_str, 3)
        
        if not tokens:
            return LegacyCommand(raw_input=input_str, command='', arguments=[])
        
        # Check for variable declaration: list fruits = [...]
        if self._is_variable_declaration(tokens):
            return self._parse_declaration(input_str)
        
        # Check for method call: fruits.append 
        if self._is_method_call(tokens):
            return self._parse_method_call(input_str)
        
        # Check for legacy command
        if self._is_legacy_command(tokens):
            return self._parse_legacy_command(input_str)
        
        # Default to variable access
        return self._parse_variable_access(input_str)
    
    def _is_variable_declaration(self, tokens: List) -> bool:
        """Check if tokens represent a variable declaration."""
        if len(tokens) >= 3:
            # Pattern: graph_type identifier =
            return (tokens[0].type == 'GRAPH_TYPE' and 
                   tokens[1].type == 'IDENTIFIER' and
                   tokens[2].type == 'EQUALS')
        return False
    
    def _is_method_call(self, tokens: List) -> bool:
        """Check if tokens represent a method call."""
        if len(tokens) >= 2:
            # Pattern: identifier.method
            return (tokens[0].type == 'IDENTIFIER' and 
                   tokens[1].type == 'DOT')
        return False
    
    def _is_legacy_command(self, tokens: List) -> bool:
        """Check if tokens represent a legacy command."""
        if tokens and tokens[0].type == 'IDENTIFIER':
            return tokens[0].value.lower() in self.legacy_commands
        return False
    
    def _parse_declaration(self, input_str: str) -> VariableDeclaration:
        """Parse a variable declaration.
        
        Examples:
            list fruits = [apple, banana]
            graph g = directed()
        """
        tokens = self.tokenizer.tokenize(input_str)
        
        if len(tokens) < 3:
            raise ValueError(f"Invalid declaration syntax: {input_str}")
        
        graph_type = tokens[0].value
        variable_name = tokens[1].value
        
        # Skip the equals sign
        if tokens[2].type != 'EQUALS':
            raise ValueError(f"Expected '=' in declaration: {input_str}")
        
        # Parse initializer
        initializer = None
        remaining = input_str[tokens[2].position + 1:].strip()
        
        if remaining.startswith('['):
            # List literal
            initializer = self.tokenizer.parse_list_literal(remaining)
        elif remaining.endswith('()'):
            # Constructor call (empty initialization)
            initializer = []
        
        return VariableDeclaration(
            raw_input=input_str,
            graph_type=graph_type,
            variable_name=variable_name,
            initializer=initializer
        )
    
    def _parse_method_call(self, input_str: str) -> MethodCall:
        """Parse a method call.
        
        Examples:
            fruits.append cherry
            numbers.reverse()
        """
        # Use regex to parse method calls (handle parentheses)
        pattern = r'^(\w+)\.(\w+)(?:\(\))?(?:\s+(.+))?$'
        match = re.match(pattern, input_str)
        
        if not match:
            raise ValueError(f"Invalid method call syntax: {input_str}")
        
        variable_name = match.group(1)
        method_name = match.group(2)
        args_str = match.group(3) or ''
        
        # Parse arguments
        arguments = []
        if args_str:
            args_str = args_str.strip()
            # Handle quoted strings
            if args_str.startswith('"') and args_str.endswith('"'):
                arguments = [args_str[1:-1]]
            elif args_str.startswith("'") and args_str.endswith("'"):
                arguments = [args_str[1:-1]]
            else:
                # Split by spaces (simple parsing for now)
                arguments = args_str.split()
        
        return MethodCall(
            raw_input=input_str,
            variable_name=variable_name,
            method_name=method_name,
            arguments=arguments
        )
    
    def _parse_variable_access(self, input_str: str) -> VariableAccess:
        """Parse a variable access.
        
        Examples:
            fruits
            fruits --show-nodes
        """
        parts = input_str.split()
        if not parts:
            raise ValueError(f"Invalid variable access: {input_str}")
        
        variable_name = parts[0]
        flags = [p for p in parts[1:] if p.startswith('--')]
        
        return VariableAccess(
            raw_input=input_str,
            variable_name=variable_name,
            flags=flags
        )
    
    def _parse_legacy_command(self, input_str: str) -> LegacyCommand:
        """Parse a legacy command format."""
        parts = input_str.split(None, 1)
        command = parts[0].lower() if parts else ''
        arguments = []
        
        if len(parts) > 1:
            # Parse the rest as arguments
            args_str = parts[1]
            # Special handling for create command with list literal
            if command == 'create' and '[' in args_str:
                # Extract variable name and list
                match = re.match(r'(\w+)\s*(\[.+\])', args_str)
                if match:
                    var_name = match.group(1)
                    list_str = match.group(2)
                    arguments = [var_name, list_str]
                else:
                    arguments = [args_str]
            else:
                arguments = args_str.split()
        
        return LegacyCommand(
            raw_input=input_str,
            command=command,
            arguments=arguments
        )
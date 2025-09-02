"""Main syntax parser for glang."""

import re
from typing import Optional, List, Any

from .ast_nodes import (
    ParsedCommand,
    VariableDeclaration,
    MethodCall,
    VariableAccess,
    IndexAccess,
    IndexAssignment,
    SliceAccess,
    SliceAssignment,
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
        
        # Quick peek at first tokens to determine input type (need 6 for type constraints)
        tokens = self.tokenizer.peek_first_tokens(input_str, 6)
        
        if not tokens:
            return LegacyCommand(raw_input=input_str, command='', arguments=[])
        
        # Check for variable declaration: list fruits = [...]
        if self._is_variable_declaration(tokens):
            return self._parse_declaration(input_str)
        
        # Check for method call: fruits.append 
        if self._is_method_call(tokens):
            return self._parse_method_call(input_str)
        
        # Check for slice assignment: fruits[1:3] = values
        if self._is_slice_assignment(input_str):
            return self._parse_slice_assignment(input_str)
        
        # Check for index assignment: fruits[0] = value
        if self._is_index_assignment(input_str):
            return self._parse_index_assignment(input_str)
        
        # Check for slice access: fruits[1:3]
        if self._is_slice_access(input_str):
            return self._parse_slice_access(input_str)
        
        # Check for index access: fruits[0]
        if self._is_index_access(input_str):
            return self._parse_index_access(input_str)
        
        # Check for legacy command
        if self._is_legacy_command(tokens):
            return self._parse_legacy_command(input_str)
        
        # Default to variable access
        return self._parse_variable_access(input_str)
    
    def _is_variable_declaration(self, tokens: List) -> bool:
        """Check if tokens represent a variable declaration."""
        if len(tokens) >= 3:
            # Pattern: graph_type identifier = (basic)
            # Pattern: graph_type<type> identifier = (with constraints)
            if (tokens[0].type == 'GRAPH_TYPE'):
                # Check for basic pattern: list var =
                if (tokens[1].type == 'IDENTIFIER' and 
                    tokens[2].type == 'EQUALS'):
                    return True
                # Check for constraint pattern: list<num> var = or list<list> var =
                elif (len(tokens) >= 6 and
                      tokens[1].type == 'LANGLE' and
                      (tokens[2].type == 'TYPE_CONSTRAINT' or 
                       tokens[2].type == 'GRAPH_TYPE' and tokens[2].value in ['list', 'num', 'string', 'bool']) and
                      tokens[3].type == 'RANGLE' and
                      tokens[4].type == 'IDENTIFIER' and
                      tokens[5].type == 'EQUALS'):
                    return True
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
    
    def _is_index_access(self, input_str: str) -> bool:
        """Check if input represents an index access."""
        # Pattern: variable[index] or variable[index1][index2]
        pattern = r'^\w+(\[[-]?\d+\])+$'
        return bool(re.match(pattern, input_str.strip()))
    
    def _is_index_assignment(self, input_str: str) -> bool:
        """Check if input represents an index assignment."""
        # Pattern: variable[index] = value or variable[index1][index2] = value
        pattern = r'^\w+(\[[-]?\d+\])+\s*=\s*.+$'
        return bool(re.match(pattern, input_str.strip()))
    
    def _is_slice_access(self, input_str: str) -> bool:
        """Check if input represents a slice access."""
        # Pattern: variable[start:stop] or variable[start:stop:step] or variable[start:] etc.
        # Allow whitespace around colons and numbers
        pattern = r'^\w+\[\s*[-]?\d*\s*:\s*[-]?\d*\s*(?::\s*[-]?\d*\s*)?\]$'
        return bool(re.match(pattern, input_str.strip()))
    
    def _is_slice_assignment(self, input_str: str) -> bool:
        """Check if input represents a slice assignment."""
        # Pattern: variable[start:stop] = value or variable[start:stop:step] = value
        # Allow whitespace around colons and numbers
        pattern = r'^\w+\[\s*[-]?\d*\s*:\s*[-]?\d*\s*(?::\s*[-]?\d*\s*)?\]\s*=\s*.+$'
        return bool(re.match(pattern, input_str.strip()))
    
    def _parse_declaration(self, input_str: str) -> VariableDeclaration:
        """Parse a variable declaration.
        
        Examples:
            list fruits = [apple, banana]
            graph g = directed()
            list<num> scores = [95, 87, 92]
        """
        tokens = self.tokenizer.tokenize(input_str)
        
        if len(tokens) < 3:
            raise ValueError(f"Invalid declaration syntax: {input_str}")
        
        graph_type = tokens[0].value
        type_constraint = None
        variable_name = None
        equals_index = None
        
        # Check if we have type constraints: list<num> var = or list<list> var =
        if (len(tokens) >= 5 and tokens[1].type == 'LANGLE' and 
            (tokens[2].type == 'TYPE_CONSTRAINT' or 
             tokens[2].type == 'GRAPH_TYPE' and tokens[2].value in ['list', 'num', 'string', 'bool']) and 
            tokens[3].type == 'RANGLE'):
            type_constraint = tokens[2].value
            variable_name = tokens[4].value
            equals_index = 5
        else:
            # Basic pattern: list var =
            variable_name = tokens[1].value
            equals_index = 2
        
        # Validate equals sign
        if equals_index >= len(tokens) or tokens[equals_index].type != 'EQUALS':
            raise ValueError(f"Expected '=' in declaration: {input_str}")
        
        # Parse initializer
        initializer = None
        remaining = input_str[tokens[equals_index].position + 1:].strip()
        
        if remaining.startswith('['):
            # List literal with type inference
            initializer = self.tokenizer.parse_list_literal_with_types(remaining)
        elif remaining.endswith('()'):
            # Constructor call (empty initialization)
            initializer = []
        else:
            # Scalar value - parse as single value
            # First check if it's a quoted string
            if (remaining.startswith("'") and remaining.endswith("'")) or \
               (remaining.startswith('"') and remaining.endswith('"')):
                # Quoted string - remove quotes
                initializer = remaining[1:-1]
            else:
                # Use the strict list item type inference (treats identifiers as variable references)
                try:
                    initializer = self.tokenizer.infer_list_item_type(remaining)
                except ValueError:
                    # If it fails (invalid identifier), fall back to standard inference
                    initializer = self.tokenizer.infer_value_type(remaining)
        
        return VariableDeclaration(
            raw_input=input_str,
            graph_type=graph_type,
            variable_name=variable_name,
            initializer=initializer,
            type_constraint=type_constraint
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
    
    def _parse_index_access(self, input_str: str) -> IndexAccess:
        """Parse an index access.
        
        Examples:
            fruits[0]
            numbers[-1]
            matrix[1][2]
        """
        input_str = input_str.strip()
        
        # Extract variable name and indices using regex
        pattern = r'^(\w+)((?:\[[-]?\d+\])+)$'
        match = re.match(pattern, input_str)
        
        if not match:
            raise ValueError(f"Invalid index access syntax: {input_str}")
        
        variable_name = match.group(1)
        indices_str = match.group(2)
        
        # Extract all indices
        indices = []
        index_pattern = r'\[([-]?\d+)\]'
        for match in re.finditer(index_pattern, indices_str):
            indices.append(int(match.group(1)))
        
        return IndexAccess(
            raw_input=input_str,
            variable_name=variable_name,
            indices=indices
        )
    
    def _parse_index_assignment(self, input_str: str) -> IndexAssignment:
        """Parse an index assignment.
        
        Examples:
            fruits[0] = 'mango'
            matrix[1][2] = 42
            items[-1] = 'last'
        """
        input_str = input_str.strip()
        
        # Split on the equals sign
        if '=' not in input_str:
            raise ValueError(f"Invalid index assignment syntax: {input_str}")
        
        left_side, right_side = input_str.split('=', 1)
        left_side = left_side.strip()
        right_side = right_side.strip()
        
        # Parse the left side (variable[indices]) like index access
        pattern = r'^(\w+)((?:\[[-]?\d+\])+)$'
        match = re.match(pattern, left_side)
        
        if not match:
            raise ValueError(f"Invalid index assignment syntax: {input_str}")
        
        variable_name = match.group(1)
        indices_str = match.group(2)
        
        # Extract all indices
        indices = []
        index_pattern = r'\[([-]?\d+)\]'
        for match in re.finditer(index_pattern, indices_str):
            indices.append(int(match.group(1)))
        
        # Parse the right side (value) with type inference
        typed_value = self.tokenizer.infer_value_type(right_side)
        
        return IndexAssignment(
            raw_input=input_str,
            variable_name=variable_name,
            indices=indices,
            value=typed_value
        )
    
    def _parse_slice_access(self, input_str: str) -> SliceAccess:
        """Parse a slice access.
        
        Examples:
            fruits[1:3]
            numbers[::2] 
            items[1:]
            data[:-1]
        """
        input_str = input_str.strip()
        
        # Pattern: variable[slice_notation] with whitespace support
        pattern = r'^(\w+)\[\s*([-]?\d*)\s*:\s*([-]?\d*)\s*(?::\s*([-]?\d*)\s*)?\]$'
        match = re.match(pattern, input_str)
        
        if not match:
            raise ValueError(f"Invalid slice syntax: {input_str}")
        
        variable_name = match.group(1)
        start_str = match.group(2)
        stop_str = match.group(3)
        step_str = match.group(4) if match.group(4) is not None else None
        
        # Convert empty strings to None, non-empty to int
        start = int(start_str) if start_str else None
        stop = int(stop_str) if stop_str else None
        step = int(step_str) if step_str else None
        
        return SliceAccess(
            raw_input=input_str,
            variable_name=variable_name,
            start=start,
            stop=stop,
            step=step
        )
    
    def _parse_slice_assignment(self, input_str: str) -> SliceAssignment:
        """Parse a slice assignment.
        
        Examples:
            fruits[1:3] = [a, b]
            numbers[::2] = [1, 3, 5]
        """
        input_str = input_str.strip()
        
        # Split on the equals sign
        if '=' not in input_str:
            raise ValueError(f"Invalid slice assignment syntax: {input_str}")
        
        left_side, right_side = input_str.split('=', 1)
        left_side = left_side.strip()
        right_side = right_side.strip()
        
        # Parse the left side (variable[slice]) with whitespace support
        pattern = r'^(\w+)\[\s*([-]?\d*)\s*:\s*([-]?\d*)\s*(?::\s*([-]?\d*)\s*)?\]$'
        match = re.match(pattern, left_side)
        
        if not match:
            raise ValueError(f"Invalid slice assignment syntax: {input_str}")
        
        variable_name = match.group(1)
        start_str = match.group(2)
        stop_str = match.group(3)
        step_str = match.group(4) if match.group(4) is not None else None
        
        # Convert empty strings to None, non-empty to int
        start = int(start_str) if start_str else None
        stop = int(stop_str) if stop_str else None
        step = int(step_str) if step_str else None
        
        # Parse the right side - if it looks like a list, parse as list, otherwise as single value
        if right_side.startswith('[') and right_side.endswith(']'):
            typed_value = self.tokenizer.parse_list_literal_with_types(right_side)
        else:
            typed_value = self.tokenizer.infer_value_type(right_side)
        
        return SliceAssignment(
            raw_input=input_str,
            variable_name=variable_name,
            start=start,
            stop=stop,
            step=step,
            value=typed_value
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
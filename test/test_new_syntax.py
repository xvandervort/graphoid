"""Tests for the new language-like syntax."""

import pytest
from glang.parser import SyntaxParser, InputType
from glang.parser.ast_nodes import (
    VariableDeclaration,
    MethodCall,
    VariableAccess,
    LegacyCommand
)


class TestSyntaxParser:
    """Test syntax parser functionality."""
    
    def setup_method(self):
        """Set up for each test."""
        self.parser = SyntaxParser()
    
    def test_variable_declaration_list(self):
        """Test parsing list variable declarations."""
        input_str = 'list fruits = ["apple", "banana", "cherry"]'
        parsed = self.parser.parse_input(input_str)
        
        assert isinstance(parsed, VariableDeclaration)
        assert parsed.input_type == InputType.VARIABLE_DECLARATION
        assert parsed.graph_type == "list"
        assert parsed.variable_name == "fruits"
        assert parsed.initializer == ["apple", "banana", "cherry"]
    
    def test_variable_declaration_with_quotes(self):
        """Test parsing with quoted strings."""
        input_str = 'list items = ["hello world", \'test item\', "unquoted"]'
        parsed = self.parser.parse_input(input_str)
        
        assert isinstance(parsed, VariableDeclaration)
        assert parsed.initializer == ["hello world", "test item", "unquoted"]
    
    def test_variable_declaration_empty(self):
        """Test parsing empty list declaration."""
        input_str = "list empty = []"
        parsed = self.parser.parse_input(input_str)
        
        assert isinstance(parsed, VariableDeclaration)
        assert parsed.initializer == []
    
    def test_method_call_with_arg(self):
        """Test parsing method calls with arguments."""
        input_str = "fruits.append orange"
        parsed = self.parser.parse_input(input_str)
        
        assert isinstance(parsed, MethodCall)
        assert parsed.input_type == InputType.METHOD_CALL
        assert parsed.variable_name == "fruits"
        assert parsed.method_name == "append"
        assert parsed.arguments == ["orange"]
    
    def test_method_call_with_quoted_arg(self):
        """Test method call with quoted argument."""
        input_str = 'fruits.append "golden delicious"'
        parsed = self.parser.parse_input(input_str)
        
        assert isinstance(parsed, MethodCall)
        assert parsed.arguments == ["golden delicious"]
    
    def test_method_call_no_args(self):
        """Test parsing method calls without arguments."""
        input_str = "numbers.reverse()"
        parsed = self.parser.parse_input(input_str)
        
        assert isinstance(parsed, MethodCall)
        assert parsed.variable_name == "numbers"
        assert parsed.method_name == "reverse"
        assert parsed.arguments == []
    
    def test_variable_access_simple(self):
        """Test parsing simple variable access."""
        input_str = "fruits"
        parsed = self.parser.parse_input(input_str)
        
        assert isinstance(parsed, VariableAccess)
        assert parsed.input_type == InputType.VARIABLE_ACCESS
        assert parsed.variable_name == "fruits"
        assert parsed.flags == []
    
    def test_variable_access_with_flags(self):
        """Test parsing variable access with flags."""
        input_str = "fruits --show-nodes"
        parsed = self.parser.parse_input(input_str)
        
        assert isinstance(parsed, VariableAccess)
        assert parsed.variable_name == "fruits"
        assert parsed.flags == ["--show-nodes"]
    
    def test_legacy_command_create(self):
        """Test parsing legacy create command with slash prefix."""
        input_str = "/create fruits [apple, banana]"
        parsed = self.parser.parse_input(input_str)
        
        assert isinstance(parsed, LegacyCommand)
        assert parsed.input_type == InputType.LEGACY_COMMAND
        assert parsed.command == "create"
        assert parsed.arguments[0] == "fruits"
        assert "[apple, banana]" in " ".join(parsed.arguments)
    
    def test_legacy_command_show(self):
        """Test parsing legacy show command with slash prefix."""
        input_str = "/show fruits"
        parsed = self.parser.parse_input(input_str)
        
        assert isinstance(parsed, LegacyCommand)
        assert parsed.command == "show"
        assert parsed.arguments == ["fruits"]
    
    def test_legacy_command_simple(self):
        """Test parsing simple legacy commands with slash prefix."""
        for cmd in ["help", "exit", "version", "namespace", "stats"]:
            parsed = self.parser.parse_input(f"/{cmd}")
            assert isinstance(parsed, LegacyCommand)
            assert parsed.command == cmd
            assert parsed.arguments == []


class TestTokenizer:
    """Test tokenizer functionality."""
    
    def test_parse_list_literal(self):
        """Test parsing list literals."""
        from glang.parser.tokenizer import Tokenizer
        tokenizer = Tokenizer()
        
        # Simple list
        result = tokenizer.parse_list_literal("[apple, banana, cherry]")
        assert result == ["apple", "banana", "cherry"]
        
        # List with quotes
        result = tokenizer.parse_list_literal('["hello world", \'test\', unquoted]')
        assert result == ["hello world", "test", "unquoted"]
        
        # Empty list
        result = tokenizer.parse_list_literal("[]")
        assert result == []
        
        # Not a list
        result = tokenizer.parse_list_literal("not a list")
        assert result is None
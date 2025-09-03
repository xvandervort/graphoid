"""
Tests for the command prefix system (Phase 3 enhancement).
"""

import pytest
from unittest.mock import patch, MagicMock
from glang.repl.repl import REPL
from glang.parser.syntax_parser import SyntaxParser
from glang.parser.ast_nodes import LegacyCommand, VariableAccess


class TestCommandPrefixSystem:
    """Test cases for the command prefix system."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.repl = REPL()
        self.parser = SyntaxParser()
    
    def test_slash_prefix_parsing(self):
        """Test that slash-prefixed commands are parsed correctly."""
        # Test /help command
        parsed = self.parser.parse_input("/help")
        assert isinstance(parsed, LegacyCommand)
        assert parsed.command == "help"
        assert parsed.arguments == []
        
        # Test /h shorthand
        parsed = self.parser.parse_input("/h")
        assert isinstance(parsed, LegacyCommand)
        assert parsed.command == "h"
        assert parsed.arguments == []
        
        # Test /exit command
        parsed = self.parser.parse_input("/exit")
        assert isinstance(parsed, LegacyCommand)
        assert parsed.command == "exit"
        assert parsed.arguments == []
        
        # Test command with arguments
        parsed = self.parser.parse_input("/create fruits [apple, banana]")
        assert isinstance(parsed, LegacyCommand)
        assert parsed.command == "create"
        assert parsed.arguments == ["fruits", "[apple, banana]"]
    
    def test_commands_require_slash_prefix(self):
        """Test that commands ALWAYS require slash prefix - no backward compatibility."""
        # Without slash prefix - should always be variable access, never command
        parsed = self.parser.parse_input("help")
        assert isinstance(parsed, VariableAccess)
        assert parsed.variable_name == "help"
        
        parsed = self.parser.parse_input("h")
        assert isinstance(parsed, VariableAccess)  
        assert parsed.variable_name == "h"
        
        parsed = self.parser.parse_input("exit")
        assert isinstance(parsed, VariableAccess)
        assert parsed.variable_name == "exit"
    
    def test_slash_prefix_enables_command_execution(self):
        """Test that slash prefix enables command execution (only way to run commands)."""
        # With slash - should be command
        parsed = self.parser.parse_input("/h")
        assert isinstance(parsed, LegacyCommand)
        assert parsed.command == "h"
        
        parsed = self.parser.parse_input("/help")
        assert isinstance(parsed, LegacyCommand)
        assert parsed.command == "help"
        
        parsed = self.parser.parse_input("/exit")
        assert isinstance(parsed, LegacyCommand)
        assert parsed.command == "exit"
    
    def test_no_backward_compatibility(self):
        """Test that commands WITHOUT slash prefix are treated as variable access."""
        commands_to_test = ["help", "h", "exit", "x", "version", "ver", 
                           "graphs", "namespace", "stats"]
        
        for cmd in commands_to_test:
            parsed = self.parser.parse_input(cmd)
            assert isinstance(parsed, VariableAccess)
            assert parsed.variable_name == cmd
    
    @patch('sys.stdout')
    def test_help_shows_mandatory_slash_syntax(self, mock_stdout):
        """Test that help command shows mandatory slash syntax."""
        self.repl._help_command()
        
        # Check if mock was called (output was produced)
        assert mock_stdout.write.called
        
        # Get all the printed content
        printed_content = ''.join([call[0][0] for call in mock_stdout.write.call_args_list])
        
        # Check for mandatory slash documentation
        assert "All commands MUST use slash prefix" in printed_content
        assert "/h or /help" in printed_content
        assert "ERROR - creates/accesses variable" in printed_content
    
    def test_empty_slash_command(self):
        """Test behavior with just a slash."""
        parsed = self.parser.parse_input("/")
        assert isinstance(parsed, LegacyCommand)
        assert parsed.command == ""
        assert parsed.arguments == []
    
    def test_slash_with_whitespace(self):
        """Test slash commands with various whitespace."""
        parsed = self.parser.parse_input("/ help")
        assert isinstance(parsed, LegacyCommand)
        assert parsed.command == "help"
        
        parsed = self.parser.parse_input("/  exit  ")
        assert isinstance(parsed, LegacyCommand)
        assert parsed.command == "exit"


class TestREPLIntegration:
    """Integration tests for command prefix system in REPL."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.repl = REPL()
    
    @patch('builtins.print')
    def test_variable_vs_slash_command(self, mock_print):
        """Test that variables and slash commands work independently."""
        # Create a variable named 'h'
        self.repl._process_input("list h = [1, 2, 3]")
        
        # Access without prefix should show variable
        self.repl._process_input("h")
        
        # Check that variable content was displayed (not help)
        printed_calls = [str(call) for call in mock_print.call_args_list]
        
        # Should show list content, not help text
        assert any("[1, 2, 3]" in call for call in printed_calls)
        
        # Clear the mock for next test
        mock_print.reset_mock()
        
        # Access with slash prefix should show help
        self.repl._process_input("/h")
        
        # Check that help was displayed
        printed_calls = [str(call) for call in mock_print.call_args_list]
        assert any("Available commands" in call for call in printed_calls)
    
    @patch('builtins.print')
    def test_commands_require_slash_always(self, mock_print):
        """Test that commands ALWAYS require slash prefix, even when no variables exist."""
        # Test that 'h' without slash is treated as variable access, even when no 'h' variable exists
        self.repl._process_input("h")
        
        # Check that we got variable not found, NOT help
        printed_calls = [str(call) for call in mock_print.call_args_list]
        assert any("Variable 'h' not found" in call for call in printed_calls)
        assert not any("Available commands" in call for call in printed_calls)
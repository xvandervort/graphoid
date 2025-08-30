"""
Tests for the Glang REPL functionality.
"""

import pytest
from unittest.mock import patch, MagicMock
from glang.repl.repl import REPL
from glang import __version__


class TestREPL:
    """Test cases for the REPL class."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.repl = REPL()
    
    def test_initialization(self):
        """Test REPL initialization."""
        assert self.repl.prompt == "glang> "
        assert self.repl.running is True
        assert "ver" in self.repl.commands
        assert "version" in self.repl.commands
        assert "h" in self.repl.commands
        assert "help" in self.repl.commands
        assert "x" in self.repl.commands
        assert "exit" in self.repl.commands
    
    def test_version_command(self, capsys):
        """Test version command output."""
        self.repl._version_command()
        captured = capsys.readouterr()
        assert __version__ in captured.out
        assert "Glang version" in captured.out
    
    def test_help_command(self, capsys):
        """Test help command output."""
        self.repl._help_command()
        captured = capsys.readouterr()
        assert "Available commands:" in captured.out
        assert "ver, version" in captured.out
        assert "h, help" in captured.out
        assert "x, exit" in captured.out
    
    def test_exit_command(self, capsys):
        """Test exit command."""
        result = self.repl._exit_command()
        captured = capsys.readouterr()
        assert result is False
        assert "Goodbye!" in captured.out
    
    def test_process_input_known_command(self, capsys):
        """Test processing of known commands."""
        self.repl._process_input("ver")
        captured = capsys.readouterr()
        assert __version__ in captured.out
    
    def test_process_input_unknown_command(self, capsys):
        """Test processing of unknown commands."""
        self.repl._process_input("unknown")
        captured = capsys.readouterr()
        assert "Unknown command: unknown" in captured.out
        assert "Type 'help' for available commands." in captured.out
    
    def test_process_input_case_insensitive(self, capsys):
        """Test that commands are case insensitive."""
        self.repl._process_input("VER")
        captured = capsys.readouterr()
        assert __version__ in captured.out
    
    def test_exit_command_stops_repl(self):
        """Test that exit command stops the REPL."""
        self.repl._process_input("exit")
        assert self.repl.running is False
    
    @patch('builtins.input')
    @patch('builtins.print')
    def test_start_with_exit(self, mock_print, mock_input):
        """Test REPL start and exit flow."""
        mock_input.side_effect = ["exit"]
        self.repl.start()
        assert not self.repl.running
    
    @patch('builtins.input')
    @patch('builtins.print')
    def test_start_with_keyboard_interrupt(self, mock_print, mock_input):
        """Test REPL handles KeyboardInterrupt gracefully."""
        mock_input.side_effect = KeyboardInterrupt()
        self.repl.start()
        # Should exit gracefully without error
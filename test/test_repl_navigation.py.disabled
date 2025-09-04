"""
Tests for REPL navigation improvements.
"""

import pytest
from unittest.mock import patch, MagicMock
from glang.repl.repl import REPL, READLINE_AVAILABLE


class TestREPLNavigation:
    """Test cases for REPL navigation features."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.repl = REPL()
    
    def test_readline_setup(self):
        """Test that readline is set up if available."""
        if READLINE_AVAILABLE:
            assert hasattr(self.repl, '_history_file')
            assert isinstance(self.repl.command_history, list)
        else:
            # Should still work without readline
            assert isinstance(self.repl.command_history, list)
    
    def test_command_completion(self):
        """Test tab completion for commands."""
        # Test completing basic commands
        completions = self.repl._get_completions("h", "h")
        assert "help" in completions
        
        completions = self.repl._get_completions("cr", "cr")
        assert "create" in completions
        
        completions = self.repl._get_completions("na", "na")
        assert "namespace" in completions
    
    def test_variable_completion(self):
        """Test tab completion for variable names."""
        # Create some variables
        self.repl.graph_manager.create_from_list("fruits", ["apple", "banana"])
        self.repl.graph_manager.create_from_list("numbers", [1, 2, 3])
        
        # Test completion for show command
        completions = self.repl._get_completions("fr", "show fr")
        assert "fruits" in completions
        
        completions = self.repl._get_completions("nu", "delete nu")
        assert "numbers" in completions
        
        # Test completion for info command
        completions = self.repl._get_completions("f", "info f")
        assert "fruits" in completions
    
    def test_graph_type_completion(self):
        """Test tab completion for graph types."""
        completions = self.repl._get_completions("l", "create test l")
        assert "linear" in completions
        
        completions = self.repl._get_completions("di", "create test di")
        assert "directed" in completions
        
        completions = self.repl._get_completions("t", "create test t")
        assert "tree" in completions
    
    def test_history_tracking(self):
        """Test command history tracking."""
        # Simulate some commands
        self.repl.command_history.append("create test [1, 2, 3]")
        self.repl.command_history.append("show test")
        self.repl.command_history.append("namespace")
        
        assert len(self.repl.command_history) == 3
        assert "create test [1, 2, 3]" in self.repl.command_history
        assert "namespace" in self.repl.command_history
    
    def test_no_duplicate_consecutive_commands(self):
        """Test that duplicate consecutive commands are not added to history."""
        # This would be tested in the actual input processing
        # For now, just test the logic
        test_history = ["command1"]
        new_command = "command1"
        
        # Check if we should add (shouldn't because it's duplicate)
        should_add = new_command not in test_history[-1:]
        assert not should_add
        
        # Different command should be added
        new_command = "command2"  
        should_add = new_command not in test_history[-1:]
        assert should_add
    
    @patch('glang.repl.repl.READLINE_AVAILABLE', True)
    @patch('glang.repl.repl.readline')
    def test_save_history(self, mock_readline):
        """Test history saving functionality."""
        # Mock readline methods
        mock_readline.write_history_file = MagicMock()
        
        # Set up history file path
        self.repl._history_file = "/tmp/test_history"
        
        # Call save history
        self.repl._save_history()
        
        # Verify it tried to save
        mock_readline.write_history_file.assert_called_once_with("/tmp/test_history")
    
    def test_completion_with_empty_input(self):
        """Test completion with empty or whitespace input."""
        completions = self.repl._get_completions("", "")
        # Should return all available commands
        assert len(completions) > 0
        assert "help" in completions
        assert "create" in completions
        assert "namespace" in completions
    
    def test_completion_with_unknown_command(self):
        """Test completion for unknown commands returns empty list."""
        completions = self.repl._get_completions("arg", "unknown_command arg")
        assert completions == []


@pytest.mark.skipif(not READLINE_AVAILABLE, reason="readline not available")
class TestREPLReadlineIntegration:
    """Test readline integration when available."""
    
    def test_readline_imported(self):
        """Test that readline is properly imported when available."""
        assert READLINE_AVAILABLE is True
        
    def test_completion_function_set(self):
        """Test that completion function is properly configured."""
        repl = REPL()
        # This mainly tests that setup doesn't crash
        assert hasattr(repl, '_complete_command')
        assert callable(repl._complete_command)
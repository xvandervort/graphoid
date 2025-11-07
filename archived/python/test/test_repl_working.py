"""Working tests for REPL functionality."""

import pytest
from unittest.mock import Mock, patch, MagicMock
from io import StringIO
import tempfile
import os

from src.glang.repl.repl import REPL
from src.glang.execution.pipeline import ExecutionResult
from src.glang.files.errors import FileOperationError


class TestREPLBasics:
    """Test basic REPL functionality."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.repl = REPL()
    
    def test_repl_initialization(self):
        """Test REPL initialization."""
        assert self.repl.prompt == "glang> "
        assert self.repl.running == True
        assert self.repl.file_manager is not None
        assert self.repl.execution_session is not None
        assert isinstance(self.repl.command_history, list)
    
    def test_slash_commands_registered(self):
        """Test that slash commands are properly registered."""
        expected_commands = [
            "help", "h", "version", "ver", "exit", "x", 
            "load", "save", "run", "namespace", "ns", 
            "stats", "clear", "methods", "type", "inspect", "can"
        ]
        
        for cmd in expected_commands:
            assert cmd in self.repl.slash_commands
            assert callable(self.repl.slash_commands[cmd])


class TestREPLSlashCommandHandlers:
    """Test REPL slash command handlers."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.repl = REPL()
    
    def test_handle_help_command(self):
        """Test help command output."""
        with patch('sys.stdout', new=StringIO()) as mock_stdout:
            self.repl._handle_help_command([])
            output = mock_stdout.getvalue()
            
            assert "Glang Modern REPL" in output
            assert "Language Features" in output
            assert "Commands:" in output
            assert "/help" in output
            assert "/exit" in output
    
    def test_handle_version_command(self):
        """Test version command output."""
        with patch('sys.stdout', new=StringIO()) as mock_stdout:
            self.repl._handle_version_command([])
            output = mock_stdout.getvalue()
            
            assert "Glang" in output
            assert "AST-based" in output
    
    def test_handle_exit_command(self):
        """Test exit command."""
        with patch('sys.stdout', new=StringIO()) as mock_stdout:
            assert self.repl.running == True
            self.repl._handle_exit_command([])
            assert self.repl.running == False
            
            output = mock_stdout.getvalue()
            assert "Goodbye!" in output
    
    def test_handle_namespace_command_empty(self):
        """Test namespace command with empty context."""
        # Mock empty execution context
        self.repl.execution_session.execution_context.variables = {}
        
        with patch('sys.stdout', new=StringIO()) as mock_stdout:
            self.repl._handle_namespace_command([])
            output = mock_stdout.getvalue()
            
            assert "No variables" in output or "empty" in output.lower()
    
    def test_handle_namespace_command_with_variables(self):
        """Test namespace command with variables."""
        # Mock execution context with variables using proper GlangValue objects
        from src.glang.execution.values import StringValue, NumberValue
        from src.glang.ast.nodes import SourcePosition
        
        pos = SourcePosition(1, 1)
        mock_variables = {
            "test_var": StringValue("test_value", pos),
            "count": NumberValue(42, pos)
        }
        self.repl.execution_session.execution_context.variables = mock_variables
        
        with patch('sys.stdout', new=StringIO()) as mock_stdout:
            self.repl._handle_namespace_command([])
            output = mock_stdout.getvalue()
            
            assert "test_var" in output or "Variable Namespace" in output
    
    def test_handle_stats_command(self):
        """Test stats command."""
        # Mock the get_session_info to return expected data
        mock_info = {
            'variable_count': 3,
            'symbol_table_size': 5,
            'variables': ['var1', 'var2', 'var3']
        }
        
        with patch.object(self.repl.execution_session, 'get_session_info', return_value=mock_info):
            with patch('sys.stdout', new=StringIO()) as mock_stdout:
                self.repl._handle_stats_command([])
                output = mock_stdout.getvalue()
                
                assert "Variables: 3" in output
                assert "Symbol table entries: 5" in output
                assert "var1" in output
    
    def test_handle_clear_command(self):
        """Test clear command."""
        # Mock the execution session's clear_variables method
        with patch.object(self.repl.execution_session, 'clear_variables') as mock_clear:
            with patch('sys.stdout', new=StringIO()) as mock_stdout:
                self.repl._handle_clear_command([])
                output = mock_stdout.getvalue()
                
                # Should call clear method and show confirmation
                mock_clear.assert_called_once()
                assert "cleared" in output.lower()


class TestREPLFileCommands:
    """Test REPL file operation commands."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.repl = REPL()
    
    def test_handle_load_command_no_args(self):
        """Test load command without arguments."""
        with patch('sys.stdout', new=StringIO()) as mock_stdout:
            self.repl._handle_load_command([])
            output = mock_stdout.getvalue()
            
            assert "Usage:" in output
            assert "/load" in output
            assert "filename" in output.lower()
    
    def test_handle_load_command_success(self):
        """Test successful load command."""
        mock_result = Mock()
        mock_result.success = True
        
        with patch.object(self.repl.file_manager, 'load_file', return_value=mock_result):
            with patch('sys.stdout', new=StringIO()) as mock_stdout:
                self.repl._handle_load_command(["test.gr"])
                output = mock_stdout.getvalue()
                
                assert "Successfully loaded" in output
                assert "test.gr" in output
    
    def test_handle_load_command_failure(self):
        """Test load command with failure."""
        mock_result = Mock()
        mock_result.success = False
        mock_result.error = "File not found"
        
        with patch.object(self.repl.file_manager, 'load_file', return_value=mock_result):
            with patch('sys.stdout', new=StringIO()) as mock_stdout:
                self.repl._handle_load_command(["missing.gr"])
                output = mock_stdout.getvalue()
                
                assert "Failed to load" in output
                assert "File not found" in output
    
    def test_handle_load_command_exception(self):
        """Test load command with exception."""
        with patch.object(self.repl.file_manager, 'load_file', 
                         side_effect=FileOperationError("Permission denied")):
            with patch('sys.stdout', new=StringIO()) as mock_stdout:
                self.repl._handle_load_command(["test.gr"])
                output = mock_stdout.getvalue()
                
                assert "Error loading" in output
                assert "Permission denied" in output
    
    def test_handle_save_command_no_args(self):
        """Test save command without arguments."""
        with patch('sys.stdout', new=StringIO()) as mock_stdout:
            self.repl._handle_save_command([])
            output = mock_stdout.getvalue()
            
            assert "Usage:" in output
            assert "/save" in output
            assert "filename" in output.lower()
    
    def test_handle_save_command_success(self):
        """Test successful save command."""
        with patch.object(self.repl.file_manager, 'save_file', return_value=True):
            with patch('sys.stdout', new=StringIO()) as mock_stdout:
                self.repl._handle_save_command(["output.gr"])
                output = mock_stdout.getvalue()
                
                assert "Successfully saved" in output
                assert "output.gr" in output
    
    def test_handle_save_command_failure(self):
        """Test save command with failure."""
        with patch.object(self.repl.file_manager, 'save_file', return_value=False):
            with patch('sys.stdout', new=StringIO()) as mock_stdout:
                self.repl._handle_save_command(["output.gr"])
                output = mock_stdout.getvalue()
                
                assert "Failed to save" in output
    
    def test_handle_run_command_no_args(self):
        """Test run command without arguments."""
        with patch('sys.stdout', new=StringIO()) as mock_stdout:
            self.repl._handle_run_command([])
            output = mock_stdout.getvalue()
            
            assert "Usage:" in output
            assert "/run" in output
            assert "filename" in output.lower()
    
    def test_handle_run_command_success(self):
        """Test successful run command."""
        mock_result = Mock()
        mock_result.success = True
        mock_result.value = "42"
        
        with patch.object(self.repl.file_manager, 'run_file', return_value=mock_result):
            with patch('sys.stdout', new=StringIO()) as mock_stdout:
                self.repl._handle_run_command(["script.gr"])
                output = mock_stdout.getvalue()
                
                assert "Successfully executed" in output
                assert "script.gr" in output
                assert "Result: 42" in output


class TestREPLInputProcessing:
    """Test REPL input processing."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.repl = REPL()
    
    def test_handle_slash_command_parsing(self):
        """Test slash command parsing."""
        # The slash commands dictionary maps directly to the handler methods
        # We need to mock the actual commands in the dictionary
        original_help = self.repl.slash_commands['help']
        original_load = self.repl.slash_commands['load']
        
        mock_help = Mock()
        mock_load = Mock()
        
        self.repl.slash_commands['help'] = mock_help
        self.repl.slash_commands['load'] = mock_load
        
        try:
            self.repl._handle_slash_command("/help")
            mock_help.assert_called_once_with([])
            
            self.repl._handle_slash_command("/load test.gr")
            mock_load.assert_called_once_with(["test.gr"])
        finally:
            # Restore original handlers
            self.repl.slash_commands['help'] = original_help
            self.repl.slash_commands['load'] = original_load
    
    def test_handle_slash_command_empty(self):
        """Test empty slash command."""
        with patch('sys.stdout', new=StringIO()) as mock_stdout:
            self.repl._handle_slash_command("/")
            output = mock_stdout.getvalue()
            
            assert "Empty command" in output
            assert "help" in output.lower()
    
    def test_handle_slash_command_unknown(self):
        """Test unknown slash command."""
        with patch('sys.stdout', new=StringIO()) as mock_stdout:
            self.repl._handle_slash_command("/unknown_command")
            output = mock_stdout.getvalue()
            
            assert "Unknown command" in output
            assert "unknown_command" in output
    
    def test_process_input_slash_command(self):
        """Test processing input that starts with slash."""
        with patch.object(self.repl, '_handle_slash_command') as mock_handler:
            self.repl._process_input("/help")
            mock_handler.assert_called_once_with("/help")
    
    def test_process_input_regular_statement_success(self):
        """Test processing regular statement with success."""
        mock_result = Mock()
        mock_result.success = True
        mock_result.value = "test_result"
        
        with patch.object(self.repl.execution_session, 'execute_statement', 
                         return_value=mock_result):
            with patch('sys.stdout', new=StringIO()) as mock_stdout:
                self.repl._process_input("string x = \"test\"")
                output = mock_stdout.getvalue()
                
                assert "test_result" in output
    
    def test_process_input_regular_statement_no_output(self):
        """Test processing statement that produces no output."""
        mock_result = Mock()
        mock_result.success = True
        mock_result.value = None
        
        with patch.object(self.repl.execution_session, 'execute_statement', 
                         return_value=mock_result):
            with patch('sys.stdout', new=StringIO()) as mock_stdout:
                self.repl._process_input("string x = \"test\"")
                output = mock_stdout.getvalue()
                
                # Should not print anything for None value
                assert output.strip() == ""
    
    def test_process_input_execution_error(self):
        """Test processing input with execution error."""
        mock_result = Mock()
        mock_result.success = False
        mock_result.error = "Type error"
        mock_result.get_formatted_error.return_value = "Formatted: Type error"
        
        with patch.object(self.repl.execution_session, 'execute_statement', 
                         return_value=mock_result):
            with patch('sys.stdout', new=StringIO()) as mock_stdout:
                self.repl._process_input("invalid_code")
                output = mock_stdout.getvalue()
                
                assert "Formatted: Type error" in output
    
    def test_process_input_unexpected_exception(self):
        """Test processing input with unexpected exception."""
        with patch.object(self.repl.execution_session, 'execute_statement', 
                         side_effect=Exception("Unexpected error")):
            with patch('sys.stdout', new=StringIO()) as mock_stdout:
                self.repl._process_input("some_code")
                output = mock_stdout.getvalue()
                
                assert "Unexpected error" in output


class TestREPLMultilineHandling:
    """Test REPL multiline input handling."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.repl = REPL()
    
    def test_is_statement_complete_simple(self):
        """Test completion check for simple statements."""
        assert self.repl._is_statement_complete("string x = \"test\"") == True
        assert self.repl._is_statement_complete("5 + 3") == True
    
    def test_is_statement_complete_braces(self):
        """Test completion check with braces."""
        assert self.repl._is_statement_complete("if true {") == False
        assert self.repl._is_statement_complete("if true { print(\"test\") }") == True
    
    def test_is_statement_complete_nested_braces(self):
        """Test completion check with nested braces."""
        incomplete = "if true { for item in items {"
        assert self.repl._is_statement_complete(incomplete) == False
        
        complete = "if true { for item in items { print(item) } }"
        assert self.repl._is_statement_complete(complete) == True
    
    def test_is_statement_complete_parentheses(self):
        """Test completion check with parentheses."""
        assert self.repl._is_statement_complete("func(") == False
        assert self.repl._is_statement_complete("func(arg)") == True
    
    def test_is_statement_complete_brackets(self):
        """Test completion check with brackets.""" 
        assert self.repl._is_statement_complete("list = [1, 2") == False
        assert self.repl._is_statement_complete("list = [1, 2, 3]") == True


class TestREPLHistoryAndReadline:
    """Test REPL history and readline functionality."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.repl = REPL()
    
    def test_setup_readline_available(self):
        """Test readline setup when available."""
        # Test that _setup_readline is called during initialization
        with patch('src.glang.repl.repl.READLINE_AVAILABLE', True):
            with patch('readline.parse_and_bind') as mock_bind:
                with patch('readline.set_history_length') as mock_history:
                    # Create new REPL to trigger _setup_readline
                    repl = REPL()
                    
                    # Should be called during initialization
                    mock_bind.assert_called_once_with("tab: complete")
                    mock_history.assert_called_once_with(1000)
    
    @patch('src.glang.repl.repl.READLINE_AVAILABLE', False)
    def test_setup_readline_unavailable(self):
        """Test readline setup when not available."""
        # Should not raise any exception
        self.repl._setup_readline()
    
    def test_save_history_with_readline(self):
        """Test history saving when readline is available."""
        with patch('src.glang.repl.repl.READLINE_AVAILABLE', True):
            with patch('readline.write_history_file') as mock_write:
                with patch('os.path.expanduser', return_value='/test/home'):
                    self.repl._save_history()
                    mock_write.assert_called_once()
    
    def test_load_history_with_readline(self):
        """Test history loading when readline is available."""
        with patch('src.glang.repl.repl.READLINE_AVAILABLE', True):
            with patch('readline.read_history_file') as mock_read:
                with patch('os.path.expanduser', return_value='/test/home'):
                    with patch('os.path.exists', return_value=True):
                        self.repl._load_history()
                        mock_read.assert_called_once()
    
    def test_load_history_file_not_exists(self):
        """Test history loading when file doesn't exist."""
        with patch('src.glang.repl.repl.READLINE_AVAILABLE', True):
            with patch('os.path.exists', return_value=False):
                # Should not raise exception
                self.repl._load_history()


class TestREPLEdgeCases:
    """Test REPL edge cases."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.repl = REPL()
    
    def test_empty_input_handling(self):
        """Test handling of empty input."""
        # Should not cause any issues
        self.repl._process_input("")
    
    def test_whitespace_only_input(self):
        """Test handling of whitespace-only input."""
        self.repl._process_input("   ")
        self.repl._process_input("\t\n")
    
    def test_slash_command_with_extra_spaces(self):
        """Test slash command with extra whitespace."""
        original_help = self.repl.slash_commands['help']
        mock_help = Mock()
        self.repl.slash_commands['help'] = mock_help
        
        try:
            self.repl._handle_slash_command("/help   ")
            mock_help.assert_called_once_with([])
        finally:
            self.repl.slash_commands['help'] = original_help
    
    def test_slash_command_with_multiple_args(self):
        """Test slash command with multiple arguments."""
        original_load = self.repl.slash_commands['load']
        mock_load = Mock()
        self.repl.slash_commands['load'] = mock_load
        
        try:
            self.repl._handle_slash_command("/load file.gr arg1 arg2")
            mock_load.assert_called_once_with(["file.gr", "arg1", "arg2"])
        finally:
            self.repl.slash_commands['load'] = original_load
"""Extended tests for CLI functionality."""

import pytest
import sys
import tempfile
import os
from pathlib import Path
from unittest.mock import Mock, patch, mock_open, MagicMock
from io import StringIO

from src.glang import cli


class TestCliArgumentParsing:
    """Test CLI argument parsing functionality."""
    
    @patch('sys.argv', ['glang', '--version'])
    @patch('sys.exit')
    def test_version_argument(self, mock_exit):
        """Test --version argument."""
        with patch('argparse.ArgumentParser.parse_args') as mock_parse:
            mock_args = Mock()
            mock_args.file = None
            mock_parse.return_value = mock_args
            
            # Mock the version action to simulate argparse behavior
            with patch('argparse.ArgumentParser.add_argument') as mock_add_arg:
                def side_effect(*args, **kwargs):
                    if kwargs.get('action') == 'version':
                        # Simulate argparse version action
                        raise SystemExit(0)
                mock_add_arg.side_effect = side_effect
                
                with pytest.raises(SystemExit):
                    parser = cli.argparse.ArgumentParser()
                    parser.add_argument('--version', action='version', version='test')
    
    def test_argument_parser_creation(self):
        """Test that argument parser is created correctly."""
        # We can test this by calling the argument setup portion
        parser = cli.argparse.ArgumentParser(
            prog="glang",
            description="Glang - A prototype programming language with graphs as first-class objects"
        )
        
        # Add arguments like in main()
        parser.add_argument("--version", action="version", version="test")
        parser.add_argument("file", nargs="?", help="Glang source file")
        parser.add_argument("--args", nargs="*", default=[], help="Arguments")
        parser.add_argument("--verbose", "-v", action="store_true", help="Verbose")
        parser.add_argument("--check-syntax", "-c", action="store_true", help="Check syntax")
        
        # Test parsing various argument combinations
        args = parser.parse_args([])
        assert args.file is None
        assert args.args == []
        assert args.verbose is False
        assert args.check_syntax is False
        
        args = parser.parse_args(['test.gr', '--verbose'])
        assert args.file == 'test.gr'
        assert args.verbose is True
        
        args = parser.parse_args(['test.gr', '--args', 'arg1', 'arg2'])
        assert args.args == ['arg1', 'arg2']


class TestExecuteFile:
    """Test file execution functionality."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.temp_dir = tempfile.mkdtemp()
        self.test_file = os.path.join(self.temp_dir, 'test.gr')
    
    def teardown_method(self):
        """Clean up test fixtures."""
        import shutil
        if os.path.exists(self.temp_dir):
            shutil.rmtree(self.temp_dir)
    
    def test_file_not_found(self):
        """Test error when file doesn't exist."""
        with patch('sys.stderr', new=StringIO()) as mock_stderr:
            exit_code = cli.execute_file('nonexistent.gr', [], False, False)
            
            assert exit_code == 1
            assert "not found" in mock_stderr.getvalue()
    
    def test_path_is_not_file(self):
        """Test error when path is not a file."""
        # Create a directory instead of a file
        dir_path = os.path.join(self.temp_dir, 'test_dir')
        os.makedirs(dir_path)
        
        with patch('sys.stderr', new=StringIO()) as mock_stderr:
            exit_code = cli.execute_file(dir_path, [], False, False)
            
            assert exit_code == 1
            assert "is not a file" in mock_stderr.getvalue()
    
    def test_non_gr_extension_warning(self):
        """Test warning for non-.gr extension."""
        # Create a test file with wrong extension
        wrong_ext_file = os.path.join(self.temp_dir, 'test.txt')
        with open(wrong_ext_file, 'w') as f:
            f.write('string x = "hello"')
        
        with patch('sys.stderr', new=StringIO()) as mock_stderr:
            with patch('src.glang.cli.ExecutionSession') as mock_session:
                mock_session_instance = Mock()
                mock_session.return_value = mock_session_instance
                mock_session_instance.execution_context.set_variable = Mock()
                mock_session_instance.execution_context.symbol_table.declare_symbol = Mock()
                
                with patch('src.glang.cli.parse_multiline_statements') as mock_parse:
                    mock_parse.return_value = ['string x = "hello"']
                    mock_session_instance.execute_statement.return_value = Mock(success=True, value=None)
                    
                    exit_code = cli.execute_file(wrong_ext_file, [], False, False)
                    
                    stderr_output = mock_stderr.getvalue()
                    assert "does not have .gr extension" in stderr_output
    
    def test_file_read_success(self):
        """Test successful file reading."""
        # Create a valid .gr file
        with open(self.test_file, 'w') as f:
            f.write('string greeting = "Hello, World!"')
        
        with patch('src.glang.cli.ExecutionSession') as mock_session:
            mock_session_instance = Mock()
            mock_session.return_value = mock_session_instance
            mock_session_instance.execution_context.set_variable = Mock()
            mock_session_instance.execution_context.symbol_table.declare_symbol = Mock()
            
            with patch('src.glang.cli.parse_multiline_statements') as mock_parse:
                mock_parse.return_value = ['string greeting = "Hello, World!"']
                mock_session_instance.execute_statement.return_value = Mock(success=True, value=None)
                
                exit_code = cli.execute_file(self.test_file, [], False, False)
                
                assert exit_code == 0
                mock_session_instance.execute_statement.assert_called_once()
    
    def test_shebang_handling(self):
        """Test that shebang lines are properly stripped."""
        # Create file with shebang
        with open(self.test_file, 'w') as f:
            f.write('#!/usr/bin/env glang\nstring x = "test"')
        
        with patch('src.glang.cli.ExecutionSession') as mock_session:
            mock_session_instance = Mock()
            mock_session.return_value = mock_session_instance
            mock_session_instance.execution_context.set_variable = Mock()
            mock_session_instance.execution_context.symbol_table.declare_symbol = Mock()
            
            with patch('src.glang.cli.parse_multiline_statements') as mock_parse:
                mock_parse.return_value = ['string x = "test"']
                mock_session_instance.execute_statement.return_value = Mock(success=True, value=None)
                
                exit_code = cli.execute_file(self.test_file, [], False, False)
                
                # Check that parse_multiline_statements was called with content without shebang
                called_content = mock_parse.call_args[0][0]
                assert not called_content.startswith('#!')
                assert 'string x = "test"' in called_content
    
    def test_verbose_output(self):
        """Test verbose output mode."""
        with open(self.test_file, 'w') as f:
            f.write('string x = "test"')
        
        with patch('sys.stdout', new=StringIO()) as mock_stdout:
            with patch('src.glang.cli.ExecutionSession') as mock_session:
                mock_session_instance = Mock()
                mock_session.return_value = mock_session_instance
                mock_session_instance.execution_context.set_variable = Mock()
                mock_session_instance.execution_context.symbol_table.declare_symbol = Mock()
                
                with patch('src.glang.cli.parse_multiline_statements') as mock_parse:
                    mock_parse.return_value = ['string x = "test"']
                    mock_session_instance.execute_statement.return_value = Mock(success=True, value=None)
                    
                    exit_code = cli.execute_file(self.test_file, ['arg1', 'arg2'], True, False)
                    
                    stdout_output = mock_stdout.getvalue()
                    assert "Executing:" in stdout_output
                    assert "Arguments:" in stdout_output
    
    def test_program_arguments_setup(self):
        """Test that program arguments are properly set up."""
        with open(self.test_file, 'w') as f:
            f.write('string x = "test"')
        
        with patch('src.glang.cli.ExecutionSession') as mock_session:
            mock_session_instance = Mock()
            mock_session.return_value = mock_session_instance
            mock_context = Mock()
            mock_session_instance.execution_context = mock_context
            mock_symbol_table = Mock()
            mock_context.symbol_table = mock_symbol_table
            
            with patch('src.glang.cli.parse_multiline_statements') as mock_parse:
                mock_parse.return_value = ['string x = "test"']
                mock_session_instance.execute_statement.return_value = Mock(success=True, value=None)
                
                test_args = ['arg1', 'arg2', 'arg3']
                cli.execute_file(self.test_file, test_args, False, False)
                
                # Verify args variable was set
                mock_context.set_variable.assert_called_once()
                call_args = mock_context.set_variable.call_args
                assert call_args[0][0] == 'args'  # Variable name
                
                # Verify symbol was declared
                mock_symbol_table.declare_symbol.assert_called_once()


class TestSyntaxChecking:
    """Test syntax checking functionality."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.temp_dir = tempfile.mkdtemp()
        self.test_file = os.path.join(self.temp_dir, 'test.gr')
    
    def teardown_method(self):
        """Clean up test fixtures."""
        import shutil
        if os.path.exists(self.temp_dir):
            shutil.rmtree(self.temp_dir)
    
    def test_syntax_check_success(self):
        """Test successful syntax checking."""
        with open(self.test_file, 'w') as f:
            f.write('string x = "hello"\nnum y = 42')
        
        with patch('sys.stdout', new=StringIO()) as mock_stdout:
            with patch('glang.parser.ast_parser.ASTParser') as mock_parser_class:
                with patch('glang.semantic.analyzer.SemanticAnalyzer') as mock_analyzer_class:
                    # Setup mocks
                    mock_parser = Mock()
                    mock_parser_class.return_value = mock_parser
                    mock_analyzer = Mock()
                    mock_analyzer_class.return_value = mock_analyzer
                    
                    # Mock successful parsing and analysis
                    mock_parser.parse.return_value = Mock()  # Mock AST
                    mock_result = Mock()
                    mock_result.success = True
                    mock_analyzer.analyze.return_value = mock_result
                    
                    exit_code = cli.execute_file(self.test_file, [], True, True)
                    
                    assert exit_code == 0
                    stdout_output = mock_stdout.getvalue()
                    assert "Syntax check passed" in stdout_output
    
    def test_syntax_check_parse_error(self):
        """Test syntax checking with parse error."""
        with open(self.test_file, 'w') as f:
            f.write('invalid syntax here')
        
        with patch('sys.stderr', new=StringIO()) as mock_stderr:
            with patch('glang.parser.ast_parser.ASTParser') as mock_parser_class:
                mock_parser = Mock()
                mock_parser_class.return_value = mock_parser
                mock_parser.parse.side_effect = Exception("Parse error")
                
                exit_code = cli.execute_file(self.test_file, [], False, True)
                
                assert exit_code == 1
                stderr_output = mock_stderr.getvalue()
                assert "Parse error" in stderr_output
    
    def test_syntax_check_semantic_error(self):
        """Test syntax checking with semantic error."""
        with open(self.test_file, 'w') as f:
            f.write('unknown_variable')
        
        with patch('sys.stderr', new=StringIO()) as mock_stderr:
            with patch('glang.parser.ast_parser.ASTParser') as mock_parser_class:
                with patch('glang.semantic.analyzer.SemanticAnalyzer') as mock_analyzer_class:
                    # Setup mocks
                    mock_parser = Mock()
                    mock_parser_class.return_value = mock_parser
                    mock_analyzer = Mock()
                    mock_analyzer_class.return_value = mock_analyzer
                    
                    # Mock successful parsing but failed analysis
                    mock_parser.parse.return_value = Mock()
                    mock_result = Mock()
                    mock_result.success = False
                    mock_result.errors = [Mock(__str__=lambda x: "Undefined variable")]
                    mock_analyzer.analyze.return_value = mock_result
                    
                    exit_code = cli.execute_file(self.test_file, [], False, True)
                    
                    assert exit_code == 1
                    stderr_output = mock_stderr.getvalue()
                    assert "Syntax error" in stderr_output


class TestMainFunction:
    """Test main CLI function."""
    
    @patch('src.glang.cli.execute_file')
    @patch('sys.exit')
    def test_main_with_file(self, mock_exit, mock_execute):
        """Test main function with file argument."""
        mock_execute.return_value = 0
        
        with patch('sys.argv', ['glang', 'test.gr']):
            cli.main()
            
            mock_execute.assert_called_once_with('test.gr', [], False, False)
            mock_exit.assert_called_once_with(0)
    
    @patch('src.glang.cli.execute_file')
    @patch('sys.exit')
    def test_main_with_file_and_args(self, mock_exit, mock_execute):
        """Test main function with file and arguments."""
        mock_execute.return_value = 0
        
        with patch('sys.argv', ['glang', 'test.gr', '--args', 'arg1', 'arg2', '--verbose']):
            cli.main()
            
            mock_execute.assert_called_once_with('test.gr', ['arg1', 'arg2'], True, False)
    
    @patch('src.glang.cli.REPL')
    def test_main_without_file(self, mock_repl_class):
        """Test main function without file (REPL mode)."""
        mock_repl = Mock()
        mock_repl_class.return_value = mock_repl
        
        with patch('sys.argv', ['glang']):
            cli.main()
            
            mock_repl_class.assert_called_once()
            mock_repl.run.assert_called_once()


class TestFileReadingAndExecution:
    """Test file reading and execution details."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.temp_dir = tempfile.mkdtemp()
        self.test_file = os.path.join(self.temp_dir, 'test.gr')
    
    def teardown_method(self):
        """Clean up test fixtures."""
        import shutil
        if os.path.exists(self.temp_dir):
            shutil.rmtree(self.temp_dir)
    
    def test_file_encoding_utf8(self):
        """Test that files are read with UTF-8 encoding."""
        # Create file with UTF-8 content
        with open(self.test_file, 'w', encoding='utf-8') as f:
            f.write('string msg = "Hello 世界!"')
        
        with patch('src.glang.cli.ExecutionSession') as mock_session:
            mock_session_instance = Mock()
            mock_session.return_value = mock_session_instance
            mock_session_instance.execution_context.set_variable = Mock()
            mock_session_instance.execution_context.symbol_table.declare_symbol = Mock()
            
            with patch('src.glang.cli.parse_multiline_statements') as mock_parse:
                mock_parse.return_value = ['string msg = "Hello 世界!"']
                mock_session_instance.execute_statement.return_value = Mock(success=True, value=None)
                
                exit_code = cli.execute_file(self.test_file, [], False, False)
                
                assert exit_code == 0
                # Check that the content was passed correctly
                called_content = mock_parse.call_args[0][0]
                assert "世界" in called_content
    
    def test_execution_error_handling(self):
        """Test handling of execution errors."""
        with open(self.test_file, 'w') as f:
            f.write('invalid_statement')
        
        with patch('sys.stderr', new=StringIO()) as mock_stderr:
            with patch('src.glang.cli.ExecutionSession') as mock_session:
                mock_session_instance = Mock()
                mock_session.return_value = mock_session_instance
                mock_session_instance.execution_context.set_variable = Mock()
                mock_session_instance.execution_context.symbol_table.declare_symbol = Mock()
                
                with patch('src.glang.cli.parse_multiline_statements') as mock_parse:
                    mock_parse.return_value = ['invalid_statement']
                    
                    # Mock execution failure
                    mock_result = Mock()
                    mock_result.success = False
                    mock_result.error = "Execution error"
                    mock_session_instance.execute_statement.return_value = mock_result
                    
                    exit_code = cli.execute_file(self.test_file, [], False, False)
                    
                    assert exit_code == 1
                    stderr_output = mock_stderr.getvalue()
                    assert "Execution error" in stderr_output


class TestCliEdgeCases:
    """Test edge cases for CLI functionality."""
    
    def test_empty_file(self):
        """Test execution of empty file."""
        temp_dir = tempfile.mkdtemp()
        try:
            empty_file = os.path.join(temp_dir, 'empty.gr')
            with open(empty_file, 'w') as f:
                f.write('')
            
            with patch('src.glang.cli.parse_multiline_statements') as mock_parse:
                mock_parse.return_value = []  # No statements
                
                exit_code = cli.execute_file(empty_file, [], False, False)
                
                assert exit_code == 0  # Empty file should succeed
        finally:
            import shutil
            shutil.rmtree(temp_dir)
    
    def test_file_with_only_whitespace(self):
        """Test file with only whitespace."""
        temp_dir = tempfile.mkdtemp()
        try:
            whitespace_file = os.path.join(temp_dir, 'whitespace.gr')
            with open(whitespace_file, 'w') as f:
                f.write('   \n  \n\t\n  ')
            
            with patch('src.glang.cli.parse_multiline_statements') as mock_parse:
                mock_parse.return_value = []  # No meaningful statements
                
                exit_code = cli.execute_file(whitespace_file, [], False, False)
                
                assert exit_code == 0
        finally:
            import shutil
            shutil.rmtree(temp_dir)
    
    def test_multiple_shebang_lines(self):
        """Test file with multiple lines starting with #."""
        temp_dir = tempfile.mkdtemp()
        try:
            multi_shebang_file = os.path.join(temp_dir, 'multi.gr')
            with open(multi_shebang_file, 'w') as f:
                f.write('#!/usr/bin/env glang\n# This is a comment\nstring x = "test"')
            
            with patch('src.glang.cli.ExecutionSession') as mock_session:
                mock_session_instance = Mock()
                mock_session.return_value = mock_session_instance
                mock_session_instance.execution_context.set_variable = Mock()
                mock_session_instance.execution_context.symbol_table.declare_symbol = Mock()
                
                with patch('src.glang.cli.parse_multiline_statements') as mock_parse:
                    mock_parse.return_value = ['# This is a comment', 'string x = "test"']
                    mock_session_instance.execute_statement.return_value = Mock(success=True, value=None)
                    
                    exit_code = cli.execute_file(multi_shebang_file, [], False, False)
                    
                    # Should only strip the first shebang line
                    called_content = mock_parse.call_args[0][0]
                    assert not called_content.startswith('#!/usr/bin/env glang')
                    assert '# This is a comment' in called_content
        finally:
            import shutil
            shutil.rmtree(temp_dir)
"""
Tests for the Glang CLI functionality.
"""

import pytest
from unittest.mock import patch, MagicMock
import sys
from glang.cli import main


class TestCLI:
    """Test cases for the CLI functionality."""
    
    @patch('glang.cli.REPL')
    @patch('sys.argv', ['glang'])
    def test_main_no_args_starts_repl(self, mock_repl_class):
        """Test that main() with no arguments starts the REPL."""
        mock_repl = MagicMock()
        mock_repl_class.return_value = mock_repl
        
        main()
        
        mock_repl_class.assert_called_once()
        mock_repl.start.assert_called_once()
    
    @patch('sys.argv', ['glang', '--version'])
    def test_version_argument(self, capsys):
        """Test --version argument."""
        with pytest.raises(SystemExit) as excinfo:
            main()
        assert excinfo.value.code == 0
    
    @patch('sys.argv', ['glang', 'test.gl'])
    @patch('builtins.print')
    def test_file_argument_not_implemented(self, mock_print):
        """Test that file execution shows not implemented message."""
        with pytest.raises(SystemExit) as excinfo:
            main()
        assert excinfo.value.code == 1
        mock_print.assert_called_with("File execution not yet implemented: test.gl")
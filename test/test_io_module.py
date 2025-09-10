"""
Test the I/O module functionality
"""

import os
import tempfile
import pytest
from pathlib import Path

from glang.execution.pipeline import ExecutionSession
from glang.execution.values import StringValue, BooleanValue, NumberValue, ListValue


class TestIOModule:
    """Test I/O module operations."""
    
    def setup_method(self):
        """Set up test environment."""
        self.session = ExecutionSession()
        # Create temporary directory for test files
        self.temp_dir = tempfile.mkdtemp(prefix="glang_test_")
        self.test_file = os.path.join(self.temp_dir, "test.txt")
        self.test_content = "Hello, Glang!"
        
    def teardown_method(self):
        """Clean up test environment."""
        # Remove test files
        import shutil
        if os.path.exists(self.temp_dir):
            shutil.rmtree(self.temp_dir)
    
    def test_import_io_module(self):
        """Test that the io module can be imported."""
        result = self.session.execute_statement('import "io"')
        assert result.success
        
        # Check that io module is available
        result = self.session.execute_statement("io")
        assert result.success
    
    def test_write_and_read_file(self):
        """Test writing and reading a file."""
        # Import io module
        result = self.session.execute_statement('import "io"')
        assert result.success
        
        # Write to file
        write_code = f'''io.write_file("{self.test_file}", "{self.test_content}")'''
        result = self.session.execute_statement(write_code)
        assert result.success
        assert isinstance(result.value, BooleanValue)
        assert result.value.value is True
        
        # Read from file
        read_code = f'''io.read_file("{self.test_file}")'''
        result = self.session.execute_statement(read_code)
        assert result.success
        assert isinstance(result.value, StringValue)
        assert result.value.value == self.test_content
    
    def test_append_file(self):
        """Test appending to a file."""
        result = self.session.execute_statement('import "io"')
        assert result.success
        
        # Write initial content
        write_code = f'''io.write_file("{self.test_file}", "Line 1\\n")'''
        result = self.session.execute_statement(write_code)
        assert result.success
        
        # Append more content
        append_code = f'''io.append_file("{self.test_file}", "Line 2\\n")'''
        result = self.session.execute_statement(append_code)
        assert result.success
        assert result.value.value is True
        
        # Read and verify
        read_code = f'''io.read_file("{self.test_file}")'''
        result = self.session.execute_statement(read_code)
        assert result.success
        assert result.value.value == "Line 1\nLine 2\n"
    
    def test_file_exists(self):
        """Test checking if file exists."""
        result = self.session.execute_statement('import "io"')
        assert result.success
        
        # Check non-existent file
        check_code = f'''io.exists("{self.test_file}")'''
        result = self.session.execute_statement(check_code)
        assert result.success
        assert isinstance(result.value, BooleanValue)
        assert result.value.value is False
        
        # Create file
        write_code = f'''io.write_file("{self.test_file}", "test")'''
        result = self.session.execute_statement(write_code)
        assert result.success
        
        # Check existing file
        result = self.session.execute_statement(check_code)
        assert result.success
        assert result.value.value is True
    
    def test_is_file_and_is_dir(self):
        """Test checking file and directory types."""
        result = self.session.execute_statement('import "io"')
        assert result.success
        
        # Create a file
        write_code = f'''io.write_file("{self.test_file}", "test")'''
        result = self.session.execute_statement(write_code)
        assert result.success
        
        # Check is_file
        check_file = f'''io.is_file("{self.test_file}")'''
        result = self.session.execute_statement(check_file)
        assert result.success
        assert result.value.value is True
        
        # Check is_dir on file
        check_dir = f'''io.is_dir("{self.test_file}")'''
        result = self.session.execute_statement(check_dir)
        assert result.success
        assert result.value.value is False
        
        # Check is_dir on directory
        check_dir = f'''io.is_dir("{self.temp_dir}")'''
        result = self.session.execute_statement(check_dir)
        assert result.success
        assert result.value.value is True
    
    def test_list_dir(self):
        """Test listing directory contents."""
        result = self.session.execute_statement('import "io"')
        assert result.success
        
        # Create some files
        for i in range(3):
            file_path = os.path.join(self.temp_dir, f"file{i}.txt")
            write_code = f'''io.write_file("{file_path}", "content {i}")'''
            result = self.session.execute_statement(write_code)
            assert result.success
        
        # List directory
        list_code = f'''io.list_dir("{self.temp_dir}")'''
        result = self.session.execute_statement(list_code)
        assert result.success
        assert isinstance(result.value, ListValue)
        
        # Check files are listed
        file_names = [elem.value for elem in result.value.elements]
        assert "file0.txt" in file_names
        assert "file1.txt" in file_names
        assert "file2.txt" in file_names
    
    def test_make_dir(self):
        """Test creating directories."""
        result = self.session.execute_statement('import "io"')
        assert result.success
        
        # Create directory
        new_dir = os.path.join(self.temp_dir, "subdir")
        make_dir_code = f'''io.make_dir("{new_dir}")'''
        result = self.session.execute_statement(make_dir_code)
        assert result.success
        assert result.value.value is True
        
        # Verify directory exists
        check_code = f'''io.is_dir("{new_dir}")'''
        result = self.session.execute_statement(check_code)
        assert result.success
        assert result.value.value is True
        
        # Create nested directories
        nested_dir = os.path.join(self.temp_dir, "a", "b", "c")
        make_nested_code = f'''io.make_dir("{nested_dir}")'''
        result = self.session.execute_statement(make_nested_code)
        assert result.success
        assert result.value.value is True
        
        # Verify nested directory exists
        check_nested = f'''io.is_dir("{nested_dir}")'''
        result = self.session.execute_statement(check_nested)
        assert result.success
        assert result.value.value is True
    
    def test_remove_file(self):
        """Test removing files."""
        result = self.session.execute_statement('import "io"')
        assert result.success
        
        # Create a file
        write_code = f'''io.write_file("{self.test_file}", "test")'''
        result = self.session.execute_statement(write_code)
        assert result.success
        
        # Remove the file
        remove_code = f'''io.remove_file("{self.test_file}")'''
        result = self.session.execute_statement(remove_code)
        assert result.success
        assert result.value.value is True
        
        # Verify file is gone
        check_code = f'''io.exists("{self.test_file}")'''
        result = self.session.execute_statement(check_code)
        assert result.success
        assert result.value.value is False
    
    def test_remove_dir(self):
        """Test removing directories."""
        result = self.session.execute_statement('import "io"')
        assert result.success
        
        # Create an empty directory
        empty_dir = os.path.join(self.temp_dir, "empty")
        make_dir_code = f'''io.make_dir("{empty_dir}")'''
        result = self.session.execute_statement(make_dir_code)
        assert result.success
        
        # Remove the directory
        remove_code = f'''io.remove_dir("{empty_dir}")'''
        result = self.session.execute_statement(remove_code)
        assert result.success
        assert result.value.value is True
        
        # Verify directory is gone
        check_code = f'''io.exists("{empty_dir}")'''
        result = self.session.execute_statement(check_code)
        assert result.success
        assert result.value.value is False
    
    def test_file_size(self):
        """Test getting file size."""
        result = self.session.execute_statement('import "io"')
        assert result.success
        
        # Create a file with known content
        content = "Hello, World!"  # 13 bytes
        write_code = f'''io.write_file("{self.test_file}", "{content}")'''
        result = self.session.execute_statement(write_code)
        assert result.success
        
        # Get file size
        size_code = f'''io.file_size("{self.test_file}")'''
        result = self.session.execute_statement(size_code)
        assert result.success
        assert isinstance(result.value, NumberValue)
        assert result.value.value == len(content)
    
    def test_read_lines(self):
        """Test reading file as lines."""
        result = self.session.execute_statement('import "io"')
        assert result.success
        
        # Create a multi-line file
        content = "Line 1\\nLine 2\\nLine 3"
        write_code = f'''io.write_file("{self.test_file}", "{content}")'''
        result = self.session.execute_statement(write_code)
        assert result.success
        
        # Read lines
        read_lines_code = f'''io.read_lines("{self.test_file}")'''
        result = self.session.execute_statement(read_lines_code)
        assert result.success
        assert isinstance(result.value, ListValue)
        assert len(result.value.elements) == 3
        
        # Check line contents
        lines = [elem.value for elem in result.value.elements]
        assert lines[0] == "Line 1"
        assert lines[1] == "Line 2"
        assert lines[2] == "Line 3"
    
    def test_write_lines(self):
        """Test writing lines to file."""
        result = self.session.execute_statement('import "io"')
        assert result.success
        
        # Create a list of lines
        create_list = 'lines = ["First", "Second", "Third"]'
        result = self.session.execute_statement(create_list)
        assert result.success
        
        # Write lines to file
        write_lines_code = f'''io.write_lines("{self.test_file}", lines)'''
        result = self.session.execute_statement(write_lines_code)
        assert result.success
        assert result.value.value is True
        
        # Read back and verify
        read_code = f'''io.read_file("{self.test_file}")'''
        result = self.session.execute_statement(read_code)
        assert result.success
        assert result.value.value == "First\nSecond\nThird"
    
    def test_get_cwd(self):
        """Test getting current working directory."""
        result = self.session.execute_statement('import "io"')
        assert result.success
        
        # Get current directory
        get_cwd_code = 'io.get_cwd()'
        result = self.session.execute_statement(get_cwd_code)
        assert result.success
        assert isinstance(result.value, StringValue)
        # Should return a valid path
        assert os.path.isdir(result.value.value)
    
    def test_io_with_variables(self):
        """Test I/O operations using variables."""
        result = self.session.execute_statement('import "io"')
        assert result.success
        
        # Store filename in variable
        filename_code = f'filename = "{self.test_file}"'
        result = self.session.execute_statement(filename_code)
        assert result.success
        
        # Store content in variable  
        content_code = 'content = "Variable content"'
        result = self.session.execute_statement(content_code)
        assert result.success
        
        # Write using variables
        write_code = 'io.write_file(filename, content)'
        result = self.session.execute_statement(write_code)
        assert result.success
        
        # Read using variable
        read_code = 'result = io.read_file(filename)'
        result = self.session.execute_statement(read_code)
        assert result.success
        
        # Check result variable
        check_code = 'result'
        result = self.session.execute_statement(check_code)
        assert result.success
        assert result.value.value == "Variable content"
    
    def test_error_handling(self):
        """Test error handling in I/O operations."""
        result = self.session.execute_statement('import "io"')
        assert result.success
        
        # Try to read non-existent file
        read_code = 'io.read_file("/nonexistent/file.txt")'
        result = self.session.execute_statement(read_code)
        assert not result.success
        assert "File not found" in str(result.error)
        
        # Try to remove non-existent file
        remove_code = 'io.remove_file("/nonexistent/file.txt")'
        result = self.session.execute_statement(remove_code)
        assert not result.success
        assert "File not found" in str(result.error)
        
        # Try to remove non-empty directory
        # Create directory with file
        dir_path = os.path.join(self.temp_dir, "nonempty")
        os.makedirs(dir_path, exist_ok=True)
        file_in_dir = os.path.join(dir_path, "file.txt")
        with open(file_in_dir, 'w') as f:
            f.write("content")
        
        remove_dir_code = f'io.remove_dir("{dir_path}")'
        result = self.session.execute_statement(remove_dir_code)
        assert not result.success
        assert "Directory not empty" in str(result.error)
"""
Test file handle operations for incremental file I/O.
"""

import pytest
import os
import tempfile
from pathlib import Path

from glang.execution.pipeline import ExecutionSession
from glang.execution.values import StringValue, BooleanValue, NumberValue


class TestFileHandles:
    """Test file handle operations."""
    
    def setup_method(self):
        """Set up test environment."""
        self.session = ExecutionSession()
        self.temp_dir = tempfile.mkdtemp(prefix="glang_test_filehandles_")
    
    def teardown_method(self):
        """Clean up test environment."""
        # Clean up temp directory
        import shutil
        shutil.rmtree(self.temp_dir, ignore_errors=True)
    
    def test_open_write_close(self):
        """Test opening, writing to, and closing a file."""
        test_file = os.path.join(self.temp_dir, "test_write.txt")
        
        # Import io module
        result = self.session.execute_statement('import "io" as io')
        assert result.success
        
        # Open file for writing
        result = self.session.execute_statement(f'file = io.open("{test_file}", "w")')
        assert result.success
        
        # Write to file
        result = self.session.execute_statement('file.write("Hello, World!")')
        assert result.success
        
        # Close file
        result = self.session.execute_statement('file.close()')
        assert result.success
        
        # Verify file was written
        with open(test_file, 'r') as f:
            content = f.read()
        assert content == "Hello, World!"
    
    def test_incremental_write(self):
        """Test incremental writing to a file."""
        test_file = os.path.join(self.temp_dir, "test_incremental.txt")
        
        # Import io module
        result = self.session.execute_statement('import "io" as io')
        assert result.success
        
        # Open file for writing
        result = self.session.execute_statement(f'file = io.open("{test_file}", "w")')
        assert result.success
        
        # Write lines incrementally
        result = self.session.execute_statement('file.write("Line 1\\n")')
        assert result.success
        result = self.session.execute_statement('file.write("Line 2\\n")')
        assert result.success
        result = self.session.execute_statement('file.write("Line 3\\n")')
        assert result.success
        
        # Flush and close
        result = self.session.execute_statement('file.flush()')
        assert result.success
        result = self.session.execute_statement('file.close()')
        assert result.success
        
        # Verify file was written
        with open(test_file, 'r') as f:
            lines = f.readlines()
        assert lines == ["Line 1\n", "Line 2\n", "Line 3\n"]
    
    def test_append_mode(self):
        """Test appending to an existing file."""
        test_file = os.path.join(self.temp_dir, "test_append.txt")
        
        # First write some initial content
        with open(test_file, 'w') as f:
            f.write("Initial content\n")
        
        # Import io module
        result = self.session.execute_statement('import "io" as io')
        assert result.success
        
        # Open file for appending
        result = self.session.execute_statement(f'file = io.open("{test_file}", "a")')
        assert result.success
        
        # Append lines
        result = self.session.execute_statement('file.write("Appended line 1\\n")')
        assert result.success
        result = self.session.execute_statement('file.write("Appended line 2\\n")')
        assert result.success
        
        # Close file
        result = self.session.execute_statement('file.close()')
        assert result.success
        
        # Verify file was appended
        with open(test_file, 'r') as f:
            lines = f.readlines()
        assert lines == ["Initial content\n", "Appended line 1\n", "Appended line 2\n"]
    
    def test_read_mode(self):
        """Test reading from a file."""
        test_file = os.path.join(self.temp_dir, "test_read.txt")
        
        # Create a file to read
        with open(test_file, 'w') as f:
            f.write("Line 1\nLine 2\nLine 3\n")
        
        # Import io module
        result = self.session.execute_statement('import "io" as io')
        assert result.success
        
        # Open and read file
        result = self.session.execute_statement(f'file = io.open("{test_file}", "r")')
        assert result.success
        result = self.session.execute_statement('content = file.read()')
        assert result.success
        result = self.session.execute_statement('file.close()')
        assert result.success
        
        # Verify content
        result = self.session.execute_statement('content')
        assert result.success
        assert result.value.value == "Line 1\nLine 2\nLine 3\n"
    
    def test_read_line(self):
        """Test reading lines from a file."""
        test_file = os.path.join(self.temp_dir, "test_readline.txt")
        
        # Create a file to read
        with open(test_file, 'w') as f:
            f.write("Line 1\nLine 2\nLine 3\n")
        
        # Import io module
        result = self.session.execute_statement('import "io" as io')
        assert result.success
        
        # Open file and read lines
        result = self.session.execute_statement(f'file = io.open("{test_file}", "r")')
        assert result.success
        result = self.session.execute_statement('line1 = file.read_line()')
        assert result.success
        result = self.session.execute_statement('line2 = file.read_line()')
        assert result.success
        result = self.session.execute_statement('line3 = file.read_line()')
        assert result.success
        result = self.session.execute_statement('file.close()')
        assert result.success
        
        # Check the lines read
        result = self.session.execute_statement('line1')
        assert result.value.value == "Line 1"
        result = self.session.execute_statement('line2')
        assert result.value.value == "Line 2"
        result = self.session.execute_statement('line3')
        assert result.value.value == "Line 3"
    
    def test_file_handle_type(self):
        """Test that file handles have the correct type."""
        test_file = os.path.join(self.temp_dir, "test_type.txt")
        
        # Import io module
        result = self.session.execute_statement('import "io" as io')
        assert result.success
        
        # Open file and check type
        result = self.session.execute_statement(f'file = io.open("{test_file}", "w")')
        assert result.success
        result = self.session.execute_statement('file_type = file.type()')
        assert result.success
        result = self.session.execute_statement('file.close()')
        assert result.success
        
        # Verify type
        result = self.session.execute_statement('file_type')
        assert result.value.value == "file"
    
    def test_capability_reactivation(self):
        """Test that capabilities can be reactivated after close (new boundary capability semantics)."""
        test_file = os.path.join(self.temp_dir, "test_reactivate.txt")
        
        # Import io module
        result = self.session.execute_statement('import "io" as io')
        assert result.success
        
        # Create write capability and use it
        result = self.session.execute_statement(f'write_cap = io.open("{test_file}", "w")')
        assert result.success
        result = self.session.execute_statement('write_cap.write("First write")')
        assert result.success
        result = self.session.execute_statement('write_cap.close()')
        assert result.success
        
        # Capability can be reactivated (lazy initialization)
        result = self.session.execute_statement('write_cap.write("Second write")')
        assert result.success
        result = self.session.execute_statement('write_cap.close()')
        assert result.success
        
        # Verify the file contains the data
        with open(test_file, 'r') as f:
            content = f.read()
        # Note: Second write overwrites because it's a "w" capability
        assert content == "Second write"
    
    def test_lazy_file_not_found_error(self):
        """Test that file-not-found errors occur on first boundary operation (lazy initialization)."""
        test_file = os.path.join(self.temp_dir, "nonexistent.txt")
        
        # Import io module
        result = self.session.execute_statement('import "io" as io')
        assert result.success
        
        # Creating capability succeeds (lazy initialization)
        result = self.session.execute_statement(f'read_cap = io.open("{test_file}", "r")')
        assert result.success
        
        # Error occurs on first boundary operation
        result = self.session.execute_statement('content = read_cap.read()')
        assert not result.success
        assert "cannot activate" in str(result.error).lower() or "not found" in str(result.error).lower()
    
    def test_capability_type_constraints(self):
        """Test that capabilities enforce unidirectional constraints."""
        test_file = os.path.join(self.temp_dir, "test_constraints.txt")
        
        # Create file first
        with open(test_file, 'w') as f:
            f.write("test data\n")
        
        # Import io module
        result = self.session.execute_statement('import "io" as io')
        assert result.success
        
        # Read capability cannot write
        result = self.session.execute_statement(f'read_cap = io.open("{test_file}", "r")')
        assert result.success
        result = self.session.execute_statement('read_cap.write("fail")')
        assert not result.success
        assert "cannot write" in str(result.error).lower()
        
        # Write capability cannot read
        result = self.session.execute_statement(f'write_cap = io.open("{test_file}", "w")')
        assert result.success  
        result = self.session.execute_statement('content = write_cap.read()')
        assert not result.success
        assert "cannot read" in str(result.error).lower()
        
        # Test capability introspection
        result = self.session.execute_statement('read_type = read_cap.capability_type()')
        assert result.success
        result = self.session.execute_statement('read_type')
        assert result.value.value == "read"
        
        result = self.session.execute_statement('write_type = write_cap.capability_type()')
        assert result.success
        result = self.session.execute_statement('write_type')
        assert result.value.value == "write"
    
    def test_write_non_string_values(self):
        """Test writing non-string values to a file."""
        test_file = os.path.join(self.temp_dir, "test_nonstring.txt")
        
        # Import io module
        result = self.session.execute_statement('import "io" as io')
        assert result.success
        
        # Open file and write different types
        result = self.session.execute_statement(f'file = io.open("{test_file}", "w")')
        assert result.success
        result = self.session.execute_statement('file.write(42)')
        assert result.success
        result = self.session.execute_statement('file.write("\\n")')
        assert result.success
        result = self.session.execute_statement('file.write(true)')
        assert result.success
        result = self.session.execute_statement('file.write("\\n")')
        assert result.success
        result = self.session.execute_statement('file.write([1, 2, 3])')
        assert result.success
        result = self.session.execute_statement('file.write("\\n")')
        assert result.success
        result = self.session.execute_statement('file.close()')
        assert result.success
        
        # Verify file was written with string representations
        with open(test_file, 'r') as f:
            lines = f.readlines()
        assert lines == ["42\n", "true\n", "[1, 2, 3]\n"]
    
    def test_read_auto_close_on_eof(self):
        """Test that read capabilities auto-close when EOF is reached."""
        test_file = os.path.join(self.temp_dir, "test_auto_close.txt")
        
        # Create a test file
        with open(test_file, 'w') as f:
            f.write("content")
        
        # Import io module
        result = self.session.execute_statement('import "io" as io')
        assert result.success
        
        # Open for reading and read all content (hits EOF)
        result = self.session.execute_statement(f'read_cap = io.open("{test_file}", "r")')
        assert result.success
        result = self.session.execute_statement('content = read_cap.read()')
        assert result.success
        
        # Verify content was read correctly
        result = self.session.execute_statement('content')
        assert result.success
        assert result.value.value == "content"
        
        # Attempting to read again should fail (capability exhausted)
        result = self.session.execute_statement('read_cap.read()')
        assert not result.success
        assert "exhausted" in str(result.error) or "EOF" in str(result.error)
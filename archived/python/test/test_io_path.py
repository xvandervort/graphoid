"""Tests for path manipulation I/O operations."""

import pytest
import os
from glang.execution.pipeline import ExecutionSession


class TestIOPathOperations:
    """Test path manipulation I/O operations."""
    
    def setup_method(self):
        """Set up test environment."""
        self.session = ExecutionSession()
        # Import IO module
        result = self.session.execute_statement('import "io"')
        assert result.success, f"Failed to import IO module: {result}"
    
    def test_join_path(self):
        """Test joining path components."""
        # Test basic path joining
        result = self.session.execute_statement('parts = ["home", "user", "documents", "file.txt"]')
        assert result.success
        
        result = self.session.execute_statement('joined = io.join_path(parts)')
        assert result.success
        
        result = self.session.execute_statement('joined')
        assert result.success
        expected = os.path.join("home", "user", "documents", "file.txt")
        assert result.value.value == expected
    
    def test_join_path_empty_list(self):
        """Test joining empty path list."""
        result = self.session.execute_statement('empty_parts = []')
        assert result.success
        
        result = self.session.execute_statement('empty_joined = io.join_path(empty_parts)')
        assert result.success
        
        result = self.session.execute_statement('empty_joined')
        assert result.success
        assert result.value.value == ""
    
    def test_join_path_single_component(self):
        """Test joining single path component."""
        result = self.session.execute_statement('single_part = ["document.txt"]')
        assert result.success
        
        result = self.session.execute_statement('single_joined = io.join_path(single_part)')
        assert result.success
        
        result = self.session.execute_statement('single_joined')
        assert result.success
        assert result.value.value == "document.txt"
    
    def test_split_path(self):
        """Test splitting path into directory and filename."""
        # Test absolute path
        result = self.session.execute_statement('full_path = "/home/user/document.txt"')
        assert result.success
        
        result = self.session.execute_statement('parts = io.split_path(full_path)')
        assert result.success
        
        result = self.session.execute_statement('parts')
        assert result.success
        parts_list = result.value
        assert len(parts_list.elements) == 2
        assert parts_list.elements[0].value == "/home/user"
        assert parts_list.elements[1].value == "document.txt"
    
    def test_split_path_no_directory(self):
        """Test splitting path with no directory component."""
        result = self.session.execute_statement('filename_only = "document.txt"')
        assert result.success
        
        result = self.session.execute_statement('parts = io.split_path(filename_only)')
        assert result.success
        
        result = self.session.execute_statement('parts')
        assert result.success
        parts_list = result.value
        assert len(parts_list.elements) == 2
        assert parts_list.elements[0].value == ""
        assert parts_list.elements[1].value == "document.txt"
    
    def test_basename(self):
        """Test getting basename (filename) from path."""
        result = self.session.execute_statement('full_path = "/home/user/document.txt"')
        assert result.success
        
        result = self.session.execute_statement('basename = io.basename(full_path)')
        assert result.success
        
        result = self.session.execute_statement('basename')
        assert result.success
        assert result.value.value == "document.txt"
    
    def test_basename_no_directory(self):
        """Test getting basename from filename only."""
        result = self.session.execute_statement('filename = "document.txt"')
        assert result.success
        
        result = self.session.execute_statement('basename = io.basename(filename)')
        assert result.success
        
        result = self.session.execute_statement('basename')
        assert result.success
        assert result.value.value == "document.txt"
    
    def test_dirname(self):
        """Test getting directory name from path."""
        result = self.session.execute_statement('full_path = "/home/user/document.txt"')
        assert result.success
        
        result = self.session.execute_statement('dirname = io.dirname(full_path)')
        assert result.success
        
        result = self.session.execute_statement('dirname')
        assert result.success
        assert result.value.value == "/home/user"
    
    def test_dirname_no_directory(self):
        """Test getting directory from filename only."""
        result = self.session.execute_statement('filename = "document.txt"')
        assert result.success
        
        result = self.session.execute_statement('dirname = io.dirname(filename)')
        assert result.success
        
        result = self.session.execute_statement('dirname')
        assert result.success
        assert result.value.value == ""
    
    def test_extension(self):
        """Test getting file extension."""
        result = self.session.execute_statement('filename = "document.txt"')
        assert result.success
        
        result = self.session.execute_statement('ext = io.extension(filename)')
        assert result.success
        
        result = self.session.execute_statement('ext')
        assert result.success
        assert result.value.value == ".txt"
    
    def test_extension_no_extension(self):
        """Test getting extension from file without extension."""
        result = self.session.execute_statement('filename = "document"')
        assert result.success
        
        result = self.session.execute_statement('ext = io.extension(filename)')
        assert result.success
        
        result = self.session.execute_statement('ext')
        assert result.success
        assert result.value.value == ""
    
    def test_extension_multiple_dots(self):
        """Test getting extension with multiple dots."""
        result = self.session.execute_statement('filename = "archive.tar.gz"')
        assert result.success
        
        result = self.session.execute_statement('ext = io.extension(filename)')
        assert result.success
        
        result = self.session.execute_statement('ext')
        assert result.success
        assert result.value.value == ".gz"
    
    def test_resolve_path(self):
        """Test resolving path to absolute form."""
        result = self.session.execute_statement('rel_path = "document.txt"')
        assert result.success
        
        result = self.session.execute_statement('abs_path = io.resolve_path(rel_path)')
        assert result.success
        
        result = self.session.execute_statement('abs_path')
        assert result.success
        # Should be absolute path
        abs_path = result.value.value
        assert os.path.isabs(abs_path)
        assert abs_path.endswith("document.txt")
    
    def test_resolve_path_relative_with_dots(self):
        """Test resolving path with relative components."""
        result = self.session.execute_statement('rel_path = "../test/document.txt"')
        assert result.success
        
        result = self.session.execute_statement('abs_path = io.resolve_path(rel_path)')
        assert result.success
        
        result = self.session.execute_statement('abs_path')
        assert result.success
        # Should be absolute path without relative components
        abs_path = result.value.value
        assert os.path.isabs(abs_path)
        assert ".." not in abs_path
    
    def test_path_operations_integration(self):
        """Test combining multiple path operations."""
        # Create a complex path and manipulate it
        result = self.session.execute_statement('base_parts = ["home", "user", "projects"]')
        assert result.success
        
        result = self.session.execute_statement('base_path = io.join_path(base_parts)')
        assert result.success
        
        # Add filename
        result = self.session.execute_statement('full_parts = ["home", "user", "projects", "myfile.tar.gz"]')
        assert result.success
        
        result = self.session.execute_statement('full_path = io.join_path(full_parts)')
        assert result.success
        
        # Extract components
        result = self.session.execute_statement('dirname = io.dirname(full_path)')
        assert result.success
        
        result = self.session.execute_statement('basename = io.basename(full_path)')
        assert result.success
        
        result = self.session.execute_statement('extension = io.extension(full_path)')
        assert result.success
        
        # Verify results
        result = self.session.execute_statement('dirname')
        assert result.success
        expected_dir = os.path.join("home", "user", "projects")
        assert result.value.value == expected_dir
        
        result = self.session.execute_statement('basename')
        assert result.success
        assert result.value.value == "myfile.tar.gz"
        
        result = self.session.execute_statement('extension')
        assert result.success
        assert result.value.value == ".gz"
    
    def test_join_path_invalid_input(self):
        """Test error handling for invalid input to join_path."""
        # Try to pass a string instead of a list
        result = self.session.execute_statement('bad_join = io.join_path("not a list")')
        assert not result.success
        assert "expects list of paths" in str(result.error)
    
    def test_path_functions_invalid_input(self):
        """Test error handling for invalid input types."""
        # Test with number instead of string
        result = self.session.execute_statement('bad_split = io.split_path(123)')
        assert not result.success
        assert "expects string" in str(result.error)
        
        result = self.session.execute_statement('bad_basename = io.basename(123)')
        assert not result.success
        assert "expects string" in str(result.error)
        
        result = self.session.execute_statement('bad_dirname = io.dirname(123)')
        assert not result.success
        assert "expects string" in str(result.error)
        
        result = self.session.execute_statement('bad_ext = io.extension(123)')
        assert not result.success
        assert "expects string" in str(result.error)
        
        result = self.session.execute_statement('bad_resolve = io.resolve_path(123)')
        assert not result.success
        assert "expects string" in str(result.error)
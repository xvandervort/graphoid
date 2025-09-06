"""Test that namespace display works correctly after file imports."""

import pytest
import tempfile
import os
from glang.execution.pipeline import ExecutionSession
from glang.files import FileManager


class TestNamespaceDisplay:
    """Test namespace display functionality."""
    
    def setup_method(self):
        """Set up test environment."""
        self.session = ExecutionSession()
        self.file_manager = FileManager()
    
    def test_empty_namespace(self):
        """Test namespace when no variables are defined."""
        variables = self.session.execution_context.variables
        assert len(variables) == 0
    
    def test_namespace_after_variable_declaration(self):
        """Test namespace after declaring variables."""
        # Declare variables
        result = self.session.execute_statement('string name = "Alice"')
        assert result.success
        
        result = self.session.execute_statement('num age = 25')
        assert result.success
        
        # Check namespace
        variables = self.session.execution_context.variables
        assert len(variables) == 2
        assert 'name' in variables
        assert 'age' in variables
        assert str(variables['name']) == "Alice"
        assert str(variables['age']) == "25"
    
    def test_namespace_after_file_load(self):
        """Test that namespace correctly shows variables from loaded file."""
        # Create a temporary file with variables
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('string greeting = "Hello from file"\n')
            f.write('num value = 42\n')
            f.write('list items = ["a", "b", "c"]\n')
            temp_file = f.name
        
        try:
            # Load the file
            result = self.file_manager.load_file(temp_file, self.session)
            assert result.success
            
            # Check namespace includes variables from file
            variables = self.session.execution_context.variables
            assert len(variables) == 3
            assert 'greeting' in variables
            assert 'value' in variables  
            assert 'items' in variables
            
            # Verify values
            assert str(variables['greeting']) == "Hello from file"
            assert str(variables['value']) == "42"
            assert str(variables['items']) == "[a, b, c]"
            
        finally:
            os.unlink(temp_file)
    
    def test_namespace_preserves_existing_variables_after_load(self):
        """Test that existing variables are preserved when loading a file."""
        # Declare some variables first
        result = self.session.execute_statement('string local_var = "local"')
        assert result.success
        
        result = self.session.execute_statement('num local_num = 100')
        assert result.success
        
        # Create a file with additional variables
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('string file_var = "from file"\n')
            f.write('num file_num = 200\n')
            temp_file = f.name
        
        try:
            # Load the file
            result = self.file_manager.load_file(temp_file, self.session)
            assert result.success
            
            # Check namespace includes both local and file variables
            variables = self.session.execution_context.variables
            assert len(variables) == 4
            
            # Local variables should still be there
            assert 'local_var' in variables
            assert 'local_num' in variables
            assert str(variables['local_var']) == "local"
            assert str(variables['local_num']) == "100"
            
            # File variables should be added
            assert 'file_var' in variables
            assert 'file_num' in variables
            assert str(variables['file_var']) == "from file"
            assert str(variables['file_num']) == "200"
            
        finally:
            os.unlink(temp_file)
    
    def test_namespace_after_failed_file_load(self):
        """Test that namespace remains intact after failed file load."""
        # Declare a variable
        result = self.session.execute_statement('string test_var = "should remain"')
        assert result.success
        
        # Try to load non-existent file (should raise exception)
        from glang.files.errors import FileOperationError
        with pytest.raises(FileOperationError):
            self.file_manager.load_file('/nonexistent/file.gr', self.session)
        
        # Original variable should still be there
        variables = self.session.execution_context.variables
        assert len(variables) == 1
        assert 'test_var' in variables
        assert str(variables['test_var']) == "should remain"
    
    def test_namespace_after_partial_file_load(self):
        """Test namespace when file partially loads (some statements succeed, some fail)."""
        # Create a file with mixed valid and invalid statements
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('string good_var = "this works"\n')
            f.write('invalid syntax here!\n')
            f.write('num another_var = 42\n')
            temp_file = f.name
        
        try:
            # Declare a variable first
            result = self.session.execute_statement('string original = "original"')
            assert result.success
            
            # Load the problematic file (should raise exception on invalid syntax)
            from glang.files.errors import FileOperationError
            with pytest.raises(FileOperationError):
                self.file_manager.load_file(temp_file, self.session)
            
            # Check that successful statements were processed before the error
            variables = self.session.execution_context.variables
            assert 'original' in variables  # Original variable preserved
            assert 'good_var' in variables  # Good statement was processed
            
            # Verify values
            assert str(variables['original']) == "original"
            assert str(variables['good_var']) == "this works"
            
            # The invalid statement should have prevented further processing
            assert 'another_var' not in variables  # This shouldn't be processed
            
        finally:
            os.unlink(temp_file)
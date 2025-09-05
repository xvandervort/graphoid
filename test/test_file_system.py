"""Tests for the glang file import system."""

import pytest
import os
import tempfile
from pathlib import Path

from glang.files import FileManager, FileOperationError, FileNotFoundError, InvalidFileFormatError
from glang.files.serializer import NamespaceSerializer
from glang.execution import ExecutionSession


class TestFileManager:
    """Test FileManager functionality."""
    
    def setup_method(self):
        self.file_manager = FileManager()
        self.session = ExecutionSession()
        self.temp_dir = tempfile.mkdtemp()
    
    def teardown_method(self):
        """Clean up temporary files."""
        import shutil
        shutil.rmtree(self.temp_dir, ignore_errors=True)
    
    def create_temp_file(self, content: str, filename: str = "test.gr") -> str:
        """Create a temporary .gr file with given content."""
        filepath = os.path.join(self.temp_dir, filename)
        with open(filepath, 'w') as f:
            f.write(content)
        return filepath
    
    def test_load_simple_file(self):
        """Test loading a simple .gr file."""
        content = '''# Test file
string greeting = "hello"
num count = 42
bool active = true
'''
        filepath = self.create_temp_file(content)
        
        result = self.file_manager.load_file(filepath, self.session)
        
        assert result.success
        assert "Successfully loaded 3 statements" in str(result.value)
        
        # Check variables were created
        greeting = self.session.execution_context.get_variable('greeting')
        count = self.session.execution_context.get_variable('count')
        active = self.session.execution_context.get_variable('active')
        
        assert greeting.value == "hello"
        assert count.value == 42
        assert active.value == True
    
    def test_load_file_with_type_inference(self):
        """Test loading file with type inference syntax."""
        content = '''# Type inference test
name = "Alice"
age = 25
items = ["a", "b", "c"]
'''
        filepath = self.create_temp_file(content)
        
        result = self.file_manager.load_file(filepath, self.session)
        
        assert result.success
        
        name = self.session.execution_context.get_variable('name')
        age = self.session.execution_context.get_variable('age')
        items = self.session.execution_context.get_variable('items')
        
        assert name.value == "Alice"
        assert name.get_type() == "string"
        assert age.value == 25
        assert age.get_type() == "num"
        assert len(items.elements) == 3
        assert items.get_type() == "list"
    
    def test_load_file_with_method_calls(self):
        """Test loading file with method calls."""
        content = '''# Method calls test
list numbers = [1, 2, 3]
numbers.append(4)
numbers.append(5)
'''
        filepath = self.create_temp_file(content)
        
        result = self.file_manager.load_file(filepath, self.session)
        
        assert result.success
        
        numbers = self.session.execution_context.get_variable('numbers')
        assert len(numbers.elements) == 5
        assert numbers.elements[3].value == 4
        assert numbers.elements[4].value == 5
    
    def test_load_file_with_comments_and_empty_lines(self):
        """Test that comments and empty lines are properly handled."""
        content = '''# Header comment

string message = "test"

# Another comment
num value = 123

# Final comment
'''
        filepath = self.create_temp_file(content)
        
        result = self.file_manager.load_file(filepath, self.session)
        
        assert result.success
        assert "Successfully loaded 2 statements" in str(result.value)
        
        message = self.session.execution_context.get_variable('message')
        value = self.session.execution_context.get_variable('value')
        
        assert message.value == "test"
        assert value.value == 123
    
    def test_run_file_fresh_namespace(self):
        """Test that run_file uses a fresh namespace."""
        content = '''string temp_var = "temporary"'''
        filepath = self.create_temp_file(content)
        
        # Add variable to main session
        self.session.execute_statement('string main_var = "main"')
        
        result = self.file_manager.run_file(filepath)
        
        assert result.success
        
        # Main session should still have main_var but not temp_var
        assert self.session.execution_context.has_variable('main_var')
        assert not self.session.execution_context.has_variable('temp_var')
    
    def test_save_file(self):
        """Test saving session to file."""
        # Create some variables
        self.session.execute_statement('string name = "Bob"')
        self.session.execute_statement('num age = 30')
        self.session.execute_statement('list items = ["x", "y"]')
        
        filepath = os.path.join(self.temp_dir, "output.gr")
        
        success = self.file_manager.save_file(filepath, self.session)
        
        assert success
        assert os.path.exists(filepath)
        
        # Read and verify content
        with open(filepath, 'r') as f:
            content = f.read()
        
        assert "# Glang program file" in content
        assert 'string name = "Bob"' in content
        assert 'num age = 30' in content
        assert 'list items = ["x", "y"]' in content
    
    def test_file_not_found_error(self):
        """Test error handling for missing files."""
        with pytest.raises(FileNotFoundError):
            self.file_manager.load_file("nonexistent.gr", self.session)
    
    def test_invalid_file_extension(self):
        """Test error handling for invalid file extensions."""
        filepath = self.create_temp_file("content", "test.txt")
        
        # Manually set extension validation by using absolute path without auto-resolution
        original_resolve = self.file_manager._resolve_path
        self.file_manager._resolve_path = lambda p: p  # Skip path resolution
        
        try:
            with pytest.raises(InvalidFileFormatError):
                self.file_manager.load_file(filepath, self.session)
        finally:
            self.file_manager._resolve_path = original_resolve
    
    def test_path_resolution(self):
        """Test that paths are resolved correctly."""
        # Test that .gr extension is added automatically
        content = "string test = \"value\""
        base_filepath = self.create_temp_file(content, "test.gr")
        
        # Remove .gr extension from test path
        no_ext_path = base_filepath[:-3]
        
        result = self.file_manager.load_file(no_ext_path, self.session)
        assert result.success
    
    def test_syntax_error_in_file(self):
        """Test handling of syntax errors in .gr files."""
        content = '''# File with syntax error
string valid = "ok"
invalid syntax here!
string another = "valid"
'''
        filepath = self.create_temp_file(content)
        
        with pytest.raises(FileOperationError) as exc_info:
            self.file_manager.load_file(filepath, self.session)
        
        assert "line 2" in str(exc_info.value)  # Line 2 because comments are skipped


class TestNamespaceSerializer:
    """Test NamespaceSerializer functionality."""
    
    def setup_method(self):
        self.serializer = NamespaceSerializer()
        self.session = ExecutionSession()
    
    def test_serialize_empty_namespace(self):
        """Test serializing empty namespace."""
        content = self.serializer.serialize_namespace(self.session)
        
        assert "# Glang program file" in content
        assert "# No variables defined" in content
    
    def test_serialize_basic_variables(self):
        """Test serializing basic variable types."""
        self.session.execute_statement('string name = "Alice"')
        self.session.execute_statement('num age = 25')
        self.session.execute_statement('bool active = true')
        
        content = self.serializer.serialize_namespace(self.session)
        
        assert 'string name = "Alice"' in content
        assert 'num age = 25' in content
        assert 'bool active = true' in content
    
    def test_serialize_lists(self):
        """Test serializing list variables."""
        self.session.execute_statement('list items = ["a", "b", "c"]')
        self.session.execute_statement('list<num> numbers = [1, 2, 3]')
        
        content = self.serializer.serialize_namespace(self.session)
        
        assert 'list items = ["a", "b", "c"]' in content
        assert 'list<num> numbers = [1, 2, 3]' in content
    
    def test_serialize_boolean_values(self):
        """Test serializing boolean values correctly."""
        self.session.execute_statement('bool flag_true = true')
        self.session.execute_statement('bool flag_false = false')
        
        content = self.serializer.serialize_namespace(self.session)
        
        assert 'bool flag_true = true' in content
        assert 'bool flag_false = false' in content
    
    def test_generate_example_file(self):
        """Test example file generation."""
        content = self.serializer.generate_example_file()
        
        assert "# Example Glang program file" in content
        assert 'string greeting = "Hello, World!"' in content
        assert "list fruits" in content
        assert "list<num> scores" in content
        assert "fruits.append" in content


class TestFileSystemIntegration:
    """Test integration of file system with full execution pipeline."""
    
    def setup_method(self):
        self.temp_dir = tempfile.mkdtemp()
    
    def teardown_method(self):
        import shutil
        shutil.rmtree(self.temp_dir, ignore_errors=True)
    
    def test_round_trip_save_load(self):
        """Test saving and loading preserves variables correctly."""
        session1 = ExecutionSession()
        session1.execute_statement('string message = "test"')
        session1.execute_statement('num value = 42')
        session1.execute_statement('list data = ["x", "y"]')
        
        # Save to file
        file_manager = FileManager()
        filepath = os.path.join(self.temp_dir, "roundtrip.gr")
        file_manager.save_file(filepath, session1)
        
        # Load in new session
        session2 = ExecutionSession()
        result = file_manager.load_file(filepath, session2)
        
        assert result.success
        
        # Verify variables match
        message1 = session1.execution_context.get_variable('message')
        message2 = session2.execution_context.get_variable('message')
        assert message1.value == message2.value
        
        value1 = session1.execution_context.get_variable('value')
        value2 = session2.execution_context.get_variable('value')
        assert value1.value == value2.value
        
        data1 = session1.execution_context.get_variable('data')
        data2 = session2.execution_context.get_variable('data')
        assert len(data1.elements) == len(data2.elements)
        assert data1.elements[0].value == data2.elements[0].value
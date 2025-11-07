"""Tests for binary file I/O operations."""

import pytest
import tempfile
import os
from glang.execution.pipeline import ExecutionSession


class TestIOBinaryOperations:
    """Test binary file I/O operations."""
    
    def setup_method(self):
        """Set up test environment."""
        self.session = ExecutionSession()
        # Import IO module
        result = self.session.execute_statement('import "io"')
        assert result.success, f"Failed to import IO module: {result}"
    
    def test_write_and_read_binary_ascii(self):
        """Test writing and reading binary data (ASCII string)."""
        # Create binary data for "Hello" in ASCII
        result = self.session.execute_statement('hello_bytes = [72, 101, 108, 108, 111]')
        assert result.success
        
        # Write binary file
        with tempfile.NamedTemporaryFile(delete=False) as tmp_file:
            tmp_path = tmp_file.name
        
        try:
            result = self.session.execute_statement(f'success = io.write_binary("{tmp_path}", hello_bytes)')
            assert result.success
            
            # Check the success variable we created
            result = self.session.execute_statement('success')
            assert result.success
            assert result.value.value is True
            
            # Read binary file back
            result = self.session.execute_statement(f'read_bytes = io.read_binary("{tmp_path}")')
            assert result.success
            
            # Check that we got the same data back
            result = self.session.execute_statement('read_bytes')
            assert result.success
            read_data = result.value
            assert len(read_data.elements) == 5
            assert read_data.elements[0].value == 72   # 'H'
            assert read_data.elements[1].value == 101  # 'e'
            assert read_data.elements[2].value == 108  # 'l'
            assert read_data.elements[3].value == 108  # 'l'
            assert read_data.elements[4].value == 111  # 'o'
            
        finally:
            if os.path.exists(tmp_path):
                os.unlink(tmp_path)
    
    def test_write_binary_empty_file(self):
        """Test writing empty binary data."""
        result = self.session.execute_statement('empty_bytes = []')
        assert result.success
        
        with tempfile.NamedTemporaryFile(delete=False) as tmp_file:
            tmp_path = tmp_file.name
        
        try:
            result = self.session.execute_statement(f'success = io.write_binary("{tmp_path}", empty_bytes)')
            assert result.success
            
            # Check the success variable
            result = self.session.execute_statement('success')
            assert result.success
            assert result.value.value is True
            
            # Read back empty file
            result = self.session.execute_statement(f'read_bytes = io.read_binary("{tmp_path}")')
            assert result.success
            
            result = self.session.execute_statement('read_bytes')
            assert result.success
            assert len(result.value.elements) == 0
            
        finally:
            if os.path.exists(tmp_path):
                os.unlink(tmp_path)
    
    def test_write_binary_large_data(self):
        """Test writing larger binary data."""
        # Create data with all byte values 0-255
        result = self.session.execute_statement('large_data = []')
        assert result.success
        
        # Add bytes 0-255 to the list
        for i in range(256):
            result = self.session.execute_statement(f'large_data.append({i})')
            assert result.success
        
        with tempfile.NamedTemporaryFile(delete=False) as tmp_file:
            tmp_path = tmp_file.name
        
        try:
            result = self.session.execute_statement(f'success = io.write_binary("{tmp_path}", large_data)')
            assert result.success
            
            # Check the success variable
            result = self.session.execute_statement('success')
            assert result.success
            assert result.value.value is True
            
            # Read back and verify
            result = self.session.execute_statement(f'read_bytes = io.read_binary("{tmp_path}")')
            assert result.success
            
            result = self.session.execute_statement('read_bytes')
            assert result.success
            read_data = result.value
            assert len(read_data.elements) == 256
            
            # Check a few specific values
            assert read_data.elements[0].value == 0
            assert read_data.elements[127].value == 127
            assert read_data.elements[255].value == 255
            
        finally:
            if os.path.exists(tmp_path):
                os.unlink(tmp_path)
    
    def test_read_binary_nonexistent_file(self):
        """Test error handling for non-existent file."""
        result = self.session.execute_statement('data = io.read_binary("nonexistent_file.bin")')
        assert not result.success
        assert "File not found" in str(result.error)
    
    def test_write_binary_invalid_data_type(self):
        """Test error handling for invalid data types."""
        # Try to write a string instead of a list
        result = self.session.execute_statement('success = io.write_binary("test.bin", "not a list")')
        assert not result.success
        assert "expects list of bytes" in str(result.error)
    
    def test_write_binary_invalid_byte_values(self):
        """Test error handling for invalid byte values."""
        # Test negative value
        result = self.session.execute_statement('invalid_bytes = [-1, 100, 200]')
        assert result.success
        
        with tempfile.NamedTemporaryFile(delete=False) as tmp_file:
            tmp_path = tmp_file.name
        
        try:
            result = self.session.execute_statement(f'success = io.write_binary("{tmp_path}", invalid_bytes)')
            assert not result.success
            assert "Byte values must be 0-255" in str(result.error)
            
        finally:
            if os.path.exists(tmp_path):
                os.unlink(tmp_path)
        
        # Test value too large
        result = self.session.execute_statement('invalid_bytes2 = [100, 256, 200]')
        assert result.success
        
        with tempfile.NamedTemporaryFile(delete=False) as tmp_file:
            tmp_path = tmp_file.name
        
        try:
            result = self.session.execute_statement(f'success = io.write_binary("{tmp_path}", invalid_bytes2)')
            assert not result.success
            assert "Byte values must be 0-255" in str(result.error)
            
        finally:
            if os.path.exists(tmp_path):
                os.unlink(tmp_path)
    
    def test_write_binary_mixed_types_in_list(self):
        """Test error handling for mixed types in byte list."""
        result = self.session.execute_statement('mixed_data = [72, "not a number", 101]')
        assert result.success
        
        with tempfile.NamedTemporaryFile(delete=False) as tmp_file:
            tmp_path = tmp_file.name
        
        try:
            result = self.session.execute_statement(f'success = io.write_binary("{tmp_path}", mixed_data)')
            assert not result.success
            assert "Binary data must be list of numbers" in str(result.error)
            
        finally:
            if os.path.exists(tmp_path):
                os.unlink(tmp_path)
    
    def test_binary_round_trip_preservation(self):
        """Test that binary data is preserved exactly through write/read cycle."""
        # Create specific byte pattern
        result = self.session.execute_statement('pattern = [0, 1, 127, 128, 254, 255, 42, 200]')
        assert result.success
        
        with tempfile.NamedTemporaryFile(delete=False) as tmp_file:
            tmp_path = tmp_file.name
        
        try:
            # Write and read back
            result = self.session.execute_statement(f'io.write_binary("{tmp_path}", pattern)')
            assert result.success
            
            result = self.session.execute_statement(f'restored = io.read_binary("{tmp_path}")')
            assert result.success
            
            # Verify exact match
            result = self.session.execute_statement('restored')
            assert result.success
            restored_data = result.value
            
            expected = [0, 1, 127, 128, 254, 255, 42, 200]
            assert len(restored_data.elements) == len(expected)
            for i, expected_byte in enumerate(expected):
                assert restored_data.elements[i].value == expected_byte
                
        finally:
            if os.path.exists(tmp_path):
                os.unlink(tmp_path)
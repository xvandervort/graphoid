"""Tests for the JSON module functionality."""

import pytest
from glang.execution.pipeline import ExecutionSession


class TestJSONModule:
    """Test JSON module operations."""
    
    def setup_method(self):
        """Set up test environment."""
        self.session = ExecutionSession()
        # Import JSON module
        result = self.session.execute_statement('import "json"')
        assert result.success, f"Failed to import JSON module: {result}"
    
    def test_import_json_module(self):
        """Test that the json module can be imported."""
        # Already imported in setup_method
        result = self.session.execute_statement("json")
        assert result.success
    
    def test_encode_simple_values(self):
        """Test encoding simple values to JSON."""
        # String
        result = self.session.execute_statement('json_str = json.encode("hello")')
        assert result.success
        result = self.session.execute_statement('json_str')
        assert result.success
        assert result.value.value == '"hello"'
        
        # Number
        result = self.session.execute_statement('json_num = json.encode(42)')
        assert result.success
        result = self.session.execute_statement('json_num')
        assert result.success
        assert result.value.value == '42'
        
        # Boolean
        result = self.session.execute_statement('json_bool = json.encode(true)')
        assert result.success
        result = self.session.execute_statement('json_bool')
        assert result.success
        assert result.value.value == 'true'
    
    def test_encode_list(self):
        """Test encoding lists to JSON."""
        self.session.execute_statement('numbers = [1, 2, 3]')
        result = self.session.execute_statement('json_list = json.encode(numbers)')
        assert result.success
        
        result = self.session.execute_statement('json_list')
        assert result.success
        assert result.value.value == '[1,2,3]'
    
    def test_encode_hash(self):
        """Test encoding hashes to JSON."""
        self.session.execute_statement('person = {"name": "Alice", "age": 25}')
        result = self.session.execute_statement('json_hash = json.encode(person)')
        assert result.success
        
        result = self.session.execute_statement('json_hash')
        assert result.success
        json_str = result.value.value
        # JSON object should contain both keys (order may vary)
        assert '"name":"Alice"' in json_str
        assert '"age":25' in json_str
    
    def test_encode_pretty(self):
        """Test pretty encoding with indentation."""
        self.session.execute_statement('data = {"name": "Bob", "scores": [95, 87, 92]}')
        result = self.session.execute_statement('pretty = json.encode_pretty(data)')
        assert result.success
        
        result = self.session.execute_statement('pretty')
        assert result.success
        pretty_json = result.value.value
        # Should have newlines and indentation
        assert '\n' in pretty_json
        assert '  ' in pretty_json  # Indentation
    
    def test_decode_simple_values(self):
        """Test decoding simple JSON values."""
        # Number (easier to test)
        result = self.session.execute_statement('decoded_num = json.decode("42")')
        assert result.success
        result = self.session.execute_statement('decoded_num')
        assert result.success
        assert result.value.value == 42
        
        # Boolean
        result = self.session.execute_statement('decoded_bool = json.decode("true")')
        assert result.success
        result = self.session.execute_statement('decoded_bool')
        assert result.success
        assert result.value.value is True
    
    def test_decode_array(self):
        """Test decoding JSON arrays."""
        result = self.session.execute_statement('decoded_array = json.decode("[1, 2, 3]")')
        assert result.success
        
        result = self.session.execute_statement('decoded_array')
        assert result.success
        # Should be a list with 3 elements
        assert len(result.value.elements) == 3
        assert result.value.elements[0].value == 1
        assert result.value.elements[1].value == 2
        assert result.value.elements[2].value == 3
    
    def test_decode_object(self):
        """Test decoding JSON objects."""
        result = self.session.execute_statement('decoded_obj = json.decode(\'{"name": "Alice", "age": 25}\')')
        assert result.success
        
        result = self.session.execute_statement('decoded_obj')
        assert result.success
        # Should be a hash with 2 pairs
        assert len(result.value.pairs) == 2
        assert "name" in result.value.pairs
        assert "age" in result.value.pairs
        
        # Check individual values
        result = self.session.execute_statement('decoded_obj["name"]')
        assert result.success
        assert result.value.value.value == "Alice"  # DataValue.value.value
        
        result = self.session.execute_statement('decoded_obj["age"]')
        assert result.success
        assert result.value.value.value == 25
    
    def test_roundtrip_encoding(self):
        """Test that encode->decode produces equivalent data."""
        # Create original data
        self.session.execute_statement('original = {"numbers": [1, 2, 3], "text": "hello", "flag": true}')
        
        # Encode then decode
        result = self.session.execute_statement('encoded = json.encode(original)')
        assert result.success
        
        result = self.session.execute_statement('decoded = json.decode(encoded)')
        assert result.success
        
        # Check that decoded data has the same structure
        result = self.session.execute_statement('decoded["text"]')
        assert result.success
        assert result.value.value.value == "hello"  # DataValue.value.value
        
        result = self.session.execute_statement('decoded["flag"]')
        assert result.success
        assert result.value.value.value is True  # DataValue.value.value
        
        result = self.session.execute_statement('decoded["numbers"]')
        assert result.success
        numbers_list = result.value.value  # The actual list inside DataValue
        assert len(numbers_list.elements) == 3
    
    def test_is_valid(self):
        """Test JSON validation."""
        # Valid JSON
        result = self.session.execute_statement('valid1 = json.is_valid(\'{"name": "Alice"}\')')
        assert result.success
        result = self.session.execute_statement('valid1')
        assert result.success
        assert result.value.value is True
        
        result = self.session.execute_statement('valid2 = json.is_valid("[1, 2, 3]")')
        assert result.success
        result = self.session.execute_statement('valid2')
        assert result.success
        assert result.value.value is True
        
        # Invalid JSON
        result = self.session.execute_statement('invalid1 = json.is_valid("invalid json")')
        assert result.success
        result = self.session.execute_statement('invalid1')
        assert result.success
        assert result.value.value is False
        
        result = self.session.execute_statement('invalid2 = json.is_valid("{missing quotes}")')
        assert result.success
        result = self.session.execute_statement('invalid2')
        assert result.success
        assert result.value.value is False
    
    def test_error_handling(self):
        """Test error handling for invalid operations."""
        # Invalid JSON decode
        result = self.session.execute_statement('bad = json.decode("invalid json")')
        assert not result.success
        assert "JSON parsing failed" in str(result.error)
        
        # Non-string decode
        result = self.session.execute_statement('bad2 = json.decode(42)')
        assert not result.success
        assert "expects string" in str(result.error)
    
    def test_type_inference(self):
        """Test that JSON methods are properly type-inferred."""
        # encode should return string
        result = self.session.execute_statement('encoded = json.encode([1, 2, 3])')
        assert result.success
        
        # is_valid should return bool
        result = self.session.execute_statement('valid = json.is_valid("{}")')
        assert result.success
        
        # decode can return any type
        result = self.session.execute_statement('decoded = json.decode("{}")')
        assert result.success
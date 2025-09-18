"""
Built-in JSON module for Glang

Provides JSON encoding and decoding functionality.
"""

import json
from typing import Optional, Any, Dict, List, Union
from pathlib import Path

from ..execution.values import (
    GlangValue, StringValue, BooleanValue, NumberValue,
    DataValue, NoneValue
)
from ..execution.graph_values import ListValue, HashValue
from ..execution.errors import RuntimeError
from ..ast.nodes import SourcePosition


class JSONModule:
    """Built-in JSON module providing JSON encoding and decoding operations."""
    
    @staticmethod
    def encode(data: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Encode a Glang value to JSON string.
        
        Usage in Glang:
            json_str = json.encode({"name": "Alice", "age": 25})
        """
        try:
            python_data = JSONModule._glang_to_python(data, position)
            json_string = json.dumps(python_data, ensure_ascii=False, separators=(',', ':'))
            return StringValue(json_string, position)
        except Exception as e:
            raise RuntimeError(f"JSON encoding failed: {str(e)}", position)
    
    @staticmethod
    def encode_pretty(data: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Encode a Glang value to pretty-formatted JSON string.
        
        Usage in Glang:
            pretty_json = json.encode_pretty({"name": "Alice", "age": 25})
        """
        try:
            python_data = JSONModule._glang_to_python(data, position)
            json_string = json.dumps(python_data, ensure_ascii=False, indent=2)
            return StringValue(json_string, position)
        except Exception as e:
            raise RuntimeError(f"JSON pretty encoding failed: {str(e)}", position)
    
    @staticmethod
    def decode(json_str: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Decode a JSON string to Glang value.
        
        Usage in Glang:
            data = json.decode('{"name": "Alice", "age": 25}')
        """
        if not isinstance(json_str, StringValue):
            raise RuntimeError(
                f"json.decode expects string, got {json_str.get_type()}",
                position
            )
        
        try:
            python_data = json.loads(json_str.value)
            return JSONModule._python_to_glang(python_data, position)
        except json.JSONDecodeError as e:
            raise RuntimeError(f"JSON parsing failed: {str(e)}", position)
        except Exception as e:
            raise RuntimeError(f"JSON decoding failed: {str(e)}", position)
    
    @staticmethod
    def is_valid(json_str: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Check if a string is valid JSON.
        
        Usage in Glang:
            valid = json.is_valid('{"name": "Alice"}')  # true
            invalid = json.is_valid('invalid json')     # false
        """
        if not isinstance(json_str, StringValue):
            return BooleanValue(False, position)
        
        try:
            json.loads(json_str.value)
            return BooleanValue(True, position)
        except (json.JSONDecodeError, TypeError):
            return BooleanValue(False, position)
    
    @staticmethod
    def _glang_to_python(value: GlangValue, position: Optional[SourcePosition] = None) -> Any:
        """Convert Glang value to Python object for JSON serialization."""
        if isinstance(value, StringValue):
            return value.value
        elif isinstance(value, NumberValue):
            return value.value
        elif isinstance(value, BooleanValue):
            return value.value
        elif isinstance(value, NoneValue):
            return None
        elif isinstance(value, (ListValue, ListValue)):
            return [JSONModule._glang_to_python(item, position) for item in value.elements]
        elif isinstance(value, HashValue):
            # Convert hash/map to dictionary
            result = {}
            for key, glang_value in value.pairs.items():
                json_value = JSONModule._glang_to_python(glang_value, position)
                result[key] = json_value
            return result
        elif isinstance(value, HashValue):
            # Convert graph hash to dictionary
            result = {}
            for key, glang_value in value.items():
                json_value = JSONModule._glang_to_python(glang_value, position)
                result[key] = json_value
            return result
        elif isinstance(value, DataValue):
            # Data node as key-value pair
            return {
                value.key: JSONModule._glang_to_python(value.value, position)
            }
        else:
            # For unknown types, try to convert to string
            return str(value.to_display_string())
    
    @staticmethod
    def _python_to_glang(data: Any, position: Optional[SourcePosition] = None) -> GlangValue:
        """Convert Python object from JSON to Glang value."""
        if isinstance(data, str):
            return StringValue(data, position)
        elif isinstance(data, bool):
            # Check bool before int/float since bool is subclass of int in Python
            return BooleanValue(data, position)
        elif isinstance(data, (int, float)):
            return NumberValue(data, position)
        elif data is None:
            return NoneValue(position)
        elif isinstance(data, list):
            # Convert to Glang list
            glang_elements = [JSONModule._python_to_glang(item, position) for item in data]
            # Infer element type from first element, default to 'any'
            element_type = 'any'
            if glang_elements and len(glang_elements) > 0:
                first_type = glang_elements[0].get_type()
                # Check if all elements have the same type
                if all(elem.get_type() == first_type for elem in glang_elements):
                    element_type = first_type
            return ListValue(glang_elements, element_type, position)
        elif isinstance(data, dict):
            # Convert to Glang hash (map)
            pairs = []
            for key, value in data.items():
                if not isinstance(key, str):
                    key = str(key)  # Convert non-string keys to strings
                glang_value = JSONModule._python_to_glang(value, position)
                pairs.append((key, glang_value))
            
            # Infer value type from first element, default to 'any'
            value_type = 'any'
            if pairs:
                first_value = pairs[0][1]
                value_type = first_value.get_type()
                # Check if all values have the same type
                if all(pair[1].get_type() == value_type for pair in pairs):
                    pass  # Keep the inferred type
                else:
                    value_type = 'any'  # Mixed types
            
            return HashValue(pairs, value_type, position)
        else:
            # Unknown type - convert to string
            return StringValue(str(data), position)


def create_json_module_namespace():
    """Create the namespace for the built-in JSON module."""
    from .module_builder import create_module

    return create_module(
        "json",
        functions={
            'encode': JSONModule.encode,
            'encode_pretty': JSONModule.encode_pretty,
            'decode': JSONModule.decode,
            'is_valid': JSONModule.is_valid,
        }
    )
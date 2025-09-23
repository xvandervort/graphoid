"""
Tests for string-as-graph wrapper functionality.

This module tests the graph operations on strings, including CharNode class,
graph conversion methods, and string operations that work on character nodes.
"""

import pytest
import sys
import os

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from glang.execution.values import StringValue, CharNode
from glang.execution.pipeline import ExecutionPipeline, ExecutionSession
from glang.ast.nodes import SourcePosition


class TestCharNode:
    """Test the CharNode class for individual character representation."""
    
    def test_char_node_creation(self):
        """Test creating CharNode instances."""
        char = CharNode('a')
        assert char.value == 'a'
        assert char.get_type() == 'char'
        assert char.to_display_string() == 'a'
        assert char.to_python() == 'a'
    
    def test_char_node_with_position(self):
        """Test CharNode with source position."""
        position = SourcePosition(1, 5)
        char = CharNode('x', position)
        assert char.value == 'x'
        assert char.position == position
    
    def test_char_node_equality(self):
        """Test CharNode equality comparison."""
        char1 = CharNode('a')
        char2 = CharNode('a')
        char3 = CharNode('b')
        
        assert char1 == char2
        assert char1 != char3
        assert char2 != char3
    
    def test_char_node_invalid_length(self):
        """Test that CharNode only accepts single grapheme clusters."""
        with pytest.raises(ValueError, match="exactly one grapheme cluster"):
            CharNode("")

        with pytest.raises(ValueError, match="exactly one grapheme cluster"):
            CharNode("ab")

        with pytest.raises(ValueError, match="exactly one grapheme cluster"):
            CharNode("hello")


class TestStringGraphConversion:
    """Test string-to-graph conversion methods."""
    
    def test_to_char_nodes_simple(self):
        """Test converting simple string to character nodes."""
        string_val = StringValue("abc")
        char_nodes = string_val.to_char_nodes()
        
        assert len(char_nodes) == 3
        assert all(isinstance(node, CharNode) for node in char_nodes)
        assert [node.value for node in char_nodes] == ['a', 'b', 'c']
    
    def test_to_char_nodes_empty(self):
        """Test converting empty string to character nodes."""
        string_val = StringValue("")
        char_nodes = string_val.to_char_nodes()
        
        assert len(char_nodes) == 0
        assert char_nodes == []
    
    def test_to_char_nodes_caching(self):
        """Test that character nodes are cached."""
        string_val = StringValue("test")
        
        # First call creates the cache
        nodes1 = string_val.to_char_nodes()
        # Second call should return the same objects
        nodes2 = string_val.to_char_nodes()
        
        assert nodes1 is nodes2  # Same object reference
    
    def test_from_char_nodes(self):
        """Test creating StringValue from character nodes."""
        string_val = StringValue("hello")
        char_nodes = string_val.to_char_nodes()
        
        # Create new string from nodes
        new_string = string_val.from_char_nodes(char_nodes)
        
        assert isinstance(new_string, StringValue)
        assert new_string.value == "hello"
        assert new_string is not string_val  # Different object
    
    def test_from_char_nodes_modified(self):
        """Test creating StringValue from modified character nodes."""
        string_val = StringValue("abc")
        char_nodes = string_val.to_char_nodes()
        
        # Reverse the nodes
        reversed_nodes = list(reversed(char_nodes))
        new_string = string_val.from_char_nodes(reversed_nodes)
        
        assert new_string.value == "cba"
    
    def test_clear_char_cache(self):
        """Test clearing the character node cache."""
        string_val = StringValue("test")
        
        # Create cache
        nodes1 = string_val.to_char_nodes()
        
        # Clear cache
        string_val.clear_char_cache()
        
        # Next call should create new nodes
        nodes2 = string_val.to_char_nodes()
        
        assert nodes1 is not nodes2  # Different object references
        assert [n.value for n in nodes1] == [n.value for n in nodes2]  # Same values


class TestStringGraphOperations:
    """Test graph operations on strings."""
    
    def setUp(self):
        """Set up execution pipeline for testing."""
        self.session = ExecutionSession()
    
    def test_string_reverse(self):
        """Test string reverse operation using graph nodes."""
        session = ExecutionSession()
        
        # Test simple reverse
        result = session.execute_statement('string text = "hello"')
        assert result.success
        
        result = session.execute_statement('text.reverse()')
        assert result.success
        assert result.value.value == "olleh"
    
    def test_string_reverse_empty(self):
        """Test reverse on empty string."""
        session = ExecutionSession()
        
        result = session.execute_statement('string empty = ""')
        assert result.success
        
        result = session.execute_statement('empty.reverse()')
        assert result.success
        assert result.value.value == ""
    
    def test_string_reverse_single_char(self):
        """Test reverse on single character."""
        session = ExecutionSession()
        
        result = session.execute_statement('string single = "a"')
        assert result.success
        
        result = session.execute_statement('single.reverse()')
        assert result.success
        assert result.value.value == "a"
    
    def test_string_unique(self):
        """Test string unique operation."""
        session = ExecutionSession()
        
        # Test removing duplicates while preserving order
        result = session.execute_statement('string text = "aabbcc"')
        assert result.success
        
        result = session.execute_statement('text.unique()')
        assert result.success
        assert result.value.value == "abc"
    
    def test_string_unique_complex(self):
        """Test unique with more complex pattern."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "programming"')
        assert result.success
        
        result = session.execute_statement('text.unique()')
        assert result.success
        assert result.value.value == "progamin"  # Order preserved, duplicates removed
    
    def test_string_unique_no_duplicates(self):
        """Test unique on string with no duplicates."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "abc"')
        assert result.success
        
        result = session.execute_statement('text.unique()')
        assert result.success
        assert result.value.value == "abc"  # Unchanged
    
    def test_string_chars(self):
        """Test string chars operation."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "abc"')
        assert result.success
        
        result = session.execute_statement('text.chars()')
        assert result.success
        
        # Should return a list of individual character strings
        assert result.value.get_type() == "list"
        chars = [elem.value for elem in result.value.elements]
        assert chars == ["a", "b", "c"]
    
    def test_string_chars_empty(self):
        """Test chars on empty string."""
        session = ExecutionSession()
        
        result = session.execute_statement('string empty = ""')
        assert result.success
        
        result = session.execute_statement('empty.chars()')
        assert result.success
        
        assert result.value.get_type() == "list"
        assert len(result.value.elements) == 0


class TestStringGraphOperationsEdgeCases:
    """Test edge cases and error conditions for string graph operations."""
    
    def test_reverse_with_arguments_fails(self):
        """Test that reverse with arguments raises error."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "hello"')
        assert result.success
        
        result = session.execute_statement('text.reverse("invalid")')
        assert not result.success
        assert "takes no arguments" in str(result.error).lower()
    
    def test_unique_with_arguments_fails(self):
        """Test that unique with arguments raises error."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "hello"')
        assert result.success
        
        result = session.execute_statement('text.unique(123)')
        assert not result.success
        assert "takes no arguments" in str(result.error).lower()
    
    def test_chars_with_arguments_fails(self):
        """Test that chars with arguments raises error."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "hello"')
        assert result.success
        
        result = session.execute_statement('text.chars("invalid")')
        assert not result.success
        assert "takes no arguments" in str(result.error).lower()
    
    def test_graph_operations_on_unicode(self):
        """Test graph operations work with Unicode characters."""
        session = ExecutionSession()
        
        # Test with emojis and accented characters
        result = session.execute_statement('string text = "café🎉"')
        assert result.success
        
        result = session.execute_statement('text.reverse()')
        assert result.success
        assert result.value.value == "🎉éfac"
        
        # Reset for unique test
        result = session.execute_statement('string text2 = "aaé🎉🎉"')
        assert result.success
        
        result = session.execute_statement('text2.unique()')
        assert result.success
        assert result.value.value == "aé🎉"


class TestStringGraphIntegration:
    """Test integration of graph operations with other string functionality."""
    
    def test_graph_operations_preserve_other_methods(self):
        """Test that graph operations don't interfere with regular string methods."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "Hello World"')
        assert result.success
        
        # Test regular string methods still work
        result = session.execute_statement('text.length()')
        assert result.success
        assert result.value.value == 11
        
        result = session.execute_statement('text.contains("World")')
        assert result.success
        assert result.value.value == True
        
        result = session.execute_statement('text.up()')
        assert result.success
        assert result.value.value == "HELLO WORLD"
        
        # Test graph operations work too
        result = session.execute_statement('text.reverse()')
        assert result.success
        assert result.value.value == "dlroW olleH"
    
    def test_chaining_graph_operations(self):
        """Test that graph operations can be chained conceptually."""
        session = ExecutionSession()
        
        # Create string with duplicates
        result = session.execute_statement('string text = "aabbcc"')
        assert result.success
        
        # First remove duplicates
        result = session.execute_statement('string unique_text = text.unique()')
        assert result.success
        unique_value = session.get_variable_value("unique_text")
        assert unique_value.value == "abc"
        
        # Then reverse the result  
        result = session.execute_statement('unique_text.reverse()')
        assert result.success
        assert result.value.value == "cba"
    
    def test_graph_operations_with_slicing(self):
        """Test graph operations work with slicing."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "programming"')
        assert result.success
        
        # Slice first, then apply graph operation
        result = session.execute_statement('string sliced = text[0:4]')
        assert result.success
        sliced_value = session.get_variable_value("sliced")
        assert sliced_value.value == "prog"
        
        result = session.execute_statement('sliced.reverse()')
        assert result.success
        assert result.value.value == "gorp"


if __name__ == "__main__":
    pytest.main([__file__])
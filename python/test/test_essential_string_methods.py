"""Tests for essential string methods: char_at, index_of, substring, etc."""

import pytest
from glang.execution.pipeline import ExecutionSession


class TestEssentialStringMethods:
    """Test the newly added essential string methods."""

    def setup_method(self):
        """Set up test environment."""
        self.session = ExecutionSession()

    def test_string_indexing_basic(self):
        """Test basic string indexing functionality (using existing indexing syntax)."""
        result = self.session.execute_statement('text = "Hello"')
        assert result.success

        # Test valid indices using existing indexing syntax
        result = self.session.execute_statement('text[0]')
        assert result.success
        assert result.value.value == "H"

        result = self.session.execute_statement('text[4]')
        assert result.success
        assert result.value.value == "o"

        # Test middle character
        result = self.session.execute_statement('text[2]')
        assert result.success
        assert result.value.value == "l"

    def test_string_indexing_negative_indices(self):
        """Test string indexing with negative indices (Python-style)."""
        result = self.session.execute_statement('text = "Hello"')
        assert result.success

        # Test negative indexing
        result = self.session.execute_statement('text[-1]')
        assert result.success
        assert result.value.value == "o"

        result = self.session.execute_statement('text[-5]')
        assert result.success
        assert result.value.value == "H"

    def test_index_of_basic(self):
        """Test basic index_of functionality."""
        result = self.session.execute_statement('text = "Hello World"')
        assert result.success

        # Test finding single characters
        result = self.session.execute_statement('text.index_of("H")')
        assert result.success
        assert result.value.value == 0

        result = self.session.execute_statement('text.index_of("o")')
        assert result.success
        assert result.value.value == 4  # First occurrence

        result = self.session.execute_statement('text.index_of("d")')
        assert result.success
        assert result.value.value == 10

        # Test finding substrings
        result = self.session.execute_statement('text.index_of("World")')
        assert result.success
        assert result.value.value == 6

        result = self.session.execute_statement('text.index_of("llo")')
        assert result.success
        assert result.value.value == 2

    def test_index_of_not_found(self):
        """Test index_of when substring is not found."""
        result = self.session.execute_statement('text = "Hello World"')
        assert result.success

        result = self.session.execute_statement('text.index_of("xyz")')
        assert result.success
        assert result.value.value == -1

        result = self.session.execute_statement('text.index_of("Z")')
        assert result.success
        assert result.value.value == -1

    def test_index_of_with_start_index(self):
        """Test index_of with start index parameter."""
        result = self.session.execute_statement('text = "Hello World"')
        assert result.success

        # Find second occurrence of "o"
        result = self.session.execute_statement('text.index_of("o", 5)')
        assert result.success
        assert result.value.value == 7

        # Start from index where substring doesn't exist
        result = self.session.execute_statement('text.index_of("H", 1)')
        assert result.success
        assert result.value.value == -1

    def test_last_index_of_basic(self):
        """Test basic last_index_of functionality."""
        result = self.session.execute_statement('text = "Hello World"')
        assert result.success

        # Test finding last occurrence
        result = self.session.execute_statement('text.last_index_of("o")')
        assert result.success
        assert result.value.value == 7  # Last occurrence

        result = self.session.execute_statement('text.last_index_of("l")')
        assert result.success
        assert result.value.value == 9  # Last "l" in "World"

    def test_last_index_of_with_end_index(self):
        """Test last_index_of with end index parameter."""
        result = self.session.execute_statement('text = "Hello World"')
        assert result.success

        # Find last "o" before index 6
        result = self.session.execute_statement('text.last_index_of("o", 6)')
        assert result.success
        assert result.value.value == 4  # First "o" in "Hello"

    def test_substring_basic(self):
        """Test basic substring functionality."""
        result = self.session.execute_statement('text = "Hello World"')
        assert result.success

        # Test substring with start only
        result = self.session.execute_statement('text.substring(6)')
        assert result.success
        assert result.value.value == "World"

        result = self.session.execute_statement('text.substring(0)')
        assert result.success
        assert result.value.value == "Hello World"

        # Test substring with start and end
        result = self.session.execute_statement('text.substring(0, 5)')
        assert result.success
        assert result.value.value == "Hello"

        result = self.session.execute_statement('text.substring(6, 11)')
        assert result.success
        assert result.value.value == "World"

        result = self.session.execute_statement('text.substring(2, 7)')
        assert result.success
        assert result.value.value == "llo W"

    def test_substring_negative_indices(self):
        """Test substring with negative indices."""
        result = self.session.execute_statement('text = "Hello World"')
        assert result.success

        # Test negative start
        result = self.session.execute_statement('text.substring(-5)')
        assert result.success
        assert result.value.value == "World"

        # Test negative end
        result = self.session.execute_statement('text.substring(0, -6)')
        assert result.success
        assert result.value.value == "Hello"

        # Test both negative
        result = self.session.execute_statement('text.substring(-5, -1)')
        assert result.success
        assert result.value.value == "Worl"

    def test_substring_edge_cases(self):
        """Test substring edge cases."""
        result = self.session.execute_statement('text = "Hello"')
        assert result.success

        # Test start > end (should swap)
        result = self.session.execute_statement('text.substring(3, 1)')
        assert result.success
        assert result.value.value == "el"

        # Test out of bounds indices (should clamp)
        result = self.session.execute_statement('text.substring(-10, 10)')
        assert result.success
        assert result.value.value == "Hello"

        # Test empty result
        result = self.session.execute_statement('text.substring(2, 2)')
        assert result.success
        assert result.value.value == ""

    def test_repeat_basic(self):
        """Test string repeat functionality."""
        result = self.session.execute_statement('text = "Hi"')
        assert result.success

        # Test basic repetition
        result = self.session.execute_statement('text.repeat(3)')
        assert result.success
        assert result.value.value == "HiHiHi"

        result = self.session.execute_statement('text.repeat(1)')
        assert result.success
        assert result.value.value == "Hi"

        result = self.session.execute_statement('text.repeat(0)')
        assert result.success
        assert result.value.value == ""

    def test_repeat_edge_cases(self):
        """Test repeat edge cases."""
        result = self.session.execute_statement('empty = ""')
        assert result.success

        # Test repeating empty string
        result = self.session.execute_statement('empty.repeat(5)')
        assert result.success
        assert result.value.value == ""

    def test_pad_left_basic(self):
        """Test basic pad_left functionality."""
        result = self.session.execute_statement('text = "Hi"')
        assert result.success

        # Test padding with default space
        result = self.session.execute_statement('text.pad_left(5)')
        assert result.success
        assert result.value.value == "   Hi"

        # Test padding with custom character
        result = self.session.execute_statement('text.pad_left(5, "0")')
        assert result.success
        assert result.value.value == "000Hi"

    def test_pad_left_edge_cases(self):
        """Test pad_left edge cases."""
        result = self.session.execute_statement('text = "Hello"')
        assert result.success

        # Test when text is already long enough
        result = self.session.execute_statement('text.pad_left(3)')
        assert result.success
        assert result.value.value == "Hello"

        # Test exact length
        result = self.session.execute_statement('text.pad_left(5)')
        assert result.success
        assert result.value.value == "Hello"

    def test_pad_right_basic(self):
        """Test basic pad_right functionality."""
        result = self.session.execute_statement('text = "Hi"')
        assert result.success

        # Test padding with default space
        result = self.session.execute_statement('text.pad_right(5)')
        assert result.success
        assert result.value.value == "Hi   "

        # Test padding with custom character
        result = self.session.execute_statement('text.pad_right(5, "*")')
        assert result.success
        assert result.value.value == "Hi***"

    def test_pad_right_edge_cases(self):
        """Test pad_right edge cases."""
        result = self.session.execute_statement('text = "Hello"')
        assert result.success

        # Test when text is already long enough
        result = self.session.execute_statement('text.pad_right(3)')
        assert result.success
        assert result.value.value == "Hello"

    def test_combined_methods(self):
        """Test combining multiple string methods."""
        result = self.session.execute_statement('text = "Hello World"')
        assert result.success

        # Get character and check it using indexing syntax
        result = self.session.execute_statement('char = text[6]')
        assert result.success
        result = self.session.execute_statement('char == "W"')
        assert result.success
        assert result.value.value == True

        # Find and extract
        result = self.session.execute_statement('start = text.index_of("World")')
        assert result.success
        result = self.session.execute_statement('word = text.substring(start, start + 5)')
        assert result.success
        result = self.session.execute_statement('word')
        assert result.success
        assert result.value.value == "World"

        # Pad and repeat
        result = self.session.execute_statement('short = "A"')
        assert result.success
        result = self.session.execute_statement('repeated = short.repeat(3)')
        assert result.success
        result = self.session.execute_statement('padded = repeated.pad_left(10, "-")')
        assert result.success
        result = self.session.execute_statement('padded')
        assert result.success
        assert result.value.value == "-------AAA"

    def test_empty_string_methods(self):
        """Test methods with empty strings."""
        result = self.session.execute_statement('empty = ""')
        assert result.success

        # Note: empty string indexing would error, so we test other methods

        # index_of on empty string
        result = self.session.execute_statement('empty.index_of("a")')
        assert result.success
        assert result.value.value == -1

        # substring on empty string
        result = self.session.execute_statement('empty.substring(0)')
        assert result.success
        assert result.value.value == ""

        # repeat empty string
        result = self.session.execute_statement('empty.repeat(5)')
        assert result.success
        assert result.value.value == ""

        # pad empty string
        result = self.session.execute_statement('empty.pad_left(3)')
        assert result.success
        assert result.value.value == "   "
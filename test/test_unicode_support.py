"""Tests for Unicode support in Glang string operations."""

import pytest
from glang.execution.pipeline import ExecutionSession


class TestUnicodeSupport:
    """Test Unicode handling in all string operations."""

    def setup_method(self):
        """Set up test environment."""
        self.session = ExecutionSession()

    def test_basic_unicode_strings(self):
        """Test basic Unicode string handling."""
        # Test various Unicode ranges
        result = self.session.execute_statement('ascii = "Hello"')
        assert result.success

        result = self.session.execute_statement('chinese = "你好世界"')
        assert result.success

        result = self.session.execute_statement('emoji = "👋🌍🚀"')
        assert result.success

        result = self.session.execute_statement('mixed = "Hello 世界 🌍"')
        assert result.success

    def test_unicode_length_calculation(self):
        """Test length calculation with Unicode characters."""
        # ASCII should be straightforward
        result = self.session.execute_statement('ascii = "Hello"')
        assert result.success
        result = self.session.execute_statement('ascii.length()')
        assert result.success
        assert result.value.value == 5

        # Chinese characters (each should count as 1)
        result = self.session.execute_statement('chinese = "你好"')
        assert result.success
        result = self.session.execute_statement('chinese.length()')
        assert result.success
        assert result.value.value == 2

        # Emoji (should count as 1 each, not code units)
        result = self.session.execute_statement('emoji = "🌍"')
        assert result.success
        result = self.session.execute_statement('emoji.length()')
        assert result.success
        # Note: This might fail if we're counting code units instead of graphemes

    def test_unicode_indexing(self):
        """Test indexing with Unicode characters."""
        result = self.session.execute_statement('text = "H世🌍"')
        assert result.success

        # Test accessing each character
        result = self.session.execute_statement('text[0]')
        assert result.success
        assert result.value.value == "H"

        result = self.session.execute_statement('text[1]')
        assert result.success
        assert result.value.value == "世"

        result = self.session.execute_statement('text[2]')
        assert result.success
        assert result.value.value == "🌍"

    def test_unicode_substring_operations(self):
        """Test substring operations with Unicode."""
        result = self.session.execute_statement('text = "Hello世界🌍"')
        assert result.success

        # Extract parts containing Unicode
        result = self.session.execute_statement('text.substring(5, 7)')
        assert result.success
        assert result.value.value == "世界"

        result = self.session.execute_statement('text.substring(7)')
        assert result.success
        assert result.value.value == "🌍"

    def test_unicode_index_of_operations(self):
        """Test index_of with Unicode characters."""
        result = self.session.execute_statement('text = "Hello世界🌍World"')
        assert result.success

        # Find Unicode characters
        result = self.session.execute_statement('text.index_of("世界")')
        assert result.success
        assert result.value.value == 5

        result = self.session.execute_statement('text.index_of("🌍")')
        assert result.success
        assert result.value.value == 7

    def test_unicode_case_conversion(self):
        """Test case conversion with Unicode."""
        # Test with accented characters
        result = self.session.execute_statement('accented = "José Müller"')
        assert result.success

        result = self.session.execute_statement('accented.up()')
        assert result.success
        assert result.value.value == "JOSÉ MÜLLER"

        result = self.session.execute_statement('accented.down()')
        assert result.success
        assert result.value.value == "josé müller"

    def test_unicode_contains_operations(self):
        """Test contains operations with Unicode."""
        result = self.session.execute_statement('text = "Programming in 中文 is fun! 🎉"')
        assert result.success

        result = self.session.execute_statement('text.contains("中文")')
        assert result.success
        assert result.value.value == True

        result = self.session.execute_statement('text.contains("🎉")')
        assert result.success
        assert result.value.value == True

    def test_unicode_starts_ends_with(self):
        """Test starts_with and ends_with with Unicode."""
        result = self.session.execute_statement('text = "🚀Hello世界🌍"')
        assert result.success

        result = self.session.execute_statement('text.starts_with("🚀")')
        assert result.success
        assert result.value.value == True

        result = self.session.execute_statement('text.ends_with("🌍")')
        assert result.success
        assert result.value.value == True

    def test_unicode_padding_operations(self):
        """Test padding operations with Unicode."""
        result = self.session.execute_statement('text = "世界"')
        assert result.success

        result = self.session.execute_statement('text.pad_left(5, "🌟")')
        assert result.success
        assert result.value.value == "🌟🌟🌟世界"

        result = self.session.execute_statement('text.pad_right(5, "⭐")')
        assert result.success
        assert result.value.value == "世界⭐⭐⭐"

    def test_unicode_repeat_operations(self):
        """Test repeat operations with Unicode."""
        result = self.session.execute_statement('emoji = "🎉"')
        assert result.success

        result = self.session.execute_statement('emoji.repeat(3)')
        assert result.success
        assert result.value.value == "🎉🎉🎉"

    def test_complex_emoji_sequences(self):
        """Test handling of complex emoji sequences."""
        # Complex emoji with ZWJ (Zero Width Joiner) sequences
        result = self.session.execute_statement('family = "👨‍👩‍👧‍👦"')
        assert result.success

        # This is tricky - should this count as 1 grapheme or multiple?
        result = self.session.execute_statement('family.length()')
        assert result.success
        # The length might be unexpected due to ZWJ sequences

        # Test if we can work with it as a unit
        result = self.session.execute_statement('family.repeat(2)')
        assert result.success

    def test_unicode_splitting(self):
        """Test string splitting with Unicode."""
        result = self.session.execute_statement('text = "Hello,世界,🌍"')
        assert result.success

        result = self.session.execute_statement('parts = text.split(",")')
        assert result.success

        # Check the parts
        result = self.session.execute_statement('parts[0]')
        assert result.success
        assert result.value.value == "Hello"

        result = self.session.execute_statement('parts[1]')
        assert result.success
        assert result.value.value == "世界"

        result = self.session.execute_statement('parts[2]')
        assert result.success
        assert result.value.value == "🌍"

    def test_unicode_normalization_issues(self):
        """Test potential Unicode normalization issues."""
        # Test with characters that can be represented in multiple ways
        # For example: é can be é (single char) or e + ́ (combining)

        # This might reveal normalization issues
        result = self.session.execute_statement('single = "café"')  # é as single character
        assert result.success

        result = self.session.execute_statement('composed = "cafe\u0301"')  # e + combining acute
        assert result.success

        # These should be treated as equal ideally
        result = self.session.execute_statement('single.length()')
        assert result.success
        single_length = result.value.value

        result = self.session.execute_statement('composed.length()')
        assert result.success
        composed_length = result.value.value

        # Record the behavior for analysis
        print(f"Single char é length: {single_length}, Composed e+́ length: {composed_length}")

    def test_unicode_edge_cases(self):
        """Test Unicode edge cases."""
        # Empty string
        result = self.session.execute_statement('empty = ""')
        assert result.success
        result = self.session.execute_statement('empty.length()')
        assert result.success
        assert result.value.value == 0

        # String with only whitespace including Unicode spaces
        result = self.session.execute_statement('spaces = " \u00a0\u2000"')  # space, NBSP, en quad
        assert result.success
        result = self.session.execute_statement('spaces.trim()')
        assert result.success
        # Should handle Unicode whitespace in trim
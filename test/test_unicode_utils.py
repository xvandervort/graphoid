"""Tests for Unicode utility functions."""

import pytest
from glang.execution.unicode_utils import UnicodeUtils


class TestUnicodeUtils:
    """Test Unicode utility functions."""

    def test_grapheme_length_simple(self):
        """Test grapheme length with simple ASCII."""
        assert UnicodeUtils.grapheme_length("hello") == 5
        assert UnicodeUtils.grapheme_length("") == 0

    def test_grapheme_length_emoji(self):
        """Test grapheme length with emoji."""
        # Simple emoji
        assert UnicodeUtils.grapheme_length("🌍") == 1
        assert UnicodeUtils.grapheme_length("🚀") == 1

        # Multiple emoji
        assert UnicodeUtils.grapheme_length("🌍🚀🎉") == 3

    def test_grapheme_length_complex_emoji(self):
        """Test grapheme length with complex emoji sequences."""
        # Family emoji with ZWJ sequences
        family = "👨‍👩‍👧‍👦"
        assert UnicodeUtils.grapheme_length(family) == 1

        # Profession emoji with ZWJ
        doctor = "👨‍⚕️"
        assert UnicodeUtils.grapheme_length(doctor) == 1

    def test_grapheme_length_combining_characters(self):
        """Test grapheme length with combining characters."""
        # é as single character vs e + combining acute
        single_e = "café"  # é as single char
        combined_e = "cafe\u0301"  # e + combining acute

        assert UnicodeUtils.grapheme_length(single_e) == 4
        assert UnicodeUtils.grapheme_length(combined_e) == 4

    def test_grapheme_clusters_simple(self):
        """Test grapheme cluster splitting with simple text."""
        clusters = UnicodeUtils.grapheme_clusters("hello")
        assert clusters == ["h", "e", "l", "l", "o"]

    def test_grapheme_clusters_emoji(self):
        """Test grapheme cluster splitting with emoji."""
        clusters = UnicodeUtils.grapheme_clusters("H🌍W")
        assert clusters == ["H", "🌍", "W"]

    def test_grapheme_clusters_complex_emoji(self):
        """Test grapheme cluster splitting with complex emoji."""
        family = "👨‍👩‍👧‍👦"
        clusters = UnicodeUtils.grapheme_clusters(family)
        assert len(clusters) == 1
        assert clusters[0] == family

    def test_grapheme_clusters_combining(self):
        """Test grapheme cluster splitting with combining characters."""
        text = "cafe\u0301"  # cafe with combining acute on e
        clusters = UnicodeUtils.grapheme_clusters(text)
        assert len(clusters) == 4
        assert clusters == ["c", "a", "f", "e\u0301"]

    def test_grapheme_at(self):
        """Test getting grapheme cluster at index."""
        text = "H🌍cafe\u0301"

        assert UnicodeUtils.grapheme_at(text, 0) == "H"
        assert UnicodeUtils.grapheme_at(text, 1) == "🌍"
        assert UnicodeUtils.grapheme_at(text, 2) == "c"
        assert UnicodeUtils.grapheme_at(text, -1) == "e\u0301"  # last cluster

        # Out of bounds
        assert UnicodeUtils.grapheme_at(text, 100) == ""
        assert UnicodeUtils.grapheme_at(text, -100) == ""

    def test_grapheme_substring(self):
        """Test grapheme-aware substring extraction."""
        text = "H🌍cafe\u0301"

        # Simple range
        assert UnicodeUtils.grapheme_substring(text, 1, 3) == "🌍c"

        # From start
        assert UnicodeUtils.grapheme_substring(text, 0, 2) == "H🌍"

        # To end
        assert UnicodeUtils.grapheme_substring(text, 2) == "cafe\u0301"

        # Negative indices
        assert UnicodeUtils.grapheme_substring(text, -2) == "fe\u0301"

    def test_grapheme_index_of(self):
        """Test grapheme-aware substring finding."""
        text = "Hello🌍World"

        assert UnicodeUtils.grapheme_index_of(text, "🌍") == 5
        assert UnicodeUtils.grapheme_index_of(text, "World") == 6
        assert UnicodeUtils.grapheme_index_of(text, "xyz") == -1

    def test_normalize_text(self):
        """Test Unicode normalization."""
        # Test NFC normalization (composed)
        text = "cafe\u0301"  # e + combining acute
        normalized = UnicodeUtils.normalize_text(text, 'NFC')
        assert "é" in normalized  # Should be composed

        # Test NFD normalization (decomposed)
        text = "café"  # é as single char
        normalized = UnicodeUtils.normalize_text(text, 'NFD')
        assert "\u0301" in normalized  # Should contain combining acute

    def test_unicode_category(self):
        """Test Unicode category detection."""
        assert UnicodeUtils.get_unicode_category('A') == 'Lu'  # Uppercase letter
        assert UnicodeUtils.get_unicode_category('a') == 'Ll'  # Lowercase letter
        assert UnicodeUtils.get_unicode_category('1') == 'Nd'  # Decimal digit
        assert UnicodeUtils.get_unicode_category('\u0301') == 'Mn'  # Nonspacing mark

    def test_is_emoji(self):
        """Test emoji detection."""
        assert UnicodeUtils.is_emoji('🌍') == True
        assert UnicodeUtils.is_emoji('🚀') == True
        assert UnicodeUtils.is_emoji('A') == False
        assert UnicodeUtils.is_emoji('1') == False

        # Complex emoji (simplified check)
        family = "👨‍👩‍👧‍👦"
        assert UnicodeUtils.is_emoji(family) == True

    def test_validate_utf8(self):
        """Test UTF-8 validation."""
        assert UnicodeUtils.validate_utf8(b'hello') == True
        assert UnicodeUtils.validate_utf8('hello'.encode('utf-8')) == True
        assert UnicodeUtils.validate_utf8('🌍'.encode('utf-8')) == True

        # Invalid UTF-8
        assert UnicodeUtils.validate_utf8(b'\xff\xfe') == False

    def test_grapheme_boundary(self):
        """Test grapheme boundary detection."""
        text = "cafe\u0301"  # cafe with combining acute

        assert UnicodeUtils.is_grapheme_boundary(text, 0) == True
        assert UnicodeUtils.is_grapheme_boundary(text, 1) == True
        assert UnicodeUtils.is_grapheme_boundary(text, 3) == True  # Before 'e'
        assert UnicodeUtils.is_grapheme_boundary(text, 4) == False  # Between 'e' and combining mark
        assert UnicodeUtils.is_grapheme_boundary(text, len(text)) == True
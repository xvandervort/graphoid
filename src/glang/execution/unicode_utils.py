"""Unicode utility functions for proper Unicode handling in Glang."""

import unicodedata
import re
from typing import List


class UnicodeUtils:
    """Utilities for handling Unicode text properly."""

    @staticmethod
    def grapheme_length(text: str) -> int:
        """
        Get the number of grapheme clusters in a string.

        This counts what users perceive as single characters, not code points.
        For example: "👨‍👩‍👧‍👦" is 1 grapheme cluster, not 7 code points.
        """
        return len(UnicodeUtils.grapheme_clusters(text))

    @staticmethod
    def grapheme_clusters(text: str) -> List[str]:
        """
        Split text into grapheme clusters.

        Returns a list where each element is a grapheme cluster (what users
        perceive as a single character).
        """
        # This is a simplified approach that handles most common cases
        # For production, we'd want to use a proper grapheme library like 'grapheme'

        # Handle Zero Width Joiner (ZWJ) sequences in emoji
        # ZWJ is U+200D, used to combine emoji into families, professions, etc.
        if '\u200d' in text:
            # Split on ZWJ boundaries but keep ZWJ sequences together
            clusters = []
            current_cluster = ""

            i = 0
            while i < len(text):
                char = text[i]
                current_cluster += char

                # Look ahead to see if this is part of a ZWJ sequence
                if i + 1 < len(text):
                    next_char = text[i + 1]
                    # If next char is ZWJ or we're in a ZWJ sequence, continue
                    if (next_char == '\u200d' or char == '\u200d' or
                        (current_cluster and '\u200d' in current_cluster)):
                        i += 1
                        continue

                # Check for combining characters
                if i + 1 < len(text):
                    next_char = text[i + 1]
                    if unicodedata.category(next_char).startswith('M'):  # Mark category
                        i += 1
                        current_cluster += next_char

                clusters.append(current_cluster)
                current_cluster = ""
                i += 1

            if current_cluster:
                clusters.append(current_cluster)

            return clusters

        # Handle combining characters (like é = e + ́)
        clusters = []
        current_cluster = ""

        for char in text:
            if unicodedata.category(char).startswith('M'):  # Combining mark
                if current_cluster:
                    current_cluster += char
                else:
                    # Combining mark without base character
                    clusters.append(char)
            else:
                if current_cluster:
                    clusters.append(current_cluster)
                current_cluster = char

        if current_cluster:
            clusters.append(current_cluster)

        return clusters

    @staticmethod
    def grapheme_at(text: str, index: int) -> str:
        """
        Get the grapheme cluster at the specified index.

        Returns empty string if index is out of bounds.
        Supports negative indexing.
        """
        clusters = UnicodeUtils.grapheme_clusters(text)

        if index < 0:
            index = len(clusters) + index

        if 0 <= index < len(clusters):
            return clusters[index]

        return ""

    @staticmethod
    def grapheme_substring(text: str, start: int, end: int = None) -> str:
        """
        Extract a substring based on grapheme cluster boundaries.

        Args:
            text: The input string
            start: Starting grapheme index
            end: Ending grapheme index (exclusive). If None, goes to end.

        Returns:
            Substring containing the specified grapheme clusters.
        """
        clusters = UnicodeUtils.grapheme_clusters(text)

        # Handle negative indices
        if start < 0:
            start = len(clusters) + start
        if end is not None and end < 0:
            end = len(clusters) + end

        # Clamp to valid range
        start = max(0, min(start, len(clusters)))
        if end is None:
            end = len(clusters)
        else:
            end = max(0, min(end, len(clusters)))

        # Ensure start <= end
        if start > end:
            start, end = end, start

        return ''.join(clusters[start:end])

    @staticmethod
    def grapheme_index_of(text: str, substring: str, start: int = 0) -> int:
        """
        Find the grapheme cluster index of a substring.

        Returns -1 if not found.
        """
        clusters = UnicodeUtils.grapheme_clusters(text)
        search_clusters = UnicodeUtils.grapheme_clusters(substring)

        if not search_clusters:
            return -1

        # Handle negative start
        if start < 0:
            start = len(clusters) + start
        start = max(0, start)

        # Search for the pattern
        for i in range(start, len(clusters) - len(search_clusters) + 1):
            if clusters[i:i + len(search_clusters)] == search_clusters:
                return i

        return -1

    @staticmethod
    def grapheme_last_index_of(text: str, substring: str, end: int = None) -> int:
        """
        Find the last grapheme cluster index of a substring.

        Returns -1 if not found.
        """
        clusters = UnicodeUtils.grapheme_clusters(text)
        search_clusters = UnicodeUtils.grapheme_clusters(substring)

        if not search_clusters:
            return -1

        # Handle end parameter
        if end is None:
            end = len(clusters)
        elif end < 0:
            end = len(clusters) + end
        end = max(0, min(end, len(clusters)))

        # Search backwards for the pattern
        for i in range(end - len(search_clusters), -1, -1):
            if i < 0:
                break
            if clusters[i:i + len(search_clusters)] == search_clusters:
                return i

        return -1

    @staticmethod
    def normalize_text(text: str, form: str = 'NFC') -> str:
        """
        Normalize Unicode text.

        Args:
            text: Input text
            form: Normalization form ('NFC', 'NFD', 'NFKC', 'NFKD')

        Returns:
            Normalized text
        """
        return unicodedata.normalize(form, text)

    @staticmethod
    def is_grapheme_boundary(text: str, index: int) -> bool:
        """
        Check if the position is at a grapheme cluster boundary.

        This is useful for cursor positioning and text editing.
        """
        if index <= 0 or index >= len(text):
            return True

        # Simple heuristic: check if previous character is a combining mark
        prev_char = text[index - 1]
        curr_char = text[index]

        # If current character is a combining mark, not a boundary
        if unicodedata.category(curr_char).startswith('M'):
            return False

        # If previous character is ZWJ, not a boundary
        if prev_char == '\u200d':
            return False

        # If current character is ZWJ, not a boundary
        if curr_char == '\u200d':
            return False

        return True

    @staticmethod
    def validate_utf8(data: bytes) -> bool:
        """
        Validate that bytes represent valid UTF-8.
        """
        try:
            data.decode('utf-8')
            return True
        except UnicodeDecodeError:
            return False

    @staticmethod
    def get_unicode_category(char: str) -> str:
        """
        Get the Unicode category of a character.

        Returns categories like 'Lu' (uppercase letter), 'Mn' (nonspacing mark), etc.
        """
        if len(char) != 1:
            return 'Unknown'
        return unicodedata.category(char)

    @staticmethod
    def is_emoji(char: str) -> bool:
        """
        Check if a character is an emoji.

        This is a simplified check - full emoji detection is quite complex.
        """
        if len(char) == 1:
            code_point = ord(char)
            # Common emoji ranges (simplified)
            return (
                0x1F600 <= code_point <= 0x1F64F or  # Emoticons
                0x1F300 <= code_point <= 0x1F5FF or  # Misc Symbols and Pictographs
                0x1F680 <= code_point <= 0x1F6FF or  # Transport and Map
                0x1F1E6 <= code_point <= 0x1F1FF or  # Regional Indicator Symbols
                0x2600 <= code_point <= 0x26FF or    # Misc symbols
                0x2700 <= code_point <= 0x27BF       # Dingbats
            )

        # For multi-character emoji, check if it contains emoji characters
        return any(UnicodeUtils.is_emoji(c) for c in char)
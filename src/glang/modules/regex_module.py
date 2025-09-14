"""
Glang Regular Expression Module

Provides pattern matching and text processing capabilities that complement 
Glang's existing unified string interface. While Glang's built-in string
methods handle 90% of text processing needs, regex is essential for:
- Complex pattern matching and validation
- Advanced text replacement with capture groups  
- Parsing structured text formats
- Custom tokenization and text analysis

This module integrates cleanly with Glang's type system and error handling.
"""

import re
from typing import Optional, List, Dict, Any
import sys
import os

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../../..'))

from glang.execution.values import (
    GlangValue, StringValue, BooleanValue, ListValue, 
    HashValue, DataValue, NumberValue, NoneValue
)
from glang.ast.nodes import SourcePosition


class RegexModule:
    """
    Glang Regular Expression Module
    
    Provides pattern matching and replacement capabilities using Python's re module
    as the underlying engine, but with Glang-native interfaces and error handling.
    """
    
    def __init__(self):
        # Cache compiled patterns to improve performance
        self._pattern_cache: Dict[str, re.Pattern] = {}
        self._cache_limit = 100  # Prevent memory bloat
    
    def _compile_pattern(self, pattern: str, flags: int = 0) -> re.Pattern:
        """Compile regex pattern with caching."""
        cache_key = f"{pattern}:{flags}"
        
        if cache_key in self._pattern_cache:
            return self._pattern_cache[cache_key]
        
        # Clear cache if it gets too large
        if len(self._pattern_cache) >= self._cache_limit:
            self._pattern_cache.clear()
        
        try:
            compiled = re.compile(pattern, flags)
            self._pattern_cache[cache_key] = compiled
            return compiled
        except re.error as e:
            raise ValueError(f"Invalid regex pattern '{pattern}': {str(e)}")
    
    def match(self, pattern_value: StringValue, text_value: StringValue, 
              flags_value: Optional[StringValue] = None) -> BooleanValue:
        """
        Test if pattern matches at the beginning of text.
        
        Args:
            pattern: Regular expression pattern
            text: Text to search in
            flags: Optional regex flags (i, m, s, x, etc.)
        
        Returns:
            BooleanValue: True if pattern matches at start, False otherwise
        """
        if not isinstance(pattern_value, StringValue):
            raise TypeError(f"Pattern must be string, got {pattern_value.get_type()}")
        if not isinstance(text_value, StringValue):
            raise TypeError(f"Text must be string, got {text_value.get_type()}")
        
        pattern = pattern_value.value
        text = text_value.value
        flags = self._parse_flags(flags_value) if flags_value else 0
        
        compiled_pattern = self._compile_pattern(pattern, flags)
        match = compiled_pattern.match(text)
        
        return BooleanValue(match is not None, pattern_value.position)
    
    def search(self, pattern_value: StringValue, text_value: StringValue,
               flags_value: Optional[StringValue] = None) -> BooleanValue:
        """
        Test if pattern is found anywhere in text.
        
        Args:
            pattern: Regular expression pattern
            text: Text to search in
            flags: Optional regex flags
        
        Returns:
            BooleanValue: True if pattern found, False otherwise
        """
        if not isinstance(pattern_value, StringValue):
            raise TypeError(f"Pattern must be string, got {pattern_value.get_type()}")
        if not isinstance(text_value, StringValue):
            raise TypeError(f"Text must be string, got {text_value.get_type()}")
        
        pattern = pattern_value.value
        text = text_value.value
        flags = self._parse_flags(flags_value) if flags_value else 0
        
        compiled_pattern = self._compile_pattern(pattern, flags)
        match = compiled_pattern.search(text)
        
        return BooleanValue(match is not None, pattern_value.position)
    
    def find_all(self, pattern_value: StringValue, text_value: StringValue,
                 flags_value: Optional[StringValue] = None) -> ListValue:
        """
        Find all non-overlapping matches of pattern in text.
        
        Args:
            pattern: Regular expression pattern
            text: Text to search in  
            flags: Optional regex flags
        
        Returns:
            ListValue: List of matched strings
        """
        if not isinstance(pattern_value, StringValue):
            raise TypeError(f"Pattern must be string, got {pattern_value.get_type()}")
        if not isinstance(text_value, StringValue):
            raise TypeError(f"Text must be string, got {text_value.get_type()}")
        
        pattern = pattern_value.value
        text = text_value.value
        flags = self._parse_flags(flags_value) if flags_value else 0
        
        compiled_pattern = self._compile_pattern(pattern, flags)
        matches = compiled_pattern.findall(text)
        
        # Convert to Glang string values
        result_elements = [StringValue(match, pattern_value.position) for match in matches]
        return ListValue(result_elements, position=pattern_value.position, constraint="string")
    
    def find_groups(self, pattern_value: StringValue, text_value: StringValue,
                    flags_value: Optional[StringValue] = None) -> ListValue:
        """
        Find all matches with capture groups.
        
        Args:
            pattern: Regular expression pattern with capture groups
            text: Text to search in
            flags: Optional regex flags
        
        Returns:
            ListValue: List of matches, each match is a list of captured groups
        """
        if not isinstance(pattern_value, StringValue):
            raise TypeError(f"Pattern must be string, got {pattern_value.get_type()}")
        if not isinstance(text_value, StringValue):
            raise TypeError(f"Text must be string, got {text_value.get_type()}")
        
        pattern = pattern_value.value
        text = text_value.value  
        flags = self._parse_flags(flags_value) if flags_value else 0
        
        compiled_pattern = self._compile_pattern(pattern, flags)
        matches = compiled_pattern.finditer(text)
        
        result_elements = []
        for match in matches:
            # Create list of captured groups for this match
            groups = [StringValue(group or "", pattern_value.position) 
                     for group in match.groups()]
            if not groups:  # If no groups, include the whole match
                groups = [StringValue(match.group(0), pattern_value.position)]
            
            match_list = ListValue(groups, position=pattern_value.position, constraint="string")
            result_elements.append(match_list)
        
        return ListValue(result_elements, position=pattern_value.position, constraint="list")
    
    def replace(self, pattern_value: StringValue, replacement_value: StringValue,
                text_value: StringValue, flags_value: Optional[StringValue] = None) -> StringValue:
        """
        Replace all occurrences of pattern in text.
        
        Args:
            pattern: Regular expression pattern
            replacement: Replacement string (supports \\1, \\2 for capture groups)
            text: Text to process
            flags: Optional regex flags
        
        Returns:
            StringValue: Text with replacements made
        """
        if not isinstance(pattern_value, StringValue):
            raise TypeError(f"Pattern must be string, got {pattern_value.get_type()}")
        if not isinstance(replacement_value, StringValue):
            raise TypeError(f"Replacement must be string, got {replacement_value.get_type()}")
        if not isinstance(text_value, StringValue):
            raise TypeError(f"Text must be string, got {text_value.get_type()}")
        
        pattern = pattern_value.value
        replacement = replacement_value.value
        text = text_value.value
        flags = self._parse_flags(flags_value) if flags_value else 0
        
        compiled_pattern = self._compile_pattern(pattern, flags)
        result = compiled_pattern.sub(replacement, text)
        
        return StringValue(result, pattern_value.position)
    
    def split(self, pattern_value: StringValue, text_value: StringValue,
              flags_value: Optional[StringValue] = None) -> ListValue:
        """
        Split text by regex pattern.
        
        Args:
            pattern: Regular expression pattern to split on
            text: Text to split
            flags: Optional regex flags
        
        Returns:
            ListValue: List of text segments
        """
        if not isinstance(pattern_value, StringValue):
            raise TypeError(f"Pattern must be string, got {pattern_value.get_type()}")
        if not isinstance(text_value, StringValue):
            raise TypeError(f"Text must be string, got {text_value.get_type()}")
        
        pattern = pattern_value.value
        text = text_value.value
        flags = self._parse_flags(flags_value) if flags_value else 0
        
        compiled_pattern = self._compile_pattern(pattern, flags)
        segments = compiled_pattern.split(text)
        
        # Convert to Glang string values
        result_elements = [StringValue(segment, pattern_value.position) for segment in segments]
        return ListValue(result_elements, position=pattern_value.position, constraint="string")
    
    def validate(self, pattern_value: StringValue, text_value: StringValue,
                 flags_value: Optional[StringValue] = None) -> BooleanValue:
        """
        Validate that entire text matches pattern (full match).
        
        Args:
            pattern: Regular expression pattern
            text: Text to validate
            flags: Optional regex flags
        
        Returns:
            BooleanValue: True if entire text matches, False otherwise
        """
        if not isinstance(pattern_value, StringValue):
            raise TypeError(f"Pattern must be string, got {pattern_value.get_type()}")
        if not isinstance(text_value, StringValue):
            raise TypeError(f"Text must be string, got {text_value.get_type()}")
        
        pattern = pattern_value.value
        text = text_value.value
        flags = self._parse_flags(flags_value) if flags_value else 0
        
        compiled_pattern = self._compile_pattern(pattern, flags)
        match = compiled_pattern.fullmatch(text)
        
        return BooleanValue(match is not None, pattern_value.position)
    
    def escape(self, text_value: StringValue) -> StringValue:
        """
        Escape special regex characters in text.
        
        Args:
            text: Text to escape
        
        Returns:
            StringValue: Text with regex special characters escaped
        """
        if not isinstance(text_value, StringValue):
            raise TypeError(f"Text must be string, got {text_value.get_type()}")
        
        escaped = re.escape(text_value.value)
        return StringValue(escaped, text_value.position)
    
    def _parse_flags(self, flags_value: StringValue) -> int:
        """Parse regex flags from string format."""
        if not isinstance(flags_value, StringValue):
            raise TypeError(f"Flags must be string, got {flags_value.get_type()}")
        
        flags = 0
        flag_str = flags_value.value.lower()
        
        for char in flag_str:
            if char == 'i':
                flags |= re.IGNORECASE
            elif char == 'm':
                flags |= re.MULTILINE
            elif char == 's':
                flags |= re.DOTALL
            elif char == 'x':
                flags |= re.VERBOSE
            elif char == 'a':
                flags |= re.ASCII
            elif char == 'l':
                flags |= re.LOCALE
            else:
                raise ValueError(f"Unknown regex flag: '{char}'")
        
        return flags


def create_regex_module_namespace():
    """Create the namespace for the built-in Regex module."""
    from .module_manager import ModuleNamespace
    
    namespace = ModuleNamespace("regex")
    module = RegexModule()
    
    # Register regex functions
    regex_functions = {
        'match': module.match,
        'search': module.search,
        'find_all': module.find_all,
        'find_groups': module.find_groups,
        'replace': module.replace,
        'split': module.split,
        'validate': module.validate,
        'escape': module.escape,
    }
    
    # Wrap functions as callable values
    from ..execution.function_value import BuiltinFunctionValue
    
    for name, func in regex_functions.items():
        namespace.set_symbol(name, BuiltinFunctionValue(name, func))
    
    return namespace
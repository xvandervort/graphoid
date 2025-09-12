"""
Glang Runtime Value System

Provides proper runtime value representation for glang, replacing
string-based value handling with typed value objects that support
proper operations and constraint validation.
"""

from abc import ABC, abstractmethod
from typing import Any, List, Optional, Union, Tuple
import sys
import os
from .glang_number import GlangNumber, create_glang_number

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../../..'))

from glang.ast.nodes import SourcePosition


class GlangValue(ABC):
    """Base class for all glang runtime values."""
    
    def __init__(self, position: Optional[SourcePosition] = None):
        self.position = position
        self.is_frozen = False
        self.contains_frozen = False
    
    @abstractmethod
    def to_python(self) -> Any:
        """Convert to Python equivalent."""
        pass
    
    @abstractmethod  
    def get_type(self) -> str:
        """Get glang type name."""
        pass
    
    @abstractmethod
    def to_display_string(self) -> str:
        """String for display to user."""
        pass
    
    def __str__(self) -> str:
        return self.to_display_string()
    
    def __repr__(self) -> str:
        return f"{self.__class__.__name__}({self.to_python()!r})"
    
    # Universal reflection methods - inherited by all node types
    def universal_type(self) -> 'StringValue':
        """Return the type name of this node."""
        return StringValue(self.get_type(), self.position)
    
    def universal_size(self) -> 'NumberValue':
        """Return the graph size (node count) of this value. 
        Default implementation: atomic nodes have size 1.
        Override in collection types for element count.
        """
        return NumberValue(1, self.position)
    
    def universal_inspect(self) -> 'StringValue':
        """Return detailed inspection information about this node.
        Default implementation shows value and type.
        Override in specific types for more details.
        """
        info = f"{self.to_display_string()} ({self.get_type()})"
        return StringValue(info, self.position)
    
    def universal_methods(self) -> 'ListValue':
        """Return list of all available methods for this node type.
        Must be implemented to return actual available methods.
        """
        # This will be implemented by the method resolution system
        # that has access to the full method registry
        raise NotImplementedError("universal_methods requires method registry access")
    
    def universal_can(self, method_name: str) -> 'BooleanValue':
        """Check if this node can execute the given method.
        Default implementation checks against the methods() list.
        """
        try:
            methods_list = self.universal_methods()
            method_names = [elem.value for elem in methods_list.elements]
            return BooleanValue(method_name in method_names, self.position)
        except NotImplementedError:
            # Fallback if methods() not available
            return BooleanValue(False, self.position)
    
    # Immutability methods
    def freeze(self) -> 'GlangValue':
        """Freeze this value, making it immutable.
        Returns self for method chaining.
        """
        self.is_frozen = True
        self.contains_frozen = True
        self._deep_freeze()
        return self
    
    def _deep_freeze(self):
        """Override in collection types to freeze contained elements."""
        pass
    
    def is_frozen_value(self) -> bool:
        """Check if this value is frozen."""
        return self.is_frozen
    
    def contains_frozen_data(self) -> bool:
        """Check if this value contains any frozen data."""
        return self.contains_frozen
    
    def can_accept_element(self, element: 'GlangValue') -> Tuple[bool, str]:
        """Check if this collection can accept the given element.
        Returns (can_accept, reason_if_not).
        Override in collection types for actual checking.
        """
        return True, ""
    
    def _check_not_frozen(self, operation: str):
        """Raise error if this value is frozen and operation would mutate it."""
        if self.is_frozen:
            raise RuntimeError(f"Cannot {operation}: value is frozen (immutable)")
    
    def _check_contamination_compatibility(self, element: 'GlangValue', operation: str):
        """Check if adding element would violate contamination rules."""
        if self.is_frozen:
            # Frozen collections cannot be modified anyway
            raise RuntimeError(f"Cannot {operation}: collection is frozen")
        
        if element.is_frozen_value() and not self.contains_frozen:
            # Adding frozen element to unfrozen collection
            raise RuntimeError(f"Cannot {operation}: cannot mix frozen and unfrozen data in same collection")
        
        if not element.is_frozen_value() and self.contains_frozen:
            # Adding unfrozen element to collection with frozen data
            raise RuntimeError(f"Cannot {operation}: cannot mix frozen and unfrozen data in same collection")


class CharNode(GlangValue):
    """Runtime character node for graph-based string operations."""
    
    def __init__(self, value: str, position: Optional[SourcePosition] = None):
        super().__init__(position)
        if len(value) != 1:
            raise ValueError("CharNode must contain exactly one character")
        self.value = value
    
    def to_python(self) -> str:
        return self.value
    
    def get_type(self) -> str:
        return "char"
    
    def to_display_string(self) -> str:
        return self.value
    
    def __eq__(self, other) -> bool:
        return isinstance(other, CharNode) and self.value == other.value
    
    # Character-specific operations using Glang semantics
    def is_alphabetic(self) -> bool:
        """Check if character is alphabetic."""
        return self.value.isalpha()
    
    def is_numeric(self) -> bool:
        """Check if character is numeric."""
        return self.value.isnumeric()
    
    def is_whitespace(self) -> bool:
        """Check if character is whitespace."""
        return self.value.isspace()
    
    def is_uppercase(self) -> bool:
        """Check if character is uppercase."""
        return self.value.isupper()
    
    def is_lowercase(self) -> bool:
        """Check if character is lowercase."""
        return self.value.islower()
    
    def to_uppercase(self) -> 'CharNode':
        """Convert character to uppercase."""
        return CharNode(self.value.upper(), self.position)
    
    def to_lowercase(self) -> 'CharNode':
        """Convert character to lowercase."""
        return CharNode(self.value.lower(), self.position)
    
    def compare_to(self, other: 'CharNode') -> int:
        """Compare characters using Glang semantics.
        Returns -1 if self < other, 0 if equal, 1 if self > other."""
        if not isinstance(other, CharNode):
            raise ValueError(f"Cannot compare char with {other.get_type()}")
        if self.value < other.value:
            return -1
        elif self.value > other.value:
            return 1
        else:
            return 0
    
    def is_whitespace(self) -> bool:
        """Check if character is whitespace."""
        return self.value.isspace()


class StringValue(GlangValue):
    """Runtime string value with graph operation support."""
    
    def __init__(self, value: str, position: Optional[SourcePosition] = None):
        super().__init__(position)
        self.value = value
        self._char_nodes = None  # Lazy conversion cache
    
    def to_python(self) -> str:
        return self.value
    
    def get_type(self) -> str:
        return "string"
    
    def to_display_string(self) -> str:
        return self.value
    
    def __eq__(self, other) -> bool:
        return isinstance(other, StringValue) and self.value == other.value
    
    # String operations using Glang semantics
    def concatenate(self, other: 'StringValue') -> 'StringValue':
        """Concatenate two strings using character node operations."""
        if not isinstance(other, StringValue):
            raise ValueError(f"Cannot concatenate string with {other.get_type()}")
        
        # Use character nodes for concatenation
        self_chars = self.to_char_nodes()
        other_chars = other.to_char_nodes()
        combined_chars = self_chars + other_chars
        
        return self.from_char_nodes(combined_chars)
    
    # Comparison operations using character node semantics
    def greater_than(self, other: 'StringValue') -> 'BooleanValue':
        """Compare if this string is greater than another using character node semantics."""
        if not isinstance(other, StringValue):
            raise ValueError(f"Cannot compare string with {other.get_type()}")
        return BooleanValue(self._compare_char_nodes(other) > 0)
    
    def less_than(self, other: 'StringValue') -> 'BooleanValue':
        """Compare if this string is less than another using character node semantics."""
        if not isinstance(other, StringValue):
            raise ValueError(f"Cannot compare string with {other.get_type()}")
        return BooleanValue(self._compare_char_nodes(other) < 0)
    
    def greater_equal(self, other: 'StringValue') -> 'BooleanValue':
        """Compare if this string is greater than or equal to another using character node semantics."""
        if not isinstance(other, StringValue):
            raise ValueError(f"Cannot compare string with {other.get_type()}")
        return BooleanValue(self._compare_char_nodes(other) >= 0)
    
    def less_equal(self, other: 'StringValue') -> 'BooleanValue':
        """Compare if this string is less than or equal to another using character node semantics."""
        if not isinstance(other, StringValue):
            raise ValueError(f"Cannot compare string with {other.get_type()}")
        return BooleanValue(self._compare_char_nodes(other) <= 0)
    
    def _compare_char_nodes(self, other: 'StringValue') -> int:
        """Compare two strings character by character using CharNode semantics.
        Returns -1 if self < other, 0 if equal, 1 if self > other."""
        self_chars = self.to_char_nodes()
        other_chars = other.to_char_nodes()
        
        # Compare character by character
        min_len = min(len(self_chars), len(other_chars))
        for i in range(min_len):
            char_comparison = self_chars[i].compare_to(other_chars[i])
            if char_comparison != 0:
                return char_comparison
        
        # If all compared characters are equal, the shorter string is "less"
        if len(self_chars) < len(other_chars):
            return -1
        elif len(self_chars) > len(other_chars):
            return 1
        else:
            return 0
    
    def to_char_nodes(self) -> List['CharNode']:
        """Convert string to list of character nodes for graph operations."""
        if self._char_nodes is None:
            self._char_nodes = [CharNode(char, self.position) for char in self.value]
        return self._char_nodes
    
    def from_char_nodes(self, char_nodes: List['CharNode']) -> 'StringValue':
        """Create new StringValue from list of character nodes."""
        result_string = ''.join(node.value for node in char_nodes)
        return StringValue(result_string, self.position)
    
    def clear_char_cache(self):
        """Clear the character node cache."""
        self._char_nodes = None
    
    # String manipulation operations using character nodes
    def get_char_at(self, index: int) -> 'StringValue':
        """Get character at index using character node operations."""
        char_nodes = self.to_char_nodes()
        
        # Handle negative indices
        if index < 0:
            index = len(char_nodes) + index
        
        if index < 0 or index >= len(char_nodes):
            raise IndexError(f"String index {index} out of range for string of length {len(char_nodes)}")
        return StringValue(char_nodes[index].value, self.position)
    
    def to_upper(self) -> 'StringValue':
        """Convert string to uppercase using character node operations."""
        char_nodes = self.to_char_nodes()
        upper_chars = [node.to_uppercase() for node in char_nodes]
        return self.from_char_nodes(upper_chars)
    
    def to_lower(self) -> 'StringValue':
        """Convert string to lowercase using character node operations."""
        char_nodes = self.to_char_nodes()
        lower_chars = [node.to_lowercase() for node in char_nodes]
        return self.from_char_nodes(lower_chars)
    
    def trim(self) -> 'StringValue':
        """Trim whitespace from both ends using character node operations."""
        char_nodes = self.to_char_nodes()
        
        # Find first non-whitespace character
        start = 0
        while start < len(char_nodes) and char_nodes[start].is_whitespace():
            start += 1
        
        # Find last non-whitespace character
        end = len(char_nodes) - 1
        while end >= start and char_nodes[end].is_whitespace():
            end -= 1
        
        # Return trimmed string
        if start > end:
            # All characters were whitespace
            return StringValue("", self.position)
        else:
            trimmed_chars = char_nodes[start:end + 1]
            return self.from_char_nodes(trimmed_chars)
    
    def reverse(self) -> 'StringValue':
        """Reverse string using character node operations."""
        char_nodes = self.to_char_nodes()
        reversed_chars = char_nodes[::-1]
        return self.from_char_nodes(reversed_chars)
    
    def split(self, delimiter: 'StringValue') -> 'ListValue':
        """Split string by delimiter using character node operations."""
        if not isinstance(delimiter, StringValue):
            raise ValueError(f"Split delimiter must be string, got {delimiter.get_type()}")
        
        # For now, use Python split but convert results to proper StringValues
        # TODO: Implement full character node-based splitting algorithm
        parts = self.value.split(delimiter.value)
        string_values = [StringValue(part, self.position) for part in parts]
        
        return ListValue(string_values, "string", self.position)
    
    def join(self, string_list: 'ListValue') -> 'StringValue':
        """Join list of strings with this string as delimiter using character node operations."""
        if not isinstance(string_list, ListValue):
            raise ValueError(f"Join target must be list, got {string_list.get_type()}")
        
        if not string_list.elements:
            return StringValue("", self.position)
        
        # Verify all elements are strings
        for element in string_list.elements:
            if not isinstance(element, StringValue):
                raise ValueError(f"All elements must be strings, got {element.get_type()}")
        
        # Use character node operations for joining
        result_chars = []
        delimiter_chars = self.to_char_nodes()
        
        for i, element in enumerate(string_list.elements):
            if i > 0:
                # Add delimiter between elements
                result_chars.extend(delimiter_chars)
            result_chars.extend(element.to_char_nodes())
        
        return self.from_char_nodes(result_chars)
    
    # Override universal methods for string-specific behavior
    def universal_size(self) -> 'NumberValue':
        """For strings: size is the number of character nodes."""
        return NumberValue(len(self.value), self.position)
    
    def universal_inspect(self) -> 'StringValue':
        """String-specific inspection showing character count."""
        info = f'"{self.value}" (string, {len(self.value)} chars)'
        return StringValue(info, self.position)


class NumberValue(GlangValue):
    """Runtime number value using Glang's custom number system."""
    
    def __init__(self, value: Union[int, float, str, bool, GlangNumber], position: Optional[SourcePosition] = None):
        super().__init__(position)
        if isinstance(value, GlangNumber):
            self.glang_number = value
        else:
            self.glang_number = create_glang_number(value)
    
    @property
    def value(self) -> Union[int, float]:
        """Compatibility property for existing code that accesses .value directly."""
        # Return Python equivalent for backwards compatibility
        if self.glang_number.is_integer():
            return self.glang_number.to_python_int()
        else:
            return self.glang_number.to_python_float()
    
    def to_python(self) -> Union[int, float]:
        return self.value
    
    def get_type(self) -> str:
        return "num"
    
    def to_display_string(self) -> str:
        return self.glang_number.to_string()
    
    def __eq__(self, other) -> bool:
        return isinstance(other, NumberValue) and self.glang_number == other.glang_number
    
    # Arithmetic operations using Glang semantics
    def add(self, other: 'NumberValue') -> 'NumberValue':
        """Add two numbers using Glang arithmetic semantics."""
        if not isinstance(other, NumberValue):
            raise ValueError(f"Cannot add {other.get_type()} to number")
        result = self.glang_number.add(other.glang_number)
        return NumberValue(result, self.position)
    
    def subtract(self, other: 'NumberValue') -> 'NumberValue':
        """Subtract two numbers using Glang arithmetic semantics."""
        if not isinstance(other, NumberValue):
            raise ValueError(f"Cannot subtract {other.get_type()} from number")
        result = self.glang_number.subtract(other.glang_number)
        return NumberValue(result, self.position)
    
    def multiply(self, other: 'NumberValue') -> 'NumberValue':
        """Multiply two numbers using Glang arithmetic semantics."""
        if not isinstance(other, NumberValue):
            raise ValueError(f"Cannot multiply number by {other.get_type()}")
        result = self.glang_number.multiply(other.glang_number)
        return NumberValue(result, self.position)
    
    def divide(self, other: 'NumberValue') -> 'NumberValue':
        """Divide two numbers using Glang arithmetic semantics."""
        if not isinstance(other, NumberValue):
            raise ValueError(f"Cannot divide number by {other.get_type()}")
        try:
            result = self.glang_number.divide(other.glang_number)
            return NumberValue(result, self.position)
        except ZeroDivisionError:
            raise ValueError("Division by zero")
    
    def modulo(self, other: 'NumberValue') -> 'NumberValue':
        """Perform modulo operation using Glang arithmetic semantics."""
        if not isinstance(other, NumberValue):
            raise ValueError(f"Cannot perform modulo on number with {other.get_type()}")
        try:
            result = self.glang_number.modulo(other.glang_number)
            return NumberValue(result, self.position)
        except ZeroDivisionError:
            raise ValueError("Modulo by zero")
    
    # Comparison operations using Glang semantics
    def greater_than(self, other: 'NumberValue') -> 'BooleanValue':
        """Compare if this number is greater than another using Glang semantics."""
        if not isinstance(other, NumberValue):
            raise ValueError(f"Cannot compare number with {other.get_type()}")
        comparison = self.glang_number.compare_to(other.glang_number)
        return BooleanValue(comparison > 0, self.position)
    
    def less_than(self, other: 'NumberValue') -> 'BooleanValue':
        """Compare if this number is less than another using Glang semantics."""
        if not isinstance(other, NumberValue):
            raise ValueError(f"Cannot compare number with {other.get_type()}")
        comparison = self.glang_number.compare_to(other.glang_number)
        return BooleanValue(comparison < 0, self.position)
    
    def greater_equal(self, other: 'NumberValue') -> 'BooleanValue':
        """Compare if this number is greater than or equal to another using Glang semantics."""
        if not isinstance(other, NumberValue):
            raise ValueError(f"Cannot compare number with {other.get_type()}")
        comparison = self.glang_number.compare_to(other.glang_number)
        return BooleanValue(comparison >= 0, self.position)
    
    def less_equal(self, other: 'NumberValue') -> 'BooleanValue':
        """Compare if this number is less than or equal to another using Glang semantics."""
        if not isinstance(other, NumberValue):
            raise ValueError(f"Cannot compare number with {other.get_type()}")
        comparison = self.glang_number.compare_to(other.glang_number)
        return BooleanValue(comparison <= 0, self.position)
    
    # Convenience methods for common operations
    def negate(self) -> 'NumberValue':
        """Return the negative of this number."""
        result = self.glang_number.negate()
        return NumberValue(result, self.position)
    
    def absolute(self) -> 'NumberValue':
        """Return the absolute value of this number."""
        result = self.glang_number.absolute()
        return NumberValue(result, self.position)
    
    def power(self, other: 'NumberValue') -> 'NumberValue':
        """Raise this number to the power of other."""
        if not isinstance(other, NumberValue):
            raise ValueError(f"Cannot raise number to power of {other.get_type()}")
        try:
            result = self.glang_number.power(other.glang_number)
            return NumberValue(result, self.position)
        except Exception as e:
            raise ValueError(str(e))
    
    # Mathematical methods that delegate to GlangNumber
    def sqrt(self) -> 'NumberValue':
        """Calculate square root using Glang number system."""
        result = self.glang_number.sqrt()
        return NumberValue(result, self.position)
    
    def ceil(self) -> 'NumberValue':
        """Return ceiling using Glang number system."""
        result = self.glang_number.ceil()
        return NumberValue(result, self.position)
    
    def floor(self) -> 'NumberValue':
        """Return floor using Glang number system."""
        result = self.glang_number.floor()
        return NumberValue(result, self.position)
    
    def round_to_precision(self, precision: int) -> 'NumberValue':
        """Round to specified decimal places using Glang rounding rules."""
        result = self.glang_number.round_to_precision(precision)
        return NumberValue(result, self.position)


class BooleanValue(GlangValue):
    """Runtime boolean value."""
    
    def __init__(self, value: bool, position: Optional[SourcePosition] = None):
        super().__init__(position)
        self.value = value
        
    def to_python(self) -> bool:
        return self.value
    
    def get_type(self) -> str:
        return "bool"
    
    def to_display_string(self) -> str:
        return str(self.value).lower()  # true/false instead of True/False
    
    def __eq__(self, other) -> bool:
        return isinstance(other, BooleanValue) and self.value == other.value
    
    # Override universal methods for boolean-specific behavior
    def universal_inspect(self) -> 'StringValue':
        """Boolean-specific inspection."""
        info = f"{str(self.value).lower()} (bool)"
        return StringValue(info, self.position)


class DataValue(GlangValue):
    """Runtime data node value with immutable key and mutable value."""
    
    def __init__(self, key: str, value: GlangValue, constraint: Optional[str] = None,
                 position: Optional[SourcePosition] = None):
        super().__init__(position)
        self.key = key  # Immutable string key
        self.value = value  # Mutable value
        self.constraint = constraint  # Optional type constraint for value
        # Check if value is frozen
        self._update_frozen_flag()
    
    def to_python(self) -> dict:
        return {self.key: self.value.to_python()}
    
    def get_type(self) -> str:
        return "data"
    
    def to_display_string(self) -> str:
        return f'{{ "{self.key}": {self.value.to_display_string()} }}'
    
    def validate_constraint(self, value: GlangValue) -> bool:
        """Check if value matches data node constraint."""
        if not self.constraint:
            return True
        return value.get_type() == self.constraint
    
    def set_value(self, new_value: GlangValue) -> None:
        """Set the value (with constraint validation)."""
        self._check_not_frozen("set value")
        self._check_contamination_compatibility(new_value, "set value")
        
        if not self.validate_constraint(new_value):
            from .errors import TypeConstraintError
            raise TypeConstraintError(
                f"Cannot assign {new_value.get_type()} to data<{self.constraint}>",
                new_value.position
            )
        
        old_value = self.value
        self.value = new_value
        self._update_frozen_flag()
    
    def get_key(self) -> StringValue:
        """Get the key as a StringValue."""
        return StringValue(self.key, self.position)
    
    def get_value(self) -> GlangValue:
        """Get the value."""
        return self.value
    
    def __eq__(self, other) -> bool:
        return (isinstance(other, DataValue) and 
                self.key == other.key and
                self.value == other.value and
                self.constraint == other.constraint)
    
    # Override universal methods for data-specific behavior
    def universal_size(self) -> 'NumberValue':
        """For data nodes: size is always 1 (single key-value pair)."""
        return NumberValue(1, self.position)
    
    def universal_inspect(self) -> 'StringValue':
        """Data-specific inspection showing key and value type."""
        constraint_info = f"<{self.constraint}>" if self.constraint else ""
        info = f'data{constraint_info} {{ "{self.key}": {self.value.get_type()} }}'
        return StringValue(info, self.position)
    
    # Immutability-specific methods
    def _update_frozen_flag(self):
        """Update the contains_frozen flag based on value."""
        self.contains_frozen = self.value.is_frozen_value() or self.value.contains_frozen_data()
    
    def _deep_freeze(self):
        """Freeze the contained value."""
        self.value.freeze()
    
    def can_accept_value(self, new_value: GlangValue) -> Tuple[bool, str]:
        """Check if this data node can accept the given value."""
        # Check constraint first
        if not self.validate_constraint(new_value):
            return False, f"Value type {new_value.get_type()} does not match constraint {self.constraint}"
        
        # Check contamination rules
        if self.is_frozen:
            return False, "Cannot modify frozen data node"
        
        if new_value.is_frozen_value() and not self.contains_frozen:
            return False, "Cannot mix frozen and unfrozen data in same collection"
        
        if not new_value.is_frozen_value() and self.contains_frozen:
            return False, "Cannot mix frozen and unfrozen data in same collection"
        
        return True, ""


class GlangHashTable:
    """A simple hash table implementation using Glang semantics."""
    
    def __init__(self):
        self.buckets = [[] for _ in range(16)]  # Start with 16 buckets
        self.size = 0
    
    def _hash(self, key: str) -> int:
        """Simple hash function for strings using Glang semantics."""
        # Use Python's built-in hash but modulo our bucket count
        # In the future, this could be replaced with a custom hash function
        return hash(key) % len(self.buckets)
    
    def get(self, key: str) -> Optional[GlangValue]:
        """Get value by key using Glang semantics."""
        bucket_idx = self._hash(key)
        bucket = self.buckets[bucket_idx]
        
        for stored_key, value in bucket:
            if stored_key == key:  # String equality is straightforward
                return value
        return None
    
    def set(self, key: str, value: GlangValue) -> None:
        """Set value for key using Glang semantics."""
        bucket_idx = self._hash(key)
        bucket = self.buckets[bucket_idx]
        
        # Check if key already exists
        for i, (stored_key, stored_value) in enumerate(bucket):
            if stored_key == key:
                bucket[i] = (key, value)  # Replace existing
                return
        
        # Key doesn't exist, add new entry
        bucket.append((key, value))
        self.size += 1
        
        # Resize if load factor gets too high
        if self.size > len(self.buckets) * 2:
            self._resize()
    
    def has_key(self, key: str) -> bool:
        """Check if key exists using Glang semantics."""
        return self.get(key) is not None
    
    def keys(self) -> List[str]:
        """Get all keys in insertion order (approximately)."""
        result = []
        for bucket in self.buckets:
            for key, _ in bucket:
                result.append(key)
        return result
    
    def values(self) -> List[GlangValue]:
        """Get all values in insertion order (approximately)."""
        result = []
        for bucket in self.buckets:
            for _, value in bucket:
                result.append(value)
        return result
    
    def items(self) -> List[Tuple[str, GlangValue]]:
        """Get all key-value pairs."""
        result = []
        for bucket in self.buckets:
            for key, value in bucket:
                result.append((key, value))
        return result
    
    def remove(self, key: str) -> bool:
        """Remove key-value pair. Returns True if key existed."""
        bucket_idx = self._hash(key)
        bucket = self.buckets[bucket_idx]
        
        for i, (stored_key, stored_value) in enumerate(bucket):
            if stored_key == key:
                del bucket[i]
                self.size -= 1
                return True
        return False
    
    def __len__(self) -> int:
        """Get number of key-value pairs."""
        return self.size
    
    def __contains__(self, key: str) -> bool:
        """Support Python's 'in' operator for backward compatibility."""
        return self.has_key(key)
    
    def _resize(self) -> None:
        """Resize hash table when load factor gets too high."""
        old_buckets = self.buckets
        self.buckets = [[] for _ in range(len(old_buckets) * 2)]
        old_size = self.size
        self.size = 0
        
        # Rehash all existing entries
        for bucket in old_buckets:
            for key, value in bucket:
                self.set(key, value)


class HashValue(GlangValue):
    """Runtime hash value - collection of data nodes (key-value pairs)."""
    
    def __init__(self, pairs: List[Tuple[str, GlangValue]], constraint: Optional[str] = None,
                 position: Optional[SourcePosition] = None):
        super().__init__(position)
        # Use Glang hash table instead of Python dict
        self.pairs = GlangHashTable()
        for key, value in pairs:
            self.pairs.set(key, value)
        self.constraint = constraint  # Optional type constraint for all values
        # Check if any values are frozen
        self._update_frozen_flag()
    
    def to_python(self) -> dict:
        return {key: value.to_python() for key, value in self.pairs.items()}
    
    def get_type(self) -> str:
        return "hash"
    
    def to_display_string(self) -> str:
        if not self.pairs:
            return "{}"
        
        pair_strs = []
        for key, value in self.pairs.items():
            pair_strs.append(f'"{key}": {value.to_display_string()}')
        return "{ " + ", ".join(pair_strs) + " }"
    
    def validate_constraint(self, value: GlangValue) -> bool:
        """Check if value matches map constraint."""
        if not self.constraint:
            return True
        return value.get_type() == self.constraint
    
    def get(self, key: str) -> Optional[GlangValue]:
        """Get value by key."""
        return self.pairs.get(key)
    
    def set(self, key: str, value: GlangValue) -> None:
        """Set value for key (with constraint validation)."""
        self._check_not_frozen("set key")
        self._check_contamination_compatibility(value, "set key")
        
        if not self.validate_constraint(value):
            from .errors import TypeConstraintError
            raise TypeConstraintError(
                f"Cannot assign {value.get_type()} to hash<{self.constraint}>",
                value.position
            )
        
        self.pairs.set(key, value)
        self._update_frozen_flag()
    
    def has_key(self, key: str) -> bool:
        """Check if key exists in map."""
        return self.pairs.has_key(key)
    
    def keys(self) -> List[str]:
        """Get all keys."""
        return self.pairs.keys()
    
    def values(self) -> List[GlangValue]:
        """Get all values."""
        return self.pairs.values()
    
    def remove(self, key: str) -> bool:
        """Remove key-value pair. Returns True if key existed."""
        self._check_not_frozen("remove key")
        
        if self.pairs.remove(key):
            self._update_frozen_flag()
            return True
        return False
    
    def __eq__(self, other) -> bool:
        return self.equals(other) if isinstance(other, HashValue) else False
    
    def equals(self, other: 'HashValue') -> bool:
        """Compare two hashes using Glang equality semantics."""
        if not isinstance(other, HashValue):
            return False
        
        if len(self.pairs) != len(other.pairs):
            return False
        
        if self.constraint != other.constraint:
            return False
        
        # Check that all keys exist and values are equal
        for key, value in self.pairs.items():
            if key not in other.pairs:
                return False
            # Import ListValue's _glang_equals for comparison
            from .values import ListValue
            if not ListValue._glang_equals(value, other.pairs[key]):
                return False
        
        return True
    
    # Override universal methods for map-specific behavior
    def universal_size(self) -> 'NumberValue':
        """For maps: size is the number of key-value pairs."""
        return NumberValue(len(self.pairs), self.position)
    
    def universal_inspect(self) -> 'StringValue':
        """Map-specific inspection showing constraint and size."""
        constraint_info = f"<{self.constraint}>" if self.constraint else ""
        info = f'hash{constraint_info} ({len(self.pairs)} pairs)'
        return StringValue(info, self.position)
    
    # Immutability-specific methods
    def _update_frozen_flag(self):
        """Update the contains_frozen flag based on values."""
        self.contains_frozen = any(value.is_frozen_value() or value.contains_frozen_data() 
                                 for value in self.pairs.values())
    
    def _deep_freeze(self):
        """Freeze all contained values."""
        for value in self.pairs.values():
            value.freeze()
    
    def can_accept_value(self, value: GlangValue) -> Tuple[bool, str]:
        """Check if this map can accept the given value."""
        # Check constraint first
        if not self.validate_constraint(value):
            return False, f"Value type {value.get_type()} does not match constraint {self.constraint}"
        
        # Check contamination rules
        if self.is_frozen:
            return False, "Cannot modify frozen map"
        
        if value.is_frozen_value() and not self.contains_frozen:
            return False, "Cannot mix frozen and unfrozen data in same collection"
        
        if not value.is_frozen_value() and self.contains_frozen:
            return False, "Cannot mix frozen and unfrozen data in same collection"
        
        return True, ""


class ListValue(GlangValue):
    """Runtime list value with optional type constraints."""
    
    def __init__(self, elements: List[GlangValue], constraint: Optional[str] = None, 
                 position: Optional[SourcePosition] = None):
        super().__init__(position)
        self.elements = elements
        self.constraint = constraint
        # Check if any elements are frozen
        self._update_frozen_flag()
    
    def to_python(self) -> List[Any]:
        return [elem.to_python() for elem in self.elements]
    
    def get_type(self) -> str:
        return "list"
    
    def to_display_string(self) -> str:
        element_strs = [elem.to_display_string() for elem in self.elements]
        return f"[{', '.join(element_strs)}]"
    
    def validate_constraint(self, value: GlangValue) -> bool:
        """Check if value matches list constraint."""
        if not self.constraint:
            return True
        return value.get_type() == self.constraint
    
    def append(self, value: GlangValue) -> None:
        """Append value to list (with constraint validation)."""
        self._check_not_frozen("append")
        self._check_contamination_compatibility(value, "append")
        
        if not self.validate_constraint(value):
            from .errors import TypeConstraintError
            raise TypeConstraintError(
                f"Cannot append {value.get_type()} to list<{self.constraint}>",
                value.position
            )
        
        self.elements.append(value)
        self._update_frozen_flag()
    
    def get_element(self, index: int) -> GlangValue:
        """Get element at index (with bounds checking)."""
        if not -len(self.elements) <= index < len(self.elements):
            from .errors import RuntimeError
            raise RuntimeError(f"List index {index} out of range", self.position)
        return self.elements[index]
    
    def set_element(self, index: int, value: GlangValue) -> None:
        """Set element at index (with constraint validation)."""
        if not -len(self.elements) <= index < len(self.elements):
            from .errors import RuntimeError
            raise RuntimeError(f"List index {index} out of range", value.position)
        
        self._check_not_frozen("set element")
        self._check_contamination_compatibility(value, "set element")
        
        if not self.validate_constraint(value):
            from .errors import TypeConstraintError
            raise TypeConstraintError(
                f"Cannot assign {value.get_type()} to list<{self.constraint}>",
                value.position
            )
        
        self.elements[index] = value
        self._update_frozen_flag()
    
    def __len__(self) -> int:
        return len(self.elements)
    
    def __eq__(self, other) -> bool:
        return self.equals(other) if isinstance(other, ListValue) else False
    
    def contains(self, value: GlangValue) -> bool:
        """Check if this list contains the given value using Glang equality semantics."""
        for element in self.elements:
            if self._glang_equals(element, value):
                return True
        return False
    
    def equals(self, other: 'ListValue') -> bool:
        """Compare two lists element-by-element using Glang equality semantics."""
        if not isinstance(other, ListValue):
            return False
        
        if len(self.elements) != len(other.elements):
            return False
        
        if self.constraint != other.constraint:
            return False
        
        for i in range(len(self.elements)):
            if not self._glang_equals(self.elements[i], other.elements[i]):
                return False
        
        return True
    
    @staticmethod
    def _glang_equals(left: GlangValue, right: GlangValue) -> bool:
        """Compare two Glang values using Glang's equality semantics."""
        # Different types are never equal
        if left.get_type() != right.get_type():
            return False
        
        # Use type-specific comparison
        if isinstance(left, NumberValue) and isinstance(right, NumberValue):
            return left.value == right.value
        elif isinstance(left, StringValue) and isinstance(right, StringValue):
            return left.value == right.value
        elif isinstance(left, BooleanValue) and isinstance(right, BooleanValue):
            return left.value == right.value
        elif isinstance(left, ListValue) and isinstance(right, ListValue):
            return left.equals(right)
        elif isinstance(left, DataValue) and isinstance(right, DataValue):
            return left.key == right.key and ListValue._glang_equals(left.value, right.value)
        elif isinstance(left, HashValue) and isinstance(right, HashValue):
            return left.equals(right)
        elif isinstance(left, NoneValue) and isinstance(right, NoneValue):
            return True
        else:
            # For any unknown types, fall back to Python equality
            # This should not happen in practice
            return left == right
    
    @staticmethod
    def _glang_compare(left: GlangValue, right: GlangValue) -> int:
        """Compare two Glang values using Glang's comparison semantics.
        
        Returns:
            -1 if left < right
             0 if left == right
             1 if left > right
        """
        # Different types cannot be compared (except for special cases)
        if left.get_type() != right.get_type():
            raise ValueError(f"Cannot compare {left.get_type()} with {right.get_type()}")
        
        # Use type-specific comparison
        if isinstance(left, NumberValue) and isinstance(right, NumberValue):
            return left.glang_number.compare_to(right.glang_number)
        elif isinstance(left, StringValue) and isinstance(right, StringValue):
            if left.value < right.value:
                return -1
            elif left.value > right.value:
                return 1
            else:
                return 0
        elif isinstance(left, BooleanValue) and isinstance(right, BooleanValue):
            # False < True in Glang
            if left.value < right.value:
                return -1
            elif left.value > right.value:
                return 1
            else:
                return 0
        else:
            raise ValueError(f"Comparison not supported for {left.get_type()}")
    
    # Immutability-specific methods
    def _update_frozen_flag(self):
        """Update the contains_frozen flag based on elements."""
        self.contains_frozen = any(elem.is_frozen_value() or elem.contains_frozen_data() 
                                 for elem in self.elements)
    
    def _deep_freeze(self):
        """Freeze all contained elements."""
        for element in self.elements:
            element.freeze()
    
    def can_accept_element(self, element: GlangValue) -> Tuple[bool, str]:
        """Check if this list can accept the given element."""
        # Check constraint first
        if not self.validate_constraint(element):
            return False, f"Element type {element.get_type()} does not match constraint {self.constraint}"
        
        # Check contamination rules
        if self.is_frozen:
            return False, "Cannot add to frozen list"
        
        if element.is_frozen_value() and not self.contains_frozen:
            return False, "Cannot mix frozen and unfrozen data in same collection"
        
        if not element.is_frozen_value() and self.contains_frozen:
            return False, "Cannot mix frozen and unfrozen data in same collection"
        
        return True, ""
    
    # Override universal methods for list-specific behavior
    def universal_size(self) -> 'NumberValue':
        """For lists: size is the number of element nodes."""
        return NumberValue(len(self.elements), self.position)
    
    def universal_inspect(self) -> 'StringValue':
        """List-specific inspection showing constraint and element count."""
        constraint_info = f"<{self.constraint}>" if self.constraint else ""
        info = f"list{constraint_info} with {len(self.elements)} elements"
        return StringValue(info, self.position)


def python_to_glang_value(python_value: Any, position: Optional[SourcePosition] = None) -> GlangValue:
    """Convert Python value to appropriate GlangValue."""
    if isinstance(python_value, str):
        return StringValue(python_value, position)
    elif isinstance(python_value, bool):  # Check bool before int/float since bool is subclass of int
        return BooleanValue(python_value, position)
    elif isinstance(python_value, (int, float)):
        return NumberValue(python_value, position)
    elif isinstance(python_value, dict):
        # For dict, we only handle single key-value pairs as DataValue
        if len(python_value) == 1:
            key = list(python_value.keys())[0]
            value = python_to_glang_value(python_value[key], position)
            return DataValue(key, value, None, position)
        else:
            # Multi-key dicts will become datamap in the future
            raise ValueError("Multi-key dictionaries not yet supported")
    elif isinstance(python_value, list):
        elements = [python_to_glang_value(elem, position) for elem in python_value]
        return ListValue(elements, None, position)
    else:
        # Fallback: convert to string
        return StringValue(str(python_value), position)


def glang_value_to_python(glang_value: GlangValue) -> Any:
    """Convert GlangValue to Python equivalent."""
    return glang_value.to_python()


def infer_type_from_value(value: GlangValue) -> str:
    """Infer glang type name from GlangValue for display purposes."""
    return value.get_type()


class FunctionValue(GlangValue):
    """Runtime function value representing a user-defined function."""
    
    def __init__(self, name: str, parameters: List[str], body: 'Block', closure_context: Optional['ExecutionContext'] = None, position: Optional[SourcePosition] = None):
        super().__init__(position)
        self.name = name
        self.parameters = parameters
        self.body = body
        self.closure_context = closure_context  # For closures (later enhancement)
        
    def to_python(self) -> str:
        return f"<function {self.name}>"
    
    def get_type(self) -> str:
        return "function"
    
    def to_display_string(self) -> str:
        param_list = ", ".join(self.parameters)
        return f"func {self.name}({param_list}) {{ ... }}"
    
    def arity(self) -> int:
        """Return number of parameters this function expects."""
        return len(self.parameters)


class LambdaValue(GlangValue):
    """Runtime lambda value representing an anonymous function."""
    
    def __init__(self, parameters: List[str], body: 'Expression', closure_context: Optional['ExecutionContext'] = None, position: Optional[SourcePosition] = None):
        super().__init__(position)
        self.parameters = parameters
        self.body = body
        self.closure_context = closure_context  # For closures (later enhancement)
        
    def to_python(self) -> str:
        return f"<lambda>"
    
    def get_type(self) -> str:
        return "lambda"
    
    def to_display_string(self) -> str:
        param_list = ", ".join(self.parameters)
        if len(self.parameters) == 1:
            return f"{param_list} => ..."
        else:
            return f"({param_list}) => ..."
    
    def arity(self) -> int:
        """Return number of parameters this lambda expects."""
        return len(self.parameters)


class NoneValue(GlangValue):
    """Runtime value representing absence of value (like null/None)."""
    
    def __init__(self, position: Optional[SourcePosition] = None):
        super().__init__(position)
        
    def to_python(self) -> None:
        return None
    
    def get_type(self) -> str:
        return "none"
    
    def to_display_string(self) -> str:
        return "none"


class TimeValue(GlangValue):
    """A Glang Time value representing a point in time (internally UTC timestamp)."""
    
    def __init__(self, timestamp: Union[int, float, GlangNumber], position: Optional[SourcePosition] = None):
        super().__init__(position)
        if isinstance(timestamp, (int, float)):
            self.timestamp = create_glang_number(timestamp)
        elif hasattr(timestamp, 'to_python_float'):  # GlangNumber
            self.timestamp = timestamp
        else:
            self.timestamp = create_glang_number(float(timestamp))
    
    def get_type(self) -> str:
        return "time"
    
    def to_python(self) -> float:
        """Return the timestamp as a float."""
        return self.timestamp.to_python_float()
    
    def to_string(self) -> str:
        """Default string representation as ISO datetime."""
        import datetime as python_datetime
        dt = python_datetime.datetime.fromtimestamp(self.to_python(), tz=python_datetime.timezone.utc)
        return dt.strftime("%Y-%m-%dT%H:%M:%SZ")
    
    def to_display_string(self) -> str:
        return self.to_string()
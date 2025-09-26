"""
Glang Runtime Value System

Provides proper runtime value representation for glang, replacing
string-based value handling with typed value objects that support
proper operations and constraint validation.
"""

from abc import ABC, abstractmethod
from typing import Any, Dict, List, Optional, Union, Tuple
import sys
import os
import uuid
from .glang_number import GlangNumber, create_glang_number
from ..graph_container import GraphContainer
from .errors import RuntimeError

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../../..'))

from glang.ast.nodes import SourcePosition




class GlangValue(ABC):
    """Base class for all glang runtime values.

    Provides default implementations for common methods to reduce boilerplate.
    Subclasses can override these defaults when needed.
    """

    # Class attribute to define the type name (overridden in subclasses)
    _type_name: Optional[str] = None

    def __init__(self, position: Optional[SourcePosition] = None):
        self.position = position
        self.is_frozen = False
        self.contains_frozen = False
        self._graph_node = None  # Reference to GraphNode when in a graph

    @abstractmethod
    def to_python(self) -> Any:
        """Convert to Python equivalent."""
        pass

    def get_type(self) -> str:
        """Get glang type name.

        Default implementation returns the class-level _type_name.
        Override this method for dynamic type names.
        """
        if self._type_name is not None:
            return self._type_name
        # Fallback: derive from class name (remove 'Value' suffix)
        class_name = self.__class__.__name__
        if class_name.endswith('Value'):
            return class_name[:-5].lower()
        return class_name.lower()

    def to_display_string(self) -> str:
        """String for display to user.

        Default implementation uses to_python() for simple types.
        Override for custom display formatting.
        """
        return str(self.to_python())
    
    def __str__(self) -> str:
        return self.to_display_string()

    @property
    def node(self):
        """Access the graph node wrapper for this value."""
        if self._graph_node is None:
            from .errors import GraphError
            raise GraphError(f"Value {self.to_display_string()} is not part of a graph. Cannot access .node property.")
        return self._graph_node
    
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

    _type_name = "char"

    def __init__(self, value: str, position: Optional[SourcePosition] = None):
        super().__init__(position)
        # Import Unicode utilities to validate grapheme clusters
        from .unicode_utils import UnicodeUtils

        # Accept grapheme clusters (what users perceive as single characters)
        # This includes single characters, emoji, combining characters, etc.
        grapheme_count = UnicodeUtils.grapheme_length(value)
        if grapheme_count != 1:
            raise ValueError(f"CharNode must contain exactly one grapheme cluster, got {grapheme_count}")
        self.value = value

    def to_python(self) -> str:
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

    _type_name = "string"

    def __init__(self, value: str, position: Optional[SourcePosition] = None):
        super().__init__(position)
        self.value = value
        self._char_nodes = None  # Lazy conversion cache

    def to_python(self) -> str:
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
            # Import Unicode utilities for proper grapheme cluster handling
            from .unicode_utils import UnicodeUtils

            # Split into grapheme clusters instead of individual code points
            grapheme_clusters = UnicodeUtils.grapheme_clusters(self.value)
            self._char_nodes = [CharNode(cluster, self.position) for cluster in grapheme_clusters]
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
    
    def split(self, delimiter: 'StringValue') -> 'GlangValue':
        """Split string by delimiter using character node operations."""
        # Import locally to avoid circular imports
        from .graph_values import ListValue

        if not isinstance(delimiter, StringValue):
            raise ValueError(f"Split delimiter must be string, got {delimiter.get_type()}")

        # For now, use Python split but convert results to proper StringValues
        # TODO: Implement full character node-based splitting algorithm
        parts = self.value.split(delimiter.value)
        string_values = [StringValue(part, self.position) for part in parts]

        return ListValue(string_values, "string", self.position)
    
    def join(self, string_list: 'GlangValue') -> 'StringValue':
        """Join list of strings with this string as delimiter using character node operations."""
        # Import locally to avoid circular imports
        from .graph_values import ListValue

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
    
    def starts_with(self, prefix: 'StringValue') -> 'BooleanValue':
        """Check if string starts with the given prefix."""
        if not isinstance(prefix, StringValue):
            raise ValueError(f"Prefix must be string, got {prefix.get_type()}")
        
        # Use Python's built-in for now, but could be implemented with char nodes
        result = self.value.startswith(prefix.value)
        return BooleanValue(result, self.position)
    
    def ends_with(self, suffix: 'StringValue') -> 'BooleanValue':
        """Check if string ends with the given suffix."""
        if not isinstance(suffix, StringValue):
            raise ValueError(f"Suffix must be string, got {suffix.get_type()}")
        
        # Use Python's built-in for now, but could be implemented with char nodes
        result = self.value.endswith(suffix.value)
        return BooleanValue(result, self.position)
    
    # Override universal methods for string-specific behavior
    def universal_size(self) -> 'NumberValue':
        """For strings: size is the number of character nodes."""
        return NumberValue(len(self.value), self.position)
    
    def universal_inspect(self) -> 'StringValue':
        """String-specific inspection showing character count."""
        info = f'"{self.value}" (string, {len(self.value)} chars)'
        return StringValue(info, self.position)


class SymbolValue(GlangValue):
    """Runtime symbol value - an immutable named constant like :ok or :error.

    Symbols are lightweight, immutable identifiers that are compared by identity.
    They're perfect for status codes, flags, and pattern matching.
    """

    # Class-level registry to ensure symbol uniqueness (interning)
    _symbol_registry: Dict[str, 'SymbolValue'] = {}

    def __new__(cls, name: str, position: Optional[SourcePosition] = None):
        """Ensure each symbol name maps to a single instance (interning)."""
        # Remove the leading colon if present (from lexer)
        if name.startswith(':'):
            name = name[1:]

        # Check if this symbol already exists
        if name in cls._symbol_registry:
            existing = cls._symbol_registry[name]
            # Update position if provided
            if position:
                existing.position = position
            return existing

        # Create new symbol instance
        instance = super().__new__(cls)
        cls._symbol_registry[name] = instance
        return instance

    def __init__(self, name: str, position: Optional[SourcePosition] = None):
        """Initialize a symbol value."""
        # Only initialize if not already initialized
        if hasattr(self, '_initialized'):
            return

        super().__init__(position)
        # Remove the leading colon if present
        if name.startswith(':'):
            name = name[1:]
        self.name = name
        self._initialized = True

    def to_python(self) -> str:
        """Return the symbol name without the colon."""
        return self.name

    _type_name = "symbol"

    def to_display_string(self) -> str:
        """Display symbols with leading colon."""
        return f":{self.name}"

    def __eq__(self, other) -> bool:
        """Symbols are equal if they have the same name."""
        if not isinstance(other, SymbolValue):
            return False
        return self.name == other.name

    def __hash__(self) -> int:
        """Symbols are hashable by their name."""
        return hash(self.name)

    def __repr__(self) -> str:
        return f"SymbolValue(:{self.name})"

    @classmethod
    def get_or_create(cls, name: str, position: Optional[SourcePosition] = None) -> 'SymbolValue':
        """Get existing symbol or create new one."""
        return cls(name, position)


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

    _type_name = "bool"

    def __init__(self, value: bool, position: Optional[SourcePosition] = None):
        super().__init__(position)
        self.value = value

    def to_python(self) -> bool:
        return self.value

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

    def __init__(self, name: str, parameters: List[str], body: 'Block', closure_context: Optional['ExecutionContext'] = None, module_context: Optional[Dict[str, 'GlangValue']] = None, position: Optional[SourcePosition] = None):
        super().__init__(position)
        self.name = name
        self.parameters = parameters
        self.body = body
        self.closure_context = closure_context  # For closures (later enhancement)
        self.module_context = module_context    # For module function scoping
        
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
    """Runtime value representing absence of value with safe propagation."""

    _type_name = "none"

    def __init__(self, position: Optional[SourcePosition] = None):
        super().__init__(position)

    def to_python(self) -> None:
        return None

    def to_display_string(self) -> str:
        return "none"

    def __eq__(self, other) -> bool:
        return isinstance(other, NoneValue)

    def __hash__(self) -> int:
        return hash("none")

    # Configurable conversion methods
    def to_string(self) -> 'StringValue':
        """Convert none to string based on configuration."""
        try:
            from .configuration_context import get_current_config
            config = get_current_config()
            behavior = config.get_none_conversion('to_string')

            if behavior == 'empty_string':
                return StringValue("")
            elif behavior == 'none_literal':
                return StringValue("none")
            elif behavior == 'error':
                raise RuntimeError("Cannot convert none to string (configure none_conversions.to_string to allow)")
            else:
                # Default to empty string for unknown behaviors
                return StringValue("")
        except ImportError:
            # Fallback if configuration system not available
            return StringValue("")

    def to_number(self) -> 'NumberValue':
        """Convert none to number based on configuration."""
        try:
            from .configuration_context import get_current_config
            config = get_current_config()
            behavior = config.get_none_conversion('to_number')

            if behavior == 'zero':
                return NumberValue(0)
            elif behavior == 'error':
                raise RuntimeError("Cannot convert none to number (configure none_conversions.to_number to allow)")
            else:
                # Default to zero for unknown behaviors
                return NumberValue(0)
        except ImportError:
            # Fallback if configuration system not available
            return NumberValue(0)

    def to_num(self) -> 'NumberValue':
        """Alias for to_number()."""
        return self.to_number()

    def to_bool(self) -> 'BooleanValue':
        """Convert none to boolean based on configuration."""
        try:
            from .configuration_context import get_current_config
            config = get_current_config()
            behavior = config.get_none_conversion('to_bool')

            if behavior == 'false':
                return BooleanValue(False)
            elif behavior == 'true':
                return BooleanValue(True)
            elif behavior == 'error':
                raise RuntimeError("Cannot convert none to boolean (configure none_conversions.to_bool to allow)")
            else:
                # Default to false for unknown behaviors
                return BooleanValue(False)
        except ImportError:
            # Fallback if configuration system not available
            return BooleanValue(False)

    # Arithmetic operations return none
    def negate(self) -> 'NoneValue':
        return NoneValue(self.position)

    def add(self, other: GlangValue) -> 'NoneValue':
        return NoneValue(self.position)

    def subtract(self, other: GlangValue) -> 'NoneValue':
        return NoneValue(self.position)

    def multiply(self, other: GlangValue) -> 'NoneValue':
        return NoneValue(self.position)

    def divide(self, other: GlangValue) -> 'NoneValue':
        return NoneValue(self.position)

    # String operations return none
    def to_upper(self) -> 'NoneValue':
        return NoneValue(self.position)

    def to_lower(self) -> 'NoneValue':
        return NoneValue(self.position)

    def trim(self) -> 'NoneValue':
        return NoneValue(self.position)

    def reverse(self) -> 'NoneValue':
        return NoneValue(self.position)

    # Detection methods
    def is_none(self) -> 'BooleanValue':
        """Check if value is none - returns true."""
        return BooleanValue(True, self.position)

    def is_some(self) -> 'BooleanValue':
        """Check if value has content - returns false."""
        return BooleanValue(False, self.position)


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
    
    _type_name = "time"
    
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


class FileHandleValue(GlangValue):
    """A Glang file handle representing a boundary capability.
    
    File handles are immutable boundary capabilities that provide controlled,
    unidirectional access to external file resources. They are not nodes or edges
    in the program's data graph, but rather portals that allow data to cross
    the boundary between the internal graph and the external filesystem.
    """
    
    def __init__(self, filepath: str, capability_type: str, position: Optional[SourcePosition] = None):
        super().__init__(position)
        # Immutable identity properties
        self.filepath = filepath
        self.capability_type = capability_type  # "read", "write", or "append"
        self.capability_id = id(self)  # Unique identity for this capability
        
        # Internal state (managed by boundary operations)
        self._python_handle = None  # Lazy initialization
        self._is_active = False
        self._buffer = ""  # Internal buffer for read operations
        self._position = 0  # Logical position in file
        self._is_killed = False  # Permanent destruction flag
        self._eof_reached = False  # For read capabilities: true when EOF reached and auto-closed
    
    def get_type(self) -> str:
        return "file"
    
    def get_capability_type(self) -> str:
        """Return the specific capability type (read/write/append)."""
        return self.capability_type
    
    def is_read_capability(self) -> bool:
        """Check if this is a read capability."""
        return self.capability_type == "read"
    
    def is_write_capability(self) -> bool:
        """Check if this is a write capability."""
        return self.capability_type in ["write", "append"]
    
    def to_python(self) -> dict:
        """Return a representation of the capability (not the Python file handle)."""
        return {
            "filepath": self.filepath,
            "capability_type": self.capability_type,
            "capability_id": self.capability_id,
            "is_active": self._is_active,
            "position": self._position
        }
    
    def to_display_string(self) -> str:
        if self._is_killed:
            status = "killed"
        elif self._eof_reached:
            status = "exhausted (EOF)"
        elif self._is_active:
            status = "active"
        else:
            status = "inactive"
        return f"<{self.capability_type}-capability '{self.filepath}' {status}>"
    
    def universal_size(self) -> 'NumberValue':
        """For a file capability, size represents logical position in the stream."""
        return NumberValue(self._position, self.position)
    
    def universal_inspect(self) -> 'StringValue':
        """Return detailed inspection information about this boundary capability."""
        info = f"File boundary capability:\n"
        info += f"  Path: {self.filepath}\n"
        info += f"  Type: {self.capability_type}\n"
        if self._is_killed:
            info += f"  Status: killed (permanently destroyed)\n"
        else:
            info += f"  Status: {'active' if self._is_active else 'inactive'}\n"
            info += f"  Position: {self._position}\n"
        info += f"  Capability ID: {self.capability_id}"
        return StringValue(info, self.position)
    
    def _ensure_active(self):
        """Lazy initialization of the actual file handle."""
        if self._is_killed:
            raise RuntimeError(f"Cannot activate killed {self.capability_type} capability for {self.filepath}")
        
        # Read capabilities that reached EOF cannot be reactivated
        if self.capability_type == "read" and self._eof_reached:
            raise RuntimeError(f"Cannot reactivate {self.capability_type} capability for {self.filepath}: EOF reached, capability exhausted")
        
        if not self._is_active:
            try:
                if self.capability_type == "read":
                    self._python_handle = open(self.filepath, 'r', encoding='utf-8')
                elif self.capability_type == "write":
                    # Reactivation resets to beginning of file
                    self._python_handle = open(self.filepath, 'w', encoding='utf-8')
                    self._position = 0  # Reset position on reactivation
                elif self.capability_type == "append":
                    self._python_handle = open(self.filepath, 'a', encoding='utf-8')
                self._is_active = True
            except Exception as e:
                raise RuntimeError(f"Cannot activate {self.capability_type} capability for {self.filepath}: {str(e)}")
    
    def _ensure_inactive(self):
        """Close the boundary and release resources."""
        if self._is_active and self._python_handle:
            self._python_handle.close()
            self._python_handle = None
            self._is_active = False
            
            # Mark read capabilities as EOF-reached (cannot be reopened)
            if self.capability_type == "read":
                self._eof_reached = True
    
    def _kill_capability(self):
        """Permanently destroy this capability (flush, close, and prevent reactivation).
        
        This is the proper cleanup method that should be called when the capability
        is no longer needed. After killing, the capability cannot be reactivated.
        """
        if self._is_killed:
            return  # Already killed
            
        # Ensure proper cleanup: flush and close if active
        if self._is_active and self._python_handle:
            try:
                if self.is_write_capability():
                    self._python_handle.flush()
                self._python_handle.close()
            except Exception:
                pass  # Ignore errors during cleanup
            finally:
                self._python_handle = None
                self._is_active = False
        
        # Mark as permanently destroyed
        self._is_killed = True


# Graph-based value factory functions
# These provide a migration path from old containers to graph structures

def python_to_glang_value(value: Any, position=None) -> GlangValue:
    """Convert Python value to appropriate Glang value."""
    # Import here to avoid circular imports
    from .graph_values import ListValue, HashValue

    if value is None:
        return NoneValue(position)
    elif isinstance(value, bool):
        return BooleanValue(value, position)
    elif isinstance(value, (int, float)):
        return NumberValue(value, position)
    elif isinstance(value, str):
        return StringValue(value, position)
    elif isinstance(value, list):
        # Convert list elements recursively
        elements = [python_to_glang_value(item, position) for item in value]
        return ListValue(elements, position=position)
    elif isinstance(value, dict):
        # Convert dict to hash
        pairs = [(str(k), python_to_glang_value(v, position)) for k, v in value.items()]
        return HashValue(pairs, position=position)
    else:
        # For unknown types, try to convert to string
        return StringValue(str(value), position)


def create_glang_list(elements: List[GlangValue], constraint: str = None,
                     position=None):
    """Create a list value using graph-based implementation."""
    from .graph_values import ListValue
    return ListValue(elements, constraint, position)


def create_glang_hash(pairs: List[Tuple[str, GlangValue]], constraint: str = None,
                     position=None):
    """Create a hash value using graph-based implementation."""
    from .graph_values import HashValue
    return HashValue(pairs, constraint, position)
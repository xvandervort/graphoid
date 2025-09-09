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

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../../..'))

from glang.ast.nodes import SourcePosition


class GlangValue(ABC):
    """Base class for all glang runtime values."""
    
    def __init__(self, position: Optional[SourcePosition] = None):
        self.position = position
    
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
    
    # Override universal methods for string-specific behavior
    def universal_size(self) -> 'NumberValue':
        """For strings: size is the number of character nodes."""
        return NumberValue(len(self.value), self.position)
    
    def universal_inspect(self) -> 'StringValue':
        """String-specific inspection showing character count."""
        info = f'"{self.value}" (string, {len(self.value)} chars)'
        return StringValue(info, self.position)


class NumberValue(GlangValue):
    """Runtime number value (int or float)."""
    
    def __init__(self, value: Union[int, float], position: Optional[SourcePosition] = None):
        super().__init__(position)
        self.value = value
    
    def to_python(self) -> Union[int, float]:
        return self.value
    
    def get_type(self) -> str:
        return "num"
    
    def to_display_string(self) -> str:
        return str(self.value)
    
    def __eq__(self, other) -> bool:
        return isinstance(other, NumberValue) and self.value == other.value


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
        if not self.validate_constraint(new_value):
            from .errors import TypeConstraintError
            raise TypeConstraintError(
                f"Cannot assign {new_value.get_type()} to data<{self.constraint}>",
                new_value.position
            )
        self.value = new_value
    
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


class MapValue(GlangValue):
    """Runtime map value - collection of data nodes (key-value pairs)."""
    
    def __init__(self, pairs: List[Tuple[str, GlangValue]], constraint: Optional[str] = None,
                 position: Optional[SourcePosition] = None):
        super().__init__(position)
        # Store as ordered dictionary to maintain insertion order
        self.pairs = dict(pairs)  # Convert to dict for efficient key lookup
        self.constraint = constraint  # Optional type constraint for all values
    
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
        if not self.validate_constraint(value):
            from .errors import TypeConstraintError
            raise TypeConstraintError(
                f"Cannot assign {value.get_type()} to hash<{self.constraint}>",
                value.position
            )
        self.pairs[key] = value
    
    def has_key(self, key: str) -> bool:
        """Check if key exists in map."""
        return key in self.pairs
    
    def keys(self) -> List[str]:
        """Get all keys."""
        return list(self.pairs.keys())
    
    def values(self) -> List[GlangValue]:
        """Get all values."""
        return list(self.pairs.values())
    
    def remove(self, key: str) -> bool:
        """Remove key-value pair. Returns True if key existed."""
        if key in self.pairs:
            del self.pairs[key]
            return True
        return False
    
    def __eq__(self, other) -> bool:
        return (isinstance(other, MapValue) and 
                self.pairs == other.pairs and
                self.constraint == other.constraint)
    
    # Override universal methods for map-specific behavior
    def universal_size(self) -> 'NumberValue':
        """For maps: size is the number of key-value pairs."""
        return NumberValue(len(self.pairs), self.position)
    
    def universal_inspect(self) -> 'StringValue':
        """Map-specific inspection showing constraint and size."""
        constraint_info = f"<{self.constraint}>" if self.constraint else ""
        info = f'hash{constraint_info} ({len(self.pairs)} pairs)'
        return StringValue(info, self.position)


class ListValue(GlangValue):
    """Runtime list value with optional type constraints."""
    
    def __init__(self, elements: List[GlangValue], constraint: Optional[str] = None, 
                 position: Optional[SourcePosition] = None):
        super().__init__(position)
        self.elements = elements
        self.constraint = constraint
    
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
        if not self.validate_constraint(value):
            from .errors import TypeConstraintError
            raise TypeConstraintError(
                f"Cannot append {value.get_type()} to list<{self.constraint}>",
                value.position
            )
        self.elements.append(value)
    
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
        
        if not self.validate_constraint(value):
            from .errors import TypeConstraintError
            raise TypeConstraintError(
                f"Cannot assign {value.get_type()} to list<{self.constraint}>",
                value.position
            )
        
        self.elements[index] = value
    
    def __len__(self) -> int:
        return len(self.elements)
    
    def __eq__(self, other) -> bool:
        return (isinstance(other, ListValue) and 
                self.elements == other.elements and
                self.constraint == other.constraint)
    
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
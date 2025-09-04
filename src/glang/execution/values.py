"""
Glang Runtime Value System

Provides proper runtime value representation for glang, replacing
string-based value handling with typed value objects that support
proper operations and constraint validation.
"""

from abc import ABC, abstractmethod
from typing import Any, List, Optional, Union
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


class StringValue(GlangValue):
    """Runtime string value."""
    
    def __init__(self, value: str, position: Optional[SourcePosition] = None):
        super().__init__(position)
        self.value = value
    
    def to_python(self) -> str:
        return self.value
    
    def get_type(self) -> str:
        return "string"
    
    def to_display_string(self) -> str:
        return self.value
    
    def __eq__(self, other) -> bool:
        return isinstance(other, StringValue) and self.value == other.value


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


def python_to_glang_value(python_value: Any, position: Optional[SourcePosition] = None) -> GlangValue:
    """Convert Python value to appropriate GlangValue."""
    if isinstance(python_value, str):
        return StringValue(python_value, position)
    elif isinstance(python_value, bool):  # Check bool before int/float since bool is subclass of int
        return BooleanValue(python_value, position)
    elif isinstance(python_value, (int, float)):
        return NumberValue(python_value, position)
    elif isinstance(python_value, list):
        elements = [python_to_glang_value(elem, position) for elem in python_value]
        return ListValue(elements, None, position)
    else:
        # Fallback: convert to string
        return StringValue(str(python_value), position)


def glang_value_to_python(glang_value: GlangValue) -> Any:
    """Convert GlangValue to Python equivalent."""
    return glang_value.to_python()
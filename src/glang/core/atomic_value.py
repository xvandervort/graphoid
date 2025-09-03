"""
AtomicValue class for scalar values in Glang.

This separates atomic/scalar values from graphs, preventing scalars
from having inappropriate graph methods like append(), prepend(), etc.
"""

from typing import Any, Union


class AtomicValue:
    """
    Represents an atomic (scalar) value in Glang.
    
    Atomic values are immutable single values like strings, numbers, and booleans.
    Unlike Graph objects, they do not have collection methods like append() or prepend().
    """
    
    def __init__(self, value: Any, atomic_type: str):
        """
        Initialize an atomic value.
        
        Args:
            value: The actual scalar value (str, int, float, bool)
            atomic_type: The type string ('string', 'num', 'bool')
        """
        self._value = value
        self._atomic_type = atomic_type
        self._validate_type()
    
    def _validate_type(self) -> None:
        """Validate that the value matches its declared atomic type."""
        if self._atomic_type == 'string' and not isinstance(self._value, str):
            raise ValueError(f"Value '{self._value}' is not a string")
        elif self._atomic_type == 'num' and not isinstance(self._value, (int, float)):
            raise ValueError(f"Value '{self._value}' is not a number")
        elif self._atomic_type == 'bool' and not isinstance(self._value, bool):
            raise ValueError(f"Value '{self._value}' is not a boolean")
        elif self._atomic_type not in ['string', 'num', 'bool']:
            raise ValueError(f"Unknown atomic type: {self._atomic_type}")
    
    @property
    def value(self) -> Any:
        """Get the atomic value."""
        return self._value
    
    @property
    def atomic_type(self) -> str:
        """Get the atomic type."""
        return self._atomic_type
    
    def to_string(self) -> str:
        """Convert to string representation."""
        if self._atomic_type == 'string':
            return str(self._value)
        elif self._atomic_type == 'num':
            return str(self._value)
        elif self._atomic_type == 'bool':
            return 'true' if self._value else 'false'
        return str(self._value)
    
    def to_num(self) -> Union[int, float]:
        """Convert to numeric representation if possible."""
        if self._atomic_type == 'num':
            return self._value
        elif self._atomic_type == 'string':
            try:
                # Try int first, then float
                if '.' in str(self._value):
                    return float(self._value)
                else:
                    return int(self._value)
            except ValueError:
                raise ValueError(f"Cannot convert string '{self._value}' to number")
        elif self._atomic_type == 'bool':
            return 1 if self._value else 0
        else:
            raise ValueError(f"Cannot convert {self._atomic_type} to number")
    
    def to_bool(self) -> bool:
        """Convert to boolean representation."""
        if self._atomic_type == 'bool':
            return self._value
        elif self._atomic_type == 'num':
            return bool(self._value)
        elif self._atomic_type == 'string':
            return bool(self._value)  # Empty string is False, non-empty is True
        else:
            return bool(self._value)
    
    def __str__(self) -> str:
        """String representation for display."""
        if self._atomic_type == 'string':
            return f"'{self._value}'"
        elif self._atomic_type == 'bool':
            return 'true' if self._value else 'false'
        else:
            return str(self._value)
    
    def __repr__(self) -> str:
        """Detailed representation for debugging."""
        return f"AtomicValue({self._value!r}, {self._atomic_type!r})"
    
    def __eq__(self, other) -> bool:
        """Check equality with another AtomicValue or raw value."""
        if isinstance(other, AtomicValue):
            return self._value == other._value and self._atomic_type == other._atomic_type
        else:
            return self._value == other
    
    def __hash__(self) -> int:
        """Make AtomicValue hashable."""
        return hash((self._value, self._atomic_type))
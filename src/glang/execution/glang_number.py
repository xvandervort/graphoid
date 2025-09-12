"""
Glang Number System

Custom number representation with Glang-specific semantics, independent of Python's
number types. This enables custom precision, overflow behavior, and mathematical
operations that align with Glang's language design.
"""

from abc import ABC, abstractmethod
from typing import Union, Optional
import math
from decimal import Decimal, getcontext


class GlangNumber(ABC):
    """Abstract interface for Glang's custom number representation."""
    
    @abstractmethod
    def add(self, other: 'GlangNumber') -> 'GlangNumber':
        """Add two numbers using Glang arithmetic semantics."""
        pass
    
    @abstractmethod
    def subtract(self, other: 'GlangNumber') -> 'GlangNumber':
        """Subtract two numbers using Glang arithmetic semantics."""
        pass
    
    @abstractmethod
    def multiply(self, other: 'GlangNumber') -> 'GlangNumber':
        """Multiply two numbers using Glang arithmetic semantics."""
        pass
    
    @abstractmethod
    def divide(self, other: 'GlangNumber') -> 'GlangNumber':
        """Divide two numbers using Glang arithmetic semantics."""
        pass
    
    @abstractmethod
    def modulo(self, other: 'GlangNumber') -> 'GlangNumber':
        """Perform modulo operation using Glang arithmetic semantics."""
        pass
    
    @abstractmethod
    def power(self, other: 'GlangNumber') -> 'GlangNumber':
        """Raise this number to the power of other using Glang semantics."""
        pass
    
    @abstractmethod
    def negate(self) -> 'GlangNumber':
        """Return the negative of this number."""
        pass
    
    @abstractmethod
    def absolute(self) -> 'GlangNumber':
        """Return the absolute value of this number."""
        pass
    
    @abstractmethod
    def compare_to(self, other: 'GlangNumber') -> int:
        """Compare this number to another.
        Returns -1 if self < other, 0 if equal, 1 if self > other."""
        pass
    
    @abstractmethod
    def to_python_int(self) -> int:
        """Convert to Python int (may lose precision)."""
        pass
    
    @abstractmethod
    def to_python_float(self) -> float:
        """Convert to Python float (may lose precision)."""
        pass
    
    @abstractmethod
    def to_string(self) -> str:
        """Convert to string representation."""
        pass
    
    @abstractmethod
    def is_zero(self) -> bool:
        """Check if this number is zero."""
        pass
    
    @abstractmethod
    def is_integer(self) -> bool:
        """Check if this number represents an integer value."""
        pass
    
    @abstractmethod
    def copy(self) -> 'GlangNumber':
        """Create a copy of this number."""
        pass


class PrecisionGlangNumber(GlangNumber):
    """
    High-precision implementation of GlangNumber using Python's Decimal.
    
    This implementation provides:
    - Arbitrary precision arithmetic (configurable)
    - Consistent rounding behavior
    - Proper overflow/underflow handling
    - Glang-specific mathematical semantics
    """
    
    # Default precision for Glang numbers (28 decimal places)
    DEFAULT_PRECISION = 28
    
    def __init__(self, value: Union[int, float, str, bool, Decimal]):
        # Use whatever precision is currently set in the context
        # The precision block will manage the context
        
        if isinstance(value, Decimal):
            self._decimal = value
        elif isinstance(value, bool):
            # Handle boolean values explicitly (bool is a subclass of int)
            self._decimal = Decimal(int(value))  # True -> 1, False -> 0
        elif isinstance(value, (int, float)):
            self._decimal = Decimal(str(value))  # Convert via string to avoid float precision issues
        elif isinstance(value, str):
            self._decimal = Decimal(value)
        else:
            raise ValueError(f"Cannot create GlangNumber from {type(value)}")
        
        # Apply current precision context to the created decimal
        # This ensures precision limits are respected
        current_prec = getcontext().prec
        if current_prec != 0 and current_prec < self.DEFAULT_PRECISION:
            # Round to current precision if it's lower than default
            self._decimal = +self._decimal  # Force re-evaluation with current context
    
    def add(self, other: 'GlangNumber') -> 'GlangNumber':
        """Add two numbers using Glang arithmetic semantics."""
        if not isinstance(other, PrecisionGlangNumber):
            raise TypeError(f"Cannot add PrecisionGlangNumber to {type(other)}")
        
        result = self._decimal + other._decimal
        return PrecisionGlangNumber(result)
    
    def subtract(self, other: 'GlangNumber') -> 'GlangNumber':
        """Subtract two numbers using Glang arithmetic semantics."""
        if not isinstance(other, PrecisionGlangNumber):
            raise TypeError(f"Cannot subtract {type(other)} from PrecisionGlangNumber")
        
        result = self._decimal - other._decimal
        return PrecisionGlangNumber(result)
    
    def multiply(self, other: 'GlangNumber') -> 'GlangNumber':
        """Multiply two numbers using Glang arithmetic semantics."""
        if not isinstance(other, PrecisionGlangNumber):
            raise TypeError(f"Cannot multiply PrecisionGlangNumber by {type(other)}")
        
        result = self._decimal * other._decimal
        return PrecisionGlangNumber(result)
    
    def divide(self, other: 'GlangNumber') -> 'GlangNumber':
        """Divide two numbers using Glang arithmetic semantics."""
        if not isinstance(other, PrecisionGlangNumber):
            raise TypeError(f"Cannot divide PrecisionGlangNumber by {type(other)}")
        
        if other.is_zero():
            raise ZeroDivisionError("Division by zero in Glang arithmetic")
        
        result = self._decimal / other._decimal
        return PrecisionGlangNumber(result)
    
    def modulo(self, other: 'GlangNumber') -> 'GlangNumber':
        """Perform modulo operation using Glang arithmetic semantics."""
        if not isinstance(other, PrecisionGlangNumber):
            raise TypeError(f"Cannot perform modulo with PrecisionGlangNumber and {type(other)}")
        
        if other.is_zero():
            raise ZeroDivisionError("Modulo by zero in Glang arithmetic")
        
        result = self._decimal % other._decimal
        return PrecisionGlangNumber(result)
    
    def power(self, other: 'GlangNumber') -> 'GlangNumber':
        """Raise this number to the power of other using Glang semantics."""
        if not isinstance(other, PrecisionGlangNumber):
            raise TypeError(f"Cannot raise PrecisionGlangNumber to power of {type(other)}")
        
        # For very large exponents, we may need to handle overflow
        try:
            result = self._decimal ** other._decimal
            return PrecisionGlangNumber(result)
        except (OverflowError, Decimal.Overflow):
            # Glang overflow behavior: return a special "infinity" representation
            if self._decimal > 0:
                return PrecisionGlangNumber("Infinity")
            else:
                return PrecisionGlangNumber("-Infinity")
    
    def negate(self) -> 'GlangNumber':
        """Return the negative of this number."""
        result = -self._decimal
        return PrecisionGlangNumber(result)
    
    def absolute(self) -> 'GlangNumber':
        """Return the absolute value of this number."""
        result = abs(self._decimal)
        return PrecisionGlangNumber(result)
    
    def compare_to(self, other: 'GlangNumber') -> int:
        """Compare this number to another using Glang semantics."""
        if not isinstance(other, PrecisionGlangNumber):
            raise TypeError(f"Cannot compare PrecisionGlangNumber to {type(other)}")
        
        if self._decimal < other._decimal:
            return -1
        elif self._decimal > other._decimal:
            return 1
        else:
            return 0
    
    def to_python_int(self) -> int:
        """Convert to Python int (may lose precision)."""
        return int(self._decimal)
    
    def to_python_float(self) -> float:
        """Convert to Python float (may lose precision)."""
        return float(self._decimal)
    
    def to_string(self) -> str:
        """Convert to string representation."""
        # Use Glang-specific formatting rules
        decimal_str = str(self._decimal)
        
        # Remove trailing zeros after decimal point for cleaner display
        if '.' in decimal_str:
            decimal_str = decimal_str.rstrip('0').rstrip('.')
        
        return decimal_str
    
    def is_zero(self) -> bool:
        """Check if this number is zero."""
        return self._decimal == 0
    
    def is_integer(self) -> bool:
        """Check if this number represents an integer value."""
        return self._decimal % 1 == 0
    
    def copy(self) -> 'GlangNumber':
        """Create a copy of this number."""
        return PrecisionGlangNumber(self._decimal)
    
    def __eq__(self, other) -> bool:
        """Equality comparison for GlangNumber."""
        return isinstance(other, PrecisionGlangNumber) and self._decimal == other._decimal
    
    def __hash__(self) -> int:
        """Hash for GlangNumber."""
        return hash(self._decimal)
    
    # Mathematical functions that can be built on the core operations
    def sqrt(self) -> 'GlangNumber':
        """Calculate square root using Glang semantics."""
        if self._decimal < 0:
            raise ValueError("Cannot take square root of negative number")
        result = self._decimal.sqrt()
        return PrecisionGlangNumber(result)
    
    def ceil(self) -> 'GlangNumber':
        """Return ceiling using Glang semantics."""
        import decimal
        result = self._decimal.quantize(Decimal('1'), rounding=decimal.ROUND_CEILING)
        return PrecisionGlangNumber(result)
    
    def floor(self) -> 'GlangNumber':
        """Return floor using Glang semantics."""
        import decimal
        result = self._decimal.quantize(Decimal('1'), rounding=decimal.ROUND_FLOOR)
        return PrecisionGlangNumber(result)
    
    def round_to_precision(self, precision: int) -> 'GlangNumber':
        """Round to specified decimal places using Glang rounding rules."""
        import decimal
        if precision < 0:
            raise ValueError("Precision must be non-negative")
        
        # Create quantize pattern
        if precision == 0:
            quantize_pattern = Decimal('1')
        else:
            quantize_pattern = Decimal('0.' + '0' * (precision - 1) + '1')
        
        result = self._decimal.quantize(quantize_pattern, rounding=decimal.ROUND_HALF_UP)
        return PrecisionGlangNumber(result)


def create_glang_number(value: Union[int, float, str, bool]) -> GlangNumber:
    """Factory function to create a GlangNumber from various input types."""
    return PrecisionGlangNumber(value)


def zero() -> GlangNumber:
    """Create a Glang number representing zero."""
    return PrecisionGlangNumber(0)


def one() -> GlangNumber:
    """Create a Glang number representing one."""
    return PrecisionGlangNumber(1)
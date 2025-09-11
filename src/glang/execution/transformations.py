"""Built-in transformations and predicates for functional operations."""

from typing import Callable, Dict, Optional
from .values import GlangValue, NumberValue, StringValue, BooleanValue, ListValue


class TransformationRegistry:
    """Registry of built-in transformations and predicates for map/filter operations."""
    
    def __init__(self):
        # Transformations take a value and return a transformed value
        self.transformations: Dict[str, Callable[[GlangValue], GlangValue]] = {}
        # Predicates take a value and return a boolean
        self.predicates: Dict[str, Callable[[GlangValue], bool]] = {}
        
        self._register_builtin_transformations()
        self._register_builtin_predicates()
    
    def _register_builtin_transformations(self):
        """Register built-in transformation functions."""
        
        # Numeric transformations
        def double(value: GlangValue) -> GlangValue:
            if isinstance(value, NumberValue):
                two = NumberValue(2, value.position)
                return value.multiply(two)
            raise ValueError(f"Cannot double {value.get_type()}")
        
        def square(value: GlangValue) -> GlangValue:
            if isinstance(value, NumberValue):
                two = NumberValue(2, value.position)
                return value.power(two)
            raise ValueError(f"Cannot square {value.get_type()}")
        
        def negate(value: GlangValue) -> GlangValue:
            if isinstance(value, NumberValue):
                return value.negate()
            raise ValueError(f"Cannot negate {value.get_type()}")
        
        def increment(value: GlangValue) -> GlangValue:
            if isinstance(value, NumberValue):
                one = NumberValue(1, value.position)
                return value.add(one)
            raise ValueError(f"Cannot increment {value.get_type()}")
        
        def decrement(value: GlangValue) -> GlangValue:
            if isinstance(value, NumberValue):
                one = NumberValue(1, value.position)
                return value.subtract(one)
            raise ValueError(f"Cannot decrement {value.get_type()}")
        
        # String transformations
        def upper(value: GlangValue) -> GlangValue:
            if isinstance(value, StringValue):
                return value.to_upper()
            raise ValueError(f"Cannot uppercase {value.get_type()}")
        
        def lower(value: GlangValue) -> GlangValue:
            if isinstance(value, StringValue):
                return value.to_lower()
            raise ValueError(f"Cannot lowercase {value.get_type()}")
        
        def trim(value: GlangValue) -> GlangValue:
            if isinstance(value, StringValue):
                return value.trim()
            raise ValueError(f"Cannot trim {value.get_type()}")
        
        def reverse(value: GlangValue) -> GlangValue:
            if isinstance(value, StringValue):
                return value.reverse()
            raise ValueError(f"Cannot reverse {value.get_type()}")
        
        # Type conversions
        def to_string(value: GlangValue) -> GlangValue:
            return StringValue(value.to_display_string(), value.position)
        
        def to_num(value: GlangValue) -> GlangValue:
            if isinstance(value, StringValue):
                try:
                    # Try int first, then float
                    if '.' in value.value:
                        return NumberValue(float(value.value), value.position)
                    else:
                        return NumberValue(int(value.value), value.position)
                except ValueError:
                    raise ValueError(f"Cannot convert '{value.value}' to number")
            elif isinstance(value, BooleanValue):
                return NumberValue(1 if value.value else 0, value.position)
            elif isinstance(value, NumberValue):
                return value  # Already a number
            raise ValueError(f"Cannot convert {value.get_type()} to number")
        
        def to_bool(value: GlangValue) -> GlangValue:
            if isinstance(value, NumberValue):
                return BooleanValue(value.value != 0, value.position)
            elif isinstance(value, StringValue):
                return BooleanValue(len(value.value) > 0, value.position)
            elif isinstance(value, BooleanValue):
                return value  # Already a boolean
            elif isinstance(value, ListValue):
                return BooleanValue(len(value.elements) > 0, value.position)
            raise ValueError(f"Cannot convert {value.get_type()} to boolean")
        
        # Register all transformations
        self.transformations = {
            # Numeric
            'double': double,
            'square': square,
            'negate': negate,
            'increment': increment,
            'decrement': decrement,
            'inc': increment,  # Alias
            'dec': decrement,  # Alias
            
            # String
            'upper': upper,
            'lower': lower,
            'trim': trim,
            'reverse': reverse,
            'up': upper,    # Alias
            'down': lower,  # Alias
            
            # Type conversions
            'to_string': to_string,
            'to_num': to_num,
            'to_bool': to_bool,
            'str': to_string,  # Alias
            'num': to_num,     # Alias
            'bool': to_bool,   # Alias
        }
    
    def _register_builtin_predicates(self):
        """Register built-in predicate functions."""
        
        # Numeric predicates
        def positive(value: GlangValue) -> bool:
            if isinstance(value, NumberValue):
                return value.value > 0
            return False
        
        def negative(value: GlangValue) -> bool:
            if isinstance(value, NumberValue):
                return value.value < 0
            return False
        
        def zero(value: GlangValue) -> bool:
            if isinstance(value, NumberValue):
                return value.value == 0
            return False
        
        def even(value: GlangValue) -> bool:
            if isinstance(value, NumberValue):
                return int(value.value) % 2 == 0
            return False
        
        def odd(value: GlangValue) -> bool:
            if isinstance(value, NumberValue):
                return int(value.value) % 2 != 0
            return False
        
        # String predicates
        def empty(value: GlangValue) -> bool:
            if isinstance(value, StringValue):
                return len(value.value) == 0
            elif isinstance(value, ListValue):
                return len(value.elements) == 0
            return False
        
        def non_empty(value: GlangValue) -> bool:
            return not empty(value)
        
        def uppercase(value: GlangValue) -> bool:
            if isinstance(value, StringValue):
                return value.value.isupper()
            return False
        
        def lowercase(value: GlangValue) -> bool:
            if isinstance(value, StringValue):
                return value.value.islower()
            return False
        
        def alphabetic(value: GlangValue) -> bool:
            if isinstance(value, StringValue):
                return value.value.isalpha()
            return False
        
        def numeric(value: GlangValue) -> bool:
            if isinstance(value, StringValue):
                return value.value.isdigit()
            return False
        
        # Type predicates
        def is_string(value: GlangValue) -> bool:
            return isinstance(value, StringValue)
        
        def is_number(value: GlangValue) -> bool:
            return isinstance(value, NumberValue)
        
        def is_bool(value: GlangValue) -> bool:
            return isinstance(value, BooleanValue)
        
        def is_list(value: GlangValue) -> bool:
            return isinstance(value, ListValue)
        
        # General predicates
        def truthy(value: GlangValue) -> bool:
            """Check if value is truthy according to Glang rules."""
            if isinstance(value, BooleanValue):
                return value.value
            elif isinstance(value, NumberValue):
                return value.value != 0
            elif isinstance(value, StringValue):
                return len(value.value) > 0
            elif isinstance(value, ListValue):
                return len(value.elements) > 0
            return False
        
        def falsy(value: GlangValue) -> bool:
            """Check if value is falsy according to Glang rules."""
            return not truthy(value)
        
        # Register all predicates
        self.predicates = {
            # Numeric
            'positive': positive,
            'negative': negative,
            'zero': zero,
            'even': even,
            'odd': odd,
            'pos': positive,  # Alias
            'neg': negative,  # Alias
            
            # String/Collection
            'empty': empty,
            'non_empty': non_empty,
            'uppercase': uppercase,
            'lowercase': lowercase,
            'alphabetic': alphabetic,
            'numeric': numeric,
            'alpha': alphabetic,  # Alias
            'digit': numeric,     # Alias
            
            # Type checks
            'is_string': is_string,
            'is_number': is_number,
            'is_bool': is_bool,
            'is_list': is_list,
            'string': is_string,  # Alias
            'number': is_number,  # Alias
            'boolean': is_bool,   # Alias
            'list': is_list,      # Alias
            
            # General
            'truthy': truthy,
            'falsy': falsy,
        }
    
    def get_transformation(self, name: str) -> Optional[Callable[[GlangValue], GlangValue]]:
        """Get a transformation function by name."""
        return self.transformations.get(name)
    
    def get_predicate(self, name: str) -> Optional[Callable[[GlangValue], bool]]:
        """Get a predicate function by name."""
        return self.predicates.get(name)
    
    def has_transformation(self, name: str) -> bool:
        """Check if a transformation exists."""
        return name in self.transformations
    
    def has_predicate(self, name: str) -> bool:
        """Check if a predicate exists."""
        return name in self.predicates


# Global registry instance
transformation_registry = TransformationRegistry()
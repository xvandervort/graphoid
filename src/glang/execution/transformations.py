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

        # Polymorphic transformations using try-catch instead of isinstance
        def double(value: GlangValue) -> GlangValue:
            try:
                two = NumberValue(2, value.position)
                return value.multiply(two)
            except AttributeError:
                raise ValueError(f"Cannot double {value.get_type()}")

        def square(value: GlangValue) -> GlangValue:
            try:
                two = NumberValue(2, value.position)
                return value.power(two)
            except AttributeError:
                raise ValueError(f"Cannot square {value.get_type()}")

        def negate(value: GlangValue) -> GlangValue:
            try:
                return value.negate()
            except AttributeError:
                raise ValueError(f"Cannot negate {value.get_type()}")

        def increment(value: GlangValue) -> GlangValue:
            try:
                one = NumberValue(1, value.position)
                return value.add(one)
            except AttributeError:
                raise ValueError(f"Cannot increment {value.get_type()}")

        def decrement(value: GlangValue) -> GlangValue:
            try:
                one = NumberValue(1, value.position)
                return value.subtract(one)
            except AttributeError:
                raise ValueError(f"Cannot decrement {value.get_type()}")

        # String transformations using polymorphic dispatch
        def upper(value: GlangValue) -> GlangValue:
            try:
                return value.to_upper()
            except AttributeError:
                raise ValueError(f"Cannot uppercase {value.get_type()}")

        def lower(value: GlangValue) -> GlangValue:
            try:
                return value.to_lower()
            except AttributeError:
                raise ValueError(f"Cannot lowercase {value.get_type()}")

        def trim(value: GlangValue) -> GlangValue:
            try:
                return value.trim()
            except AttributeError:
                raise ValueError(f"Cannot trim {value.get_type()}")

        def reverse(value: GlangValue) -> GlangValue:
            try:
                return value.reverse()
            except AttributeError:
                raise ValueError(f"Cannot reverse {value.get_type()}")
        
        # Type conversions
        def to_string(value: GlangValue) -> GlangValue:
            return StringValue(value.to_display_string(), value.position)
        
        def to_num(value: GlangValue) -> GlangValue:
            # Try polymorphic to_num() method first
            try:
                return value.to_num()
            except AttributeError:
                pass

            # Fallback to type-specific logic
            type_name = value.get_type()
            if type_name == "string":
                try:
                    str_val = value.to_python()
                    # Try int first, then float
                    if '.' in str_val:
                        return NumberValue(float(str_val), value.position)
                    else:
                        return NumberValue(int(str_val), value.position)
                except ValueError:
                    raise ValueError(f"Cannot convert '{str_val}' to number")
            elif type_name == "bool":
                return NumberValue(1 if value.to_python() else 0, value.position)
            elif type_name == "num":
                return value  # Already a number
            raise ValueError(f"Cannot convert {type_name} to number")

        def to_bool(value: GlangValue) -> GlangValue:
            # Try polymorphic to_bool() method first
            try:
                return value.to_bool()
            except AttributeError:
                pass

            # Fallback to type-specific logic
            type_name = value.get_type()
            if type_name == "num":
                return BooleanValue(value.to_python() != 0, value.position)
            elif type_name == "string":
                return BooleanValue(len(value.to_python()) > 0, value.position)
            elif type_name == "bool":
                return value  # Already a boolean
            elif type_name == "list":
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
        
        # Numeric predicates using polymorphic dispatch
        def positive(value: GlangValue) -> bool:
            if value.get_type() == "num":
                return value.to_python() > 0
            return False

        def negative(value: GlangValue) -> bool:
            if value.get_type() == "num":
                return value.to_python() < 0
            return False

        def zero(value: GlangValue) -> bool:
            if value.get_type() == "num":
                return value.to_python() == 0
            return False
        
        def even(value: GlangValue) -> bool:
            if value.get_type() == "num":
                return int(value.to_python()) % 2 == 0
            return False

        def odd(value: GlangValue) -> bool:
            if value.get_type() == "num":
                return int(value.to_python()) % 2 != 0
            return False
        
        # String and collection predicates using polymorphic dispatch
        def empty(value: GlangValue) -> bool:
            type_name = value.get_type()
            if type_name == "string":
                return len(value.to_python()) == 0
            elif type_name == "list":
                return len(value.elements) == 0
            return False

        def non_empty(value: GlangValue) -> bool:
            return not empty(value)

        def uppercase(value: GlangValue) -> bool:
            if value.get_type() == "string":
                return value.to_python().isupper()
            return False

        def lowercase(value: GlangValue) -> bool:
            if value.get_type() == "string":
                return value.to_python().islower()
            return False

        def alphabetic(value: GlangValue) -> bool:
            if value.get_type() == "string":
                return value.to_python().isalpha()
            return False

        def numeric(value: GlangValue) -> bool:
            if value.get_type() == "string":
                return value.to_python().isdigit()
            return False

        # Type predicates using get_type() instead of isinstance
        def is_string(value: GlangValue) -> bool:
            return value.get_type() == "string"

        def is_number(value: GlangValue) -> bool:
            return value.get_type() == "num"

        def is_bool(value: GlangValue) -> bool:
            return value.get_type() == "bool"

        def is_list(value: GlangValue) -> bool:
            return value.get_type() == "list"
        
        # General predicates
        def truthy(value: GlangValue) -> bool:
            """Check if value is truthy according to Glang rules."""
            type_name = value.get_type()
            if type_name == "bool":
                return value.to_python()
            elif type_name == "num":
                return value.to_python() != 0
            elif type_name == "string":
                return len(value.to_python()) > 0
            elif type_name == "list":
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
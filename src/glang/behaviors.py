"""Behavior system for Glang containers.

Behaviors are composable transformations and validations that can be
attached to lists, hashes, and eventually graph nodes.
"""

from typing import Any, Dict, List, Optional, Callable, Union
from .execution.values import GlangValue, NumberValue, StringValue, BooleanValue, NoneValue
from .execution.graph_values import ListValue, HashValue


class Behavior:
    """A composable behavior that can transform or validate values."""
    
    def __init__(self, name: str, 
                 transform: Optional[Callable] = None,
                 validate: Optional[Callable] = None,
                 on_invalid: Optional[Callable] = None):
        self.name = name
        self.transform = transform
        self.validate = validate
        self.on_invalid = on_invalid
    
    def apply(self, value: GlangValue, *args) -> GlangValue:
        """Apply this behavior to a value."""
        # First validate if we have a validator
        if self.validate:
            is_valid = self.validate(value, *args)
            if not is_valid:
                if self.on_invalid:
                    return self.on_invalid(value, *args)
                else:
                    raise ValueError(f"Value {value.to_display_string()} failed validation for {self.name}")
        
        # Then transform if we have a transformer
        if self.transform:
            return self.transform(value, *args)
        
        return value


class BehaviorRegistry:
    """Registry of standard behaviors."""
    
    def __init__(self):
        self.behaviors: Dict[str, Behavior] = {}
        self._register_standard_behaviors()
    
    def _register_standard_behaviors(self):
        """Register the standard library of behaviors."""
        
        # NilToZero - converts nil to 0
        self.behaviors["nil_to_zero"] = Behavior(
            "nil_to_zero",
            transform=lambda value: NumberValue(0) if isinstance(value, NoneValue) else value
        )
        
        # NilToEmpty - converts nil to empty string
        self.behaviors["nil_to_empty"] = Behavior(
            "nil_to_empty", 
            transform=lambda value: StringValue("") if isinstance(value, NoneValue) else value
        )
        
        # ValidateRange - clamps numbers to a range
        def validate_range(value: GlangValue, min_val: float, max_val: float) -> bool:
            if not isinstance(value, NumberValue):
                return True  # Skip non-numbers
            return min_val <= value.value <= max_val
        
        def clamp_range(value: GlangValue, min_val: float, max_val: float) -> GlangValue:
            if not isinstance(value, NumberValue):
                return value
            if value.value < min_val:
                return NumberValue(min_val)
            if value.value > max_val:
                return NumberValue(max_val)
            return value
        
        self.behaviors["validate_range"] = Behavior(
            "validate_range",
            validate=validate_range,
            on_invalid=clamp_range
        )
        
        # MapColors - maps color names to numbers
        def map_colors(value: GlangValue) -> GlangValue:
            if not isinstance(value, StringValue):
                return value
            color_map = {
                "red": 1, "green": 2, "blue": 3, "yellow": 4,
                "black": 0, "white": 5
            }
            color_str = value.value.lower()
            if color_str in color_map:
                return NumberValue(color_map[color_str])
            return value
        
        self.behaviors["map_colors"] = Behavior(
            "map_colors",
            transform=map_colors
        )
        
        # Uppercase - converts strings to uppercase
        self.behaviors["uppercase"] = Behavior(
            "uppercase",
            transform=lambda value: StringValue(value.value.upper()) if isinstance(value, StringValue) else value
        )
        
        # Lowercase - converts strings to lowercase  
        self.behaviors["lowercase"] = Behavior(
            "lowercase",
            transform=lambda value: StringValue(value.value.lower()) if isinstance(value, StringValue) else value
        )
        
        # RoundToInt - rounds numbers to integers
        self.behaviors["round_to_int"] = Behavior(
            "round_to_int",
            transform=lambda value: NumberValue(round(value.value)) if isinstance(value, NumberValue) else value
        )
        
        # Positive - ensures numbers are positive
        def ensure_positive(value: GlangValue) -> GlangValue:
            if isinstance(value, NumberValue) and value.value < 0:
                return NumberValue(abs(value.value))
            return value
        
        self.behaviors["positive"] = Behavior(
            "positive",
            transform=ensure_positive
        )
    
    def get(self, name: str) -> Optional[Behavior]:
        """Get a behavior by name."""
        return self.behaviors.get(name)
    
    def register(self, name: str, behavior: Behavior):
        """Register a custom behavior."""
        self.behaviors[name] = behavior


# Global registry instance
registry = BehaviorRegistry()


class BehaviorPipeline:
    """A pipeline of behaviors to apply in sequence."""
    
    def __init__(self, behavior_registry: Optional[BehaviorRegistry] = None):
        self.behaviors: List[tuple[Behavior, tuple]] = []
        self.registry = behavior_registry or registry  # Use provided or global registry
    
    def add(self, behavior: Union[str, Behavior], *args):
        """Add a behavior to the pipeline."""
        if isinstance(behavior, str):
            behavior_obj = self.registry.get(behavior)
            if not behavior_obj:
                raise ValueError(f"Unknown behavior: {behavior}")
            behavior = behavior_obj
        self.behaviors.append((behavior, args))
    
    def apply(self, value: GlangValue) -> GlangValue:
        """Apply all behaviors in sequence."""
        result = value
        for behavior, args in self.behaviors:
            result = behavior.apply(result, *args)
        return result
    
    def apply_to_list(self, lst: ListValue) -> ListValue:
        """Apply behaviors to all elements in a list."""
        new_elements = []
        for elem in lst.elements:
            new_elements.append(self.apply(elem))
        return ListValue(new_elements, lst.constraint, lst.position)
    
    def apply_to_hash_value(self, hash_val: HashValue, key: str) -> GlangValue:
        """Apply behaviors to a specific hash value."""
        current_value = hash_val.get(key)
        if current_value is not None:
            # Apply behaviors to the value
            new_value = self.apply(current_value)
            # Update the hash with the new value
            hash_val[key] = new_value
            return new_value
        return NoneValue()


def create_behavior(name: str, transform: Optional[Callable] = None,
                   validate: Optional[Callable] = None,
                   on_invalid: Optional[Callable] = None) -> Behavior:
    """Helper function to create a behavior."""
    return Behavior(name, transform, validate, on_invalid)
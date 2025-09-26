"""Behavior system for Glang containers.

Behaviors are composable transformations and validations that can be
attached to lists, hashes, and eventually graph nodes.
"""

from typing import Any, Dict, List, Optional, Callable, Union
from .execution.values import GlangValue, NumberValue, StringValue, BooleanValue, NoneValue
from .execution.graph_values import ListValue, HashValue


class ForwardFillMarker(GlangValue):
    """Marker for values that need forward fill processing."""

    def __init__(self):
        super().__init__()

    def get_type(self) -> str:
        return "forward_fill_marker"

    def to_display_string(self) -> str:
        return "<forward_fill>"

    def to_python(self) -> str:
        return "<forward_fill>"


class BackwardFillMarker(GlangValue):
    """Marker for values that need backward fill processing."""

    def __init__(self):
        super().__init__()

    def get_type(self) -> str:
        return "backward_fill_marker"

    def to_display_string(self) -> str:
        return "<backward_fill>"

    def to_python(self) -> str:
        return "<backward_fill>"


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


class MappingBehavior(Behavior):
    """A behavior that maps values according to a user-defined mapping graph (HashValue)."""

    def __init__(self, mapping: 'HashValue', default: Optional[GlangValue] = None):
        """Initialize mapping behavior with a HashValue graph.

        Args:
            mapping: HashValue graph containing the mapping rules
            default: Optional default value for unmapped keys
        """
        self.mapping = mapping
        self.default = default
        super().__init__("mapping", transform=self._transform)

    def _transform(self, value: GlangValue) -> GlangValue:
        """Transform value according to mapping graph."""
        # Convert value to string key for hash lookup
        key_str = value.to_display_string()

        # Try to get mapped value from the hash graph
        try:
            mapped_value = self.mapping.get(key_str)
            if mapped_value is not None and not isinstance(mapped_value, NoneValue):
                return mapped_value
        except:
            pass

        # Use default if provided, otherwise return original value
        if self.default is not None:
            return self.default

        return value


class BehaviorRegistry:
    """Registry of standard behaviors."""
    
    def __init__(self):
        self.behaviors: Dict[str, Behavior] = {}
        self._register_standard_behaviors()
    
    def _register_standard_behaviors(self):
        """Register the standard library of behaviors."""
        
        # NoneToZero - converts none to 0
        none_to_zero_behavior = Behavior(
            "none_to_zero",
            transform=lambda value: NumberValue(0) if isinstance(value, NoneValue) else value
        )
        self.behaviors["none_to_zero"] = none_to_zero_behavior

        # NoneToEmpty - converts none to empty string
        none_to_empty_behavior = Behavior(
            "none_to_empty",
            transform=lambda value: StringValue("") if isinstance(value, NoneValue) else value
        )
        self.behaviors["none_to_empty"] = none_to_empty_behavior
        
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

        # Forward Fill - replace None values with the last non-None value
        # Note: This is a contextual behavior that needs full list processing
        self.behaviors["forward_fill"] = Behavior(
            "forward_fill",
            transform=self._create_forward_fill_transform()
        )

        # Backward Fill - replace None values with the next non-None value
        # Note: This is a contextual behavior that needs full list processing
        self.behaviors["backward_fill"] = Behavior(
            "backward_fill",
            transform=self._create_backward_fill_transform()
        )
    
    def get(self, name: str) -> Optional[Behavior]:
        """Get a behavior by name."""
        return self.behaviors.get(name)
    
    def register(self, name: str, behavior: Behavior):
        """Register a custom behavior."""
        self.behaviors[name] = behavior

    def _create_forward_fill_transform(self):
        """Create a transform function for forward fill.

        This is a contextual behavior that should be applied to entire lists,
        not individual elements. It marks elements that need processing.
        """
        def forward_fill_transform(value: GlangValue) -> GlangValue:
            # For individual element processing, we just return a marker
            # The actual fill logic happens in the container's _apply_behaviors_to_existing
            if isinstance(value, NoneValue):
                # Mark this as needing forward fill
                return ForwardFillMarker()
            return value

        return forward_fill_transform

    def _create_backward_fill_transform(self):
        """Create a transform function for backward fill."""
        def backward_fill_transform(value: GlangValue) -> GlangValue:
            # For individual element processing, we just return a marker
            if isinstance(value, NoneValue):
                # Mark this as needing backward fill
                return BackwardFillMarker()
            return value

        return backward_fill_transform

    @staticmethod
    def process_contextual_fills(elements: List[GlangValue]) -> List[GlangValue]:
        """Process forward and backward fill markers in a list context."""
        result = list(elements)  # Make a copy

        # Forward fill pass
        last_valid_value = None
        for i, elem in enumerate(result):
            if isinstance(elem, ForwardFillMarker):
                if last_valid_value is not None:
                    result[i] = last_valid_value
                else:
                    # No previous value, leave as None or convert to NoneValue
                    result[i] = NoneValue()
            elif not isinstance(elem, (ForwardFillMarker, BackwardFillMarker)):
                last_valid_value = elem

        # Backward fill pass
        next_valid_value = None
        for i in range(len(result) - 1, -1, -1):
            elem = result[i]
            if isinstance(elem, BackwardFillMarker):
                if next_valid_value is not None:
                    result[i] = next_valid_value
                else:
                    # No next value, leave as None
                    result[i] = NoneValue()
            elif not isinstance(elem, (ForwardFillMarker, BackwardFillMarker)):
                next_valid_value = elem

        return result


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
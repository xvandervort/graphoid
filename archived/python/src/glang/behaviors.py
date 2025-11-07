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


class CustomFunctionBehavior(Behavior):
    """A behavior that applies a user-defined function to values."""

    def __init__(self, function_value: 'GlangValue'):
        """Initialize custom function behavior.

        Args:
            function_value: Either a FunctionValue or LambdaValue that takes one parameter
        """
        from .execution.values import FunctionValue, LambdaValue

        if not isinstance(function_value, (FunctionValue, LambdaValue)):
            raise ValueError(f"Custom behavior must be a function or lambda, got {function_value.get_type()}")

        self.function = function_value
        # Generate unique name based on function
        if hasattr(function_value, 'name'):
            function_name = function_value.name
        else:
            function_name = f"lambda_{id(function_value)}"

        super().__init__(f"custom_{function_name}", transform=self._transform)

    def _transform(self, value: GlangValue) -> GlangValue:
        """Transform value using the custom function."""
        from .execution.executor import ASTExecutor, ExecutionContext
        from .semantic.symbol_table import SymbolTable

        # Create execution context for function call
        symbol_table = SymbolTable()
        context = ExecutionContext(symbol_table)
        executor = ASTExecutor(context)

        try:
            # Call the function with the value as argument
            result = executor.call_function(self.function, [value])
            return result
        except Exception as e:
            # If function fails, return original value
            # In production, might want to log this error
            return value


class ConditionalBehavior(Behavior):
    """A behavior that applies transformations only when conditions are met."""

    def __init__(self, condition_func: 'GlangValue', transform_func: 'GlangValue',
                 on_fail: Optional['GlangValue'] = None):
        """Initialize conditional behavior.

        Args:
            condition_func: Function that takes value and returns boolean
            transform_func: Function to apply when condition is true
            on_fail: Optional function to apply when condition is false
        """
        from .execution.values import FunctionValue, LambdaValue

        # Validate condition function
        if not isinstance(condition_func, (FunctionValue, LambdaValue)):
            raise ValueError(f"Condition must be a function or lambda, got {condition_func.get_type()}")

        # Validate transform function
        if not isinstance(transform_func, (FunctionValue, LambdaValue)):
            raise ValueError(f"Transform must be a function or lambda, got {transform_func.get_type()}")

        # Validate on_fail function if provided
        if on_fail is not None and not isinstance(on_fail, (FunctionValue, LambdaValue)):
            raise ValueError(f"on_fail must be a function or lambda, got {on_fail.get_type()}")

        self.condition_func = condition_func
        self.transform_func = transform_func
        self.on_fail = on_fail

        # Generate unique name
        condition_id = id(condition_func)
        transform_id = id(transform_func)
        super().__init__(f"conditional_{condition_id}_{transform_id}", transform=self._transform)

    def _transform(self, value: GlangValue) -> GlangValue:
        """Apply conditional transformation."""
        from .execution.executor import ASTExecutor, ExecutionContext
        from .semantic.symbol_table import SymbolTable
        from .execution.values import BooleanValue

        # Create execution context for function calls
        symbol_table = SymbolTable()
        context = ExecutionContext(symbol_table)
        executor = ASTExecutor(context)

        try:
            # Evaluate condition
            condition_result = executor.call_function(self.condition_func, [value])

            # Check if condition is true
            is_true = False
            if isinstance(condition_result, BooleanValue):
                is_true = condition_result.value
            elif hasattr(condition_result, 'value'):
                # Handle truthy/falsy values
                if isinstance(condition_result.value, bool):
                    is_true = condition_result.value
                elif isinstance(condition_result.value, (int, float)):
                    is_true = condition_result.value != 0
                elif isinstance(condition_result.value, str):
                    is_true = condition_result.value != ""
                else:
                    is_true = condition_result.value is not None

            if is_true:
                # Apply transform function
                return executor.call_function(self.transform_func, [value])
            else:
                # Apply on_fail function if provided, otherwise return original value
                if self.on_fail is not None:
                    return executor.call_function(self.on_fail, [value])
                else:
                    return value

        except Exception as e:
            # If any function fails, return original value
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


class Ruleset:
    """A collection of behaviors that can be applied as a bundle to containers.

    Provides declarative syntax for creating reusable behavior bundles:

    data_cleaning = Rules[
        :none_to_zero,
        :validate_range[min: 60, max: 200],
        :positive_only,
        custom_sanitizer()
    ]
    """

    def __init__(self, rules: List[str] = None):
        """Initialize ruleset with optional list of rule specifications.

        Args:
            rules: List of rule specifications (strings, symbols, functions)
        """
        self.rules: List[dict] = []
        self.name = f"ruleset_{id(self)}"

        if rules:
            for rule in rules:
                self.add_rule(rule)

    def add_rule(self, rule_spec) -> 'NoneValue':
        """Add a rule to this ruleset.

        Args:
            rule_spec: Rule specification (string, symbol, function, or dict with parameters)
        """
        from .execution.values import StringValue, SymbolValue, FunctionValue, LambdaValue, NoneValue

        rule_info = {}

        if isinstance(rule_spec, StringValue):
            # String rule like "none_to_zero"
            rule_info = {
                'type': 'string',
                'name': rule_spec.value,
                'args': []
            }
        elif isinstance(rule_spec, SymbolValue):
            # Symbol rule like :none_to_zero
            rule_info = {
                'type': 'symbol',
                'name': rule_spec.name,
                'args': []
            }
        elif isinstance(rule_spec, (FunctionValue, LambdaValue)):
            # Custom function rule
            rule_info = {
                'type': 'function',
                'function': rule_spec,
                'args': []
            }
        elif isinstance(rule_spec, dict):
            # Rule with parameters like {'name': 'validate_range', 'args': [60, 200]}
            rule_info = rule_spec
        else:
            raise ValueError(f"Invalid rule specification: {rule_spec}")

        self.rules.append(rule_info)
        return NoneValue()

    def get_rules(self) -> 'ListValue':
        """Get list of all rules in this ruleset."""
        from .execution.values import StringValue
        from .execution.graph_values import ListValue

        rule_names = []
        for rule in self.rules:
            if rule['type'] in ['string', 'symbol']:
                rule_names.append(StringValue(rule['name']))
            elif rule['type'] == 'function':
                func_name = getattr(rule['function'], 'name', 'lambda')
                rule_names.append(StringValue(f"custom_{func_name}"))

        return ListValue(rule_names, "string")

    def apply_to_container(self, container) -> 'NoneValue':
        """Apply all rules in this ruleset to a container.

        Args:
            container: GraphContainer (ListValue, HashValue, etc.) to apply rules to
        """
        from .execution.values import StringValue, SymbolValue, NoneValue

        for rule in self.rules:
            if rule['type'] == 'string':
                # Standard string rule
                container.add_rule(StringValue(rule['name']), *rule.get('args', []))
            elif rule['type'] == 'symbol':
                # Symbol rule
                container.add_rule(SymbolValue(rule['name']), *rule.get('args', []))
            elif rule['type'] == 'function':
                # Custom function rule
                container.add_custom_rule(rule['function'])
            elif rule['type'] == 'mapping':
                # Mapping rule
                container.add_mapping_rule(rule['mapping'], rule.get('default'))
            elif rule['type'] == 'conditional':
                # Conditional rule
                container.add_conditional_rule(
                    rule['condition'],
                    rule['transform'],
                    rule.get('on_fail')
                )

        return NoneValue()

    def size(self) -> int:
        """Get number of rules in this ruleset."""
        return len(self.rules)

    def to_display_string(self) -> str:
        """Get display representation of this ruleset."""
        rule_count = len(self.rules)
        return f"Ruleset[{rule_count} rules]"


class RulesetValue(GlangValue):
    """Glang value wrapper for Ruleset objects."""

    def __init__(self, ruleset: Ruleset):
        super().__init__()
        self.ruleset = ruleset

    def get_type(self) -> str:
        return "ruleset"

    def to_display_string(self) -> str:
        return self.ruleset.to_display_string()

    def to_python(self) -> Ruleset:
        return self.ruleset

    def add_rule(self, rule_spec) -> 'NoneValue':
        """Add a rule to this ruleset."""
        return self.ruleset.add_rule(rule_spec)

    def get_rules(self) -> 'ListValue':
        """Get list of all rules in this ruleset."""
        return self.ruleset.get_rules()

    def size(self) -> 'NumberValue':
        """Get number of rules in this ruleset."""
        return NumberValue(self.ruleset.size())


def create_behavior(name: str, transform: Optional[Callable] = None,
                   validate: Optional[Callable] = None,
                   on_invalid: Optional[Callable] = None) -> Behavior:
    """Helper function to create a behavior."""
    return Behavior(name, transform, validate, on_invalid)


def create_ruleset(rules: List = None) -> RulesetValue:
    """Helper function to create a ruleset.

    Args:
        rules: List of rule specifications

    Returns:
        RulesetValue containing the ruleset
    """
    ruleset = Ruleset(rules)
    return RulesetValue(ruleset)
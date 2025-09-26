"""Graph container base class with intrinsic behavior support.

All Glang collections (lists, hashes, and future true graphs) are graph structures
that can have behaviors attached to automatically transform values during operations.
"""

from typing import List, Set, Optional, Tuple, Any
from abc import abstractmethod


class GraphContainer:
    """Base class for all graph-like containers with intrinsic behaviors.

    This will be mixed into ListValue, HashValue, and future GraphValue classes.
    Behaviors are rules that automatically apply to values during operations.
    """

    def __init__(self):
        # Initialize behavior tracking
        self._behaviors: List[Tuple['Behavior', tuple]] = []
        self._behavior_names: Set[str] = set()

    def add_rule(self, behavior_spec: 'GlangValue', *args) -> 'NoneValue':
        """Add a behavior rule to this container.

        Usage:
            my_list.add_rule("nil_to_zero")
            my_list.add_rule(:nil_to_zero)  # With symbols
            my_list.add_rule("validate_range", 0, 100)
        """
        from .execution.values import StringValue, SymbolValue, NoneValue, GlangValue
        from .behaviors import registry

        # Extract behavior name from string or symbol
        if isinstance(behavior_spec, StringValue):
            behavior_name = behavior_spec.value
        elif isinstance(behavior_spec, SymbolValue):
            behavior_name = behavior_spec.name
        else:
            raise ValueError(f"Behavior must be string or symbol, got {behavior_spec.get_type()}")

        # Prevent duplicate behaviors
        if behavior_name in self._behavior_names:
            return NoneValue()  # Silently ignore duplicates

        # Get behavior from registry
        behavior = registry.get(behavior_name)
        if not behavior:
            raise ValueError(f"Unknown behavior: {behavior_name}")

        # Convert args to GlangValues if needed
        glang_args = []
        for arg in args:
            if isinstance(arg, GlangValue):
                glang_args.append(arg)
            else:
                # Wrap raw Python values
                from .execution.values import NumberValue, StringValue, BooleanValue
                if isinstance(arg, (int, float)):
                    glang_args.append(NumberValue(arg))
                elif isinstance(arg, str):
                    glang_args.append(StringValue(arg))
                elif isinstance(arg, bool):
                    glang_args.append(BooleanValue(arg))
                else:
                    glang_args.append(arg)

        # Add behavior to container
        self._behaviors.append((behavior, tuple(glang_args)))
        self._behavior_names.add(behavior_name)

        # Apply to all existing elements
        self._apply_behaviors_to_existing()

        return NoneValue()

    def remove_rule(self, behavior_spec: 'GlangValue') -> 'BooleanValue':
        """Remove a behavior rule from this container."""
        from .execution.values import StringValue, SymbolValue, BooleanValue

        # Extract behavior name
        if isinstance(behavior_spec, StringValue):
            behavior_name = behavior_spec.value
        elif isinstance(behavior_spec, SymbolValue):
            behavior_name = behavior_spec.name
        else:
            raise ValueError(f"Behavior must be string or symbol, got {behavior_spec.get_type()}")

        if behavior_name not in self._behavior_names:
            return BooleanValue(False)

        # Remove behavior
        self._behaviors = [(b, args) for b, args in self._behaviors if b.name != behavior_name]
        self._behavior_names.remove(behavior_name)

        return BooleanValue(True)

    def has_rule(self, behavior_spec: 'GlangValue') -> 'BooleanValue':
        """Check if this container has a specific behavior rule."""
        from .execution.values import StringValue, SymbolValue, BooleanValue

        # Extract behavior name
        if isinstance(behavior_spec, StringValue):
            behavior_name = behavior_spec.value
        elif isinstance(behavior_spec, SymbolValue):
            behavior_name = behavior_spec.name
        else:
            raise ValueError(f"Behavior must be string or symbol, got {behavior_spec.get_type()}")

        return BooleanValue(behavior_name in self._behavior_names)

    def get_rules(self) -> 'ListValue':
        """Get list of all behavior rules as strings."""
        from .execution.values import StringValue
        from .execution.graph_values import ListValue
        rule_names = [StringValue(name) for name in sorted(self._behavior_names)]
        return ListValue(rule_names, "string")

    def clear_rules(self) -> 'NoneValue':
        """Remove all behavior rules from this container."""
        from .execution.values import NoneValue
        self._behaviors.clear()
        self._behavior_names.clear()
        return NoneValue()

    def add_mapping_rule(self, mapping: 'HashValue', default: Optional['GlangValue'] = None) -> 'NoneValue':
        """Add a generic mapping behavior rule to this container.

        Usage:
            color_map = { "red": 1, "green": 2, "blue": 3 }
            colors.add_mapping_rule(color_map)
            colors.add_mapping_rule(color_map, 0)  # With default value

        Args:
            mapping: HashValue graph containing key->value mappings
            default: Optional default value for unmapped keys
        """
        from .execution.values import NoneValue
        from .behaviors import MappingBehavior

        # Create a mapping behavior with the provided hash graph
        behavior = MappingBehavior(mapping, default)

        # Generate unique name for this mapping behavior
        mapping_id = id(mapping)
        behavior_name = f"mapping_{mapping_id}"

        # Prevent duplicate mappings
        if behavior_name in self._behavior_names:
            return NoneValue()

        # Add behavior to container
        self._behaviors.append((behavior, ()))
        self._behavior_names.add(behavior_name)

        # Apply to all existing elements
        self._apply_behaviors_to_existing()

        return NoneValue()

    def _apply_behaviors(self, value: 'GlangValue') -> 'GlangValue':
        """Apply all behaviors to a single value."""
        result = value
        for behavior, args in self._behaviors:
            # Convert args back to Python for behavior application
            py_args = []
            for arg in args:
                from .execution.values import GlangValue
                if isinstance(arg, GlangValue):
                    py_args.append(arg.to_python())
                else:
                    py_args.append(arg)
            result = behavior.apply(result, *py_args)
        return result

    @abstractmethod
    def _apply_behaviors_to_existing(self):
        """Apply behaviors to all existing elements in the container.

        Must be implemented by subclasses (ListValue, HashValue, etc.)
        """
        pass

    def _has_behaviors(self) -> bool:
        """Check if this container has any behaviors attached."""
        return len(self._behaviors) > 0

    def _get_behavior_string(self) -> str:
        """Get a display string for attached behaviors."""
        if not self._behaviors:
            return ""
        return f" with behaviors: [{', '.join(sorted(self._behavior_names))}]"
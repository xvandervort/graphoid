"""
Graph Migration Utilities

This module provides utilities for migrating from old container-based values
to the new graph-based implementations.
"""

# Global flag to track if graph values are enabled
_graph_values_enabled = True  # Graph values are always enabled in current implementation


def enable_graph_values():
    """Enable graph-based value implementations.

    In the current implementation, graph values are always enabled,
    so this is a no-op for compatibility.
    """
    global _graph_values_enabled
    _graph_values_enabled = True


def disable_graph_values():
    """Disable graph-based value implementations.

    This would revert to old container-based implementations,
    but since we've fully migrated, this is not supported.
    """
    raise NotImplementedError("Graph values cannot be disabled in this implementation")


def validate_graph_migration() -> bool:
    """Validate that graph migration is working correctly.

    Returns:
        bool: True if graph values are properly enabled and working
    """
    global _graph_values_enabled
    if not _graph_values_enabled:
        return False

    # Test that graph values are being created correctly
    try:
        from .values import python_to_glang_value
        from .graph_values import ListValue, HashValue

        # Test list creation
        test_list = python_to_glang_value([1, 2, 3])
        if not isinstance(test_list, ListValue):
            return False
        if not hasattr(test_list, 'graph'):
            return False

        # Test hash creation
        test_dict = python_to_glang_value({"key": "value"})
        if not isinstance(test_dict, HashValue):
            return False
        if not hasattr(test_dict, 'graph'):
            return False

        return True

    except Exception:
        return False


def is_graph_values_enabled() -> bool:
    """Check if graph values are currently enabled.

    Returns:
        bool: True if graph values are enabled
    """
    global _graph_values_enabled
    return _graph_values_enabled
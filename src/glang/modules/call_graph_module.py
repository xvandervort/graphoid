#!/usr/bin/env python3
"""Call Graph Module - Expose call graph functionality to Glang programs.

This module provides Glang programs with access to the internal call graph,
enabling debugging, visualization, and analysis of function connectivity.
"""

from typing import Dict, Any, Optional, List
from glang.execution.values import (
    GlangValue, StringValue, NumberValue, BooleanValue, NoneValue
)
from glang.execution.graph_values import ListValue, HashValue
from glang.modules.module_manager import ModuleNamespace
from glang.modules.module_builder import create_module


class CallGraphInterface:
    """Call graph introspection and visualization interface."""

    def __init__(self):
        """Initialize the call graph interface."""
        self.executor = None  # Set by module initialization

    def setup(self, executor):
        """Store reference to executor for accessing call graph."""
        self.executor = executor

    def visualize(self, format: Optional[StringValue] = None) -> StringValue:
        """Generate visualization of the call graph.

        Args:
            format: Output format ('text', 'dot', 'mermaid'). Defaults to 'text'.

        Returns:
            String representation of the call graph in requested format.
        """
        if not self.executor or not hasattr(self.executor.context, 'call_graph'):
            return StringValue("Call graph not available")

        fmt = format.value if format else "text"
        graph_str = self.executor.context.call_graph.visualize_graph(fmt)
        return StringValue(graph_str)

    def visualize_scope(self, scope: Optional[StringValue] = None) -> StringValue:
        """Visualize functions in a specific scope.

        Args:
            scope: Scope name to visualize. Defaults to current scope.

        Returns:
            String representation of the scope's functions.
        """
        if not self.executor or not hasattr(self.executor.context, 'call_graph'):
            return StringValue("Call graph not available")

        scope_name = scope.value if scope else None
        scope_str = self.executor.context.call_graph.visualize_scope(scope_name)
        return StringValue(scope_str)

    def get_reachable_functions(self, scope: Optional[StringValue] = None) -> ListValue:
        """Get list of functions reachable from given scope.

        Args:
            scope: Scope to search from. Defaults to current scope.

        Returns:
            List of function names reachable via graph traversal.
        """
        if not self.executor or not hasattr(self.executor.context, 'call_graph'):
            return ListValue([])

        scope_name = scope.value if scope else None
        functions = self.executor.context.call_graph.get_reachable_functions(scope_name)
        return ListValue([StringValue(name) for name in functions])

    def find_path(self, from_func: StringValue, to_func: StringValue,
                  scope: Optional[StringValue] = None) -> GlangValue:
        """Find path between two functions using graph traversal.

        Args:
            from_func: Starting function name
            to_func: Target function name
            scope: Current scope for resolving names

        Returns:
            List of function names forming path, or none if no path exists.
        """
        if not self.executor or not hasattr(self.executor.context, 'call_graph'):
            return NoneValue()

        scope_name = scope.value if scope else None
        path = self.executor.context.call_graph.find_path(
            from_func.value, to_func.value, scope_name
        )

        if path:
            return ListValue([StringValue(name) for name in path])
        else:
            return NoneValue()

    def get_function_info(self, name: StringValue, scope: Optional[StringValue] = None) -> GlangValue:
        """Get detailed information about a function in the graph.

        Args:
            name: Function name
            scope: Scope to search. Defaults to current scope.

        Returns:
            Hash with function information or none if not found.
        """
        if not self.executor or not hasattr(self.executor.context, 'call_graph'):
            return NoneValue()

        scope_name = scope.value if scope else None
        info = self.executor.context.call_graph.get_function_info(name.value, scope_name)

        if info:
            # Convert Python dict to Glang hash
            pairs = [
                ("name", StringValue(info["name"])),
                ("qualified_name", StringValue(info["qualified_name"])),
                ("scope", StringValue(info["scope"])),
                ("parameters", ListValue([StringValue(p) for p in info["parameters"]])),
                ("connected_functions", ListValue([StringValue(f) for f in info["connected_functions"]])),
                ("reachable", BooleanValue(info["reachable"]))
            ]
            hash_value = HashValue(pairs)

            return hash_value
        else:
            return NoneValue()

    def count_functions(self, scope: Optional[StringValue] = None) -> NumberValue:
        """Count functions in a specific scope.

        Args:
            scope: Scope to count functions in. Defaults to all scopes.

        Returns:
            Number of functions in the specified scope(s).
        """
        if not self.executor or not hasattr(self.executor.context, 'call_graph'):
            return NumberValue(0)

        if scope:
            scope_name = scope.value
            if scope_name in self.executor.context.call_graph.scope_connections:
                count = len(self.executor.context.call_graph.scope_connections[scope_name])
                return NumberValue(count)
            else:
                return NumberValue(0)
        else:
            # Count all functions
            total = sum(
                len(funcs)
                for funcs in self.executor.context.call_graph.scope_connections.values()
            )
            return NumberValue(total)

    def list_scopes(self) -> ListValue:
        """Get list of all scopes in the call graph.

        Returns:
            List of scope names.
        """
        if not self.executor or not hasattr(self.executor.context, 'call_graph'):
            return ListValue([])

        scopes = sorted(self.executor.context.call_graph.scope_connections.keys())
        return ListValue([StringValue(scope) for scope in scopes])

    def current_scope(self) -> StringValue:
        """Get the current scope name.

        Returns:
            Current scope name.
        """
        if not self.executor or not hasattr(self.executor.context, 'call_graph'):
            return StringValue("unknown")

        return StringValue(self.executor.context.call_graph.current_scope)


# Global instance to maintain state
_call_graph_interface = CallGraphInterface()


def create_call_graph_module_namespace() -> ModuleNamespace:
    """Create and return the call graph module namespace."""
    return create_module(
        "call_graph",
        functions={
            "visualize": _call_graph_interface.visualize,
            "visualize_scope": _call_graph_interface.visualize_scope,
            "get_reachable_functions": _call_graph_interface.get_reachable_functions,
            "find_path": _call_graph_interface.find_path,
            "get_function_info": _call_graph_interface.get_function_info,
            "count_functions": _call_graph_interface.count_functions,
            "list_scopes": _call_graph_interface.list_scopes,
            "current_scope": _call_graph_interface.current_scope,
        }
    )


def setup_call_graph_module(executor):
    """Setup the call graph module with executor reference.

    This should be called when the executor is initialized.
    """
    _call_graph_interface.setup(executor)
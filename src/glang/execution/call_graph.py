#!/usr/bin/env python3
"""Call Graph - True graph-based function discovery system.

This module implements the foundational architecture that transforms Glang
from a simulated graph language to a true graph language by replacing
variable-based function lookup with graph traversal.

CRITICAL: Without this system, Glang is just another scripting language
pretending to be graph-based. This is the core foundation that makes
Glang truly revolutionary.
"""

from typing import Dict, Set, Optional, List, Any
from glang.execution.graph_foundation import GraphStructure, GraphNode, EdgeMetadata, EdgeType
from glang.execution.values import GlangValue, FunctionValue
from glang.ast.nodes import SourcePosition


class CallGraph(GraphStructure):
    """True graph-based function discovery system.

    Replaces variable-based function lookup with graph traversal,
    making Glang truly graph-based at its foundation.
    """

    def __init__(self):
        """Initialize call graph with proper graph structure."""
        super().__init__()
        self.function_nodes: Dict[str, GraphNode] = {}
        self.scope_connections: Dict[str, Set[str]] = {}
        self.global_scope = "global"
        self.current_scope = self.global_scope

    def add_function(self, name: str, func_value: FunctionValue, scope: str = None) -> GraphNode:
        """Add function as graph node with proper connections.

        Args:
            name: Function name
            func_value: Function value object
            scope: Module/scope name (defaults to current scope)

        Returns:
            GraphNode representing the function
        """
        if scope is None:
            scope = self.current_scope

        # Create fully qualified name for the function
        qualified_name = f"{scope}::{name}" if scope != self.global_scope else name

        # Create graph node for the function
        func_node = GraphNode(func_value)
        self.add_node(func_node)
        self.function_nodes[qualified_name] = func_node

        # Initialize scope connections if needed
        if scope not in self.scope_connections:
            self.scope_connections[scope] = set()

        # Add to scope's function set
        self.scope_connections[scope].add(qualified_name)

        # Connect to all other functions in the same scope
        for other_func_name in self.scope_connections[scope]:
            if other_func_name != qualified_name and other_func_name in self.function_nodes:
                other_func_node = self.function_nodes[other_func_name]
                # Bidirectional connections within same module
                edge_metadata = EdgeMetadata(
                    edge_type=EdgeType.NAMED,
                    key="same_module",
                    bidirectional=True
                )
                func_node.add_edge_to(other_func_node, edge_metadata)

        return func_node

    def find_function(self, name: str, current_scope: str = None) -> Optional[FunctionValue]:
        """Graph traversal to find reachable function.

        This is the core method that replaces variable-based lookup
        with true graph traversal.

        Args:
            name: Function name to find
            current_scope: Current module/scope context

        Returns:
            FunctionValue if found via graph traversal, None otherwise
        """
        if current_scope is None:
            current_scope = self.current_scope

        # Search order: current module first, then global scope
        search_scopes = [current_scope]
        if current_scope != self.global_scope:
            search_scopes.append(self.global_scope)

        for scope in search_scopes:
            # Try fully qualified name first
            qualified_name = f"{scope}::{name}" if scope != self.global_scope else name

            if qualified_name in self.function_nodes:
                func_node = self.function_nodes[qualified_name]
                return func_node.value

            # For module scope, also check if any connected functions match
            if scope in self.scope_connections:
                for func_name in self.scope_connections[scope]:
                    # Extract bare name from qualified name
                    bare_name = func_name.split("::")[-1]
                    if bare_name == name:
                        func_node = self.function_nodes[func_name]
                        return func_node.value

        return None

    def connect_module_functions(self, module_name: str, functions: Dict[str, FunctionValue]):
        """Connect all module functions to each other.

        Called when a module is fully loaded to establish proper
        graph connectivity.

        Args:
            module_name: Name of the module
            functions: Dict of function name -> FunctionValue
        """
        # Set current scope for this operation
        old_scope = self.current_scope
        self.current_scope = module_name

        try:
            # Add all functions to the graph
            for func_name, func_value in functions.items():
                self.add_function(func_name, func_value, module_name)

        finally:
            # Restore previous scope
            self.current_scope = old_scope

    def enter_scope(self, scope_name: str):
        """Enter a new scope for function discovery."""
        self.current_scope = scope_name

    def exit_scope(self):
        """Exit current scope, return to global."""
        self.current_scope = self.global_scope

    def get_reachable_functions(self, scope: str = None) -> List[str]:
        """Get all functions reachable from given scope.

        Args:
            scope: Scope to search from (defaults to current)

        Returns:
            List of function names reachable via graph traversal
        """
        if scope is None:
            scope = self.current_scope

        reachable = []

        # Add functions from current scope
        if scope in self.scope_connections:
            for qualified_name in self.scope_connections[scope]:
                bare_name = qualified_name.split("::")[-1]
                reachable.append(bare_name)

        # Add global functions if not in global scope
        if scope != self.global_scope and self.global_scope in self.scope_connections:
            for qualified_name in self.scope_connections[self.global_scope]:
                bare_name = qualified_name.split("::")[-1]
                if bare_name not in reachable:
                    reachable.append(bare_name)

        return sorted(reachable)

    def visualize_scope(self, scope: str = None) -> str:
        """Generate visual representation of call graph for debugging.

        Args:
            scope: Scope to visualize (defaults to current)

        Returns:
            String representation of the call graph
        """
        if scope is None:
            scope = self.current_scope

        result = [f"Call Graph - Scope: {scope}"]
        result.append("=" * 40)

        if scope in self.scope_connections:
            functions = self.scope_connections[scope]
            result.append(f"Functions in {scope}: {len(functions)}")

            for qualified_name in sorted(functions):
                bare_name = qualified_name.split("::")[-1]
                func_node = self.function_nodes[qualified_name]

                result.append(f"  - {bare_name}")

                # Show connections
                neighbors = func_node.get_neighbors()
                if neighbors:
                    connected_names = []
                    for neighbor_node in neighbors:
                        # Find the qualified name for this neighbor node
                        for qname, qnode in self.function_nodes.items():
                            if qnode == neighbor_node:
                                bare_name = qname.split("::")[-1]
                                connected_names.append(bare_name)
                                break
                    result.append(f"    Connected to: {', '.join(sorted(connected_names))}")

        else:
            result.append(f"No functions in scope: {scope}")

        return "\n".join(result)

    def get_function_info(self, name: str, scope: str = None) -> Optional[Dict[str, Any]]:
        """Get detailed information about a function in the graph.

        Args:
            name: Function name
            scope: Scope to search (defaults to current)

        Returns:
            Dict with function information or None if not found
        """
        func_value = self.find_function(name, scope)
        if not func_value:
            return None

        # Find the qualified name
        search_scope = scope or self.current_scope
        qualified_name = f"{search_scope}::{name}" if search_scope != self.global_scope else name

        if qualified_name not in self.function_nodes:
            # Try to find it in any scope
            for scope_name, functions in self.scope_connections.items():
                for func_name in functions:
                    if func_name.split("::")[-1] == name:
                        qualified_name = func_name
                        break

        if qualified_name in self.function_nodes:
            func_node = self.function_nodes[qualified_name]
            neighbors = func_node.get_neighbors()

            # Convert neighbor nodes to their bare names
            connected_names = []
            for neighbor_node in neighbors:
                for qname, qnode in self.function_nodes.items():
                    if qnode == neighbor_node:
                        connected_names.append(qname.split("::")[-1])
                        break

            return {
                "name": name,
                "qualified_name": qualified_name,
                "scope": qualified_name.split("::")[0] if "::" in qualified_name else "global",
                "parameters": func_value.parameters,
                "connected_functions": connected_names,
                "reachable": len(neighbors) > 0
            }

        return None
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

    def visualize_graph(self, format: str = "text") -> str:
        """Generate complete call graph visualization.

        Args:
            format: Output format ('text', 'dot', 'mermaid')

        Returns:
            String representation in requested format
        """
        if format == "dot":
            return self._generate_dot_format()
        elif format == "mermaid":
            return self._generate_mermaid_format()
        else:
            return self._generate_text_format()

    def _generate_text_format(self) -> str:
        """Generate text-based graph visualization."""
        result = ["=" * 50]
        result.append("COMPLETE CALL GRAPH")
        result.append("=" * 50)

        # Group by scope
        for scope in sorted(self.scope_connections.keys()):
            result.append(f"\n[{scope}]")
            for qualified_name in sorted(self.scope_connections[scope]):
                bare_name = qualified_name.split("::")[-1]
                func_node = self.function_nodes[qualified_name]
                result.append(f"  {bare_name}")

                # Show edges
                neighbors = func_node.get_neighbors()
                if neighbors:
                    for neighbor_node in neighbors:
                        for qname, qnode in self.function_nodes.items():
                            if qnode == neighbor_node:
                                target_scope = qname.split("::")[0] if "::" in qname else "global"
                                target_name = qname.split("::")[-1]
                                if target_scope == scope:
                                    result.append(f"    → {target_name}")
                                else:
                                    result.append(f"    → {target_scope}::{target_name}")
                                break

        return "\n".join(result)

    def _generate_dot_format(self) -> str:
        """Generate Graphviz DOT format visualization."""
        dot = ["digraph CallGraph {"]
        dot.append("  rankdir=LR;")
        dot.append("  node [shape=box];")

        # Group by scope using subgraphs
        for scope in sorted(self.scope_connections.keys()):
            dot.append(f"  subgraph cluster_{scope.replace('.', '_')} {{")
            dot.append(f"    label=\"{scope}\";")
            dot.append("    style=filled;")
            dot.append("    color=lightgrey;")

            for qualified_name in sorted(self.scope_connections[scope]):
                bare_name = qualified_name.split("::")[-1]
                node_id = qualified_name.replace("::", "_").replace(".", "_")
                dot.append(f"    {node_id} [label=\"{bare_name}\"];")

            dot.append("  }")

        # Add edges
        for qualified_name, func_node in self.function_nodes.items():
            source_id = qualified_name.replace("::", "_").replace(".", "_")
            neighbors = func_node.get_neighbors()

            for neighbor_node in neighbors:
                for qname, qnode in self.function_nodes.items():
                    if qnode == neighbor_node:
                        target_id = qname.replace("::", "_").replace(".", "_")
                        dot.append(f"  {source_id} -> {target_id};")
                        break

        dot.append("}")
        return "\n".join(dot)

    def _generate_mermaid_format(self) -> str:
        """Generate Mermaid diagram format visualization."""
        mermaid = ["graph LR"]

        # Add nodes grouped by scope
        for scope in sorted(self.scope_connections.keys()):
            for qualified_name in sorted(self.scope_connections[scope]):
                bare_name = qualified_name.split("::")[-1]
                node_id = qualified_name.replace("::", "_").replace(".", "_").replace("-", "_")
                display_name = f"{scope}::{bare_name}" if scope != "global" else bare_name
                mermaid.append(f"    {node_id}[{display_name}]")

        # Add edges
        for qualified_name, func_node in self.function_nodes.items():
            source_id = qualified_name.replace("::", "_").replace(".", "_").replace("-", "_")
            neighbors = func_node.get_neighbors()

            for neighbor_node in neighbors:
                for qname, qnode in self.function_nodes.items():
                    if qnode == neighbor_node:
                        target_id = qname.replace("::", "_").replace(".", "_").replace("-", "_")
                        mermaid.append(f"    {source_id} --> {target_id}")
                        break

        return "\n".join(mermaid)

    def find_path(self, from_function: str, to_function: str, scope: str = None) -> Optional[List[str]]:
        """Find path between two functions using graph traversal.

        Args:
            from_function: Starting function name
            to_function: Target function name
            scope: Current scope for resolving names

        Returns:
            List of function names forming path, or None if no path exists
        """
        if scope is None:
            scope = self.current_scope

        # Resolve qualified names
        from_qualified = None
        to_qualified = None

        # Search for functions in scope
        for s in [scope, self.global_scope]:
            for qualified_name in self.scope_connections.get(s, []):
                bare_name = qualified_name.split("::")[-1]
                if bare_name == from_function and from_qualified is None:
                    from_qualified = qualified_name
                if bare_name == to_function and to_qualified is None:
                    to_qualified = qualified_name

        if not from_qualified or not to_qualified:
            return None

        # BFS to find shortest path
        from collections import deque

        queue = deque([(from_qualified, [from_function])])
        visited = {from_qualified}

        while queue:
            current_qualified, path = queue.popleft()

            if current_qualified == to_qualified:
                return path

            if current_qualified in self.function_nodes:
                func_node = self.function_nodes[current_qualified]

                for neighbor_node in func_node.get_neighbors():
                    # Find qualified name for neighbor
                    for qname, qnode in self.function_nodes.items():
                        if qnode == neighbor_node and qname not in visited:
                            visited.add(qname)
                            bare_name = qname.split("::")[-1]
                            queue.append((qname, path + [bare_name]))
                            break

        return None

    def create_ast_subgraph(self, ast_node, scope: str = "global") -> 'CallGraphSubgraph':
        """Create a temporary subgraph from AST function declarations.

        This is Phase 3 of the call graph architecture: AST as temporary subgraph.
        During parsing, we extract function declarations and create a subgraph
        that can be merged into the permanent call graph during load-time.

        Args:
            ast_node: Root AST node to scan for function declarations
            scope: Scope name for the functions (module name or 'global')

        Returns:
            CallGraphSubgraph containing all functions from the AST
        """
        return CallGraphSubgraph.from_ast(ast_node, scope)

    def merge_subgraph(self, subgraph: 'CallGraphSubgraph'):
        """Merge a temporary subgraph into the permanent call graph.

        This completes the AST integration: functions discovered during parsing
        are now permanently integrated into the call graph for execution.

        Args:
            subgraph: Temporary subgraph to merge
        """
        subgraph.merge_into(self)


class CallGraphSubgraph:
    """Temporary subgraph representing function declarations from AST.

    This implements Phase 3 of the call graph architecture where AST parsing
    creates temporary subgraphs that are merged into the permanent call graph
    during load-time.
    """

    def __init__(self, scope: str):
        """Initialize subgraph for a specific scope."""
        self.scope = scope
        self.function_declarations = {}  # name -> AST FunctionDeclaration
        self.function_connections = set()  # Set of function names that should be connected

    @classmethod
    def from_ast(cls, ast_node, scope: str = "global") -> 'CallGraphSubgraph':
        """Extract function declarations from AST and create subgraph.

        Scans the AST for function declarations and creates a temporary
        subgraph representing their structure and connections.

        Args:
            ast_node: Root AST node to scan
            scope: Scope name for the functions

        Returns:
            CallGraphSubgraph with all discovered functions
        """
        subgraph = cls(scope)
        subgraph._extract_functions_from_ast(ast_node)
        return subgraph

    def _extract_functions_from_ast(self, node):
        """Recursively extract function declarations from AST."""
        from glang.ast.nodes import FunctionDeclaration, Block

        if isinstance(node, FunctionDeclaration):
            # Found a function declaration - add to subgraph
            self.function_declarations[node.name] = node
            self.function_connections.add(node.name)

        # Recursively search child nodes
        if hasattr(node, '__dict__'):
            for attr_name, attr_value in node.__dict__.items():
                if isinstance(attr_value, list):
                    for item in attr_value:
                        if hasattr(item, 'accept'):  # AST node
                            self._extract_functions_from_ast(item)
                elif hasattr(attr_value, 'accept'):  # AST node
                    self._extract_functions_from_ast(attr_value)

    def merge_into(self, call_graph: CallGraph):
        """Merge this subgraph into the permanent call graph.

        Creates FunctionValue objects from AST declarations and adds them
        to the call graph with proper connections.

        Args:
            call_graph: Target call graph to merge into
        """
        from glang.execution.values import FunctionValue

        # Create FunctionValue objects for all declarations
        function_values = {}
        for func_name, func_decl in self.function_declarations.items():
            func_value = FunctionValue(
                name=func_decl.name,
                parameters=func_decl.parameters,
                body=func_decl.body,
                position=func_decl.position
            )
            function_values[func_name] = func_value

        # Add all functions to call graph using existing connect_module_functions
        # This ensures proper connectivity within the scope
        call_graph.connect_module_functions(self.scope, function_values)

    def get_function_names(self) -> List[str]:
        """Get list of all function names in this subgraph."""
        return list(self.function_declarations.keys())

    def __len__(self) -> int:
        """Return number of functions in subgraph."""
        return len(self.function_declarations)
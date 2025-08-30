"""
Graph management for the REPL.
"""

from typing import Dict, Optional, List, Any, Union
import ast
import re
from glang.core import Graph, GraphType, VariableGraph
from glang.visualization import render_graph
from glang.visualization.ascii_renderer import render_traversal


class GraphManager:
    """Manages graphs in the REPL session."""
    
    def __init__(self) -> None:
        """Initialize the graph manager."""
        # The variable namespace is itself a graph!
        self.variable_graph = VariableGraph()
        self.current_graph: Optional[str] = None
    
    def create_graph(self, name: str, graph_type: GraphType = GraphType.LINEAR) -> str:
        """Create a new graph."""
        if self.variable_graph.has_variable(name):
            return f"Graph '{name}' already exists. Use 'show {name}' to see it or 'delete {name}' to remove it."
        
        new_graph = Graph(graph_type)
        self.variable_graph.assign_variable(name, new_graph)
        self.current_graph = name
        return f"Created {graph_type} graph '{name}'"
    
    def create_from_list(self, name: str, data: List[Any]) -> str:
        """Create a linear graph from a list of data."""
        if self.variable_graph.has_variable(name):
            return f"Graph '{name}' already exists."
        
        try:
            graph = Graph.from_list(data)
            self.variable_graph.assign_variable(name, graph)
            self.current_graph = name
            return f"Created linear graph '{name}' with {len(data)} elements"
        except Exception as e:
            return f"Error creating graph: {e}"
    
    def delete_graph(self, name: str) -> str:
        """Delete a graph."""
        if not self.variable_graph.has_variable(name):
            return f"Graph '{name}' does not exist"
        
        success = self.variable_graph.delete_variable(name)
        if success:
            if self.current_graph == name:
                vars_list = self.variable_graph.list_variables()
                self.current_graph = vars_list[0] if vars_list else None
            return f"Deleted graph '{name}'"
        else:
            return f"Failed to delete graph '{name}'"
    
    def list_graphs(self) -> str:
        """List all graphs."""
        var_names = self.variable_graph.list_variables()
        if not var_names:
            return "No graphs created"
        
        lines = ["Available graphs:"]
        for name in var_names:
            graph = self.variable_graph.get_variable(name)
            current_marker = " *" if name == self.current_graph else "  "
            if graph:
                lines.append(f"{current_marker} {name}: {graph.graph_type} ({graph.size} nodes)")
            else:
                lines.append(f"{current_marker} {name}: undefined")
        
        return "\n".join(lines)
    
    def show_graph(self, name: Optional[str] = None) -> str:
        """Show a graph's structure."""
        graph_name = name or self.current_graph
        if not graph_name or not self.variable_graph.has_variable(graph_name):
            available = self.variable_graph.list_variables()
            return f"Graph not found. Available: {available}" if available else "No graphs available"
        
        graph = self.variable_graph.get_variable(graph_name)
        if not graph:
            return f"Graph '{graph_name}' is undefined"
            
        header = f"Graph '{graph_name}' ({graph.graph_type}):"
        visualization = render_graph(graph)
        
        return f"{header}\n{visualization}"
    
    def traverse_graph(self, name: Optional[str] = None) -> str:
        """Show traversal of a graph."""
        graph_name = name or self.current_graph
        if not graph_name or not self.variable_graph.has_variable(graph_name):
            return "No graph selected or graph not found"
        
        graph = self.variable_graph.get_variable(graph_name)
        if not graph:
            return f"Graph '{graph_name}' is undefined"
            
        return render_traversal(graph)
    
    def get_graph(self, name: Optional[str] = None) -> Optional[Graph]:
        """Get a graph by name."""
        graph_name = name or self.current_graph
        if not graph_name:
            return None
        return self.variable_graph.get_variable(graph_name)
    
    def parse_list_syntax(self, input_str: str) -> Optional[List[Any]]:
        """Parse list syntax like [1, 2, 3] or [a, b, c]."""
        # Simple parsing for basic list syntax
        input_str = input_str.strip()
        if not (input_str.startswith('[') and input_str.endswith(']')):
            return None
        
        # Remove brackets and split by comma
        inner = input_str[1:-1].strip()
        if not inner:
            return []
        
        items = []
        for item in inner.split(','):
            item = item.strip()
            if not item:
                continue
            
            # Try to parse as number
            try:
                if '.' in item:
                    items.append(float(item))
                else:
                    items.append(int(item))
            except ValueError:
                # Remove quotes if present and treat as string
                if (item.startswith('"') and item.endswith('"')) or \
                   (item.startswith("'") and item.endswith("'")):
                    items.append(item[1:-1])
                else:
                    items.append(item)
        
        return items
    
    def execute_graph_operation(self, graph_name: str, operation: str, *args) -> str:
        """Execute an operation on a graph."""
        if not self.variable_graph.has_variable(graph_name):
            return f"Graph '{graph_name}' not found"
        
        graph = self.variable_graph.get_variable(graph_name)
        if not graph:
            return f"Graph '{graph_name}' is undefined"
        
        try:
            if operation == "append":
                if not graph.graph_type.is_linear():
                    return "append() only works on linear graphs"
                if not args:
                    return "append requires a value"
                graph.append(args[0])
                return f"Appended {args[0]} to '{graph_name}'"
            
            elif operation == "prepend":
                if not graph.graph_type.is_linear():
                    return "prepend() only works on linear graphs"
                if not args:
                    return "prepend requires a value"
                graph.prepend(args[0])
                return f"Prepended {args[0]} to '{graph_name}'"
            
            elif operation == "insert":
                if not graph.graph_type.is_linear():
                    return "insert() only works on linear graphs"
                if len(args) < 2:
                    return "insert requires index and value"
                try:
                    index = int(args[0])
                    graph.insert(index, args[1])
                    return f"Inserted {args[1]} at index {index} in '{graph_name}'"
                except ValueError:
                    return "Index must be a number"
            
            elif operation == "delete":
                if not graph.graph_type.is_linear():
                    return "delete() only works on linear graphs"
                if not args:
                    return "delete requires an index"
                try:
                    index = int(args[0])
                    deleted = graph.delete(index)
                    if deleted is not None:
                        return f"Deleted {deleted} from index {index} in '{graph_name}'"
                    else:
                        return f"Invalid index {index}"
                except ValueError:
                    return "Index must be a number"
            
            elif operation == "reverse":
                if not graph.graph_type.is_linear():
                    return "reverse() only works on linear graphs"
                graph.reverse()
                return f"Reversed graph '{graph_name}'"
            
            else:
                return f"Unknown operation: {operation}"
        
        except Exception as e:
            return f"Error executing {operation}: {e}"
    
    def show_variable_graph(self) -> str:
        """Show the variable namespace graph itself."""
        return self.variable_graph.visualize_namespace()
    
    def get_variable_info(self, name: Optional[str] = None) -> str:
        """Get detailed information about a variable."""
        graph_name = name or self.current_graph
        if not graph_name:
            return "No graph selected"
        
        info = self.variable_graph.get_variable_info(graph_name)
        if not info:
            return f"Graph '{graph_name}' not found"
        
        lines = [
            f"Variable: {info['name']}",
            f"Type: {info['type']}",
            f"Size: {info['size']} nodes",
            f"Edges: {info['edges']}",
        ]
        
        # Add sample data for linear graphs
        if info['value'].graph_type.is_linear() and info['size'] > 0:
            sample_data = info['value'].to_list()[:5]
            if info['size'] > 5:
                lines.append(f"Sample data: {sample_data}... (showing first 5)")
            else:
                lines.append(f"Data: {sample_data}")
        
        return "\n".join(lines)
    
    def get_variable_stats(self) -> str:
        """Get statistics about the variable namespace."""
        var_count = self.variable_graph.get_variable_count()
        if var_count == 0:
            return "No variables defined"
        
        # Calculate total nodes across all graphs
        total_nodes = 0
        total_edges = 0
        type_counts = {}
        
        for name in self.variable_graph.list_variables():
            graph = self.variable_graph.get_variable(name)
            if graph:
                total_nodes += graph.size
                total_edges += graph.edge_count
                graph_type = str(graph.graph_type)
                type_counts[graph_type] = type_counts.get(graph_type, 0) + 1
        
        lines = [
            f"Variable namespace statistics:",
            f"  Variables: {var_count}",
            f"  Total data nodes: {total_nodes}",
            f"  Total data edges: {total_edges}",
            f"  Namespace nodes: {self.variable_graph.size}",
            f"  Assignment edges: {self.variable_graph.edge_count}",
        ]
        
        if type_counts:
            lines.append("  Graph types:")
            for graph_type, count in sorted(type_counts.items()):
                lines.append(f"    {graph_type}: {count}")
        
        return "\n".join(lines)
"""
Graph management for the REPL.
"""

from typing import Dict, Optional, List, Any, Union
import ast
import re
from glang.core import Graph, GraphType
from glang.visualization import render_graph
from glang.visualization.ascii_renderer import render_traversal


class GraphManager:
    """Manages graphs in the REPL session."""
    
    def __init__(self) -> None:
        """Initialize the graph manager."""
        self.graphs: Dict[str, Graph] = {}
        self.current_graph: Optional[str] = None
    
    def create_graph(self, name: str, graph_type: GraphType = GraphType.LINEAR) -> str:
        """Create a new graph."""
        if name in self.graphs:
            return f"Graph '{name}' already exists. Use 'show {name}' to see it or 'delete {name}' to remove it."
        
        self.graphs[name] = Graph(graph_type)
        self.current_graph = name
        return f"Created {graph_type} graph '{name}'"
    
    def create_from_list(self, name: str, data: List[Any]) -> str:
        """Create a linear graph from a list of data."""
        if name in self.graphs:
            return f"Graph '{name}' already exists."
        
        try:
            graph = Graph.from_list(data)
            self.graphs[name] = graph
            self.current_graph = name
            return f"Created linear graph '{name}' with {len(data)} elements"
        except Exception as e:
            return f"Error creating graph: {e}"
    
    def delete_graph(self, name: str) -> str:
        """Delete a graph."""
        if name not in self.graphs:
            return f"Graph '{name}' does not exist"
        
        del self.graphs[name]
        if self.current_graph == name:
            self.current_graph = list(self.graphs.keys())[0] if self.graphs else None
        
        return f"Deleted graph '{name}'"
    
    def list_graphs(self) -> str:
        """List all graphs."""
        if not self.graphs:
            return "No graphs created"
        
        lines = ["Available graphs:"]
        for name, graph in self.graphs.items():
            current_marker = " *" if name == self.current_graph else "  "
            lines.append(f"{current_marker} {name}: {graph.graph_type} ({graph.size} nodes)")
        
        return "\n".join(lines)
    
    def show_graph(self, name: Optional[str] = None) -> str:
        """Show a graph's structure."""
        graph_name = name or self.current_graph
        if not graph_name or graph_name not in self.graphs:
            available = list(self.graphs.keys())
            return f"Graph not found. Available: {available}" if available else "No graphs available"
        
        graph = self.graphs[graph_name]
        header = f"Graph '{graph_name}' ({graph.graph_type}):"
        visualization = render_graph(graph)
        
        return f"{header}\n{visualization}"
    
    def traverse_graph(self, name: Optional[str] = None) -> str:
        """Show traversal of a graph."""
        graph_name = name or self.current_graph
        if not graph_name or graph_name not in self.graphs:
            return "No graph selected or graph not found"
        
        graph = self.graphs[graph_name]
        return render_traversal(graph)
    
    def get_graph(self, name: Optional[str] = None) -> Optional[Graph]:
        """Get a graph by name."""
        graph_name = name or self.current_graph
        return self.graphs.get(graph_name) if graph_name else None
    
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
        if graph_name not in self.graphs:
            return f"Graph '{graph_name}' not found"
        
        graph = self.graphs[graph_name]
        
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
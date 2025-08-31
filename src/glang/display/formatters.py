"""Format-specific rendering utilities for glang display system."""

from typing import List, Optional, Dict, Any
from ..core.graph import Graph
from ..core.node import Node


class GraphFormatter:
    """Base class for graph formatters."""
    
    def format_graph(self, graph: Graph, **kwargs) -> str:
        """Format a graph for display."""
        raise NotImplementedError
    
    def format_node_data(self, data: Any) -> str:
        """Format node data for display."""
        if isinstance(data, str):
            # Check if it needs quotes for clarity
            if ' ' in data or data in ['', 'None', 'True', 'False'] or data.isdigit():
                return f"'{data}'"
            return data
        return str(data)


class SimpleListFormatter(GraphFormatter):
    """Formatter for simple list-style display: [item1, item2, item3]."""
    
    def format_graph(self, graph: Graph, compact: bool = True, **kwargs) -> str:
        """Format graph as simple list."""
        if not graph._head:
            return "[]"
        
        items = []
        current = graph._head
        visited = set()
        max_items = kwargs.get('max_items', 50)  # Prevent infinite loops
        
        while current and len(items) < max_items:
            if id(current) in visited:
                items.append("...")  # Cycle detected
                break
            visited.add(id(current))
            
            formatted_data = self.format_node_data(current.data)
            items.append(formatted_data)
            
            # Get next node
            successors = current.get_successors()
            current = next(iter(successors)) if successors else None
        
        if compact:
            return f"[{', '.join(items)}]"
        else:
            # Multi-line format for long lists
            if len(items) <= 5:
                return f"[{', '.join(items)}]"
            else:
                lines = ["["]
                for i, item in enumerate(items):
                    comma = "," if i < len(items) - 1 else ""
                    lines.append(f"  {item}{comma}")
                lines.append("]")
                return "\n".join(lines)


class DetailedNodeFormatter(GraphFormatter):
    """Formatter for detailed node-level display."""
    
    def format_graph(self, graph: Graph, variable_name: Optional[str] = None, 
                    show_ids: bool = True, **kwargs) -> str:
        """Format graph with detailed node information."""
        if not graph._head:
            name_part = f"'{variable_name}' " if variable_name else ""
            return f"Graph {name_part}(empty)"
        
        lines = []
        
        # Header
        if variable_name:
            lines.append(f"Graph '{variable_name}' ({graph.graph_type.name}):")
        else:
            lines.append(f"Graph ({graph.graph_type.name}):")
        
        # Show structure
        current = graph._head
        visited = set()
        node_strs = []
        
        while current:
            if id(current) in visited:
                node_strs.append("... (cycle)")
                break
            visited.add(id(current))
            
            if show_ids:
                node_id = str(current.id)[:8] + "..."
                node_str = f"Node({node_id}, data={current.data!r})"
            else:
                node_str = f"[{self.format_node_data(current.data)}]"
            
            node_strs.append(node_str)
            
            # Get next node
            successors = current.get_successors()
            current = next(iter(successors)) if successors else None
        
        # Join with arrows or newlines based on complexity
        if len(node_strs) <= 3:
            lines.append(" -> ".join(node_strs))
        else:
            lines.append("Nodes:")
            for i, node_str in enumerate(node_strs):
                arrow = " ↓" if i < len(node_strs) - 1 else ""
                lines.append(f"  {i}: {node_str}{arrow}")
        
        return "\n".join(lines)


class MetaInfoFormatter(GraphFormatter):
    """Formatter for meta information about graphs."""
    
    def format_graph(self, graph: Graph, variable_name: Optional[str] = None, 
                    show_stats: bool = True, show_type_info: bool = True, **kwargs) -> str:
        """Format meta information about the graph."""
        lines = []
        
        # Basic info
        if variable_name:
            lines.append(f"Variable: {variable_name}")
        
        if show_type_info:
            lines.append(f"Type: {graph.graph_type.name}")
            lines.append(f"Linear: {'Yes' if graph.graph_type.is_linear() else 'No'}")
        
        if show_stats:
            lines.append(f"Nodes: {len(graph.nodes)}")
            lines.append(f"Edges: {len(graph.edges)}")
            
            if graph.graph_type.is_linear():
                lines.append(f"Length: {len(graph.nodes)}")
        
        # Structure info
        if graph._head:
            head_data = self.format_node_data(graph._head.data)
            lines.append(f"Head: {head_data}")
        else:
            lines.append("Head: None (empty)")
        
        if graph._tail:
            tail_data = self.format_node_data(graph._tail.data)
            lines.append(f"Tail: {tail_data}")
        else:
            lines.append("Tail: None (empty)")
        
        # Sample content for large graphs
        if len(graph.nodes) > 10:
            sample_items = []
            current = graph._head
            count = 0
            while current and count < 3:
                sample_items.append(self.format_node_data(current.data))
                successors = current.get_successors()
                current = next(iter(successors)) if successors else None
                count += 1
            
            if sample_items:
                lines.append(f"Preview: [{', '.join(sample_items)}, ...]")
        
        return "\n".join(lines)


class JsonFormatter(GraphFormatter):
    """Formatter for JSON-like output."""
    
    def format_graph(self, graph: Graph, variable_name: Optional[str] = None, 
                    indent: int = 2, **kwargs) -> str:
        """Format graph as JSON structure."""
        data = {
            "type": graph.graph_type.name.lower(),
            "size": len(graph.nodes),
            "edges": len(graph.edges)
        }
        
        if variable_name:
            data["name"] = variable_name
        
        if graph.graph_type.is_linear():
            # For linear graphs, include the data sequence
            items = []
            current = graph._head
            while current:
                items.append(current.data)
                successors = current.get_successors()
                current = next(iter(successors)) if successors else None
            data["data"] = items
        
        # Simple JSON formatting (without importing json module)
        lines = ["{"]
        items = list(data.items())
        for i, (key, value) in enumerate(items):
            if isinstance(value, str):
                value_str = f'"{value}"'
            elif isinstance(value, list):
                if all(isinstance(item, str) for item in value):
                    quoted_items = [f'"{item}"' for item in value]
                    value_str = f'[{", ".join(quoted_items)}]'
                else:
                    value_str = f'[{", ".join(str(item) for item in value)}]'
            else:
                value_str = str(value)
            
            comma = "," if i < len(items) - 1 else ""
            lines.append(f'{" " * indent}"{key}": {value_str}{comma}')
        
        lines.append("}")
        return "\n".join(lines)
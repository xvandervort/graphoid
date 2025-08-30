"""
ASCII art renderer for graphs.
"""

from typing import List, Dict, Any
from glang.core import Graph, GraphType


def render_graph(graph: Graph, max_width: int = 60) -> str:
    """
    Render a graph as ASCII art.
    
    Args:
        graph: The graph to render
        max_width: Maximum width for rendering
        
    Returns:
        ASCII art representation of the graph
    """
    if graph.is_empty():
        return "Empty graph"
    
    if graph.graph_type.is_linear():
        return _render_linear_graph(graph, max_width)
    else:
        return _render_general_graph(graph, max_width)


def _render_linear_graph(graph: Graph, max_width: int) -> str:
    """Render a linear graph as a linked list."""
    if graph.is_empty():
        return "[]"
    
    data_list = graph.to_list()
    
    # Simple linear representation
    if len(data_list) <= 5:
        # Show as: [A] -> [B] -> [C] -> [D]
        node_strs = [f"[{_format_data(item)}]" for item in data_list]
        return " -> ".join(node_strs)
    else:
        # Show first few, ellipsis, last few
        first_part = [f"[{_format_data(item)}]" for item in data_list[:2]]
        last_part = [f"[{_format_data(item)}]" for item in data_list[-2:]]
        
        result = " -> ".join(first_part)
        result += " -> ... -> "
        result += " -> ".join(last_part)
        result += f"  (size: {len(data_list)})"
        
        return result


def _render_general_graph(graph: Graph, max_width: int) -> str:
    """Render a general graph showing nodes and connections."""
    lines = [f"Graph ({graph.graph_type}):"]
    lines.append(f"  Nodes: {graph.size}")
    lines.append(f"  Edges: {graph.edge_count}")
    lines.append("")
    
    if graph.size <= 10:
        # Show all nodes and edges
        lines.append("Nodes:")
        for node in list(graph.nodes)[:5]:  # Limit to first 5 for display
            data_str = _format_data(node.data)
            connections = len(node.get_successors())
            lines.append(f"  {node.id[:8]}... -> {data_str} (connections: {connections})")
        
        if graph.size > 5:
            lines.append(f"  ... and {graph.size - 5} more nodes")
    else:
        lines.append(f"Large graph with {graph.size} nodes (use 'traverse' to see data)")
    
    return "\n".join(lines)


def _format_data(data: Any, max_len: int = 10) -> str:
    """Format data for display, truncating if necessary."""
    if data is None:
        return "null"
    
    data_str = str(data)
    if len(data_str) > max_len:
        return data_str[:max_len-3] + "..."
    return data_str


def render_traversal(graph: Graph) -> str:
    """Render traversal order of the graph."""
    if graph.is_empty():
        return "Empty graph - no traversal"
    
    if graph.graph_type.is_linear():
        data_list = graph.traverse()
        if len(data_list) <= 20:
            return f"Traversal: {data_list}"
        else:
            return f"Traversal: {data_list[:10]} ... {data_list[-10:]} (total: {len(data_list)} items)"
    else:
        # For non-linear graphs, show node data
        node_data = [node.data for node in list(graph.nodes)[:10]]
        if graph.size > 10:
            return f"Node data (first 10): {node_data} ... ({graph.size} total)"
        else:
            return f"Node data: {node_data}"
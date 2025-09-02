"""Methods that work on all graph types."""

from typing import List, Optional, Any
from ..core.graph import Graph
from ..core.graph_types import GraphType


class GraphMethods:
    """Methods that work on any graph type."""
    
    @staticmethod
    def size(graph: Graph, var_name: str, args: List[str]) -> str:
        """Get the number of nodes in the graph."""
        return str(len(graph.nodes))
    
    @staticmethod
    def empty(graph: Graph, var_name: str, args: List[str]) -> str:
        """Check if the graph is empty."""
        return "true" if len(graph.nodes) == 0 else "false"
    
    @staticmethod
    def clear(graph: Graph, var_name: str, args: List[str]) -> str:
        """Clear all nodes and edges from the graph."""
        graph.clear()
        return f"Cleared {var_name}"
    
    @staticmethod
    def edges(graph: Graph, var_name: str, args: List[str]) -> str:
        """Get the number of edges in the graph."""
        return str(len(graph.edges))
    
    @staticmethod
    def type_info(graph: Graph, var_name: str, args: List[str]) -> str:
        """Get type information about the graph."""
        graph_type = graph.graph_type
        info = [
            f"Type: {graph_type.name}",
            f"Linear: {'Yes' if graph_type.is_linear() else 'No'}",
            f"Directed: {'Yes' if graph_type.is_directed() else 'No'}",
            f"Weighted: {'Yes' if graph_type.is_weighted() else 'No'}"
        ]
        return " | ".join(info)
    
    @staticmethod
    def to_list(graph: Graph, var_name: str, args: List[str]) -> str:
        """Convert graph to list representation (for linear graphs)."""
        if not graph.graph_type.is_linear():
            return f"Error: to_list() only works on linear graphs"
        
        try:
            data = graph.to_list()
            return str(data)
        except Exception as e:
            return f"Error: {str(e)}"
    
    @staticmethod
    def traverse(graph: Graph, var_name: str, args: List[str]) -> str:
        """Get traversal of the graph."""
        try:
            data = graph.traverse()
            return f"Traversal: {data}"
        except Exception as e:
            return f"Error: {str(e)}"
    
    @staticmethod
    def copy(graph: Graph, var_name: str, args: List[str]) -> str:
        """Create a copy of the graph (returns info, doesn't actually copy to new variable)."""
        # This would ideally create a new variable, but for now just show info
        return f"Graph copy would have {len(graph.nodes)} nodes and {len(graph.edges)} edges"
    
    @staticmethod
    def is_linear(graph: Graph, var_name: str, args: List[str]) -> str:
        """Check if graph is linear."""
        return "true" if graph.graph_type.is_linear() else "false"
    
    @staticmethod
    def is_directed(graph: Graph, var_name: str, args: List[str]) -> str:
        """Check if graph is directed."""
        return "true" if graph.graph_type.is_directed() else "false"
    
    @staticmethod
    def is_weighted(graph: Graph, var_name: str, args: List[str]) -> str:
        """Check if graph is weighted."""
        return "true" if graph.graph_type.is_weighted() else "false"
    
    @staticmethod
    def has_cycles(graph: Graph, var_name: str, args: List[str]) -> str:
        """Check if graph has cycles (simplified check)."""
        if graph.graph_type.is_linear():
            return "false"  # Linear graphs don't have cycles
        
        # For other graph types, this would require more complex cycle detection
        return "unknown (cycle detection not yet implemented for non-linear graphs)"
    
    @staticmethod
    def stats(graph: Graph, var_name: str, args: List[str]) -> str:
        """Get comprehensive statistics about the graph."""
        stats = [
            f"Variable: {var_name}",
            f"Type: {graph.graph_type.name}",
            f"Nodes: {len(graph.nodes)}",
            f"Edges: {len(graph.edges)}",
        ]
        
        if graph.graph_type.is_linear():
            stats.append(f"Length: {len(graph.nodes)}")
            if graph._head:
                stats.append(f"Head: {repr(graph._head.data)}")
            if graph._tail:
                stats.append(f"Tail: {repr(graph._tail.data)}")
        
        return "\n".join(stats)


class ConversionMethods:
    """Methods for converting between graph types."""
    
    @staticmethod
    def to_directed(graph: Graph, var_name: str, args: List[str]) -> str:
        """Convert graph to directed type (conceptual for now)."""
        if graph.graph_type.is_directed():
            return f"{var_name} is already a directed graph (type: {graph.graph_type.name})"
        
        # This would ideally create a new directed graph
        return f"Would convert {var_name} from {graph.graph_type.name} to DIRECTED graph"
    
    @staticmethod
    def to_undirected(graph: Graph, var_name: str, args: List[str]) -> str:
        """Convert graph to undirected type (conceptual for now)."""
        if graph.graph_type == GraphType.UNDIRECTED:
            return f"{var_name} is already an undirected graph"
        
        return f"Would convert {var_name} from {graph.graph_type.name} to UNDIRECTED graph"
    
    @staticmethod
    def to_tree(graph: Graph, var_name: str, args: List[str]) -> str:
        """Convert graph to tree type (conceptual for now)."""
        if graph.graph_type == GraphType.TREE:
            return f"{var_name} is already a tree graph"
        
        return f"Would convert {var_name} from {graph.graph_type.name} to TREE graph"
    
    @staticmethod
    def to_linear(graph: Graph, var_name: str, args: List[str]) -> str:
        """Convert graph to linear type."""
        if graph.graph_type == GraphType.LINEAR:
            return f"{var_name} is already a linear graph"
        
        # For now, just show what would happen
        return f"Would convert {var_name} from {graph.graph_type.name} to LINEAR graph"
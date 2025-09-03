"""
Variable storage system using graphs for Glang.

In glang, everything is a graph - including the variable namespace itself.
"""

from typing import Optional, List, Any, Dict, Union
from .graph import Graph
from .node import Node
from .edge import Edge
from .graph_types import GraphType
from .atomic_value import AtomicValue


class VariableNode(Node):
    """
    A specialized node for variable storage.
    
    Can represent either a variable name or a variable value.
    """
    
    def __init__(self, data: Any, node_type: str = "value", node_id: Optional[str] = None):
        """
        Initialize a variable node.
        
        Args:
            data: The data stored (variable name string, Graph value, or AtomicValue)
            node_type: Either "name" or "value" 
            node_id: Optional custom ID
        """
        super().__init__(data, node_id)
        self.node_type = node_type  # "name" or "value"
    
    def is_name_node(self) -> bool:
        """Check if this is a variable name node."""
        return self.node_type == "name"
    
    def is_value_node(self) -> bool:
        """Check if this is a variable value node."""
        return self.node_type == "value"
    
    def __str__(self) -> str:
        """String representation."""
        type_indicator = "📛" if self.is_name_node() else "📊"
        return f"{type_indicator} {super().__str__()}"


class VariableGraph(Graph):
    """
    A graph that manages variable storage in Glang.
    
    This is the meta-graph that stores the variable namespace:
    - Name nodes contain variable names (strings)
    - Value nodes contain variable values (Graph objects)
    - Edges connect names to their current values
    """
    
    def __init__(self):
        """Initialize the variable graph."""
        super().__init__(GraphType.DIRECTED)
        self._name_nodes: Dict[str, VariableNode] = {}
        self._value_nodes: List[VariableNode] = []
    
    def assign_variable(self, name: str, value: Graph) -> None:
        """
        Assign a value to a variable name.
        
        Creates or updates the name->value relationship in the graph.
        """
        # Get or create name node
        if name not in self._name_nodes:
            name_node = VariableNode(name, "name", f"name_{name}")
            self._name_nodes[name] = name_node
            self.add_node(name_node)
        else:
            name_node = self._name_nodes[name]
            # Remove existing assignment edges
            for edge in list(name_node.outgoing_edges):
                if edge.get_metadata("assignment"):
                    self.remove_edge(edge)
        
        # Create value node
        value_node = VariableNode(value, "value")
        self._value_nodes.append(value_node)
        self.add_node(value_node)
        
        # Create assignment edge
        assignment_edge = self.add_edge(name_node, value_node)
        assignment_edge.set_metadata("assignment", True)
        assignment_edge.set_metadata("variable_name", name)
    
    def get_variable(self, name: str) -> Optional[Union[Graph, AtomicValue]]:
        """Get the value of a variable (can be Graph or AtomicValue)."""
        if name not in self._name_nodes:
            return None
        
        name_node = self._name_nodes[name]
        
        # Find the assignment edge to get current value
        for edge in name_node.outgoing_edges:
            if edge.get_metadata("assignment"):
                value_node = edge.to_node
                if isinstance(value_node, VariableNode) and value_node.is_value_node():
                    return value_node.data
        
        return None
    
    def delete_variable(self, name: str) -> bool:
        """Delete a variable and its value."""
        if name not in self._name_nodes:
            return False
        
        name_node = self._name_nodes[name]
        
        # Find and remove the value node and edge
        for edge in list(name_node.outgoing_edges):
            if edge.get_metadata("assignment"):
                value_node = edge.to_node
                self.remove_node(value_node)  # This also removes the edge
                if isinstance(value_node, VariableNode):
                    self._value_nodes.remove(value_node)
        
        # Remove the name node
        self.remove_node(name_node)
        del self._name_nodes[name]
        return True
    
    def list_variables(self) -> List[str]:
        """Get list of all variable names."""
        return list(self._name_nodes.keys())
    
    def has_variable(self, name: str) -> bool:
        """Check if a variable exists."""
        return name in self._name_nodes
    
    def get_variable_count(self) -> int:
        """Get the number of variables stored."""
        return len(self._name_nodes)
    
    def get_variable_info(self, name: str) -> Optional[Dict[str, Any]]:
        """Get detailed information about a variable."""
        if name not in self._name_nodes:
            return None
        
        value = self.get_variable(name)
        if value is None:
            return None
        
        if isinstance(value, AtomicValue):
            return {
                "name": name,
                "type": f"atomic_{value.atomic_type}",
                "size": 1,
                "edges": 0,
                "value": value
            }
        else:
            # Graph object
            return {
                "name": name,
                "type": str(value.graph_type),
                "size": value.size,
                "edges": value.edge_count,
                "value": value
            }
    
    def visualize_namespace(self) -> str:
        """
        Create a visualization of the variable namespace graph.
        
        Shows the relationships between variable names and their values.
        """
        if not self._name_nodes:
            return "No variables defined"
        
        lines = ["Variable Namespace Graph:"]
        lines.append(f"  Variables: {len(self._name_nodes)}")
        lines.append(f"  Total nodes: {self.size}")
        lines.append(f"  Assignment edges: {len([e for e in self.edges if e.get_metadata('assignment')])}")
        lines.append("")
        
        for name, name_node in self._name_nodes.items():
            value = self.get_variable(name)
            if value is not None:
                if isinstance(value, AtomicValue):
                    lines.append(f"  📛 {name} -> 💎 atomic_{value.atomic_type} = {value}")
                else:
                    lines.append(f"  📛 {name} -> 📊 {value.graph_type} graph ({value.size} nodes)")
            else:
                lines.append(f"  📛 {name} -> ❌ undefined")
        
        return "\n".join(lines)
    
    def get_variable_dependencies(self) -> Dict[str, List[str]]:
        """
        Analyze dependencies between variables (future feature).
        
        This could track if one graph references nodes from another graph.
        """
        # Placeholder for future dependency analysis
        return {name: [] for name in self._name_nodes.keys()}
"""Method dispatcher for graph operations."""

from typing import List, Optional
from ..core.graph import Graph
from ..repl.graph_manager import GraphManager


class MethodDispatcher:
    """Dispatches method calls to appropriate graph operations."""
    
    def __init__(self, graph_manager: GraphManager):
        self.graph_manager = graph_manager
    
    def dispatch_method(self, variable_name: str, method_name: str, 
                       arguments: List[str]) -> str:
        """Dispatch a method call to the appropriate handler."""
        # Get the graph
        graph = self.graph_manager.get_variable(variable_name)
        if not graph:
            return f"Error: Variable '{variable_name}' not found"
        
        # Check if method exists
        method_handler = getattr(self, f'_method_{method_name}', None)
        if not method_handler:
            return f"Error: Method '{method_name}' not supported"
        
        # Call the method handler
        try:
            return method_handler(graph, variable_name, arguments)
        except Exception as e:
            return f"Error: {str(e)}"
    
    def _method_append(self, graph: Graph, var_name: str, args: List[str]) -> str:
        """Handle append method."""
        if not graph.graph_type.is_linear():
            return f"Error: append() only works on linear graphs"
        if not args:
            return f"Error: append requires a value"
        
        value = ' '.join(args)  # Join args in case of multi-word values
        graph.append(value)
        self.graph_manager.set_current(var_name)
        return f"Appended '{value}' to {var_name}"
    
    def _method_prepend(self, graph: Graph, var_name: str, args: List[str]) -> str:
        """Handle prepend method."""
        if not graph.graph_type.is_linear():
            return f"Error: prepend() only works on linear graphs"
        if not args:
            return f"Error: prepend requires a value"
        
        value = ' '.join(args)
        graph.prepend(value)
        self.graph_manager.set_current(var_name)
        return f"Prepended '{value}' to {var_name}"
    
    def _method_insert(self, graph: Graph, var_name: str, args: List[str]) -> str:
        """Handle insert method."""
        if not graph.graph_type.is_linear():
            return f"Error: insert() only works on linear graphs"
        if len(args) < 2:
            return f"Error: insert requires an index and a value"
        
        try:
            index = int(args[0])
            value = ' '.join(args[1:])
            graph.insert(index, value)
            self.graph_manager.set_current(var_name)
            return f"Inserted '{value}' at index {index} in {var_name}"
        except ValueError:
            return f"Error: First argument to insert must be a number"
        except IndexError as e:
            return f"Error: {str(e)}"
    
    def _method_reverse(self, graph: Graph, var_name: str, args: List[str]) -> str:
        """Handle reverse method."""
        if not graph.graph_type.is_linear():
            return f"Error: reverse() only works on linear graphs"
        
        graph.reverse()
        self.graph_manager.set_current(var_name)
        return f"Reversed {var_name}"
    
    def _method_delete(self, graph: Graph, var_name: str, args: List[str]) -> str:
        """Handle delete method."""
        if not graph.graph_type.is_linear():
            return f"Error: delete() only works on linear graphs"
        if not args:
            return f"Error: delete requires an index"
        
        try:
            index = int(args[0])
            old_value = None
            
            # Get the value at the index before deleting
            current = graph.head
            for i in range(index):
                if not current:
                    break
                edges = graph.get_edges_from(current)
                if edges:
                    current = edges[0].to_node
                else:
                    current = None
            if current:
                old_value = current.data
            
            # Perform deletion
            graph.delete(index)
            self.graph_manager.set_current(var_name)
            
            if old_value is not None:
                return f"Deleted '{old_value}' from index {index} in {var_name}"
            else:
                return f"Deleted element at index {index} from {var_name}"
        except ValueError:
            return f"Error: Argument to delete must be a number"
        except IndexError as e:
            return f"Error: {str(e)}"
    
    def _method_size(self, graph: Graph, var_name: str, args: List[str]) -> str:
        """Handle size method."""
        return str(len(graph.nodes))
    
    def _method_empty(self, graph: Graph, var_name: str, args: List[str]) -> str:
        """Handle empty method."""
        return "true" if len(graph.nodes) == 0 else "false"
    
    def _method_clear(self, graph: Graph, var_name: str, args: List[str]) -> str:
        """Handle clear method."""
        graph.nodes.clear()
        graph.edges.clear()
        graph.head = None
        graph.tail = None
        self.graph_manager.set_current(var_name)
        return f"Cleared {var_name}"
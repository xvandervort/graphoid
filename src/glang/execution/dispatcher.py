"""
Glang Method Dispatcher

Bridges between AST execution and existing method system.
This allows the new AST-based execution to utilize the existing
graph operations and method implementations.
"""

from typing import List, Any, Optional
import sys
import os

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../../..'))

from glang.core.graph import Graph
from glang.methods.linear_methods import LinearGraphMethods
from .values import *
from .errors import RuntimeError, MethodNotFoundError, ArgumentError


class ASTMethodDispatcher:
    """Dispatches method calls from AST execution to graph methods."""
    
    def __init__(self):
        self.linear_methods = LinearGraphMethods()
    
    def dispatch_method(self, target: GlangValue, method_name: str, 
                       args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Dispatch method call based on target type."""
        
        target_type = target.get_type()
        
        if target_type == "list":
            return self._dispatch_list_method(target, method_name, args, position)
        elif target_type == "string":
            return self._dispatch_string_method(target, method_name, args, position)
        else:
            raise MethodNotFoundError(method_name, target_type, position)
    
    def _dispatch_list_method(self, target: ListValue, method_name: str, 
                             args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Handle list method calls by integrating with existing graph system."""
        
        # Convert to graph for existing method system
        graph = self._list_value_to_graph(target)
        
        if method_name == "append":
            if len(args) != 1:
                raise ArgumentError(f"append() takes 1 argument, got {len(args)}", position)
            
            # Validate constraint
            if target.constraint and not args[0].get_type() == target.constraint:
                from .errors import TypeConstraintError
                raise TypeConstraintError(
                    f"Cannot append {args[0].get_type()} to list<{target.constraint}>", 
                    position
                )
            
            # Call existing method - convert value to string for existing system
            python_value = args[0].to_python()
            result = self.linear_methods.append(graph, "temp", [str(python_value)])
            
            # Update list value to stay in sync
            target.elements.append(args[0])
            
            return result
        
        elif method_name == "prepend":
            if len(args) != 1:
                raise ArgumentError(f"prepend() takes 1 argument, got {len(args)}", position)
            
            # Validate constraint
            if target.constraint and not args[0].get_type() == target.constraint:
                from .errors import TypeConstraintError
                raise TypeConstraintError(
                    f"Cannot prepend {args[0].get_type()} to list<{target.constraint}>", 
                    position
                )
            
            # Call existing method
            python_value = args[0].to_python()
            result = self.linear_methods.prepend(graph, "temp", [str(python_value)])
            
            # Update list value
            target.elements.insert(0, args[0])
            
            return result
        
        elif method_name == "insert":
            if len(args) != 2:
                raise ArgumentError(f"insert() takes 2 arguments, got {len(args)}", position)
            
            index_arg, value_arg = args
            if not isinstance(index_arg, NumberValue) or not isinstance(index_arg.value, int):
                raise ArgumentError("insert() first argument must be integer", position)
            
            # Validate constraint
            if target.constraint and not value_arg.get_type() == target.constraint:
                from .errors import TypeConstraintError
                raise TypeConstraintError(
                    f"Cannot insert {value_arg.get_type()} into list<{target.constraint}>", 
                    position
                )
            
            # Call existing method
            index = index_arg.value
            python_value = value_arg.to_python()
            result = self.linear_methods.insert(graph, "temp", [str(index), str(python_value)])
            
            # Update list value
            if 0 <= index <= len(target.elements):
                target.elements.insert(index, value_arg)
            
            return result
        
        elif method_name == "reverse":
            if len(args) != 0:
                raise ArgumentError(f"reverse() takes no arguments, got {len(args)}", position)
            
            # Call existing method
            result = self.linear_methods.reverse(graph, "temp", [])
            
            # Update list value
            target.elements.reverse()
            
            return result
        
        elif method_name == "show":
            # Display method - doesn't modify the list
            if len(args) != 0:
                raise ArgumentError(f"show() takes no arguments, got {len(args)}", position)
            
            return self.linear_methods.show(graph, "temp", [])
        
        elif method_name == "traverse":
            # Traversal method - doesn't modify the list
            if len(args) != 0:
                raise ArgumentError(f"traverse() takes no arguments, got {len(args)}", position)
            
            return self.linear_methods.traverse(graph, "temp", [])
        
        else:
            raise MethodNotFoundError(method_name, "list", position)
    
    def _dispatch_string_method(self, target: StringValue, method_name: str, 
                               args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Handle string method calls."""
        
        # Basic string methods (could be expanded)
        if method_name == "length":
            if len(args) != 0:
                raise ArgumentError(f"length() takes no arguments, got {len(args)}", position)
            
            return NumberValue(len(target.value), position)
        
        elif method_name == "upper":
            if len(args) != 0:
                raise ArgumentError(f"upper() takes no arguments, got {len(args)}", position)
            
            return StringValue(target.value.upper(), position)
        
        elif method_name == "lower":
            if len(args) != 0:
                raise ArgumentError(f"lower() takes no arguments, got {len(args)}", position)
            
            return StringValue(target.value.lower(), position)
        
        else:
            raise MethodNotFoundError(method_name, "string", position)
    
    def _list_value_to_graph(self, list_value: ListValue) -> Graph:
        """Convert ListValue to Graph for method calls."""
        # Create a linear graph from the list elements
        python_list = [elem.to_python() for elem in list_value.elements]
        graph = Graph.from_list(python_list)
        return graph
    
    def _graph_to_list_value(self, graph: Graph, constraint: Optional[str] = None, 
                            position: Optional[SourcePosition] = None) -> ListValue:
        """Convert Graph back to ListValue after method operations."""
        # Extract data from graph nodes in order
        elements = []
        for node in graph.nodes:
            python_value = node.data
            glang_value = python_to_glang_value(python_value, position)
            elements.append(glang_value)
        
        return ListValue(elements, constraint, position)
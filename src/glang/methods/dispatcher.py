"""Enhanced method dispatcher for graph operations."""

from typing import List, Optional, Dict, Callable, Union
from ..core.graph import Graph
from ..core.atomic_value import AtomicValue
from .linear_methods import LinearGraphMethods
from .graph_methods import GraphMethods, ConversionMethods


class MethodDispatcher:
    """Dispatches method calls to appropriate graph operations."""
    
    def __init__(self, graph_manager):
        self.graph_manager = graph_manager
        # Import here to avoid circular imports
        from ..parser.expression_evaluator import ExpressionEvaluator
        self.expression_evaluator = ExpressionEvaluator(graph_manager)
        
        # Method categories for better organization
        self.mutating_methods = {
            # Linear graph methods
            'append': LinearGraphMethods.append,
            'prepend': LinearGraphMethods.prepend,
            'insert': LinearGraphMethods.insert,
            'reverse': LinearGraphMethods.reverse,
            'delete': LinearGraphMethods.delete_at,
            'set': LinearGraphMethods.set,
            'clear': GraphMethods.clear,
        }
        
        self.query_methods = {
            # Linear graph queries
            'get': LinearGraphMethods.get,
            'find': LinearGraphMethods.find,
            'find_all': LinearGraphMethods.find_all,
            'count': LinearGraphMethods.count,
            'slice': LinearGraphMethods.slice,
            'types': LinearGraphMethods.types,
            'typeof': LinearGraphMethods.typeof,
            'constraint': LinearGraphMethods.constraint,
            'validate_constraint': LinearGraphMethods.validate_constraint,
            'type_summary': LinearGraphMethods.type_summary,
            'coerce_to_constraint': LinearGraphMethods.coerce_to_constraint,
            
            # General graph queries
            'size': GraphMethods.size,
            'empty': GraphMethods.empty,
            'edges': GraphMethods.edges,
            'type': GraphMethods.type_info,
            'stats': GraphMethods.stats,
            'traverse': GraphMethods.traverse,
            'to_list': GraphMethods.to_list,
            
            # Type checking queries
            'is_linear': GraphMethods.is_linear,
            'is_directed': GraphMethods.is_directed,
            'is_weighted': GraphMethods.is_weighted,
            'has_cycles': GraphMethods.has_cycles,
        }
        
        self.conversion_methods = {
            'to_directed': ConversionMethods.to_directed,
            'to_undirected': ConversionMethods.to_undirected,
            'to_tree': ConversionMethods.to_tree,
            'to_linear': ConversionMethods.to_linear,
            'copy': GraphMethods.copy,
        }
        
        # All available methods
        self.all_methods = {
            **self.mutating_methods,
            **self.query_methods,
            **self.conversion_methods
        }
    
    def dispatch_method(self, variable_name: str, method_name: str, 
                       arguments: List[str]) -> str:
        """Dispatch a method call to the appropriate handler."""
        # Get the variable (can be Graph or AtomicValue)
        variable = self.graph_manager.get_variable(variable_name)
        if variable is None:
            # Debug: List available variables
            available = self.graph_manager.variable_graph.list_variables() if hasattr(self.graph_manager, 'variable_graph') else 'No variable_graph'
            return f"Error: Variable '{variable_name}' not found (available: {available})"
        
        # Check if trying to call graph methods on atomic values
        if isinstance(variable, AtomicValue):
            return self._handle_atomic_method_call(variable, method_name, arguments)
        
        # For Graph objects, continue with existing logic
        graph = variable
        
        # Find method handler
        method_handler = self.all_methods.get(method_name)
        if not method_handler:
            return self._suggest_similar_methods(method_name)
        
        # Validate method compatibility with graph type
        error_msg = self._validate_method_compatibility(graph, method_name)
        if error_msg:
            return error_msg
        
        # Evaluate arguments before calling method handler
        try:
            evaluated_args = self._evaluate_arguments(arguments)
        except Exception as e:
            return f"Error evaluating arguments: {str(e)}"
        
        # Call the method handler
        try:
            result = method_handler(graph, variable_name, evaluated_args)
            
            # Update current graph if it was a mutating operation
            if method_name in self.mutating_methods:
                self.graph_manager.set_current(variable_name)
            
            return result
        except Exception as e:
            return f"Error in {method_name}: {str(e)}"
    
    def _evaluate_arguments(self, arguments: List[str]) -> List[str]:
        """Evaluate method arguments, resolving variable references and expressions."""
        evaluated = []
        
        for arg in arguments:
            # Join in case of multi-word arguments  
            arg = arg.strip()
            
            # Try to evaluate as expression (variable reference or index expression)
            try:
                # Check if it looks like a variable reference or expression
                if (self.expression_evaluator._is_simple_variable(arg) or 
                    self.expression_evaluator._is_index_expression(arg)):
                    # Evaluate the expression
                    value = self.expression_evaluator.evaluate_expression(arg, allow_multi_element=True)
                    
                    # Convert evaluated value back to string for method handler
                    # (method handlers expect string arguments and do their own type inference)
                    if isinstance(value, str):
                        evaluated.append(value)
                    elif isinstance(value, bool):
                        evaluated.append('true' if value else 'false')
                    elif isinstance(value, (int, float)):
                        evaluated.append(str(value))
                    elif isinstance(value, list):
                        # For list values, we need to convert them to a format the method can understand
                        # This is tricky - for now, let's convert to a readable list format
                        # The append method will need to handle this specially
                        evaluated.append(f"__LIST__{value}")
                    else:
                        evaluated.append(str(value))
                else:
                    # Not a variable/expression - treat as literal
                    # Apply standard type inference for literals  
                    from ..parser.tokenizer import Tokenizer
                    tokenizer = Tokenizer()
                    typed_value = tokenizer.infer_value_type(arg)
                    
                    # Convert back to string representation for method handler
                    if isinstance(typed_value, str):
                        evaluated.append(typed_value)
                    elif isinstance(typed_value, bool):
                        evaluated.append('true' if typed_value else 'false')
                    else:
                        evaluated.append(str(typed_value))
                        
            except ValueError:
                # If evaluation fails, treat as literal string
                evaluated.append(arg)
        
        return evaluated
    
    def _validate_method_compatibility(self, graph: Graph, method_name: str) -> Optional[str]:
        """Validate that a method is compatible with the graph type."""
        # Methods that require linear graphs
        linear_only_methods = {
            'append', 'prepend', 'insert', 'reverse', 'delete', 
            'get', 'set', 'find', 'find_all', 'count', 'slice', 'to_list',
            'types', 'typeof'
        }
        
        if method_name in linear_only_methods and not graph.graph_type.is_linear():
            return f"Error: {method_name}() only works on linear graphs (current: {graph.graph_type.name})"
        
        # Methods that require non-empty graphs
        non_empty_methods = {'reverse', 'get', 'set', 'delete'}
        if method_name in non_empty_methods and len(graph.nodes) == 0:
            return f"Error: {method_name}() requires a non-empty graph"
        
        return None
    
    def _handle_atomic_method_call(self, atomic_value: AtomicValue, method_name: str, arguments: List[str]) -> str:
        """Handle method calls on atomic values (with restricted methods)."""
        # Only allow conversion methods on atomic values
        allowed_atomic_methods = {
            'to_string': self._atomic_to_string,
            'to_num': self._atomic_to_num,  
            'to_bool': self._atomic_to_bool
        }
        
        if method_name not in allowed_atomic_methods:
            # Provide helpful error message for common graph methods
            graph_methods = ['append', 'prepend', 'insert', 'reverse', 'delete', 'get', 'set']
            if method_name in graph_methods:
                return f"Error: Cannot call {method_name}() on atomic value. Atomic values are immutable scalars, not collections."
            else:
                available = ', '.join(allowed_atomic_methods.keys())
                return f"Error: Method '{method_name}' not available on atomic values. Available: {available}"
        
        # Call the appropriate atomic method
        try:
            return allowed_atomic_methods[method_name](atomic_value, arguments)
        except Exception as e:
            return f"Error in {method_name}: {str(e)}"
    
    def _atomic_to_string(self, atomic_value: AtomicValue, arguments: List[str]) -> str:
        """Convert atomic value to string."""
        if arguments:
            return "Error: to_string() takes no arguments"
        return atomic_value.to_string()
    
    def _atomic_to_num(self, atomic_value: AtomicValue, arguments: List[str]) -> str:
        """Convert atomic value to number."""
        if arguments:
            return "Error: to_num() takes no arguments"
        try:
            result = atomic_value.to_num()
            return str(result)
        except ValueError as e:
            return f"Error: {str(e)}"
    
    def _atomic_to_bool(self, atomic_value: AtomicValue, arguments: List[str]) -> str:
        """Convert atomic value to boolean."""
        if arguments:
            return "Error: to_bool() takes no arguments"
        result = atomic_value.to_bool()
        return 'true' if result else 'false'
    
    def _suggest_similar_methods(self, method_name: str) -> str:
        """Suggest similar method names when method not found."""
        all_method_names = list(self.all_methods.keys())
        
        # Simple similarity check - methods that start with the same letter
        similar = [m for m in all_method_names if m.startswith(method_name[0]) and m != method_name]
        
        if similar:
            suggestions = ', '.join(sorted(similar)[:5])  # Show up to 5 suggestions
            return f"Error: Method '{method_name}' not supported. Similar methods: {suggestions}"
        else:
            # Show available methods by category
            return f"Error: Method '{method_name}' not supported. Available methods:\\n" + self.list_available_methods()
    
    def list_available_methods(self) -> str:
        """List all available methods organized by category."""
        lines = []
        
        lines.append("Mutating methods (modify the graph):")
        for method in sorted(self.mutating_methods.keys()):
            lines.append(f"  {method}")
        
        lines.append("Query methods (read information):")
        for method in sorted(self.query_methods.keys()):
            lines.append(f"  {method}")
        
        lines.append("Conversion methods (change graph type):")
        for method in sorted(self.conversion_methods.keys()):
            lines.append(f"  {method}")
        
        return "\\n".join(lines)
    
    def get_method_help(self, method_name: str) -> str:
        """Get help for a specific method."""
        if method_name not in self.all_methods:
            return f"Method '{method_name}' not found"
        
        # Basic help - could be expanded with detailed descriptions
        if method_name in self.mutating_methods:
            category = "mutating (modifies graph)"
        elif method_name in self.query_methods:
            category = "query (reads information)"
        else:
            category = "conversion (changes graph type)"
        
        return f"{method_name}: {category} method"
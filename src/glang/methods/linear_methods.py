"""Methods specifically for linear graphs."""

from typing import List, Optional, Any
from ..core.graph import Graph


class LinearGraphMethods:
    """Methods that work specifically on linear graphs."""
    
    @staticmethod
    def append(graph: Graph, var_name: str, args: List[str]) -> str:
        """Handle append method for linear graphs."""
        if not graph.graph_type.is_linear():
            return f"Error: append() only works on linear graphs"
        if not args:
            return f"Error: append requires a value"
        
        # Apply type inference to the argument
        from ..parser.tokenizer import Tokenizer
        tokenizer = Tokenizer()
        
        # Join args in case of multi-word values
        raw_value = ' '.join(args)
        
        # Check if this is a list value from expression evaluation
        if raw_value.startswith('__LIST__'):
            # Extract the list from the special format
            import ast
            try:
                list_str = raw_value[8:]  # Remove '__LIST__' prefix
                typed_value = ast.literal_eval(list_str)
                
                # For append with a list, we should extend (add all elements) rather than append the list as one element
                # This matches the expected behavior described by the user
                if isinstance(typed_value, list):
                    # Extend the graph with all elements from the list
                    for item in typed_value:
                        # Check type constraint for each item if present
                        if hasattr(graph, 'metadata') and 'type_constraint' in graph.metadata:
                            constraint = graph.metadata['type_constraint']
                            item_type = tokenizer.get_value_type(item)
                            if item_type != constraint:
                                return f"Error: Cannot append {item_type} '{item}' to {constraint}-constrained list"
                        
                        graph.append(item)
                    
                    return f"Extended {var_name} with {len(typed_value)} elements"
                    
            except (ValueError, SyntaxError):
                # If parsing fails, fall back to treating as literal string
                typed_value = raw_value
        else:
            # Apply normal type inference
            typed_value = tokenizer.infer_value_type(raw_value)
        
        # Check type constraint if present
        if hasattr(graph, 'metadata') and 'type_constraint' in graph.metadata:
            constraint = graph.metadata['type_constraint']
            value_type = tokenizer.get_value_type(typed_value)
            if value_type != constraint:
                return f"Error: Cannot append '{raw_value}' (type: {value_type}) to {var_name} with constraint: {constraint}"
        
        graph.append(typed_value)
        return f"Appended '{raw_value}' to {var_name}"
    
    @staticmethod
    def prepend(graph: Graph, var_name: str, args: List[str]) -> str:
        """Handle prepend method for linear graphs."""
        if not graph.graph_type.is_linear():
            return f"Error: prepend() only works on linear graphs"
        if not args:
            return f"Error: prepend requires a value"
        
        # Apply type inference to the argument
        from ..parser.tokenizer import Tokenizer
        tokenizer = Tokenizer()
        
        raw_value = ' '.join(args)
        typed_value = tokenizer.infer_value_type(raw_value)
        
        # Check type constraint if present
        if hasattr(graph, 'metadata') and 'type_constraint' in graph.metadata:
            constraint = graph.metadata['type_constraint']
            value_type = tokenizer.get_value_type(typed_value)
            if value_type != constraint:
                return f"Error: Cannot prepend '{raw_value}' (type: {value_type}) to {var_name} with constraint: {constraint}"
        
        graph.prepend(typed_value)
        return f"Prepended '{raw_value}' to {var_name}"
    
    @staticmethod
    def insert(graph: Graph, var_name: str, args: List[str]) -> str:
        """Handle insert method for linear graphs."""
        if not graph.graph_type.is_linear():
            return f"Error: insert() only works on linear graphs"
        if len(args) < 2:
            return f"Error: insert requires an index and a value"
        
        try:
            index = int(args[0])
            
            # Apply type inference to the value
            from ..parser.tokenizer import Tokenizer
            tokenizer = Tokenizer()
            
            raw_value = ' '.join(args[1:])
            typed_value = tokenizer.infer_value_type(raw_value)
            
            # Check type constraint if present
            if hasattr(graph, 'metadata') and 'type_constraint' in graph.metadata:
                constraint = graph.metadata['type_constraint']
                value_type = tokenizer.get_value_type(typed_value)
                if value_type != constraint:
                    return f"Error: Cannot insert '{raw_value}' (type: {value_type}) to {var_name} with constraint: {constraint}"
            
            graph.insert(index, typed_value)
            return f"Inserted '{raw_value}' at index {index} in {var_name}"
        except ValueError:
            return f"Error: First argument to insert must be a number"
        except IndexError as e:
            return f"Error: {str(e)}"
    
    @staticmethod
    def reverse(graph: Graph, var_name: str, args: List[str]) -> str:
        """Handle reverse method for linear graphs."""
        if not graph.graph_type.is_linear():
            return f"Error: reverse() only works on linear graphs"
        
        graph.reverse()
        return f"Reversed {var_name}"
    
    @staticmethod
    def delete_at(graph: Graph, var_name: str, args: List[str]) -> str:
        """Handle delete method for linear graphs."""
        if not graph.graph_type.is_linear():
            return f"Error: delete() only works on linear graphs"
        if not args:
            return f"Error: delete requires an index"
        
        try:
            index = int(args[0])
            
            # Check bounds before attempting deletion
            if index < 0 or index >= len(graph.nodes):
                return f"Error: Index {index} out of range (graph has {len(graph.nodes)} elements)"
            
            # Perform deletion
            deleted_value = graph.delete(index)
            
            if deleted_value is not None:
                return f"Deleted '{deleted_value}' from index {index} in {var_name}"
            else:
                return f"Error: Failed to delete element at index {index}"
        except ValueError:
            return f"Error: Argument to delete must be a number"
        except IndexError as e:
            return f"Error: {str(e)}"
        except Exception as e:
            return f"Error in delete: {str(e)}"
    
    @staticmethod
    def get(graph: Graph, var_name: str, args: List[str]) -> str:
        """Handle get method for linear graphs."""
        if not graph.graph_type.is_linear():
            return f"Error: get() only works on linear graphs"
        if not args:
            return f"Error: get requires an index"
        
        try:
            index = int(args[0])
            value = graph.get(index)
            if value is not None:
                return str(value)
            else:
                return f"Error: Index {index} out of range"
        except ValueError:
            return f"Error: Argument to get must be a number"
    
    @staticmethod
    def set(graph: Graph, var_name: str, args: List[str]) -> str:
        """Handle set method for linear graphs."""
        if not graph.graph_type.is_linear():
            return f"Error: set() only works on linear graphs"
        if len(args) < 2:
            return f"Error: set requires an index and a value"
        
        try:
            index = int(args[0])
            value = ' '.join(args[1:])
            success = graph.set(index, value)
            if success:
                return f"Set index {index} to '{value}' in {var_name}"
            else:
                return f"Error: Index {index} out of range"
        except ValueError:
            return f"Error: First argument to set must be a number"
    
    @staticmethod
    def find(graph: Graph, var_name: str, args: List[str]) -> str:
        """Handle find method for linear graphs."""
        if not graph.graph_type.is_linear():
            return f"Error: find() only works on linear graphs"
        if not args:
            return f"Error: find requires a value to search for"
        
        value = ' '.join(args)
        index = graph.find(value)
        if index is not None:
            return str(index)
        else:
            return f"'{value}' not found in {var_name}"
    
    @staticmethod
    def find_all(graph: Graph, var_name: str, args: List[str]) -> str:
        """Handle find_all method for linear graphs."""
        if not graph.graph_type.is_linear():
            return f"Error: find_all() only works on linear graphs"
        if not args:
            return f"Error: find_all requires a value to search for"
        
        value = ' '.join(args)
        indices = graph.find_all(value)
        if indices:
            return f"Found '{value}' at indices: {', '.join(map(str, indices))}"
        else:
            return f"'{value}' not found in {var_name}"
    
    @staticmethod
    def count(graph: Graph, var_name: str, args: List[str]) -> str:
        """Handle count method for linear graphs."""
        if not graph.graph_type.is_linear():
            return f"Error: count() only works on linear graphs"
        if not args:
            return f"Error: count requires a value to count"
        
        value = ' '.join(args)
        count = graph.count(value)
        return f"'{value}' appears {count} time(s) in {var_name}"
    
    @staticmethod
    def slice(graph: Graph, var_name: str, args: List[str]) -> str:
        """Handle slice method for linear graphs."""
        if not graph.graph_type.is_linear():
            return f"Error: slice() only works on linear graphs"
        if not args:
            return f"Error: slice requires start index"
        
        try:
            start = int(args[0])
            stop = int(args[1]) if len(args) > 1 else None
            step = int(args[2]) if len(args) > 2 else 1
            
            sliced_graph = graph.slice(start, stop, step)
            data = sliced_graph.to_list()
            return f"Slice result: {data}"
        except ValueError:
            return f"Error: slice arguments must be numbers"
        except Exception as e:
            return f"Error: {str(e)}"
    
    @staticmethod
    def types(graph: Graph, var_name: str, args: List[str]) -> str:
        """Show the types of all elements in the linear graph."""
        if not graph.graph_type.is_linear():
            return f"Error: types() only works on linear graphs"
        
        if graph._size == 0:
            return "[]"
        
        type_names = []
        current = graph._head
        while current is not None:
            data_type = type(current.data).__name__
            # Map Python types to glang type names
            type_map = {
                'int': 'num',
                'float': 'num', 
                'str': 'string',
                'bool': 'bool'
            }
            glang_type = type_map.get(data_type, data_type)
            type_names.append(glang_type)
            
            # Move to next node
            successors = current.get_successors()
            current = next(iter(successors)) if successors else None
        
        return f"[{', '.join(type_names)}]"
    
    @staticmethod
    def typeof(graph: Graph, var_name: str, args: List[str]) -> str:
        """Get the type of a specific element by index."""
        if not graph.graph_type.is_linear():
            return f"Error: typeof() only works on linear graphs"
        if not args:
            return f"Error: typeof requires an index"
        
        try:
            index = int(args[0])
            
            # Handle negative indexing
            if index < 0:
                index = graph._size + index
            
            # Check bounds
            if index < 0 or index >= graph._size:
                return f"Error: Index {args[0]} out of range (graph has {graph._size} elements)"
            
            # Traverse to the correct node
            current = graph._head
            for _ in range(index):
                if current is None:
                    return f"Error: Could not traverse to index {index}"
                successors = current.get_successors()
                current = next(iter(successors)) if successors else None
            
            if current is None:
                return f"Error: Could not access element at index {index}"
            
            # Get the type
            data_type = type(current.data).__name__
            
            # Map Python types to glang type names
            type_map = {
                'int': 'num',
                'float': 'num', 
                'str': 'string',
                'bool': 'bool'
            }
            glang_type = type_map.get(data_type, data_type)
            return glang_type
            
        except ValueError:
            return f"Error: Argument to typeof must be a number"
        except Exception as e:
            return f"Error: {str(e)}"
    
    @staticmethod
    def constraint(graph: Graph, var_name: str, args: List[str]) -> str:
        """Get the type constraint of the graph if it has one."""
        if hasattr(graph, 'metadata') and 'type_constraint' in graph.metadata:
            constraint = graph.metadata['type_constraint']
            return f"{var_name} has type constraint: {constraint}"
        else:
            return f"{var_name} has no type constraint"
    
    @staticmethod
    def validate_constraint(graph: Graph, var_name: str, args: List[str]) -> str:
        """Validate that all elements match the type constraint."""
        if not hasattr(graph, 'metadata') or 'type_constraint' not in graph.metadata:
            return f"{var_name} has no type constraint to validate"
        
        constraint = graph.metadata['type_constraint']
        
        if graph._size == 0:
            return f"{var_name} is empty - constraint validation passed"
        
        from ..parser.tokenizer import Tokenizer
        tokenizer = Tokenizer()
        
        # Validate each element
        violations = []
        current = graph._head
        index = 0
        
        while current is not None:
            value_type = tokenizer.get_value_type(current.data)
            if value_type != constraint:
                violations.append(f"index {index}: {current.data} (type: {value_type})")
            
            # Move to next node
            successors = current.get_successors()
            current = next(iter(successors)) if successors else None
            index += 1
        
        if violations:
            return f"Type constraint violations for {constraint}:\\n" + "\\n".join(violations)
        else:
            return f"All elements in {var_name} satisfy constraint: {constraint}"
    
    @staticmethod
    def type_summary(graph: Graph, var_name: str, args: List[str]) -> str:
        """Get a summary of types in the graph."""
        if not graph.graph_type.is_linear():
            return f"Error: type_summary() only works on linear graphs"
        
        if graph._size == 0:
            return f"{var_name}: empty list"
        
        from ..parser.tokenizer import Tokenizer
        tokenizer = Tokenizer()
        
        # Count types
        type_counts = {}
        current = graph._head
        
        while current is not None:
            value_type = tokenizer.get_value_type(current.data)
            type_counts[value_type] = type_counts.get(value_type, 0) + 1
            
            # Move to next node
            successors = current.get_successors()
            current = next(iter(successors)) if successors else None
        
        # Format summary
        summary_parts = []
        for type_name, count in sorted(type_counts.items()):
            summary_parts.append(f"{count} {type_name}")
        
        constraint_info = ""
        if hasattr(graph, 'metadata') and 'type_constraint' in graph.metadata:
            constraint_info = f" (constraint: {graph.metadata['type_constraint']})"
        
        return f"{var_name}: {', '.join(summary_parts)}{constraint_info}"
    
    @staticmethod
    def coerce_to_constraint(graph: Graph, var_name: str, args: List[str]) -> str:
        """Attempt to coerce all values in the graph to match the type constraint."""
        if not hasattr(graph, 'metadata') or 'type_constraint' not in graph.metadata:
            return f"Error: {var_name} has no type constraint to coerce to"
        
        constraint = graph.metadata['type_constraint']
        
        if graph._size == 0:
            return f"{var_name} is empty - no coercion needed"
        
        from ..parser.tokenizer import Tokenizer
        tokenizer = Tokenizer()
        
        # Collect all nodes that need coercion
        coercion_results = []
        current = graph._head
        index = 0
        
        while current is not None:
            current_type = tokenizer.get_value_type(current.data)
            if current_type != constraint:
                success, coerced_value, error_msg = tokenizer.coerce_to_type(current.data, constraint)
                if success:
                    current.data = coerced_value
                    coercion_results.append(f"index {index}: {current_type} -> {constraint}")
                else:
                    coercion_results.append(f"index {index}: FAILED - {error_msg}")
            
            # Move to next node
            successors = current.get_successors()
            current = next(iter(successors)) if successors else None
            index += 1
        
        if not coercion_results:
            return f"All values in {var_name} already match constraint: {constraint}"
        else:
            return f"Coercion results for {var_name}:\\n" + "\\n".join(coercion_results)
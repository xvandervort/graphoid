"""Expression evaluator for glang assignment expressions."""

import re
from typing import Any, Optional
from ..core import AtomicValue


class ExpressionEvaluator:
    """Evaluates expressions in glang, particularly for assignment operations."""
    
    def __init__(self, graph_manager):
        self.graph_manager = graph_manager
    
    def evaluate_expression(self, expression: str, allow_multi_element: bool = False) -> Any:
        """Evaluate an expression and return its value.
        
        Args:
            expression: The expression to evaluate (e.g., "a[0]", "matrix[1][2]")
            
        Returns:
            The evaluated value
            
        Raises:
            ValueError: If the expression is invalid or cannot be evaluated
        """
        expression = expression.strip()
        
        # Check if it's an index access expression
        if self._is_index_expression(expression):
            return self._evaluate_index_expression(expression)
        
        # Check if it's a simple variable reference
        if self._is_simple_variable(expression):
            return self._evaluate_variable_reference(expression, allow_multi_element)
        
        # If none of the above, it's a literal value
        return expression
    
    def _is_index_expression(self, expression: str) -> bool:
        """Check if expression is an index access like a[0] or matrix[1][2]."""
        # Pattern: valid_identifier followed by one or more [index] patterns
        pattern = r'^[a-zA-Z_][a-zA-Z0-9_]*(\[[-]?\d+\])+$'
        return bool(re.match(pattern, expression))
    
    def _is_simple_variable(self, expression: str) -> bool:
        """Check if expression is a simple variable reference."""
        if not expression:
            return False
        
        # Must start with letter or underscore
        if not (expression[0].isalpha() or expression[0] == '_'):
            return False
        
        # Rest must be alphanumeric or underscore
        return all(c.isalnum() or c == '_' for c in expression[1:])
    
    def _evaluate_index_expression(self, expression: str) -> Any:
        """Evaluate an index expression like a[0] or matrix[1][2]."""
        # Parse the expression to extract variable name and indices
        pattern = r'^(\w+)((?:\[[-]?\d+\])+)$'
        match = re.match(pattern, expression)
        
        if not match:
            raise ValueError(f"Invalid index expression: {expression}")
        
        variable_name = match.group(1)
        indices_str = match.group(2)
        
        # Extract all indices
        indices = []
        index_pattern = r'\[([-]?\d+)\]'
        for match in re.finditer(index_pattern, indices_str):
            indices.append(int(match.group(1)))
        
        # Get the variable
        variable = self.graph_manager.get_variable(variable_name)
        if variable is None:
            raise ValueError(f"Variable '{variable_name}' not found")
        
        # Handle AtomicValue (scalars cannot be indexed)
        if isinstance(variable, AtomicValue):
            raise ValueError(f"Cannot index into atomic value '{variable_name}'")
        
        # Navigate through the indices
        current_data = variable
        
        for i, index in enumerate(indices):
            if i == 0:
                # First access - use the graph's get method
                if not variable.graph_type.is_linear():
                    raise ValueError(f"Indexing only works on linear graphs (current: {variable.graph_type.name})")
                
                # Handle negative indexing
                if index < 0:
                    index = variable._size + index
                
                # Check bounds
                if index < 0 or index >= variable._size:
                    raise ValueError(f"Index {indices[i]} out of range (graph has {variable._size} elements)")
                
                # Get the value from the graph
                current_data = variable.get(index)
                if current_data is None:
                    raise ValueError(f"Could not get element at index {index}")
            else:
                # Subsequent accesses - treat as list indexing
                if not isinstance(current_data, list):
                    raise ValueError(f"Cannot index into non-list type: {type(current_data).__name__}")
                
                # Handle negative indexing
                if index < 0:
                    index = len(current_data) + index
                
                # Check bounds
                if index < 0 or index >= len(current_data):
                    raise ValueError(f"Index {indices[i]} out of range (list has {len(current_data)} elements)")
                
                # Get the value from the list
                current_data = current_data[index]
        
        return current_data
    
    def _evaluate_variable_reference(self, variable_name: str, allow_multi_element: bool = False) -> Any:
        """Evaluate a simple variable reference."""
        variable = self.graph_manager.get_variable(variable_name)
        if variable is None:
            raise ValueError(f"Variable '{variable_name}' not found")
        
        # For AtomicValue, return the value directly
        if isinstance(variable, AtomicValue):
            return variable.value
        
        # For graphs, we need to determine what to return
        # For single-element graphs, return the element
        if variable._size == 1:
            return variable.get(0)
        
        # For multi-element graphs, check if we allow full list return
        if allow_multi_element:
            # Return the entire list as a Python list
            return variable.to_list()
        
        # For scalar assignments, multi-element graphs are ambiguous
        raise ValueError(f"Cannot assign multi-element graph '{variable_name}' to scalar variable")
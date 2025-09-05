"""Namespace serialization for saving glang programs to .gr files."""

from typing import Dict, List, TYPE_CHECKING
from datetime import datetime

from ..execution.values import GlangValue, StringValue, NumberValue, BooleanValue, ListValue

if TYPE_CHECKING:
    from ..execution import ExecutionSession


class NamespaceSerializer:
    """Serializes execution session namespace to .gr file format."""
    
    def serialize_namespace(self, execution_session: 'ExecutionSession') -> str:
        """
        Serialize the execution session namespace to .gr file format.
        
        Args:
            execution_session: Session with variables to serialize
            
        Returns:
            String content in .gr file format
        """
        lines = []
        
        # File header
        lines.append("# Glang program file")
        lines.append(f"# Generated on {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
        lines.append("")
        
        # Get all variables from the execution context
        variables = execution_session.execution_context.variables
        
        if not variables:
            lines.append("# No variables defined")
            lines.append("")
            return '\n'.join(lines)
        
        # Serialize each variable as a declaration
        lines.append("# Variable declarations")
        for var_name, value in variables.items():
            declaration = self._serialize_variable_declaration(var_name, value)
            lines.append(declaration)
        
        lines.append("")
        
        return '\n'.join(lines)
    
    def _serialize_variable_declaration(self, name: str, value: GlangValue) -> str:
        """
        Serialize a single variable as a declaration statement.
        
        Args:
            name: Variable name
            value: Variable value
            
        Returns:
            Declaration statement string
        """
        if isinstance(value, StringValue):
            return f'string {name} = "{value.value}"'
        
        elif isinstance(value, NumberValue):
            return f'num {name} = {value.value}'
        
        elif isinstance(value, BooleanValue):
            bool_str = "true" if value.value else "false"
            return f'bool {name} = {bool_str}'
        
        elif isinstance(value, ListValue):
            # Handle list serialization
            if value.constraint:
                list_type = f"list<{value.constraint}>"
            else:
                list_type = "list"
            
            elements = []
            for element in value.elements:
                if isinstance(element, StringValue):
                    elements.append(f'"{element.value}"')
                elif isinstance(element, NumberValue):
                    elements.append(str(element.value))
                elif isinstance(element, BooleanValue):
                    elements.append("true" if element.value else "false")
                else:
                    # For complex nested structures, use string representation
                    elements.append(f'"{element.to_display_string()}"')
            
            elements_str = ", ".join(elements)
            return f'{list_type} {name} = [{elements_str}]'
        
        else:
            # Fallback for unknown types
            return f'# Unknown type for variable {name}: {value.get_type()}'
    
    def generate_example_file(self) -> str:
        """Generate an example .gr file for documentation."""
        lines = [
            "# Example Glang program file",
            "# This demonstrates the .gr file format",
            "",
            "# String variables",
            'string greeting = "Hello, World!"',
            'string name = "Alice"',
            "",
            "# Numeric variables", 
            "num age = 25",
            "num pi = 3.14159",
            "",
            "# Boolean variables",
            "bool is_active = true",
            "bool debug_mode = false",
            "",
            "# List variables",
            'list fruits = ["apple", "banana", "cherry"]',
            "list numbers = [1, 2, 3, 4, 5]",
            'list<string> names = ["Alice", "Bob", "Charlie"]',
            "list<num> scores = [95, 87, 92]",
            "",
            "# Comments are preserved and ignored during execution",
            "# You can use type inference too:",
            'title = "Engineer"  # Inferred as string',
            "count = 10         # Inferred as num",
            "active = true      # Inferred as bool",
            "",
            "# Method calls (executed when file is loaded)",
            "fruits.append \"orange\"",
            "scores.append 100",
            ""
        ]
        
        return '\n'.join(lines)
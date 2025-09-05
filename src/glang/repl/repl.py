"""
Core REPL implementation for Glang.
"""

import sys
import os
from typing import Dict, Callable, Optional, List
from glang import __version__, __description__
from .graph_manager import GraphManager
from ..parser import SyntaxParser, ExpressionEvaluator, InputType, LegacyIndexAccess, LegacyIndexAssignment, LegacySliceAccess, LegacySliceAssignment
from ..display import GraphRenderer, DisplayMode
from ..methods import MethodDispatcher
from ..execution import ExecutionSession, ExecutionPipeline, ExecutionResult

# Try to import readline for command history and navigation
try:
    import readline
    READLINE_AVAILABLE = True
except ImportError:
    READLINE_AVAILABLE = False


class REPL:
    """
    Read-Eval-Print Loop for the Glang programming language.
    """
    
    def __init__(self) -> None:
        self.prompt = "glang> "
        self.graph_manager = GraphManager()
        self.syntax_parser = SyntaxParser()
        self.expression_evaluator = ExpressionEvaluator(self.graph_manager)
        self.renderer = GraphRenderer(self.graph_manager)
        self.method_dispatcher = MethodDispatcher(self.graph_manager)
        
        # NEW: AST-based execution system
        self.execution_session = ExecutionSession()
        
        self.commands: Dict[str, Callable[[], Optional[bool]]] = {
            "ver": self._version_command,
            "version": self._version_command,
            "h": self._help_command,
            "help": self._help_command,
            "x": self._exit_command,
            "exit": self._exit_command,
        }
        self.running = True
        self.command_history: List[str] = []
        self._setup_readline()
    
    def start(self) -> None:
        """Start the REPL session."""
        print(f"Glang {__version__}")
        print(__description__)
        print("Type 'help' for available commands or 'exit' to quit.")
        print("✨ Try: create fruits [apple, banana] then 'namespace' to see the magic! ✨")
        print()
        
        while self.running:
            try:
                user_input = input(self.prompt).strip()
                if user_input:
                    # Add to history if using our own tracking
                    if user_input not in self.command_history[-1:]:  # Avoid duplicate consecutive commands
                        self.command_history.append(user_input)
                    self._process_input(user_input)
            except (KeyboardInterrupt, EOFError):
                print("\nGoodbye!")
                break
        
        # Save history when exiting
        self._save_history()
    
    def _process_input(self, user_input: str) -> None:
        """Process user input and execute appropriate command."""
        
        # Try AST-based execution first
        try:
            result = self.execution_session.execute_statement(user_input)
            
            if result.success:
                # Display result if there's a meaningful value
                if result.value is not None:
                    print(result.value)
                return
            else:
                # If AST execution fails, check if it's a legacy command
                if user_input.startswith('/'):
                    self._handle_legacy_command_fallback(user_input)
                    return
                else:
                    # Show the AST execution error
                    print(f"Error: {result.error}")
                    return
                    
        except Exception as e:
            # If AST execution completely fails, fall back to legacy parsing
            pass
        
        # FALLBACK: Legacy syntax parser for compatibility
        try:
            parsed = self.syntax_parser.parse_input(user_input)
        except Exception as e:
            print(f"Syntax error: {e}")
            return
        
        # Handle based on input type
        if parsed.input_type == InputType.VARIABLE_DECLARATION:
            self._handle_variable_declaration(parsed)
        elif parsed.input_type == InputType.METHOD_CALL:
            self._handle_method_call(parsed)
        elif parsed.input_type == InputType.VARIABLE_ACCESS:
            self._handle_variable_access(parsed)
        elif parsed.input_type == InputType.INDEX_ACCESS:
            self._handle_index_access(parsed)
        elif parsed.input_type == InputType.INDEX_ASSIGNMENT:
            self._handle_index_assignment(parsed)
        elif parsed.input_type == InputType.SLICE_ACCESS:
            self._handle_slice_access(parsed)
        elif parsed.input_type == InputType.SLICE_ASSIGNMENT:
            self._handle_slice_assignment(parsed)
        elif parsed.input_type == InputType.LEGACY_COMMAND:
            self._handle_legacy_command(parsed)
    
    def _handle_legacy_command_fallback(self, user_input: str) -> None:
        """Handle legacy commands when AST execution fails."""
        # Remove leading slash and parse as legacy command
        command_str = user_input[1:] if user_input.startswith('/') else user_input
        
        # Split into command and arguments
        parts = command_str.split()
        if not parts:
            return
            
        command = parts[0]
        args = parts[1:] if len(parts) > 1 else []
        
        # First check simple commands
        if command in self.commands:
            result = self.commands[command]()
            if result is False:  # Explicit exit request
                self.running = False
            return
        
        # Handle legacy commands directly
        if command == "graphs":
            print(self.graph_manager.list_graphs())
        elif command == "namespace":
            print(self.graph_manager.show_variable_graph())
        elif command == "stats":
            print(self.graph_manager.get_variable_stats())
        else:
            print(f"Unknown command: /{command}")
            print("Type '/help' for available commands.")
    
    def _version_command(self) -> None:
        """Display version information."""
        print(f"Glang version {__version__}")
        print(__description__)
    
    def _help_command(self) -> None:
        """Display help information."""
        print("Available commands:")
        print("  ver, version       - Show version information")
        print("  h, help            - Show this help message")
        print("  x, exit            - Exit the REPL")
        print()
        print("Command system:")
        print("  /command args      - All commands MUST use slash prefix")
        print("  variable_name      - Variable access (no prefix)")
        print("  Examples:")
        print("    list h = [1, 2]  # Creates variable 'h'")
        print("    h                # Shows variable 'h' contents")
        print("    /h or /help      # Shows this help (slash required)")
        print("    help             # ERROR - creates/accesses variable 'help'")
        print()
        print("Graph commands:")
        print("  /create <name> [1,2,3]  - Create graph from list (legacy)")
        print("  list <name> = [1,2,3]   - Create list graph (NEW!)")
        print("  <name>                  - Show graph contents (NEW!)")
        print("  <name> --info           - Show detailed variable info (NEW!)")
        print("  <name> --show-nodes     - Show with node details (NEW!)")
        print("  <name> --json           - Show as JSON format (NEW!)")
        print("  <name> --compact        - Compact detailed view (NEW!)")
        print("  <name>.<method> <args>  - Call method on graph (NEW!)")
        print("  <name>[index]           - Access element by index (NEW!)")
        print("  /graphs                 - List all graphs")
        print("  /show [name]            - Show graph structure")
        print("  /traverse [name]        - Show graph traversal")
        print("  /delete <name>          - Delete a graph")
        print("  /info [name]            - Show detailed variable info")
        print()
        print("Variable namespace (meta-graph):")
        print("  /namespace            - Show the variable graph itself")
        print("  /stats                - Show namespace statistics")
        print()
        print("Graph operations (on current/specified graph):")
        print("  /append <value>       - Add to end")
        print("  /prepend <value>      - Add to beginning")
        print("  /insert <index> <val> - Insert at position")
        print("  /reverse              - Reverse the graph")
        print()
        print("Examples:")
        print("  list fruits = [apple, banana, cherry]  # NEW syntax!")
        print("  fruits                     # Show contents")
        print("  fruits.append orange       # Method call")
        print("  /create fruits [apple, banana, cherry]  # Legacy syntax")
        print("  /show fruits               # Legacy show")
        print("  /append orange             # Legacy append")
        print("  /namespace                 # 🤯 Show the variable graph itself!")
        print("  /stats                     # Meta-graph statistics")
        print("  /create numbers [1, 2, 3]")
        print("  /namespace                 # Now see TWO variables in the meta-graph")
        print()
        print("🔍 The '/namespace' command shows how variables are stored as a GRAPH!")
        print()
        if READLINE_AVAILABLE:
            print("Navigation:")
            print("  ↑/↓ arrows    - Command history")
            print("  ←/→ arrows    - Cursor movement")  
            print("  Tab           - Auto-completion")
            print("  Ctrl+C        - Interrupt")
            print()
        print("Data types (auto-detected):")
        print("  Numbers: 42, 3.14 → num")
        print("  Booleans: true, false → bool")  
        print("  Strings: hello, 'quoted text' → string")
        print()
        print("Type introspection methods:")
        print("  <list>.types()         - Show types of all elements")
        print("  <list>.typeof <index>  - Show type of specific element")
        print()
        print("Glang is a prototype programming language with graphs as first-class objects.")
    
    def _exit_command(self) -> bool:
        """Exit the REPL."""
        print("Goodbye!")
        self._save_history()  # Save history on explicit exit too
        return False  # Signal to stop the REPL
    
    def _handle_create_command(self, args: List[str], original_input: str) -> None:
        """Handle create command."""
        if len(args) < 2:
            print("Usage: create <name> [list] or create <name> <type>")
            print("Examples:")
            print("  create nums [1, 2, 3]")
            print("  create empty linear")
            return
        
        name = args[0]
        
        # Check if second argument looks like a list
        list_part = " ".join(args[1:])
        parsed_list = self.graph_manager.parse_list_syntax(list_part)
        
        if parsed_list is not None:
            # Create from list
            result = self.graph_manager.create_from_list(name, parsed_list)
            print(result)
        else:
            # Create empty graph of specified type
            try:
                from glang.core import GraphType
                graph_type = GraphType.from_string(args[1])
                result = self.graph_manager.create_graph(name, graph_type)
                print(result)
            except ValueError:
                print(f"Unknown graph type: {args[1]}")
                print("Available types: linear, tree, directed, cyclic, weighted, undirected")
    
    def _handle_show_command(self, args: List[str]) -> None:
        """Handle show command."""
        name = args[0] if args else None
        result = self.graph_manager.show_graph(name)
        print(result)
    
    def _handle_traverse_command(self, args: List[str]) -> None:
        """Handle traverse command."""
        name = args[0] if args else None
        result = self.graph_manager.traverse_graph(name)
        print(result)
    
    def _handle_delete_command(self, args: List[str]) -> None:
        """Handle delete command."""
        if not args:
            print("Usage: delete <graph_name>")
            return
        
        result = self.graph_manager.delete_graph(args[0])
        print(result)
    
    def _handle_info_command(self, args: List[str]) -> None:
        """Handle info command."""
        name = args[0] if args else None
        result = self.graph_manager.get_variable_info(name)
        print(result)
    
    def _handle_graph_operation(self, operation: str, args: List[str]) -> None:
        """Handle graph operations like append, prepend, etc."""
        current_graph = self.graph_manager.current_graph
        if not current_graph:
            print("No current graph. Create a graph first with 'create <name> [data]'")
            return
        
        # Map delete_at to delete for the operation
        op_name = "delete" if operation == "delete_at" else operation
        
        result = self.graph_manager.execute_graph_operation(current_graph, op_name, *args)
        print(result)
    
    def _setup_readline(self) -> None:
        """Set up readline for command history and tab completion."""
        if not READLINE_AVAILABLE:
            return
        
        # Set up history file
        history_file = os.path.expanduser("~/.glang_history")
        try:
            readline.read_history_file(history_file)
        except FileNotFoundError:
            pass  # History file doesn't exist yet
        except Exception:
            pass  # Ignore other readline setup issues
        
        # Set history length
        readline.set_history_length(1000)
        
        # Enable tab completion
        readline.set_completer(self._complete_command)
        readline.parse_and_bind("tab: complete")
        
        # Store history file path for saving later
        self._history_file = history_file
    
    def _complete_command(self, text: str, state: int) -> Optional[str]:
        """Tab completion for commands."""
        if state == 0:
            # First time - generate completions
            line = readline.get_line_buffer()
            self._completions = self._get_completions(text, line)
        
        try:
            return self._completions[state]
        except IndexError:
            return None
    
    def _get_completions(self, text: str, line: str) -> List[str]:
        """Get list of possible completions with enhanced syntax support."""
        # Check for method call pattern: variable.method
        if self._is_method_context(line):
            return self._get_method_completions(line, text)
        
        # Check for type constraint pattern: list<type>
        if self._is_type_constraint_context(line):
            return self._get_type_constraint_completions(line, text)
        
        # Check for variable declaration: list variable = or graph variable =
        if self._is_declaring_type(line):
            return self._get_graph_type_completions(text)
        
        # Standard completion logic
        parts = line.strip().split()
        
        if not parts or (len(parts) == 1 and not line.endswith(' ')):
            # Completing first word (command, graph type, or variable)
            all_commands = list(self.commands.keys()) + [
                "create", "show", "graphs", "traverse", "delete", "info",
                "namespace", "stats", "append", "prepend", "insert", "reverse"
            ]
            
            # Add graph types for new syntax
            graph_types = ["list", "graph", "tree", "directed", "weighted", "string", "num", "bool"]
            
            # Add existing variables for variable access
            variables = self.graph_manager.variable_graph.list_variables()
            
            all_options = all_commands + graph_types + variables
            
            # Filter based on what's being typed
            current_text = text if text else (parts[0] if parts else "")
            return [opt for opt in all_options if opt.startswith(current_text)]
        
        # Completing arguments for legacy commands
        cmd = parts[0].lower()
        
        if cmd in ["show", "traverse", "delete", "info"]:
            # Complete with variable names
            variables = self.graph_manager.variable_graph.list_variables()
            return [var for var in variables if var.startswith(text)]
        
        elif cmd == "create" and len(parts) >= 3:
            # Complete graph types for legacy create command
            graph_types = ["linear", "directed", "tree", "cyclic", "weighted", "undirected"]
            return [gt for gt in graph_types if gt.startswith(text)]
        
        return []
    
    def _is_method_context(self, line: str) -> bool:
        """Check if we're in a method call context (variable.method)."""
        return '.' in line and not line.strip().endswith('.')
    
    def _is_type_constraint_context(self, line: str) -> bool:
        """Check if we're in a type constraint context (list<type>)."""
        if '<' not in line or line.strip().endswith('>'):
            return False
        
        # Must have content before < (like 'list<' not just '<')
        angle_pos = line.find('<')
        return angle_pos > 0 and line[:angle_pos].strip()
    
    def _is_declaring_type(self, line: str) -> bool:
        """Check if we're declaring a graph type (beginning of line)."""
        parts = line.strip().split()
        if len(parts) != 1 or line.endswith(' '):
            return False
        
        # Only consider it type declaration if it starts with a graph type
        graph_types = ['list', 'graph', 'tree', 'directed', 'weighted', 'undirected']
        return any(gt.startswith(parts[0]) for gt in graph_types)
    
    def _get_method_completions(self, line: str, text: str) -> List[str]:
        """Get method completions for variable.method pattern."""
        # Extract variable name from line like "fruits.app"
        dot_pos = line.rfind('.')
        if dot_pos == -1:
            return []
        
        var_part = line[:dot_pos].strip().split()[-1]  # Get last word before dot
        
        # Get the variable's graph
        graph = self.graph_manager.get_variable(var_part)
        if not graph:
            return []
        
        # Get appropriate methods based on graph type
        methods = []
        if graph.graph_type.is_linear():
            methods = [
                'append', 'prepend', 'insert', 'reverse', 'delete', 
                'size', 'empty', 'types', 'constraint', 'validate_constraint',
                'type_summary', 'coerce_to_constraint'
            ]
        else:
            methods = ['size', 'empty', 'nodes', 'edges']
        
        # Filter methods that start with the text after the dot
        method_text = text if text else line[dot_pos + 1:]
        return [method for method in methods if method.startswith(method_text)]
    
    def _get_type_constraint_completions(self, line: str, text: str) -> List[str]:
        """Get type constraint completions for list<type> pattern."""
        # Find the position of < to extract what's being typed
        angle_pos = line.rfind('<')
        if angle_pos == -1:
            return []
        
        # Available type constraints
        type_constraints = ['num', 'string', 'bool', 'list']
        
        # Get the text after <
        constraint_text = text if text else line[angle_pos + 1:]
        return [tc for tc in type_constraints if tc.startswith(constraint_text)]
    
    def _get_graph_type_completions(self, text: str) -> List[str]:
        """Get graph type completions for variable declarations."""
        graph_types = ['list', 'graph', 'tree', 'directed', 'weighted', 'undirected', 'string', 'num', 'bool']
        return [gt for gt in graph_types if gt.startswith(text)]
    
    def _save_history(self) -> None:
        """Save command history to file."""
        if READLINE_AVAILABLE and hasattr(self, '_history_file'):
            try:
                readline.write_history_file(self._history_file)
            except Exception:
                pass  # Ignore history save issues
    
    def _handle_variable_declaration(self, parsed) -> None:
        """Handle new-style variable declarations."""
        # Support both collection types (list) and scalar types (string, num, bool)
        if parsed.graph_type == 'list':
            self._handle_list_declaration(parsed)
        elif parsed.graph_type in ['string', 'num', 'bool']:
            self._handle_scalar_declaration(parsed)
        else:
            print(f"Graph type '{parsed.graph_type}' not yet implemented")
            return
    
    def _handle_list_declaration(self, parsed) -> None:
        """Handle list variable declarations."""
        if parsed.initializer is None:
            # Empty list
            parsed.initializer = []
        
        # Validate type constraints if specified
        if parsed.type_constraint:
            validation_result = self._validate_type_constraint(parsed.initializer, parsed.type_constraint)
            if not validation_result[0]:
                print(f"Type validation failed: {validation_result[1]}")
                return
        
        # Resolve any IDENTIFIER tokens in the initializer to actual variables
        try:
            resolved_initializer = self._resolve_identifiers(parsed.initializer)
        except ValueError as e:
            print(f"Error: {e}")
            return
        
        result = self.graph_manager.create_from_list(parsed.variable_name, resolved_initializer)
        
        # Store type constraint metadata if specified
        if parsed.type_constraint:
            graph = self.graph_manager.get_variable(parsed.variable_name)
            if graph is not None:
                # Add type constraint as metadata
                if not hasattr(graph, 'metadata'):
                    graph.metadata = {}
                graph.metadata['type_constraint'] = parsed.type_constraint
        
        print(result)
    
    def _handle_scalar_declaration(self, parsed) -> None:
        """Handle scalar variable declarations (string, num, bool)."""
        if parsed.initializer is None:
            print(f"Scalar variable '{parsed.variable_name}' requires an initializer")
            return
        
        # For scalar declarations, the initializer should be a single value
        if isinstance(parsed.initializer, list) and len(parsed.initializer) == 1:
            raw_value = parsed.initializer[0]
        elif isinstance(parsed.initializer, list):
            print(f"Scalar variable '{parsed.variable_name}' cannot be initialized with a list")
            return
        else:
            raw_value = parsed.initializer
        
        # Evaluate the initializer expression
        try:
            if isinstance(raw_value, dict) and raw_value.get('type') == 'IDENTIFIER':
                # This is a variable reference - extract the name and evaluate
                var_name = raw_value['name']
                value = self.expression_evaluator.evaluate_expression(var_name)
            elif isinstance(raw_value, str):
                # This might be an expression or a literal string
                # Check if it looks like an index expression
                if self.expression_evaluator._is_index_expression(raw_value):
                    # Definitely an index expression - evaluate it (errors should propagate)
                    value = self.expression_evaluator.evaluate_expression(raw_value)
                else:
                    # Try to evaluate as simple variable or treat as literal string
                    try:
                        value = self.expression_evaluator.evaluate_expression(raw_value)
                    except ValueError:
                        # If evaluation fails, treat as literal string
                        value = raw_value
            else:
                # Already a typed value (number, boolean, etc.)
                value = raw_value
                
        except Exception as e:
            print(f"Error: {e}")
            return
        
        # Type validation
        expected_type = parsed.graph_type
        try:
            if not self._validate_scalar_type(value, expected_type):
                print(f"Error: Value '{value}' is not compatible with type '{expected_type}'")
                return
        except Exception as e:
            print(f"Error during type validation: {e}")
            print(f"Value: {value}, type: {type(value)}, expected: {expected_type}")
            return
        
        # Create as AtomicValue (proper scalar storage)
        result = self.graph_manager.create_atomic_value(parsed.variable_name, value, expected_type)
        print(result)
    
    def _validate_scalar_type(self, value, expected_type: str) -> bool:
        """Validate that a value matches the expected scalar type."""
        if expected_type == 'string':
            return isinstance(value, str)
        elif expected_type == 'num':
            return isinstance(value, (int, float))
        elif expected_type == 'bool':
            return isinstance(value, bool)
        return False
    
    def _format_scalar_value(self, value) -> str:
        """Format scalar value for display."""
        if isinstance(value, str):
            return f"'{value}'"
        elif isinstance(value, bool):
            return 'true' if value else 'false'
        return str(value)
    
    def _resolve_identifiers(self, data_list: List) -> List:
        """Resolve IDENTIFIER tokens to actual variable values."""
        if not data_list:
            return data_list
        
        resolved = []
        for item in data_list:
            if isinstance(item, dict) and item.get('type') == 'IDENTIFIER':
                # This is an identifier that needs to be resolved
                var_name = item['name']
                graph = self.graph_manager.get_variable(var_name)
                if graph is None:
                    raise ValueError(f"Variable '{var_name}' not found. Use quotes for strings.")
                
                # For linear graphs, get the data in order
                if graph.graph_type.is_linear():
                    data_values = []
                    current = graph._head
                    while current:
                        data_values.append(current.data)
                        successors = current.get_successors()
                        current = next(iter(successors)) if successors else None
                    
                    # If single value, include it directly; otherwise as a list
                    if len(data_values) == 1:
                        resolved.append(data_values[0])
                    else:
                        resolved.append(data_values)
                else:
                    # For non-linear graphs, get all node data
                    node_data = [node.data for node in graph.nodes()]
                    if len(node_data) == 1:
                        resolved.append(node_data[0])
                    else:
                        resolved.append(node_data)
            elif isinstance(item, list):
                # Recursively resolve nested lists
                resolved.append(self._resolve_identifiers(item))
            else:
                # Regular value (number, string, boolean)
                resolved.append(item)
        
        return resolved
    
    def _validate_type_constraint(self, initializer, type_constraint) -> tuple[bool, str]:
        """Validate that initializer values match the type constraint.
        
        Returns:
            tuple: (is_valid, error_message)
        """
        if not initializer:
            return True, ""  # Empty lists are always valid
        
        for i, value in enumerate(initializer):
            value_type = self.syntax_parser.tokenizer.get_value_type(value)
            
            # Handle nested lists
            if isinstance(value, list) and type_constraint != 'list':
                return False, f"Element at index {i} is a list, but constraint requires '{type_constraint}'"
            elif isinstance(value, list) and type_constraint == 'list':
                continue  # Nested lists are valid for list constraint
            elif value_type != type_constraint:
                return False, f"Element at index {i} ('{value}') is type '{value_type}', but constraint requires '{type_constraint}'"
        
        return True, ""
    
    def _handle_method_call(self, parsed) -> None:
        """Handle method calls on variables."""
        result = self.method_dispatcher.dispatch_method(
            parsed.variable_name,
            parsed.method_name,
            parsed.arguments
        )
        print(result)
    
    def _handle_variable_access(self, parsed) -> None:
        """Handle variable access (display contents)."""
        from ..core import AtomicValue
        
        variable = self.graph_manager.get_variable(parsed.variable_name)
        if variable is None:
            print(f"Variable '{parsed.variable_name}' not found")
            return
        
        # Handle AtomicValue display
        if isinstance(variable, AtomicValue):
            if '--info' in parsed.flags:
                print(f"AtomicValue: {variable.atomic_type} = {variable}")
            else:
                print(str(variable))
            return
        
        # Handle Graph display (existing logic)
        graph = variable
        
        # Determine display mode from flags
        mode = DisplayMode.SIMPLE
        
        if '--info' in parsed.flags:
            mode = DisplayMode.META
        elif '--json' in parsed.flags:
            mode = DisplayMode.JSON
        elif '--show-nodes' in parsed.flags:
            mode = DisplayMode.DETAILED
        elif '--compact' in parsed.flags:
            mode = DisplayMode.COMPACT
        
        # Render with flags for additional options
        output = self.renderer.render(
            graph, 
            mode, 
            variable_name=parsed.variable_name,
            flags=parsed.flags
        )
        print(output)
        
        # Set as current graph for legacy operations
        self.graph_manager.set_current(parsed.variable_name)
    
    def _handle_index_access(self, parsed) -> None:
        """Handle index access (variable[index])."""
        graph = self.graph_manager.get_variable(parsed.variable_name)
        if graph is None:
            print(f"Variable '{parsed.variable_name}' not found")
            return
        
        try:
            # Support both single and multi-dimensional indexing
            current_data = graph
            current_indices = parsed.indices.copy()
            
            # Process each index in sequence
            for i, index in enumerate(current_indices):
                # For the first access, use the graph's get method
                if i == 0:
                    if not graph.graph_type.is_linear():
                        print(f"Indexing only works on linear graphs (current: {graph.graph_type.name})")
                        return
                    
                    # Handle negative indexing for the graph
                    if index < 0:
                        index = graph._size + index
                    
                    # Check bounds
                    if index < 0 or index >= graph._size:
                        print(f"Index {parsed.indices[i]} out of range (graph has {graph._size} elements)")
                        return
                    
                    # Get the value from the graph
                    current_data = graph.get(index)
                    if current_data is None:
                        print(f"Error: Could not get element at index {index}")
                        return
                else:
                    # For subsequent accesses, the current data should be a list
                    if not isinstance(current_data, list):
                        print(f"Cannot index into non-list type: {type(current_data).__name__}")
                        return
                    
                    # Handle negative indexing for the list
                    if index < 0:
                        index = len(current_data) + index
                    
                    # Check bounds
                    if index < 0 or index >= len(current_data):
                        print(f"Index {parsed.indices[i]} out of range (list has {len(current_data)} elements)")
                        return
                    
                    # Get the value from the list
                    current_data = current_data[index]
            
            # Print the final result using consistent formatting
            from ..display.formatters import GraphFormatter
            print(GraphFormatter.format_value(current_data))
                
        except Exception as e:
            print(f"Error accessing index: {e}")
    
    def _handle_index_assignment(self, parsed) -> None:
        """Handle index assignment (variable[index] = value)."""
        graph = self.graph_manager.get_variable(parsed.variable_name)
        if graph is None:
            print(f"Variable '{parsed.variable_name}' not found")
            return
        
        try:
            # Support both single and multi-dimensional assignment
            current_data = graph
            indices = parsed.indices.copy()
            
            # Navigate to the location where we want to assign
            for i, index in enumerate(indices):
                if i == len(indices) - 1:
                    # This is the final index - perform the assignment
                    if i == 0:
                        # Single-level assignment to graph
                        if not graph.graph_type.is_linear():
                            print(f"Assignment only works on linear graphs (current: {graph.graph_type.name})")
                            return
                        
                        # Handle negative indexing for the graph
                        if index < 0:
                            index = graph._size + index
                        
                        # Check bounds
                        if index < 0 or index >= graph._size:
                            print(f"Index {parsed.indices[i]} out of range (graph has {graph._size} elements)")
                            return
                        
                        # Set the value in the graph
                        success = graph.set(index, parsed.value)
                        if success:
                            print(f"Set {parsed.variable_name}[{parsed.indices[i]}] = {parsed.value}")
                        else:
                            print(f"Error: Could not set element at index {index}")
                    else:
                        # Multi-level assignment to nested list
                        if not isinstance(current_data, list):
                            print(f"Cannot assign to index in non-list type: {type(current_data).__name__}")
                            return
                        
                        # Handle negative indexing for the list
                        if index < 0:
                            index = len(current_data) + index
                        
                        # Check bounds
                        if index < 0 or index >= len(current_data):
                            print(f"Index {parsed.indices[i]} out of range (list has {len(current_data)} elements)")
                            return
                        
                        # Assign to the list
                        current_data[index] = parsed.value
                        print(f"Set {parsed.variable_name}[{']['.join(map(str, parsed.indices))}] = {parsed.value}")
                else:
                    # Navigate deeper into the structure
                    if i == 0:
                        # First level - get from graph
                        if not graph.graph_type.is_linear():
                            print(f"Indexing only works on linear graphs (current: {graph.graph_type.name})")
                            return
                        
                        # Handle negative indexing for the graph
                        if index < 0:
                            index = graph._size + index
                        
                        # Check bounds
                        if index < 0 or index >= graph._size:
                            print(f"Index {parsed.indices[i]} out of range (graph has {graph._size} elements)")
                            return
                        
                        # Get the value from the graph
                        current_data = graph.get(index)
                        if current_data is None:
                            print(f"Error: Could not get element at index {index}")
                            return
                    else:
                        # Subsequent levels - get from list
                        if not isinstance(current_data, list):
                            print(f"Cannot index into non-list type: {type(current_data).__name__}")
                            return
                        
                        # Handle negative indexing for the list
                        if index < 0:
                            index = len(current_data) + index
                        
                        # Check bounds
                        if index < 0 or index >= len(current_data):
                            print(f"Index {parsed.indices[i]} out of range (list has {len(current_data)} elements)")
                            return
                        
                        # Get the value from the list
                        current_data = current_data[index]
            
        except Exception as e:
            print(f"Error in assignment: {e}")
    
    def _handle_slice_access(self, parsed) -> None:
        """Handle slice access (variable[start:stop:step])."""
        graph = self.graph_manager.get_variable(parsed.variable_name)
        if graph is None:
            print(f"Variable '{parsed.variable_name}' not found")
            return
        
        try:
            if not graph.graph_type.is_linear():
                print(f"Slicing only works on linear graphs (current: {graph.graph_type.name})")
                return
            
            # Use Python slicing logic - convert graph to list, slice it, then display
            data_list = graph.to_list()
            
            # Handle None values for Python slice()
            start = parsed.start
            stop = parsed.stop
            step = parsed.step
            
            # Create slice object and apply it
            slice_obj = slice(start, stop, step)
            sliced_data = data_list[slice_obj]
            
            # Display the result using consistent formatting
            from ..display.formatters import GraphFormatter
            print(GraphFormatter.format_value(sliced_data))
            
        except Exception as e:
            print(f"Error in slicing: {e}")
    
    def _handle_slice_assignment(self, parsed) -> None:
        """Handle slice assignment (variable[start:stop:step] = values)."""
        graph = self.graph_manager.get_variable(parsed.variable_name)
        if graph is None:
            print(f"Variable '{parsed.variable_name}' not found")
            return
        
        try:
            if not graph.graph_type.is_linear():
                print(f"Slice assignment only works on linear graphs (current: {graph.graph_type.name})")
                return
            
            # Get the current data as a list
            data_list = graph.to_list()
            
            # Create slice object
            slice_obj = slice(parsed.start, parsed.stop, parsed.step)
            
            # Assign the new values to the slice
            if isinstance(parsed.value, list):
                data_list[slice_obj] = parsed.value
            else:
                # Single value assigned to slice - Python will repeat it
                data_list[slice_obj] = [parsed.value] * len(data_list[slice_obj])
            
            # Rebuild the graph from the modified list
            # Clear the graph and rebuild it
            graph.clear()
            for item in data_list:
                graph.append(item)
            
            # Show confirmation
            slice_notation = f"[{parsed.start or ''}:{parsed.stop or ''}"
            if parsed.step is not None:
                slice_notation += f":{parsed.step}"
            slice_notation += "]"
            
            print(f"Set {parsed.variable_name}{slice_notation} = {parsed.value}")
            
        except Exception as e:
            print(f"Error in slice assignment: {e}")
    
    def _handle_legacy_command(self, parsed) -> None:
        """Handle legacy command format."""
        command = parsed.command
        args = parsed.arguments
        
        # First check simple commands
        if command in self.commands:
            result = self.commands[command]()
            if result is False:  # Explicit exit request
                self.running = False
            return
        
        # Graph management commands
        if command == "create":
            self._handle_create_command(args, parsed.raw_input)
        elif command == "show":
            self._handle_show_command(args)
        elif command == "graphs":
            print(self.graph_manager.list_graphs())
        elif command == "traverse":
            self._handle_traverse_command(args)
        elif command == "delete":
            self._handle_delete_command(args)
        elif command == "info":
            self._handle_info_command(args)
        elif command == "namespace":
            print(self.graph_manager.show_variable_graph())
        elif command == "stats":
            print(self.graph_manager.get_variable_stats())
        # Graph operations
        elif command in ["append", "prepend", "insert", "delete_at", "reverse"]:
            self._handle_graph_operation(command, args)
        else:
            print(f"Unknown command: {parsed.raw_input}")
            print("Type 'help' for available commands.")
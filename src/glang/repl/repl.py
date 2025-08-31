"""
Core REPL implementation for Glang.
"""

import sys
import os
from typing import Dict, Callable, Optional, List
from glang import __version__, __description__
from .graph_manager import GraphManager
from ..parser import SyntaxParser, InputType
from ..display import GraphRenderer, DisplayMode
from ..methods import MethodDispatcher

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
        self.renderer = GraphRenderer(self.graph_manager)
        self.method_dispatcher = MethodDispatcher(self.graph_manager)
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
        # Parse the input using the syntax parser
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
        elif parsed.input_type == InputType.LEGACY_COMMAND:
            self._handle_legacy_command(parsed)
    
    def _version_command(self) -> None:
        """Display version information."""
        print(f"Glang version {__version__}")
        print(__description__)
    
    def _help_command(self) -> None:
        """Display help information."""
        print("Available commands:")
        print("  ver, version     - Show version information")
        print("  h, help          - Show this help message")
        print("  x, exit          - Exit the REPL")
        print()
        print("Graph commands:")
        print("  create <name> [1,2,3]  - Create graph from list (legacy)")
        print("  list <name> = [1,2,3]  - Create list graph (NEW!)")
        print("  <name>                 - Show graph contents (NEW!)")
        print("  <name> --info          - Show detailed variable info (NEW!)")
        print("  <name> --show-nodes    - Show with node details (NEW!)")
        print("  <name> --json          - Show as JSON format (NEW!)")
        print("  <name> --compact       - Compact detailed view (NEW!)")
        print("  <name>.<method> <args> - Call method on graph (NEW!)")
        print("  graphs                 - List all graphs")
        print("  show [name]           - Show graph structure")
        print("  traverse [name]       - Show graph traversal")
        print("  delete <name>         - Delete a graph")
        print("  info [name]           - Show detailed variable info")
        print()
        print("Variable namespace (meta-graph):")
        print("  namespace             - Show the variable graph itself")
        print("  stats                 - Show namespace statistics")
        print()
        print("Graph operations (on current/specified graph):")
        print("  append <value>        - Add to end")
        print("  prepend <value>       - Add to beginning")
        print("  insert <index> <val>  - Insert at position")
        print("  reverse               - Reverse the graph")
        print()
        print("Examples:")
        print("  list fruits = [apple, banana, cherry]  # NEW syntax!")
        print("  fruits                   # Show contents")
        print("  fruits.append orange     # Method call")
        print("  create fruits [apple, banana, cherry]  # Legacy syntax")
        print("  show fruits              # Legacy show")
        print("  append orange            # Legacy append")
        print("  namespace                # 🤯 Show the variable graph itself!")
        print("  stats                    # Meta-graph statistics")
        print("  create numbers [1, 2, 3]")
        print("  namespace                # Now see TWO variables in the meta-graph")
        print()
        print("🔍 The 'namespace' command shows how variables are stored as a GRAPH!")
        print()
        if READLINE_AVAILABLE:
            print("Navigation:")
            print("  ↑/↓ arrows    - Command history")
            print("  ←/→ arrows    - Cursor movement")  
            print("  Tab           - Auto-completion")
            print("  Ctrl+C        - Interrupt")
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
        """Get list of possible completions."""
        # Split the line into parts
        parts = line.strip().split()
        
        if not parts or (len(parts) == 1 and not line.endswith(' ')):
            # Completing first word (command or variable)
            all_commands = list(self.commands.keys()) + [
                "create", "show", "graphs", "traverse", "delete", "info",
                "namespace", "stats", "append", "prepend", "insert", "reverse",
                "list", "graph", "tree", "directed", "weighted"  # New graph types
            ]
            # Also include existing variables for variable access
            variables = self.graph_manager.variable_graph.list_variables()
            all_options = all_commands + variables
            return [opt for opt in all_options if opt.startswith(text)]
        
        # Completing arguments
        cmd = parts[0].lower()
        
        if cmd in ["show", "traverse", "delete", "info"]:
            # Complete with variable names
            variables = self.graph_manager.variable_graph.list_variables()
            return [var for var in variables if var.startswith(text)]
        
        elif cmd == "create" and len(parts) >= 3:
            # Complete graph types
            graph_types = ["linear", "directed", "tree", "cyclic", "weighted", "undirected"]
            return [gt for gt in graph_types if gt.startswith(text)]
        
        return []
    
    def _save_history(self) -> None:
        """Save command history to file."""
        if READLINE_AVAILABLE and hasattr(self, '_history_file'):
            try:
                readline.write_history_file(self._history_file)
            except Exception:
                pass  # Ignore history save issues
    
    def _handle_variable_declaration(self, parsed) -> None:
        """Handle new-style variable declarations."""
        # For now, only support 'list' type
        if parsed.graph_type != 'list':
            print(f"Graph type '{parsed.graph_type}' not yet implemented")
            return
        
        if parsed.initializer is None:
            # Empty list
            parsed.initializer = []
        
        result = self.graph_manager.create_from_list(parsed.variable_name, parsed.initializer)
        print(result)
    
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
        graph = self.graph_manager.get_variable(parsed.variable_name)
        if not graph:
            print(f"Variable '{parsed.variable_name}' not found")
            return
        
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
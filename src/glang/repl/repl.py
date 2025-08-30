"""
Core REPL implementation for Glang.
"""

import sys
import os
from typing import Dict, Callable, Optional, List
from glang import __version__, __description__
from .graph_manager import GraphManager

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
        # First check simple commands
        command = user_input.lower()
        
        if command in self.commands:
            result = self.commands[command]()
            if result is False:  # Explicit exit request
                self.running = False
            return
        
        # Parse multi-word commands
        parts = user_input.split()
        if not parts:
            return
        
        cmd = parts[0].lower()
        args = parts[1:] if len(parts) > 1 else []
        
        # Graph management commands
        if cmd == "create":
            self._handle_create_command(args, user_input)
        elif cmd == "show":
            self._handle_show_command(args)
        elif cmd == "graphs":
            print(self.graph_manager.list_graphs())
        elif cmd == "traverse":
            self._handle_traverse_command(args)
        elif cmd == "delete":
            self._handle_delete_command(args)
        elif cmd == "info":
            self._handle_info_command(args)
        elif cmd == "namespace":
            print(self.graph_manager.show_variable_graph())
        elif cmd == "stats":
            print(self.graph_manager.get_variable_stats())
        # Graph operations
        elif cmd in ["append", "prepend", "insert", "delete_at", "reverse"]:
            self._handle_graph_operation(cmd, args)
        else:
            print(f"Unknown command: {user_input}")
            print("Type 'help' for available commands.")
    
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
        print("  create <name> [1,2,3]  - Create graph from list")
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
        print("  create fruits [apple, banana, cherry]")
        print("  show fruits              # [apple] -> [banana] -> [cherry]")
        print("  append orange")
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
            # Completing first word (command)
            all_commands = list(self.commands.keys()) + [
                "create", "show", "graphs", "traverse", "delete", "info",
                "namespace", "stats", "append", "prepend", "insert", "reverse"
            ]
            return [cmd for cmd in all_commands if cmd.startswith(text)]
        
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
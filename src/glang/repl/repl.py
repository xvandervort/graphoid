"""
Modern AST-only REPL implementation for Glang.
"""

import sys
import os
from typing import Dict, Callable, Optional, List
from glang import __version__, __description__
from ..execution import ExecutionSession, ExecutionResult
from ..files import FileManager, FileOperationError

# Try to import readline for command history and navigation
try:
    import readline
    READLINE_AVAILABLE = True
except ImportError:
    READLINE_AVAILABLE = False


class REPL:
    """
    Modern AST-only Read-Eval-Print Loop for the Glang programming language.
    """
    
    def __init__(self) -> None:
        self.prompt = "glang> "
        
        # File management system
        self.file_manager = FileManager()
        
        # AST-based execution system
        self.execution_session = ExecutionSession(self.file_manager)
        
        # Built-in slash commands
        self.slash_commands: Dict[str, Callable[[List[str]], None]] = {
            "help": self._handle_help_command,
            "h": self._handle_help_command,
            "version": self._handle_version_command,
            "ver": self._handle_version_command,
            "exit": self._handle_exit_command,
            "x": self._handle_exit_command,
            "load": self._handle_load_command,
            "save": self._handle_save_command,
            "run": self._handle_run_command,
            "namespace": self._handle_namespace_command,
            "ns": self._handle_namespace_command,
            "stats": self._handle_stats_command,
            "clear": self._handle_clear_command,
            "methods": self._handle_methods_command,
            "type": self._handle_type_command,
            "inspect": self._handle_inspect_command,
            "can": self._handle_can_command,
        }
        
        self.running = True
        self.command_history: List[str] = []
        self._setup_readline()
    
    def run(self) -> None:
        """Start the REPL main loop."""
        print(f"Glang {__version__}")
        print(__description__)
        print("Type '/help' for available commands or '/exit' to quit.")
        print("✨ Try: string name = \"Alice\" then 'name' to see the magic! ✨")
        print()
        
        # Load command history
        self._load_history()
        
        try:
            while self.running:
                try:
                    # Handle multiline input
                    complete_input = self._read_complete_statement()
                    
                    if not complete_input:
                        continue
                    
                    # Add to history
                    self.command_history.append(complete_input)
                    
                    self._process_input(complete_input)
                    
                except KeyboardInterrupt:
                    print("\\n(Use /exit to quit)")
                    continue
                except EOFError:
                    print("\\nGoodbye!")
                    break
        finally:
            # Save history when exiting
            self._save_history()
    
    def _read_complete_statement(self) -> str:
        """Read a complete statement, handling multiline input for control flow."""
        lines = []
        current_prompt = self.prompt
        
        while True:
            try:
                line = input(current_prompt).strip()
                
                # Empty line handling
                if not line:
                    if not lines:  # Empty input at start
                        return ""
                    # Empty line in middle - treat as continuation
                    lines.append("")
                    current_prompt = "...> "
                    continue
                
                lines.append(line)
                
                # Check if this is a complete statement
                combined = " ".join(lines)
                
                # Slash commands are always complete
                if combined.startswith('/'):
                    return combined
                
                # Check for balanced braces to determine if statement is complete
                if self._is_statement_complete(combined):
                    return combined
                
                # Continue reading with continuation prompt
                current_prompt = "...> "
                
            except KeyboardInterrupt:
                # Reset on Ctrl+C
                if lines:
                    print("\\n(Statement cancelled)")
                    return ""
                else:
                    raise
            except EOFError:
                # If we have partial input, treat as complete
                if lines:
                    return " ".join(lines)
                else:
                    raise
    
    def _is_statement_complete(self, statement: str) -> bool:
        """Check if a statement appears to be complete by counting braces."""
        brace_count = 0
        paren_count = 0
        bracket_count = 0
        in_string = False
        escape_next = False
        
        i = 0
        while i < len(statement):
            char = statement[i]
            
            if escape_next:
                escape_next = False
                i += 1
                continue
            
            if char == '\\':
                escape_next = True
                i += 1
                continue
            
            if char == '"' and not escape_next:
                in_string = not in_string
                i += 1
                continue
            
            if in_string:
                i += 1
                continue
            
            # Count delimiters outside of strings
            if char == '{':
                brace_count += 1
            elif char == '}':
                brace_count -= 1
            elif char == '(':
                paren_count += 1
            elif char == ')':
                paren_count -= 1
            elif char == '[':
                bracket_count += 1
            elif char == ']':
                bracket_count -= 1
            
            i += 1
        
        # Statement is complete if all delimiters are balanced
        return brace_count == 0 and paren_count == 0 and bracket_count == 0
    
    def _process_input(self, user_input: str) -> None:
        """Process user input and execute appropriate command."""
        
        # Handle slash commands
        if user_input.startswith('/'):
            self._handle_slash_command(user_input)
            return
        
        # Execute as glang statement using AST system
        try:
            result = self.execution_session.execute_statement(user_input)
            
            if result.success:
                # Display result if there's a meaningful value
                if result.value is not None:
                    print(result.value)
            else:
                # Show execution error with enhanced formatting
                formatted_error = result.get_formatted_error()
                if formatted_error:
                    print(formatted_error)
                else:
                    print(f"Error: {result.error}")
                
        except Exception as e:
            print(f"Unexpected error: {e}")
    
    def _handle_slash_command(self, user_input: str) -> None:
        """Handle slash-prefixed commands."""
        # Remove leading slash and parse
        command_str = user_input[1:].strip()
        
        if not command_str:
            print("Empty command. Type '/help' for available commands.")
            return
        
        parts = command_str.split()
        command = parts[0]
        args = parts[1:] if len(parts) > 1 else []
        
        if command in self.slash_commands:
            self.slash_commands[command](args)
        else:
            print(f"Unknown command: /{command}")
            print("Type '/help' for available commands.")
    
    # Slash command handlers
    
    def _handle_help_command(self, args: List[str]) -> None:
        """Show help information."""
        print("=== Glang Modern REPL ===")
        print()
        print("Language Features:")
        print("  string name = \"Alice\"        # String variables")
        print("  num count = 42              # Number variables")
        print("  bool flag = true            # Boolean variables")
        print("  list items = [1, 2, 3]      # List variables")
        print("  list<num> nums = [1, 2, 3]  # Type-constrained lists")
        print("  name = \"Bob\"                # Type inference")
        print("  items.append(4)             # Method calls")
        print("  items[0]                    # Index access")
        print("  items[0] = 99               # Index assignment")
        print("  load \"file.gr\"              # Load other files")
        print()
        print("Commands:")
        print("  /help, /h                   # Show this help")
        print("  /version, /ver              # Show version")
        print("  /load <file>                # Load .gr file")
        print("  /save <file>                # Save session to .gr file")
        print("  /run <file>                 # Run .gr file in fresh session")
        print("  /namespace, /ns             # Show current variables")
        print("  /stats                      # Show session statistics")
        print("  /clear                      # Clear all variables")
        print("  /methods <var>              # Show methods for variable")
        print("  /type <var>                 # Show type information")
        print("  /inspect <var>              # Detailed variable inspection")
        print("  /can <var> <method>         # Check method availability")
        print("  /exit, /x                   # Exit REPL")
        print()
        print("Method Legend:")
        print("  Methods marked with * require graph/experimental mode")
        print("  Example: add_edge*, visualize_structure*")
    
    def _handle_version_command(self, args: List[str]) -> None:
        """Show version information."""
        print(f"Glang {__version__}")
        print(__description__)
        print("Modern AST-based execution system")
    
    def _handle_exit_command(self, args: List[str]) -> None:
        """Exit the REPL."""
        print("Goodbye!")
        self.running = False
    
    def _handle_load_command(self, args: List[str]) -> None:
        """Load and execute a .gr file."""
        if not args:
            print("Usage: /load <filename>")
            return
        
        filename = args[0]
        try:
            result = self.file_manager.load_file(filename, self.execution_session)
            if result.success:
                print(f"Successfully loaded {filename}")
            else:
                print(f"Failed to load {filename}: {result.error}")
        except FileOperationError as e:
            print(f"Error loading {filename}: {e}")
        except Exception as e:
            print(f"Unexpected error loading {filename}: {e}")
    
    def _handle_save_command(self, args: List[str]) -> None:
        """Save current session to a .gr file."""
        if not args:
            print("Usage: /save <filename>")
            return
        
        filename = args[0]
        try:
            success = self.file_manager.save_file(filename, self.execution_session)
            if success:
                print(f"Successfully saved session to {filename}")
            else:
                print(f"Failed to save to {filename}")
        except FileOperationError as e:
            print(f"Error saving {filename}: {e}")
        except Exception as e:
            print(f"Unexpected error saving {filename}: {e}")
    
    def _handle_run_command(self, args: List[str]) -> None:
        """Run a .gr file in a fresh session."""
        if not args:
            print("Usage: /run <filename>")
            return
        
        filename = args[0]
        try:
            result = self.file_manager.run_file(filename)
            if result.success:
                print(f"Successfully executed {filename}")
                print(f"Result: {result.value}")
            else:
                print(f"Failed to execute {filename}: {result.error}")
        except FileOperationError as e:
            print(f"Error running {filename}: {e}")
        except Exception as e:
            print(f"Unexpected error running {filename}: {e}")
    
    def _handle_namespace_command(self, args: List[str]) -> None:
        """Show current variable namespace."""
        variables = self.execution_session.execution_context.variables
        if not variables:
            print("No variables defined")
            return
        
        print("=== Variable Namespace ===")
        for name, value in variables.items():
            print(f"  [{value.get_type()}] {name} → {value.to_display_string()}")
    
    def _handle_stats_command(self, args: List[str]) -> None:
        """Show session statistics."""
        info = self.execution_session.get_session_info()
        print("=== Session Statistics ===")
        print(f"Variables: {info['variable_count']}")
        print(f"Symbol table entries: {info['symbol_table_size']}")

        if info['variable_count'] > 0:
            print(f"Variable names: {', '.join(info['variables'])}")

        # Show self-hosting metrics
        print("\n=== Self-Hosting Metrics ===")
        try:
            from pathlib import Path
            import os

            # Quick module classification
            repo_root = Path(__file__).parent.parent.parent.parent
            stdlib_path = repo_root / "stdlib"

            if stdlib_path.exists():
                gr_files = list(stdlib_path.glob("*.gr"))
                pure_count = 0
                hybrid_count = 0
                wrapper_count = 0

                # Simple heuristic: check for builtin usage
                for gr_file in gr_files:
                    try:
                        with open(gr_file, 'r') as f:
                            content = f.read()
                            builtin_count = content.count("_builtin_")
                            import_count = content.count('import "')

                            if builtin_count == 0 and import_count <= 1:
                                pure_count += 1
                            elif builtin_count < 5:
                                hybrid_count += 1
                            else:
                                wrapper_count += 1
                    except:
                        pass

                total_modules = len(gr_files)
                if total_modules > 0:
                    print(f"Stdlib Modules: {total_modules}")
                    print(f"  Pure Glang:  {pure_count} ({pure_count/total_modules*100:.1f}%)")
                    print(f"  Hybrid:      {hybrid_count} ({hybrid_count/total_modules*100:.1f}%)")
                    print(f"  Wrapper:     {wrapper_count} ({wrapper_count/total_modules*100:.1f}%)")

                    # Overall progress estimate
                    independence_ratio = pure_count / total_modules * 100
                    functionality_estimate = 51.4  # Based on detailed analysis
                    overall = (independence_ratio * 0.3 + functionality_estimate * 0.7)
                    print(f"\nOverall Self-Hosting: {overall:.1f}%")
                    print(f"Self-Hosting Level: 0 (Hosted Language)")
                else:
                    print("No stdlib modules found")
            else:
                print("Stdlib directory not found")
        except Exception as e:
            print(f"Could not calculate self-hosting metrics: {e}")
    
    def _handle_clear_command(self, args: List[str]) -> None:
        """Clear all variables from the session."""
        self.execution_session.clear_variables()
        print("All variables cleared.")
    
    def _handle_methods_command(self, args: List[str]) -> None:
        """Show all methods available on a variable."""
        if len(args) != 1:
            print("Usage: /methods <variable_name>")
            return
        
        var_name = args[0]
        variables = self.execution_session.execution_context.variables
        
        if var_name not in variables:
            print(f"Variable '{var_name}' not found.")
            return
        
        value = variables[var_name]
        
        # Execute the methods() call on the variable
        result = self.execution_session.execute_statement(f"{var_name}.methods()")
        if result.success and hasattr(result.value, 'elements'):
            methods = [elem.value for elem in result.value.elements]
            print(f"=== Methods for '{var_name}' ({value.get_type()}) ===")
            for method in sorted(methods):
                print(f"  {method}")
        else:
            print(f"Could not retrieve methods for '{var_name}'")
    
    def _handle_type_command(self, args: List[str]) -> None:
        """Show detailed type information for a variable."""
        if len(args) != 1:
            print("Usage: /type <variable_name>")
            return
        
        var_name = args[0]
        variables = self.execution_session.execution_context.variables
        
        if var_name not in variables:
            print(f"Variable '{var_name}' not found.")
            return
        
        value = variables[var_name]
        print(f"=== Type Information for '{var_name}' ===")
        print(f"Type: {value.get_type()}")
        print(f"Value: {value.to_display_string()}")
        
        # Show constraint for lists
        if hasattr(value, 'constraint') and value.constraint:
            print(f"Constraint: {value.constraint}")
        
        # Show size
        result = self.execution_session.execute_statement(f"{var_name}.size()")
        if result.success:
            print(f"Size: {result.value.value}")
    
    def _handle_inspect_command(self, args: List[str]) -> None:
        """Show detailed inspection of a variable."""
        if len(args) != 1:
            print("Usage: /inspect <variable_name>")
            return
        
        var_name = args[0]
        variables = self.execution_session.execution_context.variables
        
        if var_name not in variables:
            print(f"Variable '{var_name}' not found.")
            return
        
        # Execute the inspect() call on the variable
        result = self.execution_session.execute_statement(f"{var_name}.inspect()")
        if result.success:
            print(f"=== Inspection of '{var_name}' ===")
            print(result.value.to_display_string())
        else:
            print(f"Could not inspect '{var_name}': {result.error}")
    
    def _handle_can_command(self, args: List[str]) -> None:
        """Check if a variable supports a specific method."""
        if len(args) != 2:
            print("Usage: /can <variable_name> <method_name>")
            return
        
        var_name = args[0]
        method_name = args[1]
        variables = self.execution_session.execution_context.variables
        
        if var_name not in variables:
            print(f"Variable '{var_name}' not found.")
            return
        
        # Execute the can() call on the variable
        result = self.execution_session.execute_statement(f"{var_name}.can(\"{method_name}\")")
        if result.success:
            can_do = result.value.value
            print(f"'{var_name}' {'can' if can_do else 'cannot'} use method '{method_name}'")
        else:
            print(f"Could not check method availability: {result.error}")
    
    # History and readline support
    
    def _setup_readline(self) -> None:
        """Setup readline for command history and navigation."""
        if READLINE_AVAILABLE:
            # Enable tab completion (basic)
            readline.parse_and_bind("tab: complete")
            # Set up history
            readline.set_history_length(1000)
    
    def _load_history(self) -> None:
        """Load command history from file."""
        if READLINE_AVAILABLE:
            history_file = os.path.expanduser("~/.glang_history")
            try:
                readline.read_history_file(history_file)
            except FileNotFoundError:
                pass  # No history file yet
            except Exception:
                pass  # Ignore other errors
    
    def _save_history(self) -> None:
        """Save command history to file."""
        if READLINE_AVAILABLE:
            history_file = os.path.expanduser("~/.glang_history")
            try:
                readline.write_history_file(history_file)
            except Exception:
                pass  # Ignore errors


def main():
    """Main entry point for the REPL."""
    repl = REPL()
    repl.run()


if __name__ == "__main__":
    main()
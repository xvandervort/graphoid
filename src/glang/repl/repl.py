"""
Core REPL implementation for Glang.
"""

import sys
from typing import Dict, Callable, Optional
from glang import __version__, __description__


class REPL:
    """
    Read-Eval-Print Loop for the Glang programming language.
    """
    
    def __init__(self) -> None:
        self.prompt = "glang> "
        self.commands: Dict[str, Callable[[], Optional[bool]]] = {
            "ver": self._version_command,
            "version": self._version_command,
            "h": self._help_command,
            "help": self._help_command,
            "x": self._exit_command,
            "exit": self._exit_command,
        }
        self.running = True
    
    def start(self) -> None:
        """Start the REPL session."""
        print(f"Glang {__version__}")
        print(__description__)
        print("Type 'help' for available commands or 'exit' to quit.")
        print()
        
        while self.running:
            try:
                user_input = input(self.prompt).strip()
                if user_input:
                    self._process_input(user_input)
            except (KeyboardInterrupt, EOFError):
                print("\nGoodbye!")
                break
    
    def _process_input(self, user_input: str) -> None:
        """Process user input and execute appropriate command."""
        command = user_input.lower()
        
        if command in self.commands:
            result = self.commands[command]()
            if result is False:  # Explicit exit request
                self.running = False
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
        print("  ver, version  - Show version information")
        print("  h, help       - Show this help message")
        print("  x, exit       - Exit the REPL")
        print()
        print("Glang is a prototype programming language with graphs as first-class objects.")
    
    def _exit_command(self) -> bool:
        """Exit the REPL."""
        print("Goodbye!")
        return False  # Signal to stop the REPL
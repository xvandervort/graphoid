"""
Built-in I/O module for Glang

Provides file operations, user input, and directory management.
"""

import os
import sys
from typing import Optional, List
from pathlib import Path

from ..execution.values import (
    GlangValue, StringValue, BooleanValue, NumberValue, 
    ListValue, DataValue, MapValue
)
from ..execution.errors import RuntimeError
from ..ast.nodes import SourcePosition


class IOModule:
    """Built-in I/O module providing file and directory operations."""
    
    @staticmethod
    def read_file(filepath: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Read contents of a file as a string.
        
        Usage in Glang:
            content = io.read_file("example.txt")
        """
        if not isinstance(filepath, StringValue):
            raise RuntimeError(
                f"io.read_file expects string filepath, got {filepath.get_type()}",
                position
            )
        
        path = filepath.value
        
        try:
            with open(path, 'r', encoding='utf-8') as f:
                content = f.read()
            return StringValue(content, position)
        except FileNotFoundError:
            raise RuntimeError(f"File not found: {path}", position)
        except PermissionError:
            raise RuntimeError(f"Permission denied: {path}", position)
        except Exception as e:
            raise RuntimeError(f"Error reading file {path}: {str(e)}", position)
    
    @staticmethod
    def write_file(filepath: GlangValue, content: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Write content to a file.
        
        Usage in Glang:
            io.write_file("output.txt", "Hello, World!")
        """
        if not isinstance(filepath, StringValue):
            raise RuntimeError(
                f"io.write_file expects string filepath, got {filepath.get_type()}",
                position
            )
        
        if not isinstance(content, StringValue):
            # Try to convert to string
            content_str = content.to_display_string()
        else:
            content_str = content.value
        
        path = filepath.value
        
        try:
            # Create parent directories if they don't exist
            parent_dir = os.path.dirname(path)
            if parent_dir and not os.path.exists(parent_dir):
                os.makedirs(parent_dir, exist_ok=True)
            
            with open(path, 'w', encoding='utf-8') as f:
                f.write(content_str)
            
            return BooleanValue(True, position)
        except PermissionError:
            raise RuntimeError(f"Permission denied: {path}", position)
        except Exception as e:
            raise RuntimeError(f"Error writing file {path}: {str(e)}", position)
    
    @staticmethod
    def append_file(filepath: GlangValue, content: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Append content to a file.
        
        Usage in Glang:
            io.append_file("log.txt", "New log entry\\n")
        """
        if not isinstance(filepath, StringValue):
            raise RuntimeError(
                f"io.append_file expects string filepath, got {filepath.get_type()}",
                position
            )
        
        if not isinstance(content, StringValue):
            # Try to convert to string
            content_str = content.to_display_string()
        else:
            content_str = content.value
        
        path = filepath.value
        
        try:
            with open(path, 'a', encoding='utf-8') as f:
                f.write(content_str)
            
            return BooleanValue(True, position)
        except PermissionError:
            raise RuntimeError(f"Permission denied: {path}", position)
        except Exception as e:
            raise RuntimeError(f"Error appending to file {path}: {str(e)}", position)
    
    @staticmethod
    def exists(path: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Check if a file or directory exists.
        
        Usage in Glang:
            if io.exists("config.txt") {
                config = io.read_file("config.txt")
            }
        """
        if not isinstance(path, StringValue):
            raise RuntimeError(
                f"io.exists expects string path, got {path.get_type()}",
                position
            )
        
        return BooleanValue(os.path.exists(path.value), position)
    
    @staticmethod
    def is_file(path: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Check if path is a file.
        
        Usage in Glang:
            if io.is_file("data.txt") {
                content = io.read_file("data.txt")
            }
        """
        if not isinstance(path, StringValue):
            raise RuntimeError(
                f"io.is_file expects string path, got {path.get_type()}",
                position
            )
        
        return BooleanValue(os.path.isfile(path.value), position)
    
    @staticmethod
    def is_dir(path: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Check if path is a directory.
        
        Usage in Glang:
            if io.is_dir("data") {
                files = io.list_dir("data")
            }
        """
        if not isinstance(path, StringValue):
            raise RuntimeError(
                f"io.is_dir expects string path, got {path.get_type()}",
                position
            )
        
        return BooleanValue(os.path.isdir(path.value), position)
    
    @staticmethod
    def list_dir(path: GlangValue = None, position: Optional[SourcePosition] = None) -> GlangValue:
        """List contents of a directory.
        
        Usage in Glang:
            files = io.list_dir(".")  # List current directory
            files = io.list_dir()     # Also lists current directory
        """
        if path is None:
            dir_path = "."
        elif isinstance(path, StringValue):
            dir_path = path.value
        else:
            raise RuntimeError(
                f"io.list_dir expects string path or no argument, got {path.get_type()}",
                position
            )
        
        try:
            entries = os.listdir(dir_path)
            # Convert to list of StringValues
            glang_entries = [StringValue(entry, position) for entry in sorted(entries)]
            return ListValue(glang_entries, position)
        except FileNotFoundError:
            raise RuntimeError(f"Directory not found: {dir_path}", position)
        except PermissionError:
            raise RuntimeError(f"Permission denied: {dir_path}", position)
        except Exception as e:
            raise RuntimeError(f"Error listing directory {dir_path}: {str(e)}", position)
    
    @staticmethod
    def make_dir(path: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Create a directory.
        
        Usage in Glang:
            io.make_dir("output")
            io.make_dir("output/data")  # Creates parent directories too
        """
        if not isinstance(path, StringValue):
            raise RuntimeError(
                f"io.make_dir expects string path, got {path.get_type()}",
                position
            )
        
        dir_path = path.value
        
        try:
            os.makedirs(dir_path, exist_ok=True)
            return BooleanValue(True, position)
        except PermissionError:
            raise RuntimeError(f"Permission denied: {dir_path}", position)
        except Exception as e:
            raise RuntimeError(f"Error creating directory {dir_path}: {str(e)}", position)
    
    @staticmethod
    def remove_file(path: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Remove a file.
        
        Usage in Glang:
            io.remove_file("temp.txt")
        """
        if not isinstance(path, StringValue):
            raise RuntimeError(
                f"io.remove_file expects string path, got {path.get_type()}",
                position
            )
        
        file_path = path.value
        
        try:
            os.remove(file_path)
            return BooleanValue(True, position)
        except FileNotFoundError:
            raise RuntimeError(f"File not found: {file_path}", position)
        except PermissionError:
            raise RuntimeError(f"Permission denied: {file_path}", position)
        except IsADirectoryError:
            raise RuntimeError(f"Path is a directory, not a file: {file_path}", position)
        except Exception as e:
            raise RuntimeError(f"Error removing file {file_path}: {str(e)}", position)
    
    @staticmethod
    def remove_dir(path: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Remove an empty directory.
        
        Usage in Glang:
            io.remove_dir("empty_folder")
        """
        if not isinstance(path, StringValue):
            raise RuntimeError(
                f"io.remove_dir expects string path, got {path.get_type()}",
                position
            )
        
        dir_path = path.value
        
        try:
            os.rmdir(dir_path)
            return BooleanValue(True, position)
        except FileNotFoundError:
            raise RuntimeError(f"Directory not found: {dir_path}", position)
        except PermissionError:
            raise RuntimeError(f"Permission denied: {dir_path}", position)
        except OSError as e:
            if e.errno == 39:  # Directory not empty
                raise RuntimeError(f"Directory not empty: {dir_path}", position)
            else:
                raise RuntimeError(f"Error removing directory {dir_path}: {str(e)}", position)
    
    @staticmethod
    def get_cwd(position: Optional[SourcePosition] = None) -> GlangValue:
        """Get current working directory.
        
        Usage in Glang:
            cwd = io.get_cwd()
        """
        try:
            cwd = os.getcwd()
            return StringValue(cwd, position)
        except Exception as e:
            raise RuntimeError(f"Error getting current directory: {str(e)}", position)
    
    @staticmethod
    def set_cwd(path: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Change current working directory.
        
        Usage in Glang:
            io.set_cwd("/home/user/project")
        """
        if not isinstance(path, StringValue):
            raise RuntimeError(
                f"io.set_cwd expects string path, got {path.get_type()}",
                position
            )
        
        dir_path = path.value
        
        try:
            os.chdir(dir_path)
            return BooleanValue(True, position)
        except FileNotFoundError:
            raise RuntimeError(f"Directory not found: {dir_path}", position)
        except PermissionError:
            raise RuntimeError(f"Permission denied: {dir_path}", position)
        except Exception as e:
            raise RuntimeError(f"Error changing directory to {dir_path}: {str(e)}", position)
    
    @staticmethod
    def file_size(path: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Get size of a file in bytes.
        
        Usage in Glang:
            size = io.file_size("data.txt")
        """
        if not isinstance(path, StringValue):
            raise RuntimeError(
                f"io.file_size expects string path, got {path.get_type()}",
                position
            )
        
        file_path = path.value
        
        try:
            size = os.path.getsize(file_path)
            return NumberValue(size, position)
        except FileNotFoundError:
            raise RuntimeError(f"File not found: {file_path}", position)
        except Exception as e:
            raise RuntimeError(f"Error getting file size for {file_path}: {str(e)}", position)
    
    @staticmethod
    def read_lines(filepath: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Read file as a list of lines.
        
        Usage in Glang:
            lines = io.read_lines("data.txt")
            for line in lines {
                print(line)
            }
        """
        if not isinstance(filepath, StringValue):
            raise RuntimeError(
                f"io.read_lines expects string filepath, got {filepath.get_type()}",
                position
            )
        
        path = filepath.value
        
        try:
            with open(path, 'r', encoding='utf-8') as f:
                lines = f.readlines()
            
            # Strip newlines and convert to StringValues
            glang_lines = [
                StringValue(line.rstrip('\n\r'), position) 
                for line in lines
            ]
            return ListValue(glang_lines, position)
        except FileNotFoundError:
            raise RuntimeError(f"File not found: {path}", position)
        except PermissionError:
            raise RuntimeError(f"Permission denied: {path}", position)
        except Exception as e:
            raise RuntimeError(f"Error reading file {path}: {str(e)}", position)
    
    @staticmethod
    def write_lines(filepath: GlangValue, lines: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Write a list of lines to a file.
        
        Usage in Glang:
            lines = ["First line", "Second line", "Third line"]
            io.write_lines("output.txt", lines)
        """
        if not isinstance(filepath, StringValue):
            raise RuntimeError(
                f"io.write_lines expects string filepath, got {filepath.get_type()}",
                position
            )
        
        if not isinstance(lines, ListValue):
            raise RuntimeError(
                f"io.write_lines expects list of lines, got {lines.get_type()}",
                position
            )
        
        path = filepath.value
        
        try:
            # Create parent directories if they don't exist
            parent_dir = os.path.dirname(path)
            if parent_dir and not os.path.exists(parent_dir):
                os.makedirs(parent_dir, exist_ok=True)
            
            with open(path, 'w', encoding='utf-8') as f:
                for i, line_val in enumerate(lines.elements):
                    if isinstance(line_val, StringValue):
                        line_str = line_val.value
                    else:
                        line_str = line_val.to_display_string()
                    
                    # Add newline except for last line
                    if i < len(lines.elements) - 1:
                        f.write(line_str + '\n')
                    else:
                        f.write(line_str)
            
            return BooleanValue(True, position)
        except PermissionError:
            raise RuntimeError(f"Permission denied: {path}", position)
        except Exception as e:
            raise RuntimeError(f"Error writing file {path}: {str(e)}", position)
    
    @staticmethod
    def print_output(message: GlangValue, newline: GlangValue = None, position: Optional[SourcePosition] = None) -> GlangValue:
        """Print a message to stdout with optional newline control.
        
        Usage in Glang:
            io.print("Hello, World!")          # Prints with newline (default)
            io.print("Enter name: ", false)    # Prints without newline
        """
        if not isinstance(message, StringValue):
            # Convert to string representation
            message_str = message.to_display_string()
        else:
            message_str = message.value
        
        # Default to adding newline
        add_newline = True
        if newline is not None:
            if isinstance(newline, BooleanValue):
                add_newline = newline.value
            else:
                raise RuntimeError(
                    f"io.print newline parameter expects bool, got {newline.get_type()}",
                    position
                )
        
        try:
            if add_newline:
                print(message_str)
            else:
                print(message_str, end='')
                sys.stdout.flush()  # Ensure output is immediately visible
            
            # Return None value (void)
            from ..execution.values import NoneValue
            return NoneValue(position)
        except Exception as e:
            raise RuntimeError(f"Error printing output: {str(e)}", position)
    
    @staticmethod
    def input(prompt: GlangValue = None, position: Optional[SourcePosition] = None) -> GlangValue:
        """Read user input from stdin.
        
        Usage in Glang:
            name = io.input("Enter your name: ")
            answer = io.input()  # No prompt
        """
        if prompt is None:
            prompt_str = ""
        elif isinstance(prompt, StringValue):
            prompt_str = prompt.value
        else:
            prompt_str = prompt.to_display_string()
        
        try:
            user_input = input(prompt_str)
            return StringValue(user_input, position)
        except KeyboardInterrupt:
            # Return empty string on Ctrl+C
            return StringValue("", position)
        except EOFError:
            # Return empty string on EOF
            return StringValue("", position)
        except Exception as e:
            raise RuntimeError(f"Error reading input: {str(e)}", position)


def create_io_module_namespace():
    """Create the namespace for the built-in IO module."""
    from ..modules.module_manager import ModuleNamespace
    
    namespace = ModuleNamespace("io")
    
    # Register all IO functions
    io_functions = {
        'print': IOModule.print_output,  # Use 'print' as the public name
        'read_file': IOModule.read_file,
        'write_file': IOModule.write_file,
        'append_file': IOModule.append_file,
        'exists': IOModule.exists,
        'is_file': IOModule.is_file,
        'is_dir': IOModule.is_dir,
        'list_dir': IOModule.list_dir,
        'make_dir': IOModule.make_dir,
        'remove_file': IOModule.remove_file,
        'remove_dir': IOModule.remove_dir,
        'get_cwd': IOModule.get_cwd,
        'set_cwd': IOModule.set_cwd,
        'file_size': IOModule.file_size,
        'read_lines': IOModule.read_lines,
        'write_lines': IOModule.write_lines,
        'input': IOModule.input,
    }
    
    # Wrap functions as callable values
    from ..execution.function_value import BuiltinFunctionValue
    
    for name, func in io_functions.items():
        namespace.set_symbol(name, BuiltinFunctionValue(name, func))
    
    return namespace
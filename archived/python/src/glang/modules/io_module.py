"""
Built-in I/O module for Glang

Provides file operations, user input, directory management, and network operations.
Uses the Glang file system interface for language independence.
"""

import sys
from typing import Optional, List
from .network_interface import get_network_provider

from ..execution.values import (
    GlangValue, StringValue, BooleanValue, NumberValue,
    DataValue, FileHandleValue
)
from ..execution.graph_values import ListValue, HashValue
from ..execution.errors import RuntimeError
from ..ast.nodes import SourcePosition
from .filesystem_interface import get_filesystem


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
        filesystem = get_filesystem()
        
        try:
            content = filesystem.read_text_file(path)
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
        filesystem = get_filesystem()
        
        try:
            # Create parent directories if they don't exist
            parent_dir = filesystem.get_dirname(path)
            if parent_dir and not filesystem.file_exists(parent_dir):
                filesystem.create_directory(parent_dir, parents=True)
            
            filesystem.write_text_file(path, content_str)
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
    def open(filepath: GlangValue, mode: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Create a file boundary capability.
        
        Usage in Glang:
            read_capability = io.open("data.txt", "r")     # Read capability
            write_capability = io.open("output.txt", "w")  # Write capability (overwrites)
            append_capability = io.open("log.txt", "a")    # Append capability
            
        Returns a boundary capability that provides controlled, unidirectional access
        to external file resources. The capability is immutable and can only perform
        operations consistent with its type (read, write, or append).
        """
        if not isinstance(filepath, StringValue):
            raise RuntimeError(
                f"io.open expects string filepath, got {filepath.get_type()}",
                position
            )
        
        if not isinstance(mode, StringValue):
            raise RuntimeError(
                f"io.open expects string mode, got {mode.get_type()}",
                position
            )
        
        path = filepath.value
        mode_str = mode.value
        
        # Map mode strings to capability types
        mode_mapping = {
            "r": "read",
            "w": "write", 
            "a": "append"
        }
        
        if mode_str not in mode_mapping:
            raise RuntimeError(
                f"io.open mode must be 'r', 'w', or 'a', got '{mode_str}'",
                position
            )
        
        capability_type = mode_mapping[mode_str]
        
        # Create boundary capability (lazy initialization - doesn't open file yet)
        return FileHandleValue(path, capability_type, position)
    
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
        
        filesystem = get_filesystem()
        return BooleanValue(filesystem.file_exists(path.value), position)
    
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
        
        filesystem = get_filesystem()
        return BooleanValue(filesystem.is_file(path.value), position)
    
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
        
        filesystem = get_filesystem()
        return BooleanValue(filesystem.is_directory(path.value), position)
    
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
            filesystem = get_filesystem()
            entries = filesystem.list_directory(dir_path)
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
            filesystem = get_filesystem()
            filesystem.create_directory(dir_path, parents=True)
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
            filesystem = get_filesystem()
            filesystem.remove_file(file_path)
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
            filesystem = get_filesystem()
            filesystem.remove_directory(dir_path)
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
            filesystem = get_filesystem()
            cwd = filesystem.get_current_directory()
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
            filesystem = get_filesystem()
            filesystem.set_current_directory(dir_path)
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
            filesystem = get_filesystem()
            size = filesystem.get_file_size(file_path)
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
            filesystem = get_filesystem()
            
            # Create parent directories if they don't exist
            parent_dir = filesystem.get_dirname(path)
            if parent_dir and not filesystem.file_exists(parent_dir):
                filesystem.create_directory(parent_dir, parents=True)
            
            # Convert lines to text content
            line_strings = [line.to_display_string() for line in lines.elements]
            content = '\n'.join(line_strings)
            
            filesystem.write_text_file(path, content)
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
    
    @staticmethod
    def read_binary(filepath: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Read binary contents of a file as a list of numbers (bytes).
        
        Usage in Glang:
            bytes = io.read_binary("image.png")
        """
        if not isinstance(filepath, StringValue):
            raise RuntimeError(
                f"io.read_binary expects string filepath, got {filepath.get_type()}",
                position
            )
        
        path = filepath.value
        
        try:
            with open(path, 'rb') as f:
                binary_data = f.read()
            
            # Convert bytes to list of numbers (0-255)
            byte_list = [NumberValue(byte, position) for byte in binary_data]
            return ListValue(byte_list, 'num', position)
            
        except FileNotFoundError:
            raise RuntimeError(f"File not found: {path}", position)
        except PermissionError:
            raise RuntimeError(f"Permission denied: {path}", position)
        except Exception as e:
            raise RuntimeError(f"Error reading binary file {path}: {str(e)}", position)
    
    @staticmethod
    def write_binary(filepath: GlangValue, data: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Write binary data to a file from a list of numbers (bytes).
        
        Usage in Glang:
            bytes = [72, 101, 108, 108, 111]  # "Hello" in ASCII
            io.write_binary("output.bin", bytes)
        """
        if not isinstance(filepath, StringValue):
            raise RuntimeError(
                f"io.write_binary expects string filepath, got {filepath.get_type()}",
                position
            )
        
        if not isinstance(data, ListValue):
            raise RuntimeError(
                f"io.write_binary expects list of bytes, got {data.get_type()}",
                position
            )
        
        path = filepath.value
        
        try:
            # Convert list of numbers to bytes
            byte_data = bytearray()
            for item in data.elements:
                if not isinstance(item, NumberValue):
                    raise RuntimeError(
                        f"Binary data must be list of numbers, found {item.get_type()}",
                        position
                    )
                
                byte_value = int(item.value)
                if not (0 <= byte_value <= 255):
                    raise RuntimeError(
                        f"Byte values must be 0-255, got {byte_value}",
                        position
                    )
                
                byte_data.append(byte_value)
            
            with open(path, 'wb') as f:
                f.write(byte_data)
            
            return BooleanValue(True, position)  # Success indicator
            
        except FileNotFoundError:
            raise RuntimeError(f"Directory not found for file: {path}", position)
        except PermissionError:
            raise RuntimeError(f"Permission denied: {path}", position)
        except Exception as e:
            raise RuntimeError(f"Error writing binary file {path}: {str(e)}", position)
    
    @staticmethod
    def join_path(path_list: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Join multiple path components into a single path.
        
        Usage in Glang:
            paths = ["home", "user", "documents", "file.txt"]
            full_path = io.join_path(paths)
        """
        if not isinstance(path_list, ListValue):
            raise RuntimeError(
                f"io.join_path expects list of paths, got {path_list.get_type()}",
                position
            )
        
        path_parts = []
        for path_value in path_list.elements:
            if not isinstance(path_value, StringValue):
                raise RuntimeError(
                    f"io.join_path expects list of strings, found {path_value.get_type()}",
                    position
                )
            path_parts.append(path_value.value)
        
        try:
            if not path_parts:
                # Handle empty list case
                joined_path = ""
            else:
                filesystem = get_filesystem()
                joined_path = filesystem.join_path(*path_parts)
            return StringValue(joined_path, position)
        except Exception as e:
            raise RuntimeError(f"Error joining paths: {str(e)}", position)
    
    @staticmethod
    def split_path(filepath: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Split a path into directory and filename components.
        
        Usage in Glang:
            parts = io.split_path("/home/user/document.txt")
            # Returns list: ["/home/user", "document.txt"]
        """
        if not isinstance(filepath, StringValue):
            raise RuntimeError(
                f"io.split_path expects string filepath, got {filepath.get_type()}",
                position
            )
        
        try:
            filesystem = get_filesystem()
            directory, filename = filesystem.split_path(filepath.value)
            parts = [StringValue(directory, position), StringValue(filename, position)]
            return ListValue(parts, 'string', position)
        except Exception as e:
            raise RuntimeError(f"Error splitting path: {str(e)}", position)
    
    @staticmethod
    def basename(filepath: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Get the filename component of a path (without directory).

        Usage in Glang:
            filename = io.basename("/home/user/document.txt")  # "document.txt"
        """
        if not isinstance(filepath, StringValue):
            raise RuntimeError(
                f"io.basename expects string filepath, got {filepath.get_type()}",
                position
            )
        
        try:
            filesystem = get_filesystem()
            basename = filesystem.get_basename(filepath.value)
            return StringValue(basename, position)
        except Exception as e:
            raise RuntimeError(f"Error getting basename: {str(e)}", position)
    
    @staticmethod
    def dirname(filepath: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Get the directory component of a path (without filename).

        Usage in Glang:
            dirname = io.dirname("/home/user/document.txt")  # "/home/user"
        """
        if not isinstance(filepath, StringValue):
            raise RuntimeError(
                f"io.dirname expects string filepath, got {filepath.get_type()}",
                position
            )
        
        try:
            filesystem = get_filesystem()
            dirname = filesystem.get_dirname(filepath.value)
            return StringValue(dirname, position)
        except Exception as e:
            raise RuntimeError(f"Error getting dirname: {str(e)}", position)
    
    @staticmethod
    def extension(filepath: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Get the file extension from a path.

        Usage in Glang:
            ext = io.extension("document.txt")  # ".txt"
        """
        if not isinstance(filepath, StringValue):
            raise RuntimeError(
                f"io.extension expects string filepath, got {filepath.get_type()}",
                position
            )
        
        try:
            filesystem = get_filesystem()
            ext = filesystem.get_extension(filepath.value)
            return StringValue(ext, position)
        except Exception as e:
            raise RuntimeError(f"Error getting extension: {str(e)}", position)
    
    @staticmethod
    def resolve_path(filepath: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Resolve a path to its absolute form, resolving any relative components.
        
        Usage in Glang:
            abs_path = io.resolve_path("../documents/file.txt")
        """
        if not isinstance(filepath, StringValue):
            raise RuntimeError(
                f"io.resolve_path expects string filepath, got {filepath.get_type()}",
                position
            )
        
        try:
            filesystem = get_filesystem()
            resolved = filesystem.resolve_path(filepath.value)
            return StringValue(resolved, position)
        except Exception as e:
            raise RuntimeError(f"Error resolving path: {str(e)}", position)
    
    @staticmethod
    def list_dir_recursive(path: GlangValue = None, position: Optional[SourcePosition] = None) -> GlangValue:
        """Recursively list all files in a directory tree.

        Usage in Glang:
            all_files = io.list_dir_recursive("src")   # List all files in src/ recursively
            all_files = io.list_dir_recursive()        # List all files from current dir

        Returns a list of relative file paths.
        """
        import os

        if path is None:
            dir_path = "."
        elif isinstance(path, StringValue):
            dir_path = path.value
        else:
            raise RuntimeError(
                f"io.list_dir_recursive expects string path or no argument, got {path.get_type()}",
                position
            )

        try:
            all_files = []
            for root, dirs, files in os.walk(dir_path):
                for file in files:
                    # Get relative path from the starting directory
                    full_path = os.path.join(root, file)
                    if dir_path == ".":
                        relative_path = full_path
                    else:
                        relative_path = os.path.relpath(full_path, dir_path)
                        relative_path = os.path.join(dir_path, relative_path)
                    all_files.append(relative_path)

            # Convert to list of StringValues
            glang_files = [StringValue(file_path, position) for file_path in sorted(all_files)]
            return ListValue(glang_files, position)
        except FileNotFoundError:
            raise RuntimeError(f"Directory not found: {dir_path}", position)
        except PermissionError:
            raise RuntimeError(f"Permission denied: {dir_path}", position)
        except Exception as e:
            raise RuntimeError(f"Error traversing directory {dir_path}: {str(e)}", position)

    @staticmethod
    def find_files(path: GlangValue, pattern: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Find files matching a pattern recursively.

        Usage in Glang:
            py_files = io.find_files("src", "*.py")      # Find all Python files
            gr_files = io.find_files(".", "*.gr")        # Find all Glang files

        Pattern supports wildcards: * for any characters, ? for single character
        """
        import os
        import fnmatch

        if not isinstance(path, StringValue):
            raise RuntimeError(
                f"io.find_files expects string path, got {path.get_type()}",
                position
            )

        if not isinstance(pattern, StringValue):
            raise RuntimeError(
                f"io.find_files expects string pattern, got {pattern.get_type()}",
                position
            )

        dir_path = path.value
        file_pattern = pattern.value

        try:
            matching_files = []
            for root, dirs, files in os.walk(dir_path):
                for file in files:
                    if fnmatch.fnmatch(file, file_pattern):
                        full_path = os.path.join(root, file)
                        if dir_path == ".":
                            relative_path = full_path
                        else:
                            relative_path = os.path.relpath(full_path, dir_path)
                            relative_path = os.path.join(dir_path, relative_path)
                        matching_files.append(relative_path)

            # Convert to list of StringValues
            glang_files = [StringValue(file_path, position) for file_path in sorted(matching_files)]
            return ListValue(glang_files, position)
        except FileNotFoundError:
            raise RuntimeError(f"Directory not found: {dir_path}", position)
        except PermissionError:
            raise RuntimeError(f"Permission denied: {dir_path}", position)
        except Exception as e:
            raise RuntimeError(f"Error finding files in {dir_path}: {str(e)}", position)

    @staticmethod
    def count_lines_recursive(path: GlangValue, extension: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Count total lines in all files with given extension recursively.

        Usage in Glang:
            python_lines = io.count_lines_recursive("src", "py")
            glang_lines = io.count_lines_recursive("stdlib", "gr")

        Only counts non-empty, non-comment lines (lines starting with #).
        """
        if not isinstance(path, StringValue):
            raise RuntimeError(
                f"io.count_lines_recursive expects string path, got {path.get_type()}",
                position
            )

        if not isinstance(extension, StringValue):
            raise RuntimeError(
                f"io.count_lines_recursive expects string extension, got {extension.get_type()}",
                position
            )

        import os
        dir_path = path.value
        file_extension = extension.value

        try:
            total_lines = 0

            for root, dirs, files in os.walk(dir_path):
                for file in files:
                    if file.endswith(f".{file_extension}"):
                        file_path = os.path.join(root, file)
                        try:
                            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                                for line in f:
                                    stripped = line.strip()
                                    if stripped and not stripped.startswith('#'):
                                        total_lines += 1
                        except (UnicodeDecodeError, PermissionError):
                            # Skip files we can't read
                            continue

            return NumberValue(total_lines, position)
        except Exception as e:
            raise RuntimeError(f"Error counting lines in {dir_path}: {str(e)}", position)

    @staticmethod
    def safe_read_file(filepath: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Safely read a file, returning [:ok, content] or [:error, message].

        Usage in Glang:
            result = io.safe_read_file("data.txt")
            if result[0] == ":ok" {
                content = result[1]
            } else {
                error_msg = result[1]
            }
        """
        if not isinstance(filepath, StringValue):
            error_msg = f"io.safe_read_file expects string filepath, got {filepath.get_type()}"
            return ListValue([
                StringValue(":error", position),
                StringValue(error_msg, position)
            ], position)

        try:
            filesystem = get_filesystem()
            content = filesystem.read_text_file(filepath.value)
            return ListValue([
                StringValue(":ok", position),
                StringValue(content, position)
            ], position)
        except FileNotFoundError:
            return ListValue([
                StringValue(":error", position),
                StringValue(f"File not found: {filepath.value}", position)
            ], position)
        except PermissionError:
            return ListValue([
                StringValue(":error", position),
                StringValue(f"Permission denied: {filepath.value}", position)
            ], position)
        except Exception as e:
            return ListValue([
                StringValue(":error", position),
                StringValue(f"Error reading file: {str(e)}", position)
            ], position)

    @staticmethod
    def safe_write_file(filepath: GlangValue, content: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Safely write to a file, returning [:ok] or [:error, message].

        Usage in Glang:
            result = io.safe_write_file("output.txt", "Hello World")
            if result[0] == ":ok" {
                print("Write successful")
            } else {
                print("Write failed: " + result[1])
            }
        """
        if not isinstance(filepath, StringValue):
            error_msg = f"io.safe_write_file expects string filepath, got {filepath.get_type()}"
            return ListValue([
                StringValue(":error", position),
                StringValue(error_msg, position)
            ], position)

        try:
            # Convert content to string
            if isinstance(content, StringValue):
                content_str = content.value
            else:
                content_str = content.to_display_string()

            filesystem = get_filesystem()

            # Create parent directories if they don't exist
            parent_dir = filesystem.get_dirname(filepath.value)
            if parent_dir and not filesystem.file_exists(parent_dir):
                filesystem.create_directory(parent_dir, parents=True)

            filesystem.write_text_file(filepath.value, content_str)
            return ListValue([
                StringValue(":ok", position)
            ], position)
        except PermissionError:
            return ListValue([
                StringValue(":error", position),
                StringValue(f"Permission denied: {filepath.value}", position)
            ], position)
        except Exception as e:
            return ListValue([
                StringValue(":error", position),
                StringValue(f"Error writing file: {str(e)}", position)
            ], position)

    @staticmethod
    def safe_list_dir(path: GlangValue = None, position: Optional[SourcePosition] = None) -> GlangValue:
        """Safely list directory contents, returning [:ok, files] or [:error, message].

        Usage in Glang:
            result = io.safe_list_dir("src")
            if result[0] == ":ok" {
                files = result[1]
                for file in files {
                    print(file)
                }
            } else {
                print("Error: " + result[1])
            }
        """
        if path is None:
            dir_path = "."
        elif isinstance(path, StringValue):
            dir_path = path.value
        else:
            error_msg = f"io.safe_list_dir expects string path or no argument, got {path.get_type()}"
            return ListValue([
                StringValue(":error", position),
                StringValue(error_msg, position)
            ], position)

        try:
            filesystem = get_filesystem()
            entries = filesystem.list_directory(dir_path)
            glang_entries = [StringValue(entry, position) for entry in sorted(entries)]
            file_list = ListValue(glang_entries, position)
            return ListValue([
                StringValue(":ok", position),
                file_list
            ], position)
        except FileNotFoundError:
            return ListValue([
                StringValue(":error", position),
                StringValue(f"Directory not found: {dir_path}", position)
            ], position)
        except PermissionError:
            return ListValue([
                StringValue(":error", position),
                StringValue(f"Permission denied: {dir_path}", position)
            ], position)
        except Exception as e:
            return ListValue([
                StringValue(":error", position),
                StringValue(f"Error listing directory: {str(e)}", position)
            ], position)

    @staticmethod
    def http_get(url: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Make an HTTP GET request using Glang's network interface.
        
        Usage in Glang:
            response = io.http_get("https://api.example.com/data")
        """
        if not isinstance(url, StringValue):
            raise RuntimeError(
                f"io.http_get expects string URL, got {url.get_type()}",
                position
            )
        
        provider = get_network_provider()
        if not provider.is_available():
            raise RuntimeError("Network functionality not available", position)
        
        response = provider.http_request("GET", url.value)
        
        # Handle errors at the Glang level rather than letting Python exceptions bubble up
        if response.status_code == 0:  # Network/URL error
            raise RuntimeError(f"Network error: {response.body}", position)
        
        if response.status_code >= 400:  # HTTP error
            raise RuntimeError(f"HTTP error {response.status_code}: {response.body}", position)
        
        return StringValue(response.body, position)
    
    @staticmethod
    def http_post(url: GlangValue, data: GlangValue = None, position: Optional[SourcePosition] = None) -> GlangValue:
        """Make an HTTP POST request using Glang's network interface.
        
        Usage in Glang:
            response = io.http_post("https://api.example.com/submit", "key=value")
            response = io.http_post("https://api.example.com/submit")  # No data
        """
        if not isinstance(url, StringValue):
            raise RuntimeError(
                f"io.http_post expects string URL, got {url.get_type()}",
                position
            )
        
        # Process data using Glang's type system
        post_data = None
        if data is not None:
            if isinstance(data, StringValue):
                post_data = data.value
            else:
                # Use Glang's own to_display_string() method
                post_data = data.to_display_string()
        
        provider = get_network_provider()
        if not provider.is_available():
            raise RuntimeError("Network functionality not available", position)
        
        response = provider.http_request("POST", url.value, post_data)
        
        # Handle errors at Glang level
        if response.status_code == 0:
            raise RuntimeError(f"Network error: {response.body}", position)
        
        if response.status_code >= 400:
            raise RuntimeError(f"HTTP error {response.status_code}: {response.body}", position)
        
        return StringValue(response.body, position)
    
    @staticmethod
    def download_file(url: GlangValue, filepath: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Download a file from a URL using Glang's filesystem interface.
        
        Usage in Glang:
            io.download_file("https://example.com/file.txt", "local_file.txt")
        """
        if not isinstance(url, StringValue):
            raise RuntimeError(
                f"io.download_file expects string URL, got {url.get_type()}",
                position
            )
        
        if not isinstance(filepath, StringValue):
            raise RuntimeError(
                f"io.download_file expects string filepath, got {filepath.get_type()}",
                position
            )
        
        # Use Glang's filesystem interface for path operations
        filesystem = get_filesystem()
        path = filepath.value
        
        # Create parent directories using Glang's filesystem interface
        parent_dir = filesystem.get_dirname(path)
        if parent_dir and not filesystem.file_exists(parent_dir):
            filesystem.create_directory(parent_dir, parents=True)
        
        # Download using Glang's network provider
        provider = get_network_provider()
        if not provider.is_available():
            raise RuntimeError("Network functionality not available", position)
        
        success = provider.download_to_file(url.value, path)
        if not success:
            raise RuntimeError(f"Failed to download {url.value} to {path}", position)
        
        return BooleanValue(success, position)
    
    @staticmethod
    def send_email(to_addr: GlangValue, subject: GlangValue, body: GlangValue, 
                   smtp_server: GlangValue = None, position: Optional[SourcePosition] = None) -> GlangValue:
        """Send an email notification (requires SMTP configuration).
        
        Usage in Glang:
            io.send_email("user@example.com", "Alert", "System alert message")
            io.send_email("user@example.com", "Alert", "Message", "smtp.gmail.com:587")
        """
        if not isinstance(to_addr, StringValue):
            raise RuntimeError(
                f"io.send_email expects string to_addr, got {to_addr.get_type()}",
                position
            )
        
        if not isinstance(subject, StringValue):
            raise RuntimeError(
                f"io.send_email expects string subject, got {subject.get_type()}",
                position
            )
        
        if not isinstance(body, StringValue):
            raise RuntimeError(
                f"io.send_email expects string body, got {body.get_type()}",
                position
            )
        
        # For now, return a placeholder since email requires SMTP configuration
        # In a real implementation, this would use smtplib
        raise RuntimeError(
            "Email functionality requires SMTP server configuration (not yet implemented)",
            position
        )


def create_io_module_namespace():
    """Create the namespace for the built-in IO module."""
    from .module_builder import create_module

    return create_module(
        "io",
        functions={
            # Console operations
            'print': IOModule.print_output,  # Use 'print' as the public name
            'input': IOModule.input,

            # File operations
            'read_file': IOModule.read_file,
            'write_file': IOModule.write_file,
            'append_file': IOModule.append_file,
            'open': IOModule.open,  # File handle support
            'read_binary': IOModule.read_binary,
            'write_binary': IOModule.write_binary,
            'read_lines': IOModule.read_lines,
            'write_lines': IOModule.write_lines,

            # Safe file operations (Result-style error handling)
            'safe_read_file': IOModule.safe_read_file,
            'safe_write_file': IOModule.safe_write_file,
            'safe_list_dir': IOModule.safe_list_dir,

            # File system operations
            'exists': IOModule.exists,
            'is_file': IOModule.is_file,
            'is_dir': IOModule.is_dir,
            'list_dir': IOModule.list_dir,
            'list_dir_recursive': IOModule.list_dir_recursive,
            'find_files': IOModule.find_files,
            'count_lines_recursive': IOModule.count_lines_recursive,
            'make_dir': IOModule.make_dir,
            'remove_file': IOModule.remove_file,
            'remove_dir': IOModule.remove_dir,
            'get_cwd': IOModule.get_cwd,
            'set_cwd': IOModule.set_cwd,
            'file_size': IOModule.file_size,

            # Path operations
            'join_path': IOModule.join_path,
            'split_path': IOModule.split_path,
            'basename': IOModule.basename,
            'dirname': IOModule.dirname,
            'extension': IOModule.extension,
            'resolve_path': IOModule.resolve_path,

            # Network operations
            'http_get': IOModule.http_get,
            'http_post': IOModule.http_post,
            'download_file': IOModule.download_file,
            'send_email': IOModule.send_email,
        }
    )
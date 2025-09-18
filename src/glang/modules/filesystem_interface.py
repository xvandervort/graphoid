"""
Glang File System Interface

Provides an abstract interface for file system operations, decoupling
Glang from Python's specific file system implementation. This allows
for different backends and maintains Glang's language independence.
"""

from abc import ABC, abstractmethod
from typing import List, Optional, Tuple
from pathlib import Path

from ..execution.values import GlangValue, StringValue, BooleanValue, NumberValue
from ..execution.graph_values import ListValue
from ..ast.nodes import SourcePosition


class FileSystemInterface(ABC):
    """Abstract interface for file system operations using Glang semantics."""
    
    @abstractmethod
    def read_text_file(self, filepath: str) -> str:
        """Read text content from a file."""
        pass
    
    @abstractmethod
    def write_text_file(self, filepath: str, content: str) -> None:
        """Write text content to a file."""
        pass
    
    @abstractmethod
    def append_text_file(self, filepath: str, content: str) -> None:
        """Append text content to a file."""
        pass
    
    @abstractmethod
    def read_binary_file(self, filepath: str) -> bytes:
        """Read binary content from a file."""
        pass
    
    @abstractmethod
    def write_binary_file(self, filepath: str, data: bytes) -> None:
        """Write binary content to a file."""
        pass
    
    @abstractmethod
    def file_exists(self, filepath: str) -> bool:
        """Check if a file exists."""
        pass
    
    @abstractmethod
    def is_file(self, path: str) -> bool:
        """Check if path is a file."""
        pass
    
    @abstractmethod
    def is_directory(self, path: str) -> bool:
        """Check if path is a directory."""
        pass
    
    @abstractmethod
    def list_directory(self, path: str) -> List[str]:
        """List contents of a directory."""
        pass
    
    @abstractmethod
    def create_directory(self, path: str, parents: bool = False) -> None:
        """Create a directory, optionally creating parent directories."""
        pass
    
    @abstractmethod
    def remove_file(self, filepath: str) -> None:
        """Remove a file."""
        pass
    
    @abstractmethod
    def remove_directory(self, path: str) -> None:
        """Remove a directory."""
        pass
    
    @abstractmethod
    def get_current_directory(self) -> str:
        """Get current working directory."""
        pass
    
    @abstractmethod
    def set_current_directory(self, path: str) -> None:
        """Set current working directory."""
        pass
    
    @abstractmethod
    def get_file_size(self, filepath: str) -> int:
        """Get file size in bytes."""
        pass
    
    @abstractmethod
    def join_path(self, *parts: str) -> str:
        """Join path components."""
        pass
    
    @abstractmethod
    def split_path(self, filepath: str) -> Tuple[str, str]:
        """Split path into directory and filename."""
        pass
    
    @abstractmethod
    def get_basename(self, filepath: str) -> str:
        """Get basename of a file path."""
        pass
    
    @abstractmethod
    def get_dirname(self, filepath: str) -> str:
        """Get directory name of a file path."""
        pass
    
    @abstractmethod
    def get_extension(self, filepath: str) -> str:
        """Get file extension."""
        pass
    
    @abstractmethod
    def resolve_path(self, filepath: str) -> str:
        """Resolve path to absolute path."""
        pass


class PythonFileSystem(FileSystemInterface):
    """File system implementation using Python's standard library."""
    
    def read_text_file(self, filepath: str) -> str:
        """Read text content from a file using Python's file operations."""
        try:
            with open(filepath, 'r', encoding='utf-8') as f:
                return f.read()
        except FileNotFoundError:
            raise FileNotFoundError(f"File not found: {filepath}")
        except PermissionError:
            raise PermissionError(f"Permission denied: {filepath}")
        except Exception as e:
            raise RuntimeError(f"Error reading file {filepath}: {str(e)}")
    
    def write_text_file(self, filepath: str, content: str) -> None:
        """Write text content to a file using Python's file operations."""
        try:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
        except PermissionError:
            raise PermissionError(f"Permission denied: {filepath}")
        except Exception as e:
            raise RuntimeError(f"Error writing file {filepath}: {str(e)}")
    
    def append_text_file(self, filepath: str, content: str) -> None:
        """Append text content to a file using Python's file operations."""
        try:
            with open(filepath, 'a', encoding='utf-8') as f:
                f.write(content)
        except PermissionError:
            raise PermissionError(f"Permission denied: {filepath}")
        except Exception as e:
            raise RuntimeError(f"Error appending to file {filepath}: {str(e)}")
    
    def read_binary_file(self, filepath: str) -> bytes:
        """Read binary content from a file using Python's file operations."""
        try:
            with open(filepath, 'rb') as f:
                return f.read()
        except FileNotFoundError:
            raise FileNotFoundError(f"File not found: {filepath}")
        except PermissionError:
            raise PermissionError(f"Permission denied: {filepath}")
        except Exception as e:
            raise RuntimeError(f"Error reading binary file {filepath}: {str(e)}")
    
    def write_binary_file(self, filepath: str, data: bytes) -> None:
        """Write binary content to a file using Python's file operations."""
        try:
            with open(filepath, 'wb') as f:
                f.write(data)
        except PermissionError:
            raise PermissionError(f"Permission denied: {filepath}")
        except Exception as e:
            raise RuntimeError(f"Error writing binary file {filepath}: {str(e)}")
    
    def file_exists(self, filepath: str) -> bool:
        """Check if a file exists using Python's path operations."""
        return Path(filepath).exists()
    
    def is_file(self, path: str) -> bool:
        """Check if path is a file using Python's path operations."""
        return Path(path).is_file()
    
    def is_directory(self, path: str) -> bool:
        """Check if path is a directory using Python's path operations."""
        return Path(path).is_dir()
    
    def list_directory(self, path: str) -> List[str]:
        """List contents of a directory using Python's path operations."""
        try:
            path_obj = Path(path)
            if not path_obj.exists():
                raise FileNotFoundError(f"Directory not found: {path}")
            if not path_obj.is_dir():
                raise RuntimeError(f"Not a directory: {path}")
            
            return [item.name for item in path_obj.iterdir()]
        except PermissionError:
            raise PermissionError(f"Permission denied: {path}")
        except Exception as e:
            raise RuntimeError(f"Error listing directory {path}: {str(e)}")
    
    def create_directory(self, path: str, parents: bool = False) -> None:
        """Create a directory using Python's path operations."""
        try:
            Path(path).mkdir(parents=parents, exist_ok=False)
        except FileExistsError:
            raise RuntimeError(f"Directory already exists: {path}")
        except PermissionError:
            raise PermissionError(f"Permission denied: {path}")
        except Exception as e:
            raise RuntimeError(f"Error creating directory {path}: {str(e)}")
    
    def remove_file(self, filepath: str) -> None:
        """Remove a file using Python's path operations."""
        try:
            path_obj = Path(filepath)
            if not path_obj.exists():
                raise FileNotFoundError(f"File not found: {filepath}")
            if not path_obj.is_file():
                raise RuntimeError(f"Not a file: {filepath}")
            
            path_obj.unlink()
        except PermissionError:
            raise PermissionError(f"Permission denied: {filepath}")
        except Exception as e:
            raise RuntimeError(f"Error removing file {filepath}: {str(e)}")
    
    def remove_directory(self, path: str) -> None:
        """Remove a directory using Python's path operations."""
        try:
            path_obj = Path(path)
            if not path_obj.exists():
                raise FileNotFoundError(f"Directory not found: {path}")
            if not path_obj.is_dir():
                raise RuntimeError(f"Not a directory: {path}")
            
            path_obj.rmdir()  # Only removes empty directories
        except OSError as e:
            if "not empty" in str(e).lower():
                raise RuntimeError(f"Directory not empty: {path}")
            else:
                raise RuntimeError(f"Error removing directory {path}: {str(e)}")
        except PermissionError:
            raise PermissionError(f"Permission denied: {path}")
        except Exception as e:
            raise RuntimeError(f"Error removing directory {path}: {str(e)}")
    
    def get_current_directory(self) -> str:
        """Get current working directory using Python's path operations."""
        return str(Path.cwd())
    
    def set_current_directory(self, path: str) -> None:
        """Set current working directory using Python's os operations."""
        try:
            import os
            os.chdir(path)
        except FileNotFoundError:
            raise FileNotFoundError(f"Directory not found: {path}")
        except PermissionError:
            raise PermissionError(f"Permission denied: {path}")
        except Exception as e:
            raise RuntimeError(f"Error changing directory to {path}: {str(e)}")
    
    def get_file_size(self, filepath: str) -> int:
        """Get file size using Python's path operations."""
        try:
            path_obj = Path(filepath)
            if not path_obj.exists():
                raise FileNotFoundError(f"File not found: {filepath}")
            if not path_obj.is_file():
                raise RuntimeError(f"Not a file: {filepath}")
            
            return path_obj.stat().st_size
        except PermissionError:
            raise PermissionError(f"Permission denied: {filepath}")
        except Exception as e:
            raise RuntimeError(f"Error getting file size {filepath}: {str(e)}")
    
    def join_path(self, *parts: str) -> str:
        """Join path components using Python's path operations."""
        return str(Path(*parts))
    
    def split_path(self, filepath: str) -> Tuple[str, str]:
        """Split path into directory and filename using Python's path operations."""
        path_obj = Path(filepath)
        parent_str = str(path_obj.parent)
        # Match os.path.split behavior: return empty string for current directory
        if parent_str == ".":
            parent_str = ""
        return parent_str, path_obj.name
    
    def get_basename(self, filepath: str) -> str:
        """Get basename using Python's path operations."""
        return Path(filepath).name
    
    def get_dirname(self, filepath: str) -> str:
        """Get directory name using Python's path operations."""
        parent_str = str(Path(filepath).parent)
        # Match os.path.dirname behavior: return empty string for current directory
        if parent_str == ".":
            parent_str = ""
        return parent_str
    
    def get_extension(self, filepath: str) -> str:
        """Get file extension using Python's path operations."""
        return Path(filepath).suffix
    
    def resolve_path(self, filepath: str) -> str:
        """Resolve path to absolute path using Python's path operations."""
        return str(Path(filepath).resolve())


# Global file system instance - can be swapped for testing or different backends
_filesystem: FileSystemInterface = PythonFileSystem()


def get_filesystem() -> FileSystemInterface:
    """Get the current file system implementation."""
    return _filesystem


def set_filesystem(filesystem: FileSystemInterface) -> None:
    """Set the file system implementation (useful for testing)."""
    global _filesystem
    _filesystem = filesystem
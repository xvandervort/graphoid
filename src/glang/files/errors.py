"""File operation specific errors for the glang file import system."""

from typing import Optional
import os


class FileOperationError(Exception):
    """Base class for file operation errors."""
    
    def __init__(self, message: str, filepath: Optional[str] = None, cause: Optional[Exception] = None):
        self.message = message
        self.filepath = filepath
        self.cause = cause
        super().__init__(self._format_error())
    
    def _format_error(self) -> str:
        """Format the error message with context."""
        if self.filepath:
            return f"File operation failed for '{self.filepath}': {self.message}"
        return f"File operation failed: {self.message}"


class FileNotFoundError(FileOperationError):
    """Error when a file is not found."""
    
    def __init__(self, filepath: str):
        super().__init__(f"File not found", filepath)


class FilePermissionError(FileOperationError):
    """Error when file permissions are insufficient."""
    
    def __init__(self, filepath: str, operation: str):
        super().__init__(f"Permission denied for {operation} operation", filepath)


class InvalidFileFormatError(FileOperationError):
    """Error when file format is invalid."""
    
    def __init__(self, filepath: str, details: str = ""):
        message = f"Invalid .gr file format"
        if details:
            message += f": {details}"
        super().__init__(message, filepath)


class FileSyntaxError(FileOperationError):
    """Error when .gr file contains syntax errors."""
    
    def __init__(self, filepath: str, syntax_error: str, line_number: Optional[int] = None):
        if line_number:
            message = f"Syntax error at line {line_number}: {syntax_error}"
        else:
            message = f"Syntax error: {syntax_error}"
        super().__init__(message, filepath)
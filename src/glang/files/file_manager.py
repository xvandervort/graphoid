"""Core file manager for glang file operations."""

import os
from pathlib import Path
from typing import List, Optional, TYPE_CHECKING
from datetime import datetime

from .errors import FileOperationError, FileNotFoundError, FilePermissionError, InvalidFileFormatError

# Use TYPE_CHECKING to avoid circular imports
if TYPE_CHECKING:
    from ..execution import ExecutionSession, ExecutionResult


class FileManager:
    """Manages file operations for .gr program files."""
    
    def __init__(self):
        self.supported_extensions = {'.gr'}
    
    def load_file(self, filepath: str, execution_session: 'ExecutionSession') -> 'ExecutionResult':
        """
        Load and execute a .gr file in the current execution session.
        
        Args:
            filepath: Path to the .gr file
            execution_session: Current execution session to execute in
            
        Returns:
            ExecutionResult with success/failure status
            
        Raises:
            FileOperationError: If file cannot be loaded or executed
        """
        # Validate file path
        resolved_path = self._resolve_path(filepath)
        self._validate_file_exists(resolved_path)
        self._validate_file_extension(resolved_path)
        self._validate_file_readable(resolved_path)
        
        try:
            # Set current file context for module resolution
            execution_session.module_manager.set_current_file_context(resolved_path)
            
            try:
                # Read file contents
                with open(resolved_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                # Execute each line in the session
                lines = self._parse_file_content(content)
                
                results = []
                for line_num, line in enumerate(lines, 1):
                    if line.strip():  # Skip empty lines
                        try:
                            result = execution_session.execute_statement(line)
                            results.append(result)
                            
                            # If any statement fails, stop execution
                            if not result.success:
                                error_msg = f"Execution failed at line {line_num}: {result.error}"
                                raise FileOperationError(error_msg, filepath, result.error)
                                
                        except Exception as e:
                            error_msg = f"Error at line {line_num}: {str(e)}"
                            raise FileOperationError(error_msg, filepath, e)
                
                # Return overall success
                from ..execution import ExecutionResult
                return ExecutionResult(
                    f"Successfully loaded {len(lines)} statements from {os.path.basename(filepath)}", 
                    execution_session.execution_context, 
                    True
                )
            finally:
                # Always clear the file context
                execution_session.module_manager.clear_current_file_context()
                
        except (OSError, IOError) as e:
            raise FileOperationError(f"Failed to read file: {str(e)}", filepath, e)
    
    def run_file(self, filepath: str) -> 'ExecutionResult':
        """
        Execute a .gr file in a fresh execution session.
        
        Args:
            filepath: Path to the .gr file
            
        Returns:
            ExecutionResult with execution results
        """
        # Create fresh session
        from ..execution import ExecutionSession
        fresh_session = ExecutionSession()
        
        try:
            return self.load_file(filepath, fresh_session)
        except FileOperationError:
            raise  # Re-raise as-is
    
    def save_file(self, filepath: str, execution_session: 'ExecutionSession') -> bool:
        """
        Save the current execution session namespace to a .gr file.
        
        Args:
            filepath: Path where to save the .gr file
            execution_session: Session with variables to save
            
        Returns:
            True if save was successful
            
        Raises:
            FileOperationError: If file cannot be saved
        """
        # Validate file path
        resolved_path = self._resolve_path(filepath)
        self._validate_file_extension(resolved_path)
        self._validate_file_writable(resolved_path)
        
        try:
            # Generate file content from namespace
            from .serializer import NamespaceSerializer
            serializer = NamespaceSerializer()
            content = serializer.serialize_namespace(execution_session)
            
            # Ensure directory exists
            os.makedirs(os.path.dirname(resolved_path), exist_ok=True)
            
            # Write to file
            with open(resolved_path, 'w', encoding='utf-8') as f:
                f.write(content)
            
            return True
            
        except (OSError, IOError) as e:
            raise FileOperationError(f"Failed to write file: {str(e)}", filepath, e)
    
    def _resolve_path(self, filepath: str) -> str:
        """Resolve and normalize the file path."""
        # Handle relative paths
        if not os.path.isabs(filepath):
            filepath = os.path.abspath(filepath)
        
        # Add .gr extension if not present
        if not any(filepath.endswith(ext) for ext in self.supported_extensions):
            filepath += '.gr'
            
        return filepath
    
    def _validate_file_exists(self, filepath: str) -> None:
        """Validate that file exists."""
        if not os.path.exists(filepath):
            raise FileNotFoundError(filepath)
        
        if not os.path.isfile(filepath):
            raise FileOperationError(f"Path is not a file", filepath)
    
    def _validate_file_extension(self, filepath: str) -> None:
        """Validate file has supported extension."""
        ext = Path(filepath).suffix.lower()
        if ext not in self.supported_extensions:
            raise InvalidFileFormatError(
                filepath, 
                f"Unsupported file extension '{ext}'. Supported: {', '.join(self.supported_extensions)}"
            )
    
    def _validate_file_readable(self, filepath: str) -> None:
        """Validate file is readable."""
        if not os.access(filepath, os.R_OK):
            raise FilePermissionError(filepath, "read")
    
    def _validate_file_writable(self, filepath: str) -> None:
        """Validate file location is writable."""
        parent_dir = os.path.dirname(filepath)
        
        # Check if file exists and is writable
        if os.path.exists(filepath):
            if not os.access(filepath, os.W_OK):
                raise FilePermissionError(filepath, "write")
        # Check if parent directory is writable
        elif not os.access(parent_dir, os.W_OK):
            raise FilePermissionError(filepath, "write")
    
    def _parse_file_content(self, content: str) -> List[str]:
        """
        Parse .gr file content into executable statements.
        
        Args:
            content: Raw file content
            
        Returns:
            List of executable statements (comments and empty lines removed)
        """
        statements = []
        
        for line in content.split('\n'):
            # Strip whitespace
            line = line.strip()
            
            # Skip empty lines and comments
            if line and not line.startswith('#'):
                statements.append(line)
        
        return statements
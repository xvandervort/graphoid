"""Enhanced error message formatter with source context."""

from typing import Optional, Union
from ..parser.ast_parser import ParseError
from ..lexer.tokenizer import TokenizerError, Token
from ..semantic.errors import SemanticError


class ErrorFormatter:
    """Formats error messages with source context for better developer experience."""
    
    @staticmethod
    def format_error_with_context(
        error: Exception, 
        source_code: str, 
        source_name: str = "<input>"
    ) -> str:
        """
        Format an error with source context display.
        
        Args:
            error: The error to format
            source_code: The original source code that caused the error
            source_name: Name of the source (file name or "<input>")
            
        Returns:
            Formatted error message with context
        """
        # Extract position information from different error types
        line_num = None
        column_num = None
        message = str(error)
        
        if isinstance(error, ParseError):
            if error.token:
                line_num = error.token.line
                column_num = error.token.column
            message = error.message
        elif isinstance(error, TokenizerError):
            line_num = error.line
            column_num = error.column
            message = error.message
        elif isinstance(error, SemanticError):
            if hasattr(error, 'position') and error.position:
                line_num = error.position.line
                column_num = error.position.column
            # Remove redundant "at line X, column Y" from message if present
            message = str(error)
        
        # If we don't have position info, return basic error
        if line_num is None or column_num is None:
            return f"Error: {message}"
        
        # Split source into lines
        source_lines = source_code.split('\n')
        
        # Ensure line number is valid (1-indexed)
        if line_num < 1 or line_num > len(source_lines):
            return f"Error at line {line_num}, column {column_num}: {message}"
        
        # Get the problematic line (convert to 0-indexed)
        error_line = source_lines[line_num - 1]
        
        # Build the formatted error message
        result = []
        result.append(f"Error in {source_name} at line {line_num}, column {column_num}:")
        result.append(f"  {error_line}")
        
        # Create pointer line (adjust for 2-space indent)
        if column_num > 0:
            pointer = f"  {'~' * (column_num - 1)}^"
            result.append(pointer)
        
        result.append(f"{message}")
        
        return "\n".join(result)
    
    @staticmethod
    def format_error_simple(error: Exception) -> str:
        """Format error without context (fallback)."""
        return f"Error: {error}"
    
    @staticmethod 
    def format_multiple_errors(errors: list, source_code: str, source_name: str = "<input>") -> str:
        """Format multiple errors with context."""
        if not errors:
            return "No errors"
        
        if len(errors) == 1:
            return ErrorFormatter.format_error_with_context(errors[0], source_code, source_name)
        
        result = []
        result.append(f"Multiple errors found in {source_name}:")
        result.append("")
        
        for i, error in enumerate(errors, 1):
            result.append(f"Error {i}:")
            formatted = ErrorFormatter.format_error_with_context(error, source_code, source_name)
            # Remove the "Error in ..." line since we already have numbering
            lines = formatted.split('\n')
            if lines and lines[0].startswith("Error in"):
                lines = lines[1:]
            result.extend(f"  {line}" for line in lines)
            result.append("")
        
        return "\n".join(result[:-1])  # Remove last empty line
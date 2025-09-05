"""Tests for language-level load statement functionality."""

import pytest
import tempfile
import os
from pathlib import Path

from src.glang.execution.pipeline import ExecutionSession
from src.glang.parser.ast_parser import ASTParser
from src.glang.ast.nodes import LoadStatement


class TestLoadStatement:
    """Test language-level load statement functionality."""
    
    def test_parse_load_statement(self):
        """Test parsing of load statement."""
        parser = ASTParser()
        result = parser.parse('load "test.gr"')
        
        assert isinstance(result, LoadStatement)
        assert result.filename == "test.gr"
        assert result.position is not None
    
    def test_load_statement_execution(self):
        """Test execution of load statement."""
        session = ExecutionSession()
        
        # Create temporary file with glang code
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('string message = "Loaded successfully!"\n')
            f.write('num value = 123\n')
            temp_file = f.name
        
        try:
            # Test load statement execution
            result = session.execute_statement(f'load "{temp_file}"')
            
            assert result.success
            assert "Loaded" in str(result.value)
            
            # Check that variables were loaded
            variables = session.list_variables()
            assert "message" in variables
            assert "value" in variables
            assert variables["message"]["type"] == "string"
            assert variables["value"]["type"] == "num"
            assert variables["message"]["display"] == "Loaded successfully!"
            assert variables["value"]["display"] == "123"
            
        finally:
            # Clean up temporary file
            os.unlink(temp_file)
    
    def test_load_nonexistent_file(self):
        """Test loading a file that doesn't exist."""
        session = ExecutionSession()
        
        result = session.execute_statement('load "nonexistent.gr"')
        
        assert not result.success
        assert "not found" in str(result.error).lower()
    
    def test_load_file_with_syntax_error(self):
        """Test loading a file with syntax errors."""
        session = ExecutionSession()
        
        # Create temporary file with syntax error
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('string invalid syntax here\n')  # Missing = and value
            temp_file = f.name
        
        try:
            result = session.execute_statement(f'load "{temp_file}"')
            
            assert not result.success
            assert "error" in str(result.error).lower()
            
        finally:
            os.unlink(temp_file)
    
    def test_nested_load_statements(self):
        """Test loading a file that loads another file."""
        session = ExecutionSession()
        
        # Create first file
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f1:
            f1.write('string inner = "Inner file loaded"\n')
            inner_file = f1.name
        
        # Create second file that loads the first
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f2:
            f2.write(f'load "{inner_file}"\n')
            f2.write('string outer = "Outer file loaded"\n')
            outer_file = f2.name
        
        try:
            # Load the outer file
            result = session.execute_statement(f'load "{outer_file}"')
            
            assert result.success
            
            # Check that variables from both files are available
            variables = session.list_variables()
            assert "inner" in variables
            assert "outer" in variables
            assert variables["inner"]["display"] == "Inner file loaded"
            assert variables["outer"]["display"] == "Outer file loaded"
            
        finally:
            os.unlink(inner_file)
            os.unlink(outer_file)
    
    def test_load_statement_semantic_analysis(self):
        """Test semantic analysis of load statements."""
        from src.glang.semantic.pipeline import SemanticPipeline
        
        pipeline = SemanticPipeline()
        
        # Test valid load statement
        result = pipeline.analyze_code('load "test.gr"')
        assert result.success
        assert len(result.errors) == 0
        
        # Load statement should create LoadStatement AST node
        assert isinstance(result.ast, LoadStatement)
        assert result.ast.filename == "test.gr"
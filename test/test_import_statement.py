"""Tests for the import statement functionality."""

import pytest
from src.glang.lexer.tokenizer import Tokenizer
from src.glang.parser.ast_parser import ASTParser, ParseError
from src.glang.ast.nodes import ImportStatement

class TestImportStatementParsing:
    """Test parsing of import statements."""
    
    def setup_method(self):
        """Setup for each test."""
        self.parser = ASTParser()
    
    def test_parse_simple_import(self):
        """Test parsing /import "file.gr" """
        ast = self.parser.parse('/import "math.gr"')
        
        assert isinstance(ast, ImportStatement)
        assert ast.filename == "math.gr"
        assert ast.alias is None
    
    def test_parse_import_with_alias(self):
        """Test parsing /import "file.gr" as alias"""
        ast = self.parser.parse('/import "utilities.gr" as utils')
        
        assert isinstance(ast, ImportStatement)
        assert ast.filename == "utilities.gr"
        assert ast.alias == "utils"
    
    def test_parse_import_with_path(self):
        """Test parsing import with file path."""
        ast = self.parser.parse('/import "lib/math.gr" as math')
        
        assert isinstance(ast, ImportStatement)
        assert ast.filename == "lib/math.gr"
        assert ast.alias == "math"
    
    def test_import_without_quotes_fails(self):
        """Test that import without quotes fails."""
        with pytest.raises(ParseError) as exc:
            self.parser.parse('/import math.gr')
        assert "Expected filename string" in str(exc.value)
    
    def test_import_with_invalid_alias_fails(self):
        """Test that import with invalid alias fails."""
        with pytest.raises(ParseError) as exc:
            self.parser.parse('/import "math.gr" as 123')
        assert "Expected module name" in str(exc.value)
    
    def test_slash_without_import_fails(self):
        """Test that slash without import is not parsed as import."""
        # This should fail because / followed by something other than import
        # is not a valid statement start anymore
        with pytest.raises(ParseError):
            self.parser.parse('/help')
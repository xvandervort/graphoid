"""Tests for the import statement functionality."""

import pytest
from src.glang.lexer.tokenizer import Tokenizer
from src.glang.parser.ast_parser import ASTParser, ParseError
from src.glang.ast.nodes import ImportStatement, ModuleDeclaration, AliasDeclaration

class TestImportStatementParsing:
    """Test parsing of import statements."""
    
    def setup_method(self):
        """Setup for each test."""
        self.parser = ASTParser()
    
    def test_parse_simple_import_with_slash(self):
        """Test parsing /import "file.gr" (compatibility)"""
        ast = self.parser.parse('/import "math.gr"')
        
        assert isinstance(ast, ImportStatement)
        assert ast.filename == "math.gr"
        assert ast.alias is None
    
    def test_parse_simple_import_without_slash(self):
        """Test parsing import "file.gr" (primary syntax)"""
        ast = self.parser.parse('import "math.gr"')
        
        assert isinstance(ast, ImportStatement)
        assert ast.filename == "math.gr"
        assert ast.alias is None
    
    def test_parse_import_with_alias_with_slash(self):
        """Test parsing /import "file.gr" as alias (compatibility)"""
        ast = self.parser.parse('/import "utilities.gr" as utils')
        
        assert isinstance(ast, ImportStatement)
        assert ast.filename == "utilities.gr"
        assert ast.alias == "utils"
    
    def test_parse_import_with_alias_without_slash(self):
        """Test parsing import "file.gr" as alias (primary syntax)"""
        ast = self.parser.parse('import "utilities.gr" as utils')
        
        assert isinstance(ast, ImportStatement)
        assert ast.filename == "utilities.gr"
        assert ast.alias == "utils"
    
    def test_parse_import_with_path_with_slash(self):
        """Test parsing import with file path (compatibility)."""
        ast = self.parser.parse('/import "lib/math.gr" as math')
        
        assert isinstance(ast, ImportStatement)
        assert ast.filename == "lib/math.gr"
        assert ast.alias == "math"
    
    def test_parse_import_with_path_without_slash(self):
        """Test parsing import with file path (primary syntax)."""
        ast = self.parser.parse('import "lib/math.gr" as math')
        
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

class TestModuleDeclarationParsing:
    """Test parsing of module declarations."""
    
    def setup_method(self):
        """Setup for each test."""
        self.parser = ASTParser()
    
    def test_parse_simple_module_declaration(self):
        """Test parsing module math"""
        ast = self.parser.parse('module math')
        
        assert isinstance(ast, ModuleDeclaration)
        assert ast.name == "math"
    
    def test_parse_module_with_underscore(self):
        """Test parsing module with underscore."""
        ast = self.parser.parse('module math_utils')
        
        assert isinstance(ast, ModuleDeclaration)
        assert ast.name == "math_utils"
    
    def test_parse_module_missing_name_fails(self):
        """Test that module without name fails."""
        with pytest.raises(ParseError) as exc:
            self.parser.parse('module')
        assert "Expected module name" in str(exc.value)
    
    def test_parse_module_invalid_name_fails(self):
        """Test that module with invalid name fails."""
        with pytest.raises(ParseError) as exc:
            self.parser.parse('module 123')
        assert "Expected module name" in str(exc.value)

class TestAliasDeclarationParsing:
    """Test parsing of alias declarations."""
    
    def setup_method(self):
        """Setup for each test."""
        self.parser = ASTParser()
    
    def test_parse_simple_alias_declaration(self):
        """Test parsing alias math"""
        ast = self.parser.parse('alias math')
        
        assert isinstance(ast, AliasDeclaration)
        assert ast.name == "math"
    
    def test_parse_alias_with_underscore(self):
        """Test parsing alias with underscore."""
        ast = self.parser.parse('alias math_utils')
        
        assert isinstance(ast, AliasDeclaration)
        assert ast.name == "math_utils"
    
    def test_parse_alias_missing_name_fails(self):
        """Test that alias without name fails."""
        with pytest.raises(ParseError) as exc:
            self.parser.parse('alias')
        assert "Expected alias name" in str(exc.value)
    
    def test_parse_alias_invalid_name_fails(self):
        """Test that alias with invalid name fails."""
        with pytest.raises(ParseError) as exc:
            self.parser.parse('alias 123')
        assert "Expected alias name" in str(exc.value)
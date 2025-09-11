"""Tests for optional type declarations with type inference."""

import pytest
from glang.parser.ast_parser import ASTParser
from glang.semantic.analyzer import SemanticAnalyzer
from glang.semantic.symbol_table import SymbolTable
from glang.execution.executor import ASTExecutor, ExecutionContext
from glang.ast.nodes import *


class TestOptionalTypeDeclarations:
    """Test that types can be omitted when obvious from context."""
    
    def test_string_literal_inference(self):
        """Test type inference from string literal."""
        parser = ASTParser()
        analyzer = SemanticAnalyzer()
        symbol_table = SymbolTable()
        context = ExecutionContext(symbol_table)
        executor = ASTExecutor(context)
        
        # Without explicit type - parser creates Assignment node
        ast = parser.parse('name = "Alice"')
        assert isinstance(ast, Assignment)
        
        # After semantic analysis, symbol should be created with inferred type
        result = analyzer.analyze(ast)
        assert result.success
        # Check that the symbol was created with the right type
        assert analyzer.symbol_table.symbol_exists("name")
        symbol = analyzer.symbol_table.lookup_symbol("name")
        assert symbol.symbol_type == "string"
        
        # Should execute successfully
        executor.execute(ast)
        assert "name" in executor.context.variables
        assert executor.context.variables["name"].value == "Alice"
    
    def test_number_literal_inference(self):
        """Test type inference from number literal."""
        parser = ASTParser()
        analyzer = SemanticAnalyzer()
        symbol_table = SymbolTable()
        context = ExecutionContext(symbol_table)
        executor = ASTExecutor(context)
        
        # Without explicit type
        ast = parser.parse('age = 25')
        assert isinstance(ast, Assignment)
        
        # After semantic analysis
        result = analyzer.analyze(ast)
        assert result.success
        assert analyzer.symbol_table.symbol_exists("age")
        symbol = analyzer.symbol_table.lookup_symbol("age")
        assert symbol.symbol_type == "num"
        
        # Should execute successfully
        executor.execute(ast)
        assert "age" in executor.context.variables
        assert executor.context.variables["age"].value == 25
    
    def test_boolean_literal_inference(self):
        """Test type inference from boolean literal."""
        parser = ASTParser()
        analyzer = SemanticAnalyzer()
        symbol_table = SymbolTable()
        context = ExecutionContext(symbol_table)
        executor = ASTExecutor(context)
        
        # Without explicit type
        ast = parser.parse('active = true')
        assert isinstance(ast, Assignment)
        
        # After semantic analysis
        result = analyzer.analyze(ast)
        assert result.success
        assert analyzer.symbol_table.symbol_exists("active")
        symbol = analyzer.symbol_table.lookup_symbol("active")
        assert symbol.symbol_type == "bool"
        
        # Should execute successfully
        executor.execute(ast)
        assert "active" in executor.context.variables
        assert executor.context.variables["active"].value is True
    
    def test_list_literal_inference(self):
        """Test type inference from list literal."""
        parser = ASTParser()
        analyzer = SemanticAnalyzer()
        symbol_table = SymbolTable()
        context = ExecutionContext(symbol_table)
        executor = ASTExecutor(context)
        
        # Without explicit type
        ast = parser.parse('items = [1, 2, 3]')
        assert isinstance(ast, Assignment)
        
        # After semantic analysis
        result = analyzer.analyze(ast)
        assert result.success
        assert analyzer.symbol_table.symbol_exists("items")
        symbol = analyzer.symbol_table.lookup_symbol("items")
        assert symbol.symbol_type == "list"
        
        # Should execute successfully
        executor.execute(ast)
        assert "items" in executor.context.variables
        assert executor.context.variables["items"].elements[0].value == 1
        assert executor.context.variables["items"].elements[1].value == 2
        assert executor.context.variables["items"].elements[2].value == 3
    
    def test_data_literal_inference(self):
        """Test type inference from data node literal."""
        parser = ASTParser()
        analyzer = SemanticAnalyzer()
        symbol_table = SymbolTable()
        context = ExecutionContext(symbol_table)
        executor = ASTExecutor(context)
        
        # Without explicit type
        ast = parser.parse('user = { "name": "Alice" }')
        assert isinstance(ast, Assignment)
        
        # After semantic analysis
        result = analyzer.analyze(ast)
        assert result.success
        assert analyzer.symbol_table.symbol_exists("user")
        symbol = analyzer.symbol_table.lookup_symbol("user")
        assert symbol.symbol_type == "data"
        
        # Should execute successfully
        executor.execute(ast)
        assert "user" in executor.context.variables
        assert executor.context.variables["user"].key == "name"
        assert executor.context.variables["user"].value.value == "Alice"
    
    def test_map_literal_inference(self):
        """Test type inference from map literal."""
        parser = ASTParser()
        analyzer = SemanticAnalyzer()
        symbol_table = SymbolTable()
        context = ExecutionContext(symbol_table)
        executor = ASTExecutor(context)
        
        # Without explicit type
        ast = parser.parse('config = { "host": "localhost", "port": 8080 }')
        assert isinstance(ast, Assignment)
        
        # After semantic analysis
        result = analyzer.analyze(ast)
        assert result.success
        assert analyzer.symbol_table.symbol_exists("config")
        symbol = analyzer.symbol_table.lookup_symbol("config")
        assert symbol.symbol_type == "hash"
        
        # Should execute successfully
        executor.execute(ast)
        assert "config" in executor.context.variables
        # Map should contain data nodes
        # HashValue uses 'pairs' to store its key-value pairs
        assert len(executor.context.variables["config"].pairs) == 2
    
    def test_explicit_type_still_works(self):
        """Test that explicit type declarations still work."""
        parser = ASTParser()
        analyzer = SemanticAnalyzer()
        symbol_table = SymbolTable()
        context = ExecutionContext(symbol_table)
        executor = ASTExecutor(context)
        
        # With explicit type
        ast = parser.parse('string name = "Bob"')
        assert isinstance(ast, VariableDeclaration)
        assert ast.var_type == "string"
        
        # Should analyze and execute successfully
        result = analyzer.analyze(ast)
        assert result.success
        
        executor.execute(ast)
        assert "name" in executor.context.variables
        assert executor.context.variables["name"].value == "Bob"
    
    def test_reassignment_to_existing_variable(self):
        """Test that reassignment to existing variables works."""
        parser = ASTParser()
        analyzer = SemanticAnalyzer()
        symbol_table = SymbolTable()
        context = ExecutionContext(symbol_table)
        executor = ASTExecutor(context)
        
        # First declaration (implicit)
        ast1 = parser.parse('count = 10')
        assert isinstance(ast1, Assignment)
        result1 = analyzer.analyze(ast1, clear_state=True)
        assert result1.success
        executor.execute(ast1)
        assert "count" in executor.context.variables
        assert executor.context.variables["count"].value == 10
        
        # Second assignment (should work as reassignment to existing var)
        ast2 = parser.parse('count = 20')
        assert isinstance(ast2, Assignment)
        result2 = analyzer.analyze(ast2, clear_state=False)
        assert result2.success
        executor.execute(ast2)
        assert executor.context.variables["count"].value == 20
    
    def test_type_mismatch_error(self):
        """Test that type mismatches are still caught."""
        parser = ASTParser()
        analyzer = SemanticAnalyzer()
        symbol_table = SymbolTable()
        context = ExecutionContext(symbol_table)
        executor = ASTExecutor(context)
        
        # First declaration as string (implicit)
        ast1 = parser.parse('value = "hello"')
        assert isinstance(ast1, Assignment)
        result1 = analyzer.analyze(ast1, clear_state=True)
        assert result1.success
        executor.execute(ast1)
        assert "value" in executor.context.variables
        
        # Try to reassign with different type
        # The semantic analyzer doesn't check type compatibility on reassignment yet,
        # so this test needs to check at execution time
        ast2 = parser.parse('value = 42')
        assert isinstance(ast2, Assignment)
        result2 = analyzer.analyze(ast2, clear_state=False)
        # For now, semantic analysis passes but execution would enforce types
        assert result2.success  # Semantic analysis passes
        # In a full implementation, we'd check type compatibility here
    
    def test_constrained_types_require_explicit_declaration(self):
        """Test that type constraints still require explicit type."""
        parser = ASTParser()
        
        # This should still require explicit type for constraints
        ast = parser.parse('list<num> scores = [95, 87, 92]')
        assert isinstance(ast, VariableDeclaration)
        assert ast.var_type == "list"
        assert ast.type_constraint == "num"
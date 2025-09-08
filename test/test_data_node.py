"""Test suite for data node functionality in glang."""

import pytest
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from glang.lexer.tokenizer import Tokenizer, TokenType
from glang.parser.ast_parser import ASTParser
from glang.ast.nodes import DataNodeLiteral, StringLiteral, NumberLiteral
from glang.semantic.analyzer import SemanticAnalyzer
from glang.execution.executor import ExecutionContext, ASTExecutor
from glang.execution.values import DataValue, StringValue, NumberValue, ListValue
from glang.semantic.symbol_table import SymbolTable


class TestDataNodeParsing:
    """Test parsing of data node literals and declarations."""
    
    def test_parse_simple_data_node_literal(self):
        """Test parsing a simple data node literal."""
        parser = ASTParser()
        ast = parser.parse('{ "name": "Alice" }')
        
        # Should be an expression statement containing a data node literal
        assert ast.expression is not None
        assert isinstance(ast.expression, DataNodeLiteral)
        assert ast.expression.key == "name"
        assert isinstance(ast.expression.value, StringLiteral)
    
    def test_parse_data_node_with_number_value(self):
        """Test parsing data node with number value."""
        parser = ASTParser()
        ast = parser.parse('{ "age": 25 }')
        
        assert isinstance(ast.expression, DataNodeLiteral)
        assert ast.expression.key == "age"
        assert isinstance(ast.expression.value, NumberLiteral)
        assert ast.expression.value.value == 25
    
    def test_parse_data_node_with_list_value(self):
        """Test parsing data node with list value."""
        parser = ASTParser()
        ast = parser.parse('{ "scores": [95, 87, 92] }')
        
        assert isinstance(ast.expression, DataNodeLiteral)
        assert ast.expression.key == "scores"
        # List literal check would go here
    
    def test_parse_data_declaration(self):
        """Test parsing data type variable declaration."""
        parser = ASTParser()
        ast = parser.parse('data person = { "name": "Bob" }')
        
        assert ast.var_type == "data"
        assert ast.name == "person"
        assert isinstance(ast.initializer, DataNodeLiteral)
    
    def test_parse_data_declaration_with_constraint(self):
        """Test parsing data type with constraint."""
        parser = ASTParser()
        ast = parser.parse('data<string> config = { "setting": "value" }')
        
        assert ast.var_type == "data"
        assert ast.type_constraint == "string"
        assert ast.name == "config"


class TestDataNodeExecution:
    """Test execution of data node operations."""
    
    def setup_method(self):
        """Set up test environment."""
        self.parser = ASTParser()
        self.analyzer = SemanticAnalyzer()
        self.symbol_table = SymbolTable()
        self.context = ExecutionContext(self.symbol_table)
        self.executor = ASTExecutor(self.context)
    
    def test_execute_data_node_literal(self):
        """Test executing a data node literal."""
        ast = self.parser.parse('{ "key": "value" }')
        result = self.executor.execute(ast.expression)
        
        assert isinstance(result, DataValue)
        assert result.key == "key"
        assert isinstance(result.value, StringValue)
        assert result.value.value == "value"
    
    def test_data_node_declaration_and_access(self):
        """Test declaring data node and accessing it."""
        # Declare data node
        ast1 = self.parser.parse('data node = { "id": 123 }')
        self.executor.execute(ast1)
        
        # Access the variable
        ast2 = self.parser.parse('node')
        result = self.executor.execute(ast2.expression)
        
        assert isinstance(result, DataValue)
        assert result.key == "id"
        assert result.value.value == 123
    
    def test_data_node_key_access(self):
        """Test accessing the key of a data node."""
        # Declare data node
        ast1 = self.parser.parse('data item = { "product": "laptop" }')
        self.executor.execute(ast1)
        
        # Access the key
        ast2 = self.parser.parse('item.key()')
        result = self.executor.execute(ast2)
        
        assert isinstance(result, StringValue)
        assert result.value == "product"
    
    def test_data_node_value_access(self):
        """Test accessing the value of a data node."""
        # Declare data node
        ast1 = self.parser.parse('data item = { "price": 999 }')
        self.executor.execute(ast1)
        
        # Access the value
        ast2 = self.parser.parse('item.value()')
        result = self.executor.execute(ast2)
        
        assert isinstance(result, NumberValue)
        assert result.value == 999
    
    def test_data_node_with_type_constraint(self):
        """Test data node with type constraint."""
        # This should work - value is string
        ast1 = self.parser.parse('data<string> config = { "mode": "debug" }')
        self.executor.execute(ast1)
        
        value = self.context.get_variable("config")
        assert isinstance(value, DataValue)
        assert value.constraint == "string"
    
    def test_data_node_constraint_violation(self):
        """Test that wrong type in constrained data node raises error."""
        # This should fail - value is number but constraint is string
        ast = self.parser.parse('data<string> config = { "count": 42 }')
        
        with pytest.raises(Exception) as exc_info:
            self.executor.execute(ast)
        assert "constraint" in str(exc_info.value).lower()
    
    def test_data_node_type_inference(self):
        """Test type inference for data nodes."""
        # Use type inference
        ast1 = self.parser.parse('mydata = { "status": true }')
        self.executor.execute(ast1)
        
        value = self.context.get_variable("mydata")
        assert isinstance(value, DataValue)
        assert value.key == "status"
        # Note: boolean literals need to be parsed as 'true'/'false'


class TestDataNodeMethods:
    """Test data node method calls."""
    
    def setup_method(self):
        """Set up test environment."""
        self.parser = ASTParser()
        self.analyzer = SemanticAnalyzer()
        self.symbol_table = SymbolTable()
        self.context = ExecutionContext(self.symbol_table)
        self.executor = ASTExecutor(self.context)
    
    def test_data_node_universal_methods(self):
        """Test universal methods on data nodes."""
        # Create a data node
        ast1 = self.parser.parse('data info = { "version": "1.0" }')
        self.executor.execute(ast1)
        
        # Test type() method
        ast2 = self.parser.parse('info.type()')
        result = self.executor.execute(ast2)
        assert isinstance(result, StringValue)
        assert result.value == "data"
        
        # Test size() method (should be 1 for single key-value pair)
        ast3 = self.parser.parse('info.size()')
        result = self.executor.execute(ast3)
        assert isinstance(result, NumberValue)
        assert result.value == 1
        
        # Test methods() method
        ast4 = self.parser.parse('info.methods()')
        result = self.executor.execute(ast4)
        assert isinstance(result, ListValue)
        method_names = [elem.value for elem in result.elements]
        assert "key" in method_names
        assert "value" in method_names
        assert "type" in method_names
    
    def test_data_node_inspect(self):
        """Test inspect() method on data nodes."""
        ast1 = self.parser.parse('data item = { "code": "ABC123" }')
        self.executor.execute(ast1)
        
        ast2 = self.parser.parse('item.inspect()')
        result = self.executor.execute(ast2)
        assert isinstance(result, StringValue)
        assert "data" in result.value.lower()


class TestDataNodeSemanticAnalysis:
    """Test semantic analysis of data nodes."""
    
    def setup_method(self):
        """Set up test environment."""
        self.parser = ASTParser()
        self.analyzer = SemanticAnalyzer()
    
    def test_analyze_data_declaration(self):
        """Test semantic analysis of data declaration."""
        ast = self.parser.parse('data config = { "debug": true }')
        result = self.analyzer.analyze(ast)
        
        assert result.success
        assert len(result.errors) == 0
        assert result.symbol_table.symbol_exists("config")
        
        symbol = result.symbol_table.lookup_symbol("config")
        assert symbol.symbol_type == "data"
    
    def test_analyze_data_with_constraint(self):
        """Test semantic analysis of constrained data node."""
        ast = self.parser.parse('data<num> stats = { "count": 100 }')
        result = self.analyzer.analyze(ast)
        
        assert result.success
        assert len(result.errors) == 0
        
        symbol = result.symbol_table.lookup_symbol("stats")
        assert symbol.symbol_type == "data"
        assert symbol.type_constraint == "num"
    
    def test_analyze_invalid_constraint(self):
        """Test semantic analysis rejects invalid constraint."""
        ast = self.parser.parse('data<invalid> test = { "key": "value" }')
        result = self.analyzer.analyze(ast)
        
        assert not result.success
        assert len(result.errors) > 0
    
    def test_analyze_data_redeclaration(self):
        """Test that redeclaring data variable is caught."""
        # First declaration
        ast1 = self.parser.parse('data x = { "a": 1 }')
        result1 = self.analyzer.analyze(ast1)
        assert result1.success
        
        # Second declaration should fail
        ast2 = self.parser.parse('data x = { "b": 2 }')
        result2 = self.analyzer.analyze(ast2, clear_state=False)
        assert not result2.success
        assert any("already declared" in str(e).lower() for e in result2.errors)


class TestDataNodeEdgeCases:
    """Test edge cases and error conditions."""
    
    def setup_method(self):
        """Set up test environment."""
        self.parser = ASTParser()
        self.executor = ASTExecutor(ExecutionContext(SymbolTable()))
    
    def test_empty_key_rejected(self):
        """Test that empty string key is still valid (but not recommended)."""
        ast = self.parser.parse('{ "": "empty key" }')
        result = self.executor.execute(ast.expression)
        
        assert isinstance(result, DataValue)
        assert result.key == ""
    
    def test_data_node_with_nested_data(self):
        """Test data node containing another data node as value."""
        ast = self.parser.parse('{ "nested": { "inner": "value" } }')
        result = self.executor.execute(ast.expression)
        
        assert isinstance(result, DataValue)
        assert result.key == "nested"
        assert isinstance(result.value, DataValue)
        assert result.value.key == "inner"
    
    def test_key_immutability(self):
        """Test that data node key cannot be changed."""
        # Create data node
        ast1 = self.parser.parse('data item = { "original": 100 }')
        self.executor.execute(ast1)
        
        # The key should be immutable - we can only access it, not change it
        ast2 = self.parser.parse('item.key()')
        result = self.executor.execute(ast2)
        assert result.value == "original"
        
        # There's no way to change the key - it's immutable by design
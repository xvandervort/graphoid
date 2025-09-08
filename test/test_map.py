"""Comprehensive tests for map data type."""

import pytest
from glang.parser.ast_parser import ASTParser
from glang.execution.pipeline import ExecutionSession
from glang.ast.nodes import MapLiteral, VariableDeclaration, Assignment
from glang.execution.values import MapValue, StringValue, NumberValue, BooleanValue
from glang.semantic.analyzer import SemanticAnalyzer
from glang.execution.executor import ASTExecutor, ExecutionContext
from glang.semantic.symbol_table import SymbolTable


class TestMapParsing:
    """Test map literal parsing."""
    
    def test_empty_map_parsing(self):
        """Test parsing of empty map literal."""
        parser = ASTParser()
        ast = parser.parse('{}')
        
        # Parser wraps expressions in ExpressionStatement
        from glang.ast.nodes import ExpressionStatement
        assert isinstance(ast, ExpressionStatement)
        assert isinstance(ast.expression, MapLiteral)
        assert len(ast.expression.pairs) == 0
    
    def test_single_pair_map_parsing(self):
        """Test parsing of single key-value pair."""
        parser = ASTParser()
        ast = parser.parse('{ "name": "Alice" }')
        
        # Single pair should still return DataNodeLiteral (not MapLiteral) for backward compatibility
        from glang.ast.nodes import ExpressionStatement, DataNodeLiteral
        assert isinstance(ast, ExpressionStatement)
        assert isinstance(ast.expression, DataNodeLiteral)
        assert ast.expression.key == "name"
    
    def test_multiple_pair_map_parsing(self):
        """Test parsing of multiple key-value pairs."""
        parser = ASTParser()
        ast = parser.parse('{ "name": "Alice", "age": 30, "active": true }')
        
        from glang.ast.nodes import ExpressionStatement
        assert isinstance(ast, ExpressionStatement)
        assert isinstance(ast.expression, MapLiteral)
        assert len(ast.expression.pairs) == 3
        
        keys = [pair[0] for pair in ast.expression.pairs]
        assert "name" in keys
        assert "age" in keys  
        assert "active" in keys
    
    def test_map_with_expression_values(self):
        """Test map with complex expressions as values."""
        parser = ASTParser()
        ast = parser.parse('{ "sum": 1 + 2, "list": [1, 2, 3] }')
        
        from glang.ast.nodes import ExpressionStatement
        assert isinstance(ast, ExpressionStatement)
        assert isinstance(ast.expression, MapLiteral)
        assert len(ast.expression.pairs) == 2
        assert ast.expression.pairs[0][0] == "sum"
        assert ast.expression.pairs[1][0] == "list"


class TestMapExecution:
    """Test map execution and evaluation."""
    
    def test_empty_map_execution(self):
        """Test execution of empty map."""
        session = ExecutionSession()
        result = session.execute_statement('{}')
        
        assert result.success
        assert isinstance(result.value, MapValue)
        assert len(result.value.pairs) == 0
        assert str(result.value) == "{}"
    
    def test_map_literal_execution(self):
        """Test execution of map literal."""
        session = ExecutionSession()
        result = session.execute_statement('{ "host": "localhost", "port": 8080 }')
        
        assert result.success
        assert isinstance(result.value, MapValue)
        assert len(result.value.pairs) == 2
        assert result.value.get("host").value == "localhost"
        assert result.value.get("port").value == 8080
    
    def test_map_variable_declaration(self):
        """Test declaring map variables."""
        session = ExecutionSession()
        
        # Basic map declaration
        result = session.execute_statement('map config = { "debug": true, "port": 3000 }')
        assert result.success
        assert "Declared map variable 'config'" in str(result.value)
        
        # Verify the variable exists
        result = session.execute_statement('config')
        assert result.success
        assert isinstance(result.value, MapValue)
        assert result.value.get("debug").value is True
        assert result.value.get("port").value == 3000
    
    def test_constrained_map_declaration(self):
        """Test map with type constraints."""
        session = ExecutionSession()
        
        # Valid constraint
        result = session.execute_statement('map<string> names = { "first": "Alice", "last": "Smith" }')
        assert result.success
        
        # Verify constraint is applied
        result = session.execute_statement('names')
        assert result.success
        assert result.value.constraint == "string"
    
    def test_constrained_map_validation(self):
        """Test constraint validation for maps."""
        session = ExecutionSession()
        
        # Should fail - number values in string-constrained map
        result = session.execute_statement('map<string> config = { "name": "test", "port": 8080 }')
        assert not result.success
        assert "constraint" in str(result.error)


class TestMapMethods:
    """Test map method calls."""
    
    def test_map_get_method(self):
        """Test map.get() method."""
        session = ExecutionSession()
        session.execute_statement('map config = { "host": "localhost", "port": 8080 }')
        
        # Get existing key - now returns data node, not raw value
        result = session.execute_statement('config.get("host")')
        assert result.success
        assert str(result.value) == '{ "host": localhost }'
        
        # Get missing key (should return empty string)
        result = session.execute_statement('config.get("missing")')
        assert result.success
        assert str(result.value) == ""
    
    def test_map_set_method(self):
        """Test map.set() method."""
        session = ExecutionSession()
        session.execute_statement('map config = { "host": "localhost" }')
        
        # Set new key
        result = session.execute_statement('config.set("port", 8080)')
        assert result.success
        
        # Verify it was set - get() now returns data node
        result = session.execute_statement('config.get("port")')
        assert result.success
        assert str(result.value) == '{ "port": 8080 }'
        
        # Update existing key
        result = session.execute_statement('config.set("host", "127.0.0.1")')
        assert result.success
        
        result = session.execute_statement('config.get("host")')
        assert result.success
        assert str(result.value) == '{ "host": 127.0.0.1 }'  # get() returns data node
    
    def test_map_has_key_method(self):
        """Test map.has_key() method."""
        session = ExecutionSession()
        session.execute_statement('map config = { "debug": true, "port": 3000 }')
        
        # Existing key
        result = session.execute_statement('config.has_key("debug")')
        assert result.success
        assert result.value.value is True
        
        # Missing key
        result = session.execute_statement('config.has_key("missing")')
        assert result.success
        assert result.value.value is False
    
    def test_map_count_values_method(self):
        """Test map.count_values() method."""
        session = ExecutionSession()
        session.execute_statement('map data = { "a": 100, "b": 200, "c": 100, "d": 300, "e": 100 }')
        
        # Count value that appears multiple times
        result = session.execute_statement('data.count_values(100)')
        assert result.success
        assert result.value.value == 3  # appears 3 times (keys a, c, e)
        
        # Count value that appears once
        result = session.execute_statement('data.count_values(200)')
        assert result.success
        assert result.value.value == 1
        
        # Count value that doesn't exist
        result = session.execute_statement('data.count_values(999)')
        assert result.success
        assert result.value.value == 0
        
        # Test with string values
        session.execute_statement('map names = { "first": "Alice", "second": "Bob", "third": "Alice" }')
        result = session.execute_statement('names.count_values("Alice")')
        assert result.success
        assert result.value.value == 2
    
    def test_map_keys_method(self):
        """Test map.keys() method."""
        session = ExecutionSession()
        session.execute_statement('map config = { "host": "localhost", "port": 8080, "debug": true }')
        
        result = session.execute_statement('config.keys()')
        assert result.success
        
        # Should return a list of strings
        keys_list = result.value
        assert keys_list.get_type() == "list"
        assert len(keys_list.elements) == 3
        
        key_strings = [elem.value for elem in keys_list.elements]
        assert "host" in key_strings
        assert "port" in key_strings
        assert "debug" in key_strings
    
    def test_map_values_method(self):
        """Test map.values() method."""
        session = ExecutionSession()
        session.execute_statement('map data = { "name": "Alice", "age": 30 }')
        
        result = session.execute_statement('data.values()')
        assert result.success
        
        # Should return a list
        values_list = result.value
        assert values_list.get_type() == "list"
        assert len(values_list.elements) == 2
        
        # Check that values are present (order may vary)
        value_strs = [str(elem) for elem in values_list.elements]
        assert "Alice" in value_strs
        assert "30" in value_strs
    
    def test_map_remove_method(self):
        """Test map.remove() method."""
        session = ExecutionSession()
        session.execute_statement('map config = { "host": "localhost", "port": 8080, "debug": true }')
        
        # Remove existing key
        result = session.execute_statement('config.remove("debug")')
        assert result.success
        assert result.value.value is True  # Should return true for existing key
        
        # Verify it was removed
        result = session.execute_statement('config.has_key("debug")')
        assert result.success
        assert result.value.value is False
        
        # Remove non-existing key
        result = session.execute_statement('config.remove("missing")')
        assert result.success
        assert result.value.value is False  # Should return false for missing key
    
    def test_map_empty_method(self):
        """Test map.empty() method."""
        session = ExecutionSession()
        
        # Empty map
        session.execute_statement('map empty_config = {}')
        result = session.execute_statement('empty_config.empty()')
        assert result.success
        assert result.value.value is True
        
        # Non-empty map
        session.execute_statement('map config = { "host": "localhost" }')
        result = session.execute_statement('config.empty()')
        assert result.success
        assert result.value.value is False
    
    def test_map_size_method(self):
        """Test map.size() universal method."""
        session = ExecutionSession()
        session.execute_statement('map config = { "host": "localhost", "port": 8080, "debug": true }')
        
        result = session.execute_statement('config.size()')
        assert result.success
        assert result.value.value == 3
    
    def test_map_merge_method(self):
        """Test map.merge() method."""
        session = ExecutionSession()
        session.execute_statement('map map1 = { "a": 1, "b": 2 }')
        session.execute_statement('map map2 = { "c": 3, "d": 4 }')
        
        # Basic merge
        result = session.execute_statement('map1.merge(map2)')
        assert result.success
        assert "Merged 2 pairs" in str(result.value)
        
        # Check result
        result = session.execute_statement('map1.size()')
        assert result.success
        assert result.value.value == 4
        
        # Check that keys from both maps exist
        result = session.execute_statement('map1.has_key("a")')
        assert result.success
        assert result.value.value is True
        
        result = session.execute_statement('map1.has_key("c")')
        assert result.success
        assert result.value.value is True
        
        # Test key collision (should overwrite)
        session.execute_statement('map map3 = { "a": 999 }')
        result = session.execute_statement('map1.merge(map3)')
        assert result.success
        
        result = session.execute_statement('map1.get("a")')
        assert result.success
        assert str(result.value) == '{ "a": 999 }'
    
    def test_map_push_method(self):
        """Test map.push() method for data nodes."""
        session = ExecutionSession()
        session.execute_statement('data node1 = { "name": "Alice" }')
        session.execute_statement('data node2 = { "age": 25 }')
        session.execute_statement('map people = {}')
        
        # Push data nodes
        result = session.execute_statement('people.push(node1)')
        assert result.success
        assert "Pushed data node with key 'name'" in str(result.value)
        
        result = session.execute_statement('people.push(node2)')
        assert result.success
        assert "Pushed data node with key 'age'" in str(result.value)
        
        # Check that both nodes were added with their keys
        result = session.execute_statement('people.size()')
        assert result.success
        assert result.value.value == 2
        
        result = session.execute_statement('people.has_key("name")')
        assert result.success
        assert result.value.value is True
        
        result = session.execute_statement('people.has_key("age")')
        assert result.success
        assert result.value.value is True
        
        # Verify the data node values - get() returns data node, not raw value
        result = session.execute_statement('people.get("name")')
        assert result.success
        assert str(result.value) == '{ "name": Alice }'  # get() returns data node
        
        result = session.execute_statement('people.get("age")')
        assert result.success
        assert str(result.value) == '{ "age": 25 }'  # get() returns data node
    
    def test_map_pop_method(self):
        """Test map.pop() method."""
        session = ExecutionSession()
        session.execute_statement('map people = { "alice": { "name": "Alice" }, "bob": { "name": "Bob" } }')
        
        # Pop existing key
        result = session.execute_statement('people.pop("alice")')
        assert result.success
        assert '{ "name": Alice }' in str(result.value)
        
        # Verify it was removed
        result = session.execute_statement('people.has_key("alice")')
        assert result.success
        assert result.value.value is False
        
        result = session.execute_statement('people.size()')
        assert result.success
        assert result.value.value == 1
        
        # Pop non-existing key
        result = session.execute_statement('people.pop("missing")')
        assert result.success
        assert str(result.value) == ""


class TestMapConstraints:
    """Test map type constraints."""
    
    def test_valid_string_constraint(self):
        """Test valid string constraint."""
        session = ExecutionSession()
        
        result = session.execute_statement('map<string> names = { "first": "Alice", "last": "Smith" }')
        assert result.success
        
        # Adding valid string value
        result = session.execute_statement('names.set("middle", "Jane")')
        assert result.success
    
    def test_invalid_constraint_violation(self):
        """Test constraint violation."""
        session = ExecutionSession()
        session.execute_statement('map<string> names = { "first": "Alice" }')
        
        # Try to set non-string value (should fail)
        result = session.execute_statement('names.set("age", 30)')
        assert not result.success
        assert "map<string>" in str(result.error)
    
    def test_number_constrained_map(self):
        """Test number-constrained map."""
        session = ExecutionSession()
        
        result = session.execute_statement('map<num> scores = { "math": 95, "science": 87 }')
        assert result.success
        
        # Valid number
        result = session.execute_statement('scores.set("english", 92)')
        assert result.success
        
        # Invalid string (should fail)
        result = session.execute_statement('scores.set("art", "A+")')
        assert not result.success
        assert "map<num>" in str(result.error)
    
    def test_map_merge_constraints(self):
        """Test constraint validation in merge operations."""
        session = ExecutionSession()
        session.execute_statement('map<string> strings = { "a": "hello" }')
        session.execute_statement('map<string> more_strings = { "b": "world" }')
        session.execute_statement('map<num> numbers = { "c": 42 }')
        
        # Valid merge with same constraint
        result = session.execute_statement('strings.merge(more_strings)')
        assert result.success
        
        # Invalid merge with different constraint
        result = session.execute_statement('strings.merge(numbers)')
        assert not result.success
        assert "Cannot merge map<num> into map<string>" in str(result.error)


class TestMapSemanticAnalysis:
    """Test semantic analysis of maps."""
    
    def test_map_variable_declaration_analysis(self):
        """Test semantic analysis of map variable declarations."""
        parser = ASTParser()
        analyzer = SemanticAnalyzer()
        
        # Valid map declaration
        ast = parser.parse('map config = { "host": "localhost", "port": 8080 }')
        result = analyzer.analyze(ast)
        assert result.success
        
        # Test that parser catches invalid syntax early
        # The test for invalid type behavior is now at the parser level
        # The semantic analyzer validates the logic of valid parsed constructs
    
    def test_map_constraint_analysis(self):
        """Test semantic analysis of map constraints."""
        parser = ASTParser()
        analyzer = SemanticAnalyzer()
        
        # Valid constraint
        ast = parser.parse('map<string> names = { "first": "Alice" }')
        result = analyzer.analyze(ast)
        assert result.success
        
        # Invalid constraint
        ast = parser.parse('map<invalid_constraint> data = {}')
        result = analyzer.analyze(ast)
        assert not result.success
    
    def test_map_method_validation(self):
        """Test validation of map method calls."""
        parser = ASTParser()
        analyzer = SemanticAnalyzer()
        
        # First declare the map
        ast = parser.parse('map config = { "host": "localhost" }')
        result = analyzer.analyze(ast)
        assert result.success
        
        # Valid method
        ast = parser.parse('config.get("host")')
        result = analyzer.analyze(ast, clear_state=False)
        assert result.success
        
        # Invalid method
        ast = parser.parse('config.invalid_method()')
        result = analyzer.analyze(ast, clear_state=False)
        assert not result.success


class TestMapDisplay:
    """Test map display and string representation."""
    
    def test_empty_map_display(self):
        """Test display of empty map."""
        session = ExecutionSession()
        result = session.execute_statement('{}')
        
        assert result.success
        assert str(result.value) == "{}"
    
    def test_single_pair_map_display(self):
        """Test display of single pair."""
        session = ExecutionSession()
        # Force multiple pairs to get MapLiteral instead of DataNodeLiteral
        result = session.execute_statement('{ "name": "Alice", "temp": "delete" }')
        assert result.success
        
        # Remove the temp key to get single pair
        session.execute_statement('map m = { "name": "Alice" }')
        result = session.execute_statement('m')
        assert result.success
        assert str(result.value) == '{ "name": Alice }'
    
    def test_multiple_pair_map_display(self):
        """Test display of multiple pairs."""
        session = ExecutionSession()
        result = session.execute_statement('{ "host": "localhost", "port": 8080, "debug": true }')
        
        assert result.success
        display = str(result.value)
        assert display.startswith("{")
        assert display.endswith("}")
        assert '"host": localhost' in display
        assert '"port": 8080' in display
        assert '"debug": true' in display


class TestMapEdgeCases:
    """Test edge cases and error conditions."""
    
    def test_map_method_argument_errors(self):
        """Test method calls with wrong number of arguments."""
        session = ExecutionSession()
        session.execute_statement('map config = { "host": "localhost" }')
        
        # get() with wrong argument count
        result = session.execute_statement('config.get()')
        assert not result.success
        assert "argument" in str(result.error)
        
        result = session.execute_statement('config.get("host", "extra")')
        assert not result.success
        assert "argument" in str(result.error)
        
        # set() with wrong argument count
        result = session.execute_statement('config.set("key")')
        assert not result.success
        assert "argument" in str(result.error)
    
    def test_map_method_type_errors(self):
        """Test method calls with wrong argument types."""
        session = ExecutionSession()
        session.execute_statement('map config = { "host": "localhost" }')
        
        # Non-string key
        result = session.execute_statement('config.get(123)')
        assert not result.success
        assert "string" in str(result.error)
        
        result = session.execute_statement('config.set(123, "value")')
        assert not result.success
        assert "string" in str(result.error)
    
    def test_map_merge_edge_cases(self):
        """Test merge method edge cases."""
        session = ExecutionSession()
        session.execute_statement('map config = { "host": "localhost" }')
        
        # Wrong argument count
        result = session.execute_statement('config.merge()')
        assert not result.success
        assert "argument" in str(result.error)
        
        result = session.execute_statement('config.merge("not_a_map", "extra")')
        assert not result.success
        assert "argument" in str(result.error)
        
        # Wrong argument type
        result = session.execute_statement('config.merge("not_a_map")')
        assert not result.success
        assert "map argument" in str(result.error)
    
    def test_map_push_edge_cases(self):
        """Test push method edge cases."""
        session = ExecutionSession()
        session.execute_statement('map config = { "host": "localhost" }')
        
        # Wrong argument count
        result = session.execute_statement('config.push()')
        assert not result.success
        assert "argument" in str(result.error)
        
        result = session.execute_statement('config.push("data", "extra")')
        assert not result.success
        assert "argument" in str(result.error)
        
        # Wrong argument type (not a data node)
        result = session.execute_statement('config.push("not_a_data_node")')
        assert not result.success
        assert "data node argument" in str(result.error)
        
        result = session.execute_statement('config.push(123)')
        assert not result.success
        assert "data node argument" in str(result.error)
    
    def test_map_pop_edge_cases(self):
        """Test pop method edge cases."""
        session = ExecutionSession()
        session.execute_statement('map config = { "host": "localhost" }')
        
        # Wrong argument count
        result = session.execute_statement('config.pop()')
        assert not result.success
        assert "argument" in str(result.error)
        
        result = session.execute_statement('config.pop("key", "extra")')
        assert not result.success
        assert "argument" in str(result.error)
        
        # Wrong argument type (non-string key)
        result = session.execute_statement('config.pop(123)')
        assert not result.success
        assert "string" in str(result.error)
    
    def test_map_universal_methods(self):
        """Test that maps support universal methods."""
        session = ExecutionSession()
        session.execute_statement('map config = { "host": "localhost", "port": 8080 }')
        
        # type()
        result = session.execute_statement('config.type()')
        assert result.success
        assert str(result.value) == "map"
        
        # size()
        result = session.execute_statement('config.size()')
        assert result.success
        assert result.value.value == 2
        
        # inspect()
        result = session.execute_statement('config.inspect()')
        assert result.success
        assert "map" in str(result.value)
        assert "2 pairs" in str(result.value)
        
        # methods() - should return complete list
        result = session.execute_statement('config.methods()')
        assert result.success
        methods_list = result.value
        assert methods_list.get_type() == "list"
        
        # Should have 16 methods total (5 universal + 11 map-specific)
        assert len(methods_list.elements) == 16  # Updated count with count_values()
        
        # Check that all expected methods are present
        method_names = [elem.value for elem in methods_list.elements]
        
        # Universal methods
        for method in ['type', 'methods', 'can', 'inspect', 'size']:
            assert method in method_names, f"Missing universal method: {method}"
            
        # Map-specific methods  
        for method in ['get', 'set', 'has_key', 'keys', 'values', 'remove', 'empty', 'merge', 'push', 'pop']:
            assert method in method_names, f"Missing map method: {method}"
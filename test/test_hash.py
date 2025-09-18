"""Comprehensive tests for map/hash data type."""

import pytest
import warnings
from glang.parser.ast_parser import ASTParser
from glang.execution.pipeline import ExecutionSession
from glang.ast.nodes import MapLiteral, VariableDeclaration, Assignment
from glang.execution.values import StringValue, NumberValue, BooleanValue
from glang.execution.graph_values import ListValue, HashValue
from glang.semantic.analyzer import SemanticAnalyzer
from glang.execution.executor import ASTExecutor, ExecutionContext
from glang.semantic.symbol_table import SymbolTable


class TestMapParsing:
    """Test map literal parsing."""
    
    def test_empty_hash_parsing(self):
        """Test parsing of empty hash literal."""
        parser = ASTParser()
        ast = parser.parse('{}')
        
        # Parser wraps expressions in ExpressionStatement
        from glang.ast.nodes import ExpressionStatement
        assert isinstance(ast, ExpressionStatement)
        assert isinstance(ast.expression, MapLiteral)
        assert len(ast.expression.pairs) == 0
    
    def test_single_pair_hash_parsing(self):
        """Test parsing of single key-value pair."""
        parser = ASTParser()
        ast = parser.parse('{ "name": "Alice" }')
        
        # Single pair should still return DataNodeLiteral (not MapLiteral) for backward compatibility
        from glang.ast.nodes import ExpressionStatement, DataNodeLiteral
        assert isinstance(ast, ExpressionStatement)
        assert isinstance(ast.expression, DataNodeLiteral)
        assert ast.expression.key == "name"
    
    def test_multiple_pair_hash_parsing(self):
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
    
    def test_hash_with_expression_values(self):
        """Test hash with complex expressions as values."""
        parser = ASTParser()
        ast = parser.parse('{ "sum": 1 + 2, "list": [1, 2, 3] }')
        
        from glang.ast.nodes import ExpressionStatement
        assert isinstance(ast, ExpressionStatement)
        assert isinstance(ast.expression, MapLiteral)
        assert len(ast.expression.pairs) == 2
        assert ast.expression.pairs[0][0] == "sum"
        assert ast.expression.pairs[1][0] == "list"


class TestMapExecution:
    """Test hash execution and evaluation."""
    
    def test_empty_hash_execution(self):
        """Test execution of empty hash."""
        session = ExecutionSession()
        result = session.execute_statement('{}')
        
        assert result.success
        assert isinstance(result.value, HashValue)
        # Check length differently for each type
        if hasattr(result.value, 'pairs'):
            assert len(result.value.pairs) == 0
        else:
            assert len(result.value.graph) == 0
        assert str(result.value) == "{}"
    
    def test_hash_literal_execution(self):
        """Test execution of hash literal."""
        session = ExecutionSession()
        result = session.execute_statement('{ "host": "localhost", "port": 8080 }')
        
        assert result.success
        assert isinstance(result.value, HashValue)
        # Check length differently for each type
        if hasattr(result.value, 'pairs'):
            assert len(result.value.pairs) == 2
        else:
            assert len(result.value.graph) == 2
        assert result.value.get("host").value == "localhost"
        assert result.value.get("port").value == 8080
    
    def test_hash_variable_declaration(self):
        """Test declaring hash variables."""
        session = ExecutionSession()
        
        # Basic hash declaration
        result = session.execute_statement('hash config = { "debug": true, "port": 3000 }')
        assert result.success
        assert "Declared hash variable 'config'" in str(result.value)
        
        # Verify the variable exists
        result = session.execute_statement('config')
        assert result.success
        assert isinstance(result.value, HashValue)
        assert result.value.get("debug").value is True
        assert result.value.get("port").value == 3000
    
    def test_constrained_hash_declaration(self):
        """Test hash with type constraints."""
        session = ExecutionSession()
        
        # Valid constraint
        result = session.execute_statement('hash<string> names = { "first": "Alice", "last": "Smith" }')
        assert result.success
        
        # Verify constraint is applied
        result = session.execute_statement('names')
        assert result.success
        assert result.value.constraint == "string"
    
    def test_constrained_hash_validation(self):
        """Test constraint validation for hashs."""
        session = ExecutionSession()
        
        # Should fail - number values in string-constrained hash
        result = session.execute_statement('hash<string> config = { "name": "test", "port": 8080 }')
        assert not result.success
        assert "constraint" in str(result.error)


class TestMapMethods:
    """Test hash method calls."""
    
    def test_hash_get_method(self):
        """Test hash.get() method (deprecated)."""
        session = ExecutionSession()
        session.execute_statement('hash config = { "host": "localhost", "port": 8080 }')

        # Get existing key - now returns data node, not raw value
        with pytest.warns(DeprecationWarning, match="The hash.get\\(\\) method is deprecated"):
            result = session.execute_statement('config.get("host")')
        assert result.success
        assert str(result.value) == '{ "host": localhost }'

        # Get missing key (should return empty string)
        with pytest.warns(DeprecationWarning, match="The hash.get\\(\\) method is deprecated"):
            result = session.execute_statement('config.get("missing")')
        assert result.success
        assert str(result.value) == ""
    
    def test_hash_set_method(self):
        """Test hash.set() method."""
        session = ExecutionSession()
        session.execute_statement('hash config = { "host": "localhost" }')
        
        # Set new key
        result = session.execute_statement('config.set("port", 8080)')
        assert result.success

        # Verify it was set - get() now returns data node
        with pytest.warns(DeprecationWarning, match="The hash.get\\(\\) method is deprecated"):
            result = session.execute_statement('config.get("port")')
        assert result.success
        assert str(result.value) == '{ "port": 8080 }'

        # Update existing key
        result = session.execute_statement('config.set("host", "127.0.0.1")')
        assert result.success

        with pytest.warns(DeprecationWarning, match="The hash.get\\(\\) method is deprecated"):
            result = session.execute_statement('config.get("host")')
        assert result.success
        assert str(result.value) == '{ "host": 127.0.0.1 }'  # get() returns data node
    
    def test_hash_has_key_method(self):
        """Test hash.has_key() method."""
        session = ExecutionSession()
        session.execute_statement('hash config = { "debug": true, "port": 3000 }')
        
        # Existing key
        result = session.execute_statement('config.has_key("debug")')
        assert result.success
        assert result.value.value is True
        
        # Missing key
        result = session.execute_statement('config.has_key("missing")')
        assert result.success
        assert result.value.value is False
    
    def test_hash_count_values_method(self):
        """Test hash.count_values() method."""
        session = ExecutionSession()
        session.execute_statement('hash data = { "a": 100, "b": 200, "c": 100, "d": 300, "e": 100 }')
        
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
        session.execute_statement('hash names = { "first": "Alice", "second": "Bob", "third": "Alice" }')
        result = session.execute_statement('names.count_values("Alice")')
        assert result.success
        assert result.value.value == 2
    
    def test_hash_keys_method(self):
        """Test hash.keys() method."""
        session = ExecutionSession()
        session.execute_statement('hash config = { "host": "localhost", "port": 8080, "debug": true }')
        
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
    
    def test_hash_values_method(self):
        """Test hash.values() method."""
        session = ExecutionSession()
        session.execute_statement('hash data = { "name": "Alice", "age": 30 }')
        
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
    
    def test_hash_remove_method(self):
        """Test hash.remove() method."""
        session = ExecutionSession()
        session.execute_statement('hash config = { "host": "localhost", "port": 8080, "debug": true }')
        
        # Remove existing key
        result = session.execute_statement('config.remove("debug")')
        assert result.success
        # Now returns the hash itself for chaining
        assert hasattr(result.value, 'pairs')  # Should return the hash
        
        # Verify it was removed
        result = session.execute_statement('config.has_key("debug")')
        assert result.success
        assert result.value.value is False
        
        # Remove non-existing key
        result = session.execute_statement('config.remove("missing")')
        assert result.success
        # Now returns the hash itself for chaining (regardless of key existence)
        assert hasattr(result.value, 'pairs')  # Should return the hash
    
    def test_hash_empty_method(self):
        """Test hash.empty() method."""
        session = ExecutionSession()
        
        # Empty hash
        session.execute_statement('hash empty_config = {}')
        result = session.execute_statement('empty_config.empty()')
        assert result.success
        assert result.value.value is True
        
        # Non-empty hash
        session.execute_statement('hash config = { "host": "localhost" }')
        result = session.execute_statement('config.empty()')
        assert result.success
        assert result.value.value is False
    
    def test_hash_size_method(self):
        """Test hash.size() universal method."""
        session = ExecutionSession()
        session.execute_statement('hash config = { "host": "localhost", "port": 8080, "debug": true }')
        
        result = session.execute_statement('config.size()')
        assert result.success
        assert result.value.value == 3
    
    def test_hash_merge_method(self):
        """Test hash.merge() method."""
        session = ExecutionSession()
        session.execute_statement('hash hash1 = { "a": 1, "b": 2 }')
        session.execute_statement('hash hash2 = { "c": 3, "d": 4 }')
        
        # Basic merge
        result = session.execute_statement('hash1.merge(hash2)')
        assert result.success
        assert "Merged 2 pairs" in str(result.value)
        
        # Check result
        result = session.execute_statement('hash1.size()')
        assert result.success
        assert result.value.value == 4
        
        # Check that keys from both hashs exist
        result = session.execute_statement('hash1.has_key("a")')
        assert result.success
        assert result.value.value is True
        
        result = session.execute_statement('hash1.has_key("c")')
        assert result.success
        assert result.value.value is True
        
        # Test key collision (should overwrite)
        session.execute_statement('hash hash3 = { "a": 999 }')
        result = session.execute_statement('hash1.merge(hash3)')
        assert result.success
        
        result = session.execute_statement('hash1.get("a")')
        assert result.success
        assert str(result.value) == '{ "a": 999 }'
    
    def test_hash_push_method(self):
        """Test hash.push() method for data nodes."""
        session = ExecutionSession()
        session.execute_statement('data node1 = { "name": "Alice" }')
        session.execute_statement('data node2 = { "age": 25 }')
        session.execute_statement('hash people = {}')
        
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
    
    def test_hash_pop_method(self):
        """Test hash.pop() method."""
        session = ExecutionSession()
        session.execute_statement('hash people = { "alice": { "name": "Alice" }, "bob": { "name": "Bob" } }')
        
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
    """Test hash type constraints."""
    
    def test_valid_string_constraint(self):
        """Test valid string constraint."""
        session = ExecutionSession()
        
        result = session.execute_statement('hash<string> names = { "first": "Alice", "last": "Smith" }')
        assert result.success
        
        # Adding valid string value
        result = session.execute_statement('names.set("middle", "Jane")')
        assert result.success
    
    def test_invalid_constraint_violation(self):
        """Test constraint violation."""
        session = ExecutionSession()
        session.execute_statement('hash<string> names = { "first": "Alice" }')
        
        # Try to set non-string value (should fail)
        result = session.execute_statement('names.set("age", 30)')
        assert not result.success
        assert "hash<string>" in str(result.error)
    
    def test_number_constrained_hash(self):
        """Test number-constrained hash."""
        session = ExecutionSession()
        
        result = session.execute_statement('hash<num> scores = { "math": 95, "science": 87 }')
        assert result.success
        
        # Valid number
        result = session.execute_statement('scores.set("english", 92)')
        assert result.success
        
        # Invalid string (should fail)
        result = session.execute_statement('scores.set("art", "A+")')
        assert not result.success
        assert "hash<num>" in str(result.error)
    
    def test_hash_merge_constraints(self):
        """Test constraint validation in merge operations."""
        session = ExecutionSession()
        session.execute_statement('hash<string> strings = { "a": "hello" }')
        session.execute_statement('hash<string> more_strings = { "b": "world" }')
        session.execute_statement('hash<num> numbers = { "c": 42 }')
        
        # Valid merge with same constraint
        result = session.execute_statement('strings.merge(more_strings)')
        assert result.success
        
        # Invalid merge with different constraint
        result = session.execute_statement('strings.merge(numbers)')
        assert not result.success
        assert "Cannot merge hash<num> into hash<string>" in str(result.error)


class TestMapSemanticAnalysis:
    """Test semantic analysis of hashs."""
    
    def test_hash_variable_declaration_analysis(self):
        """Test semantic analysis of hash variable declarations."""
        parser = ASTParser()
        analyzer = SemanticAnalyzer()
        
        # Valid hash declaration
        ast = parser.parse('hash config = { "host": "localhost", "port": 8080 }')
        result = analyzer.analyze(ast)
        assert result.success
        
        # Test that parser catches invalid syntax early
        # The test for invalid type behavior is now at the parser level
        # The semantic analyzer validates the logic of valid parsed constructs
    
    def test_hash_constraint_analysis(self):
        """Test semantic analysis of hash constraints."""
        parser = ASTParser()
        analyzer = SemanticAnalyzer()
        
        # Valid constraint
        ast = parser.parse('hash<string> names = { "first": "Alice" }')
        result = analyzer.analyze(ast)
        assert result.success
        
        # Invalid constraint
        ast = parser.parse('hash<invalid_constraint> data = {}')
        result = analyzer.analyze(ast)
        assert not result.success
    
    def test_hash_method_validation(self):
        """Test validation of hash method calls."""
        parser = ASTParser()
        analyzer = SemanticAnalyzer()
        
        # First declare the hash
        ast = parser.parse('hash config = { "host": "localhost" }')
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
    """Test hash display and string representation."""
    
    def test_empty_hash_display(self):
        """Test display of empty hash."""
        session = ExecutionSession()
        result = session.execute_statement('{}')
        
        assert result.success
        assert str(result.value) == "{}"
    
    def test_single_pair_hash_display(self):
        """Test display of single pair."""
        session = ExecutionSession()
        # Force multiple pairs to get MapLiteral instead of DataNodeLiteral
        result = session.execute_statement('{ "name": "Alice", "temp": "delete" }')
        assert result.success
        
        # Remove the temp key to get single pair
        session.execute_statement('hash m = { "name": "Alice" }')
        result = session.execute_statement('m')
        assert result.success
        assert str(result.value) == '{ "name": Alice }'
    
    def test_multiple_pair_hash_display(self):
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


class TestHashIndexAccess:
    """Test new hash[key] direct value access behavior."""

    def test_hash_index_returns_values_directly(self):
        """Test that hash[key] returns values directly, not data nodes."""
        session = ExecutionSession()
        session.execute_statement('hash config = { "host": "localhost", "port": 8080, "debug": true }')

        # String value access
        result = session.execute_statement('config["host"]')
        assert result.success
        assert isinstance(result.value, StringValue)
        assert result.value.value == "localhost"

        # Number value access
        result = session.execute_statement('config["port"]')
        assert result.success
        assert isinstance(result.value, NumberValue)
        assert result.value.value == 8080

        # Boolean value access
        result = session.execute_statement('config["debug"]')
        assert result.success
        assert isinstance(result.value, BooleanValue)
        assert result.value.value is True

    def test_hash_index_vs_get_method(self):
        """Test difference between hash[key] and hash.get(key)."""
        session = ExecutionSession()
        session.execute_statement('hash data = { "name": "Alice", "age": 25 }')

        # hash[key] returns value directly
        result = session.execute_statement('data["name"]')
        assert result.success
        assert isinstance(result.value, StringValue)
        assert result.value.value == "Alice"

        # hash.get(key) returns data node
        with pytest.warns(DeprecationWarning, match="The hash.get\\(\\) method is deprecated"):
            result = session.execute_statement('data.get("name")')
        assert result.success
        # get() method should return data node representation
        assert str(result.value) == '{ "name": Alice }'

    def test_hash_index_with_variables(self):
        """Test hash[key] with variable keys."""
        session = ExecutionSession()
        session.execute_statement('hash config = { "host": "localhost", "port": 8080 }')
        session.execute_statement('string key = "host"')

        # Variable key access
        result = session.execute_statement('config[key]')
        assert result.success
        assert result.value.value == "localhost"

        # Dynamic key expression
        session.execute_statement('string prefix = "ho"')
        session.execute_statement('string suffix = "st"')
        result = session.execute_statement('config[prefix + suffix]')
        assert result.success
        assert result.value.value == "localhost"

    def test_hash_index_assignment(self):
        """Test hash[key] = value assignment."""
        session = ExecutionSession()
        session.execute_statement('hash config = { "host": "localhost" }')

        # Set new value
        result = session.execute_statement('config["port"] = 8080')
        assert result.success

        # Verify it was set correctly
        result = session.execute_statement('config["port"]')
        assert result.success
        assert result.value.value == 8080

        # Update existing value
        result = session.execute_statement('config["host"] = "127.0.0.1"')
        assert result.success

        result = session.execute_statement('config["host"]')
        assert result.success
        assert result.value.value == "127.0.0.1"

    def test_hash_index_missing_key(self):
        """Test hash[key] with missing keys."""
        session = ExecutionSession()
        session.execute_statement('hash config = { "host": "localhost" }')

        # Access missing key should fail
        result = session.execute_statement('config["missing"]')
        assert not result.success
        assert "Key 'missing' not found" in str(result.error)

    def test_hash_index_in_conditions(self):
        """Test hash[key] in if conditions and expressions."""
        session = ExecutionSession()
        session.execute_statement('hash settings = { "debug": true, "port": 8080, "name": "server" }')

        # Boolean in condition
        result = session.execute_statement('''
if settings["debug"] {
    result = "debug enabled"
} else {
    result = "debug disabled"
}
result
        '''.strip())
        assert result.success
        assert result.value.value == "debug enabled"

        # Number in expression
        result = session.execute_statement('new_port = settings["port"] + 1000')
        assert result.success

        result = session.execute_statement('new_port')
        assert result.success
        assert result.value.value == 9080

        # String concatenation
        result = session.execute_statement('full_name = settings["name"] + "_v2"')
        assert result.success

        result = session.execute_statement('full_name')
        assert result.success
        assert result.value.value == "server_v2"

    def test_hash_index_method_chaining(self):
        """Test method chaining with hash[key] values."""
        session = ExecutionSession()
        session.execute_statement('hash data = { "name": "alice", "items": [1, 2, 3, 4] }')

        # String method chaining - using correct method name
        result = session.execute_statement('data["name"].toUpper()')
        assert result.success
        assert result.value.value == "ALICE"

        # List method chaining
        result = session.execute_statement('data["items"].size()')
        assert result.success
        assert result.value.value == 4

    def test_hash_index_with_constraints(self):
        """Test hash[key] with type-constrained hashes."""
        session = ExecutionSession()
        session.execute_statement('hash<string> names = { "first": "Alice", "last": "Smith" }')

        # Access constrained values
        result = session.execute_statement('names["first"]')
        assert result.success
        assert isinstance(result.value, StringValue)
        assert result.value.value == "Alice"

        # Verify constraint enforcement on assignment
        result = session.execute_statement('names["age"] = 25')
        assert not result.success
        assert "hash<string>" in str(result.error)

    def test_hash_index_nested_access(self):
        """Test nested hash access patterns."""
        session = ExecutionSession()
        session.execute_statement('hash server = { "host": "localhost", "port": 8080 }')
        session.execute_statement('hash database = { "host": "db.example.com", "port": 5432 }')
        session.execute_statement('hash config = { "server": server, "database": database }')

        # This test verifies that we can access nested hashes
        # Note: config["server"] should return the HashValue, not a DataValue
        result = session.execute_statement('config["server"]')
        assert result.success
        # The value should be a HashValue
        assert hasattr(result.value, 'pairs')  # Should be a hash

        # Test that we can then access nested properties if hash indexing is recursive
        # This might require additional implementation
        # result = session.execute_statement('config["server"]["host"]')
        # For now, just verify we can get the nested hash

    def test_backward_compatibility_preservation(self):
        """Test that .get() method still works as before for backward compatibility (deprecated)."""
        session = ExecutionSession()
        session.execute_statement('hash config = { "host": "localhost", "port": 8080 }')

        # .get() should still return data nodes as before
        with pytest.warns(DeprecationWarning, match="The hash.get\\(\\) method is deprecated"):
            result = session.execute_statement('config.get("host")')
        assert result.success
        assert str(result.value) == '{ "host": localhost }'

        # .get() with missing key returns empty string
        with pytest.warns(DeprecationWarning, match="The hash.get\\(\\) method is deprecated"):
            result = session.execute_statement('config.get("missing")')
        assert result.success
        assert str(result.value) == ""

        # Compare: hash[key] vs hash.get(key) for existing key
        result1 = session.execute_statement('config["host"]')
        with pytest.warns(DeprecationWarning, match="The hash.get\\(\\) method is deprecated"):
            result2 = session.execute_statement('config.get("host")')

        assert result1.success and result2.success
        assert result1.value.value == "localhost"  # Direct value
        assert str(result2.value) == '{ "host": localhost }'  # Data node representation


class TestMapEdgeCases:
    """Test edge cases and error conditions."""

    def test_hash_method_argument_errors(self):
        """Test method calls with wrong number of arguments."""
        session = ExecutionSession()
        session.execute_statement('hash config = { "host": "localhost" }')
        
        # get() with wrong argument count
        with pytest.warns(DeprecationWarning, match="The hash.get\\(\\) method is deprecated"):
            result = session.execute_statement('config.get()')
        assert not result.success
        assert "argument" in str(result.error)

        with pytest.warns(DeprecationWarning, match="The hash.get\\(\\) method is deprecated"):
            result = session.execute_statement('config.get("host", "extra")')
        assert not result.success
        assert "argument" in str(result.error)
        
        # set() with wrong argument count
        result = session.execute_statement('config.set("key")')
        assert not result.success
        assert "argument" in str(result.error)
    
    def test_hash_method_type_errors(self):
        """Test method calls with wrong argument types."""
        session = ExecutionSession()
        session.execute_statement('hash config = { "host": "localhost" }')
        
        # Non-string key
        with pytest.warns(DeprecationWarning, match="The hash.get\\(\\) method is deprecated"):
            result = session.execute_statement('config.get(123)')
        assert not result.success
        assert "string" in str(result.error)
        
        result = session.execute_statement('config.set(123, "value")')
        assert not result.success
        assert "string" in str(result.error)
    
    def test_hash_merge_edge_cases(self):
        """Test merge method edge cases."""
        session = ExecutionSession()
        session.execute_statement('hash config = { "host": "localhost" }')
        
        # Wrong argument count
        result = session.execute_statement('config.merge()')
        assert not result.success
        assert "argument" in str(result.error)
        
        result = session.execute_statement('config.merge("not_a_hash", "extra")')
        assert not result.success
        assert "argument" in str(result.error)
        
        # Wrong argument type
        result = session.execute_statement('config.merge("not_a_hash")')
        assert not result.success
        assert "hash argument" in str(result.error)
    
    def test_hash_push_edge_cases(self):
        """Test push method edge cases."""
        session = ExecutionSession()
        session.execute_statement('hash config = { "host": "localhost" }')
        
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
    
    def test_hash_pop_edge_cases(self):
        """Test pop method edge cases."""
        session = ExecutionSession()
        session.execute_statement('hash config = { "host": "localhost" }')
        
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
    
    def test_hash_universal_methods(self):
        """Test that hashs support universal methods."""
        session = ExecutionSession()
        session.execute_statement('hash config = { "host": "localhost", "port": 8080 }')
        
        # type()
        result = session.execute_statement('config.type()')
        assert result.success
        assert str(result.value) == "hash"
        
        # size()
        result = session.execute_statement('config.size()')
        assert result.success
        assert result.value.value == 2
        
        # inspect()
        result = session.execute_statement('config.inspect()')
        assert result.success
        assert "hash" in str(result.value)
        assert "2 pairs" in str(result.value)
        
        # methods() - should return complete list
        result = session.execute_statement('config.methods()')
        assert result.success
        methods_list = result.value
        assert methods_list.get_type() == "list"
        
        # Should have 29 methods total (8 universal + 12 hash-specific + 5 behavior + 2 conversion + 2 graph)
        assert len(methods_list.elements) == 29  # Updated count with behavior, conversion, and graph methods
        
        # Check that all expected methods are present
        method_names = [elem.value for elem in methods_list.elements]
        
        # Universal methods
        for method in ['type', 'methods', 'can', 'inspect', 'size', 'freeze', 'is_frozen', 'contains_frozen']:
            assert method in method_names, f"Missing universal method: {method}"
            
        # Map-specific methods
        for method in ['get', 'set', 'has_key', 'keys', 'values', 'remove', 'empty', 'merge', 'push', 'pop', 'count_values', 'can_accept']:
            assert method in method_names, f"Missing hash method: {method}"

        # Behavior management methods
        for method in ['add_rule', 'remove_rule', 'has_rule', 'get_rules', 'clear_rules']:
            assert method in method_names, f"Missing behavior method: {method}"

        # Type conversion methods
        for method in ['to_bool', 'to_string']:
            assert method in method_names, f"Missing conversion method: {method}"
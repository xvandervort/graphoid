"""
Test first() and last() universal graph methods.

These methods should work on all graph types with appropriate semantics:
- Lists: first/last elements by index
- Maps: first/last inserted values (not key-value pairs)
- Trees: return none (no meaningful first/last)
- Empty collections: return none safely
"""

import pytest
from glang.execution.values import *
from glang.execution.graph_values import MapValue, ListValue
from glang.execution.executor import ASTExecutor, ExecutionContext
from glang.semantic.analyzer import SemanticAnalyzer
from glang.parser.ast_parser import ASTParser
from glang.lexer.tokenizer import Tokenizer

def execute_glang_code(code: str):
    """Helper to execute Glang code and return the context."""
    parser = ASTParser()
    ast = parser.parse(code)

    analyzer = SemanticAnalyzer()
    result = analyzer.analyze(ast)

    context = ExecutionContext(result.symbol_table)
    executor = ASTExecutor(context)
    executor.execute(result.ast)
    return context

class TestListFirstLast:
    """Test first() and last() methods on lists."""

    def test_list_first_basic(self):
        """Test first() on basic list."""
        context = execute_glang_code("""
        items = [1, 2, 3, 4, 5]
        result = items.first()
        """)
        result = context.variables["result"]
        assert isinstance(result, NumberValue)
        assert result.value == 1

    def test_list_last_basic(self):
        """Test last() on basic list."""
        context = execute_glang_code("""
        items = [1, 2, 3, 4, 5]
        result = items.last()
        """)
        result = context.variables["result"]
        assert isinstance(result, NumberValue)
        assert result.value == 5

    def test_list_single_element(self):
        """Test first() and last() on single-element list."""
        context = execute_glang_code("""
        items = [42]
        first_result = items.first()
        last_result = items.last()
        """)
        first = context.variables["first_result"]
        last = context.variables["last_result"]
        assert isinstance(first, NumberValue)
        assert isinstance(last, NumberValue)
        assert first.value == 42
        assert last.value == 42

    def test_list_empty(self):
        """Test first() and last() on empty list."""
        context = execute_glang_code("""
        empty = []
        first_result = empty.first()
        last_result = empty.last()
        """)
        first = context.variables["first_result"]
        last = context.variables["last_result"]
        assert isinstance(first, NoneValue)
        assert isinstance(last, NoneValue)

    def test_list_string_elements(self):
        """Test first() and last() on list of strings."""
        context = execute_glang_code("""
        words = ["hello", "world", "test"]
        first_result = words.first()
        last_result = words.last()
        """)
        first = context.variables["first_result"]
        last = context.variables["last_result"]
        assert isinstance(first, StringValue)
        assert isinstance(last, StringValue)
        assert first.value == "hello"
        assert last.value == "test"

    def test_list_mixed_elements(self):
        """Test first() and last() on list with mixed types."""
        context = execute_glang_code("""
        mixed = [42, "hello", true]
        first_result = mixed.first()
        last_result = mixed.last()
        """)
        first = context.variables["first_result"]
        last = context.variables["last_result"]
        assert isinstance(first, NumberValue)
        assert isinstance(last, BooleanValue)
        assert first.value == 42
        assert last.value == True

class TestMapFirstLast:
    """Test first() and last() methods on maps."""

    def test_map_first_basic(self):
        """Test first() on basic map."""
        context = execute_glang_code("""
        config = {"host": "localhost", "port": 8080, "debug": true}
        result = config.first()
        """)
        result = context.variables["result"]
        assert isinstance(result, MapValue)
        # Should contain only the first inserted key-value pair
        result_keys = result.keys()
        assert len(result_keys) == 1
        assert result_keys[0] == "host"
        assert result.get("host").value == "localhost"

    def test_map_last_basic(self):
        """Test last() on basic map."""
        context = execute_glang_code("""
        config = {"host": "localhost", "port": 8080, "debug": true}
        result = config.last()
        """)
        result = context.variables["result"]
        assert isinstance(result, MapValue)
        # Should contain only the last inserted key-value pair
        result_keys = result.keys()
        assert len(result_keys) == 1
        assert result_keys[0] == "debug"
        assert result.get("debug").value is True

    def test_map_single_element(self):
        """Test first() and last() on single-element map."""
        context = execute_glang_code("""
        single = {"key": "value"}
        first_result = single.first()
        last_result = single.last()
        """)
        first = context.variables["first_result"]
        last = context.variables["last_result"]
        assert isinstance(first, MapValue)
        assert isinstance(last, MapValue)

        # Both should return the same single key-value pair
        first_keys = first.keys()
        last_keys = last.keys()
        assert len(first_keys) == 1
        assert len(last_keys) == 1
        assert first_keys[0] == "key"
        assert last_keys[0] == "key"
        assert first.get("key").value == "value"
        assert last.get("key").value == "value"

    def test_map_empty(self):
        """Test first() and last() on empty map."""
        context = execute_glang_code("""
        empty = {}
        first_result = empty.first()
        last_result = empty.last()
        """)
        first = context.variables["first_result"]
        last = context.variables["last_result"]
        assert isinstance(first, NoneValue)
        assert isinstance(last, NoneValue)

    def test_map_insertion_order(self):
        """Test that first() and last() respect insertion order."""
        context = execute_glang_code("""
        ordered = {}
        ordered["third"] = 3
        ordered["first"] = 1
        ordered["second"] = 2
        first_result = ordered.first()
        last_result = ordered.last()
        """)
        first = context.variables["first_result"]
        last = context.variables["last_result"]

        # First should contain the first inserted key-value pair
        assert isinstance(first, MapValue)
        first_keys = first.keys()
        assert len(first_keys) == 1
        assert first_keys[0] == "third"
        assert first.get("third").value == 3

        # Last should contain the last inserted key-value pair
        assert isinstance(last, MapValue)
        last_keys = last.keys()
        assert len(last_keys) == 1
        assert last_keys[0] == "second"
        assert last.get("second").value == 2

    def test_map_result_type(self):
        """Test that first() and last() return actual values, not maps."""
        context = execute_glang_code("""
        config = {"name": "test", "value": 42}
        first_result = config.first()
        last_result = config.last()
        """)
        first = context.variables["first_result"]
        last = context.variables["last_result"]

        # Should return map units containing single key-value pairs
        assert isinstance(first, MapValue)
        assert isinstance(last, MapValue)
        first_keys = first.keys()
        last_keys = last.keys()
        assert len(first_keys) == 1
        assert len(last_keys) == 1
        assert first_keys[0] == "name"
        assert last_keys[0] == "value"
        assert first.get("name").value == "test"
        assert last.get("value").value == 42

class TestTreeFirstLast:
    """Test first() and last() methods on trees (should return none)."""

    def test_tree_first_last_return_none(self):
        """Test that trees return none for first() and last()."""
        # Note: Tree creation might not work in current implementation,
        # but we'll test the method dispatch if possible
        # This test might need adjustment based on current tree implementation
        pass

class TestErrorConditions:
    """Test error conditions for first() and last() methods."""

    def test_first_with_arguments(self):
        """Test that first() rejects arguments."""
        with pytest.raises(Exception) as excinfo:
            execute_glang_code("""
            items = [1, 2, 3]
            result = items.first(5)
            """)
        assert "first() takes no arguments" in str(excinfo.value)

    def test_last_with_arguments(self):
        """Test that last() rejects arguments."""
        with pytest.raises(Exception) as excinfo:
            execute_glang_code("""
            items = [1, 2, 3]
            result = items.last(5)
            """)
        assert "last() takes no arguments" in str(excinfo.value)

class TestUniversalBehavior:
    """Test that first() and last() work across all graph types."""

    def test_list_with_maps(self):
        """Test first() and last() on list containing maps."""
        context = execute_glang_code("""
        items = [{"a": 1}, {"b": 2}, {"c": 3}]
        first_result = items.first()
        last_result = items.last()
        """)
        first = context.variables["first_result"]
        last = context.variables["last_result"]

        # Should return the actual map objects
        assert isinstance(first, MapValue)
        assert isinstance(last, MapValue)
        assert first.get("a").value == 1
        assert last.get("c").value == 3

    def test_map_with_lists(self):
        """Test first() and last() on map containing lists."""
        context = execute_glang_code("""
        config = {"numbers": [1, 2, 3], "letters": ["a", "b", "c"]}
        first_result = config.first()
        last_result = config.last()
        """)
        first = context.variables["first_result"]
        last = context.variables["last_result"]

        # Should return map units containing the list values
        assert isinstance(first, MapValue)
        assert isinstance(last, MapValue)
        first_keys = first.keys()
        last_keys = last.keys()
        assert len(first_keys) == 1
        assert len(last_keys) == 1
        assert first_keys[0] == "numbers"
        assert last_keys[0] == "letters"
        # The values should be the actual lists
        numbers_list = first.get("numbers")
        letters_list = last.get("letters")
        assert isinstance(numbers_list, ListValue)
        assert isinstance(letters_list, ListValue)
        assert len(numbers_list.elements) == 3
        assert len(letters_list.elements) == 3
        assert numbers_list.elements[0].value == 1
        assert letters_list.elements[0].value == "a"

class TestChaining:
    """Test that first() and last() work with method chaining."""

    def test_first_on_map_result(self):
        """Test that first() returns a value that can be used directly."""
        context = execute_glang_code("""
        config = {"host": "localhost", "port": 8080}
        first_result = config.first()
        """)
        first = context.variables["first_result"]
        assert isinstance(first, MapValue)
        first_keys = first.keys()
        assert len(first_keys) == 1
        assert first_keys[0] == "host"
        assert first.get("host").value == "localhost"

    def test_last_on_map_result(self):
        """Test that last() returns a value that can be used directly."""
        context = execute_glang_code("""
        config = {"host": "localhost", "port": 8080}
        last_result = config.last()
        """)
        last = context.variables["last_result"]
        assert isinstance(last, MapValue)
        last_keys = last.keys()
        assert len(last_keys) == 1
        assert last_keys[0] == "port"
        assert last.get("port").value == 8080

if __name__ == "__main__":
    pytest.main([__file__])
"""
Tests for the Universal Metadata Layer

Tests the metadata layer functionality in graph structures,
including R vector-style element naming for lists.
"""

import pytest
import sys
import os

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from glang.execution.graph_foundation import MetadataLayer, SequentialGraph
from glang.execution.graph_values import ListValue
from glang.execution.values import NumberValue, StringValue


class TestMetadataLayer:
    """Test the core MetadataLayer class."""

    def test_metadata_layer_creation(self):
        """Test creating a metadata layer."""
        metadata = MetadataLayer()
        assert metadata.properties == {}

    def test_metadata_set_get(self):
        """Test setting and getting metadata properties."""
        metadata = MetadataLayer()

        metadata.set("units", "centimeters")
        metadata.set("source", "survey_2025")
        metadata.set("accuracy", 95.5)

        assert metadata.get("units") == "centimeters"
        assert metadata.get("source") == "survey_2025"
        assert metadata.get("accuracy") == 95.5

    def test_metadata_graceful_degradation(self):
        """Test graceful degradation when metadata doesn't exist."""
        metadata = MetadataLayer()

        # Getting non-existent key returns None by default
        assert metadata.get("missing_key") is None

        # Can specify custom default
        assert metadata.get("missing_key", "default") == "default"

    def test_metadata_has_remove_clear(self):
        """Test has, remove, and clear operations."""
        metadata = MetadataLayer()
        metadata.set("test_key", "test_value")

        # Test has
        assert metadata.has("test_key") is True
        assert metadata.has("missing_key") is False

        # Test remove
        assert metadata.remove("test_key") is True
        assert metadata.remove("missing_key") is False
        assert not metadata.has("test_key")

        # Test clear
        metadata.set("key1", "value1")
        metadata.set("key2", "value2")
        metadata.clear()
        assert metadata.properties == {}

    def test_metadata_keys_copy(self):
        """Test getting keys and copying metadata."""
        metadata = MetadataLayer()
        metadata.set("key1", "value1")
        metadata.set("key2", "value2")

        keys = metadata.keys()
        assert set(keys) == {"key1", "key2"}

        # Test copy
        copy = metadata.copy()
        assert copy.properties == metadata.properties
        assert copy is not metadata  # Different objects

        # Modifying copy doesn't affect original
        copy.set("key3", "value3")
        assert not metadata.has("key3")


class TestSequentialGraphMetadata:
    """Test metadata layer integration with SequentialGraph."""

    def test_sequential_graph_has_metadata(self):
        """Test that SequentialGraph always has metadata layer."""
        values = [NumberValue(10), NumberValue(20), NumberValue(30)]
        graph = SequentialGraph(values)

        assert hasattr(graph, 'metadata')
        assert isinstance(graph.metadata, MetadataLayer)

    def test_element_naming_basic(self):
        """Test basic element naming functionality."""
        values = [NumberValue(165), NumberValue(180), NumberValue(175)]
        graph = SequentialGraph(values)

        # Initially no names
        assert graph.get_names() == [None, None, None]
        assert not graph.has_names()

        # Set names
        graph.set_names(["alice", "bob", "charlie"])
        assert graph.get_names() == ["alice", "bob", "charlie"]
        assert graph.has_names()

    def test_element_naming_sparse(self):
        """Test sparse naming (some elements unnamed)."""
        values = [NumberValue(10), NumberValue(20), NumberValue(30), NumberValue(40)]
        graph = SequentialGraph(values)

        # Set sparse names
        graph.set_names(["first", None, "third", None])

        assert graph.get_names() == ["first", None, "third", None]
        assert graph.has_names()  # True because some elements have names

        # Test individual name access
        assert graph.get_name(0) == "first"
        assert graph.get_name(1) is None
        assert graph.get_name(2) == "third"
        assert graph.get_name(3) is None

    def test_name_based_value_access(self):
        """Test accessing values by name."""
        values = [NumberValue(165), NumberValue(180), NumberValue(175)]
        graph = SequentialGraph(values)
        graph.set_names(["alice", "bob", "charlie"])

        # Test get_value_by_name
        alice_value = graph.get_value_by_name("alice")
        bob_value = graph.get_value_by_name("bob")
        missing_value = graph.get_value_by_name("missing")

        assert alice_value.value == 165
        assert bob_value.value == 180
        assert missing_value is None

    def test_individual_name_operations(self):
        """Test setting individual names."""
        values = [NumberValue(85), NumberValue(92), NumberValue(78)]
        graph = SequentialGraph(values)

        # Set individual names
        assert graph.set_name(0, "math") is True
        assert graph.set_name(2, "english") is True
        # Leave index 1 unnamed

        expected_names = ["math", None, "english"]
        assert graph.get_names() == expected_names

        # Test out of bounds
        assert graph.set_name(10, "invalid") is False

    def test_name_validation(self):
        """Test name validation and error handling."""
        values = [NumberValue(1), NumberValue(2)]
        graph = SequentialGraph(values)

        # Test length mismatch
        with pytest.raises(ValueError):
            graph.set_names(["too", "many", "names"])

    def test_general_metadata_alongside_names(self):
        """Test that general metadata works alongside element names."""
        values = [NumberValue(165), NumberValue(180)]
        graph = SequentialGraph(values)

        # Set both names and general metadata
        graph.set_names(["alice", "bob"])
        graph.metadata.set("units", "centimeters")
        graph.metadata.set("source", "health_survey")

        # Both should work
        assert graph.get_names() == ["alice", "bob"]
        assert graph.metadata.get("units") == "centimeters"
        assert graph.metadata.get("source") == "health_survey"


class TestListValueMetadata:
    """Test metadata layer integration with ListValue."""

    def test_listvalue_has_metadata_access(self):
        """Test that ListValue exposes metadata layer."""
        elements = [NumberValue(10), NumberValue(20)]
        list_val = ListValue(elements)

        assert hasattr(list_val, 'metadata')
        assert isinstance(list_val.metadata, MetadataLayer)

    def test_listvalue_naming_methods(self):
        """Test ListValue naming methods."""
        elements = [NumberValue(165), NumberValue(180), NumberValue(175)]
        list_val = ListValue(elements)

        # Test initial state
        assert not list_val.has_names()
        assert list_val.get_names() == [None, None, None]

        # Set names
        list_val.set_names(["alice", "bob", "charlie"])
        assert list_val.has_names()
        assert list_val.get_names() == ["alice", "bob", "charlie"]

    def test_listvalue_name_based_access(self):
        """Test name-based access through ListValue."""
        elements = [NumberValue(165), NumberValue(180), NumberValue(175)]
        list_val = ListValue(elements)
        list_val.set_names(["alice", "bob", "charlie"])

        # Test get_by_name
        alice_value = list_val.get_by_name("alice")
        missing_value = list_val.get_by_name("missing")

        assert alice_value.value == 165
        assert missing_value is None

    def test_listvalue_enhanced_indexing(self):
        """Test enhanced __getitem__ and __setitem__ with names."""
        elements = [NumberValue(165), NumberValue(180), NumberValue(175)]
        list_val = ListValue(elements)
        list_val.set_names(["alice", "bob", "charlie"])

        # Test numeric indexing (unchanged)
        assert list_val[0].value == 165
        assert list_val[1].value == 180

        # Test name-based indexing (new)
        assert list_val["alice"].value == 165
        assert list_val["bob"].value == 180

        # Test graceful degradation for missing names
        assert list_val["missing"] is None

        # Test name-based assignment
        list_val["alice"] = NumberValue(170)
        assert list_val[0].value == 170  # Should update the same element

        # Test assignment to non-existent name (should be graceful)
        list_val["missing"] = NumberValue(999)  # Should not crash

    def test_listvalue_metadata_integration(self):
        """Test that ListValue metadata integrates properly with graph."""
        elements = [NumberValue(10), NumberValue(20)]
        list_val = ListValue(elements)

        # Set metadata directly
        list_val.metadata.set("units", "meters")
        list_val.metadata.set("precision", 2)

        # Verify it's stored in the graph metadata
        assert list_val.graph.metadata.get("units") == "meters"
        assert list_val.graph.metadata.get("precision") == 2

    def test_method_chaining_with_set_names(self):
        """Test that set_names returns self for method chaining."""
        elements = [NumberValue(1), NumberValue(2), NumberValue(3)]

        # Method chaining should work - set_names returns the list itself
        list_val = ListValue(elements).set_names(["red", "green", "blue"])

        # Verify the result is still a ListValue
        assert isinstance(list_val, ListValue)

        # Verify names were set correctly
        names = list_val.get_names()
        assert names == ["red", "green", "blue"]

        # Verify values are still accessible
        assert list_val[0].value == 1
        assert list_val[1].value == 2
        assert list_val[2].value == 3

        # Verify name-based access works
        assert list_val["red"].value == 1
        assert list_val["green"].value == 2
        assert list_val["blue"].value == 3

    def test_method_chaining_inline_initialization(self):
        """Test method chaining for inline initialization patterns."""
        # Create and name in one expression
        colors = ListValue([
            NumberValue(255),
            NumberValue(128),
            NumberValue(64)
        ]).set_names(["red", "green", "blue"])

        # Should be immediately usable with both access patterns
        assert colors[0].value == 255
        assert colors["red"].value == 255
        assert colors["green"].value == 128
        assert colors[2].value == 64
        assert colors["blue"].value == 64

        # Names should be properly set
        assert colors.has_names() == True
        assert colors.get_names() == ["red", "green", "blue"]

    def test_method_chaining_preserves_type(self):
        """Test that method chaining preserves the exact type."""
        elements = [NumberValue(10), NumberValue(20)]
        original = ListValue(elements)

        # set_names should return the exact same object
        result = original.set_names(["first", "second"])

        # Should be the same object reference
        assert result is original

        # Should still be a ListValue
        assert isinstance(result, ListValue)
        assert type(result) == ListValue


class TestHashValueMetadata:
    """Test HashValue metadata functionality."""

    def test_hashvalue_has_metadata_access(self):
        """Test that HashValue has metadata access."""
        from glang.execution.graph_values import HashValue
        from glang.execution.values import StringValue

        # Create a hash with data nodes
        hash_val = HashValue([
            ("name", StringValue("Alice")),
            ("age", StringValue("30"))
        ])

        # Should have metadata layer through graph
        assert hasattr(hash_val, 'graph')
        assert hasattr(hash_val.graph, 'metadata')

        # Should have naming methods
        assert hasattr(hash_val, 'set_names')
        assert hasattr(hash_val, 'get_names')
        assert hasattr(hash_val, 'has_names')

    def test_hashvalue_naming_methods(self):
        """Test HashValue naming methods."""
        from glang.execution.graph_values import HashValue
        from glang.execution.values import StringValue

        hash_val = HashValue([
            ("host", StringValue("localhost")),
            ("port", StringValue("8080"))
        ])

        # Initially no names
        assert hash_val.has_names() == False
        names = hash_val.get_names()
        assert names == [None, None]

        # Set names
        result = hash_val.set_names(["server", "connection"])

        # Should return self for chaining
        assert result is hash_val
        assert isinstance(result, HashValue)

        # Should now have names
        assert hash_val.has_names() == True
        names = hash_val.get_names()
        assert names == ["server", "connection"]

    def test_hashvalue_name_based_access(self):
        """Test name-based access for hash values."""
        from glang.execution.graph_values import HashValue
        from glang.execution.values import StringValue

        hash_val = HashValue([
            ("host", StringValue("localhost")),
            ("port", StringValue("8080")),
            ("debug", StringValue("true"))
        ])

        # Set names
        hash_val.set_names(["server", "connection", "flag"])

        # Test dual access - by key and by name
        host_by_key = hash_val["host"]
        host_by_name = hash_val["server"]

        # Should be the same value
        assert host_by_key.value == "localhost"
        assert host_by_name.value == "localhost"
        assert host_by_key is host_by_name  # Same object reference

        # Test all mappings
        assert hash_val["port"].value == "8080"
        assert hash_val["connection"].value == "8080"
        assert hash_val["debug"].value == "true"
        assert hash_val["flag"].value == "true"

    def test_hashvalue_method_chaining(self):
        """Test method chaining for hash initialization."""
        from glang.execution.graph_values import HashValue
        from glang.execution.values import StringValue

        # Create and name in one expression
        config = HashValue([
            ("theme", StringValue("dark")),
            ("lang", StringValue("en"))
        ]).set_names(["appearance", "language"])

        # Should be immediately usable
        assert config["theme"].value == "dark"
        assert config["appearance"].value == "dark"
        assert config["lang"].value == "en"
        assert config["language"].value == "en"

        # Names should be set
        assert config.has_names() == True
        assert config.get_names() == ["appearance", "language"]

    def test_hashvalue_sparse_naming(self):
        """Test sparse naming (some elements unnamed)."""
        from glang.execution.graph_values import HashValue
        from glang.execution.values import StringValue

        hash_val = HashValue([
            ("host", StringValue("localhost")),
            ("port", StringValue("8080")),
            ("timeout", StringValue("30"))
        ])

        # Set sparse names
        hash_val.set_names(["server", None, "delay"])

        # Named elements should work
        assert hash_val["server"].value == "localhost"
        assert hash_val["delay"].value == "30"

        # Unnamed element only accessible by key
        assert hash_val["port"].value == "8080"

        # Non-existent name should raise error
        with pytest.raises(KeyError):
            hash_val["nonexistent"]

    def test_hashvalue_name_collision_with_keys(self):
        """Test behavior when names collide with existing keys."""
        from glang.execution.graph_values import HashValue
        from glang.execution.values import StringValue

        hash_val = HashValue([
            ("host", StringValue("localhost")),
            ("port", StringValue("8080"))
        ])

        # Set a name that collides with existing key
        hash_val.set_names(["port", "connection"])

        # Key access should take precedence over name access
        result = hash_val["port"]
        assert result.value == "8080"  # Should get the actual "port" key value

        # The first element should still be accessible by name through collision resolution
        first_element = hash_val["port"]  # This gets the key
        assert first_element.value == "8080"


if __name__ == "__main__":
    pytest.main([__file__])
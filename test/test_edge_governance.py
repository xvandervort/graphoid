"""
Tests for the Edge Governance System (Control Layer - Layer 3)

This test module verifies that the control layer properly enforces edge rules
to prevent dangerous graph operations like cycles and cross-contamination.
"""

import pytest
import sys
import os

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from glang.execution.graph_values import ListValue, HashValue
from glang.execution.values import NumberValue, StringValue
from glang.execution.control_layer import RuleViolationError, ControlLayer
from glang.execution.graph_foundation import GraphStructure, EdgeMetadata, EdgeType


class TestControlLayer:
    """Test the core ControlLayer class."""

    def test_control_layer_creation(self):
        """Test creating a control layer."""
        graph = GraphStructure()
        control = ControlLayer(graph)

        assert control.parent_graph is graph
        assert len(control.disabled_rules) == 0
        assert control.custom_rules is None  # Lazy creation

    def test_get_active_rules(self):
        """Test getting active rules."""
        graph = GraphStructure()
        control = ControlLayer(graph)

        active_rules = control.get_active_rules()
        assert "no_list_cycles" in active_rules
        assert "same_structure_only" in active_rules

    def test_rule_status_checking(self):
        """Test checking rule status."""
        graph = GraphStructure()
        control = ControlLayer(graph)

        assert control.get_rule_status("no_list_cycles") == "active"
        assert control.get_rule_status("same_structure_only") == "active"
        assert control.get_rule_status("nonexistent_rule") == "unknown"

    def test_rule_disabling_and_enabling(self):
        """Test disabling and re-enabling rules."""
        graph = GraphStructure()
        control = ControlLayer(graph)

        # Initially active
        assert control.get_rule_status("no_list_cycles") == "active"

        # Disable
        control.disable_rule("no_list_cycles")
        assert control.get_rule_status("no_list_cycles") == "disabled"
        assert "no_list_cycles" not in control.get_active_rules()

        # Re-enable
        control.enable_rule("no_list_cycles")
        assert control.get_rule_status("no_list_cycles") == "active"
        assert "no_list_cycles" in control.get_active_rules()


class TestEdgeGovernanceListOperations:
    """Test edge governance for list operations."""

    def test_valid_edge_allowed(self):
        """Test that valid edges are allowed."""
        numbers = ListValue([NumberValue(1), NumberValue(2), NumberValue(3)])

        # Valid forward edge should work
        success = numbers.add_edge(0, 1, "friend")
        assert success is True

        edges = numbers.get_edges()
        assert len(edges) == 1
        assert edges[0] == (0, 1, "friend")

    def test_cycle_prevention(self):
        """Test that cycles are prevented."""
        numbers = ListValue([NumberValue(1), NumberValue(2), NumberValue(3)])

        # Cycle edge should be blocked
        with pytest.raises(RuleViolationError) as exc_info:
            numbers.add_edge(2, 0, "circular")

        assert "no_list_cycles" in str(exc_info.value)
        assert "cycle" in str(exc_info.value).lower()

    def test_backward_edge_prevention(self):
        """Test that backward edges are prevented (simple cycle check)."""
        numbers = ListValue([NumberValue(1), NumberValue(2), NumberValue(3), NumberValue(4)])

        # All these should be blocked as they go backward in sequence
        with pytest.raises(RuleViolationError):
            numbers.add_edge(3, 1, "backward")

        with pytest.raises(RuleViolationError):
            numbers.add_edge(2, 1, "backward")

        with pytest.raises(RuleViolationError):
            numbers.add_edge(1, 0, "backward")

    def test_edge_inspection_methods(self):
        """Test edge inspection methods."""
        numbers = ListValue([NumberValue(1), NumberValue(2), NumberValue(3)])

        # Initially no edges
        assert numbers.get_edge_count() == 0
        assert numbers.get_edges() == []

        # Add some edges
        numbers.add_edge(0, 1, "friend")
        numbers.add_edge(0, 2, "colleague")

        # Check edges
        assert numbers.get_edge_count() == 2
        edges = numbers.get_edges()
        assert len(edges) == 2
        assert (0, 1, "friend") in edges
        assert (0, 2, "colleague") in edges

    def test_can_add_edge_validation(self):
        """Test the can_add_edge validation method."""
        numbers = ListValue([NumberValue(1), NumberValue(2), NumberValue(3)])

        # Valid edge
        can_add, reason = numbers.can_add_edge(0, 1, "friend")
        assert can_add is True

        # Invalid cycle edge
        can_add, reason = numbers.can_add_edge(2, 0, "cycle")
        assert can_add is False
        assert "cycle" in reason.lower()

        # Invalid index
        can_add, reason = numbers.can_add_edge(0, 10, "invalid")
        assert can_add is False

    def test_rule_disabling_allows_dangerous_operations(self):
        """Test that disabling rules allows dangerous operations."""
        numbers = ListValue([NumberValue(1), NumberValue(2), NumberValue(3)])

        # Initially blocked
        with pytest.raises(RuleViolationError):
            numbers.add_edge(2, 0, "cycle")

        # Disable the rule
        numbers.graph.control_layer.disable_rule("no_list_cycles")

        # Now allowed
        success = numbers.add_edge(2, 0, "now_allowed")
        assert success is True


class TestCrossStructureProtection:
    """Test cross-structure contamination prevention."""

    def test_cross_structure_edge_blocked(self):
        """Test that edges between different structures are blocked."""
        list1 = ListValue([NumberValue(10), NumberValue(20)])
        list2 = ListValue([NumberValue(30), NumberValue(40)])

        # Get nodes from different structures
        node1 = list1.get_node_at_index(0)
        node2 = list2.get_node_at_index(0)

        assert node1 is not None
        assert node2 is not None

        # Try to create cross-structure edge
        metadata = EdgeMetadata(EdgeType.NAMED, key="dangerous")

        with pytest.raises(RuleViolationError) as exc_info:
            node1.add_edge_to(node2, metadata)

        assert "same_structure_only" in str(exc_info.value)
        assert "different graph structures" in str(exc_info.value)

    def test_same_structure_edge_allowed(self):
        """Test that edges within the same structure are allowed."""
        numbers = ListValue([NumberValue(1), NumberValue(2), NumberValue(3)])

        node1 = numbers.get_node_at_index(0)
        node2 = numbers.get_node_at_index(1)

        # This should work fine
        metadata = EdgeMetadata(EdgeType.NAMED, key="same_structure")
        node1.add_edge_to(node2, metadata)  # Should not raise


class TestHashValueEdgeGovernance:
    """Test edge governance for hash values."""

    def test_hash_has_control_layer(self):
        """Test that hash values have control layer."""
        hash_val = HashValue([("key1", StringValue("value1"))])

        assert hasattr(hash_val.graph, 'control_layer')
        assert isinstance(hash_val.graph.control_layer, ControlLayer)

    def test_hash_cross_structure_protection(self):
        """Test cross-structure protection for hashes."""
        hash1 = HashValue([("key1", StringValue("value1"))])
        hash2 = HashValue([("key2", StringValue("value2"))])

        # Get nodes from different hash structures
        node1 = hash1.get_value_node("key1")
        node2 = hash2.get_value_node("key2")

        assert node1 is not None
        assert node2 is not None

        # Cross-structure edge should be blocked
        metadata = EdgeMetadata(EdgeType.NAMED, key="cross_hash")

        with pytest.raises(RuleViolationError) as exc_info:
            node1.add_edge_to(node2, metadata)

        assert "same_structure_only" in str(exc_info.value)


class TestRuleValidationLogic:
    """Test the individual rule validation logic."""

    def test_no_list_cycles_rule_logic(self):
        """Test the no_list_cycles rule validation logic."""
        from glang.execution.control_layer import ControlLayer

        # Create a list structure
        numbers = ListValue([NumberValue(1), NumberValue(2), NumberValue(3)])
        context = {
            'graph_type': 'list',
            'parent_graph': numbers.graph
        }

        node0 = numbers.get_node_at_index(0)
        node1 = numbers.get_node_at_index(1)
        node2 = numbers.get_node_at_index(2)

        metadata = EdgeMetadata(EdgeType.NAMED, key="test")

        # Forward edges should be allowed
        is_valid, reason = ControlLayer._validate_no_list_cycles(node0, node1, metadata, context)
        assert is_valid is True

        is_valid, reason = ControlLayer._validate_no_list_cycles(node1, node2, metadata, context)
        assert is_valid is True

        # Backward edges should be blocked
        is_valid, reason = ControlLayer._validate_no_list_cycles(node2, node0, metadata, context)
        assert is_valid is False
        assert "cycle" in reason.lower()

        is_valid, reason = ControlLayer._validate_no_list_cycles(node1, node0, metadata, context)
        assert is_valid is False

    def test_same_structure_rule_logic(self):
        """Test the same_structure_only rule validation logic."""
        from glang.execution.control_layer import ControlLayer

        list1 = ListValue([NumberValue(1), NumberValue(2)])
        list2 = ListValue([NumberValue(3), NumberValue(4)])

        node1_from_list1 = list1.get_node_at_index(0)
        node2_from_list1 = list1.get_node_at_index(1)
        node1_from_list2 = list2.get_node_at_index(0)

        metadata = EdgeMetadata(EdgeType.NAMED, key="test")
        context = {'nodes': list1.graph.nodes}

        # Same structure should be allowed
        is_valid, reason = ControlLayer._validate_same_structure_only(
            node1_from_list1, node2_from_list1, metadata, context)
        assert is_valid is True

        # Different structures should be blocked
        is_valid, reason = ControlLayer._validate_same_structure_only(
            node1_from_list1, node1_from_list2, metadata, context)
        assert is_valid is False
        assert "different graph structures" in reason


if __name__ == "__main__":
    pytest.main([__file__])
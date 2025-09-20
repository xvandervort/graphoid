"""
Tests for Binary Tree Implementation

This test module verifies that binary trees work correctly with the edge governance
system, providing safe tree operations while hiding edge complexity from users.
"""

import pytest
import sys
import os

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from glang.execution.tree_structures import BinaryTreeValue
from glang.execution.values import NumberValue, StringValue
from glang.execution.control_layer import RuleViolationError


class TestBinaryTreeBasics:
    """Test basic binary tree operations."""

    def test_tree_creation(self):
        """Test creating an empty binary tree."""
        tree = BinaryTreeValue()

        assert tree.get_type() == "tree"
        assert tree.empty().value is True
        assert tree.size().value == 0
        assert tree.height().value == 0

    def test_tree_with_constraint(self):
        """Test creating a type-constrained tree."""
        tree = BinaryTreeValue(constraint="num")

        assert tree.constraint == "num"
        assert tree.empty().value is True

    def test_single_insertion(self):
        """Test inserting a single value."""
        tree = BinaryTreeValue()

        tree.insert(NumberValue(10))

        assert tree.empty().value is False
        assert tree.size().value == 1
        assert tree.height().value == 1
        assert tree.search(NumberValue(10)).value is True
        assert tree.search(NumberValue(5)).value is False

    def test_multiple_insertions(self):
        """Test inserting multiple values in BST order."""
        tree = BinaryTreeValue()

        # Insert values to create a balanced tree
        tree.insert(NumberValue(10))
        tree.insert(NumberValue(5))
        tree.insert(NumberValue(15))
        tree.insert(NumberValue(3))
        tree.insert(NumberValue(7))

        assert tree.size().value == 5
        assert tree.height().value == 3

        # Check all values are searchable
        assert tree.search(NumberValue(10)).value is True
        assert tree.search(NumberValue(5)).value is True
        assert tree.search(NumberValue(15)).value is True
        assert tree.search(NumberValue(3)).value is True
        assert tree.search(NumberValue(7)).value is True
        assert tree.search(NumberValue(1)).value is False


class TestTreeTraversals:
    """Test tree traversal methods."""

    def test_in_order_traversal(self):
        """Test in-order traversal gives sorted sequence."""
        tree = BinaryTreeValue()

        # Insert values in random order
        values = [10, 5, 15, 3, 7, 12, 18]
        for val in values:
            tree.insert(NumberValue(val))

        # In-order should give sorted sequence
        result = tree.in_order()
        result_values = [item.value for item in result.elements]

        assert result_values == [3, 5, 7, 10, 12, 15, 18]

    def test_pre_order_traversal(self):
        """Test pre-order traversal gives root-first sequence."""
        tree = BinaryTreeValue()

        # Insert in specific order to predict pre-order
        tree.insert(NumberValue(10))
        tree.insert(NumberValue(5))
        tree.insert(NumberValue(15))

        result = tree.pre_order()
        result_values = [item.value for item in result.elements]

        # Pre-order: root first, then left subtree, then right subtree
        assert result_values == [10, 5, 15]

    def test_post_order_traversal(self):
        """Test post-order traversal gives children-first sequence."""
        tree = BinaryTreeValue()

        tree.insert(NumberValue(10))
        tree.insert(NumberValue(5))
        tree.insert(NumberValue(15))

        result = tree.post_order()
        result_values = [item.value for item in result.elements]

        # Post-order: children first, then root
        assert result_values == [5, 15, 10]

    def test_empty_tree_traversals(self):
        """Test that traversals work on empty trees."""
        tree = BinaryTreeValue()

        assert len(tree.in_order().elements) == 0
        assert len(tree.pre_order().elements) == 0
        assert len(tree.post_order().elements) == 0


class TestTreeConstraints:
    """Test type constraint enforcement."""

    def test_constraint_enforcement(self):
        """Test that constrained trees enforce types."""
        tree = BinaryTreeValue(constraint="num")

        # This should work
        tree.insert(NumberValue(10))
        tree.insert(NumberValue(5))

        # This should fail
        with pytest.raises(RuntimeError) as exc_info:
            tree.insert(StringValue("hello"))

        assert "does not match tree constraint" in str(exc_info.value)

    def test_unconstrained_tree_accepts_mixed_types(self):
        """Test that unconstrained trees accept any types."""
        tree = BinaryTreeValue()

        # Should be able to insert different types
        tree.insert(NumberValue(10))
        tree.insert(StringValue("hello"))
        tree.insert(NumberValue(5))

        assert tree.size().value == 3


class TestTreeEdgeGovernance:
    """Test edge governance integration."""

    def test_tree_has_governance_rules(self):
        """Test that trees have proper governance rules."""
        tree = BinaryTreeValue()

        active_rules = tree.get_active_rules()

        # Trees should have all base rules plus tree-specific rules
        assert "no_list_cycles" in active_rules
        assert "same_structure_only" in active_rules
        assert "max_children_two" in active_rules
        assert "tree_hierarchy" in active_rules
        assert "no_tree_cycles" in active_rules

    def test_rule_status_checking(self):
        """Test checking rule status."""
        tree = BinaryTreeValue()

        assert tree.get_rule_status("max_children_two") == "active"
        assert tree.get_rule_status("tree_hierarchy") == "active"
        assert tree.get_rule_status("nonexistent_rule") == "unknown"

    def test_rule_management(self):
        """Test enabling and disabling rules."""
        tree = BinaryTreeValue()

        # Initially active
        assert tree.get_rule_status("max_children_two") == "active"

        # Disable
        tree.disable_rule("max_children_two")
        assert tree.get_rule_status("max_children_two") == "disabled"

        # Re-enable
        tree.enable_rule("max_children_two")
        assert tree.get_rule_status("max_children_two") == "active"


class TestTreeVisualization:
    """Test tree visualization methods."""

    def test_graph_summary(self):
        """Test getting tree structure summary."""
        tree = BinaryTreeValue()
        tree.insert(NumberValue(10))
        tree.insert(NumberValue(5))

        summary = tree.get_graph_summary()

        assert summary['type'] == 'tree'
        assert summary['node_count'] == 2
        assert summary['edge_count'] == 1  # One parent-child relationship
        assert len(summary['active_rules']) >= 5  # At least 5 rules active

    def test_visualization_formats(self):
        """Test different visualization formats."""
        tree = BinaryTreeValue()
        tree.insert(NumberValue(10))
        tree.insert(NumberValue(5))

        # Text format
        text_viz = tree.visualize_structure("text")
        assert "Type: tree" in text_viz
        assert "Nodes: 2" in text_viz

        # DOT format
        dot_viz = tree.visualize_structure("dot")
        assert "digraph GraphStructure" in dot_viz

        # Summary format
        summary_viz = tree.visualize_structure("summary")
        assert "[TREE]" in summary_viz


class TestTreeStringRepresentation:
    """Test string representation methods."""

    def test_empty_tree_display(self):
        """Test display of empty tree."""
        tree = BinaryTreeValue()

        display = tree.to_display_string()
        assert display == "tree{}"

    def test_populated_tree_display(self):
        """Test display of populated tree."""
        tree = BinaryTreeValue()
        tree.insert(NumberValue(10))
        tree.insert(NumberValue(5))

        display = tree.to_display_string()
        assert "tree" in display
        assert "(2 nodes)" in display

    def test_constrained_tree_display(self):
        """Test display of constrained tree."""
        tree = BinaryTreeValue(constraint="num")
        tree.insert(NumberValue(10))

        display = tree.to_display_string()
        assert "tree<num>" in display
        assert "(1 nodes)" in display


class TestTreePythonConversion:
    """Test conversion to Python data structures."""

    def test_empty_tree_to_python(self):
        """Test converting empty tree to Python."""
        tree = BinaryTreeValue()

        result = tree.to_python()
        assert result == {}

    def test_simple_tree_to_python(self):
        """Test converting simple tree to Python."""
        tree = BinaryTreeValue()
        tree.insert(NumberValue(10))
        tree.insert(NumberValue(5))
        tree.insert(NumberValue(15))

        result = tree.to_python()

        # Should have root with left and right children
        assert result['value'] == 10
        assert result['left']['value'] == 5
        assert result['right']['value'] == 15
        assert 'left' not in result['left']  # Leaf nodes have no children
        assert 'right' not in result['right']


if __name__ == "__main__":
    pytest.main([__file__])
"""
Binary Tree Implementation for Glang

This module provides BinaryTreeValue as a true graph-based tree structure that uses
the edge governance system for safe operations while providing high-level tree methods.

The tree uses specialized rules:
- no_tree_cycles: Prevents cycles (more strict than list cycles)
- tree_hierarchy: Enforces proper parent-child relationships
- max_children_two: Each node can have at most 2 children
"""

from typing import List, Optional, Any, Tuple, Union
from .values import GlangValue, NumberValue, StringValue, BooleanValue, NoneValue
from .graph_foundation import GraphStructure, GraphNode, EdgeType, EdgeMetadata
from .control_layer import ControlLayer, EdgeRule, RuleViolationError
from ..ast.nodes import SourcePosition
from ..graph_container import GraphContainer


class TreeGraph(GraphStructure):
    """Specialized graph structure for binary trees."""

    def __init__(self):
        super().__init__()
        # Tree-specific attributes
        self.root_node_id: Optional[str] = None
        self.size = 0

        # Add tree-specific rules to control layer
        self.control_layer._add_tree_rules()

    def set_root(self, node_id: str) -> None:
        """Set the root node of the tree."""
        self.root_node_id = node_id

    def get_root(self) -> Optional[GraphNode]:
        """Get the root node of the tree."""
        if self.root_node_id:
            return self.nodes.get(self.root_node_id)
        return None


class BinaryTreeValue(GlangValue, GraphContainer):
    """Binary tree implementation using graph foundation with edge governance."""

    def __init__(self, constraint: Optional[str] = None, position: Optional[SourcePosition] = None):
        GlangValue.__init__(self, position)
        GraphContainer.__init__(self)

        self.constraint = constraint
        self.graph = TreeGraph()

        # Configure for tree structures (strict hierarchy)
        self.graph.control_layer.configure_for_tree_structures()

    def get_type(self) -> str:
        return "tree"

    def to_display_string(self) -> str:
        if self.graph.size == 0:
            return "tree{}"

        constraint_info = f'<{self.constraint}>' if self.constraint else ''
        return f'tree{constraint_info} ({self.graph.size} nodes)'

    def to_python(self) -> dict:
        """Convert tree to Python dict representation."""
        root = self.graph.get_root()
        if not root:
            return {}

        def node_to_dict(node: GraphNode) -> dict:
            result = {"value": node.value.to_python() if node.value else None}

            # Find left and right children
            left_edge = node.get_edge_by_key("left")
            right_edge = node.get_edge_by_key("right")

            if left_edge:
                left_child, _ = left_edge
                result["left"] = node_to_dict(left_child)
            if right_edge:
                right_child, _ = right_edge
                result["right"] = node_to_dict(right_child)

            return result

        return node_to_dict(root)

    # Tree operations
    def insert(self, value: GlangValue) -> 'NoneValue':
        """Insert a value into the binary search tree."""
        if self.constraint and not self._check_constraint(value):
            raise RuntimeError(f"Value {value.to_display_string()} does not match tree constraint {self.constraint}")

        # Create new node with the value
        from .graph_foundation import GraphNode
        new_node = GraphNode(value)
        self.graph.nodes[new_node.node_id] = new_node

        if self.graph.size == 0:
            # First node becomes root
            self.graph.set_root(new_node.node_id)
            self.graph.size = 1
        else:
            # Insert according to BST rules
            self._insert_recursive(self.graph.get_root(), new_node, value)
            self.graph.size += 1

        return NoneValue()

    def _insert_recursive(self, current: GraphNode, new_node: GraphNode, value: GlangValue) -> None:
        """Recursively insert a node in the correct BST position."""
        current_value = current.value

        # Compare values to determine left/right placement
        if self._compare_values(value, current_value) <= 0:
            # Go left
            left_child = self._get_child(current, "left")
            if left_child is None:
                # Add as left child
                metadata = EdgeMetadata(EdgeType.NAMED, key="left")
                current.add_edge_to(new_node, metadata)
            else:
                self._insert_recursive(left_child, new_node, value)
        else:
            # Go right
            right_child = self._get_child(current, "right")
            if right_child is None:
                # Add as right child
                metadata = EdgeMetadata(EdgeType.NAMED, key="right")
                current.add_edge_to(new_node, metadata)
            else:
                self._insert_recursive(right_child, new_node, value)

    def _get_child(self, node: GraphNode, direction: str) -> Optional[GraphNode]:
        """Get left or right child of a node."""
        edge_info = node.get_edge_by_key(direction)
        if edge_info:
            target_node, metadata = edge_info
            return target_node
        return None

    def _compare_values(self, a: GlangValue, b: GlangValue) -> int:
        """Compare two values for BST ordering. Returns -1, 0, or 1."""
        # Use existing comparison logic from GlangValue._glang_compare if available
        try:
            if hasattr(a, 'value') and hasattr(b, 'value'):
                if a.value < b.value:
                    return -1
                elif a.value > b.value:
                    return 1
                else:
                    return 0
            return 0
        except (TypeError, AttributeError):
            # Fallback: compare string representations
            a_str = str(a.to_python() if hasattr(a, 'to_python') else a)
            b_str = str(b.to_python() if hasattr(b, 'to_python') else b)
            if a_str < b_str:
                return -1
            elif a_str > b_str:
                return 1
            else:
                return 0

    def search(self, value: GlangValue) -> BooleanValue:
        """Search for a value in the tree."""
        found = self._search_recursive(self.graph.get_root(), value)
        return BooleanValue(found, self.position)

    def _search_recursive(self, node: Optional[GraphNode], value: GlangValue) -> bool:
        """Recursively search for a value."""
        if node is None:
            return False

        comparison = self._compare_values(value, node.value)
        if comparison == 0:
            return True
        elif comparison < 0:
            return self._search_recursive(self._get_child(node, "left"), value)
        else:
            return self._search_recursive(self._get_child(node, "right"), value)

    def size(self) -> NumberValue:
        """Get the number of nodes in the tree."""
        return NumberValue(self.graph.size, self.position)

    def empty(self) -> BooleanValue:
        """Check if the tree is empty."""
        return BooleanValue(self.graph.size == 0, self.position)

    def height(self) -> NumberValue:
        """Get the height of the tree."""
        root = self.graph.get_root()
        if root is None:
            return NumberValue(0, self.position)

        def calculate_height(node: Optional[GraphNode]) -> int:
            if node is None:
                return 0

            left_height = calculate_height(self._get_child(node, "left"))
            right_height = calculate_height(self._get_child(node, "right"))

            return 1 + max(left_height, right_height)

        return NumberValue(calculate_height(root), self.position)

    # Traversal methods
    def in_order(self) -> 'ListValue':
        """Return values in in-order traversal (left, root, right)."""
        from .graph_values import ListValue

        result = []

        def traverse_in_order(node: Optional[GraphNode]):
            if node is not None:
                traverse_in_order(self._get_child(node, "left"))
                result.append(node.value)
                traverse_in_order(self._get_child(node, "right"))

        traverse_in_order(self.graph.get_root())
        return ListValue(result, self.constraint, self.position)

    def pre_order(self) -> 'ListValue':
        """Return values in pre-order traversal (root, left, right)."""
        from .graph_values import ListValue

        result = []

        def traverse_pre_order(node: Optional[GraphNode]):
            if node is not None:
                result.append(node.value)
                traverse_pre_order(self._get_child(node, "left"))
                traverse_pre_order(self._get_child(node, "right"))

        traverse_pre_order(self.graph.get_root())
        return ListValue(result, self.constraint, self.position)

    def post_order(self) -> 'ListValue':
        """Return values in post-order traversal (left, right, root)."""
        from .graph_values import ListValue

        result = []

        def traverse_post_order(node: Optional[GraphNode]):
            if node is not None:
                traverse_post_order(self._get_child(node, "left"))
                traverse_post_order(self._get_child(node, "right"))
                result.append(node.value)

        traverse_post_order(self.graph.get_root())
        return ListValue(result, self.constraint, self.position)

    # Edge governance methods (inherited from GraphContainer but tree-specific)
    def get_active_rules(self) -> List[str]:
        """Get list of currently active edge rules."""
        return self.graph.control_layer.get_active_rules()

    def get_rule_status(self, rule_name: str) -> str:
        """Get status of a specific rule: 'active', 'disabled', or 'unknown'."""
        return self.graph.control_layer.get_rule_status(rule_name)

    def disable_rule(self, rule_name: str) -> 'NoneValue':
        """Disable a specific edge rule."""
        self.graph.control_layer.disable_rule(rule_name)
        return NoneValue()

    def enable_rule(self, rule_name: str) -> 'NoneValue':
        """Re-enable a previously disabled edge rule."""
        self.graph.control_layer.enable_rule(rule_name)
        return NoneValue()

    # Visualization methods
    def get_graph_summary(self) -> dict:
        """Get a summary of the tree structure."""
        return self.graph.control_layer.get_graph_summary()

    def visualize_structure(self, format: str = "text") -> str:
        """Visualize the tree structure in different formats."""
        return self.graph.control_layer.visualize_structure(format)

    def _check_constraint(self, value: GlangValue) -> bool:
        """Check if a value matches the tree's type constraint."""
        if self.constraint is None:
            return True

        value_type = value.get_type()
        return value_type == self.constraint


# Extend ControlLayer with tree-specific rules
def _add_tree_rules(self):
    """Add tree-specific rules to the control layer."""
    if not hasattr(self, '_tree_rules_added'):
        # Import EdgeRule to define tree-specific rules
        from .control_layer import EdgeRule

        # Add tree-specific rules
        tree_rules = {
            'max_children_two': EdgeRule(
                name='max_children_two',
                description='Each node can have at most 2 children (left and right)',
                validator=_validate_max_children_two
            ),
            'tree_hierarchy': EdgeRule(
                name='tree_hierarchy',
                description='Enforce proper parent-child relationships in trees',
                validator=_validate_tree_hierarchy
            ),
            'no_tree_cycles': EdgeRule(
                name='no_tree_cycles',
                description='Prevent any cycles in tree structures (stricter than list cycles)',
                validator=_validate_no_tree_cycles
            )
        }

        # Initialize custom rules if not already done
        if self.custom_rules is None:
            self.custom_rules = {}

        # Add tree rules to custom rules
        self.custom_rules.update(tree_rules)

        self._tree_rules_added = True

def _validate_max_children_two(from_node, to_node, metadata, context):
    """Validate that a node doesn't have more than 2 children."""
    if context.get('graph_type') != 'tree':
        return True, ""

    # Count current children of from_node
    child_count = 0
    if from_node.get_edge_by_key('left'):
        child_count += 1
    if from_node.get_edge_by_key('right'):
        child_count += 1

    # Check if adding this edge would exceed 2 children
    if metadata.key in ['left', 'right'] and child_count >= 2:
        return False, f"Node already has 2 children, cannot add more (tree constraint)"

    return True, ""

def _validate_tree_hierarchy(from_node, to_node, metadata, context):
    """Validate proper tree hierarchy (prevent parent-child violations)."""
    if context.get('graph_type') != 'tree':
        return True, ""

    # Only allow 'left' and 'right' relationships in trees
    if metadata.key not in ['left', 'right']:
        return False, f"Tree nodes can only have 'left' or 'right' children, not '{metadata.key}'"

    return True, ""

def _validate_no_tree_cycles(from_node, to_node, metadata, context):
    """Prevent any cycles in tree structures (stricter than list cycles)."""
    if context.get('graph_type') != 'tree':
        return True, ""

    # In trees, no node should be reachable from itself through any path
    # This is stricter than the list cycle check
    visited = set()

    def has_path_to(current, target):
        if current.node_id == target.node_id:
            return True
        if current.node_id in visited:
            return False

        visited.add(current.node_id)

        # Check all outgoing neighbors
        for neighbor in current.get_neighbors():
            if has_path_to(neighbor, target):
                return True

        return False

    # Check if to_node already has a path to from_node
    if has_path_to(to_node, from_node):
        return False, "Adding this edge would create a cycle in tree structure"

    return True, ""

# Monkey-patch the ControlLayer class to add tree rules
ControlLayer._add_tree_rules = _add_tree_rules
ControlLayer._validate_max_children_two = staticmethod(_validate_max_children_two)
ControlLayer._validate_tree_hierarchy = staticmethod(_validate_tree_hierarchy)
ControlLayer._validate_no_tree_cycles = staticmethod(_validate_no_tree_cycles)
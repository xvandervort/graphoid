"""
Control Layer (Layer 3) for Glang Graph Architecture

This module implements the control layer that provides governance and rule enforcement
for graph operations, particularly edge creation and manipulation. The control layer
ensures structural integrity and prevents dangerous operations like cycles in sequential
structures and cross-contamination between different data structures.

Key Features:
- Minimal overhead - shared default rules, copy-on-write customization
- Strict by default - safe operations unless explicitly overridden
- Extensible rule system - easy to add new constraints
- Five-layer integration - validates across all graph layers
"""

from typing import Dict, Set, Optional, Tuple, Any, TYPE_CHECKING
from enum import Enum

if TYPE_CHECKING:
    from .graph_foundation import GraphNode, GraphStructure, EdgeMetadata


class RuleViolationError(Exception):
    """Raised when an edge operation violates a control layer rule."""

    def __init__(self, rule_name: str, message: str):
        self.rule_name = rule_name
        self.message = message
        super().__init__(f"Rule '{rule_name}' violated: {message}")


class EdgeRule:
    """A rule that governs edge operations in the graph."""

    def __init__(self, name: str, description: str, validator: callable):
        self.name = name
        self.description = description
        self.validator = validator  # Function: (from_node, to_node, metadata, context) -> (bool, str)

    def validate(self, from_node: 'GraphNode', to_node: 'GraphNode',
                metadata: 'EdgeMetadata', context: Dict[str, Any]) -> Tuple[bool, str]:
        """Validate an edge operation using this rule."""
        try:
            return self.validator(from_node, to_node, metadata, context)
        except Exception as e:
            return False, f"Rule validation error: {str(e)}"


class ControlLayer:
    """Layer 3: Rule enforcement and governance for graph operations."""

    def __init__(self, parent_graph: 'GraphStructure'):
        self.parent_graph = parent_graph

        # Rule management
        self.disabled_rules: Set[str] = set()
        self.custom_rules: Optional[Dict[str, EdgeRule]] = None  # Lazy creation

        # Rule definitions (shared by all instances)
        self._standard_rules = self._get_standard_rules()

    def validate_edge_operation(self, from_node: 'GraphNode', to_node: 'GraphNode',
                               metadata: 'EdgeMetadata') -> Tuple[bool, str]:
        """Validate an edge operation using active rules."""

        # Build validation context
        context = {
            'graph_type': self._infer_graph_type(),
            'nodes': self.parent_graph.nodes,
            'structure_info': self._analyze_structure(),
            'parent_graph': self.parent_graph
        }

        # Check all active rules
        active_rules = self._get_active_rules()
        for rule_name, rule in active_rules.items():
            if rule_name in self.disabled_rules:
                continue

            is_valid, reason = rule.validate(from_node, to_node, metadata, context)
            if not is_valid:
                return False, f"Rule '{rule_name}' violated: {reason}"

        return True, ""

    def disable_rule(self, rule_name: str) -> None:
        """Disable a specific rule for this graph."""
        self.disabled_rules.add(rule_name)

    def enable_rule(self, rule_name: str) -> None:
        """Re-enable a previously disabled rule."""
        self.disabled_rules.discard(rule_name)

    def get_active_rules(self) -> list[str]:
        """Get list of currently active rule names."""
        active_rules = self._get_active_rules()
        return [name for name in active_rules.keys() if name not in self.disabled_rules]

    def get_rule_status(self, rule_name: str) -> str:
        """Get status of a specific rule: 'active', 'disabled', or 'unknown'."""
        if rule_name not in self._get_active_rules():
            return 'unknown'
        return 'disabled' if rule_name in self.disabled_rules else 'active'

    def _get_active_rules(self) -> Dict[str, EdgeRule]:
        """Get active rules: defaults + custom rules."""
        if self.custom_rules is None:
            # Pure default mode - no copying overhead
            return self._standard_rules
        else:
            # Custom mode - merge defaults with customizations
            combined = self._standard_rules.copy()
            combined.update(self.custom_rules)
            return combined

    def _infer_graph_type(self) -> str:
        """Infer the type of graph structure."""
        # Check if this is a SequentialGraph (list-like)
        if hasattr(self.parent_graph, 'sequence_order'):
            return 'list'

        # Check if this is a KeyedGraph (hash-like)
        if hasattr(self.parent_graph, 'key_to_node'):
            return 'hash'

        # Default to generic graph
        return 'graph'

    def _analyze_structure(self) -> Dict[str, Any]:
        """Analyze current graph structure for rule validation."""
        info = {
            'node_count': len(self.parent_graph.nodes),
            'is_empty': len(self.parent_graph.nodes) == 0
        }

        # Add type-specific information
        graph_type = self._infer_graph_type()
        if graph_type == 'list':
            info['sequence_length'] = len(getattr(self.parent_graph, 'sequence_order', []))
        elif graph_type == 'hash':
            info['key_count'] = len(getattr(self.parent_graph, 'key_to_node', {}))

        return info

    @classmethod
    def _get_standard_rules(cls) -> Dict[str, EdgeRule]:
        """Get the standard set of edge rules."""
        return {
            'no_list_cycles': EdgeRule(
                name='no_list_cycles',
                description='Prevent circular references in sequential structures',
                validator=cls._validate_no_list_cycles
            ),
            'same_structure_only': EdgeRule(
                name='same_structure_only',
                description='Prevent cross-contamination between different data structures',
                validator=cls._validate_same_structure_only
            )
        }

    @staticmethod
    def _validate_no_list_cycles(from_node: 'GraphNode', to_node: 'GraphNode',
                                metadata: 'EdgeMetadata', context: Dict[str, Any]) -> Tuple[bool, str]:
        """Validate that adding this edge won't create a cycle in a sequential structure."""

        # Only apply to sequential structures (lists)
        if context.get('graph_type') != 'list':
            return True, ""

        # Check if this would create a cycle by doing a simple path check
        # For now, implement a basic check: prevent any edge that goes "backwards" in sequence
        parent_graph = context['parent_graph']
        if hasattr(parent_graph, 'sequence_order'):
            sequence = parent_graph.sequence_order

            try:
                from_idx = sequence.index(from_node)
                to_idx = sequence.index(to_node)

                # Prevent edges that go backwards in the sequence (simple cycle prevention)
                if to_idx <= from_idx:
                    return False, f"Edge from index {from_idx} to {to_idx} would create a cycle in sequential structure"
            except ValueError:
                # Node not in sequence - allow edge
                pass

        return True, ""

    @staticmethod
    def _validate_same_structure_only(from_node: 'GraphNode', to_node: 'GraphNode',
                                    metadata: 'EdgeMetadata', context: Dict[str, Any]) -> Tuple[bool, str]:
        """Validate that both nodes belong to the same graph structure."""

        # Check if both nodes belong to the same parent graph
        if from_node._graph is not to_node._graph:
            return False, "Cannot create edges between nodes from different graph structures"

        # Additional check: ensure both nodes are actually in this graph
        graph_nodes = context['nodes']
        if from_node.node_id not in graph_nodes or to_node.node_id not in graph_nodes:
            return False, "Cannot create edges to nodes not in this graph structure"

        return True, ""
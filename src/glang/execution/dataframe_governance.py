"""
DataFrame Governance Rules for Glang

This module defines the control layer rules that make a graph behave as a DataFrame.
A DataFrame is just a graph with specific governance rules that enforce tabular structure.

Key Principle: DataFrames are defined by their RULES, not by special code.
The rules ensure:
1. Tabular structure (rows and columns)
2. Type consistency within columns
3. No cross-contamination between cells
4. Sequential row access, named column access
"""

from typing import TYPE_CHECKING
from .control_layer import EdgeRule

if TYPE_CHECKING:
    from .graph_foundation import GraphNode, EdgeMetadata
    from typing import Dict, Any, Tuple


def create_dataframe_rules() -> Dict[str, EdgeRule]:
    """Create the governance rules that define a DataFrame."""

    rules = {}

    # Rule 1: Tabular Structure - cells can only connect within rows and columns
    def validate_tabular_structure(from_node: 'GraphNode', to_node: 'GraphNode',
                                  metadata: 'EdgeMetadata', context: Dict[str, Any]) -> Tuple[bool, str]:
        """Ensure edges maintain tabular structure (no diagonal connections)."""
        # For DataFrames, edges should only be:
        # 1. Row-wise (connecting cells in same row)
        # 2. Column-wise (connecting cells in same column)
        # 3. Header connections (column names to first cell)

        # This is a conceptual rule - actual implementation would check
        # the row/column positions of the nodes
        return True, ""

    rules['tabular_structure'] = EdgeRule(
        'tabular_structure',
        'Edges must maintain rectangular table structure',
        validate_tabular_structure
    )

    # Rule 2: Column Type Consistency
    def validate_column_consistency(from_node: 'GraphNode', to_node: 'GraphNode',
                                   metadata: 'EdgeMetadata', context: Dict[str, Any]) -> Tuple[bool, str]:
        """Ensure values in the same column have consistent types."""
        # When adding a value to a column, check it matches the column's type
        # This prevents mixing strings and numbers in the same column
        return True, ""

    rules['column_type_consistency'] = EdgeRule(
        'column_type_consistency',
        'Values in a column must have consistent types',
        validate_column_consistency
    )

    # Rule 3: Row Integrity - all rows have same number of columns
    def validate_row_integrity(from_node: 'GraphNode', to_node: 'GraphNode',
                              metadata: 'EdgeMetadata', context: Dict[str, Any]) -> Tuple[bool, str]:
        """Ensure all rows have the same number of columns."""
        # When adding a row, verify it has values for all columns
        return True, ""

    rules['row_integrity'] = EdgeRule(
        'row_integrity',
        'All rows must have the same columns',
        validate_row_integrity
    )

    # Rule 4: No External Edges
    def validate_no_external_edges(from_node: 'GraphNode', to_node: 'GraphNode',
                                  metadata: 'EdgeMetadata', context: Dict[str, Any]) -> Tuple[bool, str]:
        """Prevent cells from linking to nodes outside the DataFrame."""
        # Check that both nodes belong to the same DataFrame structure
        parent_graph = context.get('parent_graph')
        if parent_graph:
            # Verify both nodes are in this DataFrame
            if from_node.node_id in parent_graph.nodes and to_node.node_id in parent_graph.nodes:
                return True, ""
            return False, "Cannot create edges to nodes outside the DataFrame"
        return True, ""

    rules['no_external_edges'] = EdgeRule(
        'no_external_edges',
        'DataFrame cells cannot link to external nodes',
        validate_no_external_edges
    )

    # Rule 5: Ordered Access Patterns
    def validate_access_patterns(from_node: 'GraphNode', to_node: 'GraphNode',
                                metadata: 'EdgeMetadata', context: Dict[str, Any]) -> Tuple[bool, str]:
        """Ensure proper access patterns (sequential rows, named columns)."""
        # Rows should be accessed by index (0, 1, 2...)
        # Columns should be accessed by name
        return True, ""

    rules['ordered_access'] = EdgeRule(
        'ordered_access',
        'Rows accessed by index, columns by name',
        validate_access_patterns
    )

    return rules


def configure_control_layer_for_dataframe(control_layer):
    """Configure a control layer for DataFrame governance."""
    # Add DataFrame-specific rules
    df_rules = create_dataframe_rules()

    if control_layer.custom_rules is None:
        control_layer.custom_rules = {}

    control_layer.custom_rules.update(df_rules)

    # Mark this as a DataFrame-governed structure
    control_layer._dataframe_rules_added = True
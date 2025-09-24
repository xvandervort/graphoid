"""
DataFrame Implementation for Glang

DataFrames are graphs with tabular governance rules. This module provides
the minimal Python wrapper needed to create DataFrame graph structures.

The actual DataFrame logic is implemented in Glang (stdlib/dataframe.gr).
This Python code just provides the graph structure with appropriate governance.
"""

from typing import Optional, List, Dict, Any
from .values import GlangValue
from .graph_foundation import GraphStructure, GraphNode
from .control_layer import ControlLayer
from .dataframe_governance import configure_control_layer_for_dataframe
from ..ast.nodes import SourcePosition
from ..graph_container import GraphContainer


class DataFrameGraph(GraphStructure):
    """Graph structure specialized for DataFrames with tabular governance."""

    def __init__(self, columns: List[str]):
        super().__init__()
        self.columns = columns
        self.row_count = 0

        # Configure control layer for DataFrame governance
        configure_control_layer_for_dataframe(self.control_layer)

        # Create column header nodes
        self.column_nodes = {}
        for col in columns:
            node = GraphNode(None, f"col_{col}")
            self.add_node(node)
            self.column_nodes[col] = node

    def add_row(self, row_data: Dict[str, Any]) -> None:
        """Add a row to the DataFrame (creates nodes for cells)."""
        row_idx = self.row_count

        for col in self.columns:
            # Get value for this column (or None if missing)
            value = row_data.get(col)

            # Create node for this cell
            cell_node = GraphNode(value, f"cell_{row_idx}_{col}")
            self.add_node(cell_node)

            # Connect to column header
            col_node = self.column_nodes[col]
            # Edge from column to cell represents column membership
            # This is where governance rules would validate the operation

        self.row_count += 1


class DataFrameValue(GlangValue, GraphContainer):
    """DataFrame value type - a graph with tabular governance rules."""

    def __init__(self, columns: List[str], position: Optional[SourcePosition] = None):
        GlangValue.__init__(self, position)
        GraphContainer.__init__(self)

        self.graph = DataFrameGraph(columns)

    def get_type(self) -> str:
        return "dataframe"

    def to_display_string(self) -> str:
        return f"DataFrame({self.graph.row_count} rows, {len(self.graph.columns)} columns)"

    def to_python(self) -> dict:
        """Convert to Python dict for interop."""
        result = {
            "_type": "dataframe",
            "_columns": self.graph.columns,
            "_row_count": self.graph.row_count
        }

        # Add column data
        for col in self.graph.columns:
            result[col] = []  # Would populate from graph nodes

        return result
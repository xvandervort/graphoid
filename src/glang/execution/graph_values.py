"""
Graph-based Value Types for Glang

This module provides ListValue and HashValue as graph-based implementations that replace the old
container-based ListValue and HashValue with true graph structures.

These maintain full backward compatibility while providing graph capabilities.
"""

from typing import List, Optional, Any, Tuple, Union, Iterator, Dict
from .values import GlangValue, NoneValue
from .graph_foundation import SequentialGraph, KeyedGraph, GraphNode, EdgeType, EdgeMetadata
from ..graph_container import GraphContainer
from ..ast.nodes import SourcePosition


class ListValue(GlangValue, GraphContainer):
    """Graph-based list value that maintains sequential ordering through graph edges."""

    def __init__(self, elements: List[GlangValue], constraint: Optional[str] = None,
                 position: Optional[SourcePosition] = None):
        GlangValue.__init__(self, position)
        GraphContainer.__init__(self)

        self.constraint = constraint
        self.graph = SequentialGraph(elements)

        # Check if any elements are frozen
        self._update_frozen_flag()

    def _update_frozen_flag(self):
        """Update the contains_frozen flag based on elements."""
        self.contains_frozen = any(elem.is_frozen_value() or elem.contains_frozen_data()
                                 for elem in self.elements)

    def _apply_behaviors_to_existing(self):
        """Apply behaviors to all existing elements in the list."""
        if self._has_behaviors():
            values = self.elements
            processed_values = [self._apply_behaviors(value) for value in values]
            self.graph = SequentialGraph(processed_values)

    @property
    def elements(self) -> List[GlangValue]:
        """Backward compatibility: provide elements list interface."""
        return self.graph.get_values()

    def to_python(self) -> List[Any]:
        return [elem.to_python() for elem in self.elements]

    def get_type(self) -> str:
        return "list"

    def to_display_string(self) -> str:
        element_strs = [elem.to_display_string() for elem in self.elements]
        return f"[{', '.join(element_strs)}]"

    def validate_constraint(self, value: GlangValue) -> bool:
        """Check if value matches list constraint."""
        if not self.constraint:
            return True
        return value.get_type() == self.constraint

    def can_accept_element(self, element: GlangValue) -> Tuple[bool, str]:
        """Check if this list can accept the given element."""
        # Check constraint first
        if not self.validate_constraint(element):
            return False, f"Element type {element.get_type()} does not match constraint {self.constraint}"

        # Check contamination rules
        if self.is_frozen:
            return False, "Cannot modify frozen list"

        if element.is_frozen_value() and not self.contains_frozen:
            return False, "Cannot mix frozen and unfrozen data in same collection"

        if not element.is_frozen_value() and self.contains_frozen:
            return False, "Cannot mix frozen and unfrozen data in same collection"

        return True, ""

    def append(self, value: GlangValue) -> None:
        """Append value to list (with behavior application and constraint validation)."""
        self._check_not_frozen("append")
        self._check_contamination_compatibility(value, "append")

        # Apply behaviors if any
        if self._has_behaviors():
            value = self._apply_behaviors(value)

        if not self.validate_constraint(value):
            from .errors import TypeConstraintError
            raise TypeConstraintError(
                f"Cannot append {value.get_type()} to list<{self.constraint}>",
                value.position
            )

        self.graph.append(value)

    def prepend(self, value: GlangValue) -> None:
        """Prepend value to the beginning of list."""
        self._check_not_frozen("prepend")
        self._check_contamination_compatibility(value, "prepend")

        # Apply behaviors if any
        if self._has_behaviors():
            value = self._apply_behaviors(value)

        if not self.validate_constraint(value):
            from .errors import TypeConstraintError
            raise TypeConstraintError(
                f"Cannot prepend {value.get_type()} to list<{self.constraint}>",
                value.position
            )

        # Insert at beginning by creating new sequential graph
        values = [value] + self.elements
        self.graph = SequentialGraph(values)

    def insert(self, index: int, value: GlangValue) -> None:
        """Insert value at specific index."""
        self._check_not_frozen("insert")
        self._check_contamination_compatibility(value, "insert")

        # Apply behaviors if any
        if self._has_behaviors():
            value = self._apply_behaviors(value)

        if not self.validate_constraint(value):
            from .errors import TypeConstraintError
            raise TypeConstraintError(
                f"Cannot insert {value.get_type()} to list<{self.constraint}>",
                value.position
            )

        # Rebuild graph with inserted value
        values = self.elements
        if 0 <= index <= len(values):
            values.insert(index, value)
            self.graph = SequentialGraph(values)
        else:
            raise IndexError(f"Index {index} out of range for list of length {len(values)}")

    def get_at_index(self, index: int) -> Optional[GlangValue]:
        """Get value at specific index."""
        return self.graph.get_at_index(index)

    def set_at_index(self, index: int, value: GlangValue) -> bool:
        """Set value at specific index."""
        self._check_not_frozen("index assignment")

        # Apply behaviors if any
        if self._has_behaviors():
            value = self._apply_behaviors(value)

        if not self.validate_constraint(value):
            from .errors import TypeConstraintError
            raise TypeConstraintError(
                f"Cannot assign {value.get_type()} to list<{self.constraint}>",
                value.position
            )

        return self.graph.set_at_index(index, value)

    def remove_at_index(self, index: int) -> Optional[GlangValue]:
        """Remove and return value at specific index."""
        self._check_not_frozen("remove")

        values = self.elements
        if 0 <= index < len(values):
            removed_value = values.pop(index)
            self.graph = SequentialGraph(values)
            return removed_value
        return None

    def size(self) -> int:
        """Get the size of the list."""
        return len(self.graph)

    def universal_size(self) -> 'NumberValue':
        """Universal size method for method dispatch."""
        from .values import NumberValue
        return NumberValue(self.size(), self.position)

    def is_empty(self) -> bool:
        """Check if list is empty."""
        return len(self.graph) == 0

    def contains(self, value: GlangValue) -> bool:
        """Check if list contains a specific value."""
        for element in self.elements:
            if element == value:
                return True
        return False

    def index_of(self, value: GlangValue) -> int:
        """Get index of first occurrence of value, or -1 if not found."""
        for i, element in enumerate(self.elements):
            if element == value:
                return i
        return -1

    def reverse(self) -> 'ListValue':
        """Return a new list with elements in reverse order."""
        reversed_elements = list(reversed(self.elements))
        return ListValue(reversed_elements, self.constraint, self.position)

    def slice(self, start: int, end: Optional[int] = None, step: int = 1) -> 'ListValue':
        """Create a slice of the list."""
        elements = self.elements[start:end:step]
        return ListValue(elements, self.constraint, self.position)

    # Graph-specific methods

    def get_node_at_index(self, index: int) -> Optional[GraphNode]:
        """Get the graph node at a specific index."""
        if 0 <= index < len(self.graph.sequence_order):
            return self.graph.sequence_order[index]
        return None

    def get_graph_structure(self) -> SequentialGraph:
        """Get the underlying graph structure."""
        return self.graph

    def get_neighbors(self, index: int) -> List[GlangValue]:
        """Get neighboring values of the element at index."""
        neighbors = []
        if 0 <= index < len(self.graph.sequence_order):
            node = self.graph.sequence_order[index]
            neighbor_nodes = node.get_neighbors() + node.get_incoming_neighbors()
            neighbors = [n.value for n in neighbor_nodes]
        return neighbors

    def add_edge(self, from_index: int, to_index: int, relationship: str = "related") -> bool:
        """Add a custom edge between two elements (beyond sequential ordering)."""
        if (0 <= from_index < len(self.graph.sequence_order) and
            0 <= to_index < len(self.graph.sequence_order)):

            from_node = self.graph.sequence_order[from_index]
            to_node = self.graph.sequence_order[to_index]

            metadata = EdgeMetadata(
                edge_type=EdgeType.NAMED,
                key=relationship
            )
            from_node.add_edge_to(to_node, metadata)
            return True
        return False

    def get_connected_to(self, index: int, relationship: str = "related") -> List[int]:
        """Get indices of elements connected via a specific relationship."""
        if 0 <= index < len(self.graph.sequence_order):
            node = self.graph.sequence_order[index]
            connected_indices = []

            for i, other_node in enumerate(self.graph.sequence_order):
                if node.has_edge_to(other_node):
                    edge_metadata = node.get_edge_to(other_node)
                    if edge_metadata and edge_metadata.key == relationship:
                        connected_indices.append(i)

            return connected_indices
        return []

    def get_edges(self) -> List[Tuple[int, int, str]]:
        """Get all custom edges (non-sequential) in the list."""
        edges = []
        for i, node in enumerate(self.graph.sequence_order):
            for target, metadata in node._outgoing.values():
                # Skip sequential edges (those are structural, not custom)
                if metadata.edge_type.value != "sequential":
                    try:
                        target_index = self.graph.sequence_order.index(target)
                        edges.append((i, target_index, str(metadata.key)))
                    except ValueError:
                        # Target not in sequence - shouldn't happen but be safe
                        pass
        return edges

    def get_edge_count(self) -> int:
        """Get total number of custom edges."""
        return len(self.get_edges())

    def can_add_edge(self, from_index: int, to_index: int, relationship: str = "related") -> Tuple[bool, str]:
        """Check if an edge can be added without actually adding it."""
        if not (0 <= from_index < len(self.graph.sequence_order) and
                0 <= to_index < len(self.graph.sequence_order)):
            return False, "Invalid index range"

        from_node = self.graph.sequence_order[from_index]
        to_node = self.graph.sequence_order[to_index]

        from .graph_foundation import EdgeMetadata, EdgeType
        metadata = EdgeMetadata(
            edge_type=EdgeType.NAMED,
            key=relationship
        )

        # Use control layer validation
        return self.graph.control_layer.validate_edge_operation(from_node, to_node, metadata)

    # Control layer access methods (Layer 3)
    def get_active_rules(self) -> List[str]:
        """Get list of currently active edge rules."""
        return self.graph.control_layer.get_active_rules()

    def get_rule_status(self, rule_name: str) -> str:
        """Get status of a specific rule: 'active', 'disabled', or 'unknown'."""
        return self.graph.control_layer.get_rule_status(rule_name)

    def disable_rule(self, rule_name: str) -> 'NoneValue':
        """Disable a specific edge rule."""
        from .values import NoneValue
        self.graph.control_layer.disable_rule(rule_name)
        return NoneValue()

    def enable_rule(self, rule_name: str) -> 'NoneValue':
        """Re-enable a previously disabled edge rule."""
        from .values import NoneValue
        self.graph.control_layer.enable_rule(rule_name)
        return NoneValue()

    # Visualization methods
    def get_graph_summary(self) -> Dict[str, Any]:
        """Get a summary of the graph structure."""
        return self.graph.control_layer.get_graph_summary()

    def visualize_structure(self, format: str = "text") -> str:
        """Visualize the graph structure in different formats."""
        return self.graph.control_layer.visualize_structure(format)

    def to_graph(self, connection_pattern: str = "chain") -> 'ListValue':
        """Convert to a graph with a specific connection pattern."""
        if connection_pattern == "chain":
            # Already a chain, return copy
            return ListValue(self.elements, self.constraint, self.position)

        elif connection_pattern == "star":
            # First element connects to all others
            result = ListValue(self.elements, self.constraint, self.position)
            if len(result.elements) > 1:
                for i in range(1, len(result.elements)):
                    result.add_edge(0, i, "star")
            return result

        elif connection_pattern == "complete":
            # Every element connects to every other
            result = ListValue(self.elements, self.constraint, self.position)
            for i in range(len(result.elements)):
                for j in range(len(result.elements)):
                    if i != j:
                        result.add_edge(i, j, "complete")
            return result

        else:
            raise ValueError(f"Unknown connection pattern: {connection_pattern}")

    def __eq__(self, other) -> bool:
        if not isinstance(other, (ListValue, list)):
            return False

        if isinstance(other, list):
            # Compare with Python list
            return self.elements == other

        # Compare with another ListValue
        return (self.elements == other.elements and
                self.constraint == other.constraint)

    def __len__(self) -> int:
        return len(self.graph)

    def __iter__(self) -> Iterator[GlangValue]:
        return iter(self.elements)


    def universal_inspect(self) -> 'StringValue':
        """List-specific inspection showing constraint and element count."""
        from .values import StringValue
        constraint_info = f"<{self.constraint}>" if self.constraint else ""
        info = f"list{constraint_info} with {len(self.elements)} elements"
        return StringValue(info, self.position)

    def _deep_freeze(self):
        """Freeze all elements in the list (deep freeze)."""
        for element in self.elements:
            element.freeze()

    @staticmethod
    def _glang_equals(left: 'GlangValue', right: 'GlangValue') -> bool:
        """General-purpose equality comparison for any two Glang values."""
        # Handle None/null values
        if left is None and right is None:
            return True
        if left is None or right is None:
            return False

        # Use the __eq__ method of the left operand
        try:
            return left.__eq__(right)
        except (AttributeError, TypeError):
            # Fallback: compare by value if available
            try:
                return left.value == right.value
            except AttributeError:
                # Last resort: object identity
                return left is right

    @staticmethod
    def _glang_compare(left: 'GlangValue', right: 'GlangValue') -> int:
        """General-purpose comparison for any two Glang values.

        Returns:
            -1 if left < right
             0 if left == right
             1 if left > right
        """
        # Handle None/null values
        if left is None and right is None:
            return 0
        if left is None:
            return -1
        if right is None:
            return 1

        # Get underlying Python values for comparison
        try:
            left_val = left.to_python()
            right_val = right.to_python()

            # Compare values
            if left_val < right_val:
                return -1
            elif left_val > right_val:
                return 1
            else:
                return 0
        except (AttributeError, TypeError):
            # Fallback: try to compare by value attribute
            try:
                left_val = left.value
                right_val = right.value

                if left_val < right_val:
                    return -1
                elif left_val > right_val:
                    return 1
                else:
                    return 0
            except (AttributeError, TypeError):
                # Last resort: compare string representations
                left_str = str(left)
                right_str = str(right)

                if left_str < right_str:
                    return -1
                elif left_str > right_str:
                    return 1
                else:
                    return 0

    # Metadata layer methods - expose graph metadata functionality
    @property
    def metadata(self):
        """Access the graph's metadata layer."""
        return self.graph.metadata

    # Element naming methods (R vector style)
    def set_names(self, names: List[Optional[str]]) -> 'ListValue':
        """Set names for all elements (None for unnamed elements)."""
        self.graph.set_names(names)
        return self

    def get_names(self) -> List[Optional[str]]:
        """Get names for all elements (None for unnamed elements)."""
        return self.graph.get_names()

    def get_name(self, index: int) -> Optional[str]:
        """Get the name of an element at given index."""
        return self.graph.get_name(index)

    def set_name(self, index: int, name: Optional[str]) -> 'NoneValue':
        """Set the name for a single element."""
        from .values import NoneValue
        self.graph.set_name(index, name)
        return NoneValue()

    def has_names(self) -> bool:
        """Check if any elements have names."""
        return self.graph.has_names()

    def get_by_name(self, name: str) -> Optional[GlangValue]:
        """Get an element by its name."""
        return self.graph.get_value_by_name(name)

    # Enhanced indexing to support name-based access
    def __getitem__(self, key):
        """Enhanced indexing that supports both numeric and name-based access."""
        if isinstance(key, int):
            # Standard numeric indexing with proper bounds checking
            value = self.get_at_index(key)
            if value is None:
                raise IndexError(f"Index {key} out of range")
            return value
        elif isinstance(key, str):
            # Name-based access (new)
            value = self.get_by_name(key)
            if value is not None:
                return value
            else:
                # Graceful degradation - return None instead of error
                return None
        else:
            raise TypeError(f"List indices must be integers or strings, not {type(key)}")

    def __setitem__(self, key, value):
        """Enhanced assignment that supports both numeric and name-based access."""
        if isinstance(key, int):
            # Standard numeric assignment with proper bounds checking
            if not self.set_at_index(key, value):
                raise IndexError(f"Index {key} out of range")
        elif isinstance(key, str):
            # Name-based assignment (new)
            index = self.graph.get_index_by_name(key)
            if index is not None:
                self.set_at_index(index, value)
            else:
                # Graceful degradation - could add as new element or ignore
                # For now, do nothing (like R when accessing non-existent named element)
                pass
        else:
            raise TypeError(f"List indices must be integers or strings, not {type(key)}")


class HashValue(GlangValue, GraphContainer):
    """Graph-based hash value where keys are edges connecting root to value nodes."""

    def __init__(self, pairs: List[Tuple[str, GlangValue]], constraint: Optional[str] = None,
                 position: Optional[SourcePosition] = None):
        GlangValue.__init__(self, position)
        GraphContainer.__init__(self)

        self.constraint = constraint
        self.graph = KeyedGraph(pairs)

        # Check if any values are frozen
        self._update_frozen_flag()

    def _update_frozen_flag(self):
        """Update the contains_frozen flag based on values."""
        self.contains_frozen = any(value.is_frozen_value() or value.contains_frozen_data()
                                 for value in self.graph.values())

    def _apply_behaviors_to_existing(self):
        """Apply behaviors to all existing values in the hash."""
        if self._has_behaviors():
            for key in list(self.graph.keys()):
                current_value = self.graph.get(key)
                if current_value:
                    processed_value = self._apply_behaviors(current_value)
                    self.graph.set(key, processed_value)

    def to_python(self) -> dict:
        return {key: value.to_python() for key, value in self.graph.items()}

    def get_type(self) -> str:
        return "hash"

    def to_display_string(self) -> str:
        pairs = []
        for key, value in self.graph.items():
            pairs.append(f'"{key}": {value.to_display_string()}')

        if not pairs:
            return "{}"
        else:
            return f"{{ {', '.join(pairs)} }}"

    def validate_constraint(self, value: GlangValue) -> bool:
        """Check if value matches hash constraint."""
        if not self.constraint:
            return True
        return value.get_type() == self.constraint

    def can_accept_element(self, element: GlangValue) -> Tuple[bool, str]:
        """Check if this hash can accept the given element (value)."""
        # Check constraint first
        if not self.validate_constraint(element):
            return False, f"Element type {element.get_type()} does not match constraint {self.constraint}"

        # Check contamination rules
        if self.is_frozen:
            return False, "Cannot modify frozen hash"

        if element.is_frozen_value() and not self.contains_frozen:
            return False, "Cannot mix frozen and unfrozen data in same collection"

        if not element.is_frozen_value() and self.contains_frozen:
            return False, "Cannot mix frozen and unfrozen data in same collection"

        return True, ""

    def set(self, key: str, value: GlangValue) -> None:
        """Set a key-value pair."""
        self._check_not_frozen("set")
        self._check_contamination_compatibility(value, "set")

        # Apply behaviors if any
        if self._has_behaviors():
            value = self._apply_behaviors(value)

        if not self.validate_constraint(value):
            from .errors import TypeConstraintError
            raise TypeConstraintError(
                f"Cannot set {value.get_type()} in hash<{self.constraint}>",
                value.position
            )

        self.graph.set(key, value)

    def get(self, key: str) -> Optional[GlangValue]:
        """Get value by key."""
        return self.graph.get(key)

    def has_key(self, key: str) -> bool:
        """Check if key exists."""
        return self.graph.has_key(key)

    def remove(self, key: str) -> bool:
        """Remove a key-value pair."""
        self._check_not_frozen("remove")
        return self.graph.remove(key)

    def keys(self) -> List[str]:
        """Get all keys."""
        return self.graph.keys()

    def values(self) -> List[GlangValue]:
        """Get all values."""
        return self.graph.values()

    def items(self) -> List[Tuple[str, GlangValue]]:
        """Get all key-value pairs."""
        return self.graph.items()

    def size(self) -> int:
        """Get number of key-value pairs."""
        return len(self.graph)

    def universal_size(self) -> 'NumberValue':
        """Universal size method for method dispatch."""
        from .values import NumberValue
        return NumberValue(self.size(), self.position)

    def universal_inspect(self) -> 'StringValue':
        """Hash-specific inspection showing constraint and size."""
        from .values import StringValue
        constraint_info = f"<{self.constraint}>" if self.constraint else ""
        info = f'hash{constraint_info} ({len(self.graph)} pairs)'
        return StringValue(info, self.position)

    @property
    def pairs(self):
        """Backward compatibility property for tests expecting .pairs attribute."""
        # Create a simple object that behaves like the old GlangHashTable for tests
        class PairsCompat:
            def __init__(self, graph_hash):
                self._graph_hash = graph_hash

            def __len__(self):
                return len(self._graph_hash.graph)

            def __iter__(self):
                return iter(self._graph_hash.items())

            def __contains__(self, key: str) -> bool:
                """Support 'key in pairs' syntax for backward compatibility."""
                return self._graph_hash.has_key(key)

            def items(self):
                """Return items for backward compatibility."""
                return self._graph_hash.items()

        return PairsCompat(self)

    def is_empty(self) -> bool:
        """Check if hash is empty."""
        return len(self.graph) == 0

    def merge(self, other: 'HashValue') -> 'HashValue':
        """Create a new hash with combined key-value pairs."""
        combined_items = list(self.items())
        for key, value in other.items():
            # Later values override earlier ones
            combined_items = [(k, v) for k, v in combined_items if k != key]
            combined_items.append((key, value))

        return HashValue(combined_items, self.constraint, self.position)

    # Graph-specific methods

    def get_graph_structure(self) -> KeyedGraph:
        """Get the underlying graph structure."""
        return self.graph

    def get_value_node(self, key: str) -> Optional[GraphNode]:
        """Get the graph node for a specific key."""
        return self.graph.key_to_node.get(key)

    def get_connected_keys(self, key: str, relationship: str = "related") -> List[str]:
        """Get keys whose values are connected to the given key's value."""
        value_node = self.get_value_node(key)
        if not value_node:
            return []

        connected_keys = []
        for check_key, check_node in self.graph.key_to_node.items():
            if key != check_key and value_node.has_edge_to(check_node):
                edge_metadata = value_node.get_edge_to(check_node)
                if edge_metadata and edge_metadata.key == relationship:
                    connected_keys.append(check_key)

        return connected_keys

    def add_value_edge(self, from_key: str, to_key: str, relationship: str = "related") -> bool:
        """Add an edge between two values (beyond the key-based structure)."""
        from_node = self.get_value_node(from_key)
        to_node = self.get_value_node(to_key)

        if from_node and to_node:
            metadata = EdgeMetadata(
                edge_type=EdgeType.NAMED,
                key=relationship
            )
            from_node.add_edge_to(to_node, metadata)
            return True
        return False

    def __eq__(self, other) -> bool:
        if not isinstance(other, (HashValue, dict)):
            return False

        if isinstance(other, dict):
            # Compare with Python dict
            return self.to_python() == other

        # Compare with another HashValue
        return (self.to_python() == other.to_python() and
                self.constraint == other.constraint)

    def __len__(self) -> int:
        return len(self.graph)

    def __contains__(self, key: str) -> bool:
        return self.has_key(key)

    def __getitem__(self, key: str) -> GlangValue:
        # First try as a regular key
        value = self.get(key)
        if value is not None:
            return value

        # If not found, try as a name
        value = self.graph.get_value_by_name(key)
        if value is not None:
            return value

        raise KeyError(f"Key '{key}' not found")

    def __setitem__(self, key: str, value: GlangValue) -> None:
        # First check if it's an existing key
        if self.has_key(key):
            self.set(key, value)
        # Otherwise, check if it's a name
        elif self.graph.set_value_by_name(key, value):
            # Successfully set by name
            pass
        else:
            # Neither a key nor a name - create new key
            self.set(key, value)

    # Element naming methods (R vector style) - same as lists for consistency
    def set_names(self, names: List[Optional[str]]) -> 'HashValue':
        """Set names for all hash elements (None for unnamed elements)."""
        self.graph.set_names(names)
        return self

    def get_names(self) -> List[Optional[str]]:
        """Get names for all hash elements (None for unnamed elements)."""
        return self.graph.get_names()

    def get_name(self, index: int) -> Optional[str]:
        """Get the name of a hash element at given index."""
        return self.graph.get_name(index)

    def set_name(self, index: int, name: Optional[str]) -> 'NoneValue':
        """Set the name for a single hash element."""
        from .values import NoneValue
        self.graph.set_name(index, name)
        return NoneValue()

    def has_names(self) -> bool:
        """Check if any hash elements have names."""
        return self.graph.has_names()

    # Edge inspection methods (for consistency with ListValue)
    def get_edges(self) -> List[Tuple[str, str, str]]:
        """Get all edges between hash values as (from_key, to_key, relationship) tuples."""
        edges = []
        for from_key, from_node in self.graph.key_to_node.items():
            for edge in from_node.edges:
                # Find the target key
                target_node = edge.target
                for to_key, to_node in self.graph.key_to_node.items():
                    if to_node is target_node:
                        edges.append((from_key, to_key, edge.metadata.key))
                        break
        return edges

    def get_edge_count(self) -> int:
        """Get total number of edges between hash values."""
        return len(self.get_edges())

    def can_add_edge(self, from_key: str, to_key: str, relationship: str = "related") -> Tuple[bool, str]:
        """Check if an edge can be added between two hash values."""
        from_node = self.get_value_node(from_key)
        to_node = self.get_value_node(to_key)

        if not from_node:
            return False, f"Key '{from_key}' not found in hash"
        if not to_node:
            return False, f"Key '{to_key}' not found in hash"

        from .graph_foundation import EdgeMetadata, EdgeType
        metadata = EdgeMetadata(
            edge_type=EdgeType.NAMED,
            key=relationship
        )

        # Use control layer validation
        return self.graph.control_layer.validate_edge_operation(from_node, to_node, metadata)

    # Control layer access methods (Layer 3)
    def get_active_rules(self) -> List[str]:
        """Get list of currently active edge rules."""
        return self.graph.control_layer.get_active_rules()

    def get_rule_status(self, rule_name: str) -> str:
        """Get status of a specific rule: 'active', 'disabled', or 'unknown'."""
        return self.graph.control_layer.get_rule_status(rule_name)

    def disable_rule(self, rule_name: str) -> 'NoneValue':
        """Disable a specific edge rule."""
        from .values import NoneValue
        self.graph.control_layer.disable_rule(rule_name)
        return NoneValue()

    def enable_rule(self, rule_name: str) -> 'NoneValue':
        """Re-enable a previously disabled edge rule."""
        from .values import NoneValue
        self.graph.control_layer.enable_rule(rule_name)
        return NoneValue()

    # Visualization methods
    def get_graph_summary(self) -> Dict[str, Any]:
        """Get a summary of the graph structure."""
        return self.graph.control_layer.get_graph_summary()

    def visualize_structure(self, format: str = "text") -> str:
        """Visualize the graph structure in different formats."""
        return self.graph.control_layer.visualize_structure(format)

    def _deep_freeze(self):
        """Freeze all values in the hash (deep freeze)."""
        for value in self.values():
            value.freeze()
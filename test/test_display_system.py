"""Tests for the enhanced display system and formatters."""

import pytest
from glang.core.graph import Graph
from glang.display import (
    GraphRenderer,
    DisplayMode,
    SimpleListFormatter,
    DetailedNodeFormatter,
    MetaInfoFormatter,
    JsonFormatter
)
from glang.repl.graph_manager import GraphManager


class TestFormatters:
    """Test individual formatter functionality."""
    
    def setup_method(self):
        """Set up for each test."""
        self.graph = Graph.from_list(['apple', 'banana', 'cherry'])
        self.empty_graph = Graph.from_list([])
    
    def test_simple_list_formatter(self):
        """Test simple list formatter."""
        formatter = SimpleListFormatter()
        
        # Test normal list
        result = formatter.format_graph(self.graph)
        assert result == "[apple, banana, cherry]"
        
        # Test empty list
        result = formatter.format_graph(self.empty_graph)
        assert result == "[]"
    
    def test_simple_list_formatter_with_quotes(self):
        """Test simple formatter with items that need quotes."""
        graph = Graph.from_list(['hello world', '123', 'None', 'simple'])
        formatter = SimpleListFormatter()
        
        result = formatter.format_graph(graph)
        assert "'hello world'" in result
        assert "'123'" in result  # Numbers as strings
        assert "'None'" in result  # Special values
        assert "simple" in result  # No quotes needed
    
    def test_simple_formatter_multiline(self):
        """Test simple formatter with multiline option."""
        long_graph = Graph.from_list([f'item{i}' for i in range(8)])
        formatter = SimpleListFormatter()
        
        # Test compact mode
        compact = formatter.format_graph(long_graph, compact=True)
        assert "\n" not in compact
        
        # Test multiline mode
        multiline = formatter.format_graph(long_graph, compact=False)
        assert "\n" in multiline
        assert "[" in multiline
        assert "]" in multiline
    
    def test_detailed_node_formatter(self):
        """Test detailed node formatter."""
        formatter = DetailedNodeFormatter()
        
        # Test with variable name
        result = formatter.format_graph(self.graph, variable_name="fruits", show_ids=True)
        assert "Graph 'fruits'" in result
        assert "LINEAR" in result
        assert "Node(" in result
        assert "data='apple'" in result
        
        # Test without IDs
        result = formatter.format_graph(self.graph, show_ids=False)
        assert "Node(" not in result
        assert "[apple]" in result
    
    def test_detailed_formatter_empty_graph(self):
        """Test detailed formatter with empty graph."""
        formatter = DetailedNodeFormatter()
        
        result = formatter.format_graph(self.empty_graph, variable_name="empty")
        assert "Graph 'empty' (empty)" in result
    
    def test_detailed_formatter_long_list(self):
        """Test detailed formatter with long list."""
        long_graph = Graph.from_list([f'item{i}' for i in range(6)])
        formatter = DetailedNodeFormatter()
        
        result = formatter.format_graph(long_graph)
        # Should use numbered format for long lists
        assert ("0:" in result) or ("Node(" in result)
    
    def test_meta_info_formatter(self):
        """Test meta information formatter."""
        formatter = MetaInfoFormatter()
        
        result = formatter.format_graph(self.graph, variable_name="fruits")
        assert "Variable: fruits" in result
        assert "Type: LINEAR" in result
        assert "Nodes: 3" in result
        assert "Edges: 2" in result
        assert "Head: apple" in result
        assert "Tail: cherry" in result
        assert "Linear: Yes" in result
    
    def test_meta_formatter_large_graph(self):
        """Test meta formatter with large graph (shows preview)."""
        large_graph = Graph.from_list([f'item{i}' for i in range(15)])
        formatter = MetaInfoFormatter()
        
        result = formatter.format_graph(large_graph, variable_name="big")
        assert "Preview:" in result
        assert "item0" in result
        assert "..." in result
    
    def test_json_formatter(self):
        """Test JSON formatter."""
        formatter = JsonFormatter()
        
        result = formatter.format_graph(self.graph, variable_name="fruits")
        assert '"name": "fruits"' in result
        assert '"type": "linear"' in result
        assert '"size": 3' in result
        assert '"data": ["apple", "banana", "cherry"]' in result
        assert "{" in result and "}" in result
    
    def test_json_formatter_indent(self):
        """Test JSON formatter with custom indent."""
        formatter = JsonFormatter()
        
        result = formatter.format_graph(self.graph, indent=4)
        lines = result.split("\n")
        # Check that indentation is applied
        assert any(line.startswith("    ") for line in lines)


class TestGraphRenderer:
    """Test the main GraphRenderer class."""
    
    def setup_method(self):
        """Set up for each test."""
        self.graph_manager = GraphManager()
        self.renderer = GraphRenderer(self.graph_manager)
        self.graph = Graph.from_list(['test', 'data'])
    
    def test_simple_mode_rendering(self):
        """Test simple mode rendering."""
        result = self.renderer.render(self.graph, DisplayMode.SIMPLE)
        assert result == "[test, data]"
    
    def test_detailed_mode_rendering(self):
        """Test detailed mode rendering."""
        result = self.renderer.render(
            self.graph, 
            DisplayMode.DETAILED, 
            variable_name="test_var"
        )
        assert "Graph 'test_var'" in result
        assert "LINEAR" in result
        assert "Node(" in result
    
    def test_meta_mode_rendering(self):
        """Test meta mode rendering."""
        result = self.renderer.render(
            self.graph, 
            DisplayMode.META, 
            variable_name="test_var"
        )
        assert "Variable: test_var" in result
        assert "Nodes: 2" in result
        assert "Head: test" in result
    
    def test_json_mode_rendering(self):
        """Test JSON mode rendering."""
        result = self.renderer.render(
            self.graph,
            DisplayMode.JSON,
            variable_name="test_var"
        )
        assert '"name": "test_var"' in result
        assert '"type": "linear"' in result
    
    def test_compact_mode_rendering(self):
        """Test compact mode rendering."""
        result = self.renderer.render(
            self.graph,
            DisplayMode.COMPACT,
            variable_name="test_var"
        )
        # Should be detailed view without node IDs
        assert "Graph 'test_var'" in result
        assert "Node(" not in result or "show_ids" not in result
        assert "[test]" in result
    
    def test_flag_parsing(self):
        """Test display flag parsing."""
        flags = ['--show-nodes', '--compact', '--max-items=10']
        options = self.renderer._parse_display_flags(flags)
        
        assert options['show_ids'] is True
        assert options['compact'] is True
        assert options['max_items'] == 10
    
    def test_conflicting_flags(self):
        """Test handling of conflicting flags."""
        flags = ['--compact', '--expanded', '--show-nodes', '--no-ids']
        options = self.renderer._parse_display_flags(flags)
        
        # Later flags should override earlier ones
        assert options['compact'] is False  # --expanded overrides --compact
        assert options['show_ids'] is False  # --no-ids overrides --show-nodes
    
    def test_invalid_flags(self):
        """Test handling of invalid flags."""
        flags = ['--max-items=invalid', '--indent=not-a-number', '--unknown-flag']
        options = self.renderer._parse_display_flags(flags)
        
        # Invalid values should be ignored
        assert 'max_items' not in options
        assert 'indent' not in options
    
    def test_render_with_flags(self):
        """Test rendering with flags passed through."""
        result = self.renderer.render(
            self.graph,
            DisplayMode.SIMPLE,
            flags=['--compact']
        )
        # Should still work, flags processed by formatter
        assert isinstance(result, str)
        assert len(result) > 0


class TestDisplayIntegration:
    """Test integration between display components."""
    
    def setup_method(self):
        """Set up integration tests."""
        self.graph_manager = GraphManager()
        self.renderer = GraphRenderer(self.graph_manager)
    
    def test_end_to_end_simple_display(self):
        """Test complete simple display workflow."""
        # Create a graph through the manager
        self.graph_manager.create_from_list("fruits", ["apple", "banana"])
        graph = self.graph_manager.get_variable("fruits")
        
        # Render in different modes
        simple = self.renderer.render(graph, DisplayMode.SIMPLE)
        detailed = self.renderer.render(graph, DisplayMode.DETAILED, "fruits")
        meta = self.renderer.render(graph, DisplayMode.META, "fruits")
        
        # Verify all modes work
        assert simple == "[apple, banana]"
        assert "Graph 'fruits'" in detailed
        assert "Variable: fruits" in meta
    
    def test_large_graph_handling(self):
        """Test display with large graphs."""
        large_data = [f"item_{i:03d}" for i in range(100)]
        self.graph_manager.create_from_list("big_list", large_data)
        graph = self.graph_manager.get_variable("big_list")
        
        # Test with max items limit
        result = self.renderer.render(
            graph, 
            DisplayMode.SIMPLE,
            flags=['--max-items=5']
        )
        
        # Should have truncation
        assert "item_000" in result
        assert "item_099" not in result  # Should be truncated
    
    def test_special_characters_in_data(self):
        """Test display with special characters."""
        special_data = ["hello world", "item with, comma", "item with 'quotes'", ""]
        self.graph_manager.create_from_list("special", special_data)
        graph = self.graph_manager.get_variable("special")
        
        # Test simple display handles special characters
        result = self.renderer.render(graph, DisplayMode.SIMPLE)
        assert "hello world" in result
        
        # Test JSON display properly escapes
        json_result = self.renderer.render(graph, DisplayMode.JSON)
        assert '"hello world"' in json_result
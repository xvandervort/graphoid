"""Graph rendering for different display modes."""

from enum import Enum, auto
from typing import Optional, List, Dict, Any

from ..core.graph import Graph
from .formatters import (
    SimpleListFormatter,
    DetailedNodeFormatter,
    MetaInfoFormatter,
    JsonFormatter
)


class DisplayMode(Enum):
    """Display modes for graph rendering."""
    SIMPLE = auto()     # [item1, item2, item3]
    DETAILED = auto()   # Node -> Node -> Node
    META = auto()       # Variable info
    JSON = auto()       # JSON-like output
    COMPACT = auto()    # Compact detailed view


class GraphRenderer:
    """Renders graphs in different display modes."""
    
    def __init__(self, graph_manager=None):
        self.graph_manager = graph_manager
        
        # Initialize formatters
        self.simple_formatter = SimpleListFormatter()
        self.detailed_formatter = DetailedNodeFormatter()
        self.meta_formatter = MetaInfoFormatter()
        self.json_formatter = JsonFormatter()
    
    def render(self, graph: Graph, mode: DisplayMode = DisplayMode.SIMPLE, 
               variable_name: Optional[str] = None, 
               flags: Optional[List[str]] = None, **kwargs) -> str:
        """Render a graph in the specified display mode with optional flags."""
        
        # Parse flags for additional options
        options = self._parse_display_flags(flags or [])
        options.update(kwargs)
        
        if mode == DisplayMode.SIMPLE:
            return self.simple_formatter.format_graph(graph, **options)
        elif mode == DisplayMode.DETAILED:
            return self.detailed_formatter.format_graph(graph, variable_name=variable_name, **options)
        elif mode == DisplayMode.META:
            return self.meta_formatter.format_graph(graph, variable_name=variable_name, **options)
        elif mode == DisplayMode.JSON:
            return self.json_formatter.format_graph(graph, variable_name=variable_name, **options)
        elif mode == DisplayMode.COMPACT:
            # Compact detailed view
            options['show_ids'] = False
            return self.detailed_formatter.format_graph(graph, variable_name=variable_name, **options)
        else:
            return str(graph)
    
    def _parse_display_flags(self, flags: List[str]) -> Dict[str, Any]:
        """Parse display flags into options dictionary."""
        options = {}
        
        for flag in flags:
            if flag == '--show-nodes':
                options['show_ids'] = True
            elif flag == '--no-ids':
                options['show_ids'] = False
            elif flag == '--compact':
                options['compact'] = True
            elif flag == '--expanded' or flag == '--multiline':
                options['compact'] = False
            elif flag == '--info':
                # This changes the mode, handled in calling code
                pass
            elif flag == '--json':
                # This changes the mode, handled in calling code
                pass
            elif flag.startswith('--max-items='):
                try:
                    options['max_items'] = int(flag.split('=')[1])
                except (IndexError, ValueError):
                    pass  # Ignore invalid format
            elif flag.startswith('--indent='):
                try:
                    options['indent'] = int(flag.split('=')[1])
                except (IndexError, ValueError):
                    pass  # Ignore invalid format
        
        return options
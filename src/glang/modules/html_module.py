"""
Glang HTML Parsing Module

Provides HTML parsing and web scraping capabilities using Python's html.parser
with Glang-native string processing where possible.
"""

from typing import Optional, Dict, List, Any
from ..execution.values import GlangValue, StringValue, BooleanValue, NumberValue, DataValue
from ..execution.graph_values import ListValue, HashValue
from ..ast.nodes import SourcePosition
import html.parser
import html


class GlangHTMLParser(html.parser.HTMLParser):
    """HTML parser that builds Glang-native data structures."""

    def __init__(self):
        super().__init__()
        self.elements = []
        self.current_element = None
        self.stack = []

    def handle_starttag(self, tag, attrs):
        """Handle opening HTML tags."""
        # Create element data structure
        element_pairs = [
            ("tag", DataValue("tag", StringValue(tag))),
            ("type", DataValue("type", StringValue("element"))),
            ("attributes", DataValue("attributes", self._attrs_to_hash(attrs))),
            ("children", DataValue("children", ListValue([]))),
            ("text", DataValue("text", StringValue("")))
        ]

        element = HashValue(element_pairs)

        # Handle nesting
        if self.current_element:
            children = self.current_element.graph.get("children").value
            children.append(element)
            self.stack.append(self.current_element)
        else:
            self.elements.append(element)

        self.current_element = element

    def handle_endtag(self, tag):
        """Handle closing HTML tags."""
        if self.stack:
            self.current_element = self.stack.pop()
        else:
            self.current_element = None

    def handle_data(self, data):
        """Handle text content."""
        if self.current_element and data.strip():
            current_text = self.current_element.graph.get("text").value
            new_text = StringValue(current_text.value + data.strip())
            text_data = DataValue("text", new_text)
            # Update the text in the current element
            self.current_element.graph.set("text", text_data)

    def _attrs_to_hash(self, attrs):
        """Convert HTML attributes to Glang hash."""
        attr_pairs = []
        for name, value in attrs:
            attr_pairs.append((name, DataValue(name, StringValue(value or ""))))
        return HashValue(attr_pairs)


class HTMLModule:
    """HTML parsing operations using Glang-native processing where possible."""

    @staticmethod
    def parse_html(html_content: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Parse HTML content into Glang data structures."""
        if not isinstance(html_content, StringValue):
            raise RuntimeError(f"parse_html expects string content, got {html_content.get_type()}", position)

        parser = GlangHTMLParser()
        try:
            parser.feed(html_content.value)
        except Exception as e:
            raise RuntimeError(f"HTML parsing error: {str(e)}", position)

        # Return the parsed elements as a list
        return ListValue(parser.elements, position=position)

    @staticmethod
    def find_elements_by_tag(html_elements: GlangValue, tag: GlangValue,
                           position: Optional[SourcePosition] = None) -> GlangValue:
        """Find all elements with a specific tag name."""
        if not isinstance(html_elements, ListValue):
            raise RuntimeError(f"find_elements_by_tag expects list, got {html_elements.get_type()}", position)

        if not isinstance(tag, StringValue):
            raise RuntimeError(f"find_elements_by_tag expects string tag, got {tag.get_type()}", position)

        found_elements = []
        target_tag = tag.value.lower()

        def search_recursive(elements_list):
            for element in elements_list.elements:
                if isinstance(element, HashValue):
                    element_tag = element.graph.get("tag")
                    if element_tag and isinstance(element_tag.value, StringValue):
                        if element_tag.value.value.lower() == target_tag:
                            found_elements.append(element)

                    # Search children recursively
                    children = element.graph.get("children")
                    if children and isinstance(children.value, ListValue):
                        search_recursive(children.value)

        search_recursive(html_elements)
        return ListValue(found_elements, position=position)

    @staticmethod
    def find_element_by_id(html_elements: GlangValue, element_id: GlangValue,
                          position: Optional[SourcePosition] = None) -> GlangValue:
        """Find element with specific ID attribute."""
        if not isinstance(html_elements, ListValue):
            raise RuntimeError(f"find_element_by_id expects list, got {html_elements.get_type()}", position)

        if not isinstance(element_id, StringValue):
            raise RuntimeError(f"find_element_by_id expects string ID, got {element_id.get_type()}", position)

        target_id = element_id.value

        def search_recursive(elements_list):
            for element in elements_list.elements:
                if isinstance(element, HashValue):
                    attrs = element.graph.get("attributes")
                    if attrs and isinstance(attrs.value, HashValue):
                        id_attr = attrs.value.graph.get("id")
                        if id_attr and isinstance(id_attr.value, StringValue):
                            if id_attr.value.value == target_id:
                                return element

                    # Search children recursively
                    children = element.graph.get("children")
                    if children and isinstance(children.value, ListValue):
                        result = search_recursive(children.value)
                        if result:
                            return result
            return None

        result = search_recursive(html_elements)
        if result:
            return result
        else:
            # Return none/null - for now return empty hash
            return HashValue([], position)

    @staticmethod
    def find_elements_by_class(html_elements: GlangValue, class_name: GlangValue,
                              position: Optional[SourcePosition] = None) -> GlangValue:
        """Find all elements with specific CSS class."""
        if not isinstance(html_elements, ListValue):
            raise RuntimeError(f"find_elements_by_class expects list, got {html_elements.get_type()}", position)

        if not isinstance(class_name, StringValue):
            raise RuntimeError(f"find_elements_by_class expects string class, got {class_name.get_type()}", position)

        found_elements = []
        target_class = class_name.value

        def search_recursive(elements_list):
            for element in elements_list.elements:
                if isinstance(element, HashValue):
                    attrs = element.graph.get("attributes")
                    if attrs and isinstance(attrs.value, HashValue):
                        class_attr = attrs.value.graph.get("class")
                        if class_attr and isinstance(class_attr.value, StringValue):
                            # Check if target class is in the class list
                            classes = class_attr.value.value.split()
                            if target_class in classes:
                                found_elements.append(element)

                    # Search children recursively
                    children = element.graph.get("children")
                    if children and isinstance(children.value, ListValue):
                        search_recursive(children.value)

        search_recursive(html_elements)
        return ListValue(found_elements, position=position)

    @staticmethod
    def get_element_text(element: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Get text content from an HTML element."""
        if not isinstance(element, HashValue):
            raise RuntimeError(f"get_element_text expects hash element, got {element.get_type()}", position)

        text_content = []

        def collect_text(elem):
            if isinstance(elem, HashValue):
                # Get direct text
                text = elem.graph.get("text")
                if text and isinstance(text.value, StringValue) and text.value.value:
                    text_content.append(text.value.value)

                # Get text from children
                children = elem.graph.get("children")
                if children and isinstance(children.value, ListValue):
                    for child in children.value.elements:
                        collect_text(child)

        collect_text(element)
        return StringValue(" ".join(text_content), position)

    @staticmethod
    def get_element_attribute(element: GlangValue, attr_name: GlangValue,
                             position: Optional[SourcePosition] = None) -> GlangValue:
        """Get specific attribute value from an HTML element."""
        if not isinstance(element, HashValue):
            raise RuntimeError(f"get_element_attribute expects hash element, got {element.get_type()}", position)

        if not isinstance(attr_name, StringValue):
            raise RuntimeError(f"get_element_attribute expects string attr name, got {attr_name.get_type()}", position)

        attrs = element.graph.get("attributes")
        if attrs and isinstance(attrs.value, HashValue):
            attr = attrs.value.graph.get(attr_name.value)
            if attr and isinstance(attr.value, StringValue):
                return attr.value

        # Return empty string if attribute not found
        return StringValue("", position)

    @staticmethod
    def html_decode(text: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Decode HTML entities in text."""
        if not isinstance(text, StringValue):
            raise RuntimeError(f"html_decode expects string text, got {text.get_type()}", position)

        decoded = html.unescape(text.value)
        return StringValue(decoded, position)

    @staticmethod
    def html_encode(text: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Encode text for safe HTML display."""
        if not isinstance(text, StringValue):
            raise RuntimeError(f"html_encode expects string text, got {text.get_type()}", position)

        encoded = html.escape(text.value)
        return StringValue(encoded, position)


def create_html_module() -> 'ModuleNamespace':
    """Create the HTML module namespace with all functions."""
    from ..modules.module_manager import ModuleNamespace
    from ..execution.function_value import BuiltinFunctionValue

    namespace = ModuleNamespace("html")

    # HTML parsing functions organized by category
    html_functions = {
        # Core parsing
        'parse': HTMLModule.parse_html,

        # Element finding
        'find_by_tag': HTMLModule.find_elements_by_tag,
        'find_by_id': HTMLModule.find_element_by_id,
        'find_by_class': HTMLModule.find_elements_by_class,

        # Content extraction
        'get_text': HTMLModule.get_element_text,
        'get_attribute': HTMLModule.get_element_attribute,

        # Encoding/decoding
        'decode': HTMLModule.html_decode,
        'encode': HTMLModule.html_encode,
    }

    # Wrap functions as callable values
    for name, func in html_functions.items():
        wrapped_func = BuiltinFunctionValue(name, func)
        namespace.set_symbol(name, wrapped_func, export=True)

    return namespace
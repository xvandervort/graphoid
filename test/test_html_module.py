#!/usr/bin/env python3
"""Test the HTML parsing module functionality."""

import pytest
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../src'))

from glang.execution.values import StringValue, BooleanValue, NumberValue, DataValue
from glang.execution.graph_values import ListValue, HashValue
from glang.modules.html_module import HTMLModule
from glang.ast.nodes import SourcePosition


class TestHTMLModule:
    """Test the HTMLModule class."""

    def test_parse_simple_html(self):
        """Test parsing simple HTML content."""
        html_content = StringValue('<p>Hello World</p>')

        result = HTMLModule.parse_html(html_content)

        assert isinstance(result, ListValue)
        assert len(result.elements) == 1

        # Check the parsed element
        element = result.elements[0]
        assert isinstance(element, HashValue)

        tag = element.graph.get("tag")
        assert tag and isinstance(tag.value, StringValue)
        assert tag.value.value == "p"

        text = element.graph.get("text")
        assert text and isinstance(text.value, StringValue)
        assert text.value.value == "Hello World"

    def test_parse_html_with_attributes(self):
        """Test parsing HTML with attributes."""
        html_content = StringValue('<div class="container" id="main">Content</div>')

        result = HTMLModule.parse_html(html_content)

        assert isinstance(result, ListValue)
        element = result.elements[0]

        # Check attributes
        attrs = element.graph.get("attributes")
        assert attrs and isinstance(attrs.value, HashValue)

        class_attr = attrs.value.graph.get("class")
        assert class_attr and isinstance(class_attr.value, StringValue)
        assert class_attr.value.value == "container"

        id_attr = attrs.value.graph.get("id")
        assert id_attr and isinstance(id_attr.value, StringValue)
        assert id_attr.value.value == "main"

    def test_parse_nested_html(self):
        """Test parsing nested HTML elements."""
        html_content = StringValue('<div><p>Paragraph</p><span>Span text</span></div>')

        result = HTMLModule.parse_html(html_content)

        assert isinstance(result, ListValue)
        div_element = result.elements[0]

        # Check children
        children = div_element.graph.get("children")
        assert children and isinstance(children.value, ListValue)
        assert len(children.value.elements) == 2

        # Check first child (p)
        p_element = children.value.elements[0]
        p_tag = p_element.graph.get("tag")
        assert p_tag.value.value == "p"

        p_text = p_element.graph.get("text")
        assert p_text.value.value == "Paragraph"

        # Check second child (span)
        span_element = children.value.elements[1]
        span_tag = span_element.graph.get("tag")
        assert span_tag.value.value == "span"

        span_text = span_element.graph.get("text")
        assert span_text.value.value == "Span text"

    def test_find_elements_by_tag(self):
        """Test finding elements by tag name."""
        html_content = StringValue('<div><p>First</p><p>Second</p><span>Span</span></div>')
        parsed = HTMLModule.parse_html(html_content)

        # Find all p elements
        p_elements = HTMLModule.find_elements_by_tag(parsed, StringValue("p"))

        assert isinstance(p_elements, ListValue)
        assert len(p_elements.elements) == 2

        # Check first p element
        first_p = p_elements.elements[0]
        first_p_text = first_p.graph.get("text")
        assert first_p_text.value.value == "First"

        # Check second p element
        second_p = p_elements.elements[1]
        second_p_text = second_p.graph.get("text")
        assert second_p_text.value.value == "Second"

    def test_find_element_by_id(self):
        """Test finding element by ID."""
        html_content = StringValue('<div><p id="first">First paragraph</p><p id="second">Second paragraph</p></div>')
        parsed = HTMLModule.parse_html(html_content)

        # Find element with specific ID
        element = HTMLModule.find_element_by_id(parsed, StringValue("second"))

        assert isinstance(element, HashValue)
        text = element.graph.get("text")
        assert text.value.value == "Second paragraph"

    def test_find_element_by_id_not_found(self):
        """Test finding non-existent element by ID."""
        html_content = StringValue('<div><p id="first">First paragraph</p></div>')
        parsed = HTMLModule.parse_html(html_content)

        # Find element with non-existent ID
        element = HTMLModule.find_element_by_id(parsed, StringValue("nonexistent"))

        assert isinstance(element, HashValue)
        # Should return empty hash when not found
        assert len(element.graph.keys()) == 0

    def test_find_elements_by_class(self):
        """Test finding elements by CSS class."""
        html_content = StringValue('<div><p class="highlight">First</p><p class="normal">Second</p><span class="highlight">Span</span></div>')
        parsed = HTMLModule.parse_html(html_content)

        # Find elements with specific class
        elements = HTMLModule.find_elements_by_class(parsed, StringValue("highlight"))

        assert isinstance(elements, ListValue)
        assert len(elements.elements) == 2

        # Check first element (p)
        first_element = elements.elements[0]
        first_tag = first_element.graph.get("tag")
        assert first_tag.value.value == "p"

        # Check second element (span)
        second_element = elements.elements[1]
        second_tag = second_element.graph.get("tag")
        assert second_tag.value.value == "span"

    def test_get_element_text(self):
        """Test extracting text from element."""
        html_content = StringValue('<div>Hello <span>World</span> Test</div>')
        parsed = HTMLModule.parse_html(html_content)

        div_element = parsed.elements[0]
        text = HTMLModule.get_element_text(div_element)

        assert isinstance(text, StringValue)
        # Should combine text from element and its children
        assert "Hello" in text.value
        assert "World" in text.value

    def test_get_element_attribute(self):
        """Test getting element attribute."""
        html_content = StringValue('<a href="https://example.com" title="Example Link">Link</a>')
        parsed = HTMLModule.parse_html(html_content)

        link_element = parsed.elements[0]

        # Get href attribute
        href = HTMLModule.get_element_attribute(link_element, StringValue("href"))
        assert isinstance(href, StringValue)
        assert href.value == "https://example.com"

        # Get title attribute
        title = HTMLModule.get_element_attribute(link_element, StringValue("title"))
        assert isinstance(title, StringValue)
        assert title.value == "Example Link"

        # Get non-existent attribute
        nonexistent = HTMLModule.get_element_attribute(link_element, StringValue("nonexistent"))
        assert isinstance(nonexistent, StringValue)
        assert nonexistent.value == ""

    def test_html_decode(self):
        """Test HTML entity decoding."""
        encoded_text = StringValue("&lt;div&gt;Hello &amp; World&lt;/div&gt;")

        decoded = HTMLModule.html_decode(encoded_text)

        assert isinstance(decoded, StringValue)
        assert decoded.value == "<div>Hello & World</div>"

    def test_html_encode(self):
        """Test HTML encoding."""
        raw_text = StringValue("<div>Hello & World</div>")

        encoded = HTMLModule.html_encode(raw_text)

        assert isinstance(encoded, StringValue)
        assert encoded.value == "&lt;div&gt;Hello &amp; World&lt;/div&gt;"

    def test_parse_invalid_html_type(self):
        """Test parsing with invalid input type."""
        with pytest.raises(RuntimeError, match="parse_html expects string content"):
            HTMLModule.parse_html(NumberValue(123))

    def test_find_elements_invalid_input(self):
        """Test finding elements with invalid input types."""
        with pytest.raises(RuntimeError, match="find_elements_by_tag expects list"):
            HTMLModule.find_elements_by_tag(StringValue("not a list"), StringValue("p"))

        html_content = StringValue('<p>Test</p>')
        parsed = HTMLModule.parse_html(html_content)

        with pytest.raises(RuntimeError, match="find_elements_by_tag expects string tag"):
            HTMLModule.find_elements_by_tag(parsed, NumberValue(123))

    def test_complex_html_structure(self):
        """Test parsing more complex HTML structure."""
        html_content = StringValue('''
        <html>
            <head><title>Test Page</title></head>
            <body>
                <div class="header">
                    <h1>Main Title</h1>
                    <nav>
                        <a href="/home">Home</a>
                        <a href="/about">About</a>
                    </nav>
                </div>
                <div class="content">
                    <p>Paragraph 1</p>
                    <p>Paragraph 2</p>
                </div>
            </body>
        </html>
        ''')

        parsed = HTMLModule.parse_html(html_content)

        assert isinstance(parsed, ListValue)
        # Should have parsed the HTML structure

        # Find all links
        links = HTMLModule.find_elements_by_tag(parsed, StringValue("a"))
        assert len(links.elements) == 2

        # Find all paragraphs
        paragraphs = HTMLModule.find_elements_by_tag(parsed, StringValue("p"))
        assert len(paragraphs.elements) == 2

        # Find header div by class
        header_divs = HTMLModule.find_elements_by_class(parsed, StringValue("header"))
        assert len(header_divs.elements) == 1


if __name__ == '__main__':
    pytest.main([__file__])
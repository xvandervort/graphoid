#!/usr/bin/env python3
"""Test the pure Glang network module functionality."""

import pytest

from glang.execution.pipeline import ExecutionSession
from glang.execution.values import StringValue, BooleanValue, NumberValue, DataValue
from glang.execution.graph_values import ListValue, HashValue


class TestNetworkGlangModule:
    """Test the pure Glang network module (stdlib/network.gr)."""

    def setup_method(self):
        """Set up test environment."""
        self.session = ExecutionSession()

    def test_parse_url_https_with_path(self):
        """Test URL parsing with HTTPS and path."""
        # Import the network module
        result = self.session.execute_statement('import "network" as net')
        assert result.success

        # Parse a URL
        result = self.session.execute_statement('net.parse_url("https://example.com/path/to/file")')
        assert result.success
        assert isinstance(result.value, HashValue)

        # Check the parsed components
        parsed_hash = result.value
        assert "protocol" in parsed_hash.graph.keys()
        assert "host" in parsed_hash.graph.keys()
        assert "path" in parsed_hash.graph.keys()

        protocol_data = parsed_hash.graph.get("protocol")
        host_data = parsed_hash.graph.get("host")
        path_data = parsed_hash.graph.get("path")

        # The hash values are StringValue objects directly
        assert isinstance(protocol_data, StringValue)
        assert protocol_data.value == "https"

        assert isinstance(host_data, StringValue)
        assert host_data.value == "example.com"

        assert isinstance(path_data, StringValue)
        assert path_data.value == "/path/to/file"

    def test_url_encoding(self):
        """Test URL encoding of special characters."""
        # Import the network module
        result = self.session.execute_statement('import "network" as net')
        assert result.success

        # Test URL encoding
        result = self.session.execute_statement('net.encode_url("hello world & special=chars")')
        assert result.success
        assert isinstance(result.value, StringValue)
        encoded = result.value.value

        # Should encode space, &, and =
        assert "%20" in encoded  # space
        assert "%26" in encoded  # &
        assert "%3D" in encoded  # =

    def test_url_decoding(self):
        """Test URL decoding."""
        # Import the network module
        result = self.session.execute_statement('import "network" as net')
        assert result.success

        # Test URL decoding
        result = self.session.execute_statement('net.decode_url("hello%20world%26test%3Dvalue")')
        assert result.success
        assert isinstance(result.value, StringValue)
        assert result.value.value == "hello world&test=value"

    def test_extract_domain(self):
        """Test domain extraction."""
        # Import the network module
        result = self.session.execute_statement('import "network" as net')
        assert result.success

        # Test domain extraction
        result = self.session.execute_statement('net.extract_domain("https://www.example.com:8080/path")')
        assert result.success
        assert isinstance(result.value, StringValue)
        assert result.value.value == "example.com"  # Should strip www and port

    def test_is_valid_url(self):
        """Test URL validation."""
        # Import the network module
        result = self.session.execute_statement('import "network" as net')
        assert result.success

        # Test valid URL
        result = self.session.execute_statement('net.is_valid_url("https://example.com/path")')
        assert result.success
        assert isinstance(result.value, BooleanValue)
        assert result.value.value is True

        # Test invalid URL
        result = self.session.execute_statement('net.is_valid_url("not-a-valid-url")')
        assert result.success
        assert isinstance(result.value, BooleanValue)
        assert result.value.value is False

    def test_http_functions_return_error_messages(self):
        """Test that HTTP functions return helpful error messages."""
        # Import the network module
        result = self.session.execute_statement('import "network" as net')
        assert result.success

        # Test that HTTP functions return error messages
        result = self.session.execute_statement('net.http_get("https://example.com")')
        assert result.success
        assert isinstance(result.value, StringValue)
        assert "ERROR: HTTP operations not yet implemented" in result.value.value


if __name__ == '__main__':
    pytest.main([__file__])
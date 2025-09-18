#!/usr/bin/env python3
"""Test the network module functionality."""

import pytest
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../src'))

from glang.execution.values import StringValue, BooleanValue, NumberValue, DataValue
from glang.execution.graph_values import HashValue
from glang.modules.network_module import NetworkModule
from glang.modules.network_interface import NetworkResponse, PythonNetworkProvider
from glang.ast.nodes import SourcePosition


class MockNetworkProvider:
    """Mock network provider for testing."""

    def __init__(self):
        self.available = True
        self.requests = []  # Track requests made
        self.mock_responses = {}  # URL -> NetworkResponse mapping

    def is_available(self) -> bool:
        return self.available

    def set_mock_response(self, url: str, response: NetworkResponse):
        """Set a mock response for a URL."""
        self.mock_responses[url] = response

    def http_request(self, method: str, url: str, data=None, headers=None) -> NetworkResponse:
        """Mock HTTP request."""
        # Record the request
        self.requests.append({
            'method': method,
            'url': url,
            'data': data,
            'headers': headers or {}
        })

        # Return mock response if available
        if url in self.mock_responses:
            return self.mock_responses[url]

        # Default successful response
        return NetworkResponse(200, f"Mock response for {method} {url}", {"Content-Type": "text/plain"})

    def download_to_file(self, url: str, filepath: str) -> bool:
        """Mock file download."""
        self.requests.append({
            'method': 'DOWNLOAD',
            'url': url,
            'filepath': filepath
        })
        return True


class TestNetworkModule:
    """Test the NetworkModule class."""

    def setup_method(self):
        """Set up test environment."""
        self.mock_provider = MockNetworkProvider()
        # Replace the global provider
        import glang.modules.network_interface as net_interface
        self.original_provider = net_interface._network_provider
        net_interface._network_provider = self.mock_provider

    def teardown_method(self):
        """Clean up test environment."""
        import glang.modules.network_interface as net_interface
        net_interface._network_provider = self.original_provider

    def test_http_get_success(self):
        """Test successful HTTP GET request."""
        url = StringValue("https://example.com/api")

        # Set up mock response
        self.mock_provider.set_mock_response(
            "https://example.com/api",
            NetworkResponse(200, '{"status": "ok"}', {"Content-Type": "application/json"})
        )

        result = NetworkModule.http_get(url)

        assert isinstance(result, StringValue)
        assert result.value == '{"status": "ok"}'

        # Verify request was made
        assert len(self.mock_provider.requests) == 1
        assert self.mock_provider.requests[0]['method'] == 'GET'
        assert self.mock_provider.requests[0]['url'] == "https://example.com/api"

    def test_http_get_with_error(self):
        """Test HTTP GET with error response."""
        url = StringValue("https://example.com/notfound")

        # Set up error response
        self.mock_provider.set_mock_response(
            "https://example.com/notfound",
            NetworkResponse(404, "Not Found", {})
        )

        with pytest.raises(RuntimeError, match="HTTP error 404"):
            NetworkModule.http_get(url)

    def test_http_get_invalid_url_type(self):
        """Test HTTP GET with invalid URL type."""
        url = NumberValue(123)

        with pytest.raises(RuntimeError, match="http_get expects string URL, got num"):
            NetworkModule.http_get(url)

    def test_http_post_with_string_data(self):
        """Test HTTP POST with string data."""
        url = StringValue("https://example.com/api")
        data = StringValue("test_data=hello")

        result = NetworkModule.http_post(url, data)

        assert isinstance(result, StringValue)

        # Verify request was made with data
        assert len(self.mock_provider.requests) == 1
        request = self.mock_provider.requests[0]
        assert request['method'] == 'POST'
        assert request['url'] == "https://example.com/api"
        assert request['data'] == "test_data=hello"

    def test_http_post_with_non_string_data(self):
        """Test HTTP POST with non-string data (should convert to display string)."""
        url = StringValue("https://example.com/api")
        data = NumberValue(42)

        result = NetworkModule.http_post(url, data)

        assert isinstance(result, StringValue)

        # Verify data was converted
        assert len(self.mock_provider.requests) == 1
        request = self.mock_provider.requests[0]
        assert request['data'] == "42"  # NumberValue.to_display_string()

    def test_http_request_full_with_headers(self):
        """Test full HTTP request with headers."""
        method = StringValue("GET")
        url = StringValue("https://example.com/api")
        data = None

        # Create headers hash
        headers_pairs = [
            ("Authorization", DataValue("Authorization", StringValue("Bearer token123"))),
            ("Content-Type", DataValue("Content-Type", StringValue("application/json")))
        ]
        headers = HashValue(headers_pairs)

        result = NetworkModule.http_request_full(method, url, data, headers)

        assert isinstance(result, HashValue)

        # Check that status, body, success, and headers are in result
        assert "status" in result.graph.keys()
        assert "body" in result.graph.keys()
        assert "success" in result.graph.keys()

        # Verify request headers were passed
        assert len(self.mock_provider.requests) == 1
        request = self.mock_provider.requests[0]
        assert request['headers']['Authorization'] == "Bearer token123"
        assert request['headers']['Content-Type'] == "application/json"

    def test_download_file(self):
        """Test file download functionality."""
        url = StringValue("https://example.com/file.txt")
        filepath = StringValue("/tmp/test_file.txt")

        result = NetworkModule.download_file(url, filepath)

        assert isinstance(result, BooleanValue)
        assert result.value is True

        # Verify download request was made
        assert len(self.mock_provider.requests) == 1
        request = self.mock_provider.requests[0]
        assert request['method'] == 'DOWNLOAD'
        assert request['url'] == "https://example.com/file.txt"
        assert request['filepath'] == "/tmp/test_file.txt"

    def test_url_parse_valid_url(self):
        """Test URL parsing with valid URL."""
        url = StringValue("https://example.com/path/to/resource")

        result = NetworkModule.url_parse(url)

        assert isinstance(result, HashValue)
        assert "protocol" in result.graph.keys()
        assert "host" in result.graph.keys()
        assert "path" in result.graph.keys()
        assert "url" in result.graph.keys()

        # Check parsed values
        protocol = result.graph.get("protocol").value
        host = result.graph.get("host").value
        path = result.graph.get("path").value

        assert isinstance(protocol, StringValue)
        assert protocol.value == "https"
        assert isinstance(host, StringValue)
        assert host.value == "example.com"
        assert isinstance(path, StringValue)
        assert path.value == "/path/to/resource"

    def test_url_parse_simple_url(self):
        """Test URL parsing with simple URL (no path)."""
        url = StringValue("https://example.com")

        result = NetworkModule.url_parse(url)

        assert isinstance(result, HashValue)

        protocol = result.graph.get("protocol").value
        host = result.graph.get("host").value
        path = result.graph.get("path").value

        assert protocol.value == "https"
        assert host.value == "example.com"
        assert path.value == "/"  # Should default to "/"

    def test_url_parse_invalid_url(self):
        """Test URL parsing with invalid URL."""
        url = StringValue("not-a-valid-url")

        with pytest.raises(RuntimeError, match="Invalid URL: missing protocol"):
            NetworkModule.url_parse(url)

    def test_url_encode_basic(self):
        """Test URL encoding of basic characters."""
        text = StringValue("hello world test")

        result = NetworkModule.url_encode(text)

        assert isinstance(result, StringValue)
        assert result.value == "hello%20world%20test"

    def test_url_encode_special_characters(self):
        """Test URL encoding of special characters."""
        text = StringValue("hello & world!")

        result = NetworkModule.url_encode(text)

        assert isinstance(result, StringValue)
        # Should encode space and special characters
        assert "%20" in result.value  # space
        assert "%26" in result.value  # &
        assert "%21" in result.value  # !

    def test_network_unavailable(self):
        """Test behavior when network is unavailable."""
        self.mock_provider.available = False

        url = StringValue("https://example.com")

        with pytest.raises(RuntimeError, match="Network functionality not available"):
            NetworkModule.http_get(url)


class TestNetworkInterface:
    """Test the network interface components."""

    def test_network_response_creation(self):
        """Test NetworkResponse creation and conversion."""
        response = NetworkResponse(200, '{"test": "data"}', {"Content-Type": "application/json"})

        assert response.status_code == 200
        assert response.body == '{"test": "data"}'
        assert response.headers["Content-Type"] == "application/json"

        # Test conversion to Glang values
        glang_values = response.to_glang_values()
        assert isinstance(glang_values['status'], NumberValue)
        assert glang_values['status'].value == 200
        assert isinstance(glang_values['body'], StringValue)
        assert glang_values['body'].value == '{"test": "data"}'
        assert isinstance(glang_values['success'], BooleanValue)
        assert glang_values['success'].value is True

    def test_network_response_error_status(self):
        """Test NetworkResponse with error status."""
        response = NetworkResponse(404, "Not Found", {})

        glang_values = response.to_glang_values()
        assert glang_values['status'].value == 404
        assert glang_values['success'].value is False

    def test_python_network_provider_availability(self):
        """Test PythonNetworkProvider availability check."""
        provider = PythonNetworkProvider()

        # Should be available (urllib is standard library)
        assert provider.is_available() is True


if __name__ == '__main__':
    pytest.main([__file__])
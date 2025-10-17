#!/usr/bin/env python3
"""Test the HTTP module functionality."""

import pytest
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../src'))

from unittest.mock import Mock, patch
from glang.execution.values import StringValue, BooleanValue, NumberValue
from glang.execution.graph_values import HashValue
from glang.modules.http_module import HTTPModule, create_http_module_namespace
from glang.modules.network_interface import NetworkResponse
from glang.ast.nodes import SourcePosition


class TestHTTPModule:
    """Test the HTTPModule class."""

    def test_create_namespace(self):
        """Test creating the HTTP module namespace."""
        namespace = create_http_module_namespace()

        # Check that all functions are defined as symbols
        assert 'get' in namespace.symbols
        assert 'post' in namespace.symbols
        assert 'request' in namespace.symbols
        assert 'download_file' in namespace.symbols
        assert 'is_available' in namespace.symbols

    @patch('glang.modules.http_module.get_network_provider')
    def test_http_get_success(self, mock_get_provider):
        """Test successful HTTP GET request."""
        # Mock the network provider
        mock_provider = Mock()
        mock_response = NetworkResponse(200, "Hello World", {"content-type": "text/plain"})
        mock_provider.http_request.return_value = mock_response
        mock_get_provider.return_value = mock_provider

        url = StringValue("https://example.com")
        result = HTTPModule.http_get(url)

        # Check the result
        assert isinstance(result, HashValue)
        assert result.get('status').value == 200
        assert result.get('body').value == "Hello World"
        assert result.get('success').value == True
        assert result.get('headers') is not None

        # Verify the provider was called correctly
        mock_provider.http_request.assert_called_once_with("GET", "https://example.com", None, None)

    @patch('glang.modules.http_module.get_network_provider')
    def test_http_get_with_headers(self, mock_get_provider):
        """Test HTTP GET request with custom headers."""
        # Mock the network provider
        mock_provider = Mock()
        mock_response = NetworkResponse(200, "Response", {"server": "nginx"})
        mock_provider.http_request.return_value = mock_response
        mock_get_provider.return_value = mock_provider

        url = StringValue("https://api.example.com")

        # Create headers hash
        header_pairs = [
            ('Authorization', StringValue('Bearer token123')),
            ('User-Agent', StringValue('Glang/1.0'))
        ]
        headers = HashValue(header_pairs, 'string')

        result = HTTPModule.http_get(url, headers)

        # Check the result
        assert isinstance(result, HashValue)
        assert result.get('status').value == 200

        # Verify headers were passed correctly
        expected_headers = {
            'Authorization': 'Bearer token123',
            'User-Agent': 'Glang/1.0'
        }
        mock_provider.http_request.assert_called_once_with("GET", "https://api.example.com", None, expected_headers)

    @patch('glang.modules.http_module.get_network_provider')
    def test_http_post_success(self, mock_get_provider):
        """Test successful HTTP POST request."""
        # Mock the network provider
        mock_provider = Mock()
        mock_response = NetworkResponse(201, '{"id": 123}', {"content-type": "application/json"})
        mock_provider.http_request.return_value = mock_response
        mock_get_provider.return_value = mock_provider

        url = StringValue("https://api.example.com/users")
        data = StringValue('{"name": "Alice"}')

        result = HTTPModule.http_post(url, data)

        # Check the result
        assert isinstance(result, HashValue)
        assert result.get('status').value == 201
        assert result.get('body').value == '{"id": 123}'
        assert result.get('success').value == True

        # Verify the provider was called correctly
        mock_provider.http_request.assert_called_once_with("POST", "https://api.example.com/users", '{"name": "Alice"}', None)

    @patch('glang.modules.http_module.get_network_provider')
    def test_http_request_custom_method(self, mock_get_provider):
        """Test HTTP request with custom method."""
        # Mock the network provider
        mock_provider = Mock()
        mock_response = NetworkResponse(204, "", {})
        mock_provider.http_request.return_value = mock_response
        mock_get_provider.return_value = mock_provider

        method = StringValue("DELETE")
        url = StringValue("https://api.example.com/users/123")

        result = HTTPModule.http_request(method, url)

        # Check the result
        assert isinstance(result, HashValue)
        assert result.get('status').value == 204
        assert result.get('success').value == True

        # Verify the provider was called correctly
        mock_provider.http_request.assert_called_once_with("DELETE", "https://api.example.com/users/123", None, None)

    @patch('glang.modules.http_module.get_network_provider')
    def test_http_request_error_response(self, mock_get_provider):
        """Test HTTP request with error response."""
        # Mock the network provider
        mock_provider = Mock()
        mock_response = NetworkResponse(404, "Not Found", {})
        mock_provider.http_request.return_value = mock_response
        mock_get_provider.return_value = mock_provider

        url = StringValue("https://example.com/nonexistent")
        result = HTTPModule.http_get(url)

        # Check the result
        assert isinstance(result, HashValue)
        assert result.get('status').value == 404
        assert result.get('body').value == "Not Found"
        assert result.get('success').value == False

    @patch('glang.modules.http_module.get_network_provider')
    def test_download_file_success(self, mock_get_provider):
        """Test successful file download."""
        # Mock the network provider
        mock_provider = Mock()
        mock_provider.download_to_file.return_value = True
        mock_get_provider.return_value = mock_provider

        url = StringValue("https://example.com/file.txt")
        filepath = StringValue("/tmp/downloaded.txt")

        result = HTTPModule.download_file(url, filepath)

        # Check the result
        assert isinstance(result, BooleanValue)
        assert result.value == True

        # Verify the provider was called correctly
        mock_provider.download_to_file.assert_called_once_with("https://example.com/file.txt", "/tmp/downloaded.txt")

    @patch('glang.modules.http_module.get_network_provider')
    def test_download_file_failure(self, mock_get_provider):
        """Test failed file download."""
        # Mock the network provider
        mock_provider = Mock()
        mock_provider.download_to_file.return_value = False
        mock_get_provider.return_value = mock_provider

        url = StringValue("https://example.com/nonexistent.txt")
        filepath = StringValue("/tmp/failed.txt")

        result = HTTPModule.download_file(url, filepath)

        # Check the result
        assert isinstance(result, BooleanValue)
        assert result.value == False

    @patch('glang.modules.http_module.get_network_provider')
    def test_is_network_available(self, mock_get_provider):
        """Test network availability check."""
        # Mock the network provider
        mock_provider = Mock()
        mock_provider.is_available.return_value = True
        mock_get_provider.return_value = mock_provider

        result = HTTPModule.is_network_available()

        # Check the result
        assert isinstance(result, BooleanValue)
        assert result.value == True

    def test_invalid_url_type(self):
        """Test error handling for invalid URL type."""
        with pytest.raises(Exception, match="http_get expects string URL"):
            HTTPModule.http_get(NumberValue(123))

    def test_invalid_headers_type(self):
        """Test error handling for invalid headers type."""
        url = StringValue("https://example.com")
        invalid_headers = StringValue("not a hash")

        with pytest.raises(Exception, match="http_get headers must be a hash"):
            HTTPModule.http_get(url, invalid_headers)

    def test_invalid_data_type(self):
        """Test error handling for invalid POST data type."""
        url = StringValue("https://example.com")
        invalid_data = NumberValue(123)

        with pytest.raises(Exception, match="http_post data must be a string"):
            HTTPModule.http_post(url, invalid_data)

    @patch('glang.modules.http_module.get_network_provider')
    def test_response_headers_conversion(self, mock_get_provider):
        """Test that response headers are properly converted to Glang values."""
        # Mock the network provider
        mock_provider = Mock()
        mock_response = NetworkResponse(200, "OK", {
            "Content-Type": "application/json",
            "Server": "nginx/1.18.0",
            "Content-Length": "42"
        })
        mock_provider.http_request.return_value = mock_response
        mock_get_provider.return_value = mock_provider

        url = StringValue("https://example.com")
        result = HTTPModule.http_get(url)

        # Check that headers are included and properly formatted
        assert result.get('headers') is not None
        headers = result.get('headers')
        assert isinstance(headers, HashValue)

        # Check specific headers
        assert headers.get('Content-Type') is not None
        assert headers.get('Content-Type').value == "application/json"

        assert headers.get('Server') is not None
        assert headers.get('Server').value == "nginx/1.18.0"

    @patch('glang.modules.http_module.get_network_provider')
    def test_put_request(self, mock_get_provider):
        """Test HTTP PUT request."""
        # Mock the network provider
        mock_provider = Mock()
        mock_response = NetworkResponse(200, "Updated", {})
        mock_provider.http_request.return_value = mock_response
        mock_get_provider.return_value = mock_provider

        method = StringValue("PUT")
        url = StringValue("https://api.example.com/users/123")
        data = StringValue('{"name": "Updated Name"}')

        result = HTTPModule.http_request(method, url, data)

        # Check the result
        assert isinstance(result, HashValue)
        assert result.get('status').value == 200
        assert result.get('body').value == "Updated"

        # Verify the provider was called correctly
        mock_provider.http_request.assert_called_once_with("PUT", "https://api.example.com/users/123", '{"name": "Updated Name"}', None)


if __name__ == '__main__':
    pytest.main([__file__])
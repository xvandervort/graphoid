"""
HTTP Module for Glang

Provides HTTP client functionality using the network interface.
This module bridges Python's network capabilities to Glang's HTTP operations.
"""

from typing import Optional, Dict, Any
from ..execution.values import GlangValue, StringValue, BooleanValue, NumberValue
from ..execution.graph_values import HashValue, ListValue
from ..ast.nodes import SourcePosition
from .module_manager import ModuleNamespace
from .network_interface import get_network_provider
from ..execution.errors import RuntimeError


class HTTPModule:
    """HTTP module providing network operations."""

    @staticmethod
    def http_get(url: GlangValue, headers: GlangValue = None,
                position: Optional[SourcePosition] = None) -> GlangValue:
        """Make an HTTP GET request.

        Args:
            url: URL to request (StringValue)
            headers: Optional headers as HashValue
            position: Source position for error reporting

        Returns:
            HashValue with response data
        """
        if not isinstance(url, StringValue):
            raise RuntimeError("http_get expects string URL", position)

        # Convert headers to dict if provided
        header_dict = None
        if headers is not None:
            if isinstance(headers, HashValue):
                header_dict = {}
                # Use proper iteration for HashValue
                for key in headers.keys():
                    value = headers.get(key)
                    if isinstance(value, StringValue):
                        header_dict[key] = value.value
                    else:
                        header_dict[key] = str(value.value if hasattr(value, 'value') else value)
            else:
                raise RuntimeError("http_get headers must be a hash", position)

        # Make the request
        provider = get_network_provider()
        response = provider.http_request("GET", url.value, None, header_dict)

        # Convert to Glang values
        result = response.to_glang_values(position)

        # Add headers as pairs for HashValue
        header_pairs = [(key, StringValue(value, position)) for key, value in response.headers.items()]
        result['headers'] = HashValue(header_pairs, 'string', position)

        # Return as HashValue with all result data
        result_pairs = [(key, value) for key, value in result.items()]
        return HashValue(result_pairs, 'any', position)

    @staticmethod
    def http_post(url: GlangValue, data: GlangValue = None, headers: GlangValue = None,
                 position: Optional[SourcePosition] = None) -> GlangValue:
        """Make an HTTP POST request.

        Args:
            url: URL to request (StringValue)
            data: Optional POST data (StringValue)
            headers: Optional headers as HashValue
            position: Source position for error reporting

        Returns:
            HashValue with response data
        """
        if not isinstance(url, StringValue):
            raise RuntimeError("http_post expects string URL", position)

        # Convert data
        post_data = None
        if data is not None:
            if isinstance(data, StringValue):
                post_data = data.value
            else:
                raise RuntimeError("http_post data must be a string", position)

        # Convert headers to dict if provided
        header_dict = None
        if headers is not None:
            if isinstance(headers, HashValue):
                header_dict = {}
                # Use proper iteration for HashValue
                for key in headers.keys():
                    value = headers.get(key)
                    if isinstance(value, StringValue):
                        header_dict[key] = value.value
                    else:
                        header_dict[key] = str(value.value if hasattr(value, 'value') else value)
            else:
                raise RuntimeError("http_post headers must be a hash", position)

        # Make the request
        provider = get_network_provider()
        response = provider.http_request("POST", url.value, post_data, header_dict)

        # Convert to Glang values
        result = response.to_glang_values(position)

        # Add headers as pairs for HashValue
        header_pairs = [(key, StringValue(value, position)) for key, value in response.headers.items()]
        result['headers'] = HashValue(header_pairs, 'string', position)

        # Return as HashValue with all result data
        result_pairs = [(key, value) for key, value in result.items()]
        return HashValue(result_pairs, 'any', position)

    @staticmethod
    def http_request(method: GlangValue, url: GlangValue, data: GlangValue = None,
                    headers: GlangValue = None, position: Optional[SourcePosition] = None) -> GlangValue:
        """Make a generic HTTP request.

        Args:
            method: HTTP method (StringValue)
            url: URL to request (StringValue)
            data: Optional request data (StringValue)
            headers: Optional headers as HashValue
            position: Source position for error reporting

        Returns:
            HashValue with response data
        """
        if not isinstance(method, StringValue):
            raise RuntimeError("http_request expects string method", position)

        if not isinstance(url, StringValue):
            raise RuntimeError("http_request expects string URL", position)

        # Convert data
        request_data = None
        if data is not None:
            if isinstance(data, StringValue):
                request_data = data.value
            else:
                raise RuntimeError("http_request data must be a string", position)

        # Convert headers to dict if provided
        header_dict = None
        if headers is not None:
            if isinstance(headers, HashValue):
                header_dict = {}
                # Use proper iteration for HashValue
                for key in headers.keys():
                    value = headers.get(key)
                    if isinstance(value, StringValue):
                        header_dict[key] = value.value
                    else:
                        header_dict[key] = str(value.value if hasattr(value, 'value') else value)
            else:
                raise RuntimeError("http_request headers must be a hash", position)

        # Make the request
        provider = get_network_provider()
        response = provider.http_request(method.value.upper(), url.value, request_data, header_dict)

        # Convert to Glang values
        result = response.to_glang_values(position)

        # Add headers as pairs for HashValue
        header_pairs = [(key, StringValue(value, position)) for key, value in response.headers.items()]
        result['headers'] = HashValue(header_pairs, 'string', position)

        # Return as HashValue with all result data
        result_pairs = [(key, value) for key, value in result.items()]
        return HashValue(result_pairs, 'any', position)

    @staticmethod
    def download_file(url: GlangValue, filepath: GlangValue,
                     position: Optional[SourcePosition] = None) -> GlangValue:
        """Download a file from URL to local path.

        Args:
            url: URL to download (StringValue)
            filepath: Local file path (StringValue)
            position: Source position for error reporting

        Returns:
            BooleanValue indicating success
        """
        if not isinstance(url, StringValue):
            raise RuntimeError("download_file expects string URL", position)

        if not isinstance(filepath, StringValue):
            raise RuntimeError("download_file expects string filepath", position)

        # Make the request
        provider = get_network_provider()
        success = provider.download_to_file(url.value, filepath.value)

        return BooleanValue(success, position)

    @staticmethod
    def is_network_available(position: Optional[SourcePosition] = None) -> GlangValue:
        """Check if network functionality is available.

        Returns:
            BooleanValue indicating availability
        """
        provider = get_network_provider()
        return BooleanValue(provider.is_available(), position)


def create_http_module_namespace():
    """Create the HTTP module namespace."""
    from .module_builder import create_module

    return create_module(
        "http",
        functions={
            'get': HTTPModule.http_get,
            'post': HTTPModule.http_post,
            'request': HTTPModule.http_request,
            'download_file': HTTPModule.download_file,
            'is_available': HTTPModule.is_network_available,
        }
    )
"""
Glang Network Module - More native implementation

Uses Glang's own string manipulation capabilities and the network interface
to provide HTTP functionality with better Glang integration.
"""

from typing import Optional, Dict
from ..execution.values import GlangValue, StringValue, BooleanValue, NumberValue, ListValue, HashValue, DataValue
from ..ast.nodes import SourcePosition
from .network_interface import get_network_provider, NetworkResponse
from .filesystem_interface import get_filesystem


class NetworkModule:
    """Network operations using Glang-native string processing where possible."""
    
    @staticmethod
    def http_get(url: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Make HTTP GET request with Glang-native error handling."""
        if not isinstance(url, StringValue):
            raise RuntimeError(
                f"http_get expects string URL, got {url.get_type()}",
                position
            )
        
        provider = get_network_provider()
        if not provider.is_available():
            raise RuntimeError("Network functionality not available", position)
        
        response = provider.http_request("GET", url.value)
        
        # Use Glang's own error handling instead of throwing exceptions
        if response.status_code == 0:  # URL/network error
            raise RuntimeError(f"Network error: {response.body}", position)
        
        if response.status_code >= 400:  # HTTP error
            raise RuntimeError(f"HTTP error {response.status_code}: {response.body}", position)
        
        return StringValue(response.body, position)
    
    @staticmethod
    def http_post(url: GlangValue, data: GlangValue = None, position: Optional[SourcePosition] = None) -> GlangValue:
        """Make HTTP POST request with Glang-native data processing."""
        if not isinstance(url, StringValue):
            raise RuntimeError(
                f"http_post expects string URL, got {url.get_type()}",
                position
            )
        
        # Process data using Glang's own type system
        post_data = None
        if data is not None:
            if isinstance(data, StringValue):
                post_data = data.value
            else:
                # Use Glang's to_display_string() method
                post_data = data.to_display_string()
        
        provider = get_network_provider()
        if not provider.is_available():
            raise RuntimeError("Network functionality not available", position)
        
        response = provider.http_request("POST", url.value, post_data)
        
        # Glang-native error handling
        if response.status_code == 0:
            raise RuntimeError(f"Network error: {response.body}", position)
        
        if response.status_code >= 400:
            raise RuntimeError(f"HTTP error {response.status_code}: {response.body}", position)
        
        return StringValue(response.body, position)
    
    @staticmethod
    def http_request_full(method: GlangValue, url: GlangValue, data: GlangValue = None, 
                         headers: GlangValue = None, position: Optional[SourcePosition] = None) -> GlangValue:
        """Full HTTP request with headers support - returns hash with response details."""
        if not isinstance(method, StringValue):
            raise RuntimeError(f"http_request_full expects string method, got {method.get_type()}", position)
        
        if not isinstance(url, StringValue):
            raise RuntimeError(f"http_request_full expects string URL, got {url.get_type()}", position)
        
        # Process data using Glang methods
        post_data = None
        if data is not None:
            if isinstance(data, StringValue):
                post_data = data.value
            else:
                post_data = data.to_display_string()
        
        # Process headers using Glang hash operations
        request_headers = {}
        if headers is not None:
            if isinstance(headers, HashValue):
                # Convert Glang hash to Python dict using Glang's own methods
                for key, value_node in headers.pairs.items():
                    if isinstance(value_node, DataValue):
                        header_value = value_node.value
                        if isinstance(header_value, StringValue):
                            request_headers[key] = header_value.value
                        else:
                            request_headers[key] = header_value.to_display_string()
            else:
                raise RuntimeError(f"headers must be hash, got {headers.get_type()}", position)
        
        provider = get_network_provider()
        if not provider.is_available():
            raise RuntimeError("Network functionality not available", position)
        
        response = provider.http_request(method.value.upper(), url.value, post_data, request_headers)
        
        # Build response hash using Glang's hash construction
        response_pairs = {}
        response_pairs["status"] = DataValue("status", NumberValue(response.status_code, position), position)
        response_pairs["body"] = DataValue("body", StringValue(response.body, position), position)
        response_pairs["success"] = DataValue("success", BooleanValue(200 <= response.status_code < 300, position), position)
        
        # Add headers as nested hash
        if response.headers:
            header_pairs = {}
            for key, value in response.headers.items():
                header_pairs[key] = DataValue(key, StringValue(value, position), position)
            headers_hash = HashValue(header_pairs, position)
            response_pairs["headers"] = DataValue("headers", headers_hash, position)
        
        return HashValue(response_pairs, position)
    
    @staticmethod
    def download_file(url: GlangValue, filepath: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Download file using Glang's filesystem interface."""
        if not isinstance(url, StringValue):
            raise RuntimeError(f"download_file expects string URL, got {url.get_type()}", position)
        
        if not isinstance(filepath, StringValue):
            raise RuntimeError(f"download_file expects string filepath, got {filepath.get_type()}", position)
        
        # Use Glang's filesystem interface for path operations
        filesystem = get_filesystem()
        path = filepath.value
        
        # Create parent directories using Glang's filesystem interface
        parent_dir = filesystem.get_dirname(path)
        if parent_dir and not filesystem.file_exists(parent_dir):
            filesystem.create_directory(parent_dir, parents=True)
        
        # Download using network provider
        provider = get_network_provider()
        if not provider.is_available():
            raise RuntimeError("Network functionality not available", position)
        
        success = provider.download_to_file(url.value, path)
        return BooleanValue(success, position)
    
    @staticmethod
    def url_parse(url: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Parse URL into components using Glang's string operations."""
        if not isinstance(url, StringValue):
            raise RuntimeError(f"url_parse expects string URL, got {url.get_type()}", position)
        
        url_str = url.value
        
        # Use Glang's string methods to parse URL
        # This is more Glang-native than using Python's urllib.parse
        
        # Check for protocol
        if url_str.find("://") == -1:
            raise RuntimeError("Invalid URL: missing protocol", position)
        
        # Split protocol and rest
        protocol_parts = StringValue(url_str, position).split(StringValue("://", position))
        if not hasattr(protocol_parts, 'elements') or len(protocol_parts.elements) != 2:
            raise RuntimeError("Invalid URL format", position)
        
        protocol = protocol_parts.elements[0]
        rest = protocol_parts.elements[1]
        
        # Split host and path
        path_parts = rest.split(StringValue("/", position))
        host = path_parts.elements[0]
        
        # Reconstruct path
        if len(path_parts.elements) > 1:
            path_elements = path_parts.elements[1:]
            path = StringValue("/", position).join(ListValue(path_elements, "string", position))
            path = StringValue("/" + path.value, position)  # Add leading slash
        else:
            path = StringValue("/", position)
        
        # Build result hash using Glang construction
        result_pairs = {}
        result_pairs["protocol"] = DataValue("protocol", protocol, position)
        result_pairs["host"] = DataValue("host", host, position)
        result_pairs["path"] = DataValue("path", path, position)
        result_pairs["url"] = DataValue("url", url, position)
        
        return HashValue(result_pairs, position)
    
    @staticmethod
    def url_encode(data: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """URL encode data using Glang's string operations where possible."""
        if not isinstance(data, StringValue):
            raise RuntimeError(f"url_encode expects string data, got {data.get_type()}", position)
        
        # Basic URL encoding using Glang's string replace method
        # This is more limited than Python's urllib.parse.quote but uses Glang methods
        text = data.value
        
        # Replace common characters that need encoding
        replacements = [
            (" ", "%20"),
            ("!", "%21"),
            ("\"", "%22"),
            ("#", "%23"),
            ("$", "%24"),
            ("%", "%25"),
            ("&", "%26"),
            ("'", "%27"),
            ("(", "%28"),
            (")", "%29"),
            ("*", "%2A"),
            ("+", "%2B"),
            (",", "%2C"),
            ("/", "%2F"),
            (":", "%3A"),
            (";", "%3B"),
            ("<", "%3C"),
            ("=", "%3D"),
            (">", "%3E"),
            ("?", "%3F"),
            ("@", "%40"),
        ]
        
        result_str = StringValue(text, position)
        for old, new in replacements:
            result_str = StringValue(
                result_str.value.replace(old, new), 
                position
            )
        
        return result_str


def create_network_module() -> 'ModuleNamespace':
    """Create the network module namespace with all functions."""
    from ..modules.module_manager import ModuleNamespace
    from ..execution.function_value import BuiltinFunctionValue
    
    namespace = ModuleNamespace("network")
    
    # Network functions organized by category
    network_functions = {
        # Basic HTTP operations
        'http_get': NetworkModule.http_get,
        'http_post': NetworkModule.http_post,
        'http_request': NetworkModule.http_request_full,
        
        # File operations
        'download_file': NetworkModule.download_file,
        
        # URL utilities (more Glang-native)
        'url_parse': NetworkModule.url_parse,
        'url_encode': NetworkModule.url_encode,
    }
    
    # Wrap functions as callable values
    for name, func in network_functions.items():
        wrapped_func = BuiltinFunctionValue(name, func)
        namespace.define(name, wrapped_func)
    
    return namespace
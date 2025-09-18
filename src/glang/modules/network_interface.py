"""
Glang Network Interface

Provides an abstract interface for network operations, decoupling
Glang from Python's specific network implementation. This allows
for different backends and maintains Glang's language independence.
"""

from abc import ABC, abstractmethod
from typing import Dict, Optional, List, Tuple
from ..execution.values import GlangValue, StringValue, BooleanValue, NumberValue
from ..execution.graph_values import ListValue
from ..ast.nodes import SourcePosition


class NetworkResponse:
    """Represents a network response in Glang-native format."""
    
    def __init__(self, status_code: int, body: str, headers: Dict[str, str]):
        self.status_code = status_code
        self.body = body
        self.headers = headers
    
    def to_glang_values(self, position: Optional[SourcePosition] = None) -> Dict[str, GlangValue]:
        """Convert response to Glang values."""
        return {
            'status': NumberValue(self.status_code, position),
            'body': StringValue(self.body, position),
            'success': BooleanValue(200 <= self.status_code < 300, position)
        }


class NetworkInterface(ABC):
    """Abstract interface for network operations using Glang semantics."""
    
    @abstractmethod
    def http_request(self, method: str, url: str, data: Optional[str] = None, 
                    headers: Optional[Dict[str, str]] = None) -> NetworkResponse:
        """Make an HTTP request and return a NetworkResponse."""
        pass
    
    @abstractmethod
    def download_to_file(self, url: str, filepath: str) -> bool:
        """Download content from URL directly to a file."""
        pass
    
    @abstractmethod
    def is_available(self) -> bool:
        """Check if network functionality is available."""
        pass


class PythonNetworkProvider(NetworkInterface):
    """Python-based implementation of network interface."""
    
    def __init__(self):
        self.available = True
        try:
            import urllib.request
            import urllib.error
            self._urllib_request = urllib.request
            self._urllib_error = urllib.error
        except ImportError:
            self.available = False
    
    def is_available(self) -> bool:
        return self.available
    
    def http_request(self, method: str, url: str, data: Optional[str] = None, 
                    headers: Optional[Dict[str, str]] = None) -> NetworkResponse:
        """Make HTTP request using urllib."""
        if not self.available:
            raise RuntimeError("Network functionality not available")
        
        try:
            # Prepare request
            post_data = data.encode('utf-8') if data else None
            req = self._urllib_request.Request(url, data=post_data, method=method)
            
            # Add headers
            if headers:
                for key, value in headers.items():
                    req.add_header(key, value)
            
            # Add default content type for POST with data
            if method == 'POST' and post_data and not (headers and 'Content-Type' in headers):
                req.add_header('Content-Type', 'application/x-www-form-urlencoded')
            
            # Make request
            with self._urllib_request.urlopen(req) as response:
                body = response.read().decode('utf-8')
                status = response.getcode()
                
                # Convert headers to dict
                response_headers = {}
                if hasattr(response, 'headers'):
                    response_headers = dict(response.headers.items())
                
                return NetworkResponse(status, body, response_headers)
                
        except self._urllib_error.HTTPError as e:
            # Return error response instead of throwing
            error_body = f"HTTP Error {e.code}: {e.reason}"
            return NetworkResponse(e.code, error_body, {})
            
        except self._urllib_error.URLError as e:
            # Return error response for URL errors
            error_body = f"URL Error: {e.reason}"
            return NetworkResponse(0, error_body, {})  # Use 0 for URL errors
            
        except Exception as e:
            # Return error response for other errors
            error_body = f"Network Error: {str(e)}"
            return NetworkResponse(0, error_body, {})
    
    def download_to_file(self, url: str, filepath: str) -> bool:
        """Download file using urllib."""
        if not self.available:
            return False
        
        try:
            self._urllib_request.urlretrieve(url, filepath)
            return True
        except Exception:
            return False


# Global network provider instance
_network_provider: Optional[NetworkInterface] = None


def get_network_provider() -> NetworkInterface:
    """Get the current network provider, initializing if needed."""
    global _network_provider
    if _network_provider is None:
        _network_provider = PythonNetworkProvider()
    return _network_provider


def set_network_provider(provider: NetworkInterface) -> None:
    """Set a custom network provider (for testing or alternative implementations)."""
    global _network_provider
    _network_provider = provider
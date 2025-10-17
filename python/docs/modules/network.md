# Network Module Documentation

The `network` module provides comprehensive networking capabilities for Glang, including HTTP client functionality, URL manipulation, and network utilities. This module enables web scraping, API integration, file downloads, and other network-based operations.

## Table of Contents

- [Quick Start](#quick-start)
- [HTTP Client Functions](#http-client-functions)
- [URL Utilities](#url-utilities)
- [Network Status](#network-status)
- [Response Objects](#response-objects)
- [Error Handling](#error-handling)
- [Examples](#examples)
- [Best Practices](#best-practices)

## Quick Start

```glang
import "network" as net

# Simple GET request
response = net.http_get("https://api.github.com/users/octocat")
if response["success"] {
    print("Status: " + response["status"].to_string())
    print("Body: " + response["body"])
}

# URL parsing
url_parts = net.parse_url("https://example.com/path?query=value")
print("Host: " + url_parts["host"].value)
print("Path: " + url_parts["path"].value)
```

## HTTP Client Functions

### `http_get(url, headers = {})`

Makes an HTTP GET request to the specified URL.

**Parameters:**
- `url` (string): The URL to request
- `headers` (map, optional): HTTP headers to include

**Returns:** Response object (see [Response Objects](#response-objects))

```glang
# Simple GET
response = net.http_get("https://httpbin.org/json")

# GET with custom headers
headers = {
    "User-Agent": "MyApp/1.0",
    "Accept": "application/json"
}
response = net.http_get("https://api.example.com/data", headers)
```

### `http_post(url, data = "", headers = {})`

Makes an HTTP POST request with data.

**Parameters:**
- `url` (string): The URL to request
- `data` (string, optional): POST data to send
- `headers` (map, optional): HTTP headers to include

**Returns:** Response object

```glang
# POST with form data
post_data = "name=Alice&email=alice@example.com"
response = net.http_post("https://httpbin.org/post", post_data)

# POST with JSON
headers = { "Content-Type": "application/json" }
json_data = '{"name": "Bob", "age": 30}'
response = net.http_post("https://api.example.com/users", json_data, headers)
```

### `http_request(method, url, data = "", headers = {})`

Makes an HTTP request with a custom method.

**Parameters:**
- `method` (string): HTTP method (GET, POST, PUT, DELETE, PATCH, etc.)
- `url` (string): The URL to request
- `data` (string, optional): Request body data
- `headers` (map, optional): HTTP headers to include

**Returns:** Response object

```glang
# PUT request
response = net.http_request("PUT", "https://httpbin.org/put", "updated_data")

# DELETE request
response = net.http_request("DELETE", "https://api.example.com/users/123")

# PATCH request with JSON
headers = { "Content-Type": "application/json" }
patch_data = '{"email": "newemail@example.com"}'
response = net.http_request("PATCH", "https://api.example.com/users/123", patch_data, headers)
```

### `download_file(url, filename, headers = {})`

Downloads a file from a URL and saves it locally.

**Parameters:**
- `url` (string): The URL of the file to download
- `filename` (string): Local filename to save to
- `headers` (map, optional): HTTP headers to include

**Returns:** Response object with download status

```glang
# Download an image
response = net.download_file("https://httpbin.org/image/png", "test_image.png")
if response["success"] {
    print("Downloaded successfully!")
} else {
    print("Download failed: " + response["status"].to_string())
}

# Download with authentication headers
headers = { "Authorization": "Bearer your_token_here" }
response = net.download_file("https://api.example.com/files/doc.pdf", "document.pdf", headers)
```

## URL Utilities

### `parse_url(url)`

Parses a URL into its components.

**Parameters:**
- `url` (string): The URL to parse

**Returns:** Map with URL components

```glang
parts = net.parse_url("https://user:pass@example.com:8080/path/to/file?query=value#section")

# Available components:
print("Protocol: " + parts["protocol"].value)  # "https"
print("Host: " + parts["host"].value)          # "example.com"
print("Port: " + parts["port"].value)          # "8080"
print("Path: " + parts["path"].value)          # "/path/to/file"
print("Query: " + parts["query"].value)        # "query=value"
print("Fragment: " + parts["fragment"].value)  # "section"
print("Username: " + parts["username"].value)  # "user"
print("Password: " + parts["password"].value)  # "pass"
```

### `encode_url(text)`

Encodes text for safe use in URLs (percent encoding).

**Parameters:**
- `text` (string): Text to encode

**Returns:** URL-encoded string

```glang
encoded = net.encode_url("hello world & special=chars")
print(encoded)  # "hello%20world%20%26%20special%3Dchars"

# Use in query parameters
base_url = "https://api.example.com/search?q="
query = "coffee & tea"
full_url = base_url + net.encode_url(query)
```

### `decode_url(encoded_text)`

Decodes URL-encoded text.

**Parameters:**
- `encoded_text` (string): URL-encoded text to decode

**Returns:** Decoded string

```glang
decoded = net.decode_url("hello%20world%26test%3Dvalue")
print(decoded)  # "hello world&test=value"
```

### `extract_domain(url)`

Extracts the domain name from a URL.

**Parameters:**
- `url` (string): The URL to extract domain from

**Returns:** Domain name (string)

```glang
domain = net.extract_domain("https://www.example.com:8080/path")
print(domain)  # "example.com" (strips www and port)

domain = net.extract_domain("http://subdomain.site.org/page")
print(domain)  # "subdomain.site.org"
```

### `is_valid_url(url)`

Checks if a string is a valid URL.

**Parameters:**
- `url` (string): The URL to validate

**Returns:** Boolean (true if valid, false otherwise)

```glang
if net.is_valid_url("https://example.com/path") {
    print("Valid URL")
}

if net.is_valid_url("not-a-valid-url") == false {
    print("Invalid URL")
}
```

## Network Status

### `is_network_available()`

Checks if network connectivity is available.

**Returns:** Boolean (true if network is available, false otherwise)

```glang
if net.is_network_available() {
    print("Network is available")
    response = net.http_get("https://api.example.com/status")
} else {
    print("No network connection")
}
```

## Response Objects

All HTTP functions return a response object (map) with the following structure:

```glang
response = {
    "success": true,        # Boolean: true if request succeeded, false if failed
    "status": 200,          # Number: HTTP status code (200, 404, 500, etc.)
    "body": "...",          # String: Response body content
    "headers": {            # Map: Response headers
        "content-type": "application/json",
        "server": "nginx/1.18.0",
        "content-length": "1234"
        # ... other headers
    }
}
```

### Response Fields

- **`success`**: Boolean indicating if the request completed successfully
- **`status`**: HTTP status code (200 = OK, 404 = Not Found, 500 = Server Error, etc.)
- **`body`**: The response content as a string
- **`headers`**: Map of response headers (lowercase keys)

### Working with Responses

```glang
response = net.http_get("https://httpbin.org/json")

# Check if request succeeded
if response["success"] {
    print("Request successful!")

    # Check specific status codes
    status = response["status"]
    if status == 200 {
        print("OK: " + response["body"])
    } else if status == 404 {
        print("Not found")
    }

    # Access headers
    if response["headers"].has_key("content-type") {
        content_type = response["headers"]["content-type"]
        print("Content type: " + content_type.to_string())
    }
} else {
    print("Request failed with status: " + response["status"].to_string())
}
```

## Error Handling

The network module provides robust error handling:

### Connection Errors

```glang
# Network unavailable
response = net.http_get("https://nonexistent-domain-12345.com")
if response["success"] == false {
    print("Connection failed: " + response["status"].to_string())
    # status will be 0 for connection errors
}
```

### HTTP Errors

```glang
# Server errors (4xx, 5xx status codes)
response = net.http_get("https://httpbin.org/status/404")
if response["success"] == false {
    status = response["status"]
    if status >= 400 and status < 500 {
        print("Client error: " + status.to_string())
    } else if status >= 500 {
        print("Server error: " + status.to_string())
    }
}
```

### Invalid URLs

```glang
# Always validate URLs before making requests
url = "user-input-url"
if net.is_valid_url(url) {
    response = net.http_get(url)
} else {
    print("Invalid URL provided")
}
```

## Examples

### Web Scraping

```glang
import "network" as net

# Fetch a web page
headers = {
    "User-Agent": "Mozilla/5.0 (compatible; GlangBot/1.0)"
}
response = net.http_get("https://example.com", headers)

if response["success"] {
    html = response["body"]
    print("Page title found in: " + html.length().to_string() + " characters")

    # Extract specific data (you'd use html_parser module for real parsing)
    if html.contains("<title>") {
        print("Page has a title tag")
    }
}
```

### API Integration

```glang
import "network" as net
import "json"

# REST API example
func get_user_data(user_id) {
    url = "https://jsonplaceholder.typicode.com/users/" + user_id.to_string()
    headers = {
        "Accept": "application/json",
        "User-Agent": "MyApp/1.0"
    }

    response = net.http_get(url, headers)

    if response["success"] and response["status"] == 200 {
        if json.is_valid(response["body"]) {
            return json.decode(response["body"])
        }
    }

    return none
}

# Usage
user = get_user_data(1)
if user != none {
    print("User name: " + user["name"].to_string())
}
```

### File Upload Simulation

```glang
import "network" as net

# Simulate form-based file upload
func upload_data(endpoint, data, filename) {
    # Create multipart-like data (simplified)
    boundary = "----GlangFormBoundary123456"
    post_data = "--" + boundary + "\n" +
                "Content-Disposition: form-data; name=\"file\"; filename=\"" + filename + "\"\n" +
                "Content-Type: text/plain\n\n" +
                data + "\n" +
                "--" + boundary + "--"

    headers = {
        "Content-Type": "multipart/form-data; boundary=" + boundary,
        "User-Agent": "GlangUploader/1.0"
    }

    return net.http_post(endpoint, post_data, headers)
}

# Usage
response = upload_data("https://httpbin.org/post", "file content here", "data.txt")
```

### Cryptocurrency Price Tracker

```glang
import "network" as net
import "json"

func get_bitcoin_price() {
    # Try API first
    api_url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd"
    response = net.http_get(api_url)

    if response["success"] and json.is_valid(response["body"]) {
        data = json.decode(response["body"])
        if data.has_key("bitcoin") and data["bitcoin"].has_key("usd") {
            return data["bitcoin"]["usd"]
        }
    }

    # Fallback to web scraping (simplified)
    response = net.http_get("https://coinmarketcap.com/currencies/bitcoin/")
    if response["success"] {
        # Parse HTML for price (would use html_parser module in practice)
        html = response["body"]
        if html.contains("$") {
            print("Found price data in HTML")
        }
    }

    return none
}

price = get_bitcoin_price()
if price != none {
    print("Bitcoin price: $" + price.to_string())
}
```

## Best Practices

### 1. Always Check Network Availability

```glang
if net.is_network_available() == false {
    print("No network connection available")
    return
}
```

### 2. Validate URLs Before Requests

```glang
if net.is_valid_url(user_url) == false {
    print("Invalid URL provided")
    return
}
```

### 3. Handle All Response Cases

```glang
response = net.http_get(url)

if response["success"] {
    status = response["status"]
    if status == 200 {
        # Success case
        process_data(response["body"])
    } else if status == 404 {
        print("Resource not found")
    } else if status >= 500 {
        print("Server error, try again later")
    } else {
        print("Unexpected status: " + status.to_string())
    }
} else {
    print("Network error: " + response["status"].to_string())
}
```

### 4. Use Appropriate Headers

```glang
# For APIs
api_headers = {
    "Accept": "application/json",
    "User-Agent": "YourApp/1.0",
    "Authorization": "Bearer your_token"
}

# For web scraping
scraping_headers = {
    "User-Agent": "Mozilla/5.0 (compatible; YourBot/1.0)",
    "Accept": "text/html,application/xhtml+xml",
    "Accept-Language": "en-US,en;q=0.5"
}
```

### 5. Implement Retry Logic

```glang
func reliable_request(url, max_retries) {
    retries = 0
    while retries < max_retries {
        response = net.http_get(url)

        if response["success"] {
            return response
        }

        retries = retries + 1
        print("Retry " + retries.to_string() + "/" + max_retries.to_string())
    }

    return response  # Return last failed attempt
}
```

### 6. Rate Limiting

```glang
import "time"

func respectful_requests(urls) {
    for url in urls {
        response = net.http_get(url)
        process_response(response)

        # Wait between requests
        time.sleep(1)  # 1 second delay
    }
}
```

### 7. Clean URLs for Logging

```glang
func log_request(url) {
    # Extract domain for logging (removes sensitive query params)
    domain = net.extract_domain(url)
    print("Making request to: " + domain)
}
```

## Security Notes

1. **Never log sensitive data**: Avoid logging complete URLs that may contain API keys or credentials
2. **Validate all user input**: Always validate URLs and data before making requests
3. **Use HTTPS**: Prefer HTTPS URLs for secure communication
4. **Handle credentials safely**: Don't hardcode API keys; use environment variables or config files
5. **Respect robots.txt**: When web scraping, check the site's robots.txt file
6. **Rate limiting**: Don't overwhelm servers with too many requests

## Module Integration

The network module works well with other Glang modules:

- **JSON module**: For parsing API responses
- **IO module**: For saving downloaded files
- **Time module**: For timestamps and rate limiting
- **HTML parser module**: For web scraping (when available)

```glang
import "network" as net
import "json"
import "io"
import "time"

# Complete example: Fetch data, parse JSON, save to file
response = net.http_get("https://api.github.com/users/octocat")
if response["success"] and json.is_valid(response["body"]) {
    data = json.decode(response["body"])
    pretty_json = json.encode(data, true)  # Pretty format

    timestamp = time.now().to_string()
    filename = "github_user_" + timestamp + ".json"

    io.write_file(filename, pretty_json)
    print("Data saved to: " + filename)
}
```

The network module provides everything you need for HTTP-based networking in Glang, from simple API calls to complex web scraping applications.
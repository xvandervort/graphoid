# HTTP Module API Reference

The `http` module provides both HTTP/HTTPS client and server capabilities. The client supports GET and POST requests with full TLS 1.3 (implemented in pure Graphoid). The server provides TCP-based HTTP request handling with routing, JSON responses, and static file serving.

## Import

```graphoid
import "http"
```

---

## Client Functions

### `http.get(url)`

Perform an HTTP or HTTPS GET request.

**Parameters:**
- `url` (string) — Full URL including protocol

**Returns:** Map with `status`, `headers`, and `body` keys

```graphoid
response = http.get("https://example.com/")
print(response["status"])   # 200
print(response["body"])     # HTML content
```

### `http.post(url, body, content_type)`

Perform an HTTP or HTTPS POST request.

**Parameters:**
- `url` (string) — Full URL including protocol
- `body` (string) — Request body content
- `content_type` (string) — MIME type for Content-Type header

**Returns:** Map with `status`, `headers`, and `body` keys

```graphoid
response = http.post("https://api.example.com/data", '{"key":"value"}', "application/json")
print(response["status"])
```

### `http.parse_url(url)`

Parse a URL into its components.

**Parameters:**
- `url` (string) — URL to parse

**Returns:** Map with `host`, `port`, `path`, and `is_https` keys

```graphoid
parts = http.parse_url("https://example.com:8443/api/data")
print(parts["host"])       # "example.com"
print(parts["port"])       # 8443
print(parts["path"])       # "/api/data"
print(parts["is_https"])   # true
```

### `http.parse_response(response_text)`

Parse a raw HTTP response string into structured data.

**Parameters:**
- `response_text` (string) — Raw HTTP response

**Returns:** Map with `status`, `headers`, and `body` keys

---

## Server Functions

### `http.create_server(host, port)`

Create a server configuration map.

**Parameters:**
- `host` (string) — Bind address (e.g., `"127.0.0.1"` or `"0.0.0.0"`)
- `port` (num) — Port number to listen on

**Returns:** Server configuration map

```graphoid
server = http.create_server("127.0.0.1", 8080)
```

### `http.add_route(server, method, path, handler)`

Register a route handler on the server. Because maps are value types in Graphoid, this returns a new server map — you must reassign it.

**Parameters:**
- `server` (map) — Server configuration from `create_server`
- `method` (string) — HTTP method (e.g., `"GET"`, `"POST"`)
- `path` (string) — URL path to match (e.g., `"/"`, `"/api/status"`)
- `handler` (function) — Handler function that takes a request map and returns a response string

**Returns:** Updated server map with the route registered

```graphoid
server = http.add_route(server, "GET", "/", handle_home)
server = http.add_route(server, "GET", "/api/status", handle_status)
server = http.add_route(server, "POST", "/data", handle_post)
```

**Important:** You must reassign the result back to `server`. Simply calling `http.add_route(server, ...)` without capturing the return value will not register the route.

### `http.serve(server)`

Start the server and begin accepting connections. This function blocks and runs the server loop indefinitely until the process is interrupted (Ctrl+C).

**Parameters:**
- `server` (map) — Server configuration with routes registered

For each incoming connection, the server:
1. Accepts the TCP connection
2. Reads the raw HTTP request
3. Parses it with `parse_request`
4. Looks up a matching route handler
5. Calls the handler (or returns 404)
6. Sends the response and closes the connection

```graphoid
http.serve(server)  # Blocks forever, serving requests
```

### `http.response(status_code, content_type, body)`

Build a complete HTTP response string with headers.

**Parameters:**
- `status_code` (num) — HTTP status code (e.g., `200`, `404`, `500`)
- `content_type` (string) — MIME type for Content-Type header
- `body` (string) — Response body content

**Returns:** Complete HTTP response string including status line, headers, and body

```graphoid
resp = http.response(200, "text/html", "<h1>Hello</h1>")
resp = http.response(404, "text/plain", "Not Found")
resp = http.response(201, "application/json", '{"created": true}')
```

### `http.json_response(data)`

Build an HTTP response with JSON content. Automatically serializes the data to JSON and sets the content type to `application/json`.

**Parameters:**
- `data` (map or list) — Data to serialize as JSON

**Returns:** Complete HTTP response string with JSON body

```graphoid
resp = http.json_response({"status": "ok", "count": 42})
```

### `http.file_response(file_path)`

Serve a static file as an HTTP response. Automatically detects the content type from the file extension. Returns a 404 response if the file doesn't exist.

**Parameters:**
- `file_path` (string) — Path to the file to serve

**Returns:** Complete HTTP response string with file contents

```graphoid
resp = http.file_response("public/index.html")
resp = http.file_response("assets/style.css")
```

### `http.parse_request(raw_text)`

Parse a raw HTTP request string into structured data.

**Parameters:**
- `raw_text` (string) — Raw HTTP request text

**Returns:** Map with these keys:
- `method` (string) — HTTP method (`"GET"`, `"POST"`, etc.)
- `path` (string) — Request path (e.g., `"/api/data"`)
- `version` (string) — HTTP version (e.g., `"HTTP/1.1"`)
- `headers` (map) — Request headers (keys lowercased, values trimmed)
- `body` (string) — Request body (empty string if none)

```graphoid
req = http.parse_request("GET /api HTTP/1.1\r\nHost: localhost\r\n\r\n")
print(req["method"])              # "GET"
print(req["path"])                # "/api"
print(req["headers"]["host"])     # "localhost"
```

### `http.status_text(code)`

Get the standard reason phrase for an HTTP status code.

**Parameters:**
- `code` (num) — HTTP status code

**Returns:** Reason phrase string

```graphoid
http.status_text(200)   # "OK"
http.status_text(404)   # "Not Found"
http.status_text(500)   # "Internal Server Error"
http.status_text(201)   # "Created"
http.status_text(301)   # "Moved Permanently"
http.status_text(400)   # "Bad Request"
http.status_text(403)   # "Forbidden"
```

### `http.content_type_for(file_path)`

Detect the MIME type from a file path's extension.

**Parameters:**
- `file_path` (string) — File path or name

**Returns:** MIME type string

```graphoid
http.content_type_for("index.html")   # "text/html"
http.content_type_for("data.json")    # "application/json"
http.content_type_for("style.css")    # "text/css"
http.content_type_for("app.js")       # "application/javascript"
http.content_type_for("image.png")    # "image/png"
http.content_type_for("icon.svg")     # "image/svg+xml"
http.content_type_for("readme.txt")   # "text/plain"
http.content_type_for("unknown.xyz")  # "application/octet-stream"
```

---

## Complete Server Example

```graphoid
import "http"

# Define route handlers
fn handle_home(req) {
    html = "<html><body>"
    html = html + "<h1>Welcome to Graphoid</h1>"
    html = html + "<p>Everything is a graph!</p>"
    html = html + "</body></html>"
    return http.response(200, "text/html", html)
}

fn handle_status(req) {
    data = {
        "status": "running",
        "language": "Graphoid"
    }
    return http.json_response(data)
}

fn handle_about(req) {
    return http.response(200, "text/plain", "Graphoid web server")
}

# Create server and register routes
server = http.create_server("127.0.0.1", 8080)
server = http.add_route(server, "GET", "/", handle_home)
server = http.add_route(server, "GET", "/api/status", handle_status)
server = http.add_route(server, "GET", "/about", handle_about)

print("Server running on http://localhost:8080/")
http.serve(server)
```

Test with:
```bash
curl http://localhost:8080/
curl http://localhost:8080/api/status
curl http://localhost:8080/about
```

---

## Handler Functions

Route handler functions receive a parsed request map and must return a complete HTTP response string (use `http.response()`, `http.json_response()`, or `http.file_response()` to build one).

```graphoid
fn my_handler(req) {
    # req["method"]  — "GET", "POST", etc.
    # req["path"]    — "/api/data"
    # req["headers"] — {"host": "localhost", "content-type": "..."}
    # req["body"]    — request body string

    return http.response(200, "text/plain", "OK")
}
```

Handlers must be named functions. Pass the function name (without parentheses) to `add_route`:

```graphoid
server = http.add_route(server, "GET", "/", my_handler)
```

---

## Design Notes

- **Maps are value types** — `add_route` returns a new server map; always reassign the result
- **Blocking server** — `serve()` runs a sequential accept loop; each request is handled before accepting the next
- **Minimal Rust** — Only TCP primitives (`net.bind`, `net.accept`) are in Rust; all HTTP logic is pure Graphoid
- **Unmatched routes** return a 404 response automatically

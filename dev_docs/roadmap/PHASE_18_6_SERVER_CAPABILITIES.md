# Phase 18.6: Server Capabilities

**Duration**: 3-5 days
**Priority**: **Critical** (Unblocks interactive simulations)
**Dependencies**: None (Phases 11-12 complete)
**Status**: Planned

---

## Goal

Enable Graphoid to act as a **TCP/HTTP Server**, allowing it to serve web applications and API endpoints. This bridges the gap between the current CLI-only environment and full web interactivity, without waiting for WASM (Phase 30).

---

## The Gap

The current `net` module (Phase 11) is **Client-Only**.
-   ✅ `net.connect()` (TCP Client)
-   ❌ `net.bind()` (TCP Server)
-   ❌ `net.listen()`
-   ❌ `net.accept()`

Graphoid cannot currently listen for incoming connections, making it impossible to build interactive web-based tools where Graphoid runs the backend logic.

---

## Implementation Plan

### 1. Native `net` Module Extensions (Rust)

Add the following primitives to `src/stdlib/net.rs`:

```graphoid
# Create a listener on a port
listener = net.bind("127.0.0.1", 8080)

# Accept a connection (blocking)
# Returns a socket_id similar to net.connect()
socket = net.accept(listener)

# Send/Recv work exactly as they do for client sockets
msg = net.recv(socket, 1024)
net.send(socket, "HTTP/1.1 200 OK...")
```

### 2. Pure Graphoid `http.Server` (Stdlib)

Extend `stdlib/http.gr` with a high-level Server class:

```graphoid
# stdlib/http.gr

class Server {
    _listener: none
    _routes: {}

    fn new(port) {
        # ... bind logic ...
    }

    fn handle(path, handler) {
        _routes[path] = handler
    }

    fn listen() {
        while true {
            client = net.accept(_listener)
            request_text = net.recv(client, 4096)
            request = parse_request(request_text)
            
            handler = _routes[request.path]
            response = handler.call(request)
            
            net.send(client, response.to_string())
            net.close(client)
        }
    }
}
```

### 3. Usage Example (The Target)

```graphoid
import "http"
import "json"

server = http.Server.new(8080)

# Serve static UI
server.handle("/", fn(req) {
    return http.Response.file("public/index.html")
})

# API Endpoint for Simulation
server.handle("/api/tick", fn(req) {
    # Run one simulation step
    sim.tick()
    return http.Response.json(sim.state())
})

print("Simulation running at http://localhost:8080")
server.listen()
```

---

## Impact

1.  **Immediate Interactivity**: Allows building web UIs *now* instead of waiting 6 months for WASM.
2.  **Simulation Dashboard**: The "Dysregulation" project can become an interactive web app with sliders and real-time charts (via Chart.js).
3.  **API Development**: Proves Graphoid's viability for backend service development.

---

## Success Criteria

- [ ] `net.bind` and `net.accept` implemented in Rust
- [ ] `http.Server` class implemented in `stdlib/http.gr`
- [ ] Ability to serve static HTML files
- [ ] Ability to handle JSON API requests
- [ ] Working example: `samples/06-projects/web_server/simple.gr`

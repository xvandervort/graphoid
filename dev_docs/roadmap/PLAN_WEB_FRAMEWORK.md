# Plan: "GraphWeb" - A Graphoid Web Framework

**Target**: Post-Phase 18.6
**Inspiration**: Sinatra (Ruby), Express (Node.js), but backed by Graph Traversal.

## Goal

Create a lightweight, idiomatic web framework for Graphoid that leverages the "Everything is a Graph" philosophy for high-performance routing and clean architecture.

## Architecture

### 1. The Router as a Graph

Instead of iterating through a list of Regex routes (O(n)), GraphWeb will use a **Graph-based Trie** for O(depth) routing.

```graphoid
# Conceptual Routing Graph
Root
 ├── "api"
 │    └── "v1"
 │         ├── "users"
 │         │    ├── [GET] -> UserListHandler
 │         │    └── [POST] -> UserCreateHandler
 │         └── "posts"
 │              └── [GET] -> PostListHandler
└── "static"
      └── [GET] -> StaticFileHandler
```

**Implementation**:
-   Nodes represent path segments.
-   Edges represent transitions (static strings or parameters like `:id`).
-   Request matching is simply walking the graph from the root.

### 2. Middleware as a Chain

Middleware functions (logging, auth, body parsing) will be organized as a behavior chain or a linked list of closures.

```graphoid
app.use(Logger)
app.use(BodyParser)
app.use(AuthCheck)
```

### 3. API Design (Sinatra-style)

```graphoid
import "web"

app = web.App.new()

# Middleware
app.use(web.Logger)
app.static("/public")

# Simple Routes
app.get("/", fn(ctx) {
    return ctx.html("<h1>Hello</h1>")
})

# Parameterized Routes
app.get("/users/:id", fn(ctx) {
    user_id = ctx.params["id"]
    return ctx.json({ id: user_id, name: "Alice" })
})

# POST with JSON body
app.post("/api/data", fn(ctx) {
    data = ctx.body()
    # ... save to db ...
    return ctx.status(201).json({ result: "saved" })
})

app.listen(8080)
```

## Components

1.  **`web.Server`**: Wraps the raw `http.Server` from Phase 18.6.
2.  **`web.Context`**: Wraps `Request` and `Response` into a single object for the handler.
    -   Methods: `json()`, `html()`, `status()`, `redirect()`.
3.  **`web.Router`**: The graph-based routing logic.
4.  **`web.Middleware`**: Standard components (Static files, CORS, Logger).

## Implementation Stages

1.  **Stage 1 (Post-18.6)**:
    -   Basic `App` class.
    -   Simple dictionary-based routing (exact match only).
    -   Static file serving.
    -   *Enables: Interactive Dysregulation Sim.*

2.  **Stage 2 (Optimization)**:
    -   Graph-based Trie Router (parameter support `/users/:id`).
    -   Middleware chain.

3.  **Stage 3 (Post-Phase 22)**:
    -   Database integration helpers.
    -   Session storage (Redis/DB).

## Why Graphoid is Great for This

Routing is fundamentally a graph traversal problem. Most frameworks simulate this with lists or regexes. Graphoid can implement the router as a **literal graph**, making inspection, visualization, and debugging of the routing table trivial. You could literally "print" your routing tree to see the structure of your API.

# Phase 22: Database Connectivity

**Duration**: 7-10 days
**Priority**: High
**Dependencies**: Phase 20 (FFI), Phase 21 (Package Manager)
**Status**: Blocked on Phase 20, 21

---

## Goal

Provide native database connectivity for PostgreSQL, SQLite, MySQL, and Redis, enabling Graphoid programs to persist and query data from production databases.

---

## Why This Phase Matters

1. **Real Applications Need Databases** - Can't build production apps without persistence
2. **Graph-Database Synergy** - Store graph data in databases, query with Graphoid
3. **Enterprise Adoption** - Database connectivity is table stakes

---

## Universe Graph Integration

Database connections are **Bridge Nodes** in the universe graph (consistent with the FFI model from Phase 20). They live in `universe:connections` as defined in Phase 18.

### Connection as Bridge Node

```
Universe Graph                        Foreign Realm
┌──────────────────────────────────┐  ┌──────────────────────────┐
│                                  │  │                          │
│  universe:connections            │  │  PostgreSQL (libpq)      │
│     │                            │  │  ┌──────────────────┐    │
│     └── pg_main ─────────────────┼──┼─►│ PGconn* handle   │    │
│         │ type: :postgres        │  │  │ socket: fd=42    │    │
│         │ state: :connected      │  │  └──────────────────┘    │
│         │ database: "mydb"       │  │                          │
│         │ pool: { min: 5, ... }  │  │  SQLite (sqlite3)        │
│         │                        │  │  ┌──────────────────┐    │
│         └── opened_by ──► module:my_app  │  │ sqlite3* handle  │    │
│                                  │  │  └──────────────────┘    │
│     └── sqlite_cache ────────────┼──┼─►                        │
│         type: :sqlite            │  │                          │
│         path: "cache.db"         │  │                          │
│                                  │  └──────────────────────────┘
└──────────────────────────────────┘
```

### Why Connections Are Nodes (Not Edges)

1. **Connections are entities** - They have state, properties, methods
2. **Consistent with FFI** - Phase 20 established Bridge Nodes for foreign resources
3. **Queryable** - Can inspect all connections via `reflect.universe().connections()`
4. **Edges express relationships** - `opened_by`, `pool`, etc.

### Connection Pools

Pools are properties of the connection, not separate top-level entities:

```graphoid
# Create pooled connection
pg = db.connect("postgresql://...", {
    pool: {
        min: 5,
        max: 20,
        idle_timeout: 300
    }
})

# Pool state is part of connection node
conn_info = reflect.universe().connections()["pg_main"]
conn_info.pool.active   # Number of active connections
conn_info.pool.idle     # Number of idle connections
```

### Transactions Are Ephemeral

Transactions are **execution state**, not persistent nodes. They exist only during the transaction block:

```graphoid
pg.transaction {
    # This block is ephemeral - no transaction node in universe
    pg.execute("INSERT ...")
    pg.execute("UPDATE ...")
}
# Transaction is gone - committed or rolled back
# Only the side effects (database changes) persist
```

The **result** of a query may be a graph (rows as nodes), but the transaction wrapper is control flow.

### Querying Connections

```graphoid
# List all open connections
for conn in reflect.universe().connections() {
    print(conn.type + ": " + conn.database + " [" + conn.state + "]")
}

# Find connections opened by a specific module
my_conns = reflect.universe().connections().filter(c =>
    c.has_edge({ label: "opened_by", to: "module:my_app" })
)

# Check pool health
pg = reflect.universe().connections()["pg_main"]
if pg.pool.active >= pg.pool.max {
    warn("Connection pool exhausted!")
}
```

---

## Core Features

### 1. Unified Database API

```graphoid
import "db"

# Connect to different databases with same API
pg = db.connect("postgresql://user:pass@localhost/mydb")
sqlite = db.connect("sqlite:///path/to/db.sqlite")
mysql = db.connect("mysql://user:pass@localhost/mydb")
redis = db.connect("redis://localhost:6379")

# Common query interface
results = pg.query("SELECT * FROM users WHERE age > $1", [18])

# Parameterized queries (safe from SQL injection)
user = pg.query_one("SELECT * FROM users WHERE id = $1", [user_id])

# Execute non-SELECT statements
pg.execute("INSERT INTO users (name, age) VALUES ($1, $2)", ["Alice", 30])
```

### 2. Transaction Support

```graphoid
# Explicit transactions
pg.transaction {
    pg.execute("UPDATE accounts SET balance = balance - 100 WHERE id = $1", [from_id])
    pg.execute("UPDATE accounts SET balance = balance + 100 WHERE id = $1", [to_id])
    # Commits on success, rolls back on error
}

# Manual transaction control
tx = pg.begin()
try {
    tx.execute("INSERT INTO orders ...")
    tx.execute("UPDATE inventory ...")
    tx.commit()
} catch e {
    tx.rollback()
    raise e
}
```

### 3. Connection Pooling

```graphoid
# Create connection pool
pool = db.pool("postgresql://...", {
    min_connections: 5,
    max_connections: 20,
    idle_timeout: 300  # seconds
})

# Get connection from pool
conn = pool.acquire()
results = conn.query("SELECT ...")
pool.release(conn)

# Or use with block
pool.with_connection(conn => {
    conn.query("SELECT ...")
})
```

### 4. Graph-to-Database Mapping

```graphoid
# Store a graph in database
graph_db.store(my_graph, "social_network")

# Load graph from database
loaded_graph = graph_db.load("social_network")

# Query graph data with SQL
friends = pg.query("
    SELECT n2.name
    FROM graph_nodes n1
    JOIN graph_edges e ON n1.id = e.from_node
    JOIN graph_nodes n2 ON e.to_node = n2.id
    WHERE n1.name = $1 AND e.type = 'FRIEND'
", ["Alice"])
```

### 5. Redis (Key-Value & Pub/Sub)

```graphoid
import "db/redis"

redis = db.connect("redis://localhost:6379")

# Key-value operations
redis.set("user:1:name", "Alice")
name = redis.get("user:1:name")

# Hash operations
redis.hset("user:1", "name", "Alice")
redis.hset("user:1", "age", "30")
user = redis.hgetall("user:1")

# Lists
redis.lpush("queue", "job1")
job = redis.rpop("queue")

# Pub/Sub
redis.subscribe("events", fn(message) {
    print("Received: " + message)
})
redis.publish("events", "user_created")
```

---

## Database-Specific Features

### PostgreSQL

```graphoid
# JSON support
pg.execute("INSERT INTO data (doc) VALUES ($1::jsonb)", [json.encode(my_map)])
doc = pg.query_one("SELECT doc FROM data WHERE doc->>'name' = $1", ["Alice"])

# Array support
pg.execute("INSERT INTO tags (values) VALUES ($1)", [["tag1", "tag2", "tag3"]])

# LISTEN/NOTIFY
pg.listen("events", fn(payload) {
    print("Event: " + payload)
})
pg.notify("events", "something happened")
```

### SQLite

```graphoid
# In-memory database
mem_db = db.connect("sqlite://:memory:")

# Attach additional databases
sqlite.execute("ATTACH DATABASE 'other.db' AS other")

# Custom functions
sqlite.create_function("my_upper", fn(s) {
    return s.upper()
})
sqlite.query("SELECT my_upper(name) FROM users")
```

### MySQL

```graphoid
# Prepared statements
stmt = mysql.prepare("SELECT * FROM users WHERE status = ?")
active = stmt.execute(["active"])
inactive = stmt.execute(["inactive"])
stmt.close()

# Multiple result sets
results = mysql.query_multi("CALL get_user_with_orders($1)", [user_id])
user = results[0]
orders = results[1]
```

---

## Implementation Plan

### Day 1-2: Core Database Abstraction

```rust
// Database connection trait
trait DatabaseConnection {
    fn query(&self, sql: &str, params: &[Value]) -> Result<Vec<Row>>;
    fn execute(&self, sql: &str, params: &[Value]) -> Result<u64>;
    fn begin_transaction(&self) -> Result<Transaction>;
}

// Row type
struct Row {
    columns: Vec<String>,
    values: Vec<Value>,
}
```

### Day 3-4: PostgreSQL Driver (via FFI)

```rust
// Use libpq via FFI
struct PostgresConnection {
    conn: *mut PGconn,
}

impl DatabaseConnection for PostgresConnection {
    fn query(&self, sql: &str, params: &[Value]) -> Result<Vec<Row>> {
        // Convert params to C format
        // Call PQexecParams
        // Convert result to Rows
    }
}
```

### Day 5-6: SQLite Driver (via FFI)

```rust
// Use sqlite3 via FFI
struct SqliteConnection {
    db: *mut sqlite3,
}

impl DatabaseConnection for SqliteConnection {
    // Similar implementation using sqlite3_* functions
}
```

### Day 7-8: Redis Client

```rust
// Redis protocol implementation (pure Graphoid possible!)
struct RedisConnection {
    socket: TcpStream,
}

impl RedisConnection {
    fn command(&self, cmd: &[&str]) -> Result<Value> {
        // Send RESP protocol
        // Parse response
    }
}
```

### Day 9-10: Connection Pooling & Testing

```rust
struct ConnectionPool {
    connections: Vec<Box<dyn DatabaseConnection>>,
    available: VecDeque<usize>,
    config: PoolConfig,
}

impl ConnectionPool {
    fn acquire(&self) -> Result<PooledConnection>;
    fn release(&self, conn: PooledConnection);
}
```

---

## Success Criteria

### Universe Integration
- [ ] Connections are Bridge Nodes in `universe:connections`
- [ ] `reflect.universe().connections()` returns all open connections
- [ ] Connection nodes have `opened_by` edges to owning module
- [ ] Pool state queryable via connection node properties

### Database Support
- [ ] PostgreSQL: connect, query, execute, transactions
- [ ] SQLite: connect, query, execute, in-memory
- [ ] MySQL: connect, query, execute (basic support)
- [ ] Redis: get, set, hget, hset, lpush, rpop, pub/sub
- [ ] Connection pooling
- [ ] Parameterized queries (SQL injection safe)
- [ ] Transaction support with rollback

### Testing & Documentation
- [ ] At least 50 database tests
- [ ] Example: CRUD application
- [ ] Example: Graph persistence
- [ ] Example: Connection introspection via universe graph
- [ ] Documentation complete

---

## Example: Graph Persistence

```graphoid
import "db"
import "json"

# Store graph nodes and edges in PostgreSQL
fn save_graph(pg, graph, name) {
    pg.transaction {
        # Clear existing data
        pg.execute("DELETE FROM graph_edges WHERE graph_name = $1", [name])
        pg.execute("DELETE FROM graph_nodes WHERE graph_name = $1", [name])

        # Insert nodes
        for node_id in graph.nodes() {
            value = graph.get_node_value(node_id)
            pg.execute("
                INSERT INTO graph_nodes (graph_name, node_id, value)
                VALUES ($1, $2, $3)
            ", [name, node_id, json.encode(value)])
        }

        # Insert edges
        for edge in graph.edges() {
            pg.execute("
                INSERT INTO graph_edges (graph_name, from_node, to_node, edge_type, weight)
                VALUES ($1, $2, $3, $4, $5)
            ", [name, edge.from, edge.to, edge.type, edge.weight])
        }
    }
}

fn load_graph(pg, name) {
    g = graph { type: :directed }

    # Load nodes
    nodes = pg.query("SELECT node_id, value FROM graph_nodes WHERE graph_name = $1", [name])
    for row in nodes {
        g.add_node(row["node_id"], json.decode(row["value"]))
    }

    # Load edges
    edges = pg.query("SELECT * FROM graph_edges WHERE graph_name = $1", [name])
    for row in edges {
        g.add_edge(row["from_node"], row["to_node"], row["edge_type"], row["weight"])
    }

    return g
}
```

---

## Schema for Graph Storage

```sql
-- PostgreSQL schema for graph storage
CREATE TABLE graph_nodes (
    id SERIAL PRIMARY KEY,
    graph_name VARCHAR(255) NOT NULL,
    node_id VARCHAR(255) NOT NULL,
    value JSONB,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(graph_name, node_id)
);

CREATE TABLE graph_edges (
    id SERIAL PRIMARY KEY,
    graph_name VARCHAR(255) NOT NULL,
    from_node VARCHAR(255) NOT NULL,
    to_node VARCHAR(255) NOT NULL,
    edge_type VARCHAR(255),
    weight DOUBLE PRECISION,
    properties JSONB,
    created_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (graph_name, from_node) REFERENCES graph_nodes(graph_name, node_id),
    FOREIGN KEY (graph_name, to_node) REFERENCES graph_nodes(graph_name, node_id)
);

CREATE INDEX idx_graph_nodes_name ON graph_nodes(graph_name);
CREATE INDEX idx_graph_edges_name ON graph_edges(graph_name);
CREATE INDEX idx_graph_edges_from ON graph_edges(from_node);
CREATE INDEX idx_graph_edges_to ON graph_edges(to_node);
```

---

## Open Questions

1. **ORM or Raw SQL?** - Should we provide an ORM-like layer?
2. **Migration support?** - Schema migrations for graph storage?
3. **Query builder?** - Programmatic SQL construction?
4. **Async database access?** - Integration with Phase 15 concurrency?

---

## Resolved Questions

| Question | Resolution |
|----------|------------|
| Are DB connections nodes? | Yes - Bridge Nodes in `universe:connections`, consistent with FFI model |
| Are connections edges? | No - connections are entities with state/properties, not relationships |
| Where do pools go? | Pool is a property of the connection node, not a separate entity |
| Are transactions nodes? | No - transactions are ephemeral execution state, not persistent nodes |

---

## Related Documents

- [PHASE_18_COMPLETE_GRAPH_MODEL.md](PHASE_18_COMPLETE_GRAPH_MODEL.md) - Universe graph structure (defines `universe:connections`)
- [PHASE_20_FFI.md](PHASE_20_FFI.md) - Bridge Node pattern for foreign resources
- [PHASE_21_PACKAGE_MANAGER.md](PHASE_21_PACKAGE_MANAGER.md) - Third-party DB drivers via packages
- [PHASE_23_DISTRIBUTED_PRIMITIVES.md](PHASE_23_DISTRIBUTED_PRIMITIVES.md) - Distributed graph storage

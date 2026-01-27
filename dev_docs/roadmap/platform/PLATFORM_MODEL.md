# Platform Model

**Status**: Design
**Priority**: 6 (Application Layer)
**Dependencies**: Graphoid graph rules, Platform Loader, Database (Phase 22)

---

## Overview

The Platform Model component provides data modeling capabilities where **models ARE graphs**. This isn't an ORM wrapping graphs — validation IS graph rules, relationships ARE edges, and queries ARE graph traversal.

**Key Principles**:
- Models are graphs (not wrappers around graphs)
- Validation is graph rules
- Relationships are edges
- Queries are graph traversal
- No separate abstraction layer

---

## Basic Model

A model is a graph with structure and behavior:

```graphoid
# app/models/player.gr
import "platform/model"

Player = model.define("player")

# Attributes
Player.attribute("name", :string)
Player.attribute("health", :integer, { default: 100 })
Player.attribute("level", :integer, { default: 1 })
Player.attribute("position", :list, { default: [0, 0] })
Player.attribute("created_at", :timestamp)

# Validations (become graph rules)
Player.validate("name", :presence)
Player.validate("name", :length, { min: 1, max: 50 })
Player.validate("health", :range, { min: 0, max: 100 })
Player.validate("level", :range, { min: 1 })
```

---

## Creating Instances

```graphoid
import "models/player"

# Create a new player
p = Player.new({ name: "Alice" })
# → Graph with type: :player, attributes set

p.name          # → "Alice"
p.health        # → 100 (default)
p.level         # → 1 (default)

# Modify
p.health = 80
p.level = 5
```

---

## Validation

Validation uses Graphoid's graph rules under the hood:

```graphoid
p = Player.new({ name: "" })

p.valid?()      # → false
p.errors()      # → { name: ["can't be blank", "is too short (minimum 1)"] }

p.name = "Bob"
p.valid?()      # → true
p.errors()      # → {}
```

### Built-in Validators

| Validator | Usage | Description |
|-----------|-------|-------------|
| `:presence` | `validate("name", :presence)` | Must not be nil/empty |
| `:length` | `validate("name", :length, {min: 1, max: 50})` | String length |
| `:range` | `validate("age", :range, {min: 0, max: 150})` | Numeric range |
| `:format` | `validate("email", :format, {pattern: ".*@.*"})` | Regex match |
| `:inclusion` | `validate("status", :inclusion, {in: [:active, :inactive]})` | Must be in set |
| `:exclusion` | `validate("name", :exclusion, {from: ["admin", "root"]})` | Must not be in set |
| `:uniqueness` | `validate("email", :uniqueness)` | Unique in collection (requires persistence) |

### Custom Validators

```graphoid
Player.validate("position", fn(player) {
    x = player.position[0]
    y = player.position[1]

    if x < 0 or y < 0 {
        return "position must be non-negative"
    }

    if x > 100 or y > 100 {
        return "position must be within bounds"
    }

    return none  # Valid
})
```

### Cross-Field Validation

```graphoid
# Event model
Event = model.define("event")
Event.attribute("start_date", :date)
Event.attribute("end_date", :date)

Event.validate(fn(event) {
    if event.end_date < event.start_date {
        return { end_date: "must be after start date" }
    }
    return none
})
```

---

## Callbacks

Lifecycle hooks for custom behavior:

```graphoid
Player.before_validate(fn(player) {
    player.name = player.name.trim()
})

Player.after_create(fn(player) {
    log("Player created: " + player.name)
    stats.increment("players_created")
})

Player.before_save(fn(player) {
    player.updated_at = time.now()
})

Player.after_destroy(fn(player) {
    log("Player deleted: " + player.name)
})
```

### Callback Order

```
new()
  → before_validate
  → validate (graph rules run)
  → after_validate

save()
  → before_validate
  → validate
  → after_validate
  → before_save
  → [before_create or before_update]
  → [persist to database]
  → [after_create or after_update]
  → after_save

destroy()
  → before_destroy
  → [remove from database]
  → after_destroy
```

---

## Dirty Tracking

Know what changed since load or last save:

```graphoid
p = Player.new({ name: "Alice", health: 100 })

p.changed?()          # → false
p.changes()           # → {}

p.health = 80

p.changed?()          # → true
p.changes()           # → { health: { from: 100, to: 80 } }
p.health_changed?()   # → true
p.health_was()        # → 100

p.save()
p.changed?()          # → false (reset after save)
```

---

## Querying (In-Memory)

Query models in memory collections:

```graphoid
# Assuming players is a list of Player instances
players = [player1, player2, player3, ...]

# Find by attribute
alive = Player.where(players, fn(p) { p.health > 0 })
high_level = Player.where(players, fn(p) { p.level > 10 })

# Find one
alice = Player.find(players, fn(p) { p.name == "Alice" })

# Sorting
by_level = Player.order(players, "level", :desc)

# Chaining
result = players
    .filter(fn(p) { p.health > 0 })
    .filter(fn(p) { p.level > 5 })
    .sort(fn(a, b) { b.level - a.level })
```

---

## Persistence (Optional)

When database is available, models can be persisted:

```graphoid
import "platform/model"
import "database"

Player = model.define("player", { table: "players" })

# ... attribute and validation definitions ...

# Connect to database
db = database.connect("game.db")
Player.use_database(db)
```

### CRUD Operations

```graphoid
# Create and save
p = Player.new({ name: "Alice" })
p.save()        # Inserts into database

# Read
p = Player.find(123)                    # By ID
p = Player.find_by({ name: "Alice" })   # By attribute
players = Player.all()                   # All records
players = Player.where({ level: 5 })    # With conditions

# Update
p.health = 80
p.save()        # Updates database

# Delete
p.destroy()     # Removes from database
```

### Query Builder

```graphoid
# SQL-like querying
players = Player
    .where({ active: true })
    .where("level > ?", 5)
    .order("created_at", :desc)
    .limit(10)
    .all()

# Aggregates
count = Player.count()
avg_level = Player.average("level")
max_health = Player.maximum("health")
```

---

## Associations (Graph Edges)

Relationships between models are edges in a graph:

```graphoid
# Player has many games — creates edges
Player.has_many("games")

# Game belongs to player — edge in opposite direction
Game = model.define("game")
Game.belongs_to("player")

# Usage — just graph traversal
player = Player.find(1)
games = player.games           # Follow edges to related nodes

game = Game.find(1)
owner = game.player            # Follow edge back to owner

# The relationship IS an edge
player.games.add(new_game)     # Creates edge: player → new_game
```

Since models are graphs and relationships are edges, querying relationships is natural graph traversal.

---

## Serialization

Convert to/from common formats:

```graphoid
p = Player.new({ name: "Alice", health: 100 })

# To hash
data = p.to_hash()
# → { name: "Alice", health: 100, level: 1, position: [0, 0] }

# To JSON
json_str = p.to_json()
# → '{"name":"Alice","health":100,"level":1,"position":[0,0]}'

# From hash
p2 = Player.from_hash(data)

# From JSON
p3 = Player.from_json(json_str)
```

### Controlling Serialization

```graphoid
Player.attribute("password", :string, { serialize: false })  # Never serialize
Player.attribute("email", :string, { serialize: :admin })    # Only in admin context

p.to_json()                     # Excludes password
p.to_json({ context: :admin })  # Includes email
```

---

## Migrations

Schema evolution for persistent models:

```graphoid
# db/migrations/001_create_players.gr

fn up(db) {
    db.create_table("players", {
        id: :auto,
        name: { type: :string, null: false },
        health: { type: :integer, default: 100 },
        level: { type: :integer, default: 1 },
        position: :json,
        created_at: :timestamp,
        updated_at: :timestamp
    })

    db.add_index("players", "name")
}

fn down(db) {
    db.drop_table("players")
}
```

```graphoid
# db/migrations/002_add_email_to_players.gr

fn up(db) {
    db.add_column("players", "email", :string)
    db.add_index("players", "email", { unique: true })
}

fn down(db) {
    db.remove_column("players", "email")
}
```

---

## Integration with Graph Rules

Models leverage Graphoid's native graph rules:

```graphoid
Player = model.define("player")

# This validation...
Player.validate("health", :range, { min: 0, max: 100 })

# ...becomes this graph rule on each instance
# player.add_rule("health_range", 0, 100)

# Custom rules can also be attached
Player.after_create(fn(player) {
    player.add_rule("position_bounds", 0, 100)
    player.add_behavior(:trackable)
})
```

---

## Language Requirements

| Capability | Status | Notes |
|------------|--------|-------|
| Graph rules | Exists | Validation foundation |
| Graph behaviors | Exists | For callbacks, tracking |
| JSON encode/decode | Exists | Serialization |
| Database access | Phase 22 | For persistence |

### What's Already in Graphoid

Models primarily wrap existing capabilities:
- Validation → Graph rules (`add_rule`)
- Behaviors → Graph behaviors (`add_behavior`)
- Serialization → JSON module

### What's Needed

1. **Schema introspection**: Know what attributes a graph type has
2. **Dirty tracking behavior**: Built-in behavior for change tracking
3. **Database abstraction**: Phase 22 provides this

---

## Implementation Phases

### Phase 1: Attribute Definition

- `model.define()` function
- Attribute declaration with types and defaults
- Instance creation with `Model.new()`

**Milestone**: Can define and create models

### Phase 2: Validation

- Built-in validators (presence, range, length, etc.)
- Custom validators
- Error collection
- Integration with graph rules

**Milestone**: Validation works

### Phase 3: Callbacks

- Lifecycle hooks (before/after validate, save, create, update, destroy)
- Callback ordering

**Milestone**: Callbacks fire correctly

### Phase 4: Dirty Tracking

- Change detection
- `changed?()`, `changes()`, `*_was()` methods
- Reset on save

**Milestone**: Know what changed

### Phase 5: Serialization

- `to_hash()`, `to_json()`
- `from_hash()`, `from_json()`
- Serialization control (exclude fields, contexts)

**Milestone**: Models serialize correctly

### Phase 6: In-Memory Querying

- `where()`, `find()`, `order()`
- Method chaining
- Works without database

**Milestone**: Query in-memory collections

### Phase 7: Persistence

- Database integration (requires Phase 22)
- `save()`, `find()`, `destroy()`
- Query builder for SQL

**Milestone**: Models persist to database

### Phase 8: Associations

- `has_many`, `belongs_to`, `has_one`
- Eager loading
- Cascading operations

**Milestone**: Model relationships work

---

## Open Questions

1. **Graph type vs model**: Is a Model a special graph type, or a wrapper around graphs?

2. **Inheritance**: Can models inherit from other models?

3. **Polymorphism**: Can different model types be stored in same collection?

4. **Transactions**: How to handle atomic multi-model operations?

---

## Related Documents

- [PLATFORM_LOADER.md](PLATFORM_LOADER.md) — Auto-loading models from app/models/
- [PHASE_22_DATABASE.md](../PHASE_22_DATABASE.md) — Database connectivity
- Graphoid graph rules — Foundation for validation

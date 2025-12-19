# Social Network Analyzer

A graph-based social network analyzer demonstrating Class-Like Graphs (CLGs) and the "everything is a graph" philosophy.

## Status

**Current:** Complete (December 2025)

- [x] User management (add, remove, profiles)
- [x] Friendship relationships (bidirectional)
- [x] Follow relationships (unidirectional)
- [x] Network analysis (mutual friends, degrees of separation)
- [x] Friend suggestions based on mutual connections

## Run

```bash
graphoid samples/06-projects/social/main.gr
```

## What This Demonstrates

### 1. Social Networks ARE Graphs

This isn't a metaphor - the social network IS literally stored as a graph:

```
alice --friend--> bob
alice --friend--> charlie
bob --friend--> alice       (bidirectional)
henry --follows--> alice    (unidirectional)
```

Users are nodes. Relationships are edges. Graph algorithms provide network analysis.

### 2. CLG as Network Container

The `SocialNetwork` CLG encapsulates all network logic:

```graphoid
network = SocialNetwork.new("Tech Community")
network.add_user("alice", {"role": "engineer"})
network.add_friendship("alice", "bob")
network.degrees_of_separation("alice", "charlie")  # BFS path finding
```

### 3. Graph Algorithms for Network Analysis

- **Degrees of separation**: BFS traversal to find connection distance
- **Mutual friends**: Set intersection of friend lists
- **Friend suggestions**: Friends-of-friends ranked by mutual connections

## File Structure

```
social/
├── README.md           # This file
├── social_network.gr   # SocialNetwork CLG
└── main.gr             # Demo script
```

## API Reference

### User Management

| Method | Description |
|--------|-------------|
| `add_user(name, profile)` | Add a user with profile data |
| `get_user(name)` | Get user's profile |
| `remove_user(name)` | Remove a user |

### Relationships

| Method | Description |
|--------|-------------|
| `add_friendship(user1, user2)` | Add bidirectional friendship |
| `add_follow(follower, followed)` | Add unidirectional follow |
| `remove_friendship(user1, user2)` | Remove friendship |
| `are_friends(user1, user2)` | Check if two users are friends |

### Analysis

| Method | Description |
|--------|-------------|
| `friends_of(user)` | List of user's friends |
| `followers_of(user)` | List of user's followers |
| `following(user)` | List of users this person follows |
| `friend_count(user)` | Number of friends |
| `mutual_friends(user1, user2)` | Friends in common |
| `degrees_of_separation(user1, user2)` | Connection distance (-1 if not connected) |
| `friends_of_friends(user)` | 2nd-degree connections |
| `suggest_friends(user)` | Friend suggestions ranked by mutual count |

### Network Stats

| Property/Method | Description |
|-----------------|-------------|
| `user_count` | Total users |
| `connection_count` | Total friendships |
| `follow_count` | Total follows |
| `most_connected()` | User with most friends |
| `least_connected()` | User with fewest friends |
| `average_connections()` | Mean friends per user |

### Reporting

| Method | Description |
|--------|-------------|
| `report()` | Print network summary |
| `user_report(name)` | Print individual user stats |

## Sample Output

```
=== Tech Community Report ===

Network Size:
  Users: 8
  Friendships: 12
  Follows: 3

Connectivity:
  Most connected: alice (4 friends)
  Least connected: frank (1 friends)
  Average connections: 3

--- Network Analysis ---

Degrees of Separation:
  Alice -> Bob: 1 degree(s)
  Alice -> Grace: 2 degree(s)
  Frank -> Grace: 3 degree(s)

Friend Suggestions for Frank:
  alice - 1 mutual friend(s): [eve]
  diana - 1 mutual friend(s): [eve]
```

## Key Takeaways

1. **Social networks ARE graphs** - Not a wrapper, the actual data structure
2. **Bidirectional vs unidirectional** - Friendships are symmetric, follows are not
3. **Graph algorithms shine** - BFS makes "degrees of separation" straightforward
4. **CLGs encapsulate complexity** - Clean API hides graph traversal details
5. **No async needed** - Tick-based isn't relevant here; analysis is synchronous

## CLG Features Used

| Feature | Where Used |
|---------|------------|
| Graph declaration | `graph SocialNetwork { }` |
| Properties | `_name` |
| Methods | `add_user()`, `friends_of()`, `mutual_friends()`, etc. |
| Private methods | `_is_user_node()` to filter internal nodes |
| Graph operations | `add_node()`, `add_edge()`, `edges()`, `nodes()` |
| Iteration | `for edge in self.edges()`, `for node in self.nodes()` |
| BFS algorithm | `degrees_of_separation()` implements path finding |

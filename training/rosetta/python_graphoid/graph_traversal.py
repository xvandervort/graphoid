"""Graph Traversal — Python equivalent of graph_traversal.gr

Uses networkx for graph operations. Python doesn't have built-in
graph types, so an external library is required.
"""

import networkx as nx

# --- Create a directed graph ---
g = nx.DiGraph()

# Add nodes with data
g.add_node("alice", label="Alice")
g.add_node("bob", label="Bob")
g.add_node("carol", label="Carol")
g.add_node("dave", label="Dave")
g.add_node("eve", label="Eve")

# Add edges with type labels
g.add_edge("alice", "bob", type="FRIEND")
g.add_edge("alice", "carol", type="FRIEND")
g.add_edge("bob", "carol", type="FRIEND")
g.add_edge("bob", "dave", type="FRIEND")
g.add_edge("carol", "eve", type="FRIEND")

# --- Basic queries ---
print(f"Nodes: {list(g.nodes())}")
print(f"Edge count: {g.number_of_edges()}")
print(f"Has alice? {g.has_node('alice')}")

# --- Neighbors ---
alice_friends = list(g.successors("alice"))
print(f"Alice's friends: {alice_friends}")

# --- Path finding ---
has_path = nx.has_path(g, "alice", "eve")
print(f"Path alice->eve exists? {has_path}")

shortest = nx.shortest_path(g, "alice", "eve")
print(f"Shortest path: {shortest}")

# --- BFS and DFS ---
bfs_order = list(nx.bfs_tree(g, "alice").nodes())
print(f"BFS from alice: {bfs_order}")

dfs_order = list(nx.dfs_tree(g, "alice").nodes())
print(f"DFS from alice: {dfs_order}")

# --- Mutual friends ---
alice_friends_set = set(g.successors("alice"))
bob_friends_set = set(g.successors("bob"))
mutual = alice_friends_set & bob_friends_set
print(f"Mutual friends of alice and bob: {mutual}")

# --- Subgraph ---
# Extract nodes reachable within 1 hop from alice
close_friends = set(g.successors("alice")) | {"alice"}
sub = g.subgraph(close_friends)
print(f"Alice's neighborhood: {list(sub.nodes())}")

# --- Cycle detection ---
# Python: must call a function explicitly
print(f"Has cycles? {not nx.is_directed_acyclic_graph(g)}")

print("Graph traversal demo complete!")

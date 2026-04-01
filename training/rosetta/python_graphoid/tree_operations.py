"""Tree Operations — Python equivalent of tree_operations.gr

Python has no built-in tree type. We use a class-based approach
to model trees, which requires significant boilerplate.
"""

class TreeNode:
    def __init__(self, key, value):
        self.key = key
        self.value = value
        self.children = []

    def add_child(self, child):
        self.children.append(child)

    def is_leaf(self):
        return len(self.children) == 0


def build_org_chart():
    """Build an organizational hierarchy."""
    ceo = TreeNode("ceo", "CEO")
    vp_eng = TreeNode("vp_eng", "VP Engineering")
    vp_sales = TreeNode("vp_sales", "VP Sales")
    lead_be = TreeNode("lead_be", "Backend Lead")
    lead_fe = TreeNode("lead_fe", "Frontend Lead")
    dev1 = TreeNode("dev1", "Developer 1")
    dev2 = TreeNode("dev2", "Developer 2")

    ceo.add_child(vp_eng)
    ceo.add_child(vp_sales)
    vp_eng.add_child(lead_be)
    vp_eng.add_child(lead_fe)
    lead_be.add_child(dev1)
    lead_be.add_child(dev2)

    return ceo


def count_nodes(node):
    """Count total nodes in tree."""
    total = 1
    for child in node.children:
        total += count_nodes(child)
    return total


def find_leaves(node):
    """Find all leaf nodes."""
    if node.is_leaf():
        return [node.key]
    leaves = []
    for child in node.children:
        leaves.extend(find_leaves(child))
    return leaves


def tree_depth(node):
    """Calculate tree depth."""
    if node.is_leaf():
        return 1
    return 1 + max(tree_depth(child) for child in node.children)


def find_node(node, key):
    """Find a node by key (DFS)."""
    if node.key == key:
        return node
    for child in node.children:
        result = find_node(child, key)
        if result is not None:
            return result
    return None


# --- Build and query the tree ---
root = build_org_chart()

print(f"Total nodes: {count_nodes(root)}")
print(f"Leaves: {find_leaves(root)}")
print(f"Depth: {tree_depth(root)}")

# Find a node
vp = find_node(root, "vp_eng")
if vp:
    print(f"Found: {vp.value}")
    print(f"Direct reports: {[c.key for c in vp.children]}")

# No built-in cycle prevention, single-root enforcement,
# or structural rules — must implement manually.

print("Tree operations demo complete!")

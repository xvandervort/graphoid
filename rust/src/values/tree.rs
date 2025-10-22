//! Tree data structure implementation
//!
//! Trees in Graphoid are binary search trees. They can also be represented
//! as graphs with tree-specific rules.

use super::Value;

/// A node in the binary search tree
#[derive(Debug, Clone, PartialEq)]
pub struct TreeNode {
    /// Node value (must be comparable)
    pub value: Value,
    /// Left child
    pub left: Option<Box<TreeNode>>,
    /// Right child
    pub right: Option<Box<TreeNode>>,
}

/// Binary search tree
#[derive(Debug, Clone, PartialEq)]
pub struct Tree {
    /// Root node
    pub root: Option<Box<TreeNode>>,
    /// Number of elements
    pub size: usize,
}

impl Tree {
    /// Create a new empty tree
    pub fn new() -> Self {
        Tree {
            root: None,
            size: 0,
        }
    }

    /// Insert a value into the tree
    pub fn insert(&mut self, value: Value) {
        self.root = Self::insert_rec(self.root.take(), value);
        self.size += 1;
    }

    /// Recursive helper for insertion
    fn insert_rec(node: Option<Box<TreeNode>>, value: Value) -> Option<Box<TreeNode>> {
        match node {
            None => Some(Box::new(TreeNode {
                value,
                left: None,
                right: None,
            })),
            Some(mut n) => {
                // Compare values for BST property
                // For now, only support numbers for comparison
                if let (Value::Number(new_val), Value::Number(node_val)) = (&value, &n.value) {
                    if new_val < node_val {
                        n.left = Self::insert_rec(n.left, value);
                    } else {
                        n.right = Self::insert_rec(n.right, value);
                    }
                }
                Some(n)
            }
        }
    }

    /// Check if tree contains a value
    pub fn contains(&self, value: &Value) -> bool {
        Self::contains_rec(&self.root, value)
    }

    /// Recursive helper for contains
    fn contains_rec(node: &Option<Box<TreeNode>>, value: &Value) -> bool {
        match node {
            None => false,
            Some(n) => {
                if n.value == *value {
                    true
                } else if let (Value::Number(search_val), Value::Number(node_val)) = (value, &n.value) {
                    if search_val < node_val {
                        Self::contains_rec(&n.left, value)
                    } else {
                        Self::contains_rec(&n.right, value)
                    }
                } else {
                    false
                }
            }
        }
    }

    /// Get in-order traversal (sorted for BST)
    pub fn in_order(&self) -> Vec<Value> {
        let mut result = Vec::new();
        Self::in_order_rec(&self.root, &mut result);
        result
    }

    /// Recursive helper for in-order traversal
    fn in_order_rec(node: &Option<Box<TreeNode>>, result: &mut Vec<Value>) {
        if let Some(n) = node {
            Self::in_order_rec(&n.left, result);
            result.push(n.value.clone());
            Self::in_order_rec(&n.right, result);
        }
    }

    /// Get pre-order traversal
    pub fn pre_order(&self) -> Vec<Value> {
        let mut result = Vec::new();
        Self::pre_order_rec(&self.root, &mut result);
        result
    }

    /// Recursive helper for pre-order traversal
    fn pre_order_rec(node: &Option<Box<TreeNode>>, result: &mut Vec<Value>) {
        if let Some(n) = node {
            result.push(n.value.clone());
            Self::pre_order_rec(&n.left, result);
            Self::pre_order_rec(&n.right, result);
        }
    }

    /// Get post-order traversal
    pub fn post_order(&self) -> Vec<Value> {
        let mut result = Vec::new();
        Self::post_order_rec(&self.root, &mut result);
        result
    }

    /// Recursive helper for post-order traversal
    fn post_order_rec(node: &Option<Box<TreeNode>>, result: &mut Vec<Value>) {
        if let Some(n) = node {
            Self::post_order_rec(&n.left, result);
            Self::post_order_rec(&n.right, result);
            result.push(n.value.clone());
        }
    }

    /// Get size of tree
    pub fn len(&self) -> usize {
        self.size
    }

    /// Check if tree is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self::new()
    }
}

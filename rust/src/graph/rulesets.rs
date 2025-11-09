//! Predefined rulesets for common graph structures
//!
//! This module defines predefined bundles of rules that can be applied to graphs
//! to enforce specific structural constraints. Rulesets support composition,
//! allowing more specialized rulesets to inherit rules from base rulesets.
//!
//! # Ruleset Hierarchy
//!
//! ```text
//! graph (no rules)
//!   └─ :tree (no_cycles, single_root, connected)
//!       └─ :binary_tree (tree + max_degree 2)
//!           └─ :bst (binary_tree + bst_ordering)
//!   └─ :dag (no_cycles only)
//! ```
//!
//! # Example
//!
//! ```
//! use graphoid::values::{Graph, GraphType};
//! use graphoid::graph::rulesets::get_ruleset_rules;
//!
//! let mut g = Graph::new(GraphType::Directed).with_ruleset("tree".to_string());
//! // Graph now enforces tree rules: no_cycles, single_root, connected
//! ```

use super::rules::{RuleSpec, RuleInstance};

/// Get the rules that should be applied for a given ruleset name
///
/// This function returns all rules for the given ruleset, including any
/// inherited rules from parent rulesets.
///
/// # Supported Rulesets
///
/// - `:tree` - Basic tree structure (no_cycles, single_root, connected)
/// - `:binary_tree` - Binary tree (includes :tree + max 2 children)
/// - `:bst` - Binary search tree (includes :binary_tree + BST ordering)
/// - `:dag` - Directed acyclic graph (no_cycles only)
///
/// # Arguments
///
/// * `ruleset` - The name of the ruleset (e.g., "tree", "dag", "binary_tree")
///
/// # Returns
///
/// A vector of RuleInstance objects that should be enforced for this ruleset.
/// Returns an empty vector if the ruleset name is not recognized.
pub fn get_ruleset_rules(ruleset: &str) -> Vec<RuleInstance> {
    match ruleset {
        "tree" => ruleset_tree(),
        "binary_tree" => ruleset_binary_tree(),
        "bst" => ruleset_bst(),
        "dag" => ruleset_dag(),
        _ => Vec::new(), // Unknown ruleset - no rules
    }
}

/// Check if a ruleset name is recognized
pub fn is_valid_ruleset(ruleset: &str) -> bool {
    matches!(ruleset, "tree" | "binary_tree" | "bst" | "dag")
}

/// List all available predefined rulesets
pub fn available_rulesets() -> Vec<&'static str> {
    vec!["tree", "binary_tree", "bst", "dag"]
}

// ============================================================================
// Ruleset Definitions
// ============================================================================

/// :tree ruleset - Basic tree structure
///
/// Enforces:
/// - no_cycles: Tree cannot have circular paths
/// - single_root: Exactly one node with no incoming edges
/// - connected: All nodes must be reachable from the root
///
/// This is the most general tree type, allowing any branching factor.
fn ruleset_tree() -> Vec<RuleInstance> {
    vec![
        RuleInstance::new(RuleSpec::NoCycles),
        RuleInstance::new(RuleSpec::SingleRoot),
        RuleInstance::new(RuleSpec::Connected),
    ]
}

/// :binary_tree ruleset - Binary tree structure
///
/// Includes all :tree rules plus:
/// - max_degree 2: Each node can have at most 2 children
///
/// This allows for any binary tree structure (not necessarily ordered).
fn ruleset_binary_tree() -> Vec<RuleInstance> {
    let mut rules = ruleset_tree(); // Inherit tree rules
    rules.push(RuleInstance::new(RuleSpec::MaxDegree(2)));
    rules
}

/// :bst ruleset - Binary search tree
///
/// Includes all :binary_tree rules plus:
/// - BST ordering: Left child < parent < right child
///
/// The BST ordering rule enforces that:
/// - All node values must be numeric
/// - For edges labeled "left": child value < parent value
/// - For edges labeled "right": child value > parent value
/// - Equality is not allowed (strict ordering)
fn ruleset_bst() -> Vec<RuleInstance> {
    let mut rules = ruleset_binary_tree(); // Inherit binary_tree rules
    rules.push(RuleInstance::new(RuleSpec::BSTOrdering));
    rules
}

/// :dag ruleset - Directed acyclic graph
///
/// Enforces:
/// - no_cycles: Graph cannot have circular paths
///
/// Unlike trees, DAGs allow:
/// - Multiple roots (nodes with no incoming edges)
/// - Multiple parents per node
/// - Disconnected components
fn ruleset_dag() -> Vec<RuleInstance> {
    vec![RuleInstance::new(RuleSpec::NoCycles)]
}

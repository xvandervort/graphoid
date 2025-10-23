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
//!           └─ :bst (binary_tree + bst_ordering - not yet implemented)
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
/// - `:bst` - Binary search tree (includes :binary_tree + ordering - TODO)
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
/// - BST ordering: Left child < parent < right child (TODO - not yet implemented)
///
/// Note: The BST ordering rule is not yet implemented. Currently this ruleset
/// behaves identically to :binary_tree. BST-specific behavior (automatic ordering
/// on insert) will be added in a future phase.
fn ruleset_bst() -> Vec<RuleInstance> {
    // TODO: Add BSTOrderingRule when implemented
    // For now, BST is identical to binary_tree
    ruleset_binary_tree()
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

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::rules::RuleSeverity;

    #[test]
    fn test_tree_ruleset_has_three_rules() {
        let rules = ruleset_tree();
        assert_eq!(rules.len(), 3);

        // Verify the specific rules (order doesn't matter)
        let specs: Vec<&RuleSpec> = rules.iter().map(|r| &r.spec).collect();
        assert!(specs.contains(&&RuleSpec::NoCycles));
        assert!(specs.contains(&&RuleSpec::SingleRoot));
        assert!(specs.contains(&&RuleSpec::Connected));
    }

    #[test]
    fn test_binary_tree_includes_tree_rules() {
        let rules = ruleset_binary_tree();
        assert_eq!(rules.len(), 4); // 3 tree rules + 1 max_degree rule

        let specs: Vec<&RuleSpec> = rules.iter().map(|r| &r.spec).collect();
        assert!(specs.contains(&&RuleSpec::NoCycles));
        assert!(specs.contains(&&RuleSpec::SingleRoot));
        assert!(specs.contains(&&RuleSpec::Connected));
        assert!(specs.contains(&&RuleSpec::MaxDegree(2)));
    }

    #[test]
    fn test_bst_includes_binary_tree_rules() {
        let rules = ruleset_bst();
        // Currently identical to binary_tree
        assert_eq!(rules.len(), 4);

        let specs: Vec<&RuleSpec> = rules.iter().map(|r| &r.spec).collect();
        assert!(specs.contains(&&RuleSpec::MaxDegree(2)));
    }

    #[test]
    fn test_dag_has_one_rule() {
        let rules = ruleset_dag();
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].spec, RuleSpec::NoCycles);
    }

    #[test]
    fn test_get_ruleset_rules_tree() {
        let rules = get_ruleset_rules("tree");
        assert_eq!(rules.len(), 3);
    }

    #[test]
    fn test_get_ruleset_rules_binary_tree() {
        let rules = get_ruleset_rules("binary_tree");
        assert_eq!(rules.len(), 4);
    }

    #[test]
    fn test_get_ruleset_rules_dag() {
        let rules = get_ruleset_rules("dag");
        assert_eq!(rules.len(), 1);
    }

    #[test]
    fn test_get_ruleset_rules_unknown() {
        let rules = get_ruleset_rules("unknown_ruleset");
        assert_eq!(rules.len(), 0);
    }

    #[test]
    fn test_is_valid_ruleset() {
        assert!(is_valid_ruleset("tree"));
        assert!(is_valid_ruleset("binary_tree"));
        assert!(is_valid_ruleset("bst"));
        assert!(is_valid_ruleset("dag"));
        assert!(!is_valid_ruleset("unknown"));
        assert!(!is_valid_ruleset(""));
    }

    #[test]
    fn test_available_rulesets() {
        let rulesets = available_rulesets();
        assert_eq!(rulesets.len(), 4);
        assert!(rulesets.contains(&"tree"));
        assert!(rulesets.contains(&"binary_tree"));
        assert!(rulesets.contains(&"bst"));
        assert!(rulesets.contains(&"dag"));
    }

    #[test]
    fn test_all_rulesets_use_default_severity() {
        let rules = ruleset_tree();
        for rule in rules {
            // RuleInstance::new() uses default severity (Warning)
            assert_eq!(rule.severity, RuleSeverity::Warning);
        }
    }
}

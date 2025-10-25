//! Graph types and operations
//!
//! This module implements graph data structures, rules, and behaviors.

pub mod rules;
pub mod rulesets;
pub mod behaviors;

// Re-export commonly used types
pub use rules::{RuleSpec, RuleInstance, RuleSeverity, RetroactivePolicy};
pub use rulesets::{get_ruleset_rules, is_valid_ruleset, available_rulesets};
pub use behaviors::{BehaviorSpec, BehaviorInstance, Behavior, apply_behaviors, apply_retroactive_to_list, apply_retroactive_to_hash};

//! Graph types and operations
//!
//! This module implements graph data structures and rules.

pub mod rules;

// Re-export commonly used types
pub use rules::{RuleSpec, RuleInstance, RuleSeverity, RetroactivePolicy};

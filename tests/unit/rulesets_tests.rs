use graphoid::graph::rulesets::{get_ruleset_rules, is_valid_ruleset, available_rulesets};
use graphoid::graph::rules::{RuleSpec, RuleSeverity};

#[test]
fn test_tree_ruleset_has_three_rules() {
    let rules = get_ruleset_rules("tree");
    assert_eq!(rules.len(), 3);

    // Verify the specific rules (order doesn't matter)
    let specs: Vec<&RuleSpec> = rules.iter().map(|r| &r.spec).collect();
    assert!(specs.contains(&&RuleSpec::NoCycles));
    assert!(specs.contains(&&RuleSpec::SingleRoot));
    assert!(specs.contains(&&RuleSpec::Connected));
}

#[test]
fn test_binary_tree_includes_tree_rules() {
    let rules = get_ruleset_rules("binary_tree");
    assert_eq!(rules.len(), 4); // 3 tree rules + 1 max_degree rule

    let specs: Vec<&RuleSpec> = rules.iter().map(|r| &r.spec).collect();
    assert!(specs.contains(&&RuleSpec::NoCycles));
    assert!(specs.contains(&&RuleSpec::SingleRoot));
    assert!(specs.contains(&&RuleSpec::Connected));
    assert!(specs.contains(&&RuleSpec::MaxDegree(2)));
}

#[test]
fn test_bst_includes_binary_tree_rules() {
    let rules = get_ruleset_rules("bst");
    // BST includes binary_tree rules + BST ordering
    assert_eq!(rules.len(), 5);

    let specs: Vec<&RuleSpec> = rules.iter().map(|r| &r.spec).collect();
    assert!(specs.contains(&&RuleSpec::MaxDegree(2)));
    assert!(specs.contains(&&RuleSpec::BSTOrdering));
}

#[test]
fn test_dag_has_one_rule() {
    let rules = get_ruleset_rules("dag");
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
    let rules = get_ruleset_rules("tree");
    for rule in rules {
        // RuleInstance::new() uses default severity (Warning)
        assert_eq!(rule.severity, RuleSeverity::Warning);
    }
}

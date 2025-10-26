//! Behavior system for automatic value transformation
//!
//! Behaviors transform values during operations (append, insert, set).
//! Unlike rules (which validate), behaviors accept values after transformation.
//!
//! # Architecture
//!
//! - **Behavior trait**: Defines how to transform a value
//! - **BehaviorSpec**: Specification that can be stored and cloned
//! - **BehaviorInstance**: Wrapper with RetroactivePolicy
//! - **Application**: Applied sequentially, first added = first applied
//!
//! # Retroactive vs Proactive
//!
//! - **Retroactive**: Applied to existing values when behavior is added
//! - **Proactive**: Applied to new values during operations
//!
//! # Example
//!
//! ```graphoid
//! temperatures = [98.6, none, 102.5]
//! temperatures.add_rule(:none_to_zero)  # Retroactively transforms none → 0
//! temperatures.append(none)              # Proactively transforms none → 0
//! ```

use crate::error::GraphoidError;
use crate::graph::rules::{RetroactivePolicy, Rule, RuleContext, GraphOperation};
use crate::values::{Value, List, Graph};
use std::collections::HashMap;

/// Core behavior trait - transforms a value
///
/// Behaviors are applied during operations to automatically transform values.
/// Unlike rules (which validate), behaviors return transformed values.
pub trait Behavior: std::fmt::Debug {
    /// Transform a value according to this behavior
    ///
    /// # Arguments
    /// * `value` - The value to transform
    ///
    /// # Returns
    /// The transformed value, or an error if transformation fails
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError>;

    /// Get the name of this behavior
    ///
    /// Used for introspection and error messages
    fn name(&self) -> &str;

    /// Check if this behavior applies to a given value type
    ///
    /// # Arguments
    /// * `value` - The value to check
    ///
    /// # Returns
    /// `true` if the behavior should run on this value, `false` otherwise
    ///
    /// # Default Behavior
    /// Returns `true` for all values (universal application)
    fn applies_to(&self, _value: &Value) -> bool {
        true  // Default: applies to all values
    }
}

/// Specification for a behavior that can be stored and cloned
///
/// BehaviorSpec is the clonable, serializable representation of a behavior.
/// Use `instantiate()` to create an actual Behavior trait object.
#[derive(Debug, Clone, PartialEq)]
pub enum BehaviorSpec {
    // ============================================================================
    // Standard value transformations (Sub-phase 7.2)
    // ============================================================================

    /// Transform `none` values to `0`
    NoneToZero,

    /// Transform `none` values to empty string `""`
    NoneToEmpty,

    /// Transform negative numbers to their absolute value
    Positive,

    /// Round numbers to nearest integer
    RoundToInt,

    // ============================================================================
    // String transformations (Sub-phase 7.2)
    // ============================================================================

    /// Convert strings to uppercase
    Uppercase,

    /// Convert strings to lowercase
    Lowercase,

    // ============================================================================
    // Validation/clamping (Sub-phase 7.2)
    // ============================================================================

    /// Clamp numbers to a specified range [min, max]
    ValidateRange { min: f64, max: f64 },

    // ============================================================================
    // Mapping behaviors (Sub-phase 7.3)
    // ============================================================================

    /// Map values using a hash table, with default for unmapped values
    Mapping {
        mapping: HashMap<String, Value>,
        default: Value,
    },

    // ============================================================================
    // Custom function behavior (Sub-phase 7.4)
    // ============================================================================

    /// User-defined transformation function
    CustomFunction {
        function: Value,  // Must be Value::Function
    },

    // ============================================================================
    // Conditional behavior (Sub-phase 7.4)
    // ============================================================================

    /// Apply transformation based on a condition
    Conditional {
        condition: Value,   // Predicate function
        transform: Value,   // Transform function
        fallback: Option<Value>,  // Optional fallback function
    },

    // ============================================================================
    // Ordering behavior (Sub-phase 7.5)
    // ============================================================================

    /// Maintain sorted order using comparison function
    Ordering {
        compare_fn: Option<Value>,  // Optional comparison function
                                     // None = use default ordering
    },
}

impl BehaviorSpec {
    /// Convert specification to actual Behavior instance
    ///
    /// # Returns
    /// A boxed Behavior trait object
    ///
    /// # Implementation Note
    /// Concrete implementations are in Sub-phases 7.2-7.5.
    /// For Sub-phase 7.1 (framework), we provide stub implementations.
    pub fn instantiate(&self) -> Box<dyn Behavior> {
        match self {
            BehaviorSpec::NoneToZero => Box::new(NoneToZeroBehavior),
            BehaviorSpec::NoneToEmpty => Box::new(NoneToEmptyBehavior),
            BehaviorSpec::Positive => Box::new(PositiveBehavior),
            BehaviorSpec::RoundToInt => Box::new(RoundToIntBehavior),
            BehaviorSpec::Uppercase => Box::new(UppercaseBehavior),
            BehaviorSpec::Lowercase => Box::new(LowercaseBehavior),
            BehaviorSpec::ValidateRange { min, max } => {
                Box::new(ValidateRangeBehavior { min: *min, max: *max })
            }
            BehaviorSpec::Mapping { mapping, default } => {
                Box::new(MappingBehavior {
                    mapping: mapping.clone(),
                    default: default.clone(),
                })
            }
            BehaviorSpec::CustomFunction { function } => {
                Box::new(CustomFunctionBehavior {
                    function: function.clone(),
                })
            }
            BehaviorSpec::Conditional { condition, transform, fallback } => {
                Box::new(ConditionalBehavior {
                    condition: condition.clone(),
                    transform: transform.clone(),
                    fallback: fallback.clone(),
                })
            }
            BehaviorSpec::Ordering { compare_fn } => {
                Box::new(OrderingBehavior {
                    compare_fn: compare_fn.clone(),
                })
            }
        }
    }

    /// Get the name of this behavior
    ///
    /// Used for introspection and debugging
    pub fn name(&self) -> &str {
        match self {
            BehaviorSpec::NoneToZero => "none_to_zero",
            BehaviorSpec::NoneToEmpty => "none_to_empty",
            BehaviorSpec::Positive => "positive",
            BehaviorSpec::RoundToInt => "round_to_int",
            BehaviorSpec::Uppercase => "uppercase",
            BehaviorSpec::Lowercase => "lowercase",
            BehaviorSpec::ValidateRange { .. } => "validate_range",
            BehaviorSpec::Mapping { .. } => "mapping",
            BehaviorSpec::CustomFunction { .. } => "custom_function",
            BehaviorSpec::Conditional { .. } => "conditional",
            BehaviorSpec::Ordering { .. } => "ordering",
        }
    }

    /// Create BehaviorSpec from symbol (for Graphoid syntax)
    ///
    /// # Arguments
    /// * `sym` - Symbol name (without leading colon)
    ///
    /// # Returns
    /// `Some(BehaviorSpec)` if recognized, `None` otherwise
    ///
    /// # Example
    /// ```
    /// use graphoid::graph::behaviors::BehaviorSpec;
    /// let spec = BehaviorSpec::from_symbol("none_to_zero");
    /// assert_eq!(spec, Some(BehaviorSpec::NoneToZero));
    /// ```
    pub fn from_symbol(sym: &str) -> Option<BehaviorSpec> {
        match sym {
            "none_to_zero" => Some(BehaviorSpec::NoneToZero),
            "none_to_empty" => Some(BehaviorSpec::NoneToEmpty),
            "positive" => Some(BehaviorSpec::Positive),
            "round_to_int" => Some(BehaviorSpec::RoundToInt),
            "uppercase" => Some(BehaviorSpec::Uppercase),
            "lowercase" => Some(BehaviorSpec::Lowercase),
            _ => None,
        }
    }
}

/// Instance of a behavior with retroactive policy
///
/// Similar to RuleInstance but for transformations.
/// Wraps a BehaviorSpec with configuration for how to handle existing values.
#[derive(Debug, Clone, PartialEq)]
pub struct BehaviorInstance {
    /// The behavior specification
    pub spec: BehaviorSpec,

    /// How to handle existing values when behavior is added
    pub retroactive_policy: RetroactivePolicy,
}

impl BehaviorInstance {
    /// Create new behavior instance with default retroactive policy (Clean)
    ///
    /// # Arguments
    /// * `spec` - The behavior specification
    ///
    /// # Returns
    /// A new BehaviorInstance with RetroactivePolicy::Clean
    pub fn new(spec: BehaviorSpec) -> Self {
        BehaviorInstance {
            spec,
            retroactive_policy: RetroactivePolicy::Clean,
        }
    }

    /// Create behavior instance with specific retroactive policy
    ///
    /// # Arguments
    /// * `spec` - The behavior specification
    /// * `policy` - The retroactive policy to use
    ///
    /// # Returns
    /// A new BehaviorInstance with the specified policy
    pub fn with_policy(spec: BehaviorSpec, policy: RetroactivePolicy) -> Self {
        BehaviorInstance {
            spec,
            retroactive_policy: policy,
        }
    }
}

// ============================================================================
// Behavior Application Logic
// ============================================================================

/// Apply a sequence of behaviors to a value
///
/// Behaviors are applied in order: first in the list = first applied.
/// Each behavior can optionally skip values using `applies_to()`.
///
/// # Arguments
/// * `value` - The value to transform
/// * `behaviors` - The behaviors to apply, in order
///
/// # Returns
/// The transformed value, or an error if any transformation fails
///
/// # Example
/// ```
/// use graphoid::graph::behaviors::{BehaviorSpec, BehaviorInstance, apply_behaviors};
/// use graphoid::values::Value;
///
/// let value = Value::None;
/// let behaviors = vec![
///     BehaviorInstance::new(BehaviorSpec::NoneToZero),
///     BehaviorInstance::new(BehaviorSpec::Positive),
/// ];
/// let result = apply_behaviors(value, &behaviors).unwrap();
/// // none → 0 → 0 (already positive)
/// assert_eq!(result, Value::Number(0.0));
/// ```
pub fn apply_behaviors(
    value: Value,
    behaviors: &[BehaviorInstance],
) -> Result<Value, GraphoidError> {
    let mut current = value;

    for behavior_instance in behaviors {
        let behavior = behavior_instance.spec.instantiate();

        // Only apply if behavior applies to this value type
        if behavior.applies_to(&current) {
            current = behavior.transform(&current)?;
        }
    }

    Ok(current)
}

/// Apply behaviors retroactively to all existing values in a list
///
/// Used when a new behavior is added to a list with existing values.
/// Respects the RetroactivePolicy setting.
///
/// # Arguments
/// * `list` - The list to apply behaviors to
/// * `new_behavior` - The behavior to apply
///
/// # Returns
/// `Ok(())` if successful, or an error if RetroactivePolicy::Enforce fails
///
/// # Retroactive Policies
///
/// - **Clean**: Transform all existing values that apply
/// - **Warn**: Keep existing data, print warnings
/// - **Enforce**: Error if any values would be transformed
/// - **Ignore**: Don't check or transform existing values
pub fn apply_retroactive_to_list(
    list: &mut List,
    new_behavior: &BehaviorInstance,
) -> Result<(), GraphoidError> {
    let behavior = new_behavior.spec.instantiate();
    let elements = list.to_vec();

    match new_behavior.retroactive_policy {
        RetroactivePolicy::Clean => {
            // Transform all existing values that apply
            for (index, element) in elements.iter().enumerate() {
                if behavior.applies_to(element) {
                    let transformed = behavior.transform(element)?;
                    let node_id = format!("node_{}", index);
                    if let Some(node) = list.graph.nodes.get_mut(&node_id) {
                        node.value = transformed;
                    }
                }
            }
        }
        RetroactivePolicy::Warn => {
            // Keep existing data, warn about values that would be transformed
            let mut warned = false;
            for (index, element) in elements.iter().enumerate() {
                if behavior.applies_to(element) {
                    let transformed = behavior.transform(element)?;
                    if transformed != *element {
                        eprintln!(
                            "WARNING: Behavior '{}' would transform element at index {} from {:?} to {:?}",
                            behavior.name(), index, element, transformed
                        );
                        warned = true;
                    }
                }
            }
            if warned {
                eprintln!("WARNING: Existing values NOT transformed. Use RetroactivePolicy::Clean to transform.");
            }
        }
        RetroactivePolicy::Enforce => {
            // Error if any values would be transformed
            for (index, element) in elements.iter().enumerate() {
                if behavior.applies_to(element) {
                    let transformed = behavior.transform(element)?;
                    if transformed != *element {
                        return Err(GraphoidError::runtime(format!(
                            "Behavior '{}' would transform existing element at index {} from {:?} to {:?}. \
                             Cannot add behavior with RetroactivePolicy::Enforce.",
                            behavior.name(), index, element, transformed
                        )));
                    }
                }
            }
        }
        RetroactivePolicy::Ignore => {
            // Don't check or transform existing values
            // Only new values will be transformed
        }
    }

    Ok(())
}

/// Apply behaviors retroactively to all existing values in a hash
///
/// Used when a new behavior is added to a hash with existing values.
/// Respects the RetroactivePolicy setting.
///
/// # Arguments
/// * `hash` - The hash to apply behaviors to
/// * `new_behavior` - The behavior to apply
///
/// # Returns
/// `Ok(())` if successful, or an error if RetroactivePolicy::Enforce fails
///
/// # Retroactive Policies
///
/// - **Clean**: Transform all existing values that apply
/// - **Warn**: Keep existing data, print warnings
/// - **Enforce**: Error if any values would be transformed
/// - **Ignore**: Don't check or transform existing values
pub fn apply_retroactive_to_hash(
    hash: &mut crate::values::Hash,
    new_behavior: &BehaviorInstance,
) -> Result<(), GraphoidError> {
    let behavior = new_behavior.spec.instantiate();
    let keys: Vec<String> = hash.keys();

    match new_behavior.retroactive_policy {
        RetroactivePolicy::Clean => {
            // Transform all existing values that apply
            for key in keys {
                if let Some(value) = hash.get(&key).cloned() {
                    if behavior.applies_to(&value) {
                        let transformed = behavior.transform(&value)?;
                        // Update the value in the graph directly
                        if let Some(node) = hash.graph.nodes.get_mut(&key) {
                            node.value = transformed;
                        }
                    }
                }
            }
        }
        RetroactivePolicy::Warn => {
            // Keep existing data, warn about values that would be transformed
            let mut warned = false;
            for key in keys {
                if let Some(value) = hash.get(&key) {
                    if behavior.applies_to(value) {
                        let transformed = behavior.transform(value)?;
                        if transformed != *value {
                            eprintln!(
                                "WARNING: Behavior '{}' would transform value for key '{}' from {:?} to {:?}",
                                behavior.name(), key, value, transformed
                            );
                            warned = true;
                        }
                    }
                }
            }
            if warned {
                eprintln!("WARNING: Existing values NOT transformed. Use RetroactivePolicy::Clean to transform.");
            }
        }
        RetroactivePolicy::Enforce => {
            // Error if any values would be transformed
            for key in keys {
                if let Some(value) = hash.get(&key) {
                    if behavior.applies_to(value) {
                        let transformed = behavior.transform(value)?;
                        if transformed != *value {
                            return Err(GraphoidError::runtime(format!(
                                "Behavior '{}' would transform existing value for key '{}' from {:?} to {:?}. \
                                 Cannot add behavior with RetroactivePolicy::Enforce.",
                                behavior.name(), key, value, transformed
                            )));
                        }
                    }
                }
            }
        }
        RetroactivePolicy::Ignore => {
            // Don't check or transform existing values
            // Only new values will be transformed
        }
    }

    Ok(())
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Convert a Value to a string key for mapping lookups
///
/// This function provides a consistent way to convert any Value to a String
/// that can be used as a key in HashMap lookups for mapping behaviors.
///
/// # Arguments
/// * `value` - The value to convert
///
/// # Returns
/// A String representation suitable for use as a hash key
fn value_to_key(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => {
            // For integers, use simple format; for floats, use full precision
            if n.fract() == 0.0 {
                format!("{}", *n as i64)
            } else {
                n.to_string()
            }
        }
        Value::Symbol(s) => s.clone(),
        Value::Boolean(b) => b.to_string(),
        Value::None => "none".to_string(),
        // For complex types, use Debug representation
        _ => format!("{:?}", value),
    }
}

// ============================================================================
// Behavior Implementations
// ============================================================================

#[derive(Debug)]
pub struct NoneToZeroBehavior;

impl Behavior for NoneToZeroBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        match value {
            Value::None => Ok(Value::Number(0.0)),
            other => Ok(other.clone()),
        }
    }

    fn name(&self) -> &str {
        "none_to_zero"
    }

    fn applies_to(&self, value: &Value) -> bool {
        matches!(value, Value::None)
    }
}

impl Rule for NoneToZeroBehavior {
    fn name(&self) -> &str {
        "none_to_zero"
    }

    fn is_transformation_rule(&self) -> bool {
        true
    }

    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        // Delegate to Behavior trait implementation
        Behavior::transform(self, value)
    }

    fn validate(&self, _graph: &Graph, _context: &RuleContext) -> Result<(), GraphoidError> {
        // Transformation rules don't validate - they transform
        Ok(())
    }

    fn should_run_on(&self, _operation: &GraphOperation) -> bool {
        // Transformation rules are applied during value insertion, not graph operations
        false
    }
}

#[derive(Debug)]
pub struct NoneToEmptyBehavior;

impl Behavior for NoneToEmptyBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        match value {
            Value::None => Ok(Value::String(String::new())),
            other => Ok(other.clone()),
        }
    }

    fn name(&self) -> &str {
        "none_to_empty"
    }

    fn applies_to(&self, value: &Value) -> bool {
        matches!(value, Value::None)
    }
}

impl Rule for NoneToEmptyBehavior {
    fn name(&self) -> &str {
        "none_to_empty"
    }

    fn is_transformation_rule(&self) -> bool {
        true
    }

    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        Behavior::transform(self, value)
    }

    fn validate(&self, _graph: &Graph, _context: &RuleContext) -> Result<(), GraphoidError> {
        Ok(())
    }

    fn should_run_on(&self, _operation: &GraphOperation) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct PositiveBehavior;

impl Behavior for PositiveBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        match value {
            Value::Number(n) if *n < 0.0 => Ok(Value::Number(n.abs())),
            other => Ok(other.clone()),
        }
    }

    fn name(&self) -> &str {
        "positive"
    }

    fn applies_to(&self, value: &Value) -> bool {
        matches!(value, Value::Number(_))
    }
}

impl Rule for PositiveBehavior {
    fn name(&self) -> &str {
        "positive"
    }

    fn is_transformation_rule(&self) -> bool {
        true
    }

    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        Behavior::transform(self, value)
    }

    fn validate(&self, _graph: &Graph, _context: &RuleContext) -> Result<(), GraphoidError> {
        Ok(())
    }

    fn should_run_on(&self, _operation: &GraphOperation) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct RoundToIntBehavior;

impl Behavior for RoundToIntBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        match value {
            Value::Number(n) => Ok(Value::Number(n.round())),
            other => Ok(other.clone()),
        }
    }

    fn name(&self) -> &str {
        "round_to_int"
    }

    fn applies_to(&self, value: &Value) -> bool {
        matches!(value, Value::Number(_))
    }
}

impl Rule for RoundToIntBehavior {
    fn name(&self) -> &str {
        "round_to_int"
    }

    fn is_transformation_rule(&self) -> bool {
        true
    }

    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        Behavior::transform(self, value)
    }

    fn validate(&self, _graph: &Graph, _context: &RuleContext) -> Result<(), GraphoidError> {
        Ok(())
    }

    fn should_run_on(&self, _operation: &GraphOperation) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct UppercaseBehavior;

impl Behavior for UppercaseBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        match value {
            Value::String(s) => Ok(Value::String(s.to_uppercase())),
            other => Ok(other.clone()),
        }
    }

    fn name(&self) -> &str {
        "uppercase"
    }

    fn applies_to(&self, value: &Value) -> bool {
        matches!(value, Value::String(_))
    }
}

impl Rule for UppercaseBehavior {
    fn name(&self) -> &str {
        "uppercase"
    }

    fn is_transformation_rule(&self) -> bool {
        true
    }

    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        Behavior::transform(self, value)
    }

    fn validate(&self, _graph: &Graph, _context: &RuleContext) -> Result<(), GraphoidError> {
        Ok(())
    }

    fn should_run_on(&self, _operation: &GraphOperation) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct LowercaseBehavior;

impl Behavior for LowercaseBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        match value {
            Value::String(s) => Ok(Value::String(s.to_lowercase())),
            other => Ok(other.clone()),
        }
    }

    fn name(&self) -> &str {
        "lowercase"
    }

    fn applies_to(&self, value: &Value) -> bool {
        matches!(value, Value::String(_))
    }
}

impl Rule for LowercaseBehavior {
    fn name(&self) -> &str {
        "lowercase"
    }

    fn is_transformation_rule(&self) -> bool {
        true
    }

    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        Behavior::transform(self, value)
    }

    fn validate(&self, _graph: &Graph, _context: &RuleContext) -> Result<(), GraphoidError> {
        Ok(())
    }

    fn should_run_on(&self, _operation: &GraphOperation) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct ValidateRangeBehavior {
    pub min: f64,
    pub max: f64,
}

impl Behavior for ValidateRangeBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        match value {
            Value::Number(n) => {
                let clamped = n.clamp(self.min, self.max);
                Ok(Value::Number(clamped))
            }
            other => Ok(other.clone()),
        }
    }

    fn name(&self) -> &str {
        "validate_range"
    }

    fn applies_to(&self, value: &Value) -> bool {
        matches!(value, Value::Number(_))
    }
}

impl Rule for ValidateRangeBehavior {
    fn name(&self) -> &str {
        "validate_range"
    }

    fn is_transformation_rule(&self) -> bool {
        true
    }

    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        Behavior::transform(self, value)
    }

    fn validate(&self, _graph: &Graph, _context: &RuleContext) -> Result<(), GraphoidError> {
        Ok(())
    }

    fn should_run_on(&self, _operation: &GraphOperation) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct MappingBehavior {
    pub mapping: HashMap<String, Value>,
    pub default: Value,
}

impl Behavior for MappingBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        // Convert value to string key for lookup
        let key = value_to_key(value);

        // Look up in mapping, use default if not found
        if let Some(mapped_value) = self.mapping.get(&key) {
            Ok(mapped_value.clone())
        } else {
            Ok(self.default.clone())
        }
    }

    fn name(&self) -> &str {
        "mapping"
    }

    fn applies_to(&self, _value: &Value) -> bool {
        // Applies to all values (mapping can transform any type)
        true
    }
}

impl Rule for MappingBehavior {
    fn name(&self) -> &str {
        "mapping"
    }

    fn is_transformation_rule(&self) -> bool {
        true
    }

    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        Behavior::transform(self, value)
    }

    fn validate(&self, _graph: &Graph, _context: &RuleContext) -> Result<(), GraphoidError> {
        Ok(())
    }

    fn should_run_on(&self, _operation: &GraphOperation) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct CustomFunctionBehavior {
    pub function: Value,
}

impl Behavior for CustomFunctionBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        // Stub implementation for Sub-phase 7.1
        // Full implementation in Sub-phase 7.4
        Ok(value.clone())
    }

    fn name(&self) -> &str {
        "custom_function"
    }
}

impl Rule for CustomFunctionBehavior {
    fn name(&self) -> &str {
        "custom_function"
    }

    fn is_transformation_rule(&self) -> bool {
        true
    }

    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        Behavior::transform(self, value)
    }

    fn validate(&self, _graph: &Graph, _context: &RuleContext) -> Result<(), GraphoidError> {
        Ok(())
    }

    fn should_run_on(&self, _operation: &GraphOperation) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct ConditionalBehavior {
    pub condition: Value,
    pub transform: Value,
    pub fallback: Option<Value>,
}

impl Behavior for ConditionalBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        // Stub implementation for Sub-phase 7.1
        // Full implementation in Sub-phase 7.4
        Ok(value.clone())
    }

    fn name(&self) -> &str {
        "conditional"
    }
}

impl Rule for ConditionalBehavior {
    fn name(&self) -> &str {
        "conditional"
    }

    fn is_transformation_rule(&self) -> bool {
        true
    }

    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        Behavior::transform(self, value)
    }

    fn validate(&self, _graph: &Graph, _context: &RuleContext) -> Result<(), GraphoidError> {
        Ok(())
    }

    fn should_run_on(&self, _operation: &GraphOperation) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct OrderingBehavior {
    pub compare_fn: Option<Value>,
}

impl Behavior for OrderingBehavior {
    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        // Stub implementation for Sub-phase 7.1
        // Full implementation in Sub-phase 7.5
        Ok(value.clone())
    }

    fn name(&self) -> &str {
        "ordering"
    }
}

impl Rule for OrderingBehavior {
    fn name(&self) -> &str {
        "ordering"
    }

    fn is_transformation_rule(&self) -> bool {
        true
    }

    fn transform(&self, value: &Value) -> Result<Value, GraphoidError> {
        Behavior::transform(self, value)
    }

    fn validate(&self, _graph: &Graph, _context: &RuleContext) -> Result<(), GraphoidError> {
        Ok(())
    }

    fn should_run_on(&self, _operation: &GraphOperation) -> bool {
        false
    }
}

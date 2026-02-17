use std::collections::HashMap;
use std::time::Instant;
use crate::values::{Value, Function};

/// A node in the function graph representing a function definition
#[derive(Debug, Clone)]
pub struct FunctionNode {
    /// Unique identifier for this function node
    pub node_id: String,

    /// The actual function data
    pub function: Function,

    /// Functions this function calls (outgoing edges)
    pub outgoing_calls: Vec<String>,

    /// Functions that call this function (incoming edges)
    pub incoming_calls: Vec<String>,

    /// Number of times this function has been called
    pub call_count: usize,

    /// Total execution time across all calls (in seconds)
    pub total_time: f64,

    /// Variables captured by this function (for closures)
    /// Format: (variable_name, variable_node_id_in_namespace)
    pub captured_vars: Vec<(String, String)>,

    /// Timestamp when function was defined (seconds since epoch)
    pub defined_at: f64,
}

/// An edge representing a function call
#[derive(Debug, Clone)]
pub struct CallEdge {
    /// Caller function node ID
    pub from: String,

    /// Callee function node ID
    pub to: String,

    /// Edge type (Call, Captures, PassedTo, Imports)
    pub edge_type: FunctionEdgeType,

    /// Arguments passed (for debugging/profiling)
    pub arguments: Vec<Value>,

    /// Return value (Some after call completes, None during call)
    pub return_value: Option<Value>,

    /// When the call started (seconds since epoch)
    pub start_time: f64,

    /// How long the call took in seconds (Some after completion, None during call)
    pub duration: Option<f64>,
}

/// Type of relationship between function nodes
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionEdgeType {
    /// Direct function call (A calls B)
    Call,

    /// Closure capture (lambda captures variable)
    Captures,

    /// Higher-order function (A passes B as argument to C)
    PassedTo,

    /// Module import
    Imports,

    /// Exception propagation (function exited via exception to caller)
    ExceptionPropagation,
}

/// The global function graph tracking all functions and calls
pub struct FunctionGraph {
    /// All function nodes indexed by node_id
    nodes: HashMap<String, FunctionNode>,

    /// All call edges
    edges: Vec<CallEdge>,

    /// Current call stack as a path through the graph
    /// Format: (function_node_id, arguments, start_instant)
    call_path: Vec<(String, Vec<Value>, Instant)>,

    /// Counter for generating unique node IDs
    next_node_id: usize,

    /// Whether to track profiling data (performance overhead)
    profiling_enabled: bool,
}

impl FunctionGraph {
    /// Create a new function graph
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            call_path: Vec::new(),
            next_node_id: 0,
            profiling_enabled: false,
        }
    }

    /// Enable or disable profiling
    pub fn set_profiling(&mut self, enabled: bool) {
        self.profiling_enabled = enabled;
    }

    /// Register a new function definition
    /// Returns the node_id for this function
    pub fn register_function(&mut self, function: Function) -> String {
        // Generate unique node ID
        let node_id = if let Some(name) = &function.name {
            format!("fn_{}_{}", name, self.next_node_id)
        } else {
            format!("lambda_{}", self.next_node_id)
        };
        self.next_node_id += 1;

        // Create function node
        let node = FunctionNode {
            node_id: node_id.clone(),
            function,
            outgoing_calls: Vec::new(),
            incoming_calls: Vec::new(),
            call_count: 0,
            total_time: 0.0,
            captured_vars: Vec::new(),
            defined_at: current_time(),
        };

        self.nodes.insert(node_id.clone(), node);
        node_id
    }

    /// Push a function call onto the stack
    pub fn push_call(&mut self, func_id: String, args: Vec<Value>) {
        let start_instant = Instant::now();

        // Update call count
        if let Some(node) = self.nodes.get_mut(&func_id) {
            node.call_count += 1;
        }

        // Add to call path (this is the graph path!)
        self.call_path.push((func_id.clone(), args.clone(), start_instant));

        // Create call edge if there's a caller (previous function in path)
        if self.call_path.len() > 1 {
            let caller_id = self.call_path[self.call_path.len() - 2].0.clone();
            self.add_call_edge(caller_id, func_id, args);
        }
    }

    /// Pop a function return from the stack
    pub fn pop_call(&mut self, return_value: Value) {
        if let Some((func_id, _, start_instant)) = self.call_path.pop() {
            let duration = start_instant.elapsed().as_secs_f64();

            // Update total time if profiling enabled
            if self.profiling_enabled {
                if let Some(node) = self.nodes.get_mut(&func_id) {
                    node.total_time += duration;
                }

                // Update most recent call edge with return value and duration
                if let Some(edge) = self.edges.iter_mut().rev().find(|e| e.to == func_id && e.duration.is_none()) {
                    edge.return_value = Some(return_value);
                    edge.duration = Some(duration);
                }
            }
        }
    }

    /// Pop a function that exited via exception, creating an ExceptionPropagation edge
    /// from this function to its caller (if any).
    pub fn pop_call_exception(&mut self, error_type: String) {
        if let Some((func_id, _, start_instant)) = self.call_path.pop() {
            let duration = start_instant.elapsed().as_secs_f64();

            // Update total time if profiling enabled
            if self.profiling_enabled {
                if let Some(node) = self.nodes.get_mut(&func_id) {
                    node.total_time += duration;
                }
            }

            // Create exception propagation edge to caller (if any)
            if let Some((caller_id, _, _)) = self.call_path.last() {
                let edge = CallEdge {
                    from: func_id,
                    to: caller_id.clone(),
                    edge_type: FunctionEdgeType::ExceptionPropagation,
                    arguments: vec![Value::string(error_type)],
                    return_value: None,
                    start_time: current_time(),
                    duration: Some(duration),
                };
                self.edges.push(edge);
            }
        }
    }

    /// Add a call edge between two functions
    fn add_call_edge(&mut self, from: String, to: String, args: Vec<Value>) {
        // Update outgoing/incoming call lists
        if let Some(caller) = self.nodes.get_mut(&from) {
            if !caller.outgoing_calls.contains(&to) {
                caller.outgoing_calls.push(to.clone());
            }
        }

        if let Some(callee) = self.nodes.get_mut(&to) {
            if !callee.incoming_calls.contains(&from) {
                callee.incoming_calls.push(from.clone());
            }
        }

        // Create edge
        let edge = CallEdge {
            from,
            to,
            edge_type: FunctionEdgeType::Call,
            arguments: args,
            return_value: None,
            start_time: current_time(),
            duration: None,
        };

        self.edges.push(edge);
    }

    /// Add a closure capture edge
    pub fn add_capture_edge(&mut self, func_id: String, var_name: String, var_node_id: String) {
        // Update function node's captured vars
        if let Some(node) = self.nodes.get_mut(&func_id) {
            node.captured_vars.push((var_name.clone(), var_node_id.clone()));
        }

        // Create capture edge
        let edge = CallEdge {
            from: func_id,
            to: var_node_id,
            edge_type: FunctionEdgeType::Captures,
            arguments: Vec::new(),
            return_value: None,
            start_time: current_time(),
            duration: None,
        };

        self.edges.push(edge);
    }

    /// Get current call depth (length of the call path)
    pub fn call_depth(&self) -> usize {
        self.call_path.len()
    }

    /// Get current call path as function names
    pub fn current_path(&self) -> Vec<String> {
        self.call_path.iter()
            .filter_map(|(id, _, _)| {
                self.nodes.get(id).and_then(|n| {
                    n.function.name.clone().or(Some(id.clone()))
                })
            })
            .collect()
    }

    /// Get current call path as node IDs
    pub fn current_path_ids(&self) -> Vec<String> {
        self.call_path.iter()
            .map(|(id, _, _)| id.clone())
            .collect()
    }

    /// Check if a function is recursive (has edge to itself)
    pub fn is_recursive(&self, func_id: &str) -> bool {
        if let Some(node) = self.nodes.get(func_id) {
            node.outgoing_calls.contains(&func_id.to_string())
        } else {
            false
        }
    }

    /// Find all recursive functions
    pub fn find_recursive_functions(&self) -> Vec<String> {
        self.nodes.keys()
            .filter(|id| self.is_recursive(id))
            .cloned()
            .collect()
    }

    /// Get all functions called by a given function (outgoing edges)
    pub fn get_callees(&self, func_id: &str) -> Vec<String> {
        self.nodes.get(func_id)
            .map(|n| n.outgoing_calls.clone())
            .unwrap_or_default()
    }

    /// Get all functions that call a given function (incoming edges)
    pub fn get_callers(&self, func_id: &str) -> Vec<String> {
        self.nodes.get(func_id)
            .map(|n| n.incoming_calls.clone())
            .unwrap_or_default()
    }

    /// Get function node by ID
    pub fn get_function(&self, func_id: &str) -> Option<&FunctionNode> {
        self.nodes.get(func_id)
    }

    /// Get function node by name (returns first match)
    pub fn get_function_by_name(&self, name: &str) -> Option<&FunctionNode> {
        self.nodes.values()
            .find(|n| n.function.name.as_deref() == Some(name))
    }

    /// Get all function nodes
    pub fn get_all_functions(&self) -> Vec<&FunctionNode> {
        self.nodes.values().collect()
    }

    /// Get all function nodes with their IDs
    pub fn get_all_function_nodes(&self) -> impl Iterator<Item = (&String, &FunctionNode)> {
        self.nodes.iter()
    }

    /// Get user-defined functions (excludes __toplevel__ synthetic function)
    pub fn get_user_functions(&self) -> Vec<&FunctionNode> {
        self.nodes.values()
            .filter(|n| n.function.name.as_deref() != Some("__toplevel__"))
            .collect()
    }

    /// Get all call edges
    pub fn get_all_edges(&self) -> &[CallEdge] {
        &self.edges
    }

    /// Get total number of function calls across entire program
    pub fn total_calls(&self) -> usize {
        self.nodes.values().map(|n| n.call_count).sum()
    }

    /// Get total number of user function calls (excludes __toplevel__)
    pub fn user_calls(&self) -> usize {
        self.nodes.values()
            .filter(|n| n.function.name.as_deref() != Some("__toplevel__"))
            .map(|n| n.call_count)
            .sum()
    }

    /// Get hotspots (functions with highest total time)
    pub fn get_hotspots(&self, limit: usize) -> Vec<(String, f64)> {
        let mut functions: Vec<_> = self.nodes.values()
            .map(|n| (n.function.name.clone().unwrap_or_else(|| n.node_id.clone()), n.total_time))
            .collect();

        functions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        functions.truncate(limit);
        functions
    }

    /// Get profiling report as a formatted string
    pub fn profiling_report(&self) -> String {
        let mut report = String::new();
        report.push_str("Function Profile Report\n");
        report.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        report.push_str(&format!("{:<20} {:<8} {:<12} {:<12}\n", "Function", "Calls", "Total Time", "Avg Time"));
        report.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        let mut functions: Vec<_> = self.nodes.values().collect();
        functions.sort_by(|a, b| b.total_time.partial_cmp(&a.total_time).unwrap());

        for node in functions {
            let name = node.function.name.clone().unwrap_or_else(|| node.node_id.clone());
            let avg_time = if node.call_count > 0 {
                node.total_time / node.call_count as f64
            } else {
                0.0
            };

            report.push_str(&format!(
                "{:<20} {:<8} {:<12.3}s {:<12.6}s\n",
                name,
                node.call_count,
                node.total_time,
                avg_time
            ));
        }

        report.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        // Add hotspots section
        let hotspots = self.get_hotspots(5);
        if !hotspots.is_empty() && hotspots[0].1 > 1.0 {
            report.push_str("\nHotspots (>1s): ");
            let hotspot_names: Vec<String> = hotspots.iter()
                .filter(|(_, time)| *time > 1.0)
                .map(|(name, time)| format!("{} ({:.3}s)", name, time))
                .collect();
            report.push_str(&hotspot_names.join(", "));
            report.push('\n');
        }

        report
    }

    /// Get captured variables for a function (closures)
    pub fn get_captured_vars(&self, func_id: &str) -> Vec<(String, String)> {
        self.nodes.get(func_id)
            .map(|n| n.captured_vars.clone())
            .unwrap_or_default()
    }

    /// Find functions that are never called (dead code)
    pub fn find_dead_functions(&self) -> Vec<String> {
        self.nodes.values()
            .filter(|n| n.call_count == 0 && n.incoming_calls.is_empty())
            .map(|n| n.function.name.clone().unwrap_or_else(|| n.node_id.clone()))
            .collect()
    }

    /// Clear all profiling data (reset counters and times)
    pub fn clear_profiling_data(&mut self) {
        for node in self.nodes.values_mut() {
            node.call_count = 0;
            node.total_time = 0.0;
        }
        self.edges.clear();
    }
}

impl Default for FunctionGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Get current time as f64 (seconds since UNIX epoch)
fn current_time() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

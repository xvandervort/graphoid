use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use crate::error::{Result, GraphoidError, SourcePosition};
use crate::execution::Environment;
use crate::stdlib::NativeModule;

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub name: String,
    pub alias: Option<String>,
    pub namespace: Environment,
    pub file_path: PathBuf,
    pub config: Option<ConfigScope>,
    pub private_symbols: std::collections::HashSet<String>,  // Phase 10: Track private symbols
    pub exports: Vec<String>,  // Phase 17: Public symbol names (all bindings minus private_symbols)
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConfigScope {
    pub decimal_places: Option<u8>,
    pub error_mode: Option<ErrorMode>,
    pub bounds_checking: Option<BoundsMode>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorMode {
    Strict,
    Lenient,
    Collect,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoundsMode {
    Strict,
    Lenient,
}

pub struct ModuleManager {
    /// Loaded modules: name → Module
    modules: HashMap<String, Module>,

    /// Import stack for cycle detection
    import_stack: Vec<PathBuf>,

    /// Currently being loaded (for cycle detection)
    loading: HashSet<PathBuf>,

    /// Module search paths
    search_paths: Vec<PathBuf>,

    /// Native modules: name → NativeModule
    native_modules: HashMap<String, Box<dyn NativeModule>>,

    /// Phase 17: Module dependency graph — module_key → set of imported module_keys
    dependencies: HashMap<String, HashSet<String>>,

    /// Phase 17: Reverse dependencies — module_key → set of modules that import it
    dependents: HashMap<String, HashSet<String>>,
}

impl ModuleManager {
    pub fn new() -> Self {
        let mut manager = Self {
            modules: HashMap::new(),
            import_stack: Vec::new(),
            loading: HashSet::new(),
            search_paths: vec![
                PathBuf::from("src"),
                PathBuf::from("lib"),
                Self::get_stdlib_path(),
            ],
            native_modules: HashMap::new(),
            dependencies: HashMap::new(),
            dependents: HashMap::new(),
        };

        // Register built-in native modules
        manager.register_native_modules();
        manager
    }

    /// Register all built-in native modules
    fn register_native_modules(&mut self) {
        use crate::stdlib::{ConstantsModule, RandomModule, CryptoModule, OSModule, FSModule, NetModule};

        self.register_native_module(Box::new(ConstantsModule));
        self.register_native_module(Box::new(RandomModule::new()));
        self.register_native_module(Box::new(CryptoModule));
        self.register_native_module(Box::new(OSModule));
        self.register_native_module(Box::new(FSModule));
        self.register_native_module(Box::new(NetModule));
    }

    /// Register a native module
    pub fn register_native_module(&mut self, module: Box<dyn NativeModule>) {
        let name = module.name().to_string();
        self.native_modules.insert(name, module);
    }

    /// Check if a module name refers to a native module
    pub fn is_native_module(&self, name: &str) -> bool {
        self.native_modules.contains_key(name)
    }

    /// Get a native module's environment (constants and functions as bindings)
    pub fn get_native_module_env(&self, name: &str) -> Option<(Environment, Option<String>)> {
        self.native_modules.get(name).map(|module| {
            let mut env = Environment::new();

            // Add all constants
            for (const_name, value) in module.constants() {
                env.define(const_name, value);
            }

            // Add all functions wrapped as NativeFunction values
            for (func_name, func) in module.functions() {
                use crate::values::{Value, ValueKind};
                let func_value = Value {
                    kind: ValueKind::NativeFunction(func),
                    frozen: false,
                };
                env.define(func_name, func_value);
            }

            // Return environment and alias
            (env, module.alias().map(|s| s.to_string()))
        })
    }

    pub fn search_paths(&self) -> &[PathBuf] {
        &self.search_paths
    }

    fn get_stdlib_path() -> PathBuf {
        // 1. Relative to executable: ../share/graphoid/stdlib
        //    This handles standard installations like:
        //    /usr/local/bin/gr -> /usr/local/share/graphoid/stdlib
        //    ~/.local/bin/gr -> ~/.local/share/graphoid/stdlib
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(bin_dir) = exe_path.parent() {
                if let Some(prefix) = bin_dir.parent() {
                    let stdlib = prefix.join("share/graphoid/stdlib");
                    if stdlib.exists() && stdlib.is_dir() {
                        return stdlib;
                    }
                }
            }
        }

        // 2. User installation: ~/.local/share/graphoid/stdlib (XDG standard)
        if let Some(home) = std::env::var_os("HOME") {
            let user_stdlib = PathBuf::from(home).join(".local/share/graphoid/stdlib");
            if user_stdlib.exists() && user_stdlib.is_dir() {
                return user_stdlib;
            }
        }

        // 3. System-wide installations
        for system_path in &[
            "/usr/local/share/graphoid/stdlib",
            "/usr/share/graphoid/stdlib",
        ] {
            let stdlib = PathBuf::from(system_path);
            if stdlib.exists() && stdlib.is_dir() {
                return stdlib;
            }
        }

        // 4. Development mode: walk up from executable looking for stdlib/
        //    This handles cargo run from the repo:
        //    target/debug/gr -> ../../stdlib
        if let Ok(exe_path) = std::env::current_exe() {
            let mut path = exe_path.clone();
            for _ in 0..5 {
                path.pop();
                let stdlib = path.join("stdlib");
                if stdlib.exists() && stdlib.is_dir() {
                    // Verify it's actually the Graphoid stdlib (has gspec.gr)
                    if stdlib.join("gspec.gr").exists() {
                        return stdlib;
                    }
                }
            }
        }

        // 5. Fallback to "stdlib" in current directory (for running from repo root)
        PathBuf::from("stdlib")
    }

    /// Resolve module name to file path
    pub fn resolve_module_path(
        &self,
        module_name: &str,
        from_file: Option<&Path>,
    ) -> Result<PathBuf> {
        // 1. Relative paths (./file or ../file)
        if module_name.starts_with("./") || module_name.starts_with("../") {
            if let Some(from) = from_file {
                let base = from.parent().ok_or_else(|| GraphoidError::ModuleNotFound {
                    module: module_name.to_string(),
                    position: SourcePosition::unknown(),
                })?;

                let path = base.join(module_name);

                // Try exact path
                if path.exists() && path.is_file() {
                    return path.canonicalize().map_err(|e| {
                        GraphoidError::IOError {
                            message: format!("Failed to canonicalize path: {}", e),
                            position: SourcePosition::unknown(),
                        }
                    });
                }

                // Try with .gr extension
                let with_ext = if path.extension().is_none() {
                    path.with_extension("gr")
                } else {
                    path
                };

                if with_ext.exists() {
                    return with_ext.canonicalize().map_err(|e| {
                        GraphoidError::IOError {
                            message: format!("Failed to canonicalize path: {}", e),
                            position: SourcePosition::unknown(),
                        }
                    });
                }
            }

            return Err(GraphoidError::ModuleNotFound {
                module: module_name.to_string(),
                position: SourcePosition::unknown(),
            });
        }

        // 2. Try same directory as current file (for non-relative imports)
        if let Some(from) = from_file {
            if let Some(base) = from.parent() {
                let candidate = base.join(module_name);

                // Try direct path
                if candidate.exists() && candidate.is_file() {
                    return candidate.canonicalize().map_err(|e| {
                        GraphoidError::IOError {
                            message: format!("Failed to canonicalize path: {}", e),
                            position: SourcePosition::unknown(),
                        }
                    });
                }

                // Try with .gr extension
                let with_ext = if candidate.extension().is_none() {
                    candidate.with_extension("gr")
                } else {
                    candidate
                };

                if with_ext.exists() {
                    return with_ext.canonicalize().map_err(|e| {
                        GraphoidError::IOError {
                            message: format!("Failed to canonicalize path: {}", e),
                            position: SourcePosition::unknown(),
                        }
                    });
                }
            }
        }

        // 3. Project modules (search in search paths)
        for search_path in &self.search_paths {
            let candidate = search_path.join(module_name);

            // Try direct path
            if candidate.exists() && candidate.is_file() {
                return candidate.canonicalize().map_err(|e| {
                    GraphoidError::IOError {
                        message: format!("Failed to canonicalize path: {}", e),
                        position: SourcePosition::unknown(),
                    }
                });
            }

            // Try with .gr extension
            let with_ext = if candidate.extension().is_none() {
                candidate.with_extension("gr")
            } else {
                candidate
            };

            if with_ext.exists() {
                return with_ext.canonicalize().map_err(|e| {
                    GraphoidError::IOError {
                        message: format!("Failed to canonicalize path: {}", e),
                        position: SourcePosition::unknown(),
                    }
                });
            }
        }

        // Not found
        Err(GraphoidError::ModuleNotFound {
            module: module_name.to_string(),
            position: SourcePosition::unknown(),
        })
    }

    /// Register a loaded module
    pub fn register_module(&mut self, name: String, module: Module) {
        self.modules.insert(name, module);
    }

    /// Get a loaded module
    pub fn get_module(&self, name: &str) -> Option<&Module> {
        self.modules.get(name)
    }

    /// Check if module is already loaded
    pub fn is_loaded(&self, name: &str) -> bool {
        self.modules.contains_key(name)
    }

    /// Check if path is currently being loaded
    pub fn is_loading(&self, path: &Path) -> bool {
        self.loading.contains(path)
    }

    /// Check for circular dependency
    pub fn check_circular(&self, path: &Path) -> Result<()> {
        if self.loading.contains(path) {
            // Circular dependency detected!
            let mut chain: Vec<String> = self.import_stack.iter()
                .map(|p| p.display().to_string())
                .collect();
            chain.push(path.display().to_string());

            return Err(GraphoidError::CircularDependency {
                chain,
                position: SourcePosition::unknown(),
            });
        }
        Ok(())
    }

    /// Mark module as being loaded
    pub fn begin_loading(&mut self, path: PathBuf) -> Result<()> {
        self.check_circular(&path)?;
        self.loading.insert(path.clone());
        self.import_stack.push(path);
        Ok(())
    }

    /// Mark module as finished loading
    pub fn end_loading(&mut self, path: &Path) {
        self.loading.remove(path);
        // Remove from stack
        if let Some(pos) = self.import_stack.iter().position(|p| p == path) {
            self.import_stack.remove(pos);
        }
    }

    /// Get import stack depth (for testing)
    pub fn import_stack_depth(&self) -> usize {
        self.import_stack.len()
    }

    // =========================================================================
    // Phase 17: Module dependency graph
    // =========================================================================

    /// Record that `from_module` imports `to_module` (creates a dependency edge)
    pub fn record_dependency(&mut self, from_module: &str, to_module: &str) {
        self.dependencies
            .entry(from_module.to_string())
            .or_default()
            .insert(to_module.to_string());
        self.dependents
            .entry(to_module.to_string())
            .or_default()
            .insert(from_module.to_string());
    }

    /// Helper: convert HashSet<String> to sorted Vec<String>
    fn sorted_vec_from_set(set: Option<&HashSet<String>>) -> Vec<String> {
        set.map(|deps| {
            let mut v: Vec<String> = deps.iter().cloned().collect();
            v.sort();
            v
        })
        .unwrap_or_default()
    }

    /// Get modules that `module_key` directly imports
    pub fn get_dependencies(&self, module_key: &str) -> Vec<String> {
        Self::sorted_vec_from_set(self.dependencies.get(module_key))
    }

    /// Get modules that import `module_key`
    pub fn get_dependents(&self, module_key: &str) -> Vec<String> {
        Self::sorted_vec_from_set(self.dependents.get(module_key))
    }

    /// Get all loaded module keys
    pub fn get_all_module_keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.modules.keys().cloned().collect();
        keys.sort();
        keys
    }

    /// Get all dependency edges as (from, to) pairs
    pub fn get_dependency_edges(&self) -> Vec<(String, String)> {
        let mut edges = Vec::new();
        for (from, tos) in &self.dependencies {
            for to in tos {
                edges.push((from.clone(), to.clone()));
            }
        }
        edges.sort();
        edges
    }

    /// Topological sort of modules (returns error if cycles exist)
    pub fn topological_order(&self) -> Result<Vec<String>> {
        // Collect all nodes
        let mut all_nodes: HashSet<String> = HashSet::new();
        for (from, tos) in &self.dependencies {
            all_nodes.insert(from.clone());
            for to in tos {
                all_nodes.insert(to.clone());
            }
        }
        // Also include modules with no dependencies
        for key in self.modules.keys() {
            all_nodes.insert(key.clone());
        }

        // Kahn's algorithm
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        for node in &all_nodes {
            in_degree.insert(node.clone(), 0);
        }
        for (_from, tos) in &self.dependencies {
            for to in tos {
                *in_degree.entry(to.clone()).or_insert(0) += 1;
            }
        }

        let mut queue: Vec<String> = in_degree.iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(k, _)| k.clone())
            .collect();
        queue.sort(); // deterministic order

        let mut result = Vec::new();
        while let Some(node) = queue.pop() {
            result.push(node.clone());
            if let Some(deps) = self.dependencies.get(&node) {
                for dep in deps {
                    if let Some(deg) = in_degree.get_mut(dep) {
                        *deg -= 1;
                        if *deg == 0 {
                            // Insert sorted to maintain deterministic order
                            let pos = queue.binary_search(dep).unwrap_or_else(|p| p);
                            queue.insert(pos, dep.clone());
                        }
                    }
                }
            }
        }

        if result.len() < all_nodes.len() {
            Err(GraphoidError::runtime(
                "Circular dependency detected in module graph".to_string()
            ))
        } else {
            Ok(result)
        }
    }

    /// Find all cycles in the dependency graph (returns list of cycle paths)
    pub fn find_cycles(&self) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited: HashSet<String> = HashSet::new();
        let mut rec_stack: HashSet<String> = HashSet::new();
        let mut path: Vec<String> = Vec::new();

        let mut all_nodes: Vec<String> = HashSet::<String>::new()
            .into_iter()
            .collect();
        for (from, tos) in &self.dependencies {
            all_nodes.push(from.clone());
            for to in tos {
                all_nodes.push(to.clone());
            }
        }
        all_nodes.sort();
        all_nodes.dedup();

        for node in &all_nodes {
            if !visited.contains(node) {
                self.find_cycles_dfs(node, &mut visited, &mut rec_stack, &mut path, &mut cycles);
            }
        }

        cycles.sort();
        cycles
    }

    fn find_cycles_dfs(
        &self,
        node: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());

        if let Some(deps) = self.dependencies.get(node) {
            let mut sorted_deps: Vec<&String> = deps.iter().collect();
            sorted_deps.sort();
            for dep in sorted_deps {
                if !visited.contains(dep.as_str()) {
                    self.find_cycles_dfs(dep, visited, rec_stack, path, cycles);
                } else if rec_stack.contains(dep.as_str()) {
                    // Found a cycle — extract it from path
                    if let Some(start) = path.iter().position(|p| p == dep) {
                        let mut cycle: Vec<String> = path[start..].to_vec();
                        cycle.push(dep.clone()); // complete the cycle
                        cycles.push(cycle);
                    }
                }
            }
        }

        path.pop();
        rec_stack.remove(node);
    }
}

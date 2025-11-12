use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use crate::error::{Result, GraphoidError, SourcePosition};
use crate::execution::environment::Environment;
use crate::stdlib::NativeModule;

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub name: String,
    pub alias: Option<String>,
    pub namespace: Environment,
    pub file_path: PathBuf,
    pub config: Option<ConfigScope>,
    pub private_symbols: std::collections::HashSet<String>,  // Phase 10: Track private symbols
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
        };

        // Register built-in native modules
        manager.register_native_modules();
        manager
    }

    /// Register all built-in native modules
    fn register_native_modules(&mut self) {
        use crate::stdlib::{ConstantsModule, RandomModule};

        self.register_native_module(Box::new(ConstantsModule));
        self.register_native_module(Box::new(RandomModule::new()));
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
        // Check GRAPHOID_STDLIB_PATH environment variable
        if let Ok(path) = std::env::var("GRAPHOID_STDLIB_PATH") {
            PathBuf::from(path)
        } else {
            // Default to "stdlib" directory
            PathBuf::from("stdlib")
        }
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
}

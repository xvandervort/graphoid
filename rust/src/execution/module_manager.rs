use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use crate::error::{Result, GraphoidError, SourcePosition};
use crate::execution::environment::Environment;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub alias: Option<String>,
    pub namespace: Environment,
    pub file_path: PathBuf,
    pub config: Option<ConfigScope>,
}

#[derive(Debug, Clone)]
pub struct ConfigScope {
    pub decimal_places: Option<u8>,
    pub error_mode: Option<ErrorMode>,
    pub bounds_checking: Option<BoundsMode>,
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorMode {
    Strict,
    Lenient,
    Collect,
}

#[derive(Debug, Clone, Copy)]
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
}

impl ModuleManager {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            import_stack: Vec::new(),
            loading: HashSet::new(),
            search_paths: vec![
                PathBuf::from("src"),
                PathBuf::from("lib"),
                Self::get_stdlib_path(),
            ],
        }
    }

    pub fn search_paths(&self) -> &[PathBuf] {
        &self.search_paths
    }

    fn get_stdlib_path() -> PathBuf {
        // TODO: Make configurable
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

        // 2. Project modules (search in search paths)
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

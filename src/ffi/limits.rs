//! FFI resource limits and usage tracking.

/// Resource limits for FFI operations.
#[derive(Debug, Clone, Default)]
pub struct FfiLimits {
    pub max_bridge_nodes: Option<usize>,
    pub max_memory_bytes: Option<usize>,
    pub max_libraries: Option<usize>,
    pub max_pinned_callbacks: Option<usize>,
}

/// Current FFI resource usage.
#[derive(Debug, Clone, Default)]
pub struct FfiUsage {
    pub bridge_nodes: usize,
    pub allocated_bytes: usize,
    pub library_count: usize,
    pub pinned_callbacks: usize,
}

impl FfiLimits {
    pub fn check_bridge_nodes(&self, usage: &FfiUsage) -> Result<(), String> {
        if let Some(max) = self.max_bridge_nodes {
            if usage.bridge_nodes >= max {
                return Err(format!("FFI bridge node limit exceeded ({}/{})", usage.bridge_nodes, max));
            }
        }
        Ok(())
    }

    pub fn check_memory(&self, usage: &FfiUsage, additional: usize) -> Result<(), String> {
        if let Some(max) = self.max_memory_bytes {
            if usage.allocated_bytes + additional > max {
                return Err(format!(
                    "FFI memory limit exceeded ({}+{} > {} bytes)",
                    usage.allocated_bytes, additional, max
                ));
            }
        }
        Ok(())
    }

    pub fn check_libraries(&self, usage: &FfiUsage) -> Result<(), String> {
        if let Some(max) = self.max_libraries {
            if usage.library_count >= max {
                return Err(format!("FFI library limit exceeded ({}/{})", usage.library_count, max));
            }
        }
        Ok(())
    }

    pub fn check_pinned_callbacks(&self, usage: &FfiUsage) -> Result<(), String> {
        if let Some(max) = self.max_pinned_callbacks {
            if usage.pinned_callbacks >= max {
                return Err(format!("FFI pinned callback limit exceeded ({}/{})", usage.pinned_callbacks, max));
            }
        }
        Ok(())
    }
}

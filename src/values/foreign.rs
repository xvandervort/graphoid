//! Foreign value types for FFI (Phase 20)
//!
//! ForeignLib wraps a loaded dynamic library (via libloading).
//! ForeignPtr wraps a raw pointer with state tracking for safety.

use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::ffi::types::FfiDeclaration;

// ============================================================
// ForeignLib — loaded dynamic library
// ============================================================

pub struct ForeignLibInner {
    pub library: libloading::Library,
    pub name: String,
    pub path: PathBuf,
    pub declarations: HashMap<String, FfiDeclaration>,
}

/// A loaded dynamic library with declared functions.
/// Clone shares the same library handle (Arc).
#[derive(Clone)]
pub struct ForeignLib {
    pub inner: Arc<Mutex<ForeignLibInner>>,
}

impl ForeignLib {
    pub fn new(library: libloading::Library, name: String, path: PathBuf) -> Self {
        Self {
            inner: Arc::new(Mutex::new(ForeignLibInner {
                library,
                name,
                path,
                declarations: HashMap::new(),
            })),
        }
    }

    pub fn name(&self) -> String {
        self.inner.lock().unwrap().name.clone()
    }

    pub fn path(&self) -> PathBuf {
        self.inner.lock().unwrap().path.clone()
    }

    pub fn add_declaration(&self, decl: FfiDeclaration) {
        let mut inner = self.inner.lock().unwrap();
        inner.declarations.insert(decl.name.clone(), decl);
    }

    pub fn get_declaration(&self, name: &str) -> Option<FfiDeclaration> {
        self.inner.lock().unwrap().declarations.get(name).cloned()
    }

    pub fn declaration_names(&self) -> Vec<String> {
        self.inner.lock().unwrap().declarations.keys().cloned().collect()
    }
}

impl fmt::Debug for ForeignLib {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.inner.lock().unwrap();
        write!(f, "ForeignLib({:?})", inner.name)
    }
}

impl fmt::Display for ForeignLib {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.inner.lock().unwrap();
        write!(f, "<foreign_lib:{}>", inner.name)
    }
}

impl PartialEq for ForeignLib {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

// ============================================================
// ForeignPtr — tracked raw pointer
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PtrState {
    Allocated,
    Freed,
}

pub struct ForeignPtrInner {
    pub ptr: *mut u8,
    pub state: PtrState,
    pub size: Option<usize>,
    pub source: String,
    pub owned: bool, // true = we allocated (ffi.alloc), false = C returned it
}

/// A tracked raw pointer with state tracking.
/// Clone shares the same pointer tracking (Arc).
#[derive(Clone)]
pub struct ForeignPtr {
    pub inner: Arc<Mutex<ForeignPtrInner>>,
}

impl ForeignPtr {
    pub fn new(ptr: *mut u8, size: Option<usize>, source: String, owned: bool) -> Self {
        Self {
            inner: Arc::new(Mutex::new(ForeignPtrInner {
                ptr,
                state: PtrState::Allocated,
                size,
                source,
                owned,
            })),
        }
    }

    pub fn state(&self) -> PtrState {
        self.inner.lock().unwrap().state
    }

    pub fn is_freed(&self) -> bool {
        self.inner.lock().unwrap().state == PtrState::Freed
    }

    pub fn address(&self) -> usize {
        self.inner.lock().unwrap().ptr as usize
    }

    pub fn size(&self) -> Option<usize> {
        self.inner.lock().unwrap().size
    }

    /// Mark as freed. Returns error if already freed.
    pub fn mark_freed(&self) -> Result<(), String> {
        let mut inner = self.inner.lock().unwrap();
        if inner.state == PtrState::Freed {
            return Err("Double free detected".to_string());
        }
        inner.state = PtrState::Freed;
        Ok(())
    }

    /// Get the raw pointer. Returns error if freed.
    pub fn get_ptr(&self) -> Result<*mut u8, String> {
        let inner = self.inner.lock().unwrap();
        if inner.state == PtrState::Freed {
            return Err("Use after free detected".to_string());
        }
        Ok(inner.ptr)
    }
}

impl fmt::Debug for ForeignPtr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.inner.lock().unwrap();
        write!(f, "ForeignPtr({:?}, {:p})", inner.state, inner.ptr)
    }
}

impl fmt::Display for ForeignPtr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.inner.lock().unwrap();
        match inner.state {
            PtrState::Allocated => write!(f, "<foreign_ptr:{:p}>", inner.ptr),
            PtrState::Freed => write!(f, "<foreign_ptr:freed>"),
        }
    }
}

impl PartialEq for ForeignPtr {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

// Safety: ForeignLib and ForeignPtr use Arc<Mutex<...>> internally,
// so they are already Send + Sync safe. The raw pointer in ForeignPtr
// is only accessed through the Mutex, ensuring thread safety.
unsafe impl Send for ForeignPtrInner {}
unsafe impl Sync for ForeignPtrInner {}

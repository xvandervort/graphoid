//! Foreign value types for FFI (Phase 20)
//!
//! ForeignLib wraps a loaded dynamic library (via libloading).
//! ForeignPtr wraps a raw pointer with state tracking for safety.

use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::ffi::types::{FfiDeclaration, FfiStructDef};

// ============================================================
// ForeignLib — loaded dynamic library
// ============================================================

pub struct ForeignLibInner {
    pub library: libloading::Library,
    pub name: String,
    pub path: PathBuf,
    pub declarations: HashMap<String, FfiDeclaration>,
    pub struct_defs: HashMap<String, Arc<FfiStructDef>>,
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
                struct_defs: HashMap::new(),
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

    pub fn add_struct_def(&self, def: FfiStructDef) {
        let mut inner = self.inner.lock().unwrap();
        let name = def.name.clone();
        inner.struct_defs.insert(name, Arc::new(def));
    }

    pub fn get_struct_def(&self, name: &str) -> Option<Arc<FfiStructDef>> {
        self.inner.lock().unwrap().struct_defs.get(name).cloned()
    }

    pub fn struct_def_names(&self) -> Vec<String> {
        self.inner.lock().unwrap().struct_defs.keys().cloned().collect()
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

// ============================================================
// ForeignStruct — C struct instance with typed field access
// ============================================================

/// A C struct instance backed by allocated memory with known layout.
/// Clone shares the same underlying memory (via ForeignPtr's Arc).
#[derive(Clone)]
pub struct ForeignStruct {
    pub ptr: ForeignPtr,
    pub struct_def: Arc<FfiStructDef>,
    pub lib_name: String,
}

impl fmt::Debug for ForeignStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ForeignStruct({})", self.struct_def.name)
    }
}

impl fmt::Display for ForeignStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let addr = self.ptr.address();
        write!(f, "<foreign_struct:{}@0x{:x}>", self.struct_def.name, addr)
    }
}

impl PartialEq for ForeignStruct {
    fn eq(&self, other: &Self) -> bool {
        // Identity comparison via the underlying pointer
        self.ptr == other.ptr
    }
}

// ============================================================
// ForeignCallback — pinned C function pointer wrapping a Graphoid function
// ============================================================

/// A persistent callback (pinned libffi closure).
/// Clone shares the same callback (Arc).
#[derive(Clone)]
pub struct ForeignCallback {
    pub inner: Arc<Mutex<ForeignCallbackInner>>,
}

pub struct ForeignCallbackInner {
    pub code_ptr: *const (),
    pub closure_raw: *mut u8,  // Pointer to the leaked CallbackClosure
    pub pinned: bool,
    pub sig: crate::ffi::types::FfiCallbackSig,
}

impl ForeignCallback {
    pub fn new(code_ptr: *const (), closure_raw: *mut u8, sig: crate::ffi::types::FfiCallbackSig) -> Self {
        Self {
            inner: Arc::new(Mutex::new(ForeignCallbackInner {
                code_ptr,
                closure_raw,
                pinned: true,
                sig,
            })),
        }
    }

    pub fn code_ptr(&self) -> *const () {
        self.inner.lock().unwrap().code_ptr
    }

    pub fn is_pinned(&self) -> bool {
        self.inner.lock().unwrap().pinned
    }

    pub fn unpin(&self) -> Result<(), crate::error::GraphoidError> {
        let mut inner = self.inner.lock().unwrap();
        if !inner.pinned {
            return Err(crate::error::GraphoidError::runtime("Callback already unpinned".to_string()));
        }
        inner.pinned = false;
        // Reclaim the leaked CallbackClosure
        unsafe {
            let _ = Box::from_raw(inner.closure_raw as *mut crate::ffi::callback::CallbackClosure);
        }
        inner.closure_raw = std::ptr::null_mut();
        Ok(())
    }
}

impl fmt::Debug for ForeignCallback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.inner.lock().unwrap();
        write!(f, "ForeignCallback(pinned={})", inner.pinned)
    }
}

impl fmt::Display for ForeignCallback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.inner.lock().unwrap();
        if inner.pinned {
            write!(f, "<foreign_callback:{:p}>", inner.code_ptr)
        } else {
            write!(f, "<foreign_callback:unpinned>")
        }
    }
}

impl PartialEq for ForeignCallback {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

unsafe impl Send for ForeignCallbackInner {}
unsafe impl Sync for ForeignCallbackInner {}

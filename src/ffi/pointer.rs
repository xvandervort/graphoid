//! Pointer operations — alloc, free, read, write.

use std::ffi::CStr;
use crate::error::GraphoidError;
use crate::values::foreign::ForeignPtr;

/// Allocate memory of the given size, returning a tracked ForeignPtr.
pub fn ffi_alloc(size: usize) -> Result<ForeignPtr, GraphoidError> {
    if size == 0 {
        return Err(GraphoidError::runtime("ffi.alloc: size must be > 0".to_string()));
    }
    let layout = std::alloc::Layout::from_size_align(size, 8)
        .map_err(|_| GraphoidError::runtime("ffi.alloc: invalid size".to_string()))?;
    let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
    if ptr.is_null() {
        return Err(GraphoidError::runtime("ffi.alloc: allocation failed".to_string()));
    }
    Ok(ForeignPtr::new(ptr, Some(size), "ffi.alloc".to_string(), true))
}

/// Create a pointer-sized allocation (for out parameters like void**).
pub fn ffi_ptr() -> ForeignPtr {
    let size = std::mem::size_of::<*mut u8>();
    let layout = std::alloc::Layout::from_size_align(size, std::mem::align_of::<*mut u8>()).unwrap();
    let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
    ForeignPtr::new(ptr, Some(size), "ffi.ptr".to_string(), true)
}

/// Free a previously allocated pointer.
pub fn ffi_free(fp: &ForeignPtr) -> Result<(), GraphoidError> {
    let inner = fp.inner.lock().unwrap();
    if !inner.owned {
        return Err(GraphoidError::runtime(
            "ffi.free: cannot free pointer not owned by Graphoid (allocated by C)".to_string()
        ));
    }
    let size = inner.size.ok_or_else(|| GraphoidError::runtime(
        "ffi.free: cannot free pointer with unknown size".to_string()
    ))?;
    let ptr = inner.ptr;
    let state = inner.state;
    drop(inner); // Release the lock before mark_freed

    if state == crate::values::foreign::PtrState::Freed {
        return Err(GraphoidError::runtime("Double free detected".to_string()));
    }

    let layout = std::alloc::Layout::from_size_align(size, 8).unwrap();
    unsafe { std::alloc::dealloc(ptr, layout); }
    fp.mark_freed().map_err(|e| GraphoidError::runtime(e))
}

/// Read a null-terminated string from a pointer.
pub fn ptr_read_str(fp: &ForeignPtr) -> Result<String, GraphoidError> {
    let ptr = fp.get_ptr().map_err(|e| GraphoidError::runtime(e))?;
    if ptr.is_null() {
        return Err(GraphoidError::runtime("Cannot read string from null pointer".to_string()));
    }
    let cstr = unsafe { CStr::from_ptr(ptr as *const i8) };
    Ok(cstr.to_string_lossy().into_owned())
}

/// Write a string to a pointer at the given offset.
pub fn ptr_write_str(fp: &ForeignPtr, s: &str, offset: usize) -> Result<(), GraphoidError> {
    let ptr = fp.get_ptr().map_err(|e| GraphoidError::runtime(e))?;
    if let Some(size) = fp.size() {
        if offset + s.len() + 1 > size {
            return Err(GraphoidError::runtime(format!(
                "Write would exceed buffer: offset {} + len {} + 1 > size {}",
                offset, s.len(), size
            )));
        }
    }
    unsafe {
        let dest = ptr.add(offset);
        std::ptr::copy_nonoverlapping(s.as_ptr(), dest, s.len());
        *dest.add(s.len()) = 0; // null terminator
    }
    Ok(())
}

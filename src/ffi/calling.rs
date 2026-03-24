//! Foreign function invocation via libffi.

use std::ffi::{CStr, CString};
use crate::error::GraphoidError;
use crate::ffi::types::{FfiDeclaration, FfiType};
use crate::values::Value;
use crate::values::foreign::{ForeignLib, ForeignPtr};

/// Extract a numeric argument, returning an FFI-specific error on type mismatch.
fn expect_number(arg: &Value, idx: usize, func_name: &str) -> Result<f64, GraphoidError> {
    arg.to_number().ok_or_else(|| GraphoidError::runtime(
        format!("FFI arg {} for '{}': expected number, got {}", idx, func_name, arg.type_name())
    ))
}

/// Extract a raw pointer from a foreign_ptr, foreign_struct, or none value.
fn extract_ptr_arg(arg: &Value, idx: usize, func_name: &str) -> Result<*mut u8, GraphoidError> {
    match &arg.kind {
        crate::values::ValueKind::ForeignPtr(fp) => {
            fp.get_ptr().map_err(|e| GraphoidError::runtime(
                format!("FFI arg {} for '{}': {}", idx, func_name, e)
            ))
        }
        crate::values::ValueKind::ForeignStruct(fs) => {
            fs.ptr.get_ptr().map_err(|e| GraphoidError::runtime(
                format!("FFI arg {} for '{}': {}", idx, func_name, e)
            ))
        }
        crate::values::ValueKind::None => Ok(std::ptr::null_mut()),
        _ => Err(GraphoidError::runtime(
            format!("FFI arg {} for '{}': expected foreign_ptr, foreign_struct, or none, got {}", idx, func_name, arg.type_name())
        )),
    }
}

/// Call a declared foreign function with the given arguments.
pub fn call_foreign_function(
    lib: &ForeignLib,
    decl: &FfiDeclaration,
    args: &[Value],
    globals: &std::collections::HashMap<String, Value>,
) -> Result<Value, GraphoidError> {
    if args.len() != decl.params.len() {
        return Err(GraphoidError::runtime(format!(
            "FFI function '{}' expects {} arguments, got {}",
            decl.name, decl.params.len(), args.len()
        )));
    }

    // Look up the symbol
    let inner = lib.inner.lock().unwrap();
    let symbol: libloading::Symbol<*const ()> = unsafe {
        inner.library.get(decl.name.as_bytes())
    }.map_err(|e| GraphoidError::runtime(format!(
        "Symbol '{}' not found in library: {}", decl.name, e
    )))?;
    let fn_ptr = libffi::middle::CodePtr::from_ptr(*symbol as *const _);

    // Build libffi argument types
    let arg_types: Vec<libffi::middle::Type> = decl.params.iter()
        .map(|t| t.to_libffi_type())
        .collect();
    let ret_type = decl.return_type.to_libffi_type();

    // Build the CIF (Call Interface)
    let cif = libffi::middle::Cif::new(arg_types, ret_type);

    // Marshal arguments from Graphoid values to C values
    // We need to keep CStrings alive for the duration of the call
    let mut c_strings: Vec<CString> = Vec::new();
    let mut c_args: Vec<libffi::middle::Arg> = Vec::new();

    // We store the actual C values here to keep them alive
    let mut i32_vals: Vec<i32> = Vec::new();
    let mut i64_vals: Vec<i64> = Vec::new();
    let mut u32_vals: Vec<u32> = Vec::new();
    let mut u64_vals: Vec<u64> = Vec::new();
    let mut f32_vals: Vec<f32> = Vec::new();
    let mut f64_vals: Vec<f64> = Vec::new();
    let mut usize_vals: Vec<usize> = Vec::new();
    let mut i8_vals: Vec<i8> = Vec::new();
    let mut i16_vals: Vec<i16> = Vec::new();
    let mut u8_vals: Vec<u8> = Vec::new();
    let mut u16_vals: Vec<u16> = Vec::new();
    let mut bool_vals: Vec<u8> = Vec::new();
    let mut ptr_vals: Vec<*mut u8> = Vec::new();
    // For string args, we store the pointer to pass
    let mut str_ptrs: Vec<*const i8> = Vec::new();
    // For callback args: (ptr_vals index, function, signature)
    let mut callback_slots: Vec<(usize, crate::values::Function, crate::ffi::types::FfiCallbackSig)> = Vec::new();

    for (i, (param_type, arg)) in decl.params.iter().zip(args.iter()).enumerate() {
        match param_type {
            FfiType::Int | FfiType::I32 => {
                i32_vals.push(expect_number(arg, i, &decl.name)? as i32);
            }
            FfiType::I64 => {
                i64_vals.push(expect_number(arg, i, &decl.name)? as i64);
            }
            FfiType::U32 => {
                u32_vals.push(expect_number(arg, i, &decl.name)? as u32);
            }
            FfiType::U64 => {
                u64_vals.push(expect_number(arg, i, &decl.name)? as u64);
            }
            FfiType::I8 => {
                i8_vals.push(expect_number(arg, i, &decl.name)? as i8);
            }
            FfiType::I16 => {
                i16_vals.push(expect_number(arg, i, &decl.name)? as i16);
            }
            FfiType::U8 => {
                u8_vals.push(expect_number(arg, i, &decl.name)? as u8);
            }
            FfiType::U16 => {
                u16_vals.push(expect_number(arg, i, &decl.name)? as u16);
            }
            FfiType::F32 => {
                f32_vals.push(expect_number(arg, i, &decl.name)? as f32);
            }
            FfiType::F64 => {
                f64_vals.push(expect_number(arg, i, &decl.name)? as f64);
            }
            FfiType::USize => {
                usize_vals.push(expect_number(arg, i, &decl.name)? as usize);
            }
            FfiType::Bool => {
                let v = if arg.is_truthy() { 1u8 } else { 0u8 };
                bool_vals.push(v);
            }
            FfiType::Str => {
                let s = match &arg.kind {
                    crate::values::ValueKind::String(s) => s.clone(),
                    _ => return Err(GraphoidError::runtime(
                        format!("FFI arg {} for '{}': expected string, got {}", i, decl.name, arg.type_name())
                    )),
                };
                let cs = CString::new(s).map_err(|_| GraphoidError::runtime(
                    format!("FFI arg {} for '{}': string contains null byte", i, decl.name)
                ))?;
                c_strings.push(cs);
            }
            FfiType::Ptr | FfiType::Struct(_) => {
                ptr_vals.push(extract_ptr_arg(arg, i, &decl.name)?);
            }
            FfiType::Callback(ref sig) => {
                match &arg.kind {
                    crate::values::ValueKind::Function(func) => {
                        // Will create closure in second pass (need all values collected first)
                        // For now store a placeholder null — actual pointer set in closure creation pass
                        ptr_vals.push(std::ptr::null_mut());
                        // Store (index, function, sig) for closure creation
                        callback_slots.push((ptr_vals.len() - 1, func.clone(), sig.clone()));
                    }
                    crate::values::ValueKind::ForeignCallback(cb) => {
                        ptr_vals.push(cb.code_ptr() as *mut u8);
                    }
                    crate::values::ValueKind::None => {
                        ptr_vals.push(std::ptr::null_mut());
                    }
                    _ => return Err(GraphoidError::runtime(
                        format!("FFI arg {} for '{}': expected function or foreign_callback, got {}", i, decl.name, arg.type_name())
                    )),
                }
            }
            FfiType::Void => {
                return Err(GraphoidError::runtime(
                    format!("FFI arg {} for '{}': void is not a valid parameter type", i, decl.name)
                ));
            }
        }
    }

    // Now build the actual Arg references — we need to iterate again
    // because we pushed to separate vecs to keep values alive
    let mut i32_idx = 0;
    let mut i64_idx = 0;
    let mut u32_idx = 0;
    let mut u64_idx = 0;
    let mut f32_idx = 0;
    let mut f64_idx = 0;
    let mut usize_idx = 0;
    let mut i8_idx = 0;
    let mut i16_idx = 0;
    let mut u8_idx = 0;
    let mut u16_idx = 0;
    let mut bool_idx = 0;
    let mut str_idx = 0;
    let mut ptr_idx = 0;

    // Build string pointers array (must be done after all CStrings are pushed)
    for cs in &c_strings {
        str_ptrs.push(cs.as_ptr());
    }

    // Create callback closures and update ptr_vals with actual function pointers
    let mut _callback_closures: Vec<crate::ffi::callback::CallbackClosure> = Vec::new();
    for (idx, func, sig) in callback_slots {
        let closure = crate::ffi::callback::create_callback(&func, &sig, globals)?;
        ptr_vals[idx] = closure.fn_ptr as *mut u8;
        _callback_closures.push(closure); // Keep alive for duration of C call
    }

    for param_type in &decl.params {
        match param_type {
            FfiType::Int | FfiType::I32 => {
                c_args.push(libffi::middle::arg(&i32_vals[i32_idx]));
                i32_idx += 1;
            }
            FfiType::I64 => {
                c_args.push(libffi::middle::arg(&i64_vals[i64_idx]));
                i64_idx += 1;
            }
            FfiType::U32 => {
                c_args.push(libffi::middle::arg(&u32_vals[u32_idx]));
                u32_idx += 1;
            }
            FfiType::U64 => {
                c_args.push(libffi::middle::arg(&u64_vals[u64_idx]));
                u64_idx += 1;
            }
            FfiType::I8 => {
                c_args.push(libffi::middle::arg(&i8_vals[i8_idx]));
                i8_idx += 1;
            }
            FfiType::I16 => {
                c_args.push(libffi::middle::arg(&i16_vals[i16_idx]));
                i16_idx += 1;
            }
            FfiType::U8 => {
                c_args.push(libffi::middle::arg(&u8_vals[u8_idx]));
                u8_idx += 1;
            }
            FfiType::U16 => {
                c_args.push(libffi::middle::arg(&u16_vals[u16_idx]));
                u16_idx += 1;
            }
            FfiType::F32 => {
                c_args.push(libffi::middle::arg(&f32_vals[f32_idx]));
                f32_idx += 1;
            }
            FfiType::F64 => {
                c_args.push(libffi::middle::arg(&f64_vals[f64_idx]));
                f64_idx += 1;
            }
            FfiType::USize => {
                c_args.push(libffi::middle::arg(&usize_vals[usize_idx]));
                usize_idx += 1;
            }
            FfiType::Bool => {
                c_args.push(libffi::middle::arg(&bool_vals[bool_idx]));
                bool_idx += 1;
            }
            FfiType::Str => {
                c_args.push(libffi::middle::arg(&str_ptrs[str_idx]));
                str_idx += 1;
            }
            FfiType::Ptr | FfiType::Struct(_) | FfiType::Callback(_) => {
                c_args.push(libffi::middle::arg(&ptr_vals[ptr_idx]));
                ptr_idx += 1;
            }
            FfiType::Void => unreachable!(),
        }
    }

    // Make the call
    let mut result = unsafe {
        match &decl.return_type {
            FfiType::Void => {
                cif.call::<()>(fn_ptr, &c_args);
                Value::none()
            }
            FfiType::Int | FfiType::I32 => {
                let r: i32 = cif.call(fn_ptr, &c_args);
                Value::number(r as f64)
            }
            FfiType::I64 => {
                let r: i64 = cif.call(fn_ptr, &c_args);
                Value::number(r as f64)
            }
            FfiType::U32 => {
                let r: u32 = cif.call(fn_ptr, &c_args);
                Value::number(r as f64)
            }
            FfiType::U64 => {
                let r: u64 = cif.call(fn_ptr, &c_args);
                Value::number(r as f64)
            }
            FfiType::I8 => {
                let r: i8 = cif.call(fn_ptr, &c_args);
                Value::number(r as f64)
            }
            FfiType::I16 => {
                let r: i16 = cif.call(fn_ptr, &c_args);
                Value::number(r as f64)
            }
            FfiType::U8 => {
                let r: u8 = cif.call(fn_ptr, &c_args);
                Value::number(r as f64)
            }
            FfiType::U16 => {
                let r: u16 = cif.call(fn_ptr, &c_args);
                Value::number(r as f64)
            }
            FfiType::F32 => {
                let r: f32 = cif.call(fn_ptr, &c_args);
                Value::number(r as f64)
            }
            FfiType::F64 => {
                let r: f64 = cif.call(fn_ptr, &c_args);
                Value::number(r)
            }
            FfiType::USize => {
                let r: usize = cif.call(fn_ptr, &c_args);
                Value::number(r as f64)
            }
            FfiType::Bool => {
                let r: u8 = cif.call(fn_ptr, &c_args);
                Value::boolean(r != 0)
            }
            FfiType::Str => {
                let r: *const i8 = cif.call(fn_ptr, &c_args);
                if r.is_null() {
                    Value::none()
                } else {
                    let cstr = CStr::from_ptr(r);
                    Value::string(cstr.to_string_lossy().into_owned())
                }
            }
            FfiType::Ptr | FfiType::Struct(_) | FfiType::Callback(_) => {
                let r: *mut u8 = cif.call(fn_ptr, &c_args);
                if r.is_null() {
                    Value::none()
                } else {
                    Value::foreign_ptr(ForeignPtr::new(r, None, decl.name.clone(), false))
                }
            }
        }
    };

    // All values returned from foreign calls are tainted
    result.tainted = true;
    result.taint_source = Some(format!("bridge:func:{}", decl.name));

    Ok(result)
}

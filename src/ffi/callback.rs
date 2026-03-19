//! Callback support — wrapping Graphoid functions as C function pointers.
//!
//! Uses libffi closures to create C-callable function pointers that
//! dispatch to Graphoid function bodies.

use crate::ffi::types::{FfiType, FfiCallbackSig};
use crate::values::Value;
use crate::values::foreign::ForeignCallback;

/// Data attached to a callback closure as userdata.
pub struct CallbackUserdata {
    pub function: crate::values::Function,
    pub sig: FfiCallbackSig,
    pub global_functions: std::collections::HashMap<String, Value>,
}

// Safety: CallbackUserdata is only accessed from the thread that created the closure
// (synchronous C callbacks run on the calling thread).
unsafe impl Send for CallbackUserdata {}
unsafe impl Sync for CallbackUserdata {}

/// Create a call-scoped callback closure from a Graphoid function.
/// Returns a raw function pointer that can be passed to C, and a Box that must be kept alive
/// for the duration of the C call.
///
/// The returned `*const ()` is the C-callable function pointer.
/// The returned `Box<CallbackClosure>` owns the libffi closure and userdata — dropping it
/// invalidates the function pointer.
pub fn create_callback(
    function: &crate::values::Function,
    sig: &FfiCallbackSig,
    global_functions: &std::collections::HashMap<String, Value>,
) -> Result<CallbackClosure, crate::error::GraphoidError> {
    let userdata = Box::new(CallbackUserdata {
        function: function.clone(),
        sig: sig.clone(),
        global_functions: global_functions.clone(),
    });
    // Build the CIF for the callback
    let arg_types: Vec<libffi::middle::Type> = sig.params.iter()
        .map(|t| t.to_libffi_type())
        .collect();
    let ret_type = sig.return_type.to_libffi_type();
    let cif = libffi::middle::Cif::new(arg_types, ret_type);

    // Leak userdata so the closure can reference it with 'static lifetime
    let userdata_ref: &'static CallbackUserdata = Box::leak(userdata);
    let userdata_ptr = userdata_ref as *const CallbackUserdata as *mut CallbackUserdata;

    // Create the closure — trampoline is called by C with userdata_ref
    let closure = libffi::middle::Closure::new(cif, trampoline_callback, userdata_ref);

    let code_ptr = closure.code_ptr();
    let fn_ptr = *code_ptr as *const ();

    Ok(CallbackClosure {
        _closure: closure,
        _userdata: userdata_ptr,
        fn_ptr,
    })
}

/// Owns the libffi closure and userdata. The function pointer is valid as long as this lives.
pub struct CallbackClosure {
    _closure: libffi::middle::Closure<'static>,
    _userdata: *mut CallbackUserdata,
    pub fn_ptr: *const (),
}

impl Drop for CallbackClosure {
    fn drop(&mut self) {
        // Reclaim the userdata
        unsafe { let _ = Box::from_raw(self._userdata); }
    }
}

// Safety: CallbackClosure is only used on the creating thread (synchronous FFI calls)
unsafe impl Send for CallbackClosure {}
unsafe impl Sync for CallbackClosure {}

/// Create a persistent (pinned) callback that lives until explicitly unpinned.
pub fn create_pinned_callback(
    function: &crate::values::Function,
    sig: &FfiCallbackSig,
    global_functions: &std::collections::HashMap<String, Value>,
) -> Result<ForeignCallback, crate::error::GraphoidError> {
    let closure = create_callback(function, sig, global_functions)?;
    let fn_ptr = closure.fn_ptr;

    // Leak the closure so it lives forever (until unpin)
    let leaked = Box::new(closure);
    let raw = Box::into_raw(leaked);

    Ok(ForeignCallback::new(fn_ptr, raw as *mut u8, sig.clone()))
}

/// Unpin a persistent callback, reclaiming its memory.
pub fn unpin_callback(cb: &ForeignCallback) -> Result<(), crate::error::GraphoidError> {
    cb.unpin()
}

/// The trampoline function called by C code via libffi closure.
unsafe extern "C" fn trampoline_callback(
    _cif: &libffi::low::ffi_cif,
    result: &mut libffi::low::ffi_arg,
    args: *const *const std::ffi::c_void,
    userdata: &CallbackUserdata,
) {
    let ud = userdata;

    // Marshal C arguments to Graphoid values
    let mut graphoid_args = Vec::new();
    for (i, param_type) in ud.sig.params.iter().enumerate() {
        let arg_ptr = *args.add(i);
        let value = marshal_c_to_graphoid(arg_ptr, param_type);
        graphoid_args.push(value);
    }

    // Execute the Graphoid function
    let return_value = execute_callback_function(&ud.function, &graphoid_args, &ud.global_functions);

    // Marshal return value back to C
    marshal_graphoid_to_c(&return_value, &ud.sig.return_type, result as *mut libffi::low::ffi_arg as *mut u8);
}

/// Marshal a C argument pointer to a Graphoid Value.
unsafe fn marshal_c_to_graphoid(ptr: *const std::ffi::c_void, ffi_type: &FfiType) -> Value {
    match ffi_type {
        FfiType::Int | FfiType::I32 => Value::number(*(ptr as *const i32) as f64),
        FfiType::I64 => Value::number(*(ptr as *const i64) as f64),
        FfiType::U32 => Value::number(*(ptr as *const u32) as f64),
        FfiType::U64 => Value::number(*(ptr as *const u64) as f64),
        FfiType::I8 => Value::number(*(ptr as *const i8) as f64),
        FfiType::I16 => Value::number(*(ptr as *const i16) as f64),
        FfiType::U8 => Value::number(*(ptr as *const u8) as f64),
        FfiType::U16 => Value::number(*(ptr as *const u16) as f64),
        FfiType::F32 => Value::number(*(ptr as *const f32) as f64),
        FfiType::F64 => Value::number(*(ptr as *const f64)),
        FfiType::USize => Value::number(*(ptr as *const usize) as f64),
        FfiType::Bool => Value::boolean(*(ptr as *const u8) != 0),
        FfiType::Str => {
            let cstr_ptr = *(ptr as *const *const i8);
            if cstr_ptr.is_null() {
                Value::none()
            } else {
                let cstr = std::ffi::CStr::from_ptr(cstr_ptr);
                Value::string(cstr.to_string_lossy().into_owned())
            }
        }
        FfiType::Ptr | FfiType::Struct(_) | FfiType::Callback(_) => {
            let p = *(ptr as *const *mut u8);
            if p.is_null() {
                Value::none()
            } else {
                Value::foreign_ptr(crate::values::ForeignPtr::new(p, None, "callback_arg".to_string(), false))
            }
        }
        FfiType::Void => Value::none(),
    }
}

/// Execute a Graphoid function body with the given arguments.
fn execute_callback_function(
    function: &crate::values::Function,
    args: &[Value],
    global_functions: &std::collections::HashMap<String, Value>,
) -> Value {
    use crate::execution_graph::graph_executor::GraphExecutor;

    let mut executor = GraphExecutor::new();

    // Register global functions
    for (name, val) in global_functions {
        executor.set_variable(name, val.clone());
    }

    // Bind parameters
    for (i, param_name) in function.params.iter().enumerate() {
        if i < args.len() {
            executor.set_variable(param_name, args[i].clone());
        }
    }

    // Also bind captured closure variables
    for (name, val) in function.env.borrow().get_all_bindings() {
        executor.set_variable(&name, val.clone());
    }

    // Execute the function body
    match executor.execute_function_body(&function.body) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("FFI callback error: {}", e);
            Value::number(0.0)
        }
    }
}

/// Marshal a Graphoid return value back to a C result buffer.
unsafe fn marshal_graphoid_to_c(value: &Value, ffi_type: &FfiType, result: *mut u8) {
    match ffi_type {
        FfiType::Void => {}
        FfiType::Int | FfiType::I32 => {
            let v = value.to_number().unwrap_or(0.0) as i32;
            *(result as *mut libffi::low::ffi_arg) = v as libffi::low::ffi_arg;
        }
        FfiType::I64 => {
            let v = value.to_number().unwrap_or(0.0) as i64;
            *(result as *mut i64) = v;
        }
        FfiType::U32 => {
            let v = value.to_number().unwrap_or(0.0) as u32;
            *(result as *mut libffi::low::ffi_arg) = v as libffi::low::ffi_arg;
        }
        FfiType::U64 => {
            let v = value.to_number().unwrap_or(0.0) as u64;
            *(result as *mut u64) = v;
        }
        FfiType::F32 => {
            let v = value.to_number().unwrap_or(0.0) as f32;
            *(result as *mut f32) = v;
        }
        FfiType::F64 => {
            let v = value.to_number().unwrap_or(0.0);
            *(result as *mut f64) = v;
        }
        FfiType::Bool => {
            let v = if value.is_truthy() { 1u8 } else { 0u8 };
            *(result as *mut libffi::low::ffi_arg) = v as libffi::low::ffi_arg;
        }
        _ => {
            // For pointer/string/other types, return 0 (null)
            *(result as *mut libffi::low::ffi_arg) = 0;
        }
    }
}

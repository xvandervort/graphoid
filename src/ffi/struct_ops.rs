//! Struct operations — allocation, field read/write.

use std::sync::Arc;
use crate::error::GraphoidError;
use crate::ffi::types::{FfiType, FfiStructDef};
use crate::values::Value;
use crate::values::foreign::{ForeignPtr, ForeignStruct};

/// Create a new struct instance with the given field values.
pub fn create_struct(
    def: &Arc<FfiStructDef>,
    fields: &std::collections::HashMap<String, Value>,
    lib_name: &str,
) -> Result<ForeignStruct, GraphoidError> {
    // Validate all fields are provided
    for field in &def.fields {
        if !fields.contains_key(&field.name) {
            return Err(GraphoidError::runtime(format!(
                "Missing field '{}' for struct '{}'", field.name, def.name
            )));
        }
    }

    // Allocate memory
    let ptr = crate::ffi::pointer::ffi_alloc(def.size)?;

    // Write initial field values
    for field in &def.fields {
        if let Some(value) = fields.get(&field.name) {
            write_field_raw(&ptr, field.offset, &field.ffi_type, value)?;
        }
    }

    Ok(ForeignStruct {
        ptr,
        struct_def: Arc::clone(def),
        lib_name: lib_name.to_string(),
    })
}

/// Read a field from a struct instance.
pub fn read_field(fs: &ForeignStruct, field_name: &str) -> Result<Value, GraphoidError> {
    let field = fs.struct_def.get_field(field_name).ok_or_else(|| {
        GraphoidError::runtime(format!(
            "Struct '{}' has no field '{}'", fs.struct_def.name, field_name
        ))
    })?;

    let raw = fs.ptr.get_ptr().map_err(|e| GraphoidError::runtime(e))?;
    let mut val = read_field_raw(raw, field.offset, &field.ffi_type)?;
    // Struct field reads are tainted — data comes from foreign memory
    val.tainted = true;
    val.taint_source = Some(format!("bridge:struct:{}:{}", fs.struct_def.name, field_name));
    Ok(val)
}

/// Write a value to a struct field.
pub fn write_field(fs: &ForeignStruct, field_name: &str, value: &Value) -> Result<(), GraphoidError> {
    let field = fs.struct_def.get_field(field_name).ok_or_else(|| {
        GraphoidError::runtime(format!(
            "Struct '{}' has no field '{}'", fs.struct_def.name, field_name
        ))
    })?;

    write_field_raw(&fs.ptr, field.offset, &field.ffi_type, value)
}

/// Read all fields as a map.
pub fn read_all_fields(fs: &ForeignStruct) -> Result<crate::values::hash::Hash, GraphoidError> {
    let raw = fs.ptr.get_ptr().map_err(|e| GraphoidError::runtime(e))?;
    let mut map = crate::values::hash::Hash::new();
    for field in &fs.struct_def.fields {
        let mut val = read_field_raw(raw, field.offset, &field.ffi_type)?;
        val.tainted = true;
        val.taint_source = Some(format!("bridge:struct:{}:{}", fs.struct_def.name, field.name));
        let _ = map.insert(field.name.clone(), val);
    }
    Ok(map)
}

/// Read a value from raw memory at the given offset, interpreting as the given type.
fn read_field_raw(base: *mut u8, offset: usize, ffi_type: &FfiType) -> Result<Value, GraphoidError> {
    unsafe {
        let ptr = base.add(offset);
        match ffi_type {
            FfiType::Bool => Ok(Value::boolean(*ptr != 0)),
            FfiType::I8 => Ok(Value::number(*(ptr as *const i8) as f64)),
            FfiType::U8 => Ok(Value::number(*ptr as f64)),
            FfiType::I16 => Ok(Value::number(*(ptr as *const i16) as f64)),
            FfiType::U16 => Ok(Value::number(*(ptr as *const u16) as f64)),
            FfiType::I32 | FfiType::Int => Ok(Value::number(*(ptr as *const i32) as f64)),
            FfiType::U32 => Ok(Value::number(*(ptr as *const u32) as f64)),
            FfiType::I64 => Ok(Value::number(*(ptr as *const i64) as f64)),
            FfiType::U64 => Ok(Value::number(*(ptr as *const u64) as f64)),
            FfiType::F32 => Ok(Value::number(*(ptr as *const f32) as f64)),
            FfiType::F64 => Ok(Value::number(*(ptr as *const f64))),
            FfiType::USize => Ok(Value::number(*(ptr as *const usize) as f64)),
            FfiType::Ptr | FfiType::Str | FfiType::Struct(_) | FfiType::Callback(_) => {
                let p = *(ptr as *const *mut u8);
                if p.is_null() {
                    Ok(Value::none())
                } else {
                    Ok(Value::foreign_ptr(ForeignPtr::new(p, None, "struct_field".to_string(), false)))
                }
            }
            FfiType::Void => Ok(Value::none()),
        }
    }
}

/// Write a value to raw memory at the given offset, interpreting as the given type.
fn write_field_raw(fp: &ForeignPtr, offset: usize, ffi_type: &FfiType, value: &Value) -> Result<(), GraphoidError> {
    let base = fp.get_ptr().map_err(|e| GraphoidError::runtime(e))?;
    let num = || value.to_number().ok_or_else(|| {
        GraphoidError::type_error("number", value.type_name())
    });
    unsafe {
        let ptr = base.add(offset);
        match ffi_type {
            FfiType::Bool => {
                *ptr = if value.is_truthy() { 1 } else { 0 };
            }
            FfiType::I8 => { *(ptr as *mut i8) = num()? as i8; }
            FfiType::U8 => { *ptr = num()? as u8; }
            FfiType::I16 => { *(ptr as *mut i16) = num()? as i16; }
            FfiType::U16 => { *(ptr as *mut u16) = num()? as u16; }
            FfiType::I32 | FfiType::Int => { *(ptr as *mut i32) = num()? as i32; }
            FfiType::U32 => { *(ptr as *mut u32) = num()? as u32; }
            FfiType::I64 => { *(ptr as *mut i64) = num()? as i64; }
            FfiType::U64 => { *(ptr as *mut u64) = num()? as u64; }
            FfiType::F32 => { *(ptr as *mut f32) = num()? as f32; }
            FfiType::F64 => { *(ptr as *mut f64) = num()?; }
            FfiType::USize => { *(ptr as *mut usize) = num()? as usize; }
            FfiType::Ptr | FfiType::Str | FfiType::Struct(_) | FfiType::Callback(_) => {
                return Err(GraphoidError::runtime(
                    "Cannot write pointer/string/callback fields directly".to_string()
                ));
            }
            FfiType::Void => {}
        }
    }
    Ok(())
}

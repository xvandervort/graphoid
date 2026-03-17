//! FFI type system — C type declarations and mapping to Graphoid values.

use crate::error::GraphoidError;

/// C/FFI type used in function declarations.
#[derive(Debug, Clone, PartialEq)]
pub enum FfiType {
    Void,
    Bool,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Int,   // Platform int (typically i32)
    USize, // Platform size_t
    Str,   // const char* (null-terminated)
    Ptr,   // void*
}

impl FfiType {
    /// Parse an FFI type from a string (used by lib.decl()).
    pub fn from_str(s: &str) -> Result<Self, GraphoidError> {
        match s {
            "void" => Ok(FfiType::Void),
            "bool" => Ok(FfiType::Bool),
            "i8" | "int8" | "int8_t" => Ok(FfiType::I8),
            "i16" | "int16" | "int16_t" => Ok(FfiType::I16),
            "i32" | "int32" | "int32_t" => Ok(FfiType::I32),
            "i64" | "int64" | "int64_t" => Ok(FfiType::I64),
            "u8" | "uint8" | "uint8_t" => Ok(FfiType::U8),
            "u16" | "uint16" | "uint16_t" => Ok(FfiType::U16),
            "u32" | "uint32" | "uint32_t" => Ok(FfiType::U32),
            "u64" | "uint64" | "uint64_t" => Ok(FfiType::U64),
            "f32" | "float" => Ok(FfiType::F32),
            "f64" | "double" => Ok(FfiType::F64),
            "int" => Ok(FfiType::Int),
            "usize" | "size_t" => Ok(FfiType::USize),
            "str" | "string" | "char*" => Ok(FfiType::Str),
            "ptr" | "void*" | "pointer" => Ok(FfiType::Ptr),
            _ => Err(GraphoidError::runtime(format!("Unknown FFI type: '{}'", s))),
        }
    }

    /// Convert to libffi type representation.
    pub fn to_libffi_type(&self) -> libffi::middle::Type {
        match self {
            FfiType::Void => libffi::middle::Type::void(),
            FfiType::Bool => libffi::middle::Type::u8(),
            FfiType::I8 => libffi::middle::Type::i8(),
            FfiType::I16 => libffi::middle::Type::i16(),
            FfiType::I32 => libffi::middle::Type::i32(),
            FfiType::I64 => libffi::middle::Type::i64(),
            FfiType::U8 => libffi::middle::Type::u8(),
            FfiType::U16 => libffi::middle::Type::u16(),
            FfiType::U32 => libffi::middle::Type::u32(),
            FfiType::U64 => libffi::middle::Type::u64(),
            FfiType::F32 => libffi::middle::Type::f32(),
            FfiType::F64 => libffi::middle::Type::f64(),
            FfiType::Int => libffi::middle::Type::i32(),
            FfiType::USize => libffi::middle::Type::usize(),
            FfiType::Str => libffi::middle::Type::pointer(),
            FfiType::Ptr => libffi::middle::Type::pointer(),
        }
    }
}

/// A declared foreign function with its signature.
#[derive(Debug, Clone)]
pub struct FfiDeclaration {
    pub name: String,
    pub params: Vec<FfiType>,
    pub return_type: FfiType,
}

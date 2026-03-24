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
    Struct(String),  // Named struct type (passed by pointer)
    Callback(FfiCallbackSig),  // Function pointer type
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
            other if other.starts_with("struct:") => {
                Ok(FfiType::Struct(other[7..].to_string()))
            }
            _ => Err(GraphoidError::runtime(format!(
                "Unknown FFI type: '{}'. For struct types use 'struct:Name' or lib.cdef()", s
            ))),
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
            FfiType::Struct(_) => libffi::middle::Type::pointer(),  // Structs passed by pointer
            FfiType::Callback(_) => libffi::middle::Type::pointer(),  // Function pointers
        }
    }

    /// Size of this type in bytes (C ABI).
    pub fn size(&self) -> usize {
        match self {
            FfiType::Void => 0,
            FfiType::Bool | FfiType::I8 | FfiType::U8 => 1,
            FfiType::I16 | FfiType::U16 => 2,
            FfiType::I32 | FfiType::U32 | FfiType::F32 | FfiType::Int => 4,
            FfiType::I64 | FfiType::U64 | FfiType::F64 => 8,
            FfiType::USize | FfiType::Str | FfiType::Ptr
            | FfiType::Struct(_) | FfiType::Callback(_) => std::mem::size_of::<*mut u8>(),
        }
    }

    /// Alignment of this type in bytes (C ABI).
    pub fn alignment(&self) -> usize {
        match self {
            FfiType::Void => 1,
            FfiType::Bool | FfiType::I8 | FfiType::U8 => 1,
            FfiType::I16 | FfiType::U16 => 2,
            FfiType::I32 | FfiType::U32 | FfiType::F32 | FfiType::Int => 4,
            FfiType::I64 | FfiType::U64 | FfiType::F64 => 8,
            FfiType::USize | FfiType::Str | FfiType::Ptr
            | FfiType::Struct(_) | FfiType::Callback(_) => std::mem::align_of::<*mut u8>(),
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

/// Callback (function pointer) signature.
#[derive(Debug, Clone, PartialEq)]
pub struct FfiCallbackSig {
    pub params: Vec<FfiType>,
    pub return_type: Box<FfiType>,
}

/// A field in a C struct definition.
#[derive(Debug, Clone)]
pub struct FfiStructField {
    pub name: String,
    pub ffi_type: FfiType,
    pub offset: usize,
    pub size: usize,
}

/// A C struct definition with computed layout.
#[derive(Debug, Clone)]
pub struct FfiStructDef {
    pub name: String,
    pub fields: Vec<FfiStructField>,
    pub size: usize,
    pub alignment: usize,
}

impl FfiStructDef {
    /// Compute struct layout from field names and types using C ABI alignment rules.
    pub fn compute_layout(name: String, raw_fields: Vec<(String, FfiType)>) -> Self {
        let mut fields = Vec::new();
        let mut offset = 0usize;
        let mut max_alignment = 1usize;

        for (field_name, ffi_type) in raw_fields {
            let field_align = ffi_type.alignment();
            let field_size = ffi_type.size();

            // Align offset to field's natural alignment
            let padding = (field_align - (offset % field_align)) % field_align;
            offset += padding;

            fields.push(FfiStructField {
                name: field_name,
                ffi_type,
                offset,
                size: field_size,
            });

            offset += field_size;
            if field_align > max_alignment {
                max_alignment = field_align;
            }
        }

        // Pad total size to struct alignment
        let padding = (max_alignment - (offset % max_alignment)) % max_alignment;
        let size = offset + padding;

        FfiStructDef {
            name,
            fields,
            size,
            alignment: max_alignment,
        }
    }

    /// Look up a field by name.
    pub fn get_field(&self, name: &str) -> Option<&FfiStructField> {
        self.fields.iter().find(|f| f.name == name)
    }
}

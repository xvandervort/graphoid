//! C declaration parser for lib.cdef().
//!
//! Parses a practical subset of C declarations:
//! - Function declarations: `int abs(int x);`
//! - Struct definitions: `struct Point { double x; double y; };`
//! - Opaque typedefs: `typedef struct sqlite3 sqlite3;`
//! - Callback params: `int (*compar)(const void*, const void*)`

use crate::error::GraphoidError;
use crate::ffi::types::{FfiType, FfiDeclaration, FfiCallbackSig, FfiStructDef};

/// A parsed item from a cdef string.
#[derive(Debug)]
pub enum CdefItem {
    Function(FfiDeclaration),
    Struct(FfiStructDef),
    OpaqueType(String),
}

/// Parse a cdef string into a list of declarations.
pub fn parse_cdef(input: &str) -> Result<Vec<CdefItem>, GraphoidError> {
    let mut items = Vec::new();
    let tokens = tokenize(input);
    let mut pos = 0;

    while pos < tokens.len() {
        // Skip semicolons between declarations
        if tokens[pos] == ";" {
            pos += 1;
            continue;
        }

        if tokens[pos] == "typedef" {
            let (item, next) = parse_typedef(&tokens, pos)?;
            items.push(item);
            pos = next;
        } else if tokens[pos] == "struct" && pos + 1 < tokens.len() {
            // Could be struct definition or struct in a return type
            if pos + 2 < tokens.len() && tokens[pos + 2] == "{" {
                let (item, next) = parse_struct_def(&tokens, pos)?;
                items.push(item);
                pos = next;
            } else {
                // struct used as return type: `struct Name* func(...)`
                let (item, next) = parse_function_decl(&tokens, pos)?;
                items.push(item);
                pos = next;
            }
        } else {
            let (item, next) = parse_function_decl(&tokens, pos)?;
            items.push(item);
            pos = next;
        }
    }

    Ok(items)
}

/// Tokenize C declaration text into words and punctuation.
fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        match c {
            // Punctuation tokens
            '{' | '}' | '(' | ')' | ';' | ',' | '*' => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
                tokens.push(c.to_string());
            }
            // Whitespace
            ' ' | '\t' | '\n' | '\r' => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            // Part of identifier or keyword
            _ => {
                current.push(c);
            }
        }
        i += 1;
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

/// Parse a typedef declaration.
/// Handles: `typedef struct Name Name;`
fn parse_typedef(tokens: &[String], pos: usize) -> Result<(CdefItem, usize), GraphoidError> {
    // typedef struct Name Name ;
    let mut i = pos + 1; // skip "typedef"

    if i < tokens.len() && tokens[i] == "struct" {
        i += 1; // skip "struct"
        if i >= tokens.len() {
            return Err(GraphoidError::runtime("cdef: incomplete typedef".to_string()));
        }
        let struct_name = tokens[i].clone();
        i += 1;

        // Check if this is a typedef with struct body: typedef struct Name { ... } Name;
        if i < tokens.len() && tokens[i] == "{" {
            // Parse struct body
            let (fields, next) = parse_struct_body(tokens, i)?;
            i = next;
            // Expect alias name
            if i >= tokens.len() {
                return Err(GraphoidError::runtime("cdef: expected alias name after struct body".to_string()));
            }
            let _alias = &tokens[i];
            i += 1;
            // Expect semicolon
            if i < tokens.len() && tokens[i] == ";" {
                i += 1;
            }
            let def = FfiStructDef::compute_layout(struct_name, fields);
            return Ok((CdefItem::Struct(def), i));
        }

        // Simple opaque typedef: typedef struct Name Name;
        // Skip the alias name
        if i < tokens.len() && tokens[i] != ";" {
            i += 1; // skip alias
        }
        if i < tokens.len() && tokens[i] == ";" {
            i += 1;
        }
        return Ok((CdefItem::OpaqueType(struct_name), i));
    }

    Err(GraphoidError::runtime(format!(
        "cdef: unsupported typedef at '{}'", tokens.get(pos + 1).unwrap_or(&"EOF".to_string())
    )))
}

/// Parse a struct definition: `struct Name { type field; ... };`
fn parse_struct_def(tokens: &[String], pos: usize) -> Result<(CdefItem, usize), GraphoidError> {
    let mut i = pos + 1; // skip "struct"
    if i >= tokens.len() {
        return Err(GraphoidError::runtime("cdef: expected struct name".to_string()));
    }
    let name = tokens[i].clone();
    i += 1;

    if i >= tokens.len() || tokens[i] != "{" {
        return Err(GraphoidError::runtime(format!("cdef: expected '{{' after struct {}", name)));
    }

    let (fields, next) = parse_struct_body(tokens, i)?;
    i = next;

    // Optional semicolon after closing brace
    if i < tokens.len() && tokens[i] == ";" {
        i += 1;
    }

    let def = FfiStructDef::compute_layout(name, fields);
    Ok((CdefItem::Struct(def), i))
}

/// Parse struct body: `{ type field; ... }` — returns field list and position after `}`.
fn parse_struct_body(tokens: &[String], pos: usize) -> Result<(Vec<(String, FfiType)>, usize), GraphoidError> {
    let mut i = pos + 1; // skip "{"
    let mut fields = Vec::new();

    while i < tokens.len() && tokens[i] != "}" {
        let (ffi_type, next) = parse_c_type(tokens, i)?;
        i = next;

        if i >= tokens.len() || tokens[i] == "}" {
            return Err(GraphoidError::runtime("cdef: expected field name in struct".to_string()));
        }
        let field_name = tokens[i].clone();
        i += 1;

        // Expect semicolon
        if i < tokens.len() && tokens[i] == ";" {
            i += 1;
        }

        fields.push((field_name, ffi_type));
    }

    if i >= tokens.len() {
        return Err(GraphoidError::runtime("cdef: unterminated struct body".to_string()));
    }
    i += 1; // skip "}"

    Ok((fields, i))
}

/// Parse a function declaration: `type name(type param, ...);`
fn parse_function_decl(tokens: &[String], pos: usize) -> Result<(CdefItem, usize), GraphoidError> {
    let (return_type, mut i) = parse_c_type(tokens, pos)?;

    if i >= tokens.len() {
        return Err(GraphoidError::runtime("cdef: expected function name".to_string()));
    }
    let func_name = tokens[i].clone();
    i += 1;

    if i >= tokens.len() || tokens[i] != "(" {
        return Err(GraphoidError::runtime(format!("cdef: expected '(' after function name '{}'", func_name)));
    }
    i += 1; // skip "("

    let mut params = Vec::new();

    // Parse parameter list
    while i < tokens.len() && tokens[i] != ")" {
        if tokens[i] == "," {
            i += 1;
            continue;
        }

        // Check for callback parameter: type (*name)(params)
        // We detect this by looking for "(" after seeing "*" in what looks like a param
        if tokens[i] == "..." {
            // Variadic — skip (Graphoid passes fixed args)
            i += 1;
            continue;
        }

        let (param_type, next) = parse_param(tokens, i)?;
        params.push(param_type);
        i = next;
    }

    if i >= tokens.len() {
        return Err(GraphoidError::runtime(format!("cdef: unterminated parameter list for '{}'", func_name)));
    }
    i += 1; // skip ")"

    if i < tokens.len() && tokens[i] == ";" {
        i += 1;
    }

    Ok((CdefItem::Function(FfiDeclaration {
        name: func_name,
        params,
        return_type,
    }), i))
}

/// Parse a single function parameter, which may be a callback type.
fn parse_param(tokens: &[String], pos: usize) -> Result<(FfiType, usize), GraphoidError> {
    let (base_type, mut i) = parse_c_type(tokens, pos)?;

    // Check if next token is a parameter name (identifier before "," or ")")
    // If so, skip it
    if i < tokens.len() && tokens[i] != "," && tokens[i] != ")" && tokens[i] != "(" {
        i += 1; // skip param name
    }

    // Check for callback: if we parsed a return type and now see "("
    // This handles: `int (*name)(type, type)` — but parse_c_type would have consumed `int`
    // and we'd be at `(` for `(*name)`. Let's check.
    // Actually the pattern is: return_type ( * name ) ( params )
    // parse_c_type would stop at `(`, so we check here.

    Ok((base_type, i))
}

/// Parse a C type expression (handles const, struct, pointers, and callback signatures).
fn parse_c_type(tokens: &[String], pos: usize) -> Result<(FfiType, usize), GraphoidError> {
    let mut i = pos;

    // Skip qualifiers
    while i < tokens.len() && (tokens[i] == "const" || tokens[i] == "unsigned" || tokens[i] == "signed") {
        i += 1;
    }

    if i >= tokens.len() {
        return Err(GraphoidError::runtime("cdef: expected type".to_string()));
    }

    let base = &tokens[i];
    let mut ffi_type = match base.as_str() {
        "void" => FfiType::Void,
        "char" => FfiType::I8,  // plain char, will become Str if followed by *
        "short" => FfiType::I16,
        "int" => FfiType::Int,
        "long" => {
            // long long = i64, long = platform long
            if i + 1 < tokens.len() && tokens[i + 1] == "long" {
                i += 1;
                FfiType::I64
            } else {
                FfiType::I64  // long is 64-bit on LP64 (Linux/macOS x86_64)
            }
        }
        "float" => FfiType::F32,
        "double" => FfiType::F64,
        "size_t" => FfiType::USize,
        "int8_t" => FfiType::I8,
        "int16_t" => FfiType::I16,
        "int32_t" => FfiType::I32,
        "int64_t" => FfiType::I64,
        "uint8_t" => FfiType::U8,
        "uint16_t" => FfiType::U16,
        "uint32_t" => FfiType::U32,
        "uint64_t" => FfiType::U64,
        "struct" => {
            i += 1;
            if i >= tokens.len() {
                return Err(GraphoidError::runtime("cdef: expected struct name".to_string()));
            }
            let name = tokens[i].clone();
            FfiType::Struct(name)
        }
        "_Bool" | "bool" => FfiType::Bool,
        other => {
            // Could be a typedef name (opaque type) — treat as pointer type
            FfiType::Struct(other.to_string())
        }
    };
    i += 1;

    // Check for pointer(s)
    let mut pointer_count = 0;
    while i < tokens.len() && tokens[i] == "*" {
        pointer_count += 1;
        i += 1;
    }

    // Check for callback syntax: return_type ( * name ) ( params )
    // After consuming base type, if we see "(" and then "*"
    if pointer_count == 0 && i < tokens.len() && tokens[i] == "(" {
        // Might be a callback: type (*name)(params)
        if i + 1 < tokens.len() && tokens[i + 1] == "*" {
            return parse_callback_type(tokens, i, ffi_type);
        }
    }

    if pointer_count > 0 {
        // char* / const char* -> Str
        if matches!(ffi_type, FfiType::I8) && pointer_count == 1 {
            ffi_type = FfiType::Str;
        } else if matches!(ffi_type, FfiType::Void) && pointer_count == 1 {
            ffi_type = FfiType::Ptr;
        } else if pointer_count >= 1 {
            // T* -> Ptr (we don't track pointed-to type for now)
            ffi_type = FfiType::Ptr;
        }
    }

    Ok((ffi_type, i))
}

/// Parse a callback (function pointer) type: `return_type (*name)(param_types)`
/// Called when we've already parsed the return type and are at `(`.
fn parse_callback_type(tokens: &[String], pos: usize, return_type: FfiType) -> Result<(FfiType, usize), GraphoidError> {
    let mut i = pos + 1; // skip "("

    // Skip "*"
    if i < tokens.len() && tokens[i] == "*" {
        i += 1;
    }

    // Skip optional name
    if i < tokens.len() && tokens[i] != ")" {
        i += 1;
    }

    // Expect ")"
    if i >= tokens.len() || tokens[i] != ")" {
        return Err(GraphoidError::runtime("cdef: expected ')' in callback type".to_string()));
    }
    i += 1;

    // Expect "(" for parameter list
    if i >= tokens.len() || tokens[i] != "(" {
        return Err(GraphoidError::runtime("cdef: expected '(' for callback parameters".to_string()));
    }
    i += 1;

    let mut params = Vec::new();
    while i < tokens.len() && tokens[i] != ")" {
        if tokens[i] == "," {
            i += 1;
            continue;
        }
        let (param_type, next) = parse_c_type(tokens, i)?;
        i = next;
        // Skip optional param name
        if i < tokens.len() && tokens[i] != "," && tokens[i] != ")" {
            i += 1;
        }
        params.push(param_type);
    }

    if i >= tokens.len() {
        return Err(GraphoidError::runtime("cdef: unterminated callback parameter list".to_string()));
    }
    i += 1; // skip ")"

    Ok((FfiType::Callback(FfiCallbackSig {
        params,
        return_type: Box::new(return_type),
    }), i))
}

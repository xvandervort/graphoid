//! Crypto Module - Production-grade cryptographic functions
//!
//! This module provides industry-standard cryptographic primitives using
//! well-vetted Rust libraries. All algorithms are cryptographically secure
//! and suitable for production use.
//!
//! # Security Notes
//! - Uses constant-time operations where appropriate
//! - All encryption is authenticated (AEAD)
//! - Secure random number generation
//! - Modern, audited algorithms

use super::{NativeFunction, NativeModule};
use crate::error::{GraphoidError, Result};
use crate::values::{Hash, Value, ValueKind};
use std::collections::HashMap;

// Hashing
use sha2::{Sha256, Sha512, Digest};
use sha1::{Sha1, Digest as Sha1Digest};
use md5::{Md5, Digest as Md5Digest};
use blake2::{Blake2b512, Digest as Blake2Digest};
use blake3::Hasher as Blake3Hasher;

// Encoding
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

// Encryption
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use chacha20poly1305::{ChaCha20Poly1305, Key as ChaChaKey};

// HMAC
use hmac::Hmac;
use hmac::Mac as HmacMac;

// Key derivation
use pbkdf2::pbkdf2_hmac_array;
use argon2::{Argon2, PasswordHasher, PasswordHash, PasswordVerifier};

// Digital signatures
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};

/// Crypto module providing cryptographic functions
pub struct CryptoModule;

impl NativeModule for CryptoModule {
    fn name(&self) -> &str {
        "crypto"
    }

    fn alias(&self) -> Option<&str> {
        None
    }

    fn functions(&self) -> HashMap<String, NativeFunction> {
        let mut functions = HashMap::new();

        // Hashing functions
        functions.insert("sha256".to_string(), sha256 as NativeFunction);
        functions.insert("sha512".to_string(), sha512 as NativeFunction);
        functions.insert("sha1".to_string(), sha1 as NativeFunction);  // deprecated
        functions.insert("md5".to_string(), md5 as NativeFunction);    // deprecated
        functions.insert("blake2b".to_string(), blake2b as NativeFunction);
        functions.insert("blake3".to_string(), blake3 as NativeFunction);

        // Encoding functions
        functions.insert("to_hex".to_string(), to_hex as NativeFunction);
        functions.insert("from_hex".to_string(), from_hex as NativeFunction);
        functions.insert("to_base64".to_string(), to_base64 as NativeFunction);
        functions.insert("from_base64".to_string(), from_base64 as NativeFunction);

        // Key generation
        functions.insert("generate_key".to_string(), generate_key as NativeFunction);

        // Symmetric encryption (AES-256-GCM)
        functions.insert("aes_encrypt".to_string(), aes_encrypt as NativeFunction);
        functions.insert("aes_decrypt".to_string(), aes_decrypt as NativeFunction);

        // Symmetric encryption (ChaCha20-Poly1305)
        functions.insert("chacha20_encrypt".to_string(), chacha20_encrypt as NativeFunction);
        functions.insert("chacha20_decrypt".to_string(), chacha20_decrypt as NativeFunction);

        // HMAC
        functions.insert("hmac_sha256".to_string(), hmac_sha256 as NativeFunction);
        functions.insert("hmac_verify".to_string(), hmac_verify as NativeFunction);

        // Key derivation
        functions.insert("pbkdf2".to_string(), pbkdf2_fn as NativeFunction);

        // Digital signatures
        functions.insert("generate_keypair".to_string(), generate_keypair as NativeFunction);
        functions.insert("sign".to_string(), sign as NativeFunction);
        functions.insert("verify".to_string(), verify as NativeFunction);

        functions
    }
}

// =============================================================================
// HASHING FUNCTIONS
// =============================================================================

/// SHA-256 hash (32 bytes)
fn sha256(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(GraphoidError::runtime("sha256() requires 1 argument: data".to_string()));
    }

    let data = match &args[0].kind {
        ValueKind::String(s) => s.as_bytes(),
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();

    // Return as hex string
    Ok(Value::string(hex::encode(result)))
}

/// SHA-512 hash (64 bytes)
fn sha512(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(GraphoidError::runtime("sha512() requires 1 argument: data".to_string()));
    }

    let data = match &args[0].kind {
        ValueKind::String(s) => s.as_bytes(),
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let mut hasher = Sha512::new();
    hasher.update(data);
    let result = hasher.finalize();

    Ok(Value::string(hex::encode(result)))
}

/// SHA-1 hash (20 bytes) - DEPRECATED, for legacy compatibility only
fn sha1(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(GraphoidError::runtime("sha1() requires 1 argument: data".to_string()));
    }

    let data = match &args[0].kind {
        ValueKind::String(s) => s.as_bytes(),
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();

    Ok(Value::string(hex::encode(result)))
}

/// MD5 hash (16 bytes) - DEPRECATED, for legacy compatibility only
fn md5(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(GraphoidError::runtime("md5() requires 1 argument: data".to_string()));
    }

    let data = match &args[0].kind {
        ValueKind::String(s) => s.as_bytes(),
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let mut hasher = Md5::new();
    hasher.update(data);
    let result = hasher.finalize();

    Ok(Value::string(hex::encode(result)))
}

/// BLAKE2b hash (64 bytes) - Modern, fast, secure
fn blake2b(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(GraphoidError::runtime("blake2b() requires 1 argument: data".to_string()));
    }

    let data = match &args[0].kind {
        ValueKind::String(s) => s.as_bytes(),
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let mut hasher = Blake2b512::new();
    hasher.update(data);
    let result = hasher.finalize();

    Ok(Value::string(hex::encode(result)))
}

/// BLAKE3 hash (32 bytes) - Fastest cryptographic hash
fn blake3(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(GraphoidError::runtime("blake3() requires 1 argument: data".to_string()));
    }

    let data = match &args[0].kind {
        ValueKind::String(s) => s.as_bytes(),
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let mut hasher = Blake3Hasher::new();
    hasher.update(data);
    let result = hasher.finalize();

    Ok(Value::string(hex::encode(result.as_bytes())))
}

// =============================================================================
// ENCODING FUNCTIONS
// =============================================================================

/// Convert bytes to hexadecimal string
fn to_hex(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(GraphoidError::runtime("to_hex() requires 1 argument: data".to_string()));
    }

    let data = match &args[0].kind {
        ValueKind::String(s) => s.as_bytes(),
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    Ok(Value::string(hex::encode(data)))
}

/// Convert hexadecimal string to bytes
fn from_hex(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(GraphoidError::runtime("from_hex() requires 1 argument: hex_string".to_string()));
    }

    let hex_str = match &args[0].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let bytes = hex::decode(hex_str)
        .map_err(|e| GraphoidError::runtime(format!("Invalid hex string: {}", e)))?;

    // Convert bytes to string (may not be valid UTF-8)
    let result = String::from_utf8_lossy(&bytes).to_string();
    Ok(Value::string(result))
}

/// Convert bytes to base64 string
fn to_base64(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(GraphoidError::runtime("to_base64() requires 1 argument: data".to_string()));
    }

    let data = match &args[0].kind {
        ValueKind::String(s) => s.as_bytes(),
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    Ok(Value::string(BASE64.encode(data)))
}

/// Convert base64 string to bytes
fn from_base64(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(GraphoidError::runtime("from_base64() requires 1 argument: base64_string".to_string()));
    }

    let b64_str = match &args[0].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let bytes = BASE64.decode(b64_str)
        .map_err(|e| GraphoidError::runtime(format!("Invalid base64 string: {}", e)))?;

    let result = String::from_utf8_lossy(&bytes).to_string();
    Ok(Value::string(result))
}

// =============================================================================
// KEY GENERATION
// =============================================================================

/// Generate a random cryptographic key
fn generate_key(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(GraphoidError::runtime("generate_key() requires 1 argument: length_in_bytes".to_string()));
    }

    let length = match &args[0].kind {
        ValueKind::Number(n) => *n as usize,
        _ => return Err(GraphoidError::type_error("number", args[0].type_name())),
    };

    if length == 0 || length > 1024 {
        return Err(GraphoidError::runtime("Key length must be between 1 and 1024 bytes".to_string()));
    }

    use rand::RngCore;
    let mut key = vec![0u8; length];
    rand::thread_rng().fill_bytes(&mut key);

    Ok(Value::string(hex::encode(key)))
}

// =============================================================================
// SYMMETRIC ENCRYPTION - AES-256-GCM
// =============================================================================

/// Encrypt data using AES-256-GCM
fn aes_encrypt(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::runtime("aes_encrypt() requires 2 arguments: plaintext, key".to_string()));
    }

    let plaintext = match &args[0].kind {
        ValueKind::String(s) => s.as_bytes(),
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let key_hex = match &args[1].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[1].type_name())),
    };

    let key_bytes = hex::decode(key_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid key (must be hex): {}", e)))?;

    if key_bytes.len() != 32 {
        return Err(GraphoidError::runtime("AES-256 requires 32-byte (256-bit) key".to_string()));
    }

    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Generate random nonce (12 bytes for GCM)
    use rand::RngCore;
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext)
        .map_err(|e| GraphoidError::runtime(format!("Encryption failed: {}", e)))?;

    // Return nonce + ciphertext as hex
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);

    Ok(Value::string(hex::encode(result)))
}

/// Decrypt data using AES-256-GCM
fn aes_decrypt(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::runtime("aes_decrypt() requires 2 arguments: ciphertext, key".to_string()));
    }

    let ciphertext_hex = match &args[0].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let key_hex = match &args[1].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[1].type_name())),
    };

    let combined = hex::decode(ciphertext_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid ciphertext (must be hex): {}", e)))?;

    if combined.len() < 12 {
        return Err(GraphoidError::runtime("Invalid ciphertext (too short)".to_string()));
    }

    let key_bytes = hex::decode(key_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid key (must be hex): {}", e)))?;

    if key_bytes.len() != 32 {
        return Err(GraphoidError::runtime("AES-256 requires 32-byte (256-bit) key".to_string()));
    }

    // Extract nonce and ciphertext
    let nonce = Nonce::from_slice(&combined[..12]);
    let ciphertext = &combined[12..];

    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| GraphoidError::runtime(format!("Decryption failed: {}", e)))?;

    let result = String::from_utf8_lossy(&plaintext).to_string();
    Ok(Value::string(result))
}

// =============================================================================
// SYMMETRIC ENCRYPTION - ChaCha20-Poly1305
// =============================================================================

/// Encrypt data using ChaCha20-Poly1305
fn chacha20_encrypt(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::runtime("chacha20_encrypt() requires 2 arguments: plaintext, key".to_string()));
    }

    let plaintext = match &args[0].kind {
        ValueKind::String(s) => s.as_bytes(),
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let key_hex = match &args[1].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[1].type_name())),
    };

    let key_bytes = hex::decode(key_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid key (must be hex): {}", e)))?;

    if key_bytes.len() != 32 {
        return Err(GraphoidError::runtime("ChaCha20 requires 32-byte (256-bit) key".to_string()));
    }

    let key = ChaChaKey::from_slice(&key_bytes);
    let cipher = ChaCha20Poly1305::new(key);

    // Generate random nonce (12 bytes)
    use rand::RngCore;
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = chacha20poly1305::Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext)
        .map_err(|e| GraphoidError::runtime(format!("Encryption failed: {}", e)))?;

    // Return nonce + ciphertext as hex
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);

    Ok(Value::string(hex::encode(result)))
}

/// Decrypt data using ChaCha20-Poly1305
fn chacha20_decrypt(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::runtime("chacha20_decrypt() requires 2 arguments: ciphertext, key".to_string()));
    }

    let ciphertext_hex = match &args[0].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let key_hex = match &args[1].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[1].type_name())),
    };

    let combined = hex::decode(ciphertext_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid ciphertext (must be hex): {}", e)))?;

    if combined.len() < 12 {
        return Err(GraphoidError::runtime("Invalid ciphertext (too short)".to_string()));
    }

    let key_bytes = hex::decode(key_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid key (must be hex): {}", e)))?;

    if key_bytes.len() != 32 {
        return Err(GraphoidError::runtime("ChaCha20 requires 32-byte (256-bit) key".to_string()));
    }

    // Extract nonce and ciphertext
    let nonce = chacha20poly1305::Nonce::from_slice(&combined[..12]);
    let ciphertext = &combined[12..];

    let key = ChaChaKey::from_slice(&key_bytes);
    let cipher = ChaCha20Poly1305::new(key);

    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| GraphoidError::runtime(format!("Decryption failed: {}", e)))?;

    let result = String::from_utf8_lossy(&plaintext).to_string();
    Ok(Value::string(result))
}

// =============================================================================
// HMAC - Message Authentication
// =============================================================================

/// Compute HMAC-SHA256
fn hmac_sha256(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::runtime("hmac_sha256() requires 2 arguments: message, key".to_string()));
    }

    let message = match &args[0].kind {
        ValueKind::String(s) => s.as_bytes(),
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let key = match &args[1].kind {
        ValueKind::String(s) => s.as_bytes(),
        _ => return Err(GraphoidError::type_error("string", args[1].type_name())),
    };

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = <HmacSha256 as HmacMac>::new_from_slice(key)
        .map_err(|e| GraphoidError::runtime(format!("Invalid key: {}", e)))?;

    <HmacSha256 as HmacMac>::update(&mut mac, message);
    let result = <HmacSha256 as HmacMac>::finalize(mac);
    let code_bytes = result.into_bytes();

    Ok(Value::string(hex::encode(code_bytes)))
}

/// Verify HMAC-SHA256
fn hmac_verify(args: &[Value]) -> Result<Value> {
    if args.len() != 3 {
        return Err(GraphoidError::runtime("hmac_verify() requires 3 arguments: message, mac, key".to_string()));
    }

    let message = match &args[0].kind {
        ValueKind::String(s) => s.as_bytes(),
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let mac_hex = match &args[1].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[1].type_name())),
    };

    let key = match &args[2].kind {
        ValueKind::String(s) => s.as_bytes(),
        _ => return Err(GraphoidError::type_error("string", args[2].type_name())),
    };

    let mac_bytes = hex::decode(mac_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid MAC (must be hex): {}", e)))?;

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = <HmacSha256 as HmacMac>::new_from_slice(key)
        .map_err(|e| GraphoidError::runtime(format!("Invalid key: {}", e)))?;

    <HmacSha256 as HmacMac>::update(&mut mac, message);

    match <HmacSha256 as HmacMac>::verify_slice(mac, &mac_bytes) {
        Ok(_) => Ok(Value::boolean(true)),
        Err(_) => Ok(Value::boolean(false)),
    }
}

// =============================================================================
// KEY DERIVATION - PBKDF2
// =============================================================================

/// Derive key from password using PBKDF2-HMAC-SHA256
fn pbkdf2_fn(args: &[Value]) -> Result<Value> {
    if args.len() < 2 || args.len() > 3 {
        return Err(GraphoidError::runtime("pbkdf2() requires 2-3 arguments: password, salt, [iterations=100000]".to_string()));
    }

    let password = match &args[0].kind {
        ValueKind::String(s) => s.as_bytes(),
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let salt = match &args[1].kind {
        ValueKind::String(s) => s.as_bytes(),
        _ => return Err(GraphoidError::type_error("string", args[1].type_name())),
    };

    let iterations = if args.len() == 3 {
        match &args[2].kind {
            ValueKind::Number(n) => *n as u32,
            _ => return Err(GraphoidError::type_error("number", args[2].type_name())),
        }
    } else {
        100_000 // Default: 100,000 iterations
    };

    // Derive 32-byte key
    let key: [u8; 32] = pbkdf2_hmac_array::<Sha256, 32>(password, salt, iterations);

    Ok(Value::string(hex::encode(key)))
}

// =============================================================================
// DIGITAL SIGNATURES - Ed25519
// =============================================================================

/// Generate Ed25519 keypair
fn generate_keypair(_args: &[Value]) -> Result<Value> {
    use rand::RngCore;
    let mut rng = rand::rngs::OsRng;
    let mut secret_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_bytes);

    let signing_key = SigningKey::from_bytes(&secret_bytes);
    let verifying_key = signing_key.verifying_key();

    let mut result = Hash::new();
    result.insert("public".to_string(), Value::string(hex::encode(verifying_key.as_bytes())));
    result.insert("secret".to_string(), Value::string(hex::encode(signing_key.to_bytes())));

    Ok(Value::map(result))
}

/// Sign message with Ed25519 secret key
fn sign(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::runtime("sign() requires 2 arguments: message, secret_key".to_string()));
    }

    let message = match &args[0].kind {
        ValueKind::String(s) => s.as_bytes(),
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let secret_hex = match &args[1].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[1].type_name())),
    };

    let secret_bytes = hex::decode(secret_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid secret key (must be hex): {}", e)))?;

    if secret_bytes.len() != 32 {
        return Err(GraphoidError::runtime("Ed25519 secret key must be 32 bytes".to_string()));
    }

    let mut secret_array = [0u8; 32];
    secret_array.copy_from_slice(&secret_bytes);
    let signing_key = SigningKey::from_bytes(&secret_array);

    let signature = signing_key.sign(message);

    Ok(Value::string(hex::encode(signature.to_bytes())))
}

/// Verify Ed25519 signature
fn verify(args: &[Value]) -> Result<Value> {
    if args.len() != 3 {
        return Err(GraphoidError::runtime("verify() requires 3 arguments: message, signature, public_key".to_string()));
    }

    let message = match &args[0].kind {
        ValueKind::String(s) => s.as_bytes(),
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let sig_hex = match &args[1].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[1].type_name())),
    };

    let pub_hex = match &args[2].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[2].type_name())),
    };

    let sig_bytes = hex::decode(sig_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid signature (must be hex): {}", e)))?;

    if sig_bytes.len() != 64 {
        return Err(GraphoidError::runtime("Ed25519 signature must be 64 bytes".to_string()));
    }

    let pub_bytes = hex::decode(pub_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid public key (must be hex): {}", e)))?;

    if pub_bytes.len() != 32 {
        return Err(GraphoidError::runtime("Ed25519 public key must be 32 bytes".to_string()));
    }

    let mut sig_array = [0u8; 64];
    sig_array.copy_from_slice(&sig_bytes);
    let signature = Signature::from_bytes(&sig_array);

    let mut pub_array = [0u8; 32];
    pub_array.copy_from_slice(&pub_bytes);
    let verifying_key = VerifyingKey::from_bytes(&pub_array)
        .map_err(|e| GraphoidError::runtime(format!("Invalid public key: {}", e)))?;

    match verifying_key.verify(message, &signature) {
        Ok(_) => Ok(Value::boolean(true)),
        Err(_) => Ok(Value::boolean(false)),
    }
}

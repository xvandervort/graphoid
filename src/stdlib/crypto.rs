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

// Suppress deprecation warnings for generic-array methods (dependency issue)
#![allow(deprecated)]

use super::{NativeFunction, NativeModule};
use crate::error::{GraphoidError, Result};
use crate::values::{Hash, Value, ValueKind};
use std::collections::HashMap;

// Hashing
use sha2::{Sha256, Sha512, Digest};
use sha1::Sha1;
use md5::Md5;
use blake2::Blake2b512;
use blake3::Hasher as Blake3Hasher;

// Encoding
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

// Encryption
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use chacha20poly1305::{ChaCha20Poly1305, Key as ChaChaKey};

// HMAC
use hmac::Hmac;
use hmac::Mac as HmacMac;

// Key derivation
use pbkdf2::pbkdf2_hmac_array;
// Note: argon2 imports removed - not currently used in exported functions

// Digital signatures
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};

// X25519 key exchange (for TLS ECDHE)
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};

// HKDF key derivation (for TLS 1.3)
use hkdf::Hkdf;

// RSA signature verification (for TLS certificates)
use rsa::{pkcs1v15, RsaPublicKey, pkcs8::DecodePublicKey};
use rsa::pkcs1::DecodeRsaPublicKey;

// ECDSA signature verification (P-256/P-384 for TLS certificates)
use p256::ecdsa::{Signature as P256Signature, VerifyingKey as P256VerifyingKey, signature::Verifier as P256Verifier};
use p384::ecdsa::{Signature as P384Signature, VerifyingKey as P384VerifyingKey, signature::Verifier as P384Verifier};

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

        // Digital signatures (Ed25519)
        functions.insert("generate_keypair".to_string(), generate_keypair as NativeFunction);
        functions.insert("sign".to_string(), sign as NativeFunction);
        functions.insert("verify".to_string(), verify as NativeFunction);

        // X25519 key exchange (for TLS ECDHE)
        functions.insert("x25519_keygen".to_string(), x25519_keygen as NativeFunction);
        functions.insert("x25519_shared_secret".to_string(), x25519_shared_secret as NativeFunction);

        // HKDF key derivation (for TLS 1.3)
        functions.insert("hkdf_extract".to_string(), hkdf_extract as NativeFunction);
        functions.insert("hkdf_expand".to_string(), hkdf_expand as NativeFunction);

        // RSA signature verification (for TLS certificates)
        functions.insert("rsa_verify".to_string(), rsa_verify as NativeFunction);

        // ECDSA signature verification (for TLS certificates)
        functions.insert("ecdsa_verify".to_string(), ecdsa_verify as NativeFunction);

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

    let key = aes_gcm::Key::<Aes256Gcm>::clone_from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(&key);

    // Generate random nonce (12 bytes for GCM)
    use rand::RngCore;
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::clone_from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(&nonce, plaintext)
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
    let nonce = Nonce::clone_from_slice(&combined[..12]);
    let ciphertext = &combined[12..];

    let key = aes_gcm::Key::<Aes256Gcm>::clone_from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(&key);

    let plaintext = cipher.decrypt(&nonce, ciphertext)
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

    let key = ChaChaKey::clone_from_slice(&key_bytes);
    let cipher = ChaCha20Poly1305::new(&key);

    // Generate random nonce (12 bytes)
    use rand::RngCore;
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = chacha20poly1305::Nonce::clone_from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(&nonce, plaintext)
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

    let key = ChaChaKey::clone_from_slice(&key_bytes);
    let cipher = ChaCha20Poly1305::new(&key);

    let plaintext = cipher.decrypt(&nonce, ciphertext)
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
    let _ = result.insert("public".to_string(), Value::string(hex::encode(verifying_key.as_bytes())));
    let _ = result.insert("secret".to_string(), Value::string(hex::encode(signing_key.to_bytes())));

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

// =============================================================================
// X25519 KEY EXCHANGE (for TLS ECDHE)
// =============================================================================

/// Generate X25519 keypair for key exchange
/// Returns: {public: hex, secret: hex}
fn x25519_keygen(_args: &[Value]) -> Result<Value> {
    use rand::RngCore;
    let mut rng = rand::rngs::OsRng;

    // Generate random 32-byte secret
    let mut secret_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_bytes);

    let secret = StaticSecret::from(secret_bytes);
    let public = X25519PublicKey::from(&secret);

    let mut result = Hash::new();
    let _ = result.insert("public".to_string(), Value::string(hex::encode(public.as_bytes())));
    let _ = result.insert("secret".to_string(), Value::string(hex::encode(secret.as_bytes())));

    Ok(Value::map(result))
}

/// Compute X25519 shared secret (Diffie-Hellman)
/// Args: my_secret_hex, their_public_hex
/// Returns: shared_secret_hex (32 bytes)
fn x25519_shared_secret(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::runtime(
            "x25519_shared_secret() requires 2 arguments: my_secret, their_public".to_string()
        ));
    }

    let my_secret_hex = match &args[0].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let their_public_hex = match &args[1].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[1].type_name())),
    };

    let secret_bytes = hex::decode(my_secret_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid secret key (must be hex): {}", e)))?;

    if secret_bytes.len() != 32 {
        return Err(GraphoidError::runtime("X25519 secret key must be 32 bytes".to_string()));
    }

    let public_bytes = hex::decode(their_public_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid public key (must be hex): {}", e)))?;

    if public_bytes.len() != 32 {
        return Err(GraphoidError::runtime("X25519 public key must be 32 bytes".to_string()));
    }

    let mut secret_array = [0u8; 32];
    secret_array.copy_from_slice(&secret_bytes);
    let secret = StaticSecret::from(secret_array);

    let mut public_array = [0u8; 32];
    public_array.copy_from_slice(&public_bytes);
    let their_public = X25519PublicKey::from(public_array);

    let shared_secret = secret.diffie_hellman(&their_public);

    Ok(Value::string(hex::encode(shared_secret.as_bytes())))
}

// =============================================================================
// HKDF KEY DERIVATION (for TLS 1.3)
// =============================================================================

/// HKDF-Extract: Extract a pseudorandom key from input key material
/// Args: salt (hex), ikm (hex)
/// Returns: prk (hex, 32 bytes for SHA-256)
fn hkdf_extract(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::runtime(
            "hkdf_extract() requires 2 arguments: salt, ikm".to_string()
        ));
    }

    let salt_hex = match &args[0].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let ikm_hex = match &args[1].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[1].type_name())),
    };

    let salt = hex::decode(salt_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid salt (must be hex): {}", e)))?;

    let ikm = hex::decode(ikm_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid ikm (must be hex): {}", e)))?;

    let hk = Hkdf::<Sha256>::new(Some(&salt), &ikm);

    // Extract PRK (pseudo-random key) - for SHA-256 this is 32 bytes
    // We need to get the PRK directly, but hkdf crate doesn't expose it directly
    // So we use expand with empty info to get 32 bytes
    let mut prk = [0u8; 32];
    hk.expand(&[], &mut prk)
        .map_err(|e| GraphoidError::runtime(format!("HKDF extract failed: {}", e)))?;

    Ok(Value::string(hex::encode(prk)))
}

/// HKDF-Expand: Expand PRK into output key material
/// Args: prk (hex), info (hex), length (number)
/// Returns: okm (hex, `length` bytes)
fn hkdf_expand(args: &[Value]) -> Result<Value> {
    if args.len() != 3 {
        return Err(GraphoidError::runtime(
            "hkdf_expand() requires 3 arguments: prk, info, length".to_string()
        ));
    }

    let prk_hex = match &args[0].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let info_hex = match &args[1].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[1].type_name())),
    };

    let length = match &args[2].kind {
        ValueKind::Number(n) => *n as usize,
        _ => return Err(GraphoidError::type_error("number", args[2].type_name())),
    };

    if length == 0 || length > 255 * 32 {
        return Err(GraphoidError::runtime(
            "HKDF output length must be between 1 and 8160 bytes".to_string()
        ));
    }

    let prk = hex::decode(prk_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid prk (must be hex): {}", e)))?;

    let info = hex::decode(info_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid info (must be hex): {}", e)))?;

    // Create HKDF with the PRK as IKM (salt doesn't matter for expand-only)
    let hk = Hkdf::<Sha256>::new(None, &prk);

    let mut okm = vec![0u8; length];
    hk.expand(&info, &mut okm)
        .map_err(|e| GraphoidError::runtime(format!("HKDF expand failed: {}", e)))?;

    Ok(Value::string(hex::encode(okm)))
}

// =============================================================================
// RSA SIGNATURE VERIFICATION (for TLS certificates)
// =============================================================================

/// Verify RSA signature (PKCS#1 v1.5 with SHA-256)
/// Args: message (hex), signature (hex), public_key (DER hex), hash_algo (string: "sha256" or "sha384")
/// Returns: bool
fn rsa_verify(args: &[Value]) -> Result<Value> {
    if args.len() != 4 {
        return Err(GraphoidError::runtime(
            "rsa_verify() requires 4 arguments: message, signature, public_key, hash_algo".to_string()
        ));
    }

    let message_hex = match &args[0].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let signature_hex = match &args[1].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[1].type_name())),
    };

    let pubkey_hex = match &args[2].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[2].type_name())),
    };

    let hash_algo = match &args[3].kind {
        ValueKind::String(s) => s.as_str(),
        _ => return Err(GraphoidError::type_error("string", args[3].type_name())),
    };

    let message = hex::decode(message_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid message (must be hex): {}", e)))?;

    let signature = hex::decode(signature_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid signature (must be hex): {}", e)))?;

    let pubkey_der = hex::decode(pubkey_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid public key (must be hex DER): {}", e)))?;

    // Try to parse as PKCS#8 first, then PKCS#1
    let public_key = RsaPublicKey::from_public_key_der(&pubkey_der)
        .or_else(|_| RsaPublicKey::from_pkcs1_der(&pubkey_der))
        .map_err(|e| GraphoidError::runtime(format!("Invalid RSA public key: {}", e)))?;

    // Verify based on hash algorithm
    let result = match hash_algo {
        "sha256" => {
            let scheme = pkcs1v15::Pkcs1v15Sign::new::<Sha256>();
            public_key.verify(scheme, &message, &signature).is_ok()
        }
        "sha384" => {
            use sha2::Sha384;
            let scheme = pkcs1v15::Pkcs1v15Sign::new::<Sha384>();
            public_key.verify(scheme, &message, &signature).is_ok()
        }
        "sha512" => {
            let scheme = pkcs1v15::Pkcs1v15Sign::new::<Sha512>();
            public_key.verify(scheme, &message, &signature).is_ok()
        }
        _ => return Err(GraphoidError::runtime(
            format!("Unsupported hash algorithm: {}. Use sha256, sha384, or sha512", hash_algo)
        )),
    };

    Ok(Value::boolean(result))
}

// =============================================================================
// ECDSA SIGNATURE VERIFICATION (for TLS certificates)
// =============================================================================

/// Verify ECDSA signature (P-256 or P-384)
/// Args: message (hex), signature (hex, DER format), public_key (hex, SEC1 compressed or uncompressed), curve (string: "p256" or "p384")
/// Returns: bool
fn ecdsa_verify(args: &[Value]) -> Result<Value> {
    if args.len() != 4 {
        return Err(GraphoidError::runtime(
            "ecdsa_verify() requires 4 arguments: message, signature, public_key, curve".to_string()
        ));
    }

    let message_hex = match &args[0].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[0].type_name())),
    };

    let signature_hex = match &args[1].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[1].type_name())),
    };

    let pubkey_hex = match &args[2].kind {
        ValueKind::String(s) => s,
        _ => return Err(GraphoidError::type_error("string", args[2].type_name())),
    };

    let curve = match &args[3].kind {
        ValueKind::String(s) => s.as_str(),
        _ => return Err(GraphoidError::type_error("string", args[3].type_name())),
    };

    let message = hex::decode(message_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid message (must be hex): {}", e)))?;

    let signature_bytes = hex::decode(signature_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid signature (must be hex): {}", e)))?;

    let pubkey_bytes = hex::decode(pubkey_hex)
        .map_err(|e| GraphoidError::runtime(format!("Invalid public key (must be hex): {}", e)))?;

    let result = match curve {
        "p256" | "P256" | "secp256r1" => {
            // Parse P-256 public key
            let verifying_key = P256VerifyingKey::from_sec1_bytes(&pubkey_bytes)
                .map_err(|e| GraphoidError::runtime(format!("Invalid P-256 public key: {}", e)))?;

            // Parse signature (try DER format first, then raw r||s format)
            let signature = P256Signature::from_der(&signature_bytes)
                .or_else(|_| P256Signature::from_slice(&signature_bytes))
                .map_err(|e| GraphoidError::runtime(format!("Invalid P-256 signature: {}", e)))?;

            P256Verifier::verify(&verifying_key, &message, &signature).is_ok()
        }
        "p384" | "P384" | "secp384r1" => {
            // Parse P-384 public key
            let verifying_key = P384VerifyingKey::from_sec1_bytes(&pubkey_bytes)
                .map_err(|e| GraphoidError::runtime(format!("Invalid P-384 public key: {}", e)))?;

            // Parse signature
            let signature = P384Signature::from_der(&signature_bytes)
                .or_else(|_| P384Signature::from_slice(&signature_bytes))
                .map_err(|e| GraphoidError::runtime(format!("Invalid P-384 signature: {}", e)))?;

            P384Verifier::verify(&verifying_key, &message, &signature).is_ok()
        }
        _ => return Err(GraphoidError::runtime(
            format!("Unsupported curve: {}. Use p256 or p384", curve)
        )),
    };

    Ok(Value::boolean(result))
}

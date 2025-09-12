# Crypto Module

The crypto module provides cryptographic operations for secure data handling, including hashing, encryption, random number generation, and format conversion.

## Importing

```glang
import "crypto"
```

## Important Notes

- All crypto functions work with **byte arrays** (lists of numbers 0-255)
- Strings must be converted to byte arrays before processing
- The module uses industry-standard algorithms (AES-256, SHA-256, etc.)
- All random generation uses cryptographically secure sources

## Hashing Functions

### hash_md5(data)
Computes MD5 hash of byte data. Returns 16-byte hash.

⚠️ **Warning**: MD5 is cryptographically broken. Use only for compatibility with legacy systems.

```glang
# Hash byte data
data = [72, 101, 108, 108, 111]  # "Hello" in ASCII
hash = crypto.hash_md5(data)     # Returns 16-byte list
hex_hash = crypto.to_hex(hash)   # Convert to hex string
```

### hash_sha1(data)
Computes SHA-1 hash of byte data. Returns 20-byte hash.

⚠️ **Warning**: SHA-1 is deprecated for security. Use SHA-256 or SHA-512 for new applications.

```glang
data = [116, 101, 115, 116]  # "test"
hash = crypto.hash_sha1(data)  # Returns 20-byte list
```

### hash_sha256(data)
Computes SHA-256 hash of byte data. Returns 32-byte hash.

✅ **Recommended** for most hashing needs.

```glang
message = [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]  # "Hello World"
hash = crypto.hash_sha256(message)  # Returns 32-byte list
hex_hash = crypto.to_hex(hash)
print("SHA-256: " + hex_hash)
```

### hash_sha512(data)
Computes SHA-512 hash of byte data. Returns 64-byte hash.

✅ **Most secure** hashing option, use for high-security applications.

```glang
sensitive_data = [1, 2, 3, 4, 5]
hash = crypto.hash_sha512(sensitive_data)  # Returns 64-byte list
```

## Random Generation

### random_bytes(length)
Generates cryptographically secure random bytes.

```glang
# Generate random data
salt = crypto.random_bytes(16)      # 16 random bytes for salt
key = crypto.random_bytes(32)       # 32 bytes for AES-256 key
nonce = crypto.random_bytes(12)     # 12 bytes for GCM nonce
session_id = crypto.random_bytes(24) # 24 bytes for session ID

# Convert to hex for storage/display
key_hex = crypto.to_hex(key)
print("Generated key: " + key_hex)
```

## Symmetric Encryption (AES)

### AES-CBC Mode (Basic)

#### aes_encrypt(data, key)
Encrypts data using AES-256-CBC. Key must be exactly 32 bytes.
Returns IV (16 bytes) + ciphertext.

```glang
# Prepare data and key
plaintext = [72, 101, 108, 108, 111]  # "Hello"
key = crypto.random_bytes(32)          # Generate 256-bit key

# Encrypt
encrypted = crypto.aes_encrypt(plaintext, key)
# Returns: [IV(16 bytes)][Ciphertext]

# Save key for decryption
key_hex = crypto.to_hex(key)
print("Save this key: " + key_hex)
```

#### aes_decrypt(encrypted_data, key)
Decrypts data encrypted with aes_encrypt. Key must be exactly 32 bytes.

```glang
# Use same key from encryption
decrypted = crypto.aes_decrypt(encrypted, key)
# Returns original plaintext
```

### AES-GCM Mode (Authenticated Encryption)

#### aes_gcm_encrypt(data, key)
Encrypts data using AES-256-GCM with authentication. Key must be exactly 32 bytes.
Returns nonce (12 bytes) + ciphertext + auth_tag (16 bytes).

✅ **Recommended** for new applications - provides both encryption and authentication.

```glang
# Encrypt with authentication
plaintext = [1, 2, 3, 4, 5]
key = crypto.random_bytes(32)

encrypted = crypto.aes_gcm_encrypt(plaintext, key)
# Returns: [Nonce(12)][Ciphertext][AuthTag(16)]

# GCM provides integrity verification
# Any tampering will be detected during decryption
```

#### aes_gcm_decrypt(encrypted_data, key)
Decrypts and authenticates data encrypted with aes_gcm_encrypt.
Throws error if authentication fails (data was tampered).

```glang
# Decrypt and verify
decrypted = crypto.aes_gcm_decrypt(encrypted, key)
# Returns original data only if authentic
# Throws error if tampered
```

## Message Authentication

### hmac_sha256(data, key)
Computes HMAC-SHA256 message authentication code. Returns 32-byte MAC.

```glang
# Create message authentication code
message = [72, 101, 108, 108, 111]  # "Hello"
secret_key = crypto.random_bytes(32)

mac = crypto.hmac_sha256(message, secret_key)
mac_hex = crypto.to_hex(mac)
print("HMAC: " + mac_hex)

# Verify message integrity
received_mac = crypto.hmac_sha256(received_message, secret_key)
if received_mac == expected_mac {
    print("Message is authentic")
}
```

## Key Derivation

### hkdf_expand(prk, length, info)
Expands key material using HKDF (HMAC-based Key Derivation Function).
- `prk`: Pseudo-random key (input key material)
- `length`: Desired output length (max 8160 bytes)
- `info`: Optional context/info bytes or string

```glang
# Derive multiple keys from master key
master_key = crypto.random_bytes(32)

# Derive encryption key
enc_key = crypto.hkdf_expand(master_key, 32, "encryption")

# Derive authentication key
auth_key = crypto.hkdf_expand(master_key, 32, "authentication")

# Derive with numeric info
session_key = crypto.hkdf_expand(master_key, 32, [1, 2, 3, 4])
```

## Asymmetric Encryption (RSA)

### rsa_generate_keypair(key_size)
Generates RSA public/private key pair. Key size must be 2048, 3072, or 4096.
Returns hash with "private" and "public" keys as DER-encoded byte lists.

```glang
# Generate RSA keypair
keypair = crypto.rsa_generate_keypair(2048)
private_key = keypair["private"].value()  # DER-encoded private key
public_key = keypair["public"].value()    # DER-encoded public key

# Save keys
private_hex = crypto.to_hex(private_key)
public_hex = crypto.to_hex(public_key)
```

### rsa_encrypt(data, public_key)
Encrypts data using RSA public key with OAEP padding.

```glang
# Encrypt with public key
message = [72, 101, 108, 108, 111]  # "Hello"
ciphertext = crypto.rsa_encrypt(message, public_key)

# Only private key holder can decrypt
```

### rsa_decrypt(ciphertext, private_key)
Decrypts data using RSA private key.

```glang
# Decrypt with private key
plaintext = crypto.rsa_decrypt(ciphertext, private_key)
# Returns original message
```

## Elliptic Curve Diffie-Hellman (ECDH)

### ecdh_generate_keypair(curve_name)
Generates ECDH key pair. Supported curves: "P-256", "P-384", "P-521".
Returns hash with "private", "public", and "curve" fields.

```glang
# Generate ECDH keypair
keypair = crypto.ecdh_generate_keypair("P-256")
private_key = keypair["private"].value()
public_key = keypair["public"].value()
curve = keypair["curve"].value()  # "P-256"
```

### ecdh_compute_shared_secret(private_key, peer_public_key)
Computes shared secret from private key and peer's public key.

```glang
# Alice generates keypair
alice_keys = crypto.ecdh_generate_keypair("P-256")
alice_private = alice_keys["private"].value()
alice_public = alice_keys["public"].value()

# Bob generates keypair
bob_keys = crypto.ecdh_generate_keypair("P-256")
bob_private = bob_keys["private"].value()
bob_public = bob_keys["public"].value()

# Both compute same shared secret
alice_secret = crypto.ecdh_compute_shared_secret(alice_private, bob_public)
bob_secret = crypto.ecdh_compute_shared_secret(bob_private, alice_public)

# alice_secret == bob_secret (same shared key)
```

### ecdh_public_key_from_private(private_key)
Extracts public key from ECDH private key.

```glang
# Recover public key from private
private = keypair["private"].value()
public = crypto.ecdh_public_key_from_private(private)
```

## Format Conversion

### to_hex(data)
Converts byte array to hexadecimal string.

```glang
bytes = [255, 171, 205]
hex_string = crypto.to_hex(bytes)  # Returns "ffabcd"

# Useful for displaying binary data
hash = crypto.hash_sha256(data)
print("Hash: " + crypto.to_hex(hash))
```

### from_hex(hex_string)
Converts hexadecimal string to byte array.

```glang
hex_string = "ffabcd"
bytes = crypto.from_hex(hex_string)  # Returns [255, 171, 205]

# Load saved keys
key_hex = "0123456789abcdef..."
key = crypto.from_hex(key_hex)
```

### to_base64(data)
Converts byte array to base64 string.

```glang
data = [72, 101, 108, 108, 111]  # "Hello"
b64 = crypto.to_base64(data)     # Returns "SGVsbG8="

# Useful for text-safe encoding
encrypted = crypto.aes_encrypt(data, key)
encrypted_b64 = crypto.to_base64(encrypted)
print("Encrypted (Base64): " + encrypted_b64)
```

### from_base64(base64_string)
Converts base64 string to byte array.

```glang
b64 = "SGVsbG8="
data = crypto.from_base64(b64)  # Returns [72, 101, 108, 108, 111]

# Decode received data
received_b64 = "U29tZSBkYXRh"
received = crypto.from_base64(received_b64)
```

## Complete Examples

### Password Hashing
```glang
import "crypto"

func hash_password(password_bytes, salt) {
    # Combine password and salt
    combined = password_bytes + salt
    
    # Hash multiple times for security
    hash = crypto.hash_sha256(combined)
    for i in [1, 2, 3, 4, 5] {
        hash = crypto.hash_sha256(hash)
    }
    
    return hash
}

# Hash a password
password = [112, 97, 115, 115, 119, 111, 114, 100]  # "password"
salt = crypto.random_bytes(16)

password_hash = hash_password(password, salt)
stored = crypto.to_hex(salt) + ":" + crypto.to_hex(password_hash)
print("Store this: " + stored)
```

### File Encryption
```glang
import "crypto"
import "io"

# Encrypt a file
func encrypt_file(input_path, output_path, key) {
    # Read file
    data = io.read_binary(input_path)
    
    # Encrypt with AES-GCM
    encrypted = crypto.aes_gcm_encrypt(data, key)
    
    # Save encrypted file
    io.write_binary(output_path, encrypted)
    
    return true
}

# Decrypt a file
func decrypt_file(input_path, output_path, key) {
    # Read encrypted file
    encrypted = io.read_binary(input_path)
    
    # Decrypt
    data = crypto.aes_gcm_decrypt(encrypted, key)
    
    # Save decrypted file
    io.write_binary(output_path, data)
    
    return true
}

# Use encryption
key = crypto.random_bytes(32)
encrypt_file("document.txt", "document.enc", key)
decrypt_file("document.enc", "document_dec.txt", key)

# Save key for later use
key_hex = crypto.to_hex(key)
io.write_file("key.txt", key_hex)
```

### Digital Signatures (using HMAC)
```glang
import "crypto"

func sign_message(message, secret_key) {
    # Create signature
    signature = crypto.hmac_sha256(message, secret_key)
    
    # Return message with signature
    return {
        "message": message,
        "signature": signature
    }
}

func verify_signature(signed_data, secret_key) {
    message = signed_data["message"].value()
    signature = signed_data["signature"].value()
    
    # Recompute signature
    expected = crypto.hmac_sha256(message, secret_key)
    
    # Compare signatures
    return signature == expected
}

# Sign and verify
secret = crypto.random_bytes(32)
message = [72, 101, 108, 108, 111]  # "Hello"

signed = sign_message(message, secret)
is_valid = verify_signature(signed, secret)
print("Signature valid: " + is_valid.to_string())
```

### Secure Key Exchange
```glang
import "crypto"

# Alice's side
alice_ecdh = crypto.ecdh_generate_keypair("P-256")
alice_private = alice_ecdh["private"].value()
alice_public = alice_ecdh["public"].value()

# Send alice_public to Bob...

# Bob's side
bob_ecdh = crypto.ecdh_generate_keypair("P-256")
bob_private = bob_ecdh["private"].value()
bob_public = bob_ecdh["public"].value()

# Send bob_public to Alice...

# Both compute shared secret
alice_shared = crypto.ecdh_compute_shared_secret(alice_private, bob_public)
bob_shared = crypto.ecdh_compute_shared_secret(bob_private, alice_public)

# Derive encryption keys from shared secret
alice_key = crypto.hkdf_expand(alice_shared, 32, "encryption")
bob_key = crypto.hkdf_expand(bob_shared, 32, "encryption")

# alice_key == bob_key, can now encrypt messages
```

### Data Integrity Check
```glang
import "crypto"
import "io"

func create_checksum(file_path) {
    # Read file
    data = io.read_binary(file_path)
    
    # Create checksum
    hash = crypto.hash_sha256(data)
    checksum = crypto.to_hex(hash)
    
    # Save checksum
    io.write_file(file_path + ".sha256", checksum)
    
    return checksum
}

func verify_checksum(file_path) {
    # Read file and checksum
    data = io.read_binary(file_path)
    expected = io.read_file(file_path + ".sha256").trim()
    
    # Compute actual checksum
    hash = crypto.hash_sha256(data)
    actual = crypto.to_hex(hash)
    
    # Compare
    if actual == expected {
        print("File integrity verified")
        return true
    } else {
        print("File has been modified!")
        return false
    }
}

# Create and verify
checksum = create_checksum("important.dat")
print("Checksum: " + checksum)

is_valid = verify_checksum("important.dat")
```

## Security Best Practices

1. **Use proper key sizes**:
   - AES: Always use 32 bytes (256 bits)
   - RSA: Minimum 2048 bits, prefer 3072 or 4096
   - ECDH: P-256 is sufficient for most uses

2. **Choose secure algorithms**:
   - Hashing: Use SHA-256 or SHA-512
   - Encryption: Use AES-GCM for authenticated encryption
   - Key exchange: Use ECDH or RSA

3. **Generate strong random values**:
   - Always use `crypto.random_bytes()` for keys, salts, and nonces
   - Never use predictable values

4. **Store keys securely**:
   - Never hardcode keys in source code
   - Store keys in separate files with restricted permissions
   - Consider key derivation from master keys

5. **Validate and authenticate**:
   - Use AES-GCM or add HMAC for authentication
   - Always verify signatures and checksums
   - Handle decryption failures gracefully

6. **Keep keys separate**:
   - Use different keys for different purposes
   - Derive keys using HKDF when needed
   - Rotate keys regularly

## Common Patterns

### String to Bytes Conversion
Until Glang has native string-to-bytes conversion:
```glang
# Manual ASCII conversion
"Hello" = [72, 101, 108, 108, 111]
"World" = [87, 111, 114, 108, 100]
"test" = [116, 101, 115, 116]
```

### Hex Strings for Storage
```glang
# Save binary data as hex
key = crypto.random_bytes(32)
key_hex = crypto.to_hex(key)
io.write_file("key.txt", key_hex)

# Load binary data from hex
saved_hex = io.read_file("key.txt").trim()
key = crypto.from_hex(saved_hex)
```

### Error Handling
```glang
# Wrap crypto operations in error handling
func safe_decrypt(data, key) {
    # Validate inputs
    if data.size() < 28 {  # Minimum for AES-GCM
        return { "error": "Invalid data size", "data": none }
    }
    
    if key.size() != 32 {
        return { "error": "Invalid key size", "data": none }
    }
    
    # Attempt decryption
    # In practice, would need try-catch
    decrypted = crypto.aes_gcm_decrypt(data, key)
    return { "error": none, "data": decrypted }
}
```
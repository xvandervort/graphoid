"""Cryptographic operations module for Glang."""

import hashlib
import os
import base64
import hmac
from typing import Optional, Any
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.primitives.asymmetric import rsa, ec, padding
from cryptography.hazmat.primitives import hashes, kdf
from cryptography.hazmat.primitives.kdf.hkdf import HKDF
from cryptography.hazmat.primitives.serialization import Encoding, PrivateFormat, NoEncryption, PublicFormat
from cryptography.hazmat.backends import default_backend

from glang.execution.values import GlangValue, StringValue, ListValue, NumberValue, HashValue, DataValue
from glang.ast.nodes import SourcePosition
from glang.modules.errors import ModuleError


class CryptoModule:
    """Cryptographic operations using Glang's built-in types."""
    
    @staticmethod
    def hash_md5(data: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Compute MD5 hash of binary data."""
        if not isinstance(data, ListValue):
            raise ModuleError("hash_md5 expects list of bytes", position)
        
        # Convert Glang list to bytes
        try:
            byte_data = bytes([
                int(item.value) for item in data.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
        except (ValueError, AttributeError):
            raise ModuleError("hash_md5 expects list of valid byte values (0-255)", position)
        
        # Compute hash
        hash_obj = hashlib.md5(byte_data)
        hash_bytes = hash_obj.digest()
        
        # Convert back to Glang list
        result = [NumberValue(byte, position) for byte in hash_bytes]
        return ListValue(result, 'num', position)
    
    @staticmethod
    def hash_sha1(data: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Compute SHA1 hash of binary data."""
        if not isinstance(data, ListValue):
            raise ModuleError("hash_sha1 expects list of bytes", position)
        
        try:
            byte_data = bytes([
                int(item.value) for item in data.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
        except (ValueError, AttributeError):
            raise ModuleError("hash_sha1 expects list of valid byte values (0-255)", position)
        
        hash_obj = hashlib.sha1(byte_data)
        hash_bytes = hash_obj.digest()
        
        result = [NumberValue(byte, position) for byte in hash_bytes]
        return ListValue(result, 'num', position)
    
    @staticmethod
    def hash_sha256(data: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Compute SHA256 hash of binary data."""
        if not isinstance(data, ListValue):
            raise ModuleError("hash_sha256 expects list of bytes", position)
        
        try:
            byte_data = bytes([
                int(item.value) for item in data.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
        except (ValueError, AttributeError):
            raise ModuleError("hash_sha256 expects list of valid byte values (0-255)", position)
        
        hash_obj = hashlib.sha256(byte_data)
        hash_bytes = hash_obj.digest()
        
        result = [NumberValue(byte, position) for byte in hash_bytes]
        return ListValue(result, 'num', position)
    
    @staticmethod
    def hash_sha512(data: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Compute SHA512 hash of binary data."""
        if not isinstance(data, ListValue):
            raise ModuleError("hash_sha512 expects list of bytes", position)
        
        try:
            byte_data = bytes([
                int(item.value) for item in data.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
        except (ValueError, AttributeError):
            raise ModuleError("hash_sha512 expects list of valid byte values (0-255)", position)
        
        hash_obj = hashlib.sha512(byte_data)
        hash_bytes = hash_obj.digest()
        
        result = [NumberValue(byte, position) for byte in hash_bytes]
        return ListValue(result, 'num', position)
    
    @staticmethod
    def random_bytes(length: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Generate cryptographically secure random bytes."""
        if not isinstance(length, NumberValue):
            raise ModuleError("random_bytes expects number for length", position)
        
        byte_length = int(length.value)
        if byte_length < 0:
            raise ModuleError("random_bytes length must be non-negative", position)
        if byte_length > 1024 * 1024:  # 1MB limit for safety
            raise ModuleError("random_bytes length too large (max 1MB)", position)
        
        # Generate cryptographically secure random bytes
        random_data = os.urandom(byte_length)
        
        # Convert to Glang list
        result = [NumberValue(byte, position) for byte in random_data]
        return ListValue(result, 'num', position)
    
    @staticmethod
    def aes_encrypt(data: GlangValue, key: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Encrypt data using AES-256-CBC."""
        if not isinstance(data, ListValue):
            raise ModuleError("aes_encrypt expects list of bytes for data", position)
        if not isinstance(key, ListValue):
            raise ModuleError("aes_encrypt expects list of bytes for key", position)
        
        # Convert data to bytes
        try:
            data_bytes = bytes([
                int(item.value) for item in data.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
        except (ValueError, AttributeError):
            raise ModuleError("aes_encrypt data must be list of valid byte values (0-255)", position)
        
        # Convert key to bytes
        try:
            key_bytes = bytes([
                int(item.value) for item in key.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
        except (ValueError, AttributeError):
            raise ModuleError("aes_encrypt key must be list of valid byte values (0-255)", position)
        
        # Validate key length (AES-256 needs 32 bytes)
        if len(key_bytes) != 32:
            raise ModuleError("aes_encrypt requires 32-byte key for AES-256", position)
        
        # Generate random IV
        iv = os.urandom(16)  # AES block size is 16 bytes
        
        # Pad data to block size (PKCS7 padding)
        padding_length = 16 - (len(data_bytes) % 16)
        padded_data = data_bytes + bytes([padding_length] * padding_length)
        
        # Encrypt
        cipher = Cipher(algorithms.AES(key_bytes), modes.CBC(iv), backend=default_backend())
        encryptor = cipher.encryptor()
        ciphertext = encryptor.update(padded_data) + encryptor.finalize()
        
        # Return IV + ciphertext as single byte array
        result_bytes = iv + ciphertext
        result = [NumberValue(byte, position) for byte in result_bytes]
        return ListValue(result, 'num', position)
    
    @staticmethod
    def aes_decrypt(encrypted_data: GlangValue, key: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Decrypt data using AES-256-CBC."""
        if not isinstance(encrypted_data, ListValue):
            raise ModuleError("aes_decrypt expects list of bytes for encrypted data", position)
        if not isinstance(key, ListValue):
            raise ModuleError("aes_decrypt expects list of bytes for key", position)
        
        # Convert encrypted data to bytes
        try:
            data_bytes = bytes([
                int(item.value) for item in encrypted_data.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
        except (ValueError, AttributeError):
            raise ModuleError("aes_decrypt data must be list of valid byte values (0-255)", position)
        
        # Convert key to bytes
        try:
            key_bytes = bytes([
                int(item.value) for item in key.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
        except (ValueError, AttributeError):
            raise ModuleError("aes_decrypt key must be list of valid byte values (0-255)", position)
        
        # Validate key length
        if len(key_bytes) != 32:
            raise ModuleError("aes_decrypt requires 32-byte key for AES-256", position)
        
        # Extract IV and ciphertext
        if len(data_bytes) < 16:
            raise ModuleError("aes_decrypt data too short (missing IV)", position)
        
        iv = data_bytes[:16]
        ciphertext = data_bytes[16:]
        
        if len(ciphertext) % 16 != 0:
            raise ModuleError("aes_decrypt invalid ciphertext length", position)
        
        # Decrypt
        cipher = Cipher(algorithms.AES(key_bytes), modes.CBC(iv), backend=default_backend())
        decryptor = cipher.decryptor()
        padded_plaintext = decryptor.update(ciphertext) + decryptor.finalize()
        
        # Remove PKCS7 padding
        if not padded_plaintext:
            raise ModuleError("aes_decrypt failed: empty plaintext", position)
        
        padding_length = padded_plaintext[-1]
        if padding_length < 1 or padding_length > 16:
            raise ModuleError("aes_decrypt failed: invalid padding", position)
        
        plaintext = padded_plaintext[:-padding_length]
        
        # Convert back to Glang list
        result = [NumberValue(byte, position) for byte in plaintext]
        return ListValue(result, 'num', position)
    
    @staticmethod
    def to_hex(data: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Convert byte array to hexadecimal string."""
        if not isinstance(data, ListValue):
            raise ModuleError("to_hex expects list of bytes", position)
        
        try:
            byte_data = bytes([
                int(item.value) for item in data.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
        except (ValueError, AttributeError):
            raise ModuleError("to_hex expects list of valid byte values (0-255)", position)
        
        hex_string = byte_data.hex()
        return StringValue(hex_string, position)
    
    @staticmethod
    def from_hex(hex_string: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Convert hexadecimal string to byte array."""
        if not isinstance(hex_string, StringValue):
            raise ModuleError("from_hex expects string", position)
        
        try:
            # Remove any whitespace and convert to bytes
            clean_hex = hex_string.value.replace(' ', '').replace('\n', '').replace('\t', '')
            byte_data = bytes.fromhex(clean_hex)
        except ValueError as e:
            raise ModuleError(f"from_hex invalid hexadecimal string: {e}", position)
        
        result = [NumberValue(byte, position) for byte in byte_data]
        return ListValue(result, 'num', position)
    
    @staticmethod
    def to_base64(data: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Convert byte array to base64 string."""
        if not isinstance(data, ListValue):
            raise ModuleError("to_base64 expects list of bytes", position)
        
        try:
            byte_data = bytes([
                int(item.value) for item in data.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
        except (ValueError, AttributeError):
            raise ModuleError("to_base64 expects list of valid byte values (0-255)", position)
        
        b64_string = base64.b64encode(byte_data).decode('ascii')
        return StringValue(b64_string, position)
    
    @staticmethod
    def from_base64(b64_string: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Convert base64 string to byte array."""
        if not isinstance(b64_string, StringValue):
            raise ModuleError("from_base64 expects string", position)
        
        try:
            byte_data = base64.b64decode(b64_string.value)
        except Exception as e:
            raise ModuleError(f"from_base64 invalid base64 string: {e}", position)
        
        result = [NumberValue(byte, position) for byte in byte_data]
        return ListValue(result, 'num', position)
    
    @staticmethod
    def aes_gcm_encrypt(data: GlangValue, key: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Encrypt data using AES-256-GCM (authenticated encryption)."""
        if not isinstance(data, ListValue):
            raise ModuleError("aes_gcm_encrypt expects list of bytes for data", position)
        if not isinstance(key, ListValue):
            raise ModuleError("aes_gcm_encrypt expects list of bytes for key", position)
        
        # Convert data to bytes
        try:
            data_bytes = bytes([
                int(item.value) for item in data.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
        except (ValueError, AttributeError):
            raise ModuleError("aes_gcm_encrypt data must be list of valid byte values (0-255)", position)
        
        # Convert key to bytes
        try:
            key_bytes = bytes([
                int(item.value) for item in key.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
        except (ValueError, AttributeError):
            raise ModuleError("aes_gcm_encrypt key must be list of valid byte values (0-255)", position)
        
        # Validate key length (AES-256 needs 32 bytes)
        if len(key_bytes) != 32:
            raise ModuleError("aes_gcm_encrypt requires 32-byte key for AES-256", position)
        
        # Generate random nonce (12 bytes for GCM)
        nonce = os.urandom(12)
        
        # Encrypt with GCM mode
        cipher = Cipher(algorithms.AES(key_bytes), modes.GCM(nonce), backend=default_backend())
        encryptor = cipher.encryptor()
        ciphertext = encryptor.update(data_bytes) + encryptor.finalize()
        
        # Return nonce + ciphertext + auth_tag as single byte array
        result_bytes = nonce + ciphertext + encryptor.tag
        result = [NumberValue(byte, position) for byte in result_bytes]
        return ListValue(result, 'num', position)
    
    @staticmethod
    def aes_gcm_decrypt(encrypted_data: GlangValue, key: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Decrypt data using AES-256-GCM (authenticated encryption)."""
        if not isinstance(encrypted_data, ListValue):
            raise ModuleError("aes_gcm_decrypt expects list of bytes for encrypted data", position)
        if not isinstance(key, ListValue):
            raise ModuleError("aes_gcm_decrypt expects list of bytes for key", position)
        
        # Convert encrypted data to bytes
        try:
            data_bytes = bytes([
                int(item.value) for item in encrypted_data.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
        except (ValueError, AttributeError):
            raise ModuleError("aes_gcm_decrypt data must be list of valid byte values (0-255)", position)
        
        # Convert key to bytes
        try:
            key_bytes = bytes([
                int(item.value) for item in key.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
        except (ValueError, AttributeError):
            raise ModuleError("aes_gcm_decrypt key must be list of valid byte values (0-255)", position)
        
        # Validate key length
        if len(key_bytes) != 32:
            raise ModuleError("aes_gcm_decrypt requires 32-byte key for AES-256", position)
        
        # Validate minimum data length (nonce + tag)
        if len(data_bytes) < 28:  # 12 (nonce) + 16 (tag) = 28 minimum
            raise ModuleError("aes_gcm_decrypt data too short (missing nonce/tag)", position)
        
        # Extract components
        nonce = data_bytes[:12]
        ciphertext = data_bytes[12:-16]
        auth_tag = data_bytes[-16:]
        
        # Decrypt with GCM mode
        cipher = Cipher(algorithms.AES(key_bytes), modes.GCM(nonce, auth_tag), backend=default_backend())
        decryptor = cipher.decryptor()
        
        try:
            plaintext = decryptor.update(ciphertext) + decryptor.finalize()
        except Exception as e:
            raise ModuleError(f"aes_gcm_decrypt authentication failed: {e}", position)
        
        # Convert back to Glang list
        result = [NumberValue(byte, position) for byte in plaintext]
        return ListValue(result, 'num', position)
    
    @staticmethod
    def hmac_sha256(data: GlangValue, key: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Compute HMAC-SHA256 message authentication code."""
        if not isinstance(data, ListValue):
            raise ModuleError("hmac_sha256 expects list of bytes for data", position)
        if not isinstance(key, ListValue):
            raise ModuleError("hmac_sha256 expects list of bytes for key", position)
        
        # Convert to bytes
        try:
            data_bytes = bytes([
                int(item.value) for item in data.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
            key_bytes = bytes([
                int(item.value) for item in key.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
        except (ValueError, AttributeError):
            raise ModuleError("hmac_sha256 expects list of valid byte values (0-255)", position)
        
        # Compute HMAC
        mac = hmac.new(key_bytes, data_bytes, hashlib.sha256).digest()
        
        result = [NumberValue(byte, position) for byte in mac]
        return ListValue(result, 'num', position)
    
    @staticmethod
    def hkdf_expand(prk: GlangValue, length: GlangValue, info: GlangValue = None, position: Optional[SourcePosition] = None) -> GlangValue:
        """Expand key material using HKDF (HMAC-based Key Derivation Function)."""
        if not isinstance(prk, ListValue):
            raise ModuleError("hkdf_expand expects list of bytes for PRK", position)
        if not isinstance(length, NumberValue):
            raise ModuleError("hkdf_expand expects number for length", position)
        
        # Convert PRK to bytes
        try:
            prk_bytes = bytes([
                int(item.value) for item in prk.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
        except (ValueError, AttributeError):
            raise ModuleError("hkdf_expand PRK must be list of valid byte values (0-255)", position)
        
        # Convert info to bytes if provided
        info_bytes = b""
        if info is not None:
            if isinstance(info, ListValue):
                try:
                    info_bytes = bytes([
                        int(item.value) for item in info.elements
                        if isinstance(item, NumberValue) and 0 <= item.value <= 255
                    ])
                except (ValueError, AttributeError):
                    raise ModuleError("hkdf_expand info must be list of valid byte values (0-255)", position)
            elif isinstance(info, StringValue):
                info_bytes = info.value.encode('utf-8')
        
        target_length = int(length.value)
        if target_length < 0:
            raise ModuleError("hkdf_expand length must be non-negative", position)
        if target_length > 255 * 32:  # HKDF max output length
            raise ModuleError("hkdf_expand length too large (max 8160 bytes)", position)
        
        # Perform HKDF expand
        hkdf = HKDF(
            algorithm=hashes.SHA256(),
            length=target_length,
            salt=None,  # No salt for expand-only
            info=info_bytes,
            backend=default_backend()
        )
        
        try:
            derived = hkdf.derive(prk_bytes)
        except Exception as e:
            raise ModuleError(f"hkdf_expand failed: {e}", position)
        
        result = [NumberValue(byte, position) for byte in derived]
        return ListValue(result, 'num', position)
    
    @staticmethod
    def rsa_generate_keypair(key_size: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Generate RSA public/private key pair."""
        if not isinstance(key_size, NumberValue):
            raise ModuleError("rsa_generate_keypair expects number for key size", position)
        
        size = int(key_size.value)
        if size not in [2048, 3072, 4096]:
            raise ModuleError("rsa_generate_keypair key size must be 2048, 3072, or 4096", position)
        
        # Generate RSA key pair
        private_key = rsa.generate_private_key(
            public_exponent=65537,
            key_size=size,
            backend=default_backend()
        )
        public_key = private_key.public_key()
        
        # Serialize keys to DER format
        private_der = private_key.private_bytes(
            encoding=Encoding.DER,
            format=PrivateFormat.PKCS8,
            encryption_algorithm=NoEncryption()
        )
        public_der = public_key.public_bytes(
            encoding=Encoding.DER,
            format=PublicFormat.SubjectPublicKeyInfo
        )
        
        # Convert to Glang lists
        private_list = [NumberValue(byte, position) for byte in private_der]
        public_list = [NumberValue(byte, position) for byte in public_der]
        
        # Return as hash with private and public keys
        result_hash = {}
        result_hash["private"] = ListValue(private_list, 'num', position)
        result_hash["public"] = ListValue(public_list, 'num', position)
        
        return HashValue(result_hash, position)
    
    @staticmethod
    def rsa_encrypt(data: GlangValue, public_key: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Encrypt data using RSA public key."""
        if not isinstance(data, ListValue):
            raise ModuleError("rsa_encrypt expects list of bytes for data", position)
        
        # Handle DataValue wrapper for public key (HashValue automatically wraps in DataValue)
        key_value = public_key
        if isinstance(public_key, DataValue):
            key_value = public_key.value
        
        if not isinstance(key_value, ListValue):
            raise ModuleError("rsa_encrypt expects list of bytes for public key", position)
        
        # Convert to bytes
        try:
            data_bytes = bytes([
                int(item.value) for item in data.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
            key_bytes = bytes([
                int(item.value) for item in key_value.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
        except (ValueError, AttributeError):
            raise ModuleError("rsa_encrypt expects list of valid byte values (0-255)", position)
        
        # Load public key from DER
        try:
            from cryptography.hazmat.primitives.serialization import load_der_public_key
            pub_key = load_der_public_key(key_bytes, backend=default_backend())
        except Exception as e:
            raise ModuleError(f"rsa_encrypt invalid public key: {e}", position)
        
        # Encrypt with OAEP padding
        try:
            ciphertext = pub_key.encrypt(
                data_bytes,
                padding.OAEP(
                    mgf=padding.MGF1(algorithm=hashes.SHA256()),
                    algorithm=hashes.SHA256(),
                    label=None
                )
            )
        except Exception as e:
            raise ModuleError(f"rsa_encrypt failed: {e}", position)
        
        result = [NumberValue(byte, position) for byte in ciphertext]
        return ListValue(result, 'num', position)
    
    @staticmethod
    def rsa_decrypt(ciphertext: GlangValue, private_key: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Decrypt data using RSA private key."""
        if not isinstance(ciphertext, ListValue):
            raise ModuleError("rsa_decrypt expects list of bytes for ciphertext", position)
        
        # Handle DataValue wrapper for private key
        key_value = private_key
        if isinstance(private_key, DataValue):
            key_value = private_key.value
        
        if not isinstance(key_value, ListValue):
            raise ModuleError("rsa_decrypt expects list of bytes for private key", position)
        
        # Convert to bytes
        try:
            cipher_bytes = bytes([
                int(item.value) for item in ciphertext.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
            key_bytes = bytes([
                int(item.value) for item in key_value.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
        except (ValueError, AttributeError):
            raise ModuleError("rsa_decrypt expects list of valid byte values (0-255)", position)
        
        # Load private key from DER
        try:
            from cryptography.hazmat.primitives.serialization import load_der_private_key
            priv_key = load_der_private_key(key_bytes, password=None, backend=default_backend())
        except Exception as e:
            raise ModuleError(f"rsa_decrypt invalid private key: {e}", position)
        
        # Decrypt with OAEP padding
        try:
            plaintext = priv_key.decrypt(
                cipher_bytes,
                padding.OAEP(
                    mgf=padding.MGF1(algorithm=hashes.SHA256()),
                    algorithm=hashes.SHA256(),
                    label=None
                )
            )
        except Exception as e:
            raise ModuleError(f"rsa_decrypt failed: {e}", position)
        
        result = [NumberValue(byte, position) for byte in plaintext]
        return ListValue(result, 'num', position)
    
    @staticmethod
    def ecdh_generate_keypair(curve_name: GlangValue = None, position: Optional[SourcePosition] = None) -> GlangValue:
        """Generate ECDH public/private key pair."""
        # Default to P-256 if no curve specified
        curve_str = "P-256"
        if curve_name is not None:
            if not isinstance(curve_name, StringValue):
                raise ModuleError("ecdh_generate_keypair expects string for curve name", position)
            curve_str = curve_name.value
        
        # Select curve
        if curve_str == "P-256":
            curve = ec.SECP256R1()
        elif curve_str == "P-384":
            curve = ec.SECP384R1()
        elif curve_str == "P-521":
            curve = ec.SECP521R1()
        else:
            raise ModuleError(f"ecdh_generate_keypair unsupported curve: {curve_str} (supported: P-256, P-384, P-521)", position)
        
        # Generate ECDH key pair
        private_key = ec.generate_private_key(curve, backend=default_backend())
        public_key = private_key.public_key()
        
        # Serialize keys to DER format
        private_der = private_key.private_bytes(
            encoding=Encoding.DER,
            format=PrivateFormat.PKCS8,
            encryption_algorithm=NoEncryption()
        )
        public_der = public_key.public_bytes(
            encoding=Encoding.DER,
            format=PublicFormat.SubjectPublicKeyInfo
        )
        
        # Convert to Glang lists
        private_list = [NumberValue(byte, position) for byte in private_der]
        public_list = [NumberValue(byte, position) for byte in public_der]
        
        # Return as hash with private and public keys
        result_hash = {}
        result_hash["private"] = ListValue(private_list, 'num', position)
        result_hash["public"] = ListValue(public_list, 'num', position)
        result_hash["curve"] = StringValue(curve_str, position)
        
        return HashValue(result_hash, position)
    
    @staticmethod
    def ecdh_compute_shared_secret(private_key: GlangValue, peer_public_key: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Compute ECDH shared secret from private key and peer's public key."""
        # Handle DataValue wrapper for private key
        priv_key_value = private_key
        if isinstance(private_key, DataValue):
            priv_key_value = private_key.value
        
        # Handle DataValue wrapper for public key
        pub_key_value = peer_public_key
        if isinstance(peer_public_key, DataValue):
            pub_key_value = peer_public_key.value
        
        if not isinstance(priv_key_value, ListValue):
            raise ModuleError("ecdh_compute_shared_secret expects list of bytes for private key", position)
        if not isinstance(pub_key_value, ListValue):
            raise ModuleError("ecdh_compute_shared_secret expects list of bytes for peer public key", position)
        
        # Convert to bytes
        try:
            priv_key_bytes = bytes([
                int(item.value) for item in priv_key_value.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
            pub_key_bytes = bytes([
                int(item.value) for item in pub_key_value.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
        except (ValueError, AttributeError):
            raise ModuleError("ecdh_compute_shared_secret expects list of valid byte values (0-255)", position)
        
        # Load keys from DER
        try:
            from cryptography.hazmat.primitives.serialization import load_der_private_key, load_der_public_key
            priv_key = load_der_private_key(priv_key_bytes, password=None, backend=default_backend())
            pub_key = load_der_public_key(pub_key_bytes, backend=default_backend())
        except Exception as e:
            raise ModuleError(f"ecdh_compute_shared_secret invalid key format: {e}", position)
        
        # Verify both keys are EC keys
        if not isinstance(priv_key, ec.EllipticCurvePrivateKey):
            raise ModuleError("ecdh_compute_shared_secret private key must be EC key", position)
        if not isinstance(pub_key, ec.EllipticCurvePublicKey):
            raise ModuleError("ecdh_compute_shared_secret public key must be EC key", position)
        
        # Compute shared secret
        try:
            shared_key = priv_key.exchange(ec.ECDH(), pub_key)
        except Exception as e:
            raise ModuleError(f"ecdh_compute_shared_secret failed: {e}", position)
        
        # Convert to Glang list
        result = [NumberValue(byte, position) for byte in shared_key]
        return ListValue(result, 'num', position)
    
    @staticmethod
    def ecdh_public_key_from_private(private_key: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Extract public key from ECDH private key."""
        # Handle DataValue wrapper for private key
        priv_key_value = private_key
        if isinstance(private_key, DataValue):
            priv_key_value = private_key.value
        
        if not isinstance(priv_key_value, ListValue):
            raise ModuleError("ecdh_public_key_from_private expects list of bytes for private key", position)
        
        # Convert to bytes
        try:
            priv_key_bytes = bytes([
                int(item.value) for item in priv_key_value.elements
                if isinstance(item, NumberValue) and 0 <= item.value <= 255
            ])
        except (ValueError, AttributeError):
            raise ModuleError("ecdh_public_key_from_private expects list of valid byte values (0-255)", position)
        
        # Load private key from DER
        try:
            from cryptography.hazmat.primitives.serialization import load_der_private_key
            priv_key = load_der_private_key(priv_key_bytes, password=None, backend=default_backend())
        except Exception as e:
            raise ModuleError(f"ecdh_public_key_from_private invalid private key: {e}", position)
        
        # Verify it's an EC key
        if not isinstance(priv_key, ec.EllipticCurvePrivateKey):
            raise ModuleError("ecdh_public_key_from_private private key must be EC key", position)
        
        # Extract public key
        public_key = priv_key.public_key()
        
        # Serialize to DER format
        public_der = public_key.public_bytes(
            encoding=Encoding.DER,
            format=PublicFormat.SubjectPublicKeyInfo
        )
        
        # Convert to Glang list
        result = [NumberValue(byte, position) for byte in public_der]
        return ListValue(result, 'num', position)


# Module function registry
CRYPTO_FUNCTIONS = {
    # Hashing functions
    'hash_md5': CryptoModule.hash_md5,
    'hash_sha1': CryptoModule.hash_sha1,
    'hash_sha256': CryptoModule.hash_sha256,
    'hash_sha512': CryptoModule.hash_sha512,
    
    # Random generation
    'random_bytes': CryptoModule.random_bytes,
    
    # Symmetric encryption (AES)
    'aes_encrypt': CryptoModule.aes_encrypt,
    'aes_decrypt': CryptoModule.aes_decrypt,
    'aes_gcm_encrypt': CryptoModule.aes_gcm_encrypt,
    'aes_gcm_decrypt': CryptoModule.aes_gcm_decrypt,
    
    # Message authentication
    'hmac_sha256': CryptoModule.hmac_sha256,
    
    # Key derivation
    'hkdf_expand': CryptoModule.hkdf_expand,
    
    # Asymmetric encryption (RSA)
    'rsa_generate_keypair': CryptoModule.rsa_generate_keypair,
    'rsa_encrypt': CryptoModule.rsa_encrypt,
    'rsa_decrypt': CryptoModule.rsa_decrypt,
    
    # Elliptic Curve Diffie-Hellman (ECDH)
    'ecdh_generate_keypair': CryptoModule.ecdh_generate_keypair,
    'ecdh_compute_shared_secret': CryptoModule.ecdh_compute_shared_secret,
    'ecdh_public_key_from_private': CryptoModule.ecdh_public_key_from_private,
    
    # Format conversion
    'to_hex': CryptoModule.to_hex,
    'from_hex': CryptoModule.from_hex,
    'to_base64': CryptoModule.to_base64,
    'from_base64': CryptoModule.from_base64,
}


def create_crypto_module_namespace():
    """Create the crypto module namespace."""
    from .module_builder import create_module

    return create_module("crypto", functions=CRYPTO_FUNCTIONS)
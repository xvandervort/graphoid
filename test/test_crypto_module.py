"""Tests for crypto module operations."""

import pytest
from glang.execution.pipeline import ExecutionSession


class TestCryptoModule:
    """Test crypto module operations."""
    
    def setup_method(self):
        """Set up test environment."""
        self.session = ExecutionSession()
        # Import crypto module
        result = self.session.execute_statement('import "crypto"')
        assert result.success, f"Failed to import crypto module: {result}"
    
    def test_hash_md5(self):
        """Test MD5 hashing."""
        # Test with "Hello" in ASCII
        result = self.session.execute_statement('hello_bytes = [72, 101, 108, 108, 111]')
        assert result.success
        
        result = self.session.execute_statement('hash = crypto.hash_md5(hello_bytes)')
        assert result.success
        
        result = self.session.execute_statement('hash')
        assert result.success
        hash_list = result.value
        assert len(hash_list.elements) == 16  # MD5 produces 16 bytes
        
        # Verify it's a list of valid byte values
        for byte_val in hash_list.elements:
            assert 0 <= byte_val.value <= 255
    
    def test_hash_sha1(self):
        """Test SHA1 hashing."""
        result = self.session.execute_statement('data = [116, 101, 115, 116]')  # "test"
        assert result.success
        
        result = self.session.execute_statement('hash = crypto.hash_sha1(data)')
        assert result.success
        
        result = self.session.execute_statement('hash')
        assert result.success
        hash_list = result.value
        assert len(hash_list.elements) == 20  # SHA1 produces 20 bytes
    
    def test_hash_sha256(self):
        """Test SHA256 hashing."""
        result = self.session.execute_statement('data = [116, 101, 115, 116]')  # "test"
        assert result.success
        
        result = self.session.execute_statement('hash = crypto.hash_sha256(data)')
        assert result.success
        
        result = self.session.execute_statement('hash')
        assert result.success
        hash_list = result.value
        assert len(hash_list.elements) == 32  # SHA256 produces 32 bytes
    
    def test_hash_sha512(self):
        """Test SHA512 hashing."""
        result = self.session.execute_statement('data = [116, 101, 115, 116]')  # "test"
        assert result.success
        
        result = self.session.execute_statement('hash = crypto.hash_sha512(data)')
        assert result.success
        
        result = self.session.execute_statement('hash')
        assert result.success
        hash_list = result.value
        assert len(hash_list.elements) == 64  # SHA512 produces 64 bytes
    
    def test_random_bytes(self):
        """Test cryptographic random byte generation."""
        # Test different lengths
        lengths = [16, 32, 64]
        for i, length in enumerate(lengths):
            result = self.session.execute_statement(f'random_data_{i} = crypto.random_bytes({length})')
            assert result.success
            
            result = self.session.execute_statement(f'random_data_{i}')
            assert result.success
            random_list = result.value
            assert len(random_list.elements) == length
            
            # Verify all values are valid bytes
            for byte_val in random_list.elements:
                assert 0 <= byte_val.value <= 255
    
    def test_random_bytes_zero_length(self):
        """Test zero-length random bytes."""
        result = self.session.execute_statement('empty_random = crypto.random_bytes(0)')
        assert result.success
        
        result = self.session.execute_statement('empty_random')
        assert result.success
        assert len(result.value.elements) == 0
    
    def test_aes_encryption_decryption(self):
        """Test AES encryption and decryption roundtrip."""
        # Create test data
        result = self.session.execute_statement('plaintext = [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]')  # "Hello World"
        assert result.success
        
        # Generate 32-byte key for AES-256
        result = self.session.execute_statement('key = crypto.random_bytes(32)')
        assert result.success
        
        # Encrypt
        result = self.session.execute_statement('encrypted = crypto.aes_encrypt(plaintext, key)')
        assert result.success
        
        result = self.session.execute_statement('encrypted')
        assert result.success
        encrypted_data = result.value
        assert len(encrypted_data.elements) > len([72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100])  # Should be longer due to IV and padding
        
        # Decrypt
        result = self.session.execute_statement('decrypted = crypto.aes_decrypt(encrypted, key)')
        assert result.success
        
        # Verify roundtrip
        result = self.session.execute_statement('decrypted')
        assert result.success
        decrypted_data = result.value
        
        # Should match original plaintext
        original_bytes = [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]
        assert len(decrypted_data.elements) == len(original_bytes)
        for i, expected_byte in enumerate(original_bytes):
            assert decrypted_data.elements[i].value == expected_byte
    
    def test_to_hex_conversion(self):
        """Test converting bytes to hexadecimal string."""
        # Test with known values
        result = self.session.execute_statement('bytes = [255, 171, 205]')  # Should be "FFABCD"
        assert result.success
        
        result = self.session.execute_statement('hex_string = crypto.to_hex(bytes)')
        assert result.success
        
        result = self.session.execute_statement('hex_string')
        assert result.success
        assert result.value.value == "ffabcd"  # Python hex() returns lowercase
    
    def test_from_hex_conversion(self):
        """Test converting hexadecimal string to bytes."""
        result = self.session.execute_statement('hex_str = "ffabcd"')
        assert result.success
        
        result = self.session.execute_statement('bytes = crypto.from_hex(hex_str)')
        assert result.success
        
        result = self.session.execute_statement('bytes')
        assert result.success
        byte_list = result.value
        assert len(byte_list.elements) == 3
        assert byte_list.elements[0].value == 255
        assert byte_list.elements[1].value == 171
        assert byte_list.elements[2].value == 205
    
    def test_hex_roundtrip(self):
        """Test hex conversion roundtrip."""
        result = self.session.execute_statement('original = [0, 1, 127, 128, 254, 255]')
        assert result.success
        
        # Convert to hex
        result = self.session.execute_statement('hex_str = crypto.to_hex(original)')
        assert result.success
        
        # Convert back to bytes
        result = self.session.execute_statement('restored = crypto.from_hex(hex_str)')
        assert result.success
        
        # Verify roundtrip
        result = self.session.execute_statement('restored')
        assert result.success
        restored_data = result.value
        
        original_bytes = [0, 1, 127, 128, 254, 255]
        assert len(restored_data.elements) == len(original_bytes)
        for i, expected_byte in enumerate(original_bytes):
            assert restored_data.elements[i].value == expected_byte
    
    def test_to_base64_conversion(self):
        """Test converting bytes to base64 string."""
        # Test with "Hello" -> SGVsbG8=
        result = self.session.execute_statement('hello_bytes = [72, 101, 108, 108, 111]')
        assert result.success
        
        result = self.session.execute_statement('b64_string = crypto.to_base64(hello_bytes)')
        assert result.success
        
        result = self.session.execute_statement('b64_string')
        assert result.success
        assert result.value.value == "SGVsbG8="
    
    def test_from_base64_conversion(self):
        """Test converting base64 string to bytes."""
        result = self.session.execute_statement('b64_str = "SGVsbG8="')  # "Hello"
        assert result.success
        
        result = self.session.execute_statement('bytes = crypto.from_base64(b64_str)')
        assert result.success
        
        result = self.session.execute_statement('bytes')
        assert result.success
        byte_list = result.value
        
        # Should be "Hello"
        expected = [72, 101, 108, 108, 111]
        assert len(byte_list.elements) == len(expected)
        for i, expected_byte in enumerate(expected):
            assert byte_list.elements[i].value == expected_byte
    
    def test_base64_roundtrip(self):
        """Test base64 conversion roundtrip."""
        result = self.session.execute_statement('original = [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]')  # "Hello World"
        assert result.success
        
        # Convert to base64
        result = self.session.execute_statement('b64_str = crypto.to_base64(original)')
        assert result.success
        
        # Convert back to bytes
        result = self.session.execute_statement('restored = crypto.from_base64(b64_str)')
        assert result.success
        
        # Verify roundtrip
        result = self.session.execute_statement('restored')
        assert result.success
        restored_data = result.value
        
        original_bytes = [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]
        assert len(restored_data.elements) == len(original_bytes)
        for i, expected_byte in enumerate(original_bytes):
            assert restored_data.elements[i].value == expected_byte
    
    def test_hash_invalid_input(self):
        """Test error handling for invalid hash input."""
        # Try to hash a string instead of byte list
        result = self.session.execute_statement('bad_hash = crypto.hash_sha256("not a list")')
        assert not result.success
        assert "expects list of bytes" in str(result.error)
    
    def test_aes_invalid_key_length(self):
        """Test error handling for invalid AES key length."""
        result = self.session.execute_statement('data = [1, 2, 3, 4]')
        assert result.success
        
        result = self.session.execute_statement('short_key = [1, 2, 3]')  # Too short for AES-256
        assert result.success
        
        result = self.session.execute_statement('bad_encrypt = crypto.aes_encrypt(data, short_key)')
        assert not result.success
        assert "requires 32-byte key" in str(result.error)
    
    def test_hex_invalid_input(self):
        """Test error handling for invalid hex input."""
        result = self.session.execute_statement('bad_hex = crypto.from_hex("zzz")')
        assert not result.success
        assert "invalid hexadecimal string" in str(result.error)
    
    def test_base64_invalid_input(self):
        """Test error handling for invalid base64 input."""
        result = self.session.execute_statement('bad_b64 = crypto.from_base64("invalid!@#$")')
        assert not result.success
        assert "invalid base64 string" in str(result.error)
    
    def test_random_bytes_negative_length(self):
        """Test error handling for negative random bytes length."""
        result = self.session.execute_statement('bad_random = crypto.random_bytes(-1)')
        assert not result.success
        assert "must be non-negative" in str(result.error)
    
    def test_hash_consistency(self):
        """Test that hashing the same data produces the same result."""
        result = self.session.execute_statement('data = [116, 101, 115, 116]')  # "test"
        assert result.success
        
        # Hash twice
        result = self.session.execute_statement('hash1 = crypto.hash_sha256(data)')
        assert result.success
        
        result = self.session.execute_statement('hash2 = crypto.hash_sha256(data)')
        assert result.success
        
        # Should be identical
        result = self.session.execute_statement('hash1')
        assert result.success
        hash1_data = result.value
        
        result = self.session.execute_statement('hash2')
        assert result.success
        hash2_data = result.value
        
        assert len(hash1_data.elements) == len(hash2_data.elements)
        for i in range(len(hash1_data.elements)):
            assert hash1_data.elements[i].value == hash2_data.elements[i].value
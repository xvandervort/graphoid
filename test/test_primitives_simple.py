"""Test primitives module functionality - focused on actual methods."""

import pytest
import re
import uuid
from unittest.mock import patch, Mock

from glang.modules.primitives import PrimitiveModule
from glang.execution.values import (
    StringValue, NumberValue, BooleanValue, NoneValue, ListValue
)


class TestPrimitiveModule:
    """Test the PrimitiveModule class."""

    def setup_method(self):
        """Set up test fixtures."""
        self.primitives = PrimitiveModule()

    def test_builtin_secure_random(self):
        """Test secure random number generation."""
        result = self.primitives.builtin_secure_random()

        assert isinstance(result, NumberValue)
        assert 0.0 <= result.value < 1.0

        # Should be different each time
        result2 = self.primitives.builtin_secure_random()
        assert result.value != result2.value  # Very unlikely to be the same

    def test_builtin_deterministic_random(self):
        """Test deterministic random number generation."""
        result = self.primitives.builtin_deterministic_random()

        assert isinstance(result, NumberValue)
        assert 0.0 <= result.value < 1.0

    def test_builtin_secure_randint(self):
        """Test secure random integer generation."""
        min_val = NumberValue(10)
        max_val = NumberValue(20)
        result = self.primitives.builtin_secure_randint(min_val, max_val)

        assert isinstance(result, NumberValue)
        assert 10 <= result.value <= 20
        assert isinstance(result.value, int)

    def test_builtin_deterministic_randint(self):
        """Test deterministic random integer generation."""
        min_val = NumberValue(5)
        max_val = NumberValue(15)
        result = self.primitives.builtin_deterministic_randint(min_val, max_val)

        assert isinstance(result, NumberValue)
        assert 5 <= result.value <= 15
        assert isinstance(result.value, int)

    def test_builtin_secure_uniform(self):
        """Test secure uniform distribution."""
        min_val = NumberValue(1.0)
        max_val = NumberValue(10.0)
        result = self.primitives.builtin_secure_uniform(min_val, max_val)

        assert isinstance(result, NumberValue)
        assert 1.0 <= result.value <= 10.0

    def test_builtin_deterministic_uniform(self):
        """Test deterministic uniform distribution."""
        min_val = NumberValue(2.0)
        max_val = NumberValue(8.0)
        result = self.primitives.builtin_deterministic_uniform(min_val, max_val)

        assert isinstance(result, NumberValue)
        assert 2.0 <= result.value <= 8.0

    def test_builtin_secure_normal(self):
        """Test secure normal distribution."""
        mean = NumberValue(0.0)
        std_dev = NumberValue(1.0)
        result = self.primitives.builtin_secure_normal(mean, std_dev)

        assert isinstance(result, NumberValue)
        # Normal distribution can produce any value, but should be reasonable
        assert -10.0 <= result.value <= 10.0  # Very loose bounds

    def test_builtin_deterministic_normal(self):
        """Test deterministic normal distribution."""
        mean = NumberValue(5.0)
        std_dev = NumberValue(2.0)
        result = self.primitives.builtin_deterministic_normal(mean, std_dev)

        assert isinstance(result, NumberValue)
        assert -10.0 <= result.value <= 20.0  # Loose bounds around mean

    def test_builtin_seed_generator(self):
        """Test seeding deterministic generator."""
        seed = NumberValue(42)
        result = self.primitives.builtin_seed_generator(seed)

        assert isinstance(result, NoneValue)

        # Seeded generator should produce same sequence
        val1 = self.primitives.builtin_deterministic_random().value
        self.primitives.builtin_seed_generator(seed)  # Re-seed
        val2 = self.primitives.builtin_deterministic_random().value

        assert val1 == val2  # Same seed should produce same value

    def test_builtin_reset_generator(self):
        """Test resetting deterministic generator."""
        result = self.primitives.builtin_reset_generator()

        assert isinstance(result, NoneValue)

    def test_builtin_secure_token(self):
        """Test secure token generation."""
        length = NumberValue(16)
        result = self.primitives.builtin_secure_token(length)

        assert isinstance(result, StringValue)
        assert len(result.value) == 32  # 16 bytes = 32 hex chars

    def test_builtin_uuid4(self):
        """Test UUID4 generation."""
        result = self.primitives.builtin_uuid4()

        assert isinstance(result, StringValue)
        assert len(result.value) == 36  # Standard UUID string length
        assert result.value.count("-") == 4  # UUID has 4 hyphens

        # Should be different each time
        result2 = self.primitives.builtin_uuid4()
        assert result.value != result2.value

    def test_builtin_uuid1(self):
        """Test UUID1 generation."""
        result = self.primitives.builtin_uuid1()

        assert isinstance(result, StringValue)
        assert len(result.value) == 36  # Standard UUID string length
        assert result.value.count("-") == 4  # UUID has 4 hyphens

    def test_builtin_current_time_millis(self):
        """Test current time in milliseconds."""
        with patch('time.time', return_value=1642291200.123):
            result = self.primitives.builtin_current_time_millis()

            assert isinstance(result, NumberValue)
            assert result.value == 1642291200123  # milliseconds

    def test_builtin_regex_compile(self):
        """Test regex compilation."""
        pattern = StringValue(r"\d+")
        result = self.primitives.builtin_regex_compile(pattern)

        assert isinstance(result, StringValue)
        # The cache may use a different key format (e.g., "pattern:flags")
        assert len(self.primitives._regex_cache) > 0  # Should have cached something
        assert result.value in self.primitives._regex_cache  # The result key should be in cache

    def test_builtin_regex_match(self):
        """Test regex matching using compiled pattern."""
        pattern = StringValue(r"\d+")
        cache_key = self.primitives.builtin_regex_compile(pattern)
        text = StringValue("123abc")

        result = self.primitives.builtin_regex_match(cache_key, text)

        assert isinstance(result, BooleanValue)
        assert result.value is True

    def test_builtin_regex_search(self):
        """Test regex search using compiled pattern."""
        pattern = StringValue(r"\d+")
        cache_key = self.primitives.builtin_regex_compile(pattern)
        text = StringValue("abc123def")

        result = self.primitives.builtin_regex_search(cache_key, text)

        assert isinstance(result, BooleanValue)
        assert result.value is True

    def test_builtin_regex_findall(self):
        """Test regex findall using compiled pattern."""
        pattern = StringValue(r"\d+")
        cache_key = self.primitives.builtin_regex_compile(pattern)
        text = StringValue("abc123def456ghi")

        result = self.primitives.builtin_regex_findall(cache_key, text)

        assert isinstance(result, ListValue)
        assert len(result.elements) == 2
        assert result.elements[0].value == "123"
        assert result.elements[1].value == "456"

    def test_builtin_regex_replace(self):
        """Test regex replacement using compiled pattern."""
        pattern = StringValue(r"\d+")
        cache_key = self.primitives.builtin_regex_compile(pattern)
        replacement = StringValue("XXX")
        text = StringValue("abc123def456")

        result = self.primitives.builtin_regex_replace(cache_key, replacement, text)

        assert isinstance(result, StringValue)
        assert result.value == "abcXXXdefXXX"

    def test_builtin_regex_split(self):
        """Test regex split using compiled pattern."""
        pattern = StringValue(r"[,;]")
        cache_key = self.primitives.builtin_regex_compile(pattern)
        text = StringValue("a,b;c,d")

        result = self.primitives.builtin_regex_split(cache_key, text)

        assert isinstance(result, ListValue)
        assert len(result.elements) == 4
        assert result.elements[0].value == "a"
        assert result.elements[1].value == "b"
        assert result.elements[2].value == "c"
        assert result.elements[3].value == "d"

    def test_regex_invalid_pattern(self):
        """Test error handling for invalid regex patterns."""
        invalid_pattern = StringValue("[invalid")  # Missing closing bracket

        with pytest.raises(Exception):
            self.primitives.builtin_regex_compile(invalid_pattern)

    def test_regex_cache_functionality(self):
        """Test that regex cache works properly."""
        pattern1 = StringValue(r"\d+")
        pattern2 = StringValue(r"\w+")

        # Compile patterns
        key1 = self.primitives.builtin_regex_compile(pattern1)
        key2 = self.primitives.builtin_regex_compile(pattern2)

        # Both keys should be cached
        assert key1.value in self.primitives._regex_cache
        assert key2.value in self.primitives._regex_cache
        assert key1 != key2
        assert len(self.primitives._regex_cache) >= 2  # Should have at least 2 entries

    def test_exponential_distribution(self):
        """Test exponential distribution functions."""
        lambda_val = NumberValue(1.0)

        # Secure exponential
        result1 = self.primitives.builtin_secure_exponential(lambda_val)
        assert isinstance(result1, NumberValue)
        assert result1.value >= 0  # Exponential distribution is always non-negative

        # Deterministic exponential
        result2 = self.primitives.builtin_deterministic_exponential(lambda_val)
        assert isinstance(result2, NumberValue)
        assert result2.value >= 0

    def test_gamma_distribution(self):
        """Test gamma distribution functions."""
        alpha = NumberValue(2.0)
        beta = NumberValue(1.0)

        # Secure gamma
        result1 = self.primitives.builtin_secure_gamma(alpha, beta)
        assert isinstance(result1, NumberValue)
        assert result1.value >= 0  # Gamma distribution is always non-negative

        # Deterministic gamma
        result2 = self.primitives.builtin_deterministic_gamma(alpha, beta)
        assert isinstance(result2, NumberValue)
        assert result2.value >= 0
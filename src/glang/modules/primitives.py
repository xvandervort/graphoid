"""
Minimal Python primitives needed for Glang standard library modules.

This module provides the absolute minimum Python functions needed to implement
Glang standard library modules like random and regex. The goal is to keep
this as small as possible and implement everything else in Glang itself.
"""

import random
import secrets
import uuid
import time
import re
from typing import Optional
import sys
import os

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../../..'))

from glang.execution.values import (
    GlangValue, StringValue, NumberValue, BooleanValue, NoneValue
)
from glang.ast.nodes import SourcePosition


class PrimitiveModule:
    """Provides minimal Python primitives for Glang stdlib modules."""
    
    def __init__(self):
        # Separate generators for secure vs deterministic randomness
        self._secure_generator = secrets.SystemRandom()
        self._deterministic_generator = random.Random()
        self._regex_cache = {}
        self._cache_limit = 100
    
    # === Random Number Primitives ===
    
    def builtin_secure_random(self, position=None) -> NumberValue:
        """Generate cryptographically secure random float [0.0, 1.0)."""
        value = self._secure_generator.random()
        return NumberValue(value, position or SourcePosition(0, 0))
    
    def builtin_deterministic_random(self) -> NumberValue:
        """Generate deterministic random float [0.0, 1.0)."""
        value = self._deterministic_generator.random()
        return NumberValue(value, SourcePosition(0, 0))
    
    def builtin_secure_randint(self, min_val: NumberValue, max_val: NumberValue) -> NumberValue:
        """Generate cryptographically secure random integer [min, max]."""
        min_int = int(min_val.value)
        max_int = int(max_val.value)
        value = self._secure_generator.randint(min_int, max_int)
        return NumberValue(value, min_val.position)
    
    def builtin_deterministic_randint(self, min_val: NumberValue, max_val: NumberValue) -> NumberValue:
        """Generate deterministic random integer [min, max]."""
        min_int = int(min_val.value)
        max_int = int(max_val.value)
        value = self._deterministic_generator.randint(min_int, max_int)
        return NumberValue(value, min_val.position)
    
    def builtin_secure_uniform(self, min_val: NumberValue, max_val: NumberValue) -> NumberValue:
        """Generate cryptographically secure uniform float [min, max)."""
        min_float = float(min_val.value)
        max_float = float(max_val.value)
        value = self._secure_generator.uniform(min_float, max_float)
        return NumberValue(value, min_val.position)
    
    def builtin_deterministic_uniform(self, min_val: NumberValue, max_val: NumberValue) -> NumberValue:
        """Generate deterministic uniform float [min, max)."""
        min_float = float(min_val.value)
        max_float = float(max_val.value)
        value = self._deterministic_generator.uniform(min_float, max_float)
        return NumberValue(value, min_val.position)
    
    def builtin_secure_normal(self, mean: NumberValue, std_dev: NumberValue) -> NumberValue:
        """Generate cryptographically secure normal distribution."""
        mean_val = float(mean.value)
        std_val = float(std_dev.value)
        value = self._secure_generator.normalvariate(mean_val, std_val)
        return NumberValue(value, mean.position)
    
    def builtin_deterministic_normal(self, mean: NumberValue, std_dev: NumberValue) -> NumberValue:
        """Generate deterministic normal distribution."""
        mean_val = float(mean.value)
        std_val = float(std_dev.value)
        value = self._deterministic_generator.normalvariate(mean_val, std_val)
        return NumberValue(value, mean.position)
    
    def builtin_secure_exponential(self, lambda_val: NumberValue) -> NumberValue:
        """Generate cryptographically secure exponential distribution."""
        lambda_float = float(lambda_val.value)
        value = self._secure_generator.expovariate(lambda_float)
        return NumberValue(value, lambda_val.position)
    
    def builtin_deterministic_exponential(self, lambda_val: NumberValue) -> NumberValue:
        """Generate deterministic exponential distribution."""
        lambda_float = float(lambda_val.value)
        value = self._deterministic_generator.expovariate(lambda_float)
        return NumberValue(value, lambda_val.position)
    
    def builtin_secure_gamma(self, alpha: NumberValue, beta: NumberValue) -> NumberValue:
        """Generate cryptographically secure gamma distribution."""
        alpha_val = float(alpha.value)
        beta_val = float(beta.value)
        value = self._secure_generator.gammavariate(alpha_val, beta_val)
        return NumberValue(value, alpha.position)
    
    def builtin_deterministic_gamma(self, alpha: NumberValue, beta: NumberValue) -> NumberValue:
        """Generate deterministic gamma distribution."""
        alpha_val = float(alpha.value)
        beta_val = float(beta.value)
        value = self._deterministic_generator.gammavariate(alpha_val, beta_val)
        return NumberValue(value, alpha.position)
    
    def builtin_seed_generator(self, seed_val: NumberValue) -> NoneValue:
        """Seed the deterministic random number generator."""
        seed_int = int(seed_val.value) if seed_val else None
        self._deterministic_generator.seed(seed_int)
        return NoneValue(seed_val.position if seed_val else SourcePosition(0, 0))
    
    def builtin_reset_generator(self) -> NoneValue:
        """Reset the deterministic random number generator."""
        self._deterministic_generator = random.Random()
        return NoneValue(SourcePosition(0, 0))
    
    def builtin_secure_token(self, length: NumberValue) -> StringValue:
        """Generate cryptographically secure random token as hex string."""
        byte_length = int(length.value)
        token = secrets.token_hex(byte_length)
        return StringValue(token, length.position)
    
    def builtin_uuid4(self) -> StringValue:
        """Generate a random UUID (version 4)."""
        uuid_value = str(uuid.uuid4())
        return StringValue(uuid_value, SourcePosition(0, 0))
    
    def builtin_uuid1(self) -> StringValue:
        """Generate a time-based UUID (version 1)."""
        uuid_value = str(uuid.uuid1())
        return StringValue(uuid_value, SourcePosition(0, 0))
    
    def builtin_current_time_millis(self) -> NumberValue:
        """Get current time in milliseconds since epoch."""
        millis = int(time.time() * 1000)
        return NumberValue(millis, SourcePosition(0, 0))
    
    # === Regex Primitives ===
    
    def builtin_regex_compile(self, pattern: StringValue, flags: Optional[NumberValue] = None) -> StringValue:
        """Compile regex pattern and return cache key."""
        pattern_str = pattern.value
        flags_int = int(flags.value) if flags else 0
        
        cache_key = f"{pattern_str}:{flags_int}"
        
        # Clear cache if too large
        if len(self._regex_cache) >= self._cache_limit:
            self._regex_cache.clear()
        
        try:
            compiled = re.compile(pattern_str, flags_int)
            self._regex_cache[cache_key] = compiled
            return StringValue(cache_key, pattern.position)
        except re.error as e:
            raise ValueError(f"Invalid regex pattern '{pattern_str}': {str(e)}")
    
    def builtin_regex_match(self, cache_key: StringValue, text: StringValue) -> BooleanValue:
        """Test if pattern matches at start of text."""
        key = cache_key.value
        text_str = text.value
        
        if key not in self._regex_cache:
            raise ValueError(f"Invalid regex cache key: {key}")
        
        compiled = self._regex_cache[key]
        match = compiled.match(text_str)
        
        return BooleanValue(match is not None, text.position)
    
    def builtin_regex_search(self, cache_key: StringValue, text: StringValue) -> BooleanValue:
        """Test if pattern is found anywhere in text."""
        key = cache_key.value
        text_str = text.value
        
        if key not in self._regex_cache:
            raise ValueError(f"Invalid regex cache key: {key}")
        
        compiled = self._regex_cache[key]
        match = compiled.search(text_str)
        
        return BooleanValue(match is not None, text.position)
    
    def builtin_regex_findall(self, cache_key: StringValue, text: StringValue) -> 'ListValue':
        """Find all non-overlapping matches."""
        from glang.execution.values import ListValue
        
        key = cache_key.value
        text_str = text.value
        
        if key not in self._regex_cache:
            raise ValueError(f"Invalid regex cache key: {key}")
        
        compiled = self._regex_cache[key]
        matches = compiled.findall(text_str)
        
        # Convert to list of StringValues
        string_matches = [StringValue(match, text.position) for match in matches]
        return ListValue(string_matches, text.position)
    
    def builtin_regex_findgroups(self, cache_key: StringValue, text: StringValue) -> 'ListValue':
        """Find all matches with capture groups."""
        from glang.execution.values import ListValue
        
        key = cache_key.value
        text_str = text.value
        
        if key not in self._regex_cache:
            raise ValueError(f"Invalid regex cache key: {key}")
        
        compiled = self._regex_cache[key]
        matches = compiled.finditer(text_str)
        
        # Convert to list of lists
        group_matches = []
        for match in matches:
            groups = [StringValue(group, text.position) for group in match.groups()]
            group_matches.append(ListValue(groups, text.position))
        
        return ListValue(group_matches, text.position)
    
    def builtin_regex_replace(self, cache_key: StringValue, replacement: StringValue, text: StringValue) -> StringValue:
        """Replace all occurrences of pattern."""
        key = cache_key.value
        text_str = text.value
        replacement_str = replacement.value
        
        if key not in self._regex_cache:
            raise ValueError(f"Invalid regex cache key: {key}")
        
        compiled = self._regex_cache[key]
        result = compiled.sub(replacement_str, text_str)
        
        return StringValue(result, text.position)
    
    def builtin_regex_split(self, cache_key: StringValue, text: StringValue) -> 'ListValue':
        """Split text using regex pattern as delimiter."""
        from glang.execution.values import ListValue
        
        key = cache_key.value
        text_str = text.value
        
        if key not in self._regex_cache:
            raise ValueError(f"Invalid regex cache key: {key}")
        
        compiled = self._regex_cache[key]
        parts = compiled.split(text_str)
        
        # Convert to list of StringValues, filtering out empty strings
        string_parts = [StringValue(part, text.position) for part in parts if part]
        return ListValue(string_parts, text.position)


def create_primitives_namespace():
    """Create the namespace for builtin primitives."""
    from .module_manager import ModuleNamespace
    from ..execution.function_value import BuiltinFunctionValue
    
    namespace = ModuleNamespace("_primitives")
    module = PrimitiveModule()
    
    # Register primitive functions
    primitive_functions = {
        # Random primitives
        '_builtin_secure_random': module.builtin_secure_random,
        '_builtin_deterministic_random': module.builtin_deterministic_random,
        '_builtin_secure_randint': module.builtin_secure_randint,
        '_builtin_deterministic_randint': module.builtin_deterministic_randint,
        '_builtin_secure_uniform': module.builtin_secure_uniform,
        '_builtin_deterministic_uniform': module.builtin_deterministic_uniform,
        '_builtin_secure_normal': module.builtin_secure_normal,
        '_builtin_deterministic_normal': module.builtin_deterministic_normal,
        '_builtin_secure_exponential': module.builtin_secure_exponential,
        '_builtin_deterministic_exponential': module.builtin_deterministic_exponential,
        '_builtin_secure_gamma': module.builtin_secure_gamma,
        '_builtin_deterministic_gamma': module.builtin_deterministic_gamma,
        '_builtin_seed_generator': module.builtin_seed_generator,
        '_builtin_reset_generator': module.builtin_reset_generator,
        '_builtin_secure_token': module.builtin_secure_token,
        '_builtin_uuid4': module.builtin_uuid4,
        '_builtin_uuid1': module.builtin_uuid1,
        '_builtin_current_time_millis': module.builtin_current_time_millis,
        
        # Regex primitives
        '_builtin_regex_compile': module.builtin_regex_compile,
        '_builtin_regex_match': module.builtin_regex_match,
        '_builtin_regex_search': module.builtin_regex_search,
        '_builtin_regex_findall': module.builtin_regex_findall,
        '_builtin_regex_findgroups': module.builtin_regex_findgroups,
        '_builtin_regex_replace': module.builtin_regex_replace,
        '_builtin_regex_split': module.builtin_regex_split,
    }
    
    # Wrap functions as callable values
    for name, func in primitive_functions.items():
        namespace.set_symbol(name, BuiltinFunctionValue(name, func))
    
    return namespace
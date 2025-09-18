"""
Glang Random Number Generation Module

Provides comprehensive random number generation capabilities including:
- Cryptographically secure random numbers
- Deterministic seeded random numbers
- Statistical distributions (uniform, normal, exponential, etc.)
- Random sampling and shuffling
- UUID generation

This module integrates cleanly with Glang's type system and provides
both secure and reproducible randomness as needed.
"""

import random
import secrets
import uuid
import math
from typing import Optional, List, Any, Dict
import sys
import os

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../../..'))

from glang.execution.values import (
    GlangValue, StringValue, BooleanValue,
    NumberValue, NoneValue
)
from glang.execution.graph_values import ListValue
from glang.ast.nodes import SourcePosition


class RandomModule:
    """
    Glang Random Number Generation Module
    
    Provides both cryptographically secure and deterministic random number
    generation with support for various statistical distributions.
    """
    
    def __init__(self):
        # Separate generators for secure vs deterministic randomness
        self._secure_generator = secrets.SystemRandom()
        self._deterministic_generator = random.Random()
        self._is_seeded = False
        self._current_seed = None
    
    # === Core Random Functions ===
    
    def random(self) -> NumberValue:
        """
        Generate a random float between 0.0 and 1.0 (exclusive).
        Uses deterministic generator if seeded, otherwise secure generator.
        
        Returns:
            NumberValue: Random float in range [0.0, 1.0)
        """
        if self._is_seeded:
            value = self._deterministic_generator.random()
        else:
            value = self._secure_generator.random()
        
        return NumberValue(value, SourcePosition(0, 0))
    
    def randint(self, min_val: NumberValue, max_val: NumberValue) -> NumberValue:
        """
        Generate a random integer in the inclusive range [min, max].
        
        Args:
            min_val: Minimum value (inclusive)
            max_val: Maximum value (inclusive)
        
        Returns:
            NumberValue: Random integer in specified range
        """
        if not isinstance(min_val, NumberValue):
            raise TypeError(f"Min value must be number, got {min_val.get_type()}")
        if not isinstance(max_val, NumberValue):
            raise TypeError(f"Max value must be number, got {max_val.get_type()}")
        
        min_int = int(min_val.value)
        max_int = int(max_val.value)
        
        if min_int > max_int:
            raise ValueError(f"Min value {min_int} cannot be greater than max value {max_int}")
        
        if self._is_seeded:
            value = self._deterministic_generator.randint(min_int, max_int)
        else:
            value = self._secure_generator.randint(min_int, max_int)
        
        return NumberValue(value, min_val.position)
    
    def uniform(self, min_val: NumberValue, max_val: NumberValue) -> NumberValue:
        """
        Generate a random float from uniform distribution in range [min, max).
        
        Args:
            min_val: Minimum value (inclusive)
            max_val: Maximum value (exclusive)
        
        Returns:
            NumberValue: Random float in specified range
        """
        if not isinstance(min_val, NumberValue):
            raise TypeError(f"Min value must be number, got {min_val.get_type()}")
        if not isinstance(max_val, NumberValue):
            raise TypeError(f"Max value must be number, got {max_val.get_type()}")
        
        min_float = float(min_val.value)
        max_float = float(max_val.value)
        
        if min_float >= max_float:
            raise ValueError(f"Min value {min_float} must be less than max value {max_float}")
        
        if self._is_seeded:
            value = self._deterministic_generator.uniform(min_float, max_float)
        else:
            value = self._secure_generator.uniform(min_float, max_float)
        
        return NumberValue(value, min_val.position)
    
    # === Statistical Distributions ===
    
    def normal(self, mean: NumberValue, std_dev: NumberValue) -> NumberValue:
        """
        Generate a random number from normal (Gaussian) distribution.
        
        Args:
            mean: Mean of the distribution
            std_dev: Standard deviation of the distribution
        
        Returns:
            NumberValue: Random number from normal distribution
        """
        if not isinstance(mean, NumberValue):
            raise TypeError(f"Mean must be number, got {mean.get_type()}")
        if not isinstance(std_dev, NumberValue):
            raise TypeError(f"Standard deviation must be number, got {std_dev.get_type()}")
        
        mean_val = float(mean.value)
        std_val = float(std_dev.value)
        
        if std_val <= 0:
            raise ValueError(f"Standard deviation must be positive, got {std_val}")
        
        if self._is_seeded:
            value = self._deterministic_generator.normalvariate(mean_val, std_val)
        else:
            value = self._secure_generator.normalvariate(mean_val, std_val)
        
        return NumberValue(value, mean.position)
    
    def exponential(self, lambda_val: NumberValue) -> NumberValue:
        """
        Generate a random number from exponential distribution.
        
        Args:
            lambda_val: Rate parameter (lambda) of the distribution
        
        Returns:
            NumberValue: Random number from exponential distribution
        """
        if not isinstance(lambda_val, NumberValue):
            raise TypeError(f"Lambda must be number, got {lambda_val.get_type()}")
        
        lambda_float = float(lambda_val.value)
        
        if lambda_float <= 0:
            raise ValueError(f"Lambda must be positive, got {lambda_float}")
        
        if self._is_seeded:
            value = self._deterministic_generator.expovariate(lambda_float)
        else:
            value = self._secure_generator.expovariate(lambda_float)
        
        return NumberValue(value, lambda_val.position)
    
    def gamma(self, alpha: NumberValue, beta: NumberValue) -> NumberValue:
        """
        Generate a random number from gamma distribution.
        
        Args:
            alpha: Shape parameter
            beta: Scale parameter
        
        Returns:
            NumberValue: Random number from gamma distribution
        """
        if not isinstance(alpha, NumberValue):
            raise TypeError(f"Alpha must be number, got {alpha.get_type()}")
        if not isinstance(beta, NumberValue):
            raise TypeError(f"Beta must be number, got {beta.get_type()}")
        
        alpha_val = float(alpha.value)
        beta_val = float(beta.value)
        
        if alpha_val <= 0:
            raise ValueError(f"Alpha must be positive, got {alpha_val}")
        if beta_val <= 0:
            raise ValueError(f"Beta must be positive, got {beta_val}")
        
        if self._is_seeded:
            value = self._deterministic_generator.gammavariate(alpha_val, beta_val)
        else:
            value = self._secure_generator.gammavariate(alpha_val, beta_val)
        
        return NumberValue(value, alpha.position)
    
    # === Seeding and State Management ===
    
    def seed(self, seed_value: Optional[NumberValue] = None) -> NoneValue:
        """
        Seed the deterministic random number generator for reproducible results.
        
        Args:
            seed_value: Seed value (if None, uses system time)
        
        Returns:
            NoneValue
        """
        if seed_value is not None:
            if not isinstance(seed_value, NumberValue):
                raise TypeError(f"Seed must be number or none, got {seed_value.get_type()}")
            seed_int = int(seed_value.value)
        else:
            seed_int = None  # Will use system time
        
        self._deterministic_generator.seed(seed_int)
        self._is_seeded = True
        self._current_seed = seed_int
        
        return NoneValue(seed_value.position if seed_value else SourcePosition(0, 0))
    
    def get_state(self) -> StringValue:
        """
        Get the current state of the random number generator as a string.
        
        Returns:
            StringValue: String representation of generator state
        """
        if not self._is_seeded:
            return StringValue("unseeded", SourcePosition(0, 0))
        
        state_info = f"seeded:{self._current_seed}"
        return StringValue(state_info, SourcePosition(0, 0))
    
    def reset(self) -> NoneValue:
        """
        Reset the random number generator to unseeded state.
        
        Returns:
            NoneValue
        """
        self._deterministic_generator = random.Random()
        self._is_seeded = False
        self._current_seed = None
        
        return NoneValue(SourcePosition(0, 0))
    
    # === Random Selection and Sampling ===
    
    def choice(self, choices: ListValue) -> GlangValue:
        """
        Choose a random element from a list.
        
        Args:
            choices: List of values to choose from
        
        Returns:
            GlangValue: Randomly selected element
        """
        if not isinstance(choices, ListValue):
            raise TypeError(f"Choices must be list, got {choices.get_type()}")
        
        if not choices.elements:
            raise ValueError("Cannot choose from empty list")
        
        if self._is_seeded:
            selected = self._deterministic_generator.choice(choices.elements)
        else:
            selected = self._secure_generator.choice(choices.elements)
        
        return selected
    
    def sample(self, population: ListValue, sample_size: NumberValue) -> ListValue:
        """
        Select a random sample from a population without replacement.
        
        Args:
            population: List to sample from
            sample_size: Number of elements to select
        
        Returns:
            ListValue: List of randomly selected elements
        """
        if not isinstance(population, ListValue):
            raise TypeError(f"Population must be list, got {population.get_type()}")
        if not isinstance(sample_size, NumberValue):
            raise TypeError(f"Sample size must be number, got {sample_size.get_type()}")
        
        k = int(sample_size.value)
        
        if k < 0:
            raise ValueError(f"Sample size must be non-negative, got {k}")
        if k > len(population.elements):
            raise ValueError(f"Sample size {k} larger than population {len(population.elements)}")
        
        if self._is_seeded:
            selected = self._deterministic_generator.sample(population.elements, k)
        else:
            selected = self._secure_generator.sample(population.elements, k)
        
        return ListValue(selected, position=population.position, constraint=population.constraint)
    
    def shuffle(self, items: ListValue) -> ListValue:
        """
        Return a new shuffled copy of the list.
        
        Args:
            items: List to shuffle
        
        Returns:
            ListValue: New shuffled list
        """
        if not isinstance(items, ListValue):
            raise TypeError(f"Items must be list, got {items.get_type()}")
        
        # Create a copy to avoid modifying the original
        shuffled_elements = items.elements.copy()
        
        if self._is_seeded:
            self._deterministic_generator.shuffle(shuffled_elements)
        else:
            self._secure_generator.shuffle(shuffled_elements)
        
        return ListValue(shuffled_elements, position=items.position, constraint=items.constraint)
    
    # === Secure Random Functions ===
    
    def secure_random(self) -> NumberValue:
        """
        Generate a cryptographically secure random float.
        Always uses secure generator regardless of seeding.
        
        Returns:
            NumberValue: Secure random float in range [0.0, 1.0)
        """
        value = self._secure_generator.random()
        return NumberValue(value, SourcePosition(0, 0))
    
    def secure_randint(self, min_val: NumberValue, max_val: NumberValue) -> NumberValue:
        """
        Generate a cryptographically secure random integer.
        Always uses secure generator regardless of seeding.
        
        Args:
            min_val: Minimum value (inclusive)
            max_val: Maximum value (inclusive)
        
        Returns:
            NumberValue: Secure random integer in specified range
        """
        if not isinstance(min_val, NumberValue):
            raise TypeError(f"Min value must be number, got {min_val.get_type()}")
        if not isinstance(max_val, NumberValue):
            raise TypeError(f"Max value must be number, got {max_val.get_type()}")
        
        min_int = int(min_val.value)
        max_int = int(max_val.value)
        
        if min_int > max_int:
            raise ValueError(f"Min value {min_int} cannot be greater than max value {max_int}")
        
        value = self._secure_generator.randint(min_int, max_int)
        return NumberValue(value, min_val.position)
    
    def secure_token(self, length: NumberValue) -> StringValue:
        """
        Generate a cryptographically secure random token string.
        
        Args:
            length: Length of the token in bytes
        
        Returns:
            StringValue: Secure random token as hex string
        """
        if not isinstance(length, NumberValue):
            raise TypeError(f"Length must be number, got {length.get_type()}")
        
        byte_length = int(length.value)
        
        if byte_length <= 0:
            raise ValueError(f"Length must be positive, got {byte_length}")
        
        token = secrets.token_hex(byte_length)
        return StringValue(token, length.position)
    
    # === UUID Generation ===
    
    def uuid4(self) -> StringValue:
        """
        Generate a random UUID (version 4).
        
        Returns:
            StringValue: Random UUID as string
        """
        uuid_value = str(uuid.uuid4())
        return StringValue(uuid_value, SourcePosition(0, 0))
    
    def uuid1(self) -> StringValue:
        """
        Generate a time-based UUID (version 1).
        
        Returns:
            StringValue: Time-based UUID as string
        """
        uuid_value = str(uuid.uuid1())
        return StringValue(uuid_value, SourcePosition(0, 0))


def create_random_module_namespace():
    """Create the namespace for the built-in Random module."""
    from .module_builder import create_module

    module = RandomModule()

    return create_module(
        "random",
        functions={
            # Core random functions
            'random': module.random,
            'randint': module.randint,
            'uniform': module.uniform,

            # Statistical distributions
            'normal': module.normal,
            'exponential': module.exponential,
            'gamma': module.gamma,

            # Seeding and state
            'seed': module.seed,
            'get_state': module.get_state,
            'reset': module.reset,

            # Random selection
            'choice': module.choice,
            'sample': module.sample,
            'shuffle': module.shuffle,

            # Secure random
            'secure_random': module.secure_random,
            'secure_randint': module.secure_randint,
            'secure_token': module.secure_token,

            # UUID generation
            'uuid4': module.uuid4,
            'uuid1': module.uuid1,
        }
    )
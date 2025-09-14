"""Tests for the random module."""

import pytest
from glang.modules.random_module import RandomModule, create_random_module_namespace
from glang.execution.values import StringValue, NumberValue, ListValue, NoneValue
from glang.ast.nodes import SourcePosition


class TestRandomModule:
    """Test the random module functionality."""
    
    def setup_method(self):
        self.random = RandomModule()
        self.pos = SourcePosition(1, 1)
    
    def test_random_basic(self):
        """Test basic random number generation."""
        result = self.random.random()
        
        assert isinstance(result, NumberValue)
        assert 0.0 <= result.value < 1.0
    
    def test_randint_basic(self):
        """Test random integer generation."""
        min_val = NumberValue(1, self.pos)
        max_val = NumberValue(10, self.pos)
        
        result = self.random.randint(min_val, max_val)
        
        assert isinstance(result, NumberValue)
        assert 1 <= int(result.value) <= 10
        assert result.value == int(result.value)  # Should be integer
    
    def test_randint_validation(self):
        """Test randint parameter validation."""
        min_val = NumberValue(10, self.pos)
        max_val = NumberValue(5, self.pos)
        
        with pytest.raises(ValueError, match="Min value.*cannot be greater than max value"):
            self.random.randint(min_val, max_val)
    
    def test_uniform_basic(self):
        """Test uniform distribution."""
        min_val = NumberValue(2.5, self.pos)
        max_val = NumberValue(7.5, self.pos)
        
        result = self.random.uniform(min_val, max_val)
        
        assert isinstance(result, NumberValue)
        assert 2.5 <= result.value < 7.5
    
    def test_uniform_validation(self):
        """Test uniform parameter validation."""
        min_val = NumberValue(5.0, self.pos)
        max_val = NumberValue(5.0, self.pos)  # Equal values
        
        with pytest.raises(ValueError, match="Min value.*must be less than max value"):
            self.random.uniform(min_val, max_val)
    
    def test_normal_distribution(self):
        """Test normal distribution generation."""
        mean = NumberValue(0.0, self.pos)
        std_dev = NumberValue(1.0, self.pos)
        
        # Generate multiple samples to test distribution properties
        samples = []
        for _ in range(100):
            result = self.random.normal(mean, std_dev)
            assert isinstance(result, NumberValue)
            samples.append(result.value)
        
        # Basic statistical checks (not rigorous, but should catch major issues)
        sample_mean = sum(samples) / len(samples)
        assert -1.0 < sample_mean < 1.0  # Should be roughly centered around 0
    
    def test_normal_validation(self):
        """Test normal distribution parameter validation."""
        mean = NumberValue(0.0, self.pos)
        std_dev = NumberValue(-1.0, self.pos)  # Negative std dev
        
        with pytest.raises(ValueError, match="Standard deviation must be positive"):
            self.random.normal(mean, std_dev)
    
    def test_exponential_distribution(self):
        """Test exponential distribution."""
        lambda_val = NumberValue(1.5, self.pos)
        
        result = self.random.exponential(lambda_val)
        
        assert isinstance(result, NumberValue)
        assert result.value >= 0  # Exponential distribution is non-negative
    
    def test_exponential_validation(self):
        """Test exponential distribution parameter validation."""
        lambda_val = NumberValue(-1.0, self.pos)  # Negative lambda
        
        with pytest.raises(ValueError, match="Lambda must be positive"):
            self.random.exponential(lambda_val)
    
    def test_gamma_distribution(self):
        """Test gamma distribution."""
        alpha = NumberValue(2.0, self.pos)
        beta = NumberValue(1.5, self.pos)
        
        result = self.random.gamma(alpha, beta)
        
        assert isinstance(result, NumberValue)
        assert result.value >= 0  # Gamma distribution is non-negative
    
    def test_gamma_validation(self):
        """Test gamma distribution parameter validation."""
        alpha = NumberValue(-1.0, self.pos)  # Negative alpha
        beta = NumberValue(1.0, self.pos)
        
        with pytest.raises(ValueError, match="Alpha must be positive"):
            self.random.gamma(alpha, beta)
    
    def test_seeding(self):
        """Test deterministic seeding."""
        seed_val = NumberValue(12345, self.pos)
        
        # Seed the generator
        self.random.seed(seed_val)
        
        # Generate some numbers
        first_sequence = [self.random.random().value for _ in range(5)]
        
        # Re-seed with same value
        self.random.seed(seed_val)
        
        # Generate same sequence
        second_sequence = [self.random.random().value for _ in range(5)]
        
        # Should be identical
        assert first_sequence == second_sequence
    
    def test_get_state(self):
        """Test getting generator state."""
        # Initially unseeded
        state = self.random.get_state()
        assert isinstance(state, StringValue)
        assert state.value == "unseeded"
        
        # After seeding
        seed_val = NumberValue(42, self.pos)
        self.random.seed(seed_val)
        
        state = self.random.get_state()
        assert "seeded:42" in state.value
    
    def test_reset(self):
        """Test resetting generator."""
        # Seed the generator
        seed_val = NumberValue(123, self.pos)
        self.random.seed(seed_val)
        
        # Verify it's seeded
        state = self.random.get_state()
        assert "seeded" in state.value
        
        # Reset
        self.random.reset()
        
        # Should be unseeded again
        state = self.random.get_state()
        assert state.value == "unseeded"
    
    def test_choice(self):
        """Test random choice from list."""
        elements = [
            NumberValue(1, self.pos),
            NumberValue(2, self.pos), 
            NumberValue(3, self.pos)
        ]
        choices = ListValue(elements, self.pos)
        
        result = self.random.choice(choices)
        
        assert result in elements
    
    def test_choice_empty_list(self):
        """Test choice from empty list raises error."""
        empty_list = ListValue([], self.pos)
        
        with pytest.raises(ValueError, match="Cannot choose from empty list"):
            self.random.choice(empty_list)
    
    def test_sample(self):
        """Test sampling without replacement."""
        elements = [NumberValue(i, self.pos) for i in range(10)]
        population = ListValue(elements, self.pos)
        sample_size = NumberValue(3, self.pos)
        
        result = self.random.sample(population, sample_size)
        
        assert isinstance(result, ListValue)
        assert len(result.elements) == 3
        
        # All elements should be unique (no replacement)
        assert len(set(elem.value for elem in result.elements)) == 3
        
        # All elements should be from original population
        for elem in result.elements:
            assert elem in elements
    
    def test_sample_validation(self):
        """Test sample parameter validation."""
        elements = [NumberValue(i, self.pos) for i in range(3)]
        population = ListValue(elements, self.pos)
        sample_size = NumberValue(5, self.pos)  # Larger than population
        
        with pytest.raises(ValueError, match="Sample size.*larger than population"):
            self.random.sample(population, sample_size)
    
    def test_shuffle(self):
        """Test list shuffling."""
        elements = [NumberValue(i, self.pos) for i in range(10)]
        original_list = ListValue(elements.copy(), self.pos)
        
        shuffled = self.random.shuffle(original_list)
        
        assert isinstance(shuffled, ListValue)
        assert len(shuffled.elements) == len(original_list.elements)
        
        # Should contain same elements (just reordered)
        original_values = sorted([elem.value for elem in original_list.elements])
        shuffled_values = sorted([elem.value for elem in shuffled.elements])
        assert original_values == shuffled_values
        
        # Original should be unchanged
        assert original_list.elements == elements
    
    def test_secure_random(self):
        """Test secure random generation."""
        result = self.random.secure_random()
        
        assert isinstance(result, NumberValue)
        assert 0.0 <= result.value < 1.0
    
    def test_secure_randint(self):
        """Test secure random integer generation."""
        min_val = NumberValue(100, self.pos)
        max_val = NumberValue(200, self.pos)
        
        result = self.random.secure_randint(min_val, max_val)
        
        assert isinstance(result, NumberValue)
        assert 100 <= int(result.value) <= 200
    
    def test_secure_token(self):
        """Test secure token generation."""
        length = NumberValue(16, self.pos)
        
        result = self.random.secure_token(length)
        
        assert isinstance(result, StringValue)
        assert len(result.value) == 32  # 16 bytes = 32 hex characters
        
        # Should be valid hex string
        int(result.value, 16)  # Will raise ValueError if not hex
    
    def test_secure_token_validation(self):
        """Test secure token parameter validation."""
        length = NumberValue(-1, self.pos)  # Negative length
        
        with pytest.raises(ValueError, match="Length must be positive"):
            self.random.secure_token(length)
    
    def test_uuid4(self):
        """Test UUID4 generation."""
        result = self.random.uuid4()
        
        assert isinstance(result, StringValue)
        assert len(result.value) == 36  # Standard UUID format
        assert result.value.count('-') == 4  # Standard UUID has 4 hyphens
        
        # Should be different each time
        result2 = self.random.uuid4()
        assert result.value != result2.value
    
    def test_uuid1(self):
        """Test UUID1 generation."""
        result = self.random.uuid1()
        
        assert isinstance(result, StringValue)
        assert len(result.value) == 36  # Standard UUID format
        assert result.value.count('-') == 4  # Standard UUID has 4 hyphens
    
    def test_secure_independence_from_seeding(self):
        """Test that secure functions are independent of seeding."""
        # Generate secure numbers without seeding
        secure1 = self.random.secure_random().value
        secure_int1 = self.random.secure_randint(NumberValue(1, self.pos), NumberValue(100, self.pos)).value
        
        # Seed the generator
        self.random.seed(NumberValue(12345, self.pos))
        
        # Generate secure numbers after seeding
        secure2 = self.random.secure_random().value
        secure_int2 = self.random.secure_randint(NumberValue(1, self.pos), NumberValue(100, self.pos)).value
        
        # Secure functions should still produce different results
        # (extremely unlikely to be the same by chance)
        assert secure1 != secure2 or secure_int1 != secure_int2
    
    def test_type_validation(self):
        """Test type validation for method parameters."""
        invalid_value = StringValue("not_a_number", self.pos)
        
        with pytest.raises(TypeError, match="must be number"):
            self.random.randint(invalid_value, NumberValue(10, self.pos))
        
        with pytest.raises(TypeError, match="must be list"):
            self.random.choice(invalid_value)


class TestRandomModuleIntegration:
    """Test random module integration with Glang module system."""
    
    def test_module_namespace_creation(self):
        """Test that module namespace is created correctly."""
        namespace = create_random_module_namespace()
        
        assert namespace.filename == "random"
        
        # Check that all expected functions are available
        expected_functions = [
            'random', 'randint', 'uniform',
            'normal', 'exponential', 'gamma',
            'seed', 'get_state', 'reset',
            'choice', 'sample', 'shuffle',
            'secure_random', 'secure_randint', 'secure_token',
            'uuid4', 'uuid1'
        ]
        
        for func_name in expected_functions:
            assert namespace.get_symbol(func_name) is not None, f"Missing function: {func_name}"


class TestRandomPracticalExamples:
    """Test practical random usage examples."""
    
    def setup_method(self):
        self.random = RandomModule()
        self.pos = SourcePosition(1, 1)
    
    def test_dice_simulation(self):
        """Test simulating dice rolls."""
        # Roll a six-sided die 100 times
        rolls = []
        for _ in range(100):
            roll = self.random.randint(NumberValue(1, self.pos), NumberValue(6, self.pos))
            rolls.append(roll.value)
        
        # All rolls should be valid dice values
        for roll in rolls:
            assert 1 <= roll <= 6
            assert roll == int(roll)  # Should be integers
    
    def test_password_generation(self):
        """Test generating random passwords."""
        # Create character set
        chars = []
        
        # Add letters and numbers
        for i in range(26):
            chars.append(StringValue(chr(ord('a') + i), self.pos))  # lowercase
            chars.append(StringValue(chr(ord('A') + i), self.pos))  # uppercase
        
        for i in range(10):
            chars.append(StringValue(str(i), self.pos))  # digits
        
        char_list = ListValue(chars, self.pos)
        
        # Generate 12-character password
        password_chars = []
        for _ in range(12):
            char = self.random.choice(char_list)
            password_chars.append(char.value)
        
        password = ''.join(password_chars)
        assert len(password) == 12
        
        # Should contain mixed characters
        has_lower = any(c.islower() for c in password)
        has_upper = any(c.isupper() for c in password)
        has_digit = any(c.isdigit() for c in password)
        
        # Note: Not guaranteed, but very likely with 12 characters from mixed set
        # This is more of a smoke test than a rigorous validation
    
    def test_lottery_simulation(self):
        """Test lottery number generation."""
        # Generate 6 unique numbers from 1 to 49
        numbers = [NumberValue(i, self.pos) for i in range(1, 50)]
        population = ListValue(numbers, self.pos)
        sample_size = NumberValue(6, self.pos)
        
        lottery_numbers = self.random.sample(population, sample_size)
        
        assert len(lottery_numbers.elements) == 6
        
        # All numbers should be unique
        values = [elem.value for elem in lottery_numbers.elements]
        assert len(set(values)) == 6
        
        # All should be in valid range
        for value in values:
            assert 1 <= value <= 49
    
    def test_monte_carlo_simulation(self):
        """Test Monte Carlo estimation of pi."""
        # Estimate pi by random sampling in unit circle
        inside_circle = 0
        total_samples = 1000
        
        for _ in range(total_samples):
            x = self.random.uniform(NumberValue(-1, self.pos), NumberValue(1, self.pos)).value
            y = self.random.uniform(NumberValue(-1, self.pos), NumberValue(1, self.pos)).value
            
            if x*x + y*y <= 1:
                inside_circle += 1
        
        pi_estimate = 4 * inside_circle / total_samples
        
        # Should be reasonably close to pi (this is probabilistic)
        # Allow for significant variance with only 1000 samples
        assert 2.5 < pi_estimate < 3.8
    
    def test_gaussian_noise_generation(self):
        """Test generating gaussian noise for simulation."""
        # Generate noise with mean=0, std=0.1
        mean = NumberValue(0.0, self.pos)
        std_dev = NumberValue(0.1, self.pos)
        
        noise_samples = []
        for _ in range(100):
            noise = self.random.normal(mean, std_dev).value
            noise_samples.append(noise)
        
        # Basic statistical properties
        sample_mean = sum(noise_samples) / len(noise_samples)
        
        # Mean should be close to 0 (allowing for sampling variance)
        assert -0.1 < sample_mean < 0.1
        
        # Most samples should be within 3 standard deviations
        within_3_sigma = sum(1 for x in noise_samples if abs(x) <= 0.3)
        assert within_3_sigma / len(noise_samples) > 0.9  # Should be ~99.7%
    
    def test_reproducible_simulation(self):
        """Test reproducible results with seeding."""
        seed_val = NumberValue(98765, self.pos)
        
        # Run simulation 1
        self.random.seed(seed_val)
        results1 = []
        for _ in range(10):
            roll = self.random.randint(NumberValue(1, self.pos), NumberValue(20, self.pos))
            results1.append(roll.value)
        
        # Run simulation 2 with same seed
        self.random.seed(seed_val)
        results2 = []
        for _ in range(10):
            roll = self.random.randint(NumberValue(1, self.pos), NumberValue(20, self.pos))
            results2.append(roll.value)
        
        # Should produce identical results
        assert results1 == results2
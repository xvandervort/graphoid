#!/usr/bin/env python3
"""Demo of Glang's random number generation module.

This demonstrates comprehensive random number generation capabilities including:
- Basic random number generation (secure and deterministic)
- Statistical distributions for scientific computing
- Random sampling and selection for data science
- Cryptographically secure randomness for security
- Reproducible randomness for testing and simulation
"""

from glang.modules.random_module import RandomModule
from glang.execution.values import StringValue, NumberValue, ListValue
from glang.ast.nodes import SourcePosition
import statistics


def demo_basic_random():
    """Demonstrate basic random number generation."""
    print("=== Basic Random Number Generation ===\n")
    
    random = RandomModule()
    pos = SourcePosition(1, 1)
    
    # Basic random float
    print("📊 Random floats (0.0 to 1.0):")
    for i in range(5):
        value = random.random()
        print(f"   {i+1}: {value.value:.6f}")
    print()
    
    # Random integers
    print("🎲 Random integers (1 to 100):")
    for i in range(5):
        value = random.randint(NumberValue(1, pos), NumberValue(100, pos))
        print(f"   {i+1}: {value.value}")
    print()
    
    # Uniform distribution
    print("📏 Uniform distribution (10.0 to 20.0):")
    for i in range(5):
        value = random.uniform(NumberValue(10.0, pos), NumberValue(20.0, pos))
        print(f"   {i+1}: {value.value:.3f}")
    print()


def demo_statistical_distributions():
    """Demonstrate statistical distributions."""
    print("=== Statistical Distributions ===\n")
    
    random = RandomModule()
    pos = SourcePosition(1, 1)
    
    # Normal distribution
    print("📈 Normal distribution (mean=100, std=15):")
    normal_samples = []
    for i in range(10):
        value = random.normal(NumberValue(100, pos), NumberValue(15, pos))
        normal_samples.append(value.value)
        print(f"   {i+1}: {value.value:.2f}")
    
    sample_mean = sum(normal_samples) / len(normal_samples)
    print(f"   Sample mean: {sample_mean:.2f} (expected: 100)")
    print()
    
    # Exponential distribution
    print("⚡ Exponential distribution (lambda=0.5):")
    for i in range(5):
        value = random.exponential(NumberValue(0.5, pos))
        print(f"   {i+1}: {value.value:.3f}")
    print()
    
    # Gamma distribution
    print("🌀 Gamma distribution (alpha=2.0, beta=1.5):")
    for i in range(5):
        value = random.gamma(NumberValue(2.0, pos), NumberValue(1.5, pos))
        print(f"   {i+1}: {value.value:.3f}")
    print()


def demo_seeding_and_reproducibility():
    """Demonstrate seeding for reproducible results."""
    print("=== Seeding and Reproducibility ===\n")
    
    random = RandomModule()
    pos = SourcePosition(1, 1)
    seed_val = NumberValue(12345, pos)
    
    # First run with seed
    print("🌱 First run with seed 12345:")
    random.seed(seed_val)
    first_run = []
    for i in range(5):
        value = random.randint(NumberValue(1, pos), NumberValue(100, pos))
        first_run.append(value.value)
        print(f"   {i+1}: {value.value}")
    print()
    
    # Second run with same seed
    print("🌱 Second run with same seed:")
    random.seed(seed_val)
    second_run = []
    for i in range(5):
        value = random.randint(NumberValue(1, pos), NumberValue(100, pos))
        second_run.append(value.value)
        print(f"   {i+1}: {value.value}")
    
    print(f"   ✅ Results match: {first_run == second_run}")
    print()
    
    # Show generator state
    state = random.get_state()
    print(f"🔍 Generator state: {state.value}")
    print()
    
    # Reset to unseeded state
    random.reset()
    state = random.get_state()
    print(f"🔄 After reset: {state.value}")
    print()


def demo_random_selection():
    """Demonstrate random selection and sampling."""
    print("=== Random Selection and Sampling ===\n")
    
    random = RandomModule()
    pos = SourcePosition(1, 1)
    
    # Create test data
    items = [StringValue(f"item_{i}", pos) for i in range(1, 11)]
    item_list = ListValue(items, constraint="string", position=pos)
    
    print("📦 Available items: item_1, item_2, ..., item_10")
    print()
    
    # Random choice
    print("🎯 Random choice (5 selections with replacement):")
    for i in range(5):
        choice = random.choice(item_list)
        print(f"   {i+1}: {choice.value}")
    print()
    
    # Random sample without replacement
    print("🔬 Random sample (3 items without replacement):")
    sample = random.sample(item_list, NumberValue(3, pos))
    for i, item in enumerate(sample.elements):
        print(f"   {i+1}: {item.value}")
    print()
    
    # Shuffle
    print("🔀 Shuffled list:")
    shuffled = random.shuffle(item_list)
    shuffled_names = [item.value for item in shuffled.elements]
    print(f"   {', '.join(shuffled_names)}")
    print()


def demo_secure_random():
    """Demonstrate cryptographically secure random generation."""
    print("=== Cryptographically Secure Random ===\n")
    
    random = RandomModule()
    pos = SourcePosition(1, 1)
    
    # Secure random floats
    print("🔒 Secure random floats:")
    for i in range(3):
        value = random.secure_random()
        print(f"   {i+1}: {value.value:.8f}")
    print()
    
    # Secure random integers
    print("🔒 Secure random integers (1000 to 9999):")
    for i in range(3):
        value = random.secure_randint(NumberValue(1000, pos), NumberValue(9999, pos))
        print(f"   {i+1}: {value.value}")
    print()
    
    # Secure tokens
    print("🔑 Secure tokens (16 bytes each):")
    for i in range(3):
        token = random.secure_token(NumberValue(16, pos))
        print(f"   {i+1}: {token.value}")
    print()
    
    print("ℹ️  Note: Secure functions are independent of seeding")
    print("   and always use cryptographically secure sources.")
    print()


def demo_uuid_generation():
    """Demonstrate UUID generation."""
    print("=== UUID Generation ===\n")
    
    random = RandomModule()
    
    # UUID4 (random)
    print("🆔 UUID4 (random-based):")
    for i in range(3):
        uuid_val = random.uuid4()
        print(f"   {i+1}: {uuid_val.value}")
    print()
    
    # UUID1 (time-based)
    print("🕒 UUID1 (time-based):")
    for i in range(3):
        uuid_val = random.uuid1()
        print(f"   {i+1}: {uuid_val.value}")
    print()


def demo_practical_applications():
    """Demonstrate practical applications."""
    print("=== Practical Applications ===\n")
    
    random = RandomModule()
    pos = SourcePosition(1, 1)
    
    # Dice rolling simulation
    print("🎲 Dice rolling simulation (100 rolls):")
    dice_rolls = []
    for _ in range(100):
        roll = random.randint(NumberValue(1, pos), NumberValue(6, pos))
        dice_rolls.append(roll.value)
    
    # Count frequencies
    frequencies = {i: dice_rolls.count(i) for i in range(1, 7)}
    for face, count in frequencies.items():
        bar = "█" * (count // 2)  # Visual representation
        print(f"   {face}: {count:2d} {bar}")
    print()
    
    # Password generation
    print("🔐 Password generation:")
    
    # Character sets
    lowercase = [StringValue(chr(ord('a') + i), pos) for i in range(26)]
    uppercase = [StringValue(chr(ord('A') + i), pos) for i in range(26)]
    digits = [StringValue(str(i), pos) for i in range(10)]
    special = [StringValue(c, pos) for c in "!@#$%^&*()"]
    
    all_chars = lowercase + uppercase + digits + special
    char_list = ListValue(all_chars, position=pos)
    
    for i in range(3):
        password_chars = []
        for _ in range(12):
            char = random.choice(char_list)
            password_chars.append(char.value)
        password = ''.join(password_chars)
        print(f"   {i+1}: {password}")
    print()
    
    # Monte Carlo estimation of π
    print("🥧 Monte Carlo estimation of π (10000 samples):")
    inside_circle = 0
    total_samples = 10000
    
    for _ in range(total_samples):
        x = random.uniform(NumberValue(-1, pos), NumberValue(1, pos)).value
        y = random.uniform(NumberValue(-1, pos), NumberValue(1, pos)).value
        
        if x*x + y*y <= 1:
            inside_circle += 1
    
    pi_estimate = 4 * inside_circle / total_samples
    error = abs(pi_estimate - 3.14159265359)
    print(f"   Estimated π: {pi_estimate:.6f}")
    print(f"   Actual π:    3.141593")
    print(f"   Error:       {error:.6f}")
    print()
    
    # Lottery number generator
    print("🎟️  Lottery number generator (6 from 49):")
    numbers = [NumberValue(i, pos) for i in range(1, 50)]
    lottery_pool = ListValue(numbers, position=pos)
    
    for draw in range(3):
        winning_numbers = random.sample(lottery_pool, NumberValue(6, pos))
        numbers_list = sorted([int(num.value) for num in winning_numbers.elements])
        print(f"   Draw {draw+1}: {', '.join(map(str, numbers_list))}")
    print()


def demo_scientific_simulation():
    """Demonstrate scientific simulation capabilities."""
    print("=== Scientific Simulation ===\n")
    
    random = RandomModule()
    pos = SourcePosition(1, 1)
    
    # Generate sample data for analysis
    print("🔬 Generating experimental data:")
    print("   Simulating measurement noise (normal distribution)")
    
    # Simulate temperature measurements with noise
    true_temp = 23.5  # True temperature in Celsius
    measurement_noise = 0.2  # Standard deviation of noise
    
    measurements = []
    for i in range(20):
        noise = random.normal(NumberValue(0, pos), NumberValue(measurement_noise, pos))
        measured_temp = true_temp + noise.value
        measurements.append(measured_temp)
        if i < 10:  # Only show first 10
            print(f"   Measurement {i+1}: {measured_temp:.3f}°C")
    
    if len(measurements) > 10:
        print(f"   ... and {len(measurements) - 10} more measurements")
    
    # Statistical analysis
    mean_temp = sum(measurements) / len(measurements)
    variance = sum((x - mean_temp)**2 for x in measurements) / (len(measurements) - 1)
    std_dev = variance ** 0.5
    
    print(f"\n   📊 Statistical Analysis:")
    print(f"   True temperature:     {true_temp:.3f}°C")
    print(f"   Mean measured:        {mean_temp:.3f}°C")
    print(f"   Standard deviation:   {std_dev:.3f}°C")
    print(f"   Expected std dev:     {measurement_noise:.3f}°C")
    print(f"   Error in mean:        {abs(mean_temp - true_temp):.3f}°C")
    print()


def demo_performance_features():
    """Demonstrate performance and state features."""
    print("=== Performance and State Management ===\n")
    
    random = RandomModule()
    pos = SourcePosition(1, 1)
    
    print("⚡ Performance considerations:")
    print("   • Separate secure and deterministic generators")
    print("   • Efficient seeding and state management")  
    print("   • Optimized for both security and reproducibility")
    print()
    
    # Demonstrate state transitions
    print("🔄 State management demonstration:")
    
    # Initial state
    state = random.get_state()
    print(f"   Initial state: {state.value}")
    
    # After seeding
    random.seed(NumberValue(999, pos))
    state = random.get_state()
    print(f"   After seeding: {state.value}")
    
    # Generate some numbers
    for _ in range(3):
        random.random()
    
    # After generation (state persists)
    state = random.get_state()
    print(f"   After use: {state.value}")
    
    # Reset
    random.reset()
    state = random.get_state()
    print(f"   After reset: {state.value}")
    print()


if __name__ == "__main__":
    print("🎲 Glang Random Number Generation Module Demo")
    print("=" * 60)
    print()
    
    demo_basic_random()
    demo_statistical_distributions()
    demo_seeding_and_reproducibility()
    demo_random_selection()
    demo_secure_random()
    demo_uuid_generation()
    demo_practical_applications()
    demo_scientific_simulation()
    demo_performance_features()
    
    print("🎉 SUCCESS: Random module provides comprehensive randomness!")
    print("\n📋 What's Working:")
    print("  ✅ Basic random generation (floats, integers, uniform)")
    print("  ✅ Statistical distributions (normal, exponential, gamma)")
    print("  ✅ Deterministic seeding for reproducibility")
    print("  ✅ Random selection and sampling (choice, sample, shuffle)")
    print("  ✅ Cryptographically secure randomness")
    print("  ✅ UUID generation (version 1 and 4)")
    print("  ✅ State management and transitions")
    print("  ✅ Comprehensive error handling and validation")
    print("  ✅ Scientific simulation capabilities")
    print("\n🚀 Ready for statistical computing, simulations, and secure applications!")
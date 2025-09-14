# Random Module

The **random** module provides comprehensive random number generation capabilities for Glang applications. It offers both cryptographically secure and deterministic (seeded) random number generation, statistical distributions, sampling functions, and UUID generation.

## Overview

The random module is designed to handle a wide range of randomization needs, from simple dice rolls to complex statistical simulations. It provides two modes of operation:

- **Secure Mode** (default): Uses cryptographically secure random number generation
- **Deterministic Mode**: Uses seeded random generation for reproducible results

### Key Features

- **Basic Random Numbers**: `random()`, `randint()`, `uniform()` for common use cases
- **Statistical Distributions**: `normal()`, `exponential()`, `gamma()` for advanced applications
- **Random Sampling**: `choice()`, `sample()`, `shuffle()` for list operations
- **Secure Random**: `secure_random()`, `secure_token()` for cryptographic applications
- **UUID Generation**: `uuid4()`, `uuid1()` for unique identifier creation
- **Seeding Control**: `seed()`, `reset()`, `get_state()` for reproducible results

## Basic Usage

```glang
import "random" as rand

# Basic random numbers
dice_roll = rand.randint(1, 6)                    # 1-6 inclusive
probability = rand.random()                       # 0.0 to 1.0 (exclusive)
price = rand.uniform(10.0, 50.0)                 # 10.0 to 50.0 (exclusive)

# Random choice from list
colors = ["red", "green", "blue", "yellow"]
chosen_color = rand.choice(colors)                # Pick one random color

# Generate secure token for authentication
auth_token = rand.secure_token(32)               # 64-character hex string
session_id = rand.uuid4()                        # Random UUID
```

## Core Random Functions

### `random()`

Generates a random float between 0.0 and 1.0 (exclusive).

```glang
# Generate random probabilities
prob = rand.random()                             # 0.0 ≤ prob < 1.0

# Use for random events
if rand.random() < 0.3 {                        # 30% chance
    print("Rare event occurred!")
}

# Generate random percentages
percentage = rand.random() * 100                 # 0.0 to 100.0
```

### `randint(min, max)`

Generates a random integer in the inclusive range [min, max].

```glang
# Dice and games
dice = rand.randint(1, 6)                       # Standard die: 1, 2, 3, 4, 5, or 6
card = rand.randint(1, 52)                      # Playing card number
lottery = rand.randint(1, 1000000)              # Lottery ticket

# Array indices
list_size = my_list.size()
random_index = rand.randint(0, list_size - 1)   # Valid list index

# Age simulation
age = rand.randint(18, 65)                      # Working age range
```

### `uniform(min, max)`

Generates a random float from uniform distribution in range [min, max).

```glang
# Physical measurements with precision
temperature = rand.uniform(-10.0, 40.0)         # Temperature in Celsius
weight = rand.uniform(50.5, 120.8)              # Weight in kg
height = rand.uniform(1.5, 2.1)                 # Height in meters

# Financial calculations
price = rand.uniform(9.99, 199.99)              # Product price
discount = rand.uniform(0.05, 0.25)             # 5% to 25% discount
```

## Statistical Distributions

### `normal(mean, std_dev)`

Generates numbers from a normal (Gaussian) distribution.

```glang
# Human characteristics (normally distributed)
height = rand.normal(170.0, 10.0)               # Height: mean=170cm, std=10cm
iq_score = rand.normal(100.0, 15.0)             # IQ: mean=100, std=15
test_score = rand.normal(75.0, 12.0)            # Test score: mean=75, std=12

# Measurement errors
true_value = 100.0
measurement = true_value + rand.normal(0.0, 0.5) # Add measurement noise

# Financial modeling
daily_return = rand.normal(0.001, 0.02)         # Stock daily return
```

### `exponential(lambda)`

Generates numbers from an exponential distribution (useful for time intervals).

```glang
# Time between events
wait_time = rand.exponential(0.5)               # Average wait time = 1/0.5 = 2 units
service_time = rand.exponential(1.0)            # Average service time = 1 unit
failure_time = rand.exponential(0.1)            # Average time to failure = 10 units

# Network delays
network_delay = rand.exponential(2.0)           # Network latency modeling
packet_interval = rand.exponential(10.0)        # Time between packets
```

### `gamma(alpha, beta)`

Generates numbers from a gamma distribution (useful for modeling positive skewed data).

```glang
# Processing times (often gamma-distributed)
task_duration = rand.gamma(2.0, 3.0)            # Shape=2, Scale=3
rendering_time = rand.gamma(1.5, 0.5)           # Graphics rendering time

# Financial modeling
claim_amount = rand.gamma(3.0, 1000.0)          # Insurance claim amounts
income_distribution = rand.gamma(2.5, 20000.0)  # Income modeling
```

## Seeding and Reproducibility

### `seed(value?)`

Seeds the random number generator for reproducible results.

```glang
# Reproducible random sequences
rand.seed(12345)
first_sequence = [rand.randint(1, 100), rand.randint(1, 100), rand.randint(1, 100)]

rand.seed(12345)  # Same seed
second_sequence = [rand.randint(1, 100), rand.randint(1, 100), rand.randint(1, 100)]
# first_sequence == second_sequence (true)

# Seed with current time (default behavior)
rand.seed()       # Uses system time as seed

# Testing with known seeds
rand.seed(42)     # Fixed seed for unit tests
```

### `get_state()`

Returns the current state of the random number generator.

```glang
# Check if generator is seeded
state = rand.get_state()
if state == "unseeded" {
    print("Using secure random generation")
} else {
    print("Using seeded generation: " + state)
}
```

### `reset()`

Resets the generator to unseeded state (secure mode).

```glang
# Switch from deterministic back to secure
rand.seed(12345)         # Now deterministic
value1 = rand.random()   # Predictable

rand.reset()             # Back to secure mode
value2 = rand.random()   # Cryptographically secure
```

## Random Selection and Sampling

### `choice(list)`

Selects a random element from a list.

```glang
# Random selection
colors = ["red", "green", "blue", "yellow"]
chosen = rand.choice(colors)                     # One random color

names = ["Alice", "Bob", "Charlie", "Diana"]
winner = rand.choice(names)                      # Contest winner

# Random responses
responses = ["Yes", "No", "Maybe", "Ask again later"]
magic_8_ball = rand.choice(responses)
```

### `sample(population, count)`

Selects multiple random elements without replacement.

```glang
# Random sampling without replacement
deck = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]
hand = rand.sample(deck, 5)                      # Draw 5 cards without replacement

students = ["Alice", "Bob", "Charlie", "Diana", "Eve"]
test_group = rand.sample(students, 3)            # Select 3 students for testing

# Survey participants
population = generate_population(1000)
survey_sample = rand.sample(population, 50)      # 50 random participants
```

### `shuffle(list)`

Returns a new randomly shuffled copy of a list.

```glang
# Shuffle data
original = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
shuffled = rand.shuffle(original)                # New shuffled list
# original remains unchanged

# Randomize order
playlist = ["Song A", "Song B", "Song C", "Song D"]
random_playlist = rand.shuffle(playlist)

# Game setup
cards = generate_deck()
shuffled_deck = rand.shuffle(cards)              # Shuffle for dealing
```

## Secure Random Functions

### `secure_random()`

Generates cryptographically secure random floats (always secure, ignores seeding).

```glang
# Security applications
security_check = rand.secure_random()           # Always cryptographically secure
threshold = rand.secure_random()                # Even when generator is seeded

# Cryptographic operations
nonce_factor = rand.secure_random()
session_entropy = rand.secure_random()
```

### `secure_randint(min, max)`

Generates cryptographically secure random integers (always secure, ignores seeding).

```glang
# Secure random integers
secure_pin = rand.secure_randint(1000, 9999)    # 4-digit PIN
challenge_code = rand.secure_randint(100000, 999999)  # 6-digit code

# Security tokens
verification_code = rand.secure_randint(10000000, 99999999)  # 8-digit code
```

### `secure_token(length)`

Generates a cryptographically secure random token as a hexadecimal string.

```glang
# Authentication tokens
session_token = rand.secure_token(16)           # 32-character hex string
api_key = rand.secure_token(32)                 # 64-character hex string
password_salt = rand.secure_token(8)            # 16-character hex string

# Database keys
record_id = rand.secure_token(12)               # Unique record identifier
temporary_filename = "temp_" + rand.secure_token(8) + ".dat"
```

## UUID Generation

### `uuid4()`

Generates a random UUID (version 4).

```glang
# Unique identifiers
user_id = rand.uuid4()                          # e.g., "f47ac10b-58cc-4372-a567-0e02b2c3d479"
session_id = rand.uuid4()                       # Globally unique session
transaction_id = rand.uuid4()                   # Financial transaction ID

# File names
temp_file = rand.uuid4() + ".tmp"               # Guaranteed unique filename
backup_name = "backup_" + rand.uuid4() + ".sql"
```

### `uuid1()`

Generates a time-based UUID (version 1).

```glang
# Time-based identifiers (sortable by creation time)
log_entry_id = rand.uuid1()                     # Includes timestamp
event_id = rand.uuid1()                         # Event tracking
order_number = rand.uuid1()                     # Order processing

# Database records with time ordering
record_uuid = rand.uuid1()                      # Can be sorted by creation time
```

## Practical Examples

### Game Development

```glang
import "random" as rand

# Dice rolling system
func roll_dice(sides) {
    return rand.randint(1, sides)
}

func roll_multiple(count, sides) {
    results = []
    for i = 0; i < count; i = i + 1 {
        results.append(roll_dice(sides))
    }
    return results
}

# Usage
d6_roll = roll_dice(6)                          # Standard die
d20_roll = roll_dice(20)                        # RPG die
three_dice = roll_multiple(3, 6)                # Roll 3d6

# Random encounter system
encounters = ["goblin", "orc", "dragon", "treasure", "trap"]
random_encounter = rand.choice(encounters)

# Loot generation
if rand.random() < 0.1 {                        # 10% chance
    rare_item = "magical_sword"
}
```

### Data Simulation

```glang
import "random" as rand

# Generate realistic test data
func generate_person() {
    names = ["Alice", "Bob", "Charlie", "Diana", "Eve"]
    name = rand.choice(names)
    age = rand.randint(18, 65)
    height = rand.normal(170.0, 10.0)           # cm
    weight = rand.normal(70.0, 15.0)            # kg
    
    return {
        "name": name,
        "age": age,
        "height": height,
        "weight": weight
    }
}

# Generate dataset
people = []
for i = 0; i < 100; i = i + 1 {
    people.append(generate_person())
}
```

### A/B Testing

```glang
import "random" as rand

# Reproducible A/B test assignment
func assign_test_group(user_id) {
    # Use user_id as seed for consistent assignment
    rand.seed(user_id.hash())
    
    if rand.random() < 0.5 {
        group = "A"
    } else {
        group = "B"
    }
    
    rand.reset()  # Return to secure mode
    return group
}

# Usage
user_group = assign_test_group(current_user_id)
if user_group == "A" {
    show_old_interface()
} else {
    show_new_interface()
}
```

### Security Token Generation

```glang
import "random" as rand

# Generate various security tokens
func generate_session_token() {
    return rand.secure_token(32)                 # 64-character hex
}

func generate_api_key() {
    prefix = "gsk_"
    token = rand.secure_token(24)                # 48-character hex
    return prefix + token
}

func generate_verification_code() {
    return rand.secure_randint(100000, 999999)   # 6-digit code
}

# Usage
session = generate_session_token()
api_key = generate_api_key()
code = generate_verification_code()
```

## Performance and Best Practices

### When to Use Seeding

**Use seeded random generation for:**
- Unit testing (reproducible test cases)
- Simulations requiring reproducible results
- Debugging random algorithms
- Generating consistent test data

**Use secure random generation for:**
- Cryptographic applications
- Security tokens and passwords
- Session identifiers
- Authentication codes

### Performance Considerations

```glang
# Efficient random sampling
large_list = generate_large_dataset()

# Good: Single sample() call
sample_data = rand.sample(large_list, 100)

# Avoid: Multiple choice() calls (slower)
sample_data = []
for i = 0; i < 100; i = i + 1 {
    sample_data.append(rand.choice(large_list))   # May have duplicates
}
```

### Memory Usage

```glang
# Memory-efficient shuffling
original_data = load_large_dataset()
shuffled = rand.shuffle(original_data)           # Creates new list

# If memory is limited, consider sampling instead
sample_size = 1000
subset = rand.sample(original_data, sample_size) # Smaller memory footprint
```

## Error Handling

The random module provides clear error messages for common mistakes:

```glang
# Invalid range
try {
    rand.randint(10, 5)  # min > max
} catch error {
    # Error: "Min value 10 cannot be greater than max value 5"
}

# Invalid distribution parameters
try {
    rand.normal(0, -1)   # negative standard deviation
} catch error {
    # Error: "Standard deviation must be positive, got -1"
}

# Empty list choice
try {
    empty_list = []
    rand.choice(empty_list)
} catch error {
    # Error: "Cannot choose from empty list"
}
```

## Integration with Other Modules

### With Time Module

```glang
import "random" as rand
import "time" as time

# Random delays
func random_delay() {
    delay_seconds = rand.uniform(1.0, 5.0)
    return time.sleep(delay_seconds)
}

# Random timestamps
base_time = time.now()
random_offset = rand.randint(-86400, 86400)     # ±1 day in seconds
random_time = base_time + random_offset
```

### With JSON Module

```glang
import "random" as rand
import "json" as json

# Generate random data for APIs
func generate_random_user() {
    user_data = {
        "id": rand.uuid4(),
        "name": rand.choice(["Alice", "Bob", "Charlie"]),
        "score": rand.randint(0, 100),
        "created": time.now().to_string()
    }
    return json.encode(user_data)
}
```

## See Also

- [Time Module](time.md) - Date/time operations and delays
- [JSON Module](json.md) - Data serialization and APIs
- [String Methods](../builtins/string_methods.md) - String processing and validation
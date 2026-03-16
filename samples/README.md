# Graphoid Sample Programs

This directory contains 30 carefully curated educational examples demonstrating Graphoid language features, organized by difficulty level.

## Quick Start

**New to Graphoid?** Start here:
```bash
gr samples/01-basics/hello_world.gr
```

Then work through the basics in order:
1. `hello_world.gr` - Your first Graphoid program
2. `functions.gr` - Functions, lambdas, and control flow
3. `collections.gr` - Lists, maps, and transformations
4. `graphs.gr` - Basic graph operations

## Directory Structure

```
samples/
├── 01-basics/          (4 files)  - Start here!
├── 02-intermediate/    (13 files) - Core features
├── 03-advanced/        (9 files)  - Graph algorithms + concurrency
├── 04-modules/         (6 files)  - Code organization
├── 05-stdlib/          (5 files)  - Standard library
└── 06-projects/        (3 projects) - Full applications
```

## Learning Path

**Recommended progression:**

1. **Basics** (30 minutes) → Learn core syntax and concepts
2. **Intermediate** (2-3 hours) → Explore language features
3. **Advanced** (1-2 hours) → Master graph operations
4. **Modules** (1 hour) → Organize multi-file projects
5. **Stdlib** (1 hour) → Leverage standard library

---

## 01-basics/ - Core Language Fundamentals

Start here if you're new to Graphoid.

### `hello_world.gr` ⭐⭐⭐
**Your first Graphoid program**

Topics:
- `print()` function
- Variables and type inference
- Basic math operations
- String methods (`.upper()`, `.len()`, `.replace()`)

```bash
gr samples/01-basics/hello_world.gr
```

Expected output:
```
Hello, World!
Welcome to Graphoid!
Alice is 25 years old
10 + 5 * 2 = 20
...
```

### `functions.gr` ⭐⭐⭐
**Functions, lambdas, and control flow**

Topics:
- Function definitions (`fn`)
- Lambda expressions
- Conditionals (`if`/`else`)
- Loops (`while`, `for`)
- Pattern matching basics

```bash
gr samples/01-basics/functions.gr
```

### `collections.gr` ⭐⭐⭐
**Lists, maps, and transformations**

Topics:
- Lists: `.append()`, `.map()`, `.filter()`, `.reject()`
- Maps (hashes): key-value storage, `.keys()`, `.values()`
- Functional transformations
- Immutability by default

```bash
gr samples/01-basics/collections.gr
```

### `graphs.gr` ⭐⭐
**Basic graph operations**

Topics:
- Creating graphs
- Adding nodes and edges
- Querying graph structure
- Understanding "everything is a graph"

```bash
gr samples/01-basics/graphs.gr
```

---

## 02-intermediate/ - Language Features

Explore Graphoid's unique features.

### `behaviors.gr` ⭐⭐⭐
**Automatic value transformations**

Topics:
- Behavior rules (`.add_rule()`)
- Built-in transformations: `none_to_zero`, `positive`, `round_to_int`
- Range validation: `validate_range`
- Chaining behaviors
- Self-managing data structures

```bash
gr samples/02-intermediate/behaviors.gr
```

**Key Concept:** Rules transform values automatically, eliminating repetitive validation code.

### `pattern_matching.gr` ⭐⭐⭐
**Match expressions**

Topics:
- Matching numbers, strings, booleans
- List patterns and destructuring
- Rest patterns (`...`)
- Binding variables in patterns

```bash
gr samples/02-intermediate/pattern_matching.gr
```

### `string_generators.gr` ⭐⭐⭐
**String generation with static methods**

Topics:
- Repetition mode: `string.generate(" ", 10)` → `"          "`
- Sequence mode: `string.generate("a", "z")` → `"abcdefghijklmnopqrstuvwxyz"`
- Practical uses: bar charts, tables, padding

```bash
gr samples/02-intermediate/string_generators.gr
```

### `string_mutating_methods.gr` ⭐⭐
**Mutating vs non-mutating methods**

Topics:
- Immutability by default
- The `!` suffix for mutation
- `.upper()` vs `.upper!()`
- `.trim()` vs `.trim!()`

```bash
gr samples/02-intermediate/string_mutating_methods.gr
```

**Key Principle:** If you don't see `!`, nothing mutates.

### `string_pattern_methods.gr` ⭐⭐⭐
**String pattern matching**

Topics:
- `.contains()` - check for patterns (`:digits`, `:letters`, `:emails`)
- `.extract()` - extract matching patterns
- `.count()` - count occurrences
- `.find()` - find positions

```bash
gr samples/02-intermediate/string_pattern_methods.gr
```

### `number_methods.gr` ⭐⭐
**Numeric operations**

Topics:
- `.sqrt()`, `.abs()`
- `.up()` (ceiling), `.down()` (floor), `.round()`
- `.log()`, `.ln()`, `.log2()`, `.log10()`
- Practical examples

```bash
gr samples/02-intermediate/number_methods.gr
```

### `universal_casting.gr` ⭐⭐⭐
**Type casting and truthiness**

Topics:
- Casting to `num`, `string`, `bool`
- Truthiness rules
- Empty collection handling
- Practical examples with conditionals

```bash
gr samples/02-intermediate/universal_casting.gr
```

### `integer_mode.gr` ⭐⭐
**Configuration directive: `:integer`**

Topics:
- File-level directives
- `:integer` mode truncates floats
- Scoped configuration blocks
- When to use integer mode

```bash
gr samples/02-intermediate/integer_mode.gr
```

### `function_overloading.gr` ⭐⭐
**Overloading by arity**

Topics:
- Multiple definitions with different parameter counts
- Overloading patterns
- Module function overloading

```bash
gr samples/02-intermediate/function_overloading.gr
```

### `bitwise_operations.gr` ⭐⭐⭐
**Bitwise operators**

Topics:
- AND (`&`), OR (`|`), XOR (`^`), NOT (`~`)
- Left shift (`<<`), right shift (`>>`)
- Power operator (`**`)
- Signed vs unsigned shifts
- Binary literals (`0b1010`)

```bash
gr samples/02-intermediate/bitwise_operations.gr
```

### `bitwise_unsigned.gr` ⭐⭐
**Unsigned right shift with `:unsigned` directive**

Topics:
- Signed arithmetic shift (default)
- Unsigned logical shift (`:unsigned` mode)
- Scoped configuration
- Use cases for unsigned shifts

```bash
gr samples/02-intermediate/bitwise_unsigned.gr
```

### `graph_equality.gr` ⭐⭐⭐
**Graph equality and layer comparison**

Topics:
- Basic equality (`==`) compares data only
- `equals()` method with options
- `include:` mode - compare data plus specified layers
- `only:` mode - compare only specified layers
- Comparison layers: `:data`, `:rules`, `:rulesets`, `:methods`, `:all`
- List equality with behaviors

```bash
gr samples/02-intermediate/graph_equality.gr
```

**Key Concept:** Graphoid graphs have layered architecture. `==` compares data only; use `equals()` with options for precise layer control.

### `exception_handling.gr` ⭐⭐⭐
**Try/catch/finally and error handling**

Topics:
- Basic try/catch blocks
- Multiple catch clauses by error type
- Finally blocks
- Error object methods (`.type()`, `.message()`, `.stack_trace()`)
- Raising custom errors

```bash
gr samples/02-intermediate/exception_handling.gr
```

---

## 03-advanced/ - Graph Pattern Matching

Advanced graph queries and algorithms.

### `property_projection.gr` ⭐⭐
**Pattern matching with property filters**

Topics:
- Graph pattern matching
- Property-based queries
- Edge type filtering
- Node property extraction

```bash
gr samples/03-advanced/property_projection.gr
```

**Prerequisites:** Understanding of graphs and pattern matching.

### `subgraph_operations.gr` ⭐⭐
**Subgraph extraction and manipulation**

Topics:
- `.extract_subgraph()` - select nodes/edges
- `.delete_subgraph()` - remove portions
- `.merge()` - combine graphs
- Conflict resolution strategies

```bash
gr samples/03-advanced/subgraph_operations.gr
```

### `recommendation_system.gr` ⭐⭐
**Friend recommendation algorithm**

Topics:
- Friends-of-friends queries
- Variable-length paths
- Graph traversal patterns
- Social network algorithms

```bash
gr samples/03-advanced/recommendation_system.gr
```

### `social_network_patterns.gr` ⭐⭐
**Social network queries**

Topics:
- Relationship patterns (FRIEND, FOLLOWS)
- Bidirectional connections
- 2-hop queries
- Result projection

```bash
gr samples/03-advanced/social_network_patterns.gr
```

### `variable_length_paths.gr` ⭐⭐
**Variable-length path matching**

Topics:
- 1-hop, 2-hop, N-hop queries
- Path length constraints
- Edge type filtering
- Reachability queries

```bash
gr samples/03-advanced/variable_length_paths.gr
```

### `select.gr` ⭐⭐⭐
**Channel multiplexing with select()**

Topics:
- Basic `select()` usage
- Channel identity comparison (`==`)
- Timeout handling (`timeout:`)
- Non-blocking polling (`default: true`)
- Multi-producer pattern with `spawn`

```bash
gr samples/03-advanced/select.gr
```

**Key Concept:** `select(ch1, ch2, ...)` blocks until any channel has data, returning `[source, msg]`. Compare `source == ch1` to identify which channel fired. Use `timeout:` for time-limited waits and `default: true` for non-blocking polls.

### `actors.gr` ⭐⭐⭐
**Actor-style concurrency with graph-native messaging**

Topics:
- Defining actor graphs (graph with `on_message`)
- Spawning actors with `spawn Actor{}`
- Fire-and-forget messaging (`.send()`)
- Request-response messaging (`.request()`)
- State persistence across messages
- Initial state overrides
- Graph-native messaging (`g.send(to:)`, `g.broadcast()`, `g.request(to:)`)
- Actor lifecycle (`.close()`, `.is_closed()`)

```bash
gr samples/03-advanced/actors.gr
```

**Key Concept:** A graph with `fn on_message(msg)` IS an actor — no separate `actor` keyword. Actors process messages one at a time with isolated state, and can be stored as graph nodes for graph-native messaging.

### `supervision.gr` ⭐⭐⭐
**Actor supervision with automatic restart**

Topics:
- Supervisor template (`graph X from supervisor {}`)
- Supervising child actors (`.supervise()`)
- Automatic restart on crash
- Restart modes (`:permanent`, `:transient`, `:temporary`)
- Custom strategy and `max_restarts`
- Actor `.id()` method

```bash
gr samples/03-advanced/supervision.gr
```

**Key Concept:** Supervisors monitor child actors and automatically restart them on failure. Use `graph X from supervisor {}` to inherit supervisor behavior, then `.supervise(child, restart: :permanent)` to register children.

### `concurrency.gr` ⭐⭐⭐
**Spawn + Channels concurrency**

Topics:
- Creating channels (unbuffered and buffered)
- Sending and receiving values
- Spawning concurrent tasks with `spawn { }`
- Share-nothing semantics
- Multiple producers pattern
- Worker pattern (fan-out computation)
- Channel close and error handling

```bash
gr samples/03-advanced/concurrency.gr
```

**Key Concept:** Graphoid uses share-nothing concurrency — spawned tasks get deep copies of captured values, communicating exclusively through channels.

### `timers.gr` ⭐⭐⭐
**Timer module — sleep, one-shot, and recurring timers**

Topics:
- `timer.sleep()` - blocking delay
- `timer.after()` - one-shot timer returning a channel
- `timer.every()` - recurring timer returning a channel
- Timer cancellation via `channel.close()`
- `for..in` channel iteration
- Spawn + timer patterns

```bash
gr samples/03-advanced/timers.gr
```

**Key Concept:** All timer functions return channels — the same primitive used for spawn communication. No callbacks, no promises.

### `signals.gr` ⭐⭐
**OS signal handling via channels**

Topics:
- `signal.on(:sigint)` - register signal handler
- Graceful shutdown pattern
- Racing signals against timeouts

```bash
gr samples/03-advanced/signals.gr
```

**Key Concept:** Signals are channels too — `signal.on(:sigint)` returns a channel that receives when Ctrl+C is pressed.

---

## 04-modules/ - Code Organization

Learn to organize multi-file projects.

### `app_main.gr` ⭐⭐⭐
**Three-level module hierarchy**

Topics:
- Module imports
- Namespace organization
- Multi-file projects
- Demonstrates: `app → service → utils`

```bash
gr samples/04-modules/app_main.gr
```

**Key Example:** Shows how modules can import other modules, creating a dependency chain.

### `priv_keyword.gr` ⭐⭐⭐
**Private symbols in modules**

Topics:
- `priv` keyword for encapsulation
- Public vs private functions
- Public vs private variables
- API design

```bash
gr samples/04-modules/priv_keyword.gr
```

**Best Practice:** Hide implementation details, expose clean APIs.

### `load_vs_import.gr` ⭐⭐⭐
**Understanding `load` vs `import`**

Topics:
- `import`: Creates namespace, cached, access via `module.symbol`
- `load`: Merges into scope, not cached, direct access
- When to use each
- Practical examples

```bash
gr samples/04-modules/load_vs_import.gr
```

**Critical Distinction:**
- `import` for reusable modules and libraries
- `load` for configuration files and utilities

### Supporting Files

- `service_module.gr` - Used by `app_main.gr`
- `utils_module.gr` - Used by `service_module.gr` and `load_vs_import.gr`
- `modules_main.gr` - Demonstrates using stdlib math module

These files show how modules work together in a multi-file project.

---

## 05-stdlib/ - Standard Library

Explore Graphoid's standard library modules.

### `constants.gr` ⭐⭐⭐
**Mathematical and physical constants**

Topics:
- Math constants: π, e, τ, φ (golden ratio), √2, √3
- Logarithmic constants: ln(2), ln(10), log₂(e), log₁₀(e)
- Angle conversion: degrees ↔ radians
- Physical constants: c (speed of light), G (gravitational), h (Planck)

```bash
gr samples/05-stdlib/constants.gr
```

### `random.gr` ⭐⭐⭐
**Random number generation**

Topics:
- `random.random()` - float in [0, 1)
- `random.randint()` - random integers
- `random.uniform()` - floats in range
- `random.choice()` - pick from list
- `random.sample()` - pick N items
- `random.shuffle()` - randomize order
- `random.normal()` - Gaussian distribution
- `random.exponential()` - exponential distribution
- `random.seed()` - deterministic random
- `random.uuid()` - generate UUIDs

```bash
gr samples/05-stdlib/random.gr
```

### `approx_demo.gr` ⭐⭐⭐
**Approximate equality comparisons**

Topics:
- `approx()` - compare with tolerance
- Absolute tolerance mode
- Relative tolerance mode
- Time comparisons
- Floating-point safety

```bash
gr samples/05-stdlib/approx_demo.gr
```

**Use Case:** Comparing floating-point numbers safely.

### `time_type.gr` ⭐⭐⭐
**Time values and operations**

Topics:
- Creating time values: `time.now()`, `time.from_string()`, `time.from_timestamp()`
- Extracting components: `.year()`, `.month()`, `.day()`, `.hour()`, `.minute()`, `.second()`
- Conversions: `.to_timestamp()`, `.to_string()`
- Practical examples

```bash
gr samples/05-stdlib/time_type.gr
```

### `runtime_introspection.gr` ⭐⭐⭐
**Runtime introspection APIs**

Topics:
- `runtime.version()`, `runtime.uptime()`, `runtime.memory()`
- `modules.list()`, `modules.info()`
- `error.stack()` - structured stack traces
- `__MODULE__` - current module name

```bash
gr samples/05-stdlib/runtime_introspection.gr
```

## 06-projects/ - Full Applications

Real-world applications built with Graphoid.

### `dysregulation/` ⭐⭐⭐
**Systems theory simulation of addiction and regulatory failure**

Topics:
- Complex system dynamics (Homeostasis vs. Dysregulation)
- Simulation loop with time-series data
- ASCII visualization of system state
- Modeling hidden variables (Integrity)

```bash
gr samples/06-projects/dysregulation/main.gr
```

### `elevator/` ⭐⭐⭐
**Elevator simulation**

Topics:
- Object-oriented graph patterns
- State machines
- Simulation logic

```bash
gr samples/06-projects/elevator/sim_demo.gr
```

### `web_server/` ⭐⭐⭐
**HTTP web server**

Topics:
- HTTP server with routes and handlers
- HTML, JSON, and plain text responses
- Request parsing and response building
- Pure Graphoid server on top of TCP primitives

```bash
gr samples/06-projects/web_server/simple.gr
# Then visit http://localhost:8080/ or use curl
```

---

## Running Examples

### Basic Execution
```bash
# From rust/ directory
gr samples/01-basics/hello_world.gr
```

### With Stdlib (if needed)
```bash
# Set stdlib path
gr samples/05-stdlib/random.gr
```

### Release Mode (faster)
If running from source without installing:
```bash
gr samples/02-intermediate/behaviors.gr
```
If installed, `gr` already runs at release speed.

### Run Multiple Examples
```bash
# Test all basics
for f in samples/01-basics/*.gr; do
    echo "Running $f..."
    gr "$f"
done
```

---

## Contributing Examples

Want to add a new example? Great! Follow these guidelines:

### Good Examples Should:

✅ **Be self-contained** - Run without external dependencies when possible
✅ **Have clear comments** - Explain concepts, not just code
✅ **Show expected output** - Users should know what to expect
✅ **Focus on one concept** - Don't try to teach everything at once
✅ **Use realistic names** - `user`, `total`, not `x`, `foo`
✅ **Include practical use cases** - Show real-world applications

### Examples Should NOT:

❌ Be feature tests (those belong in `tests/`)
❌ Require deep Graphoid knowledge (for basics/intermediate)
❌ Have cryptic variable names
❌ Mix multiple unrelated concepts
❌ Depend on incomplete features

### Example Template

```graphoid
# example_name.gr - Brief one-line description
#
# This example demonstrates [main concept].
# Topics covered:
# - Feature 1
# - Feature 2
# - Feature 3

print("=== Example Title ===")
print()

# Section 1: Basic Usage
print("--- Basic Usage ---")
# Clear example with explanation
value = some_function()
print("Result:", value)

# Section 2: Advanced Usage
print("--- Advanced Usage ---")
# More complex example
# ...

print()
print("=== Summary ===")
print("Key takeaway: [main point]")
```

### Adding Your Example

1. Determine appropriate directory:
   - Basics: Core language features for beginners
   - Intermediate: Unique Graphoid features
   - Advanced: Graph algorithms and complex queries
   - Modules: Multi-file organization
   - Stdlib: Standard library demonstrations

2. Add file to directory
3. Update this README with description
4. Test that it runs: `gr samples/XX-category/your_example.gr`
5. Ensure it produces clear, educational output

---

## Troubleshooting

### "Module not found" Error

If you see module import errors:
```bash
# Make sure you're in the rust/ directory
# From project root

# Check file paths in import statements
# Relative imports should use "./" prefix
import "./module_name"
```

### "Undefined variable" in Stdlib Examples

Some examples require the stdlib:
```bash
gr samples/05-stdlib/random.gr
```

### Example Runs but Output is Unexpected

Check if the example uses newer features. The language is in alpha, so some examples may need updates as features evolve.

---

## Example Statistics

- **Total Examples:** 35 files
- **Basics:** 4 files (~20 minutes)
- **Intermediate:** 13 files (~2-3 hours)
- **Advanced:** 8 files (~1-2 hours)
- **Modules:** 6 files (~1 hour)
- **Stdlib:** 4 files (~1 hour)

**Quality Distribution:**
- ⭐⭐⭐ Excellent: 13 files (43%)
- ⭐⭐ Good: 12 files (40%)
- ⭐ Adequate: 5 files (17%)

---

## See Also

- [Language Specification](../../dev_docs/LANGUAGE_SPECIFICATION.md) - Complete language reference
- [User Guide](../../docs/user-guide/01-getting-started.md) - Step-by-step tutorials
- [API Reference](../../docs/api-reference/README.md) - Full API documentation
- [Design Philosophy](../../docs/DESIGN_PHILOSOPHY.md) - Why Graphoid works this way
- [Why Graphoid](../../docs/WHY_GRAPHOID.md) - Overview for new users

---

## Feedback

Found an issue with an example? Have a suggestion? Open an issue on GitHub or contribute a fix!

**Happy coding with Graphoid!** 🎉

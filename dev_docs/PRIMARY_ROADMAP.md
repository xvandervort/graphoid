# Glang Primary Development Roadmap
*Last Updated: January 2025*

## Mission Statement
Transform Glang from a practical programming language into a revolutionary platform for graph-based, self-aware computational systems.

## Current State (September 2025)
- **Version**: Pre-1.0 (Development Phase)
- **Test Coverage**: 67% (1239 tests passing)
- **Core Features**: Complete function system, type inference, collections, file loading
- **Architecture**: Container-based (lists, hashes, data nodes)
- **🚨 CRITICAL ARCHITECTURE ISSUE**: **Functions use variable lookup instead of graph traversal**
  - **Glang is not truly graph-based** - it's simulating graph features
  - **Foundational bug** prevents basic module function composition
  - **Highest priority fix required** before any other development
- **Production Readiness**: BLOCKED until graph foundation is implemented
- **🎯 Self-Hosting Progress**: 80% reduction in Python dependency achieved!
  - Network/HTML processing now 80% pure Glang (major breakthrough)
  - Only network I/O and complex DOM parsing still require Python
  - Demonstrates Glang's capability for real-world applications

## Development Phases

### ✅ Phase 0: Foundation (COMPLETED)
- AST-based execution system
- Type system with inference
- Functions and lambdas
- Basic collections (lists, hashes)
- File loading system
- REPL environment
- 70% test coverage

### 🔥 FOUNDATIONAL PRIORITY #1: True Graph Architecture (IMMEDIATE)
**Goal**: Transform Glang from simulated graph language to TRUE graph language

**CRITICAL INSIGHT**: Without graph-based function discovery, Glang isn't Glang - it's just pretending to be graph-based.

#### 🚨 FOUNDATIONAL ARCHITECTURE BUG (BLOCKS EVERYTHING)
**Current Problem**: Functions use variable-based lookup instead of graph traversal
- ❌ Functions stored as "variables" in flat dictionaries
- ❌ Function calls do `get_variable(name)` lookups
- ❌ No graph structure for function discovery
- ❌ **Glang is theoretically broken** - not actually graph-based

**Required Fix**: **Graph-based function discovery system**
- ✅ Functions as graph nodes with connectivity
- ✅ Function calls traverse graph edges
- ✅ Module functions connected through graph structure
- ✅ AST as temporary subgraph that merges into permanent call graph

**Implementation Priority**: **ABSOLUTE HIGHEST** - No other features until this is fixed
**Timeline**: 2 weeks (Phase 1: Foundation, Phase 2: AST Integration)
**Impact**: Transforms Glang from fake graph language to revolutionary true graph language

See: [`dev_docs/FOUNDATIONAL_PRIORITY_CALL_GRAPH.md`](./FOUNDATIONAL_PRIORITY_CALL_GRAPH.md)

---

### 📍 Phase 1: Production Readiness (Q1-Q2 2025) - BLOCKED UNTIL GRAPH FOUNDATION
**Goal**: Make Glang practical for real-world applications (AFTER graph foundation is complete)

#### ✅ 1.0 CRITICAL PARSER FIXES (COMPLETED September 2025)
**Status**: All blocking issues from cryptocurrency analytics experiment have been resolved

- [x] **Logical Operator Precedence**: Parser correctly handles `a && b && c` and `a == 1 or b == 2`
  - Implemented: `parse_logical_or` → `parse_logical_and` → `parse_comparison` precedence levels
  - Expressions like `a == 1 or b == 2 and c == 3` now parse with correct operator precedence
- [x] **Hash Variable Key Access**: `hash[variable_key]` syntax fully functional
  - Dynamic key access now works: `config[key_name]` where `key_name` is a variable
  - Enables flexible hash operations required for real-world applications
- [x] **Variable Scoping**: Proper lexical scoping implemented
  - Variables can be reused in different scopes without conflicts
  - For-loops and other block scopes properly isolate their variables

**Validation**: Cryptocurrency analytics experiments now run without workarounds

#### 1.1 Data Analytics & Visualization Support
**Status**: Foundational capabilities needed for real-world data processing

- [ ] **CSV Module**: Native CSV parsing and generation
  - `csv.read(filename)` → structured data
  - `csv.write(data, filename)` → file output
  - Support for headers, type inference, custom delimiters
- [ ] **Statistics Module**: Essential statistical functions
  - Descriptive statistics: `mean()`, `median()`, `std_dev()`, `variance()`
  - Data aggregation: `sum()`, `count()`, `group_by()`, `aggregate()`
  - Time series: `rolling_average()`, `trend_analysis()`
- [ ] **Data Structures**: Higher-level data organization
  - `DataFrame` type for structured data with column access
  - Filtering and transformation operations
  - Join and merge capabilities
- [ ] **Visualization Library**: Essential for data analysis
  - Basic charts: line plots, bar charts, histograms, scatter plots
  - Export formats: PNG, SVG, ASCII art for terminal
  - Integration with data structures for easy plotting
  - Example: `data.plot("line", x="date", y="price").save("chart.png")`

**Experiment**: Retry cryptocurrency analytics with new capabilities

#### 1.2 Standard Library Completion
- [x] Math module with constants
- [x] JSON encoding/decoding
- [x] Complete I/O operations (file, network, console)
- [x] String manipulation utilities
- [x] Date/time handling (with precision integration and data node consistency)
- [x] Regular expressions (comprehensive pattern matching and text processing)
- [x] Random number generation (secure, deterministic, statistical distributions)
- [x] **Network Library**: HTTP client for web requests and API calls (✅ COMPLETE - 80% Pure Glang)
- [x] **HTML Parsing Library**: Web scraping and HTML processing (✅ COMPLETE - 80% Pure Glang)

#### 🎯 1.2.1 MAJOR ARCHITECTURAL BREAKTHROUGH: Pure Glang Implementation (September 2025)
**Achievement**: Successfully shifted from Python-heavy to Glang-native processing

**Network Library Analysis**:
- ✅ **URL Parsing**: 100% Pure Glang (protocol, host, path extraction)
- ✅ **URL Encoding/Decoding**: 100% Pure Glang (comprehensive character support)
- ✅ **Domain Extraction**: 100% Pure Glang (www. removal, port handling)
- ✅ **URL Validation**: 100% Pure Glang (format checking)
- ✅ **Query String Processing**: 100% Pure Glang (build/parse parameters)
- ⚠️ **HTTP Requests**: Python (actual network I/O only)

**HTML Library Analysis**:
- ✅ **HTML Entity Encoding/Decoding**: 100% Pure Glang (&amp;, &lt;, etc.)
- ✅ **Tag Stripping**: 100% Pure Glang (clean text extraction)
- ✅ **URL Extraction**: 100% Pure Glang (href/src attribute parsing)
- ✅ **Meta Tag Parsing**: 100% Pure Glang (name/content extraction)
- ✅ **Table Data Extraction**: 100% Pure Glang (tr/td/th processing)
- ✅ **Email Detection**: 100% Pure Glang (regex-based extraction)
- ⚠️ **Complex DOM Operations**: Python (parse tree construction only)

**Key Insight**: Most "heavy lifting" in web processing is actually string manipulation - which Glang excels at!

**Before vs After**:
- **Before**: Python handled parsing, validation, encoding, DOM manipulation
- **After**: Python only handles network I/O and complex DOM parsing
- **Result**: 80% reduction in Python dependency, demonstrating Glang's self-hosting potential

**Impact**: Bitcoin tracker and web scraping applications now run almost entirely in native Glang!

**Next Steps**: Continue eliminating Python dependencies:
- [ ] Native HTTP client implementation in Glang
- [ ] Pure Glang DOM parsing for simple HTML structures
- [ ] File I/O operations in native Glang

#### 1.3 Developer Experience
- [x] **Enhanced error messages with stack traces** (complements error-as-data pattern)
- [ ] Debugger support
- [ ] IDE integration (VS Code extension)
- [x] Package manager design (see PACKAGING_SYSTEM_DESIGN.md)
- [ ] Package manager implementation (glang-package command)
- [ ] Documentation generator

#### 1.4 Core Language Features

##### 🚨 1.4.1 CRITICAL: String Parsing Enhancements (IMMEDIATE PRIORITY)
**Status**: Essential for full Glang self-hosting capability
**Discovered**: During HTML module conversion - major limitations prevent pure Glang implementations

- [ ] **Quote Character Handling**: Fix lexer to properly handle `"\""` escaped quotes in string literals
  - **Current Issue**: `"\""` becomes empty string instead of quote character
  - **Impact**: Cannot properly decode HTML entities like `&quot;` or parse quoted attributes
  - **Blocker for**: Full Glang HTML/XML/JSON parsing, configuration file processing
- [ ] **Essential String Methods**: Add missing string manipulation primitives
  - `string.char_at(index)` - Get character at specific position
  - `string.index_of(substring)` - Find position of substring (-1 if not found)
  - `string.substring(start, end)` - Extract substring by position range
  - `string.last_index_of(substring)` - Find last occurrence of substring
- [ ] **Pattern Matching Support**: Enable complex text processing
  - Basic regex support OR enhanced string patterns
  - Pattern-based replacement operations
  - Multi-character delimiter splitting
- [ ] **Unicode Support**: Proper handling of international characters
  - UTF-8 string operations
  - Character classification (letter, digit, whitespace)
  - Case conversion for international text

**Validation Test**: Reimplement HTML DOM parsing in 100% pure Glang after these fixes

##### 1.4.2 Other Core Features
- [ ] **Pattern Matching**: `match` expressions for elegant control flow and data destructuring
- [ ] **Status Symbols**: Limited symbols (`:ok`, `:error`) for result patterns
- [ ] **Error-as-Data**: Result lists `[:ok, value]` / `[:error, message]` for clean error handling
- [ ] **Implicit Success Wrapping**: Auto-wrap plain returns as `[:ok, value]`
- [ ] **Module Scoping**: Functions can access module-level variables
- [x] **None Literal**: Add `none` as a language keyword for null values (completed)
- [ ] **Symbol Lexing/Parsing**: Support `:symbol` syntax for behavior names and status codes

#### 1.5 Enhanced Behavior System
- [x] **Intrinsic Behaviors**: Behaviors attached directly to data structures (completed)
- [x] **Scoped Behavior Configuration**: File/function/block-level behavior settings (✅ COMPLETE)
  - See [SCOPED_BEHAVIOR_CONFIGURATION.md](./SCOPED_BEHAVIOR_CONFIGURATION.md) for design
  - ✅ Phase 1: Configuration syntax parsing and AST integration
  - ✅ Phase 2: Configuration context and behavior enforcement (September 2025)
    - Configuration stack management with proper scoping
    - `skip_none`/`skip_nil` enforcement in list operations (sum, min, max)
    - `decimal_places` enforcement in arithmetic operations
    - Nested configuration override and restoration
    - Full test coverage with 11 configuration behavior tests
- [ ] **Custom Value Mappings**: User-defined conversions (`"red" → 7`, `"kg" → "mass"`)
- [ ] **Function-Based Behaviors**: Attach user-written functions as behaviors
- [ ] **Conditional Behaviors**: Apply behaviors based on context or conditions
- [ ] **Behavior Inheritance**: Child containers inherit parent behaviors
- [ ] **History Tracking**: Audit trail of all value transformations (before/after)
- [ ] **Derived Column Operations**: Calculate new values from neighboring data
- [ ] **Pattern-Based Transformations**: Rules that trigger on data patterns
- [ ] **Domain-Specific Behavior Libraries**: Pre-built behavior sets for common domains

#### 1.6 Performance & Stability
- [ ] Performance benchmarking suite
- [ ] Memory leak detection
- [ ] Optimization pass on hot paths
- [ ] Achieve 85% test coverage

**Deliverables**: v0.9 release with standard library and modern error handling

### 🎯 Phase 2: Graph Foundation (Q3-Q4 2025)
**Goal**: Transform containers into true graph structures

> **See**: [ABSTRACTION_LAYER_ROADMAP.md](./ABSTRACTION_LAYER_ROADMAP.md) for detailed design

#### 2.1 Behavior-Oriented Test Framework
**Priority**: Essential for quality assurance and ecosystem growth
> **See**: [GLANG_TEST_FRAMEWORK_PLAN.md](./GLANG_TEST_FRAMEWORK_PLAN.md) for detailed design

- [ ] **Test Isolation**: `let` syntax with lazy evaluation and memoization
- [ ] **Factory System**: Object creation with traits, sequences, and realistic data
- [ ] **Shared Examples**: DRY testing patterns for common behaviors
- [ ] **RSpec-Style Testing**: Natural language test descriptions (`describe`, `context`, `it`)
- [ ] **Expectation System**: Chainable matchers (`expect(value).to_equal(42)`)
- [ ] **CLI Test Runner**: `glang test` command with discovery and reporting

**Benefits**: Quality assurance, executable documentation, ecosystem enablement

#### 2.2 Core Graph Features
- Edge implementation with metadata
- Node awareness (knows container and siblings)
- Graph traversal algorithms
- Path finding and connectivity analysis

**Deliverables**: v1.0 release with graph primitives and test framework

### 🔮 Phase 3: Self-Aware Systems (2026)
**Goal**: Enable self-understanding and self-modification

> **See**: [ABSTRACTION_LAYER_ROADMAP.md](./ABSTRACTION_LAYER_ROADMAP.md#phase-2-self-aware-data-structures-q3-2025)

#### Key Features
- Reflection API for structure introspection
- Method-data unification
- Controlled self-mutation with governance
- Evolution patterns and genetic algorithms

**Deliverables**: v1.5 release with self-aware features

### 🌐 Phase 4: Distributed Computing (2027)
**Goal**: Multi-machine graph systems

> **See**: [ABSTRACTION_LAYER_ROADMAP.md](./ABSTRACTION_LAYER_ROADMAP.md#phase-4-distributed-graph-systems-2026)

#### Key Features
- Distributed graph runtime
- Consensus mechanisms
- Network transparency
- Fault tolerance

**Deliverables**: v2.0 release with distributed capabilities

## Technical Debt & Improvements

### High Priority
1. **Module System Completion**: Finish implementing all planned standard library modules
2. **Error Handling**: Improve error messages and add stack traces
3. **Performance**: Profile and optimize critical paths

### Medium Priority  
1. **Documentation**: Complete language reference and tutorials
2. **Tooling**: Build debugger and profiler
3. **Platform Support**: Ensure Windows/Mac/Linux compatibility

### Low Priority (Philosophical Purity)
1. **Custom Number System**: Replace Python float/int with GlangNumber
2. **String Implementation**: Native string handling without Python strings
3. **Pure Glang Bootstrap**: Rewrite interpreter in Glang itself

## Success Criteria

### For v1.0 Release
- [ ] Can build a web service in pure Glang
- [ ] Performance within 10x of Python for common tasks
- [ ] Zero segfaults/crashes in production use
- [ ] Comprehensive standard library
- [ ] Active community of 100+ users

### For v2.0 Release  
- [ ] Self-modifying AI agents running in Glang
- [ ] Distributed applications with <100ms latency
- [ ] Academic papers published on graph computing model
- [ ] Industry adoption for specific use cases

## Resource Planning

### Current Team
- Core development (1 person)
- Community contributors (as available)

### Future Needs
- Graph theory expert (Phase 2)
- Distributed systems engineer (Phase 4)
- Technical writer for documentation
- Community manager

## Risk Management

### Technical Risks
- **Performance overhead** from graph abstraction → Mitigate with C extensions
- **Complexity explosion** in API → User testing and iterative design
- **Distributed consensus bugs** → Formal verification of protocols

### Market Risks
- **Limited adoption** → Focus on unique use cases (AI, simulation)
- **Competition** from established languages → Emphasize unique graph features

## Related Documentation

### Design Documents
- [ABSTRACTION_LAYER_ROADMAP.md](./ABSTRACTION_LAYER_ROADMAP.md) - Detailed system abstraction plans
- [CLAUDE.md](../CLAUDE.md) - Development guidelines and project overview

### Historical Documents (Archived)
The following documents have been superseded by this roadmap:
- ARCHITECTURAL_ISSUES.md → Incorporated into Phase 1-2
- FUTURE_ENHANCEMENTS.md → Merged into phase planning
- LONG_TERM_ARCHITECTURAL_PLAN.md → Consolidated here
- SESSION_HANDOFF.md → No longer needed
- CLAUDE_SESSION_NOTES.md → Historical reference only

### Reference Documents (Keep)
- [README.md](../README.md) - User-facing project description
- [MODULE_SYSTEM_LESSONS.md](./MODULE_SYSTEM_LESSONS.md) - Lessons learned from module implementation
- [OPERATOR_SEPARATION_SUMMARY.md](./OPERATOR_SEPARATION_SUMMARY.md) - Design decision documentation
- [GLANG_LANGUAGE_COMPARISON.md](./GLANG_LANGUAGE_COMPARISON.md) - Language feature comparison

## Next Actions

### Immediate (This Month)
1. ✅ ~~Complete I/O module implementation~~
2. ✅ ~~Add string manipulation functions~~
3. ✅ ~~Write v0.9 release notes~~
4. ✅ ~~Update README with roadmap reference~~
5. ✅ ~~Add date/time handling module~~
6. ✅ ~~Implement regular expression module~~
7. ✅ ~~Create random number generation module~~
8. ✅ ~~Add intrinsic behavior system for automatic value transformation~~
9. ✅ ~~Improve error handling with enhanced stack traces~~
10. Implement package manager (glang-package command)

### Q1 2025
1. Finish standard library modules
2. Build performance benchmarking suite  
3. Implement basic packaging system (glang-package commands)
4. Create VS Code extension prototype
5. Begin Phase 2 design review

---

## 📦 Not Yet Scheduled

*Features and requests that are acknowledged but not yet assigned to specific phases*

### Network & System Interface
- **TCP Socket Support** - Create, bind, listen, accept network connections
- **HTTP Server Framework** - Built-in web server capabilities (experiment shows application logic works perfectly)
- **Concurrency Primitives** - Threads, async/await, or event loops for handling multiple connections
- **System Integration** - Signal handling (SIGINT/SIGTERM), process management
- **Network I/O** - Non-blocking socket read/write operations
- **Security Layer** - HTTPS/TLS support, input validation, rate limiting
- **Logging System** - Structured logging with levels and output control

### Infrastructure & Tooling
- Database drivers (PostgreSQL, SQLite, MySQL, MongoDB)
- HTTP client libraries (complement server framework)
- Testing framework beyond basic assertions
- Code formatting tool (glfmt)
- Linting and static analysis tools
- REPL improvements (syntax highlighting, auto-completion)
- Language server protocol (LSP) implementation

### Platform & Deployment  
- Cross-platform compilation targets
- WebAssembly compilation support
- Mobile platform support (iOS/Android)
- Container/Docker integration
- Cloud deployment tools
- Binary distribution system

### Advanced Language Features
- Generics/parametric types
- Async/await and concurrent programming
- Macro system or metaprogramming
- Foreign function interface (FFI) for C libraries
- Memory management optimization
- JIT compilation for performance

### Web Development Findings (January 2025)

**Experiment Result**: A complete web server was successfully implemented in Glang (`samples/working_web_server.gr`), demonstrating:

**✅ What Works Perfectly:**
- HTTP request parsing and response generation
- JSON API endpoints with full encoding/decoding
- Function-based routing systems
- Error handling with proper HTTP status codes
- String templating for HTML generation
- Hash/map operations for data structures
- All core language features needed for web applications

**❌ Missing for Production Web Servers:**
- TCP socket creation and network binding
- Concurrency for handling multiple clients
- System signal handling and process management
- Non-blocking I/O operations

**Key Insight**: Glang's language design is excellent for web application logic. Missing pieces are primarily in the system interface layer, not the language itself. This validates the current Phase 1 focus on completing standard library features before adding system programming capabilities.

### Ecosystem & Community
- Package registry and hosting
- Documentation hosting (docs.glang.dev)
- Community forums and support channels
- Tutorial and learning materials
- Conference presentations and outreach
- Commercial support and consulting

### Advanced Behavior System (Future Enhancements)

The behavior system will evolve to support increasingly sophisticated transformations and domain-specific logic:

#### Custom Value Mappings
```glang
# User-defined conversion tables
colors = []
colors.add_mapping("red", 7)      # Direct value mapping
colors.add_mapping("blue", 12)
colors.add_mapping("green", 5)

colors.append("red")              # Becomes 7
colors.append("blue")             # Becomes 12

# Unit conversions
weights = []
weights.add_mapping("kg", func(v) { return v * 2.204 })  # kg to lbs
weights.add_mapping("g", func(v) { return v / 453.592 }) # grams to lbs

weights.append({ "value": 70, "unit": "kg" })  # Becomes 154.28 lbs
```

#### Function-Based Behaviors
```glang
# Attach user functions as behaviors
func normalize_temperature(temp) {
    if temp < -273.15 { return -273.15 }  # Absolute zero
    if temp > 1000 { return 1000 }       # Reasonable max
    return temp
}

temperatures = []
temperatures.add_rule(normalize_temperature)  # Use function directly

# Or with parameters
func validate_range_func(value, min, max) {
    if value < min { return min }
    if value > max { return max }
    return value
}

scores.add_rule(validate_range_func, 0, 100)
```

#### Conditional Behaviors
```glang
# Apply behaviors based on context
medical_data = []
medical_data.add_rule_if("nil_to_zero", "sensor_data")    # Only for sensor readings
medical_data.add_rule_if("uppercase", "patient_name")     # Only for names
medical_data.add_rule_unless("positive", "temperature")   # Except temperatures

# Context-aware processing
patient_data.set_context("emergency", true)
patient_data.append(reading)  # Different behaviors apply in emergency context
```

#### Behavior Inheritance
```glang
# Parent behaviors automatically inherited by children
hospital_system = {}
hospital_system.add_rule("nil_to_zero")
hospital_system.add_rule("validate_medical_ranges")

# Child inherits parent behaviors
patient_records = hospital_system.create_child()
patient_records.append(nil)      # Becomes 0 (inherited behavior)

# Override or extend inherited behaviors
patient_records.add_rule("encrypt_sensitive_data")  # Additional behavior
```

#### Domain-Specific Behavior Libraries
```glang
# Pre-built behavior sets for common domains
import "behaviors/medical" as MedicalBehaviors
import "behaviors/financial" as FinancialBehaviors
import "behaviors/scientific" as ScientificBehaviors

# Apply entire behavior suites
lab_results = []
lab_results.apply_behaviors(MedicalBehaviors.lab_standards)  # Multiple behaviors at once

financial_data = []
financial_data.apply_behaviors(FinancialBehaviors.currency_processing)
```

#### History Tracking and Transformation Audit
```glang
# Enable history tracking to see before/after transformations
patient_data = []
patient_data.enable_history()              # Track all transformations
patient_data.add_rule("nil_to_zero")
patient_data.add_rule("validate_range", 95, 105)

patient_data.append(nil)                   # Value: 0
patient_data.append(110)                   # Value: 105

# Access transformation history
history = patient_data.get_history(0)     # First element's transformation chain
print(history)  # [
                #   { original: nil, rule: "nil_to_zero", result: 0 },
                # ]

history = patient_data.get_history(1)     # Second element's history
print(history)  # [
                #   { original: 110, rule: "validate_range", result: 105, params: [95, 105] }
                # ]

# Query transformations
transformed_items = patient_data.find_transformed()           # All items that were changed
nil_conversions = patient_data.find_by_rule("nil_to_zero")   # Items affected by specific rule
original_values = patient_data.get_original_values()         # Pre-transformation values
```

#### Derived Column Operations (Advanced Graph Features)
```glang
# Create derived values based on neighboring data and patterns
data_table = [
    { "name": "Alice", "height": 165, "weight": 60 },
    { "name": "Bob", "height": 180, "weight": 75 },
    { "name": "Charlie", "height": 175, "weight": 70 }
]

# Add derived column behavior based on neighboring values
data_table.add_derived_rule("bmi", func(row) {
    # Calculate BMI from height and weight in same row
    height_m = row["height"] / 100
    return row["weight"] / (height_m * height_m)
})

# Results in:
# { "name": "Alice", "height": 165, "weight": 60, "bmi": 22.04 }
# { "name": "Bob", "height": 180, "weight": 75, "bmi": 23.15 }

# Advanced: Cross-row calculations
sensor_readings = [
    { "time": 1, "temp": 20.5, "humidity": 45 },
    { "time": 2, "temp": 21.0, "humidity": 47 },
    { "time": 3, "temp": 20.8, "humidity": 46 }
]

# Add derived column that depends on previous row
sensor_readings.add_derived_rule("temp_change", func(row, context) {
    if context.previous_row {
        return row["temp"] - context.previous_row["temp"]
    }
    return 0  # First row has no change
})

# Pattern-based derived columns
financial_data.add_derived_rule("trend", func(row, context) {
    # If price increased AND volume > 1000, mark as "bullish"
    if row["price_change"] > 0 && context.neighbor("volume") > 1000 {
        return "bullish"
    } else if row["price_change"] < 0 && context.neighbor("volume") > 1000 {
        return "bearish"
    }
    return "neutral"
})

# Conditional derived columns based on patterns
medical_data.add_derived_rule_when("risk_score",
    condition: func(row) { return row["age"] > 65 },
    calculation: func(row) {
        # Complex risk calculation only for seniors
        return calculate_senior_risk(row["bp"], row["cholesterol"], row["bmi"])
    }
)
```

#### Graph-Aware Transformations
```glang
# Future: True graph operations where behaviors understand relationships
social_network = create_graph()
social_network.add_derived_rule("influence_score", func(person, graph) {
    # Calculate influence based on network connections
    followers = graph.get_connections(person, "follows")
    return followers.count() * avg_engagement_rate(followers)
})

# Propagation behaviors across graph edges
social_network.add_propagation_rule("trending_topic", func(topic, connections) {
    # If topic appears in X% of connected nodes, propagate to all
    if topic_frequency(connections) > 0.3 {
        return propagate_to_all(topic, connections)
    }
})
```

### Specialized Libraries
- Machine learning and data science bindings
- Graphics and game development libraries
- Cryptography and security libraries
- Audio/video processing
- Scientific computing modules
- GUI toolkit (native desktop applications)

---

**Note**: This is the authoritative roadmap for Glang development. All other planning documents should reference this document or be considered historical artifacts.
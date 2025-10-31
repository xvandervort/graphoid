# Phase 9: Advanced Features - Detailed Implementation Plan

**Version**: 1.0
**Created**: October 2025
**Duration**: 18-25 days
**Dependencies**: Phases 0-8 complete

---

## Overview

Phase 9 implements the advanced runtime features that enable production-quality modules and applications:

1. **Configuration System** - Scoped settings for error handling, type coercion, etc.
2. **Precision Context Blocks** - Decimal place control for numeric operations
3. **Error Handling** - Try/catch/finally with configurable modes
4. **Freeze System** - Immutability for collections with fine-grained control
5. **Freeze Control Rules** - Behavior rules for freeze operations

**Why This Phase Comes First**: Stdlib modules (Phases 10-11) need these features to provide production-quality error handling, configuration options, and data protection.

---

## Architecture Overview

### 1. Configuration Stack

The runtime maintains a **configuration context stack** that tracks active settings:

```rust
// src/execution/config.rs
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Config {
    // Error handling
    pub error_mode: ErrorMode,
    pub bounds_checking: BoundsCheckingMode,
    pub type_coercion: TypeCoercionMode,
    pub none_handling: NoneHandlingMode,

    // Numeric precision
    pub decimal_places: Option<usize>,  // None = no rounding

    // Type system
    pub strict_types: bool,

    // Graph validation
    pub edge_validation: bool,
    pub strict_edge_rules: bool,

    // None conversions
    pub none_conversions: bool,

    // Skip none values in operations
    pub skip_none: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorMode {
    Strict,   // Errors stop execution unless caught
    Lenient,  // Use safe defaults (none, skip, etc.)
    Collect,  // Collect errors, continue execution
}

#[derive(Debug, Clone, PartialEq)]
pub enum BoundsCheckingMode {
    Strict,   // Out of bounds access raises error
    Lenient,  // Out of bounds returns none
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeCoercionMode {
    Strict,   // Type mismatches raise errors
    Lenient,  // Attempt conversions, return none on failure
}

#[derive(Debug, Clone, PartialEq)]
pub enum NoneHandlingMode {
    Propagate,  // none values pass through operations
    Skip,       // Skip none values in operations
    Error,      // Treat none as an error
}

impl Default for Config {
    fn default() -> Self {
        Config {
            error_mode: ErrorMode::Strict,
            bounds_checking: BoundsCheckingMode::Strict,
            type_coercion: TypeCoercionMode::Strict,
            none_handling: NoneHandlingMode::Propagate,
            decimal_places: None,
            strict_types: true,
            edge_validation: true,
            strict_edge_rules: true,
            none_conversions: true,
            skip_none: false,
        }
    }
}

pub struct ConfigStack {
    stack: Vec<Config>,
}

impl ConfigStack {
    pub fn new() -> Self {
        ConfigStack {
            stack: vec![Config::default()],
        }
    }

    pub fn current(&self) -> &Config {
        self.stack.last().unwrap()
    }

    pub fn current_mut(&mut self) -> &mut Config {
        self.stack.last_mut().unwrap()
    }

    pub fn push(&mut self, config: Config) {
        self.stack.push(config);
    }

    pub fn pop(&mut self) -> Option<Config> {
        if self.stack.len() > 1 {
            self.stack.pop()
        } else {
            None  // Never pop the base config
        }
    }

    pub fn push_with_changes(&mut self, changes: HashMap<String, Value>) -> Result<()> {
        let mut new_config = self.current().clone();

        // Apply changes to new_config
        for (key, value) in changes {
            match key.as_str() {
                "error_mode" => {
                    new_config.error_mode = parse_error_mode(value)?;
                }
                "bounds_checking" => {
                    new_config.bounds_checking = parse_bounds_checking_mode(value)?;
                }
                "decimal_places" => {
                    new_config.decimal_places = Some(value.to_number()? as usize);
                }
                "skip_none" => {
                    new_config.skip_none = value.is_truthy();
                }
                _ => {
                    return Err(GraphoidError::ConfigError {
                        message: format!("Unknown configuration key: {}", key),
                    });
                }
            }
        }

        self.push(new_config);
        Ok(())
    }
}
```

### 2. Executor Integration

```rust
// src/execution/executor.rs (additions)
pub struct Executor {
    // Existing fields...
    pub environment: Environment,
    pub module_manager: ModuleManager,
    pub current_file: Option<PathBuf>,

    // New fields for Phase 9
    pub config_stack: ConfigStack,
    pub error_collector: ErrorCollector,  // For :collect mode
    pub precision_stack: Vec<Option<usize>>,  // For precision blocks
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            environment: Environment::new(),
            module_manager: ModuleManager::new(),
            current_file: None,
            config_stack: ConfigStack::new(),
            error_collector: ErrorCollector::new(),
            precision_stack: vec![],
        }
    }

    pub fn current_config(&self) -> &Config {
        self.config_stack.current()
    }

    pub fn push_config(&mut self, changes: HashMap<String, Value>) -> Result<()> {
        self.config_stack.push_with_changes(changes)
    }

    pub fn pop_config(&mut self) {
        self.config_stack.pop();
    }

    pub fn current_precision(&self) -> Option<usize> {
        self.precision_stack.last().copied().flatten()
    }

    pub fn push_precision(&mut self, places: Option<usize>) {
        self.precision_stack.push(places);
    }

    pub fn pop_precision(&mut self) {
        self.precision_stack.pop();
    }
}
```

### 3. Error Collector for :collect Mode

```rust
// src/execution/error_collector.rs
use crate::error::{GraphoidError, SourcePosition};

#[derive(Debug, Clone)]
pub struct CollectedError {
    pub error: GraphoidError,
    pub file: Option<String>,
    pub position: SourcePosition,
}

pub struct ErrorCollector {
    errors: Vec<CollectedError>,
}

impl ErrorCollector {
    pub fn new() -> Self {
        ErrorCollector {
            errors: Vec::new(),
        }
    }

    pub fn collect(&mut self, error: GraphoidError, file: Option<String>, position: SourcePosition) {
        self.errors.push(CollectedError { error, file, position });
    }

    pub fn get_errors(&self) -> &[CollectedError] {
        &self.errors
    }

    pub fn clear(&mut self) {
        self.errors.clear();
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}
```

### 4. Freeze System

```rust
// src/values/freeze.rs

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FreezeState {
    Mutable,           // Not frozen at all
    ShallowFrozen,     // Collection structure frozen, elements mutable
    DeepFrozen,        // Collection and all nested elements frozen
}

// Add to Value enum variants:
// Each collection type (List, Hash, Graph, Tree) gets a freeze_state field

// src/values/list.rs (additions)
pub struct List {
    pub graph: Graph,
    length: usize,
    pub freeze_state: FreezeState,  // NEW
}

impl List {
    // Freeze methods
    pub fn freeze(&self, shallow: bool) -> Result<Value> {
        let mut new_list = self.clone();

        if shallow {
            new_list.freeze_state = FreezeState::ShallowFrozen;
        } else {
            new_list.freeze_state = FreezeState::DeepFrozen;

            // Deep freeze: freeze all elements
            for i in 0..new_list.length {
                let node_id = format!("node_{}", i);
                let value = new_list.graph.get_node_value(&node_id)?;

                if let Some(frozen_value) = self.freeze_value(value, false)? {
                    new_list.graph.set_node_value(&node_id, frozen_value)?;
                }
            }
        }

        Ok(Value::List(new_list))
    }

    pub fn freeze_in_place(&mut self, shallow: bool) -> Result<()> {
        if shallow {
            self.freeze_state = FreezeState::ShallowFrozen;
        } else {
            self.freeze_state = FreezeState::DeepFrozen;

            // Deep freeze: freeze all elements
            for i in 0..self.length {
                let node_id = format!("node_{}", i);
                let value = self.graph.get_node_value(&node_id)?;

                if let Some(frozen_value) = self.freeze_value(value, false)? {
                    self.graph.set_node_value(&node_id, frozen_value)?;
                }
            }
        }

        Ok(())
    }

    pub fn is_frozen(&self) -> bool {
        self.freeze_state != FreezeState::Mutable
    }

    pub fn has_frozen(&self, verbose: bool) -> Result<Value> {
        if verbose {
            // Return detailed hash
            let mut info = Hash::new();
            let (has_frozen, count, collections, primitives) = self.count_frozen_elements()?;

            info.insert("has_frozen", Value::Bool(has_frozen))?;
            info.insert("frozen_count", Value::Number(count as f64))?;
            info.insert("frozen_collections", Value::Number(collections as f64))?;
            info.insert("frozen_primitives", Value::Number(primitives as f64))?;

            Ok(Value::Hash(info))
        } else {
            // Return simple boolean
            Ok(Value::Bool(self.contains_frozen_elements()?))
        }
    }

    fn freeze_value(&self, value: Value, shallow: bool) -> Result<Option<Value>> {
        match value {
            Value::List(ref list) => {
                let mut new_list = list.clone();
                if shallow {
                    new_list.freeze_state = FreezeState::ShallowFrozen;
                } else {
                    new_list.freeze_state = FreezeState::DeepFrozen;
                    // Recursively freeze elements...
                }
                Ok(Some(Value::List(new_list)))
            }
            Value::Hash(ref hash) => {
                // Similar logic
                Ok(Some(/* frozen hash */))
            }
            // Primitives don't need freezing at the value level
            _ => Ok(None)
        }
    }
}
```

---

## Implementation Tasks

### Milestone 1: Configuration System (4-5 days)

#### Task 1.1: Configuration Types and Stack
**File**: `src/execution/config.rs`

- [ ] Define `Config` struct with all settings
- [ ] Define mode enums (`ErrorMode`, `BoundsCheckingMode`, etc.)
- [ ] Implement `ConfigStack` with push/pop operations
- [ ] Implement `push_with_changes()` for partial config updates
- [ ] Add parsing functions for mode values from symbols
- [ ] Add validation for configuration values

**Tests**: 15+ tests
- Stack push/pop operations
- Default configuration values
- Partial config updates
- Nested config blocks
- Invalid config keys/values
- Config cloning and inheritance

#### Task 1.2: AST Node for Configure Blocks
**File**: `src/ast/mod.rs`

```rust
Stmt::Configure {
    settings: HashMap<String, Expr>,  // Key-value pairs
    body: Option<Vec<Stmt>>,          // Optional block body
    position: SourcePosition,
}
```

- [ ] Add `Configure` statement variant
- [ ] Add `Precision` statement variant (similar structure)

#### Task 1.3: Parser Support
**File**: `src/parser/mod.rs`

- [ ] Parse `configure { key: value, ... }` statements
- [ ] Parse `configure { ... } { statements }` blocks
- [ ] Parse `precision N { statements }` blocks
- [ ] Handle nested blocks correctly
- [ ] Validate syntax (only symbols/numbers for values)

**Tests**: 10+ tests
- File-level configure statements
- Block-level configure statements
- Nested configure blocks
- Precision blocks
- Syntax errors (invalid keys, missing braces, etc.)

#### Task 1.4: Executor Implementation
**File**: `src/execution/executor.rs`

- [ ] Add `config_stack` field to Executor
- [ ] Implement `execute_configure()` method
- [ ] Push config before executing block
- [ ] Pop config after block completes
- [ ] Handle errors during config setup
- [ ] Apply config settings to operations

**Tests**: 20+ tests
- Basic configure block execution
- Nested configuration
- Config restoration after errors
- File-level vs block-level configs
- All configuration options working

### Milestone 2: Error Handling System (5-6 days)

#### Task 2.1: Try/Catch AST Nodes
**File**: `src/ast/mod.rs`

```rust
Stmt::Try {
    body: Vec<Stmt>,
    catch_clauses: Vec<CatchClause>,
    finally_block: Option<Vec<Stmt>>,
    position: SourcePosition,
}

struct CatchClause {
    error_type: Option<String>,  // None = catch all
    variable: Option<String>,    // None = no binding
    body: Vec<Stmt>,
    position: SourcePosition,
}
```

- [ ] Add Try/Catch statement variants
- [ ] Add `raise` expression for re-throwing

#### Task 2.2: Error Types
**File**: `src/error.rs`

- [ ] Add error type hierarchy
- [ ] Add `JSONParseError`, `TypeError`, `RuntimeError`, etc.
- [ ] Add error introspection methods (`.message()`, `.type()`, etc.)
- [ ] Make errors cloneable for collect mode

#### Task 2.3: Error Collector
**File**: `src/execution/error_collector.rs`

- [ ] Implement `ErrorCollector` struct (see architecture above)
- [ ] Add to Executor
- [ ] Implement `get_errors()` built-in function
- [ ] Implement `clear_errors()` built-in function

#### Task 2.4: Parser Support
**File**: `src/parser/mod.rs`

- [ ] Parse `try { } catch Type as var { } finally { }`
- [ ] Handle multiple catch clauses
- [ ] Handle optional error type and variable
- [ ] Validate syntax

**Tests**: 12+ tests
- Basic try/catch
- Multiple catch clauses
- Catch with and without binding
- Finally block execution
- Try with only finally (no catch)
- Nested try blocks

#### Task 2.5: Executor Implementation
**File**: `src/execution/executor.rs`

- [ ] Implement `execute_try_catch()` method
- [ ] Match error types to catch clauses
- [ ] Bind error to variable in catch scope
- [ ] Always execute finally block
- [ ] Handle error collection mode
- [ ] Apply error modes to operations

**Tests**: 35+ tests
- Basic error catching
- Error type matching
- Error variable binding
- Finally block execution (with and without errors)
- Nested try blocks
- Error re-throwing
- Error collection mode (:collect)
- Error modes affecting list access, type conversions, etc.

### Milestone 3: Precision Context Blocks (2-3 days)

#### Task 3.1: Precision Stack
**File**: `src/execution/executor.rs`

- [ ] Add `precision_stack` field
- [ ] Implement push/pop for precision contexts
- [ ] Integrate with numeric operations

#### Task 3.2: Parser Support
**File**: `src/parser/mod.rs`

- [ ] Parse `precision N { statements }`
- [ ] Parse `precision :int { statements }` (alias for 0)
- [ ] Validate N is integer >= 0

**Tests**: 8+ tests
- Basic precision blocks
- Integer mode (precision 0)
- Nested precision blocks
- Precision restoration

#### Task 3.3: Numeric Operation Updates
**Files**: Various execution files

- [ ] Check `executor.current_precision()` before returning numbers
- [ ] Apply rounding based on precision
- [ ] Handle precision in arithmetic operations
- [ ] Handle precision in string-to-number conversions

**Tests**: 20+ tests
- Arithmetic with precision
- Nested precision contexts
- Integer mode (no decimal point)
- Precision with scientific notation
- String conversions with precision
- Division with precision

### Milestone 4: Freeze System (5-7 days)

#### Task 4.1: Freeze State Infrastructure
**File**: `src/values/freeze.rs`

- [ ] Define `FreezeState` enum
- [ ] Add freeze_state field to all collections
- [ ] Implement freeze state checks before mutations

#### Task 4.2: Freeze Methods for Collections
**Files**: `src/values/list.rs`, `src/values/hash.rs`, `src/values/tree.rs`, `src/values/graph_value.rs`

Each collection needs:
- [ ] `.freeze()` method (returns new frozen copy)
- [ ] `.freeze(shallow: true)` option
- [ ] `.freeze!()` mutation method (freezes in place)
- [ ] `.is_frozen()` query method
- [ ] `.has_frozen()` query method
- [ ] `.has_frozen(:verbose)` detailed query

**Implementation per collection** (~1 day each):
- List (Day 1)
- Hash (Day 2)
- Tree (Day 3)
- Graph (Day 4)

#### Task 4.3: Deep vs Shallow Freeze
**File**: `src/values/freeze.rs`

- [ ] Implement recursive deep freeze
- [ ] Implement shallow freeze (collection only)
- [ ] Add helper to freeze individual values
- [ ] Track freeze state correctly

#### Task 4.4: Freeze Enforcement
**Files**: Collection mutation methods

- [ ] Check freeze state before `append()`, `insert()`, `remove()`, etc.
- [ ] Return appropriate error if frozen
- [ ] Block index assignment on frozen collections
- [ ] Prevent modification of elements in frozen collection

#### Task 4.5: Freeze Queries
**Files**: Collection implementation files

- [ ] Implement `is_frozen()` - returns boolean
- [ ] Implement `has_frozen()` - returns boolean
- [ ] Implement `has_frozen(:verbose)` - returns detailed hash
- [ ] Count frozen elements recursively

**Tests**: 60+ tests (15 per collection type)
- Basic freeze/freeze!
- Deep freeze vs shallow freeze
- Freeze query methods
- Attempting to modify frozen collections
- Mixed frozen and unfrozen elements
- Nested frozen structures
- Freeze state inheritance

### Milestone 5: Freeze Control Rules (2-3 days)

#### Task 5.1: Freeze Rule Types
**File**: `src/graph/rules.rs`

Add to `RuleSpec` enum:
```rust
pub enum RuleSpec {
    // ... existing rules ...

    // Freeze control rules
    NoFrozen,              // Reject frozen elements
    CopyElements,          // Copy elements on insert
    ShallowFreezeOnly,     // Prevent deep freeze
}
```

#### Task 5.2: Rule Implementation
**File**: `src/graph/rule_validation.rs`

- [ ] Implement `:no_frozen` rule check
- [ ] Implement `:copy_elements` behavior
- [ ] Implement `:shallow_freeze_only` constraint

#### Task 5.3: Integration with Collections
**Files**: Collection mutation methods

- [ ] Check `:no_frozen` before adding frozen elements
- [ ] Apply `:copy_elements` during insertion
- [ ] Apply `:shallow_freeze_only` during freeze operations

**Tests**: 25+ tests
- `:no_frozen` rule enforcement
- `:copy_elements` behavior
- `:shallow_freeze_only` constraint
- Interaction with transformation rules
- Error messages for rule violations

---

## Testing Strategy

### Unit Tests (150+ total)

**Configuration System** (~35 tests):
- Config stack operations
- Config parsing and validation
- Config application to operations
- Nested configuration blocks

**Error Handling** (~50 tests):
- Try/catch/finally execution
- Error type matching
- Error collection mode
- Error modes (strict/lenient/collect)
- Error introspection

**Precision Blocks** (~25 tests):
- Precision context push/pop
- Numeric rounding
- Integer mode
- Nested precision

**Freeze System** (~65 tests):
- Freeze/freeze! for all collections
- Deep vs shallow freeze
- Freeze queries
- Freeze enforcement
- Freeze control rules

### Integration Tests (25+ tests)

**File**: `tests/integration/phase9_tests.rs`

- [ ] Complex nested configuration scenarios
- [ ] Error handling across module boundaries
- [ ] Precision in financial calculations
- [ ] Freeze with transformation rules
- [ ] Configure blocks in .gr files
- [ ] Error collection in batch operations

### REPL Tests (15+ tests)

Verify all features work correctly in REPL:
- [ ] Multi-line configure blocks
- [ ] Try/catch with REPL state
- [ ] Precision blocks
- [ ] Freeze operations
- [ ] Error messages in REPL

---

## Success Criteria

### Must Have ✅

1. **Configuration System**
   - ✅ File-level and block-level configuration
   - ✅ All documented config options working
   - ✅ Nested configs with proper scoping
   - ✅ Config restoration after errors

2. **Error Handling**
   - ✅ Try/catch/finally syntax working
   - ✅ Error type matching
   - ✅ All three error modes (strict/lenient/collect)
   - ✅ Error introspection methods
   - ✅ get_errors() and clear_errors() functions

3. **Precision Blocks**
   - ✅ Decimal place control working
   - ✅ Integer mode (precision 0)
   - ✅ Nested precision contexts
   - ✅ Applied to all arithmetic operations

4. **Freeze System**
   - ✅ freeze() and freeze!() on all collections
   - ✅ Deep and shallow freeze
   - ✅ Freeze query methods (is_frozen, has_frozen)
   - ✅ Mutation prevention on frozen collections
   - ✅ All freeze control rules working

5. **Testing**
   - ✅ 150+ unit tests passing
   - ✅ 25+ integration tests passing
   - ✅ 15+ REPL tests passing
   - ✅ Zero compiler warnings

### Nice to Have (Defer if Needed)

- Performance optimization for freeze checks
- Freeze visualization in debugger (Phase 13)
- Advanced error recovery strategies
- Custom error types from user code

---

## Timeline Breakdown

### Week 1 (Days 1-5): Configuration & Error Infrastructure
- Days 1-2: Configuration system
- Days 3-5: Error handling (try/catch, error types)

### Week 2 (Days 6-10): Error Modes & Precision
- Days 6-8: Error modes and collection integration
- Days 9-10: Precision context blocks

### Week 3 (Days 11-17): Freeze System
- Days 11-12: Freeze infrastructure
- Days 13-14: List and Hash freeze
- Days 15-16: Tree and Graph freeze
- Day 17: Freeze queries and integration

### Week 4 (Days 18-20): Freeze Rules & Polish
- Days 18-19: Freeze control rules
- Day 20: Integration testing and bug fixes

### Buffer (Days 21-25): Testing & Documentation
- Days 21-23: Comprehensive testing
- Days 24-25: Documentation and examples

---

## Dependencies & Risks

### Dependencies
- **Phase 8 complete** - Module system working
- **Parser extensible** - Can add new statement types
- **Executor ready** - Can handle context stacks

### Risks & Mitigations

**Risk**: Configuration system becomes too complex
**Mitigation**: Start with essential configs only, add more gradually

**Risk**: Error collection mode affects performance
**Mitigation**: Only active when explicitly enabled

**Risk**: Freeze system has edge cases
**Mitigation**: Comprehensive test coverage, start with List/Hash

**Risk**: Integration with existing features breaks things
**Mitigation**: Thorough regression testing after each milestone

---

## Next Steps

After Phase 9 completion, we'll have all the infrastructure needed for:
- **Phase 10**: Pure Graphoid stdlib (can use configs and error handling)
- **Phase 11**: Native stdlib modules (can use error modes)
- **Phase 12**: Testing framework (can test error scenarios)

**To begin Phase 9**: Start with Milestone 1 (Configuration System) and implement Task 1.1.

---

## References

- Language Specification: `/home/irv/work/grang/dev_docs/LANGUAGE_SPECIFICATION.md`
  - Lines 968-1010: Precision Context Blocks
  - Lines 1012-1049: Configuration Blocks
  - Lines 1966-2165: Freeze System
  - Lines 2777-2999: Error Handling

- Current Implementation:
  - `src/execution/executor.rs` - Executor structure
  - `src/values/` - Collection implementations
  - `src/graph/rules.rs` - Rule system

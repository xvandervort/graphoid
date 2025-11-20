# Updated Phase 13 Roadmap

## Overview

Phase 13 completes core language features and prepares for public release. The original "Bitwise Operators & Integer Types" scope is obsolete (features already implemented). New scope reflects actual needs.

---

## Phase 13: :32bit Directive Implementation

**Duration**: 5-7 days
**Priority**: HIGH
**Status**: 🔲 Pending

### Scope

Implement `:32bit` directive for clean 32-bit wrapping arithmetic (needed for crypto, binary protocols, embedded systems).

### Deliverables

1. **Parser changes** - Recognize `:32bit` in configure blocks
2. **AST additions** - BitWidth enum (Bits32, Bits64)
3. **Environment tracking** - Track bit width state
4. **Executor wrapping** - Apply 32-bit wrapping to all arithmetic/bitwise ops
5. **Configure block support** - Save/restore bit width state
6. **Test suite** - 15+ tests for wrapping behavior

### Success Criteria

```graphoid
configure { :unsigned, :32bit } {
    x = 0xffffffff
    y = 1
    result = x + y  # Should be 0 (wrapped)
}
```

### Performance Impact

- SHA-256: 2.4s → ~1.8s (~25% faster)
- Eliminates ~900 manual masking operations per crypto block

### Technical Spec

See `dev_docs/CRYPTO_PRODUCTION_PROPOSAL.md` Part 1 for detailed implementation.

---

## Phase 13.1: Documentation & Publishing Preparation

**Duration**: 10-14 days
**Priority**: CRITICAL (blocks public release)
**Status**: 🔲 Pending

### Goal

Prepare Graphoid for first public release with complete, professional documentation.

### Scope

#### 1. User Guide / Tutorial (4-5 days)

**Target audience**: Programmers new to Graphoid

**Contents**:
```
docs/
  user-guide/
    01-getting-started.md       - Installation, first program, REPL
    02-basics.md                - Variables, types, operators
    03-control-flow.md          - if, while, for, match
    04-functions.md             - Function definition, lambdas, closures
    05-collections.md           - Lists, hashes, trees, graphs
    06-graph-operations.md      - Graph manipulation, traversal, rules
    07-modules.md               - Import, export, organizing code
    08-directives.md            - :integer, :unsigned, :32bit, :high
    09-standard-library.md      - Overview of stdlib modules
    10-best-practices.md        - Idiomatic Graphoid, common patterns
```

**Style**:
- Short chapters (5-10 minutes each)
- Code examples for every concept
- Exercises at end of each chapter
- Progressive difficulty

**Format**: Markdown, renderable to HTML/PDF

#### 2. API Reference (3-4 days)

**Target audience**: Graphoid developers needing function lookup

**Contents**:
```
docs/
  api-reference/
    core/
      num.md              - Number operations, methods
      string.md           - String operations, methods
      list.md             - List operations, methods
      hash.md             - Hash operations, methods
      tree.md             - Tree operations, methods
      graph.md            - Graph operations, methods
    
    stdlib/
      io.md               - I/O functions
      math.md             - Mathematical functions
      string-ops.md       - String manipulation
      time.md             - Time/date operations
      regex.md            - Regular expressions
      random.md           - Random number generation
      crypto.md           - Cryptography (native module)
      constants.md        - Mathematical constants
    
    directives.md         - Configure blocks, numeric modes
    operators.md          - All operators, precedence
```

**Format**:
- Consistent structure for each function
  ```markdown
  ## function_name(arg1, arg2)
  
  **Description**: What it does
  
  **Parameters**:
  - `arg1` (type): Description
  - `arg2` (type): Description
  
  **Returns**: Return type and description
  
  **Examples**:
  ```graphoid
  # Example code
  ```
  
  **See also**: Related functions
  ```

#### 3. Examples Collection (2-3 days)

**Target audience**: Learn by example

**Structure**:
```
examples/
  01-hello-world/
    hello.gr
    README.md
  
  02-basics/
    variables.gr
    functions.gr
    loops.gr
    README.md
  
  03-collections/
    list-operations.gr
    hash-tables.gr
    tree-traversal.gr
    README.md
  
  04-graph-algorithms/
    dijkstra.gr
    bfs-dfs.gr
    topological-sort.gr
    README.md
  
  05-real-world/
    file-processing.gr
    json-parser.gr
    web-scraper.gr
    data-analysis.gr
    README.md
  
  06-crypto/ (using native module)
    hashing.gr
    hmac-signing.gr
    encryption.gr
    digital-signatures.gr
    README.md
  
  07-advanced/
    pattern-matching.gr
    graph-rules.gr
    behaviors.gr
    README.md
```

**Requirements**:
- Every example must run without errors
- Every example has comments explaining what it does
- README.md in each directory explains the category
- Range from beginner to advanced
- Cover all major language features

#### 4. Code Cleanup (1-2 days)

**Tasks**:
- Remove all debug print statements
- Ensure zero compiler warnings
- Consistent code formatting
- Remove commented-out code
- Update all TODOs/FIXMEs
- Verify all tests pass

**Verification**:
```bash
cargo clippy -- -D warnings  # No warnings allowed
cargo test                   # All tests pass
cargo fmt --check            # Formatted correctly
```

#### 5. Roadmap Sanitization (1 day)

**Tasks**:
- Remove internal notes/context
- Remove time estimates that are obsolete
- Focus on "what's done" and "what's coming"
- Remove references to private discussions
- Create public-facing `ROADMAP.md`

**Public roadmap should include**:
- Current status (what works now)
- Near-term plans (next 2-3 months)
- Long-term vision (future)
- How to contribute

#### 6. Repository Preparation (1 day)

**Tasks**:
- LICENSE file (choose license)
- CONTRIBUTING.md (how to contribute)
- CODE_OF_CONDUCT.md
- Clean up .gitignore
- Update README.md for public audience
- Remove any sensitive information
- Verify CI/CD works (if using)

### Success Criteria

- [ ] User guide complete and reviewed
- [ ] API reference covers all stdlib functions
- [ ] 30+ working example programs
- [ ] Zero compiler warnings
- [ ] All tests pass
- [ ] Public roadmap created
- [ ] Repository ready for open source

### Deliverables Checklist

```
docs/
  ✓ user-guide/ (10 chapters)
  ✓ api-reference/ (complete)
  ✓ README.md (updated)

examples/
  ✓ 30+ example programs
  ✓ All categories covered
  ✓ All examples run successfully

Repository root:
  ✓ LICENSE
  ✓ CONTRIBUTING.md
  ✓ CODE_OF_CONDUCT.md
  ✓ ROADMAP.md (public version)
  ✓ README.md (public-facing)

Code:
  ✓ Zero warnings
  ✓ All tests pass
  ✓ Formatted consistently
```

---

## Phase 13.5: Bytecode Compilation (Future)

**Duration**: 14-21 days
**Priority**: HIGH (post-release)
**Status**: 🔲 Future

### Scope

Implement bytecode compilation for 10-100x performance improvement. This is POST-release work.

---

## Phase 13.6: Optimization Framework (Future)

**Duration**: 7-10 days
**Priority**: MEDIUM (post-release)
**Status**: 🔲 Future

### Scope

Function inlining and other optimizations. Requires bytecode (13.5) first.

---

## Updated Roadmap Structure

```
Phase 13:   :32bit Directive                    (5-7 days)   🔲 Pending
Phase 13.1: Documentation & Publishing Prep     (10-14 days) 🔲 Pending
            └─ PUBLISH GRAPHOID v0.1           
Phase 13.5: Bytecode Compilation                (14-21 days) 🔲 Future
Phase 13.6: Optimization Framework              (7-10 days)  🔲 Future
Phase 14:   Stdlib Translation to Pure Graphoid (7-10 days)  🔲 Pending
Phase 15:   Testing Framework (RSpec-style)     (7-10 days)  🔲 Pending
Phase 16:   Debugger                            (10-14 days) 🔲 Pending
Phase 17:   Package Manager                     (14-21 days) 🔲 Pending
```

## Timeline to Publication

**Phase 13**: 5-7 days
**Phase 13.1**: 10-14 days
**Total**: 15-21 days (~3-4 weeks)

After Phase 13.1 completes: **Graphoid is ready for public release!**

Performance improvements (13.5, 13.6) come after initial release.

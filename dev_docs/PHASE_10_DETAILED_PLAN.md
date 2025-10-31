# Phase 10: Complete Module System - Detailed Implementation Plan

**Duration**: 3-4 days (reduced from 4-6 days - already 40% complete)
**Status**: Partially implemented (31 tests passing)
**Goal**: Complete multi-file project support with robust module system

---

## Overview

The Module System enables code organization across multiple files, reusable libraries, and clean namespace management. It's essential for building real-world Graphoid applications.

**Current Status**:
- ✅ Module manager exists (`src/execution/module_manager.rs`, 250 lines)
- ✅ 31 module tests passing (~40% complete)
- ✅ Basic import/load functionality works
- ⏳ Missing: Module declaration syntax (`module name`)
- ⏳ Missing: Module aliases
- ⏳ Missing: Standard library modules
- ⏳ Missing: Project structure support (graphoid.toml)
- ⏳ Missing: Module loading from .gr files
- ⏳ Missing: Namespace management and scoping

**Dependencies**:
- Needs to be complete before stdlib work (Phases 11-12)
- Final prerequisite for production standard library

**Why This Phase Comes After Phase 9**:
- Pattern matching and behaviors complete first
- Module system is needed to organize stdlib code
- Clean separation of concerns

---

## Architecture Summary

**Files Involved**:
- `src/execution/module_manager.rs` - Module loading and resolution
- `src/ast/mod.rs` - AST nodes for module/import statements
- `src/parser/mod.rs` - Parsing module/import syntax
- `src/execution/executor.rs` - Module execution context
- `src/values/mod.rs` - Module value type

**Module Types**:
1. **File Modules** - Individual .gr files
2. **Standard Library** - Built-in modules (json, http, etc.)
3. **External Packages** - Dependencies (future - Phase 14)

**Resolution Order**:
1. Relative paths (`./`, `../`)
2. Project modules (`src/`, `lib/`)
3. Standard library
4. External packages

---

## Day 1: Module Declaration Syntax

### Goal
Support `module name` and `alias` declarations in .gr files.

### Tasks

#### 1.1 AST Nodes
**File**: `src/ast/mod.rs`

Add module declaration statement:
```rust
pub enum Stmt {
    // ... existing ...
    ModuleDeclaration {
        name: String,
        alias: Option<String>,
        position: SourcePosition,
    },
    // ... existing ...
}
```

#### 1.2 Parser Support
**File**: `src/parser/mod.rs`

Parse this syntax:
```graphoid
module my_utilities
alias utils

# Module contents below
helper_value = 42
fn process(data) { return data * 2 }
```

**Implementation**:
```rust
fn parse_module_declaration(&mut self) -> Result<Stmt> {
    // Expect: module <name>
    let name = self.expect_identifier()?;

    // Check for optional alias
    let alias = if self.check_keyword("alias") {
        self.advance();
        Some(self.expect_identifier()?)
    } else {
        None
    };

    Ok(Stmt::ModuleDeclaration { name, alias, position })
}
```

#### 1.3 Executor Integration
**File**: `src/execution/executor.rs`

When executing a module file:
1. Create module namespace
2. Register alias if provided
3. Execute module contents in module scope
4. Return Module value

**Tests**: 8 tests
- Basic module declaration
- Module with alias
- Module without alias
- Multiple modules (error - only one allowed)
- Module in wrong location (must be at top)
- Invalid module names
- Duplicate aliases
- Module exports

**Acceptance Criteria**:
- ✅ `module name` syntax works
- ✅ `alias name` syntax works
- ✅ Module creates proper namespace
- ✅ 8+ tests passing

---

## Day 2: Import Syntax Enhancements

### Goal
Complete import syntax with all variations from spec.

### Tasks

#### 2.1 Import Variations
**Current**: Basic `import "module"` works

**Add Support For**:
```graphoid
# Import from stdlib
import "json"
import "http"

# Import .gr file
import "path/to/file.gr"

# Import with custom alias (discouraged for stdlib)
import "module" as custom_name

# Relative imports
import "./helpers.gr"
import "../config.gr"

# Project modules
import "models/user"
import "app/server"
```

#### 2.2 Module Resolution Enhanced
**File**: `src/execution/module_manager.rs`

Implement resolution priority:
1. **Relative paths**: Start with `./` or `../`
2. **Project modules**: Check `src/`, then `lib/`
3. **Standard library**: Built-in modules
4. **External packages**: From dependencies (Phase 14)

**Implementation**:
```rust
impl ModuleManager {
    pub fn resolve_module(&mut self, path: &str, current_file: Option<&Path>)
        -> Result<PathBuf, GraphoidError> {

        // 1. Check if relative path
        if path.starts_with("./") || path.starts_with("../") {
            return self.resolve_relative(path, current_file);
        }

        // 2. Check project directories
        if let Some(project_path) = self.find_in_project(path) {
            return Ok(project_path);
        }

        // 3. Check standard library
        if let Some(stdlib_path) = self.find_in_stdlib(path) {
            return Ok(stdlib_path);
        }

        // 4. Not found
        Err(GraphoidError::runtime(format!("Module not found: {}", path)))
    }
}
```

#### 2.3 Module Caching
Prevent duplicate loading:
```rust
pub struct ModuleManager {
    // Cache: canonical_path -> Module value
    loaded_modules: HashMap<PathBuf, Value>,
}
```

**Tests**: 12 tests
- Relative import (./)
- Relative import (../)
- Project module import
- Stdlib import
- Custom alias
- Module caching (same module twice)
- Circular import detection
- Import non-existent module (error)
- Import syntax errors
- Case sensitivity
- Path traversal security
- Module re-exports

**Acceptance Criteria**:
- ✅ All import variations work
- ✅ Resolution priority correct
- ✅ Caching prevents duplicates
- ✅ 12+ tests passing

---

## Day 3: Standard Library Modules

### Goal
Implement core standard library modules.

### Tasks

#### 3.1 Module Registry
**File**: `src/execution/stdlib_registry.rs` (new file)

```rust
pub struct StdlibRegistry {
    modules: HashMap<String, Box<dyn StdlibModule>>,
}

pub trait StdlibModule {
    fn name(&self) -> &str;
    fn aliases(&self) -> Vec<&str>;
    fn initialize(&self) -> Environment;
}
```

#### 3.2 Core Modules
Implement these standard library modules:

**1. JSON Module** (`stdlib/json`)
```graphoid
import "json"

data = {"name": "Alice", "age": 30}
json_string = json.encode(data)
parsed = json.decode(json_string)
```

Functions:
- `encode(value)` -> string
- `decode(string)` -> value
- `pretty(value)` -> string (formatted JSON)

**2. IO Module** (`stdlib/io`)
```graphoid
import "io"

content = io.read_file("data.txt")
io.write_file("output.txt", content)
io.print("message")
```

Functions:
- `read_file(path)` -> string
- `write_file(path, content)` -> none
- `print(value)` -> none
- `println(value)` -> none
- `read_line()` -> string

**3. Math Module** (`stdlib/math`)
```graphoid
import "math"

value = math.sqrt(16)  # 4
angle = math.sin(math.PI / 2)  # 1
```

Functions:
- `sqrt(x)`, `pow(x, y)`, `abs(x)`
- `sin(x)`, `cos(x)`, `tan(x)`
- `floor(x)`, `ceil(x)`, `round(x)`
- Constants: `PI`, `E`

**4. String Module** (`stdlib/string`)
```graphoid
import "string"

padded = string.pad_left("hello", 10)
repeated = string.repeat("x", 5)
```

Functions:
- `pad_left(str, width)`, `pad_right(str, width)`
- `repeat(str, count)`
- `join(list, delimiter)`
- `lines(str)` - split by newlines

**5. List Module** (`stdlib/list`)
```graphoid
import "list"

flattened = list.flatten([[1, 2], [3, 4]])
zipped = list.zip([1, 2], [3, 4])
```

Functions:
- `flatten(nested_list)`
- `zip(list1, list2, ...)`
- `range(start, end, step?)`
- `repeat(value, count)`

#### 3.3 Module Aliases
**File**: `src/execution/stdlib_registry.rs`

Register built-in aliases:
- `statistics` → `stats`
- `random` → `rand`
- `regex` → `re`
- `constants` → `const`

**Implementation**:
Both names automatically available:
```graphoid
import "statistics"
# Both work:
stats.mean([1, 2, 3])
statistics.mean([1, 2, 3])
```

**Tests**: 25 tests (5 per module × 5 modules)
- JSON encode/decode
- IO read/write
- Math functions
- String utilities
- List operations
- Module aliases work
- Import names correct
- Error handling

**Acceptance Criteria**:
- ✅ 5 core stdlib modules implemented
- ✅ All functions work correctly
- ✅ Aliases automatically available
- ✅ 25+ tests passing

---

## Day 4: Load vs Import

### Goal
Clarify and complete `load` vs `import` semantics.

### Tasks

#### 4.1 Load Implementation
**File**: `src/execution/executor.rs`

```graphoid
# load merges into current namespace
load "config.gr"

# Variables from config.gr now available directly
if debug {
    print("Debug mode")
}
```

**vs**

```graphoid
# import creates module namespace
import "config"

# Access via module name
if config.debug {
    print("Debug mode")
}
```

#### 4.2 Load Execution
**Implementation**:
```rust
fn eval_load_stmt(&mut self, path: &str) -> Result<()> {
    // 1. Resolve path
    let file_path = self.module_manager.resolve_module(path, self.current_file.as_deref())?;

    // 2. Parse and execute in CURRENT environment
    let content = fs::read_to_string(&file_path)?;
    let program = self.parse(content)?;

    // 3. Execute statements (modifies current env)
    for stmt in program.statements {
        self.eval_stmt(&stmt)?;
    }

    Ok(())
}
```

**Tests**: 8 tests
- Load merges variables
- Load can access current scope
- Load can modify current scope
- Import does NOT merge
- Load relative path
- Load absolute path
- Load non-existent (error)
- Load cyclic (error)

**Acceptance Criteria**:
- ✅ `load` merges into current namespace
- ✅ `import` creates separate namespace
- ✅ Semantics clear and documented
- ✅ 8+ tests passing

---

## Day 5: Project Structure Support

### Goal
Support multi-file projects with proper structure.

### Tasks

#### 5.1 Project Detection
**File**: `src/execution/module_manager.rs`

```rust
impl ModuleManager {
    pub fn detect_project_root(&self, current_file: &Path) -> Option<PathBuf> {
        // Walk up directory tree looking for graphoid.toml
        let mut path = current_file.to_path_buf();
        while path.pop() {
            let toml_path = path.join("graphoid.toml");
            if toml_path.exists() {
                return Some(path);
            }
        }
        None
    }
}
```

#### 5.2 Project Module Resolution
When project root detected:
```
my_project/
├── graphoid.toml
├── src/
│   ├── main.gr
│   ├── app/
│   │   └── server.gr
│   └── models/
│       └── user.gr
└── lib/
    └── utils/
        └── helpers.gr
```

Resolution for `import "app/server"`:
1. Check `src/app/server.gr` ✓
2. Check `src/app/server/mod.gr`
3. Check `lib/app/server.gr`
4. Not found → error

#### 5.3 Module Index Files
Support `mod.gr` for directory modules:
```
models/
├── mod.gr          # Module entry point
├── user.gr
└── product.gr
```

In `models/mod.gr`:
```graphoid
module models

# Re-export submodules
import "./user"
import "./product"
```

**Tests**: 10 tests
- Project root detection
- Import from src/
- Import from lib/
- Directory module with mod.gr
- Multi-level imports
- Relative imports in project
- No project root (fallback)
- Invalid project structure
- Module not in project
- Cross-directory imports

**Acceptance Criteria**:
- ✅ Project root detection works
- ✅ src/ and lib/ directories searched
- ✅ mod.gr files work
- ✅ 10+ tests passing

---

## Day 6: graphoid.toml Support

### Goal
Basic manifest file support for project metadata.

### Tasks

#### 6.1 TOML Parsing
**Dependencies**: Add `toml` crate to `Cargo.toml`

```toml
[dependencies]
toml = "0.8"
serde = { version = "1.0", features = ["derive"] }
```

#### 6.2 Manifest Structure
**File**: `src/project/manifest.rs` (new file)

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GraphoidManifest {
    pub project: ProjectInfo,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    #[serde(default)]
    pub dev_dependencies: HashMap<String, String>,
    #[serde(default)]
    pub build: BuildConfig,
    #[serde(default)]
    pub test: TestConfig,
}

#[derive(Debug, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub graphoid_version: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct BuildConfig {
    pub entry_point: Option<String>,
    pub output_dir: Option<String>,
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct TestConfig {
    pub test_pattern: Option<String>,
    pub coverage_threshold: Option<u8>,
}
```

#### 6.3 Manifest Loading
**File**: `src/execution/module_manager.rs`

```rust
impl ModuleManager {
    pub fn load_manifest(&mut self, project_root: &Path) -> Result<GraphoidManifest> {
        let toml_path = project_root.join("graphoid.toml");
        let content = fs::read_to_string(toml_path)?;
        let manifest: GraphoidManifest = toml::from_str(&content)?;
        Ok(manifest)
    }
}
```

#### 6.4 Manifest Usage
- Project name/version in error messages
- Entry point for `graphoid run`
- Test pattern for `graphoid test`
- Include/exclude for module resolution

**Tests**: 8 tests
- Parse valid manifest
- Parse minimal manifest
- Invalid TOML (error)
- Missing required fields (error)
- Use entry_point
- Use test_pattern
- Include/exclude patterns
- Manifest in subdirectory

**Acceptance Criteria**:
- ✅ graphoid.toml parsing works
- ✅ Manifest data accessible
- ✅ Used in module resolution
- ✅ 8+ tests passing

**Note**: Full dependency resolution deferred to Phase 14 (Package Manager)

---

## Module Value Type

### Module Representation
**File**: `src/values/mod.rs`

```rust
#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub namespace: Environment,
    pub file_path: PathBuf,
    pub aliases: Vec<String>,
}

pub enum Value {
    // ... existing ...
    Module(Rc<Module>),
    // ... existing ...
}
```

### Module Methods
```graphoid
# Get module info
name = my_module.name()
path = my_module.path()
exports = my_module.exports()  # List of exported names
```

**Tests**: 5 tests

---

## Error Messages

### Improved Module Errors
**File**: `src/error.rs`

```rust
pub enum GraphoidError {
    // ... existing ...
    ModuleNotFound {
        module: String,
        searched_paths: Vec<PathBuf>,
    },
    CircularImport {
        chain: Vec<String>,
    },
    ModuleParseError {
        module: String,
        error: Box<GraphoidError>,
    },
}
```

**Implementation**: Clear error messages
```
Error: Module not found: 'app/server'

Searched paths:
  1. src/app/server.gr
  2. src/app/server/mod.gr
  3. lib/app/server.gr
  4. Standard library

Suggestion: Check the module name and ensure the file exists.
```

**Tests**: 6 tests for error messages

---

## Documentation

### Files to Update

1. **Language Specification** (`dev_docs/LANGUAGE_SPECIFICATION.md`)
   - Verify module examples
   - Add graphoid.toml spec
   - Document resolution order

2. **Project Guide** (new: `docs/MULTI_FILE_PROJECTS.md`)
   - Standard project layout
   - Best practices
   - Module organization patterns
   - graphoid.toml reference

3. **Module Tutorial** (new: `docs/MODULE_SYSTEM.md`)
   - Step-by-step guide
   - Common patterns
   - Import vs load
   - Stdlib reference

---

## Integration Tests

**File**: `tests/module_integration_tests.rs`

Comprehensive scenarios:
1. Multi-file application
2. Standard library usage
3. Nested module imports
4. Circular import detection
5. Module re-exports
6. load vs import
7. Relative imports in subdirectories
8. Project structure with src/ and lib/
9. graphoid.toml integration
10. Error handling and recovery

**Tests**: 20+ integration tests

---

## Complete Phase 8 Acceptance Criteria

**Module Declaration**:
- ✅ `module name` syntax works
- ✅ `alias` declaration works
- ✅ 8+ tests passing

**Import System**:
- ✅ All import variations work
- ✅ Resolution priority correct
- ✅ Module caching works
- ✅ 12+ tests passing

**Standard Library**:
- ✅ 5 core modules implemented
- ✅ JSON, IO, Math, String, List modules work
- ✅ Aliases automatically available
- ✅ 25+ tests passing

**Load vs Import**:
- ✅ Distinct semantics clear
- ✅ Both work correctly
- ✅ 8+ tests passing

**Project Structure**:
- ✅ Project root detection works
- ✅ src/ and lib/ supported
- ✅ mod.gr files work
- ✅ 10+ tests passing

**Manifest**:
- ✅ graphoid.toml parsing works
- ✅ Metadata available
- ✅ 8+ tests passing

**Integration**:
- ✅ 20+ integration tests passing
- ✅ Real-world scenarios work

**Totals**:
- ✅ **91+ new tests** (current: 31, target: 122+)
- ✅ Zero compiler warnings
- ✅ Documentation complete
- ✅ All spec examples work

---

## Risk Assessment

**Low Risk**:
- Module declaration (straightforward)
- Load vs import (already partially working)

**Medium Risk**:
- Standard library modules (implementation work)
- Project structure (file system complexity)

**High Risk**:
- Module resolution edge cases
- Circular import detection
- Cross-platform path handling

**Mitigation**:
- Comprehensive path testing on all platforms
- Clear error messages for debugging
- Extensive integration tests

---

## Success Metrics

1. **Test Coverage**: 122+ module tests passing
2. **Stdlib Completeness**: 5 core modules working
3. **Real Projects**: Multi-file examples work
4. **Documentation**: Complete guides available
5. **Zero Warnings**: Clean compilation

---

## Next Phase Preview

**Phase 9** (Advanced Features) will build on modules by:
- Adding more stdlib modules
- Module-level configuration
- Advanced import patterns
- Performance optimizations

The module system enables real-world Graphoid applications with clean code organization!

---

## Quick Start for Next Session

```bash
# Run existing module tests
cargo test module

# Check what's already implemented
grep -r "module " src/
ls -la src/execution/module_manager.rs

# Start with Day 1: Module declaration
# File: src/ast/mod.rs
# Add: ModuleDeclaration statement
```

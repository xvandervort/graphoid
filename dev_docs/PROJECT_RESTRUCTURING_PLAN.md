# Project Restructuring Plan: Flatten rust/ Directory

**Date:** November 24, 2025
**Status:** In Progress
**Priority:** High - Fixes critical stdlib import issues

---

## Problem Statement

The current project structure has an unnecessary `rust/` subdirectory that causes confusion:

```
/home/irv/work/grang/
├── rust/                    # ❌ Unnecessary nesting
│   ├── Cargo.toml
│   ├── src/
│   ├── samples/
│   ├── tests/
│   └── stdlib/             # ❌ Old/obsolete (4 modules)
├── stdlib/                 # ✅ Main stdlib (13 modules, but missing math.gr)
├── python/                 # Reference implementation
└── docs/
```

**Issues:**
1. `rust/` nesting is non-standard for Rust projects
2. Two stdlib directories cause confusion
3. Stdlib imports require `GRAPHOID_STDLIB_PATH` environment variable
4. Default stdlib path `"stdlib"` is relative to `rust/`, not project root
5. Main stdlib missing `math.gr` and `string.gr`

---

## Target Structure

Standard Rust project layout at top level:

```
/home/irv/work/grang/
├── Cargo.toml              # Root of Rust project
├── src/                    # Rust source code
├── stdlib/                 # Standard library (complete)
├── samples/                # Example programs
│   ├── 01-basics/
│   ├── 02-intermediate/
│   ├── 03-advanced/
│   ├── 04-modules/
│   ├── 05-stdlib/
│   └── README.md
├── tests/                  # Integration tests
├── target/                 # Build artifacts (gitignored)
├── docs/                   # User documentation
├── dev_docs/               # Development documentation
├── python/                 # Python reference implementation
├── README.md
└── CLAUDE.md
```

---

## Benefits

1. ✅ Standard Rust project structure
2. ✅ Single stdlib location - no confusion
3. ✅ `import "math"` works automatically - no environment variable needed
4. ✅ Simpler paths in documentation
5. ✅ Easier for contributors to understand
6. ✅ Matches expectations from cargo/rustup

---

## Detailed Execution Plan

### Phase 1: Backup and Preparation (5 minutes)

**Safety first:**
```bash
# Create git commit of current state
cd /home/irv/work/grang
git add -A
git commit -m "Backup before restructuring: flatten rust/ directory"

# Verify we're in a clean state
git status
```

### Phase 2: Copy Missing Stdlib Modules (2 minutes)

Before deleting `rust/stdlib/`, copy modules to main stdlib:

```bash
# Copy math.gr
cp rust/stdlib/math.gr stdlib/math.gr

# Copy string.gr
cp rust/stdlib/string.gr stdlib/string.gr

# Verify copies
ls -la stdlib/math.gr stdlib/string.gr
```

**Result:** Main stdlib now has 15 modules (was 13).

### Phase 3: Move Rust Project to Root (10 minutes)

**Step 3.1: Move Cargo.toml**
```bash
mv rust/Cargo.toml ./Cargo.toml
```

**Step 3.2: Move src/**
```bash
mv rust/src ./src
```

**Step 3.3: Move samples/**
```bash
mv rust/samples ./samples
```

**Step 3.4: Move tests/**
```bash
mv rust/tests ./tests
```

**Step 3.5: Move target/ (build artifacts)**
```bash
# Only if it exists
if [ -d rust/target ]; then
    mv rust/target ./target
fi
```

**Step 3.6: Move benches/ (if exists)**
```bash
if [ -d rust/benches ]; then
    mv rust/benches ./benches
fi
```

**Step 3.7: Move examples/ (if exists)**
```bash
if [ -d rust/examples ]; then
    mv rust/examples ./examples
fi
```

**Step 3.8: Move .gitignore**
```bash
if [ -f rust/.gitignore ]; then
    # Merge with root .gitignore if needed
    cat rust/.gitignore >> .gitignore
    rm rust/.gitignore
fi
```

### Phase 4: Delete Obsolete rust/ Directory (1 minute)

**Step 4.1: Verify rust/ is empty (except obsolete stdlib)**
```bash
ls -la rust/
# Should only show stdlib/ and maybe some config files
```

**Step 4.2: Remove rust/stdlib/**
```bash
rm -rf rust/stdlib/
```

**Step 4.3: Remove remaining rust/ contents**
```bash
# Check what's left
find rust/ -type f

# Remove any remaining files
rm -rf rust/
```

**Verification:**
```bash
# rust/ should be gone
ls -la rust/  # Should error: No such file or directory
```

### Phase 5: Update Source Code (15 minutes)

**Step 5.1: Update module_manager.rs**

File: `src/execution/module_manager.rs`

Change line ~131:
```rust
// OLD:
PathBuf::from("stdlib")

// NEW: (no change needed - "stdlib" now correct!)
PathBuf::from("stdlib")
```

**No change needed** - "stdlib" is now correct relative path from project root!

**Step 5.2: Search for hardcoded "rust/" paths**
```bash
grep -r "rust/" src/ --include="*.rs" | grep -v target | grep -v ".git"
```

Update any hardcoded paths found.

**Step 5.3: Update Cargo.toml paths (if any)**
```bash
cat Cargo.toml | grep -i path
```

Ensure no paths reference old `rust/` structure.

### Phase 6: Update Documentation (20 minutes)

**Step 6.1: Update CLAUDE.md**

Search and replace:
- `cd rust` → `cd /home/irv/work/grang` or just `# From project root`
- `rust/samples/` → `samples/`
- `rust/src/` → `src/`
- `rust/tests/` → `tests/`
- Remove all references to `GRAPHOID_STDLIB_PATH` in examples
- Update directory structure diagram

**Step 6.2: Update samples/README.md**

Replace all:
- `rust/samples/` → `samples/`
- `cd rust` → `# From project root`
- Remove `GRAPHOID_STDLIB_PATH` from all command examples

**Step 6.3: Update docs/WHY_GRAPHOID.md**
- Update installation commands
- Remove `cd rust` steps

**Step 6.4: Update docs/DESIGN_PHILOSOPHY.md**
- Update any code examples with paths

**Step 6.5: Update docs/SAMPLES_AUDIT_REPORT.md**
- Replace `rust/samples/` → `samples/`

**Step 6.6: Update dev_docs/RUST_IMPLEMENTATION_ROADMAP.md**
- Update project structure examples
- Remove `rust/` from all paths

**Step 6.7: Update README.md**
```bash
# Update installation section
# Remove cd rust steps
# Update cargo commands to run from root
```

### Phase 7: Test Everything (15 minutes)

**Step 7.1: Build from root**
```bash
cd /home/irv/work/grang
cargo build
```

**Expected:** ✅ Clean build, no warnings

**Step 7.2: Run tests**
```bash
cargo test --lib
```

**Expected:** ✅ All tests pass

**Step 7.3: Test stdlib imports WITHOUT environment variable**
```bash
# Test math import
echo 'import "math"
print("math.abs(-5):", math.abs(-5))
print("math.max(10, 20):", math.max(10, 20))' > /tmp/test_math_import.gr

cargo run -- /tmp/test_math_import.gr
```

**Expected:** ✅ Math module loads, functions work

**Step 7.4: Test sample programs**
```bash
# Test basics
cargo run -- samples/01-basics/hello_world.gr

# Test behaviors
cargo run -- samples/02-intermediate/behaviors.gr

# Test modules
cargo run -- samples/04-modules/app_main.gr

# Test stdlib
cargo run -- samples/05-stdlib/constants.gr
```

**Expected:** ✅ All samples run successfully without `GRAPHOID_STDLIB_PATH`

**Step 7.5: Run full test suite**
```bash
# Run all unit tests
cargo test --lib

# Run integration tests
cargo test --test '*'
```

### Phase 8: Update Git Configuration (5 minutes)

**Step 8.1: Update .gitignore**

Ensure root `.gitignore` has:
```gitignore
/target/
**/*.rs.bk
Cargo.lock
```

**Step 8.2: Commit the restructuring**
```bash
git add -A
git status  # Review changes

git commit -m "Restructure: Flatten rust/ directory to project root

- Move Cargo.toml, src/, samples/, tests/ to root
- Copy math.gr and string.gr to main stdlib
- Delete rust/stdlib/ (obsolete)
- Remove rust/ directory entirely
- Update all documentation (CLAUDE.md, README.md, docs/)
- Stdlib imports now work without GRAPHOID_STDLIB_PATH
- Standard Rust project structure

BREAKING CHANGE: Project structure changed, run from root now
"
```

---

## Rollback Plan (If Needed)

If something goes wrong:

```bash
# Reset to backup commit
git log --oneline -5  # Find backup commit hash
git reset --hard <backup-commit-hash>

# Alternative: stash changes
git stash save "Failed restructuring attempt"
```

---

## Verification Checklist

After restructuring, verify:

- [ ] `cargo build` succeeds from root
- [ ] `cargo test` passes all tests
- [ ] `cargo run -- samples/01-basics/hello_world.gr` works
- [ ] `import "math"` works without environment variable
- [ ] `import "time"` works (stdlib)
- [ ] `import "statistics"` works (stdlib)
- [ ] All 30 sample files run successfully
- [ ] Documentation paths are correct
- [ ] `rust/` directory is completely gone
- [ ] Git commit is clean

---

## Post-Restructuring Tasks

**Immediate:**
1. Update CI/CD configurations (if any)
2. Notify contributors of structure change
3. Update any external documentation

**Future:**
1. Consider `stdlib/` → `lib/` rename (more standard)
2. Add `stdlib/` documentation
3. Create stdlib module index

---

## Timeline

- **Phase 1:** 5 minutes
- **Phase 2:** 2 minutes
- **Phase 3:** 10 minutes
- **Phase 4:** 1 minute
- **Phase 5:** 15 minutes
- **Phase 6:** 20 minutes
- **Phase 7:** 15 minutes
- **Phase 8:** 5 minutes

**Total estimated time:** ~75 minutes (1.25 hours)

---

## Notes

- This is a **breaking change** for anyone working on the project
- All documentation references to `rust/` will be removed
- Commands will be simpler: `cargo run -- samples/hello.gr` not `cd rust && cargo run -- samples/hello.gr`
- Stdlib imports will "just work" - no environment variables needed
- The `python/` reference implementation remains unchanged

---

**Status:** Ready to execute
**Next Step:** Update session docs, then begin Phase 1

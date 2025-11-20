# Experimental Stdlib Modules

⚠️ **WARNING: Modules in this directory are EXPERIMENTAL and NOT suitable for production use!**

---

## Purpose

This directory contains pure Graphoid implementations of algorithms and modules that:

1. **Demonstrate language capabilities** - Show what can be built in pure .gr code
2. **Stress-test the language** - Identify missing features and performance bottlenecks
3. **Educational value** - Help users understand complex algorithms
4. **Benchmarking** - Measure interpreter performance

These modules are **intentionally unoptimized** compared to their native counterparts.

---

## Current Modules

### crypto_experimental.gr

**Status:** Experimental demonstration only - DO NOT USE IN PRODUCTION

**What it is:**
- Pure Graphoid implementation of SHA-256
- NIST FIPS 180-4 compliant
- ~295 lines of pure .gr code
- No native dependencies

**Performance:**
- ~2.4 seconds per hash (with `:integer` directive)
- **3500x slower** than native `crypto` module

**Use cases:**
- ✅ Educational: Understanding SHA-256 algorithm
- ✅ Language demo: Shows Graphoid can implement complex cryptography
- ✅ Testing: Validates bitwise operations work correctly
- ✅ Benchmarking: Measures interpreter performance
- ❌ Production: Use native `crypto` module instead!

**For production cryptography:**
```graphoid
# Use this instead:
import "crypto"
hash = crypto.sha256("message")  # Fast, secure, audited
```

**For learning/experimentation:**
```graphoid
# Educational use only:
import "experimental/crypto_experimental"
hash = crypto_experimental.sha256("message")  # Slow, but pure Graphoid!
```

---

## Guidelines for Experimental Modules

Modules in this directory MUST:

1. **Have clear warnings** - Top of file must warn users about experimental status
2. **Document limitations** - Performance, security, completeness issues
3. **Reference production alternatives** - Point to optimized native modules
4. **Explain purpose** - Why this exists (demo, testing, education, etc.)
5. **Use clear naming** - Suffix with `_experimental` to avoid confusion

Modules in this directory SHOULD NOT:

1. Be imported by production code
2. Be used in security-critical contexts
3. Be recommended over native alternatives
4. Have ambiguous names that suggest production-readiness

---

## Future Experimental Modules

Potential candidates for this directory:

- **parser_experimental.gr** - Pure Graphoid parser implementation
- **compiler_experimental.gr** - Self-hosting compiler experiments
- **regex_engine_experimental.gr** - Pure Graphoid regex engine
- **interpreter_experimental.gr** - Meta-circular interpreter

---

## When to Move Modules OUT of Experimental

A module can graduate from experimental/ to stdlib/ when:

1. ✅ Performance is acceptable for production use
2. ✅ Security has been audited (if applicable)
3. ✅ API is stable and documented
4. ✅ Test coverage is comprehensive
5. ✅ No better native alternative exists
6. ✅ Maintenance commitment established

---

## Summary

**Use experimental modules for:**
- Learning and education
- Language capability demonstrations
- Benchmarking and testing
- Experimentation and research

**DO NOT use experimental modules for:**
- Production applications
- Security-critical operations
- Performance-sensitive code
- User-facing features

**Always prefer native stdlib modules for production use.**

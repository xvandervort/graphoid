# Graphoid

A graph-theoretic programming language where everything is a graph.

## Building

```bash
cargo build
```

## Running

```bash
# REPL
cargo run

# Execute file
cargo run -- path/to/file.gr
```

## Testing

```bash
# Run all tests
cargo test

# Run specific test file
cargo test graph_rules

# Run with output
cargo test -- --nocapture
```

**Testing Guidelines**: See [../dev_docs/TESTING_GUIDELINES.md](../dev_docs/TESTING_GUIDELINES.md) for detailed testing conventions and best practices.

**MANDATORY Practices**:
- ✅ **TDD Required**: Write tests FIRST, then implement (RED-GREEN-REFACTOR)
- ✅ **Separate Files**: Tests in `tests/unit/` or `tests/integration/`, never inline with `#[cfg(test)]` in `src/`

**Current Status**: 625 tests passing, 100% TDD compliance

## Documentation

See `../dev_docs/` for development documentation and `docs/` for user documentation.

**Development Docs**:
- [../dev_docs/TESTING_GUIDELINES.md](../dev_docs/TESTING_GUIDELINES.md) - Testing conventions and best practices
- [../dev_docs/STATUS.md](../dev_docs/STATUS.md) - Current implementation status
- [../dev_docs/LANGUAGE_SPECIFICATION.md](../dev_docs/LANGUAGE_SPECIFICATION.md) - Language specification
- [../dev_docs/RUST_IMPLEMENTATION_ROADMAP.md](../dev_docs/RUST_IMPLEMENTATION_ROADMAP.md) - Implementation roadmap

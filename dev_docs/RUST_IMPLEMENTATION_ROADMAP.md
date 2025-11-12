# Graphoid/Glang: Rust Implementation Roadmap

**Version**: 2.2
**Last Updated**: November 12, 2025
**Status**: Phase 8 ✅ Complete, Phase 12 (Stdlib) 44% Complete

**🚨 CRITICAL UPDATE (November 6, 2025)**: An executability audit revealed that features implemented at the Rust API level are not accessible from .gr user-facing files. Phase completion claims are being revised based on actual .gr file executability. See `EXECUTABILITY_AUDIT.md` for details.

**Previous Status**: Phase 7 claimed complete (1,609 Rust tests passing), Phase 8 ~75% complete
**Revised Status**: Phases 3-9 usability uncertain, comprehensive audit in progress

**⚠️ IMPORTANT**: This roadmap has been significantly updated based on new language features. See [archived updates document](archive/sessions/2025-01-roadmap-updates/ROADMAP_UPDATES_FOR_NEW_FEATURES.md) for complete details of changes.

This roadmap provides a step-by-step plan for implementing Graphoid/Glang in Rust, following the canonical [Language Specification](LANGUAGE_SPECIFICATION.md).

---

## Philosophy & Approach

### Incremental Development
- Build in layers, testing thoroughly at each stage
- Each phase produces a working subset of the language
- Prioritize core functionality before advanced features
- Use TDD (Test-Driven Development) wherever practical

### Quality Standards
- **Code Quality**: Idiomatic Rust, clear documentation
- **Test Coverage**: Minimum 80% for core features
- **Performance**: Baseline first, optimize later
- **Safety**: Leverage Rust's type system and borrow checker

### Reference Implementation
The old Python implementation in `src/glang/` serves as a reference for behavior, but we're not bound by its architecture. The Rust implementation should be idiomatic and leverage Rust's strengths.

### Test-Driven Development (TDD)
**CRITICAL**: ALL development MUST follow TDD principles:
- Write tests BEFORE implementation (Red → Green → Refactor)
- **TWO-LEVEL TESTING REQUIREMENT** (see below)
- Every feature tested in BOTH REPL and CLI modes
- Minimum test counts specified for each phase

### 🚨 Two-Level Testing Requirement (MANDATORY)

**A feature is NOT complete until it works from `.gr` files.**

Passing Rust unit tests alone is INSUFFICIENT. Every feature requires:

**Level 1: Rust API Testing (Unit Tests)**
- Location: `tests/unit/`
- Purpose: Verify internal implementation correctness
- Tests: Data structures, algorithms, edge cases
- Command: `cargo test --lib`

**Level 2: .gr Integration Testing (Executor Tests)**
- Location: `tests/integration/`
- Purpose: Verify feature is accessible from user-facing language
- Tests: End-to-end workflows, realistic use cases
- Command: `cargo run --quiet path/to/test.gr`

**Feature Completion Checklist**:
- [ ] Rust API implemented in `src/values/*.rs`
- [ ] Rust unit tests written and passing
- [ ] **Method/function registered in `src/execution/executor.rs`** ⚠️ CRITICAL
- [ ] `.gr` integration test written in `tests/integration/`
- [ ] `.gr` integration test passes (verified manually)
- [ ] Example `.gr` file added to `examples/` (if significant feature)
- [ ] Documentation updated

**If ANY checkbox is unchecked, the feature is NOT complete!**

See `INTEGRATION_TESTING_GUIDE.md` for complete implementation details.

---

## Major Updates (January 2025)

This roadmap has been significantly expanded based on 10 new language features added to the specification:

1. **Inline Conditionals** - `if-then-else` expressions, suffix `if`/`unless`
2. **Mutation Operators** - Dual-version methods (`sort()` / `sort!()`)
3. **Graph Querying** - 5-level system from simple navigation to pattern matching DSL
4. **Integer Division** - `//` operator
5. **Element-wise Operations** - `.* .+ .^ ./ .//` etc. for vectorized operations
6. **Freeze Model** - Deep/shallow freeze with control rules
7. **Project Structure** - `graphoid.toml` manifest system
8. **REPL/CLI Parity** - Strict consistency requirements
9. **Error Handling** - Configurable modes (`strict`, `lenient`, `collect`)
10. **Graph Indexing** - `graph["node_id"]` consistency with hash/list

### Key Changes to Roadmap

**New Phase Added**:
- **Phase 6a**: Standard Graph Types Library (pure Graphoid) - 5-7 days
  - Implement BST, DAG, Heap, AVL, Trie in pure Graphoid
  - Dogfooding the language to build language features
  - Used for testing the graph query system

**Documentation Milestones (Updated for Revised Phase Ordering)**:
- After Phase 5: Basic syntax and collections samples (2-3 days)
- After Phase 6: Graph programming guide (3-4 days)
- After Phase 9: Advanced pattern matching guide (2-3 days)
- After Phase 10: Multi-file project guide (2-3 days)
- After Phase 12: Complete reference (4-5 days)

**Timeline Increases (Updated for Revised Phase Ordering)**:
- Phase 1: +1 day (new tokens: `then`, `unless`, `//`, `!`, element-wise operators)
- Phase 2: +2 days (inline conditionals, try/catch, configure blocks)
- Phase 3: +2 days (configuration stack, error modes - ✅ already done)
- Phase 5: +3 days (mutation operator convention doubles method surface area)
- Phase 6: +7 days (5-level graph querying is the "make or break" feature)
- Phase 7: +5 days (NEW - Function pattern matching)
- Phase 8: +2 days (Behavior system completion - was old Phase 7)
- Phase 9: +7 days (NEW - Graph pattern matching & Level 3-5 querying)
- Phase 10: +3 days (Module system completion - was old Phase 8)
- **Note**: Old "Phase 9 Advanced Features" (config/error/precision) are ✅ already implemented

**See**: [ROADMAP_UPDATES_FOR_NEW_FEATURES.md](archive/sessions/2025-01-roadmap-updates/ROADMAP_UPDATES_FOR_NEW_FEATURES.md) for complete details.

---

## Phase 0: Project Setup & Foundation

**Duration**: 1-2 days
**Goal**: Create project structure and development environment

### Tasks

#### 1. Initialize Rust Project
```bash
# Create new Rust project
cargo new --lib graphoid
cd graphoid

# Set up directory structure
mkdir -p src/{lexer,parser,ast,execution,values,graph,stdlib}
mkdir -p tests/{unit,integration}
mkdir -p benches
mkdir -p examples
mkdir -p docs
```

#### 2. Configure Cargo.toml
```toml
[package]
name = "graphoid"
version = "0.1.0"
edition = "2021"
authors = ["Your Name"]
description = "Graphoid/Glang: A graph-theoretic programming language"
license = "MIT OR Apache-2.0"

[dependencies]
# Core dependencies
thiserror = "1.0"        # Error handling
lazy_static = "1.4"      # Global state
regex = "1.10"           # Regex support
chrono = "0.4"           # Time handling

# Random number generation
rand = "0.8"
rand_distr = "0.4"

# Cryptography
sha2 = "0.10"
ed25519-dalek = "2.0"
aes-gcm = "0.10"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Optional: Performance profiling
criterion = { version = "0.5", optional = true }

[dev-dependencies]
pretty_assertions = "1.4"

[features]
default = []
profiling = ["criterion"]

[[bin]]
name = "graphoid"
path = "src/main.rs"

[lib]
name = "graphoid"
path = "src/lib.rs"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

#### 3. Create Basic Project Structure

**src/lib.rs**:
```rust
//! Graphoid/Glang: A graph-theoretic programming language
//!
//! This crate implements the Graphoid language, where everything is a graph.

pub mod lexer;
pub mod parser;
pub mod ast;
pub mod execution;
pub mod values;
pub mod graph;
pub mod error;

pub use error::{GraphoidError, Result};
```

**src/main.rs**:
```rust
//! Graphoid CLI and REPL

use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // File execution mode
        run_file(&args[1]);
    } else {
        // REPL mode
        run_repl();
    }
}

fn run_file(path: &str) {
    match fs::read_to_string(path) {
        Ok(source) => {
            println!("Executing: {}", path);
            // TODO: Execute the file
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_repl() {
    println!("Graphoid v0.1.0");
    println!("Type /exit to quit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "/exit" {
            break;
        }

        // TODO: Execute input
        println!("TODO: Execute '{}'", input);
    }
}
```

**src/error.rs**:
```rust
//! Error types for Graphoid

use std::fmt;

pub type Result<T> = std::result::Result<T, GraphoidError>;

#[derive(Debug)]
pub struct SourcePosition {
    pub line: usize,
    pub column: usize,
    pub file: Option<String>,
}

impl fmt::Display for SourcePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)?;
        if let Some(ref file) = self.file {
            write!(f, " in {}", file)?;
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GraphoidError {
    #[error("Syntax error: {message} at {position}")]
    SyntaxError {
        message: String,
        position: SourcePosition,
    },

    #[error("Type error: {message} at {position}")]
    TypeError {
        message: String,
        position: SourcePosition,
    },

    #[error("Runtime error: {message}")]
    RuntimeError { message: String },

    #[error("Graph rule violated: {rule} - {message}")]
    RuleViolation { rule: String, message: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
```

#### 4. Set Up Testing Infrastructure

**tests/unit/mod.rs**:
```rust
//! Unit test organization

#[cfg(test)]
mod lexer_tests;
#[cfg(test)]
mod parser_tests;
#[cfg(test)]
mod value_tests;
```

#### 5. Set Up CI/CD (Optional but Recommended)

**.github/workflows/rust.yml**:
```yaml
name: Rust

on:
  push:
    branches: [ main, rust_port ]
  pull_request:
    branches: [ main, rust_port ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Build
      run: cargo build --verbose
    - name: Run tests
      run: cargo test --verbose
    - name: Check formatting
      run: cargo fmt --check
    - name: Run clippy
      run: cargo clippy -- -D warnings
```

#### 6. Documentation Setup

**README.md** (in project root):
```markdown
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
cargo test
```

## Documentation

See `dev_docs/` for development documentation and `docs/` for user documentation.
```

### Success Criteria
- ✅ `cargo build` succeeds
- ✅ `cargo test` runs (even with no tests yet)
- ✅ Project structure in place
- ✅ Basic CLI runs and shows REPL prompt

---

## Phase 1: Lexer (Tokenization)

**Duration**: 4-6 days (+1 day for new language features)
**Status**: 🟡 PARTIALLY COMPLETE - Lexer implementation updated with new tokens
**Goal**: Convert source code into tokens

**✅ COMPLETED**:
- Comment syntax changed from `//` to `#` (single-line)
- `//` now tokenizes as integer division (`SlashSlash`)
- Added `then` and `unless` keywords
- Added `!` token for mutation operators (`Bang`)
- Added all element-wise operators: `.+`, `.-`, `.*`, `./`, `.//`, `.%`, `.^`, `.==`, `.!=`, `.<`, `.<=`, `.>`, `.>=`
- Lexer tests passing with new tokens

**🔲 TODO**:
- Complete lexer test coverage (currently basic tests only)
- Add error handling tests (invalid input, edge cases)
- Reach 20+ comprehensive tests
- Add tests for `try`, `catch`, `finally`, `raise` keywords (Phase 3 feature)

### Tasks

#### 1. Define Token Types

**src/lexer/token.rs**:
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Number(f64),
    String(String),
    Symbol(String),      // :symbol
    Regex(String),       // /pattern/flags
    True,
    False,
    None,

    // Identifiers and Keywords
    Identifier(String),
    Func,
    If,
    Else,
    While,
    For,
    In,
    Return,
    Break,
    Continue,
    Import,
    Load,
    Module,
    Alias,
    Configure,
    Precision,
    And,
    Or,
    Not,

    // Types
    Num,
    String,
    Bool,
    List,
    Map,
    Tree,
    Graph,
    Data,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    RegexMatch,      // =~
    RegexNoMatch,    // !~
    Arrow,           // =>

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Colon,
    Semicolon,

    // Special
    Newline,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize, column: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            column,
        }
    }
}
```

#### 2. Implement Lexer

**src/lexer/mod.rs**:
```rust
pub mod token;

use token::{Token, TokenType};
use crate::error::{GraphoidError, Result, SourcePosition};

pub struct Lexer {
    source: Vec<char>,
    current: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace_except_newline();

            if self.is_at_end() {
                break;
            }

            let token = self.next_token()?;
            tokens.push(token);
        }

        tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            self.line,
            self.column,
        ));

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token> {
        let start_line = self.line;
        let start_column = self.column;
        let ch = self.advance();

        let token_type = match ch {
            // Single character tokens
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            ':' => {
                // Could be : or :symbol
                if self.peek().is_alphabetic() {
                    return self.symbol();
                }
                TokenType::Colon
            }
            ';' => TokenType::Semicolon,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Star,
            '%' => TokenType::Percent,
            '^' => TokenType::Caret,

            // Two-character tokens
            '=' => {
                if self.match_char('=') {
                    TokenType::EqualEqual
                } else if self.match_char('~') {
                    TokenType::RegexMatch
                } else if self.match_char('>') {
                    TokenType::Arrow
                } else {
                    TokenType::Equal
                }
            }
            '!' => {
                if self.match_char('=') {
                    TokenType::BangEqual
                } else if self.match_char('~') {
                    TokenType::RegexNoMatch
                } else {
                    return Err(GraphoidError::SyntaxError {
                        message: format!("Unexpected character: {}", ch),
                        position: SourcePosition {
                            line: start_line,
                            column: start_column,
                            file: None,
                        },
                    });
                }
            }
            '<' => {
                if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            '>' => {
                if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }

            // Division or comment
            '/' => {
                if self.peek() == '/' {
                    // Single-line comment
                    self.skip_line_comment();
                    return self.next_token();
                } else if self.peek() == '*' {
                    // Multi-line comment
                    self.skip_block_comment()?;
                    return self.next_token();
                } else if self.is_regex_context() {
                    // Regex literal
                    return self.regex();
                } else {
                    TokenType::Slash
                }
            }

            // Strings
            '"' | '\'' => return self.string(ch),

            // Numbers
            ch if ch.is_ascii_digit() => return self.number(),

            // Identifiers and keywords
            ch if ch.is_alphabetic() || ch == '_' => return self.identifier(),

            // Newlines
            '\n' => {
                self.line += 1;
                self.column = 1;
                TokenType::Newline
            }

            _ => {
                return Err(GraphoidError::SyntaxError {
                    message: format!("Unexpected character: {}", ch),
                    position: SourcePosition {
                        line: start_line,
                        column: start_column,
                        file: None,
                    },
                });
            }
        };

        Ok(Token::new(
            token_type,
            ch.to_string(),
            start_line,
            start_column,
        ))
    }

    // Helper methods (implement these)
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let ch = self.source[self.current];
        self.current += 1;
        self.column += 1;
        ch
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            false
        } else {
            self.advance();
            true
        }
    }

    fn skip_whitespace_except_newline(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    // TODO: Implement these methods
    fn skip_line_comment(&mut self) { /* ... */ }
    fn skip_block_comment(&mut self) -> Result<()> { /* ... */ }
    fn string(&mut self, quote: char) -> Result<Token> { /* ... */ }
    fn number(&mut self) -> Result<Token> { /* ... */ }
    fn identifier(&mut self) -> Result<Token> { /* ... */ }
    fn symbol(&mut self) -> Result<Token> { /* ... */ }
    fn regex(&mut self) -> Result<Token> { /* ... */ }
    fn is_regex_context(&self) -> bool { /* ... */ }
}
```

#### 3. Write Lexer Tests

**tests/unit/lexer_tests.rs**:
```rust
use graphoid::lexer::{Lexer, token::TokenType};

#[test]
fn test_simple_tokens() {
    let mut lexer = Lexer::new("+ - * / % ^");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type, TokenType::Plus);
    assert_eq!(tokens[1].token_type, TokenType::Minus);
    assert_eq!(tokens[2].token_type, TokenType::Star);
    assert_eq!(tokens[3].token_type, TokenType::Slash);
    assert_eq!(tokens[4].token_type, TokenType::Percent);
    assert_eq!(tokens[5].token_type, TokenType::Caret);
}

#[test]
fn test_numbers() {
    let mut lexer = Lexer::new("42 3.14 0.5");
    let tokens = lexer.tokenize().unwrap();

    match tokens[0].token_type {
        TokenType::Number(n) => assert_eq!(n, 42.0),
        _ => panic!("Expected number"),
    }

    match tokens[1].token_type {
        TokenType::Number(n) => assert_eq!(n, 3.14),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_strings() {
    let mut lexer = Lexer::new(r#""hello" 'world'"#);
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::String(s) => assert_eq!(s, "hello"),
        _ => panic!("Expected string"),
    }

    match &tokens[1].token_type {
        TokenType::String(s) => assert_eq!(s, "world"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn test_identifiers_and_keywords() {
    let mut lexer = Lexer::new("fn if else while name");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type, TokenType::Fn);
    assert_eq!(tokens[1].token_type, TokenType::If);
    assert_eq!(tokens[2].token_type, TokenType::Else);
    assert_eq!(tokens[3].token_type, TokenType::While);

    match &tokens[4].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "name"),
        _ => panic!("Expected identifier"),
    }
}

#[test]
fn test_symbols() {
    let mut lexer = Lexer::new(":ok :error :pending");
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::Symbol(s) => assert_eq!(s, "ok"),
        _ => panic!("Expected symbol"),
    }
}

#[test]
fn test_comments() {
    let mut lexer = Lexer::new("42 // comment\n43 /* block */ 44");
    let tokens = lexer.tokenize().unwrap();

    // Should skip comments
    match tokens[0].token_type {
        TokenType::Number(n) => assert_eq!(n, 42.0),
        _ => panic!(),
    }

    match tokens[2].token_type {
        TokenType::Number(n) => assert_eq!(n, 43.0),
        _ => panic!(),
    }
}
```

### Success Criteria
- ✅ All token types recognized
- ✅ Position tracking accurate (line, column)
- ✅ Comments properly skipped
- ✅ String escapes handled
- ✅ Numbers (integer and float) parsed
- ✅ Keywords vs identifiers distinguished
- ✅ 20+ passing tests

---

## Phase 2: Parser & AST

**Duration**: 5-7 days
**Goal**: Build Abstract Syntax Tree from tokens

### Tasks

#### 1. Define AST Node Types

**src/ast/mod.rs**:
```rust
use crate::error::SourcePosition;

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VariableDecl {
        name: String,
        type_annotation: Option<TypeAnnotation>,
        value: Expr,
        position: SourcePosition,
    },
    Assignment {
        target: AssignmentTarget,
        value: Expr,
        position: SourcePosition,
    },
    FunctionDecl {
        name: String,
        params: Vec<Parameter>,
        body: Vec<Stmt>,
        position: SourcePosition,
    },
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
        position: SourcePosition,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
        position: SourcePosition,
    },
    For {
        variable: String,
        iterable: Expr,
        body: Vec<Stmt>,
        position: SourcePosition,
    },
    Return {
        value: Option<Expr>,
        position: SourcePosition,
    },
    Break {
        position: SourcePosition,
    },
    Continue {
        position: SourcePosition,
    },
    Import {
        module: String,
        alias: Option<String>,
        position: SourcePosition,
    },
    ModuleDecl {
        name: String,
        alias: Option<String>,
        position: SourcePosition,
    },
    Expression {
        expr: Expr,
        position: SourcePosition,
    },
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal {
        value: LiteralValue,
        position: SourcePosition,
    },
    Variable {
        name: String,
        position: SourcePosition,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        position: SourcePosition,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
        position: SourcePosition,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        position: SourcePosition,
    },
    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Expr>,
        position: SourcePosition,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
        position: SourcePosition,
    },
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
        position: SourcePosition,
    },
    List {
        elements: Vec<Expr>,
        position: SourcePosition,
    },
    Map {
        entries: Vec<(String, Expr)>,
        position: SourcePosition,
    },
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    None,
    Symbol(String),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add, Subtract, Multiply, Divide, Modulo, Power,
    Equal, NotEqual, Less, LessEqual, Greater, GreaterEqual,
    And, Or,
    RegexMatch, RegexNoMatch,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub default_value: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct TypeAnnotation {
    pub base_type: String,
    pub constraint: Option<String>,
}

#[derive(Debug, Clone)]
pub enum AssignmentTarget {
    Variable(String),
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
}
```

#### 2. Implement Recursive Descent Parser

**src/parser/mod.rs**:
```rust
use crate::lexer::token::{Token, TokenType};
use crate::ast::*;
use crate::error::{GraphoidError, Result, SourcePosition};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            // Skip newlines between statements
            self.skip_newlines();

            if self.is_at_end() {
                break;
            }

            statements.push(self.statement()?);
        }

        Ok(Program { statements })
    }

    fn statement(&mut self) -> Result<Stmt> {
        // Skip leading newlines
        self.skip_newlines();

        match &self.peek().token_type {
            TokenType::Func => self.function_declaration(),
            TokenType::If => self.if_statement(),
            TokenType::While => self.while_statement(),
            TokenType::For => self.for_statement(),
            TokenType::Return => self.return_statement(),
            TokenType::Break => self.break_statement(),
            TokenType::Continue => self.continue_statement(),
            TokenType::Import => self.import_statement(),
            TokenType::Module => self.module_declaration(),

            // Type annotations or identifiers
            TokenType::Num | TokenType::String | TokenType::Bool |
            TokenType::List | TokenType::Map | TokenType::Tree | TokenType::Graph => {
                self.variable_or_assignment()
            }

            TokenType::Identifier(_) => {
                // Could be variable declaration or assignment
                self.variable_or_assignment()
            }

            _ => {
                // Expression statement
                let expr = self.expression()?;
                self.skip_newlines();
                Ok(Stmt::Expression {
                    position: expr.position().clone(),
                    expr,
                })
            }
        }
    }

    fn expression(&mut self) -> Result<Expr> {
        self.or_expression()
    }

    fn or_expression(&mut self) -> Result<Expr> {
        let mut left = self.and_expression()?;

        while self.match_token(&[TokenType::Or]) {
            let op = BinaryOp::Or;
            let right = self.and_expression()?;
            let position = left.position().clone();
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
                position,
            };
        }

        Ok(left)
    }

    fn and_expression(&mut self) -> Result<Expr> {
        let mut left = self.equality()?;

        while self.match_token(&[TokenType::And]) {
            let op = BinaryOp::And;
            let right = self.equality()?;
            let position = left.position().clone();
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
                position,
            };
        }

        Ok(left)
    }

    // Implement precedence climbing for expressions
    fn equality(&mut self) -> Result<Expr> { /* ... */ }
    fn comparison(&mut self) -> Result<Expr> { /* ... */ }
    fn term(&mut self) -> Result<Expr> { /* ... */ }
    fn factor(&mut self) -> Result<Expr> { /* ... */ }
    fn unary(&mut self) -> Result<Expr> { /* ... */ }
    fn power(&mut self) -> Result<Expr> { /* ... */ }
    fn call(&mut self) -> Result<Expr> { /* ... */ }
    fn primary(&mut self) -> Result<Expr> { /* ... */ }

    // Statement parsing methods
    fn function_declaration(&mut self) -> Result<Stmt> { /* ... */ }
    fn if_statement(&mut self) -> Result<Stmt> { /* ... */ }
    fn while_statement(&mut self) -> Result<Stmt> { /* ... */ }
    fn for_statement(&mut self) -> Result<Stmt> { /* ... */ }
    fn return_statement(&mut self) -> Result<Stmt> { /* ... */ }
    fn variable_or_assignment(&mut self) -> Result<Stmt> { /* ... */ }

    // Helper methods
    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens[self.current - 1].clone()
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn skip_newlines(&mut self) {
        while matches!(self.peek().token_type, TokenType::Newline) {
            self.advance();
        }
    }

    fn position(&self) -> SourcePosition {
        let token = self.peek();
        SourcePosition {
            line: token.line,
            column: token.column,
            file: None,
        }
    }
}

impl Expr {
    fn position(&self) -> &SourcePosition {
        match self {
            Expr::Literal { position, .. } => position,
            Expr::Variable { position, .. } => position,
            Expr::Binary { position, .. } => position,
            // ... etc
        }
    }
}
```

#### 3. Write Parser Tests

**tests/unit/parser_tests.rs**:
```rust
use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
use graphoid::ast::*;

#[test]
fn test_parse_number() {
    let mut lexer = Lexer::new("42");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::Literal { value: LiteralValue::Number(n), .. } => {
                    assert_eq!(*n, 42.0);
                }
                _ => panic!("Expected number literal"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parse_binary_expression() {
    let mut lexer = Lexer::new("2 + 3 * 4");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    // Should parse as: 2 + (3 * 4) due to precedence
    // Verify AST structure
}

#[test]
fn test_parse_variable_declaration() {
    let mut lexer = Lexer::new("num x = 42");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::VariableDecl { name, .. } => {
            assert_eq!(name, "x");
        }
        _ => panic!("Expected variable declaration"),
    }
}

#[test]
fn test_parse_function() {
    let source = r#"
    fn add(x, y) {
        return x + y
    }
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::FunctionDecl { name, params, .. } => {
            assert_eq!(name, "add");
            assert_eq!(params.len(), 2);
        }
        _ => panic!("Expected function declaration"),
    }
}
```

### Success Criteria
- ✅ All statement types parsed correctly
- ✅ Expression precedence handled correctly
- ✅ Error messages show source positions
- ✅ 30+ passing tests

---

## Phase 3: Value System & Basic Execution

**Duration**: 5-7 days
**Goal**: Execute simple expressions and statements

### Tasks

#### 1. Implement Value System

**src/values/mod.rs**:
```rust
use std::collections::HashMap;
use crate::error::Result;

#[derive(Debug, Clone, PartialEq)]
pub struct NumberData {
    pub value: f64,
    pub display_precision: Option<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(NumberData),
    String(String),
    Boolean(bool),
    None,
    Symbol(String),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
    // More types added later
}

impl Value {
    pub fn number(n: f64) -> Self {
        Value::Number(NumberData {
            value: n,
            display_precision: None,
        })
    }

    pub fn number_with_precision(n: f64, precision: i64) -> Self {
        Value::Number(NumberData {
            value: n,
            display_precision: Some(precision),
        })
    }

    pub fn get_type(&self) -> &str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Boolean(_) => "bool",
            Value::None => "none",
            Value::Symbol(_) => "symbol",
            Value::List(_) => "list",
            Value::Map(_) => "map",
        }
    }

    pub fn to_display_string(&self) -> String {
        match self {
            Value::Number(n) => {
                if let Some(precision) = n.display_precision {
                    format!("{:.prec$}", n.value, prec = precision as usize)
                } else if n.value.fract() == 0.0 && n.value.is_finite() {
                    format!("{}", n.value as i64)
                } else {
                    format!("{}", n.value)
                }
            }
            Value::String(s) => s.clone(),
            Value::Boolean(b) => if *b { "true" } else { "false" }.to_string(),
            Value::None => "none".to_string(),
            Value::Symbol(s) => format!(":{}", s),
            Value::List(items) => {
                let strs: Vec<String> = items.iter()
                    .map(|v| match v {
                        Value::String(s) => format!("\"{}\"", s),
                        _ => v.to_display_string(),
                    })
                    .collect();
                format!("[{}]", strs.join(", "))
            }
            Value::Map(m) => {
                let strs: Vec<String> = m.iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, v.to_display_string()))
                    .collect();
                format!("{{{}}}", strs.join(", "))
            }
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::None => false,
            Value::Number(n) => n.value != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Map(m) => !m.is_empty(),
            _ => true,
        }
    }

    // Arithmetic operations
    pub fn add(&self, other: &Value) -> Result<Value> { /* ... */ }
    pub fn subtract(&self, other: &Value) -> Result<Value> { /* ... */ }
    pub fn multiply(&self, other: &Value) -> Result<Value> { /* ... */ }
    pub fn divide(&self, other: &Value) -> Result<Value> { /* ... */ }
    pub fn modulo(&self, other: &Value) -> Result<Value> { /* ... */ }
    pub fn power(&self, other: &Value) -> Result<Value> { /* ... */ }

    // Comparison
    pub fn equals(&self, other: &Value) -> bool { /* ... */ }
    pub fn less_than(&self, other: &Value) -> Result<bool> { /* ... */ }
    pub fn greater_than(&self, other: &Value) -> Result<bool> { /* ... */ }
}
```

#### 2. Implement Execution Engine

**src/execution/mod.rs**:
```rust
use std::collections::HashMap;
use crate::ast::*;
use crate::values::Value;
use crate::error::{GraphoidError, Result};

pub struct Executor {
    globals: HashMap<String, Value>,
    locals: Vec<HashMap<String, Value>>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
            locals: Vec::new(),
        }
    }

    pub fn execute(&mut self, program: &Program) -> Result<Value> {
        let mut last_value = Value::None;

        for stmt in &program.statements {
            last_value = self.execute_statement(stmt)?;
        }

        Ok(last_value)
    }

    fn execute_statement(&mut self, stmt: &Stmt) -> Result<Value> {
        match stmt {
            Stmt::VariableDecl { name, value, .. } => {
                let val = self.evaluate_expression(value)?;
                self.define_variable(name.clone(), val.clone());
                Ok(val)
            }

            Stmt::Assignment { target, value, .. } => {
                let val = self.evaluate_expression(value)?;
                match target {
                    AssignmentTarget::Variable(name) => {
                        self.assign_variable(name, val.clone())?;
                    }
                    AssignmentTarget::Index { object, index } => {
                        // TODO: Handle index assignment
                    }
                }
                Ok(val)
            }

            Stmt::Expression { expr, .. } => {
                self.evaluate_expression(expr)
            }

            Stmt::If { condition, then_branch, else_branch, .. } => {
                let cond = self.evaluate_expression(condition)?;
                if cond.is_truthy() {
                    self.execute_block(then_branch)
                } else if let Some(else_stmts) = else_branch {
                    self.execute_block(else_stmts)
                } else {
                    Ok(Value::None)
                }
            }

            Stmt::While { condition, body, .. } => {
                while self.evaluate_expression(condition)?.is_truthy() {
                    self.execute_block(body)?;
                }
                Ok(Value::None)
            }

            Stmt::Return { value, .. } => {
                let val = if let Some(expr) = value {
                    self.evaluate_expression(expr)?
                } else {
                    Value::None
                };
                // TODO: Handle return properly (needs control flow)
                Ok(val)
            }

            _ => Ok(Value::None),
        }
    }

    fn evaluate_expression(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Literal { value, .. } => {
                Ok(match value {
                    LiteralValue::Number(n) => Value::number(*n),
                    LiteralValue::String(s) => Value::String(s.clone()),
                    LiteralValue::Boolean(b) => Value::Boolean(*b),
                    LiteralValue::None => Value::None,
                    LiteralValue::Symbol(s) => Value::Symbol(s.clone()),
                })
            }

            Expr::Variable { name, position } => {
                self.get_variable(name)
                    .ok_or_else(|| GraphoidError::RuntimeError {
                        message: format!("Undefined variable: {}", name),
                    })
            }

            Expr::Binary { left, op, right, .. } => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;

                match op {
                    BinaryOp::Add => left_val.add(&right_val),
                    BinaryOp::Subtract => left_val.subtract(&right_val),
                    BinaryOp::Multiply => left_val.multiply(&right_val),
                    BinaryOp::Divide => left_val.divide(&right_val),
                    BinaryOp::Modulo => left_val.modulo(&right_val),
                    BinaryOp::Power => left_val.power(&right_val),

                    BinaryOp::Equal => Ok(Value::Boolean(left_val.equals(&right_val))),
                    BinaryOp::NotEqual => Ok(Value::Boolean(!left_val.equals(&right_val))),
                    BinaryOp::Less => Ok(Value::Boolean(left_val.less_than(&right_val)?)),
                    BinaryOp::Greater => Ok(Value::Boolean(left_val.greater_than(&right_val)?)),

                    BinaryOp::And => {
                        Ok(Value::Boolean(left_val.is_truthy() && right_val.is_truthy()))
                    }
                    BinaryOp::Or => {
                        Ok(Value::Boolean(left_val.is_truthy() || right_val.is_truthy()))
                    }

                    _ => Err(GraphoidError::RuntimeError {
                        message: format!("Operator {:?} not yet implemented", op),
                    }),
                }
            }

            Expr::Unary { op, operand, .. } => {
                let val = self.evaluate_expression(operand)?;
                match op {
                    UnaryOp::Negate => {
                        if let Value::Number(n) = val {
                            Ok(Value::Number(NumberData {
                                value: -n.value,
                                display_precision: n.display_precision,
                            }))
                        } else {
                            Err(GraphoidError::RuntimeError {
                                message: format!("Cannot negate {}", val.get_type()),
                            })
                        }
                    }
                    UnaryOp::Not => Ok(Value::Boolean(!val.is_truthy())),
                }
            }

            Expr::List { elements, .. } => {
                let mut values = Vec::new();
                for elem in elements {
                    values.push(self.evaluate_expression(elem)?);
                }
                Ok(Value::List(values))
            }

            Expr::Map { entries, .. } => {
                let mut map = HashMap::new();
                for (key, value_expr) in entries {
                    let value = self.evaluate_expression(value_expr)?;
                    map.insert(key.clone(), value);
                }
                Ok(Value::Map(map))
            }

            _ => Err(GraphoidError::RuntimeError {
                message: "Expression type not yet implemented".to_string(),
            }),
        }
    }

    fn execute_block(&mut self, stmts: &[Stmt]) -> Result<Value> {
        self.push_scope();
        let mut last = Value::None;
        for stmt in stmts {
            last = self.execute_statement(stmt)?;
        }
        self.pop_scope();
        Ok(last)
    }

    fn define_variable(&mut self, name: String, value: Value) {
        if let Some(scope) = self.locals.last_mut() {
            scope.insert(name, value);
        } else {
            self.globals.insert(name, value);
        }
    }

    fn assign_variable(&mut self, name: &str, value: Value) -> Result<()> {
        // Try locals first
        for scope in self.locals.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }

        // Then globals
        if self.globals.contains_key(name) {
            self.globals.insert(name.to_string(), value);
            Ok(())
        } else {
            Err(GraphoidError::RuntimeError {
                message: format!("Undefined variable: {}", name),
            })
        }
    }

    fn get_variable(&self, name: &str) -> Option<Value> {
        // Try locals first
        for scope in self.locals.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Some(val.clone());
            }
        }

        // Then globals
        self.globals.get(name).cloned()
    }

    fn push_scope(&mut self) {
        self.locals.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.locals.pop();
    }
}
```

#### 3. Write Execution Tests

**tests/unit/execution_tests.rs**:
```rust
use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
use graphoid::execution::Executor;
use graphoid::values::Value;

fn execute(source: &str) -> Value {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut executor = Executor::new();
    executor.execute(&program).unwrap()
}

#[test]
fn test_execute_number() {
    let result = execute("42");
    match result {
        Value::Number(n) => assert_eq!(n.value, 42.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_execute_arithmetic() {
    let result = execute("2 + 3 * 4");
    match result {
        Value::Number(n) => assert_eq!(n.value, 14.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_execute_variable() {
    let result = execute("x = 42\nx + 1");
    match result {
        Value::Number(n) => assert_eq!(n.value, 43.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_execute_if() {
    let result = execute(r#"
        x = 5
        if x > 3 {
            y = 10
        } else {
            y = 20
        }
        y
    "#);
    match result {
        Value::Number(n) => assert_eq!(n.value, 10.0),
        _ => panic!("Expected number"),
    }
}
```

### Success Criteria
- ✅ Basic arithmetic works
- ✅ Variables can be defined and accessed
- ✅ If/else statements work
- ✅ While loops work
- ✅ Scoping rules enforced
- ✅ 25+ passing tests

---

## Phase 4: Functions & Lambdas

**Duration**: 4-6 days (completed)
**Status**: ✅ COMPLETE - 521 tests passing
**Goal**: Support user-defined functions and lambdas

### Implemented Features

#### Core Function System
- Regular function declarations with `fn` keyword
- Function parameters with default values
- Named argument passing
- Variadic parameters (`*args`)
- Function closures with environment capture
- Recursive functions
- Function call stack management

#### Lambda Expressions
- Single-expression lambdas: `x => x * 2`
- Multi-parameter lambdas: `(x, y) => x + y`
- **Lambda block bodies**: `x => { temp = x * 2; return temp + 1 }`
- Closure capture in lambdas
- Lambdas as first-class values

#### Trailing Block Syntax
- **Ruby/Smalltalk-style trailing blocks**: `list.map { |x| x * 2 }`
- Multi-parameter blocks: `{ |x, y| x + y }`
- Empty parameter list: `{ || print("Hello") }`
- Method chaining with blocks: `.filter { |x| x > 0 }.map { |x| x * 2 }`
- Trailing blocks on function calls: `apply(5) { |n| n * 2 }`
- Complex block bodies with statements, conditionals, loops

#### Test Coverage
- 22 advanced function tests (default params, named args, variadics, closures, recursion)
- 15 lambda block body tests (single/multi-param, conditionals, loops, closure capture)
- 17 trailing block syntax tests (map/filter/each, chaining, mixed syntax)
- 100% pass rate across all function-related features

**Notes**:
- Lambda block bodies and trailing block syntax were originally planned for Phase 11 but implemented in Phase 4 to support Phase 5 (Collections & Methods)
- Trailing blocks are syntactic sugar - they desugar to regular lambda arguments
- All features fully integrated with the execution engine and support proper closure semantics

---

## Remaining Phases Summary

### Phase 5: Collections & Methods (7-10 days)
- List operations and methods
- Map operations
- String methods
- Method dispatch system
- **Note**: Graphs are NOT implemented in Phase 5. Phase 6 implements graphs with tree{} as syntactic sugar for graph{}.with_ruleset(:tree)

### Phase 6: Graph Types, Rules & Auto-Performance (14-18 days)

**Duration**: 14-18 days  
**Goal**: Production-ready graph system with automatic performance optimization  
**Philosophy**: "It just works" - Graphs optimize themselves transparently

**IMPORTANT**: This phase focuses on **automatic optimization only**. Manual performance tuning (hints, explicit indices, query plan manipulation) is deferred to **v2.0+**. See `dev_docs/FUTURE_FEATURES.md` for advanced features.

### Overview

Phase 6 delivers a complete, performant graph system that automatically optimizes based on usage patterns and declared rules. Users don't need to think about performance - the graph learns and adapts.

### Week 1 (Days 1-6): Core Graph Structures

**Goal**: Basic graph operations with index-free adjacency

**IMPORTANT PHILOSOPHY**: Trees are NOT a separate type. Trees are graphs with rules applied. The `tree{}` syntax is syntactic sugar for `graph{}.with_ruleset(:tree)`, implemented in Week 2 when the rule system is added.

#### Tasks

1. **Graph Value Representation** (`src/values/graph.rs`)
   - Node storage with direct neighbor pointers (index-free adjacency)
   - Edge storage with properties
   - Directed vs undirected graph support
   - Traversal methods (BFS, DFS, in-order, pre-order, post-order)
   - Note: Traversal methods work on ALL graphs, not just trees

2. **Basic Graph Operations**
   - `add_node(id, value)` - O(1)
   - `add_edge(from, to, type, properties)` - O(1)
   - `insert(value, parent?)` - Convenience method for tree-like insertion
   - `remove_node(id)` - O(degree)
   - `remove_edge(from, to)` - O(1)
   - `neighbors(id)` - O(1) lookup, O(degree) iteration
   - `has_node(id)`, `has_edge(from, to)` - O(1)
   - `contains(value)` - Search for node with value

3. **Traversal Methods** (work on all graphs)
   - `bfs(start_node)` - Breadth-first traversal
   - `dfs(start_node)` - Depth-first traversal
   - `in_order()` - In-order traversal (assumes binary tree structure)
   - `pre_order()` - Pre-order traversal
   - `post_order()` - Post-order traversal
   - Note: Tree traversals don't enforce constraints yet (rules added in Week 2)

#### Tests (Week 1)

```rust
#[test]
fn test_graph_creation() {
    let mut g = Graph::new(GraphType::Directed);
    g.add_node("alice", Value::Number(1.0));
    g.add_node("bob", Value::Number(2.0));
    g.add_edge("alice", "bob", "follows", HashMap::new());

    assert_eq!(g.node_count(), 2);
    assert_eq!(g.edge_count(), 1);
    assert!(g.has_edge("alice", "bob"));
}

#[test]
fn test_index_free_adjacency() {
    let mut g = create_large_graph(10000);

    // Neighbor lookup should be O(1), not require index scan
    let start = Instant::now();
    let neighbors = g.neighbors("node_5000");
    let elapsed = start.elapsed();

    assert!(elapsed < Duration::from_micros(10)); // Very fast
}

#[test]
fn test_graph_insert_and_traversal() {
    // Create a graph and use it like a binary search tree
    // (constraints will be enforced via rules in Week 2)
    let mut g = Graph::new(GraphType::Directed);

    // Manually build BST structure
    g.add_node("5", Value::Number(5.0));
    g.add_node("3", Value::Number(3.0));
    g.add_node("7", Value::Number(7.0));
    g.add_edge("5", "3", "left", HashMap::new());
    g.add_edge("5", "7", "right", HashMap::new());

    // Test traversal methods work on any graph
    assert!(g.contains(&Value::Number(5.0)));
    let values = g.in_order("5");  // Start from root
    assert_eq!(values, vec![Value::Number(3.0), Value::Number(5.0), Value::Number(7.0)]);
}

#[test]
fn test_insert_convenience_method() {
    let mut g = Graph::new(GraphType::Directed);

    // insert() is a convenience method for tree-like insertion
    // It doesn't enforce BST ordering yet (rules do that in Week 2)
    g.insert(Value::Number(5.0), None);  // Root
    g.insert(Value::Number(3.0), Some("5"));  // Parent: 5
    g.insert(Value::Number(7.0), Some("5"));  // Parent: 5

    assert_eq!(g.node_count(), 3);
    assert!(g.has_edge("5", "3"));
    assert!(g.has_edge("5", "7"));
}

#[test]
fn test_bfs_dfs_traversal() {
    let mut g = Graph::new(GraphType::Directed);
    g.add_node("A", Value::String("A".to_string()));
    g.add_node("B", Value::String("B".to_string()));
    g.add_node("C", Value::String("C".to_string()));
    g.add_edge("A", "B", "edge", HashMap::new());
    g.add_edge("A", "C", "edge", HashMap::new());

    let bfs = g.bfs("A");
    assert_eq!(bfs, vec!["A", "B", "C"]); // BFS order

    let dfs = g.dfs("A");
    // DFS order depends on edge order, just verify all nodes visited
    assert_eq!(dfs.len(), 3);
}
```

**Acceptance Criteria (Week 1)**:
- ✅ Basic graph operations work correctly
- ✅ Index-free adjacency provides O(1) neighbor lookups
- ✅ Traversal methods (BFS, DFS, in-order, etc.) work on graphs
- ✅ insert() convenience method works for tree-like structures
- ✅ No separate Tree type exists - only Graph
- ✅ 20+ tests passing
- ✅ Zero compiler warnings

---

### Week 2 (Days 7-12): Rules & Automatic Optimization

**Goal**: Rule validation + query pattern detection + auto-indexing + tree{} syntax

**PHILOSOPHY IMPLEMENTATION**: This week implements the core philosophy that **trees are graphs with rules**. The `tree{}` syntax is syntactic sugar for `graph{}.with_ruleset(:tree)`.

#### Tasks

1. **Rule System** (`src/graph/rules.rs`)
   - Rule declaration: `graph.add_rule("no_cycles")`
   - Built-in rules: `no_cycles`, `connected`, `max_degree`, `single_root`, `binary_tree`
   - Ruleset support: `graph.with_ruleset(:tree)` applies multiple rules
   - Built-in rulesets: `:tree` (no_cycles, single_root, connected)
   - Rule validation on mutations
   - Rule metadata for optimizer
   - **Rule Severity System**:
     - `RuleSeverity` enum: `Silent`, `Warning` (default), `Error`
     - All violations are REJECTED (operation prevented)
     - Severity controls notification only, not enforcement
     - Syntax: `graph.add_rule(:no_cycles, :error_on_violation)` for strict mode
     - Default is `Warning` (user-friendly: reject + log)
   - **Retroactive Cleaning Policies**:
     - When adding a rule to existing data, handle existing violations
     - `RetroactivePolicy::Clean` (default) - Try to fix violations automatically
     - `RetroactivePolicy::Warn` - Keep data, warn about violations
     - `RetroactivePolicy::Enforce` - Error if violations exist
     - `RetroactivePolicy::Ignore` - Don't check existing data
     - If cleaning fails, reject the `add_rule()` operation itself
   - **Cleaning Configuration Modes** (implemented in Phase 11 via configure blocks):
     - `:strict` mode (default) - If automatic cleaning fails, reject `add_rule()` with warning
     - `:any` mode - For ambiguous cleanings (e.g., which edge to remove from cycle), pick arbitrary solution (e.g., first edge encountered)
     - `:ask` mode (future) - Interactive mode prompting user for cleaning decisions
     - Configure syntax: `configure { rule_cleaning_mode: :any } { graph.add_rule(:no_cycles) }`
     - Mode affects behavior when `clean()` method has multiple valid solutions

2. **Tree Syntax Sugar** (parser/AST)
   - Parse `tree{}` as syntactic sugar for `graph{}.with_ruleset(:tree)`
   - Parse `tree<num>{}` as `graph<num>{}.with_ruleset(:tree)`
   - Desugar during parsing - no separate Tree type in runtime

3. **Query Pattern Detection** (`src/graph/optimizer.rs`)
   - Track access patterns (property lookups, traversals)
   - Frequency counters for operations
   - Automatic index creation threshold (e.g., after 10 lookups on same property)

4. **Automatic Index Creation**
   - Property indices (hash-based): Fast `find_node(property: value)` lookups
   - Edge type indices: Fast traversal by edge type
   - Degree indices: Quick degree lookups for hub detection
   - Transparent to user - indices created/destroyed automatically

5. **Rule-Aware Algorithm Selection**
   - `no_cycles` → Use topological algorithms, skip cycle detection
   - `max_degree` → Choose algorithms with better complexity bounds
   - `connected` → Skip component detection
   - Automatic selection based on declared rules

#### Example: Auto-Optimization

```graphoid
# User code - no optimization hints needed!
social = graph { type: :directed }

# First few lookups are slower (no index yet)
social.find_node(user_id: 42)   # ~100μs
social.find_node(user_id: 99)   # ~100μs  
social.find_node(user_id: 17)   # ~100μs

# After pattern detected, index auto-created
social.find_node(user_id: 55)   # ~1μs (100x faster!)

# Check what happened:
social.stats()
# => {
#   auto_indices: ["user_id"],
#   rationale: "Frequent property lookups (10+ times)"
# }
```

#### Tests (Week 2)

```rust
#[test]
fn test_rule_validation() {
    let mut dag = Graph::new(GraphType::Directed);
    dag.add_rule("no_cycles").unwrap();
    
    dag.add_edge("A", "B", "depends_on");
    dag.add_edge("B", "C", "depends_on");
    
    // This should fail - creates cycle
    let result = dag.add_edge("C", "A", "depends_on");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("no_cycles"));
}

#[test]
fn test_auto_index_creation() {
    let mut g = create_large_graph_with_properties(10000);
    
    // Trigger auto-indexing with repeated lookups
    for _ in 0..10 {
        g.find_node_by_property("email", "test@example.com");
    }
    
    // Verify index was created
    assert!(g.has_auto_index("email"));
    
    // Verify performance improvement
    let start = Instant::now();
    g.find_node_by_property("email", "another@example.com");
    let elapsed = start.elapsed();
    
    assert!(elapsed < Duration::from_micros(10)); // Fast with index
}

#[test]
fn test_rule_aware_shortest_path() {
    let mut dag = Graph::new(GraphType::Directed);
    dag.add_rule("no_cycles").unwrap();

    // Add nodes forming a DAG
    dag.add_edge("A", "B", "edge");
    dag.add_edge("A", "C", "edge");
    dag.add_edge("B", "D", "edge");
    dag.add_edge("C", "D", "edge");

    // Shortest path should use topological ordering, not general BFS
    let path = dag.shortest_path("A", "D");
    assert_eq!(path.algorithm_used(), "topological_shortest_path");
}

#[test]
fn test_tree_syntax_is_graph_with_rules() {
    // This test shows that tree{} creates a graph with the :tree ruleset
    // In actual Graphoid code: my_tree = tree{}
    // Parser desugars to: my_tree = graph{}.with_ruleset(:tree)

    let mut tree = Graph::new(GraphType::Directed);
    tree.with_ruleset(Ruleset::Tree).unwrap();  // Applies no_cycles, single_root, connected

    // Build a valid tree structure
    tree.insert(Value::Number(5.0), None);  // Root
    tree.insert(Value::Number(3.0), Some("5"));
    tree.insert(Value::Number(7.0), Some("5"));

    // Verify tree rules are enforced
    assert!(tree.has_ruleset(Ruleset::Tree));
    assert_eq!(tree.count_roots(), 1);  // single_root enforced

    // Try to violate tree structure - should fail
    tree.add_node("orphan", Value::Number(10.0));
    let result = tree.add_edge("7", "3", "edge", HashMap::new());  // Would create cycle
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("no_cycles"));
}

#[test]
fn test_bst_insertion_with_rules() {
    // Demonstrate that BST behavior comes from rules/behaviors
    // (This might be implemented via behaviors in Phase 7, but we show the structure)
    let mut tree = Graph::new(GraphType::Directed);
    tree.with_ruleset(Ruleset::Tree).unwrap();

    // For now, manual BST insertion (automatic BST ordering via behaviors in Phase 7)
    tree.insert(Value::Number(5.0), None);
    tree.insert(Value::Number(3.0), Some("5"));
    tree.insert(Value::Number(7.0), Some("5"));
    tree.insert(Value::Number(1.0), Some("3"));
    tree.insert(Value::Number(9.0), Some("7"));

    // Tree rules ensure no cycles, single root
    assert_eq!(tree.count_roots(), 1);

    // Traversal methods work on this tree-structured graph
    let in_order = tree.in_order("5");
    assert_eq!(in_order, vec![
        Value::Number(1.0), Value::Number(3.0), Value::Number(5.0),
        Value::Number(7.0), Value::Number(9.0)
    ]);
}
```

**Acceptance Criteria (Week 2)**:
- ✅ Rules validate on every mutation
- ✅ Ruleset support works (`:tree`, `:dag`, etc.)
- ✅ tree{} syntax desugars to graph{}.with_ruleset(:tree) in parser
- ✅ Auto-indexing triggers after threshold (default: 10 lookups)
- ✅ Rule-aware algorithms automatically selected
- ✅ Performance measurably improves with auto-indices
- ✅ 25+ tests passing (including tree syntax tests)
- ✅ Zero compiler warnings

---

### Week 3 (Days 13-18): Explain, Stats & Performance Verification

**Goal**: Transparency and performance verification

#### Tasks

1. **Explain Functionality** (`src/graph/explain.rs`)
   - `graph.explain { operation }` shows execution plan
   - Human-readable plan output
   - Cost estimates
   - Shows rule optimizations applied

2. **Statistics Tracking** (`src/graph/stats.rs`)
   - Node/edge counts
   - Degree distribution stats
   - Index metadata
   - Query performance metrics

3. **Performance Benchmarks**
   - Baseline: Operations without optimization
   - Verify: Auto-optimization provides measurable speedup
   - Benchmark: Large graphs (10k+ nodes)

#### Example: Explain

```graphoid
dag = graph { rules: ["no_cycles"] }
# ... add nodes/edges ...

plan = dag.explain { dag.shortest_path("A", "B") }
plan.show()

# Output:
# Execution Plan:
#   1. Topological sort (uses rule: no_cycles)
#   2. BFS from A
#   3. Path reconstruction
# Estimated cost: 50 operations
# Rules applied: ["no_cycles" → "enabled topological algorithms"]
```

#### Tests (Week 3)

```rust
#[test]
fn test_explain_shows_plan() {
    let mut g = Graph::new(GraphType::Directed);
    g.add_rule("no_cycles").unwrap();
    
    let plan = g.explain_shortest_path("A", "B");
    
    assert!(plan.to_string().contains("topological"));
    assert!(plan.to_string().contains("uses rule: no_cycles"));
    assert!(plan.shows_estimated_cost());
}

#[test]
fn test_stats_tracking() {
    let mut g = Graph::new(GraphType::Directed);
    // Add nodes, trigger auto-indexing...
    
    let stats = g.stats();
    assert_eq!(stats.node_count, 100);
    assert_eq!(stats.edge_count, 250);
    assert!(stats.auto_optimizations.contains(&"user_id indexed".to_string()));
}

#[test]
fn test_performance_improvement_measurable() {
    let mut g = create_large_graph(10000);
    
    // Measure without index
    let mut times_before = vec![];
    for _ in 0..5 {
        let start = Instant::now();
        g.find_node_by_property("category", "A");
        times_before.push(start.elapsed());
    }
    
    // Trigger auto-indexing
    for _ in 0..10 {
        g.find_node_by_property("category", "A");
    }
    
    // Measure with index
    let mut times_after = vec![];
    for _ in 0..5 {
        let start = Instant::now();
        g.find_node_by_property("category", "A");
        times_after.push(start.elapsed());
    }
    
    let avg_before: Duration = times_before.iter().sum::<Duration>() / times_before.len() as u32;
    let avg_after: Duration = times_after.iter().sum::<Duration>() / times_after.len() as u32;
    
    // At least 5x improvement
    assert!(avg_after < avg_before / 5);
}
```

**Acceptance Criteria (Week 3)**:
- ✅ Explain shows human-readable execution plans
- ✅ Stats provide insight into graph structure and optimizations
- ✅ Benchmarks prove auto-optimization works (5x+ speedup)
- ✅ All Phase 6 tests passing (50+ total)
- ✅ Zero compiler warnings

---

### Complete Phase 6 Acceptance Criteria

**Functionality**:
```graphoid
# 1. Graphs work correctly
g = graph { type: :directed }
g.add_node("alice", {age: 30})
g.add_edge("alice", "bob", "follows")
assert g.nodes().size() == 2

# 2. Auto-optimization happens transparently
10.times { g.find_node(user_id: rand()) }
# Now fast for user_id lookups (auto-indexed)

# 3. Rules enable automatic optimization
dag = graph { rules: ["no_cycles"] }
# Shortest path automatically uses DAG algorithm
dag.shortest_path("A", "B")

# 4. Explain provides transparency
plan = g.explain { g.path_between("alice", "bob") }
print(plan)  # Shows BFS, cost estimate, etc.

# 5. Performance is measurably good
large_graph = create_graph(nodes: 10000, edges: 50000)
time = measure { large_graph.path_between("A", "B") }
assert time < 0.1  # Fast enough for real use
```

**Testing**: 50+ tests covering:
- Basic graph operations
- Tree operations
- Rule validation
- Auto-indexing
- Rule-aware algorithms
- Explain functionality
- Performance benchmarks

**Performance**:
- Auto-indexing provides 5x+ speedup on repeated property lookups
- Index-free adjacency: O(1) neighbor lookups
- Large graphs (10k nodes) perform well
- Rule-aware algorithms show measurable improvement

**Documentation**:
- API docs for all graph methods
- Examples of auto-optimization
- Rule catalog
- Performance characteristics

---

### What's NOT in Phase 6 (Deferred to v2.0+)

The following features are explicitly deferred to keep Phase 6 focused and achievable:

❌ **Manual index management**
```graphoid
# NOT IN v1.0
g.create_index(:by_property, "email")
g.drop_index("user_id")
```

❌ **Query hints**
```graphoid
# NOT IN v1.0
g.hint(:prefer_bfs)
g.hint(:parallelize, threshold: 500)
```

❌ **Manual optimization controls**
```graphoid
# NOT IN v1.0
g.optimize_for(:writes)  # vs :reads
g.cache_paths(["A", "B"], ttl: 3600)
```

❌ **Advanced explain with optimization suggestions**
```graphoid
# NOT IN v1.0
plan = g.explain(:verbose) { ... }
plan.suggest_optimizations()
```

❌ **Query plan manipulation**
```graphoid
# NOT IN v1.0
plan = g.build_query_plan { ... }
plan.force_index("user_id")
plan.execute()
```

See `dev_docs/FUTURE_FEATURES.md` for complete v2.0+ roadmap.

---

### Files to Create/Modify

**New files**:
- `src/values/graph.rs` - Graph value type (includes all graph/tree functionality)
- `src/graph/mod.rs` - Graph module root
- `src/graph/rules.rs` - Rule system (includes ruleset support)
- `src/graph/optimizer.rs` - Auto-optimization
- `src/graph/explain.rs` - Query explanation
- `src/graph/stats.rs` - Statistics tracking
- `tests/unit/graph_tests.rs` - Graph unit tests
- `tests/integration/graph_performance_tests.rs` - Performance tests

**Modified files**:
- `src/values/mod.rs` - Add Graph variant to Value enum (NO separate Tree variant!)
- `src/parser/mod.rs` - Add tree{} syntax desugaring to graph{}.with_ruleset(:tree)
- `src/lib.rs` - Add graph module

**IMPORTANT**: No `src/values/tree.rs` file! Trees are graphs with rules, not a separate type.

---

### Success Metrics

**Code Quality**:
- Zero compiler warnings
- 80%+ test coverage
- Idiomatic Rust

**Functionality**:
- All graph operations work correctly
- Auto-optimization triggers appropriately
- Rules validate correctly
- Explain shows useful information

**Performance**:
- Auto-indexing provides 5x+ speedup
- Large graphs (10k nodes) performant
- Index-free adjacency O(1)
- Rule-aware algorithms show improvement

**User Experience**:
- "It just works" - no manual tuning needed
- Transparent optimization
- Clear error messages when rules violated

---

## Phase 6.5: Foundational Gaps & Verification (5-7 days)

**📋 Verification Report**: See [`dev_docs/archive/phase_reports/PHASE_6_5_VERIFICATION_REPORT.md`](archive/phase_reports/PHASE_6_5_VERIFICATION_REPORT.md) for detailed verification results of all architectural assumptions.

**Duration**: 5-7 days
**Goal**: Complete all missing foundational features from Phases 0-6 before proceeding to Phase 7
**Philosophy**: "Solid foundations enable rapid progress" - Fix gaps now to avoid technical debt

**CRITICAL**: Phase 7+ cannot begin until Phase 6.5 is complete. The behavior system, module system, and all subsequent phases depend on these foundational features being fully implemented.

### Overview

Gap analysis revealed several critical features specified in the language spec that are missing from Phases 0-6 implementation:

1. **Parser completeness gaps** - Tokens exist but parsing/execution incomplete
2. **Mutation operator convention** - Fundamental design pattern not implemented
3. **Missing collection methods** - Spec-required methods absent
4. **Graph querying levels** - Only partial implementation (Level 1 + partial Level 4)
5. **Verification tasks** - Assumptions not validated

This phase systematically addresses all gaps to create a solid foundation for Phase 7+.

---

### Area 1: Verification & Validation (Day 1)

**Goal**: Verify that existing implementations match architectural assumptions

#### Tasks

1. **Verify Graph-Backed Collections** (`tests/integration/architecture_verification.rs`)
   - Write tests confirming `List` uses graph internally (src/values/list.rs)
   - Write tests confirming `Hash` uses graph internally (src/values/hash.rs)
   - Verify graph operations accessible through collection methods
   - Document actual architecture vs assumed architecture

2. **Verify Tree Syntax Sugar** (`tests/unit/parser_tests.rs`)
   - Test that `tree{}` parses to `graph{}.with_ruleset(:tree)`
   - Test that `tree<num>{}` parses to `graph<num>{}.with_ruleset(:tree)`
   - Verify no separate Tree type exists in runtime
   - Test tree operations work via graph operations

3. **Verify Graph Indexing** (`tests/unit/graph_tests.rs`)
   - Test `graph["node_id"]` returns node value
   - Test `graph["node_id"] = value` adds/updates node
   - Verify consistency with hash/list indexing syntax
   - Test error handling for missing nodes

4. **Verify NO_GENERICS_POLICY Enforcement** (`tests/unit/parser_tests.rs`)
   - Test parser rejects `hash<string, num>` (multiple params)
   - Test parser rejects `fn process<T>(x: T)` (generic functions)
   - Test parser rejects `list<list<num>>` (nested constraints)
   - Test parser accepts `list<num>`, `hash<string>` (single param)
   - Verify error messages reference NO_GENERICS_POLICY.md

5. **Document Verification Results** (`dev_docs/PHASE_6_5_VERIFICATION_REPORT.md`)
   - Create report of all verification findings
   - List confirmed behaviors
   - List discrepancies found
   - Recommendations for fixes

#### Tests (Area 1)

Minimum **15 tests**:
- 3 tests for graph-backed lists
- 3 tests for graph-backed hashes
- 3 tests for tree syntax sugar
- 3 tests for graph indexing
- 3 tests for NO_GENERICS_POLICY enforcement

#### Acceptance Criteria (Area 1)

- ✅ All architectural assumptions verified or corrected
- ✅ Verification report documents findings
- ✅ Any discrepancies have filed issues or immediate fixes
- ✅ 15+ verification tests passing
- ✅ Zero compiler warnings

---

### Area 2: Parser Completeness (Days 2-3)

**Goal**: Complete parser/executor support for tokens that lexer already recognizes

#### Tasks

1. **Inline Conditionals** (`src/parser/expression.rs`, `src/execution/executor.rs`)

   **Syntax to support**:
   ```graphoid
   # if-then-else expression
   status = if age >= 18 then "adult" else "minor"

   # Suffix if (evaluates when true, otherwise none)
   value = compute() if condition
   result = x.round(2) if x > threshold

   # Suffix unless (evaluates when false, otherwise none)
   message = "OK" unless error_occurred
   status = "active" unless paused
   ```

   **Implementation**:
   - Add `parse_inline_conditional()` to parser
   - Add `IfThenElse` AST node with condition, then_expr, else_expr
   - Add `SuffixIf` and `SuffixUnless` AST nodes
   - Implement execution in executor
   - Handle none return values for suffix forms

   **Tests**: 10+ tests covering:
   - Simple if-then-else
   - Nested inline conditionals
   - Suffix if with various expressions
   - Suffix unless with various expressions
   - Edge cases (none values, type mismatches)

2. **Element-Wise Operations** (`src/parser/expression.rs`, `src/execution/executor.rs`)

   **Syntax to support**:
   ```graphoid
   # Scalar operations
   [1, 2, 3] .* 2        # [2, 4, 6]
   [1, 2, 3] .+ 10       # [11, 12, 13]
   [10, 20, 30] ./ 5     # [2, 4, 6]

   # Vector operations (zip)
   [1, 2, 3] .+ [4, 5, 6]    # [5, 7, 9]
   [2, 3, 4] .^ [2, 2, 2]    # [4, 9, 16]

   # Comparisons (returns list of bools)
   [10, 20, 30] .> 15        # [false, true, true]
   ```

   **Implementation**:
   - Parser already recognizes `.+`, `.*`, `./`, `.//`, `.%`, `.^` tokens
   - Add `parse_element_wise_operation()` handling
   - Add `ElementWiseOp` AST node with operator, left, right
   - Implement in executor:
     - Scalar case: Apply op to each element
     - Vector case: Zip lists, apply pairwise
     - Handle length mismatch (use shorter length)
   - Return new list (immutable operation)

   **Tests**: 15+ tests covering:
   - All operators with scalars
   - All operators with vectors
   - Comparison operators
   - Length mismatches
   - Type errors
   - Chaining element-wise ops

3. **Integer Division** (`src/parser/expression.rs`, `src/execution/executor.rs`)

   **Syntax to support**:
   ```graphoid
   10 // 3      # 3 (not 3.333...)
   -10 // 3     # -3 (truncates toward zero)
   10.5 // 2    # 5 (works on floats)
   ```

   **Implementation**:
   - Parser already recognizes `//` token
   - Add handling in `parse_binary_op()`
   - Implement in executor: `(a / b).trunc()` (truncate toward zero)
   - Ensure precedence matches `/` operator

   **Tests**: 5+ tests covering:
   - Positive integers
   - Negative integers
   - Float operands
   - Edge cases (division by zero, very large numbers)

4. **Optional Parentheses for Zero-Arg Methods** (`src/parser/expression.rs`)

   **Syntax to support**:
   ```graphoid
   # Both should work
   count = items.size()
   count = items.size

   # But multi-arg always needs parens
   items.insert(0, value)    # OK
   items.insert 0, value     # ERROR
   ```

   **Implementation**:
   - When parsing method call, check if `(` follows
   - If no `(` and method is known zero-arg, treat as call with no args
   - Requires method signature lookup or runtime checking
   - May defer to Phase 8 if too complex (not critical)

   **Tests**: 5+ tests (if implemented in this phase)

#### Acceptance Criteria (Area 2)

- ✅ Inline conditionals fully functional (if-then-else, suffix if/unless)
- ✅ Element-wise operators work for all arithmetic/comparison ops
- ✅ Integer division (`//`) returns integers, truncates toward zero
- ✅ 30+ parser/executor tests passing
- ✅ All tests work in both REPL and CLI modes (parity)
- ✅ Zero compiler warnings

---

### Area 3: Mutation Operator Convention (Days 4-5)

**Goal**: Implement dual-version methods (immutable and mutating) across all collections

**CRITICAL DESIGN PATTERN**: This is a fundamental language design principle. Every transformative method must have two versions:
- **Immutable** (no suffix): Returns new collection, original unchanged
- **Mutating** (`!` suffix): Modifies in place, returns none

#### Tasks

1. **List Methods** (`src/values/list.rs`)

   **Current state**: Only single versions exist (behavior varies)

   **Implement dual versions**:
   - `sort()` / `sort!()` - Sort elements
   - `reverse()` / `reverse!()` - Reverse order
   - `uniq()` / `uniq!()` - Remove duplicates
   - `map(transform)` / `map!(transform)` - Transform elements
   - `filter(predicate)` / `filter!(predicate)` - Keep matching
   - `select(predicate)` / `select!(predicate)` - Alias for filter
   - `reject(predicate)` / `reject!(predicate)` - Remove matching
   - `compact()` / `compact!()` - Remove none values

   **Implementation**:
   - Immutable: Clone graph, apply transformation, return new List
   - Mutating: Transform graph in place, return Value::None
   - Ensure consistent behavior across all methods

   **Tests**: 24 tests minimum (8 methods × 3 test cases each)

2. **Hash Methods** (`src/values/hash.rs`)

   **Note**: Hashes have fewer transformative methods (mostly mutation-by-nature)

   **Implement dual versions** (if applicable):
   - `freeze()` / `freeze!()` - Make immutable (deferred to freeze implementation)

   **Tests**: Defer until freeze system implemented

3. **Graph Methods** (`src/values/graph.rs`)

   **Implement dual versions**:
   - `freeze()` / `freeze!()` - Make immutable (deferred to freeze implementation)

   **Tests**: Defer until freeze system implemented

4. **Update Method Dispatch** (`src/execution/executor.rs`)
   - Recognize `!` suffix in method names
   - Route to correct implementation
   - Ensure error messages distinguish between versions

5. **Document Convention** (`rust/MUTATION_OPERATOR_CONVENTION.md`)
   - Explain immutable vs mutating pattern
   - List all dual-version methods
   - Provide examples
   - Guidelines for adding new methods

#### Tests (Area 3)

Minimum **30 tests**:
- 24 tests for list methods (8 methods × 3 cases)
- 3 tests for method dispatch with `!`
- 3 tests for error messages

#### Acceptance Criteria (Area 3)

- ✅ All list transformative methods have both versions
- ✅ Immutable methods return new collection, original unchanged
- ✅ Mutating methods modify in place, return none
- ✅ Naming convention consistent (`method` / `method!`)
- ✅ Method dispatch handles `!` suffix correctly
- ✅ 30+ tests passing
- ✅ Convention documented
- ✅ Zero compiler warnings

---

### Area 4: Missing Collection Methods (Day 6)

**Goal**: Implement spec-required methods missing from collections

#### Tasks

1. **List Slicing with Step** (`src/values/list.rs`)
   ```graphoid
   items.slice(start, end, step)
   [1,2,3,4,5,6].slice(0, 6, 2)  # [1, 3, 5]
   ```

   **Tests**: 5 tests

2. **List Generators** (`src/values/list.rs`)
   ```graphoid
   # Range mode
   list.generate(1, 10, 2)       # [1, 3, 5, 7, 9]

   # Function mode
   list.generate(1, 5, x => x * x)  # [1, 4, 9, 16, 25]

   # upto helper
   list.upto(5)                  # [0, 1, 2, 3, 4, 5]
   ```

   **Tests**: 8 tests

3. **Additional Predicates** (`src/values/list.rs`)

   Ensure all spec predicates are implemented:
   - `:frozen` / `:unfrozen` (deferred to freeze system)
   - Verify existing: `:positive`, `:negative`, `:even`, `:odd`, `:empty`, `:non_empty`, etc.

   **Tests**: 3 tests for any missing predicates

4. **Additional Transformations** (`src/values/list.rs`)

   Ensure all spec transformations are implemented:
   - Verify existing: `:double`, `:square`, `:negate`, `:increment`, `:decrement`, etc.

   **Tests**: 3 tests for any missing transformations

#### Acceptance Criteria (Area 4)

- ✅ slice() supports step parameter
- ✅ generate() works in range and function modes
- ✅ upto() helper implemented
- ✅ All spec predicates available
- ✅ All spec transformations available
- ✅ 15+ tests passing
- ✅ Zero compiler warnings

---

### Area 5: Enhanced Graph Querying (Day 7)

**Goal**: Complete missing levels of graph querying system

**Current State**: Level 1 (basic navigation) + partial Level 4 (shortest_path only)

**Note**: Full 5-level querying is a large feature. This area implements **critical gaps** only. Full implementation may require dedicated phase.

#### Tasks

1. **Level 4: Additional Path Algorithms** (`src/values/graph.rs`)

   **Currently have**: `shortest_path(from, to)`, `topological_sort()`

   **Implement critical missing algorithms**:
   ```graphoid
   graph.has_path(from, to)              # Boolean path existence
   graph.distance(from, to)              # Shortest path length
   graph.all_paths(from, to, max_len)    # All paths up to max length
   ```

   **Defer to later** (not critical for Phase 7):
   - `dijkstra()` (weighted shortest path)
   - `nodes_within()` (N-hop neighbors)

   **Tests**: 12 tests (4 per new method)

2. **Level 2: Method Chaining on Results** (Defer to Phase 7/11)

   **Rationale**: Requires returning graph node collections with chainable methods. Complex feature, not blocking for Phase 7.

   **Status**: Document as deferred feature

3. **Level 3: Pattern-Based Querying** (Defer to Phase 11)

   **Rationale**: Requires pattern matching DSL, parser extensions. Major feature deserving dedicated phase.

   **Status**: Document as deferred feature

4. **Level 5: Subgraph Operations** (Defer to Phase 11)

   **Rationale**: Complex graph manipulation, not needed for behavior system.

   **Status**: Document as deferred feature

#### Acceptance Criteria (Area 5)

- ✅ has_path() implemented and tested
- ✅ distance() implemented and tested
- ✅ all_paths() implemented and tested
- ✅ 12+ tests passing
- ✅ Deferred features documented in roadmap
- ✅ Zero compiler warnings

---

### Complete Phase 6.5 Acceptance Criteria

**Verification**:
- ✅ All architectural assumptions validated
- ✅ Verification report completed
- ✅ 15+ verification tests passing

**Parser Completeness**:
- ✅ Inline conditionals functional
- ✅ Element-wise operators functional
- ✅ Integer division functional
- ✅ 30+ parser tests passing

**Mutation Convention**:
- ✅ All transformative list methods have dual versions
- ✅ Convention documented
- ✅ 30+ mutation tests passing

**Collection Methods**:
- ✅ slice(start, end, step) implemented
- ✅ generate() and upto() implemented
- ✅ All spec predicates/transformations available
- ✅ 15+ collection tests passing

**Graph Querying**:
- ✅ has_path(), distance(), all_paths() implemented
- ✅ 12+ graph algorithm tests passing

**Overall**:
- ✅ 100+ new tests passing (15+30+30+15+12 = 102)
- ✅ Total test count: ~800+ (704 current + ~100 new)
- ✅ Zero compiler warnings
- ✅ All tests pass in both REPL and CLI modes (parity maintained)
- ✅ **Quality Gate passed** - Spec conformance verified
- ✅ Ready to proceed to Phase 7

---

### Files to Create/Modify

**New files**:
- `tests/integration/architecture_verification.rs` - Verify graph-backed collections
- `dev_docs/PHASE_6_5_VERIFICATION_REPORT.md` - Document verification findings
- `rust/MUTATION_OPERATOR_CONVENTION.md` - Document dual-version pattern

**Modified files**:
- `src/parser/expression.rs` - Inline conditionals, element-wise ops, integer division
- `src/ast/nodes.rs` - New AST nodes for inline conditionals
- `src/execution/executor.rs` - Execute new syntax features, method dispatch
- `src/values/list.rs` - Dual-version methods, slice with step, generators
- `src/values/graph.rs` - Additional path algorithms
- `tests/unit/parser_tests.rs` - Parser verification and new syntax tests
- `tests/unit/executor_tests.rs` - Execution tests for new features
- `tests/unit/list_tests.rs` - Dual-version method tests
- `tests/unit/graph_tests.rs` - New algorithm tests

**Documentation updates**:
- `rust/START_HERE_NEXT_SESSION.md` - Update with Phase 6.5 status
- `dev_docs/RUST_IMPLEMENTATION_ROADMAP.md` - Mark Phase 6.5 complete when done

---

### Success Metrics

**Code Quality**:
- Zero compiler warnings
- 80%+ test coverage
- Idiomatic Rust
- Clear, documented code

**Functionality**:
- All spec-required Phases 0-6 features implemented
- Parser handles all declared syntax
- Collections support dual-version methods
- Graph querying at functional level

**Testing**:
- 100+ new tests passing
- All tests work in REPL and CLI modes
- No regressions in existing tests

**Documentation**:
- Verification report complete
- Mutation convention documented
- Deferred features clearly marked

**Readiness**:
- **Phase 7 can begin** with confidence
- Solid foundation for behavior system
- No known foundational gaps

---

### Why This Phase Is Critical

**Without Phase 6.5**:
- ❌ Behavior system built on incomplete foundation
- ❌ Mutation convention inconsistent across language
- ❌ Parser gaps create confusion and bugs
- ❌ Missing methods require workarounds
- ❌ Technical debt accumulates

**With Phase 6.5**:
- ✅ Solid foundation for all future phases
- ✅ Consistent design patterns throughout
- ✅ Complete parser coverage
- ✅ Spec compliance for Phases 0-6
- ✅ Confidence in architecture

**Phase 7 (Function Pattern Matching) dependencies**: None - can start immediately

**Phase 8 (Behavior System) depends on**:
- Mutation convention (behaviors may trigger transforms)
- Complete collection methods (behaviors attach to methods)
- Parser completeness (behavior syntax)
- Verified architecture (behaviors integrate with graphs)

**Phase 9 (Graph Pattern Matching) depends on**:
- Function pattern matching (Phase 7) - shares parser/engine
- Behavior system (Phase 8) - for result transformation

**Timeline Impact**: 5-7 days investment now saves weeks of refactoring later.

---

### Quality Gate: Spec Conformance Check

**CRITICAL**: Before proceeding to Phase 7, perform a comprehensive conformance check.

#### Tasks

1. **Re-Analyze Language Specification** (`dev_docs/PHASE_6_5_CONFORMANCE_REPORT.md`)
   - Re-read LANGUAGE_SPECIFICATION.md thoroughly
   - Cross-reference against Phase 6.5 implementation
   - Identify any remaining gaps in Phases 0-6.5
   - Document findings with severity levels:
     - **BLOCKER**: Must fix before Phase 7
     - **CRITICAL**: Should fix before Phase 7
     - **IMPORTANT**: Can defer but document
     - **NICE-TO-HAVE**: Defer to later phase

2. **Verify All Phase 6.5 Deliverables**
   - ✅ 100+ new tests passing
   - ✅ ~800 total tests passing
   - ✅ Zero compiler warnings
   - ✅ REPL/CLI parity maintained
   - ✅ Verification report complete
   - ✅ Mutation convention documented
   - ✅ All deferred features documented

3. **Update Future Phases Based on Findings**
   - Review Phase 7 roadmap section
   - Update with any new requirements discovered
   - Add detailed specification if missing
   - Review Phases 8-14 for potential gaps
   - Document dependencies between phases

4. **Create Go/No-Go Decision**
   - **GO**: All blockers resolved, foundation solid
   - **NO-GO**: Critical gaps remain, continue Phase 6.5

#### Acceptance Criteria (Quality Gate)

- ✅ Conformance report completed and reviewed
- ✅ No BLOCKER-level gaps remaining
- ✅ All CRITICAL gaps either fixed or have mitigation plan
- ✅ Future phases updated with new requirements
- ✅ Phase 7 specification adequate for implementation
- ✅ User approval to proceed to Phase 7

#### Deliverables

**Required before Phase 7**:
1. `dev_docs/PHASE_6_5_CONFORMANCE_REPORT.md` - Comprehensive spec analysis
2. Updated roadmap sections for Phases 7+ (if needed)
3. Phase 7 detailed specification (if currently inadequate)
4. Go/No-Go decision documented in START_HERE_NEXT_SESSION.md

#### Timeline

**Duration**: 0.5-1 day
- Analysis: 2-3 hours
- Updates: 1-2 hours
- Review and approval: 1 hour

**Total Phase 6.5 Duration**: 5.5-8 days (including quality gate)

---

**Phase 6.5 is NOT complete until quality gate is passed.**

---

## Phase 7: Function Pattern Matching (5-7 days)

**📋 Detailed Implementation Plan**: See [`dev_docs/PHASE_7_DETAILED_PLAN.md`](PHASE_7_DETAILED_PLAN.md) for:
- 7-day implementation plan with daily deliverables
- 40+ test specifications covering all pattern types
- Parser extension for pipe syntax `|pattern| => result`
- Pattern matching engine (literals, variables, guards)
- Function dispatch with pattern matching
- Copy-paste ready Rust code examples
- Integration tests and acceptance criteria

**Duration**: 5-7 days
**Goal**: Implement pipe syntax pattern matching for elegant function definitions

**Overview**:
- **Pipe Syntax**: `|pattern| => result` for function pattern matching
- **Literal Patterns**: `|0| => 1`, `|"dog"| => "woof"`
- **Variable Patterns**: `|x| => x * 2`
- **Guards** (future enhancement): `|x| if x > 0 => "positive"`
- **Fallthrough**: Returns `none` if no pattern matches
- **Disambiguation**: Clear separation from lambda syntax `x => x * 2`

**Why This Phase Comes First**:
- Zero dependencies - can start immediately
- Foundation for graph pattern matching (Phase 9)
- Provides immediate value for recursive functions
- Elegant syntax for case handling

**Key Use Cases**:
```graphoid
# Recursive functions with pattern matching
fn factorial(n) {
    |0| => 1
    |1| => 1
    |x| => x * factorial(x - 1)
}

# String case handling
fn get_sound(animal) {
    |"dog"| => "woof"
    |"cat"| => "meow"
    |"cow"| => "moo"
    # Automatic fallthrough returns none
}
```

**Current Status**: Not started - new phase

---

## Phase 8: Complete Behavior System (2-3 days)

**📋 Detailed Implementation Plan**: See [`dev_docs/PHASE_8_DETAILED_PLAN.md`](PHASE_8_DETAILED_PLAN.md) for:
- 3-day implementation plan with daily deliverables
- 20-30 new test specifications (target: 95-105 total behavior tests)
- Remaining standard behaviors to implement
- Behavior rulesets completion
- Copy-paste ready Rust code examples
- Integration tests and acceptance criteria

**Duration**: 2-3 days
**Goal**: Complete the behavior transformation system (a subset of the rules system)

**Overview**:
- Finish remaining standard behaviors (none_to_zero, positive, uppercase, validate_range, etc.)
- Complete mapping behaviors (hash-based value transformations)
- Finalize custom function behaviors (user-defined transformations)
- Complete conditional behaviors (context-aware with predicates)
- Finalize rulesets (bundled behavior collections for reusability)

**Why This Phase Comes Second**:
- Already 75% done (75 tests passing)
- Completes the rule/transformation architecture
- Behaviors are "rules that transform" - architectural consistency
- Needed before graph querying for result transformation
- Low effort, high value

**Architectural Note**: As correctly identified, "behavior system is really just a subset of rules" - behaviors transform values while rules validate structure. Both use the same `RetroactivePolicy` and application system.

**Current Status**: 75 behavior tests passing, framework exists in `src/graph/behaviors.rs` (1,005 lines), ~75% complete

---

## Phase 9: Graph Pattern Matching & Advanced Querying (7-10 days)

**📋 Detailed Implementation Plan**: See [`dev_docs/PHASE_9_DETAILED_PLAN.md`](PHASE_9_DETAILED_PLAN.md) for:
- 10-day implementation plan with daily deliverables
- 60+ test specifications covering all query levels
- Explicit pattern syntax implementation
- Pattern parser and matcher for graphs
- Query execution engine
- Subgraph operations
- Copy-paste ready Rust code examples
- Integration tests and acceptance criteria

**Duration**: 7-10 days
**Goal**: Implement declarative graph pattern matching - the "make or break" feature for a graph language

**Overview**:
- **Level 3: Pattern-Based Querying** - Declarative pattern matching (concept inspired by Cypher)
  - Pattern syntax: `node("var", type: "Type"), edge(type: "EDGE"), node("other", type: "Type")`
  - Pattern parser and AST nodes
  - Pattern matching engine for graphs
  - `.where_*()` filtering methods
  - `.return_vars()` / `.return_properties()` projection for specific fields
  - Variable-length paths: `path(type: "FOLLOWS", min: 1, max: 3)`
  - Bidirectional patterns: `edge(direction: :both)`

- **Level 5: Subgraph Operations** - Extract, manipulate, and compose subgraphs
  - `graph.extract { nodes: ..., edges: ... }`
  - `graph.delete { nodes: ..., edges: ... }`
  - `graph.add_subgraph(other)`
  - Filter-based extraction

**Why This Phase Comes Third**:
- Depends on function pattern matching (Phase 7) - shares parser/engine
- Benefits from complete behavior system (Phase 8) - for result transformation
- Fills critical capability gap (Level 3 querying currently missing)
- Makes Graphoid credible as a graph-theoretic language
- "Make or break" feature per language specification

**Key Use Cases**:
```graphoid
# Level 3: Pattern-based querying
results = graph.match(
    (person:User) -[:FRIEND]-> (friend:User)
).where(person.age > 18, friend.age > 18)

# Variable-length paths
influencers = graph.match(
    (user:User) -[:FOLLOWS*1..3]-> (influencer:User)
).where(influencer.follower_count > 1000)

# Level 5: Subgraph extraction
active_users = graph.extract {
    nodes: n => n.type == "User" and n.get_attribute("active") == true
}
```

**Current Status**: Level 1, 2, 4 complete (basic navigation, filtering, path algorithms). Level 3 & 5 missing.

**Note**: Level 4 path algorithms (`shortest_path`, `has_path`, `distance`, etc.) are already implemented and working.

---

## Phase 10: Complete Module System (3-4 days)

**📋 Detailed Implementation Plan**: See [`dev_docs/PHASE_10_DETAILED_PLAN.md`](PHASE_10_DETAILED_PLAN.md) for:
- 4-day implementation plan with daily deliverables
- 30+ new test specifications (target: 60+ total module tests)
- Module loading from .gr files
- Namespace management and scoping
- Standard library imports
- graphoid.toml specification
- Copy-paste ready Rust code examples
- Integration tests and acceptance criteria

**Duration**: 3-4 days
**Goal**: Complete module system for code organization and stdlib imports

**Overview**:
- **Module Loading**: Load .gr files as modules
- **Namespace Management**: Proper scoping and symbol resolution
- **Import Variations**: Relative, project-root, stdlib paths
- **Module Declaration**: `module name`, `alias` syntax
- **Load vs Import**: Executable vs library semantics
- **Project Structure**: src/, lib/, graphoid.toml support
- **Module Search Paths**: Resolution algorithm

**Why This Phase Comes Fourth**:
- Needs to be complete before stdlib work (Phases 11-12)
- Module structure already exists (31 tests passing)
- Final prerequisite for production stdlib
- Clean separation of concerns

**Current Status**: 31 module tests passing, module manager exists in `src/execution/module_manager.rs` (250 lines), ~40% complete

**All Prerequisites Now Complete**: try/catch (✅), configure blocks (✅), precision (✅), pattern matching (Phases 7 & 9), behaviors (Phase 8), modules (Phase 10).

---

## Phase 11: Pure Graphoid Stdlib (10-14 days)

**📋 Detailed Implementation Plan**: See [`dev_docs/PHASE_11_DETAILED_PLAN.md`](PHASE_11_DETAILED_PLAN.md) for complete specifications.

Standard library modules written in .gr files:

- **Statistics** (stats) - Descriptive statistics, using config for missing data handling
- **CSV** - CSV parsing and generation with validation
- **SQL** - SQL query builder (fluent interface)
- **HTML** - HTML parsing and manipulation
- **HTTP** - HTTP client for RESTful APIs
- **Pretty-Print** (pp) - Formatted output for debugging and display
- **Option Parser** (optparse) - Command-line argument parsing

**Why Pure Graphoid**: These modules benefit from pattern matching, behaviors, and high-level abstractions. Implemented in .gr to dogfood the language.

---

## Phase 12: Native Stdlib Modules (14-21 days) - ✅ 44% COMPLETE

**📋 Detailed Implementation Plan**: See [`dev_docs/PHASE_12_DETAILED_PLAN.md`](PHASE_12_DETAILED_PLAN.md) for complete specifications.

**Current Status (November 12, 2025)**: 4 of 9 modules implemented (44%)

### Completed Modules

- ✅ **Constants** - Mathematical and physical constants (PI, E, GOLDEN_RATIO, etc.)
- ✅ **Random** (rand) - Cryptographically secure random number generation (native Rust)
- ✅ **OS** - Operating system interface (7 functions: system_timestamp, env, env_all, getcwd, platform, arch, args)
- ✅ **Time** - Date/time handling (PURE GRAPHOID implementation in `stdlib/time.gr`)
  - Calendar-aware arithmetic (add_months, add_years with edge case handling)
  - Predicates (is_weekday, is_weekend, is_leap_year)
  - Navigation (start_of_week, end_of_month, skip_weekends)
  - Only requires `os.system_timestamp()` primitive - everything else is pure computation!

### Remaining Modules

- 🔲 **Regex** (re) - Regular expression engine
- 🔲 **I/O** (io) - File operations (read, write, append, delete)
- 🔲 **JSON** - JSON parsing and serialization
- 🔲 **YAML** - YAML parsing and serialization
- 🔲 **Crypto** - Cryptographic primitives (hashing, encryption)

**Why Native Rust**: These modules require system calls, performance, or complex algorithms best implemented in Rust.

**Important Discovery**: Time module successfully implemented in PURE GRAPHOID, demonstrating self-hosting capability. Only needed one system primitive (`os.system_timestamp()`). This validates the 90%+ self-hosting goal.

**Status**: These are TEMPORARY Rust implementations. Phase 14 will translate most modules to pure Graphoid for self-hosting (90%+ goal).

---

## Phase 13: Bitwise Operators & Integer Types (5-7 days)

**Goal**: Add bitwise operations and integer types to enable stdlib translation to pure Graphoid.

**Critical for Self-Hosting**: Without bitwise operators, cryptographic algorithms, random number generators, UUIDs, hashing, and compression cannot be implemented in pure Graphoid.

### Operator Changes

**Change Power Operator**:
```graphoid
# Old (Phase 0-12): ^ means power
x = 2 ^ 8  # 256

# New (Phase 13+): ** means power, ^ means XOR
x = 2 ** 8       # Power: 256
y = 0b1010 ^ 0b0101  # XOR: 15
```

**New Bitwise Operators**:
- `&` - Bitwise AND
- `|` - Bitwise OR
- `^` - Bitwise XOR (operator conflict resolved!)
- `~` - Bitwise NOT (complement)
- `<<` - Left shift
- `>>` - Right shift

**Integer Types**:
- Add `int` type for 64-bit signed integers
- Add `uint` type for 64-bit unsigned integers
- Support binary literals: `0b1010`, `0b11111111`
- Support hex literals: `0xFF`, `0xDEADBEEF`

### Implementation Steps

**Day 1-2: Lexer Updates** (src/lexer/mod.rs)
```rust
// Add new tokens
pub enum TokenType {
    // ... existing tokens

    // Change: Power now uses **
    DoubleStar,      // **

    // Bitwise operators
    Ampersand,       // &
    Pipe,            // |
    Caret,           // ^ (now XOR, not power!)
    Tilde,           // ~
    LeftShift,       // <<
    RightShift,      // >>

    // Integer type keywords
    Int,             // int
    UInt,            // uint
}

// Update tokenization for number literals
fn tokenize_number(&mut self) -> Token {
    // Support 0b prefix for binary
    // Support 0x prefix for hex
    // Default to int if no decimal point
}
```

**Tests** (15+ tests):
- Binary literal parsing: `0b1010`, `0b11111111`
- Hex literal parsing: `0xFF`, `0xDEADBEEF`
- Operator tokenization: `&`, `|`, `^`, `~`, `<<`, `>>`
- Power operator: `**`

**Day 3-4: Parser Updates** (src/parser/mod.rs)

Add operator precedence (lower number = lower precedence):
```rust
fn precedence(&self, op: &TokenType) -> u8 {
    match op {
        TokenType::Pipe => 30,           // | (bitwise OR)
        TokenType::Caret => 35,          // ^ (bitwise XOR)
        TokenType::Ampersand => 40,      // & (bitwise AND)
        TokenType::LeftShift => 50,      // <<
        TokenType::RightShift => 50,     // >>
        TokenType::DoubleStar => 90,     // ** (power)
        // ... rest of operators
    }
}
```

**Tests** (20+ tests):
- Precedence: `2 + 3 & 4` vs `(2 + 3) & 4`
- Associativity: `16 >> 2 >> 1` = `(16 >> 2) >> 1` = 2
- Power operator: `2 ** 3 ** 2` = `2 ** (3 ** 2)` = 512 (right-associative)
- Complex expressions: `(a & 0xFF) | (b << 8)`

**Day 5-6: Executor Implementation** (src/execution/executor.rs, src/values/mod.rs)

Add Value variants:
```rust
pub enum ValueKind {
    Number(f64),          // Existing floating point
    Integer(i64),         // NEW: 64-bit signed integer
    UInteger(u64),        // NEW: 64-bit unsigned integer
    // ... rest
}
```

Implement operations:
```rust
fn eval_binary_op(&mut self, left: Value, op: TokenType, right: Value) -> Result<Value> {
    match op {
        TokenType::Ampersand => {
            // Bitwise AND - requires integer operands
            let l = left.to_integer()?;
            let r = right.to_integer()?;
            Ok(Value::integer(l & r))
        }
        TokenType::Pipe => {
            // Bitwise OR
            let l = left.to_integer()?;
            let r = right.to_integer()?;
            Ok(Value::integer(l | r))
        }
        TokenType::Caret => {
            // Bitwise XOR
            let l = left.to_integer()?;
            let r = right.to_integer()?;
            Ok(Value::integer(l ^ r))
        }
        TokenType::LeftShift => {
            // Left shift
            let l = left.to_integer()?;
            let r = right.to_integer()? as u32;
            Ok(Value::integer(l << r))
        }
        TokenType::RightShift => {
            // Right shift (arithmetic for signed, logical for unsigned)
            let l = left.to_integer()?;
            let r = right.to_integer()? as u32;
            Ok(Value::integer(l >> r))
        }
        TokenType::DoubleStar => {
            // Power operation (moved from Caret)
            Ok(Value::number(left.to_number()?.powf(right.to_number()?)))
        }
        // ... rest
    }
}

fn eval_unary_op(&mut self, op: TokenType, operand: Value) -> Result<Value> {
    match op {
        TokenType::Tilde => {
            // Bitwise NOT
            let val = operand.to_integer()?;
            Ok(Value::integer(!val))
        }
        // ... rest
    }
}
```

**Tests** (40+ tests):
- Bitwise AND: `0b1100 & 0b1010` = `0b1000` = 8
- Bitwise OR: `0b1100 | 0b1010` = `0b1110` = 14
- Bitwise XOR: `0b1100 ^ 0b1010` = `0b0110` = 6
- Bitwise NOT: `~0b1010` = `-11` (two's complement)
- Left shift: `5 << 2` = 20
- Right shift: `20 >> 2` = 5
- Rotate left: `(x << k) | (x >> (64 - k))`
- Rotate right: `(x >> k) | (x << (64 - k))`
- Power operator migration: `2 ** 8` = 256
- Type conversions: num to int, int to num

**Day 7: Integration & Examples**

Create examples demonstrating use cases:
- `examples/bitwise_basics.gr` - Basic bitwise operations
- `examples/bit_manipulation.gr` - Flags, masks, permissions
- `examples/rotate_operations.gr` - Bit rotations
- `examples/uuid_manual.gr` - Manual UUID generation using bitwise ops

**Success Criteria**:
- ✅ All bitwise operators working (`&`, `|`, `^`, `~`, `<<`, `>>`)
- ✅ Power operator migrated to `**`
- ✅ Integer types (int, uint) supported
- ✅ Binary and hex literals parsed correctly
- ✅ 75+ tests passing (15 lexer + 20 parser + 40 executor)
- ✅ Zero compiler warnings
- ✅ Example files demonstrate all operators
- ✅ Operator precedence and associativity correct

**Blocks**: Phase 14 (cannot translate RNG/crypto without bitwise ops)

---

## Phase 14: Stdlib Translation to Pure Graphoid (7-10 days)

**Goal**: Translate Rust stdlib modules to pure Graphoid, achieving 90%+ self-hosting.

**Vision**: Only lexer, parser, and core executor remain in Rust. All standard library code written in Graphoid itself.

### Translation Priority

**Immediate (Can Translate Now)**:
1. **Constants** → `stdlib/constants.gr`
   - Pure data, no algorithms
   - 15 constants: pi, e, tau, phi, sqrt2, ln2, etc.
   - **Effort**: 1 day

**After Phase 13 (Requires Bitwise Operators)**:
2. **Random** → `stdlib/random.gr`
   - Implement xoshiro256** PRNG in pure Graphoid
   - Requires bitwise: `^`, `<<`, `>>`, `|`
   - **Effort**: 2 days

3. **UUID** → `stdlib/uuid.gr`
   - UUID v4 generation
   - Requires bitwise: `&`, `|` for version/variant bits
   - **Effort**: 1 day

4. **Crypto** → `stdlib/crypto.gr`
   - SHA-256, MD5 hashing
   - HMAC authentication
   - Requires extensive bitwise operations
   - **Effort**: 3 days

**Keep in Rust (Core System Functions)**:
- **I/O** - File system operations (system calls)
- **OS** - Process management, environment variables
- **Time** (partial) - System time access (rest can be Graphoid)
- **Regex** - Complex parsing (for now)

### Implementation Strategy

**Day 1: Constants Module**
```graphoid
# stdlib/constants.gr
# Pure Graphoid implementation of constants module

export const.pi = 3.141592653589793
export const.e = 2.718281828459045
export const.tau = 6.283185307179586
export const.phi = 1.618033988749895
export const.sqrt2 = 1.4142135623730951
export const.sqrt3 = 1.7320508075688772
export const.ln2 = 0.6931471805599453
export const.ln10 = 2.302585092994046
export const.log2e = 1.4426950408889634
export const.log10e = 0.4342944819032518
export const.euler_gamma = 0.5772156649015329
export const.speed_of_light = 299792458.0
export const.planck_constant = 6.62607015e-34
export const.avogadro_number = 6.02214076e23
export const.boltzmann_constant = 1.380649e-23
```

**Day 2-3: Random Module**
```graphoid
# stdlib/random.gr
# Pure Graphoid implementation of random number generation

class XoShiRo256StarStar {
    priv state = [0, 0, 0, 0]  # Four 64-bit integers

    fn seed(seed_value) {
        # Initialize state from seed using SplitMix64
        self.state = self.splitmix64_init(seed_value)
    }

    fn next() {
        # xoshiro256** algorithm
        result = self.rotl(self.state[1] * 5, 7) * 9
        t = self.state[1] << 17

        self.state[2] = self.state[2] ^ self.state[0]
        self.state[3] = self.state[3] ^ self.state[1]
        self.state[1] = self.state[1] ^ self.state[2]
        self.state[0] = self.state[0] ^ self.state[3]

        self.state[2] = self.state[2] ^ t
        self.state[3] = self.rotl(self.state[3], 45)

        return result
    }

    fn rotl(x, k) {
        return (x << k) | (x >> (64 - k))
    }
}

# Global RNG instance
priv _global_rng = XoShiRo256StarStar()
_global_rng.seed(os.system_time_nanos())

export fn random() {
    return _global_rng.next() / (2 ** 64)
}

export fn randint(a, b) {
    range = b - a + 1
    return a + (_global_rng.next() % range)
}

export fn choice(list) {
    return list[randint(0, list.length() - 1)]
}

# ... rest of random functions
```

**Day 4: UUID Module**
```graphoid
# stdlib/uuid.gr
import "random"

export fn uuid4() {
    # Generate random bytes
    bytes = []
    for i in 0..16 {
        bytes.append(rand.randint(0, 255))
    }

    # Set version 4 bits
    bytes[6] = (bytes[6] & 0x0F) | 0x40

    # Set variant bits (RFC 4122)
    bytes[8] = (bytes[8] & 0x3F) | 0x80

    # Format as UUID string
    return format_uuid(bytes)
}

priv fn format_uuid(bytes) {
    # Convert to hex and format with hyphens
    hex = bytes.map(b => to_hex(b, 2))
    return hex[0..4].join("") + "-" +
           hex[4..6].join("") + "-" +
           hex[6..8].join("") + "-" +
           hex[8..10].join("") + "-" +
           hex[10..16].join("")
}

priv fn to_hex(num, width) {
    # Convert number to hex string
    hex_digits = "0123456789abcdef"
    result = ""
    for i in 0..width {
        digit = (num >> ((width - i - 1) * 4)) & 0xF
        result = result + hex_digits[digit]
    }
    return result
}
```

**Day 5-7: Crypto Module (Hashing)**
```graphoid
# stdlib/crypto.gr
# Pure Graphoid implementation of SHA-256

class SHA256 {
    priv K = [  # SHA-256 constants
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
        # ... 60 more constants
    ]

    fn hash(message) {
        # SHA-256 algorithm
        # 1. Padding
        padded = self.pad_message(message)

        # 2. Initialize hash values
        h = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
             0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19]

        # 3. Process 512-bit blocks
        for block in padded.chunks(64) {
            w = self.prepare_schedule(block)
            h = self.compress(h, w)
        }

        # 4. Return hex digest
        return h.map(x => to_hex(x, 8)).join("")
    }

    priv fn ch(x, y, z) {
        return (x & y) ^ (~x & z)
    }

    priv fn maj(x, y, z) {
        return (x & y) ^ (x & z) ^ (y & z)
    }

    priv fn rotr(x, n) {
        return (x >> n) | (x << (32 - n))
    }

    priv fn sigma0(x) {
        return self.rotr(x, 2) ^ self.rotr(x, 13) ^ self.rotr(x, 22)
    }

    priv fn sigma1(x) {
        return self.rotr(x, 6) ^ self.rotr(x, 11) ^ self.rotr(x, 25)
    }

    # ... rest of SHA-256 implementation
}

export fn sha256(message) {
    hasher = SHA256()
    return hasher.hash(message)
}
```

**Day 8-10: Integration & Testing**

Update module system to prefer `.gr` implementations:
```rust
// In module_manager.rs
pub fn resolve_module_path(&self, module_name: &str) -> Result<PathBuf> {
    // 1. Check for .gr implementation in stdlib/
    let stdlib_gr = PathBuf::from("stdlib").join(module_name).with_extension("gr");
    if stdlib_gr.exists() {
        return Ok(stdlib_gr);
    }

    // 2. Fall back to native Rust implementation
    if self.is_native_module(module_name) {
        return Ok(PathBuf::from(format!("<native:{}>", module_name)));
    }

    // ... rest of resolution
}
```

Create comprehensive tests:
- `tests/stdlib_gr/constants_tests.gr`
- `tests/stdlib_gr/random_tests.gr`
- `tests/stdlib_gr/uuid_tests.gr`
- `tests/stdlib_gr/crypto_tests.gr`

**Success Criteria**:
- ✅ Constants, Random, UUID, Crypto translated to pure .gr
- ✅ All functionality matches Rust implementation
- ✅ 150+ tests passing for .gr stdlib
- ✅ Module system prefers .gr over native when available
- ✅ 90%+ of stdlib in pure Graphoid (by line count)
- ✅ Only I/O, OS, Time (partial), Regex remain in Rust
- ✅ Self-hosting goal achieved!

**Documentation**:
- Update CLAUDE.md to reflect self-hosting status
- Document translation guidelines for future modules
- Show performance comparison (Rust vs .gr implementations)

---

## Phase 15: Testing Framework (7-10 days)

Built-in RSpec-style testing framework:

- Built-in `assert` module with rich assertions
- Test file discovery (.test.gr files)
- Test runner with color output
- Setup/teardown hooks
- Test groups and organization
- Coverage reporting
- Mocking and stubbing system
- Property-based testing

---

## Phase 16: Debugger (10-14 days)

Interactive debugging and profiling tools:

- Breakpoint support (`debug.break()`)
- Debug REPL with inspection commands
- Variable and stack inspection
- Step-through execution
- Debug module API
- Performance profiling tools
- DAP (Debug Adapter Protocol) integration for IDEs
- Graph visualization in debugger

---

## Phase 17: Package Manager (14-21 days)

Graph-based dependency management:

- Package manifest format (`graphoid.toml`)
- Dependency resolution algorithm (graph-based!)
- Lock file generation (`graphoid.lock`)
- `graphoid new` - Project scaffolding
- `graphoid install` - Dependency installation
- `graphoid publish` - Package publishing
- `graphoid test` - Run test suite
- Package registry client
- SemVer version constraint handling
- Build scripts support

---

## Development Best Practices

### Code Organization
- One module per major feature area
- Clear separation of concerns
- Documentation for all public APIs

### Testing Strategy
- Unit tests for each component
- Integration tests for workflows
- Property-based testing where applicable
- Regression tests for bugs

### Performance
- Profile before optimizing
- Use `cargo bench` for benchmarks
- Focus on algorithmic improvements first

### Error Handling
- Rich error messages with source positions
- Helpful suggestions where possible
- Never panic in production code

### Git Workflow
- Commit after each passing test suite
- Clear commit messages
- Feature branches for major work

---

## Timeline Estimates

**NOTE**: These estimates have been revised based on 10 new language features, standard graph types library, and documentation milestones.

### Basic Language (Updated)
**8-11 weeks** - Basic language with collections (+2-3 weeks from original)
- Phases 0-5 complete (with mutation operators)
- Documentation Milestone 1: Basic samples
- Can run simple programs
- Basic REPL works
- **WHY LONGER**: Mutation operator convention doubles method surface area

### Feature Complete (Updated - Revised Phase Ordering)
**20-25 weeks** - Full language specification (+8-9 weeks from original)
- Phases 0-14 complete (includes pattern matching in Phases 7 & 9)
- Phase 6.5: Standard graph types (BST, DAG, Heap, AVL, Trie)
- Function pattern matching (Phase 7) + Graph pattern matching (Phase 9)
- Complete behavior system (Phase 8)
- 5-level graph querying system (Phases 6 & 9)
- Error handling with configurable modes (✅ already done)
- Module system complete (Phase 10)
- Pure Graphoid stdlib (Phase 11)
- Native stdlib modules (Phase 12)
- Bitwise operators & integer types (Phase 13)
- Stdlib translation to pure Graphoid (Phase 14) - 90%+ self-hosting!
- Documentation Milestones 1-4 complete
- **WHY LONGER**: Graph querying is massive (+7 days), pattern matching is critical (+12 days), behaviors complete, bitwise operators (+7 days), stdlib translation (+10 days)

### Production Tools Complete (Updated - Revised Phase Ordering)
**24-32 weeks** - Professional tooling added (+8-10 weeks from original)
- Phases 0-17 complete
- Testing framework operational (Phase 15 - RSpec-style)
- Debugger functional (Phase 16)
- Package manager working (Phase 17)
- **WHY LONGER**: More features to test, REPL/CLI parity for everything, self-hosting validation

### Production Ready (Updated)
**32-40 weeks** - Optimized, polished, and professional (+8-12 weeks from original)
- All phases complete (0-17)
- Performance tuning done
- Comprehensive testing (500+ tests)
- Full documentation with samples
- Examples and tutorials
- Package registry live
- 90%+ self-hosting with pure .gr stdlib

### Comparison

| Milestone | Old Estimate | New Estimate | Increase |
|-----------|-------------|--------------|----------|
| Basic Language | 6-8 weeks | 8-11 weeks | +2-3 weeks |
| Feature Complete | 12-16 weeks | 20-25 weeks | +8-9 weeks |
| Production Tools | 16-22 weeks | 24-32 weeks | +8-10 weeks |
| **Production Ready** | **24-28 weeks** | **32-40 weeks** | **+8-12 weeks** |

**We're building this right, not fast.**

---

## Success Metrics

### Code Quality
- 80%+ test coverage
- Zero compiler warnings
- Passes clippy lints
- Formatted with rustfmt

### Functionality
- All specification features implemented
- Matches Python implementation behavior
- 500+ passing tests

### Performance
- Startup time < 100ms
- Basic operations < 1µs
- Can handle 10,000+ node graphs

### Documentation
- API docs complete
- User guide written
- Examples for all features
- Migration guide from Python

### Production Tooling
- 95%+ test coverage via testing framework
- Debugger works in REPL and IDE
- Package manager can install from registry
- 100+ packages published (community goal)
- `graphoid test`, `graphoid debug`, `graphoid install` all functional

---

## Next Steps

1. **Review this roadmap** - Ensure alignment with goals ✅
2. **Set up development environment** - Phase 0 ✅ COMPLETE
3. **Lexer implementation** - Phase 1 🟡 PARTIALLY COMPLETE
   - ✅ New tokens implemented (then, unless, //, !, element-wise operators)
   - ✅ Comment syntax changed to #
   - 🔲 Complete test coverage (need 20+ tests)
4. **Complete Phase 1** - Finish lexer tests ← START HERE NEXT
5. **Begin Phase 2** - Parser & AST with new syntax nodes
6. **Follow TDD religiously** - Tests before implementation, always
7. **Regular check-ins** - Weekly progress reviews
8. **Adjust as needed** - Roadmap is flexible but comprehensive

---

**Related Documents**:
- [Language Specification](LANGUAGE_SPECIFICATION.md) - Canonical reference for all language features
- [Roadmap Updates](archive/sessions/2025-01-roadmap-updates/ROADMAP_UPDATES_FOR_NEW_FEATURES.md) - **CRITICAL**: Detailed phase-by-phase implementation guidance for new features
- [Production Tooling Specification](PRODUCTION_TOOLING_SPECIFICATION.md) - Testing, debugging, packages
- [Architecture Design](ARCHITECTURE_DESIGN.md) - Internal architecture decisions
- [Design Decisions](archive/sessions/2025-01-design-decisions/DESIGN_DECISIONS_SESSION_1.md) - Approved design decisions for inline conditionals, mutation operators, graph querying, etc.

**For Implementation**: When working on each phase, consult both this roadmap AND the archived updates document for complete implementation details, especially for:
- Mutation operator metadata system (Rust `sort()` / `sort_mut()` → Graphoid `sort()` / `sort!()`)
- 5-level graph querying system specification
- Error handling configuration stack
- Freeze model semantics
- TDD requirements and test count minimums per phase

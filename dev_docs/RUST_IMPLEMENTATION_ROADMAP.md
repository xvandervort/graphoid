# Graphoid/Glang: Rust Implementation Roadmap

**Version**: 1.0
**Last Updated**: January 2025
**Status**: Fresh implementation starting from clean slate

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

**Duration**: 3-5 days
**Goal**: Convert source code into tokens

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
    let mut lexer = Lexer::new("func if else while name");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type, TokenType::Func);
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
    func add(x, y) {
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

**Duration**: 4-6 days
**Goal**: Support user-defined functions and lambdas

See detailed implementation in next section...

---

## Remaining Phases Summary

### Phase 5: Collections & Methods (7-10 days)
- List operations and methods
- Map operations
- String methods
- Method dispatch system

### Phase 6: Graph Types & Rules (10-14 days)
- Graph value representation
- Tree implementation
- Rule system and validators
- Edge governance

### Phase 7: Behavior System (5-7 days)
- Standard behaviors
- Custom function behaviors
- Conditional behaviors
- Rulesets

### Phase 8: Module System (4-6 days)
- Module loading
- Import/export
- Stdlib structure

### Phase 9: Native Stdlib Modules (14-21 days)
- Constants module
- Random module
- Time module
- Regex module
- I/O module
- JSON module
- Crypto module

### Phase 10: Pure Graphoid Stdlib (10-14 days)
- Statistics module
- CSV module
- SQL module
- HTML module
- HTTP module

### Phase 11: Advanced Features (14-21 days)
- Precision context blocks
- Configuration blocks
- Pattern matching (future)
- Optimizations

### Phase 12: Testing Framework (7-10 days)
- Built-in `assert` module with rich assertions
- Test file discovery (.test.gr files)
- Test runner with color output
- Setup/teardown hooks
- Test groups and organization
- Coverage reporting
- Mocking and stubbing system
- Property-based testing

### Phase 13: Debugger (10-14 days)
- Breakpoint support (`debug.break()`)
- Debug REPL with inspection commands
- Variable and stack inspection
- Step-through execution
- Debug module API
- Performance profiling tools
- DAP (Debug Adapter Protocol) integration for IDEs
- Graph visualization in debugger

### Phase 14: Package Manager (14-21 days)
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

### Minimal Viable Product (MVP)
**6-8 weeks** - Basic language without graph features
- Phases 0-5 complete
- Can run simple programs
- Basic REPL works

### Feature Complete
**12-16 weeks** - Full language specification
- Phases 0-11 complete
- Graph types working
- Stdlib mostly complete

### Production Tools Complete
**16-22 weeks** - Professional tooling added
- Phases 0-14 complete
- Testing framework operational
- Debugger functional
- Package manager working

### Production Ready
**24-28 weeks** - Optimized, polished, and professional
- All phases complete
- Performance tuning done
- Comprehensive testing
- Full documentation
- Examples and tutorials
- Package registry live

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

1. **Review this roadmap** - Ensure alignment with goals
2. **Set up development environment** - Phase 0 ✅ COMPLETE
3. **Start with lexer** - Phase 1 ← START HERE NEXT
4. **Regular check-ins** - Weekly progress reviews
5. **Adjust as needed** - Roadmap is flexible

---

**Related Documents**:
- [Language Specification](LANGUAGE_SPECIFICATION.md) - Canonical reference
- [Production Tooling Specification](PRODUCTION_TOOLING_SPECIFICATION.md) - Testing, debugging, packages
- [Architecture Design](ARCHITECTURE_DESIGN.md) - Internal architecture decisions
- [Philosophy](PHILOSOPHY.md) - Core design principles (if exists)
- [Edge Governance Design](EDGE_GOVERNANCE_DESIGN.md) - Edge system details (if exists)

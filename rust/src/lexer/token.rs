//! Token types for the Graphoid lexer

use crate::error::SourcePosition;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Number(f64),
    String(String),
    Symbol(String),      // :symbol
    Regex(String, String), // pattern, flags
    True,
    False,
    None,

    // Identifiers and Keywords
    Identifier(String),
    Func,
    If,
    Then,
    Else,
    Unless,
    While,
    For,
    In,
    Return,
    Break,
    Continue,
    Try,
    Catch,
    Finally,
    Raise,
    As,
    Import,
    Load,
    Module,
    Alias,
    Priv,        // priv keyword for private symbols
    Configure,
    Precision,
    And,
    Or,
    Not,
    Describe,
    Context,
    It,
    Before,
    After,
    Expect,
    Where,
    Shared,

    // Types
    NumType,
    StringType,
    BoolType,
    ListType,
    MapType,
    TreeType,
    GraphType,
    DataType,
    TimeType,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    SlashSlash,      // // (integer division)
    Percent,
    Caret,
    Equal,
    EqualEqual,
    Bang,            // ! (for mutation operators like sort!())
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    RegexMatch,      // =~
    RegexNoMatch,    // !~
    Arrow,           // =>
    Ampersand,       // &
    Pipe,            // |
    AmpersandAmpersand, // &&
    PipePipe,        // ||

    // Element-wise operators (dot-prefix)
    DotPlus,         // .+
    DotMinus,        // .-
    DotStar,         // .*
    DotSlash,        // ./
    DotSlashSlash,   // .//
    DotPercent,      // .%
    DotCaret,        // .^
    DotEqualEqual,   // .==
    DotBangEqual,    // .!=
    DotLess,         // .<
    DotLessEqual,    // .<=
    DotGreater,      // .>
    DotGreaterEqual, // .>=

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

    pub fn position(&self) -> SourcePosition {
        SourcePosition {
            line: self.line,
            column: self.column,
            file: None,
        }
    }
}

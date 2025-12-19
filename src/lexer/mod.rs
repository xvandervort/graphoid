//! Lexer module for tokenization
//!
//! This module converts source code into tokens.

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
        loop {
            // Check if we're at end before advancing
            if self.is_at_end() {
                return Ok(Token::new(
                    TokenType::Eof,
                    String::new(),
                    self.line,
                    self.column,
                ));
            }

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
            '.' => {
                // Check for ... (rest/spread operator)
                if self.peek() == '.' && self.peek_next() == '.' {
                    self.advance(); // consume second dot
                    self.advance(); // consume third dot
                    return Ok(Token::new(
                        TokenType::DotDotDot,
                        "...".to_string(),
                        start_line,
                        start_column,
                    ));
                }

                // Check for element-wise operators
                let next_ch = self.peek();
                match next_ch {
                    '+' => {
                        self.advance();
                        return Ok(Token::new(
                            TokenType::DotPlus,
                            ".+".to_string(),
                            start_line,
                            start_column,
                        ));
                    }
                    '-' => {
                        self.advance();
                        return Ok(Token::new(
                            TokenType::DotMinus,
                            ".-".to_string(),
                            start_line,
                            start_column,
                        ));
                    }
                    '*' => {
                        self.advance();
                        // Check for .**
                        if self.peek() == '*' {
                            self.advance();
                            return Ok(Token::new(
                                TokenType::DotDoubleStar,
                                ".**".to_string(),
                                start_line,
                                start_column,
                            ));
                        }
                        return Ok(Token::new(
                            TokenType::DotStar,
                            ".*".to_string(),
                            start_line,
                            start_column,
                        ));
                    }
                    '/' => {
                        self.advance();
                        // Check for .//
                        if self.peek() == '/' {
                            self.advance();
                            return Ok(Token::new(
                                TokenType::DotSlashSlash,
                                ".//".to_string(),
                                start_line,
                                start_column,
                            ));
                        }
                        return Ok(Token::new(
                            TokenType::DotSlash,
                            "./".to_string(),
                            start_line,
                            start_column,
                        ));
                    }
                    '%' => {
                        self.advance();
                        return Ok(Token::new(
                            TokenType::DotPercent,
                            ".%".to_string(),
                            start_line,
                            start_column,
                        ));
                    }
                    '^' => {
                        self.advance();
                        return Ok(Token::new(
                            TokenType::DotCaret,
                            ".^".to_string(),
                            start_line,
                            start_column,
                        ));
                    }
                    '=' => {
                        self.advance();
                        if self.peek() == '=' {
                            self.advance();
                            return Ok(Token::new(
                                TokenType::DotEqualEqual,
                                ".==".to_string(),
                                start_line,
                                start_column,
                            ));
                        }
                        // Just .= is not valid, backtrack
                        self.current -= 1;
                        self.column -= 1;
                        TokenType::Dot
                    }
                    '!' => {
                        self.advance();
                        if self.peek() == '=' {
                            self.advance();
                            return Ok(Token::new(
                                TokenType::DotBangEqual,
                                ".!=".to_string(),
                                start_line,
                                start_column,
                            ));
                        }
                        // Just .! is not valid, backtrack
                        self.current -= 1;
                        self.column -= 1;
                        TokenType::Dot
                    }
                    '<' => {
                        self.advance();
                        // Check for .<=
                        if self.peek() == '=' {
                            self.advance();
                            return Ok(Token::new(
                                TokenType::DotLessEqual,
                                ".<=".to_string(),
                                start_line,
                                start_column,
                            ));
                        }
                        return Ok(Token::new(
                            TokenType::DotLess,
                            ".<".to_string(),
                            start_line,
                            start_column,
                        ));
                    }
                    '>' => {
                        self.advance();
                        // Check for .>=
                        if self.peek() == '=' {
                            self.advance();
                            return Ok(Token::new(
                                TokenType::DotGreaterEqual,
                                ".>=".to_string(),
                                start_line,
                                start_column,
                            ));
                        }
                        return Ok(Token::new(
                            TokenType::DotGreater,
                            ".>".to_string(),
                            start_line,
                            start_column,
                        ));
                    }
                    // Check if this is a number like .5
                    _ if next_ch.is_ascii_digit() => {
                        self.current -= 1;
                        self.column -= 1;
                        return self.number();
                    }
                    _ => TokenType::Dot
                }
            }
            ':' => {
                // Could be : or :symbol
                // Allow symbols starting with letters, digits, or underscore
                if self.peek().is_alphanumeric() || self.peek() == '_' {
                    return self.symbol();
                }
                TokenType::Colon
            }
            ';' => TokenType::Semicolon,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => {
                // Check for ** (power operator)
                if self.match_char('*') {
                    return Ok(Token::new(
                        TokenType::DoubleStar,
                        "**".to_string(),
                        start_line,
                        start_column,
                    ));
                }
                TokenType::Star
            }
            '%' => TokenType::Percent,
            '^' => TokenType::Caret,  // Now XOR (Phase 13+), was power (Phase 0-12)
            '~' => TokenType::Tilde,  // NEW: Bitwise NOT

            // Two-character operators
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
                    // Standalone ! for mutation operators (e.g., sort!())
                    TokenType::Bang
                }
            }
            '<' => {
                if self.match_char('<') {
                    // Left shift <<
                    return Ok(Token::new(
                        TokenType::LeftShift,
                        "<<".to_string(),
                        start_line,
                        start_column,
                    ));
                } else if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            '>' => {
                if self.match_char('>') {
                    // Right shift >>
                    return Ok(Token::new(
                        TokenType::RightShift,
                        ">>".to_string(),
                        start_line,
                        start_column,
                    ));
                } else if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            '&' => {
                if self.match_char('&') {
                    TokenType::AmpersandAmpersand
                } else {
                    TokenType::Ampersand
                }
            }
            '|' => {
                if self.match_char('|') {
                    TokenType::PipePipe
                } else {
                    TokenType::Pipe
                }
            }

            // Division and integer division
            '/' => {
                if self.peek() == '/' {
                    // Integer division //
                    self.advance();
                    return Ok(Token::new(
                        TokenType::SlashSlash,
                        "//".to_string(),
                        start_line,
                        start_column,
                    ));
                } else if self.peek() == '*' {
                    // Multi-line comment /* */
                    self.skip_block_comment()?;
                    self.skip_whitespace_except_newline();
                    continue; // Loop back to get next token
                } else {
                    TokenType::Slash
                }
            }

            // Single-line comments
            '#' => {
                self.skip_line_comment();
                self.skip_whitespace_except_newline();
                continue; // Loop back to get next token
            }

            // Strings
            '"' | '\'' => return self.string(ch),

            // Numbers (including binary 0b and hex 0x)
            ch if ch.is_ascii_digit() => {
                self.current -= 1;
                self.column -= 1;
                return self.number_or_literal();
            }

            // Identifiers and keywords
            ch if ch.is_alphabetic() || ch == '_' => {
                self.current -= 1;
                self.column -= 1;
                return self.identifier();
            }

            // Newlines
            '\n' => {
                let token = Token::new(
                    TokenType::Newline,
                    "\n".to_string(),
                    start_line,
                    start_column,
                );
                self.line += 1;
                self.column = 1;
                return Ok(token);
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

            return Ok(Token::new(
                token_type,
                ch.to_string(),
                start_line,
                start_column,
            ));
        } // End of loop
    }

    // Helper methods
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

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
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

    fn skip_line_comment(&mut self) {
        // Skip until newline (# has already been consumed)
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
    }

    fn skip_block_comment(&mut self) -> Result<()> {
        let start_line = self.line;
        let start_column = self.column;

        // Skip the *
        self.advance();

        while !self.is_at_end() {
            if self.peek() == '*' && self.peek_next() == '/' {
                self.advance(); // consume *
                self.advance(); // consume /
                return Ok(());
            }

            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }

            self.advance();
        }

        Err(GraphoidError::SyntaxError {
            message: "Unterminated block comment".to_string(),
            position: SourcePosition {
                line: start_line,
                column: start_column,
                file: None,
            },
        })
    }

    fn string(&mut self, quote: char) -> Result<Token> {
        let start_line = self.line;
        let start_column = self.column - 1;
        let mut value = String::new();

        while !self.is_at_end() && self.peek() != quote {
            if self.peek() == '\\' {
                self.advance(); // consume backslash
                if self.is_at_end() {
                    return Err(GraphoidError::SyntaxError {
                        message: "Unterminated string".to_string(),
                        position: SourcePosition {
                            line: start_line,
                            column: start_column,
                            file: None,
                        },
                    });
                }

                let escaped = self.advance();
                let ch = match escaped {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '\\' => '\\',
                    '\'' => '\'',
                    '"' => '"',
                    _ => escaped,
                };
                value.push(ch);
            } else {
                if self.peek() == '\n' {
                    self.line += 1;
                    self.column = 0;
                }
                value.push(self.advance());
            }
        }

        if self.is_at_end() {
            return Err(GraphoidError::SyntaxError {
                message: "Unterminated string".to_string(),
                position: SourcePosition {
                    line: start_line,
                    column: start_column,
                    file: None,
                },
            });
        }

        // Consume closing quote
        self.advance();

        Ok(Token::new(
            TokenType::String(value.clone()),
            format!("{}{}{}", quote, value, quote),
            start_line,
            start_column,
        ))
    }

    fn number_or_literal(&mut self) -> Result<Token> {
        // Check for 0b (binary) or 0x (hex) prefix
        if self.peek() == '0' {
            let next = self.peek_next();
            if next == 'b' || next == 'B' {
                return self.binary_literal();
            } else if next == 'x' || next == 'X' {
                return self.hex_literal();
            }
        }

        // Regular decimal number
        self.number()
    }

    fn number(&mut self) -> Result<Token> {
        let start_line = self.line;
        let start_column = self.column;
        let mut lexeme = String::new();

        while !self.is_at_end() && self.peek().is_ascii_digit() {
            lexeme.push(self.advance());
        }

        // Check for decimal point
        if !self.is_at_end() && self.peek() == '.' && self.peek_next().is_ascii_digit() {
            lexeme.push(self.advance()); // consume .

            while !self.is_at_end() && self.peek().is_ascii_digit() {
                lexeme.push(self.advance());
            }
        }

        let value = lexeme.parse::<f64>().map_err(|_| GraphoidError::SyntaxError {
            message: format!("Invalid number: {}", lexeme),
            position: SourcePosition {
                line: start_line,
                column: start_column,
                file: None,
            },
        })?;

        Ok(Token::new(
            TokenType::Number(value),
            lexeme,
            start_line,
            start_column,
        ))
    }

    fn binary_literal(&mut self) -> Result<Token> {
        let start_line = self.line;
        let start_column = self.column;
        let mut lexeme = String::new();

        // Consume '0'
        lexeme.push(self.advance());
        // Consume 'b' or 'B'
        lexeme.push(self.advance());

        let mut value: i64 = 0;
        let mut has_digits = false;

        while !self.is_at_end() {
            let ch = self.peek();
            match ch {
                '0' | '1' => {
                    value = value.checked_mul(2).ok_or_else(|| GraphoidError::SyntaxError {
                        message: "Binary literal overflow".to_string(),
                        position: SourcePosition {
                            line: start_line,
                            column: start_column,
                            file: None,
                        },
                    })?;
                    value = value.checked_add((ch as i64) - ('0' as i64)).ok_or_else(|| GraphoidError::SyntaxError {
                        message: "Binary literal overflow".to_string(),
                        position: SourcePosition {
                            line: start_line,
                            column: start_column,
                            file: None,
                        },
                    })?;
                    lexeme.push(self.advance());
                    has_digits = true;
                }
                '_' => {
                    // Underscores for readability, skip them
                    lexeme.push(self.advance());
                }
                '2'..='9' | 'a'..='z' | 'A'..='Z' => {
                    // Invalid character in binary literal
                    return Err(GraphoidError::SyntaxError {
                        message: format!("Invalid character '{}' in binary literal", ch),
                        position: SourcePosition {
                            line: self.line,
                            column: self.column,
                            file: None,
                        },
                    });
                }
                _ => break,
            }
        }

        if !has_digits {
            return Err(GraphoidError::SyntaxError {
                message: "Binary literal must have at least one digit".to_string(),
                position: SourcePosition {
                    line: start_line,
                    column: start_column,
                    file: None,
                },
            });
        }

        Ok(Token::new(
            TokenType::Number(value as f64),
            lexeme,
            start_line,
            start_column,
        ))
    }

    fn hex_literal(&mut self) -> Result<Token> {
        let start_line = self.line;
        let start_column = self.column;
        let mut lexeme = String::new();

        // Consume '0'
        lexeme.push(self.advance());
        // Consume 'x' or 'X'
        lexeme.push(self.advance());

        let mut value: i64 = 0;
        let mut has_digits = false;

        while !self.is_at_end() {
            let ch = self.peek();
            match ch {
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    let digit_value = match ch {
                        '0'..='9' => (ch as i64) - ('0' as i64),
                        'a'..='f' => (ch as i64) - ('a' as i64) + 10,
                        'A'..='F' => (ch as i64) - ('A' as i64) + 10,
                        _ => unreachable!(),
                    };
                    value = value.checked_mul(16).ok_or_else(|| GraphoidError::SyntaxError {
                        message: "Hexadecimal literal overflow".to_string(),
                        position: SourcePosition {
                            line: start_line,
                            column: start_column,
                            file: None,
                        },
                    })?;
                    value = value.checked_add(digit_value).ok_or_else(|| GraphoidError::SyntaxError {
                        message: "Hexadecimal literal overflow".to_string(),
                        position: SourcePosition {
                            line: start_line,
                            column: start_column,
                            file: None,
                        },
                    })?;
                    lexeme.push(self.advance());
                    has_digits = true;
                }
                '_' => {
                    // Underscores for readability, skip them
                    lexeme.push(self.advance());
                }
                'g'..='z' | 'G'..='Z' => {
                    // Invalid character in hex literal
                    return Err(GraphoidError::SyntaxError {
                        message: format!("Invalid character '{}' in hexadecimal literal", ch),
                        position: SourcePosition {
                            line: self.line,
                            column: self.column,
                            file: None,
                        },
                    });
                }
                _ => break,
            }
        }

        if !has_digits {
            return Err(GraphoidError::SyntaxError {
                message: "Hexadecimal literal must have at least one digit".to_string(),
                position: SourcePosition {
                    line: start_line,
                    column: start_column,
                    file: None,
                },
            });
        }

        Ok(Token::new(
            TokenType::Number(value as f64),
            lexeme,
            start_line,
            start_column,
        ))
    }

    fn identifier(&mut self) -> Result<Token> {
        let start_line = self.line;
        let start_column = self.column;
        let mut lexeme = String::new();

        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            lexeme.push(self.advance());
        }

        // Note: Trailing ! for mutating methods is now handled at the parser level
        // The parser checks for a Bang token after method names and mutable arguments

        let token_type = match lexeme.as_str() {
            // Keywords
            "fn" => TokenType::Func,
            "if" => TokenType::If,
            "then" => TokenType::Then,
            "else" => TokenType::Else,
            "unless" => TokenType::Unless,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "in" => TokenType::In,
            "return" => TokenType::Return,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "try" => TokenType::Try,
            "catch" => TokenType::Catch,
            "finally" => TokenType::Finally,
            "raise" => TokenType::Raise,
            "as" => TokenType::As,
            "import" => TokenType::Import,
            "load" => TokenType::Load,
            "module" => TokenType::Module,
            "alias" => TokenType::Alias,
            "priv" => TokenType::Priv,
            "match" => TokenType::Match,
            "from" => TokenType::From,
            "super" => TokenType::Super,
            "set" => TokenType::Set,
            "static" => TokenType::Static,
            "when" => TokenType::When,
            "configure" => TokenType::Configure,
            "rule" => TokenType::Rule,
            "precision" => TokenType::Precision,
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "not" => TokenType::Not,

            // Boolean literals
            "true" => TokenType::True,
            "false" => TokenType::False,
            "none" => TokenType::None,

            // Type keywords
            "num" => TokenType::NumType,
            "bignum" => TokenType::BigNumType,  // Phase 1B
            "string" => TokenType::StringType,
            "bool" => TokenType::BoolType,
            "list" => TokenType::ListType,
            "map" => TokenType::MapType,
            "hash" => TokenType::MapType,  // Alias for map
            "tree" => TokenType::TreeType,
            "graph" => TokenType::GraphType,

            _ => TokenType::Identifier(lexeme.clone()),
        };

        Ok(Token::new(token_type, lexeme, start_line, start_column))
    }

    fn symbol(&mut self) -> Result<Token> {
        let start_line = self.line;
        let start_column = self.column - 1; // Already consumed ':'
        let mut lexeme = String::from(":");

        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            lexeme.push(self.advance());
        }

        let symbol_name = lexeme[1..].to_string(); // Remove leading ':'

        Ok(Token::new(
            TokenType::Symbol(symbol_name),
            lexeme,
            start_line,
            start_column,
        ))
    }
}

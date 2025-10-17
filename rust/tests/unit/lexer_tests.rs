//! Lexer unit tests

use graphoid::lexer::{Lexer, token::TokenType};

// ============================================================================
// PHASE 1: Token Creation Tests
// ============================================================================

#[test]
fn test_token_creation() {
    use graphoid::lexer::token::Token;

    let token = Token::new(TokenType::Plus, "+".to_string(), 1, 1);
    assert_eq!(token.lexeme, "+");
    assert_eq!(token.line, 1);
    assert_eq!(token.column, 1);
}

// ============================================================================
// PHASE 2: Simple Single-Character Tokens
// ============================================================================

#[test]
fn test_simple_operators() {
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
fn test_delimiters() {
    let mut lexer = Lexer::new("( ) { } [ ] , . : ;");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type, TokenType::LeftParen);
    assert_eq!(tokens[1].token_type, TokenType::RightParen);
    assert_eq!(tokens[2].token_type, TokenType::LeftBrace);
    assert_eq!(tokens[3].token_type, TokenType::RightBrace);
    assert_eq!(tokens[4].token_type, TokenType::LeftBracket);
    assert_eq!(tokens[5].token_type, TokenType::RightBracket);
    assert_eq!(tokens[6].token_type, TokenType::Comma);
    assert_eq!(tokens[7].token_type, TokenType::Dot);
    assert_eq!(tokens[8].token_type, TokenType::Colon);
    assert_eq!(tokens[9].token_type, TokenType::Semicolon);
}

// ============================================================================
// PHASE 3: Number Tokenization
// ============================================================================

#[test]
fn test_integer_numbers() {
    let mut lexer = Lexer::new("0 42 100 999");
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::Number(n) => assert_eq!(*n, 0.0),
        _ => panic!("Expected number"),
    }

    match &tokens[1].token_type {
        TokenType::Number(n) => assert_eq!(*n, 42.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_float_numbers() {
    let mut lexer = Lexer::new("3.14 0.5 99.999");
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::Number(n) => assert_eq!(*n, 3.14),
        _ => panic!("Expected number"),
    }

    match &tokens[1].token_type {
        TokenType::Number(n) => assert_eq!(*n, 0.5),
        _ => panic!("Expected number"),
    }
}

// ============================================================================
// PHASE 4: String Tokenization
// ============================================================================

#[test]
fn test_double_quoted_strings() {
    let mut lexer = Lexer::new(r#""hello" "world""#);
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
fn test_single_quoted_strings() {
    let mut lexer = Lexer::new(r#"'hello' 'world'"#);
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::String(s) => assert_eq!(s, "hello"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn test_string_with_escapes() {
    let mut lexer = Lexer::new(r#""hello\nworld" "tab\there""#);
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::String(s) => assert_eq!(s, "hello\nworld"),
        _ => panic!("Expected string with newline"),
    }

    match &tokens[1].token_type {
        TokenType::String(s) => assert_eq!(s, "tab\there"),
        _ => panic!("Expected string with tab"),
    }
}

// ============================================================================
// PHASE 5: Identifiers and Keywords
// ============================================================================

#[test]
fn test_keywords() {
    let mut lexer = Lexer::new("func if else while for in return break continue");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type, TokenType::Func);
    assert_eq!(tokens[1].token_type, TokenType::If);
    assert_eq!(tokens[2].token_type, TokenType::Else);
    assert_eq!(tokens[3].token_type, TokenType::While);
    assert_eq!(tokens[4].token_type, TokenType::For);
    assert_eq!(tokens[5].token_type, TokenType::In);
    assert_eq!(tokens[6].token_type, TokenType::Return);
    assert_eq!(tokens[7].token_type, TokenType::Break);
    assert_eq!(tokens[8].token_type, TokenType::Continue);
}

#[test]
fn test_type_keywords() {
    let mut lexer = Lexer::new("num string bool list map tree graph data");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type, TokenType::NumType);
    assert_eq!(tokens[1].token_type, TokenType::StringType);
    assert_eq!(tokens[2].token_type, TokenType::BoolType);
    assert_eq!(tokens[3].token_type, TokenType::ListType);
    assert_eq!(tokens[4].token_type, TokenType::MapType);
    assert_eq!(tokens[5].token_type, TokenType::TreeType);
    assert_eq!(tokens[6].token_type, TokenType::GraphType);
    assert_eq!(tokens[7].token_type, TokenType::DataType);
}

#[test]
fn test_boolean_keywords() {
    let mut lexer = Lexer::new("true false none");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type, TokenType::True);
    assert_eq!(tokens[1].token_type, TokenType::False);
    assert_eq!(tokens[2].token_type, TokenType::None);
}

#[test]
fn test_identifiers() {
    let mut lexer = Lexer::new("name x value_123 _private");
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "name"),
        _ => panic!("Expected identifier"),
    }

    match &tokens[1].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "x"),
        _ => panic!("Expected identifier"),
    }

    match &tokens[2].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "value_123"),
        _ => panic!("Expected identifier"),
    }
}

// ============================================================================
// PHASE 6: Multi-Character Operators
// ============================================================================

#[test]
fn test_comparison_operators() {
    let mut lexer = Lexer::new("== != < <= > >=");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type, TokenType::EqualEqual);
    assert_eq!(tokens[1].token_type, TokenType::BangEqual);
    assert_eq!(tokens[2].token_type, TokenType::Less);
    assert_eq!(tokens[3].token_type, TokenType::LessEqual);
    assert_eq!(tokens[4].token_type, TokenType::Greater);
    assert_eq!(tokens[5].token_type, TokenType::GreaterEqual);
}

#[test]
fn test_regex_operators() {
    let mut lexer = Lexer::new("=~ !~");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type, TokenType::RegexMatch);
    assert_eq!(tokens[1].token_type, TokenType::RegexNoMatch);
}

#[test]
fn test_arrow_operator() {
    let mut lexer = Lexer::new("=>");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type, TokenType::Arrow);
}

#[test]
fn test_logical_operators() {
    let mut lexer = Lexer::new("and or && ||");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type, TokenType::And);
    assert_eq!(tokens[1].token_type, TokenType::Or);
    assert_eq!(tokens[2].token_type, TokenType::AmpersandAmpersand);
    assert_eq!(tokens[3].token_type, TokenType::PipePipe);
}

// ============================================================================
// PHASE 7: Comments
// ============================================================================

#[test]
fn test_single_line_comment() {
    let mut lexer = Lexer::new("42 // this is a comment\n43");
    let tokens = lexer.tokenize().unwrap();

    // Should have: 42, newline, 43, eof
    match &tokens[0].token_type {
        TokenType::Number(n) => assert_eq!(*n, 42.0),
        _ => panic!("Expected number"),
    }

    assert_eq!(tokens[1].token_type, TokenType::Newline);

    match &tokens[2].token_type {
        TokenType::Number(n) => assert_eq!(*n, 43.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_block_comment() {
    let mut lexer = Lexer::new("42 /* block comment */ 43");
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::Number(n) => assert_eq!(*n, 42.0),
        _ => panic!("Expected number"),
    }

    match &tokens[1].token_type {
        TokenType::Number(n) => assert_eq!(*n, 43.0),
        _ => panic!("Expected number"),
    }
}

// ============================================================================
// PHASE 8: Symbols
// ============================================================================

#[test]
fn test_symbol_literals() {
    let mut lexer = Lexer::new(":ok :error :pending");
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::Symbol(s) => assert_eq!(s, "ok"),
        _ => panic!("Expected symbol"),
    }

    match &tokens[1].token_type {
        TokenType::Symbol(s) => assert_eq!(s, "error"),
        _ => panic!("Expected symbol"),
    }
}

// ============================================================================
// PHASE 9: Newlines and Position Tracking
// ============================================================================

#[test]
fn test_newline_handling() {
    let mut lexer = Lexer::new("42\n43\n44");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type, TokenType::Number(42.0));
    assert_eq!(tokens[1].token_type, TokenType::Newline);
    assert_eq!(tokens[2].token_type, TokenType::Number(43.0));
    assert_eq!(tokens[3].token_type, TokenType::Newline);
    assert_eq!(tokens[4].token_type, TokenType::Number(44.0));
}

#[test]
fn test_position_tracking() {
    let mut lexer = Lexer::new("x + y");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].line, 1);
    assert_eq!(tokens[0].column, 1);

    assert_eq!(tokens[1].line, 1);
    assert_eq!(tokens[1].column, 3);

    assert_eq!(tokens[2].line, 1);
    assert_eq!(tokens[2].column, 5);
}

// ============================================================================
// PHASE 10: Integration Tests
// ============================================================================

#[test]
fn test_complete_expression() {
    let mut lexer = Lexer::new("x = 42 + 3.14");
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "x"),
        _ => panic!("Expected identifier"),
    }
    assert_eq!(tokens[1].token_type, TokenType::Equal);
    assert_eq!(tokens[2].token_type, TokenType::Number(42.0));
    assert_eq!(tokens[3].token_type, TokenType::Plus);
    assert_eq!(tokens[4].token_type, TokenType::Number(3.14));
}

#[test]
fn test_function_declaration() {
    let source = r#"func add(x, y) {
    return x + y
}"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type, TokenType::Func);
    match &tokens[1].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "add"),
        _ => panic!("Expected identifier"),
    }
    assert_eq!(tokens[2].token_type, TokenType::LeftParen);
    // ... and so on
}

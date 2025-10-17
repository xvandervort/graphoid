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
fn test_single_line_comment_legacy() {
    // NOTE: This test is deprecated - Graphoid now uses # for comments
    // Kept for historical reference showing // is now integer division
    let mut lexer = Lexer::new("42 // 3");
    let tokens = lexer.tokenize().unwrap();

    // Should tokenize as: 42 (int division) 3
    match &tokens[0].token_type {
        TokenType::Number(n) => assert_eq!(*n, 42.0),
        _ => panic!("Expected number"),
    }

    assert_eq!(tokens[1].token_type, TokenType::SlashSlash); // Now integer division!

    match &tokens[2].token_type {
        TokenType::Number(n) => assert_eq!(*n, 3.0),
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

// ============================================================================
// PHASE 11: NEW LANGUAGE FEATURES (January 2025)
// ============================================================================

// ---------------------------------------------------------------------------
// NEW KEYWORDS: then, unless
// ---------------------------------------------------------------------------

#[test]
fn test_then_keyword() {
    let mut lexer = Lexer::new("if x > 0 then y else z");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type, TokenType::If);
    assert_eq!(tokens[4].token_type, TokenType::Then);
    assert_eq!(tokens[6].token_type, TokenType::Else);
}

#[test]
fn test_unless_keyword() {
    let mut lexer = Lexer::new("unless error");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type, TokenType::Unless);
}

#[test]
fn test_inline_conditional_syntax() {
    let mut lexer = Lexer::new("status = if age >= 18 then \"adult\" else \"minor\"");
    let tokens = lexer.tokenize().unwrap();

    // Should tokenize: status = if age >= 18 then "adult" else "minor"
    match &tokens[0].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "status"),
        _ => panic!("Expected identifier"),
    }
    assert_eq!(tokens[1].token_type, TokenType::Equal);
    assert_eq!(tokens[2].token_type, TokenType::If);
    assert_eq!(tokens[6].token_type, TokenType::Then);
    assert_eq!(tokens[8].token_type, TokenType::Else);
}

// ---------------------------------------------------------------------------
// INTEGER DIVISION OPERATOR: //
// ---------------------------------------------------------------------------

#[test]
fn test_integer_division_operator() {
    let mut lexer = Lexer::new("10 // 3");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type, TokenType::Number(10.0));
    assert_eq!(tokens[1].token_type, TokenType::SlashSlash);
    assert_eq!(tokens[2].token_type, TokenType::Number(3.0));
}

#[test]
fn test_division_vs_integer_division() {
    let mut lexer = Lexer::new("10 / 3 10 // 3");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type, TokenType::Number(10.0));
    assert_eq!(tokens[1].token_type, TokenType::Slash); // Regular division
    assert_eq!(tokens[2].token_type, TokenType::Number(3.0));
    assert_eq!(tokens[3].token_type, TokenType::Number(10.0));
    assert_eq!(tokens[4].token_type, TokenType::SlashSlash); // Integer division
    assert_eq!(tokens[5].token_type, TokenType::Number(3.0));
}

#[test]
fn test_integer_division_in_expression() {
    let mut lexer = Lexer::new("pages = total_items // items_per_page");
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "pages"),
        _ => panic!("Expected identifier"),
    }
    assert_eq!(tokens[1].token_type, TokenType::Equal);
    match &tokens[2].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "total_items"),
        _ => panic!("Expected identifier"),
    }
    assert_eq!(tokens[3].token_type, TokenType::SlashSlash);
    match &tokens[4].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "items_per_page"),
        _ => panic!("Expected identifier"),
    }
}

// ---------------------------------------------------------------------------
// BANG OPERATOR: ! (for mutation operators like sort!())
// ---------------------------------------------------------------------------

#[test]
fn test_bang_operator() {
    let mut lexer = Lexer::new("items.sort!()");
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "items"),
        _ => panic!("Expected identifier"),
    }
    assert_eq!(tokens[1].token_type, TokenType::Dot);
    match &tokens[2].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "sort"),
        _ => panic!("Expected identifier"),
    }
    assert_eq!(tokens[3].token_type, TokenType::Bang);
    assert_eq!(tokens[4].token_type, TokenType::LeftParen);
    assert_eq!(tokens[5].token_type, TokenType::RightParen);
}

#[test]
fn test_bang_vs_bang_equal() {
    let mut lexer = Lexer::new("! !=");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type, TokenType::Bang); // Standalone !
    assert_eq!(tokens[1].token_type, TokenType::BangEqual); // !=
}

// ---------------------------------------------------------------------------
// ELEMENT-WISE OPERATORS: .+ .- .* ./ .// .% .^
// ---------------------------------------------------------------------------

#[test]
fn test_element_wise_arithmetic() {
    let mut lexer = Lexer::new("items .+ 10");
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "items"),
        _ => panic!("Expected identifier"),
    }
    assert_eq!(tokens[1].token_type, TokenType::DotPlus);
    assert_eq!(tokens[2].token_type, TokenType::Number(10.0));
}

#[test]
fn test_all_element_wise_operators() {
    let mut lexer = Lexer::new(".+ .- .* ./ .// .% .^");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type, TokenType::DotPlus);
    assert_eq!(tokens[1].token_type, TokenType::DotMinus);
    assert_eq!(tokens[2].token_type, TokenType::DotStar);
    assert_eq!(tokens[3].token_type, TokenType::DotSlash);
    assert_eq!(tokens[4].token_type, TokenType::DotSlashSlash);
    assert_eq!(tokens[5].token_type, TokenType::DotPercent);
    assert_eq!(tokens[6].token_type, TokenType::DotCaret);
}

#[test]
fn test_element_wise_comparisons() {
    let mut lexer = Lexer::new(".== .!= .< .<= .> .>=");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type, TokenType::DotEqualEqual);
    assert_eq!(tokens[1].token_type, TokenType::DotBangEqual);
    assert_eq!(tokens[2].token_type, TokenType::DotLess);
    assert_eq!(tokens[3].token_type, TokenType::DotLessEqual);
    assert_eq!(tokens[4].token_type, TokenType::DotGreater);
    assert_eq!(tokens[5].token_type, TokenType::DotGreaterEqual);
}

#[test]
fn test_element_wise_in_expression() {
    let mut lexer = Lexer::new("doubled = items .* 2");
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "doubled"),
        _ => panic!("Expected identifier"),
    }
    assert_eq!(tokens[1].token_type, TokenType::Equal);
    match &tokens[2].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "items"),
        _ => panic!("Expected identifier"),
    }
    assert_eq!(tokens[3].token_type, TokenType::DotStar);
    assert_eq!(tokens[4].token_type, TokenType::Number(2.0));
}

#[test]
fn test_chained_element_wise() {
    let mut lexer = Lexer::new("[1,2,3] .* 2 .+ 10");
    let tokens = lexer.tokenize().unwrap();

    // [1,2,3] .* 2 .+ 10
    assert_eq!(tokens[0].token_type, TokenType::LeftBracket);
    // ... numbers and commas ...
    assert_eq!(tokens[6].token_type, TokenType::RightBracket);
    assert_eq!(tokens[7].token_type, TokenType::DotStar);
    assert_eq!(tokens[8].token_type, TokenType::Number(2.0));
    assert_eq!(tokens[9].token_type, TokenType::DotPlus);
    assert_eq!(tokens[10].token_type, TokenType::Number(10.0));
}

// ---------------------------------------------------------------------------
// COMMENT SYNTAX: # for single-line
// ---------------------------------------------------------------------------

#[test]
fn test_hash_comment() {
    let mut lexer = Lexer::new("42 # this is a comment\n43");
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
fn test_hash_comment_at_end_of_line() {
    let mut lexer = Lexer::new("x = 10 # set x to 10");
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "x"),
        _ => panic!("Expected identifier"),
    }
    assert_eq!(tokens[1].token_type, TokenType::Equal);
    assert_eq!(tokens[2].token_type, TokenType::Number(10.0));
    assert_eq!(tokens[3].token_type, TokenType::Eof);
}

#[test]
fn test_multiple_hash_comments() {
    let source = r#"# Comment line 1
# Comment line 2
x = 42 # inline comment
# Comment line 3
y = 43"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    // Should skip all comments, get: newlines from comment-only lines, x = 42, newline, y = 43
    // Comment-only lines at start produce newlines: tokens[0] and tokens[1]
    assert_eq!(tokens[0].token_type, TokenType::Newline); // After "# Comment line 1"
    assert_eq!(tokens[1].token_type, TokenType::Newline); // After "# Comment line 2"

    match &tokens[2].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "x"),
        _ => panic!("Expected identifier x"),
    }
    assert_eq!(tokens[3].token_type, TokenType::Equal);
    match &tokens[4].token_type {
        TokenType::Number(n) => assert_eq!(*n, 42.0),
        _ => panic!("Expected number 42"),
    }
    assert_eq!(tokens[5].token_type, TokenType::Newline); // After inline comment
    assert_eq!(tokens[6].token_type, TokenType::Newline); // After "# Comment line 3"

    match &tokens[7].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "y"),
        _ => panic!("Expected identifier y"),
    }
}

// ---------------------------------------------------------------------------
// EDGE CASES
// ---------------------------------------------------------------------------

#[test]
fn test_number_starting_with_dot() {
    let mut lexer = Lexer::new(".5 .25");
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::Number(n) => assert_eq!(*n, 0.5),
        _ => panic!("Expected number"),
    }

    match &tokens[1].token_type {
        TokenType::Number(n) => assert_eq!(*n, 0.25),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_dot_vs_dot_operator() {
    let mut lexer = Lexer::new("obj.method() items .* 2");
    let tokens = lexer.tokenize().unwrap();

    // obj.method() - dot is member access
    assert_eq!(tokens[1].token_type, TokenType::Dot);

    // items .* 2 - .* is element-wise multiply
    assert_eq!(tokens[6].token_type, TokenType::DotStar);
}

#[test]
fn test_empty_string() {
    let mut lexer = Lexer::new(r#""""#);
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::String(s) => assert_eq!(s, ""),
        _ => panic!("Expected empty string"),
    }
}

#[test]
fn test_string_with_quotes() {
    let mut lexer = Lexer::new(r#""He said \"hello\"""#);
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::String(s) => assert_eq!(s, "He said \"hello\""),
        _ => panic!("Expected string with escaped quotes"),
    }
}

#[test]
fn test_multiline_string() {
    let source = "\"line1\nline2\nline3\"";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::String(s) => assert_eq!(s, "line1\nline2\nline3"),
        _ => panic!("Expected multiline string"),
    }
}

#[test]
fn test_symbol_with_underscores() {
    let mut lexer = Lexer::new(":nearest_ten :max_children_2");
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::Symbol(s) => assert_eq!(s, "nearest_ten"),
        _ => panic!("Expected symbol"),
    }

    match &tokens[1].token_type {
        TokenType::Symbol(s) => assert_eq!(s, "max_children_2"),
        _ => panic!("Expected symbol"),
    }
}

#[test]
fn test_large_numbers() {
    let mut lexer = Lexer::new("1000000 0.000001 1e10");
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::Number(n) => assert_eq!(*n, 1000000.0),
        _ => panic!("Expected number"),
    }

    match &tokens[1].token_type {
        TokenType::Number(n) => assert_eq!(*n, 0.000001),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_consecutive_operators() {
    let mut lexer = Lexer::new("x+-y");
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "x"),
        _ => panic!("Expected identifier"),
    }
    assert_eq!(tokens[1].token_type, TokenType::Plus);
    assert_eq!(tokens[2].token_type, TokenType::Minus);
    match &tokens[3].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "y"),
        _ => panic!("Expected identifier"),
    }
}

// ---------------------------------------------------------------------------
// ERROR CASES
// ---------------------------------------------------------------------------

#[test]
fn test_unterminated_string() {
    let mut lexer = Lexer::new("\"unterminated");
    let result = lexer.tokenize();

    assert!(result.is_err(), "Should error on unterminated string");
}

#[test]
fn test_unterminated_block_comment() {
    let mut lexer = Lexer::new("/* unterminated comment");
    let result = lexer.tokenize();

    assert!(result.is_err(), "Should error on unterminated block comment");
}

#[test]
fn test_invalid_number() {
    let mut lexer = Lexer::new("123abc");
    let tokens = lexer.tokenize().unwrap();

    // Should parse as: number(123) identifier(abc)
    assert_eq!(tokens[0].token_type, TokenType::Number(123.0));
    match &tokens[1].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "abc"),
        _ => panic!("Expected identifier"),
    }
}

// ---------------------------------------------------------------------------
// COMPREHENSIVE INTEGRATION TESTS
// ---------------------------------------------------------------------------

#[test]
fn test_complete_mutation_expression() {
    let mut lexer = Lexer::new("items.sort!()");
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "items"),
        _ => panic!("Expected identifier"),
    }
    assert_eq!(tokens[1].token_type, TokenType::Dot);
    match &tokens[2].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "sort"),
        _ => panic!("Expected identifier"),
    }
    assert_eq!(tokens[3].token_type, TokenType::Bang);
    assert_eq!(tokens[4].token_type, TokenType::LeftParen);
    assert_eq!(tokens[5].token_type, TokenType::RightParen);
}

#[test]
fn test_complete_inline_conditional() {
    let source = "value = compute() if condition";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "value"),
        _ => panic!("Expected identifier"),
    }
    assert_eq!(tokens[1].token_type, TokenType::Equal);
    match &tokens[2].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "compute"),
        _ => panic!("Expected identifier"),
    }
    assert_eq!(tokens[3].token_type, TokenType::LeftParen);
    assert_eq!(tokens[4].token_type, TokenType::RightParen);
    assert_eq!(tokens[5].token_type, TokenType::If);
    match &tokens[6].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "condition"),
        _ => panic!("Expected identifier"),
    }
}

#[test]
fn test_complete_element_wise_expression() {
    let source = "result = [1, 2, 3] .* 2 .+ 10";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    match &tokens[0].token_type {
        TokenType::Identifier(s) => assert_eq!(s, "result"),
        _ => panic!("Expected identifier"),
    }
    assert_eq!(tokens[1].token_type, TokenType::Equal);
    assert_eq!(tokens[2].token_type, TokenType::LeftBracket);
    // ...numbers...
    assert_eq!(tokens[8].token_type, TokenType::RightBracket);
    assert_eq!(tokens[9].token_type, TokenType::DotStar);
    assert_eq!(tokens[10].token_type, TokenType::Number(2.0));
    assert_eq!(tokens[11].token_type, TokenType::DotPlus);
    assert_eq!(tokens[12].token_type, TokenType::Number(10.0));
}

#[test]
fn test_real_world_code_sample() {
    let source = r#"# Calculate pages needed
total = 100
per_page = 10
pages = total // per_page

# Process each page
for i in [1, 2, 3] {
    items = fetch_items(i)
    items.sort!() # Mutate in place
    doubled = items .* 2 # Element-wise
}

# Conditional assignment
status = if pages > 10 then "many" else "few"
message = "OK" unless error"#;

    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize();

    assert!(result.is_ok(), "Real-world code should tokenize successfully");
    let tokens = result.unwrap();

    // Verify we have a reasonable number of tokens
    assert!(tokens.len() > 50, "Should have many tokens");

    // Spot-check some key tokens
    // Find the // operator
    let has_int_div = tokens.iter().any(|t| t.token_type == TokenType::SlashSlash);
    assert!(has_int_div, "Should have integer division operator");

    // Find the ! operator
    let has_bang = tokens.iter().any(|t| t.token_type == TokenType::Bang);
    assert!(has_bang, "Should have bang operator");

    // Find element-wise operator
    let has_element_wise = tokens.iter().any(|t| t.token_type == TokenType::DotStar);
    assert!(has_element_wise, "Should have element-wise operator");

    // Find then keyword
    let has_then = tokens.iter().any(|t| t.token_type == TokenType::Then);
    assert!(has_then, "Should have 'then' keyword");

    // Find unless keyword
    let has_unless = tokens.iter().any(|t| t.token_type == TokenType::Unless);
    assert!(has_unless, "Should have 'unless' keyword");
}

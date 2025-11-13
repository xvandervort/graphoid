use graphoid::execution::Executor;
use graphoid::values::ValueKind;

// ============================================================================
// Phase 1: Mutating Methods Tests
// ============================================================================

#[test]
fn test_upper_mutating_returns_none() {
    let mut executor = Executor::new();
    let code = r#"
        text = "hello"
        result = text.upper!()
    "#;

    executor.execute_source(code).unwrap();

    // Verify upper!() returned none
    let result_val = executor.env().get("result").unwrap();
    assert!(matches!(&result_val.kind, ValueKind::None), "upper!() should return none");
}

#[test]
fn test_upper_mutating_modifies_variable() {
    let mut executor = Executor::new();
    let code = r#"
        text = "hello world"
        text.upper!()
    "#;

    executor.execute_source(code).unwrap();

    // Verify mutation happened
    let text_val = executor.env().get("text").unwrap();
    assert!(matches!(&text_val.kind, ValueKind::String(s) if s == "HELLO WORLD"));
}

#[test]
fn test_upper_mutating_no_arguments() {
    let mut executor = Executor::new();
    let code = r#"
        text = "hello"
        text.upper!(42)
    "#;

    let result = executor.execute_source(code);
    assert!(result.is_err(), "upper!() should reject arguments");
    assert!(result.unwrap_err().to_string().contains("takes no arguments"));
}

#[test]
fn test_upper_mutating_requires_variable() {
    let mut executor = Executor::new();
    let code = r#"
        "literal".upper!()
    "#;

    let result = executor.execute_source(code);
    assert!(result.is_err(), "upper!() should only work on variables");
    assert!(result.unwrap_err().to_string().contains("requires a variable"));
}

#[test]
fn test_lower_mutating_returns_none() {
    let mut executor = Executor::new();
    let code = r#"
        text = "HELLO"
        result = text.lower!()
    "#;

    executor.execute_source(code).unwrap();

    // Verify lower!() returned none
    let result_val = executor.env().get("result").unwrap();
    assert!(matches!(&result_val.kind, ValueKind::None), "lower!() should return none");
}

#[test]
fn test_lower_mutating_modifies_variable() {
    let mut executor = Executor::new();
    let code = r#"
        text = "HELLO WORLD"
        text.lower!()
    "#;

    executor.execute_source(code).unwrap();

    // Verify mutation happened
    let text_val = executor.env().get("text").unwrap();
    assert!(matches!(&text_val.kind, ValueKind::String(s) if s == "hello world"));
}

#[test]
fn test_lower_mutating_no_arguments() {
    let mut executor = Executor::new();
    let code = r#"
        text = "HELLO"
        text.lower!(42)
    "#;

    let result = executor.execute_source(code);
    assert!(result.is_err(), "lower!() should reject arguments");
    assert!(result.unwrap_err().to_string().contains("takes no arguments"));
}

#[test]
fn test_lower_mutating_requires_variable() {
    let mut executor = Executor::new();
    let code = r#"
        "LITERAL".lower!()
    "#;

    let result = executor.execute_source(code);
    assert!(result.is_err(), "lower!() should only work on variables");
    assert!(result.unwrap_err().to_string().contains("requires a variable"));
}

#[test]
fn test_trim_mutating_returns_none() {
    let mut executor = Executor::new();
    let code = r#"
        text = "  hello  "
        result = text.trim!()
    "#;

    executor.execute_source(code).unwrap();

    // Verify trim!() returned none
    let result_val = executor.env().get("result").unwrap();
    assert!(matches!(&result_val.kind, ValueKind::None), "trim!() should return none");
}

#[test]
fn test_trim_mutating_modifies_variable() {
    let mut executor = Executor::new();
    let code = r#"
        text = "  spaces around  "
        text.trim!()
    "#;

    executor.execute_source(code).unwrap();

    // Verify mutation happened
    let text_val = executor.env().get("text").unwrap();
    assert!(matches!(&text_val.kind, ValueKind::String(s) if s == "spaces around"));
}

#[test]
fn test_trim_mutating_handles_tabs_newlines() {
    let mut executor = Executor::new();
    let code = "text = \"\t\nhello\n\t\"\ntext.trim!()";

    executor.execute_source(code).unwrap();

    // Verify mutation happened
    let text_val = executor.env().get("text").unwrap();
    assert!(matches!(&text_val.kind, ValueKind::String(s) if s == "hello"));
}

#[test]
fn test_trim_mutating_no_arguments() {
    let mut executor = Executor::new();
    let code = r#"
        text = "  hello  "
        text.trim!(42)
    "#;

    let result = executor.execute_source(code);
    assert!(result.is_err(), "trim!() should reject arguments");
    assert!(result.unwrap_err().to_string().contains("takes no arguments"));
}

#[test]
fn test_trim_mutating_requires_variable() {
    let mut executor = Executor::new();
    let code = r#"
        "  literal  ".trim!()
    "#;

    let result = executor.execute_source(code);
    assert!(result.is_err(), "trim!() should only work on variables");
    assert!(result.unwrap_err().to_string().contains("requires a variable"));
}

#[test]
fn test_reverse_mutating_returns_none() {
    let mut executor = Executor::new();
    let code = r#"
        text = "hello"
        result = text.reverse!()
    "#;

    executor.execute_source(code).unwrap();

    // Verify reverse!() returned none
    let result_val = executor.env().get("result").unwrap();
    assert!(matches!(&result_val.kind, ValueKind::None), "reverse!() should return none");
}

#[test]
fn test_reverse_mutating_modifies_variable() {
    let mut executor = Executor::new();
    let code = r#"
        text = "hello"
        text.reverse!()
    "#;

    executor.execute_source(code).unwrap();

    // Verify mutation happened
    let text_val = executor.env().get("text").unwrap();
    assert!(matches!(&text_val.kind, ValueKind::String(s) if s == "olleh"));
}

#[test]
fn test_reverse_mutating_handles_unicode() {
    let mut executor = Executor::new();
    let code = r#"
        text = "hello 🌍"
        text.reverse!()
    "#;

    executor.execute_source(code).unwrap();

    // Verify mutation happened with proper unicode handling
    let text_val = executor.env().get("text").unwrap();
    assert!(matches!(&text_val.kind, ValueKind::String(s) if s == "🌍 olleh"));
}

#[test]
fn test_reverse_mutating_no_arguments() {
    let mut executor = Executor::new();
    let code = r#"
        text = "hello"
        text.reverse!(42)
    "#;

    let result = executor.execute_source(code);
    assert!(result.is_err(), "reverse!() should reject arguments");
    assert!(result.unwrap_err().to_string().contains("takes no arguments"));
}

#[test]
fn test_reverse_mutating_requires_variable() {
    let mut executor = Executor::new();
    let code = r#"
        "literal".reverse!()
    "#;

    let result = executor.execute_source(code);
    assert!(result.is_err(), "reverse!() should only work on variables");
    assert!(result.unwrap_err().to_string().contains("requires a variable"));
}

#[test]
fn test_mutating_methods_chain_behavior() {
    let mut executor = Executor::new();
    let code = r#"
        text = "  hello world  "
        text.trim!()
        text.upper!()
    "#;

    executor.execute_source(code).unwrap();

    // Verify both mutations happened in sequence
    let text_val = executor.env().get("text").unwrap();
    assert!(matches!(&text_val.kind, ValueKind::String(s) if s == "HELLO WORLD"));
}

#[test]
fn test_immutable_vs_mutating_methods() {
    let mut executor = Executor::new();
    let code = r#"
        original = "hello"

        # Immutable method - returns new value
        upper_result = original.upper()

        # Mutating method - modifies in place
        original.reverse!()
    "#;

    executor.execute_source(code).unwrap();

    // Immutable method returned new value
    let upper_result = executor.env().get("upper_result").unwrap();
    assert!(matches!(&upper_result.kind, ValueKind::String(s) if s == "HELLO"));

    // Mutating method changed original
    let original_val = executor.env().get("original").unwrap();
    assert!(matches!(&original_val.kind, ValueKind::String(s) if s == "olleh"));
}

// ============================================================================
// Phase 3: Advanced Pattern Methods Tests
// ============================================================================

// contains(mode, patterns...) tests

#[test]
fn test_contains_any_single_pattern() {
    let mut executor = Executor::new();
    let code = r#"
        result = "Hello123".contains(:any, :digits)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(true)));
}

#[test]
fn test_contains_any_multiple_patterns() {
    let mut executor = Executor::new();
    let code = r#"
        result = "Hello".contains(:any, :digits, :letters)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(true)));
}

#[test]
fn test_contains_any_no_match() {
    let mut executor = Executor::new();
    let code = r#"
        result = "Hello".contains(:any, :digits)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(false)));
}

#[test]
fn test_contains_all_mode() {
    let mut executor = Executor::new();
    let code = r#"
        result = "Hello123".contains(:all, :letters, :digits)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(true)));
}

#[test]
fn test_contains_all_mode_missing_pattern() {
    let mut executor = Executor::new();
    let code = r#"
        result = "Hello".contains(:all, :letters, :digits)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(false)));
}

#[test]
fn test_contains_only_mode_pure() {
    let mut executor = Executor::new();
    let code = r#"
        result = "abc".contains(:only, :letters)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(true)));
}

#[test]
fn test_contains_only_mode_with_spaces() {
    let mut executor = Executor::new();
    let code = r#"
        result = "Hello World".contains(:only, :letters, :spaces)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(true)));
}

#[test]
fn test_contains_only_mode_has_extra() {
    let mut executor = Executor::new();
    let code = r#"
        result = "Hello123!".contains(:only, :letters, :spaces)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(false)));
}

#[test]
fn test_contains_pattern_aliases() {
    let mut executor = Executor::new();
    let code = r#"
        result1 = "123".contains(:any, :numbers)
        result2 = "   ".contains(:any, :whitespace)
    "#;

    executor.execute_source(code).unwrap();
    let result1 = executor.env().get("result1").unwrap();
    let result2 = executor.env().get("result2").unwrap();
    assert!(matches!(&result1.kind, ValueKind::Boolean(true)));
    assert!(matches!(&result2.kind, ValueKind::Boolean(true)));
}

#[test]
fn test_contains_uppercase_lowercase_patterns() {
    let mut executor = Executor::new();
    let code = r#"
        result1 = "ABC".contains(:any, :uppercase)
        result2 = "abc".contains(:any, :lowercase)
        result3 = "aBc".contains(:all, :uppercase, :lowercase)
    "#;

    executor.execute_source(code).unwrap();
    let result1 = executor.env().get("result1").unwrap();
    let result2 = executor.env().get("result2").unwrap();
    let result3 = executor.env().get("result3").unwrap();
    assert!(matches!(&result1.kind, ValueKind::Boolean(true)));
    assert!(matches!(&result2.kind, ValueKind::Boolean(true)));
    assert!(matches!(&result3.kind, ValueKind::Boolean(true)));
}

#[test]
fn test_contains_punctuation_pattern() {
    let mut executor = Executor::new();
    let code = r#"
        result = "Hello, World!".contains(:any, :punctuation)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(true)));
}

#[test]
fn test_contains_symbols_pattern() {
    let mut executor = Executor::new();
    let code = r#"
        result = "Hello@World".contains(:any, :symbols)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(true)));
}

// extract(pattern) tests

#[test]
fn test_extract_numbers() {
    let mut executor = Executor::new();
    let code = r#"
        result = "Hello 123 World 456".extract(:numbers)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    if let ValueKind::List(list) = &result.kind {
        let items = list.to_vec();
        assert_eq!(items.len(), 2);
        assert!(matches!(&items[0].kind, ValueKind::String(s) if s == "123"));
        assert!(matches!(&items[1].kind, ValueKind::String(s) if s == "456"));
    } else {
        panic!("Expected list");
    }
}

#[test]
fn test_extract_letters() {
    let mut executor = Executor::new();
    let code = r#"
        result = "one, two, three".extract(:letters)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    if let ValueKind::List(list) = &result.kind {
        let items = list.to_vec();
        assert_eq!(items.len(), 3);
        assert!(matches!(&items[0].kind, ValueKind::String(s) if s == "one"));
        assert!(matches!(&items[1].kind, ValueKind::String(s) if s == "two"));
        assert!(matches!(&items[2].kind, ValueKind::String(s) if s == "three"));
    } else {
        panic!("Expected list");
    }
}

#[test]
fn test_extract_words() {
    let mut executor = Executor::new();
    let code = r#"
        result = "Hello World Test".extract(:words)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    if let ValueKind::List(list) = &result.kind {
        let items = list.to_vec();
        assert_eq!(items.len(), 3);
        assert!(matches!(&items[0].kind, ValueKind::String(s) if s == "Hello"));
        assert!(matches!(&items[1].kind, ValueKind::String(s) if s == "World"));
        assert!(matches!(&items[2].kind, ValueKind::String(s) if s == "Test"));
    } else {
        panic!("Expected list");
    }
}

#[test]
fn test_extract_emails() {
    let mut executor = Executor::new();
    let code = r#"
        result = "Contact test@example.com or hello@world.org".extract(:emails)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    if let ValueKind::List(list) = &result.kind {
        let items = list.to_vec();
        assert_eq!(items.len(), 2);
        assert!(matches!(&items[0].kind, ValueKind::String(s) if s == "test@example.com"));
        assert!(matches!(&items[1].kind, ValueKind::String(s) if s == "hello@world.org"));
    } else {
        panic!("Expected list");
    }
}

#[test]
fn test_extract_empty_result() {
    let mut executor = Executor::new();
    let code = r#"
        result = "no numbers here".extract(:digits)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    if let ValueKind::List(list) = &result.kind {
        assert_eq!(list.to_vec().len(), 0);
    } else {
        panic!("Expected list");
    }
}

// count(pattern) tests

#[test]
fn test_count_digits() {
    let mut executor = Executor::new();
    let code = r#"
        result = "Hello 123 World".count(:digits)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 3.0));
}

#[test]
fn test_count_words() {
    let mut executor = Executor::new();
    let code = r#"
        result = "Hello World Test".count(:words)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 3.0));
}

#[test]
fn test_count_emails() {
    let mut executor = Executor::new();
    let code = r#"
        result = "Email test@example.com and hello@world.org".count(:emails)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 2.0));
}

// find(pattern, options...) tests

#[test]
fn test_find_digits_all_positions() {
    let mut executor = Executor::new();
    let code = r#"
        result = "Hello123".find(:digits)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    if let ValueKind::List(list) = &result.kind {
        let items = list.to_vec();
        assert_eq!(items.len(), 3);
        assert!(matches!(&items[0].kind, ValueKind::Number(n) if *n == 5.0));
        assert!(matches!(&items[1].kind, ValueKind::Number(n) if *n == 6.0));
        assert!(matches!(&items[2].kind, ValueKind::Number(n) if *n == 7.0));
    } else {
        panic!("Expected list");
    }
}

#[test]
fn test_find_digits_with_limit() {
    let mut executor = Executor::new();
    let code = r#"
        result = "Hello123".find(:digits, 2)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    if let ValueKind::List(list) = &result.kind {
        let items = list.to_vec();
        assert_eq!(items.len(), 2);
        assert!(matches!(&items[0].kind, ValueKind::Number(n) if *n == 5.0));
        assert!(matches!(&items[1].kind, ValueKind::Number(n) if *n == 6.0));
    } else {
        panic!("Expected list");
    }
}

#[test]
fn test_find_digits_first_mode() {
    let mut executor = Executor::new();
    let code = r#"
        result = "Hello123".find(:digits, :first)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 5.0));
}

#[test]
fn test_find_not_found() {
    let mut executor = Executor::new();
    let code = r#"
        result = "Hello".find(:digits, :first)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == -1.0));
}

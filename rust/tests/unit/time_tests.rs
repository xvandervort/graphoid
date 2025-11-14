use graphoid::execution::Executor;
use graphoid::values::ValueKind;

// ============================================================================
// Time Type Tests
// ============================================================================

// ============================================================================
// Static Constructor Tests
// ============================================================================

#[test]
fn test_time_now() {
    let mut executor = Executor::new();
    let code = r#"
        t = time.now()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("t").unwrap();
    // Should be a time value
    assert!(matches!(&result.kind, ValueKind::Time(_)));
}

#[test]
fn test_time_today() {
    let mut executor = Executor::new();
    let code = r#"
        t = time.today()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("t").unwrap();
    assert!(matches!(&result.kind, ValueKind::Time(_)));
}

#[test]
fn test_time_from_numbers() {
    let mut executor = Executor::new();
    let code = r#"
        t = time.from_numbers(2025, 1, 15, 14, 30, 45)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("t").unwrap();
    assert!(matches!(&result.kind, ValueKind::Time(_)));
}

#[test]
fn test_time_from_numbers_invalid() {
    let mut executor = Executor::new();
    let code = r#"
        t = time.from_numbers(2025, 13, 32, 25, 61, 61)
    "#;

    let result = executor.execute_source(code);
    assert!(result.is_err());
}

#[test]
fn test_time_from_string_iso8601() {
    let mut executor = Executor::new();
    let code = r#"
        t = time.from_string("2025-01-15T14:30:45Z")
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("t").unwrap();
    assert!(matches!(&result.kind, ValueKind::Time(_)));
}

#[test]
fn test_time_from_string_with_timezone() {
    let mut executor = Executor::new();
    let code = r#"
        t = time.from_string("2025-01-15T14:30:45+05:30")
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("t").unwrap();
    assert!(matches!(&result.kind, ValueKind::Time(_)));
}

#[test]
fn test_time_from_string_invalid() {
    let mut executor = Executor::new();
    let code = r#"
        t = time.from_string("not a date")
    "#;

    let result = executor.execute_source(code);
    assert!(result.is_err());
}

#[test]
fn test_time_from_timestamp() {
    let mut executor = Executor::new();
    let code = r#"
        t = time.from_timestamp(1704067200)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("t").unwrap();
    assert!(matches!(&result.kind, ValueKind::Time(ts) if *ts == 1704067200.0));
}

#[test]
fn test_time_from_timestamp_with_decimals() {
    let mut executor = Executor::new();
    let code = r#"
        t = time.from_timestamp(1704067200.5)
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("t").unwrap();
    assert!(matches!(&result.kind, ValueKind::Time(ts) if *ts == 1704067200.5));
}

// ============================================================================
// Instance Method Tests
// ============================================================================

#[test]
fn test_time_numbers() {
    let mut executor = Executor::new();
    let code = r#"
        t = time.from_numbers(2025, 1, 15, 14, 30, 45)
        components = t.time_numbers()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("components").unwrap();

    // Should be a hash
    if let ValueKind::Map(hash) = &result.kind {
        // Check year
        let year = hash.get("year").unwrap();
        assert!(matches!(&year.kind, ValueKind::Number(n) if *n == 2025.0));

        // Check month
        let month = hash.get("month").unwrap();
        assert!(matches!(&month.kind, ValueKind::Number(n) if *n == 1.0));

        // Check day
        let day = hash.get("day").unwrap();
        assert!(matches!(&day.kind, ValueKind::Number(n) if *n == 15.0));

        // Check hour
        let hour = hash.get("hour").unwrap();
        assert!(matches!(&hour.kind, ValueKind::Number(n) if *n == 14.0));

        // Check minute
        let minute = hash.get("minute").unwrap();
        assert!(matches!(&minute.kind, ValueKind::Number(n) if *n == 30.0));

        // Check second
        let second = hash.get("second").unwrap();
        assert!(matches!(&second.kind, ValueKind::Number(n) if *n == 45.0));

        // Check weekday exists
        assert!(hash.get("weekday").is_some());

        // Check day_of_year exists
        assert!(hash.get("day_of_year").is_some());
    } else {
        panic!("Expected hash, got {:?}", result.kind);
    }
}

// ============================================================================
// Universal Casting Tests for Time
// ============================================================================

#[test]
fn test_time_to_num() {
    let mut executor = Executor::new();
    let code = r#"
        t = time.from_timestamp(1704067200)
        timestamp = t.to_num()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("timestamp").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 1704067200.0));
}

#[test]
fn test_time_to_string() {
    let mut executor = Executor::new();
    let code = r#"
        t = time.from_numbers(2025, 1, 15, 14, 30, 45)
        iso_string = t.to_string()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("iso_string").unwrap();

    if let ValueKind::String(s) = &result.kind {
        // Should be in ISO 8601 format
        assert!(s.contains("2025-01-15"));
        assert!(s.contains("14:30:45"));
    } else {
        panic!("Expected string, got {:?}", result.kind);
    }
}

#[test]
fn test_time_to_bool() {
    let mut executor = Executor::new();
    let code = r#"
        t = time.now()
        is_truthy = t.to_bool()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("is_truthy").unwrap();
    // Time values are always truthy
    assert!(matches!(&result.kind, ValueKind::Boolean(b) if *b));
}

// ============================================================================
// Round-trip Conversion Tests
// ============================================================================

#[test]
fn test_time_roundtrip_timestamp() {
    let mut executor = Executor::new();
    let code = r#"
        original_ts = 1704067200.5
        t = time.from_timestamp(original_ts)
        recovered_ts = t.to_num()
    "#;

    executor.execute_source(code).unwrap();
    let original = executor.env().get("original_ts").unwrap();
    let recovered = executor.env().get("recovered_ts").unwrap();

    if let (ValueKind::Number(orig), ValueKind::Number(rec)) = (&original.kind, &recovered.kind) {
        assert_eq!(orig, rec);
    } else {
        panic!("Expected numbers");
    }
}

#[test]
fn test_time_roundtrip_string() {
    let mut executor = Executor::new();
    let code = r#"
        original_str = "2025-01-15T14:30:45Z"
        t = time.from_string(original_str)
        recovered_str = t.to_string()
    "#;

    executor.execute_source(code).unwrap();
    let recovered = executor.env().get("recovered_str").unwrap();

    if let ValueKind::String(s) = &recovered.kind {
        // The recovered string should contain the same date/time (might have slight formatting differences)
        assert!(s.contains("2025-01-15"));
        assert!(s.contains("14:30:45"));
    } else {
        panic!("Expected string, got {:?}", recovered.kind);
    }
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_time_epoch() {
    let mut executor = Executor::new();
    let code = r#"
        t = time.from_timestamp(0)
        components = t.time_numbers()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("components").unwrap();

    if let ValueKind::Map(hash) = &result.kind {
        let year = hash.get("year").unwrap();
        assert!(matches!(&year.kind, ValueKind::Number(n) if *n == 1970.0));

        let month = hash.get("month").unwrap();
        assert!(matches!(&month.kind, ValueKind::Number(n) if *n == 1.0));

        let day = hash.get("day").unwrap();
        assert!(matches!(&day.kind, ValueKind::Number(n) if *n == 1.0));
    } else {
        panic!("Expected hash");
    }
}

#[test]
fn test_time_print() {
    let mut executor = Executor::new();
    let code = r#"
        t = time.from_numbers(2025, 1, 15, 14, 30, 45)
        print(t)
    "#;

    // Should not error
    executor.execute_source(code).unwrap();
}

#[test]
fn test_time_type_name() {
    let mut executor = Executor::new();
    let code = r#"
        t = time.now()
        type_name = t.type()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("type_name").unwrap();
    assert!(matches!(&result.kind, ValueKind::String(s) if s == "time"));
}

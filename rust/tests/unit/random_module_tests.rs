use graphoid::stdlib::{NativeModule, random::RandomModule};
use graphoid::values::{Value, ValueKind};

#[test]
fn test_random_module_name() {
    let module = RandomModule::new();
    assert_eq!(module.name(), "random");
}

#[test]
fn test_random_module_alias() {
    let module = RandomModule::new();
    assert_eq!(module.alias(), Some("rand"));
}

#[test]
fn test_random_has_random_function() {
    let module = RandomModule::new();
    let functions = module.functions();
    assert!(functions.contains_key("random"), "Should have random() function");
}

#[test]
fn test_random_returns_value_in_range() {
    let module = RandomModule::new();
    let functions = module.functions();
    let random_fn = functions.get("random").expect("Should have random function");

    // Call random() 100 times to verify range
    for _ in 0..100 {
        let result = random_fn(&[]).expect("Should return value");
        match &result.kind {
            ValueKind::Number(n) => {
                assert!(*n >= 0.0 && *n < 1.0, "random() should return [0.0, 1.0)");
            }
            _ => panic!("random() should return a number"),
        }
    }
}

#[test]
fn test_randint_function_exists() {
    let module = RandomModule::new();
    let functions = module.functions();
    assert!(functions.contains_key("randint"), "Should have randint() function");
}

#[test]
fn test_randint_returns_integer_in_range() {
    let module = RandomModule::new();
    let functions = module.functions();
    let randint_fn = functions.get("randint").expect("Should have randint function");

    // Test randint(1, 10) returns integers in [1, 10]
    let min = Value::number(1.0);
    let max = Value::number(10.0);

    for _ in 0..50 {
        let result = randint_fn(&[min.clone(), max.clone()]).expect("Should return value");
        match &result.kind {
            ValueKind::Number(n) => {
                assert!(*n >= 1.0 && *n <= 10.0, "randint(1, 10) should return [1, 10]");
                assert!(n.fract() == 0.0, "randint should return integers");
            }
            _ => panic!("randint() should return a number"),
        }
    }
}

#[test]
fn test_uniform_function_exists() {
    let module = RandomModule::new();
    let functions = module.functions();
    assert!(functions.contains_key("uniform"), "Should have uniform() function");
}

#[test]
fn test_uniform_returns_float_in_range() {
    let module = RandomModule::new();
    let functions = module.functions();
    let uniform_fn = functions.get("uniform").expect("Should have uniform function");

    let min = Value::number(5.0);
    let max = Value::number(15.0);

    for _ in 0..50 {
        let result = uniform_fn(&[min.clone(), max.clone()]).expect("Should return value");
        match &result.kind {
            ValueKind::Number(n) => {
                assert!(*n >= 5.0 && *n < 15.0, "uniform(5, 15) should return [5.0, 15.0)");
            }
            _ => panic!("uniform() should return a number"),
        }
    }
}

#[test]
fn test_choice_function_exists() {
    let module = RandomModule::new();
    let functions = module.functions();
    assert!(functions.contains_key("choice"), "Should have choice() function");
}

#[test]
fn test_uuid4_function_exists() {
    let module = RandomModule::new();
    let functions = module.functions();
    assert!(functions.contains_key("uuid4"), "Should have uuid4() function");
}

#[test]
fn test_uuid4_returns_valid_format() {
    let module = RandomModule::new();
    let functions = module.functions();
    let uuid4_fn = functions.get("uuid4").expect("Should have uuid4 function");

    let result = uuid4_fn(&[]).expect("Should return value");
    match &result.kind {
        ValueKind::String(s) => {
            // UUID v4 format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
            assert_eq!(s.len(), 36, "UUID should be 36 characters");
            assert_eq!(s.chars().nth(8), Some('-'), "UUID should have dash at position 8");
            assert_eq!(s.chars().nth(13), Some('-'), "UUID should have dash at position 13");
            assert_eq!(s.chars().nth(14), Some('4'), "UUID v4 should have '4' at position 14");
            assert_eq!(s.chars().nth(18), Some('-'), "UUID should have dash at position 18");
            assert_eq!(s.chars().nth(23), Some('-'), "UUID should have dash at position 23");
        }
        _ => panic!("uuid4() should return a string"),
    }
}

#[test]
fn test_seed_function_exists() {
    let module = RandomModule::new();
    let functions = module.functions();
    assert!(functions.contains_key("seed"), "Should have seed() function");
}

#[test]
fn test_seed_affects_det_random() {
    let module = RandomModule::new();
    let functions = module.functions();
    let seed_fn = functions.get("seed").expect("Should have seed function");
    let det_random_fn = functions.get("det_random").expect("Should have det_random function");

    // Seed with 42
    seed_fn(&[Value::number(42.0)]).expect("Seed should work");
    let first = det_random_fn(&[]).expect("Should return value");

    // Seed again with 42
    seed_fn(&[Value::number(42.0)]).expect("Seed should work");
    let second = det_random_fn(&[]).expect("Should return value");

    // Should get same values with same seed
    match (&first.kind, &second.kind) {
        (ValueKind::Number(n1), ValueKind::Number(n2)) => {
            assert_eq!(n1, n2, "Same seed should produce same deterministic values");
        }
        _ => panic!("det_random() should return numbers"),
    }
}

#[test]
fn test_token_function_exists() {
    let module = RandomModule::new();
    let functions = module.functions();
    assert!(functions.contains_key("token"), "Should have token() function");
}

#[test]
fn test_token_returns_hex_string() {
    let module = RandomModule::new();
    let functions = module.functions();
    let token_fn = functions.get("token").expect("Should have token function");

    let result = token_fn(&[Value::number(16.0)]).expect("Should return value");
    match &result.kind {
        ValueKind::String(s) => {
            assert_eq!(s.len(), 32, "16 bytes = 32 hex characters");
            assert!(s.chars().all(|c| c.is_ascii_hexdigit()), "Token should be hex");
        }
        _ => panic!("token() should return a string"),
    }
}

#[test]
fn test_normal_function_exists() {
    let module = RandomModule::new();
    let functions = module.functions();
    assert!(functions.contains_key("normal"), "Should have normal() function");
}

#[test]
fn test_normal_distribution_approximate_mean() {
    let module = RandomModule::new();
    let functions = module.functions();
    let normal_fn = functions.get("normal").expect("Should have normal function");

    let mean = 100.0;
    let std_dev = 15.0;

    // Generate 1000 samples
    let mut sum = 0.0;
    let samples = 1000;
    for _ in 0..samples {
        let result = normal_fn(&[Value::number(mean), Value::number(std_dev)])
            .expect("Should return value");
        match &result.kind {
            ValueKind::Number(n) => sum += n,
            _ => panic!("normal() should return a number"),
        }
    }

    let observed_mean = sum / (samples as f64);
    // Allow 5% error margin for statistical test
    let error = (observed_mean - mean).abs();
    assert!(error < mean * 0.05, "Mean should be approximately {}, got {}", mean, observed_mean);
}

#[test]
fn test_exponential_function_exists() {
    let module = RandomModule::new();
    let functions = module.functions();
    assert!(functions.contains_key("exponential"), "Should have exponential() function");
}

#[test]
fn test_shuffle_function_exists() {
    let module = RandomModule::new();
    let functions = module.functions();
    assert!(functions.contains_key("shuffle"), "Should have shuffle() function");
}

#[test]
fn test_sample_function_exists() {
    let module = RandomModule::new();
    let functions = module.functions();
    assert!(functions.contains_key("sample"), "Should have sample() function");
}

#[test]
fn test_det_randint_function_exists() {
    let module = RandomModule::new();
    let functions = module.functions();
    assert!(functions.contains_key("det_randint"), "Should have det_randint() function");
}

#[test]
fn test_det_random_function_exists() {
    let module = RandomModule::new();
    let functions = module.functions();
    assert!(functions.contains_key("det_random"), "Should have det_random() function");
}

#[test]
fn test_token_urlsafe_function_exists() {
    let module = RandomModule::new();
    let functions = module.functions();
    assert!(functions.contains_key("token_urlsafe"), "Should have token_urlsafe() function");
}

#[test]
fn test_token_urlsafe_returns_safe_string() {
    let module = RandomModule::new();
    let functions = module.functions();
    let token_fn = functions.get("token_urlsafe").expect("Should have token_urlsafe function");

    let result = token_fn(&[Value::number(16.0)]).expect("Should return value");
    match &result.kind {
        ValueKind::String(s) => {
            assert!(!s.is_empty(), "Token should not be empty");
            // URL-safe chars: A-Z, a-z, 0-9, -, _
            assert!(s.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_'),
                    "Token should be URL-safe");
        }
        _ => panic!("token_urlsafe() should return a string"),
    }
}

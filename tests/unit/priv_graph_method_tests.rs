//! Tests for priv fn inside graphs — both internal calling and external blocking.

use graphoid::execution_graph::graph_executor::GraphExecutor;
use graphoid::values::Value;

fn exec(source: &str) -> GraphExecutor {
    let mut executor = GraphExecutor::new();
    executor.execute_source(source).unwrap();
    executor
}

fn exec_err(source: &str) -> String {
    let mut executor = GraphExecutor::new();
    match executor.execute_source(source) {
        Ok(_) => panic!("Expected error"),
        Err(e) => format!("{}", e),
    }
}

// =========================================================================
// Bug fix: priv fn callable from sibling methods within the same graph
// =========================================================================

#[test]
fn test_priv_fn_callable_internally() {
    let e = exec(r#"
graph Counter {
    count: 0

    priv fn validate(n) {
        if n < 0 { return 0 }
        return n
    }

    fn increment(amount) {
        count = count + validate(amount)
    }

    fn get_count() {
        return count
    }
}

c = Counter{}
c.increment(5)
result = c.get_count()
"#);
    assert_eq!(e.get_variable("result").unwrap(), Value::number(5.0));
}

#[test]
fn test_priv_fn_validates_internally() {
    let e = exec(r#"
graph Counter {
    count: 0

    priv fn validate(n) {
        if n < 0 { return 0 }
        return n
    }

    fn increment(amount) {
        count = count + validate(amount)
    }

    fn get_count() {
        return count
    }
}

c = Counter{}
c.increment(-3)
result = c.get_count()
"#);
    assert_eq!(e.get_variable("result").unwrap(), Value::number(0.0));
}

// =========================================================================
// priv fn NOT callable from outside the graph
// =========================================================================

#[test]
fn test_priv_fn_not_callable_externally() {
    let err = exec_err(r#"
graph Foo {
    priv fn secret() {
        return 42
    }
}

f = Foo{}
f.secret()
"#);
    assert!(err.contains("private") || err.contains("does not have method"),
            "Expected privacy error, got: {}", err);
}

// =========================================================================
// Public methods remain callable
// =========================================================================

#[test]
fn test_public_fn_still_works() {
    let e = exec(r#"
graph Foo {
    priv fn helper() {
        return 10
    }

    fn get_value() {
        return helper() + 5
    }
}

f = Foo{}
result = f.get_value()
"#);
    assert_eq!(e.get_variable("result").unwrap(), Value::number(15.0));
}

// =========================================================================
// Mixed public and private methods
// =========================================================================

#[test]
fn test_mixed_public_private_methods() {
    let e = exec(r#"
graph Calculator {
    priv fn square(n) {
        return n * n
    }

    priv fn cube(n) {
        return n * n * n
    }

    fn power(n, exp) {
        return match exp {
            2 => square(n),
            3 => cube(n),
            _ => n
        }
    }
}

c = Calculator{}
r1 = c.power(5, 2)
r2 = c.power(3, 3)
r3 = c.power(7, 1)
"#);
    assert_eq!(e.get_variable("r1").unwrap(), Value::number(25.0));
    assert_eq!(e.get_variable("r2").unwrap(), Value::number(27.0));
    assert_eq!(e.get_variable("r3").unwrap(), Value::number(7.0));
}

// =========================================================================
// priv { } block syntax in graph body
// =========================================================================

#[test]
fn test_priv_block_syntax_in_graph() {
    let e = exec(r#"
graph Foo {
    priv {
        fn helper_a() {
            return 10
        }

        fn helper_b() {
            return 20
        }
    }

    fn total() {
        return helper_a() + helper_b()
    }
}

f = Foo{}
result = f.total()
"#);
    assert_eq!(e.get_variable("result").unwrap(), Value::number(30.0));
}

#[test]
fn test_priv_block_methods_not_callable_externally() {
    let err = exec_err(r#"
graph Foo {
    priv {
        fn secret() {
            return 42
        }
    }
}

f = Foo{}
f.secret()
"#);
    assert!(err.contains("private") || err.contains("does not have method"),
            "Expected privacy error, got: {}", err);
}

// =========================================================================
// Private method calling another private method
// =========================================================================

#[test]
fn test_priv_fn_calls_another_priv_fn() {
    let e = exec(r#"
graph Foo {
    priv fn step1() {
        return 10
    }

    priv fn step2() {
        return step1() + 5
    }

    fn compute() {
        return step2() * 2
    }
}

f = Foo{}
result = f.compute()
"#);
    assert_eq!(e.get_variable("result").unwrap(), Value::number(30.0));
}

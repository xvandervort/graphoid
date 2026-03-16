//! Phase 19.2: Signal tests — signal.on()

use graphoid::execution_graph::graph_executor::GraphExecutor;

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
// signal.on() — returns channel
// =========================================================================

#[test]
fn test_signal_on_sigint_returns_channel() {
    let executor = exec(r#"
        ch = signal.on(:sigint)
    "#);
    let val = executor.get_variable("ch").unwrap();
    assert_eq!(val.type_name(), "channel");
}

#[test]
fn test_signal_on_sigterm_returns_channel() {
    let executor = exec(r#"
        ch = signal.on(:sigterm)
    "#);
    let val = executor.get_variable("ch").unwrap();
    assert_eq!(val.type_name(), "channel");
}

#[test]
fn test_signal_on_sighup_returns_channel() {
    let executor = exec(r#"
        ch = signal.on(:sighup)
    "#);
    let val = executor.get_variable("ch").unwrap();
    assert_eq!(val.type_name(), "channel");
}

// =========================================================================
// signal.on() — error cases
// =========================================================================

#[test]
fn test_signal_on_invalid_signal_raises() {
    let err = exec_err("signal.on(:invalid)");
    assert!(err.contains("Unknown signal") || err.contains("invalid"),
        "Expected unknown signal error, got: {}", err);
}

#[test]
fn test_signal_on_no_args_raises() {
    let err = exec_err("signal.on()");
    assert!(err.contains("argument") || err.contains("requires"),
        "Expected arg error, got: {}", err);
}

#[test]
fn test_signal_on_string_raises() {
    let err = exec_err(r#"signal.on("sigint")"#);
    assert!(err.contains("symbol") || err.contains("type"),
        "Expected type error, got: {}", err);
}

#[test]
fn test_signal_on_number_raises() {
    let err = exec_err("signal.on(2)");
    assert!(err.contains("symbol") || err.contains("type"),
        "Expected type error, got: {}", err);
}

#[test]
fn test_signal_unknown_method_raises() {
    let err = exec_err("signal.off(:sigint)");
    assert!(err.contains("no method") || err.contains("not defined"),
        "Expected unknown method error, got: {}", err);
}

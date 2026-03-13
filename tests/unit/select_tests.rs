//! Unit tests for Phase 19.5: select() channel multiplexing

use graphoid::execution_graph::graph_executor::GraphExecutor;
use graphoid::values::Value;
use graphoid::values::Channel;

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

// ============================================================
// Channel identity comparison
// ============================================================

#[test]
fn test_channel_identity_same() {
    let ch = Channel::new(Some(10));
    let ch_clone = ch.clone();
    assert_eq!(ch, ch_clone);
}

#[test]
fn test_channel_identity_different() {
    let ch1 = Channel::new(Some(10));
    let ch2 = Channel::new(Some(10));
    assert_ne!(ch1, ch2);
}

#[test]
fn test_channel_value_equality() {
    let executor = exec(r#"
        ch = channel(10)
        result = ch == ch
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::boolean(true));
}

#[test]
fn test_channel_value_inequality() {
    let executor = exec(r#"
        ch1 = channel(10)
        ch2 = channel(10)
        result = ch1 == ch2
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::boolean(false));
}

// ============================================================
// Basic select()
// ============================================================

#[test]
fn test_select_returns_list() {
    let executor = exec(r#"
        ch = channel(1)
        ch.send(42)
        result = select(ch)
        t = result.type()
    "#);
    assert_eq!(executor.get_variable("t").unwrap(), Value::string("list".to_string()));
}

#[test]
fn test_select_returns_two_elements() {
    let executor = exec(r#"
        ch = channel(1)
        ch.send(42)
        result = select(ch)
        len = result.length()
    "#);
    assert_eq!(executor.get_variable("len").unwrap(), Value::number(2.0));
}

#[test]
fn test_select_single_channel() {
    let executor = exec(r#"
        ch = channel(1)
        ch.send("hello")
        result = select(ch)
        source = result[0]
        msg = result[1]
        same_ch = source == ch
    "#);
    assert_eq!(executor.get_variable("msg").unwrap(), Value::string("hello".to_string()));
    assert_eq!(executor.get_variable("same_ch").unwrap(), Value::boolean(true));
}

#[test]
fn test_select_picks_ready_channel() {
    let executor = exec(r#"
        ch1 = channel(1)
        ch2 = channel(1)
        ch2.send("from_ch2")
        result = select(ch1, ch2)
        source = result[0]
        msg = result[1]
        from_ch2 = source == ch2
    "#);
    assert_eq!(executor.get_variable("msg").unwrap(), Value::string("from_ch2".to_string()));
    assert_eq!(executor.get_variable("from_ch2").unwrap(), Value::boolean(true));
}

#[test]
fn test_select_source_identity() {
    let executor = exec(r#"
        ch1 = channel(1)
        ch2 = channel(1)
        ch1.send(99)
        result = select(ch1, ch2)
        source = result[0]
        is_ch1 = source == ch1
        is_ch2 = source == ch2
    "#);
    assert_eq!(executor.get_variable("is_ch1").unwrap(), Value::boolean(true));
    assert_eq!(executor.get_variable("is_ch2").unwrap(), Value::boolean(false));
}

// ============================================================
// select() with default
// ============================================================

#[test]
fn test_select_default_nothing_ready() {
    let executor = exec(r#"
        ch1 = channel(1)
        ch2 = channel(1)
        result = select(ch1, ch2, default: true)
        source = result[0]
        msg = result[1]
        is_default = source == :default
        is_none = msg == none
    "#);
    assert_eq!(executor.get_variable("is_default").unwrap(), Value::boolean(true));
    assert_eq!(executor.get_variable("is_none").unwrap(), Value::boolean(true));
}

#[test]
fn test_select_default_data_available() {
    let executor = exec(r#"
        ch = channel(1)
        ch.send("ready")
        result = select(ch, default: true)
        source = result[0]
        msg = result[1]
        is_ch = source == ch
    "#);
    assert_eq!(executor.get_variable("is_ch").unwrap(), Value::boolean(true));
    assert_eq!(executor.get_variable("msg").unwrap(), Value::string("ready".to_string()));
}

// ============================================================
// select() with timeout
// ============================================================

#[test]
fn test_select_timeout_expired() {
    let executor = exec(r#"
        ch = channel(1)
        result = select(ch, timeout: 10)
        source = result[0]
        is_timeout = source == :timeout
    "#);
    assert_eq!(executor.get_variable("is_timeout").unwrap(), Value::boolean(true));
}

#[test]
fn test_select_timeout_data_arrives_before() {
    let executor = exec(r#"
        ch = channel(1)
        ch.send("fast")
        result = select(ch, timeout: 5000)
        source = result[0]
        msg = result[1]
        is_ch = source == ch
    "#);
    assert_eq!(executor.get_variable("msg").unwrap(), Value::string("fast".to_string()));
    assert_eq!(executor.get_variable("is_ch").unwrap(), Value::boolean(true));
}

// ============================================================
// Error cases
// ============================================================

#[test]
fn test_select_all_closed_error() {
    let err = exec_err(r#"
        ch = channel(1)
        ch.close()
        result = select(ch)
    "#);
    assert!(err.contains("closed"), "Expected 'closed' error: {}", err);
}

#[test]
fn test_select_no_args_error() {
    let err = exec_err(r#"
        result = select()
    "#);
    assert!(err.contains("at least one channel"), "Expected arg error: {}", err);
}

#[test]
fn test_select_non_channel_error() {
    let err = exec_err(r#"
        result = select(42)
    "#);
    assert!(err.contains("channel"), "Expected type error: {}", err);
}

// ============================================================
// select() with spawn — real concurrency
// ============================================================

#[test]
fn test_select_with_spawned_sender() {
    let executor = exec(r#"
        ch1 = channel(1)
        ch2 = channel(1)
        spawn {
            ch1.send("from_spawn")
        }
        result = select(ch1, ch2, timeout: 5000)
        msg = result[1]
    "#);
    assert_eq!(executor.get_variable("msg").unwrap(), Value::string("from_spawn".to_string()));
}

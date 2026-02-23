//! Phase 19.1: Concurrency tests — spawn + channels

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
// Channel creation
// =========================================================================

#[test]
fn test_channel_creates_channel_value() {
    let executor = exec("ch = channel()");
    let val = executor.get_variable("ch").unwrap();
    assert_eq!(val.type_name(), "channel");
}

#[test]
fn test_channel_buffered() {
    let executor = exec("ch = channel(10)");
    let val = executor.get_variable("ch").unwrap();
    assert_eq!(val.type_name(), "channel");
}

#[test]
fn test_typeof_channel() {
    let executor = exec("ch = channel()\nt = typeof(ch)");
    let val = executor.get_variable("t").unwrap();
    assert_eq!(val, Value::string("channel".to_string()));
}

#[test]
fn test_channel_is_truthy() {
    let executor = exec("ch = channel()\nresult = if ch { true } else { false }");
    let val = executor.get_variable("result").unwrap();
    assert_eq!(val, Value::boolean(true));
}

// =========================================================================
// Channel send/receive (buffered, same thread)
// =========================================================================

#[test]
fn test_buffered_channel_send_receive() {
    let executor = exec(r#"
        ch = channel(1)
        ch.send(42)
        result = ch.receive()
    "#);
    let val = executor.get_variable("result").unwrap();
    assert_eq!(val, Value::number(42.0));
}

#[test]
fn test_buffered_channel_string() {
    let executor = exec(r#"
        ch = channel(1)
        ch.send("hello")
        result = ch.receive()
    "#);
    let val = executor.get_variable("result").unwrap();
    assert_eq!(val, Value::string("hello".to_string()));
}

#[test]
fn test_buffered_channel_list() {
    let executor = exec(r#"
        ch = channel(1)
        ch.send([1, 2, 3])
        result = ch.receive()
    "#);
    let val = executor.get_variable("result").unwrap();
    assert_eq!(val.to_string(), "[1, 2, 3]");
}

#[test]
fn test_buffered_channel_map() {
    let executor = exec(r#"
        ch = channel(1)
        ch.send({"key": "value"})
        result = ch.receive()
    "#);
    let val = executor.get_variable("result").unwrap();
    assert!(val.to_string().contains("key"));
}

#[test]
fn test_buffered_channel_multiple_values() {
    let executor = exec(r#"
        ch = channel(3)
        ch.send(1)
        ch.send(2)
        ch.send(3)
        a = ch.receive()
        b = ch.receive()
        c = ch.receive()
    "#);
    assert_eq!(executor.get_variable("a").unwrap(), Value::number(1.0));
    assert_eq!(executor.get_variable("b").unwrap(), Value::number(2.0));
    assert_eq!(executor.get_variable("c").unwrap(), Value::number(3.0));
}

// =========================================================================
// try_receive
// =========================================================================

#[test]
fn test_try_receive_empty_returns_none() {
    let executor = exec(r#"
        ch = channel()
        result = ch.try_receive()
    "#);
    let val = executor.get_variable("result").unwrap();
    assert_eq!(val, Value::none());
}

#[test]
fn test_try_receive_with_data() {
    let executor = exec(r#"
        ch = channel(1)
        ch.send(42)
        result = ch.try_receive()
    "#);
    let val = executor.get_variable("result").unwrap();
    assert_eq!(val, Value::number(42.0));
}

// =========================================================================
// Channel close
// =========================================================================

#[test]
fn test_close_drains_remaining() {
    let executor = exec(r#"
        ch = channel(2)
        ch.send(1)
        ch.send(2)
        ch.close()
        a = ch.receive()
        b = ch.receive()
    "#);
    assert_eq!(executor.get_variable("a").unwrap(), Value::number(1.0));
    assert_eq!(executor.get_variable("b").unwrap(), Value::number(2.0));
}

#[test]
fn test_receive_from_closed_empty_returns_none() {
    let executor = exec(r#"
        ch = channel(1)
        ch.close()
        result = ch.receive()
    "#);
    let val = executor.get_variable("result").unwrap();
    assert_eq!(val, Value::none());
}

#[test]
fn test_send_on_closed_channel_raises() {
    let err = exec_err(r#"
        ch = channel()
        ch.close()
        ch.send(42)
    "#);
    assert!(err.contains("Cannot send on closed channel"), "Got: {}", err);
}

// =========================================================================
// Spawn basics
// =========================================================================

#[test]
fn test_spawn_fire_and_forget() {
    // spawn should not error
    exec("spawn { x = 42 }");
}

#[test]
fn test_spawn_does_not_modify_parent_scope() {
    let executor = exec(r#"
        x = 1
        ch = channel()
        spawn {
            x = 999
            ch.send(x)
        }
        spawned_x = ch.receive()
    "#);
    // x should still be 1 in parent scope (share-nothing)
    assert_eq!(executor.get_variable("x").unwrap(), Value::number(1.0));
    assert_eq!(executor.get_variable("spawned_x").unwrap(), Value::number(999.0));
}

// =========================================================================
// Spawn + channel communication
// =========================================================================

#[test]
fn test_spawn_channel_send_receive() {
    let executor = exec(r#"
        ch = channel()
        spawn {
            ch.send(42)
        }
        result = ch.receive()
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::number(42.0));
}

#[test]
fn test_spawn_captures_parent_values() {
    let executor = exec(r#"
        ch = channel()
        name = "Alice"
        age = 30
        spawn {
            ch.send(name)
            ch.send(age)
        }
        r_name = ch.receive()
        r_age = ch.receive()
    "#);
    assert_eq!(executor.get_variable("r_name").unwrap(), Value::string("Alice".to_string()));
    assert_eq!(executor.get_variable("r_age").unwrap(), Value::number(30.0));
}

#[test]
fn test_spawn_calls_function() {
    let executor = exec(r#"
        ch = channel()
        fn double(n) { return n * 2 }
        spawn {
            ch.send(double(21))
        }
        result = ch.receive()
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::number(42.0));
}

#[test]
fn test_spawn_multiple_producers() {
    let executor = exec(r#"
        ch = channel()
        spawn { ch.send(1) }
        spawn { ch.send(2) }
        spawn { ch.send(3) }
        a = ch.receive()
        b = ch.receive()
        c = ch.receive()
        total = a + b + c
    "#);
    assert_eq!(executor.get_variable("total").unwrap(), Value::number(6.0));
}

#[test]
fn test_spawn_sends_list() {
    let executor = exec(r#"
        ch = channel()
        items = [10, 20, 30]
        spawn {
            ch.send(items)
        }
        result = ch.receive()
    "#);
    assert_eq!(executor.get_variable("result").unwrap().to_string(), "[10, 20, 30]");
}

#[test]
fn test_spawn_share_nothing_list() {
    let executor = exec(r#"
        ch = channel()
        items = [1, 2, 3]
        spawn {
            items = [4, 5, 6]
            ch.send(items)
        }
        spawned_items = ch.receive()
    "#);
    // Parent's items unchanged
    assert_eq!(executor.get_variable("items").unwrap().to_string(), "[1, 2, 3]");
    // Spawned task had its own copy
    assert_eq!(executor.get_variable("spawned_items").unwrap().to_string(), "[4, 5, 6]");
}

// =========================================================================
// Spawn exception handling
// =========================================================================

#[test]
fn test_spawn_exception_does_not_crash_parent() {
    // spawn with raise should not crash parent
    let executor = exec(r#"
        ch = channel()
        spawn {
            raise("boom")
        }
        spawn {
            ch.send("ok")
        }
        result = ch.receive()
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::string("ok".to_string()));
}

// =========================================================================
// Channel with named capacity argument
// =========================================================================

#[test]
fn test_channel_named_capacity() {
    let executor = exec(r#"
        ch = channel(capacity: 5)
        ch.send(1)
        ch.send(2)
        ch.send(3)
        ch.send(4)
        ch.send(5)
        a = ch.receive()
    "#);
    assert_eq!(executor.get_variable("a").unwrap(), Value::number(1.0));
}

//! Phase 19.2: Timer tests — timer.sleep, timer.after, timer.every

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
// timer.sleep()
// =========================================================================

#[test]
fn test_timer_sleep_basic() {
    // Should complete without error
    exec("timer.sleep(10)");
}

#[test]
fn test_timer_sleep_zero() {
    // sleep(0) should complete immediately
    exec("timer.sleep(0)");
}

#[test]
fn test_timer_sleep_negative_raises() {
    let err = exec_err("timer.sleep(-1)");
    assert!(err.contains("non-negative"), "Expected non-negative error, got: {}", err);
}

#[test]
fn test_timer_sleep_string_raises() {
    let err = exec_err(r#"timer.sleep("abc")"#);
    assert!(err.contains("number") || err.contains("type"), "Expected type error, got: {}", err);
}

#[test]
fn test_timer_sleep_no_args_raises() {
    let err = exec_err("timer.sleep()");
    assert!(err.contains("1 argument") || err.contains("requires"), "Expected arg error, got: {}", err);
}

#[test]
fn test_timer_sleep_actually_blocks() {
    // Verify sleep actually takes time
    let start = std::time::Instant::now();
    exec("timer.sleep(50)");
    let elapsed = start.elapsed().as_millis();
    assert!(elapsed >= 40, "Expected at least 40ms, got {}ms", elapsed);
}

// =========================================================================
// timer.after()
// =========================================================================

#[test]
fn test_timer_after_returns_channel() {
    let executor = exec("ch = timer.after(10)");
    let val = executor.get_variable("ch").unwrap();
    assert_eq!(val.type_name(), "channel");
}

#[test]
fn test_timer_after_receives_tick() {
    let executor = exec(r#"
        ch = timer.after(10)
        msg = ch.receive()
    "#);
    let msg = executor.get_variable("msg").unwrap();
    assert_eq!(msg, Value::symbol("tick".to_string()));
}

#[test]
fn test_timer_after_custom_value() {
    let executor = exec(r#"
        ch = timer.after(10, "wake up")
        msg = ch.receive()
    "#);
    let msg = executor.get_variable("msg").unwrap();
    assert_eq!(msg, Value::string("wake up".to_string()));
}

#[test]
fn test_timer_after_one_shot() {
    // After receiving, channel should be closed — second receive returns none
    let executor = exec(r#"
        ch = timer.after(10)
        msg1 = ch.receive()
        msg2 = ch.receive()
    "#);
    let msg1 = executor.get_variable("msg1").unwrap();
    let msg2 = executor.get_variable("msg2").unwrap();
    assert_eq!(msg1, Value::symbol("tick".to_string()));
    assert_eq!(msg2, Value::none());
}

#[test]
fn test_timer_after_no_args_raises() {
    let err = exec_err("timer.after()");
    assert!(err.contains("argument") || err.contains("requires"), "Expected arg error, got: {}", err);
}

#[test]
fn test_timer_after_negative_raises() {
    let err = exec_err("timer.after(-1)");
    assert!(err.contains("non-negative"), "Expected non-negative error, got: {}", err);
}

#[test]
fn test_timer_after_cancel_by_close() {
    // Closing before timer fires should not cause errors
    exec(r#"
        ch = timer.after(1000)
        ch.close()
    "#);
}

// =========================================================================
// timer.every()
// =========================================================================

#[test]
fn test_timer_every_returns_channel() {
    // Just verify it doesn't error — channel was closed immediately
    exec(r#"
        ch = timer.every(50)
        ch.close()
    "#);
}

#[test]
fn test_timer_every_receives_tick() {
    let executor = exec(r#"
        ch = timer.every(20)
        msg = ch.receive()
        ch.close()
    "#);
    let msg = executor.get_variable("msg").unwrap();
    assert_eq!(msg, Value::symbol("tick".to_string()));
}

#[test]
fn test_timer_every_multiple_ticks() {
    let executor = exec(r#"
        ch = timer.every(20)
        t1 = ch.receive()
        t2 = ch.receive()
        t3 = ch.receive()
        ch.close()
    "#);
    assert_eq!(executor.get_variable("t1").unwrap(), Value::symbol("tick".to_string()));
    assert_eq!(executor.get_variable("t2").unwrap(), Value::symbol("tick".to_string()));
    assert_eq!(executor.get_variable("t3").unwrap(), Value::symbol("tick".to_string()));
}

#[test]
fn test_timer_every_cancel_by_close() {
    let executor = exec(r#"
        ch = timer.every(20)
        ch.receive()
        ch.close()
        timer.sleep(60)
        result = ch.try_receive()
    "#);
    // After close + waiting, no more ticks should arrive
    let result = executor.get_variable("result").unwrap();
    assert_eq!(result, Value::none());
}

#[test]
fn test_timer_every_no_args_raises() {
    let err = exec_err("timer.every()");
    assert!(err.contains("argument") || err.contains("requires"), "Expected arg error, got: {}", err);
}

#[test]
fn test_timer_every_negative_raises() {
    let err = exec_err("timer.every(-1)");
    assert!(err.contains("non-negative"), "Expected non-negative error, got: {}", err);
}

// =========================================================================
// for..in channel iteration
// =========================================================================

#[test]
fn test_for_in_channel_iterates() {
    let executor = exec(r#"
        ch = channel(10)
        ch.send(1)
        ch.send(2)
        ch.send(3)
        ch.close()
        total = 0
        for msg in ch {
            total = total + msg
        }
    "#);
    let total = executor.get_variable("total").unwrap();
    assert_eq!(total, Value::number(6.0));
}

#[test]
fn test_for_in_channel_empty_close() {
    // Closed empty channel produces no iterations
    let executor = exec(r#"
        ch = channel()
        ch.close()
        count = 0
        for msg in ch {
            count = count + 1
        }
    "#);
    let count = executor.get_variable("count").unwrap();
    assert_eq!(count, Value::number(0.0));
}

#[test]
fn test_for_in_channel_with_break() {
    let executor = exec(r#"
        ch = channel(10)
        ch.send(1)
        ch.send(2)
        ch.send(3)
        ch.close()
        last = 0
        for msg in ch {
            last = msg
            if msg == 2 {
                break
            }
        }
    "#);
    let last = executor.get_variable("last").unwrap();
    assert_eq!(last, Value::number(2.0));
}

#[test]
fn test_for_in_channel_with_spawn() {
    // Producer sends via spawn, consumer iterates
    let executor = exec(r#"
        ch = channel()
        spawn {
            ch.send(10)
            ch.send(20)
            ch.send(30)
            ch.close()
        }
        total = 0
        for msg in ch {
            total = total + msg
        }
    "#);
    let total = executor.get_variable("total").unwrap();
    assert_eq!(total, Value::number(60.0));
}

#[test]
fn test_for_in_channel_with_timer_every() {
    // Collect 3 ticks from timer.every, then break
    let executor = exec(r#"
        ch = timer.every(10)
        count = 0
        for msg in ch {
            count = count + 1
            if count == 3 {
                break
            }
        }
        ch.close()
    "#);
    let count = executor.get_variable("count").unwrap();
    assert_eq!(count, Value::number(3.0));
}

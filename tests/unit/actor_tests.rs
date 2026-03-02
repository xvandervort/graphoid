//! Unit tests for Phase 19.3: Actors as Graph Nodes

use graphoid::execution_graph::graph_executor::GraphExecutor;
use graphoid::values::Value;
use graphoid::values::actor::ActorRef;
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
// Step 1: ActorRef value type
// ============================================================

#[test]
fn test_actor_ref_creation() {
    let ch = Channel::new(Some(10));
    let actor = ActorRef::new(ch, Some("Counter".to_string()));
    assert!(actor.id > 0);
    assert_eq!(actor.type_name, Some("Counter".to_string()));
}

#[test]
fn test_actor_ref_unique_ids() {
    let ch1 = Channel::new(Some(10));
    let ch2 = Channel::new(Some(10));
    let a1 = ActorRef::new(ch1, None);
    let a2 = ActorRef::new(ch2, None);
    assert_ne!(a1.id, a2.id);
}

#[test]
fn test_actor_value_type_name() {
    let ch = Channel::new(Some(10));
    let actor = ActorRef::new(ch, Some("Counter".to_string()));
    let val = Value::actor(actor);
    assert_eq!(val.type_name(), "actor");
}

#[test]
fn test_actor_value_to_string() {
    let ch = Channel::new(Some(10));
    let actor = ActorRef::new(ch, Some("Counter".to_string()));
    let val = Value::actor(actor.clone());
    let s = val.to_string_value();
    assert!(s.starts_with("<actor:Counter#"));
    assert!(s.ends_with(">"));
}

#[test]
fn test_actor_value_to_string_no_type() {
    let ch = Channel::new(Some(10));
    let actor = ActorRef::new(ch, None);
    let val = Value::actor(actor.clone());
    let s = val.to_string_value();
    assert!(s.starts_with("<actor#"));
    assert!(s.ends_with(">"));
}

#[test]
fn test_actor_deep_clone_for_send() {
    let ch = Channel::new(Some(10));
    let actor = ActorRef::new(ch.clone(), Some("Worker".to_string()));
    let val = Value::actor(actor.clone());
    let cloned = val.deep_clone_for_send();
    // Cloned value should still reference the same mailbox channel
    if let graphoid::values::ValueKind::Actor(ref cloned_actor) = cloned.0.kind {
        assert_eq!(cloned_actor.id, actor.id);
        // Can send through original, receive through clone's mailbox (same channel)
        ch.send(graphoid::values::channel::SendableValue(Value::number(42.0))).unwrap();
        let received = cloned_actor.mailbox.receive().unwrap();
        assert_eq!(received.0, Value::number(42.0));
    } else {
        panic!("Expected Actor value kind");
    }
}

#[test]
fn test_actor_equality_by_id() {
    let ch = Channel::new(Some(10));
    let actor = ActorRef::new(ch, Some("Counter".to_string()));
    let val1 = Value::actor(actor.clone());
    let val2 = Value::actor(actor.clone());
    assert_eq!(val1, val2);
}

#[test]
fn test_actor_inequality() {
    let ch1 = Channel::new(Some(10));
    let ch2 = Channel::new(Some(10));
    let a1 = ActorRef::new(ch1, Some("A".to_string()));
    let a2 = ActorRef::new(ch2, Some("B".to_string()));
    let val1 = Value::actor(a1);
    let val2 = Value::actor(a2);
    assert_ne!(val1, val2);
}

#[test]
fn test_actor_is_truthy() {
    let ch = Channel::new(Some(10));
    let actor = ActorRef::new(ch, None);
    let val = Value::actor(actor);
    assert!(val.is_truthy());
}

// ============================================================
// Step 3: Parser tests for spawn Counter{}
// ============================================================

#[test]
fn test_parse_spawn_block_still_works() {
    // Existing spawn { } syntax must still work
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
fn test_parse_spawn_actor_syntax() {
    // spawn Counter{} should parse without error
    // (execution will fail until Step 4, but parsing should work)
    use graphoid::parser::Parser;
    use graphoid::lexer::Lexer;
    let source = r#"
        graph Worker {
            status: "idle"
            fn on_message(msg) {
                return msg
            }
        }
        w = spawn Worker{}
    "#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    // Should have 2 statements: graph decl + assignment with spawn
    assert!(program.statements.len() >= 2);
}

#[test]
fn test_parse_spawn_actor_with_overrides() {
    use graphoid::parser::Parser;
    use graphoid::lexer::Lexer;
    let source = r#"
        graph Worker {
            status: "idle"
            fn on_message(msg) {
                return msg
            }
        }
        w = spawn Worker{ status: "active" }
    "#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    assert!(program.statements.len() >= 2);
}

// ============================================================
// Steps 4-5: Actor spawn execution, message loop, method dispatch
// ============================================================

#[test]
fn test_spawn_actor_returns_actor_type() {
    let executor = exec(r#"
        graph Echo {
            fn on_message(msg) {
                return msg
            }
        }
        a = spawn Echo{}
        t = typeof(a)
    "#);
    assert_eq!(executor.get_variable("t").unwrap(), Value::string("actor".to_string()));
}

#[test]
fn test_spawn_actor_requires_on_message() {
    let err = exec_err(r#"
        graph NoHandler {
            value: 0
        }
        a = spawn NoHandler{}
    "#);
    assert!(err.contains("on_message"), "Error should mention on_message: {}", err);
}

#[test]
fn test_actor_request_returns_value() {
    let executor = exec(r#"
        graph Echo {
            fn on_message(msg) {
                return msg
            }
        }
        a = spawn Echo{}
        result = a.request("hello")
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::string("hello".to_string()));
}

#[test]
fn test_actor_send_fire_and_forget() {
    let executor = exec(r#"
        graph Echo {
            fn on_message(msg) {
                return msg
            }
        }
        a = spawn Echo{}
        result = a.send("hello")
    "#);
    // send returns none (fire-and-forget)
    assert_eq!(executor.get_variable("result").unwrap(), Value::none());
}

#[test]
fn test_actor_state_persists() {
    let executor = exec(r#"
        graph Counter {
            count: 0
            fn on_message(msg) {
                if msg == "increment" { count = count + 1 }
                if msg == "get" { return count }
            }
        }
        c = spawn Counter{}
        c.send("increment")
        c.send("increment")
        c.send("increment")
        result = c.request("get")
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::number(3.0));
}

#[test]
fn test_actor_state_isolated() {
    let executor = exec(r#"
        graph Counter {
            count: 0
            fn on_message(msg) {
                if msg == "increment" { count = count + 1 }
                if msg == "get" { return count }
            }
        }
        a = spawn Counter{}
        b = spawn Counter{}
        a.send("increment")
        a.send("increment")
        b.send("increment")
        result_a = a.request("get")
        result_b = b.request("get")
    "#);
    assert_eq!(executor.get_variable("result_a").unwrap(), Value::number(2.0));
    assert_eq!(executor.get_variable("result_b").unwrap(), Value::number(1.0));
}

#[test]
fn test_actor_initial_override() {
    let executor = exec(r#"
        graph Counter {
            count: 0
            fn on_message(msg) {
                if msg == "get" { return count }
            }
        }
        c = spawn Counter{ count: 100 }
        result = c.request("get")
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::number(100.0));
}

#[test]
fn test_actor_close() {
    let executor = exec(r#"
        graph Echo {
            fn on_message(msg) {
                return msg
            }
        }
        a = spawn Echo{}
        a.close()
        result = a.is_closed()
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::boolean(true));
}

// ============================================================
// Step 6: Graph-native messaging
// ============================================================

#[test]
fn test_graph_send_to_named_node() {
    let executor = exec(r#"
        graph Echo {
            fn on_message(msg) {
                return msg
            }
        }
        g = graph{}
        g.add_node("alice", spawn Echo{})
        g.send("hello", to: "alice")
        result = "sent"
    "#);
    // send is fire-and-forget, just verify no error
    assert_eq!(executor.get_variable("result").unwrap(), Value::string("sent".to_string()));
}

#[test]
fn test_graph_broadcast() {
    let executor = exec(r#"
        graph Counter {
            count: 0
            fn on_message(msg) {
                if msg == "increment" { count = count + 1 }
                if msg == "get" { return count }
            }
        }
        g = graph{}
        g.add_node("a", spawn Counter{})
        g.add_node("b", spawn Counter{})
        g.broadcast("increment")
        result_a = g.request("get", to: "a")
        result_b = g.request("get", to: "b")
    "#);
    assert_eq!(executor.get_variable("result_a").unwrap(), Value::number(1.0));
    assert_eq!(executor.get_variable("result_b").unwrap(), Value::number(1.0));
}

#[test]
fn test_graph_broadcast_skips_non_actors() {
    let executor = exec(r#"
        graph Echo {
            fn on_message(msg) {
                return msg
            }
        }
        g = graph{}
        g.add_node("alice", spawn Echo{})
        g.add_node("count", 42)
        g.broadcast("ping")
        result = "no_error"
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::string("no_error".to_string()));
}

#[test]
fn test_graph_request_to_node() {
    let executor = exec(r#"
        graph Echo {
            fn on_message(msg) {
                return msg
            }
        }
        g = graph{}
        g.add_node("alice", spawn Echo{})
        result = g.request("hello", to: "alice")
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::string("hello".to_string()));
}

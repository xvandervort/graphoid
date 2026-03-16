//! Unit tests for Phase 19.5: Supervision — actor restart strategies

use graphoid::execution_graph::graph_executor::GraphExecutor;
use graphoid::values::actor::{ActorRef, RestartMode, SupervisorConfig, SUPERVISION_REGISTRY, ACTOR_REGISTRY};
use graphoid::values::Channel;

fn exec(source: &str) -> GraphExecutor {
    let mut executor = GraphExecutor::new();
    executor.execute_source(source).unwrap();
    executor
}

fn exec_get(source: &str, var: &str) -> graphoid::values::Value {
    let executor = exec(source);
    executor.env().get(var).unwrap()
}

fn exec_err(source: &str) -> String {
    let mut executor = GraphExecutor::new();
    match executor.execute_source(source) {
        Ok(_) => panic!("Expected error"),
        Err(e) => format!("{}", e),
    }
}

// ============================================================
// Supervisor template
// ============================================================

#[test]
fn test_supervisor_template_exists() {
    let executor = exec("");
    let val = executor.env().get("supervisor").unwrap();
    assert_eq!(val.type_name(), "graph");
}

#[test]
fn test_supervisor_template_type_name() {
    let val = exec_get("result = supervisor.type()", "result");
    assert_eq!(val.to_string_value(), "graph");
}

#[test]
fn test_supervisor_inherits_creates_graph() {
    let val = exec_get(r#"
        graph MySup from supervisor {}
        result = MySup{}.type()
    "#, "result");
    assert_eq!(val.to_string_value(), "graph");
}

#[test]
fn test_supervisor_default_strategy() {
    let executor = exec("");
    let sup_val = executor.env().get("supervisor").unwrap();
    if let graphoid::values::ValueKind::Graph(ref g) = sup_val.kind {
        let graph = g.borrow();
        let prop = graph.get_node(&graphoid::values::graph::Graph::property_node_id("strategy"));
        assert!(prop.is_some());
        if let Some(v) = prop {
            assert_eq!(v.to_string_value(), ":one_for_one");
        }
    } else {
        panic!("supervisor is not a graph");
    }
}

#[test]
fn test_supervisor_default_max_restarts() {
    let executor = exec("");
    let sup_val = executor.env().get("supervisor").unwrap();
    if let graphoid::values::ValueKind::Graph(ref g) = sup_val.kind {
        let graph = g.borrow();
        let prop = graph.get_node(&graphoid::values::graph::Graph::property_node_id("max_restarts"));
        assert!(prop.is_some());
        if let Some(v) = prop {
            assert_eq!(v.to_string_value(), "3");
        }
    } else {
        panic!("supervisor is not a graph");
    }
}

// ============================================================
// RestartMode
// ============================================================

#[test]
fn test_restart_mode_from_symbol() {
    assert_eq!(RestartMode::from_symbol("permanent"), Some(RestartMode::Permanent));
    assert_eq!(RestartMode::from_symbol("transient"), Some(RestartMode::Transient));
    assert_eq!(RestartMode::from_symbol("temporary"), Some(RestartMode::Temporary));
    assert_eq!(RestartMode::from_symbol("unknown"), None);
}

// ============================================================
// SupervisorConfig
// ============================================================

#[test]
fn test_supervisor_config_creation() {
    let config = SupervisorConfig::new("one_for_one".to_string(), 5);
    assert_eq!(config.strategy, "one_for_one");
    assert_eq!(config.max_restarts, 5);
    let children = config.children.lock().unwrap();
    assert!(children.is_empty());
}

// ============================================================
// ActorRef redirect
// ============================================================

#[test]
fn test_actor_ref_effective_mailbox_default() {
    let ch = Channel::new(Some(10));
    let actor = ActorRef::new(ch.clone(), None);
    let effective = actor.effective_mailbox();
    assert_eq!(effective, ch);
}

#[test]
fn test_actor_ref_redirect() {
    let ch1 = Channel::new(Some(10));
    let ch2 = Channel::new(Some(10));
    let actor = ActorRef::new(ch1.clone(), None);

    assert_eq!(actor.effective_mailbox(), ch1);

    actor.redirect_to(ch2.clone());
    assert_eq!(actor.effective_mailbox(), ch2);
}

#[test]
fn test_actor_ref_redirect_shared_across_clones() {
    let ch1 = Channel::new(Some(10));
    let ch2 = Channel::new(Some(10));
    let actor1 = ActorRef::new(ch1.clone(), None);
    let actor2 = actor1.clone();

    actor1.redirect_to(ch2.clone());

    // Clone should also see the redirect (Arc-shared)
    assert_eq!(actor2.effective_mailbox(), ch2);
}

// ============================================================
// SpawnTemplate
// ============================================================

#[test]
fn test_spawn_template_stored_at_spawn() {
    let executor = exec(r#"
        graph Worker {
            fn on_message(msg) { return msg }
        }
        w = spawn Worker{}
    "#);
    let w = executor.env().get("w").unwrap();
    if let graphoid::values::ValueKind::Actor(ref a) = w.kind {
        let tmpl = a.spawn_template.lock().unwrap();
        assert!(tmpl.is_some(), "Spawn template should be stored on actor");
    } else {
        panic!("w is not an actor");
    }
}

// ============================================================
// Supervisor spawn detection
// ============================================================

#[test]
fn test_supervisor_actor_has_config() {
    let executor = exec(r#"
        graph MySup from supervisor {
            fn on_message(msg) { return msg }
        }
        s = spawn MySup{}
    "#);
    let s = executor.env().get("s").unwrap();
    if let graphoid::values::ValueKind::Actor(ref a) = s.kind {
        assert!(a.supervisor_config.is_some(), "Supervisor actor should have config");
        let config = a.supervisor_config.as_ref().unwrap();
        assert_eq!(config.strategy, "one_for_one");
        assert_eq!(config.max_restarts, 3);
    } else {
        panic!("s is not an actor");
    }
}

#[test]
fn test_non_supervisor_has_no_config() {
    let executor = exec(r#"
        graph Worker {
            fn on_message(msg) { return msg }
        }
        w = spawn Worker{}
    "#);
    let w = executor.env().get("w").unwrap();
    if let graphoid::values::ValueKind::Actor(ref a) = w.kind {
        assert!(a.supervisor_config.is_none(), "Regular actor should not have supervisor config");
    } else {
        panic!("w is not an actor");
    }
}

// ============================================================
// .supervise() method
// ============================================================

#[test]
fn test_supervise_registers_child() {
    let executor = exec(r#"
        graph MySup from supervisor {
            fn on_message(msg) { return msg }
        }
        graph Worker {
            fn on_message(msg) { return msg }
        }
        sup = spawn MySup{}
        worker = spawn Worker{}
        sup.supervise(worker)
    "#);

    let worker = executor.env().get("worker").unwrap();
    if let graphoid::values::ValueKind::Actor(ref a) = worker.kind {
        let registry = SUPERVISION_REGISTRY.lock().unwrap();
        assert!(registry.contains_key(&a.id), "Worker should be in supervision registry");
        let entry = registry.get(&a.id).unwrap();
        assert_eq!(entry.restart_mode, RestartMode::Permanent);
    } else {
        panic!("worker is not an actor");
    }
}

#[test]
fn test_supervise_with_restart_mode() {
    let executor = exec(r#"
        graph MySup from supervisor {
            fn on_message(msg) { return msg }
        }
        graph Worker {
            fn on_message(msg) { return msg }
        }
        sup = spawn MySup{}
        w = spawn Worker{}
        sup.supervise(w, restart: :transient)
    "#);

    let w = executor.env().get("w").unwrap();
    if let graphoid::values::ValueKind::Actor(ref a) = w.kind {
        let registry = SUPERVISION_REGISTRY.lock().unwrap();
        let entry = registry.get(&a.id).unwrap();
        assert_eq!(entry.restart_mode, RestartMode::Transient);
    } else {
        panic!("w is not an actor");
    }
}

#[test]
fn test_supervise_on_non_supervisor_errors() {
    let err = exec_err(r#"
        graph Worker {
            fn on_message(msg) { return msg }
        }
        w1 = spawn Worker{}
        w2 = spawn Worker{}
        w1.supervise(w2)
    "#);
    assert!(err.contains("Only supervisors"), "Error: {}", err);
}

#[test]
fn test_supervise_non_actor_arg_errors() {
    let err = exec_err(r#"
        graph MySup from supervisor {
            fn on_message(msg) { return msg }
        }
        sup = spawn MySup{}
        sup.supervise("not_an_actor")
    "#);
    assert!(err.contains("must be an actor"), "Error: {}", err);
}

// ============================================================
// Actor registered in ACTOR_REGISTRY
// ============================================================

#[test]
fn test_actor_registered_in_global_registry() {
    let executor = exec(r#"
        graph Worker {
            fn on_message(msg) { return msg }
        }
        w = spawn Worker{}
    "#);

    let w = executor.env().get("w").unwrap();
    if let graphoid::values::ValueKind::Actor(ref a) = w.kind {
        let registry = ACTOR_REGISTRY.lock().unwrap();
        assert!(registry.contains_key(&a.id), "Actor should be in global registry");
    } else {
        panic!("w is not an actor");
    }
}

// ============================================================
// Actor .id method
// ============================================================

#[test]
fn test_actor_id_method() {
    let val = exec_get(r#"
        graph Worker {
            fn on_message(msg) { return msg }
        }
        w = spawn Worker{}
        result = w.id()
    "#, "result");
    match &val.kind {
        graphoid::values::ValueKind::Number(n) => assert!(*n > 0.0),
        _ => panic!("Expected number, got {}", val.type_name()),
    }
}

// ============================================================
// Supervised actor restart (integration)
// ============================================================

#[test]
fn test_supervised_actor_restarts_on_crash() {
    let val = exec_get(r#"
        graph MySup from supervisor {
            fn on_message(msg) { return msg }
        }
        graph CrashWorker {
            fn on_message(msg) {
                if msg == "crash" {
                    raise "Intentional crash"
                }
                return "ok:" + msg
            }
        }
        sup = spawn MySup{}
        worker = spawn CrashWorker{}
        sup.supervise(worker)

        # Verify normal operation
        result1 = worker.request("hello")

        # Trigger crash
        worker.send("crash")

        # Wait for restart
        timer.sleep(200)

        # After restart, actor should work again
        result2 = worker.request("world")
        result = result1 + "|" + result2
    "#, "result");
    assert_eq!(val.to_string_value(), "ok:hello|ok:world");
}

#[test]
fn test_temporary_actor_does_not_restart() {
    let val = exec_get(r#"
        graph MySup from supervisor {
            fn on_message(msg) { return msg }
        }
        graph CrashWorker {
            fn on_message(msg) {
                if msg == "crash" {
                    raise "Intentional crash"
                }
                return "ok:" + msg
            }
        }
        sup = spawn MySup{}
        worker = spawn CrashWorker{}
        sup.supervise(worker, restart: :temporary)

        # Normal operation
        result = worker.request("hello")

        # Trigger crash — :temporary means no restart
        worker.send("crash")
        timer.sleep(100)

        # result should still be from before crash
        result = result
    "#, "result");
    assert_eq!(val.to_string_value(), "ok:hello");
}

#[test]
fn test_custom_supervisor_strategy() {
    let val = exec_get(r#"
        graph MySup from supervisor {
            strategy: :one_for_all
            max_restarts: 5
            fn on_message(msg) { return msg }
        }
        sup = spawn MySup{}
        result = sup.type()
    "#, "result");
    assert_eq!(val.to_string_value(), "actor");
}

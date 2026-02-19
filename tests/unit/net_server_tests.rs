//! Tests for net module server capabilities (bind, accept, close_listener, listener_port, set_timeout)

use graphoid::stdlib::net::NetModule;
use graphoid::stdlib::NativeModule;
use graphoid::values::{Value, ValueKind};

fn get_net_functions() -> std::collections::HashMap<String, graphoid::stdlib::NativeFunction> {
    NetModule.functions()
}

#[test]
fn test_net_module_has_bind_function() {
    let functions = get_net_functions();
    assert!(functions.contains_key("bind"), "net module should have bind function");
}

#[test]
fn test_net_module_has_accept_function() {
    let functions = get_net_functions();
    assert!(functions.contains_key("accept"), "net module should have accept function");
}

#[test]
fn test_net_module_has_close_listener_function() {
    let functions = get_net_functions();
    assert!(functions.contains_key("close_listener"), "net module should have close_listener function");
}

#[test]
fn test_net_module_has_listener_port_function() {
    let functions = get_net_functions();
    assert!(functions.contains_key("listener_port"), "net module should have listener_port function");
}

#[test]
fn test_net_module_has_set_timeout_function() {
    let functions = get_net_functions();
    assert!(functions.contains_key("set_timeout"), "net module should have set_timeout function");
}

#[test]
fn test_bind_returns_listener_id() {
    let functions = get_net_functions();
    let bind = functions.get("bind").unwrap();

    // Bind to port 0 (OS assigns free port)
    let result = bind(&[
        Value::string("127.0.0.1".to_string()),
        Value::number(0.0),
    ]).expect("bind should succeed");

    match &result.kind {
        ValueKind::Number(n) => assert!(*n > 0.0, "listener_id should be positive"),
        _ => panic!("bind should return a number"),
    }

    // Clean up
    let close_listener = functions.get("close_listener").unwrap();
    close_listener(&[result]).unwrap();
}

#[test]
fn test_bind_requires_two_arguments() {
    let functions = get_net_functions();
    let bind = functions.get("bind").unwrap();

    let result = bind(&[Value::string("127.0.0.1".to_string())]);
    assert!(result.is_err(), "bind should require exactly 2 arguments");
}

#[test]
fn test_bind_requires_string_host() {
    let functions = get_net_functions();
    let bind = functions.get("bind").unwrap();

    let result = bind(&[Value::number(127.0), Value::number(8080.0)]);
    assert!(result.is_err(), "bind should require string host");
}

#[test]
fn test_accept_requires_valid_listener_id() {
    let functions = get_net_functions();
    let accept = functions.get("accept").unwrap();

    let result = accept(&[Value::number(99999.0)]);
    assert!(result.is_err(), "accept should fail with invalid listener_id");
}

#[test]
fn test_listener_port_returns_assigned_port() {
    let functions = get_net_functions();
    let bind = functions.get("bind").unwrap();
    let listener_port = functions.get("listener_port").unwrap();
    let close_listener = functions.get("close_listener").unwrap();

    let listener = bind(&[
        Value::string("127.0.0.1".to_string()),
        Value::number(0.0),
    ]).unwrap();

    let port_result = listener_port(&[listener.clone()]).unwrap();
    match &port_result.kind {
        ValueKind::Number(n) => assert!(*n > 0.0, "assigned port should be positive"),
        _ => panic!("listener_port should return a number"),
    }

    close_listener(&[listener]).unwrap();
}

#[test]
fn test_close_listener_works() {
    let functions = get_net_functions();
    let bind = functions.get("bind").unwrap();
    let close_listener = functions.get("close_listener").unwrap();

    let listener = bind(&[
        Value::string("127.0.0.1".to_string()),
        Value::number(0.0),
    ]).unwrap();

    let result = close_listener(&[listener]);
    assert!(result.is_ok(), "close_listener should succeed");
}

#[test]
fn test_close_listener_invalid_id_fails() {
    let functions = get_net_functions();
    let close_listener = functions.get("close_listener").unwrap();

    let result = close_listener(&[Value::number(99999.0)]);
    assert!(result.is_err(), "close_listener should fail with invalid id");
}

#[test]
fn test_bind_accept_roundtrip() {
    let functions = get_net_functions();
    let bind = functions.get("bind").unwrap();
    let accept = functions.get("accept").unwrap();
    let listener_port = functions.get("listener_port").unwrap();
    let close_fn = functions.get("close").unwrap();
    let close_listener = functions.get("close_listener").unwrap();

    // Bind to port 0
    let listener = bind(&[
        Value::string("127.0.0.1".to_string()),
        Value::number(0.0),
    ]).unwrap();

    let port = match &listener_port(&[listener.clone()]).unwrap().kind {
        ValueKind::Number(n) => *n as u16,
        _ => panic!("Expected number"),
    };

    // Connect from a thread
    let handle = std::thread::spawn(move || {
        std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
    });

    // Accept the connection
    let socket = accept(&[listener.clone()]).unwrap();
    match &socket.kind {
        ValueKind::Number(n) => assert!(*n > 0.0, "accepted socket_id should be positive"),
        _ => panic!("accept should return a number"),
    }

    handle.join().unwrap();

    // Clean up
    close_fn(&[socket]).unwrap();
    close_listener(&[listener]).unwrap();
}

#[test]
fn test_bind_accept_send_recv_roundtrip() {
    let functions = get_net_functions();
    let bind = functions.get("bind").unwrap();
    let accept = functions.get("accept").unwrap();
    let listener_port = functions.get("listener_port").unwrap();
    let send = functions.get("send").unwrap();
    let recv = functions.get("recv").unwrap();
    let close_fn = functions.get("close").unwrap();
    let close_listener = functions.get("close_listener").unwrap();

    // Bind
    let listener = bind(&[
        Value::string("127.0.0.1".to_string()),
        Value::number(0.0),
    ]).unwrap();

    let port = match &listener_port(&[listener.clone()]).unwrap().kind {
        ValueKind::Number(n) => *n as u16,
        _ => panic!("Expected number"),
    };

    // Client thread sends data
    let handle = std::thread::spawn(move || {
        use std::io::Write;
        let mut stream = std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
        stream.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();
        stream.flush().unwrap();
    });

    // Accept and read
    let socket = accept(&[listener.clone()]).unwrap();
    let data = recv(&[socket.clone(), Value::number(4096.0)]).unwrap();

    match &data.kind {
        ValueKind::String(s) => {
            assert!(s.contains("GET / HTTP/1.1"), "should receive HTTP request");
        }
        _ => panic!("recv should return a string"),
    }

    // Send response
    send(&[socket.clone(), Value::string("HTTP/1.1 200 OK\r\n\r\nHello".to_string())]).unwrap();

    handle.join().unwrap();

    // Clean up
    close_fn(&[socket]).unwrap();
    close_listener(&[listener]).unwrap();
}

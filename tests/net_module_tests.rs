use graphoid::stdlib::{NetModule, NativeModule};
use graphoid::values::Value;

#[test]
fn test_net_module_name() {
    let module = NetModule;
    assert_eq!(module.name(), "net");
    assert_eq!(module.alias(), None);
}

#[test]
fn test_net_module_has_functions() {
    let module = NetModule;
    let functions = module.functions();

    assert!(functions.contains_key("connect"));
    assert!(functions.contains_key("send"));
    assert!(functions.contains_key("recv"));
    assert!(functions.contains_key("close"));
}

#[test]
fn test_net_connect_send_recv_close() {
    let module = NetModule;
    let functions = module.functions();

    // Connect to example.com
    let connect_fn = functions.get("connect").unwrap();
    let result = connect_fn(&[
        Value::string("example.com".to_string()),
        Value::number(80.0),
    ]);

    // Note: This test requires network access and may fail in isolated environments
    if result.is_err() {
        eprintln!("Skipping network test (no network access)");
        return;
    }

    let socket_id = result.unwrap();

    // Verify we got a numeric socket ID
    match &socket_id.kind {
        graphoid::values::ValueKind::Number(n) => {
            assert!(*n > 0.0);
        }
        _ => panic!("Expected number for socket ID"),
    }

    // Send HTTP request
    let send_fn = functions.get("send").unwrap();
    let request = "GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";
    let send_result = send_fn(&[
        socket_id.clone(),
        Value::string(request.to_string()),
    ]);
    assert!(send_result.is_ok());

    let bytes_sent = send_result.unwrap();
    match &bytes_sent.kind {
        graphoid::values::ValueKind::Number(n) => {
            assert!(*n > 0.0);
        }
        _ => panic!("Expected number for bytes sent"),
    }

    // Receive response
    let recv_fn = functions.get("recv").unwrap();
    let recv_result = recv_fn(&[
        socket_id.clone(),
        Value::number(4096.0),
    ]);
    assert!(recv_result.is_ok());

    let response = recv_result.unwrap();
    match &response.kind {
        graphoid::values::ValueKind::String(s) => {
            assert!(s.contains("HTTP"));
            assert!(!s.is_empty());
        }
        _ => panic!("Expected string for response"),
    }

    // Close socket
    let close_fn = functions.get("close").unwrap();
    let close_result = close_fn(&[socket_id]);
    assert!(close_result.is_ok());
}

#[test]
fn test_net_connect_invalid_host() {
    let module = NetModule;
    let functions = module.functions();

    let connect_fn = functions.get("connect").unwrap();
    let result = connect_fn(&[
        Value::string("invalid.invalid.invalid".to_string()),
        Value::number(80.0),
    ]);

    assert!(result.is_err());
}

#[test]
fn test_net_invalid_socket_handle() {
    let module = NetModule;
    let functions = module.functions();

    // Try to send with invalid handle
    let send_fn = functions.get("send").unwrap();
    let result = send_fn(&[
        Value::number(99999.0),
        Value::string("test".to_string()),
    ]);

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid socket handle"));
}

#[test]
fn test_net_close_invalid_handle() {
    let module = NetModule;
    let functions = module.functions();

    let close_fn = functions.get("close").unwrap();
    let result = close_fn(&[Value::number(99999.0)]);

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid socket handle"));
}

#[test]
fn test_net_connect_requires_two_args() {
    let module = NetModule;
    let functions = module.functions();

    let connect_fn = functions.get("connect").unwrap();

    // Too few arguments
    let result = connect_fn(&[Value::string("example.com".to_string())]);
    assert!(result.is_err());

    // Too many arguments
    let result = connect_fn(&[
        Value::string("example.com".to_string()),
        Value::number(80.0),
        Value::string("extra".to_string()),
    ]);
    assert!(result.is_err());
}

#[test]
fn test_net_send_requires_two_args() {
    let module = NetModule;
    let functions = module.functions();

    let send_fn = functions.get("send").unwrap();

    // Too few arguments
    let result = send_fn(&[Value::number(1.0)]);
    assert!(result.is_err());
}

#[test]
fn test_net_recv_requires_two_args() {
    let module = NetModule;
    let functions = module.functions();

    let recv_fn = functions.get("recv").unwrap();

    // Too few arguments
    let result = recv_fn(&[Value::number(1.0)]);
    assert!(result.is_err());
}

#![allow(dead_code)]
use graphoid::execution_graph::graph_executor::GraphExecutor;
use graphoid::values::ValueKind;

pub fn eval_var(source: &str, var_name: &str) -> graphoid::values::Value {
    let mut executor = GraphExecutor::new();
    executor.execute_source(source).unwrap();
    executor.get_variable(var_name).expect(&format!("Variable '{}' not found", var_name))
}

pub fn eval_source(source: &str) -> graphoid::values::Value {
    let mut executor = GraphExecutor::new();
    executor.execute_source_value(source).unwrap()
}

pub fn as_number(val: &graphoid::values::Value) -> f64 {
    match &val.kind { ValueKind::Number(n) => *n, _ => panic!("Expected number, got {:?}", val.kind) }
}

pub fn as_string(val: &graphoid::values::Value) -> String {
    match &val.kind { ValueKind::String(s) => s.clone(), _ => panic!("Expected string, got {:?}", val.kind) }
}

pub fn as_bool(val: &graphoid::values::Value) -> bool {
    match &val.kind { ValueKind::Boolean(b) => *b, _ => panic!("Expected boolean, got {:?}", val.kind) }
}

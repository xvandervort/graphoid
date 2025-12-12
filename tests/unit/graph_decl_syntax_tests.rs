//! Tests for the new graph declaration syntax: graph Name { properties, methods }

use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
use graphoid::ast::*;
use graphoid::execution::Executor;
use graphoid::values::ValueKind;

// ============================================================================
// PARSER TESTS - Verify AST structure
// ============================================================================

#[test]
fn test_parse_empty_graph_decl() {
    let source = "graph Point { }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::GraphDecl { name, graph_type, parent, properties, methods, .. } => {
            assert_eq!(name, "Point");
            assert!(graph_type.is_none());
            assert!(parent.is_none());
            assert!(properties.is_empty());
            assert!(methods.is_empty());
        }
        other => panic!("Expected GraphDecl, got {:?}", other),
    }
}

#[test]
fn test_parse_graph_with_properties() {
    let source = r#"
graph Point {
    x: 0
    y: 0
}
"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::GraphDecl { name, properties, methods, .. } => {
            assert_eq!(name, "Point");
            assert_eq!(properties.len(), 2);
            assert_eq!(properties[0].name, "x");
            assert_eq!(properties[1].name, "y");
            assert!(methods.is_empty());
        }
        other => panic!("Expected GraphDecl, got {:?}", other),
    }
}

#[test]
fn test_parse_graph_with_single_method() {
    let source = r#"
graph Counter {
    fn increment() {
        self.count = 1
    }
}
"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::GraphDecl { name, properties, methods, .. } => {
            assert_eq!(name, "Counter");
            assert!(properties.is_empty());
            assert_eq!(methods.len(), 1);
            assert_eq!(methods[0].name, "increment");
            assert!(methods[0].params.is_empty());
        }
        other => panic!("Expected GraphDecl, got {:?}", other),
    }
}

#[test]
fn test_parse_graph_with_properties_and_methods() {
    let source = r#"
graph Counter {
    count: 0

    fn increment() {
        self.count = self.count + 1
    }

    fn value() {
        return self.count
    }
}
"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::GraphDecl { name, properties, methods, .. } => {
            assert_eq!(name, "Counter");
            assert_eq!(properties.len(), 1);
            assert_eq!(properties[0].name, "count");
            assert_eq!(methods.len(), 2);
            assert_eq!(methods[0].name, "increment");
            assert_eq!(methods[1].name, "value");
        }
        other => panic!("Expected GraphDecl, got {:?}", other),
    }
}

#[test]
fn test_parse_graph_with_method_parameters() {
    let source = r#"
graph Math {
    fn add(a, b) {
        return a + b
    }
}
"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::GraphDecl { methods, .. } => {
            assert_eq!(methods.len(), 1);
            assert_eq!(methods[0].name, "add");
            assert_eq!(methods[0].params.len(), 2);
            assert_eq!(methods[0].params[0].name, "a");
            assert_eq!(methods[0].params[1].name, "b");
        }
        other => panic!("Expected GraphDecl, got {:?}", other),
    }
}

#[test]
fn test_parse_graph_with_default_parameters() {
    let source = r#"
graph Greeter {
    fn greet(name = "World") {
        print("Hello, " + name)
    }
}
"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::GraphDecl { methods, .. } => {
            assert_eq!(methods[0].params.len(), 1);
            assert_eq!(methods[0].params[0].name, "name");
            assert!(methods[0].params[0].default_value.is_some());
        }
        other => panic!("Expected GraphDecl, got {:?}", other),
    }
}

#[test]
fn test_parse_graph_inheritance() {
    let source = r#"
graph Dog from Animal {
    sound: "woof"
}
"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::GraphDecl { name, parent, properties, .. } => {
            assert_eq!(name, "Dog");
            assert!(parent.is_some());
            // Parent should be a Variable expression referencing "Animal"
            if let Some(parent_expr) = parent {
                match parent_expr.as_ref() {
                    Expr::Variable { name: parent_name, .. } => {
                        assert_eq!(parent_name, "Animal");
                    }
                    other => panic!("Expected Variable for parent, got {:?}", other),
                }
            }
            assert_eq!(properties.len(), 1);
            assert_eq!(properties[0].name, "sound");
        }
        other => panic!("Expected GraphDecl, got {:?}", other),
    }
}

#[test]
fn test_parse_graph_with_type() {
    let source = r#"
graph TaskGraph(:dag) {
    tasks: []
}
"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::GraphDecl { name, graph_type, properties, .. } => {
            assert_eq!(name, "TaskGraph");
            assert_eq!(graph_type.as_deref(), Some("dag"));
            assert_eq!(properties.len(), 1);
        }
        other => panic!("Expected GraphDecl, got {:?}", other),
    }
}

#[test]
fn test_parse_graph_with_getter() {
    let source = r#"
graph Rectangle {
    width: 0
    height: 0

    get area() {
        return self.width * self.height
    }
}
"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::GraphDecl { methods, .. } => {
            assert_eq!(methods.len(), 1);
            assert_eq!(methods[0].name, "area");
            assert!(methods[0].is_getter);
            assert!(!methods[0].is_setter);
        }
        other => panic!("Expected GraphDecl, got {:?}", other),
    }
}

#[test]
fn test_parse_graph_with_setter() {
    let source = r#"
graph Temperature {
    _celsius: 0

    set celsius(value) {
        self._celsius = value
    }
}
"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::GraphDecl { methods, .. } => {
            assert_eq!(methods.len(), 1);
            assert_eq!(methods[0].name, "celsius");
            assert!(!methods[0].is_getter);
            assert!(methods[0].is_setter);
            assert_eq!(methods[0].params.len(), 1);
        }
        other => panic!("Expected GraphDecl, got {:?}", other),
    }
}

#[test]
fn test_parse_graph_with_private_method() {
    let source = r#"
graph Secret {
    priv fn helper() {
        return 42
    }
}
"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Stmt::GraphDecl { methods, .. } => {
            assert_eq!(methods.len(), 1);
            assert_eq!(methods[0].name, "helper");
            assert!(methods[0].is_private);
        }
        other => panic!("Expected GraphDecl, got {:?}", other),
    }
}

#[test]
fn test_parse_graph_flexible_separators() {
    // Commas
    let source1 = "graph P { x: 0, y: 0 }";
    let mut lexer = Lexer::new(source1);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    match &program.statements[0] {
        Stmt::GraphDecl { properties, .. } => {
            assert_eq!(properties.len(), 2);
        }
        _ => panic!("Expected GraphDecl"),
    }

    // Semicolons
    let source2 = "graph P { x: 0; y: 0 }";
    let mut lexer = Lexer::new(source2);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    match &program.statements[0] {
        Stmt::GraphDecl { properties, .. } => {
            assert_eq!(properties.len(), 2);
        }
        _ => panic!("Expected GraphDecl"),
    }
}

// ============================================================================
// EXECUTOR TESTS - Verify runtime behavior
// ============================================================================

#[test]
fn test_exec_graph_decl_creates_binding() {
    let source = "graph Point { x: 0, y: 0 }";
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // Point should be defined in the environment
    let point = executor.get_variable("Point");
    assert!(point.is_some(), "Point should be defined");
}

#[test]
fn test_exec_graph_decl_has_intrinsic_name() {
    let source = r#"
graph Animal {
    sound: "..."
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let animal = executor.get_variable("Animal").unwrap();
    match &animal.kind {
        ValueKind::Graph(g) => {
            assert_eq!(g.type_name, Some("Animal".to_string()));
        }
        _ => panic!("Expected graph"),
    }
}

#[test]
fn test_exec_graph_decl_properties_as_nodes() {
    let source = r#"
graph Point {
    x: 10
    y: 20
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let point = executor.get_variable("Point").unwrap();
    match &point.kind {
        ValueKind::Graph(g) => {
            assert!(g.has_node("x"));
            assert!(g.has_node("y"));
            let x_val = g.get_node("x").unwrap();
            match &x_val.kind {
                ValueKind::Number(n) => assert_eq!(*n, 10.0),
                _ => panic!("Expected number"),
            }
        }
        _ => panic!("Expected graph"),
    }
}

#[test]
fn test_exec_graph_decl_methods_attached() {
    let source = r#"
graph Counter {
    count: 0

    fn increment() {
        self.count = self.count + 1
    }
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let counter = executor.get_variable("Counter").unwrap();
    match &counter.kind {
        ValueKind::Graph(g) => {
            assert!(g.has_method("increment"));
        }
        _ => panic!("Expected graph"),
    }
}

#[test]
fn test_exec_graph_clone_and_method_call() {
    let source = r#"
graph Counter {
    count: 0

    fn increment() {
        self.count = self.count + 1
    }

    fn value() {
        return self.count
    }
}

c = Counter.clone()
c.increment()
c.increment()
result = c.value()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 2.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_exec_graph_inheritance() {
    let source = r#"
graph Animal {
    name: "unknown"
    sound: "..."

    fn speak() {
        return self.name + " says " + self.sound
    }
}

graph Dog from Animal {
    sound: "woof!"
}

fido = Dog.clone()
fido.name = "Fido"
result = fido.speak()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::String(s) => {
            assert_eq!(s, "Fido says woof!");
        }
        _ => panic!("Expected string"),
    }
}

#[test]
fn test_exec_graph_dag_type() {
    let source = r#"
graph TaskGraph(:dag) {
    tasks: []
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let graph = executor.get_variable("TaskGraph").unwrap();
    match &graph.kind {
        ValueKind::Graph(g) => {
            assert!(g.rulesets.contains(&"dag".to_string()));
        }
        _ => panic!("Expected graph"),
    }
}

#[test]
fn test_exec_graph_private_method_renamed() {
    let source = r#"
graph Secret {
    priv fn helper() {
        return 42
    }
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let secret = executor.get_variable("Secret").unwrap();
    match &secret.kind {
        ValueKind::Graph(g) => {
            // Private method should be renamed with underscore prefix
            assert!(g.has_method("_helper"));
            assert!(!g.has_method("helper"));
        }
        _ => panic!("Expected graph"),
    }
}

#[test]
fn test_exec_multiple_graph_declarations() {
    let source = r#"
graph Point { x: 0, y: 0 }
graph Circle { radius: 1 }
graph Rectangle { width: 0, height: 0 }
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    assert!(executor.get_variable("Point").is_some());
    assert!(executor.get_variable("Circle").is_some());
    assert!(executor.get_variable("Rectangle").is_some());
}

// ============================================================================
// ERROR TESTS - Verify proper error handling
// ============================================================================

#[test]
fn test_parse_anonymous_graph_still_works() {
    // graph { x: 0 } without a name is an anonymous graph expression (old style)
    // This should fall back to the expression parser, not the declaration parser
    let source = "g = graph { }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);

    let result = parser.parse();
    assert!(result.is_ok(), "Anonymous graph expression should still work");

    // Verify it's parsed as an assignment, not a declaration
    let program = result.unwrap();
    match &program.statements[0] {
        Stmt::VariableDecl { name, value, .. } => {
            assert_eq!(name, "g");
            match value {
                Expr::Graph { .. } => {} // Good - it's a graph expression
                other => panic!("Expected Graph expression, got {:?}", other),
            }
        }
        Stmt::Assignment { .. } => {} // Also acceptable
        other => panic!("Expected VariableDecl or Assignment, got {:?}", other),
    }
}

#[test]
fn test_parse_error_missing_brace() {
    let source = "graph Point x: 0 }";  // Missing opening brace
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_parse_error_missing_colon_in_property() {
    let source = "graph Point { x 0 }";  // Missing colon
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
}

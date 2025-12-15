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

// ============================================================================
// PHASE 2: IMPLICIT SELF RESOLUTION TESTS
// ============================================================================

#[test]
fn test_implicit_self_read_property() {
    // Reading a property without explicit self.
    let source = r#"
graph Counter {
    count: 0

    fn value() {
        return count  # Should resolve to self.count
    }
}

c = Counter.clone()
c.count = 42
result = c.value()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 42.0),
        other => panic!("Expected number, got {:?}", other),
    }
}

#[test]
fn test_implicit_self_write_property() {
    // Writing a property without explicit self.
    let source = r#"
graph Counter {
    count: 0

    fn set_count(n) {
        count = n  # Should assign to self.count
    }

    fn value() {
        return self.count
    }
}

c = Counter.clone()
c.set_count(99)
result = c.value()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 99.0),
        other => panic!("Expected number, got {:?}", other),
    }
}

#[test]
fn test_implicit_self_read_and_write() {
    // Both reading and writing without explicit self.
    let source = r#"
graph Counter {
    count: 0

    fn increment() {
        count = count + 1  # Should work without self.
    }

    fn value() {
        return count
    }
}

c = Counter.clone()
c.increment()
c.increment()
c.increment()
result = c.value()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 3.0),
        other => panic!("Expected number, got {:?}", other),
    }
}

#[test]
fn test_implicit_self_parameters_override() {
    // Parameters should take precedence over graph properties
    let source = r#"
graph Calculator {
    value: 100

    fn set_value(value) {
        # 'value' here refers to the parameter, not self.value
        self.value = value
    }

    fn get_value() {
        return self.value
    }
}

calc = Calculator.clone()
calc.set_value(50)
result = calc.get_value()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 50.0),
        other => panic!("Expected number, got {:?}", other),
    }
}

#[test]
fn test_implicit_self_local_vars_for_new_names() {
    // With implicit self, assignment to a property name updates the property.
    // True local variables are created only for names that DON'T exist on self.
    let source = r#"
graph Example {
    x: 100

    fn test_with_local() {
        # 'y' is not a property, so this creates a local variable
        y = 5
        # 'x' IS a property, so this updates self.x
        x = x + y
        return y  # Returns local variable
    }

    fn get_x() {
        return x  # implicit self.x
    }
}

ex = Example.clone()
local_result = ex.test_with_local()
prop_result = ex.get_x()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    // local_result should be 5 (the local 'y' variable)
    let local_result = executor.get_variable("local_result").unwrap();
    match &local_result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 5.0),
        other => panic!("Expected number for local, got {:?}", other),
    }

    // prop_result should be 105 (x was updated to x + y = 100 + 5)
    let prop_result = executor.get_variable("prop_result").unwrap();
    match &prop_result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 105.0),
        other => panic!("Expected number for property, got {:?}", other),
    }
}

#[test]
fn test_implicit_self_method_call() {
    // Calling another method without explicit self.
    let source = r#"
graph Calculator {
    value: 0

    fn add(n) {
        value = value + n
    }

    fn double() {
        add(value)  # Should call self.add(self.value)
    }

    fn result() {
        return value
    }
}

calc = Calculator.clone()
calc.value = 5
calc.double()
result = calc.result()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 10.0),
        other => panic!("Expected number, got {:?}", other),
    }
}

#[test]
fn test_implicit_self_explicit_still_works() {
    // Explicit self. should still work
    let source = r#"
graph Counter {
    count: 0

    fn mixed() {
        count = count + 1        # implicit
        self.count = self.count + 1  # explicit
        return count  # implicit read
    }
}

c = Counter.clone()
result = c.mixed()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 2.0),
        other => panic!("Expected number, got {:?}", other),
    }
}

#[test]
fn test_implicit_self_multiple_properties() {
    // Multiple properties accessed implicitly
    let source = r#"
graph Point {
    x: 0
    y: 0

    fn move_by(dx, dy) {
        x = x + dx
        y = y + dy
    }

    fn distance_from_origin() {
        return (x * x + y * y)
    }
}

p = Point.clone()
p.move_by(3, 4)
result = p.distance_from_origin()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 25.0),  // 3*3 + 4*4 = 25
        other => panic!("Expected number, got {:?}", other),
    }
}

#[test]
fn test_implicit_self_in_getter() {
    // Implicit self should work in getters too
    let source = r#"
graph Rectangle {
    width: 0
    height: 0

    get area() {
        return width * height  # implicit self
    }
}

r = Rectangle.clone()
r.width = 5
r.height = 3
result = r.area
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 15.0),
        other => panic!("Expected number, got {:?}", other),
    }
}

#[test]
fn test_implicit_self_with_inheritance() {
    // Implicit self should work with inherited properties
    let source = r#"
graph Animal {
    name: "unknown"
    sound: "..."

    fn speak() {
        return name + " says " + sound  # implicit self
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
        ValueKind::String(s) => assert_eq!(s, "Fido says woof!"),
        other => panic!("Expected string, got {:?}", other),
    }
}

// ============================================================================
// PHASE 4: CONFIGURE BLOCK TESTS
// ============================================================================

#[test]
fn test_configure_readable_generates_getter() {
    // configure { readable: [:x, :y] } generates getter methods
    let source = r#"
graph Point {
    configure { readable: [:x, :y] }

    x: 10
    y: 20
}

p = Point.clone()
result_x = p.x()
result_y = p.y()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result_x = executor.get_variable("result_x").unwrap();
    match &result_x.kind {
        ValueKind::Number(n) => assert_eq!(*n, 10.0),
        other => panic!("Expected number, got {:?}", other),
    }

    let result_y = executor.get_variable("result_y").unwrap();
    match &result_y.kind {
        ValueKind::Number(n) => assert_eq!(*n, 20.0),
        other => panic!("Expected number, got {:?}", other),
    }
}

#[test]
fn test_configure_writable_generates_setter() {
    // configure { writable: :x } generates a set_x(value) method
    let source = r#"
graph Point {
    configure { writable: :x }

    x: 0
}

p = Point.clone()
p.set_x(42)
result = p.x
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 42.0),
        other => panic!("Expected number, got {:?}", other),
    }
}

#[test]
fn test_configure_accessible_generates_both() {
    // configure { accessible: :count } generates both getter and setter
    let source = r#"
graph Counter {
    configure { accessible: :count }

    count: 0
}

c = Counter.clone()
c.set_count(10)
result = c.count()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 10.0),
        other => panic!("Expected number, got {:?}", other),
    }
}

#[test]
fn test_configure_multiple_symbols() {
    // configure with list of multiple symbols
    let source = r#"
graph Rectangle {
    configure { readable: [:width, :height] }

    width: 5
    height: 3
}

r = Rectangle.clone()
w = r.width()
h = r.height()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let w = executor.get_variable("w").unwrap();
    match &w.kind {
        ValueKind::Number(n) => assert_eq!(*n, 5.0),
        other => panic!("Expected number for width, got {:?}", other),
    }

    let h = executor.get_variable("h").unwrap();
    match &h.kind {
        ValueKind::Number(n) => assert_eq!(*n, 3.0),
        other => panic!("Expected number for height, got {:?}", other),
    }
}

#[test]
fn test_configure_explicit_method_not_overwritten() {
    // If user defines a method, configure should not overwrite it
    let source = r#"
graph Custom {
    configure { readable: :x }

    x: 10

    fn x() {
        return x * 2  # Custom getter that doubles
    }
}

c = Custom.clone()
result = c.x()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 20.0),  // Custom method returns 10 * 2 = 20
        other => panic!("Expected number, got {:?}", other),
    }
}

#[test]
fn test_configure_syntax() {
    // Verify parsing works with different configure options
    let source = r#"
graph Example {
    configure {
        readable: :a
        writable: :b
        accessible: :c
    }

    a: 1
    b: 2
    c: 3
}

e = Example.clone()
"#;
    let mut executor = Executor::new();
    // Should parse and execute without error
    executor.execute_source(source).unwrap();
}

// ============================================================================
// PRIVATE METHOD TESTS (using priv fn)
// ============================================================================

#[test]
fn test_priv_fn_external_call_blocked() {
    // External call to private method (priv fn) should fail
    let source = r#"
graph Secret {
    priv fn internal_helper() {
        return 42
    }
}

s = Secret.clone()
result = s._internal_helper()
"#;
    let mut executor = Executor::new();
    let result = executor.execute_source(source);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("private"));
}

#[test]
fn test_priv_fn_internal_call_allowed() {
    // Internal call to private method should work
    let source = r#"
graph Secret {
    secret: 42

    fn get_secret() {
        return self.secret
    }
}

s = Secret.clone()
result = s.get_secret()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 42.0),
        other => panic!("Expected number, got {:?}", other),
    }
}

#[test]
fn test_priv_fn_called_internally_via_underscore() {
    // Private method called internally via underscore prefix
    let source = r#"
graph Secret {
    priv fn helper() {
        return 42
    }

    fn public_method() {
        return _helper()  # Call via underscore prefix internally
    }
}

s = Secret.clone()
result = s.public_method()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 42.0),
        other => panic!("Expected number, got {:?}", other),
    }
}

// ============================================================================
// PHASE 5: RULE KEYWORD TESTS
// ============================================================================

#[test]
fn test_rule_keyword_basic() {
    // rule :name inside graph body
    let source = r#"
graph Tree {
    rule :no_cycles

    root: "A"
}

t = Tree.clone()
result = t.has_rule(:no_cycles)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::Boolean(b) => assert!(*b),
        other => panic!("Expected boolean true, got {:?}", other),
    }
}

#[test]
fn test_rule_keyword_with_parameter() {
    // rule :name, param inside graph body
    let source = r#"
graph BinaryTree {
    rule :max_degree, 2

    root: "root"
}

bt = BinaryTree.clone()
has_rule = bt.has_rule(:max_degree)
param_value = bt.rule(:max_degree)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let has_rule = executor.get_variable("has_rule").unwrap();
    match &has_rule.kind {
        ValueKind::Boolean(b) => assert!(*b),
        other => panic!("Expected boolean true, got {:?}", other),
    }

    let param_value = executor.get_variable("param_value").unwrap();
    match &param_value.kind {
        ValueKind::Number(n) => assert_eq!(*n, 2.0),
        other => panic!("Expected number 2, got {:?}", other),
    }
}

#[test]
fn test_rule_keyword_multiple_rules() {
    // Multiple rule declarations
    let source = r#"
graph DAG {
    rule :no_cycles
    rule :single_root

    start: "entry"
}

d = DAG.clone()
has_no_cycles = d.has_rule(:no_cycles)
has_single_root = d.has_rule(:single_root)
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let has_no_cycles = executor.get_variable("has_no_cycles").unwrap();
    match &has_no_cycles.kind {
        ValueKind::Boolean(b) => assert!(*b),
        other => panic!("Expected boolean true, got {:?}", other),
    }

    let has_single_root = executor.get_variable("has_single_root").unwrap();
    match &has_single_root.kind {
        ValueKind::Boolean(b) => assert!(*b),
        other => panic!("Expected boolean true, got {:?}", other),
    }
}

#[test]
fn test_rule_keyword_with_configure() {
    // Rules can coexist with configure block
    let source = r#"
graph ValidatedCounter {
    configure { readable: :count }
    rule :no_cycles

    count: 0
}

vc = ValidatedCounter.clone()
has_rule = vc.has_rule(:no_cycles)
count_value = vc.count()
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let has_rule = executor.get_variable("has_rule").unwrap();
    match &has_rule.kind {
        ValueKind::Boolean(b) => assert!(*b),
        other => panic!("Expected boolean true, got {:?}", other),
    }

    let count_value = executor.get_variable("count_value").unwrap();
    match &count_value.kind {
        ValueKind::Number(n) => assert_eq!(*n, 0.0),
        other => panic!("Expected number 0, got {:?}", other),
    }
}

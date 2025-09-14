# Pattern Matching Design for Glang

## Overview

Pattern matching enables elegant destructuring and conditional logic, essential for error-as-data patterns and complex data handling.

## Syntax Design

### Basic Match Expression

```glang
match expression {
    pattern1 => result1
    pattern2 => result2
    _ => default_result
}
```

### Result Pattern Matching (Primary Use Case)

```glang
result = some_function()
match result {
    [:ok, value] => {
        print("Success: " + value.to_string())
        value
    }
    [:error, message] => {
        print("Error: " + message)
        none
    }
}
```

### List Pattern Matching

```glang
match items {
    [] => "empty list"
    [first] => "single item: " + first.to_string()
    [first, second] => "two items: " + first.to_string() + ", " + second.to_string()
    [first, ...rest] => "first: " + first.to_string() + ", rest has " + rest.size().to_string() + " items"
}
```

### Literal Value Matching

```glang
match status_code {
    200 => "OK"
    404 => "Not Found"
    500 => "Server Error"
    _ => "Unknown status"
}
```

### Variable Binding in Patterns

```glang
match data {
    [:ok, user_data] => {
        # user_data is automatically bound from the pattern
        "User: " + user_data["name"]
    }
    [:error, error_info] => {
        # error_info is automatically bound
        "Error: " + error_info
    }
}
```

### Guards (Future Enhancement)

```glang
match number {
    n if n > 0 => "positive"
    n if n < 0 => "negative"
    0 => "zero"
}
```

## Pattern Types

### 1. Literal Patterns
- Numbers: `42`, `3.14`
- Strings: `"hello"`
- Booleans: `true`, `false`
- Symbols: `:ok`, `:error` (internal use only)

### 2. Variable Patterns
- Binds matched value to variable: `x`, `value`, `message`

### 3. Wildcard Pattern
- Matches anything: `_`

### 4. List Patterns
- Empty list: `[]`
- Fixed elements: `[a, b, c]`
- Head/tail: `[first, ...rest]`
- Mixed: `[:ok, value]`, `[:error, message]`

### 5. Data Node Patterns (Future)
- `{ "key": value }`
- `{ "name": user_name, "age": user_age }`

## Implementation Phases

### Phase 1: Core Infrastructure
- [ ] Add `match`, `=>` tokens to lexer
- [ ] Create `MatchExpression` and `Pattern` AST nodes
- [ ] Basic parsing for match expressions

### Phase 2: Basic Patterns
- [ ] Literal pattern matching (numbers, strings, booleans)
- [ ] Variable binding
- [ ] Wildcard patterns

### Phase 3: List Patterns
- [ ] Empty list patterns
- [ ] Fixed-length list patterns
- [ ] Head/tail destructuring with `...rest`

### Phase 4: Symbol Patterns
- [ ] Status symbol patterns (`:ok`, `:error`)
- [ ] Result tuple patterns (`[:ok, value]`, `[:error, message]`)

### Phase 5: Advanced Features
- [ ] Guards with `if` conditions
- [ ] Exhaustiveness checking
- [ ] Performance optimization

## AST Design

```python
@dataclass
class MatchExpression(Expression):
    """Match expression: match expr { pattern => result, ... }"""
    expr: Expression
    arms: List[MatchArm]
    position: Optional[SourcePosition] = None

@dataclass
class MatchArm:
    """Single match arm: pattern => result"""
    pattern: Pattern
    result: Expression
    guard: Optional[Expression] = None  # Future: for 'if' guards

@dataclass
class Pattern(ASTNode):
    """Base class for all patterns"""
    pass

@dataclass
class LiteralPattern(Pattern):
    """Literal value pattern: 42, "hello", true"""
    value: Any
    position: Optional[SourcePosition] = None

@dataclass
class VariablePattern(Pattern):
    """Variable binding pattern: x, value, message"""
    name: str
    position: Optional[SourcePosition] = None

@dataclass
class WildcardPattern(Pattern):
    """Wildcard pattern: _"""
    position: Optional[SourcePosition] = None

@dataclass
class ListPattern(Pattern):
    """List pattern: [], [a, b], [first, ...rest]"""
    elements: List[Pattern]
    rest_variable: Optional[str] = None  # For ...rest syntax
    position: Optional[SourcePosition] = None
```

## Execution Semantics

### Pattern Matching Algorithm

1. **Evaluate match expression** to get target value
2. **Try each arm in order** until one matches
3. **For each arm**:
   - Check if pattern matches target value
   - If match succeeds, bind variables from pattern
   - Execute result expression with bindings
   - Return result value
4. **If no arms match**, raise runtime error

### Variable Binding

When a pattern matches, variables in the pattern are bound to corresponding values:

```glang
match [:ok, "user123"] {
    [:ok, user_id] => {
        # user_id is bound to "user123"
        print("Found user: " + user_id)
    }
}
```

### Binding Scope

Pattern variables are scoped to their match arm:

```glang
match result {
    [:ok, value] => {
        # 'value' is available here
        process(value)
    }
    [:error, message] => {
        # 'message' is available here
        # 'value' is NOT available
        log_error(message)
    }
}
# Neither 'value' nor 'message' available here
```

## Error Handling

### Non-Exhaustive Matches

If no pattern matches, runtime error:
```
MatchError: No pattern matched value [:warning, "deprecated"] at line 15
```

### Wildcard Safety

Always include wildcard for safety:
```glang
match status {
    :ok => "success"
    :error => "failure"
    _ => "unknown status"  # Catches any new status symbols
}
```

## Integration with Error-as-Data

Pattern matching makes error-as-data practical:

```glang
# Function returns result tuple
func find_user(id) {
    if database.has_user(id) {
        return [:ok, database.get_user(id)]
    } else {
        return [:error, "User not found"]
    }
}

# Clean usage with pattern matching
user_result = find_user("123")
match user_result {
    [:ok, user] => {
        print("Welcome " + user["name"])
        user
    }
    [:error, message] => {
        print("Login failed: " + message)
        none
    }
}
```

This design enables elegant error handling without exceptions, making error paths explicit and composable.
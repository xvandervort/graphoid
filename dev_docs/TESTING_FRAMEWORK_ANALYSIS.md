# Glang Testing Framework Analysis

*Comprehensive analysis of testing framework requirements and design approaches - January 2025*

## Problem Statement

After investigating RSpec-style testing for Glang, it became clear that direct translation of Ruby's implicit block syntax (`describe "test" do...end`) would require either:
1. Significant parser/AST changes for implicit blocks
2. Clunky workarounds like `func describe(description, block)` with explicit lambdas
3. String-based DSLs requiring eval() functionality

None of these approaches align with Glang's design principles of intuitive syntax and graph-theoretic foundations.

## Core Requirements for Glang Testing

### Essential Capabilities
- **Simple**: Minimal boilerplate, clear syntax
- **Readable**: Self-documenting test structure
- **Flexible**: Support setup/teardown, shared examples, custom matchers
- **No Clunky Workarounds**: Avoid forced lambda parameters or string eval
- **Recognizably Glangish**: Test suites as graphs with tests as subgraphs
- **Graph-Theoretic**: Leverage Glang's native graph structures

### Additional Requirements
- **let statements**: Lazy evaluation for test data setup
- **before/after hooks**: Setup and teardown capabilities
- **Nested contexts**: Hierarchical test organization
- **Rich assertions**: Expressive expectation syntax
- **Test discovery**: Automatic finding and running of tests
- **Failure reporting**: Clear error messages with context

## Current Capabilities Analysis

### What Works Well
```glang
# Test suite as a structured data object
test_suite = TestSuite("User authentication")
test_suite.add_test("validates correct credentials", validate_correct_test)
test_suite.add_test("rejects invalid credentials", validate_invalid_test)

# Test functions with clear names
func validate_correct_test() {
    user = User("alice", "password123")
    expect(user.authenticate("password123")).to.be(true)
}

# Assertion capabilities using existing reflection
expect(value).to.equal(42)
expect(user).to.respond_to("authenticate")
expect(list).to.contain("item")
```

### Missing Critical Features
- **let statements**: No lazy evaluation mechanism
- **Context sharing**: No way to share setup between tests
- **Hierarchical organization**: No nested test structure
- **Graph integration**: Not leveraging Glang's graph nature

## Potential Glangish Approaches

### Approach 1: Graph-Based Test Structure
```glang
# Test suite as an actual graph structure
auth_tests = TestGraph("User Authentication") {
    # Nodes are individual tests
    test("validates correct credentials") {
        # Test implementation
    }

    test("rejects invalid credentials") {
        # Test implementation
    }

    # Subgraphs for nested contexts
    context("password requirements") {
        test("requires minimum length") { }
        test("requires special characters") { }
    }
}
```

### Approach 2: Declaration-Based Syntax
```glang
# Tests as declarations, not function calls
test_suite "User Authentication"

let user = User("alice", "password123")
let invalid_user = User("bob", "wrongpass")

before_each {
    database.clear()
    user.save()
}

test "validates correct credentials" {
    result = user.authenticate("password123")
    expect(result).to.be(true)
}

test "rejects invalid credentials" {
    result = user.authenticate("wrongpass")
    expect(result).to.be(false)
}
```

### Approach 3: Module-Based Organization
```glang
module UserAuthenticationTests {
    # let statements as module-level declarations
    let user = lazy(() => User("alice", "password123"))
    let database = lazy(() => TestDatabase())

    # Setup/teardown as special functions
    func setup() {
        database().clear()
    }

    # Tests as specially named functions
    func test_validates_correct_credentials() {
        result = user().authenticate("password123")
        assert_equal(result, true)
    }

    func test_rejects_invalid_credentials() {
        result = user().authenticate("wrongpass")
        assert_equal(result, false)
    }
}
```

## Language Feature Requirements

### Required for Elegant Testing
1. **Lazy Evaluation**: `let` statements that defer computation until first access
2. **Block Syntax**: Either implicit blocks or more natural lambda syntax
3. **Declaration Keywords**: `test`, `before_each`, `after_each` as language constructs
4. **Graph Literals**: Native syntax for creating test graph structures
5. **Pattern Matching**: For sophisticated assertion patterns

### Nice-to-Have Enhancements
1. **Macro System**: Custom syntax transformations
2. **String Interpolation**: Better assertion message formatting
3. **Symbol Literals**: `:passed`, `:failed` status indicators
4. **Module Import/Export**: Better test organization across files

## Proposed Design Parameters

### Graph-Theoretic Foundation
- **Test Suite**: Root graph node containing all tests
- **Test Cases**: Individual nodes with execution functions
- **Contexts**: Subgraphs for logical grouping
- **Setup/Teardown**: Edge relationships between setup nodes and test nodes
- **Dependencies**: Edges representing test ordering or shared data

### Syntax Goals
```glang
# Ideal Glangish testing syntax (requires language enhancements)
TestGraph("Authentication") {
    let user = lazy(() => User("alice", "password"))

    before_each { database.reset() }

    test("correct credentials") {
        expect(user.authenticate("password")).to.be(true)
    }

    context("invalid attempts") {
        test("wrong password") {
            expect(user.authenticate("wrong")).to.be(false)
        }

        test("empty password") {
            expect(user.authenticate("")).to.be(false)
        }
    }
}
```

## Recommendation

The testing framework should be **deferred pending a workable design** that:

1. **Leverages Graph Structures**: Tests as graph nodes, contexts as subgraphs
2. **Adds Language Support**: `let`, `test`, `before_each` as language constructs
3. **Maintains Simplicity**: No forced lambdas or string-based workarounds
4. **Enables Rich Assertions**: Using existing reflection capabilities
5. **Supports Test Discovery**: Automatic finding of test graphs/modules

## Next Steps

1. **Design lazy evaluation** (`let` statements) for the core language
2. **Add declaration syntax** for test, before_each, after_each
3. **Implement graph literals** for natural test structure creation
4. **Create assertion DSL** using existing reflection methods
5. **Build test runner** that traverses test graphs and executes nodes

This analysis suggests that a truly elegant, Glangish testing framework requires several core language enhancements before implementation should begin.

---

**Status**: Deferred pending language design enhancements
**Priority**: High (critical for self-hosting goals)
**Dependencies**: Lazy evaluation, declaration syntax, graph literals
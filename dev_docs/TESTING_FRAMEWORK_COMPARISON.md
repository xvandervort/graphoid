# Testing Framework Comparison: Traditional vs RSpec-Style

**Date**: January 2025

This document compares the traditional assert-based testing approach with the RSpec-style behavior-driven approach now specified for Graphoid.

---

## Simple Test Case

### ❌ Traditional Assert Style (Original)

```glang
# calculator.test.gr
import "assert"
import "../src/calculator"

func test_addition() {
    result = calculator.add(2, 3)
    assert.equal(result, 5, "2 + 3 should equal 5")
}

func test_subtraction() {
    result = calculator.subtract(10, 4)
    assert.equal(result, 6)
}

func test_division_by_zero() {
    assert.raises(func() {
        calculator.divide(10, 0)
    }, "RuntimeError")
}
```

**Output**:
```
tests/calculator.test.gr
  ✓ test_addition (0.2ms)
  ✓ test_subtraction (0.1ms)
  ✗ test_division_by_zero
    Expected: RuntimeError
    Got: No error
```

**Problems**:
- Function names like `test_addition` are not natural language
- No clear grouping or hierarchy
- Hard to see what's being tested
- Output is flat, no structure

---

### ✅ RSpec-Style (New Specification)

```glang
# calculator.spec.gr
import "spec"
import "../src/calculator"

describe "Calculator" {

    describe "add" {
        it "adds two positive numbers" {
            result = calculator.add(2, 3)
            expect(result).to_equal(5)
        }

        it "adds negative numbers" {
            result = calculator.add(-2, -3)
            expect(result).to_equal(-5)
        }
    }

    describe "subtract" {
        it "subtracts positive numbers" {
            result = calculator.subtract(10, 4)
            expect(result).to_equal(6)
        }
    }

    describe "divide" {
        context "when dividing by zero" {
            it "raises an error" {
                expect(func() {
                    calculator.divide(10, 0)
                }).to_raise("RuntimeError")
            }
        }
    }
}
```

**Output**:
```
Calculator
  add
    ✓ adds two positive numbers (0.2ms)
    ✓ adds negative numbers (0.1ms)
  subtract
    ✓ subtracts positive numbers (0.1ms)
  divide
    when dividing by zero
      ✗ raises an error
        Expected RuntimeError to be raised
        But got no error
        at line 25 in specs/calculator.spec.gr
```

**Benefits**:
- ✅ Natural language descriptions
- ✅ Clear hierarchical structure
- ✅ Easy to see what's being tested
- ✅ Output reads like documentation
- ✅ Context blocks for scenarios

---

## Complex Example: Authentication System

### ❌ Traditional Assert Style

```glang
import "assert"
import "../src/auth"

func test_login_with_correct_password() {
    user = create_user("alice", "password123")
    result = auth.login("alice", "password123")
    assert.true(result)
}

func test_login_with_incorrect_password() {
    user = create_user("alice", "password123")
    result = auth.login("alice", "wrongpassword")
    assert.false(result)
}

func test_login_with_nonexistent_user() {
    assert.raises(func() {
        auth.login("nonexistent", "password")
    }, "UserNotFoundError")
}

func test_logout() {
    user = create_user("alice", "password123")
    auth.login("alice", "password123")
    result = auth.logout("alice")
    assert.true(result)
}
```

**Problems**:
- Repeated setup in every test
- No clear relationship between tests
- Hard to understand scenarios
- Long function names get unwieldy

---

### ✅ RSpec-Style (Much Better)

```glang
import "spec"
import "../src/auth"

describe "User authentication" {

    context "when user exists" {
        before_each {
            user = create_user("alice", "password123")
        }

        describe "login" {
            it "succeeds with correct password" {
                result = auth.login("alice", "password123")
                expect(result).to_be_truthy()
            }

            it "fails with incorrect password" {
                result = auth.login("alice", "wrongpassword")
                expect(result).to_be_falsy()
            }
        }

        describe "logout" {
            before_each {
                auth.login("alice", "password123")
            }

            it "successfully logs out the user" {
                result = auth.logout("alice")
                expect(result).to_be_truthy()
            }

            it "makes subsequent logins succeed" {
                auth.logout("alice")
                result = auth.login("alice", "password123")
                expect(result).to_be_truthy()
            }
        }
    }

    context "when user does not exist" {
        it "raises an error on login attempt" {
            expect(func() {
                auth.login("nonexistent", "password")
            }).to_raise("UserNotFoundError")
        }
    }
}
```

**Output**:
```
User authentication
  when user exists
    login
      ✓ succeeds with correct password (1.2ms)
      ✓ fails with incorrect password (1.1ms)
    logout
      ✓ successfully logs out the user (0.8ms)
      ✓ makes subsequent logins succeed (1.5ms)
  when user does not exist
    ✓ raises an error on login attempt (0.5ms)

Specs: 5 passed, 0 failed, 5 total
Time: 5.1s
```

**Benefits**:
- ✅ Setup code reused via `before_each`
- ✅ Clear scenarios with `context`
- ✅ Hierarchical organization
- ✅ Reads like a specification document

---

## Advanced Features Comparison

### ❌ Traditional: Parameterized Tests

```glang
import "assert"

test_data = [
    { input: 2, expected: 4 },
    { input: 3, expected: 9 },
    { input: 4, expected: 16 },
]

for case in test_data {
    func_name = "test_square_" + case.input.to_string()
    create_test(func_name, func() {
        result = square(case.input)
        assert.equal(result, case.expected)
    })
}
```

**Problems**:
- Meta-programming required
- Function names generated dynamically
- Hard to debug individual cases

---

### ✅ RSpec-Style: Table-Driven Tests

```glang
import "spec"

describe "square function" {
    where {
        input | expected
        2     | 4
        3     | 9
        4     | 16
        5     | 25
        -2    | 4
        -3    | 9
    }

    it "squares {input} to get {expected}" {
        result = square(input)
        expect(result).to_equal(expected)
    }
}
```

**Output**:
```
square function
  ✓ squares 2 to get 4 (0.1ms)
  ✓ squares 3 to get 9 (0.1ms)
  ✓ squares 4 to get 16 (0.1ms)
  ✓ squares 5 to get 25 (0.1ms)
  ✓ squares -2 to get 4 (0.1ms)
  ✓ squares -3 to get 9 (0.1ms)
```

**Benefits**:
- ✅ Clean table syntax
- ✅ Each case shows clearly in output
- ✅ Easy to add more cases
- ✅ No meta-programming needed

---

## Shared Behavior Comparison

### ❌ Traditional: No Built-in Support

```glang
# Would need to copy-paste or create helper functions
# No standardized way to share behavior
```

---

### ✅ RSpec-Style: Shared Examples

```glang
import "spec"

# Define once
shared_examples "a collection" {
    it "has a size method" {
        expect(collection.size).to_be_a("num")
    }

    it "can be empty" {
        collection.clear()
        expect(collection).to_be_empty()
    }

    it "reports correct size after modification" {
        original_size = collection.size()
        collection.clear()
        expect(collection.size()).to_equal(0)
    }
}

# Reuse multiple times
describe "List" {
    before_each {
        collection = [1, 2, 3]
    }

    it_behaves_like "a collection"

    it "supports indexing" {
        expect(collection[0]).to_equal(1)
    }
}

describe "Map" {
    before_each {
        collection = {"a": 1, "b": 2}
    }

    it_behaves_like "a collection"

    it "supports key access" {
        expect(collection["a"]).to_equal(1)
    }
}
```

**Benefits**:
- ✅ Define behavior once, use many times
- ✅ Ensures consistency across similar types
- ✅ Reduces duplication
- ✅ Makes contracts explicit

---

## Expectation Readability Comparison

### ❌ Traditional Assertions

```glang
assert.equal(user.age, 30)
assert.greater(score, 90)
assert.contains(list, item)
assert.close(pi, 3.14, 0.01)
assert.raises(func() { dangerous_op() }, "Error")
```

**Problems**:
- Order of arguments can be confusing (`actual, expected` or `expected, actual`?)
- Reads like code, not like a sentence
- Hard to chain or compose

---

### ✅ RSpec-Style Expectations

```glang
expect(user.age).to_equal(30)
expect(score).to_be_greater_than(90)
expect(list).to_contain(item)
expect(pi).to_be_close_to(3.14, 0.01)
expect(func() { dangerous_op() }).to_raise("Error")

# Can use negation naturally
expect(list).not_to_contain(excluded_item)
expect(user.age).not_to_equal(0)

# Chaining reads like English
expect(3.14159).to_be_within(0.01).of(3.14)
expect(value).to_be_between(0, 100)
```

**Benefits**:
- ✅ Reads like natural language
- ✅ Order is always `expect(actual).to_*(expected)`
- ✅ Negation is simple and clear
- ✅ Composable and chainable

---

## Summary

| Aspect | Traditional Assert | RSpec-Style |
|--------|-------------------|-------------|
| **Readability** | Code-like | Natural language |
| **Organization** | Flat functions | Hierarchical blocks |
| **Setup/Teardown** | Manual in each test | Hooks (`before_each`, etc.) |
| **Scenarios** | Hard to group | `context` blocks |
| **Parameterized Tests** | Meta-programming | Table syntax (`where`) |
| **Shared Behavior** | No support | `shared_examples` |
| **Output** | Flat list | Hierarchical tree |
| **Documentation Value** | Low | High (living docs) |
| **Test Names** | `test_*` functions | Natural language strings |

---

## Why RSpec-Style Wins

1. **Self-Documenting**: Specs read like documentation
2. **Better Organization**: Hierarchical structure mirrors code structure
3. **Clearer Intent**: `expect(x).to_equal(y)` is clearer than `assert.equal(x, y)`
4. **Proven**: RSpec has been successful for 15+ years in Ruby community
5. **Modern**: Jest (JavaScript), pytest (with plugins), and modern frameworks all move toward this style
6. **Living Documentation**: Output reads like a specification document

---

## Migration Path (If Needed)

For users coming from traditional testing:

```glang
# Old style (still works, but not idiomatic)
func test_something() {
    assert.equal(actual, expected)
}

# New style (idiomatic Graphoid)
describe "Feature" {
    it "does something" {
        expect(actual).to_equal(expected)
    }
}
```

Both can coexist initially, but RSpec-style is the **recommended and idiomatic approach** for Graphoid.

---

**Conclusion**: RSpec-style testing makes Graphoid's testing framework modern, professional, and a joy to use.

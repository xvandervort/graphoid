# gspec Module - Testing Framework

The `gspec` module provides an RSpec-style behavior-driven testing framework for Graphoid. Tests are written in a natural, readable style using `describe`, `context`, and `it` blocks.

## Running Tests

Spec files must end with `_spec.gr`. Run tests using:

```bash
gr spec                     # Run all specs in current directory
gr spec tests/              # Run all specs in tests/
gr spec path/to/file_spec.gr  # Run a specific spec file
```

## Writing Tests

### Basic Structure

```graphoid
describe "Calculator" {
    describe "add" {
        it "adds two positive numbers" {
            result = 2 + 3
            assert(expect(result).to_equal(5))
        }

        it "handles negative numbers" {
            assert(expect(-2 + -3).to_equal(-5))
        }
    }
}
```

### DSL Functions

#### `describe(name, block)`

Groups related tests together. Can be nested.

```graphoid
describe "User" {
    describe "validation" {
        it "requires a name" {
            # test here
        }
    }
}
```

#### `context(name, block)`

Alias for `describe`, typically used for conditional groupings. Automatically prefixes with "when ".

```graphoid
describe "Division" {
    context "dividing by zero" {
        it "raises an error" {
            # test here
        }
    }
}
# Outputs: "when dividing by zero"
```

#### `it(description, block)`

Defines a single test case.

```graphoid
it "returns the correct value" {
    assert(expect(calculate()).to_equal(42))
}
```

#### `xit(description, block)`

Skips a test (excluded it).

```graphoid
xit "not yet implemented" {
    # This test won't run
}
```

#### `pending(description)`

Marks a test as pending (placeholder).

```graphoid
pending("needs database connection")
```

### Hooks

#### `before_each(hook)`

Runs before each test in the current describe block.

```graphoid
describe "Database" {
    before_each {
        db = create_test_database()
    }

    it "inserts records" {
        db.insert("test")
        assert(expect(db.count()).to_equal(1))
    }
}
```

#### `after_each(hook)`

Runs after each test in the current describe block.

```graphoid
describe "File operations" {
    after_each {
        cleanup_temp_files()
    }

    it "creates a file" {
        create_file("test.txt")
        assert(expect(fs.exists("test.txt")).to_equal(true))
    }
}
```

## Expectations

### `expect(value)`

Creates an expectation wrapper for a value.

```graphoid
expect(5).to_equal(5)
expect(result).to_be_truthy()
```

### `assert(result)`

Asserts that an expectation passed. Raises an error if it failed.

```graphoid
assert(expect(x).to_equal(5))  # Raises error if x != 5
```

## Matchers

### Equality

#### `to_equal(expected)` / `to_be(expected)`

Checks for equality.

```graphoid
assert(expect(5).to_equal(5))
assert(expect("hello").to_be("hello"))
```

### Truthiness

#### `to_be_truthy()`

Checks that the value is truthy.

```graphoid
assert(expect(1).to_be_truthy())
assert(expect("hello").to_be_truthy())
assert(expect([1, 2]).to_be_truthy())
```

#### `to_be_falsy()`

Checks that the value is falsy.

```graphoid
assert(expect(0).to_be_falsy())
assert(expect("").to_be_falsy())
assert(expect([]).to_be_falsy())
```

#### `to_be_none()`

Checks that the value is `none`.

```graphoid
assert(expect(none).to_be_none())
```

### Type Checking

#### `to_be_a(type_name)`

Checks the type of a value.

```graphoid
assert(expect(5).to_be_a("num"))
assert(expect("hello").to_be_a("string"))
assert(expect([1, 2]).to_be_a("list"))
```

### Collections

#### `to_contain(element)`

Checks if a collection contains an element.

```graphoid
assert(expect([1, 2, 3]).to_contain(2))
assert(expect("hello").to_contain("ell"))
```

#### `to_be_empty()`

Checks if a collection is empty.

```graphoid
assert(expect([]).to_be_empty())
assert(expect("").to_be_empty())
```

#### `to_have_length(n)`

Checks the length of a collection.

```graphoid
assert(expect([1, 2, 3]).to_have_length(3))
assert(expect("hello").to_have_length(5))
```

### Comparisons

#### `to_be_greater_than(expected)`

```graphoid
assert(expect(10).to_be_greater_than(5))
```

#### `to_be_less_than(expected)`

```graphoid
assert(expect(5).to_be_less_than(10))
```

#### `to_be_at_least(expected)`

```graphoid
assert(expect(10).to_be_at_least(10))
assert(expect(11).to_be_at_least(10))
```

#### `to_be_at_most(expected)`

```graphoid
assert(expect(10).to_be_at_most(10))
assert(expect(9).to_be_at_most(10))
```

### Approximate Matching

#### `to_be_close_to(expected, tolerance)`

Checks if a number is within a tolerance of the expected value.

```graphoid
assert(expect(3.14159).to_be_close_to(3.14, 0.01))
```

#### `to_be_within(tolerance).of(expected)`

Alternative syntax for approximate matching.

```graphoid
assert(expect(3.14159).to_be_within(0.01).of(3.14))
```

### Negation

#### `to_not()` / `negate()`

Negates the expectation.

```graphoid
assert(expect(5).to_not().to_equal(10))
assert(expect([1, 2]).to_not().to_be_empty())
```

### Exception Testing

#### `to_raise(error_type)`

Checks that a function raises a specific error type.

```graphoid
fn divide_by_zero() {
    return 1 / 0
}

assert(expect(divide_by_zero).to_raise("RuntimeError"))
```

#### `to_not_raise()`

Checks that a function does not raise any error.

```graphoid
fn safe_function() {
    return 42
}

assert(expect(safe_function).to_not_raise())
```

## Complete Example

```graphoid
# calculator_spec.gr

describe "Calculator" {
    describe "basic operations" {
        it "adds numbers" {
            assert(expect(2 + 3).to_equal(5))
        }

        it "subtracts numbers" {
            assert(expect(10 - 4).to_equal(6))
        }

        it "multiplies numbers" {
            assert(expect(3 * 4).to_equal(12))
        }

        it "divides numbers" {
            assert(expect(10 / 2).to_equal(5))
        }
    }

    describe "floating point" {
        it "handles precision" {
            result = 0.1 + 0.2
            assert(expect(result).to_be_close_to(0.3, 0.0001))
        }
    }

    context "edge cases" {
        it "handles zero" {
            assert(expect(0 + 0).to_equal(0))
            assert(expect(5 * 0).to_equal(0))
        }

        it "handles negative numbers" {
            assert(expect(-5 + 3).to_equal(-2))
            assert(expect(-5 * -3).to_equal(15))
        }
    }
}
```

## Test Output

Running specs produces output like:

```
Running 1 spec file(s)...

Calculator
  basic operations
      PASS: adds numbers
      PASS: subtracts numbers
      PASS: multiplies numbers
      PASS: divides numbers
  floating point
      PASS: handles precision
  when edge cases
      PASS: handles zero
      PASS: handles negative numbers

============================================================
FINAL SUMMARY
============================================================
7 passed, 0 failed, 0 skipped, 7 total

ALL TESTS PASSED
```

When tests fail, the output includes the failure details:

```
Calculator
  basic operations
      FAIL: adds numbers
        Error: Expected 6 but got 5

============================================================
FINAL SUMMARY
============================================================
0 passed, 1 failed, 0 skipped, 1 total

Failed tests:
  Calculator > basic operations > adds numbers
    Error: Expected 6 but got 5

SOME TESTS FAILED
```

# Glang Test Framework Design Plan
**Phase 2 Priority: Clean, Non-Redundant Behavior-Oriented Testing**

## Vision: Clean Testing Framework Without Redundancy

Create a behavior-driven testing framework with minimal syntax that follows Glang's clean design principles.

## Core Design Principles

### 1. **Clean, Non-Redundant Syntax**
```glang
import "test"

describe("User Authentication", func() {
    context("when user provides valid credentials", func() {
        it("should authenticate successfully", func() {
            user = authenticate("alice", "password123")
            expect(user.authenticated).to_be(true)
            expect(user.name).to_equal("alice")
        })

        it("should set session token", func() {
            user = authenticate("alice", "password123")
            expect(user.token).to_not_be_nil()
        })
    })

    context("when user provides invalid credentials", func() {
        it("should reject authentication", func() {
            user = authenticate("alice", "wrongpassword")
            expect(user.authenticated).to_be(false)
        })
    })
})
```

### 2. **Let System with Clean Syntax**
```glang
import "test"

describe("Shopping Cart", func() {
    # Clean let syntax - no redundant prefixes
    let("user", func() {
        return build("user", {"name": "CartUser"})
    })

    let("cart", func() {
        return create_cart_for_user(user)
    })

    let("expensive_product", func() {
        return build("product", {
            "name": "Expensive Item",
            "price": 999.99
        })
    })

    describe("adding items", func() {
        it("should start empty", func() {
            expect(cart.item_count).to_equal(0)    # No redundant parens
            expect(cart.total).to_equal(0.0)
        })

        it("should add single item", func() {
            cart.add_item(expensive_product, 1)

            expect(cart.item_count).to_equal(1)
            expect(cart.total).to_equal(999.99)
            expect(cart.contains_product(expensive_product)).to_be(true)
        })
    })

    context("when cart has items", func() {
        before_each(func() {
            cart.add_item(expensive_product, 1)
        })

        it("should calculate correct total", func() {
            expect(cart.total).to_equal(999.99)
        })
    })
})
```

### 3. **Factory System Without Redundancy**
```glang
# factories/user_factory.gr
import "test"

factory("user", func(traits) {
    attributes = {
        "name": "DefaultUser",
        "email": "user@example.com",
        "role": "user",
        "active": true
    }

    # Apply traits
    if traits.has_key("admin") {
        attributes["role"] = "admin"
        attributes["name"] = "AdminUser"
    }

    if traits.has_key("inactive") {
        attributes["active"] = false
    }

    # Custom attributes override defaults
    for key in traits.keys() {
        if key != "admin" and key != "inactive" {
            attributes[key] = traits[key]
        }
    }

    return create_user_from_attributes(attributes)
})

# Usage in tests
describe("User Management", func() {
    let("regular_user", func() {
        return build("user")  # Default user
    })

    let("admin_user", func() {
        return build("user", {"admin": true})
    })

    let("custom_user", func() {
        return build("user", {
            "name": "CustomName",
            "email": "custom@example.com",
            "admin": true
        })
    })

    it("should create user with correct attributes", func() {
        expect(custom_user.name).to_equal("CustomName")
        expect(custom_user.is_admin).to_be(true)     # No redundant parens
    })
})
```

### 4. **Clean Expectation System**
```glang
# Core expectation syntax - following RSpec closely
expect(result).to_equal(42)
expect(list).to_include("item")
expect(value).to_be_greater_than(10)
expect(string).to_match_pattern("email")
expect(function_call).to_raise_error("ValidationError")

# Negative assertions
expect(result).to_not_equal(0)
expect(list).to_not_be_empty()

# Boolean checks - clean syntax
expect(user.is_admin).to_be(true)      # Not is_admin()
expect(cart.empty).to_be(false)        # Not empty()
expect(product.available).to_be(true)  # Not available()

# Collection matchers
expect(items).to_have_size(3)
expect(users).to_all_satisfy("active")
expect(names).to_contain_exactly(["Alice", "Bob"])

# Type and method matchers
expect(object).to_be_instance_of("user")
expect(object).to_respond_to("authenticate")
```

### 5. **Behavior Testing Example**
```glang
# test/test_behaviors.gr
import "test"
load "stdlib/behaviors.gr"

describe("Behavior Functions", func() {
    describe("positive", func() {
        it("should make negative numbers positive", func() {
            result = positive(-5)
            expect(result).to_equal(5)
        })

        it("should leave positive numbers unchanged", func() {
            result = positive(10)
            expect(result).to_equal(10)
        })

        it("should pass through non-numbers", func() {
            result = positive("hello")
            expect(result).to_equal("hello")
        })
    })

    describe("validate_range", func() {
        it("should clamp values above max", func() {
            result = validate_range(150, 0, 100)
            expect(result).to_equal(100)
        })

        it("should clamp values below min", func() {
            result = validate_range(-20, 0, 100)
            expect(result).to_equal(0)
        })

        it("should leave values within range unchanged", func() {
            result = validate_range(50, 0, 100)
            expect(result).to_equal(50)
        })
    })
})

run_all()  # Clean, simple
```

## Technical Implementation

### 1. **Module Import Strategy**
```glang
# Import brings functions into global scope
import "test"

# After import, these functions are available:
describe()
context()
it()
before_each()
after_each()
let()
expect()
build()
create()
factory()
sequence()
run_all()
```

### 2. **Let System Implementation**
```glang
# Internal let registry
let_values = {}
let_factories = {}

func let(name, factory_func) {
    context_id = current_test_context()
    let_factories[context_id + ":" + name] = factory_func
}

# Automatic resolution when variable is accessed
func resolve_let(name) {
    context_id = current_test_context()
    cache_key = context_id + ":" + name

    # Return cached if exists
    if let_values.has_key(cache_key) {
        return let_values[cache_key]
    }

    # Find and execute factory
    factory = let_factories[cache_key]
    if factory == nil {
        raise_error("Let variable '" + name + "' not defined")
    }

    value = factory.call()
    let_values[cache_key] = value
    return value
}
```

### 3. **Clean Expectation Implementation**
```glang
# Expectation object
expectation = {
    actual: value,
    negated: false,

    # Core matchers - clean method names
    to_equal: func(expected) {
        result = (actual == expected)
        if negated { result = not result }
        assert_result(result, "equal", expected)
    },

    to_be: func(expected) {
        result = (actual == expected)
        if negated { result = not result }
        assert_result(result, "be", expected)
    },

    to_include: func(item) {
        result = actual.contains(item)
        if negated { result = not result }
        assert_result(result, "include", item)
    },

    to_be_greater_than: func(threshold) {
        result = (actual > threshold)
        if negated { result = not result }
        assert_result(result, "be greater than", threshold)
    },

    # Negation creates new expectation with negated flag
    to_not: create_negated_expectation(actual)
}

func expect(actual_value) {
    return create_expectation(actual_value, false)
}
```

### 4. **Factory System**
```glang
factories = {}

func factory(name, builder_func) {
    factories[name] = builder_func
}

func build(factory_name, traits) {
    if not factories.has_key(factory_name) {
        raise_error("Factory '" + factory_name + "' not defined")
    }

    builder = factories[factory_name]
    return builder.call(traits || {})
}

func create(factory_name, traits) {
    object = build(factory_name, traits)
    return save_to_database(object)  # Persist if needed
}

func sequence(name, formatter) {
    # Generate unique sequential values
    counter = get_next_sequence_value(name)
    if formatter != nil {
        return formatter.call(counter)
    }
    return counter
}
```

## Complete Clean Example

```glang
# test/test_shopping_cart.gr
import "test"
load "stdlib/shopping_cart.gr"
load "test/factories.gr"

describe("Shopping Cart", func() {
    let("user", func() {
        return build("user", {"name": "CartUser"})
    })

    let("cart", func() {
        return create_cart_for_user(user)
    })

    let("expensive_product", func() {
        return build("product", {
            "name": "Expensive Item",
            "price": 999.99,
            "category": "luxury"
        })
    })

    let("cheap_product", func() {
        return build("product", {
            "name": "Cheap Item",
            "price": 9.99,
            "category": "basic"
        })
    })

    describe("adding items", func() {
        it("should start empty", func() {
            expect(cart.item_count).to_equal(0)
            expect(cart.total).to_equal(0.0)
        })

        it("should add single item", func() {
            cart.add_item(expensive_product, 1)

            expect(cart.item_count).to_equal(1)
            expect(cart.total).to_equal(999.99)
            expect(cart.contains_product(expensive_product)).to_be(true)
        })
    })

    context("when cart has items", func() {
        before_each(func() {
            cart.add_item(expensive_product, 1)
            cart.add_item(cheap_product, 3)
        })

        it("should calculate correct total", func() {
            expected_total = 999.99 + (9.99 * 3)
            expect(cart.total).to_equal(expected_total)
        })

        it("should remove items correctly", func() {
            cart.remove_item(cheap_product, 1)

            expect(cart.item_count).to_equal(3)  # 1 expensive + 2 remaining cheap
            expected_total = 999.99 + (9.99 * 2)
            expect(cart.total).to_equal(expected_total)
        })
    })

    context("with discount rules", func() {
        let("discount_rule", func() {
            return build("discount", {
                "type": "percentage",
                "value": 10,
                "minimum_amount": 500
            })
        })

        before_each(func() {
            cart.add_discount_rule(discount_rule)
            cart.add_item(expensive_product, 1)
        })

        it("should apply discount when minimum met", func() {
            expected_total = 999.99 * 0.9  # 10% discount
            expect(cart.total_with_discounts).to_equal(expected_total)
        })
    })
})

run_all()
```

## Key Improvements

### 1. **No Redundant Prefixes**
- ❌ `test.describe`, `test.it`, `test.expect`
- ✅ `describe`, `it`, `expect`

### 2. **Minimal Parentheses**
- ❌ `expect(cart.item_count()).to_equal(0)`
- ✅ `expect(cart.item_count).to_equal(0)`

### 3. **Clean RSpec-Style Syntax**
- ✅ `expect(result).to_equal(5)` (proper RSpec style)
- ✅ `expect(list).to_include(item)`
- ✅ `expect(value).to_not_be_nil()`

### 4. **Natural Glang Patterns**
- Clean function definitions without redundancy
- Property access instead of unnecessary method calls
- Minimal syntax that reads naturally

### 5. **Proper Test Isolation**
- `let` with lazy evaluation
- `before_each`/`after_each` hooks
- Factory system with traits
- Sequence generation

This design respects Glang's principles while providing all the power of RSpec-style testing!
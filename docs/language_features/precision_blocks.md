# Precision Context Blocks

Precision blocks are a revolutionary language-level feature in Glang that allows you to control numeric precision for calculations within a specific scope. Unlike most programming languages that treat precision as a global setting or require verbose library calls, Glang provides clean, intuitive syntax for precision control.

## Overview

Precision blocks use **decimal places precision** - meaning the number you specify is exactly how many digits will appear after the decimal point. This is different from "significant digits" used by some mathematical libraries.

```glang
precision N {
    # All numeric calculations use N decimal places
}
```

## Basic Syntax

### Simple Precision Block
```glang
precision 3 {
    num result = 22.0 / 7.0    # Result: 3.143 (exactly 3 decimal places)
    print(result.to_string())  # "3.143"
}
```

### Variable Precision
```glang
num decimal_places = 5
precision decimal_places {
    num pi = 3.14159265358979323846  # Result: 3.14159 (5 decimal places)
    print(pi.to_string())            # "3.14159"
}
```

## Precision Modes

### Integer Mode (precision 0)
When you specify `precision 0`, all calculations produce true integers with no decimal point:

```glang
precision 0 {
    num pi = 3.14159265358979323846  # Result: 3 (integer, no decimal)
    num area = pi * 10 * 10          # Result: 300 (integer arithmetic)
    print(pi.to_string())            # "3" (no decimal point)
    print(area.to_string())          # "300"
}
```

### Financial Mode (precision 2)
Perfect for monetary calculations requiring exactly 2 decimal places:

```glang
precision 2 {
    num price = 19.99
    num tax = price * 0.085          # Result: 1.70 (exactly 2 decimal places)
    num total = price + tax          # Result: 21.69 (exactly 2 decimal places)
    
    print("Price: $" + price.to_string())  # "Price: $19.99"
    print("Tax: $" + tax.to_string())      # "Tax: $1.7"  (trailing zeros removed by to_string)
    print("Total: $" + total.to_string())  # "Total: $21.69"
}
```

### Scientific Mode (precision 5+)
For scientific calculations requiring specific decimal precision:

```glang
precision 8 {
    num pi = 3.14159265358979323846
    num radius = 7.5
    num area = pi * radius * radius      # All calculations use 8 decimal places
    num circumference = 2 * pi * radius
    
    print("pi = " + pi.to_string())                    # "pi = 3.14159265"
    print("area = " + area.to_string())                # "area = 176.71448543"
    print("circumference = " + circumference.to_string())  # "circumference = 47.12388981"
}
```

## Advanced Features

### Nested Precision Contexts
Precision blocks can be nested, and the precision is automatically restored when exiting inner blocks:

```glang
precision 4 {
    num outer = 22.0 / 7.0           # Result: 3.1429 (4 decimal places)
    print("Outer: " + outer.to_string())  # "Outer: 3.1429"
    
    precision 1 {
        num inner = 22.0 / 7.0       # Result: 3.1 (1 decimal place)  
        print("Inner: " + inner.to_string())  # "Inner: 3.1"
        
        precision 0 {
            num integer = 22.0 / 7.0  # Result: 3 (integer mode)
            print("Integer: " + integer.to_string())  # "Integer: 3"
        }
        
        num back_to_inner = 22.0 / 7.0  # Result: 3.1 (1 decimal place restored)
        print("Back to inner: " + back_to_inner.to_string())  # "Back to inner: 3.1"
    }
    
    num back_to_outer = 22.0 / 7.0     # Result: 3.1429 (4 decimal places restored)
    print("Back to outer: " + back_to_outer.to_string())  # "Back to outer: 3.1429"
}
```

### Precision with Complex Expressions
All arithmetic operations within a precision block use the specified precision:

```glang
precision 3 {
    num a = 10.0 / 3.0        # 3.333
    num b = a * a             # 11.109 (3.333 * 3.333)
    num c = b.sqrt()          # 3.333 (√11.109)
    num d = c + a - b         # -7.776 (3.333 + 3.333 - 11.109)
}
```

## Memory Efficiency and Implementation

Glang's precision blocks are designed for both accuracy and memory efficiency:

- **Internal Precision**: Uses requested precision + 2 extra digits internally for accurate rounding
- **Memory Optimization**: Avoids excessive precision that would waste memory
- **Accurate Rounding**: The extra internal precision ensures proper rounding at the specified decimal places

```glang
# Example: requesting precision 3 uses ~5-6 digits internally
precision 3 {
    num precise = 1.0 / 7.0   # Calculated with ~5-6 digits, displayed as 3 decimal places
    # Result: 0.143 (exactly 3 decimal places)
}
```

## Validation and Error Handling

Precision blocks validate the precision value:

```glang
# Valid precision ranges: 0 to 1000
precision -1 {    # Error: Invalid precision value
    num x = 1.0
}

precision 1001 {  # Error: Precision too large  
    num y = 2.0
}
```

Error messages provide clear guidance:
```
Runtime error: Precision must be between 0 and 1000, got: -1
```

## Precision Retention

**Important**: Numbers calculated within precision blocks retain their precision permanently. The precision setting affects the calculation and storage of the number, not just its display.

```glang
precision 2 {
    num pi_short = 3.14159265358979323846  # Stored as 3.14 (2 decimal places)
}

# Outside the precision block
print(pi_short.to_string())  # Still "3.14" - the precision is permanently applied
```

## Practical Examples

### Configuration-Driven Precision
```glang
# Load precision settings from configuration
import "io"

config_content = io.read_file("math_config.json")
config = json.decode(config_content)
num math_precision = config["precision"].value()

precision math_precision {
    num result = expensive_calculation()
    io.write_file("output.txt", result.to_string())
}
```

### Multi-Precision Scientific Computing
```glang
# Different precision for different types of calculations

# Quick approximation with low precision
precision 2 {
    num rough_pi = 22.0 / 7.0        # 3.14 - fast calculation
    num rough_area = rough_pi * 5 * 5  # 78.5
}

# High precision for final results  
precision 10 {
    num precise_pi = 3.14159265358979323846    # 3.1415926536
    num precise_area = precise_pi * 5 * 5      # 78.5398163375
}

print("Rough approximation: " + rough_area.to_string())
print("Precise calculation: " + precise_area.to_string())
```

### Financial Calculations
```glang
# Ensure all monetary calculations use exactly 2 decimal places
precision 2 {
    num principal = 1000.0
    num interest_rate = 0.045  # 4.5% annual
    num years = 10
    
    # Compound interest calculation
    num amount = principal * (1 + interest_rate).pow(years)
    num interest_earned = amount - principal
    
    print("Principal: $" + principal.to_string())
    print("Final amount: $" + amount.to_string())
    print("Interest earned: $" + interest_earned.to_string())
}
```

### Performance Optimization
```glang
# Use lower precision for performance-critical loops
precision 1 {
    list results = []
    for i in range(100000) {
        num quick_calc = i.to_num() / 7.0  # Fast, 1 decimal place precision
        results.append(quick_calc)
    }
}

print("Processed " + results.size().to_string() + " items with optimized precision")
```

## Comparison with Other Approaches

### Traditional Approach (Verbose)
```python
# Python with Decimal (verbose)
from decimal import Decimal, getcontext
getcontext().prec = 5
result = Decimal('22') / Decimal('7')
```

### Glang Approach (Clean)
```glang
# Glang (concise and readable)
precision 3 {
    num result = 22.0 / 7.0
}
```

## Best Practices

1. **Choose Appropriate Precision**: Use the minimum precision needed for your application
   - Precision 0: Integer calculations, counters, indices
   - Precision 2: Financial calculations, prices, percentages  
   - Precision 3-5: General scientific calculations
   - Precision 6+: High-precision scientific or mathematical work

2. **Use Nested Blocks Judiciously**: Avoid excessive nesting that makes code hard to follow

3. **Document Precision Requirements**: Comment why specific precision levels are chosen

4. **Test Precision Behavior**: Verify that your precision settings produce expected results

5. **Consider Performance**: Lower precision can improve performance in calculation-heavy code

## Integration with Other Glang Features

### With Functions
```glang
func calculate_circle_properties(radius, precision_level) {
    precision precision_level {
        num area = 3.14159265358979323846 * radius * radius
        num circumference = 2 * 3.14159265358979323846 * radius
        return { "area": area, "circumference": circumference }
    }
}

properties = calculate_circle_properties(5.0, 4)
```

### With Control Flow
```glang
for precision_level in [1, 2, 3, 4, 5] {
    precision precision_level {
        num pi_approx = 22.0 / 7.0
        print("Precision " + precision_level.to_string() + ": " + pi_approx.to_string())
    }
}
```

### With Data Structures
```glang
precision 3 {
    list calculations = []
    for value in input_data {
        num processed = complex_formula(value)
        calculations.append(processed)
    }
    
    hash results = {
        "data": calculations,
        "precision": "3 decimal places",
        "timestamp": current_time()
    }
}
```

---

## Summary

Precision context blocks represent a fundamental innovation in programming language design, providing:

- **Intuitive decimal places semantics** instead of confusing significant digits
- **Language-level integration** rather than library-based solutions  
- **Automatic precision management** with memory efficiency
- **Nested context support** with automatic restoration
- **Clean, readable syntax** that expresses intent clearly

This feature makes Glang uniquely powerful for applications requiring precise numeric control, from financial systems to scientific computing.
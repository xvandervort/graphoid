#!/usr/bin/env python3
"""Demo of the behavior system for Glang containers.

This shows how behaviors can be composed to transform and validate data
in lists and hashes, providing a flexible way to handle custom node types.
"""

from glang.behaviors import BehaviorRegistry, BehaviorPipeline, create_behavior
from glang.execution.values import NumberValue, StringValue, NoneValue, ListValue, HashValue


def demo_basic_behaviors():
    """Demonstrate individual behaviors."""
    print("=== Basic Behaviors ===\n")
    
    registry = BehaviorRegistry()
    
    # Nil to zero
    nil_to_zero = registry.get("nil_to_zero")
    result = nil_to_zero.apply(NoneValue())
    print(f"nil -> {result.value}")  # 0
    
    # Validate range
    validate_range = registry.get("validate_range")
    result = validate_range.apply(NumberValue(150), 0, 100)
    print(f"150 clamped to [0,100] -> {result.value}")  # 100
    
    # Map colors
    map_colors = registry.get("map_colors")
    result = map_colors.apply(StringValue("green"))
    print(f"'green' -> {result.value}")  # 2
    print()


def demo_pipeline():
    """Demonstrate behavior pipelines."""
    print("=== Behavior Pipeline ===\n")
    
    # Create a pipeline for sensor data validation
    sensor_pipeline = BehaviorPipeline()
    sensor_pipeline.add("nil_to_zero")
    sensor_pipeline.add("validate_range", -50, 150)  # Temperature range
    sensor_pipeline.add("round_to_int")
    
    # Apply to various inputs
    inputs = [
        NumberValue(23.7),    # Normal reading
        NoneValue(),          # Missing sensor
        NumberValue(200),     # Out of range
        NumberValue(-100),    # Below range
    ]
    
    print("Temperature sensor readings:")
    for input_val in inputs:
        display = input_val.to_display_string() if not isinstance(input_val, NoneValue) else "nil"
        result = sensor_pipeline.apply(input_val)
        print(f"  {display:>6} -> {result.value}")
    print()


def demo_medical_data():
    """Demonstrate medical data validation."""
    print("=== Medical Data Validation ===\n")
    
    # Create pipeline for blood pressure readings
    bp_pipeline = BehaviorPipeline()
    bp_pipeline.add("nil_to_zero")
    bp_pipeline.add("validate_range", 60, 200)  # Reasonable BP range
    
    # Patient readings over time
    readings = ListValue([
        NumberValue(120),    # Normal systolic
        NumberValue(85),     # Normal diastolic
        NoneValue(),         # Missing reading
        NumberValue(250),    # Dangerously high
        NumberValue(45),     # Too low
    ])
    
    print("Blood pressure readings (systolic/diastolic):")
    result = bp_pipeline.apply_to_list(readings)
    for i, elem in enumerate(result.elements):
        original = readings.elements[i]
        orig_str = original.to_display_string() if not isinstance(original, NoneValue) else "nil"
        print(f"  Reading {i+1}: {orig_str:>6} -> {elem.value}")
    print()


def demo_custom_behavior():
    """Demonstrate creating custom behaviors."""
    print("=== Custom Behaviors ===\n")
    
    # Create a custom behavior for parsing blood pressure strings
    def parse_bp(value):
        if isinstance(value, StringValue):
            # Parse "120/80" format
            if "/" in value.value:
                parts = value.value.split("/")
                systolic = float(parts[0])
                diastolic = float(parts[1])
                # Return average for simplicity (in real code, return both)
                return NumberValue((systolic + diastolic) / 2)
        return value
    
    bp_parser = create_behavior("parse_bp", transform=parse_bp)
    
    # Create registry with custom behavior
    custom_registry = BehaviorRegistry()
    custom_registry.register("parse_bp", bp_parser)
    
    # Use in pipeline
    pipeline = BehaviorPipeline(custom_registry)
    pipeline.add("parse_bp")
    pipeline.add("round_to_int")
    
    # Test with BP string
    bp_string = StringValue("120/80")
    result = pipeline.apply(bp_string)
    print(f"Blood pressure '{bp_string.value}' -> average: {result.value}")
    print()


def demo_config_normalization():
    """Demonstrate configuration value normalization."""
    print("=== Configuration Normalization ===\n")
    
    # Create pipeline for server configuration
    config_pipeline = BehaviorPipeline()
    config_pipeline.add("nil_to_zero")
    config_pipeline.add("validate_range", 1024, 65535)  # Valid port range
    
    # Server configuration
    config = HashValue([
        ("port", NumberValue(8080)),
        ("admin_port", NumberValue(80)),      # Too low, will be adjusted
        ("debug_port", NoneValue()),          # Not configured
        ("max_port", NumberValue(70000)),     # Too high
    ])
    
    print("Server port configuration:")
    for key in ["port", "admin_port", "debug_port", "max_port"]:
        original = config.pairs.get(key)
        orig_str = original.to_display_string() if original else "nil"
        config_pipeline.apply_to_hash_value(config, key)
        new_val = config.pairs.get(key)
        print(f"  {key:12} {orig_str:>6} -> {new_val.value}")
    print()


if __name__ == "__main__":
    demo_basic_behaviors()
    demo_pipeline()
    demo_medical_data()
    demo_custom_behavior()
    demo_config_normalization()
    
    print("=== Summary ===")
    print("Behaviors provide a flexible way to:")
    print("- Transform values (nil -> 0, colors -> numbers)")
    print("- Validate and constrain (range clamping)")
    print("- Compose multiple transformations (pipelines)")
    print("- Create domain-specific logic (medical, config)")
    print("\nThis system allows Glang to handle custom node types")
    print("without adding language complexity!")
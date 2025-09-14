"""Tests for the behavior system."""

import pytest
from glang.behaviors import Behavior, BehaviorRegistry, BehaviorPipeline, create_behavior
from glang.execution.values import (
    NumberValue, StringValue, BooleanValue, NoneValue, ListValue, HashValue, DataValue
)


class TestBasicBehaviors:
    """Test individual behaviors."""
    
    def test_nil_to_zero(self):
        """Test nil to zero conversion."""
        registry = BehaviorRegistry()
        behavior = registry.get("nil_to_zero")
        
        # Nil becomes 0
        result = behavior.apply(NoneValue())
        assert isinstance(result, NumberValue)
        assert result.value == 0
        
        # Numbers stay unchanged
        num = NumberValue(42)
        result = behavior.apply(num)
        assert result.value == 42
        
        # Strings stay unchanged
        string = StringValue("hello")
        result = behavior.apply(string)
        assert result.value == "hello"
    
    def test_validate_range(self):
        """Test range validation and clamping."""
        registry = BehaviorRegistry()
        behavior = registry.get("validate_range")
        
        # Within range - unchanged
        result = behavior.apply(NumberValue(50), 0, 100)
        assert result.value == 50
        
        # Below range - clamped to min
        result = behavior.apply(NumberValue(-10), 0, 100)
        assert result.value == 0
        
        # Above range - clamped to max
        result = behavior.apply(NumberValue(150), 0, 100)
        assert result.value == 100
        
        # Non-numbers pass through
        result = behavior.apply(StringValue("test"), 0, 100)
        assert result.value == "test"
    
    def test_map_colors(self):
        """Test color name to number mapping."""
        registry = BehaviorRegistry()
        behavior = registry.get("map_colors")
        
        # Map known colors
        assert behavior.apply(StringValue("red")).value == 1
        assert behavior.apply(StringValue("green")).value == 2
        assert behavior.apply(StringValue("blue")).value == 3
        assert behavior.apply(StringValue("GREEN")).value == 2  # Case insensitive
        
        # Unknown colors stay as strings
        result = behavior.apply(StringValue("purple"))
        assert isinstance(result, StringValue)
        assert result.value == "purple"
        
        # Non-strings pass through
        result = behavior.apply(NumberValue(42))
        assert result.value == 42
    
    def test_uppercase_lowercase(self):
        """Test string case transformations."""
        registry = BehaviorRegistry()
        
        upper = registry.get("uppercase")
        result = upper.apply(StringValue("hello"))
        assert result.value == "HELLO"
        
        lower = registry.get("lowercase") 
        result = lower.apply(StringValue("WORLD"))
        assert result.value == "world"
        
        # Non-strings pass through
        result = upper.apply(NumberValue(42))
        assert result.value == 42


class TestBehaviorPipeline:
    """Test behavior composition."""
    
    def test_pipeline_composition(self):
        """Test applying multiple behaviors in sequence."""
        pipeline = BehaviorPipeline()
        pipeline.add("nil_to_zero")
        pipeline.add("validate_range", 0, 100)
        pipeline.add("round_to_int")
        
        # Nil -> 0 -> within range -> rounded
        result = pipeline.apply(NoneValue())
        assert result.value == 0
        
        # 150 -> clamped to 100 -> rounded
        result = pipeline.apply(NumberValue(150.7))
        assert result.value == 100
        
        # 42.7 -> within range -> rounded to 43
        result = pipeline.apply(NumberValue(42.7))
        assert result.value == 43
    
    def test_pipeline_on_list(self):
        """Test applying pipeline to all list elements."""
        pipeline = BehaviorPipeline()
        pipeline.add("nil_to_zero")
        pipeline.add("validate_range", 0, 100)
        
        # Create list with mixed values
        elements = [
            NumberValue(50),
            NoneValue(),
            NumberValue(150),
            NumberValue(-10)
        ]
        lst = ListValue(elements)
        
        # Apply pipeline to all elements
        result = pipeline.apply_to_list(lst)
        
        assert result.elements[0].value == 50   # unchanged
        assert result.elements[1].value == 0     # nil -> 0
        assert result.elements[2].value == 100   # 150 -> 100
        assert result.elements[3].value == 0     # -10 -> 0
    
    def test_pipeline_on_hash(self):
        """Test applying pipeline to specific hash values."""
        pipeline = BehaviorPipeline()
        pipeline.add("nil_to_zero")
        pipeline.add("validate_range", 1024, 65535)
        
        # Create hash - HashValue expects list of tuples
        pairs = [
            ("port", NumberValue(8080)),
            ("timeout", NoneValue()),
            ("max_port", NumberValue(70000))
        ]
        hash_val = HashValue(pairs)
        
        # Apply to specific keys
        pipeline.apply_to_hash_value(hash_val, "port")      # 8080 stays
        pipeline.apply_to_hash_value(hash_val, "timeout")   # nil -> 0 -> 1024
        pipeline.apply_to_hash_value(hash_val, "max_port")  # 70000 -> 65535
        
        assert hash_val.pairs.get("port").value == 8080
        assert hash_val.pairs.get("timeout").value == 1024
        assert hash_val.pairs.get("max_port").value == 65535


class TestCustomBehaviors:
    """Test creating custom behaviors."""
    
    def test_custom_transform(self):
        """Test creating a custom transformation behavior."""
        # Create behavior that doubles numbers
        double = create_behavior(
            "double",
            transform=lambda value: NumberValue(value.value * 2) if isinstance(value, NumberValue) else value
        )
        
        result = double.apply(NumberValue(21))
        assert result.value == 42
        
        result = double.apply(StringValue("test"))
        assert result.value == "test"
    
    def test_custom_validation(self):
        """Test creating a custom validation behavior."""
        # Create behavior that only allows even numbers
        def validate_even(value):
            if not isinstance(value, NumberValue):
                return True
            return value.value % 2 == 0
        
        def make_even(value):
            if isinstance(value, NumberValue) and value.value % 2 != 0:
                return NumberValue(value.value + 1)
            return value
        
        even_only = create_behavior(
            "even_only",
            validate=validate_even,
            on_invalid=make_even
        )
        
        # Even numbers pass through
        result = even_only.apply(NumberValue(42))
        assert result.value == 42
        
        # Odd numbers get incremented
        result = even_only.apply(NumberValue(41))
        assert result.value == 42
    
    def test_registering_custom(self):
        """Test registering and using custom behaviors."""
        registry = BehaviorRegistry()
        
        # Create and register custom behavior
        prefix = create_behavior(
            "add_prefix",
            transform=lambda value: StringValue("PREFIX_" + value.value) if isinstance(value, StringValue) else value
        )
        registry.register("add_prefix", prefix)
        
        # Use it in a pipeline with the custom registry
        pipeline = BehaviorPipeline(registry)
        pipeline.add("add_prefix")
        pipeline.add("uppercase")
        
        result = pipeline.apply(StringValue("test"))
        assert result.value == "PREFIX_TEST"


class TestPracticalExamples:
    """Test real-world use cases."""
    
    def test_medical_data_validation(self):
        """Test medical data with multiple validations."""
        # Create pipeline for temperature readings
        temp_pipeline = BehaviorPipeline()
        temp_pipeline.add("nil_to_zero")  # Missing readings become 0
        temp_pipeline.add("validate_range", 95.0, 105.0)  # Normal body temp range
        temp_pipeline.add("round_to_int")  # Round for display
        
        readings = ListValue([
            NumberValue(98.6),   # Normal
            NumberValue(104.5),  # High fever
            NoneValue(),       # Missing reading
            NumberValue(110.0),  # Impossible reading
            NumberValue(93.2)    # Hypothermia
        ])
        
        result = temp_pipeline.apply_to_list(readings)
        
        assert result.elements[0].value == 99   # 98.6 -> 99
        assert result.elements[1].value == 104  # 104.5 -> 104 (banker's rounding)
        assert result.elements[2].value == 95   # nil -> 0 -> clamped to 95 -> 95
        assert result.elements[3].value == 105  # 110 -> clamped to 105 -> 105
        assert result.elements[4].value == 95   # 93.2 -> clamped to 95 -> 95
    
    def test_config_normalization(self):
        """Test configuration value normalization."""
        # Create pipeline for config values
        config_pipeline = BehaviorPipeline()
        config_pipeline.add("lowercase")  # Normalize to lowercase
        config_pipeline.add("nil_to_empty")  # Nil becomes empty string
        
        # Mock config hash
        config_values = ListValue([
            StringValue("DEBUG"),
            StringValue("Production"),
            NoneValue(),
            StringValue("STAGING")
        ])
        
        result = config_pipeline.apply_to_list(config_values)
        
        assert result.elements[0].value == "debug"
        assert result.elements[1].value == "production"
        assert result.elements[2].value == ""
        assert result.elements[3].value == "staging"
"""
Test configurable none conversion behaviors.

This module tests the new configurable none conversion system that allows
none values to be gracefully converted to other types based on user
configuration rather than throwing disruptive errors.
"""

import sys
import os

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from glang.execution.values import NoneValue, StringValue, NumberValue, BooleanValue
from glang.execution.configuration_context import (
    get_current_config, set_global_none_conversions,
    ConfigurationContext, push_config, pop_config
)


class TestNoneConversions:
    """Test none conversion behaviors with different configurations."""

    def setup_method(self):
        """Reset configuration before each test."""
        from glang.execution.configuration_context import get_current_config
        config = get_current_config()
        config.set_none_conversion('to_string', 'empty_string')
        config.set_none_conversion('to_number', 'zero')
        config.set_none_conversion('to_bool', 'false')

    def test_default_none_to_string(self):
        """Test default none to string conversion."""
        none_val = NoneValue()
        result = none_val.to_string()

        print(f"none.to_string() result: '{result.value}' (type: {type(result)})")
        assert isinstance(result, StringValue)
        assert result.value == ""  # Default: empty string

    def test_default_none_to_number(self):
        """Test default none to number conversion."""
        none_val = NoneValue()
        result = none_val.to_number()

        print(f"none.to_number() result: {result.value} (type: {type(result)})")
        assert isinstance(result, NumberValue)
        assert result.value == 0  # Default: zero

    def test_default_none_to_bool(self):
        """Test default none to boolean conversion."""
        none_val = NoneValue()
        result = none_val.to_bool()

        print(f"none.to_bool() result: {result.value} (type: {type(result)})")
        assert isinstance(result, BooleanValue)
        assert result.value == False  # Default: false

    def test_none_to_num_alias(self):
        """Test that to_num() is an alias for to_number()."""
        none_val = NoneValue()
        result1 = none_val.to_number()
        result2 = none_val.to_num()

        print(f"to_number(): {result1.value}, to_num(): {result2.value}")
        assert result1.value == result2.value
        assert type(result1) == type(result2)


class TestConfigurableConversions:
    """Test different configuration scenarios for none conversions."""

    def setup_method(self):
        """Reset configuration before each test."""
        # We can't easily reset the global config, so we'll test with local context
        pass

    def test_none_literal_string_conversion(self):
        """Test none to string conversion with 'none_literal' behavior."""
        # Create a temporary configuration context
        config = ConfigurationContext()
        config.set_none_conversion('to_string', 'none_literal')

        # We would need dependency injection to fully test this
        # For now, test that the configuration system works
        assert config.get_none_conversion('to_string') == 'none_literal'

    def test_error_conversion_config(self):
        """Test that error conversion can be configured."""
        config = ConfigurationContext()
        config.set_none_conversion('to_number', 'error')

        assert config.get_none_conversion('to_number') == 'error'

    def test_true_boolean_conversion(self):
        """Test none to boolean conversion with 'true' behavior."""
        config = ConfigurationContext()
        config.set_none_conversion('to_bool', 'true')

        assert config.get_none_conversion('to_bool') == 'true'


class TestPracticalUsage:
    """Test practical usage scenarios for none conversions."""

    def setup_method(self):
        """Reset configuration before each test."""
        from glang.execution.configuration_context import get_current_config
        config = get_current_config()
        config.set_none_conversion('to_string', 'empty_string')
        config.set_none_conversion('to_number', 'zero')
        config.set_none_conversion('to_bool', 'false')

    def test_data_processing_with_none(self):
        """Test data processing that includes none values."""
        data_points = [
            NumberValue(42),
            NoneValue(),
            NumberValue(17),
            NoneValue(),
            NumberValue(23)
        ]

        # Process data gracefully
        processed = []
        for value in data_points:
            if isinstance(value, NoneValue):
                # Convert none to number for calculations
                processed.append(value.to_number())
            else:
                processed.append(value)

        # Verify all values are now numbers
        assert all(isinstance(v, NumberValue) for v in processed)

        # Verify converted values
        assert processed[0].value == 42
        assert processed[1].value == 0  # none became 0
        assert processed[2].value == 17
        assert processed[3].value == 0  # none became 0
        assert processed[4].value == 23

    def test_string_processing_with_none(self):
        """Test string processing that includes none values."""
        names = [
            StringValue("Alice"),
            NoneValue(),
            StringValue("Bob"),
            NoneValue(),
            StringValue("Charlie")
        ]

        # Process names gracefully
        greetings = []
        for name in names:
            if isinstance(name, NoneValue):
                # Convert none to string gracefully
                name_str = name.to_string()
                greeting = f"Hello, {name_str.value}(unknown)!"
            else:
                greeting = f"Hello, {name.value}!"
            greetings.append(greeting)

        # Verify all greetings were created without errors
        assert len(greetings) == 5
        assert greetings[0] == "Hello, Alice!"
        assert greetings[1] == "Hello, (unknown)!"  # none became empty string
        assert greetings[2] == "Hello, Bob!"
        assert greetings[3] == "Hello, (unknown)!"  # none became empty string
        assert greetings[4] == "Hello, Charlie!"


class TestConfigurationSystem:
    """Test the configuration system for none conversions."""

    def test_get_none_conversion_default(self):
        """Test getting none conversion with default values."""
        config = ConfigurationContext()

        # Test default values
        assert config.get_none_conversion('to_string') == 'empty_string'
        assert config.get_none_conversion('to_number') == 'zero'
        assert config.get_none_conversion('to_bool') == 'false'

    def test_set_none_conversion(self):
        """Test setting none conversion behaviors."""
        config = ConfigurationContext()

        # Set custom behaviors
        config.set_none_conversion('to_string', 'none_literal')
        config.set_none_conversion('to_number', 'error')
        config.set_none_conversion('to_bool', 'true')

        # Verify they were set
        assert config.get_none_conversion('to_string') == 'none_literal'
        assert config.get_none_conversion('to_number') == 'error'
        assert config.get_none_conversion('to_bool') == 'true'

    def test_unknown_conversion_default(self):
        """Test behavior with unknown conversion types."""
        config = ConfigurationContext()

        # Unknown conversion types should return 'error'
        assert config.get_none_conversion('to_unknown') == 'error'


def run_tests():
    """Run all tests manually without pytest."""
    print("=" * 60)
    print("RUNNING NONE CONVERSION TESTS")
    print("=" * 60)

    # Test basic conversions
    test_conversions = TestNoneConversions()
    print("\n1. Testing Basic None Conversions...")
    try:
        test_conversions.test_default_none_to_string()
        print("✅ none.to_string() works correctly")
    except Exception as e:
        print(f"❌ none.to_string() failed: {e}")

    try:
        test_conversions.test_default_none_to_number()
        print("✅ none.to_number() works correctly")
    except Exception as e:
        print(f"❌ none.to_number() failed: {e}")

    try:
        test_conversions.test_default_none_to_bool()
        print("✅ none.to_bool() works correctly")
    except Exception as e:
        print(f"❌ none.to_bool() failed: {e}")

    try:
        test_conversions.test_none_to_num_alias()
        print("✅ to_num() alias works correctly")
    except Exception as e:
        print(f"❌ to_num() alias failed: {e}")

    # Test configuration system
    test_config = TestConfigurationSystem()
    print("\n2. Testing Configuration System...")
    try:
        test_config.test_get_none_conversion_default()
        print("✅ Default configuration values correct")
    except Exception as e:
        print(f"❌ Default configuration failed: {e}")

    try:
        test_config.test_set_none_conversion()
        print("✅ Setting custom conversions works")
    except Exception as e:
        print(f"❌ Setting custom conversions failed: {e}")

    try:
        test_config.test_unknown_conversion_default()
        print("✅ Unknown conversion handling works")
    except Exception as e:
        print(f"❌ Unknown conversion handling failed: {e}")

    # Test practical usage
    test_practical = TestPracticalUsage()
    print("\n3. Testing Practical Usage Scenarios...")
    try:
        test_practical.test_data_processing_with_none()
        print("✅ Data processing with none values works")
    except Exception as e:
        print(f"❌ Data processing failed: {e}")

    try:
        test_practical.test_string_processing_with_none()
        print("✅ String processing with none values works")
    except Exception as e:
        print(f"❌ String processing failed: {e}")

    print("\n" + "=" * 60)
    print("NONE CONVERSION TESTS COMPLETED")
    print("=" * 60)

if __name__ == "__main__":
    run_tests()
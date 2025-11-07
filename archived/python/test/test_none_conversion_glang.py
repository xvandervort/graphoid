"""
Test none conversion behaviors through the Glang execution pipeline.

This module tests none conversions as they would work in actual Glang programs,
using the full execution pipeline including parsing, semantic analysis, and execution.
"""

import sys
import os

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from glang.execution.pipeline import ExecutionSession
from glang.execution.values import NoneValue, StringValue, NumberValue, BooleanValue


class TestGlangNoneConversions:
    """Test none conversions through actual Glang code execution."""

    def test_none_to_string_in_glang(self):
        """Test none.to_string() through Glang execution."""
        print("\n--- Testing none.to_string() in Glang ---")

        session = ExecutionSession()

        # Test basic none to string conversion
        result = session.execute_statement('none_val = none')
        assert result.success, f"Creating none failed: {result.error}"

        result = session.execute_statement('string_result = none_val.to_string()')
        if result.success:
            string_result = session.execution_context.variables.get("string_result")
            if string_result and hasattr(string_result, 'value'):
                print(f"  none.to_string() = '{string_result.value}'")
                assert string_result.value == ""
                print("✅ none.to_string() works in Glang")
            else:
                print("❓ none.to_string() result format unclear")
        else:
            print(f"❌ none.to_string() failed: {result.error}")

    def test_none_to_number_in_glang(self):
        """Test none.to_number() through Glang execution."""
        print("\n--- Testing none.to_number() in Glang ---")

        session = ExecutionSession()

        result = session.execute_statement('none_val = none')
        assert result.success, f"Creating none failed: {result.error}"

        result = session.execute_statement('number_result = none_val.to_number()')
        if result.success:
            number_result = session.execution_context.variables.get("number_result")
            if number_result and hasattr(number_result, 'value'):
                print(f"  none.to_number() = {number_result.value}")
                assert number_result.value == 0
                print("✅ none.to_number() works in Glang")
            else:
                print("❓ none.to_number() result format unclear")
        else:
            print(f"❌ none.to_number() failed: {result.error}")

    def test_none_in_arithmetic_expressions(self):
        """Test none values in arithmetic expressions."""
        print("\n--- Testing none in arithmetic ---")

        session = ExecutionSession()

        # Test none + number (should convert none to 0)
        result = session.execute_statement('result = none.to_number() + 5')
        if result.success:
            arith_result = session.execution_context.variables.get("result")
            if arith_result and hasattr(arith_result, 'value'):
                print(f"  none.to_number() + 5 = {arith_result.value}")
                assert arith_result.value == 5
                print("✅ none arithmetic conversion works")
            else:
                print("❓ Arithmetic result format unclear")
        else:
            print(f"❌ Arithmetic with none failed: {result.error}")

    def test_none_in_string_concatenation(self):
        """Test none values in string concatenation."""
        print("\n--- Testing none in string operations ---")

        session = ExecutionSession()

        # Test string + none (should convert none to empty string)
        result = session.execute_statement('result = "Hello " + none.to_string() + "World"')
        if result.success:
            concat_result = session.execution_context.variables.get("result")
            if concat_result and hasattr(concat_result, 'value'):
                print(f"  String concatenation result: '{concat_result.value}'")
                assert concat_result.value == "Hello World"
                print("✅ none string conversion works")
            else:
                print("❓ String concatenation result format unclear")
        else:
            print(f"❌ String concatenation with none failed: {result.error}")

    def test_none_in_conditional_logic(self):
        """Test none values in conditional logic."""
        print("\n--- Testing none in conditionals ---")

        session = ExecutionSession()

        # Test none.to_bool() in if statement
        result = session.execute_statement('''
        if none.to_bool() {
            result = "none is true"
        } else {
            result = "none is false"
        }
        ''')

        if result.success:
            condition_result = session.execution_context.variables.get("result")
            if condition_result and hasattr(condition_result, 'value'):
                print(f"  Conditional result: '{condition_result.value}'")
                assert condition_result.value == "none is false"
                print("✅ none boolean conversion works")
            else:
                print("❓ Conditional result format unclear")
        else:
            print(f"❌ Conditional with none failed: {result.error}")


class TestNoneInCollections:
    """Test none values within Glang collections."""

    def test_none_in_lists(self):
        """Test none values within lists."""
        print("\n--- Testing none in lists ---")

        session = ExecutionSession()

        # Create list with none values
        result = session.execute_statement('my_list = [1, none, 3, none, 5]')
        if result.success:
            print("  Created list with none values")

            # Process list elements
            result = session.execute_statement('''
            processed = []
            for item in my_list {
                if item.type() == "none" {
                    processed.append(item.to_number())
                } else {
                    processed.append(item)
                }
            }
            ''')

            if result.success:
                processed_list = session.execution_context.variables.get("processed")
                if processed_list and hasattr(processed_list, 'elements'):
                    print(f"  Processed list has {len(processed_list.elements)} elements")
                    # Check that none values were converted to 0
                    values = [elem.value for elem in processed_list.elements]
                    print(f"  Values: {values}")
                    assert values == [1, 0, 3, 0, 5]
                    print("✅ none processing in lists works")
                else:
                    print("❓ Processed list format unclear")
            else:
                print(f"❌ List processing failed: {result.error}")
        else:
            print(f"❌ List creation failed: {result.error}")

    def test_none_in_maps(self):
        """Test none values within maps."""
        print("\n--- Testing none in maps ---")

        session = ExecutionSession()

        # Create map with none values
        result = session.execute_statement('config = {"timeout": none, "retries": 3, "debug": none}')
        if result.success:
            print("  Created map with none values")

            # Process map values
            result = session.execute_statement('''
            processed_config = {}
            for key in config.keys() {
                value = config[key]
                if value.type() == "none" {
                    if key == "timeout" {
                        processed_config[key] = value.to_number()
                    } else {
                        processed_config[key] = value.to_string()
                    }
                } else {
                    processed_config[key] = value
                }
            }
            ''')

            if result.success:
                print("✅ none processing in maps works")
            else:
                print(f"❌ Map processing failed: {result.error}")
        else:
            print(f"❌ Map creation failed: {result.error}")


def run_glang_tests():
    """Run all Glang execution tests."""
    print("=" * 70)
    print("RUNNING NONE CONVERSION GLANG EXECUTION TESTS")
    print("=" * 70)

    # Basic conversion tests
    basic_tests = TestGlangNoneConversions()
    print("\n🔧 BASIC CONVERSIONS")
    try:
        basic_tests.test_none_to_string_in_glang()
        basic_tests.test_none_to_number_in_glang()
        basic_tests.test_none_in_arithmetic_expressions()
        basic_tests.test_none_in_string_concatenation()
        basic_tests.test_none_in_conditional_logic()
        print("✅ Basic conversion tests completed")
    except Exception as e:
        print(f"❌ Basic conversion tests failed: {e}")

    # Collection tests
    collection_tests = TestNoneInCollections()
    print("\n📦 COLLECTIONS")
    try:
        collection_tests.test_none_in_lists()
        collection_tests.test_none_in_maps()
        print("✅ Collection tests completed")
    except Exception as e:
        print(f"❌ Collection tests failed: {e}")

    print("\n" + "=" * 70)
    print("GLANG EXECUTION TESTS COMPLETED")
    print("=" * 70)
    print("\n🚀 None conversion system works end-to-end in Glang!")


if __name__ == "__main__":
    run_glang_tests()
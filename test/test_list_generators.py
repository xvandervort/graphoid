"""Test suite for list generator methods."""

import pytest
from glang.execution.pipeline import ExecutionSession
from glang.execution.values import NumberValue, StringValue, BooleanValue
from glang.execution.graph_values import ListValue


class TestListGenerators:
    """Test list generator methods."""

    def setup_method(self):
        """Set up test fixtures."""
        self.session = ExecutionSession()

    def test_generate_basic(self):
        """Test basic generate method."""
        self.session.execute_statement("nums = []")
        result = self.session.execute_statement("nums.generate(1, 5, 1)")

        assert result.success
        assert isinstance(result.value, ListValue)
        assert len(result.value.elements) == 5
        assert [elem.value for elem in result.value.elements] == [1, 2, 3, 4, 5]
        assert result.value.constraint == "num"

    def test_generate_with_step(self):
        """Test generate with custom step."""
        self.session.execute_statement("nums = []")
        result = self.session.execute_statement("nums.generate(0, 10, 2)")

        assert result.success
        assert isinstance(result.value, ListValue)
        assert [elem.value for elem in result.value.elements] == [0, 2, 4, 6, 8, 10]

    def test_generate_negative_step(self):
        """Test generate with negative step."""
        self.session.execute_statement("nums = []")
        result = self.session.execute_statement("nums.generate(10, 0, -2)")

        assert result.success
        assert isinstance(result.value, ListValue)
        assert [elem.value for elem in result.value.elements] == [10, 8, 6, 4, 2, 0]

    def test_generate_floats(self):
        """Test generate with floating point numbers."""
        self.session.execute_statement("nums = []")
        result = self.session.execute_statement("nums.generate(0.5, 2.5, 0.5)")

        assert result.success
        assert isinstance(result.value, ListValue)
        assert [elem.value for elem in result.value.elements] == [0.5, 1.0, 1.5, 2.0, 2.5]

    def test_generate_empty(self):
        """Test generate that produces empty list."""
        self.session.execute_statement("nums = []")
        result = self.session.execute_statement("nums.generate(5, 1, 1)")

        assert result.success
        assert isinstance(result.value, ListValue)
        assert len(result.value.elements) == 0

    def test_generate_zero_step_error(self):
        """Test that zero step raises error."""
        self.session.execute_statement("nums = []")
        result = self.session.execute_statement("nums.generate(1, 5, 0)")

        assert not result.success
        assert "step cannot be zero" in str(result.error)

    def test_upto_basic(self):
        """Test basic upto method."""
        self.session.execute_statement("nums = []")
        result = self.session.execute_statement("nums.upto(5)")

        assert result.success
        assert isinstance(result.value, ListValue)
        assert [elem.value for elem in result.value.elements] == [0, 1, 2, 3, 4, 5]
        assert result.value.constraint == "num"

    def test_upto_zero(self):
        """Test upto(0)."""
        self.session.execute_statement("nums = []")
        result = self.session.execute_statement("nums.upto(0)")

        assert result.success
        assert isinstance(result.value, ListValue)
        assert [elem.value for elem in result.value.elements] == [0]

    def test_upto_negative(self):
        """Test upto with negative number (produces empty list due to range)."""
        self.session.execute_statement("nums = []")
        result = self.session.execute_statement("nums.upto(-1)")

        assert result.success
        assert isinstance(result.value, ListValue)
        assert len(result.value.elements) == 0

    def test_from_function_with_lambda(self):
        """Test from_function with lambda expression."""
        self.session.execute_statement("nums = []")
        self.session.execute_statement("double = x => x * 2")
        result = self.session.execute_statement("nums.from_function(5, double)")

        assert result.success
        assert isinstance(result.value, ListValue)
        assert [elem.value for elem in result.value.elements] == [0, 2, 4, 6, 8]

    def test_from_function_with_function(self):
        """Test from_function with regular function."""
        self.session.execute_statement("nums = []")

        # Define function and verify it was created successfully
        func_def = self.session.execute_statement('''
        func square(x) {
            return x * x
        }
        ''')
        assert func_def.success, f"Function definition failed: {func_def.error}"

        result = self.session.execute_statement("nums.from_function(4, square)")

        assert result.success, f"from_function call failed: {result.error}"
        assert isinstance(result.value, ListValue)
        assert [elem.value for elem in result.value.elements] == [0, 1, 4, 9]

    def test_from_function_string_generation(self):
        """Test from_function generating strings."""
        self.session.execute_statement("items = []")
        self.session.execute_statement('make_label = x => "Item " + x.to_string()')
        result = self.session.execute_statement("items.from_function(3, make_label)")

        assert result.success
        assert isinstance(result.value, ListValue)
        assert result.value.constraint == "string"
        values = [elem.value for elem in result.value.elements]
        assert values == ["Item 0", "Item 1", "Item 2"]

    def test_from_function_zero_count(self):
        """Test from_function with count=0."""
        self.session.execute_statement("nums = []")
        self.session.execute_statement("identity = x => x")
        result = self.session.execute_statement("nums.from_function(0, identity)")

        assert result.success
        assert isinstance(result.value, ListValue)
        assert len(result.value.elements) == 0

    def test_from_function_negative_count_error(self):
        """Test that negative count raises error."""
        self.session.execute_statement("nums = []")
        self.session.execute_statement("identity = x => x")
        result = self.session.execute_statement("nums.from_function(-1, identity)")

        assert not result.success
        assert "count cannot be negative" in str(result.error)

    def test_chaining_generators(self):
        """Test chaining generator methods with other list methods."""
        self.session.execute_statement("nums = []")
        result = self.session.execute_statement('nums.generate(1, 10, 1).filter("even").map("double")')

        assert result.success
        assert isinstance(result.value, ListValue)
        assert [elem.value for elem in result.value.elements] == [4, 8, 12, 16, 20]

    def test_generate_with_type_errors(self):
        """Test generate with invalid argument types."""
        # Test non-number start
        self.session.execute_statement("nums = []")
        result = self.session.execute_statement('nums.generate("1", 5, 1)')
        assert not result.success
        assert "start must be a number" in str(result.error)

        # Test non-number end
        result = self.session.execute_statement('nums.generate(1, "5", 1)')
        assert not result.success
        assert "end must be a number" in str(result.error)

        # Test non-number step
        result = self.session.execute_statement('nums.generate(1, 5, "1")')
        assert not result.success
        assert "step must be a number" in str(result.error)

    def test_upto_with_type_error(self):
        """Test upto with invalid argument type."""
        self.session.execute_statement("nums = []")
        result = self.session.execute_statement('nums.upto("5")')

        assert not result.success
        assert "argument must be a number" in str(result.error)

    def test_from_function_with_type_errors(self):
        """Test from_function with invalid argument types."""
        # Test non-integer count
        self.session.execute_statement("nums = []")
        self.session.execute_statement("identity = x => x")
        result = self.session.execute_statement('nums.from_function("5", identity)')

        assert not result.success
        assert "count must be an integer" in str(result.error)

        # Test non-function second argument
        self.session.execute_statement("nums = []")
        result = self.session.execute_statement("nums.from_function(5, 42)")

        assert not result.success
        assert "second argument must be a function" in str(result.error)

    def test_generate_large_sequence(self):
        """Test generating a large sequence."""
        self.session.execute_statement("nums = []")
        result = self.session.execute_statement("nums.generate(1, 100, 1)")

        assert result.success
        assert isinstance(result.value, ListValue)
        assert len(result.value.elements) == 100
        assert result.value.elements[0].value == 1
        assert result.value.elements[-1].value == 100

    def test_generate_with_decimal_step(self):
        """Test generate with decimal step values."""
        self.session.execute_statement("nums = []")
        result = self.session.execute_statement("nums.generate(0, 1, 0.1)")

        assert result.success
        assert isinstance(result.value, ListValue)
        # Due to floating point precision, check length and bounds
        assert len(result.value.elements) == 11
        assert abs(result.value.elements[0].value - 0.0) < 0.001
        assert abs(result.value.elements[-1].value - 1.0) < 0.001
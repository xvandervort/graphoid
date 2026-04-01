# Rosetta Stone: Testing Patterns — Python (pytest)

import pytest


# --- Basic assertions ---

def test_addition():
    assert 2 + 2 == 4

def test_string_concatenation():
    assert "hello" + " " + "world" == "hello world"

def test_list_length():
    items = [1, 2, 3]
    assert len(items) == 3

def test_truthy_values():
    assert True
    assert 1
    assert "nonempty"

def test_falsy_values():
    assert not False
    assert not 0
    assert not ""
    assert not None


# --- Grouped tests with setup ---

class Calculator:
    def add(self, a, b):
        return a + b

    def divide(self, a, b):
        if b == 0:
            raise ZeroDivisionError("division by zero")
        return a / b


class TestCalculator:
    def setup_method(self):
        self.calc = Calculator()

    def test_add(self):
        assert self.calc.add(2, 3) == 5

    def test_add_negative(self):
        assert self.calc.add(-1, -2) == -3

    def test_divide(self):
        assert self.calc.divide(10, 2) == 5.0


# --- Exception testing ---

class TestCalculatorErrors:
    def setup_method(self):
        self.calc = Calculator()

    def test_divide_by_zero(self):
        with pytest.raises(ZeroDivisionError):
            self.calc.divide(10, 0)


# --- Approximate matching ---

class TestApproximation:
    def setup_method(self):
        self.calc = Calculator()

    def test_float_division(self):
        result = self.calc.divide(10, 3)
        assert result == pytest.approx(3.333, abs=0.01)


# --- Skipping tests ---

@pytest.mark.skip(reason="not yet implemented")
def test_future_feature():
    assert False


# --- Parameterized tests ---

@pytest.mark.parametrize("a,b,expected", [
    (1, 1, 2),
    (0, 0, 0),
    (-1, 1, 0),
])
def test_addition_parameterized(a, b, expected):
    assert a + b == expected


# --- Multiple assertions ---

class TestMultipleAssertions:
    def test_list_operations(self):
        items = [3, 1, 4, 1, 5]
        assert len(items) == 5
        assert min(items) == 1
        assert max(items) == 5

    def test_string_operations(self):
        s = "Hello, World!"
        assert len(s) == 13
        assert s.upper() == "HELLO, WORLD!"
        assert s.startswith("Hello")
        assert "World" in s


# --- Fixtures ---

@pytest.fixture
def sample_list():
    return [10, 20, 30, 40, 50]

def test_list_sum(sample_list):
    assert sum(sample_list) == 150

def test_list_contains(sample_list):
    assert 30 in sample_list

# Summary:
# pytest uses: assert, classes, setup_method, pytest.raises, pytest.approx
# pytest.mark.skip, pytest.mark.parametrize, @pytest.fixture

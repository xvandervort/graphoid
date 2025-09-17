"""Tests for configuration block behavior enforcement."""

import pytest
from glang.parser import ASTParser
from glang.semantic import SemanticAnalyzer
from glang.execution import ASTExecutor, ExecutionContext
from glang.semantic.symbol_table import SymbolTable


class TestConfigurationBehavior:
    """Test configuration blocks enforce their settings."""

    def setup_method(self):
        """Set up test environment."""
        self.parser = ASTParser()
        self.analyzer = SemanticAnalyzer()  # Creates its own symbol table
        self.symbol_table = self.analyzer.symbol_table
        self.context = ExecutionContext(self.symbol_table)
        self.executor = ASTExecutor(self.context)

    def execute_code(self, code: str):
        """Parse, analyze and execute code."""
        ast = self.parser.parse(code)
        self.analyzer.analyze(ast)
        return self.executor.execute(ast)

    def test_skip_none_default_behavior(self):
        """Test that none values cause errors by default."""
        code = """
        data = [1, 2, none, 4]
        result = data.sum()
        """
        with pytest.raises(Exception) as exc_info:
            self.execute_code(code)
        assert "none" in str(exc_info.value).lower()

    def test_skip_none_true(self):
        """Test that skip_none: true skips none values in operations."""
        code = """
        data = [1, 2, none, 4]
        configure { skip_none: true } {
            result = data.sum()
        }
        result
        """
        result = self.execute_code(code)
        assert result.value == 7  # 1 + 2 + 4, skipping none

    def test_skip_nil_alias(self):
        """Test that skip_nil works as an alias for skip_none."""
        code = """
        data = [1, 2, none, 4]
        configure { skip_nil: true } {
            result = data.sum()
        }
        result
        """
        result = self.execute_code(code)
        assert result.value == 7  # 1 + 2 + 4, skipping none

    def test_skip_none_with_min_max(self):
        """Test skip_none with min and max operations."""
        code = """
        data = [5, none, 2, 8, none, 1]
        configure { skip_none: true } {
            min_val = data.min()
            max_val = data.max()
        }
        min_val
        """
        result = self.execute_code(code)
        assert result.value == 1

        code = """
        data = [5, none, 2, 8, none, 1]
        configure { skip_none: true } {
            min_val = data.min()
            max_val = data.max()
        }
        max_val
        """
        result = self.execute_code(code)
        assert result.value == 8

    def test_decimal_places_configuration(self):
        """Test decimal_places configuration for arithmetic operations."""
        code = """
        configure { decimal_places: 2 } {
            result = 10.0 / 3.0
        }
        result
        """
        result = self.execute_code(code)
        assert result.value == 3.33  # Rounded to 2 decimal places

    def test_decimal_places_zero(self):
        """Test decimal_places: 0 for integer arithmetic."""
        code = """
        configure { decimal_places: 0 } {
            pi = 3.14159
            result = pi * 2
        }
        result
        """
        result = self.execute_code(code)
        assert result.value == 6  # Rounded to integer

    def test_nested_configuration_override(self):
        """Test that nested configurations override outer ones."""
        code = """
        data = [1, 2, none, 4]
        configure { skip_none: false } {
            # Outer config: skip_none = false
            configure { skip_none: true } {
                # Inner config: skip_none = true
                inner_result = data.sum()
            }
        }
        inner_result
        """
        result = self.execute_code(code)
        assert result.value == 7  # none skipped in inner block

    def test_configuration_scope_restoration(self):
        """Test that configuration is restored after block exits."""
        code = """
        data = [1, 2, none, 4]

        # First, use skip_none: true
        configure { skip_none: true } {
            result1 = data.sum()
        }

        # After block, skip_none should be back to false (default)
        # This should fail with an error about none
        """
        # Execute the configure block successfully
        self.execute_code(code[:code.rfind("# After")])

        # Now try to sum again outside the block - should fail
        with pytest.raises(Exception) as exc_info:
            self.execute_code("""
            data = [1, 2, none, 4]
            result = data.sum()
            """)
        assert "none" in str(exc_info.value).lower()

    def test_multiple_configuration_settings(self):
        """Test multiple configuration settings together."""
        code = """
        data = [1, 2, none, 4]
        configure { skip_none: true, decimal_places: 1 } {
            sum_val = data.sum()
            result = sum_val / 3.0
        }
        result
        """
        result = self.execute_code(code)
        # sum = 7, divided by 3 = 2.333..., rounded to 1 decimal = 2.3
        assert result.value == 2.3

    def test_configuration_with_arithmetic_operations(self):
        """Test that all arithmetic operations respect decimal_places."""
        code = """
        configure { decimal_places: 2 } {
            add_result = 1.111 + 2.222
            sub_result = 5.555 - 2.222
            mul_result = 3.333 * 2.0
            div_result = 10.0 / 3.0
        }
        add_result
        """
        result = self.execute_code(code)
        assert result.value == 3.33

        code = """
        configure { decimal_places: 2 } {
            add_result = 1.111 + 2.222
            sub_result = 5.555 - 2.222
            mul_result = 3.333 * 2.0
            div_result = 10.0 / 3.0
        }
        sub_result
        """
        result = self.execute_code(code)
        assert result.value == 3.33

        code = """
        configure { decimal_places: 2 } {
            add_result = 1.111 + 2.222
            sub_result = 5.555 - 2.222
            mul_result = 3.333 * 2.0
            div_result = 10.0 / 3.0
        }
        mul_result
        """
        result = self.execute_code(code)
        assert result.value == 6.67

    def test_empty_configuration_block(self):
        """Test that empty configuration blocks work."""
        code = """
        configure { } {
            result = 1 + 2
        }
        result
        """
        result = self.execute_code(code)
        assert result.value == 3
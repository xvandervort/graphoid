"""Test mathematical methods on numbers."""

import pytest
import math
from glang.execution.executor import ASTExecutor, ExecutionContext
from glang.execution.values import NumberValue, BooleanValue, StringValue
from glang.semantic.symbol_table import SymbolTable
from glang.parser.ast_parser import ASTParser
from glang.semantic.analyzer import SemanticAnalyzer


class TestMathematicalMethods:
    """Test mathematical methods for numbers."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.parser = ASTParser()
        self.analyzer = SemanticAnalyzer()
        self.symbol_table = SymbolTable()
        self.context = ExecutionContext(self.symbol_table)
        self.executor = ASTExecutor(self.context)
    
    def execute(self, code: str):
        """Parse, analyze and execute code."""
        ast = self.parser.parse(code)
        result = self.analyzer.analyze(ast, clear_state=False)  # Keep symbol table state
        assert result.success, f"Analysis failed: {result.errors}"
        return self.executor.execute(result.ast)
    
    def test_abs_method(self):
        """Test absolute value method."""
        # Positive number
        self.execute("x = 42")
        result = self.execute("x.abs()")
        assert isinstance(result, NumberValue)
        assert result.value == 42
        
        # Negative number
        self.execute("y = -42")
        result = self.execute("y.abs()")
        assert isinstance(result, NumberValue)
        assert result.value == 42
        
        # Zero
        self.execute("z = 0")
        result = self.execute("z.abs()")
        assert isinstance(result, NumberValue)
        assert result.value == 0
    
    def test_sqrt_method(self):
        """Test square root method."""
        # Perfect square
        self.execute("x = 16")
        result = self.execute("x.sqrt()")
        assert isinstance(result, NumberValue)
        assert result.value == 4.0
        
        # Non-perfect square
        self.execute("y = 2")
        result = self.execute("y.sqrt()")
        assert isinstance(result, NumberValue)
        assert abs(result.value - math.sqrt(2)) < 0.0001
        
        # Zero
        self.execute("z = 0")
        result = self.execute("z.sqrt()")
        assert isinstance(result, NumberValue)
        assert result.value == 0
        
        # Negative number should error
        self.execute("n = -4")
        with pytest.raises(Exception) as exc_info:
            self.execute("n.sqrt()")
        assert "Cannot take square root of negative number" in str(exc_info.value)
    
    def test_log_method(self):
        """Test logarithm method."""
        # Natural log
        self.execute("x = 2.718281828459045")  # e
        result = self.execute("x.log()")
        assert isinstance(result, NumberValue)
        assert abs(result.value - 1.0) < 0.0001
        
        # Log base 10
        self.execute("y = 100")
        result = self.execute("y.log(10)")
        assert isinstance(result, NumberValue)
        assert abs(result.value - 2.0) < 0.0001
        
        # Log base 2
        self.execute("z = 8")
        result = self.execute("z.log(2)")
        assert isinstance(result, NumberValue)
        assert abs(result.value - 3.0) < 0.0001
        
        # Non-positive number should error
        self.execute("n = 0")
        with pytest.raises(Exception) as exc_info:
            self.execute("n.log()")
        assert "Cannot take logarithm of non-positive number" in str(exc_info.value)
        
        # Invalid base should error
        self.execute("m = 10")
        with pytest.raises(Exception) as exc_info:
            self.execute("m.log(1)")
        assert "Logarithm base must be positive and not equal to 1" in str(exc_info.value)
    
    def test_pow_method(self):
        """Test power method."""
        # Integer exponent
        self.execute("x = 2")
        result = self.execute("x.pow(3)")
        assert isinstance(result, NumberValue)
        assert result.value == 8
        
        # Floating point exponent
        self.execute("y = 4")
        result = self.execute("y.pow(0.5)")
        assert isinstance(result, NumberValue)
        assert result.value == 2.0
        
        # Negative exponent
        self.execute("z = 2")
        result = self.execute("z.pow(-1)")
        assert isinstance(result, NumberValue)
        assert result.value == 0.5
        
        # Zero exponent
        self.execute("a = 5")
        result = self.execute("a.pow(0)")
        assert isinstance(result, NumberValue)
        assert result.value == 1
    
    def test_rounding_methods(self):
        """Test rounding methods."""
        self.execute("x = 3.14159")
        
        # Round to nearest integer
        result = self.execute("x.rnd()")
        assert isinstance(result, NumberValue)
        assert result.value == 3
        
        # Round to 2 decimal places
        result = self.execute("x.rnd(2)")
        assert isinstance(result, NumberValue)
        assert result.value == 3.14
        
        # Round up (ceiling)
        result = self.execute("x.rnd_up()")
        assert isinstance(result, NumberValue)
        assert result.value == 4
        
        # Round up to 2 decimal places
        result = self.execute("x.rnd_up(2)")
        assert isinstance(result, NumberValue)
        assert result.value == 3.15
        
        # Round down (floor)
        result = self.execute("x.rnd_dwn()")
        assert isinstance(result, NumberValue)
        assert result.value == 3
        
        # Round down to 2 decimal places
        result = self.execute("x.rnd_dwn(2)")
        assert isinstance(result, NumberValue)
        assert result.value == 3.14
    
    def test_negative_rounding(self):
        """Test rounding with negative numbers."""
        self.execute("x = -2.7")
        
        # Round to nearest
        result = self.execute("x.rnd()")
        assert isinstance(result, NumberValue)
        assert result.value == -3
        
        # Ceiling (towards positive infinity)
        result = self.execute("x.rnd_up()")
        assert isinstance(result, NumberValue)
        assert result.value == -2
        
        # Floor (towards negative infinity)
        result = self.execute("x.rnd_dwn()")
        assert isinstance(result, NumberValue)
        assert result.value == -3
    
    def test_to_method_truncation(self):
        """Test the to() method for precision truncation."""
        self.execute("x = 3.14159")
        
        # Truncate to integer
        result = self.execute("x.to(0)")
        assert isinstance(result, NumberValue)
        assert result.value == 3
        
        # Truncate to 2 decimal places
        result = self.execute("x.to(2)")
        assert isinstance(result, NumberValue)
        assert result.value == 3.14
        
        # Truncate to 4 decimal places
        result = self.execute("x.to(4)")
        assert isinstance(result, NumberValue)
        assert result.value == 3.1415


class TestBooleanMathMethods:
    """Test mathematical-related methods for booleans."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.parser = ASTParser()
        self.analyzer = SemanticAnalyzer()
        self.symbol_table = SymbolTable()
        self.context = ExecutionContext(self.symbol_table)
        self.executor = ASTExecutor(self.context)
    
    def execute(self, code: str):
        """Parse, analyze and execute code."""
        ast = self.parser.parse(code)
        result = self.analyzer.analyze(ast, clear_state=False)  # Keep symbol table state
        assert result.success, f"Analysis failed: {result.errors}"
        return self.executor.execute(result.ast)
    
    def test_flip_method(self):
        """Test boolean flip method."""
        self.execute("t = true")
        result = self.execute("t.flip()")
        assert isinstance(result, BooleanValue)
        assert result.value is False
        
        self.execute("f = false")
        result = self.execute("f.flip()")
        assert isinstance(result, BooleanValue)
        assert result.value is True
    
    def test_toggle_method(self):
        """Test boolean toggle method (alias for flip)."""
        self.execute("t = true")
        result = self.execute("t.toggle()")
        assert isinstance(result, BooleanValue)
        assert result.value is False
        
        self.execute("f = false")
        result = self.execute("f.toggle()")
        assert isinstance(result, BooleanValue)
        assert result.value is True
    
    def test_numify_method(self):
        """Test boolean to number conversion."""
        self.execute("t = true")
        result = self.execute("t.numify()")
        assert isinstance(result, NumberValue)
        assert result.value == 1
        
        self.execute("f = false")
        result = self.execute("f.numify()")
        assert isinstance(result, NumberValue)
        assert result.value == 0
    
    def test_toNum_method(self):
        """Test boolean toNum method (alias for numify)."""
        self.execute("t = true")
        result = self.execute("t.toNum()")
        assert isinstance(result, NumberValue)
        assert result.value == 1
        
        self.execute("f = false")
        result = self.execute("f.toNum()")
        assert isinstance(result, NumberValue)
        assert result.value == 0
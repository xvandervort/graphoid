"""Test type casting methods for all basic types."""

import pytest
from glang.execution.executor import ASTExecutor, ExecutionContext
from glang.execution.values import NumberValue, BooleanValue, StringValue
from glang.semantic.symbol_table import SymbolTable
from glang.parser.ast_parser import ASTParser
from glang.semantic.analyzer import SemanticAnalyzer


class TestNumberTypeCasting:
    """Test type casting methods for numbers."""
    
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
    
    def test_num_to_string(self):
        """Test converting numbers to strings."""
        # Integer
        self.execute("x = 42")
        result = self.execute("x.to_string()")
        assert isinstance(result, StringValue)
        assert result.value == "42"
        
        # Float
        self.execute("y = 3.14")
        result = self.execute("y.to_string()")
        assert isinstance(result, StringValue)
        assert result.value == "3.14"
        
        # Negative
        self.execute("z = -10")
        result = self.execute("z.to_string()")
        assert isinstance(result, StringValue)
        assert result.value == "-10"
        
        # Zero
        self.execute("a = 0")
        result = self.execute("a.to_string()")
        assert isinstance(result, StringValue)
        assert result.value == "0"
    
    def test_num_to_bool(self):
        """Test converting numbers to booleans."""
        # Positive number -> true
        self.execute("x = 42")
        result = self.execute("x.to_bool()")
        assert isinstance(result, BooleanValue)
        assert result.value is True
        
        # Negative number -> true
        self.execute("y = -1")
        result = self.execute("y.to_bool()")
        assert isinstance(result, BooleanValue)
        assert result.value is True
        
        # Zero -> false
        self.execute("z = 0")
        result = self.execute("z.to_bool()")
        assert isinstance(result, BooleanValue)
        assert result.value is False
        
        # Float -> true
        self.execute("a = 0.001")
        result = self.execute("a.to_bool()")
        assert isinstance(result, BooleanValue)
        assert result.value is True
    
    def test_num_to_num(self):
        """Test converting numbers to numbers (identity)."""
        self.execute("x = 42")
        result = self.execute("x.to_num()")
        assert isinstance(result, NumberValue)
        assert result.value == 42


class TestStringTypeCasting:
    """Test type casting methods for strings."""
    
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
    
    def test_string_to_num(self):
        """Test converting strings to numbers."""
        # Integer string
        self.execute('x = "42"')
        result = self.execute("x.to_num()")
        assert isinstance(result, NumberValue)
        assert result.value == 42
        
        # Float string
        self.execute('y = "3.14"')
        result = self.execute("y.to_num()")
        assert isinstance(result, NumberValue)
        assert result.value == 3.14
        
        # Negative number string
        self.execute('z = "-10"')
        result = self.execute("z.to_num()")
        assert isinstance(result, NumberValue)
        assert result.value == -10
        
        # Invalid number string should error
        self.execute('bad = "hello"')
        with pytest.raises(Exception) as exc_info:
            self.execute("bad.to_num()")
        assert "Cannot convert 'hello' to number" in str(exc_info.value)
    
    def test_string_to_bool(self):
        """Test converting strings to booleans."""
        # Non-empty string -> true
        self.execute('x = "hello"')
        result = self.execute("x.to_bool()")
        assert isinstance(result, BooleanValue)
        assert result.value is True
        
        # Empty string -> false
        self.execute('y = ""')
        result = self.execute("y.to_bool()")
        assert isinstance(result, BooleanValue)
        assert result.value is False
        
        # Whitespace string -> true
        self.execute('z = " "')
        result = self.execute("z.to_bool()")
        assert isinstance(result, BooleanValue)
        assert result.value is True
    
    def test_string_to_string(self):
        """Test converting strings to strings (identity)."""
        self.execute('x = "hello"')
        result = self.execute("x.to_string()")
        assert isinstance(result, StringValue)
        assert result.value == "hello"


class TestBooleanTypeCasting:
    """Test type casting methods for booleans."""
    
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
    
    def test_bool_to_string(self):
        """Test converting booleans to strings."""
        # true -> "true"
        self.execute("t = true")
        result = self.execute("t.to_string()")
        assert isinstance(result, StringValue)
        assert result.value == "true"
        
        # false -> "false"
        self.execute("f = false")
        result = self.execute("f.to_string()")
        assert isinstance(result, StringValue)
        assert result.value == "false"
    
    def test_bool_to_num(self):
        """Test converting booleans to numbers."""
        # true -> 1
        self.execute("t = true")
        result = self.execute("t.to_num()")
        assert isinstance(result, NumberValue)
        assert result.value == 1
        
        # false -> 0
        self.execute("f = false")
        result = self.execute("f.to_num()")
        assert isinstance(result, NumberValue)
        assert result.value == 0
    
    def test_bool_to_bool(self):
        """Test converting booleans to booleans (identity)."""
        self.execute("t = true")
        result = self.execute("t.to_bool()")
        assert isinstance(result, BooleanValue)
        assert result.value is True
        
        self.execute("f = false")
        result = self.execute("f.to_bool()")
        assert isinstance(result, BooleanValue)
        assert result.value is False


class TestListTypeCasting:
    """Test type casting methods for lists."""
    
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
    
    def test_list_to_string(self):
        """Test converting lists to strings."""
        # List of numbers
        self.execute("x = [1, 2, 3]")
        result = self.execute("x.to_string()")
        assert isinstance(result, StringValue)
        assert result.value == "[1, 2, 3]"
        
        # List of strings
        self.execute('y = ["a", "b", "c"]')
        result = self.execute("y.to_string()")
        assert isinstance(result, StringValue)
        assert result.value == "[a, b, c]"
        
        # Mixed list
        self.execute('z = [1, "hello", true]')
        result = self.execute("z.to_string()")
        assert isinstance(result, StringValue)
        assert result.value == "[1, hello, true]"
        
        # Empty list
        self.execute("empty = []")
        result = self.execute("empty.to_string()")
        assert isinstance(result, StringValue)
        assert result.value == "[]"
    
    def test_list_to_bool(self):
        """Test converting lists to booleans."""
        # Non-empty list -> true
        self.execute("x = [1, 2, 3]")
        result = self.execute("x.to_bool()")
        assert isinstance(result, BooleanValue)
        assert result.value is True
        
        # Single element list -> true
        self.execute("y = [42]")
        result = self.execute("y.to_bool()")
        assert isinstance(result, BooleanValue)
        assert result.value is True
        
        # Empty list -> false
        self.execute("z = []")
        result = self.execute("z.to_bool()")
        assert isinstance(result, BooleanValue)
        assert result.value is False


class TestTypeCastingIntegration:
    """Test type casting in practical scenarios."""
    
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
    
    def test_string_concatenation_with_numbers(self):
        """Test concatenating strings with converted numbers."""
        self.execute("x = 42")
        self.execute('msg = "The answer is " + x.to_string()')
        result = self.context.get_variable("msg")
        assert isinstance(result, StringValue)
        assert result.value == "The answer is 42"
    
    def test_numeric_parsing_from_string(self):
        """Test parsing numbers from strings and doing math."""
        self.execute('x = "10"')
        self.execute('y = "20"')
        self.execute("sum = x.to_num() + y.to_num()")
        result = self.context.get_variable("sum")
        assert isinstance(result, NumberValue)
        assert result.value == 30
    
    def test_boolean_arithmetic(self):
        """Test converting booleans to numbers for arithmetic."""
        self.execute("t = true")
        self.execute("f = false")
        self.execute("result = t.to_num() + f.to_num()")
        result = self.context.get_variable("result")
        assert isinstance(result, NumberValue)
        assert result.value == 1
    
    def test_chained_conversions(self):
        """Test chaining multiple type conversions."""
        # Number -> String -> Boolean
        self.execute("x = 42")
        self.execute("result = x.to_string().to_bool()")
        result = self.context.get_variable("result")
        assert isinstance(result, BooleanValue)
        assert result.value is True
        
        # Boolean -> Number -> String
        self.execute("b = true")
        self.execute("result2 = b.to_num().to_string()")
        result = self.context.get_variable("result2")
        assert isinstance(result, StringValue)
        assert result.value == "1"
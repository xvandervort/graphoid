"""Basic tests for executor functionality to improve coverage."""

import pytest
from unittest.mock import Mock

from src.glang.execution.executor import BreakException, ContinueException, ReturnException, ExecutionContext
from src.glang.execution.values import StringValue, NumberValue, BooleanValue
from src.glang.semantic.symbol_table import SymbolTable
from src.glang.ast.nodes import SourcePosition


class TestExecutorExceptions:
    """Test executor exception classes."""
    
    def test_break_exception(self):
        """Test BreakException."""
        exc = BreakException()
        assert isinstance(exc, Exception)
    
    def test_continue_exception(self):
        """Test ContinueException."""
        exc = ContinueException()
        assert isinstance(exc, Exception)
    
    def test_return_exception_no_value(self):
        """Test ReturnException without value."""
        exc = ReturnException()
        assert isinstance(exc, Exception)
        assert exc.value is None
    
    def test_return_exception_with_value(self):
        """Test ReturnException with value."""
        pos = SourcePosition(1, 1)
        value = StringValue("test", pos)
        exc = ReturnException(value)
        
        assert isinstance(exc, Exception)
        assert exc.value == value


class TestExecutionContext:
    """Test ExecutionContext class."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.symbol_table = SymbolTable()
        self.context = ExecutionContext(self.symbol_table)
    
    def test_execution_context_initialization(self):
        """Test ExecutionContext initialization."""
        assert self.context.symbol_table == self.symbol_table
        assert isinstance(self.context.variables, dict)
        assert len(self.context.variables) == 0
        assert self.context.module_manager is None
    
    def test_execution_context_with_module_manager(self):
        """Test ExecutionContext with module manager."""
        mock_manager = Mock()
        context = ExecutionContext(self.symbol_table, mock_manager)
        
        assert context.module_manager == mock_manager
    
    def test_get_variable_not_found(self):
        """Test getting non-existent variable."""
        result = self.context.get_variable("nonexistent")
        assert result is None
    
    def test_set_and_get_variable(self):
        """Test setting and getting variable."""
        pos = SourcePosition(1, 1)
        value = StringValue("test", pos)
        
        self.context.set_variable("test_var", value)
        retrieved = self.context.get_variable("test_var")
        
        assert retrieved == value
    
    def test_has_variable(self):
        """Test checking if variable exists."""
        pos = SourcePosition(1, 1)
        value = NumberValue(42, pos)
        
        assert self.context.has_variable("test_var") == False
        
        self.context.set_variable("test_var", value)
        assert self.context.has_variable("test_var") == True
    
    def test_list_variables(self):
        """Test listing all variables."""
        pos = SourcePosition(1, 1)
        value1 = StringValue("test1", pos)
        value2 = NumberValue(42, pos)
        
        self.context.set_variable("var1", value1)
        self.context.set_variable("var2", value2)
        
        var_list = self.context.list_variables()
        assert len(var_list) == 2
        assert "var1" in var_list
        assert "var2" in var_list
    
    def test_get_all_variables(self):
        """Test getting all variables through direct access."""
        pos = SourcePosition(1, 1)
        value1 = StringValue("test1", pos)
        value2 = NumberValue(42, pos)
        
        self.context.set_variable("var1", value1)
        self.context.set_variable("var2", value2)
        
        # Access variables directly through the dictionary
        all_vars = self.context.variables
        
        assert len(all_vars) == 2
        assert "var1" in all_vars
        assert "var2" in all_vars
        assert all_vars["var1"] == value1
        assert all_vars["var2"] == value2
    
    def test_module_qualified_variable_no_manager(self):
        """Test module-qualified variable access without module manager."""
        # Should return None when no module manager
        result = self.context.get_variable("math.pi")
        assert result is None
    
    def test_module_qualified_variable_with_manager(self):
        """Test module-qualified variable access with module manager."""
        mock_manager = Mock()
        mock_module = Mock()
        mock_namespace = Mock()
        mock_value = NumberValue(3.14159, SourcePosition(1, 1))
        
        # Mock chain of calls
        mock_manager.get_module.return_value = mock_module
        mock_module.namespace = mock_namespace
        # get_symbol returns the actual value, not a symbol object
        mock_namespace.get_symbol.return_value = mock_value
        
        context = ExecutionContext(self.symbol_table, mock_manager)
        result = context.get_variable("math.pi")
        
        mock_manager.get_module.assert_called_once_with("math")
        mock_namespace.get_symbol.assert_called_once_with("pi")
        assert result == mock_value
    
    def test_module_qualified_variable_not_found(self):
        """Test module-qualified variable that doesn't exist."""
        mock_manager = Mock()
        mock_module = Mock()
        mock_namespace = Mock()
        
        mock_manager.get_module.return_value = mock_module
        mock_module.namespace = mock_namespace
        mock_namespace.get_symbol.return_value = None  # Symbol not found
        
        context = ExecutionContext(self.symbol_table, mock_manager)
        result = context.get_variable("nonexistent.var")
        
        mock_manager.get_module.assert_called_once_with("nonexistent")
        mock_namespace.get_symbol.assert_called_once_with("var")
        assert result is None


class TestExecutionContextVariableOperations:
    """Test ExecutionContext variable operations."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.symbol_table = SymbolTable()
        self.context = ExecutionContext(self.symbol_table)
        self.pos = SourcePosition(1, 1)
    
    def test_variable_overwrite(self):
        """Test overwriting existing variable."""
        value1 = StringValue("original", self.pos)
        value2 = StringValue("updated", self.pos)
        
        self.context.set_variable("var", value1)
        assert self.context.get_variable("var") == value1
        
        self.context.set_variable("var", value2)
        assert self.context.get_variable("var") == value2
    
    def test_multiple_variables(self):
        """Test managing multiple variables."""
        str_val = StringValue("text", self.pos)
        num_val = NumberValue(42, self.pos)
        bool_val = BooleanValue(True, self.pos)
        
        self.context.set_variable("string_var", str_val)
        self.context.set_variable("number_var", num_val)
        self.context.set_variable("boolean_var", bool_val)
        
        assert self.context.get_variable("string_var") == str_val
        assert self.context.get_variable("number_var") == num_val
        assert self.context.get_variable("boolean_var") == bool_val
        
        # Use list_variables or direct access
        var_list = self.context.list_variables()
        assert len(var_list) == 3
    
    def test_variable_names_case_sensitive(self):
        """Test that variable names are case sensitive."""
        value = StringValue("test", self.pos)
        
        self.context.set_variable("TestVar", value)
        
        assert self.context.get_variable("TestVar") == value
        assert self.context.get_variable("testvar") is None
        assert self.context.get_variable("TESTVAR") is None
    
    def test_special_variable_names(self):
        """Test variables with special characters in names."""
        value = NumberValue(123, self.pos)
        
        # Test underscore
        self.context.set_variable("var_with_underscore", value)
        assert self.context.get_variable("var_with_underscore") == value
        
        # Test numbers in name
        self.context.set_variable("var123", value)
        assert self.context.get_variable("var123") == value
    
    def test_empty_variable_name(self):
        """Test handling of empty variable name."""
        value = StringValue("test", self.pos)
        
        # Should handle gracefully
        self.context.set_variable("", value)
        assert self.context.get_variable("") == value
    
    def test_none_variable_value(self):
        """Test setting None as variable value."""
        # Should handle None values
        self.context.set_variable("none_var", None)
        assert self.context.get_variable("none_var") is None
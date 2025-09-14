"""Tests for the AST executor."""

import pytest
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../src'))

from glang.execution.executor import ASTExecutor, ExecutionContext
from glang.execution.values import *
from glang.execution.errors import *
from glang.semantic.symbol_table import SymbolTable, Symbol
from glang.ast.nodes import *


class TestExecutionContext:
    """Test ExecutionContext functionality."""
    
    def setup_method(self):
        self.symbol_table = SymbolTable()
        self.context = ExecutionContext(self.symbol_table)
    
    def test_variable_storage(self):
        """Test basic variable storage and retrieval."""
        value = StringValue("hello")
        
        # Test setting and getting
        self.context.set_variable("test", value)
        retrieved = self.context.get_variable("test")
        assert retrieved == value
        
        # Test has_variable
        assert self.context.has_variable("test") is True
        assert self.context.has_variable("nonexistent") is False
        
        # Test list_variables
        variables = self.context.list_variables()
        assert "test" in variables
    
    def test_multiple_variables(self):
        """Test managing multiple variables."""
        str_val = StringValue("hello")
        num_val = NumberValue(42)
        
        self.context.set_variable("str_var", str_val)
        self.context.set_variable("num_var", num_val)
        
        assert len(self.context.list_variables()) == 2
        assert self.context.get_variable("str_var") == str_val
        assert self.context.get_variable("num_var") == num_val


class TestASTExecutor:
    """Test AST executor functionality."""
    
    def setup_method(self):
        self.symbol_table = SymbolTable()
        self.context = ExecutionContext(self.symbol_table)
        self.executor = ASTExecutor(self.context)
    
    def test_literal_evaluation(self):
        """Test evaluation of literal expressions."""
        # String literal
        str_node = StringLiteral('"hello"', SourcePosition(1, 1))
        result = self.executor.execute(str_node)
        assert isinstance(result, StringValue)
        assert result.value == "hello"
        
        # Number literal
        num_node = NumberLiteral(42, SourcePosition(1, 1))
        result = self.executor.execute(num_node)
        assert isinstance(result, NumberValue)
        assert result.value == 42
        
        # Boolean literal
        bool_node = BooleanLiteral(True, SourcePosition(1, 1))
        result = self.executor.execute(bool_node)
        assert isinstance(result, BooleanValue)
        assert result.value is True
    
    def test_list_literal_evaluation(self):
        """Test evaluation of list literals."""
        elements = [
            NumberLiteral(1, SourcePosition(1, 2)),
            NumberLiteral(2, SourcePosition(1, 5)),
            NumberLiteral(3, SourcePosition(1, 8))
        ]
        
        list_node = ListLiteral(elements, SourcePosition(1, 1))
        result = self.executor.execute(list_node)
        
        assert isinstance(result, ListValue)
        assert len(result) == 3
        assert result.elements[0].value == 1
        assert result.elements[1].value == 2
        assert result.elements[2].value == 3
    
    def test_variable_declaration(self):
        """Test variable declaration execution."""
        # Declare symbol in symbol table first
        symbol = Symbol("test_var", "string", None, SourcePosition(1, 1))
        self.symbol_table.declare_symbol(symbol)
        
        # Create declaration node
        initializer = StringLiteral('"hello"', SourcePosition(1, 15))
        declaration = VariableDeclaration(
            var_type="string", 
            name="test_var", 
            initializer=initializer, 
            type_constraint=None,
            behaviors=None,
            position=SourcePosition(1, 1)
        )
        
        result = self.executor.execute(declaration)
        
        # Check that variable was stored in context
        stored_value = self.context.get_variable("test_var")
        assert isinstance(stored_value, StringValue)
        assert stored_value.value == "hello"
        
        # Check result message
        assert "Declared string variable 'test_var'" in result
    
    def test_variable_declaration_with_constraint(self):
        """Test variable declaration with type constraints."""
        # Declare constrained list
        symbol = Symbol("numbers", "list", "num", SourcePosition(1, 1))
        self.symbol_table.declare_symbol(symbol)
        
        elements = [NumberLiteral(1), NumberLiteral(2), NumberLiteral(3)]
        initializer = ListLiteral(elements, SourcePosition(1, 20))
        declaration = VariableDeclaration(
            var_type="list", 
            name="numbers", 
            initializer=initializer, 
            type_constraint="num",
            behaviors=None,
            position=SourcePosition(1, 1)
        )
        
        result = self.executor.execute(declaration)
        
        # Check that constraint was set
        stored_value = self.context.get_variable("numbers")
        assert isinstance(stored_value, ListValue)
        assert stored_value.constraint == "num"
        assert len(stored_value) == 3
    
    def test_variable_reference(self):
        """Test variable reference evaluation."""
        # Store a variable first
        test_value = StringValue("hello")
        self.context.set_variable("test_var", test_value)
        
        # Create variable reference
        var_ref = VariableRef("test_var", SourcePosition(1, 1))
        result = self.executor.execute(var_ref)
        
        assert result == test_value
    
    def test_variable_reference_not_found(self):
        """Test variable reference with undefined variable."""
        var_ref = VariableRef("undefined", SourcePosition(1, 1))
        
        with pytest.raises(VariableNotFoundError) as exc_info:
            self.executor.execute(var_ref)
        
        assert "Variable 'undefined' not found" in str(exc_info.value)
    
    def test_index_access(self):
        """Test index access evaluation."""
        # Create a list variable
        list_value = ListValue([
            NumberValue(10),
            NumberValue(20),
            NumberValue(30)
        ])
        self.context.set_variable("test_list", list_value)
        
        # Create index access: test_list[1]
        target = VariableRef("test_list", SourcePosition(1, 1))
        index = NumberLiteral(1, SourcePosition(1, 11))
        index_access = IndexAccess(target, [index], SourcePosition(1, 1))
        
        result = self.executor.execute(index_access)
        
        assert isinstance(result, NumberValue)
        assert result.value == 20
    
    def test_index_access_negative(self):
        """Test negative index access."""
        list_value = ListValue([StringValue("a"), StringValue("b"), StringValue("c")])
        self.context.set_variable("test_list", list_value)
        
        # Access test_list[-1] (last element)
        target = VariableRef("test_list", SourcePosition(1, 1))
        index = NumberLiteral(-1, SourcePosition(1, 11))
        index_access = IndexAccess(target, [index], SourcePosition(1, 1))
        
        result = self.executor.execute(index_access)
        
        assert isinstance(result, StringValue)
        assert result.value == "c"
    
    def test_simple_assignment(self):
        """Test simple variable assignment."""
        # Create initial variable
        initial_value = StringValue("initial")
        self.context.set_variable("test_var", initial_value)
        
        # Create assignment: test_var = "new_value"
        target = VariableRef("test_var", SourcePosition(1, 1))
        new_value = StringLiteral('"new_value"', SourcePosition(1, 12))
        assignment = Assignment(target, new_value, SourcePosition(1, 1))
        
        result = self.executor.execute(assignment)
        
        # Check that variable was updated
        updated_value = self.context.get_variable("test_var")
        assert isinstance(updated_value, StringValue)
        assert updated_value.value == "new_value"
    
    def test_type_inference_assignment(self):
        """Test assignment to undefined variable creates new variable with inferred type."""
        # Create assignment to undefined variable: new_var = "hello"
        target = VariableRef("new_var", SourcePosition(1, 1))
        value = StringLiteral('"hello"', SourcePosition(1, 11))
        assignment = Assignment(target, value, SourcePosition(1, 1))
        
        result = self.executor.execute(assignment)
        
        # Check that result indicates type inference
        assert result == "Declared string variable 'new_var' (inferred)"
        
        # Check that variable was created with correct type
        created_value = self.context.get_variable("new_var")
        assert isinstance(created_value, StringValue)
        assert created_value.value == "hello"
        assert created_value.get_type() == "string"
    
    def test_type_inference_number_assignment(self):
        """Test type inference for number assignment."""
        target = VariableRef("count", SourcePosition(1, 1))
        value = NumberLiteral(42, SourcePosition(1, 9))
        assignment = Assignment(target, value, SourcePosition(1, 1))
        
        result = self.executor.execute(assignment)
        
        assert result == "Declared num variable 'count' (inferred)"
        created_value = self.context.get_variable("count")
        assert isinstance(created_value, NumberValue)
        assert created_value.value == 42
        assert created_value.get_type() == "num"
    
    def test_type_inference_boolean_assignment(self):
        """Test type inference for boolean assignment."""
        target = VariableRef("flag", SourcePosition(1, 1))
        value = BooleanLiteral(True, SourcePosition(1, 8))
        assignment = Assignment(target, value, SourcePosition(1, 1))
        
        result = self.executor.execute(assignment)
        
        assert result == "Declared bool variable 'flag' (inferred)"
        created_value = self.context.get_variable("flag")
        assert isinstance(created_value, BooleanValue)
        assert created_value.value == True
        assert created_value.get_type() == "bool"
    
    def test_type_inference_list_assignment(self):
        """Test type inference for list assignment."""
        target = VariableRef("items", SourcePosition(1, 1))
        elements = [StringLiteral('"a"', SourcePosition(1, 10)), StringLiteral('"b"', SourcePosition(1, 15))]
        value = ListLiteral(elements, SourcePosition(1, 9))
        assignment = Assignment(target, value, SourcePosition(1, 1))
        
        result = self.executor.execute(assignment)
        
        assert result == "Declared list variable 'items' (inferred)"
        created_value = self.context.get_variable("items")
        assert isinstance(created_value, ListValue)
        assert len(created_value.elements) == 2
        assert created_value.get_type() == "list"
        assert created_value.constraint is None  # No constraint inferred
    
    def test_index_assignment(self):
        """Test index assignment."""
        # Create list variable
        list_value = ListValue([NumberValue(1), NumberValue(2), NumberValue(3)])
        self.context.set_variable("numbers", list_value)
        
        # Create index assignment: numbers[1] = 99
        list_ref = VariableRef("numbers", SourcePosition(1, 1))
        index = NumberLiteral(1, SourcePosition(1, 9))
        target = IndexAccess(list_ref, [index], SourcePosition(1, 1))
        new_value = NumberLiteral(99, SourcePosition(1, 14))
        assignment = Assignment(target, new_value, SourcePosition(1, 1))
        
        result = self.executor.execute(assignment)
        
        # Check that list element was updated
        updated_list = self.context.get_variable("numbers")
        assert updated_list.elements[1].value == 99
    
    def test_method_call_append(self):
        """Test method call execution - append."""
        # Create list variable
        list_value = ListValue([NumberValue(1), NumberValue(2)])
        self.context.set_variable("numbers", list_value)
        
        # Create method call: numbers.append(3)
        target = VariableRef("numbers", SourcePosition(1, 1))
        args = [NumberLiteral(3, SourcePosition(1, 17))]
        method_call = MethodCall(target, "append", args, SourcePosition(1, 1))
        
        result = self.executor.execute(method_call)
        
        # Check that element was appended
        updated_list = self.context.get_variable("numbers")
        assert len(updated_list) == 3
        assert updated_list.elements[2].value == 3
    
    def test_method_call_with_constraint_violation(self):
        """Test method call that violates type constraint."""
        # Create constrained list
        list_value = ListValue([NumberValue(1), NumberValue(2)], "num")
        self.context.set_variable("numbers", list_value)
        
        # Try to append string to num-constrained list
        target = VariableRef("numbers", SourcePosition(1, 1))
        args = [StringLiteral('"hello"', SourcePosition(1, 17))]
        method_call = MethodCall(target, "append", args, SourcePosition(1, 1))
        
        with pytest.raises(TypeConstraintError) as exc_info:
            self.executor.execute(method_call)
        
        assert "Cannot append string to list<num>" in str(exc_info.value)


class TestExecutorIntegration:
    """Integration tests for the executor."""
    
    def setup_method(self):
        self.symbol_table = SymbolTable()
        self.context = ExecutionContext(self.symbol_table)
        self.executor = ASTExecutor(self.context)
    
    def test_complete_declaration_and_usage(self):
        """Test complete flow: declare variable, use in operations."""
        # 1. Declare variable: list<string> names = ["alice", "bob"]
        symbol = Symbol("names", "list", "string", SourcePosition(1, 1))
        self.symbol_table.declare_symbol(symbol)
        
        elements = [
            StringLiteral('"alice"', SourcePosition(1, 25)),
            StringLiteral('"bob"', SourcePosition(1, 33))
        ]
        initializer = ListLiteral(elements, SourcePosition(1, 24))
        declaration = VariableDeclaration(
            var_type="list", 
            name="names", 
            initializer=initializer, 
            type_constraint="string",
            behaviors=None,
            position=SourcePosition(1, 1)
        )
        
        self.executor.execute(declaration)
        
        # 2. Append to list: names.append("charlie")
        target = VariableRef("names", SourcePosition(2, 1))
        args = [StringLiteral('"charlie"', SourcePosition(2, 14))]
        method_call = MethodCall(target, "append", args, SourcePosition(2, 1))
        
        self.executor.execute(method_call)
        
        # 3. Access element: names[2]
        list_ref = VariableRef("names", SourcePosition(3, 1))
        index = NumberLiteral(2, SourcePosition(3, 7))
        index_access = IndexAccess(list_ref, [index], SourcePosition(3, 1))
        
        result = self.executor.execute(index_access)
        
        # Verify the result
        assert isinstance(result, StringValue)
        assert result.value == "charlie"
        
        # Verify list has 3 elements
        final_list = self.context.get_variable("names")
        assert len(final_list) == 3
        assert final_list.constraint == "string"


if __name__ == '__main__':
    pytest.main([__file__])
"""Tests for symbol table implementation."""

import pytest
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../src'))

from glang.semantic.symbol_table import Symbol, SymbolTable
from glang.ast.nodes import SourcePosition


class TestSymbol:
    """Test Symbol class functionality."""
    
    def test_symbol_creation(self):
        """Test basic symbol creation."""
        pos = SourcePosition(1, 5)
        symbol = Symbol("myvar", "string", None, pos)
        
        assert symbol.name == "myvar"
        assert symbol.symbol_type == "string"
        assert symbol.type_constraint is None
        assert symbol.position == pos
    
    def test_symbol_with_constraint(self):
        """Test symbol with type constraint."""
        symbol = Symbol("numbers", "list", "num")
        
        assert symbol.name == "numbers"
        assert symbol.symbol_type == "list"
        assert symbol.type_constraint == "num"
        assert symbol.has_constraint() == True
        assert symbol.matches_constraint("num") == True
        assert symbol.matches_constraint("string") == False
    
    def test_symbol_without_constraint(self):
        """Test symbol without type constraint."""
        symbol = Symbol("items", "list")
        
        assert symbol.has_constraint() == False
        assert symbol.matches_constraint("num") == True
        assert symbol.matches_constraint("string") == True
        assert symbol.matches_constraint("anything") == True
    
    def test_symbol_string_representation(self):
        """Test symbol string formatting."""
        pos = SourcePosition(2, 8)
        
        # Without constraint
        symbol1 = Symbol("var1", "string", None, pos)
        assert "string var1 at line 2, column 8" in str(symbol1)
        
        # With constraint
        symbol2 = Symbol("nums", "list", "num", pos)
        assert "list<num> nums at line 2, column 8" in str(symbol2)
        
        # Without position
        symbol3 = Symbol("var3", "bool")
        assert str(symbol3) == "bool var3"


class TestSymbolTable:
    """Test SymbolTable functionality."""
    
    def setup_method(self):
        self.symbol_table = SymbolTable()
    
    def test_empty_table(self):
        """Test empty symbol table."""
        assert self.symbol_table.size() == 0
        assert self.symbol_table.lookup_symbol("nonexistent") is None
        assert self.symbol_table.symbol_exists("nonexistent") == False
        assert self.symbol_table.get_all_symbols() == {}
        assert "empty" in str(self.symbol_table)
    
    def test_declare_symbol(self):
        """Test declaring symbols."""
        symbol = Symbol("myvar", "string")
        self.symbol_table.declare_symbol(symbol)
        
        assert self.symbol_table.size() == 1
        assert self.symbol_table.symbol_exists("myvar") == True
        
        retrieved = self.symbol_table.lookup_symbol("myvar")
        assert retrieved is not None
        assert retrieved.name == "myvar"
        assert retrieved.symbol_type == "string"
    
    def test_declare_duplicate_symbol(self):
        """Test declaring duplicate symbol raises error."""
        pos1 = SourcePosition(1, 1)
        pos2 = SourcePosition(2, 1)
        
        symbol1 = Symbol("dup", "string", None, pos1)
        symbol2 = Symbol("dup", "num", None, pos2)
        
        self.symbol_table.declare_symbol(symbol1)
        
        with pytest.raises(ValueError) as exc_info:
            self.symbol_table.declare_symbol(symbol2)
        
        assert "already declared" in str(exc_info.value)
        assert "line 1" in str(exc_info.value)
    
    def test_multiple_symbols(self):
        """Test multiple symbol declarations."""
        symbols = [
            Symbol("str_var", "string"),
            Symbol("num_var", "num"),
            Symbol("list_var", "list", "string"),
            Symbol("bool_var", "bool")
        ]
        
        for symbol in symbols:
            self.symbol_table.declare_symbol(symbol)
        
        assert self.symbol_table.size() == 4
        
        # Check all symbols exist
        for symbol in symbols:
            assert self.symbol_table.symbol_exists(symbol.name)
            retrieved = self.symbol_table.lookup_symbol(symbol.name)
            assert retrieved.name == symbol.name
            assert retrieved.symbol_type == symbol.symbol_type
            assert retrieved.type_constraint == symbol.type_constraint
    
    def test_get_symbols_by_type(self):
        """Test filtering symbols by type."""
        symbols = [
            Symbol("str1", "string"),
            Symbol("str2", "string"), 
            Symbol("num1", "num"),
            Symbol("list1", "list"),
            Symbol("list2", "list", "num")
        ]
        
        for symbol in symbols:
            self.symbol_table.declare_symbol(symbol)
        
        string_symbols = self.symbol_table.get_symbols_by_type("string")
        assert len(string_symbols) == 2
        assert all(s.symbol_type == "string" for s in string_symbols)
        
        list_symbols = self.symbol_table.get_symbols_by_type("list")
        assert len(list_symbols) == 2
        assert all(s.symbol_type == "list" for s in list_symbols)
        
        bool_symbols = self.symbol_table.get_symbols_by_type("bool")
        assert len(bool_symbols) == 0
    
    def test_get_symbols_with_constraint(self):
        """Test filtering symbols by constraint."""
        symbols = [
            Symbol("list1", "list", "string"),
            Symbol("list2", "list", "num"),
            Symbol("list3", "list", "string"),
            Symbol("list4", "list"),  # No constraint
        ]
        
        for symbol in symbols:
            self.symbol_table.declare_symbol(symbol)
        
        string_constrained = self.symbol_table.get_symbols_with_constraint("string")
        assert len(string_constrained) == 2
        assert all(s.type_constraint == "string" for s in string_constrained)
        
        num_constrained = self.symbol_table.get_symbols_with_constraint("num")
        assert len(num_constrained) == 1
        assert num_constrained[0].name == "list2"
        
        bool_constrained = self.symbol_table.get_symbols_with_constraint("bool")
        assert len(bool_constrained) == 0
    
    def test_remove_symbol(self):
        """Test removing symbols."""
        symbol = Symbol("temp", "string")
        self.symbol_table.declare_symbol(symbol)
        
        assert self.symbol_table.size() == 1
        assert self.symbol_table.symbol_exists("temp") == True
        
        # Remove existing symbol
        result = self.symbol_table.remove_symbol("temp")
        assert result == True
        assert self.symbol_table.size() == 0
        assert self.symbol_table.symbol_exists("temp") == False
        
        # Try to remove non-existent symbol
        result = self.symbol_table.remove_symbol("nonexistent")
        assert result == False
    
    def test_clear_table(self):
        """Test clearing the symbol table."""
        symbols = [
            Symbol("var1", "string"),
            Symbol("var2", "num"),
            Symbol("var3", "list")
        ]
        
        for symbol in symbols:
            self.symbol_table.declare_symbol(symbol)
        
        assert self.symbol_table.size() == 3
        
        self.symbol_table.clear()
        
        assert self.symbol_table.size() == 0
        for symbol in symbols:
            assert self.symbol_table.symbol_exists(symbol.name) == False
    
    def test_declaration_order(self):
        """Test that symbols are tracked in declaration order."""
        names = ["first", "second", "third", "fourth"]
        
        for name in names:
            symbol = Symbol(name, "string")
            self.symbol_table.declare_symbol(symbol)
        
        order = self.symbol_table.get_declaration_order()
        assert order == names
        
        # Remove middle symbol
        self.symbol_table.remove_symbol("second")
        
        order_after_remove = self.symbol_table.get_declaration_order()
        assert order_after_remove == ["first", "third", "fourth"]
    
    def test_string_representation(self):
        """Test string representation of populated table."""
        symbols = [
            Symbol("str_var", "string"),
            Symbol("list_var", "list", "num")
        ]
        
        for symbol in symbols:
            self.symbol_table.declare_symbol(symbol)
        
        table_str = str(self.symbol_table)
        assert "SymbolTable:" in table_str
        assert "string str_var" in table_str
        assert "list<num> list_var" in table_str
    
    def test_table_copy_operations(self):
        """Test that get_all_symbols returns a copy."""
        symbol = Symbol("test", "string")
        self.symbol_table.declare_symbol(symbol)
        
        all_symbols = self.symbol_table.get_all_symbols()
        
        # Modify returned dict shouldn't affect original
        all_symbols["new_symbol"] = Symbol("new", "num")
        
        assert self.symbol_table.size() == 1  # Original unchanged
        assert not self.symbol_table.symbol_exists("new_symbol")


if __name__ == '__main__':
    pytest.main([__file__])
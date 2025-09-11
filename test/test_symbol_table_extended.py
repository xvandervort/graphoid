"""Additional tests for symbol table functionality."""

import pytest
from src.glang.semantic.symbol_table import Symbol, SymbolTable
from src.glang.ast.nodes import SourcePosition


class TestSymbolExtended:
    """Extended tests for Symbol class."""
    
    def test_symbol_string_representation(self):
        """Test symbol string representation."""
        position = SourcePosition(1, 5)
        symbol = Symbol("test_var", "string", position=position)
        
        str_repr = str(symbol)
        assert "string" in str_repr
        assert "test_var" in str_repr
        assert "at line 1, column 5" in str_repr
    
    def test_symbol_with_constraint_string_representation(self):
        """Test symbol with constraint string representation."""
        position = SourcePosition(2, 10)
        symbol = Symbol("items", "list", "num", position)
        
        str_repr = str(symbol)
        assert "list<num>" in str_repr
        assert "items" in str_repr
        assert "at line 2, column 10" in str_repr
    
    def test_symbol_without_position(self):
        """Test symbol string representation without position."""
        symbol = Symbol("var", "number")
        str_repr = str(symbol)
        assert "number var" == str_repr
    
    def test_symbol_constraint_methods(self):
        """Test symbol constraint checking methods."""
        # Symbol without constraint
        symbol_no_constraint = Symbol("var1", "string")
        assert not symbol_no_constraint.has_constraint()
        assert symbol_no_constraint.matches_constraint("any_type") is True
        
        # Symbol with constraint
        symbol_with_constraint = Symbol("var2", "list", "num")
        assert symbol_with_constraint.has_constraint() is True
        assert symbol_with_constraint.matches_constraint("num") is True
        assert symbol_with_constraint.matches_constraint("string") is False


class TestSymbolTableExtended:
    """Extended tests for SymbolTable class."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.symbol_table = SymbolTable()
    
    def test_declare_multiple_symbols(self):
        """Test declaring multiple symbols."""
        pos1 = SourcePosition(1, 1)
        pos2 = SourcePosition(2, 1)
        
        symbol1 = Symbol("var1", "string", position=pos1)
        symbol2 = Symbol("var2", "number", position=pos2)
        
        self.symbol_table.declare_symbol(symbol1)
        self.symbol_table.declare_symbol(symbol2)
        
        assert self.symbol_table.size() == 2
        assert self.symbol_table.symbol_exists("var1")
        assert self.symbol_table.symbol_exists("var2")
    
    def test_get_symbols_by_type(self):
        """Test getting symbols by type."""
        symbols = [
            Symbol("str1", "string"),
            Symbol("str2", "string"),
            Symbol("num1", "number"),
            Symbol("bool1", "bool"),
        ]
        
        for symbol in symbols:
            self.symbol_table.declare_symbol(symbol)
        
        string_symbols = self.symbol_table.get_symbols_by_type("string")
        assert len(string_symbols) == 2
        assert all(s.symbol_type == "string" for s in string_symbols)
        
        number_symbols = self.symbol_table.get_symbols_by_type("number")
        assert len(number_symbols) == 1
        assert number_symbols[0].name == "num1"
        
        list_symbols = self.symbol_table.get_symbols_by_type("list")
        assert len(list_symbols) == 0
    
    def test_get_symbols_with_constraint(self):
        """Test getting symbols with specific constraint."""
        symbols = [
            Symbol("list1", "list", "num"),
            Symbol("list2", "list", "string"), 
            Symbol("list3", "list", "num"),
            Symbol("list4", "list"),  # No constraint
        ]
        
        for symbol in symbols:
            self.symbol_table.declare_symbol(symbol)
        
        num_constrained = self.symbol_table.get_symbols_with_constraint("num")
        assert len(num_constrained) == 2
        assert all(s.type_constraint == "num" for s in num_constrained)
        
        string_constrained = self.symbol_table.get_symbols_with_constraint("string")
        assert len(string_constrained) == 1
        assert string_constrained[0].name == "list2"
        
        bool_constrained = self.symbol_table.get_symbols_with_constraint("bool")
        assert len(bool_constrained) == 0
    
    def test_remove_symbol(self):
        """Test removing symbols from the table."""
        symbol = Symbol("temp_var", "string")
        self.symbol_table.declare_symbol(symbol)
        
        assert self.symbol_table.symbol_exists("temp_var")
        assert self.symbol_table.size() == 1
        
        # Remove existing symbol
        result = self.symbol_table.remove_symbol("temp_var")
        assert result is True
        assert not self.symbol_table.symbol_exists("temp_var")
        assert self.symbol_table.size() == 0
        
        # Try to remove non-existent symbol
        result = self.symbol_table.remove_symbol("nonexistent")
        assert result is False
    
    def test_remove_symbol_preserves_declaration_order(self):
        """Test that removing symbols preserves declaration order."""
        symbols = [
            Symbol("var1", "string"),
            Symbol("var2", "number"),
            Symbol("var3", "bool"),
        ]
        
        for symbol in symbols:
            self.symbol_table.declare_symbol(symbol)
        
        # Remove middle symbol
        self.symbol_table.remove_symbol("var2")
        
        order = self.symbol_table.get_declaration_order()
        assert order == ["var1", "var3"]
    
    def test_clear_symbol_table(self):
        """Test clearing all symbols."""
        symbols = [Symbol(f"var{i}", "string") for i in range(5)]
        for symbol in symbols:
            self.symbol_table.declare_symbol(symbol)
        
        assert self.symbol_table.size() == 5
        
        self.symbol_table.clear()
        assert self.symbol_table.size() == 0
        assert len(self.symbol_table.get_declaration_order()) == 0
        assert len(self.symbol_table.get_all_symbols()) == 0
    
    def test_get_all_symbols_returns_copy(self):
        """Test that get_all_symbols returns a copy."""
        symbol = Symbol("test", "string")
        self.symbol_table.declare_symbol(symbol)
        
        all_symbols = self.symbol_table.get_all_symbols()
        
        # Modify the returned dict
        all_symbols["new_symbol"] = Symbol("new", "number")
        
        # Original table should be unchanged
        assert not self.symbol_table.symbol_exists("new_symbol")
        assert self.symbol_table.size() == 1
    
    def test_get_declaration_order_returns_copy(self):
        """Test that get_declaration_order returns a copy."""
        symbols = [Symbol(f"var{i}", "string") for i in range(3)]
        for symbol in symbols:
            self.symbol_table.declare_symbol(symbol)
        
        order = self.symbol_table.get_declaration_order()
        
        # Modify the returned list
        order.append("new_var")
        
        # Original order should be unchanged
        original_order = self.symbol_table.get_declaration_order()
        assert len(original_order) == 3
        assert "new_var" not in original_order
    
    def test_symbol_table_string_representation(self):
        """Test symbol table string representation."""
        # Empty table
        empty_repr = str(self.symbol_table)
        assert empty_repr == "SymbolTable: (empty)"
        
        # Table with symbols
        pos1 = SourcePosition(1, 5)
        pos2 = SourcePosition(2, 10)
        
        symbol1 = Symbol("var1", "string", position=pos1)
        symbol2 = Symbol("var2", "list", "num", position=pos2)
        
        self.symbol_table.declare_symbol(symbol1)
        self.symbol_table.declare_symbol(symbol2)
        
        table_repr = str(self.symbol_table)
        assert "SymbolTable:" in table_repr
        assert "string var1" in table_repr
        assert "list<num> var2" in table_repr
    
    def test_symbol_table_repr(self):
        """Test symbol table __repr__ method."""
        symbol = Symbol("test", "string")
        self.symbol_table.declare_symbol(symbol)
        
        repr_str = repr(self.symbol_table)
        assert repr_str == "SymbolTable(symbols=1)"
        
        # Add more symbols
        for i in range(4):
            self.symbol_table.declare_symbol(Symbol(f"var{i}", "number"))
        
        repr_str = repr(self.symbol_table)
        assert repr_str == "SymbolTable(symbols=5)"
    
    def test_redeclaration_error_preserves_original_position(self):
        """Test that redeclaration error includes original position."""
        pos1 = SourcePosition(1, 5)
        pos2 = SourcePosition(3, 10)
        
        symbol1 = Symbol("duplicate", "string", position=pos1)
        symbol2 = Symbol("duplicate", "number", position=pos2)
        
        self.symbol_table.declare_symbol(symbol1)
        
        with pytest.raises(ValueError) as exc_info:
            self.symbol_table.declare_symbol(symbol2)
        
        error_message = str(exc_info.value)
        assert "duplicate" in error_message
        assert "already declared" in error_message
        assert "line 1, column 5" in error_message
    
    def test_lookup_nonexistent_symbol(self):
        """Test looking up non-existent symbol returns None."""
        result = self.symbol_table.lookup_symbol("nonexistent")
        assert result is None
    
    def test_lookup_existing_symbol(self):
        """Test looking up existing symbol returns correct symbol."""
        original_symbol = Symbol("existing", "bool")
        self.symbol_table.declare_symbol(original_symbol)
        
        found_symbol = self.symbol_table.lookup_symbol("existing")
        assert found_symbol is original_symbol
        assert found_symbol.name == "existing"
        assert found_symbol.symbol_type == "bool"


class TestSymbolTableDeclarationOrder:
    """Test symbol table declaration order functionality."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.symbol_table = SymbolTable()
    
    def test_declaration_order_preservation(self):
        """Test that declaration order is preserved correctly."""
        names = ["first", "second", "third", "fourth"]
        
        for name in names:
            symbol = Symbol(name, "string")
            self.symbol_table.declare_symbol(symbol)
        
        order = self.symbol_table.get_declaration_order()
        assert order == names
    
    def test_declaration_order_after_removal(self):
        """Test declaration order after removing symbols."""
        names = ["a", "b", "c", "d", "e"]
        
        for name in names:
            symbol = Symbol(name, "string")
            self.symbol_table.declare_symbol(symbol)
        
        # Remove some symbols
        self.symbol_table.remove_symbol("b")
        self.symbol_table.remove_symbol("d")
        
        order = self.symbol_table.get_declaration_order()
        assert order == ["a", "c", "e"]
    
    def test_declaration_order_in_string_representation(self):
        """Test that string representation follows declaration order."""
        names = ["third", "first", "second"]  # Alphabetically out of order
        
        for name in names:
            symbol = Symbol(name, "number")
            self.symbol_table.declare_symbol(symbol)
        
        table_str = str(self.symbol_table)
        
        # Find positions of each variable name in the string
        positions = {}
        for name in names:
            pos = table_str.find(f"number {name}")
            assert pos != -1  # Make sure it was found
            positions[name] = pos
        
        # Should appear in declaration order, not alphabetical
        assert positions["third"] < positions["first"] < positions["second"]
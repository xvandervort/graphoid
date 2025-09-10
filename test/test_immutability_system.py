#!/usr/bin/env python3
"""Test the data immutability system with freeze() and contamination rules."""

import pytest
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../src'))

from glang.execution.values import (
    StringValue, NumberValue, BooleanValue, ListValue, DataValue, MapValue
)
from glang.execution.executor import ASTExecutor, ExecutionContext
from glang.semantic.analyzer import SemanticAnalyzer
from glang.parser.ast_parser import ASTParser
from glang.semantic.symbol_table import SymbolTable


class TestBasicFreezingBehavior:
    """Test basic freezing and immutability behavior."""
    
    def test_string_freezing(self):
        """Test that strings can be frozen and become immutable."""
        s = StringValue("hello")
        assert not s.is_frozen_value()
        assert not s.contains_frozen_data()
        
        # Freeze the string
        result = s.freeze()
        assert result is s  # freeze() returns self for chaining
        assert s.is_frozen_value()
        assert s.contains_frozen_data()
    
    def test_number_freezing(self):
        """Test that numbers can be frozen and become immutable."""
        n = NumberValue(42)
        assert not n.is_frozen_value()
        
        n.freeze()
        assert n.is_frozen_value()
        assert n.contains_frozen_data()
    
    def test_boolean_freezing(self):
        """Test that booleans can be frozen and become immutable."""
        b = BooleanValue(True)
        assert not b.is_frozen_value()
        
        b.freeze()
        assert b.is_frozen_value()
        assert b.contains_frozen_data()


class TestListFreezingAndContamination:
    """Test list freezing and contamination rules."""
    
    def test_empty_list_freezing(self):
        """Test that empty lists can be frozen."""
        lst = ListValue([])
        assert not lst.is_frozen_value()
        assert not lst.contains_frozen_data()
        
        lst.freeze()
        assert lst.is_frozen_value()
        assert lst.contains_frozen_data()
    
    def test_list_deep_freezing(self):
        """Test that freezing a list also freezes its elements."""
        s1 = StringValue("hello")
        s2 = StringValue("world")
        lst = ListValue([s1, s2])
        
        assert not s1.is_frozen_value()
        assert not s2.is_frozen_value()
        assert not lst.contains_frozen_data()
        
        lst.freeze()
        assert lst.is_frozen_value()
        assert s1.is_frozen_value()  # Deep freeze
        assert s2.is_frozen_value()  # Deep freeze
        assert lst.contains_frozen_data()
    
    def test_frozen_list_rejects_mutations(self):
        """Test that frozen lists reject mutations."""
        lst = ListValue([StringValue("hello")])
        lst.freeze()
        
        with pytest.raises(RuntimeError, match="Cannot append: value is frozen"):
            lst.append(StringValue("world"))
    
    def test_list_contamination_detection(self):
        """Test that lists detect contamination from frozen elements."""
        frozen_string = StringValue("frozen")
        frozen_string.freeze()
        
        lst = ListValue([frozen_string, StringValue("unfrozen")])
        assert lst.contains_frozen_data()
    
    def test_contamination_prevents_mixing(self):
        """Test that contamination prevents mixing frozen and unfrozen data."""
        # Start with unfrozen list
        lst = ListValue([StringValue("unfrozen")])
        assert not lst.contains_frozen_data()
        
        # Try to add a frozen element
        frozen_string = StringValue("frozen")
        frozen_string.freeze()
        
        with pytest.raises(RuntimeError, match="cannot mix frozen and unfrozen data"):
            lst.append(frozen_string)
    
    def test_can_accept_element_method(self):
        """Test the can_accept_element method."""
        lst = ListValue([StringValue("unfrozen")])
        
        # Should accept unfrozen element
        can_accept, msg = lst.can_accept_element(StringValue("another"))
        assert can_accept
        assert msg == ""
        
        # Should reject frozen element due to contamination
        frozen_string = StringValue("frozen")
        frozen_string.freeze()
        can_accept, msg = lst.can_accept_element(frozen_string)
        assert not can_accept
        assert "cannot mix frozen and unfrozen data" in msg.lower()


class TestDataNodeFreezingAndContamination:
    """Test data node freezing and contamination rules."""
    
    def test_data_node_freezing(self):
        """Test that data nodes can be frozen."""
        data = DataValue("name", StringValue("Alice"))
        assert not data.is_frozen_value()
        assert not data.contains_frozen_data()
        
        data.freeze()
        assert data.is_frozen_value()
        assert data.contains_frozen_data()
        
        # The value should also be frozen (deep freeze)
        assert data.value.is_frozen_value()
    
    def test_data_node_contamination_from_value(self):
        """Test that data nodes detect contamination from frozen values."""
        frozen_value = StringValue("frozen")
        frozen_value.freeze()
        
        data = DataValue("key", frozen_value)
        assert data.contains_frozen_data()
    
    def test_frozen_data_node_rejects_mutations(self):
        """Test that frozen data nodes reject value changes."""
        data = DataValue("name", StringValue("Alice"))
        data.freeze()
        
        with pytest.raises(RuntimeError, match="Cannot set value: value is frozen"):
            data.set_value(StringValue("Bob"))
    
    def test_data_node_contamination_prevents_mixing(self):
        """Test that data nodes prevent mixing frozen and unfrozen values."""
        # Start with unfrozen data node
        data = DataValue("name", StringValue("Alice"))
        assert not data.contains_frozen_data()
        
        # Try to set a frozen value
        frozen_value = StringValue("frozen")
        frozen_value.freeze()
        
        with pytest.raises(RuntimeError, match="cannot mix frozen and unfrozen data"):
            data.set_value(frozen_value)
    
    def test_can_accept_value_method(self):
        """Test the can_accept_value method for data nodes."""
        data = DataValue("name", StringValue("Alice"))
        
        # Should accept unfrozen value
        can_accept, msg = data.can_accept_value(StringValue("Bob"))
        assert can_accept
        assert msg == ""
        
        # Should reject frozen value due to contamination
        frozen_value = StringValue("frozen")
        frozen_value.freeze()
        can_accept, msg = data.can_accept_value(frozen_value)
        assert not can_accept
        assert "cannot mix frozen and unfrozen data" in msg.lower()


class TestMapFreezingAndContamination:
    """Test map freezing and contamination rules."""
    
    def test_map_freezing(self):
        """Test that maps can be frozen."""
        pairs = [("name", StringValue("Alice")), ("age", NumberValue(25))]
        map_val = MapValue(pairs)
        assert not map_val.is_frozen_value()
        assert not map_val.contains_frozen_data()
        
        map_val.freeze()
        assert map_val.is_frozen_value()
        assert map_val.contains_frozen_data()
        
        # All values should be frozen (deep freeze)
        for value in map_val.pairs.values():
            assert value.is_frozen_value()
    
    def test_map_contamination_from_values(self):
        """Test that maps detect contamination from frozen values."""
        frozen_value = StringValue("frozen")
        frozen_value.freeze()
        
        pairs = [("key1", StringValue("normal")), ("key2", frozen_value)]
        map_val = MapValue(pairs)
        assert map_val.contains_frozen_data()
    
    def test_frozen_map_rejects_mutations(self):
        """Test that frozen maps reject mutations."""
        pairs = [("name", StringValue("Alice"))]
        map_val = MapValue(pairs)
        map_val.freeze()
        
        with pytest.raises(RuntimeError, match="Cannot set key: value is frozen"):
            map_val.set("age", NumberValue(25))
        
        with pytest.raises(RuntimeError, match="Cannot remove key: value is frozen"):
            map_val.remove("name")
    
    def test_map_contamination_prevents_mixing(self):
        """Test that maps prevent mixing frozen and unfrozen values."""
        # Start with unfrozen map
        pairs = [("name", StringValue("Alice"))]
        map_val = MapValue(pairs)
        assert not map_val.contains_frozen_data()
        
        # Try to add a frozen value
        frozen_value = StringValue("frozen")
        frozen_value.freeze()
        
        with pytest.raises(RuntimeError, match="cannot mix frozen and unfrozen data"):
            map_val.set("frozen_key", frozen_value)
    
    def test_can_accept_value_method_for_maps(self):
        """Test the can_accept_value method for maps."""
        pairs = [("name", StringValue("Alice"))]
        map_val = MapValue(pairs)
        
        # Should accept unfrozen value
        can_accept, msg = map_val.can_accept_value(StringValue("Bob"))
        assert can_accept
        assert msg == ""
        
        # Should reject frozen value due to contamination
        frozen_value = StringValue("frozen")
        frozen_value.freeze()
        can_accept, msg = map_val.can_accept_value(frozen_value)
        assert not can_accept
        assert "cannot mix frozen and unfrozen data" in msg.lower()


class TestIntegratedImmutabilityWithExecutor:
    """Test immutability system with the full executor."""
    
    def test_freeze_method_call(self):
        """Test calling freeze() method through the executor."""
        parser = ASTParser()
        analyzer = SemanticAnalyzer()
        symbol_table = SymbolTable()
        context = ExecutionContext(symbol_table)
        executor = ASTExecutor(context)
        
        # Step 1: Declare variable
        ast1 = parser.parse('string name = "Alice"')
        analyzer.analyze(ast1)
        executor.execute(ast1)
        
        # Step 2: Call freeze method
        ast2 = parser.parse('name.freeze()')
        # Skip semantic analysis since it creates a fresh symbol table
        executor.execute(ast2)
        
        # Get the variable from the execution context
        name_value = context.get_variable("name")
        assert name_value.is_frozen_value()
    
    def test_is_frozen_method_call(self):
        """Test calling is_frozen() method through the executor.""" 
        parser = ASTParser()
        analyzer = SemanticAnalyzer()
        symbol_table = SymbolTable()
        context = ExecutionContext(symbol_table)
        executor = ASTExecutor(context)
        
        # Step 1: Declare variable
        ast1 = parser.parse('string name = "Alice"')
        analyzer.analyze(ast1)
        executor.execute(ast1)
        
        # Step 2: Check is_frozen before freezing
        ast2 = parser.parse('bool frozen_before = name.is_frozen()')
        analyzer.analyze(ast2)  # This will have the bool declaration
        executor.execute(ast2)
        
        # Step 3: Freeze the variable
        ast3 = parser.parse('name.freeze()')
        executor.execute(ast3)
        
        # Step 4: Check is_frozen after freezing
        ast4 = parser.parse('bool frozen_after = name.is_frozen()')
        analyzer.analyze(ast4)  # This will have the bool declaration
        executor.execute(ast4)
        
        frozen_before = context.get_variable("frozen_before")
        frozen_after = context.get_variable("frozen_after")
        assert frozen_before is not None
        assert frozen_after is not None
        assert not frozen_before.value
        assert frozen_after.value
    
    def test_contains_frozen_method_call(self):
        """Test calling contains_frozen() method through the executor."""
        parser = ASTParser()
        analyzer = SemanticAnalyzer()
        symbol_table = SymbolTable()
        context = ExecutionContext(symbol_table)
        executor = ASTExecutor(context)
        
        # Step 1: Declare list
        ast1 = parser.parse('list items = ["hello", "world"]')
        analyzer.analyze(ast1)
        executor.execute(ast1)
        
        # Step 2: Check contains_frozen before freezing
        ast2 = parser.parse('bool has_frozen_before = items.contains_frozen()')
        analyzer.analyze(ast2)
        executor.execute(ast2)
        
        # Step 3: Freeze the list
        ast3 = parser.parse('items.freeze()')
        executor.execute(ast3)
        
        # Step 4: Check contains_frozen after freezing
        ast4 = parser.parse('bool has_frozen_after = items.contains_frozen()')
        analyzer.analyze(ast4)
        executor.execute(ast4)
        
        has_frozen_before = context.get_variable("has_frozen_before")
        has_frozen_after = context.get_variable("has_frozen_after")
        assert has_frozen_before is not None
        assert has_frozen_after is not None
        assert not has_frozen_before.value
        assert has_frozen_after.value
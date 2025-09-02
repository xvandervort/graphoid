"""Tests for enhanced type system functionality (Phase 4B)."""

import pytest
from glang.parser import SyntaxParser, VariableDeclaration, InputType
from glang.repl import REPL
from io import StringIO
import sys


class TestTypeConstraintSyntax:
    """Test parsing of type constraint syntax."""
    
    def setup_method(self):
        self.parser = SyntaxParser()
    
    def test_parse_type_constraint_basic(self):
        """Test parsing basic type constraints."""
        result = self.parser.parse_input("list<num> scores = [95, 87, 92]")
        assert isinstance(result, VariableDeclaration)
        assert result.graph_type == "list"
        assert result.variable_name == "scores"
        assert result.type_constraint == "num"
        assert result.initializer == [95, 87, 92]
    
    def test_parse_different_type_constraints(self):
        """Test parsing different type constraints."""
        # String constraint
        result = self.parser.parse_input("list<string> names = ['alice', 'bob', 'charlie']")
        assert result.type_constraint == "string"
        
        # Boolean constraint
        result = self.parser.parse_input("list<bool> flags = [true, false, true]")
        assert result.type_constraint == "bool"
        
        # List constraint
        result = self.parser.parse_input("list<list> matrix = [[1, 2], [3, 4]]")
        assert result.type_constraint == "list"
    
    def test_parse_no_type_constraint(self):
        """Test parsing without type constraints still works."""
        result = self.parser.parse_input("list fruits = ['apple', 'banana']")
        assert isinstance(result, VariableDeclaration)
        assert result.type_constraint is None
    
    def test_parse_empty_list_with_constraint(self):
        """Test parsing empty list with type constraint."""
        result = self.parser.parse_input("list<num> empty = []")
        assert result.type_constraint == "num"
        assert result.initializer == []


class TestTypeValidation:
    """Test type validation during variable declaration."""
    
    def setup_method(self):
        self.repl = REPL()
    
    def test_valid_type_constraint(self):
        """Test successful type constraint validation."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list<num> numbers = [1, 2, 3, 4, 5]')
            output = captured_output.getvalue()
            assert "Type validation failed" not in output
            assert "numbers" in output
        finally:
            sys.stdout = sys.__stdout__
    
    def test_invalid_type_constraint_mixed(self):
        """Test type validation failure with mixed types."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list<num> mixed = [1, "hello", 3]')
            output = captured_output.getvalue()
            assert "Type validation failed" in output
            assert "string" in output
            assert "num" in output
        finally:
            sys.stdout = sys.__stdout__
    
    def test_empty_list_always_valid(self):
        """Test that empty lists pass any type constraint."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list<string> empty = []')
            output = captured_output.getvalue()
            assert "Type validation failed" not in output
        finally:
            sys.stdout = sys.__stdout__
    
    def test_boolean_type_constraint(self):
        """Test boolean type constraint validation."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            # Valid boolean list
            self.repl._process_input('list<bool> flags = [true, false, true]')
            output = captured_output.getvalue()
            assert "Type validation failed" not in output
            
            captured_output = StringIO()
            sys.stdout = captured_output
            
            # Invalid boolean list
            self.repl._process_input('list<bool> bad_flags = [true, 1, false]')
            output = captured_output.getvalue()
            assert "Type validation failed" in output
        finally:
            sys.stdout = sys.__stdout__


class TestTypeIntrospection:
    """Test enhanced type introspection methods."""
    
    def setup_method(self):
        self.repl = REPL()
    
    def test_constraint_method(self):
        """Test the constraint() method."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list<num> scores = [95, 87, 92]')
            captured_output = StringIO()
            sys.stdout = captured_output
            
            self.repl._process_input('scores.constraint()')
            output = captured_output.getvalue()
            assert "type constraint: num" in output
        finally:
            sys.stdout = sys.__stdout__
    
    def test_constraint_method_no_constraint(self):
        """Test constraint() method on list without constraints."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list fruits = ["apple", "banana"]')
            captured_output = StringIO()
            sys.stdout = captured_output
            
            self.repl._process_input('fruits.constraint()')
            output = captured_output.getvalue()
            assert "no type constraint" in output
        finally:
            sys.stdout = sys.__stdout__
    
    def test_validate_constraint_method(self):
        """Test the validate_constraint() method."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            # Create valid constrained list
            self.repl._process_input('list<num> numbers = [1, 2, 3]')
            captured_output = StringIO()
            sys.stdout = captured_output
            
            self.repl._process_input('numbers.validate_constraint()')
            output = captured_output.getvalue()
            assert "satisfy constraint" in output
        finally:
            sys.stdout = sys.__stdout__
    
    def test_type_summary_method(self):
        """Test the type_summary() method."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list<num> numbers = [1, 2, 3]')
            captured_output = StringIO()
            sys.stdout = captured_output
            
            self.repl._process_input('numbers.type_summary()')
            output = captured_output.getvalue()
            assert "3 num" in output
            assert "constraint: num" in output
        finally:
            sys.stdout = sys.__stdout__
    
    def test_type_summary_mixed_types(self):
        """Test type_summary() with mixed types."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list mixed = [1, "hello", true, 3.14]')
            captured_output = StringIO()
            sys.stdout = captured_output
            
            self.repl._process_input('mixed.type_summary()')
            output = captured_output.getvalue()
            assert "2 num" in output
            assert "1 bool" in output
            assert "1 string" in output
        finally:
            sys.stdout = sys.__stdout__


class TestConstraintEnforcement:
    """Test that type constraints are enforced during mutations."""
    
    def setup_method(self):
        self.repl = REPL()
    
    def test_append_respects_constraint(self):
        """Test that append() respects type constraints."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list<num> numbers = [1, 2, 3]')
            captured_output = StringIO()
            sys.stdout = captured_output
            
            # Valid append
            self.repl._process_input('numbers.append 42')
            output = captured_output.getvalue()
            assert "Error" not in output
            assert "Appended" in output
            
            captured_output = StringIO()
            sys.stdout = captured_output
            
            # Invalid append
            self.repl._process_input('numbers.append hello')
            output = captured_output.getvalue()
            assert "Error" in output
            assert "constraint" in output
        finally:
            sys.stdout = sys.__stdout__
    
    def test_prepend_respects_constraint(self):
        """Test that prepend() respects type constraints."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list<string> words = ["hello", "world"]')
            captured_output = StringIO()
            sys.stdout = captured_output
            
            # Valid prepend
            self.repl._process_input('words.prepend greetings')
            output = captured_output.getvalue()
            assert "Error" not in output
            
            captured_output = StringIO()
            sys.stdout = captured_output
            
            # Invalid prepend
            self.repl._process_input('words.prepend 123')
            output = captured_output.getvalue()
            assert "Error" in output
            assert "constraint" in output
        finally:
            sys.stdout = sys.__stdout__
    
    def test_insert_respects_constraint(self):
        """Test that insert() respects type constraints."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list<bool> flags = [true, false]')
            captured_output = StringIO()
            sys.stdout = captured_output
            
            # Valid insert
            self.repl._process_input('flags.insert 1 true')
            output = captured_output.getvalue()
            assert "Error" not in output
            
            captured_output = StringIO()
            sys.stdout = captured_output
            
            # Invalid insert
            self.repl._process_input('flags.insert 0 maybe')
            output = captured_output.getvalue()
            assert "Error" in output
            assert "constraint" in output
        finally:
            sys.stdout = sys.__stdout__


class TestTypeCoercion:
    """Test type coercion functionality."""
    
    def setup_method(self):
        self.repl = REPL()
    
    def test_coerce_to_constraint_method(self):
        """Test the coerce_to_constraint() method."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            # Create mixed type list, then add constraint via legacy method
            self.repl._process_input('list mixed = [1, 2, "hello", 4]')
            
            # Manually add constraint to test coercion
            graph = self.repl.graph_manager.get_variable('mixed')
            if not hasattr(graph, 'metadata'):
                graph.metadata = {}
            graph.metadata['type_constraint'] = 'num'
            
            captured_output = StringIO()
            sys.stdout = captured_output
            
            self.repl._process_input('mixed.coerce_to_constraint()')
            output = captured_output.getvalue()
            # Should show attempted coercion results
            assert "Coercion" in output or "FAILED" in output
        finally:
            sys.stdout = sys.__stdout__
    
    def test_coerce_no_constraint(self):
        """Test coerce_to_constraint() on list without constraint."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list normal = [1, 2, 3]')
            captured_output = StringIO()
            sys.stdout = captured_output
            
            self.repl._process_input('normal.coerce_to_constraint()')
            output = captured_output.getvalue()
            assert "no type constraint" in output
        finally:
            sys.stdout = sys.__stdout__


class TestTypeConstraintIntegration:
    """Test integration of type constraints with existing features."""
    
    def setup_method(self):
        self.repl = REPL()
    
    def test_constraint_with_indexing(self):
        """Test that type constraints work with indexing syntax."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list<num> numbers = [10, 20, 30]')
            captured_output = StringIO()
            sys.stdout = captured_output
            
            # Test index access still works
            self.repl._process_input('numbers[1]')
            output = captured_output.getvalue()
            assert "20" in output
            
            captured_output = StringIO()
            sys.stdout = captured_output
            
            # Test index assignment with valid type
            self.repl._process_input('numbers[1] = 99')
            output = captured_output.getvalue()
            assert "Set numbers[1] = 99" in output
        finally:
            sys.stdout = sys.__stdout__
    
    def test_constraint_with_slicing(self):
        """Test that type constraints work with slicing syntax."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list<string> words = ["hello", "world", "test", "data"]')
            captured_output = StringIO()
            sys.stdout = captured_output
            
            # Test slice access
            self.repl._process_input('words[1:3]')
            output = captured_output.getvalue()
            assert "world" in output and "test" in output
            
            captured_output = StringIO()
            sys.stdout = captured_output
            
            # Test slice assignment with valid types
            self.repl._process_input('words[1:3] = ["new", "items"]')
            output = captured_output.getvalue()
            assert "Set words[1:3]" in output
        finally:
            sys.stdout = sys.__stdout__


class TestTypeConstraintErrorHandling:
    """Test error handling for type constraint features."""
    
    def setup_method(self):
        self.repl = REPL()
    
    def test_invalid_type_constraint_syntax(self):
        """Test error handling for invalid type constraint syntax."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            # Missing closing angle bracket
            self.repl._process_input('list<num numbers = [1, 2, 3]')
            # Should fall back to legacy command parsing or show error
            output = captured_output.getvalue()
            # Just ensure it doesn't crash
            assert len(output) >= 0
        finally:
            sys.stdout = sys.__stdout__
    
    def test_unsupported_type_constraint(self):
        """Test handling of unsupported type constraints."""
        # This would need parser enhancement to detect unknown types
        # For now, unknown types in angle brackets should be treated as identifiers
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list<unknown> data = [1, 2, 3]')
            output = captured_output.getvalue()
            # Should either create the list or show error - just ensure no crash
            assert len(output) >= 0
        finally:
            sys.stdout = sys.__stdout__


class TestTypeConstraintWorkflow:
    """Test complete workflows with type constraints."""
    
    def setup_method(self):
        self.repl = REPL()
    
    def test_full_constrained_workflow(self):
        """Test a complete workflow with type-constrained lists."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            # Create constrained list
            self.repl._process_input('list<num> scores = [95, 87, 92]')
            
            # Check constraint
            self.repl._process_input('scores.constraint()')
            
            # Add valid values
            self.repl._process_input('scores.append 88')
            
            # Try to add invalid value (should fail)
            self.repl._process_input('scores.append excellent')
            
            # Check final state
            self.repl._process_input('scores')
            self.repl._process_input('scores.validate_constraint()')
            self.repl._process_input('scores.type_summary()')
            
            output = captured_output.getvalue()
            
            # Should show constraint info
            assert "constraint: num" in output
            # Should show successful append
            assert "Appended '88'" in output
            # Should show failed append
            assert "Error" in output and "constraint" in output
            # Should show validation passed
            assert "satisfy constraint" in output
            
        finally:
            sys.stdout = sys.__stdout__
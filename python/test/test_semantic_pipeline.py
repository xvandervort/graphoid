"""Tests for semantic pipeline."""

import pytest
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../src'))

from glang.semantic.pipeline import SemanticPipeline, SemanticSession
from glang.semantic.analyzer import AnalysisResult
from glang.semantic.errors import SemanticError, UndefinedVariableError
from glang.ast.nodes import VariableDeclaration, MethodCall


class TestSemanticPipeline:
    """Test semantic pipeline functionality."""
    
    def setup_method(self):
        self.pipeline = SemanticPipeline()
    
    def test_successful_parse_and_analyze(self):
        """Test successful end-to-end pipeline."""
        result = self.pipeline.analyze_code('string greeting = "hello"')
        
        assert isinstance(result, AnalysisResult)
        assert result.success == True
        assert len(result.errors) == 0
        assert result.ast is not None
        assert isinstance(result.ast, VariableDeclaration)
        
        # Check symbol table
        symbol_table = result.symbol_table
        assert symbol_table.symbol_exists("greeting")
        symbol = symbol_table.lookup_symbol("greeting")
        assert symbol.symbol_type == "string"
    
    def test_parse_error_handling(self):
        """Test handling of parse errors."""
        # Invalid syntax that should fail parsing
        result = self.pipeline.analyze_code('string name')
        
        assert isinstance(result, AnalysisResult)
        assert result.success == False
        assert len(result.errors) == 1
        assert "Parse error" in str(result.errors[0])
        assert result.ast is None
    
    def test_semantic_error_handling(self):
        """Test handling of semantic errors."""
        # Valid syntax but semantic error (undefined variable)
        result = self.pipeline.analyze_code('undefined_var')
        
        assert result.success == False
        assert len(result.errors) == 1
        assert isinstance(result.errors[0], UndefinedVariableError)
        assert result.ast is not None  # Parse succeeded, semantic failed
    
    def test_complex_expression_analysis(self):
        """Test analysis of complex expressions."""
        # List declaration with constraint
        result = self.pipeline.analyze_code('list<string> names = ["alice", "bob"]')
        
        assert result.success == True
        assert len(result.errors) == 0
        
        symbol = result.symbol_table.lookup_symbol("names")
        assert symbol.symbol_type == "list"
        assert symbol.type_constraint == "string"
    
    def test_method_call_analysis(self):
        """Test method call analysis through pipeline."""
        # This will fail because the target doesn't exist
        result = self.pipeline.analyze_code('mylist.append("item")')
        
        assert result.success == False
        assert len(result.errors) == 1
        assert isinstance(result.errors[0], UndefinedVariableError)
        assert result.errors[0].variable_name == "mylist"
    
    def test_get_symbol_table(self):
        """Test getting symbol table from pipeline."""
        self.pipeline.analyze_code('num count = 42')
        
        symbol_table = self.pipeline.get_symbol_table()
        assert symbol_table.symbol_exists("count")
        assert symbol_table.lookup_symbol("count").symbol_type == "num"
    
    def test_reset_pipeline(self):
        """Test resetting pipeline state."""
        # Analyze something to populate state
        result1 = self.pipeline.analyze_code('string var1 = "test"')
        assert result1.success
        assert self.pipeline.get_symbol_table().size() == 1
        
        # Reset and verify clean state
        self.pipeline.reset()
        
        result2 = self.pipeline.analyze_code('var1')  # Should be undefined now
        assert not result2.success
        assert isinstance(result2.errors[0], UndefinedVariableError)
    
    def test_multiple_analyses(self):
        """Test multiple independent analyses."""
        # Each analysis should be independent
        result1 = self.pipeline.analyze_code('string text1 = "hello"')
        assert result1.success
        assert result1.symbol_table.symbol_exists("text1")
        
        result2 = self.pipeline.analyze_code('string text2 = "world"')
        assert result2.success
        assert result2.symbol_table.symbol_exists("text2")
        assert not result2.symbol_table.symbol_exists("text1")  # Independent


class TestSemanticSession:
    """Test semantic session functionality."""
    
    def setup_method(self):
        self.session = SemanticSession()
    
    def test_session_persistence(self):
        """Test that symbols persist across statements in a session."""
        # Declare a variable
        result1 = self.session.analyze_statement('string greeting = "hello"')
        assert result1.success
        assert result1.symbol_table.symbol_exists("greeting")
        
        # Reference the variable in next statement
        result2 = self.session.analyze_statement('greeting')
        assert result2.success  # Should not be undefined
        
        # Session symbol table should have the variable
        session_table = self.session.get_symbol_table()
        assert session_table.symbol_exists("greeting")
    
    def test_session_error_handling(self):
        """Test error handling in session context."""
        # Successful statement
        result1 = self.session.analyze_statement('list items = [1, 2, 3]')
        assert result1.success
        
        # Failed statement (shouldn't affect session table)
        result2 = self.session.analyze_statement('undefined_var')
        assert not result2.success
        
        # Session table should still have the successful declaration
        session_table = self.session.get_symbol_table()
        assert session_table.symbol_exists("items")
        assert not session_table.symbol_exists("undefined_var")
    
    def test_session_method_calls(self):
        """Test method calls in session context."""
        # Declare a list
        result1 = self.session.analyze_statement('list fruits = ["apple"]')
        assert result1.success
        
        # Call method on the list
        result2 = self.session.analyze_statement('fruits.append("banana")')
        assert result2.success
        
        # Both should work because fruits is in session context
        session_table = self.session.get_symbol_table()
        assert session_table.symbol_exists("fruits")
    
    def test_session_redeclaration_error(self):
        """Test redeclaration error in session."""
        # Declare a variable
        result1 = self.session.analyze_statement('string name = "alice"')
        assert result1.success
        
        # Try to redeclare the same variable
        result2 = self.session.analyze_statement('num name = 42')
        assert not result2.success
        assert len(result2.errors) == 1
        # Error type might vary depending on implementation
        assert "name" in str(result2.errors[0])
    
    def test_session_multiple_declarations(self):
        """Test multiple variable declarations in session."""
        declarations = [
            'string first = "one"',
            'num second = 2',
            'list<string> third = ["three"]',
            'bool fourth = true'
        ]
        
        for decl in declarations:
            result = self.session.analyze_statement(decl)
            assert result.success
        
        # All variables should be in session
        session_table = self.session.get_symbol_table()
        assert session_table.size() == 4
        assert session_table.symbol_exists("first")
        assert session_table.symbol_exists("second")
        assert session_table.symbol_exists("third")
        assert session_table.symbol_exists("fourth")
        
        # Check types
        assert session_table.lookup_symbol("first").symbol_type == "string"
        assert session_table.lookup_symbol("second").symbol_type == "num"
        assert session_table.lookup_symbol("third").symbol_type == "list"
        assert session_table.lookup_symbol("third").type_constraint == "string"
        assert session_table.lookup_symbol("fourth").symbol_type == "bool"
    
    def test_clear_session(self):
        """Test clearing session state."""
        # Add some variables to session
        self.session.analyze_statement('string var1 = "test"')
        self.session.analyze_statement('num var2 = 42')
        
        assert self.session.get_symbol_table().size() == 2
        
        # Clear session
        self.session.clear_session()
        
        assert self.session.get_symbol_table().size() == 0
        
        # Variables should now be undefined
        result = self.session.analyze_statement('var1')
        assert not result.success
        assert isinstance(result.errors[0], UndefinedVariableError)
    
    def test_session_complex_interactions(self):
        """Test complex interactions in session."""
        # Declare list
        result1 = self.session.analyze_statement('list<num> numbers = [1, 2, 3]')
        assert result1.success
        
        # Access element
        result2 = self.session.analyze_statement('numbers[0]')
        assert result2.success
        
        # Method call
        result3 = self.session.analyze_statement('numbers.append(4)')
        assert result3.success
        
        # Assignment
        result4 = self.session.analyze_statement('numbers[1] = 99')
        assert result4.success
        
        # All should work with the same list variable
        session_table = self.session.get_symbol_table()
        assert session_table.size() == 1
        symbol = session_table.lookup_symbol("numbers")
        assert symbol.symbol_type == "list"
        assert symbol.type_constraint == "num"
    
    def test_session_isolation(self):
        """Test that different sessions are isolated."""
        session2 = SemanticSession()
        
        # Add variable to first session
        self.session.analyze_statement('string session1_var = "test"')
        assert self.session.get_symbol_table().size() == 1
        
        # Second session should be empty
        assert session2.get_symbol_table().size() == 0
        
        # Add variable to second session
        session2.analyze_statement('num session2_var = 42')
        assert session2.get_symbol_table().size() == 1
        
        # Sessions should remain separate
        assert self.session.get_symbol_table().size() == 1
        assert not self.session.get_symbol_table().symbol_exists("session2_var")
        assert not session2.get_symbol_table().symbol_exists("session1_var")


if __name__ == '__main__':
    pytest.main([__file__])
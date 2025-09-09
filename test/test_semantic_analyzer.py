"""Tests for semantic analyzer."""

import pytest
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../src'))

from glang.semantic.analyzer import SemanticAnalyzer, AnalysisResult
from glang.semantic.errors import *
from glang.semantic.symbol_table import Symbol
from glang.parser.ast_parser import ASTParser, ParseError
from glang.ast.nodes import *


class TestSemanticAnalyzer:
    """Test semantic analyzer functionality."""
    
    def setup_method(self):
        self.analyzer = SemanticAnalyzer()
        self.parser = ASTParser()
    
    def analyze_code(self, code: str) -> AnalysisResult:
        """Helper to parse and analyze code."""
        ast = self.parser.parse(code)
        return self.analyzer.analyze(ast)
    
    def test_empty_analysis(self):
        """Test analyzing empty or simple expressions."""
        # Simple variable access (undefined)
        result = self.analyze_code('myvar')
        
        assert not result.success
        assert len(result.errors) == 1
        assert isinstance(result.errors[0], UndefinedVariableError)
    
    def test_variable_declaration_success(self):
        """Test successful variable declarations."""
        # Simple declaration
        result = self.analyze_code('string name = "hello"')
        
        assert result.success
        assert len(result.errors) == 0
        assert result.symbol_table.symbol_exists("name")
        
        symbol = result.symbol_table.lookup_symbol("name")
        assert symbol.symbol_type == "string"
        assert symbol.type_constraint is None
    
    def test_constrained_declaration_success(self):
        """Test successful constrained variable declarations."""
        result = self.analyze_code('list<num> numbers = [1, 2, 3]')
        
        assert result.success
        assert len(result.errors) == 0
        assert result.symbol_table.symbol_exists("numbers")
        
        symbol = result.symbol_table.lookup_symbol("numbers")
        assert symbol.symbol_type == "list"
        assert symbol.type_constraint == "num"
    
    def test_invalid_type_error(self):
        """Test that invalid types are caught by parser, not semantic analyzer."""
        # The parser should catch invalid types, so this should raise ParseError
        with pytest.raises(ParseError):
            self.analyze_code('invalid_type var = "value"')
        
        # But let's test semantic analysis directly with an invalid AST node
        invalid_decl = VariableDeclaration(
            var_type="invalid_type",
            name="var",
            initializer=StringLiteral("value"),
            position=SourcePosition(1, 1)
        )
        
        result = self.analyzer.analyze(invalid_decl)
        assert not result.success
        assert len(result.errors) == 1
        assert isinstance(result.errors[0], InvalidTypeError)
        assert "invalid_type" in str(result.errors[0])
    
    def test_invalid_constraint_error(self):
        """Test invalid constraint errors."""
        # Invalid constraint type
        result = self.analyze_code('list<invalid> items = []')
        
        assert not result.success
        assert len(result.errors) == 1
        assert isinstance(result.errors[0], InvalidConstraintError)
        
        # Constraint on non-list type
        result = self.analyze_code('string<num> text = "hello"')
        
        assert not result.success
        assert len(result.errors) == 1
        assert isinstance(result.errors[0], InvalidConstraintError)
    
    def test_redeclaration_error(self):
        """Test redeclaration errors with direct analyzer testing."""
        # First declare a symbol directly in analyzer
        symbol1 = Symbol("duplicate", "string", None, SourcePosition(1, 1))
        self.analyzer.symbol_table.declare_symbol(symbol1)
        
        # Now try to analyze a redeclaration via AST
        duplicate_decl = VariableDeclaration(
            var_type="num",
            name="duplicate", 
            initializer=NumberLiteral(42),
            position=SourcePosition(2, 1)
        )
        
        result = self.analyzer.analyze(duplicate_decl, clear_state=False)
        
        assert not result.success
        assert len(result.errors) == 1
        assert isinstance(result.errors[0], RedeclarationError)
        assert "duplicate" in str(result.errors[0])
    
    def test_undefined_variable_reference(self):
        """Test undefined variable reference error."""
        result = self.analyze_code('undefined_var')
        
        assert not result.success
        assert len(result.errors) == 1
        assert isinstance(result.errors[0], UndefinedVariableError)
        assert result.errors[0].variable_name == "undefined_var"
    
    def test_valid_variable_reference(self):
        """Test valid variable reference with direct AST construction."""
        # First declare a symbol directly
        symbol = Symbol("greeting", "string", None, SourcePosition(1, 1))
        self.analyzer.symbol_table.declare_symbol(symbol)
        
        # Create variable reference AST
        var_ref = ExpressionStatement(VariableRef("greeting", SourcePosition(2, 1)))
        
        result = self.analyzer.analyze(var_ref, clear_state=False)
        
        assert result.success
        assert len(result.errors) == 0
    
    def test_method_call_on_valid_target(self):
        """Test method call on valid target."""
        # Declare a list symbol directly
        symbol = Symbol("items", "list", None, SourcePosition(1, 1))
        self.analyzer.symbol_table.declare_symbol(symbol)
        
        # Create method call AST
        method_call = MethodCall(
            target=VariableRef("items", SourcePosition(2, 1)),
            method_name="append",
            arguments=[StringLiteral("c", SourcePosition(2, 15))],
            position=SourcePosition(2, 1)
        )
        
        result = self.analyzer.analyze(method_call, clear_state=False)
        
        assert result.success
        assert len(result.errors) == 0
    
    def test_method_call_on_undefined_target(self):
        """Test method call on undefined target."""
        result = self.analyze_code('undefined.append("value")')
        
        assert not result.success
        assert len(result.errors) == 1
        assert isinstance(result.errors[0], UndefinedVariableError)
        assert result.errors[0].variable_name == "undefined"
    
    def test_invalid_method_call(self):
        """Test invalid method call on wrong type."""
        # Declare a string symbol directly
        symbol = Symbol("text", "string", None, SourcePosition(1, 1))
        self.analyzer.symbol_table.declare_symbol(symbol)
        
        # Create invalid method call AST (append on string)
        method_call = MethodCall(
            target=VariableRef("text", SourcePosition(2, 1)),
            method_name="append",
            arguments=[StringLiteral("world", SourcePosition(2, 18))],
            position=SourcePosition(2, 1)
        )
        
        result = self.analyzer.analyze(method_call, clear_state=False)
        
        assert not result.success
        assert len(result.errors) == 1
        assert isinstance(result.errors[0], InvalidMethodCallError)
        assert "append" in str(result.errors[0])
        assert "string" in str(result.errors[0])
    
    def test_index_access_validation(self):
        """Test index access validation."""
        # Declare a list symbol directly
        symbol = Symbol("items", "list", None, SourcePosition(1, 1))
        self.analyzer.symbol_table.declare_symbol(symbol)
        
        # Create index access AST
        index_access = ExpressionStatement(
            IndexAccess(
                target=VariableRef("items", SourcePosition(2, 1)),
                indices=[NumberLiteral(0, SourcePosition(2, 7))],
                position=SourcePosition(2, 1)
            )
        )
        
        result = self.analyzer.analyze(index_access, clear_state=False)
        
        assert result.success
        assert len(result.errors) == 0
    
    def test_invalid_index_access(self):
        """Test invalid index access on non-indexable type."""
        # Declare a number symbol directly
        symbol = Symbol("value", "num", None, SourcePosition(1, 1))
        self.analyzer.symbol_table.declare_symbol(symbol)
        
        # Create invalid index access AST
        index_access = ExpressionStatement(
            IndexAccess(
                target=VariableRef("value", SourcePosition(2, 1)),
                indices=[NumberLiteral(0, SourcePosition(2, 7))],
                position=SourcePosition(2, 1)
            )
        )
        
        result = self.analyzer.analyze(index_access, clear_state=False)
        
        assert not result.success
        assert len(result.errors) == 1
        assert isinstance(result.errors[0], InvalidMethodCallError)
        assert "not indexable" in str(result.errors[0])
    
    def test_slice_access_validation(self):
        """Test slice access validation."""
        # Declare a list symbol directly
        symbol = Symbol("data", "list", None, SourcePosition(1, 1))
        self.analyzer.symbol_table.declare_symbol(symbol)
        
        # Create slice access AST
        slice_access = ExpressionStatement(
            SliceAccess(
                target=VariableRef("data", SourcePosition(2, 1)),
                start=NumberLiteral(1, SourcePosition(2, 6)),
                stop=NumberLiteral(3, SourcePosition(2, 8)),
                step=None,
                position=SourcePosition(2, 1)
            )
        )
        
        result = self.analyzer.analyze(slice_access, clear_state=False)
        
        assert result.success
        assert len(result.errors) == 0
    
    def test_invalid_slice_access(self):
        """Test invalid slice access on non-sliceable type."""
        # Declare a boolean symbol directly
        symbol = Symbol("flag", "bool", None, SourcePosition(1, 1))
        self.analyzer.symbol_table.declare_symbol(symbol)
        
        # Create invalid slice access AST  
        slice_access = ExpressionStatement(
            SliceAccess(
                target=VariableRef("flag", SourcePosition(2, 1)),
                start=NumberLiteral(1, SourcePosition(2, 6)),
                stop=NumberLiteral(2, SourcePosition(2, 8)),
                step=None,
                position=SourcePosition(2, 1)
            )
        )
        
        result = self.analyzer.analyze(slice_access, clear_state=False)
        
        assert not result.success
        assert len(result.errors) == 1
        assert isinstance(result.errors[0], InvalidMethodCallError)
        assert "not sliceable" in str(result.errors[0])
    
    def test_list_literal_analysis(self):
        """Test list literal analysis."""
        result = self.analyze_code('list items = ["apple", "banana", 42]')
        
        assert result.success
        assert len(result.errors) == 0
    
    def test_assignment_analysis(self):
        """Test assignment statement analysis."""
        # Declare array symbol directly
        symbol = Symbol("arr", "list", None, SourcePosition(1, 1))
        self.analyzer.symbol_table.declare_symbol(symbol)
        
        # Create assignment AST
        assignment = IndexAssignment(
            target=IndexAccess(
                target=VariableRef("arr", SourcePosition(2, 1)),
                indices=[NumberLiteral(0, SourcePosition(2, 5))],
                position=SourcePosition(2, 1)
            ),
            value=NumberLiteral(99, SourcePosition(2, 10)),
            position=SourcePosition(2, 1)
        )
        
        result = self.analyzer.analyze(assignment, clear_state=False)
        
        assert result.success
        assert len(result.errors) == 0
    
    def test_expression_statement_analysis(self):
        """Test expression statement analysis."""
        # Simple literal expression
        result = self.analyze_code('42')
        assert result.success
        
        # String literal expression
        result = self.analyze_code('"hello world"')
        assert result.success
        
        # Boolean literal expression
        result = self.analyze_code('true')
        assert result.success
    
    def test_multiple_errors(self):
        """Test handling multiple errors in one analysis."""
        # This is tricky with single statements, but we can test
        # a complex expression with multiple undefined variables
        result = self.analyze_code('undefined1.method(undefined2)')
        
        assert not result.success
        # Should have at least one error for undefined1
        assert len(result.errors) >= 1
        assert any(isinstance(e, UndefinedVariableError) for e in result.errors)
    
    def test_analysis_result_properties(self):
        """Test AnalysisResult properties and methods."""
        # Successful result
        success_result = self.analyze_code('string text = "hello"')
        
        assert success_result.success == True
        assert success_result.has_errors() == False
        assert success_result.ast is not None
        assert success_result.symbol_table.size() == 1
        
        # Failed result
        fail_result = self.analyze_code('undefined_var')
        
        assert fail_result.success == False
        assert fail_result.has_errors() == True
        assert fail_result.ast is not None
        assert len(fail_result.errors) == 1
    
    def test_analyzer_reset_between_analyses(self):
        """Test that analyzer properly resets between analyses."""
        # First analysis
        result1 = self.analyze_code('string var1 = "hello"')
        assert result1.success
        assert result1.symbol_table.symbol_exists("var1")
        
        # Second analysis should start fresh
        result2 = self.analyze_code('string var2 = "world"')
        assert result2.success
        assert result2.symbol_table.symbol_exists("var2")
        assert not result2.symbol_table.symbol_exists("var1")  # Previous var not there


class TestTypeInferenceSemanticAnalysis:
    """Test semantic analysis for type inference assignments."""
    
    def setup_method(self):
        self.analyzer = SemanticAnalyzer()
        self.parser = ASTParser()
    
    def analyze_code(self, code: str) -> AnalysisResult:
        """Helper to parse and analyze code."""
        ast = self.parser.parse(code)
        return self.analyzer.analyze(ast)
    
    def test_type_inference_assignment_success(self):
        """Test that assignment to undefined variable succeeds with type inference."""
        result = self.analyze_code('name = "Alice"')
        
        assert result.success
        assert len(result.errors) == 0
        
        # Check that an inferred symbol was created
        assert result.symbol_table.symbol_exists("name")
        symbol = result.symbol_table.lookup_symbol("name")
        assert symbol is not None
        assert symbol.symbol_type == "string"  # Type is now inferred as string
        assert symbol.name == "name"
    
    def test_multiple_type_inference_assignments(self):
        """Test multiple type inference assignments in sequence."""
        # This tests persistent state within one analysis
        from glang.semantic.pipeline import SemanticSession
        session = SemanticSession()
        
        # First assignment
        result1 = session.analyze_statement('age = 25')
        assert result1.success
        
        # Second assignment
        result2 = session.analyze_statement('name = "Bob"')
        assert result2.success
        
        # Check that both symbols exist
        symbol_table = session.get_symbol_table()
        assert symbol_table.symbol_exists("age")
        assert symbol_table.symbol_exists("name")
        
        age_symbol = symbol_table.lookup_symbol("age")
        name_symbol = symbol_table.lookup_symbol("name")
        assert age_symbol.symbol_type == "num"  # Type is now inferred as num
        assert name_symbol.symbol_type == "string"  # Type is now inferred as string
    
    def test_type_inference_vs_explicit_declaration(self):
        """Test that explicit declarations still work alongside type inference."""
        # Test sequence: explicit then inferred
        from glang.semantic.pipeline import SemanticSession
        session = SemanticSession()
        
        # Explicit declaration
        result1 = session.analyze_statement('string title = "Engineer"')
        assert result1.success
        
        # Type inference
        result2 = session.analyze_statement('age = 30')
        assert result2.success
        
        # Check symbols
        symbol_table = session.get_symbol_table()
        title_symbol = symbol_table.lookup_symbol("title")
        age_symbol = symbol_table.lookup_symbol("age")
        
        assert title_symbol.symbol_type == "string"  # Explicit
        assert age_symbol.symbol_type == "num"  # Type is now inferred as num
    
    def test_assignment_to_existing_variable(self):
        """Test that assignment to existing variable works normally."""
        from glang.semantic.pipeline import SemanticSession
        session = SemanticSession()
        
        # Create variable with type inference
        result1 = session.analyze_statement('count = 10')
        assert result1.success
        
        # Assign to existing variable
        result2 = session.analyze_statement('count = 20')
        assert result2.success
        
        # Should still have just one symbol
        symbol_table = session.get_symbol_table()
        symbols = symbol_table.get_all_symbols()
        assert len(symbols) == 1
        assert "count" in symbols
    
    def test_undefined_variable_access_still_fails(self):
        """Test that accessing undefined variables still produces errors."""
        result = self.analyze_code('undefined_var')
        
        assert not result.success
        assert len(result.errors) == 1
        assert isinstance(result.errors[0], UndefinedVariableError)
        assert "undefined_var" in str(result.errors[0])


if __name__ == '__main__':
    pytest.main([__file__])
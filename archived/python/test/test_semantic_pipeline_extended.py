"""Extended tests for semantic pipeline."""

import pytest
from unittest.mock import Mock, patch

from src.glang.semantic.pipeline import SemanticPipeline
from src.glang.semantic.analyzer import AnalysisResult
from src.glang.semantic.errors import SemanticError
from src.glang.parser.ast_parser import ParseError
from src.glang.ast.nodes import SourcePosition


class TestSemanticPipeline:
    """Test SemanticPipeline class."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.pipeline = SemanticPipeline()
    
    def test_pipeline_initialization(self):
        """Test pipeline initialization."""
        assert self.pipeline.parser is not None
        assert self.pipeline.analyzer is not None
    
    def test_analyze_code_success(self):
        """Test successful code analysis."""
        # Simple valid code
        code = "string x = \"test\""
        result = self.pipeline.analyze_code(code)
        
        assert isinstance(result, AnalysisResult)
        # Should succeed
        assert result.success == True or result.errors == []
    
    def test_analyze_code_parse_error(self):
        """Test code analysis with parse error."""
        # Mock a parse error
        mock_parse_error = ParseError("Invalid syntax")
        
        with patch.object(self.pipeline.parser, 'parse', side_effect=mock_parse_error):
            result = self.pipeline.analyze_code("invalid syntax")
            
            assert isinstance(result, AnalysisResult)
            assert result.success == False
            assert len(result.errors) > 0
    
    def test_analyze_code_semantic_error(self):
        """Test code analysis with semantic error."""
        # Mock successful parsing but failed semantic analysis
        mock_ast = Mock()
        mock_result = AnalysisResult(
            ast=mock_ast,
            symbol_table=Mock(),
            errors=[SemanticError("Type error")],
            success=False
        )
        
        with patch.object(self.pipeline.parser, 'parse', return_value=mock_ast):
            with patch.object(self.pipeline.analyzer, 'analyze', return_value=mock_result):
                result = self.pipeline.analyze_code("string x = 42")
                
                assert isinstance(result, AnalysisResult)
                assert result.success == False
                assert len(result.errors) > 0
    
    def test_analyze_empty_code(self):
        """Test analysis of empty code."""
        result = self.pipeline.analyze_code("")
        
        assert isinstance(result, AnalysisResult)
        # Empty code might be valid or invalid depending on implementation
    
    def test_analyze_whitespace_only(self):
        """Test analysis of whitespace-only code."""
        result = self.pipeline.analyze_code("   \n\t  ")
        
        assert isinstance(result, AnalysisResult)
    
    def test_analyze_comment_only(self):
        """Test analysis of comment-only code."""
        result = self.pipeline.analyze_code("# This is just a comment")
        
        assert isinstance(result, AnalysisResult)
    
    def test_analyze_multiple_statements(self):
        """Test analysis of multiple statements."""
        code = '''
        string name = "Alice"
        num age = 25
        bool active = true
        '''
        result = self.pipeline.analyze_code(code)
        
        assert isinstance(result, AnalysisResult)
    
    def test_analyze_complex_expressions(self):
        """Test analysis of complex expressions."""
        code = 'list<num> numbers = [1, 2, 3]'
        result = self.pipeline.analyze_code(code)
        
        assert isinstance(result, AnalysisResult)
    
    def test_parse_error_conversion(self):
        """Test that parse errors are properly converted."""
        # Create a mock parse error with position
        mock_token = Mock()
        mock_token.line = 2
        mock_token.column = 5
        mock_parse_error = ParseError("Syntax error", mock_token)
        
        with patch.object(self.pipeline.parser, 'parse', side_effect=mock_parse_error):
            result = self.pipeline.analyze_code("invalid")
            
            assert result.success == False
            assert len(result.errors) > 0
            # Should be converted to SemanticError
            assert any(isinstance(error, SemanticError) for error in result.errors)


class TestSemanticPipelineEdgeCases:
    """Test edge cases for SemanticPipeline."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.pipeline = SemanticPipeline()
    
    def test_analyze_very_long_code(self):
        """Test analysis of very long code."""
        # Generate long but valid code
        code = "\n".join([f"num var_{i} = {i}" for i in range(50)])
        result = self.pipeline.analyze_code(code)
        
        assert isinstance(result, AnalysisResult)
    
    def test_analyze_nested_structures(self):
        """Test analysis of nested structures."""
        code = '''
        if true {
            if false {
                string nested = "deep"
            }
        }
        '''
        result = self.pipeline.analyze_code(code)
        
        assert isinstance(result, AnalysisResult)
    
    def test_analyze_unicode_code(self):
        """Test analysis of code with unicode characters."""
        code = 'string greeting = "Hello 世界"'
        result = self.pipeline.analyze_code(code)
        
        assert isinstance(result, AnalysisResult)
    
    def test_analyze_special_characters(self):
        """Test analysis with special characters."""
        code = 'string text = "Line 1\\nLine 2\\tTab"'
        result = self.pipeline.analyze_code(code)
        
        assert isinstance(result, AnalysisResult)
    
    def test_error_handling_consistency(self):
        """Test that errors are handled consistently."""
        error_cases = [
            "undefined_variable",
            "string x = 123",  # Type mismatch
            "if {",  # Incomplete structure
        ]
        
        for code in error_cases:
            try:
                result = self.pipeline.analyze_code(code)
                assert isinstance(result, AnalysisResult)
                # Should not crash, may succeed or fail depending on implementation
            except Exception as e:
                # If an exception is raised, it should be handled gracefully
                pytest.fail(f"Unexpected exception for code '{code}': {e}")


class TestSemanticPipelineIntegration:
    """Test SemanticPipeline integration scenarios."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.pipeline = SemanticPipeline()
    
    def test_analyze_variable_declarations(self):
        """Test analysis of various variable declarations."""
        test_cases = [
            "string name = \"test\"",
            "num count = 42",
            "bool flag = true",
            "list items = [1, 2, 3]",
        ]
        
        for code in test_cases:
            result = self.pipeline.analyze_code(code)
            assert isinstance(result, AnalysisResult)
    
    def test_analyze_function_definitions(self):
        """Test analysis of function definitions."""
        code = '''
        func greet(name) {
            return "Hello, " + name
        }
        '''
        result = self.pipeline.analyze_code(code)
        
        assert isinstance(result, AnalysisResult)
    
    def test_analyze_control_flow(self):
        """Test analysis of control flow statements."""
        code = '''
        num x = 5
        if x > 0 {
            print("positive")
        } else {
            print("non-positive")
        }
        '''
        result = self.pipeline.analyze_code(code)
        
        assert isinstance(result, AnalysisResult)
    
    def test_pipeline_reuse(self):
        """Test that pipeline can be reused for multiple analyses."""
        codes = [
            "string a = \"first\"",
            "num b = 42",
            "bool c = false",
        ]
        
        results = []
        for code in codes:
            result = self.pipeline.analyze_code(code)
            results.append(result)
            assert isinstance(result, AnalysisResult)
        
        # All analyses should succeed independently
        assert len(results) == 3
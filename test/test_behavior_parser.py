"""Tests for behavior syntax parsing."""

import pytest
from glang.parser.ast_parser import ASTParser, ParseError
from glang.ast.nodes import VariableDeclaration, BehaviorList, BehaviorCall


class TestBehaviorParsing:
    """Test parsing of behavior syntax."""
    
    def test_parse_simple_behavior_list(self):
        """Test parsing simple behavior list."""
        parser = ASTParser()
        
        # Simple behavior list
        code = 'num temp with [nil_to_zero, round_to_int] = 98.6'
        result = parser.parse(code)
        
        assert isinstance(result, VariableDeclaration)
        assert result.var_type == "num"
        assert result.name == "temp"
        assert result.behaviors is not None
        assert isinstance(result.behaviors, BehaviorList)
        assert len(result.behaviors.behaviors) == 2
        assert result.behaviors.behaviors[0] == "nil_to_zero"
        assert result.behaviors.behaviors[1] == "round_to_int"
    
    def test_parse_behavior_with_arguments(self):
        """Test parsing behavior calls with arguments."""
        parser = ASTParser()
        
        code = 'num score with [nil_to_zero, validate_range(0, 100)] = 85'
        result = parser.parse(code)
        
        assert isinstance(result, VariableDeclaration)
        assert result.behaviors is not None
        behaviors = result.behaviors.behaviors
        assert len(behaviors) == 2
        assert behaviors[0] == "nil_to_zero"
        
        # Second behavior should be a BehaviorCall
        assert isinstance(behaviors[1], BehaviorCall)
        assert behaviors[1].name == "validate_range"
        assert len(behaviors[1].arguments) == 2
    
    def test_parse_mixed_behavior_list(self):
        """Test parsing behavior list with mix of simple names and calls."""
        parser = ASTParser()
        
        code = 'list<num> readings with [nil_to_zero, validate_range(95, 105), round_to_int] = [98.6, 99.2]'
        result = parser.parse(code)
        
        assert isinstance(result, VariableDeclaration)
        assert result.var_type == "list"
        assert result.type_constraint == "num"
        assert result.behaviors is not None
        
        behaviors = result.behaviors.behaviors
        assert len(behaviors) == 3
        assert behaviors[0] == "nil_to_zero"
        assert isinstance(behaviors[1], BehaviorCall)
        assert behaviors[1].name == "validate_range"
        assert behaviors[2] == "round_to_int"
    
    def test_parse_without_behaviors(self):
        """Test parsing variable declaration without behaviors (backward compatibility)."""
        parser = ASTParser()
        
        code = 'string name = "Alice"'
        result = parser.parse(code)
        
        assert isinstance(result, VariableDeclaration)
        assert result.var_type == "string"
        assert result.name == "name"
        assert result.behaviors is None
    
    def test_parse_empty_behavior_list(self):
        """Test parsing empty behavior list."""
        parser = ASTParser()
        
        code = 'num value with [] = 42'
        result = parser.parse(code)
        
        assert isinstance(result, VariableDeclaration)
        assert result.behaviors is not None
        assert len(result.behaviors.behaviors) == 0
    
    def test_parse_behavior_with_multiple_args(self):
        """Test parsing behavior call with multiple arguments."""
        parser = ASTParser()
        
        code = 'num temp with [validate_range(95.0, 105.0, "strict")] = 98.6'
        result = parser.parse(code)
        
        behaviors = result.behaviors.behaviors
        assert len(behaviors) == 1
        assert isinstance(behaviors[0], BehaviorCall)
        assert behaviors[0].name == "validate_range"
        assert len(behaviors[0].arguments) == 3
    
    def test_parse_behavior_syntax_error(self):
        """Test parsing errors in behavior syntax."""
        parser = ASTParser()
        
        # Missing closing bracket in behavior list
        with pytest.raises(ParseError):
            parser.parse('num temp with [nil_to_zero = 98.6')
        
        # Missing opening bracket  
        with pytest.raises(ParseError):
            parser.parse('num temp with nil_to_zero] = 98.6')
        
        # Missing comma between behaviors
        with pytest.raises(ParseError):
            parser.parse('num temp with [nil_to_zero round_to_int] = 98.6')
    
    def test_parse_complex_nested_expression(self):
        """Test parsing behavior with complex nested expressions as arguments."""
        parser = ASTParser()
        
        # This should parse but may not execute correctly yet
        code = 'num result with [validate_range(10 + 5, 20 * 5)] = 100'
        result = parser.parse(code)
        
        behaviors = result.behaviors.behaviors
        assert len(behaviors) == 1
        assert isinstance(behaviors[0], BehaviorCall)
        assert behaviors[0].name == "validate_range"
        assert len(behaviors[0].arguments) == 2
        # Arguments should be parsed as expressions (not evaluated yet)
    
    def test_parse_all_syntax_variations(self):
        """Test parsing all supported syntax variations."""
        parser = ASTParser()
        
        test_cases = [
            # Basic types with behaviors
            'num x with [nil_to_zero] = 0',
            'string s with [uppercase] = "hello"',
            'bool flag with [nil_to_zero] = true',
            
            # Constrained types with behaviors
            'list<num> scores with [nil_to_zero, validate_range(0, 100)] = [95, 87]',
            'map<string> config with [env_normalize] = {"key": "value"}',
            
            # Multiple behaviors with various argument patterns
            'num temp with [nil_to_zero, validate_range(95, 105), round_to_int] = 98.6',
        ]
        
        for code in test_cases:
            result = parser.parse(code)
            assert isinstance(result, VariableDeclaration)
            assert result.behaviors is not None
            assert isinstance(result.behaviors, BehaviorList)
            assert len(result.behaviors.behaviors) > 0
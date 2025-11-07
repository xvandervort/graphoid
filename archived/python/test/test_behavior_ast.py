"""Tests for behavior AST nodes."""

import pytest
from glang.ast.nodes import (
    BehaviorCall, BehaviorList, VariableDeclaration, 
    NumberLiteral, StringLiteral, SourcePosition
)


class MockVisitor:
    """Mock visitor for testing AST nodes."""
    
    def __init__(self):
        self.visited = []
    
    def visit_behavior_call(self, node):
        self.visited.append(("behavior_call", node.name, len(node.arguments)))
        return f"behavior_call:{node.name}"
    
    def visit_behavior_list(self, node):
        self.visited.append(("behavior_list", len(node.behaviors)))
        return f"behavior_list:{len(node.behaviors)}"
    
    def visit_variable_declaration(self, node):
        self.visited.append(("variable_declaration", node.name, bool(node.behaviors)))
        return f"var_decl:{node.name}"
    
    def visit_number_literal(self, node):
        self.visited.append(("number_literal", node.value))
        return f"number:{node.value}"


class TestBehaviorASTNodes:
    """Test behavior AST node creation and visitor pattern."""
    
    def test_behavior_call_creation(self):
        """Test creating BehaviorCall nodes."""
        pos = SourcePosition(1, 10)
        
        # Simple behavior call with no arguments
        call1 = BehaviorCall("none_to_zero", [], pos)
        assert call1.name == "none_to_zero"
        assert call1.arguments == []
        assert call1.position == pos
        
        # Behavior call with arguments
        args = [NumberLiteral(95, pos), NumberLiteral(105, pos)]
        call2 = BehaviorCall("validate_range", args, pos)
        assert call2.name == "validate_range"
        assert len(call2.arguments) == 2
        assert call2.arguments[0].value == 95
        assert call2.arguments[1].value == 105
    
    def test_behavior_call_visitor(self):
        """Test BehaviorCall accepts visitor correctly."""
        visitor = MockVisitor()
        pos = SourcePosition(1, 10)
        
        args = [NumberLiteral(0, pos), NumberLiteral(100, pos)]
        call = BehaviorCall("validate_range", args, pos)
        
        result = call.accept(visitor)
        
        assert result == "behavior_call:validate_range"
        assert ("behavior_call", "validate_range", 2) in visitor.visited
    
    def test_behavior_list_creation(self):
        """Test creating BehaviorList nodes."""
        pos = SourcePosition(1, 15)
        
        # Mix of string behaviors and BehaviorCall
        behaviors = [
            "none_to_zero",
            BehaviorCall("validate_range", [NumberLiteral(0), NumberLiteral(100)]),
            "round_to_int"
        ]
        
        behavior_list = BehaviorList(behaviors, pos)
        assert len(behavior_list.behaviors) == 3
        assert behavior_list.behaviors[0] == "none_to_zero"
        assert isinstance(behavior_list.behaviors[1], BehaviorCall)
        assert behavior_list.behaviors[2] == "round_to_int"
        assert behavior_list.position == pos
    
    def test_behavior_list_visitor(self):
        """Test BehaviorList accepts visitor correctly."""
        visitor = MockVisitor()
        pos = SourcePosition(1, 15)
        
        behaviors = ["none_to_zero", "round_to_int"]
        behavior_list = BehaviorList(behaviors, pos)
        
        result = behavior_list.accept(visitor)
        
        assert result == "behavior_list:2"
        assert ("behavior_list", 2) in visitor.visited
    
    def test_variable_declaration_with_behaviors(self):
        """Test VariableDeclaration with behaviors field."""
        pos = SourcePosition(2, 1)
        
        # Create behavior list
        behaviors = BehaviorList(["none_to_zero", "positive"], pos)
        
        # Create variable declaration with behaviors
        var_decl = VariableDeclaration(
            var_type="num",
            name="temperature", 
            initializer=NumberLiteral(98.6, pos),
            type_constraint=None,
            behaviors=behaviors,
            position=pos
        )
        
        assert var_decl.var_type == "num"
        assert var_decl.name == "temperature"
        assert var_decl.behaviors == behaviors
        assert isinstance(var_decl.behaviors, BehaviorList)
        assert len(var_decl.behaviors.behaviors) == 2
    
    def test_variable_declaration_without_behaviors(self):
        """Test VariableDeclaration without behaviors (backward compatibility)."""
        pos = SourcePosition(2, 1)
        
        var_decl = VariableDeclaration(
            var_type="string",
            name="name",
            initializer=StringLiteral("test", pos),
            position=pos
        )
        
        assert var_decl.var_type == "string"
        assert var_decl.name == "name"
        assert var_decl.behaviors is None  # Default value
    
    def test_variable_declaration_visitor_with_behaviors(self):
        """Test VariableDeclaration visitor handles behaviors."""
        visitor = MockVisitor()
        pos = SourcePosition(2, 1)
        
        behaviors = BehaviorList(["none_to_zero"], pos)
        var_decl = VariableDeclaration(
            var_type="num",
            name="score",
            initializer=NumberLiteral(85, pos),
            behaviors=behaviors,
            position=pos
        )
        
        result = var_decl.accept(visitor)
        
        assert result == "var_decl:score"
        assert ("variable_declaration", "score", True) in visitor.visited
    
    def test_complex_behavior_structure(self):
        """Test complex nested behavior structure."""
        pos = SourcePosition(3, 5)
        
        # Create complex behavior list with calls and simple names
        behaviors = [
            "none_to_zero",
            BehaviorCall("validate_range", [
                NumberLiteral(95.0, pos),
                NumberLiteral(105.0, pos)
            ], pos),
            "round_to_int",
            BehaviorCall("map_colors", [], pos)  # No arguments
        ]
        
        behavior_list = BehaviorList(behaviors, pos)
        
        # Verify structure
        assert len(behavior_list.behaviors) == 4
        assert behavior_list.behaviors[0] == "none_to_zero"
        
        validate_call = behavior_list.behaviors[1]
        assert isinstance(validate_call, BehaviorCall)
        assert validate_call.name == "validate_range"
        assert len(validate_call.arguments) == 2
        assert validate_call.arguments[0].value == 95.0
        
        assert behavior_list.behaviors[2] == "round_to_int"
        
        map_call = behavior_list.behaviors[3]
        assert isinstance(map_call, BehaviorCall)
        assert map_call.name == "map_colors"
        assert len(map_call.arguments) == 0
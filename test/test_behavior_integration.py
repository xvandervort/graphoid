"""Integration test for complete behavior system - parser through execution."""

import pytest
from glang.parser.ast_parser import ASTParser
from glang.semantic.analyzer import SemanticAnalyzer
from glang.execution.executor import ASTExecutor


class TestBehaviorIntegration:
    """Test the complete behavior system from parsing to execution."""
    
    def test_end_to_end_behavior_processing(self):
        """Test complete pipeline: parse -> analyze -> execute with behaviors."""
        
        # Parse the code with behavior syntax
        parser = ASTParser()
        code = 'num temperature with [nil_to_zero, validate_range(95, 105)] = 110'
        ast = parser.parse(code)
        
        # Semantic analysis
        analyzer = SemanticAnalyzer()
        analyzer.analyze(ast)
        assert len(analyzer.errors) == 0, f"Semantic errors: {analyzer.errors}"
        
        # Execute - this should apply the behaviors
        from glang.execution.executor import ExecutionContext
        from glang.semantic.symbol_table import SymbolTable
        symbol_table = SymbolTable()
        context = ExecutionContext(symbol_table)
        executor = ASTExecutor(context)
        executor.execute(ast)
        
        # Check that the variable was created with behaviors applied
        # 110 should be clamped to 105 by validate_range(95, 105)
        result = executor.context.get_variable("temperature")
        assert result.value == 105, f"Expected 105, got {result.value}"
    
    def test_nil_handling_with_behaviors(self):
        """Test nil value handling through behaviors."""
        from glang.execution.values import NoneValue
        from glang.ast.nodes import VariableDeclaration, BehaviorList
        
        # Create AST manually for nil value
        nil_literal = NoneValue()  # This would normally be parsed as NilLiteral
        
        # For now, test with Python API since parser doesn't handle nil literals yet
        from glang.behaviors import BehaviorPipeline
        
        pipeline = BehaviorPipeline()
        pipeline.add("nil_to_zero")
        pipeline.add("validate_range", 0, 100)
        
        result = pipeline.apply(nil_literal)
        assert result.value == 0, f"Expected 0, got {result.value}"
    
    def test_list_behavior_processing(self):
        """Test behavior processing on list values."""
        from glang.behaviors import BehaviorPipeline
        from glang.execution.values import ListValue, NumberValue, NoneValue
        
        # Create a list with mixed values
        elements = [
            NumberValue(95),    # Normal
            NoneValue(),        # Missing -> should become 0 -> clamped to 50
            NumberValue(200),   # Too high -> should be clamped to 100
            NumberValue(-10)    # Negative -> should be clamped to 50
        ]
        test_list = ListValue(elements)
        
        # Create pipeline
        pipeline = BehaviorPipeline()
        pipeline.add("nil_to_zero")
        pipeline.add("validate_range", 50, 100)
        
        # Apply to list
        result = pipeline.apply_to_list(test_list)
        
        # Check results
        assert result.elements[0].value == 95   # unchanged
        assert result.elements[1].value == 50   # nil -> 0 -> clamped to 50
        assert result.elements[2].value == 100  # 200 -> clamped to 100
        assert result.elements[3].value == 50   # -10 -> clamped to 50
    
    def test_multiple_behavior_composition(self):
        """Test multiple behaviors applied in sequence."""
        from glang.behaviors import BehaviorPipeline
        from glang.execution.values import NumberValue
        
        # Test value that will be affected by multiple behaviors
        test_value = NumberValue(42.7)
        
        # Create pipeline with multiple behaviors
        pipeline = BehaviorPipeline()
        pipeline.add("positive")        # Ensure positive (no effect here)
        pipeline.add("validate_range", 0, 50)    # Clamp to 0-50 (no effect)
        pipeline.add("round_to_int")    # Round 42.7 -> 43
        
        result = pipeline.apply(test_value)
        assert result.value == 43, f"Expected 43, got {result.value}"
    
    def test_behavior_error_handling(self):
        """Test error handling in behavior system."""
        from glang.behaviors import BehaviorPipeline
        
        pipeline = BehaviorPipeline()
        
        # This should raise an error for unknown behavior
        with pytest.raises(ValueError, match="Unknown behavior"):
            pipeline.add("nonexistent_behavior")
    
    def test_custom_behavior_registration(self):
        """Test registering and using custom behaviors."""
        from glang.behaviors import BehaviorRegistry, BehaviorPipeline, create_behavior
        from glang.execution.values import NumberValue
        
        # Create custom behavior
        def double_value(value):
            if hasattr(value, 'value') and isinstance(value.value, (int, float)):
                return NumberValue(value.value * 2)
            return value
        
        double_behavior = create_behavior("double", transform=double_value)
        
        # Create custom registry
        registry = BehaviorRegistry()
        registry.register("double", double_behavior)
        
        # Use in pipeline
        pipeline = BehaviorPipeline(registry)
        pipeline.add("double")
        pipeline.add("nil_to_zero")  # This should still work from standard registry
        
        result = pipeline.apply(NumberValue(21))
        assert result.value == 42, f"Expected 42, got {result.value}"
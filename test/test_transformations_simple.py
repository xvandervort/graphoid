"""Simple tests for transformations registry to improve coverage."""

import pytest
from src.glang.execution.transformations import TransformationRegistry
from src.glang.execution.values import NumberValue, StringValue, BooleanValue
from src.glang.ast.nodes import SourcePosition


class TestTransformationRegistry:
    """Test TransformationRegistry basic functionality."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.registry = TransformationRegistry()
        self.pos = SourcePosition(1, 1)
    
    def test_registry_initialization(self):
        """Test registry initialization."""
        assert isinstance(self.registry.transformations, dict)
        assert isinstance(self.registry.predicates, dict)
        assert len(self.registry.transformations) > 0
        assert len(self.registry.predicates) > 0
    
    def test_numeric_transformations(self):
        """Test basic numeric transformations."""
        num_val = NumberValue(5, self.pos)
        
        # Test transformations exist
        assert "double" in self.registry.transformations
        assert "square" in self.registry.transformations
        assert "negate" in self.registry.transformations
        assert "increment" in self.registry.transformations
        assert "decrement" in self.registry.transformations
        
        # Test double transformation
        double_func = self.registry.transformations["double"]
        result = double_func(num_val)
        assert isinstance(result, NumberValue)
        
        # Test square transformation
        square_func = self.registry.transformations["square"]
        result = square_func(num_val)
        assert isinstance(result, NumberValue)
    
    def test_string_transformations(self):
        """Test basic string transformations."""
        str_val = StringValue("test", self.pos)
        
        # Test transformations exist
        assert "upper" in self.registry.transformations
        assert "lower" in self.registry.transformations
        assert "trim" in self.registry.transformations
        assert "reverse" in self.registry.transformations
        
        # Test upper transformation
        upper_func = self.registry.transformations["upper"]
        result = upper_func(str_val)
        assert isinstance(result, StringValue)
        
        # Test lower transformation
        lower_func = self.registry.transformations["lower"]
        result = lower_func(str_val)
        assert isinstance(result, StringValue)
    
    def test_numeric_predicates(self):
        """Test numeric predicates."""
        positive_num = NumberValue(5, self.pos)
        negative_num = NumberValue(-3, self.pos)
        zero_num = NumberValue(0, self.pos)
        
        # Test predicates exist
        assert "positive" in self.registry.predicates
        assert "negative" in self.registry.predicates
        assert "zero" in self.registry.predicates
        assert "even" in self.registry.predicates
        assert "odd" in self.registry.predicates
        
        # Test positive predicate
        positive_func = self.registry.predicates["positive"]
        assert positive_func(positive_num) == True
        assert positive_func(negative_num) == False
        
        # Test zero predicate
        zero_func = self.registry.predicates["zero"]
        assert zero_func(zero_num) == True
        assert zero_func(positive_num) == False
    
    def test_type_checking_predicates(self):
        """Test type checking predicates."""
        num_val = NumberValue(42, self.pos)
        str_val = StringValue("test", self.pos)
        bool_val = BooleanValue(True, self.pos)
        
        # Test type predicates exist
        assert "is_string" in self.registry.predicates
        assert "is_number" in self.registry.predicates  
        assert "is_bool" in self.registry.predicates
        
        # Test is_number predicate
        is_number_func = self.registry.predicates["is_number"]
        assert is_number_func(num_val) == True
        assert is_number_func(str_val) == False
        
        # Test is_string predicate
        is_string_func = self.registry.predicates["is_string"]
        assert is_string_func(str_val) == True
        assert is_string_func(num_val) == False
    
    def test_transformation_error_handling(self):
        """Test transformation error handling."""
        str_val = StringValue("test", self.pos)
        
        # Double should fail on string
        double_func = self.registry.transformations["double"]
        with pytest.raises(ValueError):
            double_func(str_val)
        
        # Square should fail on string
        square_func = self.registry.transformations["square"]
        with pytest.raises(ValueError):
            square_func(str_val)
    
    def test_predicate_error_handling(self):
        """Test predicate error handling with invalid types."""
        str_val = StringValue("test", self.pos)
        
        # Numeric predicates should handle string gracefully
        positive_func = self.registry.predicates["positive"]
        try:
            result = positive_func(str_val)
            assert isinstance(result, bool)
        except ValueError:
            # Expected for type mismatch
            pass
    
    def test_get_transformation(self):
        """Test getting transformations by name."""
        # Test getting valid transformation
        double_func = self.registry.get_transformation("double")
        assert double_func is not None
        assert callable(double_func)
        
        # Test getting invalid transformation
        invalid_func = self.registry.get_transformation("invalid_transform")
        assert invalid_func is None
    
    def test_get_predicate(self):
        """Test getting predicates by name.""" 
        # Test getting valid predicate
        positive_func = self.registry.get_predicate("positive")
        assert positive_func is not None
        assert callable(positive_func)
        
        # Test getting invalid predicate
        invalid_func = self.registry.get_predicate("invalid_predicate")
        assert invalid_func is None
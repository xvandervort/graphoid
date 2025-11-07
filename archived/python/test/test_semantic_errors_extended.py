"""Additional tests for semantic error classes."""

import pytest
from src.glang.semantic.errors import (
    SemanticError,
    UndefinedVariableError,
    TypeMismatchError,
    ConstraintViolationError,
    InvalidMethodCallError,
    RedeclarationError,
    InvalidTypeError,
    InvalidConstraintError
)
from src.glang.ast.nodes import SourcePosition


class TestSemanticError:
    """Test base SemanticError class."""
    
    def test_basic_error_message(self):
        """Test basic error message without position."""
        error = SemanticError("Test error")
        assert str(error) == "Test error"
        assert error.message == "Test error"
        assert error.position is None
    
    def test_error_with_position(self):
        """Test error message with position information."""
        position = SourcePosition(5, 10)
        error = SemanticError("Test error", position)
        
        expected = "Test error at line 5, column 10"
        assert str(error) == expected
        assert error.message == "Test error"
        assert error.position == position
    
    def test_format_error_method(self):
        """Test _format_error method directly."""
        error = SemanticError("Test message")
        assert error._format_error() == "Test message"
        
        error_with_pos = SemanticError("Test message", SourcePosition(1, 1))
        assert error_with_pos._format_error() == "Test message at line 1, column 1"


class TestUndefinedVariableError:
    """Test UndefinedVariableError class."""
    
    def test_basic_undefined_variable(self):
        """Test basic undefined variable error."""
        error = UndefinedVariableError("my_var")
        assert "Undefined variable 'my_var'" in str(error)
        assert error.variable_name == "my_var"
    
    def test_undefined_variable_with_position(self):
        """Test undefined variable error with position."""
        position = SourcePosition(3, 7)
        error = UndefinedVariableError("unknown_var", position)
        
        assert "Undefined variable 'unknown_var'" in str(error)
        assert "at line 3, column 7" in str(error)
        assert error.variable_name == "unknown_var"
        assert error.position == position


class TestTypeMismatchError:
    """Test TypeMismatchError class."""
    
    def test_basic_type_mismatch(self):
        """Test basic type mismatch error."""
        error = TypeMismatchError("string", "number")
        expected = "Type mismatch: expected string, got number"
        assert str(error) == expected
        assert error.expected == "string"
        assert error.actual == "number"
    
    def test_type_mismatch_with_context(self):
        """Test type mismatch with context."""
        error = TypeMismatchError("list", "string", "assignment")
        expected = "Type mismatch in assignment: expected list, got string"
        assert str(error) == expected
        assert error.context == "assignment"
    
    def test_type_mismatch_with_position(self):
        """Test type mismatch with position."""
        position = SourcePosition(2, 5)
        error = TypeMismatchError("number", "bool", "comparison", position)
        
        assert "Type mismatch in comparison: expected number, got bool" in str(error)
        assert "at line 2, column 5" in str(error)


class TestConstraintViolationError:
    """Test ConstraintViolationError class."""
    
    def test_basic_constraint_violation(self):
        """Test basic constraint violation error."""
        error = ConstraintViolationError("items", "num", "string")
        expected = "Constraint violation for 'items': expected num, got string"
        assert expected in str(error)
        assert error.variable_name == "items"
        assert error.constraint == "num"
        assert error.actual_type == "string"
    
    def test_constraint_violation_with_position(self):
        """Test constraint violation with position."""
        position = SourcePosition(4, 12)
        error = ConstraintViolationError("values", "string", "number", position)
        
        assert "Constraint violation for 'values': expected string, got number" in str(error)
        assert "at line 4, column 12" in str(error)
        assert error.position == position


class TestInvalidMethodCallError:
    """Test InvalidMethodCallError class."""
    
    def test_basic_invalid_method(self):
        """Test basic invalid method call error."""
        error = InvalidMethodCallError("append", "string")
        expected = "Method 'append' not available on string"
        assert expected in str(error)
        assert error.method_name == "append"
        assert error.target_type == "string"
    
    def test_invalid_method_with_reason(self):
        """Test invalid method call with reason."""
        error = InvalidMethodCallError("sort", "number", "not a collection")
        expected = "Invalid method call 'sort' on number: not a collection"
        assert expected in str(error)
        assert error.reason == "not a collection"
    
    def test_invalid_method_with_position(self):
        """Test invalid method call with position."""
        position = SourcePosition(6, 8)
        error = InvalidMethodCallError("size", "bool", "not supported", position)
        
        assert "Invalid method call 'size' on bool: not supported" in str(error)
        assert "at line 6, column 8" in str(error)


class TestRedeclarationError:
    """Test RedeclarationError class."""
    
    def test_basic_redeclaration(self):
        """Test basic redeclaration error."""
        error = RedeclarationError("duplicate_var")
        assert "Variable 'duplicate_var' already declared" in str(error)
        assert error.variable_name == "duplicate_var"
    
    def test_redeclaration_with_original_position(self):
        """Test redeclaration with original position."""
        original_pos = SourcePosition(1, 5)
        error = RedeclarationError("var_name", original_pos)
        
        assert "Variable 'var_name' already declared (originally at line 1)" in str(error)
        assert error.original_position == original_pos
    
    def test_redeclaration_with_both_positions(self):
        """Test redeclaration with both positions."""
        original_pos = SourcePosition(1, 5)
        new_pos = SourcePosition(3, 10)
        error = RedeclarationError("var_name", original_pos, new_pos)
        
        error_str = str(error)
        assert "Variable 'var_name' already declared (originally at line 1)" in error_str
        assert "at line 3, column 10" in error_str
        assert error.new_position == new_pos


class TestInvalidTypeError:
    """Test InvalidTypeError class."""
    
    def test_basic_invalid_type(self):
        """Test basic invalid type error."""
        error = InvalidTypeError("unknown_type")
        assert "Invalid type 'unknown_type'" in str(error)
        assert error.type_name == "unknown_type"
    
    def test_invalid_type_with_position(self):
        """Test invalid type with position."""
        position = SourcePosition(7, 15)
        error = InvalidTypeError("bad_type", position)
        
        assert "Invalid type 'bad_type'" in str(error)
        assert "at line 7, column 15" in str(error)
        assert error.position == position


class TestInvalidConstraintError:
    """Test InvalidConstraintError class."""
    
    def test_basic_invalid_constraint(self):
        """Test basic invalid constraint error."""
        error = InvalidConstraintError("invalid_constraint", "list")
        expected = "Invalid constraint 'invalid_constraint' for type 'list'"
        assert expected in str(error)
        assert error.constraint == "invalid_constraint"
        assert error.base_type == "list"
    
    def test_invalid_constraint_with_position(self):
        """Test invalid constraint with position."""
        position = SourcePosition(9, 20)
        error = InvalidConstraintError("bad_constraint", "hash", position)
        
        assert "Invalid constraint 'bad_constraint' for type 'hash'" in str(error)
        assert "at line 9, column 20" in str(error)
        assert error.position == position


class TestErrorInheritance:
    """Test that all errors properly inherit from SemanticError."""
    
    def test_all_inherit_from_semantic_error(self):
        """Test that all error classes inherit from SemanticError."""
        error_classes = [
            UndefinedVariableError,
            TypeMismatchError,
            ConstraintViolationError,
            InvalidMethodCallError,
            RedeclarationError,
            InvalidTypeError,
            InvalidConstraintError
        ]
        
        for error_class in error_classes:
            # Create instance with minimal arguments
            if error_class == TypeMismatchError:
                error = error_class("expected", "actual")
            elif error_class == ConstraintViolationError:
                error = error_class("var", "constraint", "actual")
            elif error_class == InvalidMethodCallError:
                error = error_class("method", "type")
            elif error_class == InvalidConstraintError:
                error = error_class("constraint", "base_type")
            else:
                error = error_class("test_value")
            
            assert isinstance(error, SemanticError)
            assert isinstance(error, Exception)


class TestErrorAttributeAccess:
    """Test attribute access on error objects."""
    
    def test_semantic_error_attributes(self):
        """Test SemanticError attributes."""
        position = SourcePosition(1, 1)
        error = SemanticError("test", position)
        
        assert hasattr(error, 'message')
        assert hasattr(error, 'position')
        assert error.message == "test"
        assert error.position == position
    
    def test_type_mismatch_attributes(self):
        """Test TypeMismatchError attributes."""
        error = TypeMismatchError("string", "number", "test_context")
        
        assert hasattr(error, 'expected')
        assert hasattr(error, 'actual')
        assert hasattr(error, 'context')
        assert error.expected == "string"
        assert error.actual == "number"
        assert error.context == "test_context"
    
    def test_redeclaration_attributes(self):
        """Test RedeclarationError attributes."""
        orig_pos = SourcePosition(1, 1)
        new_pos = SourcePosition(2, 2)
        error = RedeclarationError("var", orig_pos, new_pos)
        
        assert hasattr(error, 'variable_name')
        assert hasattr(error, 'original_position')
        assert hasattr(error, 'new_position')
        assert error.variable_name == "var"
        assert error.original_position == orig_pos
        assert error.new_position == new_pos
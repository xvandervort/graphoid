"""Extended tests for execution errors."""

import pytest
from src.glang.execution.errors import (
    RuntimeError as GlangRuntimeError,
    VariableNotFoundError,
    TypeConstraintError,
    MethodNotFoundError,
    IndexError as GlangIndexError,
    LoadRequest,
    ImportRequest
)
from src.glang.ast.nodes import SourcePosition


class TestGlangRuntimeError:
    """Test base RuntimeError class."""
    
    def test_basic_runtime_error(self):
        """Test basic runtime error without position."""
        error = GlangRuntimeError("Test runtime error")
        
        assert error.message == "Test runtime error"
        assert error.position is None
        assert str(error) == "Runtime error: Test runtime error"
    
    def test_runtime_error_with_position(self):
        """Test runtime error with position information."""
        position = SourcePosition(5, 10)
        error = GlangRuntimeError("Test error with position", position)
        
        assert error.message == "Test error with position"
        assert error.position == position
        expected = "Runtime error: Test error with position at line 5, column 10"
        assert str(error) == expected
    
    def test_runtime_error_inheritance(self):
        """Test that GlangRuntimeError inherits from Exception."""
        error = GlangRuntimeError("Test error")
        
        assert isinstance(error, Exception)
        assert isinstance(error, GlangRuntimeError)
    
    def test_runtime_error_format_method(self):
        """Test the _format_error method."""
        # Without position
        error = GlangRuntimeError("No position")
        formatted = error._format_error()
        assert formatted == "Runtime error: No position"
        
        # With position
        position = SourcePosition(1, 1)
        error_with_pos = GlangRuntimeError("With position", position)
        formatted_with_pos = error_with_pos._format_error()
        assert "at line 1, column 1" in formatted_with_pos


class TestVariableNotFoundError:
    """Test VariableNotFoundError class."""
    
    def test_basic_variable_not_found(self):
        """Test basic variable not found error."""
        error = VariableNotFoundError("unknown_var")
        
        assert error.variable_name == "unknown_var"
        assert "Variable 'unknown_var' not found" in error.message
        assert isinstance(error, GlangRuntimeError)
    
    def test_variable_not_found_with_position(self):
        """Test variable not found error with position."""
        position = SourcePosition(3, 7)
        error = VariableNotFoundError("missing_var", position)
        
        assert error.variable_name == "missing_var"
        assert error.position == position
        assert "Variable 'missing_var' not found" in str(error)
        assert "at line 3, column 7" in str(error)
    
    def test_variable_not_found_inheritance(self):
        """Test VariableNotFoundError inheritance."""
        error = VariableNotFoundError("test_var")
        
        assert isinstance(error, VariableNotFoundError)
        assert isinstance(error, GlangRuntimeError)
        assert isinstance(error, Exception)
    
    def test_variable_not_found_empty_name(self):
        """Test variable not found with empty variable name."""
        error = VariableNotFoundError("")
        
        assert error.variable_name == ""
        assert "Variable '' not found" in error.message
    
    def test_variable_not_found_special_characters(self):
        """Test variable not found with special characters in name."""
        special_names = ["var_with_underscore", "var-with-dash", "var123", "var.with.dots"]
        
        for name in special_names:
            error = VariableNotFoundError(name)
            assert error.variable_name == name
            assert f"Variable '{name}' not found" in error.message


class TestTypeConstraintError:
    """Test TypeConstraintError class."""
    
    def test_basic_type_constraint_error(self):
        """Test basic type constraint error."""
        error = TypeConstraintError("Type constraint violated")
        
        assert error.message == "Type constraint violated"
        assert error.position is None
        assert isinstance(error, GlangRuntimeError)
    
    def test_type_constraint_error_with_position(self):
        """Test type constraint error with position."""
        position = SourcePosition(2, 15)
        error = TypeConstraintError("Expected number, got string", position)
        
        assert error.message == "Expected number, got string"
        assert error.position == position
        assert "Expected number, got string" in str(error)
        assert "at line 2, column 15" in str(error)
    
    def test_type_constraint_error_inheritance(self):
        """Test TypeConstraintError inheritance."""
        error = TypeConstraintError("Type error")
        
        assert isinstance(error, TypeConstraintError)
        assert isinstance(error, GlangRuntimeError)
        assert isinstance(error, Exception)
    
    def test_type_constraint_error_various_messages(self):
        """Test type constraint error with various message types."""
        messages = [
            "Cannot assign string to number variable",
            "List constraint violation: expected num, got string",
            "Invalid type for operation",
            ""  # Empty message
        ]
        
        for message in messages:
            error = TypeConstraintError(message)
            assert error.message == message


class TestMethodNotFoundError:
    """Test MethodNotFoundError class."""
    
    def test_basic_method_not_found(self):
        """Test basic method not found error."""
        error = MethodNotFoundError("append", "string")
        
        # Test that the error exists and is properly formatted
        assert isinstance(error, GlangRuntimeError)
        assert "append" in str(error)
        assert "string" in str(error)
    
    def test_method_not_found_with_position(self):
        """Test method not found error with position."""
        position = SourcePosition(4, 12)
        error = MethodNotFoundError("size", "bool", position)
        
        assert isinstance(error, GlangRuntimeError)
        assert "size" in str(error)
        assert "bool" in str(error)
        assert "line 4" in str(error)
    
    def test_method_not_found_inheritance(self):
        """Test MethodNotFoundError inheritance."""
        error = MethodNotFoundError("test_method", "test_type")
        
        assert isinstance(error, MethodNotFoundError)
        assert isinstance(error, GlangRuntimeError)
        assert isinstance(error, Exception)
    
    def test_method_not_found_various_types(self):
        """Test method not found with various type names."""
        test_cases = [
            ("append", "string"),
            ("sort", "number"),
            ("get", "boolean"),
            ("custom_method", "custom_type")
        ]
        
        for method, target_type in test_cases:
            error = MethodNotFoundError(method, target_type)
            assert isinstance(error, GlangRuntimeError)
            assert method in str(error)
            assert target_type in str(error)


class TestLoadRequest:
    """Test LoadRequest class."""
    
    def test_basic_load_request(self):
        """Test basic load request."""
        request = LoadRequest("module.gr")
        
        assert request.filename == "module.gr"
        assert request.position is None
        assert isinstance(request, Exception)
    
    def test_load_request_with_position(self):
        """Test load request with position."""
        position = SourcePosition(1, 5)
        request = LoadRequest("file.gr", position)
        
        assert request.filename == "file.gr"
        assert request.position == position
    
    def test_load_request_inheritance(self):
        """Test LoadRequest inheritance."""
        request = LoadRequest("test.gr")
        
        assert isinstance(request, LoadRequest)
        assert isinstance(request, Exception)
    
    def test_load_request_various_filenames(self):
        """Test load request with various filename formats."""
        filenames = [
            "simple.gr",
            "path/to/module.gr",
            "../relative.gr",
            "complex-name_123.gr"
        ]
        
        for filename in filenames:
            request = LoadRequest(filename)
            assert request.filename == filename


class TestImportRequest:
    """Test ImportRequest class."""
    
    def test_basic_import_request(self):
        """Test basic import request."""
        request = ImportRequest("math", None)
        
        assert isinstance(request, Exception)
        assert "math" in str(request) or hasattr(request, 'module_name')
    
    def test_import_request_inheritance(self):
        """Test ImportRequest inheritance."""
        request = ImportRequest("test_module", None)
        
        assert isinstance(request, ImportRequest)
        assert isinstance(request, Exception)


class TestErrorPositionHandling:
    """Test position handling across all error types."""
    
    def test_all_errors_handle_none_position(self):
        """Test that all error types handle None position gracefully."""
        errors = [
            GlangRuntimeError("Test", None),
            VariableNotFoundError("var", None),
            TypeConstraintError("Type error", None),
            MethodNotFoundError("method", "type", None),
            LoadRequest("file.gr", None),
            ImportRequest("module", None, None)
        ]
        
        for error in errors:
            # Should not raise exception
            str_repr = str(error)
            assert isinstance(str_repr, str)
            assert len(str_repr) > 0
    
    def test_all_errors_handle_valid_position(self):
        """Test that all error types handle valid position correctly."""
        position = SourcePosition(10, 20)
        
        errors = [
            GlangRuntimeError("Test", position),
            VariableNotFoundError("var", position),
            TypeConstraintError("Type error", position),
            MethodNotFoundError("method", "type", position),
            LoadRequest("file.gr", position),
            ImportRequest("module", None, position)
        ]
        
        for error in errors:
            if hasattr(error, 'position'):
                assert error.position == position
            str_repr = str(error)
            assert isinstance(str_repr, str)


class TestErrorStringRepresentations:
    """Test string representations of all error types."""
    
    def test_error_string_consistency(self):
        """Test that all errors produce consistent string representations."""
        test_errors = [
            GlangRuntimeError("Runtime test"),
            VariableNotFoundError("test_var"),
            TypeConstraintError("Constraint test"),
            MethodNotFoundError("test_method", "test_type"),
            LoadRequest("test.gr"),
            ImportRequest("test_module", "alias")
        ]
        
        for error in test_errors:
            str_repr = str(error)
            assert isinstance(str_repr, str)
            assert len(str_repr) > 0
            # Should not contain None or other invalid representations
            assert "None" not in str_repr or "test_module" in str_repr  # ImportRequest might legitimately contain None
    
    def test_error_repr_methods(self):
        """Test that errors have proper repr methods."""
        error = GlangRuntimeError("Test error")
        
        # Should have a repr
        repr_str = repr(error)
        assert isinstance(repr_str, str)
        assert len(repr_str) > 0


class TestIndexError:
    """Test IndexError class."""
    
    def test_basic_index_error(self):
        """Test basic index error."""
        error = GlangIndexError("Index out of bounds")
        
        assert "Index out of bounds" in str(error)
        assert isinstance(error, GlangRuntimeError)
        assert isinstance(error, Exception)
    
    def test_index_error_with_position(self):
        """Test index error with position."""
        position = SourcePosition(2, 5)
        error = GlangIndexError("List index out of range", position)
        
        assert "List index out of range" in str(error)
        assert "line 2" in str(error)
        assert "column 5" in str(error)
        assert error.position == position
    
    def test_index_error_inheritance(self):
        """Test IndexError inheritance."""
        error = GlangIndexError("Index error")
        
        assert isinstance(error, GlangIndexError)
        assert isinstance(error, GlangRuntimeError)
        assert isinstance(error, Exception)
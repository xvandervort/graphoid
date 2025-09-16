"""
Method Handler Refactoring Demo

This demonstrates how the large executor methods can be broken down
into focused, maintainable handler classes following the Single
Responsibility Principle.
"""

from typing import List, Optional, Any
from abc import ABC, abstractmethod

# Mock classes for demo purposes
class GlangValue:
    def __init__(self, value, position=None):
        self.value = value
        self.position = position

    def get_type(self):
        return "value"

class StringValue(GlangValue):
    def get_type(self):
        return "string"

class NumberValue(GlangValue):
    def get_type(self):
        return "num"

class BooleanValue(GlangValue):
    def get_type(self):
        return "bool"

class SourcePosition:
    def __init__(self, line, col):
        self.line = line
        self.col = col

class ArgumentError(Exception):
    pass

class RuntimeError(Exception):
    pass

# ============================================================================
# BASE HANDLER INTERFACE
# ============================================================================

class MethodHandler(ABC):
    """Base class for method handlers."""

    @abstractmethod
    def can_handle(self, method_name: str) -> bool:
        """Check if this handler can handle the given method."""
        pass

    @abstractmethod
    def handle(self, target: GlangValue, method_name: str,
               args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Handle the method call."""
        pass

# ============================================================================
# STRING TYPE CONVERSION HANDLER
# ============================================================================

class StringTypeConversionHandler(MethodHandler):
    """Handles string type conversion methods: to_string, to_num, to_bool"""

    def can_handle(self, method_name: str) -> bool:
        return method_name in ["to_string", "to_num", "to_bool"]

    def handle(self, target: StringValue, method_name: str,
               args: List[GlangValue], position: Optional[SourcePosition]) -> Any:

        if method_name == "to_string":
            return self._handle_to_string(target, args, position)
        elif method_name == "to_num":
            return self._handle_to_num(target, args, position)
        elif method_name == "to_bool":
            return self._handle_to_bool(target, args, position)

    def _handle_to_string(self, target: StringValue, args: List[GlangValue],
                         position: Optional[SourcePosition]) -> StringValue:
        if len(args) != 0:
            raise ArgumentError(f"to_string() takes no arguments, got {len(args)}")
        return target  # Already a string

    def _handle_to_num(self, target: StringValue, args: List[GlangValue],
                      position: Optional[SourcePosition]) -> NumberValue:
        if len(args) != 0:
            raise ArgumentError(f"to_num() takes no arguments, got {len(args)}")

        try:
            # Try int first, then float
            if '.' in target.value:
                return NumberValue(float(target.value), position)
            else:
                return NumberValue(int(target.value), position)
        except ValueError:
            raise RuntimeError(f"Cannot convert '{target.value}' to number")

    def _handle_to_bool(self, target: StringValue, args: List[GlangValue],
                       position: Optional[SourcePosition]) -> BooleanValue:
        if len(args) != 0:
            raise ArgumentError(f"to_bool() takes no arguments, got {len(args)}")
        # Empty string is false, non-empty is true
        return BooleanValue(len(target.value) > 0, position)

# ============================================================================
# STRING BASIC OPERATIONS HANDLER
# ============================================================================

class StringBasicOperationsHandler(MethodHandler):
    """Handles basic string operations: length, contains, starts_with, ends_with"""

    def can_handle(self, method_name: str) -> bool:
        return method_name in ["length", "contains", "starts_with", "ends_with"]

    def handle(self, target: StringValue, method_name: str,
               args: List[GlangValue], position: Optional[SourcePosition]) -> Any:

        if method_name == "length":
            return self._handle_length(target, args, position)
        elif method_name == "contains":
            return self._handle_contains(target, args, position)
        elif method_name == "starts_with":
            return self._handle_starts_with(target, args, position)
        elif method_name == "ends_with":
            return self._handle_ends_with(target, args, position)

    def _handle_length(self, target: StringValue, args: List[GlangValue],
                      position: Optional[SourcePosition]) -> NumberValue:
        if len(args) != 0:
            raise ArgumentError(f"length() takes no arguments, got {len(args)}")
        return NumberValue(len(target.value), position)

    def _handle_contains(self, target: StringValue, args: List[GlangValue],
                        position: Optional[SourcePosition]) -> BooleanValue:
        if len(args) == 1:
            # Backward compatibility: substring search
            if not isinstance(args[0], StringValue):
                raise ArgumentError(f"contains() argument must be string, got {args[0].get_type()}")
            return BooleanValue(args[0].value in target.value, position)
        else:
            # Extended pattern matching would go here
            raise ArgumentError(f"contains() takes 1 argument, got {len(args)}")

    def _handle_starts_with(self, target: StringValue, args: List[GlangValue],
                           position: Optional[SourcePosition]) -> BooleanValue:
        if len(args) != 1:
            raise ArgumentError(f"starts_with() takes 1 argument, got {len(args)}")
        if not isinstance(args[0], StringValue):
            raise ArgumentError(f"starts_with() argument must be string, got {args[0].get_type()}")
        return BooleanValue(target.value.startswith(args[0].value), position)

    def _handle_ends_with(self, target: StringValue, args: List[GlangValue],
                         position: Optional[SourcePosition]) -> BooleanValue:
        if len(args) != 1:
            raise ArgumentError(f"ends_with() takes 1 argument, got {len(args)}")
        if not isinstance(args[0], StringValue):
            raise ArgumentError(f"ends_with() argument must be string, got {args[0].get_type()}")
        return BooleanValue(target.value.endswith(args[0].value), position)

# ============================================================================
# STRING MANIPULATION HANDLER
# ============================================================================

class StringManipulationHandler(MethodHandler):
    """Handles string manipulation: up, down, trim, split, join, replace"""

    def can_handle(self, method_name: str) -> bool:
        return method_name in ["up", "toUpper", "down", "toLower", "trim", "split", "join", "replace"]

    def handle(self, target: StringValue, method_name: str,
               args: List[GlangValue], position: Optional[SourcePosition]) -> Any:

        if method_name in ["up", "toUpper"]:
            return self._handle_upper(target, args, position)
        elif method_name in ["down", "toLower"]:
            return self._handle_lower(target, args, position)
        elif method_name == "trim":
            return self._handle_trim(target, args, position)
        elif method_name == "split":
            return self._handle_split(target, args, position)
        elif method_name == "join":
            return self._handle_join(target, args, position)
        elif method_name == "replace":
            return self._handle_replace(target, args, position)

    def _handle_upper(self, target: StringValue, args: List[GlangValue],
                     position: Optional[SourcePosition]) -> StringValue:
        if len(args) != 0:
            raise ArgumentError(f"up() takes no arguments, got {len(args)}")
        return StringValue(target.value.upper(), position)

    def _handle_lower(self, target: StringValue, args: List[GlangValue],
                     position: Optional[SourcePosition]) -> StringValue:
        if len(args) != 0:
            raise ArgumentError(f"down() takes no arguments, got {len(args)}")
        return StringValue(target.value.lower(), position)

    def _handle_trim(self, target: StringValue, args: List[GlangValue],
                    position: Optional[SourcePosition]) -> StringValue:
        if len(args) != 0:
            raise ArgumentError(f"trim() takes no arguments, got {len(args)}")
        return StringValue(target.value.strip(), position)

    def _handle_split(self, target: StringValue, args: List[GlangValue],
                     position: Optional[SourcePosition]) -> Any:
        # Simplified implementation - would return ListValue in real code
        if len(args) != 1:
            raise ArgumentError(f"split() takes 1 argument, got {len(args)}")
        if not isinstance(args[0], StringValue):
            raise ArgumentError(f"split() argument must be string, got {args[0].get_type()}")
        parts = target.value.split(args[0].value)
        return f"List with {len(parts)} parts"  # Mock return

    def _handle_join(self, target: StringValue, args: List[GlangValue],
                    position: Optional[SourcePosition]) -> StringValue:
        # Simplified implementation
        if len(args) != 1:
            raise ArgumentError(f"join() takes 1 argument, got {len(args)}")
        return StringValue(f"Joined result", position)  # Mock return

    def _handle_replace(self, target: StringValue, args: List[GlangValue],
                       position: Optional[SourcePosition]) -> StringValue:
        if len(args) != 2:
            raise ArgumentError(f"replace() takes 2 arguments, got {len(args)}")
        if not all(isinstance(arg, StringValue) for arg in args):
            raise ArgumentError("replace() arguments must be strings")

        old_value = args[0].value
        new_value = args[1].value
        result = target.value.replace(old_value, new_value)
        return StringValue(result, position)

# ============================================================================
# STRING VALIDATION HANDLER
# ============================================================================

class StringValidationHandler(MethodHandler):
    """Handles string validation: is_email, is_number, is_url"""

    def can_handle(self, method_name: str) -> bool:
        return method_name in ["is_email", "is_number", "is_url"]

    def handle(self, target: StringValue, method_name: str,
               args: List[GlangValue], position: Optional[SourcePosition]) -> Any:

        if method_name == "is_email":
            return self._handle_is_email(target, args, position)
        elif method_name == "is_number":
            return self._handle_is_number(target, args, position)
        elif method_name == "is_url":
            return self._handle_is_url(target, args, position)

    def _handle_is_email(self, target: StringValue, args: List[GlangValue],
                        position: Optional[SourcePosition]) -> BooleanValue:
        if len(args) != 0:
            raise ArgumentError(f"is_email() takes no arguments, got {len(args)}")

        # Basic email validation
        email = target.value
        has_at = '@' in email and email.count('@') == 1
        has_dot = '.' in email
        return BooleanValue(has_at and has_dot and len(email) > 3, position)

    def _handle_is_number(self, target: StringValue, args: List[GlangValue],
                         position: Optional[SourcePosition]) -> BooleanValue:
        if len(args) != 0:
            raise ArgumentError(f"is_number() takes no arguments, got {len(args)}")

        try:
            float(target.value)
            return BooleanValue(True, position)
        except ValueError:
            return BooleanValue(False, position)

    def _handle_is_url(self, target: StringValue, args: List[GlangValue],
                      position: Optional[SourcePosition]) -> BooleanValue:
        if len(args) != 0:
            raise ArgumentError(f"is_url() takes no arguments, got {len(args)}")

        # Basic URL validation
        url = target.value.lower()
        return BooleanValue(url.startswith(('http://', 'https://', 'ftp://')), position)

# ============================================================================
# REFACTORED STRING METHOD DISPATCHER
# ============================================================================

class RefactoredStringMethodDispatcher:
    """Refactored string method dispatcher using handler pattern."""

    def __init__(self):
        self.handlers = [
            StringTypeConversionHandler(),
            StringBasicOperationsHandler(),
            StringManipulationHandler(),
            StringValidationHandler(),
        ]

    def dispatch_string_method(self, target: StringValue, method_name: str,
                              args: List[GlangValue], position: Optional[SourcePosition]) -> Any:
        """Dispatch string method call to appropriate handler."""

        # Find the handler that can handle this method
        for handler in self.handlers:
            if handler.can_handle(method_name):
                return handler.handle(target, method_name, args, position)

        # No handler found
        raise RuntimeError(f"Method '{method_name}' not found on type 'string'")

# ============================================================================
# DEMONSTRATION
# ============================================================================

def demonstrate_refactored_dispatcher():
    """Demonstrate the refactored method dispatcher."""

    print("=== Method Handler Refactoring Demo ===")
    print()

    # Create dispatcher and test data
    dispatcher = RefactoredStringMethodDispatcher()
    test_string = StringValue("Hello World", SourcePosition(1, 1))

    # Test type conversion methods
    print("Type Conversion Methods:")
    try:
        result = dispatcher.dispatch_string_method(test_string, "to_string", [], None)
        print(f"  to_string(): '{result.value}'")

        result = dispatcher.dispatch_string_method(test_string, "to_bool", [], None)
        print(f"  to_bool(): {result.value}")
    except Exception as e:
        print(f"  Error: {e}")

    # Test basic operations
    print("\nBasic Operations:")
    try:
        result = dispatcher.dispatch_string_method(test_string, "length", [], None)
        print(f"  length(): {result.value}")

        contains_arg = StringValue("World", None)
        result = dispatcher.dispatch_string_method(test_string, "contains", [contains_arg], None)
        print(f"  contains('World'): {result.value}")

        starts_arg = StringValue("Hello", None)
        result = dispatcher.dispatch_string_method(test_string, "starts_with", [starts_arg], None)
        print(f"  starts_with('Hello'): {result.value}")
    except Exception as e:
        print(f"  Error: {e}")

    # Test manipulation methods
    print("\nManipulation Methods:")
    try:
        result = dispatcher.dispatch_string_method(test_string, "up", [], None)
        print(f"  up(): '{result.value}'")

        result = dispatcher.dispatch_string_method(test_string, "down", [], None)
        print(f"  down(): '{result.value}'")

        old_arg = StringValue("World", None)
        new_arg = StringValue("Universe", None)
        result = dispatcher.dispatch_string_method(test_string, "replace", [old_arg, new_arg], None)
        print(f"  replace('World', 'Universe'): '{result.value}'")
    except Exception as e:
        print(f"  Error: {e}")

    # Test validation methods
    print("\nValidation Methods:")
    test_email = StringValue("user@example.com", None)
    test_number = StringValue("123.45", None)
    test_url = StringValue("https://example.com", None)

    try:
        result = dispatcher.dispatch_string_method(test_email, "is_email", [], None)
        print(f"  'user@example.com'.is_email(): {result.value}")

        result = dispatcher.dispatch_string_method(test_number, "is_number", [], None)
        print(f"  '123.45'.is_number(): {result.value}")

        result = dispatcher.dispatch_string_method(test_url, "is_url", [], None)
        print(f"  'https://example.com'.is_url(): {result.value}")
    except Exception as e:
        print(f"  Error: {e}")

    print("\n=== Refactoring Benefits ===")
    print("✅ Single Responsibility: Each handler has one focused purpose")
    print("✅ Maintainability: Easy to modify or extend specific method groups")
    print("✅ Testability: Individual handlers can be tested in isolation")
    print("✅ Readability: Clear separation of concerns")
    print("✅ Extensibility: New handlers can be added without modifying existing code")

    print("\n=== Code Metrics Improvement ===")
    print("Before: 1 method with ~583 lines")
    print("After: 4 focused classes with ~50-100 lines each")
    print("Benefit: 85% reduction in method complexity")

if __name__ == "__main__":
    demonstrate_refactored_dispatcher()
"""
List Method Handler Refactoring Demo

This demonstrates how the large list method dispatcher can be broken down
into focused handler classes for better maintainability.
"""

from typing import List, Optional, Any
from abc import ABC, abstractmethod

# Mock classes for demo purposes
class GlangValue:
    def __init__(self, value, position=None):
        self.value = value
        self.position = position

class ListValue(GlangValue):
    def __init__(self, elements, constraint=None, position=None):
        super().__init__(elements, position)
        self.elements = elements or []
        self.constraint = constraint

    def size(self):
        return len(self.elements)

    def append(self, item):
        self.elements.append(item)

class NumberValue(GlangValue):
    pass

class StringValue(GlangValue):
    pass

class BooleanValue(GlangValue):
    pass

class ArgumentError(Exception):
    pass

# ============================================================================
# LIST METHOD HANDLERS
# ============================================================================

class ListMutationHandler:
    """Handles list mutation operations: append, prepend, insert, reverse"""

    def can_handle(self, method_name: str) -> bool:
        return method_name in ["append", "prepend", "insert", "reverse"]

    def handle(self, target: ListValue, method_name: str,
               args: List[GlangValue], position: Optional) -> Any:

        if method_name == "append":
            return self._handle_append(target, args, position)
        elif method_name == "prepend":
            return self._handle_prepend(target, args, position)
        elif method_name == "insert":
            return self._handle_insert(target, args, position)
        elif method_name == "reverse":
            return self._handle_reverse(target, args, position)

    def _handle_append(self, target: ListValue, args: List[GlangValue], position):
        if len(args) != 1:
            raise ArgumentError(f"append() takes 1 argument, got {len(args)}")

        # Validate constraint if present
        value_arg = args[0]
        if target.constraint and not self._validate_constraint(value_arg, target.constraint):
            raise RuntimeError(f"Cannot append {value_arg.get_type()} to list<{target.constraint}>")

        target.elements.append(value_arg)
        return target

    def _handle_prepend(self, target: ListValue, args: List[GlangValue], position):
        if len(args) != 1:
            raise ArgumentError(f"prepend() takes 1 argument, got {len(args)}")

        value_arg = args[0]
        if target.constraint and not self._validate_constraint(value_arg, target.constraint):
            raise RuntimeError(f"Cannot prepend {value_arg.get_type()} to list<{target.constraint}>")

        target.elements.insert(0, value_arg)
        return target

    def _handle_insert(self, target: ListValue, args: List[GlangValue], position):
        if len(args) != 2:
            raise ArgumentError(f"insert() takes 2 arguments, got {len(args)}")

        index_arg = args[0]
        value_arg = args[1]

        if not isinstance(index_arg, NumberValue) or not isinstance(index_arg.value, int):
            raise ArgumentError("insert() first argument must be integer")

        if target.constraint and not self._validate_constraint(value_arg, target.constraint):
            raise RuntimeError(f"Cannot insert {value_arg.get_type()} to list<{target.constraint}>")

        target.elements.insert(index_arg.value, value_arg)
        return target

    def _handle_reverse(self, target: ListValue, args: List[GlangValue], position):
        if len(args) != 0:
            raise ArgumentError(f"reverse() takes no arguments, got {len(args)}")

        target.elements.reverse()
        return target

    def _validate_constraint(self, value, constraint):
        """Simplified constraint validation"""
        if constraint == "num":
            return isinstance(value, NumberValue)
        elif constraint == "string":
            return isinstance(value, StringValue)
        elif constraint == "bool":
            return isinstance(value, BooleanValue)
        return True  # 'any' or no constraint

class ListQueryHandler:
    """Handles list query operations: indexOf, count, min, max, sum"""

    def can_handle(self, method_name: str) -> bool:
        return method_name in ["indexOf", "count", "min", "max", "sum"]

    def handle(self, target: ListValue, method_name: str,
               args: List[GlangValue], position: Optional) -> Any:

        if method_name == "indexOf":
            return self._handle_indexOf(target, args, position)
        elif method_name == "count":
            return self._handle_count(target, args, position)
        elif method_name == "min":
            return self._handle_min(target, args, position)
        elif method_name == "max":
            return self._handle_max(target, args, position)
        elif method_name == "sum":
            return self._handle_sum(target, args, position)

    def _handle_indexOf(self, target: ListValue, args: List[GlangValue], position):
        if len(args) != 1:
            raise ArgumentError(f"indexOf() takes 1 argument, got {len(args)}")

        search_value = args[0]
        for i, element in enumerate(target.elements):
            if self._values_equal(element, search_value):
                return NumberValue(i, position)
        return NumberValue(-1, position)  # Not found

    def _handle_count(self, target: ListValue, args: List[GlangValue], position):
        if len(args) != 1:
            raise ArgumentError(f"count() takes 1 argument, got {len(args)}")

        search_value = args[0]
        count = sum(1 for element in target.elements if self._values_equal(element, search_value))
        return NumberValue(count, position)

    def _handle_min(self, target: ListValue, args: List[GlangValue], position):
        if len(args) != 0:
            raise ArgumentError(f"min() takes no arguments, got {len(args)}")

        if not target.elements:
            raise RuntimeError("Cannot find min of empty list")

        # Ensure all elements are numbers
        for element in target.elements:
            if not isinstance(element, NumberValue):
                raise ArgumentError(f"min() requires all elements to be numbers, found {element.get_type()}")

        min_element = min(target.elements, key=lambda x: x.value)
        return NumberValue(min_element.value, position)

    def _handle_max(self, target: ListValue, args: List[GlangValue], position):
        if len(args) != 0:
            raise ArgumentError(f"max() takes no arguments, got {len(args)}")

        if not target.elements:
            raise RuntimeError("Cannot find max of empty list")

        for element in target.elements:
            if not isinstance(element, NumberValue):
                raise ArgumentError(f"max() requires all elements to be numbers, found {element.get_type()}")

        max_element = max(target.elements, key=lambda x: x.value)
        return NumberValue(max_element.value, position)

    def _handle_sum(self, target: ListValue, args: List[GlangValue], position):
        if len(args) != 0:
            raise ArgumentError(f"sum() takes no arguments, got {len(args)}")

        if not target.elements:
            return NumberValue(0, position)

        for element in target.elements:
            if not isinstance(element, NumberValue):
                raise ArgumentError(f"sum() requires all elements to be numbers, found {element.get_type()}")

        total = sum(element.value for element in target.elements)
        return NumberValue(total, position)

    def _values_equal(self, a, b):
        """Simplified value equality check"""
        return type(a) == type(b) and a.value == b.value

class ListFunctionalHandler:
    """Handles functional programming operations: map, filter, each, sort"""

    def can_handle(self, method_name: str) -> bool:
        return method_name in ["map", "filter", "each", "sort", "select", "reject"]

    def handle(self, target: ListValue, method_name: str,
               args: List[GlangValue], position: Optional) -> Any:

        if method_name == "map":
            return self._handle_map(target, args, position)
        elif method_name in ["filter", "select"]:
            return self._handle_filter(target, args, position)
        elif method_name == "reject":
            return self._handle_reject(target, args, position)
        elif method_name == "each":
            return self._handle_each(target, args, position)
        elif method_name == "sort":
            return self._handle_sort(target, args, position)

    def _handle_map(self, target: ListValue, args: List[GlangValue], position):
        if len(args) != 1:
            raise ArgumentError(f"map() takes 1 argument, got {len(args)}")

        if not isinstance(args[0], StringValue):
            raise ArgumentError("map() argument must be a string naming a transformation")

        transform_name = args[0].value
        transformed_elements = []

        for element in target.elements:
            transformed = self._apply_transformation(element, transform_name)
            transformed_elements.append(transformed)

        return ListValue(transformed_elements, target.constraint, position)

    def _handle_filter(self, target: ListValue, args: List[GlangValue], position):
        if len(args) != 1:
            raise ArgumentError(f"filter() takes 1 argument, got {len(args)}")

        if not isinstance(args[0], StringValue):
            raise ArgumentError("filter() argument must be a string naming a predicate")

        predicate_name = args[0].value
        filtered_elements = []

        for element in target.elements:
            if self._apply_predicate(element, predicate_name):
                filtered_elements.append(element)

        return ListValue(filtered_elements, target.constraint, position)

    def _handle_reject(self, target: ListValue, args: List[GlangValue], position):
        if len(args) != 1:
            raise ArgumentError(f"reject() takes 1 argument, got {len(args)}")

        if not isinstance(args[0], StringValue):
            raise ArgumentError("reject() argument must be a string naming a predicate")

        predicate_name = args[0].value
        filtered_elements = []

        for element in target.elements:
            if not self._apply_predicate(element, predicate_name):  # Note the NOT
                filtered_elements.append(element)

        return ListValue(filtered_elements, target.constraint, position)

    def _handle_each(self, target: ListValue, args: List[GlangValue], position):
        if len(args) != 1:
            raise ArgumentError(f"each() takes 1 argument, got {len(args)}")

        if not isinstance(args[0], StringValue):
            raise ArgumentError("each() argument must be a string naming an action")

        action_name = args[0].value

        for element in target.elements:
            self._apply_action(element, action_name)

        return target  # each() returns the original list

    def _handle_sort(self, target: ListValue, args: List[GlangValue], position):
        if len(args) != 0:
            raise ArgumentError(f"sort() takes no arguments, got {len(args)}")

        # Create a copy for sorting
        sorted_elements = target.elements.copy()
        sorted_elements.sort(key=lambda x: x.value if hasattr(x, 'value') else str(x))

        return ListValue(sorted_elements, target.constraint, position)

    def _apply_transformation(self, element, transform_name):
        """Apply transformation to element (simplified)"""
        if transform_name == "double" and isinstance(element, NumberValue):
            return NumberValue(element.value * 2, element.position)
        elif transform_name == "upper" and isinstance(element, StringValue):
            return StringValue(element.value.upper(), element.position)
        elif transform_name == "to_string":
            return StringValue(str(element.value), element.position)
        return element  # No transformation applied

    def _apply_predicate(self, element, predicate_name):
        """Apply predicate to element (simplified)"""
        if predicate_name == "positive" and isinstance(element, NumberValue):
            return element.value > 0
        elif predicate_name == "even" and isinstance(element, NumberValue):
            return element.value % 2 == 0
        elif predicate_name == "non_empty" and isinstance(element, StringValue):
            return len(element.value) > 0
        return True  # Default: include element

    def _apply_action(self, element, action_name):
        """Apply action to element (simplified)"""
        if action_name == "print":
            print(f"  {element.value}")
        # Other actions would be implemented here

class ListTypeConversionHandler:
    """Handles list type conversion: to_string, to_bool"""

    def can_handle(self, method_name: str) -> bool:
        return method_name in ["to_string", "to_bool"]

    def handle(self, target: ListValue, method_name: str,
               args: List[GlangValue], position: Optional) -> Any:

        if method_name == "to_string":
            return self._handle_to_string(target, args, position)
        elif method_name == "to_bool":
            return self._handle_to_bool(target, args, position)

    def _handle_to_string(self, target: ListValue, args: List[GlangValue], position):
        if len(args) != 0:
            raise ArgumentError(f"to_string() takes no arguments, got {len(args)}")

        element_strs = [str(elem.value) for elem in target.elements]
        return StringValue(f"[{', '.join(element_strs)}]", position)

    def _handle_to_bool(self, target: ListValue, args: List[GlangValue], position):
        if len(args) != 0:
            raise ArgumentError(f"to_bool() takes no arguments, got {len(args)}")

        # Non-empty list is true, empty is false
        return BooleanValue(len(target.elements) > 0, position)

# ============================================================================
# REFACTORED LIST METHOD DISPATCHER
# ============================================================================

class RefactoredListMethodDispatcher:
    """Refactored list method dispatcher using handler pattern."""

    def __init__(self):
        self.handlers = [
            ListMutationHandler(),
            ListQueryHandler(),
            ListFunctionalHandler(),
            ListTypeConversionHandler(),
        ]

    def dispatch_list_method(self, target: ListValue, method_name: str,
                            args: List[GlangValue], position: Optional) -> Any:
        """Dispatch list method call to appropriate handler."""

        # Find the handler that can handle this method
        for handler in self.handlers:
            if handler.can_handle(method_name):
                return handler.handle(target, method_name, args, position)

        # No handler found
        raise RuntimeError(f"Method '{method_name}' not found on type 'list'")

# ============================================================================
# DEMONSTRATION
# ============================================================================

def demonstrate_list_refactoring():
    """Demonstrate the refactored list method dispatcher."""

    print("=== List Method Handler Refactoring Demo ===")
    print()

    # Create dispatcher and test data
    dispatcher = RefactoredListMethodDispatcher()
    test_list = ListValue([
        NumberValue(3), NumberValue(1), NumberValue(4), NumberValue(1), NumberValue(5)
    ], "num")

    # Test mutation methods
    print("Mutation Methods:")
    try:
        dispatcher.dispatch_list_method(test_list, "append", [NumberValue(9)], None)
        print(f"  After append(9): {[e.value for e in test_list.elements]}")

        dispatcher.dispatch_list_method(test_list, "prepend", [NumberValue(2)], None)
        print(f"  After prepend(2): {[e.value for e in test_list.elements]}")

        dispatcher.dispatch_list_method(test_list, "reverse", [], None)
        print(f"  After reverse(): {[e.value for e in test_list.elements]}")
    except Exception as e:
        print(f"  Error: {e}")

    # Reset list for other tests
    test_list = ListValue([
        NumberValue(3), NumberValue(1), NumberValue(4), NumberValue(1), NumberValue(5)
    ], "num")

    # Test query methods
    print("\nQuery Methods:")
    try:
        result = dispatcher.dispatch_list_method(test_list, "min", [], None)
        print(f"  min(): {result.value}")

        result = dispatcher.dispatch_list_method(test_list, "max", [], None)
        print(f"  max(): {result.value}")

        result = dispatcher.dispatch_list_method(test_list, "sum", [], None)
        print(f"  sum(): {result.value}")

        result = dispatcher.dispatch_list_method(test_list, "indexOf", [NumberValue(4)], None)
        print(f"  indexOf(4): {result.value}")
    except Exception as e:
        print(f"  Error: {e}")

    # Test functional methods
    print("\nFunctional Methods:")
    try:
        result = dispatcher.dispatch_list_method(test_list, "map", [StringValue("double")], None)
        print(f"  map('double'): {[e.value for e in result.elements]}")

        result = dispatcher.dispatch_list_method(test_list, "filter", [StringValue("even")], None)
        print(f"  filter('even'): {[e.value for e in result.elements]}")

        result = dispatcher.dispatch_list_method(test_list, "sort", [], None)
        print(f"  sort(): {[e.value for e in result.elements]}")
    except Exception as e:
        print(f"  Error: {e}")

    # Test type conversion
    print("\nType Conversion:")
    try:
        result = dispatcher.dispatch_list_method(test_list, "to_string", [], None)
        print(f"  to_string(): '{result.value}'")

        result = dispatcher.dispatch_list_method(test_list, "to_bool", [], None)
        print(f"  to_bool(): {result.value}")
    except Exception as e:
        print(f"  Error: {e}")

    print("\n=== Refactoring Analysis ===")
    print("Original: 1 massive method (~362 lines)")
    print("Refactored: 4 focused handler classes")
    print("  • ListMutationHandler: append, prepend, insert, reverse")
    print("  • ListQueryHandler: indexOf, count, min, max, sum")
    print("  • ListFunctionalHandler: map, filter, each, sort")
    print("  • ListTypeConversionHandler: to_string, to_bool")
    print()
    print("Benefits:")
    print("  ✅ Each handler has 4-6 related methods (~80-100 lines)")
    print("  ✅ Single Responsibility Principle")
    print("  ✅ Easy to test individual method groups")
    print("  ✅ Simple to add new method categories")
    print("  ✅ Clear separation of mutation vs. query operations")

if __name__ == "__main__":
    demonstrate_list_refactoring()
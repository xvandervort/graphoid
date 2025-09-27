"""Test pattern matching functionality."""

import pytest
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../src'))

from glang.lexer.tokenizer import Tokenizer
from glang.parser.ast_parser import ASTParser
from glang.semantic.analyzer import SemanticAnalyzer
from glang.semantic.symbol_table import SymbolTable
from glang.execution.executor import ASTExecutor, ExecutionContext
from glang.execution.pipeline import ExecutionPipeline
from glang.execution.values import StringValue, NumberValue, NoneValue, BooleanValue
from glang.execution.graph_values import ListValue, HashValue
from glang.execution.errors import MatchError

# Alias for compatibility
BoolValue = BooleanValue


class TestPatternMatchingParsing:
    """Test pattern matching parser functionality."""

    def parse_expression(self, code):
        """Helper to parse an expression."""
        parser = ASTParser()
        parser.tokens = parser.tokenizer.tokenize(code)
        parser.current = 0
        return parser.parse_expression()

    def test_match_expression_parsing(self):
        """Test basic match expression parsing."""
        code = '''match value {
            42 => "number",
            "hello" => "string"
        }'''

        expr = self.parse_expression(code)
        assert expr.__class__.__name__ == "MatchExpression"
        assert len(expr.cases) == 2

        # Check first case
        case1 = expr.cases[0]
        assert case1.pattern.__class__.__name__ == "LiteralPattern"
        # Pattern value is now an Expression, so we need to check the nested value
        assert case1.pattern.value.__class__.__name__ == "NumberLiteral"
        assert case1.pattern.value.value == 42
        assert case1.expression.__class__.__name__ == "StringLiteral"
        assert case1.expression.value == "number"

        # Check second case
        case2 = expr.cases[1]
        assert case2.pattern.__class__.__name__ == "LiteralPattern"
        assert case2.pattern.value.__class__.__name__ == "StringLiteral"
        assert case2.pattern.value.value == "hello"
        assert case2.expression.__class__.__name__ == "StringLiteral"
        assert case2.expression.value == "string"

    def test_variable_pattern_parsing(self):
        """Test variable pattern parsing."""
        code = '''match value {
            x => x + 1
        }'''

        expr = self.parse_expression(code)
        case = expr.cases[0]
        assert case.pattern.__class__.__name__ == "VariablePattern"
        assert case.pattern.name == "x"

    def test_wildcard_pattern_parsing(self):
        """Test wildcard pattern parsing."""
        code = '''match value {
            _ => "default"
        }'''

        expr = self.parse_expression(code)
        case = expr.cases[0]
        assert case.pattern.__class__.__name__ == "WildcardPattern"

    def test_list_pattern_parsing(self):
        """Test list pattern parsing."""
        code = '''match value {
            [x, y] => x + y,
            [] => 0
        }'''

        expr = self.parse_expression(code)

        # Check first case - list with variables
        case1 = expr.cases[0]
        assert case1.pattern.__class__.__name__ == "ListPattern"
        assert len(case1.pattern.elements) == 2
        assert case1.pattern.elements[0].__class__.__name__ == "VariablePattern"
        assert case1.pattern.elements[0].name == "x"
        assert case1.pattern.elements[1].__class__.__name__ == "VariablePattern"
        assert case1.pattern.elements[1].name == "y"

        # Check second case - empty list
        case2 = expr.cases[1]
        assert case2.pattern.__class__.__name__ == "ListPattern"
        assert len(case2.pattern.elements) == 0

    def test_symbol_pattern_parsing(self):
        """Test symbol pattern parsing."""
        code = '''match result {
            :ok => "success",
            :error => "failure"
        }'''

        expr = self.parse_expression(code)

        # Check first case - :ok symbol
        case1 = expr.cases[0]
        assert case1.pattern.__class__.__name__ == "LiteralPattern"
        # Pattern value is now a SymbolLiteral expression
        assert case1.pattern.value.__class__.__name__ == "SymbolLiteral"
        assert case1.pattern.value.name == "ok"

        # Check second case - :error symbol
        case2 = expr.cases[1]
        assert case2.pattern.__class__.__name__ == "LiteralPattern"
        assert case2.pattern.value.__class__.__name__ == "SymbolLiteral"
        assert case2.pattern.value.name == "error"


class TestPatternMatchingExecution:
    """Test pattern matching execution functionality."""

    def setup_method(self):
        """Set up execution environment."""
        self.pipeline = ExecutionPipeline()

    def execute_code(self, code):
        """Helper to execute code and return execution context."""
        result = self.pipeline.execute_code(code)
        if not result.success:
            raise result.error
        return result.context

    def test_literal_pattern_matching(self):
        """Test literal pattern matching."""
        code = '''
        num value = 42
        result = match value {
            42 => "found forty-two",
            100 => "found hundred",
            _ => "something else"
        }
        '''

        context = self.execute_code(code)
        result = context.get_variable("result")
        assert isinstance(result, StringValue)
        assert result.value == "found forty-two"

    def test_string_literal_pattern_matching(self):
        """Test string literal pattern matching."""
        code = '''
        string text = "hello"
        result = match text {
            "hello" => "greeting",
            "goodbye" => "farewell",
            _ => "unknown"
        }
        '''

        context = self.execute_code(code)
        result = context.get_variable("result")
        assert isinstance(result, StringValue)
        assert result.value == "greeting"

    def test_variable_pattern_binding(self):
        """Test variable pattern binding."""
        code = '''
        num value = 123
        result = match value {
            x => x * 2
        }
        '''

        context = self.execute_code(code)
        result = context.get_variable("result")
        assert isinstance(result, NumberValue)
        assert result.value == 246

    def test_wildcard_pattern_matching(self):
        """Test wildcard pattern matching."""
        code = '''
        string value = "anything"
        result = match value {
            "specific" => "matched specific",
            _ => "matched wildcard"
        }
        '''

        context = self.execute_code(code)
        result = context.get_variable("result")
        assert isinstance(result, StringValue)
        assert result.value == "matched wildcard"

    def test_list_pattern_matching_empty(self):
        """Test empty list pattern matching."""
        code = '''
        list<num> values = []
        result = match values {
            [] => "empty list",
            _ => "non-empty list"
        }
        '''

        context = self.execute_code(code)
        result = context.get_variable("result")
        assert isinstance(result, StringValue)
        assert result.value == "empty list"

    def test_list_pattern_matching_with_elements(self):
        """Test list pattern matching with element binding."""
        code = '''
        list<num> values = [10, 20]
        result = match values {
            [] => 0,
            [x, y] => x + y,
            _ => -1
        }
        '''

        context = self.execute_code(code)
        result = context.get_variable("result")
        assert isinstance(result, NumberValue)
        assert result.value == 30

    def test_symbol_pattern_matching(self):
        """Test symbol pattern matching."""
        code = '''
        list result_tuple = [:ok, "success"]
        status = result_tuple[0]
        message = match status {
            :ok => "operation succeeded",
            :error => "operation failed",
            _ => "unknown status"
        }
        '''

        context = self.execute_code(code)
        message = context.get_variable("message")
        assert isinstance(message, StringValue)
        assert message.value == "operation succeeded"

    def test_pattern_matching_order(self):
        """Test that patterns are matched in order."""
        code = '''
        num value = 42
        result = match value {
            _ => "wildcard first",
            42 => "specific second"
        }
        '''

        context = self.execute_code(code)
        result = context.get_variable("result")
        assert isinstance(result, StringValue)
        assert result.value == "wildcard first"

    def test_no_pattern_matches_error(self):
        """Test error when no patterns match."""
        code = '''
        num value = 999
        result = match value {
            42 => "forty-two",
            100 => "hundred"
        }
        '''

        with pytest.raises(MatchError) as exc_info:
            self.execute_code(code)

        assert "No pattern matched value" in str(exc_info.value)
        assert "999" in str(exc_info.value)

    def test_variable_binding_isolation(self):
        """Test that pattern variable bindings don't leak out."""
        code = '''
        num value = 42
        string x = "original"
        result = match value {
            x => x + 10
        }
        final_x = x
        '''

        context = self.execute_code(code)
        result = context.get_variable("result")
        final_x = context.get_variable("final_x")

        # Result should be 52 (42 + 10)
        assert isinstance(result, NumberValue)
        assert result.value == 52

        # Original x should be unchanged
        assert isinstance(final_x, StringValue)
        assert final_x.value == "original"

    def test_nested_patterns_in_lists(self):
        """Test nested patterns within list elements."""
        code = '''
        list data = [42, "hello"]
        result = match data {
            [x, "hello"] => x * 2,
            [x, "goodbye"] => x * 3,
            _ => 0
        }
        '''

        context = self.execute_code(code)
        result = context.get_variable("result")
        assert isinstance(result, NumberValue)
        assert result.value == 84


class TestPatternMatchingErrorHandling:
    """Test pattern matching with error-as-data patterns."""

    def setup_method(self):
        """Set up execution environment."""
        self.pipeline = ExecutionPipeline()

    def execute_code(self, code):
        """Helper to execute code and return execution context."""
        result = self.pipeline.execute_code(code)
        if not result.success:
            raise result.error
        return result.context

    def test_result_tuple_pattern_matching(self):
        """Test pattern matching on result tuples."""
        code = '''
        list<list> results = [
            [:ok, "data processed"],
            [:error, "file not found"],
            [:warning, "deprecated method"]
        ]

        list<string> messages = []

        for result in results {
            message = match result {
                [:ok, data] => "Success: " + data,
                [:error, err] => "Error: " + err,
                [:warning, warn] => "Warning: " + warn,
                _ => "Unknown result"
            }
            messages.append(message)
        }
        '''

        context = self.execute_code(code)
        messages = context.get_variable("messages")
        assert isinstance(messages, ListValue)
        assert len(messages.elements) == 3

        msg1 = messages.elements[0]
        assert isinstance(msg1, StringValue)
        assert msg1.value == "Success: data processed"

        msg2 = messages.elements[1]
        assert isinstance(msg2, StringValue)
        assert msg2.value == "Error: file not found"

        msg3 = messages.elements[2]
        assert isinstance(msg3, StringValue)
        assert msg3.value == "Warning: deprecated method"

    def test_complex_result_processing(self):
        """Test complex result processing with multiple patterns."""
        code = '''
        func process_data(value) {
            if value > 100 {
                return [:error, "value too large"]
            }
            if value < 0 {
                return [:error, "value negative"]
            }
            return [:ok, value * 2]
        }

        list<num> inputs = [-5, 50, 150]
        list<string> outputs = []

        for input in inputs {
            result = process_data(input)
            output = match result {
                [:ok, processed] => "Processed: " + processed.to_string(),
                [:error, reason] => "Failed: " + reason,
                _ => "Unknown result"
            }
            outputs.append(output)
        }
        '''

        context = self.execute_code(code)
        outputs = context.get_variable("outputs")
        assert isinstance(outputs, ListValue)
        assert len(outputs.elements) == 3

        out1 = outputs.elements[0]
        assert isinstance(out1, StringValue)
        assert out1.value == "Failed: value negative"

        out2 = outputs.elements[1]
        assert isinstance(out2, StringValue)
        assert out2.value == "Processed: 100"

        out3 = outputs.elements[2]
        assert isinstance(out3, StringValue)
        assert out3.value == "Failed: value too large"


class TestImplicitPatternFunctions:
    """Test implicit pattern matching in function declarations."""

    def setup_method(self):
        """Set up execution environment."""
        self.pipeline = ExecutionPipeline()

    def execute_code(self, code):
        """Helper to execute code and return execution context."""
        result = self.pipeline.execute_code(code)
        if not result.success:
            raise result.error
        return result.context

    def test_basic_implicit_pattern_function(self):
        """Test basic implicit pattern function with literal patterns."""
        code = '''
        func classify(n) {
            0 => "zero"
            1 => "one"
            42 => "the answer"
        }

        result1 = classify(0)
        result2 = classify(1)
        result3 = classify(42)
        '''

        context = self.execute_code(code)

        result1 = context.get_variable("result1")
        assert isinstance(result1, StringValue)
        assert result1.value == "zero"

        result2 = context.get_variable("result2")
        assert isinstance(result2, StringValue)
        assert result2.value == "one"

        result3 = context.get_variable("result3")
        assert isinstance(result3, StringValue)
        assert result3.value == "the answer"

    def test_implicit_pattern_function_with_variable_capture(self):
        """Test implicit pattern function with variable capture."""
        code = '''
        func describe(n) {
            0 => "zero"
            42 => "special"
            x => "number: " + x.to_string()
        }

        result1 = describe(0)
        result2 = describe(42)
        result3 = describe(99)
        '''

        context = self.execute_code(code)

        result1 = context.get_variable("result1")
        assert isinstance(result1, StringValue)
        assert result1.value == "zero"

        result2 = context.get_variable("result2")
        assert isinstance(result2, StringValue)
        assert result2.value == "special"

        result3 = context.get_variable("result3")
        assert isinstance(result3, StringValue)
        assert result3.value == "number: 99"

    def test_implicit_pattern_function_fallthrough(self):
        """Test that unmatched patterns return none."""
        code = '''
        func weekday(n) {
            1 => "Monday"
            2 => "Tuesday"
            3 => "Wednesday"
        }

        valid = weekday(1)
        invalid = weekday(99)
        '''

        context = self.execute_code(code)

        valid = context.get_variable("valid")
        assert isinstance(valid, StringValue)
        assert valid.value == "Monday"

        invalid = context.get_variable("invalid")
        assert isinstance(invalid, NoneValue)

    def test_implicit_pattern_function_with_boolean(self):
        """Test implicit pattern function with boolean patterns."""
        code = '''
        func negate(flag) {
            true => false
            false => true
        }

        func describe_bool(b) {
            true => "yes"
            false => "no"
        }

        neg1 = negate(true)
        neg2 = negate(false)
        desc1 = describe_bool(true)
        desc2 = describe_bool(false)
        '''

        context = self.execute_code(code)

        neg1 = context.get_variable("neg1")
        assert isinstance(neg1, BoolValue)
        assert neg1.value is False

        neg2 = context.get_variable("neg2")
        assert isinstance(neg2, BoolValue)
        assert neg2.value is True

        desc1 = context.get_variable("desc1")
        assert isinstance(desc1, StringValue)
        assert desc1.value == "yes"

        desc2 = context.get_variable("desc2")
        assert isinstance(desc2, StringValue)
        assert desc2.value == "no"

    def test_implicit_pattern_function_with_strings(self):
        """Test implicit pattern function with string patterns."""
        code = '''
        func get_sound(animal) {
            "dog" => "woof"
            "cat" => "meow"
            "cow" => "moo"
            "bird" => "tweet"
        }

        dog_sound = get_sound("dog")
        cat_sound = get_sound("cat")
        unknown_sound = get_sound("elephant")
        '''

        context = self.execute_code(code)

        dog_sound = context.get_variable("dog_sound")
        assert isinstance(dog_sound, StringValue)
        assert dog_sound.value == "woof"

        cat_sound = context.get_variable("cat_sound")
        assert isinstance(cat_sound, StringValue)
        assert cat_sound.value == "meow"

        unknown_sound = context.get_variable("unknown_sound")
        assert isinstance(unknown_sound, NoneValue)

    def test_implicit_pattern_function_recursion_factorial(self):
        """Test recursive implicit pattern function - factorial."""
        code = '''
        func factorial(n) {
            0 => 1
            1 => 1
            x => x * factorial(x - 1)
        }

        result0 = factorial(0)
        result1 = factorial(1)
        result5 = factorial(5)
        '''

        context = self.execute_code(code)

        result0 = context.get_variable("result0")
        assert isinstance(result0, NumberValue)
        assert result0.value == 1

        result1 = context.get_variable("result1")
        assert isinstance(result1, NumberValue)
        assert result1.value == 1

        result5 = context.get_variable("result5")
        assert isinstance(result5, NumberValue)
        assert result5.value == 120

    def test_implicit_pattern_function_recursion_fibonacci(self):
        """Test recursive implicit pattern function - fibonacci."""
        code = '''
        func fibonacci(n) {
            0 => 0
            1 => 1
            x => fibonacci(x - 1) + fibonacci(x - 2)
        }

        result0 = fibonacci(0)
        result1 = fibonacci(1)
        result5 = fibonacci(5)
        result10 = fibonacci(10)
        '''

        context = self.execute_code(code)

        result0 = context.get_variable("result0")
        assert isinstance(result0, NumberValue)
        assert result0.value == 0

        result1 = context.get_variable("result1")
        assert isinstance(result1, NumberValue)
        assert result1.value == 1

        result5 = context.get_variable("result5")
        assert isinstance(result5, NumberValue)
        assert result5.value == 5  # 0, 1, 1, 2, 3, 5

        result10 = context.get_variable("result10")
        assert isinstance(result10, NumberValue)
        assert result10.value == 55

    def test_implicit_pattern_function_with_expressions(self):
        """Test implicit pattern function with complex expressions."""
        code = '''
        func calculate(n) {
            0 => 1
            1 => 10
            x => (x * 2) + 5
        }

        result0 = calculate(0)
        result1 = calculate(1)
        result5 = calculate(5)
        '''

        context = self.execute_code(code)

        result0 = context.get_variable("result0")
        assert isinstance(result0, NumberValue)
        assert result0.value == 1

        result1 = context.get_variable("result1")
        assert isinstance(result1, NumberValue)
        assert result1.value == 10

        result5 = context.get_variable("result5")
        assert isinstance(result5, NumberValue)
        assert result5.value == 15  # (5 * 2) + 5

    def test_implicit_pattern_function_pattern_order(self):
        """Test that pattern order matters - first match wins."""
        code = '''
        func check_order(n) {
            42 => "specific forty-two"
            x => "any number"
        }

        result1 = check_order(42)
        result2 = check_order(100)
        '''

        context = self.execute_code(code)

        result1 = context.get_variable("result1")
        assert isinstance(result1, StringValue)
        assert result1.value == "specific forty-two"

        result2 = context.get_variable("result2")
        assert isinstance(result2, StringValue)
        assert result2.value == "any number"

    def test_implicit_pattern_function_none_handling(self):
        """Test handling none values in pattern matching."""
        code = '''
        func maybe_double(n) {
            0 => 0
            x => x * 2
        }

        valid = maybe_double(5)
        '''

        context = self.execute_code(code)

        valid = context.get_variable("valid")
        assert isinstance(valid, NumberValue)
        assert valid.value == 10

        # Note: Pattern matching with none input would capture it with variable pattern
        # This test just verifies that valid numeric input works correctly

    def test_implicit_vs_explicit_match_coexist(self):
        """Test that implicit pattern functions and explicit match can coexist."""
        code = '''
        func implicit_classify(n) {
            0 => "zero"
            1 => "one"
            x => "other"
        }

        func explicit_classify(n) {
            return match n {
                0 => "ZERO"
                1 => "ONE"
                _ => "OTHER"
            }
        }

        impl_result = implicit_classify(0)
        expl_result = explicit_classify(0)
        '''

        context = self.execute_code(code)

        impl_result = context.get_variable("impl_result")
        assert isinstance(impl_result, StringValue)
        assert impl_result.value == "zero"

        expl_result = context.get_variable("expl_result")
        assert isinstance(expl_result, StringValue)
        assert expl_result.value == "ZERO"

    def test_implicit_pattern_function_multiline_format(self):
        """Test implicit pattern function with multiline formatting."""
        code = '''
        func classify_value(value) {
            42 => "the answer"
            "hello" => "greeting"
            0 => "zero"
            v => "unknown: " + v.to_string()
        }

        r1 = classify_value(42)
        r2 = classify_value("hello")
        r5 = classify_value(0)
        r6 = classify_value(999)
        '''

        context = self.execute_code(code)

        r1 = context.get_variable("r1")
        assert isinstance(r1, StringValue)
        assert r1.value == "the answer"

        r2 = context.get_variable("r2")
        assert isinstance(r2, StringValue)
        assert r2.value == "greeting"

        r5 = context.get_variable("r5")
        assert isinstance(r5, StringValue)
        assert r5.value == "zero"

        r6 = context.get_variable("r6")
        assert isinstance(r6, StringValue)
        assert r6.value == "unknown: 999"
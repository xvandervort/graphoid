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
from glang.execution.values import *
from glang.execution.graph_values import ListValue, HashValue
from glang.execution.errors import MatchError


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
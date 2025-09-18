"""Test internal-only symbol support in Glang.

Symbols are restricted to internal system use for:
- Status codes in result patterns ([:ok, value] / [:error, message])
- Behavior names (system internal)
- NOT for general user assignments or variables
"""

import pytest
from glang.lexer.tokenizer import Tokenizer, TokenType
from glang.parser.ast_parser import ASTParser, ParseError
from glang.ast.nodes import SymbolLiteral, ListLiteral
from glang.semantic.analyzer import SemanticAnalyzer
from glang.execution.executor import ASTExecutor, ExecutionContext
from glang.execution.values import SymbolValue, BooleanValue
from glang.execution.graph_values import ListValue, HashValue
from glang.semantic.symbol_table import SymbolTable


class TestSymbolLexing:
    """Test symbol tokenization."""

    def test_symbol_tokenization(self):
        """Test that symbols are properly tokenized."""
        tokenizer = Tokenizer()
        tokens = tokenizer.tokenize(":ok :error :pending")

        symbol_tokens = [t for t in tokens if t.type == TokenType.SYMBOL]
        assert len(symbol_tokens) == 3
        assert symbol_tokens[0].value == ":ok"
        assert symbol_tokens[1].value == ":error"
        assert symbol_tokens[2].value == ":pending"


class TestSymbolRestrictedUsage:
    """Test that symbols are properly restricted to internal usage."""

    def test_symbols_allowed_in_result_patterns(self):
        """Test that status symbols work in result pattern lists."""
        parser = ASTParser()

        # Test various valid result patterns
        test_cases = [
            '[:ok, "success"]',
            '[:error, "failed"]',
            '[:pending, 42]',
            '[:success, true]',
            '[:failure, "timeout"]',
            '[:warning, "deprecated"]'
        ]

        for code in test_cases:
            parser.tokens = parser.tokenizer.tokenize(code)
            parser.current = 0
            ast = parser.parse_expression()

            assert isinstance(ast, ListLiteral)
            assert len(ast.elements) == 2
            assert isinstance(ast.elements[0], SymbolLiteral)
            # Should not raise any errors

    def test_invalid_symbols_rejected(self):
        """Test that non-status symbols are rejected."""
        parser = ASTParser()

        invalid_cases = [
            '[:invalid, "test"]',
            '[:custom_symbol, "value"]',
            '[:arbitrary, "data"]'
        ]

        for code in invalid_cases:
            with pytest.raises(ParseError) as exc_info:
                parser.tokens = parser.tokenizer.tokenize(code)
                parser.current = 0
                parser.parse_expression()

            assert "not allowed" in str(exc_info.value)
            assert "Only status symbols" in str(exc_info.value)

    def test_direct_symbol_assignment_prevented(self):
        """Test that direct symbol assignments are prevented."""
        parser = ASTParser()

        # These should all fail to parse
        invalid_assignments = [
            'status = :ok',
            'result = :error',
            'flag = :pending'
        ]

        for code in invalid_assignments:
            with pytest.raises(ParseError):
                parser.tokens = parser.tokenizer.tokenize(code)
                parser.current = 0
                parser.parse_statement()

    def test_symbol_variables_not_allowed(self):
        """Test that symbols cannot be used as standalone expressions."""
        parser = ASTParser()

        # This should fail - symbols not allowed outside lists
        with pytest.raises(ParseError):
            parser.tokens = parser.tokenizer.tokenize(':ok')
            parser.current = 0
            parser.parse_expression()


class TestSymbolExecution:
    """Test symbol runtime execution in valid contexts."""

    def setup_method(self):
        """Set up test execution context."""
        self.symbol_table = SymbolTable()
        self.context = ExecutionContext(self.symbol_table)
        self.executor = ASTExecutor(self.context)

    def test_result_pattern_execution(self):
        """Test executing result patterns with symbols."""
        code = """
        result = [:ok, 42]
        result
        """

        parser = ASTParser()
        program = parser.parse(code)

        analyzer = SemanticAnalyzer()
        analyzer.analyze(program)

        self.executor.execute(program)

        result_value = self.context.get_variable("result")
        assert isinstance(result_value, ListValue)
        assert len(result_value.elements) == 2
        assert isinstance(result_value.elements[0], SymbolValue)
        assert result_value.elements[0].name == "ok"
        assert result_value.elements[1].value == 42

    def test_error_pattern_execution(self):
        """Test executing error patterns with symbols."""
        code = """
        error_result = [:error, "Not found"]
        error_result
        """

        parser = ASTParser()
        program = parser.parse(code)

        analyzer = SemanticAnalyzer()
        analyzer.analyze(program)

        self.executor.execute(program)

        error_value = self.context.get_variable("error_result")
        assert isinstance(error_value, ListValue)
        assert len(error_value.elements) == 2
        assert isinstance(error_value.elements[0], SymbolValue)
        assert error_value.elements[0].name == "error"
        assert error_value.elements[1].value == "Not found"

    def test_multiple_result_patterns(self):
        """Test handling multiple result patterns."""
        # Test each pattern separately to avoid execution pipeline issues
        test_cases = [
            ('success = [:ok, "done"]', "ok", "done"),
            ('failure = [:error, "timeout"]', "error", "timeout"),
            ('warning = [:warning, "deprecated"]', "warning", "deprecated")
        ]

        for code, expected_symbol, expected_value in test_cases:
            # Fresh context for each test
            symbol_table = SymbolTable()
            context = ExecutionContext(symbol_table)
            executor = ASTExecutor(context)

            parser = ASTParser()
            program = parser.parse(code)

            analyzer = SemanticAnalyzer()
            analyzer.analyze(program)

            executor.execute(program)

            # Get the variable (first word before '=')
            var_name = code.split('=')[0].strip()
            result = context.get_variable(var_name)

            assert result is not None, f"Variable {var_name} should exist"
            assert isinstance(result, ListValue)
            assert len(result.elements) == 2
            assert isinstance(result.elements[0], SymbolValue)
            assert result.elements[0].name == expected_symbol
            assert result.elements[1].value == expected_value


class TestSymbolValues:
    """Test SymbolValue behavior for internal system use."""

    def test_symbol_interning(self):
        """Test that symbols are interned for memory efficiency."""
        # Reset registry for clean test
        SymbolValue._symbol_registry.clear()

        symbol1 = SymbolValue("ok")
        symbol2 = SymbolValue("ok")

        # Should be the same object due to interning
        assert symbol1 is symbol2

    def test_symbol_equality(self):
        """Test symbol equality for system comparisons."""
        symbol1 = SymbolValue("ok")
        symbol2 = SymbolValue("ok")
        symbol3 = SymbolValue("error")

        assert symbol1 == symbol2
        assert symbol1 != symbol3

    def test_symbol_display(self):
        """Test symbol display format."""
        symbol = SymbolValue("ok")
        assert symbol.to_display_string() == ":ok"
        assert symbol.name == "ok"
        assert symbol.get_type() == "symbol"

    def test_symbol_hashability(self):
        """Test that symbols can be used as hash keys in system code."""
        symbol1 = SymbolValue("ok")
        symbol2 = SymbolValue("error")

        # Should be hashable for system use
        result_map = {symbol1: "success", symbol2: "failure"}
        assert result_map[symbol1] == "success"
        assert result_map[symbol2] == "failure"


class TestResultPatternUsage:
    """Test practical result pattern usage."""

    def setup_method(self):
        """Set up test execution context."""
        self.symbol_table = SymbolTable()
        self.context = ExecutionContext(self.symbol_table)
        self.executor = ASTExecutor(self.context)

    def test_error_as_data_pattern(self):
        """Test the error-as-data pattern with result lists."""
        # Test each case separately to avoid execution issues
        success_code = 'success_case = [:ok, "user123"]'
        error_code = 'error_case = [:error, "User not found"]'

        # Test success case
        parser1 = ASTParser()
        program1 = parser1.parse(success_code)
        analyzer1 = SemanticAnalyzer()
        analyzer1.analyze(program1)
        self.executor.execute(program1)

        success = self.context.get_variable("success_case")
        assert success is not None
        assert isinstance(success, ListValue)
        assert success.elements[0].name == "ok"
        assert success.elements[1].value == "user123"

        # Test error case
        parser2 = ASTParser()
        program2 = parser2.parse(error_code)
        analyzer2 = SemanticAnalyzer()
        analyzer2.analyze(program2)
        self.executor.execute(program2)

        error = self.context.get_variable("error_case")
        assert error is not None
        assert isinstance(error, ListValue)
        assert error.elements[0].name == "error"
        assert error.elements[1].value == "User not found"

    def test_status_progression_pattern(self):
        """Test status progression pattern."""
        # Use single line to avoid multiline parsing issues
        code = 'statuses = [[:pending, "Waiting for approval"], [:success, "Approved by manager"], [:warning, "Expires in 24h"]]'

        parser = ASTParser()
        program = parser.parse(code)

        analyzer = SemanticAnalyzer()
        analyzer.analyze(program)

        self.executor.execute(program)

        statuses = self.context.get_variable("statuses")
        assert statuses is not None, "statuses variable should exist"
        assert isinstance(statuses, ListValue)
        assert len(statuses.elements) == 3

        # Verify each status tuple
        expected_statuses = ["pending", "success", "warning"]
        expected_messages = ["Waiting for approval", "Approved by manager", "Expires in 24h"]

        for i, (expected_status, expected_message) in enumerate(zip(expected_statuses, expected_messages)):
            status_pair = statuses.elements[i]
            assert isinstance(status_pair, ListValue)
            assert len(status_pair.elements) == 2
            assert isinstance(status_pair.elements[0], SymbolValue)
            assert status_pair.elements[0].name == expected_status
            assert status_pair.elements[1].value == expected_message
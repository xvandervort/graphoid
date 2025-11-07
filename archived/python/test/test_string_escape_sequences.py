"""Tests for string literal escape sequence handling, especially quote characters."""

import pytest
from glang.execution.pipeline import ExecutionSession
from glang.ast.nodes import StringLiteral
from glang.execution.executor import ASTExecutor, ExecutionContext
from glang.ast.nodes import SourcePosition
from glang.execution.values import StringValue
from glang.semantic.symbol_table import SymbolTable


class TestStringEscapeSequences:
    """Test proper handling of escape sequences in string literals."""

    def setup_method(self):
        """Set up test environment."""
        self.session = ExecutionSession()

    def test_single_escaped_quote(self):
        """Test that a single escaped quote character works correctly."""
        # Test the critical bug fix: \"" should become a single quote character
        result = self.session.execute_statement('quote = "\\""')
        assert result.success

        # Verify the value is a single quote character
        result = self.session.execute_statement('quote')
        assert result.success
        assert result.value.value == '"'

        # Verify the length is 1
        result = self.session.execute_statement('quote.length()')
        assert result.success
        assert result.value.value == 1

    def test_multiple_escaped_quotes(self):
        """Test multiple escaped quote characters."""
        # Three escaped quotes should become three quote characters
        result = self.session.execute_statement('quotes = "\\"\\"\\""')
        assert result.success

        result = self.session.execute_statement('quotes')
        assert result.success
        assert result.value.value == '"""'

        result = self.session.execute_statement('quotes.length()')
        assert result.success
        assert result.value.value == 3

    def test_escaped_quote_in_context(self):
        """Test escaped quotes within a larger string."""
        # HTML-like string with escaped quotes
        result = self.session.execute_statement('html = "<div class=\\"test\\">content</div>"')
        assert result.success

        result = self.session.execute_statement('html')
        assert result.success
        assert result.value.value == '<div class="test">content</div>'

    def test_mixed_escape_sequences(self):
        """Test various escape sequences work correctly."""
        # Test multiple escape types
        result = self.session.execute_statement('mixed = "Line1\\nLine2\\tTabbed"')
        assert result.success

        result = self.session.execute_statement('mixed')
        assert result.success
        assert result.value.value == 'Line1\nLine2\tTabbed'

    def test_escaped_backslash(self):
        """Test escaped backslash."""
        result = self.session.execute_statement('backslash = "\\\\"')
        assert result.success

        result = self.session.execute_statement('backslash')
        assert result.success
        assert result.value.value == '\\'

        result = self.session.execute_statement('backslash.length()')
        assert result.success
        assert result.value.value == 1

    def test_single_quote_strings(self):
        """Test escape sequences in single-quoted strings."""
        # Single quote in single-quoted string
        result = self.session.execute_statement("quote = '\\''")
        assert result.success

        result = self.session.execute_statement('quote')
        assert result.success
        assert result.value.value == "'"

        result = self.session.execute_statement('quote.length()')
        assert result.success
        assert result.value.value == 1

    def test_empty_string(self):
        """Test that empty strings work correctly."""
        result = self.session.execute_statement('empty = ""')
        assert result.success

        result = self.session.execute_statement('empty')
        assert result.success
        assert result.value.value == ''

        result = self.session.execute_statement('empty.length()')
        assert result.success
        assert result.value.value == 0

    def test_complex_escape_combinations(self):
        """Test complex combinations of escape sequences."""
        # String with quotes, backslashes, and newlines
        result = self.session.execute_statement('complex = "Say \\"Hello\\\\World\\"\\n"')
        assert result.success

        result = self.session.execute_statement('complex')
        assert result.success
        assert result.value.value == 'Say "Hello\\World"\n'

    def test_no_double_processing(self):
        """Test that strings are not processed twice (regression test)."""
        # This tests the specific bug where the executor was double-processing strings
        # A string containing just a quote character should remain a quote
        result = self.session.execute_statement('just_quote = "\\""')
        assert result.success

        # If double-processing occurs, this would become empty string
        result = self.session.execute_statement('just_quote')
        assert result.success
        assert result.value.value == '"'
        assert len(result.value.value) == 1

    def test_ast_direct_construction_compatibility(self):
        """Test that directly constructed AST nodes still work (for test compatibility)."""
        # Test that old test patterns still work
        symbol_table = SymbolTable()
        context = ExecutionContext(symbol_table)
        executor = ASTExecutor(context)

        # Direct AST construction with raw quoted string (like tests do)
        str_node = StringLiteral('"hello"', SourcePosition(1, 1))
        result = executor.execute(str_node)

        assert isinstance(result, StringValue)
        assert result.value == "hello"  # Quotes should be removed

    def test_processed_flag_behavior(self):
        """Test that the processed flag works correctly."""
        symbol_table = SymbolTable()
        context = ExecutionContext(symbol_table)
        executor = ASTExecutor(context)

        # Test with processed=True (parser-created)
        str_node = StringLiteral('test', SourcePosition(1, 1), processed=True)
        result = executor.execute(str_node)
        assert isinstance(result, StringValue)
        assert result.value == "test"

        # Test with processed=False (test-created, needs processing)
        str_node = StringLiteral('"test"', SourcePosition(1, 1), processed=False)
        result = executor.execute(str_node)
        assert isinstance(result, StringValue)
        assert result.value == "test"

        # Test quote character with processed=True
        str_node = StringLiteral('"', SourcePosition(1, 1), processed=True)
        result = executor.execute(str_node)
        assert isinstance(result, StringValue)
        assert result.value == '"'

    def test_json_like_escaping(self):
        """Test JSON-like escape sequences."""
        # JSON string with various escapes
        result = self.session.execute_statement('json_str = "{\\"name\\": \\"Alice\\", \\"age\\": 30}"')
        assert result.success

        result = self.session.execute_statement('json_str')
        assert result.success
        assert result.value.value == '{"name": "Alice", "age": 30}'

    def test_path_like_strings(self):
        """Test path strings with backslashes."""
        # Windows-style path (with escaped backslashes)
        result = self.session.execute_statement('path = "C:\\\\Users\\\\Alice\\\\Documents"')
        assert result.success

        result = self.session.execute_statement('path')
        assert result.success
        assert result.value.value == 'C:\\Users\\Alice\\Documents'

    def test_special_characters_preservation(self):
        """Test that non-escape characters are preserved."""
        # String with characters that look like escapes but aren't
        result = self.session.execute_statement('text = "Price: \\$50 @ 10% off"')
        assert result.success

        result = self.session.execute_statement('text')
        assert result.success
        # \$ is not a recognized escape, so backslash is kept
        assert '\\$50' in result.value.value or '$50' in result.value.value
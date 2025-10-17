"""Test logical operator precedence in the parser."""

import pytest
from glang.parser.ast_parser import ASTParser
from glang.ast.nodes import *
from glang.semantic.analyzer import SemanticAnalyzer
from glang.execution.executor import ASTExecutor, ExecutionContext
from glang.semantic.symbol_table import SymbolTable


class TestLogicalOperatorPrecedence:
    """Test that logical operators have correct precedence."""

    def test_or_has_lower_precedence_than_and(self):
        """Test that 'or' has lower precedence than 'and'."""
        parser = ASTParser()

        # a and b or c should parse as (a and b) or c
        ast = parser.parse("a = true and false or true")
        analyzer = SemanticAnalyzer()
        result = analyzer.analyze(ast)
        context = ExecutionContext(result.symbol_table)
        executor = ASTExecutor(context)
        executor.execute(result.ast)

        # true and false = false; false or true = true
        assert context.variables["a"].value == True

    def test_comparison_has_higher_precedence_than_logical(self):
        """Test that comparison operators have higher precedence than logical operators."""
        parser = ASTParser()

        # a == 1 or b == 2 should parse as (a == 1) or (b == 2)
        code = """
a = 1
b = 2
result = a == 1 or b == 3
"""
        ast = parser.parse(code)
        analyzer = SemanticAnalyzer()
        result = analyzer.analyze(ast)
        context = ExecutionContext(result.symbol_table)
        executor = ASTExecutor(context)
        executor.execute(result.ast)

        # a == 1 is true, b == 3 is false, true or false = true
        assert context.variables["result"].value == True

    def test_complex_expression_with_comparisons_and_logical(self):
        """Test complex expressions with both comparisons and logical operators."""
        parser = ASTParser()

        code = """
x = 5
y = 10
z = 15
result = x < y and y < z or x > z
"""
        ast = parser.parse(code)
        analyzer = SemanticAnalyzer()
        result = analyzer.analyze(ast)
        context = ExecutionContext(result.symbol_table)
        executor = ASTExecutor(context)
        executor.execute(result.ast)

        # x < y is true, y < z is true, true and true = true
        # x > z is false, true or false = true
        assert context.variables["result"].value == True

    def test_parentheses_override_precedence(self):
        """Test that parentheses correctly override precedence."""
        parser = ASTParser()

        code = """
a = true
b = false
c = true
result1 = a or b and c
result2 = (a or b) and c
"""
        ast = parser.parse(code)
        analyzer = SemanticAnalyzer()
        result = analyzer.analyze(ast)
        context = ExecutionContext(result.symbol_table)
        executor = ASTExecutor(context)
        executor.execute(result.ast)

        # result1: b and c = false, a or false = true
        assert context.variables["result1"].value == True

        # result2: a or b = true, true and c = true
        assert context.variables["result2"].value == True

    def test_operator_synonyms_have_same_precedence(self):
        """Test that && and || have the same precedence as 'and' and 'or'."""
        parser = ASTParser()

        code = """
a = 1
b = 2
result1 = a == 1 || b == 2
result2 = a == 1 or b == 2
result3 = a == 1 && b == 2
result4 = a == 1 and b == 2
"""
        ast = parser.parse(code)
        analyzer = SemanticAnalyzer()
        result = analyzer.analyze(ast)
        context = ExecutionContext(result.symbol_table)
        executor = ASTExecutor(context)
        executor.execute(result.ast)

        # Both || and or should work the same
        assert context.variables["result1"].value == True
        assert context.variables["result2"].value == True

        # Both && and and should work the same
        # a==1 is true, b==2 is true, so true && true = true
        assert context.variables["result3"].value == True
        assert context.variables["result4"].value == True

    def test_chained_comparisons_with_logical(self):
        """Test that chained comparisons work correctly with logical operators."""
        parser = ASTParser()

        code = """
x = 5
y = 10
result = x == 5 and y == 10 or x == 10 and y == 5
"""
        ast = parser.parse(code)
        analyzer = SemanticAnalyzer()
        result = analyzer.analyze(ast)
        context = ExecutionContext(result.symbol_table)
        executor = ASTExecutor(context)
        executor.execute(result.ast)

        # (x == 5 and y == 10) or (x == 10 and y == 5)
        # (true and true) or (false and false)
        # true or false = true
        assert context.variables["result"].value == True

    def test_arithmetic_has_higher_precedence_than_comparison(self):
        """Test that arithmetic operators have higher precedence than comparisons."""
        parser = ASTParser()

        code = """
a = 5
b = 3
result = a + b == 8
"""
        ast = parser.parse(code)
        analyzer = SemanticAnalyzer()
        result = analyzer.analyze(ast)
        context = ExecutionContext(result.symbol_table)
        executor = ASTExecutor(context)
        executor.execute(result.ast)

        # a + b = 8, 8 == 8 = true
        assert context.variables["result"].value == True

    def test_mixed_logical_operators(self):
        """Test mixing and/or operators in complex expressions."""
        parser = ASTParser()

        code = """
a = true
b = false
c = true
d = false
result = a and b or c and d or a and c
"""
        ast = parser.parse(code)
        analyzer = SemanticAnalyzer()
        result = analyzer.analyze(ast)
        context = ExecutionContext(result.symbol_table)
        executor = ASTExecutor(context)
        executor.execute(result.ast)

        # (a and b) or (c and d) or (a and c)
        # (true and false) or (true and false) or (true and true)
        # false or false or true = true
        assert context.variables["result"].value == True

    def test_not_operator_precedence(self):
        """Test that 'not' operator has higher precedence than 'and' and 'or'."""
        parser = ASTParser()

        code = """
a = true
b = false
result = !a or b
"""
        ast = parser.parse(code)
        analyzer = SemanticAnalyzer()
        result = analyzer.analyze(ast)
        context = ExecutionContext(result.symbol_table)
        executor = ASTExecutor(context)
        executor.execute(result.ast)

        # !a = false, false or false = false
        assert context.variables["result"].value == False

    def test_real_world_example_from_claude_md(self):
        """Test the specific example mentioned in CLAUDE.md that was broken."""
        parser = ASTParser()

        code = """
a = 1
b = 2
# This should now work without parentheses!
result = a == 1 or b == 2
"""
        ast = parser.parse(code)
        analyzer = SemanticAnalyzer()
        result = analyzer.analyze(ast)
        context = ExecutionContext(result.symbol_table)
        executor = ASTExecutor(context)
        executor.execute(result.ast)

        # a == 1 is true, b == 2 is true, true or true = true
        assert context.variables["result"].value == True

    def test_short_circuit_evaluation_with_precedence(self):
        """Test that short-circuit evaluation works correctly with precedence."""
        parser = ASTParser()

        code = """
# Test with function that would cause error if called
func will_cause_error() {
    error_val = 1 / 0  # Division by zero
    return false
}

# Test OR short-circuit - should not evaluate right side
result1 = true or will_cause_error()

# Test AND short-circuit - should not evaluate right side
result2 = false and will_cause_error()

# Test that it DOES evaluate when needed
result3 = false or true  # Should evaluate both sides
result4 = true and true   # Should evaluate both sides
"""
        ast = parser.parse(code)
        analyzer = SemanticAnalyzer()
        result = analyzer.analyze(ast)
        context = ExecutionContext(result.symbol_table)
        executor = ASTExecutor(context)

        # Execute - should not raise any errors due to short-circuit
        executor.execute(result.ast)

        # Verify short-circuit worked
        assert context.variables["result1"].value == True  # true or (not evaluated)
        assert context.variables["result2"].value == False  # false and (not evaluated)
        assert context.variables["result3"].value == True  # false or true
        assert context.variables["result4"].value == True  # true and true
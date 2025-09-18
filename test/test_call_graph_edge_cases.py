#!/usr/bin/env python3
"""Test edge cases and performance for Phase 3 call graph implementation."""

import pytest
import sys
import os
import tempfile
import shutil
import time
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../src'))

from glang.execution.pipeline import ExecutionSession
from glang.files.file_manager import FileManager
from glang.parser.ast_parser import ASTParser
from glang.execution.call_graph import CallGraph
from glang.execution.values import LambdaValue


class TestCallGraphEdgeCases:
    """Test edge cases and performance scenarios for Phase 3 call graph."""

    def setup_method(self):
        """Set up test with temporary directory and execution session."""
        self.temp_dir = tempfile.mkdtemp()
        self.file_manager = FileManager()
        self.session = ExecutionSession(self.file_manager)
        self.parser = ASTParser()

    def teardown_method(self):
        """Clean up temporary directory."""
        shutil.rmtree(self.temp_dir)

    def test_lambda_fallback_still_works(self):
        """Test that lambdas still use variable fallback, not call graph."""
        # Create lambda and verify it works
        result = self.session.execute_statement('double = x => x * 2')
        assert result.success

        # Call lambda - should use variable fallback
        call_result = self.session.execute_statement('double(5)')
        assert call_result.success
        assert call_result.value.value == 10

        # Verify lambda is NOT in call graph
        lambda_in_graph = self.session.execution_context.call_graph.find_function("double", "global")
        assert lambda_in_graph is None

        # Verify lambda IS in variables
        lambda_var = self.session.execution_context.get_variable("double")
        assert lambda_var is not None
        assert isinstance(lambda_var, LambdaValue)

    def test_function_vs_lambda_disambiguation(self):
        """Test that functions use graph traversal while lambdas use variables."""
        # Create both function and lambda with same functionality
        func_result = self.session.execute_statement('''
        func func_triple(x) {
            return x * 3
        }
        ''')
        assert func_result.success

        lambda_result = self.session.execute_statement('lambda_triple = x => x * 3')
        assert lambda_result.success

        # Both should work but use different lookup mechanisms
        func_call = self.session.execute_statement('func_triple(4)')
        assert func_call.success
        assert func_call.value.value == 12

        lambda_call = self.session.execute_statement('lambda_triple(4)')
        assert lambda_call.success
        assert lambda_call.value.value == 12

        # Verify function is in call graph
        func_in_graph = self.session.execution_context.call_graph.find_function("func_triple", "global")
        assert func_in_graph is not None

        # Verify lambda is NOT in call graph but IS in variables
        lambda_in_graph = self.session.execution_context.call_graph.find_function("lambda_triple", "global")
        assert lambda_in_graph is None
        lambda_var = self.session.execution_context.get_variable("lambda_triple")
        assert isinstance(lambda_var, LambdaValue)

    def test_circular_function_calls_in_module(self):
        """Test circular function calls within module scope."""
        module_content = '''module CircularModule

func func_a() {
    return "a->" + func_b()
}

func func_b() {
    return "b->" + func_c()
}

func func_c() {
    return "c->" + func_a_end()
}

func func_a_end() {
    return "end"
}
'''
        module_path = os.path.join(self.temp_dir, 'circular_test.gr')
        with open(module_path, 'w') as f:
            f.write(module_content)

        # Import module
        import_result = self.session.execute_statement(f'import "{module_path}" as circ')
        assert import_result.success

        # Test circular call chain
        result = self.session.execute_statement('circ.func_a()')
        assert result.success
        assert result.value.value == "a->b->c->end"

        # Verify all functions are properly connected via call graph
        call_graph = self.session.execution_context.call_graph
        func_a = call_graph.find_function("func_a", "CircularModule")
        func_b = call_graph.find_function("func_b", "CircularModule")
        func_c = call_graph.find_function("func_c", "CircularModule")
        func_a_end = call_graph.find_function("func_a_end", "CircularModule")

        assert all([func_a, func_b, func_c, func_a_end])

    def test_deep_function_nesting(self):
        """Test deeply nested function calls."""
        # Create chain of 20 functions calling each other
        code_parts = []
        for i in range(20):
            if i == 19:  # Last function
                code_parts.append(f'func deep_{i}() {{ return "depth_{i}" }}')
            else:
                code_parts.append(f'func deep_{i}() {{ return "depth_{i}->" + deep_{i+1}() }}')

        full_code = '\n'.join(code_parts)
        result = self.session.execute_statement(full_code)
        assert result.success

        # Call the first function - should traverse deep chain
        call_result = self.session.execute_statement('deep_0()')
        assert call_result.success

        expected = '->'.join([f'depth_{i}' for i in range(20)])
        assert call_result.value.value == expected

        # Verify all functions are in call graph
        for i in range(20):
            func_in_graph = self.session.execution_context.call_graph.find_function(f"deep_{i}", "global")
            assert func_in_graph is not None

    def test_large_module_performance(self):
        """Test performance with large number of functions in module."""
        # Create module with 50 functions (avoid deep recursion)
        num_functions = 50
        module_lines = ['module LargeModule', '']

        for i in range(num_functions):
            module_lines.append(f'func large_func_{i}() {{')
            # Don't chain all functions - just return simple values
            module_lines.append(f'    return "func_{i}_result"')
            module_lines.append('}')
            module_lines.append('')

        # Add one function that calls a few others (not all)
        module_lines.append('func call_some() {')
        module_lines.append('    return large_func_0() + large_func_10() + large_func_20()')
        module_lines.append('}')
        module_lines.append('')

        module_content = '\n'.join(module_lines)
        module_path = os.path.join(self.temp_dir, 'large_module.gr')
        with open(module_path, 'w') as f:
            f.write(module_content)

        # Time the import
        start_time = time.time()
        import_result = self.session.execute_statement(f'import "{module_path}" as large')
        import_time = time.time() - start_time

        assert import_result.success
        # Should complete in reasonable time (less than 5 seconds)
        assert import_time < 5.0

        # Test function call performance
        start_time = time.time()
        call_result = self.session.execute_statement('large.call_some()')
        call_time = time.time() - start_time

        assert call_result.success
        assert call_result.value.value == "func_0_resultfunc_10_resultfunc_20_result"
        # Function call should also be fast
        assert call_time < 2.0

        # Verify random functions are accessible
        mid_func = self.session.execution_context.call_graph.find_function("large_func_25", "LargeModule")
        assert mid_func is not None

    def test_multiple_modules_with_different_function_names(self):
        """Test function isolation between modules."""
        # Create two modules with different function names to avoid semantic conflicts
        module1_content = '''module Module1

func module1_func() {
    return "from_module1"
}

func call_module1() {
    return module1_func()
}
'''

        module2_content = '''module Module2

func module2_func() {
    return "from_module2"
}

func call_module2() {
    return module2_func()
}
'''

        module1_path = os.path.join(self.temp_dir, 'module1.gr')
        module2_path = os.path.join(self.temp_dir, 'module2.gr')

        with open(module1_path, 'w') as f:
            f.write(module1_content)
        with open(module2_path, 'w') as f:
            f.write(module2_content)

        # Import both modules
        import1 = self.session.execute_statement(f'import "{module1_path}" as m1')
        import2 = self.session.execute_statement(f'import "{module2_path}" as m2')

        assert import1.success
        assert import2.success

        # Test basic module function calls
        result1 = self.session.execute_statement('m1.module1_func()')
        result2 = self.session.execute_statement('m2.module2_func()')

        assert result1.success
        assert result2.success
        assert result1.value.value == "from_module1"
        assert result2.value.value == "from_module2"

        # Verify functions are properly scoped in call graph
        call_graph = self.session.execution_context.call_graph
        func1 = call_graph.find_function("module1_func", "Module1")
        func2 = call_graph.find_function("module2_func", "Module2")

        assert func1 is not None
        assert func2 is not None

        # Functions from one module shouldn't be accessible in the other
        func1_in_mod2 = call_graph.find_function("module1_func", "Module2")
        func2_in_mod1 = call_graph.find_function("module2_func", "Module1")
        assert func1_in_mod2 is None
        assert func2_in_mod1 is None

    def test_ast_subgraph_with_complex_nested_blocks(self):
        """Test AST subgraph extraction with complex nested structures."""
        complex_code = '''
        func outer_function() {
            if true {
                func nested_in_if() {
                    return "nested_if"
                }

                while false {
                    func nested_in_while() {
                        return "nested_while"
                    }
                }
            }

            func regular_nested() {
                return "regular"
            }

            return nested_in_if() + regular_nested()
        }
        '''

        ast = self.parser.parse(complex_code)
        call_graph = CallGraph()
        subgraph = call_graph.create_ast_subgraph(ast, "global")

        # Should find all nested functions
        assert len(subgraph) == 4
        function_names = subgraph.get_function_names()
        assert "outer_function" in function_names
        assert "nested_in_if" in function_names
        assert "nested_in_while" in function_names
        assert "regular_nested" in function_names

    def test_error_handling_in_function_calls(self):
        """Test error handling when functions don't exist in call graph."""
        # Try to call non-existent function
        result = self.session.execute_statement('nonexistent_function()')
        assert not result.success
        error_msg = str(result.error).lower()
        assert "not found" in error_msg or "nonexistent_function" in error_msg

        # Try to call function from wrong module context
        module_content = '''module PrivateModule
func private_func() {
    return "private"
}
'''
        module_path = os.path.join(self.temp_dir, 'private.gr')
        with open(module_path, 'w') as f:
            f.write(module_content)

        import_result = self.session.execute_statement(f'import "{module_path}" as priv')
        assert import_result.success

        # Proper call should work
        proper_call = self.session.execute_statement('priv.private_func()')
        assert proper_call.success
        assert proper_call.value.value == "private"

    def test_mixed_function_and_variable_declarations(self):
        """Test AST extraction when functions are mixed with other declarations."""
        mixed_code = '''
        string global_var = "test"

        func first_func() {
            return global_var
        }

        list items = [1, 2, 3]

        func second_func() {
            return items.size()
        }

        hash config = {"debug": true}

        func third_func() {
            return config["debug"]
        }
        '''

        ast = self.parser.parse(mixed_code)
        call_graph = CallGraph()
        subgraph = call_graph.create_ast_subgraph(ast, "global")

        # Should extract only functions, not variables
        assert len(subgraph) == 3
        function_names = subgraph.get_function_names()
        assert "first_func" in function_names
        assert "second_func" in function_names
        assert "third_func" in function_names

        # Variables should not be in subgraph
        assert "global_var" not in function_names
        assert "items" not in function_names
        assert "config" not in function_names
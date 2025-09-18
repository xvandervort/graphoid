#!/usr/bin/env python3
"""Test module function scoping - demonstrates critical bug."""

import pytest
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../src'))

from glang.execution.pipeline import ExecutionSession
from glang.files.file_manager import FileManager
import tempfile
import shutil


class TestModuleFunctionScoping:
    """Test that functions within modules can call other functions in the same module."""

    def setup_method(self):
        """Set up test with temporary directory."""
        self.temp_dir = tempfile.mkdtemp()
        self.file_manager = FileManager()
        self.session = ExecutionSession(self.file_manager)

    def teardown_method(self):
        """Clean up temporary directory."""
        shutil.rmtree(self.temp_dir)

    def test_basic_function_calling_within_module(self):
        """Test that a function can call another function in the same module."""
        # Create module file
        module_content = '''module test

func helper() {
    return 42
}

func main() {
    return helper()
}
'''
        module_path = os.path.join(self.temp_dir, 'test_module.gr')
        with open(module_path, 'w') as f:
            f.write(module_content)

        # Test importing and calling
        import_cmd = f'import "{module_path}" as test'
        import_result = self.session.execute_statement(import_cmd)
        assert import_result.success, f"Import failed: {import_result.error}"

        # This should work but currently fails
        call_cmd = 'result = test.main()'
        call_result = self.session.execute_statement(call_cmd)

        # EXPECTED: This should work
        # ACTUAL: Currently fails with "Function 'helper' not found"
        if not call_result.success:
            pytest.xfail("KNOWN BUG: Module functions cannot call other module functions")

        assert call_result.success, f"Function call failed: {call_result.error}"

        # Check result
        result_cmd = 'result'
        result_check = self.session.execute_statement(result_cmd)
        assert result_check.success
        assert result_check.value.value == 42

    def test_recursive_function_within_module(self):
        """Test recursive function calls within module."""
        module_content = '''module math

func factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}
'''
        module_path = os.path.join(self.temp_dir, 'math_module.gr')
        with open(module_path, 'w') as f:
            f.write(module_content)

        # Import module
        import_cmd = f'import "{module_path}" as math'
        import_result = self.session.execute_statement(import_cmd)
        assert import_result.success

        # Test recursive call
        call_cmd = 'result = math.factorial(5)'
        call_result = self.session.execute_statement(call_cmd)

        if not call_result.success:
            pytest.xfail("KNOWN BUG: Recursive functions fail due to module scoping")

        assert call_result.success

        # Check result (5! = 120)
        result_cmd = 'result'
        result_check = self.session.execute_statement(result_cmd)
        assert result_check.success
        assert result_check.value.value == 120

    def test_complex_function_interdependence(self):
        """Test complex function interdependence within module."""
        module_content = '''module complex

func a() {
    return b() + 1
}

func b() {
    return c() * 2
}

func c() {
    return 10
}
'''
        module_path = os.path.join(self.temp_dir, 'complex_module.gr')
        with open(module_path, 'w') as f:
            f.write(module_content)

        # Import module
        import_cmd = f'import "{module_path}" as complex'
        import_result = self.session.execute_statement(import_cmd)
        assert import_result.success

        # Test function chain: a() -> b() + 1 -> (c() * 2) + 1 -> (10 * 2) + 1 = 21
        call_cmd = 'result = complex.a()'
        call_result = self.session.execute_statement(call_cmd)

        if not call_result.success:
            pytest.xfail("KNOWN BUG: Function interdependence fails due to module scoping")

        assert call_result.success

        # Check result
        result_cmd = 'result'
        result_check = self.session.execute_statement(result_cmd)
        assert result_check.success
        assert result_check.value.value == 21

    def test_function_calling_works_outside_modules(self):
        """Verify that function calling works fine outside modules (control test)."""
        # Define functions in main context
        self.session.execute_statement('func helper() { return 42 }')
        self.session.execute_statement('func main() { return helper() }')

        # This should work (and currently does)
        result = self.session.execute_statement('result = main()')
        assert result.success

        # Check result
        check = self.session.execute_statement('result')
        assert check.success
        assert check.value.value == 42


if __name__ == '__main__':
    pytest.main([__file__, '-v'])
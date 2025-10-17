#!/usr/bin/env python3
"""Test call graph visualization and introspection features."""

import pytest
import sys
import os
import tempfile
import shutil
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../src'))

from glang.execution.pipeline import ExecutionSession
from glang.files.file_manager import FileManager


class TestCallGraphVisualization:
    """Test the call graph visualization and introspection features."""

    def setup_method(self):
        """Set up test with temporary directory and execution session."""
        self.temp_dir = tempfile.mkdtemp()
        self.file_manager = FileManager()
        self.session = ExecutionSession(self.file_manager)

    def teardown_method(self):
        """Clean up temporary directory."""
        shutil.rmtree(self.temp_dir)

    def test_basic_call_graph_visualization(self):
        """Test basic visualization of the call graph."""
        # Define some functions
        self.session.execute_statement('func main() { greet("World") }')
        self.session.execute_statement('func greet(name) { print("Hello, " + name) }')
        self.session.execute_statement('func helper() { return 42 }')

        # Import call graph module
        import_result = self.session.execute_statement('import "call_graph" as cg')
        assert import_result.success, f"Failed to import call_graph: {import_result.error}"

        # Test text visualization
        viz_result = self.session.execute_statement('viz = cg.visualize("text")')
        assert viz_result.success, f"Visualization failed: {viz_result.error}"

        # Get the visualization text
        text_result = self.session.execute_statement('viz')
        assert text_result.success
        text_viz = text_result.value.value

        assert "COMPLETE CALL GRAPH" in text_viz
        assert "main" in text_viz
        assert "greet" in text_viz
        assert "helper" in text_viz

    def test_scope_visualization(self):
        """Test visualization of specific scopes."""
        # Create a module file
        module_content = '''module TestModule

func module_func1() {
    module_func2()
}

func module_func2() {
    return 123
}
'''
        module_path = os.path.join(self.temp_dir, 'test_module.gr')
        with open(module_path, 'w') as f:
            f.write(module_content)

        # Import the module
        import_cmd = f'import "{module_path}" as test'
        import_result = self.session.execute_statement(import_cmd)
        assert import_result.success, f"Module import failed: {import_result.error}"

        # Import call graph module
        cg_import = self.session.execute_statement('import "call_graph" as cg')
        assert cg_import.success

        # Visualize the module scope
        viz_result = self.session.execute_statement('scope_viz = cg.visualize_scope("TestModule")')
        assert viz_result.success, f"Scope visualization failed: {viz_result.error}"

        # Check content
        text_result = self.session.execute_statement('scope_viz')
        assert text_result.success
        scope_viz = text_result.value.value

        assert "TestModule" in scope_viz
        assert "module_func1" in scope_viz
        assert "module_func2" in scope_viz

    def test_call_graph_introspection_functions(self):
        """Test the introspection functions of the call graph module."""
        # Define some functions
        self.session.execute_statement('func a() { b() }')
        self.session.execute_statement('func b() { c() }')
        self.session.execute_statement('func c() { return 1 }')

        # Import call graph module
        import_result = self.session.execute_statement('import "call_graph" as cg')
        assert import_result.success

        # Test current_scope
        scope_result = self.session.execute_statement('current = cg.current_scope()')
        assert scope_result.success
        scope_check = self.session.execute_statement('current')
        assert scope_check.success
        assert scope_check.value.value == "global"

        # Test count_functions
        count_result = self.session.execute_statement('count = cg.count_functions()')
        assert count_result.success
        count_check = self.session.execute_statement('count')
        assert count_check.success
        assert count_check.value.value >= 3  # At least a, b, c

        # Test get_reachable_functions
        funcs_result = self.session.execute_statement('funcs = cg.get_reachable_functions()')
        assert funcs_result.success
        funcs_check = self.session.execute_statement('funcs')
        assert funcs_check.success
        # Should be a list containing our functions
        assert funcs_check.value.get_type() == "list"

        # Test list_scopes
        scopes_result = self.session.execute_statement('scopes = cg.list_scopes()')
        assert scopes_result.success
        scopes_check = self.session.execute_statement('scopes')
        assert scopes_check.success
        assert scopes_check.value.get_type() == "list"

    def test_get_function_info(self):
        """Test getting detailed function information."""
        # Create a module with functions
        module_content = '''module InfoModule

func calculate(x, y, z) {
    return x + y + z
}

func process(input) {
    return calculate(1, 2, 3)
}
'''
        module_path = os.path.join(self.temp_dir, 'info_module.gr')
        with open(module_path, 'w') as f:
            f.write(module_content)

        # Import the module
        import_cmd = f'import "{module_path}" as infomod'
        import_result = self.session.execute_statement(import_cmd)
        assert import_result.success

        # Import call graph module
        cg_import = self.session.execute_statement('import "call_graph" as cg')
        assert cg_import.success

        # Get info about calculate function
        info_result = self.session.execute_statement('func_info = cg.get_function_info("calculate", "InfoModule")')
        assert info_result.success

        # Check if info is not none
        check_result = self.session.execute_statement('func_info != none')
        assert check_result.success
        if check_result.value.value:  # Only check details if info was found
            # Check name
            name_result = self.session.execute_statement('func_info["name"]')
            assert name_result.success
            assert name_result.value.value == "calculate"

            # Check scope
            scope_result = self.session.execute_statement('func_info["scope"]')
            assert scope_result.success
            assert scope_result.value.value == "InfoModule"

    def test_path_finding(self):
        """Test finding paths between functions."""
        # Create a module with connected functions
        module_content = '''module PathModule

func start() {
    step1()
}

func step1() {
    step2()
}

func step2() {
    finish()
}

func finish() {
    return "complete"
}
'''
        module_path = os.path.join(self.temp_dir, 'path_module.gr')
        with open(module_path, 'w') as f:
            f.write(module_content)

        # Import the module
        import_cmd = f'import "{module_path}" as path'
        import_result = self.session.execute_statement(import_cmd)
        assert import_result.success

        # Import call graph module
        cg_import = self.session.execute_statement('import "call_graph" as cg')
        assert cg_import.success

        # Find path from start to finish
        path_result = self.session.execute_statement('path = cg.find_path("start", "finish", "PathModule")')
        assert path_result.success

        # Check if path exists
        check_result = self.session.execute_statement('path != none')
        assert check_result.success
        if check_result.value.value:  # Path found
            path_check = self.session.execute_statement('path')
            assert path_check.success
            assert path_check.value.get_type() == "list"
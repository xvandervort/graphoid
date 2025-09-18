#!/usr/bin/env python3
"""Test Phase 3 call graph features: AST as temporary subgraph."""

import pytest
import sys
import os
import tempfile
import shutil
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../src'))

from glang.execution.pipeline import ExecutionSession
from glang.files.file_manager import FileManager
from glang.parser.ast_parser import ASTParser
from glang.execution.call_graph import CallGraph, CallGraphSubgraph


class TestCallGraphPhase3:
    """Test Phase 3 call graph implementation: AST as temporary subgraph."""

    def setup_method(self):
        """Set up test with temporary directory and execution session."""
        self.temp_dir = tempfile.mkdtemp()
        self.file_manager = FileManager()
        self.session = ExecutionSession(self.file_manager)
        self.parser = ASTParser()
        self.call_graph = CallGraph()

    def teardown_method(self):
        """Clean up temporary directory."""
        shutil.rmtree(self.temp_dir)

    def test_ast_subgraph_creation(self):
        """Test creating AST subgraph from function declarations."""
        # Parse AST with function declarations
        code = '''
        func test_func1() {
            return 1
        }

        func test_func2() {
            return test_func1() + 1
        }

        func test_func3() {
            return test_func2() + 1
        }
        '''

        ast = self.parser.parse(code)

        # Create subgraph from AST
        subgraph = self.call_graph.create_ast_subgraph(ast, "global")

        # Verify subgraph contains all functions
        assert len(subgraph) == 3
        function_names = subgraph.get_function_names()
        assert "test_func1" in function_names
        assert "test_func2" in function_names
        assert "test_func3" in function_names

        # Verify subgraph scope
        assert subgraph.scope == "global"

    def test_ast_subgraph_from_module(self):
        """Test creating AST subgraph from module with declared name."""
        code = '''
        module TestModule

        func module_func1() {
            return "module1"
        }

        func module_func2() {
            return module_func1() + "_module2"
        }
        '''

        ast = self.parser.parse(code)

        # Create subgraph with module scope
        subgraph = self.call_graph.create_ast_subgraph(ast, "TestModule")

        # Verify module subgraph
        assert len(subgraph) == 2
        assert subgraph.scope == "TestModule"
        function_names = subgraph.get_function_names()
        assert "module_func1" in function_names
        assert "module_func2" in function_names

    def test_subgraph_merge_into_call_graph(self):
        """Test merging subgraph into permanent call graph."""
        code = '''
        func merge_test1() {
            return "test1"
        }

        func merge_test2() {
            return merge_test1() + "_test2"
        }
        '''

        ast = self.parser.parse(code)
        subgraph = self.call_graph.create_ast_subgraph(ast, "global")

        # Merge subgraph into call graph
        self.call_graph.merge_subgraph(subgraph)

        # Verify functions are now in call graph
        func1 = self.call_graph.find_function("merge_test1", "global")
        func2 = self.call_graph.find_function("merge_test2", "global")

        assert func1 is not None
        assert func2 is not None
        assert func1.name == "merge_test1"
        assert func2.name == "merge_test2"

    def test_subgraph_merge_with_module_scope(self):
        """Test merging module subgraph maintains proper scope."""
        code = '''
        func scoped_func1() {
            return "scoped1"
        }

        func scoped_func2() {
            return scoped_func1() + "_scoped2"
        }
        '''

        ast = self.parser.parse(code)
        subgraph = self.call_graph.create_ast_subgraph(ast, "ScopeTestModule")

        # Merge with module scope
        self.call_graph.merge_subgraph(subgraph)

        # Verify functions are in correct scope
        func1 = self.call_graph.find_function("scoped_func1", "ScopeTestModule")
        func2 = self.call_graph.find_function("scoped_func2", "ScopeTestModule")

        assert func1 is not None
        assert func2 is not None

        # Verify they're NOT in global scope
        func1_global = self.call_graph.find_function("scoped_func1", "global")
        func2_global = self.call_graph.find_function("scoped_func2", "global")

        assert func1_global is None
        assert func2_global is None

    def test_phase3_execution_pipeline_integration(self):
        """Test Phase 3 integration in execution pipeline."""
        # Test that execution pipeline uses AST subgraph approach
        result = self.session.execute_statement('''
        func pipeline_test1() {
            return "pipeline1"
        }

        func pipeline_test2() {
            return pipeline_test1() + "_pipeline2"
        }

        pipeline_test2()
        ''')

        assert result.success
        assert result.value.value == "pipeline1_pipeline2"

        # Verify functions are in call graph via subgraph merge
        func1 = self.session.execution_context.call_graph.find_function("pipeline_test1", "global")
        func2 = self.session.execution_context.call_graph.find_function("pipeline_test2", "global")

        assert func1 is not None
        assert func2 is not None

    def test_pure_graph_traversal_no_variable_fallback(self):
        """Test that function calls use pure graph traversal, not variables."""
        # Execute functions that should be in call graph
        result1 = self.session.execute_statement('''
        func pure_test1() {
            return "pure1"
        }
        ''')
        assert result1.success

        result2 = self.session.execute_statement('''
        func pure_test2() {
            return pure_test1() + "_pure2"
        }
        ''')
        assert result2.success

        # Call the function to verify graph traversal works
        result3 = self.session.execute_statement('pure_test2()')
        assert result3.success
        assert result3.value.value == "pure1_pure2"

        # Verify function was found via call graph, not variables
        func_in_graph = self.session.execution_context.call_graph.find_function("pure_test1", "global")
        assert func_in_graph is not None

    def test_module_loading_with_ast_subgraph(self):
        """Test module loading uses AST subgraph approach."""
        # Create a module file
        module_content = '''module Phase3TestModule

func phase3_mod1() {
    return "phase3_1"
}

func phase3_mod2() {
    return phase3_mod1() + "_phase3_2"
}

func phase3_mod3() {
    return phase3_mod2() + "_phase3_3"
}
'''
        module_path = os.path.join(self.temp_dir, 'phase3_test.gr')
        with open(module_path, 'w') as f:
            f.write(module_content)

        # Import the module
        import_result = self.session.execute_statement(f'import "{module_path}" as p3test')
        assert import_result.success

        # Test module function chain (requires call graph connectivity)
        call_result = self.session.execute_statement('p3test.phase3_mod3()')
        assert call_result.success
        assert call_result.value.value == "phase3_1_phase3_2_phase3_3"

        # Verify functions are in call graph with correct scope
        call_graph = self.session.execution_context.call_graph
        func1 = call_graph.find_function("phase3_mod1", "Phase3TestModule")
        func2 = call_graph.find_function("phase3_mod2", "Phase3TestModule")
        func3 = call_graph.find_function("phase3_mod3", "Phase3TestModule")

        assert func1 is not None
        assert func2 is not None
        assert func3 is not None

    def test_ast_subgraph_empty_code(self):
        """Test AST subgraph creation with no functions."""
        code = '''
        string test_var = "no functions here"
        list items = [1, 2, 3]
        '''

        ast = self.parser.parse(code)
        subgraph = self.call_graph.create_ast_subgraph(ast, "global")

        # Should create empty subgraph
        assert len(subgraph) == 0
        assert subgraph.get_function_names() == []

    def test_ast_subgraph_nested_functions(self):
        """Test AST subgraph extraction with nested function declarations."""
        code = '''
        func outer_func() {
            func inner_func() {
                return "inner"
            }
            return inner_func() + "_outer"
        }
        '''

        ast = self.parser.parse(code)
        subgraph = self.call_graph.create_ast_subgraph(ast, "global")

        # Should find both outer and inner functions
        assert len(subgraph) == 2
        function_names = subgraph.get_function_names()
        assert "outer_func" in function_names
        assert "inner_func" in function_names

    def test_subgraph_function_connectivity(self):
        """Test that merged subgraph functions are properly connected."""
        code = '''
        func conn_test1() {
            return "conn1"
        }

        func conn_test2() {
            return "conn2"
        }

        func conn_test3() {
            return "conn3"
        }
        '''

        ast = self.parser.parse(code)
        subgraph = self.call_graph.create_ast_subgraph(ast, "ConnTestModule")
        self.call_graph.merge_subgraph(subgraph)

        # Verify all functions can reach each other (bidirectional connectivity)
        path1to2 = self.call_graph.find_path("conn_test1", "conn_test2", "ConnTestModule")
        path2to3 = self.call_graph.find_path("conn_test2", "conn_test3", "ConnTestModule")
        path3to1 = self.call_graph.find_path("conn_test3", "conn_test1", "ConnTestModule")

        assert path1to2 is not None
        assert path2to3 is not None
        assert path3to1 is not None

        # Should be direct connections (length 2: start + end)
        assert len(path1to2) == 2
        assert len(path2to3) == 2
        assert len(path3to1) == 2
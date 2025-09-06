"""Tests for program-level imports (not just REPL)."""

import pytest
import os
import tempfile
from src.glang.execution.pipeline import ExecutionSession
from src.glang.files.file_manager import FileManager

class TestProgramImports:
    """Test that imports work correctly in program files, not just REPL."""
    
    def setup_method(self):
        """Setup for each test."""
        self.file_manager = FileManager()
        self.execution_session = ExecutionSession(self.file_manager)
    
    def test_basic_program_import(self):
        """Test basic import functionality in program files."""
        # Create a math library file
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('module advanced_mathematics\n')
            f.write('alias math\n')
            f.write('num pi = 3.14159\n')
            f.write('num two = 2\n')
            math_file = f.name
        
        # Create a main program that imports the math library
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('/import "{}"\n'.format(os.path.basename(math_file)))
            f.write('num result = math.pi\n')
            f.write('num doubled = math.two\n')
            main_file = f.name
        
        try:
            # Execute the main program (this should load the import)
            result = self.execution_session.execute_statement(f'load "{main_file}"')
            
            # Check that the execution was successful
            assert result.success, f"Execution failed: {result.error}"
            
            # Verify that the math module was loaded
            math_module = self.execution_session.module_manager.get_module("math")
            assert math_module is not None, "Math module was not loaded"
            assert math_module.name == "math"
            
            # Verify that variables from main file were created
            result_var = self.execution_session.execution_context.get_variable("result")
            assert result_var is not None, "result variable not found"
            assert result_var.value == 3.14159, f"Expected pi, got {result_var.value}"
            
            doubled_var = self.execution_session.execution_context.get_variable("doubled")
            assert doubled_var is not None, "doubled variable not found"
            assert doubled_var.value == 2, f"Expected 2, got {doubled_var.value}"
            
        finally:
            os.unlink(math_file)
            os.unlink(main_file)
    
    def test_module_qualified_access_in_programs(self):
        """Test that module-qualified variable access works in program files."""
        # Create a utilities library
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('module string_utilities\n')
            f.write('alias utils\n')
            f.write('string greeting = "Hello"\n')
            f.write('num count = 42\n')
            utils_file = f.name
        
        # Create a main program that uses module-qualified access
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('/import "{}"\n'.format(os.path.basename(utils_file)))
            # Test module-qualified access
            f.write('string msg = utils.greeting\n') 
            f.write('num number = utils.count\n')
            main_file = f.name
        
        try:
            # Execute the main program
            result = self.execution_session.execute_statement(f'load "{main_file}"')
            
            # Check execution success
            assert result.success, f"Execution failed: {result.error}"
            
            # Verify module-qualified variables were accessed correctly
            msg_var = self.execution_session.execution_context.get_variable("msg")
            assert msg_var is not None, "msg variable not found"
            assert msg_var.value == "Hello", f"Expected 'Hello', got {msg_var.value}"
            
            number_var = self.execution_session.execution_context.get_variable("number")
            assert number_var is not None, "number variable not found"
            assert number_var.value == 42, f"Expected 42, got {number_var.value}"
            
        finally:
            os.unlink(utils_file)
            os.unlink(main_file)
    
    def test_multiple_imports_in_program(self):
        """Test multiple imports in a single program file."""
        # Create first library
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('module mathematics\n')
            f.write('alias math\n')
            f.write('num pi = 3.14159\n')
            math_file = f.name
        
        # Create second library  
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('module text_processing\n')
            f.write('alias text\n')
            f.write('string hello = "Hello World"\n')
            text_file = f.name
        
        # Create main program with multiple imports
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('/import "{}"\n'.format(os.path.basename(math_file)))
            f.write('/import "{}"\n'.format(os.path.basename(text_file)))
            f.write('num my_pi = math.pi\n')
            f.write('string my_text = text.hello\n')
            main_file = f.name
        
        try:
            # Execute the main program
            result = self.execution_session.execute_statement(f'load "{main_file}"')
            
            assert result.success, f"Execution failed: {result.error}"
            
            # Verify both modules are loaded
            math_module = self.execution_session.module_manager.get_module("math")
            text_module = self.execution_session.module_manager.get_module("text")
            assert math_module is not None, "Math module not loaded"
            assert text_module is not None, "Text module not loaded"
            
            # Verify variables from both modules
            pi_var = self.execution_session.execution_context.get_variable("my_pi")
            text_var = self.execution_session.execution_context.get_variable("my_text")
            assert pi_var.value == 3.14159
            assert text_var.value == "Hello World"
            
        finally:
            os.unlink(math_file)
            os.unlink(text_file)
            os.unlink(main_file)
    
    def test_import_with_explicit_alias_in_program(self):
        """Test import with explicit alias override in program files."""
        # Create a library
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('module complex_mathematical_operations\n')
            f.write('alias math\n')  # Default alias is 'math'
            f.write('num value = 100\n')
            lib_file = f.name
        
        # Create main program with explicit alias override
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('/import "{}" as custom\n'.format(os.path.basename(lib_file)))
            f.write('num my_value = custom.value\n')
            main_file = f.name
        
        try:
            # Execute the main program
            result = self.execution_session.execute_statement(f'load "{main_file}"')
            
            assert result.success, f"Execution failed: {result.error}"
            
            # Verify module is accessible by explicit alias
            custom_module = self.execution_session.module_manager.get_module("custom")
            assert custom_module is not None, "Custom aliased module not found"
            
            # Verify variable access through explicit alias
            value_var = self.execution_session.execution_context.get_variable("my_value")
            assert value_var is not None, "my_value variable not found"
            assert value_var.value == 100, f"Expected 100, got {value_var.value}"
            
        finally:
            os.unlink(lib_file)
            os.unlink(main_file)
    
    def test_nested_program_imports(self):
        """Test that a program can import another program that also has imports."""
        # Create base library
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('module basic_math\n')
            f.write('alias math\n')
            f.write('num pi = 3.14159\n')
            base_file = f.name
        
        # Create intermediate library that imports the base
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('/import "{}"\n'.format(os.path.basename(base_file)))
            f.write('module advanced_operations\n') 
            f.write('alias ops\n')
            f.write('num circle_area = math.pi\n')  # Uses imported math
            intermediate_file = f.name
        
        # Create main program that imports the intermediate
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gr', delete=False) as f:
            f.write('/import "{}"\n'.format(os.path.basename(intermediate_file)))
            f.write('num area = ops.circle_area\n')
            main_file = f.name
        
        try:
            # Execute the main program
            result = self.execution_session.execute_statement(f'load "{main_file}"')
            
            assert result.success, f"Execution failed: {result.error}"
            
            # Verify all modules are loaded
            math_module = self.execution_session.module_manager.get_module("math")
            ops_module = self.execution_session.module_manager.get_module("ops") 
            assert math_module is not None, "Base math module not loaded"
            assert ops_module is not None, "Intermediate ops module not loaded"
            
            # Verify the nested computation worked
            area_var = self.execution_session.execution_context.get_variable("area")
            assert area_var is not None, "area variable not found"
            assert area_var.value == 3.14159, f"Expected pi, got {area_var.value}"
            
        finally:
            os.unlink(base_file)
            os.unlink(intermediate_file)
            os.unlink(main_file)
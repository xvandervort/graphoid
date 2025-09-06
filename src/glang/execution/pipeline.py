"""
Glang Execution Pipeline

Complete pipeline from source code to execution:
source → tokens → AST → semantic analysis → execution

This integrates all phases of the new AST-based system.
"""

from typing import Any, Optional
from dataclasses import dataclass
import sys
import os

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../../..'))

from glang.semantic.pipeline import SemanticPipeline, SemanticSession, AnalysisResult
from glang.semantic.symbol_table import SymbolTable
from .executor import ASTExecutor, ExecutionContext
from .values import GlangValue
from .errors import RuntimeError as GlangRuntimeError, LoadRequest, ImportRequest
from glang.files import FileManager


@dataclass
class ExecutionResult:
    """Result of code execution."""
    value: Any
    context: ExecutionContext
    success: bool
    error: Optional[Exception] = None
    source_code: Optional[str] = None
    source_name: Optional[str] = None
    
    def __str__(self) -> str:
        if not self.success:
            return f"Execution failed: {self.error}"
        return str(self.value) if self.value is not None else "No result"
    
    def get_formatted_error(self) -> Optional[str]:
        """Get formatted error message with context if available."""
        if self.success or not self.error:
            return None
        
        if self.source_code and self.source_name:
            from ..errors import ErrorFormatter
            return ErrorFormatter.format_error_with_context(
                self.error, self.source_code, self.source_name
            )
        else:
            from ..errors import ErrorFormatter
            return ErrorFormatter.format_error_simple(self.error)


class ExecutionPipeline:
    """Complete pipeline: source → tokens → AST → semantic analysis → execution"""
    
    def __init__(self):
        self.semantic_pipeline = SemanticPipeline()
    
    def execute_code(self, input_str: str) -> ExecutionResult:
        """Execute source code and return result."""
        
        # Phase 1 & 2: Parse and analyze
        try:
            analysis_result = self.semantic_pipeline.analyze_code(input_str)
        except Exception as e:
            return ExecutionResult(None, None, False, e)
        
        if not analysis_result.success:
            # Return semantic errors
            error_messages = []
            for error in analysis_result.errors:
                if hasattr(error, 'message'):
                    error_messages.append(str(error.message))
                else:
                    error_messages.append(str(error))
                    
            return ExecutionResult(
                None, 
                None, 
                False, 
                Exception(f"Semantic analysis failed: {', '.join(error_messages)}")
            )
        
        # Phase 3: Execute
        context = ExecutionContext(analysis_result.symbol_table)
        executor = ASTExecutor(context)
        
        try:
            result = executor.execute(analysis_result.ast)
            return ExecutionResult(result, context, True)
        except GlangRuntimeError as e:
            return ExecutionResult(None, context, False, e)
        except Exception as e:
            # Wrap unexpected errors
            wrapped_error = GlangRuntimeError(f"Unexpected execution error: {str(e)}")
            return ExecutionResult(None, context, False, wrapped_error)


class ExecutionSession:
    """Session with persistent execution context for REPL-like behavior."""
    
    def __init__(self, file_manager: Optional[FileManager] = None):
        self.semantic_session = SemanticSession()
        # File manager for load operations
        self.file_manager = file_manager or FileManager()
        # Module manager for import operations
        from ..modules import ModuleManager
        self.module_manager = ModuleManager(self.file_manager)
        # Create execution context that shares the symbol table
        self.execution_context = ExecutionContext(self.semantic_session.get_symbol_table(), self.module_manager)
    
    def execute_statement(self, input_str: str) -> ExecutionResult:
        """Execute statement in persistent context."""
        
        # First, do semantic analysis using the session
        try:
            analysis_result = self.semantic_session.analyze_statement(input_str)
        except Exception as e:
            return ExecutionResult(
                None, 
                self.execution_context, 
                False, 
                e, 
                input_str, 
                "<input>"
            )
        
        if not analysis_result.success:
            # Handle multiple semantic errors
            if len(analysis_result.errors) == 1:
                error = analysis_result.errors[0]
            else:
                from ..errors import ErrorFormatter
                formatted_errors = ErrorFormatter.format_multiple_errors(
                    analysis_result.errors, input_str, "<input>"
                )
                # Create a wrapper exception with the formatted message
                error = Exception(formatted_errors)
            
            return ExecutionResult(
                None,
                self.execution_context,
                False,
                error,
                input_str,
                "<input>"
            )
        
        # Now execute the AST
        executor = ASTExecutor(self.execution_context, self.file_manager)
        
        try:
            result = executor.execute(analysis_result.ast)
            return ExecutionResult(result, self.execution_context, True)
        except LoadRequest as load_req:
            # Handle load request by loading the file and continuing
            try:
                load_result = self.file_manager.load_file(load_req.filename, self)
                if not load_result.success:
                    return ExecutionResult(
                        None, 
                        self.execution_context, 
                        False, 
                        GlangRuntimeError(f"Failed to load {load_req.filename}: {load_result.error}", load_req.position),
                        input_str,
                        "<input>"
                    )
                return ExecutionResult(f"Loaded {load_req.filename}", self.execution_context, True)
            except Exception as e:
                return ExecutionResult(
                    None, 
                    self.execution_context, 
                    False,
                    GlangRuntimeError(f"Error loading {load_req.filename}: {str(e)}", load_req.position),
                    input_str,
                    "<input>"
                )
        except ImportRequest as import_req:
            # Handle import request by importing the module
            try:
                module = self.module_manager.import_module(
                    import_req.filename, 
                    import_req.alias, 
                    import_req.position
                )
                
                # Load the module file and execute it in the module's namespace
                load_result = self.file_manager.load_file(import_req.filename, self)
                if not load_result.success:
                    return ExecutionResult(
                        None, 
                        self.execution_context, 
                        False, 
                        GlangRuntimeError(f"Failed to import {import_req.filename}: {load_result.error}", import_req.position),
                        input_str,
                        "<input>"
                    )
                
                # Move variables from main context to module namespace
                # This is a simplified approach - in reality we'd execute in a separate context
                for var_name in list(self.execution_context.variables.keys()):
                    if var_name not in ['_temp_vars_before_import']:  # Skip internal variables
                        value = self.execution_context.variables[var_name]
                        module.namespace.set_symbol(var_name, value)
                        # Don't remove from main context for now to keep load compatibility
                
                module_name = module.name
                return ExecutionResult(f"Imported {import_req.filename} as {module_name}", self.execution_context, True)
                
            except Exception as e:
                return ExecutionResult(
                    None, 
                    self.execution_context, 
                    False,
                    GlangRuntimeError(f"Error importing {import_req.filename}: {str(e)}", import_req.position),
                    input_str,
                    "<input>"
                )
        except GlangRuntimeError as e:
            return ExecutionResult(None, self.execution_context, False, e, input_str, "<input>")
        except Exception as e:
            wrapped_error = GlangRuntimeError(f"Unexpected execution error: {str(e)}")
            return ExecutionResult(None, self.execution_context, False, wrapped_error, input_str, "<input>")
    
    def get_variable_value(self, name: str) -> Optional[GlangValue]:
        """Get current value of a variable."""
        return self.execution_context.get_variable(name)
    
    def list_variables(self) -> dict:
        """Get list of variables with their types and values."""
        variables = {}
        
        # Get variables from execution context
        for name, value in self.execution_context.variables.items():
            variables[name] = {
                'name': name,
                'type': value.get_type(),
                'value': value,
                'display': value.to_display_string()
            }
        
        return variables
    
    def clear_variables(self) -> None:
        """Clear all variables from the session."""
        self.semantic_session.persistent_symbol_table = SymbolTable()
        self.execution_context = ExecutionContext(self.semantic_session.get_symbol_table())
    
    def get_session_info(self) -> dict:
        """Get information about the current session."""
        variables = self.list_variables()
        
        return {
            'variable_count': len(variables),
            'variables': list(variables.keys()),
            'symbol_table_size': len(self.semantic_session.get_symbol_table().symbols)
        }


# Convenience functions for quick testing
def execute_code(code: str) -> ExecutionResult:
    """Quick execution of a single piece of code."""
    pipeline = ExecutionPipeline()
    return pipeline.execute_code(code)


def create_session() -> ExecutionSession:
    """Create a new execution session."""
    return ExecutionSession()


# Example usage
if __name__ == "__main__":
    # Test the execution pipeline
    session = create_session()
    
    # Test variable declaration
    result1 = session.execute_statement('list<num> numbers = [1, 2, 3]')
    print(f"Declaration: {result1}")
    
    # Test method call
    result2 = session.execute_statement('numbers.append(4)')
    print(f"Method call: {result2}")
    
    # Test variable reference
    result3 = session.execute_statement('numbers[0]')
    print(f"Index access: {result3}")
    
    # Show session info
    print(f"Session info: {session.get_session_info()}")
    print(f"Variables: {session.list_variables()}")
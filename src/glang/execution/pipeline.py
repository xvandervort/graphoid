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
from .errors import RuntimeError as GlangRuntimeError


@dataclass
class ExecutionResult:
    """Result of code execution."""
    value: Any
    context: ExecutionContext
    success: bool
    error: Optional[Exception] = None
    
    def __str__(self) -> str:
        if not self.success:
            return f"Execution failed: {self.error}"
        return str(self.value) if self.value is not None else "No result"


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
    
    def __init__(self):
        self.semantic_session = SemanticSession()
        # Create execution context that shares the symbol table
        self.execution_context = ExecutionContext(self.semantic_session.get_symbol_table())
    
    def execute_statement(self, input_str: str) -> ExecutionResult:
        """Execute statement in persistent context."""
        
        # First, do semantic analysis using the session
        try:
            analysis_result = self.semantic_session.analyze_statement(input_str)
        except Exception as e:
            return ExecutionResult(None, self.execution_context, False, e)
        
        if not analysis_result.success:
            return ExecutionResult(
                None,
                self.execution_context,
                False,
                Exception(f"Semantic analysis failed: {', '.join(str(e) for e in analysis_result.errors)}")
            )
        
        # Now execute the AST
        executor = ASTExecutor(self.execution_context)
        
        try:
            result = executor.execute(analysis_result.ast)
            return ExecutionResult(result, self.execution_context, True)
        except GlangRuntimeError as e:
            return ExecutionResult(None, self.execution_context, False, e)
        except Exception as e:
            wrapped_error = GlangRuntimeError(f"Unexpected execution error: {str(e)}")
            return ExecutionResult(None, self.execution_context, False, wrapped_error)
    
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
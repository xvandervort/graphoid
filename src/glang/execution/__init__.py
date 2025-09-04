"""
Glang Execution Engine Package

This package implements AST-based execution for the glang language,
replacing the previous string-based execution system with proper
AST interpretation.

Components:
- values: Runtime value representation system
- executor: AST executor using visitor pattern
- errors: Runtime error handling with position information
- dispatcher: Method dispatch integration with existing graph system
- pipeline: Complete execution pipeline
"""

from .values import GlangValue, StringValue, NumberValue, BooleanValue, ListValue
from .executor import ASTExecutor, ExecutionContext
from .errors import RuntimeError, VariableNotFoundError, TypeConstraintError
from .pipeline import ExecutionPipeline, ExecutionSession, ExecutionResult

__all__ = [
    'GlangValue', 'StringValue', 'NumberValue', 'BooleanValue', 'ListValue',
    'ASTExecutor', 'ExecutionContext',
    'RuntimeError', 'VariableNotFoundError', 'TypeConstraintError',
    'ExecutionPipeline', 'ExecutionSession', 'ExecutionResult'
]
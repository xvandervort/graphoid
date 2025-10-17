"""Semantic analysis package for glang.

This package provides semantic analysis capabilities including:
- Symbol table management
- Type checking and constraint validation
- Variable resolution
- Semantic error reporting
"""

from .symbol_table import Symbol, SymbolTable
from .errors import (
    SemanticError, UndefinedVariableError, TypeMismatchError, 
    ConstraintViolationError, InvalidMethodCallError, RedeclarationError,
    InvalidTypeError, InvalidConstraintError
)
from .analyzer import SemanticAnalyzer, AnalysisResult
from .pipeline import SemanticPipeline, SemanticSession

__all__ = [
    'Symbol',
    'SymbolTable', 
    'SemanticError',
    'UndefinedVariableError',
    'TypeMismatchError', 
    'ConstraintViolationError',
    'InvalidMethodCallError',
    'RedeclarationError',
    'InvalidTypeError',
    'InvalidConstraintError',
    'SemanticAnalyzer',
    'AnalysisResult',
    'SemanticPipeline',
    'SemanticSession'
]
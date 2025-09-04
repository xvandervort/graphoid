"""Parser components for glang - both old and new."""

# Import existing parser components
from .syntax_parser import SyntaxParser
from .expression_evaluator import ExpressionEvaluator
from .ast_nodes import (
    InputType,
    ParsedCommand,
    VariableDeclaration as LegacyVariableDeclaration,
    MethodCall as LegacyMethodCall,
    VariableAccess,
    IndexAccess as LegacyIndexAccess,
    IndexAssignment as LegacyIndexAssignment,
    SliceAccess as LegacySliceAccess,
    SliceAssignment as LegacySliceAssignment,
    LegacyCommand as LegacyLegacyCommand
)

# Import new AST parser
from .ast_parser import ASTParser, ParseError

__all__ = [
    # Existing components (for backward compatibility)
    'SyntaxParser',
    'ExpressionEvaluator',
    'InputType',
    'ParsedCommand',
    'LegacyVariableDeclaration',
    'LegacyMethodCall', 
    'VariableAccess',
    'LegacyIndexAccess',
    'LegacyIndexAssignment',
    'LegacySliceAccess', 
    'LegacySliceAssignment',
    'LegacyLegacyCommand',
    
    # New AST parser
    'ASTParser',
    'ParseError'
]
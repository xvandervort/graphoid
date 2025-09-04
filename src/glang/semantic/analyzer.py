"""Semantic analyzer for glang AST nodes."""

from typing import List, Optional, Any
from dataclasses import dataclass

from ..ast.nodes import *
from .symbol_table import Symbol, SymbolTable
from .errors import *


@dataclass 
class AnalysisResult:
    """Result of semantic analysis."""
    ast: Optional[ASTNode]
    symbol_table: SymbolTable
    errors: List[SemanticError]
    success: bool
    
    def has_errors(self) -> bool:
        """Check if analysis found any errors."""
        return len(self.errors) > 0
    
    def add_error(self, error: SemanticError) -> None:
        """Add an error to the result."""
        self.errors.append(error)
        self.success = False


class SemanticAnalyzer(ASTVisitor):
    """Semantic analyzer using visitor pattern."""
    
    def __init__(self):
        self.symbol_table = SymbolTable()
        self.errors: List[SemanticError] = []
    
    def analyze(self, ast: ASTNode, clear_state: bool = True) -> AnalysisResult:
        """Perform semantic analysis on an AST node.
        
        Args:
            ast: The AST node to analyze
            clear_state: Whether to clear existing state before analysis
            
        Returns:
            AnalysisResult with symbol table and any errors found
        """
        if clear_state:
            self.errors.clear()
            self.symbol_table.clear()
        else:
            self.errors.clear()  # Always clear errors
        
        try:
            ast.accept(self)
            success = len(self.errors) == 0
            return AnalysisResult(ast, self.symbol_table, self.errors.copy(), success)
        except Exception as e:
            # Catch any unexpected errors during analysis
            error = SemanticError(f"Internal analysis error: {str(e)}")
            self.errors.append(error)
            return AnalysisResult(ast, self.symbol_table, self.errors.copy(), False)
    
    def report_error(self, error: SemanticError) -> None:
        """Report a semantic error."""
        self.errors.append(error)
    
    # Statement visitors
    
    def visit_variable_declaration(self, node: VariableDeclaration) -> None:
        """Analyze variable declarations."""
        # Validate type
        valid_types = {'list', 'string', 'num', 'bool'}
        if node.var_type not in valid_types:
            self.report_error(InvalidTypeError(node.var_type, node.position))
            return
        
        # Validate constraint if present
        if node.type_constraint:
            valid_constraints = {'string', 'num', 'bool', 'list'}
            if node.type_constraint not in valid_constraints:
                self.report_error(InvalidConstraintError(
                    node.type_constraint, node.var_type, node.position))
                return
            
            # Only list type supports constraints currently
            if node.var_type != 'list':
                self.report_error(InvalidConstraintError(
                    node.type_constraint, node.var_type, node.position))
                return
        
        # Check if variable already exists
        if self.symbol_table.symbol_exists(node.name):
            existing = self.symbol_table.lookup_symbol(node.name)
            self.report_error(RedeclarationError(
                node.name, existing.position, node.position))
            return
        
        # Create and declare symbol
        symbol = Symbol(
            name=node.name,
            symbol_type=node.var_type,
            type_constraint=node.type_constraint,
            position=node.position
        )
        
        try:
            self.symbol_table.declare_symbol(symbol)
        except ValueError as e:
            self.report_error(SemanticError(str(e), node.position))
        
        # Analyze initializer
        if node.initializer:
            node.initializer.accept(self)
            # TODO: Type check initializer against declaration
    
    def visit_method_call(self, node: MethodCall) -> None:
        """Analyze method call statements."""
        # Analyze target
        node.target.accept(self)
        
        # For variable references, validate method call if variable exists
        if isinstance(node.target, VariableRef):
            symbol = self.symbol_table.lookup_symbol(node.target.name)
            if symbol:  # Only validate if symbol exists (undefined already reported by target visitor)
                # Check if method is valid for the target type
                self._validate_method_call(node.method_name, symbol.symbol_type, node.position)
        
        # Analyze arguments
        for arg in node.arguments:
            arg.accept(self)
    
    def visit_assignment(self, node: Assignment) -> None:
        """Analyze simple assignment statements."""
        # Analyze the target (usually a variable reference)
        node.target.accept(self)
        
        # Analyze the value
        node.value.accept(self)
        
        # For variable assignments, check type compatibility if needed
        if isinstance(node.target, VariableRef):
            symbol = self.symbol_table.lookup_symbol(node.target.name)
            # Note: No need to report undefined variable error here since
            # visit_variable_ref already handles that when analyzing the target
    
    def visit_index_assignment(self, node: IndexAssignment) -> None:
        """Analyze index assignments."""
        # Analyze target
        node.target.accept(self)
        
        # Analyze value
        node.value.accept(self)
        
        # TODO: Type check assignment
    
    def visit_slice_assignment(self, node: SliceAssignment) -> None:
        """Analyze slice assignments."""
        # Analyze target
        node.target.accept(self)
        
        # Analyze value
        node.value.accept(self)
        
        # TODO: Type check assignment
    
    def visit_expression_statement(self, node: ExpressionStatement) -> None:
        """Analyze expression statements."""
        node.expression.accept(self)
    
    def visit_legacy_command(self, node: LegacyCommand) -> None:
        """Analyze legacy commands (no semantic checks needed)."""
        pass
    
    # Expression visitors
    
    def visit_variable_ref(self, node: VariableRef) -> None:
        """Analyze variable references."""
        symbol = self.symbol_table.lookup_symbol(node.name)
        if not symbol:
            self.report_error(UndefinedVariableError(node.name, node.position))
    
    def visit_string_literal(self, node: StringLiteral) -> None:
        """Analyze string literals (no checks needed)."""
        pass
    
    def visit_number_literal(self, node: NumberLiteral) -> None:
        """Analyze number literals (no checks needed)."""
        pass
    
    def visit_boolean_literal(self, node: BooleanLiteral) -> None:
        """Analyze boolean literals (no checks needed)."""
        pass
    
    def visit_list_literal(self, node: ListLiteral) -> None:
        """Analyze list literals."""
        # Analyze all elements
        for element in node.elements:
            element.accept(self)
        
        # TODO: Check type consistency if this is for a constrained list
    
    def visit_index_access(self, node: IndexAccess) -> None:
        """Analyze index access expressions."""
        # Analyze target
        node.target.accept(self)
        
        # Analyze indices
        for index in node.indices:
            index.accept(self)
        
        # Check if target is indexable
        if isinstance(node.target, VariableRef):
            symbol = self.symbol_table.lookup_symbol(node.target.name)
            if symbol and symbol.symbol_type not in {'list', 'string'}:
                self.report_error(InvalidMethodCallError(
                    "index access", symbol.symbol_type,
                    "Type is not indexable", node.position))
    
    def visit_slice_access(self, node: SliceAccess) -> None:
        """Analyze slice access expressions."""
        # Analyze target  
        node.target.accept(self)
        
        # Analyze slice components
        if node.start:
            node.start.accept(self)
        if node.stop:
            node.stop.accept(self)
        if node.step:
            node.step.accept(self)
        
        # Check if target is sliceable
        if isinstance(node.target, VariableRef):
            symbol = self.symbol_table.lookup_symbol(node.target.name)
            if symbol and symbol.symbol_type not in {'list', 'string'}:
                self.report_error(InvalidMethodCallError(
                    "slice access", symbol.symbol_type,
                    "Type is not sliceable", node.position))
    
    def visit_method_call_expression(self, node: MethodCallExpression) -> None:
        """Analyze method call expressions."""
        # Analyze target
        node.target.accept(self)
        
        # Analyze arguments
        for arg in node.arguments:
            arg.accept(self)
        
        # Check method validity
        if isinstance(node.target, VariableRef):
            symbol = self.symbol_table.lookup_symbol(node.target.name)
            if symbol:
                self._validate_method_call(node.method_name, symbol.symbol_type, node.position)
    
    # Helper methods
    
    def _validate_method_call(self, method_name: str, target_type: str, 
                            position: Optional[SourcePosition]) -> None:
        """Validate that a method call is valid for the target type."""
        # Define valid methods for each type
        valid_methods = {
            'list': {
                'append', 'prepend', 'insert', 'remove', 'pop', 'clear', 'reverse',
                'size', 'empty', 'constraint', 'validate_constraint', 'type_summary',
                'types', 'coerce_to_constraint'
            },
            'string': {'size', 'empty', 'upper', 'lower', 'split'},
            'num': {'abs', 'round'},
            'bool': {}  # Boolean values don't have methods currently
        }
        
        if target_type not in valid_methods:
            self.report_error(InvalidMethodCallError(
                method_name, target_type, f"Unknown type '{target_type}'", position))
            return
        
        if method_name not in valid_methods[target_type]:
            available = ", ".join(sorted(valid_methods[target_type]))
            reason = f"Available methods: {available}" if available else "No methods available"
            self.report_error(InvalidMethodCallError(
                method_name, target_type, reason, position))
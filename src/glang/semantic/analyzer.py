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


class SemanticAnalyzer(BaseASTVisitor):
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
        valid_types = {'list', 'string', 'num', 'bool', 'data', 'map'}
        if node.var_type not in valid_types:
            self.report_error(InvalidTypeError(node.var_type, node.position))
            return
        
        # Validate constraint if present
        if node.type_constraint:
            valid_constraints = {'string', 'num', 'bool', 'list', 'data', 'map'}
            if node.type_constraint not in valid_constraints:
                self.report_error(InvalidConstraintError(
                    node.type_constraint, node.var_type, node.position))
                return
            
            # Only list, data, and map types support constraints currently
            if node.var_type not in ['list', 'data', 'map']:
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
        
        # Special handling for variable assignments to support type inference
        if isinstance(node.target, VariableRef):
            var_name = node.target.name
            symbol = self.symbol_table.lookup_symbol(var_name)
            
            if not symbol:
                # NEW: Type inference - treat as implicit variable declaration
                # Analyze the value to ensure it's valid
                node.value.accept(self)
                
                # Create a symbol for the new variable (type will be inferred at runtime)
                inferred_symbol = Symbol(var_name, "inferred", None, node.target.position)
                self.symbol_table.declare_symbol(inferred_symbol)
            else:
                # Variable exists - analyze normally
                node.target.accept(self)
                node.value.accept(self)
        else:
            # Non-variable assignment (index assignment, etc.) - analyze normally
            node.target.accept(self)
            node.value.accept(self)
    
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
    
    def visit_load_statement(self, node: LoadStatement) -> None:
        """Analyze load statements."""
        # For load statements, we just need to validate the filename is a string
        # The actual file loading is handled at execution time
        if not isinstance(node.filename, str):
            self.errors.append(SemanticError(
                f"Load statement filename must be a string, got {type(node.filename).__name__}",
                node.position
            ))
    
    def visit_print_statement(self, node: 'PrintStatement') -> None:
        """Analyze print statements."""
        from ..ast.nodes import PrintStatement
        
        # Analyze all arguments to ensure they're valid expressions
        for arg in node.arguments:
            arg.accept(self)
    
    def visit_print_expression(self, node: 'PrintExpression') -> None:
        """Analyze print expressions."""
        from ..ast.nodes import PrintExpression
        
        # Analyze all arguments to ensure they're valid expressions
        for arg in node.arguments:
            arg.accept(self)
    
    
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
    
    def visit_data_node_literal(self, node: DataNodeLiteral) -> None:
        """Analyze data node literals."""
        # Analyze the value expression
        node.value.accept(self)
        # Key is a string literal, no need to validate further
    
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
            if symbol and symbol.symbol_type not in {'list', 'string', 'map'}:
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
        # Check if this might be module-qualified access (e.g., math.pi with no arguments)
        from ..ast.nodes import VariableRef
        is_module_qualified = (isinstance(node.target, VariableRef) and 
                             len(node.arguments) == 0)
        
        if is_module_qualified:
            # For potential module access, we don't analyze the target as a variable
            # The execution system will handle module resolution
            pass
        else:
            # Analyze target for regular method calls
            node.target.accept(self)
            
            # Check method validity
            if isinstance(node.target, VariableRef):
                symbol = self.symbol_table.lookup_symbol(node.target.name)
                if symbol:
                    self._validate_method_call(node.method_name, symbol.symbol_type, node.position)
        
        # Always analyze arguments
        for arg in node.arguments:
            arg.accept(self)
    
    def visit_binary_operation(self, node: BinaryOperation) -> None:
        """Analyze binary operations."""
        # Analyze both operands
        node.left.accept(self)
        node.right.accept(self)
        
        # TODO: Add type checking for binary operations
        # For now, we just validate that operands are analyzed
    
    def visit_unary_operation(self, node: UnaryOperation) -> None:
        """Analyze unary operations."""
        # Analyze operand
        node.operand.accept(self)
        
        # TODO: Add type checking for unary operations
        # For now, we just validate that operand is analyzed
    
    # Helper methods
    
    def _validate_method_call(self, method_name: str, target_type: str, 
                            position: Optional[SourcePosition]) -> None:
        """Validate that a method call is valid for the target type."""
        # Define valid methods for each type
        # Universal reflection methods available on all types
        universal_methods = {'type', 'methods', 'can', 'inspect', 'size'}
        
        valid_methods = {
            'list': {
                'append', 'prepend', 'insert', 'remove', 'pop', 'clear', 'reverse',
                'size', 'empty', 'constraint', 'validate_constraint', 'type_summary',
                'types', 'coerce_to_constraint', 'indexOf', 'count', 'min', 'max', 'sum', 'sort'
            } | universal_methods,
            'string': {
                'size', 'empty', 'upper', 'lower', 'split', 'trim', 'join',
                'matches', 'replace', 'findAll',
                'length', 'contains', 'up', 'toUpper', 'down', 'toLower',
                'reverse', 'unique', 'chars'
            } | universal_methods,
            'num': {'abs', 'round', 'to'} | universal_methods,
            'bool': {'flip', 'toggle', 'numify', 'toNum'} | universal_methods,
            'data': {'key', 'value'} | universal_methods,
            'map': {
                'get', 'set', 'has_key', 'count_values', 'keys', 'values', 'remove', 'empty', 'merge', 'push', 'pop'
            } | universal_methods
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
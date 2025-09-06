"""AST node definitions for glang language."""

from abc import ABC, abstractmethod
from typing import List, Optional, Union, Any
from dataclasses import dataclass

@dataclass
class SourcePosition:
    """Position information for error reporting and debugging."""
    line: int
    column: int
    
    def __str__(self):
        return f"line {self.line}, column {self.column}"

class ASTNode(ABC):
    """Base class for all AST nodes."""
    
    def __init__(self, position: Optional[SourcePosition] = None):
        self.position = position
    
    @abstractmethod
    def accept(self, visitor): 
        """Accept a visitor for traversal."""
        pass

# =============================================================================
# Expression Nodes - represent values and computations
# =============================================================================

class Expression(ASTNode):
    """Base class for all expressions (things that evaluate to values)."""
    pass

@dataclass
class VariableRef(Expression):
    """Reference to a variable: myvar"""
    name: str
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_variable_ref(self)

@dataclass  
class StringLiteral(Expression):
    """String literal: "hello world" """
    value: str
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_string_literal(self)

@dataclass
class NumberLiteral(Expression):
    """Number literal: 42 or 3.14"""
    value: Union[int, float]
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_number_literal(self)

@dataclass
class BooleanLiteral(Expression):
    """Boolean literal: true or false"""
    value: bool
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_boolean_literal(self)

@dataclass
class ListLiteral(Expression):
    """List literal: [1, 2, 3] or ["a", "b"]"""
    elements: List[Expression]
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_list_literal(self)

@dataclass
class IndexAccess(Expression):
    """Index access: arr[0] or matrix[1][2]"""
    target: Expression
    indices: List[Expression]  # Support multi-dimensional indexing
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_index_access(self)

@dataclass
class SliceAccess(Expression):
    """Slice access: arr[1:3] or arr[::2]"""
    target: Expression
    start: Optional[Expression]
    stop: Optional[Expression] 
    step: Optional[Expression]
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_slice_access(self)

@dataclass  
class MethodCallExpression(Expression):
    """Method call in expression context: obj.method(args)"""
    target: Expression
    method_name: str
    arguments: List[Expression]
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_method_call_expression(self)

# =============================================================================
# Statement Nodes - represent actions and declarations
# =============================================================================

class Statement(ASTNode):
    """Base class for all statements (things that perform actions)."""
    pass

@dataclass
class VariableDeclaration(Statement):
    """Variable declaration: string name = "value" or list<num> nums = [1, 2]"""
    var_type: str  # 'string', 'num', 'bool', 'list'
    name: str
    initializer: Expression
    type_constraint: Optional[str] = None  # For list<num> etc
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_variable_declaration(self)

@dataclass
class MethodCall(Statement):
    """Method call statement: obj.method(args)"""
    target: Expression  # The object being called on
    method_name: str
    arguments: List[Expression]  # Properly typed arguments
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_method_call(self)

@dataclass
class Assignment(Statement):
    """Simple assignment: variable = value"""
    target: Expression  # Usually VariableRef, but could be IndexAccess
    value: Expression
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_assignment(self)

@dataclass
class IndexAssignment(Statement):
    """Index assignment: arr[0] = value"""
    target: IndexAccess  # The indexed expression
    value: Expression
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_index_assignment(self)

@dataclass
class SliceAssignment(Statement):
    """Slice assignment: arr[1:3] = [a, b]"""
    target: SliceAccess  # The slice expression  
    value: Expression
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_slice_assignment(self)

@dataclass
class ExpressionStatement(Statement):
    """Standalone expression: variable_name or some_function()"""
    expression: Expression
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_expression_statement(self)

@dataclass
class LoadStatement(Statement):
    """Load statement: load "filename.gr" """
    filename: str  # The file to load
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_load_statement(self)

@dataclass 
class LegacyCommand(Statement):
    """Legacy slash commands: /help, /show, etc."""
    command: str
    arguments: List[str]
    raw_input: str
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_legacy_command(self)

@dataclass
class NoOp(Statement):
    """No-operation statement (e.g., comment-only lines, empty lines)."""
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_noop(self)

# =============================================================================
# Visitor Pattern for AST Traversal
# =============================================================================

class ASTVisitor(ABC):
    """Abstract visitor for AST traversal and processing."""
    
    # Expression visitors
    @abstractmethod
    def visit_variable_ref(self, node: VariableRef): 
        """Visit a variable reference."""
        pass
    
    @abstractmethod
    def visit_string_literal(self, node: StringLiteral): 
        """Visit a string literal."""
        pass
    
    @abstractmethod
    def visit_number_literal(self, node: NumberLiteral): 
        """Visit a number literal."""
        pass
    
    @abstractmethod  
    def visit_boolean_literal(self, node: BooleanLiteral): 
        """Visit a boolean literal."""
        pass
    
    @abstractmethod
    def visit_list_literal(self, node: ListLiteral): 
        """Visit a list literal."""
        pass
    
    @abstractmethod
    def visit_index_access(self, node: IndexAccess): 
        """Visit an index access expression."""
        pass
    
    @abstractmethod
    def visit_slice_access(self, node: SliceAccess): 
        """Visit a slice access expression."""
        pass
    
    @abstractmethod
    def visit_method_call_expression(self, node: MethodCallExpression):
        """Visit a method call in expression context."""
        pass
    
    # Statement visitors
    @abstractmethod
    def visit_variable_declaration(self, node: VariableDeclaration): 
        """Visit a variable declaration."""
        pass
    
    @abstractmethod
    def visit_method_call(self, node: MethodCall): 
        """Visit a method call statement."""
        pass
    
    @abstractmethod
    def visit_assignment(self, node: Assignment):
        """Visit a simple assignment."""
        pass
    
    @abstractmethod
    def visit_index_assignment(self, node: IndexAssignment): 
        """Visit an index assignment."""
        pass
    
    @abstractmethod
    def visit_slice_assignment(self, node: SliceAssignment): 
        """Visit a slice assignment."""
        pass
    
    @abstractmethod
    def visit_expression_statement(self, node: ExpressionStatement): 
        """Visit an expression statement."""
        pass
    
    @abstractmethod
    def visit_load_statement(self, node: LoadStatement):
        """Visit a load statement."""
        pass
    
    @abstractmethod
    def visit_legacy_command(self, node: LegacyCommand):
        """Visit a legacy command."""
        pass
    
    @abstractmethod
    def visit_noop(self, node: NoOp):
        """Visit a no-op statement."""
        pass

# =============================================================================
# Utility Classes  
# =============================================================================

class BaseASTVisitor(ASTVisitor):
    """Base visitor implementation with default behaviors."""
    
    def visit_variable_ref(self, node: VariableRef):
        return node
    
    def visit_string_literal(self, node: StringLiteral):
        return node
    
    def visit_number_literal(self, node: NumberLiteral):
        return node
    
    def visit_boolean_literal(self, node: BooleanLiteral):
        return node
    
    def visit_list_literal(self, node: ListLiteral):
        for element in node.elements:
            element.accept(self)
        return node
    
    def visit_index_access(self, node: IndexAccess):
        node.target.accept(self)
        for index in node.indices:
            index.accept(self)
        return node
    
    def visit_slice_access(self, node: SliceAccess):
        node.target.accept(self)
        if node.start:
            node.start.accept(self)
        if node.stop:
            node.stop.accept(self)
        if node.step:
            node.step.accept(self)
        return node
    
    def visit_method_call_expression(self, node: MethodCallExpression):
        node.target.accept(self)
        for arg in node.arguments:
            arg.accept(self)
        return node
    
    def visit_variable_declaration(self, node: VariableDeclaration):
        node.initializer.accept(self)
        return node
    
    def visit_method_call(self, node: MethodCall):
        node.target.accept(self)
        for arg in node.arguments:
            arg.accept(self)
        return node
    
    def visit_assignment(self, node: Assignment):
        node.target.accept(self)
        node.value.accept(self)
        return node
    
    def visit_index_assignment(self, node: IndexAssignment):
        node.target.accept(self)
        node.value.accept(self)
        return node
    
    def visit_slice_assignment(self, node: SliceAssignment):
        node.target.accept(self)
        node.value.accept(self)
        return node
    
    def visit_expression_statement(self, node: ExpressionStatement):
        node.expression.accept(self)
        return node
    
    def visit_load_statement(self, node: LoadStatement):
        return node
    
    def visit_legacy_command(self, node: LegacyCommand):
        return node
    
    def visit_noop(self, node: NoOp):
        return node
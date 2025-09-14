"""AST node definitions for glang language."""

from abc import ABC, abstractmethod
from typing import List, Optional, Union, Any, Tuple
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
class SymbolLiteral(Expression):
    """Symbol literal: :ok, :error, :pending, etc."""
    name: str  # The symbol name without the colon
    position: Optional[SourcePosition] = None

    def accept(self, visitor):
        return visitor.visit_symbol_literal(self)

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

@dataclass
class PrintExpression(Expression):
    """Print function call expression: print(args) or print args"""
    arguments: List[Expression]  # The expressions to print
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_print_expression(self)

@dataclass
class BinaryOperation(Expression):
    """Binary operation: left op right (e.g., 5 + 3, a > b)"""
    left: Expression
    operator: str  # "+", "-", "*", "/", "%", ">", "<", "==", "!=", ">=", "<=", "!>", "!<"
    right: Expression
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_binary_operation(self)

@dataclass
class UnaryOperation(Expression):
    """Unary operation: op operand (e.g., -5, !flag)"""
    operator: str  # "-", "!"
    operand: Expression
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_unary_operation(self)

@dataclass
class DataNodeLiteral(Expression):
    """Data node literal: { "key": value }"""
    key: str  # The key (must be a string literal)
    value: Expression  # The value (can be any expression)
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_data_node_literal(self)

@dataclass
class MapLiteral(Expression):
    """Map literal: { "key1": value1, "key2": value2, ... }"""
    pairs: List[Tuple[str, Expression]]  # List of (key, value) pairs
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_map_literal(self)

# =============================================================================
# Statement Nodes - represent actions and declarations
# =============================================================================

class Statement(ASTNode):
    """Base class for all statements (things that perform actions)."""
    pass

@dataclass
class BehaviorCall(Expression):
    """Behavior call with parameters: validate_range(95, 105)"""
    name: str
    arguments: List[Expression]
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_behavior_call(self)

@dataclass  
class BehaviorList(Expression):
    """List of behaviors: [nil_to_zero, validate_range(0, 100)]"""
    behaviors: List[Union[str, 'BehaviorCall']]
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_behavior_list(self)

@dataclass
class VariableDeclaration(Statement):
    """Variable declaration: string name = "value" or list<num> nums = [1, 2]"""
    var_type: str  # 'string', 'num', 'bool', 'list'
    name: str
    initializer: Expression
    type_constraint: Optional[str] = None  # For list<num> etc
    behaviors: Optional[BehaviorList] = None  # For with [behaviors...]
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
class PrintStatement(Statement):
    """Print statement: print(expression1, expression2, ...)"""
    arguments: List[Expression]  # The expressions to print
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_print_statement(self)

@dataclass
class ImportStatement(Statement):
    """Import statement: /import "filename.gr" as module_name"""
    filename: str  # The file to import
    alias: Optional[str] = None  # Optional alias (e.g., 'as math')
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_import_statement(self)

@dataclass
class ModuleDeclaration(Statement):
    """Module declaration: module module_name"""
    name: str  # The declared module name
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_module_declaration(self)

@dataclass
class AliasDeclaration(Statement):
    """Alias declaration: alias short_name"""
    name: str  # The alias/short name for the module
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_alias_declaration(self)

@dataclass
class NoOp(Statement):
    """No-operation statement (e.g., comment-only lines, empty lines)."""
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_noop(self)

# =============================================================================
# Control Flow Statements
# =============================================================================

@dataclass
class Block(ASTNode):
    """Block of statements: { stmt1; stmt2; ... }"""
    statements: List[Statement]
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_block(self)

@dataclass
class PrecisionBlock(Statement):
    """Precision context: precision <value> { body }"""
    precision_value: Expression  # Expression that evaluates to precision (integer)
    body: Block
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_precision_block(self)

@dataclass 
class IfStatement(Statement):
    """If statement: if condition { then_block } else { else_block }"""
    condition: Expression
    then_block: Block
    else_block: Optional[Block] = None
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_if_statement(self)

@dataclass
class WhileStatement(Statement):
    """While loop: while condition { body }"""
    condition: Expression
    body: Block
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_while_statement(self)

@dataclass
class ForInStatement(Statement):
    """For-in loop: for variable in iterable { body }"""
    variable: str  # Loop variable name
    iterable: Expression  # Expression that evaluates to an iterable
    body: Block
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_for_in_statement(self)

@dataclass
class BreakStatement(Statement):
    """Break statement: break"""
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_break_statement(self)

@dataclass  
class ContinueStatement(Statement):
    """Continue statement: continue"""
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_continue_statement(self)


@dataclass
class FunctionDeclaration(Statement):
    """Function declaration: func name(param1, param2) { body }"""
    name: str
    parameters: List[str]  # Parameter names
    body: 'Block'
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_function_declaration(self)


@dataclass
class ReturnStatement(Statement):
    """Return statement: return expression"""
    value: Optional[Expression] = None  # None for bare 'return'
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_return_statement(self)


@dataclass  
class FunctionCall(Expression):
    """Function call expression: func_name(arg1, arg2)"""
    name: str
    arguments: List[Expression]
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_function_call(self)


@dataclass
class LambdaExpression(Expression):
    """Lambda expression: param => expression or (param1, param2) => expression"""
    parameters: List[str]  # Parameter names
    body: Expression  # Single expression (not a block)
    position: Optional[SourcePosition] = None
    
    def accept(self, visitor):
        return visitor.visit_lambda_expression(self)

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
    
    @abstractmethod
    def visit_print_expression(self, node: PrintExpression):
        """Visit a print expression."""
        pass
    
    @abstractmethod
    def visit_binary_operation(self, node: BinaryOperation):
        """Visit a binary operation."""
        pass
    
    @abstractmethod
    def visit_unary_operation(self, node: UnaryOperation):
        """Visit a unary operation."""
        pass
    
    @abstractmethod
    def visit_data_node_literal(self, node: DataNodeLiteral):
        """Visit a data node literal."""
        pass
    
    @abstractmethod
    def visit_map_literal(self, node: MapLiteral):
        """Visit a map literal."""
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
    def visit_print_statement(self, node: PrintStatement):
        """Visit a print statement."""
        pass
    
    @abstractmethod
    def visit_import_statement(self, node: ImportStatement):
        """Visit an import statement."""
        pass
    
    @abstractmethod
    def visit_precision_block(self, node: PrecisionBlock):
        """Visit a precision block."""
        pass
    
    @abstractmethod
    def visit_module_declaration(self, node: ModuleDeclaration):
        """Visit a module declaration."""
        pass
    
    @abstractmethod
    def visit_alias_declaration(self, node: AliasDeclaration):
        """Visit an alias declaration."""
        pass
    
    @abstractmethod
    def visit_noop(self, node: NoOp):
        """Visit a no-op statement."""
        pass
    
    # Control flow visitors
    @abstractmethod
    def visit_block(self, node: Block):
        """Visit a block of statements."""
        pass
    
    @abstractmethod
    def visit_if_statement(self, node: IfStatement):
        """Visit an if statement."""
        pass
    
    @abstractmethod
    def visit_while_statement(self, node: WhileStatement):
        """Visit a while statement."""
        pass
    
    @abstractmethod
    def visit_for_in_statement(self, node: ForInStatement):
        """Visit a for-in statement."""
        pass
    
    @abstractmethod
    def visit_break_statement(self, node: BreakStatement):
        """Visit a break statement."""
        pass
    
    @abstractmethod
    def visit_continue_statement(self, node: ContinueStatement):
        """Visit a continue statement."""
        pass
    
    @abstractmethod
    def visit_function_declaration(self, node: FunctionDeclaration):
        """Visit a function declaration."""
        pass
    
    @abstractmethod
    def visit_return_statement(self, node: ReturnStatement):
        """Visit a return statement."""
        pass
    
    @abstractmethod
    def visit_function_call(self, node: FunctionCall):
        """Visit a function call."""
        pass
    
    @abstractmethod
    def visit_lambda_expression(self, node: LambdaExpression):
        """Visit a lambda expression."""
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
    
    def visit_print_expression(self, node: PrintExpression):
        for arg in node.arguments:
            arg.accept(self)
        return node
    
    def visit_binary_operation(self, node: BinaryOperation):
        node.left.accept(self)
        node.right.accept(self)
        return node
    
    def visit_unary_operation(self, node: UnaryOperation):
        node.operand.accept(self)
        return node
    
    def visit_data_node_literal(self, node: DataNodeLiteral):
        node.value.accept(self)
        return node
    
    def visit_map_literal(self, node: MapLiteral):
        for key, value in node.pairs:
            value.accept(self)
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
    
    def visit_print_statement(self, node: PrintStatement):
        for arg in node.arguments:
            arg.accept(self)
        return node
    
    def visit_import_statement(self, node: ImportStatement):
        return node
    
    def visit_precision_block(self, node: PrecisionBlock):
        node.precision_value.accept(self)
        node.body.accept(self)
        return node
    
    def visit_module_declaration(self, node: ModuleDeclaration):
        return node
    
    def visit_alias_declaration(self, node: AliasDeclaration):
        return node
    
    def visit_noop(self, node: NoOp):
        return node
    
    # Control flow visitor implementations
    def visit_block(self, node: Block):
        for stmt in node.statements:
            stmt.accept(self)
        return node
    
    def visit_if_statement(self, node: IfStatement):
        node.condition.accept(self)
        node.then_block.accept(self)
        if node.else_block:
            node.else_block.accept(self)
        return node
    
    def visit_while_statement(self, node: WhileStatement):
        node.condition.accept(self)
        node.body.accept(self)
        return node
    
    def visit_for_in_statement(self, node: ForInStatement):
        node.iterable.accept(self)
        node.body.accept(self)
        return node
    
    def visit_break_statement(self, node: BreakStatement):
        return node
    
    def visit_continue_statement(self, node: ContinueStatement):
        return node
    
    def visit_function_declaration(self, node: FunctionDeclaration):
        node.body.accept(self)
        return node
    
    def visit_return_statement(self, node: ReturnStatement):
        if node.value:
            node.value.accept(self)
        return node
    
    def visit_function_call(self, node: FunctionCall):
        for arg in node.arguments:
            arg.accept(self)
        return node
    
    def visit_lambda_expression(self, node: LambdaExpression):
        node.body.accept(self)
        return node

    def visit_match_expression(self, node: 'MatchExpression'):
        node.expr.accept(self)
        for arm in node.arms:
            arm.pattern.accept(self)
            arm.result.accept(self)
            if arm.guard:
                arm.guard.accept(self)
        return node

    def visit_literal_pattern(self, node: 'LiteralPattern'):
        return node

    def visit_variable_pattern(self, node: 'VariablePattern'):
        return node

    def visit_wildcard_pattern(self, node: 'WildcardPattern'):
        return node

    def visit_list_pattern(self, node: 'ListPattern'):
        for element in node.elements:
            element.accept(self)
        return node


# Pattern Matching AST Nodes

class Pattern(ASTNode):
    """Base class for all patterns in match expressions."""
    pass

@dataclass
class MatchExpression(Expression):
    """Match expression: match expr { pattern => result, ... }"""
    expr: Expression
    arms: List['MatchArm']
    position: Optional[SourcePosition] = None

    def accept(self, visitor):
        return visitor.visit_match_expression(self)

@dataclass
class MatchArm:
    """Single match arm: pattern => result"""
    pattern: Pattern
    result: Expression
    guard: Optional[Expression] = None  # Future: for 'if' guards
    position: Optional[SourcePosition] = None

@dataclass
class LiteralPattern(Pattern):
    """Literal value pattern: 42, "hello", true, :ok"""
    value: Any
    position: Optional[SourcePosition] = None

    def accept(self, visitor):
        return visitor.visit_literal_pattern(self)

@dataclass
class VariablePattern(Pattern):
    """Variable binding pattern: x, value, message"""
    name: str
    position: Optional[SourcePosition] = None

    def accept(self, visitor):
        return visitor.visit_variable_pattern(self)

@dataclass
class WildcardPattern(Pattern):
    """Wildcard pattern: _"""
    position: Optional[SourcePosition] = None

    def accept(self, visitor):
        return visitor.visit_wildcard_pattern(self)

@dataclass
class ListPattern(Pattern):
    """List pattern: [], [a, b], [first, ...rest]"""
    elements: List[Pattern]
    rest_variable: Optional[str] = None  # For ...rest syntax
    position: Optional[SourcePosition] = None

    def accept(self, visitor):
        return visitor.visit_list_pattern(self)
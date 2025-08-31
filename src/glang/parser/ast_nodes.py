"""AST node definitions for glang syntax parser."""

from dataclasses import dataclass
from enum import Enum, auto
from typing import List, Optional, Any


class InputType(Enum):
    """Types of input that can be parsed."""
    VARIABLE_DECLARATION = auto()  # list fruits = [...]
    METHOD_CALL = auto()           # fruits.append value
    VARIABLE_ACCESS = auto()       # fruits
    LEGACY_COMMAND = auto()        # create fruits [...]


@dataclass
class ParsedCommand:
    """Base class for all parsed commands."""
    raw_input: str
    input_type: InputType = None


@dataclass
class VariableDeclaration(ParsedCommand):
    """Represents a variable declaration.
    
    Examples:
        list fruits = [apple, banana]
        graph g = directed()
    """
    graph_type: str = None  # 'list', 'graph', 'directed', etc.
    variable_name: str = None
    initializer: Optional[List[Any]] = None
    input_type: InputType = InputType.VARIABLE_DECLARATION


@dataclass
class MethodCall(ParsedCommand):
    """Represents a method call on a variable.
    
    Examples:
        fruits.append cherry
        numbers.reverse()
    """
    variable_name: str = None
    method_name: str = None
    arguments: List[Any] = None
    input_type: InputType = InputType.METHOD_CALL


@dataclass
class VariableAccess(ParsedCommand):
    """Represents accessing a variable to display its contents.
    
    Examples:
        fruits
        fruits --show-nodes
    """
    variable_name: str = None
    flags: List[str] = None
    input_type: InputType = InputType.VARIABLE_ACCESS
    
    def __post_init__(self):
        if self.flags is None:
            self.flags = []


@dataclass
class LegacyCommand(ParsedCommand):
    """Represents a legacy command format.
    
    Examples:
        create fruits [apple, banana]
        show fruits
        append cherry
    """
    command: str = None
    arguments: List[str] = None
    input_type: InputType = InputType.LEGACY_COMMAND
    
    def __post_init__(self):
        if self.arguments is None:
            self.arguments = []
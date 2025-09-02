"""AST node definitions for glang syntax parser."""

from dataclasses import dataclass
from enum import Enum, auto
from typing import List, Optional, Any


class InputType(Enum):
    """Types of input that can be parsed."""
    VARIABLE_DECLARATION = auto()  # list fruits = [...]
    METHOD_CALL = auto()           # fruits.append value
    VARIABLE_ACCESS = auto()       # fruits
    INDEX_ACCESS = auto()          # fruits[0], fruits[-1]
    INDEX_ASSIGNMENT = auto()      # fruits[0] = 'mango'
    SLICE_ACCESS = auto()          # fruits[1:3], fruits[::2]
    SLICE_ASSIGNMENT = auto()      # fruits[1:3] = [a, b]
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
        list<num> scores = [95, 87, 92]
    """
    graph_type: str = None  # 'list', 'graph', 'directed', etc.
    variable_name: str = None
    initializer: Optional[List[Any]] = None
    type_constraint: Optional[str] = None  # 'num', 'string', 'bool', etc.
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
class IndexAccess(ParsedCommand):
    """Represents accessing an element of a variable by index.
    
    Examples:
        fruits[0]
        numbers[-1]
        matrix[1][2]
    """
    variable_name: str = None
    indices: List[int] = None
    input_type: InputType = InputType.INDEX_ACCESS
    
    def __post_init__(self):
        if self.indices is None:
            self.indices = []


@dataclass
class IndexAssignment(ParsedCommand):
    """Represents assigning a value to an indexed location.
    
    Examples:
        fruits[0] = 'mango'
        matrix[1][2] = 42
        items[-1] = 'last'
    """
    variable_name: str = None
    indices: List[int] = None
    value: Any = None
    input_type: InputType = InputType.INDEX_ASSIGNMENT
    
    def __post_init__(self):
        if self.indices is None:
            self.indices = []


@dataclass
class SliceAccess(ParsedCommand):
    """Represents accessing a slice of a variable.
    
    Examples:
        fruits[1:3]
        numbers[::2]
        items[1:]
        data[:-1]
    """
    variable_name: str = None
    start: Optional[int] = None
    stop: Optional[int] = None
    step: Optional[int] = None
    input_type: InputType = InputType.SLICE_ACCESS


@dataclass
class SliceAssignment(ParsedCommand):
    """Represents assigning values to a slice of a variable.
    
    Examples:
        fruits[1:3] = [a, b]
        numbers[::2] = [1, 3, 5]
    """
    variable_name: str = None
    start: Optional[int] = None
    stop: Optional[int] = None
    step: Optional[int] = None
    value: Any = None
    input_type: InputType = InputType.SLICE_ASSIGNMENT


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
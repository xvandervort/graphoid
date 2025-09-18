"""
Enhanced Stack Trace System for Glang

Provides detailed execution context including call chains, variable states,
and source locations for comprehensive error reporting.
"""

from typing import List, Dict, Any, Optional
from dataclasses import dataclass
from ..ast.nodes import SourcePosition


@dataclass
class StackFrame:
    """Represents a single frame in the execution stack."""
    function_name: str
    source_position: Optional[SourcePosition]
    local_variables: Dict[str, Any]
    arguments: Dict[str, Any]
    source_line: Optional[str] = None

    def __str__(self) -> str:
        """String representation of stack frame."""
        pos_str = ""
        if self.source_position:
            pos_str = f" at line {self.source_position.line}, column {self.source_position.column}"

        return f"  in {self.function_name}(){pos_str}"


@dataclass
class EnhancedStackTrace:
    """Complete stack trace with execution context."""
    frames: List[StackFrame]
    error_message: str
    error_type: str
    source_code: Optional[str] = None
    source_name: str = "<input>"

    def format_full_trace(self) -> str:
        """Format complete stack trace with context."""
        lines = []

        # Error header
        lines.append(f"Traceback (most recent call last):")

        # Stack frames (reverse order - most recent last)
        for frame in reversed(self.frames):
            lines.append(str(frame))

            # Add source line if available
            if frame.source_line and frame.source_position:
                lines.append(f"    {frame.source_line.strip()}")

                # Add pointer if we have column info
                if frame.source_position.column > 0:
                    pointer = f"    {'~' * (frame.source_position.column - 1)}^"
                    lines.append(pointer)

            # Add local variables if any (limit to important ones)
            if frame.local_variables:
                important_vars = self._filter_important_variables(frame.local_variables)
                if important_vars:
                    lines.append(f"    Local variables: {important_vars}")

        # Final error
        lines.append(f"{self.error_type}: {self.error_message}")

        return "\n".join(lines)

    def format_compact_trace(self) -> str:
        """Format compact stack trace for quick debugging."""
        if not self.frames:
            return f"{self.error_type}: {self.error_message}"

        # Show just the error location and call chain
        call_chain = " → ".join([frame.function_name for frame in reversed(self.frames)])

        last_frame = self.frames[-1] if self.frames else None
        location = ""
        if last_frame and last_frame.source_position:
            location = f" at line {last_frame.source_position.line}"

        return f"{self.error_type}: {self.error_message}{location}\n  Call chain: {call_chain}"

    def _filter_important_variables(self, variables: Dict[str, Any]) -> Dict[str, str]:
        """Filter and format important variables for display."""
        important = {}

        # Show user-defined variables (not internal ones)
        for name, value in variables.items():
            if not name.startswith('_') and not name.startswith('__'):
                # Limit string representation length
                str_value = str(value)
                if len(str_value) > 50:
                    str_value = str_value[:47] + "..."
                important[name] = str_value

                # Limit to 3 most important variables
                if len(important) >= 3:
                    break

        return important


class StackTraceCollector:
    """Collects stack trace information during execution."""

    def __init__(self):
        self.frames: List[StackFrame] = []
        self.source_lines: Dict[int, str] = {}
        self.current_source_code: Optional[str] = None

    def push_frame(
        self,
        function_name: str,
        position: Optional[SourcePosition] = None,
        arguments: Optional[Dict[str, Any]] = None
    ):
        """Push a new frame onto the stack."""
        frame = StackFrame(
            function_name=function_name,
            source_position=position,
            local_variables={},
            arguments=arguments or {},
            source_line=self._get_source_line(position) if position else None
        )
        self.frames.append(frame)

    def pop_frame(self):
        """Pop the top frame from the stack."""
        if self.frames:
            self.frames.pop()

    def update_local_variables(self, variables: Dict[str, Any]):
        """Update local variables for the current frame."""
        if self.frames:
            self.frames[-1].local_variables.update(variables)

    def set_source_code(self, source_code: str):
        """Set the source code for line lookup."""
        self.current_source_code = source_code
        self.source_lines = {}

        # Cache source lines for quick lookup
        for i, line in enumerate(source_code.split('\n'), 1):
            self.source_lines[i] = line

    def create_enhanced_trace(
        self,
        error_message: str,
        error_type: str,
        source_name: str = "<input>"
    ) -> EnhancedStackTrace:
        """Create enhanced stack trace from current state."""
        return EnhancedStackTrace(
            frames=self.frames.copy(),
            error_message=error_message,
            error_type=error_type,
            source_code=self.current_source_code,
            source_name=source_name
        )

    def _get_source_line(self, position: SourcePosition) -> Optional[str]:
        """Get source line for a position."""
        if position and position.line in self.source_lines:
            return self.source_lines[position.line]
        return None


# Global stack trace collector instance
_stack_collector = StackTraceCollector()


def get_stack_collector() -> StackTraceCollector:
    """Get the global stack trace collector."""
    return _stack_collector


def push_execution_frame(function_name: str, position: Optional[SourcePosition] = None, arguments: Optional[Dict[str, Any]] = None):
    """Convenience function to push an execution frame."""
    _stack_collector.push_frame(function_name, position, arguments)


def pop_execution_frame():
    """Convenience function to pop an execution frame."""
    _stack_collector.pop_frame()


def update_frame_variables(variables: Dict[str, Any]):
    """Convenience function to update current frame variables."""
    _stack_collector.update_local_variables(variables)


def create_enhanced_error_trace(error_message: str, error_type: str, source_name: str = "<input>") -> EnhancedStackTrace:
    """Convenience function to create enhanced error trace."""
    return _stack_collector.create_enhanced_trace(error_message, error_type, source_name)


def create_error_result_tuple(error_message: str, include_stack_trace: bool = True):
    """Create an error result tuple in the form [:error, message] with optional stack trace.

    This supports the error-as-data pattern by providing structured error results
    that can be used in pattern matching and normal control flow.

    Args:
        error_message: The error message
        include_stack_trace: Whether to include enhanced stack trace information

    Returns:
        A list containing [:error, enhanced_message] where enhanced_message includes
        stack trace information if requested.
    """
    from .values import SymbolValue, StringValue
    from .graph_values import ListValue

    if include_stack_trace:
        stack_trace = _stack_collector.create_enhanced_trace(error_message, "RuntimeError")
        enhanced_message = stack_trace.format_compact_trace()
    else:
        enhanced_message = error_message

    error_symbol = SymbolValue("error")
    message_value = StringValue(enhanced_message)

    return ListValue([error_symbol, message_value])


def create_success_result_tuple(value):
    """Create a success result tuple in the form [:ok, value].

    This supports the error-as-data pattern by providing structured success results.

    Args:
        value: The success value (should be a GlangValue)

    Returns:
        A list containing [:ok, value]
    """
    from .values import SymbolValue
    from .graph_values import ListValue

    success_symbol = SymbolValue("ok")
    return ListValue([success_symbol, value])
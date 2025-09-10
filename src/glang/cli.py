"""
Command-line interface for Glang.
"""

import sys
import os
import argparse
from pathlib import Path
from glang import __version__
from glang.repl import REPL
from glang.execution.pipeline import ExecutionSession
from glang.files import FileManager


def main() -> None:
    """Main CLI entry point."""
    parser = argparse.ArgumentParser(
        prog="glang",
        description="Glang - A prototype programming language with graphs as first-class objects"
    )
    parser.add_argument(
        "--version",
        action="version",
        version=f"Glang {__version__}"
    )
    parser.add_argument(
        "file",
        nargs="?",
        help="Glang source file to execute (.gr file)"
    )
    parser.add_argument(
        "--args",
        nargs="*",
        default=[],
        help="Arguments to pass to the Glang program"
    )
    parser.add_argument(
        "--verbose", "-v",
        action="store_true",
        help="Enable verbose output"
    )
    parser.add_argument(
        "--check-syntax", "-c",
        action="store_true", 
        help="Check syntax without executing"
    )
    
    args = parser.parse_args()
    
    if args.file:
        exit_code = execute_file(args.file, args.args, args.verbose, args.check_syntax)
        sys.exit(exit_code)
    else:
        # Start interactive REPL
        repl = REPL()
        repl.run()


def execute_file(file_path: str, program_args: list, verbose: bool, check_syntax: bool) -> int:
    """Execute a Glang source file.
    
    Args:
        file_path: Path to the .gr file
        program_args: Arguments to pass to the program
        verbose: Enable verbose output
        check_syntax: Only check syntax, don't execute
        
    Returns:
        Exit code (0 for success, non-zero for error)
    """
    # Validate file exists and has .gr extension
    path = Path(file_path)
    
    if not path.exists():
        print(f"Error: File '{file_path}' not found", file=sys.stderr)
        return 1
    
    if not path.is_file():
        print(f"Error: '{file_path}' is not a file", file=sys.stderr)
        return 1
    
    if path.suffix != '.gr':
        print(f"Warning: File '{file_path}' does not have .gr extension", file=sys.stderr)
    
    try:
        # Read the file content
        with open(path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Handle shebang lines
        if content.startswith('#!'):
            lines = content.split('\n')
            content = '\n'.join(lines[1:])  # Skip shebang line
        
        if verbose:
            print(f"Executing: {path.absolute()}")
            if program_args:
                print(f"Arguments: {program_args}")
        
        # Create execution session
        file_manager = FileManager()
        session = ExecutionSession(file_manager)
        
        # Set up program arguments (make them available as 'args' variable)
        from glang.execution.values import ListValue, StringValue
        args_list = ListValue([StringValue(arg, None) for arg in program_args], None, None)
        session.execution_context.set_variable('args', args_list)
        
        # Also add to symbol table so semantic analysis knows about it
        from glang.semantic.symbol_table import Symbol
        symbol_table = session.execution_context.symbol_table
        args_symbol = Symbol('args', 'list', None, None)
        symbol_table.declare_symbol(args_symbol)
        
        if check_syntax:
            # Only parse and analyze, don't execute
            from glang.parser.ast_parser import ASTParser
            from glang.semantic.analyzer import SemanticAnalyzer
            
            parser = ASTParser()
            analyzer = SemanticAnalyzer()
            
            # Parse each statement
            statements = content.strip().split('\n')
            for i, statement in enumerate(statements, 1):
                if not statement.strip():
                    continue
                    
                try:
                    ast = parser.parse(statement.strip())
                    result = analyzer.analyze(ast, clear_state=False)
                    
                    if not result.success:
                        print(f"Syntax error on line {i}: {result.errors[0]}", file=sys.stderr)
                        return 1
                        
                except Exception as e:
                    print(f"Parse error on line {i}: {e}", file=sys.stderr)
                    return 1
            
            if verbose:
                print("Syntax check passed")
            return 0
        
        # Execute the program statement by statement (handling multiline constructs)
        statements = parse_multiline_statements(content)
        last_result = None
        
        for i, statement in enumerate(statements, 1):
            if verbose:
                lines = statement.split('\n')
                if len(lines) == 1:
                    print(f"[{i}] {statement}")
                else:
                    print(f"[{i}] {lines[0]}")
                    for line in lines[1:]:
                        print(f"     {line}")
                
            result = session.execute_statement(statement)
            
            if not result.success:
                print(f"Runtime error in statement {i}: {result.error}", file=sys.stderr)
                return 1
            
            last_result = result
            
            # Show output for expressions that return values (like REPL)
            if result.value is not None and not any(statement.strip().endswith(x) for x in ['=', '}', '{']):
                if verbose:
                    print(f"    -> {result.value}")
        
        if verbose:
            print("Program completed successfully")
        
        return 0
        
    except FileNotFoundError:
        print(f"Error: Cannot read file '{file_path}'", file=sys.stderr)
        return 1
    except UnicodeDecodeError:
        print(f"Error: File '{file_path}' is not valid UTF-8", file=sys.stderr)
        return 1
    except KeyboardInterrupt:
        print("\nProgram interrupted by user", file=sys.stderr)
        return 130  # Standard exit code for SIGINT
    except Exception as e:
        print(f"Unexpected error: {e}", file=sys.stderr)
        if verbose:
            import traceback
            traceback.print_exc()
        return 1


def parse_multiline_statements(content: str) -> list:
    """Parse content into complete statements, handling multiline constructs.
    
    This function groups lines into complete statements by counting braces,
    similar to the REPL's multiline handling.
    """
    statements = []
    current_statement = []
    
    lines = content.strip().split('\n')
    
    for line in lines:
        # Skip comment-only lines and empty lines at statement boundaries
        if line.strip().startswith('#') or not line.strip():
            if current_statement:  # Add empty line to current statement
                current_statement.append(line)
            continue
        
        current_statement.append(line)
        
        # Check if current statement is complete
        combined = '\n'.join(current_statement)
        if is_statement_complete(combined):
            statements.append(combined)
            current_statement = []
    
    # Handle any remaining incomplete statement
    if current_statement:
        statements.append('\n'.join(current_statement))
    
    return [s for s in statements if s.strip()]


def is_statement_complete(statement: str) -> bool:
    """Check if a statement appears to be complete by counting braces."""
    brace_count = 0
    paren_count = 0  
    bracket_count = 0
    in_string = False
    escape_next = False
    
    i = 0
    while i < len(statement):
        char = statement[i]
        
        if escape_next:
            escape_next = False
            i += 1
            continue
        
        if char == '\\':
            escape_next = True
            i += 1
            continue
        
        if char == '"' and not escape_next:
            in_string = not in_string
            i += 1
            continue
        
        if in_string:
            i += 1
            continue
        
        # Count delimiters outside of strings
        if char == '{':
            brace_count += 1
        elif char == '}':
            brace_count -= 1
        elif char == '(':
            paren_count += 1
        elif char == ')':
            paren_count -= 1
        elif char == '[':
            bracket_count += 1
        elif char == ']':
            bracket_count -= 1
        
        i += 1
    
    # Statement is complete if all delimiters are balanced
    return brace_count == 0 and paren_count == 0 and bracket_count == 0


if __name__ == "__main__":
    main()
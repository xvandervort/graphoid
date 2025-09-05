"""
Command-line interface for Glang.
"""

import sys
import argparse
from glang import __version__
from glang.repl import REPL


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
        help="Glang source file to execute (not yet implemented)"
    )
    
    args = parser.parse_args()
    
    if args.file:
        print(f"File execution not yet implemented: {args.file}")
        sys.exit(1)
    else:
        # Start interactive REPL
        repl = REPL()
        repl.run()


if __name__ == "__main__":
    main()
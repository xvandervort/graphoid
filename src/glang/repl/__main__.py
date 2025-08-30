"""
Entry point for running the REPL as a module.
"""

from .repl import REPL


def main() -> None:
    """Main entry point for the REPL."""
    repl = REPL()
    repl.start()


if __name__ == "__main__":
    main()
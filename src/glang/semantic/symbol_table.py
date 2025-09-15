"""Symbol table implementation for glang semantic analysis."""

from dataclasses import dataclass
from typing import Dict, Optional, List
from ..ast.nodes import SourcePosition


@dataclass
class Symbol:
    """Represents a symbol (variable) in the symbol table."""
    name: str
    symbol_type: str  # 'list', 'string', 'num', 'bool'
    type_constraint: Optional[str] = None
    position: Optional[SourcePosition] = None
    
    def __str__(self) -> str:
        constraint_str = f"<{self.type_constraint}>" if self.type_constraint else ""
        pos_str = f" at {self.position}" if self.position else ""
        return f"{self.symbol_type}{constraint_str} {self.name}{pos_str}"
    
    def has_constraint(self) -> bool:
        """Check if this symbol has a type constraint."""
        return self.type_constraint is not None
    
    def matches_constraint(self, value_type: str) -> bool:
        """Check if a value type matches this symbol's constraint."""
        if not self.has_constraint():
            return True  # No constraint, anything matches
        return self.type_constraint == value_type


class SymbolTable:
    """Symbol table for tracking variables and their types with scope support."""

    def __init__(self):
        self.symbols: Dict[str, Symbol] = {}
        self._declaration_order: List[str] = []
        self._scopes: List[Dict[str, Symbol]] = [{}]  # Stack of scopes, global scope at index 0
    
    def enter_scope(self) -> None:
        """Enter a new scope."""
        self._scopes.append({})

    def exit_scope(self) -> None:
        """Exit the current scope."""
        if len(self._scopes) > 1:  # Don't remove global scope
            self._scopes.pop()

    def _current_scope(self) -> Dict[str, Symbol]:
        """Get the current scope."""
        return self._scopes[-1]

    def declare_symbol(self, symbol: Symbol) -> None:
        """Declare a new symbol in the current scope.

        Args:
            symbol: The symbol to declare

        Raises:
            ValueError: If symbol already exists in current scope
        """
        current_scope = self._current_scope()

        # Only check for redeclaration within the current scope
        if symbol.name in current_scope:
            existing = current_scope[symbol.name]
            raise ValueError(f"Symbol '{symbol.name}' already declared in current scope at {existing.position}")

        # Add to current scope
        current_scope[symbol.name] = symbol

        # Only update global symbols dict if we're in global scope OR if it's a new symbol
        # (This prevents overwrites of global variables from nested scopes)
        if len(self._scopes) == 1 or symbol.name not in self.symbols:
            self.symbols[symbol.name] = symbol
            if len(self._scopes) == 1:  # Only track declaration order for global scope
                self._declaration_order.append(symbol.name)
    
    def lookup_symbol(self, name: str) -> Optional[Symbol]:
        """Look up a symbol by name, searching from current scope to global.

        Args:
            name: The symbol name to look up

        Returns:
            The symbol if found, None otherwise
        """
        # Search from current scope back to global scope
        for scope in reversed(self._scopes):
            if name in scope:
                return scope[name]
        return None
    
    def symbol_exists(self, name: str) -> bool:
        """Check if a symbol exists in any scope.

        Args:
            name: The symbol name to check

        Returns:
            True if symbol exists, False otherwise
        """
        return self.lookup_symbol(name) is not None

    def symbol_exists_in_current_scope(self, name: str) -> bool:
        """Check if a symbol exists in the current scope only.

        Args:
            name: The symbol name to check

        Returns:
            True if symbol exists in current scope, False otherwise
        """
        return name in self._current_scope()
    
    def get_all_symbols(self) -> Dict[str, Symbol]:
        """Get all symbols in the table.
        
        Returns:
            Dictionary of all symbols
        """
        return self.symbols.copy()
    
    def get_symbols_by_type(self, symbol_type: str) -> List[Symbol]:
        """Get all symbols of a specific type.
        
        Args:
            symbol_type: The type to filter by ('list', 'string', 'num', 'bool')
            
        Returns:
            List of symbols matching the type
        """
        return [symbol for symbol in self.symbols.values() 
                if symbol.symbol_type == symbol_type]
    
    def get_symbols_with_constraint(self, constraint: str) -> List[Symbol]:
        """Get all symbols with a specific type constraint.
        
        Args:
            constraint: The constraint to filter by
            
        Returns:
            List of symbols with the specified constraint
        """
        return [symbol for symbol in self.symbols.values()
                if symbol.type_constraint == constraint]
    
    def remove_symbol(self, name: str) -> bool:
        """Remove a symbol from the table.

        Args:
            name: The symbol name to remove

        Returns:
            True if symbol was removed, False if not found
        """
        # Remove from scopes (search from current scope to global)
        removed = False
        for scope in reversed(self._scopes):
            if name in scope:
                del scope[name]
                removed = True
                break

        # Also remove from global symbols dict for backward compatibility
        if name in self.symbols:
            del self.symbols[name]
            if name in self._declaration_order:
                self._declaration_order.remove(name)
            removed = True

        return removed
    
    def clear(self) -> None:
        """Clear all symbols from the table."""
        self.symbols.clear()
        self._declaration_order.clear()
        # Reset to just the global scope
        self._scopes = [{}]
    
    def size(self) -> int:
        """Get the number of symbols in the table."""
        return len(self.symbols)
    
    def get_declaration_order(self) -> List[str]:
        """Get symbol names in declaration order.
        
        Returns:
            List of symbol names in the order they were declared
        """
        return self._declaration_order.copy()
    
    def __str__(self) -> str:
        """String representation of the symbol table."""
        if not self.symbols:
            return "SymbolTable: (empty)"
        
        lines = ["SymbolTable:"]
        for name in self._declaration_order:
            symbol = self.symbols[name]
            lines.append(f"  {symbol}")
        
        return "\n".join(lines)
    
    def __repr__(self) -> str:
        return f"SymbolTable(symbols={len(self.symbols)})"
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
    """Symbol table for tracking variables and their types."""
    
    def __init__(self):
        self.symbols: Dict[str, Symbol] = {}
        self._declaration_order: List[str] = []
    
    def declare_symbol(self, symbol: Symbol) -> None:
        """Declare a new symbol in the table.
        
        Args:
            symbol: The symbol to declare
            
        Raises:
            ValueError: If symbol already exists
        """
        if symbol.name in self.symbols:
            existing = self.symbols[symbol.name]
            raise ValueError(f"Symbol '{symbol.name}' already declared at {existing.position}")
        
        self.symbols[symbol.name] = symbol
        self._declaration_order.append(symbol.name)
    
    def lookup_symbol(self, name: str) -> Optional[Symbol]:
        """Look up a symbol by name.
        
        Args:
            name: The symbol name to look up
            
        Returns:
            The symbol if found, None otherwise
        """
        return self.symbols.get(name)
    
    def symbol_exists(self, name: str) -> bool:
        """Check if a symbol exists in the table.
        
        Args:
            name: The symbol name to check
            
        Returns:
            True if symbol exists, False otherwise
        """
        return name in self.symbols
    
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
        if name not in self.symbols:
            return False
        
        del self.symbols[name]
        if name in self._declaration_order:
            self._declaration_order.remove(name)
        return True
    
    def clear(self) -> None:
        """Clear all symbols from the table."""
        self.symbols.clear()
        self._declaration_order.clear()
    
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
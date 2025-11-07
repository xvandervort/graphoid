"""Semantic analysis pipeline for glang."""

from typing import Optional
from ..parser.ast_parser import ASTParser, ParseError
from .analyzer import SemanticAnalyzer, AnalysisResult
from .symbol_table import Symbol, SymbolTable
from .errors import SemanticError


class SemanticPipeline:
    """Complete pipeline from source code to semantically analyzed AST."""
    
    def __init__(self):
        self.parser = ASTParser()
        self.analyzer = SemanticAnalyzer()
    
    def analyze_code(self, input_str: str) -> AnalysisResult:
        """Analyze source code through the complete pipeline.
        
        Args:
            input_str: The source code to analyze
            
        Returns:
            AnalysisResult containing AST, symbol table, and any errors
        """
        # Phase 1: Parse to AST
        try:
            ast = self.parser.parse(input_str)
        except ParseError as e:
            # Convert parse error to semantic error for consistent error handling
            semantic_error = SemanticError(f"Parse error: {str(e)}")
            return AnalysisResult(
                ast=None,
                symbol_table=SymbolTable(),
                errors=[semantic_error],
                success=False
            )
        
        # Phase 2: Perform semantic analysis
        result = self.analyzer.analyze(ast)
        return result
    
    def get_symbol_table(self) -> SymbolTable:
        """Get the current symbol table from the analyzer.
        
        Returns:
            The symbol table from the most recent analysis
        """
        return self.analyzer.symbol_table
    
    def reset(self) -> None:
        """Reset the pipeline state."""
        self.analyzer.symbol_table.clear()
        self.analyzer.errors.clear()


class SemanticSession:
    """Session manager for semantic analysis with persistent symbol table."""
    
    def __init__(self):
        self.pipeline = SemanticPipeline()
        self.persistent_symbol_table = SymbolTable()
    
    def analyze_statement(self, input_str: str) -> AnalysisResult:
        """Analyze a single statement in the context of the current session.
        
        Args:
            input_str: The statement to analyze
            
        Returns:
            AnalysisResult with session state updated
        """
        # Copy current symbols to analyzer
        self._sync_symbol_table_to_analyzer()
        
        # Analyze the statement without clearing existing state
        try:
            ast = self.pipeline.parser.parse(input_str)
        except ParseError as e:
            # Convert parse error to semantic error for consistent error handling
            semantic_error = SemanticError(f"Parse error: {str(e)}")
            return AnalysisResult(
                ast=None,
                symbol_table=self.persistent_symbol_table,
                errors=[semantic_error],
                success=False
            )
        
        # Analyze with existing symbol table
        result = self.pipeline.analyzer.analyze(ast, clear_state=False)
        
        # If successful, update persistent symbol table
        if result.success:
            self._sync_symbol_table_from_analyzer(result.symbol_table)
        
        return result
    
    def get_symbol_table(self) -> SymbolTable:
        """Get the persistent symbol table for this session."""
        return self.persistent_symbol_table
    
    def clear_session(self) -> None:
        """Clear the session state."""
        self.persistent_symbol_table.clear()
        self.pipeline.reset()
    
    def _sync_symbol_table_to_analyzer(self) -> None:
        """Sync persistent symbol table to analyzer."""
        self.pipeline.analyzer.symbol_table.clear()
        
        # Copy all symbols from persistent table
        for symbol in self.persistent_symbol_table.get_all_symbols().values():
            try:
                self.pipeline.analyzer.symbol_table.declare_symbol(symbol)
            except ValueError:
                # Symbol already exists, this shouldn't happen but handle gracefully
                pass
    
    def _sync_symbol_table_from_analyzer(self, analyzer_table: SymbolTable) -> None:
        """Sync analyzer symbol table back to persistent table."""
        # Add any new symbols from the analyzer
        for symbol in analyzer_table.get_all_symbols().values():
            if not self.persistent_symbol_table.symbol_exists(symbol.name):
                try:
                    self.persistent_symbol_table.declare_symbol(symbol)
                except ValueError:
                    # Symbol conflict, skip
                    pass
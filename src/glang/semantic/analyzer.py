"""Semantic analyzer for glang AST nodes."""

from typing import List, Optional, Any
from dataclasses import dataclass

from ..ast.nodes import *
from .symbol_table import Symbol, SymbolTable
from .errors import *


@dataclass 
class AnalysisResult:
    """Result of semantic analysis."""
    ast: Optional[ASTNode]
    symbol_table: SymbolTable
    errors: List[SemanticError]
    success: bool
    
    def has_errors(self) -> bool:
        """Check if analysis found any errors."""
        return len(self.errors) > 0
    
    def add_error(self, error: SemanticError) -> None:
        """Add an error to the result."""
        self.errors.append(error)
        self.success = False


class SemanticAnalyzer(BaseASTVisitor):
    """Semantic analyzer using visitor pattern."""
    
    def __init__(self):
        self.symbol_table = SymbolTable()
        self.errors: List[SemanticError] = []
    
    def analyze(self, ast: ASTNode, clear_state: bool = True) -> AnalysisResult:
        """Perform semantic analysis on an AST node.
        
        Args:
            ast: The AST node to analyze
            clear_state: Whether to clear existing state before analysis
            
        Returns:
            AnalysisResult with symbol table and any errors found
        """
        if clear_state:
            self.errors.clear()
            self.symbol_table.clear()
        else:
            self.errors.clear()  # Always clear errors
        
        try:
            ast.accept(self)
            success = len(self.errors) == 0
            return AnalysisResult(ast, self.symbol_table, self.errors.copy(), success)
        except Exception as e:
            # Catch any unexpected errors during analysis
            error = SemanticError(f"Internal analysis error: {str(e)}")
            self.errors.append(error)
            return AnalysisResult(ast, self.symbol_table, self.errors.copy(), False)
    
    def report_error(self, error: SemanticError) -> None:
        """Report a semantic error."""
        self.errors.append(error)
    
    def infer_type_from_expression(self, expr: Expression) -> Optional[str]:
        """Infer the type of an expression for implicit variable declarations."""
        if isinstance(expr, StringLiteral):
            return 'string'
        elif isinstance(expr, NumberLiteral):
            return 'num'
        elif isinstance(expr, BooleanLiteral):
            return 'bool'
        elif isinstance(expr, ListLiteral):
            return 'list'
        elif isinstance(expr, DataNodeLiteral):
            return 'data'
        elif isinstance(expr, MapLiteral):
            return 'hash'
        elif isinstance(expr, SymbolLiteral):
            return 'symbol'
        elif isinstance(expr, VariableRef):
            # Look up the type of the referenced variable
            if self.symbol_table.symbol_exists(expr.name):
                symbol = self.symbol_table.lookup_symbol(expr.name)
                return symbol.symbol_type
        elif isinstance(expr, IndexAccess):
            # For index access, we need to determine what the indexed expression returns
            # Get the type of the target being indexed
            target_type = self.infer_type_from_expression(expr.target)
            if target_type == 'list':
                # Special case: if indexing into a literal list with a literal index
                if isinstance(expr.target, ListLiteral) and isinstance(expr.index, NumberLiteral):
                    index_val = int(expr.index.value)
                    if 0 <= index_val < len(expr.target.elements):
                        # Infer type from the specific element
                        element = expr.target.elements[index_val]
                        return self.infer_type_from_expression(element)

                # Special case: if target is a method call that returns a known list type
                if isinstance(expr.target, MethodCallExpression):
                    if expr.target.method_name == 'keys':
                        # keys() returns list<string>
                        return 'string'
                return None  # Cannot determine element type without type constraints
            elif target_type == 'string':
                # String indexing returns a string (single character)
                return 'string'
            elif target_type == 'hash':
                # Hash indexing returns data nodes
                return 'data'
            else:
                return None  # Cannot determine
        elif isinstance(expr, MethodCallExpression) or isinstance(expr, MethodCall):
            # Get the target type
            target_type = None
            if isinstance(expr.target, MethodCallExpression):
                # Recursively infer the type of the chained target
                target_type = self.infer_type_from_expression(expr.target)
                if target_type is None:
                    return None  # Can't determine
            elif isinstance(expr.target, VariableRef):
                # Look up the type of the target variable
                if self.symbol_table.symbol_exists(expr.target.name):
                    symbol = self.symbol_table.lookup_symbol(expr.target.name)
                    target_type = symbol.symbol_type
            
            # Some methods have known return types
            if expr.method_name == 'len':
                return 'num'
            elif expr.method_name == 'get_type':
                return 'string'  # get_type() always returns a string
            elif expr.method_name == 'to_string':
                return 'string'  # to_string() always returns a string
            elif expr.method_name == 'to_num':
                return 'num'  # to_num() always returns a number
            elif expr.method_name == 'to_time':
                return 'time'  # to_time() always returns a time
            elif expr.method_name in ['map', 'filter', 'select', 'reject']:
                # These methods return lists - try to infer from target type
                if isinstance(expr.target, VariableRef):
                    if self.symbol_table.symbol_exists(expr.target.name):
                        symbol = self.symbol_table.lookup_symbol(expr.target.name)
                        if symbol.symbol_type == 'list':
                            return 'list'
                return 'list'  # Default to list type
            elif expr.method_name == 'each':
                # each() returns the original target (for chaining)
                if isinstance(expr.target, VariableRef):
                    if self.symbol_table.symbol_exists(expr.target.name):
                        symbol = self.symbol_table.lookup_symbol(expr.target.name)
                        return symbol.symbol_type
                return None  # Cannot determine
            elif expr.method_name in ['node', 'pop']:
                # Hash methods that return data nodes
                return 'data'
            elif expr.method_name == 'keys':
                # keys() returns a list (of strings, but we can't express that yet)
                return 'list'
            elif expr.method_name == 'values':
                # values() returns a list (of values, type depends on hash constraint)
                return 'list'
            # Handle file handle method return types
            elif target_type == 'file':
                file_method_types = {
                    'write': 'bool',
                    'read': 'string',
                    'read_line': 'string',
                    'flush': 'bool',
                    'close': 'bool',
                    'kill': 'bool',
                    'capability_type': 'string',
                    'type': 'string',  # Universal method
                }
                if expr.method_name in file_method_types:
                    return file_method_types[expr.method_name]
            # Handle I/O module method return types
            elif (isinstance(expr.target, VariableRef) and 
                  self.symbol_table.symbol_exists(expr.target.name) and
                  self.symbol_table.lookup_symbol(expr.target.name).symbol_type == 'module'):
                # Get the module symbol to determine which module it is
                symbol = self.symbol_table.lookup_symbol(expr.target.name)
                # For module symbols, we need to check the import to get the original module name
                # Currently we use the alias name, but we should map aliases to actual module names
                module_name = symbol.name if symbol else 'unknown'
                
                # Handle common module aliases
                alias_to_module = {
                    'Time': 'time',
                    'IO': 'io', 
                    'Json': 'json',
                    'Crypto': 'crypto'
                }
                if module_name in alias_to_module:
                    module_name = alias_to_module[module_name]
                
                # Map module methods to their return types
                if module_name == 'io':
                    method_types = {
                        'print': 'void',
                        'read_file': 'string',
                        'read_lines': 'list',
                        'read_binary': 'list',  # Returns list of numbers (bytes)
                        'write_file': 'bool',
                        'write_lines': 'bool', 
                        'write_binary': 'bool', # Returns boolean success indicator
                        'append_file': 'bool',
                        'open': 'file',           # Returns file handle
                        'exists': 'bool',
                        'is_file': 'bool',
                        'is_dir': 'bool',
                        'make_dir': 'bool',
                        'remove_file': 'bool',
                        'remove_dir': 'bool',
                        'set_cwd': 'bool',
                        'get_cwd': 'string',
                        'file_size': 'num',
                        'list_dir': 'list',
                        'join_path': 'string',     # Joins paths into single string
                        'split_path': 'list',      # Splits path into [dir, filename]
                        'get_basename': 'string',  # Gets filename component
                        'get_dirname': 'string',   # Gets directory component
                        'get_extension': 'string', # Gets file extension
                        'resolve_path': 'string',  # Resolves to absolute path
                        'input': 'string'
                    }
                elif module_name == 'json':
                    method_types = {
                        'encode': 'string',
                        'encode_pretty': 'string',
                        'decode': 'any',  # JSON can decode to any type
                        'is_valid': 'bool'
                    }
                elif module_name == 'crypto':
                    method_types = {
                        # Hashing functions
                        'hash_md5': 'list',      # Returns list of bytes (16 bytes for MD5)
                        'hash_sha1': 'list',     # Returns list of bytes (20 bytes for SHA1)
                        'hash_sha256': 'list',   # Returns list of bytes (32 bytes for SHA256)
                        'hash_sha512': 'list',   # Returns list of bytes (64 bytes for SHA512)
                        
                        # Random generation
                        'random_bytes': 'list',  # Returns list of random bytes
                        
                        # Symmetric encryption (AES)
                        'aes_encrypt': 'list',   # Returns list of encrypted bytes (IV + ciphertext)
                        'aes_decrypt': 'list',   # Returns list of decrypted bytes
                        'aes_gcm_encrypt': 'list',   # Returns list of encrypted bytes (nonce + ciphertext + tag)
                        'aes_gcm_decrypt': 'list',   # Returns list of decrypted bytes
                        
                        # Message authentication
                        'hmac_sha256': 'list',   # Returns list of HMAC bytes (32 bytes)
                        
                        # Key derivation
                        'hkdf_expand': 'list',   # Returns list of derived key bytes
                        
                        # Asymmetric encryption (RSA)
                        'rsa_generate_keypair': 'hash',  # Returns hash with private/public keys
                        'rsa_encrypt': 'list',   # Returns list of encrypted bytes
                        'rsa_decrypt': 'list',   # Returns list of decrypted bytes
                        
                        # Elliptic Curve Diffie-Hellman (ECDH)
                        'ecdh_generate_keypair': 'hash',  # Returns hash with private/public/curve
                        'ecdh_compute_shared_secret': 'list',  # Returns list of shared secret bytes
                        'ecdh_public_key_from_private': 'list',  # Returns list of public key bytes
                        
                        # Format conversion
                        'to_hex': 'string',      # Converts bytes to hex string
                        'from_hex': 'list',      # Converts hex string to bytes
                        'to_base64': 'string',   # Converts bytes to base64 string
                        'from_base64': 'list'    # Converts base64 string to bytes
                    }
                elif module_name == 'time':
                    method_types = {
                        'now': 'time',
                        'today': 'time', 
                        'from_components': 'time',
                        'from_string': 'time'
                    }
                else:
                    method_types = {}
                
                # Legacy support for old io_method_types variable name
                io_method_types = method_types
                if expr.method_name in io_method_types:
                    return io_method_types[expr.method_name]
            elif expr.method_name in ['keys', 'values']:
                # Hash methods that return lists
                return 'list'
            elif expr.method_name in ['has_key', 'empty']:
                # Methods that return booleans
                return 'bool'
            elif expr.method_name in ['size', 'count', 'indexOf', 'count_values', 'min', 'max', 'sum']:
                # Methods that return numbers
                return 'num'
            elif expr.method_name in ['up', 'down', 'toUpper', 'toLower', 'trim', 'reverse', 'replace', 'join', 'chars', 'split', 'findAll']:
                # String methods that return strings or lists
                if expr.method_name in ['chars', 'split', 'findAll']:
                    return 'list'
                else:
                    return 'string'
            elif expr.method_name == 'to_string':
                # Type casting to string
                return 'string'
            elif expr.method_name in ['to_num', 'numify', 'toNum']:
                # Type casting to number
                return 'num'
            elif expr.method_name in ['to_bool', 'flip', 'toggle']:
                # Type casting to boolean or boolean operations
                return 'bool'
            elif expr.method_name in ['abs', 'sqrt', 'log', 'pow', 'rnd', 'rnd_up', 'rnd_dwn', 'to']:
                # Mathematical methods that return numbers
                return 'num'
            # For other methods, we'd need more context
        elif isinstance(expr, BinaryOperation):
            # Arithmetic operations return numbers
            if expr.operator in ['+', '-', '*', '/', '%', '//', '**']:
                return 'num'
            # Dot operators (element-wise operations) return lists
            elif expr.operator in ['+.', '-.', '*.', '/.', '%.', '//.', '**.']:
                return 'list'
            # Comparison operations return booleans
            elif expr.operator in ['<', '>', '<=', '>=', '==', '!=']:
                return 'bool'
            # Logical operations return booleans
            elif expr.operator in ['and', 'or']:
                return 'bool'
        elif isinstance(expr, UnaryOperation):
            if expr.operator == 'not':
                return 'bool'
            elif expr.operator == '-':
                return 'num'
        elif isinstance(expr, FunctionCall):
            # Function calls - for now, we can't infer return type without analyzing function body
            # Return generic type for successful inference
            return 'any'
        elif isinstance(expr, LambdaExpression):
            # Lambda expressions - treat as function type
            return 'function'
        elif isinstance(expr, IndexAccess):
            # Need to check the type of the indexed object
            if isinstance(expr.target, VariableRef):
                if self.symbol_table.symbol_exists(expr.target.name):
                    symbol = self.symbol_table.lookup_symbol(expr.target.name)
                    if symbol.symbol_type == 'list':
                        # Could be constrained type or any type
                        return symbol.type_constraint if symbol.type_constraint else None
                    elif symbol.symbol_type == 'string':
                        return 'string'  # String indexing returns string
                    elif symbol.symbol_type == 'hash':
                        return 'data'  # Hash indexing returns data node
        elif isinstance(expr, MatchExpression):
            # For match expressions, infer type from the first arm's result
            # All arms should return the same type (this could be enforced later)
            if expr.arms:
                return self.infer_type_from_expression(expr.arms[0].result)

        return None  # Cannot infer type
    
    # Statement visitors
    
    def visit_variable_declaration(self, node: VariableDeclaration) -> None:
        """Analyze variable declarations."""
        # Validate type
        valid_types = {'list', 'string', 'num', 'bool', 'data', 'hash', 'function', 'file', 'any'}
        if node.var_type not in valid_types:
            self.report_error(InvalidTypeError(node.var_type, node.position))
            return
        
        # Validate constraint if present
        if node.type_constraint:
            valid_constraints = {'string', 'num', 'bool', 'list', 'data', 'hash'}
            if node.type_constraint not in valid_constraints:
                self.report_error(InvalidConstraintError(
                    node.type_constraint, node.var_type, node.position))
                return
            
            # Only list, data, and hash types support constraints currently
            if node.var_type not in ['list', 'data', 'hash']:
                self.report_error(InvalidConstraintError(
                    node.type_constraint, node.var_type, node.position))
                return
        
        # Check if variable already exists
        if self.symbol_table.symbol_exists(node.name):
            existing = self.symbol_table.lookup_symbol(node.name)
            self.report_error(RedeclarationError(
                node.name, existing.position, node.position))
            return
        
        # Create and declare symbol
        symbol = Symbol(
            name=node.name,
            symbol_type=node.var_type,
            type_constraint=node.type_constraint,
            position=node.position
        )
        
        try:
            self.symbol_table.declare_symbol(symbol)
        except ValueError as e:
            self.report_error(SemanticError(str(e), node.position))
        
        # Analyze behaviors if present
        if node.behaviors:
            node.behaviors.accept(self)
        
        # Analyze initializer
        if node.initializer:
            node.initializer.accept(self)
            # TODO: Type check initializer against declaration
    
    def visit_method_call(self, node: MethodCall) -> None:
        """Analyze method call statements."""
        # Analyze target
        node.target.accept(self)
        
        # For variable references, validate method call if variable exists
        if isinstance(node.target, VariableRef):
            symbol = self.symbol_table.lookup_symbol(node.target.name)
            if symbol:  # Only validate if symbol exists (undefined already reported by target visitor)
                # Check if method is valid for the target type
                self._validate_method_call(node.method_name, symbol.symbol_type, node.position)
        
        # Analyze arguments
        for arg in node.arguments:
            arg.accept(self)
    
    def visit_assignment(self, node: Assignment) -> None:
        """Analyze simple assignment statements."""
        
        # Special handling for variable assignments to support type inference
        if isinstance(node.target, VariableRef):
            var_name = node.target.name
            symbol = self.symbol_table.lookup_symbol(var_name)
            
            if not symbol:
                # NEW: Type inference - treat as implicit variable declaration
                # Infer type from the value expression
                inferred_type = self.infer_type_from_expression(node.value)
                
                if inferred_type is None:
                    # If we can't infer the type, use 'any' as a fallback
                    inferred_type = 'any'
                
                # Create a symbol for the new variable with inferred type
                inferred_symbol = Symbol(var_name, inferred_type, None, node.target.position)
                self.symbol_table.declare_symbol(inferred_symbol)
                
                # Now analyze the value
                node.value.accept(self)
            else:
                # Variable exists - analyze normally
                node.target.accept(self)
                node.value.accept(self)
        else:
            # Non-variable assignment (index assignment, etc.) - analyze normally
            node.target.accept(self)
            node.value.accept(self)
    
    def visit_index_assignment(self, node: IndexAssignment) -> None:
        """Analyze index assignments."""
        # Analyze target
        node.target.accept(self)
        
        # Analyze value
        node.value.accept(self)
        
        # TODO: Type check assignment
    
    def visit_slice_assignment(self, node: SliceAssignment) -> None:
        """Analyze slice assignments."""
        # Analyze target
        node.target.accept(self)
        
        # Analyze value
        node.value.accept(self)
        
        # TODO: Type check assignment
    
    def visit_expression_statement(self, node: ExpressionStatement) -> None:
        """Analyze expression statements."""
        node.expression.accept(self)
    
    def visit_import_statement(self, node: ImportStatement) -> None:
        """Analyze import statements."""
        # For import statements, register the module as a variable
        # The actual importing is handled at execution time
        if not isinstance(node.filename, str):
            self.errors.append(SemanticError(
                f"Import statement filename must be a string, got {type(node.filename).__name__}",
                node.position
            ))
            return
        
        # Determine the module name (use alias if provided, otherwise extract from filename)
        if node.alias:
            module_name = node.alias
        else:
            # Extract module name from filename (e.g., "io" from "io" or "io.gr")
            module_name = node.filename.replace('.gr', '') if node.filename.endswith('.gr') else node.filename
        
        # Register the module as a variable with type 'module'
        symbol = Symbol(module_name, 'module', position=node.position)
        # Use declare_symbol but check if it already exists first
        if not self.symbol_table.symbol_exists(module_name):
            self.symbol_table.declare_symbol(symbol)
    
    def visit_module_declaration(self, node: ModuleDeclaration) -> None:
        """Analyze module declarations."""
        # Module declarations just declare the module name
        # No symbols to add to the table
        pass
    
    def visit_alias_declaration(self, node: AliasDeclaration) -> None:
        """Analyze alias declarations."""
        # Alias declarations just declare the module alias
        # No symbols to add to the table
        pass
    
    def visit_load_statement(self, node: LoadStatement) -> None:
        """Analyze load statements."""
        # For load statements, we just need to validate the filename is a string
        # The actual file loading is handled at execution time
        if not isinstance(node.filename, str):
            self.errors.append(SemanticError(
                f"Load statement filename must be a string, got {type(node.filename).__name__}",
                node.position
            ))
    
    def visit_print_statement(self, node: 'PrintStatement') -> None:
        """Analyze print statements."""
        from ..ast.nodes import PrintStatement
        
        # Analyze all arguments to ensure they're valid expressions
        for arg in node.arguments:
            arg.accept(self)
    
    def visit_print_expression(self, node: 'PrintExpression') -> None:
        """Analyze print expressions."""
        from ..ast.nodes import PrintExpression
        
        # Analyze all arguments to ensure they're valid expressions
        for arg in node.arguments:
            arg.accept(self)
    
    
    # Expression visitors
    
    def visit_variable_ref(self, node: VariableRef) -> None:
        """Analyze variable references."""
        symbol = self.symbol_table.lookup_symbol(node.name)
        if not symbol:
            self.report_error(UndefinedVariableError(node.name, node.position))
    
    def visit_string_literal(self, node: StringLiteral) -> None:
        """Analyze string literals (no checks needed)."""
        pass
    
    def visit_number_literal(self, node: NumberLiteral) -> None:
        """Analyze number literals (no checks needed)."""
        pass
    
    def visit_boolean_literal(self, node: BooleanLiteral) -> None:
        """Analyze boolean literals (no checks needed)."""
        pass
    
    def visit_list_literal(self, node: ListLiteral) -> None:
        """Analyze list literals."""
        # Analyze all elements
        for element in node.elements:
            element.accept(self)
        
        # TODO: Check type consistency if this is for a constrained list
    
    def visit_data_node_literal(self, node: DataNodeLiteral) -> None:
        """Analyze data node literals."""
        # Analyze the value expression
        node.value.accept(self)
        # Key is a string literal, no need to validate further
    
    def visit_index_access(self, node: IndexAccess) -> None:
        """Analyze index access expressions."""
        # Analyze target
        node.target.accept(self)
        
        # Analyze indices
        for index in node.indices:
            index.accept(self)
        
        # Check if target is indexable
        if isinstance(node.target, VariableRef):
            symbol = self.symbol_table.lookup_symbol(node.target.name)
            if symbol and symbol.symbol_type not in {'list', 'string', 'hash', 'any'}:
                self.report_error(InvalidMethodCallError(
                    "index access", symbol.symbol_type,
                    "Type is not indexable", node.position))
    
    def visit_slice_access(self, node: SliceAccess) -> None:
        """Analyze slice access expressions."""
        # Analyze target  
        node.target.accept(self)
        
        # Analyze slice components
        if node.start:
            node.start.accept(self)
        if node.stop:
            node.stop.accept(self)
        if node.step:
            node.step.accept(self)
        
        # Check if target is sliceable
        if isinstance(node.target, VariableRef):
            symbol = self.symbol_table.lookup_symbol(node.target.name)
            if symbol and symbol.symbol_type not in {'list', 'string'}:
                self.report_error(InvalidMethodCallError(
                    "slice access", symbol.symbol_type,
                    "Type is not sliceable", node.position))
    
    def visit_method_call_expression(self, node: MethodCallExpression) -> None:
        """Analyze method call expressions."""
        # Check if this might be module-qualified access (e.g., math.pi with no arguments)
        from ..ast.nodes import VariableRef
        is_module_qualified = (isinstance(node.target, VariableRef) and 
                             len(node.arguments) == 0)
        
        if is_module_qualified:
            # For potential module access, we don't analyze the target as a variable
            # The execution system will handle module resolution
            pass
        else:
            # Analyze target for regular method calls
            node.target.accept(self)
            
            # Check method validity
            if isinstance(node.target, VariableRef):
                symbol = self.symbol_table.lookup_symbol(node.target.name)
                if symbol:
                    self._validate_method_call(node.method_name, symbol.symbol_type, node.position)
        
        # Always analyze arguments
        for arg in node.arguments:
            arg.accept(self)
    
    def visit_binary_operation(self, node: BinaryOperation) -> None:
        """Analyze binary operations."""
        # Analyze both operands
        node.left.accept(self)
        node.right.accept(self)
        
        # TODO: Add type checking for binary operations
        # For now, we just validate that operands are analyzed
    
    def visit_unary_operation(self, node: UnaryOperation) -> None:
        """Analyze unary operations."""
        # Analyze operand
        node.operand.accept(self)
        
        # TODO: Add type checking for unary operations
        # For now, we just validate that operand is analyzed
    
    # Helper methods
    
    def _validate_method_call(self, method_name: str, target_type: str,
                            position: Optional[SourcePosition]) -> None:
        """Validate that a method call is valid for the target type."""
        # Special handling for 'any' type - skip validation, allow runtime to handle it
        if target_type == 'any':
            # Type 'any' can call any method - will be validated at runtime
            return

        # Define valid methods for each type
        # Universal reflection methods available on all types
        universal_methods = {'type', 'methods', 'can', 'inspect', 'size'}

        # Behavior management methods (available on list and hash types)
        behavior_methods = {'add_rule', 'remove_rule', 'has_rule', 'get_rules', 'clear_rules'}

        valid_methods = {
            'list': {
                'append', 'prepend', 'insert', 'remove', 'pop', 'clear', 'reverse',
                'size', 'empty', 'constraint', 'validate_constraint', 'type_summary',
                'types', 'coerce_to_constraint', 'indexOf', 'count', 'min', 'max', 'sum', 'sort',
                'map', 'filter', 'each', 'select', 'reject',
                'to_string', 'to_bool',
                'add_edge', 'get_connected_to', 'to_graph', 'get_edges', 'get_edge_count', 'can_add_edge',
                'get_active_rules', 'get_rule_status', 'disable_rule', 'enable_rule',
                'get_graph_summary', 'visualize_structure',
                'set_names', 'get_names', 'has_names', 'get_name', 'set_name', 'metadata'
            } | universal_methods | behavior_methods,
            'string': {
                'size', 'empty', 'upper', 'lower', 'split', 'split_on_any', 'trim', 'join',
                'matches', 'replace', 'find_all', 'findAll',
                'length', 'contains', 'extract', 
                'count', 'count_chars', 'find_first', 'find_first_char',
                'is_email', 'is_number', 'is_url',
                'up', 'toUpper', 'down', 'toLower',
                'reverse', 'unique', 'chars',
                'starts_with', 'ends_with',
                'to_string', 'to_num', 'to_bool', 'to_time'
            } | universal_methods,
            'num': {'abs', 'round', 'to', 'sqrt', 'log', 'pow', 'rnd', 'rnd_up', 'rnd_dwn', 'to_string', 'to_num', 'to_bool', 'to_time'} | universal_methods,
            'bool': {'flip', 'toggle', 'numify', 'toNum', 'to_string', 'to_num', 'to_bool'} | universal_methods,
            'data': {'key', 'value'} | universal_methods,
            'hash': {
                'node', 'set', 'has_key', 'count_values', 'keys', 'values', 'remove', 'empty', 'merge', 'push', 'pop',
                'to_string', 'to_bool', 'can_accept',
                'add_value_edge', 'get_connected_keys', 'get_edges', 'get_edge_count', 'can_add_edge',
                'get_active_rules', 'get_rule_status', 'disable_rule', 'enable_rule',
                'get_graph_summary', 'visualize_structure',
                'set_names', 'get_names', 'has_names', 'get_name', 'set_name', 'metadata'
            } | universal_methods | behavior_methods,
            'tree': {
                'insert', 'search', 'size', 'empty', 'height',
                'in_order', 'pre_order', 'post_order',
                'to_string', 'to_bool',
                'get_active_rules', 'get_rule_status', 'disable_rule', 'enable_rule',
                'get_graph_summary', 'visualize_structure'
            } | universal_methods,
            'time': {'get_type', 'to_string', 'to_num'} | universal_methods,
            'file': {'write', 'read', 'read_line', 'flush', 'close', 'kill', 'capability_type'} | universal_methods,
            'module': universal_methods.copy()  # Modules can have any method - validated at runtime
        }
        
        if target_type not in valid_methods:
            self.report_error(InvalidMethodCallError(
                method_name, target_type, f"Unknown type '{target_type}'", position))
            return
        
        # Skip method validation for modules - they have dynamic methods
        if target_type == 'module':
            return
        
        if method_name not in valid_methods[target_type]:
            available = ", ".join(sorted(valid_methods[target_type]))
            reason = f"Available methods: {available}" if available else "No methods available"
            self.report_error(InvalidMethodCallError(
                method_name, target_type, reason, position))
    
    # Control flow visit methods
    
    def visit_if_statement(self, node: IfStatement) -> None:
        """Visit if statement node."""
        # Check condition expression
        node.condition.accept(self)
        
        # Visit then block
        node.then_block.accept(self)
        
        # Visit else block if present
        if node.else_block:
            node.else_block.accept(self)
    
    def visit_while_statement(self, node: WhileStatement) -> None:
        """Visit while statement node."""
        # Check condition expression
        node.condition.accept(self)
        
        # Visit body block
        node.body.accept(self)
    
    def visit_for_in_statement(self, node: ForInStatement) -> None:
        """Visit for-in statement node."""
        # Check iterable expression
        node.iterable.accept(self)

        # Enter new scope for loop body and loop variable
        self.symbol_table.enter_scope()

        try:
            # Create a symbol for the loop variable in the new scope
            loop_var_symbol = Symbol(node.variable, "any", position=node.position)
            self.symbol_table.declare_symbol(loop_var_symbol)

            # Visit body block in the new scope
            node.body.accept(self)
        finally:
            # Always exit scope, even if there's an error
            self.symbol_table.exit_scope()
    
    def visit_precision_block(self, node) -> None:
        """Visit precision block node."""
        # Check precision value expression
        node.precision_value.accept(self)
        
        # Visit body block in new scope (precision context)
        node.body.accept(self)
    
    def visit_break_statement(self, node: BreakStatement) -> None:
        """Visit break statement node."""
        # TODO: Validate that break is inside a loop
        # For now, just pass - the executor will handle loop context
        pass
    
    def visit_continue_statement(self, node: ContinueStatement) -> None:
        """Visit continue statement node."""
        # TODO: Validate that continue is inside a loop
        # For now, just pass - the executor will handle loop context
        pass
    
    def visit_block(self, node: Block) -> None:
        """Visit block node."""
        # Visit all statements in the block
        for statement in node.statements:
            statement.accept(self)
    
    # Function-related visitor methods
    
    def visit_function_declaration(self, node: FunctionDeclaration) -> None:
        """Analyze function declarations."""
        # Check if function name already exists
        if self.symbol_table.symbol_exists(node.name):
            existing = self.symbol_table.lookup_symbol(node.name)
            self.report_error(RedeclarationError(
                node.name, existing.position, node.position))
            return
        
        # Create function symbol
        symbol = Symbol(
            name=node.name,
            symbol_type='function',
            type_constraint=None,
            position=node.position
        )
        
        try:
            self.symbol_table.declare_symbol(symbol)
        except ValueError as e:
            self.report_error(SemanticError(str(e), node.position))
        
        # TODO: Function body analysis should be deferred until function is called
        # or we implement proper scoping with parameter declarations
        # For now, skip analyzing the function body during declaration
        # The parameters are only valid within the function's execution context
        pass
    
    def visit_function_call(self, node: FunctionCall) -> None:
        """Analyze function calls."""
        # Check that function exists
        symbol = self.symbol_table.lookup_symbol(node.name)
        if not symbol:
            self.report_error(UndefinedVariableError(node.name, node.position))
        elif symbol.symbol_type != 'function':
            self.report_error(SemanticError(
                f"'{node.name}' is not a function (it's a {symbol.symbol_type})",
                node.position))
        
        # Analyze arguments
        for arg in node.arguments:
            arg.accept(self)
    
    def visit_return_statement(self, node: ReturnStatement) -> None:
        """Analyze return statements."""
        # TODO: Check that return is inside a function
        # For now, just analyze the value if present
        if node.value:
            node.value.accept(self)
    
    def visit_lambda_expression(self, node: LambdaExpression) -> None:
        """Analyze lambda expressions."""
        # TODO: Lambda body analysis should be deferred until lambda is called
        # or we implement proper scoping with parameter declarations
        # For now, skip analyzing the lambda body during declaration
        # The parameters are only valid within the lambda's execution context
        pass
    
    def visit_behavior_call(self, node) -> None:
        """Analyze behavior call expressions."""
        # Import here to avoid circular dependency
        from glang.behaviors import registry
        
        # Check if behavior exists
        if not registry.get(node.name):
            self.errors.append(SemanticError(
                f"Unknown behavior '{node.name}'",
                node.position))
        
        # Analyze arguments
        for arg in node.arguments:
            arg.accept(self)
    
    def visit_behavior_list(self, node) -> None:
        """Analyze behavior list expressions."""
        for behavior in node.behaviors:
            if isinstance(behavior, str):
                # Import here to avoid circular dependency
                from glang.behaviors import registry
                if not registry.get(behavior):
                    self.errors.append(SemanticError(
                        f"Unknown behavior '{behavior}'",
                        node.position))
            else:
                # It's a BehaviorCall
                behavior.accept(self)

    def visit_match_expression(self, node) -> None:
        """Analyze match expressions."""
        # Analyze the expression being matched
        node.expr.accept(self)

        # Analyze each match arm in its own scope
        for arm in node.arms:
            # Enter new scope for this match arm
            self.symbol_table.enter_scope()

            try:
                # Analyze pattern and register pattern variables in the new scope
                pattern_vars, _ = self.analyze_pattern_bindings(arm.pattern)

                # Analyze result expression with pattern variables in scope
                arm.result.accept(self)
            finally:
                # Exit the scope, automatically cleaning up pattern variables
                self.symbol_table.exit_scope()

    def visit_symbol_literal(self, node) -> None:
        """Analyze symbol literals (like :ok, :error)."""
        # Symbol literals need no special analysis
        pass

    def analyze_pattern(self, pattern) -> None:
        """Analyze pattern nodes."""
        from ..ast.nodes import ListPattern, VariablePattern, LiteralPattern, WildcardPattern

        if isinstance(pattern, ListPattern):
            # Analyze each element pattern
            for element in pattern.elements:
                self.analyze_pattern(element)
        elif isinstance(pattern, VariablePattern):
            # Variables in patterns are bindings, not references
            # No need to check if they exist - they're being created
            pass
        elif isinstance(pattern, (LiteralPattern, WildcardPattern)):
            # Literals and wildcards need no analysis
            pass

    def analyze_pattern_bindings(self, pattern) -> tuple[List[str], dict]:
        """Analyze pattern bindings and register variables.

        Returns:
            Tuple of (pattern_vars, shadowed_symbols) where:
            - pattern_vars: List of variable names that were registered
            - shadowed_symbols: Dict mapping variable names to their original Symbol objects (unused with scoping)
        """
        from typing import List
        from ..ast.nodes import ListPattern, VariablePattern, LiteralPattern, WildcardPattern
        from .symbol_table import Symbol

        pattern_vars = []
        shadowed_symbols = {}  # No longer used with scoping, but kept for API compatibility

        if isinstance(pattern, ListPattern):
            # Register bindings for each element pattern
            for element in pattern.elements:
                element_vars, element_shadowed = self.analyze_pattern_bindings(element)
                pattern_vars.extend(element_vars)
                shadowed_symbols.update(element_shadowed)
        elif isinstance(pattern, VariablePattern):
            # Register the pattern variable as a symbol with 'any' type
            # We use 'any' since we can't determine the exact type at semantic analysis time

            # With scoping system, shadowing is handled automatically
            # Just declare the symbol in the current scope
            symbol = Symbol(pattern.name, 'any', None, pattern.position)
            self.symbol_table.declare_symbol(symbol)
            pattern_vars.append(pattern.name)
        elif isinstance(pattern, (LiteralPattern, WildcardPattern)):
            # Literals and wildcards don't create bindings
            pass

        return pattern_vars, shadowed_symbols
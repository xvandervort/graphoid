"""AST parser for glang - builds properly typed Abstract Syntax Trees."""

from typing import List, Optional, Union
from ..lexer.tokenizer import Token, TokenType, Tokenizer
from ..ast.nodes import *
from ..language import KEYWORD_REGISTRY, get_parser_method_name, KeywordCategory

class ParseError(Exception):
    """Error during AST parsing."""
    def __init__(self, message: str, token: Optional[Token] = None):
        self.message = message
        self.token = token
        if token:
            super().__init__(f"{message} at line {token.line}, column {token.column}")
        else:
            super().__init__(message)

class ASTParser:
    """
    Recursive descent parser that builds a properly typed AST.
    
    This parser creates AST nodes where each argument and expression 
    is properly typed, eliminating the need for runtime string parsing.
    """
    
    def __init__(self):
        self.tokenizer = Tokenizer()
        self.tokens: List[Token] = []
        self.current = 0
    
    def parse(self, input_str: str) -> Statement:
        """
        Parse input string into AST.
        
        Args:
            input_str: The glang code to parse
            
        Returns:
            A Statement AST node
            
        Raises:
            ParseError: If the input has syntax errors
        """
        self.tokens = self.tokenizer.tokenize(input_str)
        self.current = 0
        
        # Skip leading newlines
        while self.match(TokenType.NEWLINE):
            pass
            
        if self.is_at_end():
            # Return NoOp for empty input (e.g., comment-only lines)
            return NoOp(SourcePosition(1, 1))
        
        return self.parse_statement()
    
    def parse_statement(self) -> Statement:
        """Parse a statement."""
        
        # Registry-driven statement keyword dispatch!
        current_token = self.peek()
        if current_token and current_token.type != TokenType.EOF:
            # Check if current token corresponds to a statement keyword
            statement_keywords = KEYWORD_REGISTRY.get_keywords_by_category(KeywordCategory.STATEMENT)
            for keyword_name, definition in statement_keywords.items():
                token_type_name = definition.get_token_type_name()
                if hasattr(TokenType, token_type_name):
                    expected_token_type = getattr(TokenType, token_type_name)
                    if self.check(expected_token_type):
                        # Get the parser method name and call it dynamically
                        parser_method_name = definition.parser_method
                        parser_method = getattr(self, parser_method_name, None)
                        if parser_method:
                            return parser_method()
                        else:
                            raise ParseError(f"Parser method '{parser_method_name}' not implemented for keyword '{keyword_name}'", current_token)
        
        # Legacy slash-prefixed import for compatibility
        if self.check(TokenType.SLASH):
            next_token = self.tokens[self.current + 1] if self.current + 1 < len(self.tokens) else None
            if next_token and next_token.type == TokenType.IMPORT:
                return self.parse_import_statement()
        
        # Explicit variable declaration: type name = expr
        if self.check_variable_declaration():
            return self.parse_variable_declaration()
        
        # Try to parse as expression (could be method call, variable access, etc.)
        expr = self.parse_expression()
        
        # Check if it's an assignment
        if isinstance(expr, IndexAccess) and self.match(TokenType.ASSIGN):
            value = self.parse_expression()
            return IndexAssignment(expr, value)
        
        if isinstance(expr, SliceAccess) and self.match(TokenType.ASSIGN):
            value = self.parse_expression()
            return SliceAssignment(expr, value)
        
        # Simple variable assignment: variable = value
        if isinstance(expr, VariableRef) and self.match(TokenType.ASSIGN):
            value = self.parse_expression()
            return Assignment(expr, value, SourcePosition(self.previous().line, self.previous().column))
        
        # Method call assignment: obj.method = value (will be validated at semantic/execution phase)
        if isinstance(expr, (MethodCall, MethodCallExpression)) and self.match(TokenType.ASSIGN):
            value = self.parse_expression()
            return Assignment(expr, value, SourcePosition(self.previous().line, self.previous().column))
        
        # Check for malformed variable declaration (type keyword followed by = without variable name)  
        if isinstance(expr, VariableRef) and self.is_type_keyword_name(expr.name) and self.check(TokenType.ASSIGN):
            raise ParseError(f"Missing variable name after type '{expr.name}'", self.peek())
        
        # Check for malformed variable declaration (type followed by identifier but no equals)
        if isinstance(expr, VariableRef) and self.is_type_keyword_name(expr.name) and self.check(TokenType.IDENTIFIER):
            raise ParseError(f"Missing '=' after variable name in declaration", self.peek())
        
        # Check for invalid type in declaration pattern (identifier identifier =)
        if isinstance(expr, VariableRef) and not self.is_type_keyword_name(expr.name) and self.check(TokenType.IDENTIFIER):
            # Look ahead to see if there's an equals after the second identifier
            saved_pos = self.current
            try:
                self.advance()  # consume the identifier
                if self.check(TokenType.ASSIGN):
                    raise ParseError(f"Invalid type '{expr.name}' in variable declaration", expr.position)
            finally:
                self.current = saved_pos
        
        # Check if it's a method call (convert expression to statement)
        if isinstance(expr, MethodCallExpression):
            return MethodCall(expr.target, expr.method_name, expr.arguments)
        
        # Otherwise it's an expression statement
        return ExpressionStatement(expr)
    
    def check_variable_declaration(self) -> bool:
        """Check if current tokens form a variable declaration."""
        if not self.check_type_keyword():
            return False
            
        # Look ahead for: type [constraint] identifier =
        saved_pos = self.current
        
        try:
            self.advance()  # consume type
            
            # Optional type constraint: <type>
            if self.match(TokenType.LESS):
                if not (self.check_type_keyword() or self.check(TokenType.IDENTIFIER)):
                    return False
                self.advance()  # consume constraint type
                if not self.match(TokenType.GREATER):
                    return False
            
            # identifier = (allow keywords as variable names)
            if not (self.check(TokenType.IDENTIFIER) or self.check_type_keyword()):
                return False
            self.advance()
            
            return self.check(TokenType.ASSIGN)
            
        finally:
            self.current = saved_pos
    
    def parse_variable_declaration(self) -> VariableDeclaration:
        """Parse variable declaration: type [<constraint>] name = expr"""
        
        # Parse type
        type_token = self.consume_type_keyword("Expected variable type")
        var_type = type_token.value
        pos = SourcePosition(type_token.line, type_token.column)
        
        # Optional type constraint
        type_constraint = None
        if self.match(TokenType.LESS):
            constraint_token = self.advance()
            if constraint_token.type not in [TokenType.STRING, TokenType.NUM, 
                                           TokenType.BOOL, TokenType.LIST]:
                if constraint_token.type != TokenType.IDENTIFIER:
                    raise ParseError(f"Invalid type constraint '{constraint_token.value}'", 
                                   constraint_token)
            type_constraint = constraint_token.value
            self.consume(TokenType.GREATER, "Expected '>' after type constraint")
        
        # Variable name (allow keywords as variable names)
        if self.check(TokenType.IDENTIFIER) or self.check_type_keyword():
            name_token = self.advance()
            name = name_token.value
        else:
            raise ParseError("Expected variable name", self.peek())
        
        # Equals
        self.consume(TokenType.ASSIGN, "Expected '=' after variable name")
        
        # Initializer expression
        initializer = self.parse_expression()
        
        return VariableDeclaration(
            var_type=var_type,
            name=name, 
            initializer=initializer,
            type_constraint=type_constraint,
            position=pos
        )
    
    def parse_import_statement(self) -> ImportStatement:
        """Parse import statement: /import "filename.gr" as module_name"""
        
        # Parse '/' prefix
        slash_token = self.consume(TokenType.SLASH, "Expected '/'")
        pos = SourcePosition(slash_token.line, slash_token.column)
        
        # Parse 'import' keyword
        import_token = self.consume(TokenType.IMPORT, "Expected 'import' after '/'")
        
        # Parse filename (must be a string literal)
        filename_token = self.consume(TokenType.STRING_LITERAL, "Expected filename string after 'import'")
        filename = self.process_string_literal(filename_token.value)
        
        # Check for optional 'as' alias
        alias = None
        if self.check(TokenType.IDENTIFIER) and self.peek().value == "as":
            self.advance()  # consume 'as'
            alias_token = self.consume(TokenType.IDENTIFIER, "Expected module name after 'as'")
            alias = alias_token.value
        
        return ImportStatement(filename=filename, alias=alias, position=pos)
    
    def parse_import_statement_without_slash(self) -> ImportStatement:
        """Parse import statement: import "filename.gr" as module_name"""
        
        # Parse 'import' keyword
        import_token = self.consume(TokenType.IMPORT, "Expected 'import'")
        pos = SourcePosition(import_token.line, import_token.column)
        
        # Parse filename (must be a string literal)
        filename_token = self.consume(TokenType.STRING_LITERAL, "Expected filename string after 'import'")
        filename = self.process_string_literal(filename_token.value)
        
        # Check for optional 'as' alias
        alias = None
        if self.check(TokenType.IDENTIFIER) and self.peek().value == "as":
            self.advance()  # consume 'as'
            alias_token = self.consume(TokenType.IDENTIFIER, "Expected module name after 'as'")
            alias = alias_token.value
            
            if not alias.replace('_', '').isalnum() or alias[0].isdigit():
                raise ParseError(f"Invalid module name: {alias}. Must be a valid identifier", alias_token)
        
        return ImportStatement(filename=filename, alias=alias, position=pos)
    
    def parse_module_declaration(self) -> ModuleDeclaration:
        """Parse module declaration: module module_name"""
        
        # Parse 'module' keyword
        module_token = self.consume(TokenType.MODULE, "Expected 'module'")
        pos = SourcePosition(module_token.line, module_token.column)
        
        # Parse module name (must be an identifier)
        name_token = self.consume(TokenType.IDENTIFIER, "Expected module name after 'module'")
        module_name = name_token.value
        
        return ModuleDeclaration(name=module_name, position=pos)
    
    def parse_alias_declaration(self) -> AliasDeclaration:
        """Parse alias declaration: alias short_name"""
        
        # Parse 'alias' keyword
        alias_token = self.consume(TokenType.ALIAS, "Expected 'alias'")
        pos = SourcePosition(alias_token.line, alias_token.column)
        
        # Parse alias name (must be an identifier)
        name_token = self.consume(TokenType.IDENTIFIER, "Expected alias name after 'alias'")
        alias_name = name_token.value
        
        return AliasDeclaration(name=alias_name, position=pos)
    
    def parse_load_statement(self) -> LoadStatement:
        """Parse load statement: load \"filename.gr\" """
        
        # Parse 'load' keyword
        load_token = self.consume(TokenType.LOAD, "Expected 'load'")
        pos = SourcePosition(load_token.line, load_token.column)
        
        # Parse filename (must be a string literal)
        filename_token = self.consume(TokenType.STRING_LITERAL, "Expected filename string after 'load'")
        filename = self.process_string_literal(filename_token.value)
        
        return LoadStatement(filename=filename, position=pos)
    
    def parse_if_statement(self) -> IfStatement:
        """Parse if statement: if condition { statements } [else { statements }]"""
        # Parse 'if' keyword
        if_token = self.consume(TokenType.IF, "Expected 'if'")
        pos = SourcePosition(if_token.line, if_token.column)
        
        # Parse condition expression
        condition = self.parse_expression()
        
        # Parse then block
        then_block = self.parse_block()
        
        # Optional else block
        else_block = None
        if self.check(TokenType.ELSE):
            self.advance()  # consume 'else'
            else_block = self.parse_block()
        
        return IfStatement(condition=condition, then_block=then_block, else_block=else_block, position=pos)
    
    def parse_while_statement(self) -> WhileStatement:
        """Parse while statement: while condition { statements }"""
        # Parse 'while' keyword
        while_token = self.consume(TokenType.WHILE, "Expected 'while'")
        pos = SourcePosition(while_token.line, while_token.column)
        
        # Parse condition expression
        condition = self.parse_expression()
        
        # Parse body block
        body = self.parse_block()
        
        return WhileStatement(condition=condition, body=body, position=pos)
    
    def parse_for_statement(self) -> ForInStatement:
        """Parse for-in statement: for variable in iterable { statements }"""
        # Parse 'for' keyword
        for_token = self.consume(TokenType.FOR, "Expected 'for'")
        pos = SourcePosition(for_token.line, for_token.column)
        
        # Parse variable name
        variable_token = self.consume(TokenType.IDENTIFIER, "Expected variable name after 'for'")
        variable = variable_token.value
        
        # Parse 'in' keyword
        self.consume(TokenType.IN, "Expected 'in' after variable name")
        
        # Parse iterable expression
        iterable = self.parse_expression()
        
        # Parse body block
        body = self.parse_block()
        
        return ForInStatement(variable=variable, iterable=iterable, body=body, position=pos)
    
    def parse_break_statement(self) -> BreakStatement:
        """Parse break statement: break"""
        break_token = self.consume(TokenType.BREAK, "Expected 'break'")
        pos = SourcePosition(break_token.line, break_token.column)
        
        return BreakStatement(position=pos)
    
    def parse_continue_statement(self) -> ContinueStatement:
        """Parse continue statement: continue"""
        continue_token = self.consume(TokenType.CONTINUE, "Expected 'continue'")
        pos = SourcePosition(continue_token.line, continue_token.column)
        
        return ContinueStatement(position=pos)
    
    def parse_block(self) -> Block:
        """Parse a block of statements: { statement1; statement2; ... }"""
        # Parse opening brace
        brace_token = self.consume(TokenType.LBRACE, "Expected '{'")
        pos = SourcePosition(brace_token.line, brace_token.column)
        
        # Parse statements
        statements = []
        
        # Skip leading newlines
        while self.match(TokenType.NEWLINE):
            pass
        
        while not self.check(TokenType.RBRACE) and not self.is_at_end():
            # Parse statement
            stmt = self.parse_statement()
            statements.append(stmt)
            
            # Skip trailing newlines and semicolons
            while self.match(TokenType.NEWLINE, TokenType.SEMICOLON):
                pass
        
        # Parse closing brace
        self.consume(TokenType.RBRACE, "Expected '}'")
        
        return Block(statements=statements, position=pos)
    
    def parse_print_statement(self) -> 'PrintStatement':
        """Parse print statement: print(expression1, expression2, ...)"""
        from ..ast.nodes import PrintStatement
        
        # Parse 'print' keyword
        print_token = self.consume(TokenType.PRINT, "Expected 'print'")
        pos = SourcePosition(print_token.line, print_token.column)
        
        # Parse opening parenthesis
        self.consume(TokenType.LPAREN, "Expected '(' after 'print'")
        
        # Parse arguments (can be empty)
        arguments = []
        if not self.check(TokenType.RPAREN):
            # Parse first argument
            arguments.append(self.parse_expression())
            
            # Parse additional arguments separated by commas
            while self.match(TokenType.COMMA):
                arguments.append(self.parse_expression())
        
        # Parse closing parenthesis
        self.consume(TokenType.RPAREN, "Expected ')' after print arguments")
        
        return PrintStatement(arguments=arguments, position=pos)
    
    def parse_print_function_call(self) -> Expression:
        """Parse print function call with optional parentheses: print args or print(args)"""
        from ..ast.nodes import PrintExpression
        
        # Consume 'print' token
        print_token = self.consume(TokenType.IDENTIFIER, "Expected 'print'")
        pos = SourcePosition(print_token.line, print_token.column)
        
        # Check for optional opening parenthesis
        has_parens = self.match(TokenType.LPAREN)
        
        # Parse arguments
        arguments = []
        if (has_parens and not self.check(TokenType.RPAREN)) or \
           (not has_parens and not self.is_at_end() and not self.check(TokenType.NEWLINE) and not self.check(TokenType.EOF)):
            arguments.append(self.parse_expression())
            
            # Arguments can be comma-separated
            while self.match(TokenType.COMMA):
                arguments.append(self.parse_expression())
        
        # If we had opening paren, consume closing paren
        if has_parens:
            self.consume(TokenType.RPAREN, "Expected ')' after print arguments")
        
        # Return a special PrintExpression (we'll need to create this)
        return PrintExpression(arguments=arguments, position=pos)
    
    def parse_expression(self) -> Expression:
        """Parse an expression."""
        return self.parse_comparison()
    
    def parse_comparison(self) -> Expression:
        """Parse comparison operators: >, <, >=, <=, ==, !=, !>, !<"""
        expr = self.parse_term()
        
        while self.check(TokenType.GREATER) or self.check(TokenType.LESS) or \
              self.check(TokenType.GREATER_EQUAL) or self.check(TokenType.LESS_EQUAL) or \
              self.check(TokenType.EQUAL) or self.check(TokenType.NOT_EQUAL) or \
              self.check(TokenType.NOT_GREATER) or self.check(TokenType.NOT_LESS):
            
            operator_token = self.advance()
            right = self.parse_term()
            pos = SourcePosition(operator_token.line, operator_token.column)
            expr = BinaryOperation(expr, operator_token.value, right, pos)
        
        return expr
    
    def parse_term(self) -> Expression:
        """Parse addition, subtraction, and intersection: +, -, &"""
        expr = self.parse_factor()
        
        while (self.match(TokenType.PLUS) or self.match(TokenType.MINUS) or 
               self.match(TokenType.AMPERSAND) or self.match(TokenType.PLUS_DOT) or 
               self.match(TokenType.MINUS_DOT)):
            operator_token = self.previous()
            right = self.parse_factor()
            pos = SourcePosition(operator_token.line, operator_token.column)
            expr = BinaryOperation(expr, operator_token.value, right, pos)
        
        return expr
    
    def parse_factor(self) -> Expression:
        """Parse multiplication, division, and modulo: *, /, %"""
        expr = self.parse_unary()
        
        while (self.match(TokenType.MULTIPLY) or self.match(TokenType.SLASH) or 
               self.match(TokenType.MODULO) or self.match(TokenType.MULTIPLY_DOT) or 
               self.match(TokenType.DIVIDE_DOT) or self.match(TokenType.MODULO_DOT)):
            operator_token = self.previous()
            right = self.parse_unary()
            pos = SourcePosition(operator_token.line, operator_token.column)
            expr = BinaryOperation(expr, operator_token.value, right, pos)
        
        return expr
    
    def parse_unary(self) -> Expression:
        """Parse unary operators: -, !"""
        if self.match(TokenType.MINUS) or self.match(TokenType.NOT):
            operator_token = self.previous()
            expr = self.parse_unary()  # Right associative
            pos = SourcePosition(operator_token.line, operator_token.column)
            return UnaryOperation(operator_token.value, expr, pos)
        
        return self.parse_method_call()
    
    def parse_method_call(self) -> Expression:
        """Parse method calls: expr.method(args)"""
        expr = self.parse_index_access()
        
        while self.match(TokenType.DOT):
            method_token = self.consume(TokenType.IDENTIFIER, "Expected method name")
            method_name = method_token.value
            pos = SourcePosition(method_token.line, method_token.column)
            
            # Optional parentheses for method calls
            self.match(TokenType.LPAREN)
            
            # Parse arguments
            arguments = []
            # Only parse arguments if we have parentheses or if the next token isn't an operator that should end the method call
            if not self.check(TokenType.RPAREN) and not self.is_at_end() and \
               not self.check(TokenType.NEWLINE) and not self.check(TokenType.EOF) and \
               not self.check(TokenType.DOT) and \
               not self.check(TokenType.ASSIGN) and \
               not self.check(TokenType.EQUAL) and not self.check(TokenType.NOT_EQUAL) and \
               not self.check(TokenType.GREATER) and not self.check(TokenType.LESS) and \
               not self.check(TokenType.GREATER_EQUAL) and not self.check(TokenType.LESS_EQUAL) and \
               not self.check(TokenType.NOT_GREATER) and not self.check(TokenType.NOT_LESS) and \
               not self.check(TokenType.PLUS) and not self.check(TokenType.MINUS) and \
               not self.check(TokenType.MULTIPLY) and not self.check(TokenType.SLASH) and \
               not self.check(TokenType.MODULO) and \
               not self.check(TokenType.PLUS_DOT) and not self.check(TokenType.MINUS_DOT) and \
               not self.check(TokenType.MULTIPLY_DOT) and not self.check(TokenType.DIVIDE_DOT) and \
               not self.check(TokenType.MODULO_DOT):
                arguments.append(self.parse_expression())
                
                # Arguments can be comma-separated or space-separated
                while self.match(TokenType.COMMA):
                    arguments.append(self.parse_expression())
            
            # Optional closing paren
            self.match(TokenType.RPAREN)
            
            # Create method call expression
            expr = MethodCallExpression(expr, method_name, arguments, pos)
        
        return expr
    
    def parse_index_access(self) -> Expression:
        """Parse index/slice access: expr[index] or expr[start:stop:step]"""
        expr = self.parse_primary()
        
        while self.match(TokenType.LBRACKET):
            # Check if this is a slice (contains :)
            if self.check_slice_syntax():
                start = None
                if not self.check(TokenType.COLON):
                    start = self.parse_expression()
                
                self.consume(TokenType.COLON, "Expected ':'")
                
                stop = None
                if not self.check(TokenType.COLON) and not self.check(TokenType.RBRACKET):
                    stop = self.parse_expression()
                
                step = None
                if self.match(TokenType.COLON):
                    if not self.check(TokenType.RBRACKET):
                        step = self.parse_expression()
                
                self.consume(TokenType.RBRACKET, "Expected ']'")
                expr = SliceAccess(expr, start, stop, step)
            else:
                # Regular index access - support chaining
                index = self.parse_expression()
                self.consume(TokenType.RBRACKET, "Expected ']'")
                
                # For now, treat each [index] as a separate IndexAccess
                # This allows for natural chaining: arr[i][j] -> IndexAccess(IndexAccess(arr, [i]), [j])
                expr = IndexAccess(expr, [index])
        
        return expr
    
    def parse_primary(self) -> Expression:
        """Parse primary expressions."""
        
        # Boolean literals
        if self.match(TokenType.TRUE):
            token = self.previous()
            return BooleanLiteral(True, SourcePosition(token.line, token.column))
        if self.match(TokenType.FALSE):
            token = self.previous()
            return BooleanLiteral(False, SourcePosition(token.line, token.column))
        
        # Number literals
        if self.check(TokenType.NUMBER_LITERAL):
            token = self.advance()
            value = float(token.value) if '.' in token.value else int(token.value)
            return NumberLiteral(value, SourcePosition(token.line, token.column))
        
        # String literals
        if self.check(TokenType.STRING_LITERAL):
            token = self.advance()
            # Remove quotes and handle escape sequences
            value = self.process_string_literal(token.value)
            return StringLiteral(value, SourcePosition(token.line, token.column))
        
        # List literals
        if self.match(TokenType.LBRACKET):
            bracket_token = self.previous()
            elements = []
            
            if not self.check(TokenType.RBRACKET):
                elements.append(self.parse_expression())
                
                while self.match(TokenType.COMMA):
                    elements.append(self.parse_expression())
            
            self.consume(TokenType.RBRACKET, "Expected ']' after list elements")
            return ListLiteral(elements, SourcePosition(bracket_token.line, bracket_token.column))
        
        # Data node literals and Map literals: { "key": value } or { "key1": value1, "key2": value2 }
        if self.match(TokenType.LBRACE):
            brace_token = self.previous()
            pairs = []
            
            # Handle empty braces (empty map)
            if self.check(TokenType.RBRACE):
                self.advance()  # consume closing brace
                return MapLiteral(pairs, SourcePosition(brace_token.line, brace_token.column))
            
            # Parse first key-value pair
            if not self.check(TokenType.STRING_LITERAL):
                raise ParseError("Key must be a string literal", self.peek())
            key_token = self.advance()
            key = self.process_string_literal(key_token.value)
            
            self.consume(TokenType.COLON, "Expected ':' after key")
            value = self.parse_expression()
            pairs.append((key, value))
            
            # Check if there are more pairs (comma-separated)
            while self.match(TokenType.COMMA):
                if not self.check(TokenType.STRING_LITERAL):
                    raise ParseError("Key must be a string literal", self.peek())
                key_token = self.advance()
                key = self.process_string_literal(key_token.value)
                
                self.consume(TokenType.COLON, "Expected ':' after key")
                value = self.parse_expression()
                pairs.append((key, value))
            
            self.consume(TokenType.RBRACE, "Expected '}' after pairs")
            
            # If there's exactly one pair, return a DataNodeLiteral for backward compatibility
            if len(pairs) == 1:
                key, value = pairs[0]
                return DataNodeLiteral(key, value, SourcePosition(brace_token.line, brace_token.column))
            else:
                # Multiple pairs = map literal
                return MapLiteral(pairs, SourcePosition(brace_token.line, brace_token.column))
        
        # Special print function call
        if self.check(TokenType.IDENTIFIER) and self.peek().value == "print":
            return self.parse_print_function_call()
        
        # Variable references (including keywords used as variables)  
        if self.check(TokenType.IDENTIFIER) or self.check_type_keyword():
            token = self.advance()
            return VariableRef(token.value, SourcePosition(token.line, token.column))
        
        # Parenthesized expressions
        if self.match(TokenType.LPAREN):
            expr = self.parse_expression()
            self.consume(TokenType.RPAREN, "Expected ')' after expression")
            return expr
        
        # Error case - provide helpful context-specific error messages
        current_token = self.peek()
        error_msg = self.get_helpful_error_message(current_token)
        raise ParseError(error_msg, current_token)
    
    
    def get_helpful_error_message(self, token: Token) -> str:
        """Generate a helpful error message based on the current token and context."""
        if token.type == TokenType.PLUS:
            # Check if this might be a method call issue
            if self.current > 0:
                prev_token = self.tokens[self.current - 1]
                if prev_token.type == TokenType.IDENTIFIER:
                    return f"'{prev_token.value}' looks like a method call. Did you mean '{prev_token.value}()' or forget an operator before '+'?"
            return f"Unexpected '+' operator. Are you missing a left operand or trying to use '+' in an invalid context?"
        
        elif token.type == TokenType.IDENTIFIER:
            # Common mistakes with identifiers
            return f"Unexpected identifier '{token.value}'. Are you missing an operator, assignment (=), or trying to call a method?"
        
        elif token.type == TokenType.DOT:
            return "Unexpected '.'. Method calls need an object before the dot (e.g., 'object.method()')"
        
        elif token.type == TokenType.COLON:
            return "Unexpected ':'. Colons are used in data structures like { \"key\": value } or control flow like if/while statements"
        
        elif token.type == TokenType.RBRACE:
            return "Unexpected '}'. Check if you have a matching '{' or are missing content in a data structure"
        
        elif token.type == TokenType.RBRACKET:
            return "Unexpected ']'. Check if you have a matching '[' or are missing content in a list"
        
        elif token.type == TokenType.RPAREN:
            return "Unexpected ')'. Check if you have a matching '(' or are missing content in parentheses"
        
        elif token.type == TokenType.COMMA:
            return "Unexpected ','. Commas separate elements in lists, function arguments, or data structures"
        
        elif token.type == TokenType.ASSIGN:
            return "Unexpected '=' assignment operator. Are you trying to assign to an invalid target?"
        
        elif token.type in [TokenType.EQUAL, TokenType.NOT_EQUAL, TokenType.GREATER, TokenType.LESS]:
            return f"Unexpected comparison operator '{token.value}'. Comparisons need expressions on both sides"
        
        elif token.type == TokenType.EOF:
            return "Unexpected end of input. Are you missing a closing bracket, brace, or parenthesis?"
        
        elif token.type == TokenType.NEWLINE:
            return "Unexpected newline. Are you missing a semicolon or have an incomplete statement?"
        
        else:
            # Generic fallback with more context
            return f"Unexpected token '{token.value}' of type {token.type.name}. Expected a value, variable name, or expression"

    def process_string_literal(self, literal: str) -> str:
        """Process string literal, removing quotes and handling escapes."""
        # Remove surrounding quotes
        content = literal[1:-1]
        
        # Handle basic escape sequences
        content = content.replace('\\"', '"')
        content = content.replace("\\'", "'")
        content = content.replace('\\\\', '\\')
        content = content.replace('\\n', '\n')
        content = content.replace('\\t', '\t')
        
        return content
    
    # Helper methods for parsing
    def match(self, *types: TokenType) -> bool:
        """Check if current token matches any of the given types."""
        for token_type in types:
            if self.check(token_type):
                self.advance()
                return True
        return False
    
    def check(self, token_type: TokenType) -> bool:
        """Check if current token is of given type."""
        if self.is_at_end():
            return False
        return self.peek().type == token_type
    
    def advance(self) -> Token:
        """Consume current token and return it."""
        if not self.is_at_end():
            self.current += 1
        return self.previous()
    
    def is_at_end(self) -> bool:
        """Check if we're at end of tokens."""
        return self.peek().type == TokenType.EOF
    
    def peek(self) -> Token:
        """Return current token without consuming it."""
        return self.tokens[self.current]
    
    def previous(self) -> Token:
        """Return previous token."""
        return self.tokens[self.current - 1]
    
    def consume(self, token_type: TokenType, message: str) -> Token:
        """Consume token of expected type or raise error."""
        if self.check(token_type):
            return self.advance()
        
        current_token = self.peek()
        enhanced_message = self.enhance_expected_token_message(token_type, current_token, message)
        raise ParseError(enhanced_message, current_token)
    
    def enhance_expected_token_message(self, expected_type: TokenType, actual_token: Token, base_message: str) -> str:
        """Enhance error messages for expected token mismatches."""
        # More helpful messages for common expected token scenarios
        if expected_type == TokenType.RPAREN:
            if actual_token.type == TokenType.EOF:
                return f"{base_message}. Reached end of input - are you missing a closing parenthesis ')' somewhere?"
            elif actual_token.type == TokenType.NEWLINE:
                return f"{base_message}. Found newline instead - missing closing parenthesis ')' on this line?"
            else:
                return f"{base_message}. Got '{actual_token.value}' instead of ')'"
        
        elif expected_type == TokenType.RBRACKET:
            if actual_token.type == TokenType.EOF:
                return f"{base_message}. Reached end of input - are you missing a closing bracket ']' somewhere?"
            else:
                return f"{base_message}. Got '{actual_token.value}' instead of ']'"
        
        elif expected_type == TokenType.RBRACE:
            if actual_token.type == TokenType.EOF:
                return f"{base_message}. Reached end of input - are you missing a closing brace '}}' somewhere?"
            else:
                return f"{base_message}. Got '{actual_token.value}' instead of '}}'"
        
        elif expected_type == TokenType.COLON:
            return f"{base_message}. Data structures need colons between keys and values (e.g., {{ \"key\": value }}). Got '{actual_token.value}'"
        
        elif expected_type == TokenType.IDENTIFIER:
            if actual_token.type == TokenType.STRING_LITERAL:
                return f"{base_message}. Got string '{actual_token.value}' - did you forget to remove quotes from a variable name?"
            elif actual_token.type == TokenType.NUMBER_LITERAL:
                return f"{base_message}. Got number '{actual_token.value}' - variable names must start with a letter"
            else:
                return f"{base_message}. Got '{actual_token.value}' - expected a variable or method name"
        
        else:
            # Fallback with better context
            expected_symbol = self.token_type_to_symbol(expected_type)
            return f"{base_message}. Expected '{expected_symbol}' but got '{actual_token.value}'"
    
    def token_type_to_symbol(self, token_type: TokenType) -> str:
        """Convert token type to human-readable symbol."""
        symbol_map = {
            TokenType.LPAREN: '(',
            TokenType.RPAREN: ')',
            TokenType.LBRACKET: '[',
            TokenType.RBRACKET: ']',
            TokenType.LBRACE: '{',
            TokenType.RBRACE: '}',
            TokenType.COLON: ':',
            TokenType.COMMA: ',',
            TokenType.DOT: '.',
            TokenType.ASSIGN: '=',
            TokenType.PLUS: '+',
            TokenType.MINUS: '-',
            TokenType.MULTIPLY: '*',
            TokenType.SLASH: '/',
            TokenType.MODULO: '%',
            TokenType.SEMICOLON: ';',
            TokenType.IDENTIFIER: 'identifier',
            TokenType.STRING_LITERAL: 'string',
            TokenType.NUMBER_LITERAL: 'number'
        }
        return symbol_map.get(token_type, token_type.name.lower())
    
    def check_type_keyword(self) -> bool:
        """Check if current token is a type keyword."""
        current_token = self.peek()
        if not current_token or current_token.type == TokenType.EOF:
            return False
        
        # Use registry to check type keywords
        type_keywords = KEYWORD_REGISTRY.get_keywords_by_category(KeywordCategory.TYPE)
        for keyword_name in type_keywords:
            token_type_name = KEYWORD_REGISTRY.get_token_type_name(keyword_name)
            if hasattr(TokenType, token_type_name):
                expected_token_type = getattr(TokenType, token_type_name)
                if self.check(expected_token_type):
                    return True
        return False
    
    def is_type_keyword_name(self, name: str) -> bool:
        """Check if a string name is a type keyword."""
        type_keywords = KEYWORD_REGISTRY.get_keywords_by_category(KeywordCategory.TYPE)
        return name.lower() in type_keywords
    
    def consume_type_keyword(self, message: str) -> Token:
        """Consume a type keyword token."""
        if self.check_type_keyword():
            return self.advance()
        current = self.peek()
        raise ParseError(f"{message}. Got '{current.value}'", current)
    
    def check_slice_syntax(self) -> bool:
        """Look ahead to see if this is slice syntax (contains : before ])"""
        saved_pos = self.current
        try:
            # Look for : before ]
            paren_depth = 0
            bracket_depth = 0
            
            while not self.is_at_end():
                token_type = self.peek().type
                
                if token_type == TokenType.RBRACKET and bracket_depth == 0:
                    return False  # Found ] without :
                elif token_type == TokenType.COLON and bracket_depth == 0 and paren_depth == 0:
                    return True   # Found : at the right level
                elif token_type == TokenType.LBRACKET:
                    bracket_depth += 1
                elif token_type == TokenType.RBRACKET:
                    bracket_depth -= 1
                elif token_type == TokenType.LPAREN:
                    paren_depth += 1
                elif token_type == TokenType.RPAREN:
                    paren_depth -= 1
                
                self.advance()
                
            return False
        finally:
            self.current = saved_pos
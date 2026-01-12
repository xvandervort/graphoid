//! Parser module for building AST from tokens
//!
//! This module implements a recursive descent parser with precedence climbing
//! for expression parsing.

use crate::ast::{
    Argument, AssignmentTarget, BinaryOp, Expr, GraphMethod, GraphProperty, GraphRule,
    LiteralValue, Parameter, Pattern, PatternClause, Program, Stmt, TypeAnnotation,
    UnaryOp,
};
use std::collections::HashMap;
use crate::error::{GraphoidError, Result, SourcePosition};
use crate::lexer::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    /// When true, disables parsing ClassName {} as instantiation in postfix()
    /// Used when parsing graph parent expressions where {} is the graph body
    disable_brace_instantiation: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            disable_brace_instantiation: false,
        }
    }

    pub fn parse(&mut self) -> Result<Program> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            // Skip newlines at the top level
            if self.match_token(&TokenType::Newline) {
                continue;
            }

            if self.is_at_end() {
                break;
            }

            statements.push(self.statement()?);
        }

        Ok(Program { statements })
    }

    // Statement parsing
    fn statement(&mut self) -> Result<Stmt> {
        // Skip leading newlines
        while self.match_token(&TokenType::Newline) {}

        if self.is_at_end() {
            return Err(GraphoidError::SyntaxError {
                message: "Unexpected end of input".to_string(),
                position: self.previous_position(),
            });
        }

        // Check for priv keyword (Phase 10)
        let is_private = self.match_token(&TokenType::Priv);

        // If we have priv followed by an identifier and =, it's a private variable declaration
        if is_private && matches!(self.peek().token_type, TokenType::Identifier(_)) {
            // Look ahead to see if there's an = after the identifier
            let is_assignment = self.tokens.get(self.current + 1)
                .map(|t| matches!(t.token_type, TokenType::Equal))
                .unwrap_or(false);

            if is_assignment {
                return self.priv_variable_declaration_without_type();
            }
        }

        // Check for type annotations or keywords
        // BUT: If ListType or StringType is followed by dot, it's a static method call, not a declaration
        let is_list_static_call = self.check(&TokenType::ListType) && self.check_next(&TokenType::Dot);
        let is_string_static_call = self.check(&TokenType::StringType) && self.check_next(&TokenType::Dot);

        // Check for named graph declaration: graph Name { }
        // GraphType followed by Identifier (not { or from or () is a named declaration
        let is_named_graph_decl = self.check(&TokenType::GraphType) && self.check_next_is_identifier();

        let result = if is_named_graph_decl {
            self.graph_declaration()
        } else if !is_list_static_call && !is_string_static_call && (
            self.check(&TokenType::NumType)
            || self.check(&TokenType::BigNumType)  // Phase 1B
            || self.check(&TokenType::StringType)
            || self.check(&TokenType::BoolType)
            || self.check(&TokenType::ListType)
            || self.check(&TokenType::MapType)
            || self.check(&TokenType::TreeType)
            || self.check(&TokenType::GraphType)
        ) {
            self.variable_declaration(is_private)
        } else if self.match_token(&TokenType::Func) {
            self.function_declaration(is_private, false, false)  // fn = not a setter, not static
        } else if self.match_token(&TokenType::Set) {
            self.function_declaration(is_private, true, false)   // set = setter
        } else if self.match_token(&TokenType::Static) {
            // static fn ... - expect fn keyword next
            if !self.match_token(&TokenType::Func) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected 'fn' after 'static'".to_string(),
                    position: self.peek().position(),
                });
            }
            self.function_declaration(is_private, false, true)   // static fn = static method
        } else if self.match_token(&TokenType::If) {
            self.if_statement()
        } else if self.match_token(&TokenType::While) {
            self.while_statement()
        } else if self.match_token(&TokenType::For) {
            self.for_statement()
        } else if self.match_token(&TokenType::Return) {
            self.return_statement()
        } else if self.match_token(&TokenType::Break) {
            let position = self.previous_position();
            Ok(Stmt::Break { position })
        } else if self.match_token(&TokenType::Continue) {
            let position = self.previous_position();
            Ok(Stmt::Continue { position })
        } else if self.match_token(&TokenType::Import) {
            self.import_statement()
        } else if self.match_token(&TokenType::Load) {
            self.load_statement()
        } else if self.match_token(&TokenType::Module) {
            self.module_declaration()
        } else if self.match_token(&TokenType::Configure) {
            self.configure_statement()
        } else if self.match_token(&TokenType::Precision) {
            self.precision_statement()
        } else if self.match_token(&TokenType::Try) {
            self.try_catch_statement()
        } else {
            // Try to parse as assignment or expression
            self.assignment_or_expression()
        };

        // Consume optional trailing newline
        self.match_token(&TokenType::Newline);

        result
    }

    fn variable_declaration(&mut self, is_private: bool) -> Result<Stmt> {
        let position = self.peek().position();

        // Parse type annotation
        let type_annotation = self.type_annotation()?;

        // Parse the rest of the variable declaration
        self.variable_declaration_common(position, Some(type_annotation), is_private)
    }

    /// Parse a private variable declaration without an explicit type annotation
    /// e.g., priv SECRET = "value"
    fn priv_variable_declaration_without_type(&mut self) -> Result<Stmt> {
        let position = self.peek().position();

        // Parse variable declaration without type annotation
        self.variable_declaration_common(position, None, true)
    }

    /// Common logic for parsing variable declarations
    fn variable_declaration_common(
        &mut self,
        position: SourcePosition,
        type_annotation: Option<TypeAnnotation>,
        is_private: bool
    ) -> Result<Stmt> {
        // Expect identifier
        let name = if let TokenType::Identifier(id) = &self.peek().token_type {
            let n = id.clone();
            self.advance();
            n
        } else {
            return Err(GraphoidError::SyntaxError {
                message: format!("Expected identifier, got {:?}", self.peek().token_type),
                position: self.peek().position(),
            });
        };

        // Expect '='
        if !self.match_token(&TokenType::Equal) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '=' after variable name".to_string(),
                position: self.peek().position(),
            });
        }

        // Parse value expression
        let value = self.expression()?;

        Ok(Stmt::VariableDecl {
            name,
            type_annotation,
            value,
            is_private,
            position,
        })
    }

    fn type_annotation(&mut self) -> Result<TypeAnnotation> {
        let base_type = match &self.peek().token_type {
            TokenType::NumType => "num",
            TokenType::BigNumType => "bignum",  // Phase 1B
            TokenType::StringType => "string",
            TokenType::BoolType => "bool",
            TokenType::ListType => "list",
            TokenType::MapType => "map",
            TokenType::TreeType => "tree",
            TokenType::GraphType => "graph",
            TokenType::DataType => "data",
            _ => {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected type annotation".to_string(),
                    position: self.peek().position(),
                })
            }
        }
        .to_string();

        self.advance();

        // Parse type constraints (e.g., list<num>)
        // ⚠️  IMPORTANT: NO GENERICS POLICY ENFORCEMENT
        // See: dev_docs/NO_GENERICS_POLICY.md
        let constraint = if self.match_token(&TokenType::Less) {
            // Type constraints only allowed on built-in collections
            if base_type != "list" && base_type != "hash" && base_type != "tree" && base_type != "graph" {
                return Err(GraphoidError::SyntaxError {
                    message: format!("Type parameters only allowed on built-in collections (list, hash, tree, graph), not '{}'", base_type),
                    position: self.peek().position(),
                });
            }

            // Parse the constraint type (must be a primitive)
            let constraint_type = match &self.peek().token_type {
                TokenType::NumType => "num",
                TokenType::BigNumType => "bignum",  // Phase 1B
                TokenType::StringType => "string",
                TokenType::BoolType => "bool",
                TokenType::Symbol(_) => {
                    // Allow symbol types like :symbol
                    if let TokenType::Symbol(s) = &self.peek().token_type {
                        s.as_str()
                    } else {
                        "symbol"
                    }
                }
                TokenType::Identifier(id) if id == "none" => "none",
                _ => {
                    return Err(GraphoidError::SyntaxError {
                        message: format!("Type constraint must be a primitive type (num, string, bool, time, none), got {:?}", self.peek().token_type),
                        position: self.peek().position(),
                    });
                }
            }.to_string();

            self.advance();

            // Check for multiple type parameters (FORBIDDEN)
            if self.check(&TokenType::Comma) {
                return Err(GraphoidError::SyntaxError {
                    message: "Multiple type parameters not supported. See: dev_docs/NO_GENERICS_POLICY.md".to_string(),
                    position: self.peek().position(),
                });
            }

            // Check for nested constraints (FORBIDDEN)
            if self.check(&TokenType::Less) {
                return Err(GraphoidError::SyntaxError {
                    message: "Nested type constraints not supported. See: dev_docs/NO_GENERICS_POLICY.md".to_string(),
                    position: self.peek().position(),
                });
            }

            // Expect closing '>'
            if !self.match_token(&TokenType::Greater) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '>' after type constraint".to_string(),
                    position: self.peek().position(),
                });
            }

            Some(constraint_type)
        } else {
            None
        };

        Ok(TypeAnnotation {
            base_type,
            constraint,
        })
    }

    fn function_declaration(&mut self, is_private: bool, is_setter: bool, is_static: bool) -> Result<Stmt> {
        let position = self.previous_position();

        // Expect identifier (could be function name or receiver for method syntax)
        let first_ident = if let TokenType::Identifier(id) = &self.peek().token_type {
            let n = id.clone();
            self.advance();
            n
        } else {
            return Err(GraphoidError::SyntaxError {
                message: "Expected function name".to_string(),
                position: self.peek().position(),
            });
        };

        // Check for method syntax: fn receiver.method_name()
        let (name, receiver) = if self.match_token(&TokenType::Dot) {
            // fn receiver.method_name() syntax
            let method_name = if let TokenType::Identifier(id) = &self.peek().token_type {
                let n = id.clone();
                self.advance();
                n
            } else {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected method name after '.'".to_string(),
                    position: self.peek().position(),
                });
            };
            (method_name, Some(first_ident))
        } else {
            // Regular function: fn function_name()
            (first_ident, None)
        };

        // ⚠️  NO GENERICS POLICY ENFORCEMENT
        // See: dev_docs/NO_GENERICS_POLICY.md
        //
        // If next token is '<', this is an attempt at generic function syntax
        // fn foo<T>(...) → FORBIDDEN
        if self.check(&TokenType::Less) {
            return Err(GraphoidError::SyntaxError {
                message: "Generic functions are not supported in Graphoid. Use duck typing instead - functions work on values, not types. See: dev_docs/NO_GENERICS_POLICY.md".to_string(),
                position: self.peek().position(),
            });
        }

        // Expect '('
        if !self.match_token(&TokenType::LeftParen) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '(' after function name".to_string(),
                position: self.peek().position(),
            });
        }

        // Parse parameters
        let mut params = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                // Check for variadic parameter (...)
                let is_variadic = if self.match_token(&TokenType::DotDotDot) {
                    true
                } else {
                    false
                };

                let param_name = if let TokenType::Identifier(id) = &self.peek().token_type {
                    let n = id.clone();
                    self.advance();
                    n
                } else {
                    return Err(GraphoidError::SyntaxError {
                        message: "Expected parameter name".to_string(),
                        position: self.peek().position(),
                    });
                };

                // Variadic parameters cannot have default values
                let default_value = if is_variadic {
                    if self.check(&TokenType::Equal) {
                        return Err(GraphoidError::SyntaxError {
                            message: "Variadic parameters cannot have default values".to_string(),
                            position: self.peek().position(),
                        });
                    }
                    None
                } else if self.match_token(&TokenType::Equal) {
                    Some(self.expression()?)
                } else {
                    None
                };

                params.push(Parameter {
                    name: param_name,
                    default_value,
                    is_variadic,
                });

                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }

        // Expect ')'
        if !self.match_token(&TokenType::RightParen) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected ')' after parameters".to_string(),
                position: self.peek().position(),
            });
        }

        // Phase 19: Setters must have exactly one parameter (the value being assigned)
        if is_setter && params.len() != 1 {
            return Err(GraphoidError::SyntaxError {
                message: "Setters must have exactly one parameter (the value being assigned).".to_string(),
                position: self.peek().position(),
            });
        }

        // Phase 20: Static methods require a receiver (must be attached to a graph)
        if is_static && receiver.is_none() {
            return Err(GraphoidError::SyntaxError {
                message: "Static methods must be attached to a graph. Use `static fn GraphName.method_name()`.".to_string(),
                position: self.peek().position(),
            });
        }

        // Phase 21: Parse optional `when` guard clause
        // Syntax: fn Graph.method() when condition { ... }
        let guard = if self.match_token(&TokenType::When) {
            // Parse the guard expression
            let guard_expr = self.expression()?;
            Some(Box::new(guard_expr))
        } else {
            None
        };

        // Expect '{'
        if !self.match_token(&TokenType::LeftBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '{' before function body".to_string(),
                position: self.peek().position(),
            });
        }

        // Skip newlines after opening brace
        while self.match_token(&TokenType::Newline) {}

        // Check if function body starts with pattern clauses (|pattern| => ...)
        // or regular body statements
        let (body, pattern_clauses) = if self.check(&TokenType::Pipe) {
            // Pattern matching function
            let mut clauses = Vec::new();

            // Parse pattern clauses
            while self.check(&TokenType::Pipe) {
                clauses.push(self.parse_pattern_clause()?);

                // Skip newlines between clauses
                while self.match_token(&TokenType::Newline) {}
            }

            if clauses.is_empty() {
                return Err(GraphoidError::SyntaxError {
                    message: "Pattern matching function must have at least one pattern clause".to_string(),
                    position: self.peek().position(),
                });
            }

            // Pattern matching functions have empty body
            (vec![], Some(clauses))
        } else {
            // Regular function with body
            let body = self.block()?;
            (body, None)
        };

        // Expect '}'
        if !self.match_token(&TokenType::RightBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '}' after function body".to_string(),
                position: self.peek().position(),
            });
        }

        Ok(Stmt::FunctionDecl {
            name,
            receiver,  // For method syntax: fn Graph.method()
            params,
            body,
            pattern_clauses,
            is_private,  // Phase 10: priv keyword support
            is_setter,   // Phase 19: computed property assignment
            is_static,   // Phase 20: class methods
            guard,       // Phase 21: structure-based dispatch
            position,
        })
    }

    fn if_statement(&mut self) -> Result<Stmt> {
        let position = self.previous_position();

        // Parse condition
        let condition = self.expression()?;

        // Expect 'then' or '{'
        let has_then = self.match_token(&TokenType::Then);
        let has_brace = self.match_token(&TokenType::LeftBrace);

        if !has_then && !has_brace {
            return Err(GraphoidError::SyntaxError {
                message: "Expected 'then' or '{' after if condition".to_string(),
                position: self.peek().position(),
            });
        }

        // Parse then branch
        let then_branch = if has_brace {
            let stmts = self.block()?;
            if !self.match_token(&TokenType::RightBrace) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '}' after if body".to_string(),
                    position: self.peek().position(),
                });
            }
            stmts
        } else {
            vec![self.statement()?]
        };

        // Parse optional else branch
        let else_branch = if self.match_token(&TokenType::Else) {
            if self.match_token(&TokenType::LeftBrace) {
                let stmts = self.block()?;
                if !self.match_token(&TokenType::RightBrace) {
                    return Err(GraphoidError::SyntaxError {
                        message: "Expected '}' after else body".to_string(),
                        position: self.peek().position(),
                    });
                }
                Some(stmts)
            } else {
                Some(vec![self.statement()?])
            }
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
            position,
        })
    }

    fn while_statement(&mut self) -> Result<Stmt> {
        let position = self.previous_position();

        // Parse condition
        let condition = self.expression()?;

        // Expect '{'
        if !self.match_token(&TokenType::LeftBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '{' after while condition".to_string(),
                position: self.peek().position(),
            });
        }

        // Parse body
        let body = self.block()?;

        // Expect '}'
        if !self.match_token(&TokenType::RightBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '}' after while body".to_string(),
                position: self.peek().position(),
            });
        }

        Ok(Stmt::While {
            condition,
            body,
            position,
        })
    }

    fn for_statement(&mut self) -> Result<Stmt> {
        let position = self.previous_position();

        // Expect identifier
        let variable = if let TokenType::Identifier(id) = &self.peek().token_type {
            let v = id.clone();
            self.advance();
            v
        } else {
            return Err(GraphoidError::SyntaxError {
                message: "Expected variable name in for loop".to_string(),
                position: self.peek().position(),
            });
        };

        // Expect 'in'
        if !self.match_token(&TokenType::In) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected 'in' after for variable".to_string(),
                position: self.peek().position(),
            });
        }

        // Parse iterable
        let iterable = self.expression()?;

        // Expect '{'
        if !self.match_token(&TokenType::LeftBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '{' after for clause".to_string(),
                position: self.peek().position(),
            });
        }

        // Parse body
        let body = self.block()?;

        // Expect '}'
        if !self.match_token(&TokenType::RightBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '}' after for body".to_string(),
                position: self.peek().position(),
            });
        }

        Ok(Stmt::For {
            variable,
            iterable,
            body,
            position,
        })
    }

    fn return_statement(&mut self) -> Result<Stmt> {
        let position = self.previous_position();

        // Check if there's a value to return
        let value = if self.check(&TokenType::Newline) || self.is_at_end() {
            None
        } else {
            Some(self.expression()?)
        };

        Ok(Stmt::Return { value, position })
    }

    fn import_statement(&mut self) -> Result<Stmt> {
        let position = self.previous_position();

        // Expect string literal
        let module = if let TokenType::String(s) = &self.peek().token_type {
            let m = s.clone();
            self.advance();
            m
        } else {
            return Err(GraphoidError::SyntaxError {
                message: "Expected string literal after 'import'".to_string(),
                position: self.peek().position(),
            });
        };

        // Check for 'as' alias
        let alias = if self.match_token(&TokenType::As) {
            if let TokenType::Identifier(id) = &self.peek().token_type {
                let a = id.clone();
                self.advance();
                Some(a)
            } else {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected identifier after 'as'".to_string(),
                    position: self.peek().position(),
                });
            }
        } else {
            None
        };

        Ok(Stmt::Import {
            module,
            alias,
            position,
        })
    }

    fn load_statement(&mut self) -> Result<Stmt> {
        let position = self.previous_position();

        // Expect string literal
        let path = if let TokenType::String(s) = &self.peek().token_type {
            let p = s.clone();
            self.advance();
            p
        } else {
            return Err(GraphoidError::SyntaxError {
                message: "Expected string literal after 'load'".to_string(),
                position: self.peek().position(),
            });
        };

        Ok(Stmt::Load { path, position })
    }

    fn module_declaration(&mut self) -> Result<Stmt> {
        let position = self.previous_position();

        // Expect identifier
        let name = if let TokenType::Identifier(id) = &self.peek().token_type {
            let n = id.clone();
            self.advance();
            n
        } else {
            return Err(GraphoidError::SyntaxError {
                message: "Expected module name".to_string(),
                position: self.peek().position(),
            });
        };

        // Check for 'alias' keyword (optional)
        let alias = if self.match_token(&TokenType::Alias) {
            if let TokenType::Identifier(id) = &self.peek().token_type {
                let a = id.clone();
                self.advance();
                Some(a)
            } else {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected identifier after 'alias'".to_string(),
                    position: self.peek().position(),
                });
            }
        } else {
            None
        };

        Ok(Stmt::ModuleDecl {
            name,
            alias,
            position,
        })
    }

    /// Parse named graph declaration: graph Name { properties, methods }
    /// or: graph Name(:type) { ... }
    /// or: graph Name from Parent { ... }
    fn graph_declaration(&mut self) -> Result<Stmt> {
        let position = self.peek().position();

        // Consume the 'graph' keyword
        self.advance();

        // Get the graph name (required for named declarations)
        let name = if let TokenType::Identifier(id) = &self.peek().token_type {
            let n = id.clone();
            self.advance();
            n
        } else {
            return Err(GraphoidError::SyntaxError {
                message: "Expected graph name after 'graph' keyword".to_string(),
                position: self.peek().position(),
            });
        };

        // Check for optional graph type: graph Name(:dag) { }
        let graph_type = if self.match_token(&TokenType::LeftParen) {
            // Expect a symbol like :dag, :tree (tokenized as Symbol)
            let gtype = if let TokenType::Symbol(s) = &self.peek().token_type {
                let t = s.clone();
                self.advance();
                Some(t)
            } else {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected graph type symbol (e.g., :dag, :tree)".to_string(),
                    position: self.peek().position(),
                });
            };

            if !self.match_token(&TokenType::RightParen) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected ')' after graph type".to_string(),
                    position: self.peek().position(),
                });
            }

            gtype
        } else {
            None
        };

        // Check for inheritance: graph Name from Parent { }
        let parent = if self.match_token(&TokenType::From) {
            // Disable brace instantiation so `Parent { ... }` isn't parsed as
            // an instantiation - the { } is the graph body, not instantiation overrides
            self.disable_brace_instantiation = true;
            let parent_expr = self.expression()?;
            self.disable_brace_instantiation = false;
            Some(Box::new(parent_expr))
        } else {
            None
        };

        // Expect opening brace
        if !self.match_token(&TokenType::LeftBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '{' after graph declaration".to_string(),
                position: self.peek().position(),
            });
        }

        // Parse graph body: properties, methods, rules, and config
        let (properties, methods, rules, config) = self.parse_graph_body()?;

        // Expect closing brace
        if !self.match_token(&TokenType::RightBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '}' to close graph declaration".to_string(),
                position: self.peek().position(),
            });
        }

        Ok(Stmt::GraphDecl {
            name,
            graph_type,
            parent,
            properties,
            methods,
            rules,
            config,
            position,
        })
    }

    /// Parse the body of a graph declaration: properties, methods, rules, and config
    fn parse_graph_body(&mut self) -> Result<(Vec<GraphProperty>, Vec<GraphMethod>, Vec<GraphRule>, HashMap<String, Vec<String>>)> {
        let mut properties = Vec::new();
        let mut methods = Vec::new();
        let mut rules = Vec::new();
        let mut config: HashMap<String, Vec<String>> = HashMap::new();

        loop {
            // Skip newlines and semicolons (flexible separators)
            while self.match_token(&TokenType::Newline) || self.match_token(&TokenType::Semicolon) {}

            // Check for end of body
            if self.check(&TokenType::RightBrace) || self.is_at_end() {
                break;
            }

            // Skip commas between entries
            if self.match_token(&TokenType::Comma) {
                continue;
            }

            // Check for configure block: configure { readable: :x, writable: [:y, :z] }
            if self.match_token(&TokenType::Configure) {
                if !self.match_token(&TokenType::LeftBrace) {
                    return Err(GraphoidError::SyntaxError {
                        message: "Expected '{' after 'configure'".to_string(),
                        position: self.peek().position(),
                    });
                }

                // Parse config entries: key: value, key: [values]
                self.parse_graph_config(&mut config)?;

                if !self.match_token(&TokenType::RightBrace) {
                    return Err(GraphoidError::SyntaxError {
                        message: "Expected '}' to close configure block".to_string(),
                        position: self.peek().position(),
                    });
                }
                continue;
            }

            // Check for rule declaration: rule :name or rule :name, param
            if self.match_token(&TokenType::Rule) {
                let rule_pos = self.previous().position();

                // Expect a symbol
                if let TokenType::Symbol(rule_name) = &self.peek().token_type {
                    let name = rule_name.clone();
                    self.advance();

                    // Check for optional parameter (comma followed by expression)
                    let param = if self.match_token(&TokenType::Comma) {
                        Some(self.expression()?)
                    } else {
                        None
                    };

                    rules.push(GraphRule {
                        name,
                        param,
                        position: rule_pos,
                    });
                    continue;
                } else {
                    return Err(GraphoidError::SyntaxError {
                        message: "Expected symbol after 'rule' (e.g., rule :no_cycles)".to_string(),
                        position: self.peek().position(),
                    });
                }
            }

            let is_private = self.match_token(&TokenType::Priv);
            let is_static = if !is_private { self.match_token(&TokenType::Static) } else { false };

            if self.match_token(&TokenType::Func) {
                // fn name(...) { ... }
                let method = self.parse_graph_method(is_private, false, is_static)?;
                methods.push(method);
            } else if self.match_token(&TokenType::Set) {
                // set name(value) { ... }
                let method = self.parse_graph_method(is_private, true, is_static)?;
                methods.push(method);
            } else if is_static && self.match_token(&TokenType::Func) {
                // static fn - already consumed static
                let method = self.parse_graph_method(is_private, false, true)?;
                methods.push(method);
            } else if is_private {
                // priv without fn/set - error
                return Err(GraphoidError::SyntaxError {
                    message: "Expected 'fn' or 'set' after 'priv' in graph body".to_string(),
                    position: self.peek().position(),
                });
            } else if let TokenType::Identifier(id) = &self.peek().token_type {
                // Property: name: value
                let prop_name = id.clone();
                let prop_pos = self.peek().position();
                self.advance();

                if !self.match_token(&TokenType::Colon) {
                    return Err(GraphoidError::SyntaxError {
                        message: format!("Expected ':' after property name '{}' in graph body", prop_name),
                        position: self.peek().position(),
                    });
                }

                let value = self.expression()?;
                properties.push(GraphProperty {
                    name: prop_name,
                    value,
                    position: prop_pos,
                });
            } else {
                return Err(GraphoidError::SyntaxError {
                    message: format!("Unexpected token in graph body: {:?}", self.peek().token_type),
                    position: self.peek().position(),
                });
            }
        }

        Ok((properties, methods, rules, config))
    }

    /// Parse config entries inside a configure block: readable: :x, writable: [:y, :z]
    fn parse_graph_config(&mut self, config: &mut HashMap<String, Vec<String>>) -> Result<()> {
        loop {
            // Skip newlines
            while self.match_token(&TokenType::Newline) {}

            // Check for end of config block
            if self.check(&TokenType::RightBrace) || self.is_at_end() {
                break;
            }

            // Skip commas between entries
            if self.match_token(&TokenType::Comma) {
                continue;
            }

            // Parse key: value
            let key = if let TokenType::Identifier(id) = &self.peek().token_type {
                let k = id.clone();
                self.advance();
                k
            } else {
                return Err(GraphoidError::SyntaxError {
                    message: format!("Expected config key (readable, writable, accessible), got {:?}", self.peek().token_type),
                    position: self.peek().position(),
                });
            };

            // Validate key
            if !["readable", "writable", "accessible"].contains(&key.as_str()) {
                return Err(GraphoidError::SyntaxError {
                    message: format!("Unknown config key '{}'. Valid keys: readable, writable, accessible", key),
                    position: self.peek().position(),
                });
            }

            if !self.match_token(&TokenType::Colon) {
                return Err(GraphoidError::SyntaxError {
                    message: format!("Expected ':' after config key '{}'", key),
                    position: self.peek().position(),
                });
            }

            // Parse value: either single symbol or list of symbols
            let symbols = self.parse_config_symbols()?;
            if symbols.is_empty() {
                return Err(GraphoidError::SyntaxError {
                    message: format!("Config '{}' requires at least one symbol", key),
                    position: self.peek().position(),
                });
            }

            // Merge with existing (allows multiple configure blocks or repeated keys)
            config.entry(key).or_insert_with(Vec::new).extend(symbols);
        }

        Ok(())
    }

    /// Parse config value: either :symbol or [:sym1, :sym2]
    fn parse_config_symbols(&mut self) -> Result<Vec<String>> {
        let mut symbols = Vec::new();

        if self.match_token(&TokenType::LeftBracket) {
            // List of symbols: [:x, :y, :z]
            loop {
                // Skip newlines
                while self.match_token(&TokenType::Newline) {}

                if self.check(&TokenType::RightBracket) {
                    break;
                }

                if self.match_token(&TokenType::Comma) {
                    continue;
                }

                if let TokenType::Symbol(sym) = &self.peek().token_type {
                    symbols.push(sym.clone());
                    self.advance();
                } else {
                    return Err(GraphoidError::SyntaxError {
                        message: format!("Expected symbol in config list, got {:?}", self.peek().token_type),
                        position: self.peek().position(),
                    });
                }
            }

            if !self.match_token(&TokenType::RightBracket) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected ']' to close config symbol list".to_string(),
                    position: self.peek().position(),
                });
            }
        } else if let TokenType::Symbol(sym) = &self.peek().token_type {
            // Single symbol: :x
            symbols.push(sym.clone());
            self.advance();
        } else {
            return Err(GraphoidError::SyntaxError {
                message: format!("Expected symbol or symbol list, got {:?}", self.peek().token_type),
                position: self.peek().position(),
            });
        }

        Ok(symbols)
    }

    /// Parse a method definition inside a graph body
    fn parse_graph_method(
        &mut self,
        is_private: bool,
        is_setter: bool,
        is_static: bool,
    ) -> Result<GraphMethod> {
        let position = self.peek().position();

        // Get method name
        let name = if let TokenType::Identifier(id) = &self.peek().token_type {
            let n = id.clone();
            self.advance();
            n
        } else {
            return Err(GraphoidError::SyntaxError {
                message: "Expected method name".to_string(),
                position: self.peek().position(),
            });
        };

        // Parse parameters
        if !self.match_token(&TokenType::LeftParen) {
            return Err(GraphoidError::SyntaxError {
                message: format!("Expected '(' after method name '{}'", name),
                position: self.peek().position(),
            });
        }

        // Parse parameter list
        let mut params = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                // Check for variadic parameter (...)
                let is_variadic = self.match_token(&TokenType::DotDotDot);

                let param_name = if let TokenType::Identifier(id) = &self.peek().token_type {
                    let n = id.clone();
                    self.advance();
                    n
                } else {
                    return Err(GraphoidError::SyntaxError {
                        message: "Expected parameter name".to_string(),
                        position: self.peek().position(),
                    });
                };

                // Variadic parameters cannot have default values
                let default_value = if is_variadic {
                    if self.check(&TokenType::Equal) {
                        return Err(GraphoidError::SyntaxError {
                            message: "Variadic parameters cannot have default values".to_string(),
                            position: self.peek().position(),
                        });
                    }
                    None
                } else if self.match_token(&TokenType::Equal) {
                    Some(self.expression()?)
                } else {
                    None
                };

                params.push(Parameter {
                    name: param_name,
                    default_value,
                    is_variadic,
                });

                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }

        if !self.match_token(&TokenType::RightParen) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected ')' after parameters".to_string(),
                position: self.peek().position(),
            });
        }

        // Validate setter constraints
        if is_setter && params.len() != 1 {
            return Err(GraphoidError::SyntaxError {
                message: "Setters must have exactly one parameter".to_string(),
                position: self.peek().position(),
            });
        }

        // Check for guard clause: when expr
        let guard = if self.match_token(&TokenType::When) {
            Some(Box::new(self.expression()?))
        } else {
            None
        };

        // Parse body
        if !self.match_token(&TokenType::LeftBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '{' before method body".to_string(),
                position: self.peek().position(),
            });
        }

        let body = self.block()?;

        // Consume the closing brace of the method body
        if !self.match_token(&TokenType::RightBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '}' after method body".to_string(),
                position: self.peek().position(),
            });
        }

        Ok(GraphMethod {
            name,
            params,
            body,
            is_static,
            is_setter,
            is_private,
            guard,
            position,
        })
    }

    fn configure_statement(&mut self) -> Result<Stmt> {
        use std::collections::HashMap;
        let position = self.previous_position();

        // Expect opening brace for settings
        if !self.match_token(&TokenType::LeftBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '{' after 'configure'".to_string(),
                position: self.peek().position(),
            });
        }

        // Parse settings as key: value pairs
        let mut settings = HashMap::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            // Skip newlines
            while self.match_token(&TokenType::Newline) {}

            // Check if we're at the end of settings
            if self.check(&TokenType::RightBrace) {
                break;
            }

            // Check if this is a standalone symbol (like :unsigned)
            if let TokenType::Symbol(sym) = &self.peek().token_type {
                let symbol_name = sym.clone();
                let pos = self.peek().position();
                self.advance();

                // Standalone symbol: treat it as a flag (symbol_name -> true)
                settings.insert(
                    symbol_name.clone(),
                    Expr::Literal {
                        value: LiteralValue::Symbol(symbol_name),
                        position: pos,
                    }
                );
            } else {
                // Parse key-value pair (key: value)
                // Parse key (must be identifier or precision keyword)
                let key = match &self.peek().token_type {
                    TokenType::Identifier(id) => {
                        let k = id.clone();
                        self.advance();
                        k
                    }
                    TokenType::Precision => {
                        self.advance();
                        "precision".to_string()
                    }
                    _ => {
                        return Err(GraphoidError::SyntaxError {
                            message: format!("Expected configuration key or symbol, got {:?}", self.peek().token_type),
                            position: self.peek().position(),
                        });
                    }
                };

                // Expect colon
                if !self.match_token(&TokenType::Colon) {
                    return Err(GraphoidError::SyntaxError {
                        message: "Expected ':' after configuration key".to_string(),
                        position: self.peek().position(),
                    });
                }

                // Parse value (expression)
                let value = self.expression()?;

                settings.insert(key, value);
            }

            // Optional comma or newline
            if !self.check(&TokenType::RightBrace) {
                if !self.match_token(&TokenType::Comma) {
                    self.match_token(&TokenType::Newline);
                }
            }
        }

        // Expect closing brace
        if !self.match_token(&TokenType::RightBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '}' after configuration settings".to_string(),
                position: self.peek().position(),
            });
        }

        // Check for optional body block
        let body = if self.match_token(&TokenType::LeftBrace) {
            let stmts = self.block()?;
            if !self.match_token(&TokenType::RightBrace) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '}' after configure body".to_string(),
                    position: self.peek().position(),
                });
            }
            Some(stmts)
        } else {
            None
        };

        Ok(Stmt::Configure {
            settings,
            body,
            position,
        })
    }

    fn precision_statement(&mut self) -> Result<Stmt> {
        let position = self.previous_position();

        // Parse precision value (number or :int symbol)
        let places = if let TokenType::Number(n) = &self.peek().token_type {
            let num = *n;
            self.advance();

            // Validate it's a non-negative integer
            if num < 0.0 || num.fract() != 0.0 {
                return Err(GraphoidError::SyntaxError {
                    message: "Precision must be a non-negative integer".to_string(),
                    position: self.previous_position(),
                });
            }

            Some(num as usize)
        } else if let TokenType::Symbol(s) = &self.peek().token_type {
            if s == "int" {
                self.advance();
                Some(0) // :int is equivalent to precision 0
            } else {
                return Err(GraphoidError::SyntaxError {
                    message: format!("Invalid precision specifier :{}, expected :int", s),
                    position: self.peek().position(),
                });
            }
        } else {
            return Err(GraphoidError::SyntaxError {
                message: "Expected number or :int after 'precision'".to_string(),
                position: self.peek().position(),
            });
        };

        // Expect opening brace for body
        if !self.match_token(&TokenType::LeftBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '{' after precision value".to_string(),
                position: self.peek().position(),
            });
        }

        // Parse body block
        let body = self.block()?;

        // Expect closing brace
        if !self.match_token(&TokenType::RightBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '}' after precision body".to_string(),
                position: self.peek().position(),
            });
        }

        Ok(Stmt::Precision {
            places,
            body,
            position,
        })
    }

    fn try_catch_statement(&mut self) -> Result<Stmt> {
        use crate::ast::CatchClause;
        let position = self.previous_position();

        // Expect opening brace for try body
        if !self.match_token(&TokenType::LeftBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '{' after 'try'".to_string(),
                position: self.peek().position(),
            });
        }

        // Parse try body
        let body = self.block()?;

        // Expect closing brace
        if !self.match_token(&TokenType::RightBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '}' after try body".to_string(),
                position: self.peek().position(),
            });
        }

        // Skip newlines before catch/finally
        while self.match_token(&TokenType::Newline) {}

        // Parse catch clauses (zero or more)
        let mut catch_clauses = Vec::new();

        while self.match_token(&TokenType::Catch) {
            let catch_pos = self.previous_position();
            let mut error_type = None;
            let mut variable = None;

            // Check for optional error type
            if let TokenType::Identifier(type_name) = &self.peek().token_type {
                error_type = Some(type_name.clone());
                self.advance();

                // Check for 'as' keyword and variable binding
                if self.match_token(&TokenType::As) {
                    if let TokenType::Identifier(var_name) = &self.peek().token_type {
                        variable = Some(var_name.clone());
                        self.advance();
                    } else {
                        return Err(GraphoidError::SyntaxError {
                            message: "Expected variable name after 'as'".to_string(),
                            position: self.peek().position(),
                        });
                    }
                }
            } else if self.match_token(&TokenType::As) {
                // Catch all with variable binding: catch as e
                if let TokenType::Identifier(var_name) = &self.peek().token_type {
                    variable = Some(var_name.clone());
                    self.advance();
                } else {
                    return Err(GraphoidError::SyntaxError {
                        message: "Expected variable name after 'as'".to_string(),
                        position: self.peek().position(),
                    });
                }
            }

            // Expect opening brace for catch body
            if !self.match_token(&TokenType::LeftBrace) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '{' after catch clause".to_string(),
                    position: self.peek().position(),
                });
            }

            // Parse catch body
            let catch_body = self.block()?;

            // Expect closing brace
            if !self.match_token(&TokenType::RightBrace) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '}' after catch body".to_string(),
                    position: self.peek().position(),
                });
            }

            catch_clauses.push(CatchClause {
                error_type,
                variable,
                body: catch_body,
                position: catch_pos,
            });

            // Skip newlines before next catch or finally
            while self.match_token(&TokenType::Newline) {}
        }

        // Parse optional finally block
        let finally_block = if self.match_token(&TokenType::Finally) {
            if !self.match_token(&TokenType::LeftBrace) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '{' after 'finally'".to_string(),
                    position: self.peek().position(),
                });
            }

            let finally_body = self.block()?;

            if !self.match_token(&TokenType::RightBrace) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '}' after finally body".to_string(),
                    position: self.peek().position(),
                });
            }

            Some(finally_body)
        } else {
            None
        };

        // Validate: must have at least one catch or finally
        if catch_clauses.is_empty() && finally_block.is_none() {
            return Err(GraphoidError::SyntaxError {
                message: "Try statement must have at least one catch or finally block".to_string(),
                position,
            });
        }

        Ok(Stmt::Try {
            body,
            catch_clauses,
            finally_block,
            position,
        })
    }

    fn assignment_or_expression(&mut self) -> Result<Stmt> {
        let position = self.peek().position();

        // Try command-style call first: identifier args { block }
        // This allows syntax like: describe "test" { body }
        if let Some(stmt) = self.try_command_style_call()? {
            return Ok(stmt);
        }

        // Try to parse as assignment
        // Look ahead to see if there's an '=' after an identifier or index expression
        if let TokenType::Identifier(_) = &self.peek().token_type {
            let checkpoint = self.current;

            // Try to parse the left side
            let expr = self.expression()?;

            // Check if followed by '='
            if self.match_token(&TokenType::Equal) {
                // This is an assignment
                let target = match expr {
                    Expr::Variable { name, .. } => AssignmentTarget::Variable(name),
                    Expr::Index { object, index, .. } => {
                        AssignmentTarget::Index { object, index }
                    }
                    Expr::PropertyAccess { object, property, .. } => {
                        AssignmentTarget::Property { object, property }
                    }
                    _ => {
                        return Err(GraphoidError::SyntaxError {
                            message: "Invalid assignment target".to_string(),
                            position,
                        })
                    }
                };

                // Parse value - could be a lambda
                let value = self.lambda_or_expression()?;

                return Ok(Stmt::Assignment {
                    target,
                    value,
                    position,
                });
            } else {
                // Not an assignment, rewind and parse as expression statement
                self.current = checkpoint;
            }
        }

        // Parse as expression statement
        let expr = self.expression()?;
        Ok(Stmt::Expression { expr, position })
    }

    /// Try to parse a command-style function call: identifier args { block }
    /// Returns Some(stmt) if this is a command-style call, None otherwise.
    ///
    /// Command-style calls are statements like:
    ///   describe "test" { ... }
    ///   it "should work" { ... }
    ///   context "when empty" { ... }
    ///
    /// The syntax is: identifier [args...] { block }
    /// - identifier: function name
    /// - args: zero or more arguments (strings, numbers, symbols, etc.)
    /// - { block }: a trailing block that becomes a lambda argument
    fn try_command_style_call(&mut self) -> Result<Option<Stmt>> {
        // Must start with an identifier
        let func_name = match &self.peek().token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => return Ok(None),
        };

        let position = self.peek().position();
        let checkpoint = self.current;
        self.advance(); // consume function name

        // Check what comes next - must be an argument-like token or {
        // NOT: =, (, ., operators
        if !self.is_command_arg_start() {
            self.current = checkpoint;
            return Ok(None);
        }

        // Parse arguments until we hit { or newline
        let mut args = Vec::new();

        while self.is_command_arg_start() && !self.check(&TokenType::LeftBrace) {
            // Parse one argument
            let arg = self.command_arg()?;
            args.push(Argument::Positional { expr: arg, mutable: false });

            // Optional comma between args (but not required)
            self.match_token(&TokenType::Comma);
        }

        // Must have a trailing block for this to be a command-style call
        if !self.check(&TokenType::LeftBrace) {
            self.current = checkpoint;
            return Ok(None);
        }

        // Parse the trailing block as a lambda
        let block_lambda = self.parse_command_block()?;
        args.push(Argument::Positional { expr: block_lambda, mutable: false });

        // Build the call expression
        let callee = Box::new(Expr::Variable {
            name: func_name,
            position: position.clone(),
        });

        let call_expr = Expr::Call {
            callee,
            args,
            position: position.clone(),
        };

        Ok(Some(Stmt::Expression { expr: call_expr, position }))
    }

    /// Check if the current token can start a command argument
    /// NOTE: We intentionally exclude LeftBracket and Identifier to avoid
    /// ambiguity with index access (foo[x]) and regular expressions (foo bar)
    fn is_command_arg_start(&self) -> bool {
        matches!(
            &self.peek().token_type,
            TokenType::String(_)
            | TokenType::Number(_)
            | TokenType::True
            | TokenType::False
            | TokenType::None
            | TokenType::Symbol(_)
            | TokenType::LeftBrace    // trailing block
        )
    }

    /// Parse a single command argument (limited subset of expressions)
    /// Only parses simple literals - no identifiers or complex expressions
    /// to avoid ambiguity with regular expressions
    fn command_arg(&mut self) -> Result<Expr> {
        let position = self.peek().position();

        match &self.peek().token_type.clone() {
            TokenType::String(s) => {
                let value = s.clone();
                self.advance();
                Ok(Expr::Literal {
                    value: LiteralValue::String(value),
                    position,
                })
            }
            TokenType::Number(n) => {
                let value = *n;
                self.advance();
                Ok(Expr::Literal {
                    value: LiteralValue::Number(value),
                    position,
                })
            }
            TokenType::True => {
                self.advance();
                Ok(Expr::Literal {
                    value: LiteralValue::Boolean(true),
                    position,
                })
            }
            TokenType::False => {
                self.advance();
                Ok(Expr::Literal {
                    value: LiteralValue::Boolean(false),
                    position,
                })
            }
            TokenType::None => {
                self.advance();
                Ok(Expr::Literal {
                    value: LiteralValue::None,
                    position,
                })
            }
            TokenType::Symbol(s) => {
                let name = s.clone();
                self.advance();
                Ok(Expr::Literal {
                    value: LiteralValue::Symbol(name),
                    position,
                })
            }
            _ => Err(GraphoidError::SyntaxError {
                message: "Expected command argument".to_string(),
                position,
            }),
        }
    }

    /// Parse a command block: { body } (no parameter list required)
    /// Returns a Lambda expression with empty parameters
    fn parse_command_block(&mut self) -> Result<Expr> {
        let position = self.peek().position();

        if !self.match_token(&TokenType::LeftBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '{' to start block".to_string(),
                position,
            });
        }

        // Check for optional parameter list: |params| or ||
        let params = if self.check(&TokenType::PipePipe) {
            // Empty parameter list: ||
            self.advance();
            vec![]
        } else if self.check(&TokenType::Pipe) {
            // Parameter list: |a, b, c|
            self.advance();
            let mut params = Vec::new();
            if !self.check(&TokenType::Pipe) {
                loop {
                    if let TokenType::Identifier(name) = &self.peek().token_type {
                        params.push(name.clone());
                        self.advance();
                        if !self.match_token(&TokenType::Comma) {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
            if !self.match_token(&TokenType::Pipe) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '|' to close parameter list".to_string(),
                    position: self.peek().position(),
                });
            }
            params
        } else {
            // No parameter list - empty params
            vec![]
        };

        // Parse body statements
        let statements = self.block()?;

        if !self.match_token(&TokenType::RightBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '}' to close block".to_string(),
                position: self.peek().position(),
            });
        }

        // Create lambda with block body
        let body = Box::new(Expr::Block {
            statements,
            position: position.clone(),
        });

        Ok(Expr::Lambda {
            params,
            body,
            position,
        })
    }

    fn block(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            // Skip newlines
            if self.match_token(&TokenType::Newline) {
                continue;
            }

            if self.check(&TokenType::RightBrace) {
                break;
            }

            statements.push(self.statement()?);
        }

        Ok(statements)
    }

    /// Parse a lambda block body: { statements }
    /// Returns a Block expression
    fn parse_lambda_block(&mut self) -> Result<Expr> {
        let position = self.peek().position();

        if !self.match_token(&TokenType::LeftBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '{' to start lambda block body".to_string(),
                position,
            });
        }

        let statements = self.block()?;

        if !self.match_token(&TokenType::RightBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '}' to close lambda block body".to_string(),
                position: self.peek().position(),
            });
        }

        Ok(Expr::Block {
            statements,
            position,
        })
    }

    /// Check if the next token is the start of a trailing block
    /// Trailing blocks start with { followed by | or || for parameters
    /// This distinguishes trailing blocks from control structure bodies
    fn is_trailing_block(&self) -> bool {
        if !self.check(&TokenType::LeftBrace) { return false; }
        if self.current + 1 < self.tokens.len() {
            matches!(
                self.tokens[self.current + 1].token_type,
                TokenType::Pipe | TokenType::PipePipe
            )
        } else { false }
    }

    /// Parse a trailing block: { |params| body } or { body }
    /// Returns a Lambda expression
    fn parse_trailing_block(&mut self) -> Result<Expr> {
        let position = self.peek().position();

        if !self.match_token(&TokenType::LeftBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '{' to start trailing block".to_string(),
                position,
            });
        }

        // Check for parameter list: |params| or ||
        let params = if self.check(&TokenType::PipePipe) {
            // Empty parameter list: ||
            self.advance(); // consume ||
            vec![]
        } else if self.check(&TokenType::Pipe) {
            self.advance(); // consume |

            let mut params = Vec::new();
            if !self.check(&TokenType::Pipe) {
                loop {
                    // Allow identifiers and type keywords as parameter names
                    let param_name = match &self.peek().token_type {
                        TokenType::Identifier(name) => name.clone(),
                        // Allow type keywords as parameter names in blocks
                        TokenType::NumType => "num".to_string(),
                        TokenType::StringType => "string".to_string(),
                        TokenType::BoolType => "bool".to_string(),
                        TokenType::ListType => "list".to_string(),
                        TokenType::MapType => "map".to_string(),
                        TokenType::GraphType => "graph".to_string(),
                        TokenType::TreeType => "tree".to_string(),
                        _ => {
                            return Err(GraphoidError::SyntaxError {
                                message: format!("Expected parameter name, got {:?}", self.peek().token_type),
                                position: self.peek().position(),
                            });
                        }
                    };

                    params.push(param_name);
                    self.advance();

                    if !self.match_token(&TokenType::Comma) {
                        break;
                    }
                }
            }

            if !self.match_token(&TokenType::Pipe) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '|' after block parameters".to_string(),
                    position: self.peek().position(),
                });
            }

            params
        } else {
            vec![] // No parameters
        };

        // Parse block body
        let statements = self.block()?;

        if !self.match_token(&TokenType::RightBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '}' to close trailing block".to_string(),
                position: self.peek().position(),
            });
        }

        // Create lambda body: if single expression statement, use expression directly
        // Otherwise, use block
        let body = if statements.len() == 1 {
            // Check if it's a single expression statement
            if let Stmt::Expression { expr, .. } = &statements[0] {
                // Single expression: treat as expression lambda
                Box::new(expr.clone())
            } else {
                // Other statement types: use block
                Box::new(Expr::Block {
                    statements,
                    position: position.clone(),
                })
            }
        } else {
            // Multiple statements: use block
            Box::new(Expr::Block {
                statements,
                position: position.clone(),
            })
        };

        Ok(Expr::Lambda {
            params,
            body,
            position,
        })
    }

    // Expression parsing with precedence climbing
    fn expression(&mut self) -> Result<Expr> {
        // Note: Lambda parsing happens in primary() via Pipe token
        self.conditional_expression()
    }

    fn conditional_expression(&mut self) -> Result<Expr> {
        // Parse the base expression
        let expr = self.or_expression()?;

        // Check for inline conditional (if-then-else or suffix if/unless)
        if self.check(&TokenType::If) || self.check(&TokenType::Unless) {
            let is_unless = self.check(&TokenType::Unless);
            let if_position = self.peek().position();
            self.advance(); // consume 'if' or 'unless'

            // Parse condition
            let condition = self.or_expression()?;

            // Check for 'else' (if-then-else form)
            if self.match_token(&TokenType::Else) {
                if is_unless {
                    return Err(GraphoidError::SyntaxError {
                        message: "'unless' cannot be used with 'else'".to_string(),
                        position: if_position,
                    });
                }
                // Parse else expression
                let else_expr = self.or_expression()?;

                // For if-then-else: `then_expr if condition else else_expr`
                // expr is the then_expr
                return Ok(Expr::Conditional {
                    condition: Box::new(condition),
                    then_expr: Box::new(expr),
                    else_expr: Some(Box::new(else_expr)),
                    is_unless: false,
                    position: if_position,
                });
            } else {
                // Suffix if/unless form (no else)
                // Important: Suffix forms should NOT have a block after the condition
                // If we see `{`, this is likely a statement-level if, not a suffix conditional
                if self.check(&TokenType::LeftBrace) {
                    return Err(GraphoidError::SyntaxError {
                        message: "Suffix conditional (value if condition) cannot have a block. Did you mean to use a statement-level if?".to_string(),
                        position: self.peek().position(),
                    });
                }

                return Ok(Expr::Conditional {
                    condition: Box::new(condition),
                    then_expr: Box::new(expr),
                    else_expr: None,
                    is_unless,
                    position: if_position,
                });
            }
        }

        Ok(expr)
    }

    fn or_expression(&mut self) -> Result<Expr> {
        let mut expr = self.and_expression()?;

        while self.match_token(&TokenType::Or) || self.match_token(&TokenType::PipePipe) {
            let position = self.previous_position();
            let right = self.and_expression()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Or,
                right: Box::new(right),
                position,
            };
        }

        Ok(expr)
    }

    fn and_expression(&mut self) -> Result<Expr> {
        let mut expr = self.equality()?;

        while self.match_token(&TokenType::And) || self.match_token(&TokenType::AmpersandAmpersand)
        {
            let position = self.previous_position();
            let right = self.equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::And,
                right: Box::new(right),
                position,
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        while self.match_token(&TokenType::EqualEqual)
            || self.match_token(&TokenType::BangEqual)
            || self.match_token(&TokenType::RegexMatch)
            || self.match_token(&TokenType::RegexNoMatch)
            || self.match_token(&TokenType::DotEqualEqual)
            || self.match_token(&TokenType::DotBangEqual)
        {
            let op = match &self.previous().token_type {
                TokenType::EqualEqual => BinaryOp::Equal,
                TokenType::BangEqual => BinaryOp::NotEqual,
                TokenType::RegexMatch => BinaryOp::RegexMatch,
                TokenType::RegexNoMatch => BinaryOp::RegexNoMatch,
                TokenType::DotEqualEqual => BinaryOp::DotEqual,
                TokenType::DotBangEqual => BinaryOp::DotNotEqual,
                _ => unreachable!(),
            };
            let position = self.previous_position();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                position,
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.bitwise_or()?;

        while self.match_token(&TokenType::Less)
            || self.match_token(&TokenType::LessEqual)
            || self.match_token(&TokenType::Greater)
            || self.match_token(&TokenType::GreaterEqual)
            || self.match_token(&TokenType::DotLess)
            || self.match_token(&TokenType::DotLessEqual)
            || self.match_token(&TokenType::DotGreater)
            || self.match_token(&TokenType::DotGreaterEqual)
        {
            let op = match &self.previous().token_type {
                TokenType::Less => BinaryOp::Less,
                TokenType::LessEqual => BinaryOp::LessEqual,
                TokenType::Greater => BinaryOp::Greater,
                TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                TokenType::DotLess => BinaryOp::DotLess,
                TokenType::DotLessEqual => BinaryOp::DotLessEqual,
                TokenType::DotGreater => BinaryOp::DotGreater,
                TokenType::DotGreaterEqual => BinaryOp::DotGreaterEqual,
                _ => unreachable!(),
            };
            let position = self.previous_position();
            let right = self.bitwise_or()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                position,
            };
        }

        Ok(expr)
    }

    // Phase 13: Bitwise OR (|) - Lower precedence than XOR
    fn bitwise_or(&mut self) -> Result<Expr> {
        let mut expr = self.bitwise_xor()?;

        while self.match_token(&TokenType::Pipe) {
            let position = self.previous_position();
            let right = self.bitwise_xor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::BitwiseOr,
                right: Box::new(right),
                position,
            };
        }

        Ok(expr)
    }

    // Phase 13: Bitwise XOR (^) - Between OR and AND
    fn bitwise_xor(&mut self) -> Result<Expr> {
        let mut expr = self.bitwise_and()?;

        while self.match_token(&TokenType::Caret) || self.match_token(&TokenType::DotCaret) {
            let op = match &self.previous().token_type {
                TokenType::Caret => BinaryOp::BitwiseXor,
                TokenType::DotCaret => BinaryOp::DotXor,
                _ => unreachable!(),
            };
            let position = self.previous_position();
            let right = self.bitwise_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                position,
            };
        }

        Ok(expr)
    }

    // Phase 13: Bitwise AND (&) - Higher precedence than XOR
    fn bitwise_and(&mut self) -> Result<Expr> {
        let mut expr = self.shift()?;

        while self.match_token(&TokenType::Ampersand) {
            let position = self.previous_position();
            let right = self.shift()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::BitwiseAnd,
                right: Box::new(right),
                position,
            };
        }

        Ok(expr)
    }

    // Phase 13: Bit shifts (<<, >>) - Between bitwise AND and addition
    fn shift(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;

        while self.match_token(&TokenType::LeftShift) || self.match_token(&TokenType::RightShift) {
            let op = match &self.previous().token_type {
                TokenType::LeftShift => BinaryOp::LeftShift,
                TokenType::RightShift => BinaryOp::RightShift,
                _ => unreachable!(),
            };
            let position = self.previous_position();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                position,
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;

        while self.match_token(&TokenType::Plus)
            || self.match_token(&TokenType::Minus)
            || self.match_token(&TokenType::DotPlus)
            || self.match_token(&TokenType::DotMinus)
        {
            let op = match &self.previous().token_type {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Subtract,
                TokenType::DotPlus => BinaryOp::DotAdd,
                TokenType::DotMinus => BinaryOp::DotSubtract,
                _ => unreachable!(),
            };
            let position = self.previous_position();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                position,
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.power()?;

        while self.match_token(&TokenType::Star)
            || self.match_token(&TokenType::Slash)
            || self.match_token(&TokenType::SlashSlash)
            || self.match_token(&TokenType::Percent)
            || self.match_token(&TokenType::DotStar)
            || self.match_token(&TokenType::DotSlash)
            || self.match_token(&TokenType::DotSlashSlash)
            || self.match_token(&TokenType::DotPercent)
        {
            let op = match &self.previous().token_type {
                TokenType::Star => BinaryOp::Multiply,
                TokenType::Slash => BinaryOp::Divide,
                TokenType::SlashSlash => BinaryOp::IntDiv,
                TokenType::Percent => BinaryOp::Modulo,
                TokenType::DotStar => BinaryOp::DotMultiply,
                TokenType::DotSlash => BinaryOp::DotDivide,
                TokenType::DotSlashSlash => BinaryOp::DotIntDiv,
                TokenType::DotPercent => BinaryOp::DotModulo,
                _ => unreachable!(),
            };
            let position = self.previous_position();
            let right = self.power()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                position,
            };
        }

        Ok(expr)
    }

    // Phase 13: Power (**) - Right-associative!
    fn power(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;

        // Check for ** or .** (element-wise power)
        if self.match_token(&TokenType::DoubleStar) || self.match_token(&TokenType::DotDoubleStar) {
            let op = match &self.previous().token_type {
                TokenType::DoubleStar => BinaryOp::Power,
                TokenType::DotDoubleStar => BinaryOp::DotPower,
                _ => unreachable!(),
            };
            let position = self.previous_position();
            // Right-associative: recursively call power() for right side
            // This makes 2 ** 3 ** 2 = 2 ** (3 ** 2) = 2 ** 9 = 512
            let right = self.power()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                position,
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.match_token(&TokenType::Minus) {
            let position = self.previous_position();
            let operand = self.unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Negate,
                operand: Box::new(operand),
                position,
            });
        }

        if self.match_token(&TokenType::Not) {
            let position = self.previous_position();
            let operand = self.unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Not,
                operand: Box::new(operand),
                position,
            });
        }

        // Phase 13: Bitwise NOT (~)
        if self.match_token(&TokenType::Tilde) {
            let position = self.previous_position();
            let operand = self.unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::BitwiseNot,
                operand: Box::new(operand),
                position,
            });
        }

        self.postfix()
    }

    fn postfix(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;

        loop {
            // Save position before skipping newlines
            let checkpoint = self.current;

            // Skip newlines to allow method chaining across lines
            while self.match_token(&TokenType::Newline) {}

            // Check if we have a postfix operator
            // LeftBrace is only a postfix for CLG instantiation (e.g., Point { x: 10 })
            let has_postfix = self.check(&TokenType::LeftParen)
                || self.check(&TokenType::LeftBracket)
                || self.check(&TokenType::Dot)
                || self.check(&TokenType::LeftBrace);

            // If we skipped newlines but don't have a postfix operator, rewind
            // This prevents suffix conditionals from crossing line boundaries
            if checkpoint != self.current && !has_postfix {
                self.current = checkpoint;
                break;
            }

            // CLG instantiation: ClassName { } or ClassName { prop: value, ... }
            // Only valid after a Variable expression (the class name)
            // Must distinguish from control structure blocks like `for x in list { ... }`
            // Also disabled when parsing graph parent expressions (graph Child from Parent { })
            if self.check(&TokenType::LeftBrace) && !self.disable_brace_instantiation {
                if let Expr::Variable { .. } = &expr {
                    // Look ahead to see if this is instantiation (empty or key:value pairs)
                    // vs a block (statements like `i = 0`)
                    let brace_pos = self.current;
                    self.advance(); // consume '{'

                    // Skip newlines
                    while self.match_token(&TokenType::Newline) {}

                    // Check what follows the '{'
                    let is_instantiation = if self.check(&TokenType::RightBrace) {
                        // Empty braces: ClassName {}
                        true
                    } else if let TokenType::Identifier(_) = &self.peek().token_type {
                        // Look ahead: is the next token after identifier a ':'?
                        // If so, this is instantiation. If '=' or other, it's a block.
                        self.check_next(&TokenType::Colon)
                    } else {
                        // Not an identifier - not instantiation syntax
                        false
                    };

                    if !is_instantiation {
                        // Not instantiation - rewind and let block parsing handle it
                        self.current = brace_pos;
                        break;
                    }

                    // This IS instantiation - parse property overrides
                    let position = expr.position().clone();
                    let mut overrides = Vec::new();

                    if !self.check(&TokenType::RightBrace) {
                        loop {
                            // Skip newlines before each entry
                            while self.match_token(&TokenType::Newline) {}

                            // Parse key (must be identifier)
                            let key = if let TokenType::Identifier(id) = &self.peek().token_type {
                                let k = id.clone();
                                self.advance();
                                k
                            } else {
                                return Err(GraphoidError::SyntaxError {
                                    message: "Expected property name in instantiation".to_string(),
                                    position: self.peek().position(),
                                });
                            };

                            // Expect ':'
                            if !self.match_token(&TokenType::Colon) {
                                return Err(GraphoidError::SyntaxError {
                                    message: "Expected ':' after property name".to_string(),
                                    position: self.peek().position(),
                                });
                            }

                            // Parse value
                            let value = self.expression()?;
                            overrides.push((key, value));

                            // Skip newlines before comma/closing brace
                            while self.match_token(&TokenType::Newline) {}

                            if !self.match_token(&TokenType::Comma) {
                                break;
                            }
                        }
                    }

                    // Skip newlines before closing brace
                    while self.match_token(&TokenType::Newline) {}

                    if !self.match_token(&TokenType::RightBrace) {
                        return Err(GraphoidError::SyntaxError {
                            message: "Expected '}' after instantiation properties".to_string(),
                            position: self.peek().position(),
                        });
                    }

                    expr = Expr::Instantiate {
                        class_name: Box::new(expr),
                        overrides,
                        position,
                    };
                    continue;
                }
                // If not a Variable, don't treat {} as instantiation - fall through
                break;
            }

            if self.match_token(&TokenType::LeftParen) {
                // Function call
                let position = expr.position().clone();
                let mut args = self.arguments()?;
                if !self.match_token(&TokenType::RightParen) {
                    return Err(GraphoidError::SyntaxError {
                        message: "Expected ')' after arguments".to_string(),
                        position: self.peek().position(),
                    });
                }

                // Check for trailing block: function(args) { |params| body }
                if self.is_trailing_block() {
                    let block_lambda = self.parse_trailing_block()?;
                    args.push(Argument::Positional { expr: block_lambda, mutable: false });
                }

                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                    position,
                };
            } else if self.match_token(&TokenType::LeftBracket) {
                // Index access
                let position = expr.position().clone();
                let index = self.expression()?;
                if !self.match_token(&TokenType::RightBracket) {
                    return Err(GraphoidError::SyntaxError {
                        message: "Expected ']' after index".to_string(),
                        position: self.peek().position(),
                    });
                }
                expr = Expr::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                    position,
                };
            } else if self.match_token(&TokenType::Dot) {
                // Method call or property access
                let position = expr.position().clone();

                // Method name can be an identifier OR a keyword (like "map", "filter", etc.)
                // We use the lexeme directly to allow keywords as method names
                let method = match &self.peek().token_type {
                    TokenType::Identifier(id) => {
                        let mut m = id.clone();
                        self.advance();

                        // Check for ! suffix (mutating method convention)
                        if self.match_token(&TokenType::Bang) {
                            m.push('!');
                        }

                        m
                    }
                    // Allow keywords as method names by using their lexeme
                    _ => {
                        let mut lexeme = self.peek().lexeme.clone();
                        // Check if it's a valid method name (alphanumeric + underscore)
                        if lexeme.chars().all(|c| c.is_alphabetic() || c == '_') && !lexeme.is_empty() {
                            self.advance();

                            // Check for ! suffix (mutating method convention)
                            if self.match_token(&TokenType::Bang) {
                                lexeme.push('!');
                            }

                            lexeme
                        } else {
                            return Err(GraphoidError::SyntaxError {
                                message: "Expected method name after '.'".to_string(),
                                position: self.peek().position(),
                            });
                        }
                    }
                };

                // Check for command-style method call: obj.method "arg" { block }
                // This must come before the LeftParen check
                if self.is_command_arg_start() && !self.check(&TokenType::LeftBrace) {
                    // Parse command arguments (strings, numbers, symbols - not the trailing block yet)
                    let mut args = Vec::new();
                    while self.is_command_arg_start() && !self.check(&TokenType::LeftBrace) {
                        let arg = self.command_arg()?;
                        args.push(Argument::Positional { expr: arg, mutable: false });
                    }

                    // Must have a trailing block for command-style method calls
                    if !self.check(&TokenType::LeftBrace) {
                        return Err(GraphoidError::SyntaxError {
                            message: "Expected '{' block after command-style method arguments".to_string(),
                            position: self.peek().position(),
                        });
                    }

                    let block_lambda = self.parse_command_block()?;
                    args.push(Argument::Positional { expr: block_lambda, mutable: false });

                    expr = Expr::MethodCall {
                        object: Box::new(expr),
                        method,
                        args,
                        position,
                    };
                } else if self.match_token(&TokenType::LeftParen) {
                    // Regular method call with parentheses (including explicit syntax for g.match())
                    let mut args = self.arguments()?;
                    if !self.match_token(&TokenType::RightParen) {
                        return Err(GraphoidError::SyntaxError {
                            message: "Expected ')' after method arguments".to_string(),
                                position: self.peek().position(),
                            });
                        }

                        // Check for trailing block: method(args) { |params| body }
                        if self.is_trailing_block() {
                            let block_lambda = self.parse_trailing_block()?;
                            args.push(Argument::Positional { expr: block_lambda, mutable: false });
                        }

                    expr = Expr::MethodCall {
                        object: Box::new(expr),
                        method,
                        args,
                        position,
                    };
                } else {
                    // No parentheses - check for trailing block first
                    // If there's a trailing block, treat as method call: list.map { |x| x * 2 }
                    // Otherwise, treat as property access: self.name
                    if self.is_trailing_block() {
                        let block_lambda = self.parse_trailing_block()?;
                        let args = vec![Argument::Positional { expr: block_lambda, mutable: false }];
                        expr = Expr::MethodCall {
                            object: Box::new(expr),
                            method,
                            args,
                            position,
                        };
                    } else if self.check(&TokenType::LeftBrace) {
                        // Command-style block without other args: obj.method { block }
                        let block_lambda = self.parse_command_block()?;
                        let args = vec![Argument::Positional { expr: block_lambda, mutable: false }];
                        expr = Expr::MethodCall {
                            object: Box::new(expr),
                            method,
                            args,
                            position,
                        };
                    } else {
                        // Property access: object.property
                        expr = Expr::PropertyAccess {
                            object: Box::new(expr),
                            property: method,
                            position,
                        };
                    }
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn arguments(&mut self) -> Result<Vec<Argument>> {
        use crate::ast::Argument;
        let mut args = Vec::new();

        // Skip leading newlines
        self.skip_newlines();

        if !self.check(&TokenType::RightParen) {
            loop {
                // Skip newlines before each argument
                self.skip_newlines();

                // Check for named argument syntax: name: value
                if let TokenType::Identifier(name) = &self.peek().token_type {
                    // Look ahead to see if this is a named argument
                    if self.current + 1 < self.tokens.len() {
                        if let TokenType::Colon = self.tokens[self.current + 1].token_type {
                            // Named argument
                            let param_name = name.clone();
                            self.advance(); // consume identifier
                            self.advance(); // consume colon
                            let value = self.lambda_or_expression()?;

                            // Check for mutable marker: name: value!
                            let mutable = self.match_token(&TokenType::Bang);

                            args.push(Argument::Named {
                                name: param_name,
                                value,
                                mutable,
                            });

                            // Skip newlines before comma check
                            self.skip_newlines();

                            if !self.match_token(&TokenType::Comma) {
                                break;
                            }
                            continue;
                        }
                    }
                }

                // Positional argument
                let expr = self.lambda_or_expression()?;

                // Check for mutable marker: expr!
                // This enables write-back semantics for the argument (e.g., self!)
                let mutable = self.match_token(&TokenType::Bang);

                args.push(Argument::Positional { expr, mutable });

                // Skip newlines before comma check
                self.skip_newlines();

                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }

        // Skip trailing newlines before closing paren
        self.skip_newlines();

        Ok(args)
    }

    /// Try to parse a lambda, otherwise parse a regular expression
    fn lambda_or_expression(&mut self) -> Result<Expr> {
        let position = self.peek().position();

        // Case 1: Single parameter lambda: x => expr
        if let TokenType::Identifier(param_name) = &self.peek().token_type {
            // Look ahead to see if this is a lambda
            if self.current + 1 < self.tokens.len() {
                if let TokenType::Arrow = self.tokens[self.current + 1].token_type {
                    // This is a single-param lambda!
                    let param = param_name.clone();
                    self.advance(); // consume param
                    self.advance(); // consume =>

                    // Check if body is a block or single expression
                    let body = if self.check(&TokenType::LeftBrace) {
                        // Block body: x => { statements }
                        Box::new(self.parse_lambda_block()?)
                    } else {
                        // Expression body: x => expr
                        Box::new(self.or_expression()?)
                    };

                    return Ok(Expr::Lambda {
                        params: vec![param],
                        body,
                        position,
                    });
                }
            }
        }

        // Case 2: Multi-param or zero-param lambda: (a, b) => expr  OR  () => expr
        if self.check(&TokenType::LeftParen) {
            let paren_checkpoint = self.current;
            self.advance(); // consume '('

            let mut params = Vec::new();
            let mut could_be_lambda = true;

            // Parse parameter list
            if !self.check(&TokenType::RightParen) {
                loop {
                    if let TokenType::Identifier(param_name) = &self.peek().token_type {
                        params.push(param_name.clone());
                        self.advance();

                        if !self.match_token(&TokenType::Comma) {
                            break;
                        }
                    } else {
                        // Not a valid param, not a lambda
                        could_be_lambda = false;
                        break;
                    }
                }
            }

            if could_be_lambda && self.match_token(&TokenType::RightParen) && self.check(&TokenType::Arrow) {
                // This is a lambda!
                self.advance(); // consume =>

                // Check if body is a block or single expression
                let body = if self.check(&TokenType::LeftBrace) {
                    // Block body: (x, y) => { statements }
                    Box::new(self.parse_lambda_block()?)
                } else {
                    // Expression body: (x, y) => expr
                    Box::new(self.or_expression()?)
                };

                return Ok(Expr::Lambda {
                    params,
                    body,
                    position,
                });
            }

            // Not a lambda, rewind and parse as expression
            self.current = paren_checkpoint;
        }

        // Case 3: Block lambda: { || ... } or { |x, y| ... }
        // This allows block lambdas to be passed as function arguments
        if self.check(&TokenType::LeftBrace) {
            // Look ahead to see if this is a block lambda (has || or |)
            if self.current + 1 < self.tokens.len() {
                match &self.tokens[self.current + 1].token_type {
                    TokenType::PipePipe | TokenType::Pipe => {
                        // This is a block lambda!
                        return self.parse_command_block();
                    }
                    _ => {}
                }
            }
        }

        // Not a lambda, parse as regular expression
        self.expression()
    }

    fn primary(&mut self) -> Result<Expr> {
        let position = self.peek().position();

        // Raise expressions
        if self.match_token(&TokenType::Raise) {
            let error_expr = Box::new(self.expression()?);
            return Ok(Expr::Raise {
                error: error_expr,
                position,
            });
        }

        // Match expressions
        if self.match_token(&TokenType::Match) {
            return self.match_expression(position);
        }

        // Super method calls: super.method(args)
        if self.match_token(&TokenType::Super) {
            if !self.match_token(&TokenType::Dot) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '.' after 'super'".to_string(),
                    position: self.peek().position(),
                });
            }

            // Get method name
            let method = if let TokenType::Identifier(id) = &self.peek().token_type {
                let name = id.clone();
                self.advance();
                name
            } else {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected method name after 'super.'".to_string(),
                    position: self.peek().position(),
                });
            };

            // Parse arguments (required for super calls)
            if !self.match_token(&TokenType::LeftParen) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '(' after super method name".to_string(),
                    position: self.peek().position(),
                });
            }

            let args = self.arguments()?;

            if !self.match_token(&TokenType::RightParen) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected ')' after super method arguments".to_string(),
                    position: self.peek().position(),
                });
            }

            return Ok(Expr::SuperMethodCall {
                method,
                args,
                position,
            });
        }

        // Numbers
        if let TokenType::Number(n) = self.peek().token_type {
            self.advance();
            return Ok(Expr::Literal {
                value: LiteralValue::Number(n),
                position,
            });
        }

        // Strings
        if let TokenType::String(s) = &self.peek().token_type {
            let str_val = s.clone();
            self.advance();
            return Ok(Expr::Literal {
                value: LiteralValue::String(str_val),
                position,
            });
        }

        // Symbols
        if let TokenType::Symbol(s) = &self.peek().token_type {
            let sym_val = s.clone();
            self.advance();
            return Ok(Expr::Literal {
                value: LiteralValue::Symbol(sym_val),
                position,
            });
        }

        // Booleans
        if self.match_token(&TokenType::True) {
            return Ok(Expr::Literal {
                value: LiteralValue::Boolean(true),
                position,
            });
        }

        if self.match_token(&TokenType::False) {
            return Ok(Expr::Literal {
                value: LiteralValue::Boolean(false),
                position,
            });
        }

        // None
        if self.match_token(&TokenType::None) {
            return Ok(Expr::Literal {
                value: LiteralValue::None,
                position,
            });
        }

        // Identifiers (variables)
        if let TokenType::Identifier(id) = &self.peek().token_type {
            let name = id.clone();
            self.advance();
            return Ok(Expr::Variable { name, position });
        }

        // Lists with optional type constraint: list<num>[] or just []
        // OR static method call on list type: list.generate()
        if self.match_token(&TokenType::ListType) {
            // Check if this is a static method call (list.method())
            if self.check(&TokenType::Dot) {
                // Treat "list" as a variable/type identifier for static method calls
                return Ok(Expr::Variable {
                    name: "list".to_string(),
                    position,
                });
            }

            // Parse optional type parameter: list<type>
            if self.match_token(&TokenType::Less) {
                // Parse the type constraint (but we'll ignore it for now - runtime checks only)
                // Accept any identifier or type keyword as the type parameter
                let is_valid_type = matches!(
                    self.peek().token_type,
                    TokenType::Identifier(_) | TokenType::NumType | TokenType::StringType |
                    TokenType::BoolType | TokenType::ListType | TokenType::MapType |
                    TokenType::GraphType | TokenType::TreeType
                );

                if !is_valid_type {
                    return Err(GraphoidError::SyntaxError {
                        message: "Expected type name after 'list<'".to_string(),
                        position: self.peek().position(),
                    });
                }
                self.advance(); // consume type name

                if !self.match_token(&TokenType::Greater) {
                    return Err(GraphoidError::SyntaxError {
                        message: "Expected '>' after type parameter".to_string(),
                        position: self.peek().position(),
                    });
                }
            }

            // Now parse the list literal
            if !self.match_token(&TokenType::LeftBracket) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '[' after 'list' or 'list<type>'".to_string(),
                    position: self.peek().position(),
                });
            }

            // Skip leading newlines
            self.skip_newlines();

            let mut elements = Vec::new();
            if !self.check(&TokenType::RightBracket) {
                loop {
                    // Skip newlines before each element
                    self.skip_newlines();

                    elements.push(self.expression()?);

                    // Skip newlines before comma check
                    self.skip_newlines();

                    if !self.match_token(&TokenType::Comma) {
                        break;
                    }
                }
            }

            // Skip trailing newlines before closing bracket
            self.skip_newlines();

            if !self.match_token(&TokenType::RightBracket) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected ']' after list elements".to_string(),
                    position: self.peek().position(),
                });
            }

            return Ok(Expr::List { elements, position });
        }

        // Lists without type: []
        if self.match_token(&TokenType::LeftBracket) {
            let mut elements = Vec::new();

            // Skip leading newlines
            self.skip_newlines();

            if !self.check(&TokenType::RightBracket) {
                loop {
                    // Skip newlines before each element
                    self.skip_newlines();

                    elements.push(self.expression()?);

                    // Skip newlines before comma check
                    self.skip_newlines();

                    if !self.match_token(&TokenType::Comma) {
                        break;
                    }
                }
            }

            // Skip trailing newlines before closing bracket
            self.skip_newlines();

            if !self.match_token(&TokenType::RightBracket) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected ']' after list elements".to_string(),
                    position: self.peek().position(),
                });
            }

            return Ok(Expr::List { elements, position });
        }

        // String static method calls: string.generate()
        if self.match_token(&TokenType::StringType) {
            // Check if this is a static method call (string.method())
            if self.check(&TokenType::Dot) {
                // Treat "string" as a variable/type identifier for static method calls
                return Ok(Expr::Variable {
                    name: "string".to_string(),
                    position,
                });
            }

            // If not a static method call, this is an error - string literals use quotes
            return Err(GraphoidError::SyntaxError {
                message: "Unexpected 'string' keyword. Did you mean a string literal (\"text\") or string.generate()?".to_string(),
                position,
            });
        }

        // Graphs: graph { type: :directed } or graph from Parent {}
        if self.match_token(&TokenType::GraphType) {
            // Check for inheritance: graph from Parent {}
            let parent = if self.match_token(&TokenType::From) {
                // Parse the parent expression (e.g., ParentGraph, module.Graph)
                let parent_expr = self.expression()?;
                Some(Box::new(parent_expr))
            } else {
                None
            };

            if !self.match_token(&TokenType::LeftBrace) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '{' after 'graph' or parent expression".to_string(),
                    position: self.peek().position(),
                });
            }

            let config = self.parse_config_entries()?;

            if !self.match_token(&TokenType::RightBrace) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '}' after graph config".to_string(),
                    position: self.peek().position(),
                });
            }

            return Ok(Expr::Graph { config, parent, position });
        }

        // Trees: tree {} desugars to graph{}.with_ruleset(:tree)
        // This implements Option A: trees are graphs with rules
        if self.match_token(&TokenType::TreeType) {
            if !self.match_token(&TokenType::LeftBrace) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '{' after 'tree'".to_string(),
                    position: self.peek().position(),
                });
            }

            let config = self.parse_config_entries()?;

            if !self.match_token(&TokenType::RightBrace) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '}' after tree config".to_string(),
                    position: self.peek().position(),
                });
            }

            // Desugar: tree{} → graph{}.with_ruleset(:tree)
            let graph_expr = Expr::Graph {
                config,
                parent: None,
                position: position.clone()
            };

            let symbol_expr = Expr::Literal {
                value: crate::ast::LiteralValue::Symbol("tree".to_string()),
                position: position.clone(),
            };

            return Ok(Expr::MethodCall {
                object: Box::new(graph_expr),
                method: "with_ruleset".to_string(),
                args: vec![crate::ast::Argument::Positional { expr: symbol_expr, mutable: false }],
                position,
            });
        }

        // Maps with optional type constraint: hash<type>{} (note: hash is tokenized as MapType)
        if self.match_token(&TokenType::MapType) {
            // Parse optional type parameter: hash<type>
            if self.match_token(&TokenType::Less) {
                // Parse the type constraint (but we'll ignore it for now - runtime checks only)
                // Accept any identifier or type keyword as the type parameter
                let is_valid_type = matches!(
                    self.peek().token_type,
                    TokenType::Identifier(_) | TokenType::NumType | TokenType::StringType |
                    TokenType::BoolType | TokenType::ListType | TokenType::MapType |
                    TokenType::GraphType | TokenType::TreeType
                );

                if !is_valid_type {
                    return Err(GraphoidError::SyntaxError {
                        message: "Expected type name after 'hash<'".to_string(),
                        position: self.peek().position(),
                    });
                }
                self.advance(); // consume type name

                if !self.match_token(&TokenType::Greater) {
                    return Err(GraphoidError::SyntaxError {
                        message: "Expected '>' after type parameter".to_string(),
                        position: self.peek().position(),
                    });
                }
            }

            // Now parse the map literal
            if !self.match_token(&TokenType::LeftBrace) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '{' after 'hash' or 'hash<type>'".to_string(),
                    position: self.peek().position(),
                });
            }

            let mut entries = Vec::new();
            // Skip newlines after opening brace
            while self.match_token(&TokenType::Newline) {}

            if !self.check(&TokenType::RightBrace) {
                loop {
                    // Skip newlines before each entry
                    while self.match_token(&TokenType::Newline) {}

                    // Parse key (must be string or identifier)
                    let key = if let TokenType::String(s) = &self.peek().token_type {
                        let k = s.clone();
                        self.advance();
                        k
                    } else if let TokenType::Identifier(id) = &self.peek().token_type {
                        let k = id.clone();
                        self.advance();
                        k
                    } else {
                        return Err(GraphoidError::SyntaxError {
                            message: "Expected string or identifier as map key".to_string(),
                            position: self.peek().position(),
                        });
                    };

                    // Expect ':'
                    if !self.match_token(&TokenType::Colon) {
                        return Err(GraphoidError::SyntaxError {
                            message: "Expected ':' after map key".to_string(),
                            position: self.peek().position(),
                        });
                    }

                    // Parse value - support lambdas
                    let value = self.lambda_or_expression()?;
                    entries.push((key, value));

                    if !self.match_token(&TokenType::Comma) {
                        break;
                    }
                }
            }

            // Skip newlines before closing brace
            while self.match_token(&TokenType::Newline) {}

            if !self.match_token(&TokenType::RightBrace) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '}' after map entries".to_string(),
                    position: self.peek().position(),
                });
            }

            return Ok(Expr::Map { entries, position });
        }

        // If expressions with block syntax: if condition { expr } else { expr }
        // This must come BEFORE map literal parsing to avoid confusion
        if self.match_token(&TokenType::If) || self.match_token(&TokenType::Unless) {
            let is_unless = self.previous().token_type == TokenType::Unless;
            let if_position = self.previous_position();

            // Parse condition
            let condition = self.or_expression()?;

            // Check for block syntax
            if self.match_token(&TokenType::LeftBrace) {
                // Parse then block
                let then_statements = self.block()?;

                if !self.match_token(&TokenType::RightBrace) {
                    return Err(GraphoidError::SyntaxError {
                        message: "Expected '}' after if block".to_string(),
                        position: self.peek().position(),
                    });
                }

                let then_expr = Expr::Block {
                    statements: then_statements,
                    position: if_position.clone(),
                };

                // Check for else
                let else_expr = if self.match_token(&TokenType::Else) {
                    if is_unless {
                        return Err(GraphoidError::SyntaxError {
                            message: "'unless' cannot be used with 'else'".to_string(),
                            position: if_position,
                        });
                    }

                    // Else must also be a block
                    if !self.match_token(&TokenType::LeftBrace) {
                        return Err(GraphoidError::SyntaxError {
                            message: "Expected '{' after 'else'".to_string(),
                            position: self.peek().position(),
                        });
                    }

                    let else_statements = self.block()?;

                    if !self.match_token(&TokenType::RightBrace) {
                        return Err(GraphoidError::SyntaxError {
                            message: "Expected '}' after else block".to_string(),
                            position: self.peek().position(),
                        });
                    }

                    Some(Box::new(Expr::Block {
                        statements: else_statements,
                        position: self.previous_position(),
                    }))
                } else {
                    None
                };

                return Ok(Expr::Conditional {
                    condition: Box::new(condition),
                    then_expr: Box::new(then_expr),
                    else_expr,
                    is_unless,
                    position: if_position,
                });
            } else {
                // No block, this shouldn't happen in primary()
                // This would be handled by conditional_expression for suffix forms
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '{' after if condition (use suffix form for inline conditionals)".to_string(),
                    position: self.peek().position(),
                });
            }
        }

        // Maps without type: {}
        if self.match_token(&TokenType::LeftBrace) {
            let mut entries = Vec::new();
            // Skip newlines after opening brace
            while self.match_token(&TokenType::Newline) {}

            if !self.check(&TokenType::RightBrace) {
                loop {
                    // Skip newlines before each entry
                    while self.match_token(&TokenType::Newline) {}

                    // Parse key (must be string or identifier)
                    let key = if let TokenType::String(s) = &self.peek().token_type {
                        let k = s.clone();
                        self.advance();
                        k
                    } else if let TokenType::Identifier(id) = &self.peek().token_type {
                        let k = id.clone();
                        self.advance();
                        k
                    } else {
                        return Err(GraphoidError::SyntaxError {
                            message: "Expected string or identifier as map key".to_string(),
                            position: self.peek().position(),
                        });
                    };

                    // Expect ':'
                    if !self.match_token(&TokenType::Colon) {
                        return Err(GraphoidError::SyntaxError {
                            message: "Expected ':' after map key".to_string(),
                            position: self.peek().position(),
                        });
                    }

                    // Parse value - support lambdas
                    let value = self.lambda_or_expression()?;
                    entries.push((key, value));

                    if !self.match_token(&TokenType::Comma) {
                        break;
                    }
                }
            }

            // Skip newlines before closing brace
            while self.match_token(&TokenType::Newline) {}

            if !self.match_token(&TokenType::RightBrace) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '}' after map entries".to_string(),
                    position: self.peek().position(),
                });
            }

            return Ok(Expr::Map { entries, position });
        }

        // Parenthesized expressions
        if self.match_token(&TokenType::LeftParen) {
            let expr = self.expression()?;
            if !self.match_token(&TokenType::RightParen) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected ')' after expression".to_string(),
                    position: self.peek().position(),
                });
            }
            return Ok(expr);
        }

        Err(GraphoidError::SyntaxError {
            message: format!("Unexpected token: {:?}", self.peek().token_type),
            position: self.peek().position(),
        })
    }

    // Helper methods
    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn previous_position(&self) -> SourcePosition {
        self.previous().position()
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type)
    }

    fn check_next(&self, token_type: &TokenType) -> bool {
        if self.current + 1 >= self.tokens.len() {
            return false;
        }
        std::mem::discriminant(&self.tokens[self.current + 1].token_type) == std::mem::discriminant(token_type)
    }

    /// Check if the next token is an identifier (any identifier, regardless of name)
    fn check_next_is_identifier(&self) -> bool {
        if self.current + 1 >= self.tokens.len() {
            return false;
        }
        matches!(self.tokens[self.current + 1].token_type, TokenType::Identifier(_))
    }

    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Skip all consecutive newline tokens
    fn skip_newlines(&mut self) {
        while self.match_token(&TokenType::Newline) {}
    }

    /// Parses key-value entries for graph/tree/map config
    fn parse_config_entries(&mut self) -> Result<Vec<(String, Expr)>> {
        let mut entries = Vec::new();

        if self.check(&TokenType::RightBrace) {
            return Ok(entries);
        }

        loop {
            // Parse key (must be string or identifier)
            let key = if let TokenType::String(s) = &self.peek().token_type {
                let k = s.clone();
                self.advance();
                k
            } else if let TokenType::Identifier(id) = &self.peek().token_type {
                let k = id.clone();
                self.advance();
                k
            } else {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected string or identifier as key".to_string(),
                    position: self.peek().position(),
                });
            };

            // Expect ':'
            if !self.match_token(&TokenType::Colon) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected ':' after key".to_string(),
                    position: self.peek().position(),
                });
            }

            // Parse value
            let value = self.expression()?;
            entries.push((key, value));

            if !self.match_token(&TokenType::Comma) {
                break;
            }
        }

        Ok(entries)
    }

    // =========================================================================
    // Pattern Matching (Phase 7)
    // =========================================================================

    /// Parse a pattern: |pattern|
    fn parse_pattern(&mut self) -> Result<Pattern> {
        let position = self.peek().position();

        // Expect '|'
        if !self.match_token(&TokenType::Pipe) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '|' to start pattern".to_string(),
                position,
            });
        }

        // Parse the pattern content
        let pattern = match &self.peek().token_type {
            TokenType::Number(n) => {
                let value = *n;
                self.advance();
                Pattern::Literal {
                    value: LiteralValue::Number(value),
                    position,
                }
            }
            TokenType::String(s) => {
                let value = s.clone();
                self.advance();
                Pattern::Literal {
                    value: LiteralValue::String(value),
                    position,
                }
            }
            TokenType::None => {
                self.advance();
                Pattern::Literal {
                    value: LiteralValue::None,
                    position,
                }
            }
            TokenType::True => {
                self.advance();
                Pattern::Literal {
                    value: LiteralValue::Boolean(true),
                    position,
                }
            }
            TokenType::False => {
                self.advance();
                Pattern::Literal {
                    value: LiteralValue::Boolean(false),
                    position,
                }
            }
            TokenType::Identifier(sym) if sym == "_" => {
                self.advance();
                Pattern::Wildcard { position }
            }
            TokenType::Identifier(name) => {
                let var_name = name.clone();
                self.advance();
                Pattern::Variable {
                    name: var_name,
                    position,
                }
            }
            _ => {
                return Err(GraphoidError::SyntaxError {
                    message: format!("Expected pattern, got {:?}", self.peek().token_type),
                    position,
                });
            }
        };

        // Expect closing '|'
        if !self.match_token(&TokenType::Pipe) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '|' after pattern".to_string(),
                position: self.peek().position(),
            });
        }

        Ok(pattern)
    }

    /// Parse pattern clause: |pattern| => result
    fn parse_pattern_clause(&mut self) -> Result<PatternClause> {
        let position = self.peek().position();

        let pattern = self.parse_pattern()?;

        // Parse optional guard: if <expr>
        let guard = if self.match_token(&TokenType::If) {
            Some(self.expression()?)
        } else {
            None
        };

        // Expect '=>'
        if !self.match_token(&TokenType::Arrow) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '=>' after pattern".to_string(),
                position: self.peek().position(),
            });
        }

        // Parse body expression
        let body = self.expression()?;

        Ok(PatternClause {
            pattern,
            guard,
            body,
            position,
        })
    }

    /// Parse match expression: match value { pattern => expr, ... }
    fn match_expression(&mut self, position: SourcePosition) -> Result<Expr> {
        use crate::ast::MatchArm;

        // Parse the value to match against
        let value = Box::new(self.expression()?);

        // Expect opening brace
        if !self.match_token(&TokenType::LeftBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '{' after match value".to_string(),
                position: self.peek().position(),
            });
        }

        // Skip optional newlines after opening brace
        self.skip_newlines();

        // Parse match arms
        let mut arms = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let arm_position = self.peek().position();

            // Parse pattern
            let pattern = self.match_pattern()?;

            // Expect arrow
            if !self.match_token(&TokenType::Arrow) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '=>' after match pattern".to_string(),
                    position: self.peek().position(),
                });
            }

            // Parse body expression (using primary() to avoid consuming postfix ops across newlines)
            let body = self.primary()?;

            arms.push(MatchArm {
                pattern,
                body,
                position: arm_position,
            });

            // Match arm must be followed by comma, newline, or closing brace
            if self.match_token(&TokenType::Comma) {
                // Comma found, skip any trailing newlines
                self.skip_newlines();
            } else if self.match_token(&TokenType::Newline) {
                // Newline found, skip any additional newlines
                self.skip_newlines();
            } else if self.check(&TokenType::RightBrace) {
                // Closing brace - we're done
                break;
            } else {
                return Err(GraphoidError::SyntaxError {
                    message: format!("Expected comma, newline, or '}}' after match arm, got {:?}", self.peek().token_type),
                    position: self.peek().position(),
                });
            }
        }

        // Skip optional newlines before closing brace
        self.skip_newlines();

        // Expect closing brace
        if !self.match_token(&TokenType::RightBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '}' after match arms".to_string(),
                position: self.peek().position(),
            });
        }

        if arms.is_empty() {
            return Err(GraphoidError::SyntaxError {
                message: "Match expression must have at least one arm".to_string(),
                position,
            });
        }

        Ok(Expr::Match {
            value,
            arms,
            position,
        })
    }

    /// Parse match pattern
    fn match_pattern(&mut self) -> Result<crate::ast::MatchPattern> {
        use crate::ast::{MatchPattern, LiteralValue};

        // Wildcard pattern: _
        if let TokenType::Identifier(name) = &self.peek().token_type {
            if name == "_" {
                self.advance();
                return Ok(MatchPattern::Wildcard);
            }
        }

        // List pattern: [], [x], [x, y], [x, ...rest]
        if self.match_token(&TokenType::LeftBracket) {
            let mut elements = Vec::new();
            let mut rest_name: Option<String> = None;

            while !self.check(&TokenType::RightBracket) && !self.is_at_end() {
                // Check for rest pattern: ...name or just ...
                if self.match_token(&TokenType::DotDotDot) {
                    // Get rest variable name (optional)
                    if let TokenType::Identifier(name) = &self.peek().token_type {
                        rest_name = Some(name.clone());
                        self.advance();
                    } else {
                        // Anonymous rest pattern: ... (captures but doesn't bind)
                        rest_name = Some("_".to_string());
                    }

                    // Optional comma after rest
                    self.match_token(&TokenType::Comma);

                    // Rest must be last element
                    if !self.check(&TokenType::RightBracket) {
                        return Err(GraphoidError::SyntaxError {
                            message: "Rest pattern must be the last element in a list pattern".to_string(),
                            position: self.peek().position(),
                        });
                    }
                    break;
                }

                // Parse regular nested pattern
                elements.push(self.match_pattern()?);

                // Optional comma
                if !self.check(&TokenType::RightBracket) {
                    self.match_token(&TokenType::Comma);
                }
            }

            if !self.match_token(&TokenType::RightBracket) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected ']' after list pattern".to_string(),
                    position: self.peek().position(),
                });
            }

            return Ok(MatchPattern::List {
                elements,
                rest_name,
            });
        }

        // Number literal
        if let TokenType::Number(n) = self.peek().token_type {
            self.advance();
            return Ok(MatchPattern::Literal(LiteralValue::Number(n)));
        }

        // String literal
        if let TokenType::String(s) = &self.peek().token_type {
            let str_val = s.clone();
            self.advance();
            return Ok(MatchPattern::Literal(LiteralValue::String(str_val)));
        }

        // Boolean literals
        if self.match_token(&TokenType::True) {
            return Ok(MatchPattern::Literal(LiteralValue::Boolean(true)));
        }

        if self.match_token(&TokenType::False) {
            return Ok(MatchPattern::Literal(LiteralValue::Boolean(false)));
        }

        // None literal
        if self.match_token(&TokenType::None) {
            return Ok(MatchPattern::Literal(LiteralValue::None));
        }

        // Variable binding pattern
        if let TokenType::Identifier(name) = &self.peek().token_type {
            let var_name = name.clone();
            self.advance();
            return Ok(MatchPattern::Variable(var_name));
        }

        Err(GraphoidError::SyntaxError {
            message: format!("Expected pattern, got {:?}", self.peek().token_type),
            position: self.peek().position(),
        })
    }

}

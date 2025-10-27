//! Parser module for building AST from tokens
//!
//! This module implements a recursive descent parser with precedence climbing
//! for expression parsing.

use crate::ast::{
    AssignmentTarget, BinaryOp, Expr, LiteralValue, Parameter, Program, Stmt, TypeAnnotation,
    UnaryOp,
};
use crate::error::{GraphoidError, Result, SourcePosition};
use crate::lexer::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
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

        // Check for type annotations or keywords
        // BUT: If ListType is followed by dot, it's a static method call, not a declaration
        let is_list_static_call = self.check(&TokenType::ListType) && self.check_next(&TokenType::Dot);

        let result = if !is_list_static_call && (
            self.check(&TokenType::NumType)
            || self.check(&TokenType::StringType)
            || self.check(&TokenType::BoolType)
            || self.check(&TokenType::ListType)
            || self.check(&TokenType::MapType)
            || self.check(&TokenType::TreeType)
            || self.check(&TokenType::GraphType)
            || self.check(&TokenType::DataType)
            || self.check(&TokenType::TimeType)
        ) {
            self.variable_declaration()
        } else if self.match_token(&TokenType::Func) {
            self.function_declaration()
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
        } else {
            // Try to parse as assignment or expression
            self.assignment_or_expression()
        };

        // Consume optional trailing newline
        self.match_token(&TokenType::Newline);

        result
    }

    fn variable_declaration(&mut self) -> Result<Stmt> {
        let position = self.peek().position();

        // Parse type annotation
        let type_annotation = self.type_annotation()?;

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
            type_annotation: Some(type_annotation),
            value,
            position,
        })
    }

    fn type_annotation(&mut self) -> Result<TypeAnnotation> {
        let base_type = match &self.peek().token_type {
            TokenType::NumType => "num",
            TokenType::StringType => "string",
            TokenType::BoolType => "bool",
            TokenType::ListType => "list",
            TokenType::MapType => "map",
            TokenType::TreeType => "tree",
            TokenType::GraphType => "graph",
            TokenType::DataType => "data",
            TokenType::TimeType => "time",
            _ => {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected type annotation".to_string(),
                    position: self.peek().position(),
                })
            }
        }
        .to_string();

        self.advance();

        // TODO: Parse type constraints like list<num>
        //
        // ⚠️  IMPORTANT: NO GENERICS POLICY ENFORCEMENT
        // See: dev_docs/NO_GENERICS_POLICY.md
        //
        // When implementing type constraints, MUST enforce:
        //
        // 1. ✅ ALLOWED: Single type parameter on built-in collections
        //    - list<num>, hash<string>, tree<num>, graph<num>
        //    - Constraint must be a PRIMITIVE type only
        //    - Runtime-checked, not compile-time
        //
        // 2. ❌ FORBIDDEN: Multiple type parameters
        //    - hash<K, V> → SYNTAX ERROR
        //    - Result<T, E> → SYNTAX ERROR
        //    - Reject with: "Multiple type parameters not supported"
        //
        // 3. ❌ FORBIDDEN: Nested constraints
        //    - list<list<num>> → SYNTAX ERROR
        //    - Reject with: "Nested type constraints not supported"
        //
        // 4. ❌ FORBIDDEN: Type constraints on user-defined types
        //    - Only allow on: list, hash, tree, graph
        //    - Reject with: "Type parameters only allowed on built-in collections"
        //
        // 5. ❌ FORBIDDEN: Generic type variables
        //    - <T> where T is not num/string/bool/etc → SYNTAX ERROR
        //    - Reject with: "Generic type variables not supported"
        //
        // Implementation strategy:
        // - if peek() == TokenType::Less:
        //     - Ensure base_type is one of: list, hash, tree, graph
        //     - Parse exactly ONE constraint type
        //     - Ensure constraint is primitive (num, string, bool, symbol, time, none)
        //     - Check for comma → reject (multiple params)
        //     - Check for another '<' → reject (nesting)
        //     - Expect TokenType::Greater
        //
        Ok(TypeAnnotation {
            base_type,
            constraint: None,
        })
    }

    fn function_declaration(&mut self) -> Result<Stmt> {
        let position = self.previous_position();

        // Expect identifier
        let name = if let TokenType::Identifier(id) = &self.peek().token_type {
            let n = id.clone();
            self.advance();
            n
        } else {
            return Err(GraphoidError::SyntaxError {
                message: "Expected function name".to_string(),
                position: self.peek().position(),
            });
        };

        // ⚠️  NO GENERICS POLICY ENFORCEMENT
        // See: dev_docs/NO_GENERICS_POLICY.md
        //
        // If next token is '<', this is an attempt at generic function syntax
        // fn foo<T>(...) → FORBIDDEN
        //
        // TODO: When we see '<' after function name, reject with:
        // "Generic functions are not supported in Graphoid.
        //  Use duck typing instead - functions work on values, not types.
        //  See: dev_docs/NO_GENERICS_POLICY.md"

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

                // TODO: Parse default values
                params.push(Parameter {
                    name: param_name,
                    default_value: None,
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

        // Expect '{'
        if !self.match_token(&TokenType::LeftBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '{' before function body".to_string(),
                position: self.peek().position(),
            });
        }

        // Parse body
        let body = self.block()?;

        // Expect '}'
        if !self.match_token(&TokenType::RightBrace) {
            return Err(GraphoidError::SyntaxError {
                message: "Expected '}' after function body".to_string(),
                position: self.peek().position(),
            });
        }

        Ok(Stmt::FunctionDecl {
            name,
            params,
            body,
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
        let alias = if self.match_token(&TokenType::Alias) {
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

    fn assignment_or_expression(&mut self) -> Result<Stmt> {
        let position = self.peek().position();

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

    // Expression parsing with precedence climbing
    fn expression(&mut self) -> Result<Expr> {
        // TODO: Add lambda parsing
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
        let mut expr = self.term()?;

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

    fn power(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;

        while self.match_token(&TokenType::Caret) || self.match_token(&TokenType::DotCaret) {
            let op = match &self.previous().token_type {
                TokenType::Caret => BinaryOp::Power,
                TokenType::DotCaret => BinaryOp::DotPower,
                _ => unreachable!(),
            };
            let position = self.previous_position();
            let right = self.unary()?;
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

        self.postfix()
    }

    fn postfix(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(&TokenType::LeftParen) {
                // Function call
                let position = expr.position().clone();
                let args = self.arguments()?;
                if !self.match_token(&TokenType::RightParen) {
                    return Err(GraphoidError::SyntaxError {
                        message: "Expected ')' after arguments".to_string(),
                        position: self.peek().position(),
                    });
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

                // Check if it's a method call (with parentheses)
                if self.match_token(&TokenType::LeftParen) {
                    let args = self.arguments()?;
                    if !self.match_token(&TokenType::RightParen) {
                        return Err(GraphoidError::SyntaxError {
                            message: "Expected ')' after method arguments".to_string(),
                            position: self.peek().position(),
                        });
                    }
                    expr = Expr::MethodCall {
                        object: Box::new(expr),
                        method,
                        args,
                        position,
                    };
                } else {
                    // Property access - treat as method call with no args for now
                    expr = Expr::MethodCall {
                        object: Box::new(expr),
                        method,
                        args: vec![],
                        position,
                    };
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn arguments(&mut self) -> Result<Vec<Expr>> {
        let mut args = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                // Try to parse lambda or regular expression
                args.push(self.lambda_or_expression()?);
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }

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

                    let body = Box::new(self.or_expression()?);
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
                let body = Box::new(self.or_expression()?);
                return Ok(Expr::Lambda {
                    params,
                    body,
                    position,
                });
            }

            // Not a lambda, rewind and parse as expression
            self.current = paren_checkpoint;
        }

        // Not a lambda, parse as regular expression
        self.expression()
    }

    fn primary(&mut self) -> Result<Expr> {
        let position = self.peek().position();

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

            let mut elements = Vec::new();
            if !self.check(&TokenType::RightBracket) {
                loop {
                    elements.push(self.expression()?);
                    if !self.match_token(&TokenType::Comma) {
                        break;
                    }
                }
            }

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

            if !self.check(&TokenType::RightBracket) {
                loop {
                    elements.push(self.expression()?);
                    if !self.match_token(&TokenType::Comma) {
                        break;
                    }
                }
            }

            if !self.match_token(&TokenType::RightBracket) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected ']' after list elements".to_string(),
                    position: self.peek().position(),
                });
            }

            return Ok(Expr::List { elements, position });
        }

        // Graphs: graph { type: :directed }
        if self.match_token(&TokenType::GraphType) {
            if !self.match_token(&TokenType::LeftBrace) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '{' after 'graph'".to_string(),
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

            return Ok(Expr::Graph { config, position });
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
                position: position.clone()
            };

            let symbol_expr = Expr::Literal {
                value: crate::ast::LiteralValue::Symbol("tree".to_string()),
                position: position.clone(),
            };

            return Ok(Expr::MethodCall {
                object: Box::new(graph_expr),
                method: "with_ruleset".to_string(),
                args: vec![symbol_expr],
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
            if !self.check(&TokenType::RightBrace) {
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

                    // Parse value
                    let value = self.expression()?;
                    entries.push((key, value));

                    if !self.match_token(&TokenType::Comma) {
                        break;
                    }
                }
            }

            if !self.match_token(&TokenType::RightBrace) {
                return Err(GraphoidError::SyntaxError {
                    message: "Expected '}' after map entries".to_string(),
                    position: self.peek().position(),
                });
            }

            return Ok(Expr::Map { entries, position });
        }

        // Maps without type: {}
        if self.match_token(&TokenType::LeftBrace) {
            let mut entries = Vec::new();

            if !self.check(&TokenType::RightBrace) {
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

                    // Parse value
                    let value = self.expression()?;
                    entries.push((key, value));

                    if !self.match_token(&TokenType::Comma) {
                        break;
                    }
                }
            }

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

    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
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
}

use std::iter::Peekable;
use std::result;

use binary_op::BinaryOp;
use error::ParseError;
use expr::Expression;
use scanner::{Scanner, Token};

pub type Result<T> = result::Result<T, ParseError>;

pub struct Parser<'a> {
    scanner: Peekable<Scanner<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { scanner: Scanner::new(input).peekable() }
    }

    // Assuming we've read an open paren, parse the inner expression and the
    // closing paren.
    fn parse_paren_expr(&mut self) -> Result<Expression> {
        let inner = match self.next() {
            Some(Ok(expr)) => expr,
            Some(Err(e)) => return Err(e),
            None => return Err(ParseError::UnexpectedEOF),
        };

        match self.scanner.next() {
            Some(Ok(Token::CloseParen)) => Ok(Expression::ParenExpr(Box::new(inner))),
            Some(Ok(t)) => Err(ParseError::Unexpected(t)),
            Some(Err(e)) => Err(ParseError::ScanError(e)),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    // Assuming we've read an open curly, parse the inner block and the closing
    // curly.
    fn parse_block(&mut self) -> Result<Expression> {
        let mut body = vec![];

        loop {
            match self.scanner.peek().cloned() {
                None => return Err(ParseError::UnexpectedEOF),
                Some(Err(e)) => return Err(ParseError::ScanError(e)),
                Some(Ok(Token::CloseCurly)) => {
                    self.scanner.next();
                    return Ok(Expression::Block(body));
                }
                _ => {
                    match self.next() {
                        Some(Ok(expr)) => body.push(expr),
                        Some(Err(e)) => return Err(e),
                        None => return Err(ParseError::UnexpectedEOF),
                    }
                }
            }
        }
    }

    // Assuming we've parsed an identifier, parse the rest of the expression.
    fn parse_identifier(&mut self, name: String) -> Result<Expression> {
        match self.scanner.peek() {
            Some(&Ok(Token::OpenParen)) => self.scanner.next(),
            _ => return Ok(Expression::Variable(name)),
        };

        match self.parse_expr_list(&Token::CloseParen) {
            Ok(args) => {
                Ok(Expression::FunctionCall {
                    name: name,
                    args: args,
                })
            }
            Err(e) => Err(e),
        }
    }

    // Assuming we've read an "if", parse the condition, the body and the else
    // branch, if present.
    fn parse_if(&mut self) -> Result<Expression> {
        let condition = match self.next() {
            None => return Err(ParseError::UnexpectedEOF),
            Some(Err(e)) => return Err(e),
            Some(Ok(expr)) => expr,
        };

        let body = match self.next() {
            None => return Err(ParseError::UnexpectedEOF),
            Some(Err(e)) => return Err(e),
            Some(Ok(expr)) => expr,
        };

        let else_branch = match self.scanner.peek() {
            Some(&Ok(Token::Else)) => {
                self.scanner.next();
                match self.next() {
                    None => return Err(ParseError::UnexpectedEOF),
                    Some(Err(e)) => return Err(e),
                    Some(Ok(expr)) => Some(Box::new(expr)),
                }
            }
            _ => None,
        };

        Ok(Expression::IfExpr {
            cond: Box::new(condition),
            body: Box::new(body),
            else_branch: else_branch,
        })
    }

    // Assuming we've read a "while", parse the condition and the body.
    fn parse_while(&mut self) -> Result<Expression> {
        let condition = match self.next() {
            None => return Err(ParseError::UnexpectedEOF),
            Some(Err(e)) => return Err(e),
            Some(Ok(expr)) => expr,
        };

        let body = match self.next() {
            None => return Err(ParseError::UnexpectedEOF),
            Some(Err(e)) => return Err(e),
            Some(Ok(expr)) => expr,
        };

        Ok(Expression::WhileLoop {
            cond: Box::new(condition),
            body: Box::new(body),
        })
    }

    // parse_expr_list parses a comma-separated list of expressions until the
    // specified token is found.
    fn parse_expr_list(&mut self, until: &Token) -> Result<Vec<Expression>> {
        let mut expressions = Vec::new();

        let mut done = false;
        if let Some(&Ok(ref t)) = self.scanner.peek() {
            done = t == until;
        }

        if done {
            self.scanner.next();
            return Ok(expressions);
        }

        loop {
            match self.next() {
                Some(Ok(expr)) => expressions.push(expr),
                Some(Err(e)) => return Err(e),
                None => return Err(ParseError::UnexpectedEOF),
            }

            match self.scanner.next() {
                Some(Ok(Token::Comma)) => continue,
                Some(Ok(ref t)) if t == until => return Ok(expressions),
                Some(Ok(t)) => return Err(ParseError::Unexpected(t)),
                Some(Err(e)) => return Err(ParseError::ScanError(e)),
                None => return Err(ParseError::UnexpectedEOF),
            }
        }
    }

    fn apply_precedence(&mut self,
                        lhs: Box<Expression>,
                        op: BinaryOp,
                        rhs: Box<Expression>)
                        -> Expression {
        match *rhs {
            Expression::BinaryExpr { left: ref lhs_r, op: ref op_r, right: ref rhs_r } => {
                if op_r.precendence() < op.precendence() {
                    return Expression::BinaryExpr {
                        left: Box::new(Expression::BinaryExpr {
                            left: lhs.clone(),
                            op: op,
                            right: lhs_r.clone(),
                        }),
                        op: op_r.clone(),
                        right: rhs_r.clone(),
                    };
                }
            }
            _ => {}
        }

        Expression::BinaryExpr {
            left: lhs,
            op: op,
            right: rhs,
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Expression>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = match self.scanner.next() {
            None => return None,
            Some(Err(e)) => return Some(Err(ParseError::ScanError(e))),
            Some(Ok(t)) => t,
        };

        let expr_res = match token {
            Token::Nil => Ok(Expression::NilLiteral),
            Token::Boolean(b) => Ok(Expression::BooleanLiteral(b)),
            Token::Number(n) => Ok(Expression::NumberLiteral(n)),
            Token::String(s) => Ok(Expression::StrLiteral(s)),
            Token::OpenParen => self.parse_paren_expr(),
            Token::OpenCurly => self.parse_block(),
            Token::Identifier(s) => self.parse_identifier(s),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            t => Err(ParseError::Unexpected(t)),
        };

        let lhs = match expr_res {
            Ok(e) => e,
            Err(e) => return Some(Err(e)),
        };

        // Copy the next token because we might be part of a larger expression.
        let next = match self.scanner.peek().cloned() {
            Some(Ok(t)) => t,
            _ => return Some(Ok(lhs)),
        };

        // Binary expression.
        if let Some(op) = next.to_binary_op() {
            self.scanner.next();
            let rhs = match self.next() {
                Some(Ok(e)) => e,
                Some(Err(e)) => return Some(Err(e)),
                None => return Some(Err(ParseError::UnexpectedEOF)),
            };

            return Some(Ok(self.apply_precedence(Box::new(lhs), op, Box::new(rhs))));
        }

        // Assignment.
        if next == Token::Eq {
            if let Expression::Variable(v) = lhs {
                self.scanner.next();
                let rhs = match self.next() {
                    Some(Ok(e)) => e,
                    Some(Err(e)) => return Some(Err(e)),
                    None => return Some(Err(ParseError::UnexpectedEOF)),
                };

                return Some(Ok(Expression::Assignment {
                    left: v,
                    right: Box::new(rhs),
                }));
            }
        }

        Some(Ok(lhs))
    }
}

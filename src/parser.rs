use std::iter::Peekable;
use std::result;

use super::ast::Expression;
use super::scanner::{Scanner, Token, TokenError};

pub type Result<T> = result::Result<T, ParseError>;

#[derive(Debug,PartialEq)]
pub enum ParseError {
    ScanError(TokenError),
    Unexpected(Token),
    UnexpectedEOF,
}

pub struct Parser<'a> {
    scanner: Peekable<Scanner<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser{scanner: Scanner::new(input).peekable()}
    }

    // Assuming we've read an open paren, parse the inner expression and the
    // closing paren.
    fn parse_bracketed_expr(&mut self) -> Result<Expression> {
        let inner = match self.next() {
            Some(Ok(expr)) => expr,
            Some(Err(e)) => return Err(e),
            None => return Err(ParseError::UnexpectedEOF),
        };

        match self.scanner.next() {
            Some(Ok(Token::CloseParen)) => Ok(inner),
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
                },
                _ => match self.next() {
                    Some(Ok(expr)) => body.push(expr),
                    Some(Err(e)) => return Err(e),
                    None => return Err(ParseError::UnexpectedEOF),
                },
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
            Ok(args) => Ok(Expression::FunctionCall {
                name: name,
                args: args,
            }),
            Err(e) => Err(e),
        }
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
            Token::OpenParen => self.parse_bracketed_expr(),
            Token::OpenCurly => self.parse_block(),
            Token::Identifier(s) => self.parse_identifier(s),
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

            return Some(Ok(Expression::BinaryExpr{
                left: Box::new(lhs),
                op: op,
                right: Box::new(rhs),
            }));
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

                return Some(Ok(Expression::Assignment{
                    left: v,
                    right: Box::new(rhs),
                }));
            }
        }

        Some(Ok(lhs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::ast::*;

    #[test]
    fn test_literals() {
        let mut parser = Parser::new(r#"nil true false 1 "foo""#);
        assert_eq!(parser.next(), Some(Ok(Expression::NilLiteral)));
        assert_eq!(parser.next(), Some(Ok(Expression::BooleanLiteral(true))));
        assert_eq!(parser.next(), Some(Ok(Expression::BooleanLiteral(false))));
        assert_eq!(parser.next(), Some(Ok(Expression::NumberLiteral(1.0))));
        assert_eq!(parser.next(), Some(Ok(Expression::StrLiteral("foo".to_owned()))));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_parentheses() {
        let mut parser = Parser::new(r#"(nil)(((true)))"#);
        assert_eq!(parser.next(), Some(Ok(Expression::NilLiteral)));
        assert_eq!(parser.next(), Some(Ok(Expression::BooleanLiteral(true))));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_identifiers_and_functions() {
        let foo_var = Expression::Variable("foo".to_owned());

        let mut parser = Parser::new(r#"foo foo() foo(foo) foo(foo, foo)"#);
        assert_eq!(parser.next(), Some(Ok(foo_var.clone())));
        assert_eq!(parser.next(), Some(Ok(Expression::FunctionCall{
            name: "foo".to_owned(),
            args: vec![],
        })));
        assert_eq!(parser.next(), Some(Ok(Expression::FunctionCall{
            name: "foo".to_owned(),
            args: vec![foo_var.clone()],
        })));
        assert_eq!(parser.next(), Some(Ok(Expression::FunctionCall{
            name: "foo".to_owned(),
            args: vec![foo_var.clone(), foo_var.clone()],
        })));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_binary_expr() {
        let mut parser = Parser::new(r#"1 + 2 - 3 * 4 / 5"#);

        assert_eq!(parser.next(), Some(Ok(Expression::BinaryExpr{
            left: Box::new(Expression::NumberLiteral(1.0)),
            op: BinaryOp::Add,
            right: Box::new(Expression::BinaryExpr{
                left: Box::new(Expression::NumberLiteral(2.0)),
                op: BinaryOp::Sub,
                right: Box::new(Expression::BinaryExpr{
                    left: Box::new(Expression::NumberLiteral(3.0)),
                    op: BinaryOp::Mul,
                    right: Box::new(Expression::BinaryExpr{
                        left: Box::new(Expression::NumberLiteral(4.0)),
                        op: BinaryOp::Div,
                        right: Box::new(Expression::NumberLiteral(5.0)),
                    }),
                }),
            }),
        })));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_binary_ops() {
        let cases = vec![
            ("+", BinaryOp::Add),
            ("-", BinaryOp::Sub),
            ("*", BinaryOp::Mul),
            ("/", BinaryOp::Div),
            ("==", BinaryOp::Eq),
            ("<", BinaryOp::Lt),
            ("<=", BinaryOp::LtEq),
            (">", BinaryOp::Gt),
            (">=", BinaryOp::GtEq),
        ];

        for (s, op) in cases {
            let expr_str = format!("1 {} 2", s);

            let mut parser = Parser::new(&expr_str);
            assert_eq!(parser.next(), Some(Ok(Expression::BinaryExpr{
                left: Box::new(Expression::NumberLiteral(1.0)),
                op: op,
                right: Box::new(Expression::NumberLiteral(2.0)),
            })));
            assert_eq!(parser.next(), None);
        }
    }

    #[test]
    fn test_blocks() {
        let mut parser = Parser::new("{1{}2}");

        assert_eq!(parser.next(), Some(Ok(Expression::Block(vec![
            Expression::NumberLiteral(1.0),
            Expression::Block(vec![]),
            Expression::NumberLiteral(2.0),
        ]))));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_assignments() {
        let mut parser = Parser::new("x = y = z");

        assert_eq!(parser.next(), Some(Ok(Expression::Assignment{
            left: "x".to_owned(),
            right: Box::new(Expression::Assignment{
                left: "y".to_owned(),
                right: Box::new(Expression::Variable("z".to_owned())),
            }),
        })));
        assert_eq!(parser.next(), None);
    }
}

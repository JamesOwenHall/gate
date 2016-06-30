use std::iter::{Iterator, Peekable};
use std::str::Chars;

#[derive(Debug,PartialEq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Eq,
    DoubleEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Plus,
    Minus,
    Times,
    Divide,
}

pub struct Scanner<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Scanner{input: input.chars().peekable()}
    }

    fn is_space(c: char) -> bool {
        c == ' ' || c == '\t' || c == '\n' || c == '\r'
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&c) = self.input.peek() {
            if Self::is_space(c) {
                self.input.next();
            } else {
                break;
            }
        }

        match self.input.peek() {
            None => None,
            Some(&'(') => {
                self.input.next();
                Some(Token::OpenParen)
            },
            Some(&')') => {
                self.input.next();
                Some(Token::CloseParen)
            },
            Some(&'=') => {
                self.input.next();
                if let Some(&'=') = self.input.peek() {
                    self.input.next();
                    Some(Token::DoubleEq)
                } else {
                    Some(Token::Eq)
                }
            },
            Some(&'<') => {
                self.input.next();
                if let Some(&'=') = self.input.peek() {
                    self.input.next();
                    Some(Token::LtEq)
                } else {
                    Some(Token::Lt)
                }
            },
            Some(&'>') => {
                self.input.next();
                if let Some(&'=') = self.input.peek() {
                    self.input.next();
                    Some(Token::GtEq)
                } else {
                    Some(Token::Gt)
                }
            },
            Some(&'+') => {
                self.input.next();
                Some(Token::Plus)
            },
            Some(&'-') => {
                self.input.next();
                Some(Token::Minus)
            },
            Some(&'*') => {
                self.input.next();
                Some(Token::Times)
            },
            Some(&'/') => {
                self.input.next();
                Some(Token::Divide)
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Token::*;

    #[test]
    fn test_punctuation() {
        let mut s = Scanner::new("() = == < <= > >= +-*/");
        assert_eq!(s.next(), Some(OpenParen));
        assert_eq!(s.next(), Some(CloseParen));
        assert_eq!(s.next(), Some(Eq));
        assert_eq!(s.next(), Some(DoubleEq));
        assert_eq!(s.next(), Some(Lt));
        assert_eq!(s.next(), Some(LtEq));
        assert_eq!(s.next(), Some(Gt));
        assert_eq!(s.next(), Some(GtEq));
        assert_eq!(s.next(), Some(Plus));
        assert_eq!(s.next(), Some(Minus));
        assert_eq!(s.next(), Some(Times));
        assert_eq!(s.next(), Some(Divide));
        assert_eq!(s.next(), None);
    }
}

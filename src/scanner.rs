use std::iter::{Iterator, Peekable};
use std::result;
use std::str::Chars;

#[derive(Clone,Debug,PartialEq)]
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
    Nil,
    Boolean(bool),
    Identifier(String),
}

#[derive(Clone,Debug,PartialEq)]
pub enum TokenError {
    UnexpectedChar(char),
}

pub type Result<T> = result::Result<T, TokenError>;

pub struct Scanner<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Scanner{input: input.chars().peekable()}
    }

    fn read_word(&mut self) -> Token {
        let mut word = String::new();
        while let Some(&c) = self.input.peek() {
            if !Self::is_digit(c) && !Self::is_alpha(c) {
                break;
            }

            self.input.next();
            word.push(c);
        }

        match word.as_ref() {
            "nil" => Token::Nil,
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            _ => Token::Identifier(word),
        }
    }

    fn is_space(c: char) -> bool {
        c == ' ' || c == '\t' || c == '\n' || c == '\r'
    }

    fn is_alpha(c: char) -> bool {
        c == '_' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z')
    }

    fn is_digit(c: char) -> bool {
        ('0' <= c && c <= '9')
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token>;

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
                Some(Ok(Token::OpenParen))
            },
            Some(&')') => {
                self.input.next();
                Some(Ok(Token::CloseParen))
            },
            Some(&'=') => {
                self.input.next();
                if let Some(&'=') = self.input.peek() {
                    self.input.next();
                    Some(Ok(Token::DoubleEq))
                } else {
                    Some(Ok(Token::Eq))
                }
            },
            Some(&'<') => {
                self.input.next();
                if let Some(&'=') = self.input.peek() {
                    self.input.next();
                    Some(Ok(Token::LtEq))
                } else {
                    Some(Ok(Token::Lt))
                }
            },
            Some(&'>') => {
                self.input.next();
                if let Some(&'=') = self.input.peek() {
                    self.input.next();
                    Some(Ok(Token::GtEq))
                } else {
                    Some(Ok(Token::Gt))
                }
            },
            Some(&'+') => {
                self.input.next();
                Some(Ok(Token::Plus))
            },
            Some(&'-') => {
                self.input.next();
                Some(Ok(Token::Minus))
            },
            Some(&'*') => {
                self.input.next();
                Some(Ok(Token::Times))
            },
            Some(&'/') => {
                self.input.next();
                Some(Ok(Token::Divide))
            },
            Some(&c) if Self::is_alpha(c) => Some(Ok(self.read_word())),
            Some(&c) => {
                self.input.next();
                Some(Err(TokenError::UnexpectedChar(c)))
            },
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
        assert_eq!(s.next(), Some(Ok(OpenParen)));
        assert_eq!(s.next(), Some(Ok(CloseParen)));
        assert_eq!(s.next(), Some(Ok(Eq)));
        assert_eq!(s.next(), Some(Ok(DoubleEq)));
        assert_eq!(s.next(), Some(Ok(Lt)));
        assert_eq!(s.next(), Some(Ok(LtEq)));
        assert_eq!(s.next(), Some(Ok(Gt)));
        assert_eq!(s.next(), Some(Ok(GtEq)));
        assert_eq!(s.next(), Some(Ok(Plus)));
        assert_eq!(s.next(), Some(Ok(Minus)));
        assert_eq!(s.next(), Some(Ok(Times)));
        assert_eq!(s.next(), Some(Ok(Divide)));
        assert_eq!(s.next(), None);
    }

    #[test]
    fn test_unexpected_char() {
        let mut s = Scanner::new("($)");
        assert_eq!(s.next(), Some(Ok(OpenParen)));
        assert_eq!(s.next(), Some(Err(TokenError::UnexpectedChar('$'))));
    }

    #[test]
    fn test_words() {
        let mut s = Scanner::new("foo FOO _123_ Nil nil false true");
        assert_eq!(s.next(), Some(Ok(Identifier("foo".to_owned()))));
        assert_eq!(s.next(), Some(Ok(Identifier("FOO".to_owned()))));
        assert_eq!(s.next(), Some(Ok(Identifier("_123_".to_owned()))));
        assert_eq!(s.next(), Some(Ok(Identifier("Nil".to_owned()))));
        assert_eq!(s.next(), Some(Ok(Nil)));
        assert_eq!(s.next(), Some(Ok(Boolean(false))));
        assert_eq!(s.next(), Some(Ok(Boolean(true))));
        assert_eq!(s.next(), None);
    }
}

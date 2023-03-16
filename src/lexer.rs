use std::io::Error;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Integer(i64),
    Float(f64),
    String(String),
    Keyword(String),
    Symbol(String),
    Quote,
}

pub struct Tokenizer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            input: input.chars().peekable(),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if ch.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }

    fn parse_string(&mut self) -> Token {
        let mut string = String::new();
        while let Some(ch) = self.input.next() {
            match ch {
                '\\' => {
                    if let Some(escaped) = self.input.next() {
                        string.push(escaped);
                    }
                }
                '"' => break,
                _ => string.push(ch),
            }
        }
        Token::String(string)
    }

    fn parse_number(&mut self, first: char) -> Token {
        let mut n = first.to_string();
        let mut has_dot = first == '.';
        while let Some(&ch) = self.input.peek() {
            if ch == '.' && !has_dot {
                has_dot = true;
                n.push(self.input.next().unwrap());
            } else if ch.is_digit(10) {
                n.push(self.input.next().unwrap());
            } else {
                break;
            }
        }
        if has_dot {
            Token::Float(n.parse().unwrap())
        } else {
            Token::Integer(n.parse().unwrap())
        }
    }

    fn is_symbol(&self, ch: char) -> bool {
        return ch.is_alphanumeric()
            || ch == '-'
            || ch == '_'
            || ch == '+'
            || ch == '*'
            || ch == '/'
            || ch == '<'
            || ch == '>'
            || ch == '='
            || ch == '!'
            || ch == '?'
            || ch == '&'
            || ch == ':'
            || ch == '.';
    }

    fn read_symbol(&mut self, first: char) -> Token {
        let mut symbol = first.to_string();
        while let Some(&ch) = self.input.peek() {
            if self.is_symbol(ch) {
                symbol.push(self.input.next().unwrap());
            } else {
                break;
            }
        }

        if symbol.starts_with(':') {
            return Token::Keyword(symbol[1..].to_string());
        }
        Token::Symbol(symbol)
    }

    fn read_comment(&mut self) {
        while let Some(ch) = self.input.next() {
            if ch == '\n' {
                break;
            }
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        match self.input.next()? {
            '(' => Some(Token::OpenParen),
            ')' => Some(Token::CloseParen),
            '"' => Some(self.parse_string()),
            ch @ '1'..='9' => Some(self.parse_number(ch)),
            ';' => {
                self.read_comment();
                self.next()
            }
            ch if self.is_symbol(ch) => Some(self.read_symbol(ch)),
            '\'' => Some(Token::Quote),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Integer
    #[test]
    fn test_addition() {
        let input = "(+ 1 2.0 (- 3 4) 'foo :keyword \"string\" ; comment\n)";
        let mut tokenizer = Tokenizer::new(input);

        assert_eq!(tokenizer.next(), Some(Token::OpenParen));
        assert_eq!(tokenizer.next(), Some(Token::Symbol("+".to_string())));
        assert_eq!(tokenizer.next(), Some(Token::Integer(1)));
        assert_eq!(tokenizer.next(), Some(Token::Float(2.0)));
        assert_eq!(tokenizer.next(), Some(Token::OpenParen));
        assert_eq!(tokenizer.next(), Some(Token::Symbol("-".to_string())));
        assert_eq!(tokenizer.next(), Some(Token::Integer(3)));
        assert_eq!(tokenizer.next(), Some(Token::Integer(4)));
        assert_eq!(tokenizer.next(), Some(Token::CloseParen));
        assert_eq!(tokenizer.next(), Some(Token::Quote));
        assert_eq!(tokenizer.next(), Some(Token::Symbol("foo".to_string())));
        assert_eq!(
            tokenizer.next(),
            Some(Token::Keyword("keyword".to_string()))
        );
        assert_eq!(tokenizer.next(), Some(Token::String("string".to_string())));
        assert_eq!(tokenizer.next(), Some(Token::CloseParen));
        assert_eq!(tokenizer.next(), None);
    }
}

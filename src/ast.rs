#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    String(String),
    Keyword(String),
    Symbol(String),
    List(Vec<Expr>),
    Quote(Box<Expr>),
}

pub struct Parser<'a> {
    tokens: Peekable<std::slice::Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens: tokens.iter().peekable(),
        }
    }

    fn next_token(&mut self) -> Option<&'a Token> {
        self.tokens.next()
    }

    fn peek_token(&mut self) -> Option<&'a Token> {
        self.tokens.peek().cloned()
    }

    fn parse_list(&mut self) -> Result<Expr, String> {
        let mut list = Vec::new();
        while let Some(token) = self.peek_token() {
            match token {
                Token::CloseParen => {
                    self.next_token(); // Consume the closing parenthesis
                    return Ok(Expr::List(list));
                }
                _ => {
                    let expr = self.parse_expr()?;
                    list.push(expr);
                }
            }
        }
        Err("Unexpected end of input".to_string())
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        match self.next_token() {
            Some(Token::OpenParen) => self.parse_list(),
            Some(Token::Integer(value)) => Ok(Expr::Integer(*value)),
            Some(Token::Float(value)) => Ok(Expr::Float(*value)),
            Some(Token::String(value)) => Ok(Expr::String(value.clone())),
            Some(Token::Keyword(value)) => Ok(Expr::Keyword(value.clone())),
            Some(Token::Symbol(value)) => Ok(Expr::Symbol(value.clone())),
            Some(Token::Quote) => {
                let expr = self.parse_expr()?;
                Ok(Expr::Quote(Box::new(expr)))
            }
            Some(_) => Err("Unexpected token".to_string()),
            None => Err("Unexpected end of input".to_string()),
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.parse_expr()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse_integer() {
        let tokens = vec![Token::Integer(1)];
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse().unwrap();
        assert_eq!(expr, Expr::Integer(1));
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            let tokens = vec![
                Token::OpenParen,
                Token::Symbol("+".to_string()),
                Token::Integer(1),
                Token::Float(2.0),
                Token::OpenParen,
                Token::Symbol("-".to_string()),
                Token::Integer(3),
                Token::Integer(4),
                Token::CloseParen,
                Token::Quote,
                Token::Symbol("foo".to_string()),
                Token::Keyword("keyword".to_string()),
                Token::String("string".to_string()),
                Token::CloseParen,
            ];
            Expr::List([
                Expr::Symbol("+"),
                Expr::Integer(1),
                Expr::Float(2.0),
                Expr::List([Expr::Symbol("-"), Expr::Integer(3), Expr::Integer(4)]),
                Expr::Quote(Symbol("foo")),
                Expr::Keyword("keyword"),
                Expr::String("string")
            ])
        )
    }
}

use std::io::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LP,
    RP,
    Integer(i64),
    Float(f64),
    String(String),
    Operator(char),
    BinaryOperator(char),
}

fn token_integer(c: &str) -> Option<Token> {
    if let Ok(i) = c.to_string().parse::<i64>() {
        Some(Token::Integer(i))
    } else {
        None
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    use Token::*;
    let result: Vec<Token>;
    let chars = input.chars().collect::<Vec<char>>();
    let mut word: Vec<char> = Vec::new();
    result = chars
        .iter()
        .filter_map(|c| {
            word.clear();
            match c {
                '(' => Some(LP),
                ')' => Some(RP),
                _ => {
                    if let Some(i) = token_integer(c.to_string().as_str()) {
                        Some(i)
                    } else {
                        match c {
                            '+' => Some(BinaryOperator(*c)),
                            _ => None,
                        }
                    }
                }
            }
        })
        .collect();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    #[test]
    fn test_addition() {
        let tokenized = tokenize("(+ 1 2)");
        assert_eq!(
            tokenized.unwrap(),
            vec![LP, BinaryOperator('+'), Integer(1), Integer(2), RP]
        )
    }

    #[test]
    fn test_integer() {
        assert_eq!(token_integer("128"), Some(Integer(128)));
    }
}

use std::{io::Error, ops::Add};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LP,
    RP,
    Integer(i64),
    Float(f64),
    String(String),
    Operator(String),
    BinaryOperator(String),
    Keyword(String),
}

fn token_integer(c: &str) -> Option<Token> {
    if let Ok(i) = c.to_string().parse::<i64>() {
        Some(Token::Integer(i))
    } else {
        None
    }
}

fn keyword(input: String) -> Option<Token> {
    use Token::*;
    match input.as_str() {
        "float" => Some(Keyword("float".to_string())),
        "print" => Some(Keyword("print".to_string())),
        _ => None,
    }
}

fn single_character(input: &str) -> Option<Token> {
    use Token::*;
    match input {
        "(" => Some(LP),
        ")" => Some(RP),
        "+" | "-" | "*" | "/" => Some(Operator(input.to_string())),
        _ => {
            if let Some(i) = token_integer(input.to_string().as_str()) {
                Some(i)
            } else {
                None
            }
        }
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    let mut result: Vec<Token> = Vec::new();
    let mut items: Vec<&str> = input
        .split("")
        .into_iter()
        .filter(|i| !i.is_empty())
        .collect();
    let mut word: Vec<&str> = Vec::new();
    while items.len() > 0 {
        match items[0] {
            " " => {
                // If `word` isn't empty it means we have a keyword under construction.
                // We assume the keyword is constructed when we reach another whitespace.
                if !word.is_empty() {
                    if let Some(keyword) = keyword(word.join("")) {
                        result.push(keyword);
                    }
                    word.clear();
                }
                items.remove(0);
            }
            "\"" => {
                // A double quote `"` means the beginning of a string.
                items.remove(0); // Remove first instance of double quote
                let mut string = String::new();
                'make_string: loop {
                    // A string is ended with a double quote `"`
                    if items[0].contains("\"") {
                        result.push(Token::String(string));
                        items.remove(0);
                        break 'make_string;
                    } else {
                        string.push_str(items[0]);
                        items.remove(0);
                    }
                }
            }
            _ => match single_character(items[0]) {
                Some(t) => {
                    result.push(t);
                    items.remove(0);
                }
                None => {
                    word.push(items[0]);
                    items.remove(0);
                }
            },
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    /// Integer
    #[test]
    fn test_addition() {
        let tokenized = tokenize("(+ 1 2)");
        assert_eq!(
            tokenized.unwrap(),
            vec![LP, Operator("+".to_string()), Integer(1), Integer(2), RP]
        )
    }

    #[test]
    fn test_subtraction() {
        let tokenized = tokenize("(- 1 2)");
        assert_eq!(
            tokenized.unwrap(),
            vec![LP, Operator("-".to_string()), Integer(1), Integer(2), RP]
        )
    }

    #[test]
    fn test_integer() {
        assert_eq!(token_integer("128"), Some(Integer(128)));
    }

    /// Floating points
    #[test]
    fn test_float() {
        let tokenzied = tokenize("(float 1 5)");
        assert_eq!(
            tokenzied.unwrap(),
            vec![LP, Keyword("float".to_string()), Integer(1), Integer(5), RP]
        );
    }

    #[test]
    fn test_add_floats() {
        let tokenized = tokenize("(+ (float 1 5) (float 2 6))}");
        assert_eq!(
            tokenized.unwrap(),
            vec![
                LP,
                Operator("+".to_string()),
                LP,
                Keyword("float".to_string()),
                Integer(1),
                Integer(5),
                RP,
                LP,
                Keyword("float".to_string()),
                Integer(2),
                Integer(6),
                RP,
                RP,
            ]
        )
    }

    /// print
    #[test]
    fn test_print_hello_world() {
        let tokenized = tokenize("(print \"Hello, world!\")");
        assert_eq!(
            tokenized.unwrap(),
            vec![
                LP,
                Keyword("print".to_string()),
                String("Hello, world!".to_string()),
                RP
            ]
        );
    }
}

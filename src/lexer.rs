use std::io::Error;

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

fn keyword(input: &str) -> Option<Token> {
    use Token::*;
    match input {
        "float" => Some(Keyword("float".to_string())),
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
        if items[0].trim().is_empty() {
            if !word.is_empty() {
                result.push(keyword(word.join("").to_string().as_str()).unwrap());
                word.clear();
            }
            items.remove(0);
        } else {
            match single_character(items[0]) {
                Some(t) => {
                    result.push(t);
                    items.remove(0);
                }
                None => {
                    word.push(items[0]);
                    items.remove(0);
                }
            }
        }
    }
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

    #[test]
    fn test_float() {
        let tokenzied = tokenize("(float 1 5)");
        assert_eq!(
            tokenzied.unwrap(),
            vec![LP, Keyword("float".to_string()), Integer(1), Integer(5), RP]
        );
    }

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
            ]
        )
    }
}

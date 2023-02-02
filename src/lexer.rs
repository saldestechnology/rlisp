use std::io::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LP,
    RP,
    Number(i64),
    Float(f64),
    String(String),
    Operator(String),
    BinaryOperator(String),
    Keyword(String),
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
        _ => None,
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
        if let Some(c) = items[0].chars().next() {
            match c {
                ' ' => {
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
                '"' => {
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
                '1'..='9' => {
                    let mut number = String::new();
                    'make_string: loop {
                        // A number end with a whitespace ` `
                        let next = items[0].chars().next().unwrap();
                        if next.is_digit(10) {
                            number.push_str(items[0]);
                            items.remove(0);
                        } else {
                            result.push(Token::Number(number.parse::<i64>().unwrap()));
                            break 'make_string;
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
        } else {
            panic!("Unable to parse character.")
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
            vec![LP, Operator("+".to_string()), Number(1), Number(2), RP]
        )
    }

    #[test]
    fn test_subtraction() {
        let tokenized = tokenize("(- 1 2)");
        assert_eq!(
            tokenized.unwrap(),
            vec![LP, Operator("-".to_string()), Number(1), Number(2), RP]
        )
    }

    #[test]
    fn test_multi_digit_number() {
        let tokenized = tokenize("(+ 123456789 99999)");
        assert_eq!(
            tokenized.unwrap(),
            vec![
                LP,
                Operator("+".to_string()),
                Number(123456789),
                Number(99999),
                RP
            ]
        );
    }

    /// Floating points
    #[test]
    fn test_float() {
        let tokenzied = tokenize("(float 1 5)");
        assert_eq!(
            tokenzied.unwrap(),
            vec![LP, Keyword("float".to_string()), Number(1), Number(5), RP]
        );
    }

    #[test]
    fn test_multi_digit_decimal() {
        let tokenzied = tokenize("(float 1 564738)");
        assert_eq!(
            tokenzied.unwrap(),
            vec![
                LP,
                Keyword("float".to_string()),
                Number(1),
                Number(564738),
                RP
            ]
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
                Number(1),
                Number(5),
                RP,
                LP,
                Keyword("float".to_string()),
                Number(2),
                Number(6),
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

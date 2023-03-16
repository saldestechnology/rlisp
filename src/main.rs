mod lexer;
use lexer::Tokenizer;

fn main() {
    let input = "(+ 1 2.0 (- 3 4) 'foo :keyword \"string\" ; comment\n)";
    let tokenizer = Tokenizer::new(input);
    tokenizer.for_each(|f| println!("{:?}", f))
}

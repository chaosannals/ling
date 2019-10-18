#[path="../source/grammar/lexer.rs"]
mod lexer;

use lexer::Lexer;

fn main() {
    let path = String::from("example/base.ling");
    let mut lexer = Lexer::new();
    for lexeme in lexer.lex(path) {
        println!("({}, {}) => {:?}", lexeme.row, lexeme.column, lexeme.token);
    }
}
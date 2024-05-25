mod lexer;
mod parser;
use lexer::*;

fn main() {
    let expr = "!var * true ^ b + c (x ^ y -> var2 <-> _3)";
    let tokens = Lexer::new(expr).lex();
    println!("{tokens:#?}");
}

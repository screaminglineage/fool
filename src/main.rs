mod lexer;
mod parser;
use lexer::*;
use parser::*;

fn main() {
    // let expr = "!var * true ^ b + c ^ (x ^ y -> var2 <-> _3)";
    let expr = "(var1 + t) * f";
    let tokens = Lexer::new(expr).lex().unwrap();
    println!("{tokens:?}");
    let parser = Parser::new(tokens).parse();
    println!("{parser:#?}");
}

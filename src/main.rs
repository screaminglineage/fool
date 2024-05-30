mod eval;
mod lexer;
mod parser;
mod stringify;
use lexer::*;
use parser::*;

fn main() {
    // let expr = "!var * true ^ b + c ^ (x ^ y -> var2 <-> _3) ^ f * t";
    let expr = "a ^ (t + abcd) ^ e";
    let tokens = Lexer::new(expr).lex().unwrap();
    println!("{tokens:?}");
    let ast = Parser::new(tokens).parse().unwrap();
    // let ast = eval::simplify(ast);
    let str = stringify::stringify(ast);
    println!("{str}");
}

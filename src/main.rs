mod eval;
mod lexer;
mod parser;
mod stringify;
use lexer::*;
use parser::*;

fn main() {
    // let expr = "!var * t ^ b + c ^ (x ^ y -> var2 <-> _3) ^ f * t";
    // let expr = "(y*z + x*!z)*!(x*!y+z)";
    let expr = "a^b";
    let tokens = Lexer::new(expr).lex().unwrap();
    let ast = Parser::new(tokens).parse().unwrap();
    let ast = eval::simplify(ast);
    let str = stringify::stringify(ast);
    println!("{str:#?}");
}

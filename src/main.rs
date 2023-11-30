mod parser;
use parser::parse;

fn main() {
    let expr = "a. a";
    let t = parse(expr).evaluate();
    println!("{t:?}");
}

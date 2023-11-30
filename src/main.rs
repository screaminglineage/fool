mod parser;
use parser::parse;

fn main() {
    let expr = "a + b . !b";
    let t = parse(expr).evaluate();
    println!("{t:?}");
}

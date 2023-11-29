use fool::Boolean;
use fool::Expression::*;
use fool::Operation;

mod parser;
use parser::parse;

fn main() {
    let expr = "c.b +c . 1+ d";
    let t = parse(expr);
    println!("{t:?}");
}

fn evaluation() {
    let expr = Operation(Box::new(Operation::Or(
        Operation(Box::new(Operation::And(
            Variable('A'),
            Boolean(Boolean::One),
        ))),
        Operation(Box::new(Operation::And(
            Variable('A'),
            Boolean(Boolean::Zero),
        ))),
    )));

    let expr2 = Operation(Box::new(Operation::Or(Variable('B'), expr.evaluate())));

    let expr3 = Operation(Box::new(Operation::Not(Operation(Box::new(
        Operation::Or(Variable('A'), Variable('A')),
    )))));

    println!("{:?}", expr2.evaluate());
    println!("{:?}", expr3.evaluate());
}

#[derive(Debug, Clone, PartialEq)]
enum Symbol {
    Variable(char),
    Complement(char),
    Boolean(Boolean),
    Operation(Box<Operation>),
}

impl Symbol {
    fn evaluate(self) -> Symbol {
        match self {
            Symbol::Variable(a) => self,
            Symbol::Complement(a) => self,
            Symbol::Boolean(a) => self,
            Symbol::Operation(op) => op.evaluate(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Boolean {
    Zero,
    One,
}

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    And(Symbol, Symbol),
    Or(Symbol, Symbol),
    Not(Symbol),
}

impl Operation {
    fn evaluate(self) -> Symbol {
        match self {
            Operation::And(a, b) => logical_and(a.evaluate(), b.evaluate()),
            Operation::Or(a, b) => logical_or(a.evaluate(), b.evaluate()),
            Operation::Not(a) => logical_not(a.evaluate()),
        }
    }
}

fn logical_or(op_a: Symbol, op_b: Symbol) -> Symbol {
    match (op_a, op_b) {
        (Symbol::Boolean(Boolean::Zero), a) | (a, Symbol::Boolean(Boolean::Zero)) => a,
        (Symbol::Boolean(Boolean::One), _) | (_, Symbol::Boolean(Boolean::One)) => {
            Symbol::Boolean(Boolean::One)
        }

        // simplify same variables
        (Symbol::Variable(a), Symbol::Variable(b)) if a == b => Symbol::Variable(a),
        (Symbol::Complement(a), Symbol::Complement(b)) if a == b => Symbol::Complement(a),

        // revert back to tree structure for different variables
        (Symbol::Variable(a), Symbol::Variable(b))
        | (Symbol::Complement(a), Symbol::Complement(b))
            if a != b => {
                Symbol::Operation(Box::new(Operation::Or(Symbol::Variable(a), Symbol::Variable(b))))
            }

        // A + !A = 1
        (Symbol::Variable(a), Symbol::Complement(b))
        | (Symbol::Complement(b), Symbol::Variable(a))
            if a == b =>
        {
            Symbol::Boolean(Boolean::One)
        }

        (Symbol::Operation(op), a) | (a, Symbol::Operation(op)) => logical_or(op.evaluate(), a),
        (Symbol::Operation(a), Symbol::Operation(b)) => logical_or(a.evaluate(), b.evaluate()),
        _ => unreachable!(),
    }
}

fn logical_and(op_a: Symbol, op_b: Symbol) -> Symbol {
    match (op_a, op_b) {
        (Symbol::Boolean(Boolean::Zero), _) | (_, Symbol::Boolean(Boolean::Zero)) => {
            Symbol::Boolean(Boolean::Zero)
        }
        (Symbol::Boolean(Boolean::One), a) | (a, Symbol::Boolean(Boolean::One)) => a,

        // simplify same variables
        (Symbol::Variable(a), Symbol::Variable(b)) if a == b => Symbol::Variable(a),
        (Symbol::Complement(a), Symbol::Complement(b)) if a == b => Symbol::Complement(a),

        // revert back to tree structure for different variables
        (Symbol::Variable(a), Symbol::Variable(b))
        | (Symbol::Complement(a), Symbol::Complement(b))
            if a != b => {
                Symbol::Operation(Box::new(Operation::And(Symbol::Variable(a), Symbol::Variable(b))))
            }

        // A . !A = 0
        (Symbol::Variable(a), Symbol::Complement(b))
        | (Symbol::Complement(b), Symbol::Variable(a))
            if a == b =>
        {
            Symbol::Boolean(Boolean::Zero)
        }

        (Symbol::Operation(op), a) | (a, Symbol::Operation(op)) => logical_and(op.evaluate(), a),
        (Symbol::Operation(a), Symbol::Operation(b)) => logical_and(a.evaluate(), b.evaluate()),
        _ => unreachable!(),
    }
}

fn logical_not(op_a: Symbol) -> Symbol {
    match op_a {
        Symbol::Boolean(Boolean::Zero) => Symbol::Boolean(Boolean::One),
        Symbol::Boolean(Boolean::One) => Symbol::Boolean(Boolean::Zero),
        Symbol::Variable(a) => Symbol::Complement(a),
        Symbol::Operation(_) => Symbol::Operation(Box::new(Operation::Not(op_a))),
        _ => unreachable!(),
    }
}

use Symbol::*;
fn main() {
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

#[derive(Debug, Clone, PartialEq)]
enum Expression {
    Variable(char),
    Complement(char),
    Boolean(Boolean),
    Operation(Box<Operation>),
}

use Expression::*;

impl Expression {
    fn evaluate(self) -> Expression {
        match self {
            Variable(_) => self,
            Complement(_) => self,
            Boolean(_) => self,
            Operation(op) => op.evaluate(),
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
    And(Expression, Expression),
    Or(Expression, Expression),
    Not(Expression),
}

impl Operation {
    fn evaluate(self) -> Expression {
        match self {
            Operation::And(a, b) => logical_and(a.evaluate(), b.evaluate()),
            Operation::Or(a, b) => logical_or(a.evaluate(), b.evaluate()),
            Operation::Not(a) => logical_not(a.evaluate()),
        }
    }
}

fn logical_or(op_a: Expression, op_b: Expression) -> Expression {
    match (op_a, op_b) {
        // logical or operations
        (Boolean(Boolean::Zero), a) | (a, Boolean(Boolean::Zero)) => a,
        (Boolean(Boolean::One), _) | (_, Boolean(Boolean::One)) => Boolean(Boolean::One),

        // simplify if both variables are the same
        (Variable(a), Variable(b)) if a == b => Variable(a),
        (Complement(a), Complement(b)) if a == b => Complement(a),

        // keep the previous structure if the variables are
        // different as no simplification can be done
        (Variable(a), Variable(b)) | (Complement(a), Complement(b)) if a != b => {
            Operation(Box::new(Operation::Or(Variable(a), Variable(b))))
        }

        // A + !A = 1
        (Variable(a), Complement(b)) | (Complement(b), Variable(a)) if a == b => {
            Boolean(Boolean::One)
        }

        // recursively evaluate the operations
        (Operation(a), Operation(b)) => logical_or(a.evaluate(), b.evaluate()),
        (Operation(op), a) | (a, Operation(op)) => logical_or(op.evaluate(), a),
        _ => unreachable!(),
    }
}

fn logical_and(op_a: Expression, op_b: Expression) -> Expression {
    match (op_a, op_b) {
        // logical and operations
        (Boolean(Boolean::Zero), _) | (_, Boolean(Boolean::Zero)) => Boolean(Boolean::Zero),
        (Boolean(Boolean::One), a) | (a, Boolean(Boolean::One)) => a,

        // simplify if both variables are the same
        (Variable(a), Variable(b)) if a == b => Variable(a),
        (Complement(a), Complement(b)) if a == b => Complement(a),

        // keep the previous structure if the variables are
        // different as no simplification can be done
        (Variable(a), Variable(b)) | (Complement(a), Complement(b)) if a != b => {
            Operation(Box::new(Operation::And(Variable(a), Variable(b))))
        }

        // A . !A = 0
        (Variable(a), Complement(b)) | (Complement(b), Variable(a)) if a == b => {
            Boolean(Boolean::Zero)
        }

        // recursively evaluate the operations
        (Operation(a), Operation(b)) => logical_and(a.evaluate(), b.evaluate()),
        (Operation(op), a) | (a, Operation(op)) => logical_and(op.evaluate(), a),
        _ => unreachable!(),
    }
}

fn logical_not(op_a: Expression) -> Expression {
    match op_a {
        Boolean(Boolean::Zero) => Boolean(Boolean::One),
        Boolean(Boolean::One) => Boolean(Boolean::Zero),
        Variable(a) => Complement(a),
        Operation(_) => Operation(Box::new(Operation::Not(op_a))),
        _ => unreachable!(),
    }
}

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

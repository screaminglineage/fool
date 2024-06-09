use crate::parser::*;

pub fn simplify(expr: Expr) -> Expr {
    match expr {
        Expr::Value(_) | Expr::Variable(_) => expr,
        Expr::Op(op) => simplify_op(op),
        Expr::Group(group) => {
            let new = simplify(*group);
            if let Expr::Value(_) | Expr::Variable(_) | Expr::Op(Op::Not(_)) = new {
                return new;
            } else {
                return Expr::Group(Box::new(new));
            }
        }
    }
}

fn simplify_op(op: Op) -> Expr {
    match op {
        Op::Not(expr) => simplify_not(*expr),
        Op::Binary(BinaryOp::Or(left, right)) => simplify_or(*left, *right),
        Op::Binary(BinaryOp::And(left, right)) => simplify_and(*left, *right),
        Op::Binary(BinaryOp::Xor(left, right)) => simplify_xor(*left, *right),
        Op::Binary(BinaryOp::Implication(left, right)) => simplify_implication(*left, *right),
        Op::Binary(BinaryOp::Biconditional(left, right)) => simplify_biconditional(*left, *right),
    }
}

use BooleanValue::False as F;
use BooleanValue::True as T;

fn simplify_not(expr: Expr) -> Expr {
    match expr {
        Expr::Value(T) => Expr::Value(F),
        Expr::Value(F) => Expr::Value(T),
        e => {
            let e = simplify(e);
            return match e {
                Expr::Value(_) => simplify_not(e),
                Expr::Op(Op::Not(a)) => *a.clone(),
                _ => Expr::Op(Op::Not(Box::new(e))),
            };
        }
    }
}

fn simplify_or(left: Expr, right: Expr) -> Expr {
    match (left, right) {
        (Expr::Value(F), Expr::Value(F)) => Expr::Value(F),
        (Expr::Value(T), _) | (_, Expr::Value(T)) => Expr::Value(T),
        (Expr::Value(F), expr) | (expr, Expr::Value(F)) => simplify(expr),

        (l, r) => {
            let (l, r) = (simplify(l), simplify(r));
            if let (Expr::Value(_), _) | (_, Expr::Value(_)) = (l.clone(), r.clone()) {
                return simplify_or(l, r);
            } else {
                return Expr::Op(Op::Binary(BinaryOp::Or(Box::new(l), Box::new(r))));
            }
        }
    }
}

fn simplify_and(left: Expr, right: Expr) -> Expr {
    match (left, right) {
        (Expr::Value(T), Expr::Value(T)) => Expr::Value(T),
        (Expr::Value(T), expr) | (expr, Expr::Value(T)) => simplify(expr),
        (Expr::Value(F), _) | (_, Expr::Value(F)) => Expr::Value(F),

        (l, r) => {
            let (l, r) = (simplify(l), simplify(r));
            if let (Expr::Value(_), _) | (_, Expr::Value(_)) = (l.clone(), r.clone()) {
                return simplify_and(l, r);
            } else {
                return Expr::Op(Op::Binary(BinaryOp::And(Box::new(l), Box::new(r))));
            }
        }
    }
}

fn simplify_xor(left: Expr, right: Expr) -> Expr {
    match (left, right) {
        (Expr::Value(a), Expr::Value(b)) if a == b => Expr::Value(F),
        (Expr::Value(a), Expr::Value(b)) if a != b => Expr::Value(T),

        // a ^ False = a
        // a ^ True = !a
        (Expr::Value(val), expr) | (expr, Expr::Value(val)) => {
            if val == F {
                return simplify(expr);
            } else {
                return Expr::Op(Op::Not(Box::new(simplify(expr))));
            }
        }
        (l, r) => {
            let (l, r) = (simplify(l), simplify(r));
            if let (Expr::Value(_), _) | (_, Expr::Value(_)) = (l.clone(), r.clone()) {
                return simplify_xor(l, r);
            } else {
                return Expr::Op(Op::Binary(BinaryOp::Xor(Box::new(l), Box::new(r))));

                // TODO: Use this instead to make it easier to further simplify expressions
                // return Expr::Op(Op::Binary(BinaryOp::Or(
                //     Box::new(Expr::Op(Op::Binary(BinaryOp::And(
                //         Box::new(Expr::Op(Op::Not(Box::new(l.clone())))),
                //         Box::new(r.clone()),
                //     )))),
                //     Box::new(Expr::Op(Op::Binary(BinaryOp::And(
                //         Box::new(r),
                //         Box::new(Expr::Op(Op::Not(Box::new(l)))),
                //     )))),
                // )));
            }
        }
    }
}

// TODO: Create a macro to not have to call simplify_ functions again
// Macro should replace with Or(Not(left), right), etc

fn simplify_implication(left: Expr, right: Expr) -> Expr {
    simplify_or(simplify_not(left), right)
}

fn simplify_biconditional(left: Expr, right: Expr) -> Expr {
    simplify_not(simplify_xor(left, right))
}

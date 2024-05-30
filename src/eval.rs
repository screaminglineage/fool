use crate::parser::*;

pub fn simplify(expr: Expr) -> Expr {
    match expr {
        Expr::Value(_) | Expr::Variable(_) => expr,
        Expr::Op(op) => simplify_op(*op),
        Expr::Group(group) => simplify(*group),
    }
}

fn simplify_op(op: Op) -> Expr {
    match op {
        Op::Not(expr) => simplify_not(expr),
        Op::Binary(BinaryOp::Or(left, right)) => simplify_or(left, right),
        Op::Binary(BinaryOp::And(left, right)) => simplify_and(left, right),
        Op::Binary(BinaryOp::Xor(left, right)) => simplify_xor(left, right),
        Op::Binary(BinaryOp::Implies(left, right)) => simplify_implies(left, right),
        Op::Binary(BinaryOp::Biconditional(left, right)) => simplify_biconditional(left, right),
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
            if let Expr::Value(_) = e {
                return simplify_not(e);
            } else {
                return Expr::Op(Box::new(Op::Not(e)));
            }
        }
    }
}

fn simplify_or(left: Expr, right: Expr) -> Expr {
    match (left, right) {
        (Expr::Value(F), Expr::Value(F)) => Expr::Value(F),
        (Expr::Value(_), Expr::Value(_)) => Expr::Value(T),
        (Expr::Value(T), _) | (_, Expr::Value(T)) => Expr::Value(T),
        (Expr::Value(F), expr) | (expr, Expr::Value(F)) => simplify(expr),

        (l, r) => {
            let (l, r) = (simplify(l), simplify(r));
            if let (Expr::Value(_), _) | (_, Expr::Value(_)) = (l.clone(), r.clone()) {
                return simplify_or(l, r);
            } else {
                return Expr::Op(Box::new(Op::Binary(BinaryOp::Or(l, r))));
            }
        }
    }
}

fn simplify_and(left: Expr, right: Expr) -> Expr {
    match (left, right) {
        (Expr::Value(T), Expr::Value(T)) => Expr::Value(T),
        (Expr::Value(_), Expr::Value(_)) => Expr::Value(F),
        (Expr::Value(T), expr) | (expr, Expr::Value(T)) => simplify(expr),
        (Expr::Value(F), _) | (_, Expr::Value(F)) => Expr::Value(F),

        (l, r) => {
            let (l, r) = (simplify(l), simplify(r));
            if let (Expr::Value(_), _) | (_, Expr::Value(_)) = (l.clone(), r.clone()) {
                return simplify_and(l, r);
            } else {
                return Expr::Op(Box::new(Op::Binary(BinaryOp::And(l, r))));
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
                return Expr::Op(Box::new(Op::Not(simplify(expr))));
            }
        }
        (l, r) => {
            let (l, r) = (simplify(l), simplify(r));
            if let (Expr::Value(_), _) | (_, Expr::Value(_)) = (l.clone(), r.clone()) {
                return simplify_xor(l, r);
            } else {
                return Expr::Op(Box::new(Op::Binary(BinaryOp::Xor(l, r))));
            }
        }
    }
}
fn simplify_implies(left: Expr, right: Expr) -> Expr {
    todo!()
}
fn simplify_biconditional(left: Expr, right: Expr) -> Expr {
    todo!()
}
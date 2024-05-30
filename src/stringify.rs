use crate::parser::*;

pub fn stringify(expr: Expr) -> String {
    match expr {
        Expr::Value(a) => match a {
            BooleanValue::True => format!("t"),
            BooleanValue::False => format!("f"),
        },
        Expr::Variable(a) => format!("{a}"),
        Expr::Op(op) => match *op {
            Op::Not(a) => format!("!{}", stringify(a)),
            Op::Binary(BinaryOp::Or(left, right)) => {
                format!("{} + {}", stringify(left), stringify(right))
            }
            Op::Binary(BinaryOp::And(left, right)) => {
                format!("{} * {}", stringify(left), stringify(right))
            }
            Op::Binary(BinaryOp::Xor(left, right)) => {
                format!("{} ^ {}", stringify(left), stringify(right))
            }
            Op::Binary(BinaryOp::Implies(left, right)) => {
                format!("{} -> {}", stringify(left), stringify(right))
            }
            Op::Binary(BinaryOp::Biconditional(left, right)) => {
                format!("{} <-> {}", stringify(left), stringify(right))
            }
        },
        Expr::Group(group) => format!("({})", stringify(*group)),
    }
}

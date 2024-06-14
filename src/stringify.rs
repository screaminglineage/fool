use crate::parser::*;

pub fn stringify(expr: Expr) -> String {
    match expr {
        Expr::Value(a) => match a {
            BooleanValue::True => format!("t"),
            BooleanValue::False => format!("f"),
        },
        Expr::Variable(a) => format!("{a}"),
        Expr::Op(op) => match op {
            Op::Not(op) => {
                // skip brackets if a value or variable is within not
                // Not(x) => !x, Not(And(a, b)) => !(a * b)
                if let Expr::Op(_) = *op {
                    format!("!({})", stringify(*op))
                } else {
                    format!("!{}", stringify(*op))
                }
            }
            Op::Or(left, right) => {
                format!("{} + {}", stringify(*left), stringify(*right))
            }
            Op::And(left, right) => {
                format!("{} * {}", stringify(*left), stringify(*right))
            }
            Op::Xor(left, right) => {
                format!("{} ^ {}", stringify(*left), stringify(*right))
            }
            Op::Implication(left, right) => {
                format!("{} -> {}", stringify(*left), stringify(*right))
            }
            Op::Biconditional(left, right) => {
                format!("{} <-> {}", stringify(*left), stringify(*right))
            }
        },
        Expr::Group(group) => format!("({})", stringify(*group)),
    }
}

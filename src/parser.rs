use fool::Boolean;
use fool::Expression;
use fool::Operation;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Variable(char),
    Bool(Boolean),
    Not,
    Op(Operator),
    WhiteSpace,
}

impl Token {
    fn is_operator(&self) -> bool {
        match self {
            Self::Variable(_) => false,
            Self::Bool(_) => false,
            Self::WhiteSpace => false,
            _ => true,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    And,
    Or,
    // Xor
}

pub fn parse(expr: &str) -> Expression {
    let tokens = remove_whitespace(create_tokens(expr));
    if tokens.is_empty() {
        panic!("Error: Empty Expression")
    }
    if valid_syntax(&tokens) {
        println!("{tokens:?}");
        return create_expr(tokens);
    } else {
        panic!("Error: Malformed Expression")
    }
}

fn create_tokens(expr: &str) -> Vec<Token> {
    expr.chars()
        .map(|c| match c {
            '0' => Token::Bool(Boolean::Zero),
            '1' => Token::Bool(Boolean::One),
            '!' => Token::Not,
            '.' => Token::Op(Operator::And),
            '+' => Token::Op(Operator::Or),
            // '^' => Token::Op(Operation::Xor),
            a if a.is_alphabetic() => Token::Variable(a),
            a if a.is_whitespace() => Token::WhiteSpace,
            _ => panic!("Error: Unrecognised Token"),
        })
        .collect()
}

fn remove_whitespace(tokens: Vec<Token>) -> Vec<Token> {
    tokens
        .into_iter()
        .filter(|t| *t != Token::WhiteSpace)
        .collect()
}

fn valid_syntax(tokens: &[Token]) -> bool {
    use Token as t;
    // first token cannot be an operator
    if let t::Op(_) = tokens[0] {
        return false;
    }

    // tokens is not empty so it's safe to unwrap
    // last token cannot be an operator or `NOT`
    match tokens.last().unwrap() {
        t::Not => return false,
        t::Op(_) => return false,
        _ => (),
    }

    // two variables or booleans cannot be placed consecutively
    // similarly two operators or `NOT` and an operator cannot be
    // placed consecutively
    for slice in tokens.windows(2) {
        match slice[0] {
            t::Variable(_) | t::Bool(_) if !matches!(slice[1], t::Op(_)) => return false,
            t::Not | t::Op(_) if matches!(slice[1], t::Op(_)) => return false,
            _ => (),
        }
    }
    true
}

fn precedence(token: &Token) -> u8 {
    match token {
        Token::Not => 3,
        Token::Op(Operator::And) => 2,
        Token::Op(Operator::Or) => 1,
        _ => panic!("invalid operation"),
    }
}

fn create_expr_from_operator(op: Token, var_stack: &mut Vec<Expression>) {
    use Expression as e;
    use Token as t;
    match op {
        t::Not => {
            let expr = var_stack.pop().expect("var_stack has atleast 1 element");
            let new_expr = e::Operation(Box::new(Operation::Not(expr)));
            var_stack.push(new_expr);
        }
        t::Op(op) => {
            let expr_1 = var_stack.pop().expect("var_stack has atleast 2 elements");
            let expr_2 = var_stack.pop().expect("var_stack has atleast 2 elements");
            let new_expr = match op {
                Operator::And => Operation::And(expr_1, expr_2),
                Operator::Or => Operation::Or(expr_1, expr_2),
            };
            let new_expr = e::Operation(Box::new(new_expr));
            var_stack.push(new_expr);
        }
        _ => unreachable!("only operators are handled"),
    }
}

fn create_expr(tokens: Vec<Token>) -> Expression {
    let mut var_stack: Vec<Expression> = Vec::new();
    let mut op_stack: Vec<Token> = Vec::new();

    use Expression as e;
    use Token as t;
    for token in tokens {
        if token.is_operator() {
            while !op_stack.is_empty()
                && precedence(&token) < precedence(op_stack.last().expect("op_stack is non-empty"))
            {
                let op = op_stack.pop().expect("op_stack is non-empty");
                create_expr_from_operator(op, &mut var_stack);
            }
            op_stack.push(token);
        } else {
            match token {
                t::Variable(a) => var_stack.push(e::Variable(a)),
                t::Bool(a) => var_stack.push(e::Boolean(a)),
                _ => (),
            }
        }
    }

    while !op_stack.is_empty() {
        let op = op_stack.pop().expect("op_stack is non-empty");
        create_expr_from_operator(op, &mut var_stack);
    }
    var_stack
        .pop()
        .expect("only 1 element left in variable stack")
}

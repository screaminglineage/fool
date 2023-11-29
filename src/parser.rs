#[derive(Debug, PartialEq)]
pub enum Token {
    Variable(char),
    Bool(Boolean),
    Not,
    Op(Operator),
    WhiteSpace,
}

#[derive(Debug, PartialEq)]
pub enum Boolean {
    Zero,
    One,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    And,
    Or,
    // Xor
}

pub fn parse(expr: &str) -> Vec<Token> {
    let tokens = remove_whitespace(create_tokens(expr));
    if tokens.is_empty() {
        panic!("Error: Empty Expression")
    }
    if valid_syntax(&tokens) {
        return tokens;
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

use crate::lexer::*;
use TokenKind as tk;

#[derive(Debug, Clone)]
pub enum Expr {
    Value(BooleanValue),
    Variable(String),
    Op(Op),
    Group(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Op {
    Not(Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
    Implication(Box<Expr>, Box<Expr>),
    Biconditional(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BooleanValue {
    True,
    False,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(mut self) -> Option<Expr> {
        let expr = self.conditional();
        // parsing error
        if let None = expr {
            return None;
        }
        // parsed correctly and completely
        if let Some(Token { kind: tk::EOF, .. }) = self.peek() {
            return expr;
        } 
        // didnt parse completely
        let extra = self.peek().unwrap();
        eprintln!("Unexpected token '{:?}' at end", extra.kind.clone()); 
        return None;
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn advance(&mut self) -> Option<&Token> {
        if let Some(_) = self.peek() {
            self.current += 1;
        }
        return self.previous();
    }

    fn check(&self, token_kind: &TokenKind) -> bool {
        if let Some(t) = self.peek() {
            *token_kind == t.kind
        } else {
            false
        }
    }

    fn expect_tokens(&mut self, tokens: &[TokenKind]) -> Option<TokenKind> {
        for token in tokens {
            if self.check(token) {
                return self.advance().map(|tok| tok.kind.clone());
            }
        }
        return None;
    }

    fn conditional(&mut self) -> Option<Expr> {
        let mut left = self.or()?;
        while let Some(token) = self.expect_tokens(&[tk::Arrow, tk::DoubleArrow]) {
            let right = self.or()?;
            match token {
                tk::Arrow => left = Expr::Op(Op::Implication(Box::new(left), Box::new(right))),
                tk::DoubleArrow => {
                    left = Expr::Op(Op::Biconditional(Box::new(left), Box::new(right)))
                }
                _ => unreachable!(),
            }
        }
        Some(left)
    }

    fn or(&mut self) -> Option<Expr> {
        let mut left = self.xor()?;
        while let Some(_) = self.expect_tokens(&[tk::Plus]) {
            let right = self.xor()?;
            left = Expr::Op(Op::Or(Box::new(left), Box::new(right)));
        }
        Some(left)
    }

    fn xor(&mut self) -> Option<Expr> {
        let mut left = self.and()?;
        while let Some(_) = self.expect_tokens(&[tk::Caret]) {
            let right = self.and()?;
            left = Expr::Op(Op::Xor(Box::new(left), Box::new(right)));
        }
        Some(left)
    }

    fn and(&mut self) -> Option<Expr> {
        let mut left = self.not()?;
        while let Some(_) = self.expect_tokens(&[tk::Star]) {
            let right = self.not()?;
            left = Expr::Op(Op::And(Box::new(left), Box::new(right)));
        }
        Some(left)
    }

    fn not(&mut self) -> Option<Expr> {
        let mut count = 0;
        while let Some(_) = self.expect_tokens(&[tk::Bang]) {
            count += 1;
        }
        let right = self.primary()?;
        if count > 0 {
            if count % 2 == 0 {
                return Some(right);
            } else {
                return Some(Expr::Op(Op::Not(Box::new(right))));
            }
        }
        Some(right)
    }

    #[rustfmt::skip]
    fn primary(&mut self) -> Option<Expr> {
        match self.advance() {
            Some(Token { kind: tk::True, .. }) => Some(Expr::Value(BooleanValue::True)),
            Some(Token { kind: tk::False, .. }) => Some(Expr::Value(BooleanValue::False)),
            Some(Token {kind: tk::Identifier(val), .. }) => Some(Expr::Variable(val.to_owned())),

            Some(Token { kind: tk::OpenParen, .. }) => {
                let inner = self.conditional()?;
                if self.check(&tk::CloseParen) {
                    self.advance();
                    return Some(Expr::Group(Box::new(inner)));
                } else {
                    eprintln!(
                        "Expected ')' after {:?}, found: {:?}", 
                        self.previous().map(|tok| tok.kind.clone()), 
                        self.peek().map(|tok| tok.kind.clone())
                    );
                    return None;
                }
            }
            _ => {
                eprintln!(
                    "Expected true, false or identifier after {:?}, found: '{:?}'", 
                    self.tokens.get(self.current.checked_sub(2)?).map(|tok| tok.kind.clone()), 
                    self.previous().map(|tok| tok.kind.clone())
                );
                return None;
            }
        }
    }
}

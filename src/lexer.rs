#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    OpenParen,
    CloseParen,
    Bang,
    Plus,
    Star,
    Caret,
    Arrow,
    DoubleArrow,
    True,
    False,
    // TODO: Store the string as a separate, 'value', field in Token struct
    Identifier(String),
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub position: usize,
}

impl Token {
    fn new(kind: TokenKind, position: usize) -> Self {
        Self { kind, position }
    }
}

pub struct Lexer {
    source: Vec<char>,
    current: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            current: 0,
        }
    }

    fn next<'a>(&'a mut self) -> Option<&'a char> {
        let next = self.source.get(self.current);
        self.current += 1;
        return next;
    }

    fn peek<'a>(&'a self) -> Option<&'a char> {
        return self.source.get(self.current);
    }

    fn peek_next<'a>(&'a self) -> Option<&'a char> {
        if self.current >= self.source.len() - 1 {
            return None;
        }
        return self.source.get(self.current + 1);
    }

    fn identifier(&mut self, tokens: &mut Vec<Token>) -> Option<()> {
        let start_index = self.current;
        let Some(first) = self.next() else {
            return None;
        };
        // check if first character is not alphabetic or '_'
        if !(first.is_alphabetic() || *first == '_') {
            eprintln!(
                "Expected alphabetic character or '_' at the beginning of identifier at index: {}",
                self.current - 1
            );
            return None;
        }
        while let Some(&ch) = self.peek() {
            if !(ch.is_alphanumeric() || ch == '_') {
                break;
            }
            self.next();
        }

        let string: String = self.source[start_index..self.current].iter().collect();

        // check for keywords
        let token = match string.as_str() {
            "true" | "t" | "1" => Token::new(TokenKind::True, start_index),
            "false" | "f" | "0" => Token::new(TokenKind::False, start_index),
            _ => Token::new(TokenKind::Identifier(string), start_index),
        };
        tokens.push(token);
        Some(())
    }

    pub fn lex(mut self) -> Option<Vec<Token>> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.peek() {
            use TokenKind::*;
            match ch {
                '(' => tokens.push(Token::new(OpenParen, self.current)),
                ')' => tokens.push(Token::new(CloseParen, self.current)),
                '!' => tokens.push(Token::new(Bang, self.current)),
                '+' => tokens.push(Token::new(Plus, self.current)),
                '*' => tokens.push(Token::new(Star, self.current)),
                '^' => tokens.push(Token::new(Caret, self.current)),
                // parse '->'
                '-' => {
                    if let Some('>') = self.peek_next() {
                        self.next();
                        tokens.push(Token::new(Arrow, self.current));
                    } else {
                        eprintln!("Expected '>' after '-' token at index: {}", self.current);
                        return None;
                    }
                }
                // parse '<->'
                '<' => {
                    if let Some('-') = self.peek_next() {
                        self.next();
                        if let Some('>') = self.peek_next() {
                            self.next();
                            tokens.push(Token::new(DoubleArrow, self.current));
                        } else {
                            eprintln!("Expected '>' after '<-' token at index: {}", self.current);
                            return None;
                        }
                    } else {
                        eprintln!("Expected '-' after '<' token at index: {}", self.current);
                        return None;
                    }
                }
                c if c.is_whitespace() => {}
                '0' | '1' | 'a'..='z' | 'A'..='Z' | '_' => {
                    self.identifier(&mut tokens)?;
                    continue;
                }
                c => {
                    eprintln!("Unexpected character '{c}' at index: {}", self.current);
                    return None;
                }
            }
            self.next();
        }
        tokens.push(Token::new(TokenKind::EOF, self.current));
        return Some(tokens);
    }
}

use crate::types::{Expr, Program, Stmt, Token};

pub struct ParseError {
    pub message: String,
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    fn consume(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        self.pos += 1;
        token
    }
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut body = Vec::new();

        while !matches!(self.peek(), Some(Token::Eof)) {
            body.push(self.parse_stmt()?);
        }
        Ok(Program {
            statements: body,
            imports: vec![],
            module_name: None,
        })
    }

    pub fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        Err(ParseError {
            message: "not implemented yet".to_string(),
        })
    }
}

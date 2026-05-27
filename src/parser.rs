use std::{collections::binary_heap::Iter, iter::Peekable};

use crate::types::{Expr, KeywordKind, OperKind, Program, Stmt, Token, Type};

#[derive(PartialEq)]
enum ParseState {
    Default,
    InExpr,
    InLet,
    InIf,
    InBlock,
}

pub struct ParseError {
    pub message: String,
}
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }
    fn next(&mut self) {
        self.pos += 1;
    }
    fn peek(&mut self) -> Option<&Token> {
        return Some(&self.tokens[self.pos + 1]);
    }
    fn advance(&mut self) -> Option<Token> {
        self.pos += 1;
        return self.tokens.get(self.pos - 1).cloned();
    }
    fn parse_primary(&mut self) -> Option<Expr> {
        match self.peek() {
            Some(Token::NumberLit(n, _))
        }
    }
    fn parse_expr(&mut self) -> Option<Expr> {
        return None;
    }
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let tokens = self.tokens;
        let mut instructions: Vec<Expr> = Vec::new();
        let mut state: ParseState = ParseState::Default;

        while let Some(token) = self.peek().cloned() {
            match state {
                ParseState::Default => match token {
                    Token::Keyword(KeywordKind::Let) => {
                        state = ParseState::InLet;
                        self.next();
                    }
                    Token::Keyword(KeywordKind::If) => {
                        state = ParseState::InIf;
                        self.next();
                    }
                    Token::LParen => {
                        state = ParseState::InBlock;
                        self.next();
                    }
                    _ => state = ParseState::InExpr,
                },
                ParseState::InLet => match token {
                    Token::Ident(name) => {
                        let mut ty: Option<Type> = None;
                        let var_name = name;
                        let val: Option<Expr> = None;
                        self.next();
                        if matches!(self.peek(), Some(Token::Colon)) {
                            self.next();
                            match self.peek() {
                                Some(Token::Type(n)) => {
                                    ty = Some(n.clone());
                                    self.next();
                                    match self.peek() {
                                        Some(Token::Semicolon) => {
                                            self.next();
                                            val = None;
                                        }
                                        Some(Token::Oper(OperKind::Assign)) => {
                                            self.next();
                                            val = self.parse_expr();
                                        }
                                        _ => println!("Uncorrect Let"),
                                    }
                                }
                            }
                        } else {
                            ty = None;
                            match self.peek() {
                                Some(Token::Oper(OperKind::Assign)) => {
                                    self.next();
                                    val = self.parse_expr();
                                }
                                _ => println!("Undefined Assign '='"),
                            }
                        }
                    }
                    _ => println!("чет пошло не так!"),
                },
                ParseState::InIf => match token {
                    Token::LParen => {
                        self.pos += 1;
                        self.parse_expr();
                    }
                    _ => {
                        println!("Заглушка");
                    }
                },
                _ => println!("Чет не то состояние парсера!"),
            }
        }

        return Ok(Program {
            statements: instructions,
            imports: vec![],
            module_name: None,
        });
    }
}

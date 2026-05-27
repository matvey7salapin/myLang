use crate::types::{KeywordKind, OperKind, Span, Token, TokenWithSpan, Type};
#[derive(PartialEq)]
enum LexState {
    Default,
    InString,
    InIdent,
    InNumber,
    InOper,
}

pub struct Lexer {
    tokens: Vec<TokenWithSpan>,
    chars: Vec<char>,
    pos: usize,

    // Позиция
    line: usize,
    col: usize,

    // Состояние
    state: LexState,

    // Буферы
    ident_buf: String,
    num_buf: String,
    str_buf: String,
}

impl Lexer {
    pub fn new(code: &str) -> Self {
        Self {
            tokens: Vec::new(),
            chars: code.chars().collect(),
            pos: 0,

            line: 1,
            col: 1,

            state: LexState::Default,

            ident_buf: String::new(),
            num_buf: String::new(),
            str_buf: String::new(),
        }
    }
    pub fn get_tokens_with_span(self) -> Vec<TokenWithSpan> {
        return self.tokens;
    }
    fn push_token_with_span(&mut self, token: Token) {
        self.tokens.push(TokenWithSpan {
            token: token,
            span: Span {
                line: self.line,
                col: self.col,
            },
        });
    }
    fn new_line(&mut self) -> usize {
        self.line += 1;
        self.col = 1;

        return self.line;
    }
    fn next(&mut self) {
        self.pos += 1;
    }
    fn peek(&self) -> Option<&char> {
        return Some(&self.chars[self.pos + 1]);
    }
    fn advance(&mut self) -> Option<char> {
        self.pos += 1;
        return self.chars.get(self.pos - 1).cloned();
    }
    fn tokenize_number(&mut self) {
        if let Some(&c) = self.peek() {
            if c.is_ascii_digit() || c == '_' || c == '.' {
                if c != '_' {
                    self.num_buf.push(c);
                }
                self.next();
            } else {
                if self.num_buf.contains('.') {
                    match self.num_buf.parse::<f64>() {
                        Ok(n) => self.push_token_with_span(Token::FloatLit(n)),
                        Err(_) => eprintln!("Warning: invalid float '{}'", self.num_buf),
                    }
                } else {
                    match self.num_buf.parse::<u64>() {
                        Ok(n) => self.push_token_with_span(Token::NumberLit(n)),
                        Err(_) => eprintln!("Warning: invalid integer '{}'", self.num_buf),
                    }
                }

                if let Ok(n) = self.num_buf.parse::<u64>() {
                    self.push_token_with_span(Token::NumberLit(n));
                } else {
                    eprintln!("Warning: invalid number '{}'", self.num_buf);
                }
                self.num_buf.clear();
                self.state = LexState::Default;
            }
        }
    }
    fn tokenize_ident(&mut self) {
        if let Some(&c) = self.peek() {
            if c.is_alphabetic() || c == '_' {
                self.ident_buf.push(c);
                self.next();
            } else {
                let token = match self.ident_buf.as_str() {
                    "if" => Token::Keyword(KeywordKind::If),
                    "let" => Token::Keyword(KeywordKind::Let),
                    "else" => Token::Keyword(KeywordKind::Else),
                    "String" => Token::Keyword(KeywordKind::String),
                    "true" => Token::BoolLit(true),
                    "false" => Token::BoolLit(false),
                    //-------- ТИПЫ -----------//
                    "i8" => Token::Type(Type::I8),
                    "i16" => Token::Type(Type::I16),
                    "i32" => Token::Type(Type::I32),
                    "i64" => Token::Type(Type::I64),
                    // Без-знаковые
                    "u8" => Token::Type(Type::U8),
                    "u16" => Token::Type(Type::U16),
                    "u32" => Token::Type(Type::U32),
                    "u64" => Token::Type(Type::U64),
                    // С плавающей точкой
                    "f16" => Token::Type(Type::F16),
                    "f32" => Token::Type(Type::F32),
                    "f64" => Token::Type(Type::F64),
                    // Другие:
                    "bool" => Token::Type(Type::Bool),
                    "str" => Token::Type(Type::Str),
                    "char" => Token::Type(Type::Char),

                    _ => Token::Ident(self.ident_buf.clone()),
                };
                self.push_token_with_span(token);
                self.ident_buf.clear();
                self.state = LexState::Default;
            }
        }
    }
    fn tokenize_oper(&mut self) {
        if let Some(&c) = self.peek() {
            match c {
                '+' => match self.peek() {
                    Some(&'+') => {
                        self.push_token_with_span(Token::Oper(OperKind::Inc));
                    }
                    Some(&'=') => {
                        self.push_token_with_span(Token::Oper(OperKind::AddAssign));
                    }
                    _ => {
                        self.push_token_with_span(Token::Oper(OperKind::Add));
                    }
                },
                '-' => match self.peek() {
                    Some(&'-') => {
                        self.push_token_with_span(Token::Oper(OperKind::Dec));
                    }
                    Some(&'>') => {
                        self.push_token_with_span(Token::Arrow);
                    }
                    Some('=') => {
                        self.push_token_with_span(Token::Oper(OperKind::SubAssign));
                    }
                    _ => {
                        self.push_token_with_span(Token::Oper(OperKind::Sub));
                    }
                },
                '*' => match self.peek() {
                    Some(&'*') => {
                        self.next();
                        match self.peek() {
                            Some(&'=') => {
                                self.push_token_with_span(Token::Oper(OperKind::PowAssign));
                            }
                            _ => {
                                self.push_token_with_span(Token::Oper(OperKind::Pow));
                            }
                        }
                    }
                    Some(&'=') => {
                        self.push_token_with_span(Token::Oper(OperKind::MulAssign));
                    }
                    _ => {
                        self.push_token_with_span(Token::Oper(OperKind::Mul));
                    }
                },
                '/' => match self.peek() {
                    Some(&'=') => {
                        self.push_token_with_span(Token::Oper(OperKind::DivAssign));
                    }
                    _ => {
                        self.push_token_with_span(Token::Oper(OperKind::Div));
                    }
                },
                '%' => {
                    self.push_token_with_span(Token::Oper(OperKind::Rem));
                }
                '=' => match self.peek() {
                    Some(&'=') => {
                        self.push_token_with_span(Token::Oper(OperKind::Eq));
                    }
                    Some(&'>') => {
                        self.push_token_with_span(Token::Arrow);
                    }
                    _ => {
                        self.push_token_with_span(Token::Oper(OperKind::Assign));
                    }
                },
                '!' => match self.peek() {
                    Some(&'=') => {
                        self.push_token_with_span(Token::Oper(OperKind::NotEq));
                    }
                    _ => {
                        self.push_token_with_span(Token::Oper(OperKind::Not));
                    }
                },
                '<' => match self.peek() {
                    Some(&'<') => {
                        self.push_token_with_span(Token::Oper(OperKind::Shl));
                    }
                    Some(&'=') => {
                        self.push_token_with_span(Token::Oper(OperKind::LtEq));
                    }
                    _ => {
                        self.push_token_with_span(Token::Oper(OperKind::Lt));
                    }
                },
                '>' => match self.peek() {
                    Some(&'>') => {
                        self.push_token_with_span(Token::Oper(OperKind::Shr));
                    }
                    Some(&'=') => {
                        self.push_token_with_span(Token::Oper(OperKind::GtEq));
                    }
                    _ => {
                        self.push_token_with_span(Token::Oper(OperKind::Gt));
                    }
                },
                '&' => match self.peek() {
                    Some(&'&') => {
                        self.push_token_with_span(Token::Oper(OperKind::And));
                    }
                    _ => {
                        self.push_token_with_span(Token::Oper(OperKind::BitAnd));
                    }
                },
                '|' => match self.peek() {
                    Some(&'|') => {
                        self.push_token_with_span(Token::Oper(OperKind::Or));
                    }
                    _ => {
                        self.push_token_with_span(Token::Oper(OperKind::BitOr));
                    }
                },
                '~' => {
                    self.push_token_with_span(Token::Oper(OperKind::BitNot));
                }
                '^' => {
                    self.push_token_with_span(Token::Oper(OperKind::BitXor));
                }
                _ => {
                    self.push_token_with_span(Token::Oper(OperKind::BitAnd));
                }
            }
            self.next();
            self.state = LexState::Default
        }
    }

    pub fn tokenize(&mut self) {
        while let Some(&c) = self.peek() {
            self.col += 1;
            match self.state {
                LexState::Default => {
                    if c == '\n' {
                        self.new_line();

                        self.next();
                        continue;
                    } else if c.is_whitespace() {
                        self.next();
                        continue;
                    } else if c.is_alphabetic() || c == '_' {
                        self.state = LexState::InIdent;
                    } else if c.is_ascii_digit() {
                        self.state = LexState::InNumber;
                    } else {
                        match c {
                            '(' => {
                                self.push_token_with_span(Token::LParen);
                                self.next();
                            }
                            ')' => {
                                self.push_token_with_span(Token::RParen);
                                self.next();
                            }
                            '{' => {
                                self.push_token_with_span(Token::LBrace);
                                self.next();
                            }
                            '}' => {
                                self.push_token_with_span(Token::RBrace);
                                self.next();
                            }
                            '[' => {
                                self.push_token_with_span(Token::LBracket);
                                self.next();
                            }
                            ']' => {
                                self.push_token_with_span(Token::RBracket);
                                self.next();
                            }
                            ',' => {
                                self.push_token_with_span(Token::Comma);
                                self.next();
                            }
                            ';' => {
                                self.push_token_with_span(Token::Semicolon);
                                self.next();
                            }
                            ':' => {
                                self.push_token_with_span(Token::Colon);
                                self.next();
                            }
                            '\'' => {
                                self.next();
                                self.push_token_with_span(Token::CharLit(
                                    self.peek().unwrap().clone(),
                                ));
                                self.next();
                                self.next();
                            }
                            _ => self.state = LexState::InOper,
                        }
                    }
                }
                LexState::InString => {
                    if c == '"' {
                        self.next();
                        self.push_token_with_span(Token::StrLit(self.str_buf.clone()));
                        self.str_buf.clear();

                        self.state = LexState::Default;
                        continue;
                    }

                    self.str_buf.push(c);
                    self.next();
                    continue;
                }
                LexState::InIdent => self.tokenize_ident(),
                LexState::InNumber => self.tokenize_number(),
                LexState::InOper => self.tokenize_oper(),
            }
        }
        self.push_token_with_span(Token::Eof);
    }
}

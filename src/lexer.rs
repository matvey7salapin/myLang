use crate::types::{KeywordKind, OperKind, Token, Type};
#[derive(PartialEq)]
enum LexState {
    Default,
    InString,
    InIdent,
    InNumber,
    InOper,
}

pub struct Lexer {
    tokens: Vec<Token>,
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
    fn new_line(&mut self) -> usize {
        self.line += 1;
        self.col = 0;

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
                        Ok(n) => self.tokens.push(Token::FloatLit(n)),
                        Err(_) => eprintln!("Warning: invalid float '{}'", self.num_buf),
                    }
                } else {
                    match self.num_buf.parse::<u64>() {
                        Ok(n) => self.tokens.push(Token::NumberLit(n)),
                        Err(_) => eprintln!("Warning: invalid integer '{}'", self.num_buf),
                    }
                }

                if let Ok(n) = self.num_buf.parse::<u64>() {
                    self.tokens.push(Token::NumberLit(n));
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
                self.tokens.push(token);
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
                        self.tokens.push(Token::Oper(OperKind::Inc));
                    }
                    Some(&'=') => {
                        self.tokens.push(Token::Oper(OperKind::AddAssign));
                    }
                    _ => {
                        self.tokens.push(Token::Oper(OperKind::Add));
                    }
                },
                '-' => match self.peek() {
                    Some(&'-') => {
                        self.tokens.push(Token::Oper(OperKind::Dec));
                    }
                    Some(&'>') => {
                        self.tokens.push(Token::Arrow);
                    }
                    Some('=') => {
                        self.tokens.push(Token::Oper(OperKind::SubAssign));
                    }
                    _ => {
                        self.tokens.push(Token::Oper(OperKind::Sub));
                    }
                },
                '*' => match self.peek() {
                    Some(&'*') => {
                        self.next();
                        match self.peek() {
                            Some(&'=') => {
                                self.tokens.push(Token::Oper(OperKind::PowAssign));
                            }
                            _ => {
                                self.tokens.push(Token::Oper(OperKind::Pow));
                            }
                        }
                    }
                    Some(&'=') => {
                        self.tokens.push(Token::Oper(OperKind::MulAssign));
                    }
                    _ => {
                        self.tokens.push(Token::Oper(OperKind::Mul));
                    }
                },
                '/' => match self.peek() {
                    Some(&'=') => {
                        self.tokens.push(Token::Oper(OperKind::DivAssign));
                    }
                    _ => {
                        self.tokens.push(Token::Oper(OperKind::Div));
                    }
                },
                '%' => {
                    self.tokens.push(Token::Oper(OperKind::Rem));
                }
                '=' => match self.peek() {
                    Some(&'=') => {
                        self.tokens.push(Token::Oper(OperKind::Eq));
                    }
                    Some(&'>') => {
                        self.tokens.push(Token::Arrow);
                    }
                    _ => {
                        self.tokens.push(Token::Oper(OperKind::Assign));
                    }
                },
                '!' => match self.peek() {
                    Some(&'=') => {
                        self.tokens.push(Token::Oper(OperKind::NotEq));
                    }
                    _ => {
                        self.tokens.push(Token::Oper(OperKind::Not));
                    }
                },
                '<' => match self.peek() {
                    Some(&'<') => {
                        self.tokens.push(Token::Oper(OperKind::Shl));
                    }
                    Some(&'=') => {
                        self.tokens.push(Token::Oper(OperKind::LtEq));
                    }
                    _ => {
                        self.tokens.push(Token::Oper(OperKind::Lt));
                    }
                },
                '>' => match self.peek() {
                    Some(&'>') => {
                        self.tokens.push(Token::Oper(OperKind::Shr));
                    }
                    Some(&'=') => {
                        self.tokens.push(Token::Oper(OperKind::GtEq));
                    }
                    _ => {
                        self.tokens.push(Token::Oper(OperKind::Gt));
                    }
                },
                '&' => match self.peek() {
                    Some(&'&') => {
                        self.tokens.push(Token::Oper(OperKind::And));
                    }
                    _ => {
                        self.tokens.push(Token::Oper(OperKind::BitAnd));
                    }
                },
                '|' => match self.peek() {
                    Some(&'|') => {
                        self.tokens.push(Token::Oper(OperKind::Or));
                    }
                    _ => {
                        self.tokens.push(Token::Oper(OperKind::BitOr));
                    }
                },
                '~' => {
                    self.tokens.push(Token::Oper(OperKind::BitNot));
                }
                '^' => {
                    self.tokens.push(Token::Oper(OperKind::BitXor));
                }
                _ => {
                    self.tokens.push(Token::Oper(OperKind::BitAnd));
                }
            }
            self.next();
            self.state = LexState::Default
        }
    }

    pub fn tokenize(&mut self) /* -> Vec<Token> */
    {
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
                                self.tokens.push(Token::LParen);
                                self.next();
                            }
                            ')' => {
                                self.tokens.push(Token::RParen);
                                self.next();
                            }
                            '{' => {
                                self.tokens.push(Token::LBrace);
                                self.next();
                            }
                            '}' => {
                                self.tokens.push(Token::RBrace);
                                self.next();
                            }
                            '[' => {
                                self.tokens.push(Token::LBracket);
                                self.next();
                            }
                            ']' => {
                                self.tokens.push(Token::RBracket);
                                self.next();
                            }
                            ',' => {
                                self.tokens.push(Token::Comma);
                                self.next();
                            }
                            ';' => {
                                self.tokens.push(Token::Semicolon);
                                self.next();
                            }
                            ':' => {
                                self.tokens.push(Token::Colon);
                                self.next();
                            }
                            '\'' => {
                                self.next();
                                self.tokens
                                    .push(Token::CharLit(self.peek().unwrap().clone()));
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
                        self.tokens.push(Token::StrLit(self.str_buf.clone()));
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
        self.tokens.push(Token::Eof);

        //return self.tokens;
    }
}

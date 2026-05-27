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
    fn next(&mut self) {
        self.pos += 1;
    }
    fn peek(&mut self) -> Option<&char> {
        return Some(&self.chars[self.pos + 1]);
    }
    fn advance(&mut self) -> Option<char> {
        self.pos += 1;
        return self.chars.get(self.pos - 1).cloned();
    }

    pub fn tokenize(&mut self, code: &str) -> Vec<Token> {
        self.chars = code.trim().chars().collect();
        let mut tokens: Vec<Token> = Vec::new();

        self.ident_buf = String::new();
        self.num_buf = String::new();
        self.str_buf = String::new();

        self.state = LexState::Default;

        while let Some(&c) = self.peek() {
            match self.state {
                LexState::Default => {
                    if c.is_whitespace() {
                        self.next();
                        continue;
                    } else if c.is_alphabetic() || c == '_' {
                        self.state = LexState::InIdent;
                    } else if c.is_ascii_digit() {
                        self.state = LexState::InNumber;
                    } else {
                        match c {
                            '(' => {
                                tokens.push(Token::LParen);
                                self.next();
                            }
                            ')' => {
                                tokens.push(Token::RParen);
                                self.next();
                            }
                            '{' => {
                                tokens.push(Token::LBrace);
                                self.next();
                            }
                            '}' => {
                                tokens.push(Token::RBrace);
                                self.next();
                            }
                            '[' => {
                                tokens.push(Token::LBracket);
                                self.next();
                            }
                            ']' => {
                                tokens.push(Token::RBracket);
                                self.next();
                            }
                            ',' => {
                                tokens.push(Token::Comma);
                                self.next();
                            }
                            ';' => {
                                tokens.push(Token::Semicolon);
                                self.next();
                            }
                            ':' => {
                                tokens.push(Token::Colon);
                                self.next();
                            }
                            '\'' => {
                                self.next();
                                tokens.push(Token::CharLit(self.peek().unwrap().clone()));
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
                        if self.state == LexState::InString {
                            tokens.push(Token::StrLit(self.str_buf.clone()));
                            self.str_buf.clear();

                            self.state = LexState::Default;
                        } else {
                            self.state = LexState::InString;
                        }
                        continue;
                    }

                    self.str_buf.push(c);
                    self.next();
                    continue;
                }
                LexState::InIdent => {
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
                        tokens.push(token);
                        self.ident_buf.clear();
                        self.state = LexState::Default;
                    }
                }
                LexState::InNumber => {
                    if c.is_ascii_digit() || c == '_' || c == '.' {
                        if c != '_' {
                            self.num_buf.push(c);
                        }
                        self.next();
                    } else {
                        if self.num_buf.contains('.') {
                            match self.num_buf.parse::<f64>() {
                                Ok(n) => tokens.push(Token::FloatLit(n)),
                                Err(_) => eprintln!("Warning: invalid float '{}'", self.num_buf),
                            }
                        } else {
                            match self.num_buf.parse::<u64>() {
                                Ok(n) => tokens.push(Token::NumberLit(n)),
                                Err(_) => eprintln!("Warning: invalid integer '{}'", self.num_buf),
                            }
                        }

                        if let Ok(n) = self.num_buf.parse::<u64>() {
                            tokens.push(Token::NumberLit(n));
                        } else {
                            eprintln!("Warning: invalid number '{}'", self.num_buf);
                        }
                        self.num_buf.clear();
                        self.state = LexState::Default;
                    }
                }
                LexState::InOper => {
                    match c {
                        '+' => match self.peek() {
                            Some('+') => {
                                tokens.push(Token::Oper(OperKind::Inc));
                            }
                            Some('=') => {
                                tokens.push(Token::Oper(OperKind::AddAssign));
                            }
                            _ => {
                                tokens.push(Token::Oper(OperKind::Add));
                            }
                        },
                        '-' => match self.peek() {
                            Some('-') => {
                                tokens.push(Token::Oper(OperKind::Dec));
                            }
                            Some('>') => {
                                tokens.push(Token::Arrow);
                            }
                            Some('=') => {
                                tokens.push(Token::Oper(OperKind::SubAssign));
                            }
                            _ => {
                                tokens.push(Token::Oper(OperKind::Sub));
                            }
                        },
                        '*' => match self.peek() {
                            Some('*') => {
                                self.next();
                                match self.peek() {
                                    Some('=') => {
                                        tokens.push(Token::Oper(OperKind::PowAssign));
                                    }
                                    _ => {
                                        tokens.push(Token::Oper(OperKind::Pow));
                                    }
                                }
                            }
                            Some('=') => {
                                tokens.push(Token::Oper(OperKind::MulAssign));
                            }
                            _ => {
                                tokens.push(Token::Oper(OperKind::Mul));
                            }
                        },
                        '/' => match self.peek() {
                            Some('=') => {
                                tokens.push(Token::Oper(OperKind::DivAssign));
                            }
                            _ => {
                                tokens.push(Token::Oper(OperKind::Div));
                            }
                        },
                        '%' => {
                            tokens.push(Token::Oper(OperKind::Rem));
                        }
                        '=' => match self.peek() {
                            Some('=') => {
                                tokens.push(Token::Oper(OperKind::Eq));
                            }
                            Some('>') => {
                                tokens.push(Token::Arrow);
                            }
                            _ => {
                                tokens.push(Token::Oper(OperKind::Assign));
                            }
                        },
                        '!' => match self.peek() {
                            Some('=') => {
                                tokens.push(Token::Oper(OperKind::NotEq));
                            }
                            _ => {
                                tokens.push(Token::Oper(OperKind::Not));
                            }
                        },
                        '<' => match self.peek() {
                            Some('<') => {
                                tokens.push(Token::Oper(OperKind::Shl));
                            }
                            Some('=') => {
                                tokens.push(Token::Oper(OperKind::LtEq));
                            }
                            _ => {
                                tokens.push(Token::Oper(OperKind::Lt));
                            }
                        },
                        '>' => match self.peek() {
                            Some('>') => {
                                tokens.push(Token::Oper(OperKind::Shr));
                            }
                            Some('=') => {
                                tokens.push(Token::Oper(OperKind::GtEq));
                            }
                            _ => {
                                tokens.push(Token::Oper(OperKind::Gt));
                            }
                        },
                        '&' => match self.peek() {
                            Some('&') => {
                                tokens.push(Token::Oper(OperKind::And));
                            }
                            _ => {
                                tokens.push(Token::Oper(OperKind::BitAnd));
                            }
                        },
                        '|' => match self.peek() {
                            Some('|') => {
                                tokens.push(Token::Oper(OperKind::Or));
                            }
                            _ => {
                                tokens.push(Token::Oper(OperKind::BitOr));
                            }
                        },
                        '~' => {
                            tokens.push(Token::Oper(OperKind::BitNot));
                        }
                        '^' => {
                            tokens.push(Token::Oper(OperKind::BitXor));
                        }
                        _ => {
                            tokens.push(Token::Oper(OperKind::BitAnd));
                        }
                    }
                    self.next();
                    self.state = LexState::Default
                }
            }
        }
        tokens.push(Token::Eof);

        return tokens;
    }
}

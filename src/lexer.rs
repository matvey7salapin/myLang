use crate::types::{KeywordKind, LexState, OperKind, Token, Type};
pub fn tokenize(code: &str) -> Vec<Token> {
    let mut chars = code.trim().chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();

    let mut ident_buf = String::new();
    let mut num_buf = String::new();
    let mut str_buf = String::new();

    let mut state = LexState::Default;

    while let Some(&c) = chars.peek() {
        match state {
            LexState::Default => {
                if c.is_whitespace() {
                    chars.next();
                    continue;
                } else if c.is_alphabetic() || c == '_' {
                    state = LexState::InIdent;
                } else if c.is_ascii_digit() {
                    state = LexState::InNumber;
                } else {
                    match c {
                        '(' => {
                            tokens.push(Token::LParen);
                            chars.next();
                        }
                        ')' => {
                            tokens.push(Token::RParen);
                            chars.next();
                        }
                        '{' => {
                            tokens.push(Token::LBrace);
                            chars.next();
                        }
                        '}' => {
                            tokens.push(Token::RBrace);
                            chars.next();
                        }
                        '[' => {
                            tokens.push(Token::LBracket);
                            chars.next();
                        }
                        ']' => {
                            tokens.push(Token::RBracket);
                            chars.next();
                        }
                        ',' => {
                            tokens.push(Token::Comma);
                            chars.next();
                        }
                        ';' => {
                            tokens.push(Token::Semicolon);
                            chars.next();
                        }
                        ':' => {
                            tokens.push(Token::Colon);
                            chars.next();
                        }
                        '\'' => {
                            chars.next();
                            tokens.push(Token::CharLit(chars.peek().unwrap().clone()));
                            chars.next();
                            chars.next();
                        }
                        _ => state = LexState::InOper,
                    }
                }
            }
            LexState::InString => {
                if c == '"' {
                    chars.next();
                    if state == LexState::InString {
                        tokens.push(Token::StrLit(str_buf.clone()));
                        str_buf.clear();

                        state = LexState::Default;
                    } else {
                        state = LexState::InString;
                    }
                    continue;
                }

                str_buf.push(c);
                chars.next();
                continue;
            }
            LexState::InIdent => {
                if c.is_alphabetic() || c == '_' {
                    ident_buf.push(c);
                    chars.next();
                } else {
                    let token = match ident_buf.as_str() {
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

                        _ => Token::Ident(ident_buf.clone()),
                    };
                    tokens.push(token);
                    ident_buf.clear();
                    state = LexState::Default;
                }
            }
            LexState::InNumber => {
                if c.is_ascii_digit() || c == '_' || c == '.' {
                    if c != '_' {
                        num_buf.push(c);
                    }
                    chars.next();
                } else {
                    if num_buf.contains('.') {
                        match num_buf.parse::<f64>() {
                            Ok(n) => tokens.push(Token::FloatLit(n)),
                            Err(_) => eprintln!("Warning: invalid float '{}'", num_buf),
                        }
                    } else {
                        match num_buf.parse::<u64>() {
                            Ok(n) => tokens.push(Token::NumberLit(n)),
                            Err(_) => eprintln!("Warning: invalid integer '{}'", num_buf),
                        }
                    }

                    if let Ok(n) = num_buf.parse::<u64>() {
                        tokens.push(Token::NumberLit(n));
                    } else {
                        eprintln!("Warning: invalid number '{}'", num_buf);
                    }
                    num_buf.clear();
                    state = LexState::Default;
                }
            }
            LexState::InOper => {
                match c {
                    '+' => match chars.peek() {
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
                    '-' => match chars.peek() {
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
                    '*' => match chars.peek() {
                        Some('*') => {
                            chars.next();
                            match chars.peek() {
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
                    '/' => match chars.peek() {
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
                    '=' => match chars.peek() {
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
                    '!' => match chars.peek() {
                        Some('=') => {
                            tokens.push(Token::Oper(OperKind::NotEq));
                        }
                        _ => {
                            tokens.push(Token::Oper(OperKind::Not));
                        }
                    },
                    '<' => match chars.peek() {
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
                    '>' => match chars.peek() {
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
                    '&' => match chars.peek() {
                        Some('&') => {
                            tokens.push(Token::Oper(OperKind::And));
                        }
                        _ => {
                            tokens.push(Token::Oper(OperKind::BitAnd));
                        }
                    },
                    '|' => match chars.peek() {
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
                chars.next();
                state = LexState::Default
            }
        }
    }
    tokens.push(Token::Eof);

    return tokens;
}

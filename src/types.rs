#[derive(Debug, PartialEq, Clone)]
pub enum KeywordKind {
    Let,
    If,
    Else,
    String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperKind {
    // Арифметические:
    Add,
    Sub,
    Mul,
    Div,
    Rem, // % остаток от деления
    Pow, // ** возведение в степень

    // Инкрементальные:
    Inc, // ++
    Dec, // --

    // Сравнения:
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    // Логические:
    And,
    Or,
    Not,
    // Присваивания:
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    PowAssign,
    // Битовые:
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    Shl,
    Shr,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(KeywordKind),
    Type(Type),
    Ident(String),
    StrLit(String),
    CharLit(char),
    Oper(OperKind),
    NumberLit(u64),
    FloatLit(f64),
    BoolLit(bool),
    LParen,
    RParen, // ( )
    LBrace,
    RBrace, // { }
    LBracket,
    RBracket,  // [ ]
    Comma,     // ,
    Semicolon, // ;
    Colon,     // :
    Arrow,
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F16,
    F32,
    F64,
    Bool,
    Str,
    Char,
}
#[derive(PartialEq)]
pub enum Expr {
    Number(u64),
    Float(f64),
    Bool(bool),
    Char(char),
    Str(String),
    Ident(String),

    Binary {
        left: Box<Expr>,
        op: OperKind,
        right: Box<Expr>,
    },
}
#[derive(PartialEq)]
pub enum Stmt {
    Let {
        name: String,
        ty: Option<Type>,
        value: Option<Expr>,
    },
    Assign {
        target: Expr,
        value: Expr,
    },
    ExprStmt(Expr),
    Block(Vec<Stmt>),
    If {
        cond: Expr,
        then: Box<Stmt>,
        else_: Option<Box<Stmt>>,
    },
    Return(Option<Expr>),
}
#[derive(PartialEq)]
pub struct Program {
    pub statements: Vec<Stmt>,
    pub imports: Vec<String>,
    pub module_name: Option<String>,
}

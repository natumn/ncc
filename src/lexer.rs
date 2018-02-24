pub use self::Token::{ClosingParenthesis, Comma, Def, Delimiter, Extern, Ident, Number,
                      OpeningParenthesis, Operator};

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    Def,
    Extern,
    Delimiter, //';' character
    OpeningParenthesis,
    ClosingParenthesis,
    Comma,
    Ident(String),
    Number(f64),
    Operator(String),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub type: TokenType,
    pub Contents: String,
    pub pos: Span,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Span {
    pub Filename: String,
    pub StartLine: i64,
    pub EndLine: i64,
}

#[derive(PartialEq, Clone, Debug)]
pub struct File {
    pub path: String,
    pub name: String,
    pub Contents: Vec<char>,
    pub NewLines: Vec<i64>,
    pub Tokens: Vec<Token>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Position {
    pub filename: String,
    pub line: i64,
    pub ch: i64,
}

struct lexer {
    input: &'a file,
    startPos: i64,
    endPos: i64,
    curPos: &'a Position,
    tokenStart: &'a Position,
}

impl lexer {
    // 式をtokenに切り出す。

    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        let mut result = Vec::new();

        token = nextCh();
        match token {}
    }

    // 次の文字(char)を取得する。
    pub fn nextCh(&mut self) -> char {}
}

pub fn Lex(input Vec<char>) -> []*Token {
    Token {
        type: Def,
        Contents: "test",
    }
}

pub use self::Token::{ClosingParenthesis, Comma, Def, Delimiter, Extern, Ident, Number,
                      OpeningParenthesis, Operator};

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
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

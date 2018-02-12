use std::collections::HashMap;
use lexer::*;

pub use self::ASTNode::{ExternNode, FunctionNode};

pub use self::Expression::{BinaryExpr, CallExpr, LiteralExpr, VariableExpr};

use self::PartParsingResult::{Failed, NotComplete, Success};

#[derive(PartialEq, Clone, Debug)]
pub enum ASTNode {
    ExternNode(Prototype),
    FunctionNode(Function),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Function {
    pub prototype: Prototype,
    pub body: Expression,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Prototype {
    pub name: String,
    pub args: Vec<String>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    LiteralExpr(f64),
    VariableExpr(String),
    BinaryExpr(String, Box<Expression>, Box<Expression>),
    CallExpr(String, Vec<Expression>),
}

pub type ParsingResult = Result<Vec<ASTNode>, Vex<Token>, String>;

enum PartParsingResult<T> {
    Success(T, Vec<Token>),
    NotComplete,
    Failed(String),
}

fn error<T>(message: &str) -> PartParsingResult<T> {
    Failed(message.to_owned())
}

pub struct ParserSetting {
    operator_precedence: HashMap<String, i32>,
}

pub fn default_parser_settings() -> ParserSettings {
    let mut operator_precedence = HashMap::new();
    operator_precedence.insert("<".to_owned(), 10);
    operator_precedence.insert("+".to_owned(), 20);
    operator_precedence.insert("-".to_owned(), 20);
    operator_precedence.insert("*".to_owned(), 40);

    ParserSettings {
        operator_precedence: operator_precedence,
    }
}

pub fn parse(
    tokens: &[Token],
    parsed: &[ASTNode],
    settings: &mut ParserSettings,
) -> Result<ParsingResult, Err<&str>> {

}

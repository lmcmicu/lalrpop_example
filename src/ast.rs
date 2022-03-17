use std::fmt::Debug;

#[derive(Debug)]
pub enum AstExpression {
    Label(String),
    Field(String, String),
    NamedArg(String, String),
    RegexMatch(String, String),
    RegexSub(String, String, String),
    Function(String, Vec<Box<AstExpression>>),
}

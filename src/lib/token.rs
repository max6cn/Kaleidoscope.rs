// The lexer returns tokens[0-255] if it is an unknow character, otherwise one of
// these for known things.
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    TokEof,
    TokDef,
    TokExtern,
    TokIdentifier(String),
    TokNumber(f64),
    TokChar(char),
}

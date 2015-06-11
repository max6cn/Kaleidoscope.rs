// The lexer returns tokens[0-255] if it is an unknow character, otherwise one of
// these for known things.
#[derive(Clone,Debug,PartialEq)]
pub enum Token {
    tok_eof,
    tok_def,
    tok_extern,
    tok_identifier(String),
    tok_number(f64),
    tok_char(char)}



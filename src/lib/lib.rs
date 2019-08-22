#[macro_use]
extern crate log;
extern crate env_logger;

pub mod Token;

pub mod Ast;
pub mod Codegen;
pub mod Lexer;
pub mod Parser;
use Token::*;
use Lexer::*;
use Parser::*;
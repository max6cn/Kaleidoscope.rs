#[macro_use]
extern crate log;
extern crate env_logger;

pub mod token;

pub mod ast;
pub mod codegen;
pub mod lexer;
pub mod parser;
use token::*;
use lexer::*;
use parser::*;
// #![feature(core,convert)]
#![feature(libc,rustc_private)]
#![allow(unused_imports,unused_features,non_camel_case_types)]
#![allow(non_snake_case,dead_code,unused_variables)]
#![allow(unused_must_use,unreachable_code)]
// #![feature(scoped)]

#[macro_use] extern crate log;
extern crate env_logger;


pub mod Token;
// use Token::{tok_char,tok_number,tok_identifier,tok_def,tok_eof,tok_extern};

pub mod Ast;
pub mod Lexer;
pub mod Parser;
pub mod Codegen;

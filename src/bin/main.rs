// #![feature(core,convert)]
// #![feature(libc,rustc_private)]
#![allow(unused_imports,unused_features,non_camel_case_types)]
#![allow(non_snake_case,dead_code,unused_variables)]
#![allow(unused_must_use,unreachable_code)]
// #![feature(scoped)]
#[macro_use] extern crate log;
extern crate env_logger;

extern crate libc;
// extern crate rustc_llvm;
use std::io;
use std::io::{Read,Write};
//use core::ops::Index;
use std::string;
use std::sync::{Arc,Mutex};
use std::thread;
use std::sync::mpsc::{channel,Receiver,Sender};
use std::collections::HashMap;
use std::vec::*;
// use rustc_llvm as llvm;
extern crate lib;
use lib::Token::*;
use lib::Lexer::*;
use lib::Parser::*;
fn main() {
    env_logger::init().unwrap();
    let (tokenSender, tokenReceiver) = channel::<Token>();
    let g1 = thread::spawn(move || {
        let mut lexer = Lexer::new(tokenSender);
        lexer.run();
    });
    let g2 = thread::spawn(move || {
        let mut parser = Parser::new(tokenReceiver);
        parser.run();
    }); //parser.run();
    let output =g1.join();
}

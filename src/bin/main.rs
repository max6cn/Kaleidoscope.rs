#[macro_use]
extern crate log;
extern crate env_logger;
extern crate libc;
// extern crate rustc_llvm;
// use std::io;
// use std::io::{Read, Write};
// use std::collections::HashMap;
// use std::string;
// use std::sync::mpsc::{channel, Receiver, Sender};
// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::vec::*;
// // use rustc_llvm as llvm;
// extern crate lib;
// use lib::Lexer::*;
// use lib::Parser::*;
// use lib::Token::*;
fn main() {
        env_logger::init().unwrap();
        info!("starting");
    //     let (tokenSender, tokenReceiver) = channel::<Token>();
    //     let g1 = thread::spawn(move || {
    //         let mut lexer = Lexer::new(tokenSender);
    //         lexer.run();
    //     });
    //     let g2 = thread::spawn(move || {
    //         let mut parser = Parser::new(tokenReceiver);
    //         parser.run();
    //     }); //parser.run();
    //     let output = g1.join();
}

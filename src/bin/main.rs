#[macro_use]
extern crate log;
extern crate env_logger;
extern crate lib;

use lib::*;
use std::io::{Read, Write};
use std::str::Chars;
use std::thread;

fn main() {
    env_logger::init();
    info!("starting");
    let (tx_char_stream, rx_char_stream) = std::sync::mpsc::channel();
    let (tx_token_stream, rx_token_stream) = std::sync::mpsc::channel();
    let mut parser = Parser::new(rx_token_stream);

    let mut line = "".to_owned();
    let mut should_quit = false;

    let _input_guard = thread::spawn(move || loop {
        //        std::io::stdout().write(">".as_bytes());
        //        std::io::stdout().flush();
        let mut c: [u8; 1] = [0];
        std::io::stdin().read_exact(&mut c);
        let c = c[0] as char;
        tx_char_stream.send(c as char);
    });

    let _lexer_guard = thread::spawn(move || {
        let mut lexer = Lexer::new(rx_char_stream, tx_token_stream.clone());
        lexer.get_token();
    });
    //    let _parser_guard = thread::spawn(move || {
    let _res = parser.run();
    println!("tokens : {:?}", parser);
    //    });
}

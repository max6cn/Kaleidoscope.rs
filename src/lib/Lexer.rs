use std::sync::mpsc::{channel,Receiver,Sender};
use ::Token::*;
use std::io;
use std::io::{Read,Write};

pub struct Lexer {
    tokenSender : Sender<Token>,
    buf: Vec<u8>,}
impl Lexer {
    pub fn new(tokenSender: Sender<Token>) -> Lexer {
        return Lexer { tokenSender : tokenSender, buf: vec![' ' as u8]};
    }
    pub fn run (&mut self) {
        let mut reader = io::stdin();
        //let mut buf : Vec<u8>= vec![' ' as u8];
        let mut bs = self.buf.as_mut_slice();
        let mut lastchar = ' ';
        loop{
            while lastchar.is_whitespace() {
                lastchar = match reader.read(bs) {
                    Ok(1)  =>  bs[0] as char,
                    Ok(_) | Err(_) => {
                        self.tokenSender.send(Token::tok_eof);
                        return ;} } ; }
            if lastchar.is_alphabetic() { // identifier [a-zA-Z][a-zA-Z0-9]*
                let mut identifierStr: String = "".to_string();
                identifierStr.push(lastchar);
                loop {
                    match reader.read(bs) {
                        Ok(1) => { let ch = bs[0] as char;
                                   if ch.is_alphabetic() {
                                       identifierStr.push(ch);
                                   } else {
                                       lastchar = ch;
                                       break;
                                   }
                        },
                        Ok(_)| Err(_) => {
                            self.tokenSender.send(Token::tok_eof);
                            return;}}}
                let identifier = identifierStr;
                if identifier == "def" {
                    self.tokenSender.send(Token::tok_def);}
                else if identifier == "extern" {
                    self.tokenSender.send(Token::tok_extern);}
                else {
                    self.tokenSender.send(Token::tok_identifier(identifier));}
                continue;}

            if lastchar.is_digit(10) || lastchar == '.' {
                let mut numString :String = "".to_string();
                numString.push(lastchar);
                loop {
                    match reader.read(bs)  {
                        Ok(1) => { let ch = bs[0]as char;
                                    if ch.is_digit(10) || ch == '.' { numString.push(ch);}
                                    else {lastchar = ch;break;}},
                        Ok(_)| Err(_) => {self.tokenSender.send(Token::tok_eof);
                                   return;unreachable!();}}}
                //let num :f64 = numString.parse();
                self.tokenSender.send(
                    Token::tok_number(
                        match numString.parse::<f64>() {
                            Ok(val) => val,
                            Err(_) => { panic!("Malformed number :{:?}",numString);}}));
                continue;}
            if lastchar == '#' {
                loop {
                    match reader.read(bs){
                        Ok(1) => { let chr = bs[0] as char;
                                   if chr == '\r' || chr == '\n' {
                                       lastchar = ' ';
                                       break;}},
                        Ok(_) | Err(_) => {self.tokenSender.send(Token::tok_eof);
                                   return;}}}
                continue;}
            self.tokenSender.send(Token::tok_char(lastchar));
            lastchar = ' ';
        }
    }
}


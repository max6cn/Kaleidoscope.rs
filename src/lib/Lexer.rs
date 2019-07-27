use std::sync::mpsc::{channel,Receiver,Sender};
use ::Token::*;
use std::io;
use std::io::{Read,Write};

pub struct Lexer {
    input : Receiver<char>,
    output : Sender<Token>
}
impl Lexer {
    pub fn new(input: Receiver<char>, output: Sender<Token>) -> Lexer {
        return Lexer { input, output };
    }

    pub fn get_token (&mut self) {

        let mut lastchar = ' ';
        loop{
            // eat all space
            while lastchar.is_whitespace() {
                if let Ok(ch) =   self.input.recv() {
                    lastchar = ch ;
                }  else {
                     self.output.send(Token::tok_eof);
                }
            }
            if lastchar.is_alphabetic() { // identifier [a-zA-Z][a-zA-Z0-9]*
                let mut identifierStr  = String::new();
                identifierStr.push(lastchar);
                loop {
                    match self.input.recv() {
                        Ok(ch) => {
                                   if ch.is_alphabetic() {
                                       identifierStr.push(ch);
                                   } else {
                                       lastchar = ch;
                                       break;
                                   }
                        },
                        Err(e) => {
                                self.output.send(Token::tok_eof);
                        }
                    }
                    match identifierStr.as_str() {
                        "def" => self.output.send(Token::tok_def),
                        "extern" => self.output.send(Token::tok_extern),
                        identifier => self.output.send(Token::tok_identifier(identifier.to_string()))
                    };
                }
            }
            if lastchar.is_digit(10) || lastchar == '.' {
                let mut numString  =  String::new();
                numString.push(lastchar);
                loop {
                    match self.input.recv()  {
                        Ok(ch) => { if ch.is_digit(10) || ch == '.' { numString.push(ch);}
                                    else { lastchar = ch; break;}},
                        Err(_) => {self.output.send(Token::tok_eof);
                                   return;unreachable!();}}
                                   }
                if let Ok(num) =  numString.parse::<f64>() {
                     self.output.send(Token::tok_number(num));
                } else {
                     panic!("Malformed number :{:?}",numString);
                }
            }
            if lastchar == '#' {
                loop {
                    match self.input.recv() {
                        Ok(chr) => {
                                   if chr == '\r' || chr == '\n' {
                                       lastchar = ' ';
                                       break;}},
                        Err(_) => {self.output.send(Token::tok_eof);
                                   return;}
                                   }
                }
            self.output.send(Token::tok_char(lastchar));
            lastchar = ' ';
        }
        }
    }
}

#[cfg(test)]
mod test {
    //#[macro_use]
    extern crate log;
    extern crate env_logger;

    use super::super::Token::*;
    use super::Lexer;
    use std::sync::mpsc::channel;
    use std::thread;
    #[test]
    fn test_lex() {
        let (tx , inputstream) = channel();
        let (tokenStream, rx) = channel();
        let mut lexer = Lexer::new(inputstream,tokenStream);
        thread::spawn (move ||  { lexer.get_token(); });
        let  check = |text,expect| {
            let tx1 = tx.clone();
            thread::spawn (move ||  {
                    for c in text {
                    tx1.send(c);
            }});
            loop {
                 if let Ok(res) = rx.recv() {
                        println!("====== {:?}",res);

                        // assert_eq!(res,  expect);
                 }
            }
            };
        //a bs sad = + . 123 1221.2 def exter ty wtf ! #
        check("    def     ".chars() , Token::tok_def );
    }

}

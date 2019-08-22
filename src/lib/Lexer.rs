use std::io;
use std::io::{Read, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use Token::*;

pub struct Lexer {
    input: Receiver<char>,
    output: Sender<Token>,
}
impl Lexer {
    pub fn new(input: Receiver<char>, output: Sender<Token>) -> Lexer {
        return Lexer { input, output };
    }

    pub fn get_token(&mut self) {
        let mut lastchar = ' ';
        loop {
            // eat all space
            while lastchar.is_whitespace() {
                if let Ok(ch) = self.input.recv() {
                    lastchar = ch;
                } else {
                    self.output.send(Token::tok_eof);
                }
            }
            if lastchar.is_alphabetic() {
                // identifier [a-zA-Z][a-zA-Z0-9]*
                let mut identifierStr = String::new();
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
                        }
                        Err(e) => {
                            self.output.send(Token::tok_eof);
                        }
                    }
                }
                // FIXME:
                // if ident matches keyword at begining, we should wait until the
                // ident match finished
                match identifierStr.as_str() {
                    "def" => self.output.send(Token::tok_def),
                    "extern" => self.output.send(Token::tok_extern),
                    identifier => self
                        .output
                        .send(Token::tok_identifier(identifier.to_string())),
                };
            } else if lastchar.is_digit(10) || lastchar == '.' {
                let mut numString = String::new();
                numString.push(lastchar);
                loop {
                    match self.input.recv() {
                        Ok(ch) => {
                            if ch.is_digit(10) || ch == '.' {
                                numString.push(ch);
                            } else {
                                lastchar = ch;
                                break;
                            }
                        }
                        Err(_) => {
                            self.output.send(Token::tok_eof);
                            return;
                            unreachable!();
                        }
                    }
                }
                if let Ok(num) = numString.parse::<f64>() {
                    self.output.send(Token::tok_number(num));
                } else {
                    panic!("Malformed number :{:?}", numString);
                }
            } else if lastchar == '#' {
                loop {
                    match self.input.recv() {
                        Ok(chr) => {
                            if chr == '\r' || chr == '\n' {
                                lastchar = ' ';
                                break;
                            }
                        }
                        Err(_) => {
                            self.output.send(Token::tok_eof);
                            return;
                        }
                    }
                }
            } else if lastchar == '$' {
                self.output.send(Token::tok_eof);
                return;
            } else {
                self.output.send(Token::tok_char(lastchar));
                lastchar = ' ';
            }
        }
    }
}

#[cfg(test)]
mod test {
    //#[macro_use]
    extern crate env_logger;
    extern crate log;

    use super::super::Token::*;
    use super::Lexer;
    use std::sync::mpsc::channel;
    use std::thread;
    #[test]
    fn test_lex() {
        let program = r" a a a a bs sad 123 = +  * - /
             1221.2 def defs externa ty wtf ! #
             ";
        //a bs sad = + . 123 1221.2 def exter ty wtf ! #
        let expected = vec![
            Token::tok_identifier("a".into()),
            Token::tok_identifier("a".into()),
            Token::tok_identifier("a".into()),
            Token::tok_identifier("a".into()),
            Token::tok_identifier("bs".into()),
            Token::tok_identifier("sad".into()),
            Token::tok_number(123.0),
            Token::tok_char('='),
            Token::tok_char('+'),
            Token::tok_char('*'),
            Token::tok_char('-'),
            Token::tok_char('/'),
            Token::tok_number(1221.2),
            Token::tok_def,
            Token::tok_identifier("externa".into()),
            Token::tok_identifier("ty".into()),
            Token::tok_identifier("wtf".into()),
            Token::tok_char('!'),
            Token::tok_eof,
        ];
        let (tx, inputstream) = channel();
        let (tokenStream, rx) = channel();
        let mut lexer = Lexer::new(inputstream, tokenStream);
        let t0 = thread::spawn(move || {
            lexer.get_token();
        });
        let check = move |text, expected| {
            let tx1 = tx.clone();
            let t = thread::spawn(move || {
                for c in text {
                    tx1.send(c);
                }
                tx1.send('$');
                drop(tx1);
            });
            let mut tokens = vec![];
            loop {
                match rx.recv() {
                    Ok(res) => {
                        // println!("====== {:?}", res);
                        tokens.push(res);
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
            println!("tokens : {:?}", tokens);
            assert_eq!(tokens, expected);
            t.join();
        };

        check(program.chars(), expected);
        t0.join();
    }

}

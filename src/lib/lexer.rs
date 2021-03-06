use std::sync::mpsc::{Receiver, Sender};
use token::*;

pub struct Lexer {
    input: Receiver<char>,
    output: Sender<Token>,
}
impl Lexer {
    pub fn new(input: Receiver<char>, output: Sender<Token>) -> Lexer {
        Lexer { input, output }
    }

    pub fn get_token(&mut self) {
        let mut lastchar = ' ';
        loop {
            // eat all space
            while lastchar.is_whitespace() {
                if let Ok(ch) = self.input.recv() {
                    lastchar = ch;
                } else {
                    self.output.send(Token::TokEof).expect("Send Error");
                }
            }
            if lastchar.is_alphabetic() {
                // identifier [a-zA-Z][a-zA-Z0-9]*
                let mut identifier_str = String::new();
                identifier_str.push(lastchar);
                loop {
                    match self.input.recv() {
                        Ok(ch) => {
                            if ch.is_alphabetic() {
                                identifier_str.push(ch);
                            } else {
                                lastchar = ch;
                                break;
                            }
                        }
                        Err(_e) => {
                            self.output.send(Token::TokEof).expect("Send Error");
                        }
                    }
                }
                // FIXME:
                // if ident matches keyword at begining, we should wait until the
                // ident match finished
                let _e = match identifier_str.as_str() {
                    "def" => self.output.send(Token::TokDef),
                    "extern" => self.output.send(Token::TokExtern),
                    identifier => self
                        .output
                        .send(Token::TokIdentifier(identifier.to_string())),
                };
            } else if lastchar.is_digit(10) || lastchar == '.' {
                let mut num_string = String::new();
                num_string.push(lastchar);
                loop {
                    match self.input.recv() {
                        Ok(ch) => {
                            if ch.is_digit(10) || ch == '.' {
                                num_string.push(ch);
                            } else {
                                lastchar = ch;
                                break;
                            }
                        }
                        Err(_) => {
                            self.output.send(Token::TokEof).expect("Send Error");
                            return;
                        }
                    }
                }
                if let Ok(num) = num_string.parse::<f64>() {
                    self.output.send(Token::TokNumber(num)).expect("Send Error");
                } else {
                    panic!("Malformed number :{:?}", num_string);
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
                            self.output.send(Token::TokEof).expect("Send Error");
                            return;
                        }
                    }
                }
            } else if lastchar == '$' {
                self.output.send(Token::TokEof).expect("Send Error");
                return;
            } else {
                self.output
                    .send(Token::TokChar(lastchar))
                    .expect("Send Error");
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
    use super::*;
    use std::sync::mpsc::channel;
    use std::thread;
    #[test]
    fn test_lex() {
        let program = r" a a a a bs sad 123 = +  * - /
             1221.2 def defs externa ty wtf ! #
             ";
        //a bs sad = + . 123 1221.2 def exter ty wtf ! #
        let expected = vec![
            Token::TokIdentifier("a".into()),
            Token::TokIdentifier("a".into()),
            Token::TokIdentifier("a".into()),
            Token::TokIdentifier("a".into()),
            Token::TokIdentifier("bs".into()),
            Token::TokIdentifier("sad".into()),
            Token::TokNumber(123.0),
            Token::TokChar('='),
            Token::TokChar('+'),
            Token::TokChar('*'),
            Token::TokChar('-'),
            Token::TokChar('/'),
            Token::TokNumber(1221.2),
            Token::TokDef,
            Token::TokIdentifier("defs".into()),
            Token::TokIdentifier("externa".into()),
            Token::TokIdentifier("ty".into()),
            Token::TokIdentifier("wtf".into()),
            Token::TokChar('!'),
            Token::TokEof,
        ];
        let (tx, inputstream) = channel();
        let (token_stream, rx) = channel();
        let mut lexer = Lexer::new(inputstream, token_stream);
        let t0 = thread::spawn(move || {
            lexer.get_token();
        });
        let check = move |text, expected| {
            let tx1 = tx.clone();
            let t = thread::spawn(move || {
                for c in text {
                    tx1.send(c).expect("Send Error");
                }
                tx1.send('$').expect("Send Error");
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
            let _ = t.join();
        };

        check(program.chars(), expected);
        let _ = t0.join();
    }

}

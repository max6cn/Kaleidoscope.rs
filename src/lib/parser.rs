use env_logger;
use std::collections::HashMap;
use std::io::{stdin, stdout, Read, Write};
use std::mem::transmute;
use std::sync::mpsc::{channel, Receiver, Sender};

//use super::token::{TokChar,TokNumber,TokIdentifier,TokDef,TokEof,TokExtern};
use Ast::*;
use Token::*;
#[derive(Debug)]
pub struct Parser {
    token_input: Receiver<Token>,
    current_token: Token,
    binop_precedence: HashMap<char, i8>,
}

impl<'a> Parser {
    pub fn new(token_input: Receiver<Token>) -> Parser {
        let mut binop_precedence = HashMap::new();
        binop_precedence.insert('<', 10);
        binop_precedence.insert('+', 20);
        binop_precedence.insert('-', 20);
        binop_precedence.insert('*', 40);
        return Parser {
            token_input: token_input,
            current_token: Token::TokChar(' '),
            binop_precedence: binop_precedence,
        };
    }
    fn get_tok_precendence(&mut self) -> i8 {
        match self.current_token {
            Token::TokChar(ch) if self.binop_precedence.contains_key(&ch) => {
                self.binop_precedence[&ch]
            }
            _ => return -1,
        }
    }
    fn get_next_token(&mut self) -> Token {
        self.current_token = match self.token_input.recv() {
            Ok(tok) => {
                //debug!("Got token :{:?}",&tok);
                tok
            }
            Err(_) => Token::TokEof,
        };
        return self.current_token.clone();
    }
    ///
    /// identifierexpr
    ///   ::= identifier
    ///   ::= identifier '(' expression* ')'
    ///
    fn parse_identifier_expr(&mut self) -> Option<&'a ExprAst> {
        debug!("Parse Identifier");
        if let Token::TokIdentifier(idName) = self.current_token.clone() {
            self.get_next_token();
            // simple variable ref
            if self.current_token.clone() != Token::TokChar('(') {
                let exp = VariableExprAst { name: idName };
                return Some(unsafe { transmute(&exp) });
            }
            // call.
            self.get_next_token(); //eat (
            let mut args: Vec<&'a ExprAst> = vec![];
            if self.current_token.clone() != Token::TokChar(')') {
                loop {
                    let e = self.parse_expression();
                    match e {
                        Some(arg) => {
                            args.push(*arg);
                        }
                        None => {
                            return None;
                        }
                    }

                    if self.current_token.clone() == Token::TokChar(')') {
                        break;
                    }
                    if self.current_token.clone() != Token::TokChar(',') {
                        debug!("Error, Expect ')' or ',' in argument list");
                        return None;
                    }
                    self.get_next_token();
                }
            }
            // eat the ')'
            self.get_next_token();
            let callast = CallExprAst {
                callee: idName,
                args: args,
            };
            return Some(unsafe { transmute(&callast) });
        }
        None
    }
    ///
    /// numberexpr ::= number
    ///
    fn parse_number_expr(&mut self) -> Option<&'a ExprAst> {
        debug!("Parse Number exp");
        let result = NumberExprAst {
            val: match self.current_token.clone() {
                Token::TokNumber(v) => v,
                _ => {
                    debug!("Invalid numebr");
                    0.0
                }
            },
        };
        self.get_next_token(); // consume number
        Some(unsafe { transmute(&result) })
    }
    ///
    /// parenexpr ::= '(' expression ')'
    ///
    fn parse_paren_expr(&mut self) -> Option<&'a ExprAst> {
        debug!("parse Paren Expr");
        self.get_next_token(); // eat (.
        if let Some(v) = self.parse_expression() {
            if self.current_token != Token::TokChar(')') {
                debug!("Error: Expected ')'");
                return None;
            }
            self.get_next_token(); // eat )
            return Some(*v);
        }
        None
    }
    ///
    /// primary
    ///   ::= identifierexpr
    ///   ::= numberexpr
    ///   ::= parenexpr
    ///
    fn parse_primary(&mut self) -> Option<&'a ExprAst> {
        debug!("Parse Primary");
        match self.current_token.clone() {
            Token::TokIdentifier(id) => self.parse_identifier_expr(),
            Token::TokNumber(val) => self.parse_number_expr(),
            Token::TokChar('(') => self.parse_paren_expr(),
            _ => {
                debug!("unknow token when expecting an expression");
                None
            }
        }
    }
    ///
    /// binoprhs
    ///   ::= ('+' primary)*
    ///
    fn parse_bin_op_rhs(&mut self, exprPrec: i8, lhs: &'a ExprAst) -> Option<Box<&'a ExprAst>> {
        debug!("Parse binOpRHS");
        loop {
            let tok_prec = self.get_tok_precendence();
            // If this is a binop that binds at least as tightly as the current binop,
            // consume it, otherwise we are done.
            if tok_prec < exprPrec {
                return Some(Box::new(lhs));
            }
            // Okay, we know this is a binop.
            let bin_op = match self.current_token {
                Token::TokChar(op) => op,
                _ => unreachable!(),
            };
            self.get_next_token(); //eat binop
                                 // Parse the primary expression after the binary operator.
            match self.parse_primary() {
                Some(rhs) => {
                    // If BinOp binds less tightly with rhs than the operator after rhs, let
                    // the pending operator take rhs as its lhs.
                    let next_prec = self.get_tok_precendence();
                    if tok_prec < next_prec {
                        match self.parse_bin_op_rhs(tok_prec + 1, rhs) {
                            Some(rhs) => {
                                let rhs = *rhs;
                                let lhs = BinaryExprAst {
                                    op: bin_op,
                                    lhs: lhs,
                                    rhs: rhs,
                                };
                                debug!("Got Expr: {:?}", &lhs);
                                return Some(Box::new(unsafe { transmute(&lhs) }));
                            }
                            None => {
                                return None;
                            }
                        }
                    }
                }
                None => {
                    return None;
                }
            }
        }
    }
    ///
    /// expression
    ///   ::= primary binoprhs
    ///
    ///
    fn parse_expression(&mut self) -> Option<Box<&'a ExprAst>> {
        debug!("Parse expression");
        if let Some(lhs) = self.parse_primary() {
            let exp = self.parse_bin_op_rhs(0, &lhs);
            match exp {
                // return Some(Box::new(*exp));
                Some(e) => return Some(Box::new(*e)),
                None => {
                    return None;
                }
            }
        }
        None
    }
    ///
    /// prototype
    ///   ::= id '(' id* ')'
    ///
    fn parse_proto_type(&mut self) -> Option<PrototypeAst> {
        debug!("Parse ProtoType ");
        match self.current_token.clone() {
            Token::TokIdentifier(id) => {
                let fn_name = id;
                // debug!("wait ( ");
                self.get_next_token();
                match self.current_token.clone() {
                    Token::TokChar('(') => {
                        let mut argname: Vec<String> = vec![];
                        // debug!("Got (");
                        loop {
                            // matching an identifier
                            self.get_next_token();
                            match self.current_token.clone() {
                                Token::TokIdentifier(arg) => {
                                    argname.push(arg);
                                }
                                _ => {
                                    break;
                                }
                            };
                        }
                        // debug!("argnames :{:?}",&argname);
                        match self.current_token {
                            Token::TokChar(')') => {
                                // success
                                self.get_next_token(); //eat ')'
                                let proto = PrototypeAst {
                                    name: fn_name,
                                    args: argname,
                                };
                                debug!("Got prototype: {:?} ", &proto);
                                Some(proto)
                            }
                            _ => {
                                debug!("Expecting ( in prototype");
                                None
                            }
                        }
                    }
                    _ => {
                        debug!("Expecting ( in prototype");
                        None
                    }
                }
            }
            _ => {
                debug!("Expecting function name in prototype");
                None
            }
        }
    }
    ///
    /// definition ::= 'def' prototype expression
    ///
    fn parse_definition(&mut self) -> Option<Box<FunctionAst>> {
        debug!("Parse  definition");
        self.get_next_token();
        match self.parse_proto_type() {
            Some(proto) => match self.parse_expression() {
                Some(expr) => Some(Box::new(FunctionAst {
                    proto: proto,
                    body: &expr,
                })),
                None => None,
            },
            None => None,
        }
    }
    ///
    /// toplevelexpr ::= expression
    ///
    fn parse_top_level_expr(&mut self) -> Option<Box<FunctionAst>> {
        if let Some(expr) = self.parse_expression() {
            // make an anonymous proto.
            debug!("make an anonymous proto.");
            let prot1 = Box::new(PrototypeAst {
                name: "".to_string(),
                args: vec![],
            });
            //let proto  :&'static PrototypeAst =&  prot1;
            return Some(Box::new(FunctionAst {
                proto: *prot1,
                body: *expr,
            }));
        }
        None
    }
    /// external ::= 'extern' prototype
    fn parse_extern(&mut self) -> Option<PrototypeAst> {
        self.get_next_token();
        return self.parse_proto_type();
    }
    //===----------------------------------------------------------------------===//
    // Top-Level parsing
    //===----------------------------------------------------------------------===//
    fn handle_definition(&mut self) {
        match self.parse_definition() {
            Some(ast) => {
                debug!("Parsed a function Definition, {:?}", ast);
            }
            None => {
                self.get_next_token();
            }
        }
    }
    fn handle_extern(&mut self) {
        match self.parse_extern() {
            Some(_) => {
                debug!("Parsed an extern \n");
            }
            None => {
                self.get_next_token();
            }
        }
    }
    fn handle_top_level_expr(&mut self) {
        match self.parse_top_level_expr() {
            Some(_) => {
                debug!("Parsed an top-level expr");
            }
            None => {
                self.get_next_token();
            }
        };
    }
    pub fn run(&mut self) -> () {
        print!("ready> ");
        stdout().flush().unwrap();
        self.get_next_token();
        loop {
            let token = self.current_token.clone();
            match token {
                Token::TokEof => {
                    return;
                }
                Token::TokChar(';') => {
                    self.get_next_token();
                    continue;
                }
                Token::TokDef => self.handle_definition(),
                Token::TokExtern => self.handle_extern(),
                ref s => self.handle_top_level_expr(),
            }
            print!("ready> ");
            stdout().flush().unwrap();
            self.get_next_token();
        }
    }
    pub fn parse_input(&mut self) {
        self.get_next_token();
        let token = self.current_token.clone();
        match token {
            Token::TokEof => {}
            Token::TokChar(';') => {
                self.get_next_token();
            }
            Token::TokDef => self.handle_definition(),
            Token::TokExtern => self.handle_extern(),
            ref s => self.handle_top_level_expr(),
        }
        self.get_next_token();
    }
}
#[cfg(test)]
mod test {
    //#[macro_use]
    extern crate env_logger;
    extern crate log;
    use std::sync::mpsc::channel;
    use std::thread;

    use super::super::Token::*;
    use super::Parser;
    use super::super::Lexer::*;
    #[test]
    fn test_binop_pre() {
        let (tx, rx) = channel();
        let mut parser = Parser::new(rx);
        let mut check = |ch, expect| {
            tx.send(Token::TokChar(ch));
            parser.get_next_token();
            let res = parser.get_tok_precendence();
            assert_eq!(res, expect);
        };
        check('+', 20);
        check('-', 20);
        check('/', -1);
        check('*', 40);
    }
    #[test]
    fn test_def() {
        env_logger::init().unwrap();

        let (tx, rx) = channel();
        thread::spawn(move || {
            // tx.send(token::TokDef);
            tx.send(Token::TokIdentifier("a".to_string()));
            tx.send(Token::TokChar('('));
            tx.send(Token::TokIdentifier("b".to_string()));
            tx.send(Token::TokChar(')'));
            tx.send(Token::TokNumber(123.1));
            tx.send(Token::TokEof);
        });
        let mut parser = Parser::new(rx);
        let res = parser.parse_definition();
        let res_str = format!("{:?}", res);
        // dbg!(res);
        assert_eq!(res_str, "None "); // test case wrong
    }
    /// rx_char_stream ----> test_stub  ----> tx_char_stream
    /// rx_char_stream --> lexer --> tx_token_stream
    /// rx_token_stream --> parser --> AstJson;
    ///
    #[test]
    fn test_parse_input() {
        env_logger::init().unwrap();
        let prog = r"
# Compute the x'th fibonacci number.
def fib(x)
  if x < 3 then
    1
  else
    fib(x-1)+fib(x-2)

# This expression will compute the 40th number.
fib(40)
";

        let (tx_char_stream, rx_char_stream) = channel();
        let (tx_token_stream, rx_token_stream) = channel();
        let lexer_guard = thread::spawn(move || {
            for c in prog.chars() {
                let _ = tx_char_stream.send(c);
            }
            let _ = tx_char_stream.send('$');
            let mut lexer = Lexer::new(rx_char_stream, tx_token_stream);
            lexer.get_token();
        });
        let parser_guard = thread::spawn(move || {
            let mut parser = Parser::new(rx_token_stream);
            let res = parser.parse_input();
            println!("tokens : {:?}", parser);
        });
        let _ = lexer_guard.join();
        let _ = parser_guard.join();
    }

}

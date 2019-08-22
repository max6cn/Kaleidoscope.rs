use env_logger;
use std::collections::HashMap;
use std::io::{stdin, stdout, Read, Write};
use std::mem::transmute;
use std::sync::mpsc::{channel, Receiver, Sender};

//use super::Token::{tok_char,tok_number,tok_identifier,tok_def,tok_eof,tok_extern};
use Ast::*;
use Token::*;
#[derive(Debug)]
pub struct Parser {
    tokenInput: Receiver<Token>,
    currentToken: Token,
    binopPrecedence: HashMap<char, i8>,
}

impl<'a> Parser {
    pub fn new(tokenInput: Receiver<Token>) -> Parser {
        let mut binopPrecedence = HashMap::new();
        binopPrecedence.insert('<', 10);
        binopPrecedence.insert('+', 20);
        binopPrecedence.insert('-', 20);
        binopPrecedence.insert('*', 40);
        return Parser {
            tokenInput: tokenInput,
            currentToken: Token::tok_char(' '),
            binopPrecedence: binopPrecedence,
        };
    }
    fn getTokPrecendence(&mut self) -> i8 {
        match self.currentToken {
            Token::tok_char(ch) if self.binopPrecedence.contains_key(&ch) => {
                self.binopPrecedence[&ch]
            }
            _ => return -1,
        }
    }
    fn getNextToken(&mut self) -> Token {
        self.currentToken = match self.tokenInput.recv() {
            Ok(tok) => {
                //debug!("Got token :{:?}",&tok);
                tok
            }
            Err(_) => Token::tok_eof,
        };
        return self.currentToken.clone();
    }
    ///
    /// identifierexpr
    ///   ::= identifier
    ///   ::= identifier '(' expression* ')'
    ///
    fn parseIdentifierExpr(&mut self) -> Option<&'a ExprAst> {
        debug!("Parse Identifier");
        if let Token::tok_identifier(idName) = self.currentToken.clone() {
            self.getNextToken();
            // simple variable ref
            if self.currentToken.clone() != Token::tok_char('(') {
                let exp = VariableExprAst { name: idName };
                return Some(unsafe { transmute(&exp) });
            }
            // call.
            self.getNextToken(); //eat (
            let mut args: Vec<&'a ExprAst> = vec![];
            if self.currentToken.clone() != Token::tok_char(')') {
                loop {
                    let e = self.parseExpression();
                    match e {
                        Some(arg) => {
                            args.push(*arg);
                        }
                        None => {
                            return None;
                        }
                    }

                    if self.currentToken.clone() == Token::tok_char(')') {
                        break;
                    }
                    if self.currentToken.clone() != Token::tok_char(',') {
                        debug!("Error, Expect ')' or ',' in argument list");
                        return None;
                    }
                    self.getNextToken();
                }
            }
            // eat the ')'
            self.getNextToken();
            let callast = CallExprAst {
                Callee: idName,
                Args: args,
            };
            return Some(unsafe { transmute(&callast) });
        }
        None
    }
    ///
    /// numberexpr ::= number
    ///
    fn parseNumberExpr(&mut self) -> Option<&'a ExprAst> {
        debug!("Parse Number exp");
        let result = NumberExprAst {
            val: match self.currentToken.clone() {
                Token::tok_number(v) => v,
                _ => {
                    debug!("Invalid numebr");
                    0.0
                }
            },
        };
        self.getNextToken(); // consume number
        Some(unsafe { transmute(&result) })
    }
    ///
    /// parenexpr ::= '(' expression ')'
    ///
    fn parseParenExpr(&mut self) -> Option<&'a ExprAst> {
        debug!("parse Paren Expr");
        self.getNextToken(); // eat (.
        if let Some(v) = self.parseExpression() {
            if self.currentToken != Token::tok_char(')') {
                debug!("Error: Expected ')'");
                return None;
            }
            self.getNextToken(); // eat )
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
    fn parsePrimary(&mut self) -> Option<&'a ExprAst> {
        debug!("Parse Primary");
        match self.currentToken.clone() {
            Token::tok_identifier(id) => self.parseIdentifierExpr(),
            Token::tok_number(val) => self.parseNumberExpr(),
            Token::tok_char('(') => self.parseParenExpr(),
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
    fn parseBinOpRHS(&mut self, exprPrec: i8, lhs: &'a ExprAst) -> Option<Box<&'a ExprAst>> {
        debug!("Parse binOpRHS");
        loop {
            let tokPrec = self.getTokPrecendence();
            // If this is a binop that binds at least as tightly as the current binop,
            // consume it, otherwise we are done.
            if tokPrec < exprPrec {
                return Some(Box::new(lhs));
            }
            // Okay, we know this is a binop.
            let binOp = match self.currentToken {
                Token::tok_char(op) => op,
                _ => unreachable!(),
            };
            self.getNextToken(); //eat binop
                                 // Parse the primary expression after the binary operator.
            match self.parsePrimary() {
                Some(rhs) => {
                    // If BinOp binds less tightly with RHS than the operator after RHS, let
                    // the pending operator take RHS as its LHS.
                    let nextPrec = self.getTokPrecendence();
                    if tokPrec < nextPrec {
                        match self.parseBinOpRHS(tokPrec + 1, rhs) {
                            Some(rhs) => {
                                let rhs = *rhs;
                                let lhs = BinaryExprAst {
                                    Op: binOp,
                                    LHS: lhs,
                                    RHS: rhs,
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
    fn parseExpression(&mut self) -> Option<Box<&'a ExprAst>> {
        debug!("Parse expression");
        if let Some(lhs) = self.parsePrimary() {
            let exp = self.parseBinOpRHS(0, &lhs);
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
    fn parseProtoType(&mut self) -> Option<PrototypeAst> {
        debug!("Parse ProtoType ");
        match self.currentToken.clone() {
            Token::tok_identifier(id) => {
                let FnName = id;
                // debug!("wait ( ");
                self.getNextToken();
                match self.currentToken.clone() {
                    Token::tok_char('(') => {
                        let mut argname: Vec<String> = vec![];
                        // debug!("Got (");
                        loop {
                            // matching an identifier
                            self.getNextToken();
                            match self.currentToken.clone() {
                                Token::tok_identifier(arg) => {
                                    argname.push(arg);
                                }
                                _ => {
                                    break;
                                }
                            };
                        }
                        // debug!("argnames :{:?}",&argname);
                        match self.currentToken {
                            Token::tok_char(')') => {
                                // success
                                self.getNextToken(); //eat ')'
                                let proto = PrototypeAst {
                                    Name: FnName,
                                    Args: argname,
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
    fn parseDefinition(&mut self) -> Option<Box<FunctionAst>> {
        debug!("Parse  definition");
        self.getNextToken();
        match self.parseProtoType() {
            Some(proto) => match self.parseExpression() {
                Some(expr) => Some(Box::new(FunctionAst {
                    Proto: proto,
                    Body: &expr,
                })),
                None => None,
            },
            None => None,
        }
    }
    ///
    /// toplevelexpr ::= expression
    ///
    fn parseTopLevelExpr(&mut self) -> Option<Box<FunctionAst>> {
        if let Some(expr) = self.parseExpression() {
            // make an anonymous proto.
            debug!("make an anonymous proto.");
            let prot1 = Box::new(PrototypeAst {
                Name: "".to_string(),
                Args: vec![],
            });
            //let proto  :&'static PrototypeAst =&  prot1;
            return Some(Box::new(FunctionAst {
                Proto: *prot1,
                Body: *expr,
            }));
        }
        None
    }
    /// external ::= 'extern' prototype
    fn parseExtern(&mut self) -> Option<PrototypeAst> {
        self.getNextToken();
        return self.parseProtoType();
    }
    //===----------------------------------------------------------------------===//
    // Top-Level parsing
    //===----------------------------------------------------------------------===//
    fn handleDefinition(&mut self) {
        match self.parseDefinition() {
            Some(ast) => {
                debug!("Parsed a function Definition, {:?}", ast);
            }
            None => {
                self.getNextToken();
            }
        }
    }
    fn handleExtern(&mut self) {
        match self.parseExtern() {
            Some(_) => {
                debug!("Parsed an extern \n");
            }
            None => {
                self.getNextToken();
            }
        }
    }
    fn handleTopLevelExpr(&mut self) {
        match self.parseTopLevelExpr() {
            Some(_) => {
                debug!("Parsed an top-level expr");
            }
            None => {
                self.getNextToken();
            }
        };
    }
    pub fn run(&mut self) -> () {
        print!("ready> ");
        stdout().flush().unwrap();
        self.getNextToken();
        loop {
            let token = self.currentToken.clone();
            match token {
                Token::tok_eof => {
                    return;
                }
                Token::tok_char(';') => {
                    self.getNextToken();
                    continue;
                }
                Token::tok_def => self.handleDefinition(),
                Token::tok_extern => self.handleExtern(),
                ref s => self.handleTopLevelExpr(),
            }
            print!("ready> ");
            stdout().flush().unwrap();
            self.getNextToken();
        }
    }
    pub fn parse_input(&mut self) {
        self.getNextToken();
        let token = self.currentToken.clone();
        match token {
            Token::tok_eof => {}
            Token::tok_char(';') => {
                self.getNextToken();
            }
            Token::tok_def => self.handleDefinition(),
            Token::tok_extern => self.handleExtern(),
            ref s => self.handleTopLevelExpr(),
        }
        self.getNextToken();
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
    fn test_binopPre() {
        let (tx, rx) = channel();
        let mut parser = Parser::new(rx);
        let mut check = |ch, expect| {
            tx.send(Token::tok_char(ch));
            parser.getNextToken();
            let res = parser.getTokPrecendence();
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
            // tx.send(Token::tok_def);
            tx.send(Token::tok_identifier("a".to_string()));
            tx.send(Token::tok_char('('));
            tx.send(Token::tok_identifier("b".to_string()));
            tx.send(Token::tok_char(')'));
            tx.send(Token::tok_number(123.1));
            tx.send(Token::tok_eof);
        });
        let mut parser = Parser::new(rx);
        let res = parser.parseDefinition();
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
                tx_char_stream.send(c);
            }
            tx_char_stream.send('$');
            let mut lexer = Lexer::new(rx_char_stream, tx_token_stream);
            lexer.get_token();
        });
        let parser_guard = thread::spawn(move || {
            let mut parser = Parser::new(rx_token_stream);
            let res = parser.parse_input();
            println!("tokens : {:?}", parser);
        });
        lexer_guard.join();
        parser_guard.join();
    }

}
